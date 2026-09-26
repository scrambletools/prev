//! Formats the `image` crate lacks, decoded by MuPDF: JPEG 2000.

use std::time::Duration;

use crate::decode::{DecodeError, Decoded, Frame, Result};

pub fn decode(bytes: &[u8]) -> Result<Decoded> {
    let invalid = |error: mupdf::Error| DecodeError::Invalid(error.to_string());
    let pixmap = mupdf::Image::from_bytes(bytes)
        .map_err(invalid)?
        .to_pixmap()
        .map_err(invalid)?;
    let (width, height) = (pixmap.width(), pixmap.height());
    let components = usize::from(pixmap.n());
    let alpha = pixmap.alpha();
    let colors = components - usize::from(alpha);
    let stride = pixmap.stride() as usize;
    let samples = pixmap.samples();

    let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
    for row in 0..height as usize {
        let row = &samples[row * stride..row * stride + width as usize * components];
        for sample in row.chunks(components) {
            let opacity = if alpha { sample[colors] } else { 255 };
            let [red, green, blue] = match colors {
                1 => [sample[0]; 3],
                3 => [sample[0], sample[1], sample[2]],
                // CMYK, converted naively.
                4 => {
                    let black = 255 - u16::from(sample[3]);
                    [0, 1, 2]
                        .map(|channel| ((255 - u16::from(sample[channel])) * black / 255) as u8)
                }
                other => {
                    return Err(DecodeError::Invalid(format!(
                        "unexpected {other} color components"
                    )));
                }
            };
            pixels.extend_from_slice(&[
                unpremultiply(red, opacity, alpha),
                unpremultiply(green, opacity, alpha),
                unpremultiply(blue, opacity, alpha),
                opacity,
            ]);
        }
    }
    Ok(Decoded {
        frames: vec![Frame {
            width,
            height,
            pixels,
            delay: Duration::ZERO,
        }],
    })
}

/// MuPDF pixmaps with alpha are premultiplied.
fn unpremultiply(value: u8, opacity: u8, alpha: bool) -> u8 {
    if !alpha || opacity == 255 {
        value
    } else if opacity == 0 {
        0
    } else {
        (u16::from(value) * 255 / u16::from(opacity)).min(255) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpremultiplies() {
        assert_eq!(unpremultiply(64, 128, true), 127);
        assert_eq!(unpremultiply(64, 128, false), 64);
        assert_eq!(unpremultiply(10, 0, true), 0);
    }

    #[test]
    fn decodes_through_mupdf() {
        // MuPDF decodes more than JPEG 2000; a PNG exercises the same path.
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            3,
            2,
            image::Rgba([10, 200, 30, 255]),
        ))
        .write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();
        let decoded = decode(&bytes).unwrap();
        assert_eq!((decoded.width(), decoded.height()), (3, 2));
        assert_eq!(&decoded.frames[0].pixels[..4], &[10, 200, 30, 255]);
        assert!(decode(b"junk").is_err());
    }
}
