// Release builds on Windows open no console window; see attach_console.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;

use std::path::PathBuf;
use std::sync::Mutex;

use iced::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
#[cfg(any(unix, windows))]
use prev::instance::{self, Role};
use prev::{omarchy, ui};

/// Events from background threads, delivered through a subscription.
#[derive(Debug, Clone)]
pub enum External {
    OpenPaths(Vec<PathBuf>),
    /// A menu bar item was chosen.
    #[cfg(target_os = "macos")]
    Menu(prev::shortcuts::Action),
    OmarchyThemeChanged,
    Drag(prev::dnd::DragEvent),
}

static EXTERNAL_EVENTS: Mutex<Option<UnboundedReceiver<External>>> = Mutex::new(None);

fn main() -> iced::Result {
    #[cfg(windows)]
    attach_console();
    // The input language Windows keeps for this thread, where iced runs.
    #[cfg(windows)]
    prev::input::remember_interface_thread();
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut options_done = false;
    for argument in std::env::args_os().skip(1) {
        if !options_done {
            match argument.to_str() {
                Some("-h" | "--help") => {
                    println!("{}", prev::fl!("usage-help"));
                    return Ok(());
                }
                Some("-V" | "--version") => {
                    let build = if prev_store::paths::PRODUCTION {
                        concat!(" (", env!("PREV_COMMIT"), ")")
                    } else {
                        concat!(
                            " (dev build, ",
                            env!("PREV_COMMIT"),
                            ", ",
                            env!("PREV_BUILT"),
                            " UTC)"
                        )
                    };
                    println!("prev {}{build}", env!("CARGO_PKG_VERSION"));
                    return Ok(());
                }
                Some("--") => {
                    options_done = true;
                    continue;
                }
                Some(option) if option.starts_with('-') && option.len() > 1 => {
                    eprintln!(
                        "prev: unknown option {option}\n\n{}\n",
                        prev::fl!("usage-help")
                    );
                    std::process::exit(2);
                }
                _ => {}
            }
        }
        paths.push(PathBuf::from(argument));
    }
    let (sender, receiver) = mpsc::unbounded();

    #[cfg(unix)]
    if let Some(socket) =
        prev_store::paths::runtime_dir().map(|dir| dir.join(instance::SOCKET_NAME))
    {
        match instance::claim_or_forward(&socket, &paths) {
            Ok(Role::Forwarded) => return Ok(()),
            Ok(Role::Primary(listener)) => {
                let sender = sender.clone();
                listener.spawn(move |paths| send(&sender, External::OpenPaths(paths)));
            }
            Err(error) => eprintln!("prev: running without single instance: {error}"),
        }
    }

    #[cfg(windows)]
    match instance::claim_or_forward(&instance::pipe_name(), &paths) {
        Ok(Role::Forwarded) => return Ok(()),
        Ok(Role::Primary(listener)) => {
            let sender = sender.clone();
            listener.spawn(move |paths| send(&sender, External::OpenPaths(paths)));
        }
        Err(error) => eprintln!("prev: running without single instance: {error}"),
    }

    // Files Finder and the Dock hand the app, which macOS sends to the
    // running prev instead of starting another.
    #[cfg(target_os = "macos")]
    {
        let files = sender.clone();
        winit::platform::open_files::set_handler(move |paths| {
            send(&files, External::OpenPaths(paths))
        });
        let menu = sender.clone();
        prev::menu_macos::set_handler(move |action| send(&menu, External::Menu(action)));
    }

    let omarchy_dir = omarchy::current_theme_dir();
    if let Some(dir) = omarchy_dir.clone() {
        let sender = sender.clone();
        omarchy::watch(dir, move || send(&sender, External::OmarchyThemeChanged));
    }
    {
        let sender = sender.clone();
        prev::dnd::set_accepted_mimes(prev::drag::accepted_types());
        prev::dnd::set_drag_handler(move |event| send(&sender, External::Drag(event)));
    }
    *EXTERNAL_EVENTS.lock().unwrap() = Some(receiver);

    iced::daemon(
        move || app::Prev::boot(paths.clone(), omarchy_dir.clone()),
        app::Prev::update,
        app::Prev::view,
    )
    .settings(iced::Settings {
        fonts: ui::font::files().collect(),
        default_font: ui::font::TEXT,
        default_text_size: ui::font::DEFAULT_SIZE.into(),
        antialiasing: true,
        ..iced::Settings::default()
    })
    .title(app::Prev::title)
    .theme(app::Prev::theme)
    .subscription(app::Prev::subscription)
    .run()
}

/// Writes `--help` and `--version` output to the terminal prev was started
/// from: an app without a console window has nowhere else to print.
#[cfg(windows)]
#[allow(unsafe_code)]
fn attach_console() {
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
    // SAFETY: takes no pointers; fails harmlessly when there is no parent
    // console or one is already attached.
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

fn send(sender: &UnboundedSender<External>, event: External) {
    let _ = sender.unbounded_send(event);
}

/// Hands the receiver to the subscription once.
fn external_events() -> impl iced::futures::Stream<Item = External> {
    let receiver = EXTERNAL_EVENTS.lock().unwrap().take();
    iced::futures::StreamExt::flatten(iced::futures::stream::iter(receiver))
}
