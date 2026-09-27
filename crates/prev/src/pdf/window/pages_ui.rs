//! Page editing in the window: the Pages menu, selecting and dragging
//! thumbnails, copying pages between windows, inserting files, applying
//! redactions and the export dialog.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use iced::keyboard::Modifiers;
use iced::widget::{column, container, row, space};
use iced::{Center, Element, Fill, Task};
use prev_pdf::annotation::Kind;
use prev_pdf::engine::{ExportOptions, PageDisplay, Reduce};
use prev_pdf::worker::flatten;

use super::markup_ui::{Menu, menu_item};
use super::{Message, PdfWindow, State, THUMBNAIL_SPACING, thumbnail_height};
use crate::dialog;
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

    fn viewer_mut(&mut self) -> Option<&mut PdfViewer> {
        match &mut self.state {
            State::Ready(viewer) => Some(viewer),
            _ => None,
        }
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
                    self.notice = Some("There are no copied pages to paste.".into());
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
                    Message::PagesCopied(count, flatten(result).map_err(|error| error.to_string()))
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
                self.notice = Some(match count {
                    1 => "Copied 1 page.".to_owned(),
                    count => format!("Copied {count} pages."),
                });
            }
            Err(error) => self.notice = Some(format!("Could not copy the pages: {error}")),
        }
    }

    /// Reads chosen or dropped PDFs, to insert after the selected pages.
    pub fn insert_files(&mut self, paths: Vec<PathBuf>) -> Task<Message> {
        let paths: Vec<PathBuf> = paths
            .into_iter()
            .filter(|path| {
                matches!(
                    crate::filetype::detect_path(path),
                    Ok(Some(crate::filetype::FileKind::Pdf))
                )
            })
            .collect();
        if paths.is_empty() {
            return Task::none();
        }
        Task::perform(
            spawn(move || {
                paths
                    .iter()
                    .map(|path| {
                        std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))
                    })
                    .collect::<Result<Vec<_>, String>>()
            }),
            |result| Message::InsertRead(result.unwrap_or_else(|_| Err("reading stopped".into()))),
        )
    }

    pub(super) fn insert_read(&mut self, result: Result<Vec<Vec<u8>>, String>) -> Task<Message> {
        let documents = match result {
            Ok(documents) => documents,
            Err(error) => {
                self.notice = Some(format!("Could not read the file: {error}"));
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
        if modifiers.control() {
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
        if modifiers.shift() || modifiers.control() {
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
            Message::PagesDragReady(pages, flatten(result).map_err(|error| error.to_string()))
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
        let open = self.menu == Some(Menu::Pages);
        let anchor = component::tip(
            ui::icon_button(Icon::Stacks)
                .selected(open)
                .on_press(Message::Menu(Some(Menu::Pages))),
            "Pages",
        );
        let count = viewer.target_pages().len();
        let plural = |one: &'a str, many: &'a str| if count == 1 { one } else { many };
        let item = |glyph: Icon, label: &'a str, action: PageAction| {
            menu_item(Some(glyph), label, false, Message::PageAction(action))
        };
        let pasteable = clipboard();
        // Rotating is on the toolbar beside this menu.
        let mut items = vec![
            item(Icon::NoteAdd, "Insert Blank Page", PageAction::InsertBlank),
            item(Icon::FileOpen, "Insert from File…", PageAction::InsertFile),
            item(
                Icon::ContentCopy,
                plural("Copy Page", "Copy Pages"),
                PageAction::Copy,
            ),
        ];
        if let Some((_, pages)) = pasteable {
            items.push(menu_item(
                Some(Icon::ContentPaste),
                if pages == 1 {
                    "Paste Page".to_owned()
                } else {
                    format!("Paste {pages} Pages")
                },
                false,
                Message::PageAction(PageAction::Paste),
            ));
        }
        if viewer.edit.area.is_some() {
            items.push(item(Icon::Crop, "Crop to Selection", PageAction::Crop));
        }
        items.push(item(
            Icon::SelectAll,
            "Select All Pages",
            PageAction::SelectAll,
        ));
        items.push(item(
            Icon::Delete,
            plural("Delete Page", "Delete Pages"),
            PageAction::Delete,
        ));
        if self.redaction_count() > 0 {
            items.push(item(
                Icon::RemoveSelection,
                "Apply Redactions…",
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
            self.notice = Some(format!("Could not delete earlier versions: {error}"));
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
            "Apply redactions?",
            format!(
                "Text, images and drawings under {} are removed from the document for good, \
and the marks become black boxes. This can't be undone, and the earlier versions of this \
file that prev keeps are deleted.",
                if count == 1 {
                    "the mark".to_owned()
                } else {
                    format!("the {count} marks")
                }
            ),
            vec![
                ui::button(ButtonKind::Text, "Cancel")
                    .on_press(Message::ConfirmRedactions(false))
                    .into(),
                ui::button(ButtonKind::Filled, "Apply")
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
                    Err(error) => format!("Could not export: {error}"),
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
                        export.error = Some("Enter a password.".into());
                        return Task::none();
                    }
                    if export.password != export.confirm {
                        export.error = Some("The passwords don't match.".into());
                        return Task::none();
                    }
                }
                export.error = None;
                let stem = self
                    .path
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "document".into());
                let suffix = if export.format == Format::Pdf && !export.selected_only {
                    " (exported)"
                } else {
                    ""
                };
                let name = format!("{stem}{suffix}.{}", export.format.extension());
                return Task::perform(dialog::save_file("Export".into(), name), |result| {
                    Message::Export(ExportMessage::Target(result))
                });
            }
            ExportMessage::Target(Ok(Some(target))) => return self.run_export(target),
            ExportMessage::Target(Ok(None)) => {}
            ExportMessage::Target(Err(error)) => {
                export.error = Some(format!("Could not show the file dialog: {error}"));
            }
            ExportMessage::Done(result) => {
                self.notice = Some(match result {
                    Ok(done) => done,
                    Err(error) => format!("Could not export: {error}"),
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
            self.notice = Some("Export to a new file; this document saves itself.".into());
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
        self.notice = Some(format!("Exporting “{name}”…"));
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
                        .map(|()| format!("Exported “{name}”."))
                        .map_err(|error| error.to_string()),
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
                    rendered.push(flatten(display.await).map_err(|error| error.to_string())?);
                }
                spawn(move || export::write_images(&target, &rendered, format, dpi, quality))
                    .await
                    .unwrap_or_else(|_| Err("exporting stopped".into()))
            },
            move |result| {
                Message::Export(ExportMessage::Done(result.map(
                    |written| match written.len() {
                        1 => format!("Exported “{name}”."),
                        count => format!("Exported {count} images."),
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
        let check = |label: &'a str, checked: bool, message: fn(bool) -> ExportMessage| {
            iced::widget::checkbox(checked)
                .label(label)
                .on_toggle(move |value| send(message(value)))
                .style(style::checkbox)
        };
        let selected = viewer.target_pages().len();
        let pages_label = if selected == 1 {
            "Only the selected page".to_owned()
        } else {
            format!("Only the {selected} selected pages")
        };
        let mut body = column![
            ui::styled("Export", Type::HeadlineSmall),
            component::section("Format"),
            formats,
        ]
        .spacing(12)
        .width(440);
        if export.format == Format::Pdf {
            body = body.push(check(
                "Reduce file size (images at 150 dpi)",
                export.reduce,
                ExportMessage::Reduce,
            ));
            body = body.push(check(
                "Flatten annotations and form fields",
                export.flatten,
                ExportMessage::Flatten,
            ));
            if export.flatten {
                body = body.push(
                    ui::styled(
                        "Markup and filled-in fields become part of the pages and can no \
longer be edited. Redaction marks not yet applied are left out.",
                        Type::BodySmall,
                    )
                    .style(style::on_surface_variant),
                );
            }
            body = body.push(check(
                "Encrypt with a password",
                export.encrypt,
                ExportMessage::Encrypt,
            ));
            if export.encrypt {
                body = body.push(component::text_field(
                    "Password",
                    &export.password,
                    Backdrop::ContainerHigh,
                    |input| {
                        input
                            .secure(true)
                            .on_input(|value| Message::Export(ExportMessage::Password(value)))
                    },
                ));
                body = body.push(component::text_field(
                    "Verify password",
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
            body = body.push(component::section("Resolution"));
            body = body.push(component::connected(
                export::RESOLUTIONS
                    .iter()
                    .map(|dpi| {
                        ui::button(ButtonKind::Tonal, format!("{dpi:.0} dpi"))
                            .size(button::Size::ExtraSmall)
                            .selected(export.dpi == *dpi)
                            .on_press(send(ExportMessage::Dpi(*dpi)))
                    })
                    .collect(),
            ));
            if export.format == Format::Jpeg {
                body = body.push(component::section("Quality"));
                body = body.push(component::connected(
                    export::QUALITIES
                        .iter()
                        .map(|(label, quality)| {
                            ui::button(ButtonKind::Tonal, *label)
                                .size(button::Size::ExtraSmall)
                                .selected(export.quality == *quality)
                                .on_press(send(ExportMessage::Quality(*quality)))
                        })
                        .collect(),
                ));
            }
            let note = if export.format.is_multipage() {
                "All pages go into one file."
            } else {
                "Each page is saved as its own file, numbered after the name you choose."
            };
            body = body.push(ui::styled(note, Type::BodySmall).style(style::on_surface_variant));
        }
        body = body.push(
            iced::widget::checkbox(export.selected_only)
                .label(pages_label)
                .on_toggle(move |value| send(ExportMessage::SelectedOnly(value)))
                .style(style::checkbox),
        );
        if let Some(error) = &export.error {
            body = body.push(ui::styled(error.as_str(), Type::BodyMedium).style(style::error_text));
        }
        body = body.push(
            row![
                space::horizontal(),
                ui::button(ButtonKind::Text, "Cancel").on_press(send(ExportMessage::Cancel)),
                ui::button(ButtonKind::Filled, "Export…").on_press(send(ExportMessage::Choose)),
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
