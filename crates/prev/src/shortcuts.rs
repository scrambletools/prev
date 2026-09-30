//! Keyboard shortcuts: the defaults, where the command key is ⌘ on
//! macOS and Ctrl elsewhere and Option is Alt, changed by the `[keys]`
//! table of the settings file.

use iced::keyboard::{Key, Modifiers, key::Named};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Open,
    CloseWindow,
    Quit,
    Settings,
    Print,
    ToggleFullscreen,
    Escape,
    Slideshow,
    Copy,
    Paste,
    SelectAll,
    Find,
    FindNext,
    FindPrevious,
    ZoomIn,
    ZoomOut,
    ActualSize,
    ZoomToFit,
    HideSidebar,
    Thumbnails,
    Contents,
    NotesSidebar,
    BookmarksSidebar,
    ShowMarkup,
    ToggleBookmark,
    GoToPage,
    RotateLeft,
    RotateRight,
    Crop,
    Undo,
    Redo,
    AdjustColor,
    Inspector,
    Export,
    NextPage,
    PreviousPage,
    FirstPage,
    LastPage,
}

#[derive(Clone, Copy)]
enum Chord {
    Character(&'static str),
    Named(Named),
}

const NONE: u8 = 0;
const CTRL: u8 = 1;
const SHIFT: u8 = 2;
const ALT: u8 = 4;

/// The default shortcuts: (modifiers, key, action); modifiers must match
/// exactly. The first for each action is the one menus and hints show.
const TABLE: &[(u8, Chord, Action)] = &[
    (CTRL, Chord::Character("o"), Action::Open),
    (CTRL, Chord::Character("w"), Action::CloseWindow),
    (CTRL, Chord::Character("q"), Action::Quit),
    (CTRL, Chord::Character(","), Action::Settings),
    (CTRL, Chord::Character("p"), Action::Print),
    (NONE, Chord::Named(Named::F11), Action::ToggleFullscreen),
    (NONE, Chord::Named(Named::Escape), Action::Escape),
    (CTRL | SHIFT, Chord::Character("f"), Action::Slideshow),
    (CTRL, Chord::Character("c"), Action::Copy),
    (CTRL, Chord::Character("v"), Action::Paste),
    (CTRL, Chord::Character("a"), Action::SelectAll),
    (CTRL, Chord::Character("f"), Action::Find),
    (CTRL, Chord::Character("g"), Action::FindNext),
    (CTRL | SHIFT, Chord::Character("g"), Action::FindPrevious),
    (CTRL, Chord::Character("="), Action::ZoomIn),
    (CTRL, Chord::Character("+"), Action::ZoomIn),
    (CTRL | SHIFT, Chord::Character("+"), Action::ZoomIn),
    (CTRL | SHIFT, Chord::Character("="), Action::ZoomIn),
    (CTRL, Chord::Character("-"), Action::ZoomOut),
    (CTRL, Chord::Character("0"), Action::ActualSize),
    (CTRL, Chord::Character("9"), Action::ZoomToFit),
    (CTRL | ALT, Chord::Character("1"), Action::HideSidebar),
    (CTRL | ALT, Chord::Character("2"), Action::Thumbnails),
    (CTRL | ALT, Chord::Character("3"), Action::Contents),
    (CTRL | ALT, Chord::Character("4"), Action::NotesSidebar),
    (CTRL | ALT, Chord::Character("5"), Action::BookmarksSidebar),
    (CTRL | SHIFT, Chord::Character("a"), Action::ShowMarkup),
    (CTRL, Chord::Character("d"), Action::ToggleBookmark),
    (CTRL | ALT, Chord::Character("g"), Action::GoToPage),
    (CTRL, Chord::Character("l"), Action::RotateLeft),
    (CTRL, Chord::Character("r"), Action::RotateRight),
    (CTRL, Chord::Character("k"), Action::Crop),
    (CTRL, Chord::Character("z"), Action::Undo),
    (CTRL | SHIFT, Chord::Character("z"), Action::Redo),
    (CTRL | SHIFT, Chord::Character("c"), Action::AdjustColor),
    (CTRL, Chord::Character("i"), Action::Inspector),
    (CTRL | SHIFT, Chord::Character("s"), Action::Export),
    (ALT, Chord::Named(Named::ArrowDown), Action::NextPage),
    (ALT, Chord::Named(Named::ArrowUp), Action::PreviousPage),
    (NONE, Chord::Named(Named::Home), Action::FirstPage),
    (NONE, Chord::Named(Named::End), Action::LastPage),
];

/// Each action's name in the `[keys]` table of the settings file. Escape
/// closes and cancels, so it stays on Escape.
pub const NAMES: &[(Action, &str)] = &[
    (Action::Open, "open"),
    (Action::CloseWindow, "close-window"),
    (Action::Quit, "quit"),
    (Action::Settings, "settings"),
    (Action::Print, "print"),
    (Action::ToggleFullscreen, "full-screen"),
    (Action::Slideshow, "slideshow"),
    (Action::Copy, "copy"),
    (Action::Paste, "paste"),
    (Action::SelectAll, "select-all"),
    (Action::Find, "find"),
    (Action::FindNext, "find-next"),
    (Action::FindPrevious, "find-previous"),
    (Action::ZoomIn, "zoom-in"),
    (Action::ZoomOut, "zoom-out"),
    (Action::ActualSize, "actual-size"),
    (Action::ZoomToFit, "zoom-to-fit"),
    (Action::HideSidebar, "hide-sidebar"),
    (Action::Thumbnails, "thumbnails"),
    (Action::Contents, "contents"),
    (Action::NotesSidebar, "notes"),
    (Action::BookmarksSidebar, "bookmarks"),
    (Action::ShowMarkup, "markup"),
    (Action::ToggleBookmark, "bookmark"),
    (Action::GoToPage, "go-to-page"),
    (Action::RotateLeft, "rotate-left"),
    (Action::RotateRight, "rotate-right"),
    (Action::Crop, "crop"),
    (Action::Undo, "undo"),
    (Action::Redo, "redo"),
    (Action::AdjustColor, "adjust-color"),
    (Action::Inspector, "inspector"),
    (Action::Export, "export"),
    (Action::NextPage, "next-page"),
    (Action::PreviousPage, "previous-page"),
    (Action::FirstPage, "first-page"),
    (Action::LastPage, "last-page"),
];

/// The settings file's name for this system's own table in `[keys]`.
const SYSTEM: &str = if cfg!(target_os = "macos") {
    "macos"
} else if cfg!(windows) {
    "windows"
} else {
    "linux"
};
const SYSTEMS: [&str; 3] = ["linux", "windows", "macos"];

/// A key, with the modifiers that must be held with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    modifiers: u8,
    key: KeyName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum KeyName {
    /// Lowercase, as `lookup` compares it.
    Character(String),
    Named(Named),
}

impl Binding {
    /// Whether the command key (⌘ on macOS, Ctrl elsewhere), Shift and
    /// Alt (Option) are part of it.
    pub fn command(&self) -> bool {
        self.modifiers & CTRL != 0
    }

    pub fn shift(&self) -> bool {
        self.modifiers & SHIFT != 0
    }

    pub fn alt(&self) -> bool {
        self.modifiers & ALT != 0
    }

    /// The key's character, lowercase, when it is one.
    pub fn character(&self) -> Option<&str> {
        match &self.key {
            KeyName::Character(text) => Some(text),
            KeyName::Named(_) => None,
        }
    }

    pub fn named(&self) -> Option<Named> {
        match &self.key {
            KeyName::Named(named) => Some(*named),
            KeyName::Character(_) => None,
        }
    }

    fn matches(&self, key: &Key, bits: u8) -> bool {
        self.modifiers == bits
            && match (&self.key, key.as_ref()) {
                (KeyName::Character(expected), Key::Character(pressed)) => {
                    pressed.to_lowercase() == *expected
                }
                (KeyName::Named(expected), Key::Named(pressed)) => *expected == pressed,
                _ => false,
            }
    }

    /// Written as the settings file takes it, such as "Ctrl+Shift+E".
    fn written(&self) -> String {
        let mut text = String::new();
        for (bit, name) in [(CTRL, "Ctrl+"), (ALT, "Alt+"), (SHIFT, "Shift+")] {
            if self.modifiers & bit != 0 {
                text.push_str(name);
            }
        }
        match &self.key {
            KeyName::Character(character) => text.push_str(&character.to_uppercase()),
            KeyName::Named(named) => text.push_str(named_label(*named)),
        }
        text
    }

    /// Reads a shortcut such as "Ctrl+Shift+E", "Cmd+E", "Alt+Down" or
    /// "F3". Ctrl and Cmd both name the command key: ⌘ on macOS, Ctrl
    /// elsewhere.
    pub fn parse(text: &str) -> Result<Self, String> {
        let text = text.trim();
        // A trailing "+" is the key itself, as in "Ctrl++".
        let (rest, last) = match text.strip_suffix("++") {
            Some(rest) => (rest, "+"),
            None => match text.rsplit_once('+') {
                Some((rest, last)) => (rest, last),
                None => ("", text),
            },
        };
        let mut modifiers = NONE;
        for part in rest.split('+').filter(|part| !part.trim().is_empty()) {
            modifiers |= match part.trim().to_lowercase().as_str() {
                "ctrl" | "control" | "cmd" | "command" | "⌘" => CTRL,
                "shift" | "⇧" => SHIFT,
                "alt" | "option" | "opt" | "⌥" => ALT,
                other => return Err(format!("“{other}” is not a modifier")),
            };
        }
        let last = last.trim();
        if last.is_empty() {
            return Err("no key".into());
        }
        let key = match named_key(last) {
            Some(named) => KeyName::Named(named),
            None if last.chars().count() == 1 => KeyName::Character(last.to_lowercase()),
            None => return Err(format!("“{last}” is not a key")),
        };
        Ok(Self { modifiers, key })
    }
}

/// Key names the settings file takes, and how hints write them.
const NAMED_KEYS: &[(Named, &str, &[&str])] = &[
    (Named::Escape, "Esc", &["esc", "escape"]),
    (Named::Enter, "Enter", &["enter", "return"]),
    (Named::Tab, "Tab", &["tab"]),
    (Named::Space, "Space", &["space"]),
    (Named::Backspace, "Backspace", &["backspace"]),
    (Named::Delete, "Delete", &["delete", "del"]),
    (Named::Insert, "Insert", &["insert", "ins"]),
    (Named::Home, "Home", &["home"]),
    (Named::End, "End", &["end"]),
    (Named::PageUp, "Page Up", &["pageup", "page up", "pgup"]),
    (
        Named::PageDown,
        "Page Down",
        &["pagedown", "page down", "pgdn"],
    ),
    (Named::ArrowUp, "Up", &["up", "arrowup"]),
    (Named::ArrowDown, "Down", &["down", "arrowdown"]),
    (Named::ArrowLeft, "Left", &["left", "arrowleft"]),
    (Named::ArrowRight, "Right", &["right", "arrowright"]),
    (Named::F1, "F1", &["f1"]),
    (Named::F2, "F2", &["f2"]),
    (Named::F3, "F3", &["f3"]),
    (Named::F4, "F4", &["f4"]),
    (Named::F5, "F5", &["f5"]),
    (Named::F6, "F6", &["f6"]),
    (Named::F7, "F7", &["f7"]),
    (Named::F8, "F8", &["f8"]),
    (Named::F9, "F9", &["f9"]),
    (Named::F10, "F10", &["f10"]),
    (Named::F11, "F11", &["f11"]),
    (Named::F12, "F12", &["f12"]),
];

fn named_key(text: &str) -> Option<Named> {
    let text = text.to_lowercase();
    NAMED_KEYS
        .iter()
        .find(|(_, _, names)| names.contains(&text.as_str()))
        .map(|(named, ..)| *named)
}

fn named_label(named: Named) -> &'static str {
    NAMED_KEYS
        .iter()
        .find(|(candidate, ..)| *candidate == named)
        .map_or("?", |(_, label, _)| label)
}

/// The shortcuts in effect: the defaults with the settings' changes.
static KEYMAP: std::sync::LazyLock<std::sync::RwLock<Vec<(Binding, Action)>>> =
    std::sync::LazyLock::new(|| std::sync::RwLock::new(defaults()));

fn defaults() -> Vec<(Binding, Action)> {
    TABLE
        .iter()
        .map(|(modifiers, chord, action)| {
            let key = match chord {
                Chord::Character(text) => KeyName::Character((*text).to_owned()),
                Chord::Named(named) => KeyName::Named(*named),
            };
            (
                Binding {
                    modifiers: *modifiers,
                    key,
                },
                *action,
            )
        })
        .collect()
}

fn keymap() -> std::sync::RwLockReadGuard<'static, Vec<(Binding, Action)>> {
    KEYMAP.read().unwrap_or_else(|error| error.into_inner())
}

/// The shortcuts `keys`, the settings' `[keys]` table, sets on this
/// system, over the defaults. An action it names gets just those
/// shortcuts, and a shortcut it gives one action leaves any other. An
/// entry none of whose shortcuts can be read changes nothing. Returns what
/// could not be read, one line each; the rest still applies.
pub fn configure(keys: &toml::Table) -> Vec<String> {
    let (map, problems) = build(keys, SYSTEM);
    *KEYMAP.write().unwrap_or_else(|error| error.into_inner()) = map;
    problems
}

fn build(keys: &toml::Table, system: &str) -> (Vec<(Binding, Action)>, Vec<String>) {
    let mut problems = Vec::new();
    // This system's table wins over the shared entries.
    let mut chosen: Vec<(Action, Vec<Binding>)> = Vec::new();
    let mut read = |table: &toml::Table, prefix: &str, problems: &mut Vec<String>| {
        for (name, value) in table {
            if SYSTEMS.contains(&name.as_str()) && prefix.is_empty() {
                continue;
            }
            let Some(action) = NAMES
                .iter()
                .find(|(_, candidate)| candidate == name)
                .map(|(action, _)| *action)
            else {
                problems.push(format!("keys: {prefix}{name}: no such action"));
                continue;
            };
            let texts: Vec<&toml::Value> = match value {
                toml::Value::Array(items) => items.iter().collect(),
                other => vec![other],
            };
            let (mut bindings, mut unread) = (Vec::new(), false);
            for text in texts {
                match text.as_str() {
                    Some("") => {}
                    Some(text) => match Binding::parse(text) {
                        Ok(binding) => bindings.push(binding),
                        Err(error) => {
                            unread = true;
                            problems.push(format!("keys: {prefix}{name}: “{text}”: {error}"));
                        }
                    },
                    None => {
                        unread = true;
                        problems.push(format!(
                            "keys: {prefix}{name}: a shortcut is text, such as \"Ctrl+E\""
                        ));
                    }
                }
            }
            if unread && bindings.is_empty() {
                continue;
            }
            chosen.retain(|(candidate, _)| *candidate != action);
            chosen.push((action, bindings));
        }
    };
    read(keys, "", &mut problems);
    for system_name in SYSTEMS {
        match keys.get(system_name) {
            Some(toml::Value::Table(table)) if system_name == system => {
                read(table, &format!("{system_name}."), &mut problems)
            }
            Some(toml::Value::Table(_)) | None => {}
            Some(_) => problems.push(format!("keys: {system_name} should be a table")),
        }
    }
    let mut map = defaults();
    let taken: Vec<&Binding> = chosen.iter().flat_map(|(_, bindings)| bindings).collect();
    map.retain(|(binding, action)| {
        !chosen.iter().any(|(changed, _)| changed == action) && !taken.contains(&binding)
    });
    // Changed shortcuts come first, so they are found first.
    let mut changed: Vec<(Binding, Action)> = chosen
        .into_iter()
        .flat_map(|(action, bindings)| bindings.into_iter().map(move |binding| (binding, action)))
        .collect();
    changed.append(&mut map);
    (changed, problems)
}

fn modifier_bits(modifiers: Modifiers) -> Option<u8> {
    // The key that is not the command key (the logo key outside macOS,
    // Control on it) belongs to the system.
    let other = if cfg!(target_os = "macos") {
        modifiers.control()
    } else {
        modifiers.logo()
    };
    if other {
        return None;
    }
    let mut bits = NONE;
    if modifiers.command() {
        bits |= CTRL;
    }
    if modifiers.shift() {
        bits |= SHIFT;
    }
    if modifiers.alt() {
        bits |= ALT;
    }
    Some(bits)
}

pub fn lookup(key: &Key, modifiers: Modifiers) -> Option<Action> {
    let bits = modifier_bits(modifiers)?;
    keymap()
        .iter()
        .find(|(binding, _)| binding.matches(key, bits))
        .map(|(_, action)| *action)
}

/// The shortcut shown for `action`: its first one in effect, if any.
pub fn binding(action: Action) -> Option<Binding> {
    keymap()
        .iter()
        .find(|(_, bound)| *bound == action)
        .map(|(binding, _)| binding.clone())
}

/// The shortcut for `action` as the platform writes it: "Ctrl+Shift+S",
/// or "⇧⌘S" on macOS. `None` when no shortcut is set for it.
pub fn label(action: Action) -> Option<String> {
    binding(action).map(|binding| platform_label(&binding.written()))
}

/// A shortcut written as "Ctrl+Shift+Z", as the platform writes it: macOS
/// puts its symbols in the order Option, Shift, Command before the key,
/// with no plus signs ("⇧⌘Z").
fn platform_label(label: &str) -> String {
    if !cfg!(target_os = "macos") {
        return label.to_owned();
    }
    let (mut modifiers, mut key) = (String::new(), label);
    for (name, symbol) in [("Alt+", "⌥"), ("Shift+", "⇧")] {
        if key.contains(name) {
            modifiers.push_str(symbol);
        }
    }
    let command = key.starts_with("Ctrl+");
    for name in ["Ctrl+", "Alt+", "Shift+"] {
        key = key.strip_prefix(name).unwrap_or(key);
    }
    if command {
        modifiers.push('⌘');
    }
    format!("{modifiers}{key}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn character(text: &str) -> Key {
        Key::Character(text.into())
    }

    #[test]
    fn command_shortcuts() {
        assert_eq!(
            lookup(&character("o"), Modifiers::COMMAND),
            Some(Action::Open)
        );
        assert_eq!(
            lookup(&character("W"), Modifiers::COMMAND),
            Some(Action::CloseWindow)
        );
        assert_eq!(
            lookup(&character(","), Modifiers::COMMAND),
            Some(Action::Settings)
        );
        assert_eq!(
            lookup(&character("F"), Modifiers::COMMAND | Modifiers::SHIFT),
            Some(Action::Slideshow)
        );
        assert_eq!(
            lookup(&character("f"), Modifiers::COMMAND),
            Some(Action::Find)
        );
        assert_eq!(
            lookup(&character("G"), Modifiers::COMMAND | Modifiers::SHIFT),
            Some(Action::FindPrevious)
        );
        assert_eq!(
            lookup(&character("2"), Modifiers::COMMAND | Modifiers::ALT),
            Some(Action::Thumbnails)
        );
    }

    #[test]
    fn zoom_in_with_or_without_shift() {
        assert_eq!(
            lookup(&character("="), Modifiers::COMMAND),
            Some(Action::ZoomIn)
        );
        assert_eq!(
            lookup(&character("+"), Modifiers::COMMAND | Modifiers::SHIFT),
            Some(Action::ZoomIn)
        );
        assert_eq!(
            lookup(&character("-"), Modifiers::COMMAND),
            Some(Action::ZoomOut)
        );
    }

    #[test]
    fn modifiers_must_match_exactly() {
        assert_eq!(lookup(&character("o"), Modifiers::empty()), None);
        assert_eq!(
            lookup(&character("o"), Modifiers::COMMAND | Modifiers::ALT),
            None
        );
        // The system's other modifier key: the logo key, or Control on
        // macOS, where the logo key is the command key.
        let other = if cfg!(target_os = "macos") {
            Modifiers::CTRL
        } else {
            Modifiers::LOGO
        };
        assert_eq!(lookup(&character("q"), Modifiers::COMMAND | other), None);
        assert_eq!(
            lookup(&character("o"), Modifiers::COMMAND | Modifiers::SHIFT),
            None
        );
        assert_eq!(lookup(&Key::Named(Named::Escape), Modifiers::COMMAND), None);
    }

    #[test]
    fn named_keys() {
        assert_eq!(
            lookup(&Key::Named(Named::F11), Modifiers::empty()),
            Some(Action::ToggleFullscreen)
        );
        assert_eq!(
            lookup(&Key::Named(Named::Escape), Modifiers::empty()),
            Some(Action::Escape)
        );
        assert_eq!(
            lookup(&Key::Named(Named::ArrowDown), Modifiers::ALT),
            Some(Action::NextPage)
        );
        assert_eq!(
            lookup(&Key::Named(Named::Home), Modifiers::empty()),
            Some(Action::FirstPage)
        );
    }

    fn keys(text: &str) -> toml::Table {
        toml::from_str(text).unwrap()
    }

    fn bound(map: &[(Binding, Action)], action: Action) -> Vec<String> {
        map.iter()
            .filter(|(_, candidate)| *candidate == action)
            .map(|(binding, _)| binding.written())
            .collect()
    }

    #[test]
    fn every_action_has_a_name_but_escape() {
        for (_, _, action) in TABLE {
            assert!(
                *action == Action::Escape || NAMES.iter().any(|(named, _)| named == action),
                "{action:?}"
            );
        }
    }

    #[test]
    fn shortcuts_are_read() {
        let parse = |text| Binding::parse(text).map(|binding| binding.written());
        assert_eq!(parse("Ctrl+Shift+e").as_deref(), Ok("Ctrl+Shift+E"));
        assert_eq!(parse("cmd+alt+g").as_deref(), Ok("Ctrl+Alt+G"));
        assert_eq!(parse("⇧⌘+Z").ok(), None);
        assert_eq!(parse("Shift+⌘+Z").as_deref(), Ok("Ctrl+Shift+Z"));
        assert_eq!(parse("Ctrl++").as_deref(), Ok("Ctrl++"));
        assert_eq!(parse("Alt+Down").as_deref(), Ok("Alt+Down"));
        assert_eq!(parse("f3").as_deref(), Ok("F3"));
        assert_eq!(parse("Page Down").as_deref(), Ok("Page Down"));
        assert!(parse("Hyper+E").is_err());
        assert!(parse("Ctrl+Enterr").is_err());
        assert!(parse("Ctrl+").is_err());
    }

    #[test]
    fn settings_replace_an_actions_shortcuts() {
        let (map, problems) = build(
            &keys("export = \"Ctrl+E\"\nfind-next = [\"Ctrl+G\", \"F3\"]\ncrop = []"),
            "linux",
        );
        assert!(problems.is_empty(), "{problems:?}");
        assert_eq!(bound(&map, Action::Export), ["Ctrl+E"]);
        assert_eq!(bound(&map, Action::FindNext), ["Ctrl+G", "F3"]);
        assert!(bound(&map, Action::Crop).is_empty());
        assert_eq!(bound(&map, Action::Open), ["Ctrl+O"]);
    }

    #[test]
    fn a_taken_shortcut_leaves_its_old_action() {
        // Ctrl+I opens the inspector by default.
        let (map, _) = build(&keys("export = \"Ctrl+I\""), "linux");
        assert!(bound(&map, Action::Inspector).is_empty());
        assert_eq!(bound(&map, Action::Export), ["Ctrl+I"]);
    }

    #[test]
    fn a_systems_table_applies_only_there() {
        let table = keys(
            "export = \"Ctrl+E\"\n[macos]\nexport = \"Ctrl+Shift+E\"\n[windows]\nprint = \"F4\"",
        );
        let (mac, _) = build(&table, "macos");
        assert_eq!(bound(&mac, Action::Export), ["Ctrl+Shift+E"]);
        assert_eq!(bound(&mac, Action::Print), ["Ctrl+P"]);
        let (linux, problems) = build(&table, "linux");
        assert!(problems.is_empty(), "{problems:?}");
        assert_eq!(bound(&linux, Action::Export), ["Ctrl+E"]);
        assert_eq!(bound(&linux, Action::Print), ["Ctrl+P"]);
        let (windows, _) = build(&table, "windows");
        assert_eq!(bound(&windows, Action::Print), ["F4"]);
    }

    #[test]
    fn problems_are_reported_and_the_rest_applies() {
        let (map, problems) = build(
            &keys("teleport = \"Ctrl+T\"\nexport = \"Hyper+E\"\nprint = 4\ncrop = \"Ctrl+J\""),
            "linux",
        );
        assert_eq!(problems.len(), 3, "{problems:?}");
        assert_eq!(bound(&map, Action::Crop), ["Ctrl+J"]);
        // Nothing readable, so the defaults stay.
        assert_eq!(bound(&map, Action::Export), ["Ctrl+Shift+S"]);
        assert_eq!(bound(&map, Action::Print), ["Ctrl+P"]);
    }
}
