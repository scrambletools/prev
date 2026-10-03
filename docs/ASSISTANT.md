# Assistant plan

prev's features, opened to language models: first to outside agents,
such as Claude Code, through the Model Context Protocol (MCP), then to an
assistant panel inside prev. Both use one set of tools; the use cases
below are what those tools must make possible.

Status: planning. Nothing here is built yet.

## Stages

1. **Outside control.** prev offers its tools as an MCP server. Any
   agent that speaks MCP drives it, with whatever model the agent uses,
   local or not; prev needs no model or account set up. This stage is
   done when every use case below works from an agent.
2. **Assistant panel.** A chat panel inside prev with a model you set up
   in Settings, using the same tools. It starts once stage 1 is
   complete.

## Use cases

The use cases are the test of the tools, not features with their own
interface: each must work through the tools alone, from an agent in
stage 1 and from the panel in stage 2. If one turns out to need more in
prev's own interface, that is added then.

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
    (the agent's image tool, or in stage 2 a provider with an image
    model) and goes on the page as an image annotation, as a pasted one
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

### Not planned for now

- Comparing two documents.
- Writing new documents or rewriting a document's text (prev does not
  edit page content).
- Working across many files at once.

## Tools

prev's actions are described once, as typed tools: a name, a JSON
schema for the inputs and a description the model reads. They live in
one registry in prev, which both stages use. Each runs the same code a
click does (`EditMessage` for markup, page actions, image edits), so
every change goes into the usual undo history and autosaves as any edit
does. A tool acts on the window it names, or the one in front.

- **Read**: `document_info` (pages, sizes in points,
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
  `set_window` (size, position, full screen), `close_window`.
- **Mark up**: `highlight`, `add_note`,
  `add_text_box`, `add_shape`, `edit_annotation`, `delete_annotation`,
  `add_redaction`, `open_file`, `focus_window`.
- **Edit**: `rotate_pages`, `delete_pages`, `move_pages`,
  `insert_blank_page`, `fill_field`, `place_image`, `crop`, `rotate`,
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
a tool of that kind runs:

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
off, the tool runs at once. The switches apply to outside agents and
the panel alike.

## Stage 1: outside control (MCP)

Stage 1 is built: every step below is done, and every use case was tried
from Claude Code on Linux and macOS and from a local model (qwen3.8
through ollama), with the tools also called directly on Windows. Where it
came out differently from this plan:

- The calls go over a control channel of their own, beside the
  single-instance socket, which stays for passing files.
- A tool whose kind asks first says so in its description, and a call
  held for the user's answer tells the agent so as MCP progress.
- Markup tools also work on an image's markup, in image pixels, opening
  the markup first; `place_signature`, `place_image` and `export` do too.
- Trying the use cases added `crop_pages`, `whole_words` for text
  matches, text and a duration for `point_at`, scaled image exports,
  colour names, and window sizes in `list_windows`.

The plan as written:

- prev's MCP server uses `rmcp`, the official Rust SDK.
- An agent starts `prev --mcp`, which speaks MCP over standard input and
  output and passes each call to the running prev over its
  single-instance socket (a named pipe on Windows), starting prev if it
  is not running. Nothing listens on the network.
- There is nothing to set up in prev beyond allowing it: the first time
  an agent connects, prev asks whether to let it in, and Settings can
  turn outside control off. What it may do without asking follows the
  approval switches.
- Setup on the agent's side is one line, such as
  `claude mcp add prev -- prev --mcp` for Claude Code; the guide gives
  it for Claude Code and for a local model's MCP client.
- Tests: the tools are tested against the sample PDFs and images, without
  a model, and a scripted MCP session tests the server end to end. Each
  use case is also tried by hand from Claude Code and from a local model.

### Steps

Each step ends with its tests passing and is usable on its own.

1. **A control channel.** Beside the single-instance socket, which
   stays as it is for passing files, the running prev opens a second
   one for tool calls (`control.sock`; a second named pipe on Windows),
   private to the user: a long-lived connection carrying one JSON
   request and reply per line. The app gets each call as a message, as
   it gets forwarded files now, and replies when the call is done.
2. **`prev --mcp`.** A mode of the prev binary that serves MCP over
   standard input and output with `rmcp` and relays every call over the
   control channel, starting prev and waiting for it when it is not
   running. It lists the tools from the registry. Settings gets
   **Allow outside control** (on), and the first time an agent connects
   prev asks whether to let it in. Done when Claude Code connects and
   lists the tools.
3. **The tool registry.** A `tools` module in prev: each tool has a
   name, its kind (read, view, mark up, edit, sign, redact or export), an
   input schema (from `schemars`), a description, and a function that
   runs it against a window and returns text, JSON or PNG. Tools that
   wait on the document thread or a render reply when it answers. A
   tool names its window, or acts on the one in front.
4. **Approval.** The seven switches in Settings, and the Allow and Don't
   Allow prompt in the window a call acts on, which holds the call until
   answered. A call refused, or for a window that closes, replies with
   an error the agent can read.
5. **Files, windows and reading.** `open_file`, `list_windows`,
   `focus_window`, `document_info`, `page_text` (lines with their
   positions in page points), `search`, `current_view`,
   `list_annotations`, `form_fields`, `list_signatures`, `render_page`,
   `render_image`, `get_image`, and the read tools in Markdown windows.
   Covers use cases 1–3, 13, 14, 19 and 20.
6. **The view.** `go_to_page`, `scroll_to`, `set_zoom`,
   `set_view_mode`, `point_at`, `show_panel`, `hide_panel`,
   `set_window` and `close_window`. On Wayland a window cannot place
   itself, so `set_window` sets only the size there and says so; it
   places windows on Windows, macOS and X11. Covers use case 21.
7. **Markup.** `highlight`, `add_note`, `add_text_box`, `add_shape`,
   `edit_annotation`, `delete_annotation` and `add_redaction`, through the same messages as the markup tools, so each
   is one Undo; each returns the ids of what it made. Covers use cases
   4–6 and 15.
8. **Edits.** Page edits (`rotate_pages`, `delete_pages`, `move_pages`,
   `insert_blank_page`), `fill_field`, `place_image`, the image edits
   (`crop`, `rotate`, `resize`, `adjust_color`) and `replace_image`.
   Covers use cases 7, 10, 11, 16 and 17.
9. **Signing, redacting and exporting.** `place_signature`,
   `apply_redactions` and `export`, behind their own switches. Covers
   use cases 8, 9, 12 and 18.
10. **Trying every use case.** Each use case from Claude Code and from a
   local model through an MCP client, on Linux, Windows and macOS;
   tool descriptions are tuned where a model misreads them.
11. **Docs.** A guide section on outside control, with the setup line
    for Claude Code and for a local model's client, the approval
    switches, and the README and specifications.

## Stage 2: assistant panel

Stage 2 is built: steps 1 to 4 and 6 are done, and step 5 in part. The
panel chatted with tool calls from a local model (qwen3.8 and qwen3-vl
through ollama) on Linux, Windows and macOS, from Claude on macOS and
from GPT on Windows; recorded replies test each provider's streaming
format in CI, including Gemini's, which no one has tried with a key yet.
Not every use case has been tried from the panel. Where it came out
differently from this plan:

- Settings finds the models to add rather than asking for their names:
  a running Ollama or compatible server and what each model can do, or
  a cloud provider's chat models once its key works. Adding a model
  tries it first. A cloud provider's key is kept once for all its
  models.
- Ollama models get a context size, 32K tokens unless set, as Ollama's
  own 4096 is too small for prev's tools.
- The panel's model menu, under the field, groups models by provider and
  changes the model mid-chat, keeping the conversation.
- Models that reason show their reasoning as it streams, folded away
  once they answer.
- The model is told which window and file the panel belongs to.
- A model's errors read as plain sentences: a context too small, a key
  turned down, no credit, a rate limit in the provider's words.
- The OpenAI-compatible provider uses Chat Completions, not the
  Responses API OpenAI's own models use.

The plan as written:

- A toolbar button opens an **Assistant** panel on the right, as the
  inspector does, in document and image windows: a plain chat with the
  model in use, which calls the same tools.
- Settings gets an **Assistant** section to add models: a provider
  (Anthropic, OpenAI, Gemini, a local Ollama, and so on), the model, and
  its key or sign-in where it needs one; and which model the panel uses.
- The model library is rig-core (rig.rs), behind a small interface in a
  new `prev-assist` crate, pinned to one version, so replacing it
  touches only that crate. It runs on its own thread with a tokio
  runtime, as iced has its own executor.
- API keys are kept in the system keychain (the `keyring` crate: Secret
  Service, Windows Credential Manager, macOS Keychain), never in
  `prev.toml`, which holds only the providers and models. Cloud models
  need an API key from the provider; Claude and ChatGPT subscriptions
  cannot be used by other apps.
- The model gets only what it asks for through the tools, never the
  whole file up front.

### Steps

Each step ends with its tests passing and is usable on its own.

1. **The `prev-assist` crate.** rig-core 0.43, pinned, behind a small
   interface: a model is a provider (Anthropic, OpenAI, Gemini, Ollama,
   or any OpenAI-compatible server by its address), a model name and its
   key or address. A chat runs on its own thread with a tokio runtime:
   prev sends it the user's message and the tool definitions, and gets
   back text as it streams, tool calls to run, and the end of the turn.
   rig's completion API takes the tools as plain JSON schemas, so the
   registry's tools go to it unchanged, and prev runs the tool loop
   itself. rig needs Rust 1.95, so prev's minimum Rust rises from 1.89.
2. **Keys and models in Settings.** An Assistant tab lists the models
   added, each with its provider; Add asks for the provider, the model
   and its key (or a local server's address), with a test that sends one
   short message; one model is the panel's. Keys go to the system
   keychain through the `keyring` crate, and `prev.toml` keeps the
   rest.
3. **Tools in process.** The panel's tool calls go through the same
   registry and approval switches as an outside agent's, without the
   control channel: a call made inside prev answers the chat directly.
   The panel's window is the one tools act on when a call names none.
4. **The panel.** A toolbar button opens it on the right, as the
   inspector opens, in document, image and Markdown windows: the chat so
   far, the model's replies as Markdown while they stream, each tool
   call as one short line (with its picture for render tools), a field
   to type in, Stop, and New Chat. A model that can see gets pictures
   from the render tools; one that cannot gets their text. The panel
   follows the window: each window has its own chat.
5. **Trying the use cases** from the panel with a local model and a
   cloud one, tuning the system prompt where it misleads them.
6. **Docs and translations.** The guide's Assistant section, the
   README, the specifications, and every language.
