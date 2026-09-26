//! Reading metadata for the inspector, and the edits Preview offers: remove
//! location, and set keywords and a description (stored in XMP).

use std::io::Cursor;

use bytes::Bytes;
use img_parts::{DynImage, ImageEXIF};

const DC: &str = "http://purl.org/dc/elements/1.1/";
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const EXIF_NS: &str = "http://ns.adobe.com/exif/1.0/";

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Details {
    /// (section, label, value) in display order.
    pub fields: Vec<(String, String, String)>,
    /// Latitude and longitude in degrees.
    pub location: Option<(f64, f64)>,
    pub keywords: Vec<String>,
    pub description: Option<String>,
}

/// EXIF fields shown in the inspector, grouped like Preview's.
const SHOWN: &[(&str, exif::Tag)] = &[
    ("Camera", exif::Tag::Make),
    ("Camera", exif::Tag::Model),
    ("Camera", exif::Tag::LensModel),
    ("Exposure", exif::Tag::ExposureTime),
    ("Exposure", exif::Tag::FNumber),
    ("Exposure", exif::Tag::PhotographicSensitivity),
    ("Exposure", exif::Tag::FocalLength),
    ("Exposure", exif::Tag::ExposureBiasValue),
    ("Exposure", exif::Tag::Flash),
    ("Image", exif::Tag::DateTimeOriginal),
    ("Image", exif::Tag::Orientation),
    ("Image", exif::Tag::ColorSpace),
    ("Image", exif::Tag::Software),
    ("Image", exif::Tag::Artist),
    ("Image", exif::Tag::Copyright),
];

pub fn read(file: &[u8]) -> Details {
    let mut details = Details::default();
    if let Ok(exif) = exif::Reader::new().read_from_container(&mut Cursor::new(file)) {
        for (section, tag) in SHOWN {
            if let Some(field) = exif.get_field(*tag, exif::In::PRIMARY) {
                let value = field.display_value().with_unit(&exif).to_string();
                let value = value.trim_matches('"').trim().to_owned();
                if !value.is_empty() {
                    details
                        .fields
                        .push(((*section).to_owned(), tag.to_string(), value));
                }
            }
        }
        details.location = gps_location(&exif);
        if let Some(field) = exif.get_field(exif::Tag::ImageDescription, exif::In::PRIMARY) {
            let text = field
                .display_value()
                .to_string()
                .trim_matches('"')
                .trim()
                .to_owned();
            if !text.is_empty() {
                details.description = Some(text);
            }
        }
    }
    if let Some(packet) = crate::xmp::extract(file)
        && let Ok(text) = std::str::from_utf8(&packet)
        && let Ok(document) = roxmltree::Document::parse(text)
    {
        details.keywords = xmp_list(&document, "subject");
        if let Some(description) = xmp_list(&document, "description").into_iter().next() {
            details.description = Some(description);
        }
        if details.location.is_none() {
            details.location = xmp_location(&document);
        }
    }
    details
}

fn gps_location(exif: &exif::Exif) -> Option<(f64, f64)> {
    let degrees = |tag, reference, negative: &str| {
        let value = match &exif.get_field(tag, exif::In::PRIMARY)?.value {
            exif::Value::Rational(parts) if parts.len() == 3 => {
                parts[0].to_f64() + parts[1].to_f64() / 60.0 + parts[2].to_f64() / 3600.0
            }
            _ => return None,
        };
        let sign = match exif.get_field(reference, exif::In::PRIMARY) {
            Some(field) if field.display_value().to_string().contains(negative) => -1.0,
            _ => 1.0,
        };
        Some(value * sign)
    };
    Some((
        degrees(exif::Tag::GPSLatitude, exif::Tag::GPSLatitudeRef, "S")?,
        degrees(exif::Tag::GPSLongitude, exif::Tag::GPSLongitudeRef, "W")?,
    ))
}

/// Values of a Dublin Core property: `rdf:li` items, or plain text.
fn xmp_list(document: &roxmltree::Document, name: &str) -> Vec<String> {
    let Some(node) = document
        .descendants()
        .find(|node| node.has_tag_name((DC, name)))
    else {
        return Vec::new();
    };
    let items: Vec<String> = node
        .descendants()
        .filter(|node| node.has_tag_name((RDF, "li")))
        .filter_map(|node| node.text())
        .map(|text| text.trim().to_owned())
        .filter(|text| !text.is_empty())
        .collect();
    if items.is_empty() {
        node.text()
            .map(|text| text.trim().to_owned())
            .filter(|text| !text.is_empty())
            .into_iter()
            .collect()
    } else {
        items
    }
}

/// XMP GPS values look like "52,30.0N".
fn xmp_location(document: &roxmltree::Document) -> Option<(f64, f64)> {
    let value = |name| {
        let text = document.descendants().find_map(|node| {
            node.attribute((EXIF_NS, name))
                .map(str::to_owned)
                .or_else(|| {
                    node.has_tag_name((EXIF_NS, name))
                        .then(|| node.text().unwrap_or_default().to_owned())
                })
        })?;
        let text = text.trim();
        let (number, direction) = text.split_at(text.len().checked_sub(1)?);
        let (whole, minutes) = number.split_once(',')?;
        let degrees = whole.parse::<f64>().ok()? + minutes.parse::<f64>().ok()? / 60.0;
        Some(if matches!(direction, "S" | "W") {
            -degrees
        } else {
            degrees
        })
    };
    Some((value("GPSLatitude")?, value("GPSLongitude")?))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataError(pub String);

impl std::fmt::Display for MetadataError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for MetadataError {}

/// Whether keywords and descriptions can be written to this file.
pub fn supports_xmp(file: &[u8]) -> bool {
    crate::xmp::embed(file.to_vec(), b"<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"/>") != file
}

/// Removes GPS data from EXIF (overwriting it) and from XMP.
pub fn remove_location(file: Vec<u8>) -> Result<Vec<u8>, MetadataError> {
    let failed = |error: crate::exif_edit::ExifError| {
        MetadataError(format!("could not remove the location: {error}"))
    };
    let mut out = file;
    if let Ok(Some(mut image)) = DynImage::from_bytes(Bytes::from(out.clone())) {
        if let Some(exif) = image.exif() {
            let mut exif = exif.to_vec();
            if crate::exif_edit::remove_gps(&mut exif).map_err(failed)? {
                image.set_exif(Some(Bytes::from(exif)));
                out = image.encoder().bytes().to_vec();
            }
        }
    } else if out.starts_with(b"II*\0") || out.starts_with(b"MM\0*") {
        // A TIFF file is itself the structure EXIF uses.
        crate::exif_edit::remove_gps(&mut out).map_err(failed)?;
    } else {
        return Err(MetadataError(
            "location info can be removed from JPEG, PNG, WebP and TIFF files".into(),
        ));
    }
    if let Some(packet) = crate::xmp::extract(&out) {
        let text = String::from_utf8_lossy(&packet).into_owned();
        let cleaned = remove_xmp_properties(&text, EXIF_NS, |name| name.starts_with("GPS"));
        if cleaned != text {
            out = crate::xmp::embed(out, cleaned.as_bytes());
        }
    }
    Ok(crate::xmp::normalize_jpeg(out))
}

/// Deletes properties of namespace `namespace` whose local name matches,
/// whether written as elements or as attributes, leaving the rest intact.
fn remove_xmp_properties(text: &str, namespace: &str, matches: impl Fn(&str) -> bool) -> String {
    let Ok(document) = roxmltree::Document::parse(text) else {
        return text.to_owned();
    };
    let mut cuts: Vec<std::ops::Range<usize>> = Vec::new();
    for node in document.descendants() {
        if node.is_element()
            && node.tag_name().namespace() == Some(namespace)
            && matches(node.tag_name().name())
        {
            cuts.push(node.range());
        }
        for attribute in node.attributes() {
            if attribute.namespace() == Some(namespace) && matches(attribute.name()) {
                cuts.push(attribute.range());
            }
        }
    }
    splice(
        text,
        cuts.into_iter()
            .map(|range| (range, String::new()))
            .collect(),
    )
}

/// Applies non-overlapping replacements, last first so offsets stay valid.
fn splice(text: &str, mut edits: Vec<(std::ops::Range<usize>, String)>) -> String {
    edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
    let mut out = text.to_owned();
    let mut previous_start = usize::MAX;
    for (range, replacement) in edits {
        if range.end <= previous_start {
            out.replace_range(range.clone(), &replacement);
            previous_start = range.start;
        }
    }
    out
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn dc_elements(description: Option<&str>, keywords: &[String]) -> String {
    let mut elements = String::new();
    if let Some(description) = description.filter(|text| !text.trim().is_empty()) {
        elements.push_str(&format!(
            "<dc:description><rdf:Alt><rdf:li xml:lang=\"x-default\">{}</rdf:li></rdf:Alt></dc:description>",
            escape(description.trim())
        ));
    }
    let keywords: Vec<&String> = keywords
        .iter()
        .filter(|keyword| !keyword.trim().is_empty())
        .collect();
    if !keywords.is_empty() {
        elements.push_str("<dc:subject><rdf:Bag>");
        for keyword in keywords {
            elements.push_str(&format!("<rdf:li>{}</rdf:li>", escape(keyword.trim())));
        }
        elements.push_str("</rdf:Bag></dc:subject>");
    }
    elements
}

/// Sets the description and keywords in the file's XMP, keeping any other
/// XMP properties. Supports JPEG, PNG and WebP.
pub fn set_description_and_keywords(
    file: Vec<u8>,
    description: Option<&str>,
    keywords: &[String],
) -> Result<Vec<u8>, MetadataError> {
    if !supports_xmp(&file) {
        return Err(MetadataError(
            "keywords and descriptions can only be saved in JPEG, PNG and WebP files".into(),
        ));
    }
    let elements = dc_elements(description, keywords);
    let packet = match crate::xmp::extract(&file).and_then(|packet| String::from_utf8(packet).ok())
    {
        Some(existing) => {
            update_packet(&existing, &elements).unwrap_or_else(|| new_packet(&elements))
        }
        None => new_packet(&elements),
    };
    Ok(crate::xmp::normalize_jpeg(crate::xmp::embed(
        file,
        packet.as_bytes(),
    )))
}

fn new_packet(elements: &str) -> String {
    format!(
        "<?xpacket begin=\"\u{feff}\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\
<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"{RDF}\">\
<rdf:Description rdf:about=\"\" xmlns:dc=\"{DC}\">{elements}</rdf:Description>\
</rdf:RDF></x:xmpmeta><?xpacket end=\"w\"?>"
    )
}

/// Replaces dc:description and dc:subject in an existing packet, adding
/// them to its first rdf:Description. `None` if the packet is unusable.
fn update_packet(existing: &str, elements: &str) -> Option<String> {
    let document = roxmltree::Document::parse(existing).ok()?;
    let target = document
        .descendants()
        .find(|node| node.has_tag_name((RDF, "Description")))?;
    let mut edits: Vec<(std::ops::Range<usize>, String)> = Vec::new();
    for node in document.descendants() {
        if node.has_tag_name((DC, "description")) || node.has_tag_name((DC, "subject")) {
            edits.push((node.range(), String::new()));
        }
        for attribute in node.attributes() {
            if attribute.namespace() == Some(DC)
                && matches!(attribute.name(), "description" | "subject")
            {
                edits.push((attribute.range(), String::new()));
            }
        }
    }
    // Insert right after the start tag of the target description.
    let start = target.range().start;
    let tag_end = start + existing[start..].find('>')?;
    let self_closing = existing[..tag_end].ends_with('/');
    let dc_declared = target.lookup_prefix(DC) == Some("dc");
    let declaration = if dc_declared {
        String::new()
    } else {
        format!(" xmlns:dc=\"{DC}\"")
    };
    if self_closing {
        let name = &existing[start + 1..]
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()?
            .to_owned();
        edits.push((
            tag_end - 1..tag_end + 1,
            format!("{declaration}>{elements}</{name}>"),
        ));
    } else {
        edits.push((tag_end..tag_end + 1, format!("{declaration}>{elements}")));
    }
    let updated = splice(existing, edits);
    roxmltree::Document::parse(&updated).ok()?;
    Some(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::Frame;
    use crate::encode::{SaveFormat, encode};
    use std::time::Duration;

    fn jpeg() -> Vec<u8> {
        let frame = Frame {
            width: 4,
            height: 4,
            pixels: vec![128; 64],
            delay: Duration::ZERO,
        };
        encode(&frame, SaveFormat::Jpeg { quality: 90 }).unwrap()
    }

    fn with_gps(file: Vec<u8>) -> Vec<u8> {
        let mut image = DynImage::from_bytes(Bytes::from(file)).unwrap().unwrap();
        image.set_exif(Some(Bytes::from(crate::exif_edit::fixture::exif(1))));
        image.encoder().bytes().to_vec()
    }

    #[test]
    fn reads_exif_fields_and_location() {
        let details = read(&with_gps(jpeg()));
        assert!(
            details
                .fields
                .iter()
                .any(|(_, label, value)| label == "Make" && value == "prevcam"),
            "{details:?}"
        );
        let (latitude, longitude) = details.location.unwrap();
        assert!((latitude - 52.5).abs() < 1e-6 && (longitude + 13.4).abs() < 1e-6);
    }

    #[test]
    fn removes_location_from_exif_and_xmp() {
        let packet = format!(
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"{RDF}\"><rdf:Description \
             xmlns:exif=\"{EXIF_NS}\" exif:GPSLatitude=\"52,30.0N\" exif:ExposureTime=\"1/200\">\
             <exif:GPSLongitude>13,24.0W</exif:GPSLongitude></rdf:Description></rdf:RDF></x:xmpmeta>"
        );
        let file = crate::xmp::embed(with_gps(jpeg()), packet.as_bytes());
        assert!(read(&file).location.is_some());
        let cleaned = remove_location(file).unwrap();
        let details = read(&cleaned);
        assert_eq!(details.location, None);
        assert!(
            details.fields.iter().any(|(_, label, _)| label == "Make"),
            "other EXIF kept"
        );
        let xmp = String::from_utf8(crate::xmp::extract(&cleaned).unwrap()).unwrap();
        assert!(
            !xmp.contains("GPS") && xmp.contains("ExposureTime"),
            "{xmp}"
        );
        assert!(crate::decode::decode(&cleaned, crate::ImageFormat::Jpeg).is_ok());
    }

    #[test]
    fn writes_keywords_and_description() {
        let keywords = vec!["holiday".to_owned(), "a < b & c".to_owned()];
        let file = set_description_and_keywords(jpeg(), Some("Beach at dusk"), &keywords).unwrap();
        let details = read(&file);
        assert_eq!(details.keywords, keywords);
        assert_eq!(details.description.as_deref(), Some("Beach at dusk"));

        let replaced = set_description_and_keywords(file, None, &["one".to_owned()]).unwrap();
        let details = read(&replaced);
        assert_eq!(details.keywords, vec!["one"]);
        assert_eq!(details.description, None);
    }

    #[test]
    fn keeps_other_xmp_properties() {
        let packet = format!(
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"{RDF}\"><rdf:Description \
             xmlns:crs=\"http://ns.adobe.com/camera-raw-settings/1.0/\" crs:Exposure2012=\"+0.50\"/>\
             </rdf:RDF></x:xmpmeta>"
        );
        let file = crate::xmp::embed(jpeg(), packet.as_bytes());
        let updated = set_description_and_keywords(file, Some("kept"), &[]).unwrap();
        let xmp = String::from_utf8(crate::xmp::extract(&updated).unwrap()).unwrap();
        assert!(xmp.contains("crs:Exposure2012=\"+0.50\""), "{xmp}");
        assert_eq!(read(&updated).description.as_deref(), Some("kept"));
    }

    #[test]
    fn unsupported_containers_are_refused() {
        let frame = Frame {
            width: 2,
            height: 2,
            pixels: vec![9; 16],
            delay: Duration::ZERO,
        };
        let tiff = encode(&frame, SaveFormat::Tiff).unwrap();
        assert!(!supports_xmp(&tiff));
        assert!(set_description_and_keywords(tiff, Some("x"), &[]).is_err());
    }
}
