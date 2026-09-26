//! The two EXIF edits prev makes, done in place on the TIFF structure of
//! an EXIF block (or a whole TIFF file): resetting the orientation, and
//! removing GPS data, including overwriting it.

const ORIENTATION: u16 = 0x0112;
const GPS_POINTER: u16 = 0x8825;
const ENTRY: usize = 12;

struct Tiff<'a> {
    data: &'a mut [u8],
    big_endian: bool,
}

impl<'a> Tiff<'a> {
    fn new(data: &'a mut [u8]) -> Option<Self> {
        let big_endian = match data.get(..4)? {
            [b'I', b'I', 42, 0] => false,
            [b'M', b'M', 0, 42] => true,
            _ => return None,
        };
        Some(Self { data, big_endian })
    }

    fn u16(&self, offset: usize) -> Option<u16> {
        let bytes: [u8; 2] = self.data.get(offset..offset + 2)?.try_into().ok()?;
        Some(if self.big_endian {
            u16::from_be_bytes(bytes)
        } else {
            u16::from_le_bytes(bytes)
        })
    }

    fn u32(&self, offset: usize) -> Option<u32> {
        let bytes: [u8; 4] = self.data.get(offset..offset + 4)?.try_into().ok()?;
        Some(if self.big_endian {
            u32::from_be_bytes(bytes)
        } else {
            u32::from_le_bytes(bytes)
        })
    }

    fn put_u16(&mut self, offset: usize, value: u16) -> Option<()> {
        let bytes = if self.big_endian {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.data
            .get_mut(offset..offset + 2)?
            .copy_from_slice(&bytes);
        Some(())
    }

    fn zero(&mut self, start: usize, length: usize) -> Option<()> {
        self.data
            .get_mut(start..start.checked_add(length)?)?
            .fill(0);
        Some(())
    }

    fn first_ifd(&self) -> Option<usize> {
        self.u32(4).map(|offset| offset as usize)
    }

    /// Offsets of the entries of the directory at `ifd`, checked to fit.
    fn entries(&self, ifd: usize) -> Option<(usize, Vec<usize>)> {
        let count = usize::from(self.u16(ifd)?);
        let first = ifd + 2;
        let end = first.checked_add(count * ENTRY)?.checked_add(4)?;
        (end <= self.data.len()).then(|| {
            (
                count,
                (0..count).map(|index| first + index * ENTRY).collect(),
            )
        })
    }

    fn find(&self, ifd: usize, tag: u16) -> Option<usize> {
        self.entries(ifd)?
            .1
            .into_iter()
            .find(|entry| self.u16(*entry) == Some(tag))
    }

    /// Bytes of an entry's value when it does not fit in the entry itself.
    fn external_value(&self, entry: usize) -> Option<(usize, usize)> {
        let unit = match self.u16(entry + 2)? {
            1 | 2 | 6 | 7 => 1,
            3 | 8 => 2,
            4 | 9 | 11 => 4,
            5 | 10 | 12 => 8,
            _ => return None,
        };
        let length = unit * self.u32(entry + 4)? as usize;
        (length > 4)
            .then(|| (self.u32(entry + 8).map(|offset| offset as usize), length))
            .and_then(|(offset, length)| {
                let offset = offset?;
                (offset.checked_add(length)? <= self.data.len()).then_some((offset, length))
            })
    }
}

/// Sets the orientation to 1 (upright). Returns whether it changed.
pub fn reset_orientation(tiff: &mut [u8]) -> bool {
    let Some(mut tiff) = Tiff::new(tiff) else {
        return false;
    };
    let Some(ifd) = tiff.first_ifd() else {
        return false;
    };
    let Some(entry) = tiff.find(ifd, ORIENTATION) else {
        return false;
    };
    if tiff.u16(entry + 2) != Some(3) || tiff.u16(entry + 8) == Some(1) {
        return false;
    }
    tiff.put_u16(entry + 8, 1).is_some()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExifError(pub &'static str);

impl std::fmt::Display for ExifError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for ExifError {}

/// Removes the GPS directory: its entry in the first directory, and its
/// data overwritten with zeros. Returns whether there was any.
pub fn remove_gps(tiff: &mut [u8]) -> Result<bool, ExifError> {
    let malformed = ExifError("the EXIF data is malformed");
    let Some(mut tiff) = Tiff::new(tiff) else {
        return Err(malformed);
    };
    let ifd = tiff.first_ifd().ok_or(malformed.clone())?;
    let (count, entries) = tiff.entries(ifd).ok_or(malformed.clone())?;
    let Some(position) = entries
        .iter()
        .position(|entry| tiff.u16(*entry) == Some(GPS_POINTER))
    else {
        return Ok(false);
    };
    let gps = tiff.u32(entries[position] + 8).ok_or(malformed.clone())? as usize;

    // Overwrite the GPS values, then the GPS directory itself.
    if let Some((gps_count, gps_entries)) = tiff.entries(gps) {
        for entry in gps_entries {
            if let Some((offset, length)) = tiff.external_value(entry) {
                tiff.zero(offset, length).ok_or(malformed.clone())?;
            }
        }
        tiff.zero(gps, 2 + gps_count * ENTRY + 4)
            .ok_or(malformed.clone())?;
    }

    // Drop the pointer entry: shift the later entries and the next
    // directory offset up by one entry, and clear the freed tail.
    let start = entries[position];
    let tail_end = ifd + 2 + count * ENTRY + 4;
    tiff.data.copy_within(start + ENTRY..tail_end, start);
    tiff.zero(tail_end - ENTRY, ENTRY)
        .ok_or(malformed.clone())?;
    tiff.put_u16(ifd, (count - 1) as u16).ok_or(malformed)?;
    Ok(true)
}

/// Test fixtures: a little-endian EXIF block with a make, an orientation
/// and a GPS directory.
#[cfg(test)]
pub(crate) mod fixture {
    fn entry(out: &mut Vec<u8>, tag: u16, kind: u16, count: u32, value: u32) {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }

    pub const LATITUDE: [u32; 6] = [52, 1, 30, 1, 0, 1];
    pub const LONGITUDE: [u32; 6] = [13, 1, 24, 1, 0, 1];

    /// Make "prevcam", the given orientation, and GPS 52°30' N, 13°24' W.
    pub fn exif(orientation: u16) -> Vec<u8> {
        let mut out = b"II*\0".to_vec();
        out.extend_from_slice(&8u32.to_le_bytes());
        // IFD0 at 8: make, orientation, GPS pointer.
        let ifd0_size = 2 + 3 * 12 + 4;
        let make_offset = 8 + ifd0_size;
        let make = b"prevcam\0";
        let gps_offset = make_offset + make.len();
        out.extend_from_slice(&3u16.to_le_bytes());
        entry(&mut out, 0x010F, 2, make.len() as u32, make_offset as u32);
        entry(&mut out, 0x0112, 3, 1, u32::from(orientation));
        entry(&mut out, 0x8825, 4, 1, gps_offset as u32);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(make);
        // GPS IFD: references and three rationals each for latitude and longitude.
        let values_offset = gps_offset + 2 + 4 * 12 + 4;
        out.extend_from_slice(&4u16.to_le_bytes());
        entry(&mut out, 0x0001, 2, 2, u32::from_le_bytes(*b"N\0\0\0"));
        entry(&mut out, 0x0002, 5, 3, values_offset as u32);
        entry(&mut out, 0x0003, 2, 2, u32::from_le_bytes(*b"W\0\0\0"));
        entry(&mut out, 0x0004, 5, 3, (values_offset + 24) as u32);
        out.extend_from_slice(&0u32.to_le_bytes());
        for value in LATITUDE.iter().chain(&LONGITUDE) {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(tiff: &[u8]) -> exif::Exif {
        exif::Reader::new().read_raw(tiff.to_vec()).unwrap()
    }

    #[test]
    fn resets_orientation() {
        let mut data = fixture::exif(6);
        assert!(reset_orientation(&mut data));
        let exif = read(&data);
        assert_eq!(
            exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                .unwrap()
                .value
                .get_uint(0),
            Some(1)
        );
        assert!(!reset_orientation(&mut data), "already upright");
        assert!(!reset_orientation(&mut b"not tiff".to_vec()));
    }

    #[test]
    fn removes_gps_and_overwrites_it() {
        let mut data = fixture::exif(1);
        assert!(
            read(&data)
                .get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY)
                .is_some()
        );
        assert_eq!(remove_gps(&mut data), Ok(true));
        let exif = read(&data);
        assert!(
            exif.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY)
                .is_none()
        );
        assert!(
            exif.get_field(exif::Tag::GPSInfoIFDPointer, exif::In::PRIMARY)
                .is_none()
        );
        assert!(
            exif.get_field(exif::Tag::Make, exif::In::PRIMARY).is_some(),
            "other tags survive"
        );
        let latitude: Vec<u8> = fixture::LATITUDE
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect();
        assert!(
            !data
                .windows(latitude.len())
                .any(|window| window == latitude),
            "GPS values are overwritten"
        );
        assert_eq!(remove_gps(&mut data), Ok(false), "nothing left to remove");
    }

    #[test]
    fn malformed_data_is_an_error_not_a_panic() {
        assert!(remove_gps(&mut b"not tiff".to_vec()).is_err());
        let mut truncated = fixture::exif(1);
        truncated.truncate(20);
        assert!(remove_gps(&mut truncated).is_err());
        for length in 0..fixture::exif(1).len() {
            let mut cut = fixture::exif(6);
            cut.truncate(length);
            let _ = remove_gps(&mut cut);
            let _ = reset_orientation(&mut cut);
        }
    }
}
