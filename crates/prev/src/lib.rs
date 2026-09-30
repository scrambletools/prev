//! Shared pieces of the prev application.

pub mod default_app;
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
#[cfg(target_os = "macos")]
pub mod menu_macos;
pub mod omarchy;
pub mod paste;
pub mod pdf;
pub mod portal;
#[cfg(target_os = "macos")]
mod print_macos;
#[cfg(windows)]
mod print_windows;
pub mod shortcuts;
pub mod ui;
