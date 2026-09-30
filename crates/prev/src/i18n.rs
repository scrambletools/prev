//! The interface's text in the user's language.
//!
//! Every string prev shows lives in `i18n/<language>/prev.ftl` at the top
//! of the repository, in Project Fluent's format. `i18n/en` is the source:
//! the `fl!` macro checks each key against it when prev is compiled, and
//! other languages fall back to it for keys they lack. Adding a language
//! is adding its folder; no code changes.

use std::collections::BTreeSet;
use std::sync::{LazyLock, Mutex, PoisonError, RwLock};

use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::unic_langid::{CharacterDirection, LanguageIdentifier};
use i18n_embed::{DesktopLanguageRequester, LanguageLoader};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../i18n/"]
struct Localizations;

/// The loaded text, in the first language the user asks for that prev
/// has, else English.
pub static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = fluent_language_loader!();
    select(&loader);
    loader
});

/// The language chosen in Settings; none follows the system.
static CHOSEN: RwLock<Option<LanguageIdentifier>> = RwLock::new(None);

/// Loads the best language for the user's requests into `loader`.
fn select(loader: &FluentLanguageLoader) {
    let _ = i18n_embed::select(loader, &Localizations, &requested_languages());
    // Values placed in a sentence are wrapped in direction marks so a
    // Latin file name keeps its order inside right to left text; left to
    // right languages need no marks.
    let right_to_left = direction_of(&loader.current_language()) == CharacterDirection::RTL;
    loader.set_use_isolating(right_to_left);
    crate::ui::dir::set_right_to_left(right_to_left);
}

/// The languages the user asks for, most wanted first: `PREV_LANG` (a
/// language tag such as `he` or `pt-BR`, for trying translations), then
/// the one chosen in Settings, then the system's.
fn requested_languages() -> Vec<LanguageIdentifier> {
    // Tests compare English text, whatever the machine's language.
    if cfg!(test) {
        return vec!["en".parse().expect("a language tag")];
    }
    let chosen = CHOSEN
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let mut languages: Vec<LanguageIdentifier> = std::env::var("PREV_LANG")
        .ok()
        .and_then(|tag| tag.parse().ok())
        .into_iter()
        .chain(chosen)
        .collect();
    languages.extend(DesktopLanguageRequester::requested_languages());
    languages
}

/// Shows the interface in the language tagged `tag` (such as `he`), or in
/// the system's language for `None` or a tag prev has no text for. Text
/// shown from then on is in that language.
pub fn set_language(tag: Option<&str>) {
    *CHOSEN.write().unwrap_or_else(PoisonError::into_inner) = tag.and_then(|tag| tag.parse().ok());
    select(&LOADER);
}

/// The languages prev has text for: each one's tag and its name in its
/// own language, such as ("he", "עברית"), sorted by tag.
pub fn languages() -> &'static [(String, String)] {
    static LANGUAGES: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
        let mut languages: Vec<(String, String)> = available()
            .into_iter()
            .map(|language| (language.to_string(), name_of(&language)))
            .collect();
        languages.sort();
        languages
    });
    &LANGUAGES
}

/// `language`'s name for itself, from its own file.
fn name_of(language: &LanguageIdentifier) -> String {
    let loader = fluent_language_loader!();
    let _ = loader.load_languages(&Localizations, std::slice::from_ref(language));
    loader.get("language-name")
}

/// The name of the language the system asks for, as prev would show it
/// when following the system.
pub fn system_language_name() -> String {
    let loader = fluent_language_loader!();
    let _ = i18n_embed::select(
        &loader,
        &Localizations,
        &DesktopLanguageRequester::requested_languages(),
    );
    loader.get("language-name")
}

/// Text that must outlive the view showing it, such as a placeholder iced
/// borrows. Each distinct text is kept once for the life of prev, so after
/// a language change the new text is kept too.
pub fn lasting(text: String) -> &'static str {
    static KEPT: Mutex<BTreeSet<&'static str>> = Mutex::new(BTreeSet::new());
    let mut kept = KEPT.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(found) = kept.get(text.as_str()) {
        return found;
    }
    let text: &'static str = Box::leak(text.into_boxed_str());
    kept.insert(text);
    text
}

fn direction_of(language: &LanguageIdentifier) -> CharacterDirection {
    language.character_direction()
}

/// The language the interface is shown in.
pub fn language() -> LanguageIdentifier {
    LOADER.current_language()
}

/// Whether the interface's language is written right to left, so layouts
/// mirror.
pub fn right_to_left() -> bool {
    direction_of(&language()) == CharacterDirection::RTL
}

/// Whether the language with `tag`, such as `ar` or `pt-BR`, is written
/// right to left. An unknown tag reads left to right.
pub fn is_right_to_left(tag: &str) -> bool {
    tag.parse::<LanguageIdentifier>()
        .is_ok_and(|language| direction_of(&language) == CharacterDirection::RTL)
}

/// The languages prev has text for.
pub fn available() -> Vec<LanguageIdentifier> {
    LOADER
        .available_languages(&Localizations)
        .unwrap_or_default()
}

/// An image format's name as the interface shows it: its own name, which
/// stays the same in every language, except where it has a common word.
pub fn format_name(format: prev_image::ImageFormat) -> String {
    match format {
        prev_image::ImageFormat::Raw => crate::fl!("format-camera-raw"),
        other => other.name().to_owned(),
    }
}

/// Errors from prev's libraries, worded in the user's language. Their
/// `Display` stays English for logs; details they carry from MuPDF or the
/// system stay as those give them.
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for prev_pdf::engine::Error {
    fn describe(&self) -> String {
        use prev_pdf::engine::Error;
        match self {
            Error::Open(detail) => crate::fl!("error-pdf-open", detail = detail.as_str()),
            Error::Engine(detail) | Error::Write(detail) => detail.clone(),
            Error::PageOutOfRange(index) => {
                let page = index + 1;
                crate::fl!("error-pdf-page-out-of-range", page = page)
            }
            Error::PasswordProtected => crate::fl!("error-pdf-password-protected"),
            Error::NoPages => crate::fl!("error-pdf-no-pages"),
            Error::CropOutsidePage => crate::fl!("error-pdf-crop-outside"),
            Error::Closed => crate::fl!("error-pdf-closed"),
            Error::SavedUnreadable => crate::fl!("error-pdf-saved-unreadable"),
        }
    }
}

impl Describe for prev_image::decode::DecodeError {
    fn describe(&self) -> String {
        use prev_image::decode::DecodeError;
        match self {
            DecodeError::Io(detail) => crate::fl!("error-image-read", detail = detail.as_str()),
            DecodeError::Invalid(detail) => {
                crate::fl!("error-image-invalid", detail = detail.as_str())
            }
            DecodeError::MissingLibrary(library) => {
                crate::fl!("error-image-missing-library", library = library.as_str())
            }
            DecodeError::Unsupported(format) => {
                crate::fl!("error-image-unsupported", format = format_name(*format))
            }
        }
    }
}

impl Describe for prev_image::encode::EncodeError {
    fn describe(&self) -> String {
        crate::fl!("error-image-encode", detail = self.0.as_str())
    }
}

impl Describe for prev_image::exif_edit::ExifError {
    fn describe(&self) -> String {
        crate::fl!("error-exif-malformed")
    }
}

impl Describe for prev_image::metadata::MetadataError {
    fn describe(&self) -> String {
        use prev_image::metadata::MetadataError;
        match self {
            MetadataError::RemoveLocation(error) => {
                crate::fl!("error-remove-location", error = error.describe())
            }
            MetadataError::LocationUnsupported => crate::fl!("error-location-unsupported"),
            MetadataError::XmpUnsupported => crate::fl!("error-xmp-unsupported"),
        }
    }
}

impl Describe for prev_store::settings::LoadError {
    fn describe(&self) -> String {
        use prev_store::settings::LoadError;
        match self {
            LoadError::Io(error) => {
                crate::fl!("error-settings-read", detail = error.to_string())
            }
            LoadError::Parse(error) => {
                crate::fl!("error-settings-invalid", detail = error.to_string())
            }
        }
    }
}

/// Text for `key` from `i18n/<language>/prev.ftl`, with Fluent arguments:
/// `fl!("page-count", count = pages)`. Checked against the English file at
/// compile time.
#[macro_export]
macro_rules! fl {
    ($key:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $key)
    }};
    ($key:literal, $($args:tt)*) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $key, $($args)*)
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::{Path, PathBuf};

    use super::*;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n")
    }

    /// Each message's key and the `{ $variables }` its text uses. Keys
    /// start a line; attributes (`.name =`) and continuation lines belong
    /// to the message above.
    fn messages(language: &str) -> BTreeMap<String, BTreeSet<String>> {
        let path = root().join(language).join("prev.ftl");
        let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("reading {path:?}"));
        let mut messages: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut current: Option<String> = None;
        for line in text.lines() {
            let starts_message =
                line.split_once('=')
                    .map(|(key, _)| key.trim_end())
                    .filter(|key| {
                        !line.starts_with([' ', '\t', '#', '.'])
                            && key.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                            && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
                    });
            if let Some(key) = starts_message {
                assert!(
                    !messages.contains_key(key),
                    "{language}: `{key}` is defined twice"
                );
                messages.insert(key.to_owned(), BTreeSet::new());
                current = Some(key.to_owned());
            } else if line.is_empty() || line.starts_with('#') {
                current = None;
            }
            if let Some(key) = &current {
                let variables = messages.get_mut(key).unwrap();
                for piece in line.split("$").skip(1) {
                    let name: String = piece
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        variables.insert(name);
                    }
                }
            }
        }
        messages
    }

    fn languages() -> Vec<String> {
        let mut languages: Vec<String> = std::fs::read_dir(root())
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().join("prev.ftl").is_file())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        languages.sort();
        languages
    }

    #[test]
    fn right_to_left_languages_are_known_by_tag() {
        for tag in ["ar", "fa", "he", "ur", "yi", "ps", "ar-EG"] {
            assert!(is_right_to_left(tag), "{tag}");
        }
        for tag in ["en", "ru", "zh", "ja", "pt-BR", "", "not a tag"] {
            assert!(!is_right_to_left(tag), "{tag}");
        }
    }

    #[test]
    fn every_language_loads() {
        for language in languages() {
            let id: LanguageIdentifier = language.parse().expect("folder names are language tags");
            let loader = fluent_language_loader!();
            loader
                .load_languages(&Localizations, &[id])
                .unwrap_or_else(|error| panic!("{language}: {error}"));
        }
    }

    /// Count messages render for every language, and pick the forms their
    /// plural rules give: in Russian, 1 and 21 differ (a count shown only
    /// from 2 up) and so do 2 and 5.
    #[test]
    fn counts_pick_their_plural_forms() {
        let render = |language: &str, key: &str, count: i64| {
            let loader = fluent_language_loader!();
            let id: LanguageIdentifier = language.parse().unwrap();
            loader.load_languages(&Localizations, &[id]).unwrap();
            loader.set_use_isolating(false);
            let mut args = std::collections::HashMap::new();
            args.insert("count", count);
            loader.get_args(key, args)
        };
        for language in languages() {
            for key in ["pages-copy", "pages-copied", "pages-delete", "pdf-page-of"] {
                for count in [0, 1, 2, 5, 21] {
                    let text = render(&language, key, count);
                    assert!(
                        !text.trim().is_empty() && !text.contains("No localization"),
                        "{language}: `{key}` with {count} gave {text:?}"
                    );
                }
            }
        }
        let russian = |count| render("ru", "pages-copied", count);
        assert_ne!(russian(1), russian(21));
        assert_ne!(russian(2), russian(5));
        assert_eq!(russian(2).replace('2', "3"), russian(3));
    }

    /// Translations may lack keys, which then show in English, but may not
    /// have keys English lacks (renamed or removed ones) or values English
    /// does not provide.
    #[test]
    fn translations_match_english() {
        let english = messages("en");
        assert!(!english.is_empty());
        for language in languages().into_iter().filter(|language| language != "en") {
            let translated = messages(&language);
            assert!(
                translated.contains_key("language-name"),
                "{language}: `language-name` names the language in the Settings list"
            );
            for (key, variables) in &translated {
                let Some(expected) = english.get(key) else {
                    panic!("{language}: `{key}` is not in the English file");
                };
                let unknown: Vec<_> = variables.difference(expected).collect();
                assert!(
                    unknown.is_empty(),
                    "{language}: `{key}` uses {unknown:?}, which English does not provide"
                );
            }
            let missing = english
                .keys()
                .filter(|key| !translated.contains_key(*key))
                .count();
            if missing > 0 {
                println!(
                    "{language}: {missing} of {} keys not translated yet",
                    english.len()
                );
            }
        }
    }
}
