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
language-name = Українська

## Common

common-cancel = Скасувати
common-close = Закрити
common-save = Зберегти

## Settings

settings-title = Параметри
settings-appearance = Оформлення
settings-colors = Кольори
settings-windows = Вікна
settings-default-app = Застосунок за замовчуванням
settings-default-app-label = Відкривати файли в prev
settings-default-app-note = Зробити prev застосунком, що відкриває PDF, зображення, рисунки SVG і файли Markdown.
settings-default-app-note-windows = У Windows застосунки за замовчуванням вибирають лише в її власних параметрах. Кнопка відкриває там сторінку prev.
settings-default-app-note-macos = macOS просить підтвердити кожен тип: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP і AVIF.
settings-default-app-status = Типи файлів, що відкриваються в prev: { $set } з { $total }.
settings-default-app-button = Зробити за замовчуванням
settings-default-app-button-windows = Відкрити параметри
settings-default-app-no-entry = Ярлик робочого столу prev не встановлено, тому система не може відкривати ним файли. Установіть prev з пакета або за допомогою scripts/install.sh.
settings-default-app-no-bundle = Відкрийте prev з prev.app, щоб зробити його застосунком за замовчуванням.
settings-default-app-failed = Не вдалося зробити prev застосунком за замовчуванням: { $error }
settings-storage = Зберігання
settings-version = prev { $version }
settings-version-development = prev { $version } (збірка для розробки)

## Markup toolbar

markup-tool-select = Вибір
markup-tool-area = Прямокутне виділення
markup-tool-sketch = Ескіз
markup-tool-draw = Малювання
markup-tool-shapes = Фігури
markup-tool-text-box = Текстове поле
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Маркер
markup-tool-note = Нотатка
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Підписати
# A verb: the tool that marks areas to black out.
markup-tool-redact = Зачорнити
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Застосувати
markup-apply-redactions = Застосувати зачорнення
markup-shape-style = Стиль фігури
markup-border-color = Колір межі
markup-fill-color = Колір заливки
markup-text-style = Стиль тексту
markup-delete = Видалити
markup-undo = Відмінити
markup-redo = Повторити

## Markup menus

markup-shape-rectangle = Прямокутник
markup-shape-rounded-rectangle = Заокруглений прямокутник
markup-shape-oval = Овал
markup-shape-line = Лінія
markup-shape-arrow = Стрілка
markup-shape-star = Зірка
markup-shape-polygon = Багатокутник
markup-shape-speech-bubble = Виноска
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Лупа
# A shape that darkens the page around it.
markup-shape-mask = Маска
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Виділення кольором
markup-style-underline = Підкреслення
markup-style-strikethrough = Закреслення
markup-style-squiggly = Хвиляста лінія
# Menu section headings.
markup-menu-color = Колір
markup-menu-font = Шрифт
markup-menu-size = Розмір
markup-menu-alignment = Вирівнювання
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } пт
markup-dashed = Пунктир

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Нотатка
markup-kind-text-box = Текстове поле
markup-kind-stamp = Штамп
markup-kind-redaction = Зачорнення
markup-kind-shape = Фігура
# Tooltips on a note being edited.
markup-note-delete = Видалити нотатку
markup-note-done = Готово
markup-note-placeholder = Введіть нотатку
markup-notes-empty = Немає виділень і нотаток
markup-notes-empty-hint = Тут з’являться виділення, нотатки й текстові поля.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Сторінка { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Не вдалося змінити документ: { $error }
markup-copy-area-failed = Не вдалося скопіювати область: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = документ закрито
markup-render-area-failed = не вдалося відобразити область
markup-copy-stopped = копіювання перервано

## Signatures

signature-menu-empty = Підписів ще немає.
signature-delete = Видалити підпис
signature-create = Створити підпис…
signature-dialog-title = Створення підпису
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Намалювати
signature-tab-type = Ввести
signature-tab-image = Зображення
signature-draw-hint = Розпишіться на лінії мишею, пером або трекпадом.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Ваше ім’я
signature-image-hint = Виберіть фото або скан свого підпису на білому папері.
signature-choose-image = Вибрати зображення…
# Placeholder of the field naming the signature in the library.
signature-description = Опис, наприклад «Повне ім’я» чи «Ініціали»
# Clears the drawing, typed name or image.
signature-clear = Очистити
# The color the signature is drawn or typed in.
signature-ink = Чорнило
# The pen's width, for drawing.
signature-thickness = Товщина
signature-sign-first = Спершу розпишіться, потім збережіть.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Підпис { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Не вдалося змінити підписи: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = немає папки даних: HOME не задано
signature-removing-stopped = видалення перервано
signature-saving-stopped = збереження перервано
signature-reading-stopped = читання перервано
signature-not-an-image = prev не може прочитати цей файл як зображення
signature-no-frames = у зображенні немає кадрів
signature-not-found = у зображенні не знайдено підпису

## Dragging

drag-pages-need-document = Сторінки можна перетягнути в документ.
drag-image-unsupported = prev не може відкрити це зображення.
# $error is a lowercase reason or a technical message.
drag-area-failed = Не вдалося перетягнути область: { $error }
drag-pages-failed = Не вдалося перетягнути сторінки: { $error }
drag-start-failed = Не вдалося почати перетягування.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Сторінки
drag-file-one-page = { $name } (с. { $page })
drag-file-page-range = { $name } (с. { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = Зображення
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = Додати до цього документа?
drop-pdf-body = Додати «{ $name }» у кінець цього документа чи відкрити в окремому вікні?
drop-pdfs-body = { $count ->
    [one] Додати ці { $count } PDF-файл у кінець цього документа чи відкрити їх в окремих вікнах?
    [few] Додати ці { $count } PDF-файли у кінець цього документа чи відкрити їх в окремих вікнах?
    [many] Додати ці { $count } PDF-файлів у кінець цього документа чи відкрити їх в окремих вікнах?
   *[other] Додати ці { $count } PDF-файлу у кінець цього документа чи відкрити їх в окремих вікнах?
}
drop-pdf-add = Додати в кінець
drop-pdf-open = Відкрити окремо

## PDF window

pdf-opening = Відкриття…
pdf-open-failed = prev не може відкрити цей документ
pdf-no-pages = У документі немає сторінок.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = документ закрито
pdf-keep-original-failed = не вдалося зберегти початкову версію: { $error }
pdf-save-failed = Не вдалося зберегти: { $error }
pdf-nothing-to-paste = Нічого вставляти.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = вставлення перервано
pdf-file-dialog-failed = Не вдалося показати вікно вибору файлу: { $error }
pdf-bookmarks-no-home = Не вдається зберегти закладки: HOME не задано
pdf-bookmarks-save-failed = Не вдалося зберегти закладки: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Сторінка { $page }

# Password prompt. $name is the file name.
pdf-password-protected = Файл «{ $name }» захищено паролем
pdf-password = Пароль
pdf-password-wrong = Неправильний пароль. Спробуйте ще раз.
# Button that opens a locked document.
pdf-unlock = Розблокувати

# Toolbar tooltips and labels.
pdf-sidebar = Бічна панель
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = з { $count }
pdf-zoom-out = Зменшити
pdf-zoom-in = Збільшити
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = За розміром сторінки
pdf-fit-width = За шириною
pdf-actual-size = Справжній розмір
pdf-view-continuous = Безперервне прокручування
pdf-view-single-page = Одна сторінка
pdf-view-two-pages = Дві сторінки
pdf-undo = Відмінити
pdf-redo = Повторити
pdf-rotate-left = Повернути ліворуч
pdf-rotate-right = Повернути праворуч
pdf-inspector = Інспектор
pdf-markup = Розмітка
# Tooltip of the button that opens the export dialog.
pdf-export = Експорт
pdf-settings = Параметри

# Search field.
pdf-search = Пошук
pdf-search-not-found = Не знайдено
pdf-searching = Пошук…
# The match shown, of all matches found.
pdf-search-match = { $current } з { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } з { $total }+

# Inspector: section headings.
pdf-inspector-file = Файл
pdf-inspector-document = Документ
pdf-inspector-pages = Сторінки
# Inspector: fact labels and values.
pdf-inspector-title = Назва
pdf-inspector-author = Автор
pdf-inspector-subject = Тема
pdf-inspector-keywords = Ключові слова
pdf-inspector-created = Створено
pdf-inspector-modified = Змінено
pdf-inspector-application = Програма
pdf-inspector-producer = Виробник PDF
pdf-inspector-version = Версія
pdf-inspector-security = Захист
pdf-inspector-not-encrypted = Не зашифровано
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Зашифровано ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } сторінка
    [few] { $count } сторінки
    [many] { $count } сторінок
   *[other] { $count } сторінки
}
pdf-inspector-page-size = Розмір сторінки
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } мм ({ $width_in } × { $height_in } дюйм.)
pdf-loading = Завантаження…

# Sidebar tabs and lists.
pdf-tab-pages = Сторінки
pdf-tab-contents = Зміст
pdf-tab-notes = Виділення й нотатки
pdf-tab-bookmarks = Закладки
pdf-no-outline = Немає змісту
pdf-no-outline-detail = Цей документ не має змісту.
pdf-no-bookmarks = Немає закладок
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Натисніть { $keys }, щоб додати закладку.
pdf-no-bookmarks-detail-unbound = Тут з’являться сторінки із закладками.
pdf-remove-bookmark = Видалити закладку

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Сторінки
pages-insert-blank = Вставити порожню сторінку
pages-insert-file = Вставити з файлу…
pages-copy = { $count ->
    [1] Скопіювати сторінку
    [one] Скопіювати сторінки
    [few] Скопіювати сторінки
    [many] Скопіювати сторінки
   *[other] Скопіювати сторінки
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [1] Вставити сторінку
    [one] Вставити { $count } сторінку
    [few] Вставити { $count } сторінки
    [many] Вставити { $count } сторінок
   *[other] Вставити { $count } сторінки
}
pages-crop = Обтяти за виділенням
pages-select-all = Вибрати всі сторінки
pages-delete = { $count ->
    [1] Видалити сторінку
    [one] Видалити сторінки
    [few] Видалити сторінки
    [many] Видалити сторінки
   *[other] Видалити сторінки
}
pages-apply-redactions = Застосувати зачорнення…
pages-no-copied = Немає скопійованих сторінок для вставлення.
pages-copied = { $count ->
    [one] Скопійовано { $count } сторінку.
    [few] Скопійовано { $count } сторінки.
    [many] Скопійовано { $count } сторінок.
   *[other] Скопійовано { $count } сторінки.
}
pages-copy-failed = Не вдалося скопіювати сторінки: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = читання перервано
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = prev не може прочитати цей файл як зображення
pages-read-failed = Не вдалося прочитати файл: { $error }
pages-at-least-one = У документі має бути хоча б одна сторінка.
pages-crop-needs-area = Спершу виберіть область інструментом прямокутного виділення.
pages-change-failed = Не вдалося змінити сторінки: { $error }
pages-no-redactions = Немає зачорнень для застосування.
pages-redactions-applied = { $count ->
    [one] Застосовано { $count } зачорнення.
    [few] Застосовано { $count } зачорнення.
    [many] Застосовано { $count } зачорнень.
   *[other] Застосовано { $count } зачорнення.
}
pages-forget-versions-failed = Не вдалося видалити попередні версії: { $error }
pages-redact-title = Застосувати зачорнення?
pages-redact-body = { $count ->
    [1] Текст, зображення й малюнки під позначкою буде назавжди видалено з документа, а позначка стане чорним прямокутником. Цю дію не можна відмінити. Попередні версії цього файлу, які зберігає prev, теж буде видалено.
    [one] Текст, зображення й малюнки під { $count } позначкою буде назавжди видалено з документа, а позначки стануть чорними прямокутниками. Цю дію не можна відмінити. Попередні версії цього файлу, які зберігає prev, теж буде видалено.
    [few] Текст, зображення й малюнки під { $count } позначками буде назавжди видалено з документа, а позначки стануть чорними прямокутниками. Цю дію не можна відмінити. Попередні версії цього файлу, які зберігає prev, теж буде видалено.
    [many] Текст, зображення й малюнки під { $count } позначками буде назавжди видалено з документа, а позначки стануть чорними прямокутниками. Цю дію не можна відмінити. Попередні версії цього файлу, які зберігає prev, теж буде видалено.
   *[other] Текст, зображення й малюнки під { $count } позначками буде назавжди видалено з документа, а позначки стануть чорними прямокутниками. Цю дію не можна відмінити. Попередні версії цього файлу, які зберігає prev, теж буде видалено.
}
# Button that applies redactions.
pages-redact-apply = Застосувати

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Експорт
pages-export-format = Формат
pages-export-reduce = Зменшити розмір файлу (зображення 150 dpi)
pages-export-flatten = Звести анотації та поля форм
pages-export-flatten-detail = Розмітка й заповнені поля стануть частиною сторінок, і їх більше не можна буде змінити. Незастосовані зачорнення не ввійдуть у файл.
pages-export-encrypt = Зашифрувати паролем
pages-export-password = Пароль
pages-export-verify-password = Підтвердження пароля
pages-export-resolution = Роздільність
pages-export-dpi = { $dpi } dpi
pages-export-quality = Якість
# JPEG quality choices.
pages-export-quality-low = Низька
pages-export-quality-medium = Середня
pages-export-quality-high = Висока
pages-export-quality-best = Найкраща
pages-export-one-file = Усі сторінки потраплять в один файл.
pages-export-file-per-page = Кожну сторінку буде збережено в окремий файл із номером після вибраної назви.
pages-export-selected-only = { $count ->
    [1] Лише вибрана сторінка
    [one] Лише { $count } вибрана сторінка
    [few] Лише { $count } вибрані сторінки
    [many] Лише { $count } вибраних сторінок
   *[other] Лише { $count } вибраної сторінки
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Експортувати…
pages-export-no-password = Введіть пароль.
pages-export-password-mismatch = Паролі не збігаються.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (експорт)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = документ
pages-export-same-file = Експортуйте в новий файл: цей документ зберігається сам.
pages-export-exporting = Експорт «{ $name }»…
pages-export-done = Файл «{ $name }» експортовано.
pages-export-done-images = Експортовано зображень: { $count }.
pages-export-failed = Не вдалося експортувати: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = експорт перервано

## Start window

# Under the app name in a window with no file open.
app-start-hint = Відкрийте або перетягніть сюди документ PDF, зображення, малюнок SVG чи файл Markdown.
app-start-open = Відкрити…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (розробка)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: цей переглядач ще не готовий.
app-cannot-open = prev не може відкривати файли цього типу.
app-cannot-read = prev не може прочитати цей файл: { $error }
app-kind-pdf = Документ PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Зображення { $format }
app-kind-svg = Малюнок SVG
app-kind-markdown = Документ Markdown
app-file-dialog-failed = Не вдалося показати вікно вибору файлу: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Відкрити
action-settings = Параметри

## Toolbar

app-toolbar-keep-shown = Завжди показувати панель інструментів
app-toolbar-auto-hide = Ховати панель інструментів, коли вказівник поза вікном
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Ще

## File facts

# Labels in a file's inspector.
app-fact-name = Назва
app-fact-folder = Папка
app-fact-size = Розмір
app-fact-modified = Змінено
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count ->
    [one] { $count } байт
    [few] { $count } байти
    [many] { $count } байтів
   *[other] { $count } байта
}
app-size-kb = { $size } КБ
app-size-mb = { $size } МБ
app-size-gb = { $size } ГБ
app-size-tb = { $size } ТБ

## Links and clipboard

app-link-invalid = Недійсне посилання { $uri }: { $error }
app-link-open-failed = Не вдалося відкрити { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = установіть wl-clipboard, щоб вставляти зображення
app-copy-needs-wl-clipboard = установіть wl-clipboard, щоб копіювати зображення
app-copy-no-pixels = в області немає пікселів
# wl-copy is a program's name.
app-copy-no-input = wl-copy не отримав даних
app-copy-failed = збій wl-copy
app-clipboard-open-failed = Не вдалося відкрити буфер обміну: { $error }
app-copy-image-failed = Не вдалося скопіювати зображення: { $error }

## Printing

print-failed = Не вдалося надрукувати: { $error }
print-stopped = Друк перервано
print-unavailable = Друк у цій системі поки недоступний.
print-no-window = Не вдалося надрукувати: немає вікна для діалогу друку
print-dialog-failed = Не вдалося показати діалог друку: { $error }
# Shown after "Could not print:".
print-job-not-started = принтер не почав завдання
# Shown after "Could not print:".
print-printer-stopped = принтер зупинився

## File dialogs

dialog-open = Відкрити
dialog-filter-all = Усі підтримувані файли
dialog-filter-pdf = Документи PDF
dialog-filter-images = Зображення
dialog-filter-svg = Малюнки SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Виберіть папку для підписів
dialog-choose-versions = Виберіть папку для історії версій
dialog-choose-bookmarks = Виберіть файл закладок

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Використання: prev [FILE]...

    Перегляд і редагування PDF та зображень. Файли відкриваються у вікнах
    запущеного prev; якщо його не запущено, він запуститься.

    Параметри:
      -h, --help     Показати цю довідку
      -V, --version  Показати версію

## Settings, continued

settings-language = Мова
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Як у системі: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Мова введення
settings-input-language-system = За розкладкою клавіатури
settings-input-language-note = Визначає, з якого боку починається порожнє текстове поле. Введений текст зберігає свій напрямок.

settings-appearance-system = Як у системі
settings-appearance-light = Світле
settings-appearance-dark = Темне
settings-system-accent = Використовувати акцентний колір системи
# $theme is the Omarchy theme's name.
settings-omarchy-note = Кольори побудовано на основі акцентного кольору теми «{ $theme }».
settings-system-accent-note = Кольори побудовано на основі акцентного кольору системи.
settings-system-accent-none = У системі немає акцентного кольору, тому prev використовує власний.
settings-auto-hide = Ховати панель інструментів, коли вказівник поза вікном
settings-auto-hide-note = Панель інструментів розташовується над документом і ховається, поки вказівник поза вікном.
settings-animations = Анімація
settings-animations-note = Висувні панелі, діалоги, що розгортаються, і пружні кнопки.
settings-animations-reduced = Вимкнено, поки в системі ввімкнено зменшення руху.
settings-corner-radius = Радіус заокруглення
settings-corner-radius-note = Для діалогів і плаваючої панелі інструментів.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Прозорість панелі
settings-overlay-note = Наскільки сторінка просвічує крізь плаваючу панель інструментів.
settings-overlay-value = { $percent }%
settings-storage-signatures = Папка підписів
settings-storage-versions = Папка історії версій
settings-storage-bookmarks = Файл закладок
settings-storage-apply = Застосувати
settings-storage-choose = Вибрати…
# $file is where the settings file is.
settings-storage-note = Файли, що вже зберігаються в старому місці, залишаються там; перенесіть їх, щоб і далі ними користуватися. Параметри prev зберігаються в { $file }.
settings-save-failed = Не вдалося зберегти параметри: { $error }
settings-no-location = Немає куди зберегти параметри: HOME не задано
settings-full-path = Вкажіть повний шлях, наприклад ~/Documents/prev.
settings-path-is-folder = { $path } — це папка, а не файл.
settings-folder-missing = Папки { $path } немає. Спершу створіть її або виберіть іншу.
settings-path-is-file = { $path } — це файл, а не папка.
settings-cannot-write = prev не може записувати в { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Експорт
# Section headings in the export dialog.
export-format = Формат
export-quality = Якість
export-size = Розмір
# Button that goes on to choose where to save the export.
export-choose = Експортувати…
# Format choice; the format name stays as it is.
export-format-webp = WebP (без втрат)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = зображення
# JPEG quality choices.
export-quality-low = Низька
export-quality-medium = Середня
export-quality-high = Висока
export-quality-best = Найкраща
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Справжній розмір
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } пікс.
# $error is the system's reason.
export-dialog-failed = Не вдалося показати вікно збереження: { $error }
# $path is where the file was saved.
export-done = Експортовано: { $path }
export-failed = Не вдалося експортувати: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = експорт перервано

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Зображення з розміткою не можна редагувати. Експортуйте зображення, щоб зберегти розмітку, або видаліть її та закрийте панель розмітки.

# Shown if a background task ends unexpectedly.
image-loading-stopped = завантаження перервано
image-reverting-stopped = повернення версії перервано
image-rendering-stopped = відображення перервано
image-saving-stopped = збереження перервано
image-markup-stopped = розмітку перервано
image-no-version-store = Немає де зберігати версії
image-revert-failed = Не вдалося повернути версію: { $error }
image-read-failed = Не вдалося прочитати { $path }: { $error }
image-keep-original-failed = Не вдалося зберегти початкову версію: { $error }
image-save-failed = Не вдалося зберегти { $path }: { $error }
image-markup-start-failed = Не вдалося запустити розмітку: { $error }
image-cannot-edit = Анімації та малюнки SVG не можна редагувати.
image-cannot-mark-up = Анімації та малюнки SVG не можна розмічати.
image-mark-up-wait = Дочекайтеся завершення редагування, потім розмічайте.
image-crop-needs-selection = Спершу виділіть область (інструмент «Вибір»), потім обтинайте.
image-size-needed = Введіть ширину й висоту в пікселях.
# $name is a file name.
image-cannot-save-format = Зміни у файлі «{ $name }» не можна зберегти в його форматі. Скористайтеся експортом ({ $keys }).
image-cannot-save-format-unbound = Зміни у файлі «{ $name }» не можна зберегти в його форматі. Скористайтеся експортом.
image-cannot-export-animation = Анімації поки не можна експортувати.
image-drop-pages = Сторінки можна перетягнути в документ.
image-drag-failed = Не вдалося почати перетягування.
image-picture-save-failed = Не вдалося зберегти зображення в папку «Завантаження».
image-open-failed = prev не може відкрити це зображення
image-opening = Відкриття…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Назва не відповідає формату
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = Файл «{ $name }» буде збережено у форматі { $format }, але його назва закінчується на .{ $extension }. Інші програми можуть його не відкрити.
image-name-mismatch-no-extension = Файл «{ $name }» буде збережено у форматі { $format }, але його назва не має розширення. Інші програми можуть його не відкрити.
image-choose-again = Вибрати знову
image-save-as-is = Зберегти як є
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = кадр { $current } з { $total }
image-position = { $current } з { $total }
image-edited = змінено
# Toolbar tooltips.
image-sidebar = Бічна панель
image-zoom-out = Зменшити
image-zoom-in = Збільшити
image-zoom = { $percent }%
image-fit = За розміром вікна
image-actual-size = Справжній розмір
image-undo = Відмінити
image-redo = Повторити
image-rotate-left = Повернути ліворуч
image-rotate-right = Повернути праворуч
image-flip-horizontal = Віддзеркалити горизонтально
image-flip-vertical = Віддзеркалити вертикально
image-select = Прямокутне виділення
image-crop = Обтяти за виділенням
image-adjust-size-tool = Змінити розмір
image-adjust-color-tool = Налаштувати колір
# Tooltip and panel title.
image-inspector = Інспектор
image-markup = Розмітка
image-export = Експорт
image-settings = Параметри
# Panel titles.
image-adjust-color = Налаштування кольору
image-adjust-size = Зміна розміру
# Adjust Color sliders.
image-exposure = Експозиція
image-contrast = Контраст
image-saturation = Насиченість
image-temperature = Температура
image-tint = Відтінок
image-sepia = Сепія
image-sharpness = Різкість
image-levels = Рівні
image-black-point = Точка чорного
image-midtones = Середні тони
image-white-point = Точка білого
image-reset-all = Скинути все
# Adjust Size panel.
image-current-size = Поточний розмір: { $width } × { $height } пікс.
image-width = Ширина
image-height = Висота
image-scale-proportionally = Масштабувати пропорційно
# Button that applies the new size.
image-resize = Змінити розмір
# Inspector panel.
image-inspector-loading = Завантаження…
image-file = Файл
image-format = Формат
image-dimensions-label = Розміри
image-pixels = { $width } × { $height } пікс.
image-no-camera = Немає даних про камеру.
image-location = Розташування
image-remove-location = Видалити дані про розташування
image-no-location = Немає даних про розташування.
image-keywords-description = Ключові слова й опис
image-keywords-hint = Ключові слова через кому
image-description = Опис
image-keywords-unsupported = Ключові слова можна зберегти у файлах JPEG, PNG і WebP.
# Heading over the earlier versions of the file.
image-revert-to = Повернути до версії
image-no-versions = Попередніх версій немає.
image-revert = Повернути
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } КБ
image-size-mb = { $size } МБ
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Закрити, не експортувавши розмітку?
image-close-body = { $count ->
    [1] Розмітка на зображенні існує, лише поки відкрите його вікно. Щоб зберегти її, експортуйте зображення: розмітку буде намальовано на збереженій копії.
    [one] Розмітка на зображеннях існує, лише поки відкриті їхні вікна. Щоб зберегти її, експортуйте кожне зображення: розмітку буде намальовано на збереженій копії.
    [few] Розмітка на зображеннях існує, лише поки відкриті їхні вікна. Щоб зберегти її, експортуйте кожне зображення: розмітку буде намальовано на збереженій копії.
    [many] Розмітка на зображеннях існує, лише поки відкриті їхні вікна. Щоб зберегти її, експортуйте кожне зображення: розмітку буде намальовано на збереженій копії.
   *[other] Розмітка на зображеннях існує, лише поки відкриті їхні вікна. Щоб зберегти її, експортуйте кожне зображення: розмітку буде намальовано на збереженій копії.
}
image-close-anyway = Усе одно закрити

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = читання перервано
markdown-read-failed = prev не може прочитати цей файл
markdown-draw-failed = Не вдалося відобразити документ
# Under the export's size choices.
markdown-export-size = Увесь документ, { $width } × { $height } пікс.
# Search results.
markdown-not-found = Не знайдено
markdown-match = { $current } з { $total }
# Placeholder of the search field.
markdown-search = Пошук
# Toolbar tooltips.
markdown-smaller-text = Зменшити текст
markdown-larger-text = Збільшити текст
markdown-zoom = { $percent }%
markdown-actual-size = Справжній розмір
# Tooltip and panel title.
markdown-inspector = Інспектор
markdown-export = Експорт
markdown-settings = Параметри
# Inspector headings and labels.
markdown-file = Файл
markdown-document = Документ
markdown-words = Слова
markdown-lines = Рядки
markdown-pictures = Малюнки

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Камера
image-meta-exposure = Експозиція
image-meta-image = Зображення
image-meta-make = Виробник
image-meta-model = Модель
image-meta-lens = Об’єктив
image-meta-exposure-time = Витримка
# The lens aperture, written like f/2.8.
image-meta-f-number = Діафрагма
image-meta-iso = ISO
image-meta-focal-length = Фокусна відстань
image-meta-exposure-bias = Експокорекція
image-meta-flash = Спалах
image-meta-date-taken = Дата зйомки
image-meta-orientation = Орієнтація
image-meta-color-space = Колірний простір
image-meta-software = Програма
image-meta-artist = Автор
image-meta-copyright = Авторські права
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } с
image-meta-millimeters = { $value } мм
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Звичайна
    [2] Віддзеркалено горизонтально
    [3] Повернуто на 180°
    [4] Віддзеркалено вертикально
    [5] Віддзеркалено горизонтально, повернуто на 90° проти годинникової стрілки
    [6] Повернуто на 90° за годинниковою стрілкою
    [7] Віддзеркалено горизонтально, повернуто на 90° за годинниковою стрілкою
    [8] Повернуто на 90° проти годинникової стрілки
   *[other] Невідомо ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Спрацював
   *[no] Не спрацював
}{ $mode ->
    [on] , примусово
    [off] , вимкнено
    [auto] , авто
   *[unknown] {""}
}{ $redeye ->
    [yes] , усунення червоних очей
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Без калібрування
   *[other] Інший ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = не вдається відкрити документ: { $detail }
error-pdf-page-out-of-range = сторінки { $page } не існує
error-pdf-password-protected = документ захищено паролем; відкрийте його та скопіюйте сторінки звідти
error-pdf-no-pages = немає сторінок для видобування
error-pdf-crop-outside = область обтинання поза межами сторінки
error-pdf-closed = документ закрито
error-pdf-saved-unreadable = збережений документ більше не відкривається
error-image-read = не вдається прочитати файл: { $detail }
error-image-invalid = зображення пошкоджене або недійсне: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = для відкриття цього формату потрібна бібліотека { $library }, але її не встановлено
# $format is an image format name, such as HEIC.
error-image-unsupported = зображення { $format } поки не підтримуються
error-image-encode = не вдається закодувати зображення: { $detail }
error-exif-malformed = дані EXIF пошкоджено
error-settings-read = не вдається прочитати параметри: { $detail }
error-settings-invalid = недійсні параметри: { $detail }
error-remove-location = не вдалося видалити розташування: { $error }
error-location-unsupported = дані про розташування можна видалити з файлів JPEG, PNG, WebP і TIFF
error-xmp-unsupported = ключові слова й описи можна зберегти лише у файлах JPEG, PNG і WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW камери

## The macOS menu bar, named as in macOS's own apps.
menu-about = Про prev
menu-settings = Параметри…
menu-services = Послуги
menu-hide = Сховати prev
menu-hide-others = Сховати інші
menu-show-all = Показати всі
menu-quit = Вийти з prev
menu-file = Файл
menu-open = Відкрити…
menu-close = Закрити вікно
menu-export = Експортувати…
menu-print = Друкувати…
menu-edit = Редагування
menu-undo = Відмінити
menu-redo = Повторити
menu-cut = Вирізати
menu-copy = Скопіювати
menu-paste = Вставити
menu-select-all = Вибрати все
menu-find = Знайти
menu-find-next = Знайти наступне
menu-find-previous = Знайти попереднє
menu-view = Вигляд
menu-hide-sidebar = Сховати бічну панель
menu-thumbnails = Мініатюри
menu-contents = Зміст
menu-notes = Виділення й нотатки
menu-bookmarks = Закладки
menu-zoom-in = Збільшити
menu-zoom-out = Зменшити
menu-actual-size = Справжній розмір
menu-zoom-to-fit = За розміром вікна
menu-inspector = Показати інспектор
menu-slideshow = Слайд-шоу
menu-full-screen = Увімкнути повноекранний режим
menu-go = Перейти
menu-next-page = Наступна сторінка
menu-previous-page = Попередня сторінка
menu-go-to-page = Перейти до сторінки…
menu-bookmark = Додати закладку
menu-tools = Інструменти
menu-markup = Показати панель розмітки
menu-rotate-left = Повернути ліворуч
menu-rotate-right = Повернути праворуч
menu-crop = Обтяти
menu-adjust-color = Налаштувати колір…
menu-window = Вікно
menu-minimize = Згорнути
menu-zoom = Масштаб
menu-bring-all-to-front = Усі на передній план
