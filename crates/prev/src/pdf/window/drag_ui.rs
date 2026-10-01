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

    /// Takes a drop at `x`, `y` in the window. With `to_panel` (Ctrl held),
    /// a drop over the page goes among the pages, at the end, as a drop on
    /// the thumbnails would. Returns files it does not take, for the app
    /// to open.
    pub fn drop_in(
        &mut self,
        x: f32,
        y: f32,
        dropped: Dropped,
        action: Action,
        to_panel: bool,
    ) -> (Task<Message>, Vec<PathBuf>) {
        self.drop_hover = None;
        if !matches!(self.state, State::Ready(_)) {
            return (Task::none(), dropped_files(dropped));
        }
        let on_pages = self.drops_on_pages(x) && !self.image_mode;
        let to_pages = on_pages || (to_panel && !self.image_mode);
        // Where among the pages it goes: the line on the thumbnails, else
        // the end.
        let gap = if on_pages {
            self.thumbnail_gap(y)
        } else {
            self.viewer().map(|viewer| viewer.page_count())
        };
        let at = self.document_point(x, y);
        match dropped {
            Dropped::Pages(bytes) if !self.image_mode => {
                let State::Ready(viewer) = &mut self.state else {
                    return (Task::none(), Vec::new());
                };
                let gap =
                    if to_pages { gap } else { None }.unwrap_or_else(|| viewer.insertion_point());
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
            Dropped::Image(image) if to_pages => {
                self.insert_at = gap;
                (self.insert_images(vec![image]), Vec::new())
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
            // PDFs and images go in among the pages; other files open as
            // usual.
            Dropped::Files(paths) if to_pages => {
                let (pages, others): (Vec<_>, Vec<_>) =
                    paths.into_iter().partition(|path| becomes_pages(path));
                if pages.is_empty() {
                    return (Task::none(), others);
                }
                self.insert_at = gap;
                (self.insert_files(pages), others)
            }
            // One image file dropped on a page goes on it.
            Dropped::Files(paths) if at.is_some() && paths.len() == 1 => {
                let path = paths[0].clone();
                let Ok(Some(crate::filetype::FileKind::Image(format))) =
                    crate::filetype::detect_path(&path)
                else {
                    return self.ask_about_pdfs(paths);
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
            Dropped::Files(paths) => self.ask_about_pdfs(paths),
            Dropped::Nothing => (Task::none(), Vec::new()),
        }
    }

    /// Tells an image's markup whether the pointer is over the image
    /// window's sidebar.
    pub fn set_pointer_over_outer_sidebar(&mut self, over: bool) {
        self.pointer_over_sidebar = over.then_some(0);
    }

    /// Whether an image annotation is being moved by its body.
    pub(super) fn moving_image(&self) -> bool {
        self.viewer()
            .is_some_and(|viewer| viewer.moving_image().is_some())
    }

    /// An image annotation moved over the sidebar shows the line where it
    /// would go; let go there, it becomes a page (or, in an image's markup,
    /// an image of the window) instead of moving on its page. Shift, or
    /// Command on macOS, moves it there. `None` leaves the message to the
    /// viewer. The page canvas does not see where the pointer is outside
    /// it, so the sidebar tracks that.
    pub(super) fn image_to_panel(&mut self, message: &PdfMessage) -> Option<Task<Message>> {
        if !matches!(message, PdfMessage::Release { .. }) || !self.moving_image() {
            return None;
        }
        self.drop_hover = None;
        let gap = self.pointer_over_sidebar?;
        let moving = self.modifiers.shift() || (cfg!(target_os = "macos") && self.modifiers.logo());
        let State::Ready(viewer) = &mut self.state else {
            return None;
        };
        let (page, annotation) = viewer
            .moving_image()
            .map(|(page, annotation)| (page, annotation.clone()))?;
        viewer.cancel_move();
        let render = viewer.render_annotation(page, &annotation);
        let id = annotation.id;
        Some(Task::perform(render, move |result| {
            Message::AnnotationRendered(result, gap, page, id.clone(), moving)
        }))
    }

    pub(super) fn annotation_rendered(
        &mut self,
        result: Result<Bitmap, String>,
        gap: usize,
        page: usize,
        id: String,
        moving: bool,
    ) -> Task<Message> {
        let image = match result {
            Ok(image) => image,
            Err(error) => {
                self.notice = Some(error);
                return Task::none();
            }
        };
        let added = if self.image_mode {
            self.effects.push(super::Effect::ToSidebar(image));
            Task::none()
        } else {
            self.insert_at = Some(gap);
            self.insert_images(vec![image])
        };
        if !moving {
            return added;
        }
        let Some(viewer) = self.viewer_mut() else {
            return added;
        };
        let removed = viewer.remove_annotation(page, &id);
        Task::batch([added, self.viewer_task(removed)])
    }

    /// PDFs dropped on the page: the window asks whether they join this
    /// document or open in their own windows. Other files open.
    fn ask_about_pdfs(&mut self, paths: Vec<PathBuf>) -> (Task<Message>, Vec<PathBuf>) {
        if self.image_mode {
            return (Task::none(), paths);
        }
        let (pdfs, others): (Vec<_>, Vec<_>) = paths.into_iter().partition(|path| {
            matches!(
                crate::filetype::detect_path(path),
                Ok(Some(crate::filetype::FileKind::Pdf))
            )
        });
        if !pdfs.is_empty() {
            self.dropped_pdfs = Some(pdfs);
        }
        (Task::none(), others)
    }

    /// The answer about dropped PDFs: added at the end, opened in their
    /// own windows, or neither.
    pub(super) fn dropped_pdfs_choice(&mut self, choice: PdfDropChoice) -> Task<Message> {
        let Some(paths) = self.dropped_pdfs.take() else {
            return Task::none();
        };
        match choice {
            PdfDropChoice::Add => {
                self.insert_at = self.viewer().map(|viewer| viewer.page_count());
                self.insert_files(paths)
            }
            PdfDropChoice::Open => {
                self.effects.push(super::Effect::Open(paths));
                Task::none()
            }
            PdfDropChoice::Cancel => Task::none(),
        }
    }

    pub(super) fn dropped_pdfs_dialog<'a>(
        &'a self,
        base: iced::Element<'a, Message>,
    ) -> iced::Element<'a, Message> {
        use crate::ui::button::Kind as ButtonKind;
        use crate::ui::{self, Icon, component};
        let Some(paths) = &self.dropped_pdfs else {
            return base;
        };
        let body = match paths.as_slice() {
            [path] => crate::fl!(
                "drop-pdf-body",
                name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default()
            ),
            _ => crate::fl!("drop-pdfs-body", count = paths.len()),
        };
        component::dialog(
            base,
            Some(Icon::NoteAdd),
            crate::fl!("drop-pdf-title"),
            body,
            vec![
                ui::button(ButtonKind::Text, crate::fl!("common-cancel"))
                    .on_press(Message::DroppedPdfs(PdfDropChoice::Cancel))
                    .into(),
                ui::button(ButtonKind::Text, crate::fl!("drop-pdf-open"))
                    .on_press(Message::DroppedPdfs(PdfDropChoice::Open))
                    .into(),
                ui::button(ButtonKind::Filled, crate::fl!("drop-pdf-add"))
                    .on_press(Message::DroppedPdfs(PdfDropChoice::Add))
                    .into(),
            ],
        )
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

/// What dropped PDFs become: added to the document or opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfDropChoice {
    Add,
    Open,
    Cancel,
}

/// Whether a file dropped among the pages goes in as pages: a PDF, or an
/// image as a page of its own.
fn becomes_pages(path: &std::path::Path) -> bool {
    matches!(
        crate::filetype::detect_path(path),
        Ok(Some(
            crate::filetype::FileKind::Pdf | crate::filetype::FileKind::Image(_)
        ))
    )
}

fn dropped_files(dropped: Dropped) -> Vec<PathBuf> {
    match dropped {
        Dropped::Files(paths) => paths,
        _ => Vec::new(),
    }
}
