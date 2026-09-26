//! SVG documents, parsed once and rendered at whatever scale the view needs.
//! Scripts and animation are not supported.

use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use crate::decode::{DecodeError, Frame};

/// Renders are capped at this many pixels; larger zooms are scaled up.
const MAX_RENDER_PIXELS: f64 = 64.0 * 1024.0 * 1024.0;

/// System fonts, loaded once for text in SVGs.
fn fonts() -> Arc<usvg::fontdb::Database> {
    static FONTS: OnceLock<Arc<usvg::fontdb::Database>> = OnceLock::new();
    Arc::clone(FONTS.get_or_init(|| {
        let mut database = usvg::fontdb::Database::new();
        database.load_system_fonts();
        Arc::new(database)
    }))
}

#[derive(Clone)]
pub struct Svg {
    tree: Arc<usvg::Tree>,
}

impl std::fmt::Debug for Svg {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.size();
        write!(formatter, "Svg({width}x{height})")
    }
}

impl Svg {
    /// Parses SVG or gzipped SVGZ. `resources` resolves relative image links.
    pub fn parse(bytes: &[u8], resources: Option<&Path>) -> Result<Self, DecodeError> {
        let options = usvg::Options {
            resources_dir: resources.map(Path::to_path_buf),
            fontdb: fonts(),
            ..usvg::Options::default()
        };
        let tree = usvg::Tree::from_data(bytes, &options)
            .map_err(|error| DecodeError::Invalid(error.to_string()))?;
        Ok(Self {
            tree: Arc::new(tree),
        })
    }

    pub fn parse_file(path: &Path) -> Result<Self, DecodeError> {
        let bytes = std::fs::read(path).map_err(|error| DecodeError::Io(error.to_string()))?;
        Self::parse(&bytes, path.parent())
    }

    /// Intrinsic size in CSS pixels.
    pub fn size(&self) -> (f32, f32) {
        let size = self.tree.size();
        (size.width(), size.height())
    }

    /// The scale actually used for `scale`, after the size cap.
    pub fn effective_scale(&self, scale: f32) -> f32 {
        let (width, height) = self.size();
        let pixels = f64::from(width * scale) * f64::from(height * scale);
        if pixels > MAX_RENDER_PIXELS {
            scale * (MAX_RENDER_PIXELS / pixels).sqrt() as f32
        } else {
            scale
        }
    }

    /// Renders the whole drawing at `scale` pixels per CSS pixel.
    pub fn render(&self, scale: f32) -> Result<Frame, DecodeError> {
        let scale = self.effective_scale(scale);
        let (width, height) = self.size();
        let pixel_width = (width * scale).ceil().max(1.0) as u32;
        let pixel_height = (height * scale).ceil().max(1.0) as u32;
        let mut pixmap = tiny_skia::Pixmap::new(pixel_width, pixel_height)
            .ok_or_else(|| DecodeError::Invalid("drawing is too large".into()))?;
        resvg::render(
            &self.tree,
            tiny_skia::Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );
        let pixels = pixmap.pixels().iter().flat_map(|pixel| {
            let color = pixel.demultiply();
            [color.red(), color.green(), color.blue(), color.alpha()]
        });
        Ok(Frame {
            width: pixel_width,
            height: pixel_height,
            pixels: pixels.collect(),
            delay: Duration::ZERO,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRAWING: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20">
        <rect x="0" y="0" width="20" height="20" fill="#ff0000"/>
        <circle cx="30" cy="10" r="5" fill="#0000ff" fill-opacity="0.5"/>
    </svg>"##;

    fn pixel(frame: &Frame, x: u32, y: u32) -> [u8; 4] {
        let offset = ((y * frame.width + x) * 4) as usize;
        frame.pixels[offset..offset + 4].try_into().unwrap()
    }

    #[test]
    fn renders_at_any_scale() {
        let svg = Svg::parse(DRAWING, None).unwrap();
        assert_eq!(svg.size(), (40.0, 20.0));
        let small = svg.render(1.0).unwrap();
        assert_eq!((small.width, small.height), (40, 20));
        assert_eq!(pixel(&small, 5, 5), [255, 0, 0, 255]);
        let large = svg.render(4.0).unwrap();
        assert_eq!((large.width, large.height), (160, 80));
        assert_eq!(
            pixel(&large, 120, 40)[2],
            255,
            "half transparent blue is not premultiplied"
        );
        assert!((120..=135).contains(&pixel(&large, 120, 40)[3]));
        assert_eq!(
            pixel(&large, 150, 5)[3],
            0,
            "outside the shapes is transparent"
        );
    }

    #[test]
    fn huge_scales_are_capped() {
        let svg = Svg::parse(DRAWING, None).unwrap();
        let scale = svg.effective_scale(10_000.0);
        assert!(scale < 10_000.0);
        let pixels = f64::from(40.0 * scale) * f64::from(20.0 * scale);
        assert!(pixels <= MAX_RENDER_PIXELS * 1.001);
    }

    #[test]
    fn reads_svgz_and_rejects_garbage() {
        use std::io::Write;
        let mut gzip = Vec::new();
        {
            let mut encoder = flate2::write::GzEncoder::new(&mut gzip, flate2::Compression::fast());
            encoder.write_all(DRAWING).unwrap();
        }
        assert_eq!(Svg::parse(&gzip, None).unwrap().size(), (40.0, 20.0));
        assert!(Svg::parse(b"<html>not svg</html>", None).is_err());
    }
}
