//! Tools that edit: a PDF's pages and form fields, images placed on a
//! page, and an image's pixels. Each edit goes through the same path as
//! prev's own, so it is one step of Undo and saves as any edit does;
//! replace_image keeps the file as it was among its versions.

use std::sync::Arc;

use base64::Engine;
use iced::{Task, window};
use prev::control::Error;
use prev::filetype::{self, FileKind};
use prev::image::editor;
use prev::image::window::{self as image_window, Edit, ImageWindow};
use prev::pdf::history::{Insertion, PageChange};
use prev::pdf::viewer::editing::AgentEdit;
use prev_image::decode::Frame;
use prev_image::edit::{ColorAdjust, CropRect, Operation};
use prev_pdf::annotation::{Annotation, FieldKind, Kind as AnnotationKind, StampContent, new_id};
use prev_pdf::engine::Bitmap;
use prev_pdf::geometry::Rect;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

use super::markup::{Route, later_edits};
use super::read::{Shown, answered, failed, invalid, off_thread};
use super::{Answer, Kind, Output, Tool, number, reply, tool};
use crate::app::{Message, Prev};

pub(super) fn tools() -> Vec<Tool> {
    vec![
        tool(
            "rotate_pages",
            "Rotate pages",
            Kind::Edit,
            "Turns PDF pages by 90, 180 or 270 degrees clockwise (-90 for counterclockwise).",
            rotate_pages,
        ),
        tool(
            "delete_pages",
            "Delete pages",
            Kind::Edit,
            "Removes PDF pages; at least one page stays. Undo puts them back.",
            delete_pages,
        ),
        tool(
            "move_pages",
            "Move pages",
            Kind::Edit,
            "Moves PDF pages, in their order, to before another page or to the end.",
            move_pages,
        ),
        tool(
            "insert_blank_page",
            "Insert a blank page",
            Kind::Edit,
            "Adds an empty page, the size of the page before it, after a page or at the start.",
            insert_blank_page,
        ),
        tool(
            "crop_pages",
            "Crop pages",
            Kind::Edit,
            "Crops PDF pages to a box in points; the content outside it is hidden, not removed, \
             and Undo brings it back.",
            crop_pages,
        ),
        tool(
            "fill_field",
            "Fill a form field",
            Kind::Edit,
            "Fills a PDF form field, by its id or name from form_fields: text for a text \
             field, one of the choices for a drop-down or list, or checked true or false for \
             a checkbox or radio button.",
            fill_field,
        ),
        tool(
            "place_image",
            "Place an image",
            Kind::Edit,
            "Places a picture on a PDF page, or on an image's open markup, from an image file \
             or PNG data, such as a picture the agent made. Give a box, or a top-left point and \
             a width; the height keeps the picture's proportions.",
            place_image,
        ),
        tool(
            "crop",
            "Crop the image",
            Kind::Edit,
            "Crops the image a window shows to a box in image pixels.",
            crop,
        ),
        tool(
            "rotate",
            "Rotate or flip the image",
            Kind::Edit,
            "Turns the image a window shows by 90, 180 or 270 degrees clockwise (-90 for \
             counterclockwise), or flips it.",
            rotate,
        ),
        tool(
            "resize",
            "Resize the image",
            Kind::Edit,
            "Scales the image a window shows to a width or height in pixels, keeping its \
             proportions when only one is given.",
            resize,
        ),
        tool(
            "adjust_color",
            "Adjust colour",
            Kind::Edit,
            "Sets the image's colour adjustments, as the Adjust Color sliders do: exposure in \
             stops (-2 to 2), contrast (-1 to 1), saturation (0 grey to 2, 1 unchanged), \
             temperature (-1 cool to 1 warm), tint (-1 green to 1 magenta), sepia (0 to 1), \
             sharpness (0 to 1), and levels black, white (0 to 1) and gamma. Those left out \
             stay as they are; reset starts from none.",
            adjust_color,
        ),
        tool(
            "replace_image",
            "Replace the image",
            Kind::Edit,
            "Replaces the image a window shows with a new one, such as get_image's picture \
             changed on the agent's side, from an image file or PNG data. It is saved in the \
             file's own format, and the file as it was is kept among its versions, which \
             Revert To brings back.",
            replace_image,
        ),
    ]
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RotatePages {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The pages, from 1.
    pages: Vec<usize>,
    /// 90, 180, 270 or -90.
    degrees: i32,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Pages {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The pages, from 1.
    pages: Vec<usize>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct MovePages {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The pages to move, from 1.
    pages: Vec<usize>,
    /// The page, counted before the move, to put them in front of; the end
    /// if left out.
    before: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertBlank {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page to insert after, from 1, or 0 for the start.
    after: usize,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CropPages {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The pages, from 1.
    pages: Vec<usize>,
    /// The box [x0, y0, x1, y1] in points to keep.
    #[serde(rename = "box")]
    area: [f32; 4],
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct FillField {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The field's page, from 1.
    page: usize,
    /// The field's id from form_fields.
    id: Option<i32>,
    /// The field's name, when it is the only one on the page with it.
    name: Option<String>,
    /// The text, or the choice for a drop-down or list.
    value: Option<String>,
    /// For a checkbox or radio button.
    checked: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PlaceImage {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// An image file to place.
    path: Option<String>,
    /// A PNG to place, base64-encoded.
    png_base64: Option<String>,
    /// The box [x0, y0, x1, y1] to fill, in points (image pixels on an
    /// image's markup).
    #[serde(rename = "box")]
    area: Option<[f32; 4]>,
    /// The top-left corner's distance from the page's left edge.
    x: Option<f32>,
    /// The top-left corner's distance from the page's top edge.
    y: Option<f32>,
    /// The width, with the height following the picture's proportions.
    width: Option<f32>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Crop {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The box [x0, y0, x1, y1] in image pixels to keep.
    #[serde(rename = "box")]
    area: [u32; 4],
}

#[derive(Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum Flip {
    Horizontal,
    Vertical,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Rotate {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// 90, 180, 270 or -90.
    degrees: Option<i32>,
    flip: Option<Flip>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Resize {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The new width in pixels.
    width: Option<u32>,
    /// The new height in pixels.
    height: Option<u32>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AdjustColor {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Start from no adjustments rather than the current ones.
    reset: Option<bool>,
    exposure: Option<f32>,
    contrast: Option<f32>,
    saturation: Option<f32>,
    temperature: Option<f32>,
    tint: Option<f32>,
    sepia: Option<f32>,
    sharpness: Option<f32>,
    black: Option<f32>,
    white: Option<f32>,
    gamma: Option<f32>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReplaceImage {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// An image file with the new image.
    path: Option<String>,
    /// The new image as a PNG, base64-encoded.
    png_base64: Option<String>,
}

/// Pages given from 1, as sorted indices without repeats.
fn page_indices(pages: &[usize], count: usize) -> Result<Vec<usize>, Error> {
    if pages.is_empty() {
        return Err(invalid("Give at least one page."));
    }
    if let Some(page) = pages.iter().find(|page| !(1..=count).contains(*page)) {
        return Err(invalid(format!(
            "Page {page} is not in the document, which has {count} pages."
        )));
    }
    let mut indices: Vec<usize> = pages.iter().map(|page| page - 1).collect();
    indices.sort_unstable();
    indices.dedup();
    Ok(indices)
}

/// "1 page" or "3 pages", for the replies agents read.
fn count_pages(count: usize) -> String {
    if count == 1 {
        "1 page".to_owned()
    } else {
        format!("{count} pages")
    }
}

/// Clockwise quarter turns for `degrees`.
fn quarter_turns(degrees: i32) -> Result<i32, Error> {
    match degrees.rem_euclid(360) {
        90 => Ok(1),
        180 => Ok(2),
        270 => Ok(3),
        _ => Err(invalid("Turn by 90, 180, 270 or -90 degrees.")),
    }
}

/// The picture a tool is given, from a file or PNG data. Slow: call it
/// off the interface thread.
fn picture(path: Option<String>, png: Option<String>) -> Result<Frame, Error> {
    match (path, png) {
        (Some(path), None) => {
            let path = prev_store::paths::expand_home(std::path::Path::new(&path));
            let format = match filetype::detect_path(&path) {
                Ok(Some(FileKind::Image(format))) => format,
                Ok(_) => {
                    return Err(invalid(format!(
                        "{} is not an image prev can read.",
                        path.display()
                    )));
                }
                Err(error) => return Err(invalid(format!("{}: {error}", path.display()))),
            };
            let decoded = prev_image::decode::decode_file(&path, format)
                .map_err(|error| failed(format!("{error:?}")))?;
            decoded
                .frames
                .into_iter()
                .next()
                .ok_or_else(|| failed("the image has no frames"))
        }
        (None, Some(png)) => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(png.trim())
                .map_err(|error| invalid(format!("The PNG data is not base64: {error}")))?;
            let image = image::load_from_memory(&bytes)
                .map_err(|error| invalid(format!("The PNG data is not an image: {error}")))?
                .into_rgba8();
            Ok(Frame {
                width: image.width(),
                height: image.height(),
                pixels: image.into_raw(),
                delay: std::time::Duration::ZERO,
            })
        }
        _ => Err(invalid("Give either path or png_base64.")),
    }
}

impl Prev {
    /// Makes page `change` in the PDF window `window` shows.
    fn page_edit(
        &mut self,
        window: Option<u64>,
        tool: &str,
        answer: &Answer,
        edit: impl FnOnce(&prev::pdf::viewer::PdfViewer) -> Result<(AgentEdit, String), Error>,
    ) -> Task<Message> {
        let made = self.tool_window(window).and_then(|id| {
            let viewer = self.shown_pdf(window, tool)?;
            Ok((id, edit(viewer)?))
        });
        match made {
            Ok((id, (edit, done))) => {
                self.agent_markup(id, Route::Pdf, answer, vec![edit], Output::Text(done))
            }
            Err(error) => reply(answer, Err(error)),
        }
    }

    /// The image window `window` shows, for an edit of its pixels.
    fn image_target(
        &self,
        window: Option<u64>,
        tool: &str,
    ) -> Result<(window::Id, &ImageWindow), Error> {
        match self.shown(window)? {
            (id, Shown::Image(images)) => {
                if images.agent_markup().is_some() {
                    return Err(invalid(
                        "The image's markup is open; edits to its pixels wait until the markup \
                         is exported or closed.",
                    ));
                }
                if images.shown_image().is_none() {
                    return Err(invalid("The image is still loading."));
                }
                Ok((id, images))
            }
            (id, shown) => Err(invalid(format!(
                "Window {} shows {}; {tool} is for images.",
                number(id),
                shown.what()
            ))),
        }
    }

    /// Applies `operation` to the image window `window` shows.
    fn image_edit(
        &mut self,
        window: Option<u64>,
        tool: &str,
        answer: &Answer,
        operation: impl FnOnce(&ImageWindow, (u32, u32)) -> Result<Operation, Error>,
    ) -> Task<Message> {
        let made = self.image_target(window, tool).and_then(|(id, images)| {
            let size = images
                .edited_size()
                .ok_or_else(|| invalid("The image is still loading."))?;
            let operation = operation(images, size)?;
            Ok((id, operation, operation.output_size(size)))
        });
        match made {
            Ok((id, operation, (width, height))) => {
                answer.send(Ok(Output::Json(json!({
                    "window": number(id),
                    "width": width,
                    "height": height,
                }))));
                self.update(Message::Image(
                    id,
                    image_window::Message::Edit(Edit::Apply(operation)),
                ))
            }
            Err(error) => reply(answer, Err(error)),
        }
    }
}

fn rotate_pages(app: &mut Prev, input: RotatePages, answer: &Answer) -> Task<Message> {
    app.page_edit(input.window, "rotate_pages", answer, |viewer| {
        let pages = page_indices(&input.pages, viewer.page_count())?;
        let quarter_turns = quarter_turns(input.degrees)?;
        let done = format!("Turned {}.", count_pages(pages.len()));
        Ok((
            AgentEdit::Pages(PageChange::Rotated {
                pages,
                quarter_turns,
            }),
            done,
        ))
    })
}

fn delete_pages(app: &mut Prev, input: Pages, answer: &Answer) -> Task<Message> {
    app.page_edit(input.window, "delete_pages", answer, |viewer| {
        let count = viewer.page_count();
        let pages = page_indices(&input.pages, count)?;
        if pages.len() >= count {
            return Err(invalid("A document keeps at least one page."));
        }
        let done = format!(
            "Deleted {}; the document has {} now.",
            count_pages(pages.len()),
            count_pages(count - pages.len())
        );
        Ok((
            AgentEdit::Pages(PageChange::Removed {
                pages,
                removed: None,
            }),
            done,
        ))
    })
}

fn move_pages(app: &mut Prev, input: MovePages, answer: &Answer) -> Task<Message> {
    app.page_edit(input.window, "move_pages", answer, |viewer| {
        let count = viewer.page_count();
        let moving = page_indices(&input.pages, count)?;
        let to = match input.before {
            None => count,
            Some(page) if (1..=count).contains(&page) => page - 1,
            Some(page) => {
                return Err(invalid(format!(
                    "Page {page} is not in the document, which has {count} pages."
                )));
            }
        };
        let done = match input.before {
            Some(page) => format!("Moved {} to before page {page}.", count_pages(moving.len())),
            None => format!("Moved {} to the end.", count_pages(moving.len())),
        };
        Ok((AgentEdit::MovePages { moving, to }, done))
    })
}

fn insert_blank_page(app: &mut Prev, input: InsertBlank, answer: &Answer) -> Task<Message> {
    app.page_edit(input.window, "insert_blank_page", answer, |viewer| {
        let count = viewer.page_count();
        if input.after > count {
            return Err(invalid(format!(
                "Page {} is not in the document, which has {count} pages.",
                input.after
            )));
        }
        let size = viewer.info.page_sizes[input.after.saturating_sub(1)];
        Ok((
            AgentEdit::Pages(PageChange::Inserted {
                at: input.after,
                count: 0,
                source: Insertion::Blank(size),
                removed: None,
            }),
            format!("Inserted a blank page as page {}.", input.after + 1),
        ))
    })
}

fn crop_pages(app: &mut Prev, input: CropPages, answer: &Answer) -> Task<Message> {
    app.page_edit(input.window, "crop_pages", answer, |viewer| {
        let pages = page_indices(&input.pages, viewer.page_count())?;
        let rect = super::markup::area(input.area);
        if rect.width() < 1.0 || rect.height() < 1.0 {
            return Err(invalid("The box is empty."));
        }
        let done = format!("Cropped {}.", count_pages(pages.len()));
        Ok((
            AgentEdit::Pages(PageChange::Cropped {
                pages,
                rect,
                before: None,
            }),
            done,
        ))
    })
}

fn fill_field(app: &mut Prev, input: FillField, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "fill_field") {
        Ok(target) if matches!(target.route, Route::Pdf) => target,
        Ok(_) => return reply(answer, Err(invalid("Images have no form fields."))),
        Err(error) => return reply(answer, Err(error)),
    };
    let (id, page, handle) = (target.id, target.page, target.viewer.handle.clone());
    let FillField {
        id: field_id,
        name,
        value,
        checked,
        ..
    } = input;
    later_edits(id, Route::Pdf, answer, async move {
        let fields = answered(handle.markup(page)).await?.fields;
        let field = match (field_id, &name) {
            (Some(wanted), _) => fields.into_iter().find(|field| field.id == wanted),
            (None, Some(name)) => {
                let mut named: Vec<_> = fields
                    .into_iter()
                    .filter(|field| &field.name == name)
                    .collect();
                if named.len() > 1 {
                    return Err(invalid(format!(
                        "Page {} has {} fields named {name}; give the id.",
                        page + 1,
                        named.len()
                    )));
                }
                named.pop()
            }
            (None, None) => return Err(invalid("Give the field's id or name.")),
        }
        .ok_or_else(|| {
            invalid(format!(
                "Page {} has no such field; see form_fields.",
                page + 1
            ))
        })?;
        if field.read_only {
            return Err(invalid(format!("{} is read-only.", field.name)));
        }
        let new_value = match (&field.kind, value, checked) {
            (FieldKind::Text { .. }, Some(text), None) => text,
            (FieldKind::Choice { options, .. }, Some(choice), None) => {
                if !options.contains(&choice) {
                    return Err(invalid(format!(
                        "{choice} is not one of {}'s choices: {}.",
                        field.name,
                        options.join(", ")
                    )));
                }
                choice
            }
            (FieldKind::Checkbox | FieldKind::Radio, None, Some(true)) => {
                field.on_value.clone().unwrap_or_else(|| "Yes".to_owned())
            }
            (FieldKind::Checkbox, None, Some(false)) => "Off".to_owned(),
            (FieldKind::Radio, None, Some(false)) => {
                return Err(invalid(
                    "A radio button turns off when another of its group is checked.",
                ));
            }
            (FieldKind::Signature | FieldKind::Button, ..) => {
                return Err(invalid(format!(
                    "{} is a signature or button field, which fill_field does not fill.",
                    field.name
                )));
            }
            (FieldKind::Checkbox | FieldKind::Radio, ..) => {
                return Err(invalid("Give checked true or false for this field."));
            }
            _ => return Err(invalid("Give value for this field.")),
        };
        let output = Output::Json(json!({
            "id": field.id,
            "name": field.name,
            "value": new_value,
        }));
        Ok((
            vec![AgentEdit::Field {
                page,
                field: Box::new(field),
                value: new_value,
            }],
            output,
        ))
    })
}

fn place_image(app: &mut Prev, input: PlaceImage, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "place_image") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let place = target.place();
    let page_size = target.viewer.info.page_sizes[target.page];
    let unit = target.unit;
    let given = match (input.area, input.x, input.y, input.width) {
        (Some(area), None, None, None) => Ok(Placement::Box(target.area(area))),
        (None, Some(x), Some(y), width) => Ok(Placement::At(
            target.point(x, y),
            width.map(|width| width * unit),
        )),
        _ => Err(invalid("Give a box, or x and y with an optional width.")),
    };
    let placement = match given {
        Ok(placement) => placement,
        Err(error) => return reply(answer, Err(error)),
    };
    let (path, png) = (input.path, input.png_base64);
    later_edits(place.id(), place.route(), answer, async move {
        let frame = off_thread(move || picture(path, png)).await??;
        let aspect = frame.height as f32 / frame.width.max(1) as f32;
        let rect = match placement {
            Placement::Box(rect) => rect,
            Placement::At(at, width) => {
                // Pixels at 96 dpi, made smaller to fit the page.
                let natural = frame.width as f32 * 0.75;
                let width = width.unwrap_or(natural.min(page_size.width * 0.8));
                Rect::new(at.x, at.y, at.x + width, at.y + width * aspect)
            }
        };
        let mut annotation = Annotation::new(new_id(), AnnotationKind::Stamp, rect);
        annotation.subject = Some("Image".into());
        let content = StampContent::Image {
            image: Arc::new(Bitmap {
                width: frame.width,
                height: frame.height,
                pixels: frame.pixels,
            }),
            round: false,
            border: None,
        };
        let output = Output::Json(json!({ "ids": [annotation.id], "page": place.page() + 1 }));
        Ok((
            vec![AgentEdit::AddStamp {
                page: place.page(),
                annotation,
                content,
            }],
            output,
        ))
    })
}

/// Where place_image puts a picture.
enum Placement {
    Box(Rect),
    /// A top-left corner and maybe a width.
    At(prev_pdf::geometry::Point, Option<f32>),
}

fn crop(app: &mut Prev, input: Crop, answer: &Answer) -> Task<Message> {
    app.image_edit(input.window, "crop", answer, |_, (width, height)| {
        let [x0, y0, x1, y1] = input.area;
        let (left, right) = (x0.min(x1).min(width), x0.max(x1).min(width));
        let (top, bottom) = (y0.min(y1).min(height), y0.max(y1).min(height));
        if right <= left || bottom <= top {
            return Err(invalid(format!(
                "The box is outside the {width} by {height} image, or empty."
            )));
        }
        Ok(Operation::Crop(CropRect {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        }))
    })
}

fn rotate(app: &mut Prev, input: Rotate, answer: &Answer) -> Task<Message> {
    app.image_edit(input.window, "rotate", answer, |_, _| {
        match (input.degrees, input.flip) {
            (Some(degrees), None) => Ok(Operation::Rotate(quarter_turns(degrees)? as u8)),
            (None, Some(Flip::Horizontal)) => Ok(Operation::FlipHorizontal),
            (None, Some(Flip::Vertical)) => Ok(Operation::FlipVertical),
            _ => Err(invalid("Give either degrees or flip.")),
        }
    })
}

fn resize(app: &mut Prev, input: Resize, answer: &Answer) -> Task<Message> {
    app.image_edit(input.window, "resize", answer, |_, size| {
        if input.width.is_none() && input.height.is_none() {
            return Err(invalid("Give a width, a height or both."));
        }
        let (width, height) = editor::proportional(size, input.width, input.height);
        if width == 0 || height == 0 || width > 65_535 || height > 65_535 {
            return Err(invalid("Sizes go from 1 to 65535 pixels."));
        }
        Ok(Operation::Resize { width, height })
    })
}

fn adjust_color(app: &mut Prev, input: AdjustColor, answer: &Answer) -> Task<Message> {
    app.image_edit(input.window, "adjust_color", answer, |images, _| {
        let mut adjust = if input.reset == Some(true) {
            ColorAdjust::default()
        } else {
            images.color()
        };
        let set = |slot: &mut f32, value: Option<f32>, low: f32, high: f32| {
            if let Some(value) = value {
                *slot = value.clamp(low, high);
            }
        };
        set(&mut adjust.exposure, input.exposure, -2.0, 2.0);
        set(&mut adjust.contrast, input.contrast, -1.0, 1.0);
        set(&mut adjust.saturation, input.saturation, 0.0, 2.0);
        set(&mut adjust.temperature, input.temperature, -1.0, 1.0);
        set(&mut adjust.tint, input.tint, -1.0, 1.0);
        set(&mut adjust.sepia, input.sepia, 0.0, 1.0);
        set(&mut adjust.sharpness, input.sharpness, 0.0, 1.0);
        set(&mut adjust.black, input.black, 0.0, 1.0);
        set(&mut adjust.white, input.white, 0.0, 1.0);
        set(&mut adjust.gamma, input.gamma, 0.1, 10.0);
        Ok(Operation::Color(adjust))
    })
}

fn replace_image(app: &mut Prev, input: ReplaceImage, answer: &Answer) -> Task<Message> {
    let target = app
        .image_target(input.window, "replace_image")
        .and_then(|(id, images)| {
            let format = images.save_format().ok_or_else(|| {
                invalid("prev cannot save this image's format in place; export it instead.")
            })?;
            Ok((id, images.current_path().to_path_buf(), format))
        });
    let (id, path, format) = match target {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let (source, png) = (input.path, input.png_base64);
    let answer = answer.clone();
    Task::perform(
        async move {
            let frame = off_thread(move || picture(source, png)).await??;
            let (width, height) = (frame.width, frame.height);
            off_thread(move || editor::save_in_place(&path, &frame, format, true))
                .await?
                .map_err(failed)?;
            Ok::<_, Error>((width, height))
        },
        move |result| Message::ToolReplaced(id, answer.clone(), result),
    )
}

impl Prev {
    /// replace_image saved the file: the window shows it anew.
    pub(in crate::app) fn image_replaced(
        &mut self,
        id: window::Id,
        answer: &Answer,
        result: Result<(u32, u32), Error>,
    ) -> Task<Message> {
        match result {
            Ok((width, height)) => {
                answer.send(Ok(Output::Json(json!({
                    "window": number(id),
                    "width": width,
                    "height": height,
                    "version_kept": true,
                }))));
                self.update(Message::Image(id, image_window::Message::Reverted(Ok(()))))
            }
            Err(error) => reply(answer, Err(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_count_from_one_and_stay_in_range() {
        assert_eq!(page_indices(&[3, 1, 3], 4).unwrap(), vec![0, 2]);
        assert!(page_indices(&[0], 4).is_err());
        assert!(page_indices(&[5], 4).is_err());
        assert!(page_indices(&[], 4).is_err());
    }

    #[test]
    fn turns_come_from_degrees() {
        assert_eq!(quarter_turns(90).unwrap(), 1);
        assert_eq!(quarter_turns(-90).unwrap(), 3);
        assert_eq!(quarter_turns(540).unwrap(), 2);
        assert!(quarter_turns(45).is_err());
    }

    #[test]
    fn pictures_come_from_png_data() {
        let image = image::RgbaImage::from_pixel(3, 2, image::Rgba([10, 20, 30, 255]));
        let mut png = Vec::new();
        image
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        let data = base64::engine::general_purpose::STANDARD.encode(png);
        let frame = picture(None, Some(data)).unwrap();
        assert_eq!((frame.width, frame.height), (3, 2));
        assert_eq!(&frame.pixels[..4], &[10, 20, 30, 255]);
        assert!(picture(None, Some("not base64!".into())).is_err());
        assert!(picture(None, None).is_err());
    }
}
