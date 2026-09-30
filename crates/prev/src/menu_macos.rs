//! The macOS menu bar, laid out like Preview's: the app menu, then File,
//! Edit, View, Go, Tools and Window. Each item runs the same action as
//! its shortcut, and shows that shortcut.
//!
//! A menu item with a key equivalent takes the key before the window
//! sees it. Cut, Copy, Paste and Select All are therefore the system's
//! own items, which only take their keys when something answers
//! `copy:` and the like; prev's window does not, so the keys reach its
//! text fields, pages and images as before. Items show the shortcuts in
//! effect, the settings' `[keys]` included.

use std::cell::RefCell;
use std::sync::Mutex;

use iced::keyboard::key::Named;
use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{AboutMetadata, IsMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};

use crate::shortcuts::{self, Action};

thread_local! {
    /// The menu bar in use; AppKit only borrows it.
    static MENU: RefCell<Option<Menu>> = const { RefCell::new(None) };
}

type Handler = Box<dyn Fn(Action) + Send>;
static HANDLER: Mutex<Option<Handler>> = Mutex::new(None);

/// Calls `handler` with the action of each menu item chosen.
pub fn set_handler(handler: impl Fn(Action) + Send + 'static) {
    *HANDLER.lock().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(handler));
    MenuEvent::set_event_handler(Some(|event: MenuEvent| {
        let Some(action) = ACTIONS
            .iter()
            .find(|(id, _)| *id == event.id.as_ref())
            .map(|(_, action)| *action)
        else {
            return;
        };
        if let Some(handler) = &*HANDLER.lock().unwrap_or_else(|e| e.into_inner()) {
            handler(action);
        }
    }));
}

/// Menu item id and action. Each item shows the action's shortcut in
/// effect, from the settings' `[keys]` or the defaults.
const ACTIONS: &[(&str, Action)] = &[
    ("settings", Action::Settings),
    ("quit", Action::Quit),
    ("open", Action::Open),
    ("close", Action::CloseWindow),
    ("export", Action::Export),
    ("print", Action::Print),
    ("undo", Action::Undo),
    ("redo", Action::Redo),
    ("copy", Action::Copy),
    ("paste", Action::Paste),
    ("select-all", Action::SelectAll),
    ("find", Action::Find),
    ("find-next", Action::FindNext),
    ("find-previous", Action::FindPrevious),
    ("hide-sidebar", Action::HideSidebar),
    ("thumbnails", Action::Thumbnails),
    ("contents", Action::Contents),
    ("notes", Action::NotesSidebar),
    ("bookmarks", Action::BookmarksSidebar),
    ("zoom-in", Action::ZoomIn),
    ("zoom-out", Action::ZoomOut),
    ("actual-size", Action::ActualSize),
    ("zoom-to-fit", Action::ZoomToFit),
    ("inspector", Action::Inspector),
    ("slideshow", Action::Slideshow),
    ("next-page", Action::NextPage),
    ("previous-page", Action::PreviousPage),
    ("go-to-page", Action::GoToPage),
    ("bookmark", Action::ToggleBookmark),
    ("markup", Action::ShowMarkup),
    ("rotate-left", Action::RotateLeft),
    ("rotate-right", Action::RotateRight),
    ("crop", Action::Crop),
    ("adjust-color", Action::AdjustColor),
];

/// The key equivalent for `action`'s shortcut, when it has one a menu can
/// show.
fn accelerator(action: Action) -> Option<Accelerator> {
    let binding = shortcuts::binding(action)?;
    let mut modifiers = Modifiers::empty();
    if binding.command() {
        modifiers |= Modifiers::META;
    }
    if binding.shift() {
        modifiers |= Modifiers::SHIFT;
    }
    if binding.alt() {
        modifiers |= Modifiers::ALT;
    }
    let code = match (binding.character(), binding.named()) {
        (Some(character), _) => character_code(character)?,
        (None, Some(named)) => named_code(named)?,
        (None, None) => return None,
    };
    Some(Accelerator::new(modifiers, code))
}

fn character_code(character: &str) -> Option<Code> {
    let mut chars = character.chars();
    let (Some(character), None) = (chars.next(), chars.next()) else {
        return None;
    };
    const LETTERS: [Code; 26] = [
        Code::KeyA,
        Code::KeyB,
        Code::KeyC,
        Code::KeyD,
        Code::KeyE,
        Code::KeyF,
        Code::KeyG,
        Code::KeyH,
        Code::KeyI,
        Code::KeyJ,
        Code::KeyK,
        Code::KeyL,
        Code::KeyM,
        Code::KeyN,
        Code::KeyO,
        Code::KeyP,
        Code::KeyQ,
        Code::KeyR,
        Code::KeyS,
        Code::KeyT,
        Code::KeyU,
        Code::KeyV,
        Code::KeyW,
        Code::KeyX,
        Code::KeyY,
        Code::KeyZ,
    ];
    const DIGITS: [Code; 10] = [
        Code::Digit0,
        Code::Digit1,
        Code::Digit2,
        Code::Digit3,
        Code::Digit4,
        Code::Digit5,
        Code::Digit6,
        Code::Digit7,
        Code::Digit8,
        Code::Digit9,
    ];
    Some(match character {
        'a'..='z' => LETTERS[(character as u8 - b'a') as usize],
        '0'..='9' => DIGITS[(character as u8 - b'0') as usize],
        ',' => Code::Comma,
        '.' => Code::Period,
        '-' => Code::Minus,
        '=' => Code::Equal,
        '/' => Code::Slash,
        ';' => Code::Semicolon,
        '\'' => Code::Quote,
        '[' => Code::BracketLeft,
        ']' => Code::BracketRight,
        '\\' => Code::Backslash,
        '`' => Code::Backquote,
        ' ' => Code::Space,
        _ => return None,
    })
}

fn named_code(named: Named) -> Option<Code> {
    Some(match named {
        Named::ArrowUp => Code::ArrowUp,
        Named::ArrowDown => Code::ArrowDown,
        Named::ArrowLeft => Code::ArrowLeft,
        Named::ArrowRight => Code::ArrowRight,
        Named::Home => Code::Home,
        Named::End => Code::End,
        Named::PageUp => Code::PageUp,
        Named::PageDown => Code::PageDown,
        Named::Enter => Code::Enter,
        Named::Tab => Code::Tab,
        Named::Space => Code::Space,
        Named::Escape => Code::Escape,
        Named::Backspace => Code::Backspace,
        Named::Delete => Code::Delete,
        Named::Insert => Code::Insert,
        Named::F1 => Code::F1,
        Named::F2 => Code::F2,
        Named::F3 => Code::F3,
        Named::F4 => Code::F4,
        Named::F5 => Code::F5,
        Named::F6 => Code::F6,
        Named::F7 => Code::F7,
        Named::F8 => Code::F8,
        Named::F9 => Code::F9,
        Named::F10 => Code::F10,
        Named::F11 => Code::F11,
        Named::F12 => Code::F12,
        _ => return None,
    })
}

/// The item for action `id`, with its label and shortcut.
fn item(id: &str, label: String) -> MenuItem {
    let key = ACTIONS
        .iter()
        .find(|(candidate, _)| *candidate == id)
        .and_then(|(_, action)| accelerator(*action));
    MenuItem::with_id(id, label, true, key)
}

/// Copy, Paste and Select All: the system's own items while they keep
/// their standard shortcuts, so text fields still get those keys; prev's,
/// with the shortcut the settings give, when that changed.
fn standard(
    id: &str,
    label: String,
    system: fn(Option<&str>) -> PredefinedMenuItem,
    standard: Code,
) -> Box<dyn IsMenuItem> {
    let action = ACTIONS
        .iter()
        .find(|(candidate, _)| *candidate == id)
        .map(|(_, action)| *action);
    let unchanged = action
        .and_then(accelerator)
        .is_some_and(|key| key == Accelerator::new(Modifiers::META, standard));
    if unchanged {
        Box::new(system(Some(&label)))
    } else {
        Box::new(item(id, label))
    }
}

/// Builds the menu bar in the interface's language and puts it up.
/// Called at start and when the language changes; on the main thread.
pub fn install() {
    match build() {
        Ok(menu) => {
            menu.init_for_nsapp();
            MENU.with(|slot| *slot.borrow_mut() = Some(menu));
        }
        Err(error) => eprintln!("prev: no menu bar: {error}"),
    }
}

fn build() -> muda::Result<Menu> {
    use crate::fl;
    let separator = PredefinedMenuItem::separator;
    let about = AboutMetadata {
        name: Some("prev".into()),
        version: Some(env!("CARGO_PKG_VERSION").into()),
        license: Some("AGPL-3.0-or-later".into()),
        website: Some("https://prev.run".into()),
        ..AboutMetadata::default()
    };
    let app = Submenu::with_items(
        "prev",
        true,
        &[
            &PredefinedMenuItem::about(Some(&fl!("menu-about")), Some(about)),
            &separator(),
            &item("settings", fl!("menu-settings")),
            &separator(),
            &PredefinedMenuItem::services(Some(&fl!("menu-services"))),
            &separator(),
            &PredefinedMenuItem::hide(Some(&fl!("menu-hide"))),
            &PredefinedMenuItem::hide_others(Some(&fl!("menu-hide-others"))),
            &PredefinedMenuItem::show_all(Some(&fl!("menu-show-all"))),
            &separator(),
            &item("quit", fl!("menu-quit")),
        ],
    )?;
    let file = Submenu::with_items(
        fl!("menu-file"),
        true,
        &[
            &item("open", fl!("menu-open")),
            &item("close", fl!("menu-close")),
            &separator(),
            &item("export", fl!("menu-export")),
            &separator(),
            &item("print", fl!("menu-print")),
        ],
    )?;
    let copy = standard(
        "copy",
        fl!("menu-copy"),
        PredefinedMenuItem::copy,
        Code::KeyC,
    );
    let paste = standard(
        "paste",
        fl!("menu-paste"),
        PredefinedMenuItem::paste,
        Code::KeyV,
    );
    let select_all = standard(
        "select-all",
        fl!("menu-select-all"),
        PredefinedMenuItem::select_all,
        Code::KeyA,
    );
    let edit = Submenu::with_items(
        fl!("menu-edit"),
        true,
        &[
            &item("undo", fl!("menu-undo")),
            &item("redo", fl!("menu-redo")),
            &separator(),
            &PredefinedMenuItem::cut(Some(&fl!("menu-cut"))),
            copy.as_ref(),
            paste.as_ref(),
            select_all.as_ref(),
            &separator(),
            &item("find", fl!("menu-find")),
            &item("find-next", fl!("menu-find-next")),
            &item("find-previous", fl!("menu-find-previous")),
        ],
    )?;
    let view = Submenu::with_items(
        fl!("menu-view"),
        true,
        &[
            &item("hide-sidebar", fl!("menu-hide-sidebar")),
            &item("thumbnails", fl!("menu-thumbnails")),
            &item("contents", fl!("menu-contents")),
            &item("notes", fl!("menu-notes")),
            &item("bookmarks", fl!("menu-bookmarks")),
            &separator(),
            &item("zoom-in", fl!("menu-zoom-in")),
            &item("zoom-out", fl!("menu-zoom-out")),
            &item("actual-size", fl!("menu-actual-size")),
            &item("zoom-to-fit", fl!("menu-zoom-to-fit")),
            &separator(),
            &item("inspector", fl!("menu-inspector")),
            &item("slideshow", fl!("menu-slideshow")),
            &PredefinedMenuItem::fullscreen(Some(&fl!("menu-full-screen"))),
        ],
    )?;
    let go = Submenu::with_items(
        fl!("menu-go"),
        true,
        &[
            &item("next-page", fl!("menu-next-page")),
            &item("previous-page", fl!("menu-previous-page")),
            &item("go-to-page", fl!("menu-go-to-page")),
            &separator(),
            &item("bookmark", fl!("menu-bookmark")),
        ],
    )?;
    let tools = Submenu::with_items(
        fl!("menu-tools"),
        true,
        &[
            &item("markup", fl!("menu-markup")),
            &separator(),
            &item("rotate-left", fl!("menu-rotate-left")),
            &item("rotate-right", fl!("menu-rotate-right")),
            &item("crop", fl!("menu-crop")),
            &item("adjust-color", fl!("menu-adjust-color")),
        ],
    )?;
    let window = Submenu::with_items(
        fl!("menu-window"),
        true,
        &[
            &PredefinedMenuItem::minimize(Some(&fl!("menu-minimize"))),
            &PredefinedMenuItem::maximize(Some(&fl!("menu-zoom"))),
            &separator(),
            &PredefinedMenuItem::bring_all_to_front(Some(&fl!("menu-bring-all-to-front"))),
        ],
    )?;
    let menu = Menu::new();
    for submenu in [&app, &file, &edit, &view, &go, &tools, &window] {
        menu.append(submenu)?;
    }
    window.set_as_windows_menu_for_nsapp();
    Ok(menu)
}
