//! Draws annotations being made or moved, and the selection handles, on
//! top of the rendered page. The page itself shows saved annotations.

use iced::widget::canvas::{Frame, LineCap, LineDash, LineJoin, Path, Stroke};
use iced::{Color, Size};
use prev_pdf::annotation::{Annotation, Kind, LineEnd, Rgb, TextMarkup};
use prev_pdf::geometry::{Point, Rect};

use super::markup::{self, handle_positions};

/// Maps page points to frame coordinates.
pub struct Mapping {
    /// Frame position of the page's top-left corner.
    pub origin: iced::Point,
    /// Logical pixels per point.
    pub scale: f32,
}

impl Mapping {
    pub fn point(&self, point: Point) -> iced::Point {
        iced::Point::new(
            self.origin.x + point.x * self.scale,
            self.origin.y + point.y * self.scale,
        )
    }

    fn rect(&self, rect: Rect) -> (iced::Point, Size) {
        (
            self.point(Point::new(rect.x0, rect.y0)),
            Size::new(rect.width() * self.scale, rect.height() * self.scale),
        )
    }
}

pub fn color(rgb: Rgb, alpha: f32) -> Color {
    Color::from_rgba(rgb.red, rgb.green, rgb.blue, alpha)
}

fn polyline(points: &[Point], mapping: &Mapping, closed: bool) -> Path {
    Path::new(|builder| {
        let mut points = points.iter();
        if let Some(first) = points.next() {
            builder.move_to(mapping.point(*first));
            for point in points {
                builder.line_to(mapping.point(*point));
            }
            if closed {
                builder.close();
            }
        }
    })
}

fn ellipse_points(rect: Rect) -> Vec<Point> {
    let center = rect.center();
    let (rx, ry) = (rect.width() / 2.0, rect.height() / 2.0);
    (0..48)
        .map(|index| {
            let angle = std::f32::consts::TAU * index as f32 / 48.0;
            Point::new(center.x + rx * angle.cos(), center.y + ry * angle.sin())
        })
        .collect()
}

fn rect_points(rect: Rect) -> Vec<Point> {
    vec![
        Point::new(rect.x0, rect.y0),
        Point::new(rect.x1, rect.y0),
        Point::new(rect.x1, rect.y1),
        Point::new(rect.x0, rect.y1),
    ]
}

/// The triangle of a closed arrow head at `tip`, pointing away from `from`.
fn arrow_head(from: Point, tip: Point, size: f32) -> Vec<Point> {
    let angle = (tip.y - from.y).atan2(tip.x - from.x);
    let spread = 0.45;
    let back = |offset: f32| {
        Point::new(
            tip.x - size * (angle + offset).cos(),
            tip.y - size * (angle + offset).sin(),
        )
    };
    vec![tip, back(spread), back(-spread)]
}

/// Draws an annotation roughly as MuPDF will render it.
pub fn paint(frame: &mut Frame, annotation: &Annotation, mapping: &Mapping) {
    let style = &annotation.style;
    let alpha = style.opacity.clamp(0.1, 1.0);
    let width = (style.line_width * mapping.scale).max(1.0);
    let dash = [6.0 * mapping.scale.max(0.5), 4.0 * mapping.scale.max(0.5)];
    let stroke = |rgb: Rgb| Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        line_dash: if style.dashed {
            LineDash {
                segments: &dash,
                offset: 0,
            }
        } else {
            LineDash::default()
        },
        ..Stroke::default().with_color(color(rgb, alpha))
    };
    let shape = |frame: &mut Frame, path: Path| {
        if let Some(fill) = style.fill {
            frame.fill(&path, color(fill, alpha));
        }
        if let Some(line) = style.color {
            frame.stroke(&path, stroke(line));
        }
    };
    match &annotation.kind {
        Kind::Square => shape(
            frame,
            polyline(&rect_points(annotation.rect), mapping, true),
        ),
        Kind::Redact => {
            let (origin, size) = mapping.rect(annotation.rect);
            frame.fill_rectangle(origin, size, Color::from_rgba(0.0, 0.0, 0.0, 0.35));
            frame.stroke(
                &Path::rectangle(origin, size),
                Stroke::default()
                    .with_color(Color::from_rgb(0.85, 0.1, 0.1))
                    .with_width(1.5),
            );
        }
        Kind::Circle => shape(
            frame,
            polyline(&ellipse_points(annotation.rect), mapping, true),
        ),
        Kind::Polygon(points) => shape(frame, polyline(points, mapping, true)),
        Kind::PolyLine(points) => {
            if let Some(line) = style.color {
                frame.stroke(&polyline(points, mapping, false), stroke(line));
            }
        }
        Kind::Ink(strokes) => {
            let line = style.color.unwrap_or(Rgb::BLACK);
            for points in strokes {
                frame.stroke(&polyline(points, mapping, false), stroke(line));
            }
        }
        Kind::Line {
            start,
            end,
            endings,
        } => {
            let line = style.color.unwrap_or(Rgb::BLACK);
            frame.stroke(&polyline(&[*start, *end], mapping, false), stroke(line));
            let head = (style.line_width * 4.0).max(8.0);
            for (tip, from, ending) in [(*end, *start, endings.1), (*start, *end, endings.0)] {
                match ending {
                    LineEnd::ClosedArrow => frame.fill(
                        &polyline(&arrow_head(from, tip, head), mapping, true),
                        color(line, alpha),
                    ),
                    LineEnd::OpenArrow => {
                        let points = arrow_head(from, tip, head);
                        frame.stroke(
                            &polyline(&[points[1], points[0], points[2]], mapping, false),
                            stroke(line),
                        );
                    }
                    _ => {}
                }
            }
        }
        Kind::Markup {
            style: markup,
            quads,
        } => {
            let rgb = style.color.unwrap_or(Rgb::new(1.0, 0.86, 0.2));
            for quad in quads {
                let bounds = quad.bounds();
                match markup {
                    TextMarkup::Highlight => {
                        let (origin, size) = mapping.rect(bounds);
                        frame.fill_rectangle(origin, size, color(rgb, 0.45));
                    }
                    _ => {
                        let y = match markup {
                            TextMarkup::StrikeOut => (bounds.y0 + bounds.y1) / 2.0,
                            _ => bounds.y1 - 1.0,
                        };
                        frame.stroke(
                            &polyline(
                                &[Point::new(bounds.x0, y), Point::new(bounds.x1, y)],
                                mapping,
                                false,
                            ),
                            Stroke::default()
                                .with_color(color(rgb, 1.0))
                                .with_width((1.5 * mapping.scale).max(1.0)),
                        );
                    }
                }
            }
        }
        Kind::FreeText | Kind::Note | Kind::Stamp | Kind::Other(_) => {
            let (origin, size) = mapping.rect(annotation.rect);
            frame.stroke(
                &Path::rectangle(origin, size),
                Stroke {
                    line_dash: LineDash {
                        segments: &dash,
                        offset: 0,
                    },
                    ..Stroke::default()
                        .with_color(Color::from_rgba(0.3, 0.3, 0.3, 0.8))
                        .with_width(1.0)
                },
            );
        }
    }
}

/// The rectangular selection: a dashed outline with the rest of the page
/// dimmed a little.
pub fn area(frame: &mut Frame, area: Rect, mapping: &Mapping, accent: Color) {
    let (origin, size) = mapping.rect(area);
    frame.fill_rectangle(origin, size, Color { a: 0.08, ..accent });
    let dash = [6.0, 4.0];
    frame.stroke(
        &Path::rectangle(origin, size),
        Stroke {
            line_dash: LineDash {
                segments: &dash,
                offset: 0,
            },
            ..Stroke::default().with_color(accent).with_width(1.5)
        },
    );
}

/// The selection of `annotation`, the same in every theme: a 1 pixel box
/// in half-transparent blue and square handles, 7 pixels across, white
/// with a black border, centred on the box's line.
/// Everything sits on whole screen pixels, so nothing is blurred by
/// anti-aliasing. A line annotation has only its two end handles, and one
/// that cannot be resized only the box. `device_scale` is the window's
/// pixels per logical pixel.
pub fn selection(frame: &mut Frame, annotation: &Annotation, mapping: &Mapping, device_scale: f32) {
    let scale = device_scale.max(0.01);
    // One logical pixel, as a whole number of screen pixels.
    let one = scale.round().max(1.0) / scale;
    let snap = |value: f32| (value * scale).floor() / scale;
    let handle = |frame: &mut Frame, x: f32, y: f32| {
        let corner = iced::Point::new(x - 3.0 * one, y - 3.0 * one);
        frame.fill_rectangle(corner, Size::new(7.0 * one, 7.0 * one), Color::BLACK);
        frame.fill_rectangle(
            iced::Point::new(corner.x + one, corner.y + one),
            Size::new(5.0 * one, 5.0 * one),
            Color::WHITE,
        );
    };
    if let Kind::Line { start, end, .. } = annotation.kind {
        for point in [start, end] {
            let point = mapping.point(point);
            handle(frame, snap(point.x), snap(point.y));
        }
        return;
    }
    let (origin, size) = mapping.rect(annotation.rect);
    let (left, top) = (snap(origin.x), snap(origin.y));
    let (right, bottom) = (snap(origin.x + size.width), snap(origin.y + size.height));
    // Top and bottom run the full width; the sides fill in between, so no
    // pixel of the half-transparent line is painted twice.
    let width = right - left + one;
    let side = (bottom - top - one).max(0.0);
    for (x, y, w, h) in [
        (left, top, width, one),
        (left, bottom, width, one),
        (left, top + one, one, side),
        (right, top + one, one, side),
    ] {
        frame.fill_rectangle(iced::Point::new(x, y), Size::new(w, h), SELECTION_LINE);
    }
    if markup::resizable(annotation) {
        let middle_x = snap((left + right) / 2.0);
        let middle_y = snap((top + bottom) / 2.0);
        for (x, y) in handle_positions() {
            let pick = |side: i8, low: f32, middle: f32, high: f32| match side {
                -1 => low,
                1 => high,
                _ => middle,
            };
            handle(
                frame,
                pick(x, left, middle_x, right),
                pick(y, top, middle_y, bottom),
            );
        }
    }
}

/// The selection line: a blue at half opacity, #9eb3fd over white and
/// #1f337e over black.
const SELECTION_LINE: Color = Color::from_rgba(61.0 / 255.0, 103.0 / 255.0, 251.0 / 255.0, 0.5);
