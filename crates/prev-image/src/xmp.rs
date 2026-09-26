//! Finding and embedding XMP packets in JPEG, PNG and WebP files.

use bytes::Bytes;
use img_parts::jpeg::{Jpeg, JpegSegment, markers};
use img_parts::png::{Png, PngChunk};
use img_parts::riff::{RiffChunk, RiffContent};
use img_parts::webp::WebP;

const JPEG_XMP_PREFIX: &[u8] = b"http://ns.adobe.com/xap/1.0/\0";
/// iTXt keyword, then compression flag, method, empty language and
/// translated keyword.
const PNG_XMP_HEADER: &[u8] = b"XML:com.adobe.xmp\0\0\0\0\0";
const WEBP_XMP_CHUNK: [u8; 4] = *b"XMP ";

fn is_jpeg_xmp(segment: &JpegSegment) -> bool {
    segment.marker() == markers::APP1 && segment.contents().starts_with(JPEG_XMP_PREFIX)
}

/// Puts JPEG metadata segments where readers look for them: every APPn
/// before the image data (color profiles after the frame header are
/// ignored by libjpeg), JFIF first, then EXIF, then XMP. The order of
/// everything else is kept. Non-JPEG files are returned unchanged.
pub fn normalize_jpeg(file: Vec<u8>) -> Vec<u8> {
    let Ok(mut jpeg) = Jpeg::from_bytes(Bytes::from(file.clone())) else {
        return file;
    };
    let segments = std::mem::take(jpeg.segments_mut());
    let is_app =
        |segment: &JpegSegment| (markers::APP0..=markers::APP15).contains(&segment.marker());
    let rank = |segment: &JpegSegment| match segment {
        segment if segment.marker() == markers::APP0 => 0,
        segment if is_jpeg_exif(segment) => 1,
        segment if is_jpeg_xmp(segment) => 2,
        _ => 3,
    };
    let (mut apps, rest): (Vec<_>, Vec<_>) = segments.into_iter().partition(is_app);
    // A stable sort keeps the original order within each rank.
    apps.sort_by_key(rank);
    jpeg.segments_mut().extend(apps.into_iter().chain(rest));
    jpeg.encoder().bytes().to_vec()
}

pub(crate) fn is_jpeg_exif(segment: &JpegSegment) -> bool {
    segment.marker() == markers::APP1 && segment.contents().starts_with(b"Exif\0\0")
}

fn is_png_xmp(chunk: &PngChunk) -> bool {
    &chunk.kind() == b"iTXt" && chunk.contents().starts_with(b"XML:com.adobe.xmp\0")
}

/// The text of an uncompressed iTXt chunk: keyword NUL, compression flag,
/// compression method, language NUL, translated keyword NUL, text.
fn png_itxt_text(contents: &[u8]) -> Option<Vec<u8>> {
    let keyword_end = contents.iter().position(|byte| *byte == 0)?;
    let compressed = *contents.get(keyword_end + 1)? != 0;
    if compressed {
        return None;
    }
    let rest = contents.get(keyword_end + 3..)?;
    let language_end = rest.iter().position(|byte| *byte == 0)?;
    let rest = &rest[language_end + 1..];
    let translated_end = rest.iter().position(|byte| *byte == 0)?;
    Some(rest[translated_end + 1..].to_vec())
}

/// The XMP packet of a JPEG, PNG or WebP file, if it has one.
pub fn extract(file: &[u8]) -> Option<Vec<u8>> {
    let bytes = Bytes::copy_from_slice(file);
    if let Ok(jpeg) = Jpeg::from_bytes(bytes.clone()) {
        let segment = jpeg
            .segments()
            .iter()
            .find(|segment| is_jpeg_xmp(segment))?;
        return Some(segment.contents()[JPEG_XMP_PREFIX.len()..].to_vec());
    }
    if let Ok(png) = Png::from_bytes(bytes.clone()) {
        let chunk = png.chunks().iter().find(|chunk| is_png_xmp(chunk))?;
        return png_itxt_text(chunk.contents());
    }
    if let Ok(webp) = WebP::from_bytes(bytes) {
        return match webp.chunk_by_id(WEBP_XMP_CHUNK)?.content() {
            RiffContent::Data(data) => Some(data.to_vec()),
            RiffContent::List { .. } => None,
        };
    }
    None
}

/// Replaces or adds the XMP packet. Other formats are returned unchanged.
pub fn embed(file: Vec<u8>, packet: &[u8]) -> Vec<u8> {
    let bytes = Bytes::from(file);
    if let Ok(mut jpeg) = Jpeg::from_bytes(bytes.clone()) {
        let segments = jpeg.segments_mut();
        segments.retain(|segment| !is_jpeg_xmp(segment));
        let contents = [JPEG_XMP_PREFIX, packet].concat();
        // Right after the JFIF and EXIF segments: EXIF readers expect it to
        // be the first APP1.
        let position = segments
            .iter()
            .rposition(|segment| segment.marker() == markers::APP0 || is_jpeg_exif(segment))
            .map_or(0, |index| index + 1);
        segments.insert(
            position,
            JpegSegment::new_with_contents(markers::APP1, contents.into()),
        );
        return jpeg.encoder().bytes().to_vec();
    }
    if let Ok(mut png) = Png::from_bytes(bytes.clone()) {
        let chunks = png.chunks_mut();
        chunks.retain(|chunk| !is_png_xmp(chunk));
        let contents = [PNG_XMP_HEADER, packet].concat();
        // Just after IHDR, as the XMP specification recommends.
        chunks.insert(
            1.min(chunks.len()),
            PngChunk::new(*b"iTXt", contents.into()),
        );
        return png.encoder().bytes().to_vec();
    }
    if let Ok(mut webp) = WebP::from_bytes(bytes.clone()) {
        webp.remove_chunks_by_id(WEBP_XMP_CHUNK);
        webp.chunks_mut().push(RiffChunk::new(
            WEBP_XMP_CHUNK,
            RiffContent::Data(Bytes::copy_from_slice(packet)),
        ));
        return webp.encoder().bytes().to_vec();
    }
    bytes.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use std::io::Cursor;

    const PACKET: &[u8] = br#"<x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF/></x:xmpmeta>"#;

    fn encoded(format: image::ImageFormat) -> Vec<u8> {
        let image =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(4, 4, image::Rgba([1, 2, 3, 255])));
        let image = if format == image::ImageFormat::Jpeg {
            DynamicImage::ImageRgb8(image.to_rgb8())
        } else {
            image
        };
        let mut bytes = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), format)
            .unwrap();
        bytes
    }

    #[test]
    fn round_trips_in_each_container() {
        for format in [
            image::ImageFormat::Jpeg,
            image::ImageFormat::Png,
            image::ImageFormat::WebP,
        ] {
            let original = encoded(format);
            assert_eq!(extract(&original), None, "{format:?}");
            let with_xmp = embed(original, PACKET);
            assert_eq!(extract(&with_xmp).as_deref(), Some(PACKET), "{format:?}");
            let replaced = embed(with_xmp, b"<x:xmpmeta/>");
            assert_eq!(
                extract(&replaced).as_deref(),
                Some(&b"<x:xmpmeta/>"[..]),
                "{format:?}"
            );
            assert!(
                image::load_from_memory(&replaced).is_ok(),
                "{format:?} still decodes"
            );
        }
    }

    #[test]
    fn other_files_pass_through() {
        assert_eq!(embed(b"not an image".to_vec(), PACKET), b"not an image");
        assert_eq!(extract(b"not an image"), None);
    }
}
