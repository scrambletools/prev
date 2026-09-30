//! Makes prev the system's default app for every file type it opens. This
//! changes only the system's setting, never prev's own.
//!
//! - Linux: the `[Default Applications]` of `mimeapps.list` in the user's
//!   config folder, as `xdg-mime default` writes it, for every MIME type
//!   the desktop entry lists.
//! - macOS: Launch Services' default handler for the common types, PDF
//!   and the usual photo formats. macOS asks the user to confirm each one,
//!   so the rarer types are left to Finder's Get Info.
//! - Windows lets only the user choose default apps, in Settings; this
//!   opens prev's page there.
//!
//! `status` tells how many of those types prev opens now.

/// What changing the default did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// prev is now the default for this many file types.
    Set(usize),
    /// The system's own page for choosing opened, where the user chooses.
    OpenedSettings,
    /// macOS: the change was asked for; the system asks the user to
    /// confirm each type.
    Asked,
}

/// Why prev could not become the default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// Linux: the desktop entry is not installed, so the system has
    /// nothing to open files with.
    NoDesktopEntry,
    /// macOS: prev is not running from prev.app.
    NoBundle,
    Other(String),
}

/// How many of the file types the button covers prev opens now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub set: usize,
    pub total: usize,
}

/// Makes prev the default app. Blocks briefly; run it off the interface
/// thread.
pub async fn make_default() -> Result<Outcome, Failure> {
    platform::make_default().await
}

/// How many of the file types prev opens now; `None` when that cannot be
/// told, such as for a Mac build not run from prev.app.
pub async fn status() -> Option<Status> {
    platform::status()
}

#[cfg(target_os = "linux")]
mod platform {
    use std::path::{Path, PathBuf};

    use super::{Failure, Outcome, Status};

    /// The desktop entry every prev package and install script installs.
    const DESKTOP_ID: &str = "io.github.scrambletools.prev.desktop";
    const SECTION: &str = "[Default Applications]";

    pub async fn make_default() -> Result<Outcome, Failure> {
        let flatpak = std::env::var_os("FLATPAK_ID").is_some();
        // Inside the Flatpak the desktop entry is exported outside the
        // sandbox, where prev cannot look.
        if !flatpak && !desktop_entry_installed() {
            return Err(Failure::NoDesktopEntry);
        }
        let types: Vec<&str> = crate::filetype::supported_mime_types().collect();
        for list in lists(flatpak) {
            let text = match std::fs::read_to_string(&list) {
                Ok(text) => text,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
                Err(error) => return Err(Failure::Other(error.to_string())),
            };
            let updated = set_defaults(&text, &types, DESKTOP_ID);
            if updated != text {
                if let Some(parent) = list.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|error| Failure::Other(error.to_string()))?;
                }
                prev_store::atomic::write(&list, updated.as_bytes())
                    .map_err(|error| Failure::Other(error.to_string()))?;
            }
        }
        Ok(Outcome::Set(types.len()))
    }

    pub fn status() -> Option<Status> {
        let flatpak = std::env::var_os("FLATPAK_ID").is_some();
        let lists = lookup_order(flatpak);
        let texts: Vec<String> = lists
            .iter()
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .collect();
        let types: Vec<&str> = crate::filetype::supported_mime_types().collect();
        let set = types
            .iter()
            .filter(|mime| {
                texts
                    .iter()
                    .find_map(|text| default_in(text, mime))
                    .is_some_and(|desktop| desktop == DESKTOP_ID)
            })
            .count();
        Some(Status {
            set,
            total: types.len(),
        })
    }

    /// The files the system reads defaults from, first one first: each
    /// config and data folder's desktop-specific list, then its
    /// `mimeapps.list`, then the older `defaults.list` in data folders.
    fn lookup_order(flatpak: bool) -> Vec<PathBuf> {
        let home = prev_store::paths::home().unwrap_or_default();
        let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .split(':')
            .filter(|name| !name.is_empty())
            .map(str::to_lowercase)
            .collect();
        let dirs = |variable: &str, fallback: &str| -> Vec<PathBuf> {
            std::env::var(variable)
                .ok()
                .filter(|dirs| !dirs.is_empty())
                .unwrap_or_else(|| fallback.into())
                .split(':')
                .map(PathBuf::from)
                .collect()
        };
        let data_home = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".local/share"));
        let mut config = vec![config_home(flatpak)];
        config.extend(dirs("XDG_CONFIG_DIRS", "/etc/xdg"));
        let mut data = vec![data_home];
        data.extend(dirs("XDG_DATA_DIRS", "/usr/local/share:/usr/share"));
        let mut order = Vec::new();
        for dir in config
            .into_iter()
            .chain(data.iter().map(|dir| dir.join("applications")))
        {
            for desktop in &desktops {
                order.push(dir.join(format!("{desktop}-mimeapps.list")));
            }
            order.push(dir.join("mimeapps.list"));
        }
        order.extend(
            data.iter()
                .map(|dir| dir.join("applications/defaults.list")),
        );
        order
    }

    /// The default app `text`, a `mimeapps.list`, names for `mime`: the
    /// first in its list under `[Default Applications]`.
    pub fn default_in(text: &str, mime: &str) -> Option<String> {
        let mut in_section = false;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                in_section = line == SECTION;
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some((key, value)) = line.split_once('=')
                && key.trim() == mime
            {
                return value
                    .split(';')
                    .map(str::trim)
                    .find(|desktop| !desktop.is_empty())
                    .map(str::to_owned);
            }
        }
        None
    }

    /// The user's `mimeapps.list`, and the desktop's own one beside it
    /// when there is one, such as `hyprland-mimeapps.list`, which the
    /// system reads first.
    fn lists(flatpak: bool) -> Vec<PathBuf> {
        let config = config_home(flatpak);
        let mut lists = vec![config.join("mimeapps.list")];
        if let Ok(desktops) = std::env::var("XDG_CURRENT_DESKTOP") {
            for desktop in desktops.split(':').filter(|name| !name.is_empty()) {
                let own = config.join(format!("{}-mimeapps.list", desktop.to_lowercase()));
                if own.exists() {
                    lists.push(own);
                }
            }
        }
        lists
    }

    /// `$XDG_CONFIG_HOME`, or `~/.config`. The Flatpak's own points into
    /// its sandbox, so there it is always the home folder's.
    fn config_home(flatpak: bool) -> PathBuf {
        let home = prev_store::paths::home().unwrap_or_else(|| PathBuf::from("/"));
        if flatpak {
            return home.join(".config");
        }
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".config"))
    }

    fn desktop_entry_installed() -> bool {
        let home = prev_store::paths::home().unwrap_or_default();
        let data_home = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".local/share"));
        let data_dirs = std::env::var("XDG_DATA_DIRS")
            .ok()
            .filter(|dirs| !dirs.is_empty())
            .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        std::iter::once(data_home)
            .chain(data_dirs.split(':').map(PathBuf::from))
            .any(|dir| {
                Path::new(&dir)
                    .join("applications")
                    .join(DESKTOP_ID)
                    .exists()
            })
    }

    /// `text`, a `mimeapps.list`, with `desktop` the default for each of
    /// `types`: existing lines for them replaced, missing ones added to
    /// `[Default Applications]`, which is made when missing. Everything
    /// else stays as it was.
    pub fn set_defaults(text: &str, types: &[&str], desktop: &str) -> String {
        let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
        let start = match lines.iter().position(|line| line.trim() == SECTION) {
            Some(start) => start,
            None => {
                if lines.last().is_some_and(|line| !line.trim().is_empty()) {
                    lines.push(String::new());
                }
                lines.push(SECTION.to_owned());
                lines.len() - 1
            }
        };
        let end = lines[start + 1..]
            .iter()
            .position(|line| line.trim_start().starts_with('['))
            .map_or(lines.len(), |offset| start + 1 + offset);
        let mut section: Vec<String> = lines[start + 1..end]
            .iter()
            .filter(|line| {
                let key = line.split('=').next().unwrap_or("").trim();
                !types.contains(&key)
            })
            .cloned()
            .collect();
        // Keep a blank line before the next section, if there was one.
        let trailing_blank = section.last().is_some_and(|line| line.trim().is_empty());
        if trailing_blank {
            section.pop();
        }
        section.extend(types.iter().map(|mime| format!("{mime}={desktop}")));
        if trailing_blank || end < lines.len() {
            section.push(String::new());
        }
        lines.splice(start + 1..end, section);
        let mut out = lines.join("\n");
        out.push('\n');
        out
    }
}

#[cfg(target_os = "macos")]
mod platform {
    #![allow(unsafe_code)]

    use std::ffi::c_void;

    use objc2::rc::Retained;
    use objc2_foundation::{NSBundle, NSString};

    use super::{Failure, Outcome, Status};

    type CFStringRef = *const c_void;

    #[link(name = "CoreServices", kind = "framework")]
    unsafe extern "C" {
        fn LSSetDefaultRoleHandlerForContentType(
            content_type: CFStringRef,
            role: u32,
            bundle_id: CFStringRef,
        ) -> i32;
        fn LSCopyDefaultRoleHandlerForContentType(
            content_type: CFStringRef,
            role: u32,
        ) -> CFStringRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFRelease(object: *const c_void);
    }

    /// Every role: viewing and editing.
    const ROLES_ALL: u32 = 0xFFFF_FFFF;

    /// The content types the button asks for: PDF and the usual photo
    /// formats. macOS asks the user about each, so the list stays short.
    pub const TYPES: &[&str] = &[
        "com.adobe.pdf",
        "public.png",
        "public.jpeg",
        "public.heic",
        "com.compuserve.gif",
        "public.tiff",
        "org.webmproject.webp",
        "public.avif",
    ];

    fn bundle() -> Option<Retained<NSString>> {
        NSBundle::mainBundle().bundleIdentifier()
    }

    pub async fn make_default() -> Result<Outcome, Failure> {
        let Some(bundle) = bundle() else {
            return Err(Failure::NoBundle);
        };
        let mut asked = 0;
        let mut last_error = None;
        for identifier in TYPES {
            let content_type = NSString::from_str(identifier);
            // SAFETY: both strings are live NSStrings, which are CFStrings.
            // macOS then asks the user to confirm each change.
            let status = unsafe {
                LSSetDefaultRoleHandlerForContentType(
                    (&raw const *content_type).cast(),
                    ROLES_ALL,
                    (&raw const *bundle).cast(),
                )
            };
            if status == 0 {
                asked += 1;
            } else {
                last_error = Some(status);
            }
        }
        match (asked, last_error) {
            (0, Some(status)) => Err(Failure::Other(format!("Launch Services error {status}"))),
            _ => Ok(Outcome::Asked),
        }
    }

    pub fn status() -> Option<Status> {
        let bundle = bundle()?.to_string();
        let set = TYPES
            .iter()
            .filter(|identifier| {
                let content_type = NSString::from_str(identifier);
                // SAFETY: a live NSString is a CFString; the result is
                // owned, copied, then released.
                let handler = unsafe {
                    LSCopyDefaultRoleHandlerForContentType(
                        (&raw const *content_type).cast(),
                        ROLES_ALL,
                    )
                };
                if handler.is_null() {
                    return false;
                }
                let name = unsafe { &*handler.cast::<NSString>() }.to_string();
                unsafe { CFRelease(handler) };
                name.eq_ignore_ascii_case(&bundle)
            })
            .count();
        Some(Status {
            set,
            total: TYPES.len(),
        })
    }
}

#[cfg(windows)]
mod platform {
    #![allow(unsafe_code)]

    use windows::Win32::UI::Shell::{ASSOCF_NONE, ASSOCSTR_EXECUTABLE, AssocQueryStringW};
    use windows_core::{HSTRING, PWSTR};

    use super::{Failure, Outcome, Status};

    /// Settings' Default apps page for prev, as the installer registers it
    /// under RegisteredApplications. Windows versions without the
    /// per-app page show the Default apps list.
    const PAGE: &str = "ms-settings:defaultapps?registeredAppUser=prev";

    pub async fn make_default() -> Result<Outcome, Failure> {
        crate::portal::open_uri(PAGE.to_owned())
            .await
            .map(|()| Outcome::OpenedSettings)
            .map_err(Failure::Other)
    }

    /// Counts the extensions whose default app is this prev.exe.
    pub fn status() -> Option<Status> {
        let exe = std::env::current_exe().ok()?;
        let name = exe.file_name()?.to_string_lossy().to_lowercase();
        let open = HSTRING::from("open");
        let set = crate::filetype::EXTENSIONS
            .iter()
            .filter(|extension| {
                let extension = HSTRING::from(format!(".{extension}"));
                let mut buffer = [0u16; 1024];
                let mut length = buffer.len() as u32;
                // SAFETY: the buffer and its length are valid for the call,
                // which writes at most `length` characters.
                let result = unsafe {
                    AssocQueryStringW(
                        ASSOCF_NONE,
                        ASSOCSTR_EXECUTABLE,
                        &extension,
                        &open,
                        Some(PWSTR(buffer.as_mut_ptr())),
                        &mut length,
                    )
                };
                if result.is_err() {
                    return false;
                }
                let end = buffer.iter().position(|&unit| unit == 0).unwrap_or(0);
                let path = String::from_utf16_lossy(&buffer[..end]);
                std::path::Path::new(&path)
                    .file_name()
                    .is_some_and(|file| file.to_string_lossy().to_lowercase() == name)
            })
            .count();
        Some(Status {
            set,
            total: crate::filetype::EXTENSIONS.len(),
        })
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::platform::set_defaults;

    const ID: &str = "prev.desktop";

    #[test]
    fn an_empty_list_gets_the_section() {
        assert_eq!(
            set_defaults("", &["application/pdf", "image/png"], ID),
            "[Default Applications]\napplication/pdf=prev.desktop\nimage/png=prev.desktop\n"
        );
    }

    #[test]
    fn other_entries_and_sections_stay() {
        let text = "[Added Associations]\nimage/png=gimp.desktop;\n\n[Default Applications]\napplication/pdf=evince.desktop\ntext/html=firefox.desktop\n\n[Removed Associations]\nimage/gif=eog.desktop;\n";
        let out = set_defaults(text, &["application/pdf", "image/png"], ID);
        assert_eq!(
            out,
            "[Added Associations]\nimage/png=gimp.desktop;\n\n[Default Applications]\ntext/html=firefox.desktop\napplication/pdf=prev.desktop\nimage/png=prev.desktop\n\n[Removed Associations]\nimage/gif=eog.desktop;\n"
        );
    }

    #[test]
    fn a_list_without_the_section_gets_it_at_the_end() {
        let out = set_defaults(
            "[Added Associations]\nimage/png=gimp.desktop;\n",
            &["image/png"],
            ID,
        );
        assert_eq!(
            out,
            "[Added Associations]\nimage/png=gimp.desktop;\n\n[Default Applications]\nimage/png=prev.desktop\n"
        );
    }

    #[test]
    fn the_first_listed_default_counts() {
        use super::platform::default_in;
        let text = "[Added Associations]\nimage/png=gimp.desktop;\n[Default Applications]\nimage/png=prev.desktop;gimp.desktop;\n";
        assert_eq!(
            default_in(text, "image/png").as_deref(),
            Some("prev.desktop")
        );
        assert_eq!(default_in(text, "image/gif"), None);
    }

    #[test]
    fn setting_twice_changes_nothing() {
        let once = set_defaults("", &["image/png"], ID);
        assert_eq!(set_defaults(&once, &["image/png"], ID), once);
    }
}
