//! Material Design 3 Expressive for iced: color scheme, type, icons,
//! shapes, motion and components.

pub mod button;
pub mod component;
pub mod dir;
pub mod enter;
pub mod export;
pub mod field;
pub mod font;
pub mod icon;
pub mod motion;
pub mod popover;
pub mod probe;
pub mod resize;
pub mod scheme;
pub mod smooth;
pub mod style;

pub use button::{button, icon_button, with_icon};
pub use font::{Type, aligned, aligned_to, styled};
pub use icon::{Icon, icon};
pub use scheme::Scheme;

/// A message from a panel the app puts in a window's side, such as the
/// assistant, carried through the window's own messages untouched for the
/// app to take back.
#[derive(Clone)]
pub struct Outside(std::sync::Arc<dyn std::any::Any + Send + Sync>);

impl Outside {
    pub fn new<M: Send + Sync + 'static>(message: M) -> Self {
        Self(std::sync::Arc::new(message))
    }

    /// The message, if it is an `M`.
    pub fn take<M: Clone + 'static>(&self) -> Option<M> {
        self.0.downcast_ref::<M>().cloned()
    }
}

impl std::fmt::Debug for Outside {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Outside")
    }
}

/// Opacity of the state layer drawn over an element in each state.
pub mod state_layer {
    pub const HOVERED: f32 = 0.08;
    pub const FOCUSED: f32 = 0.10;
    pub const PRESSED: f32 = 0.10;
    pub const DRAGGED: f32 = 0.16;
}

/// The M3 corner radius scale.
pub mod shape {
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Corner radius of dialogs and floating toolbars, from the settings;
    /// M3's extra large by default.
    static SURFACE: AtomicU32 = AtomicU32::new(EXTRA_LARGE.to_bits());

    pub fn set_surface(radius: f32) {
        SURFACE.store(radius.clamp(0.0, 32.0).to_bits(), Ordering::Relaxed);
    }

    pub fn surface() -> f32 {
        f32::from_bits(SURFACE.load(Ordering::Relaxed))
    }

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
