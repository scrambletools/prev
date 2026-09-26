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
    ArrowRightAlt = 0xe941, "arrow_right_alt";
    ArrowSelector = 0xf82f, "arrow_selector_tool";
    AutoStories = 0xe666, "auto_stories";
    Bookmark = 0xe8e7, "bookmark";
    BookmarkAdd = 0xe598, "bookmark_add";
    Bookmarks = 0xe98b, "bookmarks";
    BorderColor = 0xe22b, "border_color";
    BrokenImage = 0xe3ad, "broken_image";
    ChatBubble = 0xe0cb, "chat_bubble";
    Check = 0xe668, "check";
    Circle = 0xef4a, "circle";
    Close = 0xe5cd, "close";
    Compress = 0xe94d, "compress";
    ContentCopy = 0xe14d, "content_copy";
    ContentPaste = 0xe14f, "content_paste";
    Crop = 0xe3be, "crop";
    CropFree = 0xe3c2, "crop_free";
    DarkMode = 0xe51c, "dark_mode";
    Delete = 0xe92e, "delete";
    Draft = 0xe66d, "draft";
    Draw = 0xe746, "draw";
    Edit = 0xf097, "edit";
    EditDocument = 0xf88c, "edit_document";
    Encrypted = 0xe593, "encrypted";
    Error = 0xf8b6, "error";
    ExpandLess = 0xe5ce, "expand_less";
    ExpandMore = 0xe5cf, "expand_more";
    FileExport = 0xf3b2, "file_export";
    FileOpen = 0xeaf3, "file_open";
    FitPage = 0xf77a, "fit_page";
    FitScreen = 0xea10, "fit_screen";
    FitWidth = 0xf779, "fit_width";
    Flip = 0xe3e8, "flip";
    FlipVertical = 0xe005, "flip rotated a quarter turn";
    FolderOpen = 0xe2c8, "folder_open";
    FormatAlignCenter = 0xe234, "format_align_center";
    FormatAlignLeft = 0xe236, "format_align_left";
    FormatAlignRight = 0xe237, "format_align_right";
    FormatColorFill = 0xe23a, "format_color_fill";
    FormatColorText = 0xe23c, "format_color_text";
    FormatStrikethrough = 0xe246, "format_strikethrough";
    FormatUnderlined = 0xe249, "format_underlined";
    Gesture = 0xe155, "gesture";
    GridView = 0xe9b0, "grid_view";
    Hexagon = 0xeb39, "hexagon";
    HighlightAlt = 0xef52, "highlight_alt";
    History = 0xe8b3, "history";
    HorizontalRule = 0xf108, "horizontal_rule";
    Image = 0xe3f4, "image";
    Info = 0xe88e, "info";
    InkHighlighter = 0xe6d1, "ink_highlighter";
    KeyboardArrowDown = 0xe313, "keyboard_arrow_down";
    KeyboardArrowUp = 0xe316, "keyboard_arrow_up";
    LeftPanelClose = 0xf717, "left_panel_close";
    LeftPanelOpen = 0xf716, "left_panel_open";
    LightMode = 0xe518, "light_mode";
    LineStyle = 0xe919, "line_style";
    LineWeight = 0xe91a, "line_weight";
    LocationOff = 0xe0c7, "location_off";
    LocationOn = 0xf1db, "location_on";
    Lock = 0xe899, "lock";
    Loupe = 0xe402, "loupe";
    MoreVert = 0xe5d4, "more_vert";
    NoteAdd = 0xe89c, "note_add";
    OneToOne = 0xefcd, "1x_mobiledata";
    Photo = 0xe412, "photo_camera";
    PhotoLibrary = 0xe413, "photo_library";
    PictureAsPdf = 0xe415, "picture_as_pdf";
    Print = 0xe8ad, "print";
    Rectangle = 0xeb54, "rectangle";
    Redo = 0xe15a, "redo";
    Remove = 0xe15b, "remove";
    RemoveSelection = 0xe9d5, "remove_selection";
    ResetAll = 0xf053, "restart_alt";
    Resize = 0xf707, "resize";
    RotateLeft = 0xe419, "rotate_left";
    RotateRight = 0xe41a, "rotate_right";
    RoundedCorner = 0xe920, "rounded_corner";
    Search = 0xef7a, "search";
    SelectAll = 0xe162, "select_all";
    Sell = 0xf05b, "sell";
    Settings = 0xe8b8, "settings";
    Shapes = 0xe602, "shapes";
    Signature = 0xf74c, "signature";
    Stacks = 0xf500, "stacks";
    Star = 0xf09a, "star";
    StickyNote = 0xf1fc, "sticky_note_2";
    TextFields = 0xe262, "text_fields";
    TextFormat = 0xe165, "text_format";
    Toc = 0xe8de, "toc";
    Tune = 0xe429, "tune";
    TwoPager = 0xf51f, "two_pager";
    Undo = 0xe166, "undo";
    Upload = 0xf09b, "upload";
    ViewAgenda = 0xe8e9, "view_agenda";
    ViewDay = 0xe8ed, "view_day";
    Vignette = 0xe435, "vignette";
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
