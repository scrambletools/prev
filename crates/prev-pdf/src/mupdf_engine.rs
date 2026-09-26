//! `Engine` implementation backed by MuPDF.

use std::path::Path;
use std::sync::Arc;

use mupdf::pdf::{PdfDocument, PdfObject};
use mupdf::text_page::{TextBlockType, TextPageFlags};
use mupdf::{
    Colorspace, DestinationKind, Device, DisplayList, IRect, Matrix, MetadataName, Pixmap,
};

use crate::engine::{
    Bitmap, Document, Engine, Error, Link, LinkTarget, OutlineItem, PageDisplay, Result,
};
use crate::geometry::{PixelRect, Point, Quad, Rect, Size};
use crate::text::{TextChar, TextLayout, TextLine};

const MAX_SEARCH_HITS: u32 = 10_000;

fn engine_error(error: mupdf::Error) -> Error {
    Error::Engine(error.to_string())
}

pub struct MupdfEngine;

impl Engine for MupdfEngine {
    fn open(&self, path: &Path) -> Result<Box<dyn Document>> {
        let document =
            PdfDocument::open(path.as_os_str()).map_err(|error| Error::Open(error.to_string()))?;
        Ok(Box::new(MupdfDocument { document }))
    }
}

struct MupdfDocument {
    document: PdfDocument,
}

impl MupdfDocument {
    fn load_page(&self, index: usize) -> Result<mupdf::Page> {
        if index >= self.page_count()? {
            return Err(Error::PageOutOfRange(index));
        }
        self.document.load_page(index as i32).map_err(engine_error)
    }

    /// Converts a destination to a target in top-left page space. MuPDF
    /// already moves each page's crop box origin to (0, 0), so destination
    /// coordinates need no per-page offset.
    fn target(&self, dest: Option<mupdf::link::LinkDestination>, uri: &str) -> Option<LinkTarget> {
        match dest {
            Some(dest) => {
                let index = dest.loc.page_number as usize;
                let point = match dest.kind {
                    DestinationKind::XYZ { left, top, .. } => {
                        Some(Point::new(left.unwrap_or(0.0), top.unwrap_or(0.0)))
                    }
                    DestinationKind::FitH { top: Some(top) }
                    | DestinationKind::FitBH { top: Some(top) } => Some(Point::new(0.0, top)),
                    _ => None,
                };
                Some(LinkTarget::Page { index, point })
            }
            None if !uri.is_empty() => Some(LinkTarget::Uri(uri.to_owned())),
            None => None,
        }
    }

    fn outline_items(&self, items: Vec<mupdf::Outline>) -> Vec<OutlineItem> {
        items
            .into_iter()
            .map(|item| OutlineItem {
                target: self.target(item.dest, item.uri.as_deref().unwrap_or_default()),
                title: item.title,
                children: self.outline_items(item.down),
            })
            .collect()
    }
}

impl Document for MupdfDocument {
    fn needs_password(&self) -> bool {
        self.document.needs_password().unwrap_or(false)
    }

    fn authenticate(&mut self, password: &str) -> bool {
        self.document.authenticate(password).unwrap_or(false)
    }

    fn page_count(&self) -> Result<usize> {
        self.document
            .page_count()
            .map(|count| count.max(0) as usize)
            .map_err(engine_error)
    }

    fn page_size(&self, index: usize) -> Result<Size> {
        let bounds = self.load_page(index)?.bounds().map_err(engine_error)?;
        Ok(Size::new(bounds.width(), bounds.height()))
    }

    /// Reads sizes from the page dictionaries instead of loading each page,
    /// which matters for documents with thousands of pages.
    fn page_sizes(&self) -> Result<Vec<Size>> {
        (0..self.page_count()?)
            .map(|index| {
                let fast = self
                    .document
                    .find_page(index as i32)
                    .ok()
                    .and_then(|page| page_dictionary_size(&page));
                match fast {
                    Some(size) => Ok(size),
                    None => self.page_size(index),
                }
            })
            .collect()
    }

    fn page_label(&self, index: usize) -> Option<String> {
        self.document
            .page_label(index)
            .ok()
            .filter(|label| !label.is_empty())
    }

    fn title(&self) -> Option<String> {
        self.document
            .metadata(MetadataName::Title)
            .ok()
            .map(|title| title.trim().to_owned())
            .filter(|title| !title.is_empty())
    }

    fn outline(&self) -> Result<Vec<OutlineItem>> {
        let outlines = self.document.outlines().map_err(engine_error)?;
        Ok(self.outline_items(outlines))
    }

    fn links(&self, index: usize) -> Result<Vec<Link>> {
        let page = self.load_page(index)?;
        let origin = page.bounds().map_err(engine_error)?;
        let links = page.links().map_err(engine_error)?;
        Ok(links
            .filter_map(|link| {
                let target = self.target(link.dest, &link.uri)?;
                let bounds = offset_rect(link.bounds, origin);
                Some(Link { bounds, target })
            })
            .collect())
    }

    fn display(&self, index: usize) -> Result<Arc<dyn PageDisplay>> {
        let page = self.load_page(index)?;
        let bounds = page.bounds().map_err(engine_error)?;
        let list = page.to_display_list(true).map_err(engine_error)?;
        Ok(Arc::new(MupdfPage { list, bounds }))
    }
}

struct MupdfPage {
    list: DisplayList,
    bounds: mupdf::Rect,
}

fn offset_rect(rect: mupdf::Rect, origin: mupdf::Rect) -> Rect {
    Rect::new(
        rect.x0 - origin.x0,
        rect.y0 - origin.y0,
        rect.x1 - origin.x0,
        rect.y1 - origin.y0,
    )
}

fn offset_quad(quad: mupdf::Quad, origin: &mupdf::Rect) -> Quad {
    let point = |point: mupdf::Point| Point::new(point.x - origin.x0, point.y - origin.y0);
    Quad {
        ul: point(quad.ul),
        ur: point(quad.ur),
        ll: point(quad.ll),
        lr: point(quad.lr),
    }
}

impl PageDisplay for MupdfPage {
    fn size(&self) -> Size {
        Size::new(self.bounds.width(), self.bounds.height())
    }

    fn render(&self, scale: f32, area: PixelRect) -> Result<Bitmap> {
        let mut ctm = Matrix::new_translate(-self.bounds.x0, -self.bounds.y0);
        ctm.concat(Matrix::new_scale(scale, scale));
        let rect = IRect {
            x0: area.x,
            y0: area.y,
            x1: area.x + area.width as i32,
            y1: area.y + area.height as i32,
        };
        let mut pixmap =
            Pixmap::new_with_rect(&Colorspace::device_rgb(), rect, false).map_err(engine_error)?;
        pixmap.clear_with(0xff).map_err(engine_error)?;
        {
            let device = Device::from_pixmap(&pixmap).map_err(engine_error)?;
            let clip = mupdf::Rect::new(
                rect.x0 as f32,
                rect.y0 as f32,
                rect.x1 as f32,
                rect.y1 as f32,
            );
            self.list.run(&device, &ctm, clip).map_err(engine_error)?;
        }
        Ok(rgb_to_rgba(&pixmap, area.width, area.height))
    }

    fn text(&self) -> Result<TextLayout> {
        let page = self
            .list
            .to_text_page(TextPageFlags::empty())
            .map_err(engine_error)?;
        let mut lines = Vec::new();
        for block in page
            .blocks()
            .filter(|block| block.r#type() == TextBlockType::Text)
        {
            for line in block.lines() {
                let chars: Vec<TextChar> = line
                    .chars()
                    .filter_map(|char| {
                        Some(TextChar {
                            character: char.char()?,
                            quad: offset_quad(char.quad(), &self.bounds),
                        })
                    })
                    .collect();
                if !chars.is_empty() {
                    lines.push(TextLine {
                        bounds: offset_rect(line.bounds(), self.bounds),
                        chars,
                    });
                }
            }
        }
        Ok(TextLayout { lines })
    }

    fn search(&self, needle: &str) -> Result<Vec<Quad>> {
        if needle.trim().is_empty() {
            return Ok(Vec::new());
        }
        let hits = self
            .list
            .search(needle, MAX_SEARCH_HITS)
            .map_err(engine_error)?;
        Ok(hits
            .iter()
            .map(|quad| offset_quad(quad.clone(), &self.bounds))
            .collect())
    }
}

fn dictionary_rect(page: &PdfObject, key: &str) -> Option<Rect> {
    let array = page.get_dict_inheritable(key).ok()??;
    if !array.is_array().ok()? || array.len().ok()? != 4 {
        return None;
    }
    let value = |index: i32| {
        array
            .get_array(index)
            .ok()?
            .and_then(|number| number.as_float().ok())
    };
    let (a, b, c, d) = (value(0)?, value(1)?, value(2)?, value(3)?);
    let rect = Rect::new(a.min(c), b.min(d), a.max(c), b.max(d));
    (rect.width() > 0.0 && rect.height() > 0.0).then_some(rect)
}

/// The size MuPDF would report for a page: the crop box within the media
/// box, scaled by UserUnit and rotated.
fn page_dictionary_size(page: &PdfObject) -> Option<Size> {
    let media = dictionary_rect(page, "MediaBox")?;
    let visible = match dictionary_rect(page, "CropBox") {
        Some(crop) => Rect::new(
            media.x0.max(crop.x0),
            media.y0.max(crop.y0),
            media.x1.min(crop.x1),
            media.y1.min(crop.y1),
        ),
        None => media,
    };
    if visible.width() <= 0.0 || visible.height() <= 0.0 {
        return None;
    }
    let user_unit = page
        .get_dict("UserUnit")
        .ok()
        .flatten()
        .and_then(|unit| unit.as_float().ok())
        .filter(|unit| *unit > 0.0)
        .unwrap_or(1.0);
    let rotate = page
        .get_dict_inheritable("Rotate")
        .ok()
        .flatten()
        .and_then(|rotate| rotate.as_int().ok())
        .unwrap_or(0);
    let (width, height) = (visible.width() * user_unit, visible.height() * user_unit);
    // Snap to a quarter turn, as MuPDF does.
    let quarter_turns = (rotate.rem_euclid(360) + 45) / 90;
    Some(if quarter_turns % 2 == 1 {
        Size::new(height, width)
    } else {
        Size::new(width, height)
    })
}

fn rgb_to_rgba(pixmap: &Pixmap, width: u32, height: u32) -> Bitmap {
    let stride = pixmap.stride() as usize;
    let samples = pixmap.samples();
    let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
    for row in 0..height as usize {
        let start = row * stride;
        let (row_pixels, _) = samples[start..start + width as usize * 3].as_chunks::<3>();
        for [red, green, blue] in row_pixels {
            pixels.extend_from_slice(&[*red, *green, *blue, 0xff]);
        }
    }
    Bitmap {
        width,
        height,
        pixels,
    }
}
