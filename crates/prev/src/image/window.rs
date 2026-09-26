//! A window showing one image, or several with a thumbnail sidebar, with
//! Preview's editing tools.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bytes::Bytes;
use iced::advanced::image::Allocation;
use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::image::Handle;
use iced::widget::scrollable::{self, Direction, Scrollbar};
use iced::widget::{
    Id, button, center, checkbox, column, container, image, mouse_area, opaque, operation, row,
    rule, scrollable as scroll, slider, space, stack, text, text_input,
};
use iced::{Center, Color, Element, Fill, Length, Task};
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
use crate::shortcuts::Action;

const THUMBNAIL_SIZE: u32 = 240;
const SIDEBAR_WIDTH: f32 = 180.0;
const SIDEBAR_THUMBNAIL: f32 = 120.0;
const PANEL_WIDTH: f32 = 300.0;
/// Full images kept in memory around the current one, in each direction.
const KEEP_AROUND: usize = 1;
/// Quiet time after an edit before it is written to disk.
const AUTOSAVE_DELAY: Duration = Duration::from_millis(1200);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Raster(ImageFormat),
    Svg,
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
    /// The full-size edited image, for saving and exporting.
    edited: Option<Arc<Frame>>,
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
}

#[derive(Debug, Clone)]
pub enum Message {
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
    Edit(Edit),
    PreviewRendered(usize, u64, Handle),
    PreviewAllocated(usize, u64, Option<Allocation>),
    FullRendered(usize, u64, Arc<Frame>),
    FullAllocated(usize, u64, Arc<Frame>, Option<Allocation>),
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
    ExportAsIs,
    ExportChooseAgain,
    ExportTarget(Result<Option<dialog::SavedWithMenus>, String>),
    Exported(Result<PathBuf, String>),
}

/// An export waiting for the user to confirm a name whose extension does
/// not match the chosen format.
struct PendingExport {
    path: PathBuf,
    format: SaveFormat,
    format_choice: String,
    quality_choice: String,
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
    pending_export: Option<PendingExport>,
    notice: Option<String>,
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
            let svg = Svg::parse_file(path).map_err(|error| error.to_string())?;
            let (width, height) = svg.size();
            let scale = THUMBNAIL_SIZE as f32 / width.max(height).max(1.0);
            let thumbnail = svg.render(scale).map_err(|error| error.to_string())?;
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
            let decoded = decode_file(path, format).map_err(|error| error.to_string())?;
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

fn format_bytes(bytes: u64) -> String {
    match bytes {
        0..1_000_000 => format!("{:.0} KB", bytes as f64 / 1000.0),
        _ => format!("{:.1} MB", bytes as f64 / 1_000_000.0),
    }
}

impl ImageWindow {
    pub fn open(files: Vec<(PathBuf, Source)>) -> (Self, Task<Message>) {
        let sidebar = files.len() > 1;
        let items = files
            .into_iter()
            .map(|(path, source)| Item {
                path,
                source,
                state: ItemState::Waiting,
                thumbnail: None,
                size: None,
            })
            .collect();
        let mut window = Self {
            items,
            current: 0,
            sidebar,
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
            pending_export: None,
            notice: None,
        };
        let task = window.load_next();
        (window, task)
    }

    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.items.iter().map(|item| item.path.as_path())
    }

    pub fn current_path(&self) -> &Path {
        &self.items[self.current].path
    }

    pub fn set_device_scale(&mut self, scale: f32) -> Task<Message> {
        self.device_scale = scale;
        self.schedule()
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
            let editing = matches!(&item.state, ItemState::Loaded(shown) if shown.editor.is_some());
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
                result.unwrap_or_else(|_| Err("loading stopped".into())),
            )
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
                if index == self.current {
                    self.restart_animation();
                }
                Task::batch([self.schedule(), self.load_next()])
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
            Message::ZoomIn => self.zoom_to(self.zoom() * ZOOM_STEP, None),
            Message::ZoomOut => self.zoom_to(self.zoom() / ZOOM_STEP, None),
            Message::ActualSize => self.zoom_to(1.0, None),
            Message::FitToWindow => {
                self.fit = Fit::Fit;
                self.schedule()
            }
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
                let handle = frame_handle(&frame);
                allocate(handle, move |allocation| {
                    Message::FullAllocated(index, generation, frame.clone(), allocation)
                })
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
                        .map_err(|error| error.to_string())
                })
            }
            Message::RemoveLocation => self.metadata_edit(|bytes| {
                metadata::remove_location(bytes).map_err(|error| error.to_string())
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
                        let store =
                            VersionStore::default_location().ok_or("No place to keep versions")?;
                        store
                            .restore(&path, &version)
                            .map_err(|error| format!("Could not revert: {error}"))
                    }),
                    |result| {
                        Message::Reverted(
                            result.unwrap_or_else(|_| Err("reverting stopped".into())),
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
            Message::Export => self.export(),
            Message::ExportTarget(Ok(Some((chosen, choices)))) => {
                let selected = |menu: &str| {
                    choices
                        .iter()
                        .find(|(id, _)| id == menu)
                        .map(|(_, value)| value.clone())
                };
                let format_choice = selected("format").unwrap_or_else(|| "png".into());
                let quality_choice =
                    selected("quality").unwrap_or_else(|| editor::DEFAULT_QUALITY_CHOICE.into());
                let Some(format) = editor::format_for_choice(&format_choice, &quality_choice)
                else {
                    return Task::none();
                };
                let pending = PendingExport {
                    path: chosen,
                    format,
                    format_choice,
                    quality_choice,
                };
                if editor::extension_matches(&pending.path, format) {
                    return self.run_export(pending);
                }
                // Ask before saving under a name that does not fit the format.
                self.pending_export = Some(pending);
                Task::none()
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
                    Some(pending.format_choice),
                    Some(pending.quality_choice),
                ),
                None => Task::none(),
            },
            Message::ExportTarget(Ok(None)) => Task::none(),
            Message::ExportTarget(Err(error)) => {
                self.notice = Some(format!("Could not show the save dialog: {error}"));
                Task::none()
            }
            Message::Exported(result) => {
                self.notice = Some(match result {
                    Ok(path) => format!("Exported {}", path.display()),
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
        self.restart_animation();
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
        ])
    }

    fn restart_animation(&mut self) {
        self.frame = 0;
        self.next_frame = self
            .shown()
            .filter(|shown| shown.image.frames.len() > 1)
            .map(|shown| Instant::now() + shown.image.frames[0].1);
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
                if frames > 1 {
                    let next = (self.frame + 1) % frames;
                    let delay = shown.image.frames[next].1;
                    self.frame = next;
                    let due = self.next_frame.map_or(now, |due| due + delay);
                    // Skip ahead rather than race if we fell behind.
                    self.next_frame = Some(if due < now { now + delay } else { due });
                }
                Task::none()
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
        let index = self.current;
        let selection = self.selection;
        let size_input = self.size_input.clone();
        let Some(shown) = self.shown_mut(index) else {
            return Task::none();
        };
        if !shown.is_editable() {
            self.notice = Some("Animations and SVG drawings can't be edited.".into());
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
            Edit::Crop => match selection
                .and_then(|selection| editor::selection_to_crop(selection, size))
            {
                Some(crop) => editor.change(|stack| {
                    stack.push(Operation::Crop(crop));
                    true
                }),
                None => {
                    self.notice = Some("Drag a selection first (Select tool), then crop.".into());
                    return Task::none();
                }
            },
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
                    self.notice = Some("Enter a width and height in pixels.".into());
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
            Edit::Crop | Edit::RotateLeft | Edit::RotateRight | Edit::Undo | Edit::Redo
        ) {
            self.selection = None;
            self.selecting = false;
        }
        if matches!(edit, Edit::Undo | Edit::Redo) {
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
            Err(_) => Message::Saved(index, generation, Err("rendering stopped".into())),
        })
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
        shown.edited = Some(Arc::clone(&frame));
        self.items[index].thumbnail = Some(thumbnail_of(&frame));
        self.items[index].size = Some((frame.width, frame.height));
        let due = Task::perform(
            spawn(move || std::thread::sleep(AUTOSAVE_DELAY)),
            move |_| Message::AutosaveDue(index, generation),
        );
        Task::batch([self.schedule(), due])
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
            self.notice = Some(format!(
                "Changes to “{name}” can't be saved in its format. Use Export (Ctrl+Shift+S)."
            ));
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
                    result.unwrap_or_else(|_| Err("saving stopped".into())),
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
                Message::MetadataSaved(result.unwrap_or_else(|_| Err("saving stopped".into())))
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
        if !matches!(self.items[self.current].source, Source::Raster(_)) {
            self.notice = Some("SVG drawings can't be exported yet.".into());
            return Task::none();
        }
        if !shown.is_editable() {
            self.notice = Some("Animations can't be exported yet.".into());
            return Task::none();
        }
        self.open_export_dialog(None, None, None, None)
    }

    /// Opens the save dialog with the Format and JPEG quality menus.
    /// `None` arguments fall back to the suggestions for the current image.
    fn open_export_dialog(
        &mut self,
        name: Option<String>,
        folder: Option<PathBuf>,
        format_choice: Option<String>,
        quality_choice: Option<String>,
    ) -> Task<Message> {
        let item = &self.items[self.current];
        let Source::Raster(format) = item.source else {
            return Task::none();
        };
        let name = name.unwrap_or_else(|| editor::export_name(&item.path, format, false));
        let menus = vec![
            dialog::Menu {
                id: "format",
                label: "Format",
                initial: format_choice
                    .unwrap_or_else(|| editor::default_format_choice(format, false).to_owned()),
                options: editor::EXPORT_FORMATS
                    .iter()
                    .map(|(id, label, _)| (*id, *label))
                    .collect(),
            },
            dialog::Menu {
                id: "quality",
                label: "JPEG quality",
                initial: quality_choice
                    .unwrap_or_else(|| editor::DEFAULT_QUALITY_CHOICE.to_owned()),
                options: editor::JPEG_QUALITIES
                    .iter()
                    .map(|(id, label, _)| (*id, *label))
                    .collect(),
            },
        ];
        Task::perform(
            dialog::save_file_with_menus("Export".into(), name, folder, menus),
            Message::ExportTarget,
        )
    }

    fn run_export(&mut self, pending: PendingExport) -> Task<Message> {
        let Some(frame) = self.shown().and_then(Shown::current_frame) else {
            return Task::none();
        };
        let original = self.items[self.current].path.clone();
        let PendingExport { path, format, .. } = pending;
        Task::perform(
            spawn(move || editor::export(&original, &frame, &path, format).map(|()| path)),
            |result| Message::Exported(result.unwrap_or_else(|_| Err("exporting stopped".into()))),
        )
    }

    pub fn shortcut(&mut self, action: Action) -> Option<Task<Message>> {
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

    /// Arrow keys step through the images.
    pub fn key(&mut self, key: &Key, modifiers: Modifiers) -> Option<Task<Message>> {
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

    pub fn view(&self) -> Element<'_, Message> {
        let canvas: Element<'_, Message> = match &self.items[self.current].state {
            ItemState::Failed(error) => center(
                column![
                    text("prev can't open this image.").size(18),
                    text(error).size(13)
                ]
                .spacing(8)
                .align_x(Center),
            )
            .into(),
            _ => match self.placement() {
                Some(placement) => scroll(
                    ImageCanvas::new(
                        self.display_handle(),
                        placement,
                        self.next_frame,
                        Message::Canvas,
                    )
                    .selection(self.selecting, self.selection),
                )
                .id(self.canvas_id.clone())
                .direction(Direction::Both {
                    vertical: Scrollbar::default(),
                    horizontal: Scrollbar::default(),
                })
                .width(Fill)
                .height(Fill)
                .into(),
                None => center(text("Opening…")).into(),
            },
        };
        let mut content = row![].height(Fill);
        if self.sidebar && self.items.len() > 1 {
            content = content
                .push(
                    container(self.sidebar_view())
                        .width(SIDEBAR_WIDTH)
                        .height(Fill),
                )
                .push(rule::vertical(1));
        }
        content = content.push(canvas);
        if let Some(panel) = self.panel {
            content = content.push(rule::vertical(1)).push(
                container(scroll(container(self.panel_view(panel)).padding(12)).height(Fill))
                    .width(PANEL_WIDTH)
                    .height(Fill),
            );
        }
        let mut page = column![self.toolbar(), rule::horizontal(1), content];
        if let Some(notice) = &self.notice {
            page = page.push(container(text(notice).size(13)).padding(6));
        }
        match &self.pending_export {
            Some(pending) => stack![page, opaque(self.export_prompt(pending))].into(),
            None => page.into(),
        }
    }

    fn export_prompt<'a>(&'a self, pending: &'a PendingExport) -> Element<'a, Message> {
        let name = pending
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let extension = pending
            .path
            .extension()
            .map(|extension| format!("ends in .{}", extension.to_string_lossy()))
            .unwrap_or_else(|| "has no extension".into());
        let card = container(
            column![
                text("The name doesn't match the format").size(16),
                text(format!(
                    "“{name}” will be saved as a {} file, but its name {extension}. Other apps may not open it.",
                    editor::format_label(&pending.format_choice)
                ))
                .size(13),
                row![
                    space::horizontal(),
                    button(text("Choose Again").size(13)).style(button::secondary).on_press(Message::ExportChooseAgain),
                    button(text("Save as Is").size(13)).on_press(Message::ExportAsIs),
                ]
                .spacing(8),
            ]
            .spacing(12)
            .width(420),
        )
        .padding(20)
        .style(container::bordered_box);
        center(card)
            .style(|_| {
                container::Style::default().background(Color::from_rgba(0.0, 0.0, 0.0, 0.45))
            })
            .into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        let item = &self.items[self.current];
        let mut details = item
            .size
            .map(|(width, height)| format!("{width} × {height}"))
            .unwrap_or_default();
        let shown = self.shown();
        if let Some(shown) = shown
            && shown.image.frames.len() > 1
        {
            details.push_str(&format!(
                "  ·  frame {} of {}",
                self.frame + 1,
                shown.image.frames.len()
            ));
        }
        if self.items.len() > 1 {
            details.push_str(&format!(
                "  ·  {} of {}",
                self.current + 1,
                self.items.len()
            ));
        }
        if let Some(editor) = shown.and_then(|shown| shown.editor.as_ref())
            && !editor.is_saved()
        {
            details.push_str("  ·  edited");
        }
        let editable = shown.is_some_and(Shown::is_editable);
        let editor = shown.and_then(|shown| shown.editor.as_ref());
        let small =
            |label: &'static str, message: Message| button(text(label).size(13)).on_press(message);
        let when = |label: &'static str, enabled: bool, message: Message| {
            button(text(label).size(13)).on_press_maybe(enabled.then_some(message))
        };
        let toggled = |label: &'static str, on: bool, message: Message| {
            let style = if on {
                button::primary
            } else {
                button::secondary
            };
            button(text(label).size(13)).style(style).on_press(message)
        };
        let mut bar = row![].spacing(6).padding(6).align_y(Center);
        if self.items.len() > 1 {
            bar = bar.push(small("Sidebar", Message::ToggleSidebar));
        }
        bar.push(text(details).size(13))
            .push(space::horizontal())
            .push(when(
                "Rotate Left",
                editable,
                Message::Edit(Edit::RotateLeft),
            ))
            .push(when(
                "Rotate Right",
                editable,
                Message::Edit(Edit::RotateRight),
            ))
            .push(when(
                "Flip ↔",
                editable,
                Message::Edit(Edit::FlipHorizontal),
            ))
            .push(when("Flip ↕", editable, Message::Edit(Edit::FlipVertical)))
            .push(toggled("Select", self.selecting, Message::ToggleSelecting))
            .push(when(
                "Crop",
                editable && self.selection.is_some(),
                Message::Edit(Edit::Crop),
            ))
            .push(when(
                "Undo",
                editor.is_some_and(|editor| editor.stack.can_undo()),
                Message::Edit(Edit::Undo),
            ))
            .push(when(
                "Redo",
                editor.is_some_and(|editor| editor.stack.can_redo()),
                Message::Edit(Edit::Redo),
            ))
            .push(space::horizontal().width(10))
            .push(toggled(
                "Size",
                self.panel == Some(Panel::AdjustSize),
                Message::TogglePanel(Panel::AdjustSize),
            ))
            .push(toggled(
                "Color",
                self.panel == Some(Panel::AdjustColor),
                Message::TogglePanel(Panel::AdjustColor),
            ))
            .push(toggled(
                "Info",
                self.panel == Some(Panel::Inspector),
                Message::TogglePanel(Panel::Inspector),
            ))
            .push(when("Export…", editable, Message::Export))
            .push(space::horizontal().width(10))
            .push(small("−", Message::ZoomOut))
            .push(
                text(format!("{:.0}%", self.zoom() * 100.0))
                    .size(13)
                    .width(52)
                    .align_x(Center),
            )
            .push(small("+", Message::ZoomIn))
            .push(small("Fit", Message::FitToWindow))
            .push(small("1:1", Message::ActualSize))
            .into()
    }

    fn panel_view(&self, panel: Panel) -> Element<'_, Message> {
        match panel {
            Panel::AdjustColor => self.color_panel(),
            Panel::AdjustSize => self.size_panel(),
            Panel::Inspector => self.inspector_panel(),
        }
    }

    fn color_panel(&self) -> Element<'_, Message> {
        let adjust = self.color;
        let control = |label: &'static str,
                       range: std::ops::RangeInclusive<f32>,
                       value: f32,
                       set: fn(ColorAdjust, f32) -> ColorAdjust| {
            column![
                text(label).size(12),
                slider(range, value, move |value| Message::Edit(Edit::Color(set(
                    adjust, value
                ))))
                .step(0.01_f32)
                .on_release(Message::Edit(Edit::ColorCommitted)),
            ]
            .spacing(2)
        };
        column![
            text("Adjust Color").size(16),
            control("Exposure", -2.0..=2.0, adjust.exposure, |a, v| {
                ColorAdjust { exposure: v, ..a }
            }),
            control("Contrast", -1.0..=1.0, adjust.contrast, |a, v| {
                ColorAdjust { contrast: v, ..a }
            }),
            control("Saturation", 0.0..=2.0, adjust.saturation, |a, v| {
                ColorAdjust { saturation: v, ..a }
            }),
            control("Temperature", -1.0..=1.0, adjust.temperature, |a, v| {
                ColorAdjust {
                    temperature: v,
                    ..a
                }
            }),
            control("Tint", -1.0..=1.0, adjust.tint, |a, v| ColorAdjust {
                tint: v,
                ..a
            }),
            control("Sepia", 0.0..=1.0, adjust.sepia, |a, v| ColorAdjust {
                sepia: v,
                ..a
            }),
            control("Sharpness", 0.0..=1.0, adjust.sharpness, |a, v| {
                ColorAdjust { sharpness: v, ..a }
            }),
            text("Levels").size(14),
            control("Black point", 0.0..=0.9, adjust.black, |a, v| ColorAdjust {
                black: v.min(a.white - 0.05),
                ..a
            }),
            // A log scale centres the default gamma of 1: one third to three.
            control(
                "Midtones",
                -1.0..=1.0,
                adjust.gamma.ln() / 3f32.ln(),
                |a, v| ColorAdjust {
                    gamma: 3f32.powf(v),
                    ..a
                },
            ),
            control("White point", 0.1..=1.0, adjust.white, |a, v| ColorAdjust {
                white: v.max(a.black + 0.05),
                ..a
            }),
            button(text("Reset All").size(13)).on_press(Message::Edit(Edit::ResetColor)),
        ]
        .spacing(10)
        .into()
    }

    fn size_panel(&self) -> Element<'_, Message> {
        let current = self.items[self.current].size.unwrap_or((0, 0));
        column![
            text("Adjust Size").size(16),
            text(format!(
                "Current size: {} × {} pixels",
                current.0, current.1
            ))
            .size(12),
            row![
                text("Width").size(13).width(60),
                text_input("", &self.size_input.0)
                    .on_input(Message::SizeWidth)
                    .size(13)
            ]
            .align_y(Center),
            row![
                text("Height").size(13).width(60),
                text_input("", &self.size_input.1)
                    .on_input(Message::SizeHeight)
                    .size(13)
            ]
            .align_y(Center),
            checkbox(self.keep_proportions)
                .label("Scale proportionally")
                .on_toggle(Message::KeepProportions),
            button(text("Resize").size(13)).on_press(Message::Edit(Edit::ApplySize)),
        ]
        .spacing(10)
        .into()
    }

    fn inspector_panel(&self) -> Element<'_, Message> {
        let Some(inspector) = self
            .inspector
            .as_ref()
            .filter(|inspector| inspector.index == self.current)
        else {
            return text("Loading…").size(13).into();
        };
        let mut content = column![text("Inspector").size(16)].spacing(8);
        let mut section = "";
        for (group, label, value) in &inspector.details.fields {
            if group != section {
                section = group;
                content = content.push(text(group).size(13));
            }
            content = content
                .push(row![text(label).size(12).width(110), text(value).size(12)].spacing(6));
        }
        if inspector.details.fields.is_empty() {
            content = content.push(text("No camera information.").size(12));
        }
        content = content
            .push(rule::horizontal(1))
            .push(text("Location").size(13));
        content = match inspector.details.location {
            Some((latitude, longitude)) => content
                .push(text(format!("{latitude:.5}, {longitude:.5}")).size(12))
                .push(
                    button(text("Remove Location Info").size(13)).on_press(Message::RemoveLocation),
                ),
            None => content.push(text("No location information.").size(12)),
        };
        content = content
            .push(rule::horizontal(1))
            .push(text("Keywords (comma separated)").size(13))
            .push(
                text_input("", &inspector.keywords)
                    .on_input(Message::KeywordsChanged)
                    .size(12),
            )
            .push(text("Description").size(13))
            .push(
                text_input("", &inspector.description)
                    .on_input(Message::DescriptionChanged)
                    .size(12),
            )
            .push(
                button(text("Save Keywords and Description").size(13))
                    .on_press_maybe(inspector.writable_xmp.then_some(Message::SaveMetadata)),
            );
        if !inspector.writable_xmp {
            content =
                content.push(text("Keywords can be saved in JPEG, PNG and WebP files.").size(11));
        }
        content = content
            .push(rule::horizontal(1))
            .push(text("Revert To").size(13));
        if inspector.versions.is_empty() {
            content = content.push(text("No earlier versions.").size(12));
        }
        for version in &inspector.versions {
            content = content.push(
                row![
                    text(format!(
                        "{}, {}",
                        versions::describe_age(version.saved_at),
                        format_bytes(version.size)
                    ))
                    .size(12)
                    .width(Fill),
                    button(text("Revert").size(12)).on_press(Message::Revert(version.clone())),
                ]
                .align_y(Center),
            );
        }
        content.into()
    }

    fn sidebar_view(&self) -> Element<'_, Message> {
        let entries = self.items.iter().enumerate().map(|(index, item)| {
            let picture: Element<'_, Message> = match &item.thumbnail {
                Some(handle) => image(handle.clone())
                    .width(SIDEBAR_THUMBNAIL)
                    .height(SIDEBAR_THUMBNAIL)
                    .into(),
                None => container(space::horizontal())
                    .width(SIDEBAR_THUMBNAIL)
                    .height(SIDEBAR_THUMBNAIL)
                    .into(),
            };
            let selected = index == self.current;
            let framed = container(picture)
                .padding(3)
                .style(move |theme: &iced::Theme| {
                    let color = if selected {
                        theme.palette().primary
                    } else {
                        Color::TRANSPARENT
                    };
                    container::Style::default().border(iced::Border {
                        color,
                        width: 2.0,
                        radius: 3.0.into(),
                    })
                });
            let name = item
                .path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            mouse_area(
                column![framed, text(name).size(11)]
                    .spacing(2)
                    .align_x(Center)
                    .width(Length::Fill),
            )
            .on_press(Message::Select(index))
            .into()
        });
        scroll(column(entries).spacing(10).padding(8).width(Fill))
            .id(self.sidebar_id.clone())
            .height(Fill)
            .into()
    }
}
