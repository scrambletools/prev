//! Drag and drop onto, and from, the application's surfaces.
//!
//! Hyprland delivers drag events only to the first `wl_data_device` a
//! client creates, which is the clipboard's. So the clipboard worker
//! handles drags too: it reports drags over the application's surfaces
//! through a process-wide handler, and starts drags the application asks
//! for.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};

use sctk::reexports::calloop::channel::Sender;

use crate::worker::Command;

/// The mime type of file lists.
pub const URI_LIST_MIME: &str = "text/uri-list";

/// What a drop does with the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Copy,
    /// The source removes what was dragged once it is dropped.
    Move,
}

/// A drag over one of the application's surfaces, or the end of one it
/// started. `surface` is the `wl_surface` proxy pointer, as found in the
/// window's raw handle.
#[derive(Debug, Clone, PartialEq)]
pub enum DragEvent {
    /// A drag entered the surface; `accepted` is false when it carries
    /// nothing the application takes.
    Entered { surface: usize, x: f64, y: f64, accepted: bool },
    Moved { surface: usize, x: f64, y: f64 },
    Left { surface: usize },
    /// Data was dropped, in the most preferred of the accepted types the
    /// drag offered.
    Dropped { surface: usize, x: f64, y: f64, mime: String, data: Vec<u8>, action: Action },
    /// A drag the application started ended: dropped with `action`, or
    /// cancelled (`None`).
    SourceEnded { action: Option<Action> },
}

/// An image shown under the pointer while dragging.
#[derive(Debug, Clone)]
pub struct Icon {
    pub width: u32,
    pub height: u32,
    /// RGBA, 8 bits per channel, not premultiplied.
    pub rgba: Vec<u8>,
    /// Where the pointer sits in the image.
    pub hotspot: (i32, i32),
}

/// A drag to start from the surface the pointer button is held on.
#[derive(Debug, Clone)]
pub struct Drag {
    /// Each offered type with its data, most preferred first.
    pub data: Vec<(String, Vec<u8>)>,
    pub icon: Option<Icon>,
    /// Whether the target may move the data rather than copy it.
    pub allow_move: bool,
}

type Handler = Box<dyn Fn(DragEvent) + Send + Sync>;

static HANDLER: RwLock<Option<Handler>> = RwLock::new(None);
static ACCEPTED: RwLock<Vec<String>> = RwLock::new(Vec::new());
static PREFER_MOVE: AtomicBool = AtomicBool::new(false);
static COMMANDS: Mutex<Option<Sender<Command>>> = Mutex::new(None);

/// Installs the handler for drag events. Drags are refused until one is set.
pub fn set_drag_handler(handler: impl Fn(DragEvent) + Send + Sync + 'static) {
    *HANDLER.write().unwrap_or_else(|poison| poison.into_inner()) = Some(Box::new(handler));
}

/// The types drops are taken in, most preferred first. File lists only
/// until this is called. A type ending in `*` takes any type starting with
/// the rest, such as Chromium's `application/octet-stream;name="a.png"`.
pub fn set_accepted_mimes(mimes: Vec<String>) {
    *ACCEPTED.write().unwrap_or_else(|poison| poison.into_inner()) = mimes;
}

/// Whether drops onto the application should move rather than copy, when
/// the source allows it; follows the user's modifier keys.
pub fn set_prefer_move(prefer: bool) {
    PREFER_MOVE.store(prefer, Ordering::Relaxed);
}

/// Starts `drag` from the surface where the pointer button is held.
/// Returns false when there is no clipboard worker to start it.
pub fn start_drag(drag: Drag) -> bool {
    let commands = COMMANDS.lock().unwrap_or_else(|poison| poison.into_inner());
    match commands.as_ref() {
        Some(sender) => sender.send(Command::StartDrag(Box::new(drag))).is_ok(),
        None => false,
    }
}

pub(crate) fn set_commands(sender: Sender<Command>) {
    *COMMANDS.lock().unwrap_or_else(|poison| poison.into_inner()) = Some(sender);
}

pub(crate) fn has_handler() -> bool {
    HANDLER.read().map(|handler| handler.is_some()).unwrap_or(false)
}

pub(crate) fn prefer_move() -> bool {
    PREFER_MOVE.load(Ordering::Relaxed)
}

/// The accepted type to take from `offered`, if any.
pub(crate) fn choose_mime(offered: &[String]) -> Option<String> {
    let accepted = ACCEPTED.read().unwrap_or_else(|poison| poison.into_inner());
    let default = [URI_LIST_MIME.to_owned()];
    let accepted: &[String] = if accepted.is_empty() { &default } else { &accepted };
    accepted.iter().find_map(|wanted| match wanted.strip_suffix('*') {
        Some(prefix) => offered.iter().find(|mime| mime.starts_with(prefix)).cloned(),
        None => offered.contains(wanted).then(|| wanted.clone()),
    })
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

/// Premultiplied BGRA, as `wl_shm`'s ARGB8888 is laid out in memory.
pub(crate) fn to_argb8888(rgba: &[u8]) -> Vec<u8> {
    rgba.chunks_exact(4)
        .flat_map(|pixel| {
            let alpha = u16::from(pixel[3]);
            let premultiply = |channel: u8| ((u16::from(channel) * alpha + 127) / 255) as u8;
            [premultiply(pixel[2]), premultiply(pixel[1]), premultiply(pixel[0]), pixel[3]]
        })
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

    #[test]
    fn chooses_the_most_preferred_offered_type() {
        let offered = |mimes: &[&str]| mimes.iter().map(|mime| mime.to_string()).collect::<Vec<_>>();
        // Only file lists until the application says otherwise.
        assert_eq!(choose_mime(&offered(&["text/plain", URI_LIST_MIME])).as_deref(), Some(URI_LIST_MIME));
        assert_eq!(choose_mime(&offered(&["text/plain"])), None);
        set_accepted_mimes(vec!["image/png".into(), URI_LIST_MIME.into(), "text/plain".into()]);
        assert_eq!(
            choose_mime(&offered(&["text/plain", URI_LIST_MIME, "image/png"])).as_deref(),
            Some("image/png")
        );
        assert_eq!(choose_mime(&offered(&["text/plain"])).as_deref(), Some("text/plain"));
        set_accepted_mimes(vec!["application/octet-stream;name=*".into(), "text/plain".into()]);
        assert_eq!(
            choose_mime(&offered(&["text/plain", "application/octet-stream;name=\"a.png\""]))
                .as_deref(),
            Some("application/octet-stream;name=\"a.png\"")
        );
        set_accepted_mimes(Vec::new());
    }

    #[test]
    fn icons_become_premultiplied_bgra() {
        assert_eq!(to_argb8888(&[255, 128, 0, 255, 200, 100, 50, 0]), [0, 128, 255, 255, 0, 0, 0, 0]);
        assert_eq!(to_argb8888(&[255, 255, 255, 128]), [128, 128, 128, 128]);
    }
}
