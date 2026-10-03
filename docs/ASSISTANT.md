# The assistant and AI agents

prev's features are open to language models in two ways, with one set of
tools: outside agents, such as Claude Code, control prev through the
Model Context Protocol (MCP), and the assistant panel inside prev chats
with a model added in Settings. The [guide](https://prev.run/guide.html#assistant)
describes both for users; this document covers how they work.

## How it fits together

- **The tool registry** (`crates/prev/src/app/tools.rs` and
  `tools/`): each tool has a name, a kind (read, view, mark up, edit,
  sign, redact or export), an input schema made with `schemars`, a
  description the model reads, and a function that runs it against a
  window and answers with text, JSON or a PNG. Each runs the same code a
  click does, so every change is one step of Undo and saves by itself. A
  tool acts on the window it names, or the one in front.
- **Outside control.** An agent starts `prev --mcp`, which serves MCP
  over standard input and output with `rmcp` and relays each call to the
  running prev over a control channel of its own (`control.sock`, a
  named pipe on Windows), private to the user and beside the
  single-instance socket, starting prev when it is not running. Nothing
  listens on the network. The first time an agent connects, prev asks
  whether to let it in; Settings turns outside control off. A call held
  for the user's answer tells the agent so as MCP progress.
- **The assistant panel.** `crates/prev-assist` talks to the model
  through rig-core, pinned and known to that crate alone: Anthropic,
  OpenAI (its Responses API), Google Gemini, Ollama, and any
  OpenAI-compatible server (Chat Completions). A chat runs on its own
  thread with a tokio runtime; prev sends it what the user says and the
  tool definitions, and gets back the reply and the reasoning as they
  stream, and the tool calls to run, which go through the same registry
  and approvals as an agent's, one at a time, on the panel's window
  unless a call names another. The model is told which window and file
  the panel belongs to. Settings finds the models to add (a running
  server's, or a cloud provider's once its key works) and tries one
  before adding it. API keys are kept in the system keychain with the
  `keyring` crate, once per provider; `prev.toml` keeps the rest. Ollama
  models get a context size, 32K tokens unless set, as Ollama's own 4096
  is too small for prev's tools. A model that cannot see pictures gets
  the render tools' text without them.
- **Tests.** The tools are tested against sample PDFs and images without
  a model, and a scripted MCP session tests the server end to end.
  `prev-assist` plays back each provider's streaming format from a local
  server (`tests/recorded.rs`), so CI covers them without keys; a live
  test talks to a local Ollama, and one checks the keychain, both run by
  hand.

## Use cases

What the tools are for, and what they are tried against: each works
through the tools alone, from an outside agent and from the panel.

### Documents

1. **Ask about the document.** "When is the deadline in this contract?",
   "Summarize section 3." Tools: `document_info`, `page_text`,
   `search`, `go_to_page`.
2. **Ask about what is selected or on screen.** "What does this
   paragraph mean?", "Translate this." Tools: `current_view`,
   `page_text`.
3. **Explain a chart, table or figure.** "What does the chart on page 6
   show?" Needs a model that reads images. Tools: `render_page`.
4. **Find and highlight.** "Highlight every mention of the deposit."
   Highlights are placed from search matches, not guessed positions.
   Tools: `search`, `highlight`.
5. **Review and comment.** "Add a note wherever the dates disagree",
   "Mark anything that looks like a typo." Notes are anchored to the
   text they are about. Tools: `page_text`, `search`, `add_note`,
   `highlight`.
6. **Mark up beyond highlights.** "Circle the total on page 2", "Put a
   box around the second table", "Draw an arrow to the signature line."
   Shapes around text are placed from its position; around a figure or
   an area, from a picture of the page. Tools: `page_text`, `search`,
   `render_page`, `add_shape`, `add_text_box`.
7. **Fill in a form.** "Fill this application with: Jordan Example,
   12 Orchard Street..." Tools: `form_fields`, `fill_field`.
8. **Sign and date.** "Sign this with my signature and date it." Uses a
   signature already in the library, on the signature line or field,
   with the date beside it; a model never draws or makes up a
   signature. Tools: `list_signatures`, `form_fields`, `search`,
   `place_signature`, `add_text_box`.
9. **Redact personal information.** "Redact names, phone numbers and
   email addresses." Places redaction marks; applying them is separate.
   Tools: `search`, `page_text`, `add_redaction`, `apply_redactions`.
10. **Reorganize pages.** "Move the appendix to the end", "Delete the
    blank pages", "Rotate the scanned pages upright." Tools:
    `page_text`, `render_page`, `move_pages`, `delete_pages`,
    `rotate_pages`, `insert_blank_page`.
11. **Generate and place an image.** "Make a small logo of a pomegranate
    and put it in the top corner." The image comes from the model's side
    (the agent's image tool) and goes on the page as an image annotation, as a pasted one
    does. Tools: `place_image`.
12. **Save an export.** "Export pages 2–5 as a PDF", "Save a flattened,
    smaller copy." Uses prev's export and its options. Tools: `export`.

### Images

13. **Describe an image.** "What is in this photo?", "Write alt text for
    it." Tools: `render_image`.
14. **Read text in an image.** "What does this screenshot say?" Tools:
    `render_image`.
15. **Mark up an image.** "Circle the person on the left", "Point an
    arrow at the error message." Shapes are placed from what the model
    sees, the least precise part. Tools: `render_image`, `add_shape`,
    `add_text_box`.
16. **Edit an image.** "Crop to the cat", "Make it brighter and
    straighten it." Tools: `render_image`, `crop`, `rotate`, `resize`,
    `adjust_color`.
17. **Remove the background.** "Cut out the product from the
    background." prev bundles no models, so the model's side does the
    work: it takes the image at full size, changes it with its own image
    model, and hands back the result, which replaces the image as a new
    version, so undo and Revert To bring back the original. The same
    goes for any edit an image model makes. Tools: `get_image`,
    `replace_image`.
18. **Export an image.** "Save it as a PNG half the size." Tools:
    `export`.

### Markdown

19. **Ask about a Markdown file.** "What does this README say about
    installing?" Markdown windows get the read tools only. Tools:
    `document_info`, `page_text` (the file's text), `search`,
    `current_view`.

### Files and windows

20. **Open, find and switch.** "Open report.pdf", "Which documents are
    open?" Tools: `open_file`, `list_windows`, `focus_window`.

### Working side by side

21. **Show me as you go.** With you and an agent looking at the same
    window, the agent moves prev to what it is talking about: "The
    totals are on page 4" turns to page 4 and zooms to the table, "see
    the chart here" points at it for a moment, and it opens the
    contents, the thumbnails or the inspector when it refers to them,
    and closes them again. It can size and place windows, such as two
    documents side by side. Tools: the view tools.

### Not supported

- Comparing two documents.
- Writing new documents or rewriting a document's text (prev does not
  edit page content).
- Working across many files at once.

## Tools

- **Read**: `open_file`, `document_info` (pages, sizes in points,
  metadata, outline), `page_text` (text with line positions), `search`,
  `current_view` (page shown, selection), `list_annotations`,
  `form_fields`, `list_signatures`, `render_page` and `render_image` (a
  picture, for models that read images), `get_image` (the image itself,
  at full size), `list_windows`.
- **View** (changes nothing in the file): `go_to_page`, `scroll_to`
  (a point on a page, at a zoom), `set_zoom` (a percentage, fit page,
  fit width, actual size), `set_view_mode` (continuous, single page, two
  pages), `point_at` (outlines an area for a few seconds without
  marking up), `show_panel` and `hide_panel` (sidebar and its tab,
  inspector, markup bar, image adjustments, search with a query),
  `set_window` (size, position, full screen), `focus_window`,
  `close_window`.
- **Mark up**: `highlight`, `add_note`,
  `add_text_box`, `add_shape`, `edit_annotation`, `delete_annotation`,
  `add_redaction`. Markup tools also work on an image's markup, in image
  pixels, opening it first.
- **Edit**: `rotate_pages`, `delete_pages`, `move_pages`,
  `insert_blank_page`, `crop_pages`, `fill_field`, `place_image`, `crop`, `rotate`,
  `resize`, `adjust_color`, `replace_image` (the image, changed by the
  model's side, as a new version).
- **Sign**: `place_signature`.
- **Redact**: `apply_redactions`.
- **Export**: `export`.

Positions are in page points, with the page size given; anything tied
to text is placed from its position in `page_text` or `search`. Pictures
and images go both ways as PNG.

prev bundles no models. Anything a model makes, such as a generated
image or an image with its background removed, is made on the model's
side and handed to prev through the tools (`place_image`,
`replace_image`).

### Approval

Settings has a switch for each kind of tool, which makes prev ask before
a tool of that kind runs, for outside agents and the panel alike:

- **Ask before reading**, **Ask before changing the view**, **Ask
  before marking up**, **Ask before editing** and **Ask before
  exporting** (`export`): off at first.
- **Ask before signing** (`place_signature`) and **Ask before applying
  redactions** (`apply_redactions`): on at first.

Signing, redacting and exporting are left out of editing, as prev's
Undo cannot take them back: redactions remove the content under them and
the file's version history, a signature commits you, and an export
writes a file, possibly over another.

When a switch is on, prev shows what the tool is about to do in the
window it acts on, with Allow and Don't Allow, before it runs. With it
off, the tool runs at once. A tool whose kind asks says so in its
description, so the model can tell the user to look at prev.
