//! A window rendering a Markdown file, reloading when the file changes,
//! with a toolbar for the text size, search and the inspector like the
//! other windows have.

mod code;
mod find;

use std::cell::Cell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use iced::futures::channel::{mpsc, oneshot};
use iced::widget::image::Handle;
use iced::widget::{
    Id, column, container, image, markdown, operation, rich_text, row, scrollable, text, text_input,
};
use iced::{Center, Element, Fill, Padding, Task, Theme, padding};

use crate::filetype::{self, FileKind};
use crate::info;
use crate::portal;
use crate::shortcuts::Action;
use crate::ui::{self, Icon, Type, button, component, style};

const POLL_INTERVAL: Duration = Duration::from_millis(700);
/// Widest the text runs at the normal size; it grows with the text.
const MAX_WIDTH: f32 = 860.0;
/// Text sizes to step through; `NORMAL_SIZE` is 100%.
const SIZES: &[f32] = &[10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 24.0, 28.0, 32.0, 40.0];
const NORMAL_SIZE: usize = 3;
const SEARCH_WIDTH: f32 = 240.0;

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<String, String>),
    FileChanged,
    LinkClicked(String),
    ImageLoaded(String, Option<Picture>),
    UriOpened(Result<(), String>),
    DismissNotice,
    ZoomIn,
    ZoomOut,
    ActualSize,
    SearchChanged(String),
    NextMatch,
    PreviousMatch,
    /// How far to scroll to show the current match, if it is out of view.
    Reveal(Option<f32>),
    ToggleInspector,
    /// Handled by the app, as in the other windows.
    ToggleFloatingBars,
    OpenSettings,
}

/// Changes the app applies on the window's behalf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    OpenPath(PathBuf),
}

pub struct MarkdownWindow {
    path: PathBuf,
    items: Vec<markdown::Item>,
    /// Words and lines in the file, for the inspector.
    counts: (usize, usize),
    error: Option<String>,
    images: HashMap<String, Picture>,
    images_requested: std::collections::HashSet<String>,
    notice: Option<String>,
    effects: Vec<Effect>,
    watching: Arc<AtomicBool>,
    size: usize,
    query: String,
    current: Option<usize>,
    /// Matches found when the document was last drawn.
    found: Cell<usize>,
    highlights: code::Highlights,
    inspector: bool,
    pointer_inside: bool,
    scroll_id: Id,
    search_id: Id,
}

impl Drop for MarkdownWindow {
    fn drop(&mut self) {
        self.watching.store(false, Ordering::Relaxed);
    }
}

fn read(path: PathBuf) -> oneshot::Receiver<Result<String, String>> {
    let (sender, receiver) = oneshot::channel();
    std::thread::spawn(move || {
        let _ = sender.send(std::fs::read_to_string(&path).map_err(|error| error.to_string()));
    });
    receiver
}

fn file_signature(path: &Path) -> Option<(std::time::SystemTime, u64)> {
    let metadata = std::fs::metadata(path).ok()?;
    Some((metadata.modified().ok()?, metadata.len()))
}

/// Polls the file on a thread and yields when it changes, until `watching`
/// is cleared.
fn watch(path: PathBuf, watching: Arc<AtomicBool>) -> mpsc::UnboundedReceiver<()> {
    let (sender, receiver) = mpsc::unbounded();
    std::thread::Builder::new()
        .name("prev-markdown-watch".into())
        .spawn(move || {
            let mut last = file_signature(&path);
            while watching.load(Ordering::Relaxed) {
                std::thread::sleep(POLL_INTERVAL);
                let current = file_signature(&path);
                if current != last {
                    last = current;
                    if sender.unbounded_send(()).is_err() {
                        break;
                    }
                }
            }
        })
        .expect("spawn markdown watcher");
    receiver
}

/// Image URLs anywhere in the document.
fn image_urls(items: &[markdown::Item], urls: &mut Vec<String>) {
    for item in items {
        match item {
            markdown::Item::Image { url, .. } => urls.push(url.clone()),
            markdown::Item::Quote(children) => image_urls(children, urls),
            markdown::Item::List { bullets, .. } => {
                for bullet in bullets {
                    let (markdown::Bullet::Point { items } | markdown::Bullet::Task { items, .. }) =
                        bullet;
                    image_urls(items, urls);
                }
            }
            _ => {}
        }
    }
}

/// Resolves a link or image reference against the document's folder.
/// Web and other scheme URLs return `None`.
fn local_target(document: &Path, reference: &str) -> Option<PathBuf> {
    let reference = reference.split('#').next()?;
    if reference.is_empty() {
        return None;
    }
    if let Some(path) = reference.strip_prefix("file://") {
        return Some(PathBuf::from(path));
    }
    if reference.contains("://") || reference.starts_with("mailto:") {
        return None;
    }
    let base = document.parent().unwrap_or(Path::new("."));
    Some(base.join(reference))
}

/// A picture in the document and the width it is drawn at, in pixels at
/// the normal text size.
#[derive(Debug, Clone)]
pub struct Picture {
    handle: Handle,
    width: f32,
}

fn load_image(path: PathBuf) -> Option<Picture> {
    match filetype::detect_path(&path).ok()?? {
        FileKind::Image(format) => {
            let decoded = prev_image::decode::decode_file(&path, format).ok()?;
            let frame = decoded.frames.into_iter().next()?;
            Some(Picture {
                width: frame.width as f32,
                handle: Handle::from_rgba(frame.width, frame.height, frame.pixels),
            })
        }
        FileKind::Svg => {
            // Drawn twice as sharp as its own size, for high density screens.
            let svg = prev_image::svg::Svg::parse_file(&path).ok()?;
            let frame = svg.render(2.0).ok()?;
            Some(Picture {
                width: svg.size().0,
                handle: Handle::from_rgba(frame.width, frame.height, frame.pixels),
            })
        }
        _ => None,
    }
}

impl MarkdownWindow {
    pub fn open(path: PathBuf) -> (Self, Task<Message>) {
        let watching = Arc::new(AtomicBool::new(true));
        let window = Self {
            path: path.clone(),
            items: Vec::new(),
            counts: (0, 0),
            error: None,
            images: HashMap::new(),
            images_requested: Default::default(),
            notice: None,
            effects: Vec::new(),
            watching: Arc::clone(&watching),
            size: NORMAL_SIZE,
            query: String::new(),
            current: None,
            found: Cell::new(0),
            highlights: code::Highlights::default(),
            inspector: false,
            pointer_inside: true,
            scroll_id: Id::unique(),
            search_id: Id::unique(),
        };
        let load = Task::perform(read(path.clone()), |result| {
            Message::Loaded(result.unwrap_or_else(|_| Err("reading stopped".into())))
        });
        let changes = Task::run(watch(path, watching), |()| Message::FileChanged);
        (window, Task::batch([load, changes]))
    }

    pub fn take_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }

    pub fn set_pointer_inside(&mut self, inside: bool) {
        self.pointer_inside = inside;
    }

    fn zoom(&self) -> f32 {
        SIZES[self.size] / SIZES[NORMAL_SIZE]
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok(source)) => {
                self.items = markdown::parse(&source).collect();
                self.counts = (source.split_whitespace().count(), source.lines().count());
                self.error = None;
                self.highlights.clear();
                let mut urls = Vec::new();
                image_urls(&self.items, &mut urls);
                // A reload may change images, so load them all again.
                self.images_requested.clear();
                Task::batch(urls.into_iter().map(|url| self.request_image(url)))
            }
            Message::Loaded(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::FileChanged => Task::perform(read(self.path.clone()), |result| {
                Message::Loaded(result.unwrap_or_else(|_| Err("reading stopped".into())))
            }),
            Message::ImageLoaded(url, picture) => {
                match picture {
                    Some(picture) => {
                        self.images.insert(url, picture);
                    }
                    None => {
                        self.images.remove(&url);
                    }
                }
                Task::none()
            }
            Message::LinkClicked(url) => match local_target(&self.path, &url) {
                Some(path) => {
                    self.effects.push(Effect::OpenPath(path));
                    Task::none()
                }
                None if url.starts_with('#') => Task::none(),
                None => Task::perform(portal::open_uri(url), Message::UriOpened),
            },
            Message::DismissNotice => {
                self.notice = None;
                Task::none()
            }
            Message::UriOpened(result) => {
                self.notice = result.err();
                Task::none()
            }
            Message::ZoomIn => {
                self.size = (self.size + 1).min(SIZES.len() - 1);
                self.reveal()
            }
            Message::ZoomOut => {
                self.size = self.size.saturating_sub(1);
                self.reveal()
            }
            Message::ActualSize => {
                self.size = NORMAL_SIZE;
                self.reveal()
            }
            Message::SearchChanged(query) => {
                self.current = (!query.trim().is_empty()).then_some(0);
                self.query = query;
                self.reveal()
            }
            Message::NextMatch => self.step_match(1),
            Message::PreviousMatch => self.step_match(-1),
            Message::Reveal(Some(y)) => operation::scroll_to(
                self.scroll_id.clone(),
                scrollable::AbsoluteOffset { x: 0.0, y },
            ),
            Message::Reveal(None) => Task::none(),
            Message::ToggleInspector => {
                self.inspector = !self.inspector;
                Task::none()
            }
            Message::ToggleFloatingBars | Message::OpenSettings => Task::none(),
        }
    }

    /// Moves to the next or previous match, going round at the ends.
    fn step_match(&mut self, step: isize) -> Task<Message> {
        let found = self.found.get();
        if found == 0 {
            return Task::none();
        }
        let current = self.current.unwrap_or(0) as isize;
        self.current = Some((current + step).rem_euclid(found as isize) as usize);
        self.reveal()
    }

    /// Scrolls the current match into view, once the document is laid out
    /// with it marked.
    fn reveal(&self) -> Task<Message> {
        if self.current.is_none() {
            return Task::none();
        }
        iced::advanced::widget::operate(find::Reveal::new(self.scroll_id.clone()))
            .map(Message::Reveal)
    }

    pub fn shortcut(&mut self, action: Action) -> Option<Task<Message>> {
        let task = match action {
            Action::ZoomIn => self.update(Message::ZoomIn),
            Action::ZoomOut => self.update(Message::ZoomOut),
            Action::ActualSize | Action::ZoomToFit => self.update(Message::ActualSize),
            Action::Find => operation::focus(self.search_id.clone()),
            Action::FindNext => self.update(Message::NextMatch),
            Action::FindPrevious => self.update(Message::PreviousMatch),
            Action::Inspector => self.update(Message::ToggleInspector),
            Action::Escape if self.inspector => {
                self.inspector = false;
                Task::none()
            }
            Action::Escape if !self.query.is_empty() => {
                self.update(Message::SearchChanged(String::new()))
            }
            _ => return None,
        };
        Some(task)
    }

    fn request_image(&mut self, url: String) -> Task<Message> {
        if !self.images_requested.insert(url.clone()) {
            return Task::none();
        }
        let Some(path) = local_target(&self.path, &url) else {
            return Task::none();
        };
        let (sender, receiver) = oneshot::channel();
        std::thread::spawn(move || {
            let _ = sender.send(load_image(path));
        });
        Task::perform(receiver, move |picture| {
            Message::ImageLoaded(url, picture.ok().flatten())
        })
    }

    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        let body: Element<'_, Message> = match &self.error {
            Some(error) => container(component::empty_state(
                Icon::Error,
                "prev can't read this file",
                error.as_str(),
            ))
            .width(Fill)
            .height(Fill)
            .style(style::surface)
            .into(),
            None => self.document(theme),
        };
        let mut content = row![body].height(Fill);
        if self.inspector {
            content = content.push(component::between_bars(
                component::side_sheet("Inspector", Message::ToggleInspector, self.inspector_view()),
                component::floating_room(true),
                0.0,
            ));
        }
        // The toolbar is made after the document, which counts the matches.
        let page = container(component::window_bars(
            self.toolbar(),
            None,
            content.into(),
            self.pointer_inside,
        ))
        .width(Fill)
        .height(Fill)
        .style(style::surface);
        match &self.notice {
            Some(notice) => component::snackbar(page, notice, Message::DismissNotice),
            None => page.into(),
        }
    }

    fn document(&self, theme: &Theme) -> Element<'_, Message> {
        let scheme = ui::Scheme::of(theme);
        let zoom = self.zoom();
        let viewer = Viewer {
            images: &self.images,
            finder: find::Finder::new(
                &self.query,
                self.current,
                &self.found,
                ui::faded(scheme.tertiary, 0.35),
                ui::faded(scheme.primary, 0.6),
            ),
            highlights: &self.highlights,
            dark: scheme.dark,
            zoom,
        };
        let document = markdown::view_with(&self.items, settings(theme, SIZES[self.size]), &viewer);
        let top = 32.0 + component::floating_room(true);
        let page = container(document)
            .max_width(MAX_WIDTH * zoom)
            .padding(Padding {
                top,
                right: 40.0,
                bottom: 32.0,
                left: 40.0,
            });
        container(
            component::scroll(container(page).center_x(Fill))
                .id(self.scroll_id.clone())
                .width(Fill)
                .height(Fill),
        )
        .style(style::surface)
        .into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        let found = self.found.get();
        let matches = if self.query.trim().is_empty() {
            String::new()
        } else if found == 0 {
            "Not found".to_owned()
        } else {
            format!(
                "{} of {found}",
                self.current.map_or(0, |current| current.min(found - 1) + 1)
            )
        };
        let has_matches = found > 0;
        let search = component::search_bar(
            text_input("Search", &self.query)
                .id(self.search_id.clone())
                .on_input(Message::SearchChanged)
                .on_submit(Message::NextMatch),
            vec![
                ui::styled(matches, Type::LabelMedium)
                    .style(style::on_surface_variant)
                    .wrapping(text::Wrapping::None)
                    .into(),
                ui::icon_button(Icon::KeyboardArrowUp)
                    .size(button::Size::ExtraSmall)
                    .on_press_maybe(has_matches.then_some(Message::PreviousMatch))
                    .into(),
                ui::icon_button(Icon::KeyboardArrowDown)
                    .size(button::Size::ExtraSmall)
                    .on_press_maybe(has_matches.then_some(Message::NextMatch))
                    .into(),
            ],
            SEARCH_WIDTH,
        );
        let zoom = self.zoom();
        let bar = row![
            component::group([
                component::tool(
                    Icon::ZoomOut,
                    "Smaller text",
                    (self.size > 0).then_some(Message::ZoomOut),
                ),
                ui::styled(format!("{:.0}%", zoom * 100.0), Type::LabelLarge)
                    .width(48)
                    .align_x(Center)
                    .into(),
                component::tool(
                    Icon::ZoomIn,
                    "Larger text",
                    (self.size + 1 < SIZES.len()).then_some(Message::ZoomIn),
                ),
            ]),
            component::group([component::toggle_tool(
                Icon::OneToOne,
                "Actual size",
                self.size == NORMAL_SIZE,
                Message::ActualSize,
            )]),
            iced::widget::space::horizontal(),
            search,
            component::toolbar_divider(),
            component::group([component::toggle_tool(
                Icon::Info,
                "Inspector",
                self.inspector,
                Message::ToggleInspector,
            )]),
            component::group([
                component::floating_bars_toggle(Message::ToggleFloatingBars),
                component::tool(Icon::Settings, "Settings", Some(Message::OpenSettings)),
            ]),
        ]
        .spacing(8)
        .align_y(Center);
        component::toolbar(bar)
    }

    fn inspector_view(&self) -> Element<'_, Message> {
        let (words, lines) = self.counts;
        info::sections_view(vec![
            ("File", info::file_facts(&self.path)),
            (
                "Document",
                vec![
                    ("Words".to_owned(), words.to_string()),
                    ("Lines".to_owned(), lines.to_string()),
                    (
                        "Pictures".to_owned(),
                        self.images_requested.len().to_string(),
                    ),
                ],
            ),
        ])
    }
}

/// Markdown in the M3 type and color roles, at `size`.
fn settings(theme: &Theme, size: f32) -> markdown::Settings {
    let scheme = ui::Scheme::of(theme);
    let style = markdown::Style {
        font: ui::font::TEXT,
        inline_code_highlight: iced::advanced::text::Highlight {
            background: scheme.surface_container_highest.into(),
            border: iced::border::rounded(ui::shape::EXTRA_SMALL),
        },
        inline_code_color: scheme.on_surface,
        link_color: scheme.primary,
        ..markdown::Style::from(theme)
    };
    markdown::Settings::with_text_size(size, style)
}

struct Viewer<'a> {
    images: &'a HashMap<String, Picture>,
    finder: find::Finder<'a>,
    highlights: &'a code::Highlights,
    dark: bool,
    zoom: f32,
}

impl Viewer<'_> {
    /// Text with its matches marked, wrapped so the current match can be
    /// scrolled to.
    fn text<'b>(
        &self,
        spans: &[text::Span<'static, markdown::Uri>],
        size: iced::Pixels,
    ) -> Element<'b, Message> {
        let (spans, current) = self.finder.mark(spans);
        let text = rich_text(spans)
            .size(size)
            .on_link_click(Message::LinkClicked);
        if current {
            find::current(text)
        } else {
            text.into()
        }
    }
}

impl<'a> markdown::Viewer<'a, Message> for Viewer<'a> {
    fn on_link_click(url: markdown::Uri) -> Message {
        Message::LinkClicked(url)
    }

    fn heading(
        &self,
        settings: markdown::Settings,
        level: &'a markdown::HeadingLevel,
        text: &'a markdown::Text,
        index: usize,
    ) -> Element<'a, Message> {
        use markdown::HeadingLevel::{H1, H2, H3, H4, H5, H6};
        let size = match level {
            H1 => settings.h1_size,
            H2 => settings.h2_size,
            H3 => settings.h3_size,
            H4 => settings.h4_size,
            H5 => settings.h5_size,
            H6 => settings.h6_size,
        };
        let top = if index > 0 {
            settings.text_size.0 / 2.0
        } else {
            0.0
        };
        container(self.text(&text.spans(settings.style), size))
            .padding(padding::top(top))
            .into()
    }

    fn paragraph(
        &self,
        settings: markdown::Settings,
        text: &markdown::Text,
    ) -> Element<'a, Message> {
        self.text(&text.spans(settings.style), settings.text_size)
    }

    fn code_block(
        &self,
        settings: markdown::Settings,
        language: Option<&'a str>,
        code: &'a str,
        _lines: &'a [markdown::Text],
    ) -> Element<'a, Message> {
        let font = settings.style.code_block_font;
        let lines = self.highlights.lines(language, code, self.dark);
        let lines = column(lines.iter().map(|line| {
            let (spans, current) = self.finder.mark(line);
            let text = rich_text(spans)
                .font(font)
                .size(settings.code_size)
                .on_link_click(Message::LinkClicked);
            if current {
                find::current(text)
            } else {
                text.into()
            }
        }));
        container(
            scrollable(container(lines).padding(settings.code_size)).direction(
                scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::default()
                        .width(settings.code_size / 2)
                        .scroller_width(settings.code_size / 2),
                ),
            ),
        )
        .width(Fill)
        .padding(settings.code_size / 4)
        .style(style::code_block)
        .into()
    }

    fn image(
        &self,
        settings: markdown::Settings,
        url: &'a markdown::Uri,
        _title: &'a str,
        alt: &markdown::Text,
    ) -> Element<'a, Message> {
        match self.images.get(url) {
            // At its own size, grown with the text, and no wider than the
            // page.
            Some(picture) => container(
                image(picture.handle.clone())
                    .content_fit(iced::ContentFit::ScaleDown)
                    .width(Fill),
            )
            .max_width(picture.width * self.zoom)
            .into(),
            None => {
                container(rich_text(alt.spans(settings.style)).on_link_click(Message::LinkClicked))
                    .padding(settings.spacing.0)
                    .into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_targets_resolve_against_the_document() {
        let document = Path::new("/notes/readme.md");
        assert_eq!(
            local_target(document, "images/a.png"),
            Some(PathBuf::from("/notes/images/a.png"))
        );
        assert_eq!(
            local_target(document, "other.md#section"),
            Some(PathBuf::from("/notes/other.md"))
        );
        assert_eq!(
            local_target(document, "file:///tmp/x.md"),
            Some(PathBuf::from("/tmp/x.md"))
        );
        assert_eq!(local_target(document, "https://example.org/a.png"), None);
        assert_eq!(local_target(document, "mailto:someone@example.org"), None);
        assert_eq!(local_target(document, "#heading"), None);
    }

    #[test]
    fn finds_nested_images() {
        let source = "![one](a.png)\n\n> ![two](b.png)\n\n- item\n\n  ![three](c.svg)\n";
        let items: Vec<markdown::Item> = markdown::parse(source).collect();
        let mut urls = Vec::new();
        image_urls(&items, &mut urls);
        assert_eq!(urls, ["a.png", "b.png", "c.svg"]);
    }
}
