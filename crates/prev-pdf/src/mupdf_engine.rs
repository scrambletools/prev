//! `Engine` implementation backed by MuPDF.

use std::path::Path;
use std::sync::Arc;

use mupdf::pdf::{PdfDocument, PdfObject};
use mupdf::text_page::{TextBlockType, TextPageFlags};
use mupdf::{
    Colorspace, DestinationKind, Device, DisplayList, IRect, Matrix, MetadataName, Pixmap,
};

use crate::annotation::{Annotation, Field, Removed, StampContent};
use crate::engine::{
    Bitmap, CropBox, Document, Engine, Error, ExportOptions, Lifted, Link, LinkTarget, Metadata,
    OutlineItem, PageDisplay, RemovedPage, Result,
};
use crate::geometry::{PixelRect, Point, Quad, Rect, Size};
use crate::text::{TextChar, TextLayout, TextLine};
use crate::{mupdf_annotations, mupdf_pages};

const MAX_SEARCH_HITS: u32 = 10_000;

fn engine_error(error: mupdf::Error) -> Error {
    Error::Engine(error.to_string())
}

pub struct MupdfEngine;

impl Engine for MupdfEngine {
    fn open(&self, path: &Path) -> Result<Box<dyn Document>> {
        // Saving replaces the file, which Windows refuses while it is open,
        // so there MuPDF gets the file's contents rather than the file.
        #[cfg(windows)]
        let document = {
            let bytes = std::fs::read(path).map_err(|error| Error::Open(error.to_string()))?;
            PdfDocument::from_bytes(&bytes)
        };
        #[cfg(not(windows))]
        let document = PdfDocument::open(path.as_os_str());
        let document = document.map_err(|error| Error::Open(error.to_string()))?;
        Ok(Box::new(MupdfDocument {
            document,
            rewrite: false,
        }))
    }
}

struct MupdfDocument {
    document: PdfDocument,
    /// Set by redaction: the next save must rewrite the whole file, so the
    /// removed content does not survive in an earlier revision.
    rewrite: bool,
}

impl MupdfDocument {
    fn load_pdf_page(&self, index: usize) -> Result<mupdf::pdf::PdfPage> {
        if index >= self.page_count()? {
            return Err(Error::PageOutOfRange(index));
        }
        self.document
            .load_pdf_page(index as i32)
            .map_err(engine_error)
    }

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

    fn metadata(&self) -> Metadata {
        let read = |name: MetadataName| {
            self.document
                .metadata(name)
                .map(|value| value.trim().to_owned())
                .unwrap_or_default()
        };
        Metadata {
            title: read(MetadataName::Title),
            author: read(MetadataName::Author),
            subject: read(MetadataName::Subject),
            keywords: read(MetadataName::Keywords),
            creator: read(MetadataName::Creator),
            producer: read(MetadataName::Producer),
            created: read(MetadataName::CreationDate),
            modified: read(MetadataName::ModDate),
            format: read(MetadataName::Format),
            encryption: read(MetadataName::Encryption),
        }
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
        Ok(Arc::new(MupdfPage {
            list,
            bounds,
            transparent: false,
        }))
    }

    fn annotations(&self, page: usize) -> Result<Vec<Annotation>> {
        mupdf_annotations::read_annotations(&self.load_pdf_page(page)?)
    }

    fn add_annotation(
        &mut self,
        page: usize,
        annotation: &Annotation,
        content: Option<&StampContent>,
    ) -> Result<()> {
        let mut pdf_page = self.load_pdf_page(page)?;
        mupdf_annotations::add_annotation(&mut self.document, &mut pdf_page, annotation, content)?;
        pdf_page.update().map_err(engine_error)?;
        Ok(())
    }

    fn update_annotation(
        &mut self,
        page: usize,
        annotation: &Annotation,
        content: Option<&StampContent>,
    ) -> Result<()> {
        let mut pdf_page = self.load_pdf_page(page)?;
        mupdf_annotations::update_annotation(
            &mut self.document,
            &mut pdf_page,
            annotation,
            content,
        )?;
        pdf_page.update().map_err(engine_error)?;
        Ok(())
    }

    fn remove_annotation(&mut self, page: usize, id: &str) -> Result<Removed> {
        let mut pdf_page = self.load_pdf_page(page)?;
        let removed = mupdf_annotations::remove_annotation(&mut pdf_page, id)?;
        pdf_page.update().map_err(engine_error)?;
        Ok(removed)
    }

    fn restore_annotation(&mut self, page: usize, removed: &Removed) -> Result<()> {
        {
            let mut pdf_page = self.load_pdf_page(page)?;
            mupdf_annotations::restore_annotation(&self.document, &mut pdf_page, removed)?;
        }
        // Loading the page again picks up the restored entry.
        self.load_pdf_page(page)?.update().map_err(engine_error)?;
        Ok(())
    }

    fn fields(&self, page: usize) -> Result<Vec<Field>> {
        mupdf_annotations::read_fields(&self.load_pdf_page(page)?)
    }

    fn set_field(&mut self, page: usize, id: i32, value: &str) -> Result<()> {
        let pdf_page = self.load_pdf_page(page)?;
        mupdf_annotations::set_field(&mut self.document, &pdf_page, id, value)
    }

    fn has_changes(&self) -> bool {
        self.document.has_unsaved_changes()
    }

    fn save(&mut self) -> Result<Vec<u8>> {
        let mut options = mupdf::pdf::PdfWriteOptions::default();
        if self.rewrite {
            options
                .set_garbage_level(3)
                .set_compress(true)
                .set_encryption(mupdf::pdf::Encryption::Keep);
        } else if self.document.can_be_saved_incrementally() {
            options.set_incremental(true);
        } else {
            options.set_garbage_level(1);
        }
        let mut bytes = Vec::new();
        self.document
            .write_to_with_options(&mut bytes, options)
            .map_err(engine_error)?;
        Ok(bytes)
    }

    fn rotate_page(&mut self, page: usize, quarter_turns: i32) -> Result<()> {
        mupdf_pages::rotate_page(&self.document, page, quarter_turns)
    }

    fn remove_page(&mut self, page: usize) -> Result<RemovedPage> {
        mupdf_pages::remove_page(&mut self.document, page)
    }

    fn restore_page(&mut self, at: usize, removed: &RemovedPage) -> Result<()> {
        mupdf_pages::restore_page(&mut self.document, at, removed)
    }

    fn move_page(&mut self, from: usize, to: usize) -> Result<()> {
        mupdf_pages::move_page(&mut self.document, from, to)
    }

    fn insert_blank_page(&mut self, at: usize, size: Size) -> Result<()> {
        mupdf_pages::insert_blank_page(&mut self.document, at, size)
    }

    fn insert_document(&mut self, at: usize, bytes: &[u8]) -> Result<usize> {
        mupdf_pages::insert_document(&mut self.document, at, bytes)
    }

    fn extract_pages(&self, pages: &[usize]) -> Result<Vec<u8>> {
        mupdf_pages::extract_pages(&self.document, pages)
    }

    fn crop_page(&mut self, page: usize, rect: Rect) -> Result<CropBox> {
        mupdf_pages::crop_page(&self.document, page, rect)
    }

    fn set_crop_box(&mut self, page: usize, crop: &CropBox) -> Result<CropBox> {
        mupdf_pages::set_crop_box(&self.document, page, crop)
    }

    fn apply_redactions(&mut self) -> Result<usize> {
        let applied = mupdf_pages::apply_redactions(&self.document)?;
        if applied > 0 {
            self.rewrite = true;
        }
        Ok(applied)
    }

    fn export(&mut self, options: &ExportOptions) -> Result<Vec<u8>> {
        mupdf_pages::export(&self.document, options)
    }

    fn lift_annotation(&self, page: usize, id: &str) -> Result<Lifted> {
        let (without, alone) = mupdf_pages::lift(&self.document, page, id)?;
        Ok(Lifted {
            without: first_page_display(without, false)?,
            alone: first_page_display(alone, true)?,
        })
    }

    fn annotation_layer(&self, page: usize) -> Result<Arc<dyn PageDisplay>> {
        first_page_display(mupdf_pages::annotations_alone(&self.document, page)?, true)
    }
}

fn first_page_display(document: PdfDocument, transparent: bool) -> Result<Arc<dyn PageDisplay>> {
    let page = document.load_page(0).map_err(engine_error)?;
    let bounds = page.bounds().map_err(engine_error)?;
    let list = page.to_display_list(true).map_err(engine_error)?;
    Ok(Arc::new(MupdfPage {
        list,
        bounds,
        transparent,
    }))
}

struct MupdfPage {
    list: DisplayList,
    bounds: mupdf::Rect,
    /// Renders on a transparent background instead of white paper.
    transparent: bool,
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
        let mut pixmap = Pixmap::new_with_rect(&Colorspace::device_rgb(), rect, self.transparent)
            .map_err(engine_error)?;
        if self.transparent {
            pixmap.clear().map_err(engine_error)?;
        } else {
            pixmap.clear_with(0xff).map_err(engine_error)?;
        }
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
        if self.transparent {
            return Ok(premultiplied_to_rgba(&pixmap, area.width, area.height));
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

/// A pixmap with premultiplied alpha, as MuPDF draws them, to straight
/// RGBA.
fn premultiplied_to_rgba(pixmap: &Pixmap, width: u32, height: u32) -> Bitmap {
    let stride = pixmap.stride() as usize;
    let samples = pixmap.samples();
    let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
    for row in 0..height as usize {
        let start = row * stride;
        let (row_pixels, _) = samples[start..start + width as usize * 4].as_chunks::<4>();
        for [red, green, blue, alpha] in row_pixels {
            let straight = |channel: u8| {
                if *alpha == 0 {
                    0
                } else {
                    ((u32::from(channel) * 255 + u32::from(*alpha) / 2) / u32::from(*alpha))
                        .min(255) as u8
                }
            };
            pixels.extend_from_slice(&[straight(*red), straight(*green), straight(*blue), *alpha]);
        }
    }
    Bitmap {
        width,
        height,
        pixels,
    }
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
