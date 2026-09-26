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

/// Device pixels across a page preview, used until sharp tiles arrive and
/// for sidebar thumbnails.
const PREVIEW_WIDTH: f32 = 320.0;
const TILE_BUDGET_BYTES: usize = 128 * 1024 * 1024;
const ZOOM_STEP: f32 = 1.25;
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
    SetMode(ViewMode),
    GoTo {
        page: usize,
        point: Option<Point>,
    },
    NextPage,
    PreviousPage,
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
    SearchChanged(String),
    SearchEvent(u64, SearchEvent),
    NextMatch,
    PreviousMatch,
}

/// Something the app must do on the viewer's behalf.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    ScrollTo { x: f32, y: f32 },
    OpenUri(String),
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
    pub search: SearchState,
    requests: Vec<Request>,
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
            search: SearchState::default(),
            requests: Vec::new(),
        }
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

    pub fn tile(&self, key: &TileKey) -> Option<&Handle> {
        self.tiles.get(key).map(|(handle, _, _)| handle)
    }

    pub fn tile_key(&self, page: usize, tile: &PixelRect) -> TileKey {
        TileKey {
            page,
            scale: scale_key(self.render_scale()),
            x: tile.x,
            y: tile.y,
        }
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
                Task::batch([copy_text, self.schedule()])
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
                    self.use_clock += 1;
                    self.tile_bytes += bytes;
                    self.tiles.insert(key, (handle, bytes, self.use_clock));
                    self.evict_tiles();
                }
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
            PdfMessage::SetMode(mode) => {
                self.mode = mode;
                self.keep_position(|viewer| viewer.relayout());
                self.schedule()
            }
            PdfMessage::GoTo { page, point } => self.go_to(page, point),
            PdfMessage::NextPage => self.step_page(1),
            PdfMessage::PreviousPage => self.step_page(-1),
            PdfMessage::ScrollBy { dx, dy } => {
                self.scroll_to(self.view.x + dx, self.view.y + dy);
                self.schedule()
            }
            PdfMessage::Press { x, y, clicks } => self.press(x, y, clicks),
            PdfMessage::Drag { x, y } => {
                self.drag(x, y);
                Task::none()
            }
            PdfMessage::Release { x, y } => self.release(x, y),
            PdfMessage::SearchChanged(query) => self.start_search(query),
            PdfMessage::SearchEvent(generation, event) => {
                self.search_event(generation, event);
                Task::none()
            }
            PdfMessage::NextMatch => self.step_match(1),
            PdfMessage::PreviousMatch => self.step_match(-1),
        }
    }

    /// Runs a relayout while keeping the point at the top-left of the view
    /// (or the page start in single page modes) in place.
    fn keep_position(&mut self, change: impl FnOnce(&mut Self)) {
        let anchor = self.layout.hit_nearest(self.view.x, self.view.y);
        change(self);
        if let Some((page, point)) = anchor {
            if self.mode == ViewMode::Continuous {
                if let Some((x, y)) = self.layout.to_document(page, point) {
                    self.scroll_to(x, y);
                }
            } else {
                self.scroll_to(0.0, 0.0);
            }
        }
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

    fn step_page(&mut self, step: i64) -> Task<PdfMessage> {
        let step = if self.mode == ViewMode::TwoPages && self.current > 0 {
            step * 2
        } else {
            step
        };
        let target = (self.current as i64 + step).clamp(0, self.page_count() as i64 - 1) as usize;
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
            if !self.links.contains_key(&page) && self.links_requested.insert(page) {
                let receiver = self.handle.links(page);
                tasks.push(Task::perform(receiver, move |result| {
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
                tasks.push(Task::perform(receiver, move |result| {
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
        Task::perform(receiver, move |result| {
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
        Task::perform(receiver, move |result| {
            PdfMessage::TextReady(page, result.ok().and_then(Result::ok).map(Arc::new))
        })
    }

    /// Requests the parsed page, unless it is loaded or on its way.
    fn ensure_display(&mut self, page: usize) -> Task<PdfMessage> {
        if self.displays.contains_key(&page) || !self.displays_requested.insert(page) {
            return Task::none();
        }
        let receiver = self.handle.display(page);
        Task::perform(receiver, move |result| {
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
                None => tasks.push(self.ensure_display(page)),
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
        if let Some((press_x, press_y)) = self.press
            && (x - press_x).abs() + (y - press_y).abs() > 3.0
        {
            self.dragged = true;
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
        let text: Vec<String> = (first..=last)
            .filter_map(|page| Some(self.texts[&page].text(self.page_selection(page)?)))
            .collect();
        iced::clipboard::write(text.join("\n"))
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
