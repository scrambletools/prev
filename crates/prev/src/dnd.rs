//! The platform's drag and drop, with the same types everywhere. On Linux
//! it is the vendored smithay-clipboard's Wayland drag and drop, on
//! Windows OLE's (`dnd_windows`). Elsewhere prev cannot start drags yet,
//! and files dropped on a window arrive through iced as `FileDropped`.

#[cfg(target_os = "linux")]
pub use smithay_clipboard::dnd::*;

#[cfg(not(target_os = "linux"))]
pub use other::*;

/// Stops the clipboard worker, which outlives windows, while the Wayland
/// connection is still open; call it just before the app exits.
pub fn shutdown_clipboard() {
    #[cfg(target_os = "linux")]
    smithay_clipboard::shutdown();
}

#[cfg(not(target_os = "linux"))]
mod other {
    /// The mime type of file lists.
    pub const URI_LIST_MIME: &str = "text/uri-list";

    /// What a drop does with the data.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Action {
        Copy,
        /// The source removes what was dragged once it is dropped.
        Move,
    }

    /// A drag over one of the application's windows, or the end of one it
    /// started, as on Linux. Never sent here yet.
    #[derive(Debug, Clone, PartialEq)]
    pub enum DragEvent {
        Entered {
            surface: usize,
            x: f64,
            y: f64,
            accepted: bool,
        },
        Moved {
            surface: usize,
            x: f64,
            y: f64,
        },
        Left {
            surface: usize,
        },
        Dropped {
            surface: usize,
            x: f64,
            y: f64,
            mime: String,
            data: Vec<u8>,
            action: Action,
        },
        SourceEnded {
            action: Option<Action>,
        },
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

    /// A drag to start from the window the pointer button is held on.
    #[derive(Debug, Clone)]
    pub struct Drag {
        pub data: Vec<(String, Vec<u8>)>,
        pub icon: Option<Icon>,
        pub allow_move: bool,
    }

    #[cfg(windows)]
    pub use crate::dnd_windows::{
        register, set_accepted_mimes, set_drag_handler, set_prefer_move, start_drag,
    };

    #[cfg(not(windows))]
    pub fn set_drag_handler(_handler: impl Fn(DragEvent) + Send + Sync + 'static) {}

    #[cfg(not(windows))]
    pub fn set_accepted_mimes(_mimes: Vec<String>) {}

    #[cfg(not(windows))]
    pub fn set_prefer_move(_prefer: bool) {}

    /// Starting drags is not supported here yet.
    #[cfg(not(windows))]
    pub fn start_drag(_drag: Drag) -> bool {
        false
    }

    /// Parses a `text/uri-list` body: one URI per line, `#` lines are
    /// comments.
    pub fn parse_uri_list(body: &str) -> Vec<String> {
        body.lines()
            .map(|line| line.trim_end_matches('\r').trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_owned)
            .collect()
    }
}
