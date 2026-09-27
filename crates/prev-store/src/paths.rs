//! Where prev keeps its own files: the XDG base directories, or on
//! Windows `%APPDATA%` and `%LOCALAPPDATA%`.

use std::env;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Set when built by `scripts/install.sh`. Every other build is a
/// development build, which keeps its own settings, data and instance
/// socket so it never disturbs the installed copy.
pub const PRODUCTION: bool = option_env!("PREV_PRODUCTION").is_some();

const APP_DIR: &str = if PRODUCTION { "prev" } else { "prev-dev" };

#[cfg(not(windows))]
fn xdg_dir(variable: &str, fallback_under_home: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| home().map(|home| home.join(fallback_under_home)))
}

/// A folder named by a Windows environment variable, such as `APPDATA`.
#[cfg(windows)]
fn known_dir(variable: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

/// The home folder: `$HOME`, or `%USERPROFILE%` on Windows.
pub fn home() -> Option<PathBuf> {
    let variable = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    env::var_os(variable).map(PathBuf::from)
}

/// `$XDG_CONFIG_HOME/prev`, where settings were kept before they moved
/// to `config_file`.
#[cfg(not(windows))]
pub fn config_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CONFIG_HOME", ".config").map(|dir| dir.join(APP_DIR))
}

/// `%APPDATA%\prev`, which also holds the settings file.
#[cfg(windows)]
pub fn config_dir() -> Option<PathBuf> {
    known_dir("APPDATA").map(|dir| dir.join(APP_DIR))
}

/// `$XDG_CONFIG_HOME/prev.toml`, the settings file (`prev-dev.toml` for
/// development builds).
#[cfg(not(windows))]
pub fn config_file() -> Option<PathBuf> {
    xdg_dir("XDG_CONFIG_HOME", ".config").map(|dir| dir.join(format!("{APP_DIR}.toml")))
}

/// `%APPDATA%\prev\prev.toml` (`prev-dev` for development builds).
#[cfg(windows)]
pub fn config_file() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join(format!("{APP_DIR}.toml")))
}

/// Where prev keeps the user's data, as chosen in the settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locations {
    pub signatures: PathBuf,
    pub versions: PathBuf,
    pub bookmarks: PathBuf,
}

impl Locations {
    /// Under `$XDG_DATA_HOME/prev`: `signatures/`, `versions/` and
    /// `bookmarks.toml`.
    pub fn defaults() -> Option<Self> {
        let data = data_dir()?;
        Some(Self {
            signatures: data.join("signatures"),
            versions: data.join("versions"),
            bookmarks: data.join("bookmarks.toml"),
        })
    }
}

static LOCATIONS: RwLock<Option<Locations>> = RwLock::new(None);

/// Sets where the stores keep their files, from the settings.
pub fn set_locations(locations: Locations) {
    *LOCATIONS
        .write()
        .unwrap_or_else(|poison| poison.into_inner()) = Some(locations);
}

/// The locations in use: those set from the settings, or the defaults.
pub fn locations() -> Option<Locations> {
    let set = LOCATIONS
        .read()
        .unwrap_or_else(|poison| poison.into_inner())
        .clone();
    set.or_else(Locations::defaults)
}

/// Replaces a leading `~` with the home folder.
pub fn expand_home(path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), home()) {
        (Ok(rest), Some(home)) => home.join(rest),
        _ => path.to_path_buf(),
    }
}

/// Writes paths under the home folder with `~`, for a readable file.
pub fn abbreviate_home(path: &Path) -> PathBuf {
    match home() {
        Some(home) if path.starts_with(&home) && home != Path::new("/") => {
            Path::new("~").join(path.strip_prefix(&home).unwrap_or(path))
        }
        _ => path.to_path_buf(),
    }
}

/// `$XDG_DATA_HOME/prev`, for version history and signatures.
#[cfg(not(windows))]
pub fn data_dir() -> Option<PathBuf> {
    xdg_dir("XDG_DATA_HOME", ".local/share").map(|dir| dir.join(APP_DIR))
}

/// `$XDG_STATE_HOME/prev`, for recent files and window state.
#[cfg(not(windows))]
pub fn state_dir() -> Option<PathBuf> {
    xdg_dir("XDG_STATE_HOME", ".local/state").map(|dir| dir.join(APP_DIR))
}

/// `$XDG_CACHE_HOME/prev`, for files made for other apps, such as pages
/// dragged out as a PDF.
#[cfg(not(windows))]
pub fn cache_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CACHE_HOME", ".cache").map(|dir| dir.join(APP_DIR))
}

/// `%APPDATA%\prev`, roaming with the user, for version history and
/// signatures.
#[cfg(windows)]
pub fn data_dir() -> Option<PathBuf> {
    config_dir()
}

/// `%LOCALAPPDATA%\prev`, kept on this computer.
#[cfg(windows)]
pub fn state_dir() -> Option<PathBuf> {
    known_dir("LOCALAPPDATA").map(|dir| dir.join(APP_DIR))
}

/// `%LOCALAPPDATA%\prev\cache`.
#[cfg(windows)]
pub fn cache_dir() -> Option<PathBuf> {
    state_dir().map(|dir| dir.join("cache"))
}

/// `$XDG_RUNTIME_DIR/prev`, for the single instance socket.
pub fn runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|dir| dir.join(APP_DIR))
}
