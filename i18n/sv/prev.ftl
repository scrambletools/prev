# prev's interface text in Swedish (Svenska), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = märkning, note = anteckning, highlight = överstrykning,
# annotation = annotering, redact/redaction = svärta/svärtning, inspector = inspektör,
# selection = markering, bookmark = bokmärke, page = sida.

## Language

language-name = Svenska

## Common

common-cancel = Avbryt
common-close = Stäng
common-save = Spara

## Settings

settings-title = Inställningar
settings-appearance = Utseende
settings-colors = Färger
settings-windows = Fönster
settings-default-app = Standardapp
settings-default-app-label = Öppna filer med prev
settings-default-app-note = Gör prev till appen som öppnar PDF-filer, bilder, SVG-teckningar och Markdown-filer.
settings-default-app-note-windows = I Windows kan du bara välja standardappar i systemets egna Inställningar. Det här öppnar prevs sida där.
settings-default-app-note-macos = macOS ber dig bekräfta varje typ: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP och AVIF.
settings-default-app-status = { $set } av { $total } filtyper öppnas med prev.
settings-default-app-button = Gör till standard
settings-default-app-button-windows = Öppna Inställningar
settings-default-app-no-entry = prevs skrivbordsfil är inte installerad, så systemet kan inte öppna filer med prev. Installera prev från ett paket eller med scripts/install.sh.
settings-default-app-no-bundle = Öppna prev från prev.app för att göra den till standard.
settings-default-app-failed = Det gick inte att göra prev till standard: { $error }
settings-storage = Lagring
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (utvecklingsversion, { $build })

## Markup toolbar

markup-tool-select = Välj
markup-tool-area = Rektangulär markering
markup-tool-sketch = Skiss
markup-tool-draw = Rita
markup-tool-shapes = Former
markup-tool-text-box = Textruta
markup-tool-highlight = Överstrykning
markup-tool-note = Anteckning
markup-tool-sign = Signera
markup-tool-redact = Svärta
markup-apply = Verkställ
markup-apply-redactions = Verkställ svärtningar
markup-shape-style = Formstil
markup-border-color = Kantfärg
markup-fill-color = Fyllnadsfärg
markup-text-style = Textstil
markup-delete = Radera
markup-undo = Ångra
markup-redo = Gör om

## Markup menus

markup-shape-rectangle = Rektangel
markup-shape-rounded-rectangle = Rundad rektangel
markup-shape-oval = Ellips
markup-shape-line = Linje
markup-shape-arrow = Pil
markup-shape-star = Stjärna
markup-shape-polygon = Månghörning
markup-shape-speech-bubble = Pratbubbla
markup-shape-loupe = Lupp
markup-shape-mask = Mask
markup-style-highlight = Överstrykning
markup-style-underline = Understrykning
markup-style-strikethrough = Genomstrykning
markup-style-squiggly = Vågig linje
markup-menu-color = Färg
markup-menu-font = Typsnitt
markup-menu-size = Storlek
markup-menu-alignment = Justering
markup-line-width = { $width } pt
markup-dashed = Streckad

## Notes

markup-kind-note = Anteckning
markup-kind-text-box = Textruta
markup-kind-stamp = Stämpel
markup-kind-redaction = Svärtning
markup-kind-shape = Form
markup-note-delete = Radera anteckning
markup-note-done = Klar
markup-note-placeholder = Skriv en anteckning
markup-notes-empty = Inga överstrykningar eller anteckningar
markup-notes-empty-hint = Överstrykningar, anteckningar och textrutor visas här.
markup-notes-page = Sida { $page }

## Markup errors

markup-change-failed = Det gick inte att ändra dokumentet: { $error }
markup-copy-area-failed = Det gick inte att kopiera området: { $error }
markup-document-closed = dokumentet stängdes
markup-render-area-failed = det gick inte att återge området
markup-copy-stopped = kopieringen avbröts

## Signatures

signature-menu-empty = Inga signaturer ännu.
signature-delete = Radera signatur
signature-create = Skapa signatur…
signature-dialog-title = Skapa signatur
signature-tab-draw = Rita
signature-tab-type = Skriv
signature-tab-image = Bild
signature-draw-hint = Signera på linjen med musen, pennan eller styrplattan.
signature-your-name = Ditt namn
signature-image-hint = Välj ett foto eller en skanning av din signatur på vitt papper.
signature-choose-image = Välj bild…
signature-description = Beskrivning, till exempel Fullständigt namn eller Initialer
signature-clear = Rensa
signature-ink = Bläck
signature-thickness = Tjocklek
signature-sign-first = Signera först och spara sedan.
signature-default-name = Signatur { $number }
signature-change-failed = Det gick inte att ändra signaturerna: { $error }
signature-no-data-folder = ingen datamapp: HOME är inte angivet
signature-removing-stopped = borttagningen avbröts
signature-saving-stopped = sparandet avbröts
signature-reading-stopped = läsningen avbröts
signature-not-an-image = filen är inte en bild som prev kan läsa
signature-no-frames = bilden har inga bildrutor
signature-not-found = ingen signatur hittades i bilden

## Dragging

drag-pages-need-document = Sidor kan släppas på ett dokument.
drag-image-unsupported = prev kan inte öppna den här bilden.
drag-area-failed = Det gick inte att dra området: { $error }
drag-pages-failed = Det gick inte att dra sidorna: { $error }
drag-start-failed = Det gick inte att börja dra.
drag-file-pages = Sidor
drag-file-one-page = { $name } (sida { $page })
drag-file-page-range = { $name } (sidorna { $first }–{ $last })
drag-file-image = Bild
drop-pdf-title = Lägga till i det här dokumentet?
drop-pdf-body = Lägga till ”{ $name }” i slutet av det här dokumentet eller öppna den i ett eget fönster?
drop-pdfs-body = { $count ->
    [one] Lägga till den här PDF-filen i slutet av det här dokumentet eller öppna den i ett eget fönster?
   *[other] Lägga till de här { $count } PDF-filerna i slutet av det här dokumentet eller öppna dem i egna fönster?
}
drop-pdf-add = Lägg till i slutet
drop-pdf-open = Öppna separat

## PDF window

pdf-opening = Öppnar…
pdf-open-failed = prev kan inte öppna det här dokumentet
pdf-no-pages = Dokumentet har inga sidor.
pdf-document-closed = dokumentet stängdes
pdf-keep-original-failed = det gick inte att behålla originalversionen: { $error }
pdf-save-failed = Det gick inte att spara: { $error }
pdf-nothing-to-paste = Det finns inget att klistra in.
pdf-pasting-stopped = inklistringen avbröts
pdf-file-dialog-failed = Det gick inte att visa fildialogen: { $error }
pdf-bookmarks-no-home = Bokmärken kan inte sparas: HOME är inte angivet
pdf-bookmarks-save-failed = Det gick inte att spara bokmärkena: { $error }
pdf-bookmark-page = Sida { $page }

pdf-password-protected = ”{ $name }” är lösenordsskyddad
pdf-password = Lösenord
pdf-password-wrong = Fel lösenord. Försök igen.
pdf-unlock = Lås upp

pdf-sidebar = Sidofält
pdf-page-of = av { $count }
pdf-zoom-out = Zooma ut
pdf-zoom-in = Zooma in
pdf-zoom-percent = { $percent } %
pdf-fit-page = Anpassa till sida
pdf-fit-width = Anpassa till bredd
pdf-actual-size = Verklig storlek
pdf-view-continuous = Kontinuerlig rullning
pdf-view-single-page = En sida
pdf-view-two-pages = Två sidor
pdf-undo = Ångra
pdf-redo = Gör om
pdf-rotate-left = Rotera åt vänster
pdf-rotate-right = Rotera åt höger
pdf-inspector = Inspektör
pdf-markup = Märkning
pdf-export = Exportera
pdf-settings = Inställningar

pdf-search = Sök
pdf-search-not-found = Hittades inte
pdf-searching = Söker…
pdf-search-match = { $current } av { $total }
pdf-search-match-more = { $current } av { $total }+

pdf-inspector-file = Fil
pdf-inspector-document = Dokument
pdf-inspector-pages = Sidor
pdf-inspector-title = Titel
pdf-inspector-author = Författare
pdf-inspector-subject = Ämne
pdf-inspector-keywords = Nyckelord
pdf-inspector-created = Skapad
pdf-inspector-modified = Ändrad
pdf-inspector-application = Program
pdf-inspector-producer = PDF-producent
pdf-inspector-version = Version
pdf-inspector-security = Säkerhet
pdf-inspector-not-encrypted = Inte krypterad
pdf-inspector-encrypted = Krypterad ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } sida
   *[other] { $count } sidor
}
pdf-inspector-page-size = Sidstorlek
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } tum)
pdf-loading = Läser in…

pdf-tab-pages = Sidor
pdf-tab-contents = Innehåll
pdf-tab-notes = Överstrykningar och anteckningar
pdf-tab-bookmarks = Bokmärken
pdf-no-outline = Ingen innehållsförteckning
pdf-no-outline-detail = Det här dokumentet har ingen disposition.
pdf-no-bookmarks = Inga bokmärken
pdf-no-bookmarks-detail = Tryck på { $keys } för att lägga till ett bokmärke för en sida.
pdf-no-bookmarks-detail-unbound = Sidor med bokmärken visas här.
pdf-remove-bookmark = Ta bort bokmärke

## Page editing

pages-menu = Sidor
pages-insert-blank = Infoga tom sida
pages-insert-file = Infoga från fil…
pages-copy = { $count ->
    [one] Kopiera sida
   *[other] Kopiera sidor
}
pages-paste = { $count ->
    [one] Klistra in sida
   *[other] Klistra in { $count } sidor
}
pages-crop = Beskär till markering
pages-select-all = Markera alla sidor
pages-delete = { $count ->
    [one] Radera sida
   *[other] Radera sidor
}
pages-apply-redactions = Verkställ svärtningar…
pages-no-copied = Det finns inga kopierade sidor att klistra in.
pages-copied = { $count ->
    [one] { $count } sida kopierades.
   *[other] { $count } sidor kopierades.
}
pages-copy-failed = Det gick inte att kopiera sidorna: { $error }
pages-reading-stopped = läsningen avbröts
pages-image-unreadable = är inte en bild som prev kan läsa
pages-read-failed = Det gick inte att läsa filen: { $error }
pages-at-least-one = Ett dokument måste ha minst en sida.
pages-crop-needs-area = Välj först ett område med verktyget för rektangulär markering.
pages-change-failed = Det gick inte att ändra sidorna: { $error }
pages-no-redactions = Det fanns inga svärtningar att verkställa.
pages-redactions-applied = { $count ->
    [one] { $count } svärtning verkställdes.
   *[other] { $count } svärtningar verkställdes.
}
pages-forget-versions-failed = Det gick inte att radera tidigare versioner: { $error }
pages-redact-title = Verkställa svärtningar?
pages-redact-body = { $count ->
    [one] Text, bilder och teckningar under svärtningen tas bort permanent från dokumentet, och svärtningen blir en svart ruta. Det går inte att ångra, och de tidigare versioner av filen som prev sparar raderas.
   *[other] Text, bilder och teckningar under de { $count } svärtningarna tas bort permanent från dokumentet, och svärtningarna blir svarta rutor. Det går inte att ångra, och de tidigare versioner av filen som prev sparar raderas.
}
pages-redact-apply = Verkställ

## PDF export

pages-export-title = Exportera
pages-export-format = Filformat
pages-export-reduce = Minska filstorleken (bilder i 150 dpi)
pages-export-flatten = Platta till annoteringar och formulärfält
pages-export-flatten-detail = Märkning och ifyllda fält blir en del av sidorna och kan inte längre redigeras. Svärtningar som inte har verkställts utelämnas.
pages-export-encrypt = Kryptera med lösenord
pages-export-password = Lösenord
pages-export-verify-password = Bekräfta lösenord
pages-export-resolution = Upplösning
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kvalitet
pages-export-quality-low = Låg
pages-export-quality-medium = Medel
pages-export-quality-high = Hög
pages-export-quality-best = Bäst
pages-export-one-file = Alla sidor hamnar i en fil.
pages-export-file-per-page = Varje sida sparas som en egen fil med namnet du väljer följt av ett nummer.
pages-export-selected-only = { $count ->
    [one] Endast den markerade sidan
   *[other] Endast de { $count } markerade sidorna
}
pages-export-choose = Exportera…
pages-export-no-password = Ange ett lösenord.
pages-export-password-mismatch = Lösenorden stämmer inte överens.
pages-export-file-name = { $name } (exporterad)
pages-export-untitled = dokument
pages-export-same-file = Exportera till en ny fil; det här dokumentet sparas automatiskt.
pages-export-exporting = Exporterar ”{ $name }”…
pages-export-done = ”{ $name }” har exporterats.
pages-export-done-images = { $count } bilder har exporterats.
pages-export-failed = Det gick inte att exportera: { $error }
pages-export-stopped = exporten avbröts

## Start window

app-start-hint = Öppna eller släpp en PDF-, bild-, SVG- eller Markdown-fil.
app-start-open = Öppna…
app-title-dev = { $title } (utveckling)
app-viewer-missing = { $kind }: den här visaren är inte byggd ännu.
app-cannot-open = prev kan inte öppna den här typen av fil.
app-cannot-read = prev kan inte läsa den här filen: { $error }
app-kind-pdf = PDF-dokument
app-kind-image = { $format }-bild
app-kind-svg = SVG-teckning
app-kind-markdown = Markdown-dokument
app-file-dialog-failed = Det gick inte att visa fildialogen: { $error }

## Actions

action-open = Öppna
action-settings = Inställningar

## Toolbar

app-toolbar-keep-shown = Visa alltid verktygsfältet
app-toolbar-auto-hide = Göm verktygsfältet när pekaren lämnar fönstret
app-toolbar-more = Mer

## File facts

app-fact-name = Namn
app-fact-folder = Mapp
app-fact-size = Storlek
app-fact-modified = Ändrad
app-size-bytes = { $count } byte
app-size-kb = { $size } kB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Ogiltig länk { $uri }: { $error }
app-link-open-failed = Det gick inte att öppna { $uri }: { $error }
app-paste-needs-wl-clipboard = installera wl-clipboard för att klistra in bilder
app-copy-needs-wl-clipboard = installera wl-clipboard för att kopiera bilder
app-copy-no-pixels = området har inga pixlar
app-copy-no-input = wl-copy fick ingen indata
app-copy-failed = wl-copy misslyckades
app-clipboard-open-failed = Det gick inte att öppna Urklipp: { $error }
app-copy-image-failed = Det gick inte att kopiera bilden: { $error }

## Printing

print-failed = Det gick inte att skriva ut: { $error }
print-stopped = Utskriften avbröts
print-unavailable = Utskrift är inte tillgänglig på det här systemet ännu.
print-no-window = Det gick inte att skriva ut: det finns inget fönster att visa utskriftsdialogen över
print-dialog-failed = Det gick inte att visa utskriftsdialogen: { $error }
print-job-not-started = skrivaren startade inte jobbet
print-printer-stopped = skrivaren stannade

## File dialogs

dialog-open = Öppna
dialog-filter-all = Alla filer som stöds
dialog-filter-pdf = PDF-dokument
dialog-filter-images = Bilder
dialog-filter-svg = SVG-teckningar
dialog-filter-markdown = Markdown-filer
dialog-choose-signatures = Välj mappen för signaturer
dialog-choose-versions = Välj mappen för versionshistorik
dialog-choose-bookmarks = Välj filen för bokmärken

## Command line

usage-help =
    Användning: prev [FILE]...

    Visa och redigera PDF-filer och bilder. Filerna öppnas i fönster i den
    prev-instans som körs, och prev startas vid behov.

    Alternativ:
      -h, --help     Visa den här hjälpen
      -V, --version  Visa versionen

## Settings, continued

settings-language = Språk
settings-language-system = Systemstandard: { $language }
settings-input-language = Inmatningsspråk
settings-input-language-system = Följ tangentbordslayouten
settings-input-language-note = Anger vilken sida ett tomt textfält börjar på. Text du skriver behåller sin egen riktning.

settings-appearance-system = Följ systemet
settings-appearance-light = Ljust
settings-appearance-dark = Mörkt
settings-system-accent = Använd systemets accentfärg
settings-omarchy-note = Färgerna bygger på accentfärgen i ”{ $theme }”.
settings-system-accent-note = Färgerna bygger på systemets accentfärg.
settings-system-accent-none = Systemet har ingen accentfärg, så prev använder den som är vald nedan.
settings-accent-chosen-note = Färgerna bygger på färgen som är vald nedan.
settings-auto-hide = Göm verktygsfältet när pekaren lämnar fönstret
settings-auto-hide-note = Verktygsfältet svävar över dokumentet och glider undan medan pekaren är utanför fönstret.
settings-animations = Animeringar
settings-animations-note = Glidande fält och paneler, växande dialogrutor och fjädrande knappar.
settings-animations-reduced = Av så länge systemet begär minskad rörelse.
settings-corner-radius = Hörnradie
settings-corner-radius-note = För dialogrutor och det svävande verktygsfältet.
settings-corner-radius-value = { $radius } px
settings-overlay = Genomskinlighet för överlägg
settings-overlay-note = Hur mycket av sidan som syns genom det svävande verktygsfältet.
settings-overlay-value = { $percent } %
settings-storage-signatures = Mapp för signaturer
settings-storage-versions = Mapp för versionshistorik
settings-storage-bookmarks = Fil för bokmärken
settings-storage-apply = Verkställ
settings-storage-choose = Välj…
settings-storage-note = Filer som redan finns på en gammal plats ligger kvar där; flytta dem om du vill fortsätta använda dem. Inställningarna för prev sparas i { $file }.
settings-save-failed = Det gick inte att spara inställningarna: { $error }
settings-no-location = Ingen plats för inställningar: HOME är inte angivet
settings-full-path = Använd en fullständig sökväg, till exempel ~/Documents/prev.
settings-path-is-folder = { $path } är en mapp, inte en fil.
settings-folder-missing = Mappen { $path } finns inte. Skapa den först eller välj en annan.
settings-path-is-file = { $path } är en fil, inte en mapp.
settings-cannot-write = prev kan inte skriva i { $path }: { $error }.

## Export dialog

export-title = Exportera
export-format = Filformat
export-quality = Kvalitet
export-size = Storlek
export-choose = Exportera…
export-format-webp = WebP (förlustfri)
export-format-unknown = bild
export-quality-low = Låg
export-quality-medium = Medel
export-quality-high = Hög
export-quality-best = Bäst
export-size-actual = Verklig storlek
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } pixlar
export-dialog-failed = Det gick inte att visa dialogrutan Spara: { $error }
export-done = { $path } har exporterats
export-failed = Det gick inte att exportera: { $error }
export-stopped = exporten avbröts

## Image window

image-marked-no-edit = Bilder med märkning kan inte redigeras. Exportera för att behålla märkningen, eller radera den och stäng märkningsfältet.

image-loading-stopped = inläsningen avbröts
image-reverting-stopped = återställningen avbröts
image-rendering-stopped = återgivningen avbröts
image-saving-stopped = sparandet avbröts
image-markup-stopped = märkningen avbröts
image-no-version-store = Ingen plats att spara versioner på
image-revert-failed = Det gick inte att återställa: { $error }
image-read-failed = Det gick inte att läsa { $path }: { $error }
image-keep-original-failed = Det gick inte att behålla originalversionen: { $error }
image-save-failed = Det gick inte att spara { $path }: { $error }
image-markup-start-failed = Det gick inte att starta märkningen: { $error }
image-cannot-edit = Animeringar och SVG-teckningar kan inte redigeras.
image-cannot-mark-up = Det går inte att lägga till märkning i animeringar och SVG-teckningar.
image-mark-up-wait = Vänta tills redigeringen är klar och lägg sedan till märkning.
image-crop-needs-selection = Dra först en markering (verktyget Välj) och beskär sedan.
image-size-needed = Ange bredd och höjd i pixlar.
image-cannot-save-format = Ändringar i ”{ $name }” kan inte sparas i filens format. Använd Exportera ({ $keys }).
image-cannot-save-format-unbound = Ändringar i ”{ $name }” kan inte sparas i filens format. Använd Exportera.
image-cannot-export-animation = Animeringar kan inte exporteras ännu.
image-drop-pages = Sidor kan släppas på ett dokument.
image-drag-failed = Det gick inte att börja dra.
image-picture-save-failed = Det gick inte att spara bilden i mappen Hämtade filer.
image-open-failed = prev kan inte öppna den här bilden
image-opening = Öppnar…
image-name-mismatch-title = Namnet stämmer inte med formatet
image-name-mismatch = ”{ $name }” sparas som en { $format }-fil, men namnet slutar på .{ $extension }. Andra appar kanske inte kan öppna den.
image-name-mismatch-no-extension = ”{ $name }” sparas som en { $format }-fil, men namnet saknar filändelse. Andra appar kanske inte kan öppna den.
image-choose-again = Välj igen
image-save-as-is = Spara som den är
image-dimensions = { $width } × { $height }
image-frame-position = bildruta { $current } av { $total }
image-position = { $current } av { $total }
image-edited = redigerad
image-sidebar = Sidofält
image-zoom-out = Zooma ut
image-zoom-in = Zooma in
image-zoom = { $percent } %
image-fit = Anpassa till fönster
image-actual-size = Verklig storlek
image-undo = Ångra
image-redo = Gör om
image-rotate-left = Rotera åt vänster
image-rotate-right = Rotera åt höger
image-flip-horizontal = Vänd vågrätt
image-flip-vertical = Vänd lodrätt
image-select = Rektangulär markering
image-crop = Beskär till markering
image-adjust-size-tool = Justera storlek
image-adjust-color-tool = Justera färg
image-inspector = Inspektör
image-markup = Märkning
image-export = Exportera
image-settings = Inställningar
image-adjust-color = Justera färg
image-adjust-size = Justera storlek
image-exposure = Exponering
image-contrast = Kontrast
image-saturation = Mättnad
image-temperature = Temperatur
image-tint = Färgton
image-sepia = Sepiaton
image-sharpness = Skärpa
image-levels = Nivåer
image-black-point = Svartpunkt
image-midtones = Mellantoner
image-white-point = Vitpunkt
image-reset-all = Återställ alla
image-current-size = Aktuell storlek: { $width } × { $height } pixlar
image-width = Bredd
image-height = Höjd
image-scale-proportionally = Skala proportionellt
image-resize = Ändra storlek
image-inspector-loading = Läser in…
image-file = Fil
image-format = Filformat
image-dimensions-label = Mått
image-pixels = { $width } × { $height } pixlar
image-no-camera = Ingen kamerainformation.
image-location = Plats
image-remove-location = Ta bort platsinformation
image-no-location = Ingen platsinformation.
image-keywords-description = Nyckelord och beskrivning
image-keywords-hint = Nyckelord, åtskilda med kommatecken
image-description = Beskrivning
image-keywords-unsupported = Nyckelord kan sparas i JPEG-, PNG- och WebP-filer.
image-revert-to = Återställ till
image-no-versions = Inga tidigare versioner.
image-revert = Återställ
image-size-kb = { $size } kB
image-size-mb = { $size } MB
image-close-title = Stänga utan att exportera märkningen?
image-close-body = { $count ->
    [one] Märkning på en bild finns bara kvar medan fönstret är öppet. Exportera bilden för att behålla den: märkningen ritas in i kopian du sparar.
   *[other] Märkning på bilder finns bara kvar medan fönstret är öppet. Exportera varje bild för att behålla den: märkningen ritas in i kopian du sparar.
}
image-close-anyway = Stäng ändå

## Markdown

markdown-reading-stopped = läsningen avbröts
markdown-read-failed = prev kan inte läsa den här filen
markdown-draw-failed = Det gick inte att rita dokumentet
markdown-export-size = Hela dokumentet, { $width } × { $height } pixlar
markdown-not-found = Hittades inte
markdown-match = { $current } av { $total }
markdown-search = Sök
markdown-smaller-text = Mindre text
markdown-larger-text = Större text
markdown-zoom = { $percent } %
markdown-actual-size = Verklig storlek
markdown-inspector = Inspektör
markdown-export = Exportera
markdown-settings = Inställningar
markdown-file = Fil
markdown-document = Dokument
markdown-words = Ord
markdown-lines = Rader
markdown-pictures = Bilder

## Image details

image-meta-camera = Kamera
image-meta-exposure = Exponering
image-meta-image = Bild
image-meta-make = Tillverkare
image-meta-model = Modell
image-meta-lens = Objektiv
image-meta-exposure-time = Exponeringstid
image-meta-f-number = Bländartal
image-meta-iso = ISO
image-meta-focal-length = Brännvidd
image-meta-exposure-bias = Exponeringskompensation
image-meta-flash = Blixt
image-meta-date-taken = Fotodatum
image-meta-orientation = Orientering
image-meta-color-space = Färgrymd
image-meta-software = Programvara
image-meta-artist = Fotograf
image-meta-copyright = Upphovsrätt
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Spegelvänd vågrätt
    [3] Roterad 180°
    [4] Spegelvänd lodrätt
    [5] Spegelvänd vågrätt, roterad 90° moturs
    [6] Roterad 90° medurs
    [7] Spegelvänd vågrätt, roterad 90° medurs
    [8] Roterad 90° moturs
   *[other] Okänd ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Utlöstes
   *[no] Utlöstes inte
}{ $mode ->
    [on] , påtvingad
    [off] , av
    [auto] , automatisk
   *[unknown] {""}
}{ $redeye ->
    [yes] , reducering av röda ögon
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Okalibrerad
   *[other] Annan ({ $code })
}

## Errors

error-pdf-open = det går inte att öppna dokumentet: { $detail }
error-pdf-page-out-of-range = sidan { $page } finns inte
error-pdf-password-protected = dokumentet är lösenordsskyddat; öppna det och kopiera sidorna i stället
error-pdf-no-pages = inga sidor att extrahera
error-pdf-crop-outside = beskärningsområdet ligger utanför sidan
error-pdf-closed = dokumentet är stängt
error-pdf-saved-unreadable = det sparade dokumentet går inte längre att öppna
error-image-read = det går inte att läsa filen: { $detail }
error-image-invalid = bilden är skadad eller ogiltig: { $detail }
error-image-missing-library = det här formatet kräver { $library }, som inte är installerat
error-image-unsupported = { $format }-bilder stöds inte ännu
error-image-encode = det går inte att koda bilden: { $detail }
error-exif-malformed = EXIF-data är felaktiga
error-settings-read = det går inte att läsa inställningarna: { $detail }
error-settings-invalid = ogiltiga inställningar: { $detail }
error-remove-location = det gick inte att ta bort platsen: { $error }
error-location-unsupported = platsinformation kan tas bort från JPEG-, PNG-, WebP- och TIFF-filer
error-xmp-unsupported = nyckelord och beskrivningar kan bara sparas i JPEG-, PNG- och WebP-filer

## Formats

format-camera-raw = Kamera-RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = Om prev
menu-settings = Inställningar…
menu-services = Tjänster
menu-hide = Göm prev
menu-hide-others = Göm övriga
menu-show-all = Visa alla
menu-quit = Avsluta prev
menu-file = Arkiv
menu-open = Öppna…
menu-close = Stäng fönster
menu-export = Exportera…
menu-print = Skriv ut…
menu-edit = Redigera
menu-undo = Ångra
menu-redo = Gör om
menu-cut = Klipp ut
menu-copy = Kopiera
menu-paste = Klistra in
menu-select-all = Markera allt
menu-find = Sök
menu-find-next = Sök nästa
menu-find-previous = Sök föregående
menu-view = Innehåll
menu-hide-sidebar = Göm sidofält
menu-thumbnails = Miniatyrer
menu-contents = Innehållsförteckning
menu-notes = Överstrykningar och anteckningar
menu-bookmarks = Bokmärken
menu-zoom-in = Zooma in
menu-zoom-out = Zooma ut
menu-actual-size = Verklig storlek
menu-zoom-to-fit = Zooma till passande storlek
menu-inspector = Visa inspektör
menu-slideshow = Bildspel
menu-full-screen = Använd helskärm
menu-go = Gå
menu-next-page = Nästa sida
menu-previous-page = Föregående sida
menu-go-to-page = Gå till sida…
menu-bookmark = Lägg till bokmärke
menu-tools = Verktyg
menu-markup = Visa verktygsfältet Märkning
menu-rotate-left = Rotera åt vänster
menu-rotate-right = Rotera åt höger
menu-crop = Beskär
menu-adjust-color = Justera färg…
menu-window = Fönster
menu-minimize = Minimera
menu-zoom = Zooma
menu-bring-all-to-front = Lägg alla överst
