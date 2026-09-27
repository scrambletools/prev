mod app;

use std::path::PathBuf;
use std::sync::Mutex;

use iced::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use prev::instance::{self, Role};
use prev::{omarchy, ui};

/// Events from background threads, delivered through a subscription.
#[derive(Debug, Clone)]
pub enum External {
    OpenPaths(Vec<PathBuf>),
    OmarchyThemeChanged,
    Drag(smithay_clipboard::dnd::DragEvent),
}

static EXTERNAL_EVENTS: Mutex<Option<UnboundedReceiver<External>>> = Mutex::new(None);

const USAGE: &str = "\
Usage: prev [FILE]...

View and edit PDFs and images. Files open in windows of the running prev,
which starts if needed.

Options:
  -h, --help     Show this help
  -V, --version  Show the version
";

fn main() -> iced::Result {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut options_done = false;
    for argument in std::env::args_os().skip(1) {
        if !options_done {
            match argument.to_str() {
                Some("-h" | "--help") => {
                    print!("{USAGE}");
                    return Ok(());
                }
                Some("-V" | "--version") => {
                    let build = if prev_store::paths::PRODUCTION {
                        ""
                    } else {
                        " (development build)"
                    };
                    println!("prev {}{build}", env!("CARGO_PKG_VERSION"));
                    return Ok(());
                }
                Some("--") => {
                    options_done = true;
                    continue;
                }
                Some(option) if option.starts_with('-') && option.len() > 1 => {
                    eprintln!("prev: unknown option {option}\n\n{USAGE}");
                    std::process::exit(2);
                }
                _ => {}
            }
        }
        paths.push(PathBuf::from(argument));
    }
    let (sender, receiver) = mpsc::unbounded();

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

    let omarchy_dir = omarchy::current_theme_dir();
    if let Some(dir) = omarchy_dir.clone() {
        let sender = sender.clone();
        omarchy::watch(dir, move || send(&sender, External::OmarchyThemeChanged));
    }
    {
        let sender = sender.clone();
        smithay_clipboard::dnd::set_accepted_mimes(prev::drag::accepted_types());
        smithay_clipboard::dnd::set_drag_handler(move |event| send(&sender, External::Drag(event)));
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

fn send(sender: &UnboundedSender<External>, event: External) {
    let _ = sender.unbounded_send(event);
}

/// Hands the receiver to the subscription once.
fn external_events() -> impl iced::futures::Stream<Item = External> {
    let receiver = EXTERNAL_EVENTS.lock().unwrap().take();
    iced::futures::StreamExt::flatten(iced::futures::stream::iter(receiver))
}
