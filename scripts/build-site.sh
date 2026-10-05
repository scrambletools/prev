#!/usr/bin/env bash
# Assembles prev's website (prev.run) into OUT: the pages in site/, plus
# the icon and the screenshots, from the repository.
#
#   scripts/build-site.sh [OUT]
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=${1:-$root/target/site}

rm -rf "$out"
mkdir -p "$out/screenshots"
cp "$root"/site/* "$out/"
cp "$root/data/icons/hicolor/scalable/apps/io.github.scrambletools.prev.svg" "$out/icon.svg"
cp -r "$root/docs/screenshots/." "$out/screenshots/"
echo "Built the site in $out"
