//! M3 styles for iced's built-in widgets.

use iced::widget::{container, rule, scrollable, slider as slider_widget, text_input, toggler};
use iced::{Background, Border, Color, Theme, border};

use super::{Scheme, blend, elevation, faded, shape, state_layer};

// Containers.

fn filled(color: Color, content: Color) -> container::Style {
    container::Style {
        text_color: Some(content),
        background: Some(Background::Color(color)),
        ..container::Style::default()
    }
}

pub fn surface(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    filled(scheme.surface, scheme.on_surface)
}

pub fn surface_container_low(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    filled(scheme.surface_container_low, scheme.on_surface)
}

pub fn surface_container(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    filled(scheme.surface_container, scheme.on_surface)
}

/// A modal dialog.
pub fn dialog(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    container::Style {
        border: border::rounded(shape::EXTRA_LARGE),
        shadow: elevation::shadow(&scheme, 3),
        ..filled(scheme.surface_container_high, scheme.on_surface)
    }
}

/// Dims the window behind a dialog.
pub fn scrim(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    container::Style {
        background: Some(Background::Color(faded(scheme.scrim, 0.32))),
        ..container::Style::default()
    }
}

pub fn tooltip(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    container::Style {
        border: border::rounded(shape::EXTRA_SMALL),
        ..filled(scheme.inverse_surface, scheme.inverse_on_surface)
    }
}

pub fn snackbar(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    container::Style {
        border: border::rounded(shape::EXTRA_SMALL),
        shadow: elevation::shadow(&scheme, 3),
        ..filled(scheme.inverse_surface, scheme.inverse_on_surface)
    }
}

/// Corner radius of sidebar thumbnails, which are clipped to it.
pub const THUMBNAIL_RADIUS: f32 = shape::SMALL;
/// Space between a thumbnail and the outer edge of its selection ring.
pub const THUMBNAIL_RING: f32 = 5.0;

/// The frame around a thumbnail; `selected` gets a primary ring that
/// follows the thumbnail's rounded corners, with a gap inside it.
pub fn thumbnail(theme: &Theme, selected: bool) -> container::Style {
    let scheme = Scheme::of(theme);
    container::Style {
        border: Border {
            color: if selected {
                scheme.primary
            } else {
                Color::TRANSPARENT
            },
            width: 3.0,
            radius: (THUMBNAIL_RADIUS + THUMBNAIL_RING).into(),
        },
        ..container::Style::default()
    }
}

/// The drop target outline while files are dragged over a window.
pub fn drop_target(theme: &Theme) -> container::Style {
    let scheme = Scheme::of(theme);
    container::Style {
        border: Border {
            color: scheme.primary,
            width: 3.0,
            radius: shape::MEDIUM.into(),
        },
        background: Some(Background::Color(faded(
            scheme.primary,
            state_layer::DRAGGED / 2.0,
        ))),
        ..container::Style::default()
    }
}

// Text.

pub fn on_surface_variant(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(Scheme::of(theme).on_surface_variant),
    }
}

pub fn primary_text(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(Scheme::of(theme).primary),
    }
}

pub fn error_text(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(Scheme::of(theme).error),
    }
}

// Inputs.

/// An outlined text field.
pub fn outlined_field(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let scheme = Scheme::of(theme);
    let (color, width) = match status {
        text_input::Status::Active => (scheme.outline, 1.0),
        text_input::Status::Hovered => (scheme.on_surface, 1.0),
        text_input::Status::Focused { .. } => (scheme.primary, 2.0),
        text_input::Status::Disabled => (faded(scheme.on_surface, 0.12), 1.0),
    };
    let disabled = matches!(status, text_input::Status::Disabled);
    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color,
            width,
            radius: shape::EXTRA_SMALL.into(),
        },
        icon: scheme.on_surface_variant,
        placeholder: scheme.on_surface_variant,
        value: if disabled {
            faded(scheme.on_surface, 0.38)
        } else {
            scheme.on_surface
        },
        selection: faded(scheme.primary, 0.4),
    }
}

/// The text field inside a search bar, which draws its own container.
pub fn bare_field(theme: &Theme, _status: text_input::Status) -> text_input::Style {
    let scheme = Scheme::of(theme);
    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        icon: scheme.on_surface_variant,
        placeholder: scheme.on_surface_variant,
        value: scheme.on_surface,
        selection: faded(scheme.primary, 0.4),
    }
}

/// The M3 Expressive slider: a thick track and a narrow handle with a gap
/// on each side. The gap is drawn as a border in `backdrop`, the color
/// behind the slider.
pub fn slider(backdrop: Color) -> impl Fn(&Theme, slider_widget::Status) -> slider_widget::Style {
    move |theme, status| {
        let scheme = Scheme::of(theme);
        let handle_width = match status {
            slider_widget::Status::Dragged => 2,
            _ => 4,
        };
        slider_widget::Style {
            rail: slider_widget::Rail {
                backgrounds: (
                    Background::Color(scheme.primary),
                    Background::Color(scheme.secondary_container),
                ),
                width: SLIDER_TRACK,
                border: border::rounded(shape::SMALL),
            },
            handle: slider_widget::Handle {
                shape: slider_widget::HandleShape::Rectangle {
                    width: handle_width + 2 * SLIDER_GAP as u16,
                    border_radius: (f32::from(handle_width) / 2.0 + SLIDER_GAP).into(),
                },
                background: Background::Color(scheme.primary),
                border_width: SLIDER_GAP,
                border_color: backdrop,
            },
        }
    }
}

pub const SLIDER_TRACK: f32 = 16.0;
pub const SLIDER_GAP: f32 = 6.0;
/// Handle height plus the gap drawn above and below it. M3 uses a 44 px
/// handle; panels full of sliders use a shorter one at desktop density.
pub const SLIDER_HEIGHT: f32 = 32.0 + 2.0 * SLIDER_GAP;

/// The M3 switch.
pub fn switch(theme: &Theme, status: toggler::Status) -> toggler::Style {
    let scheme = Scheme::of(theme);
    let (toggled, hovered, disabled) = match status {
        toggler::Status::Active { is_toggled } => (is_toggled, false, false),
        toggler::Status::Hovered { is_toggled } => (is_toggled, true, false),
        toggler::Status::Disabled { is_toggled } => (is_toggled, false, true),
    };
    let (track, handle, outline) = if toggled {
        (scheme.primary, scheme.on_primary, scheme.primary)
    } else {
        (
            scheme.surface_container_highest,
            scheme.outline,
            scheme.outline,
        )
    };
    let track = if hovered {
        blend(track, handle, state_layer::HOVERED)
    } else {
        track
    };
    let fade = |color: Color| if disabled { faded(color, 0.38) } else { color };
    toggler::Style {
        background: Background::Color(fade(track)),
        background_border_width: 2.0,
        background_border_color: fade(outline),
        foreground: Background::Color(fade(handle)),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(scheme.on_surface),
        border_radius: None,
        // A bigger handle when on, as in M3.
        padding_ratio: if toggled { 0.125 } else { 0.25 },
    }
}

pub fn scrollbar(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let scheme = Scheme::of(theme);
    let rail = |hovered: bool, dragged: bool| scrollable::Rail {
        background: None,
        border: Border::default(),
        scroller: scrollable::Scroller {
            background: Background::Color(faded(
                scheme.on_surface_variant,
                if dragged {
                    0.7
                } else if hovered {
                    0.55
                } else {
                    0.35
                },
            )),
            border: border::rounded(shape::FULL),
        },
    };
    let (vertical, horizontal) = match status {
        scrollable::Status::Active { .. } => (rail(false, false), rail(false, false)),
        scrollable::Status::Hovered {
            is_vertical_scrollbar_hovered,
            is_horizontal_scrollbar_hovered,
            ..
        } => (
            rail(is_vertical_scrollbar_hovered, false),
            rail(is_horizontal_scrollbar_hovered, false),
        ),
        scrollable::Status::Dragged {
            is_vertical_scrollbar_dragged,
            is_horizontal_scrollbar_dragged,
            ..
        } => (
            rail(false, is_vertical_scrollbar_dragged),
            rail(false, is_horizontal_scrollbar_dragged),
        ),
    };
    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: vertical,
        horizontal_rail: horizontal,
        gap: None,
        auto_scroll: scrollable::default(theme, status).auto_scroll,
    }
}

pub fn divider(theme: &Theme) -> rule::Style {
    rule::Style {
        color: Scheme::of(theme).outline_variant,
        ..rule::default(theme)
    }
}
