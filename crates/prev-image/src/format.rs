//! Image formats prev can open.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Avif,
    Heif,
    Bmp,
    Ico,
    Tiff,
    Tga,
    Pnm,
    Qoi,
    Hdr,
    OpenExr,
    Jpeg2000,
    Raw,
}

impl ImageFormat {
    /// The format's name, as people know it.
    pub fn name(self) -> &'static str {
        match self {
            ImageFormat::Png => "PNG",
            ImageFormat::Jpeg => "JPEG",
            ImageFormat::Gif => "GIF",
            ImageFormat::WebP => "WebP",
            ImageFormat::Avif => "AVIF",
            ImageFormat::Heif => "HEIF",
            ImageFormat::Bmp => "BMP",
            ImageFormat::Ico => "ICO",
            ImageFormat::Tiff => "TIFF",
            ImageFormat::Tga => "TGA",
            ImageFormat::Pnm => "PNM",
            ImageFormat::Qoi => "QOI",
            ImageFormat::Hdr => "Radiance HDR",
            ImageFormat::OpenExr => "OpenEXR",
            ImageFormat::Jpeg2000 => "JPEG 2000",
            ImageFormat::Raw => "Camera RAW",
        }
    }
}
