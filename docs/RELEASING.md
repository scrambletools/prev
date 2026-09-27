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
| `.github/workflows/release.yml` | Builds all of them for a tag and drafts the GitHub release |

Packages are built with `PREV_PRODUCTION=1`, which makes the installed
copy: settings in `~/.config/prev.toml`, data under `prev`, app id
`io.github.scrambletools.prev`. Builds without it are development builds.

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
4. Try the packaging without releasing: run the Release workflow by hand
   (Actions, Release, Run workflow), which builds every package as an
   artifact.
5. Tag and push: `git tag v1.0.0 && git push origin v1.0.0`. The workflow
   checks that the tag matches `Cargo.toml`, builds the packages and
   drafts a release with them, `SHA256SUMS`, and a `PKGBUILD` with the
   source checksum filled in, beside its `prev.install`. Check the draft
   and publish it.
6. AUR: in a clone of `ssh://aur@aur.archlinux.org/prev.git`, replace
   `PKGBUILD` with the one from the release and copy
   `packaging/arch/prev/prev.install`, run `makepkg --printsrcinfo >.SRCINFO`,
   build it once with `makepkg`, commit and push. `prev-git` needs this
   only when its PKGBUILD changes.
7. Flathub (first time): open a pull request against
   `flathub/flathub` adding the manifest, with the prev source changed to
   a `git` source pinned to the tag and commit, and `cargo-sources.json`
   beside it. Later releases go to the `flathub/io.github.scrambletools.prev`
   repository Flathub creates.

## The Hyprland rule

Omarchy makes every window slightly see-through. `scripts/install.sh`
adds a rule keeping prev opaque; packages cannot change a user's
configuration, so the AUR package prints the rule after installing and
the README shows it.
