# Vendored dependencies

## smithay-clipboard 0.7.3

Copied from crates.io and wired in with `[patch.crates-io]` in the root
`Cargo.toml`. License: MIT (see `smithay-clipboard/LICENSE`).

**Why:** winit 0.30 has no file drag and drop on Wayland, and Hyprland
delivers drag events only to the first `wl_data_device` a client creates.
iced's clipboard creates that device, so a second device in prev would never
see a drag.

**Change:** `src/dnd.rs` is new, and the `DataDeviceHandler` hooks in
`src/state.rs` (empty upstream) now accept `text/uri-list` drags with the Copy
action, read the list on drop, finish the offer, and report `DragEvent`s to a
handler installed with `smithay_clipboard::dnd::set_drag_handler`. Clipboard
behaviour is unchanged. The full diff is `smithay-clipboard-dnd.patch`.

**Updating:** when iced moves to a newer smithay-clipboard, re-apply the
patch to that version, or drop the vendored copy if upstream has an
equivalent drag API.
