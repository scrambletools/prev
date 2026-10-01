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
language-name = Русский

## Common

common-cancel = Отменить
common-close = Закрыть
common-save = Сохранить

## Settings

settings-title = Настройки
settings-appearance = Оформление
settings-colors = Цвета
settings-windows = Окна
settings-default-app = Приложение по умолчанию
settings-default-app-label = Открывать файлы в prev
settings-default-app-note = Сделать prev приложением, которое открывает PDF, изображения, рисунки SVG и файлы Markdown.
settings-default-app-note-windows = В Windows приложения по умолчанию выбираются только в её собственных параметрах. Кнопка открывает там страницу prev.
settings-default-app-note-macos = macOS просит подтвердить каждый тип: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP и AVIF.
settings-default-app-status = Типы файлов, открываемые в prev: { $set } из { $total }.
settings-default-app-button = Сделать по умолчанию
settings-default-app-button-windows = Открыть параметры
settings-default-app-no-entry = Ярлык рабочего стола prev не установлен, поэтому система не может открывать им файлы. Установите prev из пакета или с помощью scripts/install.sh.
settings-default-app-no-bundle = Откройте prev из prev.app, чтобы сделать его приложением по умолчанию.
settings-default-app-failed = Не удалось сделать prev приложением по умолчанию: { $error }
settings-storage = Хранение
settings-version = prev { $version }
settings-version-development = prev { $version } (сборка для разработки)

## Markup toolbar

markup-tool-select = Выделение
markup-tool-area = Прямоугольное выделение
markup-tool-sketch = Набросок
markup-tool-draw = Рисование
markup-tool-shapes = Фигуры
markup-tool-text-box = Текстовое поле
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Маркер
markup-tool-note = Заметка
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Подписать
# A verb: the tool that marks areas to black out.
markup-tool-redact = Зачернить
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Применить
markup-apply-redactions = Применить зачернение
markup-shape-style = Стиль фигуры
markup-border-color = Цвет границы
markup-fill-color = Цвет заливки
markup-text-style = Стиль текста
markup-delete = Удалить
markup-undo = Отменить
markup-redo = Повторить

## Markup menus

markup-shape-rectangle = Прямоугольник
markup-shape-rounded-rectangle = Скруглённый прямоугольник
markup-shape-oval = Овал
markup-shape-line = Линия
markup-shape-arrow = Стрелка
markup-shape-star = Звезда
markup-shape-polygon = Многоугольник
markup-shape-speech-bubble = Выноска
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Лупа
# A shape that darkens the page around it.
markup-shape-mask = Маска
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Выделение цветом
markup-style-underline = Подчёркивание
markup-style-strikethrough = Зачёркивание
markup-style-squiggly = Волнистая линия
# Menu section headings.
markup-menu-color = Цвет
markup-menu-font = Шрифт
markup-menu-size = Размер
markup-menu-alignment = Выравнивание
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } пт
markup-dashed = Пунктир

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Заметка
markup-kind-text-box = Текстовое поле
markup-kind-stamp = Штамп
markup-kind-redaction = Зачернение
markup-kind-shape = Фигура
# Tooltips on a note being edited.
markup-note-delete = Удалить заметку
markup-note-done = Готово
markup-note-placeholder = Введите заметку
markup-notes-empty = Нет выделений и заметок
markup-notes-empty-hint = Здесь появятся выделения, заметки и текстовые поля.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Страница { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Не удалось изменить документ: { $error }
markup-copy-area-failed = Не удалось скопировать область: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = документ закрыт
markup-render-area-failed = не удалось отрисовать область
markup-copy-stopped = копирование прервано

## Signatures

signature-menu-empty = Подписей пока нет.
signature-delete = Удалить подпись
signature-create = Создать подпись…
signature-dialog-title = Создание подписи
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Нарисовать
signature-tab-type = Ввести
signature-tab-image = Изображение
signature-draw-hint = Распишитесь на линии мышью, пером или трекпадом.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Ваше имя
signature-image-hint = Выберите фото или скан своей подписи на белой бумаге.
signature-choose-image = Выбрать изображение…
# Placeholder of the field naming the signature in the library.
signature-description = Описание, например «Полное имя» или «Инициалы»
# Clears the drawing, typed name or image.
signature-clear = Очистить
# The color the signature is drawn or typed in.
signature-ink = Чернила
# The pen's width, for drawing.
signature-thickness = Толщина
signature-sign-first = Сначала распишитесь, затем сохраните.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Подпись { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Не удалось изменить подписи: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = нет папки данных: HOME не задан
signature-removing-stopped = удаление прервано
signature-saving-stopped = сохранение прервано
signature-reading-stopped = чтение прервано
signature-not-an-image = prev не может прочитать этот файл как изображение
signature-no-frames = в изображении нет кадров
signature-not-found = в изображении не найдена подпись

## Dragging

drag-pages-need-document = Страницы можно перетащить в документ.
drag-image-unsupported = prev не может открыть это изображение.
# $error is a lowercase reason or a technical message.
drag-area-failed = Не удалось перетащить область: { $error }
drag-pages-failed = Не удалось перетащить страницы: { $error }
drag-start-failed = Не удалось начать перетаскивание.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Страницы
drag-file-one-page = { $name } (стр. { $page })
drag-file-page-range = { $name } (стр. { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = Изображение
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = Добавить в этот документ?
drop-pdf-body = Добавить «{ $name }» в конец этого документа или открыть в отдельном окне?
drop-pdfs-body = { $count ->
    [one] Добавить эти { $count } PDF-файл в конец этого документа или открыть их в отдельных окнах?
    [few] Добавить эти { $count } PDF-файла в конец этого документа или открыть их в отдельных окнах?
    [many] Добавить эти { $count } PDF-файлов в конец этого документа или открыть их в отдельных окнах?
   *[other] Добавить эти { $count } PDF-файла в конец этого документа или открыть их в отдельных окнах?
}
drop-pdf-add = Добавить в конец
drop-pdf-open = Открыть отдельно

## PDF window

pdf-opening = Открытие…
pdf-open-failed = prev не может открыть этот документ
pdf-no-pages = В документе нет страниц.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = документ закрыт
pdf-keep-original-failed = не удалось сохранить исходную версию: { $error }
pdf-save-failed = Не удалось сохранить: { $error }
pdf-nothing-to-paste = Нечего вставить.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = вставка прервана
pdf-file-dialog-failed = Не удалось показать окно выбора файла: { $error }
pdf-bookmarks-no-home = Не удаётся сохранить закладки: HOME не задан
pdf-bookmarks-save-failed = Не удалось сохранить закладки: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Страница { $page }

# Password prompt. $name is the file name.
pdf-password-protected = Файл «{ $name }» защищён паролем
pdf-password = Пароль
pdf-password-wrong = Неверный пароль. Попробуйте ещё раз.
# Button that opens a locked document.
pdf-unlock = Разблокировать

# Toolbar tooltips and labels.
pdf-sidebar = Боковая панель
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = из { $count }
pdf-zoom-out = Уменьшить
pdf-zoom-in = Увеличить
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = По размеру страницы
pdf-fit-width = По ширине
pdf-actual-size = Фактический размер
pdf-view-continuous = Непрерывная прокрутка
pdf-view-single-page = Одна страница
pdf-view-two-pages = Две страницы
pdf-undo = Отменить
pdf-redo = Повторить
pdf-rotate-left = Повернуть влево
pdf-rotate-right = Повернуть вправо
pdf-inspector = Инспектор
pdf-markup = Разметка
# Tooltip of the button that opens the export dialog.
pdf-export = Экспорт
pdf-settings = Настройки

# Search field.
pdf-search = Поиск
pdf-search-not-found = Не найдено
pdf-searching = Поиск…
# The match shown, of all matches found.
pdf-search-match = { $current } из { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } из { $total }+

# Inspector: section headings.
pdf-inspector-file = Файл
pdf-inspector-document = Документ
pdf-inspector-pages = Страницы
# Inspector: fact labels and values.
pdf-inspector-title = Название
pdf-inspector-author = Автор
pdf-inspector-subject = Тема
pdf-inspector-keywords = Ключевые слова
pdf-inspector-created = Создан
pdf-inspector-modified = Изменён
pdf-inspector-application = Приложение
pdf-inspector-producer = Программа создания PDF
pdf-inspector-version = Версия
pdf-inspector-security = Защита
pdf-inspector-not-encrypted = Не зашифрован
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Зашифрован ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } страница
    [few] { $count } страницы
    [many] { $count } страниц
   *[other] { $count } страницы
}
pdf-inspector-page-size = Размер страницы
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } мм ({ $width_in } × { $height_in } дюйм.)
pdf-loading = Загрузка…

# Sidebar tabs and lists.
pdf-tab-pages = Страницы
pdf-tab-contents = Оглавление
pdf-tab-notes = Выделения и заметки
pdf-tab-bookmarks = Закладки
pdf-no-outline = Нет оглавления
pdf-no-outline-detail = В этом документе нет оглавления.
pdf-no-bookmarks = Нет закладок
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Нажмите { $keys }, чтобы добавить закладку.
pdf-no-bookmarks-detail-unbound = Здесь появятся страницы с закладками.
pdf-remove-bookmark = Удалить закладку

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Страницы
pages-insert-blank = Вставить пустую страницу
pages-insert-file = Вставить из файла…
pages-copy = { $count ->
    [1] Скопировать страницу
    [one] Скопировать страницы
    [few] Скопировать страницы
    [many] Скопировать страницы
   *[other] Скопировать страницы
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [1] Вставить страницу
    [one] Вставить { $count } страницу
    [few] Вставить { $count } страницы
    [many] Вставить { $count } страниц
   *[other] Вставить { $count } страницы
}
pages-crop = Обрезать по выделению
pages-select-all = Выбрать все страницы
pages-delete = { $count ->
    [1] Удалить страницу
    [one] Удалить страницы
    [few] Удалить страницы
    [many] Удалить страницы
   *[other] Удалить страницы
}
pages-apply-redactions = Применить зачернение…
pages-no-copied = Нет скопированных страниц для вставки.
pages-copied = { $count ->
    [one] Скопирована { $count } страница.
    [few] Скопированы { $count } страницы.
    [many] Скопировано { $count } страниц.
   *[other] Скопировано { $count } страницы.
}
pages-copy-failed = Не удалось скопировать страницы: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = чтение прервано
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = prev не может прочитать этот файл как изображение
pages-read-failed = Не удалось прочитать файл: { $error }
pages-at-least-one = В документе должна быть хотя бы одна страница.
pages-crop-needs-area = Сначала выберите область инструментом прямоугольного выделения.
pages-change-failed = Не удалось изменить страницы: { $error }
pages-no-redactions = Нет зачернений для применения.
pages-redactions-applied = { $count ->
    [one] Применено { $count } зачернение.
    [few] Применено { $count } зачернения.
    [many] Применено { $count } зачернений.
   *[other] Применено { $count } зачернения.
}
pages-forget-versions-failed = Не удалось удалить прежние версии: { $error }
pages-redact-title = Применить зачернение?
pages-redact-body = { $count ->
    [1] Текст, изображения и рисунки под отметкой будут навсегда удалены из документа, а отметка станет чёрным прямоугольником. Это действие нельзя отменить. Прежние версии этого файла, которые хранит prev, тоже будут удалены.
    [one] Текст, изображения и рисунки под { $count } отметкой будут навсегда удалены из документа, а отметки станут чёрными прямоугольниками. Это действие нельзя отменить. Прежние версии этого файла, которые хранит prev, тоже будут удалены.
    [few] Текст, изображения и рисунки под { $count } отметками будут навсегда удалены из документа, а отметки станут чёрными прямоугольниками. Это действие нельзя отменить. Прежние версии этого файла, которые хранит prev, тоже будут удалены.
    [many] Текст, изображения и рисунки под { $count } отметками будут навсегда удалены из документа, а отметки станут чёрными прямоугольниками. Это действие нельзя отменить. Прежние версии этого файла, которые хранит prev, тоже будут удалены.
   *[other] Текст, изображения и рисунки под { $count } отметками будут навсегда удалены из документа, а отметки станут чёрными прямоугольниками. Это действие нельзя отменить. Прежние версии этого файла, которые хранит prev, тоже будут удалены.
}
# Button that applies redactions.
pages-redact-apply = Применить

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Экспорт
pages-export-format = Формат
pages-export-reduce = Уменьшить размер файла (изображения 150 dpi)
pages-export-flatten = Свести аннотации и поля форм
pages-export-flatten-detail = Разметка и заполненные поля станут частью страниц, и изменить их будет нельзя. Неприменённые зачернения не войдут в файл.
pages-export-encrypt = Зашифровать паролем
pages-export-password = Пароль
pages-export-verify-password = Подтверждение пароля
pages-export-resolution = Разрешение
pages-export-dpi = { $dpi } dpi
pages-export-quality = Качество
# JPEG quality choices.
pages-export-quality-low = Низкое
pages-export-quality-medium = Среднее
pages-export-quality-high = Высокое
pages-export-quality-best = Наилучшее
pages-export-one-file = Все страницы попадут в один файл.
pages-export-file-per-page = Каждая страница сохраняется в отдельный файл с номером после выбранного имени.
pages-export-selected-only = { $count ->
    [1] Только выбранная страница
    [one] Только { $count } выбранная страница
    [few] Только { $count } выбранные страницы
    [many] Только { $count } выбранных страниц
   *[other] Только { $count } выбранной страницы
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Экспортировать…
pages-export-no-password = Введите пароль.
pages-export-password-mismatch = Пароли не совпадают.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (экспорт)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = документ
pages-export-same-file = Экспортируйте в новый файл: этот документ сохраняется сам.
pages-export-exporting = Экспорт «{ $name }»…
pages-export-done = Файл «{ $name }» экспортирован.
pages-export-done-images = Экспортировано изображений: { $count }.
pages-export-failed = Не удалось экспортировать: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = экспорт прерван

## Start window

# Under the app name in a window with no file open.
app-start-hint = Откройте или перетащите сюда документ PDF, изображение, рисунок SVG или файл Markdown.
app-start-open = Открыть…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (разработка)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: этот просмотрщик ещё не готов.
app-cannot-open = prev не может открывать файлы этого типа.
app-cannot-read = prev не может прочитать этот файл: { $error }
app-kind-pdf = Документ PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Изображение { $format }
app-kind-svg = Рисунок SVG
app-kind-markdown = Документ Markdown
app-file-dialog-failed = Не удалось показать окно выбора файла: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Открыть
action-settings = Настройки

## Toolbar

app-toolbar-keep-shown = Всегда показывать панель инструментов
app-toolbar-auto-hide = Скрывать панель инструментов, когда указатель вне окна
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Ещё

## File facts

# Labels in a file's inspector.
app-fact-name = Имя
app-fact-folder = Папка
app-fact-size = Размер
app-fact-modified = Изменён
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count ->
    [one] { $count } байт
    [few] { $count } байта
    [many] { $count } байт
   *[other] { $count } байта
}
app-size-kb = { $size } КБ
app-size-mb = { $size } МБ
app-size-gb = { $size } ГБ
app-size-tb = { $size } ТБ

## Links and clipboard

app-link-invalid = Недопустимая ссылка { $uri }: { $error }
app-link-open-failed = Не удалось открыть { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = установите wl-clipboard, чтобы вставлять изображения
app-copy-needs-wl-clipboard = установите wl-clipboard, чтобы копировать изображения
app-copy-no-pixels = в области нет пикселей
# wl-copy is a program's name.
app-copy-no-input = wl-copy не получил данных
app-copy-failed = сбой wl-copy
app-clipboard-open-failed = Не удалось открыть буфер обмена: { $error }
app-copy-image-failed = Не удалось скопировать изображение: { $error }

## Printing

print-failed = Не удалось напечатать: { $error }
print-stopped = Печать прервана
print-unavailable = Печать в этой системе пока недоступна.
print-no-window = Не удалось напечатать: нет окна для диалога печати
print-dialog-failed = Не удалось показать диалог печати: { $error }
# Shown after "Could not print:".
print-job-not-started = принтер не начал задание
# Shown after "Could not print:".
print-printer-stopped = принтер остановился

## File dialogs

dialog-open = Открыть
dialog-filter-all = Все поддерживаемые файлы
dialog-filter-pdf = Документы PDF
dialog-filter-images = Изображения
dialog-filter-svg = Рисунки SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Выберите папку для подписей
dialog-choose-versions = Выберите папку для истории версий
dialog-choose-bookmarks = Выберите файл закладок

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Использование: prev [FILE]...

    Просмотр и редактирование PDF и изображений. Файлы открываются в окнах
    запущенного prev; если он не запущен, он запустится.

    Параметры:
      -h, --help     Показать эту справку
      -V, --version  Показать версию

## Settings, continued

settings-language = Язык
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Как в системе: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Язык ввода
settings-input-language-system = По раскладке клавиатуры
settings-input-language-note = Определяет, с какой стороны начинается пустое текстовое поле. Введённый текст сохраняет своё направление.

settings-appearance-system = Как в системе
settings-appearance-light = Светлое
settings-appearance-dark = Тёмное
settings-omarchy-accent = Использовать акцентный цвет Omarchy
# $theme is the Omarchy theme's name.
settings-omarchy-note = Цвета строятся на основе акцентного цвета темы «{ $theme }».
settings-omarchy-none = Нет активной темы Omarchy.
settings-auto-hide = Скрывать панель инструментов, когда указатель вне окна
settings-auto-hide-note = Панель инструментов располагается поверх документа и скрывается, пока указатель находится вне окна.
settings-animations = Анимация
settings-animations-note = Выдвижные панели, раскрывающиеся окна и пружинящие кнопки.
settings-animations-reduced = Отключена, пока в системе включено уменьшение движения.
settings-corner-radius = Радиус скругления
settings-corner-radius-note = Для диалогов и плавающей панели инструментов.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Прозрачность панели
settings-overlay-note = Насколько страница просвечивает сквозь плавающую панель инструментов.
settings-overlay-value = { $percent }%
settings-storage-signatures = Папка подписей
settings-storage-versions = Папка истории версий
settings-storage-bookmarks = Файл закладок
settings-storage-apply = Применить
settings-storage-choose = Выбрать…
# $file is where the settings file is.
settings-storage-note = Файлы, уже хранящиеся в старом месте, остаются там; перенесите их, чтобы и дальше ими пользоваться. Настройки prev сохраняются в { $file }.
settings-save-failed = Не удалось сохранить настройки: { $error }
settings-no-location = Некуда сохранить настройки: HOME не задан
settings-full-path = Укажите полный путь, например ~/Documents/prev.
settings-path-is-folder = { $path } — это папка, а не файл.
settings-folder-missing = Папки { $path } нет. Сначала создайте её или выберите другую.
settings-path-is-file = { $path } — это файл, а не папка.
settings-cannot-write = prev не может записывать в { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Экспорт
# Section headings in the export dialog.
export-format = Формат
export-quality = Качество
export-size = Размер
# Button that goes on to choose where to save the export.
export-choose = Экспортировать…
# Format choice; the format name stays as it is.
export-format-webp = WebP (без потерь)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = изображение
# JPEG quality choices.
export-quality-low = Низкое
export-quality-medium = Среднее
export-quality-high = Высокое
export-quality-best = Наилучшее
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Фактический размер
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } пикс.
# $error is the system's reason.
export-dialog-failed = Не удалось показать окно сохранения: { $error }
# $path is where the file was saved.
export-done = Экспортировано: { $path }
export-failed = Не удалось экспортировать: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = экспорт прерван

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Изображения с разметкой нельзя редактировать. Экспортируйте изображение, чтобы сохранить разметку, или удалите её и закройте панель разметки.

# Shown if a background task ends unexpectedly.
image-loading-stopped = загрузка прервана
image-reverting-stopped = возврат версии прерван
image-rendering-stopped = отрисовка прервана
image-saving-stopped = сохранение прервано
image-markup-stopped = разметка прервана
image-no-version-store = Негде хранить версии
image-revert-failed = Не удалось вернуть версию: { $error }
image-read-failed = Не удалось прочитать { $path }: { $error }
image-keep-original-failed = Не удалось сохранить исходную версию: { $error }
image-save-failed = Не удалось сохранить { $path }: { $error }
image-markup-start-failed = Не удалось запустить разметку: { $error }
image-cannot-edit = Анимации и рисунки SVG нельзя редактировать.
image-cannot-mark-up = Анимации и рисунки SVG нельзя размечать.
image-mark-up-wait = Дождитесь окончания правки, затем размечайте.
image-crop-needs-selection = Сначала выделите область (инструмент «Выделение»), затем обрежьте.
image-size-needed = Введите ширину и высоту в пикселях.
# $name is a file name.
image-cannot-save-format = Изменения в файле «{ $name }» нельзя сохранить в его формате. Используйте экспорт ({ $keys }).
image-cannot-save-format-unbound = Изменения в файле «{ $name }» нельзя сохранить в его формате. Используйте экспорт.
image-cannot-export-animation = Анимации пока нельзя экспортировать.
image-drop-pages = Страницы можно перетащить в документ.
image-drag-failed = Не удалось начать перетаскивание.
image-picture-save-failed = Не удалось сохранить изображение в папку «Загрузки».
image-open-failed = prev не может открыть это изображение
image-opening = Открытие…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Имя не соответствует формату
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = Файл «{ $name }» будет сохранён в формате { $format }, но его имя оканчивается на .{ $extension }. Другие приложения могут его не открыть.
image-name-mismatch-no-extension = Файл «{ $name }» будет сохранён в формате { $format }, но у его имени нет расширения. Другие приложения могут его не открыть.
image-choose-again = Выбрать снова
image-save-as-is = Сохранить как есть
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = кадр { $current } из { $total }
image-position = { $current } из { $total }
image-edited = изменено
# Toolbar tooltips.
image-sidebar = Боковая панель
image-zoom-out = Уменьшить
image-zoom-in = Увеличить
image-zoom = { $percent }%
image-fit = По размеру окна
image-actual-size = Фактический размер
image-undo = Отменить
image-redo = Повторить
image-rotate-left = Повернуть влево
image-rotate-right = Повернуть вправо
image-flip-horizontal = Отразить по горизонтали
image-flip-vertical = Отразить по вертикали
image-select = Прямоугольное выделение
image-crop = Обрезать по выделению
image-adjust-size-tool = Настроить размер
image-adjust-color-tool = Настроить цвет
# Tooltip and panel title.
image-inspector = Инспектор
image-markup = Разметка
image-export = Экспорт
image-settings = Настройки
# Panel titles.
image-adjust-color = Настройка цвета
image-adjust-size = Настройка размера
# Adjust Color sliders.
image-exposure = Экспозиция
image-contrast = Контраст
image-saturation = Насыщенность
image-temperature = Температура
image-tint = Оттенок
image-sepia = Сепия
image-sharpness = Резкость
image-levels = Уровни
image-black-point = Точка чёрного
image-midtones = Средние тона
image-white-point = Точка белого
image-reset-all = Сбросить все
# Adjust Size panel.
image-current-size = Текущий размер: { $width } × { $height } пикс.
image-width = Ширина
image-height = Высота
image-scale-proportionally = Масштабировать пропорционально
# Button that applies the new size.
image-resize = Изменить размер
# Inspector panel.
image-inspector-loading = Загрузка…
image-file = Файл
image-format = Формат
image-dimensions-label = Размеры
image-pixels = { $width } × { $height } пикс.
image-no-camera = Нет сведений о камере.
image-location = Местоположение
image-remove-location = Удалить данные о местоположении
image-no-location = Нет сведений о местоположении.
image-keywords-description = Ключевые слова и описание
image-keywords-hint = Ключевые слова через запятую
image-description = Описание
image-keywords-unsupported = Ключевые слова можно сохранить в файлах JPEG, PNG и WebP.
# Heading over the earlier versions of the file.
image-revert-to = Вернуть к версии
image-no-versions = Прежних версий нет.
image-revert = Вернуть
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } КБ
image-size-mb = { $size } МБ
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Закрыть, не экспортировав разметку?
image-close-body = { $count ->
    [1] Разметка на изображении существует, только пока открыто его окно. Чтобы сохранить её, экспортируйте изображение: разметка будет нарисована на сохранённой копии.
    [one] Разметка на изображениях существует, только пока открыты их окна. Чтобы сохранить её, экспортируйте каждое изображение: разметка будет нарисована на сохранённой копии.
    [few] Разметка на изображениях существует, только пока открыты их окна. Чтобы сохранить её, экспортируйте каждое изображение: разметка будет нарисована на сохранённой копии.
    [many] Разметка на изображениях существует, только пока открыты их окна. Чтобы сохранить её, экспортируйте каждое изображение: разметка будет нарисована на сохранённой копии.
   *[other] Разметка на изображениях существует, только пока открыты их окна. Чтобы сохранить её, экспортируйте каждое изображение: разметка будет нарисована на сохранённой копии.
}
image-close-anyway = Всё равно закрыть

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = чтение прервано
markdown-read-failed = prev не может прочитать этот файл
markdown-draw-failed = Не удалось отрисовать документ
# Under the export's size choices.
markdown-export-size = Весь документ, { $width } × { $height } пикс.
# Search results.
markdown-not-found = Не найдено
markdown-match = { $current } из { $total }
# Placeholder of the search field.
markdown-search = Поиск
# Toolbar tooltips.
markdown-smaller-text = Уменьшить текст
markdown-larger-text = Увеличить текст
markdown-zoom = { $percent }%
markdown-actual-size = Фактический размер
# Tooltip and panel title.
markdown-inspector = Инспектор
markdown-export = Экспорт
markdown-settings = Настройки
# Inspector headings and labels.
markdown-file = Файл
markdown-document = Документ
markdown-words = Слова
markdown-lines = Строки
markdown-pictures = Рисунки

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Камера
image-meta-exposure = Экспозиция
image-meta-image = Изображение
image-meta-make = Производитель
image-meta-model = Модель
image-meta-lens = Объектив
image-meta-exposure-time = Выдержка
# The lens aperture, written like f/2.8.
image-meta-f-number = Диафрагма
image-meta-iso = ISO
image-meta-focal-length = Фокусное расстояние
image-meta-exposure-bias = Экспокоррекция
image-meta-flash = Вспышка
image-meta-date-taken = Дата съёмки
image-meta-orientation = Ориентация
image-meta-color-space = Цветовое пространство
image-meta-software = Программа
image-meta-artist = Автор
image-meta-copyright = Авторские права
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } с
image-meta-millimeters = { $value } мм
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Обычная
    [2] Отражено по горизонтали
    [3] Повёрнуто на 180°
    [4] Отражено по вертикали
    [5] Отражено по горизонтали, повёрнуто на 90° против часовой стрелки
    [6] Повёрнуто на 90° по часовой стрелке
    [7] Отражено по горизонтали, повёрнуто на 90° по часовой стрелке
    [8] Повёрнуто на 90° против часовой стрелки
   *[other] Неизвестно ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Сработала
   *[no] Не сработала
}{ $mode ->
    [on] , принудительно
    [off] , выключена
    [auto] , авто
   *[unknown] {""}
}{ $redeye ->
    [yes] , подавление красных глаз
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Без калибровки
   *[other] Другое ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = не удаётся открыть документ: { $detail }
error-pdf-page-out-of-range = страницы { $page } не существует
error-pdf-password-protected = документ защищён паролем; откройте его и скопируйте страницы оттуда
error-pdf-no-pages = нет страниц для извлечения
error-pdf-crop-outside = область обрезки находится за пределами страницы
error-pdf-closed = документ закрыт
error-pdf-saved-unreadable = сохранённый документ больше не открывается
error-image-read = не удаётся прочитать файл: { $detail }
error-image-invalid = изображение повреждено или недопустимо: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = для открытия этого формата нужна библиотека { $library }, но она не установлена
# $format is an image format name, such as HEIC.
error-image-unsupported = изображения { $format } пока не поддерживаются
error-image-encode = не удаётся закодировать изображение: { $detail }
error-exif-malformed = данные EXIF повреждены
error-settings-read = не удаётся прочитать настройки: { $detail }
error-settings-invalid = недопустимые настройки: { $detail }
error-remove-location = не удалось удалить местоположение: { $error }
error-location-unsupported = данные о местоположении можно удалить из файлов JPEG, PNG, WebP и TIFF
error-xmp-unsupported = ключевые слова и описания можно сохранить только в файлах JPEG, PNG и WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW камеры

## The macOS menu bar, named as in macOS's own apps.
menu-about = О программе prev
menu-settings = Настройки…
menu-services = Службы
menu-hide = Скрыть prev
menu-hide-others = Скрыть остальные
menu-show-all = Показать все
menu-quit = Завершить prev
menu-file = Файл
menu-open = Открыть…
menu-close = Закрыть окно
menu-export = Экспортировать…
menu-print = Напечатать…
menu-edit = Правка
menu-undo = Отменить
menu-redo = Повторить
menu-cut = Вырезать
menu-copy = Скопировать
menu-paste = Вставить
menu-select-all = Выбрать все
menu-find = Найти
menu-find-next = Найти далее
menu-find-previous = Найти ранее
menu-view = Вид
menu-hide-sidebar = Скрыть боковую панель
menu-thumbnails = Миниатюры
menu-contents = Оглавление
menu-notes = Выделения и заметки
menu-bookmarks = Закладки
menu-zoom-in = Увеличить
menu-zoom-out = Уменьшить
menu-actual-size = Фактический размер
menu-zoom-to-fit = По размеру окна
menu-inspector = Показать инспектор
menu-slideshow = Слайд-шоу
menu-full-screen = Перейти в полноэкранный режим
menu-go = Переход
menu-next-page = Следующая страница
menu-previous-page = Предыдущая страница
menu-go-to-page = Перейти к странице…
menu-bookmark = Добавить закладку
menu-tools = Инструменты
menu-markup = Показать панель разметки
menu-rotate-left = Повернуть влево
menu-rotate-right = Повернуть вправо
menu-crop = Обрезать
menu-adjust-color = Настроить цвет…
menu-window = Окно
menu-minimize = Свернуть
menu-zoom = Изменить масштаб
menu-bring-all-to-front = Все окна — на передний план
