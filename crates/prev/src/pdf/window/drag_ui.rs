//! Drag and drop in a document window: drops of pages, images, text and
//! files where they land, and drags of pages, selected text and areas out
//! to other windows and apps.

use std::path::PathBuf;
use std::sync::Arc;

use iced::Task;
use prev_pdf::engine::Bitmap;

use super::{Message, PdfWindow, State};
use crate::drag::{self, Action, Dropped};
use crate::image::editor::spawn;
use crate::pdf::viewer::PdfMessage;
use crate::pdf::viewer::editing::{EditMessage, Outgoing};

/// Pages this window is dragging out, until the drag ends.
#[derive(Debug, Clone)]
pub struct OutgoingPages {
    pages: Vec<usize>,
    /// Dropped back on this window, which moved or copied them itself.
    handled: bool,
}

impl PdfWindow {
    /// `x`, `y` in the window as a point in document space, when over the
    /// page canvas.
    fn document_point(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        let State::Ready(viewer) = &self.state else {
            return None;
        };
        let bounds = self.canvas_bounds.get();
        bounds
            .contains(iced::Point::new(x, y))
            .then_some((viewer.view.x + x - bounds.x, viewer.view.y + y - bounds.y))
    }

    /// Where among the pages a drop at window height `y` on the thumbnails
    /// goes.
    fn thumbnail_gap(&self, y: f32) -> Option<usize> {
        let bounds = self.thumbnails_bounds.get();
        self.drop_gap(y - bounds.y + self.thumbnails_view.0)
    }

    /// Whether `x`, `y` in the window is over the page canvas.
    pub fn is_over_pages(&self, x: f32, y: f32) -> bool {
        self.document_point(x, y).is_some()
    }

    /// Something is dragged over the window at `x`, `y`, or has left it
    /// (`None`): over the thumbnails, a line shows where it would go.
    pub fn drag_over(&mut self, at: Option<(f32, f32)>) {
        self.drop_hover = at
            .filter(|(x, _)| self.drops_on_pages(*x) && !self.image_mode)
            .and_then(|(_, y)| self.thumbnail_gap(y));
    }

    /// Takes a drop at `x`, `y` in the window. Returns files it does not
    /// take, for the app to open.
    pub fn drop_in(
        &mut self,
        x: f32,
        y: f32,
        dropped: Dropped,
        action: Action,
    ) -> (Task<Message>, Vec<PathBuf>) {
        self.drop_hover = None;
        if !matches!(self.state, State::Ready(_)) {
            return (Task::none(), dropped_files(dropped));
        }
        let on_pages = self.drops_on_pages(x) && !self.image_mode;
        let at = self.document_point(x, y);
        match dropped {
            Dropped::Pages(bytes) if !self.image_mode => {
                let gap = if on_pages {
                    self.thumbnail_gap(y)
                } else {
                    None
                };
                let State::Ready(viewer) = &mut self.state else {
                    return (Task::none(), Vec::new());
                };
                let gap = gap.unwrap_or_else(|| viewer.insertion_point());
                // Pages of this window, dragged out and back: moved or
                // copied here.
                if let Some(outgoing) = self.outgoing_pages.as_mut() {
                    outgoing.handled = true;
                    if action == Action::Move {
                        let pages = outgoing.pages.clone();
                        let task = viewer.move_pages(&pages, gap);
                        return (self.viewer_task(task), Vec::new());
                    }
                }
                let task = viewer.insert_document(gap, bytes);
                (self.viewer_task(task), Vec::new())
            }
            Dropped::Pages(_) => {
                self.notice = Some(crate::fl!("drag-pages-need-document"));
                (Task::none(), Vec::new())
            }
            Dropped::Image(image) => (
                self.viewer_update(PdfMessage::Editing(EditMessage::PasteImage(
                    Arc::new(image),
                    at,
                ))),
                Vec::new(),
            ),
            Dropped::Text(text) => (
                self.viewer_update(PdfMessage::Editing(EditMessage::PasteText(text, at))),
                Vec::new(),
            ),
            // PDFs go in among the pages; other files open as usual.
            Dropped::Files(paths) if on_pages => {
                let (pdfs, others): (Vec<_>, Vec<_>) = paths.into_iter().partition(|path| {
                    matches!(
                        crate::filetype::detect_path(path),
                        Ok(Some(crate::filetype::FileKind::Pdf))
                    )
                });
                if pdfs.is_empty() {
                    return (Task::none(), others);
                }
                self.insert_at = self.thumbnail_gap(y);
                (self.insert_files(pdfs), others)
            }
            // With the markup bar open, an image file dropped on a page
            // goes on it.
            Dropped::Files(paths)
                if (self.markup_bar || self.image_mode) && at.is_some() && paths.len() == 1 =>
            {
                let path = paths[0].clone();
                let Ok(Some(crate::filetype::FileKind::Image(format))) =
                    crate::filetype::detect_path(&path)
                else {
                    return (Task::none(), paths);
                };
                (
                    Task::perform(
                        spawn(move || {
                            std::fs::read(&path)
                                .ok()
                                .and_then(|bytes| crate::paste::decode(&bytes, format))
                        }),
                        move |result| Message::DroppedImage(result.ok().flatten(), at),
                    ),
                    Vec::new(),
                )
            }
            Dropped::Files(paths) => (Task::none(), paths),
            Dropped::Nothing => (Task::none(), Vec::new()),
        }
    }

    pub(super) fn dropped_image(
        &mut self,
        image: Option<Bitmap>,
        at: Option<(f32, f32)>,
    ) -> Task<Message> {
        match image {
            Some(image) => self.viewer_update(PdfMessage::Editing(EditMessage::PasteImage(
                Arc::new(image),
                at,
            ))),
            None => {
                self.notice = Some(crate::fl!("drag-image-unsupported"));
                Task::none()
            }
        }
    }

    /// Starts dragging selected text or an area out of the document.
    pub(super) fn drag_out(&mut self, out: Outgoing) -> Task<Message> {
        match out {
            Outgoing::Text(text) => {
                self.start_drag(drag::text_data(&text), None, false);
                Task::none()
            }
            Outgoing::Area(page, rect) => {
                let State::Ready(viewer) = &self.state else {
                    return Task::none();
                };
                match viewer.render_area(page, rect) {
                    Some(render) => Task::perform(render, Message::AreaDragReady),
                    None => Task::none(),
                }
            }
        }
    }

    pub(super) fn area_drag_ready(&mut self, result: Result<Bitmap, String>) -> Task<Message> {
        let bitmap = match result {
            Ok(bitmap) => bitmap,
            Err(error) => {
                self.notice = Some(crate::fl!("drag-area-failed", error = error));
                return Task::none();
            }
        };
        let Some(png) = drag::png(&bitmap) else {
            return Task::none();
        };
        let icon = drag::icon(bitmap.width, bitmap.height, &bitmap.pixels);
        self.start_drag(vec![("image/png".to_owned(), png)], icon, false);
        Task::none()
    }

    /// The pages dragged out of the window are ready to go, as a PDF.
    pub(super) fn pages_drag_ready(
        &mut self,
        pages: Vec<usize>,
        result: Result<Vec<u8>, String>,
    ) -> Task<Message> {
        let bytes = match result {
            Ok(bytes) => bytes,
            Err(error) => {
                self.notice = Some(crate::fl!("drag-pages-failed", error = error));
                return Task::none();
            }
        };
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        let stem = self
            .path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_else(|| crate::fl!("drag-file-pages"));
        let labels: Vec<String> = pages.iter().map(|page| viewer.page_label(*page)).collect();
        let name = match labels.as_slice() {
            [one] => crate::fl!("drag-file-one-page", name = stem, page = one.as_str()),
            [first, .., last] => crate::fl!(
                "drag-file-page-range",
                name = stem,
                first = first.as_str(),
                last = last.as_str()
            ),
            [] => stem,
        };
        let name = format!("{name}.pdf");
        let mut data = vec![
            (crate::paste::PAGES_TYPE.to_owned(), bytes.clone()),
            ("application/pdf".to_owned(), bytes.clone()),
        ];
        // File managers take a file.
        if let Some(path) = drag::temp_file(&name, &bytes) {
            data.insert(1, ("text/uri-list".to_owned(), drag::uri_list(&path)));
        }
        let icon = pages
            .first()
            .and_then(|page| viewer.previews.get(page))
            .and_then(drag::icon_from_handle);
        if self.start_drag(data, icon, true) {
            self.outgoing_pages = Some(OutgoingPages {
                pages,
                handled: false,
            });
        }
        Task::none()
    }

    fn start_drag(
        &mut self,
        data: Vec<(String, Vec<u8>)>,
        icon: Option<drag::Icon>,
        allow_move: bool,
    ) -> bool {
        let started = drag::start(data, icon, allow_move);
        if started {
            self.drag_started = true;
        } else {
            self.notice = Some(crate::fl!("drag-start-failed"));
        }
        started
    }

    /// Whether this window started the drag under way; the app tells it
    /// how the drag ended.
    pub fn take_drag_started(&mut self) -> bool {
        std::mem::take(&mut self.drag_started)
    }

    /// A drag this window started ended. Pages moved to another window or
    /// app are removed here.
    pub fn drag_ended(&mut self, action: Option<Action>) -> Task<Message> {
        let Some(outgoing) = self.outgoing_pages.take() else {
            return Task::none();
        };
        if action != Some(Action::Move) || outgoing.handled {
            return Task::none();
        }
        let State::Ready(viewer) = &mut self.state else {
            return Task::none();
        };
        viewer.selected_pages = outgoing.pages.into_iter().collect();
        let task = viewer.remove_pages();
        self.viewer_task(task)
    }
}

fn dropped_files(dropped: Dropped) -> Vec<PathBuf> {
    match dropped {
        Dropped::Files(paths) => paths,
        _ => Vec::new(),
    }
}
