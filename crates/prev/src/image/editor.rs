//! Editing one image: the edit stack over the decoded original, rendering
//! results off the UI thread, and saving, exporting and metadata changes.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::futures::channel::oneshot;
use prev_image::ImageFormat;
use prev_image::decode::Frame;
use prev_image::edit::{CropRect, EditStack, Operation};
use prev_image::encode::{self, DEFAULT_JPEG_QUALITY, SaveFormat};
use prev_store::versions::VersionStore;

/// Longest side of the quick preview shown while sliders move.
const PREVIEW_SIDE: u32 = 1600;

pub struct Editor {
    pub stack: EditStack,
    /// The decoded original, orientation applied.
    source: Arc<Frame>,
    /// A small copy of the original, for quick previews.
    preview_source: Arc<Frame>,
    /// Increments on every change; results for older ones are dropped.
    pub generation: u64,
    /// The generation whose result was last written to disk.
    pub saved_generation: u64,
    /// Whether this session already kept the original as a version.
    pub original_kept: bool,
}

fn scaled_copy(frame: &Frame, longest: u32) -> Frame {
    let scale = longest as f32 / frame.width.max(frame.height).max(1) as f32;
    if scale >= 1.0 {
        return frame.clone();
    }
    let image = image::RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone())
        .expect("frame size");
    let width = ((frame.width as f32 * scale).round() as u32).max(1);
    let height = ((frame.height as f32 * scale).round() as u32).max(1);
    let small =
        image::imageops::resize(&image, width, height, image::imageops::FilterType::Triangle);
    Frame {
        width,
        height,
        pixels: small.into_raw(),
        delay: frame.delay,
    }
}

/// Runs `work` on its own thread and resolves with its result.
pub fn spawn<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> oneshot::Receiver<T> {
    let (sender, receiver) = oneshot::channel();
    std::thread::Builder::new()
        .name("prev-image-edit".into())
        .spawn(move || {
            let _ = sender.send(work());
        })
        .expect("spawn edit thread");
    receiver
}

impl Editor {
    pub fn new(source: Frame) -> Self {
        let preview_source = scaled_copy(&source, PREVIEW_SIDE);
        Self {
            stack: EditStack::default(),
            source: Arc::new(source),
            preview_source: Arc::new(preview_source),
            generation: 0,
            saved_generation: 0,
            original_kept: false,
        }
    }

    pub fn source_size(&self) -> (u32, u32) {
        (self.source.width, self.source.height)
    }

    pub fn output_size(&self) -> (u32, u32) {
        self.stack.output_size(self.source_size())
    }

    pub fn is_saved(&self) -> bool {
        self.generation == self.saved_generation
    }

    /// Applies a change to the stack; returns whether anything changed.
    pub fn change(&mut self, change: impl FnOnce(&mut EditStack) -> bool) -> bool {
        let changed = change(&mut self.stack);
        if changed {
            self.generation += 1;
        }
        changed
    }

    /// A job rendering the edits at preview size.
    pub fn render_preview(&self) -> impl FnOnce() -> Frame + Send + 'static {
        let (stack, source) = (self.stack.clone(), Arc::clone(&self.preview_source));
        let (full_width, full_height) = self.source_size();
        move || {
            // Crops and resizes are in full-size pixels; scale them down.
            let scale = source.width as f32 / full_width.max(1) as f32;
            let scale_y = source.height as f32 / full_height.max(1) as f32;
            let mut scaled = EditStack::default();
            for operation in stack.operations() {
                scaled.push(match *operation {
                    Operation::Crop(rect) => Operation::Crop(CropRect {
                        x: (rect.x as f32 * scale) as u32,
                        y: (rect.y as f32 * scale_y) as u32,
                        width: ((rect.width as f32 * scale) as u32).max(1),
                        height: ((rect.height as f32 * scale_y) as u32).max(1),
                    }),
                    Operation::Resize { width, height } => Operation::Resize {
                        width: ((width as f32 * scale) as u32).max(1),
                        height: ((height as f32 * scale_y) as u32).max(1),
                    },
                    other => other,
                });
            }
            scaled.apply(&source)
        }
    }

    /// A job rendering the edits at full size.
    pub fn render_full(&self) -> impl FnOnce() -> Frame + Send + 'static {
        let (stack, source) = (self.stack.clone(), Arc::clone(&self.source));
        move || stack.apply(&source)
    }
}

/// Writes an edited image over its file: keeps the original as a version
/// first (once per session), carries the metadata over, writes atomically.
pub fn save_in_place(
    path: &Path,
    frame: &Frame,
    format: SaveFormat,
    keep_original: bool,
) -> Result<(), String> {
    let original = std::fs::read(path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    if keep_original && let Some(store) = VersionStore::default_location() {
        store
            .keep(path)
            .map_err(|error| format!("Could not keep the original version: {error}"))?;
    }
    let encoded = encode::encode(frame, format).map_err(|error| error.to_string())?;
    let bytes = encode::carry_metadata(&original, encoded);
    prev_store::atomic::write(path, &bytes)
        .map_err(|error| format!("Could not save {}: {error}", path.display()))
}

/// Rewrites a file through a metadata edit, keeping a version first.
pub fn edit_metadata(
    path: &Path,
    keep_original: bool,
    edit: impl FnOnce(Vec<u8>) -> Result<Vec<u8>, String>,
) -> Result<(), String> {
    let original = std::fs::read(path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    if keep_original && let Some(store) = VersionStore::default_location() {
        store
            .keep(path)
            .map_err(|error| format!("Could not keep the original version: {error}"))?;
    }
    let updated = edit(original)?;
    prev_store::atomic::write(path, &updated)
        .map_err(|error| format!("Could not save {}: {error}", path.display()))
}

/// Export formats as (choice id, menu label, extension).
pub const EXPORT_FORMATS: &[(&str, &str, &str)] = &[
    ("png", "PNG", "png"),
    ("jpeg", "JPEG", "jpg"),
    ("webp", "WebP (lossless)", "webp"),
    ("tiff", "TIFF", "tiff"),
    ("bmp", "BMP", "bmp"),
    ("tga", "TGA", "tga"),
    ("qoi", "QOI", "qoi"),
    ("ppm", "PPM", "ppm"),
    ("exr", "OpenEXR", "exr"),
];

/// JPEG quality choices as (choice id, menu label, quality).
pub const JPEG_QUALITIES: &[(&str, &str, u8)] = &[
    ("low", "Low", 50),
    ("medium", "Medium", 75),
    ("high", "High", DEFAULT_JPEG_QUALITY),
    ("best", "Best", 98),
];

pub const DEFAULT_QUALITY_CHOICE: &str = "high";

pub fn format_for_choice(format: &str, quality: &str) -> Option<SaveFormat> {
    let quality = JPEG_QUALITIES
        .iter()
        .find(|(id, ..)| *id == quality)
        .map_or(DEFAULT_JPEG_QUALITY, |(.., q)| *q);
    Some(match format {
        "png" => SaveFormat::Png,
        "jpeg" => SaveFormat::Jpeg { quality },
        "webp" => SaveFormat::WebP,
        "tiff" => SaveFormat::Tiff,
        "bmp" => SaveFormat::Bmp,
        "tga" => SaveFormat::Tga,
        "qoi" => SaveFormat::Qoi,
        "ppm" => SaveFormat::Pnm,
        "exr" => SaveFormat::OpenExr,
        _ => return None,
    })
}

/// The suggested export format: the same one if prev can write it,
/// otherwise JPEG, as Preview suggests for RAW and HEIC.
pub fn default_format_choice(format: ImageFormat, animated: bool) -> &'static str {
    match SaveFormat::for_saving(format, animated) {
        Some(SaveFormat::Png) => "png",
        Some(SaveFormat::WebP) => "webp",
        Some(SaveFormat::Tiff) => "tiff",
        Some(SaveFormat::Bmp) => "bmp",
        Some(SaveFormat::Tga) => "tga",
        Some(SaveFormat::Qoi) => "qoi",
        Some(SaveFormat::Pnm) => "ppm",
        Some(SaveFormat::OpenExr) => "exr",
        Some(SaveFormat::Jpeg { .. }) | None => "jpeg",
    }
}

/// Whether the name's extension fits `format` (".jpeg" and ".jpg" both
/// fit JPEG, ".tif" and ".tiff" fit TIFF).
pub fn extension_matches(path: &Path, format: SaveFormat) -> bool {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase);
    match (format, extension.as_deref()) {
        (SaveFormat::Jpeg { .. }, Some("jpg" | "jpeg")) => true,
        (SaveFormat::Tiff, Some("tif" | "tiff")) => true,
        (SaveFormat::Pnm, Some("ppm" | "pnm")) => true,
        (format, Some(extension)) => extension == format.extension(),
        (_, None) => false,
    }
}

/// The menu label of a format choice, for messages.
pub fn format_label(choice: &str) -> &'static str {
    EXPORT_FORMATS
        .iter()
        .find(|(id, ..)| *id == choice)
        .map_or("image", |(_, label, _)| label)
}

/// The suggested export name for `path` in the suggested format.
pub fn export_name(path: &Path, format: ImageFormat, animated: bool) -> String {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_else(|| "image".into());
    let choice = default_format_choice(format, animated);
    let extension = EXPORT_FORMATS
        .iter()
        .find(|(id, ..)| *id == choice)
        .map_or("jpg", |(.., ext)| ext);
    format!("{stem}.{extension}")
}

pub fn export(
    original: &Path,
    frame: &Frame,
    target: &Path,
    format: SaveFormat,
) -> Result<(), String> {
    let encoded = encode::encode(frame, format).map_err(|error| error.to_string())?;
    let bytes = match std::fs::read(original) {
        Ok(original) => encode::carry_metadata(&original, encoded),
        Err(_) => encoded,
    };
    prev_store::atomic::write(target, &bytes).map_err(|error| format!("Could not export: {error}"))
}

/// Longest side, in points, of the page an image is marked up on, so
/// markup comes out at a size that suits the image whatever its pixels.
const MARKUP_PAGE: f32 = 800.0;

/// Image pixels per point of the page a `width` by `height` image is
/// marked up on.
pub fn markup_scale(width: u32, height: u32) -> f32 {
    (width.max(height) as f32 / MARKUP_PAGE).max(1.0)
}

/// Writes a one-page PDF of `frame` to a private temporary file, for
/// marking it up with the PDF tools. The caller deletes the file.
pub fn markup_document(frame: &Frame) -> Result<PathBuf, String> {
    let scale = markup_scale(frame.width, frame.height);
    let size =
        prev_pdf::geometry::Size::new(frame.width as f32 / scale, frame.height as f32 / scale);
    let bitmap = prev_pdf::engine::Bitmap {
        width: frame.width,
        height: frame.height,
        pixels: frame.pixels.clone(),
    };
    let bytes = prev_pdf::image_document(&bitmap, size).map_err(|error| error.to_string())?;
    let file = tempfile::Builder::new()
        .prefix("prev-markup-")
        .suffix(".pdf")
        .tempfile()
        .map_err(|error| format!("Could not start the markup: {error}"))?;
    std::fs::write(file.path(), bytes)
        .map_err(|error| format!("Could not start the markup: {error}"))?;
    let (_, path) = file
        .keep()
        .map_err(|error| format!("Could not start the markup: {error}"))?;
    Ok(path)
}

/// `frame` with `layer`, RGBA pixels of the same size, drawn over it.
pub fn burn_in(frame: &Frame, layer: &[u8]) -> Frame {
    let mut pixels = frame.pixels.clone();
    for (under, over) in pixels
        .as_chunks_mut::<4>()
        .0
        .iter_mut()
        .zip(layer.as_chunks::<4>().0)
    {
        let top = f32::from(over[3]) / 255.0;
        if top == 0.0 {
            continue;
        }
        let bottom = f32::from(under[3]) / 255.0 * (1.0 - top);
        let alpha = top + bottom;
        for channel in 0..3 {
            let value =
                (f32::from(over[channel]) * top + f32::from(under[channel]) * bottom) / alpha;
            under[channel] = value.round().clamp(0.0, 255.0) as u8;
        }
        under[3] = (alpha * 255.0).round() as u8;
    }
    Frame {
        width: frame.width,
        height: frame.height,
        pixels,
        delay: frame.delay,
    }
}

/// Adjust Size: the other dimension for a new width or height, keeping the
/// aspect ratio of `size`.
pub fn proportional(size: (u32, u32), width: Option<u32>, height: Option<u32>) -> (u32, u32) {
    let (current_width, current_height) = (size.0.max(1) as f64, size.1.max(1) as f64);
    match (width, height) {
        (Some(width), _) => (
            width.max(1),
            ((width as f64 * current_height / current_width).round() as u32).max(1),
        ),
        (None, Some(height)) => (
            ((height as f64 * current_width / current_height).round() as u32).max(1),
            height.max(1),
        ),
        (None, None) => size,
    }
}

/// A selection in image pixels, clamped to the image, as a crop.
pub fn selection_to_crop(selection: (f32, f32, f32, f32), size: (u32, u32)) -> Option<CropRect> {
    let (x0, y0, x1, y1) = selection;
    let (left, right) = (x0.min(x1).max(0.0), x0.max(x1).min(size.0 as f32));
    let (top, bottom) = (y0.min(y1).max(0.0), y0.max(y1).min(size.1 as f32));
    let crop = CropRect {
        x: left.round() as u32,
        y: top.round() as u32,
        width: (right - left).round() as u32,
        height: (bottom - top).round() as u32,
    };
    (crop.width >= 1 && crop.height >= 1).then_some(crop)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn markup_is_burned_in() {
        let frame = Frame {
            width: 3,
            height: 1,
            pixels: vec![200, 100, 0, 255, 200, 100, 0, 255, 0, 0, 0, 0],
            delay: Duration::ZERO,
        };
        // Clear, half-covering blue, and blue over a transparent pixel.
        let layer = [0, 0, 0, 0, 0, 0, 255, 128, 0, 0, 255, 255];
        let burned = burn_in(&frame, &layer);
        assert_eq!(&burned.pixels[0..4], &[200, 100, 0, 255]);
        assert_eq!(&burned.pixels[4..8], &[100, 50, 128, 255]);
        assert_eq!(&burned.pixels[8..12], &[0, 0, 255, 255]);
    }

    #[test]
    fn markup_pages_suit_the_image() {
        assert_eq!(markup_scale(400, 300), 1.0);
        assert_eq!(markup_scale(4000, 3000), 5.0);
        let frame = Frame {
            width: 1600,
            height: 1200,
            pixels: vec![255; 1600 * 1200 * 4],
            delay: Duration::ZERO,
        };
        let path = markup_document(&frame).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert!(bytes.starts_with(b"%PDF"));
    }

    #[test]
    fn export_choices() {
        assert_eq!(
            format_for_choice("jpeg", "best"),
            Some(SaveFormat::Jpeg { quality: 98 })
        );
        assert_eq!(
            format_for_choice("jpeg", "nonsense"),
            Some(SaveFormat::Jpeg {
                quality: DEFAULT_JPEG_QUALITY
            })
        );
        assert_eq!(format_for_choice("exr", "low"), Some(SaveFormat::OpenExr));
        assert_eq!(format_for_choice("heic", "high"), None);
        assert_eq!(default_format_choice(ImageFormat::Raw, false), "jpeg");
        assert_eq!(default_format_choice(ImageFormat::Png, false), "png");
        assert_eq!(default_format_choice(ImageFormat::Gif, true), "jpeg");
        assert_eq!(
            export_name(Path::new("/p/IMG_1.CR2"), ImageFormat::Raw, false),
            "IMG_1.jpg"
        );
        assert_eq!(
            export_name(Path::new("/p/shot.png"), ImageFormat::Png, false),
            "shot.png"
        );
        for (id, ..) in EXPORT_FORMATS {
            assert!(
                format_for_choice(id, DEFAULT_QUALITY_CHOICE).is_some(),
                "{id}"
            );
        }
    }

    #[test]
    fn extension_checks() {
        let jpeg = SaveFormat::Jpeg { quality: 90 };
        assert!(extension_matches(Path::new("/a/photo.JPEG"), jpeg));
        assert!(extension_matches(Path::new("/a/photo.jpg"), jpeg));
        assert!(!extension_matches(Path::new("/a/photo.png"), jpeg));
        assert!(!extension_matches(Path::new("/a/photo"), SaveFormat::Png));
        assert!(extension_matches(
            Path::new("/a/scan.tif"),
            SaveFormat::Tiff
        ));
        assert_eq!(format_label("webp"), "WebP (lossless)");
    }

    #[test]
    fn proportional_sizes() {
        assert_eq!(proportional((4000, 3000), Some(800), None), (800, 600));
        assert_eq!(proportional((4000, 3000), None, Some(300)), (400, 300));
        assert_eq!(proportional((10, 10), None, None), (10, 10));
    }

    #[test]
    fn selections_become_crops() {
        assert_eq!(
            selection_to_crop((50.0, 80.0, 10.0, 20.0), (100, 100)),
            Some(CropRect {
                x: 10,
                y: 20,
                width: 40,
                height: 60
            })
        );
        assert_eq!(
            selection_to_crop((-10.0, -10.0, 500.0, 50.0), (100, 100)),
            Some(CropRect {
                x: 0,
                y: 0,
                width: 100,
                height: 50
            })
        );
        assert_eq!(selection_to_crop((5.0, 5.0, 5.2, 50.0), (100, 100)), None);
    }

    #[test]
    fn previews_scale_crops_to_the_small_copy() {
        let pixels = [10u8, 20, 30, 255].repeat(3200 * 1600);
        let mut editor = Editor::new(Frame {
            width: 3200,
            height: 1600,
            pixels,
            delay: Duration::ZERO,
        });
        assert!(editor.change(|stack| {
            stack.push(Operation::Crop(CropRect {
                x: 0,
                y: 0,
                width: 1600,
                height: 800,
            }));
            true
        }));
        assert_eq!(editor.output_size(), (1600, 800));
        let preview = editor.render_preview()();
        assert_eq!(
            (preview.width, preview.height),
            (800, 400),
            "half of the 1600 wide preview"
        );
        assert!(!editor.is_saved());
    }

    #[test]
    fn saves_in_place_with_a_version() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pic.png");
        let frame = Frame {
            width: 2,
            height: 2,
            pixels: vec![255; 16],
            delay: Duration::ZERO,
        };
        std::fs::write(&path, encode::encode(&frame, SaveFormat::Png).unwrap()).unwrap();
        let dark = Frame {
            pixels: [0, 0, 0, 255].repeat(4),
            ..frame
        };
        save_in_place(&path, &dark, SaveFormat::Png, false).unwrap();
        let decoded = prev_image::decode::decode_file(&path, ImageFormat::Png).unwrap();
        assert_eq!(&decoded.frames[0].pixels[..4], &[0, 0, 0, 255]);
    }
}
