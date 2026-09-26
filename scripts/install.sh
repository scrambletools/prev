#!/usr/bin/env bash
# Builds prev as the production copy and installs it for the current user:
# the binary in ~/.local/bin and the desktop entry in
# ~/.local/share/applications. Development builds (plain cargo build or
# cargo run) keep separate settings, data and instance socket, so they can
# run next to it.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
bin_dir=${PREV_BIN_DIR:-$HOME/.local/bin}
# Launchers such as Omarchy's read the session's data directory, which a
# shell's own XDG_DATA_HOME may not match, so use the standard location.
apps_dir=${PREV_APPS_DIR:-$HOME/.local/share/applications}

# A target directory of its own, so switching between production and
# development builds does not rebuild everything each time.
PREV_PRODUCTION=1 cargo build --release --locked -p prev \
    --manifest-path "$root/Cargo.toml" --target-dir "$root/target/production"

install -Dm755 "$root/target/production/release/prev" "$bin_dir/prev"
desktop="$apps_dir/io.github.scrambletools.prev.desktop"
install -Dm644 "$root/data/io.github.scrambletools.prev.desktop" "$desktop"
sed -i "s|^Exec=prev |Exec=$bin_dir/prev |" "$desktop"
if command -v update-desktop-database >/dev/null; then
    update-desktop-database "$apps_dir"
fi

echo "Installed prev $(git -C "$root" describe --always --dirty) to $bin_dir/prev"
