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

/// The docked toolbar along the top of a window, or the floating one
/// when bars float.
pub fn toolbar<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    bar(content, TOOLBAR_HEIGHT, style::surface_container)
}

/// A second toolbar under the first, such as the markup bar.
pub fn secondary_toolbar<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    height: f32,
) -> Element<'a, Message> {
    bar(content, height, style::surface_container_low)
}

/// M3 floating toolbar: a pill in the container color at elevation 3.
pub const FLOATING_TOOLBAR_HEIGHT: f32 = 64.0;
/// Floating bars' inset from the window edges, and the gap between them.
pub const FLOATING_MARGIN: f32 = 16.0;
pub const FLOATING_GAP: f32 = 8.0;

fn bar<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    height: f32,
    docked: fn(&Theme) -> container::Style,
) -> Element<'a, Message> {
    if !floating_bars_enabled() {
        return container(content)
            .padding([0, 8])
            .height(height)
            .width(Fill)
            .align_y(Center)
            .style(docked)
            .into();
    }
    container(content)
        .padding([0, 12])
        .height(FLOATING_TOOLBAR_HEIGHT)
        .width(Fill)
        .align_y(Center)
        .style(|theme: &Theme| {
            let scheme = Scheme::of(theme);
            container::Style {
                background: Some(scheme.surface_container.into()),
                text_color: Some(scheme.on_surface),
                // A pill at the most, since the radius is at most half the
                // bar's height.
                border: iced::border::rounded(shape::surface()),
                shadow: super::elevation::shadow(&scheme, 3),
                ..container::Style::default()
            }
        })
        .into()
}

static FLOATING_BARS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Makes document windows float their toolbars over the content and hide
/// them while the pointer is outside the window.
pub fn set_floating_bars(floating: bool) {
    FLOATING_BARS.store(floating, std::sync::atomic::Ordering::Relaxed);
}

pub fn floating_bars_enabled() -> bool {
    FLOATING_BARS.load(std::sync::atomic::Ordering::Relaxed)
}

/// A window's bars around its content. Docked, the top bar and then the
/// bottom one sit above `content` and push it down. Floating, they are M3
/// floating toolbars over the content, inset from the window edges: the
/// top bar at the top and the bottom bar at the bottom, shown only while
/// `shown` and sliding in from their edge as they appear.
pub fn window_bars<'a, Message: 'a>(
    top: Element<'a, Message>,
    bottom: Option<Element<'a, Message>>,
    content: Element<'a, Message>,
    shown: bool,
) -> Element<'a, Message> {
    if !floating_bars_enabled() {
        let mut docked = column![top];
        if let Some(bottom) = bottom {
            docked = docked.push(bottom);
        }
        return docked.push(content).into();
    }
    let slide = |bar: Element<'a, Message>, from: f32| -> Element<'a, Message> {
        super::enter::enter(
            container(bar).padding(FLOATING_MARGIN).width(Fill),
            super::enter::From::Offset(0.0, from),
        )
        .into()
    };
    let (top, bottom): (Element<'a, Message>, Element<'a, Message>) = if shown {
        (
            slide(top, -1.0),
            match bottom {
                Some(bottom) => slide(bottom, 1.0),
                None => iced::widget::space().into(),
            },
        )
    } else {
        (iced::widget::space().into(), iced::widget::space().into())
    };
    // The content stays the first layer, so hiding the bars keeps its
    // state, such as the scroll position.
    stack![
        content,
        column![top, iced::widget::space::vertical(), bottom]
            .width(Fill)
            .height(Fill)
    ]
    .into()
}

/// Room floating bars take at an edge: the bar and its margins.
pub fn floating_room(bar: bool) -> f32 {
    if bar && floating_bars_enabled() {
        FLOATING_MARGIN + FLOATING_TOOLBAR_HEIGHT + FLOATING_GAP
    } else {
        0.0
    }
}

/// The toolbar button that turns floating, auto-hiding bars on and off.
pub fn floating_bars_toggle<'a, Message: Clone + 'a>(message: Message) -> Element<'a, Message> {
    let floating = floating_bars_enabled();
    toggle_tool(
        if floating {
            Icon::TopPanelOpen
        } else {
            Icon::TopPanelClose
        },
        if floating {
            "Keep the toolbar shown"
        } else {
            "Hide the toolbar when the pointer leaves"
        },
        floating,
        message,
    )
}

/// Leaves room above and below `element` for floating bars, for
/// sidebars and panels the bars must not cover. The room takes the
/// sidebar color, so it reads as part of the sidebar while the bars hide.
pub fn between_bars<'a, Message: 'a>(
    element: Element<'a, Message>,
    top: f32,
    bottom: f32,
) -> Element<'a, Message> {
    if floating_bars_enabled() {
        container(column![
            iced::widget::space().height(top),
            element,
            iced::widget::space().height(bottom)
        ])
        .style(style::surface_container_low)
        .into()
    } else {
        element
    }
}

/// Width of an icon button in a toolbar, the gap between items, and a
/// divider with its padding.
pub const TOOL_WIDTH: f32 = 40.0;
pub const TOOLBAR_GAP: f32 = 8.0;
pub const DIVIDER_WIDTH: f32 = 9.0;

/// Which toolbar slots fit in `available` pixels. Each slot is its width
/// and, for slots that may move into the overflow menu, its place in the
/// order they go, lowest first. Once anything goes, room is kept for the
/// "More" button.
pub fn fitting_slots(available: f32, slots: &[(f32, Option<u8>)]) -> Vec<bool> {
    let mut shown = vec![true; slots.len()];
    let used = |shown: &[bool]| -> f32 {
        slots
            .iter()
            .zip(shown)
            .filter(|(_, shown)| **shown)
            .map(|((width, _), _)| width + TOOLBAR_GAP)
            .sum()
    };
    let mut order: Vec<(u8, usize)> = slots
        .iter()
        .enumerate()
        .filter_map(|(index, (_, order))| order.map(|order| (order, index)))
        .collect();
    order.sort();
    let mut dropped = false;
    for (_, index) in order {
        let more = if dropped {
            TOOL_WIDTH + TOOLBAR_GAP
        } else {
            0.0
        };
        if used(&shown) + more <= available {
            break;
        }
        shown[index] = false;
        dropped = true;
    }
    shown
}

/// The "More" button at the end of a toolbar, with the groups that did not
/// fit in a menu below it.
pub fn overflow<'a, Message: Clone + 'a>(
    hidden: Vec<Element<'a, Message>>,
    open: bool,
    on_toggle: Message,
    on_close: Message,
) -> Element<'a, Message> {
    let anchor = tip(
        button::icon_button(Icon::MoreVert)
            .selected(open)
            .on_press(on_toggle),
        "More",
    );
    let content = open.then(|| {
        super::popover::surface(
            column(
                hidden
                    .into_iter()
                    .map(|group| container(group).padding([4, 12]).into()),
            )
            .spacing(4),
        )
    });
    super::popover::popover(anchor, content, on_close).into()
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
    connected_with_tips(buttons.into_iter().map(|button| (button, None)).collect())
}

/// A connected button group where each button has its own tooltip.
pub fn connected_with_tips<'a, Message: Clone + 'a>(
    buttons: Vec<(Button<'a, Message>, Option<&'a str>)>,
) -> Element<'a, Message> {
    let count = buttons.len();
    row(buttons
        .into_iter()
        .enumerate()
        .map(|(index, (button, label))| {
            let position = match (index, count) {
                (_, 1) => Position::Alone,
                (0, _) => Position::First,
                (index, count) if index + 1 == count => Position::Last,
                _ => Position::Middle,
            };
            let button = button.position(position);
            match label {
                Some(label) => tip(button, label),
                None => button.into(),
            }
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
    /// Shows the icon instead of the label, which becomes a tooltip.
    pub icon: Option<Icon>,
    pub selected: bool,
    pub on_press: Message,
}

/// Primary tabs with an indicator under the selected one.
pub fn tabs<'a, Message: Clone + 'a>(tabs: Vec<Tab<'a, Message>>) -> Element<'a, Message> {
    let tabs = row(tabs.into_iter().map(|tab| {
        let selected = tab.selected;
        let label: Element<'a, Message> = match tab.icon {
            Some(glyph) if selected => icon::filled(glyph, 22).into(),
            Some(glyph) => icon::icon(glyph, 22).into(),
            None => font::styled(tab.label, Type::TitleSmall).into(),
        };
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
        let body: Element<'a, Message> = match tab.icon {
            Some(_) => tip(body, tab.label),
            None => body.into(),
        };
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
    // The same widgets whether or not there is text, so typing the first
    // character does not rebuild the field and lose the keyboard focus.
    let floating: Element<'a, Message> = if value.is_empty() {
        Space::new().into()
    } else {
        container(font::styled(label, Type::BodySmall).style(style::on_surface_variant))
            .padding([0, 4])
            .style(move |theme: &Theme| iced::widget::container::Style {
                background: Some(backdrop.color(&Scheme::of(theme)).into()),
                ..Default::default()
            })
            .into()
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_go_in_order_and_leave_room_for_more() {
        let slots = [
            (100.0, None),
            (100.0, Some(1)),
            (100.0, Some(0)),
            (100.0, None),
        ];
        assert_eq!(fitting_slots(1000.0, &slots), [true, true, true, true]);
        // 4 x 108 = 432 does not fit in 400; slot 2 goes, and the rest plus
        // "More" (48) take 372.
        assert_eq!(fitting_slots(400.0, &slots), [true, true, false, true]);
        assert_eq!(fitting_slots(300.0, &slots), [true, false, false, true]);
    }
}
