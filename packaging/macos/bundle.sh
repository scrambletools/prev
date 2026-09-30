#!/bin/bash
# Builds prev.app on macOS: the release binary, its icon, and an
# Info.plist that declares the files prev opens, so Finder offers it in
# Open With and hands it files through the app delegate. The app is
# signed ad hoc, which Apple Silicon needs to run it at all; a Developer
# ID signature and notarization come later (docs/RELEASING.md).
#
#   packaging/macos/bundle.sh [--dmg] [--out DIR]
#
# PREV_PRODUCTION=1 makes a release build, as in CI; without it the app
# is the development build, with its own settings and data.
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
out="$root/target/macos"
dmg=false
while [ $# -gt 0 ]; do
    case "$1" in
        --dmg) dmg=true ;;
        --out) out=$2; shift ;;
        *) echo "unknown option: $1" >&2; exit 2 ;;
    esac
    shift
done

version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)
if [ -n "${PREV_PRODUCTION:-}" ]; then
    name=prev; id=io.github.scrambletools.prev
else
    name=prev-dev; id=io.github.scrambletools.prev.dev
fi

export LIBCLANG_PATH=${LIBCLANG_PATH:-/Library/Developer/CommandLineTools/usr/lib}
cargo build --release --locked -p prev --manifest-path "$root/Cargo.toml"

app="$out/$name.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources" "$app/Contents/Frameworks"
cp "$root/target/release/prev" "$app/Contents/MacOS/prev"

# The icon: each size an .iconset asks for, from the PNGs the Linux
# packages use; 1024 is scaled up from 512.
icons="$root/data/icons/hicolor"
iconset=$(mktemp -d)/prev.iconset
mkdir -p "$iconset"
for size in 16 32 128 256 512; do
    cp "$icons/${size}x${size}/apps/io.github.scrambletools.prev.png" "$iconset/icon_${size}x${size}.png"
done
cp "$icons/32x32/apps/io.github.scrambletools.prev.png" "$iconset/icon_16x16@2x.png"
cp "$icons/64x64/apps/io.github.scrambletools.prev.png" "$iconset/icon_32x32@2x.png"
cp "$icons/256x256/apps/io.github.scrambletools.prev.png" "$iconset/icon_128x128@2x.png"
cp "$icons/512x512/apps/io.github.scrambletools.prev.png" "$iconset/icon_256x256@2x.png"
sips -z 1024 1024 "$icons/512x512/apps/io.github.scrambletools.prev.png" \
    --out "$iconset/icon_512x512@2x.png" >/dev/null
iconutil -c icns "$iconset" -o "$app/Contents/Resources/prev.icns"

cat > "$app/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>$name</string>
    <key>CFBundleDisplayName</key><string>$name</string>
    <key>CFBundleIdentifier</key><string>$id</string>
    <key>CFBundleExecutable</key><string>prev</string>
    <key>CFBundleIconFile</key><string>prev</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>$version</string>
    <key>CFBundleVersion</key><string>$version</string>
    <key>LSMinimumSystemVersion</key><string>11.0</string>
    <key>LSApplicationCategoryType</key><string>public.app-category.productivity</string>
    <key>NSHighResolutionCapable</key><true/>
    <key>NSHumanReadableCopyright</key><string>AGPL-3.0-or-later</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key><string>PDF document</string>
            <key>CFBundleTypeRole</key><string>Editor</string>
            <key>LSHandlerRank</key><string>Alternate</string>
            <key>LSItemContentTypes</key><array><string>com.adobe.pdf</string></array>
        </dict>
        <dict>
            <key>CFBundleTypeName</key><string>Image</string>
            <key>CFBundleTypeRole</key><string>Editor</string>
            <key>LSHandlerRank</key><string>Alternate</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>public.image</string>
                <string>public.camera-raw-image</string>
                <string>public.svg-image</string>
            </array>
        </dict>
        <dict>
            <key>CFBundleTypeName</key><string>Markdown document</string>
            <key>CFBundleTypeRole</key><string>Viewer</string>
            <key>LSHandlerRank</key><string>Alternate</string>
            <key>LSItemContentTypes</key><array><string>net.daringfireball.markdown</string></array>
        </dict>
    </array>
    <key>UTImportedTypeDeclarations</key>
    <array>
        <dict>
            <key>UTTypeIdentifier</key><string>net.daringfireball.markdown</string>
            <key>UTTypeDescription</key><string>Markdown document</string>
            <key>UTTypeConformsTo</key><array><string>public.plain-text</string></array>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key>
                <array><string>md</string><string>markdown</string></array>
            </dict>
        </dict>
    </array>
</dict>
</plist>
EOF
plutil -lint "$app/Contents/Info.plist" >/dev/null

# HEIC and AVIF: a libheif copied from Homebrew, when there is one, with
# the libraries it needs, pointed at the bundle's Frameworks folder.
for prefix in /opt/homebrew /usr/local; do
    heif="$prefix/lib/libheif.1.dylib"
    [ -e "$heif" ] || continue
    cp -L "$heif" "$app/Contents/Frameworks/"
    for dependency in $(otool -L "$heif" | awk 'NR > 1 {print $1}' | grep "^$prefix/"); do
        base=$(basename "$dependency")
        cp -L "$dependency" "$app/Contents/Frameworks/$base"
        install_name_tool -change "$dependency" "@loader_path/$base" \
            "$app/Contents/Frameworks/libheif.1.dylib"
    done
    install_name_tool -id @executable_path/../Frameworks/libheif.1.dylib \
        "$app/Contents/Frameworks/libheif.1.dylib"
    break
done

codesign --force --deep --sign - "$app"
echo "$app"

if $dmg; then
    stage=$(mktemp -d)
    cp -R "$app" "$stage/"
    ln -s /Applications "$stage/Applications"
    image="$out/$name-$version-$(uname -m).dmg"
    rm -f "$image"
    hdiutil create -volname "$name" -srcfolder "$stage" -ov -format UDZO "$image" >/dev/null
    echo "$image"
fi
