//! The MuPDF engine through the engine-neutral interface.

mod common;

use std::path::PathBuf;

use prev_pdf::engine::{Document, Engine, Error, LinkTarget};
use prev_pdf::geometry::{PixelRect, Point, Size};
use prev_pdf::mupdf_engine::MupdfEngine;

fn write_fixture(dir: &tempfile::TempDir, name: &str, bytes: &[u8]) -> PathBuf {
    let path = dir.path().join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

fn open_navigation() -> (tempfile::TempDir, Box<dyn Document>) {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(&dir, "navigation.pdf", &common::navigation_fixture());
    let document = MupdfEngine.open(&path).unwrap();
    (dir, document)
}

fn close(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() < 0.5
}

#[test]
fn page_geometry_labels_and_title() {
    let (_dir, document) = open_navigation();
    assert!(!document.needs_password());
    assert_eq!(document.page_count().unwrap(), 3);
    assert_eq!(document.page_size(0).unwrap(), Size::new(612.0, 792.0));
    assert_eq!(document.page_size(1).unwrap(), Size::new(842.0, 595.0));
    assert_eq!(
        document.page_size(2).unwrap(),
        Size::new(792.0, 612.0),
        "rotation applied"
    );
    let labels: Vec<_> = (0..3).map(|index| document.page_label(index)).collect();
    assert_eq!(
        labels,
        [Some("i".into()), Some("ii".into()), Some("1".into())]
    );
    assert_eq!(document.title().as_deref(), Some("Engine Fixture"));
    assert_eq!(document.page_size(3), Err(Error::PageOutOfRange(3)));
}

#[test]
fn outline_with_targets() {
    let (_dir, document) = open_navigation();
    let outline = document.outline().unwrap();
    let titles: Vec<_> = outline.iter().map(|item| item.title.as_str()).collect();
    assert_eq!(titles, ["Chapter One", "Chapter Two"]);
    assert_eq!(outline[0].children[0].title, "Section 1.1");
    match &outline[0].target {
        Some(LinkTarget::Page {
            index: 0,
            point: Some(point),
        }) => assert!(close(point.y, 0.0), "{point:?}"),
        other => panic!("unexpected target {other:?}"),
    }
    assert!(matches!(
        outline[1].target,
        Some(LinkTarget::Page {
            index: 1,
            point: None
        })
    ));
    assert!(matches!(
        outline[0].children[0].target,
        Some(LinkTarget::Page { index: 2, .. })
    ));
}

#[test]
fn links_in_top_left_page_space() {
    let (_dir, document) = open_navigation();
    let links = document.links(0).unwrap();
    assert_eq!(links.len(), 2);
    let internal = links
        .iter()
        .find(|link| matches!(link.target, LinkTarget::Page { .. }))
        .unwrap();
    assert!(
        close(internal.bounds.y0, 72.0) && close(internal.bounds.y1, 102.0),
        "{:?}",
        internal.bounds
    );
    match internal.target {
        // XYZ top 400 on a 595 point tall page is 195 points from its top.
        LinkTarget::Page {
            index: 1,
            point: Some(Point { y, .. }),
        } => assert!(close(y, 195.0), "y = {y}"),
        ref other => panic!("unexpected target {other:?}"),
    }
    assert!(
        links
            .iter()
            .any(|link| link.target == LinkTarget::Uri("https://example.org/".into()))
    );
    assert!(document.links(1).unwrap().is_empty());
}

#[test]
fn tiles_match_the_full_page_render() {
    let (_dir, document) = open_navigation();
    let page = document.display(0).unwrap();
    let scale = 2.0;
    let full = page
        .render(
            scale,
            PixelRect {
                x: 0,
                y: 0,
                width: 1224,
                height: 1584,
            },
        )
        .unwrap();
    assert_eq!((full.width, full.height), (1224, 1584));
    assert_eq!(&full.pixels[..4], &[255, 255, 255, 255], "white background");
    let dark = full
        .pixels
        .chunks_exact(4)
        .filter(|pixel| pixel[0] < 100)
        .count();
    assert!(dark > 1000, "text is drawn ({dark} dark pixels)");

    let tile_area = PixelRect {
        x: 128,
        y: 160,
        width: 256,
        height: 128,
    };
    let tile = page.render(scale, tile_area).unwrap();
    for row in 0..tile.height as usize {
        let tile_row = &tile.pixels[row * 256 * 4..(row + 1) * 256 * 4];
        let full_start = ((tile_area.y as usize + row) * 1224 + tile_area.x as usize) * 4;
        assert_eq!(
            tile_row,
            &full.pixels[full_start..full_start + 256 * 4],
            "row {row}"
        );
    }
}

#[test]
fn text_layout_selection_and_search() {
    let (_dir, document) = open_navigation();
    let page = document.display(0).unwrap();
    let text = page.text().unwrap();
    assert_eq!(text.lines.len(), 2);
    let all = text.select_all().unwrap();
    assert_eq!(text.text(all), "Hello world\nSecond line here");

    let first_line = text.lines[0].bounds;
    let second_line = text.lines[1].bounds;
    let selection = text
        .select(
            Point::new(first_line.x1 - 10.0, first_line.center().y),
            Point::new(second_line.x0 + 1.0, second_line.center().y),
        )
        .unwrap();
    assert_eq!(text.text(selection), "d\n");

    let hits = page.search("WORLD").unwrap();
    assert_eq!(hits.len(), 1);
    let hit = hits[0].bounds();
    assert!(
        first_line.contains(hit.center()),
        "hit {hit:?} inside {first_line:?}"
    );
    assert!(page.search("missing").unwrap().is_empty());
    assert!(page.search("  ").unwrap().is_empty());
}

#[test]
fn rotated_page_text_is_in_rotated_space() {
    let (_dir, document) = open_navigation();
    let page = document.display(2).unwrap();
    assert_eq!(page.size(), Size::new(792.0, 612.0));
    let text = page.text().unwrap();
    let bounds = text.lines[0].bounds;
    assert!(
        bounds.x0 >= 0.0 && bounds.x1 <= 792.0 && bounds.y0 >= 0.0 && bounds.y1 <= 612.0,
        "{bounds:?}"
    );
}

#[test]
fn encrypted_documents_need_a_password() {
    use mupdf::pdf::{PdfDocument, PdfWriteOptions, document::Encryption};
    let source = PdfDocument::from_bytes(&common::navigation_fixture()).unwrap();
    let mut options = PdfWriteOptions::default();
    options
        .set_encryption(Encryption::Aes256)
        .set_user_password("secret")
        .set_owner_password("owner");
    let mut bytes = Vec::new();
    source.write_to_with_options(&mut bytes, options).unwrap();

    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(&dir, "locked.pdf", &bytes);
    let mut document = MupdfEngine.open(&path).unwrap();
    assert!(document.needs_password());
    assert!(!document.authenticate("wrong"));
    assert!(document.authenticate("secret"));
    assert_eq!(document.page_count().unwrap(), 3);
}

#[test]
fn unreadable_files_fail_to_open() {
    let dir = tempfile::tempdir().unwrap();
    assert!(matches!(
        MupdfEngine.open(&dir.path().join("missing.pdf")),
        Err(Error::Open(_))
    ));
    let garbage = write_fixture(&dir, "garbage.pdf", b"this is not a pdf");
    assert!(MupdfEngine.open(&garbage).is_err());
}

#[test]
fn fast_page_sizes_match_loaded_pages() {
    let (_dir, document) = open_navigation();
    let slow: Vec<Size> = (0..3)
        .map(|index| document.page_size(index).unwrap())
        .collect();
    assert_eq!(document.page_sizes().unwrap(), slow);

    // With the render corpus present, check every page of every file too.
    let Some(corpus) = std::env::var_os("PREV_CORPUS").map(PathBuf::from) else {
        return;
    };
    for entry in std::fs::read_dir(corpus).unwrap() {
        let path = entry.unwrap().path();
        let document = MupdfEngine.open(&path).unwrap();
        let fast = document.page_sizes().unwrap();
        for (index, size) in fast.iter().enumerate() {
            let loaded = document.page_size(index).unwrap();
            assert!(
                (size.width - loaded.width).abs() < 0.5
                    && (size.height - loaded.height).abs() < 0.5,
                "{} page {index}: {size:?} vs {loaded:?}",
                path.display()
            );
        }
    }
}
