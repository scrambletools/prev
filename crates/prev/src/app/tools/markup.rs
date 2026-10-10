//! Tools that mark up a PDF: highlights, notes, text boxes, shapes and
//! redaction marks, and changing or removing annotations. Each change goes
//! through the markup tools' own path, so it is one step of Undo and saves
//! as any edit does. Positions are in page points, from the page's top-left
//! corner.

use iced::{Task, window};
use prev::control::Error;
use prev::pdf::markup::{self, Shape};
use prev::pdf::viewer::PdfViewer;
use prev::pdf::viewer::editing::{self, AgentEdit, EditMessage, HIGHLIGHT_YELLOW};
use prev::pdf::window as pdf_window;
use prev_pdf::annotation::{Annotation, Kind as AnnotationKind, Rgb, TextMarkup, new_id};
use prev_pdf::geometry::{Point, Quad, Rect};
use prev_pdf::text::TextLayout;
use prev_pdf::worker::DocumentHandle;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

use super::read::{answered, display, failed, invalid, off_thread};
use super::{Answer, Kind, Output, Tool, number, reply, tool};
use crate::app::{Message, Prev};

pub(super) fn tools() -> Vec<Tool> {
    vec![
        tool(
            "highlight",
            "Highlight text",
            Kind::Markup,
            "Highlights, underlines, strikes out or squiggles text on one PDF page: every \
             place the given text appears there (or one of them, or only whole words), or the \
             text inside a box. Gives the new annotation's id.",
            highlight,
        ),
        tool(
            "add_note",
            "Add a note",
            Kind::Markup,
            "Puts a note, shown as an icon the user opens, at a point of a PDF page.",
            add_note,
        ),
        tool(
            "add_text_box",
            "Add a text box",
            Kind::Markup,
            "Writes text on a PDF page in a box whose top-left corner is at a point; the box \
             grows to fit the text.",
            add_text_box,
        ),
        tool(
            "add_shape",
            "Add a shape",
            Kind::Markup,
            "Draws a rectangle, rounded rectangle, ellipse, line, arrow, star, hexagon or \
             speech bubble on a PDF page. To circle something, draw an ellipse a little larger \
             than its box. A line or arrow runs from the box's first corner to its second.",
            add_shape,
        ),
        tool(
            "edit_annotation",
            "Change an annotation",
            Kind::Markup,
            "Changes an annotation's text, colors, line width, font size, or box (which \
             moves and resizes it). Ids come from list_annotations and the markup tools.",
            edit_annotation,
        ),
        tool(
            "delete_annotation",
            "Delete an annotation",
            Kind::Markup,
            "Removes an annotation; Undo brings it back.",
            delete_annotation,
        ),
        tool(
            "add_redaction",
            "Mark for redaction",
            Kind::Markup,
            "Marks a box, or every place some text appears on a PDF page, for redaction. \
             Nothing is removed until apply_redactions runs; until then the marks can be \
             changed or deleted like any annotation.",
            add_redaction,
        ),
    ]
}

/// A color as "#rrggbb", or a common name such as red.
type Hex = String;

#[derive(Deserialize, JsonSchema, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
enum Style {
    #[default]
    Highlight,
    Underline,
    Strikeout,
    Squiggly,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Highlight {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// Text to mark wherever it appears on the page, ignoring case.
    text: Option<String>,
    /// Which place to mark, from 1, when the text appears more than once;
    /// all of them if left out.
    occurrence: Option<usize>,
    /// Only where the text is whole words, so "heat" leaves "Heatmaster"
    /// alone.
    whole_words: Option<bool>,
    /// Mark the text inside this box [x0, y0, x1, y1] instead.
    #[serde(rename = "box")]
    area: Option<[f32; 4]>,
    /// highlight if left out.
    style: Option<Style>,
    /// The color, such as "#ffdb33"; the markup bar's color if left out.
    color: Option<Hex>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AddNote {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// Points from the page's left edge to the note's left edge.
    x: f32,
    /// Points from the page's top edge to the note's bottom edge.
    y: f32,
    /// What the note says.
    text: String,
    /// The icon's color; yellow if left out.
    color: Option<Hex>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AddTextBox {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// Points from the page's left edge to the box's left edge.
    x: f32,
    /// Points from the page's top edge to the box's top edge.
    y: f32,
    /// The text; new lines start new lines in the box.
    text: String,
    /// The text size in points; the markup bar's if left out.
    font_size: Option<f32>,
    /// The text's color.
    text_color: Option<Hex>,
    /// The border's color, or "none".
    border: Option<Hex>,
    /// The fill color, or "none".
    fill: Option<Hex>,
    /// The narrowest the box may be, in points.
    min_width: Option<f32>,
}

#[derive(Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum ShapeName {
    Rectangle,
    RoundedRectangle,
    Ellipse,
    Line,
    Arrow,
    Star,
    Hexagon,
    SpeechBubble,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AddShape {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    shape: ShapeName,
    /// The shape's box [x0, y0, x1, y1]; a line or arrow runs from (x0, y0)
    /// to (x1, y1), the arrow head at the end.
    #[serde(rename = "box")]
    area: [f32; 4],
    /// The outline's color, or "none"; the markup bar's if left out.
    color: Option<Hex>,
    /// The fill color, or "none".
    fill: Option<Hex>,
    /// The outline's width in points.
    line_width: Option<f32>,
    dashed: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditAnnotation {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The annotation's page, from 1.
    page: usize,
    /// The annotation's id.
    id: String,
    /// New text, for a note, text box or highlight's comment.
    text: Option<String>,
    color: Option<Hex>,
    fill: Option<Hex>,
    line_width: Option<f32>,
    font_size: Option<f32>,
    /// A new box [x0, y0, x1, y1], which moves and resizes it.
    #[serde(rename = "box")]
    area: Option<[f32; 4]>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteAnnotation {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The annotation's page, from 1.
    page: usize,
    /// The annotation's id.
    id: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AddRedaction {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// The box [x0, y0, x1, y1] to mark.
    #[serde(rename = "box")]
    area: Option<[f32; 4]>,
    /// Text to mark wherever it appears on the page, ignoring case, one
    /// mark for each place.
    text: Option<String>,
    /// Only where the text is whole words.
    whole_words: Option<bool>,
}

/// `"#rrggbb"` as a color, or `None` for "none".
pub(super) fn color(hex: &str) -> Result<Option<Rgb>, Error> {
    let hex = hex.trim();
    if hex.eq_ignore_ascii_case("none") {
        return Ok(None);
    }
    // Common names, as models often give them.
    let named = match hex.to_ascii_lowercase().as_str() {
        "red" => Some("#e01b24"),
        "orange" => Some("#ff7800"),
        "yellow" => Some("#ffdb33"),
        "green" => Some("#2ec27e"),
        "blue" => Some("#1a5fb4"),
        "purple" => Some("#9141ac"),
        "pink" => Some("#f66151"),
        "brown" => Some("#865e3c"),
        "black" => Some("#000000"),
        "white" => Some("#ffffff"),
        "gray" | "grey" => Some("#77767b"),
        _ => None,
    };
    let hex = named.unwrap_or(hex);
    let digits = hex.strip_prefix('#').unwrap_or(hex);
    let channel = |at: usize| u8::from_str_radix(digits.get(at..at + 2).unwrap_or(""), 16);
    match (digits.len(), channel(0), channel(2), channel(4)) {
        (6, Ok(red), Ok(green), Ok(blue)) => Ok(Some(Rgb::from_rgb8(red, green, blue))),
        _ => Err(invalid(format!(
            "{hex} is not a color; give one as \"#rrggbb\" or a name such as red or blue."
        ))),
    }
}

/// A box given as [x0, y0, x1, y1], in either corner order.
pub(super) fn area([x0, y0, x1, y1]: [f32; 4]) -> Rect {
    Rect::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
}

/// The window a markup tool acts on, with its viewer and page index.
pub(super) struct Target<'a> {
    pub(super) id: window::Id,
    pub(super) viewer: &'a PdfViewer,
    pub(super) page: usize,
    pub(super) route: Route,
    /// Page points per unit of the tool's positions: 1 for PDFs, and
    /// the image's points per pixel for an image's markup.
    pub(super) unit: f32,
}

/// Where a window's markup edits go.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Route {
    Pdf,
    /// The markup of the image at this index in an image window.
    Image(usize),
}

/// Where a markup tool's edit goes, kept once the window is let go.
#[derive(Clone, Copy)]
pub(super) struct Place {
    id: window::Id,
    page: usize,
    route: Route,
}

impl Place {
    pub(super) fn id(self) -> window::Id {
        self.id
    }

    pub(super) fn page(self) -> usize {
        self.page
    }

    pub(super) fn route(self) -> Route {
        self.route
    }
}

impl Target<'_> {
    pub(super) fn place(&self) -> Place {
        Place {
            id: self.id,
            page: self.page,
            route: self.route,
        }
    }

    pub(super) fn point(&self, x: f32, y: f32) -> Point {
        Point::new(x * self.unit, y * self.unit)
    }

    pub(super) fn area(&self, given: [f32; 4]) -> Rect {
        let rect = area(given);
        Rect::new(
            rect.x0 * self.unit,
            rect.y0 * self.unit,
            rect.x1 * self.unit,
            rect.y1 * self.unit,
        )
    }
}

impl Prev {
    pub(super) fn markup_target(
        &self,
        window: Option<u64>,
        page: usize,
        tool: &str,
    ) -> Result<Target<'_>, Error> {
        let id = self.tool_window(window)?;
        if let (_, super::read::Shown::Image(images)) = self.shown(window)? {
            let (index, viewer, scale) = images.agent_markup().ok_or_else(|| {
                invalid(
                    "Open the image's markup first, with show_panel and panel markup_bar; \
                     then give positions in image pixels, with page 1.",
                )
            })?;
            let viewer = viewer.ok_or_else(|| invalid("The image's markup is still opening."))?;
            return Ok(Target {
                id,
                viewer,
                page: 0,
                route: Route::Image(index),
                unit: 1.0 / scale,
            });
        }
        let viewer = self.shown_pdf(window, tool)?;
        let pages = viewer.page_count();
        if !(1..=pages).contains(&page) {
            return Err(invalid(format!(
                "Page {page} is not in the document, which has {pages} pages."
            )));
        }
        Ok(Target {
            id,
            viewer,
            page: page - 1,
            route: Route::Pdf,
            unit: 1.0,
        })
    }

    /// Makes `edits` in window `id`'s PDF, then answers with `output`.
    pub(in crate::app) fn agent_markup(
        &mut self,
        id: window::Id,
        route: Route,
        answer: &Answer,
        edits: Vec<AgentEdit>,
        output: Output,
    ) -> Task<Message> {
        if !self.windows.contains_key(&id) {
            return reply(
                answer,
                Err(invalid(format!("Window {} closed.", number(id)))),
            );
        }
        let tasks: Vec<_> = edits
            .into_iter()
            .map(|edit| {
                let edit = pdf_window::Message::Edit(EditMessage::Agent(Box::new(edit)));
                self.update(match route {
                    Route::Pdf => Message::Pdf(id, edit),
                    Route::Image(index) => {
                        Message::Image(id, prev::image::window::Message::Markup(index, edit))
                    }
                })
            })
            .collect();
        answer.send(Ok(output));
        Task::batch(tasks)
    }
}

/// Answers once `work` finds the edits to make in window `id`.
pub(super) fn later_edits(
    id: window::Id,
    route: Route,
    answer: &Answer,
    work: impl Future<Output = Result<(Vec<AgentEdit>, Output), Error>> + Send + 'static,
) -> Task<Message> {
    let answer = answer.clone();
    Task::perform(work, move |result| match result {
        Ok((edits, output)) => Message::ToolMarkup(id, route, answer.clone(), edits, output),
        Err(error) => Message::ToolAnswer(answer.clone(), Err(error)),
    })
}

/// Page `page`'s text layout.
async fn page_text(handle: &DocumentHandle, page: usize) -> Result<TextLayout, Error> {
    let display = display(handle, page).await?;
    off_thread(move || display.text()).await?.map_err(failed)
}

/// Where `text` appears on page `page`, ignoring case; only as whole
/// words if `whole_words`.
async fn find(
    handle: &DocumentHandle,
    page: usize,
    text: String,
    whole_words: bool,
) -> Result<Vec<Quad>, Error> {
    if whole_words {
        return Ok(find_words(&page_text(handle, page).await?, &text));
    }
    let display = display(handle, page).await?;
    off_thread(move || display.search(&text))
        .await?
        .map_err(failed)
}

/// Where `text` appears in `layout` as whole words, ignoring case: one quad
/// for each place, within a line.
fn find_words(layout: &TextLayout, text: &str) -> Vec<Quad> {
    let wanted: Vec<char> = text.trim().chars().flat_map(char::to_lowercase).collect();
    if wanted.is_empty() {
        return Vec::new();
    }
    let mut found = Vec::new();
    for line in &layout.lines {
        let chars: Vec<char> = line
            .chars
            .iter()
            .map(|glyph| {
                glyph
                    .character
                    .to_lowercase()
                    .next()
                    .unwrap_or(glyph.character)
            })
            .collect();
        let word = |at: usize| chars.get(at).is_some_and(|c| c.is_alphanumeric());
        let mut start = 0;
        while start + wanted.len() <= chars.len() {
            let end = start + wanted.len();
            if chars[start..end] == wanted[..] && !(start > 0 && word(start - 1)) && !word(end) {
                if let Some(bounds) = line.chars[start..end]
                    .iter()
                    .map(|glyph| glyph.quad.bounds())
                    .reduce(|a, b| a.union(&b))
                {
                    found.push(Quad::from(bounds));
                }
                start = end;
            } else {
                start += 1;
            }
        }
    }
    found
}

/// The text inside `area`, as one quad per line, with the text itself. A
/// character counts when its middle is inside the box across and the box
/// covers a third of its height, as boxes drawn around text are rough.
fn text_in(layout: &TextLayout, area: Rect) -> (Vec<Quad>, String) {
    let inside = |bounds: Rect| {
        let middle = (bounds.x0 + bounds.x1) / 2.0;
        let covered = bounds.y1.min(area.y1) - bounds.y0.max(area.y0);
        (area.x0..=area.x1).contains(&middle) && covered >= bounds.height() / 3.0
    };
    let mut quads = Vec::new();
    let mut lines = Vec::new();
    for line in &layout.lines {
        let inside: Vec<_> = line
            .chars
            .iter()
            .filter(|glyph| inside(glyph.quad.bounds()))
            .collect();
        let Some(bounds) = inside
            .iter()
            .map(|glyph| glyph.quad.bounds())
            .reduce(|a, b| a.union(&b))
        else {
            continue;
        };
        quads.push(Quad::from(bounds));
        lines.push(
            inside
                .iter()
                .map(|glyph| glyph.character)
                .collect::<String>(),
        );
    }
    (quads, lines.join("\n"))
}

fn ids(edits: &[AgentEdit]) -> Vec<String> {
    edits
        .iter()
        .filter_map(|edit| match edit {
            AgentEdit::Add { annotation, .. } => Some(annotation.id.clone()),
            _ => None,
        })
        .collect()
}

fn highlight(app: &mut Prev, input: Highlight, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "highlight") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let style = match input.style.unwrap_or_default() {
        Style::Highlight => TextMarkup::Highlight,
        Style::Underline => TextMarkup::Underline,
        Style::Strikeout => TextMarkup::StrikeOut,
        Style::Squiggly => TextMarkup::Squiggly,
    };
    let chosen = match input.color.as_deref().map(color).transpose() {
        Ok(chosen) => chosen.flatten(),
        Err(error) => return reply(answer, Err(error)),
    };
    let color = chosen.unwrap_or(match style {
        TextMarkup::Highlight => target.viewer.edit.markup_color,
        _ => target.viewer.edit.markup_color_for_lines(),
    });
    if matches!(target.route, Route::Image(_)) {
        return reply(
            answer,
            Err(invalid(
                "An image has no text to highlight; draw a shape around it.",
            )),
        );
    }
    let (id, page, handle) = (target.id, target.page, target.viewer.handle.clone());
    let (text, area, occurrence) = (input.text, input.area, input.occurrence);
    let whole_words = input.whole_words == Some(true);
    if text.is_some() == area.is_some() {
        return reply(answer, Err(invalid("Give either text or box.")));
    }
    later_edits(id, Route::Pdf, answer, async move {
        let (quads, contents) = match (text, area) {
            (Some(text), _) => {
                let found = find(&handle, page, text.clone(), whole_words).await?;
                let found = match occurrence {
                    Some(nth) => found
                        .get(nth.wrapping_sub(1))
                        .map(|quad| vec![*quad])
                        .ok_or_else(|| {
                            invalid(format!(
                                "\"{text}\" appears {} times on page {}.",
                                found.len(),
                                page + 1
                            ))
                        })?,
                    None => found,
                };
                (found, text)
            }
            (None, Some(area)) => text_in(&page_text(&handle, page).await?, self::area(area)),
            (None, None) => unreachable!("checked above"),
        };
        if quads.is_empty() {
            return Err(invalid(format!("No text to mark on page {}.", page + 1)));
        }
        let rect = quads
            .iter()
            .map(Quad::bounds)
            .reduce(|a, b| a.union(&b))
            .unwrap_or_default();
        let marks = quads.len();
        let mut annotation =
            Annotation::new(new_id(), AnnotationKind::Markup { style, quads }, rect);
        annotation.style.color = Some(color);
        annotation.contents = contents;
        annotation.subject = Some(
            match style {
                TextMarkup::Highlight => "Highlight",
                TextMarkup::Underline => "Underline",
                TextMarkup::StrikeOut => "Strikethrough",
                TextMarkup::Squiggly => "Squiggly",
            }
            .into(),
        );
        let edits = vec![AgentEdit::Add { page, annotation }];
        let output = Output::Json(json!({ "ids": ids(&edits), "page": page + 1, "places": marks }));
        Ok((edits, output))
    })
}

fn add_note(app: &mut Prev, input: AddNote, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "add_note") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let chosen = match input.color.as_deref().map(color).transpose() {
        Ok(chosen) => chosen.flatten(),
        Err(error) => return reply(answer, Err(error)),
    };
    let size = markup::NOTE_SIZE;
    let at = target.point(input.x, input.y);
    let rect = Rect::new(at.x, at.y - size, at.x + size, at.y);
    let mut annotation = Annotation::new(new_id(), AnnotationKind::Note, rect);
    annotation.style.color = Some(chosen.unwrap_or(HIGHLIGHT_YELLOW));
    annotation.subject = Some("Note".into());
    annotation.contents = input.text;
    let place = target.place();
    added(app, place, annotation, answer)
}

/// Adds `annotation` to page `page` of window `id`, answering with its id.
fn added(app: &mut Prev, place: Place, annotation: Annotation, answer: &Answer) -> Task<Message> {
    let Place { id, page, route } = place;
    let output = Output::Json(json!({ "ids": [annotation.id], "page": page + 1 }));
    app.agent_markup(
        id,
        route,
        answer,
        vec![AgentEdit::Add { page, annotation }],
        output,
    )
}

fn add_text_box(app: &mut Prev, input: AddTextBox, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "add_text_box") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let mut style = target.viewer.edit.text_style;
    let colors = (|| {
        if let Some(hex) = &input.text_color {
            style.text_color = color(hex)?.unwrap_or(Rgb::BLACK);
        }
        if let Some(hex) = &input.border {
            style.color = color(hex)?;
        }
        if let Some(hex) = &input.fill {
            style.fill = color(hex)?;
        }
        Ok::<_, Error>(())
    })();
    if let Err(error) = colors {
        return reply(answer, Err(error));
    }
    if let Some(size) = input.font_size {
        style.font_size = (size * target.unit).clamp(4.0, 144.0);
    }
    let width = (input.min_width.unwrap_or(40.0) * target.unit).max(10.0);
    let at = target.point(input.x, input.y);
    let rect = Rect::new(at.x, at.y, at.x + width, at.y + style.font_size);
    let mut annotation = Annotation::new(new_id(), AnnotationKind::FreeText, rect);
    annotation.style = style;
    annotation.subject = Some("Text Box".into());
    annotation.contents = input.text;
    annotation.rect = editing::fit_text(&annotation);
    let place = target.place();
    added(app, place, annotation, answer)
}

fn add_shape(app: &mut Prev, input: AddShape, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "add_shape") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let mut style = target.viewer.edit.style;
    let colors = (|| {
        if let Some(hex) = &input.color {
            style.color = color(hex)?;
        }
        if let Some(hex) = &input.fill {
            style.fill = color(hex)?;
        }
        Ok::<_, Error>(())
    })();
    if let Err(error) = colors {
        return reply(answer, Err(error));
    }
    if let Some(width) = input.line_width {
        style.line_width = (width * target.unit).clamp(0.25, 48.0);
    }
    if let Some(dashed) = input.dashed {
        style.dashed = dashed;
    }
    let shape = match input.shape {
        ShapeName::Rectangle => Shape::Rectangle,
        ShapeName::RoundedRectangle => Shape::RoundedRectangle,
        ShapeName::Ellipse => Shape::Oval,
        ShapeName::Line => Shape::Line,
        ShapeName::Arrow => Shape::Arrow,
        ShapeName::Star => Shape::Star,
        ShapeName::Hexagon => Shape::Polygon,
        ShapeName::SpeechBubble => Shape::SpeechBubble,
    };
    let [x0, y0, x1, y1] = input.area;
    let (start, end) = if shape.is_line() {
        (target.point(x0, y0), target.point(x1, y1))
    } else {
        let rect = target.area(input.area);
        (Point::new(rect.x0, rect.y0), Point::new(rect.x1, rect.y1))
    };
    let annotation = markup::shape_annotation(shape, start, end, style);
    let place = target.place();
    added(app, place, annotation, answer)
}

/// Finds annotation `id` on page `page`, then makes the edits `change`
/// gives for it.
fn with_annotation(
    app: &mut Prev,
    window: Option<u64>,
    page: usize,
    id: String,
    tool: &str,
    answer: &Answer,
    change: impl FnOnce(usize, f32, Annotation) -> Result<(Vec<AgentEdit>, Output), Error>
    + Send
    + 'static,
) -> Task<Message> {
    let target = match app.markup_target(window, page, tool) {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let (window, page, handle) = (target.id, target.page, target.viewer.handle.clone());
    let (route, unit) = (target.route, target.unit);
    later_edits(window, route, answer, async move {
        let markup = answered(handle.markup(page)).await?;
        let annotation = markup
            .annotations
            .into_iter()
            .find(|annotation| annotation.id == id)
            .ok_or_else(|| {
                invalid(format!(
                    "Page {} has no annotation {id}; see list_annotations.",
                    page + 1
                ))
            })?;
        change(page, unit, annotation)
    })
}

fn edit_annotation(app: &mut Prev, input: EditAnnotation, answer: &Answer) -> Task<Message> {
    let EditAnnotation {
        window,
        page,
        id,
        text,
        color: new_color,
        fill,
        line_width,
        font_size,
        area: new_area,
    } = input;
    with_annotation(
        app,
        window,
        page,
        id,
        "edit_annotation",
        answer,
        move |page, unit, before| {
            if !before.kind.is_editable() {
                return Err(invalid(format!(
                    "Annotation {} is a kind prev shows but does not change.",
                    before.id
                )));
            }
            let scaled = |[x0, y0, x1, y1]: [f32; 4]| [x0 * unit, y0 * unit, x1 * unit, y1 * unit];
            let mut after = match new_area {
                Some(target) => before.resized(area(scaled(target))),
                None => before.clone(),
            };
            if let Some(text) = text {
                after.contents = text;
                if after.kind == AnnotationKind::FreeText && new_area.is_none() {
                    after.rect = editing::fit_text(&after);
                }
            }
            if let Some(hex) = new_color {
                let chosen = color(&hex)?;
                if after.kind == AnnotationKind::FreeText {
                    after.style.text_color = chosen.unwrap_or(Rgb::BLACK);
                } else {
                    after.style.color = chosen;
                }
            }
            if let Some(hex) = fill {
                after.style.fill = color(&hex)?;
            }
            if let Some(width) = line_width {
                after.style.line_width = (width * unit).clamp(0.25, 48.0);
            }
            if let Some(size) = font_size {
                after.style.font_size = (size * unit).clamp(4.0, 144.0);
                if after.kind == AnnotationKind::FreeText && new_area.is_none() {
                    after.rect = editing::fit_text(&after);
                }
            }
            let output = Output::Json(json!({ "id": after.id, "page": page + 1, "changed": true }));
            Ok((
                vec![AgentEdit::Change {
                    page,
                    before: Box::new(before),
                    after: Box::new(after),
                }],
                output,
            ))
        },
    )
}

fn delete_annotation(app: &mut Prev, input: DeleteAnnotation, answer: &Answer) -> Task<Message> {
    with_annotation(
        app,
        input.window,
        input.page,
        input.id,
        "delete_annotation",
        answer,
        |page, _, annotation| {
            let output = Output::Json(json!({ "id": annotation.id, "deleted": true }));
            Ok((vec![AgentEdit::Remove { page, annotation }], output))
        },
    )
}

fn add_redaction(app: &mut Prev, input: AddRedaction, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "add_redaction") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    if matches!(target.route, Route::Image(_)) {
        return reply(
            answer,
            Err(invalid(
                "Redaction is for PDFs; on an image, draw a filled shape over it.",
            )),
        );
    }
    if input.text.is_some() == input.area.is_some() {
        return reply(answer, Err(invalid("Give either text or box.")));
    }
    let (id, page, handle) = (target.id, target.page, target.viewer.handle.clone());
    let (text, given) = (input.text, input.area);
    let whole_words = input.whole_words == Some(true);
    later_edits(id, Route::Pdf, answer, async move {
        let rects: Vec<Rect> = match (text, given) {
            (Some(text), _) => find(&handle, page, text, whole_words)
                .await?
                .iter()
                .map(Quad::bounds)
                .collect(),
            (None, Some(given)) => vec![area(given)],
            (None, None) => unreachable!("checked above"),
        };
        if rects.is_empty() {
            return Err(invalid(format!(
                "The text does not appear on page {}.",
                page + 1
            )));
        }
        let edits: Vec<AgentEdit> = rects
            .into_iter()
            .map(|rect| AgentEdit::Add {
                page,
                annotation: editing::redaction(rect),
            })
            .collect();
        let output = Output::Json(json!({ "ids": ids(&edits), "page": page + 1 }));
        Ok((edits, output))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use prev_pdf::text::{TextChar, TextLine};

    #[test]
    fn colors_read_from_hex() {
        assert_eq!(color("#ff0000").unwrap(), Some(Rgb::from_rgb8(255, 0, 0)));
        assert_eq!(color("00ff7f").unwrap(), Some(Rgb::from_rgb8(0, 255, 127)));
        assert_eq!(color("none").unwrap(), None);
        assert!(color("#ff00").is_err());
        assert_eq!(
            color("Red").unwrap(),
            Some(Rgb::from_rgb8(0xe0, 0x1b, 0x24))
        );
        assert!(color("teal-ish").is_err());
    }

    #[test]
    fn whole_words_leave_longer_words_alone() {
        let line = |text: &str, y: f32| TextLine {
            bounds: Rect::new(0.0, y, 200.0, y + 10.0),
            chars: text
                .chars()
                .enumerate()
                .map(|(index, character)| TextChar {
                    character,
                    quad: Quad::from(Rect::new(
                        index as f32 * 5.0,
                        y,
                        index as f32 * 5.0 + 5.0,
                        y + 10.0,
                    )),
                })
                .collect(),
        };
        let layout = TextLayout {
            lines: vec![
                line("Heat waves and Heatmaster.", 0.0),
                line("the heat", 20.0),
            ],
        };
        let found = find_words(&layout, "heat");
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].bounds(), Rect::new(0.0, 0.0, 20.0, 10.0));
        assert_eq!(found[1].bounds(), Rect::new(20.0, 20.0, 40.0, 30.0));
        assert_eq!(find_words(&layout, "heat waves").len(), 1);
    }

    #[test]
    fn boxes_take_either_corner_first() {
        assert_eq!(
            area([30.0, 40.0, 10.0, 20.0]),
            Rect::new(10.0, 20.0, 30.0, 40.0)
        );
    }

    #[test]
    fn text_in_a_box_is_marked_line_by_line() {
        let glyph = |character, x: f32, y: f32| TextChar {
            character,
            quad: Quad::from(Rect::new(x, y, x + 5.0, y + 10.0)),
        };
        let line = |text: &str, y: f32| TextLine {
            bounds: Rect::new(0.0, y, 100.0, y + 10.0),
            chars: text
                .chars()
                .enumerate()
                .map(|(index, character)| glyph(character, index as f32 * 5.0, y))
                .collect(),
        };
        let layout = TextLayout {
            lines: vec![
                line("total due", 0.0),
                line("paid", 20.0),
                line("later", 40.0),
            ],
        };
        let (quads, text) = text_in(&layout, Rect::new(0.0, 0.0, 25.0, 31.0));
        assert_eq!(text, "total\npaid");
        assert_eq!(quads.len(), 2);
        assert_eq!(quads[0].bounds(), Rect::new(0.0, 0.0, 25.0, 10.0));
        // A box that clips the bottom of a line still takes it, but not
        // one that only grazes it.
        assert_eq!(
            text_in(&layout, Rect::new(0.0, 0.0, 25.0, 46.0)).1,
            "total\npaid\nlater"
        );
        assert_eq!(
            text_in(&layout, Rect::new(0.0, 0.0, 25.0, 42.0)).1,
            "total\npaid"
        );
    }
}
