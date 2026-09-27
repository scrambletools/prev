//! Reads the active Omarchy theme's palette and notices when it changes.

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;

const POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    pub name: String,
    pub mode: Mode,
    pub background: Rgb,
    pub foreground: Rgb,
    pub accent: Rgb,
    pub success: Rgb,
    pub warning: Rgb,
    pub danger: Rgb,
}

#[derive(Deserialize)]
struct ColorsFile {
    mode: Option<String>,
    background: String,
    foreground: String,
    accent: Option<String>,
    blue: Option<String>,
    green: Option<String>,
    yellow: Option<String>,
    red: Option<String>,
}

/// `~/.local/state/omarchy/current/theme`; Omarchy uses this fixed path.
/// Omarchy is Linux only.
#[cfg(not(unix))]
pub fn current_theme_dir() -> Option<PathBuf> {
    None
}

/// `~/.local/state/omarchy/current/theme`; Omarchy uses this fixed path.
#[cfg(unix)]
pub fn current_theme_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(|home| PathBuf::from(home).join(".local/state/omarchy/current/theme"))
}

/// The active theme's palette, or `None` when no Omarchy theme is active.
pub fn load(theme_dir: &Path) -> Option<Palette> {
    let text = std::fs::read_to_string(theme_dir.join("colors.toml")).ok()?;
    let name = theme_dir
        .parent()
        .and_then(|dir| std::fs::read_to_string(dir.join("theme.name")).ok())
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "omarchy".to_owned());
    parse(&text, name)
}

fn parse(text: &str, name: String) -> Option<Palette> {
    let colors: ColorsFile = toml::from_str(text).ok()?;
    let background = parse_hex(&colors.background)?;
    let foreground = parse_hex(&colors.foreground)?;
    let pick = |value: &Option<String>, fallback: Rgb| {
        value.as_deref().and_then(parse_hex).unwrap_or(fallback)
    };
    let accent = pick(&colors.accent, pick(&colors.blue, foreground));
    let mode = match colors.mode.as_deref() {
        Some("light") => Mode::Light,
        Some("dark") => Mode::Dark,
        _ if luminance(background) > 0.5 => Mode::Light,
        _ => Mode::Dark,
    };
    Some(Palette {
        name,
        mode,
        background,
        foreground,
        accent,
        success: pick(&colors.green, accent),
        warning: pick(&colors.yellow, accent),
        danger: pick(&colors.red, accent),
    })
}

/// Accepts `#rrggbb` and `#rrggbbaa` (alpha ignored).
fn parse_hex(value: &str) -> Option<Rgb> {
    let digits = value.trim().strip_prefix('#')?;
    if !matches!(digits.len(), 6 | 8) || !digits.is_ascii() {
        return None;
    }
    let channel = |index: usize| u8::from_str_radix(&digits[index..index + 2], 16).ok();
    Some(Rgb {
        red: channel(0)?,
        green: channel(2)?,
        blue: channel(4)?,
    })
}

fn luminance(color: Rgb) -> f32 {
    (0.2126 * f32::from(color.red)
        + 0.7152 * f32::from(color.green)
        + 0.0722 * f32::from(color.blue))
        / 255.0
}

/// Identity of the colors file; changes when a theme is applied, because
/// Omarchy swaps the whole theme directory.
#[cfg(unix)]
fn file_signature(theme_dir: &Path) -> Option<(u64, u64, i64, i64)> {
    use std::os::unix::fs::MetadataExt;
    let metadata = std::fs::metadata(theme_dir.join("colors.toml")).ok()?;
    Some((
        metadata.ino(),
        metadata.len(),
        metadata.mtime(),
        metadata.mtime_nsec(),
    ))
}

#[cfg(not(unix))]
fn file_signature(theme_dir: &Path) -> Option<(u64, std::time::SystemTime)> {
    let metadata = std::fs::metadata(theme_dir.join("colors.toml")).ok()?;
    Some((metadata.len(), metadata.modified().ok()?))
}

/// Polls the theme on a background thread and calls `on_change` after it
/// changes, appears or disappears.
pub fn watch(theme_dir: PathBuf, on_change: impl Fn() + Send + 'static) {
    std::thread::Builder::new()
        .name("prev-omarchy-theme".into())
        .spawn(move || {
            let mut last = file_signature(&theme_dir);
            loop {
                std::thread::sleep(POLL_INTERVAL);
                let current = file_signature(&theme_dir);
                if current != last {
                    last = current;
                    on_change();
                }
            }
        })
        .expect("spawn theme watcher thread");
}

#[cfg(test)]
mod tests {
    use super::*;

    const HACKERMAN: &str = r##"
mode = "dark"
accent = "#82FB9C"
background = "#0B0C16"
foreground = "#ddf7ff"
hyprland_active_border = "rgba(82fb9cff) rgba(9cf7c2ff) 45deg"
red = "#50f872"
yellow = "#50f7d4"
green = "#4fe88f"
blue = "#829dd4"
"##;

    #[test]
    fn parses_omarchy_colors() {
        let palette = parse(HACKERMAN, "hackerman".into()).unwrap();
        assert_eq!(palette.mode, Mode::Dark);
        assert_eq!(
            palette.background,
            Rgb {
                red: 0x0b,
                green: 0x0c,
                blue: 0x16
            }
        );
        assert_eq!(
            palette.accent,
            Rgb {
                red: 0x82,
                green: 0xfb,
                blue: 0x9c
            }
        );
        assert_eq!(
            palette.danger,
            Rgb {
                red: 0x50,
                green: 0xf8,
                blue: 0x72
            }
        );
    }

    #[test]
    fn infers_mode_and_falls_back_for_missing_colors() {
        let palette = parse(
            "background = \"#fafafa\"\nforeground = \"#202020\"\n",
            "x".into(),
        )
        .unwrap();
        assert_eq!(palette.mode, Mode::Light);
        assert_eq!(palette.accent, palette.foreground);
        assert_eq!(palette.danger, palette.accent);
    }

    #[test]
    fn rejects_unusable_files() {
        assert!(parse("foreground = \"#ffffff\"\n", "x".into()).is_none());
        assert!(parse("background = \"red\"\nforeground = \"#fff\"\n", "x".into()).is_none());
        assert!(parse("not toml", "x".into()).is_none());
        assert_eq!(
            parse_hex("#82fb9cff"),
            Some(Rgb {
                red: 0x82,
                green: 0xfb,
                blue: 0x9c
            })
        );
        assert_eq!(parse_hex("#ééé"), None);
    }

    #[test]
    fn signature_changes_when_theme_directory_is_swapped() {
        let root = tempfile::tempdir().unwrap();
        let theme = root.path().join("theme");
        std::fs::create_dir(&theme).unwrap();
        std::fs::write(theme.join("colors.toml"), HACKERMAN).unwrap();
        let before = file_signature(&theme);

        let next = root.path().join("next-theme");
        std::fs::create_dir(&next).unwrap();
        std::fs::write(next.join("colors.toml"), HACKERMAN).unwrap();
        std::fs::remove_dir_all(&theme).unwrap();
        std::fs::rename(&next, &theme).unwrap();
        assert_ne!(file_signature(&theme), before);
        std::fs::write(root.path().join("theme.name"), "hackerman\n").unwrap();
        assert_eq!(load(&theme).unwrap().name, "hackerman");
    }
}
