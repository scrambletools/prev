//! The iced application: one window per document, plus start and settings
//! windows.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{column, container, row, space, stack, text, toggler};
use iced::window::{self, settings::PlatformSpecific};
use iced::{Center, Color, Element, Event, Fill, Size, Subscription, Task, Theme, event};
use prev::filetype::{self, FileKind};
use prev::image::window::{self as image_window, ImageWindow, Source};
use prev::markdown::{self, MarkdownWindow};
use prev::pdf::window::{self as pdf_window, Effect, PdfWindow};
use prev::shortcuts::{self, Action};
use prev::ui::button::{self, Kind};
use prev::ui::{Icon, Type, component, icon, style};
use prev::{dialog, omarchy, portal, ui};
use prev_store::settings::{self, Appearance, Settings};
use raw_window_handle::RawWindowHandle;
use smithay_clipboard::dnd::DragEvent;

use crate::External;

pub const APP_ID: &str = if prev_store::paths::PRODUCTION {
    "io.github.scrambletools.prev"
} else {
    "io.github.scrambletools.prev.Devel"
};

pub struct Prev {
    windows: BTreeMap<window::Id, Window>,
    settings: Settings,
    settings_path: Option<PathBuf>,
    settings_error: Option<String>,
    omarchy_dir: Option<PathBuf>,
    omarchy: Option<omarchy::Palette>,
    system_mode: iced::theme::Mode,
    theme: Theme,
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
    pdf: Option<Box<PdfWindow>>,
    images: Option<Box<ImageWindow>>,
    markdown: Option<Box<MarkdownWindow>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    External(External),
    WindowOpened(window::Id),
    SurfaceKnown(window::Id, Option<usize>),
    WindowClosed(window::Id),
    ScaleFactor(window::Id, f32),
    Frame(std::time::Instant),
    Pdf(window::Id, pdf_window::Message),
    Image(window::Id, image_window::Message),
    Markdown(window::Id, markdown::Message),
    SystemTheme(iced::theme::Mode),
    Key(window::Id, Key, Modifiers),
    Perform(window::Id, Action),
    DialogFinished(window::Id, Result<Vec<PathBuf>, String>),
    AppearanceSelected(Appearance),
    OmarchyPaletteToggled(bool),
    DismissNotice(window::Id),
    AnimationsEnabled(Option<bool>),
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
            system_mode: iced::theme::Mode::None,
            theme: Theme::Light,
        };
        prev.reload_omarchy();
        let task = prev.open_paths(paths);
        let system = iced::system::theme().map(Message::SystemTheme);
        let motion = Task::perform(portal::animations_enabled(), Message::AnimationsEnabled);
        (prev, Task::batch([task, system, motion]))
    }

    fn reload_omarchy(&mut self) {
        self.omarchy = self.omarchy_dir.as_deref().and_then(omarchy::load);
        self.refresh_theme();
    }

    /// Rebuilds the M3 scheme after a setting, the system mode or the
    /// Omarchy theme changed.
    fn refresh_theme(&mut self) {
        let (seed, dark) = theme_choice(
            &self.settings,
            self.omarchy.as_ref(),
            self.system_mode == iced::theme::Mode::Dark,
        );
        let name = match (&self.omarchy, self.settings.omarchy_palette) {
            (Some(palette), true) => format!("prev ({})", palette.name),
            _ => "prev".to_owned(),
        };
        self.theme = ui::scheme::theme(name, seed, dark);
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
        let mut tasks = Vec::new();
        let mut images = Vec::new();
        for path in paths {
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            if let Some(id) = self.window_showing(&path) {
                tasks.push(window::gain_focus(id));
                continue;
            }
            match filetype::detect_path(&path) {
                Ok(Some(FileKind::Image(format))) => images.push((path, Source::Raster(format))),
                Ok(Some(FileKind::Svg)) => images.push((path, Source::Svg)),
                _ => tasks.push(self.open_document(path)),
            }
        }
        // Images opened together share one window, as in Preview.
        if !images.is_empty() {
            tasks.push(self.open_images(images));
        }
        tasks.extend(start_windows.into_iter().map(window::close));
        Task::batch(tasks)
    }

    fn open_images(&mut self, files: Vec<(PathBuf, Source)>) -> Task<Message> {
        let path = files[0].0.clone();
        let (images, loading) = ImageWindow::open(files);
        let (id, opened) = self.open_window_with_id(Content::Document(Document {
            path,
            kind: Ok(None),
            pdf: None,
            images: Some(Box::new(images)),
            markdown: None,
        }));
        Task::batch([
            opened,
            loading.map(move |message| Message::Image(id, message)),
        ])
    }

    fn markdown_mut(&mut self, id: window::Id) -> Option<&mut MarkdownWindow> {
        match &mut self.windows.get_mut(&id)?.content {
            Content::Document(document) => document.markdown.as_deref_mut(),
            _ => None,
        }
    }

    fn images_mut(&mut self, id: window::Id) -> Option<&mut ImageWindow> {
        match &mut self.windows.get_mut(&id)?.content {
            Content::Document(document) => document.images.as_deref_mut(),
            _ => None,
        }
    }

    fn with_images(
        &mut self,
        id: window::Id,
        run: impl FnOnce(&mut ImageWindow) -> Task<image_window::Message>,
    ) -> Task<Message> {
        match self.images_mut(id) {
            Some(images) => run(images).map(move |message| Message::Image(id, message)),
            None => Task::none(),
        }
    }

    fn open_document(&mut self, path: PathBuf) -> Task<Message> {
        let path = std::fs::canonicalize(&path).unwrap_or(path);
        if let Some(id) = self.window_showing(&path) {
            return window::gain_focus(id);
        }
        let kind = filetype::detect_path(&path).map_err(|error| error.to_string());
        if matches!(kind, Ok(Some(FileKind::Pdf))) {
            let (pdf, opening) = PdfWindow::open(path.clone());
            let (id, opened) = self.open_window_with_id(Content::Document(Document {
                path,
                kind,
                pdf: Some(Box::new(pdf)),
                images: None,
                markdown: None,
            }));
            return Task::batch([
                opened,
                opening.map(move |message| Message::Pdf(id, message)),
            ]);
        }
        if matches!(kind, Ok(Some(FileKind::Markdown))) {
            let (document, loading) = MarkdownWindow::open(path.clone());
            let (id, opened) = self.open_window_with_id(Content::Document(Document {
                path,
                kind,
                pdf: None,
                images: None,
                markdown: Some(Box::new(document)),
            }));
            return Task::batch([
                opened,
                loading.map(move |message| Message::Markdown(id, message)),
            ]);
        }
        self.open_window(Content::Document(Document {
            path,
            kind,
            pdf: None,
            images: None,
            markdown: None,
        }))
    }

    fn pdf_mut(&mut self, id: window::Id) -> Option<&mut PdfWindow> {
        match &mut self.windows.get_mut(&id)?.content {
            Content::Document(document) => document.pdf.as_deref_mut(),
            _ => None,
        }
    }

    /// Runs a PDF window update and applies the window changes it asks for.
    fn with_pdf(
        &mut self,
        id: window::Id,
        run: impl FnOnce(&mut PdfWindow) -> Task<pdf_window::Message>,
    ) -> Task<Message> {
        let Some(pdf) = self.pdf_mut(id) else {
            return Task::none();
        };
        let task = run(pdf).map(move |message| Message::Pdf(id, message));
        let effects = pdf.take_effects();
        let effect_tasks: Vec<Task<Message>> = effects
            .into_iter()
            .map(|effect| match effect {
                Effect::EnterFullscreen => self.set_fullscreen(id, Some(true)),
                Effect::LeaveFullscreen => self.set_fullscreen(id, Some(false)),
                Effect::Quit => iced::exit(),
            })
            .collect();
        Task::batch(std::iter::once(task).chain(effect_tasks))
    }

    fn window_showing(&self, path: &Path) -> Option<window::Id> {
        self.windows
            .iter()
            .find_map(|(id, window)| match &window.content {
                Content::Document(document)
                    if document.path == path
                        || document
                            .images
                            .as_ref()
                            .is_some_and(|images| images.paths().any(|shown| shown == path)) =>
                {
                    Some(*id)
                }
                _ => None,
            })
    }

    fn open_window(&mut self, content: Content) -> Task<Message> {
        self.open_window_with_id(content).1
    }

    fn open_window_with_id(&mut self, content: Content) -> (window::Id, Task<Message>) {
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
        (id, opened.map(Message::WindowOpened))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::External(External::OpenPaths(paths)) => self.open_paths(paths),
            Message::External(External::OmarchyThemeChanged) => {
                self.reload_omarchy();
                Task::none()
            }
            Message::External(External::Drag(event)) => self.handle_drag(event),
            Message::WindowOpened(id) => Task::batch([
                window::run(id, wayland_surface)
                    .map(move |surface| Message::SurfaceKnown(id, surface)),
                window::scale_factor(id).map(move |scale| Message::ScaleFactor(id, scale)),
            ]),
            Message::ScaleFactor(id, scale) => Task::batch([
                self.with_pdf(id, |pdf| pdf.set_device_scale(scale)),
                self.with_images(id, |images| images.set_device_scale(scale)),
            ]),
            Message::Pdf(id, message) => self.with_pdf(id, |pdf| pdf.update(message)),
            Message::Image(id, message) => self.with_images(id, |images| images.update(message)),
            Message::Markdown(id, message) => {
                let Some(document) = self.markdown_mut(id) else {
                    return Task::none();
                };
                let task = document
                    .update(message)
                    .map(move |message| Message::Markdown(id, message));
                let paths: Vec<PathBuf> = document
                    .take_effects()
                    .into_iter()
                    .map(|markdown::Effect::OpenPath(path)| path)
                    .collect();
                Task::batch([task, self.open_paths_if_any(paths)])
            }
            Message::SystemTheme(mode) => {
                self.system_mode = mode;
                self.refresh_theme();
                Task::none()
            }
            Message::Frame(now) => {
                let ids: Vec<window::Id> = self.windows.keys().copied().collect();
                Task::batch(
                    ids.into_iter()
                        .map(|id| self.with_pdf(id, |pdf| pdf.bench_frame(now))),
                )
            }
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
            Message::Key(_, Key::Named(keyboard::key::Named::Tab), modifiers)
                if !modifiers.command() && !modifiers.alt() =>
            {
                if modifiers.shift() {
                    iced::widget::operation::focus_previous()
                } else {
                    iced::widget::operation::focus_next()
                }
            }
            Message::Key(id, key, modifiers) => {
                let action = shortcuts::lookup(&key, modifiers);
                let handled = self.pdf_mut(id).and_then(|pdf| match action {
                    Some(action) => pdf.shortcut(action),
                    None => pdf.key(&key, modifiers),
                });
                if let Some(task) = handled {
                    let task = task.map(move |message| Message::Pdf(id, message));
                    return Task::batch([task, self.with_pdf(id, |_| Task::none())]);
                }
                let handled = self.images_mut(id).and_then(|images| match action {
                    Some(action) => images.shortcut(action),
                    None => images.key(&key, modifiers),
                });
                match (handled, action) {
                    (Some(task), _) => task.map(move |message| Message::Image(id, message)),
                    (None, Some(action)) => self.perform(id, action),
                    (None, None) => Task::none(),
                }
            }
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
                self.refresh_theme();
                Task::none()
            }
            Message::AnimationsEnabled(enabled) => {
                ui::motion::set_reduced(enabled == Some(false));
                Task::none()
            }
            Message::DismissNotice(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.notice = None;
                }
                Task::none()
            }
            Message::OmarchyPaletteToggled(enabled) => {
                self.settings.omarchy_palette = enabled;
                self.save_settings();
                self.refresh_theme();
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
            Action::Escape => self.set_fullscreen(id, Some(false)),
            _ => Task::none(),
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
        let title = self.base_title(id);
        if prev_store::paths::PRODUCTION {
            title
        } else {
            format!("{title} (dev)")
        }
    }

    fn base_title(&self, id: window::Id) -> String {
        match self.windows.get(&id).map(|window| &window.content) {
            Some(Content::Document(document)) => {
                let path = document
                    .images
                    .as_ref()
                    .map_or(document.path.as_path(), |images| images.current_path());
                path.file_name().map_or_else(
                    || path.display().to_string(),
                    |name| name.to_string_lossy().into_owned(),
                )
            }
            Some(Content::Settings) => "prev Settings".to_owned(),
            _ => "prev".to_owned(),
        }
    }

    pub fn theme(&self, _id: window::Id) -> Option<Theme> {
        Some(self.theme.clone())
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let benchmarking = self.windows.values().any(|window| {
            matches!(&window.content, Content::Document(Document { pdf: Some(pdf), .. }) if pdf.wants_frames())
        });
        let frames = if benchmarking {
            window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        };
        Subscription::batch([
            frames,
            iced::system::theme_changes().map(Message::SystemTheme),
            Subscription::run(crate::external_events).map(Message::External),
            window::close_events().map(Message::WindowClosed),
            event::listen_with(|event, status, id| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }),
                    event::Status::Ignored,
                ) => Some(Message::Key(id, key, modifiers)),
                (Event::Window(window::Event::Rescaled(scale)), _) => {
                    Some(Message::ScaleFactor(id, scale))
                }
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
            Content::Document(Document { pdf: Some(pdf), .. }) => {
                pdf.view().map(move |message| Message::Pdf(id, message))
            }
            Content::Document(Document {
                images: Some(images),
                ..
            }) => images
                .view()
                .map(move |message| Message::Image(id, message)),
            Content::Document(Document {
                markdown: Some(document),
                ..
            }) => document
                .view(&self.theme)
                .map(move |message| Message::Markdown(id, message)),
            Content::Document(document) => document_view(document),
            Content::Settings => self.settings_view(),
        };
        let body = match &window.notice {
            Some(notice) => component::snackbar(body, notice, Message::DismissNotice(id)),
            None => body,
        };
        if window.drag_hover {
            drop_highlight(body)
        } else {
            body
        }
    }

    fn settings_view(&self) -> Element<'_, Message> {
        let appearance = self.settings.appearance;
        let choice = |glyph: Icon, label: &'static str, value: Appearance| {
            ui::with_icon(Kind::Tonal, glyph, label)
                .selected(appearance == value)
                .on_press(Message::AppearanceSelected(value))
        };
        let omarchy_note = match &self.omarchy {
            Some(palette) => format!("Colors are built from the accent of “{}”.", palette.name),
            None => "No Omarchy theme is active.".to_owned(),
        };
        let mut content = column![
            ui::styled("Settings", Type::HeadlineSmall),
            component::section("Appearance"),
            component::connected(vec![
                choice(Icon::Settings, "System", Appearance::System),
                choice(Icon::LightMode, "Light", Appearance::Light),
                choice(Icon::DarkMode, "Dark", Appearance::Dark),
            ]),
            component::section("Colors"),
            row![
                column![
                    ui::styled("Use Omarchy accent color", Type::BodyLarge),
                    ui::styled(omarchy_note, Type::BodyMedium).style(style::on_surface_variant),
                ]
                .spacing(2)
                .width(Fill),
                toggler(self.settings.omarchy_palette)
                    .on_toggle(Message::OmarchyPaletteToggled)
                    .size(28)
                    .style(style::switch),
            ]
            .spacing(16)
            .align_y(Center),
        ]
        .spacing(12)
        .padding(24);
        if let Some(error) = &self.settings_error {
            content = content.push(ui::styled(error, Type::BodyMedium).style(style::error_text));
        }
        container(content)
            .width(Fill)
            .height(Fill)
            .style(style::surface)
            .into()
    }
}

/// The seed color and whether the scheme is dark. The Omarchy accent is
/// the seed when enabled; in "Follow system" the Omarchy theme's own mode
/// wins over the system's.
fn theme_choice(
    settings: &Settings,
    omarchy: Option<&omarchy::Palette>,
    system_dark: bool,
) -> (Color, bool) {
    let omarchy = omarchy.filter(|_| settings.omarchy_palette);
    let dark = match (settings.appearance, omarchy) {
        (Appearance::System, Some(palette)) => palette.mode == omarchy::Mode::Dark,
        (Appearance::System, None) => system_dark,
        (Appearance::Light, _) => false,
        (Appearance::Dark, _) => true,
    };
    let seed = omarchy.map_or(ui::scheme::PREV_SEED, |palette| {
        let accent = palette.accent;
        Color::from_rgb8(accent.red, accent.green, accent.blue)
    });
    (seed, dark)
}

fn start_view(id: window::Id) -> Element<'static, Message> {
    let hints = shortcuts::LABELS
        .iter()
        .filter(|(action, _)| matches!(action, Action::Open | Action::Settings))
        .map(|(action, label)| {
            row![
                ui::styled(action_name(*action), Type::BodyMedium).style(style::on_surface_variant),
                ui::styled(*label, Type::LabelLarge),
            ]
            .spacing(8)
            .into()
        });
    container(
        column![
            icon::filled(Icon::Draft, 64).style(style::primary_text),
            ui::styled("prev", Type::DisplaySmall),
            ui::styled(
                "Open or drop a PDF, image, SVG or Markdown file.",
                Type::BodyLarge
            )
            .style(style::on_surface_variant),
            ui::with_icon(Kind::Filled, Icon::FolderOpen, "Open…")
                .size(button::Size::Medium)
                .on_press(Message::Perform(id, Action::Open)),
            row(hints).spacing(24),
        ]
        .spacing(16)
        .align_x(Center),
    )
    .center(Fill)
    .style(style::surface)
    .into()
}

fn document_view(document: &Document) -> Element<'_, Message> {
    let status = match &document.kind {
        Ok(Some(kind)) => format!("{}: this viewer is not built yet.", kind_name(*kind)),
        Ok(None) => "prev can't open this kind of file.".to_owned(),
        Err(error) => format!("prev can't read this file: {error}"),
    };
    let name = document.path.file_name().map_or_else(
        || document.path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    container(component::empty_state(Icon::BrokenImage, name, status))
        .style(style::surface)
        .into()
}

fn drop_highlight(body: Element<'_, Message>) -> Element<'_, Message> {
    stack![
        body,
        container(space())
            .width(Fill)
            .height(Fill)
            .style(style::drop_target)
    ]
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
        Action::Settings => "Settings",
        _ => "",
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

#[cfg(test)]
mod tests {
    use super::*;

    fn omarchy_dark() -> omarchy::Palette {
        let rgb = |red, green, blue| omarchy::Rgb { red, green, blue };
        omarchy::Palette {
            name: "hackerman".into(),
            mode: omarchy::Mode::Dark,
            background: rgb(0, 0, 0),
            foreground: rgb(255, 255, 255),
            accent: rgb(0x82, 0xfb, 0x9c),
            success: rgb(0, 255, 0),
            warning: rgb(255, 255, 0),
            danger: rgb(255, 0, 0),
        }
    }

    fn choice(
        appearance: Appearance,
        omarchy_palette: bool,
        omarchy: Option<&omarchy::Palette>,
        system_dark: bool,
    ) -> (Color, bool) {
        theme_choice(
            &Settings {
                appearance,
                omarchy_palette,
            },
            omarchy,
            system_dark,
        )
    }

    #[test]
    fn follows_system_without_omarchy() {
        assert_eq!(
            choice(Appearance::System, true, None, true),
            (ui::scheme::PREV_SEED, true)
        );
        assert_eq!(
            choice(Appearance::Light, true, None, true),
            (ui::scheme::PREV_SEED, false)
        );
    }

    #[test]
    fn omarchy_accent_seeds_the_scheme() {
        let omarchy = omarchy_dark();
        let accent = Color::from_rgb8(0x82, 0xfb, 0x9c);
        assert_eq!(
            choice(Appearance::System, true, Some(&omarchy), false),
            (accent, true),
            "the Omarchy mode wins when following the system"
        );
        assert_eq!(
            choice(Appearance::Light, true, Some(&omarchy), true),
            (accent, false)
        );
    }

    #[test]
    fn omarchy_accent_can_be_turned_off() {
        let omarchy = omarchy_dark();
        assert_eq!(
            choice(Appearance::System, false, Some(&omarchy), false),
            (ui::scheme::PREV_SEED, false)
        );
    }
}
