# prev on Windows: plan

Goal: a Windows 10 and 11 (x86_64) build of prev with the same features
as on Linux, installed with an MSI and listed on winget, built by the
release workflow next to the Linux packages.

## Where things stand

Most of prev is already portable:

- iced renders through wgpu, which uses DirectX 12 or Vulkan on Windows.
  Fonts are bundled. The system light or dark mode comes from
  `iced::system::theme()`, which works on Windows.
- MuPDF's Rust bindings (`mupdf-sys` 0.8) build with MSVC through
  msbuild, and its `system-fonts` feature uses font-kit, which reads
  fonts through DirectWrite.
- Image decoding (`image`, rawler for RAW, resvg for SVG), Markdown,
  editing, markup, forms, signatures, page editing, redaction and
  encryption are pure Rust or MuPDF, with no Linux calls.

What is tied to Linux, by file:

| Area | Linux today | Files |
|---|---|---|
| Single instance | Unix socket in the runtime dir | `prev/src/instance.rs` |
| File dialogs | XDG portal (ashpd) | `prev/src/dialog.rs` |
| Open links, print, reduced motion | XDG portal (ashpd) | `prev/src/portal.rs` |
| Clipboard images, prev's page marker | `wl-copy` and `wl-paste` | `prev/src/paste.rs`, `pdf/viewer/editing.rs` |
| Drag and drop, both ways | vendored smithay-clipboard patch | `prev/src/drag.rs`, `main.rs`, `app.rs`, `vendor/` |
| Where settings and data live | XDG base directories | `prev-store/src/paths.rs` |
| File modes, path bytes | `std::os::unix` | `prev-store/src/atomic.rs`, `versions.rs`, `dialog.rs` |
| HEIC and AVIF | system `libheif.so`, loaded at run time | `prev-image/src/heif.rs` |
| Omarchy theme and Hyprland rule | Omarchy files, Hyprland | `prev/src/omarchy.rs`, `scripts/install.sh` |
| Window setup | Wayland app id and surface | `prev/src/app.rs` |
| Pictures dropped as web links | `curl` | `prev/src/drag.rs` (Windows 10 1803+ ships `curl.exe`, so this can stay) |

## Approach

Put each of these behind a small platform module with a Linux and a
Windows side, chosen with `cfg`, so the rest of the app does not change.
Where one portable crate covers both systems well, use it on both and
drop the Linux-only code.

Windows API calls need `unsafe`, which the workspace denies; allow it
only in the Windows platform modules, as `heif.rs` already does for
loading libheif.

## Milestones

### W1: it builds (small)

- Gate the Linux modules (`omarchy`, the Wayland window setup, the
  smithay drag and drop) behind `cfg(target_os = "linux")`, with Windows
  stubs that do nothing, so the app compiles and runs with those
  features missing.
- Gate `std::os::unix` uses: file modes in `atomic.rs` apply on Unix only;
  path hashing in `versions.rs` and path conversion in `dialog.rs` go
  through a portable encoding.
- Add a `windows-latest` job to `ci.yml`: build, clippy and tests on MSVC.
  GitHub's Windows runners have MSVC and LLVM (for bindgen) installed.
- `#![windows_subsystem = "windows"]` so no console window opens, with
  `AttachConsole` for `--help` and `--version` when run from a terminal.

Done when CI builds and tests pass on Windows and a PDF opens.

### W2: the platform layer (medium)

- **Paths:** settings and data in `%APPDATA%\prev` (settings file
  `prev.toml`), caches and state in `%LOCALAPPDATA%\prev`, through the
  `dirs` crate. Development builds keep `prev-dev`, as on Linux.
- **Single instance:** a named pipe per user instead of the Unix socket,
  keeping the same request format. The `interprocess` crate's local
  sockets cover both (Unix sockets and named pipes), so `instance.rs`
  can use it everywhere.
- **File dialogs:** `rfd`, which uses the native Windows dialogs. Keep
  ashpd on Linux, since the portal also works inside the Flatpak.
- **Open links:** `ShellExecuteW` (or the `open` crate).
- **Reduced motion:** `SystemParametersInfo(SPI_GETCLIENTAREAANIMATION)`.
- **Clipboard:** `clipboard-win` for images (CF_DIB and PNG) and for
  prev's page marker, registered as a custom clipboard format.
  Text keeps going through iced.
- **Atomic saves:** `std::fs::rename` replaces files on Windows, but fails
  when another program has the file open. Retry briefly and report it
  clearly instead of failing silently.

Done when the settings dialog, open and save dialogs, copy and paste of
text, images and pages, links and second launches all work.

### W3: drag and drop (medium to large)

- **Files dropped on prev:** turn off winit's built-in handler
  (`drag_and_drop: false` in iced's Windows window settings) and register
  prev's own OLE drop target, which reads files (CF_HDROP), images,
  text and web links, so drops go through the same paths as on Linux.
  Browser image drags arrive as files described by
  `FileGroupDescriptorW`, which the drop target reads too.
- **Drags out of prev** (pages, text, images, sidebar files): start OLE
  drags with `DoDragDrop`, offering files for the file explorer and data
  for other apps. The `drag` crate covers files and could be a first
  step. Windows reports whether the drop copied or moved, so the Shift
  workaround needed on Hyprland is not needed there.

Done when the drag and drop tables in the README hold on Windows.

### W4: printing and HEIC (medium)

- **Printing:** there is no print portal. Show the Windows print dialog
  (`PrintDlgEx`), render each page with MuPDF at the printer's
  resolution and print it through GDI (`StartDoc`, `StretchDIBits`).
- **HEIC and AVIF:** bundle `libheif.dll` with libde265 and the dav1d
  or aom decoders, built with vcpkg in CI, and add the DLL names to the
  list `heif.rs` loads. This matches the Flatpak, which bundles them.
  The other way, Windows' own WIC decoders, needs Microsoft Store
  extensions (HEVC is paid), so it is less dependable.

Done when printing a PDF and opening HEIC and AVIF photos work.

### W5: packaging and release (medium)

- **Icon and version:** embed an `.ico`, made from the existing icon SVG,
  and version information into `prev.exe` (the `winresource` crate).
- **Installer:** a per-user MSI made with WiX, which needs no
  administrator rights. It adds a Start menu entry and registers prev
  for "Open with" and Default apps: ProgIDs, `Capabilities` and
  `RegisteredApplications` for PDF, image, SVG and Markdown types. Windows
  10 and 11 do not let apps make themselves the default; users choose
  prev in Settings.
- **Portable zip:** `prev.exe` and its DLLs.
- **Signing:** unsigned downloads get SmartScreen's "Windows protected
  your PC" warning. Options include Azure Trusted Signing (paid, needs
  identity validation) or SignPath's free program for open source
  projects; check their current eligibility rules.
- **winget:** a manifest pointing at the release's MSI. It is submitted
  as a PR to `microsoft/winget-pkgs`; check its rules on automated and
  AI-made submissions before preparing one.
- **Release workflow:** a Windows job in `release.yml` builds the MSI and
  the zip and adds them to the draft release.
- **Docs:** Windows rows in the README install table and a "What
  differs" note; Windows paths in the guide's settings section.

## Differences that stay

- No Omarchy theme or Hyprland rule on Windows; prev uses its own colors
  or the system's light and dark mode.
- The Windows build only covers x86_64 at first; ARM64 needs its own
  MuPDF build and a runner to test on.

## Decisions to make

1. **A Windows machine to test on.** CI can build and run the tests, but
   the UI (drag and drop, printing, dialogs, the installer) needs a real
   Windows 10 or 11 machine or a virtual machine.
2. **Code signing,** and whether its yearly cost is worth it for the
   first Windows release.
3. **HEIC:** bundle libheif (recommended above) or use WIC.

## Risks

- **MuPDF with MSVC:** supported by `mupdf-sys`, but untested with prev's
  feature set and `lto = "thin"`; check this first in W1.
- **wgpu on older or virtual GPUs:** prev needs DirectX 12 or Vulkan;
  some virtual machines only offer software rendering. wgpu can fall
  back to WARP (software DirectX 12), which is slow but works.
- **OLE drag and drop** is the largest piece of new code, and iced gives
  no help with it; W3 may need changes to how `drag.rs` hands data to
  the platform side.
