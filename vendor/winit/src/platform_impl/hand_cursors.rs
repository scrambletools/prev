//! The pointing, open and closed hands for `CursorIcon::Pointer`,
//! `CursorIcon::Grab` and `CursorIcon::Grabbing` where the system has no
//! open or closed hand (Windows) or its cursor theme draws them unlike
//! macOS (Linux): Abdulkaiz Khatri's, under the GPL-3.0 (see
//! `hand_cursors/README.md`).

use crate::cursor::CursorImage;
use crate::window::CursorIcon;

/// A hand at the cursor sizes it suits: each with its image's side, the
/// image and its hotspot. The 24 pixel one is drawn smaller on a 32 pixel
/// image.
type Hand = [(u16, u16, &'static [u8], u16, u16); 5];

const POINTER: Hand = [
    (24, 32, include_bytes!("hand_cursors/pointer-24.rgba"), 7, 4),
    (32, 32, include_bytes!("hand_cursors/pointer-32.rgba"), 11, 6),
    (48, 48, include_bytes!("hand_cursors/pointer-48.rgba"), 15, 8),
    (64, 64, include_bytes!("hand_cursors/pointer-64.rgba"), 22, 13),
    (96, 96, include_bytes!("hand_cursors/pointer-96.rgba"), 30, 17),
];
const GRAB: Hand = [
    (24, 32, include_bytes!("hand_cursors/grab-24.rgba"), 11, 6),
    (32, 32, include_bytes!("hand_cursors/grab-32.rgba"), 16, 10),
    (48, 48, include_bytes!("hand_cursors/grab-48.rgba"), 22, 13),
    (64, 64, include_bytes!("hand_cursors/grab-64.rgba"), 33, 20),
    (96, 96, include_bytes!("hand_cursors/grab-96.rgba"), 44, 27),
];
const GRABBING: Hand = [
    (24, 32, include_bytes!("hand_cursors/grabbing-24.rgba"), 11, 7),
    (32, 32, include_bytes!("hand_cursors/grabbing-32.rgba"), 17, 10),
    (48, 48, include_bytes!("hand_cursors/grabbing-48.rgba"), 23, 14),
    (64, 64, include_bytes!("hand_cursors/grabbing-64.rgba"), 34, 21),
    (96, 96, include_bytes!("hand_cursors/grabbing-96.rgba"), 46, 28),
];

/// The hand for `icon` for a cursor size of `size` pixels: the smallest
/// that is at least that, or the largest; `None` for any other cursor.
pub(crate) fn image(icon: CursorIcon, size: u16) -> Option<CursorImage> {
    let sizes = match icon {
        CursorIcon::Pointer => &POINTER,
        CursorIcon::Grab => &GRAB,
        CursorIcon::Grabbing => &GRABBING,
        _ => return None,
    };
    let &(_, side, rgba, x, y) =
        sizes.iter().find(|(fits, ..)| *fits >= size).unwrap_or(&sizes[sizes.len() - 1]);
    CursorImage::from_rgba(rgba.to_vec(), side, side, x, y).ok()
}

/// The cursor size the desktop asks for on Linux, in pixels at a scale of
/// one: `XCURSOR_SIZE`, else 24.
#[cfg(any(x11_platform, wayland_platform))]
pub(crate) fn linux_size() -> f64 {
    std::env::var("XCURSOR_SIZE")
        .ok()
        .and_then(|size| size.trim().parse::<f64>().ok())
        .filter(|size| *size > 0.0)
        .unwrap_or(24.0)
}
