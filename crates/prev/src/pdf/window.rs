//! One PDF document window: opening and unlocking, the toolbar, sidebar,
//! slideshow and printing around the page canvas.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::scrollable::{Direction, Scrollbar, Viewport};
use iced::widget::{
    Id, button, center, column, container, image, mouse_area, operation, pick_list, row, rule,
    scrollable, space, text, text_input,
};
use iced::{Center, Color, Element, Fill, Length, Task};
use prev_pdf::engine::{Engine, LinkTarget, OutlineItem};
use prev_pdf::mupdf_engine::MupdfEngine;
use prev_pdf::worker::{DocumentHandle, DocumentInfo, Opened, RenderPool, flatten};
use prev_store::bookmarks::{self, Bookmark, BookmarkStore};

use super::bench::{self, Bench, Step};
use super::canvas::PageCanvas;
use super::layout::{Fit, ViewMode};
use super::viewer::{PdfMessage, PdfViewer, Request, Zoom};
use crate::shortcuts::Action;

const SIDEBAR_WIDTH: f32 = 236.0;
const THUMBNAIL_WIDTH: f32 = 120.0;
const THUMBNAIL_SPACING: f32 = 28.0;
const LINE_SCROLL: f32 = 48.0;

fn engine() -> Arc<dyn Engine> {
    static ENGINE: OnceLock<Arc<dyn Engine>> = OnceLock::new();
    Arc::clone(ENGINE.get_or_init(|| Arc::new(MupdfEngine)))
}

fn render_pool() -> Arc<RenderPool> {
    static POOL: OnceLock<Arc<RenderPool>> = OnceLock::new();
    Arc::clone(POOL.get_or_init(|| RenderPool::new(RenderPool::default_threads())))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sidebar {
    Thumbnails,
    Contents,
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

impl std::fmt::Display for ModeChoice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self.0 {
            ViewMode::Continuous => "Continuous",
            ViewMode::SinglePage => "Single page",
            ViewMode::TwoPages => "Two pages",
        })
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
    canvas_id: Id,
    search_id: Id,
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
}

impl PdfWindow {
    pub fn open(path: PathBuf) -> (Self, Task<Message>) {
        let (handle, opened) = DocumentHandle::open(engine(), path.clone());
        let window = Self {
            path,
            state: State::Opening(handle),
            sidebar: None,
            canvas_id: Id::unique(),
            search_id: Id::unique(),
            page_input_id: Id::unique(),
            page_input: None,
            slideshow: None,
            device_scale: 1.0,
            bookmarks: Vec::new(),
            outline_loaded: false,
            bench: Bench::from_env(),
            notice: None,
            effects: Vec::new(),
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
        let task = viewer.update(message).map(Message::Viewer);
        let requests = viewer.take_requests();
        Task::batch(
            std::iter::once(task).chain(
                requests
                    .into_iter()
                    .map(|request| self.perform_request(request)),
            ),
        )
    }

    fn perform_request(&self, request: Request) -> Task<Message> {
        match request {
            Request::ScrollTo { x, y } => operation::scroll_to(
                self.canvas_id.clone(),
                scrollable::AbsoluteOffset {
                    x: Some(x),
                    y: Some(y),
                },
            ),
            Request::OpenUri(uri) => Task::perform(open_uri(uri), Message::OpenUriFinished),
        }
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
                self.thumbnails_for(0.0, 1000.0)
            }
            Message::ThumbnailsScrolled(viewport) => {
                let offset = viewport.absolute_offset().y;
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
                Task::perform(print(path, title), Message::PrintFinished)
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
        let sizes = viewer.info.page_sizes.clone();
        let mut y = 0.0;
        let mut pages = Vec::new();
        for (index, size) in sizes.iter().enumerate() {
            let item_height = thumbnail_height(size) + THUMBNAIL_SPACING;
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
            Action::Copy => viewer.copy_selection().map(Message::Viewer),
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
        let body: Element<'_, Message> = match &self.state {
            State::Opening(_) => center(text("Opening…")).into(),
            State::Failed(error) => center(
                column![
                    text("prev can't open this document.").size(18),
                    text(error).size(13)
                ]
                .spacing(8)
                .align_x(Center),
            )
            .into(),
            State::Locked {
                password, wrong, ..
            } => self.password_view(password, *wrong),
            State::Ready(viewer) if self.slideshow.is_some() => {
                self.canvas(viewer, Some(Color::BLACK))
            }
            State::Ready(viewer) => {
                let content: Element<'_, Message> = match self.sidebar {
                    Some(sidebar) => row![
                        container(self.sidebar_view(viewer, sidebar))
                            .width(SIDEBAR_WIDTH)
                            .height(Fill),
                        rule::vertical(1),
                        self.canvas(viewer, None),
                    ]
                    .into(),
                    None => self.canvas(viewer, None),
                };
                column![self.toolbar(viewer), rule::horizontal(1), content].into()
            }
        };
        match &self.notice {
            Some(notice) => column![body, container(text(notice).size(13)).padding(6)].into(),
            None => body,
        }
    }

    fn password_view<'a>(&'a self, password: &'a str, wrong: bool) -> Element<'a, Message> {
        let name = self
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut content = column![
            text(format!("“{name}” is password protected.")).size(18),
            text_input("Password", password)
                .secure(true)
                .on_input(Message::PasswordChanged)
                .on_submit(Message::SubmitPassword)
                .width(280),
            button("Unlock").on_press(Message::SubmitPassword),
        ]
        .spacing(12)
        .align_x(Center);
        if wrong {
            content = content.push(text("Incorrect password. Try again.").size(13));
        }
        center(content).into()
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
            Scrollbar::default()
        };
        scrollable(canvas)
            .id(self.canvas_id.clone())
            .direction(Direction::Both {
                vertical: scrollbar,
                horizontal: scrollbar,
            })
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn toolbar<'a>(&'a self, viewer: &'a PdfViewer) -> Element<'a, Message> {
        let sidebar_button =
            button(text("Sidebar").size(13)).on_press(Message::ShowSidebar(match self.sidebar {
                Some(_) => None,
                None => Some(Sidebar::Thumbnails),
            }));
        let page_label = viewer.page_label(viewer.current);
        let page_box = text_input("", self.page_input.as_deref().unwrap_or(&page_label))
            .id(self.page_input_id.clone())
            .on_input(Message::PageInputChanged)
            .on_submit(Message::PageInputSubmitted)
            .size(13)
            .width(56);
        let pages = text(format!("of {}", viewer.page_count())).size(13);
        let zoom = |label: &'static str, zoom: Zoom| {
            button(text(label).size(13)).on_press(Message::Viewer(PdfMessage::Zoom(zoom)))
        };
        let percent = text(format!("{:.0}%", viewer.layout.zoom * 100.0))
            .size(13)
            .width(48)
            .align_x(Center);
        let mode =
            pick_list(MODES, Some(ModeChoice(viewer.mode)), Message::ModeSelected).text_size(13);

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
        let search_box = text_input("Search", &search.query)
            .id(self.search_id.clone())
            .on_input(|query| Message::Viewer(PdfMessage::SearchChanged(query)))
            .on_submit(Message::Viewer(PdfMessage::NextMatch))
            .size(13)
            .width(180);

        row![
            sidebar_button,
            page_box,
            pages,
            space::horizontal().width(8),
            zoom("−", Zoom::Out),
            percent,
            zoom("+", Zoom::In),
            zoom("Fit", Zoom::FitPage),
            zoom("Width", Zoom::FitWidth),
            mode,
            space::horizontal(),
            text(matches).size(12),
            button(text("‹").size(13)).on_press(Message::Viewer(PdfMessage::PreviousMatch)),
            button(text("›").size(13)).on_press(Message::Viewer(PdfMessage::NextMatch)),
            search_box,
        ]
        .spacing(6)
        .padding(6)
        .align_y(Center)
        .into()
    }

    fn sidebar_view<'a>(&'a self, viewer: &'a PdfViewer, sidebar: Sidebar) -> Element<'a, Message> {
        let tabs = row![
            tab_button(
                "Thumbnails",
                sidebar == Sidebar::Thumbnails,
                Sidebar::Thumbnails
            ),
            tab_button("Contents", sidebar == Sidebar::Contents, Sidebar::Contents),
            tab_button(
                "Bookmarks",
                sidebar == Sidebar::Bookmarks,
                Sidebar::Bookmarks
            ),
        ]
        .spacing(4)
        .padding(6);
        let list: Element<'a, Message> = match sidebar {
            Sidebar::Thumbnails => scrollable(
                column(
                    viewer
                        .info
                        .page_sizes
                        .iter()
                        .enumerate()
                        .map(|(page, size)| thumbnail(viewer, page, *size)),
                )
                .spacing(0)
                .width(Fill)
                .align_x(Center),
            )
            .on_scroll(Message::ThumbnailsScrolled)
            .height(Fill)
            .into(),
            Sidebar::Contents if !self.outline_loaded => center(text("Loading…").size(13)).into(),
            Sidebar::Contents if viewer.info.outline.is_empty() => {
                center(text("No table of contents").size(13)).into()
            }
            Sidebar::Bookmarks if self.bookmarks.is_empty() => center(
                text("No bookmarks. Press Ctrl+D to bookmark a page.")
                    .size(13)
                    .align_x(Center),
            )
            .padding(8)
            .into(),
            Sidebar::Bookmarks => scrollable(
                column(self.bookmarks.iter().map(|bookmark| {
                    let label = format!("{}  {}", viewer.page_label(bookmark.page), bookmark.title);
                    row![
                        button(text(label).size(13))
                            .style(button::text)
                            .padding([2, 4])
                            .width(Fill)
                            .on_press(Message::Viewer(PdfMessage::GoTo {
                                page: bookmark.page,
                                point: None
                            })),
                        button(text("×").size(13))
                            .style(button::text)
                            .on_press(Message::ToggleBookmark(bookmark.page)),
                    ]
                    .align_y(Center)
                    .into()
                }))
                .spacing(2)
                .padding(6)
                .width(Fill),
            )
            .height(Fill)
            .into(),
            Sidebar::Contents => {
                let mut entries = Vec::new();
                outline_entries(&viewer.info.outline, 0, &mut entries);
                scrollable(column(entries).spacing(2).padding(6).width(Fill))
                    .height(Fill)
                    .into()
            }
        };
        column![tabs, list].into()
    }
}

fn tab_button(label: &str, selected: bool, sidebar: Sidebar) -> Element<'_, Message> {
    let style = if selected {
        button::primary
    } else {
        button::text
    };
    button(text(label).size(12))
        .style(style)
        .on_press(Message::ShowSidebar(Some(sidebar)))
        .into()
}

fn thumbnail_height(size: &prev_pdf::geometry::Size) -> f32 {
    THUMBNAIL_WIDTH * size.height / size.width.max(1.0)
}

fn thumbnail<'a>(
    viewer: &'a PdfViewer,
    page: usize,
    size: prev_pdf::geometry::Size,
) -> Element<'a, Message> {
    let height = thumbnail_height(&size);
    let picture: Element<'a, Message> = match viewer.previews.get(&page) {
        Some(handle) => image(handle.clone())
            .width(THUMBNAIL_WIDTH)
            .height(height)
            .into(),
        None => container(space::horizontal())
            .width(THUMBNAIL_WIDTH)
            .height(height)
            .style(|_| container::Style::default().background(Color::WHITE))
            .into(),
    };
    let selected = page == viewer.current;
    let framed = container(picture)
        .padding(2)
        .style(move |theme: &iced::Theme| {
            let color = if selected {
                theme.palette().primary
            } else {
                Color::TRANSPARENT
            };
            container::Style::default().border(iced::Border {
                color,
                width: 2.0,
                radius: 2.0.into(),
            })
        });
    mouse_area(
        column![framed, text(viewer.page_label(page)).size(11)]
            .align_x(Center)
            .spacing(2)
            .height(Length::Fixed(height + THUMBNAIL_SPACING)),
    )
    .on_press(Message::Viewer(PdfMessage::GoTo { page, point: None }))
    .into()
}

fn outline_entries<'a>(
    items: &'a [OutlineItem],
    depth: usize,
    entries: &mut Vec<Element<'a, Message>>,
) {
    for item in items {
        let label = text(&item.title).size(13);
        let entry = match &item.target {
            Some(LinkTarget::Page { index, point }) => button(label)
                .style(button::text)
                .padding([2, 4])
                .on_press(Message::Viewer(PdfMessage::GoTo {
                    page: *index,
                    point: *point,
                })),
            Some(LinkTarget::Uri(_)) | None => button(label).style(button::text).padding([2, 4]),
        };
        entries.push(row![space::horizontal().width(depth as f32 * 12.0), entry].into());
        outline_entries(&item.children, depth + 1, entries);
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

async fn open_uri(uri: String) -> Result<(), String> {
    let parsed = ashpd::Uri::parse(&uri).map_err(|error| format!("Invalid link {uri}: {error}"))?;
    ashpd::desktop::open_uri::OpenFileRequest::default()
        .send_uri(&parsed)
        .await
        .map(|_| ())
        .map_err(|error| format!("Could not open {uri}: {error}"))
}

async fn print(path: PathBuf, title: String) -> Result<(), String> {
    use ashpd::desktop::print::{PreparePrintOptions, PrintOptions, PrintProxy};
    use std::os::fd::AsFd;

    let failed = |error: ashpd::Error| format!("Could not print: {error}");
    let proxy = PrintProxy::new().await.map_err(failed)?;
    let prepared = proxy
        .prepare_print(
            None,
            &title,
            Default::default(),
            Default::default(),
            PreparePrintOptions::default(),
        )
        .await
        .map_err(failed)?;
    let prepared = match prepared.response() {
        Ok(prepared) => prepared,
        Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => return Ok(()),
        Err(error) => return Err(failed(error)),
    };
    let file = std::fs::File::open(&path).map_err(|error| format!("Could not print: {error}"))?;
    proxy
        .print(
            None,
            &title,
            &file.as_fd(),
            PrintOptions::default().set_token(prepared.token),
        )
        .await
        .map_err(failed)?;
    Ok(())
}
