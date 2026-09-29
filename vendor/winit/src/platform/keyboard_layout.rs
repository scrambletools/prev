//! prev: the keyboard layout the user is typing with.
//!
//! X11 and Wayland tell clients which of the keymap's layouts is active,
//! but winit keeps it to itself. This publishes what the active layout
//! types on the letter keys, from which an application can tell the
//! layout's script and so its writing direction.

use std::sync::Mutex;

static LETTERS: Mutex<Option<String>> = Mutex::new(None);

/// The letters the active layout types on the three letter rows, in
/// order, or `None` before the first keyboard event.
pub fn letters() -> Option<String> {
    LETTERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

pub(crate) fn publish(letters: String) {
    *LETTERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(letters);
}
