//! prev: files macOS asks the application to open.
//!
//! Finder, the Dock and `open` hand a Mac application its files through
//! the application delegate's `application:openURLs:`, not as arguments,
//! both at launch and while it runs. winit owns the delegate, so it keeps
//! them here until the application takes them.

use std::path::PathBuf;
use std::sync::Mutex;

type Handler = Box<dyn Fn(Vec<PathBuf>) + Send>;

static HANDLER: Mutex<Option<Handler>> = Mutex::new(None);
/// Files that came before a handler was set, such as at launch.
static PENDING: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

/// Calls `handler` with each batch of files macOS asks to open, first
/// with any that came before it was set.
pub fn set_handler(handler: impl Fn(Vec<PathBuf>) + Send + 'static) {
    let pending = std::mem::take(&mut *PENDING.lock().unwrap_or_else(|e| e.into_inner()));
    if !pending.is_empty() {
        handler(pending);
    }
    *HANDLER.lock().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(handler));
}

pub(crate) fn deliver(paths: Vec<PathBuf>) {
    if paths.is_empty() {
        return;
    }
    match &*HANDLER.lock().unwrap_or_else(|e| e.into_inner()) {
        Some(handler) => handler(paths),
        None => PENDING.lock().unwrap_or_else(|e| e.into_inner()).extend(paths),
    }
}
