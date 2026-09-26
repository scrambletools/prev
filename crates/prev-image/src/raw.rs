//! Camera RAW through rawler: decode the sensor data, then develop it with
//! demosaicing, white balance, camera calibration and sRGB conversion.

use image::metadata::Orientation;
use rawler::decoders::RawDecodeParams;
use rawler::imgop::develop::RawDevelop;
use rawler::rawsource::RawSource;

use crate::decode::{DecodeError, Decoded, to_frame};

pub fn decode(bytes: &[u8]) -> Result<Decoded, DecodeError> {
    let invalid = |error: rawler::RawlerError| DecodeError::Invalid(error.to_string());
    let source = RawSource::new_from_slice(bytes);
    let raw = rawler::decode(&source, &RawDecodeParams { image_index: 0 }).map_err(invalid)?;
    let orientation = orientation(raw.orientation);
    let mut image = RawDevelop::default()
        .develop_intermediate(&raw)
        .map_err(invalid)?
        .to_dynamic_image()
        .ok_or_else(|| DecodeError::Invalid("developed image has an unexpected size".into()))?;
    image.apply_orientation(orientation);
    Ok(Decoded {
        frames: vec![to_frame(image)],
    })
}

fn orientation(orientation: rawler::Orientation) -> Orientation {
    use rawler::Orientation as Raw;
    match orientation {
        Raw::Normal | Raw::Unknown => Orientation::NoTransforms,
        Raw::HorizontalFlip => Orientation::FlipHorizontal,
        Raw::Rotate180 => Orientation::Rotate180,
        Raw::VerticalFlip => Orientation::FlipVertical,
        Raw::Transpose => Orientation::Rotate90FlipH,
        Raw::Rotate90 => Orientation::Rotate90,
        Raw::Transverse => Orientation::Rotate270FlipH,
        Raw::Rotate270 => Orientation::Rotate270,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garbage_is_an_error() {
        assert!(matches!(
            decode(b"II*\0 not really a raw file"),
            Err(DecodeError::Invalid(_))
        ));
    }
}
