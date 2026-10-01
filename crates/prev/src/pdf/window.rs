//! One PDF document window: opening and unlocking, the toolbar, sidebar,
//! slideshow and printing around the page canvas.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use crate::ui::dir::column;
use crate::{column, row};
use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::scrollable::{Direction, Scrollbar, Viewport};
use iced::widget::{
    Id, container, image, mouse_area, operation, scrollable, space, text, text_input,
};
use iced::{Center, Color, Element, Fill, Length, Task};
use prev_pdf::engine::{Engine, LinkTarget, OutlineItem};
use prev_pdf::mupdf_engine::MupdfEngine;
use prev_pdf::worker::{DocumentHandle, DocumentInfo, Opened, RenderPool, flatten};
use prev_store::bookmarks::{self, Bookmark, BookmarkStore};

use super::bench::{self, Bench, Step};
use super::canvas::PageCanvas;
use super::layout::{Fit, ViewMode};
use super::viewer::editing::EditMessage;
use super::viewer::{PdfMessage, PdfViewer, Request, Zoom};

mod drag_ui;
mod markup_ui;
mod pages_ui;
use crate::i18n::Describe;
use crate::image::editor::spawn;
use crate::portal;
use crate::shortcuts::Action;
use crate::ui::button::{self, Kind};
use crate::ui::component::{self, Backdrop};
use crate::ui::resize::{self, Drag, Width};
use crate::ui::{self, Icon, Type, icon, style};
pub use markup_ui::{LoadedSignature, Menu, SignatureTab};
pub use pages_ui::{ExportMessage, PageAction};

const SIDEBAR_WIDTH: Width = Width::new(264.0, 248.0, 480.0);
/// Sidebar width not taken by a thumbnail: margins, frame and scrollbar.
const THUMBNAIL_INSET: f32 = 64.0;
const THUMBNAIL_SPACING: f32 = 41.0;
const LINE_SCROLL: f32 = 48.0;
/// Quiet time after an edit before the document is written.
const AUTOSAVE_DELAY: std::time::Duration = std::time::Duration::from_millis(1500);

fn write_document(path: &std::path::Path, bytes: &[u8], keep_original: bool) -> Result<(), String> {
    if keep_original && let Some(store) = prev_store::versions::VersionStore::default_location() {
        store
            .keep(path)
            .map_err(|error| crate::fl!("pdf-keep-original-failed", error = error.to_string()))?;
    }
    prev_store::atomic::write(path, bytes).map_err(|error| error.to_string())
}

fn engine() -> Arc<dyn Engine> {
    static ENGINE: OnceLock<Arc<dyn Engine>> = OnceLock::new();
    Arc::clone(ENGINE.get_or_init(|| Arc::new(MupdfEngine)))
}

fn render_pool() -> Arc<RenderPool> {
    static POOL: OnceLock<Arc<RenderPool>> = OnceLock::new();
    Arc::clone(POOL.get_or_init(|| RenderPool::new(RenderPool::default_threads())))
}

/// The search field in the toolbar, and how much wider it grows when
/// there is room.
const SEARCH_WIDTH: f32 = 200.0;
const SEARCH_GROWTH: f32 = 160.0;
/// When the search field moves into "More", among the toolbar's slots.
const SEARCH_SLOT_ORDER: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bar {
    Main,
    Markup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sidebar {
    Thumbnails,
    Contents,
    Notes,
    Bookmarks,
}

enum State {
    Opening(DocumentHandle),
    Locked {
        handle: DocumentHandle,
        password: String,
        wrong: bool,
    },
    Ready(Box<PdfViewer>),
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Opened(Result<Opened, String>),
    PasswordChanged(String),
    SubmitPassword,
    Unlocked(Result<Option<DocumentInfo>, String>),
    Viewer(PdfMessage),
    ShowSidebar(Option<Sidebar>),
    ThumbnailsScrolled(Viewport),
    PageInputChanged(String),
    PageInputSubmitted,
    ModeSelected(ModeChoice),
    ToggleBookmark(usize),
    OutlineLoaded(Vec<OutlineItem>),
    BenchIdleDone,
    Print,
    PrintFinished(Result<(), String>),
    OpenUriFinished(Result<(), String>),
    DismissNotice,
    SidebarResized(Drag),
    AutosaveDue(u64),
    Saved(u64, Result<(), String>),
    ToggleMarkupBar,
    Overflow(Option<Bar>),
    Menu(Option<Menu>),
    Edit(EditMessage),
    NotesLoaded(Vec<(usize, Vec<prev_pdf::annotation::Annotation>)>),
    ShowAnnotation(usize, String),
    SignaturesLoaded(Vec<LoadedSignature>),
    PlaceSignature(usize),
    RemoveSignature(usize),
    NewSignature,
    SignatureTab(SignatureTab),
    SignatureStroke(Vec<(f32, f32)>),
    ClearSignature,
    SignatureText(String),
    SignaturePenWidth(f32),
    SignatureInk([u8; 3]),
    SignatureDescription(String),
    ChooseSignatureImage,
    SignatureImageChosen(Result<Vec<PathBuf>, String>),
    SignatureImageMade(Result<Vec<u8>, String>),
    SaveSignature,
    SignatureSaved(Result<(), String>),
    CancelSignature,
    PageAction(PageAction),
    PagesCopied(usize, Result<Vec<u8>, String>),
    InsertChosen(Result<Vec<PathBuf>, String>),
    InsertRead(Result<Vec<Vec<u8>>, String>),
    ThumbnailPressed(usize),
    ThumbnailsMoved(iced::Point),
    ThumbnailsLeft,
    /// The mouse button was let go anywhere in the window.
    ThumbnailsReleased,
    Export(ExportMessage),
    ConfirmRedactions(bool),
    DroppedPdfs(drag_ui::PdfDropChoice),
    /// An image annotation let go over the sidebar, rendered: where among
    /// the pages it goes, its page and id, and whether it moves there.
    AnnotationRendered(
        Result<prev_pdf::engine::Bitmap, String>,
        usize,
        usize,
        String,
        bool,
    ),
    ApplyRedactions,
    /// Turns floating, auto-hiding toolbars on or off; the app handles it.
    ToggleFloatingBars,
    /// Opens the settings window; the app handles it.
    OpenSettings,
    ToggleInspector,
    MetadataLoaded(prev_pdf::engine::Metadata),
    /// What the clipboard held when Paste was pressed.
    Pasted(Result<crate::paste::Clip, String>),
    /// Clipboard text, read without wl-clipboard.
    PastedText(Option<String>),
    /// An area dragged out of the document, rendered.
    AreaDragReady(Result<prev_pdf::engine::Bitmap, String>),
    /// Pages dragged out of the window, as a PDF.
    PagesDragReady(Vec<usize>, Result<Vec<u8>, String>),
    /// An image file dropped on a page, decoded, and where it goes.
    DroppedImage(Option<prev_pdf::engine::Bitmap>, Option<(f32, f32)>),
}

/// Changes the app applies to the window itself, files it opens, or, for
/// an image's markup, a picture for the image window's sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    EnterFullscreen,
    LeaveFullscreen,
    Quit,
    Open(Vec<PathBuf>),
    ToSidebar(prev_pdf::engine::Bitmap),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeChoice(pub ViewMode);

impl ModeChoice {
    fn label(self) -> String {
        match self.0 {
            ViewMode::Continuous => crate::fl!("pdf-view-continuous"),
            ViewMode::SinglePage => crate::fl!("pdf-view-single-page"),
            ViewMode::TwoPages => crate::fl!("pdf-view-two-pages"),
        }
    }

    fn icon(self) -> Icon {
        match self.0 {
            ViewMode::Continuous => Icon::ViewDay,
            ViewMode::SinglePage => Icon::Draft,
            ViewMode::TwoPages => Icon::TwoPager,
        }
    }
}

const MODES: [ModeChoice; 3] = [
    ModeChoice(ViewMode::Continuous),
    ModeChoice(ViewMode::SinglePage),
    ModeChoice(ViewMode::TwoPages),
];

pub struct PdfWindow {
    pub path: PathBuf,
    state: State,
    sidebar: Option<Sidebar>,
    sidebar_width: Width,
    canvas_id: Id,
    search_id: Id,
    thumbnails_id: Id,
    /// Scroll offset and height of the thumbnail list.
    thumbnails_view: (f32, f32),
    page_input_id: Id,
    page_input: Option<String>,
    /// View mode and fit to restore after a slideshow.
    slideshow: Option<(ViewMode, Fit)>,
    device_scale: f32,
    bookmarks: Vec<Bookmark>,
    outline_loaded: bool,
    bench: Option<Bench>,
    notice: Option<String>,
    effects: Vec<Effect>,
    save_generation: u64,
    saved_generation: u64,
    original_kept: bool,
    markup_bar: bool,
    menu: Option<Menu>,
    /// The toolbar whose "More" menu is open.
    overflow: Option<Bar>,
    signatures: Vec<LoadedSignature>,
    signature_dialog: Option<markup_ui::SignatureDialog>,
    notes: Vec<(usize, Vec<prev_pdf::annotation::Annotation>)>,
    text_editor_id: Id,
    field_input_id: Id,
    modifiers: Modifiers,
    thumbnail_drag: Option<pages_ui::ThumbnailDrag>,
    /// Whether the page thumbnails were clicked last, so Delete, Ctrl+C
    /// and Ctrl+A act on pages.
    pages_focus: bool,
    export_dialog: Option<pages_ui::ExportDialog>,
    redact_confirm: bool,
    /// PDFs dropped on the page, waiting for the answer to whether they
    /// join this document or open on their own.
    dropped_pdfs: Option<Vec<PathBuf>>,
    /// Where among the pages the pointer is over the thumbnails, while it
    /// is; or, for an image's markup, `Some` while it is over the image
    /// window's sidebar.
    pointer_over_sidebar: Option<usize>,
    /// Whether the pointer is over the window, for floating toolbars.
    pointer_inside: bool,
    /// The inspector panel, with the document information once loaded.
    inspector: Option<Option<prev_pdf::engine::Metadata>>,
    /// Marking up an image: the document is a page made from the image,
    /// never saved, shown inside the image window with the markup bar.
    image_mode: bool,
    /// For image markup: the zoom and point to show first, handed to the
    /// viewer once the document is open.
    start_view: Option<(f32, (f32, f32))>,
    /// Where the page canvas and the page thumbnails were drawn, for
    /// placing drops.
    canvas_bounds: std::cell::Cell<iced::Rectangle>,
    thumbnails_bounds: std::cell::Cell<iced::Rectangle>,
    /// Pages being dragged out of the window.
    outgoing_pages: Option<drag_ui::OutgoingPages>,
    /// A drag started since the app last asked.
    drag_started: bool,
    /// Where the next inserted pages go, instead of after the selection.
    insert_at: Option<usize>,
    /// Where among the thumbnails something dragged over them would go.
    drop_hover: Option<usize>,
}

/// What an image window shows of a PDF window marking up its image.
pub struct MarkupParts<'a> {
    pub canvas: Element<'a, Message>,
    pub bar: Option<Element<'a, Message>>,
    /// Dialogs and notices, over the whole window.
    pub overlay: Element<'a, Message>,
}

impl PdfWindow {
    /// The notice showing, if any.
    pub fn notice(&self) -> Option<&str> {
        self.notice.as_deref()
    }

    /// Clears the notice if it is still `text`.
    pub fn dismiss_notice(&mut self, text: &str) {
        if self.notice.as_deref() == Some(text) {
            self.notice = None;
        }
    }

    pub fn open(path: PathBuf) -> (Self, Task<Message>) {
        let (handle, opened) = DocumentHandle::open(engine(), path.clone());
        let window = Self {
            path,
            state: State::Opening(handle),
            sidebar: None,
            sidebar_width: SIDEBAR_WIDTH,
            canvas_id: Id::unique(),
            search_id: Id::unique(),
            thumbnails_id: Id::unique(),
            thumbnails_view: (0.0, 600.0),
            page_input_id: Id::unique(),
            page_input: None,
            slideshow: None,
            device_scale: 1.0,
            bookmarks: Vec::new(),
            outline_loaded: false,
            bench: Bench::from_env(),
            notice: None,
            effects: Vec::new(),
            save_generation: 0,
            saved_generation: 0,
            original_kept: false,
            markup_bar: false,
            menu: None,
            overflow: None,
            signatures: Vec::new(),
            signature_dialog: None,
            notes: Vec::new(),
            text_editor_id: Id::unique(),
            field_input_id: Id::unique(),
            modifiers: Modifiers::default(),
            thumbnail_drag: None,
            pages_focus: false,
            export_dialog: None,
            redact_confirm: false,
            dropped_pdfs: None,
            pointer_over_sidebar: None,
            pointer_inside: true,
            inspector: None,
            image_mode: false,
            start_view: None,
            canvas_bounds: std::cell::Cell::default(),
            thumbnails_bounds: std::cell::Cell::default(),
            outgoing_pages: None,
            drag_started: false,
            insert_at: None,
            drop_hover: None,
        };
        let task = Task::perform(opened, |result| {
            Message::Opened(flatten(result).map_err(|error| error.describe()))
        });
        (window, task)
    }

    /// A window for marking up an image, from `path`, a PDF made of it
    /// with `prev_pdf::image_document`. It shows one page fitted to the
    /// view with the markup bar, and never writes the file.
    pub fn open_image_markup(path: PathBuf) -> (Self, Task<Message>) {
        let (mut window, task) = Self::open(path);
        window.image_mode = true;
        window.markup_bar = true;
        (window, task)
    }

    /// Shows the page at `zoom` with the point at `fraction` of its size in
    /// the middle, instead of fitting it, once the document is open.
    pub fn start_at(&mut self, zoom: f32, fraction: (f32, f32)) {
        self.start_view = Some((zoom, fraction));
    }

    /// The zoom and the point of the page in the middle of the view, as
    /// fractions of its size, unless the page is fitted to the view.
    pub fn zoomed_center(&self) -> Option<(f32, (f32, f32))> {
        match &self.state {
            State::Ready(viewer) => viewer.zoomed_center(),
            _ => None,
        }
    }

    /// Whether the page has any annotations.
    pub fn has_annotations(&self) -> bool {
        match &self.state {
            State::Ready(viewer) => viewer
                .markup
                .values()
                .any(|markup| !markup.annotations.is_empty()),
            _ => false,
        }
    }

    /// The document, once open.
    pub fn document(&self) -> Option<&DocumentHandle> {
        match &self.state {
            State::Ready(viewer) => Some(&viewer.handle),
            _ => None,
        }
    }

    /// Counts changes to the document, to tell whether any came after a
    /// point in time.
    pub fn edits(&self) -> u64 {
        self.save_generation
    }

    /// The zoom, in view pixels per page point.
    pub fn zoom(&self) -> Option<f32> {
        match &self.state {
            State::Ready(viewer) => Some(viewer.layout.zoom),
            _ => None,
        }
    }

    /// Whether an edit can be undone, and redone.
    pub fn can_undo(&self) -> (bool, bool) {
        match &self.state {
            State::Ready(viewer) => (
                viewer.edit.history.can_undo(),
                viewer.edit.history.can_redo(),
            ),
            _ => (false, false),
        }
    }

    /// Whether the page is fitted to the view.
    pub fn fits_page(&self) -> bool {
        matches!(&self.state, State::Ready(viewer) if viewer.fit == Fit::Page)
    }

    /// Zooms to `zoom` around the middle of the view.
    pub fn zoom_to(&mut self, zoom: f32) -> Task<Message> {
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        let factor = zoom / viewer.layout.zoom.max(1e-3);
        let anchor = (viewer.view.width / 2.0, viewer.view.height / 2.0);
        self.viewer_update(PdfMessage::Zoom(Zoom::By { factor, anchor }))
    }

    pub fn markup_bar_shown(&self) -> bool {
        self.markup_bar
    }

    /// The canvas, markup bar and dialogs, for an image window to place.
    pub fn markup_parts(&self) -> Option<MarkupParts<'_>> {
        let State::Ready(viewer) = &self.state else {
            return None;
        };
        let full = || Element::from(space().width(Fill).height(Fill));
        let overlay = if self.signature_dialog.is_some() {
            self.signature_dialog_view(full())
        } else if let Some(notice) = &self.notice {
            component::snackbar_above_bar(full(), notice, Message::DismissNotice, self.markup_bar)
        } else {
            space().into()
        };
        Some(MarkupParts {
            canvas: self.canvas(viewer, None),
            bar: self.markup_bar.then(|| self.markup_toolbar(viewer)),
            overlay,
        })
    }

    /// Whether menus or dialogs opened from the markup bar are open, so a
    /// floating bar stays.
    pub fn holds_bars(&self) -> bool {
        self.menu.is_some() || self.overflow.is_some() || self.signature_dialog.is_some()
    }

    pub fn title(&self) -> Option<String> {
        match &self.state {
            State::Ready(viewer) => viewer.info.title.clone(),
            _ => None,
        }
    }

    pub fn take_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }

    pub fn set_pointer_inside(&mut self, inside: bool) {
        self.pointer_inside = inside;
    }

    /// Whether floating toolbars show: while the pointer is over the
    /// window, or something opened from them is still open.
    fn bars_shown(&self) -> bool {
        self.pointer_inside
            || self.menu.is_some()
            || self.overflow.is_some()
            || self.export_dialog.is_some()
            || self.redact_confirm
            || self.signature_dialog.is_some()
    }

    /// Whether thumbnails are being pressed or dragged, so the app sends
    /// the button's release wherever it happens.
    pub fn wants_release(&self) -> bool {
        self.thumbnail_drag.is_some()
    }

    pub fn in_slideshow(&self) -> bool {
        self.slideshow.is_some()
    }

    /// Whether a benchmark wants a message on every frame.
    pub fn wants_frames(&self) -> bool {
        self.bench.as_ref().is_some_and(Bench::is_driving)
    }

    pub fn bench_frame(&mut self, now: std::time::Instant) -> Task<Message> {
        let Some(step) = self.bench.as_mut().and_then(|bench| bench.frame(now)) else {
            return Task::none();
        };
        match step {
            Step::ScrollBy(dy) => self.viewer_update(PdfMessage::ScrollBy { dx: 0.0, dy }),
            Step::Zoom(factor) => {
                let State::Ready(viewer) = &self.state else {
                    return Task::none();
                };
                let anchor = (viewer.view.width / 2.0, viewer.view.height / 2.0);
                self.viewer_update(PdfMessage::Zoom(Zoom::By { factor, anchor }))
            }
            Step::Finish => {
                if let (Some(bench), State::Ready(viewer)) = (&self.bench, &self.state) {
                    let latencies: Vec<_> = viewer.tile_latencies.iter().copied().collect();
                    eprintln!("{}", bench.report(&latencies));
                }
                self.effects.push(Effect::Quit);
                Task::none()
            }
        }
    }

    pub fn set_device_scale(&mut self, scale: f32) -> Task<Message> {
        self.device_scale = scale;
        self.viewer_update(PdfMessage::DeviceScale(scale))
    }

    fn viewer_update(&mut self, message: PdfMessage) -> Task<Message> {
        if let Some(task) = self.image_to_panel(&message) {
            return task;
        }
        // The page canvas does not see the pointer over the image window's
        // sidebar, so while an image annotation moves on an image's markup
        // the app reports where the pointer is.
        if self.image_mode
            && matches!(
                message,
                PdfMessage::Drag { .. } | PdfMessage::Release { .. }
            )
        {
            crate::drag::watch_pointer(
                matches!(message, PdfMessage::Drag { .. }) && self.moving_image(),
            );
        }
        let State::Ready(viewer) = &mut self.state else {
            return Task::none();
        };
        if matches!(message, PdfMessage::Press { .. }) {
            self.pages_focus = false;
        }
        let page_before = viewer.current;
        let text_before = viewer.edit.text.is_some();
        let field_before = viewer.edit.field.as_ref().map(|field| field.field.id);
        let task = viewer.update(message).map(Message::Viewer);
        let mut focus = Vec::new();
        if !text_before && viewer.edit.text.is_some() {
            focus.push(operation::focus(self.text_editor_id.clone()));
        }
        let field_after = viewer.edit.field.as_ref().map(|field| field.field.id);
        if field_after.is_some() && field_after != field_before {
            focus.push(operation::focus(self.field_input_id.clone()));
            focus.push(operation::select_all(self.field_input_id.clone()));
        }
        let task = Task::batch(std::iter::once(task).chain(focus));
        // Moving elsewhere in the document leaves the chosen pages behind.
        if viewer.current != page_before && !viewer.selected_pages.contains(&viewer.current) {
            viewer.selected_pages.clear();
        }
        let requests = viewer.take_requests();
        let follow = if viewer.current == page_before {
            Task::none()
        } else {
            self.follow_thumbnail()
        };
        Task::batch(
            std::iter::once(task)
                .chain(
                    requests
                        .into_iter()
                        .map(|request| self.perform_request(request)),
                )
                .chain(std::iter::once(follow)),
        )
    }

    fn thumbnail_width(&self) -> f32 {
        (self.sidebar_width.value - THUMBNAIL_INSET).max(1.0)
    }

    /// Scrolls the thumbnail list so the current page's thumbnail is in view.
    fn follow_thumbnail(&mut self) -> Task<Message> {
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        if self.sidebar != Some(Sidebar::Thumbnails) {
            return Task::none();
        }
        let heights = viewer
            .info
            .page_sizes
            .iter()
            .map(|size| thumbnail_height(size, self.thumbnail_width()) + THUMBNAIL_SPACING);
        let top: f32 = heights.clone().take(viewer.current).sum();
        let height = heights.clone().nth(viewer.current).unwrap_or(0.0);
        let (offset, visible) = self.thumbnails_view;
        if top >= offset && top + height <= offset + visible {
            return Task::none();
        }
        let target = (top - (visible - height) / 2.0).max(0.0);
        self.thumbnails_view.0 = target;
        operation::scroll_to(
            self.thumbnails_id.clone(),
            scrollable::AbsoluteOffset {
                x: None,
                y: Some(target),
            },
        )
    }

    fn perform_request(&mut self, request: Request) -> Task<Message> {
        match request {
            Request::ScrollTo { x, y } => operation::scroll_to(
                self.canvas_id.clone(),
                scrollable::AbsoluteOffset {
                    x: Some(x),
                    y: Some(y),
                },
            ),
            Request::OpenUri(uri) => Task::perform(portal::open_uri(uri), Message::OpenUriFinished),
            Request::Changed if self.image_mode => {
                self.save_generation += 1;
                self.load_notes()
            }
            Request::Changed => {
                let notes = self.load_notes();
                self.save_generation += 1;
                let generation = self.save_generation;
                Task::batch([
                    notes,
                    Task::perform(
                        spawn(move || std::thread::sleep(AUTOSAVE_DELAY)),
                        move |_| Message::AutosaveDue(generation),
                    ),
                ])
            }
            Request::Notice(notice) => {
                self.notice = Some(notice);
                Task::none()
            }
            Request::DragOut(out) => self.drag_out(out),
            Request::PagesChanged => {
                let State::Ready(viewer) = &self.state else {
                    return Task::none();
                };
                let outline = viewer.handle.outline();
                let (offset, height) = self.thumbnails_view;
                Task::batch([
                    Task::perform(outline, |result| {
                        Message::OutlineLoaded(flatten(result).unwrap_or_default())
                    }),
                    self.thumbnails_for(offset, height),
                    self.follow_thumbnail(),
                ])
            }
        }
    }

    /// Saves edits the autosave has not written yet, for a closing window.
    pub fn flush(&mut self) -> Option<Task<Message>> {
        (!self.image_mode && self.save_generation > self.saved_generation).then(|| {
            let generation = self.save_generation;
            self.autosave(generation)
        })
    }

    /// Writes the document with its edits in place, keeping the original
    /// as a version the first time.
    fn autosave(&mut self, generation: u64) -> Task<Message> {
        if generation != self.save_generation {
            return Task::none();
        }
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        let path = self.path.clone();
        let keep_original = !self.original_kept;
        let receiver = viewer.handle.save(Box::new(move |bytes| {
            write_document(&path, bytes, keep_original)
        }));
        Task::perform(
            async move {
                match receiver.await {
                    Ok(result) => result.map_err(|error| error.describe()),
                    Err(_) => Err(crate::fl!("pdf-document-closed")),
                }
            },
            move |result| Message::Saved(generation, result),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Opened(Ok(Opened::Ready(info))) => self.ready(info),
            Message::Opened(Ok(Opened::NeedsPassword)) => {
                if let State::Opening(handle) = &self.state {
                    self.state = State::Locked {
                        handle: handle.clone(),
                        password: String::new(),
                        wrong: false,
                    };
                }
                Task::none()
            }
            Message::Opened(Err(error)) => {
                self.state = State::Failed(error);
                Task::none()
            }
            Message::PasswordChanged(value) => {
                if let State::Locked { password, .. } = &mut self.state {
                    *password = value;
                }
                Task::none()
            }
            Message::SubmitPassword => match &self.state {
                State::Locked {
                    handle, password, ..
                } => {
                    let receiver = handle.authenticate(password.clone());
                    Task::perform(receiver, |result| {
                        Message::Unlocked(flatten(result).map_err(|error| error.describe()))
                    })
                }
                _ => Task::none(),
            },
            Message::Unlocked(Ok(Some(info))) => self.ready(info),
            Message::Unlocked(Ok(None)) => {
                if let State::Locked {
                    wrong, password, ..
                } = &mut self.state
                {
                    *wrong = true;
                    password.clear();
                }
                Task::none()
            }
            Message::Unlocked(Err(error)) => {
                self.state = State::Failed(error);
                Task::none()
            }
            Message::Viewer(message) => self.viewer_update(message),
            Message::ShowSidebar(sidebar) => {
                self.sidebar = sidebar;
                self.thumbnails_view = (0.0, self.thumbnails_view.1);
                Task::batch([
                    self.thumbnails_for(0.0, 1000.0),
                    self.follow_thumbnail(),
                    self.load_notes(),
                ])
            }
            Message::ThumbnailsScrolled(viewport) => {
                let offset = viewport.absolute_offset().y;
                self.thumbnails_view = (offset, viewport.bounds().height);
                self.thumbnails_for(offset, viewport.bounds().height)
            }
            Message::PageInputChanged(value) => {
                self.page_input = Some(value);
                Task::none()
            }
            Message::PageInputSubmitted => {
                let input = self.page_input.take().unwrap_or_default();
                let State::Ready(viewer) = &self.state else {
                    return Task::none();
                };
                let go_to = match resolve_page(&input, viewer) {
                    Some(page) => self.viewer_update(PdfMessage::GoTo { page, point: None }),
                    None => Task::none(),
                };
                // Hand the keyboard back to the document so shortcuts work.
                let unfocus = iced::advanced::widget::operate(
                    iced::advanced::widget::operation::focusable::unfocus(),
                );
                Task::batch([go_to, unfocus])
            }
            Message::ModeSelected(ModeChoice(mode)) => {
                self.viewer_update(PdfMessage::SetMode(mode))
            }
            Message::BenchIdleDone => {
                let Some(bench) = &mut self.bench else {
                    return Task::none();
                };
                bench.idle_finished();
                // Any change starts the redraws that drive the run.
                self.viewer_update(PdfMessage::ScrollBy { dx: 0.0, dy: 1.0 })
            }
            Message::OutlineLoaded(outline) => {
                self.outline_loaded = true;
                if let State::Ready(viewer) = &mut self.state {
                    if !outline.is_empty() && self.sidebar.is_none() {
                        self.sidebar = Some(Sidebar::Contents);
                    }
                    viewer.info.outline = outline;
                }
                Task::none()
            }
            Message::ToggleBookmark(page) => {
                self.toggle_bookmark(page);
                Task::none()
            }
            Message::Print => {
                let path = self.path.clone();
                let title = self
                    .path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default();
                Task::perform(portal::print(path, title), Message::PrintFinished)
            }
            Message::AutosaveDue(generation) => self.autosave(generation),
            Message::Pasted(result) => self.pasted(result),
            Message::AreaDragReady(result) => self.area_drag_ready(result),
            Message::PagesDragReady(pages, result) => self.pages_drag_ready(pages, result),
            Message::DroppedImage(image, at) => self.dropped_image(image, at),
            Message::PastedText(Some(text)) if !text.trim().is_empty() => {
                self.viewer_update(PdfMessage::Editing(EditMessage::PasteText(text, None)))
            }
            Message::PastedText(_) => {
                self.notice = Some(crate::fl!("pdf-nothing-to-paste"));
                Task::none()
            }
            Message::Saved(generation, result) => {
                match result {
                    Ok(()) => {
                        self.original_kept = true;
                        self.saved_generation = self.saved_generation.max(generation);
                    }
                    Err(error) => self.notice = Some(crate::fl!("pdf-save-failed", error = error)),
                }
                Task::none()
            }
            Message::SidebarResized(drag) => {
                self.sidebar_width.drag(drag);
                if drag != Drag::Ended {
                    return Task::none();
                }
                // Thumbnail heights follow the width, so the list moved.
                let (offset, height) = self.thumbnails_view;
                Task::batch([self.follow_thumbnail(), self.thumbnails_for(offset, height)])
            }
            Message::DismissNotice => {
                self.notice = None;
                Task::none()
            }
            Message::Overflow(bar) => {
                self.overflow = bar;
                Task::none()
            }
            Message::ToggleMarkupBar
            | Message::Menu(_)
            | Message::Edit(_)
            | Message::NotesLoaded(_)
            | Message::ShowAnnotation(..)
            | Message::SignaturesLoaded(_)
            | Message::PlaceSignature(_)
            | Message::RemoveSignature(_)
            | Message::NewSignature
            | Message::SignatureTab(_)
            | Message::SignatureStroke(_)
            | Message::ClearSignature
            | Message::SignatureText(_)
            | Message::SignaturePenWidth(_)
            | Message::SignatureInk(_)
            | Message::SignatureDescription(_)
            | Message::ChooseSignatureImage
            | Message::SignatureImageChosen(_)
            | Message::SignatureImageMade(_)
            | Message::SaveSignature
            | Message::SignatureSaved(_)
            | Message::CancelSignature => self.markup_update(message),
            Message::PageAction(action) => self.page_action(action),
            Message::PagesCopied(count, result) => {
                self.pages_copied(count, result);
                Task::none()
            }
            Message::InsertChosen(Ok(paths)) => self.insert_files(paths),
            Message::InsertChosen(Err(error)) => {
                self.notice = Some(crate::fl!("pdf-file-dialog-failed", error = error));
                Task::none()
            }
            Message::InsertRead(result) => self.insert_read(result),
            Message::ThumbnailPressed(page) => self.thumbnail_pressed(page),
            Message::ThumbnailsMoved(point) => {
                self.thumbnails_moved(point.y);
                self.pointer_over_sidebar = self.drop_gap(point.y);
                if self.moving_image() {
                    self.drop_hover = self.pointer_over_sidebar;
                }
                Task::none()
            }
            Message::ThumbnailsLeft => {
                self.pointer_over_sidebar = None;
                if self.moving_image() {
                    self.drop_hover = None;
                }
                Task::none()
            }
            Message::ThumbnailsReleased => self.thumbnails_released(),
            Message::Export(message) => self.export_update(message),
            Message::ConfirmRedactions(show) => {
                self.redact_confirm = show;
                Task::none()
            }
            Message::ApplyRedactions => self.apply_redactions(),
            Message::DroppedPdfs(choice) => self.dropped_pdfs_choice(choice),
            Message::AnnotationRendered(result, gap, page, id, moving) => {
                self.annotation_rendered(result, gap, page, id, moving)
            }
            Message::ToggleFloatingBars | Message::OpenSettings => Task::none(),
            Message::ToggleInspector => self.toggle_inspector(),
            Message::MetadataLoaded(metadata) => {
                if let Some(inspector) = self.inspector.as_mut() {
                    *inspector = Some(metadata);
                }
                Task::none()
            }
            Message::PrintFinished(result) | Message::OpenUriFinished(result) => {
                self.notice = result.err();
                Task::none()
            }
        }
    }

    fn ready(&mut self, info: DocumentInfo) -> Task<Message> {
        let handle = match &self.state {
            State::Opening(handle) | State::Locked { handle, .. } => handle.clone(),
            _ => return Task::none(),
        };
        if info.page_sizes.is_empty() {
            self.state = State::Failed(crate::fl!("pdf-no-pages"));
            return Task::none();
        }
        if let Some(store) = bookmarks::default_path() {
            self.bookmarks = BookmarkStore::load_from(&store).get(&self.path).to_vec();
        }
        let outline = handle.outline();
        self.state = State::Ready(Box::new(PdfViewer::new(handle, info, render_pool())));
        let outline = Task::perform(outline, |result| {
            Message::OutlineLoaded(flatten(result).unwrap_or_default())
        });
        let scale = self.device_scale;
        let bench = match &mut self.bench {
            Some(bench) => {
                bench.start_idle();
                Task::perform(async { std::thread::sleep(bench::IDLE) }, |()| {
                    Message::BenchIdleDone
                })
            }
            None => Task::none(),
        };
        let image = if self.image_mode {
            let task = Task::batch([
                self.viewer_update(PdfMessage::SetMode(ViewMode::SinglePage)),
                self.viewer_update(PdfMessage::Zoom(Zoom::FitPage)),
            ]);
            if let State::Ready(viewer) = &mut self.state {
                viewer.pending_view = self.start_view.take();
            }
            task
        } else {
            Task::none()
        };
        Task::batch([
            self.viewer_update(PdfMessage::DeviceScale(scale)),
            self.thumbnails_for(0.0, 1000.0),
            outline,
            bench,
            image,
        ])
    }

    fn thumbnails_for(&mut self, offset: f32, height: f32) -> Task<Message> {
        if self.sidebar != Some(Sidebar::Thumbnails) {
            return Task::none();
        }
        let State::Ready(viewer) = &mut self.state else {
            return Task::none();
        };
        let width = (self.sidebar_width.value - THUMBNAIL_INSET).max(1.0);
        let sizes = viewer.info.page_sizes.clone();
        let mut y = 0.0;
        let mut pages = Vec::new();
        for (index, size) in sizes.iter().enumerate() {
            let item_height = thumbnail_height(size, width) + THUMBNAIL_SPACING;
            if y + item_height >= offset - 200.0 && y <= offset + height + 200.0 {
                pages.push(index);
            }
            y += item_height;
        }
        viewer.request_thumbnails(pages).map(Message::Viewer)
    }

    /// Handles a shortcut meant for this window. Returns `None` for actions
    /// that it does not handle.
    pub fn shortcut(&mut self, action: Action) -> Option<Task<Message>> {
        // Marking up an image, the image window handles everything but
        // editing the markup.
        if self.image_mode
            && !matches!(
                action,
                Action::Undo
                    | Action::Redo
                    | Action::Paste
                    | Action::Escape
                    | Action::ShowMarkup
                    | Action::ZoomIn
                    | Action::ZoomOut
                    | Action::ActualSize
                    | Action::ZoomToFit
            )
        {
            return None;
        }
        let State::Ready(viewer) = &mut self.state else {
            return None;
        };
        let task = match action {
            Action::Copy if self.pages_focus => self.page_action(PageAction::Copy),
            Action::Paste => self.paste(),
            Action::SelectAll if self.pages_focus => self.page_action(PageAction::SelectAll),
            Action::RotateLeft => self.page_action(PageAction::RotateLeft),
            Action::RotateRight => self.page_action(PageAction::RotateRight),
            Action::Crop => self.page_action(PageAction::Crop),
            Action::Export => self.page_action(PageAction::Export),
            Action::Inspector => self.toggle_inspector(),
            Action::Escape if self.export_dialog.is_some() => {
                self.export_dialog = None;
                Task::none()
            }
            Action::Escape if self.redact_confirm => {
                self.redact_confirm = false;
                Task::none()
            }
            Action::Copy => match viewer.copy_area() {
                Some(task) => task.map(Message::Viewer),
                None => viewer.copy_selection().map(Message::Viewer),
            },
            Action::SelectAll => {
                viewer.select_all();
                Task::none()
            }
            Action::Find => operation::focus(self.search_id.clone()),
            Action::FindNext => self.viewer_update(PdfMessage::NextMatch),
            Action::FindPrevious => self.viewer_update(PdfMessage::PreviousMatch),
            Action::ZoomIn => self.viewer_update(PdfMessage::Zoom(Zoom::In)),
            Action::ZoomOut => self.viewer_update(PdfMessage::Zoom(Zoom::Out)),
            Action::ActualSize => self.viewer_update(PdfMessage::Zoom(Zoom::ActualSize)),
            Action::ZoomToFit => self.viewer_update(PdfMessage::Zoom(Zoom::FitPage)),
            Action::HideSidebar => self.update(Message::ShowSidebar(None)),
            Action::Thumbnails => self.update(Message::ShowSidebar(Some(Sidebar::Thumbnails))),
            Action::Contents => self.update(Message::ShowSidebar(Some(Sidebar::Contents))),
            Action::BookmarksSidebar => self.update(Message::ShowSidebar(Some(Sidebar::Bookmarks))),
            Action::NotesSidebar => self.update(Message::ShowSidebar(Some(Sidebar::Notes))),
            Action::ShowMarkup => self.update(Message::ToggleMarkupBar),
            Action::Undo if viewer.edit.history.can_undo() && viewer.edit.text.is_none() => {
                self.update(Message::Edit(EditMessage::Undo))
            }
            Action::Redo if viewer.edit.history.can_redo() && viewer.edit.text.is_none() => {
                self.update(Message::Edit(EditMessage::Redo))
            }
            Action::ToggleBookmark => {
                let page = viewer.current;
                self.update(Message::ToggleBookmark(page))
            }
            Action::GoToPage => {
                self.page_input = Some(String::new());
                Task::batch([
                    operation::focus(self.page_input_id.clone()),
                    operation::select_all(self.page_input_id.clone()),
                ])
            }
            Action::NextPage => self.viewer_update(PdfMessage::NextPage),
            Action::PreviousPage => self.viewer_update(PdfMessage::PreviousPage),
            Action::FirstPage => self.viewer_update(PdfMessage::GoTo {
                page: 0,
                point: None,
            }),
            Action::LastPage => {
                let last = viewer.page_count() - 1;
                self.viewer_update(PdfMessage::GoTo {
                    page: last,
                    point: None,
                })
            }
            Action::Print => self.update(Message::Print),
            Action::Slideshow => self.toggle_slideshow(),
            Action::Escape if self.slideshow.is_some() => self.toggle_slideshow(),
            Action::Escape if self.signature_dialog.is_some() => {
                self.update(Message::CancelSignature)
            }
            Action::Escape if self.menu.is_some() => self.update(Message::Menu(None)),
            Action::Escape if self.overflow.is_some() => self.update(Message::Overflow(None)),
            Action::Escape
                if viewer.edit.text.is_some()
                    || viewer.edit.field.is_some()
                    || viewer.edit.choice.is_some()
                    || viewer.edit.selected.is_some()
                    || viewer.edit.tool != super::markup::Tool::Select =>
            {
                self.update(Message::Edit(EditMessage::Deselect))
            }
            Action::Escape => {
                viewer.selection.take()?;
                Task::none()
            }
            _ => return None,
        };
        Some(task)
    }

    /// Pastes what the clipboard holds, whatever tool is chosen: an image
    /// or text onto the current page, or pages copied in prev after the
    /// selected page. Pages go first once their thumbnails were clicked.
    fn paste(&mut self) -> Task<Message> {
        if self.pages_focus && !self.image_mode && pages_ui::has_copied_pages() {
            return self.page_action(PageAction::Paste);
        }
        Task::perform(spawn(crate::paste::read), |result| {
            Message::Pasted(result.unwrap_or_else(|_| Err(crate::fl!("pdf-pasting-stopped"))))
        })
    }

    fn pasted(&mut self, result: Result<crate::paste::Clip, String>) -> Task<Message> {
        use crate::paste::Clip;
        match result {
            Ok(Clip::Image(image)) => self.viewer_update(PdfMessage::Editing(
                EditMessage::PasteImage(Arc::new(image), None),
            )),
            Ok(Clip::Text(text)) => {
                self.viewer_update(PdfMessage::Editing(EditMessage::PasteText(text, None)))
            }
            Ok(Clip::Pages | Clip::Nothing) if !self.image_mode && pages_ui::has_copied_pages() => {
                self.page_action(PageAction::Paste)
            }
            Ok(_) => {
                self.notice = Some(crate::fl!("pdf-nothing-to-paste"));
                Task::none()
            }
            // Without wl-clipboard, text still comes through iced.
            Err(_) => iced::clipboard::read().map(Message::PastedText),
        }
    }

    /// Adds or removes a bookmark, rereading the file first so other windows'
    /// changes are kept.
    fn toggle_bookmark(&mut self, page: usize) {
        let State::Ready(viewer) = &self.state else {
            return;
        };
        let title = bookmark_title(viewer, page);
        let Some(store_path) = bookmarks::default_path() else {
            self.notice = Some(crate::fl!("pdf-bookmarks-no-home"));
            return;
        };
        let mut store = BookmarkStore::load_from(&store_path);
        store.toggle(&self.path, page, title);
        self.bookmarks = store.get(&self.path).to_vec();
        self.notice = store
            .save_to(&store_path)
            .err()
            .map(|error| crate::fl!("pdf-bookmarks-save-failed", error = error.to_string()));
    }

    fn toggle_slideshow(&mut self) -> Task<Message> {
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        match self.slideshow.take() {
            Some((mode, fit)) => {
                self.effects.push(Effect::LeaveFullscreen);
                let zoom = match fit {
                    Fit::Width => Zoom::FitWidth,
                    Fit::Page => Zoom::FitPage,
                    Fit::Zoom(zoom) => Zoom::By {
                        factor: zoom / viewer.layout.zoom,
                        anchor: (0.0, 0.0),
                    },
                };
                Task::batch([
                    self.viewer_update(PdfMessage::SetMode(mode)),
                    self.viewer_update(PdfMessage::Zoom(zoom)),
                ])
            }
            None => {
                self.slideshow = Some((viewer.mode, viewer.fit));
                self.effects.push(Effect::EnterFullscreen);
                Task::batch([
                    self.viewer_update(PdfMessage::SetMode(ViewMode::SinglePage)),
                    self.viewer_update(PdfMessage::Zoom(Zoom::FitPage)),
                ])
            }
        }
    }

    /// Navigation keys without modifiers.
    pub fn key(&mut self, key: &Key, modifiers: Modifiers) -> Option<Task<Message>> {
        let State::Ready(viewer) = &self.state else {
            return None;
        };
        if modifiers.control() || modifiers.alt() || modifiers.logo() {
            return None;
        }
        let deletes = matches!(key.as_ref(), Key::Named(Named::Delete | Named::Backspace));
        if deletes && viewer.edit.selected.is_some() && viewer.edit.text.is_none() {
            return Some(self.update(Message::Edit(EditMessage::Delete)));
        }
        if deletes && self.pages_focus && self.sidebar == Some(Sidebar::Thumbnails) {
            return Some(self.page_action(PageAction::Delete));
        }
        if self.image_mode {
            return None;
        }
        let paged = self.slideshow.is_some() || viewer.mode != ViewMode::Continuous;
        let page_height = viewer.view.height * 0.9;
        let message = match key.as_ref() {
            Key::Named(Named::ArrowDown) if paged && self.slideshow.is_some() => {
                PdfMessage::NextPage
            }
            Key::Named(Named::ArrowUp) if paged && self.slideshow.is_some() => {
                PdfMessage::PreviousPage
            }
            Key::Named(Named::ArrowRight) if paged => PdfMessage::NextPage,
            Key::Named(Named::ArrowLeft) if paged => PdfMessage::PreviousPage,
            Key::Named(Named::PageDown) if paged => PdfMessage::NextPage,
            Key::Named(Named::PageUp) if paged => PdfMessage::PreviousPage,
            Key::Named(Named::Space) if paged && !modifiers.shift() => PdfMessage::NextPage,
            Key::Named(Named::Space) if paged => PdfMessage::PreviousPage,
            Key::Named(Named::ArrowDown) => PdfMessage::ScrollBy {
                dx: 0.0,
                dy: LINE_SCROLL,
            },
            Key::Named(Named::ArrowUp) => PdfMessage::ScrollBy {
                dx: 0.0,
                dy: -LINE_SCROLL,
            },
            Key::Named(Named::ArrowRight) => PdfMessage::ScrollBy {
                dx: LINE_SCROLL,
                dy: 0.0,
            },
            Key::Named(Named::ArrowLeft) => PdfMessage::ScrollBy {
                dx: -LINE_SCROLL,
                dy: 0.0,
            },
            Key::Named(Named::PageDown) => PdfMessage::ScrollBy {
                dx: 0.0,
                dy: page_height,
            },
            Key::Named(Named::PageUp) => PdfMessage::ScrollBy {
                dx: 0.0,
                dy: -page_height,
            },
            Key::Named(Named::Space) if modifiers.shift() => PdfMessage::ScrollBy {
                dx: 0.0,
                dy: -page_height,
            },
            Key::Named(Named::Space) => PdfMessage::ScrollBy {
                dx: 0.0,
                dy: page_height,
            },
            _ => return None,
        };
        Some(self.viewer_update(message))
    }

    pub fn view(&self) -> Element<'_, Message> {
        let name = self
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let body: Element<'_, Message> = match &self.state {
            State::Opening(_) => {
                component::empty_state(Icon::Draft, crate::fl!("pdf-opening"), name)
            }
            State::Failed(error) => {
                component::empty_state(Icon::Error, crate::fl!("pdf-open-failed"), error.as_str())
            }
            State::Locked {
                password, wrong, ..
            } => self.password_view(name, password, *wrong),
            State::Ready(viewer) if self.slideshow.is_some() => {
                self.canvas(viewer, Some(Color::BLACK))
            }
            State::Ready(viewer) => {
                // Hidden parts leave an empty slot, so the canvas keeps its
                // place in the widget tree, and with it the scroll position.
                let (sidebar, handle): (Element<'_, Message>, Element<'_, Message>) =
                    match self.sidebar {
                        Some(sidebar) => (
                            ui::enter::from_left(
                                container(self.sidebar_view(viewer, sidebar))
                                    .clip(true)
                                    .width(self.sidebar_width.value)
                                    .height(Fill)
                                    .style(style::surface_container_low),
                            ),
                            resize::handle(Message::SidebarResized).into(),
                        ),
                        None => (space().into(), space().into()),
                    };
                let markup = self.markup_bar.then(|| self.markup_toolbar(viewer));
                let (top, bottom) = (
                    component::floating_room(true),
                    component::floating_room(self.markup_bar),
                );
                // The panels keep their sides in every language.
                let mut content = iced::widget::row![
                    component::between_bars(sidebar, top, bottom),
                    component::between_bars(handle, top, bottom),
                    self.canvas(viewer, None)
                ];
                // Last, so opening it leaves the canvas where it is in the
                // widget tree.
                if let Some(metadata) = &self.inspector {
                    content = content.push(component::between_bars(
                        self.inspector_view(viewer, metadata.as_ref()),
                        top,
                        bottom,
                    ));
                }
                component::window_bars(
                    self.toolbar(viewer),
                    markup,
                    content.into(),
                    self.bars_shown(),
                )
            }
        };
        let body = container(body)
            .width(Fill)
            .height(Fill)
            .style(style::surface);
        // Stacks take their size from the first layer.
        let full = || Element::from(space().width(Fill).height(Fill));
        // Layers over the document are always in the tree, empty when not
        // shown, so opening one keeps the scroll position underneath.
        let notice: Element<'_, Message> = match &self.notice {
            Some(notice) => component::snackbar_above_bar(
                full(),
                notice,
                Message::DismissNotice,
                self.markup_bar,
            ),
            None => space().into(),
        };
        let dialog: Element<'_, Message> = if self.signature_dialog.is_some() {
            self.signature_dialog_view(full())
        } else if self.redact_confirm {
            self.redact_dialog(full())
        } else if self.export_dialog.is_some() {
            self.export_dialog_view(full())
        } else if self.dropped_pdfs.is_some() {
            self.dropped_pdfs_dialog(full())
        } else {
            space().into()
        };
        iced::widget::stack![body, notice, dialog].into()
    }

    fn password_view<'a>(
        &'a self,
        name: String,
        password: &'a str,
        wrong: bool,
    ) -> Element<'a, Message> {
        let mut content = column![
            icon::icon(Icon::Lock, 48).style(style::on_surface_variant),
            ui::styled(
                crate::fl!("pdf-password-protected", name = name),
                Type::TitleLarge
            ),
            container(component::text_field(
                crate::i18n::lasting(crate::fl!("pdf-password")),
                password,
                Backdrop::Surface,
                |input| input
                    .secure(true)
                    .on_input(Message::PasswordChanged)
                    .on_submit(Message::SubmitPassword),
            ))
            .width(320),
        ]
        .spacing(16)
        .align_x(Center);
        if wrong {
            content = content.push(ui::aligned(
                ui::styled(crate::fl!("pdf-password-wrong"), Type::BodyMedium)
                    .style(style::error_text),
            ));
        }
        content = content.push(
            ui::button(Kind::Filled, crate::fl!("pdf-unlock")).on_press(Message::SubmitPassword),
        );
        container(content).center(Fill).into()
    }

    fn canvas<'a>(
        &'a self,
        viewer: &'a PdfViewer,
        backdrop: Option<Color>,
    ) -> Element<'a, Message> {
        let mut canvas = PageCanvas::new(viewer, Message::Viewer);
        if let Some(color) = backdrop {
            canvas = canvas.backdrop(color);
        }
        let scrollbar = if backdrop.is_some() {
            Scrollbar::hidden()
        } else {
            component::thin_scrollbar()
        };
        let scroll = scrollable(canvas)
            .id(self.canvas_id.clone())
            .direction(Direction::Both {
                vertical: scrollbar,
                horizontal: scrollbar,
            })
            .style(style::scrollbar)
            .width(Fill)
            .height(Fill);
        if backdrop.is_some() {
            return scroll.into();
        }
        let scroll = ui::probe::probe(scroll, &self.canvas_bounds);
        let editors = self.page_editors(viewer).unwrap_or_else(|| space().into());
        iced::widget::stack![scroll, editors].into()
    }

    /// The toolbar, with groups that do not fit the window's width in a
    /// "More" menu at its end.
    fn toolbar<'a>(&'a self, viewer: &'a PdfViewer) -> Element<'a, Message> {
        component::toolbar(iced::widget::responsive(move |size| {
            self.toolbar_at(viewer, size.width)
        }))
    }

    fn toolbar_at<'a>(&'a self, viewer: &'a PdfViewer, width: f32) -> Element<'a, Message> {
        let _fixed = ui::dir::fixed();
        let sidebar_toggle = component::toggle_tool(
            if self.sidebar.is_some() {
                Icon::LeftPanelClose
            } else {
                Icon::LeftPanelOpen
            },
            crate::fl!("pdf-sidebar"),
            self.sidebar.is_some(),
            Message::ShowSidebar(match self.sidebar {
                Some(_) => None,
                None => Some(Sidebar::Thumbnails),
            }),
        );
        let page_label = viewer.page_label(viewer.current);
        let page_box = container(
            text_input("", self.page_input.as_deref().unwrap_or(&page_label))
                .id(self.page_input_id.clone())
                .on_input(Message::PageInputChanged)
                .on_submit(Message::PageInputSubmitted)
                .style(style::outlined_field)
                .align_x(Center)
                .padding([6, 4])
                .width(52),
        );
        let pages_label = crate::fl!("pdf-page-of", count = viewer.page_count());
        let pages_width = ui::font::measure(&pages_label, Type::BodyMedium).ceil();
        let pages = ui::styled(pages_label, Type::BodyMedium).style(style::on_surface_variant);
        let zoom = |glyph: Icon, label: String, zoom: Zoom| {
            component::tool(glyph, label, Some(Message::Viewer(PdfMessage::Zoom(zoom))))
        };
        let percent = ui::styled(
            crate::fl!(
                "pdf-zoom-percent",
                percent = ((viewer.layout.zoom * 100.0).round() as i64)
            ),
            Type::LabelLarge,
        )
        .width(48)
        .align_x(Center);
        let fit = |glyph: Icon, label: String, selected: bool, zoom: Zoom| {
            component::tip(
                ui::icon_button(glyph)
                    .selected(selected)
                    .on_press(Message::Viewer(PdfMessage::Zoom(zoom))),
                label,
            )
        };
        let actual_size = matches!(viewer.fit, Fit::Zoom(zoom) if (zoom - 1.0).abs() < 1e-3);
        let modes = component::connected_with_tips(
            MODES
                .iter()
                .map(|mode| {
                    (
                        ui::icon_button(mode.icon())
                            .kind(Kind::Tonal)
                            .size(button::Size::ExtraSmall)
                            .selected(viewer.mode == mode.0)
                            .on_press(Message::ModeSelected(*mode)),
                        Some(mode.label().into()),
                    )
                })
                .collect(),
        );

        let search = &viewer.search;
        let matches = if search.query.trim().is_empty() {
            String::new()
        } else if search.matches.is_empty() && search.finished {
            crate::fl!("pdf-search-not-found")
        } else if search.matches.is_empty() {
            crate::fl!("pdf-searching")
        } else {
            let current = search.current.map_or(0, |current| current + 1);
            let total = search.matches.len();
            if search.finished {
                crate::fl!("pdf-search-match", current = current, total = total)
            } else {
                crate::fl!("pdf-search-match-more", current = current, total = total)
            }
        };
        let has_matches = !search.matches.is_empty();
        let search_bar = |width: f32| {
            component::search_bar(
                text_input(
                    crate::i18n::lasting(crate::fl!("pdf-search")),
                    &search.query,
                )
                .align_x(crate::ui::dir::input_align(&search.query))
                .id(self.search_id.clone())
                .on_input(|query| Message::Viewer(PdfMessage::SearchChanged(query)))
                .on_submit(Message::Viewer(PdfMessage::NextMatch)),
                vec![
                    ui::styled(matches, Type::LabelMedium)
                        .style(style::on_surface_variant)
                        .wrapping(text::Wrapping::None)
                        .into(),
                    ui::icon_button(Icon::KeyboardArrowUp)
                        .size(button::Size::ExtraSmall)
                        .on_press_maybe(
                            has_matches.then_some(Message::Viewer(PdfMessage::PreviousMatch)),
                        )
                        .into(),
                    ui::icon_button(Icon::KeyboardArrowDown)
                        .size(button::Size::ExtraSmall)
                        .on_press_maybe(
                            has_matches.then_some(Message::Viewer(PdfMessage::NextMatch)),
                        )
                        .into(),
                ],
                width,
            )
        };

        use component::{DIVIDER_WIDTH, TOOL_WIDTH};
        // Each slot: its content, its width, and when it moves into "More".
        let mut slots: Vec<(Element<'a, Message>, f32, Option<u8>)> = vec![
            (
                row![sidebar_toggle, component::toolbar_divider()]
                    .spacing(8)
                    .align_y(Center)
                    .into(),
                TOOL_WIDTH + DIVIDER_WIDTH + 8.0,
                None,
            ),
            (
                row![
                    {
                        // "Page 1 of 9" reads in the interface's direction:
                        // the page box is on the right in right to left
                        // languages.
                        let _reading = ui::dir::reading();
                        row![page_box, pages].spacing(8).align_y(Center)
                    },
                    component::toolbar_divider()
                ]
                .spacing(8)
                .align_y(Center)
                .into(),
                52.0 + pages_width + DIVIDER_WIDTH + 16.0,
                None,
            ),
            (
                component::group([
                    zoom(Icon::ZoomOut, crate::fl!("pdf-zoom-out"), Zoom::Out),
                    percent.into(),
                    zoom(Icon::ZoomIn, crate::fl!("pdf-zoom-in"), Zoom::In),
                ]),
                TOOL_WIDTH * 2.0 + 48.0 + 8.0,
                Some(3),
            ),
            (
                component::group([
                    fit(
                        Icon::FitPage,
                        crate::fl!("pdf-fit-page"),
                        viewer.fit == Fit::Page,
                        Zoom::FitPage,
                    ),
                    fit(
                        Icon::FitWidth,
                        crate::fl!("pdf-fit-width"),
                        viewer.fit == Fit::Width,
                        Zoom::FitWidth,
                    ),
                    fit(
                        Icon::OneToOne,
                        crate::fl!("pdf-actual-size"),
                        actual_size,
                        Zoom::ActualSize,
                    ),
                ]),
                TOOL_WIDTH * 3.0 + 8.0,
                Some(1),
            ),
            (modes, 3.0 * 32.0 + 4.0, Some(0)),
        ];
        // Undo and redo, unless the markup bar shows its own.
        let undo = !self.markup_bar;
        if undo {
            let history = &viewer.edit.history;
            slots.push((
                row![
                    component::tool(
                        Icon::Undo,
                        crate::fl!("pdf-undo"),
                        history.can_undo().then(|| Message::Edit(EditMessage::Undo)),
                    ),
                    component::tool(
                        Icon::Redo,
                        crate::fl!("pdf-redo"),
                        history.can_redo().then(|| Message::Edit(EditMessage::Redo)),
                    ),
                    component::toolbar_divider(),
                ]
                .spacing(4)
                .align_y(Center)
                .into(),
                TOOL_WIDTH * 2.0 + DIVIDER_WIDTH + 8.0,
                None,
            ));
        }
        slots.extend([
            (
                component::group([
                    component::tool(
                        Icon::RotateLeft,
                        crate::fl!("pdf-rotate-left"),
                        Some(Message::PageAction(PageAction::RotateLeft)),
                    ),
                    component::tool(
                        Icon::RotateRight,
                        crate::fl!("pdf-rotate-right"),
                        Some(Message::PageAction(PageAction::RotateRight)),
                    ),
                    self.pages_menu(viewer),
                ]),
                TOOL_WIDTH * 3.0 + 8.0,
                Some(2),
            ),
            (
                component::group([
                    component::toggle_tool(
                        Icon::Info,
                        crate::fl!("pdf-inspector"),
                        self.inspector.is_some(),
                        Message::ToggleInspector,
                    ),
                    component::toggle_tool(
                        Icon::EditDocument,
                        crate::fl!("pdf-markup"),
                        self.markup_bar,
                        Message::ToggleMarkupBar,
                    ),
                ]),
                DIVIDER_WIDTH + TOOL_WIDTH * 2.0 + 12.0,
                None,
            ),
            // Made once the room left over is known.
            (space().into(), SEARCH_WIDTH, Some(SEARCH_SLOT_ORDER)),
            (
                component::tip(
                    ui::icon_button(Icon::FileExport)
                        .kind(Kind::Tonal)
                        .on_press(Message::PageAction(PageAction::Export)),
                    crate::fl!("pdf-export"),
                ),
                component::TOOL_WIDTH,
                Some(5),
            ),
            (
                component::group([
                    component::floating_bars_toggle(Message::ToggleFloatingBars),
                    component::tool(
                        Icon::Settings,
                        crate::fl!("pdf-settings"),
                        Some(Message::OpenSettings),
                    ),
                ]),
                TOOL_WIDTH * 2.0 + 4.0,
                None,
            ),
        ]);
        let widths: Vec<(f32, Option<u8>)> = slots
            .iter()
            .map(|(_, width, order)| (*width, *order))
            .collect();
        let shown = component::fitting_slots(width, &widths);
        // The search field takes room the other slots leave, so a query
        // shows beside its match count.
        let used: f32 = widths
            .iter()
            .zip(&shown)
            .filter(|(_, shown)| **shown)
            .map(|((width, _), _)| width + component::TOOLBAR_GAP)
            .sum();
        let more = if shown.contains(&false) {
            TOOL_WIDTH + component::TOOLBAR_GAP
        } else {
            0.0
        };
        let search_width = SEARCH_WIDTH + (width - used - more).clamp(0.0, SEARCH_GROWTH);
        if let Some(slot) = slots
            .iter_mut()
            .find(|(_, _, order)| *order == Some(SEARCH_SLOT_ORDER))
        {
            slot.0 = search_bar(search_width);
        }
        // The right side: undo, page tools, then the panels, set apart
        // from the page tools while both are shown.
        let right = 5;
        let page_tools = right + usize::from(undo);
        let page_tools_shown = shown[page_tools];
        let mut bar = crate::line![].spacing(8).align_y(Center);
        let mut hidden = Vec::new();
        for (index, ((element, _, _), shown)) in slots.into_iter().zip(shown).enumerate() {
            // Page tools, the markup button and search bar sit on the right.
            if index == right {
                bar = bar.push(space::horizontal());
            }
            if index == page_tools + 1 && page_tools_shown {
                bar = bar.push(component::toolbar_divider());
            }
            if shown {
                bar = bar.push(element);
            } else {
                hidden.push(element);
            }
        }
        if !hidden.is_empty() {
            let open = self.overflow == Some(Bar::Main);
            bar = bar.push(component::overflow(
                hidden,
                open,
                Message::Overflow((!open).then_some(Bar::Main)),
                Message::Overflow(None),
            ));
        }
        // The bar fills the toolbar's height; keep the buttons in its middle.
        container(bar).height(Fill).align_y(Center).into()
    }

    fn toggle_inspector(&mut self) -> Task<Message> {
        if self.inspector.take().is_some() {
            return Task::none();
        }
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        self.inspector = Some(None);
        Task::perform(viewer.handle.metadata(), |result| {
            Message::MetadataLoaded(result.unwrap_or_default())
        })
    }

    /// Everything about the document: the file, its information entries,
    /// and its pages.
    fn inspector_view<'a>(
        &'a self,
        viewer: &'a PdfViewer,
        metadata: Option<&'a prev_pdf::engine::Metadata>,
    ) -> Element<'a, Message> {
        use crate::info::{self, Fact};
        let fact = |label: String, value: String| -> Fact { (label, value) };
        let mut document = Vec::new();
        if let Some(metadata) = metadata {
            document.extend([
                fact(crate::fl!("pdf-inspector-title"), metadata.title.clone()),
                fact(crate::fl!("pdf-inspector-author"), metadata.author.clone()),
                fact(
                    crate::fl!("pdf-inspector-subject"),
                    metadata.subject.clone(),
                ),
                fact(
                    crate::fl!("pdf-inspector-keywords"),
                    metadata.keywords.clone(),
                ),
                fact(
                    crate::fl!("pdf-inspector-created"),
                    info::pdf_date(&metadata.created),
                ),
                fact(
                    crate::fl!("pdf-inspector-modified"),
                    info::pdf_date(&metadata.modified),
                ),
                fact(
                    crate::fl!("pdf-inspector-application"),
                    metadata.creator.clone(),
                ),
                fact(
                    crate::fl!("pdf-inspector-producer"),
                    metadata.producer.clone(),
                ),
                fact(crate::fl!("pdf-inspector-version"), metadata.format.clone()),
                fact(
                    crate::fl!("pdf-inspector-security"),
                    match metadata.encryption.as_str() {
                        "" | "None" => crate::fl!("pdf-inspector-not-encrypted"),
                        other => crate::fl!("pdf-inspector-encrypted", method = other),
                    },
                ),
            ]);
        }
        let count = viewer.page_count();
        let mut pages = vec![fact(
            crate::fl!("pdf-inspector-pages"),
            crate::fl!("pdf-inspector-page-count", count = count),
        )];
        if let Some(size) = viewer.info.page_sizes.get(viewer.current) {
            let millimetres = |points: f32| points / 72.0 * 25.4;
            pages.push(fact(
                crate::fl!("pdf-inspector-page-size"),
                crate::fl!(
                    "pdf-inspector-page-size-value",
                    width_mm = format!("{:.0}", millimetres(size.width)),
                    height_mm = format!("{:.0}", millimetres(size.height)),
                    width_in = format!("{:.2}", size.width / 72.0),
                    height_in = format!("{:.2}", size.height / 72.0)
                ),
            ));
        }
        let content: Element<'a, Message> = if metadata.is_none() {
            ui::styled(crate::fl!("pdf-loading"), Type::BodyMedium)
                .style(style::on_surface_variant)
                .into()
        } else {
            info::sections_view(vec![
                (
                    crate::i18n::lasting(crate::fl!("pdf-inspector-file")),
                    info::file_facts(&self.path),
                ),
                (
                    crate::i18n::lasting(crate::fl!("pdf-inspector-document")),
                    document,
                ),
                (
                    crate::i18n::lasting(crate::fl!("pdf-inspector-pages")),
                    pages,
                ),
            ])
        };
        component::side_sheet(
            crate::i18n::lasting(crate::fl!("pdf-inspector")),
            Message::ToggleInspector,
            content,
        )
    }

    fn sidebar_view<'a>(&'a self, viewer: &'a PdfViewer, sidebar: Sidebar) -> Element<'a, Message> {
        let tab = |label: &'a str, glyph: Icon, which: Sidebar| component::Tab {
            label,
            icon: Some(glyph),
            selected: sidebar == which,
            on_press: Message::ShowSidebar(Some(which)),
        };
        let tabs = component::tabs(vec![
            tab(
                crate::i18n::lasting(crate::fl!("pdf-tab-pages")),
                Icon::GridView,
                Sidebar::Thumbnails,
            ),
            tab(
                crate::i18n::lasting(crate::fl!("pdf-tab-contents")),
                Icon::Toc,
                Sidebar::Contents,
            ),
            tab(
                crate::i18n::lasting(crate::fl!("pdf-tab-notes")),
                Icon::StickyNote,
                Sidebar::Notes,
            ),
            tab(
                crate::i18n::lasting(crate::fl!("pdf-tab-bookmarks")),
                Icon::Bookmarks,
                Sidebar::Bookmarks,
            ),
        ]);
        let list: Element<'a, Message> = match sidebar {
            Sidebar::Thumbnails => {
                let gap = self.dragging_gap();
                let count = viewer.page_count();
                let thumbnails = viewer
                    .info
                    .page_sizes
                    .iter()
                    .enumerate()
                    .map(|(page, size)| {
                        let dragged = self.is_dragged(page);
                        thumbnail(
                            viewer,
                            page,
                            *size,
                            self.thumbnail_width(),
                            gap == Some(page),
                            dragged,
                        )
                    });
                let list = column(thumbnails)
                    .push(pages_ui::drop_marker(gap == Some(count)))
                    .spacing(0)
                    .width(Fill)
                    .align_x(Center);
                let list = mouse_area(container(list).padding([0, 16]))
                    .on_move(Message::ThumbnailsMoved)
                    .on_exit(Message::ThumbnailsLeft);
                ui::probe::probe(
                    component::scroll(list)
                        .on_scroll(Message::ThumbnailsScrolled)
                        .id(self.thumbnails_id.clone())
                        .height(Fill),
                    &self.thumbnails_bounds,
                )
            }
            Sidebar::Contents if !self.outline_loaded => {
                component::empty_state(Icon::Toc, crate::fl!("pdf-loading"), "")
            }
            Sidebar::Contents if viewer.info.outline.is_empty() => component::empty_state(
                Icon::Toc,
                crate::fl!("pdf-no-outline"),
                crate::fl!("pdf-no-outline-detail"),
            ),
            Sidebar::Notes => self.notes_view(viewer),
            Sidebar::Bookmarks if self.bookmarks.is_empty() => component::empty_state(
                Icon::Bookmarks,
                crate::fl!("pdf-no-bookmarks"),
                match crate::shortcuts::label(Action::ToggleBookmark) {
                    Some(keys) => crate::fl!("pdf-no-bookmarks-detail", keys = keys),
                    None => crate::fl!("pdf-no-bookmarks-detail-unbound"),
                },
            ),
            Sidebar::Bookmarks => component::scroll(
                column(self.bookmarks.iter().map(|bookmark| {
                    let label = format!("{}  {}", viewer.page_label(bookmark.page), bookmark.title);
                    row![
                        component::content_list_row(
                            Some(Icon::Bookmark),
                            label,
                            0.0,
                            bookmark.page == viewer.current,
                            Some(Message::Viewer(PdfMessage::GoTo {
                                page: bookmark.page,
                                point: None
                            })),
                        ),
                        component::tip(
                            ui::icon_button(Icon::Close)
                                .size(button::Size::ExtraSmall)
                                .on_press(Message::ToggleBookmark(bookmark.page)),
                            crate::fl!("pdf-remove-bookmark")
                        ),
                    ]
                    .spacing(4)
                    .align_y(Center)
                    .into()
                }))
                .spacing(2)
                .padding(12)
                .width(Fill),
            )
            .height(Fill)
            .into(),
            Sidebar::Contents => {
                let mut entries = Vec::new();
                outline_entries(&viewer.info.outline, 0, viewer.current, &mut entries);
                component::scroll(column(entries).spacing(2).padding(12).width(Fill))
                    .height(Fill)
                    .into()
            }
        };
        column![tabs, list].into()
    }
}

fn thumbnail_height(size: &prev_pdf::geometry::Size, width: f32) -> f32 {
    width * size.height / size.width.max(1.0)
}

fn thumbnail<'a>(
    viewer: &'a PdfViewer,
    page: usize,
    size: prev_pdf::geometry::Size,
    width: f32,
    marker: bool,
    dragged: bool,
) -> Element<'a, Message> {
    let height = thumbnail_height(&size, width);
    let picture: Element<'a, Message> = match viewer.previews.get(&page) {
        Some(handle) => image(handle.clone())
            .width(width)
            .height(height)
            .border_radius(style::THUMBNAIL_RADIUS)
            .opacity(if dragged { 0.4_f32 } else { 1.0 })
            .into(),
        None => container(space::horizontal())
            .width(width)
            .height(height)
            .style(|_| container::Style {
                background: Some(Color::WHITE.into()),
                border: iced::border::rounded(style::THUMBNAIL_RADIUS),
                ..container::Style::default()
            })
            .into(),
    };
    let current = page == viewer.current;
    let selected = if viewer.selected_pages.is_empty() {
        current
    } else {
        viewer.selected_pages.contains(&page)
    };
    let framed = container(picture)
        .padding(style::THUMBNAIL_RING)
        .style(move |theme: &iced::Theme| style::thumbnail(theme, selected));
    let label = ui::styled(viewer.page_label(page), Type::LabelMedium);
    let label = if current {
        label.style(style::primary_text)
    } else {
        label.style(style::on_surface_variant)
    };
    column![
        pages_ui::drop_marker(marker),
        mouse_area(column![framed, label].align_x(Center).spacing(4))
            .on_press(Message::ThumbnailPressed(page))
            .interaction(iced::mouse::Interaction::Pointer),
    ]
    .align_x(Center)
    .spacing(4)
    .height(Length::Fixed(height + THUMBNAIL_SPACING))
    .into()
}

fn outline_entries<'a>(
    items: &'a [OutlineItem],
    depth: usize,
    current: usize,
    entries: &mut Vec<Element<'a, Message>>,
) {
    for item in items {
        let target = match &item.target {
            Some(LinkTarget::Page { index, point }) => Some((*index, *point)),
            Some(LinkTarget::Uri(_)) | None => None,
        };
        let selected = target.is_some_and(|(index, _)| index == current);
        let message = target.map(|(page, point)| Message::Viewer(PdfMessage::GoTo { page, point }));
        entries.push(
            component::content_list_row(
                None,
                item.title.as_str(),
                depth as f32 * 12.0,
                selected,
                message,
            )
            .into(),
        );
        outline_entries(&item.children, depth + 1, current, entries);
    }
}

/// The title of the last outline entry starting at or before `page`, or the
/// page label.
fn bookmark_title(viewer: &PdfViewer, page: usize) -> String {
    fn walk<'a>(items: &'a [OutlineItem], page: usize, best: &mut Option<(usize, &'a str)>) {
        for item in items {
            if let Some(LinkTarget::Page { index, .. }) = item.target
                && index <= page
                && best.is_none_or(|(best_index, _)| index >= best_index)
            {
                *best = Some((index, &item.title));
            }
            walk(&item.children, page, best);
        }
    }
    let mut best = None;
    walk(&viewer.info.outline, page, &mut best);
    best.map_or_else(
        || crate::fl!("pdf-bookmark-page", page = viewer.page_label(page)),
        |(_, title)| title.to_owned(),
    )
}

/// A page label or 1-based page number to a page index.
fn resolve_page(input: &str, viewer: &PdfViewer) -> Option<usize> {
    let input = input.trim();
    if let Some(page) = viewer
        .info
        .page_labels
        .iter()
        .position(|label| label.as_deref() == Some(input))
    {
        return Some(page);
    }
    let number: usize = input.parse().ok()?;
    (1..=viewer.page_count())
        .contains(&number)
        .then(|| number - 1)
}
