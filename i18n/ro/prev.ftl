# prev's interface text in Romanian (Română), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = marcaje (a marca), note = notă,
# highlight = evidențiere, annotation = adnotare, redact/redaction =
# înnegrire, inspector = inspector, zoom in/out = mărește/micșorează,
# bookmark = semn de carte, page = pagină, folder = dosar, settings = configurări.
# Buttons and menu items use the second-person imperative (Salvează, Anulează, Închide).
# Plurals use one/few/other; other puts „de” before the noun (20 de pagini).

## Language

language-name = Română

## Common

common-cancel = Anulează
common-close = Închide
common-save = Salvează

## Settings

settings-title = Configurări
settings-appearance = Aspect
settings-colors = Culori
settings-windows = Ferestre
settings-default-app = Aplicație implicită
settings-default-app-label = Deschide fișierele cu prev
settings-default-app-note = Fă din prev aplicația care deschide PDF-uri, imagini, desene SVG și fișiere Markdown.
settings-default-app-note-windows = Windows îți permite să alegi aplicațiile implicite doar în propriile Setări. Butonul deschide acolo pagina prev.
settings-default-app-note-macos = macOS îți cere să confirmi fiecare tip: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP și AVIF.
settings-default-app-status = Tipuri de fișiere deschise cu prev: { $set } din { $total }.
settings-default-app-button = Setează ca implicită
settings-default-app-button-windows = Deschide Setări
settings-default-app-no-entry = Intrarea de desktop a prev nu este instalată, așa că sistemul nu poate deschide fișiere cu el. Instalează prev dintr-un pachet sau cu scripts/install.sh.
settings-default-app-no-bundle = Deschide prev din prev.app pentru a-l face implicit.
settings-default-app-failed = prev nu a putut fi setat ca implicit: { $error }
settings-storage = Stocare
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (versiune de dezvoltare, { $build })

## Markup toolbar

markup-tool-select = Selectează
markup-tool-area = Selecție dreptunghiulară
markup-tool-sketch = Schiță
markup-tool-draw = Desenează
markup-tool-shapes = Forme
markup-tool-text-box = Casetă de text
markup-tool-highlight = Evidențiază
markup-tool-note = Notă
markup-tool-sign = Semnează
markup-tool-redact = Înnegrește
markup-apply = Aplică
markup-apply-redactions = Aplică înnegririle
markup-shape-style = Stil formă
markup-border-color = Culoare contur
markup-fill-color = Culoare umplere
markup-text-style = Stil text
markup-delete = Șterge
markup-undo = Anulează
markup-redo = Refă

## Markup menus

markup-shape-rectangle = Dreptunghi
markup-shape-rounded-rectangle = Dreptunghi rotunjit
markup-shape-oval = Oval
markup-shape-line = Linie
markup-shape-arrow = Săgeată
markup-shape-star = Stea
markup-shape-polygon = Poligon
markup-shape-speech-bubble = Balon de dialog
markup-shape-loupe = Lupă
markup-shape-mask = Mască
markup-style-highlight = Evidențiere
markup-style-underline = Subliniere
markup-style-strikethrough = Tăiere cu o linie
markup-style-squiggly = Subliniere ondulată
markup-menu-color = Culoare
markup-menu-font = Font
markup-menu-size = Dimensiune
markup-menu-alignment = Aliniere
markup-line-width = { $width } pt
markup-dashed = Întreruptă

## Notes

markup-kind-note = Notă
markup-kind-text-box = Casetă de text
markup-kind-stamp = Ștampilă
markup-kind-redaction = Înnegrire
markup-kind-shape = Formă
markup-note-delete = Șterge nota
markup-note-done = Gata
markup-note-placeholder = Scrie o notă
markup-notes-empty = Nicio evidențiere sau notă
markup-notes-empty-hint = Evidențierile, notele și casetele de text apar aici.
markup-notes-page = Pagina { $page }

## Markup errors

markup-change-failed = Documentul nu a putut fi modificat: { $error }
markup-copy-area-failed = Zona nu a putut fi copiată: { $error }
markup-document-closed = documentul s-a închis
markup-render-area-failed = zona nu a putut fi randată
markup-copy-stopped = copierea s-a oprit

## Signatures

signature-menu-empty = Nicio semnătură încă.
signature-delete = Șterge semnătura
signature-create = Creează o semnătură…
signature-dialog-title = Creează o semnătură
signature-tab-draw = Desenează
signature-tab-type = Scrie
signature-tab-image = Imagine
signature-draw-hint = Semnează pe linie cu mouse-ul, stiloul sau touchpadul.
signature-your-name = Numele tău
signature-image-hint = Alege o fotografie sau o scanare a semnăturii tale pe hârtie albă.
signature-choose-image = Alege o imagine…
signature-description = Descriere, de exemplu Nume complet sau Inițiale
signature-clear = Golește
signature-ink = Cerneală
signature-thickness = Grosime
signature-sign-first = Semnează mai întâi, apoi salvează.
signature-default-name = Semnătura { $number }
signature-change-failed = Semnăturile nu au putut fi modificate: { $error }
signature-no-data-folder = niciun dosar de date: HOME nu este setat
signature-removing-stopped = eliminarea s-a oprit
signature-saving-stopped = salvarea s-a oprit
signature-reading-stopped = citirea s-a oprit
signature-not-an-image = fișierul nu este o imagine pe care prev o poate citi
signature-no-frames = imaginea nu are cadre
signature-not-found = nu s-a găsit nicio semnătură în imagine

## Dragging

drag-pages-need-document = Paginile pot fi plasate pe un document.
drag-image-unsupported = prev nu poate deschide această imagine.
drag-area-failed = Zona nu a putut fi trasă: { $error }
drag-pages-failed = Paginile nu au putut fi trase: { $error }
drag-start-failed = Tragerea nu a putut începe.
drag-file-pages = Pagini
drag-file-one-page = { $name } (pagina { $page })
drag-file-page-range = { $name } (paginile { $first }–{ $last })
drag-file-image = Imagine
drop-pdf-title = Adaugi în acest document?
drop-pdf-body = Adaugi „{ $name }” la sfârșitul acestui document sau îl deschizi într-o fereastră separată?
drop-pdfs-body = { $count ->
    [one] Adaugi acest fișier PDF la sfârșitul acestui document sau îl deschizi într-o fereastră separată?
    [few] Adaugi aceste { $count } fișiere PDF la sfârșitul acestui document sau le deschizi în ferestre separate?
   *[other] Adaugi aceste { $count } de fișiere PDF la sfârșitul acestui document sau le deschizi în ferestre separate?
}
drop-pdf-add = Adaugă la sfârșit
drop-pdf-open = Deschide separat

## PDF window

pdf-opening = Se deschide…
pdf-open-failed = prev nu poate deschide acest document
pdf-no-pages = Documentul nu are pagini.
pdf-document-closed = documentul s-a închis
pdf-keep-original-failed = versiunea originală nu a putut fi păstrată: { $error }
pdf-save-failed = Nu s-a putut salva: { $error }
pdf-nothing-to-paste = Nu există nimic de lipit.
pdf-pasting-stopped = lipirea s-a oprit
pdf-file-dialog-failed = Dialogul de fișiere nu a putut fi afișat: { $error }
pdf-bookmarks-no-home = Semnele de carte nu pot fi salvate: HOME nu este setat
pdf-bookmarks-save-failed = Semnele de carte nu au putut fi salvate: { $error }
pdf-bookmark-page = Pagina { $page }

pdf-password-protected = „{ $name }” este protejat prin parolă
pdf-password = Parolă
pdf-password-wrong = Parolă incorectă. Încearcă din nou.
pdf-unlock = Deblochează

pdf-sidebar = Bară laterală
pdf-page-of = din { $count }
pdf-zoom-out = Micșorează
pdf-zoom-in = Mărește
pdf-zoom-percent = { $percent }%
pdf-fit-page = Potrivește pagina
pdf-fit-width = Potrivește lățimea
pdf-actual-size = Dimensiune reală
pdf-view-continuous = Derulare continuă
pdf-view-single-page = O singură pagină
pdf-view-two-pages = Două pagini
pdf-undo = Anulează
pdf-redo = Refă
pdf-rotate-left = Rotește la stânga
pdf-rotate-right = Rotește la dreapta
pdf-inspector = Inspector
pdf-markup = Marcaje
pdf-export = Exportă
pdf-settings = Configurări

pdf-search = Caută
pdf-search-not-found = Nu s-a găsit
pdf-searching = Se caută…
pdf-search-match = { $current } din { $total }
pdf-search-match-more = { $current } din { $total }+

pdf-inspector-file = Fișier
pdf-inspector-document = Document
pdf-inspector-pages = Pagini
pdf-inspector-title = Titlu
pdf-inspector-author = Autor
pdf-inspector-subject = Subiect
pdf-inspector-keywords = Cuvinte cheie
pdf-inspector-created = Creat
pdf-inspector-modified = Modificat
pdf-inspector-application = Aplicație
pdf-inspector-producer = Producător PDF
pdf-inspector-version = Versiune
pdf-inspector-security = Securitate
pdf-inspector-not-encrypted = Necriptat
pdf-inspector-encrypted = Criptat ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } pagină
    [few] { $count } pagini
   *[other] { $count } de pagini
}
pdf-inspector-page-size = Dimensiune pagină
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inchi)
pdf-loading = Se încarcă…

pdf-tab-pages = Pagini
pdf-tab-contents = Cuprins
pdf-tab-notes = Evidențieri și note
pdf-tab-bookmarks = Semne de carte
pdf-no-outline = Niciun cuprins
pdf-no-outline-detail = Acest document nu are cuprins.
pdf-no-bookmarks = Niciun semn de carte
pdf-no-bookmarks-detail = Apasă { $keys } pentru a pune un semn de carte la o pagină.
pdf-no-bookmarks-detail-unbound = Paginile cu semn de carte apar aici.
pdf-remove-bookmark = Elimină semnul de carte

## Page editing

pages-menu = Pagini
pages-insert-blank = Inserează o pagină goală
pages-insert-file = Inserează din fișier…
pages-copy = { $count ->
    [one] Copiază pagina
   *[other] Copiază paginile
}
pages-paste = { $count ->
    [one] Lipește pagina
    [few] Lipește { $count } pagini
   *[other] Lipește { $count } de pagini
}
pages-crop = Decupează la selecție
pages-select-all = Selectează toate paginile
pages-delete = { $count ->
    [one] Șterge pagina
   *[other] Șterge paginile
}
pages-apply-redactions = Aplică înnegririle…
pages-no-copied = Nu există pagini copiate de lipit.
pages-copied = { $count ->
    [one] S-a copiat { $count } pagină.
    [few] S-au copiat { $count } pagini.
   *[other] S-au copiat { $count } de pagini.
}
pages-copy-failed = Paginile nu au putut fi copiate: { $error }
pages-reading-stopped = citirea s-a oprit
pages-image-unreadable = nu este o imagine pe care prev o poate citi
pages-read-failed = Fișierul nu a putut fi citit: { $error }
pages-at-least-one = Un document trebuie să aibă cel puțin o pagină.
pages-crop-needs-area = Alege mai întâi o zonă cu instrumentul de selecție dreptunghiulară.
pages-change-failed = Paginile nu au putut fi modificate: { $error }
pages-no-redactions = Nu existau înnegriri de aplicat.
pages-redactions-applied = { $count ->
    [one] S-a aplicat { $count } înnegrire.
    [few] S-au aplicat { $count } înnegriri.
   *[other] S-au aplicat { $count } de înnegriri.
}
pages-forget-versions-failed = Versiunile anterioare nu au putut fi șterse: { $error }
pages-redact-title = Aplici înnegririle?
pages-redact-body = { $count ->
    [one] Textul, imaginile și desenele de sub marcaj sunt eliminate definitiv din document, iar marcajul devine o casetă neagră. Acțiunea nu poate fi anulată, iar versiunile anterioare ale acestui fișier păstrate de prev sunt șterse.
    [few] Textul, imaginile și desenele de sub cele { $count } marcaje sunt eliminate definitiv din document, iar marcajele devin casete negre. Acțiunea nu poate fi anulată, iar versiunile anterioare ale acestui fișier păstrate de prev sunt șterse.
   *[other] Textul, imaginile și desenele de sub cele { $count } de marcaje sunt eliminate definitiv din document, iar marcajele devin casete negre. Acțiunea nu poate fi anulată, iar versiunile anterioare ale acestui fișier păstrate de prev sunt șterse.
}
pages-redact-apply = Aplică

## PDF export

pages-export-title = Exportă
pages-export-format = Format
pages-export-reduce = Redu dimensiunea fișierului (imagini la 150 dpi)
pages-export-flatten = Aplatizează adnotările și câmpurile de formular
pages-export-flatten-detail = Marcajele și câmpurile completate devin parte din pagini și nu mai pot fi editate. Înnegririle neaplicate încă sunt omise.
pages-export-encrypt = Criptează cu o parolă
pages-export-password = Parolă
pages-export-verify-password = Confirmă parola
pages-export-resolution = Rezoluție
pages-export-dpi = { $dpi } dpi
pages-export-quality = Calitate
pages-export-quality-low = Scăzută
pages-export-quality-medium = Medie
pages-export-quality-high = Ridicată
pages-export-quality-best = Maximă
pages-export-one-file = Toate paginile intră într-un singur fișier.
pages-export-file-per-page = Fiecare pagină este salvată în propriul fișier, numerotat după numele pe care îl alegi.
pages-export-selected-only = { $count ->
    [one] Doar pagina selectată
    [few] Doar cele { $count } pagini selectate
   *[other] Doar cele { $count } de pagini selectate
}
pages-export-choose = Exportă…
pages-export-no-password = Introdu o parolă.
pages-export-password-mismatch = Parolele nu coincid.
pages-export-file-name = { $name } (exportat)
pages-export-untitled = document
pages-export-same-file = Exportă într-un fișier nou; acest document se salvează automat.
pages-export-exporting = Se exportă „{ $name }”…
pages-export-done = S-a exportat „{ $name }”.
pages-export-done-images = { $count ->
    [one] S-a exportat { $count } imagine.
    [few] S-au exportat { $count } imagini.
   *[other] S-au exportat { $count } de imagini.
}
pages-export-failed = Nu s-a putut exporta: { $error }
pages-export-stopped = exportul s-a oprit

## Start window

app-start-hint = Deschide sau trage aici un fișier PDF, o imagine, un desen SVG sau un fișier Markdown.
app-start-open = Deschide…
app-title-dev = { $title } (dezvoltare)
app-viewer-missing = { $kind }: acest vizualizator nu este încă gata.
app-cannot-open = prev nu poate deschide acest tip de fișier.
app-cannot-read = prev nu poate citi acest fișier: { $error }
app-kind-pdf = Document PDF
app-kind-image = Imagine { $format }
app-kind-svg = Desen SVG
app-kind-markdown = Document Markdown
app-file-dialog-failed = Dialogul de fișiere nu a putut fi afișat: { $error }

## Actions

action-open = Deschide
action-settings = Configurări

## Toolbar

app-toolbar-keep-shown = Păstrează bara de instrumente vizibilă
app-toolbar-auto-hide = Ascunde bara de instrumente când indicatorul iese din fereastră
app-toolbar-more = Mai multe

## File facts

app-fact-name = Nume
app-fact-folder = Dosar
app-fact-size = Dimensiune
app-fact-modified = Modificat
app-size-bytes = { $count ->
    [one] { $count } octet
    [few] { $count } octeți
   *[other] { $count } de octeți
}
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Link nevalid { $uri }: { $error }
app-link-open-failed = { $uri } nu a putut fi deschis: { $error }
app-paste-needs-wl-clipboard = instalează wl-clipboard pentru a lipi imagini
app-copy-needs-wl-clipboard = instalează wl-clipboard pentru a copia imagini
app-copy-no-pixels = zona nu are pixeli
app-copy-no-input = wl-copy nu a primit date de intrare
app-copy-failed = wl-copy a eșuat
app-clipboard-open-failed = Clipboardul nu a putut fi deschis: { $error }
app-copy-image-failed = Imaginea nu a putut fi copiată: { $error }

## Printing

print-failed = Nu s-a putut tipări: { $error }
print-stopped = Tipărirea s-a oprit
print-unavailable = Tipărirea nu este încă disponibilă pe acest sistem.
print-no-window = Nu s-a putut tipări: nu există nicio fereastră peste care să apară dialogul de tipărire
print-dialog-failed = Dialogul de tipărire nu a putut fi afișat: { $error }
print-job-not-started = imprimanta nu a pornit lucrarea
print-printer-stopped = imprimanta s-a oprit

## File dialogs

dialog-open = Deschide
dialog-filter-all = Toate fișierele acceptate
dialog-filter-pdf = Documente PDF
dialog-filter-images = Imagini
dialog-filter-svg = Desene SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Alege dosarul de semnături
dialog-choose-versions = Alege dosarul istoricului de versiuni
dialog-choose-bookmarks = Alege fișierul de semne de carte

## Command line

usage-help =
    Utilizare: prev [FILE]...
               prev --mcp

    Vizualizează și editează PDF-uri și imagini. Fișierele se deschid în ferestre
    ale instanței prev care rulează, pornită la nevoie.

    Opțiuni:
      -h, --help     Afișează acest ajutor
      -V, --version  Afișează versiunea
          --mcp      Servește MCP pe stdin și stdout, ca agenții AI să controleze
                     instanța prev care rulează

## Settings, continued

settings-language = Limbă
settings-language-system = Limba sistemului: { $language }
settings-input-language = Limba de introducere
settings-input-language-system = Urmează aspectul tastaturii
settings-input-language-note = Stabilește partea din care pornește un câmp de text gol. Textul scris își păstrează propria direcție.

settings-appearance-system = Sistem
settings-appearance-light = Luminos
settings-appearance-dark = Întunecat
settings-system-accent = Folosește culoarea de accent a sistemului
settings-omarchy-note = Culorile sunt create din culoarea de accent a temei „{ $theme }”.
settings-system-accent-note = Culorile sunt create din culoarea de accent a sistemului.
settings-system-accent-none = Sistemul nu are o culoare de accent, așa că prev o folosește pe cea aleasă mai jos.
settings-accent-chosen-note = Culorile sunt create din culoarea aleasă mai jos.
settings-auto-hide = Ascunde bara de instrumente când indicatorul iese din fereastră
settings-auto-hide-note = Bara de instrumente plutește deasupra documentului și se retrage cât timp indicatorul este în afara ferestrei.
settings-animations = Animații
settings-animations-note = Bare și panouri care glisează, dialoguri care cresc și butoane elastice.
settings-animations-reduced = Dezactivate cât timp sistemul cere mișcare redusă.
settings-corner-radius = Raza colțurilor
settings-corner-radius-note = Pentru dialoguri și bara de instrumente plutitoare.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparența suprapunerii
settings-overlay-note = Cât din pagină se vede prin bara de instrumente plutitoare.
settings-overlay-value = { $percent }%
settings-storage-signatures = Dosarul de semnături
settings-storage-versions = Dosarul istoricului de versiuni
settings-storage-bookmarks = Fișierul de semne de carte
settings-storage-apply = Aplică
settings-storage-choose = Alege…
settings-storage-note = Fișierele păstrate deja într-un loc vechi rămân acolo; mută-le ca să le folosești în continuare. Configurările aplicației prev sunt salvate în { $file }.
settings-save-failed = Configurările nu au putut fi salvate: { $error }
settings-no-location = Niciun loc pentru configurări: HOME nu este setat
settings-full-path = Folosește o cale completă, de exemplu ~/Documents/prev.
settings-path-is-folder = { $path } este un dosar, nu un fișier.
settings-folder-missing = Dosarul { $path } nu există. Creează-l mai întâi sau alege altul.
settings-path-is-file = { $path } este un fișier, nu un dosar.
settings-cannot-write = prev nu poate scrie în { $path }: { $error }.

## Export dialog

export-title = Exportă
export-format = Format
export-quality = Calitate
export-size = Dimensiune
export-choose = Exportă…
export-format-webp = WebP (fără pierderi)
export-format-unknown = imagine
export-quality-low = Scăzută
export-quality-medium = Medie
export-quality-high = Ridicată
export-quality-best = Maximă
export-size-actual = Dimensiune reală
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } pixeli
export-dialog-failed = Dialogul de salvare nu a putut fi afișat: { $error }
export-done = Exportat în { $path }
export-failed = Nu s-a putut exporta: { $error }
export-stopped = exportul s-a oprit

## Image window

image-marked-no-edit = Imaginile cu marcaje nu pot fi editate. Exportă imaginea ca să păstrezi marcajele sau șterge-le și închide bara de marcaje.

image-loading-stopped = încărcarea s-a oprit
image-reverting-stopped = revenirea s-a oprit
image-rendering-stopped = randarea s-a oprit
image-saving-stopped = salvarea s-a oprit
image-markup-stopped = marcarea s-a oprit
image-no-version-store = Niciun loc pentru păstrarea versiunilor
image-revert-failed = Nu s-a putut reveni: { $error }
image-read-failed = { $path } nu a putut fi citit: { $error }
image-keep-original-failed = Versiunea originală nu a putut fi păstrată: { $error }
image-save-failed = { $path } nu a putut fi salvat: { $error }
image-markup-start-failed = Marcarea nu a putut porni: { $error }
image-cannot-edit = Animațiile și desenele SVG nu pot fi editate.
image-cannot-mark-up = Animațiile și desenele SVG nu pot fi marcate.
image-mark-up-wait = Așteaptă să se termine editarea, apoi adaugă marcaje.
image-crop-needs-selection = Trage mai întâi o selecție (instrumentul Selectează), apoi decupează.
image-size-needed = Introdu o lățime și o înălțime în pixeli.
image-cannot-save-format = Modificările aduse fișierului „{ $name }” nu pot fi salvate în formatul său. Folosește Exportă ({ $keys }).
image-cannot-save-format-unbound = Modificările aduse fișierului „{ $name }” nu pot fi salvate în formatul său. Folosește Exportă.
image-cannot-export-animation = Animațiile nu pot fi exportate încă.
image-drop-pages = Paginile pot fi plasate pe un document.
image-drag-failed = Tragerea nu a putut începe.
image-picture-save-failed = Imaginea nu a putut fi salvată în dosarul Descărcări.
image-open-failed = prev nu poate deschide această imagine
image-opening = Se deschide…
image-name-mismatch-title = Numele nu se potrivește cu formatul
image-name-mismatch = „{ $name }” va fi salvat ca fișier { $format }, dar numele se termină în .{ $extension }. Este posibil ca alte aplicații să nu îl deschidă.
image-name-mismatch-no-extension = „{ $name }” va fi salvat ca fișier { $format }, dar numele nu are extensie. Este posibil ca alte aplicații să nu îl deschidă.
image-choose-again = Alege din nou
image-save-as-is = Salvează așa
image-dimensions = { $width } × { $height }
image-frame-position = cadrul { $current } din { $total }
image-position = { $current } din { $total }
image-edited = editată
image-sidebar = Bară laterală
image-zoom-out = Micșorează
image-zoom-in = Mărește
image-zoom = { $percent }%
image-fit = Potrivește în fereastră
image-actual-size = Dimensiune reală
image-undo = Anulează
image-redo = Refă
image-rotate-left = Rotește la stânga
image-rotate-right = Rotește la dreapta
image-flip-horizontal = Oglindește orizontal
image-flip-vertical = Oglindește vertical
image-select = Selecție dreptunghiulară
image-crop = Decupează la selecție
image-adjust-size-tool = Ajustează dimensiunea
image-adjust-color-tool = Ajustează culoarea
image-inspector = Inspector
image-markup = Marcaje
image-export = Exportă
image-settings = Configurări
image-adjust-color = Ajustare culoare
image-adjust-size = Ajustare dimensiune
image-exposure = Expunere
image-contrast = Contrast
image-saturation = Saturație
image-temperature = Temperatură
image-tint = Nuanță
image-sepia = Sepia
image-sharpness = Claritate
image-levels = Niveluri
image-black-point = Punct negru
image-midtones = Tonuri medii
image-white-point = Punct alb
image-reset-all = Resetează tot
image-current-size = Dimensiune actuală: { $width } × { $height } pixeli
image-width = Lățime
image-height = Înălțime
image-scale-proportionally = Scalează proporțional
image-resize = Redimensionează
image-inspector-loading = Se încarcă…
image-file = Fișier
image-format = Format
image-dimensions-label = Dimensiuni
image-pixels = { $width } × { $height } pixeli
image-no-camera = Nicio informație despre cameră.
image-location = Locație
image-remove-location = Elimină informațiile de locație
image-no-location = Nicio informație de locație.
image-keywords-description = Cuvinte cheie și descriere
image-keywords-hint = Cuvinte cheie, separate prin virgule
image-description = Descriere
image-keywords-unsupported = Cuvintele cheie pot fi salvate în fișiere JPEG, PNG și WebP.
image-revert-to = Revino la
image-no-versions = Nicio versiune anterioară.
image-revert = Revino
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Închizi fără să exporți marcajele?
image-close-body = { $count ->
    [one] Marcajele de pe o imagine durează doar cât timp fereastra ei este deschisă. Exportă imaginea ca să le păstrezi: marcajele sunt desenate în copia pe care o salvezi.
   *[other] Marcajele de pe imagini durează doar cât timp fereastra lor este deschisă. Exportă fiecare imagine ca să le păstrezi: marcajele sunt desenate în copia pe care o salvezi.
}
image-close-anyway = Închide oricum

## Markdown

markdown-reading-stopped = citirea s-a oprit
markdown-read-failed = prev nu poate citi acest fișier
markdown-draw-failed = Documentul nu a putut fi desenat
markdown-export-size = Întregul document, { $width } × { $height } pixeli
markdown-not-found = Nu s-a găsit
markdown-match = { $current } din { $total }
markdown-search = Caută
markdown-smaller-text = Text mai mic
markdown-larger-text = Text mai mare
markdown-zoom = { $percent }%
markdown-actual-size = Dimensiune reală
markdown-inspector = Inspector
markdown-export = Exportă
markdown-settings = Configurări
markdown-file = Fișier
markdown-document = Document
markdown-words = Cuvinte
markdown-lines = Rânduri
markdown-pictures = Imagini

## Image details

image-meta-camera = Cameră
image-meta-exposure = Expunere
image-meta-image = Imagine
image-meta-make = Producător
image-meta-model = Model
image-meta-lens = Obiectiv
image-meta-exposure-time = Timp de expunere
image-meta-f-number = Număr f
image-meta-iso = ISO
image-meta-focal-length = Distanță focală
image-meta-exposure-bias = Compensare expunere
image-meta-flash = Bliț
image-meta-date-taken = Data fotografierii
image-meta-orientation = Orientare
image-meta-color-space = Spațiu de culoare
image-meta-software = Software
image-meta-artist = Autor
image-meta-copyright = Drepturi de autor
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normală
    [2] Oglindită orizontal
    [3] Rotită cu 180°
    [4] Oglindită vertical
    [5] Oglindită orizontal, rotită cu 90° în sens antiorar
    [6] Rotită cu 90° în sens orar
    [7] Oglindită orizontal, rotită cu 90° în sens orar
    [8] Rotită cu 90° în sens antiorar
   *[other] Necunoscută ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Declanșat
   *[no] Nedeclanșat
}{ $mode ->
    [on] , forțat
    [off] , dezactivat
    [auto] , automat
   *[unknown] {""}
}{ $redeye ->
    [yes] , reducerea efectului de ochi roșii
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Necalibrat
   *[other] Altul ({ $code })
}

## Errors

error-pdf-open = documentul nu poate fi deschis: { $detail }
error-pdf-page-out-of-range = pagina { $page } nu există
error-pdf-password-protected = documentul este protejat prin parolă; deschide-l și copiază-i paginile
error-pdf-no-pages = nu există pagini de extras
error-pdf-crop-outside = zona de decupare este în afara paginii
error-pdf-closed = documentul s-a închis
error-pdf-saved-unreadable = documentul salvat nu se mai deschide
error-image-read = fișierul nu poate fi citit: { $detail }
error-image-invalid = imaginea este deteriorată sau nevalidă: { $detail }
error-image-missing-library = deschiderea acestui format necesită { $library }, care nu este instalat
error-image-unsupported = imaginile { $format } nu sunt încă acceptate
error-image-encode = imaginea nu poate fi codificată: { $detail }
error-exif-malformed = datele EXIF sunt incorecte
error-settings-read = configurările nu pot fi citite: { $detail }
error-settings-invalid = configurări nevalide: { $detail }
error-remove-location = locația nu a putut fi eliminată: { $error }
error-location-unsupported = informațiile de locație pot fi eliminate din fișiere JPEG, PNG, WebP și TIFF
error-xmp-unsupported = cuvintele cheie și descrierile pot fi salvate doar în fișiere JPEG, PNG și WebP

## Formats

format-camera-raw = RAW de cameră

## The macOS menu bar, named as in macOS's own apps.
menu-about = Despre prev
menu-settings = Configurări…
menu-services = Servicii
menu-hide = Ascunde prev
menu-hide-others = Ascunde-le pe celelalte
menu-show-all = Afișează-le pe toate
menu-quit = Ieși din prev
menu-file = Fișier
menu-open = Deschide…
menu-close = Închide fereastra
menu-export = Exportă…
menu-print = Tipărește…
menu-edit = Editare
menu-undo = Anulează
menu-redo = Refă
menu-cut = Taie
menu-copy = Copiază
menu-paste = Lipește
menu-select-all = Selectează tot
menu-find = Caută
menu-find-next = Caută următorul
menu-find-previous = Caută anteriorul
menu-view = Vizualizare
menu-hide-sidebar = Ascunde bara laterală
menu-thumbnails = Miniaturi
menu-contents = Cuprins
menu-notes = Evidențieri și note
menu-bookmarks = Semne de carte
menu-zoom-in = Mărește
menu-zoom-out = Micșorează
menu-actual-size = Dimensiune reală
menu-zoom-to-fit = Potrivește în fereastră
menu-inspector = Afișează inspectorul
menu-slideshow = Prezentare
menu-full-screen = Intră în modul ecran complet
menu-go = Mergi
menu-next-page = Pagina următoare
menu-previous-page = Pagina anterioară
menu-go-to-page = Mergi la pagina…
menu-bookmark = Adaugă semn de carte
menu-tools = Instrumente
menu-markup = Afișează bara de marcaje
menu-rotate-left = Rotește la stânga
menu-rotate-right = Rotește la dreapta
menu-crop = Decupează
menu-adjust-color = Ajustează culoarea…
menu-window = Fereastră
menu-minimize = Minimizează
menu-zoom = Redimensionează
menu-bring-all-to-front = Adu-le pe toate în față

## Outside control

settings-outside-control = Control extern
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = General
settings-tab-agents = Agenți
settings-allow-outside-control = Permite controlul extern
settings-allow-outside-control-note = Agenții AI precum Claude Code pot citi și modifica fișierele tale din prev, prin prev --mcp. prev întreabă înaintea fiecărui agent nou.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permiși: { $agents }
settings-forget-agents = Uită
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Permiți ca { $agent } să controleze prev?
agent-prompt-body = { $agent } cere să folosească controlul extern al prev, pentru a citi fișierele tale deschise și a le modifica. Poți dezactiva controlul extern în Configurări.
agent-prompt-allow = Permite
agent-prompt-deny = Nu permite
settings-ask-before-note = Întreabă înainte ca un agent să:
settings-ask-reading = Citească un fișier
settings-ask-viewing = Schimbe vizualizarea sau o fereastră
settings-ask-marking-up = Adauge marcaje într-un fișier
settings-ask-editing = Editeze un fișier
settings-ask-signing = Semneze un fișier
settings-ask-redacting = Aplice înnegriri
settings-ask-exporting = Exporte un fișier
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Permiți ca { $agent } să citească acest fișier?
agent-ask-view = Permiți ca { $agent } să schimbe vizualizarea?
agent-ask-markup = Permiți ca { $agent } să adauge marcaje în acest fișier?
agent-ask-edit = Permiți ca { $agent } să editeze acest fișier?
agent-ask-sign = Permiți ca { $agent } să semneze acest fișier?
agent-ask-redact = Permiți ca { $agent } să aplice înnegriri?
agent-ask-export = Permiți ca { $agent } să exporte acest fișier?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } cere să folosească „{ $tool }”. În Configurări alegi despre ce întreabă prev.
agent-ask-final = Acest lucru nu poate fi anulat.
