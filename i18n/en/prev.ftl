# prev's interface text in English, the source for every other language.
#
# Each line is `key = text`. Keys stay the same in every language; only the
# text after `=` is translated. `{ $name }` is a value prev fills in, such
# as a page number or a file name; keep it, and move it where the
# language needs it. Plural and other variants use Fluent's selectors:
# https://projectfluent.org/fluent/guide/selectors.html
#
# Sections follow the parts of the interface. Keys shared by several
# parts are under "Common".

## Language

# This language's name in itself, as the Settings language list shows it,
# such as English, Deutsch or עברית.
language-name = English

## Common

common-cancel = Cancel
common-close = Close
common-save = Save

## Settings

settings-title = Settings
settings-appearance = Appearance
settings-colors = Colors
settings-windows = Windows
settings-storage = Storage

## Markup toolbar

markup-tool-select = Select
markup-tool-area = Rectangular selection
markup-tool-sketch = Sketch
markup-tool-draw = Draw
markup-tool-shapes = Shapes
markup-tool-text-box = Text box
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Highlight
markup-tool-note = Note
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Sign
# A verb: the tool that marks areas to black out.
markup-tool-redact = Redact
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Apply
markup-apply-redactions = Apply redactions
markup-shape-style = Shape style
markup-border-color = Border color
markup-fill-color = Fill color
markup-text-style = Text style
markup-delete = Delete
markup-undo = Undo
markup-redo = Redo

## Markup menus

markup-shape-rectangle = Rectangle
markup-shape-rounded-rectangle = Rounded Rectangle
markup-shape-oval = Oval
markup-shape-line = Line
markup-shape-arrow = Arrow
markup-shape-star = Star
markup-shape-polygon = Polygon
markup-shape-speech-bubble = Speech Bubble
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Loupe
# A shape that darkens the page around it.
markup-shape-mask = Mask
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Highlight
markup-style-underline = Underline
markup-style-strikethrough = Strikethrough
markup-style-squiggly = Squiggly
# Menu section headings.
markup-menu-color = Color
markup-menu-font = Font
markup-menu-size = Size
markup-menu-alignment = Alignment
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Dashed

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Note
markup-kind-text-box = Text box
markup-kind-stamp = Stamp
markup-kind-redaction = Redaction
markup-kind-shape = Shape
# Tooltips on a note being edited.
markup-note-delete = Delete note
markup-note-done = Done
markup-note-placeholder = Type a note
markup-notes-empty = No highlights or notes
markup-notes-empty-hint = Highlights, notes and text boxes appear here.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Page { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Could not change the document: { $error }
markup-copy-area-failed = Could not copy the area: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = the document closed
markup-render-area-failed = could not render the area
markup-copy-stopped = copying stopped

## Signatures

signature-menu-empty = No signatures yet.
signature-delete = Delete signature
signature-create = Create Signature…
signature-dialog-title = Create Signature
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Draw
signature-tab-type = Type
signature-tab-image = Image
signature-draw-hint = Sign with your mouse, pen or touchpad on the line.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Your name
signature-image-hint = Choose a photo or scan of your signature on white paper.
signature-choose-image = Choose Image…
# Placeholder of the field naming the signature in the library.
signature-description = Description, such as Full name or Initials
# Clears the drawing, typed name or image.
signature-clear = Clear
# The color the signature is drawn or typed in.
signature-ink = Ink
# The pen's width, for drawing.
signature-thickness = Thickness
signature-sign-first = Sign first, then save.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Signature { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Could not change signatures: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = no data folder: HOME is not set
signature-removing-stopped = removing stopped
signature-saving-stopped = saving stopped
signature-reading-stopped = reading stopped
signature-not-an-image = that file is not an image prev can read
signature-no-frames = the image has no frames
signature-not-found = no signature found in the image

## Dragging

drag-pages-need-document = Pages can be dropped on a document.
drag-image-unsupported = prev can't open this image.
# $error is a lowercase reason or a technical message.
drag-area-failed = Could not drag the area: { $error }
drag-pages-failed = Could not drag the pages: { $error }
drag-start-failed = Could not start dragging.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Pages
drag-file-one-page = { $name } (page { $page })
drag-file-page-range = { $name } (pages { $first }–{ $last })

## PDF window

pdf-opening = Opening…
pdf-open-failed = prev can't open this document
pdf-no-pages = The document has no pages.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = the document closed
pdf-keep-original-failed = could not keep the original version: { $error }
pdf-save-failed = Could not save: { $error }
pdf-nothing-to-paste = There is nothing to paste.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = pasting stopped
pdf-file-dialog-failed = Could not show the file dialog: { $error }
pdf-bookmarks-no-home = Bookmarks cannot be saved: HOME is not set
pdf-bookmarks-save-failed = Could not save bookmarks: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Page { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” is password protected
pdf-password = Password
pdf-password-wrong = Incorrect password. Try again.
# Button that opens a locked document.
pdf-unlock = Unlock

# Toolbar tooltips and labels.
pdf-sidebar = Sidebar
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = of { $count }
pdf-zoom-out = Zoom out
pdf-zoom-in = Zoom in
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Fit page
pdf-fit-width = Fit width
pdf-actual-size = Actual size
pdf-view-continuous = Continuous scroll
pdf-view-single-page = Single page
pdf-view-two-pages = Two pages
pdf-undo = Undo
pdf-redo = Redo
pdf-rotate-left = Rotate left
pdf-rotate-right = Rotate right
pdf-inspector = Inspector
pdf-markup = Markup
# Tooltip of the button that opens the export dialog.
pdf-export = Export
pdf-settings = Settings

# Search field.
pdf-search = Search
pdf-search-not-found = Not found
pdf-searching = Searching…
# The match shown, of all matches found.
pdf-search-match = { $current } of { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } of { $total }+

# Inspector: section headings.
pdf-inspector-file = File
pdf-inspector-document = Document
pdf-inspector-pages = Pages
# Inspector: fact labels and values.
pdf-inspector-title = Title
pdf-inspector-author = Author
pdf-inspector-subject = Subject
pdf-inspector-keywords = Keywords
pdf-inspector-created = Created
pdf-inspector-modified = Modified
pdf-inspector-application = Application
pdf-inspector-producer = PDF producer
pdf-inspector-version = Version
pdf-inspector-security = Security
pdf-inspector-not-encrypted = Not encrypted
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Encrypted ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } page
   *[other] { $count } pages
}
pdf-inspector-page-size = Page size
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } in)
pdf-loading = Loading…

# Sidebar tabs and lists.
pdf-tab-pages = Pages
pdf-tab-contents = Contents
pdf-tab-notes = Highlights and notes
pdf-tab-bookmarks = Bookmarks
pdf-no-outline = No table of contents
pdf-no-outline-detail = This document has no outline.
pdf-no-bookmarks = No bookmarks
pdf-no-bookmarks-detail = Press Ctrl+D to bookmark a page.
pdf-remove-bookmark = Remove bookmark

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Pages
pages-insert-blank = Insert Blank Page
pages-insert-file = Insert from File…
pages-copy = { $count ->
    [one] Copy Page
   *[other] Copy Pages
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Paste Page
   *[other] Paste { $count } Pages
}
pages-crop = Crop to Selection
pages-select-all = Select All Pages
pages-delete = { $count ->
    [one] Delete Page
   *[other] Delete Pages
}
pages-apply-redactions = Apply Redactions…
pages-no-copied = There are no copied pages to paste.
pages-copied = { $count ->
    [one] Copied { $count } page.
   *[other] Copied { $count } pages.
}
pages-copy-failed = Could not copy the pages: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = reading stopped
pages-read-failed = Could not read the file: { $error }
pages-at-least-one = A document needs at least one page.
pages-crop-needs-area = Choose an area with the rectangular selection tool first.
pages-change-failed = Could not change the pages: { $error }
pages-no-redactions = There were no redactions to apply.
pages-redactions-applied = { $count ->
    [one] Applied { $count } redaction.
   *[other] Applied { $count } redactions.
}
pages-forget-versions-failed = Could not delete earlier versions: { $error }
pages-redact-title = Apply redactions?
pages-redact-body = { $count ->
    [one] Text, images and drawings under the mark are removed from the document for good, and the marks become black boxes. This can't be undone, and the earlier versions of this file that prev keeps are deleted.
   *[other] Text, images and drawings under the { $count } marks are removed from the document for good, and the marks become black boxes. This can't be undone, and the earlier versions of this file that prev keeps are deleted.
}
# Button that applies redactions.
pages-redact-apply = Apply

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Export
pages-export-format = Format
pages-export-reduce = Reduce file size (images at 150 dpi)
pages-export-flatten = Flatten annotations and form fields
pages-export-flatten-detail = Markup and filled-in fields become part of the pages and can no longer be edited. Redaction marks not yet applied are left out.
pages-export-encrypt = Encrypt with a password
pages-export-password = Password
pages-export-verify-password = Verify password
pages-export-resolution = Resolution
pages-export-dpi = { $dpi } dpi
pages-export-quality = Quality
# JPEG quality choices.
pages-export-quality-low = Low
pages-export-quality-medium = Medium
pages-export-quality-high = High
pages-export-quality-best = Best
pages-export-one-file = All pages go into one file.
pages-export-file-per-page = Each page is saved as its own file, numbered after the name you choose.
pages-export-selected-only = { $count ->
    [one] Only the selected page
   *[other] Only the { $count } selected pages
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Export…
pages-export-no-password = Enter a password.
pages-export-password-mismatch = The passwords don't match.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (exported)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = document
pages-export-same-file = Export to a new file; this document saves itself.
pages-export-exporting = Exporting “{ $name }”…
pages-export-done = Exported “{ $name }”.
pages-export-done-images = Exported { $count } images.
pages-export-failed = Could not export: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = exporting stopped

## Start window

# Under the app name in a window with no file open.
app-start-hint = Open or drop a PDF, image, SVG or Markdown file.
app-start-open = Open…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (dev)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: this viewer is not built yet.
app-cannot-open = prev can't open this kind of file.
app-cannot-read = prev can't read this file: { $error }
app-kind-pdf = PDF document
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format } image
app-kind-svg = SVG drawing
app-kind-markdown = Markdown document
app-file-dialog-failed = Could not show the file dialog: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Open
action-settings = Settings

## Toolbar

app-toolbar-keep-shown = Keep the toolbar shown
app-toolbar-auto-hide = Hide the toolbar when the pointer leaves
# The button that shows the toolbar's hidden tools.
app-toolbar-more = More

## File facts

# Labels in a file's inspector.
app-fact-name = Name
app-fact-folder = Folder
app-fact-size = Size
app-fact-modified = Modified
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } bytes
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Invalid link { $uri }: { $error }
app-link-open-failed = Could not open { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = install wl-clipboard to paste images
app-copy-needs-wl-clipboard = install wl-clipboard to copy images
app-copy-no-pixels = the area has no pixels
# wl-copy is a program's name.
app-copy-no-input = wl-copy has no input
app-copy-failed = wl-copy failed
app-clipboard-open-failed = Could not open the clipboard: { $error }
app-copy-image-failed = Could not copy the image: { $error }

## Printing

print-failed = Could not print: { $error }
print-stopped = Printing stopped
print-unavailable = Printing is not available on this system yet.
print-no-window = Could not print: no window to show the print dialog over
print-dialog-failed = Could not show the print dialog: { $error }
# Shown after "Could not print:".
print-job-not-started = the printer did not start the job
# Shown after "Could not print:".
print-printer-stopped = the printer stopped

## File dialogs

dialog-open = Open
dialog-filter-all = All supported files
dialog-filter-pdf = PDF documents
dialog-filter-images = Images
dialog-filter-svg = SVG drawings
dialog-filter-markdown = Markdown
dialog-choose-signatures = Choose the signatures folder
dialog-choose-versions = Choose the version history folder
dialog-choose-bookmarks = Choose the bookmarks file

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Usage: prev [FILE]...

    View and edit PDFs and images. Files open in windows of the running prev,
    which starts if needed.

    Options:
      -h, --help     Show this help
      -V, --version  Show the version

## Settings, continued

settings-language = Language
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = System default ({ $language })
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Input language
settings-input-language-system = Follow the keyboard layout
settings-input-language-note = Sets the side an empty text field starts on. Text you type keeps its own direction.

settings-appearance-system = System
settings-appearance-light = Light
settings-appearance-dark = Dark
settings-omarchy-accent = Use Omarchy accent color
# $theme is the Omarchy theme's name.
settings-omarchy-note = Colors are built from the accent of “{ $theme }”.
settings-omarchy-none = No Omarchy theme is active.
settings-auto-hide = Hide the toolbar when the pointer leaves
settings-auto-hide-note = The toolbar floats over the document and slides away while the pointer is outside the window.
settings-animations = Animations
settings-animations-note = Sliding bars and panels, growing dialogs and springy buttons.
settings-animations-reduced = Off while the system asks for reduced motion.
settings-corner-radius = Corner radius
settings-corner-radius-note = For dialogs and the floating toolbar.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Overlay transparency
settings-overlay-note = How much of the page shows through the floating toolbar.
settings-overlay-value = { $percent }%
settings-storage-signatures = Signatures folder
settings-storage-versions = Version history folder
settings-storage-bookmarks = Bookmarks file
settings-storage-apply = Apply
settings-storage-choose = Choose…
# $file is where the settings file is.
settings-storage-note = Files already kept at an old place stay there; move them over to keep using them. prev app settings are saved in { $file }.
settings-save-failed = Could not save settings: { $error }
settings-no-location = No settings location: HOME is not set
settings-full-path = Use a full path, such as ~/Documents/prev.
settings-path-is-folder = { $path } is a folder, not a file.
settings-folder-missing = There is no folder { $path }. Create it first, or choose one.
settings-path-is-file = { $path } is a file, not a folder.
settings-cannot-write = prev can't write in { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Export
# Section headings in the export dialog.
export-format = Format
export-quality = Quality
export-size = Size
# Button that goes on to choose where to save the export.
export-choose = Export…
# Format choice; the format name stays as it is.
export-format-webp = WebP (lossless)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = image
# JPEG quality choices.
export-quality-low = Low
export-quality-medium = Medium
export-quality-high = High
export-quality-best = Best
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Actual size
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } pixels
# $error is the system's reason.
export-dialog-failed = Could not show the save dialog: { $error }
# $path is where the file was saved.
export-done = Exported { $path }
export-failed = Could not export: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = exporting stopped

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Images with markup can't be edited. Export to keep the markup, or delete it and close the markup bar.

# Shown if a background task ends unexpectedly.
image-loading-stopped = loading stopped
image-reverting-stopped = reverting stopped
image-rendering-stopped = rendering stopped
image-saving-stopped = saving stopped
image-markup-stopped = the markup stopped
image-no-version-store = No place to keep versions
image-revert-failed = Could not revert: { $error }
image-read-failed = Could not read { $path }: { $error }
image-keep-original-failed = Could not keep the original version: { $error }
image-save-failed = Could not save { $path }: { $error }
image-markup-start-failed = Could not start the markup: { $error }
image-cannot-edit = Animations and SVG drawings can't be edited.
image-cannot-mark-up = Animations and SVG drawings can't be marked up.
image-mark-up-wait = Wait for the edit to finish, then mark up.
image-crop-needs-selection = Drag a selection first (Select tool), then crop.
image-size-needed = Enter a width and height in pixels.
# $name is a file name.
image-cannot-save-format = Changes to “{ $name }” can't be saved in its format. Use Export (Ctrl+Shift+S).
image-cannot-export-animation = Animations can't be exported yet.
image-drop-pages = Pages can be dropped on a document.
image-drag-failed = Could not start dragging.
image-open-failed = prev can't open this image
image-opening = Opening…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = The name doesn't match the format
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” will be saved as a { $format } file, but its name ends in .{ $extension }. Other apps may not open it.
image-name-mismatch-no-extension = “{ $name }” will be saved as a { $format } file, but its name has no extension. Other apps may not open it.
image-choose-again = Choose Again
image-save-as-is = Save as Is
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = frame { $current } of { $total }
image-position = { $current } of { $total }
image-edited = edited
# Toolbar tooltips.
image-sidebar = Sidebar
image-zoom-out = Zoom out
image-zoom-in = Zoom in
image-zoom = { $percent }%
image-fit = Fit to window
image-actual-size = Actual size
image-undo = Undo
image-redo = Redo
image-rotate-left = Rotate left
image-rotate-right = Rotate right
image-flip-horizontal = Flip horizontal
image-flip-vertical = Flip vertical
image-select = Rectangular selection
image-crop = Crop to selection
image-adjust-size-tool = Adjust size
image-adjust-color-tool = Adjust color
# Tooltip and panel title.
image-inspector = Inspector
image-markup = Markup
image-export = Export
image-settings = Settings
# Panel titles.
image-adjust-color = Adjust Color
image-adjust-size = Adjust Size
# Adjust Color sliders.
image-exposure = Exposure
image-contrast = Contrast
image-saturation = Saturation
image-temperature = Temperature
image-tint = Tint
image-sepia = Sepia
image-sharpness = Sharpness
image-levels = Levels
image-black-point = Black point
image-midtones = Midtones
image-white-point = White point
image-reset-all = Reset All
# Adjust Size panel.
image-current-size = Current size: { $width } × { $height } pixels
image-width = Width
image-height = Height
image-scale-proportionally = Scale proportionally
# Button that applies the new size.
image-resize = Resize
# Inspector panel.
image-inspector-loading = Loading…
image-file = File
image-format = Format
image-dimensions-label = Dimensions
image-pixels = { $width } × { $height } pixels
image-no-camera = No camera information.
image-location = Location
image-remove-location = Remove Location Info
image-no-location = No location information.
image-keywords-description = Keywords and Description
image-keywords-hint = Keywords, separated by commas
image-description = Description
image-keywords-unsupported = Keywords can be saved in JPEG, PNG and WebP files.
# Heading over the earlier versions of the file.
image-revert-to = Revert To
image-no-versions = No earlier versions.
image-revert = Revert
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Close without exporting the markup?
image-close-body = { $count ->
    [one] Markup on an image lasts only while its window is open. Export the image to keep it: the markup is drawn into the copy you save.
   *[other] Markup on images lasts only while their window is open. Export each image to keep it: the markup is drawn into the copy you save.
}
image-close-anyway = Close Anyway

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = reading stopped
markdown-read-failed = prev can't read this file
markdown-draw-failed = Could not draw the document
# Under the export's size choices.
markdown-export-size = The whole document, { $width } × { $height } pixels
# Search results.
markdown-not-found = Not found
markdown-match = { $current } of { $total }
# Placeholder of the search field.
markdown-search = Search
# Toolbar tooltips.
markdown-smaller-text = Smaller text
markdown-larger-text = Larger text
markdown-zoom = { $percent }%
markdown-actual-size = Actual size
# Tooltip and panel title.
markdown-inspector = Inspector
markdown-export = Export
markdown-settings = Settings
# Inspector headings and labels.
markdown-file = File
markdown-document = Document
markdown-words = Words
markdown-lines = Lines
markdown-pictures = Pictures

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Camera
image-meta-exposure = Exposure
image-meta-image = Image
image-meta-make = Make
image-meta-model = Model
image-meta-lens = Lens
image-meta-exposure-time = Exposure time
# The lens aperture, written like f/2.8.
image-meta-f-number = F-number
image-meta-iso = ISO
image-meta-focal-length = Focal length
image-meta-exposure-bias = Exposure bias
image-meta-flash = Flash
image-meta-date-taken = Date taken
image-meta-orientation = Orientation
image-meta-color-space = Color space
image-meta-software = Software
image-meta-artist = Artist
image-meta-copyright = Copyright
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Mirrored horizontally
    [3] Rotated 180°
    [4] Mirrored vertically
    [5] Mirrored horizontally, rotated 90° counterclockwise
    [6] Rotated 90° clockwise
    [7] Mirrored horizontally, rotated 90° clockwise
    [8] Rotated 90° counterclockwise
   *[other] Unknown ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Fired
   *[no] Did not fire
}{ $mode ->
    [on] , forced on
    [off] , off
    [auto] , auto
   *[unknown] {""}
}{ $redeye ->
    [yes] , red-eye reduction
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Uncalibrated
   *[other] Other ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = cannot open document: { $detail }
error-pdf-page-out-of-range = page { $page } does not exist
error-pdf-password-protected = the document is password protected; open it and copy its pages instead
error-pdf-no-pages = no pages to extract
error-pdf-crop-outside = the crop area is outside the page
error-pdf-closed = document closed
error-pdf-saved-unreadable = the saved document no longer opens
error-image-read = cannot read the file: { $detail }
error-image-invalid = the image is damaged or invalid: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = opening this format needs { $library }, which is not installed
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format } images are not supported yet
error-image-encode = cannot encode the image: { $detail }
error-exif-malformed = the EXIF data is malformed
error-settings-read = cannot read settings: { $detail }
error-settings-invalid = invalid settings: { $detail }
error-remove-location = could not remove the location: { $error }
error-location-unsupported = location info can be removed from JPEG, PNG, WebP and TIFF files
error-xmp-unsupported = keywords and descriptions can only be saved in JPEG, PNG and WebP files

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = Camera RAW
