//! Scores ways of giving a RAW the camera's look: the mean difference,
//! per channel on a 0 to 255 scale, from the camera's JPEG at a small size.

use image::imageops::FilterType;
use image::{DynamicImage, RgbImage};
use prev_image::tone::ToneCurve;
use rawler::decoders::RawDecodeParams;
use rawler::imgop::develop::{Intermediate, ProcessingStep, RawDevelop};
use rawler::rawsource::RawSource;

fn score(ours: &RgbImage, camera: &RgbImage) -> f64 {
    let camera = image::imageops::resize(camera, ours.width(), ours.height(), FilterType::Triangle);
    let mut total = 0.0;
    for (a, b) in ours.pixels().zip(camera.pixels()) {
        for c in 0..3 {
            total += (f64::from(a.0[c]) - f64::from(b.0[c])).abs();
        }
    }
    total / (ours.width() * ours.height() * 3) as f64
}

fn main() {
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).unwrap();
        let source = RawSource::new_from_slice(&bytes);
        let params = RawDecodeParams { image_index: 0 };
        let raw = rawler::decode(&source, &params).unwrap();
        let steps = [
            ProcessingStep::Rescale,
            ProcessingStep::Demosaic,
            ProcessingStep::FujiRotate,
            ProcessingStep::CropActiveArea,
            ProcessingStep::WhiteBalance,
            ProcessingStep::Calibrate,
            ProcessingStep::CropDefault,
        ];
        let Intermediate::ThreeColor(pixels) = RawDevelop::new_with(&steps)
            .develop_intermediate(&raw)
            .unwrap()
        else {
            continue;
        };
        let decoder = rawler::get_decoder(&source).unwrap();
        let camera = [
            decoder.full_image(&source, &params),
            decoder.preview_image(&source, &params),
        ]
        .into_iter()
        .filter_map(|i| i.ok().flatten())
        .max_by_key(|i| i.width() * i.height())
        .unwrap()
        .into_rgb8();
        let cam_px: Vec<[u8; 3]> = camera.pixels().map(|p| p.0).collect();
        let step = (pixels.data.len() / (1 << 20)).max(1);
        let sampled: Vec<[f32; 3]> = pixels.data.iter().step_by(step).copied().collect();
        let (w, h) = (pixels.width as u32, pixels.height as u32);
        let render = |f: &dyn Fn([f32; 3]) -> [f32; 3]| {
            let data: Vec<u8> = pixels
                .data
                .iter()
                .flat_map(|p| f(*p).map(|v| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8))
                .collect();
            let image = DynamicImage::ImageRgb8(RgbImage::from_raw(w, h, data).unwrap());
            image.resize(400, 400, FilterType::Triangle).into_rgb8()
        };
        let plain = render(&|p| p.map(prev_image::tone::srgb_encode));
        let luma_curve = ToneCurve::matched(&sampled, &cam_px).unwrap();
        let per_channel_one = render(&|p| luma_curve.apply(p));
        let luminance_only = render(&|p| luma_curve.apply_luminance(p));
        let curves: Vec<ToneCurve> = (0..3)
            .map(|c| {
                ToneCurve::matched_channel(
                    sampled.iter().map(|p| p[c]),
                    cam_px.iter().map(|p| p[c]),
                )
                .unwrap()
            })
            .collect();
        let three =
            render(&|p| [0, 1, 2].map(|c| curves[c].at(prev_image::tone::srgb_encode(p[c]))));
        let name = std::path::Path::new(&path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        println!(
            "{name}: plain {:.1}, one curve {:.1}, luminance only {:.1}, three curves {:.1}",
            score(&plain, &camera),
            score(&per_channel_one, &camera),
            score(&luminance_only, &camera),
            score(&three, &camera)
        );
    }
}
