//! Checks that the MuPDF bindings cover the PDF features prev relies on.

mod common;

use common::{assemble_pdf, stream_object};

use mupdf::pdf::document::Encryption;
use mupdf::pdf::{
    InsertPdfOptions, PdfAnnotationType, PdfDocument, PdfRedactImageMethod, PdfRedactLineArtMethod,
    PdfRedactOptions, PdfRedactTextMethod, PdfWriteOptions,
};
use mupdf::{
    Buffer, Colorspace, Image, Matrix, Pixmap, Point, Quad, Rect, Size, TextExtractOptions,
};

const PAGE_HEIGHT: f32 = 792.0;

/// One letter page with a heading, a secret line, a blue vector rectangle
/// and a 4x4 red image. In MuPDF (top-left) coordinates:
/// secret text around y 272..297, rectangle (72, 442)-(172, 492),
/// image (300, 442)-(400, 542).
fn fixture_pdf() -> Vec<u8> {
    let content = b"BT /F1 24 Tf 72 700 Td (Public heading) Tj ET\n\
BT /F1 24 Tf 72 500 Td (SECRET 4242) Tj ET\n\
0 0 1 rg 72 300 100 50 re f\n\
q 100 0 0 100 300 250 cm /Im1 Do Q\n";
    let red_pixels: Vec<u8> = std::iter::repeat_n([0xff, 0x00, 0x00], 16)
        .flatten()
        .collect();
    assemble_pdf(&[
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
/Resources << /Font << /F1 5 0 R >> /XObject << /Im1 6 0 R >> >> >>"
            .to_vec(),
        stream_object("", content),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
        stream_object(
            "/Type /XObject /Subtype /Image /Width 4 /Height 4 /ColorSpace /DeviceRGB /BitsPerComponent 8",
            &red_pixels,
        ),
    ])
}

fn page_text(doc: &PdfDocument, page_index: i32) -> String {
    doc.load_page(page_index)
        .unwrap()
        .text(TextExtractOptions::default())
        .unwrap()
}

fn full_rewrite_options() -> PdfWriteOptions {
    let mut options = PdfWriteOptions::default();
    options.set_garbage_level(4).set_decompress(true);
    options
}

fn save_to_vec(doc: &PdfDocument, options: PdfWriteOptions) -> Vec<u8> {
    let mut bytes = Vec::new();
    doc.write_to_with_options(&mut bytes, options).unwrap();
    bytes
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn render(doc: &PdfDocument, page_index: i32) -> Pixmap {
    doc.load_page(page_index)
        .unwrap()
        .to_pixmap(&Matrix::IDENTITY, &Colorspace::device_rgb(), false, true)
        .unwrap()
}

fn rgb_at(pixmap: &Pixmap, x: usize, y: usize) -> [u8; 3] {
    let offset = y * pixmap.stride() as usize + x * pixmap.n() as usize;
    let samples = pixmap.samples();
    [samples[offset], samples[offset + 1], samples[offset + 2]]
}

fn is_red(rgb: [u8; 3]) -> bool {
    rgb[0] > 200 && rgb[1] < 60 && rgb[2] < 60
}

#[test]
fn fixture_renders_as_expected() {
    let doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    let text = page_text(&doc, 0);
    assert!(text.contains("Public heading"));
    assert!(text.contains("SECRET 4242"));
    let rewritten = save_to_vec(&doc, full_rewrite_options());
    assert!(
        contains(&rewritten, b"SECRET 4242"),
        "byte checks below would be vacuous"
    );
    let pixmap = render(&doc, 0);
    assert!(is_red(rgb_at(&pixmap, 350, 490)));
    assert_eq!(rgb_at(&pixmap, 120, 470), [0, 0, 255]);
}

#[test]
fn redaction_removes_text_line_art_and_images() {
    let doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    {
        let mut page = doc.load_pdf_page(0).unwrap();
        page.add_redact_annotation(Rect::new(60.0, 260.0, 400.0, 305.0))
            .unwrap();
        page.add_redact_annotation(Rect::new(60.0, 430.0, 190.0, 500.0))
            .unwrap();
        page.add_redact_annotation(Rect::new(290.0, 430.0, 410.0, 550.0))
            .unwrap();
        let applied = page
            .apply_redactions_with_options(PdfRedactOptions {
                black_boxes: false,
                image_method: PdfRedactImageMethod::Remove,
                line_art: PdfRedactLineArtMethod::RemoveIfCovered,
                text: PdfRedactTextMethod::Remove,
            })
            .unwrap();
        assert!(applied);
        assert_eq!(
            page.annotations().count(),
            0,
            "redact annotations are consumed"
        );
    }

    let bytes = save_to_vec(&doc, full_rewrite_options());
    assert!(
        !contains(&bytes, b"SECRET"),
        "secret text left in file bytes"
    );
    assert!(
        !contains(&bytes, b"4242"),
        "secret digits left in file bytes"
    );
    assert!(!contains(&bytes, b"/Subtype /Image") && !contains(&bytes, b"/Subtype/Image"));

    let reopened = PdfDocument::from_bytes(&bytes).unwrap();
    let text = page_text(&reopened, 0);
    assert!(
        text.contains("Public heading"),
        "text outside the box survives"
    );
    assert!(!text.contains("SECRET"));
    let page = reopened.load_pdf_page(0).unwrap();
    assert!(page.images().unwrap().is_empty());
    let fills_in_box = page
        .drawings()
        .unwrap()
        .into_iter()
        .filter(|drawing| {
            drawing.rect.x0 < 190.0 && drawing.rect.y0 > 430.0 && drawing.rect.y1 < 500.0
        })
        .count();
    assert_eq!(fills_in_box, 0, "covered vector art removed");
    let pixmap = render(&reopened, 0);
    assert_eq!(rgb_at(&pixmap, 120, 470), [255, 255, 255]);
    assert_eq!(rgb_at(&pixmap, 350, 490), [255, 255, 255]);
}

#[test]
fn partial_image_redaction_clears_only_covered_pixels() {
    let doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    {
        let mut page = doc.load_pdf_page(0).unwrap();
        page.add_redact_annotation(Rect::new(290.0, 430.0, 352.0, 550.0))
            .unwrap();
        page.apply_redactions_with_options(PdfRedactOptions {
            image_method: PdfRedactImageMethod::Pixels,
            ..PdfRedactOptions::mupdf_default()
        })
        .unwrap();
    }
    let reopened = PdfDocument::from_bytes(&save_to_vec(&doc, full_rewrite_options())).unwrap();
    assert_eq!(
        reopened.load_pdf_page(0).unwrap().images().unwrap().len(),
        1
    );
    let pixmap = render(&reopened, 0);
    assert!(!is_red(rgb_at(&pixmap, 312, 490)), "covered pixels cleared");
    assert!(is_red(rgb_at(&pixmap, 390, 490)), "uncovered pixels kept");
}

#[test]
fn markup_annotations_round_trip_with_incremental_save() {
    let original = fixture_pdf();
    let doc = PdfDocument::from_bytes(&original).unwrap();
    {
        let mut page = doc.load_pdf_page(0).unwrap();
        let heading = Quad::from(Rect::new(72.0, 72.0, 240.0, 97.0));
        page.add_highlight_annotation(heading.clone()).unwrap();
        page.add_underline_annotation(heading.clone()).unwrap();
        page.add_strikeout_annotation(heading.clone()).unwrap();
        page.add_squiggly_annotation(heading).unwrap();
        page.add_ink_annotation([[
            Point::new(80.0, 600.0),
            Point::new(120.0, 640.0),
            Point::new(160.0, 600.0),
        ]])
        .unwrap();
        page.add_square_annotation(Rect::new(400.0, 100.0, 500.0, 160.0))
            .unwrap();
        page.add_circle_annotation(Rect::new(400.0, 200.0, 500.0, 260.0))
            .unwrap();
        page.add_line_annotation(Point::new(400.0, 300.0), Point::new(500.0, 340.0))
            .unwrap();
        page.add_polygon_annotation([
            Point::new(420.0, 600.0),
            Point::new(480.0, 600.0),
            Point::new(450.0, 650.0),
        ])
        .unwrap();
        page.add_free_text_annotation(Rect::new(72.0, 700.0, 300.0, 740.0), "Text box")
            .unwrap();
        page.add_text_annotation(Rect::new(560.0, 72.0, 580.0, 92.0), "A note")
            .unwrap();
        page.update().unwrap();
    }
    assert!(doc.can_be_saved_incrementally());
    let mut options = PdfWriteOptions::default();
    options.set_incremental(true);
    let bytes = save_to_vec(&doc, options);
    assert!(
        bytes.starts_with(&original),
        "incremental save appends to the original bytes"
    );

    let reopened = PdfDocument::from_bytes(&bytes).unwrap();
    let page = reopened.load_pdf_page(0).unwrap();
    let mut types: Vec<_> = page
        .annotations()
        .map(|annot| annot.r#type().unwrap())
        .filter(|kind| *kind != PdfAnnotationType::Popup)
        .collect();
    types.sort_by_key(|kind| *kind as i32);
    let mut expected = vec![
        PdfAnnotationType::Highlight,
        PdfAnnotationType::Underline,
        PdfAnnotationType::StrikeOut,
        PdfAnnotationType::Squiggly,
        PdfAnnotationType::Ink,
        PdfAnnotationType::Square,
        PdfAnnotationType::Circle,
        PdfAnnotationType::Line,
        PdfAnnotationType::Polygon,
        PdfAnnotationType::FreeText,
        PdfAnnotationType::Text,
    ];
    expected.sort_by_key(|kind| *kind as i32);
    assert_eq!(types, expected);
    for annot in page.annotations() {
        assert!(
            annot.object().get_dict("AP").unwrap().is_some(),
            "{:?} has an appearance stream for other viewers",
            annot.r#type().unwrap()
        );
    }
}

/// Visual signatures from an image need a stamp with a custom appearance.
/// The bindings lack `pdf_set_annot_stamp_image`, so build /AP by hand.
#[test]
fn image_stamp_with_hand_built_appearance_survives_save() {
    let mut doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    let mut green = Pixmap::new_with_w_h(&Colorspace::device_rgb(), 8, 8, false).unwrap();
    for chunk in green.samples_mut().chunks_mut(3) {
        chunk.copy_from_slice(&[0x00, 0xc0, 0x00]);
    }
    let image = Image::from_pixmap(&green).unwrap();
    let image_ref = doc.add_image(&image).unwrap();

    let stamp_rect = Rect::new(72.0, 600.0, 172.0, 650.0);
    let mut xobjects = doc.new_dict().unwrap();
    xobjects.dict_put("Sig", image_ref).unwrap();
    let mut resources = doc.new_dict().unwrap();
    resources.dict_put("XObject", xobjects).unwrap();
    let mut form = doc.new_dict().unwrap();
    form.dict_put("Type", doc.new_name("XObject").unwrap())
        .unwrap();
    form.dict_put("Subtype", doc.new_name("Form").unwrap())
        .unwrap();
    let mut bbox = doc.new_array().unwrap();
    for value in [0.0, 0.0, 100.0, 50.0] {
        bbox.array_push(doc.new_real(value).unwrap()).unwrap();
    }
    form.dict_put("BBox", bbox).unwrap();
    form.dict_put("Resources", resources).unwrap();
    let appearance = doc
        .add_stream(
            &Buffer::from_bytes(b"q 100 0 0 50 0 0 cm /Sig Do Q").unwrap(),
            Some(&form),
            false,
        )
        .unwrap();

    {
        let mut page = doc.load_pdf_page(0).unwrap();
        let stamp = page.add_stamp_annotation(stamp_rect, "Signature").unwrap();
        let mut ap = doc.new_dict().unwrap();
        ap.dict_put("N", appearance).unwrap();
        stamp.object().dict_put("AP", ap).unwrap();
        page.update().unwrap();
    }

    let reopened = PdfDocument::from_bytes(&save_to_vec(&doc, full_rewrite_options())).unwrap();
    let pixmap = render(&reopened, 0);
    let center = rgb_at(&pixmap, 122, 625);
    assert!(
        center[1] > 150 && center[0] < 60,
        "stamp drawn with our appearance, got {center:?}"
    );
}

#[test]
fn page_operations() {
    let mut doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    doc.new_page(Size::new(612.0, PAGE_HEIGHT)).unwrap();
    doc.duplicate_page(0).unwrap();
    assert_eq!(doc.page_count().unwrap(), 3);
    assert!(
        page_text(&doc, 2).trim().is_empty(),
        "blank page is now last"
    );

    doc.move_page(2, 0).unwrap();
    assert!(page_text(&doc, 0).trim().is_empty());
    assert!(page_text(&doc, 1).contains("Public heading"));

    doc.load_pdf_page(1).unwrap().set_rotation(90).unwrap();
    assert_eq!(doc.load_pdf_page(1).unwrap().rotation().unwrap(), 90);

    doc.delete_page(0).unwrap();
    assert_eq!(doc.page_count().unwrap(), 2);

    let other = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    doc.insert_pdf(&other, InsertPdfOptions::default()).unwrap();
    assert_eq!(doc.page_count().unwrap(), 3);

    let reopened = PdfDocument::from_bytes(&save_to_vec(&doc, full_rewrite_options())).unwrap();
    assert_eq!(reopened.page_count().unwrap(), 3);
    assert_eq!(reopened.load_pdf_page(0).unwrap().rotation().unwrap(), 90);
    assert!(page_text(&reopened, 2).contains("SECRET 4242"));
}

#[test]
fn aes256_encryption_requires_password() {
    let doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    let mut options = full_rewrite_options();
    options
        .set_decompress(false)
        .set_encryption(Encryption::Aes256)
        .set_user_password("open-sesame")
        .set_owner_password("owner-secret");
    let bytes = save_to_vec(&doc, options);
    assert!(!contains(&bytes, b"SECRET"));

    let mut reopened = PdfDocument::from_bytes(&bytes).unwrap();
    assert!(reopened.needs_password().unwrap());
    assert!(!reopened.authenticate("wrong").unwrap());
    assert!(reopened.authenticate("open-sesame").unwrap());
    assert!(page_text(&reopened, 0).contains("SECRET 4242"));
}

#[test]
fn display_lists_render_on_worker_threads() {
    let doc = PdfDocument::from_bytes(&fixture_pdf()).unwrap();
    let list = doc.load_page(0).unwrap().to_display_list(true).unwrap();
    let list = std::sync::Arc::new(list);
    let workers: Vec<_> = (0..4)
        .map(|worker| {
            let list = list.clone();
            std::thread::spawn(move || {
                let scale = 1.0 + worker as f32;
                let pixmap = list
                    .to_pixmap(
                        &Matrix::new_scale(scale, scale),
                        &Colorspace::device_rgb(),
                        false,
                    )
                    .unwrap();
                (
                    pixmap.width(),
                    is_red(rgb_at(
                        &pixmap,
                        (350.0 * scale) as usize,
                        (490.0 * scale) as usize,
                    )),
                )
            })
        })
        .collect();
    for (worker, handle) in workers.into_iter().enumerate() {
        let (width, saw_image) = handle.join().unwrap();
        assert_eq!(width, (612.0 * (1.0 + worker as f32)) as u32);
        assert!(saw_image);
    }
}
