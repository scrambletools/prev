//! The iced application: one window per document, plus start and settings
//! windows.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{container, pick_list, space, stack, text, toggler};
use iced::window::{self, settings::PlatformSpecific};
use iced::{Center, Color, Element, Event, Fill, Size, Subscription, Task, Theme, event};
use prev::default_app;
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

mod agents;
pub(crate) mod assistant;
mod settings_view;
mod tools;

#[cfg(target_os = "linux")]
pub const APP_ID: &str = if prev_store::paths::PRODUCTION {
    "io.github.scrambletools.prev"
} else {
    "io.github.scrambletools.prev.Devel"
};

/// How long a notice shows before it goes away by itself.
const NOTICE_TIME: std::time::Duration = std::time::Duration::from_secs(10);

pub struct Prev {
    /// The window that started the drag under way, told how it ends.
    drag_origin: Option<window::Id>,
    /// Whether Ctrl (Control on macOS) is held, which puts drops in the
    /// window's sidebar: among a document's pages, or with an image
    /// window's images.
    control: bool,
    windows: BTreeMap<window::Id, Window>,
    settings: Settings,
    settings_path: Option<PathBuf>,
    settings_error: Option<String>,
    omarchy_dir: Option<PathBuf>,
    omarchy: Option<omarchy::Palette>,
    /// The desktop's, Windows' or macOS's accent color, when it has one.
    system_accent: Option<Color>,
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
    /// Agents waiting to be allowed to control prev, and the ones turned
    /// away.
    agents: agents::Agents,
    /// The Settings tab shown, kept for the next time Settings opens.
    settings_tab: settings_view::SettingsTab,
    /// The model being added in Settings' Assistant tab.
    model_form: assistant::ModelForm,
    /// The notices each window showed after the last update, so new ones
    /// get their timer.
    notices_seen: BTreeMap<window::Id, Vec<String>>,
    /// Storage paths as typed in the settings dialog, before applying.
    storage_drafts: [String; 3],
    /// Why a typed or chosen path was refused, per storage row.
    storage_errors: [Option<String>; 3],
    /// How the last attempt to make prev the system's default app went.
    /// Not a setting: only the system's own setting changes.
    default_app: Option<Result<default_app::Outcome, default_app::Failure>>,
    /// How many of the file types prev opens now, as last read.
    default_app_status: Option<default_app::Status>,
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
    /// The `wl_surface`, window or content view pointer, used to match
    /// drag events to windows.
    surface: Option<usize>,
    drag_hover: bool,
    /// The settings dialog, shown over this window.
    settings_open: bool,
    /// The window's size, for telling when the pointer leaves it.
    size: Size,
    /// The assistant panel, when open, with the window's chat.
    assistant: Option<assistant::Panel>,
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
    /// A notice has shown for NOTICE_TIME.
    NoticeExpired(window::Id, String),
    /// A drop was read: where it landed, what it brought, and whether Ctrl
    /// sent it to the sidebar.
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
    MakeDefaultApp,
    DefaultAppDone(Result<default_app::Outcome, default_app::Failure>),
    DefaultAppStatus(Option<default_app::Status>),
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
    SystemAccentToggled(bool),
    OutsideControlToggled(bool),
    SettingsTab(settings_view::SettingsTab),
    ModelSettings(assistant::ModelMessage),
    /// Open or close a window's assistant panel.
    ToggleAssistant(window::Id),
    Assistant(window::Id, assistant::PanelMessage),
    /// A task finished with nothing to do.
    Nothing,
    /// Forget the agents allowed to control prev, so they are asked about
    /// again.
    ForgetAgents,
    /// The user allowed the agent asking to control prev, or not.
    AgentAnswered(bool),
    /// A new prompt's buttons now take clicks.
    AgentPromptReady,
    /// The user allowed the tool call waiting on a window, or not.
    RunAnswered(window::Id, bool),
    /// Settings: ask before a tool of this kind runs, or not.
    AskBeforeToggled(tools::Kind, bool),
    /// A tool that waited on something answers.
    ToolAnswer(tools::Answer, Result<tools::Output, prev::control::Error>),
    /// point_at found the text to outline on a page, or not.
    ToolPoint(
        window::Id,
        tools::Answer,
        usize,
        f32,
        Result<prev_pdf::geometry::Rect, prev::control::Error>,
    ),
    /// A tool waiting for an image's markup to open looks again.
    ToolDeferred(tools::Answer, &'static str, serde_json::Value, u32),
    /// apply_redactions counted the marks to apply, or found none.
    ToolRedact(
        window::Id,
        tools::Answer,
        Result<usize, prev::control::Error>,
    ),
    /// replace_image saved the new image, or could not.
    ToolReplaced(
        window::Id,
        tools::Answer,
        Result<(u32, u32), prev::control::Error>,
    ),
    /// A markup tool found the edits to make in a window, then answers.
    ToolMarkup(
        window::Id,
        tools::Route,
        tools::Answer,
        Vec<prev::pdf::viewer::editing::AgentEdit>,
        tools::Output,
    ),
    /// A color picked for the scheme, as "#RRGGBB".
    AccentPicked(String),
    DismissNotice(window::Id),
    AnimationsEnabled(Option<bool>),
    SystemAccent(Option<(u8, u8, u8)>),
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
        // Shortcuts the settings change, before the menu bar shows them.
        let key_problems = shortcuts::configure(&settings.keys);
        for problem in &key_problems {
            eprintln!("prev: {problem}");
        }
        let settings_error =
            settings_error.or_else(|| (!key_problems.is_empty()).then(|| key_problems.join("\n")));
        // Signatures, versions and bookmarks live where the settings say.
        prev_store::paths::set_locations(settings.locations());
        prev::i18n::set_language(settings.chosen_language());
        let mut prev = Self {
            drag_origin: None,
            control: false,
            windows: BTreeMap::new(),
            settings,
            settings_path,
            settings_error,
            omarchy_dir,
            system_accent: None,
            omarchy: None,
            system_mode: iced::theme::Mode::None,
            theme: Theme::Light,
            pending_saves: 0,
            storage_drafts: Default::default(),
            storage_errors: Default::default(),
            default_app: None,
            default_app_status: None,
            system_animations: true,
            keyboard_rtl: None,
            focused: None,
            agents: agents::Agents::default(),
            settings_tab: settings_view::SettingsTab::default(),
            model_form: assistant::ModelForm::default(),
            notices_seen: BTreeMap::new(),
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
        (
            prev,
            Task::batch([task, system, motion, read_system_accent()]),
        )
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
            self.system_accent,
            self.system_mode == iced::theme::Mode::Dark,
        );
        let name = match (&self.omarchy, self.settings.system_accent) {
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
                tasks.push(tools::bring_forward(id));
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
                Effect::Open(paths) => self.open_paths_if_any(paths),
                Effect::ToSidebar(_) => Task::none(),
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
                assistant: None,
            },
        );
        (id, opened.map(Message::WindowOpened))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.update_inner(message);
        Task::batch([task, self.time_notices()])
    }

    /// Every window's notices, as they show now.
    fn notices(&self) -> BTreeMap<window::Id, Vec<String>> {
        self.windows
            .iter()
            .map(|(id, window)| {
                let mut notices: Vec<String> = window.notice.iter().cloned().collect();
                if let Content::Document(document) = &window.content {
                    notices.extend(
                        document
                            .pdf
                            .as_ref()
                            .and_then(|pdf| pdf.notice())
                            .map(str::to_owned),
                    );
                    notices.extend(
                        document
                            .markdown
                            .as_ref()
                            .and_then(|markdown| markdown.notice())
                            .map(str::to_owned),
                    );
                    if let Some(images) = &document.images {
                        notices.extend(images.notices().into_iter().map(str::to_owned));
                    }
                }
                (*id, notices)
            })
            .collect()
    }

    /// Notices that have just appeared go away after NOTICE_TIME, unless
    /// they have been replaced or dismissed by then.
    fn time_notices(&mut self) -> Task<Message> {
        let now = self.notices();
        let mut timers = Vec::new();
        for (id, notices) in &now {
            let seen = self.notices_seen.get(id);
            for notice in notices {
                if seen.is_some_and(|seen| seen.contains(notice)) {
                    continue;
                }
                let (id, notice) = (*id, notice.clone());
                timers.push(Task::perform(
                    prev::image::editor::spawn(|| std::thread::sleep(NOTICE_TIME)),
                    move |_| Message::NoticeExpired(id, notice.clone()),
                ));
            }
        }
        self.notices_seen = now;
        Task::batch(timers)
    }

    /// Clears `notice` from window `id`, wherever it still shows.
    fn expire_notice(&mut self, id: window::Id, notice: &str) {
        let Some(window) = self.windows.get_mut(&id) else {
            return;
        };
        if window.notice.as_deref() == Some(notice) {
            window.notice = None;
        }
        if let Content::Document(document) = &mut window.content {
            if let Some(pdf) = document.pdf.as_mut() {
                pdf.dismiss_notice(notice);
            }
            if let Some(markdown) = document.markdown.as_mut() {
                markdown.dismiss_notice(notice);
            }
            if let Some(images) = document.images.as_mut() {
                images.dismiss_notice(notice);
            }
        }
    }

    fn update_inner(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoticeExpired(id, notice) => {
                self.expire_notice(id, &notice);
                Task::none()
            }
            Message::External(External::OpenPaths(paths)) => self.open_paths(paths),
            Message::External(External::Control(call)) => self.control(call),
            Message::External(External::Assistant(id, heard)) => self.panel_heard(id, heard),
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
                // Back from the system's own dialogs or settings, where the
                // default app may have changed.
                if self
                    .windows
                    .get(&id)
                    .is_some_and(|window| window.settings_open)
                {
                    refresh_default_app_status()
                } else {
                    Task::none()
                }
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
            Message::Pdf(id, pdf_window::Message::ToggleAssistant)
            | Message::Image(id, image_window::Message::ToggleAssistant)
            | Message::Markdown(id, markdown::Message::ToggleAssistant)
            | Message::ToggleAssistant(id) => self.toggle_assistant(id),
            Message::Assistant(id, message) => self.panel_update(id, message),
            Message::Nothing => Task::none(),
            Message::Pdf(id, message) => self.with_pdf(id, |pdf| pdf.update(message)),
            Message::Image(id, message) => {
                // An export with markup finishes as MarkupExported.
                if let image_window::Message::Exported(result)
                | image_window::Message::MarkupExported(_, _, result) = &message
                {
                    self.image_exported(id, result);
                }
                self.with_images(id, |images| images.update(message))
            }
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
                // A change of accent often comes with one of mode.
                read_system_accent()
            }
            Message::SystemAccent(accent) => {
                self.system_accent =
                    accent.map(|(red, green, blue)| Color::from_rgb8(red, green, blue));
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
                // Drops on Windows go to a target registered on each window;
                // on macOS each window registers the types it takes.
                #[cfg(any(windows, target_os = "macos"))]
                if let Some(handle) = surface {
                    prev::dnd::register(handle);
                }
                Task::none()
            }
            Message::WindowClosed(id) => {
                self.window_gone(id);
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
                if let Some(images) = self.images_mut(id) {
                    images.pointer_at(point);
                }
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
                self.drop_in(id, x, y, prev::drag::Action::Copy, dropped, self.control)
            }
            Message::DropDecoded(id, x, y, action, dropped, to_panel) => {
                self.drop_in(id, x, y, action, dropped, to_panel)
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
                // Shift, or Command on macOS, moves dropped pages instead of
                // copying them; Ctrl puts drops in the sidebar.
                self.control = modifiers.control();
                prev::dnd::set_prefer_move(
                    modifiers.shift() || (cfg!(target_os = "macos") && modifiers.logo()),
                );
                if let Some(pdf) = self.pdf_mut(id) {
                    pdf.set_modifiers(modifiers);
                }
                if let Some(images) = self.images_mut(id) {
                    images.set_modifiers(modifiers);
                }
                Task::none()
            }
            Message::StorageDraft(storage, text) => {
                self.storage_drafts[storage.index()] = text;
                self.storage_errors[storage.index()] = None;
                Task::none()
            }
            Message::MakeDefaultApp => {
                self.default_app = None;
                Task::perform(default_app::make_default(), Message::DefaultAppDone)
            }
            Message::DefaultAppDone(result) => {
                self.default_app = Some(result);
                refresh_default_app_status()
            }
            Message::DefaultAppStatus(status) => {
                self.default_app_status = status;
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
            Message::AccentPicked(hex) => {
                self.settings.accent_color = Some(hex);
                self.save_settings();
                self.refresh_theme();
                Task::none()
            }
            Message::SystemAccentToggled(enabled) => {
                self.settings.system_accent = enabled;
                self.save_settings();
                self.refresh_theme();
                Task::none()
            }
            Message::ModelSettings(message) => self.model_settings(message),
            Message::SettingsTab(tab) => {
                self.settings_tab = tab;
                self.assistant_tab_shown()
            }
            Message::OutsideControlToggled(enabled) => {
                self.settings.outside_control = enabled;
                if !enabled {
                    self.outside_control_off();
                }
                self.save_settings();
                Task::none()
            }
            Message::ForgetAgents => {
                self.settings.allowed_agents.clear();
                self.save_settings();
                Task::none()
            }
            Message::AgentAnswered(allow) => self.agent_answered(allow),
            Message::AgentPromptReady => Task::none(),
            Message::RunAnswered(id, allow) => self.run_answered(id, allow),
            Message::AskBeforeToggled(kind, asks) => {
                kind.set_asks(&mut self.settings.ask_before, asks);
                self.save_settings();
                Task::none()
            }
            Message::ToolAnswer(answer, result) => {
                answer.send(result);
                Task::none()
            }
            Message::ToolReplaced(id, answer, result) => self.image_replaced(id, &answer, result),
            Message::ToolRedact(id, answer, marks) => self.redact(id, &answer, marks),
            Message::ToolPoint(id, answer, page, seconds, rect) => {
                self.point(id, &answer, page, seconds, rect)
            }
            Message::ToolDeferred(answer, name, arguments, tries) => match tools::find(name) {
                Some(tool) => tool.start_after(self, arguments, answer, tries),
                None => Task::none(),
            },
            Message::ToolMarkup(id, route, answer, edits, output) => {
                self.agent_markup(id, route, &answer, edits, output)
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
                let to_panel = self.control || prev::dnd::control_held();
                // Pictures joining an image window's images need files.
                let as_file = match &window.content {
                    Content::Document(Document {
                        images: Some(images),
                        ..
                    }) => images.takes_files(x, to_panel),
                    _ => false,
                };
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
                            to_panel,
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
        to_panel: bool,
    ) -> Task<Message> {
        let from_here = self.drag_origin == Some(id);
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let (task, files) = match &mut window.content {
            Content::Document(Document { pdf: Some(pdf), .. }) => {
                let (task, files) = pdf.drop_in(x, y, dropped, action, to_panel);
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
                let (task, files) = images.drop_in(x, y, dropped, action, to_panel);
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

    /// Answers a call on the control channel. The tools come in later;
    /// `ping` says which prev answers.
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
            && let Some(task) = self.answer_prompt(id, false)
        {
            return task;
        }
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
                Task::batch([refresh_default_app_status(), self.assistant_tab_shown()])
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
        // The panel keeps its side in every language, as the inspector does.
        let body = match self.panel_view(id) {
            Some(panel) => iced::widget::row![body, panel].into(),
            None => body,
        };
        let prompt = self.agent_prompt(id).unwrap_or_else(full);
        ui::smooth::smooth(stack![body, notice, drop, settings, prompt])
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

    /// The button that makes prev the system's default app, and how the
    /// last try went. Windows lets only the user choose, in its Settings.
    fn default_app_view(&self) -> Element<'_, Message> {
        let (note, button) = if cfg!(windows) {
            (
                prev::fl!("settings-default-app-note-windows"),
                prev::fl!("settings-default-app-button-windows"),
            )
        } else if cfg!(target_os = "macos") {
            (
                prev::fl!("settings-default-app-note-macos"),
                prev::fl!("settings-default-app-button"),
            )
        } else {
            (
                prev::fl!("settings-default-app-note"),
                prev::fl!("settings-default-app-button"),
            )
        };
        let summary = self.default_app_status.map(|status| {
            prev::fl!(
                "settings-default-app-status",
                set = status.set,
                total = status.total
            )
        });
        let mut action = row![].spacing(12).align_y(Center);
        if let (Some(status), Some(summary)) = (self.default_app_status, &summary) {
            action = action.push(component::tip(status_dot(status), summary.clone()));
        }
        action = action.push(ui::button(Kind::Tonal, button).on_press(Message::MakeDefaultApp));
        let mut rows = column![
            row![
                column![
                    ui::styled(prev::fl!("settings-default-app-label"), Type::BodyLarge),
                    ui::aligned(
                        ui::styled(note, Type::BodyMedium).style(style::on_surface_variant)
                    ),
                ]
                .spacing(2)
                .width(Fill),
                action,
            ]
            .spacing(16)
            .align_y(Center)
        ]
        .spacing(8);
        let problem = match &self.default_app {
            Some(Err(default_app::Failure::NoDesktopEntry)) => {
                Some(prev::fl!("settings-default-app-no-entry"))
            }
            Some(Err(default_app::Failure::NoBundle)) => {
                Some(prev::fl!("settings-default-app-no-bundle"))
            }
            Some(Err(default_app::Failure::Other(error))) => Some(prev::fl!(
                "settings-default-app-failed",
                error = error.as_str()
            )),
            _ => None,
        };
        if let Some(problem) = problem {
            rows = rows.push(ui::aligned(
                ui::styled(problem, Type::BodySmall).style(style::error_text),
            ));
        } else if let Some(summary) = summary {
            rows = rows.push(ui::aligned(
                ui::styled(summary, Type::BodySmall).style(style::on_surface_variant),
            ));
        }
        rows.into()
    }

    /// The switches that make prev ask before an agent's tool of each
    /// kind runs.
    fn ask_before_view(&self) -> Element<'_, Message> {
        let mut rows = column![ui::aligned(
            ui::styled(prev::fl!("settings-ask-before-note"), Type::BodyMedium)
                .style(style::on_surface_variant)
        )]
        .spacing(4);
        for kind in tools::Kind::ALL {
            let label = match kind {
                tools::Kind::Read => prev::fl!("settings-ask-reading"),
                tools::Kind::View => prev::fl!("settings-ask-viewing"),
                tools::Kind::Markup => prev::fl!("settings-ask-marking-up"),
                tools::Kind::Edit => prev::fl!("settings-ask-editing"),
                tools::Kind::Sign => prev::fl!("settings-ask-signing"),
                tools::Kind::Redact => prev::fl!("settings-ask-redacting"),
                tools::Kind::Export => prev::fl!("settings-ask-exporting"),
            };
            rows = rows.push(
                row![
                    container(ui::styled(label, Type::BodyLarge)).width(Fill),
                    toggler(kind.asks(&self.settings.ask_before))
                        .on_toggle(move |asks| Message::AskBeforeToggled(kind, asks))
                        .size(28)
                        .style(style::switch),
                ]
                .spacing(16)
                .align_y(Center),
            );
        }
        rows.into()
    }

    /// The agents allowed to control prev, which Forget asks about again.
    fn allowed_agents_view(&self) -> Element<'_, Message> {
        if self.settings.allowed_agents.is_empty() {
            return space().into();
        }
        row![
            container(ui::aligned(
                ui::styled(
                    prev::fl!(
                        "settings-allowed-agents",
                        agents = self.settings.allowed_agents.join(", ")
                    ),
                    Type::BodyMedium
                )
                .style(style::on_surface_variant)
            ))
            .width(Fill),
            ui::button(Kind::Text, prev::fl!("settings-forget-agents"))
                .on_press(Message::ForgetAgents),
        ]
        .spacing(16)
        .align_y(Center)
        .into()
    }
}

/// Reads again how many file types prev opens.
fn refresh_default_app_status() -> Task<Message> {
    Task::perform(default_app::status(), Message::DefaultAppStatus)
}

/// A dot for how many file types prev opens: red for none, amber for
/// some, green for all. The same in every theme, as traffic lights are.
fn status_dot(status: default_app::Status) -> Element<'static, Message> {
    let color = if status.set == 0 {
        Color::from_rgb8(0xd9, 0x30, 0x25)
    } else if status.set < status.total {
        Color::from_rgb8(0xf2, 0xa9, 0x00)
    } else {
        Color::from_rgb8(0x1e, 0x8e, 0x3e)
    };
    container(iced::widget::space().width(12).height(12))
        .style(move |_: &Theme| container::Style {
            background: Some(color.into()),
            border: iced::border::rounded(6),
            ..container::Style::default()
        })
        .into()
}

/// Languages for the input language setting, named in themselves. The
/// setting only sets the side an empty field starts on, so any language
/// can be offered, not only those prev has text for.
const INPUT_LANGUAGES: &[(&str, &str)] = &[
    ("ar", "العربية"),
    ("bn", "বাংলা"),
    ("ca", "Català"),
    ("cs", "Čeština"),
    ("da", "Dansk"),
    ("de", "Deutsch"),
    ("el", "Ελληνικά"),
    ("en", "English"),
    ("es", "Español"),
    ("fa", "فارسی"),
    ("fi", "Suomi"),
    ("fr", "Français"),
    ("he", "עברית"),
    ("hi", "हिन्दी"),
    ("hu", "Magyar"),
    ("id", "Bahasa Indonesia"),
    ("it", "Italiano"),
    ("ja", "日本語"),
    ("ko", "한국어"),
    ("ms", "Bahasa Melayu"),
    ("nb", "Norsk bokmål"),
    ("nl", "Nederlands"),
    ("pl", "Polski"),
    ("ps", "پښتو"),
    ("pt", "Português"),
    ("ro", "Română"),
    ("ru", "Русский"),
    ("sv", "Svenska"),
    ("sw", "Kiswahili"),
    ("ta", "தமிழ்"),
    ("th", "ไทย"),
    ("tr", "Türkçe"),
    ("uk", "Українська"),
    ("ur", "اردو"),
    ("vi", "Tiếng Việt"),
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

/// Colors offered for the scheme when the system accent is not used,
/// prev's blue first.
const ACCENT_SWATCHES: [Color; 11] = [
    ui::scheme::PREV_SEED,
    Color::from_rgb8(0x00, 0x89, 0x7b),
    Color::from_rgb8(0x43, 0xa0, 0x47),
    Color::from_rgb8(0xf9, 0xa8, 0x25),
    Color::from_rgb8(0xf5, 0x7c, 0x00),
    Color::from_rgb8(0xe5, 0x39, 0x35),
    Color::from_rgb8(0xd8, 0x1b, 0x60),
    Color::from_rgb8(0x8e, 0x24, 0xaa),
    Color::from_rgb8(0x39, 0x49, 0xab),
    Color::from_rgb8(0x6d, 0x4c, 0x41),
    Color::from_rgb8(0x75, 0x75, 0x75),
];

/// The color picked in the settings, or prev's blue.
fn chosen_accent(settings: &Settings) -> Color {
    settings
        .accent_color
        .as_deref()
        .and_then(hex_to_color)
        .unwrap_or(ui::scheme::PREV_SEED)
}

/// "#RRGGBB" (or "RRGGBB") as a color.
fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let value = u32::from_str_radix(hex, 16).ok()?;
    Some(Color::from_rgb8(
        (value >> 16) as u8,
        (value >> 8) as u8,
        value as u8,
    ))
}

fn color_to_hex(color: Color) -> String {
    let [red, green, blue, _] = color.into_rgba8();
    format!("#{red:02X}{green:02X}{blue:02X}")
}

/// A round button that picks `color` for the scheme, ringed when chosen.
fn accent_swatch<'a>(color: Color, selected: bool) -> Element<'a, Message> {
    let dot = container(space().width(24).height(24)).style(move |theme: &Theme| {
        let scheme = ui::Scheme::of(theme);
        iced::widget::container::Style {
            background: Some(color.into()),
            border: iced::Border {
                color: if selected {
                    scheme.on_surface
                } else {
                    scheme.outline_variant
                },
                width: if selected { 3.0 } else { 1.0 },
                radius: ui::shape::FULL.into(),
            },
            ..Default::default()
        }
    });
    let hex = color_to_hex(color);
    component::tip(
        button::custom(Kind::Standard, dot)
            .size(button::Size::ExtraSmall)
            .width(36)
            .on_press(Message::AccentPicked(hex.clone())),
        hex,
    )
}

/// Reads the system's accent color, for the scheme's seed.
fn read_system_accent() -> Task<Message> {
    Task::perform(portal::accent_color(), Message::SystemAccent)
}

/// The seed color and whether the scheme is dark. With the system accent
/// on, the Omarchy theme's accent is the seed, else the system's own, else
/// prev's; in "Follow system" the Omarchy theme's own mode wins over the
/// system's.
fn theme_choice(
    settings: &Settings,
    omarchy: Option<&omarchy::Palette>,
    system_accent: Option<Color>,
    system_dark: bool,
) -> (Color, bool) {
    let omarchy = omarchy.filter(|_| settings.system_accent);
    let dark = match (settings.appearance, omarchy) {
        (Appearance::System, Some(palette)) => palette.mode == omarchy::Mode::Dark,
        (Appearance::System, None) => system_dark,
        (Appearance::Light, _) => false,
        (Appearance::Dark, _) => true,
    };
    let seed = match omarchy {
        Some(palette) => {
            let accent = palette.accent;
            Color::from_rgb8(accent.red, accent.green, accent.blue)
        }
        None => system_accent
            .filter(|_| settings.system_accent)
            .unwrap_or_else(|| chosen_accent(settings)),
    };
    (seed, dark)
}

fn start_view(id: window::Id) -> Element<'static, Message> {
    let hints = [Action::Open, Action::Settings]
        .into_iter()
        .filter_map(|action| Some((action, shortcuts::label(action)?)))
        .map(|(action, label)| {
            row![
                ui::styled(action_name(action), Type::BodyMedium).style(style::on_surface_variant),
                ui::styled(label, Type::LabelLarge),
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
        RawWindowHandle::AppKit(handle) => Some(handle.ns_view.as_ptr() as usize),
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
        system_accent: bool,
        omarchy: Option<&omarchy::Palette>,
        system_dark: bool,
    ) -> (Color, bool) {
        theme_choice(
            &Settings {
                appearance,
                system_accent,
                ..Settings::default()
            },
            omarchy,
            None,
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
    fn system_accent_seeds_the_scheme_without_omarchy() {
        let accent = Color::from_rgb8(0x00, 0x78, 0xd4);
        let with = |on: bool, omarchy: Option<&omarchy::Palette>| {
            theme_choice(
                &Settings {
                    system_accent: on,
                    ..Settings::default()
                },
                omarchy,
                Some(accent),
                false,
            )
            .0
        };
        assert_eq!(with(true, None), accent);
        assert_eq!(with(false, None), ui::scheme::PREV_SEED);
        assert_eq!(
            with(true, Some(&omarchy_dark())),
            Color::from_rgb8(0x82, 0xfb, 0x9c),
            "the Omarchy accent comes first"
        );
    }

    #[test]
    fn picked_color_seeds_the_scheme_without_the_system_accent() {
        let red = Color::from_rgb8(0xe5, 0x39, 0x35);
        let settings = |system_accent: bool| Settings {
            system_accent,
            accent_color: Some("#E53935".into()),
            ..Settings::default()
        };
        let blue = Color::from_rgb8(0x00, 0x78, 0xd4);
        assert_eq!(
            theme_choice(&settings(false), None, Some(blue), false).0,
            red
        );
        assert_eq!(
            theme_choice(&settings(true), None, None, false).0,
            red,
            "no system accent"
        );
        assert_eq!(
            theme_choice(&settings(true), None, Some(blue), false).0,
            blue
        );
        assert_eq!(hex_to_color("#e53935"), Some(red));
        assert_eq!(color_to_hex(red), "#E53935");
        assert_eq!(hex_to_color("nope"), None);
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
