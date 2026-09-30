//! The macOS menu bar, laid out like Preview's: the app menu, then File,
//! Edit, View, Go, Tools and Window. Each item runs the same action as
//! its shortcut, and shows that shortcut.
//!
//! A menu item with a key equivalent takes the key before the window
//! sees it. Cut, Copy, Paste and Select All are therefore the system's
//! own items, which only take their keys when something answers
//! `copy:` and the like; prev's window does not, so the keys reach its
//! text fields, pages and images as before.

use std::cell::RefCell;
use std::sync::Mutex;

use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};

use crate::shortcuts::Action;

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
            .find(|(id, ..)| *id == event.id.as_ref())
            .map(|(_, action, ..)| *action)
        else {
            return;
        };
        if let Some(handler) = &*HANDLER.lock().unwrap_or_else(|e| e.into_inner()) {
            handler(action);
        }
    }));
}

const CMD: Modifiers = Modifiers::META;

/// Menu item id, action, key.
const ACTIONS: &[(&str, Action, Modifiers, Code)] = &[
    ("settings", Action::Settings, CMD, Code::Comma),
    ("quit", Action::Quit, CMD, Code::KeyQ),
    ("open", Action::Open, CMD, Code::KeyO),
    ("close", Action::CloseWindow, CMD, Code::KeyW),
    (
        "export",
        Action::Export,
        CMD.union(Modifiers::SHIFT),
        Code::KeyS,
    ),
    ("print", Action::Print, CMD, Code::KeyP),
    ("undo", Action::Undo, CMD, Code::KeyZ),
    (
        "redo",
        Action::Redo,
        CMD.union(Modifiers::SHIFT),
        Code::KeyZ,
    ),
    ("find", Action::Find, CMD, Code::KeyF),
    ("find-next", Action::FindNext, CMD, Code::KeyG),
    (
        "find-previous",
        Action::FindPrevious,
        CMD.union(Modifiers::SHIFT),
        Code::KeyG,
    ),
    (
        "hide-sidebar",
        Action::HideSidebar,
        CMD.union(Modifiers::ALT),
        Code::Digit1,
    ),
    (
        "thumbnails",
        Action::Thumbnails,
        CMD.union(Modifiers::ALT),
        Code::Digit2,
    ),
    (
        "contents",
        Action::Contents,
        CMD.union(Modifiers::ALT),
        Code::Digit3,
    ),
    (
        "notes",
        Action::NotesSidebar,
        CMD.union(Modifiers::ALT),
        Code::Digit4,
    ),
    (
        "bookmarks",
        Action::BookmarksSidebar,
        CMD.union(Modifiers::ALT),
        Code::Digit5,
    ),
    ("zoom-in", Action::ZoomIn, CMD, Code::Equal),
    ("zoom-out", Action::ZoomOut, CMD, Code::Minus),
    ("actual-size", Action::ActualSize, CMD, Code::Digit0),
    ("zoom-to-fit", Action::ZoomToFit, CMD, Code::Digit9),
    ("inspector", Action::Inspector, CMD, Code::KeyI),
    (
        "slideshow",
        Action::Slideshow,
        CMD.union(Modifiers::SHIFT),
        Code::KeyF,
    ),
    (
        "next-page",
        Action::NextPage,
        Modifiers::ALT,
        Code::ArrowDown,
    ),
    (
        "previous-page",
        Action::PreviousPage,
        Modifiers::ALT,
        Code::ArrowUp,
    ),
    (
        "go-to-page",
        Action::GoToPage,
        CMD.union(Modifiers::ALT),
        Code::KeyG,
    ),
    ("bookmark", Action::ToggleBookmark, CMD, Code::KeyD),
    (
        "markup",
        Action::ShowMarkup,
        CMD.union(Modifiers::SHIFT),
        Code::KeyA,
    ),
    ("rotate-left", Action::RotateLeft, CMD, Code::KeyL),
    ("rotate-right", Action::RotateRight, CMD, Code::KeyR),
    ("crop", Action::Crop, CMD, Code::KeyK),
    (
        "adjust-color",
        Action::AdjustColor,
        CMD.union(Modifiers::SHIFT),
        Code::KeyC,
    ),
];

/// The item for action `id`, with its label and key.
fn item(id: &str, label: String) -> MenuItem {
    let key = ACTIONS
        .iter()
        .find(|(candidate, ..)| *candidate == id)
        .map(|(_, _, modifiers, code)| Accelerator::new(*modifiers, *code));
    MenuItem::with_id(id, label, true, key)
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
    let edit = Submenu::with_items(
        fl!("menu-edit"),
        true,
        &[
            &item("undo", fl!("menu-undo")),
            &item("redo", fl!("menu-redo")),
            &separator(),
            &PredefinedMenuItem::cut(Some(&fl!("menu-cut"))),
            &PredefinedMenuItem::copy(Some(&fl!("menu-copy"))),
            &PredefinedMenuItem::paste(Some(&fl!("menu-paste"))),
            &PredefinedMenuItem::select_all(Some(&fl!("menu-select-all"))),
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
