//! A window showing one image, or several with a thumbnail sidebar, with
//! Preview's editing tools.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::ui::dir::column;
use crate::{column, row};
use bytes::Bytes;
use iced::advanced::image::Allocation;
use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::image::Handle;
use iced::widget::scrollable::{self, Direction};
use iced::widget::{
    Id, container, image, mouse_area, operation, scrollable as scroll, slider, space, text, toggler,
};
use iced::{Center, Element, Fill, Length, Padding, Task};
use prev_image::ImageFormat;
use prev_image::decode::{Frame, decode_file};
use prev_image::edit::{ColorAdjust, Operation};
use prev_image::encode::SaveFormat;
use prev_image::metadata::{self, Details};
use prev_image::svg::Svg;
use prev_store::versions::{self, Version, VersionStore};

use super::canvas::{CanvasEvent, ImageCanvas, Selection};
use super::editor::{self, Editor, spawn};
use super::view::{self, Fit, Placement, ZOOM_STEP};
use crate::dialog;
use crate::i18n::Describe;
use crate::pdf::viewer::editing::EditMessage;
use crate::pdf::viewer::{PdfMessage, Zoom};
use crate::pdf::window::{self as pdf_window, PdfWindow};
use crate::shortcuts::Action;
use crate::ui::button::Kind;
use crate::ui::component::{self, Backdrop};
use crate::ui::resize::{self, Drag, Width};
use crate::ui::{self, Icon, Type, icon, style};

mod dnd;
mod markup;

const THUMBNAIL_SIZE: u32 = 480;
const SIDEBAR_WIDTH: Width = Width::new(184.0, 140.0, 400.0);
/// Space around a sidebar thumbnail: list padding and the frame.
const SIDEBAR_THUMBNAIL_MARGIN: f32 =
    2.0 * 12.0 + 2.0 * style::THUMBNAIL_RING + resize::HANDLE_WIDTH;
/// Full images kept in memory around the current one, in each direction.
const KEEP_AROUND: usize = 1;
/// Animation frames kept on the GPU ahead of the one showing. iced draws
/// nothing the first time it meets an image, so a frame shows only once it
/// is uploaded; uploading every frame of a long animation would take too
/// much memory.
const FRAMES_AHEAD: usize = 4;
/// Quiet time after an edit before it is written to disk.
const AUTOSAVE_DELAY: Duration = Duration::from_millis(1200);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Raster(ImageFormat),
    Svg,
}

/// The image a window shows, as agents read it.
pub struct ShownImage {
    /// Its place among the window's images, from 0.
    pub index: usize,
    pub count: usize,
    pub width: u32,
    pub height: u32,
    /// Frames of an animation, 1 for a still image.
    pub frames: usize,
    pub pixels: Pixels,
}

/// An image's pixels, ready to draw at any size.
#[derive(Clone)]
pub enum Pixels {
    /// The first frame, with the window's edits.
    Raster(Arc<Frame>),
    /// Edits still being drawn: the work that draws them, once, and the
    /// size they come out at.
    Edits(Arc<std::sync::Mutex<Option<DrawEdits>>>, (u32, u32)),
    Svg(Svg),
}

/// Draws an image with its edits.
type DrawEdits = Box<dyn FnOnce() -> Frame + Send>;

impl Pixels {
    /// The pixels, scaled down to fit `max_side` if they are larger.
    /// Slow: call it off the interface thread.
    pub fn draw(&self, max_side: Option<u32>) -> Result<Frame, String> {
        let fit = |width: f32, height: f32| {
            max_side.map_or(1.0, |side| (side as f32 / width.max(height)).min(1.0))
        };
        match self {
            Pixels::Svg(svg) => {
                let (width, height) = svg.size();
                svg.render(fit(width, height))
                    .map_err(|error| error.describe())
            }
            Pixels::Edits(draw, _) => {
                let draw = draw
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .take()
                    .ok_or("the edits were drawn already")?;
                Pixels::Raster(Arc::new(draw())).draw(max_side)
            }
            Pixels::Raster(frame) => {
                let scale = fit(frame.width as f32, frame.height as f32);
                if scale >= 1.0 {
                    return Ok(Frame::clone(frame));
                }
                let image =
                    ::image::RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone())
                        .ok_or("the image's pixels are incomplete")?;
                let width = ((frame.width as f32 * scale).round() as u32).max(1);
                let height = ((frame.height as f32 * scale).round() as u32).max(1);
                let small = ::image::imageops::resize(
                    &image,
                    width,
                    height,
                    ::image::imageops::FilterType::Triangle,
                );
                Ok(Frame {
                    width,
                    height,
                    pixels: small.into_raw(),
                    delay: Duration::ZERO,
                })
            }
        }
    }
}

/// A decoded image, ready to show.
#[derive(Debug, Clone)]
pub struct LoadedImage {
    pub width: u32,
    pub height: u32,
    /// Raster frames with their delays; empty for SVG.
    frames: Vec<(Handle, Duration)>,
    /// First frame pixels, for editing and making downscaled copies.
    full: Option<Bytes>,
    svg: Option<Svg>,
    thumbnail: Handle,
}

enum ItemState {
    Waiting,
    Loading,
    Loaded(Box<Shown>),
    Failed(String),
}

struct Shown {
    image: LoadedImage,
    /// Downscaled copies by divisor, held on the GPU.
    levels: HashMap<u32, Allocation>,
    levels_pending: HashSet<u32>,
    /// SVG render and the scale it was made at.
    render: Option<(f32, Allocation)>,
    render_pending: Option<f32>,
    editor: Option<Editor>,
    /// Latest quick preview of the edits and its generation.
    preview: Option<(u64, Allocation)>,
    /// Keeps the edited full image on the GPU while it is shown.
    full_allocation: Option<Allocation>,
    preview_running: bool,
    preview_wanted: bool,
    /// Generation of the edits shown in `image`.
    shown_generation: u64,
    /// Animation frames on the GPU, by frame number, and those on their way.
    frame_allocations: HashMap<usize, Allocation>,
    frames_pending: HashSet<usize>,
    /// The full-size edited image, for saving and exporting.
    edited: Option<Arc<Frame>>,
    /// Generation of the edits in `edited`.
    kept_generation: u64,
}

impl Shown {
    fn new(image: LoadedImage) -> Self {
        Self {
            image,
            levels: HashMap::new(),
            levels_pending: HashSet::new(),
            render: None,
            render_pending: None,
            editor: None,
            preview: None,
            full_allocation: None,
            preview_running: false,
            preview_wanted: false,
            shown_generation: 0,
            edited: None,
            kept_generation: 0,
            frame_allocations: HashMap::new(),
            frames_pending: HashSet::new(),
        }
    }

    fn is_editable(&self) -> bool {
        self.image.svg.is_none() && self.image.frames.len() == 1 && self.image.full.is_some()
    }

    /// The image as currently edited, or the original.
    fn current_frame(&self) -> Option<Arc<Frame>> {
        if let Some(edited) = &self.edited {
            return Some(Arc::clone(edited));
        }
        let full = self.image.full.as_ref()?;
        Some(Arc::new(Frame {
            width: self.image.width,
            height: self.image.height,
            pixels: full.to_vec(),
            delay: Duration::ZERO,
        }))
    }
}

struct Item {
    path: PathBuf,
    source: Source,
    state: ItemState,
    thumbnail: Option<Handle>,
    size: Option<(u32, u32)>,
    markup: Option<Markup>,
    /// The markup's page is being made.
    markup_starting: bool,
    /// What to do once the markup opens.
    on_open: Option<dnd::OnOpen>,
}

/// Markup over an image: the PDF tools on a page made of it. Markup lives
/// only as long as the window; exporting draws it into the pixels.
struct Markup {
    window: Box<PdfWindow>,
    /// The PDF made of the image, deleted once open.
    file: PathBuf,
    /// Image pixels per page point.
    scale: f32,
    /// The markup's edit count when it was last exported.
    exported: u64,
}

impl Markup {
    /// Whether there are annotations that no export holds yet.
    fn unexported(&self) -> bool {
        self.window.has_annotations() && self.window.edits() != self.exported
    }
}

impl Drop for Markup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.file);
    }
}

/// What to do about markup that would be lost by closing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseChoice {
    Cancel,
    Discard,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    AdjustColor,
    AdjustSize,
    Inspector,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edit {
    RotateLeft,
    RotateRight,
    FlipHorizontal,
    FlipVertical,
    Crop,
    Undo,
    Redo,
    /// A slider moved: preview only.
    Color(ColorAdjust),
    /// A slider was released: render in full.
    ColorCommitted,
    ResetColor,
    ApplySize,
    /// An edit an agent asks for, made as the toolbar makes its own.
    Apply(Operation),
}

#[derive(Debug, Clone)]
pub enum Message {
    /// Turns floating, auto-hiding toolbars on or off; the app handles it.
    ToggleFloatingBars,
    /// Opens the settings window; the app handles it.
    OpenSettings,
    /// Open or close the assistant panel, which the app keeps.
    ToggleAssistant,
    /// A message from the app's panel in this window's side, which the
    /// app takes back.
    Side(crate::ui::Outside),
    Loaded(usize, Result<LoadedImage, String>),
    LevelReady(usize, u32, Option<Handle>),
    LevelAllocated(usize, u32, Option<Allocation>),
    SvgRendered(usize, f32, Option<Handle>),
    SvgAllocated(usize, f32, Option<Allocation>),
    Canvas(CanvasEvent),
    Select(usize),
    ToggleSidebar,
    ZoomIn,
    ZoomOut,
    ActualSize,
    FitToWindow,
    /// A zoom, where 1.0 is actual size.
    ZoomTo(f32),
    /// Export the current image to a file, as an agent asks: the path,
    /// format, format choice and, for SVG drawings, scale.
    Exporting(PathBuf, SaveFormat, String, Option<f32>),
    Edit(Edit),
    PreviewRendered(usize, u64, Handle),
    PreviewAllocated(usize, u64, Option<Allocation>),
    FullRendered(usize, u64, Arc<Frame>),
    FullAllocated(usize, u64, Arc<Frame>, Option<Allocation>),
    /// An animation frame of an image is on the GPU.
    FrameAllocated(usize, usize, Option<Allocation>),
    AutosaveDue(usize, u64),
    Saved(usize, u64, Result<(), String>),
    TogglePanel(Panel),
    ToggleSelecting,
    SizeWidth(String),
    SizeHeight(String),
    KeepProportions(bool),
    InspectorLoaded(usize, Details, bool, Vec<Version>),
    KeywordsChanged(String),
    DescriptionChanged(String),
    SaveMetadata,
    RemoveLocation,
    MetadataSaved(Result<(), String>),
    Revert(Version),
    Reverted(Result<(), String>),
    Export,
    /// Choices in prev's export dialog, before the save dialog.
    ExportFormat(&'static str),
    ExportQuality(&'static str),
    ExportSize(&'static str),
    ExportChoose,
    ExportCancel,
    ExportAsIs,
    ExportChooseAgain,
    ExportTarget(Result<Option<dialog::SavedWithMenus>, String>),
    Exported(Result<PathBuf, String>),
    DismissNotice,
    SidebarResized(Drag),
    ToggleOverflow,
    CloseOverflow,
    ToggleMarkup,
    /// Undoes the latest image edit, or markup change while there is markup.
    Undo,
    Redo,
    MarkupMade(usize, f32, Result<PathBuf, String>),
    Markup(usize, pdf_window::Message),
    MarkupExported(usize, u64, Result<PathBuf, String>),
    Close(CloseChoice),
    SidebarPressed(usize),
    SidebarMoved(iced::Point),
    SidebarReleased,
    /// A picture from the markup was saved to join the images.
    PictureSaved(Option<PathBuf>),
}

/// An export waiting for the user to confirm a name whose extension does
/// not match the chosen format.
/// What prev's export dialog asks: the format, the JPEG quality and, for
/// SVG drawings, the size.
#[derive(Debug, Clone)]
struct ExportChoice {
    format: String,
    quality: String,
    size: String,
    open: bool,
}

struct PendingExport {
    path: PathBuf,
    format: SaveFormat,
    format_choice: String,
    /// For SVG drawings: how many pixels per point of the drawing.
    scale: Option<f32>,
}

/// Inspector contents for the current image.
struct Inspector {
    index: usize,
    details: Details,
    writable_xmp: bool,
    versions: Vec<Version>,
    keywords: String,
    description: String,
}

pub struct ImageWindow {
    items: Vec<Item>,
    current: usize,
    sidebar: bool,
    sidebar_width: Width,
    /// The toolbar's "More" menu is open.
    overflow_open: bool,
    /// Whether the pointer is over the window, for floating toolbars.
    pointer_inside: bool,
    fit: Fit,
    view: (f32, f32, f32, f32),
    device_scale: f32,
    canvas_id: Id,
    sidebar_id: Id,
    frame: usize,
    next_frame: Option<Instant>,
    panel: Option<Panel>,
    selecting: bool,
    selection: Option<Selection>,
    color: ColorAdjust,
    size_input: (String, String),
    keep_proportions: bool,
    inspector: Option<Inspector>,
    /// Whether the app shows its assistant panel beside this window.
    assistant_shown: bool,
    pending_export: Option<PendingExport>,
    /// The export choices, kept for the next export from this window, and
    /// whether their dialog is showing.
    export_choice: Option<ExportChoice>,
    notice: Option<String>,
    /// Asking whether to close with markup not exported.
    close_prompt: bool,
    /// The window should close; the app does it.
    closing: bool,
    /// A press on a sidebar image, which may drag it out.
    sidebar_press: Option<dnd::SidebarPress>,
    /// A drag started since the app last asked.
    drag_started: bool,
    /// Whether the pointer is over the sidebar, where an image annotation
    /// moved on the markup can be let go to join the images.
    pointer_over_sidebar: bool,
}

/// Uploads `handle` to the GPU, then yields the allocation that keeps it
/// there, so swapping it in draws immediately instead of a blank frame.
fn allocate(
    handle: Handle,
    done: impl Fn(Option<Allocation>) -> Message + Send + 'static,
) -> Task<Message> {
    iced_runtime::image::allocate(handle).map(move |result| done(result.ok()))
}

fn rgba_handle(width: u32, height: u32, pixels: Bytes) -> Handle {
    Handle::from_rgba(width, height, pixels)
}

fn frame_handle(frame: &Frame) -> Handle {
    rgba_handle(frame.width, frame.height, Bytes::from(frame.pixels.clone()))
}

fn load(path: &Path, source: Source) -> Result<LoadedImage, String> {
    match source {
        Source::Svg => {
            let svg = Svg::parse_file(path).map_err(|error| error.describe())?;
            let (width, height) = svg.size();
            let scale = THUMBNAIL_SIZE as f32 / width.max(height).max(1.0);
            let thumbnail = svg.render(scale).map_err(|error| error.describe())?;
            Ok(LoadedImage {
                width: width.round().max(1.0) as u32,
                height: height.round().max(1.0) as u32,
                frames: Vec::new(),
                full: None,
                thumbnail: rgba_handle(thumbnail.width, thumbnail.height, thumbnail.pixels.into()),
                svg: Some(svg),
            })
        }
        Source::Raster(format) => {
            let decoded = decode_file(path, format).map_err(|error| error.describe())?;
            let (width, height) = (decoded.width(), decoded.height());
            let thumbnail = thumbnail_of(&decoded.frames[0]);
            let mut full = None;
            let frames = decoded
                .frames
                .into_iter()
                .map(|frame| {
                    let pixels = Bytes::from(frame.pixels);
                    full.get_or_insert_with(|| pixels.clone());
                    (rgba_handle(frame.width, frame.height, pixels), frame.delay)
                })
                .collect();
            Ok(LoadedImage {
                width,
                height,
                frames,
                full,
                svg: None,
                thumbnail,
            })
        }
    }
}

fn thumbnail_of(frame: &Frame) -> Handle {
    let Some(buffer) =
        ::image::RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone())
    else {
        return rgba_handle(1, 1, Bytes::from_static(&[0, 0, 0, 0]));
    };
    let scale = THUMBNAIL_SIZE as f32 / frame.width.max(frame.height).max(1) as f32;
    let small = if scale < 1.0 {
        ::image::imageops::thumbnail(
            &buffer,
            ((frame.width as f32 * scale).round() as u32).max(1),
            ((frame.height as f32 * scale).round() as u32).max(1),
        )
    } else {
        buffer
    };
    rgba_handle(small.width(), small.height(), small.into_raw().into())
}

fn downscale(pixels: &Bytes, width: u32, height: u32, divisor: u32) -> Option<Handle> {
    let buffer =
        ::image::ImageBuffer::<::image::Rgba<u8>, &[u8]>::from_raw(width, height, pixels.as_ref())?;
    let small = ::image::imageops::resize(
        &buffer,
        (width / divisor).max(1),
        (height / divisor).max(1),
        ::image::imageops::FilterType::Triangle,
    );
    Some(rgba_handle(
        small.width(),
        small.height(),
        small.into_raw().into(),
    ))
}

/// Between the details in the toolbar.
const DETAILS_SEPARATOR: &str = "  ·  ";

fn format_bytes(bytes: u64) -> String {
    match bytes {
        0..1_000_000 => crate::fl!(
            "image-size-kb",
            size = format!("{:.0}", bytes as f64 / 1000.0)
        ),
        _ => crate::fl!(
            "image-size-mb",
            size = format!("{:.1}", bytes as f64 / 1_000_000.0)
        ),
    }
}

impl ImageWindow {
    /// The notices showing: the window's, then its current markup's.
    pub fn notices(&self) -> Vec<&str> {
        let markup = self.markup().and_then(|markup| markup.window.notice());
        self.notice.as_deref().into_iter().chain(markup).collect()
    }

    /// Clears the window's or its markup's notice if it is still `text`.
    pub fn dismiss_notice(&mut self, text: &str) {
        if self.notice.as_deref() == Some(text) {
            self.notice = None;
        }
        for markup in self
            .items
            .iter_mut()
            .filter_map(|item| item.markup.as_mut())
        {
            markup.window.dismiss_notice(text);
        }
    }

    pub fn open(files: Vec<(PathBuf, Source)>) -> (Self, Task<Message>) {
        let sidebar = files.len() > 1;
        let items = files
            .into_iter()
            .map(|(path, source)| Item::new(path, source))
            .collect();
        let mut window = Self {
            items,
            current: 0,
            sidebar,
            sidebar_width: SIDEBAR_WIDTH,
            overflow_open: false,
            pointer_inside: true,
            fit: Fit::Fit,
            view: (0.0, 0.0, 900.0, 700.0),
            device_scale: 1.0,
            canvas_id: Id::unique(),
            sidebar_id: Id::unique(),
            frame: 0,
            next_frame: None,
            panel: None,
            selecting: false,
            selection: None,
            color: ColorAdjust::default(),
            size_input: (String::new(), String::new()),
            keep_proportions: true,
            inspector: None,
            assistant_shown: false,
            pending_export: None,
            export_choice: None,
            notice: None,
            close_prompt: false,
            closing: false,
            sidebar_press: None,
            drag_started: false,
            pointer_over_sidebar: false,
        };
        let task = window.load_next();
        (window, task)
    }

    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.items.iter().map(|item| item.path.as_path())
    }

    /// The image shown, for agents: where it is in the window, its size,
    /// and its pixels as edited so far; `None` while it loads.
    pub fn shown_image(&self) -> Option<ShownImage> {
        let shown = self.shown()?;
        // Edits not yet drawn in full are drawn from the editor, so a
        // picture taken right after an edit shows it.
        let pending = shown
            .editor
            .as_ref()
            .filter(|editor| editor.generation != shown.kept_generation);
        let pixels = match (&shown.image.svg, pending) {
            (Some(svg), _) => Pixels::Svg(svg.clone()),
            (None, Some(editor)) => Pixels::Edits(
                Arc::new(std::sync::Mutex::new(Some(Box::new(editor.render_full())))),
                editor.output_size(),
            ),
            (None, None) => Pixels::Raster(shown.current_frame()?),
        };
        let (width, height) = match &pixels {
            Pixels::Raster(frame) => (frame.width, frame.height),
            Pixels::Edits(_, size) => *size,
            Pixels::Svg(_) => (shown.image.width, shown.image.height),
        };
        Some(ShownImage {
            index: self.current,
            count: self.items.len(),
            width,
            height,
            frames: shown.image.frames.len().max(1),
            pixels,
        })
    }

    /// Shows the assistant button pressed, or not.
    pub fn set_assistant_shown(&mut self, shown: bool) {
        self.assistant_shown = shown;
        // One panel at a time on the right: the assistant takes the place
        // of the inspector or Adjust Color.
        if shown {
            self.panel = None;
        }
    }

    /// Whether the window shows a panel of its own on the right, which the
    /// app's assistant would otherwise share the side with.
    pub fn has_side_panel(&self) -> bool {
        self.panel.is_some()
    }

    /// The panel shown beside the image, if any.
    pub fn panel(&self) -> Option<Panel> {
        self.panel
    }

    pub fn sidebar_shown(&self) -> bool {
        self.sidebar
    }

    /// The current image's size with every edit made so far, before the
    /// edits are drawn.
    pub fn edited_size(&self) -> Option<(u32, u32)> {
        let item = self.items.get(self.current)?;
        item.size
            .or_else(|| self.shown_image().map(|image| (image.width, image.height)))
    }

    /// Exports the current image, with its edits and any markup, to
    /// `path` in `format`; an SVG drawing is drawn at `scale` pixels a
    /// point. The window answers with [`Message::Exported`].
    pub fn export_to(
        &mut self,
        path: PathBuf,
        format: SaveFormat,
        choice: &str,
        scale: Option<f32>,
    ) -> Task<Message> {
        self.run_export(PendingExport {
            path,
            format,
            format_choice: choice.to_owned(),
            scale,
        })
    }

    /// The colour adjustments of the current image, as its sliders show.
    pub fn color(&self) -> ColorAdjust {
        self.color
    }

    /// The format the current image is saved in, when prev can save it
    /// in place.
    pub fn save_format(&self) -> Option<SaveFormat> {
        match self.items.get(self.current)?.source {
            Source::Raster(format) => SaveFormat::for_saving(format, false),
            Source::Svg => None,
        }
    }

    pub fn current_path(&self) -> &Path {
        &self.items[self.current].path
    }

    pub fn set_device_scale(&mut self, scale: f32) -> Task<Message> {
        self.device_scale = scale;
        Task::batch([self.schedule(), self.markup_device_scale(scale)])
    }

    fn shown(&self) -> Option<&Shown> {
        match &self.items.get(self.current)?.state {
            ItemState::Loaded(shown) => Some(shown),
            _ => None,
        }
    }

    fn shown_mut(&mut self, index: usize) -> Option<&mut Shown> {
        match &mut self.items.get_mut(index)?.state {
            ItemState::Loaded(shown) => Some(shown),
            _ => None,
        }
    }

    fn placement(&self) -> Option<Placement> {
        let (width, height) = self.items.get(self.current)?.size?;
        let image = (width as f32, height as f32);
        let viewport = (self.view.2, self.view.3);
        let zoom = view::resolve_zoom(self.fit, image, viewport);
        Some(view::place(image, zoom, viewport))
    }

    /// Loads the current image first, then the others one at a time for
    /// their thumbnails, and drops full images far from the current one
    /// unless they are being edited.
    fn load_next(&mut self) -> Task<Message> {
        if self
            .items
            .iter()
            .any(|item| matches!(item.state, ItemState::Loading))
        {
            return Task::none();
        }
        let current = self.current;
        let next = std::iter::once(current)
            .chain((1..=KEEP_AROUND).flat_map(|step| [current + step, current.wrapping_sub(step)]))
            .filter(|index| *index < self.items.len())
            .find(|index| matches!(self.items[*index].state, ItemState::Waiting))
            .or_else(|| {
                self.items.iter().position(|item| {
                    item.thumbnail.is_none() && matches!(item.state, ItemState::Waiting)
                })
            });
        for (index, item) in self.items.iter_mut().enumerate() {
            let editing = matches!(&item.state, ItemState::Loaded(shown) if shown.editor.is_some())
                || markup::keeps(item);
            if index.abs_diff(current) > KEEP_AROUND
                && matches!(item.state, ItemState::Loaded(_))
                && !editing
            {
                item.state = ItemState::Waiting;
            }
        }
        let Some(index) = next else {
            return Task::none();
        };
        let item = &mut self.items[index];
        item.state = ItemState::Loading;
        let (path, source) = (item.path.clone(), item.source);
        Task::perform(spawn(move || load(&path, source)), move |result| {
            Message::Loaded(
                index,
                result.unwrap_or_else(|_| Err(crate::fl!("image-loading-stopped"))),
            )
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleFloatingBars
            | Message::OpenSettings
            | Message::ToggleAssistant
            | Message::Side(_) => Task::none(),
            Message::Loaded(index, result) => {
                let near = index.abs_diff(self.current) <= KEEP_AROUND;
                let Some(item) = self.items.get_mut(index) else {
                    return Task::none();
                };
                item.state = match result {
                    Ok(image) => {
                        item.thumbnail = Some(image.thumbnail.clone());
                        item.size = Some((image.width, image.height));
                        if near {
                            ItemState::Loaded(Box::new(Shown::new(image)))
                        } else {
                            ItemState::Waiting
                        }
                    }
                    Err(error) => ItemState::Failed(error),
                };
                let frames = if index == self.current {
                    self.restart_animation()
                } else {
                    Task::none()
                };
                Task::batch([self.schedule(), self.load_next(), frames])
            }
            Message::LevelReady(index, level, handle) => match handle {
                Some(handle) => allocate(handle, move |allocation| {
                    Message::LevelAllocated(index, level, allocation)
                }),
                None => self.update(Message::LevelAllocated(index, level, None)),
            },
            Message::LevelAllocated(index, level, allocation) => {
                if let Some(shown) = self.shown_mut(index) {
                    shown.levels_pending.remove(&level);
                    if let Some(allocation) = allocation {
                        shown.levels.insert(level, allocation);
                    }
                }
                Task::none()
            }
            Message::SvgRendered(index, scale, handle) => match handle {
                Some(handle) => allocate(handle, move |allocation| {
                    Message::SvgAllocated(index, scale, allocation)
                }),
                None => self.update(Message::SvgAllocated(index, scale, None)),
            },
            Message::SvgAllocated(index, scale, allocation) => {
                if let Some(shown) = self.shown_mut(index) {
                    shown.render_pending = None;
                    if let Some(allocation) = allocation {
                        shown.render = Some((scale, allocation));
                    }
                }
                self.schedule()
            }
            Message::Canvas(event) => self.canvas_event(event),
            Message::Select(index) => self.select(index),
            Message::ToggleSidebar => {
                self.sidebar = !self.sidebar;
                Task::none()
            }
            Message::ZoomIn | Message::ZoomOut | Message::FitToWindow if self.marked() => {
                let zoom = match message {
                    Message::ZoomIn => Zoom::In,
                    Message::ZoomOut => Zoom::Out,
                    _ => Zoom::FitPage,
                };
                self.with_markup(|window, _| {
                    window.update(pdf_window::Message::Viewer(PdfMessage::Zoom(zoom)))
                })
                .unwrap_or_else(Task::none)
            }
            Message::ActualSize if self.marked() => {
                self.markup_zoom_to(1.0).unwrap_or_else(Task::none)
            }
            Message::ZoomTo(zoom) if self.marked() => {
                self.markup_zoom_to(zoom).unwrap_or_else(Task::none)
            }
            Message::ZoomIn => self.zoom_to(self.zoom() * ZOOM_STEP, None),
            Message::ZoomOut => self.zoom_to(self.zoom() / ZOOM_STEP, None),
            Message::ActualSize => self.zoom_to(1.0, None),
            Message::ZoomTo(zoom) => self.zoom_to(zoom, None),
            Message::Exporting(path, format, choice, scale) => {
                self.export_to(path, format, &choice, scale)
            }
            Message::FitToWindow => {
                self.fit = Fit::Fit;
                self.schedule()
            }
            Message::ToggleMarkup => self.toggle_markup(),
            Message::Undo | Message::Redo => {
                let redo = matches!(message, Message::Redo);
                match self.with_markup(|window, _| {
                    window.update(pdf_window::Message::Edit(if redo {
                        EditMessage::Redo
                    } else {
                        EditMessage::Undo
                    }))
                }) {
                    Some(task) => task,
                    None => self.edit(if redo { Edit::Redo } else { Edit::Undo }),
                }
            }
            Message::MarkupMade(index, scale, result) => self.markup_made(index, scale, result),
            Message::Markup(index, message) => self.markup_message(index, message),
            Message::MarkupExported(index, edits, result) => {
                self.markup_exported(index, edits, result)
            }
            Message::Close(choice) => self.close_choice(choice),
            Message::SidebarPressed(index) => self.sidebar_pressed(index),
            Message::SidebarMoved(point) => {
                self.sidebar_moved(point);
                Task::none()
            }
            Message::SidebarReleased => {
                self.sidebar_released();
                Task::none()
            }
            Message::PictureSaved(path) => self.picture_saved(path),
            Message::Edit(edit) => self.edit(edit),
            Message::PreviewRendered(index, generation, handle) => {
                let Some(shown) = self.shown_mut(index) else {
                    return Task::none();
                };
                shown.preview_running = false;
                let latest = shown.editor.as_ref().map_or(0, |editor| editor.generation);
                let wanted = std::mem::take(&mut shown.preview_wanted);
                let show = if generation == latest {
                    allocate(handle, move |allocation| {
                        Message::PreviewAllocated(index, generation, allocation)
                    })
                } else {
                    Task::none()
                };
                let next = if wanted {
                    self.render_preview(index)
                } else {
                    Task::none()
                };
                Task::batch([show, next])
            }
            Message::FrameAllocated(index, number, allocation) => {
                let Some(shown) = self.shown_mut(index) else {
                    return Task::none();
                };
                shown.frames_pending.remove(&number);
                if let Some(allocation) = allocation {
                    shown.frame_allocations.insert(number, allocation);
                }
                Task::none()
            }
            Message::PreviewAllocated(index, generation, allocation) => {
                let Some(shown) = self.shown_mut(index) else {
                    return Task::none();
                };
                let newer = shown
                    .preview
                    .as_ref()
                    .is_none_or(|(shown_generation, _)| generation > *shown_generation);
                if let (true, Some(allocation)) = (newer, allocation) {
                    shown.preview = Some((generation, allocation));
                }
                Task::none()
            }
            Message::FullRendered(index, generation, frame) => {
                // Kept and saved at once; shown once on the GPU, which a
                // window out of sight may not get to soon.
                let saving = self.full_kept(index, generation, &frame);
                let handle = frame_handle(&frame);
                let shown = allocate(handle, move |allocation| {
                    Message::FullAllocated(index, generation, frame.clone(), allocation)
                });
                Task::batch([saving, shown])
            }
            Message::FullAllocated(index, generation, frame, allocation) => {
                self.full_rendered(index, generation, frame, allocation)
            }
            Message::AutosaveDue(index, generation) => self.autosave(index, generation),
            Message::Saved(index, generation, result) => {
                match result {
                    Ok(()) => {
                        if let Some(editor) = self
                            .shown_mut(index)
                            .and_then(|shown| shown.editor.as_mut())
                        {
                            editor.saved_generation = editor.saved_generation.max(generation);
                            editor.original_kept = true;
                        }
                        return self.refresh_inspector();
                    }
                    Err(error) => self.notice = Some(error),
                }
                Task::none()
            }
            Message::TogglePanel(panel) => {
                let edits = matches!(panel, Panel::AdjustSize | Panel::AdjustColor);
                if edits && self.panel != Some(panel) && !self.editable() {
                    return Task::none();
                }
                self.panel = if self.panel == Some(panel) {
                    None
                } else {
                    Some(panel)
                };
                match self.panel {
                    Some(Panel::AdjustColor) => {
                        self.color = self
                            .shown()
                            .and_then(|shown| shown.editor.as_ref())
                            .map_or_else(ColorAdjust::default, |editor| {
                                editor.stack.current_color()
                            });
                        Task::none()
                    }
                    Some(Panel::AdjustSize) => {
                        let (width, height) = self.items[self.current].size.unwrap_or((0, 0));
                        self.size_input = (width.to_string(), height.to_string());
                        Task::none()
                    }
                    Some(Panel::Inspector) => self.refresh_inspector(),
                    None => Task::none(),
                }
            }
            Message::ToggleSelecting => {
                if !self.selecting && !self.editable() {
                    return Task::none();
                }
                self.selecting = !self.selecting;
                if !self.selecting {
                    self.selection = None;
                }
                Task::none()
            }
            Message::SizeWidth(value) => {
                self.size_input.0 = value.clone();
                if self.keep_proportions
                    && let (Ok(width), Some(size)) =
                        (value.trim().parse::<u32>(), self.items[self.current].size)
                {
                    self.size_input.1 = editor::proportional(size, Some(width), None).1.to_string();
                }
                Task::none()
            }
            Message::SizeHeight(value) => {
                self.size_input.1 = value.clone();
                if self.keep_proportions
                    && let (Ok(height), Some(size)) =
                        (value.trim().parse::<u32>(), self.items[self.current].size)
                {
                    self.size_input.0 =
                        editor::proportional(size, None, Some(height)).0.to_string();
                }
                Task::none()
            }
            Message::KeepProportions(keep) => {
                self.keep_proportions = keep;
                Task::none()
            }
            Message::InspectorLoaded(index, details, writable_xmp, versions) => {
                if index == self.current {
                    self.inspector = Some(Inspector {
                        index,
                        keywords: details.keywords.join(", "),
                        description: details.description.clone().unwrap_or_default(),
                        details,
                        writable_xmp,
                        versions,
                    });
                }
                Task::none()
            }
            Message::KeywordsChanged(value) => {
                if let Some(inspector) = &mut self.inspector {
                    inspector.keywords = value;
                }
                Task::none()
            }
            Message::DescriptionChanged(value) => {
                if let Some(inspector) = &mut self.inspector {
                    inspector.description = value;
                }
                Task::none()
            }
            Message::SaveMetadata => {
                let Some(inspector) = &self.inspector else {
                    return Task::none();
                };
                let keywords: Vec<String> = inspector
                    .keywords
                    .split(',')
                    .map(|keyword| keyword.trim().to_owned())
                    .filter(|keyword| !keyword.is_empty())
                    .collect();
                let description =
                    Some(inspector.description.trim().to_owned()).filter(|text| !text.is_empty());
                self.metadata_edit(move |bytes| {
                    metadata::set_description_and_keywords(bytes, description.as_deref(), &keywords)
                        .map_err(|error| error.describe())
                })
            }
            Message::RemoveLocation => self.metadata_edit(|bytes| {
                metadata::remove_location(bytes).map_err(|error| error.describe())
            }),
            Message::MetadataSaved(result) => {
                if let Err(error) = result {
                    self.notice = Some(error);
                }
                self.refresh_inspector()
            }
            Message::Revert(version) => {
                let path = self.items[self.current].path.clone();
                Task::perform(
                    spawn(move || {
                        let store = VersionStore::default_location()
                            .ok_or_else(|| crate::fl!("image-no-version-store"))?;
                        store.restore(&path, &version).map_err(|error| {
                            crate::fl!("image-revert-failed", error = error.to_string())
                        })
                    }),
                    |result| {
                        Message::Reverted(
                            result.unwrap_or_else(|_| Err(crate::fl!("image-reverting-stopped"))),
                        )
                    },
                )
            }
            Message::Reverted(result) => {
                if let Err(error) = result {
                    self.notice = Some(error);
                    return Task::none();
                }
                // Reload from disk and start editing afresh.
                self.items[self.current].state = ItemState::Waiting;
                self.selection = None;
                Task::batch([self.load_next(), self.refresh_inspector()])
            }
            Message::ToggleOverflow => {
                self.overflow_open = !self.overflow_open;
                Task::none()
            }
            Message::CloseOverflow => {
                self.overflow_open = false;
                Task::none()
            }
            Message::SidebarResized(drag) => {
                self.sidebar_width.drag(drag);
                Task::none()
            }
            Message::DismissNotice => {
                self.notice = None;
                Task::none()
            }
            Message::Export => self.export(),
            Message::ExportTarget(Ok(Some((chosen, _)))) => {
                let Some(choice) = self.export_choice.clone() else {
                    return Task::none();
                };
                let format_choice = choice.format;
                let quality_choice = choice.quality;
                let Some(format) = editor::format_for_choice(&format_choice, &quality_choice)
                else {
                    return Task::none();
                };
                let svg = matches!(self.items[self.current].source, Source::Svg);
                let scale = svg.then(|| editor::svg_scale_for_choice(&choice.size));
                let pending = PendingExport {
                    path: chosen,
                    format,
                    format_choice,
                    scale,
                };
                if editor::extension_matches(&pending.path, format) {
                    return self.run_export(pending);
                }
                // Ask before saving under a name that does not fit the format.
                self.pending_export = Some(pending);
                Task::none()
            }
            Message::ExportFormat(format) => {
                if let Some(choice) = &mut self.export_choice {
                    choice.format = format.to_owned();
                }
                Task::none()
            }
            Message::ExportQuality(quality) => {
                if let Some(choice) = &mut self.export_choice {
                    choice.quality = quality.to_owned();
                }
                Task::none()
            }
            Message::ExportSize(size) => {
                if let Some(choice) = &mut self.export_choice {
                    choice.size = size.to_owned();
                }
                Task::none()
            }
            Message::ExportCancel => {
                if let Some(choice) = &mut self.export_choice {
                    choice.open = false;
                }
                Task::none()
            }
            Message::ExportChoose => {
                if let Some(choice) = &mut self.export_choice {
                    choice.open = false;
                }
                self.open_export_dialog(None, None)
            }
            Message::ExportAsIs => match self.pending_export.take() {
                Some(pending) => self.run_export(pending),
                None => Task::none(),
            },
            Message::ExportChooseAgain => match self.pending_export.take() {
                Some(pending) => self.open_export_dialog(
                    pending
                        .path
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned()),
                    pending.path.parent().map(Path::to_path_buf),
                ),
                None => Task::none(),
            },
            Message::ExportTarget(Ok(None)) => Task::none(),
            Message::ExportTarget(Err(error)) => {
                self.notice = Some(crate::fl!(
                    "export-dialog-failed",
                    error = error.to_string()
                ));
                Task::none()
            }
            Message::Exported(result) => {
                self.notice = Some(match result {
                    Ok(path) => crate::fl!("export-done", path = path.display().to_string()),
                    Err(error) => error,
                });
                Task::none()
            }
        }
    }

    fn zoom(&self) -> f32 {
        self.placement().map_or(1.0, |placement| placement.zoom)
    }

    fn select(&mut self, index: usize) -> Task<Message> {
        if index >= self.items.len() || index == self.current {
            return Task::none();
        }
        self.current = index;
        self.fit = Fit::Fit;
        self.selection = None;
        let frames = self.restart_animation();
        // Sidebar entries share one height, so scrolling proportionally keeps
        // the current one fully in view.
        let fraction = index as f32 / (self.items.len() - 1).max(1) as f32;
        let follow = operation::snap_to(
            self.sidebar_id.clone(),
            scrollable::RelativeOffset {
                x: 0.0,
                y: fraction,
            },
        );
        Task::batch([
            self.load_next(),
            self.schedule(),
            follow,
            self.refresh_inspector(),
            frames,
        ])
    }

    fn restart_animation(&mut self) -> Task<Message> {
        self.frame = 0;
        self.next_frame = self
            .shown()
            .filter(|shown| shown.image.frames.len() > 1)
            .map(|shown| Instant::now() + shown.image.frames[0].1);
        self.upload_frames()
    }

    /// Uploads the frames from the one showing to FRAMES_AHEAD after it,
    /// and lets go of the others.
    fn upload_frames(&mut self) -> Task<Message> {
        let (index, frame) = (self.current, self.frame);
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        let count = shown.image.frames.len();
        if count < 2 {
            return Task::none();
        }
        let wanted: Vec<usize> = (0..=FRAMES_AHEAD.min(count - 1))
            .map(|ahead| (frame + ahead) % count)
            .collect();
        shown
            .frame_allocations
            .retain(|number, _| wanted.contains(number));
        let tasks: Vec<Task<Message>> = wanted
            .into_iter()
            .filter(|number| {
                !shown.frame_allocations.contains_key(number)
                    && shown.frames_pending.insert(*number)
            })
            .map(|number| {
                let handle = shown.image.frames[number].0.clone();
                allocate(handle, move |allocation| {
                    Message::FrameAllocated(index, number, allocation)
                })
            })
            .collect();
        Task::batch(tasks)
    }

    fn canvas_event(&mut self, event: CanvasEvent) -> Task<Message> {
        match event {
            CanvasEvent::ViewChanged {
                x,
                y,
                width,
                height,
            } => {
                self.view = (x, y, width, height);
                self.schedule()
            }
            CanvasEvent::ZoomBy { factor, anchor } => {
                self.zoom_to(self.zoom() * factor, Some(anchor))
            }
            CanvasEvent::Pan { dx, dy } => operation::scroll_by(
                self.canvas_id.clone(),
                scrollable::AbsoluteOffset { x: dx, y: dy },
            ),
            CanvasEvent::Selection(selection) => {
                self.selection = selection;
                Task::none()
            }
            CanvasEvent::Tick(now) => {
                let Some(shown) = self.shown() else {
                    return Task::none();
                };
                let frames = shown.image.frames.len();
                if frames < 2 {
                    return Task::none();
                }
                let next = (self.frame + 1) % frames;
                // A frame not yet on the GPU would draw as nothing; keep the
                // one showing until it is.
                if !shown.frame_allocations.contains_key(&next) {
                    return self.upload_frames();
                }
                let delay = shown.image.frames[next].1;
                self.frame = next;
                let due = self.next_frame.map_or(now, |due| due + delay);
                // Skip ahead rather than race if we fell behind.
                self.next_frame = Some(if due < now { now + delay } else { due });
                self.upload_frames()
            }
        }
    }

    /// Zooms keeping the image point under `anchor` (view coordinates, the
    /// view centre by default) in place.
    fn zoom_to(&mut self, zoom: f32, anchor: Option<(f32, f32)>) -> Task<Message> {
        let Some(before) = self.placement() else {
            return Task::none();
        };
        let anchor = anchor.unwrap_or((self.view.2 / 2.0, self.view.3 / 2.0));
        let fraction =
            view::image_fraction(&before, self.view.0 + anchor.0, self.view.1 + anchor.1);
        self.fit = Fit::Zoom(zoom.clamp(view::MIN_ZOOM, view::MAX_ZOOM));
        let Some(after) = self.placement() else {
            return Task::none();
        };
        let (x, y) = view::fraction_to_content(&after, fraction);
        let offset = scrollable::AbsoluteOffset {
            x: Some((x - anchor.0).clamp(0.0, (after.content.0 - self.view.2).max(0.0))),
            y: Some((y - anchor.1).clamp(0.0, (after.content.1 - self.view.3).max(0.0))),
        };
        Task::batch([
            operation::scroll_to(self.canvas_id.clone(), offset),
            self.schedule(),
        ])
    }

    /// The zoom and the point of the image in the middle of the view, as
    /// fractions of its size, unless the image is fitted to the window.
    fn zoomed_center(&self) -> Option<(f32, (f32, f32))> {
        if !matches!(self.fit, Fit::Zoom(_)) {
            return None;
        }
        let placement = self.placement()?;
        let fraction = view::image_fraction(
            &placement,
            self.view.0 + self.view.2 / 2.0,
            self.view.1 + self.view.3 / 2.0,
        );
        Some((placement.zoom, fraction))
    }

    /// Shows the image at `zoom` with the point at `fraction` of its size in
    /// the middle of the view, or fitted to the window without one.
    fn show_centered(&mut self, center: Option<(f32, (f32, f32))>) -> Task<Message> {
        let Some((zoom, fraction)) = center else {
            self.fit = Fit::Fit;
            return self.schedule();
        };
        self.fit = Fit::Zoom(zoom.clamp(view::MIN_ZOOM, view::MAX_ZOOM));
        let Some(placement) = self.placement() else {
            return self.schedule();
        };
        let (x, y) = view::fraction_to_content(&placement, fraction);
        let x = (x - self.view.2 / 2.0).clamp(0.0, (placement.content.0 - self.view.2).max(0.0));
        let y = (y - self.view.3 / 2.0).clamp(0.0, (placement.content.1 - self.view.3).max(0.0));
        self.view.0 = x;
        self.view.1 = y;
        let offset = scrollable::AbsoluteOffset {
            x: Some(x),
            y: Some(y),
        };
        Task::batch([
            operation::scroll_to(self.canvas_id.clone(), offset),
            self.schedule(),
        ])
    }

    /// Makes the downscaled copy or SVG render the current view needs.
    fn schedule(&mut self) -> Task<Message> {
        let Some(placement) = self.placement() else {
            return Task::none();
        };
        let index = self.current;
        let device_scale = self.device_scale;
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        if let Some(svg) = &shown.image.svg {
            let wanted = svg.effective_scale(placement.zoom * device_scale);
            let current = shown
                .render
                .as_ref()
                .map(|(scale, _)| *scale)
                .or(shown.render_pending);
            let stale = current.is_none_or(|scale| (scale - wanted).abs() / wanted > 0.05);
            if !stale {
                return Task::none();
            }
            shown.render_pending = Some(wanted);
            let svg = svg.clone();
            return Task::perform(spawn(move || svg.render(wanted)), move |result| {
                let handle = result
                    .ok()
                    .and_then(Result::ok)
                    .map(|frame| frame_handle(&frame));
                Message::SvgRendered(index, wanted, handle)
            });
        }
        if shown.image.frames.len() != 1 {
            return Task::none();
        }
        let level = view::downscale_level(shown.image.width, placement.image.width * device_scale);
        if level == 1 || shown.levels.contains_key(&level) || !shown.levels_pending.insert(level) {
            return Task::none();
        }
        let Some(full) = shown.image.full.clone() else {
            return Task::none();
        };
        let (width, height) = (shown.image.width, shown.image.height);
        Task::perform(
            spawn(move || downscale(&full, width, height, level)),
            move |result| Message::LevelReady(index, level, result.ok().flatten()),
        )
    }

    /// The handle to draw: a newer edit preview, the animation frame, the
    /// best downscaled copy, the SVG render, or the full image.
    fn display_handle(&self) -> Option<&Handle> {
        let shown = self.shown()?;
        if let Some((generation, allocation)) = &shown.preview
            && *generation > shown.shown_generation
        {
            return Some(allocation.handle());
        }
        if shown.image.svg.is_some() {
            return shown
                .render
                .as_ref()
                .map(|(_, allocation)| allocation.handle())
                .or(Some(&shown.image.thumbnail));
        }
        if shown.image.frames.len() > 1 {
            return shown.image.frames.get(self.frame).map(|(handle, _)| handle);
        }
        let placement = self.placement()?;
        let level =
            view::downscale_level(shown.image.width, placement.image.width * self.device_scale);
        shown
            .levels
            .get(&level)
            .map(Allocation::handle)
            .or_else(|| shown.image.frames.first().map(|(handle, _)| handle))
    }

    // Editing.

    fn edit(&mut self, edit: Edit) -> Task<Message> {
        match edit {
            Edit::Color(adjust) => self.color = adjust,
            Edit::ResetColor => self.color = ColorAdjust::default(),
            _ => {}
        }
        if self.marked() {
            self.notice = Some(crate::fl!("image-marked-no-edit"));
            return Task::none();
        }
        let index = self.current;
        let selection = self.selection;
        let size_input = self.size_input.clone();
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        if !shown.is_editable() {
            self.notice = Some(crate::fl!("image-cannot-edit"));
            return Task::none();
        }
        if shown.editor.is_none() {
            let full = shown
                .image
                .full
                .clone()
                .expect("editable images have pixels");
            let frame = Frame {
                width: shown.image.width,
                height: shown.image.height,
                pixels: full.to_vec(),
                delay: Duration::ZERO,
            };
            shown.editor = Some(Editor::new(frame));
        }
        let editor = shown.editor.as_mut().expect("editor was just created");
        let size = editor.output_size();
        let changed = match edit {
            Edit::RotateLeft => editor.change(|stack| {
                stack.push(Operation::Rotate(3));
                true
            }),
            Edit::RotateRight => editor.change(|stack| {
                stack.push(Operation::Rotate(1));
                true
            }),
            Edit::FlipHorizontal => editor.change(|stack| {
                stack.push(Operation::FlipHorizontal);
                true
            }),
            Edit::FlipVertical => editor.change(|stack| {
                stack.push(Operation::FlipVertical);
                true
            }),
            Edit::Crop => {
                match selection.and_then(|selection| editor::selection_to_crop(selection, size)) {
                    Some(crop) => editor.change(|stack| {
                        stack.push(Operation::Crop(crop));
                        true
                    }),
                    None => {
                        self.notice = Some(crate::fl!("image-crop-needs-selection"));
                        return Task::none();
                    }
                }
            }
            Edit::Undo => editor.change(|stack| stack.undo()),
            Edit::Redo => editor.change(|stack| stack.redo()),
            Edit::Color(adjust) => {
                let changed = editor.change(|stack| {
                    if stack.current_color() == adjust {
                        return false;
                    }
                    stack.set_color(adjust);
                    true
                });
                // Sliders only preview; the full render waits for release.
                return if changed {
                    self.render_preview(index)
                } else {
                    Task::none()
                };
            }
            Edit::ColorCommitted => true,
            Edit::Apply(Operation::Color(adjust)) => editor.change(|stack| {
                if stack.current_color() == adjust {
                    return false;
                }
                stack.set_color(adjust);
                true
            }),
            Edit::Apply(operation) => editor.change(|stack| {
                stack.push(operation);
                true
            }),
            Edit::ResetColor => editor.change(|stack| {
                if stack.current_color().is_identity() {
                    return false;
                }
                stack.set_color(ColorAdjust::default());
                true
            }),
            Edit::ApplySize => {
                let parsed = (
                    size_input.0.trim().parse::<u32>(),
                    size_input.1.trim().parse::<u32>(),
                );
                let (Ok(width), Ok(height)) = parsed else {
                    self.notice = Some(crate::fl!("image-size-needed"));
                    return Task::none();
                };
                if (width, height) == size
                    || width == 0
                    || height == 0
                    || width > 65_535
                    || height > 65_535
                {
                    return Task::none();
                }
                editor.change(|stack| {
                    stack.push(Operation::Resize { width, height });
                    true
                })
            }
        };
        if !changed {
            return Task::none();
        }
        let output = editor.output_size();
        self.items[index].size = Some(output);
        if matches!(
            edit,
            Edit::Crop
                | Edit::RotateLeft
                | Edit::RotateRight
                | Edit::Undo
                | Edit::Redo
                | Edit::Apply(_)
        ) {
            self.selection = None;
            self.selecting = false;
        }
        if matches!(
            edit,
            Edit::Undo | Edit::Redo | Edit::Apply(Operation::Color(_))
        ) {
            self.color = self
                .shown()
                .and_then(|shown| shown.editor.as_ref())
                .map_or_else(ColorAdjust::default, |editor| editor.stack.current_color());
        }
        Task::batch([
            self.render_preview(index),
            self.render_full(index),
            self.schedule(),
        ])
    }

    fn render_preview(&mut self, index: usize) -> Task<Message> {
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        let Some(editor) = &shown.editor else {
            return Task::none();
        };
        if shown.preview_running {
            shown.preview_wanted = true;
            return Task::none();
        }
        shown.preview_running = true;
        let generation = editor.generation;
        let job = editor.render_preview();
        Task::perform(
            spawn(move || frame_handle(&job())),
            move |result| match result {
                Ok(handle) => Message::PreviewRendered(index, generation, handle),
                Err(_) => Message::PreviewRendered(index, 0, Handle::from_rgba(1, 1, vec![0; 4])),
            },
        )
    }

    fn render_full(&self, index: usize) -> Task<Message> {
        let Some(editor) = self.items.get(index).and_then(|item| match &item.state {
            ItemState::Loaded(shown) => shown.editor.as_ref(),
            _ => None,
        }) else {
            return Task::none();
        };
        let generation = editor.generation;
        let job = editor.render_full();
        Task::perform(spawn(move || Arc::new(job())), move |result| match result {
            Ok(frame) => Message::FullRendered(index, generation, frame),
            Err(_) => Message::Saved(
                index,
                generation,
                Err(crate::fl!("image-rendering-stopped")),
            ),
        })
    }

    /// Keeps a finished full-size render as the edited image, which the
    /// autosave writes.
    fn full_kept(&mut self, index: usize, generation: u64, frame: &Arc<Frame>) -> Task<Message> {
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        let latest = shown.editor.as_ref().map_or(0, |editor| editor.generation);
        if generation != latest {
            return Task::none();
        }
        shown.edited = Some(Arc::clone(frame));
        shown.kept_generation = generation;
        self.items[index].thumbnail = Some(thumbnail_of(frame));
        self.items[index].size = Some((frame.width, frame.height));
        Task::perform(
            spawn(move || std::thread::sleep(AUTOSAVE_DELAY)),
            move |_| Message::AutosaveDue(index, generation),
        )
    }

    /// Swaps in a finished full-size render once it is on the GPU, so the
    /// view never shows a frame without an image.
    fn full_rendered(
        &mut self,
        index: usize,
        generation: u64,
        frame: Arc<Frame>,
        allocation: Option<Allocation>,
    ) -> Task<Message> {
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        let latest = shown.editor.as_ref().map_or(0, |editor| editor.generation);
        if generation != latest {
            return Task::none();
        }
        let pixels = Bytes::from(frame.pixels.clone());
        let handle = allocation.as_ref().map_or_else(
            || rgba_handle(frame.width, frame.height, pixels.clone()),
            |allocation| allocation.handle().clone(),
        );
        shown.full_allocation = allocation;
        shown.image.width = frame.width;
        shown.image.height = frame.height;
        shown.image.frames = vec![(handle, Duration::ZERO)];
        shown.image.full = Some(pixels);
        shown.levels.clear();
        shown.levels_pending.clear();
        shown.shown_generation = generation;
        self.schedule()
    }

    fn autosave(&mut self, index: usize, generation: u64) -> Task<Message> {
        let Some(item) = self.items.get(index) else {
            return Task::none();
        };
        let path = item.path.clone();
        let Source::Raster(format) = item.source else {
            return Task::none();
        };
        let ItemState::Loaded(shown) = &item.state else {
            return Task::none();
        };
        let (Some(editor), Some(frame)) = (&shown.editor, &shown.edited) else {
            return Task::none();
        };
        if editor.generation != generation || editor.is_saved() {
            return Task::none();
        }
        let Some(save_format) = SaveFormat::for_saving(format, false) else {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            self.notice = Some(match crate::shortcuts::label(Action::Export) {
                Some(keys) => crate::fl!("image-cannot-save-format", name = name, keys = keys),
                None => crate::fl!("image-cannot-save-format-unbound", name = name),
            });
            return Task::none();
        };
        let keep_original = !editor.original_kept;
        let frame = Arc::clone(frame);
        Task::perform(
            spawn(move || editor::save_in_place(&path, &frame, save_format, keep_original)),
            move |result| {
                Message::Saved(
                    index,
                    generation,
                    result.unwrap_or_else(|_| Err(crate::fl!("image-saving-stopped"))),
                )
            },
        )
    }

    fn metadata_edit(
        &mut self,
        edit: impl FnOnce(Vec<u8>) -> Result<Vec<u8>, String> + Send + 'static,
    ) -> Task<Message> {
        let index = self.current;
        let path = self.items[index].path.clone();
        let keep_original = self
            .shown()
            .and_then(|shown| shown.editor.as_ref())
            .is_none_or(|editor| !editor.original_kept);
        if let Some(editor) = self
            .shown_mut(index)
            .and_then(|shown| shown.editor.as_mut())
        {
            editor.original_kept = true;
        }
        Task::perform(
            spawn(move || editor::edit_metadata(&path, keep_original, edit)),
            |result| {
                Message::MetadataSaved(
                    result.unwrap_or_else(|_| Err(crate::fl!("image-saving-stopped"))),
                )
            },
        )
    }

    fn refresh_inspector(&mut self) -> Task<Message> {
        if self.panel != Some(Panel::Inspector) {
            return Task::none();
        }
        let index = self.current;
        let path = self.items[index].path.clone();
        Task::perform(
            spawn(move || {
                let bytes = std::fs::read(&path).unwrap_or_default();
                let versions = VersionStore::default_location()
                    .map(|store| store.list(&path))
                    .unwrap_or_default();
                (
                    metadata::read(&bytes),
                    metadata::supports_xmp(&bytes),
                    versions,
                )
            }),
            move |result| {
                let (details, writable, versions) = result.unwrap_or_default();
                Message::InspectorLoaded(index, details, writable, versions)
            },
        )
    }

    fn export(&mut self) -> Task<Message> {
        let Some(shown) = self.shown() else {
            return Task::none();
        };
        let format = match self.items[self.current].source {
            Source::Svg => "png",
            Source::Raster(_) if !shown.is_editable() => {
                self.notice = Some(crate::fl!("image-cannot-export-animation"));
                return Task::none();
            }
            Source::Raster(format) => editor::default_format_choice(format, false),
        };
        let choice = self.export_choice.get_or_insert_with(|| ExportChoice {
            format: format.to_owned(),
            quality: editor::DEFAULT_QUALITY_CHOICE.to_owned(),
            size: editor::SVG_SIZES[0].0.to_owned(),
            open: false,
        });
        choice.open = true;
        Task::none()
    }

    /// Opens the save dialog, suggesting a name with the chosen format's
    /// extension, or `name` in `folder` when choosing again.
    fn open_export_dialog(
        &mut self,
        name: Option<String>,
        folder: Option<PathBuf>,
    ) -> Task<Message> {
        let format = self
            .export_choice
            .as_ref()
            .map_or("png", |choice| choice.format.as_str());
        let name =
            name.unwrap_or_else(|| editor::export_name(&self.items[self.current].path, format));
        Task::perform(
            dialog::save_file_with_menus(crate::fl!("export-title"), name, folder, Vec::new()),
            Message::ExportTarget,
        )
    }

    fn run_export(&mut self, pending: PendingExport) -> Task<Message> {
        if let Some(svg) = self.shown().and_then(|shown| shown.image.svg.clone()) {
            let original = self.items[self.current].path.clone();
            let PendingExport {
                path,
                format,
                scale,
                ..
            } = pending;
            let scale = scale.unwrap_or(1.0);
            return Task::perform(
                spawn(move || {
                    let frame = svg.render(scale).map_err(|error| error.describe())?;
                    editor::export(&original, &frame, &path, format).map(|()| path)
                }),
                |result| {
                    Message::Exported(result.unwrap_or_else(|_| Err(crate::fl!("export-stopped"))))
                },
            );
        }
        let Some(frame) = self.shown().and_then(Shown::current_frame) else {
            return Task::none();
        };
        if self
            .markup()
            .is_some_and(|markup| markup.window.has_annotations())
        {
            return self
                .export_marked(pending, frame)
                .unwrap_or_else(Task::none);
        }
        let original = self.items[self.current].path.clone();
        let PendingExport { path, format, .. } = pending;
        Task::perform(
            spawn(move || editor::export(&original, &frame, &path, format).map(|()| path)),
            |result| {
                Message::Exported(result.unwrap_or_else(|_| Err(crate::fl!("export-stopped"))))
            },
        )
    }

    /// Whether the image shown can be edited: not an SVG drawing or an
    /// animation.
    fn editable(&self) -> bool {
        self.shown().is_some_and(Shown::is_editable)
    }

    pub fn shortcut(&mut self, action: Action) -> Option<Task<Message>> {
        if action == Action::Escape && self.close_prompt {
            return Some(self.close_choice(CloseChoice::Cancel));
        }
        if action == Action::ShowMarkup {
            return Some(self.toggle_markup());
        }
        if action == Action::Paste && self.items[self.current].markup.is_none() {
            return Some(self.paste_into_new_markup());
        }
        if action == Action::ActualSize && self.marked() {
            return self.markup_zoom_to(1.0);
        }
        if self.marked() {
            let index = self.current;
            if let Some(markup) = self.items[index].markup.as_mut()
                && let Some(task) = markup.window.shortcut(action)
            {
                markup.window.take_effects();
                return Some(task.map(markup::wrap(index)));
            }
            // Undo and redo belong to the markup while there is some.
            if matches!(action, Action::Undo | Action::Redo) {
                return Some(Task::none());
            }
        }
        let task = match action {
            Action::ZoomIn => self.update(Message::ZoomIn),
            Action::ZoomOut => self.update(Message::ZoomOut),
            Action::ActualSize => self.update(Message::ActualSize),
            Action::ZoomToFit => self.update(Message::FitToWindow),
            Action::HideSidebar => {
                self.sidebar = false;
                Task::none()
            }
            Action::Thumbnails => {
                self.sidebar = true;
                Task::none()
            }
            Action::NextPage => self.select(self.current + 1),
            Action::PreviousPage => self.select(self.current.saturating_sub(1)),
            Action::FirstPage => self.select(0),
            Action::LastPage => self.select(self.items.len() - 1),
            Action::RotateLeft => self.edit(Edit::RotateLeft),
            Action::RotateRight => self.edit(Edit::RotateRight),
            Action::Crop => self.edit(Edit::Crop),
            Action::Undo => self.edit(Edit::Undo),
            Action::Redo => self.edit(Edit::Redo),
            Action::AdjustColor => self.update(Message::TogglePanel(Panel::AdjustColor)),
            Action::Inspector => self.update(Message::TogglePanel(Panel::Inspector)),
            Action::Export => self.export(),
            Action::Escape if self.export_dialog_open() => self.update(Message::ExportCancel),
            Action::Escape if self.overflow_open => {
                self.overflow_open = false;
                Task::none()
            }
            Action::Escape if self.selection.is_some() || self.selecting => {
                self.selection = None;
                self.selecting = false;
                Task::none()
            }
            Action::Escape if self.panel.is_some() => {
                self.panel = None;
                Task::none()
            }
            _ => return None,
        };
        Some(task)
    }

    /// Arrow keys step through the images; Delete removes selected markup.
    pub fn key(&mut self, key: &Key, modifiers: Modifiers) -> Option<Task<Message>> {
        let index = self.current;
        if let Some(markup) = self.items[index].markup.as_mut()
            && let Some(task) = markup.window.key(key, modifiers)
        {
            return Some(task.map(markup::wrap(index)));
        }
        if modifiers.control() || modifiers.alt() || modifiers.logo() {
            return None;
        }
        match key.as_ref() {
            Key::Named(Named::ArrowRight | Named::ArrowDown | Named::PageDown) => {
                Some(self.select(self.current + 1))
            }
            Key::Named(Named::ArrowLeft | Named::ArrowUp | Named::PageUp) => {
                Some(self.select(self.current.saturating_sub(1)))
            }
            _ => None,
        }
    }

    // Views.

    pub fn set_pointer_inside(&mut self, inside: bool) {
        self.pointer_inside = inside;
        self.markup_pointer(inside);
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.view_with(None)
    }

    /// The window, with `side` in the place of a panel on the right: below
    /// the toolbar, as the inspector is.
    pub fn view_with<'a>(&'a self, side: Option<Element<'a, Message>>) -> Element<'a, Message> {
        let index = self.current;
        if let Some(parts) = self.markup_parts() {
            let wrap = markup::wrap(index);
            let bar = parts.bar.map(|bar| bar.map(wrap.clone()));
            return self.frame_view(
                parts.canvas.map(wrap.clone()),
                bar,
                parts.overlay.map(wrap),
                side,
            );
        }
        let canvas: Element<'_, Message> = match &self.items[self.current].state {
            ItemState::Failed(error) => component::empty_state(
                Icon::BrokenImage,
                crate::fl!("image-open-failed"),
                error.as_str(),
            ),
            _ => match self.placement() {
                Some(placement) => scroll(
                    ImageCanvas::new(
                        self.display_handle(),
                        placement,
                        self.next_frame,
                        Message::Canvas,
                    )
                    .selection(
                        self.selecting && self.editable(),
                        self.selection.filter(|_| self.editable()),
                    ),
                )
                .id(self.canvas_id.clone())
                .direction(Direction::Both {
                    vertical: component::thin_scrollbar(),
                    horizontal: component::thin_scrollbar(),
                })
                .style(style::scrollbar)
                .width(Fill)
                .height(Fill)
                .into(),
                None => component::empty_state(Icon::Image, crate::fl!("image-opening"), ""),
            },
        };
        self.frame_view(canvas, None, space().into(), side)
    }

    /// The window around `canvas`: bars, sidebar, panel, notices and
    /// dialogs, with `overlay` on top.
    fn frame_view<'a>(
        &'a self,
        canvas: Element<'a, Message>,
        markup_bar: Option<Element<'a, Message>>,
        overlay: Element<'a, Message>,
        side: Option<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        let bottom = markup_bar.is_some();
        // The panels keep their sides in every language.
        let mut content = iced::widget::Row::new().height(Fill);
        if self.sidebar && self.items.len() > 1 {
            content = content
                .push(below_bars(
                    ui::enter::from_left(
                        container(self.sidebar_view())
                            .clip(true)
                            .width(self.sidebar_width.value)
                            .height(Fill)
                            .style(style::chrome),
                    ),
                    bottom,
                ))
                .push(below_bars(
                    resize::handle(Message::SidebarResized).into(),
                    bottom,
                ));
        }
        content = content.push(canvas);
        if let Some(panel) = self.panel {
            content = content.push(below_bars(self.panel_view(panel), bottom));
        }
        if let Some(side) = side {
            content = content.push(below_bars(side, bottom));
        }
        let shown = self.pointer_inside
            || self.overflow_open
            || self.pending_export.is_some()
            || self.export_dialog_open()
            || self.close_prompt
            || self.markup_holds_bars();
        let page = container(component::window_bars(
            self.toolbar(),
            markup_bar,
            content.into(),
            shown,
        ))
        .width(Fill)
        .height(Fill)
        .style(style::surface);
        let page: Element<'_, Message> = match &self.notice {
            Some(notice) => {
                component::snackbar_above_bar(page, notice, Message::DismissNotice, bottom)
            }
            None => page.into(),
        };
        let page = match &self.pending_export {
            Some(pending) => self.export_prompt(page, pending),
            None => page,
        };
        let page = match self.export_choice.as_ref().filter(|choice| choice.open) {
            Some(choice) => self.export_dialog_view(page, choice),
            None => page,
        };
        let page = if self.close_prompt {
            self.close_prompt_view(page)
        } else {
            page
        };
        iced::widget::stack![page, overlay].into()
    }

    fn export_dialog_open(&self) -> bool {
        self.export_choice
            .as_ref()
            .is_some_and(|choice| choice.open)
    }

    /// prev's export dialog: the format, the JPEG quality and, for SVG
    /// drawings, the size.
    fn export_dialog_view<'a>(
        &'a self,
        page: Element<'a, Message>,
        choice: &'a ExportChoice,
    ) -> Element<'a, Message> {
        let sizes = self
            .shown()
            .and_then(|shown| shown.image.svg.as_ref())
            .map(|svg| {
                let scale = svg.effective_scale(editor::svg_scale_for_choice(&choice.size));
                let (width, height) = svg.size();
                ui::export::Sizes {
                    options: editor::SVG_SIZES,
                    chosen: &choice.size,
                    note: crate::fl!(
                        "export-size-pixels",
                        width = ((width * scale).round() as u32),
                        height = ((height * scale).round() as u32)
                    ),
                }
            });
        ui::export::dialog(
            page,
            ui::export::Dialog {
                format: &choice.format,
                quality: &choice.quality,
                sizes,
                on_format: Message::ExportFormat,
                on_quality: Message::ExportQuality,
                on_size: Message::ExportSize,
                on_cancel: Message::ExportCancel,
                on_choose: Message::ExportChoose,
            },
        )
    }

    fn export_prompt<'a>(
        &'a self,
        page: Element<'a, Message>,
        pending: &'a PendingExport,
    ) -> Element<'a, Message> {
        let name = pending
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let format = editor::format_label(&pending.format_choice);
        let body = match pending.path.extension() {
            Some(extension) => crate::fl!(
                "image-name-mismatch",
                name = name,
                format = format,
                extension = extension.to_string_lossy().into_owned()
            ),
            None => crate::fl!(
                "image-name-mismatch-no-extension",
                name = name,
                format = format
            ),
        };
        component::dialog(
            page,
            Some(Icon::Warning),
            crate::fl!("image-name-mismatch-title"),
            body,
            vec![
                ui::button(Kind::Text, crate::fl!("image-choose-again"))
                    .on_press(Message::ExportChooseAgain)
                    .into(),
                ui::button(Kind::Text, crate::fl!("image-save-as-is"))
                    .on_press(Message::ExportAsIs)
                    .into(),
            ],
        )
    }

    /// The toolbar, with groups that do not fit the window's width in a
    /// "More" menu at its end.
    fn toolbar(&self) -> Element<'_, Message> {
        component::toolbar(iced::widget::responsive(move |size| {
            self.toolbar_at(size.width)
        }))
    }

    fn toolbar_at<'a>(&'a self, width: f32) -> Element<'a, Message> {
        let _fixed = crate::ui::dir::fixed();
        let item = &self.items[self.current];
        let mut details = item
            .size
            .map(|(width, height)| crate::fl!("image-dimensions", width = width, height = height))
            .unwrap_or_default();
        let shown = self.shown();
        if let Some(shown) = shown
            && shown.image.frames.len() > 1
        {
            details.push_str(DETAILS_SEPARATOR);
            details.push_str(&crate::fl!(
                "image-frame-position",
                current = (self.frame + 1),
                total = shown.image.frames.len()
            ));
        }
        if self.items.len() > 1 {
            details.push_str(DETAILS_SEPARATOR);
            details.push_str(&crate::fl!(
                "image-position",
                current = (self.current + 1),
                total = self.items.len()
            ));
        }
        if let Some(editor) = shown.and_then(|shown| shown.editor.as_ref())
            && !editor.is_saved()
        {
            details.push_str(DETAILS_SEPARATOR);
            details.push_str(&crate::fl!("image-edited"));
        }
        let markable = shown.is_some_and(Shown::is_editable);
        let marked = self.marked();
        let editable = markable && !marked;
        let editor = shown
            .and_then(|shown| shown.editor.as_ref())
            .filter(|_| !marked);
        let when = |glyph: Icon, label: String, enabled: bool, message: Message| {
            component::tool(glyph, label, enabled.then_some(message))
        };
        let panel = |glyph: Icon, label: String, which: Panel| {
            component::toggle_tool(
                glyph,
                label,
                self.panel == Some(which),
                Message::TogglePanel(which),
            )
        };
        // Tools that change pixels, for images that can be edited.
        let edit_panel = |glyph: Icon, label: String, which: Panel| {
            if markable {
                panel(glyph, label, which)
            } else {
                component::tool(glyph, label, None)
            }
        };
        let is_svg = matches!(self.items[self.current].source, Source::Svg);
        use component::{DIVIDER_WIDTH, TOOL_WIDTH};
        let tools = |count: f32| count * TOOL_WIDTH + (count - 1.0) * 4.0;
        // Each slot: its content, its width, when it moves into "More", and
        // whether a divider goes before it in the bar.
        let mut slots: Vec<(Element<'_, Message>, f32, Option<u8>, bool)> = Vec::new();
        if self.items.len() > 1 {
            slots.push((
                row![
                    component::toggle_tool(
                        if self.sidebar {
                            Icon::LeftPanelClose
                        } else {
                            Icon::LeftPanelOpen
                        },
                        crate::fl!("image-sidebar"),
                        self.sidebar,
                        Message::ToggleSidebar,
                    ),
                    component::toolbar_divider()
                ]
                .spacing(8)
                .align_y(Center)
                .into(),
                TOOL_WIDTH + DIVIDER_WIDTH + 8.0,
                None,
                false,
            ));
        }
        // As in the PDF toolbar: what is shown and the view on the left,
        // editing, panels and export on the right.
        let details_width = ui::font::measure(&details, Type::BodyMedium).ceil() + 8.0;
        slots.push((
            row![
                ui::styled(details, Type::BodyMedium)
                    .style(style::on_surface_variant)
                    .wrapping(text::Wrapping::None),
                component::toolbar_divider(),
            ]
            .spacing(8)
            .align_y(Center)
            .into(),
            details_width + DIVIDER_WIDTH + 8.0,
            Some(0),
            false,
        ));
        let zoom = self.markup_zoom().unwrap_or(self.zoom());
        slots.push((
            component::group([
                component::tool(
                    Icon::ZoomOut,
                    crate::fl!("image-zoom-out"),
                    Some(Message::ZoomOut),
                ),
                ui::styled(
                    crate::fl!("image-zoom", percent = ((zoom * 100.0).round() as i64)),
                    Type::LabelLarge,
                )
                .width(48)
                .align_x(Center)
                .into(),
                component::tool(
                    Icon::ZoomIn,
                    crate::fl!("image-zoom-in"),
                    Some(Message::ZoomIn),
                ),
            ]),
            TOOL_WIDTH * 2.0 + 48.0 + 8.0,
            Some(4),
            false,
        ));
        let fitted = match self.markup() {
            Some(markup) => markup.window.fits_page(),
            None => self.fit == Fit::Fit,
        };
        slots.push((
            component::group([
                component::toggle_tool(
                    Icon::FitPage,
                    crate::fl!("image-fit"),
                    fitted,
                    Message::FitToWindow,
                ),
                component::toggle_tool(
                    Icon::OneToOne,
                    crate::fl!("image-actual-size"),
                    !fitted && (zoom - 1.0).abs() < 0.005,
                    Message::ActualSize,
                ),
            ]),
            tools(2.0),
            Some(1),
            false,
        ));
        let right = slots.len();
        // Undo and redo act on the markup while there is some, and give
        // way to the markup bar's own.
        let markup_bar = self
            .markup()
            .is_some_and(|markup| markup.window.markup_bar_shown());
        // Their room stays, so the bar does not shift as the markup bar
        // comes and goes; they sit between Export and the bar's own
        // buttons, as in a PDF window.
        let undo: Element<'a, Message> = if markup_bar {
            space().width(tools(2.0)).into()
        } else {
            let (can_undo, can_redo) = match self.markup() {
                Some(markup) => markup.window.can_undo(),
                None => (
                    editor.is_some_and(|editor| editor.stack.can_undo()),
                    editor.is_some_and(|editor| editor.stack.can_redo()),
                ),
            };
            component::group([
                when(
                    Icon::Undo,
                    crate::fl!("image-undo"),
                    can_undo,
                    Message::Undo,
                ),
                when(
                    Icon::Redo,
                    crate::fl!("image-redo"),
                    can_redo,
                    Message::Redo,
                ),
            ])
        };
        slots.push((
            component::group([
                when(
                    Icon::RotateLeft,
                    crate::fl!("image-rotate-left"),
                    editable,
                    Message::Edit(Edit::RotateLeft),
                ),
                when(
                    Icon::RotateRight,
                    crate::fl!("image-rotate-right"),
                    editable,
                    Message::Edit(Edit::RotateRight),
                ),
                when(
                    Icon::Flip,
                    crate::fl!("image-flip-horizontal"),
                    editable,
                    Message::Edit(Edit::FlipHorizontal),
                ),
                when(
                    Icon::FlipVertical,
                    crate::fl!("image-flip-vertical"),
                    editable,
                    Message::Edit(Edit::FlipVertical),
                ),
            ]),
            tools(4.0),
            Some(2),
            false,
        ));
        slots.push((
            component::group([
                if markable {
                    component::toggle_tool(
                        Icon::HighlightAlt,
                        crate::fl!("image-select"),
                        self.selecting,
                        Message::ToggleSelecting,
                    )
                } else {
                    component::tool(Icon::HighlightAlt, crate::fl!("image-select"), None)
                },
                when(
                    Icon::Crop,
                    crate::fl!("image-crop"),
                    editable && self.selection.is_some(),
                    Message::Edit(Edit::Crop),
                ),
            ]),
            tools(2.0),
            Some(3),
            false,
        ));
        slots.push((
            component::group([
                edit_panel(
                    Icon::Resize,
                    crate::fl!("image-adjust-size-tool"),
                    Panel::AdjustSize,
                ),
                edit_panel(
                    Icon::Tune,
                    crate::fl!("image-adjust-color-tool"),
                    Panel::AdjustColor,
                ),
                panel(Icon::Info, crate::fl!("image-inspector"), Panel::Inspector),
                component::toggle_tool(
                    Icon::SmartToy,
                    crate::fl!("assistant-title"),
                    self.assistant_shown,
                    Message::ToggleAssistant,
                ),
                if markable {
                    component::toggle_tool(
                        Icon::EditDocument,
                        crate::fl!("image-markup"),
                        self.markup()
                            .is_some_and(|markup| markup.window.markup_bar_shown()),
                        Message::ToggleMarkup,
                    )
                } else {
                    component::tool(Icon::EditDocument, crate::fl!("image-markup"), None)
                },
            ]),
            DIVIDER_WIDTH + tools(5.0) + 8.0,
            Some(5),
            true,
        ));
        slots.push((
            component::tip(
                ui::icon_button(Icon::FileExport)
                    .kind(Kind::Tonal)
                    .on_press_maybe((markable || is_svg).then_some(Message::Export)),
                crate::fl!("image-export"),
            ),
            component::TOOL_WIDTH,
            Some(6),
            false,
        ));
        slots.push((undo, tools(2.0), None, false));
        slots.push((
            component::group([
                component::floating_bars_toggle(Message::ToggleFloatingBars),
                component::tool(
                    Icon::Settings,
                    crate::fl!("image-settings"),
                    Some(Message::OpenSettings),
                ),
            ]),
            TOOL_WIDTH * 2.0 + 4.0,
            None,
            false,
        ));
        let widths: Vec<(f32, Option<u8>)> = slots
            .iter()
            .map(|(_, width, order, _)| (*width, *order))
            .collect();
        let shown = component::fitting_slots(width, &widths);
        let mut bar = crate::line![].spacing(8).align_y(Center);
        let mut hidden = Vec::new();
        for (index, ((element, _, _, divider), shown)) in slots.into_iter().zip(shown).enumerate() {
            if index == right {
                bar = bar.push(space::horizontal());
            }
            if shown {
                if divider {
                    bar = bar.push(component::toolbar_divider());
                }
                bar = bar.push(element);
            } else {
                hidden.push(element);
            }
        }
        if !hidden.is_empty() {
            bar = bar.push(component::overflow(
                hidden,
                self.overflow_open,
                Message::ToggleOverflow,
                Message::CloseOverflow,
            ));
        }
        // The bar fills the toolbar's height; keep the buttons in its middle.
        container(bar).height(Fill).align_y(Center).into()
    }

    fn panel_view(&self, panel: Panel) -> Element<'_, Message> {
        let (title, content) = match panel {
            Panel::AdjustColor => (crate::fl!("image-adjust-color"), self.color_panel()),
            Panel::AdjustSize => (crate::fl!("image-adjust-size"), self.size_panel()),
            Panel::Inspector => (crate::fl!("image-inspector"), self.inspector_panel()),
        };
        component::side_sheet(title, Message::TogglePanel(panel), content)
    }

    fn color_panel(&self) -> Element<'_, Message> {
        let adjust = self.color;
        let control = |label: String,
                       range: std::ops::RangeInclusive<f32>,
                       value: f32,
                       set: fn(ColorAdjust, f32) -> ColorAdjust| {
            let backdrop = move |theme: &iced::Theme, status| {
                let scheme = ui::Scheme::of(theme);
                style::slider(Backdrop::ContainerLow.color(&scheme))(theme, status)
            };
            column![
                row![
                    ui::aligned(ui::styled(label, Type::BodyMedium)),
                    ui::styled(format!("{value:+.2}"), Type::LabelMedium)
                        .style(style::on_surface_variant),
                ],
                slider(range, value, move |value| Message::Edit(Edit::Color(set(
                    adjust, value
                ))))
                .step(0.01_f32)
                .height(style::SLIDER_HEIGHT)
                .style(backdrop)
                .on_release(Message::Edit(Edit::ColorCommitted)),
            ]
            .spacing(0)
        };
        column![
            control(
                crate::fl!("image-exposure"),
                -2.0..=2.0,
                adjust.exposure,
                |a, v| { ColorAdjust { exposure: v, ..a } }
            ),
            control(
                crate::fl!("image-contrast"),
                -1.0..=1.0,
                adjust.contrast,
                |a, v| { ColorAdjust { contrast: v, ..a } }
            ),
            control(
                crate::fl!("image-saturation"),
                0.0..=2.0,
                adjust.saturation,
                |a, v| { ColorAdjust { saturation: v, ..a } }
            ),
            control(
                crate::fl!("image-temperature"),
                -1.0..=1.0,
                adjust.temperature,
                |a, v| {
                    ColorAdjust {
                        temperature: v,
                        ..a
                    }
                }
            ),
            control(crate::fl!("image-tint"), -1.0..=1.0, adjust.tint, |a, v| {
                ColorAdjust { tint: v, ..a }
            }),
            control(
                crate::fl!("image-sepia"),
                0.0..=1.0,
                adjust.sepia,
                |a, v| ColorAdjust { sepia: v, ..a }
            ),
            control(
                crate::fl!("image-sharpness"),
                0.0..=1.0,
                adjust.sharpness,
                |a, v| { ColorAdjust { sharpness: v, ..a } }
            ),
            component::section(crate::fl!("image-levels")),
            control(
                crate::fl!("image-black-point"),
                0.0..=0.9,
                adjust.black,
                |a, v| ColorAdjust {
                    black: v.min(a.white - 0.05),
                    ..a
                }
            ),
            // A log scale centres the default gamma of 1: one third to three.
            control(
                crate::fl!("image-midtones"),
                -1.0..=1.0,
                adjust.gamma.ln() / 3f32.ln(),
                |a, v| ColorAdjust {
                    gamma: 3f32.powf(v),
                    ..a
                },
            ),
            control(
                crate::fl!("image-white-point"),
                0.1..=1.0,
                adjust.white,
                |a, v| ColorAdjust {
                    white: v.max(a.black + 0.05),
                    ..a
                }
            ),
            container(
                ui::with_icon(
                    Kind::Outlined,
                    Icon::ResetAll,
                    crate::fl!("image-reset-all")
                )
                .on_press(Message::Edit(Edit::ResetColor))
            )
            .padding(Padding {
                top: 12.0,
                ..Padding::ZERO
            }),
        ]
        .spacing(4)
        .into()
    }

    fn size_panel(&self) -> Element<'_, Message> {
        let current = self.items[self.current].size.unwrap_or((0, 0));
        column![
            ui::styled(
                crate::fl!("image-current-size", width = current.0, height = current.1),
                Type::BodyMedium
            )
            .style(style::on_surface_variant),
            component::text_field(
                crate::fl!("image-width"),
                &self.size_input.0,
                Backdrop::ContainerLow,
                |input| input.on_input(Message::SizeWidth),
            ),
            component::text_field(
                crate::fl!("image-height"),
                &self.size_input.1,
                Backdrop::ContainerLow,
                |input| input.on_input(Message::SizeHeight),
            ),
            row![
                ui::aligned(ui::styled(
                    crate::fl!("image-scale-proportionally"),
                    Type::BodyLarge
                )),
                toggler(self.keep_proportions)
                    .on_toggle(Message::KeepProportions)
                    .size(28)
                    .style(style::switch),
            ]
            .align_y(Center),
            ui::button(Kind::Filled, crate::fl!("image-resize"))
                .on_press(Message::Edit(Edit::ApplySize)),
        ]
        .spacing(16)
        .into()
    }

    fn inspector_panel(&self) -> Element<'_, Message> {
        let Some(inspector) = self
            .inspector
            .as_ref()
            .filter(|inspector| inspector.index == self.current)
        else {
            return ui::styled(crate::fl!("image-inspector-loading"), Type::BodyMedium)
                .style(style::on_surface_variant)
                .into();
        };
        // The file first, as Preview's inspector shows it.
        let item = &self.items[self.current];
        let mut file = crate::info::file_facts(&item.path);
        file.push((
            crate::fl!("image-format"),
            match item.source {
                Source::Raster(format) => crate::i18n::format_name(format),
                Source::Svg => "SVG".to_owned(),
            },
        ));
        if let Some((width, height)) = item.size {
            file.push((
                crate::fl!("image-dimensions-label"),
                crate::fl!("image-pixels", width = width, height = height),
            ));
        }
        let mut content = column![crate::info::sections_view(vec![(
            crate::fl!("image-file"),
            file
        )])]
        .spacing(6);
        let mut section = None;
        for field in &inspector.details.fields {
            if section != Some(field.section) {
                section = Some(field.section);
                content = content.push(component::section(meta_section(field.section)));
            }
            content = content.push(
                row![
                    ui::aligned_to(
                        ui::styled(meta_label(field.label), Type::BodySmall)
                            .style(style::on_surface_variant),
                        104,
                    ),
                    ui::aligned(ui::styled(meta_value(&field.value), Type::BodyMedium)),
                ]
                .spacing(8),
            );
        }
        if inspector.details.fields.is_empty() {
            content = content.push(
                ui::styled(crate::fl!("image-no-camera"), Type::BodyMedium)
                    .style(style::on_surface_variant),
            );
        }
        content = content.push(component::section(crate::fl!("image-location")));
        content = match inspector.details.location {
            Some((latitude, longitude)) => content
                .push(
                    row![
                        icon::icon(Icon::LocationOn, 20).style(style::on_surface_variant),
                        ui::styled(format!("{latitude:.5}, {longitude:.5}"), Type::BodyMedium),
                    ]
                    .spacing(8)
                    .align_y(Center),
                )
                .push(
                    ui::with_icon(
                        Kind::Outlined,
                        Icon::LocationOff,
                        crate::fl!("image-remove-location"),
                    )
                    .on_press(Message::RemoveLocation),
                ),
            None => content.push(
                ui::styled(crate::fl!("image-no-location"), Type::BodyMedium)
                    .style(style::on_surface_variant),
            ),
        };
        content = content
            .push(component::section(crate::fl!("image-keywords-description")))
            .push(component::text_field(
                crate::fl!("image-keywords-hint"),
                &inspector.keywords,
                Backdrop::ContainerLow,
                |input| input.on_input(Message::KeywordsChanged),
            ))
            .push(component::text_field(
                crate::fl!("image-description"),
                &inspector.description,
                Backdrop::ContainerLow,
                |input| input.on_input(Message::DescriptionChanged),
            ))
            .push(
                container(
                    ui::button(Kind::Tonal, crate::fl!("common-save"))
                        .on_press_maybe(inspector.writable_xmp.then_some(Message::SaveMetadata)),
                )
                .padding(Padding {
                    top: 4.0,
                    ..Padding::ZERO
                }),
            );
        if !inspector.writable_xmp {
            content = content.push(ui::aligned(
                ui::styled(crate::fl!("image-keywords-unsupported"), Type::BodySmall)
                    .style(style::on_surface_variant),
            ));
        }
        content = content.push(component::section(crate::fl!("image-revert-to")));
        if inspector.versions.is_empty() {
            content = content.push(
                ui::styled(crate::fl!("image-no-versions"), Type::BodyMedium)
                    .style(style::on_surface_variant),
            );
        }
        for version in &inspector.versions {
            content = content.push(
                row![
                    icon::icon(Icon::History, 20).style(style::on_surface_variant),
                    column![
                        ui::styled(versions::describe_age(version.saved_at), Type::BodyMedium),
                        ui::styled(format_bytes(version.size), Type::BodySmall)
                            .style(style::on_surface_variant),
                    ]
                    .width(Fill),
                    ui::button(Kind::Text, crate::fl!("image-revert"))
                        .on_press(Message::Revert(version.clone())),
                ]
                .spacing(12)
                .align_y(Center),
            );
        }
        content.into()
    }

    fn sidebar_view(&self) -> Element<'_, Message> {
        let side = self.sidebar_width.value - SIDEBAR_THUMBNAIL_MARGIN;
        let entries = self.items.iter().enumerate().map(|(index, item)| {
            // Fit the thumbnail in a square, so the ring hugs the image.
            let (width, height) = match item.size {
                Some((width, height)) if width > 0 && height > 0 => {
                    let scale = side / width.max(height) as f32;
                    (width as f32 * scale, height as f32 * scale)
                }
                _ => (side, side),
            };
            let picture: Element<'_, Message> = match &item.thumbnail {
                Some(handle) => image(handle.clone())
                    .width(width)
                    .height(height)
                    .border_radius(style::THUMBNAIL_RADIUS)
                    .into(),
                None => container(space::horizontal())
                    .width(width)
                    .height(height)
                    .into(),
            };
            let selected = index == self.current;
            let framed = container(picture)
                .padding(style::THUMBNAIL_RING)
                .style(move |theme: &iced::Theme| style::thumbnail(theme, selected));
            let name = item
                .path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            let label = ui::styled(name, Type::LabelMedium).wrapping(text::Wrapping::None);
            let label = if selected {
                label.style(style::primary_text)
            } else {
                label.style(style::on_surface_variant)
            };
            // Every entry takes the same square, which scrolling relies on;
            // the image sits at its bottom, just above the name.
            let framed = container(framed)
                .center_x(side + 2.0 * style::THUMBNAIL_RING)
                .align_bottom(side + 2.0 * style::THUMBNAIL_RING);
            mouse_area(
                column![framed, label]
                    .spacing(4)
                    .align_x(Center)
                    .width(Length::Fill),
            )
            .on_press(Message::SidebarPressed(index))
            .interaction(iced::mouse::Interaction::Idle)
            .into()
        });
        let list = mouse_area(column(entries).spacing(12).padding(12).width(Fill))
            .on_move(Message::SidebarMoved)
            .on_release(Message::SidebarReleased);
        component::scroll(list)
            .id(self.sidebar_id.clone())
            .height(Fill)
            .into()
    }
}

/// Room above sidebars and panels for the toolbar when it floats.
fn below_bars(element: Element<'_, Message>, bottom_bar: bool) -> Element<'_, Message> {
    component::between_bars(
        element,
        component::floating_room(true),
        component::floating_room(bottom_bar),
    )
}

/// The inspector's heading for a group of the camera's fields.
fn meta_section(section: metadata::Section) -> String {
    use metadata::Section;
    match section {
        Section::Camera => crate::fl!("image-meta-camera"),
        Section::Exposure => crate::fl!("image-meta-exposure"),
        Section::Image => crate::fl!("image-meta-image"),
    }
}

fn meta_label(label: metadata::Label) -> String {
    use metadata::Label;
    match label {
        Label::Make => crate::fl!("image-meta-make"),
        Label::Model => crate::fl!("image-meta-model"),
        Label::Lens => crate::fl!("image-meta-lens"),
        Label::ExposureTime => crate::fl!("image-meta-exposure-time"),
        Label::FNumber => crate::fl!("image-meta-f-number"),
        Label::Iso => crate::fl!("image-meta-iso"),
        Label::FocalLength => crate::fl!("image-meta-focal-length"),
        Label::ExposureBias => crate::fl!("image-meta-exposure-bias"),
        Label::Flash => crate::fl!("image-meta-flash"),
        Label::DateTaken => crate::fl!("image-meta-date-taken"),
        Label::Orientation => crate::fl!("image-meta-orientation"),
        Label::ColorSpace => crate::fl!("image-meta-color-space"),
        Label::Software => crate::fl!("image-meta-software"),
        Label::Artist => crate::fl!("image-meta-artist"),
        Label::Copyright => crate::fl!("image-meta-copyright"),
    }
}

fn meta_value(value: &metadata::Value) -> String {
    use metadata::{ColorSpace, FlashMode, Value};
    let yes = |on: bool| if on { "yes" } else { "no" };
    match value {
        Value::Text(text) => text.clone(),
        Value::Seconds(value) => crate::fl!("image-meta-seconds", value = value.as_str()),
        Value::Millimeters(value) => crate::fl!("image-meta-millimeters", value = value.as_str()),
        Value::Ev(value) => crate::fl!("image-meta-ev", value = value.as_str()),
        Value::Orientation(value) => {
            let value = *value;
            crate::fl!("image-meta-orientation-value", value = value)
        }
        Value::Flash {
            fired,
            mode,
            red_eye,
        } => {
            let mode = match mode {
                FlashMode::On => "on",
                FlashMode::Off => "off",
                FlashMode::Auto => "auto",
                FlashMode::Unknown => "unknown",
            };
            crate::fl!(
                "image-meta-flash-value",
                fired = yes(*fired),
                mode = mode,
                redeye = yes(*red_eye)
            )
        }
        Value::ColorSpace(space) => {
            let (space, code) = match space {
                ColorSpace::Srgb => ("srgb", 1),
                ColorSpace::AdobeRgb => ("adobe", 2),
                ColorSpace::Uncalibrated => ("uncalibrated", 0xffff),
                ColorSpace::Other(code) => ("other", *code),
            };
            crate::fl!("image-meta-color-space-value", space = space, code = code)
        }
    }
}

#[cfg(test)]
mod meta_tests {
    use prev_image::metadata::{ColorSpace, FlashMode, Label, Value};

    use super::{meta_label, meta_value};

    #[test]
    fn words_camera_values() {
        assert_eq!(meta_label(Label::ExposureTime), "Exposure time");
        assert_eq!(meta_value(&Value::Seconds("1/200".into())), "1/200 s");
        assert_eq!(meta_value(&Value::Orientation(6)), "Rotated 90° clockwise");
        assert_eq!(meta_value(&Value::Orientation(9)), "Unknown (9)");
        let flash = |fired, mode, red_eye| {
            meta_value(&Value::Flash {
                fired,
                mode,
                red_eye,
            })
        };
        assert_eq!(flash(true, FlashMode::Auto, false), "Fired, auto");
        assert_eq!(flash(false, FlashMode::Unknown, false), "Did not fire");
        assert_eq!(
            flash(true, FlashMode::On, true),
            "Fired, forced on, red-eye reduction"
        );
        assert_eq!(meta_value(&Value::ColorSpace(ColorSpace::Srgb)), "sRGB");
        assert_eq!(
            meta_value(&Value::ColorSpace(ColorSpace::Other(7))),
            "Other (7)"
        );
    }
}
