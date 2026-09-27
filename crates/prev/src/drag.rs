//! Drag and drop, both ways: what drops onto prev bring, in the types
//! Paste reads, and the drags prev starts, of pages, text and images.
//! The platform side is in `dnd`.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::dnd;
pub use crate::dnd::{Action, DragEvent, Icon};
use prev_pdf::engine::Bitmap;

use crate::paste::{IMAGE_TYPES, PAGES_TYPE, TEXT_TYPES};

/// Largest side of a drag's preview image, in pixels.
const ICON_SIDE: u32 = 128;

/// Set while a drag of page thumbnails is under way inside a window, so
/// the app reports the pointer leaving the window, where the drag becomes
/// one other windows and apps can take.
pub static WATCH_POINTER: AtomicBool = AtomicBool::new(false);

pub fn watch_pointer(watch: bool) {
    WATCH_POINTER.store(watch, Ordering::Relaxed);
}

pub fn watching_pointer() -> bool {
    WATCH_POINTER.load(Ordering::Relaxed)
}

/// Chromium's type for a dragged file's contents, with its name, as in
/// `application/octet-stream;name="photo.png"`.
const NAMED_BYTES: &str = "application/octet-stream;name=";

/// Largest image fetched for a drop that brings only its web address.
const MAX_DOWNLOAD: u64 = 64 << 20;

/// The types drops are taken in, most preferred first: prev's pages,
/// images, a file's contents from a browser, files, then text.
pub fn accepted_types() -> Vec<String> {
    let named_bytes = format!("{NAMED_BYTES}*");
    std::iter::once(PAGES_TYPE)
        .chain(IMAGE_TYPES.iter().map(|(kind, _)| *kind))
        .chain(std::iter::once(named_bytes.as_str()))
        .chain(std::iter::once(dnd::URI_LIST_MIME))
        .chain(TEXT_TYPES)
        .map(str::to_owned)
        .collect()
}

/// What a drop brought.
#[derive(Debug, Clone)]
pub enum Dropped {
    /// Pages dragged from a prev window, as a PDF.
    Pages(Arc<Vec<u8>>),
    Image(Bitmap),
    Files(Vec<PathBuf>),
    Text(String),
    Nothing,
}

/// Reads dropped `data` of type `mime`. Decoding images takes a moment;
/// run it off the UI thread.
pub fn decode(mime: &str, data: Vec<u8>) -> Dropped {
    if mime == PAGES_TYPE {
        return Dropped::Pages(Arc::new(data));
    }
    if let Some((_, format)) = IMAGE_TYPES.iter().find(|(kind, _)| *kind == mime) {
        return crate::paste::decode(&data, *format).map_or(Dropped::Nothing, Dropped::Image);
    }
    if let Some(name) = mime.strip_prefix(NAMED_BYTES) {
        let name = name.trim_matches('"');
        return file_contents(
            data,
            Path::new(name).extension().and_then(|ext| ext.to_str()),
        );
    }
    if mime == dnd::URI_LIST_MIME {
        let uris = dnd::parse_uri_list(&String::from_utf8_lossy(&data));
        let paths: Vec<PathBuf> = uris
            .iter()
            .filter_map(|uri| crate::dialog::file_uri_to_path(uri))
            .collect();
        if !paths.is_empty() {
            return Dropped::Files(paths);
        }
        // A picture from a web page that comes only as its address is
        // fetched; other addresses, or pictures that cannot be fetched,
        // arrive as text.
        let Some(uri) = uris.iter().find(|uri| is_web(uri)) else {
            return Dropped::Nothing;
        };
        return web_image(uri)
            .map(|bytes| file_contents(bytes, None))
            .filter(|dropped| !matches!(dropped, Dropped::Nothing))
            .unwrap_or_else(|| Dropped::Text(uri.clone()));
    }
    if TEXT_TYPES.contains(&mime) {
        let text = String::from_utf8_lossy(&data).replace("\r\n", "\n");
        if !text.trim().is_empty() {
            return Dropped::Text(text);
        }
    }
    Dropped::Nothing
}

/// Reads a drop made while holding Shift, which takes images as files:
/// a file stays itself, and a picture from a web page is saved to the
/// Downloads folder first, as a browser would save it. Pages and text are
/// read as in `decode`.
pub fn decode_as_file(mime: &str, data: Vec<u8>) -> Dropped {
    decode_as_file_in(mime, data, &downloads_dir())
}

fn decode_as_file_in(mime: &str, data: Vec<u8>, folder: &Path) -> Dropped {
    let saved = |name: &str, bytes: &[u8]| {
        save_in(folder, name, bytes).map_or(Dropped::Nothing, |path| Dropped::Files(vec![path]))
    };
    if let Some(name) = mime.strip_prefix(NAMED_BYTES) {
        let name = name.trim_matches('"');
        return match file_contents(data.clone(), None) {
            Dropped::Image(_) | Dropped::Pages(_) => saved(name, &data),
            _ => Dropped::Nothing,
        };
    }
    if let Some((_, format)) = IMAGE_TYPES.iter().find(|(kind, _)| *kind == mime) {
        let extension = format.name().to_ascii_lowercase();
        return saved(&format!("Dropped image.{extension}"), &data);
    }
    if mime == dnd::URI_LIST_MIME {
        let uris = dnd::parse_uri_list(&String::from_utf8_lossy(&data));
        if uris.iter().any(|uri| uri.starts_with("file://")) {
            return decode(mime, data);
        }
        let Some(uri) = uris.iter().find(|uri| is_web(uri)) else {
            return Dropped::Nothing;
        };
        let Some(bytes) = web_image(uri) else {
            return Dropped::Text(uri.clone());
        };
        if !matches!(
            file_contents(bytes.clone(), None),
            Dropped::Image(_) | Dropped::Pages(_)
        ) {
            return Dropped::Text(uri.clone());
        }
        let name = web_name(uri);
        return saved(&name, &bytes);
    }
    decode(mime, data)
}

/// A file name for what `uri` points at: its last path segment.
fn web_name(uri: &str) -> String {
    let path = uri.split(['?', '#']).next().unwrap_or(uri);
    let last = path.rsplit('/').next().unwrap_or_default();
    let decoded = crate::dialog::file_uri_to_path(&format!("file:///{last}")).and_then(|path| {
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
    });
    decoded
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "Dropped image".to_owned())
}

/// The user's Downloads folder, as `user-dirs.dirs` names it, or
/// `~/Downloads`, or the home folder.
fn downloads_dir() -> PathBuf {
    let home = prev_store::paths::home().unwrap_or_else(std::env::temp_dir);
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".config"));
    let named = std::fs::read_to_string(config.join("user-dirs.dirs"))
        .ok()
        .and_then(|dirs| {
            dirs.lines().find_map(|line| {
                let value = line.trim().strip_prefix("XDG_DOWNLOAD_DIR=")?;
                let value = value.trim_matches('"');
                Some(match value.strip_prefix("$HOME") {
                    Some(rest) => home.join(rest.trim_start_matches('/')),
                    None => PathBuf::from(value),
                })
            })
        });
    [named, Some(home.join("Downloads"))]
        .into_iter()
        .flatten()
        .find(|folder| folder.is_dir())
        .unwrap_or(home)
}

/// Writes `bytes` as `name` in `folder`, numbering the name if a file has
/// it already.
fn save_in(folder: &Path, name: &str, bytes: &[u8]) -> Option<PathBuf> {
    let name = sanitize(name);
    let name = Path::new(&name);
    let stem = name.file_stem()?.to_string_lossy().into_owned();
    let extension = name
        .extension()
        .map(|ext| ext.to_string_lossy().into_owned());
    let named = |number: usize| {
        let stem = if number == 1 {
            stem.clone()
        } else {
            format!("{stem} ({number})")
        };
        match &extension {
            Some(extension) => folder.join(format!("{stem}.{extension}")),
            None => folder.join(stem),
        }
    };
    let path = (1..).map(named).find(|path| !path.exists())?;
    std::fs::write(&path, bytes).ok()?;
    Some(path)
}

/// What the contents of a dropped file are: an image, or a PDF's pages.
fn file_contents(data: Vec<u8>, extension: Option<&str>) -> Dropped {
    let header = &data[..data.len().min(64)];
    match crate::filetype::detect(header, extension) {
        Some(crate::filetype::FileKind::Image(format)) => {
            crate::paste::decode(&data, format).map_or(Dropped::Nothing, Dropped::Image)
        }
        Some(crate::filetype::FileKind::Pdf) => Dropped::Pages(Arc::new(data)),
        _ => Dropped::Nothing,
    }
}

fn is_web(uri: &str) -> bool {
    uri.starts_with("https://") || uri.starts_with("http://")
}

/// Extensions of the pictures and documents worth fetching from a web
/// address; other addresses are links, kept as text.
const WEB_EXTENSIONS: [&str; 11] = [
    "png", "jpg", "jpeg", "gif", "webp", "avif", "bmp", "tif", "tiff", "heic", "pdf",
];

/// The contents at `uri` when its name says it is a picture (or a PDF).
fn web_image(uri: &str) -> Option<Vec<u8>> {
    let name = web_name(uri);
    let extension = Path::new(&name).extension()?.to_str()?.to_ascii_lowercase();
    WEB_EXTENSIONS
        .contains(&extension.as_str())
        .then(|| download(uri))
        .flatten()
}

/// Fetches `url` with curl, if it is installed, within a time and size.
fn download(url: &str) -> Option<Vec<u8>> {
    let output = std::process::Command::new("curl")
        .args(["--silent", "--fail", "--location", "--max-time", "20"])
        .args(["--max-filesize", &MAX_DOWNLOAD.to_string(), "--", url])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    (output.status.success() && !output.stdout.is_empty()).then_some(output.stdout)
}

/// Starts a drag of `data`, each type with its bytes, from where the
/// pointer button is held. False when it could not start.
pub fn start(data: Vec<(String, Vec<u8>)>, icon: Option<Icon>, allow_move: bool) -> bool {
    dnd::start_drag(dnd::Drag {
        data,
        icon,
        allow_move,
    })
}

/// `text` in the text types other apps read.
pub fn text_data(text: &str) -> Vec<(String, Vec<u8>)> {
    TEXT_TYPES
        .iter()
        .take(3)
        .map(|kind| ((*kind).to_owned(), text.as_bytes().to_vec()))
        .collect()
}

/// `bitmap` as a PNG, for dragging images out.
pub fn png(bitmap: &Bitmap) -> Option<Vec<u8>> {
    let image = image::RgbaImage::from_raw(bitmap.width, bitmap.height, bitmap.pixels.clone())?;
    let mut png = Vec::new();
    image
        .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    Some(png)
}

/// A preview of an RGBA image for under the pointer, scaled down to fit
/// `ICON_SIDE`, held near its middle.
pub fn icon(width: u32, height: u32, rgba: &[u8]) -> Option<Icon> {
    let image = image::RgbaImage::from_raw(width, height, rgba.to_vec())?;
    let scale = (ICON_SIDE as f32 / width.max(height).max(1) as f32).min(1.0);
    let (width, height) = (
        ((width as f32 * scale).round() as u32).max(1),
        ((height as f32 * scale).round() as u32).max(1),
    );
    let small = image::imageops::thumbnail(&image, width, height);
    // Slightly see-through, as drag previews are.
    let mut rgba = small.into_raw();
    for pixel in rgba.as_chunks_mut::<4>().0 {
        pixel[3] = (u16::from(pixel[3]) * 220 / 255) as u8;
    }
    Some(Icon {
        width,
        height,
        rgba,
        hotspot: (width as i32 / 2, height as i32 / 2),
    })
}

/// A preview from an iced image handle made of RGBA pixels.
pub fn icon_from_handle(handle: &iced::widget::image::Handle) -> Option<Icon> {
    match handle {
        iced::widget::image::Handle::Rgba {
            width,
            height,
            pixels,
            ..
        } => icon(*width, *height, pixels),
        _ => None,
    }
}

/// Where files for drags are written: the cache folder, which other apps
/// can read even when prev runs in a sandbox (a Flatpak's own temporary
/// folder is private), else the temporary folder.
fn drag_folder() -> PathBuf {
    match prev_store::paths::cache_dir() {
        Some(cache) => cache.join("drag"),
        None => std::env::temp_dir().join(format!("prev-drag-{}", std::process::id())),
    }
}

/// Writes `bytes` to a file called `name` in a fresh folder, for drops
/// that want a file. Earlier drag files are removed first.
pub fn temp_file(name: &str, bytes: &[u8]) -> Option<PathBuf> {
    let base = drag_folder();
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).ok()?;
    let path = base.join(sanitize(name));
    std::fs::write(&path, bytes).ok()?;
    Some(path)
}

/// Removes the folder of drag files, for when prev quits.
pub fn clean_up() {
    let _ = std::fs::remove_dir_all(drag_folder());
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|character| if character == '/' { '-' } else { character })
        .collect()
}

/// A `text/uri-list` naming `path`.
pub fn uri_list(path: &Path) -> Vec<u8> {
    let mut uri = String::from("file://");
    for byte in uri_path(path) {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
                uri.push(byte as char)
            }
            b':' if cfg!(windows) => uri.push(':'),
            _ => uri.push_str(&format!("%{byte:02X}")),
        }
    }
    uri.push_str("\r\n");
    uri.into_bytes()
}

/// The bytes of `path` as a URI writes them.
#[cfg(unix)]
fn uri_path(path: &Path) -> Vec<u8> {
    path.as_os_str().as_encoded_bytes().to_vec()
}

/// `C:\Users\me` as `/C:/Users/me`, for `file:///C:/Users/me`.
#[cfg(not(unix))]
fn uri_path(path: &Path) -> Vec<u8> {
    let path = path.to_string_lossy().replace('\\', "/");
    let path = if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    };
    // The drive's colon stays as it is; the rest is escaped as usual.
    path.into_bytes()
}

/// The data for dragging a file: its path only, so file managers copy
/// the file itself rather than saving its contents anew.
pub fn file_data(path: &Path) -> Vec<(String, Vec<u8>)> {
    vec![(dnd::URI_LIST_MIME.to_owned(), uri_list(path))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_are_read_by_type() {
        assert!(matches!(
            decode(PAGES_TYPE, b"%PDF".to_vec()),
            Dropped::Pages(_)
        ));
        assert!(
            matches!(decode("text/plain;charset=utf-8", b"Hi\r\nthere".to_vec()), Dropped::Text(text) if text == "Hi\nthere")
        );
        assert!(matches!(
            decode("text/plain", b"  ".to_vec()),
            Dropped::Nothing
        ));
        let files = decode(
            dnd::URI_LIST_MIME,
            b"file:///tmp/a%20b.png\r\nhttps://example.com/x.png\r\n".to_vec(),
        );
        assert!(matches!(files, Dropped::Files(paths) if paths == [PathBuf::from("/tmp/a b.png")]));
        // A link that is no picture arrives as its address.
        assert!(matches!(
            decode(dnd::URI_LIST_MIME, b"https://example.com\r\n".to_vec()),
            Dropped::Text(text) if text == "https://example.com"
        ));
        assert!(matches!(
            decode("image/png", b"not a png".to_vec()),
            Dropped::Nothing
        ));
    }

    #[test]
    fn browser_file_contents_are_read_by_what_they_are() {
        let image = image::RgbaImage::from_pixel(2, 2, image::Rgba([9, 8, 7, 255]));
        let mut png = Vec::new();
        image
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        let dropped = decode("application/octet-stream;name=\"moon.png\"", png);
        assert!(matches!(dropped, Dropped::Image(bitmap) if bitmap.width == 2));
        let pdf = decode(
            "application/octet-stream;name=\"report.pdf\"",
            b"%PDF-1.7\n".to_vec(),
        );
        assert!(matches!(pdf, Dropped::Pages(_)));
        assert!(matches!(
            decode(
                "application/octet-stream;name=\"notes.txt\"",
                b"hello".to_vec()
            ),
            Dropped::Nothing
        ));
        let types = accepted_types();
        let named = types.iter().position(|kind| kind.ends_with('*')).unwrap();
        assert!(
            named
                < types
                    .iter()
                    .position(|kind| kind == dnd::URI_LIST_MIME)
                    .unwrap()
        );
    }

    #[test]
    fn shift_drops_become_files() {
        let folder = tempfile::tempdir().unwrap();
        let image = image::RgbaImage::from_pixel(2, 2, image::Rgba([9, 8, 7, 255]));
        let mut png = Vec::new();
        image
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        let named = "application/octet-stream;name=\"moon.png\"";
        let first = decode_as_file_in(named, png.clone(), folder.path());
        let second = decode_as_file_in(named, png.clone(), folder.path());
        let path = |dropped: Dropped| match dropped {
            Dropped::Files(mut paths) => paths.remove(0),
            other => panic!("expected a file, got {other:?}"),
        };
        assert_eq!(path(first), folder.path().join("moon.png"));
        assert_eq!(path(second), folder.path().join("moon (2).png"));
        let data = path(decode_as_file_in("image/png", png, folder.path()));
        assert_eq!(data, folder.path().join("Dropped image.png"));
        // Files stay where they are; text and pages are as without Shift.
        let local = decode_as_file_in(
            dnd::URI_LIST_MIME,
            b"file:///tmp/a.png\r\n".to_vec(),
            folder.path(),
        );
        assert!(matches!(local, Dropped::Files(paths) if paths == [PathBuf::from("/tmp/a.png")]));
        assert!(matches!(
            decode_as_file_in("text/plain", b"hi".to_vec(), folder.path()),
            Dropped::Text(_)
        ));
        assert!(matches!(
            decode_as_file_in(named, b"not an image".to_vec(), folder.path()),
            Dropped::Nothing
        ));
        assert_eq!(
            web_name("https://example.com/a/My%20moon.png?size=2"),
            "My moon.png"
        );
        assert_eq!(web_name("https://example.com/"), "Dropped image");
    }

    #[test]
    fn web_images_are_fetched_when_only_their_address_comes() {
        // Nothing listens on port 9: the address arrives as text.
        let dropped = decode(
            dnd::URI_LIST_MIME,
            b"http://127.0.0.1:9/none.png\r\n".to_vec(),
        );
        assert!(matches!(dropped, Dropped::Text(text) if text == "http://127.0.0.1:9/none.png"));
        // Links to pages are not fetched at all.
        let link = decode(
            dnd::URI_LIST_MIME,
            b"https://example.com/article\r\n".to_vec(),
        );
        assert!(matches!(link, Dropped::Text(text) if text == "https://example.com/article"));
        assert!(is_web("https://example.com/a.png") && !is_web("file:///a.png"));
    }

    #[test]
    fn pages_come_first_and_text_last() {
        let types = accepted_types();
        assert_eq!(types[0], PAGES_TYPE);
        assert!(
            types.iter().position(|kind| kind == "image/png")
                < types.iter().position(|kind| kind == dnd::URI_LIST_MIME)
        );
        assert_eq!(types.last().map(String::as_str), Some("TEXT"));
    }

    #[test]
    fn file_uris_round_trip() {
        let path = Path::new("/tmp/My pages (2).pdf");
        let list = String::from_utf8(uri_list(path)).unwrap();
        assert_eq!(list, "file:///tmp/My%20pages%20%282%29.pdf\r\n");
        let uri = dnd::parse_uri_list(&list).remove(0);
        assert_eq!(crate::dialog::file_uri_to_path(&uri).as_deref(), Some(path));
    }

    #[test]
    fn icons_fit_and_center() {
        let icon = icon(400, 200, &vec![255; 400 * 200 * 4]).unwrap();
        assert_eq!((icon.width, icon.height), (128, 64));
        assert_eq!(icon.hotspot, (64, 32));
        assert_eq!(icon.rgba[3], 220);
    }
}
