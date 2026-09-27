//! Camera RAW through rawler: decode the sensor data, develop it to linear
//! light with demosaicing, white balance and camera calibration, then give
//! it the camera's own look with a tone curve matched to the JPEG preview
//! the camera embeds in the file.

use image::metadata::Orientation;
use image::{DynamicImage, RgbaImage};
use rawler::decoders::RawDecodeParams;
use rawler::imgop::develop::{Intermediate, ProcessingStep, RawDevelop};
use rawler::rawsource::RawSource;

use crate::decode::{DecodeError, Decoded, to_frame};
use crate::tone::{ToneCurve, srgb_encode};

/// Pixels sampled from each image to build the histograms.
const SAMPLES: usize = 1 << 20;

pub fn decode(bytes: &[u8]) -> Result<Decoded, DecodeError> {
    let invalid = |error: rawler::RawlerError| DecodeError::Invalid(error.to_string());
    let source = RawSource::new_from_slice(bytes);
    let params = RawDecodeParams { image_index: 0 };
    let raw = rawler::decode(&source, &params).map_err(invalid)?;
    let orientation = orientation(raw.orientation);
    // Everything but the final sRGB encoding: the curve does that.
    let linear = RawDevelop::new_with(&[
        ProcessingStep::Rescale,
        ProcessingStep::Demosaic,
        ProcessingStep::FujiRotate,
        ProcessingStep::CropActiveArea,
        ProcessingStep::WhiteBalance,
        ProcessingStep::Calibrate,
        ProcessingStep::CropDefault,
    ])
    .develop_intermediate(&raw)
    .map_err(invalid)?;
    let (width, height, pixels) = match linear {
        Intermediate::ThreeColor(pixels) => (pixels.width, pixels.height, pixels.data),
        Intermediate::Monochrome(pixels) => (
            pixels.width,
            pixels.height,
            pixels.data.into_iter().map(|value| [value; 3]).collect(),
        ),
        Intermediate::FourColor(_) => {
            return Err(DecodeError::Invalid(
                "four color sensors are not supported".into(),
            ));
        }
    };
    let curves = camera_preview(&source, &params)
        .and_then(|preview| matched_curves(&sample(&pixels), &preview))
        .unwrap_or_else(|| {
            [
                ToneCurve::standard(),
                ToneCurve::standard(),
                ToneCurve::standard(),
            ]
        });
    let mut rgba = Vec::with_capacity(pixels.len() * 4);
    for pixel in &pixels {
        let [red, green, blue] =
            [0, 1, 2].map(|channel| curves[channel].at(srgb_encode(pixel[channel])));
        let byte = |value: f32| (value * 255.0 + 0.5) as u8;
        rgba.extend_from_slice(&[byte(red), byte(green), byte(blue), 255]);
    }
    let image = RgbaImage::from_raw(width as u32, height as u32, rgba)
        .ok_or_else(|| DecodeError::Invalid("developed image has an unexpected size".into()))?;
    let mut image = DynamicImage::ImageRgba8(image);
    image.apply_orientation(orientation);
    Ok(Decoded {
        frames: vec![to_frame(image)],
    })
}

/// A curve per channel, each matching the channel's histogram to the
/// camera's: they carry the camera's brightness, contrast and color
/// balance. Measured against the camera JPEGs of sample files, this comes
/// closer than one curve for all channels or a brightness curve alone.
fn matched_curves(raw: &[[f32; 3]], camera: &[[u8; 3]]) -> Option<[ToneCurve; 3]> {
    let curve = |channel: usize| {
        ToneCurve::matched_channel(
            raw.iter().map(|pixel| pixel[channel]),
            camera.iter().map(|pixel| pixel[channel]),
        )
    };
    Some([curve(0)?, curve(1)?, curve(2)?])
}

/// Every nth pixel, about `SAMPLES` of them.
fn sample<T: Copy>(pixels: &[T]) -> Vec<T> {
    let step = (pixels.len() / SAMPLES).max(1);
    pixels.iter().step_by(step).copied().collect()
}

/// The largest JPEG the camera stored with the RAW, without black bars
/// around it, as RGB pixels.
fn camera_preview(source: &RawSource, params: &RawDecodeParams) -> Option<Vec<[u8; 3]>> {
    let decoder = rawler::get_decoder(source).ok()?;
    let preview = [
        decoder.full_image(source, params),
        decoder.preview_image(source, params),
        decoder.thumbnail_image(source, params),
    ]
    .into_iter()
    .filter_map(|image| image.ok().flatten())
    .max_by_key(|image| u64::from(image.width()) * u64::from(image.height()))?
    .into_rgb8();
    let image = trim_black_bars(&preview);
    let pixels: Vec<[u8; 3]> = image.pixels().map(|pixel| pixel.0).collect();
    // Tiny thumbnails say little about the curve.
    (pixels.len() >= 160 * 120).then(|| sample(&pixels))
}

/// Crops rows and columns along the edges that are all but black, which
/// some cameras add to fit a preview to the screen's shape.
fn trim_black_bars(image: &image::RgbImage) -> image::RgbImage {
    let dark = |pixel: &image::Rgb<u8>| pixel.0.iter().all(|channel| *channel < 12);
    let (width, height) = image.dimensions();
    let row_dark = |y: u32| (0..width).all(|x| dark(image.get_pixel(x, y)));
    let column_dark =
        |x: u32, top: u32, bottom: u32| (top..bottom).all(|y| dark(image.get_pixel(x, y)));
    let mut top = 0;
    while top < height / 3 && row_dark(top) {
        top += 1;
    }
    let mut bottom = height;
    while bottom > height * 2 / 3 && row_dark(bottom - 1) {
        bottom -= 1;
    }
    let mut left = 0;
    while left < width / 3 && column_dark(left, top, bottom) {
        left += 1;
    }
    let mut right = width;
    while right > width * 2 / 3 && column_dark(right - 1, top, bottom) {
        right -= 1;
    }
    image::imageops::crop_imm(image, left, top, right - left, bottom - top).to_image()
}

fn orientation(orientation: rawler::Orientation) -> Orientation {
    use rawler::Orientation as Raw;
    match orientation {
        Raw::Normal | Raw::Unknown => Orientation::NoTransforms,
        Raw::HorizontalFlip => Orientation::FlipHorizontal,
        Raw::Rotate180 => Orientation::Rotate180,
        Raw::VerticalFlip => Orientation::FlipVertical,
        Raw::Transpose => Orientation::Rotate90FlipH,
        Raw::Rotate90 => Orientation::Rotate90,
        Raw::Transverse => Orientation::Rotate270FlipH,
        Raw::Rotate270 => Orientation::Rotate270,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garbage_is_an_error() {
        assert!(matches!(
            decode(b"II*\0 not really a raw file"),
            Err(DecodeError::Invalid(_))
        ));
    }

    #[test]
    fn matched_curves_take_on_the_camera_color_balance() {
        // A RAW with a blue cast, and a camera JPEG of the same gray ramp
        // rendered neutral: after the curves, the channels agree.
        let raw: Vec<[f32; 3]> = (0..10_000)
            .map(|index| {
                let value = (index % 500) as f32 / 499.0 * 0.8;
                [value * 0.8, value, value * 1.2]
            })
            .collect();
        let camera: Vec<[u8; 3]> = raw
            .iter()
            .map(|pixel| {
                let byte = (srgb_encode(pixel[1]) * 255.0).round() as u8;
                [byte; 3]
            })
            .collect();
        let curves = matched_curves(&raw, &camera).unwrap();
        for pixel in raw.iter().step_by(97) {
            let out = [0, 1, 2].map(|channel| curves[channel].at(srgb_encode(pixel[channel])));
            assert!((out[0] - out[2]).abs() < 0.04, "{out:?} for {pixel:?}");
        }
    }

    #[test]
    fn black_bars_are_trimmed() {
        let mut image = image::RgbImage::from_pixel(100, 60, image::Rgb([0, 0, 0]));
        for y in 10..50 {
            for x in 0..100 {
                image.put_pixel(x, y, image::Rgb([200, 120, 40]));
            }
        }
        let trimmed = trim_black_bars(&image);
        assert_eq!(trimmed.dimensions(), (100, 40));
    }
}
