//! Reading the system clipboard for Paste: an image, text, or a marker
//! that pages copied in prev are the latest thing copied; and copying
//! images and that marker. Through wl-clipboard on Linux, the Windows
//! clipboard on Windows, and the general pasteboard on macOS.

#[cfg(not(any(windows, target_os = "macos")))]
use std::process::{Command, Stdio};

use prev_image::ImageFormat;
use prev_pdf::engine::Bitmap;

/// Offered while pages copied in prev are what the clipboard holds, so
/// Paste knows they are newer than any text or image.
pub const PAGES_TYPE: &str = "application/x-prev-pages";

/// What Paste would put in.
#[derive(Debug, Clone)]
pub enum Clip {
    /// Pages copied in prev.
    Pages,
    Image(Bitmap),
    Text(String),
    Nothing,
}

/// Image types in the order they are asked for.
pub(crate) const IMAGE_TYPES: [(&str, ImageFormat); 7] = [
    ("image/png", ImageFormat::Png),
    ("image/jpeg", ImageFormat::Jpeg),
    ("image/webp", ImageFormat::WebP),
    ("image/avif", ImageFormat::Avif),
    ("image/tiff", ImageFormat::Tiff),
    ("image/bmp", ImageFormat::Bmp),
    ("image/gif", ImageFormat::Gif),
];

pub(crate) const TEXT_TYPES: [&str; 5] = [
    "text/plain;charset=utf-8",
    "UTF8_STRING",
    "text/plain",
    "STRING",
    "TEXT",
];

#[cfg(not(any(windows, target_os = "macos")))]
fn wl_paste(args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("wl-paste")
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| crate::fl!("app-paste-needs-wl-clipboard"))?;
    // wl-paste fails when the clipboard is empty.
    Ok(if output.status.success() {
        output.stdout
    } else {
        Vec::new()
    })
}

/// Reads the clipboard. Blocks; run it off the UI thread. `Err` when
/// wl-clipboard is missing.
#[cfg(not(any(windows, target_os = "macos")))]
pub fn read() -> Result<Clip, String> {
    let listed = String::from_utf8_lossy(&wl_paste(&["--list-types"])?).into_owned();
    let types: Vec<&str> = listed.lines().map(str::trim).collect();
    Ok(choose(&types, |kind| {
        wl_paste(&["--no-newline", "--type", kind]).unwrap_or_default()
    }))
}

/// Picks what to paste from the offered `types`, fetching contents with
/// `fetch`: prev's pages, then an image, an image file copied in a file
/// manager, then text.
fn choose(types: &[&str], fetch: impl Fn(&str) -> Vec<u8>) -> Clip {
    if types.contains(&PAGES_TYPE) {
        return Clip::Pages;
    }
    for (kind, format) in IMAGE_TYPES {
        if types.contains(&kind)
            && let Some(bitmap) = decode(&fetch(kind), format)
        {
            return Clip::Image(bitmap);
        }
    }
    if types.contains(&"text/uri-list")
        && let Some(bitmap) = image_file(&String::from_utf8_lossy(&fetch("text/uri-list")))
    {
        return Clip::Image(bitmap);
    }
    for kind in TEXT_TYPES {
        if types.contains(&kind) {
            let text = String::from_utf8_lossy(&fetch(kind)).into_owned();
            if !text.trim().is_empty() {
                return Clip::Text(text);
            }
        }
    }
    Clip::Nothing
}

pub(crate) fn decode(bytes: &[u8], format: ImageFormat) -> Option<Bitmap> {
    let decoded = prev_image::decode::decode(bytes, format).ok()?;
    let frame = decoded.frames.into_iter().next()?;
    Some(Bitmap {
        width: frame.width,
        height: frame.height,
        pixels: frame.pixels,
    })
}

/// The first file in a URI list, if it is an image.
fn image_file(uris: &str) -> Option<Bitmap> {
    let path = uris
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#'))
        .find_map(crate::dialog::file_uri_to_path)?;
    let Ok(Some(crate::filetype::FileKind::Image(format))) = crate::filetype::detect_path(&path)
    else {
        return None;
    };
    decode(&std::fs::read(&path).ok()?, format)
}

/// Marks pages as the latest thing copied, replacing what the clipboard
/// held. Blocks briefly; wl-copy keeps serving it in the background.
#[cfg(not(any(windows, target_os = "macos")))]
pub fn mark_pages() {
    let _ = Command::new("wl-copy")
        .args(["--type", PAGES_TYPE, "pages"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

/// The pixels as a PNG.
fn png_bytes(bitmap: &Bitmap) -> Result<Vec<u8>, String> {
    let image = image::RgbaImage::from_raw(bitmap.width, bitmap.height, bitmap.pixels.clone())
        .ok_or_else(|| crate::fl!("app-copy-no-pixels"))?;
    let mut png = Vec::new();
    image
        .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(|error| error.to_string())?;
    Ok(png)
}

/// Copies `bitmap` as a PNG image, replacing what the clipboard held.
#[cfg(not(any(windows, target_os = "macos")))]
pub fn copy_image(bitmap: &Bitmap) -> Result<(), String> {
    use std::io::Write;
    let png = png_bytes(bitmap)?;
    let mut child = Command::new("wl-copy")
        .args(["--type", "image/png"])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|_| crate::fl!("app-copy-needs-wl-clipboard"))?;
    child
        .stdin
        .take()
        .ok_or_else(|| crate::fl!("app-copy-no-input"))?
        .write_all(&png)
        .map_err(|error| error.to_string())?;
    let status = child.wait().map_err(|error| error.to_string())?;
    status
        .success()
        .then_some(())
        .ok_or_else(|| crate::fl!("app-copy-failed"))
}

#[cfg(target_os = "macos")]
pub use mac::{copy_image, mark_pages, read};

/// The macOS general pasteboard, its types named for `choose`: prev's
/// page marker, images, a copied file's URL and UTF-8 text.
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
mod mac {
    use objc2::rc::Retained;
    use objc2_app_kit::NSPasteboard;
    use objc2_foundation::{NSData, NSString};
    use prev_pdf::engine::Bitmap;

    use super::{Clip, PAGES_TYPE, choose, png_bytes};

    /// Pasteboard types and the names `choose` knows them by.
    const TYPES: [(&str, &str); 10] = [
        ("io.github.scrambletools.prev.pages", PAGES_TYPE),
        ("public.png", "image/png"),
        ("public.jpeg", "image/jpeg"),
        ("org.webmproject.webp", "image/webp"),
        ("public.avif", "image/avif"),
        ("public.tiff", "image/tiff"),
        ("com.microsoft.bmp", "image/bmp"),
        ("com.compuserve.gif", "image/gif"),
        ("public.file-url", "text/uri-list"),
        ("public.utf8-plain-text", "text/plain;charset=utf-8"),
    ];

    fn board() -> Retained<NSPasteboard> {
        NSPasteboard::generalPasteboard()
    }

    fn pasteboard_type(name: &str) -> Option<&'static str> {
        TYPES
            .iter()
            .find(|(_, known)| *known == name)
            .map(|(kind, _)| *kind)
    }

    /// Reads the pasteboard. The general pasteboard may be read from any
    /// thread.
    pub fn read() -> Result<Clip, String> {
        let board = board();
        let offered: Vec<String> = board
            .types()
            .map(|types| types.iter().map(|kind| kind.to_string()).collect())
            .unwrap_or_default();
        let names: Vec<&str> = TYPES
            .iter()
            .filter(|(kind, _)| offered.iter().any(|offered| offered == kind))
            .map(|(_, name)| *name)
            .collect();
        Ok(choose(&names, |name| {
            pasteboard_type(name)
                .and_then(|kind| board.dataForType(&NSString::from_str(kind)))
                .map(|data| data.to_vec())
                .unwrap_or_default()
        }))
    }

    /// Marks pages as the latest thing copied, replacing what the
    /// pasteboard held.
    pub fn mark_pages() {
        let board = board();
        board.clearContents();
        let kind = NSString::from_str(pasteboard_type(PAGES_TYPE).unwrap_or_default());
        board.setString_forType(&NSString::from_str("pages"), &kind);
    }

    /// Copies `bitmap` as a PNG image, replacing what the pasteboard held.
    pub fn copy_image(bitmap: &Bitmap) -> Result<(), String> {
        let png = png_bytes(bitmap)?;
        let board = board();
        board.clearContents();
        let data = NSData::with_bytes(&png);
        board
            .setData_forType(Some(&data), &NSString::from_str("public.png"))
            .then_some(())
            .ok_or_else(|| crate::fl!("app-copy-failed"))
    }
}

#[cfg(windows)]
pub(crate) use windows::{bmp_file, dib};
#[cfg(windows)]
pub use windows::{copy_image, mark_pages, read};

/// The Windows clipboard, in the types `choose` understands: prev's page
/// marker and PNG as registered formats, bitmaps as BMP files, copied
/// files as a URI list, and Unicode text.
#[cfg(windows)]
mod windows {
    use clipboard_win::{Clipboard, formats, raw};

    use super::{Bitmap, Clip, PAGES_TYPE, choose, png_bytes};

    fn open() -> Result<Clipboard, String> {
        Clipboard::new_attempts(10)
            .map_err(|error| crate::fl!("app-clipboard-open-failed", error = error.to_string()))
    }

    fn registered(name: &str) -> Option<u32> {
        raw::register_format(name).map(|format| format.get())
    }

    /// Reads the clipboard. Blocks briefly; run it off the UI thread.
    pub fn read() -> Result<Clip, String> {
        let _open = open()?;
        let pages = registered(PAGES_TYPE);
        let png = registered("PNG");
        let offered = |format: Option<u32>| format.is_some_and(raw::is_format_avail);
        let mut types = Vec::new();
        if offered(pages) {
            types.push(PAGES_TYPE);
        }
        if offered(png) {
            types.push("image/png");
        }
        if offered(Some(formats::CF_DIB)) {
            types.push("image/bmp");
        }
        if offered(Some(formats::CF_HDROP)) {
            types.push("text/uri-list");
        }
        if offered(Some(formats::CF_UNICODETEXT)) {
            types.push("text/plain;charset=utf-8");
        }
        Ok(choose(&types, |kind| {
            let mut data = Vec::new();
            match kind {
                "image/png" => {
                    if let Some(png) = png {
                        let _ = raw::get_vec(png, &mut data);
                    }
                }
                "image/bmp" => {
                    let _ = raw::get_vec(formats::CF_DIB, &mut data);
                    data = bmp_file(&data).unwrap_or_default();
                }
                "text/uri-list" => {
                    let mut paths = Vec::new();
                    let _ = raw::get_file_list_path(&mut paths);
                    for path in paths {
                        data.extend(crate::drag::uri_list(&path));
                    }
                }
                "text/plain;charset=utf-8" => {
                    let _ = raw::get_string(&mut data);
                }
                _ => {}
            }
            data
        }))
    }

    /// Marks pages as the latest thing copied, replacing what the
    /// clipboard held.
    pub fn mark_pages() {
        let (Ok(_open), Some(pages)) = (open(), registered(PAGES_TYPE)) else {
            return;
        };
        let _ = raw::set(pages, b"pages");
    }

    /// Copies `bitmap` as PNG, for apps that take it, and as a bitmap for
    /// the rest, replacing what the clipboard held.
    pub fn copy_image(bitmap: &Bitmap) -> Result<(), String> {
        let png = png_bytes(bitmap)?;
        let _open = open()?;
        let failed = |error: clipboard_win::ErrorCode| {
            crate::fl!("app-copy-image-failed", error = error.to_string())
        };
        raw::empty().map_err(failed)?;
        if let Some(format) = registered("PNG") {
            raw::set_without_clear(format, &png).map_err(failed)?;
        }
        raw::set_without_clear(formats::CF_DIB, &dib(bitmap)).map_err(failed)
    }

    /// A BMP file from a clipboard DIB, which is the file without its
    /// 14 byte file header.
    pub(crate) fn bmp_file(dib: &[u8]) -> Option<Vec<u8>> {
        let u32_at = |at: usize| Some(u32::from_le_bytes(dib.get(at..at + 4)?.try_into().ok()?));
        let header = u32_at(0)? as usize;
        let bits = u16::from_le_bytes(dib.get(14..16)?.try_into().ok()?);
        let compression = u32_at(16)?;
        let colors = u32_at(32)? as usize;
        let palette = if colors > 0 {
            colors * 4
        } else if bits <= 8 {
            (1 << bits) * 4
        } else {
            0
        };
        // BI_BITFIELDS keeps its three masks after a plain info header.
        let masks = if compression == 3 && header == 40 {
            12
        } else {
            0
        };
        let offset = 14 + header + palette + masks;
        let size = u32::try_from(14 + dib.len()).ok()?;
        let mut file = Vec::with_capacity(14 + dib.len());
        file.extend_from_slice(b"BM");
        file.extend_from_slice(&size.to_le_bytes());
        file.extend_from_slice(&[0; 4]);
        file.extend_from_slice(&u32::try_from(offset).ok()?.to_le_bytes());
        file.extend_from_slice(dib);
        Some(file)
    }

    /// `bitmap` as a 32 bit, bottom-up DIB with a plain info header.
    pub(crate) fn dib(bitmap: &Bitmap) -> Vec<u8> {
        let (width, height) = (bitmap.width as usize, bitmap.height as usize);
        let mut dib = Vec::with_capacity(40 + width * height * 4);
        dib.extend_from_slice(&40u32.to_le_bytes());
        dib.extend_from_slice(&(bitmap.width as i32).to_le_bytes());
        dib.extend_from_slice(&(bitmap.height as i32).to_le_bytes());
        dib.extend_from_slice(&1u16.to_le_bytes());
        dib.extend_from_slice(&32u16.to_le_bytes());
        dib.extend_from_slice(&0u32.to_le_bytes());
        dib.extend_from_slice(&((width * height * 4) as u32).to_le_bytes());
        dib.extend_from_slice(&[0; 16]);
        for row in bitmap.pixels.chunks_exact(width * 4).rev() {
            for pixel in row.as_chunks::<4>().0 {
                dib.extend_from_slice(&[pixel[2], pixel[1], pixel[0], pixel[3]]);
            }
        }
        dib
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png() -> Vec<u8> {
        let image = image::RgbaImage::from_pixel(3, 2, image::Rgba([1, 2, 3, 255]));
        let mut bytes = Vec::new();
        image
            .write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::Png,
            )
            .unwrap();
        bytes
    }

    #[test]
    fn copied_pages_come_first() {
        let clip = choose(&[PAGES_TYPE, "text/plain"], |_| b"pages".to_vec());
        assert!(matches!(clip, Clip::Pages));
    }

    #[test]
    fn images_come_before_text() {
        let clip = choose(&["text/plain", "image/png"], |kind| match kind {
            "image/png" => png(),
            _ => b"caption".to_vec(),
        });
        let Clip::Image(bitmap) = clip else {
            panic!("expected an image, got {clip:?}");
        };
        assert_eq!((bitmap.width, bitmap.height), (3, 2));
    }

    #[test]
    fn text_is_pasted_as_text() {
        let clip = choose(&["UTF8_STRING", "TEXT"], |_| b"Hello there".to_vec());
        assert!(matches!(clip, Clip::Text(text) if text == "Hello there"));
        let clip = choose(&["text/plain"], |_| b"   ".to_vec());
        assert!(matches!(clip, Clip::Nothing));
        assert!(matches!(choose(&[], |_| Vec::new()), Clip::Nothing));
    }

    #[cfg(windows)]
    #[test]
    fn bitmaps_round_trip_through_a_dib() {
        let bitmap = Bitmap {
            width: 2,
            height: 2,
            pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 9, 9, 9, 255],
        };
        let file = windows::bmp_file(&windows::dib(&bitmap)).unwrap();
        let read = decode(&file, ImageFormat::Bmp).unwrap();
        assert_eq!((read.width, read.height), (2, 2));
        assert_eq!(&read.pixels[..3], &[255, 0, 0]);
        assert_eq!(&read.pixels[12..15], &[9, 9, 9]);
    }

    #[test]
    fn copied_image_files_paste_as_images() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("copied.png");
        std::fs::write(&path, png()).unwrap();
        let uris = String::from_utf8(crate::drag::uri_list(&path)).unwrap();
        let clip = choose(&["text/uri-list", "text/plain"], |kind| match kind {
            "text/uri-list" => uris.clone().into_bytes(),
            _ => path.display().to_string().into_bytes(),
        });
        assert!(matches!(clip, Clip::Image(_)));
    }
}
