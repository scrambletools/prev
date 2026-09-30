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
language-name = Nederlands

## Common

common-cancel = Annuleren
common-close = Sluiten
common-save = Opslaan

## Settings

settings-title = Instellingen
settings-appearance = Weergave
settings-colors = Kleuren
settings-windows = Vensters
settings-storage = Opslag

## Markup toolbar

markup-tool-select = Selecteren
markup-tool-area = Rechthoekige selectie
markup-tool-sketch = Schetsen
markup-tool-draw = Tekenen
markup-tool-shapes = Vormen
markup-tool-text-box = Tekstvak
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Markeren
markup-tool-note = Notitie
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Ondertekenen
# A verb: the tool that marks areas to black out.
markup-tool-redact = Zwartlakken
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Toepassen
markup-apply-redactions = Zwartlakkingen toepassen
markup-shape-style = Vormstijl
markup-border-color = Randkleur
markup-fill-color = Vulkleur
markup-text-style = Tekststijl
markup-delete = Verwijderen
markup-undo = Ongedaan maken
markup-redo = Opnieuw

## Markup menus

markup-shape-rectangle = Rechthoek
markup-shape-rounded-rectangle = Afgeronde rechthoek
markup-shape-oval = Ovaal
markup-shape-line = Lijn
markup-shape-arrow = Pijl
markup-shape-star = Ster
markup-shape-polygon = Veelhoek
markup-shape-speech-bubble = Tekstballon
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Loep
# A shape that darkens the page around it.
markup-shape-mask = Masker
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Markering
markup-style-underline = Onderstreping
markup-style-strikethrough = Doorhaling
markup-style-squiggly = Golflijn
# Menu section headings.
markup-menu-color = Kleur
markup-menu-font = Lettertype
markup-menu-size = Grootte
markup-menu-alignment = Uitlijning
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Gestreept

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Notitie
markup-kind-text-box = Tekstvak
markup-kind-stamp = Stempel
markup-kind-redaction = Zwartlakking
markup-kind-shape = Vorm
# Tooltips on a note being edited.
markup-note-delete = Notitie verwijderen
markup-note-done = Gereed
markup-note-placeholder = Typ een notitie
markup-notes-empty = Geen markeringen of notities
markup-notes-empty-hint = Markeringen, notities en tekstvakken verschijnen hier.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Pagina { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Kan het document niet wijzigen: { $error }
markup-copy-area-failed = Kan het gebied niet kopiëren: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = het document is gesloten
markup-render-area-failed = kan het gebied niet weergeven
markup-copy-stopped = kopiëren is gestopt

## Signatures

signature-menu-empty = Nog geen handtekeningen.
signature-delete = Handtekening verwijderen
signature-create = Handtekening maken…
signature-dialog-title = Handtekening maken
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Tekenen
signature-tab-type = Typen
signature-tab-image = Afbeelding
signature-draw-hint = Onderteken op de lijn met je muis, pen of touchpad.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Je naam
signature-image-hint = Kies een foto of scan van je handtekening op wit papier.
signature-choose-image = Afbeelding kiezen…
# Placeholder of the field naming the signature in the library.
signature-description = Beschrijving, zoals Volledige naam of Initialen
# Clears the drawing, typed name or image.
signature-clear = Wissen
# The color the signature is drawn or typed in.
signature-ink = Inkt
# The pen's width, for drawing.
signature-thickness = Dikte
signature-sign-first = Eerst ondertekenen, dan opslaan.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Handtekening { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Kan de handtekeningen niet wijzigen: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = geen gegevensmap: HOME is niet ingesteld
signature-removing-stopped = verwijderen is gestopt
signature-saving-stopped = opslaan is gestopt
signature-reading-stopped = lezen is gestopt
signature-not-an-image = dat bestand is geen afbeelding die prev kan lezen
signature-no-frames = de afbeelding heeft geen frames
signature-not-found = geen handtekening gevonden in de afbeelding

## Dragging

drag-pages-need-document = Pagina's kunnen op een document worden neergezet.
drag-image-unsupported = prev kan deze afbeelding niet openen.
# $error is a lowercase reason or a technical message.
drag-area-failed = Kan het gebied niet slepen: { $error }
drag-pages-failed = Kan de pagina's niet slepen: { $error }
drag-start-failed = Kan niet beginnen met slepen.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Pagina's
drag-file-one-page = { $name } (pagina { $page })
drag-file-page-range = { $name } (pagina's { $first }–{ $last })

## PDF window

pdf-opening = Openen…
pdf-open-failed = prev kan dit document niet openen
pdf-no-pages = Het document heeft geen pagina's.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = het document is gesloten
pdf-keep-original-failed = kan de originele versie niet behouden: { $error }
pdf-save-failed = Kan niet opslaan: { $error }
pdf-nothing-to-paste = Er is niets om te plakken.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = plakken is gestopt
pdf-file-dialog-failed = Kan het bestandsvenster niet tonen: { $error }
pdf-bookmarks-no-home = Bladwijzers kunnen niet worden opgeslagen: HOME is niet ingesteld
pdf-bookmarks-save-failed = Kan bladwijzers niet opslaan: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Pagina { $page }

# Password prompt. $name is the file name.
pdf-password-protected = ‘{ $name }’ is beveiligd met een wachtwoord
pdf-password = Wachtwoord
pdf-password-wrong = Onjuist wachtwoord. Probeer het opnieuw.
# Button that opens a locked document.
pdf-unlock = Ontgrendelen

# Toolbar tooltips and labels.
pdf-sidebar = Navigatiekolom
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = van { $count }
pdf-zoom-out = Uitzoomen
pdf-zoom-in = Inzoomen
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Hele pagina
pdf-fit-width = Paginabreedte
pdf-actual-size = Ware grootte
pdf-view-continuous = Doorlopend scrollen
pdf-view-single-page = Eén pagina
pdf-view-two-pages = Twee pagina's
pdf-undo = Ongedaan maken
pdf-redo = Opnieuw
pdf-rotate-left = Linksom draaien
pdf-rotate-right = Rechtsom draaien
pdf-inspector = Infovenster
pdf-markup = Annotaties
# Tooltip of the button that opens the export dialog.
pdf-export = Exporteren
pdf-settings = Instellingen

# Search field.
pdf-search = Zoeken
pdf-search-not-found = Niet gevonden
pdf-searching = Zoeken…
# The match shown, of all matches found.
pdf-search-match = { $current } van { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } van { $total }+

# Inspector: section headings.
pdf-inspector-file = Bestand
pdf-inspector-document = Document
pdf-inspector-pages = Pagina's
# Inspector: fact labels and values.
pdf-inspector-title = Titel
pdf-inspector-author = Auteur
pdf-inspector-subject = Onderwerp
pdf-inspector-keywords = Trefwoorden
pdf-inspector-created = Aangemaakt
pdf-inspector-modified = Gewijzigd
pdf-inspector-application = Programma
pdf-inspector-producer = PDF-producent
pdf-inspector-version = Versie
pdf-inspector-security = Beveiliging
pdf-inspector-not-encrypted = Niet versleuteld
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Versleuteld ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } pagina
   *[other] { $count } pagina's
}
pdf-inspector-page-size = Paginaformaat
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inch)
pdf-loading = Laden…

# Sidebar tabs and lists.
pdf-tab-pages = Pagina's
pdf-tab-contents = Inhoud
pdf-tab-notes = Markeringen en notities
pdf-tab-bookmarks = Bladwijzers
pdf-no-outline = Geen inhoudsopgave
pdf-no-outline-detail = Dit document heeft geen inhoudsopgave.
pdf-no-bookmarks = Geen bladwijzers
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Druk op { $keys } om een bladwijzer voor een pagina te maken.
pdf-remove-bookmark = Bladwijzer verwijderen

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Pagina's
pages-insert-blank = Lege pagina invoegen
pages-insert-file = Invoegen uit bestand…
pages-copy = { $count ->
    [one] Pagina kopiëren
   *[other] Pagina's kopiëren
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Pagina plakken
   *[other] { $count } pagina's plakken
}
pages-crop = Bijsnijden tot selectie
pages-select-all = Alle pagina's selecteren
pages-delete = { $count ->
    [one] Pagina verwijderen
   *[other] Pagina's verwijderen
}
pages-apply-redactions = Zwartlakkingen toepassen…
pages-no-copied = Er zijn geen gekopieerde pagina's om te plakken.
pages-copied = { $count ->
    [one] { $count } pagina gekopieerd.
   *[other] { $count } pagina's gekopieerd.
}
pages-copy-failed = Kan de pagina's niet kopiëren: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = lezen is gestopt
pages-read-failed = Kan het bestand niet lezen: { $error }
pages-at-least-one = Een document moet minstens één pagina hebben.
pages-crop-needs-area = Kies eerst een gebied met het gereedschap voor rechthoekige selectie.
pages-change-failed = Kan de pagina's niet wijzigen: { $error }
pages-no-redactions = Er waren geen zwartlakkingen om toe te passen.
pages-redactions-applied = { $count ->
    [one] { $count } zwartlakking toegepast.
   *[other] { $count } zwartlakkingen toegepast.
}
pages-forget-versions-failed = Kan eerdere versies niet verwijderen: { $error }
pages-redact-title = Zwartlakkingen toepassen?
pages-redact-body = { $count ->
    [one] Tekst, afbeeldingen en tekeningen onder de markering worden definitief uit het document verwijderd, en de markering wordt een zwart vlak. Dit kan niet ongedaan worden gemaakt, en de eerdere versies van dit bestand die prev bewaart, worden verwijderd.
   *[other] Tekst, afbeeldingen en tekeningen onder de { $count } markeringen worden definitief uit het document verwijderd, en de markeringen worden zwarte vlakken. Dit kan niet ongedaan worden gemaakt, en de eerdere versies van dit bestand die prev bewaart, worden verwijderd.
}
# Button that applies redactions.
pages-redact-apply = Toepassen

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Exporteren
pages-export-format = Indeling
pages-export-reduce = Bestand verkleinen (afbeeldingen op 150 dpi)
pages-export-flatten = Annotaties en formuliervelden samenvoegen
pages-export-flatten-detail = Annotaties en ingevulde velden worden onderdeel van de pagina's en kunnen niet meer worden gewijzigd. Nog niet toegepaste zwartlakkingen worden weggelaten.
pages-export-encrypt = Versleutelen met een wachtwoord
pages-export-password = Wachtwoord
pages-export-verify-password = Wachtwoord bevestigen
pages-export-resolution = Resolutie
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kwaliteit
# JPEG quality choices.
pages-export-quality-low = Laag
pages-export-quality-medium = Gemiddeld
pages-export-quality-high = Hoog
pages-export-quality-best = Beste
pages-export-one-file = Alle pagina's komen in één bestand.
pages-export-file-per-page = Elke pagina wordt als apart bestand opgeslagen, met een nummer achter de naam die je kiest.
pages-export-selected-only = { $count ->
    [one] Alleen de geselecteerde pagina
   *[other] Alleen de { $count } geselecteerde pagina's
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Exporteren…
pages-export-no-password = Voer een wachtwoord in.
pages-export-password-mismatch = De wachtwoorden komen niet overeen.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (geëxporteerd)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = document
pages-export-same-file = Exporteer naar een nieuw bestand; dit document wordt vanzelf opgeslagen.
pages-export-exporting = ‘{ $name }’ wordt geëxporteerd…
pages-export-done = ‘{ $name }’ geëxporteerd.
pages-export-done-images = { $count } afbeeldingen geëxporteerd.
pages-export-failed = Kan niet exporteren: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = exporteren is gestopt

## Start window

# Under the app name in a window with no file open.
app-start-hint = Open of sleep een PDF, afbeelding, SVG- of Markdown-bestand hierheen.
app-start-open = Openen…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (dev)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: deze viewer bestaat nog niet.
app-cannot-open = prev kan dit soort bestand niet openen.
app-cannot-read = prev kan dit bestand niet lezen: { $error }
app-kind-pdf = PDF-document
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format }-afbeelding
app-kind-svg = SVG-tekening
app-kind-markdown = Markdown-document
app-file-dialog-failed = Kan het bestandsvenster niet tonen: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Openen
action-settings = Instellingen

## Toolbar

app-toolbar-keep-shown = Knoppenbalk altijd tonen
app-toolbar-auto-hide = Knoppenbalk verbergen als de aanwijzer het venster verlaat
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Meer

## File facts

# Labels in a file's inspector.
app-fact-name = Naam
app-fact-folder = Map
app-fact-size = Grootte
app-fact-modified = Gewijzigd
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } bytes
}
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Ongeldige link { $uri }: { $error }
app-link-open-failed = Kan { $uri } niet openen: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = installeer wl-clipboard om afbeeldingen te plakken
app-copy-needs-wl-clipboard = installeer wl-clipboard om afbeeldingen te kopiëren
app-copy-no-pixels = het gebied heeft geen pixels
# wl-copy is a program's name.
app-copy-no-input = wl-copy heeft geen invoer
app-copy-failed = wl-copy is mislukt
app-clipboard-open-failed = Kan het klembord niet openen: { $error }
app-copy-image-failed = Kan de afbeelding niet kopiëren: { $error }

## Printing

print-failed = Kan niet afdrukken: { $error }
print-stopped = Afdrukken is gestopt
print-unavailable = Afdrukken is nog niet beschikbaar op dit systeem.
print-no-window = Kan niet afdrukken: geen venster om het afdrukvenster boven te tonen
print-dialog-failed = Kan het afdrukvenster niet tonen: { $error }
# Shown after "Could not print:".
print-job-not-started = de printer is niet met de taak begonnen
# Shown after "Could not print:".
print-printer-stopped = de printer is gestopt

## File dialogs

dialog-open = Openen
dialog-filter-all = Alle ondersteunde bestanden
dialog-filter-pdf = PDF-documenten
dialog-filter-images = Afbeeldingen
dialog-filter-svg = SVG-tekeningen
dialog-filter-markdown = Markdown
dialog-choose-signatures = Kies de map voor handtekeningen
dialog-choose-versions = Kies de map voor de versiegeschiedenis
dialog-choose-bookmarks = Kies het bestand voor bladwijzers

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Gebruik: prev [FILE]...

    Bekijk en bewerk PDF's en afbeeldingen. Bestanden openen in vensters van
    prev als dat al draait; anders wordt prev eerst gestart.

    Opties:
      -h, --help     Deze hulp tonen
      -V, --version  De versie tonen

## Settings, continued

settings-language = Taal
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Systeemstandaard: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Invoertaal
settings-input-language-system = Toetsenbordindeling volgen
settings-input-language-note = Bepaalt aan welke kant een leeg tekstveld begint. Getypte tekst houdt zijn eigen richting.

settings-appearance-system = Systeem
settings-appearance-light = Licht
settings-appearance-dark = Donker
settings-omarchy-accent = Accentkleur van Omarchy gebruiken
# $theme is the Omarchy theme's name.
settings-omarchy-note = De kleuren zijn afgeleid van het accent van ‘{ $theme }’.
settings-omarchy-none = Er is geen Omarchy-thema actief.
settings-auto-hide = Knoppenbalk verbergen als de aanwijzer het venster verlaat
settings-auto-hide-note = De knoppenbalk zweeft boven het document en schuift weg zolang de aanwijzer buiten het venster is.
settings-animations = Animaties
settings-animations-note = Schuivende balken en panelen, groeiende dialoogvensters en verende knoppen.
settings-animations-reduced = Uit zolang het systeem om minder beweging vraagt.
settings-corner-radius = Hoekradius
settings-corner-radius-note = Voor dialoogvensters en de zwevende knoppenbalk.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparantie van de overlay
settings-overlay-note = Hoeveel van de pagina door de zwevende knoppenbalk schijnt.
settings-overlay-value = { $percent }%
settings-storage-signatures = Map voor handtekeningen
settings-storage-versions = Map voor versiegeschiedenis
settings-storage-bookmarks = Bestand voor bladwijzers
settings-storage-apply = Toepassen
settings-storage-choose = Kiezen…
# $file is where the settings file is.
settings-storage-note = Bestanden die al op een oude plek staan, blijven daar; verplaats ze om ze te blijven gebruiken. De instellingen van prev worden opgeslagen in { $file }.
settings-save-failed = Kan instellingen niet opslaan: { $error }
settings-no-location = Geen plek voor instellingen: HOME is niet ingesteld
settings-full-path = Gebruik een volledig pad, zoals ~/Documents/prev.
settings-path-is-folder = { $path } is een map, geen bestand.
settings-folder-missing = De map { $path } bestaat niet. Maak hem eerst aan of kies er een.
settings-path-is-file = { $path } is een bestand, geen map.
settings-cannot-write = prev kan niet schrijven in { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Exporteren
# Section headings in the export dialog.
export-format = Indeling
export-quality = Kwaliteit
export-size = Grootte
# Button that goes on to choose where to save the export.
export-choose = Exporteren…
# Format choice; the format name stays as it is.
export-format-webp = WebP (verliesvrij)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = afbeelding
# JPEG quality choices.
export-quality-low = Laag
export-quality-medium = Gemiddeld
export-quality-high = Hoog
export-quality-best = Beste
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Ware grootte
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } pixels
# $error is the system's reason.
export-dialog-failed = Kan het opslagvenster niet tonen: { $error }
# $path is where the file was saved.
export-done = { $path } geëxporteerd
export-failed = Kan niet exporteren: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = exporteren is gestopt

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Afbeeldingen met annotaties kunnen niet worden bewerkt. Exporteer om de annotaties te behouden, of verwijder ze en sluit de annotatiebalk.

# Shown if a background task ends unexpectedly.
image-loading-stopped = laden is gestopt
image-reverting-stopped = terugzetten is gestopt
image-rendering-stopped = weergeven is gestopt
image-saving-stopped = opslaan is gestopt
image-markup-stopped = annoteren is gestopt
image-no-version-store = Geen plek om versies te bewaren
image-revert-failed = Kan niet terugzetten: { $error }
image-read-failed = Kan { $path } niet lezen: { $error }
image-keep-original-failed = Kan de originele versie niet behouden: { $error }
image-save-failed = Kan { $path } niet opslaan: { $error }
image-markup-start-failed = Kan annoteren niet starten: { $error }
image-cannot-edit = Animaties en SVG-tekeningen kunnen niet worden bewerkt.
image-cannot-mark-up = Animaties en SVG-tekeningen kunnen niet worden geannoteerd.
image-mark-up-wait = Wacht tot de bewerking klaar is en annoteer dan.
image-crop-needs-selection = Sleep eerst een selectie (gereedschap Selecteren) en snijd dan bij.
image-size-needed = Voer een breedte en hoogte in pixels in.
# $name is a file name.
image-cannot-save-format = Wijzigingen in ‘{ $name }’ kunnen niet in de eigen indeling worden opgeslagen. Gebruik Exporteren ({ $keys }).
image-cannot-export-animation = Animaties kunnen nog niet worden geëxporteerd.
image-drop-pages = Pagina's kunnen op een document worden neergezet.
image-drag-failed = Kan niet beginnen met slepen.
image-open-failed = prev kan deze afbeelding niet openen
image-opening = Openen…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = De naam past niet bij de indeling
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = ‘{ $name }’ wordt opgeslagen als { $format }-bestand, maar de naam eindigt op .{ $extension }. Andere apps kunnen het misschien niet openen.
image-name-mismatch-no-extension = ‘{ $name }’ wordt opgeslagen als { $format }-bestand, maar de naam heeft geen extensie. Andere apps kunnen het misschien niet openen.
image-choose-again = Opnieuw kiezen
image-save-as-is = Toch opslaan
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = frame { $current } van { $total }
image-position = { $current } van { $total }
image-edited = gewijzigd
# Toolbar tooltips.
image-sidebar = Navigatiekolom
image-zoom-out = Uitzoomen
image-zoom-in = Inzoomen
image-zoom = { $percent }%
image-fit = Passend in venster
image-actual-size = Ware grootte
image-undo = Ongedaan maken
image-redo = Opnieuw
image-rotate-left = Linksom draaien
image-rotate-right = Rechtsom draaien
image-flip-horizontal = Horizontaal spiegelen
image-flip-vertical = Verticaal spiegelen
image-select = Rechthoekige selectie
image-crop = Bijsnijden tot selectie
image-adjust-size-tool = Grootte aanpassen
image-adjust-color-tool = Kleur aanpassen
# Tooltip and panel title.
image-inspector = Infovenster
image-markup = Annotaties
image-export = Exporteren
image-settings = Instellingen
# Panel titles.
image-adjust-color = Kleur aanpassen
image-adjust-size = Grootte aanpassen
# Adjust Color sliders.
image-exposure = Belichting
image-contrast = Contrast
image-saturation = Verzadiging
image-temperature = Temperatuur
image-tint = Tint
image-sepia = Sepia
image-sharpness = Scherpte
image-levels = Niveaus
image-black-point = Zwartpunt
image-midtones = Middentonen
image-white-point = Witpunt
image-reset-all = Alles herstellen
# Adjust Size panel.
image-current-size = Huidige grootte: { $width } × { $height } pixels
image-width = Breedte
image-height = Hoogte
image-scale-proportionally = Proportioneel schalen
# Button that applies the new size.
image-resize = Grootte wijzigen
# Inspector panel.
image-inspector-loading = Laden…
image-file = Bestand
image-format = Indeling
image-dimensions-label = Afmetingen
image-pixels = { $width } × { $height } pixels
image-no-camera = Geen cameragegevens.
image-location = Locatie
image-remove-location = Locatiegegevens verwijderen
image-no-location = Geen locatiegegevens.
image-keywords-description = Trefwoorden en beschrijving
image-keywords-hint = Trefwoorden, gescheiden door komma's
image-description = Beschrijving
image-keywords-unsupported = Trefwoorden kunnen worden opgeslagen in JPEG-, PNG- en WebP-bestanden.
# Heading over the earlier versions of the file.
image-revert-to = Terugzetten naar
image-no-versions = Geen eerdere versies.
image-revert = Terugzetten
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Sluiten zonder de annotaties te exporteren?
image-close-body = { $count ->
    [one] Annotaties op een afbeelding blijven alleen bestaan zolang het venster open is. Exporteer de afbeelding om ze te behouden: de annotaties worden getekend in de kopie die je opslaat.
   *[other] Annotaties op afbeeldingen blijven alleen bestaan zolang hun venster open is. Exporteer elke afbeelding om ze te behouden: de annotaties worden getekend in de kopie die je opslaat.
}
image-close-anyway = Toch sluiten

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = lezen is gestopt
markdown-read-failed = prev kan dit bestand niet lezen
markdown-draw-failed = Kan het document niet weergeven
# Under the export's size choices.
markdown-export-size = Het hele document, { $width } × { $height } pixels
# Search results.
markdown-not-found = Niet gevonden
markdown-match = { $current } van { $total }
# Placeholder of the search field.
markdown-search = Zoeken
# Toolbar tooltips.
markdown-smaller-text = Kleinere tekst
markdown-larger-text = Grotere tekst
markdown-zoom = { $percent }%
markdown-actual-size = Ware grootte
# Tooltip and panel title.
markdown-inspector = Infovenster
markdown-export = Exporteren
markdown-settings = Instellingen
# Inspector headings and labels.
markdown-file = Bestand
markdown-document = Document
markdown-words = Woorden
markdown-lines = Regels
markdown-pictures = Afbeeldingen

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Camera
image-meta-exposure = Belichting
image-meta-image = Afbeelding
image-meta-make = Merk
image-meta-model = Model
image-meta-lens = Lens
image-meta-exposure-time = Sluitertijd
# The lens aperture, written like f/2.8.
image-meta-f-number = Diafragma
image-meta-iso = ISO
image-meta-focal-length = Brandpuntsafstand
image-meta-exposure-bias = Belichtingscorrectie
image-meta-flash = Flitser
image-meta-date-taken = Opnamedatum
image-meta-orientation = Richting
image-meta-color-space = Kleurruimte
image-meta-software = Software
image-meta-artist = Maker
image-meta-copyright = Copyright
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normaal
    [2] Horizontaal gespiegeld
    [3] 180° gedraaid
    [4] Verticaal gespiegeld
    [5] Horizontaal gespiegeld, 90° linksom gedraaid
    [6] 90° rechtsom gedraaid
    [7] Horizontaal gespiegeld, 90° rechtsom gedraaid
    [8] 90° linksom gedraaid
   *[other] Onbekend ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Geflitst
   *[no] Niet geflitst
}{ $mode ->
    [on] , geforceerd
    [off] , uit
    [auto] , automatisch
   *[unknown] {""}
}{ $redeye ->
    [yes] , rode-ogenreductie
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Niet gekalibreerd
   *[other] Anders ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = kan het document niet openen: { $detail }
error-pdf-page-out-of-range = pagina { $page } bestaat niet
error-pdf-password-protected = het document is beveiligd met een wachtwoord; open het en kopieer in plaats daarvan de pagina's
error-pdf-no-pages = geen pagina's om te extraheren
error-pdf-crop-outside = het bijsnijgebied ligt buiten de pagina
error-pdf-closed = document gesloten
error-pdf-saved-unreadable = het opgeslagen document opent niet meer
error-image-read = kan het bestand niet lezen: { $detail }
error-image-invalid = de afbeelding is beschadigd of ongeldig: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = voor het openen van deze indeling is { $library } nodig, maar dat is niet geïnstalleerd
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format }-afbeeldingen worden nog niet ondersteund
error-image-encode = kan de afbeelding niet coderen: { $detail }
error-exif-malformed = de EXIF-gegevens zijn ongeldig
error-settings-read = kan de instellingen niet lezen: { $detail }
error-settings-invalid = ongeldige instellingen: { $detail }
error-remove-location = kan de locatie niet verwijderen: { $error }
error-location-unsupported = locatiegegevens kunnen worden verwijderd uit JPEG-, PNG-, WebP- en TIFF-bestanden
error-xmp-unsupported = trefwoorden en beschrijvingen kunnen alleen worden opgeslagen in JPEG-, PNG- en WebP-bestanden

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = Camera-RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = Over prev
menu-settings = Instellingen…
menu-services = Voorzieningen
menu-hide = Verberg prev
menu-hide-others = Verberg andere
menu-show-all = Toon alles
menu-quit = Stop prev
menu-file = Archief
menu-open = Open…
menu-close = Sluit venster
menu-export = Exporteer…
menu-print = Druk af…
menu-edit = Wijzig
menu-undo = Herstel
menu-redo = Opnieuw
menu-cut = Knip
menu-copy = Kopieer
menu-paste = Plak
menu-select-all = Selecteer alles
menu-find = Zoek
menu-find-next = Zoek volgende
menu-find-previous = Zoek vorige
menu-view = Weergave
menu-hide-sidebar = Verberg navigatiekolom
menu-thumbnails = Miniaturen
menu-contents = Inhoudsopgave
menu-notes = Markeringen en notities
menu-bookmarks = Bladwijzers
menu-zoom-in = Zoom in
menu-zoom-out = Zoom uit
menu-actual-size = Ware grootte
menu-zoom-to-fit = Zoom naar passend
menu-inspector = Toon infovenster
menu-slideshow = Diavoorstelling
menu-full-screen = Activeer schermvullende weergave
menu-go = Ga
menu-next-page = Volgende pagina
menu-previous-page = Vorige pagina
menu-go-to-page = Ga naar pagina…
menu-bookmark = Voeg bladwijzer toe
menu-tools = Gereedschap
menu-markup = Toon annotatieknoppenbalk
menu-rotate-left = Roteer linksom
menu-rotate-right = Roteer rechtsom
menu-crop = Snij bij
menu-adjust-color = Pas kleur aan…
menu-window = Venster
menu-minimize = Minimaliseer
menu-zoom = Zoom
menu-bring-all-to-front = Breng alles naar voren
