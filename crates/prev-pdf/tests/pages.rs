//! Page operations, redaction and export.

mod common;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use futures::executor::block_on;
use prev_pdf::annotation::{Annotation, Kind, new_id};
use prev_pdf::engine::{Engine, ExportOptions, Reduce};
use prev_pdf::geometry::{Rect, Size};
use prev_pdf::mupdf_engine::MupdfEngine;
use prev_pdf::pages;
use prev_pdf::worker::{
    DocumentHandle, DocumentInfo, Edit, Opened, PageEdit, PageOutcome, Restructured, flatten,
};

fn engine() -> Arc<dyn Engine> {
    Arc::new(MupdfEngine)
}

fn open(dir: &tempfile::TempDir, bytes: &[u8]) -> (DocumentHandle, DocumentInfo, PathBuf) {
    let path = dir.path().join("fixture.pdf");
    std::fs::write(&path, bytes).unwrap();
    let (handle, opened) = DocumentHandle::open(engine(), path.clone());
    match flatten(block_on(opened)).unwrap() {
        Opened::Ready(info) => (handle, info, path),
        Opened::NeedsPassword => panic!("fixture is not encrypted"),
    }
}

fn save(handle: &DocumentHandle, path: &Path) {
    let target = path.to_owned();
    flatten(block_on(handle.save(Box::new(move |bytes| {
        std::fs::write(&target, bytes).map_err(|error| error.to_string())
    }))))
    .unwrap();
}

fn edit(handle: &DocumentHandle, edit: PageEdit) -> Restructured {
    flatten(block_on(handle.pages(edit))).unwrap()
}

/// The first line of text on each page, which names the fixture's pages.
fn page_names(handle: &DocumentHandle, count: usize) -> Vec<String> {
    (0..count)
        .map(|page| {
            let display = flatten(block_on(handle.display(page))).unwrap();
            let text = display.text().unwrap();
            text.lines
                .first()
                .map(|line| line.chars.iter().map(|char| char.character).collect())
                .unwrap_or_default()
        })
        .collect()
}

const HELLO: &str = "Hello world";
const TWO: &str = "Page two text";
const ROTATED: &str = "Rotated page";

#[test]
fn rotating_turns_pages_and_turns_back() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, info, _) = open(&dir, &common::navigation_fixture());
    assert_eq!(info.page_sizes[0], Size::new(612.0, 792.0));
    let turned = edit(
        &handle,
        PageEdit::Rotate {
            pages: vec![0, 2],
            quarter_turns: 1,
        },
    );
    assert_eq!(turned.info.page_sizes[0], Size::new(792.0, 612.0));
    // The third page was already turned once; now it is upside down.
    assert_eq!(turned.info.page_sizes[2], Size::new(612.0, 792.0));
    let back = edit(
        &handle,
        PageEdit::Rotate {
            pages: vec![0, 2],
            quarter_turns: -1,
        },
    );
    assert_eq!(back.info.page_sizes, info.page_sizes);
}

#[test]
fn removed_pages_come_back_even_after_a_save() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, path) = open(&dir, &common::navigation_fixture());
    let removed = edit(&handle, PageEdit::Remove(vec![2, 0]));
    assert_eq!(removed.info.page_sizes.len(), 1);
    assert_eq!(page_names(&handle, 1), [TWO]);
    let PageOutcome::Removed(tokens) = removed.outcome else {
        panic!("no removal tokens");
    };
    assert_eq!(
        tokens.iter().map(|(page, _)| *page).collect::<Vec<_>>(),
        [0, 2]
    );
    save(&handle, &path);
    assert_eq!(MupdfEngine.open(&path).unwrap().page_count().unwrap(), 1);
    let restored = edit(&handle, PageEdit::Restore(tokens));
    assert_eq!(restored.info.page_sizes.len(), 3);
    assert_eq!(page_names(&handle, 3), [HELLO, TWO, ROTATED]);
    save(&handle, &path);
    let reopened = MupdfEngine.open(&path).unwrap();
    assert_eq!(reopened.page_count().unwrap(), 3);
    // Links on the restored first page still work.
    assert_eq!(reopened.links(0).unwrap().len(), 2);
}

#[test]
fn the_last_page_cannot_be_removed() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, _) = open(&dir, &common::navigation_fixture());
    assert!(flatten(block_on(handle.pages(PageEdit::Remove(vec![0, 1, 2])))).is_err());
}

#[test]
fn reordering_and_undoing_it() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, path) = open(&dir, &common::navigation_fixture());
    let order = pages::moved_order(3, &[0], 3);
    let reordered = edit(&handle, PageEdit::Reorder(order.clone()));
    // Labels stay with positions.
    assert_eq!(
        reordered.info.page_labels,
        [Some("i".into()), Some("ii".into()), Some("1".into())]
    );
    assert_eq!(page_names(&handle, 3), [TWO, ROTATED, HELLO]);
    save(&handle, &path);
    let reopened = MupdfEngine.open(&path).unwrap();
    assert_eq!(reopened.page_size(2).unwrap(), Size::new(612.0, 792.0));
    edit(&handle, PageEdit::Reorder(pages::inverse(&order)));
    assert_eq!(page_names(&handle, 3), [HELLO, TWO, ROTATED]);
}

#[test]
fn inserting_blank_pages_and_other_documents() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, path) = open(&dir, &common::navigation_fixture());
    let blank = edit(
        &handle,
        PageEdit::InsertBlank {
            at: 1,
            size: Size::new(300.0, 400.0),
        },
    );
    assert_eq!(blank.outcome, PageOutcome::Inserted { at: 1, count: 1 });
    assert_eq!(blank.info.page_sizes[1], Size::new(300.0, 400.0));
    // A document without labels does not get any.
    let (plain, _, _) = open(&tempfile::tempdir().unwrap(), &common::form_fixture());
    let first = edit(
        &plain,
        PageEdit::InsertBlank {
            at: 0,
            size: Size::new(100.0, 100.0),
        },
    );
    assert_eq!(first.info.page_labels, [None, None]);
    let order = pages::moved_order(2, &[1], 0);
    let moved = edit(&plain, PageEdit::Reorder(order));
    assert_eq!(moved.info.page_labels, [None, None]);
    let other = Arc::new(common::navigation_fixture());
    let inserted = edit(
        &handle,
        PageEdit::Insert {
            at: 4,
            bytes: other,
        },
    );
    assert_eq!(inserted.outcome, PageOutcome::Inserted { at: 4, count: 3 });
    assert_eq!(
        page_names(&handle, 7),
        [HELLO, "", TWO, ROTATED, HELLO, TWO, ROTATED]
    );
    // Undoing an insert is removing what it added.
    let removed = edit(&handle, PageEdit::Remove((4..7).collect()));
    assert_eq!(removed.info.page_sizes.len(), 4);
    save(&handle, &path);
    assert_eq!(MupdfEngine.open(&path).unwrap().page_count().unwrap(), 4);
}

#[test]
fn extracted_pages_make_a_new_document() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, _) = open(&dir, &common::navigation_fixture());
    let bytes = flatten(block_on(handle.extract(vec![2, 0]))).unwrap();
    let (extracted, _, _) = open(&tempfile::tempdir().unwrap(), &bytes);
    assert_eq!(page_names(&extracted, 2), [ROTATED, HELLO]);
    // The rotation and links come along.
    let reopened = {
        let path = dir.path().join("extracted.pdf");
        std::fs::write(&path, &bytes).unwrap();
        MupdfEngine.open(&path).unwrap()
    };
    assert_eq!(reopened.page_size(0).unwrap(), Size::new(792.0, 612.0));
    // The web link comes along; the one to a page left behind does not.
    let links = reopened.links(1).unwrap();
    assert_eq!(links.len(), 1);
    assert!(matches!(
        links[0].target,
        prev_pdf::engine::LinkTarget::Uri(_)
    ));
}

#[test]
fn copied_pages_keep_annotations_and_form_fields() {
    use prev_pdf::annotation::FieldKind;
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, _) = open(&dir, &common::form_fixture());
    let square = Annotation::new(
        new_id(),
        Kind::Square,
        Rect::new(300.0, 300.0, 400.0, 400.0),
    );
    flatten(block_on(handle.edit(0, Edit::Add(square.clone(), None)))).unwrap();
    let bytes = flatten(block_on(handle.extract(vec![0]))).unwrap();
    let (copy, _, _) = open(&tempfile::tempdir().unwrap(), &bytes);
    let markup = flatten(block_on(copy.markup(0))).unwrap();
    assert!(
        markup
            .annotations
            .iter()
            .any(|annotation| annotation.id == square.id)
    );
    let mut names: Vec<&str> = markup
        .fields
        .iter()
        .map(|field| field.name.as_str())
        .collect();
    names.sort_unstable();
    assert_eq!(names, ["agree", "color", "name", "size", "size"]);
    // The radio buttons are still one group: turning one on turns the
    // other off.
    let radios: Vec<_> = markup
        .fields
        .iter()
        .filter(|field| field.kind == FieldKind::Radio)
        .cloned()
        .collect();
    let on = |field: &prev_pdf::annotation::Field| field.on_value.clone().unwrap();
    flatten(block_on(copy.edit(
        0,
        Edit::SetField {
            id: radios[0].id,
            value: on(&radios[0]),
        },
    )))
    .unwrap();
    let edited = flatten(block_on(copy.edit(
        0,
        Edit::SetField {
            id: radios[1].id,
            value: on(&radios[1]),
        },
    )))
    .unwrap();
    let states: Vec<bool> = edited
        .fields
        .iter()
        .filter(|field| field.kind == FieldKind::Radio)
        .map(|field| field.is_on())
        .collect();
    assert_eq!(states, [false, true]);
    // Pasting the page into its own document works too.
    let inserted = edit(
        &handle,
        PageEdit::Insert {
            at: 1,
            bytes: Arc::new(bytes),
        },
    );
    assert_eq!(inserted.info.page_sizes.len(), 2);
    let pasted = flatten(block_on(handle.markup(1))).unwrap();
    assert_eq!(pasted.fields.len(), 5);
}

#[test]
fn cropping_and_uncropping() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, path) = open(&dir, &common::navigation_fixture());
    let cropped = edit(
        &handle,
        PageEdit::Crop {
            pages: vec![0],
            rect: Rect::new(50.0, 60.0, 350.0, 260.0),
        },
    );
    let size = cropped.info.page_sizes[0];
    assert!((size.width - 300.0).abs() < 0.5 && (size.height - 200.0).abs() < 0.5);
    // The text at the top of the page is still in view, moved with the crop.
    let text = flatten(block_on(handle.display(0)))
        .unwrap()
        .text()
        .unwrap();
    let first = &text.lines[0];
    assert!((first.bounds.x0 - 22.0).abs() < 2.0, "{:?}", first.bounds);
    let PageOutcome::Cropped(before) = cropped.outcome else {
        panic!("no crop to undo");
    };
    save(&handle, &path);
    let undone = edit(&handle, PageEdit::SetCrop(before));
    assert_eq!(undone.info.page_sizes[0], Size::new(612.0, 792.0));

    // A rotated page crops to what is on screen.
    let rotated = edit(
        &handle,
        PageEdit::Crop {
            pages: vec![2],
            rect: Rect::new(0.0, 0.0, 400.0, 100.0),
        },
    );
    let size = rotated.info.page_sizes[2];
    assert!((size.width - 400.0).abs() < 0.5 && (size.height - 100.0).abs() < 0.5);
}

/// A page with text to keep, text, an image and a drawing to redact.
fn redaction_fixture() -> Vec<u8> {
    let red: Vec<u8> = [255u8, 0, 0].repeat(16 * 16);
    common::assemble_pdf(&[
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
/Resources << /Font << /F1 5 0 R >> /XObject << /Im1 6 0 R >> >> >>"
            .to_vec(),
        common::stream_object(
            "",
            b"BT /F1 18 Tf 72 700 Td (Visible text) Tj ET\n\
BT /F1 18 Tf 72 600 Td (SECRET-1234) Tj ET\n\
q 200 0 0 100 72 400 cm /Im1 Do Q\n\
0 0 1 rg 300 200 50 50 re f\n",
        ),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
        common::stream_object(
            "/Type /XObject /Subtype /Image /Width 16 /Height 16 \
/ColorSpace /DeviceRGB /BitsPerComponent 8",
            &red,
        ),
    ])
}

/// Every stream in the file, decoded.
fn decoded_streams(path: &Path) -> Vec<Vec<u8>> {
    let document = mupdf::pdf::PdfDocument::open(path.to_str().unwrap()).unwrap();
    let count = document.xref_len().unwrap();
    (1..count as i32)
        .filter_map(|number| {
            let object = document.new_indirect(number, 0).ok()?;
            object
                .is_stream()
                .ok()?
                .then(|| object.read_stream().ok())?
        })
        .collect()
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn mark(rect: Rect) -> Annotation {
    Annotation::new(new_id(), Kind::Redact, rect)
}

#[test]
fn redaction_removes_text_images_and_drawings_for_good() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, path) = open(&dir, &redaction_fixture());
    // Page space has its origin at the top-left.
    for rect in [
        Rect::new(60.0, 170.0, 250.0, 200.0),
        Rect::new(60.0, 280.0, 290.0, 400.0),
        Rect::new(290.0, 530.0, 360.0, 600.0),
    ] {
        flatten(block_on(handle.edit(0, Edit::Add(mark(rect), None)))).unwrap();
    }
    let markup = flatten(block_on(handle.markup(0))).unwrap();
    assert_eq!(
        markup
            .annotations
            .iter()
            .filter(|annotation| annotation.kind == Kind::Redact)
            .count(),
        3
    );
    // Marks alone hide nothing yet, and survive a save.
    save(&handle, &path);
    assert!(contains(&std::fs::read(&path).unwrap(), b"SECRET-1234"));

    let applied = edit(&handle, PageEdit::ApplyRedactions);
    assert_eq!(applied.outcome, PageOutcome::Redacted(3));
    save(&handle, &path);

    let bytes = std::fs::read(&path).unwrap();
    assert!(!contains(&bytes, b"SECRET"), "the file still has the text");
    assert!(
        !contains(&bytes, b"%%EOF\n%"),
        "the file was appended to, not rewritten"
    );
    let streams = decoded_streams(&path);
    let red_run: Vec<u8> = [255u8, 0, 0].repeat(8);
    for stream in &streams {
        assert!(!contains(stream, b"SECRET"), "a stream still has the text");
        assert!(!contains(stream, &red_run), "the image's pixels survived");
        assert!(!contains(stream, b"0 0 1 rg"), "the drawing survived");
    }
    assert!(
        streams
            .iter()
            .any(|stream| contains(stream, b"Visible text")),
        "text outside the marks was removed"
    );
    // The marks are gone; black boxes are drawn in their place.
    let reopened = MupdfEngine.open(&path).unwrap();
    assert!(reopened.annotations(0).unwrap().is_empty());
    let display = reopened.display(0).unwrap();
    let text: String = display
        .text()
        .unwrap()
        .lines
        .iter()
        .flat_map(|line| line.chars.iter().map(|char| char.character))
        .collect();
    assert_eq!(text, "Visible text");
}

#[test]
fn exports_encrypt_and_reduce() {
    let dir = tempfile::tempdir().unwrap();
    // A large gradient image on a four inch page.
    let size = 1000usize;
    let mut pixels = Vec::with_capacity(size * size * 3);
    for y in 0..size {
        for x in 0..size {
            pixels.extend_from_slice(&[(x * 255 / size) as u8, (y * 255 / size) as u8, 128]);
        }
    }
    let bytes = common::assemble_pdf(&[
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 288 288] /Contents 4 0 R \
/Resources << /XObject << /Im1 5 0 R >> >> >>"
            .to_vec(),
        common::stream_object("", b"q 288 0 0 288 0 0 cm /Im1 Do Q\n"),
        common::stream_object(
            &format!(
                "/Type /XObject /Subtype /Image /Width {size} /Height {size} \
/ColorSpace /DeviceRGB /BitsPerComponent 8"
            ),
            &pixels,
        ),
    ]);
    let (handle, _, path) = open(&dir, &bytes);
    let export = |options: ExportOptions, name: &str| {
        let target = dir.path().join(name);
        let destination = target.clone();
        flatten(block_on(handle.export(
            options,
            Box::new(move |bytes| std::fs::write(&destination, bytes).map_err(|e| e.to_string())),
        )))
        .unwrap();
        target
    };

    let plain = export(ExportOptions::default(), "plain.pdf");
    let reduced = export(
        ExportOptions {
            reduce: Some(Reduce::DEFAULT),
            ..ExportOptions::default()
        },
        "reduced.pdf",
    );
    let plain_size = std::fs::metadata(&plain).unwrap().len();
    let reduced_size = std::fs::metadata(&reduced).unwrap().len();
    assert!(
        reduced_size * 4 < plain_size,
        "reduced {reduced_size} bytes, plain {plain_size}"
    );
    let document = mupdf::pdf::PdfDocument::open(reduced.to_str().unwrap()).unwrap();
    let page = document.load_pdf_page(0).unwrap();
    let image = &page.images().unwrap()[0];
    // Four inches at 150 dpi.
    assert_eq!((image.width, image.height), (600, 600));
    // The open document keeps its images.
    let original = mupdf::pdf::PdfDocument::open(path.to_str().unwrap()).unwrap();
    assert_eq!(
        original.load_pdf_page(0).unwrap().images().unwrap()[0].width,
        1000
    );
    let pixel = |path: &Path| {
        let display = MupdfEngine.open(path).unwrap().display(0).unwrap();
        let bitmap = display
            .render(
                1.0,
                prev_pdf::geometry::PixelRect {
                    x: 0,
                    y: 0,
                    width: 288,
                    height: 288,
                },
            )
            .unwrap();
        let at = (144 * 288 + 216) * 4;
        bitmap.pixels[at..at + 3].to_vec()
    };
    let (before, after) = (pixel(&plain), pixel(&reduced));
    for (a, b) in before.iter().zip(&after) {
        assert!(a.abs_diff(*b) < 12, "{before:?} became {after:?}");
    }

    let locked = export(
        ExportOptions {
            password: Some("open sesame".into()),
            ..ExportOptions::default()
        },
        "locked.pdf",
    );
    let mut document = MupdfEngine.open(&locked).unwrap();
    assert!(!document.authenticate(""));
    assert!(document.needs_password());
    assert!(!document.authenticate("wrong"));
    assert!(document.authenticate("open sesame"));
    assert_eq!(document.page_count().unwrap(), 1);
    assert!(!contains(&std::fs::read(&locked).unwrap(), b"Im1 Do"));
}

#[test]
fn flattened_exports_draw_annotations_into_the_page() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, _) = open(&dir, &common::form_fixture());
    let mut square = Annotation::new(
        new_id(),
        Kind::Square,
        Rect::new(300.0, 300.0, 400.0, 400.0),
    );
    square.style.fill = Some(prev_pdf::annotation::Rgb::new(0.0, 0.0, 1.0));
    flatten_add(&handle, square);
    flatten_add(&handle, mark(Rect::new(72.0, 80.0, 300.0, 100.0)));
    let markup = flatten_markup(&handle);
    let name = markup
        .fields
        .iter()
        .find(|field| field.name == "name")
        .unwrap()
        .id;
    flatten_block(handle.edit(
        0,
        Edit::SetField {
            id: name,
            value: "Ada".into(),
        },
    ));
    let target = dir.path().join("flat.pdf");
    let destination = target.clone();
    flatten(block_on(handle.export(
        ExportOptions {
            flatten: true,
            ..ExportOptions::default()
        },
        Box::new(move |bytes| std::fs::write(&destination, bytes).map_err(|e| e.to_string())),
    )))
    .unwrap();
    let flat = MupdfEngine.open(&target).unwrap();
    assert!(
        flat.annotations(0).unwrap().is_empty(),
        "annotations remain"
    );
    assert!(flat.fields(0).unwrap().is_empty(), "form fields remain");
    // The square is part of the page now, and so is the field's value.
    let display = flat.display(0).unwrap();
    let bitmap = display
        .render(
            1.0,
            prev_pdf::geometry::PixelRect {
                x: 0,
                y: 0,
                width: 612,
                height: 792,
            },
        )
        .unwrap();
    let at = (350 * 612 + 350) * 4;
    assert_eq!(&bitmap.pixels[at..at + 3], &[0, 0, 255]);
    let text: String = display
        .text()
        .unwrap()
        .lines
        .iter()
        .flat_map(|line| line.chars.iter().map(|char| char.character))
        .collect();
    assert!(text.contains("Ada"), "{text}");
    // The unapplied redaction mark is not drawn: the text under it shows.
    assert!(text.contains("Annotate this line"), "{text}");
}

fn flatten_add(handle: &DocumentHandle, annotation: Annotation) {
    flatten_block(handle.edit(0, Edit::Add(annotation, None)));
}

fn flatten_markup(handle: &DocumentHandle) -> prev_pdf::worker::PageMarkup {
    flatten(block_on(handle.markup(0))).unwrap()
}

fn flatten_block<T>(
    receiver: futures::channel::oneshot::Receiver<prev_pdf::engine::Result<T>>,
) -> T {
    flatten(block_on(receiver)).unwrap()
}

#[test]
fn lifting_an_annotation_splits_the_page() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _, _) = open(&dir, &common::navigation_fixture());
    let mut square = Annotation::new(
        new_id(),
        Kind::Square,
        Rect::new(300.0, 300.0, 400.0, 400.0),
    );
    square.style.fill = Some(prev_pdf::annotation::Rgb::new(0.0, 0.0, 1.0));
    flatten(block_on(handle.edit(0, Edit::Add(square.clone(), None)))).unwrap();
    let lifted = flatten(block_on(handle.lift(0, square.id.clone()))).unwrap();
    let pixel = |display: &std::sync::Arc<dyn prev_pdf::engine::PageDisplay>, x: i32, y: i32| {
        let bitmap = display
            .render(
                1.0,
                prev_pdf::geometry::PixelRect {
                    x,
                    y,
                    width: 1,
                    height: 1,
                },
            )
            .unwrap();
        [
            bitmap.pixels[0],
            bitmap.pixels[1],
            bitmap.pixels[2],
            bitmap.pixels[3],
        ]
    };
    // Without it, the page shows through where the square was; the text
    // at the top is still there.
    assert_eq!(pixel(&lifted.without, 350, 350), [255, 255, 255, 255]);
    let text = lifted.without.text().unwrap();
    assert!(!text.lines.is_empty());
    // Alone, the square is opaque blue on a transparent page.
    assert_eq!(pixel(&lifted.alone, 350, 350), [0, 0, 255, 255]);
    assert_eq!(pixel(&lifted.alone, 100, 100)[3], 0);
    assert!(
        lifted.alone.text().unwrap().lines.is_empty(),
        "no page content"
    );
    // The document itself is unchanged.
    let markup = flatten(block_on(handle.markup(0))).unwrap();
    assert!(
        markup
            .annotations
            .iter()
            .any(|annotation| annotation.id == square.id)
    );
}

#[test]
fn images_become_pages_that_take_markup() {
    // A 4 x 2 image, left half red and right half green, on a 40 x 20 page.
    let mut pixels = Vec::new();
    for _ in 0..2 {
        for x in 0..4 {
            pixels.extend_from_slice(if x < 2 {
                &[255, 0, 0, 255]
            } else {
                &[0, 255, 0, 255]
            });
        }
    }
    let image = prev_pdf::engine::Bitmap {
        width: 4,
        height: 2,
        pixels,
    };
    let bytes = prev_pdf::image_document(&image, Size::new(40.0, 20.0)).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let (handle, info, _) = open(&dir, &bytes);
    assert_eq!(info.page_sizes, vec![Size::new(40.0, 20.0)]);
    let pixel = |display: &Arc<dyn prev_pdf::engine::PageDisplay>, x: i32, y: i32| {
        let bitmap = display
            .render(
                1.0,
                prev_pdf::geometry::PixelRect {
                    x,
                    y,
                    width: 1,
                    height: 1,
                },
            )
            .unwrap();
        [
            bitmap.pixels[0],
            bitmap.pixels[1],
            bitmap.pixels[2],
            bitmap.pixels[3],
        ]
    };
    let page = flatten(block_on(handle.display(0))).unwrap();
    assert_eq!(pixel(&page, 5, 10), [255, 0, 0, 255]);
    assert_eq!(pixel(&page, 35, 10), [0, 255, 0, 255]);
    // The markup layer holds the annotations and nothing of the image.
    let mut square = Annotation::new(new_id(), Kind::Square, Rect::new(2.0, 2.0, 12.0, 18.0));
    square.style.fill = Some(prev_pdf::annotation::Rgb::new(0.0, 0.0, 1.0));
    flatten(block_on(handle.edit(0, Edit::Add(square, None)))).unwrap();
    let layer = flatten(block_on(handle.annotation_layer(0))).unwrap();
    assert_eq!(pixel(&layer, 7, 10), [0, 0, 255, 255]);
    assert_eq!(pixel(&layer, 35, 10)[3], 0);
}

#[test]
fn image_pages_fit_the_image_in_the_middle() {
    // A red 2 x 1 image on a 40 x 40 page: 40 x 20, with white above and
    // below.
    let image = prev_pdf::engine::Bitmap {
        width: 2,
        height: 1,
        pixels: [255, 0, 0, 255].repeat(2),
    };
    let bytes = prev_pdf::image_page_document(&image, Size::new(40.0, 40.0)).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let (handle, info, _) = open(&dir, &bytes);
    assert_eq!(info.page_sizes, vec![Size::new(40.0, 40.0)]);
    let page = flatten(block_on(handle.display(0))).unwrap();
    let pixel = |x: i32, y: i32| {
        let bitmap = page
            .render(
                1.0,
                prev_pdf::geometry::PixelRect {
                    x,
                    y,
                    width: 1,
                    height: 1,
                },
            )
            .unwrap();
        bitmap.pixels[..3].to_vec()
    };
    assert_eq!(pixel(20, 5), [255, 255, 255]);
    assert_eq!(pixel(20, 20), [255, 0, 0]);
    assert_eq!(pixel(20, 35), [255, 255, 255]);
}
