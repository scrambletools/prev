//! Annotations and form fields written through the engine read back the
//! same in MuPDF, and render in Poppler, so other viewers see them.

mod common;

use std::path::Path;
use std::sync::Arc;

use common::{assemble_pdf, stream_object};
use prev_pdf::annotation::{
    Annotation, Field, FieldKind, Kind, LineEnd, Rgb, StampContent, TextMarkup, new_id,
};
use prev_pdf::engine::{Bitmap, Document, Engine};
use prev_pdf::geometry::{PixelRect, Point, Quad, Rect};
use prev_pdf::mupdf_engine::MupdfEngine;

/// One letter page with a line of text at the top, and a form with a text
/// field, a checkbox, two radio buttons and a drop-down menu.
fn form_fixture() -> Vec<u8> {
    let content = b"BT /F1 24 Tf 72 700 Td (Annotate this line) Tj ET\n";
    assemble_pdf(&[
        // 1
        b"<< /Type /Catalog /Pages 2 0 R /AcroForm << /Fields [6 0 R 7 0 R 8 0 R 11 0 R] \
/DA (/Helv 12 Tf 0 g) /DR << /Font << /Helv 5 0 R >> >> >> >>"
            .to_vec(),
        // 2
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        // 3
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
/Resources << /Font << /F1 5 0 R >> >> /Annots [6 0 R 7 0 R 9 0 R 10 0 R 11 0 R] >>"
            .to_vec(),
        // 4
        stream_object("", content),
        // 5
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
        // 6: text field
        b"<< /Type /Annot /Subtype /Widget /FT /Tx /T (name) /Rect [72 600 300 624] \
/DA (/Helv 12 Tf 0 g) /P 3 0 R /F 4 >>"
            .to_vec(),
        // 7: checkbox
        b"<< /Type /Annot /Subtype /Widget /FT /Btn /T (agree) /Rect [72 560 90 578] /P 3 0 R /F 4 \
/V /Off /AS /Off /MK << /CA (4) >> /DA (/ZaDb 0 Tf 0 g) \
/AP << /N << /Yes 12 0 R /Off 13 0 R >> >> >>"
            .to_vec(),
        // 8: radio group
        b"<< /FT /Btn /Ff 49152 /T (size) /V /Off /Kids [9 0 R 10 0 R] >>".to_vec(),
        // 9
        b"<< /Type /Annot /Subtype /Widget /Parent 8 0 R /Rect [72 520 90 538] /P 3 0 R /F 4 \
/AS /Off /MK << /CA (l) >> /DA (/ZaDb 0 Tf 0 g) /AP << /N << /Small 12 0 R /Off 13 0 R >> >> >>"
            .to_vec(),
        // 10
        b"<< /Type /Annot /Subtype /Widget /Parent 8 0 R /Rect [120 520 138 538] /P 3 0 R /F 4 \
/AS /Off /MK << /CA (l) >> /DA (/ZaDb 0 Tf 0 g) /AP << /N << /Large 12 0 R /Off 13 0 R >> >> >>"
            .to_vec(),
        // 11: drop-down menu
        b"<< /Type /Annot /Subtype /Widget /FT /Ch /Ff 131072 /T (color) /Rect [72 470 200 494] \
/Opt [(Red) (Green) (Blue)] /V (Red) /DA (/Helv 12 Tf 0 g) /P 3 0 R /F 4 >>"
            .to_vec(),
        // 12
        stream_object("/BBox [0 0 18 18]", b"0 g 3 3 12 12 re f"),
        // 13
        stream_object("/BBox [0 0 18 18]", b""),
    ])
}

fn open(bytes: &[u8]) -> (tempfile::TempDir, Box<dyn Document>) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fixture.pdf");
    std::fs::write(&path, bytes).unwrap();
    let document = MupdfEngine.open(&path).unwrap();
    (dir, document)
}

fn reopen(document: &mut Box<dyn Document>) -> (tempfile::TempDir, Box<dyn Document>, Vec<u8>) {
    let bytes = document.save().unwrap();
    let (dir, reopened) = open(&bytes);
    (dir, reopened, bytes)
}

const RED: Rgb = Rgb::new(1.0, 0.0, 0.0);
const BLUE: Rgb = Rgb::new(0.0, 0.0, 1.0);

fn every_kind() -> Vec<Annotation> {
    let line = Quad::from(Rect::new(72.0, 72.0, 300.0, 97.0));
    let mut annotations = vec![
        Annotation::new(
            new_id(),
            Kind::Markup {
                style: TextMarkup::Highlight,
                quads: vec![line],
            },
            line.bounds(),
        ),
        Annotation::new(
            new_id(),
            Kind::Markup {
                style: TextMarkup::StrikeOut,
                quads: vec![line],
            },
            line.bounds(),
        ),
        Annotation::new(
            new_id(),
            Kind::Ink(vec![vec![
                Point::new(80.0, 300.0),
                Point::new(120.0, 340.0),
                Point::new(160.0, 300.0),
            ]]),
            Rect::new(80.0, 300.0, 160.0, 340.0),
        ),
        Annotation::new(
            new_id(),
            Kind::Square,
            Rect::new(400.0, 100.0, 500.0, 160.0),
        ),
        Annotation::new(
            new_id(),
            Kind::Circle,
            Rect::new(400.0, 200.0, 500.0, 260.0),
        ),
        Annotation::new(
            new_id(),
            Kind::Line {
                start: Point::new(400.0, 300.0),
                end: Point::new(500.0, 340.0),
                endings: (LineEnd::None, LineEnd::ClosedArrow),
            },
            Rect::new(400.0, 300.0, 500.0, 340.0),
        ),
        Annotation::new(
            new_id(),
            Kind::Polygon(vec![
                Point::new(420.0, 600.0),
                Point::new(480.0, 600.0),
                Point::new(450.0, 650.0),
            ]),
            Rect::new(420.0, 600.0, 480.0, 650.0),
        ),
        Annotation::new(
            new_id(),
            Kind::FreeText,
            Rect::new(72.0, 700.0, 300.0, 740.0),
        ),
        Annotation::new(new_id(), Kind::Note, Rect::new(560.0, 72.0, 580.0, 92.0)),
    ];
    for annotation in &mut annotations {
        annotation.contents = format!("{:?}", std::mem::discriminant(&annotation.kind));
        annotation.subject = Some("Test".into());
    }
    annotations[3].style.fill = Some(BLUE);
    annotations[3].style.dashed = true;
    annotations[7].contents = "Hello from prev".into();
    annotations[7].style.font_size = 18.0;
    annotations[7].style.text_color = BLUE;
    annotations
}

#[test]
fn every_kind_round_trips() {
    let (_dir, mut document) = open(&form_fixture());
    let written = every_kind();
    for annotation in &written {
        document.add_annotation(0, annotation, None).unwrap();
    }
    let (_dir, reopened, _) = reopen(&mut document);
    let read = reopened.annotations(0).unwrap();
    assert_eq!(read.len(), written.len());
    for expected in &written {
        let found = read
            .iter()
            .find(|annotation| annotation.id == expected.id)
            .unwrap_or_else(|| panic!("{:?} missing", expected.kind));
        assert_eq!(
            std::mem::discriminant(&found.kind),
            std::mem::discriminant(&expected.kind)
        );
        assert_eq!(found.contents, expected.contents);
        assert_eq!(found.subject.as_deref(), Some("Test"));
        if !matches!(expected.kind, Kind::Markup { .. } | Kind::Note) {
            assert_eq!(found.style.color, Some(Rgb::new(0.9, 0.1, 0.1)));
        }
    }
    let square = read.iter().find(|a| a.kind == Kind::Square).unwrap();
    assert_eq!(square.style.fill, Some(BLUE));
    assert!(square.style.dashed);
    let text = read.iter().find(|a| a.kind == Kind::FreeText).unwrap();
    assert_eq!(text.style.font_size, 18.0);
    assert_eq!(text.style.text_color, BLUE);
    let line = read
        .iter()
        .find(|a| matches!(a.kind, Kind::Line { .. }))
        .unwrap();
    let Kind::Line { endings, .. } = line.kind else {
        unreachable!()
    };
    assert_eq!(endings.1, LineEnd::ClosedArrow);
}

#[test]
fn updates_move_and_restyle() {
    let (_dir, mut document) = open(&form_fixture());
    let square = Annotation::new(
        new_id(),
        Kind::Square,
        Rect::new(100.0, 100.0, 200.0, 150.0),
    );
    document.add_annotation(0, &square, None).unwrap();
    let mut moved = square.translated(50.0, 200.0);
    moved.style.color = Some(BLUE);
    moved.style.line_width = 5.0;
    document.update_annotation(0, &moved, None).unwrap();
    let read = document.annotations(0).unwrap();
    let found = read.iter().find(|a| a.id == square.id).unwrap();
    let close = |a: f32, b: f32| (a - b).abs() < 3.0;
    assert!(
        close(found.rect.x0, 150.0) && close(found.rect.y0, 300.0),
        "{:?}",
        found.rect
    );
    assert_eq!(found.style.color, Some(BLUE));
    assert_eq!(found.style.line_width, 5.0);
}

#[test]
fn removal_can_be_undone_exactly() {
    let (_dir, mut document) = open(&form_fixture());
    let mut circle = Annotation::new(
        new_id(),
        Kind::Circle,
        Rect::new(100.0, 100.0, 200.0, 150.0),
    );
    circle.contents = "keep me".into();
    document.add_annotation(0, &circle, None).unwrap();
    let removed = document.remove_annotation(0, &circle.id).unwrap();
    assert!(document.annotations(0).unwrap().is_empty());
    document.restore_annotation(0, &removed).unwrap();
    let (_dir, reopened, _) = reopen(&mut document);
    let read = reopened.annotations(0).unwrap();
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].id, circle.id);
    assert_eq!(read[0].contents, "keep me");
}

fn signature_bitmap() -> Arc<Bitmap> {
    // Opaque green on the left half, transparent on the right.
    let (width, height) = (40, 20);
    let mut pixels = Vec::new();
    for _ in 0..height {
        for x in 0..width {
            pixels.extend_from_slice(if x < width / 2 {
                &[0, 160, 0, 255]
            } else {
                &[0, 0, 0, 0]
            });
        }
    }
    Arc::new(Bitmap {
        width,
        height,
        pixels,
    })
}

fn pixel(document: &dyn Document, x: f32, y: f32) -> [u8; 4] {
    let display = document.display(0).unwrap();
    let bitmap = display
        .render(
            1.0,
            PixelRect {
                x: x as i32,
                y: y as i32,
                width: 1,
                height: 1,
            },
        )
        .unwrap();
    bitmap.pixels[..4].try_into().unwrap()
}

#[test]
fn image_stamps_keep_transparency_and_survive_moves() {
    let (_dir, mut document) = open(&form_fixture());
    let mut stamp = Annotation::new(new_id(), Kind::Stamp, Rect::new(100.0, 300.0, 300.0, 400.0));
    stamp.subject = Some("Signature".into());
    let content = StampContent::Image {
        image: signature_bitmap(),
        round: false,
        border: None,
    };
    document.add_annotation(0, &stamp, Some(&content)).unwrap();
    let green = pixel(document.as_ref(), 150.0, 350.0);
    assert!(
        green[1] > 120 && green[0] < 60,
        "opaque half drawn: {green:?}"
    );
    assert_eq!(pixel(document.as_ref(), 250.0, 350.0), [255, 255, 255, 255]);

    let moved = stamp.translated(0.0, 200.0);
    document.update_annotation(0, &moved, None).unwrap();
    let (_dir, reopened, _) = reopen(&mut document);
    let green = pixel(reopened.as_ref(), 150.0, 550.0);
    assert!(
        green[1] > 120 && green[0] < 60,
        "appearance kept after a move: {green:?}"
    );
    assert_eq!(pixel(reopened.as_ref(), 150.0, 350.0), [255, 255, 255, 255]);
    let read = reopened.annotations(0).unwrap();
    assert_eq!(read[0].subject.as_deref(), Some("Signature"));
}

#[test]
fn masks_darken_all_but_the_hole() {
    let (_dir, mut document) = open(&form_fixture());
    let page = Rect::new(0.0, 0.0, 612.0, 792.0);
    let hole = Rect::new(100.0, 100.0, 300.0, 200.0);
    let mask = Annotation::new(new_id(), Kind::Stamp, page);
    let content = StampContent::Mask {
        hole,
        round: false,
        opacity: 0.5,
    };
    document.add_annotation(0, &mask, Some(&content)).unwrap();
    let inside = pixel(document.as_ref(), 200.0, 150.0);
    let outside = pixel(document.as_ref(), 400.0, 400.0);
    assert_eq!(inside, [255, 255, 255, 255]);
    assert!(
        outside[0] < 160 && outside[0] > 90,
        "half dark: {outside:?}"
    );
}

fn field<'a>(fields: &'a [Field], name: &str) -> &'a Field {
    fields
        .iter()
        .find(|field| field.name == name)
        .unwrap_or_else(|| panic!("no field {name}"))
}

#[test]
fn form_fields_fill_and_round_trip() {
    let (_dir, mut document) = open(&form_fixture());
    let fields = document.fields(0).unwrap();
    assert_eq!(fields.len(), 5, "{fields:#?}");
    let name = field(&fields, "name");
    assert!(matches!(name.kind, FieldKind::Text { .. }));
    assert_eq!(name.font_size, 12.0);
    let agree = field(&fields, "agree");
    assert_eq!(agree.kind, FieldKind::Checkbox);
    assert_eq!(agree.on_value.as_deref(), Some("Yes"));
    assert!(!agree.is_on());
    let color = field(&fields, "color");
    assert_eq!(
        color.kind,
        FieldKind::Choice {
            options: vec!["Red".into(), "Green".into(), "Blue".into()],
            combo: true
        }
    );
    let radios: Vec<&Field> = fields
        .iter()
        .filter(|f| f.kind == FieldKind::Radio)
        .collect();
    assert_eq!(radios.len(), 2);
    let large = radios
        .iter()
        .find(|radio| radio.on_value.as_deref() == Some("Large"))
        .unwrap();

    let (name_id, agree_id, color_id, large_id) = (name.id, agree.id, color.id, large.id);
    document.set_field(0, name_id, "Ada Lovelace").unwrap();
    document.set_field(0, agree_id, "Yes").unwrap();
    document.set_field(0, color_id, "Blue").unwrap();
    document.set_field(0, large_id, "Large").unwrap();

    let (dir, reopened, _) = reopen(&mut document);
    let fields = reopened.fields(0).unwrap();
    assert_eq!(field(&fields, "name").value, "Ada Lovelace");
    assert!(field(&fields, "agree").is_on());
    assert_eq!(field(&fields, "color").value, "Blue");
    let large = fields
        .iter()
        .find(|f| f.on_value.as_deref() == Some("Large"))
        .unwrap();
    assert!(large.is_on());
    let small = fields
        .iter()
        .find(|f| f.on_value.as_deref() == Some("Small"))
        .unwrap();
    assert!(!small.is_on());
    // Only the chosen radio button shows as on.
    let states = appearance_states(&dir.path().join("fixture.pdf"));
    assert!(states.contains(&"Large".to_owned()), "{states:?}");
    assert!(!states.contains(&"Small".to_owned()), "{states:?}");

    // Poppler sees the filled text.
    if let Some(text) = poppler_text(&dir.path().join("fixture.pdf")) {
        assert!(text.contains("Ada Lovelace"), "{text}");
    }
}

/// Every `/AS` entry in the file, uncompressed by MuPDF first.
fn appearance_states(path: &Path) -> Vec<String> {
    let document = mupdf::pdf::PdfDocument::open(path.as_os_str()).unwrap();
    let mut options = mupdf::pdf::PdfWriteOptions::default();
    options.set_decompress(true).set_garbage_level(1);
    let mut bytes = Vec::new();
    document.write_to_with_options(&mut bytes, options).unwrap();
    let text = String::from_utf8_lossy(&bytes);
    // "/AS /Large" or "/AS/Large": keep the name that follows.
    text.match_indices("/AS")
        .filter_map(|(index, _)| {
            let rest = text[index + 3..].trim_start().strip_prefix('/')?;
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric()).collect();
            Some(name)
        })
        .collect()
}

/// Text Poppler extracts, or `None` when Poppler is not installed.
fn poppler_text(path: &Path) -> Option<String> {
    let output = std::process::Command::new("pdftotext")
        .arg(path)
        .arg("-")
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Renders page one with Poppler at 72 dpi; `None` when not installed.
fn poppler_render(bytes: &[u8]) -> Option<image_rgb::Rgb> {
    let dir = tempfile::tempdir().ok()?;
    let path = dir.path().join("in.pdf");
    std::fs::write(&path, bytes).ok()?;
    let out = dir.path().join("out");
    let status = std::process::Command::new("pdftoppm")
        .args(["-r", "72", "-singlefile"])
        .arg(&path)
        .arg(&out)
        .status()
        .ok()?;
    status
        .success()
        .then(|| image_rgb::read_ppm(&dir.path().join("out.ppm")))
}

/// Just enough PPM reading for the Poppler checks.
mod image_rgb {
    pub struct Rgb {
        pub width: usize,
        pub pixels: Vec<u8>,
    }

    impl Rgb {
        pub fn at(&self, x: usize, y: usize) -> [u8; 3] {
            let offset = (y * self.width + x) * 3;
            self.pixels[offset..offset + 3].try_into().unwrap()
        }
    }

    pub fn read_ppm(path: &std::path::Path) -> Rgb {
        let bytes = std::fs::read(path).unwrap();
        // "P6\n<width> <height>\n255\n" then the pixels.
        let mut fields = Vec::new();
        let mut position = 0;
        while fields.len() < 4 {
            while bytes[position].is_ascii_whitespace() {
                position += 1;
            }
            let start = position;
            while !bytes[position].is_ascii_whitespace() {
                position += 1;
            }
            fields.push(String::from_utf8_lossy(&bytes[start..position]).into_owned());
        }
        Rgb {
            width: fields[1].parse().unwrap(),
            pixels: bytes[position + 1..].to_vec(),
        }
    }
}

#[test]
fn poppler_draws_prev_annotations() {
    let (_dir, mut document) = open(&form_fixture());
    let mut square = Annotation::new(
        new_id(),
        Kind::Square,
        Rect::new(300.0, 200.0, 400.0, 260.0),
    );
    square.style.color = Some(RED);
    square.style.fill = Some(BLUE);
    document.add_annotation(0, &square, None).unwrap();
    let mut stamp = Annotation::new(new_id(), Kind::Stamp, Rect::new(100.0, 300.0, 300.0, 400.0));
    stamp.subject = Some("Signature".into());
    let content = StampContent::Image {
        image: signature_bitmap(),
        round: false,
        border: None,
    };
    document.add_annotation(0, &stamp, Some(&content)).unwrap();
    let bytes = document.save().unwrap();
    let Some(page) = poppler_render(&bytes) else {
        eprintln!("pdftoppm not installed; skipping the Poppler check");
        return;
    };
    let fill = page.at(350, 230);
    assert!(fill[2] > 200 && fill[0] < 60, "square fill: {fill:?}");
    let signature = page.at(150, 350);
    assert!(
        signature[1] > 120 && signature[0] < 60,
        "signature: {signature:?}"
    );
    assert_eq!(page.at(250, 350), [255, 255, 255], "transparent half");
}

#[test]
fn saving_appends_to_the_original() {
    let original = form_fixture();
    let (_dir, mut document) = open(&original);
    assert!(!document.has_changes());
    let note = Annotation::new(new_id(), Kind::Note, Rect::new(10.0, 10.0, 30.0, 30.0));
    document.add_annotation(0, &note, None).unwrap();
    assert!(document.has_changes());
    let bytes = document.save().unwrap();
    assert!(bytes.starts_with(&original));
}
