# Changelog

All notable changes to prev. Versions follow
[Semantic Versioning](https://semver.org).

## [2.1.0] - 2026-10-03

### Changed

- The assistant no longer asks before signing, applying redactions or
  anything else: it does what you asked it in the chat. The switches in
  Settings that make prev ask first are for outside agents.
- The pointer stays the plain arrow over buttons, links, annotations and
  form fields, and while panning; it changes only to resize, to select
  text and to draw. prev no longer has hand cursors of its own.
- The assistant opens below the toolbar, where the inspector does, and
  takes its place: a window shows one panel on its right at a time, so
  opening the inspector or Adjust Color closes the assistant.
- In the toolbar the assistant button comes before the markup button,
  and undo and redo sit beside Export, keeping their place while the
  markup bar is open, so the toolbar no longer shifts as it opens.
- A selected annotation's box and handles follow its own outline, which
  a thick line straddles, for every kind of shape, ink and redaction
  mark, and a note's box is its icon.

### Fixed

- Dragging an annotation's handles hides it where it was and draws it as
  it will be: lines keep their width, text boxes wrap their text, and
  rounded corners and arrow heads keep their size.
- A loupe shows what is under it while it is moved or resized, and no
  longer magnifies itself.
- Dragging past the view's edge scrolls the page, and dragged
  annotations stay on their page.
- A note can be dragged to move it; a click still opens it.

## [2.0.0] - 2026-10-03

### Added

- The assistant: a chat panel beside the document, opened with the robot
  head on the toolbar, that answers about the file and works in it with
  prev's own tools, finding and pointing at things, marking up, filling
  in, editing, signing, redacting and exporting, asking first where
  Settings says to. Each window has its own chat, replies stream in as
  Markdown, models that think show their reasoning, and the model can be
  changed mid-chat.
- Settings' new Assistant tab adds models: a local model from a running
  Ollama or other OpenAI-compatible server on this computer or another,
  listed with what each can do, or a cloud model from Anthropic, OpenAI
  or Google Gemini with an API key, kept in the system keychain once per
  provider. Adding a model tries it first, and a problem, such as a key
  turned down or an account with no credit, says what to do.
- AI agents such as Claude Code control the prev you are using over MCP
  with `prev --mcp`: reading, showing, marking up, editing, signing,
  redacting and exporting in your own windows. prev asks before letting
  a new agent in, and Settings chooses which kinds of step ask first.
- The pointing hand over buttons and links is drawn as on macOS on Linux
  and Windows too, like the open and closed hands.

### Changed

- Settings is split into tabs: General, Appearance, Assistant, Agents
  and Storage.
- On Windows and macOS, toolbars, side panels, menus and dialogs take
  the system's title bar color, and on macOS prev's toolbar color runs
  up behind the title bar.
- Colors follow a change of the system accent color straight away.
- The table of contents is 15% closer from line to line.
- The program is larger, 58 MB on Linux, as it now carries the agent
  server and the assistant's connections to its models.
- prev needs Rust 1.95 to build.

### Fixed

- Bold text, as in Markdown, is drawn in Roboto Flex; on macOS it fell
  back to a monospaced font.
- On macOS, prev follows a switch between light and dark in System
  Settings.
- On Linux, prev starts in the desktop's light or dark setting even when
  the settings portal is slow to answer.
- With animations off, buttons and menu rows light up under the pointer
  straight away.
- Fit to width no longer shows a horizontal scrollbar, which rounding
  could bring up by a fraction of a pixel.

## [1.6.0] - 2026-10-01

### Added

- Drag and drop on macOS, both ways, as on Linux and Windows: pictures,
  text, files and pages in; text, areas, pages and images out.
- Images dropped on the page thumbnails, or dragged there as image
  annotations, become pages of their own, fitted to the page size.
- A PDF file dropped on a document's page asks whether to add it to the
  end or open it in its own window.
- Ctrl (Control on macOS) while dropping on the page view sends the drop
  to the sidebar: among the pages, or with an image window's images.
- An image annotation dragged onto an image window's sidebar is saved to
  Downloads and joins the window.

### Changed

- Shift no longer takes dropped images as files; Ctrl sends them to the
  sidebar instead. Shift still moves pages, and on macOS Command does
  too.
- An image file dropped on a page goes on it even with the markup bar
  closed.
- Notices go away by themselves after 10 seconds, and their bar takes
  the theme's colors instead of inverted ones.
- The accent color setting follows the system on every platform: the
  desktop theme's accent on Linux, and the accent color set on Windows
  and macOS. The settings file's `omarchy-palette` is now
  `system-accent`, and the old name still works.
- With the system accent off, or where the system has none, a row of
  colors in Settings picks the one prev's colors are built from; grey
  gives a neutral scheme. It is saved as `accent-color`.
- Ctrl+drag (⌘+drag on macOS) pans a zoomed-in document, whatever tool
  is chosen; the pointer turns into an open hand as soon as the key is
  held, and a closed one while dragging, drawn as on macOS on Linux and
  Windows too.
- The version in Settings and `prev --version` names the commit it was
  built from, and the time for development builds.
- New settings default to no animations, a 20 px corner radius and 25%
  overlay transparency; settings already saved keep their values.

### Fixed

- Animated images no longer flicker while they play.
- Closing the file chooser with Escape no longer shows an error.
- Wheel zoom on macOS moves at the same speed as elsewhere instead of
  barely moving with each notch.

## [1.5.0] - 2026-09-30

### Added

- The interface in 17 more languages, 38 in all: Bengali, Catalan,
  Czech, Finnish, Greek, Hungarian, Indonesian, Malay, Norwegian
  (Bokmål), Portuguese (Portugal), Romanian, Swahili, Swedish, Tamil,
  Thai, Urdu and Vietnamese, laid out right to left for Urdu. They are
  first drafts, like the other translations.

## [1.4.0] - 2026-09-30

### Added

- Keyboard shortcuts can be changed in the `[keys]` table of the
  settings file, for every system or for one; menus on macOS and the
  hints prev shows name the shortcuts in effect.
- A Make Default button in Settings makes prev the system's default app
  for the file types it opens, with a dot showing whether none, some or
  all of them open with prev. On macOS it asks for the common types,
  which macOS confirms one by one; on Windows it opens prev's page in
  Default apps.

### Changed

- A selected annotation shows a thin blue box with square handles,
  the same in every theme.
- On Windows, HEIC and AVIF come from one smaller `heif.dll`.
- On macOS, HEIC and AVIF open without Homebrew: prev.app carries a
  decode-only libheif.
- The tour of prev shows each system's differences and has an
  Inspector section, and page reordering is shown as an animation.

## [1.3.0] - 2026-09-29

### Added

- macOS 11 and later on Apple Silicon, as a disk image: a menu bar laid
  out like Preview's, ⌘ shortcuts, files opened from Finder and the
  Dock, the pasteboard, and printing through the system print panel
  with pages drawn as prev shows them. Not yet notarized.
- The interface in 21 languages, following the system's language or the
  one chosen in Settings, laid out right to left for Arabic, Hebrew and
  Persian. The translations other than English are first drafts.
- An input language setting, following the keyboard layout by default,
  which sets the side an empty text field starts on.
- prev's version at the foot of Settings.

### Changed

- Field labels rise as soon as a field is focused.
- The tour of prev moved to the website, prev.run/guide.html.

### Fixed

- Turning the floating toolbar on or off no longer goes back to the
  first page.
- Two pages side by side fit the width without a sideways scrollbar.
- Switching to two pages keeps the page in view.
- Confirmation dialogs no longer shrink to a narrow bar.
- On Wayland, prev no longer quits when a window closes while another
  app offers it a clipboard or drag.

## [1.2.1] - 2026-09-28

### Fixed

- Showing or hiding the markup bar no longer scrolls PDFs and images
  back to the top.
- Opening markup on a zoomed-in image keeps its zoom and the part in
  view, and closing the markup returns the image to where the markup
  showed it.

## [1.2.0] - 2026-09-28

prev is now built for ARM64 on Linux and Windows, and for RISC-V on
Linux.

### Packages

- Linux on ARM64 (aarch64): .deb, .rpm, AppImage, tarball and Flatpak.
- Linux on RISC-V (riscv64): .deb, .rpm and tarball.
- Windows on ARM64: the installer and the portable zip.
- The Flatpak bundles are named for their architecture:
  `prev-x86_64.flatpak` and `prev-aarch64.flatpak`.

## [1.1.0] - 2026-09-28

prev now runs on Windows 10 and 11.

### Windows

- A per-user installer (MSI) with a Start menu entry, "Open with" and
  Default apps for every file type prev opens, and a portable zip.
- Everything the Linux version does: single instance, the clipboard
  (images, copied files and pages), drag and drop both ways, printing
  through the Windows print dialog, and HEIC and AVIF with libheif
  included.
- Settings and data in `%APPDATA%\prev`.

### Markdown

- A toolbar like the other windows': text size, search with every match
  marked and counted, an inspector, and export as a picture.
- Code colored to suit the light or dark look; pictures at their own
  size, shrunk only to fit the page.

### Images and SVG

- Export asks for the format, JPEG quality and, for SVG drawings, the
  size in prev's own dialog, before the save dialog.
- SVG drawings export as pictures at one, two or four times their size.
- Tools that change pixels are greyed out where nothing can be edited.

### Everywhere

- Smooth scrolling with the mouse wheel.
- prev's website: https://prev.run.
- On Linux, the install script puts the desktop entry where launchers
  look.

## [1.0.0] - 2026-09-27

The first release.

### PDFs

- Continuous, single page and two page layouts, zoom and fit, sharp at
  any zoom, with search, a table of contents, page thumbnails, bookmarks,
  links, text selection, a slideshow and printing.
- Markup with Preview's tools: highlight, underline and strikethrough,
  sketch and draw, shapes (including a loupe and a mask), text boxes,
  notes and signatures, with styles, undo and redo, saved as standard
  annotations.
- Forms, a signature library (drawn, typed or from a photo), and
  redaction that removes the content from the file for good.
- Page editing: reorder, rotate, insert, delete, crop, copy and paste
  pages between documents, and merge by dragging thumbnails.
- Export to PDF (reduced, flattened or encrypted) or to images.
- An inspector with the document's information.

### Images

- PNG, JPEG, GIF, WebP, AVIF, HEIC, TIFF, BMP, ICO, TGA, PNM, QOI,
  JPEG 2000, OpenEXR, Radiance HDR and camera RAW, with RAW photos
  developed to look like the camera's own JPEG.
- Rotate, flip, crop, resize and adjust color, saved in place with EXIF,
  color profiles and XMP kept, and earlier versions to revert to.
- Markup with the PDF tools, drawn into the image on export.
- An inspector with camera details, location removal, keywords and a
  description.

### Everywhere

- Copy and paste, and drag and drop both ways with other windows and
  apps: images, text, files and pages.
- SVG drawings and Markdown documents.
- Autosave, a single running instance, and settings in
  `~/.config/prev.toml`.
- Material 3 design, light and dark, with colors from the desktop theme,
  an optional floating toolbar, and reduced motion.

[2.1.0]: https://github.com/scrambletools/prev/releases/tag/v2.1.0
[2.0.0]: https://github.com/scrambletools/prev/releases/tag/v2.0.0
[1.6.0]: https://github.com/scrambletools/prev/releases/tag/v1.6.0
[1.5.0]: https://github.com/scrambletools/prev/releases/tag/v1.5.0
[1.4.0]: https://github.com/scrambletools/prev/releases/tag/v1.4.0
[1.3.0]: https://github.com/scrambletools/prev/releases/tag/v1.3.0
[1.2.1]: https://github.com/scrambletools/prev/releases/tag/v1.2.1
[1.2.0]: https://github.com/scrambletools/prev/releases/tag/v1.2.0
[1.1.0]: https://github.com/scrambletools/prev/releases/tag/v1.1.0
[1.0.0]: https://github.com/scrambletools/prev/releases/tag/v1.0.0
