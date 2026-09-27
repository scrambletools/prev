//! File dialogs: through the XDG desktop portal on Linux, and the
//! system's own dialogs (through rfd) elsewhere.

use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use portal::*;

#[cfg(not(target_os = "linux"))]
pub use native::*;

/// A drop-down menu in a file dialog: id, label, initial option id, and
/// (option id, option label) pairs.
pub struct Menu {
    pub id: &'static str,
    pub label: &'static str,
    pub initial: String,
    pub options: Vec<(&'static str, &'static str)>,
}

/// A chosen path and the selected option of each menu, as (menu id, option id).
pub type SavedWithMenus = (PathBuf, Vec<(String, String)>);

#[cfg(target_os = "linux")]
mod portal {
    use std::path::PathBuf;

    use ashpd::desktop::file_chooser::{Choice, FileFilter, SelectedFiles};

    use super::{Menu, SavedWithMenus, file_uri_to_path};
    use crate::filetype;

    fn filter(label: &str, mime_types: impl IntoIterator<Item = &'static str>) -> FileFilter {
        mime_types
            .into_iter()
            .fold(FileFilter::new(label), FileFilter::mimetype)
    }

    /// Asks the user for files to open. Cancelling yields an empty list.
    pub async fn open_files() -> Result<Vec<PathBuf>, String> {
        let all = filter("All supported files", filetype::supported_mime_types());
        let request = SelectedFiles::open_file()
            .title("Open")
            .multiple(true)
            .filters([
                all.clone(),
                filter("PDF documents", filetype::PDF_MIME_TYPES.iter().copied()),
                filter("Images", filetype::IMAGE_MIME_TYPES.iter().copied()),
                filter("SVG drawings", filetype::SVG_MIME_TYPES.iter().copied()),
                filter("Markdown", filetype::MARKDOWN_MIME_TYPES.iter().copied()),
            ])
            .current_filter(all)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .iter()
                .filter_map(|uri| file_uri_to_path(uri.as_str()))
                .collect()),
            Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => Ok(Vec::new()),
            Err(error) => Err(error.to_string()),
        }
    }

    /// Asks for a folder, starting in `current`. Cancelling yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let request = SelectedFiles::open_file()
            .title(title.as_str())
            .directory(true)
            .current_folder::<&std::path::Path>(current.as_deref())
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .first()
                .and_then(|uri| file_uri_to_path(uri.as_str()))),
            Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }

    /// Asks where to save a file, suggesting `name`. Cancelling yields `None`.
    pub async fn save_file(title: String, name: String) -> Result<Option<PathBuf>, String> {
        let request = SelectedFiles::save_file()
            .title(title.as_str())
            .current_name(name.as_str())
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .first()
                .and_then(|uri| file_uri_to_path(uri.as_str()))),
            Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }

    /// Asks where to save, with extra drop-down menus. `None` when cancelled.
    pub async fn save_file_with_menus(
        title: String,
        name: String,
        folder: Option<PathBuf>,
        menus: Vec<Menu>,
    ) -> Result<Option<SavedWithMenus>, String> {
        let choices = menus.iter().map(|menu| {
            menu.options.iter().fold(
                Choice::new(menu.id, menu.label, &menu.initial),
                |choice, (id, label)| choice.insert(id, label),
            )
        });
        let request = SelectedFiles::save_file()
            .title(title.as_str())
            .current_name(name.as_str())
            .current_folder::<&std::path::Path>(folder.as_deref())
            .map_err(|error| error.to_string())?
            .choices(choices)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .first()
                .and_then(|uri| file_uri_to_path(uri.as_str()))
                .map(|path| (path, selected.choices().to_vec()))),
            Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod native {
    use std::path::PathBuf;

    use rfd::AsyncFileDialog;

    use super::{Menu, SavedWithMenus};
    use crate::filetype;

    const PDF: &[&str] = &["pdf"];
    const IMAGES: &[&str] = &[
        "png", "jpg", "jpeg", "gif", "webp", "avif", "heic", "heif", "bmp", "ico", "tif", "tiff",
        "tga", "pnm", "pbm", "pgm", "ppm", "qoi", "hdr", "exr", "jp2", "dng", "cr2", "cr3", "crw",
        "raf", "nef", "nrw", "orf", "rw2", "pef", "arw", "sr2", "srf", "srw", "x3f",
    ];
    const SVG: &[&str] = &["svg", "svgz"];

    /// The folder the last dialog ended in, where the next one starts;
    /// Documents until then, rather than wherever prev was started from.
    static LAST_FOLDER: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

    fn starting(dialog: AsyncFileDialog) -> AsyncFileDialog {
        let last = LAST_FOLDER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .clone();
        let folder = last.or_else(|| {
            prev_store::paths::home()
                .map(|home| home.join("Documents"))
                .filter(|documents| documents.is_dir())
        });
        match folder {
            Some(folder) => dialog.set_directory(folder),
            None => dialog,
        }
    }

    fn remember(path: &std::path::Path) {
        let folder = if path.is_dir() {
            Some(path)
        } else {
            path.parent()
        };
        if let Some(folder) = folder {
            *LAST_FOLDER
                .lock()
                .unwrap_or_else(|poison| poison.into_inner()) = Some(folder.to_path_buf());
        }
    }

    /// Asks the user for files to open. Cancelling yields an empty list.
    pub async fn open_files() -> Result<Vec<PathBuf>, String> {
        let all: Vec<&str> = [PDF, IMAGES, SVG, filetype::MARKDOWN_EXTENSIONS].concat();
        let files = starting(AsyncFileDialog::new())
            .set_title("Open")
            .add_filter("All supported files", &all)
            .add_filter("PDF documents", PDF)
            .add_filter("Images", IMAGES)
            .add_filter("SVG drawings", SVG)
            .add_filter("Markdown", filetype::MARKDOWN_EXTENSIONS)
            .pick_files()
            .await;
        let files: Vec<PathBuf> = files
            .unwrap_or_default()
            .iter()
            .map(|file| file.path().to_path_buf())
            .collect();
        if let Some(first) = files.first() {
            remember(first);
        }
        Ok(files)
    }

    /// Asks for a folder, starting in `current`. Cancelling yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let mut dialog = starting(AsyncFileDialog::new()).set_title(title);
        if let Some(current) = current {
            dialog = dialog.set_directory(current);
        }
        Ok(dialog
            .pick_folder()
            .await
            .map(|folder| folder.path().to_path_buf()))
    }

    /// Asks where to save a file, suggesting `name`. Cancelling yields `None`.
    pub async fn save_file(title: String, name: String) -> Result<Option<PathBuf>, String> {
        let path = starting(AsyncFileDialog::new())
            .set_title(title)
            .set_file_name(name)
            .save_file()
            .await
            .map(|file| file.path().to_path_buf());
        if let Some(path) = &path {
            remember(path);
        }
        Ok(path)
    }

    /// Asks where to save. The system dialog has no room for extra menus,
    /// so each keeps its initial option.
    pub async fn save_file_with_menus(
        title: String,
        name: String,
        folder: Option<PathBuf>,
        menus: Vec<Menu>,
    ) -> Result<Option<SavedWithMenus>, String> {
        let mut dialog = starting(AsyncFileDialog::new())
            .set_title(title)
            .set_file_name(name);
        if let Some(folder) = folder {
            dialog = dialog.set_directory(folder);
        }
        let choices = menus
            .into_iter()
            .map(|menu| (menu.id.to_owned(), menu.initial))
            .collect();
        let path = dialog
            .save_file()
            .await
            .map(|file| file.path().to_path_buf());
        if let Some(path) = &path {
            remember(path);
        }
        Ok(path.map(|path| (path, choices)))
    }
}

/// Converts a local `file://` URI to a path, decoding percent escapes.
pub fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let path = rest.strip_prefix("localhost").unwrap_or(rest);
    if !path.starts_with('/') {
        return None;
    }
    let mut bytes = Vec::with_capacity(path.len());
    let mut input = path.bytes();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let high = input
                .next()
                .and_then(|digit| (digit as char).to_digit(16))?;
            let low = input
                .next()
                .and_then(|digit| (digit as char).to_digit(16))?;
            bytes.push((high * 16 + low) as u8);
        } else {
            bytes.push(byte);
        }
    }
    path_from_bytes(bytes)
}

#[cfg(unix)]
fn path_from_bytes(bytes: Vec<u8>) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    Some(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}

/// `/C:/Users/...` from a `file:///C:/Users/...` URI becomes `C:/Users/...`.
#[cfg(not(unix))]
fn path_from_bytes(bytes: Vec<u8>) -> Option<PathBuf> {
    let path = String::from_utf8(bytes).ok()?;
    let path = match path.as_bytes() {
        [b'/', drive, b':', ..] if drive.is_ascii_alphabetic() => &path[1..],
        _ => &path,
    };
    Some(PathBuf::from(path))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        assert_eq!(
            file_uri_to_path("file:///home/me/a%20b.pdf"),
            Some(PathBuf::from("/home/me/a b.pdf"))
        );
        assert_eq!(
            file_uri_to_path("file://localhost/tmp/x.png"),
            Some(PathBuf::from("/tmp/x.png"))
        );
        assert_eq!(
            file_uri_to_path("file:///tmp/%C3%A9t%C3%A9.jpg"),
            Some(PathBuf::from("/tmp/été.jpg"))
        );
        assert_eq!(file_uri_to_path("file://server/share/x.pdf"), None);
        assert_eq!(file_uri_to_path("https://example.com/x.pdf"), None);
        assert_eq!(file_uri_to_path("file:///bad%zz"), None);
        assert_eq!(file_uri_to_path("file:///cut%4"), None);
    }
}
