//! Material Symbols Rounded icons. `scripts/build-fonts.py` reads the
//! table below to subset the icon fonts, so every icon prev draws must be
//! listed here, with its Material Symbols name.

use iced::widget::{Text, text};
use iced::{Font, Pixels};

use super::font;

macro_rules! icons {
    ($($variant:ident = $codepoint:literal, $name:literal;)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Icon {
            $($variant,)*
        }

        impl Icon {
            pub fn codepoint(self) -> char {
                match self {
                    $(Icon::$variant => char::from_u32($codepoint).unwrap_or('?'),)*
                }
            }
        }
    };
}

icons! {
    Add = 0xe145, "add";
    ArrowBack = 0xe5c4, "arrow_back";
    ArrowDropDown = 0xe5c5, "arrow_drop_down";
    ArrowForward = 0xe5c8, "arrow_forward";
    AutoStories = 0xe666, "auto_stories";
    Bookmark = 0xe8e7, "bookmark";
    BookmarkAdd = 0xe598, "bookmark_add";
    Bookmarks = 0xe98b, "bookmarks";
    BrokenImage = 0xe3ad, "broken_image";
    Check = 0xe668, "check";
    Close = 0xe5cd, "close";
    Crop = 0xe3be, "crop";
    DarkMode = 0xe51c, "dark_mode";
    Draft = 0xe66d, "draft";
    Error = 0xf8b6, "error";
    ExpandLess = 0xe5ce, "expand_less";
    ExpandMore = 0xe5cf, "expand_more";
    FileExport = 0xf3b2, "file_export";
    FitPage = 0xf77a, "fit_page";
    FitScreen = 0xea10, "fit_screen";
    FitWidth = 0xf779, "fit_width";
    Flip = 0xe3e8, "flip";
    FlipVertical = 0xe005, "flip rotated a quarter turn";
    FolderOpen = 0xe2c8, "folder_open";
    GridView = 0xe9b0, "grid_view";
    HighlightAlt = 0xef52, "highlight_alt";
    History = 0xe8b3, "history";
    Image = 0xe3f4, "image";
    Info = 0xe88e, "info";
    KeyboardArrowDown = 0xe313, "keyboard_arrow_down";
    KeyboardArrowUp = 0xe316, "keyboard_arrow_up";
    LeftPanelClose = 0xf717, "left_panel_close";
    LeftPanelOpen = 0xf716, "left_panel_open";
    LightMode = 0xe518, "light_mode";
    LocationOff = 0xe0c7, "location_off";
    LocationOn = 0xf1db, "location_on";
    Lock = 0xe899, "lock";
    OneToOne = 0xefcd, "1x_mobiledata";
    Photo = 0xe412, "photo_camera";
    Print = 0xe8ad, "print";
    Redo = 0xe15a, "redo";
    Remove = 0xe15b, "remove";
    ResetAll = 0xf053, "restart_alt";
    RotateLeft = 0xe419, "rotate_left";
    RotateRight = 0xe41a, "rotate_right";
    Resize = 0xf707, "resize";
    Search = 0xef7a, "search";
    Sell = 0xf05b, "sell";
    Settings = 0xe8b8, "settings";
    Toc = 0xe8de, "toc";
    Tune = 0xe429, "tune";
    TwoPager = 0xf51f, "two_pager";
    Undo = 0xe166, "undo";
    ViewAgenda = 0xe8e9, "view_agenda";
    ViewDay = 0xe8ed, "view_day";
    Warning = 0xf083, "warning";
    ZoomIn = 0xe8ff, "zoom_in";
    ZoomOut = 0xe900, "zoom_out";
}

/// An outlined icon at `size` pixels.
pub fn icon<'a>(icon: Icon, size: impl Into<Pixels>) -> Text<'a> {
    glyph(icon, size, font::ICONS)
}

/// A filled icon, used for selected states.
pub fn filled<'a>(icon: Icon, size: impl Into<Pixels>) -> Text<'a> {
    glyph(icon, size, font::ICONS_FILLED)
}

fn glyph<'a>(icon: Icon, size: impl Into<Pixels>, font: Font) -> Text<'a> {
    let size = size.into();
    text(icon.codepoint())
        .font(font)
        .size(size)
        .line_height(iced::widget::text::LineHeight::Absolute(size))
        .shaping(iced::widget::text::Shaping::Advanced)
        .center()
}
