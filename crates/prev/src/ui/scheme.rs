//! Material 3 color roles, generated from a seed color with
//! material-color-utilities, and the iced theme built from them.

use std::cell::RefCell;

use iced::theme::palette::{self, Extended, Pair};
use iced::{Color, Theme};
use material_colors::color::Argb;
use material_colors::dynamic_color::{DynamicScheme, Variant};

/// Seed when no Omarchy accent is in use.
pub const PREV_SEED: Color = Color::from_rgb8(0x2f, 0x6f, 0xde);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scheme {
    pub dark: bool,
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub inverse_primary: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
    pub surface: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub scrim: Color,
    pub shadow: Color,
    /// The window's own chrome: its toolbars, side panels, menus and
    /// dialogs. On Windows and macOS, the system's title bar color, so
    /// prev's bars run on from it; elsewhere the container tone.
    pub chrome: Color,
}

/// The system's title bar color, in light or dark: Windows 11's, and the
/// window background macOS draws its title bar from. `None` where the
/// theme has the say.
fn system_chrome(dark: bool) -> Option<Color> {
    if cfg!(windows) {
        Some(if dark {
            Color::from_rgb8(0x20, 0x20, 0x20)
        } else {
            Color::from_rgb8(0xf3, 0xf3, 0xf3)
        })
    } else if cfg!(target_os = "macos") {
        Some(if dark {
            Color::from_rgb8(0x1e, 0x1e, 0x1e)
        } else {
            Color::from_rgb8(0xec, 0xec, 0xec)
        })
    } else {
        None
    }
}

fn color(argb: Argb) -> Color {
    Color::from_rgba8(
        argb.red,
        argb.green,
        argb.blue,
        f32::from(argb.alpha) / 255.0,
    )
}

fn argb(color: Color) -> Argb {
    let [red, green, blue, _] = color.into_rgba8();
    Argb::new(255, red, green, blue)
}

impl Scheme {
    pub fn new(seed: Color, dark: bool) -> Self {
        // A grey seed has no hue to build a tonal scheme from; it gets a
        // monochrome one rather than an arbitrary tint.
        let [red, green, blue, _] = seed.into_rgba8();
        let grey = red.max(green).max(blue) - red.min(green).min(blue) <= 8;
        let variant = if grey {
            Variant::Monochrome
        } else {
            Variant::TonalSpot
        };
        let scheme = DynamicScheme::by_variant(argb(seed), &variant, dark, None);
        Self {
            dark,
            primary: color(scheme.primary()),
            on_primary: color(scheme.on_primary()),
            primary_container: color(scheme.primary_container()),
            on_primary_container: color(scheme.on_primary_container()),
            inverse_primary: color(scheme.inverse_primary()),
            secondary: color(scheme.secondary()),
            on_secondary: color(scheme.on_secondary()),
            secondary_container: color(scheme.secondary_container()),
            on_secondary_container: color(scheme.on_secondary_container()),
            tertiary: color(scheme.tertiary()),
            on_tertiary: color(scheme.on_tertiary()),
            tertiary_container: color(scheme.tertiary_container()),
            on_tertiary_container: color(scheme.on_tertiary_container()),
            error: color(scheme.error()),
            on_error: color(scheme.on_error()),
            error_container: color(scheme.error_container()),
            on_error_container: color(scheme.on_error_container()),
            surface: color(scheme.surface()),
            surface_dim: color(scheme.surface_dim()),
            surface_bright: color(scheme.surface_bright()),
            surface_container_lowest: color(scheme.surface_container_lowest()),
            surface_container_low: color(scheme.surface_container_low()),
            surface_container: color(scheme.surface_container()),
            surface_container_high: color(scheme.surface_container_high()),
            surface_container_highest: color(scheme.surface_container_highest()),
            on_surface: color(scheme.on_surface()),
            on_surface_variant: color(scheme.on_surface_variant()),
            outline: color(scheme.outline()),
            outline_variant: color(scheme.outline_variant()),
            inverse_surface: color(scheme.inverse_surface()),
            inverse_on_surface: color(scheme.inverse_on_surface()),
            scrim: color(scheme.scrim()),
            shadow: color(scheme.shadow()),
            chrome: system_chrome(dark).unwrap_or_else(|| color(scheme.surface_container())),
        }
    }

    /// The scheme a theme was built from. Themes not made by [`theme`]
    /// get a scheme seeded from their primary color.
    pub fn of(theme: &Theme) -> Self {
        let palette = theme.palette();
        let key = key(&palette);
        SCHEMES.with_borrow_mut(|schemes| {
            if let Some((_, scheme)) = schemes.iter().find(|(known, _)| *known == key) {
                return *scheme;
            }
            let scheme = Self::new(palette.primary, theme.extended_palette().is_dark);
            remember(schemes, key, scheme);
            scheme
        })
    }

    fn palette(&self) -> iced::theme::Palette {
        iced::theme::Palette {
            background: self.surface,
            text: self.on_surface,
            primary: self.primary,
            success: if self.dark {
                Color::from_rgb8(0x8c, 0xd5, 0x8f)
            } else {
                Color::from_rgb8(0x2e, 0x6b, 0x30)
            },
            warning: if self.dark {
                Color::from_rgb8(0xef, 0xc2, 0x6a)
            } else {
                Color::from_rgb8(0x7a, 0x59, 0x00)
            },
            danger: self.error,
        }
    }

    /// iced's own widgets read the extended palette; map it onto M3 roles.
    fn extended(&self) -> Extended {
        let pair = Pair::new;
        let base = self.palette();
        let generated = Extended::generate(base);
        Extended {
            background: palette::Background {
                base: pair(self.surface, self.on_surface),
                weakest: pair(self.surface_container_lowest, self.on_surface),
                weaker: pair(self.surface_container_low, self.on_surface),
                weak: pair(self.surface_container, self.on_surface),
                neutral: pair(self.surface_container_high, self.on_surface),
                strong: pair(self.surface_container_highest, self.on_surface),
                stronger: pair(self.outline_variant, self.on_surface),
                strongest: pair(self.outline, self.surface),
            },
            primary: palette::Primary {
                base: pair(self.primary, self.on_primary),
                weak: pair(self.primary_container, self.on_primary_container),
                strong: pair(self.on_primary_container, self.primary_container),
            },
            secondary: palette::Secondary {
                base: pair(self.secondary_container, self.on_secondary_container),
                weak: pair(self.surface_container_high, self.on_surface),
                strong: pair(self.secondary, self.on_secondary),
            },
            danger: palette::Danger {
                base: pair(self.error, self.on_error),
                weak: pair(self.error_container, self.on_error_container),
                strong: pair(self.on_error_container, self.error_container),
            },
            is_dark: self.dark,
            ..generated
        }
    }
}

type Key = [u32; 6];

fn key(palette: &iced::theme::Palette) -> Key {
    let bits = |color: Color| u32::from_be_bytes(color.into_rgba8());
    [
        bits(palette.background),
        bits(palette.text),
        bits(palette.primary),
        bits(palette.success),
        bits(palette.warning),
        bits(palette.danger),
    ]
}

thread_local! {
    static SCHEMES: RefCell<Vec<(Key, Scheme)>> = const { RefCell::new(Vec::new()) };
}

/// Theme changes are rare, so a handful of entries is plenty.
const REMEMBERED: usize = 8;

fn remember(schemes: &mut Vec<(Key, Scheme)>, key: Key, scheme: Scheme) {
    if schemes.len() >= REMEMBERED {
        schemes.remove(0);
    }
    schemes.push((key, scheme));
}

/// An iced theme whose colors come from the M3 scheme for `seed`.
pub fn theme(name: String, seed: Color, dark: bool) -> Theme {
    let scheme = Scheme::new(seed, dark);
    let palette = scheme.palette();
    SCHEMES.with_borrow_mut(|schemes| {
        let key = key(&palette);
        if !schemes.iter().any(|(known, _)| *known == key) {
            remember(schemes, key, scheme);
        }
    });
    Theme::custom_with_fn(name, palette, move |_| scheme.extended())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luminance(color: Color) -> f32 {
        0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b
    }

    #[test]
    fn light_and_dark_surfaces() {
        let light = Scheme::new(PREV_SEED, false);
        let dark = Scheme::new(PREV_SEED, true);
        assert!(luminance(light.surface) > 0.8);
        assert!(luminance(dark.surface) < 0.1);
        assert!(luminance(light.on_surface) < 0.2);
        assert!(luminance(dark.on_surface) > 0.7);
        // Containers step away from the surface in order.
        let steps = [
            light.surface_container_lowest,
            light.surface_container_low,
            light.surface_container,
            light.surface_container_high,
            light.surface_container_highest,
        ];
        assert!(
            steps
                .windows(2)
                .all(|pair| luminance(pair[0]) >= luminance(pair[1]))
        );
    }

    #[test]
    fn themes_find_their_scheme() {
        let theme = theme("test".into(), Color::from_rgb8(0xd0, 0x40, 0x40), true);
        let scheme = Scheme::of(&theme);
        assert_eq!(
            scheme,
            Scheme::new(Color::from_rgb8(0xd0, 0x40, 0x40), true)
        );
        assert_eq!(theme.palette().background, scheme.surface);
        assert!(theme.extended_palette().is_dark);
    }

    #[test]
    fn other_themes_get_a_scheme() {
        let scheme = Scheme::of(&Theme::Light);
        assert!(!scheme.dark);
    }
}
