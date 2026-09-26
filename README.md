# prev

A fast, open source document and image viewer for Linux, modelled on macOS
Preview. Built in Rust with [iced](https://iced.rs) and
[MuPDF](https://mupdf.com), for Wayland desktops such as Hyprland and
[Omarchy](https://omarchy.org).

> **Status:** early development. PDF, image, SVG and Markdown viewing work
> today; markup and editing are on the way. See the
> [development plan](docs/PLAN.md).

![A PDF with its table of contents in the sidebar](docs/screenshots/table-of-contents.png)

![Searching a PDF, with matches highlighted and page thumbnails in the sidebar](docs/screenshots/search.png)

<sub>Document shown: NIST SP 800-63-3, a US government publication in the
public domain.</sub>

## Features

### Available now

- **PDF viewing**
  - Continuous, single page and two page layouts.
  - Zoom, fit to page or width, actual size, Ctrl+scroll zoom.
  - Sharp at any zoom: pages render in tiles on background threads.
  - Password protected documents.
- **Sidebar:** page thumbnails, table of contents and bookmarks.
- **Search** with every match highlighted and next/previous navigation.
- **Text selection and copy**, including across pages; double-click selects a
  word, triple-click a line.
- **Links** inside the document and to the web.
- **Slideshow**, full screen and printing through the system print dialog.
- **Images:** PNG, JPEG, GIF and animated GIF, WebP, AVIF, HEIC, TIFF, BMP,
  ICO, TGA, PNM, QOI, JPEG 2000, OpenEXR, Radiance HDR and camera RAW
  from the cameras [rawler](https://github.com/dnglab/dnglab) supports.
  - Images opened together share one window with a thumbnail sidebar.
  - Zoom, fit, actual size, Ctrl+scroll zoom and drag to pan.
  - HEIC and AVIF use the system's libheif when it is installed.
- **SVG** drawings, sharp at any zoom.
- **Markdown** with tables, task lists, syntax highlighted code and images;
  reloads when the file changes on disk.
- **Desktop integration**
  - One window per document, with a single running instance.
  - Open files from the file dialog or by dragging them onto a window.
  - Follows the system light or dark setting, or the active Omarchy theme.

### Planned

- **Image editing:** crop, rotate, resize, color adjustment and metadata.
- **PDF markup:** highlights, shapes, notes, text boxes and signatures.
- **PDF editing:** reorder, insert and delete pages, true redaction,
  encryption and export to images.

## Building

prev needs Linux, Rust 1.89 or newer, a Vulkan capable GPU driver, and these
build dependencies:

```sh
# Arch
sudo pacman -S clang pkgconf fontconfig freetype2 libxkbcommon wayland
# Debian and Ubuntu
sudo apt install clang libclang-dev pkg-config libfontconfig1-dev libfreetype-dev libxkbcommon-dev libwayland-dev
```

Then build and run:

```sh
cargo build --release
./target/release/prev document.pdf
```

MuPDF is compiled from source as part of the build.

## Keyboard shortcuts

| Action | Shortcut |
|---|---|
| Open | Ctrl+O |
| Find, next, previous | Ctrl+F, Ctrl+G, Ctrl+Shift+G |
| Zoom in, out, actual size, fit | Ctrl+=, Ctrl+-, Ctrl+0, Ctrl+9 |
| Go to page | Ctrl+Alt+G |
| Bookmark page | Ctrl+D |
| Sidebar: hide, thumbnails, contents, bookmarks | Ctrl+Alt+1, 2, 3, 5 |
| Slideshow | Ctrl+Shift+F |
| Full screen | F11 |
| Print | Ctrl+P |
| Settings | Ctrl+, |

## License

prev is licensed under the [GNU Affero General Public License v3.0 or
later](LICENSE), as required by MuPDF.
