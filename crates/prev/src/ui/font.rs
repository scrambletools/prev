//! Bundled fonts and the M3 type scale.

use std::borrow::Cow;

use iced::font::{Family, Weight};
use iced::widget::text::LineHeight;
use iced::widget::{Text, text};
use iced::{Font, Pixels};

pub const TEXT: Font = Font {
    family: Family::Name("Roboto Flex"),
    ..Font::DEFAULT
};
pub const ICONS: Font = Font::with_name("Material Symbols Rounded");
pub const ICONS_FILLED: Font = Font::with_name("Material Symbols Rounded Filled");

/// Font files to load at startup.
pub const FILES: [&[u8]; 3] = [
    include_bytes!("../../assets/fonts/RobotoFlex.ttf"),
    include_bytes!("../../assets/fonts/MaterialSymbolsRounded.ttf"),
    include_bytes!("../../assets/fonts/MaterialSymbolsRoundedFilled.ttf"),
];

pub fn files() -> impl Iterator<Item = Cow<'static, [u8]>> {
    FILES.into_iter().map(Cow::Borrowed)
}

/// Body medium, the size of most interface text.
pub const DEFAULT_SIZE: f32 = 14.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    DisplayLarge,
    DisplayMedium,
    DisplaySmall,
    HeadlineLarge,
    HeadlineMedium,
    HeadlineSmall,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    BodyLarge,
    BodyMedium,
    BodySmall,
    LabelLarge,
    LabelMedium,
    LabelSmall,
}

impl Type {
    /// Size and line height in pixels.
    pub fn metrics(self) -> (f32, f32) {
        match self {
            Type::DisplayLarge => (57.0, 64.0),
            Type::DisplayMedium => (45.0, 52.0),
            Type::DisplaySmall => (36.0, 44.0),
            Type::HeadlineLarge => (32.0, 40.0),
            Type::HeadlineMedium => (28.0, 36.0),
            Type::HeadlineSmall => (24.0, 32.0),
            Type::TitleLarge => (22.0, 28.0),
            Type::TitleMedium => (16.0, 24.0),
            Type::TitleSmall => (14.0, 20.0),
            Type::BodyLarge => (16.0, 24.0),
            Type::BodyMedium => (14.0, 20.0),
            Type::BodySmall => (12.0, 16.0),
            Type::LabelLarge => (14.0, 20.0),
            Type::LabelMedium => (12.0, 16.0),
            Type::LabelSmall => (11.0, 16.0),
        }
    }

    pub fn weight(self, emphasized: bool) -> Weight {
        use Type::*;
        match (self, emphasized) {
            (TitleMedium | TitleSmall | LabelLarge | LabelMedium | LabelSmall, false) => {
                Weight::Medium
            }
            (TitleMedium | TitleSmall | LabelLarge | LabelMedium | LabelSmall, true) => {
                Weight::Bold
            }
            (_, false) => Weight::Normal,
            (_, true) => Weight::Medium,
        }
    }

    pub fn size(self) -> Pixels {
        Pixels(self.metrics().0)
    }

    pub fn font(self, emphasized: bool) -> Font {
        Font {
            weight: self.weight(emphasized),
            ..TEXT
        }
    }

    fn apply<'a>(self, text: Text<'a>, emphasized: bool) -> Text<'a> {
        let (size, line_height) = self.metrics();
        text.size(size)
            .line_height(LineHeight::Absolute(Pixels(line_height)))
            .font(self.font(emphasized))
    }
}

/// Text in one of the M3 type styles.
pub fn styled<'a>(content: impl text::IntoFragment<'a>, style: Type) -> Text<'a> {
    style.apply(text(content), false)
}
