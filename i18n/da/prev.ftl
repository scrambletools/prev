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
language-name = Dansk

## Common

common-cancel = Annuller
common-close = Luk
common-save = Gem

## Settings

settings-title = Indstillinger
settings-appearance = Udseende
settings-colors = Farver
settings-windows = Vinduer
settings-default-app = Standardapp
settings-default-app-label = Åbn filer med prev
settings-default-app-note = Gør prev til den app, der åbner PDF'er, billeder, SVG-tegninger og Markdown-filer.
settings-default-app-note-windows = Windows lader dig kun vælge standardapps i sine egne Indstillinger. Dette åbner prevs side der.
settings-default-app-note-macos = macOS beder dig bekræfte hver type: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP og AVIF.
settings-default-app-status = { $set } af { $total } filtyper åbnes med prev.
settings-default-app-button = Gør til standard
settings-default-app-button-windows = Åbn Indstillinger
settings-default-app-no-entry = prevs skrivebordsfil er ikke installeret, så systemet kan ikke åbne filer med den. Installer prev fra en pakke eller med scripts/install.sh.
settings-default-app-no-bundle = Åbn prev fra prev.app for at gøre den til standard.
settings-default-app-failed = prev kunne ikke gøres til standard: { $error }
settings-storage = Lagring
settings-version = prev { $version }
settings-version-development = prev { $version } (udviklingsversion)

## Markup toolbar

markup-tool-select = Vælg
markup-tool-area = Rektangulær markering
markup-tool-sketch = Skitse
markup-tool-draw = Tegn
markup-tool-shapes = Figurer
markup-tool-text-box = Tekstfelt
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Fremhæv
markup-tool-note = Note
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Underskriv
# A verb: the tool that marks areas to black out.
markup-tool-redact = Sværte
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Anvend
markup-apply-redactions = Anvend sværtninger
markup-shape-style = Figurstil
markup-border-color = Kantfarve
markup-fill-color = Fyldfarve
markup-text-style = Tekststil
markup-delete = Slet
markup-undo = Fortryd
markup-redo = Gentag

## Markup menus

markup-shape-rectangle = Rektangel
markup-shape-rounded-rectangle = Afrundet rektangel
markup-shape-oval = Oval
markup-shape-line = Linje
markup-shape-arrow = Pil
markup-shape-star = Stjerne
markup-shape-polygon = Polygon
markup-shape-speech-bubble = Taleboble
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Lup
# A shape that darkens the page around it.
markup-shape-mask = Maske
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Fremhævning
markup-style-underline = Understregning
markup-style-strikethrough = Gennemstregning
markup-style-squiggly = Bølgelinje
# Menu section headings.
markup-menu-color = Farve
markup-menu-font = Skrift
markup-menu-size = Størrelse
markup-menu-alignment = Justering
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Stiplet

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Note
markup-kind-text-box = Tekstfelt
markup-kind-stamp = Stempel
markup-kind-redaction = Sværtning
markup-kind-shape = Figur
# Tooltips on a note being edited.
markup-note-delete = Slet note
markup-note-done = Færdig
markup-note-placeholder = Skriv en note
markup-notes-empty = Ingen fremhævninger eller noter
markup-notes-empty-hint = Fremhævninger, noter og tekstfelter vises her.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Side { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Dokumentet kunne ikke ændres: { $error }
markup-copy-area-failed = Området kunne ikke kopieres: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = dokumentet blev lukket
markup-render-area-failed = området kunne ikke gengives
markup-copy-stopped = kopieringen stoppede

## Signatures

signature-menu-empty = Ingen underskrifter endnu.
signature-delete = Slet underskrift
signature-create = Opret underskrift…
signature-dialog-title = Opret underskrift
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Tegn
signature-tab-type = Skriv
signature-tab-image = Billede
signature-draw-hint = Skriv under på linjen med mus, pen eller pegefelt.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Dit navn
signature-image-hint = Vælg et foto eller en scanning af din underskrift på hvidt papir.
signature-choose-image = Vælg billede…
# Placeholder of the field naming the signature in the library.
signature-description = Beskrivelse, f.eks. Fuldt navn eller Initialer
# Clears the drawing, typed name or image.
signature-clear = Ryd
# The color the signature is drawn or typed in.
signature-ink = Blæk
# The pen's width, for drawing.
signature-thickness = Tykkelse
signature-sign-first = Skriv under først, og gem derefter.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Underskrift { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Underskrifterne kunne ikke ændres: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = ingen datamappe: HOME er ikke angivet
signature-removing-stopped = fjernelsen stoppede
signature-saving-stopped = lagringen stoppede
signature-reading-stopped = læsningen stoppede
signature-not-an-image = filen er ikke et billede, som prev kan læse
signature-no-frames = billedet har ingen enkeltbilleder
signature-not-found = der blev ikke fundet nogen underskrift i billedet

## Dragging

drag-pages-need-document = Sider kan slippes på et dokument.
drag-image-unsupported = prev kan ikke åbne dette billede.
# $error is a lowercase reason or a technical message.
drag-area-failed = Området kunne ikke trækkes: { $error }
drag-pages-failed = Siderne kunne ikke trækkes: { $error }
drag-start-failed = Trækningen kunne ikke startes.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Sider
drag-file-one-page = { $name } (side { $page })
drag-file-page-range = { $name } (sider { $first }–{ $last })

## PDF window

pdf-opening = Åbner…
pdf-open-failed = prev kan ikke åbne dette dokument
pdf-no-pages = Dokumentet har ingen sider.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = dokumentet blev lukket
pdf-keep-original-failed = den oprindelige version kunne ikke bevares: { $error }
pdf-save-failed = Kunne ikke gemme: { $error }
pdf-nothing-to-paste = Der er intet at indsætte.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = indsættelsen stoppede
pdf-file-dialog-failed = Fildialogen kunne ikke vises: { $error }
pdf-bookmarks-no-home = Bogmærker kan ikke gemmes: HOME er ikke angivet
pdf-bookmarks-save-failed = Bogmærker kunne ikke gemmes: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Side { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” er beskyttet med en adgangskode
pdf-password = Adgangskode
pdf-password-wrong = Forkert adgangskode. Prøv igen.
# Button that opens a locked document.
pdf-unlock = Lås op

# Toolbar tooltips and labels.
pdf-sidebar = Sidepanel
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = af { $count }
pdf-zoom-out = Zoom ud
pdf-zoom-in = Zoom ind
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Hel side
pdf-fit-width = Sidebredde
pdf-actual-size = Faktisk størrelse
pdf-view-continuous = Fortløbende rulning
pdf-view-single-page = Én side
pdf-view-two-pages = To sider
pdf-undo = Fortryd
pdf-redo = Gentag
pdf-rotate-left = Roter mod venstre
pdf-rotate-right = Roter mod højre
pdf-inspector = Inspektør
pdf-markup = Markering
# Tooltip of the button that opens the export dialog.
pdf-export = Eksporter
pdf-settings = Indstillinger

# Search field.
pdf-search = Søg
pdf-search-not-found = Ikke fundet
pdf-searching = Søger…
# The match shown, of all matches found.
pdf-search-match = { $current } af { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } af { $total }+

# Inspector: section headings.
pdf-inspector-file = Fil
pdf-inspector-document = Dokument
pdf-inspector-pages = Sider
# Inspector: fact labels and values.
pdf-inspector-title = Titel
pdf-inspector-author = Forfatter
pdf-inspector-subject = Emne
pdf-inspector-keywords = Nøgleord
pdf-inspector-created = Oprettet
pdf-inspector-modified = Ændret
pdf-inspector-application = Program
pdf-inspector-producer = PDF-producent
pdf-inspector-version = Version
pdf-inspector-security = Sikkerhed
pdf-inspector-not-encrypted = Ikke krypteret
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Krypteret ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } side
   *[other] { $count } sider
}
pdf-inspector-page-size = Sidestørrelse
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } tommer)
pdf-loading = Indlæser…

# Sidebar tabs and lists.
pdf-tab-pages = Sider
pdf-tab-contents = Indhold
pdf-tab-notes = Fremhævninger og noter
pdf-tab-bookmarks = Bogmærker
pdf-no-outline = Ingen indholdsfortegnelse
pdf-no-outline-detail = Dette dokument har ingen disposition.
pdf-no-bookmarks = Ingen bogmærker
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Tryk på { $keys } for at sætte et bogmærke på en side.
pdf-no-bookmarks-detail-unbound = Sider med bogmærker vises her.
pdf-remove-bookmark = Fjern bogmærke

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Sider
pages-insert-blank = Indsæt tom side
pages-insert-file = Indsæt fra fil…
pages-copy = { $count ->
    [one] Kopier side
   *[other] Kopier sider
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Indsæt side
   *[other] Indsæt { $count } sider
}
pages-crop = Beskær til markering
pages-select-all = Vælg alle sider
pages-delete = { $count ->
    [one] Slet side
   *[other] Slet sider
}
pages-apply-redactions = Anvend sværtninger…
pages-no-copied = Der er ingen kopierede sider at indsætte.
pages-copied = { $count ->
    [one] { $count } side kopieret.
   *[other] { $count } sider kopieret.
}
pages-copy-failed = Siderne kunne ikke kopieres: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = læsningen stoppede
pages-read-failed = Filen kunne ikke læses: { $error }
pages-at-least-one = Et dokument skal have mindst én side.
pages-crop-needs-area = Vælg først et område med værktøjet til rektangulær markering.
pages-change-failed = Siderne kunne ikke ændres: { $error }
pages-no-redactions = Der var ingen sværtninger at anvende.
pages-redactions-applied = { $count ->
    [one] { $count } sværtning anvendt.
   *[other] { $count } sværtninger anvendt.
}
pages-forget-versions-failed = Tidligere versioner kunne ikke slettes: { $error }
pages-redact-title = Anvend sværtninger?
pages-redact-body = { $count ->
    [one] Tekst, billeder og tegninger under markeringen fjernes permanent fra dokumentet, og markeringen bliver til en sort boks. Det kan ikke fortrydes, og de tidligere versioner af filen, som prev gemmer, slettes.
   *[other] Tekst, billeder og tegninger under de { $count } markeringer fjernes permanent fra dokumentet, og markeringerne bliver til sorte bokse. Det kan ikke fortrydes, og de tidligere versioner af filen, som prev gemmer, slettes.
}
# Button that applies redactions.
pages-redact-apply = Anvend

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Eksporter
pages-export-format = Format
pages-export-reduce = Reducer filstørrelse (billeder i 150 dpi)
pages-export-flatten = Gør annoteringer og formularfelter flade
pages-export-flatten-detail = Markeringer og udfyldte felter bliver en del af siderne og kan ikke længere redigeres. Sværtninger, der ikke er anvendt endnu, udelades.
pages-export-encrypt = Krypter med en adgangskode
pages-export-password = Adgangskode
pages-export-verify-password = Bekræft adgangskode
pages-export-resolution = Opløsning
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kvalitet
# JPEG quality choices.
pages-export-quality-low = Lav
pages-export-quality-medium = Middel
pages-export-quality-high = Høj
pages-export-quality-best = Bedst
pages-export-one-file = Alle sider samles i én fil.
pages-export-file-per-page = Hver side gemmes som sin egen fil med et nummer efter det navn, du vælger.
pages-export-selected-only = { $count ->
    [one] Kun den valgte side
   *[other] Kun de { $count } valgte sider
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Eksporter…
pages-export-no-password = Angiv en adgangskode.
pages-export-password-mismatch = Adgangskoderne er ikke ens.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (eksporteret)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = dokument
pages-export-same-file = Eksporter til en ny fil; dette dokument gemmes automatisk.
pages-export-exporting = Eksporterer “{ $name }”…
pages-export-done = “{ $name }” er eksporteret.
pages-export-done-images = { $count } billeder er eksporteret.
pages-export-failed = Kunne ikke eksportere: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = eksporten stoppede

## Start window

# Under the app name in a window with no file open.
app-start-hint = Åbn eller slip en PDF-, billed-, SVG- eller Markdown-fil.
app-start-open = Åbn…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (udvikling)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: denne fremviser findes ikke endnu.
app-cannot-open = prev kan ikke åbne denne filtype.
app-cannot-read = prev kan ikke læse denne fil: { $error }
app-kind-pdf = PDF-dokument
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format }-billede
app-kind-svg = SVG-tegning
app-kind-markdown = Markdown-dokument
app-file-dialog-failed = Fildialogen kunne ikke vises: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Åbn
action-settings = Indstillinger

## Toolbar

app-toolbar-keep-shown = Vis altid værktøjslinjen
app-toolbar-auto-hide = Skjul værktøjslinjen, når markøren forlader vinduet
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Mere

## File facts

# Labels in a file's inspector.
app-fact-name = Navn
app-fact-folder = Mappe
app-fact-size = Størrelse
app-fact-modified = Ændret
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } byte
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Ugyldigt link { $uri }: { $error }
app-link-open-failed = { $uri } kunne ikke åbnes: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = installer wl-clipboard for at indsætte billeder
app-copy-needs-wl-clipboard = installer wl-clipboard for at kopiere billeder
app-copy-no-pixels = området har ingen pixels
# wl-copy is a program's name.
app-copy-no-input = wl-copy fik ingen input
app-copy-failed = wl-copy mislykkedes
app-clipboard-open-failed = Udklipsholderen kunne ikke åbnes: { $error }
app-copy-image-failed = Billedet kunne ikke kopieres: { $error }

## Printing

print-failed = Kunne ikke udskrive: { $error }
print-stopped = Udskrivningen stoppede
print-unavailable = Udskrivning er endnu ikke tilgængelig på dette system.
print-no-window = Kunne ikke udskrive: intet vindue at vise udskrivningsdialogen over
print-dialog-failed = Udskrivningsdialogen kunne ikke vises: { $error }
# Shown after "Could not print:".
print-job-not-started = printeren startede ikke jobbet
# Shown after "Could not print:".
print-printer-stopped = printeren stoppede

## File dialogs

dialog-open = Åbn
dialog-filter-all = Alle understøttede filer
dialog-filter-pdf = PDF-dokumenter
dialog-filter-images = Billeder
dialog-filter-svg = SVG-tegninger
dialog-filter-markdown = Markdown
dialog-choose-signatures = Vælg mappen til underskrifter
dialog-choose-versions = Vælg mappen til versionshistorik
dialog-choose-bookmarks = Vælg filen til bogmærker

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Brug: prev [FILE]...

    Se og rediger PDF'er og billeder. Filer åbnes i vinduer i den kørende
    prev, som starter, hvis det er nødvendigt.

    Tilvalg:
      -h, --help     Vis denne hjælp
      -V, --version  Vis versionen

## Settings, continued

settings-language = Sprog
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Systemstandard: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Indtastningssprog
settings-input-language-system = Følg tastaturlayoutet
settings-input-language-note = Bestemmer, hvilken side et tomt tekstfelt starter i. Tekst, du skriver, beholder sin egen retning.

settings-appearance-system = System
settings-appearance-light = Lys
settings-appearance-dark = Mørk
settings-omarchy-accent = Brug Omarchys accentfarve
# $theme is the Omarchy theme's name.
settings-omarchy-note = Farverne er dannet ud fra accentfarven i “{ $theme }”.
settings-omarchy-none = Intet Omarchy-tema er aktivt.
settings-auto-hide = Skjul værktøjslinjen, når markøren forlader vinduet
settings-auto-hide-note = Værktøjslinjen svæver over dokumentet og glider væk, mens markøren er uden for vinduet.
settings-animations = Animationer
settings-animations-note = Glidende linjer og paneler, voksende dialoger og fjedrende knapper.
settings-animations-reduced = Slået fra, mens systemet beder om reduceret bevægelse.
settings-corner-radius = Hjørneradius
settings-corner-radius-note = Til dialoger og den svævende værktøjslinje.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Gennemsigtighed for overlay
settings-overlay-note = Hvor meget af siden der skinner igennem den svævende værktøjslinje.
settings-overlay-value = { $percent }%
settings-storage-signatures = Mappe til underskrifter
settings-storage-versions = Mappe til versionshistorik
settings-storage-bookmarks = Fil til bogmærker
settings-storage-apply = Anvend
settings-storage-choose = Vælg…
# $file is where the settings file is.
settings-storage-note = Filer, der allerede ligger et gammelt sted, bliver der; flyt dem, hvis du vil bruge dem fortsat. Indstillingerne for prev gemmes i { $file }.
settings-save-failed = Indstillingerne kunne ikke gemmes: { $error }
settings-no-location = Intet sted til indstillinger: HOME er ikke angivet
settings-full-path = Brug en fuld sti, f.eks. ~/Documents/prev.
settings-path-is-folder = { $path } er en mappe, ikke en fil.
settings-folder-missing = Mappen { $path } findes ikke. Opret den først, eller vælg en.
settings-path-is-file = { $path } er en fil, ikke en mappe.
settings-cannot-write = prev kan ikke skrive i { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Eksporter
# Section headings in the export dialog.
export-format = Format
export-quality = Kvalitet
export-size = Størrelse
# Button that goes on to choose where to save the export.
export-choose = Eksporter…
# Format choice; the format name stays as it is.
export-format-webp = WebP (uden tab)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = billede
# JPEG quality choices.
export-quality-low = Lav
export-quality-medium = Middel
export-quality-high = Høj
export-quality-best = Bedst
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Faktisk størrelse
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } pixels
# $error is the system's reason.
export-dialog-failed = Dialogen Gem kunne ikke vises: { $error }
# $path is where the file was saved.
export-done = { $path } er eksporteret
export-failed = Kunne ikke eksportere: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = eksporten stoppede

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Billeder med markeringer kan ikke redigeres. Eksporter for at beholde markeringerne, eller slet dem og luk markeringslinjen.

# Shown if a background task ends unexpectedly.
image-loading-stopped = indlæsningen stoppede
image-reverting-stopped = gendannelsen stoppede
image-rendering-stopped = gengivelsen stoppede
image-saving-stopped = lagringen stoppede
image-markup-stopped = markeringen stoppede
image-no-version-store = Intet sted at gemme versioner
image-revert-failed = Kunne ikke gendanne: { $error }
image-read-failed = { $path } kunne ikke læses: { $error }
image-keep-original-failed = Den oprindelige version kunne ikke bevares: { $error }
image-save-failed = { $path } kunne ikke gemmes: { $error }
image-markup-start-failed = Markeringen kunne ikke startes: { $error }
image-cannot-edit = Animationer og SVG-tegninger kan ikke redigeres.
image-cannot-mark-up = Der kan ikke tilføjes markeringer til animationer og SVG-tegninger.
image-mark-up-wait = Vent, til redigeringen er færdig, og tilføj derefter markeringer.
image-crop-needs-selection = Træk først en markering (værktøjet Vælg), og beskær derefter.
image-size-needed = Angiv en bredde og højde i pixels.
# $name is a file name.
image-cannot-save-format = Ændringer i “{ $name }” kan ikke gemmes i filens format. Brug Eksporter ({ $keys }).
image-cannot-save-format-unbound = Ændringer i “{ $name }” kan ikke gemmes i filens format. Brug Eksporter.
image-cannot-export-animation = Animationer kan endnu ikke eksporteres.
image-drop-pages = Sider kan slippes på et dokument.
image-drag-failed = Trækningen kunne ikke startes.
image-open-failed = prev kan ikke åbne dette billede
image-opening = Åbner…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Navnet passer ikke til formatet
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” bliver gemt som en { $format }-fil, men navnet slutter på .{ $extension }. Andre apps kan muligvis ikke åbne den.
image-name-mismatch-no-extension = “{ $name }” bliver gemt som en { $format }-fil, men navnet har ingen filtype. Andre apps kan muligvis ikke åbne den.
image-choose-again = Vælg igen
image-save-as-is = Gem som den er
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = enkeltbillede { $current } af { $total }
image-position = { $current } af { $total }
image-edited = redigeret
# Toolbar tooltips.
image-sidebar = Sidepanel
image-zoom-out = Zoom ud
image-zoom-in = Zoom ind
image-zoom = { $percent }%
image-fit = Tilpas til vindue
image-actual-size = Faktisk størrelse
image-undo = Fortryd
image-redo = Gentag
image-rotate-left = Roter mod venstre
image-rotate-right = Roter mod højre
image-flip-horizontal = Spejlvend vandret
image-flip-vertical = Spejlvend lodret
image-select = Rektangulær markering
image-crop = Beskær til markering
image-adjust-size-tool = Juster størrelse
image-adjust-color-tool = Juster farve
# Tooltip and panel title.
image-inspector = Inspektør
image-markup = Markering
image-export = Eksporter
image-settings = Indstillinger
# Panel titles.
image-adjust-color = Juster farve
image-adjust-size = Juster størrelse
# Adjust Color sliders.
image-exposure = Eksponering
image-contrast = Kontrast
image-saturation = Mætning
image-temperature = Temperatur
image-tint = Nuance
image-sepia = Sepia
image-sharpness = Skarphed
image-levels = Niveauer
image-black-point = Sortpunkt
image-midtones = Mellemtoner
image-white-point = Hvidpunkt
image-reset-all = Nulstil alt
# Adjust Size panel.
image-current-size = Nuværende størrelse: { $width } × { $height } pixels
image-width = Bredde
image-height = Højde
image-scale-proportionally = Skaler proportionalt
# Button that applies the new size.
image-resize = Skift størrelse
# Inspector panel.
image-inspector-loading = Indlæser…
image-file = Fil
image-format = Format
image-dimensions-label = Dimensioner
image-pixels = { $width } × { $height } pixels
image-no-camera = Ingen kameraoplysninger.
image-location = Placering
image-remove-location = Fjern placeringsoplysninger
image-no-location = Ingen placeringsoplysninger.
image-keywords-description = Nøgleord og beskrivelse
image-keywords-hint = Nøgleord adskilt af kommaer
image-description = Beskrivelse
image-keywords-unsupported = Nøgleord kan gemmes i JPEG-, PNG- og WebP-filer.
# Heading over the earlier versions of the file.
image-revert-to = Gendan til
image-no-versions = Ingen tidligere versioner.
image-revert = Gendan
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Luk uden at eksportere markeringerne?
image-close-body = { $count ->
    [one] Markeringer på et billede findes kun, mens dets vindue er åbent. Eksporter billedet for at beholde dem: Markeringerne tegnes ind i den kopi, du gemmer.
   *[other] Markeringer på billeder findes kun, mens deres vindue er åbent. Eksporter hvert billede for at beholde dem: Markeringerne tegnes ind i den kopi, du gemmer.
}
image-close-anyway = Luk alligevel

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = læsningen stoppede
markdown-read-failed = prev kan ikke læse denne fil
markdown-draw-failed = Dokumentet kunne ikke tegnes
# Under the export's size choices.
markdown-export-size = Hele dokumentet, { $width } × { $height } pixels
# Search results.
markdown-not-found = Ikke fundet
markdown-match = { $current } af { $total }
# Placeholder of the search field.
markdown-search = Søg
# Toolbar tooltips.
markdown-smaller-text = Mindre tekst
markdown-larger-text = Større tekst
markdown-zoom = { $percent }%
markdown-actual-size = Faktisk størrelse
# Tooltip and panel title.
markdown-inspector = Inspektør
markdown-export = Eksporter
markdown-settings = Indstillinger
# Inspector headings and labels.
markdown-file = Fil
markdown-document = Dokument
markdown-words = Ord
markdown-lines = Linjer
markdown-pictures = Billeder

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Kamera
image-meta-exposure = Eksponering
image-meta-image = Billede
image-meta-make = Mærke
image-meta-model = Model
image-meta-lens = Objektiv
image-meta-exposure-time = Lukkertid
# The lens aperture, written like f/2.8.
image-meta-f-number = Blænde
image-meta-iso = ISO
image-meta-focal-length = Brændvidde
image-meta-exposure-bias = Eksponeringskorrektion
image-meta-flash = Blitz
image-meta-date-taken = Optagelsesdato
image-meta-orientation = Retning
image-meta-color-space = Farverum
image-meta-software = Software
image-meta-artist = Fotograf
image-meta-copyright = Ophavsret
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Spejlvendt vandret
    [3] Roteret 180°
    [4] Spejlvendt lodret
    [5] Spejlvendt vandret, roteret 90° mod uret
    [6] Roteret 90° med uret
    [7] Spejlvendt vandret, roteret 90° med uret
    [8] Roteret 90° mod uret
   *[other] Ukendt ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Udløst
   *[no] Ikke udløst
}{ $mode ->
    [on] , tvunget til
    [off] , fra
    [auto] , auto
   *[unknown] {""}
}{ $redeye ->
    [yes] , rødøjereduktion
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Ukalibreret
   *[other] Andet ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = dokumentet kan ikke åbnes: { $detail }
error-pdf-page-out-of-range = side { $page } findes ikke
error-pdf-password-protected = dokumentet er beskyttet med en adgangskode; åbn det, og kopier dets sider i stedet
error-pdf-no-pages = ingen sider at udtrække
error-pdf-crop-outside = beskæringsområdet ligger uden for siden
error-pdf-closed = dokumentet er lukket
error-pdf-saved-unreadable = det gemte dokument kan ikke længere åbnes
error-image-read = filen kan ikke læses: { $detail }
error-image-invalid = billedet er beskadiget eller ugyldigt: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = dette format kræver { $library }, som ikke er installeret
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format }-billeder understøttes ikke endnu
error-image-encode = billedet kan ikke kodes: { $detail }
error-exif-malformed = EXIF-dataene er ugyldige
error-settings-read = indstillingerne kan ikke læses: { $detail }
error-settings-invalid = ugyldige indstillinger: { $detail }
error-remove-location = placeringen kunne ikke fjernes: { $error }
error-location-unsupported = placeringsoplysninger kan fjernes fra JPEG-, PNG-, WebP- og TIFF-filer
error-xmp-unsupported = nøgleord og beskrivelser kan kun gemmes i JPEG-, PNG- og WebP-filer

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = Kamera-RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = Om prev
menu-settings = Indstillinger…
menu-services = Tjenester
menu-hide = Skjul prev
menu-hide-others = Skjul andre
menu-show-all = Vis alle
menu-quit = Slut prev
menu-file = Arkiv
menu-open = Åbn…
menu-close = Luk vindue
menu-export = Eksporter…
menu-print = Udskriv…
menu-edit = Rediger
menu-undo = Fortryd
menu-redo = Gentag
menu-cut = Klip
menu-copy = Kopier
menu-paste = Indsæt
menu-select-all = Vælg alt
menu-find = Find
menu-find-next = Find næste
menu-find-previous = Find forrige
menu-view = Oversigt
menu-hide-sidebar = Skjul sidepanel
menu-thumbnails = Miniaturer
menu-contents = Indholdsfortegnelse
menu-notes = Fremhævninger og noter
menu-bookmarks = Bogmærker
menu-zoom-in = Zoom ind
menu-zoom-out = Zoom ud
menu-actual-size = Faktisk størrelse
menu-zoom-to-fit = Zoom til tilpasning
menu-inspector = Vis inspektør
menu-slideshow = Lysbilledshow
menu-full-screen = Slå fuld skærm til
menu-go = Gå
menu-next-page = Næste side
menu-previous-page = Forrige side
menu-go-to-page = Gå til side…
menu-bookmark = Tilføj bogmærke
menu-tools = Værktøjer
menu-markup = Vis markeringslinje
menu-rotate-left = Roter mod venstre
menu-rotate-right = Roter mod højre
menu-crop = Beskær
menu-adjust-color = Juster farve…
menu-window = Vindue
menu-minimize = Minimer
menu-zoom = Zoom
menu-bring-all-to-front = Anbring alle forrest
