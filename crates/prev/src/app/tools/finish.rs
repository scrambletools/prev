//! Tools whose results Undo cannot take back, each behind its own approval
//! switch: placing a signature, applying redactions, and exporting.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::{Task, window};
use prev::control::Error;
use prev::image::editor;
use prev::pdf::export::{self as pdf_export, Format};
use prev::pdf::viewer::editing::AgentEdit;
use prev::pdf::window as pdf_window;
use prev_image::decode::Frame;
use prev_image::encode::SaveFormat;
use prev_pdf::annotation::{Annotation, Kind as AnnotationKind, StampContent, new_id};
use prev_pdf::engine::{ExportOptions, PageDisplay, Reduce};
use prev_pdf::geometry::Rect;
use prev_pdf::worker::flatten;
use prev_store::signatures::SignatureStore;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

use super::markup::later_edits;
use super::read::{Shown, answered, display, failed, invalid, off_thread};
use super::{Answer, Kind, On, Output, Tool, number, reply, tool};
use crate::app::{Message, Prev};

/// How wide a signature goes on a page when no width is given, in points.
const SIGNATURE_WIDTH: f32 = 180.0;

pub(super) fn tools() -> Vec<Tool> {
    vec![
        tool(
            "place_signature",
            "Sign",
            Kind::Sign,
            "Places one of the user's saved signatures, by its name from list_signatures, on \
             a PDF page or an image's markup: in a box, or at a top-left point with a width. \
             Add the date with add_text_box.",
            place_signature,
        ),
        tool(
            "apply_redactions",
            "Apply redactions",
            Kind::Redact,
            "Removes for good the text, pictures and drawing under every redaction mark in a \
             PDF, and the file's earlier versions with them. Undo cannot bring any of it back.",
            apply_redactions,
        ),
        tool(
            "export",
            "Export",
            Kind::Export,
            "Writes what a window shows to a new file. A PDF exports as a PDF, optionally only \
             some pages, flattened so its markup and fields can no longer be changed, made \
             smaller, or with a password; or as PNG, JPEG, TIFF, WebP or OpenEXR pictures of \
             its pages, one file per page except TIFF. An image exports with its edits and \
             markup as PNG, JPEG, WebP, TIFF, BMP, TGA, QOI, PPM or OpenEXR, at its own size or \
             scaled, which leaves the open image as it is. The format \
             follows the file name's extension. An existing file is replaced only with \
             overwrite.",
            export,
        ),
    ]
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PlaceSignature {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The page, from 1.
    page: usize,
    /// The signature's name from list_signatures.
    signature: String,
    /// The box [x0, y0, x1, y1] to fit it in, keeping its proportions.
    #[serde(rename = "box")]
    area: Option<[f32; 4]>,
    /// The top-left corner's distance from the page's left edge.
    x: Option<f32>,
    /// The top-left corner's distance from the page's top edge.
    y: Option<f32>,
    /// The width; 180 points if left out.
    width: Option<f32>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Export {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The file to write, absolute or with ~ for the home folder. Its
    /// extension picks the format.
    path: String,
    /// Replace the file if it exists.
    overwrite: Option<bool>,
    /// PDF only: the pages to export, from 1, in this order; all if left
    /// out.
    pages: Option<Vec<usize>>,
    /// PDF to PDF only: draw the markup and form fields into the pages.
    flatten: Option<bool>,
    /// PDF to PDF only: scale down and recompress pictures for a smaller
    /// file.
    reduce: Option<bool>,
    /// PDF to PDF only: a password needed to open the file.
    password: Option<String>,
    /// Pictures of PDF pages: pixels per inch, 72 to 600; 150 if left out.
    dpi: Option<f32>,
    /// JPEG quality, 1 to 100; 92 if left out.
    quality: Option<u8>,
    /// Images: the size to export at, as a share of the image's own, such
    /// as 0.5 for half; SVG drawings: pixels per point of the drawing. 1
    /// if left out. The open image keeps its size.
    scale: Option<f32>,
}

/// The rect for a picture `width` by `height` pixels: fitted inside
/// `area`, or at `at` with `width` and the picture's proportions.
fn fitted(area: Option<Rect>, at: Option<(f32, f32, f32)>, width: u32, height: u32) -> Rect {
    let aspect = height as f32 / width.max(1) as f32;
    match (area, at) {
        (Some(area), _) => {
            let fit_width = area.width().min(area.height() / aspect.max(0.0001));
            let fit_height = fit_width * aspect;
            // Centred in the box.
            let x = area.x0 + (area.width() - fit_width) / 2.0;
            let y = area.y0 + (area.height() - fit_height) / 2.0;
            Rect::new(x, y, x + fit_width, y + fit_height)
        }
        (None, Some((x, y, wide))) => Rect::new(x, y, x + wide, y + wide * aspect),
        (None, None) => Rect::default(),
    }
}

fn place_signature(app: &mut Prev, input: PlaceSignature, answer: &Answer) -> Task<Message> {
    let target = match app.markup_target(input.window, input.page, "place_signature") {
        Ok(target) => target,
        Err(error) => return reply(answer, Err(error)),
    };
    let place = target.place();
    let unit = target.unit;
    let area = input.area.map(|area| target.area(area));
    let at = match (input.x, input.y) {
        (Some(x), Some(y)) => {
            let point = target.point(x, y);
            Some((
                point.x,
                point.y,
                input.width.map_or(SIGNATURE_WIDTH, |width| width * unit),
            ))
        }
        (None, None) => None,
        _ => return reply(answer, Err(invalid("Give both x and y."))),
    };
    if area.is_some() == at.is_some() {
        return reply(
            answer,
            Err(invalid("Give a box, or x and y with an optional width.")),
        );
    }
    let store = SignatureStore::new(app.settings.signatures.clone());
    let wanted = input.signature;
    later_edits(place.id(), place.route(), answer, async move {
        let bitmap = off_thread(move || {
            let signatures = store.list();
            let names = || {
                signatures
                    .iter()
                    .map(|signature| signature.description.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let signature = signatures
                .iter()
                .find(|signature| signature.description.eq_ignore_ascii_case(wanted.trim()))
                .or_else(|| signatures.iter().find(|signature| signature.file == wanted))
                .ok_or_else(|| {
                    invalid(if signatures.is_empty() {
                        "The signature library is empty; the user adds signatures from the \
                         markup bar's Sign menu."
                            .to_owned()
                    } else {
                        format!("There is no signature {wanted}; there are: {}.", names())
                    })
                })?;
            let png = store.read(signature).map_err(failed)?;
            prev::pdf::signature::decode(&png).ok_or_else(|| failed("the signature is unreadable"))
        })
        .await??;
        let rect = fitted(area, at, bitmap.width, bitmap.height);
        let mut annotation = Annotation::new(new_id(), AnnotationKind::Stamp, rect);
        annotation.subject = Some("Signature".into());
        let content = StampContent::Image {
            image: Arc::new(bitmap),
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

fn apply_redactions(app: &mut Prev, input: On, answer: &Answer) -> Task<Message> {
    let id = match app.tool_window(input.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let handle = match app.shown_pdf(input.window, "apply_redactions") {
        Ok(viewer) => viewer.handle.clone(),
        Err(error) => return reply(answer, Err(error)),
    };
    let answer = answer.clone();
    Task::perform(
        async move {
            let pages = answered(handle.all_annotations()).await?;
            let marks = pages
                .iter()
                .flat_map(|(_, annotations)| annotations)
                .filter(|annotation| annotation.kind == AnnotationKind::Redact)
                .count();
            if marks == 0 {
                return Err(invalid(
                    "The PDF has no redaction marks; add them with add_redaction.",
                ));
            }
            Ok(marks)
        },
        move |result| Message::ToolRedact(id, answer.clone(), result),
    )
}

impl Prev {
    /// apply_redactions counted the marks: they are applied now.
    pub(in crate::app) fn redact(
        &mut self,
        id: window::Id,
        answer: &Answer,
        marks: Result<usize, Error>,
    ) -> Task<Message> {
        let marks = match marks {
            Ok(marks) => marks,
            Err(error) => return reply(answer, Err(error)),
        };
        if !self.windows.contains_key(&id) {
            return reply(
                answer,
                Err(invalid(format!("Window {} closed.", number(id)))),
            );
        }
        answer.send(Ok(Output::Json(json!({
            "window": number(id),
            "applied": marks,
        }))));
        self.update(Message::Pdf(id, pdf_window::Message::ApplyRedactions))
    }

    /// An image window finished an export that an agent asked for.
    pub(in crate::app) fn image_exported(
        &mut self,
        id: window::Id,
        result: &Result<PathBuf, String>,
    ) {
        let (answers, kept) = std::mem::take(&mut self.agents.exports)
            .into_iter()
            .partition(|(window, _)| *window == id);
        self.agents.exports = kept;
        for (_, answer) in answers {
            answer.send(match result {
                Ok(path) => Ok(Output::Json(
                    json!({ "files": [path.display().to_string()] }),
                )),
                Err(error) => Err(failed(error)),
            });
        }
    }
}

/// What an export's extension asks for.
fn extension(path: &Path) -> String {
    path.extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
}

fn export(app: &mut Prev, input: Export, answer: &Answer) -> Task<Message> {
    let target = prev_store::paths::expand_home(Path::new(&input.path));
    if !target.is_absolute() {
        return reply(answer, Err(invalid("Give the file's absolute path.")));
    }
    if target.exists() && input.overwrite != Some(true) {
        return reply(
            answer,
            Err(invalid(format!(
                "{} exists; give overwrite true to replace it.",
                target.display()
            ))),
        );
    }
    if target.parent().is_some_and(|folder| !folder.is_dir()) {
        return reply(
            answer,
            Err(invalid(format!(
                "The folder for {} does not exist.",
                target.display()
            ))),
        );
    }
    let (id, shown) = match app.shown(input.window) {
        Ok(shown) => shown,
        Err(error) => return reply(answer, Err(error)),
    };
    if app.shown_path(id).is_some_and(|open| {
        prev_store::paths::canonical(&open) == prev_store::paths::canonical(&target)
    }) {
        return reply(
            answer,
            Err(invalid("That is the open file; export to a new one.")),
        );
    }
    let quality = input.quality.unwrap_or(92).clamp(1, 100);
    match shown {
        Shown::Pdf(viewer) => {
            let count = viewer.page_count();
            let pages = match &input.pages {
                Some(pages) => match pages
                    .iter()
                    .map(|page| {
                        (1..=count).contains(page).then(|| page - 1).ok_or_else(|| {
                            invalid(format!(
                                "Page {page} is not in the document, which has {count} pages."
                            ))
                        })
                    })
                    .collect::<Result<Vec<usize>, Error>>()
                {
                    Ok(pages) => Some(pages),
                    Err(error) => return reply(answer, Err(error)),
                },
                None => None,
            };
            let format = match extension(&target).as_str() {
                "pdf" => Format::Pdf,
                "png" => Format::Png,
                "jpg" | "jpeg" => Format::Jpeg,
                "tif" | "tiff" => Format::Tiff,
                "webp" => Format::WebP,
                "exr" => Format::OpenExr,
                other => {
                    return reply(
                        answer,
                        Err(invalid(format!(
                            "A PDF exports as .pdf, .png, .jpg, .tiff, .webp or .exr, not .{other}."
                        ))),
                    );
                }
            };
            let handle = viewer.handle.clone();
            let answer = answer.clone();
            if format == Format::Pdf {
                let options = ExportOptions {
                    password: input.password.filter(|password| !password.is_empty()),
                    reduce: (input.reduce == Some(true)).then_some(Reduce::DEFAULT),
                    pages,
                    flatten: input.flatten == Some(true),
                };
                let destination = target.clone();
                let receiver = handle.export(
                    options,
                    Box::new(move |bytes| {
                        prev_store::atomic::write(&destination, bytes)
                            .map_err(|error| error.to_string())
                    }),
                );
                return answer.clone().later(Task::perform(
                    async move {
                        flatten(receiver.await).map_err(failed)?;
                        Ok(Output::Json(
                            json!({ "files": [target.display().to_string()] }),
                        ))
                    },
                    |result| result,
                ));
            }
            let pages = pages.unwrap_or_else(|| (0..count).collect());
            let dpi = input.dpi.unwrap_or(150.0).clamp(72.0, 600.0);
            answer.clone().later(Task::perform(
                async move {
                    let mut displays: Vec<Arc<dyn PageDisplay>> = Vec::new();
                    for page in pages {
                        displays.push(display(&handle, page).await?);
                    }
                    let written = off_thread(move || {
                        pdf_export::write_images(&target, &displays, format, dpi, quality)
                    })
                    .await?
                    .map_err(failed)?;
                    let files: Vec<String> = written
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect();
                    Ok(Output::Json(json!({ "files": files })))
                },
                |result| result,
            ))
        }
        Shown::Image(images) => {
            let scaled = input
                .scale
                .filter(|scale| (*scale - 1.0).abs() > f32::EPSILON);
            let raster = images
                .shown_image()
                .filter(|image| !matches!(image.pixels, prev::image::window::Pixels::Svg(_)));
            let markup = images.agent_markup().and_then(|(_, viewer, scale)| {
                viewer.map(|viewer| (viewer.handle.clone(), viewer.info.page_sizes[0], scale))
            });
            let original = images.current_path().to_path_buf();
            let choice = match extension(&target).as_str() {
                "jpg" | "jpeg" => "jpeg",
                "tif" | "tiff" => "tiff",
                "pnm" => "ppm",
                other => other,
            }
            .to_owned();
            let quality_choice = match quality {
                0..=60 => "low",
                61..=82 => "medium",
                83..=95 => "high",
                _ => "best",
            };
            let Some(format) = editor::format_for_choice(&choice, quality_choice) else {
                return reply(
                    answer,
                    Err(invalid(
                        "An image exports as .png, .jpg, .webp, .tiff, .bmp, .tga, .qoi, .ppm or \
                         .exr.",
                    )),
                );
            };
            let format = match format {
                SaveFormat::Jpeg { .. } => SaveFormat::Jpeg { quality },
                format => format,
            };
            // A raster image at another size is drawn and written here, so
            // the open image keeps its size.
            if let (Some(scale), Some(image)) = (scaled, raster) {
                let scale = scale.clamp(0.01, 16.0);
                let width = ((image.width as f32 * scale).round() as u32).max(1);
                let height = ((image.height as f32 * scale).round() as u32).max(1);
                let pixels = image.pixels;
                return answer.clone().later(Task::perform(
                    async move {
                        let frame = match markup {
                            Some((handle, size, image_scale)) => {
                                let display = display(&handle, 0).await?;
                                let at = image_scale * scale;
                                let (wide, high) = prev_pdf::engine::page_pixels(size, at);
                                let bitmap = off_thread(move || {
                                    display.render(
                                        at,
                                        prev_pdf::geometry::PixelRect {
                                            x: 0,
                                            y: 0,
                                            width: wide,
                                            height: high,
                                        },
                                    )
                                })
                                .await?
                                .map_err(failed)?;
                                Frame {
                                    width: bitmap.width,
                                    height: bitmap.height,
                                    pixels: bitmap.pixels,
                                    delay: std::time::Duration::ZERO,
                                }
                            }
                            None => off_thread(move || {
                                pixels
                                    .draw(None)
                                    .map(|frame| resized(&frame, width, height))
                            })
                            .await?
                            .map_err(failed)?,
                        };
                        let (wide, high) = (frame.width, frame.height);
                        let path = target.clone();
                        off_thread(move || editor::export(&original, &frame, &path, format))
                            .await?
                            .map_err(failed)?;
                        Ok(Output::Json(json!({
                            "files": [target.display().to_string()],
                            "width": wide,
                            "height": high,
                        })))
                    },
                    |result| result,
                ));
            }
            let scale = input.scale.map(|scale| scale.clamp(0.1, 16.0));
            app.agents.exports.push((id, answer.clone()));
            app.update(Message::Image(
                id,
                prev::image::window::Message::Exporting(target, format, choice, scale),
            ))
        }
        shown => reply(
            answer,
            Err(invalid(format!(
                "Window {} shows {}; export is for PDFs and images here.",
                number(id),
                shown.what()
            ))),
        ),
    }
}

/// `frame` scaled to `width` by `height`.
fn resized(frame: &Frame, width: u32, height: u32) -> Frame {
    if (frame.width, frame.height) == (width, height) {
        return frame.clone();
    }
    let Some(image) = image::RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone())
    else {
        return frame.clone();
    };
    let scaled =
        image::imageops::resize(&image, width, height, image::imageops::FilterType::Lanczos3);
    Frame {
        width,
        height,
        pixels: scaled.into_raw(),
        delay: std::time::Duration::ZERO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pictures_fit_their_box_in_the_middle() {
        // A 2:1 picture in a square box fills its width.
        let rect = fitted(Some(Rect::new(0.0, 0.0, 100.0, 100.0)), None, 200, 100);
        assert_eq!(rect, Rect::new(0.0, 25.0, 100.0, 75.0));
        let rect = fitted(None, Some((10.0, 20.0, 50.0)), 200, 100);
        assert_eq!(rect, Rect::new(10.0, 20.0, 60.0, 45.0));
    }
}
