//! Page editing in the window: the Pages menu, selecting and dragging
//! thumbnails, copying pages between windows, inserting files, applying
//! redactions and the export dialog.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::ui::dir::column;
use crate::{column, row};
use iced::keyboard::Modifiers;
use iced::widget::{container, space};
use iced::{Center, Element, Fill, Task};
use prev_pdf::annotation::Kind;
use prev_pdf::engine::{ExportOptions, PageDisplay, Reduce};
use prev_pdf::worker::flatten;

use super::markup_ui::{Menu, menu_item};
use super::{Message, PdfWindow, State, THUMBNAIL_SPACING, thumbnail_height};
use crate::dialog;
use crate::i18n::Describe;
use crate::image::editor::spawn;
use crate::pdf::export::{self, Format};
use crate::pdf::viewer::{PdfMessage, PdfViewer, Pick};
use crate::ui::button::{self, Kind as ButtonKind};
use crate::ui::component::{self, Backdrop};
use crate::ui::popover::{self, popover};
use crate::ui::{self, Icon, Type, style};

/// Pages copied with Ctrl+C, as a PDF, for pasting into any window.
static CLIPBOARD: Mutex<Option<(Arc<Vec<u8>>, usize)>> = Mutex::new(None);

fn clipboard() -> Option<(Arc<Vec<u8>>, usize)> {
    CLIPBOARD.lock().ok()?.clone()
}

/// Whether pages were copied in this run of prev.
pub(super) fn has_copied_pages() -> bool {
    clipboard().is_some()
}

/// How far the pointer moves before a press on a thumbnail becomes a drag.
const DRAG_THRESHOLD: f32 = 6.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageAction {
    RotateLeft,
    RotateRight,
    Delete,
    InsertBlank,
    InsertFile,
    Copy,
    Paste,
    Crop,
    SelectAll,
    Export,
    /// Asks before applying redactions.
    Redact,
}

/// A press on a thumbnail, which may turn into a drag.
#[derive(Debug, Clone)]
pub struct ThumbnailDrag {
    pages: Vec<usize>,
    /// Pointer height in the list where the press began, once known.
    start: Option<f32>,
    current: f32,
    dragging: bool,
    /// A plain press on a selected page selects only it on release,
    /// unless the press became a drag of the whole selection.
    select_on_release: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ExportDialog {
    format: Format,
    dpi: f32,
    quality: u8,
    /// Only the pages selected in the sidebar.
    selected_only: bool,
    reduce: bool,
    /// Annotations and form fields drawn into the pages for good.
    flatten: bool,
    encrypt: bool,
    password: String,
    confirm: String,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ExportMessage {
    Format(Format),
    Dpi(f32),
    Quality(u8),
    SelectedOnly(bool),
    Reduce(bool),
    Flatten(bool),
    Encrypt(bool),
    Password(String),
    Confirm(String),
    Cancel,
    Choose,
    Target(Result<Option<PathBuf>, String>),
    Done(Result<String, String>),
}

impl PdfWindow {
    pub fn set_modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
    }

    pub(super) fn viewer_mut(&mut self) -> Option<&mut PdfViewer> {
        match &mut self.state {
            State::Ready(viewer) => Some(viewer),
            _ => None,
        }
    }

    pub fn viewer(&self) -> Option<&PdfViewer> {
        match &self.state {
            State::Ready(viewer) => Some(viewer),
            _ => None,
        }
    }

    /// The size for pages made from images inserted at `insert_at`: that
    /// of the page before it, else the first, else Letter.
    fn image_page_size(&self) -> prev_pdf::geometry::Size {
        let Some(viewer) = self.viewer() else {
            return prev_pdf::geometry::Size::new(612.0, 792.0);
        };
        let at = self.insert_at.unwrap_or_else(|| viewer.insertion_point());
        let sizes = &viewer.info.page_sizes;
        sizes
            .get(at.saturating_sub(1))
            .or(sizes.first())
            .copied()
            .unwrap_or(prev_pdf::geometry::Size::new(612.0, 792.0))
    }

    pub(super) fn page_action(&mut self, action: PageAction) -> Task<Message> {
        self.menu = None;
        self.overflow = None;
        let Some(viewer) = self.viewer_mut() else {
            return Task::none();
        };
        let task = match action {
            PageAction::RotateLeft => viewer.rotate_pages(-1),
            PageAction::RotateRight => viewer.rotate_pages(1),
            PageAction::Delete => viewer.remove_pages(),
            PageAction::InsertBlank => viewer.insert_blank_page(),
            PageAction::Crop => viewer.crop_to_area(),
            PageAction::SelectAll => {
                viewer.select_all_pages();
                self.pages_focus = true;
                return Task::none();
            }
            PageAction::Paste => {
                let Some((bytes, _)) = clipboard() else {
                    self.notice = Some(crate::fl!("pages-no-copied"));
                    return Task::none();
                };
                let at = viewer.insertion_point();
                viewer.insert_document(at, bytes)
            }
            PageAction::Copy => {
                let pages = viewer.target_pages();
                let receiver = viewer.handle.extract(pages.clone());
                let count = pages.len();
                return Task::perform(receiver, move |result| {
                    Message::PagesCopied(count, flatten(result).map_err(|error| error.describe()))
                });
            }
            PageAction::InsertFile => {
                return Task::perform(dialog::open_files(), Message::InsertChosen);
            }
            PageAction::Export => {
                self.export_dialog = Some(ExportDialog {
                    format: Format::Pdf,
                    dpi: 150.0,
                    quality: 92,
                    selected_only: viewer.selected_pages.len() > 1,
                    reduce: false,
                    flatten: false,
                    encrypt: false,
                    password: String::new(),
                    confirm: String::new(),
                    error: None,
                });
                return Task::none();
            }
            PageAction::Redact => {
                self.redact_confirm = true;
                return Task::none();
            }
        };
        self.viewer_task(task)
    }

    /// Runs a viewer task and handles the requests it left.
    pub(super) fn viewer_task(&mut self, task: Task<PdfMessage>) -> Task<Message> {
        let task = task.map(Message::Viewer);
        Task::batch([
            task,
            self.viewer_update(PdfMessage::ScrollBy { dx: 0.0, dy: 0.0 }),
        ])
    }

    pub(super) fn pages_copied(&mut self, count: usize, result: Result<Vec<u8>, String>) {
        match result {
            Ok(bytes) => {
                if let Ok(mut clipboard) = CLIPBOARD.lock() {
                    *clipboard = Some((Arc::new(bytes), count));
                }
                // Pages are now the latest thing copied, ahead of any text
                // or image on the clipboard.
                std::thread::spawn(crate::paste::mark_pages);
                self.notice = Some(crate::fl!("pages-copied", count = count));
            }
            Err(error) => self.notice = Some(crate::fl!("pages-copy-failed", error = error)),
        }
    }

    /// Reads chosen or dropped PDFs, and images as pages of their own, to
    /// insert at `insert_at`, else after the selected pages.
    pub fn insert_files(&mut self, paths: Vec<PathBuf>) -> Task<Message> {
        let paths: Vec<(PathBuf, Option<prev_image::ImageFormat>)> = paths
            .into_iter()
            .filter_map(|path| match crate::filetype::detect_path(&path) {
                Ok(Some(crate::filetype::FileKind::Pdf)) => Some((path, None)),
                Ok(Some(crate::filetype::FileKind::Image(format))) => Some((path, Some(format))),
                _ => None,
            })
            .collect();
        if paths.is_empty() {
            return Task::none();
        }
        let size = self.image_page_size();
        Task::perform(
            spawn(move || {
                paths
                    .iter()
                    .map(|(path, image)| {
                        let failed = |error: String| format!("{}: {error}", path.display());
                        let bytes =
                            std::fs::read(path).map_err(|error| failed(error.to_string()))?;
                        match image {
                            None => Ok(bytes),
                            Some(format) => crate::paste::decode(&bytes, *format)
                                .ok_or_else(|| failed(crate::fl!("pages-image-unreadable")))
                                .and_then(|image| image_page(&image, size)),
                        }
                    })
                    .collect::<Result<Vec<_>, String>>()
            }),
            |result| {
                Message::InsertRead(
                    result.unwrap_or_else(|_| Err(crate::fl!("pages-reading-stopped"))),
                )
            },
        )
    }

    /// Inserts `images` as pages of their own at `insert_at`.
    pub fn insert_images(&mut self, images: Vec<prev_pdf::engine::Bitmap>) -> Task<Message> {
        let size = self.image_page_size();
        Task::perform(
            spawn(move || {
                images
                    .iter()
                    .map(|image| image_page(image, size))
                    .collect::<Result<Vec<_>, String>>()
            }),
            |result| {
                Message::InsertRead(
                    result.unwrap_or_else(|_| Err(crate::fl!("pages-reading-stopped"))),
                )
            },
        )
    }

    pub(super) fn insert_read(&mut self, result: Result<Vec<Vec<u8>>, String>) -> Task<Message> {
        let documents = match result {
            Ok(documents) => documents,
            Err(error) => {
                self.notice = Some(crate::fl!("pages-read-failed", error = error));
                return Task::none();
            }
        };
        let insert_at = self.insert_at.take();
        let Some(viewer) = self.viewer_mut() else {
            return Task::none();
        };
        // Each goes in front of the next, so insert the last first.
        let at = insert_at.unwrap_or_else(|| viewer.insertion_point());
        let tasks: Vec<_> = documents
            .into_iter()
            .rev()
            .map(|bytes| viewer.insert_document(at, Arc::new(bytes)))
            .collect();
        self.viewer_task(Task::batch(tasks))
    }

    /// Whether a drop at `x` lands on the page thumbnails.
    pub fn drops_on_pages(&self, x: f32) -> bool {
        self.sidebar == Some(super::Sidebar::Thumbnails)
            && matches!(self.state, State::Ready(_))
            && x <= self.sidebar_width.value
    }

    // Thumbnails.

    pub(super) fn thumbnail_pressed(&mut self, page: usize) -> Task<Message> {
        let modifiers = self.modifiers;
        self.pages_focus = true;
        let Some(viewer) = self.viewer_mut() else {
            return Task::none();
        };
        let mut select_on_release = None;
        if modifiers.command() {
            viewer.pick_page(page, Pick::Toggle);
        } else if modifiers.shift() {
            viewer.pick_page(page, Pick::Extend);
        } else if viewer.selected_pages.contains(&page) {
            select_on_release = Some(page);
        } else {
            viewer.pick_page(page, Pick::Only);
        }
        let pages = viewer.target_pages();
        self.thumbnail_drag = Some(ThumbnailDrag {
            pages: if pages.contains(&page) {
                pages
            } else {
                vec![page]
            },
            start: None,
            current: 0.0,
            dragging: false,
            select_on_release,
        });
        if modifiers.shift() || modifiers.command() {
            return Task::none();
        }
        self.viewer_update(PdfMessage::GoTo { page, point: None })
    }

    pub(super) fn thumbnails_moved(&mut self, y: f32) {
        let Some(drag) = self.thumbnail_drag.as_mut() else {
            return;
        };
        let start = *drag.start.get_or_insert(y);
        drag.current = y;
        if (y - start).abs() > DRAG_THRESHOLD && !drag.dragging {
            drag.dragging = true;
            // Leaving the window takes the pages to other windows and apps.
            crate::drag::watch_pointer(true);
        }
    }

    /// The pointer left the window while dragging thumbnails: the drag
    /// goes on as one other windows and apps can take, carrying the pages
    /// as a PDF.
    pub fn drag_pages_out(&mut self) -> Task<Message> {
        crate::drag::watch_pointer(false);
        let Some(drag) = self.thumbnail_drag.take().filter(|drag| drag.dragging) else {
            return Task::none();
        };
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        let pages = drag.pages;
        let receiver = viewer.handle.extract(pages.clone());
        Task::perform(receiver, move |result| {
            Message::PagesDragReady(pages, flatten(result).map_err(|error| error.describe()))
        })
    }

    pub(super) fn thumbnails_released(&mut self) -> Task<Message> {
        crate::drag::watch_pointer(false);
        let Some(drag) = self.thumbnail_drag.take() else {
            return Task::none();
        };
        let gap = drag.dragging.then(|| self.drop_gap(drag.current)).flatten();
        let Some(viewer) = self.viewer_mut() else {
            return Task::none();
        };
        match gap {
            Some(gap) => {
                let task = viewer.move_pages(&drag.pages, gap);
                self.viewer_task(task)
            }
            None => {
                if let Some(page) = drag.select_on_release {
                    viewer.pick_page(page, Pick::Only);
                }
                Task::none()
            }
        }
    }

    /// The gap between thumbnails nearest to height `y` in the list: the
    /// page a drop there goes in front of.
    pub(super) fn drop_gap(&self, y: f32) -> Option<usize> {
        let State::Ready(viewer) = &self.state else {
            return None;
        };
        let width = self.thumbnail_width();
        let mut top = 0.0;
        for (page, size) in viewer.info.page_sizes.iter().enumerate() {
            let height = thumbnail_height(size, width) + THUMBNAIL_SPACING;
            if y < top + height / 2.0 {
                return Some(page);
            }
            top += height;
        }
        Some(viewer.page_count())
    }

    /// The gap a drag would drop into now, for drawing the marker.
    pub(super) fn dragging_gap(&self) -> Option<usize> {
        match self.thumbnail_drag.as_ref().filter(|drag| drag.dragging) {
            Some(drag) => self.drop_gap(drag.current),
            // Something dragged in from another window or app.
            None => self.drop_hover,
        }
    }

    pub(super) fn is_dragged(&self, page: usize) -> bool {
        self.thumbnail_drag
            .as_ref()
            .is_some_and(|drag| drag.dragging && drag.pages.contains(&page))
    }

    // The Pages menu.

    pub(super) fn pages_menu<'a>(&'a self, viewer: &'a PdfViewer) -> Element<'a, Message> {
        let _reading = crate::ui::dir::reading();
        let open = self.menu == Some(Menu::Pages);
        let anchor = component::tip(
            ui::icon_button(Icon::Stacks)
                .selected(open)
                .on_press(Message::Menu(Some(Menu::Pages))),
            crate::fl!("pages-menu"),
        );
        let count = viewer.target_pages().len();
        let item = |glyph: Icon, label: String, action: PageAction| {
            menu_item(Some(glyph), label, false, Message::PageAction(action))
        };
        let pasteable = clipboard();
        // Rotating is on the toolbar beside this menu.
        let mut items = vec![
            item(
                Icon::NoteAdd,
                crate::fl!("pages-insert-blank"),
                PageAction::InsertBlank,
            ),
            item(
                Icon::FileOpen,
                crate::fl!("pages-insert-file"),
                PageAction::InsertFile,
            ),
            item(
                Icon::ContentCopy,
                crate::fl!("pages-copy", count = count),
                PageAction::Copy,
            ),
        ];
        if let Some((_, pages)) = pasteable {
            items.push(menu_item(
                Some(Icon::ContentPaste),
                crate::fl!("pages-paste", count = pages),
                false,
                Message::PageAction(PageAction::Paste),
            ));
        }
        if viewer.edit.area.is_some() {
            items.push(item(Icon::Crop, crate::fl!("pages-crop"), PageAction::Crop));
        }
        items.push(item(
            Icon::SelectAll,
            crate::fl!("pages-select-all"),
            PageAction::SelectAll,
        ));
        items.push(item(
            Icon::Delete,
            crate::fl!("pages-delete", count = count),
            PageAction::Delete,
        ));
        if self.redaction_count() > 0 {
            items.push(item(
                Icon::RemoveSelection,
                crate::fl!("pages-apply-redactions"),
                PageAction::Redact,
            ));
        }
        let content = column(items).width(260).padding([0, 8]);
        popover(
            anchor,
            open.then(|| popover::surface(content)),
            Message::Menu(None),
        )
        .into()
    }

    /// Redaction marks waiting to be applied, from the notes list, which
    /// holds every page's annotations.
    pub(super) fn redaction_count(&self) -> usize {
        self.notes
            .iter()
            .flat_map(|(_, annotations)| annotations)
            .filter(|annotation| annotation.kind == Kind::Redact)
            .count()
    }

    pub(super) fn apply_redactions(&mut self) -> Task<Message> {
        self.redact_confirm = false;
        // The original must not survive as a version either.
        self.original_kept = true;
        if let Some(store) = prev_store::versions::VersionStore::default_location()
            && let Err(error) = store.forget(&self.path)
        {
            self.notice = Some(crate::fl!(
                "pages-forget-versions-failed",
                error = error.to_string()
            ));
        }
        let Some(viewer) = self.viewer_mut() else {
            return Task::none();
        };
        let task = viewer.apply_redactions();
        self.viewer_task(task)
    }

    pub(super) fn redact_dialog<'a>(&'a self, base: Element<'a, Message>) -> Element<'a, Message> {
        if !self.redact_confirm {
            return base;
        }
        let count = self.redaction_count();
        component::dialog(
            base,
            Some(Icon::RemoveSelection),
            crate::fl!("pages-redact-title"),
            crate::fl!("pages-redact-body", count = count),
            vec![
                ui::button(ButtonKind::Text, crate::fl!("common-cancel"))
                    .on_press(Message::ConfirmRedactions(false))
                    .into(),
                ui::button(ButtonKind::Filled, crate::fl!("pages-redact-apply"))
                    .on_press(Message::ApplyRedactions)
                    .into(),
            ],
        )
    }

    // Export.

    pub(super) fn export_update(&mut self, message: ExportMessage) -> Task<Message> {
        let Some(export) = self.export_dialog.as_mut() else {
            if let ExportMessage::Done(result) = message {
                self.notice = Some(match result {
                    Ok(done) => done,
                    Err(error) => crate::fl!("pages-export-failed", error = error),
                });
            }
            return Task::none();
        };
        match message {
            ExportMessage::Format(format) => export.format = format,
            ExportMessage::Dpi(dpi) => export.dpi = dpi,
            ExportMessage::Quality(quality) => export.quality = quality,
            ExportMessage::SelectedOnly(selected) => export.selected_only = selected,
            ExportMessage::Reduce(reduce) => export.reduce = reduce,
            ExportMessage::Flatten(flatten) => export.flatten = flatten,
            ExportMessage::Encrypt(encrypt) => export.encrypt = encrypt,
            ExportMessage::Password(password) => export.password = password,
            ExportMessage::Confirm(confirm) => export.confirm = confirm,
            ExportMessage::Cancel => self.export_dialog = None,
            ExportMessage::Choose => {
                if export.format == Format::Pdf && export.encrypt {
                    if export.password.is_empty() {
                        export.error = Some(crate::fl!("pages-export-no-password"));
                        return Task::none();
                    }
                    if export.password != export.confirm {
                        export.error = Some(crate::fl!("pages-export-password-mismatch"));
                        return Task::none();
                    }
                }
                export.error = None;
                let stem = self
                    .path
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_else(|| crate::fl!("pages-export-untitled"));
                let stem = if export.format == Format::Pdf && !export.selected_only {
                    // Without the direction marks right to left languages
                    // place around values, which would end up in the name.
                    crate::fl!("pages-export-file-name", name = stem)
                        .replace(['\u{2068}', '\u{2069}'], "")
                } else {
                    stem
                };
                let name = format!("{stem}.{}", export.format.extension());
                return Task::perform(
                    dialog::save_file(crate::fl!("pages-export-title"), name),
                    |result| Message::Export(ExportMessage::Target(result)),
                );
            }
            ExportMessage::Target(Ok(Some(target))) => return self.run_export(target),
            ExportMessage::Target(Ok(None)) => {}
            ExportMessage::Target(Err(error)) => {
                export.error = Some(crate::fl!("pdf-file-dialog-failed", error = error));
            }
            ExportMessage::Done(result) => {
                self.notice = Some(match result {
                    Ok(done) => done,
                    Err(error) => crate::fl!("pages-export-failed", error = error),
                });
            }
        }
        Task::none()
    }

    fn run_export(&mut self, target: PathBuf) -> Task<Message> {
        let Some(export) = self.export_dialog.take() else {
            return Task::none();
        };
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        if target == self.path {
            self.notice = Some(crate::fl!("pages-export-same-file"));
            return Task::none();
        }
        let pages = if export.selected_only {
            viewer.target_pages()
        } else {
            (0..viewer.page_count()).collect()
        };
        let name = target
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.notice = Some(crate::fl!("pages-export-exporting", name = name.as_str()));
        if export.format == Format::Pdf {
            let options = ExportOptions {
                password: export.encrypt.then(|| export.password.clone()),
                reduce: export.reduce.then_some(Reduce::DEFAULT),
                pages: export.selected_only.then_some(pages),
                flatten: export.flatten,
            };
            let destination = target.clone();
            let receiver = viewer.handle.export(
                options,
                Box::new(move |bytes| {
                    prev_store::atomic::write(&destination, bytes)
                        .map_err(|error| error.to_string())
                }),
            );
            return Task::perform(receiver, move |result| {
                Message::Export(ExportMessage::Done(
                    flatten(result)
                        .map(|()| crate::fl!("pages-export-done", name = name.as_str()))
                        .map_err(|error| error.describe()),
                ))
            });
        }
        let displays: Vec<_> = pages
            .iter()
            .map(|page| viewer.handle.display(*page))
            .collect();
        let (format, dpi, quality) = (export.format, export.dpi, export.quality);
        Task::perform(
            async move {
                let mut rendered: Vec<Arc<dyn PageDisplay>> = Vec::new();
                for display in displays {
                    rendered.push(flatten(display.await).map_err(|error| error.describe())?);
                }
                spawn(move || export::write_images(&target, &rendered, format, dpi, quality))
                    .await
                    .unwrap_or_else(|_| Err(crate::fl!("pages-export-stopped")))
            },
            move |result| {
                Message::Export(ExportMessage::Done(result.map(
                    |written| match written.len() {
                        1 => crate::fl!("pages-export-done", name = name.as_str()),
                        count => crate::fl!("pages-export-done-images", count = count),
                    },
                )))
            },
        )
    }

    pub(super) fn export_dialog_view<'a>(
        &'a self,
        base: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let Some(export) = &self.export_dialog else {
            return base;
        };
        let State::Ready(viewer) = &self.state else {
            return base;
        };
        let send = |message: ExportMessage| Message::Export(message);
        let formats = component::connected(
            Format::ALL
                .iter()
                .map(|format| {
                    ui::button(ButtonKind::Tonal, format.label())
                        .size(button::Size::ExtraSmall)
                        .selected(export.format == *format)
                        .on_press(send(ExportMessage::Format(*format)))
                })
                .collect(),
        );
        let check = |label: String, checked: bool, message: fn(bool) -> ExportMessage| {
            iced::widget::checkbox(checked)
                .label(label)
                .on_toggle(move |value| send(message(value)))
                .style(style::checkbox)
        };
        let selected = viewer.target_pages().len();
        let pages_label = crate::fl!("pages-export-selected-only", count = selected);
        let mut body = column![
            ui::styled(crate::fl!("pages-export-title"), Type::HeadlineSmall),
            component::section(crate::fl!("pages-export-format")),
            formats,
        ]
        .spacing(12)
        .width(440);
        if export.format == Format::Pdf {
            body = body.push(check(
                crate::fl!("pages-export-reduce"),
                export.reduce,
                ExportMessage::Reduce,
            ));
            body = body.push(check(
                crate::fl!("pages-export-flatten"),
                export.flatten,
                ExportMessage::Flatten,
            ));
            if export.flatten {
                body = body.push(ui::aligned(
                    ui::styled(crate::fl!("pages-export-flatten-detail"), Type::BodySmall)
                        .style(style::on_surface_variant),
                ));
            }
            body = body.push(check(
                crate::fl!("pages-export-encrypt"),
                export.encrypt,
                ExportMessage::Encrypt,
            ));
            if export.encrypt {
                body = body.push(component::text_field(
                    crate::i18n::lasting(crate::fl!("pages-export-password")),
                    &export.password,
                    Backdrop::ContainerHigh,
                    |input| {
                        input
                            .secure(true)
                            .on_input(|value| Message::Export(ExportMessage::Password(value)))
                    },
                ));
                body = body.push(component::text_field(
                    crate::i18n::lasting(crate::fl!("pages-export-verify-password")),
                    &export.confirm,
                    Backdrop::ContainerHigh,
                    |input| {
                        input
                            .secure(true)
                            .on_input(|value| Message::Export(ExportMessage::Confirm(value)))
                            .on_submit(send(ExportMessage::Choose))
                    },
                ));
            }
        } else {
            body = body.push(component::section(crate::fl!("pages-export-resolution")));
            body = body.push(component::connected(
                export::RESOLUTIONS
                    .iter()
                    .map(|dpi| {
                        ui::button(
                            ButtonKind::Tonal,
                            crate::fl!("pages-export-dpi", dpi = (dpi.round() as i64)),
                        )
                        .size(button::Size::ExtraSmall)
                        .selected(export.dpi == *dpi)
                        .on_press(send(ExportMessage::Dpi(*dpi)))
                    })
                    .collect(),
            ));
            if export.format == Format::Jpeg {
                body = body.push(component::section(crate::fl!("pages-export-quality")));
                body = body.push(component::connected(
                    export::qualities()
                        .into_iter()
                        .map(|(label, quality)| {
                            ui::button(ButtonKind::Tonal, label)
                                .size(button::Size::ExtraSmall)
                                .selected(export.quality == quality)
                                .on_press(send(ExportMessage::Quality(quality)))
                        })
                        .collect(),
                ));
            }
            let note = if export.format.is_multipage() {
                crate::fl!("pages-export-one-file")
            } else {
                crate::fl!("pages-export-file-per-page")
            };
            body = body.push(ui::aligned(
                ui::styled(note, Type::BodySmall).style(style::on_surface_variant),
            ));
        }
        body = body.push(
            iced::widget::checkbox(export.selected_only)
                .label(pages_label)
                .on_toggle(move |value| send(ExportMessage::SelectedOnly(value)))
                .style(style::checkbox),
        );
        if let Some(error) = &export.error {
            body = body.push(ui::aligned(
                ui::styled(error.as_str(), Type::BodyMedium).style(style::error_text),
            ));
        }
        body = body.push(
            row![
                space::horizontal(),
                ui::button(ButtonKind::Text, crate::fl!("common-cancel"))
                    .on_press(send(ExportMessage::Cancel)),
                ui::button(ButtonKind::Filled, crate::fl!("pages-export-choose"))
                    .on_press(send(ExportMessage::Choose)),
            ]
            .spacing(8)
            .align_y(Center),
        );
        let card = container(body).padding(24).style(style::dialog);
        iced::widget::stack![
            base,
            iced::widget::opaque(
                container(ui::enter::grow(card))
                    .center(Fill)
                    .style(style::scrim)
            )
        ]
        .into()
    }
}

/// The marker between thumbnails where dragged pages will go.
pub(super) fn drop_marker<'a>(shown: bool) -> Element<'a, Message> {
    let marker = container(space().height(3).width(Fill));
    if shown {
        marker
            .style(|theme: &iced::Theme| iced::widget::container::Style {
                background: Some(crate::ui::Scheme::of(theme).primary.into()),
                border: iced::border::rounded(2),
                ..Default::default()
            })
            .into()
    } else {
        marker.into()
    }
}

/// A one-page PDF of `image` fitted on a page of `size`.
fn image_page(
    image: &prev_pdf::engine::Bitmap,
    size: prev_pdf::geometry::Size,
) -> Result<Vec<u8>, String> {
    prev_pdf::image_page_document(image, size).map_err(|error| error.describe())
}
