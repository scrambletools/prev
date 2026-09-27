# prev: development plan

prev is a fast, open source document and image viewer for Linux, modelled on
macOS Preview. It views and edits PDFs (annotations, signatures, page editing,
redaction, export), views and lightly edits images, and views SVG and Markdown.

## Guiding priorities

1. Open source: prev and every dependency are open source, checked in CI.
2. Performance: instant open, smooth scroll and zoom, low latency input.
3. Small footprint: minimal dependencies, lean binary and memory use.

Where this plan says "Preview parity", the macOS Preview behaviour is the
reference unless it is technically infeasible on Linux. Preview parity covers
features and behaviour; the look follows Material Design 3 (see Interface
design).

## Decisions

| Area | Decision |
|---|---|
| Name / repo | `prev`, public, github.com/scrambletools/prev |
| License | AGPL-3.0-or-later (required by MuPDF) |
| Platform | Linux only, Wayland first (Hyprland/Omarchy), X11 supported |
| Language | Rust, stable toolchain, edition 2024 |
| GUI | iced 0.14 (wgpu renderer, tiny-skia software fallback) |
| PDF engine | MuPDF via `mupdf` crate, behind an internal engine trait ([ADR 0001](decisions/0001-pdf-engine.md)) |
| SVG | resvg |
| Markdown | pulldown-cmark + iced markdown widget, syntect highlighting, view only |
| Images | `image` crate + format-specific decoders (see below) |
| HEIC | Open only, via system libheif (LGPL); never encode |
| Camera RAW | View and export via rawler (LGPL-2.1, pure Rust) |
| Windows | One window per document; opening several images groups them in one window with a thumbnail sidebar |
| Saving | Autosave in place with local version history ("Revert To"), Duplicate, Export As |
| Design system | Material Design 3 Expressive, drawn with custom iced styles and widgets |
| Theming | M3 dynamic color: tonal palettes from a seed color, which is the Omarchy accent when an Omarchy theme is active and a fixed prev color otherwise; light or dark follows the system (freedesktop portal) |
| Fonts | Roboto Flex (OFL-1.1) for the interface and Material Symbols Rounded (Apache-2.0) for icons, both bundled |
| Packaging | AUR (`prev`, `prev-git`), Flatpak, AppImage, .deb, .rpm |

## Feature scope

### PDF

- **Viewing:** continuous scroll, single page, two-page; fit width/page,
  actual size, zoom; rotate view; full screen and slideshow; dark-mode page
  inversion (optional).
- **Sidebar:** thumbnails, table of contents, highlights and notes, bookmarks,
  contact sheet.
- **Navigation:** search with result list, text selection and copy, internal
  and external links, go to page, history back/forward.
- **Markup (Preview parity):** text and rectangle selection, sketch with shape
  recognition, draw, shapes (rectangle, rounded rectangle, oval, line, arrow,
  star, polygon, speech bubble, loupe, mask), text box, highlight, underline,
  strikethrough, notes, shape and text styles. Saved as standard PDF
  annotations, editable in other viewers.
- **Forms:** fill AcroForm fields.
- **Signatures:** visual signatures captured by drawing, typing (script font)
  or importing an image; stored for reuse; placed as stamp annotations.
  Cryptographic (PAdES) signing is deferred, but the save path must allow it
  later.
- **As built (M5):**
  - Annotations go through `prev-pdf`'s engine-neutral model
    (`annotation.rs`); `mupdf_annotations.rs` reads and writes them with
    MuPDF. Each annotation carries a `/NM` id, and prev names its shapes in
    `/Subj` ("Star", "Signature") so they read back as what was drawn.
  - Rounded rectangles, stars, polygons and speech bubbles are polygon
    annotations; arrows are lines with arrow heads. Loupes and masks are
    stamps with prev's own appearance: a loupe holds a magnified image of
    the page under it, taken again when it moves; a mask darkens the page
    except its hole.
  - Signatures are PNGs in `$XDG_DATA_HOME/prev/signatures/` with an index
    of descriptions, and become image stamps with transparency. Typed
    signatures use Dancing Script (OFL). Imported photos and scans have
    their paper made transparent.
  - Every edit is undoable: removals keep the object so undo puts back
    the same annotation.
  - Saving is incremental. MuPDF assumes an incremental save is appended to
    the file it opened, so after writing, the document thread reopens it
    from disk; without that, the second save of a session wrote a broken
    cross-reference chain.
  - Rectangular selection copies an image through `wl-copy`, since the
    window clipboard only carries text. Cropping to it is page editing (M6).
  - Not done: text inside shapes, shadows, and editing a signature's ink
    after placing it.
- **Page editing:** reorder by drag, delete, rotate, insert blank page, insert
  from file, merge by dragging between windows, extract pages to new PDF.
- **Redaction:** true redaction via MuPDF (removes text, images and vector
  content under the redaction box, not an overlay).
- **Security and size:** password encryption, "reduce file size" (image
  downsampling and recompression).
- **Export:** PDF, PNG, JPEG, JPEG 2000, TIFF (including multi-page), OpenEXR,
  with DPI and quality options.
- **As built (M6):**
  - Page edits go through the engine (`mupdf_pages.rs`): rotate, remove and
    restore, move, insert blank, insert a PDF, extract, crop, apply
    redactions and export. The document thread answers each with a new
    `DocumentInfo`; the viewer moves every cache keyed by page number to
    the new numbers and drops results that were on their way from before
    (an epoch on page-keyed messages).
  - Undo uses the same history as markup, strictly in order, so page
    numbers held by earlier changes are right again when they are undone.
    Removed pages stay in the file as objects and are put back by number,
    as removed annotations are; reorders undo with the inverse order.
  - MuPDF's page graft leaves annotations and form fields behind, so prev
    copies them onto copied pages itself, keeping each field's parents (a
    radio group stays one group); links into the source document and
    popups are dropped.
  - MuPDF shifts page label ranges on every insert and delete and makes
    labels up for documents without any, numbering from 1 again after an
    insert at the start. prev keeps labels as they were for moves and
    removes invented ones.
  - Merging between windows is copy and paste (Ctrl+C and Ctrl+V after
    clicking a thumbnail), with the pages travelling as a PDF inside the
    one prev process, or, since the drag and drop work after M7, dragging
    thumbnails from one window to another (Shift moves them). Dropping
    PDF files on the thumbnails inserts them.
  - Redaction marks are standard Redact annotations until applied. Applying
    removes text, image pixels and drawings covered by a mark and draws
    black boxes; the next save rewrites the whole file with unused objects
    dropped, and prev deletes the versions it kept of the file. Tests check
    the saved bytes for the text, the image's pixels and the drawing.
  - Export writes a full rewrite (compressed, garbage collected), optionally
    AES-256 encrypted, of all or the selected pages. Reduce file size
    recompresses 8-bit gray and RGB images (JPEG or Flate) as JPEG at
    150 dpi of the page they are on, in place so shared images shrink once,
    and keeps an image unchanged when that would not save 10%.
  - Flatten bakes annotations and widgets into the page content with
    MuPDF and drops the AcroForm. Unapplied redaction marks are removed
    first, since baking them would cover content without removing it.
  - Images: PNG, JPEG, WebP, OpenEXR one file per page, TIFF as one
    multi-page LZW file, at 72 to 600 dpi. JPEG 2000 is not offered: the
    image crate has no encoder.
  - Not done: auto-scrolling the thumbnails while dragging near an edge
    (the wheel scrolls during a drag).
- **Print:** print dialog built in prev, submitted to CUPS.

- **As built (after M6):**
  - Settings live in `$XDG_CONFIG_HOME/prev.toml` (`prev-dev.toml` for
    development builds), read once from the older `prev/settings.toml`.
    The file lists every setting; missing keys are written when prev
    starts, which pins the storage paths (signatures, versions,
    bookmarks) the first time. Paths are stored with `~` and checked
    (existing, writable) when changed from the dialog.
  - The Settings dialog opens over the window that asked for it: iced
    0.14 cannot give a Wayland window a parent.
  - Floating toolbars (optional): M3 floating toolbars over the content,
    the main one at the top and the markup bar at the bottom, shown
    while the pointer is over the window. Content layers stay first in
    the widget tree so showing and hiding keeps scroll positions.
  - Animations and the corner radius of dialogs and floating toolbars
    are settings; the system's reduced motion setting also turns motion
    off.

### Images

- **Formats (open):** PNG, JPEG, GIF (animated), WebP, AVIF, BMP, ICO, TIFF,
  TGA, PNM, QOI, HDR, OpenEXR, JPEG 2000, HEIC (system libheif), camera RAW.
- **Editing:** crop, rotate, flip, resize (Adjust Size), Adjust Color
  (exposure, contrast, saturation, temperature, tint, sepia, sharpness,
  levels), and the PDF markup tools on images.
- **Metadata (Preview parity):** inspector for EXIF/XMP/IPTC/GPS, Remove
  Location Info, edit keywords and description.
- **Export:** PNG, JPEG, JPEG 2000, TIFF, OpenEXR, WebP; HEIC never.

- **As built (M7):**
  - RAW is developed by rawler to linear light (demosaic, white balance,
    camera matrix), then given the camera's look with a tone curve per
    channel, each matching that channel's histogram to the camera's
    embedded JPEG (the largest of its full, preview and thumbnail
    images, with black bars trimmed). This follows RawTherapee's
    auto-matched tone curve; the curves are monotone cubics through 32
    quantiles, pinned at black and white. Files without a preview get a
    standard contrast curve. Scored against the camera JPEGs of Sony,
    Canon and Fuji samples, per channel curves came closest (mean
    difference 1.2, 3.1 and 13.9 on a 0 to 255 scale, from 27 to 34 for
    rawler's plain development); Fuji's film simulations are beyond a
    global curve. `examples/raw_look.rs` and `raw_score.rs` in
    `prev-image` reproduce the comparison.
  - Edited TIFFs are rewritten with kamadak-exif's writer so they keep
    the original's EXIF, GPS, ICC profile, XMP and descriptive tags, with
    the layout tags of the new pixels and the orientation reset. XMP is
    read from TIFF tag 700; keywords stay read-only in TIFF.

- **As built (after M7):**
  - Markup on images reuses the PDF markup: the image becomes a one-page
    PDF (its long side 800 points, so markup sizes suit any resolution),
    shown by an embedded PDF window without saving, page tools, text
    highlighting or redaction. The markup lasts while the window is
    open; image edits wait until it is gone. Export renders the
    annotations alone on a transparent page at the image's resolution
    and draws them over the original pixels. Closing a window (or
    quitting) with markup not yet exported asks first.
  - Paste (Ctrl+V) reads the system clipboard with wl-paste: an image
    (PNG first, then JPEG, WebP and others, or an image file from a
    `text/uri-list`) becomes an image stamp, text a text box, whatever
    tool is chosen. Copying pages offers `application/x-prev-pages` on
    the clipboard, so pages paste when they were copied last; after a
    thumbnail click they always do. Without wl-clipboard, text still
    pastes through iced.
  - Drag and drop, both ways, through the vendored smithay-clipboard
    (see `vendor/PATCHES.md`): drops are accepted in Paste's types, most
    preferred first (prev pages, images, Chromium's named file contents,
    file lists, text; web addresses in file lists are fetched with curl
    when their names say picture or PDF, and arrive as text otherwise or
    when fetching fails), read off the UI thread and handed to the
    window under them with their position,
    found from where the canvas and thumbnails were last drawn (a `Probe`
    widget). Drags out start from a press on selected text, a chosen
    area, an image sidebar entry, or when a thumbnail drag leaves its
    window (the app watches the pointer only then). Pages go as prev's
    type, `text/uri-list` of a temp PDF and `application/pdf`; sidebar
    images as their file only, so file managers copy the file. Hyprland
    reports no DnD action, so Shift decides move on both ends; the source
    removes moved pages once the drag ends, unless it took them itself.
    Shift while dropping images takes them as files (added to an image
    window, opened elsewhere); picture data without a file is saved to
    the Downloads folder (`user-dirs.dirs`), numbered if the name is
    taken. prev sees Shift during drags from other apps only because
    Hyprland moves keyboard focus with the pointer.
  - Toolbars: the PDF and image toolbars share one layout and icon set
    (context and view on the left; undo, editing, panels, search and an
    export icon button on the right), with Undo and Redo on the main bar
    unless the markup bar shows its own. Actual size joined the PDF fit
    group, Rotate left the Pages menu, Adjust Size got a resize icon, and
    the search field takes the room the other slots leave. "More" menus
    close once a choice is made, unless the click opens a nested menu.
  - Inspectors: the PDF window gained one (Ctrl+I) with the file, the
    document information and page size; the image inspector shows the
    file's name, folder, size and date too.

### SVG

- Static SVG rendering via resvg with zoom and export to PNG. No scripting or
  animation.

### Markdown

- GitHub Flavored Markdown: headings, tables, task lists, footnotes,
  strikethrough, images (relative paths), links (opened externally), syntax
  highlighted code. View only; reloads when the file changes on disk.
- Not in scope: math, Mermaid, raw HTML blocks (rendered as text).

### Interface design

Material Design 3 Expressive (m3.material.io) applied to Preview's layout:
the same windows, tools and panels, drawn as M3 components.

- **Color:** `material-colors` (MIT OR Apache-2.0, a port of Google's
  material-color-utilities) builds the scheme from the seed color. All
  surfaces use the M3 roles (surface containers, on-surface, outline,
  primary and tertiary containers); nothing uses hard-coded colors. The
  Omarchy colors setting becomes "Use Omarchy accent"; the scheme changes
  live when the Omarchy theme or system mode changes.
- **Type:** the M3 Expressive type scale (display, headline, title, body,
  label, with emphasized variants) as named styles, used everywhere
  instead of ad hoc sizes.
- **Shape and elevation:** the M3 corner scale and tonal elevation;
  shadows only where M3 uses them (menus, dialogs, floating toolbars).
- **Components:**
  - Toolbar: M3 docked toolbar with icon buttons and button groups
    (zoom, rotate, markup tools); segmented buttons for view modes.
  - Sidebar: standard side sheet with M3 tabs; thumbnails and list items
    use M3 list and selection states.
  - Panels (Adjust Color, Adjust Size, Inspector): M3 side sheets or
    dialogs with M3 sliders, text fields, switches and menus.
  - Prompts (export mismatch, errors, unsaved changes): M3 basic dialogs
    over a scrim; short notices as snackbars.
  - Menus and tooltips: M3 menus and plain tooltips.
  - Search: M3 search bar; page box as an M3 outlined text field.
- **State layers:** hover, focus, pressed and dragged states on every
  interactive element, with visible keyboard focus rings.
- **Motion:** M3 Expressive spring motion (standard and expressive
  schemes, fast/default/slow) for sheets, dialogs, menus and button shape
  changes; respects reduced-motion settings.
- **Icons:** Material Symbols Rounded, subset to the icons prev uses.
- **Density and targets:** 48 px minimum touch targets where layout
  allows; compact density for toolbars as M3 permits for desktop.
- File dialogs, print dialogs and the window frame stay native (portal
  and compositor); M3 applies inside prev's windows only.
- **As built (M4.1):** components live in `crates/prev/src/ui`. Left
  sidebars resize by dragging (PDF 248 to 480 px, images 140 to 400 px)
  and their thumbnails follow the width. Desktop density changes from the
  spec: 56 px toolbar instead of 64, 32 px slider handles instead of 44.
  Text field labels float once there is text rather than on focus. Sheets,
  dialogs and snackbars spring in but do not animate out, and nothing fades,
  as iced cannot draw a layer at partial opacity. Tab focus moves in every
  window of the process at once. Toolbars measure their width and move
  the groups that don't fit into a "More" menu, in a fixed order per bar.

### Out of scope

Audio and video, Markdown editing, cryptographic signatures (deferred), OCR
(possible later via MuPDF with Tesseract), macOS and Windows builds.

## Architecture

### Workspace layout

```
crates/
  prev/          binary: iced application, windows, tools, dialogs
  prev-pdf/      PdfEngine trait + MuPDF implementation, redaction, page ops
  prev-image/    decoding, editing operations, metadata, export
  prev-store/    autosave, version history, settings, signature library
```

Markdown and SVG are thin enough to live in `prev` until they grow.

### PDF engine trait

All PDF access goes through a `PdfEngine` / `PdfDocument` trait in
`prev-pdf`: open and authenticate, page count and sizes, render tile, display
list, text and glyph geometry, search, links, outline, annotations (read,
create, edit, delete), form fields, page operations, redaction, encryption,
save (incremental and full). MuPDF is the only implementation. If MuPDF's
license ever becomes a problem, a PDFium implementation can be added without
touching the UI.

### Rendering pipeline

- MuPDF documents are not `Send`, so each open document is owned by its own
  document thread, which loads pages, applies edits and builds display lists.
  The UI thread never calls MuPDF.
- Each page is parsed once into a MuPDF display list (`Send + Sync`), which is
  cached and replayed for every zoom level and tile by a render worker pool.
  Each thread gets its own cloned MuPDF context automatically.
- Pages are rendered as fixed-size tiles at the current scale. Visible tiles
  come first, then prefetch. A low resolution pass shows immediately and is
  replaced by the sharp pass.
- An LRU tile cache has a memory budget. Tiles are uploaded as GPU textures.
- Stale requests are cancelled when the user scrolls or zooms past them.
- Annotation and selection overlays are drawn by iced on top of the tiles,
  using page geometry from the engine, so they never trigger a re-render.

### Document model and editing

- Every edit is a command with undo/redo, applied to the in-memory document.
- Autosave writes in place a short time after edits stop, using write to
  temp file, fsync, then atomic rename. File mode is preserved.
- Before the first write in a session, the original is kept as a version under
  `$XDG_DATA_HOME/prev/versions/`. Versions are pruned by age and size.
- PDF annotations are saved incrementally where possible; page operations and
  redaction force a full rewrite (redaction must never be incremental, so the
  removed content cannot be recovered from earlier revisions).

### Windows and desktop integration

- iced multi-window (`iced::daemon`): one window per document, with a single
  process handling later `prev <file>` calls over a local socket.
- `.desktop` file with MIME types for PDF, images, SVG and Markdown, so prev
  can be set as the default viewer.
- Wayland: xdg-desktop-portal for file dialogs and color scheme.

## Dependency and license policy

- `cargo-deny` runs in CI with a license allowlist of AGPL-3.0 compatible
  licenses: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib,
  Unicode-3.0, BSL-1.0, CC0-1.0, MPL-2.0, LGPL-2.1-or-later, LGPL-3.0,
  GPL-3.0, AGPL-3.0. GPL-2.0-only is denied (incompatible with AGPL-3.0).
- No new crate is added without checking its license, maintenance, and
  whether an existing dependency already covers the need.
- Optional features (HEIC, RAW) are cargo features so lean builds are possible.
- MuPDF fonts: bundle only the base 14 fonts plus a Noto subset. CJK is
  loaded from system fonts through fontconfig to save space.

## Performance targets

Measured on the development machine (RTX 4080 SUPER plus AMD iGPU, 3840x1080
at 120 Hz, scale 1.25), release build. `cargo run --release -p prev-pdf
--example open_bench -- <file>` times opening; `PREV_BENCH=<seconds> prev
<file>` runs a scripted scroll and zoom and prints frame, CPU, memory and tile
statistics.

| Metric | Target | Measured (2026-09-26) |
|---|---|---|
| Cold start to window | < 150 ms | ~230 ms; wgpu Vulkan setup is ~200 ms of it |
| Open 100-page PDF to first page drawn | < 100 ms | 20 ms (499 pages), 93 ms (7025 pages) |
| Scroll and zoom | 60 fps, no dropped frames during tile loads | 120 fps, no slow frames |
| Idle memory, one 100-page PDF open | < 150 MB | 61 MB heap, 179 MB resident with GPU drivers |
| Release binary (stripped, without HEIC/RAW) | < 30 MB | 29.9 MB (36.1 MB with RAW); was 22.6 MB before image, SVG and Markdown support |

Renderer: wgpu with Vulkan. The CPU renderer (tiny-skia) starts in 41 ms but
drops to about 60 fps scrolling and 30 fps zooming at 108% CPU, against 27%
for Vulkan. wgpu's OpenGL backend cannot draw to iced's Wayland windows. Peak
memory during heavy scrolling is ~430 MB with the 128 MB tile cache. Most of
the rest is MuPDF's resource store (fonts, images, glyphs; 256 MB default,
not configurable through the `mupdf` crate) and GPU driver mappings. A
smaller tile cache saved little memory and cost CPU in re-rendering.

## Testing

- **Unit tests** in each crate.
- **Render regression:** a curated PDF corpus (a subset of the pdf.js and
  PDFium test suites, fetched in CI, not committed) rendered and compared
  against stored hashes.
- **Round-trip:** annotations, page edits and form fills written by prev are
  read back by MuPDF and by Poppler (`pdftoppm`, `pdfinfo`) to confirm other
  viewers see them.
- **Redaction:** after redacting, text extraction, content stream inspection
  and image extraction under the box must return nothing; tests for text,
  images, vector art and form XObjects.
- **UI:** headless interaction tests with `iced_test`.
- **Fuzzing:** `cargo-fuzz` targets for file-type detection and image
  decoding.

## CI (GitHub Actions)

- On every push and PR: `cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test`, `cargo deny check`, render regression suite.
- On tags: build release artifacts (AppImage, .deb, .rpm), update the AUR
  package, publish the Flatpak manifest.

## Milestones

Each milestone ends with a usable build.

| # | Milestone | Contents |
|---|---|---|
| M0 | Foundation | Workspace, CI, cargo-deny, license, `.desktop` and MIME, file-type detection, spikes: MuPDF redaction and annotation APIs in `mupdf-rs`, iced multi-window on Hyprland |
| M1 | App shell | iced app, window per document, open via CLI and dialog, drag and drop, single instance, theming (system + Omarchy), keyboard shortcuts, settings |
| M2 | PDF viewing | Engine trait and MuPDF implementation, tile pipeline, view modes, zoom, sidebar (thumbnails, TOC, bookmarks), search, text selection and copy, links, full screen, slideshow, print |
| M3 | Images, SVG, Markdown viewing | All image formats including HEIC and RAW, multi-image window with sidebar, animated GIF, SVG via resvg, Markdown with highlighting and live reload |
| M4 | Image editing | Crop, rotate, flip, Adjust Size, Adjust Color, metadata inspector and edits, export, autosave and version history |
| M4.1 | Material 3 design | M3 Expressive color scheme from the Omarchy accent, Roboto Flex and the M3 type scale, Material Symbols, M3 components for toolbar, sidebar, panels, dialogs, menus and fields, state layers and focus, spring motion; screenshots and README updated |
| M5 | PDF markup | All markup tools, notes, form filling, signatures, undo/redo, annotation round-trip tests (MuPDF and Poppler), highlights and notes sidebar |
| M6 | PDF page editing | Reorder, delete, rotate, crop, insert, merge between windows (copy and paste), extract, redaction, encryption, reduce file size, export to images |
| M7 | Image polish | RAW tone curves matched to the camera's embedded JPEG (after RawTherapee's auto-matched curve, in Rust), keep EXIF in edited TIFFs, read XMP from TIFF (done) |
| M8 | Release | AUR, Flatpak, AppImage, .deb/.rpm, release workflow, user docs, 1.0 |

Later: cryptographic signatures, OCR, markup on SVG.

## Risks

| Risk | Mitigation |
|---|---|
| `mupdf-rs` does not expose every API we need | Checked: redaction, markup annotations, page operations, encryption and incremental save are covered (`crates/prev-pdf/tests/mupdf_capabilities.rs`). Gap: no safe call for stamp images or custom appearances; image signatures work by building the appearance stream with the object API. Contribute the missing wrappers upstream |
| MuPDF weaker on JBIG2 refinement and halftone scans ([ADR 0001](decisions/0001-pdf-engine.md)) | Track upstream jbig2dec; keep such files in the regression corpus |
| MuPDF license change by Artifex | Engine trait; pin versions; released AGPL versions remain usable |
| iced multi-window or text input gaps on Wayland | Checked on Hyprland 0.56: native Wayland windows, per-window titles and app id, per-window shortcuts, exit on last close. Text input and IME still to be tried by hand; contribute fixes upstream |
| No file drag and drop on Wayland: winit 0.30 implements it only for X11, and Hyprland sends drag events only to a client's first `wl_data_device`, which iced's clipboard (smithay-clipboard) owns | Handled in that same device: smithay-clipboard is vendored with a drag and drop patch (`vendor/PATCHES.md`); offer it upstream. The patch now also starts drags (with an icon) and takes any drop type, so pages, text, areas and images drag both ways. Hyprland reports no copy or move action, so prev goes by Shift |
| Metadata lost or altered by image edits | Saving carries EXIF, ICC and XMP for JPEG, PNG and WebP and resets the EXIF orientation; EXIF edits (orientation, GPS removal, which overwrites the data) are done in place by prev's own code. TIFF too since M7 (keywords are read-only there) |
| Autosave damaging files | Atomic writes, original kept as a version before the first write, fuzzed save paths. PDFs are reopened after every incremental save, and a test checks every cross-reference offset after repeated saves |
| Redaction leaking content | Dedicated test suite; full rewrite only, never incremental |
| Memory during heavy use, largely MuPDF's store | Keep MuPDF's 256 MB default. If memory becomes a problem, add a store size limit to the `mupdf` crate upstream and remeasure with `PREV_BENCH` |
| iced lacks M3 components and spring motion | Build them as custom widgets in a `prev/src/ui` module (styles from the scheme, springs driven by `request_redraw_at` as the image canvas already does) |
| Variable fonts (Roboto Flex, Material Symbols) not fully supported by iced's text stack | Check weight axes in cosmic-text first; if unsupported, bundle static instances cut from the variable fonts with fonttools |
| Seed-derived schemes drift from the Omarchy theme's look | The seed is the Omarchy accent, so hue matches; the M3 scheme variant (tonal spot, fidelity, vibrant) is chosen once by comparing against several Omarchy themes |
| Footprint growth from fonts and codecs | Feature flags, fontconfig for CJK, binary size tracked in CI |
