//! Compares RAW rendering with the camera's JPEG: writes, for each file
//! given, a PNG with rawler's plain development, prev's matched curve and
//! the camera preview side by side.
//!
//! cargo run --release --example raw_look -- photo.cr2 [out-dir]

use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, RgbaImage};
use prev_image::ImageFormat;
use rawler::decoders::RawDecodeParams;
use rawler::imgop::develop::RawDevelop;
use rawler::rawsource::RawSource;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let out = if args.len() > 1 && std::path::Path::new(args.last().unwrap()).is_dir() {
        args.pop().unwrap()
    } else {
        ".".into()
    };
    for path in args {
        let bytes = std::fs::read(&path).expect("read");
        let source = RawSource::new_from_slice(&bytes);
        let params = RawDecodeParams { image_index: 0 };
        let raw = rawler::decode(&source, &params).expect("decode");
        let plain = RawDevelop::default()
            .develop_intermediate(&raw)
            .expect("develop")
            .to_dynamic_image()
            .expect("image");
        let started = std::time::Instant::now();
        let decoded = prev_image::decode::decode(&bytes, ImageFormat::Raw).expect("prev decode");
        let elapsed = started.elapsed();
        let frame = &decoded.frames[0];
        let matched = DynamicImage::ImageRgba8(
            RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone()).unwrap(),
        );
        let decoder = rawler::get_decoder(&source).expect("decoder");
        let camera = [
            decoder.full_image(&source, &params),
            decoder.preview_image(&source, &params),
            decoder.thumbnail_image(&source, &params),
        ]
        .into_iter()
        .filter_map(|image| image.ok().flatten())
        .max_by_key(|image| image.width() * image.height());
        let height = 500;
        let fit = |image: &DynamicImage| {
            image
                .resize(10_000, height, FilterType::Triangle)
                .into_rgba8()
        };
        let mut panels = vec![fit(&plain), fit(&matched)];
        if let Some(camera) = &camera {
            panels.push(fit(camera));
        }
        let width: u32 = panels.iter().map(|panel| panel.width() + 8).sum();
        let mut sheet = RgbaImage::from_pixel(width, height, image::Rgba([40, 40, 40, 255]));
        let mut x = 0;
        for panel in &panels {
            sheet.copy_from(panel, x, 0).unwrap();
            x += panel.width() + 8;
        }
        let name = std::path::Path::new(&path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let target = std::path::Path::new(&out).join(format!("{name}.look.png"));
        sheet.save(&target).expect("save");
        println!(
            "{name}: prev decode {elapsed:.2?}, camera preview {:?} -> {}",
            camera.map(|c| (c.width(), c.height())),
            target.display()
        );
    }
}
