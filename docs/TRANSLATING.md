# Translating prev

Every word prev shows is in one place: `i18n/<language>/prev.ftl`, one file
per language, in [Project Fluent](https://projectfluent.org)'s format.
`i18n/en/prev.ftl` is the source; other languages translate its messages.

## Adding a language

1. Make a folder named for the language's tag, such as `i18n/he` for
   Hebrew or `i18n/pt-BR` for Brazilian Portuguese.
2. Copy `i18n/en/prev.ftl` into it and translate the text after each `=`.
   Keep the keys (before `=`) as they are. `language-name` is the
   language's name in itself (such as Deutsch or עברית); the Settings
   language list shows it.
3. Try it: choose the language in Settings, or run
   `PREV_LANG=he cargo run -- some.pdf`, which wins over Settings. By
   default prev follows the system's language.
4. Run `cargo test -p prev i18n`, which checks that the file loads and
   matches the English one.

A language can be translated a part at a time: messages it does not have
show in English.

## The format

```
# A comment for translators, about the message below it.
pdf-page-of = of { $count }

pages-copied = { $count ->
    [one] Copied { $count } page.
   *[other] Copied { $count } pages.
}
```

- `{ $name }` is a value prev fills in, such as a page number or a file
  name. Keep it, and move it wherever the language needs it.
- `{ $count -> … }` chooses a variant by number. Each language uses its
  own plural categories (`zero`, `one`, `two`, `few`, `many`, `other`);
  add the ones your language needs, and mark the default with `*`.
- Multi-line messages continue on indented lines.

The [Fluent syntax guide](https://projectfluent.org/fluent/guide/) has the
rest.

## For developers

- Show text with `crate::fl!("key")`, or `prev::fl!("key")` in the app's
  binary, and pass values as `fl!("key", count = pages)`. The macro fails
  the build when the key is missing from the English file.
- Add new keys to `i18n/en/prev.ftl`, in the section for that part of the
  interface, with a `#` comment when a translator needs context.
- Never join translated pieces into a sentence; give the whole sentence
  one message with variables.
- Text that goes into files (PDF annotation subjects, form values, file
  names on disk) stays in English, as do log messages.
- prev's libraries (`prev-pdf`, `prev-image`, `prev-store`) have no text
  of their own for the interface: they return typed errors and data, and
  the app words them. Show a library error with
  `i18n::Describe::describe()`, not `to_string()`, which stays English
  for logs; show an image format with `i18n::format_name()`.
- Details that come from MuPDF or the operating system inside an error
  stay as those give them.
- `i18n::right_to_left()` says whether the language is written right to
  left. In right to left languages the insides of panels, dialogs and
  menus mirror when built with `ui::dir`'s `row!`, `column!` and `line!`.
  The window's bars and the sides its panels sit on do not: build a bar
  while holding `ui::dir::fixed()`, and a menu inside one while holding
  `ui::dir::reading()`.
- Text fields and search boxes align with `ui::dir::input_align(value)`:
  by their text's own direction, or while empty by the input language,
  which follows the keyboard layout (`input.rs`) unless Settings chooses
  one. Their labels follow the interface.
- Say "right to left" rather than naming a language when code or a
  comment means every right to left script.
