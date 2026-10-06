//! The markup toolbar and its menus, text and form editors over the page,
//! the notes sidebar, and the signature library and its dialog.

use std::sync::Arc;

use crate::ui::dir::{column, row};
use crate::{column, row};
use iced::widget::image::Handle as ImageHandle;
use iced::widget::{container, image, pin, space, stack, text, text_editor, text_input};
use iced::{Center, Color, Element, Fill, Length, Task, Theme};
use prev_pdf::annotation::{Align, Annotation, FieldKind, Font, Kind, Rgb, TextMarkup};
use prev_pdf::engine::Bitmap;
use prev_store::signatures::{Signature, SignatureStore};

use super::{Bar, Message, PdfWindow, Sidebar, State};
use crate::dialog;
use crate::i18n::Describe;
use crate::image::editor::spawn;
use crate::pdf::layout;
use crate::pdf::markup::{Shape, Tool};
use crate::pdf::signature;
use crate::pdf::signature_pad::signature_pad;
use crate::pdf::viewer::PdfMessage;
use crate::pdf::viewer::PdfViewer;
use crate::pdf::viewer::editing::{EditMessage, StyleChange};
use crate::ui::button::{self, Kind as ButtonKind};
use crate::ui::component::{self, Backdrop};
use crate::ui::dropdown::{self, Entry, Segment, Swatch};
use crate::ui::{self, Icon, Scheme, Type, icon, shape, style};

/// Where the redaction tools are among the markup bar's slots.
const REDACT_SLOT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Pages,
    Shapes,
    Highlight,
    Signatures,
    LineStyle,
    BorderColor,
    FillColor,
    TextStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureTab {
    Draw,
    Type,
    Image,
}

#[derive(Debug, Clone)]
pub struct LoadedSignature {
    pub signature: Signature,
    pub bitmap: Arc<Bitmap>,
    pub handle: ImageHandle,
}

pub struct SignatureDialog {
    tab: SignatureTab,
    strokes: Vec<Vec<(f32, f32)>>,
    typed: String,
    /// An imported image, already turned into a transparent PNG.
    image: Option<(Vec<u8>, ImageHandle)>,
    description: String,
    error: Option<String>,
    saving: bool,
    /// Pen width in pad pixels, for drawn signatures.
    pen_width: f32,
    /// Ink for drawn and typed signatures.
    ink: [u8; 3],
    /// Labels the dialog lends to widgets that borrow their text.
    text: DialogText,
}

/// The dialog's text that widgets borrow rather than own, looked up when
/// the dialog opens.
struct DialogText {
    draw: String,
    type_: String,
    image: String,
    your_name: String,
    description: String,
}

impl DialogText {
    fn new() -> Self {
        Self {
            draw: crate::fl!("signature-tab-draw"),
            type_: crate::fl!("signature-tab-type"),
            image: crate::fl!("signature-tab-image"),
            your_name: crate::fl!("signature-your-name"),
            description: crate::fl!("signature-description"),
        }
    }
}

pub(super) const MARKUP_BAR_HEIGHT: f32 = 48.0;

const PAD: iced::Size = iced::Size::new(460.0, 170.0);

/// Preview's markup colors.
const SWATCHES: [(Rgb, &str); 10] = [
    (Rgb::new(0.0, 0.0, 0.0), "Black"),
    (Rgb::new(0.5, 0.5, 0.5), "Gray"),
    (Rgb::new(1.0, 1.0, 1.0), "White"),
    (Rgb::new(0.93, 0.16, 0.14), "Red"),
    (Rgb::new(1.0, 0.55, 0.0), "Orange"),
    (Rgb::new(1.0, 0.86, 0.2), "Yellow"),
    (Rgb::new(0.2, 0.7, 0.3), "Green"),
    (Rgb::new(0.1, 0.45, 0.95), "Blue"),
    (Rgb::new(0.55, 0.3, 0.85), "Purple"),
    (Rgb::new(1.0, 0.4, 0.7), "Pink"),
];

const HIGHLIGHT_COLORS: [(Rgb, &str); 5] = [
    (Rgb::new(1.0, 0.86, 0.2), "Yellow"),
    (Rgb::new(0.45, 0.9, 0.35), "Green"),
    (Rgb::new(0.4, 0.75, 1.0), "Blue"),
    (Rgb::new(1.0, 0.5, 0.75), "Pink"),
    (Rgb::new(0.75, 0.55, 1.0), "Purple"),
];

const LINE_WIDTHS: [f32; 6] = [0.5, 1.0, 2.0, 3.0, 5.0, 8.0];
const FONT_SIZES: [f32; 7] = [9.0, 10.0, 12.0, 14.0, 18.0, 24.0, 36.0];

fn to_color(rgb: Rgb) -> Color {
    Color::from_rgb(rgb.red, rgb.green, rgb.blue)
}

fn shape_icon(shape: Shape) -> Icon {
    match shape {
        Shape::Rectangle => Icon::Rectangle,
        Shape::RoundedRectangle => Icon::RoundedCorner,
        Shape::Oval => Icon::Circle,
        Shape::Line => Icon::HorizontalRule,
        Shape::Arrow => Icon::ArrowRightAlt,
        Shape::Star => Icon::Star,
        Shape::Polygon => Icon::Hexagon,
        Shape::SpeechBubble => Icon::ChatBubble,
        Shape::Loupe => Icon::Loupe,
        Shape::Mask => Icon::Vignette,
    }
}

fn markup_label(style: TextMarkup) -> (String, Icon) {
    let label = match style {
        TextMarkup::Highlight => crate::fl!("markup-style-highlight"),
        TextMarkup::Underline => crate::fl!("markup-style-underline"),
        TextMarkup::StrikeOut => crate::fl!("markup-style-strikethrough"),
        TextMarkup::Squiggly => crate::fl!("markup-style-squiggly"),
    };
    (label, markup_icon(style))
}

fn markup_icon(style: TextMarkup) -> Icon {
    match style {
        TextMarkup::Highlight => Icon::InkHighlighter,
        TextMarkup::Underline | TextMarkup::Squiggly => Icon::FormatUnderlined,
        TextMarkup::StrikeOut => Icon::FormatStrikethrough,
    }
}

/// A menu's color swatch; `None` stands for no color.
fn swatch(color: Option<Rgb>, selected: bool, message: Message) -> Swatch<Message> {
    Swatch {
        color: color.map(to_color),
        selected,
        message,
    }
}

fn edit(message: EditMessage) -> Message {
    Message::Edit(message)
}

impl PdfWindow {
    pub(super) fn markup_update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleMarkupBar => {
                self.markup_bar = !self.markup_bar;
                self.menu = None;
                if self.markup_bar {
                    let signatures = if self.signatures.is_empty() {
                        load_signatures()
                    } else {
                        Task::none()
                    };
                    // The bar offers to apply redaction marks.
                    return Task::batch([signatures, self.load_notes()]);
                }
                if !self.markup_bar {
                    return self
                        .viewer_update(PdfMessage::Editing(EditMessage::SetTool(Tool::Select)));
                }
                Task::none()
            }
            Message::Menu(menu) => {
                self.menu = if self.menu == menu { None } else { menu };
                if self.menu == Some(Menu::Signatures) {
                    return load_signatures();
                }
                Task::none()
            }
            Message::Edit(message) => {
                // Choosing from a menu closes it, except for styles, which
                // stay open to try a few.
                if matches!(
                    message,
                    EditMessage::SetTool(_) | EditMessage::Delete | EditMessage::PlaceStamp { .. }
                ) {
                    self.menu = None;
                    self.overflow = None;
                }
                self.viewer_update(PdfMessage::Editing(message))
            }
            Message::NotesLoaded(notes) => {
                self.notes = notes;
                Task::none()
            }
            Message::ShowAnnotation(page, id) => {
                let State::Ready(viewer) = &mut self.state else {
                    return Task::none();
                };
                let point = viewer.annotation(page, &id).map(|annotation| {
                    prev_pdf::geometry::Point::new(0.0, (annotation.rect.y0 - 72.0).max(0.0))
                });
                viewer.edit.selected = Some((page, id));
                self.viewer_update(PdfMessage::GoTo { page, point })
            }
            Message::SignaturesLoaded(signatures) => {
                self.signatures = signatures;
                Task::none()
            }
            Message::PlaceSignature(index) => {
                let Some(loaded) = self.signatures.get(index) else {
                    return Task::none();
                };
                let image = Arc::clone(&loaded.bitmap);
                self.menu = None;
                self.viewer_update(PdfMessage::Editing(EditMessage::PlaceStamp {
                    image,
                    // Saved in the file, so it stays in English.
                    subject: "Signature".into(),
                }))
            }
            Message::RemoveSignature(index) => {
                let Some(loaded) = self.signatures.get(index).cloned() else {
                    return Task::none();
                };
                Task::perform(
                    spawn(move || {
                        let store = SignatureStore::default_location()
                            .ok_or_else(|| crate::fl!("signature-no-data-folder"))?;
                        store
                            .remove(&loaded.signature)
                            .map_err(|error| error.to_string())
                    }),
                    |result| {
                        Message::SignatureSaved(
                            result
                                .unwrap_or_else(|_| Err(crate::fl!("signature-removing-stopped"))),
                        )
                    },
                )
            }
            Message::NewSignature => {
                self.menu = None;
                self.signature_dialog = Some(SignatureDialog {
                    tab: SignatureTab::Draw,
                    strokes: Vec::new(),
                    typed: String::new(),
                    image: None,
                    description: String::new(),
                    error: None,
                    saving: false,
                    pen_width: signature::PEN_WIDTH,
                    ink: signature::INK,
                    text: DialogText::new(),
                });
                Task::none()
            }
            Message::SignatureTab(tab) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.tab = tab;
                    dialog.error = None;
                }
                Task::none()
            }
            Message::SignatureStroke(stroke) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.strokes.push(stroke);
                }
                Task::none()
            }
            Message::ClearSignature => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    match dialog.tab {
                        SignatureTab::Draw => dialog.strokes.clear(),
                        SignatureTab::Type => dialog.typed.clear(),
                        SignatureTab::Image => dialog.image = None,
                    }
                }
                Task::none()
            }
            Message::SignatureText(text) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.typed = text;
                }
                Task::none()
            }
            Message::SignaturePenWidth(width) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.pen_width = width;
                }
                Task::none()
            }
            Message::SignatureInk(ink) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.ink = ink;
                }
                Task::none()
            }
            Message::SignatureDescription(text) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.description = text;
                }
                Task::none()
            }
            Message::ChooseSignatureImage => {
                Task::perform(dialog::open_files(), Message::SignatureImageChosen)
            }
            Message::SignatureImageChosen(result) => {
                match result {
                    Ok(paths) => {
                        let Some(path) = paths.into_iter().next() else {
                            return Task::none();
                        };
                        Task::perform(
                            spawn(move || {
                                let format = match crate::filetype::detect_path(&path) {
                                    Ok(Some(crate::filetype::FileKind::Image(format))) => format,
                                    _ => return Err(crate::fl!("signature-not-an-image")),
                                };
                                let decoded = prev_image::decode::decode_file(&path, format)
                                    .map_err(|error| error.describe())?;
                                let frame = decoded
                                    .frames
                                    .into_iter()
                                    .next()
                                    .ok_or_else(|| crate::fl!("signature-no-frames"))?;
                                let bitmap = Bitmap {
                                    width: frame.width,
                                    height: frame.height,
                                    pixels: frame.pixels.to_vec(),
                                };
                                signature::from_image(&bitmap)
                                    .ok_or_else(|| crate::fl!("signature-not-found"))
                            }),
                            |result| {
                                Message::SignatureImageMade(result.unwrap_or_else(|_| {
                                    Err(crate::fl!("signature-reading-stopped"))
                                }))
                            },
                        )
                    }
                    Err(error) => {
                        if let Some(dialog) = self.signature_dialog.as_mut() {
                            dialog.error = Some(error);
                        }
                        Task::none()
                    }
                }
            }
            Message::SignatureImageMade(result) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    match result {
                        Ok(png) => {
                            let handle = ImageHandle::from_bytes(png.clone());
                            dialog.image = Some((png, handle));
                            dialog.error = None;
                        }
                        Err(error) => dialog.error = Some(error),
                    }
                }
                Task::none()
            }
            Message::SaveSignature => {
                let Some(dialog) = self.signature_dialog.as_mut() else {
                    return Task::none();
                };
                let png = match dialog.tab {
                    SignatureTab::Draw => {
                        signature::from_strokes(&dialog.strokes, dialog.pen_width, dialog.ink)
                    }
                    SignatureTab::Type => signature::from_text(&dialog.typed, dialog.ink),
                    SignatureTab::Image => dialog.image.as_ref().map(|(png, _)| png.clone()),
                };
                let Some(png) = png else {
                    dialog.error = Some(crate::fl!("signature-sign-first"));
                    return Task::none();
                };
                let description = match dialog.description.trim() {
                    "" if dialog.tab == SignatureTab::Type => dialog.typed.trim().to_owned(),
                    "" => {
                        let number = self.signatures.len() + 1;
                        crate::fl!("signature-default-name", number = number)
                    }
                    description => description.to_owned(),
                };
                dialog.saving = true;
                Task::perform(
                    spawn(move || {
                        let store = SignatureStore::default_location()
                            .ok_or_else(|| crate::fl!("signature-no-data-folder"))?;
                        store
                            .add(&description, &png)
                            .map(|_| ())
                            .map_err(|error| error.to_string())
                    }),
                    |result| {
                        Message::SignatureSaved(
                            result.unwrap_or_else(|_| Err(crate::fl!("signature-saving-stopped"))),
                        )
                    },
                )
            }
            Message::SignatureSaved(result) => {
                match result {
                    Ok(()) => {
                        let placing = self.signature_dialog.take().is_some();
                        let reload = load_signatures();
                        if placing {
                            // Show the library with the new signature to place.
                            self.markup_bar = true;
                            self.menu = Some(Menu::Signatures);
                        }
                        return reload;
                    }
                    Err(error) => match self.signature_dialog.as_mut() {
                        Some(dialog) => {
                            dialog.saving = false;
                            dialog.error = Some(error);
                        }
                        None => {
                            self.notice = Some(crate::fl!("signature-change-failed", error = error))
                        }
                    },
                }
                Task::none()
            }
            Message::CancelSignature => {
                self.signature_dialog = None;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    /// Reloads the notes sidebar when it shows.
    pub(super) fn load_notes(&self) -> Task<Message> {
        let State::Ready(viewer) = &self.state else {
            return Task::none();
        };
        // Listing every page's annotations can be slow, so only while
        // something shows them: the notes sidebar, or the markup bar with
        // its count of redaction marks.
        if self.sidebar != Some(Sidebar::Notes) && !self.markup_bar {
            return Task::none();
        }
        let receiver = viewer.handle.all_annotations();
        Task::perform(receiver, |result| {
            Message::NotesLoaded(result.ok().and_then(Result::ok).unwrap_or_default())
        })
    }

    pub(super) fn markup_toolbar<'a>(&'a self, viewer: &'a PdfViewer) -> Element<'a, Message> {
        component::secondary_toolbar(
            iced::widget::responsive(move |size| self.markup_bar_at(viewer, size.width)),
            MARKUP_BAR_HEIGHT,
        )
    }

    /// The markup bar at `width`, with groups that do not fit in "More".
    fn markup_bar_at<'a>(&'a self, viewer: &'a PdfViewer, width: f32) -> Element<'a, Message> {
        let tool = viewer.edit.tool;
        let tool_button = |glyph: Icon, label: String, this: Tool| {
            component::toggle_tool(glyph, label, tool == this, edit(EditMessage::SetTool(this)))
        };
        let menu_button = |glyph: Icon,
                           label: String,
                           menu: Menu,
                           selected: bool,
                           entries: Vec<Entry<Message>>| {
            Element::from(
                dropdown::icon_menu(glyph, entries)
                    .size(button::Size::Small)
                    .selected(selected)
                    .tip(label)
                    // Signatures take the width they are given.
                    .menu_width(if menu == Menu::Signatures { 300.0 } else { 0.0 })
                    .open(self.menu == Some(menu), move |open| {
                        Message::Menu(open.then_some(menu))
                    }),
            )
        };
        let shape_tool = match tool {
            Tool::Shape(shape) => Some(shape),
            _ => None,
        };
        let shapes = Shape::ALL
            .iter()
            .map(|shape| {
                Entry::item(
                    shape.label(),
                    edit(EditMessage::SetTool(Tool::Shape(*shape))),
                )
                .icon(shape_icon(*shape))
                .checked(shape_tool == Some(*shape))
            })
            .collect();
        let highlight_tool = match tool {
            Tool::Highlight(style) => Some(style),
            _ => None,
        };
        let mut highlight: Vec<Entry<Message>> = [
            TextMarkup::Highlight,
            TextMarkup::Underline,
            TextMarkup::StrikeOut,
        ]
        .into_iter()
        .map(|style| {
            let (label, glyph) = markup_label(style);
            Entry::item(label, edit(EditMessage::SetTool(Tool::Highlight(style))))
                .icon(glyph)
                .checked(highlight_tool == Some(style))
        })
        .collect();
        highlight.push(Entry::heading(crate::fl!("markup-menu-color")));
        highlight.push(
            Entry::swatches(
                HIGHLIGHT_COLORS
                    .iter()
                    .map(|(color, _)| {
                        swatch(
                            Some(*color),
                            viewer.edit.markup_color == *color,
                            edit(EditMessage::MarkupColor(*color)),
                        )
                    })
                    .collect(),
                HIGHLIGHT_COLORS.len(),
            )
            .keep_open(),
        );

        let selected = viewer
            .selected_annotation()
            .map(|(_, annotation)| annotation);
        let style = selected.map_or(viewer.edit.style, |annotation| annotation.style);
        let text_style = selected
            .filter(|annotation| annotation.kind == Kind::FreeText)
            .map_or(viewer.edit.text_style, |annotation| annotation.style);
        // Styles stay open to try a few.
        let mut line_style: Vec<Entry<Message>> = LINE_WIDTHS
            .iter()
            .map(|width| {
                let width: f32 = *width;
                Entry::item(
                    crate::fl!("markup-line-width", width = width),
                    edit(EditMessage::Style(StyleChange::LineWidth(width))),
                )
                .icon(Icon::LineWeight)
                .checked(style.line_width == width)
                .keep_open()
            })
            .collect();
        line_style.push(
            Entry::item(
                crate::fl!("markup-dashed"),
                edit(EditMessage::Style(StyleChange::Dashed(!style.dashed))),
            )
            .icon(Icon::LineStyle)
            .checked(style.dashed)
            .keep_open(),
        );
        let colors = |current: Option<Rgb>, make: fn(Option<Rgb>) -> StyleChange| {
            let none = swatch(
                None,
                current.is_none(),
                edit(EditMessage::Style(make(None))),
            );
            let swatches = std::iter::once(none)
                .chain(SWATCHES.iter().map(|(color, _)| {
                    swatch(
                        Some(*color),
                        current == Some(*color),
                        edit(EditMessage::Style(make(Some(*color)))),
                    )
                }))
                .collect();
            vec![Entry::swatches(swatches, 6).keep_open()]
        };
        let border_colors = colors(style.color, StyleChange::Color);
        let fill_colors = colors(style.fill, StyleChange::Fill);
        // Font names, not translated.
        let fonts = [
            (Font::Helvetica, "Helvetica"),
            (Font::Times, "Times"),
            (Font::Courier, "Courier"),
        ];
        let mut text_menu = vec![Entry::heading(crate::fl!("markup-menu-font"))];
        text_menu.extend(fonts.into_iter().map(|(font, label)| {
            Entry::item(label, edit(EditMessage::Style(StyleChange::Font(font))))
                .checked(text_style.font == font)
                .keep_open()
        }));
        text_menu.push(Entry::heading(crate::fl!("markup-menu-size")));
        text_menu.push(
            Entry::segments(
                FONT_SIZES
                    .iter()
                    .map(|size| Segment {
                        icon: None,
                        label: format!("{size}"),
                        selected: text_style.font_size == *size,
                        message: edit(EditMessage::Style(StyleChange::FontSize(*size))),
                    })
                    .collect(),
            )
            .keep_open(),
        );
        text_menu.push(Entry::heading(crate::fl!("markup-menu-color")));
        text_menu.push(
            Entry::swatches(
                SWATCHES
                    .iter()
                    .take(8)
                    .map(|(color, _)| {
                        swatch(
                            Some(*color),
                            text_style.text_color == *color,
                            edit(EditMessage::Style(StyleChange::TextColor(*color))),
                        )
                    })
                    .collect(),
                8,
            )
            .keep_open(),
        );
        text_menu.push(Entry::heading(crate::fl!("markup-menu-alignment")));
        text_menu.push(
            Entry::segments(
                [
                    (Align::Left, Icon::FormatAlignLeft),
                    (Align::Center, Icon::FormatAlignCenter),
                    (Align::Right, Icon::FormatAlignRight),
                ]
                .into_iter()
                .map(|(align, glyph)| Segment {
                    icon: Some(glyph),
                    label: String::new(),
                    selected: text_style.align == align,
                    message: edit(EditMessage::Style(StyleChange::Align(align))),
                })
                .collect(),
            )
            .keep_open(),
        );

        let signatures = self.signature_menu();
        // The menus above read in the interface's direction; the bar does
        // not mirror.
        let _fixed = crate::ui::dir::fixed();
        let redactions = self.redaction_count();
        let history = &viewer.edit.history;
        use component::{DIVIDER_WIDTH, TOOL_WIDTH};
        let tools = |count: f32| count * TOOL_WIDTH + (count - 1.0) * 4.0;
        let group = |items: Vec<Element<'a, Message>>| -> Element<'a, Message> {
            row(items).spacing(4).align_y(Center).into()
        };
        let mut drawing_tools = vec![
            tool_button(
                Icon::Gesture,
                crate::fl!("markup-tool-sketch"),
                Tool::Sketch,
            ),
            tool_button(Icon::Draw, crate::fl!("markup-tool-draw"), Tool::Draw),
            menu_button(
                shape_tool.map_or(Icon::Shapes, shape_icon),
                crate::fl!("markup-tool-shapes"),
                Menu::Shapes,
                shape_tool.is_some(),
                shapes,
            ),
            tool_button(
                Icon::TextFields,
                crate::fl!("markup-tool-text-box"),
                Tool::TextBox,
            ),
        ];
        // Images have no text to highlight.
        if !self.image_mode {
            drawing_tools.push(menu_button(
                highlight_tool.map_or(Icon::InkHighlighter, markup_icon),
                crate::fl!("markup-tool-highlight"),
                Menu::Highlight,
                highlight_tool.is_some(),
                highlight,
            ));
        }
        drawing_tools.push(tool_button(
            Icon::StickyNote,
            crate::fl!("markup-tool-note"),
            Tool::Note,
        ));
        let drawing_tools_count = drawing_tools.len() as f32;
        // Each slot: its content, its width, when it moves into "More", and
        // whether a divider goes before it in the bar.
        let mut slots: Vec<(Element<'a, Message>, f32, Option<u8>, bool)> = vec![
            if self.image_mode {
                (
                    tool_button(
                        Icon::ArrowSelector,
                        crate::fl!("markup-tool-select"),
                        Tool::Select,
                    ),
                    tools(1.0),
                    None,
                    false,
                )
            } else {
                (
                    group(vec![
                        tool_button(
                            Icon::ArrowSelector,
                            crate::fl!("markup-tool-select"),
                            Tool::Select,
                        ),
                        tool_button(
                            Icon::HighlightAlt,
                            crate::fl!("markup-tool-area"),
                            Tool::Area,
                        ),
                    ]),
                    tools(2.0),
                    None,
                    false,
                )
            },
            (
                group(drawing_tools),
                tools(drawing_tools_count) + DIVIDER_WIDTH + 4.0,
                Some(1),
                true,
            ),
            (
                menu_button(
                    Icon::Signature,
                    crate::fl!("markup-tool-sign"),
                    Menu::Signatures,
                    false,
                    signatures,
                ),
                DIVIDER_WIDTH + TOOL_WIDTH + 4.0,
                Some(2),
                true,
            ),
            (
                group(if redactions > 0 {
                    vec![
                        tool_button(
                            Icon::RemoveSelection,
                            crate::fl!("markup-tool-redact"),
                            Tool::Redact,
                        ),
                        component::tip(
                            ui::button(ButtonKind::Tonal, crate::fl!("markup-apply"))
                                .size(button::Size::ExtraSmall)
                                .on_press(Message::ConfirmRedactions(true)),
                            crate::fl!("markup-apply-redactions"),
                        ),
                    ]
                } else {
                    vec![tool_button(
                        Icon::RemoveSelection,
                        crate::fl!("markup-tool-redact"),
                        Tool::Redact,
                    )]
                }),
                DIVIDER_WIDTH + TOOL_WIDTH + 4.0 + if redactions > 0 { 68.0 } else { 0.0 },
                Some(2),
                true,
            ),
            (
                group(vec![
                    menu_button(
                        Icon::LineWeight,
                        crate::fl!("markup-shape-style"),
                        Menu::LineStyle,
                        false,
                        line_style,
                    ),
                    menu_button(
                        Icon::BorderColor,
                        crate::fl!("markup-border-color"),
                        Menu::BorderColor,
                        false,
                        border_colors,
                    ),
                    menu_button(
                        Icon::FormatColorFill,
                        crate::fl!("markup-fill-color"),
                        Menu::FillColor,
                        false,
                        fill_colors,
                    ),
                    menu_button(
                        Icon::TextFormat,
                        crate::fl!("markup-text-style"),
                        Menu::TextStyle,
                        false,
                        text_menu,
                    ),
                ]),
                DIVIDER_WIDTH + tools(4.0) + 4.0,
                Some(0),
                true,
            ),
            (
                group(vec![
                    component::tool(
                        Icon::Delete,
                        crate::fl!("markup-delete"),
                        viewer
                            .edit
                            .selected
                            .is_some()
                            .then(|| edit(EditMessage::Delete)),
                    ),
                    component::toolbar_divider(),
                    component::tool(
                        Icon::Undo,
                        crate::fl!("markup-undo"),
                        history.can_undo().then(|| edit(EditMessage::Undo)),
                    ),
                    component::tool(
                        Icon::Redo,
                        crate::fl!("markup-redo"),
                        history.can_redo().then(|| edit(EditMessage::Redo)),
                    ),
                ]),
                tools(3.0) + DIVIDER_WIDTH + 4.0,
                None,
                false,
            ),
        ];
        // An image's markup is burned in on export; redacting means
        // drawing a filled box.
        if self.image_mode {
            slots.remove(REDACT_SLOT);
        }
        let widths: Vec<(f32, Option<u8>)> = slots
            .iter()
            .map(|(_, width, order, _)| (*width, *order))
            .collect();
        let shown = component::fitting_slots(width, &widths);
        let last = slots.len() - 1;
        let mut bar = crate::line![].spacing(4).align_y(Center);
        let mut hidden = Vec::new();
        for (index, ((element, _, _, divider), shown)) in slots.into_iter().zip(shown).enumerate() {
            // Delete, undo and redo sit on the right.
            if index == last {
                bar = bar.push(space::horizontal());
            }
            if shown {
                if divider {
                    bar = bar.push(component::toolbar_divider());
                }
                bar = bar.push(element);
            } else {
                hidden.push(element);
            }
        }
        if !hidden.is_empty() {
            let open = self.overflow == Some(Bar::Markup);
            bar = bar.push(component::overflow(
                hidden,
                open,
                Message::Overflow((!open).then_some(Bar::Markup)),
                Message::Overflow(None),
            ));
        }
        container(bar).height(Fill).align_y(Center).into()
    }

    fn signature_menu(&self) -> Vec<Entry<Message>> {
        let mut entries = Vec::new();
        if self.signatures.is_empty() {
            entries.push(Entry::note(crate::fl!("signature-menu-empty")));
        }
        for (index, loaded) in self.signatures.iter().enumerate() {
            entries.push(
                Entry::picture(
                    loaded.handle.clone(),
                    &loaded.signature.description,
                    Message::PlaceSignature(index),
                )
                .action(
                    Icon::Delete,
                    crate::fl!("signature-delete"),
                    Message::RemoveSignature(index),
                ),
            );
        }
        entries.push(Entry::divider());
        entries.push(
            Entry::item(crate::fl!("signature-create"), Message::NewSignature).icon(Icon::Add),
        );
        entries
    }

    /// Editors floating over the page: a text box or note being typed, a
    /// form field being filled in, or a drop-down's choices.
    pub(super) fn page_editors<'a>(
        &'a self,
        viewer: &'a PdfViewer,
    ) -> Option<Element<'a, Message>> {
        let scale = layout::points_to_pixels(viewer.layout.zoom);
        let to_view = |page: usize, point: prev_pdf::geometry::Point| {
            let (x, y) = viewer.layout.to_document(page, point)?;
            Some((x - viewer.view.x, y - viewer.view.y))
        };
        let placed = |element: Element<'a, Message>, (x, y): (f32, f32)| -> Element<'a, Message> {
            pin(element)
                .x(x.max(0.0))
                .y(y.max(0.0))
                .width(Fill)
                .height(Fill)
                .into()
        };
        if let Some(edit) = &viewer.edit.text {
            let annotation = &edit.annotation;
            let origin = to_view(
                edit.page,
                prev_pdf::geometry::Point::new(annotation.rect.x0, annotation.rect.y0),
            )?;
            let editor = text_editor(&edit.content)
                .id(self.text_editor_id.clone())
                .on_action(|action| Message::Edit(EditMessage::TextAction(action)));
            let element: Element<'a, Message> = if annotation.kind == Kind::FreeText {
                let font = match annotation.style.font {
                    Font::Helvetica => iced::Font::DEFAULT,
                    Font::Times => iced::Font {
                        family: iced::font::Family::Serif,
                        ..iced::Font::DEFAULT
                    },
                    Font::Courier => iced::Font::MONOSPACE,
                };
                let text_color = to_color(annotation.style.text_color);
                container(
                    editor
                        .font(font)
                        .size(annotation.style.font_size * scale)
                        .padding(2.0 * scale)
                        .height(Length::Shrink)
                        .style(move |theme: &Theme, _status| {
                            let scheme = Scheme::of(theme);
                            iced::widget::text_editor::Style {
                                background: Color::WHITE.into(),
                                border: iced::Border {
                                    color: scheme.primary,
                                    width: 2.0,
                                    radius: 2.0.into(),
                                },
                                placeholder: scheme.outline,
                                value: text_color,
                                selection: crate::ui::faded(scheme.primary, 0.3),
                            }
                        }),
                )
                .width((annotation.rect.width() * scale).max(120.0))
                .into()
            } else {
                note_card(editor, edit.page, annotation)
            };
            let position = if annotation.kind == Kind::Note {
                (origin.0 + annotation.rect.width() * scale + 8.0, origin.1)
            } else {
                origin
            };
            return Some(placed(element, position));
        }
        if let Some(edit) = &viewer.edit.field {
            let field = &edit.field;
            let origin = to_view(
                edit.page,
                prev_pdf::geometry::Point::new(field.rect.x0, field.rect.y0),
            )?;
            let size = if field.font_size > 0.0 {
                field.font_size
            } else {
                (field.rect.height() * 0.65).clamp(8.0, 14.0)
            };
            let secure = matches!(field.kind, FieldKind::Text { password: true, .. });
            let input = text_input("", &edit.value)
                .id(self.field_input_id.clone())
                .on_input(|value| Message::Edit(EditMessage::FieldInput(value)))
                .on_submit(Message::Edit(EditMessage::CommitField))
                .secure(secure)
                .size(size * scale)
                .padding([0.0, 2.0 * scale])
                .width(field.rect.width() * scale)
                .style(|theme: &Theme, _| {
                    let scheme = Scheme::of(theme);
                    iced::widget::text_input::Style {
                        background: Color::WHITE.into(),
                        border: iced::Border {
                            color: scheme.primary,
                            width: 2.0,
                            radius: 2.0.into(),
                        },
                        icon: scheme.on_surface_variant,
                        placeholder: scheme.outline,
                        value: Color::BLACK,
                        selection: crate::ui::faded(scheme.primary, 0.3),
                    }
                });
            let input = container(input)
                .height(field.rect.height() * scale)
                .align_y(Center);
            return Some(placed(input.into(), origin));
        }
        if let Some((page, field)) = &viewer.edit.choice {
            let FieldKind::Choice { options, .. } = &field.kind else {
                return None;
            };
            let origin = to_view(
                *page,
                prev_pdf::geometry::Point::new(field.rect.x0, field.rect.y1),
            )?;
            let menu = dropdown::surface(
                options
                    .iter()
                    .map(|option| {
                        Entry::item(
                            option.as_str(),
                            Message::Edit(EditMessage::Choose(option.clone())),
                        )
                        .checked(field.value == *option)
                    })
                    .collect(),
                (field.rect.width() * scale).max(160.0),
            );
            return Some(placed(menu, (origin.0, origin.1 + 2.0)));
        }
        None
    }

    pub(super) fn notes_view<'a>(&'a self, viewer: &'a PdfViewer) -> Element<'a, Message> {
        let entries: Vec<Element<'a, Message>> = self
            .notes
            .iter()
            .flat_map(|(page, annotations)| {
                annotations
                    .iter()
                    .filter(|annotation| {
                        annotation.kind.is_editable()
                            && (!annotation.contents.trim().is_empty()
                                || matches!(annotation.kind, Kind::Markup { .. } | Kind::Note))
                    })
                    .map(move |annotation| note_entry(viewer, *page, annotation))
            })
            .collect();
        if entries.is_empty() {
            return component::empty_state(
                Icon::StickyNote,
                crate::fl!("markup-notes-empty"),
                crate::fl!("markup-notes-empty-hint"),
            );
        }
        component::scroll(column(entries).spacing(4).padding(12).width(Fill))
            .height(Fill)
            .into()
    }

    pub(super) fn signature_dialog_view<'a>(
        &'a self,
        base: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let Some(dialog) = &self.signature_dialog else {
            return base;
        };
        let tab = |label: &'a str, which: SignatureTab| component::Tab {
            label,
            icon: None,
            selected: dialog.tab == which,
            on_press: Message::SignatureTab(which),
        };
        let surface: Element<'a, Message> = match dialog.tab {
            SignatureTab::Draw => column![
                signature_pad(&dialog.strokes, PAD, Message::SignatureStroke)
                    .scale(self.device_scale)
                    .pen(dialog.pen_width, dialog.ink),
                ui::aligned(
                    ui::styled(crate::fl!("signature-draw-hint"), Type::BodySmall)
                        .style(style::on_surface_variant)
                ),
                pen_controls(dialog, true),
            ]
            .spacing(8)
            .into(),
            SignatureTab::Type => column![
                component::text_field(
                    &dialog.text.your_name,
                    &dialog.typed,
                    Backdrop::ContainerHigh,
                    |input| { input.on_input(Message::SignatureText) }
                ),
                container(
                    text(if dialog.typed.is_empty() {
                        dialog.text.your_name.clone()
                    } else {
                        dialog.typed.clone()
                    })
                    .font(crate::pdf::signature::FONT)
                    .size(48)
                    .color(if dialog.typed.is_empty() {
                        Color::from_rgb(0.7, 0.7, 0.7)
                    } else {
                        Color::from_rgb8(dialog.ink[0], dialog.ink[1], dialog.ink[2])
                    })
                    .wrapping(text::Wrapping::None)
                )
                .center_x(Fill)
                .center_y(110)
                .clip(true)
                .style(|_| iced::widget::container::Style {
                    background: Some(Color::WHITE.into()),
                    border: iced::border::rounded(shape::MEDIUM),
                    ..Default::default()
                }),
                pen_controls(dialog, false),
            ]
            .spacing(8)
            .width(PAD.width)
            .into(),
            SignatureTab::Image => {
                let preview: Element<'a, Message> = match &dialog.image {
                    Some((_, handle)) => image(handle.clone())
                        .content_fit(iced::ContentFit::Contain)
                        .height(120)
                        .into(),
                    None => ui::aligned(
                        ui::styled(crate::fl!("signature-image-hint"), Type::BodyMedium).style(
                            |_| iced::widget::text::Style {
                                color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                            },
                        ),
                    ),
                };
                column![
                    container(preview)
                        .center_x(PAD.width)
                        .center_y(PAD.height)
                        .style(|_| iced::widget::container::Style {
                            background: Some(Color::WHITE.into()),
                            border: iced::border::rounded(shape::MEDIUM),
                            ..Default::default()
                        }),
                    ui::with_icon(
                        ButtonKind::Tonal,
                        Icon::Upload,
                        crate::fl!("signature-choose-image"),
                    )
                    .on_press(Message::ChooseSignatureImage),
                ]
                .spacing(8)
                .into()
            }
        };
        let mut body = column![
            ui::styled(crate::fl!("signature-dialog-title"), Type::HeadlineSmall),
            component::tabs(vec![
                tab(&dialog.text.draw, SignatureTab::Draw),
                tab(&dialog.text.type_, SignatureTab::Type),
                tab(&dialog.text.image, SignatureTab::Image),
            ]),
            surface,
            component::text_field(
                &dialog.text.description,
                &dialog.description,
                Backdrop::ContainerHigh,
                |input| input.on_input(Message::SignatureDescription)
            ),
        ]
        .spacing(16)
        .width(PAD.width);
        if let Some(error) = &dialog.error {
            body = body.push(ui::aligned(
                ui::styled(error.as_str(), Type::BodyMedium).style(style::error_text),
            ));
        }
        body = body.push(
            row![
                ui::button(ButtonKind::Text, crate::fl!("signature-clear"))
                    .on_press(Message::ClearSignature),
                space::horizontal(),
                ui::button(ButtonKind::Text, crate::fl!("common-cancel"))
                    .on_press(Message::CancelSignature),
                ui::button(ButtonKind::Filled, crate::fl!("common-save"))
                    .on_press_maybe((!dialog.saving).then_some(Message::SaveSignature)),
            ]
            .spacing(8),
        );
        let card = container(body).padding(24).style(style::dialog);
        stack![
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

/// Ink swatches and, for drawing, a pen width slider.
fn pen_controls<'a>(dialog: &SignatureDialog, width: bool) -> Element<'a, Message> {
    let inks = row(signature::INKS.iter().map(|(ink, _)| {
        component::swatch(
            Some(Color::from_rgb8(ink[0], ink[1], ink[2])),
            dialog.ink == *ink,
            Message::SignatureInk(*ink),
        )
    }))
    .spacing(2)
    .align_y(Center);
    let mut controls = crate::line![
        ui::styled(crate::fl!("signature-ink"), Type::LabelLarge).style(style::on_surface_variant),
        inks
    ]
    .spacing(8)
    .align_y(Center);
    if width {
        let backdrop = |theme: &Theme, status| {
            style::slider(Backdrop::ContainerHigh.color(&Scheme::of(theme)))(theme, status)
        };
        controls = controls.push(space::horizontal()).push(
            row![
                ui::styled(crate::fl!("signature-thickness"), Type::LabelLarge)
                    .style(style::on_surface_variant),
                iced::widget::slider(
                    signature::PEN_WIDTHS,
                    dialog.pen_width,
                    Message::SignaturePenWidth
                )
                .step(0.5_f32)
                .width(120)
                .height(style::SLIDER_HEIGHT)
                .style(backdrop),
            ]
            .spacing(8)
            .align_y(Center),
        );
    }
    controls.width(PAD.width).into()
}

/// Ink on a note: the note is always yellow, so its icons use a fixed dark
/// color rather than the theme's, which is faint on yellow in dark mode.
const NOTE_INK: Color = Color::from_rgb(0.24, 0.22, 0.1);

/// A small icon button on a note.
fn note_button<'a>(glyph: Icon, message: Message) -> Element<'a, Message> {
    button::custom(
        ButtonKind::Standard,
        icon::icon(glyph, 20).style(|_| text::Style {
            color: Some(NOTE_INK),
        }),
    )
    .size(button::Size::ExtraSmall)
    .on_press(message)
    .into()
}

fn note_card<'a>(
    editor: iced::widget::TextEditor<'a, iced::advanced::text::highlighter::PlainText, Message>,
    page: usize,
    annotation: &'a Annotation,
) -> Element<'a, Message> {
    let _ = page;
    let yellow = Color::from_rgb(1.0, 0.96, 0.72);
    container(
        column![
            row![
                icon::icon(Icon::StickyNote, 18).style(|_| text::Style {
                    color: Some(NOTE_INK)
                }),
                ui::aligned(ui::styled(
                    annotation
                        .author
                        .clone()
                        .unwrap_or_else(|| crate::fl!("markup-kind-note")),
                    Type::LabelLarge
                )),
                component::tip(
                    note_button(Icon::Delete, edit(EditMessage::Delete)),
                    crate::fl!("markup-note-delete")
                ),
                component::tip(
                    note_button(Icon::Check, edit(EditMessage::CommitText)),
                    crate::fl!("markup-note-done")
                ),
            ]
            .spacing(6)
            .align_y(Center),
            editor
                .height(140)
                .size(14)
                .placeholder(crate::fl!("markup-note-placeholder"))
                .style(move |theme: &Theme, _status| {
                    let scheme = Scheme::of(theme);
                    iced::widget::text_editor::Style {
                        background: Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                        placeholder: Color::from_rgb(0.5, 0.5, 0.4),
                        value: Color::from_rgb(0.1, 0.1, 0.1),
                        selection: crate::ui::faded(scheme.primary, 0.3),
                    }
                }),
        ]
        .spacing(4),
    )
    .padding(crate::ui::dir::padding(6.0, 6.0, 10.0, 12.0))
    .width(260)
    .style(move |theme: &Theme| {
        let scheme = Scheme::of(theme);
        iced::widget::container::Style {
            background: Some(yellow.into()),
            text_color: Some(Color::from_rgb(0.15, 0.15, 0.1)),
            border: iced::border::rounded(shape::MEDIUM),
            shadow: crate::ui::elevation::shadow(&scheme, 3),
            ..Default::default()
        }
    })
    .into()
}

fn note_entry<'a>(
    viewer: &'a PdfViewer,
    page: usize,
    annotation: &'a Annotation,
) -> Element<'a, Message> {
    let (glyph, fallback) = match &annotation.kind {
        Kind::Markup { style, .. } => {
            let (label, glyph) = markup_label(*style);
            (glyph, label)
        }
        Kind::Note => (Icon::StickyNote, crate::fl!("markup-kind-note")),
        Kind::FreeText => (Icon::TextFields, crate::fl!("markup-kind-text-box")),
        Kind::Stamp => (Icon::Signature, crate::fl!("markup-kind-stamp")),
        Kind::Redact => (Icon::RemoveSelection, crate::fl!("markup-kind-redaction")),
        _ => (Icon::Shapes, crate::fl!("markup-kind-shape")),
    };
    let swatch_color = annotation.style.color.map(to_color);
    let marker = container(icon::icon(glyph, 18)).style(move |theme: &Theme| {
        iced::widget::container::Style {
            text_color: Some(swatch_color.unwrap_or(Scheme::of(theme).on_surface_variant)),
            ..Default::default()
        }
    });
    // Rows have a fixed height, so long text is cut to about two lines.
    const LONGEST: usize = 64;
    let body = match annotation
        .contents
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
    {
        text if text.is_empty() => fallback,
        text if text.chars().count() > LONGEST => {
            format!(
                "{}…",
                text.chars().take(LONGEST).collect::<String>().trim_end()
            )
        }
        text => text,
    };
    let selected = viewer
        .edit
        .selected
        .as_ref()
        .is_some_and(|(selected_page, id)| *selected_page == page && *id == annotation.id);
    button::custom(
        ButtonKind::Row,
        row![
            marker,
            column![
                ui::styled(
                    crate::fl!("markup-notes-page", page = viewer.page_label(page)),
                    Type::LabelSmall
                )
                .style(style::on_surface_variant),
                ui::aligned(ui::styled(body, Type::BodyMedium)),
            ]
            .spacing(2)
            .width(Fill),
        ]
        .spacing(12)
        .align_y(Center)
        .padding([8, 0]),
    )
    .selected(selected)
    .shape(button::Shape::Square)
    .height(64.0)
    .width(Fill)
    .on_press(Message::ShowAnnotation(page, annotation.id.clone()))
    .into()
}

fn load_signatures() -> Task<Message> {
    Task::perform(
        spawn(|| {
            let Some(store) = SignatureStore::default_location() else {
                return Vec::new();
            };
            store
                .list()
                .into_iter()
                .filter_map(|signature| {
                    let png = store.read(&signature).ok()?;
                    let bitmap = signature::decode(&png)?;
                    Some(LoadedSignature {
                        handle: ImageHandle::from_bytes(png),
                        bitmap: Arc::new(bitmap),
                        signature,
                    })
                })
                .collect()
        }),
        |result| Message::SignaturesLoaded(result.unwrap_or_default()),
    )
}
