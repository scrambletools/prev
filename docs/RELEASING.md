# Releasing prev

Everything a release ships is built from this repository:

| Path | What |
|---|---|
| `data/io.github.scrambletools.prev.desktop` | Desktop entry |
| `data/io.github.scrambletools.prev.metainfo.xml` | AppStream metadata, with the release history |
| `data/icons/hicolor/` | App icon: the SVG and PNGs made from it |
| `docs/prev.1` | Man page |
| `docs/THIRD-PARTY.md` | Bundled components and their licenses, from `scripts/third-party.py` |
| `scripts/dist-install.sh` | Installs a build and the files above into a system layout; every package uses it |
| `packaging/arch/` | AUR packages `prev` (release tarball) and `prev-git` |
| `packaging/nfpm.yaml` | Debian and RPM packages, made with nfpm |
| `packaging/appimage/build.sh` | The AppImage |
| `packaging/flatpak/` | Flatpak manifest and the crate sources it builds from |
| `packaging/windows/` | Windows MSI (WiX 5) and zip, made by `build.ps1`; the ARM64 patch for mupdf-sys; `msvc-toolset.ps1`, which builds MuPDF with the installed Visual Studio's toolset; `vcpkg/`, the pinned, decode-only `heif.dll` (move its baseline to update libheif, libde265 and aom) |
| `packaging/macos/bundle.sh` | prev.app and its disk image, with an Info.plist for the file types it opens |
| `packaging/macos/build-libheif.sh` | The decode-only libheif prev.app carries, from pinned, checksummed sources; update its versions and checksums for new libheif, libde265 and aom releases |
| `scripts/cross-setup.sh` | Prepares Ubuntu to cross-compile for RISC-V |
| `.github/workflows/release.yml` | Builds all of them for a tag and drafts the GitHub release |

Each package is built for x86_64 and ARM64 on GitHub's runners of that
architecture; the Linux .deb, .rpm and tarball are also cross-compiled
for RISC-V, which has no runners. The macOS disk image is built for
Apple Silicon only.

Packages are built with `PREV_PRODUCTION=1`, which makes the installed
copy: settings in `prev.toml` (`~/.config` on Linux, `%APPDATA%\prev` on
Windows, `~/Library/Application Support/prev` on macOS), data under
`prev`, app id `io.github.scrambletools.prev`.
Builds without it are development builds.

## Steps

1. Update the version in `Cargo.toml` (the workspace version) and run
   `cargo check` so `Cargo.lock` follows; update `pkgver` in
   `packaging/arch/prev/PKGBUILD`, the version and date in `docs/prev.1`,
   and add a `<release>` to the metainfo.
2. Write the version's section in `CHANGELOG.md`; the release notes are
   taken from it.
3. After dependency changes, regenerate `docs/THIRD-PARTY.md` with
   `python3 scripts/third-party.py` and `packaging/flatpak/cargo-sources.json`
   with flatpak-builder-tools' `flatpak-cargo-generator.py Cargo.lock`.
   `scripts/check-cargo-sources.py`, also run in CI, fails when the
   Flatpak's list is missing a crate.
4. Try the packaging without releasing: run the Release workflow by hand
   (Actions, Release, Run workflow), which builds every package as an
   artifact, or with "Packages to build" only one system's, such as
   `gh workflow run release.yml -f only=macos`.
5. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`. The workflow
   checks that the tag matches `Cargo.toml`, builds the packages and
   drafts a release with them, `SHA256SUMS`, and a `PKGBUILD` with the
   source checksum filled in, beside its `prev.install`. Check the draft
   and publish it. Then update the version and links on the website
   (`site/index.html`), the version named on `site/specs.html`, and in
   the README's install table; pushing
   `site/` redeploys prev.run.
6. AUR: in a clone of `ssh://aur@aur.archlinux.org/prev.git`, replace
   `PKGBUILD` with the one from the release and copy
   `packaging/arch/prev/prev.install`, run `makepkg --printsrcinfo >.SRCINFO`,
   build it once with `makepkg`, commit and push. `prev-git` needs this
   only when its PKGBUILD changes.
7. The Flatpak is offered only as `prev-x86_64.flatpak` and
   `prev-aarch64.flatpak` on the release; prev is not published on
   Flathub.

## Windows code signing

Until releases are signed, Windows shows SmartScreen's "Windows protected
your PC" warning for the MSI and `prev.exe`. Signing is planned through
the SignPath Foundation, free for open source projects, which signs only
releases built by CI it can check:

1. The project owner applies at signpath.org/apply, with two-factor
   sign-in on GitHub and SignPath, naming who authors, reviews and
   approves each signing.
2. Once approved, create the project `prev` with the signing policy
   `release-signing` and the artifact configurations `exe` and `msi`,
   then set the repository variable `SIGNPATH_ORGANIZATION_ID` and the
   secret `SIGNPATH_API_TOKEN`. The release workflow's signing steps run
   only when that variable is set. The publisher Windows shows is
   "SignPath Foundation".
3. With signed releases, list prev on winget: a manifest pointing at the
   release's MSI, submitted as a pull request to `microsoft/winget-pkgs`.

## macOS signing

The app is signed ad hoc, which Apple Silicon needs to run it at all, so
Gatekeeper blocks a downloaded copy until the user chooses Open Anyway in
System Settings, Privacy & Security. To open without a warning:

1. Join the Apple Developer Program and make a Developer ID Application
   certificate.
2. Store it and an App Store Connect API key as repository secrets, sign
   `prev.app` with it and the hardened runtime in `bundle.sh`, then
   submit the disk image with `xcrun notarytool submit --wait` and
   staple the ticket with `xcrun stapler staple`.
3. With notarized releases, offer a Homebrew cask.

## The Hyprland rule

Omarchy makes every window slightly see-through. `scripts/install.sh`
adds a rule keeping prev opaque; packages cannot change a user's
configuration, so the AUR package prints the rule after installing and
the README shows it.
