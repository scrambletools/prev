//! Detects which viewer a file needs, from its leading bytes first and its
//! extension second.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Bytes read from the start of a file for detection.
pub const SNIFF_LEN: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Pdf,
    Image(ImageFormat),
    Svg,
    Markdown,
}

pub use prev_image::ImageFormat;

const RAW_EXTENSIONS: &[&str] = &[
    "3fr", "arw", "cr2", "cr3", "crw", "dcr", "dng", "erf", "iiq", "kdc", "mef", "mos", "mrw",
    "nef", "nrw", "orf", "pef", "raf", "rw2", "rwl", "sr2", "srf", "srw", "x3f",
];

pub const PDF_MIME_TYPES: &[&str] = &["application/pdf"];

pub const IMAGE_MIME_TYPES: &[&str] = &[
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "image/avif",
    "image/heif",
    "image/heic",
    "image/bmp",
    "image/vnd.microsoft.icon",
    "image/x-icon",
    "image/tiff",
    "image/x-tga",
    "image/x-portable-anymap",
    "image/x-portable-bitmap",
    "image/x-portable-graymap",
    "image/x-portable-pixmap",
    "image/qoi",
    "image/vnd.radiance",
    "image/x-exr",
    "image/jp2",
    "image/jpx",
    "image/x-dcraw",
    "image/x-adobe-dng",
    "image/x-canon-cr2",
    "image/x-canon-cr3",
    "image/x-canon-crw",
    "image/x-fuji-raf",
    "image/x-nikon-nef",
    "image/x-nikon-nrw",
    "image/x-olympus-orf",
    "image/x-panasonic-rw2",
    "image/x-pentax-pef",
    "image/x-sony-arw",
    "image/x-sony-sr2",
    "image/x-sony-srf",
    "image/x-samsung-srw",
    "image/x-sigma-x3f",
];

pub const SVG_MIME_TYPES: &[&str] = &["image/svg+xml", "image/svg+xml-compressed"];

pub const MARKDOWN_MIME_TYPES: &[&str] = &["text/markdown"];

/// Every MIME type prev opens, in the order listed in the desktop file.
pub fn supported_mime_types() -> impl Iterator<Item = &'static str> {
    [
        PDF_MIME_TYPES,
        IMAGE_MIME_TYPES,
        SVG_MIME_TYPES,
        MARKDOWN_MIME_TYPES,
    ]
    .into_iter()
    .flatten()
    .copied()
}

pub const MARKDOWN_EXTENSIONS: &[&str] = &["md", "markdown", "mdown", "mkd", "mkdn"];

pub fn detect_path(path: &Path) -> io::Result<Option<FileKind>> {
    let mut header = Vec::with_capacity(SNIFF_LEN);
    File::open(path)?
        .take(SNIFF_LEN as u64)
        .read_to_end(&mut header)?;
    let extension = path.extension().and_then(|ext| ext.to_str());
    Ok(detect(&header, extension))
}

pub fn detect(header: &[u8], extension: Option<&str>) -> Option<FileKind> {
    let extension = extension.map(str::to_ascii_lowercase);
    let extension = extension.as_deref();
    let has_raw_extension = extension.is_some_and(|ext| RAW_EXTENSIONS.contains(&ext));

    if let Some(format) = sniff_image(header) {
        // Most camera RAW formats are TIFF containers, so trust the extension.
        if format == ImageFormat::Tiff && has_raw_extension {
            return Some(FileKind::Image(ImageFormat::Raw));
        }
        return Some(FileKind::Image(format));
    }
    if is_pdf(header) {
        return Some(FileKind::Pdf);
    }
    if is_svg(header) || (extension == Some("svgz") && header.starts_with(&[0x1f, 0x8b])) {
        return Some(FileKind::Svg);
    }

    match extension {
        Some(ext) if MARKDOWN_EXTENSIONS.contains(&ext) => Some(FileKind::Markdown),
        Some("tga") => Some(FileKind::Image(ImageFormat::Tga)),
        Some(_) if has_raw_extension => Some(FileKind::Image(ImageFormat::Raw)),
        _ => None,
    }
}

fn sniff_image(header: &[u8]) -> Option<ImageFormat> {
    use ImageFormat::*;
    let starts = |magic: &[u8]| header.starts_with(magic);

    if starts(b"\x89PNG\r\n\x1a\n") {
        Some(Png)
    } else if starts(&[0xff, 0xd8, 0xff]) {
        Some(Jpeg)
    } else if starts(b"GIF87a") || starts(b"GIF89a") {
        Some(Gif)
    } else if starts(b"RIFF") && header.get(8..12) == Some(b"WEBP") {
        Some(WebP)
    } else if let Some(brand) = iso_bmff_brand(header) {
        bmff_image_format(brand)
    } else if starts(b"FUJIFILMCCD-RAW") || starts(b"IIRO") || starts(b"IIRS") || starts(b"IIU\0") {
        Some(Raw)
    } else if starts(b"II*\0") || starts(b"MM\0*") || starts(b"II+\0") || starts(b"MM\0+") {
        Some(Tiff)
    } else if starts(b"BM") && header.len() >= 14 {
        Some(Bmp)
    } else if starts(&[0x00, 0x00, 0x01, 0x00])
        && header.get(4..6).is_some_and(|count| count != [0, 0])
    {
        Some(Ico)
    } else if starts(b"qoif") {
        Some(Qoi)
    } else if starts(b"#?RADIANCE") || starts(b"#?RGBE") {
        Some(Hdr)
    } else if starts(&[0x76, 0x2f, 0x31, 0x01]) {
        Some(OpenExr)
    } else if starts(&[
        0x00, 0x00, 0x00, 0x0c, b'j', b'P', b' ', b' ', 0x0d, 0x0a, 0x87, 0x0a,
    ]) || starts(&[0xff, 0x4f, 0xff, 0x51])
    {
        Some(Jpeg2000)
    } else if is_pnm(header) {
        Some(Pnm)
    } else {
        None
    }
}

/// Major brand of an ISO base media file (`....ftyp<brand>`).
fn iso_bmff_brand(header: &[u8]) -> Option<&[u8]> {
    (header.get(4..8)? == b"ftyp")
        .then(|| header.get(8..12))
        .flatten()
}

fn bmff_image_format(brand: &[u8]) -> Option<ImageFormat> {
    match brand {
        b"avif" | b"avis" => Some(ImageFormat::Avif),
        b"heic" | b"heix" | b"hevc" | b"hevx" | b"heim" | b"heis" | b"hevm" | b"hevs" | b"mif1"
        | b"msf1" => Some(ImageFormat::Heif),
        b"crx " => Some(ImageFormat::Raw),
        _ => None,
    }
}

fn is_pnm(header: &[u8]) -> bool {
    matches!(header, [b'P', b'1'..=b'7', next, ..] if next.is_ascii_whitespace())
}

/// PDF readers accept up to 1024 bytes of junk before the header.
fn is_pdf(header: &[u8]) -> bool {
    let window = &header[..header.len().min(1024 + 5)];
    window.windows(5).any(|candidate| candidate == b"%PDF-")
}

/// Looks for an `<svg` root element, skipping the XML declaration,
/// doctype, comments and whitespace.
fn is_svg(header: &[u8]) -> bool {
    let mut rest = header.strip_prefix(b"\xef\xbb\xbf").unwrap_or(header);
    loop {
        rest = rest.trim_ascii_start();
        if rest.starts_with(b"<svg") {
            return rest
                .get(4)
                .is_none_or(|next| next.is_ascii_whitespace() || *next == b'>');
        }
        let closing: &[u8] = if rest.starts_with(b"<!--") {
            b"-->"
        } else if rest.starts_with(b"<?") {
            b"?>"
        } else if rest.starts_with(b"<!") {
            b">"
        } else {
            return false;
        };
        match rest
            .windows(closing.len())
            .position(|window| window == closing)
        {
            Some(end) => rest = &rest[end + closing.len()..],
            None => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use FileKind::*;
    use ImageFormat::*;

    #[test]
    fn detects_by_magic_regardless_of_extension() {
        let cases: &[(&[u8], FileKind)] = &[
            (b"%PDF-1.7\n", Pdf),
            (b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR", Image(Png)),
            (&[0xff, 0xd8, 0xff, 0xe0], Image(Jpeg)),
            (b"GIF89a", Image(Gif)),
            (b"RIFF\x10\0\0\0WEBPVP8 ", Image(WebP)),
            (b"\0\0\0\x1cftypavif\0\0\0\0", Image(Avif)),
            (b"\0\0\0\x18ftypheic\0\0\0\0", Image(Heif)),
            (b"\0\0\0\x18ftypcrx \0\0\0\x01", Image(Raw)),
            (b"II*\0\x08\0\0\0", Image(Tiff)),
            (b"MM\0*\0\0\0\x08", Image(Tiff)),
            (b"BM\x36\0\0\0\0\0\0\0\x36\0\0\0", Image(Bmp)),
            (&[0, 0, 1, 0, 1, 0, 16, 16], Image(Ico)),
            (b"qoif\0\0\0\x01", Image(Qoi)),
            (b"#?RADIANCE\n", Image(Hdr)),
            (&[0x76, 0x2f, 0x31, 0x01, 2, 0, 0, 0], Image(OpenExr)),
            (
                &[
                    0, 0, 0, 0x0c, b'j', b'P', b' ', b' ', 0x0d, 0x0a, 0x87, 0x0a,
                ],
                Image(Jpeg2000),
            ),
            (b"P6\n4 4\n255\n", Image(Pnm)),
            (b"FUJIFILMCCD-RAW 0201", Image(Raw)),
            (b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>", Svg),
        ];
        for (header, expected) in cases {
            assert_eq!(detect(header, Some("bin")), Some(*expected), "{header:?}");
        }
    }

    #[test]
    fn pdf_header_after_leading_junk() {
        let mut header = vec![b' '; 500];
        header.extend_from_slice(b"%PDF-1.4");
        assert_eq!(detect(&header, None), Some(Pdf));
        let mut too_far = vec![b' '; 1100];
        too_far.extend_from_slice(b"%PDF-1.4");
        assert_eq!(detect(&too_far, None), None);
    }

    #[test]
    fn tiff_based_raw_needs_raw_extension() {
        assert_eq!(detect(b"II*\0\x08\0\0\0", Some("NEF")), Some(Image(Raw)));
        assert_eq!(detect(b"II*\0\x08\0\0\0", Some("tif")), Some(Image(Tiff)));
    }

    #[test]
    fn svg_with_prolog() {
        let svg = b"\xef\xbb\xbf<?xml version=\"1.0\"?>\n<!-- drawn by hand -->\n\
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"x\">\n<svg width=\"10\">";
        assert_eq!(detect(svg, None), Some(Svg));
        assert_eq!(detect(b"<?xml version=\"1.0\"?><html>", Some("svg")), None);
        assert_eq!(detect(b"<svgfoo>", None), None);
        assert_eq!(detect(&[0x1f, 0x8b, 8, 0], Some("svgz")), Some(Svg));
    }

    #[test]
    fn extension_only_formats() {
        assert_eq!(detect(b"# Title\n", Some("md")), Some(Markdown));
        assert_eq!(detect(b"# Title\n", Some("MARKDOWN")), Some(Markdown));
        assert_eq!(detect(&[0, 0, 2, 0], Some("tga")), Some(Image(Tga)));
        assert_eq!(detect(b"# Title\n", Some("txt")), None);
        assert_eq!(detect(b"", None), None);
    }

    #[test]
    fn desktop_file_lists_every_supported_mime_type() {
        let desktop = include_str!("../../../data/io.github.scrambletools.prev.desktop");
        let listed = desktop
            .lines()
            .find_map(|line| line.strip_prefix("MimeType="))
            .expect("MimeType line");
        let listed: Vec<&str> = listed.split(';').filter(|mime| !mime.is_empty()).collect();
        let supported: Vec<&str> = supported_mime_types().collect();
        assert_eq!(listed, supported);
    }

    #[test]
    fn short_or_truncated_headers_do_not_panic() {
        for len in 0..16 {
            let header = &b"\0\0\0\x1cftypavif\0\0\0\0"[..len];
            let _ = detect(header, Some("avif"));
            let _ = detect(&b"<!-- unterminated"[..len.min(17)], None);
        }
    }
}
