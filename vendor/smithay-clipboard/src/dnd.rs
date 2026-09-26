//! File drag and drop onto the application's surfaces.
//!
//! Hyprland delivers drag events only to the first
//! `wl_data_device` a client creates, which is the clipboard's. So the
//! clipboard worker handles drags too and reports them through a
//! process-wide handler.

use std::sync::RwLock;

/// The mime type accepted for file drops.
pub const URI_LIST_MIME: &str = "text/uri-list";

/// A drag over one of the application's surfaces. `surface` is the
/// `wl_surface` proxy pointer, as found in the window's raw handle.
#[derive(Debug, Clone, PartialEq)]
pub enum DragEvent {
    /// A drag entered the surface; `accepted` is false when it carries no files.
    Entered { surface: usize, x: f64, y: f64, accepted: bool },
    Moved { surface: usize, x: f64, y: f64 },
    Left { surface: usize },
    /// Files were dropped, as URIs from a `text/uri-list`.
    Dropped { surface: usize, x: f64, y: f64, uris: Vec<String> },
}

type Handler = Box<dyn Fn(DragEvent) + Send + Sync>;

static HANDLER: RwLock<Option<Handler>> = RwLock::new(None);

/// Installs the handler for drag events. Drags are refused until one is set.
pub fn set_drag_handler(handler: impl Fn(DragEvent) + Send + Sync + 'static) {
    *HANDLER.write().unwrap_or_else(|poison| poison.into_inner()) = Some(Box::new(handler));
}

pub(crate) fn has_handler() -> bool {
    HANDLER.read().map(|handler| handler.is_some()).unwrap_or(false)
}

pub(crate) fn emit(event: DragEvent) {
    if let Ok(handler) = HANDLER.read() {
        if let Some(handler) = handler.as_ref() {
            handler(event);
        }
    }
}

/// Parses a `text/uri-list` body: one URI per line, `#` lines are comments.
pub fn parse_uri_list(body: &str) -> Vec<String> {
    body.lines()
        .map(|line| line.trim_end_matches('\r').trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_uri_lists() {
        let body = "# comment\r\nfile:///tmp/a%20b.pdf\r\n\r\nfile:///tmp/c.png\n";
        assert_eq!(parse_uri_list(body), vec!["file:///tmp/a%20b.pdf", "file:///tmp/c.png"]);
        assert!(parse_uri_list("").is_empty());
    }
}
