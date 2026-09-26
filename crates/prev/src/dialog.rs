//! File dialogs through the XDG desktop portal.

use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};

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
    Some(PathBuf::from(OsString::from_vec(bytes)))
}

#[cfg(test)]
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
