//! State of one PDF window: layout, caches, selection and search, and the
//! scheduling that keeps visible tiles rendered.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

use iced::Task;
use iced::widget::image::Handle;
use prev_pdf::engine::{Bitmap, Link, LinkTarget, PageDisplay};
use prev_pdf::geometry::{PixelRect, Point, Quad, Rect, Size};
use prev_pdf::text::{Selection, TextLayout};
use prev_pdf::worker::{DocumentHandle, DocumentInfo, RenderPool, SearchEvent, Ticket};

use super::layout::{self, Area, Fit, Layout, ViewMode};

pub use pages::Pick;
use prev_pdf::worker::{Edited, PageMarkup, Restructured};

pub mod editing;
pub mod pages;
pub use editing::{Editing, FieldEdit, TextEdit};

/// Device pixels across a page preview, used until sharp tiles arrive and
/// for sidebar thumbnails.
const PREVIEW_WIDTH: f32 = 320.0;
const TILE_BUDGET_BYTES: usize = 128 * 1024 * 1024;
const ZOOM_STEP: f32 = 1.25;
/// Logical pixels inside the view's edge where a drag starts scrolling.
const EDGE_ZONE: f32 = 6.0;
/// Pixels scrolled per step for each pixel the pointer is into that zone
/// or past the edge, up to `EDGE_STEP_MAX`.
const EDGE_SPEED: f32 = 0.35;
const EDGE_STEP_MAX: f32 = 24.0;
/// Time between steps of scrolling at the edge: about one frame.
const EDGE_TICK: Duration = Duration::from_millis(16);
const LATENCY_SAMPLES: usize = 4096;

#[derive(Clone)]
pub struct SharedDisplay(pub Arc<dyn PageDisplay>);

impl std::fmt::Debug for SharedDisplay {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("PageDisplay")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileKey {
    pub page: usize,
    /// Bumped each time the page changes, so older tiles can stand in
    /// until the new ones arrive.
    pub generation: u32,
    /// Device pixels per point, times 1000.
    pub scale: u32,
    pub x: i32,
    pub y: i32,
}

fn scale_key(scale: f32) -> u32 {
    (scale * 1000.0).round() as u32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Granularity {
    Character,
    Word,
    Line,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextSelection {
    pub anchor: (usize, Point),
    pub focus: (usize, Point),
    pub granularity: Granularity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Zoom {
    In,
    Out,
    ActualSize,
    FitWidth,
    FitPage,
    /// Multiply by a factor around a point in the view (logical pixels).
    By {
        factor: f32,
        anchor: (f32, f32),
    },
    /// A zoom, where 1.0 is actual size, around the middle of the view.
    To(f32),
}

#[derive(Debug, Clone)]
pub enum PdfMessage {
    /// The visible part of the document, in document space.
    ViewChanged(Area),
    DeviceScale(f32),
    DisplayReady(usize, Option<SharedDisplay>),
    PreviewReady(usize, Option<Handle>),
    TileReady(TileKey, Option<(Handle, usize)>),
    TextReady(usize, Option<Arc<TextLayout>>),
    LinksReady(usize, Vec<Link>),
    Zoom(Zoom),
    /// Scroll the view by this much, for a drag that pans.
    Pan {
        dx: f32,
        dy: f32,
    },
    SetMode(ViewMode),
    GoTo {
        page: usize,
        point: Option<Point>,
    },
    NextPage,
    PreviousPage,
    /// Shows `point` of `page`, in page points, in the middle of the view,
    /// at `fit` if given.
    Show {
        page: usize,
        point: Point,
        fit: Option<Fit>,
    },
    /// Outlines `rect` of `page` for `seconds`, showing it if it is out
    /// of view; the number tells this outline from a later one.
    PointAt {
        page: usize,
        rect: Rect,
        id: u64,
        seconds: f32,
    },
    PointDone(u64),
    /// Scroll by logical pixels.
    ScrollBy {
        dx: f32,
        dy: f32,
    },
    /// Mouse press in document space, with the click count (1 to 3).
    Press {
        x: f32,
        y: f32,
        clicks: u8,
    },
    Drag {
        x: f32,
        y: f32,
    },
    Release {
        x: f32,
        y: f32,
    },
    /// Shift held during a press or drag.
    Shift(bool),
    /// The command key (Ctrl, ⌘ on macOS) is held, so a drag pans.
    Command(bool),
    /// A pan by dragging started or ended.
    Panning(bool),
    MarkupReady(usize, Option<PageMarkup>),
    Edited(editing::Sent, Result<Edited, String>),
    Editing(editing::EditMessage),
    SearchChanged(String),
    SearchEvent(u64, SearchEvent),
    NextMatch,
    PreviousMatch,
    /// A result about pages, from before or after the page numbers last
    /// changed; stale ones are dropped.
    Fresh(u64, Box<PdfMessage>),
    /// A step of scrolling while an annotation is dragged at the view's edge.
    EdgeScroll,
    Restructured(pages::PagesSent, Result<Restructured, String>),
}

/// Something the app must do on the viewer's behalf.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    ScrollTo {
        x: f32,
        y: f32,
    },
    OpenUri(String),
    /// The document changed; autosave it.
    Changed,
    /// Pages were added, removed, moved or changed: thumbnails, outline
    /// and notes need loading again.
    PagesChanged,
    Notice(String),
    /// Selected text or an area is being dragged out of the document.
    DragOut(editing::Outgoing),
}

#[derive(Debug, Default)]
pub struct SearchState {
    pub query: String,
    generation: u64,
    ticket: Option<Ticket>,
    /// Matches in document order.
    pub matches: Vec<(usize, Quad)>,
    pub current: Option<usize>,
    pub searched: usize,
    pub finished: bool,
}

pub struct PdfViewer {
    pub handle: DocumentHandle,
    pub info: DocumentInfo,
    pool: Arc<RenderPool>,
    pub mode: ViewMode,
    pub fit: Fit,
    pub layout: Layout,
    pub view: Area,
    pub device_scale: f32,
    /// The page the view is anchored to, for single page modes and fits.
    pub current: usize,
    displays: HashMap<usize, Arc<dyn PageDisplay>>,
    displays_requested: HashSet<usize>,
    pub previews: HashMap<usize, Handle>,
    previews_requested: HashSet<usize>,
    /// Sidebar thumbnails waiting for their page to load.
    thumbnails_waiting: HashSet<usize>,
    tiles: HashMap<TileKey, (Handle, usize, u64)>,
    tile_bytes: usize,
    tiles_in_flight: HashMap<TileKey, (Ticket, Instant)>,
    /// Request to arrival time of recent tiles, for benchmarks.
    pub tile_latencies: VecDeque<Duration>,
    use_clock: u64,
    pub texts: HashMap<usize, Arc<TextLayout>>,
    texts_requested: HashSet<usize>,
    pub links: HashMap<usize, Vec<Link>>,
    links_requested: HashSet<usize>,
    pub selection: Option<TextSelection>,
    press: Option<(f32, f32)>,
    dragged: bool,
    pending_copy: bool,
    /// A zoom and a point of the first page, as fractions of its size, to
    /// centre once the view's real size is known.
    pub pending_view: Option<(f32, (f32, f32))>,
    /// Where an annotation drag last had the pointer, in document space,
    /// for scrolling while it is at the view's edge.
    edge: Option<(f32, f32)>,
    /// A step of that scrolling is on its way.
    edge_ticking: bool,
    pub search: SearchState,
    requests: Vec<Request>,
    pub markup: HashMap<usize, PageMarkup>,
    markup_requested: HashSet<usize>,
    generations: HashMap<usize, u32>,
    pub edit: Editing,
    shift: bool,
    /// Kept here rather than in the canvas, whose state is made anew when
    /// the layout around it changes, as a redraw can.
    command: bool,
    panning: bool,
    /// Bumped whenever page numbers change.
    epoch: u64,
    /// Pages chosen in the sidebar, for page edits.
    pub selected_pages: std::collections::BTreeSet<usize>,
    /// An area outlined for a moment, as an agent points at it: its page,
    /// rectangle in page points and number.
    pub pointed: Option<(usize, Rect, u64)>,
}

impl PdfViewer {
    pub fn new(handle: DocumentHandle, info: DocumentInfo, pool: Arc<RenderPool>) -> Self {
        let view = Area {
            x: 0.0,
            y: 0.0,
            width: 900.0,
            height: 700.0,
        };
        let fit = Fit::Width;
        let zoom = layout::resolve_zoom(
            fit,
            ViewMode::Continuous,
            &info.page_sizes,
            0,
            size_of(&view),
        );
        let layout = layout::layout(
            ViewMode::Continuous,
            &info.page_sizes,
            0,
            zoom,
            size_of(&view),
        );
        Self {
            handle,
            info,
            pool,
            mode: ViewMode::Continuous,
            fit,
            layout,
            view,
            device_scale: 1.0,
            current: 0,
            displays: HashMap::new(),
            displays_requested: HashSet::new(),
            previews: HashMap::new(),
            previews_requested: HashSet::new(),
            thumbnails_waiting: HashSet::new(),
            tiles: HashMap::new(),
            tile_bytes: 0,
            tiles_in_flight: HashMap::new(),
            tile_latencies: VecDeque::new(),
            use_clock: 0,
            texts: HashMap::new(),
            texts_requested: HashSet::new(),
            links: HashMap::new(),
            links_requested: HashSet::new(),
            selection: None,
            press: None,
            dragged: false,
            pending_copy: false,
            pending_view: None,
            edge: None,
            edge_ticking: false,
            search: SearchState::default(),
            requests: Vec::new(),
            markup: HashMap::new(),
            markup_requested: HashSet::new(),
            generations: HashMap::new(),
            edit: Editing::default(),
            shift: false,
            command: false,
            panning: false,
            epoch: 0,
            selected_pages: std::collections::BTreeSet::new(),
            pointed: None,
        }
    }

    /// Runs `future` and makes its result a message, dropped if the page
    /// numbers change before it arrives.
    fn current<T: Send + 'static>(
        &self,
        future: impl std::future::Future<Output = T> + Send + 'static,
        message: impl FnOnce(T) -> PdfMessage + Send + 'static,
    ) -> Task<PdfMessage> {
        let epoch = self.epoch;
        Task::perform(future, move |result| {
            PdfMessage::Fresh(epoch, Box::new(message(result)))
        })
    }

    pub fn shift(&self) -> bool {
        self.shift
    }

    pub fn command(&self) -> bool {
        self.command
    }

    pub fn panning(&self) -> bool {
        self.panning
    }

    pub fn page_count(&self) -> usize {
        self.info.page_sizes.len()
    }

    /// Requests collected since the last call.
    pub fn take_requests(&mut self) -> Vec<Request> {
        std::mem::take(&mut self.requests)
    }

    /// Device pixels per point for the current zoom.
    pub fn render_scale(&self) -> f32 {
        layout::points_to_pixels(self.layout.zoom) * self.device_scale
    }

    /// The tile, or the one from before the page last changed while the
    /// new one renders.
    pub fn tile(&self, key: &TileKey) -> Option<&Handle> {
        self.tiles
            .get(key)
            .or_else(|| {
                let previous = TileKey {
                    generation: key.generation.checked_sub(1)?,
                    ..*key
                };
                self.tiles.get(&previous)
            })
            .map(|(handle, _, _)| handle)
    }

    pub fn tile_key(&self, page: usize, tile: &PixelRect) -> TileKey {
        TileKey {
            page,
            generation: self.generation(page),
            scale: scale_key(self.render_scale()),
            x: tile.x,
            y: tile.y,
        }
    }

    /// Whether every tile the view needs of `page` is drawn at its current
    /// generation.
    pub fn page_tiles_ready(&self, page: usize) -> bool {
        let Some(area) = self.layout.page_area(page) else {
            return true;
        };
        layout::visible_tiles(&area, &self.view, self.device_scale)
            .iter()
            .all(|tile| self.tiles.contains_key(&self.tile_key(page, tile)))
    }

    pub fn generation(&self, page: usize) -> u32 {
        self.generations.get(&page).copied().unwrap_or(0)
    }

    pub fn page_label(&self, page: usize) -> String {
        self.info
            .page_labels
            .get(page)
            .cloned()
            .flatten()
            .unwrap_or_else(|| (page + 1).to_string())
    }

    fn relayout(&mut self) {
        let viewport = size_of(&self.view);
        let zoom = layout::resolve_zoom(
            self.fit,
            self.mode,
            &self.info.page_sizes,
            self.current,
            viewport,
        );
        self.layout = layout::layout(
            self.mode,
            &self.info.page_sizes,
            self.current,
            zoom,
            viewport,
        );
    }

    pub fn update(&mut self, message: PdfMessage) -> Task<PdfMessage> {
        match message {
            PdfMessage::ViewChanged(view) => {
                let resized = (view.width, view.height) != (self.view.width, self.view.height);
                self.view = view;
                if let Some((zoom, fraction)) = self.pending_view.take() {
                    self.center_on(zoom, fraction);
                    return self.schedule();
                }
                if self.mode == ViewMode::Continuous
                    && let Some(page) = self.layout.current_page(&self.view)
                {
                    self.current = page;
                }
                if resized && !matches!(self.fit, Fit::Zoom(_)) {
                    self.keep_position(|viewer| viewer.relayout());
                }
                self.schedule()
            }
            PdfMessage::DeviceScale(scale) => {
                self.device_scale = scale;
                self.schedule()
            }
            PdfMessage::DisplayReady(page, display) => {
                let Some(SharedDisplay(display)) = display else {
                    self.displays_requested.remove(&page);
                    return Task::none();
                };
                self.displays.insert(page, Arc::clone(&display));
                let copy_text = if self.pending_copy {
                    self.request_text(page, &display)
                } else {
                    Task::none()
                };
                let thumbnail = if self.thumbnails_waiting.remove(&page) {
                    self.request_preview(page, &display, 12)
                } else {
                    Task::none()
                };
                Task::batch([copy_text, thumbnail, self.schedule()])
            }
            PdfMessage::PreviewReady(page, handle) => {
                if let Some(handle) = handle {
                    self.previews.insert(page, handle);
                } else {
                    self.previews_requested.remove(&page);
                }
                Task::none()
            }
            PdfMessage::TileReady(key, result) => {
                if let Some((_, requested)) = self.tiles_in_flight.remove(&key) {
                    if self.tile_latencies.len() == LATENCY_SAMPLES {
                        self.tile_latencies.pop_front();
                    }
                    self.tile_latencies.push_back(requested.elapsed());
                }
                if let Some((handle, bytes)) = result {
                    // The page's previous look is no longer needed here.
                    if let Some(previous) = key.generation.checked_sub(1)
                        && let Some((_, old_bytes, _)) = self.tiles.remove(&TileKey {
                            generation: previous,
                            ..key
                        })
                    {
                        self.tile_bytes -= old_bytes;
                    }
                    self.use_clock += 1;
                    self.tile_bytes += bytes;
                    self.tiles.insert(key, (handle, bytes, self.use_clock));
                    self.evict_tiles();
                }
                self.settle_lift();
                Task::none()
            }
            PdfMessage::TextReady(page, text) => {
                match text {
                    Some(text) => {
                        self.texts.insert(page, text);
                    }
                    None => {
                        self.texts_requested.remove(&page);
                    }
                }
                if self.pending_copy {
                    return self.copy_selection();
                }
                Task::none()
            }
            PdfMessage::LinksReady(page, links) => {
                self.links.insert(page, links);
                Task::none()
            }
            PdfMessage::Zoom(zoom) => self.zoom(zoom),
            // The window scrolls the view for it.
            PdfMessage::Pan { .. } => Task::none(),
            PdfMessage::SetMode(mode) => {
                self.mode = mode;
                self.keep_position(|viewer| viewer.relayout());
                self.schedule()
            }
            PdfMessage::GoTo { page, point } => self.go_to(page, point),
            PdfMessage::Show { page, point, fit } => self.show(page, point, fit),
            PdfMessage::PointAt {
                page,
                rect,
                id,
                seconds,
            } => {
                self.pointed = Some((page, rect, id));
                self.reveal(page, rect);
                let shown = Task::perform(
                    crate::image::editor::spawn(move || {
                        std::thread::sleep(Duration::from_secs_f32(seconds))
                    }),
                    move |_| PdfMessage::PointDone(id),
                );
                Task::batch([self.schedule(), shown])
            }
            PdfMessage::PointDone(id) => {
                if self.pointed.is_some_and(|(_, _, pointed)| pointed == id) {
                    self.pointed = None;
                }
                Task::none()
            }
            PdfMessage::NextPage => self.step_page(1),
            PdfMessage::PreviousPage => self.step_page(-1),
            PdfMessage::ScrollBy { dx, dy } => {
                self.scroll_to(self.view.x + dx, self.view.y + dy);
                self.schedule()
            }
            PdfMessage::Press { x, y, clicks } => self.press(x, y, clicks),
            PdfMessage::Drag { x, y } => {
                self.drag(x, y);
                self.edge = self.edge_scrolls().then_some((x, y));
                Task::batch([self.request_draft(), self.start_edge_scroll()])
            }
            PdfMessage::EdgeScroll => {
                self.edge_ticking = false;
                self.edge_scroll()
            }
            PdfMessage::Release { x, y } => {
                self.edge = None;
                self.release(x, y)
            }
            PdfMessage::Shift(shift) => {
                self.shift = shift;
                self.request_draft()
            }
            PdfMessage::Command(command) => {
                self.command = command;
                Task::none()
            }
            PdfMessage::Panning(panning) => {
                self.panning = panning;
                Task::none()
            }
            PdfMessage::MarkupReady(page, markup) => {
                match markup {
                    Some(markup) => {
                        self.markup.insert(page, markup);
                    }
                    None => {
                        self.markup_requested.remove(&page);
                    }
                }
                Task::none()
            }
            PdfMessage::Edited(sent, result) => self.edited(sent, result),
            PdfMessage::Editing(message) => self.editing(message),
            PdfMessage::SearchChanged(query) => self.start_search(query),
            PdfMessage::SearchEvent(generation, event) => {
                self.search_event(generation, event);
                Task::none()
            }
            PdfMessage::NextMatch => self.step_match(1),
            PdfMessage::PreviousMatch => self.step_match(-1),
            PdfMessage::Fresh(epoch, message) if epoch == self.epoch => self.update(*message),
            PdfMessage::Fresh(..) => Task::none(),
            PdfMessage::Restructured(sent, result) => self.restructured(sent, result),
        }
    }

    /// Runs a relayout while keeping the point at the top-left of the view
    /// (or the page start in single page modes) in place.
    fn keep_position(&mut self, change: impl FnOnce(&mut Self)) {
        let anchor = self.layout.hit_nearest(self.view.x, self.view.y);
        change(self);
        if let Some((page, point)) = anchor {
            // Continuous and two-page views scroll through every page, so
            // they keep the point; a single page shows from its top.
            if self.mode != ViewMode::SinglePage {
                if let Some((x, y)) = self.layout.to_document(page, point) {
                    self.scroll_to(x, y);
                }
            } else {
                self.scroll_to(0.0, 0.0);
            }
        }
    }

    /// Zooms to `zoom` with the point of the first page at `fraction` of
    /// its size in the middle of the view.
    fn center_on(&mut self, zoom: f32, fraction: (f32, f32)) {
        let Some(size) = self.info.page_sizes.first().copied() else {
            return;
        };
        self.fit = Fit::Zoom(zoom);
        self.relayout();
        let point = Point::new(fraction.0 * size.width, fraction.1 * size.height);
        if let Some((x, y)) = self.layout.to_document(0, point) {
            self.scroll_to(x - self.view.width / 2.0, y - self.view.height / 2.0);
        }
    }

    /// The zoom, and the point of the first page in the middle of the view
    /// as fractions of its size, when the zoom is not a fit.
    pub fn zoomed_center(&self) -> Option<(f32, (f32, f32))> {
        if !matches!(self.fit, Fit::Zoom(_)) {
            return None;
        }
        let size = self.info.page_sizes.first()?;
        let (page, point) = self.layout.hit_nearest(
            self.view.x + self.view.width / 2.0,
            self.view.y + self.view.height / 2.0,
        )?;
        (page == 0).then(|| {
            (
                self.layout.zoom,
                (
                    point.x / size.width.max(1.0),
                    point.y / size.height.max(1.0),
                ),
            )
        })
    }

    /// How far to scroll for one step while a drag has the pointer at `x`,
    /// `y`: nothing inside the view, more the farther past its edge.
    fn edge_velocity(&self, x: f32, y: f32) -> (f32, f32) {
        let axis = |position: f32, start: f32, length: f32| {
            let (low, high) = (start + EDGE_ZONE, start + length - EDGE_ZONE);
            if position < low {
                -((low - position) * EDGE_SPEED).min(EDGE_STEP_MAX)
            } else if position > high {
                ((position - high) * EDGE_SPEED).min(EDGE_STEP_MAX)
            } else {
                0.0
            }
        };
        (
            axis(x, self.view.x, self.view.width),
            axis(y, self.view.y, self.view.height),
        )
    }

    /// Starts scrolling when a drag has the pointer at the view's edge.
    fn start_edge_scroll(&mut self) -> Task<PdfMessage> {
        let Some((x, y)) = self.edge else {
            return Task::none();
        };
        if self.edge_ticking || self.edge_velocity(x, y) == (0.0, 0.0) {
            return Task::none();
        }
        self.edge_ticking = true;
        Task::perform(
            crate::image::editor::spawn(|| std::thread::sleep(EDGE_TICK)),
            |_| PdfMessage::EdgeScroll,
        )
    }

    /// One step of scrolling at the edge: the view moves, and the drag
    /// with it, as if the pointer had moved over the page by as much.
    fn edge_scroll(&mut self) -> Task<PdfMessage> {
        let Some((x, y)) = self.edge.filter(|_| self.edge_scrolls()) else {
            self.edge = None;
            return Task::none();
        };
        let (dx, dy) = self.edge_velocity(x, y);
        let before = (self.view.x, self.view.y);
        self.scroll_to(before.0 + dx, before.1 + dy);
        let (moved_x, moved_y) = (self.view.x - before.0, self.view.y - before.1);
        if moved_x == 0.0 && moved_y == 0.0 {
            return Task::none();
        }
        let point = (x + moved_x, y + moved_y);
        self.edge = Some(point);
        self.drag(point.0, point.1);
        Task::batch([
            self.schedule(),
            self.request_draft(),
            self.start_edge_scroll(),
        ])
    }

    fn scroll_to(&mut self, x: f32, y: f32) {
        let x = x.clamp(0.0, (self.layout.content.width - self.view.width).max(0.0));
        let y = y.clamp(
            0.0,
            (self.layout.content.height - self.view.height).max(0.0),
        );
        self.view.x = x;
        self.view.y = y;
        self.requests.push(Request::ScrollTo { x, y });
    }

    fn zoom(&mut self, zoom: Zoom) -> Task<PdfMessage> {
        let center = (self.view.width / 2.0, self.view.height / 2.0);
        let (fit, anchor) = match zoom {
            Zoom::In => (Fit::Zoom(self.layout.zoom * ZOOM_STEP), center),
            Zoom::Out => (Fit::Zoom(self.layout.zoom / ZOOM_STEP), center),
            Zoom::ActualSize => (Fit::Zoom(1.0), center),
            Zoom::FitWidth => (Fit::Width, center),
            Zoom::FitPage => (Fit::Page, center),
            Zoom::By { factor, anchor } => (Fit::Zoom(self.layout.zoom * factor), anchor),
            Zoom::To(zoom) => (Fit::Zoom(zoom), center),
        };
        let before = self
            .layout
            .hit_nearest(self.view.x + anchor.0, self.view.y + anchor.1);
        self.fit = fit;
        self.relayout();
        if let Some((page, point)) = before
            && let Some((x, y)) = self.layout.to_document(page, point)
        {
            self.scroll_to(x - anchor.0, y - anchor.1);
        }
        self.schedule()
    }

    fn go_to(&mut self, page: usize, point: Option<Point>) -> Task<PdfMessage> {
        if page >= self.page_count() {
            return Task::none();
        }
        self.current = page;
        if self.mode != ViewMode::Continuous {
            self.relayout();
        }
        let point = point.unwrap_or_default();
        if let Some((x, y)) = self.layout.to_document(page, point) {
            let x = if point.x > 0.0 {
                x - layout::MARGIN
            } else {
                self.view.x
            };
            self.scroll_to(x, y - layout::MARGIN);
        }
        self.schedule()
    }

    fn show(&mut self, page: usize, point: Point, fit: Option<Fit>) -> Task<PdfMessage> {
        if page >= self.page_count() {
            return Task::none();
        }
        self.current = page;
        if let Some(fit) = fit {
            self.fit = fit;
        }
        self.relayout();
        if let Some((x, y)) = self.layout.to_document(page, point) {
            self.scroll_to(x - self.view.width / 2.0, y - self.view.height / 2.0);
        }
        self.schedule()
    }

    /// Scrolls `rect` of `page` into view when it is not.
    fn reveal(&mut self, page: usize, rect: Rect) {
        if self.mode != ViewMode::Continuous && self.current != page {
            self.current = page;
            self.relayout();
        }
        let corner = |x, y| self.layout.to_document(page, Point::new(x, y));
        let (Some(top_left), Some(bottom_right)) =
            (corner(rect.x0, rect.y0), corner(rect.x1, rect.y1))
        else {
            return;
        };
        let visible = self.view.contains(top_left.0, top_left.1)
            && self.view.contains(bottom_right.0, bottom_right.1);
        if !visible {
            let middle = (
                (top_left.0 + bottom_right.0) / 2.0,
                (top_left.1 + bottom_right.1) / 2.0,
            );
            self.scroll_to(
                middle.0 - self.view.width / 2.0,
                middle.1 - self.view.height / 2.0,
            );
        }
    }

    fn step_page(&mut self, step: i64) -> Task<PdfMessage> {
        // Two pages steps a pair at a time, from its left page.
        let current = self.current as i64;
        let target = match self.mode {
            ViewMode::TwoPages => current - current % 2 + step * 2,
            _ => current + step,
        };
        let target = target.clamp(0, self.page_count() as i64 - 1) as usize;
        self.go_to(target, None)
    }

    /// Renders what the view needs, cancels what it no longer needs.
    pub fn schedule(&mut self) -> Task<PdfMessage> {
        let mut tasks = Vec::new();
        let visible = self.layout.visible_pages(&self.view);
        let mut wanted_pages: Vec<usize> = visible.clone();
        // Prefetch the pages around the visible ones.
        if let (Some(&first), Some(&last)) = (visible.iter().min(), visible.iter().max()) {
            wanted_pages.extend(first.checked_sub(1));
            wanted_pages.extend((last + 1 < self.page_count()).then_some(last + 1));
        }

        for &page in &wanted_pages {
            tasks.push(self.ensure_display(page));
            tasks.push(self.ensure_markup(page));
            if !self.links.contains_key(&page) && self.links_requested.insert(page) {
                let receiver = self.handle.links(page);
                tasks.push(self.current(receiver, move |result| {
                    PdfMessage::LinksReady(
                        page,
                        result.ok().and_then(Result::ok).unwrap_or_default(),
                    )
                }));
            }
        }

        let scale = self.render_scale();
        let mut wanted_tiles = HashSet::new();
        for (order, &page) in visible.iter().enumerate() {
            let Some(display) = self.displays.get(&page).cloned() else {
                continue;
            };
            tasks.push(self.request_preview(page, &display, order as u32));
            tasks.push(self.request_text(page, &display));
            let Some(area) = self.layout.page_area(page) else {
                continue;
            };
            for tile in layout::visible_tiles(&area, &self.view, self.device_scale) {
                let key = self.tile_key(page, &tile);
                wanted_tiles.insert(key);
                if let Some(entry) = self.tiles.get_mut(&key) {
                    self.use_clock += 1;
                    entry.2 = self.use_clock;
                    continue;
                }
                if self.tiles_in_flight.contains_key(&key) {
                    continue;
                }
                let ticket = Ticket::new();
                self.tiles_in_flight
                    .insert(key, (ticket.clone(), Instant::now()));
                let receiver =
                    self.pool
                        .render(Arc::clone(&display), scale, tile, 16 + order as u32, ticket);
                tasks.push(self.current(receiver, move |result| {
                    PdfMessage::TileReady(key, result.ok().and_then(Result::ok).map(to_handle))
                }));
            }
        }
        self.tiles_in_flight.retain(|key, (ticket, _)| {
            let keep = wanted_tiles.contains(key);
            if !keep {
                ticket.cancel();
            }
            keep
        });
        Task::batch(tasks)
    }

    fn request_preview(
        &mut self,
        page: usize,
        display: &Arc<dyn PageDisplay>,
        priority: u32,
    ) -> Task<PdfMessage> {
        if self.previews.contains_key(&page) || !self.previews_requested.insert(page) {
            return Task::none();
        }
        let size = display.size();
        let scale = PREVIEW_WIDTH / size.width.max(1.0);
        let (width, height) = prev_pdf::engine::page_pixels(size, scale);
        let area = PixelRect {
            x: 0,
            y: 0,
            width,
            height,
        };
        let receiver = self
            .pool
            .render(Arc::clone(display), scale, area, priority, Ticket::new());
        self.current(receiver, move |result| {
            PdfMessage::PreviewReady(
                page,
                result
                    .ok()
                    .and_then(Result::ok)
                    .map(|bitmap| to_handle(bitmap).0),
            )
        })
    }

    fn request_text(&mut self, page: usize, display: &Arc<dyn PageDisplay>) -> Task<PdfMessage> {
        if self.texts.contains_key(&page) || !self.texts_requested.insert(page) {
            return Task::none();
        }
        let receiver = self.pool.text(Arc::clone(display), 8, Ticket::new());
        self.current(receiver, move |result| {
            PdfMessage::TextReady(page, result.ok().and_then(Result::ok).map(Arc::new))
        })
    }

    fn ensure_markup(&mut self, page: usize) -> Task<PdfMessage> {
        if self.markup.contains_key(&page) || !self.markup_requested.insert(page) {
            return Task::none();
        }
        let receiver = self.handle.markup(page);
        self.current(receiver, move |result| {
            PdfMessage::MarkupReady(page, result.ok().and_then(Result::ok))
        })
    }

    /// Requests the parsed page, unless it is loaded or on its way.
    fn ensure_display(&mut self, page: usize) -> Task<PdfMessage> {
        if self.displays.contains_key(&page) || !self.displays_requested.insert(page) {
            return Task::none();
        }
        let receiver = self.handle.display(page);
        self.current(receiver, move |result| {
            PdfMessage::DisplayReady(page, result.ok().and_then(Result::ok).map(SharedDisplay))
        })
    }

    /// Requests previews for pages shown as sidebar thumbnails.
    pub fn request_thumbnails(
        &mut self,
        pages: impl IntoIterator<Item = usize>,
    ) -> Task<PdfMessage> {
        let count = self.page_count();
        let mut tasks = Vec::new();
        for page in pages.into_iter().filter(|page| *page < count) {
            match self.displays.get(&page).cloned() {
                Some(display) => tasks.push(self.request_preview(page, &display, 12)),
                None => {
                    self.thumbnails_waiting.insert(page);
                    tasks.push(self.ensure_display(page));
                }
            }
        }
        Task::batch(tasks)
    }

    /// Evicts least recently used tiles over the budget, keeping those the
    /// view uses now.
    fn evict_tiles(&mut self) {
        if self.tile_bytes <= TILE_BUDGET_BYTES {
            return;
        }
        let mut by_age: Vec<(TileKey, u64, usize)> = self
            .tiles
            .iter()
            .map(|(key, (_, bytes, used))| (*key, *used, *bytes))
            .collect();
        by_age.sort_by_key(|(_, used, _)| *used);
        let scale = scale_key(self.render_scale());
        let visible: HashSet<usize> = self.layout.visible_pages(&self.view).into_iter().collect();
        for (key, _, bytes) in by_age {
            if self.tile_bytes <= TILE_BUDGET_BYTES * 3 / 4 {
                break;
            }
            if key.scale == scale && visible.contains(&key.page) {
                continue;
            }
            self.tiles.remove(&key);
            self.tile_bytes -= bytes;
        }
    }

    // Selection and links.

    fn press(&mut self, x: f32, y: f32, clicks: u8) -> Task<PdfMessage> {
        // Pressing on the selection or the chosen area may start a drag of
        // it, whatever else would happen here.
        if clicks == 1
            && matches!(
                self.edit.tool,
                super::markup::Tool::Select | super::markup::Tool::Area
            )
            && self.edit.text.is_none()
            && let Some(out) = self.drag_out_at(x, y)
        {
            self.edit.drag = Some(editing::Drag::Out { start: (x, y), out });
            return Task::none();
        }
        if let Some(task) = self.editing_press(x, y, clicks) {
            return task;
        }
        self.text_press(x, y, clicks)
    }

    fn text_press(&mut self, x: f32, y: f32, clicks: u8) -> Task<PdfMessage> {
        self.press = Some((x, y));
        self.dragged = false;
        let Some(anchor) = self.layout.hit_nearest(x, y) else {
            return Task::none();
        };
        let granularity = match clicks {
            2 => Granularity::Word,
            3.. => Granularity::Line,
            _ => Granularity::Character,
        };
        self.selection = Some(TextSelection {
            anchor,
            focus: anchor,
            granularity,
        });
        Task::none()
    }

    fn drag(&mut self, x: f32, y: f32) {
        if self.editing_drag(x, y) {
            return;
        }
        if let Some((press_x, press_y)) = self.press
            && (x - press_x).abs() + (y - press_y).abs() > 3.0
        {
            self.dragged = true;
        }
        // Only a press that began a selection extends it; one that went
        // on to drag the selection out does not.
        if self.press.is_none() {
            return;
        }
        if let (Some(selection), Some(focus)) =
            (self.selection.as_mut(), self.layout.hit_nearest(x, y))
        {
            selection.focus = focus;
            if selection.granularity != Granularity::Character && self.dragged {
                selection.granularity = Granularity::Character;
            }
        }
    }

    fn release(&mut self, x: f32, y: f32) -> Task<PdfMessage> {
        if let Some(task) = self.editing_release(x, y) {
            return task;
        }
        let was_click = !self.dragged;
        self.press = None;
        if let Some(selection) = self.selection
            && selection.granularity == Granularity::Character
            && was_click
        {
            self.selection = None;
        }
        if was_click && let Some(target) = self.link_at(x, y) {
            return match target {
                LinkTarget::Page { index, point } => self.go_to(index, point),
                LinkTarget::Uri(uri) => {
                    self.requests.push(Request::OpenUri(uri));
                    Task::none()
                }
            };
        }
        Task::none()
    }

    pub fn link_at(&self, x: f32, y: f32) -> Option<LinkTarget> {
        let (page, point) = self.layout.hit(x, y)?;
        self.links
            .get(&page)?
            .iter()
            .find(|link| link.bounds.contains(point))
            .map(|link| link.target.clone())
    }

    pub fn is_over_text(&self, x: f32, y: f32) -> bool {
        self.layout.hit(x, y).is_some_and(|(page, point)| {
            self.texts
                .get(&page)
                .is_some_and(|text| text.is_over_text(point))
        })
    }

    /// The selected range on `page`, if the selection covers it.
    pub fn page_selection(&self, page: usize) -> Option<Selection> {
        let selection = self.selection?;
        let text = self.texts.get(&page)?;
        let (start, end) = if (selection.anchor.0, selection.anchor.1.y)
            <= (selection.focus.0, selection.focus.1.y)
        {
            (selection.anchor, selection.focus)
        } else {
            (selection.focus, selection.anchor)
        };
        if page < start.0 || page > end.0 {
            return None;
        }
        let all = text.select_all()?;
        let range = match selection.granularity {
            Granularity::Word if start == end => text.select_word(start.1)?,
            Granularity::Line if start == end => text.select_line(start.1)?,
            _ if start.0 == end.0 => text.select(start.1, end.1)?,
            _ if page == start.0 => Selection {
                start: text.caret_at(start.1)?,
                end: all.end,
            },
            _ if page == end.0 => Selection {
                start: all.start,
                end: text.caret_at(end.1)?,
            },
            _ => all,
        };
        (!range.is_empty()).then_some(range)
    }

    pub fn select_all(&mut self) {
        let last = self.page_count().saturating_sub(1);
        let end = self
            .info
            .page_sizes
            .get(last)
            .map_or(Point::default(), |size| Point::new(size.width, size.height));
        self.selection = Some(TextSelection {
            anchor: (0, Point::default()),
            focus: (last, end),
            granularity: Granularity::Character,
        });
    }

    /// Copies the selection, loading text for pages not yet seen.
    pub fn copy_selection(&mut self) -> Task<PdfMessage> {
        let Some(selection) = self.selection else {
            return Task::none();
        };
        let first = selection.anchor.0.min(selection.focus.0);
        let last = selection.anchor.0.max(selection.focus.0);
        let missing: Vec<usize> = (first..=last)
            .filter(|page| !self.texts.contains_key(page))
            .collect();
        if !missing.is_empty() {
            self.pending_copy = true;
            let mut tasks = Vec::new();
            for page in missing {
                match self.displays.get(&page).cloned() {
                    Some(display) => tasks.push(self.request_text(page, &display)),
                    // Its text is requested when it arrives, see `DisplayReady`.
                    None => tasks.push(self.ensure_display(page)),
                }
            }
            return Task::batch(tasks);
        }
        self.pending_copy = false;
        match self.selected_text() {
            Some(text) => iced::clipboard::write(text),
            None => Task::none(),
        }
    }

    /// The selected text, from the pages whose text is loaded.
    pub fn selected_text(&self) -> Option<String> {
        let selection = self.selection?;
        let first = selection.anchor.0.min(selection.focus.0);
        let last = selection.anchor.0.max(selection.focus.0);
        let text: Vec<String> = (first..=last)
            .filter_map(|page| Some(self.texts.get(&page)?.text(self.page_selection(page)?)))
            .collect();
        (!text.is_empty()).then(|| text.join("\n"))
    }

    // Search.

    fn start_search(&mut self, query: String) -> Task<PdfMessage> {
        if let Some(ticket) = self.search.ticket.take() {
            ticket.cancel();
        }
        let generation = self.search.generation + 1;
        self.search = SearchState {
            query: query.clone(),
            generation,
            ..SearchState::default()
        };
        if query.trim().is_empty() {
            return Task::none();
        }
        let ticket = Ticket::new();
        self.search.ticket = Some(ticket.clone());
        let events = self.handle.search(query, ticket);
        Task::run(events, move |event| {
            PdfMessage::SearchEvent(generation, event)
        })
    }

    fn search_event(&mut self, generation: u64, event: SearchEvent) {
        if generation != self.search.generation {
            return;
        }
        match event {
            SearchEvent::Matches { page, quads } => {
                let first_match = self.search.matches.is_empty();
                self.search
                    .matches
                    .extend(quads.into_iter().map(|quad| (page, quad)));
                if first_match {
                    self.select_match(0);
                }
            }
            SearchEvent::Progress { searched, .. } => self.search.searched = searched,
            SearchEvent::Finished => self.search.finished = true,
        }
    }

    fn step_match(&mut self, step: i64) -> Task<PdfMessage> {
        let count = self.search.matches.len() as i64;
        if count == 0 {
            return Task::none();
        }
        let next = self
            .search
            .current
            .map_or(0, |current| (current as i64 + step).rem_euclid(count));
        self.select_match(next as usize);
        self.schedule()
    }

    fn select_match(&mut self, index: usize) {
        self.search.current = Some(index);
        let (page, quad) = self.search.matches[index];
        if self.mode != ViewMode::Continuous && self.current != page {
            self.current = page;
            self.relayout();
        }
        let bounds = quad.bounds();
        let Some((x, y)) = self
            .layout
            .to_document(page, Point::new(bounds.x0, bounds.y0))
        else {
            return;
        };
        let visible = self.view.contains(x, y) && self.view.contains(x, y + 20.0);
        if !visible {
            self.scroll_to(x - self.view.width / 3.0, y - self.view.height / 3.0);
        }
    }

    /// Search matches on `page`, with whether each is the current one.
    pub fn page_matches(&self, page: usize) -> impl Iterator<Item = (Rect, bool)> + '_ {
        self.search
            .matches
            .iter()
            .enumerate()
            .filter(move |(_, (match_page, _))| *match_page == page)
            .map(|(index, (_, quad))| (quad.bounds(), self.search.current == Some(index)))
    }
}

fn size_of(area: &Area) -> Size {
    Size::new(area.width, area.height)
}

fn to_handle(bitmap: Bitmap) -> (Handle, usize) {
    let bytes = bitmap.pixels.len();
    (
        Handle::from_rgba(bitmap.width, bitmap.height, bitmap.pixels),
        bytes,
    )
}
