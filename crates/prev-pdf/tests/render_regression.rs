//! Render regression: the first page of each corpus file, rendered small and
//! in grayscale, must match its stored reference within a tolerance that
//! absorbs anti-aliasing and hinting noise but not missing content.
//!
//! The corpus is fetched with `scripts/fetch-render-corpus.sh <dir>`. Set
//! `PREV_CORPUS=<dir>` to run; `PREV_CORPUS_REQUIRED=1` fails when it is
//! missing (CI) and `PREV_CORPUS_UPDATE=1` rewrites the references.

use std::path::{Path, PathBuf};

use prev_pdf::engine::{Engine, page_pixels};
use prev_pdf::geometry::PixelRect;
use prev_pdf::mupdf_engine::MupdfEngine;

const REFERENCE_WIDTH: f32 = 128.0;
/// A pixel differs when its gray level is off by more than this.
const PIXEL_TOLERANCE: u8 = 40;
/// A page fails when more than this share of pixels differ.
const MAX_DIFFERING: f64 = 0.015;

struct Gray {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

fn references_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/render-corpus/refs")
}

fn corpus_files() -> Vec<String> {
    let list = include_str!("render-corpus/files.sha256");
    list.lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .map(str::to_owned)
        .collect()
}

fn render_first_page(path: &Path) -> Gray {
    let document = MupdfEngine
        .open(path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let display = document.display(0).unwrap();
    let size = display.size();
    let scale = REFERENCE_WIDTH / size.width;
    let (width, height) = page_pixels(size, scale);
    let bitmap = display
        .render(
            scale,
            PixelRect {
                x: 0,
                y: 0,
                width,
                height,
            },
        )
        .unwrap();
    let pixels = bitmap
        .pixels
        .chunks_exact(4)
        .map(|rgba| {
            ((u32::from(rgba[0]) * 299 + u32::from(rgba[1]) * 587 + u32::from(rgba[2]) * 114)
                / 1000) as u8
        })
        .collect();
    Gray {
        width: width as usize,
        height: height as usize,
        pixels,
    }
}

fn write_pgm(path: &Path, image: &Gray) {
    let mut bytes = format!("P5\n{} {}\n255\n", image.width, image.height).into_bytes();
    bytes.extend_from_slice(&image.pixels);
    std::fs::write(path, bytes).unwrap();
}

fn read_pgm(path: &Path) -> Option<Gray> {
    let bytes = std::fs::read(path).ok()?;
    let mut fields = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while fields.len() < 4 && index < bytes.len() {
        if bytes[index].is_ascii_whitespace() {
            if index > start {
                fields.push(std::str::from_utf8(&bytes[start..index]).ok()?.to_owned());
            }
            start = index + 1;
        }
        index += 1;
    }
    if fields.first().map(String::as_str) != Some("P5") || fields.len() < 4 {
        return None;
    }
    let width: usize = fields[1].parse().ok()?;
    let height: usize = fields[2].parse().ok()?;
    let pixels = bytes.get(start..start + width * height)?.to_vec();
    Some(Gray {
        width,
        height,
        pixels,
    })
}

/// Share of pixels in `from` with no counterpart in `to` within one pixel.
fn unmatched_share(from: &Gray, to: &Gray) -> f64 {
    let at = |image: &Gray, x: i64, y: i64| {
        let x = x.clamp(0, image.width as i64 - 1) as usize;
        let y = y.clamp(0, image.height as i64 - 1) as usize;
        image.pixels[y * image.width + x]
    };
    let mut unmatched = 0usize;
    for y in 0..from.height as i64 {
        for x in 0..from.width as i64 {
            let value = at(from, x, y);
            let close = (-1..=1).any(|dy| {
                (-1..=1).any(|dx| value.abs_diff(at(to, x + dx, y + dy)) <= PIXEL_TOLERANCE)
            });
            if !close {
                unmatched += 1;
            }
        }
    }
    unmatched as f64 / (from.width * from.height) as f64
}

/// Share of differing pixels, ignoring one-pixel shifts. Checked both ways,
/// so content missing from either image counts.
fn differing_share(actual: &Gray, expected: &Gray) -> f64 {
    unmatched_share(actual, expected).max(unmatched_share(expected, actual))
}

#[test]
fn first_pages_match_references() {
    let Some(corpus) = std::env::var_os("PREV_CORPUS").map(PathBuf::from) else {
        assert!(
            std::env::var_os("PREV_CORPUS_REQUIRED").is_none(),
            "PREV_CORPUS is not set"
        );
        eprintln!("skipping render regression: PREV_CORPUS is not set");
        return;
    };
    let update = std::env::var_os("PREV_CORPUS_UPDATE").is_some();
    let mut failures = Vec::new();
    for name in corpus_files() {
        let actual = render_first_page(&corpus.join(&name));
        let reference = references_dir().join(format!("{}.pgm", name.trim_end_matches(".pdf")));
        if update {
            write_pgm(&reference, &actual);
            continue;
        }
        match read_pgm(&reference) {
            None => failures.push(format!("{name}: no reference image")),
            Some(expected)
                if (expected.width, expected.height) != (actual.width, actual.height) =>
            {
                failures.push(format!(
                    "{name}: size {}x{} instead of {}x{}",
                    actual.width, actual.height, expected.width, expected.height
                ))
            }
            Some(expected) => {
                let share = differing_share(&actual, &expected);
                if share > MAX_DIFFERING {
                    failures.push(format!("{name}: {:.1}% of pixels differ", share * 100.0));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "render regressions:\n{}",
        failures.join("\n")
    );
}

#[test]
fn comparison_catches_blank_pages() {
    let page = Gray {
        width: 20,
        height: 20,
        pixels: (0..400)
            .map(|index| if index % 3 == 0 { 0 } else { 255 })
            .collect(),
    };
    let blank = Gray {
        width: 20,
        height: 20,
        pixels: vec![255; 400],
    };
    assert_eq!(differing_share(&page, &page), 0.0);
    assert!(differing_share(&blank, &page) > MAX_DIFFERING);
    assert!(differing_share(&page, &blank) > MAX_DIFFERING);
    let shifted = Gray {
        width: 20,
        height: 20,
        pixels: (0..400)
            .map(|index| if index % 3 == 1 { 0 } else { 255 })
            .collect(),
    };
    assert!(
        differing_share(&shifted, &page) <= MAX_DIFFERING,
        "one-pixel shifts are tolerated"
    );
}
