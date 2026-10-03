//! Page operations, redaction and export with MuPDF.

use std::collections::{HashMap, HashSet};

use mupdf::pdf::{
    Encryption, InsertPdfOptions, InsertPosition, PageSelection, PdfAnnotationType, PdfDocument,
    PdfObject, PdfRedactImageMethod, PdfRedactLineArtMethod, PdfRedactOptions, PdfRedactTextMethod,
    PdfWriteOptions, Permission,
};
use mupdf::{Buffer, Size as MupdfSize};

use crate::engine::{CropBox, Error, ExportOptions, Reduce, RemovedPage, Result};
use crate::geometry::{Rect, Size};
use crate::mupdf_annotations::Space;

fn engine_error(error: mupdf::Error) -> Error {
    Error::Engine(error.to_string())
}

fn page_count(document: &PdfDocument) -> Result<usize> {
    document
        .page_count()
        .map(|count| count.max(0) as usize)
        .map_err(engine_error)
}

fn check_page(document: &PdfDocument, page: usize) -> Result<i32> {
    if page >= page_count(document)? {
        return Err(Error::PageOutOfRange(page));
    }
    Ok(page as i32)
}

/// The document's page labels, written out, to put back after MuPDF
/// adjusts them.
fn page_labels(document: &PdfDocument) -> Result<Option<String>> {
    let catalog = document.catalog().map_err(engine_error)?;
    let labels = catalog
        .get_dict("PageLabels")
        .map_err(engine_error)?
        .map(|labels| labels.resolve())
        .transpose()
        .map_err(engine_error)?
        .flatten();
    Ok(labels.map(|labels| labels.to_string()))
}

fn set_page_labels(document: &PdfDocument, labels: Option<&str>) -> Result<()> {
    let mut catalog = document.catalog().map_err(engine_error)?;
    match labels {
        Some(labels) => catalog
            .dict_put(
                "PageLabels",
                document.new_object_from_str(labels).map_err(engine_error)?,
            )
            .map_err(engine_error),
        None => catalog.dict_delete("PageLabels").map_err(engine_error),
    }
}

/// Runs a page tree change that MuPDF follows by shifting page label
/// ranges. In a document without labels it makes some up, numbering
/// pages from 1 again where one was inserted at the start; those go.
fn keeping_labels<T>(
    document: &mut PdfDocument,
    change: impl FnOnce(&mut PdfDocument) -> Result<T>,
) -> Result<T> {
    let before = page_labels(document)?;
    let result = change(document)?;
    if before.is_none() && page_labels(document)?.is_some() {
        set_page_labels(document, None)?;
    }
    Ok(result)
}

pub(crate) fn rotate_page(document: &PdfDocument, page: usize, quarter_turns: i32) -> Result<()> {
    let index = check_page(document, page)?;
    let mut pdf_page = document.load_pdf_page(index).map_err(engine_error)?;
    let rotation = pdf_page.rotation().map_err(engine_error)?;
    let turned = (rotation + quarter_turns * 90).rem_euclid(360);
    pdf_page.set_rotation(turned).map_err(engine_error)
}

pub(crate) fn remove_page(document: &mut PdfDocument, page: usize) -> Result<RemovedPage> {
    let index = check_page(document, page)?;
    let object = document
        .find_page(index)
        .and_then(|reference| reference.as_indirect())
        .map_err(engine_error)?;
    if object <= 0 {
        return Err(Error::Engine("the page is not an indirect object".into()));
    }
    keeping_labels(document, |document| {
        document.delete_page(index).map_err(engine_error)
    })?;
    Ok(RemovedPage { object })
}

pub(crate) fn restore_page(
    document: &mut PdfDocument,
    at: usize,
    removed: &RemovedPage,
) -> Result<()> {
    if at > page_count(document)? {
        return Err(Error::PageOutOfRange(at));
    }
    let reference = document
        .new_indirect(removed.object, 0)
        .map_err(engine_error)?;
    let is_page = reference
        .resolve()
        .ok()
        .flatten()
        .and_then(|page| page.get_dict("Type").ok().flatten())
        .and_then(|kind| kind.as_name().ok())
        .is_some_and(|name| name == b"Page");
    if !is_page {
        return Err(Error::Engine("the removed page is no longer there".into()));
    }
    keeping_labels(document, |document| {
        document
            .insert_page(at as i32, &reference)
            .map_err(engine_error)
    })
}

pub(crate) fn move_page(document: &mut PdfDocument, from: usize, to: usize) -> Result<()> {
    check_page(document, from)?;
    check_page(document, to)?;
    // Labels belong to positions, which a move does not change.
    let labels = page_labels(document)?;
    document.move_page(from, to).map_err(engine_error)?;
    set_page_labels(document, labels.as_deref())
}

pub(crate) fn insert_blank_page(document: &mut PdfDocument, at: usize, size: Size) -> Result<()> {
    if at > page_count(document)? {
        return Err(Error::PageOutOfRange(at));
    }
    keeping_labels(document, |document| {
        document
            .new_page_at(at as i32, MupdfSize::new(size.width, size.height))
            .map(drop)
            .map_err(engine_error)
    })
}

pub(crate) fn insert_document(
    document: &mut PdfDocument,
    at: usize,
    bytes: &[u8],
) -> Result<usize> {
    if at > page_count(document)? {
        return Err(Error::PageOutOfRange(at));
    }
    let source = PdfDocument::from_bytes(bytes).map_err(|error| Error::Open(error.to_string()))?;
    if source.needs_password().unwrap_or(false) {
        return Err(Error::PasswordProtected);
    }
    let count = page_count(&source)?;
    let inserted = keeping_labels(document, |document| {
        document
            .insert_pdf(
                &source,
                InsertPdfOptions {
                    source_pages: PageSelection::All,
                    target: InsertPosition::Before(at),
                    ..InsertPdfOptions::default()
                },
            )
            .map_err(engine_error)
    })?;
    let pairs: Vec<(usize, usize)> = (0..count).map(|page| (page, at + page)).collect();
    copy_annotations(document, &source, &pairs, &|_| true)?;
    Ok(inserted.page_count)
}

/// Whether a link goes to a place in its own document, which a copy of
/// the page elsewhere cannot keep.
fn is_internal_link(annot: &PdfObject) -> Result<bool> {
    if annot.get_dict("Dest").map_err(engine_error)?.is_some() {
        return Ok(true);
    }
    let action = annot.get_dict("A").map_err(engine_error)?;
    Ok(action
        .and_then(|action| name(&action, "S"))
        .is_none_or(|kind| kind != b"URI"))
}

/// Copies the entries of `source` into a new dictionary, leaving out
/// `skip`.
fn copy_dict(
    document: &PdfDocument,
    map: &mut mupdf::pdf::PdfGraftMap,
    source: &PdfObject,
    skip: &[&[u8]],
) -> Result<PdfObject> {
    let mut dict = document.new_dict().map_err(engine_error)?;
    for entry in source.dict_iter().map_err(engine_error)? {
        let (key, value) = entry.map_err(engine_error)?;
        if skip.contains(&key.as_name().map_err(engine_error)?.as_slice()) {
            continue;
        }
        let value = map.graft_object(&value).map_err(engine_error)?;
        dict.dict_put(key, value).map_err(engine_error)?;
    }
    Ok(dict)
}

/// Copies a form field and the fields above it, once each, linking the
/// copies as parent and kid. Returns the copy's reference.
fn copy_field(
    document: &mut PdfDocument,
    map: &mut mupdf::pdf::PdfGraftMap,
    field: &PdfObject,
    copies: &mut HashMap<i32, PdfObject>,
    roots: &mut Vec<PdfObject>,
) -> Result<PdfObject> {
    let number = field.as_indirect().map_err(engine_error)?;
    if let Some(copy) = copies.get(&number) {
        return copy.try_clone().map_err(engine_error);
    }
    let mut dict = copy_dict(document, map, field, &[b"Parent", b"Kids", b"P"])?;
    dict.dict_put("Kids", document.new_array().map_err(engine_error)?)
        .map_err(engine_error)?;
    let reference = document.add_object(&dict).map_err(engine_error)?;
    copies.insert(number, reference.try_clone().map_err(engine_error)?);
    attach_field(document, map, field, &reference, copies, roots)?;
    Ok(reference)
}

/// Makes the copy of `field` a kid of its parent's copy, or a top-level
/// field when it has no parent.
fn attach_field(
    document: &mut PdfDocument,
    map: &mut mupdf::pdf::PdfGraftMap,
    field: &PdfObject,
    copy: &PdfObject,
    copies: &mut HashMap<i32, PdfObject>,
    roots: &mut Vec<PdfObject>,
) -> Result<()> {
    let parent = field
        .get_dict("Parent")
        .map_err(engine_error)?
        .filter(|parent| parent.is_indirect().unwrap_or(false));
    let Some(parent) = parent.filter(|_| copies.len() < 10_000) else {
        roots.push(copy.try_clone().map_err(engine_error)?);
        return Ok(());
    };
    let parent_copy = copy_field(document, map, &parent, copies, roots)?;
    let mut copy = copy.try_clone().map_err(engine_error)?;
    copy.dict_put("Parent", parent_copy.try_clone().map_err(engine_error)?)
        .map_err(engine_error)?;
    let mut kids = parent_copy
        .get_dict("Kids")
        .map_err(engine_error)?
        .ok_or_else(|| Error::Engine("a copied field has no kids".into()))?;
    kids.array_push(copy).map_err(engine_error)?;
    Ok(())
}

/// Copies the annotations of source pages onto the pages grafted from
/// them, which MuPDF leaves bare. Form fields come along with the fields
/// above them; links into the source document and popups are left behind.
fn copy_annotations(
    document: &mut PdfDocument,
    source: &PdfDocument,
    pairs: &[(usize, usize)],
    keep: &dyn Fn(&PdfObject) -> bool,
) -> Result<()> {
    let mut map = document.new_graft_map().map_err(engine_error)?;
    // Top-level fields, and the copies of every field by source number.
    let mut fields = Vec::new();
    let mut fields_copied = HashMap::new();
    for &(from, to) in pairs {
        let source_page = source.find_page(from as i32).map_err(engine_error)?;
        let Some(annots) = source_page.get_dict("Annots").map_err(engine_error)? else {
            continue;
        };
        if !annots.is_array().map_err(engine_error)? {
            continue;
        }
        let mut target_page = document.find_page(to as i32).map_err(engine_error)?;
        let mut copied = document.new_array().map_err(engine_error)?;
        for index in 0..annots.len().map_err(engine_error)? {
            let Some(annot) = annots.get_array(index as i32).map_err(engine_error)? else {
                continue;
            };
            if !annot.is_dict().map_err(engine_error)? {
                continue;
            }
            let subtype = name(&annot, "Subtype").unwrap_or_default();
            if subtype == b"Popup"
                || (subtype == b"Link" && is_internal_link(&annot)?)
                || !keep(&annot)
            {
                continue;
            }
            let mut dict = copy_dict(
                document,
                &mut map,
                &annot,
                &[b"P", b"Parent", b"Popup", b"IRT"],
            )?;
            dict.dict_put("P", target_page.try_clone().map_err(engine_error)?)
                .map_err(engine_error)?;
            let reference = document.add_object(&dict).map_err(engine_error)?;
            if subtype == b"Widget" {
                attach_field(
                    document,
                    &mut map,
                    &annot,
                    &reference,
                    &mut fields_copied,
                    &mut fields,
                )?;
            }
            copied.array_push(reference).map_err(engine_error)?;
        }
        if copied.len().map_err(engine_error)? > 0 {
            target_page
                .dict_put("Annots", copied)
                .map_err(engine_error)?;
        }
    }
    if fields.is_empty() {
        return Ok(());
    }
    let mut catalog = document.catalog().map_err(engine_error)?;
    let mut form = match catalog.get_dict("AcroForm").map_err(engine_error)? {
        Some(form) => form,
        None => {
            let mut form = document.new_dict().map_err(engine_error)?;
            if let Some(source_form) = source
                .catalog()
                .and_then(|catalog| catalog.get_dict("AcroForm"))
                .map_err(engine_error)?
            {
                for key in ["DA", "DR", "NeedAppearances"] {
                    if let Some(value) = source_form.get_dict(key).map_err(engine_error)? {
                        let value = map.graft_object(&value).map_err(engine_error)?;
                        form.dict_put(key, value).map_err(engine_error)?;
                    }
                }
            }
            let form = document.add_object(&form).map_err(engine_error)?;
            catalog
                .dict_put("AcroForm", form.try_clone().map_err(engine_error)?)
                .map_err(engine_error)?;
            form
        }
    };
    let mut list = match form.get_dict("Fields").map_err(engine_error)? {
        Some(list) if list.is_array().map_err(engine_error)? => list,
        _ => {
            form.dict_put("Fields", document.new_array().map_err(engine_error)?)
                .map_err(engine_error)?;
            form.get_dict("Fields")
                .map_err(engine_error)?
                .ok_or_else(|| Error::Engine("could not add form fields".into()))?
        }
    };
    for field in fields {
        list.array_push(field).map_err(engine_error)?;
    }
    Ok(())
}

/// Options for a complete rewrite: nothing incremental, unused objects
/// dropped and duplicates merged.
fn rewrite_options() -> PdfWriteOptions {
    let mut options = PdfWriteOptions::default();
    options
        .set_incremental(false)
        .set_garbage_level(3)
        .set_compress(true)
        .set_compress_fonts(true)
        .set_compress_images(true)
        .set_encryption(Encryption::None);
    options
}

fn write(document: &PdfDocument, options: PdfWriteOptions) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    document
        .write_to_with_options(&mut bytes, options)
        .map_err(engine_error)?;
    Ok(bytes)
}

pub(crate) fn extract_pages(document: &PdfDocument, pages: &[usize]) -> Result<Vec<u8>> {
    if pages.is_empty() {
        return Err(Error::NoPages);
    }
    for page in pages {
        check_page(document, *page)?;
    }
    let mut extracted = PdfDocument::new();
    extracted
        .insert_pdf(
            document,
            InsertPdfOptions {
                source_pages: PageSelection::Pages(pages.to_vec()),
                ..InsertPdfOptions::default()
            },
        )
        .map_err(engine_error)?;
    let pairs: Vec<(usize, usize)> = pages.iter().copied().zip(0..).collect();
    copy_annotations(&mut extracted, document, &pairs, &|_| true)?;
    write(&extracted, rewrite_options())
}

/// The id prev gives an annotation object: `/NM`, or its object number.
fn annotation_id(annot: &PdfObject) -> Option<String> {
    if let Some(name) = annot
        .get_dict("NM")
        .ok()
        .flatten()
        .and_then(|name| name.as_string().ok())
        .filter(|name| !name.is_empty())
    {
        return Some(name);
    }
    annot
        .as_indirect()
        .ok()
        .map(|number| format!("object-{number}"))
}

/// A one-page copy of `page` with the annotations `keep` accepts, and
/// with the page's own contents or without them.
fn page_copy(
    document: &PdfDocument,
    page: usize,
    keep: &dyn Fn(&PdfObject) -> bool,
    empty: bool,
) -> Result<PdfDocument> {
    let mut copy = PdfDocument::new();
    copy.insert_pdf(
        document,
        InsertPdfOptions {
            source_pages: PageSelection::Pages(vec![page]),
            ..InsertPdfOptions::default()
        },
    )
    .map_err(engine_error)?;
    if empty {
        let mut page_object = copy.find_page(0).map_err(engine_error)?;
        page_object.dict_delete("Contents").map_err(engine_error)?;
    }
    copy_annotations(&mut copy, document, &[(page, 0)], keep)?;
    Ok(copy)
}

/// Two copies of `page` for moving annotation `id` on screen: one with
/// every other annotation, to paint over where it was, and one with the
/// annotation alone on an empty page. The document is not changed.
pub(crate) fn lift(
    document: &PdfDocument,
    page: usize,
    id: &str,
) -> Result<(PdfDocument, PdfDocument)> {
    check_page(document, page)?;
    let without = page_copy(
        document,
        page,
        &|annot| annotation_id(annot).as_deref() != Some(id),
        false,
    )?;
    Ok((without, alone(document, page, id)?))
}

/// A copy of `page` with annotation `id` alone on an empty page. The
/// document is not changed.
pub(crate) fn alone(document: &PdfDocument, page: usize, id: &str) -> Result<PdfDocument> {
    check_page(document, page)?;
    page_copy(
        document,
        page,
        &|annot| annotation_id(annot).as_deref() == Some(id),
        true,
    )
}

/// A copy of `page` with its annotations and none of its own contents,
/// to draw the markup over something else.
pub(crate) fn annotations_alone(document: &PdfDocument, page: usize) -> Result<PdfDocument> {
    check_page(document, page)?;
    page_copy(document, page, &|_| true, true)
}

/// A PDF of one page of `size` points filled by `image`, for marking up
/// an image with the PDF tools.
pub fn image_document(image: &crate::engine::Bitmap, size: Size) -> Result<Vec<u8>> {
    image_page(image, size, (0.0, 0.0, size.width, size.height))
}

/// A PDF of one page of `size` points with `image` as large as fits,
/// centered, for adding an image to a document as a page.
pub fn image_page_document(image: &crate::engine::Bitmap, size: Size) -> Result<Vec<u8>> {
    let (width, height) = (image.width.max(1) as f32, image.height.max(1) as f32);
    let scale = (size.width / width).min(size.height / height);
    let (fitted_width, fitted_height) = (width * scale, height * scale);
    image_page(
        image,
        size,
        (
            (size.width - fitted_width) / 2.0,
            (size.height - fitted_height) / 2.0,
            fitted_width,
            fitted_height,
        ),
    )
}

/// A PDF of one page of `size` points with `image` drawn at `place`: its
/// left, bottom, width and height.
fn image_page(
    image: &crate::engine::Bitmap,
    size: Size,
    (left, bottom, width, height): (f32, f32, f32, f32),
) -> Result<Vec<u8>> {
    let mut document = PdfDocument::new();
    document
        .new_page(MupdfSize::new(size.width, size.height))
        .map_err(engine_error)?;
    let pixmap = crate::mupdf_annotations::rgba_pixmap(image)?;
    let image = mupdf::Image::from_pixmap(&pixmap).map_err(engine_error)?;
    let image = document.add_image(&image).map_err(engine_error)?;
    let mut xobjects = document.new_dict().map_err(engine_error)?;
    xobjects.dict_put("Im", image).map_err(engine_error)?;
    let mut resources = document.new_dict().map_err(engine_error)?;
    resources
        .dict_put("XObject", xobjects)
        .map_err(engine_error)?;
    let contents = format!("q {width} 0 0 {height} {left} {bottom} cm /Im Do Q");
    let contents = Buffer::from_bytes(contents.as_bytes()).map_err(engine_error)?;
    let contents = document
        .add_stream(&contents, None, false)
        .map_err(engine_error)?;
    let mut page = document.find_page(0).map_err(engine_error)?;
    page.dict_put("Resources", resources)
        .map_err(engine_error)?;
    page.dict_put("Contents", contents).map_err(engine_error)?;
    let mut options = PdfWriteOptions::default();
    options.set_compress(true).set_compress_images(true);
    let mut bytes = Vec::new();
    document
        .write_to_with_options(&mut bytes, options)
        .map_err(engine_error)?;
    Ok(bytes)
}

fn rect_entry(object: &PdfObject, key: &str) -> Result<Option<[f32; 4]>> {
    let Some(array) = object.get_dict(key).map_err(engine_error)? else {
        return Ok(None);
    };
    if !array.is_array().map_err(engine_error)? || array.len().map_err(engine_error)? != 4 {
        return Ok(None);
    }
    let mut values = [0.0; 4];
    for (index, value) in values.iter_mut().enumerate() {
        *value = array
            .get_array(index as i32)
            .map_err(engine_error)?
            .map_or(Ok(0.0), |number| number.as_float())
            .map_err(engine_error)?;
    }
    Ok(Some(values))
}

fn put_rect(
    document: &PdfDocument,
    object: &mut PdfObject,
    key: &str,
    rect: [f32; 4],
) -> Result<()> {
    let mut array = document.new_array().map_err(engine_error)?;
    for value in rect {
        array
            .array_push(document.new_real(value).map_err(engine_error)?)
            .map_err(engine_error)?;
    }
    object.dict_put(key, array).map_err(engine_error)
}

pub(crate) fn crop_page(document: &PdfDocument, page: usize, rect: Rect) -> Result<CropBox> {
    let index = check_page(document, page)?;
    let pdf_page = document.load_pdf_page(index).map_err(engine_error)?;
    let space = Space::of(&pdf_page)?;
    let inverse = pdf_page
        .ctm()
        .map_err(engine_error)?
        .invert()
        .ok_or_else(|| Error::Engine("page transform cannot be inverted".into()))?;
    let user = mupdf::Rect::new(
        rect.x0 + space.x0,
        rect.y0 + space.y0,
        rect.x1 + space.x0,
        rect.y1 + space.y0,
    )
    .transform(&inverse);
    let mut object = pdf_page.object();
    let before = CropBox(rect_entry(&object, "CropBox")?);
    let media = object
        .get_dict_inheritable("MediaBox")
        .ok()
        .flatten()
        .and_then(|media| {
            let value = |index| media.get_array(index).ok()??.as_float().ok();
            Some([value(0)?, value(1)?, value(2)?, value(3)?])
        })
        .map(|[a, b, c, d]| [a.min(c), b.min(d), a.max(c), b.max(d)]);
    let mut crop = [user.x0, user.y0, user.x1, user.y1];
    if let Some(media) = media {
        crop = [
            crop[0].max(media[0]),
            crop[1].max(media[1]),
            crop[2].min(media[2]),
            crop[3].min(media[3]),
        ];
    }
    if crop[2] - crop[0] < 1.0 || crop[3] - crop[1] < 1.0 {
        return Err(Error::CropOutsidePage);
    }
    put_rect(document, &mut object, "CropBox", crop)?;
    Ok(before)
}

pub(crate) fn set_crop_box(document: &PdfDocument, page: usize, crop: &CropBox) -> Result<CropBox> {
    let index = check_page(document, page)?;
    let mut object = document.find_page(index).map_err(engine_error)?;
    let before = CropBox(rect_entry(&object, "CropBox")?);
    match crop.0 {
        Some(rect) => put_rect(document, &mut object, "CropBox", rect)?,
        None => object.dict_delete("CropBox").map_err(engine_error)?,
    }
    Ok(before)
}

/// Text, images and drawings touching a mark are removed; the marks become
/// black boxes.
const REDACT: PdfRedactOptions = PdfRedactOptions {
    black_boxes: true,
    image_method: PdfRedactImageMethod::Pixels,
    line_art: PdfRedactLineArtMethod::RemoveIfCovered,
    text: PdfRedactTextMethod::Remove,
};

pub(crate) fn apply_redactions(document: &PdfDocument) -> Result<usize> {
    let mut applied = 0;
    for index in 0..page_count(document)? {
        let mut page = document.load_pdf_page(index as i32).map_err(engine_error)?;
        let marks = page
            .annotations()
            .filter(|annot| annot.r#type().ok() == Some(PdfAnnotationType::Redact))
            .count();
        if marks > 0 {
            page.redact_with_options(REDACT).map_err(engine_error)?;
            applied += marks;
        }
    }
    Ok(applied)
}

pub(crate) fn export(document: &PdfDocument, options: &ExportOptions) -> Result<Vec<u8>> {
    let mut write_options = rewrite_options();
    if let Some(password) = options
        .password
        .as_deref()
        .filter(|password| !password.is_empty())
    {
        write_options
            .set_encryption(Encryption::Aes256)
            .set_owner_password(password)
            .set_user_password(password)
            .set_permissions(Permission::all());
    }
    // Reducing and choosing pages work on a copy, so the open document
    // keeps its images and pages.
    let copy = match &options.pages {
        Some(pages) => Some(extract_pages(document, pages)?),
        None if options.reduce.is_some() || options.flatten => {
            let mut copy_options = PdfWriteOptions::default();
            copy_options.set_encryption(Encryption::None);
            Some(write(document, copy_options)?)
        }
        None => None,
    };
    let Some(copy) = copy else {
        return write(document, write_options);
    };
    let mut copy = PdfDocument::from_bytes(&copy).map_err(engine_error)?;
    if options.flatten {
        flatten(&mut copy)?;
    }
    if let Some(reduce) = options.reduce {
        reduce_images(&copy, reduce)?;
    }
    write(&copy, write_options)
}

/// Draws every annotation and form field into its page's content and
/// removes it, after taking out redaction marks not yet applied.
fn flatten(document: &mut PdfDocument) -> Result<()> {
    for index in 0..page_count(document)? {
        let mut page = document.load_pdf_page(index as i32).map_err(engine_error)?;
        let marks: Vec<_> = page
            .annotations()
            .filter(|annot| annot.r#type().ok() == Some(PdfAnnotationType::Redact))
            .collect();
        for mark in marks {
            page.delete_annotation(mark).map_err(engine_error)?;
        }
    }
    document.bake(true, true).map_err(engine_error)?;
    // The fields are drawn now; the form would only list empty entries.
    let mut catalog = document.catalog().map_err(engine_error)?;
    catalog.dict_delete("AcroForm").map_err(engine_error)
}

fn name(object: &PdfObject, key: &str) -> Option<Vec<u8>> {
    let value = object.get_dict(key).ok()??;
    if value.is_name().ok()? {
        return value.as_name().ok();
    }
    // A filter array with one entry.
    if value.is_array().ok()? && value.len().ok()? == 1 {
        return value.get_array(0).ok()??.as_name().ok();
    }
    None
}

/// Color components of an image's color space, for the ones prev can
/// write as JPEG: gray and RGB, device or ICC based.
fn components(image: &PdfObject) -> Option<u8> {
    let space = image.get_dict("ColorSpace").ok()??;
    if space.is_name().ok()? {
        return match space.as_name().ok()?.as_slice() {
            b"DeviceGray" | b"G" => Some(1),
            b"DeviceRGB" | b"RGB" => Some(3),
            _ => None,
        };
    }
    if space.is_array().ok()? && space.len().ok()? == 2 {
        let family = space.get_array(0).ok()??.as_name().ok()?;
        let stream = space.get_array(1).ok()??;
        if family == b"ICCBased" {
            return match stream.get_dict("N").ok()??.as_int().ok()? {
                1 => Some(1),
                3 => Some(3),
                _ => None,
            };
        }
    }
    None
}

/// The image's pixels, 8 bits per component, when prev can recompress it.
fn image_pixels(image: &PdfObject, width: u32, height: u32, components: u8) -> Option<Vec<u8>> {
    let bits = image.get_dict("BitsPerComponent").ok()??.as_int().ok()?;
    if bits != 8 {
        return None;
    }
    let length = width as usize * height as usize * components as usize;
    match name(image, "Filter").as_deref() {
        Some(b"DCTDecode") => {
            let raw = image.read_raw_stream().ok()?;
            let decoded =
                image::load_from_memory_with_format(&raw, image::ImageFormat::Jpeg).ok()?;
            let pixels = match components {
                1 => decoded.into_luma8().into_raw(),
                _ => decoded.into_rgb8().into_raw(),
            };
            (pixels.len() == length).then_some(pixels)
        }
        Some(b"FlateDecode") | None => {
            let pixels = image.read_stream().ok()?;
            (pixels.len() >= length).then(|| pixels[..length].to_vec())
        }
        _ => None,
    }
}

/// Makes one image smaller, in place, so every page using it gains.
/// Returns whether it changed.
fn reduce_image(image: &mut PdfObject, max_side: f32, quality: u8) -> Result<bool> {
    let int = |key: &str| {
        image
            .get_dict(key)
            .ok()
            .flatten()
            .and_then(|value| value.as_int().ok())
    };
    let (Some(width), Some(height)) = (int("Width"), int("Height")) else {
        return Ok(false);
    };
    let (width, height) = (width.max(0) as u32, height.max(0) as u32);
    // Stencil masks, color key masks and decode arrays do not survive
    // JPEG; small images gain nothing.
    let unsuitable = image
        .get_dict("ImageMask")
        .ok()
        .flatten()
        .is_some_and(|mask| mask.as_bool().unwrap_or(false))
        || image
            .get_dict("Mask")
            .ok()
            .flatten()
            .is_some_and(|mask| mask.is_array().unwrap_or(false))
        || image.get_dict("Decode").ok().flatten().is_some()
        || (width as u64 * height as u64) < 64 * 64;
    let Some(components) = components(image).filter(|_| !unsuitable) else {
        return Ok(false);
    };
    let is_jpeg = name(image, "Filter").as_deref() == Some(b"DCTDecode");
    let scale = (max_side / width.max(height) as f32).min(1.0);
    if is_jpeg && scale > 0.9 {
        return Ok(false);
    }
    let Some(pixels) = image_pixels(image, width, height, components) else {
        return Ok(false);
    };
    let new_width = ((width as f32 * scale).round() as u32).max(1);
    let new_height = ((height as f32 * scale).round() as u32).max(1);
    let filter = image::imageops::FilterType::Triangle;
    let (pixels, color) = if components == 1 {
        let gray = image::GrayImage::from_raw(width, height, pixels)
            .ok_or_else(|| Error::Engine("image size mismatch".into()))?;
        let gray = if scale < 1.0 {
            image::imageops::resize(&gray, new_width, new_height, filter)
        } else {
            gray
        };
        (gray.into_raw(), image::ExtendedColorType::L8)
    } else {
        let rgb = image::RgbImage::from_raw(width, height, pixels)
            .ok_or_else(|| Error::Engine("image size mismatch".into()))?;
        let rgb = if scale < 1.0 {
            image::imageops::resize(&rgb, new_width, new_height, filter)
        } else {
            rgb
        };
        (rgb.into_raw(), image::ExtendedColorType::Rgb8)
    };
    let mut jpeg = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, quality.clamp(1, 100))
        .encode(&pixels, new_width, new_height, color)
        .map_err(|error| Error::Engine(error.to_string()))?;
    let before = image.read_raw_stream().map_or(usize::MAX, |raw| raw.len());
    if jpeg.len() as f32 > before as f32 * 0.9 {
        return Ok(false);
    }
    image
        .write_raw_stream_buffer(&Buffer::from_bytes(&jpeg).map_err(engine_error)?)
        .map_err(engine_error)?;
    let document = image
        .document()
        .ok_or_else(|| Error::Engine("image has no document".into()))?;
    image
        .dict_put(
            "Filter",
            document.new_name("DCTDecode").map_err(engine_error)?,
        )
        .map_err(engine_error)?;
    image.dict_delete("DecodeParms").map_err(engine_error)?;
    for (key, value) in [("Width", new_width), ("Height", new_height)] {
        image
            .dict_put(key, document.new_int(value as i32).map_err(engine_error)?)
            .map_err(engine_error)?;
    }
    image
        .dict_put(
            "BitsPerComponent",
            document.new_int(8).map_err(engine_error)?,
        )
        .map_err(engine_error)?;
    Ok(true)
}

/// Downsamples and recompresses the images on every page. An image's
/// size limit comes from the largest page using it: its longer side may
/// have `dpi` pixels per inch of the page's longer side.
pub(crate) fn reduce_images(document: &PdfDocument, reduce: Reduce) -> Result<usize> {
    let mut seen = HashSet::new();
    let mut reduced = 0;
    for index in 0..page_count(document)? {
        let page = document.load_pdf_page(index as i32).map_err(engine_error)?;
        let bounds = page.bounds().map_err(engine_error)?;
        let max_side = bounds.width().max(bounds.height()) / 72.0 * reduce.dpi;
        for info in page.images().map_err(engine_error)? {
            if !seen.insert(info.xref) {
                continue;
            }
            // The reference, so stream writes find the object's number.
            let mut image = document.new_indirect(info.xref, 0).map_err(engine_error)?;
            if reduce_image(&mut image, max_side, reduce.quality)? {
                reduced += 1;
            }
        }
    }
    Ok(reduced)
}
