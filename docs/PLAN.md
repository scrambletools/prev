# prev: design and plan

prev is a fast, open source document and image viewer for Linux, Windows
and macOS, similar to macOS Preview. It views and edits PDFs (markup, forms,
signatures, page editing, redaction, export), views and edits images, and
views SVG and Markdown. The [README](../README.md) lists what it does
today and the [guide](https://prev.run/guide.html) shows how; this document covers how it is
built, what it will not do, and what comes next.

## Guiding priorities

1. Open source: prev and every dependency are open source, checked in CI.
2. Performance: instant open, smooth scroll and zoom, low latency input.
3. Small footprint: few dependencies, a lean binary and memory use.

Preview is the reference for features and behaviour, unless something is
not possible on one of the systems prev runs on; the look follows
Material Design 3.

## Decisions

| Area | Decision |
|---|---|
| License | AGPL-3.0-or-later, as MuPDF requires ([ADR 0001](decisions/0001-pdf-engine.md)) |
| Platforms | Linux, Wayland first (Hyprland and Omarchy), X11 supported; Windows 10 and 11; macOS 11 and later. x86_64 and ARM64 on Linux and Windows, RISC-V on Linux, Apple Silicon on macOS |
| Language | Rust, stable toolchain, edition 2024, minimum Rust 1.89 |
| GUI | iced 0.14: wgpu (Vulkan on Linux, DirectX 12 or Vulkan on Windows, Metal on macOS), tiny-skia on the CPU when no GPU backend starts |
| PDF engine | MuPDF through the `mupdf` crate, behind an engine trait |
| Images | The `image` crate and format decoders; rawler for camera RAW (LGPL-2.1, pure Rust) |
| HEIC and AVIF | Open only, through libheif loaded at run time: the system's on Linux, bundled in the Flatpak, on Windows and on macOS (a decode-only build with libde265 and aom's decoder, built for macOS 11). Never encoded |
| SVG | resvg |
| Markdown | iced's Markdown widget (pulldown-cmark), syntect highlighting; view and export as a picture, no editing |
| Windows | One window per document; images opened together share one window with a thumbnail sidebar; one running instance |
| Saving | Autosave in place, with the original kept as a version ("Revert To"); Export for copies |
| Design | Material Design 3 Expressive, drawn with prev's own iced styles and widgets |
| Colors | M3 dynamic color from a seed: the Omarchy accent when an Omarchy theme is active, prev's blue otherwise; light or dark follows the system |
| Fonts | Roboto Flex (OFL-1.1) and Material Symbols Rounded (Apache-2.0), bundled |
| Interface languages | Fluent files in `i18n/`, one per language, 21 languages; the layout mirrors inside panels, dialogs and menus for right to left languages ([TRANSLATING.md](TRANSLATING.md)) |
| Distribution | GitHub releases: .deb, .rpm, AppImage, tarball, Flatpak bundle, PKGBUILD, Windows MSI and zip, macOS disk image; AUR once published; not on Flathub |

## Not in scope

Audio and video, editing Markdown, math, Mermaid and raw HTML in
Markdown, SVG scripting and animation, and Intel Macs.

## Not built yet

Planned or considered, but not in prev today:

- Revert To for PDFs, as images have it.
- Dragging annotations out of a document or between documents.
- Cryptographic (PAdES) signatures; the save path leaves room for them.
- OCR (possible with MuPDF and Tesseract) and markup on SVG drawings.
- Text inside shapes, shadows, and editing a signature's ink after placing
  it.
- Page thumbnails that scroll by themselves while a drag nears their edge.
- A search results list, a contact sheet view, navigation history (back
  and forward), a rotated view that leaves the file alone, dark-mode page
  inversion, and Markdown footnotes.
- JPEG 2000 export (the `image` crate cannot encode it).
- On Windows, a picture of what is dragged under the pointer (Windows
  shows its own drag cursor).
- On macOS, drag and drop with other apps (only files dropped on a window
  open today) and copying files in Finder to paste them into prev.
- Further review of the translations other than English.

## Next

- **Windows code signing** through the SignPath Foundation, then a
  winget listing. See [RELEASING.md](RELEASING.md#windows-code-signing).
- **The AUR packages** (`prev`, `prev-git`), once an AUR account can be
  made.
- **The first macOS release**, then a Developer ID signature and
  notarization so Gatekeeper opens it without a warning, and a Homebrew
  cask. See [RELEASING.md](RELEASING.md#macos-signing).
- The items above, roughly in the order listed.

## Architecture

### Workspace

```
crates/
  prev/          the app: iced windows, tools, dialogs, platform code
  prev-pdf/      engine trait and MuPDF implementation: rendering, text,
                 annotations, forms, page operations, redaction, export
  prev-image/    decoding (including RAW and HEIC), editing, metadata, export
  prev-store/    settings, file locations, atomic writes, version history,
                 signatures, bookmarks
```

SVG and Markdown are small enough to live in `prev`.

### PDF engine

All PDF access goes through the `Engine` and `Document` traits in
`prev-pdf`. MuPDF is the only implementation; a PDFium one could be added
without touching the interface if MuPDF's license ever became a problem.

- MuPDF documents are not `Send`, so each open document lives on its own
  thread, which loads pages, applies edits and builds display lists. The
  interface thread never calls MuPDF.
- Each page becomes a display list once, replayed by a pool of render
  threads for every zoom and tile. Visible tiles come first, then
  neighbours; a quick low resolution pass is replaced by the sharp one.
  Tiles live in a 128 MB cache and are drawn as GPU textures; requests
  that scrolling or zooming made stale are cancelled.
- Selections and annotations being edited are drawn by iced over the
  tiles, so they never cause a re-render.

### Editing and saving

- Every edit is a command with undo and redo. Annotation and page edits
  share one history, undone strictly in order, so the page numbers each
  change holds are right when it is undone. Removed annotations and pages
  stay in the file as objects, so undo puts back the same ones.
- Page edits answer with new document information; the viewer moves every
  cache keyed by page number to the new numbers and drops results still
  on their way from before (an epoch on page-keyed messages).
- Autosave writes in place a moment after edits stop: a temporary file,
  fsync, then an atomic rename; permissions are kept, and on Windows the
  rename is retried briefly while another program holds the file. Before
  the first write the original is kept as a version.
- PDF annotations save incrementally. MuPDF assumes an incremental save
  is appended to the file it opened, so the document is reopened from
  disk after each one. On Windows MuPDF reads the file into memory
  instead, so the file stays free for the next save.
- Redaction never saves incrementally: applying removes text, image
  pixels and drawings under each mark, and the next save rewrites the
  whole file with unused objects dropped. Tests check the saved bytes.
- Annotations go through an engine-neutral model; each carries an `/NM`
  id, and prev names its shapes in `/Subj` so they read back as drawn.
  Loupes and masks are stamps with prev's own appearance. Signatures are
  image stamps; they are stored as PNGs with an index of descriptions.
- MuPDF's page graft leaves annotations and form fields behind, so prev
  copies them onto copied pages itself. MuPDF also shifts or invents page
  labels on insert and delete; prev keeps the labels a document had.
- Markup on images reuses the PDF markup: the image becomes a one-page
  PDF (long side 800 points) held in memory, and export draws the
  annotations, rendered alone, over the image's pixels.

### Images

RAW files are developed by rawler to linear light, then given the
camera's look with a tone curve per channel matched to the JPEG preview
stored in the file (after RawTherapee's auto-matched curve); files
without a preview get a standard curve. Edits keep EXIF, ICC and XMP in
JPEG, PNG, WebP and TIFF.

### Platform layer

Everything tied to one system sits behind a small module with a Linux,
a Windows and a macOS side; the rest of the app is shared.

| Area | Linux | Windows | macOS |
|---|---|---|---|
| Settings and data | XDG folders (`~/.config/prev.toml`, `~/.local/share/prev`) | `%APPDATA%\prev`, `%LOCALAPPDATA%\prev` | `~/Library/Application Support/prev` |
| Single instance | Unix socket | named pipe | Unix socket, and the files Finder and the Dock send the running app |
| File dialogs | XDG desktop portal (ashpd) | rfd, the system dialogs | rfd, the system panels |
| Printing | XDG desktop portal | print dialog and GDI | AppKit's print panel, with pages MuPDF renders |
| Links, reduced motion | XDG desktop portal | URL handler, system setting | NSWorkspace |
| Menus | in the window | in the window | the menu bar (muda), built in the interface language |
| Keyboard layout, for the input language | XKB layout from winit | the input locale | the input source (TIS) |
| Clipboard images and prev's page marker | `wl-copy`, `wl-paste` | clipboard-win | the general pasteboard |
| Drag and drop | vendored smithay-clipboard with a drag and drop patch ([PATCHES.md](../vendor/PATCHES.md)) | OLE drop target, data object and drop source (`dnd_windows.rs`) | files dropped on a window only |
| Theme | Omarchy accent, system light or dark | system light or dark | system light or dark |
| Pictures dropped as web addresses | curl | curl, which Windows includes | not yet |

Drag and drop on Linux and Windows feeds the same drag events, in the types
Paste reads, so dropping works the same everywhere. Hyprland never says
whether a drop copied or moved, so on Linux Shift decides at both ends.

### Interface

Material 3 components live in `crates/prev/src/ui`: toolbars that move
groups that do not fit into a "More" menu, side sheets, dialogs over a
scrim, snackbars, tabs, M3 sliders and fields, state layers and spring
motion. iced 0.14 cannot draw a layer at partial opacity, so things
spring in but do not fade; and it cannot give a Wayland window a parent,
so dialogs such as Settings open inside the window that asked. The
mouse wheel scrolls smoothly: a wrapper around each window turns wheel
notches into short eased glides.

## Dependency and license policy

- `cargo-deny` runs in CI with an allowlist of licenses compatible with
  the AGPL: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib,
  Unicode-3.0, BSL-1.0, CC0-1.0, MPL-2.0, LGPL-2.1-or-later, LGPL-3.0,
  GPL-3.0, AGPL-3.0. GPL-2.0-only is refused.
- A new crate is added only after checking its license and maintenance,
  and whether an existing dependency already does the job.
- RAW support is a cargo feature, so a smaller build is possible.
- MuPDF bundles only the base 14 fonts; other fonts come from the system.
- `docs/THIRD-PARTY.md` lists every bundled component; the Flatpak's crate
  list is checked against `Cargo.lock` in CI.

## Performance

Targets, with the last measurement (Linux, release build, RTX 4080 SUPER,
3840x1080 at 120 Hz). `cargo run --release -p prev-pdf --example
open_bench -- <file>` times opening; `PREV_BENCH=<seconds> prev <file>`
runs a scripted scroll and zoom and prints frame, CPU, memory and tile
statistics.

| Metric | Target | Measured |
|---|---|---|
| Start to window | < 150 ms | ~230 ms; Vulkan setup is ~200 ms of it |
| Open a PDF to its first page drawn | < 100 ms | 20 ms (499 pages), 93 ms (7,025 pages) |
| Scroll and zoom | 60 fps, no dropped frames | 120 fps, no slow frames |
| Idle memory, one 100-page PDF | < 150 MB | 61 MB heap, 179 MB resident with GPU drivers |
| Release binary | < 30 MB without RAW | 29.9 MB (36.1 MB with RAW) |

On the CPU renderer prev manages about 60 fps scrolling and 30 fps zooming,
at four times the CPU. wgpu's OpenGL backend cannot draw to iced's
Wayland windows. Most memory past the tile cache is MuPDF's resource
store (256 MB, not configurable through the `mupdf` crate) and GPU
driver mappings.

## Testing and CI

- Unit tests in each crate, including tests that run only on Windows.
- Render regression: a PDF corpus (a subset of the pdf.js and PDFium test
  suites, fetched in CI, not committed) rendered and compared against
  stored hashes.
- Round trips: annotations, page edits and form values written by prev are
  read back by MuPDF and by Poppler.
- Redaction: text, image pixels and drawings under a mark must be gone
  from the saved bytes.
- Every push and pull request: `cargo fmt`, clippy with warnings as
  errors, the tests and the render regression on Linux, the same build,
  lint and tests on Windows (which also builds the MSI and zip) and on
  macOS (which also builds prev.app),
  `cargo deny`, the minimum Rust version, AppStream and desktop entry
  validation, and the Flatpak crate list check.
- Tags build every package and draft the release ([RELEASING.md](RELEASING.md)):
  x86_64 and ARM64 on GitHub's runners for each, RISC-V cross-compiled
  from x86_64 and started once under QEMU, and the macOS disk image on
  Apple Silicon. Every build runs
  `prev --version` before it is packaged; the ARM64 and RISC-V builds
  are otherwise untested by hand.
- The interface is tested by hand, on Hyprland, in a Windows 11 virtual
  machine and on a Mac mini.
- A test renders every counted string in every language, and one checks
  that no translation uses a key or variable English lacks.

## Risks

| Risk | Mitigation |
|---|---|
| `mupdf-rs` lacks an API prev needs | What prev uses is covered (`crates/prev-pdf/tests/mupdf_capabilities.rs`); stamp appearances are built with the object API. Contribute missing wrappers upstream |
| MuPDF is weaker on JBIG2 refinement and halftone scans | Keep such files in the regression corpus; follow jbig2dec upstream |
| Artifex relicenses MuPDF | Released AGPL versions stay usable; the engine trait allows another engine |
| mupdf-sys builds MuPDF with MSBuild only for x86 and x64 | CI patches its build script for ARM64 (`packaging/windows/mupdf-sys-arm64.ps1`), which fails loudly when mupdf-sys changes; offer the change upstream |
| Drag and drop depends on a vendored smithay-clipboard patch | Re-apply it when iced updates smithay-clipboard; offer it upstream |
| Autosave damaging files | Atomic writes, the original kept as a version, a test that checks every cross-reference offset after repeated saves |
| Redaction leaking content | Dedicated tests; whole-file rewrite only |
| Memory under heavy use, mostly MuPDF's store | Add a store size limit to the `mupdf` crate upstream if it becomes a problem, and remeasure |
| Unsigned Windows downloads trigger SmartScreen warnings | Code signing through the SignPath Foundation |
| Gatekeeper blocks the macOS app until it is notarized | A Developer ID signature and notarization in the release workflow |
| Draft translations read awkwardly | Marked as first drafts in the README; corrections welcome |
