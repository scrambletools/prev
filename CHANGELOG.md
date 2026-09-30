# Changelog

All notable changes to prev. Versions follow
[Semantic Versioning](https://semver.org).

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
- Material 3 design, light and dark, with colors from the Omarchy theme,
  an optional floating toolbar, and reduced motion.

[1.2.1]: https://github.com/scrambletools/prev/releases/tag/v1.2.1
[1.2.0]: https://github.com/scrambletools/prev/releases/tag/v1.2.0
[1.1.0]: https://github.com/scrambletools/prev/releases/tag/v1.1.0
[1.0.0]: https://github.com/scrambletools/prev/releases/tag/v1.0.0
