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
language-name = Español

## Common

common-cancel = Cancelar
common-close = Cerrar
common-save = Guardar

## Settings

settings-title = Ajustes
settings-appearance = Apariencia
settings-colors = Colores
settings-windows = Ventanas
settings-default-app = App predeterminada
settings-default-app-label = Abrir archivos con prev
settings-default-app-note = Haz que prev sea la app que abre PDF, imágenes, dibujos SVG y archivos Markdown.
settings-default-app-note-windows = Windows solo permite elegir las apps predeterminadas en su propia Configuración. Esto abre allí la página de prev.
settings-default-app-note-macos = macOS te pide confirmar cada tipo: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP y AVIF.
settings-default-app-status = { $set } de { $total } tipos de archivo se abren con prev.
settings-default-app-button = Establecer como predeterminada
settings-default-app-button-windows = Abrir Configuración
settings-default-app-no-entry = La entrada de escritorio de prev no está instalada, así que el sistema no puede abrir archivos con ella. Instala prev desde un paquete o con scripts/install.sh.
settings-default-app-no-bundle = Abre prev desde prev.app para establecerla como predeterminada.
settings-default-app-failed = No se pudo establecer prev como predeterminada: { $error }
settings-storage = Almacenamiento
settings-version = prev { $version }
settings-version-development = prev { $version } (versión de desarrollo)

## Markup toolbar

markup-tool-select = Seleccionar
markup-tool-area = Selección rectangular
markup-tool-sketch = Boceto
markup-tool-draw = Dibujar
markup-tool-shapes = Formas
markup-tool-text-box = Cuadro de texto
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Resaltar
markup-tool-note = Nota
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Firmar
# A verb: the tool that marks areas to black out.
markup-tool-redact = Censurar
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Aplicar
markup-apply-redactions = Aplicar censuras
markup-shape-style = Estilo de forma
markup-border-color = Color del borde
markup-fill-color = Color de relleno
markup-text-style = Estilo de texto
markup-delete = Eliminar
markup-undo = Deshacer
markup-redo = Rehacer

## Markup menus

markup-shape-rectangle = Rectángulo
markup-shape-rounded-rectangle = Rectángulo redondeado
markup-shape-oval = Óvalo
markup-shape-line = Línea
markup-shape-arrow = Flecha
markup-shape-star = Estrella
markup-shape-polygon = Polígono
markup-shape-speech-bubble = Bocadillo
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Lupa
# A shape that darkens the page around it.
markup-shape-mask = Máscara
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Resaltado
markup-style-underline = Subrayado
markup-style-strikethrough = Tachado
markup-style-squiggly = Ondulado
# Menu section headings.
markup-menu-color = Color
markup-menu-font = Tipo de letra
markup-menu-size = Tamaño
markup-menu-alignment = Alineación
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Discontinua

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Nota
markup-kind-text-box = Cuadro de texto
markup-kind-stamp = Sello
markup-kind-redaction = Censura
markup-kind-shape = Forma
# Tooltips on a note being edited.
markup-note-delete = Eliminar nota
markup-note-done = Listo
markup-note-placeholder = Escribe una nota
markup-notes-empty = No hay resaltados ni notas
markup-notes-empty-hint = Aquí aparecen los resaltados, las notas y los cuadros de texto.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Página { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = No se ha podido cambiar el documento: { $error }
markup-copy-area-failed = No se ha podido copiar el área: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = el documento se ha cerrado
markup-render-area-failed = no se ha podido representar el área
markup-copy-stopped = la copia se ha detenido

## Signatures

signature-menu-empty = Aún no hay firmas.
signature-delete = Eliminar firma
signature-create = Crear firma…
signature-dialog-title = Crear firma
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Dibujar
signature-tab-type = Escribir
signature-tab-image = Imagen
signature-draw-hint = Firma sobre la línea con el ratón, el lápiz o el trackpad.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Tu nombre
signature-image-hint = Elige una foto o un escaneo de tu firma sobre papel blanco.
signature-choose-image = Elegir imagen…
# Placeholder of the field naming the signature in the library.
signature-description = Descripción, como Nombre completo o Iniciales
# Clears the drawing, typed name or image.
signature-clear = Borrar
# The color the signature is drawn or typed in.
signature-ink = Tinta
# The pen's width, for drawing.
signature-thickness = Grosor
signature-sign-first = Firma primero y luego guarda.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Firma { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = No se han podido cambiar las firmas: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = no hay carpeta de datos: HOME no está definido
signature-removing-stopped = la eliminación se ha detenido
signature-saving-stopped = el guardado se ha detenido
signature-reading-stopped = la lectura se ha detenido
signature-not-an-image = ese archivo no es una imagen que prev pueda leer
signature-no-frames = la imagen no tiene fotogramas
signature-not-found = no se ha encontrado ninguna firma en la imagen

## Dragging

drag-pages-need-document = Las páginas se pueden soltar en un documento.
drag-image-unsupported = prev no puede abrir esta imagen.
# $error is a lowercase reason or a technical message.
drag-area-failed = No se ha podido arrastrar el área: { $error }
drag-pages-failed = No se han podido arrastrar las páginas: { $error }
drag-start-failed = No se ha podido empezar a arrastrar.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Páginas
drag-file-one-page = { $name } (página { $page })
drag-file-page-range = { $name } (páginas { $first }–{ $last })

## PDF window

pdf-opening = Abriendo…
pdf-open-failed = prev no puede abrir este documento
pdf-no-pages = El documento no tiene páginas.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = el documento se ha cerrado
pdf-keep-original-failed = no se ha podido conservar la versión original: { $error }
pdf-save-failed = No se ha podido guardar: { $error }
pdf-nothing-to-paste = No hay nada que pegar.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = el pegado se ha detenido
pdf-file-dialog-failed = No se ha podido mostrar el diálogo de archivos: { $error }
pdf-bookmarks-no-home = No se pueden guardar los marcadores: HOME no está definido
pdf-bookmarks-save-failed = No se han podido guardar los marcadores: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Página { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” está protegido con contraseña
pdf-password = Contraseña
pdf-password-wrong = Contraseña incorrecta. Vuelve a intentarlo.
# Button that opens a locked document.
pdf-unlock = Desbloquear

# Toolbar tooltips and labels.
pdf-sidebar = Barra lateral
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = de { $count }
pdf-zoom-out = Reducir
pdf-zoom-in = Ampliar
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Ajustar a la página
pdf-fit-width = Ajustar al ancho
pdf-actual-size = Tamaño real
pdf-view-continuous = Desplazamiento continuo
pdf-view-single-page = Una página
pdf-view-two-pages = Dos páginas
pdf-undo = Deshacer
pdf-redo = Rehacer
pdf-rotate-left = Girar a la izquierda
pdf-rotate-right = Girar a la derecha
pdf-inspector = Inspector
pdf-markup = Marcación
# Tooltip of the button that opens the export dialog.
pdf-export = Exportar
pdf-settings = Ajustes

# Search field.
pdf-search = Buscar
pdf-search-not-found = No se ha encontrado
pdf-searching = Buscando…
# The match shown, of all matches found.
pdf-search-match = { $current } de { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } de { $total }+

# Inspector: section headings.
pdf-inspector-file = Archivo
pdf-inspector-document = Documento
pdf-inspector-pages = Páginas
# Inspector: fact labels and values.
pdf-inspector-title = Título
pdf-inspector-author = Autor
pdf-inspector-subject = Asunto
pdf-inspector-keywords = Palabras clave
pdf-inspector-created = Creado
pdf-inspector-modified = Modificado
pdf-inspector-application = Aplicación
pdf-inspector-producer = Productor de PDF
pdf-inspector-version = Versión
pdf-inspector-security = Seguridad
pdf-inspector-not-encrypted = Sin cifrar
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Cifrado ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } página
   *[other] { $count } páginas
}
pdf-inspector-page-size = Tamaño de página
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } in)
pdf-loading = Cargando…

# Sidebar tabs and lists.
pdf-tab-pages = Páginas
pdf-tab-contents = Contenido
pdf-tab-notes = Resaltados y notas
pdf-tab-bookmarks = Marcadores
pdf-no-outline = Sin tabla de contenido
pdf-no-outline-detail = Este documento no tiene índice.
pdf-no-bookmarks = Sin marcadores
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Pulsa { $keys } para añadir un marcador a una página.
pdf-no-bookmarks-detail-unbound = Las páginas con marcador aparecen aquí.
pdf-remove-bookmark = Eliminar marcador

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Páginas
pages-insert-blank = Insertar página en blanco
pages-insert-file = Insertar desde archivo…
pages-copy = { $count ->
    [one] Copiar página
   *[other] Copiar páginas
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Pegar página
   *[other] Pegar { $count } páginas
}
pages-crop = Recortar a la selección
pages-select-all = Seleccionar todas las páginas
pages-delete = { $count ->
    [one] Eliminar página
   *[other] Eliminar páginas
}
pages-apply-redactions = Aplicar censuras…
pages-no-copied = No hay páginas copiadas que pegar.
pages-copied = { $count ->
    [one] Se ha copiado { $count } página.
   *[other] Se han copiado { $count } páginas.
}
pages-copy-failed = No se han podido copiar las páginas: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = la lectura se ha detenido
pages-read-failed = No se ha podido leer el archivo: { $error }
pages-at-least-one = Un documento necesita al menos una página.
pages-crop-needs-area = Primero elige un área con la herramienta de selección rectangular.
pages-change-failed = No se han podido cambiar las páginas: { $error }
pages-no-redactions = No había censuras que aplicar.
pages-redactions-applied = { $count ->
    [one] Se ha aplicado { $count } censura.
   *[other] Se han aplicado { $count } censuras.
}
pages-forget-versions-failed = No se han podido eliminar las versiones anteriores: { $error }
pages-redact-title = ¿Aplicar las censuras?
pages-redact-body = { $count ->
    [one] El texto, las imágenes y los dibujos bajo la marca se eliminan del documento para siempre, y la marca se convierte en un recuadro negro. Esta acción no se puede deshacer, y se eliminan las versiones anteriores de este archivo que prev conserva.
   *[other] El texto, las imágenes y los dibujos bajo las { $count } marcas se eliminan del documento para siempre, y las marcas se convierten en recuadros negros. Esta acción no se puede deshacer, y se eliminan las versiones anteriores de este archivo que prev conserva.
}
# Button that applies redactions.
pages-redact-apply = Aplicar

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Exportar
pages-export-format = Formato
pages-export-reduce = Reducir el tamaño del archivo (imágenes a 150 ppp)
pages-export-flatten = Acoplar anotaciones y campos de formulario
pages-export-flatten-detail = Las marcas y los campos rellenados pasan a formar parte de las páginas y ya no se pueden editar. Las marcas de censura aún no aplicadas se omiten.
pages-export-encrypt = Cifrar con contraseña
pages-export-password = Contraseña
pages-export-verify-password = Verificar contraseña
pages-export-resolution = Resolución
pages-export-dpi = { $dpi } ppp
pages-export-quality = Calidad
# JPEG quality choices.
pages-export-quality-low = Baja
pages-export-quality-medium = Media
pages-export-quality-high = Alta
pages-export-quality-best = Máxima
pages-export-one-file = Todas las páginas van en un solo archivo.
pages-export-file-per-page = Cada página se guarda en su propio archivo, numerado a partir del nombre que elijas.
pages-export-selected-only = { $count ->
    [one] Solo la página seleccionada
   *[other] Solo las { $count } páginas seleccionadas
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Exportar…
pages-export-no-password = Introduce una contraseña.
pages-export-password-mismatch = Las contraseñas no coinciden.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (exportado)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = documento
pages-export-same-file = Exporta a un archivo nuevo; este documento se guarda solo.
pages-export-exporting = Exportando “{ $name }”…
pages-export-done = Se ha exportado “{ $name }”.
pages-export-done-images = Se han exportado { $count } imágenes.
pages-export-failed = No se ha podido exportar: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = la exportación se ha detenido

## Start window

# Under the app name in a window with no file open.
app-start-hint = Abre o suelta un archivo PDF, de imagen, SVG o Markdown.
app-start-open = Abrir…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (desarrollo)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: este visor aún no está hecho.
app-cannot-open = prev no puede abrir este tipo de archivo.
app-cannot-read = prev no puede leer este archivo: { $error }
app-kind-pdf = Documento PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Imagen { $format }
app-kind-svg = Dibujo SVG
app-kind-markdown = Documento Markdown
app-file-dialog-failed = No se ha podido mostrar el diálogo de archivos: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Abrir
action-settings = Ajustes

## Toolbar

app-toolbar-keep-shown = Mantener visible la barra de herramientas
app-toolbar-auto-hide = Ocultar la barra de herramientas cuando el puntero salga
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Más

## File facts

# Labels in a file's inspector.
app-fact-name = Nombre
app-fact-folder = Carpeta
app-fact-size = Tamaño
app-fact-modified = Modificado
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } bytes
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Enlace no válido { $uri }: { $error }
app-link-open-failed = No se ha podido abrir { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = instala wl-clipboard para pegar imágenes
app-copy-needs-wl-clipboard = instala wl-clipboard para copiar imágenes
app-copy-no-pixels = el área no tiene píxeles
# wl-copy is a program's name.
app-copy-no-input = wl-copy no tiene entrada
app-copy-failed = wl-copy ha fallado
app-clipboard-open-failed = No se ha podido abrir el portapapeles: { $error }
app-copy-image-failed = No se ha podido copiar la imagen: { $error }

## Printing

print-failed = No se ha podido imprimir: { $error }
print-stopped = La impresión se ha detenido
print-unavailable = La impresión aún no está disponible en este sistema.
print-no-window = No se ha podido imprimir: no hay ninguna ventana sobre la que mostrar el diálogo de impresión
print-dialog-failed = No se ha podido mostrar el diálogo de impresión: { $error }
# Shown after "Could not print:".
print-job-not-started = la impresora no ha iniciado el trabajo
# Shown after "Could not print:".
print-printer-stopped = la impresora se ha detenido

## File dialogs

dialog-open = Abrir
dialog-filter-all = Todos los archivos compatibles
dialog-filter-pdf = Documentos PDF
dialog-filter-images = Imágenes
dialog-filter-svg = Dibujos SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Elige la carpeta de firmas
dialog-choose-versions = Elige la carpeta del historial de versiones
dialog-choose-bookmarks = Elige el archivo de marcadores

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Uso: prev [FILE]...

    Ver y editar archivos PDF e imágenes. Los archivos se abren en ventanas
    de prev en ejecución, que se inicia si hace falta.

    Opciones:
      -h, --help     Mostrar esta ayuda
      -V, --version  Mostrar la versión

## Settings, continued

settings-language = Idioma
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Predeterminado del sistema: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Idioma de entrada
settings-input-language-system = Seguir la distribución del teclado
settings-input-language-note = Define el lado por el que empieza un campo de texto vacío. El texto que escribas mantiene su propia dirección.

settings-appearance-system = Sistema
settings-appearance-light = Claro
settings-appearance-dark = Oscuro
settings-omarchy-accent = Usar el color de acento de Omarchy
# $theme is the Omarchy theme's name.
settings-omarchy-note = Los colores se crean a partir del color de acento de “{ $theme }”.
settings-omarchy-none = No hay ningún tema de Omarchy activo.
settings-auto-hide = Ocultar la barra de herramientas cuando el puntero salga
settings-auto-hide-note = La barra de herramientas flota sobre el documento y se oculta deslizándose mientras el puntero está fuera de la ventana.
settings-animations = Animaciones
settings-animations-note = Barras y paneles que se deslizan, diálogos que crecen y botones con rebote.
settings-animations-reduced = Desactivadas mientras el sistema pida reducir el movimiento.
settings-corner-radius = Radio de las esquinas
settings-corner-radius-note = Para los diálogos y la barra de herramientas flotante.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparencia de la superposición
settings-overlay-note = Cuánto se ve la página a través de la barra de herramientas flotante.
settings-overlay-value = { $percent }%
settings-storage-signatures = Carpeta de firmas
settings-storage-versions = Carpeta del historial de versiones
settings-storage-bookmarks = Archivo de marcadores
settings-storage-apply = Aplicar
settings-storage-choose = Elegir…
# $file is where the settings file is.
settings-storage-note = Los archivos ya guardados en una ubicación anterior se quedan allí; muévelos para seguir usándolos. Los ajustes de prev se guardan en { $file }.
settings-save-failed = No se han podido guardar los ajustes: { $error }
settings-no-location = No hay ubicación para los ajustes: HOME no está definido
settings-full-path = Usa una ruta completa, como ~/Documents/prev.
settings-path-is-folder = { $path } es una carpeta, no un archivo.
settings-folder-missing = No existe la carpeta { $path }. Créala primero o elige otra.
settings-path-is-file = { $path } es un archivo, no una carpeta.
settings-cannot-write = prev no puede escribir en { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Exportar
# Section headings in the export dialog.
export-format = Formato
export-quality = Calidad
export-size = Tamaño
# Button that goes on to choose where to save the export.
export-choose = Exportar…
# Format choice; the format name stays as it is.
export-format-webp = WebP (sin pérdida)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = imagen
# JPEG quality choices.
export-quality-low = Baja
export-quality-medium = Media
export-quality-high = Alta
export-quality-best = Máxima
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Tamaño real
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } píxeles
# $error is the system's reason.
export-dialog-failed = No se ha podido mostrar el diálogo de guardado: { $error }
# $path is where the file was saved.
export-done = Se ha exportado { $path }
export-failed = No se ha podido exportar: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = la exportación se ha detenido

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Las imágenes con marcas no se pueden editar. Exporta para conservar las marcas, o elimínalas y cierra la barra de marcación.

# Shown if a background task ends unexpectedly.
image-loading-stopped = la carga se ha detenido
image-reverting-stopped = la restauración se ha detenido
image-rendering-stopped = la representación se ha detenido
image-saving-stopped = el guardado se ha detenido
image-markup-stopped = la marcación se ha detenido
image-no-version-store = No hay dónde guardar versiones
image-revert-failed = No se ha podido restaurar: { $error }
image-read-failed = No se ha podido leer { $path }: { $error }
image-keep-original-failed = No se ha podido conservar la versión original: { $error }
image-save-failed = No se ha podido guardar { $path }: { $error }
image-markup-start-failed = No se ha podido iniciar la marcación: { $error }
image-cannot-edit = Las animaciones y los dibujos SVG no se pueden editar.
image-cannot-mark-up = Las animaciones y los dibujos SVG no se pueden marcar.
image-mark-up-wait = Espera a que termine la edición y luego marca.
image-crop-needs-selection = Primero arrastra una selección (herramienta Seleccionar) y luego recorta.
image-size-needed = Introduce un ancho y un alto en píxeles.
# $name is a file name.
image-cannot-save-format = Los cambios en “{ $name }” no se pueden guardar en su formato. Usa Exportar ({ $keys }).
image-cannot-save-format-unbound = Los cambios en “{ $name }” no se pueden guardar en su formato. Usa Exportar.
image-cannot-export-animation = Las animaciones aún no se pueden exportar.
image-drop-pages = Las páginas se pueden soltar en un documento.
image-drag-failed = No se ha podido empezar a arrastrar.
image-open-failed = prev no puede abrir esta imagen
image-opening = Abriendo…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = El nombre no coincide con el formato
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” se guardará como archivo { $format }, pero su nombre termina en .{ $extension }. Es posible que otras apps no lo abran.
image-name-mismatch-no-extension = “{ $name }” se guardará como archivo { $format }, pero su nombre no tiene extensión. Es posible que otras apps no lo abran.
image-choose-again = Elegir de nuevo
image-save-as-is = Guardar así
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = fotograma { $current } de { $total }
image-position = { $current } de { $total }
image-edited = editada
# Toolbar tooltips.
image-sidebar = Barra lateral
image-zoom-out = Reducir
image-zoom-in = Ampliar
image-zoom = { $percent }%
image-fit = Ajustar a la ventana
image-actual-size = Tamaño real
image-undo = Deshacer
image-redo = Rehacer
image-rotate-left = Girar a la izquierda
image-rotate-right = Girar a la derecha
image-flip-horizontal = Voltear horizontalmente
image-flip-vertical = Voltear verticalmente
image-select = Selección rectangular
image-crop = Recortar a la selección
image-adjust-size-tool = Ajustar tamaño
image-adjust-color-tool = Ajustar color
# Tooltip and panel title.
image-inspector = Inspector
image-markup = Marcación
image-export = Exportar
image-settings = Ajustes
# Panel titles.
image-adjust-color = Ajustar color
image-adjust-size = Ajustar tamaño
# Adjust Color sliders.
image-exposure = Exposición
image-contrast = Contraste
image-saturation = Saturación
image-temperature = Temperatura
image-tint = Tinte
image-sepia = Sepia
image-sharpness = Nitidez
image-levels = Niveles
image-black-point = Punto negro
image-midtones = Medios tonos
image-white-point = Punto blanco
image-reset-all = Restablecer todo
# Adjust Size panel.
image-current-size = Tamaño actual: { $width } × { $height } píxeles
image-width = Ancho
image-height = Alto
image-scale-proportionally = Escalar proporcionalmente
# Button that applies the new size.
image-resize = Redimensionar
# Inspector panel.
image-inspector-loading = Cargando…
image-file = Archivo
image-format = Formato
image-dimensions-label = Dimensiones
image-pixels = { $width } × { $height } píxeles
image-no-camera = No hay información de la cámara.
image-location = Ubicación
image-remove-location = Eliminar información de ubicación
image-no-location = No hay información de ubicación.
image-keywords-description = Palabras clave y descripción
image-keywords-hint = Palabras clave, separadas por comas
image-description = Descripción
image-keywords-unsupported = Las palabras clave se pueden guardar en archivos JPEG, PNG y WebP.
# Heading over the earlier versions of the file.
image-revert-to = Volver a
image-no-versions = No hay versiones anteriores.
image-revert = Restaurar
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = ¿Cerrar sin exportar las marcas?
image-close-body = { $count ->
    [one] Las marcas de una imagen solo duran mientras su ventana está abierta. Exporta la imagen para conservarlas: las marcas se dibujan en la copia que guardes.
   *[other] Las marcas de las imágenes solo duran mientras su ventana está abierta. Exporta cada imagen para conservarlas: las marcas se dibujan en la copia que guardes.
}
image-close-anyway = Cerrar de todos modos

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = la lectura se ha detenido
markdown-read-failed = prev no puede leer este archivo
markdown-draw-failed = No se ha podido dibujar el documento
# Under the export's size choices.
markdown-export-size = El documento completo, { $width } × { $height } píxeles
# Search results.
markdown-not-found = No se ha encontrado
markdown-match = { $current } de { $total }
# Placeholder of the search field.
markdown-search = Buscar
# Toolbar tooltips.
markdown-smaller-text = Texto más pequeño
markdown-larger-text = Texto más grande
markdown-zoom = { $percent }%
markdown-actual-size = Tamaño real
# Tooltip and panel title.
markdown-inspector = Inspector
markdown-export = Exportar
markdown-settings = Ajustes
# Inspector headings and labels.
markdown-file = Archivo
markdown-document = Documento
markdown-words = Palabras
markdown-lines = Líneas
markdown-pictures = Imágenes

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Cámara
image-meta-exposure = Exposición
image-meta-image = Imagen
image-meta-make = Marca
image-meta-model = Modelo
image-meta-lens = Objetivo
image-meta-exposure-time = Tiempo de exposición
# The lens aperture, written like f/2.8.
image-meta-f-number = Número f
image-meta-iso = ISO
image-meta-focal-length = Distancia focal
image-meta-exposure-bias = Compensación de exposición
image-meta-flash = Flash
image-meta-date-taken = Fecha de captura
image-meta-orientation = Orientación
image-meta-color-space = Espacio de color
image-meta-software = Software
image-meta-artist = Artista
image-meta-copyright = Derechos de autor
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Reflejada horizontalmente
    [3] Girada 180°
    [4] Reflejada verticalmente
    [5] Reflejada horizontalmente, girada 90° en sentido antihorario
    [6] Girada 90° en sentido horario
    [7] Reflejada horizontalmente, girada 90° en sentido horario
    [8] Girada 90° en sentido antihorario
   *[other] Desconocida ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Se disparó
   *[no] No se disparó
}{ $mode ->
    [on] , forzado
    [off] , desactivado
    [auto] , automático
   *[unknown] {""}
}{ $redeye ->
    [yes] , reducción de ojos rojos
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Sin calibrar
   *[other] Otro ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = no se puede abrir el documento: { $detail }
error-pdf-page-out-of-range = la página { $page } no existe
error-pdf-password-protected = el documento está protegido con contraseña; ábrelo y copia sus páginas en su lugar
error-pdf-no-pages = no hay páginas que extraer
error-pdf-crop-outside = el área de recorte está fuera de la página
error-pdf-closed = documento cerrado
error-pdf-saved-unreadable = el documento guardado ya no se abre
error-image-read = no se puede leer el archivo: { $detail }
error-image-invalid = la imagen está dañada o no es válida: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = para abrir este formato hace falta { $library }, que no está instalado
# $format is an image format name, such as HEIC.
error-image-unsupported = las imágenes { $format } aún no son compatibles
error-image-encode = no se puede codificar la imagen: { $detail }
error-exif-malformed = los datos EXIF están mal formados
error-settings-read = no se pueden leer los ajustes: { $detail }
error-settings-invalid = ajustes no válidos: { $detail }
error-remove-location = no se ha podido eliminar la ubicación: { $error }
error-location-unsupported = la información de ubicación se puede eliminar de archivos JPEG, PNG, WebP y TIFF
error-xmp-unsupported = las palabras clave y las descripciones solo se pueden guardar en archivos JPEG, PNG y WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW de cámara

## The macOS menu bar, named as in macOS's own apps.
menu-about = Acerca de prev
menu-settings = Ajustes…
menu-services = Servicios
menu-hide = Ocultar prev
menu-hide-others = Ocultar otros
menu-show-all = Mostrar todo
menu-quit = Salir de prev
menu-file = Archivo
menu-open = Abrir…
menu-close = Cerrar ventana
menu-export = Exportar…
menu-print = Imprimir…
menu-edit = Edición
menu-undo = Deshacer
menu-redo = Rehacer
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Pegar
menu-select-all = Seleccionar todo
menu-find = Buscar
menu-find-next = Buscar siguiente
menu-find-previous = Buscar anterior
menu-view = Visualización
menu-hide-sidebar = Ocultar barra lateral
menu-thumbnails = Miniaturas
menu-contents = Tabla de contenido
menu-notes = Resaltados y notas
menu-bookmarks = Marcadores
menu-zoom-in = Ampliar
menu-zoom-out = Reducir
menu-actual-size = Tamaño real
menu-zoom-to-fit = Ajustar a la ventana
menu-inspector = Mostrar inspector
menu-slideshow = Pase de diapositivas
menu-full-screen = Entrar en pantalla completa
menu-go = Ir
menu-next-page = Página siguiente
menu-previous-page = Página anterior
menu-go-to-page = Ir a la página…
menu-bookmark = Añadir marcador
menu-tools = Herramientas
menu-markup = Mostrar barra de herramientas de marcación
menu-rotate-left = Girar a la izquierda
menu-rotate-right = Girar a la derecha
menu-crop = Recortar
menu-adjust-color = Ajustar color…
menu-window = Ventana
menu-minimize = Minimizar
menu-zoom = Zoom
menu-bring-all-to-front = Traer todo al frente
