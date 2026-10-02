//! Tools that read: what a window shows, its text, annotations and form
//! fields, pictures of its pages and images, and opening files. Positions
//! are in page points, from the page's top-left corner, with y growing
//! down.

use std::path::PathBuf;
use std::sync::Arc;

use iced::Task;
use iced::futures::channel::oneshot;
use iced::window;
use prev::control::{Error, code};
use prev::image::editor::spawn;
use prev::image::window::{ImageWindow, Pixels};
use prev::markdown::MarkdownWindow;
use prev::pdf::viewer::PdfViewer;
use prev_pdf::annotation::{self, Annotation, FieldKind, TextMarkup};
use prev_pdf::engine::{LinkTarget, OutlineItem, PageDisplay, page_pixels};
use prev_pdf::geometry::{Quad, Rect};
use prev_pdf::worker::DocumentHandle;
use prev_store::signatures::SignatureStore;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Answer, Kind, Nothing, On, Output, Tool, number, reply, tool};
use crate::app::{Content, Message, Prev};

/// The widest picture of a page or an image a tool makes.
const MAX_SIDE: u32 = 4096;
/// The largest image `get_image` gives, in pixels.
const MAX_PIXELS: u64 = 40_000_000;

pub(super) fn tools() -> Vec<Tool> {
    vec![
        tool(
            "open_file",
            "Open a file",
            Kind::View,
            "Opens a PDF, image, SVG or Markdown file in prev, or brings forward the window \
             that already shows it, and says which window it is in.",
            open_file,
        ),
        tool(
            "document_info",
            "Document information",
            Kind::Read,
            "Describes what a window shows. For a PDF: its pages, their sizes in points, page \
             labels, title, metadata and outline. For an image: its size, frames and the other \
             images in the window. For Markdown: its words and lines.",
            document_info,
        ),
        tool(
            "page_text",
            "Read page text",
            Kind::Read,
            "Gives the text of one PDF page as lines, each with its box [x0, y0, x1, y1] in \
             points from the page's top-left corner, for placing markup. For a Markdown window, \
             gives the whole Markdown source. Images have no text: use render_image.",
            page_text,
        ),
        tool(
            "search",
            "Search text",
            Kind::Read,
            "Finds text in a PDF, ignoring case: each match's page, box in points and the line \
             it is on. In a Markdown window, finds the lines that contain it.",
            search,
        ),
        tool(
            "current_view",
            "Current view",
            Kind::Read,
            "Says what the user sees in a window: for a PDF the page in view, the pages on \
             screen, the zoom, the view mode and any selected text; for images which image is \
             shown.",
            current_view,
        ),
        tool(
            "list_annotations",
            "List annotations",
            Kind::Read,
            "Lists a PDF's annotations, or one page's: highlights, notes, text boxes, shapes, \
             ink, stamps such as signatures, and redactions not yet applied, each with its id, \
             page, box in points, colour and text.",
            list_annotations,
        ),
        tool(
            "form_fields",
            "List form fields",
            Kind::Read,
            "Lists a PDF's form fields: each one's id, name, type, value, choices and box in \
             points, and whether it is read-only.",
            form_fields,
        ),
        tool(
            "list_signatures",
            "List signatures",
            Kind::Read,
            "Lists the signatures saved in prev's signature library, by the names the user \
             gave them.",
            list_signatures,
        ),
        tool(
            "render_page",
            "Picture of a page",
            Kind::Read,
            "Draws one PDF page, with its annotations, as a PNG, for looking at it.",
            render_page,
        ),
        tool(
            "render_image",
            "Picture of the image",
            Kind::Read,
            "Draws the image a window shows, with the edits made so far, as a PNG scaled down \
             to fit, for looking at it.",
            render_image,
        ),
        tool(
            "get_image",
            "Get the image",
            Kind::Read,
            "Gives the image a window shows at full size, with the edits made so far, as a \
             PNG, for changing it on the agent's side and handing it back with replace_image.",
            get_image,
        ),
    ]
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct OpenFile {
    /// The file's path, absolute or with ~ for the home folder.
    path: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct OnPage {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1; the page in view if left out.
    page: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Search {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The text to find.
    query: String,
    /// How many matches to give at most; 50 if left out.
    max_matches: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Annotations {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Only this page's, from 1; every page's if left out.
    page: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RenderPage {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1; the page in view if left out.
    page: Option<usize>,
    /// The picture's width in pixels, up to 4096; 1024 if left out.
    width: Option<u32>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RenderImage {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The picture's longer side in pixels, up to 4096; 1024 if left out.
    max_side: Option<u32>,
}

/// What a window shows, for the read tools.
pub(super) enum Shown<'a> {
    Start,
    Pdf(&'a PdfViewer),
    Image(&'a ImageWindow),
    Markdown(&'a MarkdownWindow),
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(code::INVALID_PARAMS, message)
}

fn failed(message: impl std::fmt::Display) -> Error {
    Error::new(code::INTERNAL_ERROR, message.to_string())
}

/// The error for a tool that does not apply to what window `window` shows.
fn not_for(window: window::Id, shows: &str, tool: &str) -> Error {
    invalid(format!(
        "Window {} shows {shows}; {tool} does not apply to it.",
        number(window)
    ))
}

impl Prev {
    /// What window `window` shows, or the window in front.
    pub(super) fn shown(&self, window: Option<u64>) -> Result<(window::Id, Shown<'_>), Error> {
        let id = self.tool_window(window)?;
        let shown = match &self.windows[&id].content {
            Content::Start => Shown::Start,
            Content::Document(document) => {
                if let Some(pdf) = &document.pdf {
                    Shown::Pdf(pdf.viewer().ok_or_else(|| {
                        invalid(format!(
                            "Window {}'s PDF is not open yet, or waits for its password.",
                            number(id)
                        ))
                    })?)
                } else if let Some(images) = &document.images {
                    Shown::Image(images)
                } else if let Some(markdown) = &document.markdown {
                    Shown::Markdown(markdown)
                } else {
                    return Err(invalid(format!(
                        "Window {} could not open {}.",
                        number(id),
                        document.path.display()
                    )));
                }
            }
        };
        Ok((id, shown))
    }

    /// The PDF window `window` shows, for a tool that reads PDFs only.
    fn shown_pdf(&self, window: Option<u64>, tool: &str) -> Result<&PdfViewer, Error> {
        match self.shown(window)? {
            (_, Shown::Pdf(viewer)) => Ok(viewer),
            (id, shown) => Err(not_for(id, shown.what(), tool)),
        }
    }

    /// The path of the file window `id` shows.
    fn shown_path(&self, id: window::Id) -> Option<PathBuf> {
        match &self.windows.get(&id)?.content {
            Content::Document(document) => Some(document.images.as_ref().map_or_else(
                || document.path.clone(),
                |images| images.current_path().into(),
            )),
            Content::Start => None,
        }
    }
}

impl Shown<'_> {
    fn what(&self) -> &'static str {
        match self {
            Shown::Start => "no file",
            Shown::Pdf(_) => "a PDF",
            Shown::Image(_) => "an image",
            Shown::Markdown(_) => "a Markdown file",
        }
    }
}

/// Page `page`, counted from 1, as an index; the page in view if `None`.
fn page_index(viewer: &PdfViewer, page: Option<usize>) -> Result<usize, Error> {
    let count = viewer.page_count();
    match page {
        None => Ok(viewer
            .layout
            .current_page(&viewer.view)
            .unwrap_or(viewer.current)
            .min(count.saturating_sub(1))),
        Some(page) if (1..=count).contains(&page) => Ok(page - 1),
        Some(page) => Err(invalid(format!(
            "Page {page} is not in the document, which has {count} pages."
        ))),
    }
}

/// A number rounded to a tenth, which is plenty for points.
fn tenth(value: f32) -> f64 {
    (f64::from(value) * 10.0).round() / 10.0
}

fn rect(rect: Rect) -> Value {
    json!([
        tenth(rect.x0),
        tenth(rect.y0),
        tenth(rect.x1),
        tenth(rect.y1)
    ])
}

fn hex(color: annotation::Rgb) -> String {
    let byte = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        byte(color.red),
        byte(color.green),
        byte(color.blue)
    )
}

/// Waits for what the document thread sends back.
async fn answered<T>(receiver: oneshot::Receiver<prev_pdf::engine::Result<T>>) -> Result<T, Error> {
    receiver
        .await
        .map_err(|_| failed("the document closed"))?
        .map_err(failed)
}

/// Page `page`'s display list, for its text, searches and pictures.
async fn display(handle: &DocumentHandle, page: usize) -> Result<Arc<dyn PageDisplay>, Error> {
    answered(handle.display(page)).await
}

/// Runs `work` off the interface thread.
async fn off_thread<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, Error> {
    spawn(work).await.map_err(|_| failed("the work stopped"))
}

fn open_file(app: &mut Prev, input: OpenFile, answer: &Answer) -> Task<Message> {
    let path = prev_store::paths::expand_home(std::path::Path::new(&input.path));
    if !path.is_file() {
        return reply(
            answer,
            Err(invalid(format!("There is no file at {}.", path.display()))),
        );
    }
    let path = prev_store::paths::canonical(&path);
    let task = app.open_paths(vec![path.clone()]);
    let result = match app.window_showing(&path) {
        Some(id) => Ok(Output::Json(json!({
            "window": number(id),
            "file": path.display().to_string(),
        }))),
        None => Err(failed(format!("prev could not open {}.", path.display()))),
    };
    answer.send(result);
    task
}

fn document_info(app: &mut Prev, input: On, answer: &Answer) -> Task<Message> {
    let (id, shown) = match app.shown(input.window) {
        Ok(shown) => shown,
        Err(error) => return reply(answer, Err(error)),
    };
    let file = app.shown_path(id).map(|path| path.display().to_string());
    match shown {
        Shown::Start => reply(
            answer,
            Ok(Output::Json(
                json!({ "window": number(id), "kind": "start" }),
            )),
        ),
        Shown::Markdown(markdown) => {
            let (words, lines) = markdown.counts();
            reply(
                answer,
                Ok(Output::Json(json!({
                    "window": number(id),
                    "kind": "markdown",
                    "file": file,
                    "words": words,
                    "lines": lines,
                }))),
            )
        }
        Shown::Image(images) => {
            let Some(image) = images.shown_image() else {
                return reply(answer, Err(invalid("The image is still loading.")));
            };
            let files: Vec<String> = images
                .paths()
                .map(|path| path.display().to_string())
                .collect();
            reply(
                answer,
                Ok(Output::Json(json!({
                    "window": number(id),
                    "kind": if matches!(image.pixels, Pixels::Svg(_)) { "svg" } else { "image" },
                    "file": file,
                    "width": image.width,
                    "height": image.height,
                    "frames": image.frames,
                    "image": image.index + 1,
                    "images": image.count,
                    "files": files,
                }))),
            )
        }
        Shown::Pdf(viewer) => {
            let info = viewer.info.clone();
            let handle = viewer.handle.clone();
            let window = number(id);
            answer.clone().later(Task::perform(
                async move {
                    let metadata = handle
                        .metadata()
                        .await
                        .map_err(|_| failed("the document closed"))?;
                    let outline = answered(handle.outline()).await.unwrap_or_default();
                    let sizes: Vec<Value> = info
                        .page_sizes
                        .iter()
                        .map(|size| json!([tenth(size.width), tenth(size.height)]))
                        .collect();
                    let mut result = json!({
                        "window": window,
                        "kind": "pdf",
                        "file": file,
                        "pages": info.page_sizes.len(),
                    });
                    // One size for every page, or each page's.
                    if sizes.windows(2).all(|pair| pair[0] == pair[1]) {
                        result["page_size"] = sizes.first().cloned().unwrap_or(Value::Null);
                    } else {
                        result["page_sizes"] = Value::Array(sizes);
                    }
                    if info.page_labels.iter().any(Option::is_some) {
                        result["page_labels"] = json!(info.page_labels);
                    }
                    let fields = [
                        ("title", &metadata.title),
                        ("author", &metadata.author),
                        ("subject", &metadata.subject),
                        ("keywords", &metadata.keywords),
                        ("creator", &metadata.creator),
                        ("producer", &metadata.producer),
                        ("created", &metadata.created),
                        ("modified", &metadata.modified),
                        ("format", &metadata.format),
                        ("encryption", &metadata.encryption),
                    ];
                    result["metadata"] = fields
                        .into_iter()
                        .filter(|(_, value)| !value.trim().is_empty())
                        .map(|(name, value)| (name.to_owned(), json!(value)))
                        .collect::<serde_json::Map<_, _>>()
                        .into();
                    if !outline.is_empty() {
                        result["outline"] = outline_json(&outline);
                    }
                    Ok(Output::Json(result))
                },
                |result| result,
            ))
        }
    }
}

fn outline_json(items: &[OutlineItem]) -> Value {
    items
        .iter()
        .map(|item| {
            let mut entry = json!({ "title": item.title });
            if let Some(LinkTarget::Page { index, .. }) = item.target {
                entry["page"] = json!(index + 1);
            }
            if !item.children.is_empty() {
                entry["children"] = outline_json(&item.children);
            }
            entry
        })
        .collect()
}

fn page_text(app: &mut Prev, input: OnPage, answer: &Answer) -> Task<Message> {
    let (id, shown) = match app.shown(input.window) {
        Ok(shown) => shown,
        Err(error) => return reply(answer, Err(error)),
    };
    match shown {
        Shown::Markdown(_) => {
            let Some(path) = app.shown_path(id) else {
                return reply(answer, Err(failed("the window has no file")));
            };
            answer.clone().later(Task::perform(
                async move {
                    let text = off_thread(move || std::fs::read_to_string(&path))
                        .await?
                        .map_err(failed)?;
                    Ok(Output::Text(text))
                },
                |result| result,
            ))
        }
        Shown::Pdf(viewer) => {
            let page = match page_index(viewer, input.page) {
                Ok(page) => page,
                Err(error) => return reply(answer, Err(error)),
            };
            let handle = viewer.handle.clone();
            let size = viewer.info.page_sizes[page];
            answer.clone().later(Task::perform(
                async move {
                    let display = display(&handle, page).await?;
                    let layout = off_thread(move || display.text()).await?.map_err(failed)?;
                    let lines: Vec<Value> = layout
                        .lines
                        .iter()
                        .map(|line| {
                            let text: String =
                                line.chars.iter().map(|glyph| glyph.character).collect();
                            json!({ "text": text, "box": rect(line.bounds) })
                        })
                        .collect();
                    Ok(Output::Json(json!({
                        "page": page + 1,
                        "width": tenth(size.width),
                        "height": tenth(size.height),
                        "lines": lines,
                    })))
                },
                |result| result,
            ))
        }
        shown => reply(answer, Err(not_for(id, shown.what(), "page_text"))),
    }
}

/// The line of `lines` that `quad` sits on.
fn line_of(lines: &[prev_pdf::text::TextLine], quad: &Quad) -> Option<String> {
    let center = quad.bounds().center();
    lines
        .iter()
        .find(|line| line.bounds.contains(center))
        .map(|line| line.chars.iter().map(|glyph| glyph.character).collect())
}

fn search(app: &mut Prev, input: Search, answer: &Answer) -> Task<Message> {
    let query = input.query.trim().to_owned();
    if query.is_empty() {
        return reply(answer, Err(invalid("The query is empty.")));
    }
    let limit = input.max_matches.unwrap_or(50).max(1);
    let (id, shown) = match app.shown(input.window) {
        Ok(shown) => shown,
        Err(error) => return reply(answer, Err(error)),
    };
    match shown {
        Shown::Markdown(_) => {
            let Some(path) = app.shown_path(id) else {
                return reply(answer, Err(failed("the window has no file")));
            };
            answer.clone().later(Task::perform(
                async move {
                    let text = off_thread(move || std::fs::read_to_string(&path))
                        .await?
                        .map_err(failed)?;
                    let needle = query.to_lowercase();
                    let matches: Vec<Value> = text
                        .lines()
                        .enumerate()
                        .filter(|(_, line)| line.to_lowercase().contains(&needle))
                        .take(limit)
                        .map(|(index, line)| json!({ "line": index + 1, "text": line }))
                        .collect();
                    Ok(Output::Json(json!({ "matches": matches })))
                },
                |result| result,
            ))
        }
        Shown::Pdf(viewer) => {
            let handle = viewer.handle.clone();
            let pages = viewer.page_count();
            answer.clone().later(Task::perform(
                async move {
                    let mut matches = Vec::new();
                    let mut more = false;
                    'pages: for page in 0..pages {
                        let display = display(&handle, page).await?;
                        let needle = query.clone();
                        let (quads, text) = off_thread(move || {
                            let quads = display.search(&needle)?;
                            let text = if quads.is_empty() {
                                None
                            } else {
                                Some(display.text()?)
                            };
                            Ok::<_, prev_pdf::engine::Error>((quads, text))
                        })
                        .await?
                        .map_err(failed)?;
                        for quad in &quads {
                            if matches.len() == limit {
                                more = true;
                                break 'pages;
                            }
                            let line = text.as_ref().and_then(|text| line_of(&text.lines, quad));
                            matches.push(json!({
                                "page": page + 1,
                                "box": rect(quad.bounds()),
                                "line": line,
                            }));
                        }
                    }
                    Ok(Output::Json(json!({ "matches": matches, "more": more })))
                },
                |result| result,
            ))
        }
        shown => reply(answer, Err(not_for(id, shown.what(), "search"))),
    }
}

fn current_view(app: &mut Prev, input: On, answer: &Answer) -> Task<Message> {
    let (id, shown) = match app.shown(input.window) {
        Ok(shown) => shown,
        Err(error) => return reply(answer, Err(error)),
    };
    let mut view = json!({
        "window": number(id),
        "file": app.shown_path(id).map(|path| path.display().to_string()),
        "focused": app.focused == Some(id),
    });
    match shown {
        Shown::Pdf(viewer) => {
            let mut visible: Vec<usize> = viewer
                .layout
                .visible_pages(&viewer.view)
                .into_iter()
                .map(|page| page + 1)
                .collect();
            visible.sort_unstable();
            // The page most in view, as the page box shows it.
            let page = viewer
                .layout
                .current_page(&viewer.view)
                .unwrap_or(viewer.current);
            view["page"] = json!(page + 1);
            view["pages"] = json!(viewer.page_count());
            view["visible_pages"] = json!(visible);
            view["zoom_percent"] = json!((viewer.layout.zoom * 100.0).round());
            view["view_mode"] = json!(match viewer.mode {
                prev::pdf::layout::ViewMode::Continuous => "continuous",
                prev::pdf::layout::ViewMode::SinglePage => "single_page",
                prev::pdf::layout::ViewMode::TwoPages => "two_pages",
            });
            if let Some(text) = viewer.selected_text().filter(|text| !text.is_empty()) {
                view["selected_text"] = json!(text);
            }
        }
        Shown::Image(images) => {
            if let Some(image) = images.shown_image() {
                view["image"] = json!(image.index + 1);
                view["images"] = json!(image.count);
            }
        }
        Shown::Markdown(_) | Shown::Start => {}
    }
    reply(answer, Ok(Output::Json(view)))
}

/// What agents call an annotation's type.
fn annotation_type(annotation: &Annotation) -> String {
    match &annotation.kind {
        annotation::Kind::Markup { style, .. } => match style {
            TextMarkup::Highlight => "highlight",
            TextMarkup::Underline => "underline",
            TextMarkup::StrikeOut => "strikeout",
            TextMarkup::Squiggly => "squiggly",
        }
        .to_owned(),
        annotation::Kind::Ink(_) => "ink".to_owned(),
        annotation::Kind::Square => "rectangle".to_owned(),
        annotation::Kind::Circle => "ellipse".to_owned(),
        annotation::Kind::Line { .. } => "line".to_owned(),
        annotation::Kind::Polygon(_) => "polygon".to_owned(),
        annotation::Kind::PolyLine(_) => "polyline".to_owned(),
        annotation::Kind::FreeText => "text_box".to_owned(),
        annotation::Kind::Note => "note".to_owned(),
        annotation::Kind::Stamp => "stamp".to_owned(),
        annotation::Kind::Redact => "redaction".to_owned(),
        annotation::Kind::Other(name) => name.to_lowercase(),
    }
}

fn annotation_json(page: usize, annotation: &Annotation) -> Value {
    let mut entry = json!({
        "id": annotation.id,
        "page": page + 1,
        "type": annotation_type(annotation),
        "box": rect(annotation.rect),
    });
    if let Some(color) = annotation.style.color {
        entry["color"] = json!(hex(color));
    }
    if let Some(fill) = annotation.style.fill {
        entry["fill"] = json!(hex(fill));
    }
    if !annotation.contents.is_empty() {
        entry["text"] = json!(annotation.contents);
    }
    if let Some(subject) = &annotation.subject {
        entry["subject"] = json!(subject);
    }
    if let Some(author) = &annotation.author {
        entry["author"] = json!(author);
    }
    entry
}

fn list_annotations(app: &mut Prev, input: Annotations, answer: &Answer) -> Task<Message> {
    let viewer = match app.shown_pdf(input.window, "list_annotations") {
        Ok(viewer) => viewer,
        Err(error) => return reply(answer, Err(error)),
    };
    let only = match input.page {
        Some(page) => match page_index(viewer, Some(page)) {
            Ok(page) => Some(page),
            Err(error) => return reply(answer, Err(error)),
        },
        None => None,
    };
    let handle = viewer.handle.clone();
    answer.clone().later(Task::perform(
        async move {
            let pages = answered(handle.all_annotations()).await?;
            let annotations: Vec<Value> = pages
                .iter()
                .filter(|(page, _)| only.is_none_or(|only| only == *page))
                .flat_map(|(page, annotations)| {
                    annotations
                        .iter()
                        .map(move |annotation| annotation_json(*page, annotation))
                })
                .collect();
            Ok(Output::Json(json!({ "annotations": annotations })))
        },
        |result| result,
    ))
}

fn form_fields(app: &mut Prev, input: On, answer: &Answer) -> Task<Message> {
    let viewer = match app.shown_pdf(input.window, "form_fields") {
        Ok(viewer) => viewer,
        Err(error) => return reply(answer, Err(error)),
    };
    let handle = viewer.handle.clone();
    let pages = viewer.page_count();
    answer.clone().later(Task::perform(
        async move {
            let mut fields = Vec::new();
            for page in 0..pages {
                let markup = answered(handle.markup(page)).await?;
                for field in markup.fields {
                    let mut entry = json!({
                        "id": field.id,
                        "page": page + 1,
                        "name": field.name,
                        "box": rect(field.rect),
                        "value": field.value,
                    });
                    entry["type"] = json!(match &field.kind {
                        FieldKind::Text { multiline, .. } => {
                            if *multiline { "text_multiline" } else { "text" }
                        }
                        FieldKind::Checkbox => "checkbox",
                        FieldKind::Radio => "radio",
                        FieldKind::Choice { combo: true, .. } => "dropdown",
                        FieldKind::Choice { .. } => "list",
                        FieldKind::Signature => "signature",
                        FieldKind::Button => "button",
                    });
                    match &field.kind {
                        FieldKind::Checkbox | FieldKind::Radio => {
                            entry["checked"] = json!(field.is_on());
                        }
                        FieldKind::Choice { options, .. } => entry["choices"] = json!(options),
                        _ => {}
                    }
                    if field.read_only {
                        entry["read_only"] = json!(true);
                    }
                    fields.push(entry);
                }
            }
            Ok(Output::Json(json!({ "fields": fields })))
        },
        |result| result,
    ))
}

fn list_signatures(app: &mut Prev, _: Nothing, answer: &Answer) -> Task<Message> {
    let store = SignatureStore::new(app.settings.signatures.clone());
    answer.clone().later(Task::perform(
        async move {
            let signatures: Vec<Value> = off_thread(move || store.list())
                .await?
                .into_iter()
                .map(|signature| {
                    json!({
                        "name": signature.description,
                        "file": signature.file,
                        "created": signature.created,
                    })
                })
                .collect();
            Ok(Output::Json(json!({ "signatures": signatures })))
        },
        |result| result,
    ))
}

fn render_page(app: &mut Prev, input: RenderPage, answer: &Answer) -> Task<Message> {
    let viewer = match app.shown_pdf(input.window, "render_page") {
        Ok(viewer) => viewer,
        Err(error) => return reply(answer, Err(error)),
    };
    let page = match page_index(viewer, input.page) {
        Ok(page) => page,
        Err(error) => return reply(answer, Err(error)),
    };
    let width = input.width.unwrap_or(1024).clamp(16, MAX_SIDE);
    let size = viewer.info.page_sizes[page];
    let handle = viewer.handle.clone();
    answer.clone().later(Task::perform(
        async move {
            let display = display(&handle, page).await?;
            // As wide as asked, and no taller than the widest picture.
            let scale = (width as f32 / size.width).min(MAX_SIDE as f32 / size.height);
            let (pixels_wide, pixels_high) = page_pixels(size, scale);
            let png = off_thread(move || {
                let bitmap = display.render(
                    scale,
                    prev_pdf::geometry::PixelRect {
                        x: 0,
                        y: 0,
                        width: pixels_wide,
                        height: pixels_high,
                    },
                )?;
                Ok::<_, prev_pdf::engine::Error>(prev::drag::png(&bitmap))
            })
            .await?
            .map_err(failed)?
            .ok_or_else(|| failed("the page could not be made into a PNG"))?;
            Ok(Output::Png(
                png,
                format!(
                    "Page {}, {} by {} points, drawn {pixels_wide} by {pixels_high} pixels: \
                     {scale:.3} pixels a point.",
                    page + 1,
                    tenth(size.width),
                    tenth(size.height),
                ),
            ))
        },
        |result| result,
    ))
}

fn render_image(app: &mut Prev, input: RenderImage, answer: &Answer) -> Task<Message> {
    let side = input.max_side.unwrap_or(1024).clamp(16, MAX_SIDE);
    picture(app, input.window, Some(side), "render_image", answer)
}

fn get_image(app: &mut Prev, input: On, answer: &Answer) -> Task<Message> {
    picture(app, input.window, None, "get_image", answer)
}

/// Draws the image a window shows as a PNG, fitting `max_side` if given.
fn picture(
    app: &Prev,
    window: Option<u64>,
    max_side: Option<u32>,
    tool: &str,
    answer: &Answer,
) -> Task<Message> {
    let image = match app.shown(window) {
        Ok((_, Shown::Image(images))) => images.shown_image(),
        Ok((id, shown)) => return reply(answer, Err(not_for(id, shown.what(), tool))),
        Err(error) => return reply(answer, Err(error)),
    };
    let Some(image) = image else {
        return reply(answer, Err(invalid("The image is still loading.")));
    };
    if max_side.is_none() && u64::from(image.width) * u64::from(image.height) > MAX_PIXELS {
        return reply(
            answer,
            Err(invalid(format!(
                "The image is {} by {} pixels, too large to send; use render_image.",
                image.width, image.height
            ))),
        );
    }
    let (width, height) = (image.width, image.height);
    let pixels = image.pixels;
    answer.clone().later(Task::perform(
        async move {
            let frame = off_thread(move || pixels.draw(max_side))
                .await?
                .map_err(failed)?;
            let (drawn_width, drawn_height) = (frame.width, frame.height);
            let png = off_thread(move || {
                prev_image::encode::encode(&frame, prev_image::encode::SaveFormat::Png)
            })
            .await?
            .map_err(|error| failed(format!("{error:?}")))?;
            Ok(Output::Png(
                png,
                format!(
                    "The image, {width} by {height} pixels, drawn {drawn_width} by \
                     {drawn_height}."
                ),
            ))
        },
        |result| result,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use prev_pdf::geometry::Point;

    #[test]
    fn annotations_read_as_agents_name_them() {
        let mut highlight = Annotation::new(
            "a1",
            annotation::Kind::Markup {
                style: TextMarkup::Highlight,
                quads: Vec::new(),
            },
            Rect::new(10.04, 20.0, 30.0, 40.06),
        );
        highlight.style.color = Some(annotation::Rgb::from_rgb8(255, 204, 0));
        highlight.contents = "Check this".to_owned();
        let entry = annotation_json(2, &highlight);
        assert_eq!(entry["page"], 3);
        assert_eq!(entry["type"], "highlight");
        assert_eq!(entry["box"], json!([10.0, 20.0, 30.0, 40.1]));
        assert_eq!(entry["color"], "#ffcc00");
        assert_eq!(entry["text"], "Check this");
        let note = Annotation::new("a2", annotation::Kind::FreeText, Rect::default());
        assert_eq!(annotation_json(0, &note)["type"], "text_box");
        assert!(annotation_json(0, &note).get("text").is_none());
    }

    #[test]
    fn outline_entries_give_pages_from_one() {
        let outline = vec![OutlineItem {
            title: "Intro".to_owned(),
            target: Some(LinkTarget::Page {
                index: 0,
                point: Some(Point::new(0.0, 0.0)),
            }),
            children: vec![OutlineItem {
                title: "Web".to_owned(),
                target: Some(LinkTarget::Uri("https://example.com".to_owned())),
                children: Vec::new(),
            }],
        }];
        assert_eq!(
            outline_json(&outline),
            json!([{ "title": "Intro", "page": 1, "children": [{ "title": "Web" }] }])
        );
    }
}
