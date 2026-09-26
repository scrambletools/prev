//! Markup and form editing in the viewer: tools turn presses and drags into
//! annotations, the select tool moves and resizes them, and every change
//! goes to the document thread and into the undo history.

use std::sync::Arc;

use iced::Task;
use iced::widget::text_editor;
use prev_pdf::annotation::{
    Align, Annotation, Field, FieldKind, Font, Kind, Rgb, StampContent, Style, TextMarkup, new_id,
};
use prev_pdf::engine::Bitmap;
use prev_pdf::geometry::{PixelRect, Point, Quad, Rect};
use prev_pdf::worker::{Edit, Edited, PageMarkup, Ticket};

use super::{PdfMessage, PdfViewer, Request};
use crate::pdf::history::{Change, History, Stack, Step};
use crate::pdf::layout;
use crate::pdf::markup::{self, Handle, Shape, Tool};

/// Preview's default markup colors.
pub const RED: Rgb = Rgb::new(0.93, 0.16, 0.14);
pub const HIGHLIGHT_YELLOW: Rgb = Rgb::new(1.0, 0.86, 0.2);
/// A loupe magnifies its area this many times.
const LOUPE_ZOOM: f32 = 2.0;
/// Device pixels per point in a loupe's image.
const LOUPE_RESOLUTION: f32 = 3.0;
const SIGNATURE_WIDTH: f32 = 180.0;
/// A drag shorter than this, in points, counts as a click.
const CLICK_DISTANCE: f32 = 3.0;

/// A FreeText box or note whose text is being typed.
pub struct TextEdit {
    pub page: usize,
    pub annotation: Annotation,
    pub content: text_editor::Content,
    /// Created just now, so an empty result removes it.
    pub fresh: bool,
}

/// A text form field being filled in.
#[derive(Debug, Clone)]
pub struct FieldEdit {
    pub page: usize,
    pub field: Field,
    pub value: String,
}

#[derive(Debug, Clone)]
pub(super) enum Drag {
    Stroke {
        page: usize,
        points: Vec<Point>,
        sketch: bool,
    },
    Create {
        page: usize,
        creation: Creation,
        start: Point,
        current: Point,
    },
    Move {
        page: usize,
        original: Box<Annotation>,
        handle: Handle,
        from: Point,
        current: Point,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Creation {
    Shape(Shape),
    TextBox,
    Note,
    Area,
    Redact,
}

pub struct Editing {
    pub tool: Tool,
    /// Style for new shapes, lines and drawings.
    pub style: Style,
    /// Style for new text boxes.
    pub text_style: Style,
    pub markup_color: Rgb,
    pub selected: Option<(usize, String)>,
    pub(super) drag: Option<Drag>,
    pub history: History,
    pub text: Option<TextEdit>,
    pub field: Option<FieldEdit>,
    /// A drop-down field whose choices are showing.
    pub choice: Option<(usize, Field)>,
    /// An area chosen with the rectangular selection tool.
    pub area: Option<(usize, Rect)>,
}

impl Default for Editing {
    fn default() -> Self {
        Self {
            tool: Tool::Select,
            style: Style {
                color: Some(RED),
                ..Style::default()
            },
            text_style: Style {
                color: None,
                line_width: 1.0,
                ..Style::default()
            },
            markup_color: HIGHLIGHT_YELLOW,
            selected: None,
            drag: None,
            history: History::default(),
            text: None,
            field: None,
            choice: None,
            area: None,
        }
    }
}

/// How the annotation style changes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleChange {
    Color(Option<Rgb>),
    Fill(Option<Rgb>),
    LineWidth(f32),
    Dashed(bool),
    Font(Font),
    FontSize(f32),
    TextColor(Rgb),
    Align(Align),
}

impl StyleChange {
    fn apply(self, style: &mut Style) {
        match self {
            StyleChange::Color(color) => style.color = color,
            StyleChange::Fill(fill) => style.fill = fill,
            StyleChange::LineWidth(width) => style.line_width = width,
            StyleChange::Dashed(dashed) => style.dashed = dashed,
            StyleChange::Font(font) => style.font = font,
            StyleChange::FontSize(size) => style.font_size = size,
            StyleChange::TextColor(color) => style.text_color = color,
            StyleChange::Align(align) => style.align = align,
        }
    }

    fn is_text(self) -> bool {
        matches!(
            self,
            StyleChange::Font(_)
                | StyleChange::FontSize(_)
                | StyleChange::TextColor(_)
                | StyleChange::Align(_)
        )
    }
}

#[derive(Debug, Clone)]
pub enum EditMessage {
    SetTool(Tool),
    Style(StyleChange),
    MarkupColor(Rgb),
    Delete,
    Deselect,
    Undo,
    Redo,
    TextAction(text_editor::Action),
    CommitText,
    FieldInput(String),
    CommitField,
    Choose(String),
    CloseChoice,
    /// Places an image, such as a signature, on the current page.
    PlaceStamp {
        image: Arc<Bitmap>,
        subject: String,
    },
    LoupeRendered(Box<LoupeRender>),
    /// An area was copied, or could not be.
    Copied(Option<String>),
}

/// A loupe's magnified image, ready to add or replace.
#[derive(Debug, Clone)]
pub struct LoupeRender {
    page: usize,
    annotation: Annotation,
    before: Option<Annotation>,
    image: Option<Arc<Bitmap>>,
}

/// What to do once an edit is back from the document thread.
#[derive(Debug, Clone, PartialEq)]
pub struct Sent {
    page: usize,
    /// Where the change went, for a removal token.
    stack: Option<Stack>,
    select: Option<String>,
    open_text: bool,
}

impl Sent {
    fn plain(page: usize) -> Self {
        Self {
            page,
            stack: None,
            select: None,
            open_text: false,
        }
    }
}

impl PdfViewer {
    /// Points per logical pixel at the current zoom.
    fn points_per_pixel(&self) -> f32 {
        1.0 / layout::points_to_pixels(self.layout.zoom)
    }

    pub fn annotation(&self, page: usize, id: &str) -> Option<&Annotation> {
        self.markup
            .get(&page)?
            .annotations
            .iter()
            .find(|annotation| annotation.id == id)
    }

    pub fn selected_annotation(&self) -> Option<(usize, &Annotation)> {
        let (page, id) = self.edit.selected.as_ref()?;
        Some((*page, self.annotation(*page, id)?))
    }

    /// The annotation being created or moved, as it would look now.
    pub fn preview(&self) -> Option<(usize, Annotation)> {
        match self.edit.drag.as_ref()? {
            Drag::Stroke {
                page,
                points,
                sketch,
            } => {
                let mut annotation = markup::stroke_annotation(points, false, self.edit.style)?;
                if *sketch {
                    annotation.style.dashed = false;
                }
                Some((*page, annotation))
            }
            Drag::Create {
                page,
                creation: Creation::Shape(shape),
                start,
                current,
            } => {
                let (start, end) = self.constrained(*shape, *start, *current);
                Some((
                    *page,
                    markup::shape_annotation(*shape, start, end, self.edit.style),
                ))
            }
            Drag::Create {
                page,
                creation: Creation::TextBox | Creation::Area,
                start,
                current,
            } => {
                let mut annotation =
                    Annotation::new("", Kind::Square, markup::normalized(*start, *current));
                annotation.style = Style {
                    color: Some(Rgb::new(0.4, 0.4, 0.4)),
                    line_width: 1.0,
                    dashed: true,
                    ..Style::default()
                };
                Some((*page, annotation))
            }
            Drag::Create {
                page,
                creation: Creation::Redact,
                start,
                current,
            } => Some((*page, redaction(markup::normalized(*start, *current)))),
            Drag::Create { .. } => None,
            Drag::Move {
                page,
                original,
                handle,
                from,
                current,
            } => Some((
                *page,
                markup::dragged(original, *handle, *from, *current, self.shift),
            )),
        }
    }

    /// Shift keeps boxes square and lines at 45 degree steps.
    fn constrained(&self, shape: Shape, start: Point, current: Point) -> (Point, Point) {
        if !self.shift {
            return (start, current);
        }
        let (dx, dy) = (current.x - start.x, current.y - start.y);
        if shape.is_line() {
            let angle = dy.atan2(dx);
            let step = std::f32::consts::FRAC_PI_4;
            let snapped = (angle / step).round() * step;
            let length = dx.hypot(dy);
            return (
                start,
                Point::new(
                    start.x + length * snapped.cos(),
                    start.y + length * snapped.sin(),
                ),
            );
        }
        let side = dx.abs().max(dy.abs());
        (
            start,
            Point::new(start.x + side * dx.signum(), start.y + side * dy.signum()),
        )
    }

    fn page_point(&self, page: usize, x: f32, y: f32) -> Point {
        self.layout.to_page(page, x, y).unwrap_or_default()
    }

    fn field_at(&self, page: usize, point: Point) -> Option<&Field> {
        self.markup.get(&page)?.fields.iter().find(|field| {
            field.rect.contains(point)
                && !field.read_only
                && !matches!(field.kind, FieldKind::Button | FieldKind::Signature)
        })
    }

    /// Handles a press for the markup tools, annotations and form fields.
    /// `None` leaves it to text selection.
    pub(super) fn editing_press(&mut self, x: f32, y: f32, clicks: u8) -> Option<Task<PdfMessage>> {
        let mut tasks = Vec::new();
        if self.edit.text.is_some() {
            tasks.push(self.commit_text());
        }
        if self.edit.field.is_some() {
            tasks.push(self.commit_field());
        }
        self.edit.choice = None;
        let hit = self.layout.hit(x, y);
        let create = |creation| {
            hit.map(|(page, point)| Drag::Create {
                page,
                creation,
                start: point,
                current: point,
            })
        };
        match self.edit.tool {
            Tool::Draw | Tool::Sketch => {
                self.edit.selected = None;
                self.edit.drag = hit.map(|(page, point)| Drag::Stroke {
                    page,
                    points: vec![point],
                    sketch: self.edit.tool == Tool::Sketch,
                });
                return Some(Task::batch(tasks));
            }
            Tool::Shape(shape) => {
                self.edit.selected = None;
                self.edit.drag = create(Creation::Shape(shape));
                return Some(Task::batch(tasks));
            }
            Tool::TextBox => {
                self.edit.selected = None;
                self.edit.drag = create(Creation::TextBox);
                return Some(Task::batch(tasks));
            }
            Tool::Note => {
                self.edit.selected = None;
                self.edit.drag = create(Creation::Note);
                return Some(Task::batch(tasks));
            }
            Tool::Area => {
                self.edit.selected = None;
                self.edit.area = None;
                self.edit.drag = create(Creation::Area);
                return Some(Task::batch(tasks));
            }
            Tool::Highlight(_) => {
                self.edit.selected = None;
                tasks.push(self.text_press(x, y, clicks));
                return Some(Task::batch(tasks));
            }
            Tool::Redact => {
                self.edit.selected = None;
                self.edit.drag = create(Creation::Redact);
                return Some(Task::batch(tasks));
            }
            Tool::Select => {}
        }

        let per_pixel = self.points_per_pixel();
        let handle_size = markup::HANDLE_PIXELS * per_pixel;
        if let Some((page, annotation)) = self
            .selected_annotation()
            .map(|(page, annotation)| (page, annotation.clone()))
        {
            let point = self.page_point(page, x, y);
            if let Some(handle) = markup::hit_handle(&annotation, point, handle_size) {
                if clicks >= 2 && self.open_text(page, &annotation) {
                    return Some(Task::batch(tasks));
                }
                self.start_move(page, annotation, handle, point);
                return Some(Task::batch(tasks));
            }
        }
        if let Some((page, point)) = hit {
            let slop = markup::HIT_SLOP * per_pixel.max(1.0);
            let found = self.markup.get(&page).and_then(|markup| {
                markup::hit_annotation(&markup.annotations, point, slop).cloned()
            });
            if let Some(annotation) = found {
                self.edit.selected = Some((page, annotation.id.clone()));
                let opens = annotation.kind == Kind::Note
                    || clicks >= 2 && annotation.kind == Kind::FreeText;
                if opens {
                    self.open_text(page, &annotation);
                } else {
                    self.start_move(page, annotation, Handle::Body, point);
                }
                return Some(Task::batch(tasks));
            }
            if let Some(field) = self.field_at(page, point).cloned() {
                self.edit.selected = None;
                tasks.push(self.press_field(page, field));
                return Some(Task::batch(tasks));
            }
            if !self.is_over_text(x, y)
                && let Some(mask) = self
                    .markup
                    .get(&page)
                    .and_then(|markup| markup::hit_mask(&markup.annotations, point))
            {
                self.edit.selected = Some((page, mask.id.clone()));
                return Some(Task::batch(tasks));
            }
        }
        self.edit.selected = None;
        if tasks.is_empty() {
            None
        } else {
            tasks.push(self.text_press(x, y, clicks));
            Some(Task::batch(tasks))
        }
    }

    fn start_move(&mut self, page: usize, annotation: Annotation, handle: Handle, point: Point) {
        let allowed = match handle {
            Handle::Body => markup::movable(&annotation),
            Handle::Edge { .. } => markup::resizable(&annotation),
            Handle::LineStart | Handle::LineEnd => true,
        };
        if allowed {
            self.edit.drag = Some(Drag::Move {
                page,
                original: Box::new(annotation),
                handle,
                from: point,
                current: point,
            });
        }
    }

    pub(super) fn editing_drag(&mut self, x: f32, y: f32) -> bool {
        let Some(drag) = self.edit.drag.as_mut() else {
            return false;
        };
        let page = match drag {
            Drag::Stroke { page, .. } | Drag::Create { page, .. } | Drag::Move { page, .. } => {
                *page
            }
        };
        let point = self.layout.to_page(page, x, y).unwrap_or_default();
        match drag {
            Drag::Stroke { points, .. } => points.push(point),
            Drag::Create { current, .. } | Drag::Move { current, .. } => *current = point,
        }
        true
    }

    pub(super) fn editing_release(&mut self, _x: f32, _y: f32) -> Option<Task<PdfMessage>> {
        let Some(drag) = self.edit.drag.take() else {
            if let Tool::Highlight(style) = self.edit.tool {
                self.press = None;
                return Some(self.highlight_selection(style));
            }
            return None;
        };
        let task = match drag {
            Drag::Stroke {
                page,
                points,
                sketch,
            } => match markup::stroke_annotation(&points, sketch, self.edit.style) {
                Some(annotation) => self.add(page, annotation, None, false),
                None => Task::none(),
            },
            Drag::Create {
                page,
                creation,
                start,
                current,
            } => self.create(page, creation, start, current),
            Drag::Move {
                page,
                original,
                handle,
                from,
                current,
            } => {
                let moved = markup::dragged(&original, handle, from, current, self.shift);
                if (current.x - from.x).hypot(current.y - from.y) < CLICK_DISTANCE / 2.0 {
                    Task::none()
                } else if original.subject.as_deref() == Some("Loupe") {
                    self.render_loupe(page, moved, Some(*original))
                } else {
                    self.change(page, *original, moved, None, None)
                }
            }
        };
        if !self.edit.tool.is_sticky() {
            self.edit.tool = Tool::Select;
        }
        Some(task)
    }

    fn create(
        &mut self,
        page: usize,
        creation: Creation,
        start: Point,
        current: Point,
    ) -> Task<PdfMessage> {
        let dragged = (current.x - start.x).hypot(current.y - start.y) >= CLICK_DISTANCE;
        match creation {
            Creation::Shape(shape) => {
                let (start, end) = if dragged {
                    self.constrained(shape, start, current)
                } else if shape.is_line() {
                    (start, Point::new(start.x + markup::DEFAULT_SHAPE, start.y))
                } else {
                    let half = markup::DEFAULT_SHAPE / 2.0;
                    (
                        Point::new(start.x - half, start.y - half),
                        Point::new(start.x + half, start.y + half),
                    )
                };
                let mut annotation = markup::shape_annotation(shape, start, end, self.edit.style);
                match shape {
                    Shape::Loupe => self.render_loupe(page, annotation, None),
                    Shape::Mask => {
                        let hole = annotation.rect;
                        let size = self.info.page_sizes[page];
                        annotation.rect = Rect::new(0.0, 0.0, size.width, size.height);
                        let content = StampContent::Mask {
                            hole,
                            round: false,
                            opacity: 0.55,
                        };
                        self.add(page, annotation, Some(content), false)
                    }
                    _ => self.add(page, annotation, None, false),
                }
            }
            Creation::TextBox => {
                let style = self.edit.text_style;
                let height = style.font_size * 1.4 + 8.0;
                let rect = if dragged {
                    markup::normalized(start, current)
                } else {
                    Rect::new(start.x, start.y, start.x + 200.0, start.y + height)
                };
                let mut annotation = Annotation::new(new_id(), Kind::FreeText, rect);
                annotation.style = style;
                annotation.subject = Some("Text Box".into());
                self.add(page, annotation, None, true)
            }
            Creation::Area => {
                if dragged {
                    self.edit.area = Some((page, markup::normalized(start, current)));
                }
                Task::none()
            }
            Creation::Redact => {
                if !dragged {
                    return Task::none();
                }
                let annotation = redaction(markup::normalized(start, current));
                self.add(page, annotation, None, false)
            }
            Creation::Note => {
                let size = markup::NOTE_SIZE;
                let rect = Rect::new(start.x, start.y - size, start.x + size, start.y);
                let mut annotation = Annotation::new(new_id(), Kind::Note, rect);
                annotation.style.color = Some(HIGHLIGHT_YELLOW);
                annotation.subject = Some("Note".into());
                self.add(page, annotation, None, true)
            }
        }
    }

    /// Marks the selected text for redaction, one mark per line.
    pub fn redact_selection(&mut self) -> Task<PdfMessage> {
        let Some(selection) = self.selection else {
            return Task::none();
        };
        let first = selection.anchor.0.min(selection.focus.0);
        let last = selection.anchor.0.max(selection.focus.0);
        let mut tasks = Vec::new();
        for page in first..=last {
            let (Some(range), Some(text)) = (self.page_selection(page), self.texts.get(&page))
            else {
                continue;
            };
            for rect in text.highlight(range) {
                tasks.push(self.add(page, redaction(rect), None, false));
            }
        }
        self.selection = None;
        Task::batch(tasks)
    }

    /// Turns the text selection into highlight, underline or strikethrough
    /// annotations, one per page.
    pub fn highlight_selection(&mut self, style: TextMarkup) -> Task<PdfMessage> {
        let Some(selection) = self.selection else {
            return Task::none();
        };
        let first = selection.anchor.0.min(selection.focus.0);
        let last = selection.anchor.0.max(selection.focus.0);
        let mut tasks = Vec::new();
        for page in first..=last {
            let (Some(range), Some(text)) = (self.page_selection(page), self.texts.get(&page))
            else {
                continue;
            };
            let quads: Vec<Quad> = text.highlight(range).into_iter().map(Quad::from).collect();
            if quads.is_empty() {
                continue;
            }
            let rect = quads
                .iter()
                .map(Quad::bounds)
                .reduce(|a, b| a.union(&b))
                .unwrap_or_default();
            let mut annotation = Annotation::new(new_id(), Kind::Markup { style, quads }, rect);
            annotation.style.color = Some(match style {
                TextMarkup::Highlight => self.edit.markup_color,
                _ => self.edit.markup_color_for_lines(),
            });
            annotation.contents = text.text(range);
            annotation.subject = Some(
                match style {
                    TextMarkup::Highlight => "Highlight",
                    TextMarkup::Underline => "Underline",
                    TextMarkup::StrikeOut => "Strikethrough",
                    TextMarkup::Squiggly => "Squiggly",
                }
                .into(),
            );
            tasks.push(self.add(page, annotation, None, false));
        }
        self.selection = None;
        Task::batch(tasks)
    }

    fn press_field(&mut self, page: usize, field: Field) -> Task<PdfMessage> {
        match &field.kind {
            FieldKind::Text { .. } => {
                self.edit.field = Some(FieldEdit {
                    page,
                    value: field.value.clone(),
                    field,
                });
                Task::none()
            }
            FieldKind::Checkbox => {
                let value = if field.is_on() {
                    "Off".to_owned()
                } else {
                    field.on_value.clone().unwrap_or_else(|| "Yes".into())
                };
                self.set_field(page, &field, value)
            }
            FieldKind::Radio => match field.on_value.clone() {
                Some(value) if !field.is_on() => self.set_field(page, &field, value),
                _ => Task::none(),
            },
            FieldKind::Choice { .. } => {
                self.edit.choice = Some((page, field));
                Task::none()
            }
            FieldKind::Signature | FieldKind::Button => Task::none(),
        }
    }

    fn set_field(&mut self, page: usize, field: &Field, value: String) -> Task<PdfMessage> {
        if value == field.value {
            return Task::none();
        }
        self.edit.history.record(Change::Field {
            page,
            id: field.id,
            before: field.value.clone(),
            after: value.clone(),
        });
        self.send(
            page,
            Edit::SetField {
                id: field.id,
                value,
            },
            Sent::plain(page),
        )
    }

    fn commit_field(&mut self) -> Task<PdfMessage> {
        let Some(edit) = self.edit.field.take() else {
            return Task::none();
        };
        self.set_field(edit.page, &edit.field, edit.value)
    }

    /// Opens the text of a note or text box for typing. Returns whether
    /// the annotation has text to edit.
    fn open_text(&mut self, page: usize, annotation: &Annotation) -> bool {
        if !matches!(annotation.kind, Kind::FreeText | Kind::Note) {
            return false;
        }
        self.edit.selected = Some((page, annotation.id.clone()));
        self.edit.text = Some(TextEdit {
            page,
            annotation: annotation.clone(),
            content: text_editor::Content::with_text(&annotation.contents),
            fresh: false,
        });
        true
    }

    fn commit_text(&mut self) -> Task<PdfMessage> {
        let Some(edit) = self.edit.text.take() else {
            return Task::none();
        };
        let text = edit.content.text().trim_end_matches('\n').to_owned();
        let before = edit.annotation;
        if text.trim().is_empty() && (edit.fresh || before.kind == Kind::FreeText) {
            // Empty text boxes and new empty notes go away, as in Preview.
            self.edit.selected = None;
            return self.remove(edit.page, before);
        }
        if text == before.contents {
            return Task::none();
        }
        let mut after = before.clone();
        after.contents = text;
        if after.kind == Kind::FreeText {
            after.rect = fit_text(&after);
        }
        self.change(edit.page, before, after, None, None)
    }

    fn add(
        &mut self,
        page: usize,
        annotation: Annotation,
        content: Option<StampContent>,
        open_text: bool,
    ) -> Task<PdfMessage> {
        self.edit.history.record(Change::Added {
            page,
            annotation: annotation.clone(),
            content: content.clone(),
            removed: None,
        });
        let sent = Sent {
            page,
            stack: Some(Stack::Done),
            select: Some(annotation.id.clone()),
            open_text,
        };
        self.send(page, Edit::Add(annotation, content), sent)
    }

    fn change(
        &mut self,
        page: usize,
        before: Annotation,
        after: Annotation,
        content_before: Option<StampContent>,
        content_after: Option<StampContent>,
    ) -> Task<PdfMessage> {
        if before == after && content_after.is_none() {
            return Task::none();
        }
        self.edit.history.record(Change::Updated {
            page,
            before: Box::new(before),
            after: Box::new(after.clone()),
            content_before,
            content_after: content_after.clone(),
        });
        let sent = Sent {
            select: Some(after.id.clone()),
            ..Sent::plain(page)
        };
        self.send(page, Edit::Update(after, content_after), sent)
    }

    fn remove(&mut self, page: usize, annotation: Annotation) -> Task<PdfMessage> {
        let id = annotation.id.clone();
        self.edit.history.record(Change::Removed {
            page,
            annotation,
            removed: None,
        });
        let sent = Sent {
            stack: Some(Stack::Done),
            ..Sent::plain(page)
        };
        self.send(page, Edit::Remove(id), sent)
    }

    /// Sends an undo or redo step; `stack` is where its change went.
    fn send_step(&mut self, step: Step, stack: Stack) -> Task<PdfMessage> {
        match step {
            Step::Annotation(page, edit) => {
                let sent = Sent {
                    stack: Some(stack),
                    ..Sent::plain(page)
                };
                self.send(page, *edit, sent)
            }
            Step::Pages(edit) => self.undo_pages(edit, stack),
        }
    }

    fn send(&mut self, page: usize, edit: Edit, sent: Sent) -> Task<PdfMessage> {
        let receiver = self.handle.edit(page, edit);
        self.current(receiver, move |result| {
            let result = match result {
                Ok(Ok(edited)) => Ok(edited),
                Ok(Err(error)) => Err(error.to_string()),
                Err(_) => Err("the document closed".into()),
            };
            PdfMessage::Edited(sent.clone(), result)
        })
    }

    pub(super) fn edited(
        &mut self,
        sent: Sent,
        result: Result<Edited, String>,
    ) -> Task<PdfMessage> {
        let edited = match result {
            Ok(edited) => edited,
            Err(error) => {
                self.requests.push(Request::Notice(format!(
                    "Could not change the document: {error}"
                )));
                return Task::none();
            }
        };
        let page = edited.page;
        let display = Arc::clone(&edited.display);
        self.displays.insert(page, Arc::clone(&display));
        *self.generations.entry(page).or_insert(0) += 1;
        self.markup.insert(
            page,
            PageMarkup {
                annotations: edited.annotations,
                fields: edited.fields,
            },
        );
        if let (Some(token), Some(stack)) = (edited.removed, sent.stack) {
            self.edit.history.removed(stack, token);
        }
        if let Some(id) = sent.select {
            self.edit.selected = Some((page, id.clone()));
            if sent.open_text
                && let Some(annotation) = self.annotation(page, &id).cloned()
                && self.open_text(page, &annotation)
                && let Some(edit) = self.edit.text.as_mut()
            {
                edit.fresh = true;
            }
        }
        // Forget a selection whose annotation is gone.
        if let Some((selected_page, id)) = self.edit.selected.clone()
            && self.annotation(selected_page, &id).is_none()
        {
            self.edit.selected = None;
        }
        self.requests.push(Request::Changed);
        let preview = self.refresh_preview(page, &display);
        Task::batch([preview, self.schedule()])
    }

    /// Renders a new thumbnail for a changed page, keeping the old one
    /// until it arrives.
    fn refresh_preview(
        &mut self,
        page: usize,
        display: &Arc<dyn prev_pdf::engine::PageDisplay>,
    ) -> Task<PdfMessage> {
        self.previews_requested.remove(&page);
        let kept = self.previews.remove(&page);
        let task = self.request_preview(page, display, 2);
        if let Some(kept) = kept {
            self.previews.insert(page, kept);
        }
        task
    }

    fn render_loupe(
        &mut self,
        page: usize,
        annotation: Annotation,
        before: Option<Annotation>,
    ) -> Task<PdfMessage> {
        let Some(display) = self.displays.get(&page).cloned() else {
            return Task::none();
        };
        let rect = annotation.rect;
        let center = rect.center();
        let (width, height) = (rect.width() / LOUPE_ZOOM, rect.height() / LOUPE_ZOOM);
        let scale = LOUPE_RESOLUTION * LOUPE_ZOOM;
        let area = PixelRect {
            x: ((center.x - width / 2.0) * scale).round() as i32,
            y: ((center.y - height / 2.0) * scale).round() as i32,
            width: (width * scale).round().max(1.0) as u32,
            height: (height * scale).round().max(1.0) as u32,
        };
        let receiver = self.pool.render(display, scale, area, 2, Ticket::new());
        self.current(receiver, move |result| {
            PdfMessage::Editing(EditMessage::LoupeRendered(Box::new(LoupeRender {
                page,
                annotation: annotation.clone(),
                before: before.clone(),
                image: result.ok().and_then(Result::ok).map(Arc::new),
            })))
        })
    }

    /// Copies the chosen area as a PNG image, through wl-copy, since the
    /// window's clipboard only carries text.
    pub fn copy_area(&mut self) -> Option<Task<PdfMessage>> {
        let (page, rect) = self.edit.area?;
        let display = self.displays.get(&page).cloned()?;
        // Twice the screen's pixels, for a sharp copy.
        let scale = layout::points_to_pixels(self.layout.zoom) * self.device_scale * 2.0;
        let area = PixelRect {
            x: (rect.x0 * scale).round() as i32,
            y: (rect.y0 * scale).round() as i32,
            width: (rect.width() * scale).round().max(1.0) as u32,
            height: (rect.height() * scale).round().max(1.0) as u32,
        };
        let receiver = self.pool.render(display, scale, area, 2, Ticket::new());
        Some(Task::perform(
            async move {
                let bitmap = match receiver.await {
                    Ok(Ok(bitmap)) => bitmap,
                    _ => return Err("could not render the area".to_owned()),
                };
                crate::image::editor::spawn(move || copy_png(&bitmap))
                    .await
                    .unwrap_or_else(|_| Err("copying stopped".into()))
            },
            |result: Result<(), String>| PdfMessage::Editing(EditMessage::Copied(result.err())),
        ))
    }

    pub(super) fn editing(&mut self, message: EditMessage) -> Task<PdfMessage> {
        match message {
            EditMessage::SetTool(tool) => {
                self.edit.drag = None;
                let commit = self.commit_text();
                self.edit.tool = tool;
                if let Tool::Highlight(style) = tool
                    && self.selection.is_some()
                {
                    // Highlighting a selection applies right away.
                    self.edit.tool = Tool::Select;
                    return Task::batch([commit, self.highlight_selection(style)]);
                }
                if tool == Tool::Redact && self.selection.is_some() {
                    self.edit.tool = Tool::Select;
                    return Task::batch([commit, self.redact_selection()]);
                }
                if tool != Tool::Select {
                    self.edit.selected = None;
                }
                if tool != Tool::Area {
                    self.edit.area = None;
                }
                commit
            }
            EditMessage::Style(change) => {
                if change.is_text() {
                    change.apply(&mut self.edit.text_style);
                } else {
                    change.apply(&mut self.edit.style);
                }
                let Some((page, before)) = self
                    .selected_annotation()
                    .map(|(page, annotation)| (page, annotation.clone()))
                else {
                    return Task::none();
                };
                if before.kind == Kind::Stamp {
                    return Task::none();
                }
                let mut after = before.clone();
                change.apply(&mut after.style);
                if after.kind == Kind::FreeText {
                    after.rect = fit_text(&after);
                }
                self.change(page, before, after, None, None)
            }
            EditMessage::MarkupColor(color) => {
                self.edit.markup_color = color;
                let Some((page, before)) = self
                    .selected_annotation()
                    .filter(|(_, annotation)| matches!(annotation.kind, Kind::Markup { .. }))
                    .map(|(page, annotation)| (page, annotation.clone()))
                else {
                    return Task::none();
                };
                let mut after = before.clone();
                after.style.color = Some(color);
                self.change(page, before, after, None, None)
            }
            EditMessage::Delete => {
                let Some((page, annotation)) = self
                    .selected_annotation()
                    .map(|(page, annotation)| (page, annotation.clone()))
                else {
                    return Task::none();
                };
                self.edit.selected = None;
                self.edit.text = None;
                self.remove(page, annotation)
            }
            EditMessage::Deselect => {
                let commit = self.commit_text();
                let field = self.commit_field();
                self.edit.selected = None;
                self.edit.area = None;
                self.edit.choice = None;
                self.edit.drag = None;
                self.edit.tool = Tool::Select;
                Task::batch([commit, field])
            }
            EditMessage::Undo => {
                self.edit.text = None;
                self.edit.field = None;
                self.edit.selected = None;
                match self.edit.history.undo() {
                    Some(step) => self.send_step(step, Stack::Undone),
                    None => Task::none(),
                }
            }
            EditMessage::Redo => {
                self.edit.text = None;
                self.edit.field = None;
                self.edit.selected = None;
                match self.edit.history.redo() {
                    Some(step) => self.send_step(step, Stack::Done),
                    None => Task::none(),
                }
            }
            EditMessage::TextAction(action) => {
                if let Some(edit) = self.edit.text.as_mut() {
                    edit.content.perform(action);
                }
                Task::none()
            }
            EditMessage::CommitText => self.commit_text(),
            EditMessage::FieldInput(value) => {
                if let Some(edit) = self.edit.field.as_mut() {
                    edit.value = value;
                }
                Task::none()
            }
            EditMessage::CommitField => self.commit_field(),
            EditMessage::Choose(value) => {
                let Some((page, field)) = self.edit.choice.take() else {
                    return Task::none();
                };
                self.set_field(page, &field, value)
            }
            EditMessage::CloseChoice => {
                self.edit.choice = None;
                Task::none()
            }
            EditMessage::PlaceStamp { image, subject } => {
                let page = self.current.min(self.page_count().saturating_sub(1));
                let size = self.info.page_sizes[page];
                let width = SIGNATURE_WIDTH.min(size.width * 0.6);
                let height = width * image.height as f32 / image.width.max(1) as f32;
                // In the middle of what is visible of the page.
                let center = self
                    .layout
                    .to_page(
                        page,
                        self.view.x + self.view.width / 2.0,
                        self.view.y + self.view.height / 2.0,
                    )
                    .unwrap_or(Point::new(size.width / 2.0, size.height / 2.0));
                let x = center.x.clamp(width / 2.0, size.width - width / 2.0);
                let y = center.y.clamp(height / 2.0, size.height - height / 2.0);
                let rect = Rect::new(
                    x - width / 2.0,
                    y - height / 2.0,
                    x + width / 2.0,
                    y + height / 2.0,
                );
                let mut annotation = Annotation::new(new_id(), Kind::Stamp, rect);
                annotation.subject = Some(subject);
                self.edit.tool = Tool::Select;
                let content = StampContent::Image {
                    image,
                    round: false,
                    border: None,
                };
                self.add(page, annotation, Some(content), false)
            }
            EditMessage::LoupeRendered(render) => {
                let LoupeRender {
                    page,
                    annotation,
                    before,
                    image,
                } = *render;
                let Some(image) = image else {
                    return Task::none();
                };
                let content = StampContent::Image {
                    image,
                    round: true,
                    border: Some((Rgb::new(0.35, 0.35, 0.35), 2.0)),
                };
                match before {
                    None => self.add(page, annotation, Some(content), false),
                    Some(before) => self.change(page, before, annotation, None, Some(content)),
                }
            }
            EditMessage::Copied(error) => {
                if let Some(error) = error {
                    self.requests
                        .push(Request::Notice(format!("Could not copy the area: {error}")));
                }
                Task::none()
            }
        }
    }
}

impl Editing {
    /// Underlines and strikethroughs read better in a darker color.
    fn markup_color_for_lines(&self) -> Rgb {
        if self.markup_color == HIGHLIGHT_YELLOW {
            RED
        } else {
            self.markup_color
        }
    }
}

/// A redaction mark over `rect`, in the colors other viewers use for
/// marks not yet applied.
fn redaction(rect: Rect) -> Annotation {
    let mut annotation = Annotation::new(new_id(), Kind::Redact, rect);
    annotation.style.color = Some(Rgb::new(0.85, 0.1, 0.1));
    annotation.style.line_width = 1.0;
    annotation.subject = Some("Redact".into());
    annotation
}

fn copy_png(bitmap: &Bitmap) -> Result<(), String> {
    use std::io::Write;
    let image = image::RgbaImage::from_raw(bitmap.width, bitmap.height, bitmap.pixels.clone())
        .ok_or("the area has no pixels")?;
    let mut png = Vec::new();
    image
        .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(|error| error.to_string())?;
    let mut child = std::process::Command::new("wl-copy")
        .args(["--type", "image/png"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|_| "install wl-clipboard to copy images".to_owned())?;
    child
        .stdin
        .take()
        .ok_or("wl-copy has no input")?
        .write_all(&png)
        .map_err(|error| error.to_string())?;
    let status = child.wait().map_err(|error| error.to_string())?;
    status
        .success()
        .then_some(())
        .ok_or_else(|| "wl-copy failed".to_owned())
}

/// A text box rect that fits its text, growing to the right and down.
pub fn fit_text(annotation: &Annotation) -> Rect {
    let size = annotation.style.font_size;
    let lines: Vec<&str> = annotation.contents.lines().collect();
    let count = lines.len().max(1) as f32;
    // Helvetica averages a little over half an em per character.
    let widest = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as f32;
    let width = (widest * size * 0.56 + 10.0).max(annotation.rect.width());
    let height = (count * size * 1.2 + 8.0).max(size * 1.4 + 8.0);
    Rect::new(
        annotation.rect.x0,
        annotation.rect.y0,
        annotation.rect.x0 + width,
        annotation.rect.y0 + height,
    )
}
