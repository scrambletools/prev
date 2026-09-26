//! One PDF document window: opening and unlocking, the toolbar, sidebar,
//! slideshow and printing around the page canvas.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::scrollable::{Direction, Scrollbar, Viewport};
use iced::widget::{
    Id, column, container, image, mouse_area, operation, row, scrollable, space, text, text_input,
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

mod markup_ui;
mod pages_ui;
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
            .map_err(|error| format!("could not keep the original version: {error}"))?;
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
    /// The mouse button was let go anywhere in the window.
    ThumbnailsReleased,
    Export(ExportMessage),
    ConfirmRedactions(bool),
    ApplyRedactions,
}

/// Changes the app applies to the window itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    EnterFullscreen,
    LeaveFullscreen,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeChoice(pub ViewMode);

impl ModeChoice {
    fn label(self) -> &'static str {
        match self.0 {
            ViewMode::Continuous => "Continuous scroll",
            ViewMode::SinglePage => "Single page",
            ViewMode::TwoPages => "Two pages",
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
}

impl PdfWindow {
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
        };
        let task = Task::perform(opened, |result| {
            Message::Opened(flatten(result).map_err(|error| error.to_string()))
        });
        (window, task)
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
        (self.save_generation > self.saved_generation).then(|| {
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
                    Ok(result) => result.map_err(|error| error.to_string()),
                    Err(_) => Err("the document closed".to_owned()),
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
                        Message::Unlocked(flatten(result).map_err(|error| error.to_string()))
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
            Message::Saved(generation, result) => {
                match result {
                    Ok(()) => {
                        self.original_kept = true;
                        self.saved_generation = self.saved_generation.max(generation);
                    }
                    Err(error) => self.notice = Some(format!("Could not save: {error}")),
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
                self.notice = Some(format!("Could not show the file dialog: {error}"));
                Task::none()
            }
            Message::InsertRead(result) => self.insert_read(result),
            Message::ThumbnailPressed(page) => self.thumbnail_pressed(page),
            Message::ThumbnailsMoved(point) => {
                self.thumbnails_moved(point.y);
                Task::none()
            }
            Message::ThumbnailsReleased => self.thumbnails_released(),
            Message::Export(message) => self.export_update(message),
            Message::ConfirmRedactions(show) => {
                self.redact_confirm = show;
                Task::none()
            }
            Message::ApplyRedactions => self.apply_redactions(),
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
            self.state = State::Failed("The document has no pages.".into());
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
        Task::batch([
            self.viewer_update(PdfMessage::DeviceScale(scale)),
            self.thumbnails_for(0.0, 1000.0),
            outline,
            bench,
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
        let State::Ready(viewer) = &mut self.state else {
            return None;
        };
        let task = match action {
            Action::Copy if self.pages_focus => self.page_action(PageAction::Copy),
            Action::Paste => self.page_action(PageAction::Paste),
            Action::SelectAll if self.pages_focus => self.page_action(PageAction::SelectAll),
            Action::RotateLeft => self.page_action(PageAction::RotateLeft),
            Action::RotateRight => self.page_action(PageAction::RotateRight),
            Action::Crop => self.page_action(PageAction::Crop),
            Action::Export => self.page_action(PageAction::Export),
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

    /// Adds or removes a bookmark, rereading the file first so other windows'
    /// changes are kept.
    fn toggle_bookmark(&mut self, page: usize) {
        let State::Ready(viewer) = &self.state else {
            return;
        };
        let title = bookmark_title(viewer, page);
        let Some(store_path) = bookmarks::default_path() else {
            self.notice = Some("Bookmarks cannot be saved: HOME is not set".into());
            return;
        };
        let mut store = BookmarkStore::load_from(&store_path);
        store.toggle(&self.path, page, title);
        self.bookmarks = store.get(&self.path).to_vec();
        self.notice = store
            .save_to(&store_path)
            .err()
            .map(|error| format!("Could not save bookmarks: {error}"));
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
            State::Opening(_) => component::empty_state(Icon::Draft, "Opening…", name),
            State::Failed(error) => {
                component::empty_state(Icon::Error, "prev can't open this document", error.as_str())
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
                let content = row![sidebar, handle, self.canvas(viewer, None)];
                let markup: Element<'_, Message> = if self.markup_bar {
                    self.markup_toolbar(viewer)
                } else {
                    space().into()
                };
                column![self.toolbar(viewer), markup, content].into()
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
            Some(notice) => component::snackbar(full(), notice, Message::DismissNotice),
            None => space().into(),
        };
        let dialog: Element<'_, Message> = if self.signature_dialog.is_some() {
            self.signature_dialog_view(full())
        } else if self.redact_confirm {
            self.redact_dialog(full())
        } else if self.export_dialog.is_some() {
            self.export_dialog_view(full())
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
            ui::styled(format!("“{name}” is password protected"), Type::TitleLarge),
            container(component::text_field(
                "Password",
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
            content = content.push(
                ui::styled("Incorrect password. Try again.", Type::BodyMedium)
                    .style(style::error_text),
            );
        }
        content =
            content.push(ui::button(Kind::Filled, "Unlock").on_press(Message::SubmitPassword));
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
        let sidebar_toggle = component::toggle_tool(
            if self.sidebar.is_some() {
                Icon::LeftPanelClose
            } else {
                Icon::LeftPanelOpen
            },
            "Sidebar",
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
        let pages = ui::styled(format!("of {}", viewer.page_count()), Type::BodyMedium)
            .style(style::on_surface_variant);
        let zoom = |glyph: Icon, label: &'static str, zoom: Zoom| {
            component::tool(glyph, label, Some(Message::Viewer(PdfMessage::Zoom(zoom))))
        };
        let percent = ui::styled(
            format!("{:.0}%", viewer.layout.zoom * 100.0),
            Type::LabelLarge,
        )
        .width(48)
        .align_x(Center);
        let fit = |glyph: Icon, label: &'static str, fit: Fit, zoom: Zoom| {
            component::tip(
                ui::icon_button(glyph)
                    .selected(viewer.fit == fit)
                    .on_press(Message::Viewer(PdfMessage::Zoom(zoom))),
                label,
            )
        };
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
                        Some(mode.label()),
                    )
                })
                .collect(),
        );

        let search = &viewer.search;
        let matches = if search.query.trim().is_empty() {
            String::new()
        } else if search.matches.is_empty() && search.finished {
            "Not found".to_owned()
        } else if search.matches.is_empty() {
            "Searching…".to_owned()
        } else {
            let current = search.current.map_or(0, |current| current + 1);
            let more = if search.finished { "" } else { "+" };
            format!("{current} of {}{more}", search.matches.len())
        };
        let has_matches = !search.matches.is_empty();
        let search_bar = component::search_bar(
            text_input("Search", &search.query)
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
                    .on_press_maybe(has_matches.then_some(Message::Viewer(PdfMessage::NextMatch)))
                    .into(),
            ],
            280.0,
        );

        use component::{DIVIDER_WIDTH, TOOL_WIDTH};
        // Each slot: its content, its width, and when it moves into "More".
        let slots: Vec<(Element<'a, Message>, f32, Option<u8>)> = vec![
            (
                row![sidebar_toggle, component::toolbar_divider()]
                    .spacing(8)
                    .align_y(Center)
                    .into(),
                TOOL_WIDTH + DIVIDER_WIDTH + 8.0,
                None,
            ),
            (
                row![page_box, pages, component::toolbar_divider()]
                    .spacing(8)
                    .align_y(Center)
                    .into(),
                52.0 + 40.0 + DIVIDER_WIDTH + 16.0,
                None,
            ),
            (
                component::group([
                    zoom(Icon::ZoomOut, "Zoom out", Zoom::Out),
                    percent.into(),
                    zoom(Icon::ZoomIn, "Zoom in", Zoom::In),
                ]),
                TOOL_WIDTH * 2.0 + 48.0 + 8.0,
                Some(3),
            ),
            (
                component::group([
                    fit(Icon::FitPage, "Fit page", Fit::Page, Zoom::FitPage),
                    fit(Icon::FitWidth, "Fit width", Fit::Width, Zoom::FitWidth),
                ]),
                TOOL_WIDTH * 2.0 + 4.0,
                Some(1),
            ),
            (modes, 3.0 * 32.0 + 4.0, Some(0)),
            (
                component::group([
                    component::tool(
                        Icon::RotateLeft,
                        "Rotate left",
                        Some(Message::PageAction(PageAction::RotateLeft)),
                    ),
                    component::tool(
                        Icon::RotateRight,
                        "Rotate right",
                        Some(Message::PageAction(PageAction::RotateRight)),
                    ),
                    self.pages_menu(viewer),
                ]),
                TOOL_WIDTH * 3.0 + 8.0,
                Some(2),
            ),
            (
                component::toggle_tool(
                    Icon::EditDocument,
                    "Markup",
                    self.markup_bar,
                    Message::ToggleMarkupBar,
                ),
                TOOL_WIDTH,
                None,
            ),
            (search_bar, 280.0, Some(4)),
        ];
        let widths: Vec<(f32, Option<u8>)> = slots
            .iter()
            .map(|(_, width, order)| (*width, *order))
            .collect();
        let shown = component::fitting_slots(width, &widths);
        let mut bar = row![].spacing(8).align_y(Center);
        let mut hidden = Vec::new();
        for (index, ((element, _, _), shown)) in slots.into_iter().zip(shown).enumerate() {
            // Page tools, the markup button and search bar sit on the right.
            if index == 5 {
                bar = bar.push(space::horizontal());
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

    fn sidebar_view<'a>(&'a self, viewer: &'a PdfViewer, sidebar: Sidebar) -> Element<'a, Message> {
        let tab = |label: &'a str, glyph: Icon, which: Sidebar| component::Tab {
            label,
            icon: Some(glyph),
            selected: sidebar == which,
            on_press: Message::ShowSidebar(Some(which)),
        };
        let tabs = component::tabs(vec![
            tab("Pages", Icon::GridView, Sidebar::Thumbnails),
            tab("Contents", Icon::Toc, Sidebar::Contents),
            tab("Highlights and notes", Icon::StickyNote, Sidebar::Notes),
            tab("Bookmarks", Icon::Bookmarks, Sidebar::Bookmarks),
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
                let list =
                    mouse_area(container(list).padding([0, 16])).on_move(Message::ThumbnailsMoved);
                component::scroll(list)
                    .on_scroll(Message::ThumbnailsScrolled)
                    .id(self.thumbnails_id.clone())
                    .height(Fill)
                    .into()
            }
            Sidebar::Contents if !self.outline_loaded => {
                component::empty_state(Icon::Toc, "Loading…", "")
            }
            Sidebar::Contents if viewer.info.outline.is_empty() => component::empty_state(
                Icon::Toc,
                "No table of contents",
                "This document has no outline.",
            ),
            Sidebar::Notes => self.notes_view(viewer),
            Sidebar::Bookmarks if self.bookmarks.is_empty() => component::empty_state(
                Icon::Bookmarks,
                "No bookmarks",
                "Press Ctrl+D to bookmark a page.",
            ),
            Sidebar::Bookmarks => component::scroll(
                column(self.bookmarks.iter().map(|bookmark| {
                    let label = format!("{}  {}", viewer.page_label(bookmark.page), bookmark.title);
                    row![
                        component::list_row(
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
                            "Remove bookmark"
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
            component::list_row(
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
        || format!("Page {}", viewer.page_label(page)),
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
