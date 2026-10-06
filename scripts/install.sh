#!/usr/bin/env bash
# Builds prev as the production copy and installs it for the current user:
# the binary in ~/.local/bin, and the desktop entry, icons, metadata, man
# page and licenses under ~/.local/share. Development builds (plain cargo
# build or cargo run) keep separate settings, data and instance socket,
# so they can run next to it.
#
# On Omarchy, it also keeps prev's windows fully opaque: Omarchy makes
# every window slightly see-through, which dims photos and pages. Set
# PREV_NO_HYPRLAND=1 to leave the Hyprland configuration alone.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
prefix=${PREV_PREFIX:-$HOME/.local}
bin_dir=$prefix/bin
# Launchers such as Omarchy's read the session's data directory, which a
# shell's own XDG_DATA_HOME may not match, so use the standard location.
share_dir=$prefix/share

# A target directory of its own, so switching between production and
# development builds does not rebuild everything each time.
PREV_PRODUCTION=1 cargo build --release --locked -p prev \
    --manifest-path "$root/Cargo.toml" --target-dir "$root/target/production"

"$root/scripts/dist-install.sh" "$root/target/production/release/prev" "" "$prefix"
desktop="$share_dir/applications/io.github.scrambletools.prev.desktop"
sed -i "s|^Exec=prev |Exec=$bin_dir/prev |" "$desktop"
if command -v update-desktop-database >/dev/null; then
    update-desktop-database "$share_dir/applications"
fi
if command -v gtk-update-icon-cache >/dev/null && [ -f "$share_dir/icons/hicolor/index.theme" ]; then
    gtk-update-icon-cache -q "$share_dir/icons/hicolor" || true
fi

# Omarchy tags every window for slight transparency; prev opts out, as
# Omarchy's own media apps do. Only added once, and undone if Hyprland
# does not take it.
hypr_dir=${XDG_CONFIG_HOME:-$HOME/.config}/hypr
hypr_config=$hypr_dir/hyprland.lua
if [ -z "${PREV_NO_HYPRLAND:-}" ] && [ -f "$hypr_config" ] && command -v omarchy >/dev/null; then
    if grep -rqs 'scrambletools' "$hypr_dir"; then
        echo "Hyprland already has a rule for prev windows; left as it is."
    else
        backup="$hypr_config.bak.$(date +%s)"
        cp "$hypr_config" "$backup"
        cat >>"$hypr_config" <<'EOF'

-- Added by prev's install script: keep prev (and its dev builds) fully
-- opaque, so photos and pages show their true colors.
o.window("^io\\.github\\.scrambletools\\.prev(\\.Devel)?$", { tag = "-default-opacity", opacity = "1 1" })
EOF
        if command -v hyprctl >/dev/null && [ -n "${HYPRLAND_INSTANCE_SIGNATURE:-}" ]; then
            hyprctl reload >/dev/null
            if [ -n "$(hyprctl configerrors 2>/dev/null | grep -v '^$' || true)" ]; then
                cp "$backup" "$hypr_config"
                hyprctl reload >/dev/null
                echo "Hyprland did not take the opacity rule; $hypr_config is back as it was."
            else
                echo "Added a Hyprland rule keeping prev opaque (backup: $backup)."
            fi
        else
            echo "Added a Hyprland rule keeping prev opaque (backup: $backup); it applies at the next login."
        fi
    fi
fi

# The version the installed program reports, from Cargo.toml and the commit.
echo "Installed $("$bin_dir/prev" --version) to $bin_dir/prev"
