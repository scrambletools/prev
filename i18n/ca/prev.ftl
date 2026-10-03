# prev's interface text in Catalan (Català), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = marques (barra de marques), note = nota,
# highlight = ressaltat, annotation = anotació, redact/redaction = censura,
# inspector = inspector, zoom in/out = amplia/redueix, bookmark = marcador,
# page = pàgina, file = fitxer, settings = configuració.
# Buttons and menu items use the imperative (Desa, Cancel·la, Tanca), as in
# Softcatalà's guide; messages address the user as tu.

## Language

language-name = Català

## Common

common-cancel = Cancel·la
common-close = Tanca
common-save = Desa

## Settings

settings-title = Configuració
settings-appearance = Aparença
settings-colors = Colors
settings-windows = Finestres
settings-default-app = Aplicació per defecte
settings-default-app-label = Obre els fitxers amb el prev
settings-default-app-note = Fes que el prev sigui l'aplicació que obre els PDF, les imatges, els dibuixos SVG i els fitxers Markdown.
settings-default-app-note-windows = El Windows només et deixa triar les aplicacions per defecte a la seva pròpia Configuració. Això hi obre la pàgina del prev.
settings-default-app-note-macos = El macOS et demana que confirmis cada tipus: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP i AVIF.
settings-default-app-status = { $set } de { $total } tipus de fitxer s'obren amb el prev.
settings-default-app-button = Estableix per defecte
settings-default-app-button-windows = Obre la Configuració
settings-default-app-no-entry = L'entrada d'escriptori del prev no està instal·lada, de manera que el sistema no hi pot obrir fitxers. Instal·la el prev des d'un paquet o amb scripts/install.sh.
settings-default-app-no-bundle = Obre el prev des de prev.app per establir-lo per defecte.
settings-default-app-failed = No s'ha pogut establir el prev per defecte: { $error }
settings-storage = Emmagatzematge
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (versió de desenvolupament, { $build })

## Markup toolbar

markup-tool-select = Selecciona
markup-tool-area = Selecció rectangular
markup-tool-sketch = Esbós
markup-tool-draw = Dibuixa
markup-tool-shapes = Formes
markup-tool-text-box = Quadre de text
markup-tool-highlight = Ressalta
markup-tool-note = Nota
markup-tool-sign = Signa
markup-tool-redact = Censura
markup-apply = Aplica
markup-apply-redactions = Aplica les censures
markup-shape-style = Estil de la forma
markup-border-color = Color de la vora
markup-fill-color = Color d'emplenament
markup-text-style = Estil del text
markup-delete = Suprimeix
markup-undo = Desfés
markup-redo = Refés

## Markup menus

markup-shape-rectangle = Rectangle
markup-shape-rounded-rectangle = Rectangle arrodonit
markup-shape-oval = Oval
markup-shape-line = Línia
markup-shape-arrow = Fletxa
markup-shape-star = Estrella
markup-shape-polygon = Polígon
markup-shape-speech-bubble = Bafarada
markup-shape-loupe = Lupa
markup-shape-mask = Màscara
markup-style-highlight = Ressaltat
markup-style-underline = Subratllat
markup-style-strikethrough = Ratllat
markup-style-squiggly = Ondulat
markup-menu-color = Color
markup-menu-font = Tipus de lletra
markup-menu-size = Mida
markup-menu-alignment = Alineació
markup-line-width = { $width } pt
markup-dashed = Discontínua

## Notes

markup-kind-note = Nota
markup-kind-text-box = Quadre de text
markup-kind-stamp = Segell
markup-kind-redaction = Censura
markup-kind-shape = Forma
markup-note-delete = Suprimeix la nota
markup-note-done = Fet
markup-note-placeholder = Escriu una nota
markup-notes-empty = No hi ha ressaltats ni notes
markup-notes-empty-hint = Els ressaltats, les notes i els quadres de text apareixen aquí.
markup-notes-page = Pàgina { $page }

## Markup errors

markup-change-failed = No s'ha pogut modificar el document: { $error }
markup-copy-area-failed = No s'ha pogut copiar l'àrea: { $error }
markup-document-closed = el document s'ha tancat
markup-render-area-failed = no s'ha pogut representar l'àrea
markup-copy-stopped = la còpia s'ha aturat

## Signatures

signature-menu-empty = Encara no hi ha signatures.
signature-delete = Suprimeix la signatura
signature-create = Crea una signatura…
signature-dialog-title = Crea una signatura
signature-tab-draw = Dibuixa
signature-tab-type = Escriu
signature-tab-image = Imatge
signature-draw-hint = Signa sobre la línia amb el ratolí, el llapis o el ratolí tàctil.
signature-your-name = El teu nom
signature-image-hint = Tria una foto o un escaneig de la teva signatura sobre paper blanc.
signature-choose-image = Tria una imatge…
signature-description = Descripció, com ara Nom complet o Inicials
signature-clear = Esborra
signature-ink = Tinta
signature-thickness = Gruix
signature-sign-first = Primer signa i després desa.
signature-default-name = Signatura { $number }
signature-change-failed = No s'han pogut modificar les signatures: { $error }
signature-no-data-folder = no hi ha carpeta de dades: HOME no està definit
signature-removing-stopped = la supressió s'ha aturat
signature-saving-stopped = el desament s'ha aturat
signature-reading-stopped = la lectura s'ha aturat
signature-not-an-image = aquest fitxer no és una imatge que el prev pugui llegir
signature-no-frames = la imatge no té fotogrames
signature-not-found = no s'ha trobat cap signatura a la imatge

## Dragging

drag-pages-need-document = Les pàgines es poden deixar anar sobre un document.
drag-image-unsupported = El prev no pot obrir aquesta imatge.
drag-area-failed = No s'ha pogut arrossegar l'àrea: { $error }
drag-pages-failed = No s'han pogut arrossegar les pàgines: { $error }
drag-start-failed = No s'ha pogut començar a arrossegar.
drag-file-pages = Pàgines
drag-file-one-page = { $name } (pàgina { $page })
drag-file-page-range = { $name } (pàgines { $first }–{ $last })
drag-file-image = Imatge
drop-pdf-title = Vols afegir-ho a aquest document?
drop-pdf-body = Vols afegir «{ $name }» al final d'aquest document o obrir-lo en una finestra pròpia?
drop-pdfs-body = { $count ->
    [one] Vols afegir aquest PDF al final d'aquest document o obrir-lo en una finestra pròpia?
   *[other] Vols afegir aquests { $count } PDF al final d'aquest document o obrir-los en finestres pròpies?
}
drop-pdf-add = Afegeix al final
drop-pdf-open = Obre a part

## PDF window

pdf-opening = S'està obrint…
pdf-open-failed = El prev no pot obrir aquest document
pdf-no-pages = El document no té pàgines.
pdf-document-closed = el document s'ha tancat
pdf-keep-original-failed = no s'ha pogut conservar la versió original: { $error }
pdf-save-failed = No s'ha pogut desar: { $error }
pdf-nothing-to-paste = No hi ha res per enganxar.
pdf-pasting-stopped = l'enganxament s'ha aturat
pdf-file-dialog-failed = No s'ha pogut mostrar el diàleg de fitxers: { $error }
pdf-bookmarks-no-home = No es poden desar els marcadors: HOME no està definit
pdf-bookmarks-save-failed = No s'han pogut desar els marcadors: { $error }
pdf-bookmark-page = Pàgina { $page }

pdf-password-protected = «{ $name }» està protegit amb contrasenya
pdf-password = Contrasenya
pdf-password-wrong = La contrasenya no és correcta. Torna-ho a provar.
pdf-unlock = Desbloqueja

pdf-sidebar = Barra lateral
pdf-page-of = de { $count }
pdf-zoom-out = Redueix
pdf-zoom-in = Amplia
pdf-zoom-percent = { $percent } %
pdf-fit-page = Ajusta a la pàgina
pdf-fit-width = Ajusta a l'amplada
pdf-actual-size = Mida real
pdf-view-continuous = Desplaçament continu
pdf-view-single-page = Una sola pàgina
pdf-view-two-pages = Dues pàgines
pdf-undo = Desfés
pdf-redo = Refés
pdf-rotate-left = Gira a l'esquerra
pdf-rotate-right = Gira a la dreta
pdf-inspector = Inspector
pdf-markup = Marques
pdf-export = Exporta
pdf-settings = Configuració

pdf-search = Cerca
pdf-search-not-found = No s'ha trobat
pdf-searching = S'està cercant…
pdf-search-match = { $current } de { $total }
pdf-search-match-more = { $current } de { $total }+

pdf-inspector-file = Fitxer
pdf-inspector-document = Document
pdf-inspector-pages = Pàgines
pdf-inspector-title = Títol
pdf-inspector-author = Autor
pdf-inspector-subject = Assumpte
pdf-inspector-keywords = Paraules clau
pdf-inspector-created = Creació
pdf-inspector-modified = Modificació
pdf-inspector-application = Aplicació
pdf-inspector-producer = Productor del PDF
pdf-inspector-version = Versió
pdf-inspector-security = Seguretat
pdf-inspector-not-encrypted = Sense xifrar
pdf-inspector-encrypted = Xifrat ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } pàgina
   *[other] { $count } pàgines
}
pdf-inspector-page-size = Mida de la pàgina
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } polz.)
pdf-loading = S'està carregant…

pdf-tab-pages = Pàgines
pdf-tab-contents = Índex
pdf-tab-notes = Ressaltats i notes
pdf-tab-bookmarks = Marcadors
pdf-no-outline = No hi ha índex
pdf-no-outline-detail = Aquest document no té índex.
pdf-no-bookmarks = No hi ha marcadors
pdf-no-bookmarks-detail = Prem { $keys } per afegir un marcador a una pàgina.
pdf-no-bookmarks-detail-unbound = Les pàgines amb marcador apareixen aquí.
pdf-remove-bookmark = Elimina el marcador

## Page editing

pages-menu = Pàgines
pages-insert-blank = Insereix una pàgina en blanc
pages-insert-file = Insereix des d'un fitxer…
pages-copy = { $count ->
    [one] Copia la pàgina
   *[other] Copia les pàgines
}
pages-paste = { $count ->
    [one] Enganxa la pàgina
   *[other] Enganxa { $count } pàgines
}
pages-crop = Retalla a la selecció
pages-select-all = Selecciona totes les pàgines
pages-delete = { $count ->
    [one] Suprimeix la pàgina
   *[other] Suprimeix les pàgines
}
pages-apply-redactions = Aplica les censures…
pages-no-copied = No hi ha pàgines copiades per enganxar.
pages-copied = { $count ->
    [one] S'ha copiat { $count } pàgina.
   *[other] S'han copiat { $count } pàgines.
}
pages-copy-failed = No s'han pogut copiar les pàgines: { $error }
pages-reading-stopped = la lectura s'ha aturat
pages-image-unreadable = no és una imatge que el prev pugui llegir
pages-read-failed = No s'ha pogut llegir el fitxer: { $error }
pages-at-least-one = Un document ha de tenir com a mínim una pàgina.
pages-crop-needs-area = Primer tria una àrea amb l'eina de selecció rectangular.
pages-change-failed = No s'han pogut modificar les pàgines: { $error }
pages-no-redactions = No hi havia censures per aplicar.
pages-redactions-applied = { $count ->
    [one] S'ha aplicat { $count } censura.
   *[other] S'han aplicat { $count } censures.
}
pages-forget-versions-failed = No s'han pogut suprimir les versions anteriors: { $error }
pages-redact-title = Vols aplicar les censures?
pages-redact-body = { $count ->
    [one] El text, les imatges i els dibuixos de sota la marca s'eliminen del document per sempre, i la marca es converteix en un quadre negre. No es pot desfer, i se suprimeixen les versions anteriors d'aquest fitxer que conserva el prev.
   *[other] El text, les imatges i els dibuixos de sota les { $count } marques s'eliminen del document per sempre, i les marques es converteixen en quadres negres. No es pot desfer, i se suprimeixen les versions anteriors d'aquest fitxer que conserva el prev.
}
pages-redact-apply = Aplica

## PDF export

pages-export-title = Exporta
pages-export-format = Format
pages-export-reduce = Redueix la mida del fitxer (imatges a 150 ppp)
pages-export-flatten = Aplana les anotacions i els camps de formulari
pages-export-flatten-detail = Les marques i els camps emplenats passen a formar part de les pàgines i ja no es poden editar. Les censures que encara no s'han aplicat queden fora.
pages-export-encrypt = Xifra amb una contrasenya
pages-export-password = Contrasenya
pages-export-verify-password = Confirma la contrasenya
pages-export-resolution = Resolució
pages-export-dpi = { $dpi } ppp
pages-export-quality = Qualitat
pages-export-quality-low = Baixa
pages-export-quality-medium = Mitjana
pages-export-quality-high = Alta
pages-export-quality-best = Màxima
pages-export-one-file = Totes les pàgines van a un sol fitxer.
pages-export-file-per-page = Cada pàgina es desa en un fitxer propi, numerat a partir del nom que triïs.
pages-export-selected-only = { $count ->
    [one] Només la pàgina seleccionada
   *[other] Només les { $count } pàgines seleccionades
}
pages-export-choose = Exporta…
pages-export-no-password = Introdueix una contrasenya.
pages-export-password-mismatch = Les contrasenyes no coincideixen.
pages-export-file-name = { $name } (exportat)
pages-export-untitled = document
pages-export-same-file = Exporta a un fitxer nou; aquest document es desa automàticament.
pages-export-exporting = S'està exportant «{ $name }»…
pages-export-done = S'ha exportat «{ $name }».
pages-export-done-images = { $count ->
    [one] S'ha exportat { $count } imatge.
   *[other] S'han exportat { $count } imatges.
}
pages-export-failed = No s'ha pogut exportar: { $error }
pages-export-stopped = l'exportació s'ha aturat

## Start window

app-start-hint = Obre o deixa anar aquí un fitxer PDF, una imatge, un dibuix SVG o un fitxer Markdown.
app-start-open = Obre…
app-title-dev = { $title } (desenvolupament)
app-viewer-missing = { $kind }: aquest visualitzador encara no està fet.
app-cannot-open = El prev no pot obrir aquest tipus de fitxer.
app-cannot-read = El prev no pot llegir aquest fitxer: { $error }
app-kind-pdf = Document PDF
app-kind-image = Imatge { $format }
app-kind-svg = Dibuix SVG
app-kind-markdown = Document Markdown
app-file-dialog-failed = No s'ha pogut mostrar el diàleg de fitxers: { $error }

## Actions

action-open = Obre
action-settings = Configuració

## Toolbar

app-toolbar-keep-shown = Mantén visible la barra d'eines
app-toolbar-auto-hide = Amaga la barra d'eines quan el punter surt
app-toolbar-more = Més

## File facts

app-fact-name = Nom
app-fact-folder = Carpeta
app-fact-size = Mida
app-fact-modified = Modificació
app-size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } bytes
}
app-size-kb = { $size } kB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = L'enllaç { $uri } no és vàlid: { $error }
app-link-open-failed = No s'ha pogut obrir { $uri }: { $error }
app-paste-needs-wl-clipboard = instal·la wl-clipboard per enganxar imatges
app-copy-needs-wl-clipboard = instal·la wl-clipboard per copiar imatges
app-copy-no-pixels = l'àrea no té píxels
app-copy-no-input = wl-copy no ha rebut cap entrada
app-copy-failed = wl-copy ha fallat
app-clipboard-open-failed = No s'ha pogut obrir el porta-retalls: { $error }
app-copy-image-failed = No s'ha pogut copiar la imatge: { $error }

## Printing

print-failed = No s'ha pogut imprimir: { $error }
print-stopped = La impressió s'ha aturat
print-unavailable = La impressió encara no està disponible en aquest sistema.
print-no-window = No s'ha pogut imprimir: no hi ha cap finestra on mostrar el diàleg d'impressió
print-dialog-failed = No s'ha pogut mostrar el diàleg d'impressió: { $error }
print-job-not-started = la impressora no ha començat la feina
print-printer-stopped = la impressora s'ha aturat

## File dialogs

dialog-open = Obre
dialog-filter-all = Tots els fitxers compatibles
dialog-filter-pdf = Documents PDF
dialog-filter-images = Imatges
dialog-filter-svg = Dibuixos SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Tria la carpeta de signatures
dialog-choose-versions = Tria la carpeta de l'historial de versions
dialog-choose-bookmarks = Tria el fitxer de marcadors

## Command line

usage-help =
    Ús: prev [FILE]...
        prev --mcp

    Mostra i edita PDF i imatges. Els fitxers s'obren en finestres del prev
    en execució, que s'inicia si cal.

    Opcions:
      -h, --help     Mostra aquesta ajuda
      -V, --version  Mostra la versió
          --mcp      Serveix MCP per stdin i stdout, perquè els agents d'IA controlin
                     el prev en execució

## Settings, continued

settings-language = Llengua
settings-language-system = Llengua del sistema: { $language }
settings-input-language = Llengua d'entrada
settings-input-language-system = Segueix la disposició del teclat
settings-input-language-note = Defineix el costat on comença un camp de text buit. El text que escrius manté la seva pròpia direcció.

settings-appearance-system = Sistema
settings-appearance-light = Clar
settings-appearance-dark = Fosc
settings-system-accent = Utilitza el color d'accent del sistema
settings-omarchy-note = Els colors es creen a partir de l'accent de «{ $theme }».
settings-system-accent-note = Els colors es creen a partir del color d'accent del sistema.
settings-system-accent-none = El sistema no té cap color d'accent, de manera que el prev utilitza el que s'ha triat a continuació.
settings-accent-chosen-note = Els colors es creen a partir del color triat a continuació.
settings-auto-hide = Amaga la barra d'eines quan el punter surt
settings-auto-hide-note = La barra d'eines sura sobre el document i s'amaga mentre el punter és fora de la finestra.
settings-animations = Animacions
settings-animations-note = Barres i panells que llisquen, diàlegs que creixen i botons elàstics.
settings-animations-reduced = Desactivades mentre el sistema demani moviment reduït.
settings-corner-radius = Radi de les cantonades
settings-corner-radius-note = Per als diàlegs i la barra d'eines flotant.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparència de la superposició
settings-overlay-note = Quanta part de la pàgina es veu a través de la barra d'eines flotant.
settings-overlay-value = { $percent } %
settings-storage-signatures = Carpeta de signatures
settings-storage-versions = Carpeta de l'historial de versions
settings-storage-bookmarks = Fitxer de marcadors
settings-storage-apply = Aplica
settings-storage-choose = Tria…
settings-storage-note = Els fitxers que ja es guarden en una ubicació antiga s'hi queden; mou-los si els vols continuar utilitzant. La configuració del prev es desa a { $file }.
settings-save-failed = No s'ha pogut desar la configuració: { $error }
settings-no-location = No hi ha ubicació per a la configuració: HOME no està definit
settings-full-path = Utilitza un camí complet, com ara ~/Documents/prev.
settings-path-is-folder = { $path } és una carpeta, no un fitxer.
settings-folder-missing = La carpeta { $path } no existeix. Crea-la primer o tria'n una altra.
settings-path-is-file = { $path } és un fitxer, no una carpeta.
settings-cannot-write = El prev no pot escriure a { $path }: { $error }.

## Export dialog

export-title = Exporta
export-format = Format
export-quality = Qualitat
export-size = Mida
export-choose = Exporta…
export-format-webp = WebP (sense pèrdua)
export-format-unknown = imatge
export-quality-low = Baixa
export-quality-medium = Mitjana
export-quality-high = Alta
export-quality-best = Màxima
export-size-actual = Mida real
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } píxels
export-dialog-failed = No s'ha pogut mostrar el diàleg de desament: { $error }
export-done = S'ha exportat a { $path }
export-failed = No s'ha pogut exportar: { $error }
export-stopped = l'exportació s'ha aturat

## Image window

image-marked-no-edit = Les imatges amb marques no es poden editar. Exporta la imatge per conservar les marques, o suprimeix-les i tanca la barra de marques.

image-loading-stopped = la càrrega s'ha aturat
image-reverting-stopped = la reversió s'ha aturat
image-rendering-stopped = la representació s'ha aturat
image-saving-stopped = el desament s'ha aturat
image-markup-stopped = el marcatge s'ha aturat
image-no-version-store = No hi ha cap lloc on guardar les versions
image-revert-failed = No s'ha pogut revertir: { $error }
image-read-failed = No s'ha pogut llegir { $path }: { $error }
image-keep-original-failed = No s'ha pogut conservar la versió original: { $error }
image-save-failed = No s'ha pogut desar { $path }: { $error }
image-markup-start-failed = No s'han pogut iniciar les marques: { $error }
image-cannot-edit = Les animacions i els dibuixos SVG no es poden editar.
image-cannot-mark-up = Les animacions i els dibuixos SVG no admeten marques.
image-mark-up-wait = Espera que acabi l'edició i després afegeix-hi marques.
image-crop-needs-selection = Primer arrossega una selecció (eina Selecciona) i després retalla.
image-size-needed = Introdueix una amplada i una alçada en píxels.
image-cannot-save-format = Els canvis a «{ $name }» no es poden desar en el seu format. Utilitza Exporta ({ $keys }).
image-cannot-save-format-unbound = Els canvis a «{ $name }» no es poden desar en el seu format. Utilitza Exporta.
image-cannot-export-animation = Encara no es poden exportar animacions.
image-drop-pages = Les pàgines es poden deixar anar sobre un document.
image-drag-failed = No s'ha pogut començar a arrossegar.
image-picture-save-failed = No s'ha pogut desar la imatge a la carpeta Baixades.
image-open-failed = El prev no pot obrir aquesta imatge
image-opening = S'està obrint…
image-name-mismatch-title = El nom no coincideix amb el format
image-name-mismatch = «{ $name }» es desarà com a fitxer { $format }, però el nom acaba en .{ $extension }. És possible que altres aplicacions no l'obrin.
image-name-mismatch-no-extension = «{ $name }» es desarà com a fitxer { $format }, però el nom no té extensió. És possible que altres aplicacions no l'obrin.
image-choose-again = Torna a triar
image-save-as-is = Desa-ho igualment
image-dimensions = { $width } × { $height }
image-frame-position = fotograma { $current } de { $total }
image-position = { $current } de { $total }
image-edited = editada
image-sidebar = Barra lateral
image-zoom-out = Redueix
image-zoom-in = Amplia
image-zoom = { $percent } %
image-fit = Ajusta a la finestra
image-actual-size = Mida real
image-undo = Desfés
image-redo = Refés
image-rotate-left = Gira a l'esquerra
image-rotate-right = Gira a la dreta
image-flip-horizontal = Inverteix horitzontalment
image-flip-vertical = Inverteix verticalment
image-select = Selecció rectangular
image-crop = Retalla a la selecció
image-adjust-size-tool = Ajusta la mida
image-adjust-color-tool = Ajusta el color
image-inspector = Inspector
image-markup = Marques
image-export = Exporta
image-settings = Configuració
image-adjust-color = Ajusta el color
image-adjust-size = Ajusta la mida
image-exposure = Exposició
image-contrast = Contrast
image-saturation = Saturació
image-temperature = Temperatura
image-tint = Tonalitat
image-sepia = Sèpia
image-sharpness = Nitidesa
image-levels = Nivells
image-black-point = Punt negre
image-midtones = Tons mitjans
image-white-point = Punt blanc
image-reset-all = Restableix-ho tot
image-current-size = Mida actual: { $width } × { $height } píxels
image-width = Amplada
image-height = Alçada
image-scale-proportionally = Escala proporcionalment
image-resize = Redimensiona
image-inspector-loading = S'està carregant…
image-file = Fitxer
image-format = Format
image-dimensions-label = Dimensions
image-pixels = { $width } × { $height } píxels
image-no-camera = No hi ha informació de la càmera.
image-location = Ubicació
image-remove-location = Elimina la informació d'ubicació
image-no-location = No hi ha informació d'ubicació.
image-keywords-description = Paraules clau i descripció
image-keywords-hint = Paraules clau, separades per comes
image-description = Descripció
image-keywords-unsupported = Les paraules clau es poden desar en fitxers JPEG, PNG i WebP.
image-revert-to = Reverteix a
image-no-versions = No hi ha versions anteriors.
image-revert = Reverteix
image-size-kb = { $size } kB
image-size-mb = { $size } MB
image-close-title = Vols tancar sense exportar les marques?
image-close-body = { $count ->
    [one] Les marques d'una imatge només duren mentre la seva finestra és oberta. Exporta la imatge per conservar-les: les marques es dibuixen a la còpia que desis.
   *[other] Les marques de les imatges només duren mentre la seva finestra és oberta. Exporta cada imatge per conservar-les: les marques es dibuixen a la còpia que desis.
}
image-close-anyway = Tanca igualment

## Markdown

markdown-reading-stopped = la lectura s'ha aturat
markdown-read-failed = El prev no pot llegir aquest fitxer
markdown-draw-failed = No s'ha pogut dibuixar el document
markdown-export-size = Tot el document, { $width } × { $height } píxels
markdown-not-found = No s'ha trobat
markdown-match = { $current } de { $total }
markdown-search = Cerca
markdown-smaller-text = Text més petit
markdown-larger-text = Text més gran
markdown-zoom = { $percent } %
markdown-actual-size = Mida real
markdown-inspector = Inspector
markdown-export = Exporta
markdown-settings = Configuració
markdown-file = Fitxer
markdown-document = Document
markdown-words = Paraules
markdown-lines = Línies
markdown-pictures = Imatges

## Image details

image-meta-camera = Càmera
image-meta-exposure = Exposició
image-meta-image = Imatge
image-meta-make = Fabricant
image-meta-model = Model
image-meta-lens = Objectiu
image-meta-exposure-time = Temps d'exposició
image-meta-f-number = Nombre f
image-meta-iso = ISO
image-meta-focal-length = Distància focal
image-meta-exposure-bias = Compensació d'exposició
image-meta-flash = Flaix
image-meta-date-taken = Data de la presa
image-meta-orientation = Orientació
image-meta-color-space = Espai de color
image-meta-software = Programari
image-meta-artist = Autor
image-meta-copyright = Drets d'autor
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Reflectida horitzontalment
    [3] Girada 180°
    [4] Reflectida verticalment
    [5] Reflectida horitzontalment, girada 90° en sentit antihorari
    [6] Girada 90° en sentit horari
    [7] Reflectida horitzontalment, girada 90° en sentit horari
    [8] Girada 90° en sentit antihorari
   *[other] Desconeguda ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Disparat
   *[no] No disparat
}{ $mode ->
    [on] , forçat
    [off] , desactivat
    [auto] , automàtic
   *[unknown] {""}
}{ $redeye ->
    [yes] , reducció d'ulls vermells
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Sense calibrar
   *[other] Altre ({ $code })
}

## Errors

error-pdf-open = no es pot obrir el document: { $detail }
error-pdf-page-out-of-range = la pàgina { $page } no existeix
error-pdf-password-protected = el document està protegit amb contrasenya; obre'l i copia'n les pàgines
error-pdf-no-pages = no hi ha pàgines per extreure
error-pdf-crop-outside = l'àrea de retall és fora de la pàgina
error-pdf-closed = el document s'ha tancat
error-pdf-saved-unreadable = el document desat ja no s'obre
error-image-read = no es pot llegir el fitxer: { $detail }
error-image-invalid = la imatge està malmesa o no és vàlida: { $detail }
error-image-missing-library = per obrir aquest format cal { $library }, que no està instal·lat
error-image-unsupported = les imatges { $format } encara no són compatibles
error-image-encode = no es pot codificar la imatge: { $detail }
error-exif-malformed = les dades EXIF estan mal formades
error-settings-read = no es pot llegir la configuració: { $detail }
error-settings-invalid = la configuració no és vàlida: { $detail }
error-remove-location = no s'ha pogut eliminar la ubicació: { $error }
error-location-unsupported = la informació d'ubicació es pot eliminar dels fitxers JPEG, PNG, WebP i TIFF
error-xmp-unsupported = les paraules clau i les descripcions només es poden desar en fitxers JPEG, PNG i WebP

## Formats

format-camera-raw = RAW de càmera

## The macOS menu bar, named as in macOS's own apps.
menu-about = Quant al prev
menu-settings = Configuració…
menu-services = Serveis
menu-hide = Amaga el prev
menu-hide-others = Amaga els altres
menu-show-all = Mostra-ho tot
menu-quit = Surt del prev
menu-file = Fitxer
menu-open = Obre…
menu-close = Tanca la finestra
menu-export = Exporta…
menu-print = Imprimeix…
menu-edit = Edició
menu-undo = Desfés
menu-redo = Refés
menu-cut = Retalla
menu-copy = Copia
menu-paste = Enganxa
menu-select-all = Selecciona-ho tot
menu-find = Cerca
menu-find-next = Cerca el següent
menu-find-previous = Cerca l'anterior
menu-view = Visualització
menu-hide-sidebar = Amaga la barra lateral
menu-thumbnails = Miniatures
menu-contents = Índex
menu-notes = Ressaltats i notes
menu-bookmarks = Marcadors
menu-zoom-in = Amplia
menu-zoom-out = Redueix
menu-actual-size = Mida real
menu-zoom-to-fit = Ajusta a la finestra
menu-inspector = Mostra l'inspector
menu-slideshow = Presentació
menu-full-screen = Entra a pantalla completa
menu-go = Vés
menu-next-page = Pàgina següent
menu-previous-page = Pàgina anterior
menu-go-to-page = Vés a la pàgina…
menu-bookmark = Afegeix un marcador
menu-tools = Eines
menu-markup = Mostra la barra de marques
menu-rotate-left = Gira a l'esquerra
menu-rotate-right = Gira a la dreta
menu-crop = Retalla
menu-adjust-color = Ajusta el color…
menu-window = Finestra
menu-minimize = Minimitza
menu-zoom = Redimensiona
menu-bring-all-to-front = Porta-ho tot al davant

## Outside control

settings-outside-control = Control extern
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = General
settings-tab-agents = Agents
settings-allow-outside-control = Permet el control extern
settings-allow-outside-control-note = Els agents d'IA com ara Claude Code poden llegir i canviar els teus fitxers al prev mitjançant prev --mcp. El prev pregunta abans de cada agent nou.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permesos: { $agents }
settings-forget-agents = Oblida
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Vols permetre que { $agent } controli el prev?
agent-prompt-body = { $agent } demana fer servir el control extern del prev per llegir els fitxers oberts i canviar-los. Pots desactivar el control extern a Configuració.
agent-prompt-allow = Permet
agent-prompt-deny = No permetis
settings-ask-before-note = Pregunta abans que un agent:
settings-ask-reading = Llegeixi un fitxer
settings-ask-viewing = Canviï la visualització o una finestra
settings-ask-marking-up = Afegeixi marques a un fitxer
settings-ask-editing = Editi un fitxer
settings-ask-signing = Signi un fitxer
settings-ask-redacting = Apliqui censures
settings-ask-exporting = Exporti un fitxer
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Vols permetre que { $agent } llegeixi aquest fitxer?
agent-ask-view = Vols permetre que { $agent } canviï la visualització?
agent-ask-markup = Vols permetre que { $agent } afegeixi marques a aquest fitxer?
agent-ask-edit = Vols permetre que { $agent } editi aquest fitxer?
agent-ask-sign = Vols permetre que { $agent } signi aquest fitxer?
agent-ask-redact = Vols permetre que { $agent } apliqui censures?
agent-ask-export = Vols permetre que { $agent } exporti aquest fitxer?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } demana fer servir «{ $tool }». A Configuració pots triar què pregunta el prev.
agent-ask-final = Aquesta acció no es pot desfer.

## The assistant

settings-tab-assistant = Assistent
settings-assistant-note = Els models amb què pot parlar el plafó de l'assistent. Les claus es desen al clauer del sistema.
settings-assistant-none = Encara no hi ha cap model. Afegeix-ne un a sota: un model local, com els d'Ollama, es queda en aquest ordinador; un model al núvol necessita una clau d'API del seu proveïdor.
settings-assistant-in-use = En ús
settings-assistant-use = Fes servir
settings-assistant-remove = Elimina
settings-assistant-add = Afegeix un model
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Servidor compatible amb OpenAI
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, com ara { $example }
settings-assistant-key = Clau d'API
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Adreça, com ara { $example }
# The menu of how much a local model reads at once.
settings-assistant-context = Context
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K tokens
settings-assistant-context-note = Més context permet que l'assistent llegeixi una part més gran d'un fitxer en un xat, però el model ocupa més memòria i pot respondre més a poc a poc.
settings-assistant-add-button = Afegeix
settings-assistant-use-key = Continua
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Crea una clau a { $provider } i enganxa-la aquí.
settings-assistant-get-key = Obtén una clau d'API
settings-assistant-key-kept = La teva clau per a { $provider } es desa al clauer del sistema.
settings-assistant-change-key = Canvia la clau
settings-assistant-key-refused = { $provider } ha rebutjat aquesta clau. Comprova que s'ha copiat sencera i del compte correcte.
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } s'està executant a { $address }.
settings-assistant-no-server = El prev no ha trobat { $provider } en execució en aquest ordinador. Inicia'l o indica'n l'adreça a sota.
settings-assistant-get-server = Obtén { $provider }
settings-assistant-look-again = Torna a cercar
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Fes servir una altra adreça
settings-assistant-looking = S'estan cercant models…
settings-assistant-found-none = { $provider } encara no té cap model. Baixa'n un amb { $provider } i torna a cercar.
settings-assistant-recommended = Recomanat
settings-assistant-uses-tools = Fa servir eines
settings-assistant-sees = Veu imatges
settings-assistant-no-tools = No pot fer servir eines, i l'assistent les necessita
settings-assistant-added-tag = Afegit
settings-assistant-trying = S'està provant…
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Obre la pàgina
# $model is the model's name.
settings-assistant-added = { $model } ha respost i s'ha afegit.
settings-assistant-key-needed = Aquest model necessita una clau d'API.
# $error is what the keychain said.
settings-assistant-key-failed = No s'ha pogut desar la clau al clauer: { $error }
assistant-title = Assistent
assistant-new-chat = Xat nou
assistant-ask = Pregunta sobre aquest fitxer
assistant-send = Envia
assistant-stop = Atura
assistant-thinking = S'està pensant…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Raonament
assistant-running = S'està executant…
assistant-stopped = Aturat.
assistant-no-model = Primer afegeix un model a Configuració.
assistant-add-model = L'assistent necessita un model: un model al núvol amb la seva clau d'API, o un model local.
assistant-open-settings = Afegeix un model
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Ara parles amb { $model }.
assistant-add-another = Afegeix un model…
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } en aquest ordinador
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } a { $host }
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = La conversa ja no cap en el que { $model } pot llegir alhora. Comença un xat nou o tria un model que pugui llegir més.
assistant-problem-key = { $provider } ha rebutjat la clau d'API. Revisa-la a Configuració.
assistant-problem-rate = { $provider } demana anar més a poc a poc. Torna-ho a provar d'aquí a un moment.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } demana anar més a poc a poc: { $message }
assistant-problem-credit = { $provider } diu que el compte no té crèdit. Un compte nou n'ha de comprar a { $provider } perquè la clau funcioni; després torna-ho a provar.
assistant-problem-model = { $provider } no té cap model anomenat { $model }. Comprova'n el nom a Configuració.
assistant-problem-unavailable = { $provider } està ocupat o té problemes. Torna-ho a provar d'aquí a un moment.
assistant-problem-unreachable = El prev no ha pogut connectar amb { $provider }. Comprova la connexió o que el servidor s'estigui executant.
assistant-problem-refused = { $model } ha declinat respondre.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } no ha respost: { $message }
