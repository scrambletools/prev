#!/usr/bin/env bash
# Assembles prev's website (prev.run) into OUT: the pages in site/, plus
# the icon, the screenshots and the font they use, from the repository.
#
#   scripts/build-site.sh [OUT]
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=${1:-$root/target/site}

rm -rf "$out"
mkdir -p "$out/screenshots"
cp "$root"/site/* "$out/"
cp "$root/data/icons/hicolor/scalable/apps/io.github.scrambletools.prev.svg" "$out/icon.svg"
cp "$root/crates/prev/assets/fonts/RobotoFlex.ttf" "$out/"
for shot in markup table-of-contents image-markup pages redaction-applied signatures signature-draw; do
    cp "$root/docs/screenshots/$shot.png" "$out/screenshots/"
done
cp "$root/docs/screenshots/drag-from-browser.gif" "$out/screenshots/"
echo "Built the site in $out"
