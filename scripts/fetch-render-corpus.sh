#!/usr/bin/env bash
# Downloads the render regression corpus (pdf.js test files at a pinned
# commit) into the given directory and verifies the checksums.
set -euo pipefail

commit=d52fdf411a6e4d338180687456e0df019e28475e
base="https://raw.githubusercontent.com/mozilla/pdf.js/$commit/test/pdfs"
list="$(cd "$(dirname "$0")/.." && pwd)/crates/prev-pdf/tests/render-corpus/files.sha256"
target=${1:?usage: fetch-render-corpus.sh <directory>}

mkdir -p "$target"
cd "$target"
while read -r _ name; do
    [[ -f $name ]] || curl -fsSL --retry 3 -o "$name" "$base/$name"
done < "$list"
sha256sum --check --quiet "$list"
