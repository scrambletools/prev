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
