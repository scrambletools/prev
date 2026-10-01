# prev's interface text in Persian (فارسی), translated from i18n/en/prev.ftl.
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
language-name = فارسی

## Common

common-cancel = لغو
common-close = بستن
common-save = ذخیره

## Settings

settings-title = تنظیمات
settings-appearance = ظاهر
settings-colors = رنگ‌ها
settings-windows = پنجره‌ها
settings-default-app = برنامهٔ پیش‌فرض
settings-default-app-label = باز کردن فایل‌ها با prev
settings-default-app-note = prev را برنامه‌ای کنید که PDFها، تصاویر، طرح‌های SVG و فایل‌های Markdown را باز می‌کند.
settings-default-app-note-windows = Windows انتخاب برنامه‌های پیش‌فرض را فقط در تنظیمات خودش ممکن می‌کند. این کار صفحهٔ prev را آنجا باز می‌کند.
settings-default-app-note-macos = macOS برای هر نوع تأیید می‌خواهد: PDF، PNG، JPEG، HEIC، GIF، TIFF، WebP و AVIF.
settings-default-app-status = { $set } از { $total } نوع فایل با prev باز می‌شوند.
settings-default-app-button = پیش‌فرض کردن
settings-default-app-button-windows = باز کردن تنظیمات
settings-default-app-no-entry = ورودی میزکار prev نصب نشده است، پس سیستم نمی‌تواند فایل‌ها را با آن باز کند. prev را از یک بسته یا با scripts/install.sh نصب کنید.
settings-default-app-no-bundle = برای پیش‌فرض کردن prev، آن را از prev.app باز کنید.
settings-default-app-failed = نمی‌توان prev را پیش‌فرض کرد: { $error }
settings-storage = ذخیره‌سازی
settings-version = prev { $version }
settings-version-development = prev { $version } (نسخهٔ توسعه)

## Markup toolbar

markup-tool-select = انتخاب
markup-tool-area = انتخاب مستطیلی
markup-tool-sketch = طراحی آزاد
markup-tool-draw = ترسیم
markup-tool-shapes = شکل‌ها
markup-tool-text-box = کادر متن
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = برجسته‌سازی
markup-tool-note = یادداشت
# Opens the menu of saved signatures (a verb).
markup-tool-sign = امضا
# A verb: the tool that marks areas to black out.
markup-tool-redact = پوشاندن
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = اعمال
markup-apply-redactions = اعمال پوشش‌ها
markup-shape-style = سبک شکل
markup-border-color = رنگ حاشیه
markup-fill-color = رنگ پرکردن
markup-text-style = سبک متن
markup-delete = حذف
markup-undo = واگرد
markup-redo = ازنو

## Markup menus

markup-shape-rectangle = مستطیل
markup-shape-rounded-rectangle = مستطیل گوشه‌گرد
markup-shape-oval = بیضی
markup-shape-line = خط
markup-shape-arrow = پیکان
markup-shape-star = ستاره
markup-shape-polygon = چندضلعی
markup-shape-speech-bubble = حباب گفتگو
# A shape that magnifies the part of the page under it.
markup-shape-loupe = ذره‌بین
# A shape that darkens the page around it.
markup-shape-mask = ماسک
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = برجسته‌سازی
markup-style-underline = زیرخط
markup-style-strikethrough = خط‌خورده
markup-style-squiggly = خط موج‌دار
# Menu section headings.
markup-menu-color = رنگ
markup-menu-font = قلم
markup-menu-size = اندازه
markup-menu-alignment = تراز
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } نقطه
markup-dashed = خط‌چین

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = یادداشت
markup-kind-text-box = کادر متن
markup-kind-stamp = مُهر
markup-kind-redaction = پوشش
markup-kind-shape = شکل
# Tooltips on a note being edited.
markup-note-delete = حذف یادداشت
markup-note-done = انجام شد
markup-note-placeholder = یادداشتی بنویسید
markup-notes-empty = هیچ برجسته‌سازی یا یادداشتی وجود ندارد
markup-notes-empty-hint = برجسته‌سازی‌ها، یادداشت‌ها و کادرهای متن اینجا نمایش داده می‌شوند.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = صفحهٔ { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = تغییر سند ممکن نشد: { $error }
markup-copy-area-failed = کپی ناحیه ممکن نشد: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = سند بسته شد
markup-render-area-failed = ترسیم ناحیه ممکن نشد
markup-copy-stopped = کپی متوقف شد

## Signatures

signature-menu-empty = هنوز امضایی وجود ندارد.
signature-delete = حذف امضا
signature-create = ایجاد امضا…
signature-dialog-title = ایجاد امضا
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = ترسیم
signature-tab-type = تایپ
signature-tab-image = تصویر
signature-draw-hint = با موشواره، قلم یا صفحهٔ لمسی روی خط امضا کنید.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = نام شما
signature-image-hint = عکس یا تصویر اسکن‌شده‌ای از امضای خود روی کاغذ سفید انتخاب کنید.
signature-choose-image = انتخاب تصویر…
# Placeholder of the field naming the signature in the library.
signature-description = توضیح، مانند نام کامل یا حروف اول نام
# Clears the drawing, typed name or image.
signature-clear = پاک کردن
# The color the signature is drawn or typed in.
signature-ink = جوهر
# The pen's width, for drawing.
signature-thickness = ضخامت
signature-sign-first = ابتدا امضا کنید، سپس ذخیره کنید.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = امضای { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = تغییر امضاها ممکن نشد: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = پوشهٔ داده‌ای وجود ندارد: HOME تنظیم نشده است
signature-removing-stopped = حذف متوقف شد
signature-saving-stopped = ذخیره متوقف شد
signature-reading-stopped = خواندن متوقف شد
signature-not-an-image = این پرونده تصویری نیست که prev بتواند بخواند
signature-no-frames = تصویر هیچ فریمی ندارد
signature-not-found = هیچ امضایی در تصویر پیدا نشد

## Dragging

drag-pages-need-document = صفحه‌ها را می‌توان روی یک سند رها کرد.
drag-image-unsupported = prev نمی‌تواند این تصویر را باز کند.
# $error is a lowercase reason or a technical message.
drag-area-failed = کشیدن ناحیه ممکن نشد: { $error }
drag-pages-failed = کشیدن صفحه‌ها ممکن نشد: { $error }
drag-start-failed = شروع کشیدن ممکن نشد.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = صفحه‌ها
drag-file-one-page = { $name } (صفحهٔ { $page })
drag-file-page-range = { $name } (صفحه‌های { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = تصویر
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = به این سند افزوده شود؟
drop-pdf-body = «{ $name }» به انتهای این سند افزوده شود یا در پنجرهٔ جداگانه‌ای باز شود؟
drop-pdfs-body = { $count ->
    [one] این { $count } PDF به انتهای این سند افزوده شوند یا هر کدام در پنجرهٔ جداگانه‌ای باز شوند؟
   *[other] این { $count } PDF به انتهای این سند افزوده شوند یا هر کدام در پنجرهٔ جداگانه‌ای باز شوند؟
}
drop-pdf-add = افزودن به انتها
drop-pdf-open = باز کردن جداگانه

## PDF window

pdf-opening = در حال باز کردن…
pdf-open-failed = prev نمی‌تواند این سند را باز کند
pdf-no-pages = سند هیچ صفحه‌ای ندارد.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = سند بسته شد
pdf-keep-original-failed = نگه‌داشتن نسخهٔ اصلی ممکن نشد: { $error }
pdf-save-failed = ذخیره ممکن نشد: { $error }
pdf-nothing-to-paste = چیزی برای چسباندن وجود ندارد.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = چسباندن متوقف شد
pdf-file-dialog-failed = نمایش پنجرهٔ انتخاب پرونده ممکن نشد: { $error }
pdf-bookmarks-no-home = نمی‌توان نشانک‌ها را ذخیره کرد: HOME تنظیم نشده است
pdf-bookmarks-save-failed = ذخیرهٔ نشانک‌ها ممکن نشد: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = صفحهٔ { $page }

# Password prompt. $name is the file name.
pdf-password-protected = «{ $name }» با گذرواژه محافظت شده است
pdf-password = گذرواژه
pdf-password-wrong = گذرواژه نادرست است. دوباره امتحان کنید.
# Button that opens a locked document.
pdf-unlock = باز کردن قفل

# Toolbar tooltips and labels.
pdf-sidebar = نوار کناری
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = از { $count }
pdf-zoom-out = کوچک‌نمایی
pdf-zoom-in = بزرگ‌نمایی
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = متناسب با صفحه
pdf-fit-width = متناسب با عرض
pdf-actual-size = اندازهٔ واقعی
pdf-view-continuous = پیمایش پیوسته
pdf-view-single-page = تک‌صفحه‌ای
pdf-view-two-pages = دوصفحه‌ای
pdf-undo = واگرد
pdf-redo = ازنو
pdf-rotate-left = چرخش به چپ
pdf-rotate-right = چرخش به راست
pdf-inspector = بازرس
pdf-markup = حاشیه‌نویسی
# Tooltip of the button that opens the export dialog.
pdf-export = صادر کردن
pdf-settings = تنظیمات

# Search field.
pdf-search = جستجو
pdf-search-not-found = پیدا نشد
pdf-searching = در حال جستجو…
# The match shown, of all matches found.
pdf-search-match = { $current } از { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } از { $total }+

# Inspector: section headings.
pdf-inspector-file = پرونده
pdf-inspector-document = سند
pdf-inspector-pages = صفحه‌ها
# Inspector: fact labels and values.
pdf-inspector-title = عنوان
pdf-inspector-author = نویسنده
pdf-inspector-subject = موضوع
pdf-inspector-keywords = کلیدواژه‌ها
pdf-inspector-created = تاریخ ایجاد
pdf-inspector-modified = تاریخ تغییر
pdf-inspector-application = برنامه
pdf-inspector-producer = تولیدکنندهٔ PDF
pdf-inspector-version = نسخه
pdf-inspector-security = امنیت
pdf-inspector-not-encrypted = رمزگذاری‌نشده
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = رمزگذاری‌شده ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } صفحه
   *[other] { $count } صفحه
}
pdf-inspector-page-size = اندازهٔ صفحه
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } میلی‌متر ({ $width_in } × { $height_in } اینچ)
pdf-loading = در حال بارگذاری…

# Sidebar tabs and lists.
pdf-tab-pages = صفحه‌ها
pdf-tab-contents = فهرست مطالب
pdf-tab-notes = برجسته‌سازی‌ها و یادداشت‌ها
pdf-tab-bookmarks = نشانک‌ها
pdf-no-outline = فهرست مطالبی وجود ندارد
pdf-no-outline-detail = این سند طرح کلی ندارد.
pdf-no-bookmarks = نشانکی وجود ندارد
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = برای نشانک‌گذاری یک صفحه، { $keys } را فشار دهید.
pdf-no-bookmarks-detail-unbound = صفحه‌های نشانک‌شده اینجا نشان داده می‌شوند.
pdf-remove-bookmark = حذف نشانک

## Page editing

# Tooltip of the Pages menu button.
pages-menu = صفحه‌ها
pages-insert-blank = درج صفحهٔ خالی
pages-insert-file = درج از پرونده…
pages-copy = { $count ->
    [one] کپی صفحه
   *[other] کپی صفحه‌ها
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] چسباندن صفحه
   *[other] چسباندن { $count } صفحه
}
pages-crop = برش به ناحیهٔ انتخاب‌شده
pages-select-all = انتخاب همهٔ صفحه‌ها
pages-delete = { $count ->
    [one] حذف صفحه
   *[other] حذف صفحه‌ها
}
pages-apply-redactions = اعمال پوشش‌ها…
pages-no-copied = هیچ صفحهٔ کپی‌شده‌ای برای چسباندن وجود ندارد.
pages-copied = { $count ->
    [one] { $count } صفحه کپی شد.
   *[other] { $count } صفحه کپی شد.
}
pages-copy-failed = کپی صفحه‌ها ممکن نشد: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = خواندن متوقف شد
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = تصویری نیست که prev بتواند بخواند
pages-read-failed = خواندن پرونده ممکن نشد: { $error }
pages-at-least-one = هر سند باید دست‌کم یک صفحه داشته باشد.
pages-crop-needs-area = ابتدا با ابزار انتخاب مستطیلی یک ناحیه انتخاب کنید.
pages-change-failed = تغییر صفحه‌ها ممکن نشد: { $error }
pages-no-redactions = پوششی برای اعمال وجود نداشت.
pages-redactions-applied = { $count ->
    [one] { $count } پوشش اعمال شد.
   *[other] { $count } پوشش اعمال شد.
}
pages-forget-versions-failed = حذف نسخه‌های قبلی ممکن نشد: { $error }
pages-redact-title = پوشش‌ها اعمال شوند؟
pages-redact-body = { $count ->
    [one] متن، تصویرها و نقاشی‌های زیر علامت برای همیشه از سند حذف می‌شوند و علامت به کادری سیاه تبدیل می‌شود. این کار برگشت‌پذیر نیست و نسخه‌های قبلی این پرونده که prev نگه می‌دارد حذف می‌شوند.
   *[other] متن، تصویرها و نقاشی‌های زیر { $count } علامت برای همیشه از سند حذف می‌شوند و علامت‌ها به کادرهای سیاه تبدیل می‌شوند. این کار برگشت‌پذیر نیست و نسخه‌های قبلی این پرونده که prev نگه می‌دارد حذف می‌شوند.
}
# Button that applies redactions.
pages-redact-apply = اعمال

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = صادر کردن
pages-export-format = قالب
pages-export-reduce = کاهش حجم پرونده (تصویرها با 150 dpi)
pages-export-flatten = یکپارچه‌سازی حاشیه‌نویسی‌ها و فیلدهای فرم با صفحه
pages-export-flatten-detail = حاشیه‌نویسی‌ها و فیلدهای پرشده بخشی از صفحه‌ها می‌شوند و دیگر ویرایش‌پذیر نیستند. علامت‌های پوششی که هنوز اعمال نشده‌اند کنار گذاشته می‌شوند.
pages-export-encrypt = رمزگذاری با گذرواژه
pages-export-password = گذرواژه
pages-export-verify-password = تأیید گذرواژه
pages-export-resolution = وضوح
pages-export-dpi = { $dpi } dpi
pages-export-quality = کیفیت
# JPEG quality choices.
pages-export-quality-low = پایین
pages-export-quality-medium = متوسط
pages-export-quality-high = بالا
pages-export-quality-best = بهترین
pages-export-one-file = همهٔ صفحه‌ها در یک پرونده قرار می‌گیرند.
pages-export-file-per-page = هر صفحه در پرونده‌ای جداگانه ذخیره می‌شود که با نامی که انتخاب می‌کنید و یک شماره نام‌گذاری می‌شود.
pages-export-selected-only = { $count ->
    [one] فقط صفحهٔ انتخاب‌شده
   *[other] فقط { $count } صفحهٔ انتخاب‌شده
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = صادر کردن…
pages-export-no-password = گذرواژه‌ای وارد کنید.
pages-export-password-mismatch = گذرواژه‌ها یکسان نیستند.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (صادرشده)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = سند
pages-export-same-file = به پرونده‌ای جدید صادر کنید؛ این سند به‌طور خودکار ذخیره می‌شود.
pages-export-exporting = در حال صادر کردن «{ $name }»…
pages-export-done = «{ $name }» صادر شد.
pages-export-done-images = { $count } تصویر صادر شد.
pages-export-failed = صادر کردن ممکن نشد: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = صادر کردن متوقف شد

## Start window

# Under the app name in a window with no file open.
app-start-hint = یک پروندهٔ PDF، تصویر، SVG یا Markdown را باز کنید یا اینجا رها کنید.
app-start-open = باز کردن…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (نسخهٔ توسعه)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: این نمایشگر هنوز ساخته نشده است.
app-cannot-open = prev نمی‌تواند این نوع پرونده را باز کند.
app-cannot-read = prev نمی‌تواند این پرونده را بخواند: { $error }
app-kind-pdf = سند PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = تصویر { $format }
app-kind-svg = نقاشی SVG
app-kind-markdown = سند Markdown
app-file-dialog-failed = نمایش پنجرهٔ انتخاب پرونده ممکن نشد: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = باز کردن
action-settings = تنظیمات

## Toolbar

app-toolbar-keep-shown = نمایش همیشگی نوار ابزار
app-toolbar-auto-hide = پنهان کردن نوار ابزار هنگام خروج نشانگر
# The button that shows the toolbar's hidden tools.
app-toolbar-more = بیشتر

## File facts

# Labels in a file's inspector.
app-fact-name = نام
app-fact-folder = پوشه
app-fact-size = اندازه
app-fact-modified = تاریخ تغییر
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } بایت
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = پیوند نامعتبر { $uri }: { $error }
app-link-open-failed = باز کردن { $uri } ممکن نشد: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = برای چسباندن تصویرها wl-clipboard را نصب کنید
app-copy-needs-wl-clipboard = برای کپی تصویرها wl-clipboard را نصب کنید
app-copy-no-pixels = ناحیه هیچ پیکسلی ندارد
# wl-copy is a program's name.
app-copy-no-input = wl-copy ورودی ندارد
app-copy-failed = wl-copy ناموفق بود
app-clipboard-open-failed = باز کردن کلیپ‌بورد ممکن نشد: { $error }
app-copy-image-failed = کپی تصویر ممکن نشد: { $error }

## Printing

print-failed = چاپ ممکن نشد: { $error }
print-stopped = چاپ متوقف شد
print-unavailable = چاپ هنوز در این سیستم در دسترس نیست.
print-no-window = چاپ ممکن نشد: پنجره‌ای برای نمایش پنجرهٔ گفتگوی چاپ روی آن وجود ندارد
print-dialog-failed = نمایش پنجرهٔ گفتگوی چاپ ممکن نشد: { $error }
# Shown after "Could not print:".
print-job-not-started = چاپگر کار را شروع نکرد
# Shown after "Could not print:".
print-printer-stopped = چاپگر متوقف شد

## File dialogs

dialog-open = باز کردن
dialog-filter-all = همهٔ پرونده‌های پشتیبانی‌شده
dialog-filter-pdf = سندهای PDF
dialog-filter-images = تصویرها
dialog-filter-svg = نقاشی‌های SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = پوشهٔ امضاها را انتخاب کنید
dialog-choose-versions = پوشهٔ تاریخچهٔ نسخه‌ها را انتخاب کنید
dialog-choose-bookmarks = پروندهٔ نشانک‌ها را انتخاب کنید

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    کاربرد: prev [FILE]...

    مشاهده و ویرایش PDFها و تصویرها. پرونده‌ها در پنجره‌های prev در حال اجرا
    باز می‌شوند و اگر prev در حال اجرا نباشد، اجرا می‌شود.

    گزینه‌ها:
      -h, --help     نمایش این راهنما
      -V, --version  نمایش نسخه

## Settings, continued

settings-language = زبان
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = پیش‌فرض سیستم: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = زبان ورودی
settings-input-language-system = پیروی از چیدمان صفحه‌کلید
settings-input-language-note = تعیین می‌کند کادر متن خالی از کدام سمت شروع شود. متنی که تایپ می‌کنید جهت خودش را حفظ می‌کند.

settings-appearance-system = سیستم
settings-appearance-light = روشن
settings-appearance-dark = تیره
settings-system-accent = استفاده از رنگ تأکیدی سیستم
# $theme is the Omarchy theme's name.
settings-omarchy-note = رنگ‌ها از رنگ تأکیدی پوستهٔ «{ $theme }» ساخته می‌شوند.
settings-system-accent-note = رنگ‌ها از رنگ تأکیدی سیستم ساخته می‌شوند.
settings-system-accent-none = سیستم رنگ تأکیدی ندارد، پس prev از رنگ خودش استفاده می‌کند.
settings-auto-hide = پنهان کردن نوار ابزار هنگام خروج نشانگر
settings-auto-hide-note = نوار ابزار روی سند شناور است و وقتی نشانگر بیرون از پنجره باشد، کنار می‌رود.
settings-animations = پویانمایی‌ها
settings-animations-note = نوارها و پنل‌های لغزان، پنجره‌های گفتگوی بازشونده و دکمه‌های فنری.
settings-animations-reduced = وقتی سیستم کاهش حرکت را درخواست کند، خاموش است.
settings-corner-radius = شعاع گوشه‌ها
settings-corner-radius-note = برای پنجره‌های گفتگو و نوار ابزار شناور.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } پیکسل
settings-overlay = شفافیت روکش
settings-overlay-note = چه مقدار از صفحه از پشت نوار ابزار شناور دیده شود.
settings-overlay-value = { $percent }%
settings-storage-signatures = پوشهٔ امضاها
settings-storage-versions = پوشهٔ تاریخچهٔ نسخه‌ها
settings-storage-bookmarks = پروندهٔ نشانک‌ها
settings-storage-apply = اعمال
settings-storage-choose = انتخاب…
# $file is where the settings file is.
settings-storage-note = پرونده‌هایی که پیش‌تر در مکان قدیمی نگه داشته شده‌اند همان‌جا می‌مانند؛ برای ادامهٔ استفاده، آن‌ها را منتقل کنید. تنظیمات برنامهٔ prev در { $file } ذخیره می‌شوند.
settings-save-failed = ذخیرهٔ تنظیمات ممکن نشد: { $error }
settings-no-location = مکانی برای تنظیمات وجود ندارد: HOME تنظیم نشده است
settings-full-path = از یک مسیر کامل استفاده کنید، مانند ~/Documents/prev.
settings-path-is-folder = { $path } یک پوشه است، نه پرونده.
settings-folder-missing = پوشه‌ای به نام { $path } وجود ندارد. ابتدا آن را بسازید یا پوشهٔ دیگری انتخاب کنید.
settings-path-is-file = { $path } یک پرونده است، نه پوشه.
settings-cannot-write = prev نمی‌تواند در { $path } بنویسد: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = صادر کردن
# Section headings in the export dialog.
export-format = قالب
export-quality = کیفیت
export-size = اندازه
# Button that goes on to choose where to save the export.
export-choose = صادر کردن…
# Format choice; the format name stays as it is.
export-format-webp = WebP (بدون افت کیفیت)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = تصویر
# JPEG quality choices.
export-quality-low = پایین
export-quality-medium = متوسط
export-quality-high = بالا
export-quality-best = بهترین
# Size choices: the picture at its own size, or scaled up.
export-size-actual = اندازهٔ واقعی
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } پیکسل
# $error is the system's reason.
export-dialog-failed = نمایش پنجرهٔ ذخیره ممکن نشد: { $error }
# $path is where the file was saved.
export-done = { $path } صادر شد
export-failed = صادر کردن ممکن نشد: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = صادر کردن متوقف شد

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = تصویرهای دارای حاشیه‌نویسی را نمی‌توان ویرایش کرد. برای نگه‌داشتن حاشیه‌نویسی، تصویر را صادر کنید، یا حاشیه‌نویسی را حذف کنید و نوار حاشیه‌نویسی را ببندید.

# Shown if a background task ends unexpectedly.
image-loading-stopped = بارگذاری متوقف شد
image-reverting-stopped = بازگردانی متوقف شد
image-rendering-stopped = ترسیم متوقف شد
image-saving-stopped = ذخیره متوقف شد
image-markup-stopped = حاشیه‌نویسی متوقف شد
image-no-version-store = جایی برای نگه‌داشتن نسخه‌ها وجود ندارد
image-revert-failed = بازگردانی ممکن نشد: { $error }
image-read-failed = خواندن { $path } ممکن نشد: { $error }
image-keep-original-failed = نگه‌داشتن نسخهٔ اصلی ممکن نشد: { $error }
image-save-failed = ذخیرهٔ { $path } ممکن نشد: { $error }
image-markup-start-failed = شروع حاشیه‌نویسی ممکن نشد: { $error }
image-cannot-edit = پویانمایی‌ها و نقاشی‌های SVG را نمی‌توان ویرایش کرد.
image-cannot-mark-up = روی پویانمایی‌ها و نقاشی‌های SVG نمی‌توان حاشیه‌نویسی کرد.
image-mark-up-wait = صبر کنید ویرایش تمام شود، سپس حاشیه‌نویسی کنید.
image-crop-needs-selection = ابتدا با کشیدن، ناحیه‌ای را انتخاب کنید (ابزار انتخاب)، سپس برش دهید.
image-size-needed = عرض و ارتفاع را به پیکسل وارد کنید.
# $name is a file name.
image-cannot-save-format = تغییرات «{ $name }» را نمی‌توان در قالب خودش ذخیره کرد. از صادر کردن ({ $keys }) استفاده کنید.
image-cannot-save-format-unbound = تغییرات «{ $name }» را نمی‌توان در قالب خودش ذخیره کرد. از صادر کردن استفاده کنید.
image-cannot-export-animation = پویانمایی‌ها هنوز صادر نمی‌شوند.
image-drop-pages = صفحه‌ها را می‌توان روی یک سند رها کرد.
image-drag-failed = شروع کشیدن ممکن نشد.
image-picture-save-failed = ذخیرهٔ تصویر در پوشهٔ بارگیری‌ها ممکن نشد.
image-open-failed = prev نمی‌تواند این تصویر را باز کند
image-opening = در حال باز کردن…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = نام با قالب مطابقت ندارد
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = «{ $name }» به‌صورت پروندهٔ { $format } ذخیره می‌شود، اما پسوند نامش .{ $extension } است. ممکن است برنامه‌های دیگر آن را باز نکنند.
image-name-mismatch-no-extension = «{ $name }» به‌صورت پروندهٔ { $format } ذخیره می‌شود، اما نامش پسوند ندارد. ممکن است برنامه‌های دیگر آن را باز نکنند.
image-choose-again = انتخاب دوباره
image-save-as-is = ذخیره به همین شکل
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = فریم { $current } از { $total }
image-position = { $current } از { $total }
image-edited = ویرایش‌شده
# Toolbar tooltips.
image-sidebar = نوار کناری
image-zoom-out = کوچک‌نمایی
image-zoom-in = بزرگ‌نمایی
image-zoom = { $percent }%
image-fit = متناسب با پنجره
image-actual-size = اندازهٔ واقعی
image-undo = واگرد
image-redo = ازنو
image-rotate-left = چرخش به چپ
image-rotate-right = چرخش به راست
image-flip-horizontal = قرینهٔ افقی
image-flip-vertical = قرینهٔ عمودی
image-select = انتخاب مستطیلی
image-crop = برش به ناحیهٔ انتخاب‌شده
image-adjust-size-tool = تنظیم اندازه
image-adjust-color-tool = تنظیم رنگ
# Tooltip and panel title.
image-inspector = بازرس
image-markup = حاشیه‌نویسی
image-export = صادر کردن
image-settings = تنظیمات
# Panel titles.
image-adjust-color = تنظیم رنگ
image-adjust-size = تنظیم اندازه
# Adjust Color sliders.
image-exposure = نوردهی
image-contrast = کنتراست
image-saturation = اشباع
image-temperature = دما
image-tint = ته‌رنگ
image-sepia = سپیا
image-sharpness = تیزی
image-levels = سطوح
image-black-point = نقطهٔ سیاه
image-midtones = تُن‌های میانی
image-white-point = نقطهٔ سفید
image-reset-all = بازنشانی همه
# Adjust Size panel.
image-current-size = اندازهٔ فعلی: { $width } × { $height } پیکسل
image-width = عرض
image-height = ارتفاع
image-scale-proportionally = تغییر مقیاس متناسب
# Button that applies the new size.
image-resize = تغییر اندازه
# Inspector panel.
image-inspector-loading = در حال بارگذاری…
image-file = پرونده
image-format = قالب
image-dimensions-label = ابعاد
image-pixels = { $width } × { $height } پیکسل
image-no-camera = اطلاعاتی از دوربین وجود ندارد.
image-location = مکان
image-remove-location = حذف اطلاعات مکان
image-no-location = اطلاعات مکان وجود ندارد.
image-keywords-description = کلیدواژه‌ها و توضیح
image-keywords-hint = کلیدواژه‌ها، جداشده با ویرگول
image-description = توضیح
image-keywords-unsupported = کلیدواژه‌ها را می‌توان در پرونده‌های JPEG، PNG و WebP ذخیره کرد.
# Heading over the earlier versions of the file.
image-revert-to = بازگشت به
image-no-versions = نسخهٔ قبلی‌ای وجود ندارد.
image-revert = بازگرداندن
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = بدون صادر کردن حاشیه‌نویسی بسته شود؟
image-close-body = { $count ->
    [one] حاشیه‌نویسی روی تصویر فقط تا وقتی پنجره‌اش باز است باقی می‌ماند. برای نگه‌داشتن آن، تصویر را صادر کنید: حاشیه‌نویسی در نسخه‌ای که ذخیره می‌کنید ترسیم می‌شود.
   *[other] حاشیه‌نویسی روی تصویرها فقط تا وقتی پنجره‌شان باز است باقی می‌ماند. برای نگه‌داشتن آن، هر تصویر را صادر کنید: حاشیه‌نویسی در نسخه‌ای که ذخیره می‌کنید ترسیم می‌شود.
}
image-close-anyway = در هر حال بسته شود

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = خواندن متوقف شد
markdown-read-failed = prev نمی‌تواند این پرونده را بخواند
markdown-draw-failed = ترسیم سند ممکن نشد
# Under the export's size choices.
markdown-export-size = کل سند، { $width } × { $height } پیکسل
# Search results.
markdown-not-found = پیدا نشد
markdown-match = { $current } از { $total }
# Placeholder of the search field.
markdown-search = جستجو
# Toolbar tooltips.
markdown-smaller-text = متن کوچک‌تر
markdown-larger-text = متن بزرگ‌تر
markdown-zoom = { $percent }%
markdown-actual-size = اندازهٔ واقعی
# Tooltip and panel title.
markdown-inspector = بازرس
markdown-export = صادر کردن
markdown-settings = تنظیمات
# Inspector headings and labels.
markdown-file = پرونده
markdown-document = سند
markdown-words = واژه‌ها
markdown-lines = خط‌ها
markdown-pictures = تصویرها

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = دوربین
image-meta-exposure = نوردهی
image-meta-image = تصویر
image-meta-make = سازنده
image-meta-model = مدل
image-meta-lens = لنز
image-meta-exposure-time = زمان نوردهی
# The lens aperture, written like f/2.8.
image-meta-f-number = عدد F
image-meta-iso = ISO
image-meta-focal-length = فاصلهٔ کانونی
image-meta-exposure-bias = جبران نوردهی
image-meta-flash = فلاش
image-meta-date-taken = تاریخ عکاسی
image-meta-orientation = جهت
image-meta-color-space = فضای رنگ
image-meta-software = نرم‌افزار
image-meta-artist = هنرمند
image-meta-copyright = حق نشر
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } ثانیه
image-meta-millimeters = { $value } میلی‌متر
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] عادی
    [2] قرینهٔ افقی
    [3] چرخش 180°
    [4] قرینهٔ عمودی
    [5] قرینهٔ افقی، چرخش 90° پادساعتگرد
    [6] چرخش 90° ساعتگرد
    [7] قرینهٔ افقی، چرخش 90° ساعتگرد
    [8] چرخش 90° پادساعتگرد
   *[other] نامشخص ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] زده شد
   *[no] زده نشد
}{ $mode ->
    [on] ، اجباری
    [off] ، خاموش
    [auto] ، خودکار
   *[unknown] {""}
}{ $redeye ->
    [yes] ، کاهش قرمزی چشم
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] کالیبره‌نشده
   *[other] دیگر ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = باز کردن سند ممکن نیست: { $detail }
error-pdf-page-out-of-range = صفحهٔ { $page } وجود ندارد
error-pdf-password-protected = سند با گذرواژه محافظت شده است؛ به‌جای آن، سند را باز کنید و صفحه‌هایش را کپی کنید
error-pdf-no-pages = صفحه‌ای برای استخراج وجود ندارد
error-pdf-crop-outside = ناحیهٔ برش بیرون از صفحه است
error-pdf-closed = سند بسته شد
error-pdf-saved-unreadable = سند ذخیره‌شده دیگر باز نمی‌شود
error-image-read = خواندن پرونده ممکن نیست: { $detail }
error-image-invalid = تصویر آسیب‌دیده یا نامعتبر است: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = باز کردن این قالب به { $library } نیاز دارد که نصب نشده است
# $format is an image format name, such as HEIC.
error-image-unsupported = تصویرهای { $format } هنوز پشتیبانی نمی‌شوند
error-image-encode = کدگذاری تصویر ممکن نیست: { $detail }
error-exif-malformed = ساختار داده‌های EXIF نادرست است
error-settings-read = خواندن تنظیمات ممکن نیست: { $detail }
error-settings-invalid = تنظیمات نامعتبر: { $detail }
error-remove-location = حذف مکان ممکن نشد: { $error }
error-location-unsupported = اطلاعات مکان را می‌توان از پرونده‌های JPEG، PNG، WebP و TIFF حذف کرد
error-xmp-unsupported = کلیدواژه‌ها و توضیحات را فقط می‌توان در پرونده‌های JPEG، PNG و WebP ذخیره کرد

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW دوربین

## The macOS menu bar, named as in macOS's own apps.
menu-about = دربارهٔ prev
menu-settings = تنظیمات…
menu-services = خدمات
menu-hide = پنهان کردن prev
menu-hide-others = پنهان کردن بقیه
menu-show-all = نمایش همه
menu-quit = خروج از prev
menu-file = پرونده
menu-open = باز کردن…
menu-close = بستن پنجره
menu-export = صادر کردن…
menu-print = چاپ…
menu-edit = ویرایش
menu-undo = واگرد
menu-redo = ازنو
menu-cut = بریدن
menu-copy = کپی
menu-paste = چسباندن
menu-select-all = انتخاب همه
menu-find = یافتن
menu-find-next = یافتن بعدی
menu-find-previous = یافتن قبلی
menu-view = نما
menu-hide-sidebar = پنهان کردن نوار کناری
menu-thumbnails = تصویرهای بندانگشتی
menu-contents = فهرست مطالب
menu-notes = برجسته‌سازی‌ها و یادداشت‌ها
menu-bookmarks = نشانک‌ها
menu-zoom-in = بزرگ‌نمایی
menu-zoom-out = کوچک‌نمایی
menu-actual-size = اندازهٔ واقعی
menu-zoom-to-fit = بزرگ‌نمایی متناسب
menu-inspector = نمایش بازرس
menu-slideshow = نمایش اسلاید
menu-full-screen = ورود به حالت تمام‌صفحه
menu-go = رفتن
menu-next-page = صفحهٔ بعد
menu-previous-page = صفحهٔ قبل
menu-go-to-page = رفتن به صفحه…
menu-bookmark = افزودن نشانک
menu-tools = ابزارها
menu-markup = نمایش نوار ابزار حاشیه‌نویسی
menu-rotate-left = چرخش به چپ
menu-rotate-right = چرخش به راست
menu-crop = برش
menu-adjust-color = تنظیم رنگ…
menu-window = پنجره
menu-minimize = کمینه کردن
menu-zoom = بزرگ/کوچک کردن
menu-bring-all-to-front = آوردن همه به جلو
