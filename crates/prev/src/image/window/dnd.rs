//! Drag and drop in an image window: dropped images join the window,
//! dropped pictures and text go on the markup, and images in the sidebar
//! drag out to other windows and apps as their files.

use std::path::PathBuf;

use iced::{Point, Task};

use super::{ImageWindow, Item, ItemState, Message, Source};
use crate::drag::{self, Action, Dropped};
use crate::filetype::{self, FileKind};

/// How far the pointer moves on a sidebar image before it drags.
const DRAG_THRESHOLD: f32 = 6.0;

/// A press on an image in the sidebar, which may become a drag.
#[derive(Debug, Clone, Copy)]
pub struct SidebarPress {
    index: usize,
    start: Option<Point>,
}

/// What to do once the markup opens.
#[derive(Debug, Clone)]
pub enum OnOpen {
    Paste,
    Drop(f32, f32, Dropped),
}

impl ImageWindow {
    /// Takes a drop at `x`, `y` in the window. Returns files it does not
    /// take, for the app to open.
    pub fn drop_in(
        &mut self,
        x: f32,
        y: f32,
        dropped: Dropped,
        action: Action,
    ) -> (Task<Message>, Vec<PathBuf>) {
        let index = self.current;
        let onto_markup = self.markup().is_some_and(|markup| {
            markup.window.markup_bar_shown() && markup.window.is_over_pages(x, y)
        });
        match dropped {
            // With the markup bar open, one image file dropped on the
            // picture goes on the markup; otherwise images join the window.
            Dropped::Files(paths) if !(onto_markup && paths.len() == 1) => {
                let (images, others): (Vec<_>, Vec<_>) = paths
                    .into_iter()
                    .map(|path| prev_store::paths::canonical(&path))
                    .partition(|path| image_source(path).is_some());
                (self.add_images(images), others)
            }
            Dropped::Pages(_) => {
                self.notice = Some(crate::fl!("image-drop-pages"));
                (Task::none(), Vec::new())
            }
            Dropped::Nothing => (Task::none(), Vec::new()),
            dropped => match self.items[index].markup.as_mut() {
                Some(markup) => {
                    let (task, files) = markup.window.drop_in(x, y, dropped, action);
                    (task.map(super::markup::wrap(index)), files)
                }
                None => {
                    // The markup opens first, then takes the drop.
                    let task = self.toggle_markup();
                    let item = &mut self.items[index];
                    if item.markup_starting {
                        item.on_open = Some(OnOpen::Drop(x, y, dropped));
                    }
                    (task, Vec::new())
                }
            },
        }
    }

    /// Adds the images among `paths` to the window, returning the other
    /// files, for opening in windows of their own.
    pub fn add_files(&mut self, paths: Vec<PathBuf>) -> (Task<Message>, Vec<PathBuf>) {
        let (images, others): (Vec<_>, Vec<_>) = paths
            .into_iter()
            .map(|path| prev_store::paths::canonical(&path))
            .partition(|path| image_source(path).is_some());
        (self.add_images(images), others)
    }

    /// Adds `paths` to the window after its images, skipping those it has,
    /// and shows the first.
    pub fn add_images(&mut self, paths: Vec<PathBuf>) -> Task<Message> {
        let first = self.items.len();
        for path in paths {
            if self.items.iter().any(|item| item.path == path) {
                continue;
            }
            let Some(source) = image_source(&path) else {
                continue;
            };
            self.items.push(Item::new(path, source));
        }
        if self.items.len() == first {
            return Task::none();
        }
        self.sidebar = true;
        self.select(first)
    }

    pub(super) fn sidebar_pressed(&mut self, index: usize) -> Task<Message> {
        self.sidebar_press = Some(SidebarPress { index, start: None });
        self.select(index)
    }

    /// The pointer moved over the sidebar: past a small move, a pressed
    /// image drags out as its file.
    pub(super) fn sidebar_moved(&mut self, point: Point) {
        let Some(press) = self.sidebar_press.as_mut() else {
            return;
        };
        let start = *press.start.get_or_insert(point);
        if (point.x - start.x).abs() + (point.y - start.y).abs() < DRAG_THRESHOLD {
            return;
        }
        let index = press.index;
        self.sidebar_press = None;
        let Some(item) = self.items.get(index) else {
            return;
        };
        let icon = item.thumbnail.as_ref().and_then(drag::icon_from_handle);
        if drag::start(drag::file_data(&item.path), icon, false) {
            self.drag_started = true;
        } else {
            self.notice = Some(crate::fl!("image-drag-failed"));
        }
    }

    pub(super) fn sidebar_released(&mut self) {
        self.sidebar_press = None;
    }

    /// Whether this window started the drag under way.
    pub fn take_drag_started(&mut self) -> bool {
        let mut started = std::mem::take(&mut self.drag_started);
        for markup in self
            .items
            .iter_mut()
            .filter_map(|item| item.markup.as_mut())
        {
            started |= markup.window.take_drag_started();
        }
        started
    }
}

/// How an image file is shown, or `None` for files that are not images.
fn image_source(path: &std::path::Path) -> Option<Source> {
    match filetype::detect_path(path) {
        Ok(Some(FileKind::Image(format))) => Some(Source::Raster(format)),
        Ok(Some(FileKind::Svg)) => Some(Source::Svg),
        _ => None,
    }
}

impl Item {
    pub(super) fn new(path: PathBuf, source: Source) -> Self {
        Item {
            path,
            source,
            state: ItemState::Waiting,
            thumbnail: None,
            size: None,
            markup: None,
            markup_starting: false,
            on_open: None,
        }
    }
}
