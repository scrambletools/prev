//! Shared pieces of the prev application.

pub mod dialog;
pub mod dnd;
#[cfg(windows)]
mod dnd_windows;
pub mod drag;
pub mod filetype;
pub mod i18n;
pub mod image;
pub mod info;
pub mod input;
#[cfg(unix)]
pub mod instance;
#[cfg(windows)]
#[path = "instance_windows.rs"]
pub mod instance;
pub mod markdown;
pub mod omarchy;
pub mod paste;
pub mod pdf;
pub mod portal;
#[cfg(windows)]
mod print_windows;
pub mod shortcuts;
pub mod ui;
