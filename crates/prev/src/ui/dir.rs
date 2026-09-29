//! Layout in the interface's reading direction. In right to left
//! languages the insides of panels, dialogs and menus are mirrored: rows
//! run from the right, and columns and text start on the right. The
//! window's own layout is not: its bars and the places of its side panels
//! stay as in left to right languages (see [`fixed`]). Documents, images
//! and their thumbnails are never mirrored.
//!
//! Interface code builds rows and columns with this module's `row!`,
//! `column!` and [`row`] instead of iced's.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

use iced::alignment::Vertical;
use iced::widget::{Column, Row, text};
use iced::{Alignment, Element, Length, Padding, Pixels};

static RIGHT_TO_LEFT: AtomicBool = AtomicBool::new(false);

/// Called when the interface's language changes.
pub fn set_right_to_left(right_to_left: bool) {
    RIGHT_TO_LEFT.store(right_to_left, Ordering::Relaxed);
}

thread_local! {
    /// Set while a bar is made, whose layout is never mirrored.
    static FIXED: Cell<bool> = const { Cell::new(false) };
}

/// Whether the interface's language reads right to left. Text follows
/// this everywhere, bars included.
pub fn rtl() -> bool {
    RIGHT_TO_LEFT.load(Ordering::Relaxed)
}

/// Whether layout made now is mirrored: in right to left languages,
/// except inside [`fixed`].
fn mirrored() -> bool {
    rtl() && !FIXED.get()
}

/// Until the returned guard drops, rows, columns and paddings are made as
/// in left to right languages, for bars and the window's arrangement of
/// panels. Text keeps its alignment. Must be held while the elements are
/// made, such as inside a `responsive` closure.
#[must_use]
pub fn fixed() -> Scope {
    Scope::set(true)
}

/// Undoes [`fixed`] until the returned guard drops, for a menu made inside
/// a bar, whose items read in the interface's direction.
#[must_use]
pub fn reading() -> Scope {
    Scope::set(false)
}

/// Restores the layout direction when dropped.
pub struct Scope {
    previous: bool,
}

impl Scope {
    fn set(fixed: bool) -> Self {
        Self {
            previous: FIXED.replace(fixed),
        }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        FIXED.set(self.previous);
    }
}

/// The side lines start on: left, or right in right to left languages.
pub fn start() -> Alignment {
    if mirrored() {
        Alignment::End
    } else {
        Alignment::Start
    }
}

/// The side lines end on.
pub fn end() -> Alignment {
    if mirrored() {
        Alignment::Start
    } else {
        Alignment::End
    }
}

/// Text alignment for interface text: its start side, right in right to
/// left languages. Left to right keeps iced's default.
pub fn text_start() -> text::Alignment {
    if rtl() {
        text::Alignment::Right
    } else {
        text::Alignment::Default
    }
}

/// The start side for widgets that take a horizontal alignment, such as
/// text inputs.
pub fn horizontal_start() -> iced::alignment::Horizontal {
    if rtl() {
        iced::alignment::Horizontal::Right
    } else {
        iced::alignment::Horizontal::Left
    }
}

/// Whether `text` reads right to left: its first letter with a direction
/// is in a right to left script (Hebrew, Arabic, Syriac, Thaana, N'Ko,
/// Samaritan, Mandaic and the historic ones). For text from documents,
/// which keeps its own direction whatever the interface's.
pub fn text_is_rtl(text: &str) -> bool {
    text_direction(text) == Some(true)
}

/// The direction of `text` by its first letter with one: right to left
/// (`true`) or left to right, or `None` when it has no letters, such as
/// an empty string or a number.
pub fn text_direction(text: &str) -> Option<bool> {
    text.chars().find_map(|c| {
        let code = c as u32;
        if matches!(
            code,
            0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF | 0x10800..=0x10FFF | 0x1E800..=0x1EFFF
        ) {
            Some(true)
        } else if c.is_alphabetic() {
            Some(false)
        } else {
            None
        }
    })
}

static INPUT_RIGHT_TO_LEFT: AtomicBool = AtomicBool::new(false);

/// Called when the input language, or the keyboard layout it follows,
/// changes.
pub fn set_input_right_to_left(right_to_left: bool) {
    INPUT_RIGHT_TO_LEFT.store(right_to_left, Ordering::Relaxed);
}

/// Whether the language the user types in is written right to left.
pub fn input_rtl() -> bool {
    INPUT_RIGHT_TO_LEFT.load(Ordering::Relaxed)
}

/// Where a text field's text sits: on the side its text starts on, or
/// while it has none, the side the input language starts on.
pub fn input_align(value: &str) -> iced::alignment::Horizontal {
    if text_direction(value).unwrap_or_else(input_rtl) {
        iced::alignment::Horizontal::Right
    } else {
        iced::alignment::Horizontal::Left
    }
}

/// Padding given by the start and end sides instead of left and right.
pub fn padding(top: f32, end: f32, bottom: f32, start: f32) -> Padding {
    let (left, right) = if mirrored() {
        (end, start)
    } else {
        (start, end)
    };
    Padding {
        top,
        right,
        bottom,
        left,
    }
}

/// A row whose first child is at the start: on the right in right to left
/// languages.
pub fn row<'a, Message>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Row<'a, Message> {
    let mut children: Vec<Element<'a, Message>> = children.into_iter().collect();
    if mirrored() {
        children.reverse();
    }
    Row::with_children(children)
}

/// A column whose children line up on the start side.
pub fn column<'a, Message>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).align_x(start())
}

/// A row built up with `push`, for rows whose children depend on state:
/// iced's `push` always adds on the right, so the children are kept until
/// the row is made, then laid out in the reading direction.
pub struct Line<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: Option<Pixels>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    align_y: Option<Vertical>,
}

impl<'a, Message: 'a> Line<'a, Message> {
    pub fn new(children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            align_y: None,
        }
    }

    /// Adds `child` at the end: after the others in reading order.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn align_y(mut self, align: impl Into<Vertical>) -> Self {
        self.align_y = Some(align.into());
        self
    }

    /// The row, laid out in the reading direction.
    pub fn build(self) -> Row<'a, Message> {
        let mut row = row(self.children);
        if let Some(spacing) = self.spacing {
            row = row.spacing(spacing);
        }
        if let Some(padding) = self.padding {
            row = row.padding(padding);
        }
        if let Some(width) = self.width {
            row = row.width(width);
        }
        if let Some(height) = self.height {
            row = row.height(height);
        }
        if let Some(align) = self.align_y {
            row = row.align_y(align);
        }
        row
    }
}

impl<'a, Message: 'a> From<Line<'a, Message>> for Element<'a, Message> {
    fn from(line: Line<'a, Message>) -> Self {
        line.build().into()
    }
}

/// A [`Line`], written like `row!`.
#[macro_export]
macro_rules! line {
    () => {
        $crate::ui::dir::Line::new(::std::vec::Vec::<::iced::Element<'_, _>>::new())
    };
    ($($child:expr),+ $(,)?) => {
        $crate::ui::dir::Line::new([$(::iced::Element::from($child)),+])
    };
}

/// iced's `row!`, running from the right in right to left languages.
#[macro_export]
macro_rules! row {
    () => {
        $crate::ui::dir::row(::std::vec::Vec::<::iced::Element<'_, _>>::new())
    };
    ($($child:expr),+ $(,)?) => {
        $crate::ui::dir::row([$(::iced::Element::from($child)),+])
    };
}

/// iced's `column!`, with children on the start side.
#[macro_export]
macro_rules! column {
    () => {
        $crate::ui::dir::column(::std::vec::Vec::<::iced::Element<'_, _>>::new())
    };
    ($($child:expr),+ $(,)?) => {
        $crate::ui::dir::column([$(::iced::Element::from($child)),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_is_the_first_letter_s() {
        for right_to_left in ["שלום", "سلام", "سلام", "ܫܠܡܐ", "ދިވެހި", "(12) مرحبا"]
        {
            assert_eq!(text_direction(right_to_left), Some(true), "{right_to_left}");
        }
        for left_to_right in ["hello", "Привет", "你好", "こんにちは", "12 abc"] {
            assert_eq!(
                text_direction(left_to_right),
                Some(false),
                "{left_to_right}"
            );
        }
        assert_eq!(text_direction(""), None);
        assert_eq!(text_direction("42 %"), None);
    }

    #[test]
    fn fields_follow_their_text_then_the_input_language() {
        use iced::alignment::Horizontal;
        set_input_right_to_left(true);
        assert_eq!(input_align(""), Horizontal::Right);
        assert_eq!(input_align("abc"), Horizontal::Left);
        set_input_right_to_left(false);
        assert_eq!(input_align(""), Horizontal::Left);
        assert_eq!(input_align("مرحبا"), Horizontal::Right);
    }
}
