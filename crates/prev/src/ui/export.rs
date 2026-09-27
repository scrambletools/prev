//! The export dialog of the image and Markdown windows: the format, the
//! JPEG quality and, where it applies, the size, before the save dialog.
//! It looks like the PDF window's export dialog.

use iced::widget::{column, container, row, space};
use iced::{Center, Element, Fill};

use super::button::{Kind, Size as ButtonSize};
use super::{Type, component, style};
use crate::image::editor;

/// Sizes to pick from: (choice id, label, scale), the one chosen, and a
/// note under them, such as the size in pixels.
pub struct Sizes<'a> {
    pub options: &'static [(&'static str, &'static str, f32)],
    pub chosen: &'a str,
    pub note: String,
}

/// What the dialog shows and the messages its choices send.
pub struct Dialog<'a, Message> {
    pub format: &'a str,
    pub quality: &'a str,
    pub sizes: Option<Sizes<'a>>,
    pub on_format: fn(&'static str) -> Message,
    pub on_quality: fn(&'static str) -> Message,
    pub on_size: fn(&'static str) -> Message,
    pub on_cancel: Message,
    pub on_choose: Message,
}

/// `page` with the export dialog over it.
pub fn dialog<'a, Message: Clone + 'a>(
    page: Element<'a, Message>,
    dialog: Dialog<'a, Message>,
) -> Element<'a, Message> {
    let options = |options: Vec<(&'static str, &'static str)>,
                   chosen: &str,
                   message: fn(&'static str) -> Message| {
        component::connected(
            options
                .into_iter()
                .map(|(id, label)| {
                    super::button(Kind::Tonal, label)
                        .size(ButtonSize::ExtraSmall)
                        .selected(chosen == id)
                        .on_press(message(id))
                })
                .collect(),
        )
    };
    // The formats in two rows, the common ones first.
    let formats: Vec<(&'static str, &'static str)> = editor::EXPORT_FORMATS
        .iter()
        .map(|(id, label, _)| (*id, label.split(" (").next().unwrap_or(label)))
        .collect();
    let (common, other) = formats.split_at(formats.len().min(5));
    let mut body = column![
        super::styled("Export", Type::HeadlineSmall),
        component::section("Format"),
        options(common.to_vec(), dialog.format, dialog.on_format),
        options(other.to_vec(), dialog.format, dialog.on_format),
    ]
    .spacing(12)
    .width(440);
    if dialog.format == "jpeg" {
        body = body.push(component::section("Quality"));
        body = body.push(options(
            editor::JPEG_QUALITIES
                .iter()
                .map(|(id, label, _)| (*id, *label))
                .collect(),
            dialog.quality,
            dialog.on_quality,
        ));
    }
    if let Some(sizes) = dialog.sizes {
        body = body.push(component::section("Size"));
        body = body.push(options(
            sizes
                .options
                .iter()
                .map(|(id, label, _)| (*id, *label))
                .collect(),
            sizes.chosen,
            dialog.on_size,
        ));
        body =
            body.push(super::styled(sizes.note, Type::BodySmall).style(style::on_surface_variant));
    }
    body = body.push(
        row![
            space::horizontal(),
            super::button(Kind::Text, "Cancel").on_press(dialog.on_cancel),
            super::button(Kind::Filled, "Export…").on_press(dialog.on_choose),
        ]
        .spacing(8)
        .align_y(Center),
    );
    let card = container(body).padding(24).style(style::dialog);
    iced::widget::stack![
        page,
        iced::widget::opaque(
            container(super::enter::grow(card))
                .center(Fill)
                .style(style::scrim)
        )
    ]
    .into()
}
