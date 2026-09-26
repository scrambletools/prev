//! The iced application: one window per document, plus start and settings
//! windows.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{button, center, checkbox, column, container, radio, rule, text};
use iced::window::{self, settings::PlatformSpecific};
use iced::{Border, Center, Color, Element, Event, Fill, Size, Subscription, Task, Theme, event};
use prev::filetype::{self, FileKind};
use prev::shortcuts::{self, Action};
use prev::{dialog, omarchy};
use prev_store::settings::{self, Appearance, Settings};
use raw_window_handle::RawWindowHandle;
use smithay_clipboard::dnd::DragEvent;

use crate::External;

pub const APP_ID: &str = "io.github.scrambletools.prev";

pub struct Prev {
    windows: BTreeMap<window::Id, Window>,
    settings: Settings,
    settings_path: Option<PathBuf>,
    settings_error: Option<String>,
    omarchy_dir: Option<PathBuf>,
    omarchy: Option<(omarchy::Mode, Theme)>,
}

struct Window {
    content: Content,
    fullscreen: bool,
    notice: Option<String>,
    /// The `wl_surface` pointer, used to match drag events to windows.
    surface: Option<usize>,
    drag_hover: bool,
}

enum Content {
    Start,
    Document(Document),
    Settings,
}

struct Document {
    path: PathBuf,
    kind: Result<Option<FileKind>, String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    External(External),
    WindowOpened(window::Id),
    SurfaceKnown(window::Id, Option<usize>),
    WindowClosed(window::Id),
    Key(window::Id, Key, Modifiers),
    Perform(window::Id, Action),
    DialogFinished(window::Id, Result<Vec<PathBuf>, String>),
    AppearanceSelected(Appearance),
    OmarchyPaletteToggled(bool),
}

impl Prev {
    pub fn boot(paths: Vec<PathBuf>, omarchy_dir: Option<PathBuf>) -> (Self, Task<Message>) {
        let settings_path = settings::default_path();
        let (settings, settings_error) = match settings_path.as_deref().map(Settings::load_from) {
            Some(Ok(settings)) => (settings, None),
            Some(Err(error)) => (Settings::default(), Some(error.to_string())),
            None => (Settings::default(), None),
        };
        let mut prev = Self {
            windows: BTreeMap::new(),
            settings,
            settings_path,
            settings_error,
            omarchy_dir,
            omarchy: None,
        };
        prev.reload_omarchy();
        let task = prev.open_paths(paths);
        (prev, task)
    }

    fn reload_omarchy(&mut self) {
        self.omarchy = self
            .omarchy_dir
            .as_deref()
            .and_then(omarchy::load)
            .map(|palette| {
                let theme = Theme::custom(palette.name.clone(), iced_palette(&palette));
                (palette.mode, theme)
            });
    }

    /// Opens a window per path; an empty list opens a start window.
    fn open_paths(&mut self, paths: Vec<PathBuf>) -> Task<Message> {
        if paths.is_empty() {
            return self.open_window(Content::Start);
        }
        let start_windows: Vec<window::Id> = self
            .windows
            .iter()
            .filter(|(_, window)| matches!(window.content, Content::Start))
            .map(|(id, _)| *id)
            .collect();
        let mut tasks: Vec<Task<Message>> = paths
            .into_iter()
            .map(|path| self.open_document(path))
            .collect();
        tasks.extend(start_windows.into_iter().map(window::close));
        Task::batch(tasks)
    }

    fn open_document(&mut self, path: PathBuf) -> Task<Message> {
        let path = std::fs::canonicalize(&path).unwrap_or(path);
        if let Some(id) = self.window_showing(&path) {
            return window::gain_focus(id);
        }
        let kind = filetype::detect_path(&path).map_err(|error| error.to_string());
        self.open_window(Content::Document(Document { path, kind }))
    }

    fn window_showing(&self, path: &Path) -> Option<window::Id> {
        self.windows
            .iter()
            .find_map(|(id, window)| match &window.content {
                Content::Document(document) if document.path == path => Some(*id),
                _ => None,
            })
    }

    fn open_window(&mut self, content: Content) -> Task<Message> {
        let size = match content {
            Content::Settings => Size::new(520.0, 360.0),
            Content::Start => Size::new(640.0, 480.0),
            Content::Document(_) => Size::new(900.0, 700.0),
        };
        let (id, opened) = window::open(window::Settings {
            size,
            platform_specific: PlatformSpecific {
                application_id: APP_ID.to_owned(),
                ..PlatformSpecific::default()
            },
            ..window::Settings::default()
        });
        self.windows.insert(
            id,
            Window {
                content,
                fullscreen: false,
                notice: None,
                surface: None,
                drag_hover: false,
            },
        );
        opened.map(Message::WindowOpened)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::External(External::OpenPaths(paths)) => self.open_paths(paths),
            Message::External(External::OmarchyThemeChanged) => {
                self.reload_omarchy();
                Task::none()
            }
            Message::External(External::Drag(event)) => self.handle_drag(event),
            Message::WindowOpened(id) => window::run(id, wayland_surface)
                .map(move |surface| Message::SurfaceKnown(id, surface)),
            Message::SurfaceKnown(id, surface) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.surface = surface;
                }
                Task::none()
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::Key(id, key, modifiers) => match shortcuts::lookup(&key, modifiers) {
                Some(action) => self.perform(id, action),
                None => Task::none(),
            },
            Message::Perform(id, action) => self.perform(id, action),
            Message::DialogFinished(id, result) => match result {
                Ok(paths) => self.open_paths_if_any(paths),
                Err(error) => {
                    if let Some(window) = self.windows.get_mut(&id) {
                        window.notice = Some(format!("Could not show the file dialog: {error}"));
                    }
                    Task::none()
                }
            },
            Message::AppearanceSelected(appearance) => {
                self.settings.appearance = appearance;
                self.save_settings();
                Task::none()
            }
            Message::OmarchyPaletteToggled(enabled) => {
                self.settings.omarchy_palette = enabled;
                self.save_settings();
                Task::none()
            }
        }
    }

    fn handle_drag(&mut self, event: DragEvent) -> Task<Message> {
        let surface = match event {
            DragEvent::Entered { surface, .. }
            | DragEvent::Moved { surface, .. }
            | DragEvent::Left { surface }
            | DragEvent::Dropped { surface, .. } => surface,
        };
        let Some(window) = self
            .windows
            .values_mut()
            .find(|window| window.surface == Some(surface))
        else {
            return Task::none();
        };
        match event {
            DragEvent::Entered { accepted, .. } => window.drag_hover = accepted,
            DragEvent::Left { .. } => window.drag_hover = false,
            DragEvent::Moved { .. } => {}
            DragEvent::Dropped { uris, .. } => {
                window.drag_hover = false;
                let paths = uris
                    .iter()
                    .filter_map(|uri| dialog::file_uri_to_path(uri))
                    .collect();
                return self.open_paths_if_any(paths);
            }
        }
        Task::none()
    }

    fn open_paths_if_any(&mut self, paths: Vec<PathBuf>) -> Task<Message> {
        if paths.is_empty() {
            Task::none()
        } else {
            self.open_paths(paths)
        }
    }

    fn perform(&mut self, id: window::Id, action: Action) -> Task<Message> {
        match action {
            Action::Open => Task::perform(dialog::open_files(), move |result| {
                Message::DialogFinished(id, result)
            }),
            Action::CloseWindow => window::close(id),
            Action::Quit => iced::exit(),
            Action::Settings => match self
                .windows
                .iter()
                .find(|(_, window)| matches!(window.content, Content::Settings))
            {
                Some((settings_id, _)) => window::gain_focus(*settings_id),
                None => self.open_window(Content::Settings),
            },
            Action::ToggleFullscreen => self.set_fullscreen(id, None),
            Action::ExitFullscreen => self.set_fullscreen(id, Some(false)),
        }
    }

    fn set_fullscreen(&mut self, id: window::Id, fullscreen: Option<bool>) -> Task<Message> {
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let fullscreen = fullscreen.unwrap_or(!window.fullscreen);
        if fullscreen == window.fullscreen {
            return Task::none();
        }
        window.fullscreen = fullscreen;
        window::set_mode(
            id,
            if fullscreen {
                window::Mode::Fullscreen
            } else {
                window::Mode::Windowed
            },
        )
    }

    fn save_settings(&mut self) {
        self.settings_error = match &self.settings_path {
            Some(path) => self
                .settings
                .save_to(path)
                .err()
                .map(|error| format!("Could not save settings: {error}")),
            None => Some("No settings location: HOME is not set".to_owned()),
        };
    }

    pub fn title(&self, id: window::Id) -> String {
        match self.windows.get(&id).map(|window| &window.content) {
            Some(Content::Document(document)) => document.path.file_name().map_or_else(
                || document.path.display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            ),
            Some(Content::Settings) => "prev Settings".to_owned(),
            _ => "prev".to_owned(),
        }
    }

    pub fn theme(&self, _id: window::Id) -> Option<Theme> {
        resolve_theme(&self.settings, self.omarchy.as_ref())
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            Subscription::run(crate::external_events).map(Message::External),
            window::close_events().map(Message::WindowClosed),
            event::listen_with(|event, status, id| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }),
                    event::Status::Ignored,
                ) => Some(Message::Key(id, key, modifiers)),
                _ => None,
            }),
        ])
    }

    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let Some(window) = self.windows.get(&id) else {
            return text("").into();
        };
        let body = match &window.content {
            Content::Start => start_view(id),
            Content::Document(document) => document_view(document),
            Content::Settings => self.settings_view(),
        };
        let body = match &window.notice {
            Some(notice) => column![body, container(text(notice).size(13)).padding(8)].into(),
            None => body,
        };
        if window.drag_hover {
            drop_highlight(body)
        } else {
            body
        }
    }

    fn settings_view(&self) -> Element<'_, Message> {
        let selected = Some(self.settings.appearance);
        let appearance = column![
            text("Appearance").size(16),
            radio(
                "Follow system",
                Appearance::System,
                selected,
                Message::AppearanceSelected
            ),
            radio(
                "Light",
                Appearance::Light,
                selected,
                Message::AppearanceSelected
            ),
            radio(
                "Dark",
                Appearance::Dark,
                selected,
                Message::AppearanceSelected
            ),
        ]
        .spacing(8);
        let omarchy_label = match &self.omarchy {
            Some((_, theme)) => format!("Use Omarchy theme colors ({theme})"),
            None => "Use Omarchy theme colors (no Omarchy theme active)".to_owned(),
        };
        let mut content = column![
            appearance,
            rule::horizontal(1),
            checkbox(self.settings.omarchy_palette)
                .label(omarchy_label)
                .on_toggle(Message::OmarchyPaletteToggled),
        ]
        .spacing(16)
        .padding(24);
        if let Some(error) = &self.settings_error {
            content = content.push(text(error).size(13));
        }
        content.into()
    }
}

/// `None` follows the system light or dark preference. The Omarchy palette
/// wins unless the user forced the opposite mode.
fn resolve_theme(settings: &Settings, omarchy: Option<&(omarchy::Mode, Theme)>) -> Option<Theme> {
    let omarchy = omarchy.filter(|_| settings.omarchy_palette);
    match (settings.appearance, omarchy) {
        (Appearance::System, Some((_, theme))) => Some(theme.clone()),
        (Appearance::System, None) => None,
        (Appearance::Light, Some((omarchy::Mode::Light, theme))) => Some(theme.clone()),
        (Appearance::Dark, Some((omarchy::Mode::Dark, theme))) => Some(theme.clone()),
        (Appearance::Light, _) => Some(Theme::Light),
        (Appearance::Dark, _) => Some(Theme::Dark),
    }
}

fn start_view(id: window::Id) -> Element<'static, Message> {
    let hints = shortcuts::LABELS
        .iter()
        .filter(|(action, _)| matches!(action, Action::Open | Action::Settings))
        .map(|(action, label)| {
            text(format!("{label}  {}", action_name(*action)))
                .size(13)
                .into()
        });
    center(
        column![
            text("prev").size(32),
            text("Open or drop a PDF, image, SVG or Markdown file."),
            button("Open…").on_press(Message::Perform(id, Action::Open)),
            column(hints).spacing(4).align_x(Center),
        ]
        .spacing(16)
        .align_x(Center),
    )
    .into()
}

fn document_view(document: &Document) -> Element<'_, Message> {
    let status = match &document.kind {
        Ok(Some(kind)) => format!("{} — viewer not built yet", kind_name(*kind)),
        Ok(None) => "prev can't open this kind of file.".to_owned(),
        Err(error) => format!("prev can't read this file: {error}"),
    };
    center(
        column![
            text(document.path.display().to_string()).size(18),
            text(status)
        ]
        .spacing(12)
        .align_x(Center),
    )
    .into()
}

fn drop_highlight(body: Element<'_, Message>) -> Element<'_, Message> {
    container(body)
        .width(Fill)
        .height(Fill)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 3.0,
                radius: 6.0.into(),
            },
            ..container::Style::default()
        })
        .into()
}

/// The window's `wl_surface` pointer; `None` off Wayland.
fn wayland_surface(window: &dyn window::Window) -> Option<usize> {
    match window.window_handle().ok()?.as_raw() {
        RawWindowHandle::Wayland(handle) => Some(handle.surface.as_ptr() as usize),
        _ => None,
    }
}

fn action_name(action: Action) -> &'static str {
    match action {
        Action::Open => "Open",
        Action::CloseWindow => "Close window",
        Action::Quit => "Quit",
        Action::Settings => "Settings",
        Action::ToggleFullscreen => "Full screen",
        Action::ExitFullscreen => "Leave full screen",
    }
}

fn kind_name(kind: FileKind) -> String {
    match kind {
        FileKind::Pdf => "PDF document".to_owned(),
        FileKind::Image(format) => format!("{format:?} image"),
        FileKind::Svg => "SVG drawing".to_owned(),
        FileKind::Markdown => "Markdown document".to_owned(),
    }
}

fn iced_palette(palette: &omarchy::Palette) -> iced::theme::Palette {
    let color = |rgb: omarchy::Rgb| Color::from_rgb8(rgb.red, rgb.green, rgb.blue);
    iced::theme::Palette {
        background: color(palette.background),
        text: color(palette.foreground),
        primary: color(palette.accent),
        success: color(palette.success),
        warning: color(palette.warning),
        danger: color(palette.danger),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn omarchy_dark() -> (omarchy::Mode, Theme) {
        (
            omarchy::Mode::Dark,
            Theme::custom("hackerman", Theme::Dark.palette()),
        )
    }

    fn resolved_name(
        appearance: Appearance,
        omarchy_palette: bool,
        omarchy: Option<&(omarchy::Mode, Theme)>,
    ) -> Option<String> {
        resolve_theme(
            &Settings {
                appearance,
                omarchy_palette,
            },
            omarchy,
        )
        .map(|theme| theme.to_string())
    }

    #[test]
    fn follows_system_without_omarchy() {
        assert_eq!(resolved_name(Appearance::System, true, None), None);
        assert_eq!(
            resolved_name(Appearance::Light, true, None),
            Some(Theme::Light.to_string())
        );
    }

    #[test]
    fn omarchy_palette_applies_when_mode_matches() {
        let omarchy = omarchy_dark();
        assert_eq!(
            resolved_name(Appearance::System, true, Some(&omarchy)).as_deref(),
            Some("hackerman")
        );
        assert_eq!(
            resolved_name(Appearance::Dark, true, Some(&omarchy)).as_deref(),
            Some("hackerman")
        );
        assert_eq!(
            resolved_name(Appearance::Light, true, Some(&omarchy)),
            Some(Theme::Light.to_string())
        );
    }

    #[test]
    fn omarchy_palette_can_be_turned_off() {
        let omarchy = omarchy_dark();
        assert_eq!(
            resolved_name(Appearance::System, false, Some(&omarchy)),
            None
        );
        assert_eq!(
            resolved_name(Appearance::Dark, false, Some(&omarchy)),
            Some(Theme::Dark.to_string())
        );
    }
}
