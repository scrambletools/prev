//! Turns what the user signs with (drawn strokes, typed text or an image)
//! into a transparent PNG for the signature library, and back into a
//! bitmap for placing on a page.

use std::sync::Arc;

use prev_pdf::engine::Bitmap;

/// Handwriting, for typed signatures. The bundled instance is semibold,
/// and iced only picks a named font at its exact weight.
pub const FONT: iced::Font = iced::Font {
    family: iced::font::Family::Name("Dancing Script"),
    weight: iced::font::Weight::Semibold,
    ..iced::Font::DEFAULT
};
pub const FONT_FILE: &[u8] = include_bytes!("../../assets/fonts/DancingScript.ttf");

/// Height of stored signature images, in pixels: enough to stay sharp
/// when a signature is placed large or the page is zoomed in.
const HEIGHT: f32 = 480.0;
/// The default ink, a dark blue as from a pen.
pub const INK: [u8; 3] = [0x10, 0x18, 0x40];
/// Ink colors to sign with, as (color, name).
pub const INKS: [([u8; 3], &str); 6] = [
    (INK, "Blue-black"),
    ([0x14, 0x14, 0x14], "Black"),
    ([0x1a, 0x4f, 0xc4], "Blue"),
    ([0xc6, 0x28, 0x28], "Red"),
    ([0x2e, 0x7d, 0x32], "Green"),
    ([0x6a, 0x1b, 0x9a], "Purple"),
];
/// Pen widths the pad offers, in pad pixels, and the default.
pub const PEN_WIDTHS: std::ops::RangeInclusive<f32> = 1.0..=8.0;
pub const PEN_WIDTH: f32 = 3.0;

/// Evens out pointer positions, which come rounded to whole pixels: each
/// point moves toward its neighbours (weights 1, 2, 1), twice. The ends
/// stay where they are.
fn smoothed(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut points = points.to_vec();
    for _ in 0..2 {
        if points.len() < 3 {
            break;
        }
        let mut next = points.clone();
        for index in 1..points.len() - 1 {
            let (a, b, c) = (points[index - 1], points[index], points[index + 1]);
            next[index] = ((a.0 + 2.0 * b.0 + c.0) / 4.0, (a.1 + 2.0 * b.1 + c.1) / 4.0);
        }
        points = next;
    }
    points
}

/// Strokes as smooth paths: the points evened out, then quadratic curves
/// through the midpoints between them. `map` places each point.
fn stroke_paths(
    strokes: &[Vec<(f32, f32)>],
    map: impl Fn((f32, f32)) -> (f32, f32),
) -> Vec<tiny_skia::Path> {
    let mut paths = Vec::new();
    for points in strokes {
        let mut builder = tiny_skia::PathBuilder::new();
        let points = smoothed(points);
        let mut mapped = points.iter().map(|point| map(*point));
        let Some(first) = mapped.next() else {
            continue;
        };
        builder.move_to(first.0, first.1);
        let rest: Vec<(f32, f32)> = mapped.collect();
        if rest.is_empty() {
            // A dot.
            builder.line_to(first.0 + 0.01, first.1);
        }
        let mut previous = first;
        for (index, point) in rest.iter().enumerate() {
            if index + 1 == rest.len() {
                builder.quad_to(previous.0, previous.1, point.0, point.1);
            } else {
                let middle = ((previous.0 + point.0) / 2.0, (previous.1 + point.1) / 2.0);
                builder.quad_to(previous.0, previous.1, middle.0, middle.1);
            }
            previous = *point;
        }
        paths.extend(builder.finish());
    }
    paths
}

/// Draws strokes in `ink` onto `pixmap`, antialiased, `width` pixels wide.
pub fn draw_strokes(
    pixmap: &mut tiny_skia::Pixmap,
    strokes: &[Vec<(f32, f32)>],
    width: f32,
    ink: [u8; 3],
    map: impl Fn((f32, f32)) -> (f32, f32),
) {
    let mut paint = tiny_skia::Paint::default();
    paint.set_color_rgba8(ink[0], ink[1], ink[2], 255);
    paint.anti_alias = true;
    let stroke = tiny_skia::Stroke {
        width,
        line_cap: tiny_skia::LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..tiny_skia::Stroke::default()
    };
    for path in stroke_paths(strokes, map) {
        pixmap.stroke_path(
            &path,
            &paint,
            &stroke,
            tiny_skia::Transform::identity(),
            None,
        );
    }
}

/// Renders strokes drawn on a pad, in pad coordinates, as a PNG.
pub fn from_strokes(strokes: &[Vec<(f32, f32)>], line_width: f32, ink: [u8; 3]) -> Option<Vec<u8>> {
    let points = strokes.iter().flatten();
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for &(x, y) in points {
        x0 = x0.min(x);
        y0 = y0.min(y);
        x1 = x1.max(x);
        y1 = y1.max(y);
    }
    if x0 > x1 {
        return None;
    }
    let pad = line_width * 2.0;
    let (width, height) = ((x1 - x0) + pad * 2.0, (y1 - y0) + pad * 2.0);
    let scale = HEIGHT / height.max(1.0);
    let mut pixmap = tiny_skia::Pixmap::new(
        (width * scale).ceil().max(1.0) as u32,
        (height * scale).ceil().max(1.0) as u32,
    )?;
    draw_strokes(&mut pixmap, strokes, line_width * scale, ink, |(x, y)| {
        ((x - x0 + pad) * scale, (y - y0 + pad) * scale)
    });
    pixmap.encode_png().ok()
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Renders a typed name in the handwriting font as a PNG.
pub fn from_text(text: &str, ink: [u8; 3]) -> Option<Vec<u8>> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let size = HEIGHT * 0.6;
    let width = size * 0.7 * text.chars().count() as f32 + size;
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{HEIGHT}"><text x="{x}" y="{y}" font-family="Dancing Script" font-size="{size}" fill="#{r:02x}{g:02x}{b:02x}">{text}</text></svg>"##,
        x = size * 0.25,
        y = HEIGHT * 0.7,
        r = ink[0],
        g = ink[1],
        b = ink[2],
        text = escape(text),
    );
    let mut fonts = usvg::fontdb::Database::new();
    fonts.load_font_data(FONT_FILE.to_vec());
    let options = usvg::Options {
        fontdb: Arc::new(fonts),
        ..usvg::Options::default()
    };
    let tree = usvg::Tree::from_str(&svg, &options).ok()?;
    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())?;
    resvg::render(
        &tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    let cropped = crop_transparent(&pixmap)?;
    cropped.encode_png().ok()
}

/// Cuts away fully transparent borders, keeping a little margin.
fn crop_transparent(pixmap: &tiny_skia::Pixmap) -> Option<tiny_skia::Pixmap> {
    let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
    let data = pixmap.data();
    let (mut x0, mut y0, mut x1, mut y1) = (width, height, 0, 0);
    for y in 0..height {
        for x in 0..width {
            if data[(y * width + x) * 4 + 3] > 8 {
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
            }
        }
    }
    if x0 > x1 {
        return None;
    }
    let margin = 8;
    let (x0, y0) = (x0.saturating_sub(margin), y0.saturating_sub(margin));
    let (x1, y1) = ((x1 + margin).min(width - 1), (y1 + margin).min(height - 1));
    let rect = tiny_skia::IntRect::from_xywh(
        x0 as i32,
        y0 as i32,
        (x1 - x0 + 1) as u32,
        (y1 - y0 + 1) as u32,
    )?;
    pixmap.clone_rect(rect)
}

/// A signature photographed or scanned on paper: the paper becomes
/// transparent and the ink keeps its darkness.
pub fn from_image(image: &Bitmap) -> Option<Vec<u8>> {
    let mut pixels = Vec::with_capacity(image.pixels.len());
    for pixel in image.pixels.as_chunks::<4>().0 {
        let luminance = (0.299 * f32::from(pixel[0])
            + 0.587 * f32::from(pixel[1])
            + 0.114 * f32::from(pixel[2]))
            / 255.0;
        // Light paper fades out; ink stays.
        let ink = ((0.85 - luminance) / 0.55).clamp(0.0, 1.0);
        let alpha = (ink * f32::from(pixel[3])).round() as u8;
        pixels.extend_from_slice(&[pixel[0], pixel[1], pixel[2], alpha]);
    }
    let size = tiny_skia::IntSize::from_wh(image.width, image.height)?;
    let mut pixmap = tiny_skia::Pixmap::from_vec(premultiply(pixels), size)?;
    if let Some(cropped) = crop_transparent(&pixmap) {
        pixmap = cropped;
    }
    // Keep stored images a sensible size.
    let scale = (HEIGHT * 2.0 / pixmap.height() as f32).min(1.0);
    if scale < 1.0 {
        let mut smaller = tiny_skia::Pixmap::new(
            ((pixmap.width() as f32 * scale).round() as u32).max(1),
            ((pixmap.height() as f32 * scale).round() as u32).max(1),
        )?;
        smaller.draw_pixmap(
            0,
            0,
            pixmap.as_ref(),
            &tiny_skia::PixmapPaint {
                quality: tiny_skia::FilterQuality::Bicubic,
                ..tiny_skia::PixmapPaint::default()
            },
            tiny_skia::Transform::from_scale(scale, scale),
            None,
        );
        pixmap = smaller;
    }
    pixmap.encode_png().ok()
}

fn premultiply(mut pixels: Vec<u8>) -> Vec<u8> {
    for pixel in pixels.as_chunks_mut::<4>().0 {
        let alpha = u16::from(pixel[3]);
        for channel in &mut pixel[..3] {
            *channel = ((u16::from(*channel) * alpha + 127) / 255) as u8;
        }
    }
    pixels
}

/// Decodes a stored PNG into an RGBA bitmap, not premultiplied.
pub fn decode(png: &[u8]) -> Option<Bitmap> {
    let image = image::load_from_memory_with_format(png, image::ImageFormat::Png)
        .ok()?
        .into_rgba8();
    Some(Bitmap {
        width: image.width(),
        height: image.height(),
        pixels: image.into_raw(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ink_coverage(bitmap: &Bitmap) -> f32 {
        let inked = bitmap
            .pixels
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|pixel| pixel[3] > 128)
            .count();
        inked as f32 / (bitmap.width * bitmap.height) as f32
    }

    #[test]
    fn smoothing_evens_out_jitter_and_keeps_the_ends() {
        let zigzag: Vec<(f32, f32)> = (0..20)
            .map(|index| (index as f32 * 3.0, if index % 2 == 0 { 0.0 } else { 1.0 }))
            .collect();
        let smooth = smoothed(&zigzag);
        assert_eq!(smooth.first(), zigzag.first());
        assert_eq!(smooth.last(), zigzag.last());
        for point in &smooth[2..smooth.len() - 2] {
            assert!((point.1 - 0.5).abs() < 0.2, "{point:?}");
        }
    }

    #[test]
    fn strokes_become_a_transparent_png() {
        let strokes = vec![vec![(10.0, 10.0), (60.0, 40.0), (110.0, 10.0)]];
        let bitmap = decode(&from_strokes(&strokes, 3.0, INK).unwrap()).unwrap();
        assert_eq!(bitmap.height, HEIGHT as u32);
        let coverage = ink_coverage(&bitmap);
        assert!(coverage > 0.01 && coverage < 0.5, "coverage {coverage}");
        assert_eq!(bitmap.pixels[3], 0, "the corner is transparent");
        assert!(from_strokes(&[], 3.0, INK).is_none());
    }

    #[test]
    fn typed_names_use_the_handwriting_font() {
        let bitmap = decode(&from_text("Ada Lovelace", INK).unwrap()).unwrap();
        assert!(
            bitmap.width > bitmap.height * 2,
            "{}x{}",
            bitmap.width,
            bitmap.height
        );
        assert!(ink_coverage(&bitmap) > 0.02);
        assert!(from_text("   ", INK).is_none());
    }

    #[test]
    fn paper_turns_transparent() {
        // White paper with a dark square in the middle.
        let (width, height) = (40, 40);
        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let ink = (15..25).contains(&x) && (15..25).contains(&y);
                pixels.extend_from_slice(if ink {
                    &[20, 20, 30, 255]
                } else {
                    &[250, 248, 240, 255]
                });
            }
        }
        let image = Bitmap {
            width,
            height,
            pixels,
        };
        let bitmap = decode(&from_image(&image).unwrap()).unwrap();
        assert!(bitmap.width < 40, "cropped to the ink");
        assert_eq!(bitmap.pixels[3], 0);
        let center = ((bitmap.height / 2 * bitmap.width + bitmap.width / 2) * 4 + 3) as usize;
        assert!(bitmap.pixels[center] > 200);
    }
}
