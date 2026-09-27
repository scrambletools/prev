//! Reading the system clipboard for Paste: an image, text, or a marker
//! that pages copied in prev are the latest thing copied. Through
//! wl-clipboard, as copying images already is.

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

fn wl_paste(args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("wl-paste")
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| "install wl-clipboard to paste images".to_owned())?;
    // wl-paste fails when the clipboard is empty.
    Ok(if output.status.success() {
        output.stdout
    } else {
        Vec::new()
    })
}

/// Reads the clipboard. Blocks; run it off the UI thread. `Err` when
/// wl-clipboard is missing.
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
pub fn mark_pages() {
    let _ = Command::new("wl-copy")
        .args(["--type", PAGES_TYPE, "pages"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
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
