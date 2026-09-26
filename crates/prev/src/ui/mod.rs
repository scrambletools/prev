//! Material Design 3 Expressive for iced: color scheme, type, icons,
//! shapes, motion and components.

pub mod button;
pub mod component;
pub mod enter;
pub mod font;
pub mod icon;
pub mod motion;
pub mod popover;
pub mod resize;
pub mod scheme;
pub mod style;

pub use button::{button, icon_button, with_icon};
pub use font::{Type, styled};
pub use icon::{Icon, icon};
pub use scheme::Scheme;

/// Opacity of the state layer drawn over an element in each state.
pub mod state_layer {
    pub const HOVERED: f32 = 0.08;
    pub const FOCUSED: f32 = 0.10;
    pub const PRESSED: f32 = 0.10;
    pub const DRAGGED: f32 = 0.16;
}

/// The M3 corner radius scale.
pub mod shape {
    pub const EXTRA_SMALL: f32 = 4.0;
    pub const SMALL: f32 = 8.0;
    pub const MEDIUM: f32 = 12.0;
    pub const LARGE: f32 = 16.0;
    pub const EXTRA_LARGE: f32 = 28.0;
    pub const FULL: f32 = 1000.0;
}

pub mod elevation {
    use iced::{Shadow, Vector};

    use super::Scheme;

    /// The shadow for an M3 elevation level, 0 to 5.
    pub fn shadow(scheme: &Scheme, level: u8) -> Shadow {
        let (offset, blur, alpha) = match level {
            0 => return Shadow::default(),
            1 => (1.0, 3.0, 0.15),
            2 => (2.0, 6.0, 0.15),
            3 => (4.0, 8.0, 0.15),
            4 => (6.0, 10.0, 0.15),
            _ => (8.0, 12.0, 0.15),
        };
        let alpha = if scheme.dark { alpha * 2.0 } else { alpha };
        Shadow {
            color: iced::Color {
                a: alpha,
                ..scheme.shadow
            },
            offset: Vector::new(0.0, offset),
            blur_radius: blur,
        }
    }
}

/// `color` with the state layer for `opacity` of `layer` drawn over it.
pub fn blend(color: iced::Color, layer: iced::Color, opacity: f32) -> iced::Color {
    let mix = |base: f32, over: f32| base + (over - base) * opacity;
    iced::Color {
        r: mix(color.r, layer.r),
        g: mix(color.g, layer.g),
        b: mix(color.b, layer.b),
        a: color.a + (1.0 - color.a) * opacity,
    }
}

/// `color` at `alpha` opacity.
pub fn faded(color: iced::Color, alpha: f32) -> iced::Color {
    iced::Color { a: alpha, ..color }
}
