//! User settings, stored as TOML in `$XDG_CONFIG_HOME/prev.toml`
//! (`prev-dev.toml` for development builds). Settings kept in the older
//! `$XDG_CONFIG_HOME/prev/settings.toml` are read when the new file does
//! not exist yet.

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

const LEGACY_FILE_NAME: &str = "settings.toml";
/// The `language` and `input-language` setting that follows the system.
pub const SYSTEM_LANGUAGE: &str = "system";
/// M3's extra large corner, which dialogs use.
pub const DEFAULT_CORNER_RADIUS: f32 = 20.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    pub appearance: Appearance,
    /// The interface's language, as a tag such as `he` or `pt-BR`, or
    /// `system` to follow the system's language.
    pub language: String,
    /// The language typed into text fields, as a tag, which sets the side
    /// an empty field starts on; or `system` to follow the keyboard layout
    /// in use.
    pub input_language: String,
    /// Build the colors from the system's accent: the Omarchy theme's on
    /// Omarchy, else the desktop's, Windows' or macOS's. Older settings
    /// files call it `omarchy-palette`.
    #[serde(alias = "omarchy-palette")]
    pub system_accent: bool,
    /// The color picked for the scheme, as "#RRGGBB", used when the system
    /// accent is off or the system has none; prev's blue when unset.
    pub accent_color: Option<String>,
    /// Float the toolbar over the document and hide it while the pointer
    /// is outside the window.
    pub auto_hide_toolbar: bool,
    /// Corner radius of dialogs and floating toolbars, 0 to 32 pixels.
    pub corner_radius: f32,
    /// How see-through floating toolbars are, in percent, 0 to 90.
    pub overlay_transparency: f32,
    /// Interface motion: springs, slides and growing dialogs. Off, or the
    /// system's reduced motion setting, makes changes happen at once.
    pub animations: bool,
    /// Let programs such as AI agents control prev over its control
    /// channel, through `prev --mcp`.
    pub outside_control: bool,
    /// The agents, by the names they give, that the user allowed to
    /// control prev. Others are asked about on their first connection.
    pub allowed_agents: Vec<String>,
    /// Folder of the signature library.
    #[serde(with = "home_path")]
    pub signatures: PathBuf,
    /// Folder of the version history kept for Revert To.
    #[serde(with = "home_path")]
    pub versions: PathBuf,
    /// File of page bookmarks.
    #[serde(with = "home_path")]
    pub bookmarks: PathBuf,
    /// Keyboard shortcuts replacing the defaults: an action's name, such
    /// as `export`, set to a shortcut such as "Ctrl+E", a list of them, or
    /// an empty list for none; and the same in `linux`, `windows` and
    /// `macos` tables, which apply only on that system. prev reads it; the
    /// Settings dialog does not show it.
    pub keys: toml::Table,
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
            language: SYSTEM_LANGUAGE.to_owned(),
            input_language: SYSTEM_LANGUAGE.to_owned(),
            system_accent: true,
            accent_color: None,
            auto_hide_toolbar: false,
            corner_radius: DEFAULT_CORNER_RADIUS,
            overlay_transparency: 25.0,
            animations: false,
            outside_control: true,
            allowed_agents: Vec::new(),
            signatures: locations.signatures,
            versions: locations.versions,
            bookmarks: locations.bookmarks,
            keys: toml::Table::new(),
        }
    }
}

impl Settings {
    /// The chosen language's tag, or `None` to follow the system.
    pub fn chosen_language(&self) -> Option<&str> {
        let tag = self.language.trim();
        (!tag.is_empty() && tag != SYSTEM_LANGUAGE).then_some(tag)
    }

    /// The chosen input language's tag, or `None` to follow the keyboard
    /// layout.
    pub fn chosen_input_language(&self) -> Option<&str> {
        let tag = self.input_language.trim();
        (!tag.is_empty() && tag != SYSTEM_LANGUAGE).then_some(tag)
    }

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
            system_accent: false,
            auto_hide_toolbar: true,
            signatures: PathBuf::from("/srv/signatures"),
            allowed_agents: vec!["claude-code".to_owned()],
            ..Settings::default()
        };
        settings.save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("appearance = \"dark\""), "{text}");
        assert!(text.contains("system-accent = false"), "{text}");
        assert!(text.contains("auto-hide-toolbar = true"), "{text}");
        assert!(text.contains("corner-radius = 20"), "{text}");
        assert!(text.contains("animations = false"), "{text}");
        assert!(text.contains("outside-control = true"), "{text}");
        assert!(
            text.contains("allowed-agents = [\"claude-code\"]"),
            "{text}"
        );
        assert!(text.contains("overlay-transparency = 25"), "{text}");
        assert!(text.contains("signatures = \"/srv/signatures\""), "{text}");
        assert!(text.contains("versions = "), "{text}");
        assert!(text.contains("bookmarks = "), "{text}");
        assert_eq!(Settings::load_from(&path).unwrap(), settings);
    }

    #[test]
    fn keys_are_kept_when_saving() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("prev.toml");
        std::fs::write(
            &path,
            "animations = false\n\n[keys]\nexport = \"Ctrl+E\"\nfind-next = [\"Ctrl+G\", \"F3\"]\n\n[keys.macos]\nexport = \"Cmd+Shift+E\"\n",
        )
        .unwrap();
        let settings = Settings::load_or_create(&path, None).unwrap();
        assert_eq!(settings.keys["export"].as_str(), Some("Ctrl+E"));
        assert!(!settings.animations);
        settings.save_to(&path).unwrap();
        let again = Settings::load_from(&path).unwrap();
        assert_eq!(again.keys, settings.keys);
        assert_eq!(again.keys["macos"]["export"].as_str(), Some("Cmd+Shift+E"));
    }

    #[test]
    fn paths_under_home_use_a_tilde() {
        let home = crate::paths::home().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("prev.toml");
        let versions = home.join("Archive").join("versions");
        let settings = Settings {
            versions: versions.clone(),
            ..Settings::default()
        };
        settings.save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        // Written from the tilde, with the system's own separators.
        let table: toml::Table = toml::from_str(&text).unwrap();
        let written = Path::new("~").join("Archive").join("versions");
        assert_eq!(table["versions"].as_str(), written.to_str(), "{text}");
        assert_eq!(Settings::load_from(&path).unwrap().versions, versions);
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
        assert!(loaded.system_accent);
        assert!(!loaded.auto_hide_toolbar);
    }

    #[test]
    fn accent_color_round_trips_and_may_be_absent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        Settings::default().save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(!text.contains("accent-color"), "{text}");
        let settings = Settings {
            accent_color: Some("#E53935".into()),
            ..Settings::default()
        };
        settings.save_to(&path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("accent-color = \"#E53935\""), "{text}");
        assert_eq!(Settings::load_from(&path).unwrap(), settings);
    }

    #[test]
    fn omarchy_palette_is_read_as_system_accent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        std::fs::write(&path, "omarchy-palette = false\n").unwrap();
        assert!(!Settings::load_from(&path).unwrap().system_accent);
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

    #[test]
    fn language_follows_the_system_unless_chosen() {
        let mut settings = Settings::default();
        assert_eq!(settings.language, SYSTEM_LANGUAGE);
        assert_eq!(settings.chosen_language(), None);
        settings.language = "he".into();
        assert_eq!(settings.chosen_language(), Some("he"));
        settings.language = String::new();
        assert_eq!(settings.chosen_language(), None);
        let text = toml::to_string(&Settings::default()).unwrap();
        assert!(text.contains("language = \"system\""), "{text}");
    }

    #[test]
    fn input_language_follows_the_keyboard_unless_chosen() {
        let mut settings = Settings::default();
        assert_eq!(settings.chosen_input_language(), None);
        settings.input_language = "ar".into();
        assert_eq!(settings.chosen_input_language(), Some("ar"));
        let text = toml::to_string(&Settings::default()).unwrap();
        assert!(text.contains("input-language = \"system\""), "{text}");
    }
}
