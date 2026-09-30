#!/bin/bash
# Builds the libheif that prev.app carries: decoding only, HEIC through
# libde265 and AVIF through aom's decoder, both linked in statically so
# the app has one library to bundle. Built for macOS 11, as prev is, and
# without Homebrew's copies, so it runs on Macs that have neither.
#
#   packaging/macos/build-libheif.sh [OUT]
#
# Leaves OUT/lib/libheif.1.dylib, which bundle.sh copies into the app.
# OUT defaults to target/libheif.
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
out=$(mkdir -p "${1:-$root/target/libheif}" && cd "${1:-$root/target/libheif}" && pwd)

libheif=1.23.5
libheif_sha=fd9036064c4432f0550d15072ddf34956a248279ee9aeaff0fba3fa0f77d8f1a
libde265=1.1.3
libde265_sha=554228bd17788c99a7e63b37ab5634722190e6e2bf60c1dcb01cef328e133905
aom=3.15.1
aom_sha=8ca0c52746174603500f0adb6f2a215d69c9ca2aab2acb3caa06fb791d8d01bf

if [ -e "$out/lib/libheif.1.dylib" ] && [ "$(cat "$out/versions" 2>/dev/null)" = "$libheif $libde265 $aom" ]; then
    echo "$out/lib/libheif.1.dylib"
    exit 0
fi

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
prefix="$work/prefix"

fetch() {
    local url=$1 sha=$2 file
    file="$work/$(basename "$url")"
    curl -sSfL -o "$file" "$url"
    echo "$sha  $file" | shasum -a 256 -c --quiet
    tar -xzf "$file" -C "$work"
}
fetch "https://storage.googleapis.com/aom-releases/libaom-$aom.tar.gz" $aom_sha
fetch "https://github.com/strukturag/libde265/releases/download/v$libde265/libde265-$libde265.tar.gz" $libde265_sha
fetch "https://github.com/strukturag/libheif/releases/download/v$libheif/libheif-$libheif.tar.gz" $libheif_sha

# Each part finds only what was built here: Homebrew's libraries would
# make the result depend on them.
common=(
    -G Ninja
    -DCMAKE_BUILD_TYPE=Release
    -DCMAKE_OSX_DEPLOYMENT_TARGET=11.0
    -DCMAKE_OSX_ARCHITECTURES=arm64
    -DCMAKE_INSTALL_PREFIX="$prefix"
    -DCMAKE_PREFIX_PATH="$prefix"
    "-DCMAKE_IGNORE_PREFIX_PATH=/opt/homebrew;/usr/local"
)
export PKG_CONFIG_LIBDIR="$prefix/lib/pkgconfig"
export PKG_CONFIG_PATH="$prefix/lib/pkgconfig"

cmake -S "$work/libaom-$aom" -B "$work/aom-build" "${common[@]}" \
    -DBUILD_SHARED_LIBS=OFF -DCONFIG_AV1_ENCODER=0 \
    -DENABLE_DOCS=0 -DENABLE_EXAMPLES=0 -DENABLE_TESTS=0 -DENABLE_TOOLS=0
cmake --build "$work/aom-build"
cmake --install "$work/aom-build"

cmake -S "$work/libde265-$libde265" -B "$work/de265-build" "${common[@]}" \
    -DBUILD_SHARED_LIBS=OFF -DENABLE_DECODER=OFF -DENABLE_ENCODER=OFF -DENABLE_SDL=OFF
cmake --build "$work/de265-build"
cmake --install "$work/de265-build"

cmake -S "$work/libheif-$libheif" -B "$work/heif-build" "${common[@]}" \
    -DBUILD_SHARED_LIBS=ON -DENABLE_PLUGIN_LOADING=OFF \
    -DWITH_LIBDE265=ON -DWITH_AOM_DECODER=ON \
    -DWITH_AOM_ENCODER=OFF -DWITH_X265=OFF -DWITH_X264=OFF -DWITH_OpenH264_DECODER=OFF \
    -DWITH_LIBSHARPYUV=OFF -DWITH_EXAMPLES=OFF -DWITH_GDK_PIXBUF=OFF \
    -DBUILD_TESTING=OFF -DBUILD_DOCUMENTATION=OFF
cmake --build "$work/heif-build"
cmake --install "$work/heif-build"

mkdir -p "$out/lib"
cp -L "$prefix/lib/libheif.1.dylib" "$out/lib/libheif.1.dylib"
# Anything outside the system would be missing on other Macs.
if otool -L "$out/lib/libheif.1.dylib" | awk 'NR > 2 {print $1}' | grep -v -E '^(/usr/lib/|/System/)'; then
    echo "libheif depends on libraries outside the system" >&2
    exit 1
fi
cp "$work/libheif-$libheif/COPYING" "$out/COPYING-libheif"
cp "$work/libde265-$libde265/COPYING" "$out/COPYING-libde265"
cp "$work/libaom-$aom/LICENSE" "$out/LICENSE-aom"
cp "$work/libaom-$aom/PATENTS" "$out/PATENTS-aom"
echo "$libheif $libde265 $aom" >"$out/versions"
echo "$out/lib/libheif.1.dylib"
