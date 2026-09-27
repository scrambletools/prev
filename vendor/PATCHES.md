# Vendored dependencies

## smithay-clipboard 0.7.3

Copied from crates.io and wired in with `[patch.crates-io]` in the root
`Cargo.toml`. License: MIT (see `smithay-clipboard/LICENSE`).

**Why:** winit 0.30 has no file drag and drop on Wayland, and Hyprland
delivers drag events only to the first `wl_data_device` a client creates.
iced's clipboard creates that device, so a second device in prev would never
see a drag.

**Change:** `src/dnd.rs` is new, and `src/state.rs` (whose drag hooks are
empty upstream) now handles drag and drop both ways, reported to a handler
installed with `smithay_clipboard::dnd::set_drag_handler`:

- Drops: drags offering any of the types given to `set_accepted_mimes`
  (a trailing `*` matches by prefix, for Chromium's
  `application/octet-stream;name="…"` file contents) are accepted in the
  most preferred one, with copy or move (move while
  `set_prefer_move` says the user holds Shift). On drop the data is read
  and reported as `DragEvent::Dropped` with its type and action, before
  the offer is finished.
- Drags: `dnd::start_drag` starts a drag from the surface where the
  pointer button is held (the press serial is tracked from the worker's
  own pointer), offering each given type with its bytes and an optional
  icon drawn into a `wl_shm` buffer on its own surface (`wl_compositor`
  and `wl_shm` are bound for this). `DragEvent::SourceEnded` reports the
  outcome.
- Hyprland sends no `action` events, so when none arrives the Shift key
  decides between copy and move on both sides.

Clipboard behaviour is unchanged. The full diff is
`smithay-clipboard-dnd.patch` (`diff -ruN` of `src/` against the crates.io
release).

**Updating:** when iced moves to a newer smithay-clipboard, re-apply the
patch to that version, or drop the vendored copy if upstream has an
equivalent drag API.
