//! User settings, stored as TOML in `$XDG_CONFIG_HOME/prev.toml`
//! (`prev-dev.toml` for development builds). Settings kept in the older
//! `$XDG_CONFIG_HOME/prev/settings.toml` are read when the new file does
//! not exist yet.

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

const LEGACY_FILE_NAME: &str = "settings.toml";
/// M3's extra large corner, which dialogs use.
pub const DEFAULT_CORNER_RADIUS: f32 = 28.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    pub appearance: Appearance,
    /// Use the active Omarchy theme's colors when its mode matches.
    pub omarchy_palette: bool,
    /// Float the toolbar over the document and hide it while the pointer
    /// is outside the window.
    pub auto_hide_toolbar: bool,
    /// Corner radius of dialogs and floating toolbars, 0 to 32 pixels.
    pub corner_radius: f32,
    /// Interface motion: springs, slides and growing dialogs. Off, or the
    /// system's reduced motion setting, makes changes happen at once.
    pub animations: bool,
    /// Folder of the signature library.
    #[serde(with = "home_path")]
    pub signatures: PathBuf,
    /// Folder of the version history kept for Revert To.
    #[serde(with = "home_path")]
    pub versions: PathBuf,
    /// File of page bookmarks.
    #[serde(with = "home_path")]
    pub bookmarks: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let locations = paths::Locations::defaults().unwrap_or(paths::Locations {
            signatures: PathBuf::from("signatures"),
            versions: PathBuf::from("versions"),
            bookmarks: PathBuf::from("bookmarks.toml"),
        });
        Self {
            appearance: Appearance::System,
            omarchy_palette: true,
            auto_hide_toolbar: false,
            corner_radius: DEFAULT_CORNER_RADIUS,
            animations: true,
            signatures: locations.signatures,
            versions: locations.versions,
            bookmarks: locations.bookmarks,
        }
    }
}

impl Settings {
    /// Where the stores keep their files.
    pub fn locations(&self) -> paths::Locations {
        paths::Locations {
            signatures: self.signatures.clone(),
            versions: self.versions.clone(),
            bookmarks: self.bookmarks.clone(),
        }
    }
}

/// Paths written with `~` for the home folder, and read back expanded.
mod home_path {
    use std::path::PathBuf;

    use serde::{Deserialize, Deserializer, Serializer};

    use crate::paths;

    pub fn serialize<S: Serializer>(
        path: &std::path::Path,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&paths::abbreviate_home(path).to_string_lossy())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PathBuf, D::Error> {
        let text = String::deserialize(deserializer)?;
        Ok(paths::expand_home(&PathBuf::from(text)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Appearance {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse(toml::de::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "cannot read settings: {error}"),
            Self::Parse(error) => write!(formatter, "invalid settings: {error}"),
        }
    }
}

impl std::error::Error for LoadError {}

pub fn default_path() -> Option<PathBuf> {
    paths::config_file()
}

/// Where settings were kept before `default_path`.
pub fn legacy_path() -> Option<PathBuf> {
    paths::config_dir().map(|dir| dir.join(LEGACY_FILE_NAME))
}

impl Settings {
    /// Loads `path`, or when it does not exist yet, `legacy`, then writes
    /// `path` so it lists every setting, the storage paths included.
    pub fn load_or_create(path: &Path, legacy: Option<&Path>) -> Result<Self, LoadError> {
        if path.exists() {
            let settings = Self::load_from(path)?;
            // Every setting shows in the file, so a file missing some, such
            // as one from before a setting existed, gets them. This also
            // pins the storage paths once.
            let text = std::fs::read_to_string(path).map_err(LoadError::Io)?;
            let present = toml::from_str::<toml::Table>(&text).map_err(LoadError::Parse)?;
            let all = toml::Table::try_from(&settings)
                .map_err(|error| LoadError::Io(io::Error::other(error)))?;
            if all.keys().any(|key| !present.contains_key(key)) {
                settings.save_to(path).map_err(LoadError::Io)?;
            }
            return Ok(settings);
        }
        let settings = match legacy {
            Some(legacy) => Self::load_from(legacy)?,
            None => Self::default(),
        };
        settings.save_to(path).map_err(LoadError::Io)?;
        Ok(settings)
    }

    /// A missing file yields the defaults.
    pub fn load_from(path: &Path) -> Result<Self, LoadError> {
        match std::fs::read_to_string(path) {
            Ok(text) => toml::from_str(&text).map_err(LoadError::Parse),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(LoadError::Io(error)),
        }
    }

    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = toml::to_string(self).map_err(io::Error::other)?;
        atomic::write(path, text.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_gives_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = Settings::load_from(&dir.path().join("settings.toml")).unwrap();
        assert_eq!(loaded, Settings::default());
    }

    #[test]
    fn round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("settings.toml");
        let settings = Settings {
            appearance: Appearance::Dark,
            omarchy_palette: false,
            auto_hide_toolbar: true,
            signatures: PathBuf::from("/srv/signatures"),
            ..Settings::default()
        };
        settings.save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("appearance = \"dark\""), "{text}");
        assert!(text.contains("omarchy-palette = false"), "{text}");
        assert!(text.contains("auto-hide-toolbar = true"), "{text}");
        assert!(text.contains("corner-radius = 28"), "{text}");
        assert!(text.contains("animations = true"), "{text}");
        assert!(text.contains("signatures = \"/srv/signatures\""), "{text}");
        assert!(text.contains("versions = "), "{text}");
        assert!(text.contains("bookmarks = "), "{text}");
        assert_eq!(Settings::load_from(&path).unwrap(), settings);
    }

    #[test]
    fn paths_under_home_use_a_tilde() {
        let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("prev.toml");
        let settings = Settings {
            versions: home.join("Archive/versions"),
            ..Settings::default()
        };
        settings.save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("versions = \"~/Archive/versions\""), "{text}");
        assert_eq!(
            Settings::load_from(&path).unwrap().versions,
            home.join("Archive/versions")
        );
    }

    #[test]
    fn a_new_file_takes_over_the_legacy_one_and_lists_everything() {
        let dir = tempfile::tempdir().unwrap();
        let legacy = dir.path().join("prev").join("settings.toml");
        std::fs::create_dir_all(legacy.parent().unwrap()).unwrap();
        std::fs::write(&legacy, "appearance = \"dark\"\n").unwrap();
        let path = dir.path().join("prev.toml");
        let loaded = Settings::load_or_create(&path, Some(&legacy)).unwrap();
        assert_eq!(loaded.appearance, Appearance::Dark);
        let text = std::fs::read_to_string(&path).unwrap();
        for key in ["appearance", "signatures", "versions", "bookmarks"] {
            assert!(text.contains(key), "{key} missing from {text}");
        }
        // A file without the storage paths gets them, keeping the rest.
        std::fs::write(&path, "appearance = \"dark\"\n").unwrap();
        Settings::load_or_create(&path, None).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(
            text.contains("appearance = \"dark\"") && text.contains("versions"),
            "{text}"
        );
        // Once it exists, the new file wins.
        std::fs::write(&legacy, "appearance = \"light\"\n").unwrap();
        let again = Settings::load_or_create(&path, Some(&legacy)).unwrap();
        assert_eq!(again.appearance, Appearance::Dark);
    }

    #[test]
    fn partial_and_unknown_keys_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        std::fs::write(&path, "appearance = \"light\"\nfuture-option = 3\n").unwrap();
        let loaded = Settings::load_from(&path).unwrap();
        assert_eq!(loaded.appearance, Appearance::Light);
        assert!(loaded.omarchy_palette);
        assert!(!loaded.auto_hide_toolbar);
    }

    #[test]
    fn invalid_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        std::fs::write(&path, "appearance = \"sepia\"\n").unwrap();
        assert!(matches!(
            Settings::load_from(&path),
            Err(LoadError::Parse(_))
        ));
    }
}
