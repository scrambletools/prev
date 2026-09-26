# prev: development plan

prev is a fast, open source document and image viewer for Linux, modelled on
macOS Preview. It views and edits PDFs (annotations, signatures, page editing,
redaction, export), views and lightly edits images, and views SVG and Markdown.

## Guiding priorities

1. Open source: prev and every dependency are open source, checked in CI.
2. Performance: instant open, smooth scroll and zoom, low latency input.
3. Small footprint: minimal dependencies, lean binary and memory use.

Where this plan says "Preview parity", the macOS Preview behaviour is the
reference unless it is technically infeasible on Linux.

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
| Theming | System light/dark (freedesktop portal), plus Omarchy palette when an Omarchy theme is active |
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
- **Page editing:** reorder by drag, delete, rotate, insert blank page, insert
  from file, merge by dragging between windows, extract pages to new PDF.
- **Redaction:** true redaction via MuPDF (removes text, images and vector
  content under the redaction box, not an overlay).
- **Security and size:** password encryption, "reduce file size" (image
  downsampling and recompression).
- **Export:** PDF, PNG, JPEG, JPEG 2000, TIFF (including multi-page), OpenEXR,
  with DPI and quality options.
- **Print:** print dialog built in prev, submitted to CUPS.

### Images

- **Formats (open):** PNG, JPEG, GIF (animated), WebP, AVIF, BMP, ICO, TIFF,
  TGA, PNM, QOI, HDR, OpenEXR, JPEG 2000, HEIC (system libheif), camera RAW.
- **Editing:** crop, rotate, flip, resize (Adjust Size), Adjust Color
  (exposure, contrast, saturation, temperature, tint, sepia, sharpness,
  levels), and the PDF markup tools on images.
- **Metadata (Preview parity):** inspector for EXIF/XMP/IPTC/GPS, Remove
  Location Info, edit keywords and description.
- **Export:** PNG, JPEG, JPEG 2000, TIFF, OpenEXR, WebP; HEIC never.

### SVG

- Static SVG rendering via resvg with zoom and export to PNG. No scripting or
  animation.

### Markdown

- GitHub Flavored Markdown: headings, tables, task lists, footnotes,
  strikethrough, images (relative paths), links (opened externally), syntax
  highlighted code. View only; reloads when the file changes on disk.
- Not in scope: math, Mermaid, raw HTML blocks (rendered as text).

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

Measured on the development machine, with a benchmark suite from the start.

| Metric | Target |
|---|---|
| Cold start to window | < 150 ms |
| Open 100-page PDF to first page drawn | < 100 ms |
| Scroll and zoom | 60 fps, no dropped frames during tile loads |
| Idle memory, one 100-page PDF open | < 150 MB |
| Release binary (stripped, without HEIC/RAW) | < 30 MB |

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
| M5 | PDF markup | All markup tools, notes, form filling, signatures, undo/redo, annotation round-trip tests |
| M6 | PDF page editing | Reorder, delete, rotate, insert, merge between windows, extract, redaction, encryption, reduce file size, export to images |
| M7 | Release | AUR, Flatpak, AppImage, .deb/.rpm, release workflow, user docs, 1.0 |

Later: cryptographic signatures, OCR, markup on SVG.

## Risks

| Risk | Mitigation |
|---|---|
| `mupdf-rs` does not expose every API we need | Checked: redaction, markup annotations, page operations, encryption and incremental save are covered (`crates/prev-pdf/tests/mupdf_capabilities.rs`). Gap: no safe call for stamp images or custom appearances; image signatures work by building the appearance stream with the object API. Contribute the missing wrappers upstream |
| MuPDF weaker on JBIG2 refinement and halftone scans ([ADR 0001](decisions/0001-pdf-engine.md)) | Track upstream jbig2dec; keep such files in the regression corpus |
| MuPDF license change by Artifex | Engine trait; pin versions; released AGPL versions remain usable |
| iced multi-window or text input gaps on Wayland | Checked on Hyprland 0.56: native Wayland windows, per-window titles and app id, per-window shortcuts, exit on last close. Text input and IME still to be tried by hand; contribute fixes upstream |
| No file drag and drop on Wayland: winit 0.30 implements it only for X11, and Hyprland sends drag events only to a client's first `wl_data_device`, which iced's clipboard (smithay-clipboard) owns | Handled in that same device: smithay-clipboard is vendored with a drag and drop patch (`vendor/PATCHES.md`); offer it upstream. Page drag between windows (M6) will extend it |
| Autosave damaging files | Atomic writes, original kept as a version before the first write, fuzzed save paths |
| Redaction leaking content | Dedicated test suite; full rewrite only, never incremental |
| Footprint growth from fonts and codecs | Feature flags, fontconfig for CJK, binary size tracked in CI |
