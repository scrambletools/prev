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

**Change:** `src/lib.rs` shares one worker among all `Clipboard`s on a
display, and dropping them no longer stops it; `smithay_clipboard::shutdown`
does, and prev calls it just before exiting, while the Wayland connection
is still open.

**Why:** iced makes a clipboard per window and drops it with the window.
When prev replaced its start window with a document window, the worker
released its data devices just as Hyprland sent them a new selection
offer. libwayland discards events for destroyed objects without taking
the server-allocated ids they carry, so the next offer's id left a gap in
its object map ("not a valid new object id"), and the connection, and
prev, ended.

Clipboard behaviour is otherwise unchanged. The full diff is
`smithay-clipboard-dnd.patch` (`diff -ruN` of `src/` against the crates.io
release).

**Updating:** when iced moves to a newer smithay-clipboard, re-apply the
patch to that version, or drop the vendored copy if upstream has an
equivalent drag API.

## iced_graphics 0.14.0 and iced_widget 0.14.2

Copied from crates.io and wired in with `[patch.crates-io]` in the root
`Cargo.toml`. License: MIT (see each crate's `LICENSE`).

**Why:** iced's text input and pick list assume left to right text. In
right to left text the cursor was drawn at the wrong character, a
selection was not drawn at all, the arrow keys moved the cursor against
the way they point, and a click or drag started left of the text began
at its start instead of its end. Pick lists kept their text on the left
in right to left interfaces.

**Changes:**

- `iced_graphics/src/text/paragraph.rs`: `grapheme_position` finds the
  grapheme's glyph by its byte offset (glyphs are in visual order) and
  measures a right to left glyph from its right edge; past the end of the
  text it uses the logically last glyph.
- `iced_widget/src/text_input.rs`: a selection is drawn from its leftmost
  to its rightmost edge, since in right to left text its logical start is
  on the right; in text that starts right to left, Left and Right are
  swapped so the cursor moves the way the arrow points; a press left of
  such text puts the cursor at its end, which is on the left; and the
  cursor of an empty field goes by the value, not the placeholder, so in
  a right aligned field it sits at the right edge.
- `iced_widget/src/text_input.rs` also takes a `placeholder_align`, so
  a search box's placeholder can sit on the interface's side while an
  empty field's cursor follows the language typed in.
- `iced_widget/src/pick_list.rs` and `src/overlay/menu.rs`: a
  `right_to_left` option puts a pick list's text and its menu's options
  on the right and the handle on the left. The text is measured and
  drawn from its left edge, since right aligned text laid out wider than
  itself is misplaced.

The full diffs are `iced_graphics-rtl.patch` and `iced_widget-rtl.patch`
(`diff -ruN` of `src/` against the crates.io releases). A test in
`crates/prev/src/ui/font.rs` checks the cursor positions.

**Updating:** when iced updates, re-apply the patches to the new versions,
or drop the vendored copies if iced handles right to left text input.

## winit 0.30.13

Copied from crates.io and wired in with `[patch.crates-io]` in the root
`Cargo.toml`. License: Apache-2.0 (see `LICENSE`).

**Why:** two things winit, under iced, receives from the system but
keeps to itself:

- the keyboard layout in use on Linux, which sets the side an empty text
  field starts on (X11 and Wayland tell every client which of the
  keymap's layouts is active);
- the files macOS asks the app to open. Finder, the Dock and `open`
  hand a Mac app its files through the application delegate's
  `application:openURLs:`, not as arguments, and winit owns the
  delegate;
- drags over a window on macOS, of which winit takes only file names;
- and on macOS it gets in the way of a menu bar (see the last two
  changes).

**Changes:**

- `src/platform_impl/linux/common/xkb/state.rs`: the XKB state keeps the
  keymap it was made from, and whenever the active layout changes (and
  when the state is made) it looks up what that layout types on the
  three letter rows.
- `src/platform/keyboard_layout.rs` (new): `letters()` returns those
  letters. prev tells the layout's direction from their script
  (`crates/prev/src/input.rs`), so it needs no list of layout names.
- `src/platform_impl/macos/app_state.rs`: the application delegate
  implements `application:openURLs:` and hands the file paths to
  `src/platform/open_files.rs` (new), which keeps them until prev sets a
  handler; `Cargo.toml` adds the `NSURL` feature it needs.
- `src/platform_impl/macos/window_delegate.rs`: the window's dragging
  destination methods (and `draggingUpdated:`, which it adds) hand the
  `NSWindow` and the `NSDraggingInfo` to a hook in
  `src/platform/drag_drop.rs` (new) when prev has set one, and return
  its drag operation; without a hook they behave as before. prev reads
  the drag's pasteboard with its own bindings (`crates/prev/src/dnd_macos.rs`).
- `src/platform/mod.rs`: the new modules.
- `src/platform_impl/macos/app_state.rs`: winit's default menu goes up
  only when the application has none, since iced starts prev, which puts
  up its menu bar, before winit's launch handler runs.
- `src/platform_impl/macos/view.rs`: the view answers `cut:`, `copy:`,
  `paste:` and `selectAll:` from the menu bar by handing the application
  the Command key press, the one that chose the item or, when it was
  chosen with the mouse, one made for it; without this, the menu's key
  equivalents would keep those keys from prev's text fields.
- `src/platform_impl/macos/window_delegate.rs` and `view.rs`: a new
  cursor is also set straight away when the pointer is over the view.
  winit sets it through cursor rects alone, which AppKit applies only
  when the mouse moves and not while a button is held, so the hand prev
  shows when ⌘ is pressed, or the closed hand when a pan starts, waited
  for the pointer to move.
- `src/platform_impl/hand_cursors.rs` (new) and `hand_cursors/`:
  pointing, open and closed hand cursors by Abdulkaiz Khatri (GPL-3.0,
  see the README there), which `CursorIcon::Pointer`, `CursorIcon::Grab`
  and `CursorIcon::Grabbing` use on Windows (`windows/util.rs`,
  `window.rs`, `event_loop.rs`), whose pointing hand is drawn unlike
  macOS's and which has no open or closed hand, falling back to the move
  cursor, and on Wayland and X11 (`linux/wayland/window/state.rs`,
  `linux/x11/window.rs`, `linux/x11/util/cursor.rs`), where they would
  come from the cursor theme; macOS keeps its own. Their size follows
  the system's cursor size and the window's scale.

The full diff is `winit.patch` (`diff -ruN` of `src/` and `Cargo.toml`
against the crates.io release), which leaves out the cursor images in
`src/platform_impl/hand_cursors/`, as they are not text.

**Updating:** when iced moves to a newer winit, re-apply the patch to that
version, or drop the copy if winit comes to expose both.
