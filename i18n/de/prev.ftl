# prev's interface text in English, the source for every other language.
#
# Each line is `key = text`. Keys stay the same in every language; only the
# text after `=` is translated. `{ $name }` is a value prev fills in, such
# as a page number or a file name; keep it, and move it where the
# language needs it. Plural and other variants use Fluent's selectors:
# https://projectfluent.org/fluent/guide/selectors.html
#
# Sections follow the parts of the interface. Keys shared by several
# parts are under "Common".

## Language

# This language's name in itself, as the Settings language list shows it,
# such as English, Deutsch or עברית.
language-name = Deutsch

## Common

common-cancel = Abbrechen
common-close = Schließen
common-save = Speichern

## Settings

settings-title = Einstellungen
settings-appearance = Erscheinungsbild
settings-colors = Farben
settings-windows = Fenster
settings-default-app = Standard-App
settings-default-app-label = Dateien mit prev öffnen
settings-default-app-note = prev zur App machen, die PDFs, Bilder, SVG-Zeichnungen und Markdown-Dateien öffnet.
settings-default-app-note-windows = Windows lässt Standard-Apps nur in seinen eigenen Einstellungen wählen. Dies öffnet dort die Seite von prev.
settings-default-app-note-macos = macOS fragt bei jedem Typ nach: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP und AVIF.
settings-default-app-status = { $set } von { $total } Dateitypen werden mit prev geöffnet.
settings-default-app-button = Als Standard festlegen
settings-default-app-button-windows = Einstellungen öffnen
settings-default-app-no-entry = Der Desktop-Eintrag von prev ist nicht installiert, daher kann das System keine Dateien damit öffnen. Installieren Sie prev aus einem Paket oder mit scripts/install.sh.
settings-default-app-no-bundle = Öffnen Sie prev aus prev.app, um es als Standard festzulegen.
settings-default-app-failed = prev konnte nicht als Standard festgelegt werden: { $error }
settings-storage = Speicherort
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (Entwicklungsversion, { $build })

## Markup toolbar

markup-tool-select = Auswählen
markup-tool-area = Rechteckige Auswahl
markup-tool-sketch = Skizzieren
markup-tool-draw = Zeichnen
markup-tool-shapes = Formen
markup-tool-text-box = Textfeld
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Hervorheben
markup-tool-note = Notiz
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Signieren
# A verb: the tool that marks areas to black out.
markup-tool-redact = Schwärzen
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Anwenden
markup-apply-redactions = Schwärzungen anwenden
markup-shape-style = Formstil
markup-border-color = Rahmenfarbe
markup-fill-color = Füllfarbe
markup-text-style = Textstil
markup-delete = Löschen
markup-undo = Rückgängig
markup-redo = Wiederholen

## Markup menus

markup-shape-rectangle = Rechteck
markup-shape-rounded-rectangle = Abgerundetes Rechteck
markup-shape-oval = Oval
markup-shape-line = Linie
markup-shape-arrow = Pfeil
markup-shape-star = Stern
markup-shape-polygon = Polygon
markup-shape-speech-bubble = Sprechblase
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Lupe
# A shape that darkens the page around it.
markup-shape-mask = Maske
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Hervorheben
markup-style-underline = Unterstreichen
markup-style-strikethrough = Durchstreichen
markup-style-squiggly = Wellenlinie
# Menu section headings.
markup-menu-color = Farbe
markup-menu-font = Schrift
markup-menu-size = Größe
markup-menu-alignment = Ausrichtung
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Gestrichelt

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Notiz
markup-kind-text-box = Textfeld
markup-kind-stamp = Stempel
markup-kind-redaction = Schwärzung
markup-kind-shape = Form
# Tooltips on a note being edited.
markup-note-delete = Notiz löschen
markup-note-done = Fertig
markup-note-placeholder = Notiz eingeben
markup-notes-empty = Keine Hervorhebungen oder Notizen
markup-notes-empty-hint = Hervorhebungen, Notizen und Textfelder erscheinen hier.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Seite { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Das Dokument konnte nicht geändert werden: { $error }
markup-copy-area-failed = Der Bereich konnte nicht kopiert werden: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = das Dokument wurde geschlossen
markup-render-area-failed = der Bereich konnte nicht dargestellt werden
markup-copy-stopped = das Kopieren wurde abgebrochen

## Signatures

signature-menu-empty = Noch keine Signaturen.
signature-delete = Signatur löschen
signature-create = Signatur erstellen …
signature-dialog-title = Signatur erstellen
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Zeichnen
signature-tab-type = Eingeben
signature-tab-image = Bild
signature-draw-hint = Unterschreiben Sie mit Maus, Stift oder Touchpad auf der Linie.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Ihr Name
signature-image-hint = Wählen Sie ein Foto oder einen Scan Ihrer Unterschrift auf weißem Papier.
signature-choose-image = Bild auswählen …
# Placeholder of the field naming the signature in the library.
signature-description = Beschreibung, z. B. „Vollständiger Name“ oder „Initialen“
# Clears the drawing, typed name or image.
signature-clear = Löschen
# The color the signature is drawn or typed in.
signature-ink = Tinte
# The pen's width, for drawing.
signature-thickness = Strichstärke
signature-sign-first = Erst unterschreiben, dann speichern.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Signatur { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Signaturen konnten nicht geändert werden: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = kein Datenordner: HOME ist nicht gesetzt
signature-removing-stopped = das Entfernen wurde abgebrochen
signature-saving-stopped = das Speichern wurde abgebrochen
signature-reading-stopped = das Lesen wurde abgebrochen
signature-not-an-image = diese Datei ist kein Bild, das prev lesen kann
signature-no-frames = das Bild hat keine Frames
signature-not-found = im Bild wurde keine Signatur gefunden

## Dragging

drag-pages-need-document = Seiten können auf einem Dokument abgelegt werden.
drag-image-unsupported = prev kann dieses Bild nicht öffnen.
# $error is a lowercase reason or a technical message.
drag-area-failed = Der Bereich konnte nicht gezogen werden: { $error }
drag-pages-failed = Die Seiten konnten nicht gezogen werden: { $error }
drag-start-failed = Das Ziehen konnte nicht gestartet werden.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Seiten
drag-file-one-page = { $name } (Seite { $page })
drag-file-page-range = { $name } (Seiten { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = Bild
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = Zu diesem Dokument hinzufügen?
drop-pdf-body = „{ $name }“ am Ende dieses Dokuments anfügen oder in einem eigenen Fenster öffnen?
drop-pdfs-body = { $count ->
    [one] Diese PDF am Ende dieses Dokuments anfügen oder in einem eigenen Fenster öffnen?
   *[other] Diese { $count } PDFs am Ende dieses Dokuments anfügen oder in eigenen Fenstern öffnen?
}
drop-pdf-add = Am Ende anfügen
drop-pdf-open = Separat öffnen

## PDF window

pdf-opening = Wird geöffnet …
pdf-open-failed = prev kann dieses Dokument nicht öffnen
pdf-no-pages = Das Dokument hat keine Seiten.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = das Dokument wurde geschlossen
pdf-keep-original-failed = die Originalversion konnte nicht behalten werden: { $error }
pdf-save-failed = Speichern fehlgeschlagen: { $error }
pdf-nothing-to-paste = Es gibt nichts zum Einfügen.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = das Einfügen wurde abgebrochen
pdf-file-dialog-failed = Der Dateidialog konnte nicht angezeigt werden: { $error }
pdf-bookmarks-no-home = Lesezeichen können nicht gespeichert werden: HOME ist nicht gesetzt
pdf-bookmarks-save-failed = Lesezeichen konnten nicht gespeichert werden: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Seite { $page }

# Password prompt. $name is the file name.
pdf-password-protected = „{ $name }“ ist durch ein Passwort geschützt
pdf-password = Passwort
pdf-password-wrong = Falsches Passwort. Versuchen Sie es erneut.
# Button that opens a locked document.
pdf-unlock = Entsperren

# Toolbar tooltips and labels.
pdf-sidebar = Seitenleiste
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = von { $count }
pdf-zoom-out = Verkleinern
pdf-zoom-in = Vergrößern
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent } %
pdf-fit-page = Ganze Seite
pdf-fit-width = Seitenbreite
pdf-actual-size = Originalgröße
pdf-view-continuous = Fortlaufend scrollen
pdf-view-single-page = Einzelseite
pdf-view-two-pages = Zwei Seiten
pdf-undo = Rückgängig
pdf-redo = Wiederholen
pdf-rotate-left = Nach links drehen
pdf-rotate-right = Nach rechts drehen
pdf-inspector = Informationen
pdf-markup = Markierungen
# Tooltip of the button that opens the export dialog.
pdf-export = Exportieren
pdf-settings = Einstellungen

# Search field.
pdf-search = Suchen
pdf-search-not-found = Nicht gefunden
pdf-searching = Suche läuft …
# The match shown, of all matches found.
pdf-search-match = { $current } von { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } von { $total }+

# Inspector: section headings.
pdf-inspector-file = Datei
pdf-inspector-document = Dokument
pdf-inspector-pages = Seiten
# Inspector: fact labels and values.
pdf-inspector-title = Titel
pdf-inspector-author = Autor
pdf-inspector-subject = Thema
pdf-inspector-keywords = Schlagwörter
pdf-inspector-created = Erstellt
pdf-inspector-modified = Geändert
pdf-inspector-application = Programm
pdf-inspector-producer = PDF-Erzeuger
pdf-inspector-version = Version
pdf-inspector-security = Sicherheit
pdf-inspector-not-encrypted = Nicht verschlüsselt
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Verschlüsselt ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } Seite
   *[other] { $count } Seiten
}
pdf-inspector-page-size = Seitengröße
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } Zoll)
pdf-loading = Wird geladen …

# Sidebar tabs and lists.
pdf-tab-pages = Seiten
pdf-tab-contents = Inhalt
pdf-tab-notes = Hervorhebungen und Notizen
pdf-tab-bookmarks = Lesezeichen
pdf-no-outline = Kein Inhaltsverzeichnis
pdf-no-outline-detail = Dieses Dokument hat keine Gliederung.
pdf-no-bookmarks = Keine Lesezeichen
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Drücken Sie { $keys }, um ein Lesezeichen für eine Seite zu setzen.
pdf-no-bookmarks-detail-unbound = Seiten mit Lesezeichen erscheinen hier.
pdf-remove-bookmark = Lesezeichen entfernen

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Seiten
pages-insert-blank = Leere Seite einfügen
pages-insert-file = Aus Datei einfügen …
pages-copy = { $count ->
    [one] Seite kopieren
   *[other] Seiten kopieren
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Seite einfügen
   *[other] { $count } Seiten einfügen
}
pages-crop = Auf Auswahl beschneiden
pages-select-all = Alle Seiten auswählen
pages-delete = { $count ->
    [one] Seite löschen
   *[other] Seiten löschen
}
pages-apply-redactions = Schwärzungen anwenden …
pages-no-copied = Es gibt keine kopierten Seiten zum Einfügen.
pages-copied = { $count ->
    [one] { $count } Seite kopiert.
   *[other] { $count } Seiten kopiert.
}
pages-copy-failed = Die Seiten konnten nicht kopiert werden: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = das Lesen wurde abgebrochen
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = kein Bild, das prev lesen kann
pages-read-failed = Die Datei konnte nicht gelesen werden: { $error }
pages-at-least-one = Ein Dokument muss mindestens eine Seite enthalten.
pages-crop-needs-area = Wählen Sie zuerst mit dem Werkzeug „Rechteckige Auswahl“ einen Bereich aus.
pages-change-failed = Die Seiten konnten nicht geändert werden: { $error }
pages-no-redactions = Es gab keine Schwärzungen zum Anwenden.
pages-redactions-applied = { $count ->
    [one] { $count } Schwärzung angewendet.
   *[other] { $count } Schwärzungen angewendet.
}
pages-forget-versions-failed = Frühere Versionen konnten nicht gelöscht werden: { $error }
pages-redact-title = Schwärzungen anwenden?
pages-redact-body = { $count ->
    [one] Text, Bilder und Zeichnungen unter der Markierung werden endgültig aus dem Dokument entfernt, und die Markierung wird zu einem schwarzen Kasten. Dies kann nicht rückgängig gemacht werden, und die früheren Versionen dieser Datei, die prev aufbewahrt, werden gelöscht.
   *[other] Text, Bilder und Zeichnungen unter den { $count } Markierungen werden endgültig aus dem Dokument entfernt, und die Markierungen werden zu schwarzen Kästen. Dies kann nicht rückgängig gemacht werden, und die früheren Versionen dieser Datei, die prev aufbewahrt, werden gelöscht.
}
# Button that applies redactions.
pages-redact-apply = Anwenden

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Exportieren
pages-export-format = Format
pages-export-reduce = Dateigröße reduzieren (Bilder mit 150 dpi)
pages-export-flatten = Anmerkungen und Formularfelder reduzieren
pages-export-flatten-detail = Markierungen und ausgefüllte Felder werden Teil der Seiten und lassen sich nicht mehr bearbeiten. Noch nicht angewendete Schwärzungen werden weggelassen.
pages-export-encrypt = Mit Passwort verschlüsseln
pages-export-password = Passwort
pages-export-verify-password = Passwort bestätigen
pages-export-resolution = Auflösung
pages-export-dpi = { $dpi } dpi
pages-export-quality = Qualität
# JPEG quality choices.
pages-export-quality-low = Niedrig
pages-export-quality-medium = Mittel
pages-export-quality-high = Hoch
pages-export-quality-best = Optimal
pages-export-one-file = Alle Seiten werden in einer Datei gespeichert.
pages-export-file-per-page = Jede Seite wird als eigene Datei gespeichert, benannt nach dem gewählten Namen mit fortlaufender Nummer.
pages-export-selected-only = { $count ->
    [one] Nur die ausgewählte Seite
   *[other] Nur die { $count } ausgewählten Seiten
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Exportieren …
pages-export-no-password = Geben Sie ein Passwort ein.
pages-export-password-mismatch = Die Passwörter stimmen nicht überein.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (exportiert)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = Dokument
pages-export-same-file = Exportieren Sie in eine neue Datei – dieses Dokument wird automatisch gespeichert.
pages-export-exporting = „{ $name }“ wird exportiert …
pages-export-done = „{ $name }“ exportiert.
pages-export-done-images = { $count } Bilder exportiert.
pages-export-failed = Exportieren fehlgeschlagen: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = das Exportieren wurde abgebrochen

## Start window

# Under the app name in a window with no file open.
app-start-hint = Öffnen Sie eine PDF-, Bild-, SVG- oder Markdown-Datei oder legen Sie sie hier ab.
app-start-open = Öffnen …
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (dev)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: Diese Ansicht gibt es noch nicht.
app-cannot-open = prev kann diese Art von Datei nicht öffnen.
app-cannot-read = prev kann diese Datei nicht lesen: { $error }
app-kind-pdf = PDF-Dokument
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format }-Bild
app-kind-svg = SVG-Zeichnung
app-kind-markdown = Markdown-Dokument
app-file-dialog-failed = Der Dateidialog konnte nicht angezeigt werden: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Öffnen
action-settings = Einstellungen

## Toolbar

app-toolbar-keep-shown = Symbolleiste immer anzeigen
app-toolbar-auto-hide = Symbolleiste ausblenden, wenn der Zeiger das Fenster verlässt
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Mehr

## File facts

# Labels in a file's inspector.
app-fact-name = Name
app-fact-folder = Ordner
app-fact-size = Größe
app-fact-modified = Geändert
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } Byte
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Ungültiger Link { $uri }: { $error }
app-link-open-failed = { $uri } konnte nicht geöffnet werden: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = installieren Sie wl-clipboard, um Bilder einzufügen
app-copy-needs-wl-clipboard = installieren Sie wl-clipboard, um Bilder zu kopieren
app-copy-no-pixels = der Bereich enthält keine Pixel
# wl-copy is a program's name.
app-copy-no-input = wl-copy hat keine Eingabe erhalten
app-copy-failed = wl-copy ist fehlgeschlagen
app-clipboard-open-failed = Die Zwischenablage konnte nicht geöffnet werden: { $error }
app-copy-image-failed = Das Bild konnte nicht kopiert werden: { $error }

## Printing

print-failed = Drucken fehlgeschlagen: { $error }
print-stopped = Drucken abgebrochen
print-unavailable = Drucken ist auf diesem System noch nicht verfügbar.
print-no-window = Drucken fehlgeschlagen: kein Fenster für den Druckdialog
print-dialog-failed = Der Druckdialog konnte nicht angezeigt werden: { $error }
# Shown after "Could not print:".
print-job-not-started = der Drucker hat den Auftrag nicht gestartet
# Shown after "Could not print:".
print-printer-stopped = der Drucker wurde angehalten

## File dialogs

dialog-open = Öffnen
dialog-filter-all = Alle unterstützten Dateien
dialog-filter-pdf = PDF-Dokumente
dialog-filter-images = Bilder
dialog-filter-svg = SVG-Zeichnungen
dialog-filter-markdown = Markdown
dialog-choose-signatures = Ordner für Signaturen auswählen
dialog-choose-versions = Ordner für den Versionsverlauf auswählen
dialog-choose-bookmarks = Lesezeichendatei auswählen

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Aufruf: prev [FILE]...
            prev --mcp

    PDFs und Bilder ansehen und bearbeiten. Dateien werden in Fenstern des
    laufenden prev geöffnet; läuft prev noch nicht, wird es gestartet.

    Optionen:
      -h, --help     Diese Hilfe anzeigen
      -V, --version  Die Version anzeigen
          --mcp      MCP über stdin und stdout bereitstellen, damit KI-Agenten das
                     laufende prev steuern können

## Settings, continued

settings-language = Sprache
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Systemstandard: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Eingabesprache
settings-input-language-system = Tastaturbelegung verwenden
settings-input-language-note = Legt fest, auf welcher Seite ein leeres Textfeld beginnt. Eingegebener Text behält seine eigene Richtung.

settings-appearance-system = System
settings-appearance-light = Hell
settings-appearance-dark = Dunkel
settings-system-accent = Akzentfarbe des Systems verwenden
# $theme is the Omarchy theme's name.
settings-omarchy-note = Die Farben werden aus der Akzentfarbe von „{ $theme }“ abgeleitet.
settings-system-accent-note = Die Farben werden aus der Akzentfarbe des Systems abgeleitet.
settings-system-accent-none = Das System hat keine Akzentfarbe, daher verwendet prev die unten gewählte.
settings-accent-chosen-note = Die Farben werden aus der unten gewählten Farbe abgeleitet.
settings-auto-hide = Symbolleiste ausblenden, wenn der Zeiger das Fenster verlässt
settings-auto-hide-note = Die Symbolleiste schwebt über dem Dokument und gleitet weg, solange der Zeiger außerhalb des Fensters ist.
settings-animations = Animationen
settings-animations-note = Gleitende Leisten und Bereiche, wachsende Dialoge und federnde Schaltflächen.
settings-animations-reduced = Aus, solange im System „Bewegung reduzieren“ aktiv ist.
settings-corner-radius = Eckenradius
settings-corner-radius-note = Für Dialoge und die schwebende Symbolleiste.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparenz der Überlagerung
settings-overlay-note = Wie viel von der Seite durch die schwebende Symbolleiste scheint.
settings-overlay-value = { $percent } %
settings-storage-signatures = Ordner für Signaturen
settings-storage-versions = Ordner für den Versionsverlauf
settings-storage-bookmarks = Lesezeichendatei
settings-storage-apply = Anwenden
settings-storage-choose = Auswählen …
# $file is where the settings file is.
settings-storage-note = Dateien am alten Ort bleiben dort; verschieben Sie sie, um sie weiter zu nutzen. Die Einstellungen von prev werden in { $file } gespeichert.
settings-save-failed = Einstellungen konnten nicht gespeichert werden: { $error }
settings-no-location = Kein Ort für Einstellungen: HOME ist nicht gesetzt
settings-full-path = Verwenden Sie einen vollständigen Pfad, etwa ~/Documents/prev.
settings-path-is-folder = { $path } ist ein Ordner, keine Datei.
settings-folder-missing = Es gibt keinen Ordner { $path }. Erstellen Sie ihn zuerst oder wählen Sie einen aus.
settings-path-is-file = { $path } ist eine Datei, kein Ordner.
settings-cannot-write = prev kann nicht in { $path } schreiben: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Exportieren
# Section headings in the export dialog.
export-format = Format
export-quality = Qualität
export-size = Größe
# Button that goes on to choose where to save the export.
export-choose = Exportieren …
# Format choice; the format name stays as it is.
export-format-webp = WebP (verlustfrei)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = Bild
# JPEG quality choices.
export-quality-low = Niedrig
export-quality-medium = Mittel
export-quality-high = Hoch
export-quality-best = Optimal
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Originalgröße
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } Pixel
# $error is the system's reason.
export-dialog-failed = Der Speichern-Dialog konnte nicht angezeigt werden: { $error }
# $path is where the file was saved.
export-done = Exportiert: { $path }
export-failed = Exportieren fehlgeschlagen: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = das Exportieren wurde abgebrochen

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Bilder mit Markierungen können nicht bearbeitet werden. Exportieren Sie das Bild, um die Markierungen zu behalten, oder löschen Sie sie und schließen Sie die Markierungssymbolleiste.

# Shown if a background task ends unexpectedly.
image-loading-stopped = das Laden wurde abgebrochen
image-reverting-stopped = das Zurücksetzen wurde abgebrochen
image-rendering-stopped = das Rendern wurde abgebrochen
image-saving-stopped = das Speichern wurde abgebrochen
image-markup-stopped = das Markieren wurde abgebrochen
image-no-version-store = Kein Ort zum Aufbewahren von Versionen
image-revert-failed = Zurücksetzen fehlgeschlagen: { $error }
image-read-failed = { $path } konnte nicht gelesen werden: { $error }
image-keep-original-failed = Die Originalversion konnte nicht behalten werden: { $error }
image-save-failed = { $path } konnte nicht gespeichert werden: { $error }
image-markup-start-failed = Das Markieren konnte nicht gestartet werden: { $error }
image-cannot-edit = Animationen und SVG-Zeichnungen können nicht bearbeitet werden.
image-cannot-mark-up = Animationen und SVG-Zeichnungen können nicht markiert werden.
image-mark-up-wait = Warten Sie, bis die Bearbeitung fertig ist, und markieren Sie dann.
image-crop-needs-selection = Ziehen Sie zuerst eine Auswahl auf (Werkzeug „Auswählen“) und beschneiden Sie dann.
image-size-needed = Geben Sie Breite und Höhe in Pixeln ein.
# $name is a file name.
image-cannot-save-format = Änderungen an „{ $name }“ können in diesem Format nicht gespeichert werden. Verwenden Sie „Exportieren“ ({ $keys }).
image-cannot-save-format-unbound = Änderungen an „{ $name }“ können in diesem Format nicht gespeichert werden. Verwenden Sie „Exportieren“.
image-cannot-export-animation = Animationen können noch nicht exportiert werden.
image-drop-pages = Seiten können auf einem Dokument abgelegt werden.
image-drag-failed = Das Ziehen konnte nicht gestartet werden.
image-picture-save-failed = Das Bild konnte nicht im Ordner „Downloads“ gespeichert werden.
image-open-failed = prev kann dieses Bild nicht öffnen
image-opening = Wird geöffnet …
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Der Name passt nicht zum Format
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = „{ $name }“ wird als { $format }-Datei gespeichert, aber der Name endet auf .{ $extension }. Andere Apps können sie möglicherweise nicht öffnen.
image-name-mismatch-no-extension = „{ $name }“ wird als { $format }-Datei gespeichert, aber der Name hat keine Endung. Andere Apps können sie möglicherweise nicht öffnen.
image-choose-again = Neu wählen
image-save-as-is = Trotzdem speichern
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = Frame { $current } von { $total }
image-position = { $current } von { $total }
image-edited = bearbeitet
# Toolbar tooltips.
image-sidebar = Seitenleiste
image-zoom-out = Verkleinern
image-zoom-in = Vergrößern
image-zoom = { $percent } %
image-fit = An Fenster anpassen
image-actual-size = Originalgröße
image-undo = Rückgängig
image-redo = Wiederholen
image-rotate-left = Nach links drehen
image-rotate-right = Nach rechts drehen
image-flip-horizontal = Horizontal spiegeln
image-flip-vertical = Vertikal spiegeln
image-select = Rechteckige Auswahl
image-crop = Auf Auswahl beschneiden
image-adjust-size-tool = Größe anpassen
image-adjust-color-tool = Farbe anpassen
# Tooltip and panel title.
image-inspector = Informationen
image-markup = Markierungen
image-export = Exportieren
image-settings = Einstellungen
# Panel titles.
image-adjust-color = Farbe anpassen
image-adjust-size = Größe anpassen
# Adjust Color sliders.
image-exposure = Belichtung
image-contrast = Kontrast
image-saturation = Sättigung
image-temperature = Temperatur
image-tint = Farbton
image-sepia = Sepia
image-sharpness = Schärfe
image-levels = Tonwerte
image-black-point = Schwarzpunkt
image-midtones = Mitteltöne
image-white-point = Weißpunkt
image-reset-all = Alles zurücksetzen
# Adjust Size panel.
image-current-size = Aktuelle Größe: { $width } × { $height } Pixel
image-width = Breite
image-height = Höhe
image-scale-proportionally = Proportional skalieren
# Button that applies the new size.
image-resize = Größe ändern
# Inspector panel.
image-inspector-loading = Wird geladen …
image-file = Datei
image-format = Format
image-dimensions-label = Abmessungen
image-pixels = { $width } × { $height } Pixel
image-no-camera = Keine Kamerainformationen.
image-location = Ort
image-remove-location = Ortsinformationen entfernen
image-no-location = Keine Ortsinformationen.
image-keywords-description = Schlagwörter und Beschreibung
image-keywords-hint = Schlagwörter, durch Kommas getrennt
image-description = Beschreibung
image-keywords-unsupported = Schlagwörter können in JPEG-, PNG- und WebP-Dateien gespeichert werden.
# Heading over the earlier versions of the file.
image-revert-to = Zurücksetzen auf
image-no-versions = Keine früheren Versionen.
image-revert = Zurücksetzen
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Schließen, ohne die Markierungen zu exportieren?
image-close-body = { $count ->
    [one] Markierungen auf einem Bild bleiben nur erhalten, solange sein Fenster offen ist. Exportieren Sie das Bild, um sie zu behalten: Die Markierungen werden in die gespeicherte Kopie gezeichnet.
   *[other] Markierungen auf Bildern bleiben nur erhalten, solange ihr Fenster offen ist. Exportieren Sie jedes Bild, um sie zu behalten: Die Markierungen werden in die gespeicherte Kopie gezeichnet.
}
image-close-anyway = Trotzdem schließen

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = das Lesen wurde abgebrochen
markdown-read-failed = prev kann diese Datei nicht lesen
markdown-draw-failed = Das Dokument konnte nicht dargestellt werden
# Under the export's size choices.
markdown-export-size = Das ganze Dokument, { $width } × { $height } Pixel
# Search results.
markdown-not-found = Nicht gefunden
markdown-match = { $current } von { $total }
# Placeholder of the search field.
markdown-search = Suchen
# Toolbar tooltips.
markdown-smaller-text = Kleinerer Text
markdown-larger-text = Größerer Text
markdown-zoom = { $percent } %
markdown-actual-size = Originalgröße
# Tooltip and panel title.
markdown-inspector = Informationen
markdown-export = Exportieren
markdown-settings = Einstellungen
# Inspector headings and labels.
markdown-file = Datei
markdown-document = Dokument
markdown-words = Wörter
markdown-lines = Zeilen
markdown-pictures = Bilder

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Kamera
image-meta-exposure = Belichtung
image-meta-image = Bild
image-meta-make = Hersteller
image-meta-model = Modell
image-meta-lens = Objektiv
image-meta-exposure-time = Belichtungszeit
# The lens aperture, written like f/2.8.
image-meta-f-number = Blende
image-meta-iso = ISO
image-meta-focal-length = Brennweite
image-meta-exposure-bias = Belichtungskorrektur
image-meta-flash = Blitz
image-meta-date-taken = Aufnahmedatum
image-meta-orientation = Ausrichtung
image-meta-color-space = Farbraum
image-meta-software = Software
image-meta-artist = Urheber
image-meta-copyright = Copyright
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Horizontal gespiegelt
    [3] Um 180° gedreht
    [4] Vertikal gespiegelt
    [5] Horizontal gespiegelt, um 90° gegen den Uhrzeigersinn gedreht
    [6] Um 90° im Uhrzeigersinn gedreht
    [7] Horizontal gespiegelt, um 90° im Uhrzeigersinn gedreht
    [8] Um 90° gegen den Uhrzeigersinn gedreht
   *[other] Unbekannt ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Ausgelöst
   *[no] Nicht ausgelöst
}{ $mode ->
    [on] , erzwungen
    [off] , aus
    [auto] , automatisch
   *[unknown] {""}
}{ $redeye ->
    [yes] , Rote-Augen-Reduzierung
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Nicht kalibriert
   *[other] Andere ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = das Dokument kann nicht geöffnet werden: { $detail }
error-pdf-page-out-of-range = Seite { $page } existiert nicht
error-pdf-password-protected = das Dokument ist durch ein Passwort geschützt; öffnen Sie es und kopieren Sie stattdessen seine Seiten
error-pdf-no-pages = keine Seiten zum Extrahieren
error-pdf-crop-outside = der Beschnittbereich liegt außerhalb der Seite
error-pdf-closed = Dokument geschlossen
error-pdf-saved-unreadable = das gespeicherte Dokument lässt sich nicht mehr öffnen
error-image-read = die Datei kann nicht gelesen werden: { $detail }
error-image-invalid = das Bild ist beschädigt oder ungültig: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = zum Öffnen dieses Formats wird { $library } benötigt, das nicht installiert ist
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format }-Bilder werden noch nicht unterstützt
error-image-encode = das Bild kann nicht codiert werden: { $detail }
error-exif-malformed = die EXIF-Daten sind fehlerhaft
error-settings-read = die Einstellungen können nicht gelesen werden: { $detail }
error-settings-invalid = ungültige Einstellungen: { $detail }
error-remove-location = die Ortsinformationen konnten nicht entfernt werden: { $error }
error-location-unsupported = Ortsinformationen können aus JPEG-, PNG-, WebP- und TIFF-Dateien entfernt werden
error-xmp-unsupported = Schlagwörter und Beschreibungen können nur in JPEG-, PNG- und WebP-Dateien gespeichert werden

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = Kamera-RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = Über prev
menu-settings = Einstellungen …
menu-services = Dienste
menu-hide = prev ausblenden
menu-hide-others = Andere ausblenden
menu-show-all = Alle einblenden
menu-quit = prev beenden
menu-file = Ablage
menu-open = Öffnen …
menu-close = Fenster schließen
menu-export = Exportieren …
menu-print = Drucken …
menu-edit = Bearbeiten
menu-undo = Widerrufen
menu-redo = Wiederholen
menu-cut = Ausschneiden
menu-copy = Kopieren
menu-paste = Einsetzen
menu-select-all = Alles auswählen
menu-find = Suchen
menu-find-next = Weitersuchen
menu-find-previous = Rückwärts suchen
menu-view = Darstellung
menu-hide-sidebar = Seitenleiste ausblenden
menu-thumbnails = Miniaturen
menu-contents = Inhaltsverzeichnis
menu-notes = Hervorhebungen und Notizen
menu-bookmarks = Lesezeichen
menu-zoom-in = Vergrößern
menu-zoom-out = Verkleinern
menu-actual-size = Originalgröße
menu-zoom-to-fit = In Fenster einpassen
menu-inspector = Informationen einblenden
menu-slideshow = Diashow
menu-full-screen = Vollbildmodus aktivieren
menu-go = Gehe zu
menu-next-page = Nächste Seite
menu-previous-page = Vorherige Seite
menu-go-to-page = Gehe zu Seite …
menu-bookmark = Lesezeichen hinzufügen
menu-tools = Werkzeuge
menu-markup = Markierungssymbolleiste einblenden
menu-rotate-left = Nach links drehen
menu-rotate-right = Nach rechts drehen
menu-crop = Beschneiden
menu-adjust-color = Farbe anpassen …
menu-window = Fenster
menu-minimize = Im Dock ablegen
menu-zoom = Zoomen
menu-bring-all-to-front = Alle nach vorne bringen

## Outside control

settings-outside-control = Steuerung von außen
settings-allow-outside-control = Steuerung von außen erlauben
settings-allow-outside-control-note = KI-Agenten wie Claude Code können über prev --mcp Ihre Dateien in prev lesen und ändern. prev fragt vor jedem neuen Agenten nach.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Erlaubt: { $agents }
settings-forget-agents = Vergessen
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent } erlauben, prev zu steuern?
agent-prompt-body = { $agent } möchte die Steuerung von außen in prev nutzen, um Ihre geöffneten Dateien zu lesen und zu ändern. Sie können die Steuerung von außen in den Einstellungen ausschalten.
agent-prompt-allow = Erlauben
agent-prompt-deny = Nicht erlauben
settings-ask-before-note = Vor diesen Aktionen eines Agenten nachfragen:
settings-ask-reading = Datei lesen
settings-ask-viewing = Ansicht oder Fenster ändern
settings-ask-marking-up = Datei markieren
settings-ask-editing = Datei bearbeiten
settings-ask-signing = Datei signieren
settings-ask-redacting = Schwärzungen anwenden
settings-ask-exporting = Datei exportieren
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = { $agent } erlauben, diese Datei zu lesen?
agent-ask-view = { $agent } erlauben, die Ansicht zu ändern?
agent-ask-markup = { $agent } erlauben, diese Datei zu markieren?
agent-ask-edit = { $agent } erlauben, diese Datei zu bearbeiten?
agent-ask-sign = { $agent } erlauben, diese Datei zu signieren?
agent-ask-redact = { $agent } erlauben, Schwärzungen anzuwenden?
agent-ask-export = { $agent } erlauben, diese Datei zu exportieren?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } möchte „{ $tool }“ verwenden. In den Einstellungen legen Sie fest, wonach prev fragt.
agent-ask-final = Das lässt sich nicht widerrufen.
