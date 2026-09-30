//! The iced application: one window per document, plus start and settings
//! windows.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{container, mouse_area, opaque, pick_list, space, stack, text, toggler};
use iced::window::{self, settings::PlatformSpecific};
use iced::{Center, Color, Element, Event, Fill, Length, Size, Subscription, Task, Theme, event};
use prev::dnd::DragEvent;
use prev::filetype::{self, FileKind};
use prev::image::window::{self as image_window, ImageWindow, Source};
use prev::markdown::{self, MarkdownWindow};
use prev::pdf::window::{self as pdf_window, Effect, PdfWindow};
use prev::shortcuts::{self, Action};
use prev::ui::button::{self, Kind};
use prev::ui::component::Backdrop;
use prev::ui::dir::row;
use prev::ui::{Icon, Type, component, icon, style};
use prev::{column, row};
use prev::{dialog, omarchy, portal, ui};
use prev_store::settings::{self, Appearance, SYSTEM_LANGUAGE, Settings};
use raw_window_handle::RawWindowHandle;

use crate::External;

#[cfg(target_os = "linux")]
pub const APP_ID: &str = if prev_store::paths::PRODUCTION {
    "io.github.scrambletools.prev"
} else {
    "io.github.scrambletools.prev.Devel"
};

pub struct Prev {
    /// The window that started the drag under way, told how it ends.
    drag_origin: Option<window::Id>,
    /// Whether Shift is held, which makes dropped images files.
    shift: bool,
    windows: BTreeMap<window::Id, Window>,
    settings: Settings,
    settings_path: Option<PathBuf>,
    settings_error: Option<String>,
    omarchy_dir: Option<PathBuf>,
    omarchy: Option<omarchy::Palette>,
    system_mode: iced::theme::Mode,
    theme: Theme,
    /// Documents still being written after their windows closed.
    pending_saves: usize,
    /// Whether the system allows animations (its reduced motion setting).
    system_animations: bool,
    /// Whether the keyboard layout in use types right to left, if the
    /// system says.
    keyboard_rtl: Option<bool>,
    /// The window that last had the focus, which menu bar items act on.
    focused: Option<window::Id>,
    /// Storage paths as typed in the settings dialog, before applying.
    storage_drafts: [String; 3],
    /// Why a typed or chosen path was refused, per storage row.
    storage_errors: [Option<String>; 3],
}

/// The files and folders prev keeps, whose places the settings choose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Storage {
    Signatures,
    Versions,
    Bookmarks,
}

impl Storage {
    const ALL: [Storage; 3] = [Storage::Signatures, Storage::Versions, Storage::Bookmarks];

    fn index(self) -> usize {
        self as usize
    }

    fn label(self) -> String {
        match self {
            Storage::Signatures => prev::fl!("settings-storage-signatures"),
            Storage::Versions => prev::fl!("settings-storage-versions"),
            Storage::Bookmarks => prev::fl!("settings-storage-bookmarks"),
        }
    }

    /// The title of the dialog that chooses where to keep it.
    fn choose_title(self) -> String {
        match self {
            Storage::Signatures => prev::fl!("dialog-choose-signatures"),
            Storage::Versions => prev::fl!("dialog-choose-versions"),
            Storage::Bookmarks => prev::fl!("dialog-choose-bookmarks"),
        }
    }

    fn path(self, settings: &Settings) -> &std::path::Path {
        match self {
            Storage::Signatures => &settings.signatures,
            Storage::Versions => &settings.versions,
            Storage::Bookmarks => &settings.bookmarks,
        }
    }
}

/// Whether prev can keep `storage` at `path`: a full path to an existing
/// folder it can write in (for bookmarks, a file in one).
fn check_storage(storage: Storage, path: &std::path::Path) -> Result<(), String> {
    let shown = shown_path(path);
    if !path.is_absolute() {
        return Err(prev::fl!("settings-full-path"));
    }
    let folder = match storage {
        Storage::Bookmarks => {
            if path.is_dir() {
                return Err(prev::fl!("settings-path-is-folder", path = shown));
            }
            path.parent().unwrap_or(path)
        }
        _ => path,
    };
    if !folder.exists() {
        return Err(prev::fl!(
            "settings-folder-missing",
            path = shown_path(folder)
        ));
    }
    if !folder.is_dir() {
        return Err(prev::fl!(
            "settings-path-is-file",
            path = shown_path(folder)
        ));
    }
    // Writing a file is the only sure test of permission.
    let probe = folder.join(format!(".prev-write-test-{}", std::process::id()));
    match std::fs::write(&probe, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(&probe);
            Ok(())
        }
        Err(error) => Err(prev::fl!(
            "settings-cannot-write",
            path = shown_path(folder),
            error = error.to_string()
        )),
    }
}

/// The app icon for window systems that take it from the window (X11);
/// Wayland compositors use the desktop entry's.
fn window_icon() -> Option<window::Icon> {
    let png =
        include_bytes!("../../../data/icons/hicolor/64x64/apps/io.github.scrambletools.prev.png");
    let image = image::load_from_memory(png).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    window::icon::from_rgba(image.into_raw(), width, height).ok()
}

/// A path as the settings dialog shows it, with `~` for the home folder.
fn shown_path(path: &std::path::Path) -> String {
    prev_store::paths::abbreviate_home(path)
        .to_string_lossy()
        .into_owned()
}

struct Window {
    content: Content,
    fullscreen: bool,
    notice: Option<String>,
    /// The `wl_surface` pointer, used to match drag events to windows.
    surface: Option<usize>,
    drag_hover: bool,
    /// The settings dialog, shown over this window.
    settings_open: bool,
    /// The window's size, for telling when the pointer leaves it.
    size: Size,
}

enum Content {
    Start,
    Document(Document),
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
    /// The window manager or a shortcut asked to close a window.
    CloseRequested(window::Id),
    Resized(window::Id, Size),
    /// The size of the monitor a new window opened on, if known. Only
    /// asked for off Linux.
    #[cfg_attr(target_os = "linux", allow(dead_code))]
    MonitorSize(window::Id, Option<Size>),
    /// The pointer moved, reported while a drag may leave its window.
    PointerMoved(window::Id, iced::Point),
    /// A file dropped through the windowing system rather than prev's own
    /// drag and drop: on Windows and X11.
    FileDropped(window::Id, PathBuf),
    /// A drop was read: where it landed, what it brought, and whether
    /// Shift made it a drop of files.
    DropDecoded(
        window::Id,
        f32,
        f32,
        prev::drag::Action,
        prev::drag::Dropped,
        bool,
    ),
    ScaleFactor(window::Id, f32),
    Frame(std::time::Instant),
    Pdf(window::Id, pdf_window::Message),
    Image(window::Id, image_window::Message),
    Markdown(window::Id, markdown::Message),
    SystemTheme(iced::theme::Mode),
    Key(window::Id, Key, Modifiers),
    Modifiers(window::Id, Modifiers),
    MouseReleased(window::Id),
    /// The pointer entered (`true`) or left a window.
    Pointer(window::Id, bool),
    CloseSettings(window::Id),
    CornerRadius(f32),
    OverlayTransparency(f32),
    AnimationsToggled(bool),
    SliderReleased,
    StorageDraft(Storage, String),
    StorageApply(Storage),
    StorageChoose(Storage),
    StorageChosen(Storage, Result<Option<PathBuf>, String>),
    AutoHideToolbarToggled(bool),
    Perform(window::Id, Action),
    DialogFinished(window::Id, Result<Vec<PathBuf>, String>),
    AppearanceSelected(Appearance),
    /// A language tag, or `system`.
    LanguageSelected(String),
    /// The input language's tag, or `system`.
    InputLanguageSelected(String),
    /// The keyboard layout changed direction.
    KeyboardDirection(Option<bool>),
    OmarchyPaletteToggled(bool),
    DismissNotice(window::Id),
    AnimationsEnabled(Option<bool>),
    FinalSaveDone,
    /// Window `id` got the keyboard focus.
    WindowFocused(window::Id),
    /// Files chosen in an Open dialog shown with no window open.
    #[cfg(target_os = "macos")]
    OpenChosen(Result<Vec<PathBuf>, String>),
}

impl Prev {
    pub fn boot(paths: Vec<PathBuf>, omarchy_dir: Option<PathBuf>) -> (Self, Task<Message>) {
        let settings_path = settings::default_path();
        let legacy = settings::legacy_path();
        let loaded = settings_path
            .as_deref()
            .map(|path| Settings::load_or_create(path, legacy.as_deref()));
        let (settings, settings_error) = match loaded {
            Some(Ok(settings)) => (settings, None),
            Some(Err(error)) => (
                Settings::default(),
                Some(prev::i18n::Describe::describe(&error)),
            ),
            None => (Settings::default(), None),
        };
        // Signatures, versions and bookmarks live where the settings say.
        prev_store::paths::set_locations(settings.locations());
        prev::i18n::set_language(settings.chosen_language());
        let mut prev = Self {
            drag_origin: None,
            shift: false,
            windows: BTreeMap::new(),
            settings,
            settings_path,
            settings_error,
            omarchy_dir,
            omarchy: None,
            system_mode: iced::theme::Mode::None,
            theme: Theme::Light,
            pending_saves: 0,
            storage_drafts: Default::default(),
            storage_errors: Default::default(),
            system_animations: true,
            keyboard_rtl: None,
            focused: None,
        };
        ui::component::set_floating_bars(prev.settings.auto_hide_toolbar);
        ui::shape::set_surface(prev.settings.corner_radius);
        ui::component::set_floating_transparency(prev.settings.overlay_transparency);
        prev.apply_motion();
        prev.apply_input_direction();
        #[cfg(target_os = "macos")]
        prev::menu_macos::install();
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
            let path = prev_store::paths::canonical(&path);
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
            Some(images) => {
                let task = run(images).map(move |message| Message::Image(id, message));
                let started = images.take_drag_started();
                let closing = images.take_closing();
                if started {
                    self.drag_origin = Some(id);
                }
                if closing {
                    Task::batch([task, window::close(id)])
                } else {
                    task
                }
            }
            None => Task::none(),
        }
    }

    fn open_document(&mut self, path: PathBuf) -> Task<Message> {
        let path = prev_store::paths::canonical(&path);
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
        let started = pdf.take_drag_started();
        let effects = pdf.take_effects();
        if started {
            self.drag_origin = Some(id);
        }
        let effect_tasks: Vec<Task<Message>> = effects
            .into_iter()
            .map(|effect| match effect {
                Effect::EnterFullscreen => self.set_fullscreen(id, Some(true)),
                Effect::LeaveFullscreen => self.set_fullscreen(id, Some(false)),
                Effect::Quit => quit(),
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
            Content::Start => Size::new(640.0, 480.0),
            Content::Document(_) => Size::new(900.0, 700.0),
        };
        let (id, opened) = window::open(window::Settings {
            size,
            platform_specific: platform_settings(),
            // Windows with markup that would be lost ask first.
            exit_on_close_request: false,
            icon: window_icon(),
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
                settings_open: false,
                size,
            },
        );
        (id, opened.map(Message::WindowOpened))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::External(External::OpenPaths(paths)) => self.open_paths(paths),
            #[cfg(target_os = "macos")]
            Message::External(External::Menu(action)) => {
                let target = self
                    .focused
                    .filter(|id| self.windows.contains_key(id))
                    .or_else(|| self.windows.keys().next().copied());
                match (target, action) {
                    (Some(id), action) => self.shortcut(id, action),
                    (None, Action::Quit) => quit(),
                    (None, Action::Open) => {
                        Task::perform(dialog::open_files(), Message::OpenChosen)
                    }
                    (None, _) => Task::none(),
                }
            }
            Message::WindowFocused(id) => {
                self.focused = Some(id);
                Task::none()
            }
            #[cfg(target_os = "macos")]
            Message::OpenChosen(result) => match result {
                Ok(paths) => self.open_paths_if_any(paths),
                Err(_) => Task::none(),
            },
            Message::External(External::OmarchyThemeChanged) => {
                self.reload_omarchy();
                Task::none()
            }
            Message::External(External::Drag(event)) => self.handle_drag(event),
            Message::WindowOpened(id) => Task::batch([
                window::run(id, wayland_surface)
                    .map(move |surface| Message::SurfaceKnown(id, surface)),
                window::scale_factor(id).map(move |scale| Message::ScaleFactor(id, scale)),
                fit_to_monitor(id),
            ]),
            Message::MonitorSize(id, Some(monitor)) => {
                // Room for the taskbar and window borders.
                let room = Size::new(monitor.width * 0.9, monitor.height * 0.85);
                let Some(window) = self.windows.get(&id) else {
                    return Task::none();
                };
                if window.size.width <= room.width && window.size.height <= room.height {
                    return Task::none();
                }
                let size = Size::new(
                    window.size.width.min(room.width),
                    window.size.height.min(room.height),
                );
                let at = iced::Point::new(
                    (monitor.width - size.width) / 2.0,
                    (monitor.height - size.height) / 2.0,
                );
                Task::batch([window::resize(id, size), window::move_to(id, at)])
            }
            Message::MonitorSize(_, None) => Task::none(),
            Message::ScaleFactor(id, scale) => Task::batch([
                self.with_pdf(id, |pdf| pdf.set_device_scale(scale)),
                self.with_images(id, |images| images.set_device_scale(scale)),
            ]),
            Message::Pdf(_, pdf_window::Message::ToggleFloatingBars)
            | Message::Image(_, image_window::Message::ToggleFloatingBars)
            | Message::Markdown(_, markdown::Message::ToggleFloatingBars) => {
                let enabled = !self.settings.auto_hide_toolbar;
                self.update(Message::AutoHideToolbarToggled(enabled))
            }
            Message::Pdf(id, pdf_window::Message::OpenSettings)
            | Message::Image(id, image_window::Message::OpenSettings)
            | Message::Markdown(id, markdown::Message::OpenSettings) => {
                self.perform(id, Action::Settings)
            }
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
                // Windows drops go to a target registered on each window.
                #[cfg(windows)]
                if let Some(hwnd) = surface {
                    prev::dnd::register(hwnd);
                }
                Task::none()
            }
            Message::WindowClosed(id) => {
                let flush = self
                    .windows
                    .remove(&id)
                    .and_then(|window| match window.content {
                        Content::Document(Document {
                            pdf: Some(mut pdf), ..
                        }) => pdf.flush(),
                        _ => None,
                    });
                if let Some(flush) = flush {
                    // Finish writing edits before the last window's exit.
                    self.pending_saves += 1;
                    return flush.map(|_| Message::FinalSaveDone);
                }
                self.exit_if_done()
            }
            Message::Resized(id, size) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.size = size;
                }
                Task::none()
            }
            Message::PointerMoved(id, point) => {
                let Some(window) = self.windows.get(&id) else {
                    return Task::none();
                };
                let size = window.size;
                let outside =
                    point.x < 0.0 || point.y < 0.0 || point.x > size.width || point.y > size.height;
                if !outside {
                    return Task::none();
                }
                self.with_pdf(id, |pdf| pdf.drag_pages_out())
            }
            Message::FileDropped(id, path) => {
                // No drop position comes with it; take the window's middle.
                let Some(window) = self.windows.get(&id) else {
                    return Task::none();
                };
                let (x, y) = (window.size.width / 2.0, window.size.height / 2.0);
                let dropped = prev::drag::Dropped::Files(vec![path]);
                self.drop_in(id, x, y, prev::drag::Action::Copy, dropped, self.shift)
            }
            Message::DropDecoded(id, x, y, action, dropped, as_file) => {
                self.drop_in(id, x, y, action, dropped, as_file)
            }
            Message::CloseRequested(id) => {
                if self
                    .images_mut(id)
                    .is_some_and(|images| !images.request_close())
                {
                    return Task::none();
                }
                window::close(id)
            }
            Message::FinalSaveDone => {
                self.pending_saves = self.pending_saves.saturating_sub(1);
                self.exit_if_done()
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
            Message::Key(id, key, modifiers) => match shortcuts::lookup(&key, modifiers) {
                Some(action) => self.shortcut(id, action),
                None => {
                    let handled = self.pdf_mut(id).and_then(|pdf| pdf.key(&key, modifiers));
                    if let Some(task) = handled {
                        let task = task.map(move |message| Message::Pdf(id, message));
                        return Task::batch([task, self.with_pdf(id, |_| Task::none())]);
                    }
                    self.images_mut(id)
                        .and_then(|images| images.key(&key, modifiers))
                        .map(|task| task.map(move |message| Message::Image(id, message)))
                        .unwrap_or_else(Task::none)
                }
            },
            Message::Modifiers(id, modifiers) => {
                // Shift moves dropped pages instead of copying them, and
                // takes dropped images as files.
                self.shift = modifiers.shift();
                prev::dnd::set_prefer_move(modifiers.shift());
                if let Some(pdf) = self.pdf_mut(id) {
                    pdf.set_modifiers(modifiers);
                }
                Task::none()
            }
            Message::StorageDraft(storage, text) => {
                self.storage_drafts[storage.index()] = text;
                self.storage_errors[storage.index()] = None;
                Task::none()
            }
            Message::StorageApply(storage) => {
                let typed = PathBuf::from(self.storage_drafts[storage.index()].trim());
                self.set_storage(storage, typed);
                Task::none()
            }
            Message::StorageChoose(storage) => {
                let path = storage.path(&self.settings);
                let current = match storage {
                    Storage::Bookmarks => path.parent().map(std::path::Path::to_path_buf),
                    _ => Some(path.to_path_buf()),
                }
                .filter(|folder| folder.is_dir());
                Task::perform(
                    dialog::choose_folder(storage.choose_title(), current),
                    move |result| Message::StorageChosen(storage, result),
                )
            }
            Message::StorageChosen(storage, Ok(Some(folder))) => {
                let path = match storage {
                    // A folder for the bookmarks file keeps its name.
                    Storage::Bookmarks => folder.join(
                        self.settings
                            .bookmarks
                            .file_name()
                            .unwrap_or(std::ffi::OsStr::new("bookmarks.toml")),
                    ),
                    _ => folder,
                };
                self.set_storage(storage, path);
                Task::none()
            }
            Message::StorageChosen(_, Ok(None)) => Task::none(),
            Message::StorageChosen(_, Err(error)) => {
                self.settings_error = Some(prev::fl!("app-file-dialog-failed", error = error));
                Task::none()
            }
            Message::AnimationsToggled(enabled) => {
                self.settings.animations = enabled;
                self.apply_motion();
                self.save_settings();
                Task::none()
            }
            Message::OverlayTransparency(percent) => {
                self.settings.overlay_transparency = percent;
                ui::component::set_floating_transparency(percent);
                Task::none()
            }
            Message::CornerRadius(radius) => {
                self.settings.corner_radius = radius;
                ui::shape::set_surface(radius);
                Task::none()
            }
            // Sliders save when let go, not on every step.
            Message::SliderReleased => {
                self.save_settings();
                Task::none()
            }
            Message::CloseSettings(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.settings_open = false;
                }
                Task::none()
            }
            Message::Pointer(id, inside) => {
                if let Some(pdf) = self.pdf_mut(id) {
                    pdf.set_pointer_inside(inside);
                }
                if let Some(images) = self.images_mut(id) {
                    images.set_pointer_inside(inside);
                }
                if let Some(document) = self.markdown_mut(id) {
                    document.set_pointer_inside(inside);
                }
                Task::none()
            }
            Message::AutoHideToolbarToggled(enabled) => {
                self.settings.auto_hide_toolbar = enabled;
                ui::component::set_floating_bars(enabled);
                self.save_settings();
                Task::none()
            }
            Message::MouseReleased(id) => match self.pdf_mut(id) {
                Some(pdf) if pdf.wants_release() => self.with_pdf(id, |pdf| {
                    pdf.update(pdf_window::Message::ThumbnailsReleased)
                }),
                _ => Task::none(),
            },
            Message::Perform(id, action) => self.perform(id, action),
            Message::DialogFinished(id, result) => match result {
                Ok(paths) => self.open_paths_if_any(paths),
                Err(error) => {
                    if let Some(window) = self.windows.get_mut(&id) {
                        window.notice = Some(prev::fl!("app-file-dialog-failed", error = error));
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
            Message::LanguageSelected(tag) => {
                self.settings.language = tag;
                prev::i18n::set_language(self.settings.chosen_language());
                #[cfg(target_os = "macos")]
                prev::menu_macos::install();
                // Without a keyboard layout, fields follow the interface.
                self.apply_input_direction();
                self.save_settings();
                Task::none()
            }
            Message::InputLanguageSelected(tag) => {
                self.settings.input_language = tag;
                self.apply_input_direction();
                self.save_settings();
                Task::none()
            }
            Message::KeyboardDirection(right_to_left) => {
                self.keyboard_rtl = right_to_left;
                self.apply_input_direction();
                Task::none()
            }
            Message::AnimationsEnabled(enabled) => {
                self.system_animations = enabled != Some(false);
                self.apply_motion();
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

    fn exit_if_done(&self) -> Task<Message> {
        if self.windows.is_empty() && self.pending_saves == 0 {
            prev::drag::clean_up();
            quit()
        } else {
            Task::none()
        }
    }

    fn handle_drag(&mut self, event: DragEvent) -> Task<Message> {
        // The window that started a drag learns how it ended.
        if let DragEvent::SourceEnded { action } = event {
            let Some(id) = self.drag_origin.take() else {
                return Task::none();
            };
            return self.with_pdf(id, |pdf| pdf.drag_ended(action));
        }
        let surface = match event {
            DragEvent::Entered { surface, .. }
            | DragEvent::Moved { surface, .. }
            | DragEvent::Left { surface }
            | DragEvent::Dropped { surface, .. } => surface,
            DragEvent::SourceEnded { .. } => return Task::none(),
        };
        let Some((&id, window)) = self
            .windows
            .iter_mut()
            .find(|(_, window)| window.surface == Some(surface))
        else {
            return Task::none();
        };
        let pdf = match &mut window.content {
            Content::Document(Document { pdf: Some(pdf), .. }) => Some(pdf),
            _ => None,
        };
        match event {
            DragEvent::Entered { accepted, x, y, .. } => {
                window.drag_hover = accepted;
                if let Some(pdf) = pdf {
                    pdf.drag_over(accepted.then_some((x as f32, y as f32)));
                }
            }
            DragEvent::Left { .. } => {
                window.drag_hover = false;
                if let Some(pdf) = pdf {
                    pdf.drag_over(None);
                }
            }
            DragEvent::Moved { x, y, .. } => {
                if let Some(pdf) = pdf
                    && window.drag_hover
                {
                    pdf.drag_over(Some((x as f32, y as f32)));
                }
            }
            DragEvent::Dropped {
                x,
                y,
                mime,
                data,
                action,
                ..
            } => {
                window.drag_hover = false;
                let (x, y) = (x as f32, y as f32);
                let as_file = self.shift;
                return Task::perform(
                    prev::image::editor::spawn(move || {
                        if as_file {
                            prev::drag::decode_as_file(&mime, data)
                        } else {
                            prev::drag::decode(&mime, data)
                        }
                    }),
                    move |dropped| {
                        Message::DropDecoded(
                            id,
                            x,
                            y,
                            action,
                            dropped.unwrap_or(prev::drag::Dropped::Nothing),
                            as_file,
                        )
                    },
                );
            }
            DragEvent::SourceEnded { .. } => {}
        }
        Task::none()
    }

    /// Hands a drop to the window it landed on, opening files it leaves.
    fn drop_in(
        &mut self,
        id: window::Id,
        x: f32,
        y: f32,
        action: prev::drag::Action,
        dropped: prev::drag::Dropped,
        as_file: bool,
    ) -> Task<Message> {
        // With Shift, dropped images are files: they join an image window,
        // and open in their own window anywhere else. Other files drop as
        // they would without it.
        let dropped = match dropped {
            prev::drag::Dropped::Files(files) if as_file => {
                let (images, others): (Vec<_>, Vec<_>) = files.into_iter().partition(|path| {
                    matches!(
                        filetype::detect_path(path),
                        Ok(Some(FileKind::Image(_) | FileKind::Svg))
                    )
                });
                let task = match self.images_mut(id) {
                    Some(images_window) => {
                        let (task, _) = images_window.add_files(images);
                        self.with_images(id, |_| task)
                    }
                    None => self.open_paths_if_any(images),
                };
                if others.is_empty() {
                    return task;
                }
                return Task::batch([
                    task,
                    self.drop_in(id, x, y, action, prev::drag::Dropped::Files(others), false),
                ]);
            }
            dropped => dropped,
        };
        let from_here = self.drag_origin == Some(id);
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let (task, files) = match &mut window.content {
            Content::Document(Document { pdf: Some(pdf), .. }) => {
                let (task, files) = pdf.drop_in(x, y, dropped, action);
                (task.map(move |message| Message::Pdf(id, message)), files)
            }
            // An image dragged out of the sidebar and let go over it again.
            Content::Document(Document {
                images: Some(_), ..
            }) if from_here && matches!(dropped, prev::drag::Dropped::Files(_)) => {
                (Task::none(), Vec::new())
            }
            Content::Document(Document {
                images: Some(images),
                ..
            }) => {
                let (task, files) = images.drop_in(x, y, dropped, action);
                (task.map(move |message| Message::Image(id, message)), files)
            }
            _ => match dropped {
                prev::drag::Dropped::Files(files) => (Task::none(), files),
                _ => (Task::none(), Vec::new()),
            },
        };
        // Updates that start drags or change windows run as usual.
        let after = self.with_pdf(id, |_| Task::none());
        Task::batch([task, after, self.open_paths_if_any(files)])
    }

    fn open_paths_if_any(&mut self, paths: Vec<PathBuf>) -> Task<Message> {
        if paths.is_empty() {
            Task::none()
        } else {
            self.open_paths(paths)
        }
    }

    /// Runs `action` in window `id`, from its shortcut or a menu item: the
    /// window's document takes it first, then the app.
    fn shortcut(&mut self, id: window::Id, action: Action) -> Task<Message> {
        if action == Action::Escape
            && self
                .windows
                .get(&id)
                .is_some_and(|window| window.settings_open)
        {
            return self.update(Message::CloseSettings(id));
        }
        if let Some(task) = self.pdf_mut(id).and_then(|pdf| pdf.shortcut(action)) {
            let task = task.map(move |message| Message::Pdf(id, message));
            return Task::batch([task, self.with_pdf(id, |_| Task::none())]);
        }
        if let Some(task) = self
            .markdown_mut(id)
            .and_then(|document| document.shortcut(action))
        {
            return task.map(move |message| Message::Markdown(id, message));
        }
        match self
            .images_mut(id)
            .and_then(|images| images.shortcut(action))
        {
            Some(task) => task.map(move |message| Message::Image(id, message)),
            None => self.perform(id, action),
        }
    }

    fn perform(&mut self, id: window::Id, action: Action) -> Task<Message> {
        match action {
            Action::Open => Task::perform(dialog::open_files(), move |result| {
                Message::DialogFinished(id, result)
            }),
            Action::CloseWindow => self.update(Message::CloseRequested(id)),
            Action::Quit => {
                // Windows with markup that would be lost ask first.
                let ids: Vec<window::Id> = self.windows.keys().copied().collect();
                let asking: Vec<window::Id> = ids
                    .into_iter()
                    .filter(|id| {
                        self.images_mut(*id)
                            .is_some_and(|images| !images.request_close())
                    })
                    .collect();
                match asking.first() {
                    Some(first) => window::gain_focus(*first),
                    None => {
                        prev::drag::clean_up();
                        quit()
                    }
                }
            }
            Action::Settings => {
                self.reset_storage_drafts();
                if let Some(window) = self.windows.get_mut(&id) {
                    window.settings_open = true;
                }
                Task::none()
            }
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
                .map(|error| prev::fl!("settings-save-failed", error = error.to_string())),
            None => Some(prev::fl!("settings-no-location")),
        };
    }

    pub fn title(&self, id: window::Id) -> String {
        let title = self.base_title(id);
        if prev_store::paths::PRODUCTION {
            title
        } else {
            prev::fl!("app-title-dev", title = title)
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
            Subscription::run(prev::input::keyboard_changes).map(Message::KeyboardDirection),
            window::close_events().map(Message::WindowClosed),
            window::close_requests().map(Message::CloseRequested),
            event::listen_with(|event, status, id| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }),
                    event::Status::Ignored,
                ) => Some(Message::Key(id, key, modifiers)),
                (Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)), _) => {
                    Some(Message::Modifiers(id, modifiers))
                }
                (
                    Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),
                    _,
                ) => Some(Message::MouseReleased(id)),
                (Event::Mouse(iced::mouse::Event::CursorEntered), _) => {
                    Some(Message::Pointer(id, true))
                }
                (Event::Mouse(iced::mouse::Event::CursorLeft), _) => {
                    Some(Message::Pointer(id, false))
                }
                (Event::Window(window::Event::Rescaled(scale)), _) => {
                    Some(Message::ScaleFactor(id, scale))
                }
                (Event::Window(window::Event::Focused), _) => Some(Message::WindowFocused(id)),
                (Event::Window(window::Event::Resized(size)), _) => {
                    Some(Message::Resized(id, size))
                }
                (Event::Window(window::Event::FileDropped(path)), _) => {
                    Some(Message::FileDropped(id, path))
                }
                (Event::Mouse(iced::mouse::Event::CursorMoved { position }), _)
                    if prev::drag::watching_pointer() =>
                {
                    Some(Message::PointerMoved(id, position))
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
        };
        // Layers over the window are always in the tree, empty when not
        // shown, so opening one keeps the state underneath, such as the
        // scroll position.
        let full = || Element::from(space().width(Fill).height(Fill));
        let notice = match &window.notice {
            Some(notice) => component::snackbar(full(), notice, Message::DismissNotice(id)),
            None => full(),
        };
        let drop = if window.drag_hover {
            drop_highlight()
        } else {
            full()
        };
        let settings = if window.settings_open {
            self.settings_dialog(id)
        } else {
            full()
        };
        ui::smooth::smooth(stack![body, notice, drop, settings])
    }

    /// Motion is on only when both the settings and the system allow it.
    fn apply_motion(&self) {
        ui::motion::set_reduced(!self.settings.animations || !self.system_animations);
    }

    fn reset_storage_drafts(&mut self) {
        for storage in Storage::ALL {
            self.storage_drafts[storage.index()] = shown_path(storage.path(&self.settings));
            self.storage_errors[storage.index()] = None;
        }
    }

    /// Keeps `storage` at `path` from now on, in the settings file too,
    /// once the place checks out; otherwise says why under its row.
    fn set_storage(&mut self, storage: Storage, path: PathBuf) {
        let path = prev_store::paths::expand_home(&path);
        let path = match storage {
            // A folder given for the bookmarks file gets the file inside it.
            Storage::Bookmarks if path.is_dir() => path.join("bookmarks.toml"),
            _ => path,
        };
        if let Err(problem) = check_storage(storage, &path) {
            self.storage_errors[storage.index()] = Some(problem);
            return;
        }
        match storage {
            Storage::Signatures => self.settings.signatures = path,
            Storage::Versions => self.settings.versions = path,
            Storage::Bookmarks => self.settings.bookmarks = path,
        }
        prev_store::paths::set_locations(self.settings.locations());
        self.save_settings();
        self.storage_errors[storage.index()] = None;
        self.reset_storage_drafts();
    }

    /// Where prev keeps its files. Each path can be typed, then applied
    /// with Enter or Apply, or chosen with the folder dialog.
    fn storage_view(&self) -> Element<'_, Message> {
        let mut rows = column![].spacing(12);
        for storage in Storage::ALL {
            let draft = &self.storage_drafts[storage.index()];
            let changed = *draft != shown_path(storage.path(&self.settings));
            let mut entry = prev::line![
                container(component::text_field(
                    storage.label(),
                    draft,
                    Backdrop::ContainerHigh,
                    move |input| {
                        input
                            .on_input(move |text| Message::StorageDraft(storage, text))
                            .on_submit(Message::StorageApply(storage))
                    },
                ))
                .width(Fill),
            ]
            .spacing(8)
            .align_y(Center);
            if changed {
                entry = entry.push(
                    ui::button(Kind::Filled, prev::fl!("settings-storage-apply"))
                        .on_press(Message::StorageApply(storage)),
                );
            }
            entry = entry.push(
                ui::button(Kind::Tonal, prev::fl!("settings-storage-choose"))
                    .on_press(Message::StorageChoose(storage)),
            );
            rows = rows.push(entry);
            if let Some(problem) = &self.storage_errors[storage.index()] {
                rows = rows.push(ui::aligned(
                    ui::styled(problem.as_str(), Type::BodySmall).style(style::error_text),
                ));
            }
        }
        let file = self
            .settings_path
            .as_deref()
            .map(shown_path)
            .unwrap_or_default();
        rows.push(
            ui::styled(
                prev::fl!("settings-storage-note", file = file),
                Type::BodySmall,
            )
            .style(style::on_surface_variant),
        )
        .into()
    }

    /// Settings as a dialog over window `id`. They apply to every window.
    /// The interface's language: the system's, or one prev has text for,
    /// each named in its own language.
    fn language_field(&self) -> Element<'_, Message> {
        let system = LanguageChoice {
            tag: SYSTEM_LANGUAGE.to_owned(),
            label: prev::fl!(
                "settings-language-system",
                language = prev::i18n::system_language_name()
            ),
        };
        let mut choices = vec![system];
        choices.extend(
            prev::i18n::languages()
                .iter()
                .map(|(tag, name)| LanguageChoice {
                    tag: tag.clone(),
                    label: name.clone(),
                }),
        );
        // A language prev no longer has shows as following the system.
        let selected = choices
            .iter()
            .find(|choice| choice.tag == self.settings.language)
            .unwrap_or(&choices[0])
            .clone();
        pick_list(choices, Some(selected), |choice: LanguageChoice| {
            Message::LanguageSelected(choice.tag)
        })
        .font(ui::font::TEXT)
        .text_size(16)
        .padding([10, 12])
        .width(Fill)
        .right_to_left(ui::dir::mirrored())
        .style(style::outlined_select)
        .menu_style(style::select_menu)
        .into()
    }

    /// The language typed into text fields: the keyboard layout's, or one
    /// of the languages in `INPUT_LANGUAGES`, each named in itself.
    fn input_language_field(&self) -> Element<'_, Message> {
        let mut choices = vec![LanguageChoice {
            tag: SYSTEM_LANGUAGE.to_owned(),
            label: prev::fl!("settings-input-language-system"),
        }];
        choices.extend(INPUT_LANGUAGES.iter().map(|(tag, name)| LanguageChoice {
            tag: (*tag).to_owned(),
            label: (*name).to_owned(),
        }));
        let selected = choices
            .iter()
            .find(|choice| choice.tag == self.settings.input_language)
            .unwrap_or(&choices[0])
            .clone();
        pick_list(choices, Some(selected), |choice: LanguageChoice| {
            Message::InputLanguageSelected(choice.tag)
        })
        .font(ui::font::TEXT)
        .text_size(16)
        .padding([10, 12])
        .width(Fill)
        .right_to_left(ui::dir::mirrored())
        .style(style::outlined_select)
        .menu_style(style::select_menu)
        .into()
    }

    /// Sets the side empty text fields start on: the chosen input
    /// language's, else the keyboard layout's, else the interface's.
    fn apply_input_direction(&self) {
        let right_to_left = match self.settings.chosen_input_language() {
            Some(tag) => prev::i18n::is_right_to_left(tag),
            None => self.keyboard_rtl.unwrap_or_else(prev::i18n::right_to_left),
        };
        ui::dir::set_input_right_to_left(right_to_left);
    }

    fn settings_dialog(&self, id: window::Id) -> Element<'_, Message> {
        let appearance = self.settings.appearance;
        let choice = |glyph: Icon, label: String, value: Appearance| {
            ui::with_icon(Kind::Tonal, glyph, label)
                .selected(appearance == value)
                .on_press(Message::AppearanceSelected(value))
        };
        let omarchy_note = match &self.omarchy {
            Some(palette) => prev::fl!("settings-omarchy-note", theme = palette.name.as_str()),
            None => prev::fl!("settings-omarchy-none"),
        };
        let mut content = column![
            row![
                ui::aligned(ui::styled(prev::fl!("settings-title"), Type::HeadlineSmall)),
                component::tip(
                    ui::icon_button(Icon::Close).on_press(Message::CloseSettings(id)),
                    prev::fl!("common-close")
                ),
            ]
            .align_y(Center),
            component::section(prev::fl!("settings-appearance")),
            component::connected(vec![
                choice(
                    Icon::Settings,
                    prev::fl!("settings-appearance-system"),
                    Appearance::System
                ),
                choice(
                    Icon::LightMode,
                    prev::fl!("settings-appearance-light"),
                    Appearance::Light
                ),
                choice(
                    Icon::DarkMode,
                    prev::fl!("settings-appearance-dark"),
                    Appearance::Dark
                ),
            ]),
            component::section(prev::fl!("settings-language")),
            self.language_field(),
            component::section(prev::fl!("settings-input-language")),
            self.input_language_field(),
            ui::aligned(
                ui::styled(prev::fl!("settings-input-language-note"), Type::BodySmall)
                    .style(style::on_surface_variant)
            ),
            component::section(prev::fl!("settings-colors")),
            row![
                column![
                    ui::styled(prev::fl!("settings-omarchy-accent"), Type::BodyLarge),
                    ui::aligned(
                        ui::styled(omarchy_note, Type::BodyMedium).style(style::on_surface_variant)
                    ),
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
            component::section(prev::fl!("settings-windows")),
            row![
                column![
                    ui::styled(prev::fl!("settings-auto-hide"), Type::BodyLarge),
                    ui::aligned(
                        ui::styled(prev::fl!("settings-auto-hide-note"), Type::BodyMedium)
                            .style(style::on_surface_variant)
                    ),
                ]
                .spacing(2)
                .width(Fill),
                toggler(self.settings.auto_hide_toolbar)
                    .on_toggle(Message::AutoHideToolbarToggled)
                    .size(28)
                    .style(style::switch),
            ]
            .spacing(16)
            .align_y(Center),
            row![
                column![
                    ui::styled(prev::fl!("settings-animations"), Type::BodyLarge),
                    ui::styled(
                        if self.system_animations {
                            prev::fl!("settings-animations-note")
                        } else {
                            prev::fl!("settings-animations-reduced")
                        },
                        Type::BodyMedium
                    )
                    .style(style::on_surface_variant),
                ]
                .spacing(2)
                .width(Fill),
                toggler(self.settings.animations)
                    .on_toggle(Message::AnimationsToggled)
                    .size(28)
                    .style(style::switch),
            ]
            .spacing(16)
            .align_y(Center),
            row![
                column![
                    ui::styled(prev::fl!("settings-corner-radius"), Type::BodyLarge),
                    ui::aligned(
                        ui::styled(prev::fl!("settings-corner-radius-note"), Type::BodyMedium)
                            .style(style::on_surface_variant)
                    ),
                ]
                .spacing(2)
                .width(Fill),
                ui::styled(
                    prev::fl!(
                        "settings-corner-radius-value",
                        radius = format!("{:.0}", self.settings.corner_radius)
                    ),
                    Type::LabelLarge
                )
                .style(style::on_surface_variant),
                iced::widget::slider(
                    0.0..=32.0,
                    self.settings.corner_radius,
                    Message::CornerRadius
                )
                .step(1.0_f32)
                .on_release(Message::SliderReleased)
                .width(160)
                .height(style::SLIDER_HEIGHT)
                .style(|theme: &Theme, status| {
                    style::slider(Backdrop::ContainerHigh.color(&ui::Scheme::of(theme)))(
                        theme, status,
                    )
                }),
            ]
            .spacing(16)
            .align_y(Center),
            row![
                column![
                    ui::styled(prev::fl!("settings-overlay"), Type::BodyLarge),
                    ui::aligned(
                        ui::styled(prev::fl!("settings-overlay-note"), Type::BodyMedium)
                            .style(style::on_surface_variant)
                    ),
                ]
                .spacing(2)
                .width(Fill),
                ui::styled(
                    prev::fl!(
                        "settings-overlay-value",
                        percent = format!("{:.0}", self.settings.overlay_transparency)
                    ),
                    Type::LabelLarge
                )
                .style(style::on_surface_variant),
                iced::widget::slider(
                    0.0..=90.0,
                    self.settings.overlay_transparency,
                    Message::OverlayTransparency
                )
                .step(5.0_f32)
                .on_release(Message::SliderReleased)
                .width(160)
                .height(style::SLIDER_HEIGHT)
                .style(|theme: &Theme, status| {
                    style::slider(Backdrop::ContainerHigh.color(&ui::Scheme::of(theme)))(
                        theme, status,
                    )
                }),
            ]
            .spacing(16)
            .align_y(Center),
            component::section(prev::fl!("settings-storage")),
            self.storage_view(),
        ]
        .spacing(12);
        if let Some(error) = &self.settings_error {
            content = content.push(ui::aligned(
                ui::styled(error, Type::BodyMedium).style(style::error_text),
            ));
        }
        let version = env!("CARGO_PKG_VERSION");
        let build = if prev_store::paths::PRODUCTION {
            prev::fl!("settings-version", version = version)
        } else {
            prev::fl!("settings-version-development", version = version)
        };
        content = content.push(
            container(ui::styled(build, Type::BodySmall).style(style::on_surface_variant))
                .padding(iced::Padding::ZERO.top(12))
                .center_x(Fill),
        );
        // Scrolls when the window is too short for all of it.
        let card = container(component::scroll(container(content).padding(24)))
            .width(Length::Fixed(520.0))
            .style(style::dialog);
        // A click on the dimmed window around the dialog closes it.
        mouse_area(
            container(ui::enter::grow(opaque(card)))
                .padding(24)
                .center(Fill)
                .style(style::scrim),
        )
        .on_press(Message::CloseSettings(id))
        .into()
    }
}

/// Languages for the input language setting, named in themselves. The
/// setting only sets the side an empty field starts on, so any language
/// can be offered, not only those prev has text for.
const INPUT_LANGUAGES: &[(&str, &str)] = &[
    ("ar", "العربية"),
    ("de", "Deutsch"),
    ("el", "Ελληνικά"),
    ("en", "English"),
    ("es", "Español"),
    ("fa", "فارسی"),
    ("fr", "Français"),
    ("he", "עברית"),
    ("hi", "हिन्दी"),
    ("it", "Italiano"),
    ("ja", "日本語"),
    ("ko", "한국어"),
    ("nl", "Nederlands"),
    ("pl", "Polski"),
    ("ps", "پښتو"),
    ("pt", "Português"),
    ("ru", "Русский"),
    ("tr", "Türkçe"),
    ("uk", "Українська"),
    ("ur", "اردو"),
    ("yi", "ייִדיש"),
    ("zh", "中文"),
];

/// An entry of the Settings language list.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LanguageChoice {
    tag: String,
    label: String,
}

impl std::fmt::Display for LanguageChoice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// Ends the app, stopping the clipboard worker first.
fn quit<T>() -> Task<T> {
    prev::dnd::shutdown_clipboard();
    iced::exit()
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
                ui::styled(shortcuts::platform_label(label), Type::LabelLarge),
            ]
            .spacing(8)
            .into()
        });
    container(
        column![
            icon::filled(Icon::Draft, 64).style(style::primary_text),
            ui::styled("prev", Type::DisplaySmall),
            ui::styled(prev::fl!("app-start-hint"), Type::BodyLarge)
                .style(style::on_surface_variant),
            ui::with_icon(Kind::Filled, Icon::FolderOpen, prev::fl!("app-start-open"))
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
        Ok(Some(kind)) => prev::fl!("app-viewer-missing", kind = kind_name(*kind)),
        Ok(None) => prev::fl!("app-cannot-open"),
        Err(error) => prev::fl!("app-cannot-read", error = error.to_string()),
    };
    let name = document.path.file_name().map_or_else(
        || document.path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    container(component::empty_state(Icon::BrokenImage, name, status))
        .style(style::surface)
        .into()
}

fn drop_highlight<'a>() -> Element<'a, Message> {
    container(space())
        .width(Fill)
        .height(Fill)
        .style(style::drop_target)
        .into()
}

/// Asks for the monitor's size, to shrink a new window that would not fit
/// it. Linux compositors place and size windows themselves.
#[cfg(not(target_os = "linux"))]
fn fit_to_monitor(id: window::Id) -> Task<Message> {
    window::monitor_size(id).map(move |size| Message::MonitorSize(id, size))
}

#[cfg(target_os = "linux")]
fn fit_to_monitor(_id: window::Id) -> Task<Message> {
    Task::none()
}

/// On Linux, the app id desktops match windows and launchers by.
#[cfg(target_os = "linux")]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific {
        application_id: APP_ID.to_owned(),
        ..PlatformSpecific::default()
    }
}

/// On Windows, prev's own OLE drop target takes drops instead of winit's.
#[cfg(windows)]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific {
        drag_and_drop: false,
        ..PlatformSpecific::default()
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific::default()
}

/// The window's `wl_surface` pointer on Wayland, or its HWND on Windows:
/// what drag events name the window by.
fn wayland_surface(window: &dyn window::Window) -> Option<usize> {
    match window.window_handle().ok()?.as_raw() {
        RawWindowHandle::Wayland(handle) => Some(handle.surface.as_ptr() as usize),
        RawWindowHandle::Win32(handle) => Some(handle.hwnd.get() as usize),
        _ => None,
    }
}

fn action_name(action: Action) -> String {
    match action {
        Action::Open => prev::fl!("action-open"),
        Action::Settings => prev::fl!("action-settings"),
        _ => String::new(),
    }
}

fn kind_name(kind: FileKind) -> String {
    match kind {
        FileKind::Pdf => prev::fl!("app-kind-pdf"),
        FileKind::Image(format) => {
            prev::fl!("app-kind-image", format = prev::i18n::format_name(format))
        }
        FileKind::Svg => prev::fl!("app-kind-svg"),
        FileKind::Markdown => prev::fl!("app-kind-markdown"),
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
                ..Settings::default()
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

#[cfg(test)]
mod storage_tests {
    use super::*;

    #[test]
    fn storage_paths_must_exist_and_be_writable() {
        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join("signatures");
        assert!(
            check_storage(Storage::Signatures, &folder).is_err(),
            "missing folder"
        );
        std::fs::create_dir(&folder).unwrap();
        assert!(check_storage(Storage::Signatures, &folder).is_ok());
        assert!(check_storage(Storage::Signatures, std::path::Path::new("relative")).is_err());
        let file = dir.path().join("note.txt");
        std::fs::write(&file, b"").unwrap();
        assert!(check_storage(Storage::Versions, &file).is_err(), "a file");
        // Bookmarks are a file in an existing folder, new or not.
        assert!(check_storage(Storage::Bookmarks, &folder.join("bookmarks.toml")).is_ok());
        assert!(
            check_storage(Storage::Bookmarks, &folder).is_err(),
            "a folder"
        );
        assert!(
            check_storage(
                Storage::Bookmarks,
                &dir.path().join("missing/bookmarks.toml")
            )
            .is_err()
        );
        assert!(
            check_storage(Storage::Signatures, std::path::Path::new("/proc/prev-test")).is_err()
        );
        // Nothing is left behind by the write test.
        assert_eq!(std::fs::read_dir(&folder).unwrap().count(), 0);
    }
}
