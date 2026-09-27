#!/usr/bin/env bash
# Makes prev-VERSION-ARCH.AppImage from a release build made with
# PREV_PRODUCTION=1:
#
#   packaging/appimage/build.sh target/release/prev VERSION [OUTPUT_DIR]
#
# Needs appimagetool on PATH (or APPIMAGETOOL set to it). The AppImage
# carries prev and its data files; the system provides the graphics,
# Wayland and font libraries, as for the other packages, so build it on
# an old distribution (the release uses Ubuntu 22.04) to run on new ones.
set -euo pipefail

binary=${1:?usage: build.sh BINARY VERSION [OUTPUT_DIR]}
version=${2:?usage: build.sh BINARY VERSION [OUTPUT_DIR]}
output=${3:-.}
root=$(cd "$(dirname "$0")/../.." && pwd)
id=io.github.scrambletools.prev
arch=$(uname -m)
tool=${APPIMAGETOOL:-appimagetool}

appdir=$(mktemp -d)/prev.AppDir
trap 'rm -rf "$(dirname "$appdir")"' EXIT
"$root/scripts/dist-install.sh" "$binary" "$appdir" /usr

# What AppImage tools look for at the top of the AppDir.
cp "$root/data/$id.desktop" "$appdir/$id.desktop"
cp "$root/data/icons/hicolor/256x256/apps/$id.png" "$appdir/$id.png"
ln -s "$id.png" "$appdir/.DirIcon"
cat >"$appdir/AppRun" <<'EOF'
#!/bin/sh
here=$(dirname "$(readlink -f "$0")")
exec "$here/usr/bin/prev" "$@"
EOF
chmod 755 "$appdir/AppRun"

mkdir -p "$output"
ARCH=$arch "$tool" --no-appstream "$appdir" "$output/prev-$version-$arch.AppImage"
