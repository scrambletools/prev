//! Encoding edited images, and carrying the original's metadata over.

use std::io::Cursor;

use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ExtendedColorType, ImageEncoder, RgbaImage};
use img_parts::{DynImage, ImageEXIF, ImageICC};

use crate::ImageFormat;
use crate::decode::Frame;

pub const DEFAULT_JPEG_QUALITY: u8 = 92;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveFormat {
    Png,
    Jpeg {
        quality: u8,
    },
    /// Lossless; the `image` crate has no lossy WebP encoder.
    WebP,
    Tiff,
    Bmp,
    Tga,
    Qoi,
    Pnm,
    OpenExr,
}

impl SaveFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg { .. } => "jpg",
            Self::WebP => "webp",
            Self::Tiff => "tiff",
            Self::Bmp => "bmp",
            Self::Tga => "tga",
            Self::Qoi => "qoi",
            Self::Pnm => "ppm",
            Self::OpenExr => "exr",
        }
    }

    /// The format an edited file is written back in, or `None` when it can
    /// only be exported: formats prev cannot encode, would lose range in
    /// (HDR, RAW), or animations.
    pub fn for_saving(format: ImageFormat, animated: bool) -> Option<Self> {
        if animated {
            return None;
        }
        Some(match format {
            ImageFormat::Png => Self::Png,
            ImageFormat::Jpeg => Self::Jpeg {
                quality: DEFAULT_JPEG_QUALITY,
            },
            ImageFormat::WebP => Self::WebP,
            ImageFormat::Tiff => Self::Tiff,
            ImageFormat::Bmp => Self::Bmp,
            ImageFormat::Tga => Self::Tga,
            ImageFormat::Qoi => Self::Qoi,
            ImageFormat::Pnm => Self::Pnm,
            ImageFormat::OpenExr => Self::OpenExr,
            ImageFormat::Gif
            | ImageFormat::Avif
            | ImageFormat::Heif
            | ImageFormat::Ico
            | ImageFormat::Hdr
            | ImageFormat::Jpeg2000
            | ImageFormat::Raw => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodeError(pub String);

impl std::fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "cannot encode the image: {}", self.0)
    }
}

impl std::error::Error for EncodeError {}

impl From<image::ImageError> for EncodeError {
    fn from(error: image::ImageError) -> Self {
        Self(error.to_string())
    }
}

/// Composites onto white, for formats without transparency.
fn flatten(image: &RgbaImage) -> image::RgbImage {
    image::RgbImage::from_fn(image.width(), image.height(), |x, y| {
        let [red, green, blue, alpha] = image.get_pixel(x, y).0;
        let over_white = |channel: u8| {
            ((u16::from(channel) * u16::from(alpha) + 255 * (255 - u16::from(alpha)) + 127) / 255)
                as u8
        };
        image::Rgb([over_white(red), over_white(green), over_white(blue)])
    })
}

fn srgb_to_linear(value: u8) -> f32 {
    let value = f32::from(value) / 255.0;
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

pub fn encode(frame: &Frame, format: SaveFormat) -> Result<Vec<u8>, EncodeError> {
    let image = RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone())
        .ok_or_else(|| EncodeError("frame pixels do not match its size".into()))?;
    let mut out = Vec::new();
    match format {
        SaveFormat::Jpeg { quality } => {
            let rgb = flatten(&image);
            JpegEncoder::new_with_quality(&mut out, quality.clamp(1, 100)).write_image(
                rgb.as_raw(),
                rgb.width(),
                rgb.height(),
                ExtendedColorType::Rgb8,
            )?;
        }
        SaveFormat::WebP => WebPEncoder::new_lossless(&mut out).write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            ExtendedColorType::Rgba8,
        )?,
        SaveFormat::Pnm => DynamicImage::ImageRgb8(flatten(&image))
            .write_to(&mut Cursor::new(&mut out), image::ImageFormat::Pnm)?,
        SaveFormat::OpenExr => {
            // EXR holds linear light.
            let linear = image::Rgba32FImage::from_fn(image.width(), image.height(), |x, y| {
                let [red, green, blue, alpha] = image.get_pixel(x, y).0;
                image::Rgba([
                    srgb_to_linear(red),
                    srgb_to_linear(green),
                    srgb_to_linear(blue),
                    f32::from(alpha) / 255.0,
                ])
            });
            DynamicImage::ImageRgba32F(linear)
                .write_to(&mut Cursor::new(&mut out), image::ImageFormat::OpenExr)?;
        }
        SaveFormat::Png
        | SaveFormat::Tiff
        | SaveFormat::Bmp
        | SaveFormat::Tga
        | SaveFormat::Qoi => {
            let target = match format {
                SaveFormat::Png => image::ImageFormat::Png,
                SaveFormat::Tiff => image::ImageFormat::Tiff,
                SaveFormat::Bmp => image::ImageFormat::Bmp,
                SaveFormat::Tga => image::ImageFormat::Tga,
                _ => image::ImageFormat::Qoi,
            };
            DynamicImage::ImageRgba8(image).write_to(&mut Cursor::new(&mut out), target)?;
        }
    }
    Ok(out)
}

/// Copies EXIF, the ICC color profile and XMP from `original` into
/// `encoded`, and resets the EXIF orientation because edits apply to the
/// upright image. JPEG, PNG and WebP only; other formats keep none.
pub fn carry_metadata(original: &[u8], encoded: Vec<u8>) -> Vec<u8> {
    let Some(source) = DynImage::from_bytes(Bytes::copy_from_slice(original))
        .ok()
        .flatten()
    else {
        return encoded;
    };
    let Ok(Some(mut target)) = DynImage::from_bytes(Bytes::from(encoded.clone())) else {
        return encoded;
    };
    target.set_icc_profile(source.icc_profile());
    target.set_exif(source.exif().map(|exif| {
        let mut exif = exif.to_vec();
        crate::exif_edit::reset_orientation(&mut exif);
        Bytes::from(exif)
    }));
    let mut out = target.encoder().bytes().to_vec();
    if let Some(packet) = crate::xmp::extract(original) {
        out = crate::xmp::embed(out, &packet);
    }
    crate::xmp::normalize_jpeg(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::decode;
    use std::time::Duration;

    fn frame() -> Frame {
        let image = RgbaImage::from_fn(8, 6, |x, y| {
            image::Rgba([x as u8 * 30, y as u8 * 40, 90, 255])
        });
        Frame {
            width: 8,
            height: 6,
            pixels: image.into_raw(),
            delay: Duration::ZERO,
        }
    }

    #[test]
    fn lossless_formats_round_trip() {
        for (format, source) in [
            (SaveFormat::Png, ImageFormat::Png),
            (SaveFormat::WebP, ImageFormat::WebP),
            (SaveFormat::Tiff, ImageFormat::Tiff),
            (SaveFormat::Bmp, ImageFormat::Bmp),
            (SaveFormat::Tga, ImageFormat::Tga),
            (SaveFormat::Qoi, ImageFormat::Qoi),
            (SaveFormat::Pnm, ImageFormat::Pnm),
            (SaveFormat::OpenExr, ImageFormat::OpenExr),
        ] {
            let bytes =
                encode(&frame(), format).unwrap_or_else(|error| panic!("{format:?}: {error}"));
            let decoded =
                decode(&bytes, source).unwrap_or_else(|error| panic!("{format:?}: {error}"));
            let worst = decoded.frames[0]
                .pixels
                .iter()
                .zip(&frame().pixels)
                .map(|(a, b)| a.abs_diff(*b))
                .max();
            assert!(worst <= Some(1), "{format:?} differs by {worst:?}");
        }
    }

    #[test]
    fn jpeg_is_close_and_flattens_alpha() {
        let mut transparent = frame();
        transparent
            .pixels
            .iter_mut()
            .skip(3)
            .step_by(4)
            .for_each(|alpha| *alpha = 0);
        let bytes = encode(&transparent, SaveFormat::Jpeg { quality: 95 }).unwrap();
        let decoded = decode(&bytes, ImageFormat::Jpeg).unwrap();
        assert!(
            decoded.frames[0].pixels[..3]
                .iter()
                .all(|channel| *channel > 240),
            "transparent becomes white"
        );
    }

    #[test]
    fn saving_formats() {
        assert_eq!(
            SaveFormat::for_saving(ImageFormat::Jpeg, false),
            Some(SaveFormat::Jpeg {
                quality: DEFAULT_JPEG_QUALITY
            })
        );
        assert_eq!(
            SaveFormat::for_saving(ImageFormat::Png, true),
            None,
            "animations are export only"
        );
        assert_eq!(SaveFormat::for_saving(ImageFormat::Heif, false), None);
        assert_eq!(SaveFormat::for_saving(ImageFormat::Raw, false), None);
    }

    /// A JPEG with EXIF (camera make, orientation 6), an ICC profile and XMP.
    fn original_jpeg() -> Vec<u8> {
        let bytes = encode(&frame(), SaveFormat::Jpeg { quality: 90 }).unwrap();
        let mut jpeg = DynImage::from_bytes(Bytes::from(bytes)).unwrap().unwrap();
        jpeg.set_exif(Some(Bytes::from(crate::exif_edit::fixture::exif(6))));
        jpeg.set_icc_profile(Some(Bytes::from_static(b"fake icc profile")));
        crate::xmp::embed(jpeg.encoder().bytes().to_vec(), b"<x:xmpmeta/>")
    }

    #[test]
    fn metadata_is_carried_and_orientation_reset() {
        let original = original_jpeg();
        let edited = encode(&frame(), SaveFormat::Jpeg { quality: 90 }).unwrap();
        let saved = carry_metadata(&original, edited);

        let exif = exif::Reader::new()
            .read_from_container(&mut Cursor::new(&saved))
            .unwrap();
        let make = exif
            .get_field(exif::Tag::Make, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .to_string();
        assert_eq!(make, "\"prevcam\"");
        let orientation = exif
            .get_field(exif::Tag::Orientation, exif::In::PRIMARY)
            .unwrap();
        assert_eq!(orientation.value.get_uint(0), Some(1));

        let parsed = DynImage::from_bytes(Bytes::from(saved.clone()))
            .unwrap()
            .unwrap();
        assert_eq!(
            parsed.icc_profile().as_deref(),
            Some(&b"fake icc profile"[..])
        );
        let DynImage::Jpeg(jpeg) = &parsed else {
            panic!("not a JPEG")
        };
        let order: Vec<u8> = jpeg
            .segments()
            .iter()
            .map(|segment| segment.marker())
            .collect();
        let first_non_app = order
            .iter()
            .position(|marker| !(0xE0..=0xEF).contains(marker))
            .unwrap();
        assert!(
            order[first_non_app..]
                .iter()
                .all(|marker| !(0xE0..=0xEF).contains(marker)),
            "{order:02X?}"
        );
        assert_eq!(order[0], 0xE0, "JFIF comes first: {order:02X?}");
        assert_eq!(
            crate::xmp::extract(&saved).as_deref(),
            Some(&b"<x:xmpmeta/>"[..])
        );
        assert!(decode(&saved, ImageFormat::Jpeg).is_ok());
    }

    #[test]
    fn metadata_free_originals_are_fine() {
        let plain = encode(&frame(), SaveFormat::Png).unwrap();
        let saved = carry_metadata(&plain, plain.clone());
        assert!(decode(&saved, ImageFormat::Png).is_ok());
        assert_eq!(carry_metadata(b"garbage", plain.clone()), plain);
    }
}
