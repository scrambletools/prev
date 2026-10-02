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
language-name = Français

## Common

common-cancel = Annuler
common-close = Fermer
common-save = Enregistrer

## Settings

settings-title = Réglages
settings-appearance = Apparence
settings-colors = Couleurs
settings-windows = Fenêtres
settings-default-app = App par défaut
settings-default-app-label = Ouvrir les fichiers avec prev
settings-default-app-note = Faire de prev l’app qui ouvre les PDF, les images, les dessins SVG et les fichiers Markdown.
settings-default-app-note-windows = Windows ne permet de choisir les apps par défaut que dans ses propres Paramètres. Ceci y ouvre la page de prev.
settings-default-app-note-macos = macOS vous demande de confirmer chaque type : PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP et AVIF.
settings-default-app-status = { $set } types de fichiers sur { $total } s’ouvrent avec prev.
settings-default-app-button = Définir par défaut
settings-default-app-button-windows = Ouvrir les Paramètres
settings-default-app-no-entry = L’entrée de bureau de prev n’est pas installée, le système ne peut donc pas ouvrir de fichiers avec elle. Installez prev depuis un paquet ou avec scripts/install.sh.
settings-default-app-no-bundle = Ouvrez prev depuis prev.app pour le définir par défaut.
settings-default-app-failed = Impossible de définir prev par défaut : { $error }
settings-storage = Stockage
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (version de développement, { $build })

## Markup toolbar

markup-tool-select = Sélectionner
markup-tool-area = Sélection rectangulaire
markup-tool-sketch = Esquisser
markup-tool-draw = Dessiner
markup-tool-shapes = Formes
markup-tool-text-box = Zone de texte
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Surligner
markup-tool-note = Note
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Signer
# A verb: the tool that marks areas to black out.
markup-tool-redact = Caviarder
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Appliquer
markup-apply-redactions = Appliquer les caviardages
markup-shape-style = Style de forme
markup-border-color = Couleur de bordure
markup-fill-color = Couleur de remplissage
markup-text-style = Style de texte
markup-delete = Supprimer
markup-undo = Annuler
markup-redo = Rétablir

## Markup menus

markup-shape-rectangle = Rectangle
markup-shape-rounded-rectangle = Rectangle arrondi
markup-shape-oval = Ovale
markup-shape-line = Ligne
markup-shape-arrow = Flèche
markup-shape-star = Étoile
markup-shape-polygon = Polygone
markup-shape-speech-bubble = Bulle de dialogue
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Loupe
# A shape that darkens the page around it.
markup-shape-mask = Masque
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Surlignage
markup-style-underline = Soulignement
markup-style-strikethrough = Barré
markup-style-squiggly = Ondulé
# Menu section headings.
markup-menu-color = Couleur
markup-menu-font = Police
markup-menu-size = Taille
markup-menu-alignment = Alignement
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Tirets

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Note
markup-kind-text-box = Zone de texte
markup-kind-stamp = Tampon
markup-kind-redaction = Caviardage
markup-kind-shape = Forme
# Tooltips on a note being edited.
markup-note-delete = Supprimer la note
markup-note-done = Terminé
markup-note-placeholder = Saisissez une note
markup-notes-empty = Aucun surlignage ni note
markup-notes-empty-hint = Les surlignages, notes et zones de texte apparaissent ici.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Page { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Impossible de modifier le document : { $error }
markup-copy-area-failed = Impossible de copier la zone : { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = le document a été fermé
markup-render-area-failed = impossible d’afficher la zone
markup-copy-stopped = la copie s’est interrompue

## Signatures

signature-menu-empty = Aucune signature pour l’instant.
signature-delete = Supprimer la signature
signature-create = Créer une signature…
signature-dialog-title = Créer une signature
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Dessiner
signature-tab-type = Saisir
signature-tab-image = Image
signature-draw-hint = Signez sur la ligne avec la souris, un stylet ou le pavé tactile.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Votre nom
signature-image-hint = Choisissez une photo ou une numérisation de votre signature sur papier blanc.
signature-choose-image = Choisir une image…
# Placeholder of the field naming the signature in the library.
signature-description = Description, par exemple Nom complet ou Initiales
# Clears the drawing, typed name or image.
signature-clear = Effacer
# The color the signature is drawn or typed in.
signature-ink = Encre
# The pen's width, for drawing.
signature-thickness = Épaisseur
signature-sign-first = Signez d’abord, puis enregistrez.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Signature { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Impossible de modifier les signatures : { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = aucun dossier de données : HOME n’est pas défini
signature-removing-stopped = la suppression s’est interrompue
signature-saving-stopped = l’enregistrement s’est interrompu
signature-reading-stopped = la lecture s’est interrompue
signature-not-an-image = ce fichier n’est pas une image que prev peut lire
signature-no-frames = l’image ne contient aucune trame
signature-not-found = aucune signature trouvée dans l’image

## Dragging

drag-pages-need-document = Les pages peuvent être déposées sur un document.
drag-image-unsupported = prev ne peut pas ouvrir cette image.
# $error is a lowercase reason or a technical message.
drag-area-failed = Impossible de faire glisser la zone : { $error }
drag-pages-failed = Impossible de faire glisser les pages : { $error }
drag-start-failed = Impossible de commencer le glisser-déposer.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Pages
drag-file-one-page = { $name } (page { $page })
drag-file-page-range = { $name } (pages { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = Image
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = Ajouter à ce document ?
drop-pdf-body = Ajouter « { $name } » à la fin de ce document, ou l’ouvrir dans sa propre fenêtre ?
drop-pdfs-body = { $count ->
    [one] Ajouter ce PDF à la fin de ce document, ou l’ouvrir dans sa propre fenêtre ?
   *[other] Ajouter ces { $count } PDF à la fin de ce document, ou les ouvrir chacun dans sa propre fenêtre ?
}
drop-pdf-add = Ajouter à la fin
drop-pdf-open = Ouvrir à part

## PDF window

pdf-opening = Ouverture…
pdf-open-failed = prev ne peut pas ouvrir ce document
pdf-no-pages = Le document ne contient aucune page.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = le document a été fermé
pdf-keep-original-failed = impossible de conserver la version d’origine : { $error }
pdf-save-failed = Impossible d’enregistrer : { $error }
pdf-nothing-to-paste = Il n’y a rien à coller.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = le collage s’est interrompu
pdf-file-dialog-failed = Impossible d’afficher la zone de dialogue de fichier : { $error }
pdf-bookmarks-no-home = Impossible d’enregistrer les signets : HOME n’est pas défini
pdf-bookmarks-save-failed = Impossible d’enregistrer les signets : { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Page { $page }

# Password prompt. $name is the file name.
pdf-password-protected = « { $name } » est protégé par un mot de passe
pdf-password = Mot de passe
pdf-password-wrong = Mot de passe incorrect. Réessayez.
# Button that opens a locked document.
pdf-unlock = Déverrouiller

# Toolbar tooltips and labels.
pdf-sidebar = Barre latérale
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = sur { $count }
pdf-zoom-out = Zoom arrière
pdf-zoom-in = Zoom avant
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent } %
pdf-fit-page = Page entière
pdf-fit-width = Pleine largeur
pdf-actual-size = Taille réelle
pdf-view-continuous = Défilement continu
pdf-view-single-page = Page unique
pdf-view-two-pages = Deux pages
pdf-undo = Annuler
pdf-redo = Rétablir
pdf-rotate-left = Faire pivoter vers la gauche
pdf-rotate-right = Faire pivoter vers la droite
pdf-inspector = Inspecteur
pdf-markup = Annotation
# Tooltip of the button that opens the export dialog.
pdf-export = Exporter
pdf-settings = Réglages

# Search field.
pdf-search = Rechercher
pdf-search-not-found = Introuvable
pdf-searching = Recherche…
# The match shown, of all matches found.
pdf-search-match = { $current } sur { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } sur { $total }+

# Inspector: section headings.
pdf-inspector-file = Fichier
pdf-inspector-document = Document
pdf-inspector-pages = Pages
# Inspector: fact labels and values.
pdf-inspector-title = Titre
pdf-inspector-author = Auteur
pdf-inspector-subject = Sujet
pdf-inspector-keywords = Mots-clés
pdf-inspector-created = Création
pdf-inspector-modified = Modification
pdf-inspector-application = Application
pdf-inspector-producer = Producteur PDF
pdf-inspector-version = Version
pdf-inspector-security = Sécurité
pdf-inspector-not-encrypted = Non chiffré
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Chiffré ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } page
   *[other] { $count } pages
}
pdf-inspector-page-size = Taille de page
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } po)
pdf-loading = Chargement…

# Sidebar tabs and lists.
pdf-tab-pages = Pages
pdf-tab-contents = Table des matières
pdf-tab-notes = Surlignages et notes
pdf-tab-bookmarks = Signets
pdf-no-outline = Aucune table des matières
pdf-no-outline-detail = Ce document n’a pas de table des matières.
pdf-no-bookmarks = Aucun signet
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Appuyez sur { $keys } pour ajouter un signet à une page.
pdf-no-bookmarks-detail-unbound = Les pages avec un signet s’affichent ici.
pdf-remove-bookmark = Supprimer le signet

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Pages
pages-insert-blank = Insérer une page vierge
pages-insert-file = Insérer depuis un fichier…
pages-copy = { $count ->
    [one] Copier la page
   *[other] Copier les pages
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Coller la page
   *[other] Coller { $count } pages
}
pages-crop = Rogner selon la sélection
pages-select-all = Sélectionner toutes les pages
pages-delete = { $count ->
    [one] Supprimer la page
   *[other] Supprimer les pages
}
pages-apply-redactions = Appliquer les caviardages…
pages-no-copied = Aucune page copiée à coller.
pages-copied = { $count ->
    [one] { $count } page copiée.
   *[other] { $count } pages copiées.
}
pages-copy-failed = Impossible de copier les pages : { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = la lecture s’est interrompue
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = n’est pas une image que prev peut lire
pages-read-failed = Impossible de lire le fichier : { $error }
pages-at-least-one = Un document doit contenir au moins une page.
pages-crop-needs-area = Choisissez d’abord une zone avec l’outil de sélection rectangulaire.
pages-change-failed = Impossible de modifier les pages : { $error }
pages-no-redactions = Il n’y avait aucun caviardage à appliquer.
pages-redactions-applied = { $count ->
    [one] { $count } caviardage appliqué.
   *[other] { $count } caviardages appliqués.
}
pages-forget-versions-failed = Impossible de supprimer les versions précédentes : { $error }
pages-redact-title = Appliquer les caviardages ?
pages-redact-body = { $count ->
    [one] Le texte, les images et les dessins situés sous la marque sont définitivement supprimés du document, et la marque devient un rectangle noir. Cette action est irréversible, et les versions précédentes de ce fichier conservées par prev sont supprimées.
   *[other] Le texte, les images et les dessins situés sous les { $count } marques sont définitivement supprimés du document, et les marques deviennent des rectangles noirs. Cette action est irréversible, et les versions précédentes de ce fichier conservées par prev sont supprimées.
}
# Button that applies redactions.
pages-redact-apply = Appliquer

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Exporter
pages-export-format = Format
pages-export-reduce = Réduire la taille du fichier (images à 150 ppp)
pages-export-flatten = Aplatir les annotations et les champs de formulaire
pages-export-flatten-detail = Les annotations et les champs remplis sont intégrés aux pages et ne peuvent plus être modifiés. Les marques de caviardage pas encore appliquées sont omises.
pages-export-encrypt = Chiffrer avec un mot de passe
pages-export-password = Mot de passe
pages-export-verify-password = Confirmer le mot de passe
pages-export-resolution = Résolution
pages-export-dpi = { $dpi } ppp
pages-export-quality = Qualité
# JPEG quality choices.
pages-export-quality-low = Faible
pages-export-quality-medium = Moyenne
pages-export-quality-high = Élevée
pages-export-quality-best = Optimale
pages-export-one-file = Toutes les pages sont placées dans un seul fichier.
pages-export-file-per-page = Chaque page est enregistrée dans son propre fichier, numéroté d’après le nom choisi.
pages-export-selected-only = { $count ->
    [one] Uniquement la page sélectionnée
   *[other] Uniquement les { $count } pages sélectionnées
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Exporter…
pages-export-no-password = Saisissez un mot de passe.
pages-export-password-mismatch = Les mots de passe ne correspondent pas.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (exporté)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = document
pages-export-same-file = Exportez vers un autre fichier ; ce document est enregistré automatiquement.
pages-export-exporting = Exportation de « { $name } »…
pages-export-done = « { $name } » exporté.
pages-export-done-images = { $count } images exportées.
pages-export-failed = Impossible d’exporter : { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = l’exportation s’est interrompue

## Start window

# Under the app name in a window with no file open.
app-start-hint = Ouvrez ou déposez un fichier PDF, image, SVG ou Markdown.
app-start-open = Ouvrir…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (dev)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind } : ce visualiseur n’est pas encore disponible.
app-cannot-open = prev ne peut pas ouvrir ce type de fichier.
app-cannot-read = prev ne peut pas lire ce fichier : { $error }
app-kind-pdf = Document PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Image { $format }
app-kind-svg = Dessin SVG
app-kind-markdown = Document Markdown
app-file-dialog-failed = Impossible d’afficher la zone de dialogue de fichier : { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Ouvrir
action-settings = Réglages

## Toolbar

app-toolbar-keep-shown = Toujours afficher la barre d’outils
app-toolbar-auto-hide = Masquer la barre d’outils quand le pointeur quitte la fenêtre
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Plus

## File facts

# Labels in a file's inspector.
app-fact-name = Nom
app-fact-folder = Dossier
app-fact-size = Taille
app-fact-modified = Modification
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count ->
    [one] { $count } octet
   *[other] { $count } octets
}
app-size-kb = { $size } Ko
app-size-mb = { $size } Mo
app-size-gb = { $size } Go
app-size-tb = { $size } To

## Links and clipboard

app-link-invalid = Lien non valide { $uri } : { $error }
app-link-open-failed = Impossible d’ouvrir { $uri } : { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = installez wl-clipboard pour coller des images
app-copy-needs-wl-clipboard = installez wl-clipboard pour copier des images
app-copy-no-pixels = la zone ne contient aucun pixel
# wl-copy is a program's name.
app-copy-no-input = wl-copy n’a reçu aucune donnée
app-copy-failed = wl-copy a échoué
app-clipboard-open-failed = Impossible d’ouvrir le presse-papiers : { $error }
app-copy-image-failed = Impossible de copier l’image : { $error }

## Printing

print-failed = Impossible d’imprimer : { $error }
print-stopped = L’impression s’est interrompue
print-unavailable = L’impression n’est pas encore disponible sur ce système.
print-no-window = Impossible d’imprimer : aucune fenêtre où afficher la zone de dialogue d’impression
print-dialog-failed = Impossible d’afficher la zone de dialogue d’impression : { $error }
# Shown after "Could not print:".
print-job-not-started = l’imprimante n’a pas démarré la tâche
# Shown after "Could not print:".
print-printer-stopped = l’imprimante s’est arrêtée

## File dialogs

dialog-open = Ouvrir
dialog-filter-all = Tous les fichiers pris en charge
dialog-filter-pdf = Documents PDF
dialog-filter-images = Images
dialog-filter-svg = Dessins SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Choisir le dossier des signatures
dialog-choose-versions = Choisir le dossier de l’historique des versions
dialog-choose-bookmarks = Choisir le fichier des signets

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Utilisation : prev [FILE]...

    Affiche et modifie des PDF et des images. Les fichiers s’ouvrent dans les
    fenêtres de prev s’il est déjà lancé ; sinon, prev démarre.

    Options :
      -h, --help     Afficher cette aide
      -V, --version  Afficher la version

## Settings, continued

settings-language = Langue
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Langue du système : { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Langue de saisie
settings-input-language-system = Suivre la disposition du clavier
settings-input-language-note = Définit le côté où commence un champ de texte vide. Le texte saisi garde son propre sens d’écriture.

settings-appearance-system = Système
settings-appearance-light = Clair
settings-appearance-dark = Sombre
settings-system-accent = Utiliser la couleur d’accentuation du système
# $theme is the Omarchy theme's name.
settings-omarchy-note = Les couleurs sont dérivées de la couleur d’accentuation de « { $theme } ».
settings-system-accent-note = Les couleurs sont dérivées de la couleur d’accentuation du système.
settings-system-accent-none = Le système n’a pas de couleur d’accentuation, prev utilise donc celle choisie ci-dessous.
settings-accent-chosen-note = Les couleurs sont dérivées de la couleur choisie ci-dessous.
settings-auto-hide = Masquer la barre d’outils quand le pointeur quitte la fenêtre
settings-auto-hide-note = La barre d’outils flotte au-dessus du document et s’escamote tant que le pointeur est hors de la fenêtre.
settings-animations = Animations
settings-animations-note = Barres et panneaux qui glissent, zones de dialogue qui s’agrandissent et boutons élastiques.
settings-animations-reduced = Désactivées tant que le système demande de réduire les animations.
settings-corner-radius = Arrondi des angles
settings-corner-radius-note = Pour les zones de dialogue et la barre d’outils flottante.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparence de la superposition
settings-overlay-note = La part de la page visible à travers la barre d’outils flottante.
settings-overlay-value = { $percent } %
settings-storage-signatures = Dossier des signatures
settings-storage-versions = Dossier de l’historique des versions
settings-storage-bookmarks = Fichier des signets
settings-storage-apply = Appliquer
settings-storage-choose = Choisir…
# $file is where the settings file is.
settings-storage-note = Les fichiers déjà stockés à l’ancien emplacement y restent ; déplacez-les pour continuer à les utiliser. Les réglages de prev sont enregistrés dans { $file }.
settings-save-failed = Impossible d’enregistrer les réglages : { $error }
settings-no-location = Aucun emplacement pour les réglages : HOME n’est pas défini
settings-full-path = Utilisez un chemin complet, par exemple ~/Documents/prev.
settings-path-is-folder = { $path } est un dossier, pas un fichier.
settings-folder-missing = Le dossier { $path } n’existe pas. Créez-le d’abord, ou choisissez-en un.
settings-path-is-file = { $path } est un fichier, pas un dossier.
settings-cannot-write = prev ne peut pas écrire dans { $path } : { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Exporter
# Section headings in the export dialog.
export-format = Format
export-quality = Qualité
export-size = Taille
# Button that goes on to choose where to save the export.
export-choose = Exporter…
# Format choice; the format name stays as it is.
export-format-webp = WebP (sans perte)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = image
# JPEG quality choices.
export-quality-low = Faible
export-quality-medium = Moyenne
export-quality-high = Élevée
export-quality-best = Optimale
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Taille réelle
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } pixels
# $error is the system's reason.
export-dialog-failed = Impossible d’afficher la zone de dialogue d’enregistrement : { $error }
# $path is where the file was saved.
export-done = Exporté vers { $path }
export-failed = Impossible d’exporter : { $error }
# Shown if exporting ends unexpectedly.
export-stopped = l’exportation s’est interrompue

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Les images annotées ne peuvent pas être modifiées. Exportez l’image pour garder les annotations, ou supprimez-les et fermez la barre d’outils Annotation.

# Shown if a background task ends unexpectedly.
image-loading-stopped = le chargement s’est interrompu
image-reverting-stopped = la restauration s’est interrompue
image-rendering-stopped = le rendu s’est interrompu
image-saving-stopped = l’enregistrement s’est interrompu
image-markup-stopped = l’annotation s’est interrompue
image-no-version-store = Aucun emplacement pour conserver les versions
image-revert-failed = Impossible de restaurer : { $error }
image-read-failed = Impossible de lire { $path } : { $error }
image-keep-original-failed = Impossible de conserver la version d’origine : { $error }
image-save-failed = Impossible d’enregistrer { $path } : { $error }
image-markup-start-failed = Impossible de lancer l’annotation : { $error }
image-cannot-edit = Les animations et les dessins SVG ne peuvent pas être modifiés.
image-cannot-mark-up = Les animations et les dessins SVG ne peuvent pas être annotés.
image-mark-up-wait = Attendez la fin de la modification, puis annotez.
image-crop-needs-selection = Faites d’abord glisser une sélection (outil Sélectionner), puis rognez.
image-size-needed = Saisissez une largeur et une hauteur en pixels.
# $name is a file name.
image-cannot-save-format = Les modifications de « { $name } » ne peuvent pas être enregistrées dans ce format. Utilisez Exporter ({ $keys }).
image-cannot-save-format-unbound = Les modifications de « { $name } » ne peuvent pas être enregistrées dans ce format. Utilisez Exporter.
image-cannot-export-animation = Les animations ne peuvent pas encore être exportées.
image-drop-pages = Les pages peuvent être déposées sur un document.
image-drag-failed = Impossible de commencer le glisser-déposer.
image-picture-save-failed = Impossible d’enregistrer l’image dans votre dossier Téléchargements.
image-open-failed = prev ne peut pas ouvrir cette image
image-opening = Ouverture…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Le nom ne correspond pas au format
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = « { $name } » sera enregistré au format { $format }, mais son nom se termine par .{ $extension }. D’autres apps risquent de ne pas pouvoir l’ouvrir.
image-name-mismatch-no-extension = « { $name } » sera enregistré au format { $format }, mais son nom n’a pas d’extension. D’autres apps risquent de ne pas pouvoir l’ouvrir.
image-choose-again = Choisir à nouveau
image-save-as-is = Enregistrer tel quel
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = trame { $current } sur { $total }
image-position = { $current } sur { $total }
image-edited = modifiée
# Toolbar tooltips.
image-sidebar = Barre latérale
image-zoom-out = Zoom arrière
image-zoom-in = Zoom avant
image-zoom = { $percent } %
image-fit = Ajuster à la fenêtre
image-actual-size = Taille réelle
image-undo = Annuler
image-redo = Rétablir
image-rotate-left = Faire pivoter vers la gauche
image-rotate-right = Faire pivoter vers la droite
image-flip-horizontal = Retourner horizontalement
image-flip-vertical = Retourner verticalement
image-select = Sélection rectangulaire
image-crop = Rogner selon la sélection
image-adjust-size-tool = Ajuster la taille
image-adjust-color-tool = Ajuster la couleur
# Tooltip and panel title.
image-inspector = Inspecteur
image-markup = Annotation
image-export = Exporter
image-settings = Réglages
# Panel titles.
image-adjust-color = Ajuster la couleur
image-adjust-size = Ajuster la taille
# Adjust Color sliders.
image-exposure = Exposition
image-contrast = Contraste
image-saturation = Saturation
image-temperature = Température
image-tint = Teinte
image-sepia = Sépia
image-sharpness = Netteté
image-levels = Niveaux
image-black-point = Point noir
image-midtones = Tons moyens
image-white-point = Point blanc
image-reset-all = Tout réinitialiser
# Adjust Size panel.
image-current-size = Taille actuelle : { $width } × { $height } pixels
image-width = Largeur
image-height = Hauteur
image-scale-proportionally = Redimensionner proportionnellement
# Button that applies the new size.
image-resize = Redimensionner
# Inspector panel.
image-inspector-loading = Chargement…
image-file = Fichier
image-format = Format
image-dimensions-label = Dimensions
image-pixels = { $width } × { $height } pixels
image-no-camera = Aucune information sur l’appareil photo.
image-location = Lieu
image-remove-location = Supprimer les informations de lieu
image-no-location = Aucune information de lieu.
image-keywords-description = Mots-clés et description
image-keywords-hint = Mots-clés, séparés par des virgules
image-description = Description
image-keywords-unsupported = Les mots-clés peuvent être enregistrés dans les fichiers JPEG, PNG et WebP.
# Heading over the earlier versions of the file.
image-revert-to = Revenir à
image-no-versions = Aucune version précédente.
image-revert = Restaurer
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } Ko
image-size-mb = { $size } Mo
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Fermer sans exporter les annotations ?
image-close-body = { $count ->
    [one] Les annotations d’une image ne sont conservées que tant que sa fenêtre est ouverte. Exportez l’image pour les garder : elles sont intégrées à la copie que vous enregistrez.
   *[other] Les annotations des images ne sont conservées que tant que leur fenêtre est ouverte. Exportez chaque image pour les garder : elles sont intégrées à la copie que vous enregistrez.
}
image-close-anyway = Fermer quand même

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = la lecture s’est interrompue
markdown-read-failed = prev ne peut pas lire ce fichier
markdown-draw-failed = Impossible d’afficher le document
# Under the export's size choices.
markdown-export-size = Le document entier, { $width } × { $height } pixels
# Search results.
markdown-not-found = Introuvable
markdown-match = { $current } sur { $total }
# Placeholder of the search field.
markdown-search = Rechercher
# Toolbar tooltips.
markdown-smaller-text = Texte plus petit
markdown-larger-text = Texte plus grand
markdown-zoom = { $percent } %
markdown-actual-size = Taille réelle
# Tooltip and panel title.
markdown-inspector = Inspecteur
markdown-export = Exporter
markdown-settings = Réglages
# Inspector headings and labels.
markdown-file = Fichier
markdown-document = Document
markdown-words = Mots
markdown-lines = Lignes
markdown-pictures = Images

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Appareil photo
image-meta-exposure = Exposition
image-meta-image = Image
image-meta-make = Marque
image-meta-model = Modèle
image-meta-lens = Objectif
image-meta-exposure-time = Temps de pose
# The lens aperture, written like f/2.8.
image-meta-f-number = Ouverture
image-meta-iso = ISO
image-meta-focal-length = Focale
image-meta-exposure-bias = Correction d’exposition
image-meta-flash = Flash
image-meta-date-taken = Date de prise de vue
image-meta-orientation = Orientation
image-meta-color-space = Espace colorimétrique
image-meta-software = Logiciel
image-meta-artist = Auteur
image-meta-copyright = Copyright
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normale
    [2] Miroir horizontal
    [3] Rotation de 180°
    [4] Miroir vertical
    [5] Miroir horizontal, rotation de 90° dans le sens antihoraire
    [6] Rotation de 90° dans le sens horaire
    [7] Miroir horizontal, rotation de 90° dans le sens horaire
    [8] Rotation de 90° dans le sens antihoraire
   *[other] Inconnue ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Déclenché
   *[no] Non déclenché
}{ $mode ->
    [on] , forcé
    [off] , désactivé
    [auto] , auto
   *[unknown] {""}
}{ $redeye ->
    [yes] , réduction des yeux rouges
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Non calibré
   *[other] Autre ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = impossible d’ouvrir le document : { $detail }
error-pdf-page-out-of-range = la page { $page } n’existe pas
error-pdf-password-protected = le document est protégé par un mot de passe ; ouvrez-le et copiez plutôt ses pages
error-pdf-no-pages = aucune page à extraire
error-pdf-crop-outside = la zone de rognage est en dehors de la page
error-pdf-closed = document fermé
error-pdf-saved-unreadable = le document enregistré ne s’ouvre plus
error-image-read = impossible de lire le fichier : { $detail }
error-image-invalid = l’image est endommagée ou non valide : { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = l’ouverture de ce format nécessite { $library }, qui n’est pas installé
# $format is an image format name, such as HEIC.
error-image-unsupported = les images { $format } ne sont pas encore prises en charge
error-image-encode = impossible d’encoder l’image : { $detail }
error-exif-malformed = les données EXIF sont mal formées
error-settings-read = impossible de lire les réglages : { $detail }
error-settings-invalid = réglages non valides : { $detail }
error-remove-location = impossible de supprimer le lieu : { $error }
error-location-unsupported = les informations de lieu peuvent être supprimées des fichiers JPEG, PNG, WebP et TIFF
error-xmp-unsupported = les mots-clés et descriptions ne peuvent être enregistrés que dans les fichiers JPEG, PNG et WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW d’appareil photo

## The macOS menu bar, named as in macOS's own apps.
menu-about = À propos de prev
menu-settings = Réglages…
menu-services = Services
menu-hide = Masquer prev
menu-hide-others = Masquer les autres
menu-show-all = Tout afficher
menu-quit = Quitter prev
menu-file = Fichier
menu-open = Ouvrir…
menu-close = Fermer la fenêtre
menu-export = Exporter…
menu-print = Imprimer…
menu-edit = Édition
menu-undo = Annuler
menu-redo = Rétablir
menu-cut = Couper
menu-copy = Copier
menu-paste = Coller
menu-select-all = Tout sélectionner
menu-find = Rechercher
menu-find-next = Rechercher le suivant
menu-find-previous = Rechercher le précédent
menu-view = Présentation
menu-hide-sidebar = Masquer la barre latérale
menu-thumbnails = Vignettes
menu-contents = Table des matières
menu-notes = Surlignages et notes
menu-bookmarks = Signets
menu-zoom-in = Zoom avant
menu-zoom-out = Zoom arrière
menu-actual-size = Taille réelle
menu-zoom-to-fit = Zoom ajusté
menu-inspector = Afficher l’inspecteur
menu-slideshow = Diaporama
menu-full-screen = Passer en mode plein écran
menu-go = Aller
menu-next-page = Page suivante
menu-previous-page = Page précédente
menu-go-to-page = Aller à la page…
menu-bookmark = Ajouter un signet
menu-tools = Outils
menu-markup = Afficher la barre d’outils Annotation
menu-rotate-left = Faire pivoter vers la gauche
menu-rotate-right = Faire pivoter vers la droite
menu-crop = Rogner
menu-adjust-color = Ajuster la couleur…
menu-window = Fenêtre
menu-minimize = Placer dans le Dock
menu-zoom = Réduire/agrandir
menu-bring-all-to-front = Tout ramener au premier plan
