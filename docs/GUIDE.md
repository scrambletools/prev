# A tour of prev

This guide walks through what prev does, in the order you are likely to
meet it. Keyboard shortcuts are listed in the [README](../README.md#keyboard-shortcuts).

<sub>Documents shown: NIST SP 800-63-3, a US government publication in the
public domain, and a sample form, sample pages and a landscape image made
for prev.</sub>

## Opening files

Open files with Ctrl+O, from the file manager's "Open With", from the
command line (`prev report.pdf photo.jpg`), or by dropping them on a prev
window. Each document gets its own window; images opened together share
one window with a thumbnail strip. prev keeps a single running instance,
so opening another file from the launcher adds a window instead of
starting over.

## Reading PDFs

![A PDF with its table of contents in the sidebar](screenshots/table-of-contents.png)

The toolbar holds the page number, zoom, fit to page or width, and the
view mode: continuous scroll, single page or two pages side by side.
Ctrl+scroll zooms around the pointer, and pages stay sharp at any zoom
because they render in tiles on background threads.

The sidebar (the button at the far left, or Ctrl+Alt+2 to 5) has four
tabs: page thumbnails, the table of contents, highlights and notes, and
bookmarks (Ctrl+D bookmarks the current page). Drag the sidebar's edge to
make it wider; the thumbnails grow with it.

![Two pages side by side](screenshots/two-pages.png)

In two-page mode pages sit in pairs, 1 and 2, 3 and 4, and the arrow keys
move a pair at a time.

![Searching a PDF, with matches highlighted and page thumbnails in the sidebar](screenshots/search.png)

Search (Ctrl+F) highlights every match and counts them as it goes;
Ctrl+G and Ctrl+Shift+G step through them. Text can be selected and
copied across pages; double-click selects a word and triple-click a line.
Links inside the document and to the web work, and Ctrl+Shift+F starts a
full-screen slideshow.

## Marking up

![A form filled in, marked up with a highlight, shapes, a text box and a note, and signed](screenshots/markup.png)

The markup toolbar (the pen button on the right, or Ctrl+Shift+A) follows
Preview's:

- **Select** moves, resizes and restyles annotations; Delete removes the
  selected one.
- **Rectangular selection** chooses an area to copy as an image, or to
  crop the page to (Ctrl+K).
- **Sketch** turns strokes that look like a rectangle, oval or line into
  that shape; **Draw** keeps them as drawn.
- **Shapes**: rectangle, rounded rectangle, oval, line, arrow, star,
  polygon, speech bubble, a loupe that magnifies the page under it, and a
  mask that dims everything but a hole.
- **Text box** and **Note**, with fonts, sizes, colors and alignment.
- **Highlight**, underline and strikethrough over selected text, in five
  colors.
- **Sign** and **Redact**, described below.
- Border color, fill color, line width and dashes for shapes.

Everything is saved as standard PDF annotations that other viewers show
and edit. Undo and redo (Ctrl+Z, Ctrl+Shift+Z) cover every change.

### Forms

Click into text fields and type; checkboxes, radio buttons and drop-down
menus work as they do in other viewers. Values are saved into the form so
other apps read them.

### Signatures

![The signature library with a drawn and a typed signature](screenshots/signatures.png)

The Sign menu keeps as many signatures as you like. Click one to place it
on the current page, then drag and resize it like any annotation.

![Drawing a signature in blue ink](screenshots/signature-draw.png)

Create Signature offers three ways in:

- **Draw** with the mouse, a pen or the touchpad. Choose the ink color
  and the pen's thickness under the pad; changing them redraws what you
  signed.
- **Type** your name, shown in a handwriting font in the ink you choose.
- **Image** takes a photo or scan of a signature on paper and makes the
  paper transparent.

## Editing pages

![Dragging a page to a new place in the page thumbnails](screenshots/pages.png)

In the page thumbnails, click a page to select it, Ctrl+click to add
pages and Shift+click to select a range. Then:

- Drag the selection to reorder; a line shows where it will go.
- Rotate with the toolbar buttons (Ctrl+L, Ctrl+R).
- Delete with the Delete key.
- Copy with Ctrl+C and paste with Ctrl+V after the selected page, in the
  same window or another document's. Pasted pages keep their annotations
  and form fields.
- Drop PDF files on the thumbnails to insert their pages.

The Pages menu (the stacked-pages button) has the same actions plus
Insert Blank Page, Insert from File, Crop to Selection and Select All
Pages. Every page edit can be undone.

## Redacting

![An account number and a routing number marked for redaction](screenshots/redaction.png)

The Redact tool marks what should go: drag a box over it, or select text
first and then pick the tool. Marks are ordinary annotations until you
press **Apply** and confirm. Applying removes the text, image pixels and
drawings under each mark from the file and draws black boxes in their
place. The next save rewrites the whole file, so no earlier revision
keeps the content, and prev deletes the earlier versions of the file it
kept. This cannot be undone.

## Exporting

![The export dialog with reduce file size and flatten chosen](screenshots/export.png)

Export (Ctrl+Shift+S, or Export in the Pages menu) writes a new file:

- **PDF**, optionally:
  - **Reduce file size**: images are downsampled to 150 dpi and saved as
    JPEG.
  - **Flatten annotations and form fields**: markup and filled-in fields
    are drawn into the pages and can no longer be edited. Redaction
    marks not yet applied are left out.
  - **Encrypt with a password** (AES-256): the password is needed to open
    the file.
- **PNG, JPEG, TIFF, WebP or OpenEXR** at 72 to 600 dpi, with a quality
  choice for JPEG. TIFF puts every page in one file; the others save a
  file per page, numbered after the name you choose.

Tick "Only the selected pages" to export just the pages chosen in the
sidebar.

## Images

![A photo open with the Adjust Color panel](screenshots/image-editing.png)

prev opens PNG, JPEG, GIF (animated too), WebP, AVIF, HEIC, TIFF, BMP,
ICO, TGA, PNM, QOI, JPEG 2000, OpenEXR, Radiance HDR and camera RAW. The
toolbar rotates, flips, crops and resizes (Adjust Size). Adjust Color
(Ctrl+Shift+C) has exposure, contrast, saturation, temperature, tint,
sepia, sharpness and levels. The inspector (Ctrl+I) shows camera details
and can remove location info and edit keywords and a description. Export
writes PNG, JPEG, WebP, TIFF, BMP, TGA, QOI, PPM or OpenEXR.

## SVG and Markdown

SVG drawings open sharp at any zoom. Markdown files show with tables,
task lists, syntax highlighted code and images, and reload when the file
changes on disk.

## Saving and versions

There is no Save command: edits to PDFs and images are written in place
a moment after you stop, and when a window closes. Before the first
write, prev keeps the original as a version in the version history
folder (`~/.local/share/prev/versions` unless the settings say
otherwise). For images, the inspector's Revert To
list puts an earlier version back; Revert To for PDFs is still to come.

## Settings

![The Settings dialog](screenshots/settings.png)

Settings opens over the current window, from the gear button at the end
of the toolbar or Ctrl+,. Close it with its close button, Escape or a
click outside it. Changes apply to every window at once:

- **Appearance**: follow the system's light or dark setting, or choose
  one.
- **Use Omarchy accent color**: on Omarchy, colors come from the active
  theme's accent; off, prev uses its own blue.
- **Hide the toolbar when the pointer leaves**: see below.
- **Animations**: off, bars, panels and dialogs appear at once. prev also
  stops moving things when the system asks for reduced motion.
- **Corner radius**: rounds dialogs and the floating toolbar, from
  square corners at 0 to a full pill at 32; the default, 28, is
  Material 3's.
- **Storage**: where prev keeps signatures, version history and
  bookmarks.

### The floating toolbar

![The toolbar and markup bar floating over the document](screenshots/floating-toolbar.png)

With "Hide the toolbar when the pointer leaves" on (or the top-panel
button at the end of the toolbar), the toolbar floats over the document
as a rounded bar, and the markup bar floats along the bottom. They slide
in while the pointer is over the window and go away when it leaves, so
the whole window shows the page. Sidebars and panels stay clear of them.

### Where files go

Settings are kept in `~/.config/prev.toml`, which lists every setting,
including where prev keeps your files:

```toml
signatures = "~/.local/share/prev/signatures"
versions = "~/.local/share/prev/versions"
bookmarks = "~/.local/share/prev/bookmarks.toml"
```

Change them in the Storage section: type a path (`~` works) and press
Enter or Apply, or pick a folder with Choose. prev checks that the folder
exists and can be written, uses the new place at once and saves it to
the file. Files already at the old place stay there; move them over to
keep using them. The paths are filled in when the file is first made and
then stay as they are.

## Looks

prev follows the system's light or dark setting and reduced motion
setting, and on Omarchy the active theme's accent. In narrow windows,
toolbar groups that don't fit move into a More menu.
