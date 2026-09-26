//! Decodes image files into 8-bit RGBA frames, applying EXIF orientation.

use std::io::{BufReader, Cursor};
use std::path::Path;
use std::time::Duration;

use image::codecs::gif::GifDecoder;
use image::codecs::png::PngDecoder;
use image::codecs::webp::WebPDecoder;
use image::{AnimationDecoder, DynamicImage, ImageDecoder, ImageReader};

use crate::ImageFormat;

/// Frames shorter than this are shown for this long, as browsers do.
const MIN_FRAME_DELAY: Duration = Duration::from_millis(20);
const DEFAULT_FRAME_DELAY: Duration = Duration::from_millis(100);

/// An image as straight (not premultiplied) 8-bit RGBA.
#[derive(Clone, PartialEq, Eq)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    /// How long an animation frame is shown; zero for still images.
    pub delay: Duration,
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Frame({}x{}, {:?})",
            self.width, self.height, self.delay
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    /// One frame for still images, several for animations.
    pub frames: Vec<Frame>,
}

impl Decoded {
    pub fn width(&self) -> u32 {
        self.frames[0].width
    }

    pub fn height(&self) -> u32 {
        self.frames[0].height
    }

    pub fn is_animated(&self) -> bool {
        self.frames.len() > 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    Io(String),
    /// The file is damaged or not what its format claims.
    Invalid(String),
    /// Decoding needs a system library that is not installed.
    MissingLibrary(String),
    Unsupported(ImageFormat),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(message) => write!(formatter, "cannot read the file: {message}"),
            Self::Invalid(message) => {
                write!(formatter, "the image is damaged or invalid: {message}")
            }
            Self::MissingLibrary(library) => write!(
                formatter,
                "opening this format needs {library}, which is not installed"
            ),
            Self::Unsupported(format) => {
                write!(formatter, "{format:?} images are not supported yet")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

impl From<image::ImageError> for DecodeError {
    fn from(error: image::ImageError) -> Self {
        match error {
            image::ImageError::IoError(error) => Self::Io(error.to_string()),
            other => Self::Invalid(other.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, DecodeError>;

pub fn decode_file(path: &Path, format: ImageFormat) -> Result<Decoded> {
    let bytes = std::fs::read(path).map_err(|error| DecodeError::Io(error.to_string()))?;
    decode(&bytes, format)
}

pub fn decode(bytes: &[u8], format: ImageFormat) -> Result<Decoded> {
    use ImageFormat::*;
    match format {
        Gif => animation(GifDecoder::new(Cursor::new(bytes))?),
        Png if is_animated_png(bytes)? => animation(PngDecoder::new(Cursor::new(bytes))?.apng()?),
        WebP if WebPDecoder::new(Cursor::new(bytes))?.has_animation() => {
            animation(WebPDecoder::new(Cursor::new(bytes))?)
        }
        Png | Jpeg | WebP | Bmp | Ico | Tiff | Tga | Pnm | Qoi | Hdr | OpenExr => {
            still(bytes, format)
        }
        Jpeg2000 => crate::mupdf_image::decode(bytes),
        Avif | Heif => crate::heif::decode(bytes),
        #[cfg(feature = "raw")]
        Raw => crate::raw::decode(bytes),
        #[cfg(not(feature = "raw"))]
        Raw => Err(DecodeError::Unsupported(format)),
    }
}

fn is_animated_png(bytes: &[u8]) -> Result<bool> {
    Ok(PngDecoder::new(Cursor::new(bytes))?.is_apng()?)
}

fn image_crate_format(format: ImageFormat) -> Option<image::ImageFormat> {
    use ImageFormat::*;
    Some(match format {
        Png => image::ImageFormat::Png,
        Jpeg => image::ImageFormat::Jpeg,
        Gif => image::ImageFormat::Gif,
        WebP => image::ImageFormat::WebP,
        Bmp => image::ImageFormat::Bmp,
        Ico => image::ImageFormat::Ico,
        Tiff => image::ImageFormat::Tiff,
        Tga => image::ImageFormat::Tga,
        Pnm => image::ImageFormat::Pnm,
        Qoi => image::ImageFormat::Qoi,
        Hdr => image::ImageFormat::Hdr,
        OpenExr => image::ImageFormat::OpenExr,
        Avif | Heif | Jpeg2000 | Raw => return None,
    })
}

fn still(bytes: &[u8], format: ImageFormat) -> Result<Decoded> {
    let mut reader = ImageReader::new(BufReader::new(Cursor::new(bytes)));
    reader.set_format(image_crate_format(format).ok_or(DecodeError::Unsupported(format))?);
    let mut decoder = reader.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut image = DynamicImage::from_decoder(decoder)?;
    image.apply_orientation(orientation);
    Ok(Decoded {
        frames: vec![to_frame(image)],
    })
}

pub(crate) fn to_frame(image: DynamicImage) -> Frame {
    let (width, height) = (image.width(), image.height());
    let pixels = match image {
        // Floating point images hold linear light.
        DynamicImage::ImageRgb32F(_) | DynamicImage::ImageRgba32F(_) => {
            tone_map(image.into_rgba32f().into_raw())
        }
        other => other.into_rgba8().into_raw(),
    };
    Frame {
        width,
        height,
        pixels,
        delay: Duration::ZERO,
    }
}

/// Linear floating point RGBA to sRGB 8-bit at exposure 0, clipping
/// highlights above 1.0 as most viewers do.
fn tone_map(linear: Vec<f32>) -> Vec<u8> {
    let encode = |value: f32| {
        let value = value.clamp(0.0, 1.0);
        let srgb = if value <= 0.003_130_8 {
            value * 12.92
        } else {
            1.055 * value.powf(1.0 / 2.4) - 0.055
        };
        (srgb * 255.0 + 0.5) as u8
    };
    linear
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|[red, green, blue, alpha]| {
            [
                encode(*red),
                encode(*green),
                encode(*blue),
                (alpha.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
            ]
        })
        .collect()
}

fn animation<'a>(decoder: impl AnimationDecoder<'a>) -> Result<Decoded> {
    let frames = decoder
        .into_frames()
        .map(|frame| {
            let frame = frame?;
            let (numerator, denominator) = frame.delay().numer_denom_ms();
            let delay = if denominator == 0 {
                DEFAULT_FRAME_DELAY
            } else {
                Duration::from_micros(u64::from(numerator) * 1000 / u64::from(denominator))
                    .max(MIN_FRAME_DELAY)
            };
            let buffer = frame.into_buffer();
            Ok(Frame {
                width: buffer.width(),
                height: buffer.height(),
                pixels: buffer.into_raw(),
                delay,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    if frames.is_empty() {
        return Err(DecodeError::Invalid("animation has no frames".into()));
    }
    Ok(Decoded { frames })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageEncoder, RgbaImage};

    fn encode(image: &RgbaImage, format: image::ImageFormat) -> Vec<u8> {
        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image.clone())
            .write_to(&mut Cursor::new(&mut bytes), format)
            .unwrap();
        bytes
    }

    fn gradient(width: u32, height: u32) -> RgbaImage {
        RgbaImage::from_fn(width, height, |x, y| {
            image::Rgba([(x * 40) as u8, (y * 40) as u8, 128, 255])
        })
    }

    #[test]
    fn decodes_lossless_formats_exactly() {
        let source = gradient(5, 3);
        for format in [
            ImageFormat::Png,
            ImageFormat::Bmp,
            ImageFormat::Tiff,
            ImageFormat::Qoi,
            ImageFormat::Tga,
        ] {
            let bytes = encode(&source, image_crate_format(format).unwrap());
            let decoded =
                decode(&bytes, format).unwrap_or_else(|error| panic!("{format:?}: {error}"));
            assert_eq!((decoded.width(), decoded.height()), (5, 3), "{format:?}");
            assert_eq!(
                decoded.frames[0].pixels,
                source.as_raw().clone(),
                "{format:?}"
            );
            assert!(!decoded.is_animated());
        }
    }

    #[test]
    fn jpeg_is_close() {
        let source = gradient(16, 16);
        let rgb = DynamicImage::ImageRgba8(source.clone()).into_rgb8();
        let mut bytes = Vec::new();
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, 95)
            .write_image(rgb.as_raw(), 16, 16, image::ExtendedColorType::Rgb8)
            .unwrap();
        let decoded = decode(&bytes, ImageFormat::Jpeg).unwrap();
        let worst = decoded.frames[0]
            .pixels
            .iter()
            .zip(source.as_raw())
            .map(|(a, b)| a.abs_diff(*b))
            .max()
            .unwrap();
        assert!(worst < 24, "worst channel error {worst}");
    }

    #[test]
    fn gif_animation_keeps_frames_and_delays() {
        let mut bytes = Vec::new();
        {
            let mut encoder = image::codecs::gif::GifEncoder::new(&mut bytes);
            let frames = [(255u8, 30u32), (0, 5)].map(|(red, centiseconds)| {
                image::Frame::from_parts(
                    RgbaImage::from_pixel(4, 4, image::Rgba([red, 0, 0, 255])),
                    0,
                    0,
                    image::Delay::from_numer_denom_ms(centiseconds * 10, 1),
                )
            });
            encoder.encode_frames(frames).unwrap();
        }
        let decoded = decode(&bytes, ImageFormat::Gif).unwrap();
        assert!(decoded.is_animated());
        assert_eq!(decoded.frames[0].delay, Duration::from_millis(300));
        assert_eq!(
            decoded.frames[1].delay,
            MIN_FRAME_DELAY.max(Duration::from_millis(50))
        );
        assert_eq!(&decoded.frames[1].pixels[..4], &[0, 0, 0, 255]);
    }

    #[test]
    fn float_images_are_srgb_encoded_and_clipped() {
        let pixels = tone_map(vec![0.0, 0.2159, 1.0, 1.0, 1000.0, -1.0, 0.5, 0.5]);
        assert_eq!(
            &pixels[..4],
            &[0, 128, 255, 255],
            "linear 0.2159 is sRGB 128"
        );
        assert_eq!(&pixels[4..6], &[255, 0], "highlights clip, negatives clamp");
    }

    #[test]
    fn garbage_is_an_error_not_a_panic() {
        for format in [
            ImageFormat::Png,
            ImageFormat::Jpeg,
            ImageFormat::Gif,
            ImageFormat::WebP,
            ImageFormat::Tiff,
        ] {
            assert!(
                decode(b"definitely not an image", format).is_err(),
                "{format:?}"
            );
        }
        assert!(decode(b"", ImageFormat::Raw).is_err());
    }
}
