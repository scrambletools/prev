//! prev: drags over a window, for the application to handle itself.
//!
//! winit's macOS window takes only file names from a drag. With a hook
//! set, the window's dragging destination methods call it instead, with
//! the `NSWindow` and the `NSDraggingInfo` as pointers, so the application
//! can read any type from the drag's pasteboard with its own bindings. The
//! hook returns the `NSDragOperation` to show, or for a drop, nonzero when
//! it took the data.

use std::ffi::c_void;
use std::sync::RwLock;

/// Which of the dragging destination methods is calling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Entered,
    Updated,
    Exited,
    Perform,
}

type Hook = Box<dyn Fn(Phase, *const c_void, *const c_void) -> usize + Send + Sync>;

static HOOK: RwLock<Option<Hook>> = RwLock::new(None);

/// Handles drags over every window with `hook`: its arguments are the
/// phase, the `NSWindow` and the `NSDraggingInfo` (null for `Exited`
/// when AppKit passes none). It runs on the main thread.
pub fn set_hook(hook: impl Fn(Phase, *const c_void, *const c_void) -> usize + Send + Sync + 'static) {
    *HOOK.write().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(hook));
}

/// The hook's answer, or `None` when no hook is set.
pub(crate) fn call(phase: Phase, window: *const c_void, info: *const c_void) -> Option<usize> {
    let hook = HOOK.read().unwrap_or_else(|e| e.into_inner());
    hook.as_ref().map(|hook| hook(phase, window, info))
}
