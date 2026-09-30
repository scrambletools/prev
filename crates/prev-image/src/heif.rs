//! HEIC and AVIF through the system libheif, loaded at run time so prev
//! neither links nor ships it. HEVC is patent encumbered, so prev only ever
//! decodes through whatever the system provides.

#![allow(unsafe_code)]

use std::ffi::{CStr, c_char, c_int, c_void};
use std::sync::OnceLock;
use std::time::Duration;

use libloading::Library;

use crate::decode::{DecodeError, Decoded, Frame, Result};

#[cfg(not(any(windows, target_os = "macos")))]
const LIBRARY_NAMES: &[&str] = &["libheif.so.1", "libheif.so"];
/// The copy in prev.app's Frameworks folder, else Homebrew's (Apple
/// Silicon, then Intel).
#[cfg(target_os = "macos")]
const LIBRARY_NAMES: &[&str] = &[
    "@executable_path/../Frameworks/libheif.1.dylib",
    "/opt/homebrew/lib/libheif.1.dylib",
    "/usr/local/lib/libheif.1.dylib",
    "libheif.1.dylib",
];
/// Found beside prev.exe or on the PATH, when a build ships it.
#[cfg(windows)]
const LIBRARY_NAMES: &[&str] = &["libheif.dll", "heif.dll"];
const COLORSPACE_RGB: c_int = 1;
const CHROMA_INTERLEAVED_RGBA: c_int = 11;
const CHANNEL_INTERLEAVED: c_int = 10;

#[repr(C)]
struct HeifError {
    code: c_int,
    subcode: c_int,
    message: *const c_char,
}

type Alloc = unsafe extern "C" fn() -> *mut c_void;
type Free = unsafe extern "C" fn(*mut c_void);
type Init = unsafe extern "C" fn(*const c_void) -> HeifError;
type ReadFromMemory =
    unsafe extern "C" fn(*mut c_void, *const c_void, usize, *const c_void) -> HeifError;
type PrimaryHandle = unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> HeifError;
type DecodeImage =
    unsafe extern "C" fn(*const c_void, *mut *mut c_void, c_int, c_int, *const c_void) -> HeifError;
type HandleQuery = unsafe extern "C" fn(*const c_void) -> c_int;
type ImageSize = unsafe extern "C" fn(*const c_void, c_int) -> c_int;
type Plane = unsafe extern "C" fn(*const c_void, c_int, *mut c_int) -> *const u8;

struct Api {
    context_alloc: Alloc,
    context_free: Free,
    read_from_memory: ReadFromMemory,
    primary_handle: PrimaryHandle,
    handle_release: Free,
    handle_premultiplied: HandleQuery,
    decode_image: DecodeImage,
    image_width: ImageSize,
    image_height: ImageSize,
    image_plane: Plane,
    image_release: Free,
    _library: Library,
}

fn api() -> std::result::Result<&'static Api, DecodeError> {
    static API: OnceLock<Option<Api>> = OnceLock::new();
    API.get_or_init(load)
        .as_ref()
        .ok_or_else(|| DecodeError::MissingLibrary("libheif".into()))
}

fn load() -> Option<Api> {
    // SAFETY: libheif runs no unsound initialisers on load.
    let library = LIBRARY_NAMES
        .iter()
        .find_map(|name| unsafe { Library::new(*name) }.ok())?;
    // SAFETY: each symbol is looked up with the signature from libheif's headers.
    unsafe {
        if let Ok(init) = library.get::<Init>(b"heif_init\0") {
            // Loads the decoder plugins; a null argument uses defaults.
            let _ = init(std::ptr::null());
        }
        Some(Api {
            context_alloc: *library.get(b"heif_context_alloc\0").ok()?,
            context_free: *library.get(b"heif_context_free\0").ok()?,
            read_from_memory: *library
                .get(b"heif_context_read_from_memory_without_copy\0")
                .ok()?,
            primary_handle: *library
                .get(b"heif_context_get_primary_image_handle\0")
                .ok()?,
            handle_release: *library.get(b"heif_image_handle_release\0").ok()?,
            handle_premultiplied: *library
                .get(b"heif_image_handle_is_premultiplied_alpha\0")
                .ok()?,
            decode_image: *library.get(b"heif_decode_image\0").ok()?,
            image_width: *library.get(b"heif_image_get_width\0").ok()?,
            image_height: *library.get(b"heif_image_get_height\0").ok()?,
            image_plane: *library.get(b"heif_image_get_plane_readonly\0").ok()?,
            image_release: *library.get(b"heif_image_release\0").ok()?,
            _library: library,
        })
    }
}

fn check(error: HeifError) -> Result<()> {
    if error.code == 0 {
        return Ok(());
    }
    // SAFETY: libheif documents `message` as always set to a static string.
    let message = unsafe { CStr::from_ptr(error.message) }
        .to_string_lossy()
        .into_owned();
    Err(DecodeError::Invalid(message))
}

/// Frees a libheif object when dropped.
struct Owned {
    pointer: *mut c_void,
    free: Free,
}

impl Drop for Owned {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            // SAFETY: `pointer` came from the matching libheif allocator.
            unsafe { (self.free)(self.pointer) }
        }
    }
}

pub fn decode(bytes: &[u8]) -> Result<Decoded> {
    let api = api()?;
    // SAFETY: calls follow libheif's documented sequence; every object is
    // released through `Owned`, and `bytes` outlives the context that reads
    // it without copying.
    unsafe {
        let context = Owned {
            pointer: (api.context_alloc)(),
            free: api.context_free,
        };
        if context.pointer.is_null() {
            return Err(DecodeError::Invalid(
                "libheif could not allocate a context".into(),
            ));
        }
        check((api.read_from_memory)(
            context.pointer,
            bytes.as_ptr().cast(),
            bytes.len(),
            std::ptr::null(),
        ))?;
        let mut handle = std::ptr::null_mut();
        check((api.primary_handle)(context.pointer, &mut handle))?;
        let handle = Owned {
            pointer: handle,
            free: api.handle_release,
        };
        let premultiplied = (api.handle_premultiplied)(handle.pointer) != 0;

        let mut image = std::ptr::null_mut();
        check((api.decode_image)(
            handle.pointer,
            &mut image,
            COLORSPACE_RGB,
            CHROMA_INTERLEAVED_RGBA,
            std::ptr::null(),
        ))?;
        let image = Owned {
            pointer: image,
            free: api.image_release,
        };
        let width = (api.image_width)(image.pointer, CHANNEL_INTERLEAVED);
        let height = (api.image_height)(image.pointer, CHANNEL_INTERLEAVED);
        let mut stride: c_int = 0;
        let plane = (api.image_plane)(image.pointer, CHANNEL_INTERLEAVED, &mut stride);
        if plane.is_null() || width <= 0 || height <= 0 || stride < width * 4 {
            return Err(DecodeError::Invalid(
                "libheif returned no image data".into(),
            ));
        }
        let (width, height, stride) = (width as usize, height as usize, stride as usize);
        let mut pixels = Vec::with_capacity(width * height * 4);
        for row in 0..height {
            let row = std::slice::from_raw_parts(plane.add(row * stride), width * 4);
            pixels.extend_from_slice(row);
        }
        if premultiplied {
            unpremultiply(&mut pixels);
        }
        Ok(Decoded {
            frames: vec![Frame {
                width: width as u32,
                height: height as u32,
                pixels,
                delay: Duration::ZERO,
            }],
        })
    }
}

fn unpremultiply(pixels: &mut [u8]) {
    for pixel in pixels.as_chunks_mut::<4>().0 {
        let alpha = u16::from(pixel[3]);
        if alpha != 0 && alpha != 255 {
            for channel in &mut pixel[..3] {
                *channel = (u16::from(*channel) * 255 / alpha).min(255) as u8;
            }
        }
    }
}

/// Whether libheif is installed and loadable.
pub fn is_available() -> bool {
    api().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_garbage_or_reports_missing_library() {
        match decode(b"not a heif file") {
            Err(DecodeError::Invalid(_) | DecodeError::MissingLibrary(_)) => {}
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn unpremultiplies_partial_alpha_only() {
        let mut pixels = vec![64, 32, 0, 128, 10, 20, 30, 255, 5, 5, 5, 0];
        unpremultiply(&mut pixels);
        assert_eq!(pixels, vec![127, 63, 0, 128, 10, 20, 30, 255, 5, 5, 5, 0]);
    }
}
