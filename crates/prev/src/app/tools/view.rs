//! Tools that change what the user sees, never the file: the page and
//! zoom, the view mode, panels, an outline pointing at an area, and the
//! window itself. For working side by side, the agent moves prev to what
//! it is talking about.

use iced::{Point, Size, Task, window};
use prev::control::{Error, code};
use prev::image::window::{self as image_window, Panel};
use prev::markdown;
use prev::pdf::layout::{Fit, ViewMode};
use prev::pdf::viewer::{PdfMessage, Zoom};
use prev::pdf::window::{self as pdf_window, ModeChoice, Sidebar};
use prev_pdf::geometry::{self, Rect};
use schemars::JsonSchema;
use serde::Deserialize;

use super::read::Shown;
use super::{Answer, Kind, On, Output, Tool, number, reply, tool};
use crate::app::{Message, Prev};

/// How many outlines `point_at` has drawn, which tells each from the next.
static POINTED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub(super) fn tools() -> Vec<Tool> {
    vec![
        tool(
            "go_to_page",
            "Go to a page",
            Kind::View,
            "Shows a PDF page from its top, or in an image window the image with that number.",
            go_to_page,
        ),
        tool(
            "scroll_to",
            "Show a place on a page",
            Kind::View,
            "Centers a point of a PDF page, in points from its top-left corner, in the window, \
             optionally at a zoom, as when showing the user what is being talked about.",
            scroll_to,
        ),
        tool(
            "set_zoom",
            "Set the zoom",
            Kind::View,
            "Zooms a PDF, image or Markdown window to a percentage, or fits the page or its \
             width (an image fits the window either way), or shows actual size. Give percent \
             or fit.",
            set_zoom,
        ),
        tool(
            "set_view_mode",
            "Set the view mode",
            Kind::View,
            "Shows a PDF's pages scrolling continuously, one page at a time, or two pages side \
             by side.",
            set_view_mode,
        ),
        tool(
            "point_at",
            "Point at an area",
            Kind::View,
            "Outlines an area of a PDF page, or some text on it, for a few seconds or as long as \
             asked, scrolling to it if needed, to point it out to the user; in a Markdown window, scrolls to some \
             text and marks it. It marks nothing up and changes nothing in the file.",
            point_at,
        ),
        tool(
            "show_panel",
            "Show a panel",
            Kind::View,
            "Opens a panel. PDF: thumbnails, contents, notes and bookmarks in the sidebar, \
             inspector, markup_bar, and search, which can take a query. Image: images in the \
             sidebar, adjust_color, adjust_size, inspector, and markup_bar, which the markup \
             tools open by themselves. Markdown: inspector, and search with a query.",
            show_panel,
        ),
        tool(
            "hide_panel",
            "Hide a panel",
            Kind::View,
            "Closes a panel opened with show_panel or by the user; sidebar closes the sidebar \
             whichever tab it shows, and search clears the search.",
            hide_panel,
        ),
        tool(
            "set_window",
            "Size and place a window",
            Kind::View,
            "Sizes a prev window in logical pixels, places it on the screen, or makes it full \
             screen. On Wayland a window cannot place itself, so only the size and full \
             screen apply there, and a tiling window manager keeps a tiled window's size.",
            set_window,
        ),
        tool(
            "close_window",
            "Close a window",
            Kind::View,
            "Closes a prev window. Edits are already saved; an image with markup not exported \
             asks the user first.",
            close_window,
        ),
    ]
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct GoToPage {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, or the image in an image window, from 1.
    page: usize,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ScrollTo {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// Points from the page's left edge.
    x: f32,
    /// Points from the page's top edge.
    y: f32,
    /// A zoom in percent, such as 200; the zoom stays as it is if left out.
    zoom_percent: Option<f32>,
}

#[derive(Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum FitChoice {
    Page,
    Width,
    ActualSize,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetZoom {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// A zoom in percent, from 10 to 6400, where 100 is actual size.
    percent: Option<f32>,
    /// Fit the page or its width in the window, or show actual size.
    fit: Option<FitChoice>,
}

#[derive(Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum Mode {
    Continuous,
    SinglePage,
    TwoPages,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetViewMode {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    mode: Mode,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PointAt {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The PDF page, from 1.
    page: Option<usize>,
    /// The area [x0, y0, x1, y1] in points from the page's top-left corner,
    /// as page_text and search give boxes.
    #[serde(rename = "box")]
    area: Option<[f32; 4]>,
    /// Text to point at instead: its first place on the PDF page, or in a
    /// Markdown file, which scrolls to it and marks it.
    text: Option<String>,
    /// How long the outline stays, 1 to 60 seconds; 4 if left out. Give
    /// the user time to look, such as while they read the answer.
    seconds: Option<f32>,
}

#[derive(Deserialize, JsonSchema, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
enum PanelName {
    Thumbnails,
    Contents,
    Notes,
    Bookmarks,
    Sidebar,
    Images,
    Inspector,
    MarkupBar,
    Search,
    AdjustColor,
    AdjustSize,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ShowPanel {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    panel: PanelName,
    /// For search: the text to find and mark in the window.
    query: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct HidePanel {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    panel: PanelName,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetWindow {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The width in logical pixels.
    width: Option<f32>,
    /// The height in logical pixels.
    height: Option<f32>,
    /// The left edge's place on the screen, in logical pixels.
    x: Option<f32>,
    /// The top edge's place on the screen, in logical pixels.
    y: Option<f32>,
    /// Make the window full screen, or bring it back from full screen.
    fullscreen: Option<bool>,
}

impl PanelName {
    /// The name agents give it.
    fn name(self) -> &'static str {
        match self {
            PanelName::Thumbnails => "thumbnails",
            PanelName::Contents => "contents",
            PanelName::Notes => "notes",
            PanelName::Bookmarks => "bookmarks",
            PanelName::Sidebar => "sidebar",
            PanelName::Images => "images",
            PanelName::Inspector => "inspector",
            PanelName::MarkupBar => "markup_bar",
            PanelName::Search => "search",
            PanelName::AdjustColor => "adjust_color",
            PanelName::AdjustSize => "adjust_size",
        }
    }
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(code::INVALID_PARAMS, message)
}

/// Whether windows are on Wayland, where they cannot place themselves.
fn on_wayland() -> bool {
    cfg!(target_os = "linux") && std::env::var_os("WAYLAND_DISPLAY").is_some()
}

impl Prev {
    fn send_pdf(&mut self, id: window::Id, message: PdfMessage) -> Task<Message> {
        self.update(Message::Pdf(id, pdf_window::Message::Viewer(message)))
    }

    fn send_pdf_window(&mut self, id: window::Id, message: pdf_window::Message) -> Task<Message> {
        self.update(Message::Pdf(id, message))
    }

    fn send_images(&mut self, id: window::Id, message: image_window::Message) -> Task<Message> {
        self.update(Message::Image(id, message))
    }

    fn send_markdown(&mut self, id: window::Id, message: markdown::Message) -> Task<Message> {
        self.update(Message::Markdown(id, message))
    }

    /// Window `window`'s number and what it shows, as a PDF's page count,
    /// whether it shows images, or Markdown.
    fn view_target(&self, window: Option<u64>) -> Result<(window::Id, Target), Error> {
        let (id, shown) = self.shown(window)?;
        let target = match shown {
            Shown::Pdf(viewer) => Target::Pdf {
                pages: viewer.page_count(),
            },
            Shown::Image(images) => Target::Image {
                count: images.paths().count(),
                panel: images.panel(),
                sidebar: images.sidebar_shown(),
                markup: images.markup_shown(),
            },
            Shown::Markdown(markdown) => Target::Markdown {
                inspector: markdown.inspector_shown(),
            },
            Shown::Start => Target::Start,
        };
        Ok((id, target))
    }

    fn pdf_window(&self, id: window::Id) -> Option<&pdf_window::PdfWindow> {
        match &self.windows.get(&id)?.content {
            crate::app::Content::Document(document) => document.pdf.as_deref(),
            crate::app::Content::Start => None,
        }
    }
}

enum Target {
    Start,
    Pdf {
        pages: usize,
    },
    Image {
        count: usize,
        panel: Option<Panel>,
        sidebar: bool,
        markup: bool,
    },
    Markdown {
        inspector: bool,
    },
}

impl Target {
    fn what(&self) -> &'static str {
        match self {
            Target::Start => "no file",
            Target::Pdf { .. } => "a PDF",
            Target::Image { .. } => "an image",
            Target::Markdown { .. } => "a Markdown file",
        }
    }
}

fn not_for(id: window::Id, target: &Target, tool: &str) -> Error {
    invalid(format!(
        "Window {} shows {}; {tool} does not apply to it.",
        number(id),
        target.what()
    ))
}

/// Page `page` of `pages`, from 1, as an index.
fn page(page: usize, pages: usize) -> Result<usize, Error> {
    if (1..=pages).contains(&page) {
        Ok(page - 1)
    } else {
        Err(invalid(format!(
            "Page {page} is not in the document, which has {pages} pages."
        )))
    }
}

/// Runs `act` on the window a view tool names, answering with its text.
fn act(
    app: &mut Prev,
    window: Option<u64>,
    answer: &Answer,
    act: impl FnOnce(&mut Prev, window::Id, Target) -> Result<(Task<Message>, String), Error>,
) -> Task<Message> {
    match app
        .view_target(window)
        .and_then(|(id, target)| act(app, id, target))
    {
        Ok((task, done)) => {
            answer.send(Ok(Output::Text(done)));
            task
        }
        Err(error) => reply(answer, Err(error)),
    }
}

fn go_to_page(app: &mut Prev, input: GoToPage, answer: &Answer) -> Task<Message> {
    act(app, input.window, answer, |app, id, target| match target {
        Target::Pdf { pages } => {
            let index = page(input.page, pages)?;
            let task = app.send_pdf(
                id,
                PdfMessage::GoTo {
                    page: index,
                    point: None,
                },
            );
            Ok((
                task,
                format!("Window {} shows page {}.", number(id), input.page),
            ))
        }
        Target::Image { count, .. } => {
            if !(1..=count).contains(&input.page) {
                return Err(invalid(format!(
                    "The window has {count} images; there is no image {}.",
                    input.page
                )));
            }
            let task = app.send_images(id, image_window::Message::Select(input.page - 1));
            Ok((
                task,
                format!("Window {} shows image {}.", number(id), input.page),
            ))
        }
        target => Err(not_for(id, &target, "go_to_page")),
    })
}

fn scroll_to(app: &mut Prev, input: ScrollTo, answer: &Answer) -> Task<Message> {
    act(app, input.window, answer, |app, id, target| {
        let Target::Pdf { pages } = target else {
            return Err(not_for(id, &target, "scroll_to"));
        };
        let index = page(input.page, pages)?;
        let fit = input
            .zoom_percent
            .map(|percent| Fit::Zoom(percent.clamp(10.0, 6400.0) / 100.0));
        let task = app.send_pdf(
            id,
            PdfMessage::Show {
                page: index,
                point: geometry::Point::new(input.x, input.y),
                fit,
            },
        );
        Ok((
            task,
            format!(
                "Window {} shows page {} around ({}, {}).",
                number(id),
                input.page,
                input.x,
                input.y
            ),
        ))
    })
}

fn set_zoom(app: &mut Prev, input: SetZoom, answer: &Answer) -> Task<Message> {
    act(app, input.window, answer, |app, id, target| {
        let percent = input.percent.map(|percent| percent.clamp(10.0, 6400.0));
        let (task, done) = match (percent, input.fit, &target) {
            (Some(_), Some(_), _) | (None, None, _) => {
                return Err(invalid("Give either percent or fit."));
            }
            (Some(percent), None, Target::Pdf { .. }) => (
                app.send_pdf(id, PdfMessage::Zoom(Zoom::To(percent / 100.0))),
                format!("{percent}%"),
            ),
            (None, Some(fit), Target::Pdf { .. }) => (
                app.send_pdf(
                    id,
                    PdfMessage::Zoom(match fit {
                        FitChoice::Page => Zoom::FitPage,
                        FitChoice::Width => Zoom::FitWidth,
                        FitChoice::ActualSize => Zoom::ActualSize,
                    }),
                ),
                fit_name(fit).to_owned(),
            ),
            (Some(percent), None, Target::Image { .. }) => (
                app.send_images(id, image_window::Message::ZoomTo(percent / 100.0)),
                format!("{percent}%"),
            ),
            (None, Some(fit), Target::Image { .. }) => (
                app.send_images(
                    id,
                    match fit {
                        FitChoice::ActualSize => image_window::Message::ActualSize,
                        FitChoice::Page | FitChoice::Width => image_window::Message::FitToWindow,
                    },
                ),
                fit_name(fit).to_owned(),
            ),
            (None, Some(FitChoice::ActualSize), Target::Markdown { .. }) => (
                app.send_markdown(id, markdown::Message::ActualSize),
                fit_name(FitChoice::ActualSize).to_owned(),
            ),
            (_, _, Target::Markdown { .. }) => {
                return Err(invalid(
                    "A Markdown window zooms only to actual size here; its text reflows to fit.",
                ));
            }
            (_, _, target) => return Err(not_for(id, target, "set_zoom")),
        };
        Ok((task, format!("Window {} is zoomed to {done}.", number(id))))
    })
}

fn fit_name(fit: FitChoice) -> &'static str {
    match fit {
        FitChoice::Page => "fit the page",
        FitChoice::Width => "fit the width",
        FitChoice::ActualSize => "actual size",
    }
}

fn set_view_mode(app: &mut Prev, input: SetViewMode, answer: &Answer) -> Task<Message> {
    act(app, input.window, answer, |app, id, target| {
        if !matches!(target, Target::Pdf { .. }) {
            return Err(not_for(id, &target, "set_view_mode"));
        }
        let mode = match input.mode {
            Mode::Continuous => ViewMode::Continuous,
            Mode::SinglePage => ViewMode::SinglePage,
            Mode::TwoPages => ViewMode::TwoPages,
        };
        let task = app.send_pdf_window(id, pdf_window::Message::ModeSelected(ModeChoice(mode)));
        let shown = match input.mode {
            Mode::Continuous => "its pages scrolling continuously",
            Mode::SinglePage => "one page at a time",
            Mode::TwoPages => "two pages side by side",
        };
        Ok((task, format!("Window {} shows {shown}.", number(id))))
    })
}

fn point_at(app: &mut Prev, input: PointAt, answer: &Answer) -> Task<Message> {
    let (id, target) = match app.view_target(input.window) {
        Ok(found) => found,
        Err(error) => return reply(answer, Err(error)),
    };
    match (&target, input.area, input.text) {
        (Target::Markdown { .. }, None, Some(text)) => {
            // The window shows the Markdown, not its syntax.
            let text = shown_markdown(&text);
            answer.send(Ok(Output::Text(format!(
                "Window {} shows and marks \"{text}\".",
                number(id)
            ))));
            app.send_markdown(id, markdown::Message::SearchChanged(text))
        }
        (Target::Markdown { .. }, ..) => reply(
            answer,
            Err(invalid("In a Markdown window, point at text, with text.")),
        ),
        (Target::Pdf { pages }, area, text) => {
            let Some(page_number) = input.page else {
                return reply(answer, Err(invalid("Give the page.")));
            };
            let index = match page(page_number, *pages) {
                Ok(index) => index,
                Err(error) => return reply(answer, Err(error)),
            };
            let seconds = input.seconds.unwrap_or(4.0).clamp(1.0, 60.0);
            match (area, text) {
                (Some([x0, y0, x1, y1]), None) => {
                    let rect = Rect::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1));
                    app.point(id, answer, index, seconds, Ok(rect))
                }
                (None, Some(text)) => {
                    let handle = match app.shown_pdf(input.window, "point_at") {
                        Ok(viewer) => viewer.handle.clone(),
                        Err(error) => return reply(answer, Err(error)),
                    };
                    let answer = answer.clone();
                    Task::perform(
                        async move {
                            let display = super::read::display(&handle, index).await?;
                            let needle = text.clone();
                            let found = super::read::off_thread(move || display.search(&needle))
                                .await?
                                .map_err(super::read::failed)?;
                            found.first().map(|quad| quad.bounds()).ok_or_else(|| {
                                invalid(format!("\"{text}\" is not on page {page_number}."))
                            })
                        },
                        move |found| Message::ToolPoint(id, answer.clone(), index, seconds, found),
                    )
                }
                _ => reply(answer, Err(invalid("Give either box or text."))),
            }
        }
        (target, ..) => reply(answer, Err(not_for(id, target, "point_at"))),
    }
}

/// `text` as a Markdown window shows it: without heading marks, emphasis,
/// code ticks or list bullets.
fn shown_markdown(text: &str) -> String {
    let text = text.trim();
    let text = text
        .trim_start_matches('#')
        .trim_start_matches('>')
        .trim_start();
    let text = text
        .strip_prefix("- ")
        .or_else(|| text.strip_prefix("* "))
        .unwrap_or(text);
    text.replace(['`', '*'], "").trim().to_owned()
}

impl Prev {
    /// Outlines `rect` of page `index` in window `id` for a moment.
    pub(in crate::app) fn point(
        &mut self,
        id: window::Id,
        answer: &Answer,
        index: usize,
        seconds: f32,
        rect: Result<Rect, Error>,
    ) -> Task<Message> {
        let rect = match rect {
            Ok(rect) => rect,
            Err(error) => return reply(answer, Err(error)),
        };
        if !self.windows.contains_key(&id) {
            return reply(
                answer,
                Err(invalid(format!("Window {} closed.", number(id)))),
            );
        }
        let pointed = POINTED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        answer.send(Ok(Output::Text(format!(
            "The area is outlined on page {} of window {} for {seconds} seconds.",
            index + 1,
            number(id)
        ))));
        self.send_pdf(
            id,
            PdfMessage::PointAt {
                page: index,
                rect,
                id: pointed,
                seconds,
            },
        )
    }
}

fn show_panel(app: &mut Prev, input: ShowPanel, answer: &Answer) -> Task<Message> {
    set_panel(app, input.window, input.panel, true, input.query, answer)
}

fn hide_panel(app: &mut Prev, input: HidePanel, answer: &Answer) -> Task<Message> {
    set_panel(app, input.window, input.panel, false, None, answer)
}

/// Shows or hides `panel`, toggling it only when that changes it.
fn set_panel(
    app: &mut Prev,
    window: Option<u64>,
    panel: PanelName,
    show: bool,
    query: Option<String>,
    answer: &Answer,
) -> Task<Message> {
    act(app, window, answer, |app, id, target| {
        let unknown = || {
            invalid(format!(
                "Window {} shows {}, which has no {} panel.",
                number(id),
                target.what(),
                panel.name()
            ))
        };
        let task = match &target {
            Target::Pdf { .. } => {
                let window = app.pdf_window(id).ok_or_else(unknown)?;
                let (sidebar, inspector, markup_bar) = (
                    window.sidebar(),
                    window.inspector_shown(),
                    window.markup_bar_shown(),
                );
                let tab = match panel {
                    PanelName::Thumbnails => Some(Sidebar::Thumbnails),
                    PanelName::Contents => Some(Sidebar::Contents),
                    PanelName::Notes => Some(Sidebar::Notes),
                    PanelName::Bookmarks => Some(Sidebar::Bookmarks),
                    PanelName::Sidebar => sidebar.or(Some(Sidebar::Thumbnails)),
                    _ => None,
                };
                match panel {
                    _ if tab.is_some() => {
                        let shown = if show {
                            tab
                        } else if panel == PanelName::Sidebar || sidebar == tab {
                            None
                        } else {
                            sidebar
                        };
                        app.send_pdf_window(id, pdf_window::Message::ShowSidebar(shown))
                    }
                    PanelName::Inspector if inspector != show => {
                        app.send_pdf_window(id, pdf_window::Message::ToggleInspector)
                    }
                    PanelName::MarkupBar if markup_bar != show => {
                        app.send_pdf_window(id, pdf_window::Message::ToggleMarkupBar)
                    }
                    PanelName::Inspector | PanelName::MarkupBar => Task::none(),
                    PanelName::Search => {
                        app.send_pdf(id, PdfMessage::SearchChanged(query.unwrap_or_default()))
                    }
                    _ => return Err(unknown()),
                }
            }
            Target::Image {
                panel: open,
                sidebar,
                markup,
                ..
            } => {
                let wanted = match panel {
                    PanelName::AdjustColor => Some(Panel::AdjustColor),
                    PanelName::AdjustSize => Some(Panel::AdjustSize),
                    PanelName::Inspector => Some(Panel::Inspector),
                    _ => None,
                };
                match (panel, wanted) {
                    (PanelName::Images | PanelName::Sidebar, _) if *sidebar != show => {
                        app.send_images(id, image_window::Message::ToggleSidebar)
                    }
                    (PanelName::Images | PanelName::Sidebar, _) => Task::none(),
                    // Marking up an image starts with its markup bar.
                    (PanelName::MarkupBar, _) if *markup != show => {
                        app.send_images(id, image_window::Message::ToggleMarkup)
                    }
                    (PanelName::MarkupBar, _) => Task::none(),
                    (_, Some(wanted)) if (*open == Some(wanted)) != show => {
                        app.send_images(id, image_window::Message::TogglePanel(wanted))
                    }
                    (_, Some(_)) => Task::none(),
                    _ => return Err(unknown()),
                }
            }
            Target::Markdown { inspector } => match panel {
                PanelName::Inspector if *inspector != show => {
                    app.send_markdown(id, markdown::Message::ToggleInspector)
                }
                PanelName::Inspector => Task::none(),
                PanelName::Search => app.send_markdown(
                    id,
                    markdown::Message::SearchChanged(query.unwrap_or_default()),
                ),
                _ => return Err(unknown()),
            },
            Target::Start => return Err(not_for(id, &target, "show_panel")),
        };
        let done = if show { "shows" } else { "hides" };
        Ok((
            task,
            format!("Window {} {done} {}.", number(id), panel.name()),
        ))
    })
}

fn set_window(app: &mut Prev, input: SetWindow, answer: &Answer) -> Task<Message> {
    let id = match app.tool_window(input.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let mut tasks = Vec::new();
    let mut done = Vec::new();
    if let Some(fullscreen) = input.fullscreen {
        tasks.push(app.set_fullscreen(id, Some(fullscreen)));
        done.push(if fullscreen {
            "is full screen".to_owned()
        } else {
            "is out of full screen".to_owned()
        });
    }
    match (input.width, input.height) {
        (Some(width), Some(height)) => {
            let size = Size::new(width.max(200.0), height.max(150.0));
            tasks.push(window::resize(id, size));
            done.push(format!("asked for {} by {}", size.width, size.height));
        }
        (None, None) => {}
        _ => return reply(answer, Err(invalid("Give both width and height."))),
    }
    match (input.x, input.y) {
        (Some(_), Some(_)) if on_wayland() => {
            done.push("cannot place itself on Wayland, so it stays where it is".to_owned());
        }
        (Some(x), Some(y)) => {
            tasks.push(window::move_to(id, Point::new(x, y)));
            done.push(format!("moved to ({x}, {y})"));
        }
        (None, None) => {}
        _ => return reply(answer, Err(invalid("Give both x and y."))),
    }
    if done.is_empty() {
        return reply(answer, Err(invalid("Give a size, a place or fullscreen.")));
    }
    answer.send(Ok(Output::Text(format!(
        "Window {} {}.",
        number(id),
        done.join(", ")
    ))));
    Task::batch(tasks)
}

fn close_window(app: &mut Prev, input: On, answer: &Answer) -> Task<Message> {
    let id = match app.tool_window(input.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    answer.send(Ok(Output::Text(format!(
        "Asked window {} to close.",
        number(id)
    ))));
    app.update(Message::CloseRequested(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_syntax_is_left_out() {
        assert_eq!(shown_markdown("## Install"), "Install");
        assert_eq!(shown_markdown("- **Fedora:** `dnf`"), "Fedora: dnf");
        assert_eq!(shown_markdown("> quoted"), "quoted");
        assert_eq!(shown_markdown("plain words"), "plain words");
    }
}
