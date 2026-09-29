//! Exporting pages as images, and the choices the export dialog offers.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::i18n::Describe;
use prev_image::decode::Frame;
use prev_image::encode::{self, SaveFormat};
use prev_pdf::engine::{Bitmap, PageDisplay, page_pixels};
use prev_pdf::geometry::PixelRect;

/// What the export dialog writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Pdf,
    Png,
    Jpeg,
    Tiff,
    WebP,
    OpenExr,
}

impl Format {
    pub const ALL: [Format; 6] = [
        Format::Pdf,
        Format::Png,
        Format::Jpeg,
        Format::Tiff,
        Format::WebP,
        Format::OpenExr,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Format::Pdf => "PDF",
            Format::Png => "PNG",
            Format::Jpeg => "JPEG",
            Format::Tiff => "TIFF",
            Format::WebP => "WebP",
            Format::OpenExr => "OpenEXR",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Format::Pdf => "pdf",
            Format::Png => "png",
            Format::Jpeg => "jpg",
            Format::Tiff => "tiff",
            Format::WebP => "webp",
            Format::OpenExr => "exr",
        }
    }

    /// Whether every page goes into one file.
    pub fn is_multipage(self) -> bool {
        matches!(self, Format::Pdf | Format::Tiff)
    }

    fn save_format(self, quality: u8) -> SaveFormat {
        match self {
            Format::Jpeg => SaveFormat::Jpeg { quality },
            Format::WebP => SaveFormat::WebP,
            Format::OpenExr => SaveFormat::OpenExr,
            Format::Tiff => SaveFormat::Tiff,
            Format::Pdf | Format::Png => SaveFormat::Png,
        }
    }
}

pub const RESOLUTIONS: [f32; 4] = [72.0, 150.0, 300.0, 600.0];
/// JPEG quality choices as (label, quality).
pub fn qualities() -> [(String, u8); 4] {
    [
        (crate::fl!("pages-export-quality-low"), 50),
        (crate::fl!("pages-export-quality-medium"), 75),
        (crate::fl!("pages-export-quality-high"), 92),
        (crate::fl!("pages-export-quality-best"), 98),
    ]
}

/// The file for page `index` of `count` when each page gets its own:
/// "name.png" becomes "name-3.png".
pub fn page_path(target: &Path, index: usize, count: usize) -> PathBuf {
    if count <= 1 {
        return target.to_owned();
    }
    let stem = target
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    let digits = count.to_string().len();
    let name = match target.extension() {
        Some(extension) => format!(
            "{stem}-{:0digits$}.{}",
            index + 1,
            extension.to_string_lossy()
        ),
        None => format!("{stem}-{:0digits$}", index + 1),
    };
    target.with_file_name(name)
}

fn render(display: &Arc<dyn PageDisplay>, dpi: f32) -> Result<Bitmap, String> {
    let scale = dpi / 72.0;
    let (width, height) = page_pixels(display.size(), scale);
    display
        .render(
            scale,
            PixelRect {
                x: 0,
                y: 0,
                width,
                height,
            },
        )
        .map_err(|error| error.describe())
}

fn rgb(bitmap: &Bitmap) -> Vec<u8> {
    let (pixels, _) = bitmap.pixels.as_chunks::<4>();
    pixels
        .iter()
        .flat_map(|[red, green, blue, _]| [*red, *green, *blue])
        .collect()
}

/// One TIFF with a page per image, LZW compressed.
fn write_tiff(target: &Path, pages: &[Arc<dyn PageDisplay>], dpi: f32) -> Result<(), String> {
    use tiff::encoder::{Compression, Rational, TiffEncoder, colortype};
    use tiff::tags::ResolutionUnit;
    let error = |error: tiff::TiffError| error.to_string();
    let mut bytes = std::io::Cursor::new(Vec::new());
    {
        let mut encoder = TiffEncoder::new(&mut bytes)
            .map_err(error)?
            .with_compression(Compression::Lzw);
        for display in pages {
            let bitmap = render(display, dpi)?;
            let mut image = encoder
                .new_image::<colortype::RGB8>(bitmap.width, bitmap.height)
                .map_err(error)?;
            image.resolution(
                ResolutionUnit::Inch,
                Rational {
                    n: dpi.round() as u32,
                    d: 1,
                },
            );
            image.write_data(&rgb(&bitmap)).map_err(error)?;
        }
    }
    prev_store::atomic::write(target, bytes.get_ref()).map_err(|error| error.to_string())
}

/// Renders `pages` at `dpi` and writes them to `target`: one TIFF, or a
/// file per page for other formats. Returns the files written.
pub fn write_images(
    target: &Path,
    pages: &[Arc<dyn PageDisplay>],
    format: Format,
    dpi: f32,
    quality: u8,
) -> Result<Vec<PathBuf>, String> {
    if format == Format::Tiff {
        write_tiff(target, pages, dpi)?;
        return Ok(vec![target.to_owned()]);
    }
    let mut written = Vec::new();
    for (index, display) in pages.iter().enumerate() {
        let bitmap = render(display, dpi)?;
        let frame = Frame {
            width: bitmap.width,
            height: bitmap.height,
            pixels: bitmap.pixels,
            delay: std::time::Duration::ZERO,
        };
        let bytes = encode::encode(&frame, format.save_format(quality))
            .map_err(|error| error.describe())?;
        let path = page_path(target, index, pages.len());
        prev_store::atomic::write(&path, &bytes).map_err(|error| error.to_string())?;
        written.push(path);
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_files_are_numbered() {
        let target = Path::new("/tmp/report.png");
        assert_eq!(page_path(target, 0, 1), target);
        assert_eq!(page_path(target, 2, 12), Path::new("/tmp/report-03.png"));
        assert_eq!(page_path(target, 0, 3), Path::new("/tmp/report-1.png"));
    }
}
