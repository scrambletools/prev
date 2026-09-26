//! XDG base directories for prev's own files.

use std::env;
use std::path::PathBuf;

const APP_DIR: &str = "prev";

fn xdg_dir(variable: &str, fallback_under_home: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(fallback_under_home)))
}

/// `$XDG_CONFIG_HOME/prev`, for settings.
pub fn config_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CONFIG_HOME", ".config").map(|dir| dir.join(APP_DIR))
}

/// `$XDG_DATA_HOME/prev`, for version history and signatures.
pub fn data_dir() -> Option<PathBuf> {
    xdg_dir("XDG_DATA_HOME", ".local/share").map(|dir| dir.join(APP_DIR))
}

/// `$XDG_STATE_HOME/prev`, for recent files and window state.
pub fn state_dir() -> Option<PathBuf> {
    xdg_dir("XDG_STATE_HOME", ".local/state").map(|dir| dir.join(APP_DIR))
}

/// `$XDG_RUNTIME_DIR/prev`, for the single instance socket.
pub fn runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|dir| dir.join(APP_DIR))
}
