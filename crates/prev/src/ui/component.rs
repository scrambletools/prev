//! M3 components built from iced widgets and prev's buttons.

use std::time::Duration;

use iced::widget::{
    Space, column, container, opaque, row, rule, space, stack, text, text_input, tooltip,
};
use iced::{Center, Element, Fill, Length, Padding, Theme};

use super::button::{self, Button, Kind, Position, Shape};
use super::font::{self, Type};
use super::icon::{self, Icon};
use super::{Scheme, enter, shape, style};

/// Height of the docked toolbar at desktop density.
pub const TOOLBAR_HEIGHT: f32 = 56.0;
pub const SIDE_SHEET_WIDTH: f32 = 320.0;
const TOOLTIP_DELAY: Duration = Duration::from_millis(500);

/// The docked toolbar along the top of a window.
pub fn toolbar<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding([0, 8])
        .height(TOOLBAR_HEIGHT)
        .width(Fill)
        .align_y(Center)
        .style(style::surface_container)
        .into()
}

/// A standard button group: related buttons with a small gap.
pub fn group<'a, Message: Clone + 'a>(
    buttons: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    row(buttons).spacing(4).align_y(Center).into()
}

/// A connected button group, where one choice is selected, as M3
/// Expressive uses in place of segmented buttons.
pub fn connected<'a, Message: Clone + 'a>(
    buttons: Vec<Button<'a, Message>>,
) -> Element<'a, Message> {
    let count = buttons.len();
    row(buttons.into_iter().enumerate().map(|(index, button)| {
        let position = match (index, count) {
            (_, 1) => Position::Alone,
            (0, _) => Position::First,
            (index, count) if index + 1 == count => Position::Last,
            _ => Position::Middle,
        };
        button.position(position).into()
    }))
    .spacing(2)
    .align_y(Center)
    .into()
}

/// A plain tooltip below `content`.
pub fn tip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    label: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    tooltip(
        content,
        container(font::styled(label, Type::BodySmall))
            .padding([4, 8])
            .style(style::tooltip),
        tooltip::Position::Bottom,
    )
    .gap(4)
    .delay(TOOLTIP_DELAY)
    .into()
}

/// An icon button with a tooltip naming it.
pub fn tool<'a, Message: Clone + 'a>(
    glyph: Icon,
    label: &'a str,
    message: Option<Message>,
) -> Element<'a, Message> {
    tip(button::icon_button(glyph).on_press_maybe(message), label)
}

/// A toggle icon button with a tooltip.
pub fn toggle_tool<'a, Message: Clone + 'a>(
    glyph: Icon,
    label: &'a str,
    selected: bool,
    message: Message,
) -> Element<'a, Message> {
    tip(
        button::icon_button(glyph)
            .selected(selected)
            .on_press(message),
        label,
    )
}

/// A vertical divider between toolbar groups.
pub fn toolbar_divider<'a, Message: 'a>() -> Element<'a, Message> {
    container(rule::vertical(1).style(style::divider))
        .height(24)
        .padding([0, 4])
        .into()
}

pub struct Tab<'a, Message> {
    pub label: &'a str,
    pub selected: bool,
    pub on_press: Message,
}

/// Primary tabs with an indicator under the selected one.
pub fn tabs<'a, Message: Clone + 'a>(tabs: Vec<Tab<'a, Message>>) -> Element<'a, Message> {
    let tabs = row(tabs.into_iter().map(|tab| {
        let selected = tab.selected;
        let label = font::styled(tab.label, Type::TitleSmall);
        let indicator = container(space().height(3))
            .width(Fill)
            .padding([0, 12])
            .style(move |theme: &Theme| {
                let scheme = Scheme::of(theme);
                if selected {
                    iced::widget::container::Style {
                        background: Some(scheme.primary.into()),
                        border: iced::border::rounded(iced::border::top(3)),
                        ..Default::default()
                    }
                } else {
                    Default::default()
                }
            });
        let body = button::custom(Kind::Tab, label)
            .shape(Shape::Flat)
            .selected(selected)
            .height(45.0)
            .width(Fill)
            .on_press(tab.on_press);
        column![body, indicator].width(Fill).into()
    }));
    column![tabs, rule::horizontal(1).style(style::divider)].into()
}

/// A docked side sheet with a title, a close button and content.
pub fn side_sheet<'a, Message: Clone + 'a>(
    title: &'a str,
    on_close: Message,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let header = row![
        font::styled(title, Type::TitleLarge),
        space::horizontal(),
        tip(button::icon_button(Icon::Close).on_press(on_close), "Close"),
    ]
    .align_y(Center)
    .padding(Padding {
        top: 12.0,
        right: 12.0,
        bottom: 8.0,
        left: 24.0,
    });
    let sheet = container(column![
        header,
        scroll(container(content).padding(Padding {
            top: 8.0,
            right: 24.0,
            bottom: 24.0,
            left: 24.0,
        }))
        .height(Fill),
    ])
    .width(SIDE_SHEET_WIDTH)
    .height(Fill)
    .style(style::surface_container_low);
    enter::from_right(sheet)
}

/// A heading for a group of controls in a sheet.
pub fn section<'a, Message: 'a>(label: &'a str) -> Element<'a, Message> {
    container(font::styled(label, Type::TitleSmall).style(style::primary_text))
        .padding(Padding {
            top: 12.0,
            bottom: 4.0,
            ..Padding::ZERO
        })
        .into()
}

/// A basic dialog over a scrim, on top of `base`.
pub fn dialog<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    glyph: Option<Icon>,
    headline: impl text::IntoFragment<'a>,
    supporting: impl text::IntoFragment<'a>,
    actions: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut body = column![].spacing(16).width(Fill);
    if let Some(glyph) = glyph {
        body = body.push(
            container(icon::icon(glyph, 24).style(|theme: &Theme| text::Style {
                color: Some(Scheme::of(theme).secondary),
            }))
            .center_x(Fill),
        );
    }
    let headline = font::styled(headline, Type::HeadlineSmall);
    body = body.push(if glyph.is_some() {
        Element::from(container(headline.center()).center_x(Fill))
    } else {
        headline.into()
    });
    body = body
        .push(font::styled(supporting, Type::BodyMedium).style(style::on_surface_variant))
        .push(
            container(row(actions).spacing(8))
                .align_right(Fill)
                .padding(Padding {
                    top: 8.0,
                    ..Padding::ZERO
                }),
        );
    let card = container(body)
        .padding(24)
        .max_width(560)
        .width(Length::Shrink)
        .style(style::dialog);
    stack![
        base.into(),
        opaque(
            container(enter::grow(
                container(card).width(Length::Fixed(DIALOG_WIDTH))
            ))
            .center(Fill)
            .style(style::scrim)
        )
    ]
    .into()
}

const DIALOG_WIDTH: f32 = 420.0;

/// A snackbar along the bottom of `base`.
pub fn snackbar<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    message: &'a str,
    on_dismiss: Message,
) -> Element<'a, Message> {
    let bar = container(
        row![
            font::styled(message, Type::BodyMedium).width(Fill),
            button::icon_button(Icon::Close)
                .size(button::Size::ExtraSmall)
                .kind(Kind::Standard)
                .on_press(on_dismiss),
        ]
        .spacing(8)
        .align_y(Center),
    )
    .padding(Padding {
        top: 6.0,
        right: 8.0,
        bottom: 6.0,
        left: 16.0,
    })
    .max_width(600)
    .style(style::snackbar);
    stack![
        base.into(),
        container(enter::from_below(bar))
            .align_bottom(Fill)
            .center_x(Fill)
            .padding(16)
    ]
    .into()
}

/// Which surface a field sits on, for the notch behind its label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backdrop {
    Surface,
    ContainerLow,
    ContainerHigh,
}

impl Backdrop {
    pub fn color(self, scheme: &Scheme) -> iced::Color {
        match self {
            Backdrop::Surface => scheme.surface,
            Backdrop::ContainerLow => scheme.surface_container_low,
            Backdrop::ContainerHigh => scheme.surface_container_high,
        }
    }
}

/// An outlined text field whose label sits in the outline once there is
/// text, and stands in for the placeholder until then.
pub fn text_field<'a, Message: Clone + 'a>(
    label: &'a str,
    value: &'a str,
    backdrop: Backdrop,
    configure: impl FnOnce(text_input::TextInput<'a, Message>) -> text_input::TextInput<'a, Message>,
) -> Element<'a, Message> {
    let placeholder = if value.is_empty() { label } else { "" };
    let field = configure(text_input(placeholder, value))
        .padding([12, 16])
        .size(Type::BodyLarge.size())
        .font(Type::BodyLarge.font(false))
        .style(style::outlined_field);
    if value.is_empty() {
        return container(field)
            .padding(Padding {
                top: 8.0,
                ..Padding::ZERO
            })
            .into();
    }
    let floating = container(font::styled(label, Type::BodySmall).style(style::on_surface_variant))
        .padding([0, 4])
        .style(move |theme: &Theme| iced::widget::container::Style {
            background: Some(backdrop.color(&Scheme::of(theme)).into()),
            ..Default::default()
        });
    stack![
        container(field).padding(Padding {
            top: 8.0,
            ..Padding::ZERO
        }),
        container(floating).padding(Padding {
            left: 12.0,
            ..Padding::ZERO
        }),
    ]
    .into()
}

/// A compact search bar for toolbars: a pill with a search icon, the
/// field, and trailing content such as a match count.
pub fn search_bar<'a, Message: Clone + 'a>(
    input: text_input::TextInput<'a, Message>,
    trailing: Vec<Element<'a, Message>>,
    width: f32,
) -> Element<'a, Message> {
    let input = input
        .style(style::bare_field)
        .padding([0, 4])
        .size(Type::BodyLarge.size())
        .font(Type::BodyLarge.font(false));
    let mut content = row![
        icon::icon(Icon::Search, 20).style(style::on_surface_variant),
        input
    ]
    .spacing(4)
    .align_y(Center)
    .padding(Padding {
        top: 0.0,
        right: 4.0,
        bottom: 0.0,
        left: 12.0,
    });
    for element in trailing {
        content = content.push(element);
    }
    container(content)
        .height(40)
        .width(width)
        .align_y(Center)
        .style(|theme: &Theme| {
            let scheme = Scheme::of(theme);
            iced::widget::container::Style {
                background: Some(scheme.surface_container_highest.into()),
                border: iced::border::rounded(shape::FULL),
                text_color: Some(scheme.on_surface),
                ..Default::default()
            }
        })
        .into()
}

/// A navigation style list row: a pill that fills when selected.
pub fn list_row<'a, Message: Clone + 'a>(
    leading: Option<Icon>,
    label: impl text::IntoFragment<'a>,
    indent: f32,
    selected: bool,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let mut content = row![].spacing(12).align_y(Center);
    if indent > 0.0 {
        content = content.push(Space::new().width(indent));
    }
    if let Some(glyph) = leading {
        content = content.push(icon::icon(glyph, 20));
    }
    content = content.push(font::styled(label, Type::LabelLarge).wrapping(text::Wrapping::None));
    button::custom(Kind::Row, content)
        .selected(selected)
        .height(40.0)
        .width(Fill)
        .on_press_maybe(on_press)
}

/// A vertical scrollable with a thin M3 style scrollbar.
pub fn scroll<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    iced::widget::scrollable(content)
        .direction(iced::widget::scrollable::Direction::Vertical(
            thin_scrollbar(),
        ))
        .style(style::scrollbar)
}

pub fn thin_scrollbar() -> iced::widget::scrollable::Scrollbar {
    iced::widget::scrollable::Scrollbar::default()
        .width(6)
        .scroller_width(6)
        .margin(2)
}

/// A centred message with an icon, for empty and error states.
pub fn empty_state<'a, Message: 'a>(
    glyph: Icon,
    headline: impl text::IntoFragment<'a>,
    supporting: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    container(
        column![
            icon::icon(glyph, 48).style(style::on_surface_variant),
            font::styled(headline, Type::TitleLarge),
            font::styled(supporting, Type::BodyMedium)
                .style(style::on_surface_variant)
                .center(),
        ]
        .spacing(12)
        .align_x(Center)
        .max_width(420),
    )
    .center(Fill)
    .into()
}
