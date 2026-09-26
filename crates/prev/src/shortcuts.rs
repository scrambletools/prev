//! Keyboard shortcuts. Preview's Command shortcuts map to Ctrl.

use iced::keyboard::{Key, Modifiers, key::Named};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Open,
    CloseWindow,
    Quit,
    Settings,
    ToggleFullscreen,
    ExitFullscreen,
}

pub fn lookup(key: &Key, modifiers: Modifiers) -> Option<Action> {
    let command = modifiers.command() && !modifiers.alt() && !modifiers.logo();
    match key.as_ref() {
        Key::Character(character) if command && !modifiers.shift() => {
            match character.to_lowercase().as_str() {
                "o" => Some(Action::Open),
                "w" => Some(Action::CloseWindow),
                "q" => Some(Action::Quit),
                "," => Some(Action::Settings),
                _ => None,
            }
        }
        Key::Character(character) if command && modifiers.shift() => {
            (character.eq_ignore_ascii_case("f")).then_some(Action::ToggleFullscreen)
        }
        Key::Named(Named::F11) if modifiers.is_empty() => Some(Action::ToggleFullscreen),
        Key::Named(Named::Escape) if modifiers.is_empty() => Some(Action::ExitFullscreen),
        _ => None,
    }
}

/// Shortcut labels for menus and help, in display order.
pub const LABELS: &[(Action, &str)] = &[
    (Action::Open, "Ctrl+O"),
    (Action::CloseWindow, "Ctrl+W"),
    (Action::Quit, "Ctrl+Q"),
    (Action::Settings, "Ctrl+,"),
    (Action::ToggleFullscreen, "F11 or Ctrl+Shift+F"),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn character(text: &str) -> Key {
        Key::Character(text.into())
    }

    #[test]
    fn command_shortcuts() {
        assert_eq!(lookup(&character("o"), Modifiers::CTRL), Some(Action::Open));
        assert_eq!(
            lookup(&character("W"), Modifiers::CTRL),
            Some(Action::CloseWindow)
        );
        assert_eq!(
            lookup(&character(","), Modifiers::CTRL),
            Some(Action::Settings)
        );
        assert_eq!(
            lookup(&character("F"), Modifiers::CTRL | Modifiers::SHIFT),
            Some(Action::ToggleFullscreen)
        );
    }

    #[test]
    fn plain_keys_and_extra_modifiers_do_nothing() {
        assert_eq!(lookup(&character("o"), Modifiers::empty()), None);
        assert_eq!(
            lookup(&character("o"), Modifiers::CTRL | Modifiers::ALT),
            None
        );
        assert_eq!(
            lookup(&character("q"), Modifiers::CTRL | Modifiers::LOGO),
            None
        );
        assert_eq!(
            lookup(&character("o"), Modifiers::CTRL | Modifiers::SHIFT),
            None
        );
    }

    #[test]
    fn named_keys() {
        assert_eq!(
            lookup(&Key::Named(Named::F11), Modifiers::empty()),
            Some(Action::ToggleFullscreen)
        );
        assert_eq!(
            lookup(&Key::Named(Named::Escape), Modifiers::empty()),
            Some(Action::ExitFullscreen)
        );
        assert_eq!(lookup(&Key::Named(Named::Escape), Modifiers::CTRL), None);
    }
}
