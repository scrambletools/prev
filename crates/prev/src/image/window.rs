//! A window showing one image, or several with a thumbnail sidebar.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bytes::Bytes;
use iced::futures::channel::oneshot;
use iced::keyboard::{Key, Modifiers, key::Named};
use iced::widget::image::Handle;
use iced::widget::scrollable::{self, Direction, Scrollbar};
use iced::widget::{
    Id, button, center, column, container, image, mouse_area, operation, row, rule,
    scrollable as scroll, space, text,
};
use iced::{Center, Color, Element, Fill, Length, Task};
use prev_image::ImageFormat;
use prev_image::decode::decode_file;
use prev_image::svg::Svg;

use super::canvas::{CanvasEvent, ImageCanvas};
use super::view::{self, Fit, Placement, ZOOM_STEP};
use crate::shortcuts::Action;

const THUMBNAIL_SIZE: u32 = 240;
const SIDEBAR_WIDTH: f32 = 180.0;
const SIDEBAR_THUMBNAIL: f32 = 120.0;
/// Full images kept in memory around the current one, in each direction.
const KEEP_AROUND: usize = 1;

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
    /// First frame pixels, for making downscaled copies.
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
    /// Downscaled copies by divisor.
    levels: HashMap<u32, Handle>,
    levels_pending: HashSet<u32>,
    /// SVG render and the scale it was made at.
    render: Option<(f32, Handle)>,
    render_pending: Option<f32>,
}

struct Item {
    path: PathBuf,
    source: Source,
    state: ItemState,
    thumbnail: Option<Handle>,
    size: Option<(u32, u32)>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(usize, Result<LoadedImage, String>),
    LevelReady(usize, u32, Option<Handle>),
    SvgRendered(usize, f32, Option<Handle>),
    Canvas(CanvasEvent),
    Select(usize),
    ToggleSidebar,
    ZoomIn,
    ZoomOut,
    ActualSize,
    FitToWindow,
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
}

/// Runs `work` on its own thread and resolves with its result.
fn spawn<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> oneshot::Receiver<T> {
    let (sender, receiver) = oneshot::channel();
    std::thread::Builder::new()
        .name("prev-image-load".into())
        .spawn(move || {
            let _ = sender.send(work());
        })
        .expect("spawn image thread");
    receiver
}

fn rgba_handle(width: u32, height: u32, pixels: Bytes) -> Handle {
    Handle::from_rgba(width, height, pixels)
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
            let thumbnail = {
                let first = &decoded.frames[0];
                let buffer =
                    ::image::RgbaImage::from_raw(first.width, first.height, first.pixels.clone())
                        .ok_or("frame has the wrong size")?;
                let scale = THUMBNAIL_SIZE as f32 / width.max(height).max(1) as f32;
                let small = if scale < 1.0 {
                    ::image::imageops::thumbnail(
                        &buffer,
                        ((width as f32 * scale).round() as u32).max(1),
                        ((height as f32 * scale).round() as u32).max(1),
                    )
                } else {
                    buffer
                };
                rgba_handle(small.width(), small.height(), small.into_raw().into())
            };
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

    fn placement(&self) -> Option<Placement> {
        let (width, height) = self.items.get(self.current)?.size?;
        let image = (width as f32, height as f32);
        let viewport = (self.view.2, self.view.3);
        let zoom = view::resolve_zoom(self.fit, image, viewport);
        Some(view::place(image, zoom, viewport))
    }

    /// Loads the current image first, then the others one at a time for
    /// their thumbnails, and drops full images far from the current one.
    fn load_next(&mut self) -> Task<Message> {
        if self
            .items
            .iter()
            .any(|item| matches!(item.state, ItemState::Loading))
        {
            return Task::none();
        }
        let near = |index: usize, current: usize| index.abs_diff(current) <= KEEP_AROUND;
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
            if !near(index, current) && matches!(item.state, ItemState::Loaded(_)) {
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
                            ItemState::Loaded(Box::new(Shown {
                                image,
                                levels: HashMap::new(),
                                levels_pending: HashSet::new(),
                                render: None,
                                render_pending: None,
                            }))
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
            Message::LevelReady(index, level, handle) => {
                if let Some(Item {
                    state: ItemState::Loaded(shown),
                    ..
                }) = self.items.get_mut(index)
                {
                    shown.levels_pending.remove(&level);
                    if let Some(handle) = handle {
                        shown.levels.insert(level, handle);
                    }
                }
                Task::none()
            }
            Message::SvgRendered(index, scale, handle) => {
                if let Some(Item {
                    state: ItemState::Loaded(shown),
                    ..
                }) = self.items.get_mut(index)
                {
                    shown.render_pending = None;
                    if let Some(handle) = handle {
                        shown.render = Some((scale, handle));
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
        Task::batch([self.load_next(), self.schedule(), follow])
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
        let Some(Item {
            state: ItemState::Loaded(shown),
            ..
        }) = self.items.get_mut(index)
        else {
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
                    .map(|frame| rgba_handle(frame.width, frame.height, frame.pixels.into()));
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

    /// The handle to draw: the animation frame, the best downscaled copy,
    /// the SVG render, or the full image.
    fn display_handle(&self) -> Option<&Handle> {
        let shown = self.shown()?;
        if shown.image.svg.is_some() {
            return shown
                .render
                .as_ref()
                .map(|(_, handle)| handle)
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
            .or_else(|| shown.image.frames.first().map(|(handle, _)| handle))
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
                Some(placement) => scroll(ImageCanvas::new(
                    self.display_handle(),
                    placement,
                    self.next_frame,
                    Message::Canvas,
                ))
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
        let content: Element<'_, Message> = if self.sidebar && self.items.len() > 1 {
            row![
                container(self.sidebar_view())
                    .width(SIDEBAR_WIDTH)
                    .height(Fill),
                rule::vertical(1),
                canvas
            ]
            .into()
        } else {
            canvas
        };
        column![self.toolbar(), rule::horizontal(1), content].into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        let item = &self.items[self.current];
        let mut details = item
            .size
            .map(|(width, height)| format!("{width} × {height}"))
            .unwrap_or_default();
        if let Some(shown) = self.shown()
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
        let small =
            |label: &'static str, message: Message| button(text(label).size(13)).on_press(message);
        let mut bar = row![].spacing(6).padding(6).align_y(Center);
        if self.items.len() > 1 {
            bar = bar.push(small("Sidebar", Message::ToggleSidebar));
        }
        bar.push(text(details).size(13))
            .push(space::horizontal())
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
