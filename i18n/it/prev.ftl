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
language-name = Italiano

## Common

common-cancel = Annulla
common-close = Chiudi
common-save = Salva

## Settings

settings-title = Impostazioni
settings-appearance = Aspetto
settings-colors = Colori
settings-windows = Finestre
settings-storage = Archiviazione
settings-version = prev { $version }
settings-version-development = prev { $version } (versione di sviluppo)

## Markup toolbar

markup-tool-select = Seleziona
markup-tool-area = Selezione rettangolare
markup-tool-sketch = Schizzo
markup-tool-draw = Disegna
markup-tool-shapes = Forme
markup-tool-text-box = Casella di testo
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Evidenzia
markup-tool-note = Nota
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Firma
# A verb: the tool that marks areas to black out.
markup-tool-redact = Oscura
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Applica
markup-apply-redactions = Applica oscuramenti
markup-shape-style = Stile forma
markup-border-color = Colore bordo
markup-fill-color = Colore riempimento
markup-text-style = Stile testo
markup-delete = Elimina
markup-undo = Annulla
markup-redo = Ripeti

## Markup menus

markup-shape-rectangle = Rettangolo
markup-shape-rounded-rectangle = Rettangolo arrotondato
markup-shape-oval = Ovale
markup-shape-line = Linea
markup-shape-arrow = Freccia
markup-shape-star = Stella
markup-shape-polygon = Poligono
markup-shape-speech-bubble = Fumetto
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Lente
# A shape that darkens the page around it.
markup-shape-mask = Maschera
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Evidenziazione
markup-style-underline = Sottolineatura
markup-style-strikethrough = Barrato
markup-style-squiggly = Ondulato
# Menu section headings.
markup-menu-color = Colore
markup-menu-font = Font
markup-menu-size = Dimensione
markup-menu-alignment = Allineamento
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Tratteggiata

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Nota
markup-kind-text-box = Casella di testo
markup-kind-stamp = Timbro
markup-kind-redaction = Oscuramento
markup-kind-shape = Forma
# Tooltips on a note being edited.
markup-note-delete = Elimina nota
markup-note-done = Fine
markup-note-placeholder = Scrivi una nota
markup-notes-empty = Nessuna evidenziazione o nota
markup-notes-empty-hint = Evidenziazioni, note e caselle di testo compaiono qui.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Pagina { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Impossibile modificare il documento: { $error }
markup-copy-area-failed = Impossibile copiare l'area: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = il documento è stato chiuso
markup-render-area-failed = impossibile elaborare l'area
markup-copy-stopped = la copia si è interrotta

## Signatures

signature-menu-empty = Ancora nessuna firma.
signature-delete = Elimina firma
signature-create = Crea firma…
signature-dialog-title = Crea firma
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Disegna
signature-tab-type = Scrivi
signature-tab-image = Immagine
signature-draw-hint = Firma sulla linea con il mouse, la penna o il trackpad.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Il tuo nome
signature-image-hint = Scegli una foto o una scansione della tua firma su carta bianca.
signature-choose-image = Scegli immagine…
# Placeholder of the field naming the signature in the library.
signature-description = Descrizione, ad esempio Nome completo o Iniziali
# Clears the drawing, typed name or image.
signature-clear = Cancella
# The color the signature is drawn or typed in.
signature-ink = Inchiostro
# The pen's width, for drawing.
signature-thickness = Spessore
signature-sign-first = Prima firma, poi salva.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Firma { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Impossibile modificare le firme: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = nessuna cartella dati: HOME non è impostata
signature-removing-stopped = la rimozione si è interrotta
signature-saving-stopped = il salvataggio si è interrotto
signature-reading-stopped = la lettura si è interrotta
signature-not-an-image = quel file non è un'immagine che prev può leggere
signature-no-frames = l'immagine non ha fotogrammi
signature-not-found = nessuna firma trovata nell'immagine

## Dragging

drag-pages-need-document = Le pagine si possono rilasciare su un documento.
drag-image-unsupported = prev non può aprire questa immagine.
# $error is a lowercase reason or a technical message.
drag-area-failed = Impossibile trascinare l'area: { $error }
drag-pages-failed = Impossibile trascinare le pagine: { $error }
drag-start-failed = Impossibile iniziare il trascinamento.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Pagine
drag-file-one-page = { $name } (pagina { $page })
drag-file-page-range = { $name } (pagine { $first }–{ $last })

## PDF window

pdf-opening = Apertura…
pdf-open-failed = prev non può aprire questo documento
pdf-no-pages = Il documento non ha pagine.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = il documento è stato chiuso
pdf-keep-original-failed = impossibile conservare la versione originale: { $error }
pdf-save-failed = Impossibile salvare: { $error }
pdf-nothing-to-paste = Non c'è niente da incollare.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = l'operazione Incolla si è interrotta
pdf-file-dialog-failed = Impossibile mostrare la finestra dei file: { $error }
pdf-bookmarks-no-home = Impossibile salvare i segnalibri: HOME non è impostata
pdf-bookmarks-save-failed = Impossibile salvare i segnalibri: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Pagina { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” è protetto da password
pdf-password = Password
pdf-password-wrong = Password errata. Riprova.
# Button that opens a locked document.
pdf-unlock = Sblocca

# Toolbar tooltips and labels.
pdf-sidebar = Barra laterale
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = di { $count }
pdf-zoom-out = Riduci
pdf-zoom-in = Ingrandisci
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Adatta alla pagina
pdf-fit-width = Adatta alla larghezza
pdf-actual-size = Dimensioni reali
pdf-view-continuous = Scorrimento continuo
pdf-view-single-page = Pagina singola
pdf-view-two-pages = Due pagine
pdf-undo = Annulla
pdf-redo = Ripeti
pdf-rotate-left = Ruota a sinistra
pdf-rotate-right = Ruota a destra
pdf-inspector = Inspector
pdf-markup = Markup
# Tooltip of the button that opens the export dialog.
pdf-export = Esporta
pdf-settings = Impostazioni

# Search field.
pdf-search = Cerca
pdf-search-not-found = Nessun risultato
pdf-searching = Ricerca…
# The match shown, of all matches found.
pdf-search-match = { $current } di { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } di { $total }+

# Inspector: section headings.
pdf-inspector-file = File
pdf-inspector-document = Documento
pdf-inspector-pages = Pagine
# Inspector: fact labels and values.
pdf-inspector-title = Titolo
pdf-inspector-author = Autore
pdf-inspector-subject = Oggetto
pdf-inspector-keywords = Parole chiave
pdf-inspector-created = Creazione
pdf-inspector-modified = Modifica
pdf-inspector-application = Applicazione
pdf-inspector-producer = Produttore PDF
pdf-inspector-version = Versione
pdf-inspector-security = Sicurezza
pdf-inspector-not-encrypted = Non codificato
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Codificato ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } pagina
   *[other] { $count } pagine
}
pdf-inspector-page-size = Dimensioni pagina
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } in)
pdf-loading = Caricamento…

# Sidebar tabs and lists.
pdf-tab-pages = Pagine
pdf-tab-contents = Indice
pdf-tab-notes = Evidenziazioni e note
pdf-tab-bookmarks = Segnalibri
pdf-no-outline = Nessun indice
pdf-no-outline-detail = Questo documento non ha un indice.
pdf-no-bookmarks = Nessun segnalibro
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Premi { $keys } per aggiungere un segnalibro a una pagina.
pdf-remove-bookmark = Rimuovi segnalibro

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Pagine
pages-insert-blank = Inserisci pagina vuota
pages-insert-file = Inserisci da file…
pages-copy = { $count ->
    [one] Copia pagina
   *[other] Copia pagine
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Incolla pagina
   *[other] Incolla { $count } pagine
}
pages-crop = Ritaglia alla selezione
pages-select-all = Seleziona tutte le pagine
pages-delete = { $count ->
    [one] Elimina pagina
   *[other] Elimina pagine
}
pages-apply-redactions = Applica oscuramenti…
pages-no-copied = Non ci sono pagine copiate da incollare.
pages-copied = { $count ->
    [one] { $count } pagina copiata.
   *[other] { $count } pagine copiate.
}
pages-copy-failed = Impossibile copiare le pagine: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = la lettura si è interrotta
pages-read-failed = Impossibile leggere il file: { $error }
pages-at-least-one = Un documento deve avere almeno una pagina.
pages-crop-needs-area = Prima scegli un'area con lo strumento di selezione rettangolare.
pages-change-failed = Impossibile modificare le pagine: { $error }
pages-no-redactions = Non c'erano oscuramenti da applicare.
pages-redactions-applied = { $count ->
    [one] { $count } oscuramento applicato.
   *[other] { $count } oscuramenti applicati.
}
pages-forget-versions-failed = Impossibile eliminare le versioni precedenti: { $error }
pages-redact-title = Applicare gli oscuramenti?
pages-redact-body = { $count ->
    [one] Testo, immagini e disegni sotto il segno vengono rimossi definitivamente dal documento, e il segno diventa un riquadro nero. L'operazione non si può annullare, e le versioni precedenti di questo file conservate da prev vengono eliminate.
   *[other] Testo, immagini e disegni sotto i { $count } segni vengono rimossi definitivamente dal documento, e i segni diventano riquadri neri. L'operazione non si può annullare, e le versioni precedenti di questo file conservate da prev vengono eliminate.
}
# Button that applies redactions.
pages-redact-apply = Applica

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Esporta
pages-export-format = Formato
pages-export-reduce = Riduci dimensioni file (immagini a 150 dpi)
pages-export-flatten = Appiattisci annotazioni e campi dei moduli
pages-export-flatten-detail = Il markup e i campi compilati diventano parte delle pagine e non si possono più modificare. I segni di oscuramento non ancora applicati vengono esclusi.
pages-export-encrypt = Codifica con una password
pages-export-password = Password
pages-export-verify-password = Verifica password
pages-export-resolution = Risoluzione
pages-export-dpi = { $dpi } dpi
pages-export-quality = Qualità
# JPEG quality choices.
pages-export-quality-low = Bassa
pages-export-quality-medium = Media
pages-export-quality-high = Alta
pages-export-quality-best = Massima
pages-export-one-file = Tutte le pagine vanno in un unico file.
pages-export-file-per-page = Ogni pagina viene salvata in un file a sé, numerato a partire dal nome che scegli.
pages-export-selected-only = { $count ->
    [one] Solo la pagina selezionata
   *[other] Solo le { $count } pagine selezionate
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Esporta…
pages-export-no-password = Inserisci una password.
pages-export-password-mismatch = Le password non corrispondono.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (esportato)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = documento
pages-export-same-file = Esporta in un nuovo file; questo documento si salva da solo.
pages-export-exporting = Esportazione di “{ $name }”…
pages-export-done = “{ $name }” esportato.
pages-export-done-images = { $count } immagini esportate.
pages-export-failed = Impossibile esportare: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = l'esportazione si è interrotta

## Start window

# Under the app name in a window with no file open.
app-start-hint = Apri o trascina qui un file PDF, un'immagine, un file SVG o Markdown.
app-start-open = Apri…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (sviluppo)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: questo visore non è ancora pronto.
app-cannot-open = prev non può aprire questo tipo di file.
app-cannot-read = prev non può leggere questo file: { $error }
app-kind-pdf = Documento PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Immagine { $format }
app-kind-svg = Disegno SVG
app-kind-markdown = Documento Markdown
app-file-dialog-failed = Impossibile mostrare la finestra dei file: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Apri
action-settings = Impostazioni

## Toolbar

app-toolbar-keep-shown = Mantieni visibile la barra degli strumenti
app-toolbar-auto-hide = Nascondi la barra degli strumenti quando il puntatore esce
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Altro

## File facts

# Labels in a file's inspector.
app-fact-name = Nome
app-fact-folder = Cartella
app-fact-size = Dimensioni
app-fact-modified = Modifica
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } byte
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Link non valido { $uri }: { $error }
app-link-open-failed = Impossibile aprire { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = installa wl-clipboard per incollare immagini
app-copy-needs-wl-clipboard = installa wl-clipboard per copiare immagini
app-copy-no-pixels = l'area non ha pixel
# wl-copy is a program's name.
app-copy-no-input = wl-copy non ha ricevuto dati
app-copy-failed = wl-copy non è riuscito
app-clipboard-open-failed = Impossibile aprire gli appunti: { $error }
app-copy-image-failed = Impossibile copiare l'immagine: { $error }

## Printing

print-failed = Impossibile stampare: { $error }
print-stopped = La stampa si è interrotta
print-unavailable = La stampa non è ancora disponibile su questo sistema.
print-no-window = Impossibile stampare: nessuna finestra su cui mostrare la finestra di stampa
print-dialog-failed = Impossibile mostrare la finestra di stampa: { $error }
# Shown after "Could not print:".
print-job-not-started = la stampante non ha avviato il lavoro
# Shown after "Could not print:".
print-printer-stopped = la stampante si è fermata

## File dialogs

dialog-open = Apri
dialog-filter-all = Tutti i file supportati
dialog-filter-pdf = Documenti PDF
dialog-filter-images = Immagini
dialog-filter-svg = Disegni SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Scegli la cartella delle firme
dialog-choose-versions = Scegli la cartella della cronologia versioni
dialog-choose-bookmarks = Scegli il file dei segnalibri

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Uso: prev [FILE]...

    Visualizza e modifica PDF e immagini. I file si aprono in finestre di prev
    in esecuzione, che si avvia se necessario.

    Opzioni:
      -h, --help     Mostra questo aiuto
      -V, --version  Mostra la versione

## Settings, continued

settings-language = Lingua
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Predefinita di sistema: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Lingua di immissione
settings-input-language-system = Segui il layout della tastiera
settings-input-language-note = Imposta il lato da cui parte un campo di testo vuoto. Il testo che scrivi mantiene la propria direzione.

settings-appearance-system = Sistema
settings-appearance-light = Chiaro
settings-appearance-dark = Scuro
settings-omarchy-accent = Usa il colore principale di Omarchy
# $theme is the Omarchy theme's name.
settings-omarchy-note = I colori derivano dal colore principale di “{ $theme }”.
settings-omarchy-none = Nessun tema Omarchy attivo.
settings-auto-hide = Nascondi la barra degli strumenti quando il puntatore esce
settings-auto-hide-note = La barra degli strumenti fluttua sopra il documento e scivola via mentre il puntatore è fuori dalla finestra.
settings-animations = Animazioni
settings-animations-note = Barre e pannelli che scorrono, finestre che si espandono e pulsanti elastici.
settings-animations-reduced = Disattivate finché il sistema richiede di ridurre il movimento.
settings-corner-radius = Raggio degli angoli
settings-corner-radius-note = Per le finestre di dialogo e la barra degli strumenti fluttuante.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Trasparenza della sovrapposizione
settings-overlay-note = Quanto della pagina si vede attraverso la barra degli strumenti fluttuante.
settings-overlay-value = { $percent }%
settings-storage-signatures = Cartella delle firme
settings-storage-versions = Cartella della cronologia versioni
settings-storage-bookmarks = File dei segnalibri
settings-storage-apply = Applica
settings-storage-choose = Scegli…
# $file is where the settings file is.
settings-storage-note = I file già conservati in una posizione precedente restano lì; spostali per continuare a usarli. Le impostazioni di prev vengono salvate in { $file }.
settings-save-failed = Impossibile salvare le impostazioni: { $error }
settings-no-location = Nessuna posizione per le impostazioni: HOME non è impostata
settings-full-path = Usa un percorso completo, ad esempio ~/Documents/prev.
settings-path-is-folder = { $path } è una cartella, non un file.
settings-folder-missing = La cartella { $path } non esiste. Creala prima, oppure scegline una.
settings-path-is-file = { $path } è un file, non una cartella.
settings-cannot-write = prev non può scrivere in { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Esporta
# Section headings in the export dialog.
export-format = Formato
export-quality = Qualità
export-size = Dimensioni
# Button that goes on to choose where to save the export.
export-choose = Esporta…
# Format choice; the format name stays as it is.
export-format-webp = WebP (senza perdita)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = immagine
# JPEG quality choices.
export-quality-low = Bassa
export-quality-medium = Media
export-quality-high = Alta
export-quality-best = Massima
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Dimensioni reali
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } pixel
# $error is the system's reason.
export-dialog-failed = Impossibile mostrare la finestra di salvataggio: { $error }
# $path is where the file was saved.
export-done = Esportato in { $path }
export-failed = Impossibile esportare: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = l'esportazione si è interrotta

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Le immagini con markup non si possono modificare. Esporta per conservare il markup, oppure eliminalo e chiudi la barra Markup.

# Shown if a background task ends unexpectedly.
image-loading-stopped = il caricamento si è interrotto
image-reverting-stopped = il ripristino si è interrotto
image-rendering-stopped = l'elaborazione si è interrotta
image-saving-stopped = il salvataggio si è interrotto
image-markup-stopped = il markup si è interrotto
image-no-version-store = Nessuna posizione in cui conservare le versioni
image-revert-failed = Impossibile ripristinare: { $error }
image-read-failed = Impossibile leggere { $path }: { $error }
image-keep-original-failed = Impossibile conservare la versione originale: { $error }
image-save-failed = Impossibile salvare { $path }: { $error }
image-markup-start-failed = Impossibile avviare il markup: { $error }
image-cannot-edit = Le animazioni e i disegni SVG non si possono modificare.
image-cannot-mark-up = Le animazioni e i disegni SVG non si possono annotare.
image-mark-up-wait = Attendi la fine della modifica, poi annota.
image-crop-needs-selection = Prima trascina una selezione (strumento Seleziona), poi ritaglia.
image-size-needed = Inserisci larghezza e altezza in pixel.
# $name is a file name.
image-cannot-save-format = Le modifiche a “{ $name }” non si possono salvare nel suo formato. Usa Esporta ({ $keys }).
image-cannot-export-animation = Le animazioni non si possono ancora esportare.
image-drop-pages = Le pagine si possono rilasciare su un documento.
image-drag-failed = Impossibile iniziare il trascinamento.
image-open-failed = prev non può aprire questa immagine
image-opening = Apertura…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Il nome non corrisponde al formato
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” verrà salvato come file { $format }, ma il suo nome termina con .{ $extension }. Altre app potrebbero non aprirlo.
image-name-mismatch-no-extension = “{ $name }” verrà salvato come file { $format }, ma il suo nome non ha estensione. Altre app potrebbero non aprirlo.
image-choose-again = Scegli di nuovo
image-save-as-is = Salva così
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = fotogramma { $current } di { $total }
image-position = { $current } di { $total }
image-edited = modificata
# Toolbar tooltips.
image-sidebar = Barra laterale
image-zoom-out = Riduci
image-zoom-in = Ingrandisci
image-zoom = { $percent }%
image-fit = Adatta alla finestra
image-actual-size = Dimensioni reali
image-undo = Annulla
image-redo = Ripeti
image-rotate-left = Ruota a sinistra
image-rotate-right = Ruota a destra
image-flip-horizontal = Rifletti orizzontalmente
image-flip-vertical = Rifletti verticalmente
image-select = Selezione rettangolare
image-crop = Ritaglia alla selezione
image-adjust-size-tool = Regola dimensioni
image-adjust-color-tool = Regola colore
# Tooltip and panel title.
image-inspector = Inspector
image-markup = Markup
image-export = Esporta
image-settings = Impostazioni
# Panel titles.
image-adjust-color = Regola colore
image-adjust-size = Regola dimensioni
# Adjust Color sliders.
image-exposure = Esposizione
image-contrast = Contrasto
image-saturation = Saturazione
image-temperature = Temperatura
image-tint = Tinta
image-sepia = Seppia
image-sharpness = Nitidezza
image-levels = Livelli
image-black-point = Punto del nero
image-midtones = Mezzitoni
image-white-point = Punto del bianco
image-reset-all = Ripristina tutto
# Adjust Size panel.
image-current-size = Dimensioni attuali: { $width } × { $height } pixel
image-width = Larghezza
image-height = Altezza
image-scale-proportionally = Ridimensiona in proporzione
# Button that applies the new size.
image-resize = Ridimensiona
# Inspector panel.
image-inspector-loading = Caricamento…
image-file = File
image-format = Formato
image-dimensions-label = Dimensioni
image-pixels = { $width } × { $height } pixel
image-no-camera = Nessuna informazione sulla fotocamera.
image-location = Posizione
image-remove-location = Rimuovi informazioni sulla posizione
image-no-location = Nessuna informazione sulla posizione.
image-keywords-description = Parole chiave e descrizione
image-keywords-hint = Parole chiave, separate da virgole
image-description = Descrizione
image-keywords-unsupported = Le parole chiave si possono salvare nei file JPEG, PNG e WebP.
# Heading over the earlier versions of the file.
image-revert-to = Ripristina a
image-no-versions = Nessuna versione precedente.
image-revert = Ripristina
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Chiudere senza esportare il markup?
image-close-body = { $count ->
    [one] Il markup su un'immagine dura solo finché la sua finestra è aperta. Esporta l'immagine per conservarlo: il markup viene disegnato nella copia che salvi.
   *[other] Il markup sulle immagini dura solo finché la loro finestra è aperta. Esporta ogni immagine per conservarlo: il markup viene disegnato nella copia che salvi.
}
image-close-anyway = Chiudi comunque

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = la lettura si è interrotta
markdown-read-failed = prev non può leggere questo file
markdown-draw-failed = Impossibile disegnare il documento
# Under the export's size choices.
markdown-export-size = L'intero documento, { $width } × { $height } pixel
# Search results.
markdown-not-found = Nessun risultato
markdown-match = { $current } di { $total }
# Placeholder of the search field.
markdown-search = Cerca
# Toolbar tooltips.
markdown-smaller-text = Testo più piccolo
markdown-larger-text = Testo più grande
markdown-zoom = { $percent }%
markdown-actual-size = Dimensioni reali
# Tooltip and panel title.
markdown-inspector = Inspector
markdown-export = Esporta
markdown-settings = Impostazioni
# Inspector headings and labels.
markdown-file = File
markdown-document = Documento
markdown-words = Parole
markdown-lines = Righe
markdown-pictures = Immagini

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Fotocamera
image-meta-exposure = Esposizione
image-meta-image = Immagine
image-meta-make = Marca
image-meta-model = Modello
image-meta-lens = Obiettivo
image-meta-exposure-time = Tempo di esposizione
# The lens aperture, written like f/2.8.
image-meta-f-number = Numero f
image-meta-iso = ISO
image-meta-focal-length = Lunghezza focale
image-meta-exposure-bias = Compensazione dell'esposizione
image-meta-flash = Flash
image-meta-date-taken = Data dello scatto
image-meta-orientation = Orientamento
image-meta-color-space = Spazio colore
image-meta-software = Software
image-meta-artist = Artista
image-meta-copyright = Copyright
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normale
    [2] Riflessa orizzontalmente
    [3] Ruotata di 180°
    [4] Riflessa verticalmente
    [5] Riflessa orizzontalmente, ruotata di 90° in senso antiorario
    [6] Ruotata di 90° in senso orario
    [7] Riflessa orizzontalmente, ruotata di 90° in senso orario
    [8] Ruotata di 90° in senso antiorario
   *[other] Sconosciuta ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Scattato
   *[no] Non scattato
}{ $mode ->
    [on] , forzato
    [off] , disattivato
    [auto] , automatico
   *[unknown] {""}
}{ $redeye ->
    [yes] , riduzione occhi rossi
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Non calibrato
   *[other] Altro ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = impossibile aprire il documento: { $detail }
error-pdf-page-out-of-range = la pagina { $page } non esiste
error-pdf-password-protected = il documento è protetto da password; aprilo e copiane invece le pagine
error-pdf-no-pages = nessuna pagina da estrarre
error-pdf-crop-outside = l'area di ritaglio è fuori dalla pagina
error-pdf-closed = documento chiuso
error-pdf-saved-unreadable = il documento salvato non si apre più
error-image-read = impossibile leggere il file: { $detail }
error-image-invalid = l'immagine è danneggiata o non valida: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = per aprire questo formato serve { $library }, che non è installato
# $format is an image format name, such as HEIC.
error-image-unsupported = le immagini { $format } non sono ancora supportate
error-image-encode = impossibile codificare l'immagine: { $detail }
error-exif-malformed = i dati EXIF non sono validi
error-settings-read = impossibile leggere le impostazioni: { $detail }
error-settings-invalid = impostazioni non valide: { $detail }
error-remove-location = impossibile rimuovere la posizione: { $error }
error-location-unsupported = le informazioni sulla posizione si possono rimuovere dai file JPEG, PNG, WebP e TIFF
error-xmp-unsupported = parole chiave e descrizioni si possono salvare solo nei file JPEG, PNG e WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW della fotocamera

## The macOS menu bar, named as in macOS's own apps.
menu-about = Informazioni su prev
menu-settings = Impostazioni…
menu-services = Servizi
menu-hide = Nascondi prev
menu-hide-others = Nascondi altre
menu-show-all = Mostra tutte
menu-quit = Esci da prev
menu-file = File
menu-open = Apri…
menu-close = Chiudi finestra
menu-export = Esporta…
menu-print = Stampa…
menu-edit = Modifica
menu-undo = Annulla
menu-redo = Ripeti
menu-cut = Taglia
menu-copy = Copia
menu-paste = Incolla
menu-select-all = Seleziona tutto
menu-find = Trova
menu-find-next = Trova successivo
menu-find-previous = Trova precedente
menu-view = Vista
menu-hide-sidebar = Nascondi barra laterale
menu-thumbnails = Miniature
menu-contents = Indice
menu-notes = Evidenziazioni e note
menu-bookmarks = Segnalibri
menu-zoom-in = Ingrandisci
menu-zoom-out = Riduci
menu-actual-size = Dimensioni reali
menu-zoom-to-fit = Adatta alla finestra
menu-inspector = Mostra Inspector
menu-slideshow = Presentazione
menu-full-screen = Attiva modalità a tutto schermo
menu-go = Vai
menu-next-page = Pagina successiva
menu-previous-page = Pagina precedente
menu-go-to-page = Vai alla pagina…
menu-bookmark = Aggiungi segnalibro
menu-tools = Strumenti
menu-markup = Mostra barra strumenti Markup
menu-rotate-left = Ruota a sinistra
menu-rotate-right = Ruota a destra
menu-crop = Ritaglia
menu-adjust-color = Regola colore…
menu-window = Finestra
menu-minimize = Contrai
menu-zoom = Zoom
menu-bring-all-to-front = Porta tutto in primo piano
