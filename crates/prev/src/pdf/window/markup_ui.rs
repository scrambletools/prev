//! The markup toolbar and its menus, text and form editors over the page,
//! the notes sidebar, and the signature library and its dialog.

use std::sync::Arc;

use iced::widget::image::Handle as ImageHandle;
use iced::widget::{
    column, container, image, pin, row, space, stack, text, text_editor, text_input,
};
use iced::{Center, Color, Element, Fill, Length, Padding, Task, Theme};
use prev_pdf::annotation::{Align, Annotation, FieldKind, Font, Kind, Rgb, TextMarkup};
use prev_pdf::engine::Bitmap;
use prev_store::signatures::{Signature, SignatureStore};

use super::{Bar, Message, PdfWindow, Sidebar, State};
use crate::dialog;
use crate::image::editor::spawn;
use crate::pdf::layout;
use crate::pdf::markup::{Shape, Tool};
use crate::pdf::signature;
use crate::pdf::signature_pad::{self, signature_pad};
use crate::pdf::viewer::PdfMessage;
use crate::pdf::viewer::PdfViewer;
use crate::pdf::viewer::editing::{EditMessage, StyleChange};
use crate::ui::button::{self, Kind as ButtonKind};
use crate::ui::component::{self, Backdrop};
use crate::ui::popover::{self, popover};
use crate::ui::{self, Icon, Scheme, Type, icon, shape, style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
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
}

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

fn markup_label(style: TextMarkup) -> (&'static str, Icon) {
    match style {
        TextMarkup::Highlight => ("Highlight", Icon::InkHighlighter),
        TextMarkup::Underline => ("Underline", Icon::FormatUnderlined),
        TextMarkup::StrikeOut => ("Strikethrough", Icon::FormatStrikethrough),
        TextMarkup::Squiggly => ("Squiggly", Icon::FormatUnderlined),
    }
}

/// A round color button; `None` shows "no color" as a slashed outline.
fn swatch<'a>(color: Option<Rgb>, selected: bool, message: Message) -> Element<'a, Message> {
    let dot = container(space().width(20).height(20)).style(move |theme: &Theme| {
        let scheme = Scheme::of(theme);
        iced::widget::container::Style {
            background: color.map(|color| to_color(color).into()),
            border: iced::Border {
                color: if selected {
                    scheme.primary
                } else {
                    scheme.outline_variant
                },
                width: if selected { 3.0 } else { 1.0 },
                radius: shape::FULL.into(),
            },
            ..Default::default()
        }
    });
    let content: Element<'a, Message> = match color {
        Some(_) => dot.into(),
        None => stack![dot, container(icon::icon(Icon::Close, 16)).center(20)].into(),
    };
    button::custom(ButtonKind::Standard, content)
        .size(button::Size::ExtraSmall)
        .width(32)
        .on_press(message)
        .into()
}

fn menu_item<'a>(
    glyph: Option<Icon>,
    label: impl text::IntoFragment<'a>,
    selected: bool,
    message: Message,
) -> Element<'a, Message> {
    component::list_row(glyph, label, 0.0, selected, Some(message)).into()
}

fn menu_section<'a>(label: &'a str) -> Element<'a, Message> {
    container(ui::styled(label, Type::LabelMedium).style(style::on_surface_variant))
        .padding(Padding {
            top: 8.0,
            right: 16.0,
            bottom: 4.0,
            left: 16.0,
        })
        .into()
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
                if self.markup_bar && self.signatures.is_empty() {
                    return load_signatures();
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
                            .ok_or("no data folder: HOME is not set")?;
                        store
                            .remove(&loaded.signature)
                            .map_err(|error| error.to_string())
                    }),
                    |result| {
                        Message::SignatureSaved(
                            result.unwrap_or_else(|_| Err("removing stopped".into())),
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
            Message::SignatureDescription(text) => {
                if let Some(dialog) = self.signature_dialog.as_mut() {
                    dialog.description = text;
                }
                Task::none()
            }
            Message::ChooseSignatureImage => {
                Task::perform(dialog::open_files(), Message::SignatureImageChosen)
            }
            Message::SignatureImageChosen(result) => match result {
                Ok(paths) => {
                    let Some(path) = paths.into_iter().next() else {
                        return Task::none();
                    };
                    Task::perform(
                        spawn(move || {
                            let format = match crate::filetype::detect_path(&path) {
                                Ok(Some(crate::filetype::FileKind::Image(format))) => format,
                                _ => return Err("that file is not an image prev can read".into()),
                            };
                            let decoded = prev_image::decode::decode_file(&path, format)
                                .map_err(|error| error.to_string())?;
                            let frame = decoded
                                .frames
                                .into_iter()
                                .next()
                                .ok_or("the image has no frames")?;
                            let bitmap = Bitmap {
                                width: frame.width,
                                height: frame.height,
                                pixels: frame.pixels.to_vec(),
                            };
                            signature::from_image(&bitmap)
                                .ok_or_else(|| "no signature found in the image".to_owned())
                        }),
                        |result| {
                            Message::SignatureImageMade(
                                result.unwrap_or_else(|_| Err("reading stopped".into())),
                            )
                        },
                    )
                }
                Err(error) => {
                    if let Some(dialog) = self.signature_dialog.as_mut() {
                        dialog.error = Some(error);
                    }
                    Task::none()
                }
            },
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
                        signature::from_strokes(&dialog.strokes, signature_pad::LINE_WIDTH)
                    }
                    SignatureTab::Type => signature::from_text(&dialog.typed),
                    SignatureTab::Image => dialog.image.as_ref().map(|(png, _)| png.clone()),
                };
                let Some(png) = png else {
                    dialog.error = Some("Sign first, then save.".into());
                    return Task::none();
                };
                let description = match dialog.description.trim() {
                    "" if dialog.tab == SignatureTab::Type => dialog.typed.trim().to_owned(),
                    "" => format!("Signature {}", self.signatures.len() + 1),
                    description => description.to_owned(),
                };
                dialog.saving = true;
                Task::perform(
                    spawn(move || {
                        let store = SignatureStore::default_location()
                            .ok_or("no data folder: HOME is not set")?;
                        store
                            .add(&description, &png)
                            .map(|_| ())
                            .map_err(|error| error.to_string())
                    }),
                    |result| {
                        Message::SignatureSaved(
                            result.unwrap_or_else(|_| Err("saving stopped".into())),
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
                        None => self.notice = Some(format!("Could not change signatures: {error}")),
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
        if self.sidebar != Some(Sidebar::Notes) {
            return Task::none();
        }
        let receiver = viewer.handle.all_annotations();
        Task::perform(receiver, |result| {
            Message::NotesLoaded(result.ok().and_then(Result::ok).unwrap_or_default())
        })
    }

    pub(super) fn markup_toolbar<'a>(&'a self, viewer: &'a PdfViewer) -> Element<'a, Message> {
        container(iced::widget::responsive(move |size| {
            self.markup_bar_at(viewer, size.width)
        }))
        .padding([0, 8])
        .height(48)
        .width(Fill)
        .style(style::surface_container_low)
        .into()
    }

    /// The markup bar at `width`, with groups that do not fit in "More".
    fn markup_bar_at<'a>(&'a self, viewer: &'a PdfViewer, width: f32) -> Element<'a, Message> {
        let tool = viewer.edit.tool;
        let tool_button = |glyph: Icon, label: &'a str, this: Tool| {
            component::toggle_tool(glyph, label, tool == this, edit(EditMessage::SetTool(this)))
        };
        let menu_button = |glyph: Icon,
                           label: &'a str,
                           menu: Menu,
                           selected: bool,
                           content: Element<'a, Message>| {
            let open = self.menu == Some(menu);
            let anchor = component::tip(
                ui::icon_button(glyph)
                    .selected(selected || open)
                    .on_press(Message::Menu(Some(menu))),
                label,
            );
            Element::from(popover(
                anchor,
                open.then(|| popover::surface(content)),
                Message::Menu(None),
            ))
        };
        let shape_tool = match tool {
            Tool::Shape(shape) => Some(shape),
            _ => None,
        };
        let shapes = column(Shape::ALL.iter().map(|shape| {
            menu_item(
                Some(shape_icon(*shape)),
                shape.label(),
                shape_tool == Some(*shape),
                edit(EditMessage::SetTool(Tool::Shape(*shape))),
            )
        }))
        .width(240)
        .padding([0, 8]);
        let highlight_tool = match tool {
            Tool::Highlight(style) => Some(style),
            _ => None,
        };
        let highlight = column![
            column(
                [
                    TextMarkup::Highlight,
                    TextMarkup::Underline,
                    TextMarkup::StrikeOut
                ]
                .into_iter()
                .map(|style| {
                    let (label, glyph) = markup_label(style);
                    menu_item(
                        Some(glyph),
                        label,
                        highlight_tool == Some(style),
                        edit(EditMessage::SetTool(Tool::Highlight(style))),
                    )
                })
            ),
            menu_section("Color"),
            row(HIGHLIGHT_COLORS.iter().map(|(color, _)| {
                swatch(
                    Some(*color),
                    viewer.edit.markup_color == *color,
                    edit(EditMessage::MarkupColor(*color)),
                )
            }))
            .spacing(2)
            .padding([0, 12]),
        ]
        .width(240)
        .padding([0, 8]);

        let selected = viewer
            .selected_annotation()
            .map(|(_, annotation)| annotation);
        let style = selected.map_or(viewer.edit.style, |annotation| annotation.style);
        let text_style = selected
            .filter(|annotation| annotation.kind == Kind::FreeText)
            .map_or(viewer.edit.text_style, |annotation| annotation.style);
        let line_style = column![
            column(LINE_WIDTHS.iter().map(|width| {
                menu_item(
                    Some(Icon::LineWeight),
                    format!("{width} pt"),
                    style.line_width == *width,
                    edit(EditMessage::Style(StyleChange::LineWidth(*width))),
                )
            })),
            menu_item(
                Some(Icon::LineStyle),
                "Dashed",
                style.dashed,
                edit(EditMessage::Style(StyleChange::Dashed(!style.dashed))),
            ),
        ]
        .width(200)
        .padding([0, 8]);
        let colors = |current: Option<Rgb>, make: fn(Option<Rgb>) -> StyleChange| {
            let mut swatches: Vec<Element<'a, Message>> = vec![swatch(
                None,
                current.is_none(),
                edit(EditMessage::Style(make(None))),
            )];
            swatches.extend(SWATCHES.iter().map(|(color, _)| {
                swatch(
                    Some(*color),
                    current == Some(*color),
                    edit(EditMessage::Style(make(Some(*color)))),
                )
            }));
            let mut grid = column![].spacing(2).padding([4, 12]);
            let mut swatches = swatches.into_iter().peekable();
            while swatches.peek().is_some() {
                grid = grid.push(row(swatches.by_ref().take(6)).spacing(2));
            }
            Element::from(grid)
        };
        let border_colors = colors(style.color, StyleChange::Color);
        let fill_colors = colors(style.fill, StyleChange::Fill);
        let fonts = [
            (Font::Helvetica, "Helvetica"),
            (Font::Times, "Times"),
            (Font::Courier, "Courier"),
        ];
        let text_menu = column![
            menu_section("Font"),
            column(fonts.into_iter().map(|(font, label)| {
                menu_item(
                    None,
                    label,
                    text_style.font == font,
                    edit(EditMessage::Style(StyleChange::Font(font))),
                )
            })),
            menu_section("Size"),
            container(component::connected(
                FONT_SIZES
                    .iter()
                    .map(|size| {
                        ui::button(ButtonKind::Tonal, format!("{size}"))
                            .size(button::Size::ExtraSmall)
                            .selected(text_style.font_size == *size)
                            .on_press(edit(EditMessage::Style(StyleChange::FontSize(*size))))
                    })
                    .collect()
            ))
            .padding([0, 12]),
            menu_section("Color"),
            row(SWATCHES.iter().take(8).map(|(color, _)| {
                swatch(
                    Some(*color),
                    text_style.text_color == *color,
                    edit(EditMessage::Style(StyleChange::TextColor(*color))),
                )
            }))
            .spacing(2)
            .padding([0, 12]),
            menu_section("Alignment"),
            container(component::connected(
                [
                    (Align::Left, Icon::FormatAlignLeft),
                    (Align::Center, Icon::FormatAlignCenter),
                    (Align::Right, Icon::FormatAlignRight),
                ]
                .into_iter()
                .map(|(align, glyph)| {
                    ui::icon_button(glyph)
                        .kind(ButtonKind::Tonal)
                        .size(button::Size::ExtraSmall)
                        .selected(text_style.align == align)
                        .on_press(edit(EditMessage::Style(StyleChange::Align(align))))
                })
                .collect()
            ))
            .padding(Padding {
                top: 0.0,
                right: 12.0,
                bottom: 8.0,
                left: 12.0,
            }),
        ]
        .width(Length::Shrink);

        let signatures = self.signature_menu();
        let history = &viewer.edit.history;
        use component::{DIVIDER_WIDTH, TOOL_WIDTH};
        let tools = |count: f32| count * TOOL_WIDTH + (count - 1.0) * 4.0;
        let group = |items: Vec<Element<'a, Message>>| -> Element<'a, Message> {
            row(items).spacing(4).align_y(Center).into()
        };
        // Each slot: its content, its width, when it moves into "More", and
        // whether a divider goes before it in the bar.
        let slots: Vec<(Element<'a, Message>, f32, Option<u8>, bool)> = vec![
            (
                group(vec![
                    tool_button(Icon::ArrowSelector, "Select", Tool::Select),
                    tool_button(Icon::HighlightAlt, "Rectangular selection", Tool::Area),
                ]),
                tools(2.0),
                None,
                false,
            ),
            (
                group(vec![
                    tool_button(Icon::Gesture, "Sketch", Tool::Sketch),
                    tool_button(Icon::Draw, "Draw", Tool::Draw),
                    menu_button(
                        shape_tool.map_or(Icon::Shapes, shape_icon),
                        "Shapes",
                        Menu::Shapes,
                        shape_tool.is_some(),
                        shapes.into(),
                    ),
                    tool_button(Icon::TextFields, "Text box", Tool::TextBox),
                    menu_button(
                        highlight_tool.map_or(Icon::InkHighlighter, |style| markup_label(style).1),
                        "Highlight",
                        Menu::Highlight,
                        highlight_tool.is_some(),
                        highlight.into(),
                    ),
                    tool_button(Icon::StickyNote, "Note", Tool::Note),
                ]),
                tools(6.0) + DIVIDER_WIDTH + 4.0,
                Some(1),
                true,
            ),
            (
                menu_button(Icon::Signature, "Sign", Menu::Signatures, false, signatures),
                DIVIDER_WIDTH + TOOL_WIDTH + 4.0,
                Some(2),
                true,
            ),
            (
                group(vec![
                    menu_button(
                        Icon::LineWeight,
                        "Shape style",
                        Menu::LineStyle,
                        false,
                        line_style.into(),
                    ),
                    menu_button(
                        Icon::BorderColor,
                        "Border color",
                        Menu::BorderColor,
                        false,
                        border_colors,
                    ),
                    menu_button(
                        Icon::FormatColorFill,
                        "Fill color",
                        Menu::FillColor,
                        false,
                        fill_colors,
                    ),
                    menu_button(
                        Icon::TextFormat,
                        "Text style",
                        Menu::TextStyle,
                        false,
                        text_menu.into(),
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
                        "Delete",
                        viewer
                            .edit
                            .selected
                            .is_some()
                            .then(|| edit(EditMessage::Delete)),
                    ),
                    component::toolbar_divider(),
                    component::tool(
                        Icon::Undo,
                        "Undo",
                        history.can_undo().then(|| edit(EditMessage::Undo)),
                    ),
                    component::tool(
                        Icon::Redo,
                        "Redo",
                        history.can_redo().then(|| edit(EditMessage::Redo)),
                    ),
                ]),
                tools(3.0) + DIVIDER_WIDTH + 4.0,
                None,
                false,
            ),
        ];
        let widths: Vec<(f32, Option<u8>)> = slots
            .iter()
            .map(|(_, width, order, _)| (*width, *order))
            .collect();
        let shown = component::fitting_slots(width, &widths);
        let last = slots.len() - 1;
        let mut bar = row![].spacing(4).align_y(Center);
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

    fn signature_menu(&self) -> Element<'_, Message> {
        let mut content = column![].width(300).padding([0, 8]);
        if self.signatures.is_empty() {
            content = content.push(
                container(
                    ui::styled("No signatures yet.", Type::BodyMedium)
                        .style(style::on_surface_variant),
                )
                .padding([8, 16]),
            );
        }
        for (index, loaded) in self.signatures.iter().enumerate() {
            let preview = container(
                image(loaded.handle.clone())
                    .height(36)
                    .content_fit(iced::ContentFit::Contain),
            )
            .width(Fill)
            .height(44)
            .align_y(Center)
            .padding([4, 8])
            .style(|_| iced::widget::container::Style {
                background: Some(Color::WHITE.into()),
                border: iced::border::rounded(shape::SMALL),
                ..Default::default()
            });
            let entry = button::custom(
                ButtonKind::Row,
                column![
                    preview,
                    ui::styled(&loaded.signature.description, Type::LabelMedium)
                        .style(style::on_surface_variant),
                ]
                .spacing(4)
                .padding([6, 0]),
            )
            .height(76.0)
            .width(Fill)
            .shape(button::Shape::Square)
            .on_press(Message::PlaceSignature(index));
            content = content.push(
                row![
                    entry,
                    component::tip(
                        ui::icon_button(Icon::Delete)
                            .size(button::Size::ExtraSmall)
                            .on_press(Message::RemoveSignature(index)),
                        "Delete signature"
                    )
                ]
                .spacing(4)
                .align_y(Center),
            );
        }
        content = content.push(
            container(iced::widget::rule::horizontal(1).style(style::divider)).padding([4, 0]),
        );
        content = content.push(menu_item(
            Some(Icon::Add),
            "Create Signature…",
            false,
            Message::NewSignature,
        ));
        content.into()
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
            let menu = popover::surface(
                column(options.iter().map(|option| {
                    menu_item(
                        (field.value == *option).then_some(Icon::Check),
                        option.as_str(),
                        field.value == *option,
                        Message::Edit(EditMessage::Choose(option.clone())),
                    )
                }))
                .width((field.rect.width() * scale).max(160.0))
                .padding([0, 8]),
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
                "No highlights or notes",
                "Highlights, notes and text boxes appear here.",
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
                signature_pad(&dialog.strokes, PAD, Message::SignatureStroke),
                ui::styled(
                    "Sign with your mouse, pen or touchpad on the line.",
                    Type::BodySmall
                )
                .style(style::on_surface_variant),
            ]
            .spacing(8)
            .into(),
            SignatureTab::Type => column![
                component::text_field(
                    "Your name",
                    &dialog.typed,
                    Backdrop::ContainerHigh,
                    |input| { input.on_input(Message::SignatureText) }
                ),
                container(
                    text(if dialog.typed.is_empty() {
                        "Your name".to_owned()
                    } else {
                        dialog.typed.clone()
                    })
                    .font(ui::font::SIGNATURE)
                    .size(48)
                    .color(if dialog.typed.is_empty() {
                        Color::from_rgb(0.7, 0.7, 0.7)
                    } else {
                        Color::from_rgb8(0x10, 0x18, 0x40)
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
                    None => ui::styled(
                        "Choose a photo or scan of your signature on white paper.",
                        Type::BodyMedium,
                    )
                    .style(|_| iced::widget::text::Style {
                        color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                    })
                    .into(),
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
                    ui::with_icon(ButtonKind::Tonal, Icon::Upload, "Choose Image…")
                        .on_press(Message::ChooseSignatureImage),
                ]
                .spacing(8)
                .into()
            }
        };
        let mut body = column![
            ui::styled("Create Signature", Type::HeadlineSmall),
            component::tabs(vec![
                tab("Draw", SignatureTab::Draw),
                tab("Type", SignatureTab::Type),
                tab("Image", SignatureTab::Image),
            ]),
            surface,
            component::text_field(
                "Description, such as Full name or Initials",
                &dialog.description,
                Backdrop::ContainerHigh,
                |input| input.on_input(Message::SignatureDescription)
            ),
        ]
        .spacing(16)
        .width(PAD.width);
        if let Some(error) = &dialog.error {
            body = body.push(ui::styled(error.as_str(), Type::BodyMedium).style(style::error_text));
        }
        body = body.push(
            row![
                ui::button(ButtonKind::Text, "Clear").on_press(Message::ClearSignature),
                space::horizontal(),
                ui::button(ButtonKind::Text, "Cancel").on_press(Message::CancelSignature),
                ui::button(ButtonKind::Filled, "Save")
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
                icon::icon(Icon::StickyNote, 18),
                ui::styled(
                    annotation.author.as_deref().unwrap_or("Note"),
                    Type::LabelLarge
                )
                .width(Fill),
                component::tip(
                    ui::icon_button(Icon::Delete)
                        .size(button::Size::ExtraSmall)
                        .on_press(edit(EditMessage::Delete)),
                    "Delete note"
                ),
                component::tip(
                    ui::icon_button(Icon::Check)
                        .size(button::Size::ExtraSmall)
                        .on_press(edit(EditMessage::CommitText)),
                    "Done"
                ),
            ]
            .spacing(6)
            .align_y(Center),
            editor
                .height(140)
                .size(14)
                .placeholder("Type a note")
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
    .padding(Padding {
        top: 6.0,
        right: 6.0,
        bottom: 10.0,
        left: 12.0,
    })
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
        Kind::Note => (Icon::StickyNote, "Note"),
        Kind::FreeText => (Icon::TextFields, "Text box"),
        Kind::Stamp => (Icon::Signature, "Stamp"),
        _ => (Icon::Shapes, "Shape"),
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
        text if text.is_empty() => fallback.to_owned(),
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
                    format!("Page {}", viewer.page_label(page)),
                    Type::LabelSmall
                )
                .style(style::on_surface_variant),
                ui::styled(body, Type::BodyMedium).width(Fill),
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
