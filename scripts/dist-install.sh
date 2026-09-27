#!/usr/bin/env bash
# Installs a built prev and its data files into a system layout, for
# packages: the binary, desktop entry, icons, AppStream metadata, man page
# and licenses.
#
#   scripts/dist-install.sh BINARY [DESTDIR] [PREFIX] [DATA_DIR]
#
# BINARY is a release build made with PREV_PRODUCTION=1. DESTDIR is the
# staging root (empty for a direct install) and PREFIX defaults to /usr.
# DATA_DIR, which defaults to PREFIX/share, takes the files desktops look
# up through XDG: the desktop entry, icons and AppStream metadata.
set -euo pipefail

binary=${1:?usage: dist-install.sh BINARY [DESTDIR] [PREFIX] [DATA_DIR]}
destdir=${2:-}
prefix=${3:-/usr}
root=$(cd "$(dirname "$0")/.." && pwd)
id=io.github.scrambletools.prev
share="$destdir$prefix/share"
xdg="$destdir${4:-$prefix/share}"

install -Dm755 "$binary" "$destdir$prefix/bin/prev"
install -Dm644 "$root/data/$id.desktop" "$xdg/applications/$id.desktop"
install -Dm644 "$root/data/$id.metainfo.xml" "$xdg/metainfo/$id.metainfo.xml"
(cd "$root/data/icons" && find hicolor -type f) | while read -r icon; do
    install -Dm644 "$root/data/icons/$icon" "$xdg/icons/$icon"
done
install -d "$share/man/man1"
gzip -9n <"$root/docs/prev.1" >"$share/man/man1/prev.1.gz"
chmod 644 "$share/man/man1/prev.1.gz"
install -Dm644 "$root/LICENSE" "$share/licenses/prev/LICENSE"
install -Dm644 "$root/docs/THIRD-PARTY.md" "$share/licenses/prev/THIRD-PARTY.md"
for font_license in OFL.txt OFL-DancingScript.txt LICENSE-MaterialSymbols.txt; do
    install -Dm644 "$root/crates/prev/assets/fonts/$font_license" \
        "$share/licenses/prev/fonts/$font_license"
done
install -Dm644 "$root/README.md" "$share/doc/prev/README.md"
install -Dm644 "$root/docs/GUIDE.md" "$share/doc/prev/GUIDE.md"
