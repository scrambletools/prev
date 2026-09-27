//! Metadata in TIFF files: carrying EXIF, GPS, the color profile and XMP
//! from an original TIFF into one written anew after an edit, and reading
//! XMP. img-parts, which does this for JPEG, PNG and WebP, has no TIFF.

use exif::experimental::Writer;
use exif::{Context, Field, In, Reader, Tag, Value};

/// XMLPacket, where TIFF keeps XMP.
const XMP: u16 = 700;

pub fn is_tiff(file: &[u8]) -> bool {
    file.starts_with(b"II*\0") || file.starts_with(b"MM\0*")
}

/// Tags that describe how a TIFF's pixels are laid out, which come from
/// the newly written image rather than the original.
fn is_layout(tag: Tag) -> bool {
    tag.context() == Context::Tiff
        && matches!(
            tag.number(),
            254 // NewSubfileType
                | 255 // SubfileType
                | 256 // ImageWidth
                | 257 // ImageLength
                | 258 // BitsPerSample
                | 259 // Compression
                | 262 // PhotometricInterpretation
                | 266 // FillOrder
                | 273 // StripOffsets
                | 274 // Orientation: edits apply to the upright image
                | 277 // SamplesPerPixel
                | 278 // RowsPerStrip
                | 279 // StripByteCounts
                | 284 // PlanarConfiguration
                | 317 // Predictor
                | 322..=325 // Tile layout
                | 330 // SubIFDs
                | 338 // ExtraSamples
                | 339 // SampleFormat
                | 347 // JPEGTables
                | 513 | 514 // JPEG thumbnail
                | 529..=532 // YCbCr layout and reference levels
        )
}

fn uint(field: &Field) -> Vec<u32> {
    (0..)
        .map_while(|index| field.value.get_uint(index))
        .collect()
}

/// The strips of the main image of `tiff`, as byte slices.
fn strips<'a>(tiff: &'a [u8], exif: &exif::Exif) -> Option<Vec<&'a [u8]>> {
    let offsets = uint(exif.get_field(Tag::StripOffsets, In::PRIMARY)?);
    let counts = uint(exif.get_field(Tag::StripByteCounts, In::PRIMARY)?);
    if offsets.len() != counts.len() || offsets.is_empty() {
        return None;
    }
    offsets
        .iter()
        .zip(&counts)
        .map(|(offset, count)| {
            let start = *offset as usize;
            tiff.get(start..start.checked_add(*count as usize)?)
        })
        .collect()
}

/// `encoded`, a TIFF written from edited pixels, with the metadata of
/// `original` added: EXIF, GPS, the color profile, XMP and descriptive
/// TIFF tags, and the orientation reset. `None` when either cannot be
/// read, in which case the caller keeps `encoded` as it is.
pub fn carry(original: &[u8], encoded: &[u8]) -> Option<Vec<u8>> {
    let reader = Reader::new();
    let source = reader.read_raw(original.to_vec()).ok()?;
    let target = reader.read_raw(encoded.to_vec()).ok()?;
    let strips = strips(encoded, &target)?;
    let mut writer = Writer::new();
    for field in target.fields().filter(|field| field.ifd_num == In::PRIMARY) {
        if is_layout(field.tag) && field.tag != Tag::Orientation {
            writer.push_field(field);
        }
    }
    let upright = Field {
        tag: Tag::Orientation,
        ifd_num: In::PRIMARY,
        value: Value::Short(vec![1]),
    };
    writer.push_field(&upright);
    for field in source.fields().filter(|field| field.ifd_num == In::PRIMARY) {
        if !is_layout(field.tag) {
            writer.push_field(field);
        }
    }
    writer.set_strips(&strips, In::PRIMARY);
    let mut out = std::io::Cursor::new(Vec::new());
    writer.write(&mut out, source.little_endian()).ok()?;
    Some(out.into_inner())
}

/// The XMP packet of a TIFF file, from its first image's tags.
pub fn xmp(file: &[u8]) -> Option<Vec<u8>> {
    if !is_tiff(file) {
        return None;
    }
    let exif = Reader::new().read_raw(file.to_vec()).ok()?;
    let field = exif.get_field(Tag(Context::Tiff, XMP), In::PRIMARY)?;
    match &field.value {
        Value::Byte(bytes) | Value::Undefined(bytes, _) => Some(bytes.clone()),
        Value::Ascii(parts) => Some(parts.concat()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small TIFF with a make, an EXIF exposure time, GPS, an ICC profile
    /// stand-in, XMP and a sideways orientation.
    fn original() -> Vec<u8> {
        let pixels = vec![200u8; 4 * 3 * 3];
        let fields = [
            Field {
                tag: Tag::ImageWidth,
                ifd_num: In::PRIMARY,
                value: Value::Long(vec![4]),
            },
            Field {
                tag: Tag::ImageLength,
                ifd_num: In::PRIMARY,
                value: Value::Long(vec![3]),
            },
            Field {
                tag: Tag::BitsPerSample,
                ifd_num: In::PRIMARY,
                value: Value::Short(vec![8, 8, 8]),
            },
            Field {
                tag: Tag::Compression,
                ifd_num: In::PRIMARY,
                value: Value::Short(vec![1]),
            },
            Field {
                tag: Tag::PhotometricInterpretation,
                ifd_num: In::PRIMARY,
                value: Value::Short(vec![2]),
            },
            Field {
                tag: Tag::SamplesPerPixel,
                ifd_num: In::PRIMARY,
                value: Value::Short(vec![3]),
            },
            Field {
                tag: Tag::RowsPerStrip,
                ifd_num: In::PRIMARY,
                value: Value::Long(vec![3]),
            },
            Field {
                tag: Tag::Orientation,
                ifd_num: In::PRIMARY,
                value: Value::Short(vec![6]),
            },
            Field {
                tag: Tag::Make,
                ifd_num: In::PRIMARY,
                value: Value::Ascii(vec![b"Prevcam".to_vec()]),
            },
            Field {
                tag: Tag::ExposureTime,
                ifd_num: In::PRIMARY,
                value: Value::Rational(vec![(1, 250).into()]),
            },
            Field {
                tag: Tag::GPSLatitudeRef,
                ifd_num: In::PRIMARY,
                value: Value::Ascii(vec![b"N".to_vec()]),
            },
            Field {
                tag: Tag(Context::Tiff, 34675),
                ifd_num: In::PRIMARY,
                value: Value::Undefined(b"icc-profile".to_vec(), 0),
            },
            Field {
                tag: Tag(Context::Tiff, XMP),
                ifd_num: In::PRIMARY,
                value: Value::Byte(b"<x:xmpmeta/>".to_vec()),
            },
        ];
        let strips: [&[u8]; 1] = [&pixels];
        let mut writer = Writer::new();
        for field in &fields {
            writer.push_field(field);
        }
        writer.set_strips(&strips, In::PRIMARY);
        let mut out = std::io::Cursor::new(Vec::new());
        writer.write(&mut out, true).unwrap();
        out.into_inner()
    }

    fn edited() -> Vec<u8> {
        // As the editor writes it: through the image crate, without metadata.
        let image = image::RgbaImage::from_pixel(3, 4, image::Rgba([10, 20, 30, 255]));
        let mut out = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut out, image::ImageFormat::Tiff)
            .unwrap();
        out.into_inner()
    }

    #[test]
    fn edited_tiffs_keep_the_metadata() {
        let carried = carry(&original(), &edited()).unwrap();
        let exif = Reader::new().read_raw(carried.clone()).unwrap();
        let text = |tag| {
            exif.get_field(tag, In::PRIMARY)
                .map(|field| field.display_value().to_string())
        };
        assert_eq!(text(Tag::Make).as_deref(), Some("\"Prevcam\""));
        assert!(exif.get_field(Tag::ExposureTime, In::PRIMARY).is_some());
        assert!(exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY).is_some());
        assert!(
            exif.get_field(Tag(Context::Tiff, 34675), In::PRIMARY)
                .is_some()
        );
        assert_eq!(xmp(&carried).as_deref(), Some(&b"<x:xmpmeta/>"[..]));
        // The new pixels and layout, upright.
        assert_eq!(
            uint(exif.get_field(Tag::ImageWidth, In::PRIMARY).unwrap()),
            [3]
        );
        assert_eq!(
            uint(exif.get_field(Tag::Orientation, In::PRIMARY).unwrap()),
            [1]
        );
        let image = image::load_from_memory(&carried).unwrap().into_rgba8();
        assert_eq!(image.dimensions(), (3, 4));
        assert_eq!(image.get_pixel(1, 1).0, [10, 20, 30, 255]);
    }

    #[test]
    fn xmp_is_read_from_tiffs_only() {
        assert_eq!(xmp(&original()).as_deref(), Some(&b"<x:xmpmeta/>"[..]));
        assert!(xmp(&edited()).is_none());
        assert!(xmp(b"\xff\xd8 not a tiff").is_none());
    }
}
