//! The engine-neutral PDF interface. The UI only talks to these traits, so a
//! second engine can be added without touching it.

use std::path::Path;
use std::sync::Arc;

use crate::annotation::{Annotation, Field, Removed, StampContent};
use crate::geometry::{PixelRect, Point, Quad, Rect, Size};
use crate::text::TextLayout;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// The file could not be read or parsed.
    Open(String),
    /// A page or operation failed.
    Engine(String),
    PageOutOfRange(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(message) => write!(formatter, "cannot open document: {message}"),
            Self::Engine(message) => formatter.write_str(message),
            Self::PageOutOfRange(index) => write!(formatter, "page {} does not exist", index + 1),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Creates documents. Shared between threads.
pub trait Engine: Send + Sync + 'static {
    fn open(&self, path: &Path) -> Result<Box<dyn Document>>;
}

/// An open document. Not `Send`: it lives on its document thread.
pub trait Document {
    fn needs_password(&self) -> bool;
    /// Returns whether the password was accepted.
    fn authenticate(&mut self, password: &str) -> bool;
    fn page_count(&self) -> Result<usize>;
    /// Page size in points after the page's own rotation.
    fn page_size(&self, index: usize) -> Result<Size>;
    /// Every page's size; engines can do this faster than page by page.
    fn page_sizes(&self) -> Result<Vec<Size>> {
        (0..self.page_count()?)
            .map(|index| self.page_size(index))
            .collect()
    }
    fn page_label(&self, index: usize) -> Option<String>;
    fn title(&self) -> Option<String>;
    fn outline(&self) -> Result<Vec<OutlineItem>>;
    fn links(&self, index: usize) -> Result<Vec<Link>>;
    /// A parsed page that can be rendered on any thread, any number of times.
    fn display(&self, index: usize) -> Result<Arc<dyn PageDisplay>>;

    /// Annotations on a page, without links, popups and form widgets.
    fn annotations(&self, page: usize) -> Result<Vec<Annotation>>;
    /// Adds `annotation` with its id; stamps take their appearance from
    /// `content`.
    fn add_annotation(
        &mut self,
        page: usize,
        annotation: &Annotation,
        content: Option<&StampContent>,
    ) -> Result<()>;
    /// Changes the annotation with `annotation.id` to match it. A stamp's
    /// appearance is replaced only when `content` is given.
    fn update_annotation(
        &mut self,
        page: usize,
        annotation: &Annotation,
        content: Option<&StampContent>,
    ) -> Result<()>;
    fn remove_annotation(&mut self, page: usize, id: &str) -> Result<Removed>;
    /// Puts a removed annotation back, exactly as it was.
    fn restore_annotation(&mut self, page: usize, removed: &Removed) -> Result<()>;
    fn fields(&self, page: usize) -> Result<Vec<Field>>;
    fn set_field(&mut self, page: usize, id: i32, value: &str) -> Result<()>;
    fn has_changes(&self) -> bool;
    /// The document with its changes, appended to the original file when
    /// possible so earlier signatures and revisions stay intact.
    fn save(&mut self) -> Result<Vec<u8>>;
}

/// A parsed page, safe to share with render threads.
pub trait PageDisplay: Send + Sync {
    /// Page bounds in points; the origin is the page's top-left.
    fn size(&self) -> Size;
    /// Renders `area` of the page at `scale` device pixels per point.
    fn render(&self, scale: f32, area: PixelRect) -> Result<Bitmap>;
    fn text(&self) -> Result<TextLayout>;
    /// Every match of `needle`, ignoring case, in reading order.
    fn search(&self, needle: &str) -> Result<Vec<Quad>>;
}

/// An RGBA image, 8 bits per channel, not premultiplied.
#[derive(Clone, PartialEq, Eq)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Bitmap({}x{})", self.width, self.height)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutlineItem {
    pub title: String,
    pub target: Option<LinkTarget>,
    pub children: Vec<OutlineItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub bounds: Rect,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkTarget {
    /// A place in this document; `point` is the top-left to show, if given.
    Page {
        index: usize,
        point: Option<Point>,
    },
    Uri(String),
}

/// Scales pages to device pixels per point for a zoom level, where 1.0 is
/// "actual size" at 96 dpi.
pub fn zoom_to_scale(zoom: f32) -> f32 {
    zoom * 96.0 / 72.0
}

/// The page area in device pixels at `scale`.
pub fn page_pixels(size: Size, scale: f32) -> (u32, u32) {
    (
        (size.width * scale).ceil().max(1.0) as u32,
        (size.height * scale).ceil().max(1.0) as u32,
    )
}
