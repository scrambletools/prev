//! Keyboard shortcuts. Preview's Command shortcuts map to Ctrl, Option to Alt.

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
    BookmarksSidebar,
    ToggleBookmark,
    GoToPage,
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

/// (modifiers, key, action); modifiers must match exactly.
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
    (CTRL | ALT, Chord::Character("5"), Action::BookmarksSidebar),
    (CTRL, Chord::Character("d"), Action::ToggleBookmark),
    (CTRL | ALT, Chord::Character("g"), Action::GoToPage),
    (ALT, Chord::Named(Named::ArrowDown), Action::NextPage),
    (ALT, Chord::Named(Named::ArrowUp), Action::PreviousPage),
    (NONE, Chord::Named(Named::Home), Action::FirstPage),
    (NONE, Chord::Named(Named::End), Action::LastPage),
];

fn modifier_bits(modifiers: Modifiers) -> Option<u8> {
    if modifiers.logo() {
        return None;
    }
    let mut bits = NONE;
    if modifiers.control() {
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
    TABLE.iter().find_map(|(required, chord, action)| {
        let matches = match (chord, key.as_ref()) {
            (Chord::Character(expected), Key::Character(pressed)) => {
                pressed.to_lowercase() == *expected
            }
            (Chord::Named(expected), Key::Named(pressed)) => *expected == pressed,
            _ => false,
        };
        (matches && *required == bits).then_some(*action)
    })
}

/// Shortcut labels for menus and help, in display order.
pub const LABELS: &[(Action, &str)] = &[
    (Action::Open, "Ctrl+O"),
    (Action::CloseWindow, "Ctrl+W"),
    (Action::Quit, "Ctrl+Q"),
    (Action::Settings, "Ctrl+,"),
    (Action::Print, "Ctrl+P"),
    (Action::ToggleFullscreen, "F11"),
    (Action::Slideshow, "Ctrl+Shift+F"),
    (Action::Find, "Ctrl+F"),
    (Action::ZoomIn, "Ctrl++"),
    (Action::ZoomOut, "Ctrl+-"),
    (Action::ActualSize, "Ctrl+0"),
    (Action::ZoomToFit, "Ctrl+9"),
    (Action::GoToPage, "Ctrl+Alt+G"),
    (Action::ToggleBookmark, "Ctrl+D"),
];

pub fn label(action: Action) -> Option<&'static str> {
    LABELS
        .iter()
        .find(|(candidate, _)| *candidate == action)
        .map(|(_, label)| *label)
}

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
            Some(Action::Slideshow)
        );
        assert_eq!(lookup(&character("f"), Modifiers::CTRL), Some(Action::Find));
        assert_eq!(
            lookup(&character("G"), Modifiers::CTRL | Modifiers::SHIFT),
            Some(Action::FindPrevious)
        );
        assert_eq!(
            lookup(&character("2"), Modifiers::CTRL | Modifiers::ALT),
            Some(Action::Thumbnails)
        );
    }

    #[test]
    fn zoom_in_with_or_without_shift() {
        assert_eq!(
            lookup(&character("="), Modifiers::CTRL),
            Some(Action::ZoomIn)
        );
        assert_eq!(
            lookup(&character("+"), Modifiers::CTRL | Modifiers::SHIFT),
            Some(Action::ZoomIn)
        );
        assert_eq!(
            lookup(&character("-"), Modifiers::CTRL),
            Some(Action::ZoomOut)
        );
    }

    #[test]
    fn modifiers_must_match_exactly() {
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
        assert_eq!(lookup(&Key::Named(Named::Escape), Modifiers::CTRL), None);
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

    #[test]
    fn every_label_names_a_bound_action() {
        for (action, _) in LABELS {
            assert!(
                TABLE.iter().any(|(_, _, bound)| bound == action),
                "{action:?}"
            );
        }
    }
}
