//! Markup on images: the PDF markup tools over a page made of the image,
//! kept while the window is open and drawn into the pixels on export.

use std::path::PathBuf;
use std::sync::Arc;

use iced::{Element, Task};
use prev_image::decode::Frame;
use prev_pdf::engine::zoom_to_scale;
use prev_pdf::geometry::PixelRect;
use prev_pdf::worker::flatten;

use super::dnd::OnOpen;
use super::{CloseChoice, ImageWindow, ItemState, Markup, Message, PendingExport, Source};
use crate::i18n::Describe;
use crate::image::editor::{self, spawn};
use crate::pdf::window::{self as pdf_window, PdfWindow};
use crate::shortcuts::Action;
use crate::ui::button::Kind;
use crate::ui::{self, Icon, component};

/// Wraps a markup window's message for the image window, passing on the
/// ones the app handles.
pub(super) fn wrap(index: usize) -> impl Fn(pdf_window::Message) -> Message + Clone {
    move |message| match message {
        pdf_window::Message::ToggleFloatingBars => Message::ToggleFloatingBars,
        pdf_window::Message::OpenSettings => Message::OpenSettings,
        message => Message::Markup(index, message),
    }
}

impl ImageWindow {
    pub(super) fn markup(&self) -> Option<&Markup> {
        self.items.get(self.current)?.markup.as_ref()
    }

    /// Whether the current image is shown with its markup, or will be.
    pub(super) fn marked(&self) -> bool {
        self.items
            .get(self.current)
            .is_some_and(|item| item.markup.is_some() || item.markup_starting)
    }

    /// The zoom in image pixels, while marking up.
    pub(super) fn markup_zoom(&self) -> Option<f32> {
        let markup = self.markup()?;
        Some(zoom_to_scale(markup.window.zoom()?) / markup.scale)
    }

    /// Runs `run` on the current image's markup window.
    pub(super) fn with_markup(
        &mut self,
        run: impl FnOnce(&mut PdfWindow, f32) -> Task<pdf_window::Message>,
    ) -> Option<Task<Message>> {
        let index = self.current;
        let markup = self.items.get_mut(index)?.markup.as_mut()?;
        let task = run(&mut markup.window, markup.scale);
        // Markup windows never go full screen.
        markup.window.take_effects();
        Some(task.map(wrap(index)))
    }

    /// Opens the markup bar, making the markup the first time; closing the
    /// bar of markup without annotations returns to the plain image.
    pub(super) fn toggle_markup(&mut self) -> Task<Message> {
        let index = self.current;
        if self.items[index].markup_starting {
            return Task::none();
        }
        if let Some(markup) = self.items[index].markup.as_mut() {
            let task = markup
                .window
                .update(pdf_window::Message::ToggleMarkupBar)
                .map(wrap(index));
            if !markup.window.markup_bar_shown() && !markup.window.has_annotations() {
                // Back to the plain image, as large and at the same spot
                // as the markup showed it.
                let scale = markup.scale;
                let center = markup
                    .window
                    .zoomed_center()
                    .map(|(zoom, fraction)| (zoom_to_scale(zoom) / scale, fraction));
                self.items[index].markup = None;
                return self.show_centered(center);
            }
            return task;
        }
        let Some(shown) = self.shown() else {
            return Task::none();
        };
        if !shown.is_editable() || !matches!(self.items[index].source, Source::Raster(_)) {
            self.notice = Some(crate::fl!("image-cannot-mark-up"));
            return Task::none();
        }
        if shown
            .editor
            .as_ref()
            .is_some_and(|editor| editor.generation != shown.shown_generation)
        {
            self.notice = Some(crate::fl!("image-mark-up-wait"));
            return Task::none();
        }
        let Some(frame) = shown.current_frame() else {
            return Task::none();
        };
        let scale = editor::markup_scale(frame.width, frame.height);
        self.items[index].markup_starting = true;
        self.selecting = false;
        self.selection = None;
        Task::perform(
            spawn(move || editor::markup_document(&frame)),
            move |result| {
                Message::MarkupMade(
                    index,
                    scale,
                    result.unwrap_or_else(|_| Err(crate::fl!("image-markup-stopped"))),
                )
            },
        )
    }

    /// Paste on an image without markup: starts the markup and pastes
    /// into it once it is open.
    pub(super) fn paste_into_new_markup(&mut self) -> Task<Message> {
        let index = self.current;
        if self.items[index].markup_starting {
            self.items[index].on_open = Some(OnOpen::Paste);
            return Task::none();
        }
        let task = self.toggle_markup();
        let item = &mut self.items[index];
        if item.markup_starting {
            item.on_open = Some(OnOpen::Paste);
        }
        task
    }

    pub(super) fn markup_made(
        &mut self,
        index: usize,
        scale: f32,
        result: Result<PathBuf, String>,
    ) -> Task<Message> {
        // The markup opens as large and at the same spot as the image,
        // unless the image is fitted to the window. PDF zooms count points
        // at 96 dpi.
        let start = (index == self.current)
            .then(|| self.zoomed_center())
            .flatten()
            .map(|(zoom, fraction)| (zoom * scale / zoom_to_scale(1.0), fraction));
        let Some(item) = self.items.get_mut(index) else {
            return Task::none();
        };
        item.markup_starting = false;
        let file = match result {
            Ok(file) => file,
            Err(error) => {
                self.notice = Some(error);
                return Task::none();
            }
        };
        let (mut window, opening) = PdfWindow::open_image_markup(file.clone());
        if let Some((zoom, fraction)) = start {
            window.start_at(zoom, fraction);
        }
        let device = window.set_device_scale(self.device_scale);
        window.set_pointer_inside(self.pointer_inside);
        item.markup = Some(Markup {
            window: Box::new(window),
            file,
            scale,
            exported: 0,
        });
        Task::batch([opening, device]).map(wrap(index))
    }

    pub(super) fn markup_message(
        &mut self,
        index: usize,
        message: pdf_window::Message,
    ) -> Task<Message> {
        let Some(markup) = self
            .items
            .get_mut(index)
            .and_then(|item| item.markup.as_mut())
        else {
            return Task::none();
        };
        let opened = matches!(message, pdf_window::Message::Opened(_));
        let mut task = markup.window.update(message);
        // MuPDF keeps the file open, so it can go now; nothing is left
        // behind however the app ends.
        if opened {
            let _ = std::fs::remove_file(&markup.file);
            let item = &mut self.items[index];
            if let Some(on_open) = item.on_open.take()
                && let Some(markup) = item.markup.as_mut()
            {
                let then = match on_open {
                    OnOpen::Paste => markup.window.shortcut(Action::Paste),
                    OnOpen::Drop(x, y, dropped) => Some(
                        markup
                            .window
                            .drop_in(x, y, dropped, crate::drag::Action::Copy)
                            .0,
                    ),
                };
                if let Some(then) = then {
                    task = Task::batch([task, then]);
                }
            }
        }
        if let Some(markup) = self.items[index].markup.as_mut() {
            markup.window.take_effects();
        }
        task.map(wrap(index))
    }

    /// Hands the device scale and pointer to every markup window.
    pub(super) fn markup_device_scale(&mut self, scale: f32) -> Task<Message> {
        Task::batch(
            self.items
                .iter_mut()
                .enumerate()
                .filter_map(|(index, item)| {
                    let markup = item.markup.as_mut()?;
                    Some(markup.window.set_device_scale(scale).map(wrap(index)))
                })
                .collect::<Vec<_>>(),
        )
    }

    pub(super) fn markup_pointer(&mut self, inside: bool) {
        for markup in self
            .items
            .iter_mut()
            .filter_map(|item| item.markup.as_mut())
        {
            markup.window.set_pointer_inside(inside);
        }
    }

    /// Zooms the markup view to `zoom` image pixels per view pixel.
    pub(super) fn markup_zoom_to(&mut self, zoom: f32) -> Option<Task<Message>> {
        // PDF zooms count points at 96 dpi.
        self.with_markup(|window, scale| window.zoom_to(zoom * scale / zoom_to_scale(1.0)))
    }

    /// Exports the current image with its markup drawn in.
    pub(super) fn export_marked(
        &mut self,
        pending: PendingExport,
        frame: Arc<Frame>,
    ) -> Option<Task<Message>> {
        let index = self.current;
        let markup = self.items[index].markup.as_ref()?;
        if !markup.window.has_annotations() {
            return None;
        }
        let layer = markup.window.document()?.annotation_layer(0);
        let (scale, edits) = (markup.scale, markup.window.edits());
        let original = self.items[index].path.clone();
        let PendingExport { path, format, .. } = pending;
        Some(Task::perform(
            async move {
                let display = flatten(layer.await).map_err(|error| error.describe())?;
                let work = spawn(move || {
                    let area = PixelRect {
                        x: 0,
                        y: 0,
                        width: frame.width,
                        height: frame.height,
                    };
                    let layer = display
                        .render(scale, area)
                        .map_err(|error| error.describe())?;
                    let burned = editor::burn_in(&frame, &layer.pixels);
                    editor::export(&original, &burned, &path, format).map(|()| path)
                });
                work.await
                    .unwrap_or_else(|_| Err(crate::fl!("export-stopped")))
            },
            move |result| Message::MarkupExported(index, edits, result),
        ))
    }

    pub(super) fn markup_exported(
        &mut self,
        index: usize,
        edits: u64,
        result: Result<PathBuf, String>,
    ) -> Task<Message> {
        if result.is_ok()
            && let Some(markup) = self
                .items
                .get_mut(index)
                .and_then(|item| item.markup.as_mut())
        {
            markup.exported = edits;
        }
        self.update(Message::Exported(result))
    }

    /// Whether the window can close now. If markup would be lost, it asks
    /// first and the answer comes as `Message::Close`.
    pub fn request_close(&mut self) -> bool {
        if self
            .items
            .iter()
            .any(|item| item.markup.as_ref().is_some_and(Markup::unexported))
        {
            self.close_prompt = true;
            return false;
        }
        true
    }

    /// Whether the user chose to close; the app closes the window.
    pub fn take_closing(&mut self) -> bool {
        std::mem::take(&mut self.closing)
    }

    pub(super) fn close_choice(&mut self, choice: CloseChoice) -> Task<Message> {
        self.close_prompt = false;
        match choice {
            CloseChoice::Cancel => Task::none(),
            CloseChoice::Discard => {
                self.closing = true;
                Task::none()
            }
            CloseChoice::Export => {
                let Some(index) = self
                    .items
                    .iter()
                    .position(|item| item.markup.as_ref().is_some_and(Markup::unexported))
                else {
                    return Task::none();
                };
                Task::batch([self.select(index), self.export()])
            }
        }
    }

    pub(super) fn close_prompt_view<'a>(
        &'a self,
        page: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let marked = self
            .items
            .iter()
            .filter(|item| item.markup.as_ref().is_some_and(Markup::unexported))
            .count();
        component::dialog(
            page,
            Some(Icon::Warning),
            crate::fl!("image-close-title"),
            crate::fl!("image-close-body", count = marked),
            vec![
                ui::button(Kind::Text, crate::fl!("common-cancel"))
                    .on_press(Message::Close(CloseChoice::Cancel))
                    .into(),
                ui::button(Kind::Text, crate::fl!("image-close-anyway"))
                    .on_press(Message::Close(CloseChoice::Discard))
                    .into(),
                ui::button(Kind::Filled, crate::fl!("export-choose"))
                    .on_press(Message::Close(CloseChoice::Export))
                    .into(),
            ],
        )
    }

    /// The markup canvas, bar and dialogs for the current image.
    pub(super) fn markup_parts(&self) -> Option<pdf_window::MarkupParts<'_>> {
        self.markup()?.window.markup_parts()
    }

    /// Whether a floating bar should stay for the markup's menus.
    pub(super) fn markup_holds_bars(&self) -> bool {
        self.markup()
            .is_some_and(|markup| markup.window.holds_bars())
    }
}

/// Keeps an image with markup in memory.
pub(super) fn keeps(item: &super::Item) -> bool {
    item.markup.is_some() && matches!(item.state, ItemState::Loaded(_))
}
