//! Shared pieces of the prev application.

pub mod control;
pub mod default_app;
pub mod dialog;
pub mod dnd;
#[cfg(target_os = "macos")]
mod dnd_macos;
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
pub mod mcp;
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
#[cfg(target_os = "macos")]
pub mod title_bar_macos;
pub mod ui;
// scramble-ui's layout macros, which follow the interface's direction,
// where prev's own were.
pub use scramble_ui::{column, line, row};
