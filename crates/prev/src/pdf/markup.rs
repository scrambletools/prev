//! Markup tools: what each tool makes, the shapes' geometry, sketch
//! recognition, and hit testing for selecting, moving and resizing.

use prev_pdf::annotation::{Annotation, Kind, LineEnd, Style, TextMarkup, new_id};
use prev_pdf::geometry::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rectangle,
    RoundedRectangle,
    Oval,
    Line,
    Arrow,
    Star,
    Polygon,
    SpeechBubble,
    Loupe,
    Mask,
}

impl Shape {
    pub const ALL: [Shape; 10] = [
        Shape::Rectangle,
        Shape::RoundedRectangle,
        Shape::Oval,
        Shape::Line,
        Shape::Arrow,
        Shape::Star,
        Shape::Polygon,
        Shape::SpeechBubble,
        Shape::Loupe,
        Shape::Mask,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Shape::Rectangle => "Rectangle",
            Shape::RoundedRectangle => "Rounded Rectangle",
            Shape::Oval => "Oval",
            Shape::Line => "Line",
            Shape::Arrow => "Arrow",
            Shape::Star => "Star",
            Shape::Polygon => "Polygon",
            Shape::SpeechBubble => "Speech Bubble",
            Shape::Loupe => "Loupe",
            Shape::Mask => "Mask",
        }
    }

    /// Whether dragging draws from one point to another rather than a box.
    pub fn is_line(self) -> bool {
        matches!(self, Shape::Line | Shape::Arrow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Selects text, annotations and form fields.
    Select,
    /// Selects an area of a page, to copy as an image.
    Area,
    Sketch,
    Draw,
    Shape(Shape),
    TextBox,
    Highlight(TextMarkup),
    Note,
    /// Marks areas for redaction.
    Redact,
}

impl Tool {
    /// Whether the tool stays on after one use, as Preview's highlight and
    /// drawing tools do.
    pub fn is_sticky(self) -> bool {
        matches!(
            self,
            Tool::Highlight(_) | Tool::Draw | Tool::Sketch | Tool::Area | Tool::Redact
        )
    }
}

/// Where on a selected annotation a drag started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handle {
    Body,
    /// A corner or edge of the bounding box: -1, 0 or 1 on each axis.
    Edge {
        x: i8,
        y: i8,
    },
    LineStart,
    LineEnd,
}

/// Extra reach around thin annotations and handles, in points.
pub const HIT_SLOP: f32 = 4.0;
/// Handle size on screen is fixed; in points it depends on the zoom.
pub const HANDLE_PIXELS: f32 = 8.0;
/// Size of a shape made by clicking without dragging.
pub const DEFAULT_SHAPE: f32 = 100.0;
pub const NOTE_SIZE: f32 = 24.0;

pub fn normalized(a: Point, b: Point) -> Rect {
    Rect::new(a.x.min(b.x), a.y.min(b.y), a.x.max(b.x), a.y.max(b.y))
}

fn expand(rect: Rect, by: f32) -> Rect {
    Rect::new(rect.x0 - by, rect.y0 - by, rect.x1 + by, rect.y1 + by)
}

fn distance_to_segment(point: Point, a: Point, b: Point) -> f32 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let length = dx * dx + dy * dy;
    let t = if length == 0.0 {
        0.0
    } else {
        (((point.x - a.x) * dx + (point.y - a.y) * dy) / length).clamp(0.0, 1.0)
    };
    let (x, y) = (a.x + t * dx, a.y + t * dy);
    ((point.x - x).powi(2) + (point.y - y).powi(2)).sqrt()
}

/// Whether `point` touches the annotation, with `slop` points of reach.
pub fn hits(annotation: &Annotation, point: Point, slop: f32) -> bool {
    if !expand(annotation.rect, slop).contains(point) {
        return false;
    }
    let reach = slop + annotation.style.line_width / 2.0;
    match &annotation.kind {
        Kind::Ink(strokes) => strokes.iter().any(|stroke| {
            stroke
                .windows(2)
                .any(|pair| distance_to_segment(point, pair[0], pair[1]) <= reach)
                || stroke.len() == 1 && distance_to_segment(point, stroke[0], stroke[0]) <= reach
        }),
        Kind::Line { start, end, .. } => distance_to_segment(point, *start, *end) <= reach,
        Kind::PolyLine(points) => points
            .windows(2)
            .any(|pair| distance_to_segment(point, pair[0], pair[1]) <= reach),
        // Filled or text-like annotations are hit anywhere inside.
        _ => true,
    }
}

/// The topmost annotation under `point`: the last one drawn. Masks cover
/// the page, so they are found separately with `hit_mask`.
pub fn hit_annotation(annotations: &[Annotation], point: Point, slop: f32) -> Option<&Annotation> {
    annotations
        .iter()
        .rev()
        .filter(|annotation| annotation.kind.is_editable() && !is_mask(annotation))
        .find(|annotation| hits(annotation, point, slop))
}

/// A mask on the page, which clicks on its darkened area select.
pub fn hit_mask(annotations: &[Annotation], point: Point) -> Option<&Annotation> {
    annotations
        .iter()
        .rev()
        .find(|annotation| is_mask(annotation) && annotation.rect.contains(point))
}

pub fn is_mask(annotation: &Annotation) -> bool {
    annotation.kind == Kind::Stamp && annotation.subject.as_deref() == Some("Mask")
}

/// The handle of a selected annotation under `point`. `handle` is the
/// handle size in points at the current zoom.
pub fn hit_handle(annotation: &Annotation, point: Point, handle: f32) -> Option<Handle> {
    let near = |target: Point| {
        (point.x - target.x).abs() <= handle && (point.y - target.y).abs() <= handle
    };
    if let Kind::Line { start, end, .. } = annotation.kind {
        if near(start) {
            return Some(Handle::LineStart);
        }
        if near(end) {
            return Some(Handle::LineEnd);
        }
        return hits(annotation, point, handle / 2.0).then_some(Handle::Body);
    }
    if resizable(annotation) {
        for (x, y) in handle_positions() {
            if near(handle_point(annotation.rect, x, y)) {
                return Some(Handle::Edge { x, y });
            }
        }
    }
    expand(annotation.rect, handle / 2.0)
        .contains(point)
        .then_some(Handle::Body)
}

/// Notes and text markup keep their size.
pub fn resizable(annotation: &Annotation) -> bool {
    !matches!(annotation.kind, Kind::Note | Kind::Markup { .. }) && !is_mask(annotation)
}

pub fn movable(annotation: &Annotation) -> bool {
    !matches!(annotation.kind, Kind::Markup { .. }) && !is_mask(annotation)
}

/// Corner and edge handle positions, as -1/0/1 offsets.
pub fn handle_positions() -> [(i8, i8); 8] {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ]
}

pub fn handle_point(rect: Rect, x: i8, y: i8) -> Point {
    let pick = |low: f32, high: f32, side: i8| match side {
        -1 => low,
        1 => high,
        _ => (low + high) / 2.0,
    };
    Point::new(pick(rect.x0, rect.x1, x), pick(rect.y0, rect.y1, y))
}

/// The annotation after dragging `handle` from `from` to `to`. `keep_ratio`
/// keeps the aspect ratio when resizing, as Shift does in Preview.
pub fn dragged(
    annotation: &Annotation,
    handle: Handle,
    from: Point,
    to: Point,
    keep_ratio: bool,
) -> Annotation {
    let (dx, dy) = (to.x - from.x, to.y - from.y);
    match handle {
        Handle::Body => annotation.translated(dx, dy),
        Handle::LineStart | Handle::LineEnd => {
            let Kind::Line {
                start,
                end,
                endings,
            } = annotation.kind
            else {
                return annotation.clone();
            };
            let (start, end) = if handle == Handle::LineStart {
                (Point::new(start.x + dx, start.y + dy), end)
            } else {
                (start, Point::new(end.x + dx, end.y + dy))
            };
            let mut line = annotation.clone();
            line.kind = Kind::Line {
                start,
                end,
                endings,
            };
            line.rect = normalized(start, end);
            line
        }
        Handle::Edge { x, y } => {
            let rect = annotation.rect;
            let mut target = rect;
            match x {
                -1 => target.x0 = (rect.x0 + dx).min(rect.x1 - 4.0),
                1 => target.x1 = (rect.x1 + dx).max(rect.x0 + 4.0),
                _ => {}
            }
            match y {
                -1 => target.y0 = (rect.y0 + dy).min(rect.y1 - 4.0),
                1 => target.y1 = (rect.y1 + dy).max(rect.y0 + 4.0),
                _ => {}
            }
            if keep_ratio && x != 0 && y != 0 && rect.height() > 0.0 {
                let ratio = rect.width() / rect.height();
                let height = target.width() / ratio;
                if y == -1 {
                    target.y0 = target.y1 - height;
                } else {
                    target.y1 = target.y0 + height;
                }
            }
            annotation.resized(target)
        }
    }
}

/// Points of a regular star with `points` tips inside `rect`.
pub fn star(rect: Rect, points: usize) -> Vec<Point> {
    let center = rect.center();
    let (rx, ry) = (rect.width() / 2.0, rect.height() / 2.0);
    (0..points * 2)
        .map(|index| {
            let angle =
                std::f32::consts::PI * index as f32 / points as f32 - std::f32::consts::FRAC_PI_2;
            let scale = if index % 2 == 0 { 1.0 } else { 0.4 };
            Point::new(
                center.x + rx * scale * angle.cos(),
                center.y + ry * scale * angle.sin(),
            )
        })
        .collect()
}

/// A regular polygon with `sides` inside `rect`, point up.
pub fn regular_polygon(rect: Rect, sides: usize) -> Vec<Point> {
    let center = rect.center();
    let (rx, ry) = (rect.width() / 2.0, rect.height() / 2.0);
    (0..sides)
        .map(|index| {
            let angle =
                std::f32::consts::TAU * index as f32 / sides as f32 - std::f32::consts::FRAC_PI_2;
            Point::new(center.x + rx * angle.cos(), center.y + ry * angle.sin())
        })
        .collect()
}

/// Quarter circle arc points from `start` angle, around `center`.
fn arc(center: Point, radius: f32, start: f32, out: &mut Vec<Point>) {
    const STEPS: usize = 6;
    for step in 0..=STEPS {
        let angle = start + std::f32::consts::FRAC_PI_2 * step as f32 / STEPS as f32;
        out.push(Point::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
}

/// A rounded rectangle as a polygon, since PDF squares have no corner
/// radius.
pub fn rounded_rectangle(rect: Rect) -> Vec<Point> {
    let radius = (rect.width().min(rect.height()) * 0.2).min(24.0);
    let mut points = Vec::new();
    use std::f32::consts::{FRAC_PI_2, PI};
    arc(
        Point::new(rect.x1 - radius, rect.y0 + radius),
        radius,
        -FRAC_PI_2,
        &mut points,
    );
    arc(
        Point::new(rect.x1 - radius, rect.y1 - radius),
        radius,
        0.0,
        &mut points,
    );
    arc(
        Point::new(rect.x0 + radius, rect.y1 - radius),
        radius,
        FRAC_PI_2,
        &mut points,
    );
    arc(
        Point::new(rect.x0 + radius, rect.y0 + radius),
        radius,
        PI,
        &mut points,
    );
    points
}

/// A rounded speech bubble with a tail at the bottom left.
pub fn speech_bubble(rect: Rect) -> Vec<Point> {
    let body = Rect::new(rect.x0, rect.y0, rect.x1, rect.y1 - rect.height() * 0.25);
    let mut points = rounded_rectangle(body);
    // The bottom edge runs between the third and fourth arcs; put the tail
    // there, pointing down and left.
    let tail_start = Point::new(body.x0 + body.width() * 0.35, body.y1);
    let tail_tip = Point::new(body.x0 + body.width() * 0.15, rect.y1);
    let tail_end = Point::new(body.x0 + body.width() * 0.2, body.y1);
    let index = points.len() / 2;
    points.splice(index..index, [tail_start, tail_tip, tail_end]);
    points
}

/// A new annotation for `shape` filling `rect`, or from `start` to `end`
/// for lines. Loupes and masks become stamps whose content the viewer
/// adds.
pub fn shape_annotation(shape: Shape, start: Point, end: Point, style: Style) -> Annotation {
    let rect = normalized(start, end);
    let kind = match shape {
        Shape::Rectangle => Kind::Square,
        Shape::Oval => Kind::Circle,
        Shape::RoundedRectangle => Kind::Polygon(rounded_rectangle(rect)),
        Shape::Star => Kind::Polygon(star(rect, 5)),
        Shape::Polygon => Kind::Polygon(regular_polygon(rect, 6)),
        Shape::SpeechBubble => Kind::Polygon(speech_bubble(rect)),
        Shape::Line | Shape::Arrow => Kind::Line {
            start,
            end,
            endings: (
                LineEnd::None,
                if shape == Shape::Arrow {
                    LineEnd::ClosedArrow
                } else {
                    LineEnd::None
                },
            ),
        },
        Shape::Loupe | Shape::Mask => Kind::Stamp,
    };
    let mut annotation = Annotation::new(new_id(), kind, rect);
    annotation.style = style;
    if shape.is_line() && annotation.style.fill.is_none() {
        // Arrow heads are filled with the line color.
        annotation.style.fill = annotation.style.color;
    }
    annotation.subject = Some(shape.label().to_owned());
    annotation
}

/// Drops points closer than `min_gap` to the previous one, so strokes stay
/// small without changing their look.
pub fn thin(points: &[Point], min_gap: f32) -> Vec<Point> {
    let mut kept: Vec<Point> = Vec::with_capacity(points.len());
    for point in points {
        match kept.last() {
            Some(last) if (point.x - last.x).hypot(point.y - last.y) < min_gap => {}
            _ => kept.push(*point),
        }
    }
    if let (Some(last), Some(end)) = (kept.last().copied(), points.last())
        && last != *end
    {
        kept.push(*end);
    }
    kept
}

pub fn bounds(points: &[Point]) -> Rect {
    points.iter().fold(
        Rect::new(
            f32::INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NEG_INFINITY,
        ),
        |rect, point| {
            Rect::new(
                rect.x0.min(point.x),
                rect.y0.min(point.y),
                rect.x1.max(point.x),
                rect.y1.max(point.y),
            )
        },
    )
}

/// What a sketched stroke looks like, as Preview's sketch tool recognizes
/// shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sketched {
    Line,
    Rectangle,
    Oval,
    Freehand,
}

pub fn recognize(points: &[Point]) -> Sketched {
    if points.len() < 3 {
        return Sketched::Freehand;
    }
    let first = points[0];
    let last = points[points.len() - 1];
    let path_length: f32 = points
        .windows(2)
        .map(|pair| (pair[1].x - pair[0].x).hypot(pair[1].y - pair[0].y))
        .sum();
    let span = (last.x - first.x).hypot(last.y - first.y);
    if path_length < 12.0 {
        return Sketched::Freehand;
    }
    // Nearly straight: the ends are as far apart as the path is long.
    if span / path_length > 0.95 {
        return Sketched::Line;
    }
    let rect = bounds(points);
    let size = rect.width().max(rect.height());
    let closed = span < size * 0.25;
    if !closed || rect.width() < 10.0 || rect.height() < 10.0 {
        return Sketched::Freehand;
    }
    // How far points sit from the box edges, and from the inscribed
    // ellipse, relative to the size.
    let center = rect.center();
    let (rx, ry) = (rect.width() / 2.0, rect.height() / 2.0);
    let mut box_error = 0.0;
    let mut ellipse_error = 0.0;
    for point in points {
        let to_edge = (point.x - rect.x0)
            .abs()
            .min((point.x - rect.x1).abs())
            .min((point.y - rect.y0).abs())
            .min((point.y - rect.y1).abs());
        box_error += to_edge / size;
        let normalized = ((point.x - center.x) / rx).hypot((point.y - center.y) / ry);
        ellipse_error += (normalized - 1.0).abs() * rx.min(ry) / size;
    }
    let count = points.len() as f32;
    let (box_error, ellipse_error) = (box_error / count, ellipse_error / count);
    if box_error < ellipse_error && box_error < 0.06 {
        Sketched::Rectangle
    } else if ellipse_error < 0.08 {
        Sketched::Oval
    } else {
        Sketched::Freehand
    }
}

/// The annotation a finished stroke becomes with the draw or sketch tool.
/// A freehand stroke evened out and made dense: pointer positions come
/// rounded and far apart, which drawn as they are gives a jagged line.
/// Each point moves toward its neighbours (weights 1, 2, 1), twice, then
/// quadratic curves through the midpoints are sampled, so the ink is drawn
/// as a smooth curve. The ends stay where they are.
pub fn smooth(points: &[Point]) -> Vec<Point> {
    let mut points = points.to_vec();
    for _ in 0..2 {
        if points.len() < 3 {
            break;
        }
        let mut next = points.clone();
        for index in 1..points.len() - 1 {
            let (a, b, c) = (points[index - 1], points[index], points[index + 1]);
            next[index] = Point::new((a.x + 2.0 * b.x + c.x) / 4.0, (a.y + 2.0 * b.y + c.y) / 4.0);
        }
        points = next;
    }
    if points.len() < 3 {
        return points;
    }
    const STEPS: usize = 4;
    let middle = |a: Point, b: Point| Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
    let mut dense = vec![points[0]];
    let mut start = points[0];
    for index in 1..points.len() - 1 {
        let control = points[index];
        let end = middle(control, points[index + 1]);
        for step in 1..=STEPS {
            let t = step as f32 / STEPS as f32;
            let u = 1.0 - t;
            dense.push(Point::new(
                u * u * start.x + 2.0 * u * t * control.x + t * t * end.x,
                u * u * start.y + 2.0 * u * t * control.y + t * t * end.y,
            ));
        }
        start = end;
    }
    dense.push(points[points.len() - 1]);
    dense
}

pub fn stroke_annotation(points: &[Point], sketch: bool, style: Style) -> Option<Annotation> {
    let points = thin(points, 0.75);
    if points.is_empty() {
        return None;
    }
    let rect = bounds(&points);
    let recognized = if sketch {
        recognize(&points)
    } else {
        Sketched::Freehand
    };
    let kind = match recognized {
        Sketched::Line => Kind::Line {
            start: points[0],
            end: points[points.len() - 1],
            endings: (LineEnd::None, LineEnd::None),
        },
        Sketched::Rectangle => Kind::Square,
        Sketched::Oval => Kind::Circle,
        Sketched::Freehand => Kind::Ink(vec![smooth(&points)]),
    };
    let rect = match &kind {
        Kind::Line { start, end, .. } => normalized(*start, *end),
        Kind::Ink(strokes) => bounds(&strokes[0]),
        _ => rect,
    };
    let mut annotation = Annotation::new(new_id(), kind, rect);
    annotation.style = style;
    annotation.style.fill = None;
    Some(annotation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoothing_evens_out_steps_and_keeps_the_ends() {
        // A staircase, as a pointer reports a diagonal.
        let stairs: Vec<Point> = (0..40)
            .map(|index| {
                Point::new(
                    (index / 2) as f32 * 2.0 + (index % 2) as f32 * 2.0,
                    (index / 2) as f32 * 2.0,
                )
            })
            .collect();
        let smooth = smooth(&stairs);
        assert_eq!(smooth.first(), stairs.first());
        assert_eq!(smooth.last(), stairs.last());
        assert!(smooth.len() > stairs.len() * 3, "made dense");
        // Away from the ends, the line is close to the diagonal y = x - 1.
        for point in &smooth[20..smooth.len() - 20] {
            assert!((point.y - (point.x - 1.0)).abs() < 1.0, "{point:?}");
        }
    }

    fn circle_points(center: Point, radius: f32) -> Vec<Point> {
        (0..=64)
            .map(|index| {
                let angle = std::f32::consts::TAU * index as f32 / 64.0;
                Point::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                )
            })
            .collect()
    }

    fn rectangle_points(rect: Rect) -> Vec<Point> {
        let corners = [
            Point::new(rect.x0, rect.y0),
            Point::new(rect.x1, rect.y0),
            Point::new(rect.x1, rect.y1),
            Point::new(rect.x0, rect.y1),
            Point::new(rect.x0, rect.y0),
        ];
        corners
            .windows(2)
            .flat_map(|pair| {
                (0..16).map(move |step| {
                    let t = step as f32 / 16.0;
                    Point::new(
                        pair[0].x + (pair[1].x - pair[0].x) * t,
                        pair[0].y + (pair[1].y - pair[0].y) * t,
                    )
                })
            })
            .collect()
    }

    #[test]
    fn recognizes_sketched_shapes() {
        assert_eq!(
            recognize(&circle_points(Point::new(100.0, 100.0), 40.0)),
            Sketched::Oval
        );
        assert_eq!(
            recognize(&rectangle_points(Rect::new(10.0, 10.0, 110.0, 70.0))),
            Sketched::Rectangle
        );
        let line: Vec<Point> = (0..20)
            .map(|index| Point::new(index as f32 * 5.0, index as f32 * 2.0 + 0.3))
            .collect();
        assert_eq!(recognize(&line), Sketched::Line);
        let zigzag: Vec<Point> = (0..20)
            .map(|index| Point::new(index as f32 * 5.0, if index % 2 == 0 { 0.0 } else { 30.0 }))
            .collect();
        assert_eq!(recognize(&zigzag), Sketched::Freehand);
    }

    #[test]
    fn stars_fit_their_box() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let points = star(rect, 5);
        assert_eq!(points.len(), 10);
        let fitted = bounds(&points);
        assert!((fitted.y0 - 0.0).abs() < 0.01, "top tip touches the top");
        assert!(fitted.x0 >= -0.01 && fitted.x1 <= 100.01);
    }

    #[test]
    fn hit_testing_follows_strokes() {
        let mut ink = Annotation::new(
            "ink",
            Kind::Ink(vec![vec![Point::new(0.0, 0.0), Point::new(100.0, 100.0)]]),
            Rect::new(0.0, 0.0, 100.0, 100.0),
        );
        ink.style.line_width = 2.0;
        assert!(hits(&ink, Point::new(50.0, 51.0), HIT_SLOP));
        assert!(!hits(&ink, Point::new(90.0, 10.0), HIT_SLOP));
        let square = Annotation::new("sq", Kind::Square, Rect::new(0.0, 0.0, 50.0, 50.0));
        assert!(hits(&square, Point::new(25.0, 25.0), HIT_SLOP));
    }

    #[test]
    fn corner_drags_resize_and_keep_ratio() {
        let square = Annotation::new("sq", Kind::Square, Rect::new(0.0, 0.0, 100.0, 50.0));
        let resized = dragged(
            &square,
            Handle::Edge { x: 1, y: 1 },
            Point::new(100.0, 50.0),
            Point::new(200.0, 60.0),
            true,
        );
        assert_eq!(resized.rect, Rect::new(0.0, 0.0, 200.0, 100.0));
        let moved = dragged(
            &square,
            Handle::Body,
            Point::new(10.0, 10.0),
            Point::new(20.0, 30.0),
            false,
        );
        assert_eq!(moved.rect, Rect::new(10.0, 20.0, 110.0, 70.0));
    }
}
