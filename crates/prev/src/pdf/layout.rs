//! Where pages go on screen: document space is logical pixels at the current
//! zoom, origin at the top-left of the scrollable content.

use prev_pdf::geometry::{PixelRect, Point, Size};

pub const PAGE_GAP: f32 = 12.0;
pub const MARGIN: f32 = 16.0;
/// Tile edge in device pixels.
pub const TILE: u32 = 512;

pub const MIN_ZOOM: f32 = 0.1;
pub const MAX_ZOOM: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Continuous,
    SinglePage,
    TwoPages,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fit {
    Width,
    Page,
    /// A fixed zoom, where 1.0 is actual size.
    Zoom(f32),
}

/// A logical-pixel rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn intersects(&self, other: &Area) -> bool {
        self.x < other.right()
            && other.x < self.right()
            && self.y < other.bottom()
            && other.y < self.bottom()
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
    }
}

/// Points to logical pixels at `zoom` (1.0 is 96 dpi).
pub fn points_to_pixels(zoom: f32) -> f32 {
    prev_pdf::engine::zoom_to_scale(zoom)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub zoom: f32,
    /// Page rectangles in document space; empty for pages not shown.
    pub pages: Vec<Option<Area>>,
    pub content: Size,
}

/// The pages shown in `SinglePage` mode for `current`: just that page, or in
/// `TwoPages` mode the spread containing it (first page alone, like a book).
fn spread_of(mode: ViewMode, current: usize) -> (usize, Option<usize>) {
    match mode {
        ViewMode::TwoPages if current == 0 => (0, None),
        ViewMode::TwoPages => {
            let left = if current % 2 == 1 {
                current
            } else {
                current - 1
            };
            (left, Some(left + 1))
        }
        _ => (current, None),
    }
}

/// The zoom that fits `fit` into the viewport.
pub fn resolve_zoom(
    fit: Fit,
    mode: ViewMode,
    sizes: &[Size],
    current: usize,
    viewport: Size,
) -> f32 {
    let zoom = match fit {
        Fit::Zoom(zoom) => zoom,
        Fit::Width | Fit::Page => {
            let (widest, tallest) = match mode {
                ViewMode::Continuous => (
                    sizes.iter().map(|size| size.width).fold(0.0, f32::max),
                    sizes.get(current).map_or(0.0, |size| size.height),
                ),
                ViewMode::SinglePage => {
                    let size = sizes.get(current).copied().unwrap_or_default();
                    (size.width, size.height)
                }
                ViewMode::TwoPages => {
                    let pair_width = |left: usize| {
                        let (_, right) = spread_of(mode, left);
                        sizes.get(left).map_or(0.0, |size| size.width)
                            + right
                                .and_then(|right| sizes.get(right))
                                .map_or(0.0, |size| size.width + PAGE_GAP)
                    };
                    let (left, right) = spread_of(mode, current);
                    let height = [Some(left), right]
                        .into_iter()
                        .flatten()
                        .filter_map(|index| sizes.get(index))
                        .map(|size| size.height)
                        .fold(0.0, f32::max);
                    (
                        pair_width(left).max(pair_width(1.min(sizes.len().saturating_sub(1)))),
                        height,
                    )
                }
            };
            let width_zoom =
                (viewport.width - 2.0 * MARGIN).max(1.0) / points_to_pixels(1.0) / widest.max(1.0);
            if fit == Fit::Width {
                width_zoom
            } else {
                let height_zoom = (viewport.height - 2.0 * MARGIN).max(1.0)
                    / points_to_pixels(1.0)
                    / tallest.max(1.0);
                width_zoom.min(height_zoom)
            }
        }
    };
    zoom.clamp(MIN_ZOOM, MAX_ZOOM)
}

pub fn layout(mode: ViewMode, sizes: &[Size], current: usize, zoom: f32, viewport: Size) -> Layout {
    let scale = points_to_pixels(zoom);
    let scaled = |index: usize| {
        let size = sizes[index];
        (size.width * scale, size.height * scale)
    };
    let mut pages = vec![None; sizes.len()];
    let mut content_width: f32 = 0.0;
    let mut y = MARGIN;

    let rows: Vec<Vec<usize>> = match mode {
        ViewMode::Continuous => (0..sizes.len()).map(|index| vec![index]).collect(),
        ViewMode::TwoPages => {
            let mut rows = vec![vec![0]];
            rows.extend(
                (1..sizes.len())
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|pair| pair.to_vec()),
            );
            rows.retain(|row| row.iter().all(|index| *index < sizes.len()));
            rows
        }
        ViewMode::SinglePage if sizes.is_empty() => Vec::new(),
        ViewMode::SinglePage => vec![vec![current.min(sizes.len() - 1)]],
    };

    let row_width = |row: &[usize]| {
        row.iter().map(|index| scaled(*index).0).sum::<f32>()
            + PAGE_GAP * row.len().saturating_sub(1) as f32
    };
    for row in &rows {
        content_width = content_width.max(row_width(row));
    }
    let content_width = content_width.max(viewport.width - 2.0 * MARGIN);

    for row in &rows {
        let height = row.iter().map(|index| scaled(*index).1).fold(0.0, f32::max);
        let mut x = MARGIN + (content_width - row_width(row)) / 2.0;
        for &index in row {
            let (width, page_height) = scaled(index);
            pages[index] = Some(Area {
                x,
                y: y + (height - page_height) / 2.0,
                width,
                height: page_height,
            });
            x += width + PAGE_GAP;
        }
        y += height + PAGE_GAP;
    }
    let content_height = (y - PAGE_GAP + MARGIN).max(viewport.height);
    // A single page is centred vertically when it is shorter than the view.
    if mode == ViewMode::SinglePage {
        for area in pages.iter_mut().flatten() {
            area.y = area.y.max((viewport.height - area.height) / 2.0);
        }
    }
    Layout {
        zoom,
        pages,
        content: Size::new(content_width + 2.0 * MARGIN, content_height),
    }
}

impl Layout {
    pub fn page_area(&self, index: usize) -> Option<Area> {
        self.pages.get(index).copied().flatten()
    }

    /// Pages overlapping `view`, nearest to its centre first.
    pub fn visible_pages(&self, view: &Area) -> Vec<usize> {
        let center_y = view.y + view.height / 2.0;
        let mut visible: Vec<usize> = self
            .pages
            .iter()
            .enumerate()
            .filter_map(|(index, area)| area.filter(|area| area.intersects(view)).map(|_| index))
            .collect();
        visible.sort_by(|a, b| {
            let distance = |index: &usize| {
                let area = self.pages[*index].unwrap();
                (area.y + area.height / 2.0 - center_y).abs()
            };
            distance(a).total_cmp(&distance(b))
        });
        visible
    }

    /// The page whose area contains the view's vertical centre, or the
    /// nearest visible one: the page shown in the page counter.
    pub fn current_page(&self, view: &Area) -> Option<usize> {
        self.visible_pages(view).first().copied()
    }

    /// A document-space point to a page and a point on it, in page points.
    pub fn hit(&self, x: f32, y: f32) -> Option<(usize, Point)> {
        let scale = points_to_pixels(self.zoom);
        self.pages.iter().enumerate().find_map(|(index, area)| {
            let area = (*area)?;
            area.contains(x, y).then(|| {
                (
                    index,
                    Point::new((x - area.x) / scale, (y - area.y) / scale),
                )
            })
        })
    }

    /// Like `hit`, but outside pages it picks the nearest page and clamps,
    /// so a selection drag can leave the page.
    pub fn hit_nearest(&self, x: f32, y: f32) -> Option<(usize, Point)> {
        let scale = points_to_pixels(self.zoom);
        self.pages
            .iter()
            .enumerate()
            .filter_map(|(index, area)| area.map(|area| (index, area)))
            .min_by(|(_, a), (_, b)| distance_to(a, x, y).total_cmp(&distance_to(b, x, y)))
            .map(|(index, area)| {
                let px = (x - area.x).clamp(0.0, area.width) / scale;
                let py = (y - area.y).clamp(0.0, area.height) / scale;
                (index, Point::new(px, py))
            })
    }

    /// Document space to a point on `page`, which may lie off the page, so
    /// a drag that started on a page keeps its coordinates there.
    pub fn to_page(&self, page: usize, x: f32, y: f32) -> Option<Point> {
        let scale = points_to_pixels(self.zoom);
        let area = self.page_area(page)?;
        Some(Point::new((x - area.x) / scale, (y - area.y) / scale))
    }

    /// A page-point to document space.
    pub fn to_document(&self, page: usize, point: Point) -> Option<(f32, f32)> {
        let scale = points_to_pixels(self.zoom);
        let area = self.page_area(page)?;
        Some((area.x + point.x * scale, area.y + point.y * scale))
    }
}

fn distance_to(area: &Area, x: f32, y: f32) -> f32 {
    let dx = if x < area.x {
        area.x - x
    } else if x > area.right() {
        x - area.right()
    } else {
        0.0
    };
    let dy = if y < area.y {
        area.y - y
    } else if y > area.bottom() {
        y - area.bottom()
    } else {
        0.0
    };
    dx * dx + dy * dy
}

/// The tiles of a page, in device pixels at `scale`, that overlap `view`.
/// `page` and `view` are in document space; `device_scale` is the window's
/// scale factor.
pub fn visible_tiles(page: &Area, view: &Area, device_scale: f32) -> Vec<PixelRect> {
    let page_width = (page.width * device_scale).ceil() as i64;
    let page_height = (page.height * device_scale).ceil() as i64;
    let to_device = |value: f32| (value * device_scale).floor() as i64;
    let left = (to_device(view.x - page.x)).clamp(0, page_width);
    let top = (to_device(view.y - page.y)).clamp(0, page_height);
    let right = (to_device(view.right() - page.x) + 1).clamp(0, page_width);
    let bottom = (to_device(view.bottom() - page.y) + 1).clamp(0, page_height);
    let tile = i64::from(TILE);
    let mut tiles = Vec::new();
    let mut ty = top / tile * tile;
    while ty < bottom {
        let mut tx = left / tile * tile;
        while tx < right {
            tiles.push(PixelRect {
                x: tx as i32,
                y: ty as i32,
                width: (tile.min(page_width - tx)) as u32,
                height: (tile.min(page_height - ty)) as u32,
            });
            tx += tile;
        }
        ty += tile;
    }
    tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    const LETTER: Size = Size::new(612.0, 792.0);
    const LANDSCAPE: Size = Size::new(792.0, 612.0);

    #[test]
    fn continuous_stacks_and_centres_pages() {
        let sizes = [LETTER, LANDSCAPE, LETTER];
        let layout = layout(
            ViewMode::Continuous,
            &sizes,
            0,
            1.0,
            Size::new(800.0, 600.0),
        );
        let first = layout.page_area(0).unwrap();
        let second = layout.page_area(1).unwrap();
        assert_eq!(first.width, 816.0, "612 points at 96 dpi");
        assert_eq!(second.y, first.bottom() + PAGE_GAP);
        assert!(
            (first.x + first.width / 2.0 - (second.x + second.width / 2.0)).abs() < 0.01,
            "centred"
        );
        assert_eq!(
            layout.content.width,
            1056.0 + 2.0 * MARGIN,
            "widest page plus margins"
        );
    }

    #[test]
    fn fits() {
        let sizes = [LETTER, LETTER];
        let viewport = Size::new(1000.0, 800.0);
        let width = resolve_zoom(Fit::Width, ViewMode::Continuous, &sizes, 0, viewport);
        let page = resolve_zoom(Fit::Page, ViewMode::Continuous, &sizes, 0, viewport);
        let fitted = layout(ViewMode::Continuous, &sizes, 0, width, viewport);
        assert!((fitted.page_area(0).unwrap().width - (1000.0 - 2.0 * MARGIN)).abs() < 0.5);
        let fitted = layout(ViewMode::Continuous, &sizes, 0, page, viewport);
        assert!(fitted.page_area(0).unwrap().height <= 800.0 - 2.0 * MARGIN + 0.5);
        assert!(page < width);
        assert_eq!(
            resolve_zoom(Fit::Zoom(100.0), ViewMode::Continuous, &sizes, 0, viewport),
            MAX_ZOOM
        );
    }

    #[test]
    fn two_pages_puts_the_first_page_alone() {
        let sizes = [LETTER; 5];
        let layout = layout(ViewMode::TwoPages, &sizes, 0, 1.0, Size::new(800.0, 600.0));
        let areas: Vec<Area> = (0..5)
            .map(|index| layout.page_area(index).unwrap())
            .collect();
        assert!(areas[1].y > areas[0].y);
        assert_eq!(areas[1].y, areas[2].y, "pages 2 and 3 share a row");
        assert!(areas[2].x > areas[1].x);
        assert_eq!(areas[3].y, areas[4].y);
    }

    #[test]
    fn single_page_shows_only_the_current_page() {
        let sizes = [LETTER; 3];
        let layout = layout(
            ViewMode::SinglePage,
            &sizes,
            1,
            0.5,
            Size::new(800.0, 900.0),
        );
        assert!(layout.page_area(0).is_none() && layout.page_area(2).is_none());
        let area = layout.page_area(1).unwrap();
        assert!(
            (area.y - (900.0 - area.height) / 2.0).abs() < 0.01,
            "vertically centred"
        );
    }

    #[test]
    fn visible_pages_and_hits() {
        let sizes = [LETTER; 10];
        let layout = layout(
            ViewMode::Continuous,
            &sizes,
            0,
            1.0,
            Size::new(900.0, 700.0),
        );
        let second = layout.page_area(1).unwrap();
        let view = Area {
            x: 0.0,
            y: second.y + 500.0,
            width: 900.0,
            height: 700.0,
        };
        assert_eq!(layout.current_page(&view), Some(1));
        let visible = layout.visible_pages(&view);
        assert!(visible.contains(&1) && visible.contains(&2) && !visible.contains(&0));

        let (page, point) = layout.hit(second.x + 96.0, second.y + 48.0).unwrap();
        assert_eq!(page, 1);
        assert!(
            (point.x - 72.0).abs() < 0.01 && (point.y - 36.0).abs() < 0.01,
            "{point:?}"
        );
        assert_eq!(layout.hit(second.x - 5.0, second.y), None);
        let (page, point) = layout.hit_nearest(second.x - 5.0, second.y + 48.0).unwrap();
        assert_eq!((page, point.x), (1, 0.0));
        let (x, y) = layout.to_document(1, Point::new(72.0, 36.0)).unwrap();
        assert!((x - (second.x + 96.0)).abs() < 0.01 && (y - (second.y + 48.0)).abs() < 0.01);
    }

    #[test]
    fn tiles_cover_the_visible_part_of_a_page() {
        let page = Area {
            x: 16.0,
            y: 16.0,
            width: 816.0,
            height: 1056.0,
        };
        let view = Area {
            x: 0.0,
            y: 0.0,
            width: 900.0,
            height: 600.0,
        };
        let tiles = visible_tiles(&page, &view, 2.0);
        // 1632 x 1056 device pixels of the page are visible: 4 x 3 tiles.
        assert_eq!(tiles.len(), 12);
        let last_column = tiles.iter().map(|tile| tile.x).max().unwrap();
        assert_eq!(last_column, 1536);
        assert!(tiles.iter().all(|tile| tile.x as u32 + tile.width <= 1632));
        assert!(
            visible_tiles(
                &page,
                &Area {
                    x: 0.0,
                    y: 5000.0,
                    width: 10.0,
                    height: 10.0
                },
                1.0
            )
            .is_empty()
        );
    }
}
