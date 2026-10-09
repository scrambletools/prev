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
| `packaging/windows/` | Windows MSI (WiX 5) and zip, made by `build.ps1`; the ARM64 patch for mupdf-sys; `msvc-toolset.ps1`, which builds MuPDF with the installed Visual Studio's toolset; `bindgen-headers.ps1`, which has MuPDF's bindings read clang's own headers; `vcpkg/`, the pinned, decode-only `heif.dll` (move its baseline to update libheif, libde265 and aom) |
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

Since 2.2.1 the MSI and `prev.exe` are signed with Microsoft's Azure
Artifact Signing (formerly Trusted Signing), under the Scramble Tools
LLC organization, which Windows shows as the publisher. The release
workflow signs while the variables below are set, signing in to Azure
with GitHub's OpenID Connect token, so no secret is stored. To set it
up again, such as in a new account:

1. In the Azure portal, with a subscription, register the resource
   provider `Microsoft.CodeSigning` and create an Artifact Signing
   account (the Basic plan is enough). Note its region's endpoint, such
   as `https://eus.codesigning.azure.net`.
2. In the account, give your own user the role "Artifact Signing
   Identity Verifier", then make an identity validation of type
   Organization for Scramble Tools. Microsoft checks the business's
   registration and may ask for documents; this takes days.
3. Once validated, create a certificate profile of type Public Trust,
   program type None, from that validation; ours is `scramble-apps`.
4. In Microsoft Entra ID, register an app, such as `prev`, and add a
   federated credential for GitHub Actions: organization
   `scrambletoolsllc`, repository `prev`, entity type Environment, name
   `release`. GitHub names the run by its ids, so the subject is
   `repo:scrambletoolsllc@339905752/prev@1388495165:environment:release`.
   Give the app the role "Artifact Signing Certificate Profile Signer"
   on the account (Access control (IAM), Add role assignment, then
   search for the app by name).
5. Set the repository variables (Settings, Secrets and variables,
   Actions, Variables): `AZURE_CLIENT_ID` (the app's client ID),
   `AZURE_TENANT_ID`, `AZURE_SUBSCRIPTION_ID`, `AZURE_SIGNING_ENDPOINT`,
   `AZURE_SIGNING_ACCOUNT` and `AZURE_CERTIFICATE_PROFILE`. The signing
   steps run once `AZURE_SIGNING_ACCOUNT` is set, and the job fails rather
   than ship an unsigned file as signed.
6. Try it without releasing: run the Release workflow by hand with
   Windows only, and check the signature of `prev.exe` in the artifact.
7. After each release, update prev on winget: manifests for
   `ScrambleTools.prev` pointing at the release's MSIs, submitted as a
   pull request to `microsoft/winget-pkgs`.

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

Some Hyprland setups make every window slightly see-through. `scripts/install.sh`
adds a rule keeping prev opaque; packages cannot change a user's
configuration, so the AUR package prints the rule after installing and
the README shows it.
