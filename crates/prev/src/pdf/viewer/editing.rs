//! Markup and form editing in the viewer: tools turn presses and drags into
//! annotations, the select tool moves and resizes them, and every change
//! goes to the document thread and into the undo history.

use std::sync::Arc;

use iced::Task;
use iced::advanced::image::Allocation;
use iced::widget::image::Handle as ImageHandle;
use iced::widget::text_editor;
use prev_pdf::annotation::{
    Align, Annotation, Field, FieldKind, Font, Kind, Rgb, StampContent, Style, TextMarkup, new_id,
};
use prev_pdf::engine::{Bitmap, PageDisplay};
use prev_pdf::geometry::{PixelRect, Point, Quad, Rect};
use prev_pdf::worker::{Edit, Edited, PageMarkup, Ticket};

use super::{PdfMessage, PdfViewer, Request};
use crate::i18n::Describe;
use crate::pdf::history::{Change, Entry, History, Step};
use crate::pdf::layout;
use crate::pdf::markup::{self, Handle, Shape, Tool};

/// Preview's default markup colors.
pub const RED: Rgb = Rgb::new(0.93, 0.16, 0.14);
pub const HIGHLIGHT_YELLOW: Rgb = Rgb::new(1.0, 0.86, 0.2);
/// A loupe magnifies its area this many times.
const LOUPE_ZOOM: f32 = 2.0;
/// Device pixels per point in a loupe's image.
const LOUPE_RESOLUTION: f32 = 3.0;
/// Logical pixels past the view that drafts of an annotation still cover.
const DRAFT_SLACK: f32 = 96.0;
/// A loupe's border, and its width in points.
const LOUPE_BORDER: Rgb = Rgb::new(0.35, 0.35, 0.35);
const LOUPE_BORDER_WIDTH: f32 = 2.0;
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
    /// A press on selected text or a chosen area, which becomes a drag to
    /// other windows and apps once the pointer moves.
    Out { start: (f32, f32), out: Outgoing },
}

/// What a drag out of the document carries.
#[derive(Debug, Clone, PartialEq)]
pub enum Outgoing {
    Text(String),
    /// An area of a page, as an image.
    Area(usize, Rect),
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
    /// The annotation being moved, drawn from its own images so the page
    /// underneath does not keep showing it where it was.
    pub lift: Option<Lift>,
    /// The images of the annotation last clicked without moving it, kept
    /// so dragging it next starts with them ready.
    lift_kept: Option<Lift>,
}

/// Images for moving one annotation: the page without it over where it
/// was, and the annotation alone, which follows the pointer.
#[derive(Debug, Clone)]
pub struct Lift {
    pub page: usize,
    pub id: String,
    /// The page's generation when the move began; the lift ends once the
    /// page has been drawn anew after the change.
    generation: u32,
    /// The area the images cover, in page points: the annotation's rect
    /// with room for its line, out to whole device pixels as rendered.
    pub area: Rect,
    /// The annotation's rect when the move began. Once the change is
    /// saved, the page's annotations already have the new one.
    original: Rect,
    /// Device pixels per point the images were rendered at.
    scale: f32,
    pub images: Option<(ImageHandle, ImageHandle)>,
    /// Keeps the images on the GPU while they are shown.
    uploaded: Option<(Allocation, Allocation)>,
    /// Where the annotation went, once let go.
    pub placed: Option<Rect>,
    /// The annotation as its handles reshape it, rendered anew, since its
    /// image stretched would thicken its lines and squash its text.
    draft: Option<Draft>,
    /// A render of the reshaped annotation is under way; the next waits
    /// for it, so the renders keep up with the pointer without piling up.
    drafting: bool,
    /// Whether this lift has asked for a draft yet: the first starts from
    /// the annotation as the document has it.
    drafted: bool,
    /// The annotation as it was let go after a reshape, drawn until the
    /// page shows it.
    reshaped: Option<Annotation>,
    /// The page without the annotation, which a loupe magnifies.
    source: Option<Source>,
}

/// A page to render, kept with the lift.
#[derive(Clone)]
pub struct Source(Arc<dyn PageDisplay>);

impl std::fmt::Debug for Source {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Source")
    }
}

/// A render of an annotation being reshaped.
#[derive(Debug, Clone)]
pub struct Draft {
    /// The annotation as rendered.
    shape: Annotation,
    /// The area the image covers, in page points.
    area: Rect,
    image: ImageHandle,
    /// Keeps the image on the GPU while it is shown.
    _uploaded: Allocation,
}

/// A draft, rendered and on its way to the GPU.
#[derive(Debug, Clone)]
pub struct DraftRender {
    shape: Annotation,
    area: Rect,
    image: Option<ImageHandle>,
}

/// Room around an annotation's rect for its line and antialiasing, in
/// page points.
fn lift_margin(annotation: &Annotation) -> f32 {
    annotation.style.line_width + 3.0
}

/// Where an image of an annotation with `rect`, covering `area`, goes for
/// the annotation to have `to`: each side keeps its room around the
/// annotation, scaled with it.
fn stretched(rect: Rect, area: Rect, to: Rect) -> Rect {
    let scale_x = to.width() / rect.width().max(0.01);
    let scale_y = to.height() / rect.height().max(0.01);
    Rect::new(
        to.x0 - (rect.x0 - area.x0) * scale_x,
        to.y0 - (rect.y0 - area.y0) * scale_y,
        to.x1 + (area.x1 - rect.x1) * scale_x,
        to.y1 + (area.y1 - rect.y1) * scale_y,
    )
}

fn is_loupe(annotation: &Annotation) -> bool {
    annotation.kind == Kind::Stamp && annotation.subject.as_deref() == Some("Loupe")
}

/// Whether dragging `handle` draws the annotation anew, rather than moving
/// its image: a reshape, or a loupe, which shows what is under it.
fn redrawn(annotation: &Annotation, handle: Handle) -> bool {
    handle != Handle::Body || is_loupe(annotation)
}

/// Cuts `bitmap` to the ellipse filling it, with a border `width` pixels
/// wide just inside its edge, as a round stamp with a border draws it.
fn round_with_border(bitmap: &mut Bitmap, width: f32, color: Rgb) {
    let (w, h) = (bitmap.width as f32, bitmap.height as f32);
    let (rx, ry) = (w / 2.0, h / 2.0);
    // The distance of a pixel's middle outside the ellipse with radii
    // `rx`, `ry`, in pixels, roughly: negative inside.
    let outside = |x: f32, y: f32, rx: f32, ry: f32| {
        let (dx, dy) = ((x - w / 2.0) / rx.max(0.5), (y - h / 2.0) / ry.max(0.5));
        ((dx * dx + dy * dy).sqrt() - 1.0) * rx.min(ry)
    };
    let ink = [color.red * 255.0, color.green * 255.0, color.blue * 255.0];
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let (cx, cy) = (x as f32 + 0.5, y as f32 + 0.5);
            let shape = (0.5 - outside(cx, cy, rx, ry)).clamp(0.0, 1.0);
            let inner = (0.5 - outside(cx, cy, rx - width, ry - width)).clamp(0.0, 1.0);
            let ring = (shape - inner).max(0.0);
            let index = ((y * bitmap.width + x) * 4) as usize;
            let pixel = &mut bitmap.pixels[index..index + 4];
            for (channel, ink) in pixel[..3].iter_mut().zip(ink) {
                let blended = if shape > 0.0 {
                    (*channel as f32 * (shape - ring) + ink * ring) / shape
                } else {
                    *channel as f32
                };
                *channel = blended.round().clamp(0.0, 255.0) as u8;
            }
            pixel[3] = (pixel[3] as f32 * shape).round() as u8;
        }
    }
}

/// Room around a reshaped annotation for what its rect leaves out: a
/// line's rect runs between its ends, without the arrow heads.
fn draft_margin(annotation: &Annotation) -> f32 {
    match annotation.kind {
        Kind::Line { .. } => annotation.style.line_width * 8.0 + 8.0,
        _ => lift_margin(annotation),
    }
}

impl Editing {
    /// Drops images kept for a drag, when page numbers change.
    pub(super) fn forget_kept_lift(&mut self) {
        self.lift_kept = None;
    }
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
            lift: None,
            lift_kept: None,
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
    /// Pastes an image at its own size where it fits: centered on a point
    /// in document space, or in the middle of the view.
    PasteImage(Arc<Bitmap>, Option<(f32, f32)>),
    /// Pastes text as a text box, placed as images are.
    PasteText(String, Option<(f32, f32)>),
    LoupeRendered(Box<LoupeRender>),
    /// An area was copied, or could not be.
    Copied(Option<String>),
    /// The images of an annotation being moved: the page without it, and
    /// the annotation alone.
    Lifted(String, Option<(ImageHandle, ImageHandle, Source)>),
    /// Those images, now on the GPU, so the first frame that shows them
    /// draws them both.
    LiftUploaded(String, Option<(Allocation, Allocation)>),
    /// A render of an annotation being reshaped.
    Drafted(Box<DraftRender>),
    /// That render, now on the GPU.
    DraftUploaded(Box<DraftRender>, Option<Allocation>),
    /// A change an agent asks for, made as the markup tools make it, so
    /// it is one step of Undo.
    Agent(Box<AgentEdit>),
}

/// A change to an annotation that an agent asks for.
#[derive(Debug, Clone)]
pub enum AgentEdit {
    Add {
        page: usize,
        annotation: Annotation,
    },
    Change {
        page: usize,
        before: Box<Annotation>,
        after: Box<Annotation>,
    },
    Remove {
        page: usize,
        annotation: Annotation,
    },
    /// An image or drawing over `annotation`'s rect, such as a placed
    /// picture or signature.
    AddStamp {
        page: usize,
        annotation: Annotation,
        content: StampContent,
    },
    Field {
        page: usize,
        field: Box<Field>,
        value: String,
    },
    Pages(crate::pdf::history::PageChange),
    /// Moves pages to the gap before page `to`.
    MovePages {
        moving: Vec<usize>,
        to: usize,
    },
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
    entry: Option<Entry>,
    select: Option<String>,
    open_text: bool,
}

impl Sent {
    fn plain(page: usize) -> Self {
        Self {
            page,
            entry: None,
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

    /// Whether the annotation being pressed on has moved far enough to
    /// count as a move rather than a click.
    fn moving(&self) -> bool {
        match &self.edit.drag {
            Some(Drag::Move { from, current, .. }) => {
                (current.x - from.x).hypot(current.y - from.y) >= CLICK_DISTANCE / 2.0
            }
            _ => false,
        }
    }

    /// The annotation being created or moved, as it would look now. A
    /// press that has not moved yet shows nothing new.
    pub fn preview(&self) -> Option<(usize, Annotation)> {
        if matches!(self.edit.drag, Some(Drag::Move { .. })) && !self.moving() {
            return None;
        }
        // A lifted annotation moves once its images are ready, so the
        // handles and the annotation go together.
        if let Some(Drag::Move { original, .. }) = &self.edit.drag
            && self
                .edit
                .lift
                .as_ref()
                .is_some_and(|lift| lift.id == original.id && lift.images.is_none())
        {
            return None;
        }
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
            Drag::Create { .. } | Drag::Out { .. } => None,
            Drag::Move {
                page,
                original,
                handle,
                from,
                current,
            } => Some((
                *page,
                self.dragged(*page, original, *handle, *from, *current),
            )),
        }
    }

    /// `annotation` dragged by `handle` from `from` to `current`, kept on
    /// its page: a move stops at the page's edges, and a handle goes no
    /// farther than them.
    fn dragged(
        &self,
        page: usize,
        annotation: &Annotation,
        handle: Handle,
        from: Point,
        current: Point,
    ) -> Annotation {
        let current = match self.info.page_sizes.get(page) {
            Some(size) => {
                // Bounds that cannot hold the annotation leave it be.
                let between = |value: f32, low: f32, high: f32| {
                    if low <= high {
                        value.clamp(low, high)
                    } else {
                        value
                    }
                };
                match handle {
                    Handle::Body => {
                        let rect = annotation.rect;
                        Point::new(
                            from.x + between(current.x - from.x, -rect.x0, size.width - rect.x1),
                            from.y + between(current.y - from.y, -rect.y0, size.height - rect.y1),
                        )
                    }
                    _ => Point::new(
                        current.x.clamp(0.0, size.width),
                        current.y.clamp(0.0, size.height),
                    ),
                }
            }
            None => current,
        };
        markup::dragged(annotation, handle, from, current, self.shift)
    }

    /// The page of the annotation being dragged.
    fn drag_page(&self) -> usize {
        match &self.edit.drag {
            Some(Drag::Move { page, .. }) => *page,
            _ => 0,
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
                tasks.push(self.start_move(page, annotation, handle, point));
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
                // A note moves when dragged, and opens when clicked, once
                // it is let go without moving.
                let opens = clicks >= 2 && annotation.kind == Kind::FreeText;
                if opens {
                    self.open_text(page, &annotation);
                } else {
                    tasks.push(self.start_move(page, annotation, Handle::Body, point));
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

    fn start_move(
        &mut self,
        page: usize,
        annotation: Annotation,
        handle: Handle,
        point: Point,
    ) -> Task<PdfMessage> {
        let allowed = match handle {
            Handle::Body => markup::movable(&annotation),
            Handle::Edge { .. } => markup::resizable(&annotation),
            Handle::LineStart | Handle::LineEnd => true,
        };
        if !allowed {
            return Task::none();
        }
        // Masks cover the page.
        let lifts = annotation.kind.is_editable() && !markup::is_mask(&annotation);
        let task = if lifts {
            self.lift(page, &annotation)
        } else {
            Task::none()
        };
        self.edit.drag = Some(Drag::Move {
            page,
            original: Box::new(annotation),
            handle,
            from: point,
            current: point,
        });
        task
    }

    /// Renders the page without `annotation`, and `annotation` alone, over
    /// its area at the current zoom.
    fn lift(&mut self, page: usize, annotation: &Annotation) -> Task<PdfMessage> {
        // The same annotation, unchanged, at the same zoom: its images are
        // still good.
        if let Some(kept) = self.edit.lift_kept.take()
            && kept.page == page
            && kept.id == annotation.id
            && kept.original == annotation.rect
            && kept.generation == self.generation(page)
            && kept.scale == self.render_scale()
        {
            self.edit.lift = Some(kept);
            return Task::none();
        }
        let margin = lift_margin(annotation);
        let rect = annotation.rect;
        let area = Rect::new(
            rect.x0 - margin,
            rect.y0 - margin,
            rect.x1 + margin,
            rect.y1 + margin,
        );
        self.edit.lift = Some(Lift {
            page,
            id: annotation.id.clone(),
            generation: self.generation(page),
            area,
            original: rect,
            scale: self.render_scale(),
            images: None,
            uploaded: None,
            placed: None,
            draft: None,
            drafting: false,
            drafted: false,
            reshaped: None,
            source: None,
        });
        let scale = self.render_scale();
        let pixels = PixelRect {
            x: (area.x0 * scale).floor() as i32,
            y: (area.y0 * scale).floor() as i32,
            width: (area.width() * scale).ceil().max(1.0) as u32 + 1,
            height: (area.height() * scale).ceil().max(1.0) as u32 + 1,
        };
        // Drawn exactly over the pixels it was rendered from, so the page
        // and the lifted annotation line up when the page takes over.
        let area = Rect::new(
            pixels.x as f32 / scale,
            pixels.y as f32 / scale,
            (pixels.x + pixels.width as i32) as f32 / scale,
            (pixels.y + pixels.height as i32) as f32 / scale,
        );
        if let Some(lift) = self.edit.lift.as_mut() {
            lift.area = area;
        }
        let receiver = self.handle.lift(page, annotation.id.clone());
        let pool = std::sync::Arc::clone(&self.pool);
        let id = annotation.id.clone();
        self.current(
            async move {
                let lifted = receiver.await.ok()?.ok()?;
                let without =
                    pool.render(Arc::clone(&lifted.without), scale, pixels, 1, Ticket::new());
                let alone = pool.render(lifted.alone, scale, pixels, 1, Ticket::new());
                let (without, alone) = (without.await.ok()?.ok()?, alone.await.ok()?.ok()?);
                let handle = |bitmap: Bitmap| {
                    ImageHandle::from_rgba(bitmap.width, bitmap.height, bitmap.pixels)
                };
                Some((handle(without), handle(alone), Source(lifted.without)))
            },
            move |images| PdfMessage::Editing(EditMessage::Lifted(id, images)),
        )
    }

    /// The lift's images and where to draw them on `page`: the page without
    /// the annotation over its old place, and the annotation where the
    /// pointer has it.
    pub fn lift_images(&self, page: usize) -> Option<(&ImageHandle, Rect, &ImageHandle, Rect)> {
        let lift = self.edit.lift.as_ref().filter(|lift| lift.page == page)?;
        // A press is not a move until the pointer goes somewhere.
        if lift.placed.is_none() && !self.moving() {
            return None;
        }
        let (without, alone) = lift.images.as_ref()?;
        let moved = match (&lift.placed, self.preview()) {
            (Some(placed), _) => *placed,
            (None, Some((_, moved))) if moved.id == lift.id => moved.rect,
            _ => self.annotation(page, &lift.id)?.rect,
        };
        let line = self
            .annotation(page, &lift.id)
            .is_some_and(|annotation| matches!(annotation.kind, Kind::Line { .. }));
        if self.reshaping() || lift.reshaped.is_some() {
            if let Some(draft) = &lift.draft {
                // A line shows where it was rendered: stretched between
                // other ends, its arrow heads would bend.
                let target = if line {
                    draft.area
                } else {
                    stretched(draft.shape.rect, draft.area, moved)
                };
                return Some((without, lift.area, &draft.image, target));
            }
            // As it was, until its first draft is ready.
            if line {
                return Some((without, lift.area, alone, lift.area));
            }
        }
        Some((
            without,
            lift.area,
            alone,
            stretched(lift.original, lift.area, moved),
        ))
    }

    /// Whether a handle that reshapes the selected annotation is being
    /// dragged, rather than the annotation moved whole.
    fn reshaping(&self) -> bool {
        matches!(
            &self.edit.drag,
            Some(Drag::Move { handle, original, .. }) if redrawn(original, *handle)
        ) && self.moving()
    }

    /// Whether a drag that draws, makes or moves an annotation is under
    /// way, which scrolls the view at its edge.
    pub(super) fn edge_scrolls(&self) -> bool {
        match &self.edit.drag {
            Some(Drag::Stroke { .. } | Drag::Create { .. }) => true,
            Some(Drag::Move { .. }) => self.moving(),
            Some(Drag::Out { .. }) | None => false,
        }
    }

    /// The handle being dragged, so the pointer keeps its shape all the
    /// way, wherever it goes.
    pub fn dragged_handle(&self) -> Option<Handle> {
        match &self.edit.drag {
            Some(Drag::Move { handle, .. }) => Some(*handle),
            _ => None,
        }
    }

    /// The part of `page` in view, in page points, with `slack` logical
    /// pixels more on each side.
    fn visible_on_page(&self, page: usize, slack: f32) -> Option<Rect> {
        let view = &self.view;
        let start = self.layout.to_page(page, view.x - slack, view.y - slack)?;
        let end = self.layout.to_page(
            page,
            view.x + view.width + slack,
            view.y + view.height + slack,
        )?;
        Some(Rect::new(start.x, start.y, end.x, end.y))
    }

    /// Renders a loupe as it is now, from the page without it: what is
    /// under its middle, magnified, cut round with its border, as its stamp
    /// will show it. Made here, at the screen's resolution, rather than as
    /// a stamp, which would add an image to the document at every step.
    fn draft_loupe(
        &self,
        shaped: Annotation,
        source: Arc<dyn PageDisplay>,
        scale: f32,
    ) -> Task<PdfMessage> {
        let rect = shaped.rect;
        let center = rect.center();
        let magnified = scale * LOUPE_ZOOM;
        let (width, height) = (rect.width() / LOUPE_ZOOM, rect.height() / LOUPE_ZOOM);
        let pixels = PixelRect {
            x: ((center.x - width / 2.0) * magnified).round() as i32,
            y: ((center.y - height / 2.0) * magnified).round() as i32,
            width: (width * magnified).round().max(1.0) as u32,
            height: (height * magnified).round().max(1.0) as u32,
        };
        // The rendered area starts where rounding to whole pixels put it;
        // the image moves by as much, magnified, so what it shows sits
        // where the loupe's stamp will put it.
        let (left, top) = (
            rect.x0 + (pixels.x as f32 / magnified - (center.x - width / 2.0)) * LOUPE_ZOOM,
            rect.y0 + (pixels.y as f32 / magnified - (center.y - height / 2.0)) * LOUPE_ZOOM,
        );
        let area = Rect::new(
            left,
            top,
            left + pixels.width as f32 / scale,
            top + pixels.height as f32 / scale,
        );
        let border = LOUPE_BORDER_WIDTH * scale;
        let receiver = self
            .pool
            .render(source, magnified, pixels, 1, Ticket::new());
        self.current(
            async move {
                let mut bitmap = receiver.await.ok()?.ok()?;
                round_with_border(&mut bitmap, border, LOUPE_BORDER);
                Some(ImageHandle::from_rgba(
                    bitmap.width,
                    bitmap.height,
                    bitmap.pixels,
                ))
            },
            move |image| {
                PdfMessage::Editing(EditMessage::Drafted(Box::new(DraftRender {
                    shape: shaped,
                    area,
                    image,
                })))
            },
        )
    }

    /// Asks for the annotation being reshaped to be rendered as it is now,
    /// unless a render is under way or the last one already shows it.
    pub(super) fn request_draft(&mut self) -> Task<PdfMessage> {
        let shaped = match &self.edit.drag {
            Some(Drag::Move {
                original,
                handle,
                from,
                current,
                ..
            }) if redrawn(original, *handle) => {
                if !self.moving() {
                    return Task::none();
                }
                self.dragged(self.drag_page(), original, *handle, *from, *current)
            }
            _ => match self
                .edit
                .lift
                .as_ref()
                .and_then(|lift| lift.reshaped.clone())
            {
                Some(reshaped) => reshaped,
                None => return Task::none(),
            },
        };
        let scale = self.render_scale();
        let visible = self
            .edit
            .lift
            .as_ref()
            .and_then(|lift| self.visible_on_page(lift.page, DRAFT_SLACK));
        let Some(lift) = self.edit.lift.as_mut().filter(|lift| lift.id == shaped.id) else {
            return Task::none();
        };
        if lift.drafting
            || lift
                .draft
                .as_ref()
                .is_some_and(|draft| draft.shape == shaped)
        {
            return Task::none();
        }
        if is_loupe(&shaped) {
            let Some(Source(source)) = lift.source.clone() else {
                return Task::none();
            };
            lift.drafting = true;
            return self.draft_loupe(shaped, source, scale);
        }
        lift.drafting = true;
        let fresh = !lift.drafted;
        lift.drafted = true;
        let page = lift.page;
        let margin = draft_margin(&shaped);
        let rect = shaped.rect;
        let mut wanted = Rect::new(
            rect.x0 - margin,
            rect.y0 - margin,
            rect.x1 + margin,
            rect.y1 + margin,
        );
        // Only what can be seen, and a little more for the pointer to go
        // on into: a large annotation zoomed in would otherwise render
        // and upload far more than the screen at every step.
        if let Some(visible) = visible {
            wanted = Rect::new(
                wanted.x0.max(visible.x0),
                wanted.y0.max(visible.y0),
                wanted.x1.min(visible.x1),
                wanted.y1.min(visible.y1),
            );
            if wanted.width() <= 0.0 || wanted.height() <= 0.0 {
                lift.drafting = false;
                lift.drafted = !fresh;
                return Task::none();
            }
        }
        let pixels = PixelRect {
            x: (wanted.x0 * scale).floor() as i32,
            y: (wanted.y0 * scale).floor() as i32,
            width: (wanted.width() * scale).ceil().max(1.0) as u32 + 1,
            height: (wanted.height() * scale).ceil().max(1.0) as u32 + 1,
        };
        let area = Rect::new(
            pixels.x as f32 / scale,
            pixels.y as f32 / scale,
            (pixels.x + pixels.width as i32) as f32 / scale,
            (pixels.y + pixels.height as i32) as f32 / scale,
        );
        let receiver = self.handle.draft(page, shaped.clone(), fresh);
        let pool = std::sync::Arc::clone(&self.pool);
        self.current(
            async move {
                let display = receiver.await.ok()?.ok()?;
                let bitmap = pool
                    .render(display, scale, pixels, 1, Ticket::new())
                    .await
                    .ok()?
                    .ok()?;
                Some(ImageHandle::from_rgba(
                    bitmap.width,
                    bitmap.height,
                    bitmap.pixels,
                ))
            },
            move |image| {
                PdfMessage::Editing(EditMessage::Drafted(Box::new(DraftRender {
                    shape: shaped,
                    area,
                    image,
                })))
            },
        )
    }

    /// Ends the lift once the page shows the annotation in its new place.
    pub(super) fn settle_lift(&mut self) {
        let Some(lift) = &self.edit.lift else {
            return;
        };
        if lift.placed.is_none() {
            return;
        }
        let page = lift.page;
        if self.generation(page) > lift.generation && self.page_tiles_ready(page) {
            self.edit.lift = None;
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
            Drag::Out { start, out } => {
                // Past a small move, the drag leaves for other windows.
                if (x - start.0).abs() + (y - start.1).abs() > 4.0 {
                    let out = out.clone();
                    self.edit.drag = None;
                    self.press = None;
                    self.requests.push(Request::DragOut(out));
                }
                return true;
            }
        };
        let point = self.layout.to_page(page, x, y).unwrap_or_default();
        match drag {
            Drag::Stroke { points, .. } => points.push(point),
            Drag::Create { current, .. } | Drag::Move { current, .. } => *current = point,
            Drag::Out { .. } => {}
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
            // A click on the selection: it goes, as any click clears it.
            Drag::Out {
                out: Outgoing::Text(_),
                ..
            } => {
                self.press = None;
                self.selection = None;
                Task::none()
            }
            Drag::Out { .. } => Task::none(),
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
                let moved = self.dragged(page, &original, handle, from, current);
                let unmoved = (current.x - from.x).hypot(current.y - from.y) < CLICK_DISTANCE / 2.0;
                match self.edit.lift.as_mut() {
                    // Keep showing it where it went until the page is drawn
                    // anew.
                    Some(lift) if !unmoved && lift.id == original.id => {
                        lift.placed = Some(moved.rect);
                        if redrawn(&original, handle) {
                            lift.reshaped = Some(moved.clone());
                        }
                    }
                    // A click: keep the images for a drag that may follow.
                    Some(lift) if unmoved && lift.id == original.id => {
                        self.edit.lift_kept = self.edit.lift.take();
                    }
                    _ => self.edit.lift = None,
                }
                // The selection handles show the new place at once, not the
                // old one until the document thread answers.
                if !unmoved
                    && let Some(annotation) = self.markup.get_mut(&page).and_then(|markup| {
                        markup
                            .annotations
                            .iter_mut()
                            .find(|annotation| annotation.id == moved.id)
                    })
                {
                    *annotation = moved.clone();
                }
                // Its final shape, should the pointer have moved on since
                // the last draft.
                let draft = self.request_draft();
                if unmoved {
                    if original.kind == Kind::Note && self.edit.text.is_none() {
                        self.open_text(page, &original);
                    }
                    Task::none()
                } else if is_loupe(&original) {
                    // From the page without the loupe, which the page
                    // shown still has where it was.
                    let source = self
                        .edit
                        .lift
                        .as_ref()
                        .and_then(|lift| lift.source.clone())
                        .map(|Source(source)| source);
                    Task::batch([
                        draft,
                        self.render_loupe(page, moved, Some(*original), source),
                    ])
                } else {
                    Task::batch([draft, self.change(page, *original, moved, None, None)])
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
                    Shape::Loupe => self.render_loupe(page, annotation, None, None),
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

    /// The page something pasted at `at`, in document space, goes on: the
    /// one nearest it, or the current page.
    fn paste_page(&self, at: Option<(f32, f32)>) -> usize {
        at.and_then(|(x, y)| self.layout.hit_nearest(x, y))
            .map_or(self.current, |(page, _)| page)
            .min(self.page_count().saturating_sub(1))
    }

    /// A `width` by `height` rect on `page` centered on `at`, in document
    /// space, or in the middle of what is visible, kept on the page.
    fn placed(&self, page: usize, width: f32, height: f32, at: Option<(f32, f32)>) -> Rect {
        let size = self.info.page_sizes[page];
        let (x, y) = at.unwrap_or((
            self.view.x + self.view.width / 2.0,
            self.view.y + self.view.height / 2.0,
        ));
        let center = self
            .layout
            .to_page(page, x, y)
            .unwrap_or(Point::new(size.width / 2.0, size.height / 2.0));
        let x = center
            .x
            .clamp(width / 2.0, (size.width - width / 2.0).max(width / 2.0));
        let y = center
            .y
            .clamp(height / 2.0, (size.height - height / 2.0).max(height / 2.0));
        Rect::new(
            x - width / 2.0,
            y - height / 2.0,
            x + width / 2.0,
            y + height / 2.0,
        )
    }

    fn add(
        &mut self,
        page: usize,
        annotation: Annotation,
        content: Option<StampContent>,
        open_text: bool,
    ) -> Task<PdfMessage> {
        let entry = self.edit.history.record(Change::Added {
            page,
            annotation: annotation.clone(),
            content: content.clone(),
            removed: None,
        });
        let sent = Sent {
            page,
            entry: Some(entry),
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
        let entry = self.edit.history.record(Change::Removed {
            page,
            annotation,
            removed: None,
        });
        let sent = Sent {
            entry: Some(entry),
            ..Sent::plain(page)
        };
        self.send(page, Edit::Remove(id), sent)
    }

    /// Sends an undo or redo step for the change `entry`.
    fn send_step(&mut self, step: Step, entry: Entry) -> Task<PdfMessage> {
        match step {
            Step::Annotation(page, edit) => {
                let sent = Sent {
                    entry: Some(entry),
                    ..Sent::plain(page)
                };
                self.send(page, *edit, sent)
            }
            Step::Pages(edit) => self.undo_pages(edit, entry),
        }
    }

    fn send(&mut self, page: usize, edit: Edit, sent: Sent) -> Task<PdfMessage> {
        let receiver = self.handle.edit(page, edit);
        self.current(receiver, move |result| {
            let result = match result {
                Ok(Ok(edited)) => Ok(edited),
                Ok(Err(error)) => Err(error.describe()),
                Err(_) => Err(crate::fl!("markup-document-closed")),
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
                self.requests.push(Request::Notice(crate::fl!(
                    "markup-change-failed",
                    error = error
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
        if let (Some(token), Some(entry)) = (edited.removed, sent.entry) {
            self.edit.history.removed(entry, token);
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
        source: Option<Arc<dyn PageDisplay>>,
    ) -> Task<PdfMessage> {
        let Some(display) = source.or_else(|| self.displays.get(&page).cloned()) else {
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
    /// What a press at `x`, `y` in document space would drag out: the
    /// selected text or the chosen area under it.
    pub(super) fn drag_out_at(&self, x: f32, y: f32) -> Option<Outgoing> {
        let (page, point) = self.layout.hit(x, y)?;
        if self.edit.tool == Tool::Area {
            let (area_page, rect) = self.edit.area?;
            return (area_page == page && rect.contains(point))
                .then_some(Outgoing::Area(page, rect));
        }
        let selection = self.page_selection(page)?;
        let text = self.texts.get(&page)?;
        text.highlight(selection)
            .iter()
            .any(|rect| rect.contains(point))
            .then(|| self.selected_text())
            .flatten()
            .map(Outgoing::Text)
    }

    /// The image annotation being moved by its body, with its page, as it
    /// was before the move.
    pub fn moving_image(&self) -> Option<(usize, &Annotation)> {
        match &self.edit.drag {
            Some(Drag::Move {
                page,
                original,
                handle: Handle::Body,
                ..
            }) if original.kind == Kind::Stamp
                && original.subject.as_deref() != Some("Loupe")
                && !markup::is_mask(original) =>
            {
                Some((*page, original))
            }
            _ => None,
        }
    }

    /// Ends a move without changing the annotation, which stays where it
    /// was.
    pub fn cancel_move(&mut self) {
        if matches!(self.edit.drag, Some(Drag::Move { .. })) {
            self.edit.drag = None;
            self.edit.lift = None;
            self.press = None;
        }
    }

    /// Removes the annotation `id` from `page`, as Delete would.
    pub fn remove_annotation(&mut self, page: usize, id: &str) -> Task<PdfMessage> {
        let Some(annotation) = self.markup.get(&page).and_then(|markup| {
            markup
                .annotations
                .iter()
                .find(|annotation| annotation.id == id)
                .cloned()
        }) else {
            return Task::none();
        };
        self.remove(page, annotation)
    }

    /// Renders `annotation` on `page` alone, on transparency, at about
    /// print resolution and at most 4096 pixels on a side.
    pub fn render_annotation(
        &self,
        page: usize,
        annotation: &Annotation,
    ) -> impl std::future::Future<Output = Result<Bitmap, String>> + use<> {
        let rect = annotation.rect;
        let longest = rect.width().max(rect.height()).max(1.0);
        let scale = (300.0 / 72.0_f32)
            .max(self.render_scale() * 2.0)
            .min(4096.0 / longest);
        let pixels = PixelRect {
            x: (rect.x0 * scale).round() as i32,
            y: (rect.y0 * scale).round() as i32,
            width: (rect.width() * scale).round().max(1.0) as u32,
            height: (rect.height() * scale).round().max(1.0) as u32,
        };
        let receiver = self.handle.lift(page, annotation.id.clone());
        let pool = std::sync::Arc::clone(&self.pool);
        async move {
            let failed = || crate::fl!("markup-render-area-failed");
            let lifted = receiver
                .await
                .ok()
                .and_then(Result::ok)
                .ok_or_else(failed)?;
            match pool
                .render(lifted.alone, scale, pixels, 1, Ticket::new())
                .await
            {
                Ok(Ok(bitmap)) => Ok(bitmap),
                _ => Err(failed()),
            }
        }
    }

    /// Renders `rect` of `page` at twice the screen's pixels.
    pub fn render_area(
        &self,
        page: usize,
        rect: Rect,
    ) -> Option<impl std::future::Future<Output = Result<Bitmap, String>> + use<>> {
        let display = self.displays.get(&page).cloned()?;
        let scale = layout::points_to_pixels(self.layout.zoom) * self.device_scale * 2.0;
        let area = PixelRect {
            x: (rect.x0 * scale).round() as i32,
            y: (rect.y0 * scale).round() as i32,
            width: (rect.width() * scale).round().max(1.0) as u32,
            height: (rect.height() * scale).round().max(1.0) as u32,
        };
        let receiver = self.pool.render(display, scale, area, 2, Ticket::new());
        Some(async move {
            match receiver.await {
                Ok(Ok(bitmap)) => Ok(bitmap),
                _ => Err(crate::fl!("markup-render-area-failed")),
            }
        })
    }

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
                    _ => return Err(crate::fl!("markup-render-area-failed")),
                };
                crate::image::editor::spawn(move || copy_png(&bitmap))
                    .await
                    .unwrap_or_else(|_| Err(crate::fl!("markup-copy-stopped")))
            },
            |result: Result<(), String>| PdfMessage::Editing(EditMessage::Copied(result.err())),
        ))
    }

    pub(super) fn editing(&mut self, message: EditMessage) -> Task<PdfMessage> {
        match message {
            EditMessage::Agent(edit) => match *edit {
                AgentEdit::Add { page, annotation } => self.add(page, annotation, None, false),
                AgentEdit::Change {
                    page,
                    before,
                    after,
                } => self.change(page, *before, *after, None, None),
                AgentEdit::Remove { page, annotation } => self.remove(page, annotation),
                AgentEdit::AddStamp {
                    page,
                    annotation,
                    content,
                } => self.add(page, annotation, Some(content), false),
                AgentEdit::Field { page, field, value } => self.set_field(page, &field, value),
                AgentEdit::Pages(change) => self.change_pages(change),
                AgentEdit::MovePages { moving, to } => self.move_pages(&moving, to),
            },
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
                    Some((step, entry)) => self.send_step(step, entry),
                    None => Task::none(),
                }
            }
            EditMessage::Redo => {
                self.edit.text = None;
                self.edit.field = None;
                self.edit.selected = None;
                match self.edit.history.redo() {
                    Some((step, entry)) => self.send_step(step, entry),
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
                let rect = self.placed(page, width, height, None);
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
            EditMessage::PasteImage(image, at) => {
                let page = self.paste_page(at);
                let size = self.info.page_sizes[page];
                // Pixels at 96 dpi, made smaller to fit the page.
                let (width, height) = (image.width as f32 * 0.75, image.height as f32 * 0.75);
                let fit = (size.width * 0.8 / width.max(1.0))
                    .min(size.height * 0.8 / height.max(1.0))
                    .min(1.0);
                let rect = self.placed(page, width * fit, height * fit, at);
                let mut annotation = Annotation::new(new_id(), Kind::Stamp, rect);
                annotation.subject = Some("Image".into());
                self.edit.tool = Tool::Select;
                let content = StampContent::Image {
                    image,
                    round: false,
                    border: None,
                };
                self.add(page, annotation, Some(content), false)
            }
            EditMessage::PasteText(text, at) => {
                let page = self.paste_page(at);
                let text = text.replace("\r\n", "\n");
                let mut annotation = Annotation::new(new_id(), Kind::FreeText, Rect::default());
                annotation.style = self.edit.text_style;
                annotation.subject = Some("Text Box".into());
                annotation.contents = text.trim_end().to_owned();
                let fitted = fit_text(&annotation);
                let size = self.info.page_sizes[page];
                annotation.rect = self.placed(
                    page,
                    fitted.width().min(size.width * 0.9),
                    fitted.height().min(size.height * 0.9),
                    at,
                );
                self.edit.tool = Tool::Select;
                self.add(page, annotation, None, false)
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
                    border: Some((LOUPE_BORDER, LOUPE_BORDER_WIDTH)),
                };
                match before {
                    None => self.add(page, annotation, Some(content), false),
                    Some(before) => self.change(page, before, annotation, None, Some(content)),
                }
            }
            EditMessage::Lifted(id, images) => {
                let Some((without, alone, source)) = images else {
                    return self.editing(EditMessage::LiftUploaded(id, None));
                };
                for lift in [&mut self.edit.lift, &mut self.edit.lift_kept]
                    .into_iter()
                    .flatten()
                    .filter(|lift| lift.id == id)
                {
                    lift.source = Some(source.clone());
                }
                // Both go to the GPU before either is shown: a large image
                // uploaded while drawing would miss its first frame, and the
                // page without the annotation would show on its own.
                let upload =
                    |handle: ImageHandle| iced_runtime::image::allocate(handle).map(Result::ok);
                upload(without).then(move |without| {
                    let id = id.clone();
                    upload(alone.clone()).map(move |alone| {
                        PdfMessage::Editing(EditMessage::LiftUploaded(
                            id.clone(),
                            without.clone().zip(alone),
                        ))
                    })
                })
            }
            EditMessage::LiftUploaded(id, uploaded) => {
                // They may arrive after the click that asked for them.
                for slot in [&mut self.edit.lift, &mut self.edit.lift_kept] {
                    match slot.as_mut() {
                        Some(lift) if lift.id == id && lift.images.is_none() => match uploaded {
                            Some((without, alone)) => {
                                lift.images =
                                    Some((without.handle().clone(), alone.handle().clone()));
                                lift.uploaded = Some((without, alone));
                                break;
                            }
                            None => *slot = None,
                        },
                        _ => {}
                    }
                }
                Task::none()
            }
            EditMessage::Drafted(render) => {
                let Some(image) = render.image.clone() else {
                    return self.editing(EditMessage::DraftUploaded(render, None));
                };
                // On the GPU before it replaces the last one, so no frame
                // goes without.
                iced_runtime::image::allocate(image).map(move |uploaded| {
                    PdfMessage::Editing(EditMessage::DraftUploaded(render.clone(), uploaded.ok()))
                })
            }
            EditMessage::DraftUploaded(render, uploaded) => {
                let Some(lift) = self
                    .edit
                    .lift
                    .as_mut()
                    .filter(|lift| lift.id == render.shape.id)
                else {
                    return Task::none();
                };
                lift.drafting = false;
                if let Some(uploaded) = uploaded {
                    let render = *render;
                    lift.draft = Some(Draft {
                        image: uploaded.handle().clone(),
                        _uploaded: uploaded,
                        shape: render.shape,
                        area: render.area,
                    });
                }
                // The pointer may have gone on while this one rendered.
                self.request_draft()
            }
            EditMessage::Copied(error) => {
                if let Some(error) = error {
                    self.requests.push(Request::Notice(crate::fl!(
                        "markup-copy-area-failed",
                        error = error
                    )));
                }
                Task::none()
            }
        }
    }
}

impl Editing {
    /// Underlines and strikethroughs read better in a darker color.
    pub fn markup_color_for_lines(&self) -> Rgb {
        if self.markup_color == HIGHLIGHT_YELLOW {
            RED
        } else {
            self.markup_color
        }
    }
}

/// A redaction mark over `rect`, in the colors other viewers use for
/// marks not yet applied.
pub fn redaction(rect: Rect) -> Annotation {
    let mut annotation = Annotation::new(new_id(), Kind::Redact, rect);
    annotation.style.color = Some(Rgb::new(0.85, 0.1, 0.1));
    annotation.style.line_width = 1.0;
    annotation.subject = Some("Redact".into());
    annotation
}

fn copy_png(bitmap: &Bitmap) -> Result<(), String> {
    crate::paste::copy_image(bitmap)
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
