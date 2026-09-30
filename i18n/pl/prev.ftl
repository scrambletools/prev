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
language-name = Polski

## Common

common-cancel = Anuluj
common-close = Zamknij
common-save = Zapisz

## Settings

settings-title = Ustawienia
settings-appearance = Wygląd
settings-colors = Kolory
settings-windows = Okna
settings-default-app = Domyślna aplikacja
settings-default-app-label = Otwieraj pliki w prev
settings-default-app-note = Ustaw prev jako aplikację otwierającą pliki PDF, obrazy, rysunki SVG i pliki Markdown.
settings-default-app-note-windows = Windows pozwala wybierać domyślne aplikacje tylko we własnych Ustawieniach. Ten przycisk otwiera tam stronę prev.
settings-default-app-note-macos = macOS prosi o potwierdzenie każdego typu: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP i AVIF.
settings-default-app-status = Typy plików otwierane w prev: { $set } z { $total }.
settings-default-app-button = Ustaw jako domyślną
settings-default-app-button-windows = Otwórz Ustawienia
settings-default-app-no-entry = Wpis pulpitu prev nie jest zainstalowany, więc system nie może nim otwierać plików. Zainstaluj prev z pakietu lub za pomocą scripts/install.sh.
settings-default-app-no-bundle = Otwórz prev z prev.app, aby ustawić ją jako domyślną.
settings-default-app-failed = Nie można ustawić prev jako domyślnej: { $error }
settings-storage = Przechowywanie
settings-version = prev { $version }
settings-version-development = prev { $version } (wersja deweloperska)

## Markup toolbar

markup-tool-select = Zaznaczanie
markup-tool-area = Zaznaczenie prostokątne
markup-tool-sketch = Szkic
markup-tool-draw = Rysowanie
markup-tool-shapes = Kształty
markup-tool-text-box = Pole tekstowe
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Wyróżnianie
markup-tool-note = Notatka
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Podpisz
# A verb: the tool that marks areas to black out.
markup-tool-redact = Zaczernij
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Zastosuj
markup-apply-redactions = Zastosuj zaczernienia
markup-shape-style = Styl kształtu
markup-border-color = Kolor obramowania
markup-fill-color = Kolor wypełnienia
markup-text-style = Styl tekstu
markup-delete = Usuń
markup-undo = Cofnij
markup-redo = Powtórz

## Markup menus

markup-shape-rectangle = Prostokąt
markup-shape-rounded-rectangle = Zaokrąglony prostokąt
markup-shape-oval = Owal
markup-shape-line = Linia
markup-shape-arrow = Strzałka
markup-shape-star = Gwiazda
markup-shape-polygon = Wielokąt
markup-shape-speech-bubble = Dymek
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Lupa
# A shape that darkens the page around it.
markup-shape-mask = Maska
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Wyróżnienie
markup-style-underline = Podkreślenie
markup-style-strikethrough = Przekreślenie
markup-style-squiggly = Falista linia
# Menu section headings.
markup-menu-color = Kolor
markup-menu-font = Czcionka
markup-menu-size = Rozmiar
markup-menu-alignment = Wyrównanie
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pkt
markup-dashed = Przerywana

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Notatka
markup-kind-text-box = Pole tekstowe
markup-kind-stamp = Pieczątka
markup-kind-redaction = Zaczernienie
markup-kind-shape = Kształt
# Tooltips on a note being edited.
markup-note-delete = Usuń notatkę
markup-note-done = Gotowe
markup-note-placeholder = Wpisz notatkę
markup-notes-empty = Brak wyróżnień i notatek
markup-notes-empty-hint = Tutaj pojawią się wyróżnienia, notatki i pola tekstowe.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Strona { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Nie można zmienić dokumentu: { $error }
markup-copy-area-failed = Nie można skopiować obszaru: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = dokument został zamknięty
markup-render-area-failed = nie można wyrenderować obszaru
markup-copy-stopped = kopiowanie przerwane

## Signatures

signature-menu-empty = Nie ma jeszcze podpisów.
signature-delete = Usuń podpis
signature-create = Utwórz podpis…
signature-dialog-title = Utwórz podpis
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Narysuj
signature-tab-type = Wpisz
signature-tab-image = Obraz
signature-draw-hint = Podpisz się na linii myszą, piórem lub gładzikiem.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Imię i nazwisko
signature-image-hint = Wybierz zdjęcie lub skan swojego podpisu na białym papierze.
signature-choose-image = Wybierz obraz…
# Placeholder of the field naming the signature in the library.
signature-description = Opis, np. „Imię i nazwisko” lub „Inicjały”
# Clears the drawing, typed name or image.
signature-clear = Wyczyść
# The color the signature is drawn or typed in.
signature-ink = Atrament
# The pen's width, for drawing.
signature-thickness = Grubość
signature-sign-first = Najpierw się podpisz, potem zapisz.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Podpis { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Nie można zmienić podpisów: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = brak folderu danych: nie ustawiono zmiennej HOME
signature-removing-stopped = usuwanie przerwane
signature-saving-stopped = zapisywanie przerwane
signature-reading-stopped = odczyt przerwany
signature-not-an-image = prev nie może odczytać tego pliku jako obrazu
signature-no-frames = obraz nie ma klatek
signature-not-found = nie znaleziono podpisu na obrazie

## Dragging

drag-pages-need-document = Strony można upuścić na dokument.
drag-image-unsupported = prev nie może otworzyć tego obrazu.
# $error is a lowercase reason or a technical message.
drag-area-failed = Nie można przeciągnąć obszaru: { $error }
drag-pages-failed = Nie można przeciągnąć stron: { $error }
drag-start-failed = Nie można rozpocząć przeciągania.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Strony
drag-file-one-page = { $name } (str. { $page })
drag-file-page-range = { $name } (str. { $first }–{ $last })

## PDF window

pdf-opening = Otwieranie…
pdf-open-failed = prev nie może otworzyć tego dokumentu
pdf-no-pages = Dokument nie ma stron.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = dokument został zamknięty
pdf-keep-original-failed = nie można zachować oryginalnej wersji: { $error }
pdf-save-failed = Nie można zapisać: { $error }
pdf-nothing-to-paste = Nie ma nic do wklejenia.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = wklejanie przerwane
pdf-file-dialog-failed = Nie można wyświetlić okna wyboru pliku: { $error }
pdf-bookmarks-no-home = Nie można zapisać zakładek: nie ustawiono zmiennej HOME
pdf-bookmarks-save-failed = Nie można zapisać zakładek: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Strona { $page }

# Password prompt. $name is the file name.
pdf-password-protected = Plik „{ $name }” jest chroniony hasłem
pdf-password = Hasło
pdf-password-wrong = Nieprawidłowe hasło. Spróbuj ponownie.
# Button that opens a locked document.
pdf-unlock = Odblokuj

# Toolbar tooltips and labels.
pdf-sidebar = Pasek boczny
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = z { $count }
pdf-zoom-out = Pomniejsz
pdf-zoom-in = Powiększ
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Dopasuj stronę
pdf-fit-width = Dopasuj szerokość
pdf-actual-size = Rzeczywisty rozmiar
pdf-view-continuous = Przewijanie ciągłe
pdf-view-single-page = Jedna strona
pdf-view-two-pages = Dwie strony
pdf-undo = Cofnij
pdf-redo = Powtórz
pdf-rotate-left = Obróć w lewo
pdf-rotate-right = Obróć w prawo
pdf-inspector = Inspektor
pdf-markup = Oznaczenia
# Tooltip of the button that opens the export dialog.
pdf-export = Eksportuj
pdf-settings = Ustawienia

# Search field.
pdf-search = Szukaj
pdf-search-not-found = Nie znaleziono
pdf-searching = Wyszukiwanie…
# The match shown, of all matches found.
pdf-search-match = { $current } z { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } z { $total }+

# Inspector: section headings.
pdf-inspector-file = Plik
pdf-inspector-document = Dokument
pdf-inspector-pages = Strony
# Inspector: fact labels and values.
pdf-inspector-title = Tytuł
pdf-inspector-author = Autor
pdf-inspector-subject = Temat
pdf-inspector-keywords = Słowa kluczowe
pdf-inspector-created = Utworzono
pdf-inspector-modified = Zmodyfikowano
pdf-inspector-application = Aplikacja
pdf-inspector-producer = Producent PDF
pdf-inspector-version = Wersja
pdf-inspector-security = Zabezpieczenia
pdf-inspector-not-encrypted = Niezaszyfrowany
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Zaszyfrowany ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } strona
    [few] { $count } strony
    [many] { $count } stron
   *[other] { $count } strony
}
pdf-inspector-page-size = Rozmiar strony
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } cala)
pdf-loading = Wczytywanie…

# Sidebar tabs and lists.
pdf-tab-pages = Strony
pdf-tab-contents = Spis treści
pdf-tab-notes = Wyróżnienia i notatki
pdf-tab-bookmarks = Zakładki
pdf-no-outline = Brak spisu treści
pdf-no-outline-detail = Ten dokument nie ma spisu treści.
pdf-no-bookmarks = Brak zakładek
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Naciśnij { $keys }, aby dodać zakładkę do strony.
pdf-no-bookmarks-detail-unbound = Tutaj pojawiają się strony z zakładkami.
pdf-remove-bookmark = Usuń zakładkę

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Strony
pages-insert-blank = Wstaw pustą stronę
pages-insert-file = Wstaw z pliku…
pages-copy = { $count ->
    [one] Kopiuj stronę
    [few] Kopiuj strony
    [many] Kopiuj strony
   *[other] Kopiuj strony
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Wklej stronę
    [few] Wklej { $count } strony
    [many] Wklej { $count } stron
   *[other] Wklej { $count } strony
}
pages-crop = Przytnij do zaznaczenia
pages-select-all = Zaznacz wszystkie strony
pages-delete = { $count ->
    [one] Usuń stronę
    [few] Usuń strony
    [many] Usuń strony
   *[other] Usuń strony
}
pages-apply-redactions = Zastosuj zaczernienia…
pages-no-copied = Nie ma skopiowanych stron do wklejenia.
pages-copied = { $count ->
    [one] Skopiowano { $count } stronę.
    [few] Skopiowano { $count } strony.
    [many] Skopiowano { $count } stron.
   *[other] Skopiowano { $count } strony.
}
pages-copy-failed = Nie można skopiować stron: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = odczyt przerwany
pages-read-failed = Nie można odczytać pliku: { $error }
pages-at-least-one = Dokument musi mieć co najmniej jedną stronę.
pages-crop-needs-area = Najpierw wybierz obszar narzędziem zaznaczenia prostokątnego.
pages-change-failed = Nie można zmienić stron: { $error }
pages-no-redactions = Nie było zaczernień do zastosowania.
pages-redactions-applied = { $count ->
    [one] Zastosowano { $count } zaczernienie.
    [few] Zastosowano { $count } zaczernienia.
    [many] Zastosowano { $count } zaczernień.
   *[other] Zastosowano { $count } zaczernienia.
}
pages-forget-versions-failed = Nie można usunąć wcześniejszych wersji: { $error }
pages-redact-title = Zastosować zaczernienia?
pages-redact-body = { $count ->
    [one] Tekst, obrazy i rysunki pod oznaczeniem zostaną trwale usunięte z dokumentu, a oznaczenie stanie się czarnym prostokątem. Tej operacji nie można cofnąć. Zostaną też usunięte wcześniejsze wersje tego pliku przechowywane przez prev.
    [few] Tekst, obrazy i rysunki pod { $count } oznaczeniami zostaną trwale usunięte z dokumentu, a oznaczenia staną się czarnymi prostokątami. Tej operacji nie można cofnąć. Zostaną też usunięte wcześniejsze wersje tego pliku przechowywane przez prev.
    [many] Tekst, obrazy i rysunki pod { $count } oznaczeniami zostaną trwale usunięte z dokumentu, a oznaczenia staną się czarnymi prostokątami. Tej operacji nie można cofnąć. Zostaną też usunięte wcześniejsze wersje tego pliku przechowywane przez prev.
   *[other] Tekst, obrazy i rysunki pod { $count } oznaczeniami zostaną trwale usunięte z dokumentu, a oznaczenia staną się czarnymi prostokątami. Tej operacji nie można cofnąć. Zostaną też usunięte wcześniejsze wersje tego pliku przechowywane przez prev.
}
# Button that applies redactions.
pages-redact-apply = Zastosuj

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Eksportuj
pages-export-format = Format
pages-export-reduce = Zmniejsz rozmiar pliku (obrazy w 150 dpi)
pages-export-flatten = Spłaszcz adnotacje i pola formularzy
pages-export-flatten-detail = Oznaczenia i wypełnione pola staną się częścią stron i nie będzie można ich już edytować. Niezastosowane jeszcze zaczernienia zostaną pominięte.
pages-export-encrypt = Zaszyfruj hasłem
pages-export-password = Hasło
pages-export-verify-password = Potwierdź hasło
pages-export-resolution = Rozdzielczość
pages-export-dpi = { $dpi } dpi
pages-export-quality = Jakość
# JPEG quality choices.
pages-export-quality-low = Niska
pages-export-quality-medium = Średnia
pages-export-quality-high = Wysoka
pages-export-quality-best = Najlepsza
pages-export-one-file = Wszystkie strony trafią do jednego pliku.
pages-export-file-per-page = Każda strona zostanie zapisana jako osobny plik, numerowany po wybranej nazwie.
pages-export-selected-only = { $count ->
    [one] Tylko zaznaczona strona
    [few] Tylko { $count } zaznaczone strony
    [many] Tylko { $count } zaznaczonych stron
   *[other] Tylko { $count } zaznaczonej strony
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Eksportuj…
pages-export-no-password = Wprowadź hasło.
pages-export-password-mismatch = Hasła nie są zgodne.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (eksport)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = dokument
pages-export-same-file = Eksportuj do nowego pliku; ten dokument zapisuje się sam.
pages-export-exporting = Eksportowanie „{ $name }”…
pages-export-done = Wyeksportowano „{ $name }”.
pages-export-done-images = Wyeksportowane obrazy: { $count }.
pages-export-failed = Nie można wyeksportować: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = eksportowanie przerwane

## Start window

# Under the app name in a window with no file open.
app-start-hint = Otwórz lub upuść tutaj dokument PDF, obraz, rysunek SVG albo plik Markdown.
app-start-open = Otwórz…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (wersja deweloperska)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: ta przeglądarka nie jest jeszcze gotowa.
app-cannot-open = prev nie może otwierać plików tego rodzaju.
app-cannot-read = prev nie może odczytać tego pliku: { $error }
app-kind-pdf = Dokument PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Obraz { $format }
app-kind-svg = Rysunek SVG
app-kind-markdown = Dokument Markdown
app-file-dialog-failed = Nie można wyświetlić okna wyboru pliku: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Otwórz
action-settings = Ustawienia

## Toolbar

app-toolbar-keep-shown = Zawsze pokazuj pasek narzędzi
app-toolbar-auto-hide = Ukrywaj pasek narzędzi, gdy wskaźnik opuści okno
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Więcej

## File facts

# Labels in a file's inspector.
app-fact-name = Nazwa
app-fact-folder = Folder
app-fact-size = Rozmiar
app-fact-modified = Zmodyfikowano
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count ->
    [one] { $count } bajt
    [few] { $count } bajty
    [many] { $count } bajtów
   *[other] { $count } bajta
}
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Nieprawidłowy link { $uri }: { $error }
app-link-open-failed = Nie można otworzyć { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = zainstaluj wl-clipboard, aby wklejać obrazy
app-copy-needs-wl-clipboard = zainstaluj wl-clipboard, aby kopiować obrazy
app-copy-no-pixels = obszar nie zawiera pikseli
# wl-copy is a program's name.
app-copy-no-input = wl-copy nie otrzymał danych
app-copy-failed = błąd wl-copy
app-clipboard-open-failed = Nie można otworzyć schowka: { $error }
app-copy-image-failed = Nie można skopiować obrazu: { $error }

## Printing

print-failed = Nie można wydrukować: { $error }
print-stopped = Drukowanie przerwane
print-unavailable = Drukowanie nie jest jeszcze dostępne w tym systemie.
print-no-window = Nie można wydrukować: brak okna, nad którym można pokazać okno drukowania
print-dialog-failed = Nie można wyświetlić okna drukowania: { $error }
# Shown after "Could not print:".
print-job-not-started = drukarka nie rozpoczęła zadania
# Shown after "Could not print:".
print-printer-stopped = drukarka się zatrzymała

## File dialogs

dialog-open = Otwórz
dialog-filter-all = Wszystkie obsługiwane pliki
dialog-filter-pdf = Dokumenty PDF
dialog-filter-images = Obrazy
dialog-filter-svg = Rysunki SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Wybierz folder podpisów
dialog-choose-versions = Wybierz folder historii wersji
dialog-choose-bookmarks = Wybierz plik zakładek

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Użycie: prev [FILE]...

    Przeglądanie i edycja plików PDF i obrazów. Pliki otwierają się w oknach
    działającej aplikacji prev, która w razie potrzeby się uruchomi.

    Opcje:
      -h, --help     Pokaż tę pomoc
      -V, --version  Pokaż wersję

## Settings, continued

settings-language = Język
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Zgodnie z systemem: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Język wprowadzania
settings-input-language-system = Zgodnie z układem klawiatury
settings-input-language-note = Określa, po której stronie zaczyna się puste pole tekstowe. Wpisany tekst zachowuje własny kierunek.

settings-appearance-system = Systemowy
settings-appearance-light = Jasny
settings-appearance-dark = Ciemny
settings-omarchy-accent = Użyj koloru akcentu Omarchy
# $theme is the Omarchy theme's name.
settings-omarchy-note = Kolory są tworzone na podstawie akcentu motywu „{ $theme }”.
settings-omarchy-none = Żaden motyw Omarchy nie jest aktywny.
settings-auto-hide = Ukrywaj pasek narzędzi, gdy wskaźnik opuści okno
settings-auto-hide-note = Pasek narzędzi unosi się nad dokumentem i chowa się, gdy wskaźnik jest poza oknem.
settings-animations = Animacje
settings-animations-note = Wysuwane paski i panele, rozwijane okna dialogowe i sprężyste przyciski.
settings-animations-reduced = Wyłączone, gdy system prosi o ograniczenie ruchu.
settings-corner-radius = Promień narożników
settings-corner-radius-note = Dla okien dialogowych i pływającego paska narzędzi.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Przezroczystość paska
settings-overlay-note = Na ile strona prześwituje przez pływający pasek narzędzi.
settings-overlay-value = { $percent }%
settings-storage-signatures = Folder podpisów
settings-storage-versions = Folder historii wersji
settings-storage-bookmarks = Plik zakładek
settings-storage-apply = Zastosuj
settings-storage-choose = Wybierz…
# $file is where the settings file is.
settings-storage-note = Pliki przechowywane już w starym miejscu zostają tam; przenieś je, aby nadal z nich korzystać. Ustawienia prev są zapisywane w { $file }.
settings-save-failed = Nie można zapisać ustawień: { $error }
settings-no-location = Brak miejsca na ustawienia: nie ustawiono zmiennej HOME
settings-full-path = Użyj pełnej ścieżki, np. ~/Documents/prev.
settings-path-is-folder = { $path } to folder, a nie plik.
settings-folder-missing = Folder { $path } nie istnieje. Najpierw go utwórz albo wybierz inny.
settings-path-is-file = { $path } to plik, a nie folder.
settings-cannot-write = prev nie może zapisywać w { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Eksportuj
# Section headings in the export dialog.
export-format = Format
export-quality = Jakość
export-size = Rozmiar
# Button that goes on to choose where to save the export.
export-choose = Eksportuj…
# Format choice; the format name stays as it is.
export-format-webp = WebP (bezstratny)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = obraz
# JPEG quality choices.
export-quality-low = Niska
export-quality-medium = Średnia
export-quality-high = Wysoka
export-quality-best = Najlepsza
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Rzeczywisty rozmiar
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } px
# $error is the system's reason.
export-dialog-failed = Nie można wyświetlić okna zapisywania: { $error }
# $path is where the file was saved.
export-done = Wyeksportowano { $path }
export-failed = Nie można wyeksportować: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = eksportowanie przerwane

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Obrazów z oznaczeniami nie można edytować. Wyeksportuj obraz, aby zachować oznaczenia, albo usuń je i zamknij pasek oznaczeń.

# Shown if a background task ends unexpectedly.
image-loading-stopped = wczytywanie przerwane
image-reverting-stopped = przywracanie przerwane
image-rendering-stopped = renderowanie przerwane
image-saving-stopped = zapisywanie przerwane
image-markup-stopped = oznaczanie przerwane
image-no-version-store = Brak miejsca na wersje
image-revert-failed = Nie można przywrócić: { $error }
image-read-failed = Nie można odczytać { $path }: { $error }
image-keep-original-failed = Nie można zachować oryginalnej wersji: { $error }
image-save-failed = Nie można zapisać { $path }: { $error }
image-markup-start-failed = Nie można rozpocząć oznaczania: { $error }
image-cannot-edit = Animacji i rysunków SVG nie można edytować.
image-cannot-mark-up = Na animacjach i rysunkach SVG nie można dodawać oznaczeń.
image-mark-up-wait = Poczekaj na zakończenie edycji, a potem dodaj oznaczenia.
image-crop-needs-selection = Najpierw przeciągnij zaznaczenie (narzędzie Zaznaczanie), a potem przytnij.
image-size-needed = Wprowadź szerokość i wysokość w pikselach.
# $name is a file name.
image-cannot-save-format = Zmian w pliku „{ $name }” nie można zapisać w jego formacie. Użyj eksportu ({ $keys }).
image-cannot-save-format-unbound = Zmian w pliku „{ $name }” nie można zapisać w jego formacie. Użyj eksportu.
image-cannot-export-animation = Animacji nie można jeszcze eksportować.
image-drop-pages = Strony można upuścić na dokument.
image-drag-failed = Nie można rozpocząć przeciągania.
image-open-failed = prev nie może otworzyć tego obrazu
image-opening = Otwieranie…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Nazwa nie pasuje do formatu
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = Plik „{ $name }” zostanie zapisany w formacie { $format }, ale jego nazwa kończy się na .{ $extension }. Inne aplikacje mogą go nie otworzyć.
image-name-mismatch-no-extension = Plik „{ $name }” zostanie zapisany w formacie { $format }, ale jego nazwa nie ma rozszerzenia. Inne aplikacje mogą go nie otworzyć.
image-choose-again = Wybierz ponownie
image-save-as-is = Zapisz bez zmian
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = klatka { $current } z { $total }
image-position = { $current } z { $total }
image-edited = zmieniony
# Toolbar tooltips.
image-sidebar = Pasek boczny
image-zoom-out = Pomniejsz
image-zoom-in = Powiększ
image-zoom = { $percent }%
image-fit = Dopasuj do okna
image-actual-size = Rzeczywisty rozmiar
image-undo = Cofnij
image-redo = Powtórz
image-rotate-left = Obróć w lewo
image-rotate-right = Obróć w prawo
image-flip-horizontal = Odbij w poziomie
image-flip-vertical = Odbij w pionie
image-select = Zaznaczenie prostokątne
image-crop = Przytnij do zaznaczenia
image-adjust-size-tool = Dopasuj rozmiar
image-adjust-color-tool = Dopasuj kolor
# Tooltip and panel title.
image-inspector = Inspektor
image-markup = Oznaczenia
image-export = Eksportuj
image-settings = Ustawienia
# Panel titles.
image-adjust-color = Dopasuj kolor
image-adjust-size = Dopasuj rozmiar
# Adjust Color sliders.
image-exposure = Ekspozycja
image-contrast = Kontrast
image-saturation = Nasycenie
image-temperature = Temperatura
image-tint = Odcień
image-sepia = Sepia
image-sharpness = Ostrość
image-levels = Poziomy
image-black-point = Punkt czerni
image-midtones = Półtony
image-white-point = Punkt bieli
image-reset-all = Resetuj wszystko
# Adjust Size panel.
image-current-size = Obecny rozmiar: { $width } × { $height } px
image-width = Szerokość
image-height = Wysokość
image-scale-proportionally = Skaluj proporcjonalnie
# Button that applies the new size.
image-resize = Zmień rozmiar
# Inspector panel.
image-inspector-loading = Wczytywanie…
image-file = Plik
image-format = Format
image-dimensions-label = Wymiary
image-pixels = { $width } × { $height } px
image-no-camera = Brak informacji o aparacie.
image-location = Lokalizacja
image-remove-location = Usuń dane lokalizacji
image-no-location = Brak informacji o lokalizacji.
image-keywords-description = Słowa kluczowe i opis
image-keywords-hint = Słowa kluczowe oddzielone przecinkami
image-description = Opis
image-keywords-unsupported = Słowa kluczowe można zapisać w plikach JPEG, PNG i WebP.
# Heading over the earlier versions of the file.
image-revert-to = Przywróć do
image-no-versions = Brak wcześniejszych wersji.
image-revert = Przywróć
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Zamknąć bez eksportowania oznaczeń?
image-close-body = { $count ->
    [one] Oznaczenia na obrazie istnieją tylko do zamknięcia jego okna. Aby je zachować, wyeksportuj obraz: oznaczenia zostaną naniesione na zapisaną kopię.
    [few] Oznaczenia na obrazach istnieją tylko do zamknięcia ich okien. Aby je zachować, wyeksportuj każdy obraz: oznaczenia zostaną naniesione na zapisaną kopię.
    [many] Oznaczenia na obrazach istnieją tylko do zamknięcia ich okien. Aby je zachować, wyeksportuj każdy obraz: oznaczenia zostaną naniesione na zapisaną kopię.
   *[other] Oznaczenia na obrazach istnieją tylko do zamknięcia ich okien. Aby je zachować, wyeksportuj każdy obraz: oznaczenia zostaną naniesione na zapisaną kopię.
}
image-close-anyway = Zamknij mimo to

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = odczyt przerwany
markdown-read-failed = prev nie może odczytać tego pliku
markdown-draw-failed = Nie można narysować dokumentu
# Under the export's size choices.
markdown-export-size = Cały dokument, { $width } × { $height } px
# Search results.
markdown-not-found = Nie znaleziono
markdown-match = { $current } z { $total }
# Placeholder of the search field.
markdown-search = Szukaj
# Toolbar tooltips.
markdown-smaller-text = Mniejszy tekst
markdown-larger-text = Większy tekst
markdown-zoom = { $percent }%
markdown-actual-size = Rzeczywisty rozmiar
# Tooltip and panel title.
markdown-inspector = Inspektor
markdown-export = Eksportuj
markdown-settings = Ustawienia
# Inspector headings and labels.
markdown-file = Plik
markdown-document = Dokument
markdown-words = Słowa
markdown-lines = Wiersze
markdown-pictures = Obrazy

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Aparat
image-meta-exposure = Ekspozycja
image-meta-image = Obraz
image-meta-make = Producent
image-meta-model = Model
image-meta-lens = Obiektyw
image-meta-exposure-time = Czas naświetlania
# The lens aperture, written like f/2.8.
image-meta-f-number = Przysłona
image-meta-iso = ISO
image-meta-focal-length = Ogniskowa
image-meta-exposure-bias = Korekta ekspozycji
image-meta-flash = Lampa błyskowa
image-meta-date-taken = Data wykonania
image-meta-orientation = Orientacja
image-meta-color-space = Przestrzeń kolorów
image-meta-software = Oprogramowanie
image-meta-artist = Autor
image-meta-copyright = Prawa autorskie
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normalna
    [2] Odbicie w poziomie
    [3] Obrót o 180°
    [4] Odbicie w pionie
    [5] Odbicie w poziomie, obrót o 90° w lewo
    [6] Obrót o 90° w prawo
    [7] Odbicie w poziomie, obrót o 90° w prawo
    [8] Obrót o 90° w lewo
   *[other] Nieznana ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Lampa zadziałała
   *[no] Lampa nie zadziałała
}{ $mode ->
    [on] , wymuszona
    [off] , wyłączona
    [auto] , automatyczna
   *[unknown] {""}
}{ $redeye ->
    [yes] , redukcja czerwonych oczu
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Nieskalibrowana
   *[other] Inna ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = nie można otworzyć dokumentu: { $detail }
error-pdf-page-out-of-range = strona { $page } nie istnieje
error-pdf-password-protected = dokument jest chroniony hasłem; otwórz go i skopiuj z niego strony
error-pdf-no-pages = brak stron do wyodrębnienia
error-pdf-crop-outside = obszar przycinania jest poza stroną
error-pdf-closed = dokument zamknięty
error-pdf-saved-unreadable = zapisany dokument już się nie otwiera
error-image-read = nie można odczytać pliku: { $detail }
error-image-invalid = obraz jest uszkodzony lub nieprawidłowy: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = otwarcie tego formatu wymaga biblioteki { $library }, która nie jest zainstalowana
# $format is an image format name, such as HEIC.
error-image-unsupported = obrazy { $format } nie są jeszcze obsługiwane
error-image-encode = nie można zakodować obrazu: { $detail }
error-exif-malformed = dane EXIF są uszkodzone
error-settings-read = nie można odczytać ustawień: { $detail }
error-settings-invalid = nieprawidłowe ustawienia: { $detail }
error-remove-location = nie można usunąć lokalizacji: { $error }
error-location-unsupported = dane lokalizacji można usunąć z plików JPEG, PNG, WebP i TIFF
error-xmp-unsupported = słowa kluczowe i opisy można zapisać tylko w plikach JPEG, PNG i WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW z aparatu

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev — informacje
menu-settings = Ustawienia…
menu-services = Usługi
menu-hide = Ukryj prev
menu-hide-others = Ukryj pozostałe
menu-show-all = Pokaż wszystko
menu-quit = Zakończ prev
menu-file = Plik
menu-open = Otwórz…
menu-close = Zamknij okno
menu-export = Eksportuj…
menu-print = Drukuj…
menu-edit = Edycja
menu-undo = Cofnij
menu-redo = Powtórz
menu-cut = Wytnij
menu-copy = Kopiuj
menu-paste = Wklej
menu-select-all = Zaznacz wszystko
menu-find = Znajdź
menu-find-next = Znajdź następny
menu-find-previous = Znajdź poprzedni
menu-view = Widok
menu-hide-sidebar = Ukryj pasek boczny
menu-thumbnails = Miniaturki
menu-contents = Spis treści
menu-notes = Wyróżnienia i notatki
menu-bookmarks = Zakładki
menu-zoom-in = Powiększ
menu-zoom-out = Pomniejsz
menu-actual-size = Rzeczywisty rozmiar
menu-zoom-to-fit = Dopasuj do okna
menu-inspector = Pokaż inspektora
menu-slideshow = Pokaz slajdów
menu-full-screen = Włącz tryb pełnoekranowy
menu-go = Idź
menu-next-page = Następna strona
menu-previous-page = Poprzednia strona
menu-go-to-page = Idź do strony…
menu-bookmark = Dodaj zakładkę
menu-tools = Narzędzia
menu-markup = Pokaż pasek narzędzi oznaczeń
menu-rotate-left = Obróć w lewo
menu-rotate-right = Obróć w prawo
menu-crop = Przytnij
menu-adjust-color = Dopasuj kolor…
menu-window = Okno
menu-minimize = Minimalizuj
menu-zoom = Zmień rozmiar
menu-bring-all-to-front = Przenieś wszystko na wierzch
