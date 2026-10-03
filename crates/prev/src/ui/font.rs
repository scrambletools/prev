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
/// Handwriting, for typed signatures.
/// The bundled instance is semibold, and iced only picks a named font at
/// its exact weight.
pub const SIGNATURE: Font = Font {
    family: Family::Name("Dancing Script"),
    weight: Weight::Semibold,
    ..Font::DEFAULT
};
pub const SIGNATURE_FILE: &[u8] = include_bytes!("../../assets/fonts/DancingScript.ttf");

/// Font files to load at startup.
pub const FILES: [&[u8]; 5] = [
    include_bytes!("../../assets/fonts/RobotoFlex.ttf"),
    // Bold text finds Roboto Flex only at a weight a face registers.
    include_bytes!("../../assets/fonts/RobotoFlexBold.ttf"),
    include_bytes!("../../assets/fonts/MaterialSymbolsRounded.ttf"),
    include_bytes!("../../assets/fonts/MaterialSymbolsRoundedFilled.ttf"),
    SIGNATURE_FILE,
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

/// How wide `content` is in `style`, laid out as the interface lays it
/// out, for layouts that must know before drawing, such as a toolbar
/// deciding what fits. Measures with the fonts of every script, so it holds
/// in any language.
pub fn measure(content: &str, style: Type) -> f32 {
    use iced::advanced::graphics::text::Paragraph;
    use iced::advanced::text::{self, Paragraph as _};
    let (size, line_height) = style.metrics();
    Paragraph::with_text(text::Text {
        content,
        bounds: iced::Size::INFINITE,
        size: Pixels(size),
        line_height: LineHeight::Absolute(Pixels(line_height)),
        font: style.font(false),
        align_x: text::Alignment::Default,
        align_y: iced::alignment::Vertical::Top,
        shaping: text::Shaping::Advanced,
        wrapping: text::Wrapping::None,
    })
    .min_width()
}

/// Text in one of the M3 type styles.
pub fn styled<'a>(content: impl text::IntoFragment<'a>, style: Type) -> Text<'a> {
    style.apply(text(content), false)
}

/// `text` in the width it is given, at its start: on the right in right to
/// left languages. For titles, labels beside controls and descriptions that
/// may wrap. The text keeps its default alignment, which lines up the lines
/// of a paragraph with its own direction, and the container places it;
/// iced's right alignment misplaces text laid out wider than itself.
pub fn aligned<'a, Message: 'a>(text: Text<'a>) -> iced::Element<'a, Message> {
    aligned_to(text, iced::Fill)
}

/// [`aligned`] in a column `width` wide, such as labels in front of values,
/// so they all start at the same edge.
pub fn aligned_to<'a, Message: 'a>(
    text: Text<'a>,
    width: impl Into<iced::Length>,
) -> iced::Element<'a, Message> {
    iced::widget::container(text)
        .width(width)
        .align_x(super::dir::horizontal_start())
        .into()
}

#[cfg(test)]
mod cursor_tests {
    use iced::advanced::graphics::text::Paragraph;
    use iced::advanced::text::{self, Paragraph as _};

    fn paragraph(content: &str) -> Paragraph {
        Paragraph::with_text(text::Text {
            content,
            bounds: iced::Size::INFINITE,
            size: iced::Pixels(16.0),
            line_height: text::LineHeight::default(),
            font: iced::Font::DEFAULT,
            align_x: text::Alignment::Default,
            align_y: iced::alignment::Vertical::Top,
            shaping: text::Shaping::Advanced,
            wrapping: text::Wrapping::None,
        })
    }

    fn cursor_x(paragraph: &Paragraph, index: usize) -> f32 {
        paragraph.grapheme_position(0, index).unwrap().x
    }

    /// The vendored iced_graphics places the cursor by the character's
    /// own direction (vendor/PATCHES.md).
    #[test]
    fn cursor_follows_the_text_direction() {
        let english = paragraph("abc");
        assert_eq!(cursor_x(&english, 0), 0.0);
        assert!(cursor_x(&english, 1) < cursor_x(&english, 2));
        assert!(cursor_x(&english, 2) < cursor_x(&english, 3));

        // Right to left text, in any right to left script: the start is
        // on the right, and each character further in is further left.
        // Arabic letters join, which must not change that.
        for sample in ["שלום", "سلام", "ܫܠܡܐ"] {
            let right_to_left = paragraph(sample);
            let count = sample.chars().count();
            let positions: Vec<f32> = (0..=count)
                .map(|index| cursor_x(&right_to_left, index))
                .collect();
            assert!(
                positions.windows(2).all(|pair| pair[0] > pair[1]),
                "{sample}: {positions:?}"
            );
            assert!(
                positions[count].abs() < 0.5,
                "{sample}: the end is at the left edge: {positions:?}"
            );
        }
    }
}
