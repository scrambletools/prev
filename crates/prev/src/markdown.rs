//! A window rendering a Markdown file, reloading when the file changes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use iced::futures::channel::{mpsc, oneshot};
use iced::widget::image::Handle;
use iced::widget::{container, image, markdown, rich_text};
use iced::{ContentFit, Element, Fill, Task, Theme};

use crate::filetype::{self, FileKind};
use crate::portal;
use crate::ui::{self, Icon, component, style};

const POLL_INTERVAL: Duration = Duration::from_millis(700);
const MAX_WIDTH: f32 = 860.0;

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<String, String>),
    FileChanged,
    LinkClicked(String),
    ImageLoaded(String, Option<Handle>),
    UriOpened(Result<(), String>),
    DismissNotice,
}

/// Changes the app applies on the window's behalf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    OpenPath(PathBuf),
}

pub struct MarkdownWindow {
    path: PathBuf,
    items: Vec<markdown::Item>,
    error: Option<String>,
    images: HashMap<String, Handle>,
    images_requested: std::collections::HashSet<String>,
    notice: Option<String>,
    effects: Vec<Effect>,
    watching: Arc<AtomicBool>,
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

fn load_image(path: PathBuf) -> Option<Handle> {
    match filetype::detect_path(&path).ok()?? {
        FileKind::Image(format) => {
            let decoded = prev_image::decode::decode_file(&path, format).ok()?;
            let frame = decoded.frames.into_iter().next()?;
            Some(Handle::from_rgba(frame.width, frame.height, frame.pixels))
        }
        FileKind::Svg => {
            let svg = prev_image::svg::Svg::parse_file(&path).ok()?;
            let frame = svg.render(2.0).ok()?;
            Some(Handle::from_rgba(frame.width, frame.height, frame.pixels))
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
            error: None,
            images: HashMap::new(),
            images_requested: Default::default(),
            notice: None,
            effects: Vec::new(),
            watching: Arc::clone(&watching),
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok(source)) => {
                self.items = markdown::parse(&source).collect();
                self.error = None;
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
            Message::ImageLoaded(url, handle) => {
                match handle {
                    Some(handle) => {
                        self.images.insert(url, handle);
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
        }
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
        Task::perform(receiver, move |handle| {
            Message::ImageLoaded(url, handle.ok().flatten())
        })
    }

    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        if let Some(error) = &self.error {
            return container(component::empty_state(
                Icon::Error,
                "prev can't read this file",
                error.as_str(),
            ))
            .style(style::surface)
            .into();
        }
        let viewer = Viewer {
            images: &self.images,
        };
        let document = markdown::view_with(&self.items, settings(theme), &viewer);
        let page = container(document).max_width(MAX_WIDTH).padding([32, 40]);
        let body = container(
            component::scroll(container(page).center_x(Fill))
                .width(Fill)
                .height(Fill),
        )
        .style(style::surface);
        match &self.notice {
            Some(notice) => component::snackbar(body, notice, Message::DismissNotice),
            None => body.into(),
        }
    }
}

/// Markdown in the M3 type and color roles.
fn settings(theme: &Theme) -> markdown::Settings {
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
    markdown::Settings::with_style(style)
}

struct Viewer<'a> {
    images: &'a HashMap<String, Handle>,
}

impl<'a> markdown::Viewer<'a, Message> for Viewer<'a> {
    fn on_link_click(url: markdown::Uri) -> Message {
        Message::LinkClicked(url)
    }

    fn image(
        &self,
        settings: markdown::Settings,
        url: &'a markdown::Uri,
        title: &'a str,
        alt: &markdown::Text,
    ) -> Element<'a, Message> {
        match self.images.get(url) {
            Some(handle) => image(handle.clone())
                .content_fit(ContentFit::ScaleDown)
                .width(Fill)
                .into(),
            None => {
                let _ = (url, title);
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
