# prev's interface text in Norwegian Bokmål (Norsk bokmål), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = markering, note = notat, highlight = utheving,
# annotation = merknad, redact/redaction = sladde/sladding, inspector = inspektør,
# selection = utvalg, bookmark = bokmerke, page = side.

## Language

language-name = Norsk bokmål

## Common

common-cancel = Avbryt
common-close = Lukk
common-save = Lagre

## Settings

settings-title = Innstillinger
settings-appearance = Utseende
settings-colors = Farger
settings-windows = Vinduer
settings-default-app = Standardapp
settings-default-app-label = Åpne filer med prev
settings-default-app-note = Gjør prev til appen som åpner PDF-er, bilder, SVG-tegninger og Markdown-filer.
settings-default-app-note-windows = I Windows kan du bare velge standardapper i systemets egne Innstillinger. Dette åpner prevs side der.
settings-default-app-note-macos = macOS ber deg bekrefte hver type: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP og AVIF.
settings-default-app-status = { $set } av { $total } filtyper åpnes med prev.
settings-default-app-button = Gjør til standard
settings-default-app-button-windows = Åpne Innstillinger
settings-default-app-no-entry = prevs skrivebordsfil er ikke installert, så systemet kan ikke åpne filer med prev. Installer prev fra en pakke eller med scripts/install.sh.
settings-default-app-no-bundle = Åpne prev fra prev.app for å gjøre den til standard.
settings-default-app-failed = Kunne ikke gjøre prev til standard: { $error }
settings-storage = Lagring
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (utviklerversjon, { $build })

## Markup toolbar

markup-tool-select = Velg
markup-tool-area = Rektangulært utvalg
markup-tool-sketch = Skisse
markup-tool-draw = Tegn
markup-tool-shapes = Figurer
markup-tool-text-box = Tekstboks
markup-tool-highlight = Uthev
markup-tool-note = Notat
markup-tool-sign = Signer
markup-tool-redact = Sladd
markup-apply = Bruk
markup-apply-redactions = Bruk sladding
markup-shape-style = Figurstil
markup-border-color = Kantfarge
markup-fill-color = Fyllfarge
markup-text-style = Tekststil
markup-delete = Slett
markup-undo = Angre
markup-redo = Gjør om

## Markup menus

markup-shape-rectangle = Rektangel
markup-shape-rounded-rectangle = Avrundet rektangel
markup-shape-oval = Ellipse
markup-shape-line = Linje
markup-shape-arrow = Pil
markup-shape-star = Stjerne
markup-shape-polygon = Mangekant
markup-shape-speech-bubble = Snakkeboble
markup-shape-loupe = Lupe
markup-shape-mask = Maske
markup-style-highlight = Utheving
markup-style-underline = Understreking
markup-style-strikethrough = Gjennomstreking
markup-style-squiggly = Bølgelinje
markup-menu-color = Farge
markup-menu-font = Skrift
markup-menu-size = Størrelse
markup-menu-alignment = Justering
markup-line-width = { $width } pkt
markup-dashed = Stiplet

## Notes

markup-kind-note = Notat
markup-kind-text-box = Tekstboks
markup-kind-stamp = Stempel
markup-kind-redaction = Sladding
markup-kind-shape = Figur
markup-note-delete = Slett notat
markup-note-done = Ferdig
markup-note-placeholder = Skriv et notat
markup-notes-empty = Ingen uthevinger eller notater
markup-notes-empty-hint = Uthevinger, notater og tekstbokser vises her.
markup-notes-page = Side { $page }

## Markup errors

markup-change-failed = Kunne ikke endre dokumentet: { $error }
markup-copy-area-failed = Kunne ikke kopiere området: { $error }
markup-document-closed = dokumentet ble lukket
markup-render-area-failed = kunne ikke gjengi området
markup-copy-stopped = kopieringen stoppet

## Signatures

signature-menu-empty = Ingen signaturer ennå.
signature-delete = Slett signatur
signature-create = Opprett signatur…
signature-dialog-title = Opprett signatur
signature-tab-draw = Tegn
signature-tab-type = Skriv
signature-tab-image = Bilde
signature-draw-hint = Signer på linjen med musen, pennen eller styreflaten.
signature-your-name = Navnet ditt
signature-image-hint = Velg et bilde eller en skanning av signaturen din på hvitt papir.
signature-choose-image = Velg bilde…
signature-description = Beskrivelse, for eksempel Fullt navn eller Initialer
signature-clear = Tøm
signature-ink = Blekk
signature-thickness = Tykkelse
signature-sign-first = Signer først, og lagre deretter.
signature-default-name = Signatur { $number }
signature-change-failed = Kunne ikke endre signaturene: { $error }
signature-no-data-folder = ingen datamappe: HOME er ikke angitt
signature-removing-stopped = fjerningen stoppet
signature-saving-stopped = lagringen stoppet
signature-reading-stopped = lesingen stoppet
signature-not-an-image = filen er ikke et bilde som prev kan lese
signature-no-frames = bildet har ingen bilderuter
signature-not-found = fant ingen signatur i bildet

## Dragging

drag-pages-need-document = Sider kan slippes på et dokument.
drag-image-unsupported = prev kan ikke åpne dette bildet.
drag-area-failed = Kunne ikke dra området: { $error }
drag-pages-failed = Kunne ikke dra sidene: { $error }
drag-start-failed = Kunne ikke begynne å dra.
drag-file-pages = Sider
drag-file-one-page = { $name } (side { $page })
drag-file-page-range = { $name } (sider { $first }–{ $last })
drag-file-image = Bilde
drop-pdf-title = Vil du legge til i dette dokumentet?
drop-pdf-body = Vil du legge til «{ $name }» på slutten av dette dokumentet, eller åpne den i et eget vindu?
drop-pdfs-body = { $count ->
    [one] Vil du legge til denne PDF-en på slutten av dette dokumentet, eller åpne den i et eget vindu?
   *[other] Vil du legge til disse { $count } PDF-ene på slutten av dette dokumentet, eller åpne dem i hvert sitt vindu?
}
drop-pdf-add = Legg til på slutten
drop-pdf-open = Åpne separat

## PDF window

pdf-opening = Åpner…
pdf-open-failed = prev kan ikke åpne dette dokumentet
pdf-no-pages = Dokumentet har ingen sider.
pdf-document-closed = dokumentet ble lukket
pdf-keep-original-failed = kunne ikke beholde originalversjonen: { $error }
pdf-save-failed = Kunne ikke lagre: { $error }
pdf-nothing-to-paste = Det er ingenting å lime inn.
pdf-pasting-stopped = innlimingen stoppet
pdf-file-dialog-failed = Kunne ikke vise fildialogen: { $error }
pdf-bookmarks-no-home = Bokmerker kan ikke lagres: HOME er ikke angitt
pdf-bookmarks-save-failed = Kunne ikke lagre bokmerkene: { $error }
pdf-bookmark-page = Side { $page }

pdf-password-protected = «{ $name }» er passordbeskyttet
pdf-password = Passord
pdf-password-wrong = Feil passord. Prøv igjen.
pdf-unlock = Lås opp

pdf-sidebar = Sidepanel
pdf-page-of = av { $count }
pdf-zoom-out = Zoom ut
pdf-zoom-in = Zoom inn
pdf-zoom-percent = { $percent } %
pdf-fit-page = Tilpass til side
pdf-fit-width = Tilpass til bredde
pdf-actual-size = Faktisk størrelse
pdf-view-continuous = Kontinuerlig rulling
pdf-view-single-page = Enkeltside
pdf-view-two-pages = To sider
pdf-undo = Angre
pdf-redo = Gjør om
pdf-rotate-left = Roter mot venstre
pdf-rotate-right = Roter mot høyre
pdf-inspector = Inspektør
pdf-markup = Markering
pdf-export = Eksporter
pdf-settings = Innstillinger

pdf-search = Søk
pdf-search-not-found = Ikke funnet
pdf-searching = Søker…
pdf-search-match = { $current } av { $total }
pdf-search-match-more = { $current } av { $total }+

pdf-inspector-file = Fil
pdf-inspector-document = Dokument
pdf-inspector-pages = Sider
pdf-inspector-title = Tittel
pdf-inspector-author = Forfatter
pdf-inspector-subject = Emne
pdf-inspector-keywords = Nøkkelord
pdf-inspector-created = Opprettet
pdf-inspector-modified = Endret
pdf-inspector-application = Program
pdf-inspector-producer = PDF-produsent
pdf-inspector-version = Versjon
pdf-inspector-security = Sikkerhet
pdf-inspector-not-encrypted = Ikke kryptert
pdf-inspector-encrypted = Kryptert ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } side
   *[other] { $count } sider
}
pdf-inspector-page-size = Sidestørrelse
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } tommer)
pdf-loading = Laster inn…

pdf-tab-pages = Sider
pdf-tab-contents = Innhold
pdf-tab-notes = Uthevinger og notater
pdf-tab-bookmarks = Bokmerker
pdf-no-outline = Ingen innholdsfortegnelse
pdf-no-outline-detail = Dette dokumentet har ingen disposisjon.
pdf-no-bookmarks = Ingen bokmerker
pdf-no-bookmarks-detail = Trykk på { $keys } for å sette et bokmerke på en side.
pdf-no-bookmarks-detail-unbound = Sider med bokmerker vises her.
pdf-remove-bookmark = Fjern bokmerke

## Page editing

pages-menu = Sider
pages-insert-blank = Sett inn tom side
pages-insert-file = Sett inn fra fil…
pages-copy = { $count ->
    [one] Kopier side
   *[other] Kopier sider
}
pages-paste = { $count ->
    [one] Lim inn side
   *[other] Lim inn { $count } sider
}
pages-crop = Beskjær til utvalg
pages-select-all = Marker alle sider
pages-delete = { $count ->
    [one] Slett side
   *[other] Slett sider
}
pages-apply-redactions = Bruk sladding…
pages-no-copied = Det er ingen kopierte sider å lime inn.
pages-copied = { $count ->
    [one] Kopierte { $count } side.
   *[other] Kopierte { $count } sider.
}
pages-copy-failed = Kunne ikke kopiere sidene: { $error }
pages-reading-stopped = lesingen stoppet
pages-image-unreadable = er ikke et bilde som prev kan lese
pages-read-failed = Kunne ikke lese filen: { $error }
pages-at-least-one = Et dokument må ha minst én side.
pages-crop-needs-area = Velg først et område med verktøyet for rektangulært utvalg.
pages-change-failed = Kunne ikke endre sidene: { $error }
pages-no-redactions = Det var ingen sladding å bruke.
pages-redactions-applied = { $count ->
    [one] Brukte { $count } sladding.
   *[other] Brukte { $count } sladdinger.
}
pages-forget-versions-failed = Kunne ikke slette tidligere versjoner: { $error }
pages-redact-title = Vil du bruke sladdingen?
pages-redact-body = { $count ->
    [one] Tekst, bilder og tegninger under merket fjernes fra dokumentet for godt, og merket blir en svart boks. Dette kan ikke angres, og de tidligere versjonene av filen som prev tar vare på, slettes.
   *[other] Tekst, bilder og tegninger under de { $count } merkene fjernes fra dokumentet for godt, og merkene blir svarte bokser. Dette kan ikke angres, og de tidligere versjonene av filen som prev tar vare på, slettes.
}
pages-redact-apply = Bruk

## PDF export

pages-export-title = Eksporter
pages-export-format = Filformat
pages-export-reduce = Reduser filstørrelsen (bilder i 150 dpi)
pages-export-flatten = Gjør merknader og skjemafelt flate
pages-export-flatten-detail = Markering og utfylte felt blir en del av sidene og kan ikke lenger redigeres. Sladding som ikke er brukt ennå, utelates.
pages-export-encrypt = Krypter med passord
pages-export-password = Passord
pages-export-verify-password = Bekreft passord
pages-export-resolution = Oppløsning
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kvalitet
pages-export-quality-low = Lav
pages-export-quality-medium = Middels
pages-export-quality-high = Høy
pages-export-quality-best = Best
pages-export-one-file = Alle sidene legges i én fil.
pages-export-file-per-page = Hver side lagres som en egen fil med navnet du velger etterfulgt av et nummer.
pages-export-selected-only = { $count ->
    [one] Bare den valgte siden
   *[other] Bare de { $count } valgte sidene
}
pages-export-choose = Eksporter…
pages-export-no-password = Skriv inn et passord.
pages-export-password-mismatch = Passordene samsvarer ikke.
pages-export-file-name = { $name } (eksportert)
pages-export-untitled = dokument
pages-export-same-file = Eksporter til en ny fil; dette dokumentet lagres automatisk.
pages-export-exporting = Eksporterer «{ $name }»…
pages-export-done = Eksporterte «{ $name }».
pages-export-done-images = Eksporterte { $count } bilder.
pages-export-failed = Kunne ikke eksportere: { $error }
pages-export-stopped = eksporten stoppet

## Start window

app-start-hint = Åpne eller slipp en PDF-, bilde-, SVG- eller Markdown-fil.
app-start-open = Åpne…
app-title-dev = { $title } (utvikling)
app-viewer-missing = { $kind }: denne visningen er ikke laget ennå.
app-cannot-open = prev kan ikke åpne denne filtypen.
app-cannot-read = prev kan ikke lese denne filen: { $error }
app-kind-pdf = PDF-dokument
app-kind-image = { $format }-bilde
app-kind-svg = SVG-tegning
app-kind-markdown = Markdown-dokument
app-file-dialog-failed = Kunne ikke vise fildialogen: { $error }

## Actions

action-open = Åpne
action-settings = Innstillinger

## Toolbar

app-toolbar-keep-shown = Vis alltid verktøylinjen
app-toolbar-auto-hide = Skjul verktøylinjen når pekeren forlater vinduet
app-toolbar-more = Mer

## File facts

app-fact-name = Navn
app-fact-folder = Mappe
app-fact-size = Størrelse
app-fact-modified = Endret
app-size-bytes = { $count } byte
app-size-kb = { $size } kB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Ugyldig lenke { $uri }: { $error }
app-link-open-failed = Kunne ikke åpne { $uri }: { $error }
app-paste-needs-wl-clipboard = installer wl-clipboard for å lime inn bilder
app-copy-needs-wl-clipboard = installer wl-clipboard for å kopiere bilder
app-copy-no-pixels = området har ingen piksler
app-copy-no-input = wl-copy fikk ingen inndata
app-copy-failed = wl-copy mislyktes
app-clipboard-open-failed = Kunne ikke åpne utklippstavlen: { $error }
app-copy-image-failed = Kunne ikke kopiere bildet: { $error }

## Printing

print-failed = Kunne ikke skrive ut: { $error }
print-stopped = Utskriften stoppet
print-unavailable = Utskrift er ikke tilgjengelig på dette systemet ennå.
print-no-window = Kunne ikke skrive ut: det finnes ikke noe vindu å vise utskriftsdialogen over
print-dialog-failed = Kunne ikke vise utskriftsdialogen: { $error }
print-job-not-started = skriveren startet ikke jobben
print-printer-stopped = skriveren stoppet

## File dialogs

dialog-open = Åpne
dialog-filter-all = Alle støttede filer
dialog-filter-pdf = PDF-dokumenter
dialog-filter-images = Bilder
dialog-filter-svg = SVG-tegninger
dialog-filter-markdown = Markdown-filer
dialog-choose-signatures = Velg mappen for signaturer
dialog-choose-versions = Velg mappen for versjonslogg
dialog-choose-bookmarks = Velg filen for bokmerker

## Command line

usage-help =
    Bruk: prev [FILE]...
          prev --mcp

    Vis og rediger PDF-er og bilder. Filene åpnes i vinduer i prev-instansen
    som kjører, og prev startes ved behov.

    Valg:
      -h, --help     Vis denne hjelpen
      -V, --version  Vis versjonen
          --mcp      Kjør MCP på stdin og stdout, så KI-agenter kan styre prev
                     som kjører

## Settings, continued

settings-language = Språk
settings-language-system = Systemstandard: { $language }
settings-input-language = Inndataspråk
settings-input-language-system = Følg tastaturoppsettet
settings-input-language-note = Bestemmer hvilken side et tomt tekstfelt starter på. Tekst du skriver, beholder sin egen retning.

settings-appearance-system = Følg systemet
settings-appearance-light = Lys
settings-appearance-dark = Mørk
settings-system-accent = Bruk aksentfargen fra systemet
settings-omarchy-note = Fargene bygger på aksentfargen i «{ $theme }».
settings-system-accent-note = Fargene bygger på systemets aksentfarge.
settings-system-accent-none = Systemet har ingen aksentfarge, så prev bruker den som er valgt nedenfor.
settings-accent-chosen-note = Fargene bygger på fargen som er valgt nedenfor.
settings-auto-hide = Skjul verktøylinjen når pekeren forlater vinduet
settings-auto-hide-note = Verktøylinjen svever over dokumentet og glir bort mens pekeren er utenfor vinduet.
settings-animations = Animasjoner
settings-animations-note = Glidende linjer og paneler, voksende dialogbokser og fjærende knapper.
settings-animations-reduced = Av så lenge systemet ber om redusert bevegelse.
settings-corner-radius = Hjørneradius
settings-corner-radius-note = For dialogbokser og den svevende verktøylinjen.
settings-corner-radius-value = { $radius } px
settings-overlay = Gjennomsiktighet for overlegg
settings-overlay-note = Hvor mye av siden som synes gjennom den svevende verktøylinjen.
settings-overlay-value = { $percent } %
settings-storage-signatures = Mappe for signaturer
settings-storage-versions = Mappe for versjonslogg
settings-storage-bookmarks = Fil for bokmerker
settings-storage-apply = Bruk
settings-storage-choose = Velg…
settings-storage-note = Filer som allerede ligger på et gammelt sted, blir liggende der; flytt dem over for å fortsette å bruke dem. Innstillingene for prev lagres i { $file }.
settings-save-failed = Kunne ikke lagre innstillingene: { $error }
settings-no-location = Ingen plassering for innstillinger: HOME er ikke angitt
settings-full-path = Bruk en fullstendig bane, for eksempel ~/Documents/prev.
settings-path-is-folder = { $path } er en mappe, ikke en fil.
settings-folder-missing = Mappen { $path } finnes ikke. Opprett den først, eller velg en annen.
settings-path-is-file = { $path } er en fil, ikke en mappe.
settings-cannot-write = prev kan ikke skrive i { $path }: { $error }.

## Export dialog

export-title = Eksporter
export-format = Filformat
export-quality = Kvalitet
export-size = Størrelse
export-choose = Eksporter…
export-format-webp = WebP (tapsfri)
export-format-unknown = bilde
export-quality-low = Lav
export-quality-medium = Middels
export-quality-high = Høy
export-quality-best = Best
export-size-actual = Faktisk størrelse
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } piksler
export-dialog-failed = Kunne ikke vise lagringsdialogen: { $error }
export-done = Eksporterte { $path }
export-failed = Kunne ikke eksportere: { $error }
export-stopped = eksporten stoppet

## Image window

image-marked-no-edit = Bilder med markering kan ikke redigeres. Eksporter for å beholde markeringen, eller slett den og lukk markeringslinjen.

image-loading-stopped = innlastingen stoppet
image-reverting-stopped = tilbakestillingen stoppet
image-rendering-stopped = gjengivelsen stoppet
image-saving-stopped = lagringen stoppet
image-markup-stopped = markeringen stoppet
image-no-version-store = Ingen plass å ta vare på versjoner
image-revert-failed = Kunne ikke tilbakestille: { $error }
image-read-failed = Kunne ikke lese { $path }: { $error }
image-keep-original-failed = Kunne ikke beholde originalversjonen: { $error }
image-save-failed = Kunne ikke lagre { $path }: { $error }
image-markup-start-failed = Kunne ikke starte markeringen: { $error }
image-cannot-edit = Animasjoner og SVG-tegninger kan ikke redigeres.
image-cannot-mark-up = Du kan ikke legge til markering i animasjoner og SVG-tegninger.
image-mark-up-wait = Vent til redigeringen er ferdig, og legg deretter til markering.
image-crop-needs-selection = Dra først et utvalg (Velg-verktøyet), og beskjær deretter.
image-size-needed = Angi bredde og høyde i piksler.
image-cannot-save-format = Endringer i «{ $name }» kan ikke lagres i filens format. Bruk Eksporter ({ $keys }).
image-cannot-save-format-unbound = Endringer i «{ $name }» kan ikke lagres i filens format. Bruk Eksporter.
image-cannot-export-animation = Animasjoner kan ikke eksporteres ennå.
image-drop-pages = Sider kan slippes på et dokument.
image-drag-failed = Kunne ikke begynne å dra.
image-picture-save-failed = Kunne ikke lagre bildet i Nedlastinger-mappen.
image-open-failed = prev kan ikke åpne dette bildet
image-opening = Åpner…
image-name-mismatch-title = Navnet samsvarer ikke med formatet
image-name-mismatch = «{ $name }» lagres som en { $format }-fil, men navnet slutter på .{ $extension }. Det kan hende andre apper ikke kan åpne den.
image-name-mismatch-no-extension = «{ $name }» lagres som en { $format }-fil, men navnet har ingen filetternavn. Det kan hende andre apper ikke kan åpne den.
image-choose-again = Velg på nytt
image-save-as-is = Lagre som den er
image-dimensions = { $width } × { $height }
image-frame-position = bilderute { $current } av { $total }
image-position = { $current } av { $total }
image-edited = redigert
image-sidebar = Sidepanel
image-zoom-out = Zoom ut
image-zoom-in = Zoom inn
image-zoom = { $percent } %
image-fit = Tilpass til vinduet
image-actual-size = Faktisk størrelse
image-undo = Angre
image-redo = Gjør om
image-rotate-left = Roter mot venstre
image-rotate-right = Roter mot høyre
image-flip-horizontal = Speilvend vannrett
image-flip-vertical = Speilvend loddrett
image-select = Rektangulært utvalg
image-crop = Beskjær til utvalg
image-adjust-size-tool = Juster størrelse
image-adjust-color-tool = Juster farge
image-inspector = Inspektør
image-markup = Markering
image-export = Eksporter
image-settings = Innstillinger
image-adjust-color = Juster farge
image-adjust-size = Juster størrelse
image-exposure = Eksponering
image-contrast = Kontrast
image-saturation = Metning
image-temperature = Temperatur
image-tint = Fargetone
image-sepia = Sepiatone
image-sharpness = Skarphet
image-levels = Nivåer
image-black-point = Svartpunkt
image-midtones = Mellomtoner
image-white-point = Hvitpunkt
image-reset-all = Tilbakestill alt
image-current-size = Nåværende størrelse: { $width } × { $height } piksler
image-width = Bredde
image-height = Høyde
image-scale-proportionally = Skaler proporsjonalt
image-resize = Endre størrelse
image-inspector-loading = Laster inn…
image-file = Fil
image-format = Filformat
image-dimensions-label = Dimensjoner
image-pixels = { $width } × { $height } piksler
image-no-camera = Ingen kamerainformasjon.
image-location = Sted
image-remove-location = Fjern stedsinformasjon
image-no-location = Ingen stedsinformasjon.
image-keywords-description = Nøkkelord og beskrivelse
image-keywords-hint = Nøkkelord, atskilt med komma
image-description = Beskrivelse
image-keywords-unsupported = Nøkkelord kan lagres i JPEG-, PNG- og WebP-filer.
image-revert-to = Tilbakestill til
image-no-versions = Ingen tidligere versjoner.
image-revert = Tilbakestill
image-size-kb = { $size } kB
image-size-mb = { $size } MB
image-close-title = Vil du lukke uten å eksportere markeringen?
image-close-body = { $count ->
    [one] Markering på et bilde varer bare mens vinduet er åpent. Eksporter bildet for å beholde den: Markeringen tegnes inn i kopien du lagrer.
   *[other] Markering på bilder varer bare mens vinduet er åpent. Eksporter hvert bilde for å beholde den: Markeringen tegnes inn i kopien du lagrer.
}
image-close-anyway = Lukk likevel

## Markdown

markdown-reading-stopped = lesingen stoppet
markdown-read-failed = prev kan ikke lese denne filen
markdown-draw-failed = Kunne ikke tegne dokumentet
markdown-export-size = Hele dokumentet, { $width } × { $height } piksler
markdown-not-found = Ikke funnet
markdown-match = { $current } av { $total }
markdown-search = Søk
markdown-smaller-text = Mindre tekst
markdown-larger-text = Større tekst
markdown-zoom = { $percent } %
markdown-actual-size = Faktisk størrelse
markdown-inspector = Inspektør
markdown-export = Eksporter
markdown-settings = Innstillinger
markdown-file = Fil
markdown-document = Dokument
markdown-words = Ord
markdown-lines = Linjer
markdown-pictures = Bilder

## Image details

image-meta-camera = Kamera
image-meta-exposure = Eksponering
image-meta-image = Bilde
image-meta-make = Produsent
image-meta-model = Modell
image-meta-lens = Objektiv
image-meta-exposure-time = Lukkertid
image-meta-f-number = Blendertall
image-meta-iso = ISO
image-meta-focal-length = Brennvidde
image-meta-exposure-bias = Eksponeringskompensasjon
image-meta-flash = Blits
image-meta-date-taken = Opptaksdato
image-meta-orientation = Retning
image-meta-color-space = Fargerom
image-meta-software = Programvare
image-meta-artist = Fotograf
image-meta-copyright = Opphavsrett
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Speilvendt vannrett
    [3] Rotert 180°
    [4] Speilvendt loddrett
    [5] Speilvendt vannrett, rotert 90° mot klokken
    [6] Rotert 90° med klokken
    [7] Speilvendt vannrett, rotert 90° med klokken
    [8] Rotert 90° mot klokken
   *[other] Ukjent ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Utløst
   *[no] Ikke utløst
}{ $mode ->
    [on] , tvunget på
    [off] , av
    [auto] , automatisk
   *[unknown] {""}
}{ $redeye ->
    [yes] , reduksjon av røde øyne
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Ukalibrert
   *[other] Annet ({ $code })
}

## Errors

error-pdf-open = kan ikke åpne dokumentet: { $detail }
error-pdf-page-out-of-range = side { $page } finnes ikke
error-pdf-password-protected = dokumentet er passordbeskyttet; åpne det og kopier sidene i stedet
error-pdf-no-pages = ingen sider å trekke ut
error-pdf-crop-outside = beskjæringsområdet er utenfor siden
error-pdf-closed = dokumentet er lukket
error-pdf-saved-unreadable = det lagrede dokumentet kan ikke lenger åpnes
error-image-read = kan ikke lese filen: { $detail }
error-image-invalid = bildet er skadet eller ugyldig: { $detail }
error-image-missing-library = dette formatet krever { $library }, som ikke er installert
error-image-unsupported = { $format }-bilder støttes ikke ennå
error-image-encode = kan ikke kode bildet: { $detail }
error-exif-malformed = EXIF-dataene er feilformaterte
error-settings-read = kan ikke lese innstillingene: { $detail }
error-settings-invalid = ugyldige innstillinger: { $detail }
error-remove-location = kunne ikke fjerne stedet: { $error }
error-location-unsupported = stedsinformasjon kan fjernes fra JPEG-, PNG-, WebP- og TIFF-filer
error-xmp-unsupported = nøkkelord og beskrivelser kan bare lagres i JPEG-, PNG- og WebP-filer

## Formats

format-camera-raw = Kamera-RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = Om prev
menu-settings = Innstillinger…
menu-services = Tjenester
menu-hide = Skjul prev
menu-hide-others = Skjul andre
menu-show-all = Vis alle
menu-quit = Avslutt prev
menu-file = Arkiv
menu-open = Åpne…
menu-close = Lukk vindu
menu-export = Eksporter…
menu-print = Skriv ut…
menu-edit = Rediger
menu-undo = Angre
menu-redo = Gjør om
menu-cut = Klipp ut
menu-copy = Kopier
menu-paste = Lim inn
menu-select-all = Marker alt
menu-find = Søk
menu-find-next = Søk etter neste
menu-find-previous = Søk etter forrige
menu-view = Vis
menu-hide-sidebar = Skjul sidepanel
menu-thumbnails = Miniatyrer
menu-contents = Innholdsfortegnelse
menu-notes = Uthevinger og notater
menu-bookmarks = Bokmerker
menu-zoom-in = Zoom inn
menu-zoom-out = Zoom ut
menu-actual-size = Faktisk størrelse
menu-zoom-to-fit = Zoom for å tilpasse
menu-inspector = Vis inspektør
menu-slideshow = Lysbildeserie
menu-full-screen = Bruk fullskjerm
menu-go = Gå
menu-next-page = Neste side
menu-previous-page = Forrige side
menu-go-to-page = Gå til side…
menu-bookmark = Legg til bokmerke
menu-tools = Verktøy
menu-markup = Vis markeringsverktøylinjen
menu-rotate-left = Roter mot venstre
menu-rotate-right = Roter mot høyre
menu-crop = Beskjær
menu-adjust-color = Juster farge…
menu-window = Vindu
menu-minimize = Minimer
menu-zoom = Zoom
menu-bring-all-to-front = Legg alle fremst

## Outside control

settings-outside-control = Ekstern styring
settings-allow-outside-control = Tillat ekstern styring
settings-allow-outside-control-note = KI-agenter som Claude Code kan lese og endre filene dine i prev via prev --mcp. prev spør før hver nye agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Tillatt: { $agents }
settings-forget-agents = Glem
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Vil du la { $agent } styre prev?
agent-prompt-body = { $agent } ber om å bruke prevs eksterne styring til å lese de åpne filene dine og endre dem. Du kan slå av ekstern styring i Innstillinger.
agent-prompt-allow = Tillat
agent-prompt-deny = Ikke tillat
settings-ask-before-note = Spør før en agent:
settings-ask-reading = Leser en fil
settings-ask-viewing = Endrer visningen eller et vindu
settings-ask-marking-up = Legger til markering i en fil
settings-ask-editing = Redigerer en fil
settings-ask-signing = Signerer en fil
settings-ask-redacting = Bruker sladding
settings-ask-exporting = Eksporterer en fil
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Vil du la { $agent } lese denne filen?
agent-ask-view = Vil du la { $agent } endre visningen?
agent-ask-markup = Vil du la { $agent } legge til markering i denne filen?
agent-ask-edit = Vil du la { $agent } redigere denne filen?
agent-ask-sign = Vil du la { $agent } signere denne filen?
agent-ask-redact = Vil du la { $agent } bruke sladding?
agent-ask-export = Vil du la { $agent } eksportere denne filen?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } ber om å bruke «{ $tool }». I Innstillinger velger du hva prev spør om.
agent-ask-final = Dette kan ikke angres.
