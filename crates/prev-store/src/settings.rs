//! User settings, stored as TOML in `$XDG_CONFIG_HOME/prev/settings.toml`.

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

const FILE_NAME: &str = "settings.toml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    pub appearance: Appearance,
    /// Use the active Omarchy theme's colors when its mode matches.
    pub omarchy_palette: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            appearance: Appearance::System,
            omarchy_palette: true,
        }
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
    paths::config_dir().map(|dir| dir.join(FILE_NAME))
}

impl Settings {
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
        };
        settings.save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("appearance = \"dark\""), "{text}");
        assert!(text.contains("omarchy-palette = false"), "{text}");
        assert_eq!(Settings::load_from(&path).unwrap(), settings);
    }

    #[test]
    fn partial_and_unknown_keys_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        std::fs::write(&path, "appearance = \"light\"\nfuture-option = 3\n").unwrap();
        let loaded = Settings::load_from(&path).unwrap();
        assert_eq!(loaded.appearance, Appearance::Light);
        assert!(loaded.omarchy_palette);
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
