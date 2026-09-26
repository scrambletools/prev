//! Engine-neutral annotations and form fields: what prev reads from a page
//! and what it asks the engine to write. Coordinates are page points with
//! the origin at the top-left, as everywhere in `prev-pdf`.

use std::sync::Arc;

use crate::engine::Bitmap;
use crate::geometry::{Point, Quad, Rect};

/// An RGB color, each channel 0 to 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Rgb {
    pub const BLACK: Rgb = Rgb::new(0.0, 0.0, 0.0);
    pub const WHITE: Rgb = Rgb::new(1.0, 1.0, 1.0);

    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
        Self::new(
            f32::from(red) / 255.0,
            f32::from(green) / 255.0,
            f32::from(blue) / 255.0,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextMarkup {
    Highlight,
    Underline,
    StrikeOut,
    Squiggly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnd {
    None,
    OpenArrow,
    ClosedArrow,
    Circle,
    Square,
    Diamond,
    Butt,
    Slash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

/// The standard fonts a text box can use; every PDF viewer has them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Font {
    #[default]
    Helvetica,
    Times,
    Courier,
}

impl Font {
    /// The resource name MuPDF and other viewers use in `/DA`.
    pub fn resource_name(self) -> &'static str {
        match self {
            Font::Helvetica => "Helv",
            Font::Times => "TiRo",
            Font::Courier => "Cour",
        }
    }

    pub fn from_resource_name(name: &str) -> Self {
        match name {
            "TiRo" | "Times-Roman" | "Times" => Font::Times,
            "Cour" | "Courier" => Font::Courier,
            _ => Font::Helvetica,
        }
    }
}

/// What an annotation is and the geometry that goes with it.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// Highlight, underline, strikethrough or squiggly over text.
    Markup {
        style: TextMarkup,
        quads: Vec<Quad>,
    },
    /// Freehand strokes.
    Ink(Vec<Vec<Point>>),
    /// A rectangle filling `rect`.
    Square,
    /// An ellipse filling `rect`.
    Circle,
    Line {
        start: Point,
        end: Point,
        endings: (LineEnd, LineEnd),
    },
    /// A closed shape: stars, rounded rectangles and speech bubbles too.
    Polygon(Vec<Point>),
    PolyLine(Vec<Point>),
    /// A text box; its text is the annotation's contents.
    FreeText,
    /// A note, shown as an icon; its text is the annotation's contents.
    Note,
    /// An image or drawing with its own appearance: signatures, loupes
    /// and masks.
    Stamp,
    /// A type prev shows but does not edit.
    Other(String),
}

impl Kind {
    /// Whether prev can move, resize and restyle it.
    pub fn is_editable(&self) -> bool {
        !matches!(self, Kind::Other(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// Stroke, markup or text box border color. `None` draws no outline.
    pub color: Option<Rgb>,
    pub fill: Option<Rgb>,
    pub line_width: f32,
    pub dashed: bool,
    pub opacity: f32,
    pub font: Font,
    pub font_size: f32,
    pub text_color: Rgb,
    pub align: Align,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: Some(Rgb::new(0.9, 0.1, 0.1)),
            fill: None,
            line_width: 2.0,
            dashed: false,
            opacity: 1.0,
            font: Font::Helvetica,
            font_size: 14.0,
            text_color: Rgb::BLACK,
            align: Align::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// Unique within the document; stored as the PDF `/NM` entry.
    pub id: String,
    pub kind: Kind,
    pub rect: Rect,
    pub style: Style,
    pub contents: String,
    /// Stored as `/Subj`; prev names its shapes there ("Star",
    /// "Signature") so they read back as what the user drew.
    pub subject: Option<String>,
    pub author: Option<String>,
}

impl Annotation {
    pub fn new(id: impl Into<String>, kind: Kind, rect: Rect) -> Self {
        Self {
            id: id.into(),
            kind,
            rect,
            style: Style::default(),
            contents: String::new(),
            subject: None,
            author: None,
        }
    }

    /// The same annotation moved by `dx`, `dy` points.
    pub fn translated(&self, dx: f32, dy: f32) -> Self {
        self.mapped(|point| Point::new(point.x + dx, point.y + dy))
    }

    /// The same annotation stretched from its rect to `target`.
    pub fn resized(&self, target: Rect) -> Self {
        let from = self.rect;
        let scale_x = target.width() / from.width().max(0.01);
        let scale_y = target.height() / from.height().max(0.01);
        let mut resized = self.mapped(|point| {
            Point::new(
                target.x0 + (point.x - from.x0) * scale_x,
                target.y0 + (point.y - from.y0) * scale_y,
            )
        });
        resized.rect = target;
        resized
    }

    fn mapped(&self, map: impl Fn(Point) -> Point) -> Self {
        let map_rect = |rect: Rect| {
            let a = map(Point::new(rect.x0, rect.y0));
            let b = map(Point::new(rect.x1, rect.y1));
            Rect::new(a.x.min(b.x), a.y.min(b.y), a.x.max(b.x), a.y.max(b.y))
        };
        let kind = match &self.kind {
            Kind::Markup { style, quads } => Kind::Markup {
                style: *style,
                quads: quads
                    .iter()
                    .map(|quad| Quad {
                        ul: map(quad.ul),
                        ur: map(quad.ur),
                        ll: map(quad.ll),
                        lr: map(quad.lr),
                    })
                    .collect(),
            },
            Kind::Ink(strokes) => Kind::Ink(
                strokes
                    .iter()
                    .map(|stroke| stroke.iter().copied().map(&map).collect())
                    .collect(),
            ),
            Kind::Line {
                start,
                end,
                endings,
            } => Kind::Line {
                start: map(*start),
                end: map(*end),
                endings: *endings,
            },
            Kind::Polygon(points) => Kind::Polygon(points.iter().copied().map(&map).collect()),
            Kind::PolyLine(points) => Kind::PolyLine(points.iter().copied().map(&map).collect()),
            other => other.clone(),
        };
        Self {
            kind,
            rect: map_rect(self.rect),
            ..self.clone()
        }
    }
}

/// How a stamp is drawn, when prev creates or replaces its appearance.
#[derive(Debug, Clone, PartialEq)]
pub enum StampContent {
    /// An RGBA image filling the rect, clipped to an ellipse when `round`,
    /// with an optional border. Signatures and loupes.
    Image {
        image: Arc<Bitmap>,
        round: bool,
        border: Option<(Rgb, f32)>,
    },
    /// Darkens the whole page except a hole; the stamp's rect is the page
    /// and `hole` is the area left clear.
    Mask {
        hole: Rect,
        round: bool,
        opacity: f32,
    },
}

/// A deleted annotation, kept so the deletion can be undone exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Removed {
    pub(crate) object: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
    Text {
        multiline: bool,
        password: bool,
    },
    Checkbox,
    Radio,
    /// A drop-down menu (`combo`) or list, with its choices.
    Choice {
        options: Vec<String>,
        combo: bool,
    },
    Signature,
    Button,
}

/// A form field widget on a page.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The widget's object number, stable while the document is open.
    pub id: i32,
    pub name: String,
    pub kind: FieldKind,
    pub rect: Rect,
    pub value: String,
    /// The value that turns a checkbox or radio button on.
    pub on_value: Option<String>,
    pub read_only: bool,
    pub font_size: f32,
}

impl Field {
    pub fn is_on(&self) -> bool {
        self.on_value.as_deref() == Some(self.value.as_str())
    }
}

/// Makes a new annotation id: prev, the time and a counter, which is
/// unique enough within one document.
pub fn new_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("prev-{nanos:x}-{count}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resizing_moves_geometry_with_the_rect() {
        let ink = Annotation::new(
            "a",
            Kind::Ink(vec![vec![Point::new(10.0, 10.0), Point::new(20.0, 30.0)]]),
            Rect::new(10.0, 10.0, 20.0, 30.0),
        );
        let resized = ink.resized(Rect::new(100.0, 100.0, 120.0, 140.0));
        assert_eq!(
            resized.kind,
            Kind::Ink(vec![vec![
                Point::new(100.0, 100.0),
                Point::new(120.0, 140.0)
            ]])
        );
        let moved = ink.translated(5.0, -5.0);
        assert_eq!(moved.rect, Rect::new(15.0, 5.0, 25.0, 25.0));
    }

    #[test]
    fn ids_are_unique() {
        assert_ne!(new_id(), new_id());
    }
}
