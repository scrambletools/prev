# prev's interface text in Urdu (اردو), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = مارک اپ, note = نوٹ, highlight = نمایاں کاری
# (verb نمایاں کریں), annotation = تشریح, redact/redaction = پوشیدہ کریں/پوشیدگی,
# inspector = انسپکٹر, zoom in/out = زوم ان/زوم آؤٹ, bookmark = بک مارک,
# page = صفحہ (plural صفحات), export = برآمد کریں, revert = بحال کریں,
# undo/redo = کالعدم کریں/دوبارہ کریں. Buttons and menu items use the polite
# imperative (محفوظ کریں، منسوخ کریں); labels use nouns.

## Language

language-name = اردو

## Common

common-cancel = منسوخ کریں
common-close = بند کریں
common-save = محفوظ کریں

## Settings

settings-title = ترتیبات
settings-appearance = ظاہری شکل
settings-colors = رنگ
settings-windows = ونڈوز
settings-default-app = ڈیفالٹ ایپ
settings-default-app-label = فائلیں prev سے کھولیں
settings-default-app-note = prev کو وہ ایپ بنائیں جو PDF، تصاویر، SVG ڈرائنگز اور Markdown فائلیں کھولتی ہے۔
settings-default-app-note-windows = Windows ڈیفالٹ ایپس صرف اپنی ترتیبات میں منتخب کرنے دیتا ہے۔ یہ بٹن وہاں prev کا صفحہ کھولتا ہے۔
settings-default-app-note-macos = macOS ہر قسم کی الگ تصدیق مانگتا ہے: PDF، PNG، JPEG، HEIC، GIF، TIFF، WebP اور AVIF۔
settings-default-app-status = { $total } میں سے { $set } اقسام کی فائلیں prev سے کھلتی ہیں۔
settings-default-app-button = ڈیفالٹ بنائیں
settings-default-app-button-windows = ترتیبات کھولیں
settings-default-app-no-entry = prev کا ڈیسک ٹاپ اندراج انسٹال نہیں ہے، اس لیے سسٹم اس سے فائلیں نہیں کھول سکتا۔ prev کو کسی پیکیج سے یا scripts/install.sh سے انسٹال کریں۔
settings-default-app-no-bundle = prev کو ڈیفالٹ بنانے کے لیے اسے prev.app سے کھولیں۔
settings-default-app-failed = prev کو ڈیفالٹ نہیں بنایا جا سکا: { $error }
settings-storage = اسٹوریج
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (ڈیولپمنٹ بلڈ، { $build })

## Markup toolbar

markup-tool-select = منتخب کریں
markup-tool-area = مستطیل انتخاب
markup-tool-sketch = خاکہ
markup-tool-draw = ڈرا کریں
markup-tool-shapes = اشکال
markup-tool-text-box = ٹیکسٹ باکس
markup-tool-highlight = نمایاں کریں
markup-tool-note = نوٹ
markup-tool-sign = دستخط کریں
markup-tool-redact = پوشیدہ کریں
markup-apply = لاگو کریں
markup-apply-redactions = پوشیدگیاں لاگو کریں
markup-shape-style = شکل کا انداز
markup-border-color = بارڈر کا رنگ
markup-fill-color = بھرائی کا رنگ
markup-text-style = متن کا انداز
markup-delete = حذف کریں
markup-undo = کالعدم کریں
markup-redo = دوبارہ کریں

## Markup menus

markup-shape-rectangle = مستطیل
markup-shape-rounded-rectangle = گول کونوں والا مستطیل
markup-shape-oval = بیضوی
markup-shape-line = لکیر
markup-shape-arrow = تیر
markup-shape-star = ستارہ
markup-shape-polygon = کثیر الاضلاع
markup-shape-speech-bubble = گفتگو کا غبارہ
markup-shape-loupe = عدسہ
markup-shape-mask = ماسک
markup-style-highlight = نمایاں کاری
markup-style-underline = زیر خط
markup-style-strikethrough = قلم زد
markup-style-squiggly = لہردار لکیر
markup-menu-color = رنگ
markup-menu-font = فونٹ
markup-menu-size = سائز
markup-menu-alignment = صف بندی
markup-line-width = { $width } پوائنٹ
markup-dashed = ڈیش دار

## Notes

markup-kind-note = نوٹ
markup-kind-text-box = ٹیکسٹ باکس
markup-kind-stamp = مہر
markup-kind-redaction = پوشیدگی
markup-kind-shape = شکل
markup-note-delete = نوٹ حذف کریں
markup-note-done = ہو گیا
markup-note-placeholder = نوٹ لکھیں
markup-notes-empty = کوئی نمایاں کاری یا نوٹ نہیں
markup-notes-empty-hint = نمایاں کاریاں، نوٹس اور ٹیکسٹ باکسز یہاں ظاہر ہوتے ہیں۔
markup-notes-page = صفحہ { $page }

## Markup errors

markup-change-failed = دستاویز تبدیل نہیں کی جا سکی: { $error }
markup-copy-area-failed = حصہ کاپی نہیں کیا جا سکا: { $error }
markup-document-closed = دستاویز بند ہو گئی
markup-render-area-failed = حصہ رینڈر نہیں کیا جا سکا
markup-copy-stopped = کاپی کرنا رک گیا

## Signatures

signature-menu-empty = ابھی کوئی دستخط نہیں۔
signature-delete = دستخط حذف کریں
signature-create = دستخط بنائیں…
signature-dialog-title = دستخط بنائیں
signature-tab-draw = ڈرا کریں
signature-tab-type = ٹائپ کریں
signature-tab-image = تصویر
signature-draw-hint = لکیر پر اپنے ماؤس، قلم یا ٹچ پیڈ سے دستخط کریں۔
signature-your-name = آپ کا نام
signature-image-hint = سفید کاغذ پر اپنے دستخط کی تصویر یا اسکین منتخب کریں۔
signature-choose-image = تصویر منتخب کریں…
signature-description = تفصیل، جیسے پورا نام یا نام کے ابتدائی حروف
signature-clear = صاف کریں
signature-ink = روشنائی
signature-thickness = موٹائی
signature-sign-first = پہلے دستخط کریں، پھر محفوظ کریں۔
signature-default-name = دستخط { $number }
signature-change-failed = دستخط تبدیل نہیں کیے جا سکے: { $error }
signature-no-data-folder = کوئی ڈیٹا فولڈر نہیں: HOME سیٹ نہیں ہے
signature-removing-stopped = ہٹانا رک گیا
signature-saving-stopped = محفوظ کرنا رک گیا
signature-reading-stopped = پڑھنا رک گیا
signature-not-an-image = یہ فائل ایسی تصویر نہیں جسے prev پڑھ سکے
signature-no-frames = تصویر میں کوئی فریم نہیں
signature-not-found = تصویر میں کوئی دستخط نہیں ملا

## Dragging

drag-pages-need-document = صفحات کسی دستاویز پر ڈراپ کیے جا سکتے ہیں۔
drag-image-unsupported = prev یہ تصویر نہیں کھول سکتا۔
drag-area-failed = حصہ ڈریگ نہیں کیا جا سکا: { $error }
drag-pages-failed = صفحات ڈریگ نہیں کیے جا سکے: { $error }
drag-start-failed = ڈریگ کرنا شروع نہیں ہو سکا۔
drag-file-pages = صفحات
drag-file-one-page = { $name } (صفحہ { $page })
drag-file-page-range = { $name } (صفحات { $first }–{ $last })
drag-file-image = تصویر
drop-pdf-title = اس دستاویز میں شامل کریں؟
drop-pdf-body = “{ $name }” کو اس دستاویز کے آخر میں شامل کریں، یا اسے الگ ونڈو میں کھولیں؟
drop-pdfs-body = { $count ->
    [one] اس PDF فائل کو اس دستاویز کے آخر میں شامل کریں، یا اسے الگ ونڈو میں کھولیں؟
   *[other] ان { $count } PDF فائلوں کو اس دستاویز کے آخر میں شامل کریں، یا انہیں الگ الگ ونڈوز میں کھولیں؟
}
drop-pdf-add = آخر میں شامل کریں
drop-pdf-open = الگ سے کھولیں

## PDF window

pdf-opening = کھل رہا ہے…
pdf-open-failed = prev یہ دستاویز نہیں کھول سکتا
pdf-no-pages = دستاویز میں کوئی صفحہ نہیں ہے۔
pdf-document-closed = دستاویز بند ہو گئی
pdf-keep-original-failed = اصل ورژن نہیں رکھا جا سکا: { $error }
pdf-save-failed = محفوظ نہیں کیا جا سکا: { $error }
pdf-nothing-to-paste = چسپاں کرنے کے لیے کچھ نہیں ہے۔
pdf-pasting-stopped = چسپاں کرنا رک گیا
pdf-file-dialog-failed = فائل ڈائیلاگ نہیں دکھایا جا سکا: { $error }
pdf-bookmarks-no-home = بک مارکس محفوظ نہیں کیے جا سکتے: HOME سیٹ نہیں ہے
pdf-bookmarks-save-failed = بک مارکس محفوظ نہیں کیے جا سکے: { $error }
pdf-bookmark-page = صفحہ { $page }

pdf-password-protected = “{ $name }” پاس ورڈ سے محفوظ ہے
pdf-password = پاس ورڈ
pdf-password-wrong = غلط پاس ورڈ۔ دوبارہ کوشش کریں۔
pdf-unlock = غیر مقفل کریں

pdf-sidebar = سائڈ بار
pdf-page-of = / { $count }
pdf-zoom-out = زوم آؤٹ
pdf-zoom-in = زوم ان
pdf-zoom-percent = { $percent }%
pdf-fit-page = صفحہ فٹ کریں
pdf-fit-width = چوڑائی فٹ کریں
pdf-actual-size = اصل سائز
pdf-view-continuous = مسلسل اسکرول
pdf-view-single-page = واحد صفحہ
pdf-view-two-pages = دو صفحات
pdf-undo = کالعدم کریں
pdf-redo = دوبارہ کریں
pdf-rotate-left = بائیں گھمائیں
pdf-rotate-right = دائیں گھمائیں
pdf-inspector = انسپکٹر
pdf-markup = مارک اپ
pdf-export = برآمد کریں
pdf-settings = ترتیبات

pdf-search = تلاش کریں
pdf-search-not-found = نہیں ملا
pdf-searching = تلاش جاری ہے…
pdf-search-match = { $total } میں سے { $current }
pdf-search-match-more = { $total }+ میں سے { $current }

pdf-inspector-file = فائل
pdf-inspector-document = دستاویز
pdf-inspector-pages = صفحات
pdf-inspector-title = عنوان
pdf-inspector-author = مصنف
pdf-inspector-subject = موضوع
pdf-inspector-keywords = کلیدی الفاظ
pdf-inspector-created = تخلیق کی تاریخ
pdf-inspector-modified = ترمیم کی تاریخ
pdf-inspector-application = ایپلیکیشن
pdf-inspector-producer = PDF پروڈیوسر
pdf-inspector-version = ورژن
pdf-inspector-security = سیکیورٹی
pdf-inspector-not-encrypted = خفیہ کردہ نہیں
pdf-inspector-encrypted = خفیہ کردہ ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } صفحہ
   *[other] { $count } صفحات
}
pdf-inspector-page-size = صفحے کا سائز
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } ملی میٹر ({ $width_in } × { $height_in } انچ)
pdf-loading = لوڈ ہو رہا ہے…

pdf-tab-pages = صفحات
pdf-tab-contents = فہرست مضامین
pdf-tab-notes = نمایاں کاریاں اور نوٹس
pdf-tab-bookmarks = بک مارکس
pdf-no-outline = کوئی فہرست مضامین نہیں
pdf-no-outline-detail = اس دستاویز میں کوئی آؤٹ لائن نہیں ہے۔
pdf-no-bookmarks = کوئی بک مارک نہیں
pdf-no-bookmarks-detail = کسی صفحے کو بک مارک کرنے کے لیے { $keys } دبائیں۔
pdf-no-bookmarks-detail-unbound = بک مارک کیے گئے صفحات یہاں دکھائی دیتے ہیں۔
pdf-remove-bookmark = بک مارک ہٹائیں

## Page editing

pages-menu = صفحات
pages-insert-blank = خالی صفحہ داخل کریں
pages-insert-file = فائل سے داخل کریں…
pages-copy = { $count ->
    [one] صفحہ کاپی کریں
   *[other] صفحات کاپی کریں
}
pages-paste = { $count ->
    [one] صفحہ چسپاں کریں
   *[other] { $count } صفحات چسپاں کریں
}
pages-crop = انتخاب تک تراشیں
pages-select-all = تمام صفحات منتخب کریں
pages-delete = { $count ->
    [one] صفحہ حذف کریں
   *[other] صفحات حذف کریں
}
pages-apply-redactions = پوشیدگیاں لاگو کریں…
pages-no-copied = چسپاں کرنے کے لیے کوئی کاپی شدہ صفحات نہیں ہیں۔
pages-copied = { $count ->
    [one] { $count } صفحہ کاپی ہو گیا۔
   *[other] { $count } صفحات کاپی ہو گئے۔
}
pages-copy-failed = صفحات کاپی نہیں کیے جا سکے: { $error }
pages-reading-stopped = پڑھنا رک گیا
pages-image-unreadable = ایسی تصویر نہیں جسے prev پڑھ سکے
pages-read-failed = فائل پڑھی نہیں جا سکی: { $error }
pages-at-least-one = دستاویز میں کم از کم ایک صفحہ ہونا ضروری ہے۔
pages-crop-needs-area = پہلے مستطیل انتخاب کے ٹول سے کوئی حصہ منتخب کریں۔
pages-change-failed = صفحات تبدیل نہیں کیے جا سکے: { $error }
pages-no-redactions = لاگو کرنے کے لیے کوئی پوشیدگی نہیں تھی۔
pages-redactions-applied = { $count ->
    [one] { $count } پوشیدگی لاگو ہو گئی۔
   *[other] { $count } پوشیدگیاں لاگو ہو گئیں۔
}
pages-forget-versions-failed = پچھلے ورژن حذف نہیں کیے جا سکے: { $error }
pages-redact-title = پوشیدگیاں لاگو کریں؟
pages-redact-body = { $count ->
    [one] نشان کے نیچے موجود متن، تصاویر اور ڈرائنگز دستاویز سے ہمیشہ کے لیے ہٹا دی جاتی ہیں، اور نشان سیاہ خانہ بن جاتا ہے۔ اسے کالعدم نہیں کیا جا سکتا، اور اس فائل کے وہ پچھلے ورژن حذف ہو جاتے ہیں جو prev رکھتا ہے۔
   *[other] { $count } نشانات کے نیچے موجود متن، تصاویر اور ڈرائنگز دستاویز سے ہمیشہ کے لیے ہٹا دی جاتی ہیں، اور نشانات سیاہ خانے بن جاتے ہیں۔ اسے کالعدم نہیں کیا جا سکتا، اور اس فائل کے وہ پچھلے ورژن حذف ہو جاتے ہیں جو prev رکھتا ہے۔
}
pages-redact-apply = لاگو کریں

## PDF export

pages-export-title = برآمد کریں
pages-export-format = فارمیٹ
pages-export-reduce = فائل کا سائز کم کریں (تصاویر 150 dpi پر)
pages-export-flatten = تشریحات اور فارم فیلڈز کو صفحات میں ضم کریں
pages-export-flatten-detail = مارک اپ اور پُر کیے گئے فیلڈز صفحات کا حصہ بن جاتے ہیں اور ان میں مزید ترمیم نہیں کی جا سکتی۔ پوشیدگی کے جو نشانات ابھی لاگو نہیں ہوئے وہ شامل نہیں کیے جاتے۔
pages-export-encrypt = پاس ورڈ سے خفیہ کریں
pages-export-password = پاس ورڈ
pages-export-verify-password = پاس ورڈ کی تصدیق کریں
pages-export-resolution = ریزولوشن
pages-export-dpi = { $dpi } dpi
pages-export-quality = معیار
pages-export-quality-low = کم
pages-export-quality-medium = درمیانہ
pages-export-quality-high = اعلیٰ
pages-export-quality-best = بہترین
pages-export-one-file = تمام صفحات ایک ہی فائل میں جاتے ہیں۔
pages-export-file-per-page = ہر صفحہ الگ فائل کے طور پر محفوظ ہوتا ہے، جس کا نام آپ کے منتخب کردہ نام کے بعد نمبر لگا کر رکھا جاتا ہے۔
pages-export-selected-only = { $count ->
    [one] صرف منتخب صفحہ
   *[other] صرف { $count } منتخب صفحات
}
pages-export-choose = برآمد کریں…
pages-export-no-password = پاس ورڈ درج کریں۔
pages-export-password-mismatch = پاس ورڈ مماثل نہیں ہیں۔
pages-export-file-name = { $name } (برآمد شدہ)
pages-export-untitled = دستاویز
pages-export-same-file = نئی فائل میں برآمد کریں؛ یہ دستاویز خود بخود محفوظ ہوتی ہے۔
pages-export-exporting = “{ $name }” برآمد کیا جا رہا ہے…
pages-export-done = “{ $name }” برآمد ہو گیا۔
pages-export-done-images = { $count ->
    [one] { $count } تصویر برآمد ہو گئی۔
   *[other] { $count } تصاویر برآمد ہو گئیں۔
}
pages-export-failed = برآمد نہیں کیا جا سکا: { $error }
pages-export-stopped = برآمد کرنا رک گیا

## Start window

app-start-hint = کوئی PDF، تصویر، SVG یا Markdown فائل کھولیں یا یہاں ڈراپ کریں۔
app-start-open = کھولیں…
app-title-dev = { $title } (ڈیولپمنٹ)
app-viewer-missing = { $kind }: یہ ویوئر ابھی تیار نہیں ہوا۔
app-cannot-open = prev اس قسم کی فائل نہیں کھول سکتا۔
app-cannot-read = prev یہ فائل نہیں پڑھ سکتا: { $error }
app-kind-pdf = PDF دستاویز
app-kind-image = { $format } تصویر
app-kind-svg = SVG ڈرائنگ
app-kind-markdown = Markdown دستاویز
app-file-dialog-failed = فائل ڈائیلاگ نہیں دکھایا جا سکا: { $error }

## Actions

action-open = کھولیں
action-settings = ترتیبات

## Toolbar

app-toolbar-keep-shown = ٹول بار دکھاتے رہیں
app-toolbar-auto-hide = پوائنٹر ہٹنے پر ٹول بار چھپائیں
app-toolbar-more = مزید

## File facts

app-fact-name = نام
app-fact-folder = فولڈر
app-fact-size = سائز
app-fact-modified = ترمیم کی تاریخ
app-size-bytes = { $count } بائٹس
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = غلط لنک { $uri }: { $error }
app-link-open-failed = { $uri } نہیں کھولا جا سکا: { $error }
app-paste-needs-wl-clipboard = تصاویر چسپاں کرنے کے لیے wl-clipboard انسٹال کریں
app-copy-needs-wl-clipboard = تصاویر کاپی کرنے کے لیے wl-clipboard انسٹال کریں
app-copy-no-pixels = اس حصے میں کوئی پکسل نہیں
app-copy-no-input = wl-copy کو کوئی ان پٹ نہیں ملا
app-copy-failed = wl-copy ناکام ہو گیا
app-clipboard-open-failed = کلپ بورڈ نہیں کھولا جا سکا: { $error }
app-copy-image-failed = تصویر کاپی نہیں کی جا سکی: { $error }

## Printing

print-failed = پرنٹ نہیں کیا جا سکا: { $error }
print-stopped = پرنٹنگ رک گئی
print-unavailable = اس سسٹم پر ابھی پرنٹنگ دستیاب نہیں ہے۔
print-no-window = پرنٹ نہیں کیا جا سکا: پرنٹ ڈائیلاگ دکھانے کے لیے کوئی ونڈو نہیں
print-dialog-failed = پرنٹ ڈائیلاگ نہیں دکھایا جا سکا: { $error }
print-job-not-started = پرنٹر نے کام شروع نہیں کیا
print-printer-stopped = پرنٹر رک گیا

## File dialogs

dialog-open = کھولیں
dialog-filter-all = تمام معاون فائلیں
dialog-filter-pdf = PDF دستاویزات
dialog-filter-images = تصاویر
dialog-filter-svg = SVG ڈرائنگز
dialog-filter-markdown = Markdown
dialog-choose-signatures = دستخطوں کا فولڈر منتخب کریں
dialog-choose-versions = ورژن ہسٹری کا فولڈر منتخب کریں
dialog-choose-bookmarks = بک مارکس کی فائل منتخب کریں

## Command line

usage-help =
    استعمال: prev [FILE]...
             prev --mcp

    PDF اور تصاویر دیکھیں اور ان میں ترمیم کریں۔ فائلیں چلتے ہوئے prev کی
    ونڈوز میں کھلتی ہیں، جو ضرورت ہو تو شروع ہو جاتا ہے۔

    اختیارات:
      -h, --help     یہ مدد دکھائیں
      -V, --version  ورژن دکھائیں
          --mcp      stdin اور stdout پر MCP فراہم کریں، تاکہ AI ایجنٹ چلتے ہوئے
                     prev کو کنٹرول کر سکیں

## Settings, continued

settings-language = زبان
settings-language-system = سسٹم ڈیفالٹ: { $language }
settings-input-language = ان پٹ زبان
settings-input-language-system = کی بورڈ لے آؤٹ کے مطابق
settings-input-language-note = طے کرتا ہے کہ خالی ٹیکسٹ فیلڈ کس طرف سے شروع ہو۔ آپ جو متن ٹائپ کرتے ہیں وہ اپنی سمت برقرار رکھتا ہے۔

settings-appearance-system = سسٹم
settings-appearance-light = روشن
settings-appearance-dark = تاریک
settings-system-accent = سسٹم کا ایکسنٹ رنگ استعمال کریں
settings-omarchy-note = رنگ “{ $theme }” کے ایکسنٹ رنگ سے بنائے جاتے ہیں۔
settings-system-accent-note = رنگ سسٹم کے ایکسنٹ رنگ سے بنائے جاتے ہیں۔
settings-system-accent-none = سسٹم میں کوئی ایکسنٹ رنگ نہیں ہے، اس لیے prev نیچے منتخب کیا گیا رنگ استعمال کرتا ہے۔
settings-accent-chosen-note = رنگ نیچے منتخب کیے گئے رنگ سے بنائے جاتے ہیں۔
settings-auto-hide = پوائنٹر ہٹنے پر ٹول بار چھپائیں
settings-auto-hide-note = ٹول بار دستاویز کے اوپر تیرتا ہے اور جب پوائنٹر ونڈو سے باہر ہو تو کھسک کر ہٹ جاتا ہے۔
settings-animations = اینیمیشنز
settings-animations-note = کھسکتی بارز اور پینلز، پھیلتے ڈائیلاگز اور لچکدار بٹن۔
settings-animations-reduced = جب سسٹم کم حرکت کا تقاضا کرے تو بند رہتی ہیں۔
settings-corner-radius = کونوں کی گولائی
settings-corner-radius-note = ڈائیلاگز اور تیرتے ٹول بار کے لیے۔
settings-corner-radius-value = { $radius } پکسل
settings-overlay = اوورلے کی شفافیت
settings-overlay-note = تیرتے ٹول بار میں سے صفحہ کتنا نظر آئے۔
settings-overlay-value = { $percent }%
settings-storage-signatures = دستخطوں کا فولڈر
settings-storage-versions = ورژن ہسٹری کا فولڈر
settings-storage-bookmarks = بک مارکس کی فائل
settings-storage-apply = لاگو کریں
settings-storage-choose = منتخب کریں…
settings-storage-note = پرانی جگہ پر پہلے سے رکھی گئی فائلیں وہیں رہتی ہیں؛ انہیں استعمال کرتے رہنے کے لیے نئی جگہ منتقل کریں۔ prev ایپ کی ترتیبات { $file } میں محفوظ ہوتی ہیں۔
settings-save-failed = ترتیبات محفوظ نہیں کی جا سکیں: { $error }
settings-no-location = ترتیبات کا کوئی مقام نہیں: HOME سیٹ نہیں ہے
settings-full-path = پورا پاتھ استعمال کریں، جیسے ~/Documents/prev۔
settings-path-is-folder = { $path } فولڈر ہے، فائل نہیں۔
settings-folder-missing = { $path } نام کا کوئی فولڈر نہیں ہے۔ پہلے اسے بنائیں، یا کوئی فولڈر منتخب کریں۔
settings-path-is-file = { $path } فائل ہے، فولڈر نہیں۔
settings-cannot-write = prev { $path } میں نہیں لکھ سکتا: { $error }۔

## Export dialog

export-title = برآمد کریں
export-format = فارمیٹ
export-quality = معیار
export-size = سائز
export-choose = برآمد کریں…
export-format-webp = WebP (بغیر نقصان)
export-format-unknown = تصویر
export-quality-low = کم
export-quality-medium = درمیانہ
export-quality-high = اعلیٰ
export-quality-best = بہترین
export-size-actual = اصل سائز
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } پکسلز
export-dialog-failed = محفوظ کرنے کا ڈائیلاگ نہیں دکھایا جا سکا: { $error }
export-done = { $path } برآمد ہو گیا
export-failed = برآمد نہیں کیا جا سکا: { $error }
export-stopped = برآمد کرنا رک گیا

## Image window

image-marked-no-edit = مارک اپ والی تصاویر میں ترمیم نہیں کی جا سکتی۔ مارک اپ رکھنے کے لیے برآمد کریں، یا اسے حذف کر کے مارک اپ بار بند کریں۔

image-loading-stopped = لوڈ ہونا رک گیا
image-reverting-stopped = بحال کرنا رک گیا
image-rendering-stopped = رینڈر کرنا رک گیا
image-saving-stopped = محفوظ کرنا رک گیا
image-markup-stopped = مارک اپ رک گیا
image-no-version-store = ورژن رکھنے کی کوئی جگہ نہیں
image-revert-failed = بحال نہیں کیا جا سکا: { $error }
image-read-failed = { $path } پڑھا نہیں جا سکا: { $error }
image-keep-original-failed = اصل ورژن نہیں رکھا جا سکا: { $error }
image-save-failed = { $path } محفوظ نہیں کیا جا سکا: { $error }
image-markup-start-failed = مارک اپ شروع نہیں ہو سکا: { $error }
image-cannot-edit = اینیمیشنز اور SVG ڈرائنگز میں ترمیم نہیں کی جا سکتی۔
image-cannot-mark-up = اینیمیشنز اور SVG ڈرائنگز پر مارک اپ نہیں کیا جا سکتا۔
image-mark-up-wait = ترمیم مکمل ہونے کا انتظار کریں، پھر مارک اپ کریں۔
image-crop-needs-selection = پہلے ڈریگ کر کے انتخاب کریں (انتخاب کا ٹول)، پھر تراشیں۔
image-size-needed = چوڑائی اور اونچائی پکسلز میں درج کریں۔
image-cannot-save-format = “{ $name }” میں تبدیلیاں اس کے فارمیٹ میں محفوظ نہیں کی جا سکتیں۔ برآمد ({ $keys }) استعمال کریں۔
image-cannot-save-format-unbound = “{ $name }” میں تبدیلیاں اس کے فارمیٹ میں محفوظ نہیں کی جا سکتیں۔ برآمد استعمال کریں۔
image-cannot-export-animation = اینیمیشنز ابھی برآمد نہیں کی جا سکتیں۔
image-drop-pages = صفحات کسی دستاویز پر ڈراپ کیے جا سکتے ہیں۔
image-drag-failed = ڈریگ کرنا شروع نہیں ہو سکا۔
image-picture-save-failed = تصویر آپ کے ڈاؤن لوڈز فولڈر میں محفوظ نہیں کی جا سکی۔
image-open-failed = prev یہ تصویر نہیں کھول سکتا
image-opening = کھل رہا ہے…
image-name-mismatch-title = نام فارمیٹ سے مماثل نہیں ہے
image-name-mismatch = “{ $name }” بطور { $format } فائل محفوظ ہو گی، لیکن اس کا نام .{ $extension } پر ختم ہوتا ہے۔ ہو سکتا ہے دوسری ایپس اسے نہ کھول سکیں۔
image-name-mismatch-no-extension = “{ $name }” بطور { $format } فائل محفوظ ہو گی، لیکن اس کے نام میں کوئی ایکسٹینشن نہیں ہے۔ ہو سکتا ہے دوسری ایپس اسے نہ کھول سکیں۔
image-choose-again = دوبارہ منتخب کریں
image-save-as-is = جوں کا توں محفوظ کریں
image-dimensions = { $width } × { $height }
image-frame-position = فریم { $current } از { $total }
image-position = { $total } میں سے { $current }
image-edited = ترمیم شدہ
image-sidebar = سائڈ بار
image-zoom-out = زوم آؤٹ
image-zoom-in = زوم ان
image-zoom = { $percent }%
image-fit = ونڈو میں فٹ کریں
image-actual-size = اصل سائز
image-undo = کالعدم کریں
image-redo = دوبارہ کریں
image-rotate-left = بائیں گھمائیں
image-rotate-right = دائیں گھمائیں
image-flip-horizontal = افقی پلٹیں
image-flip-vertical = عمودی پلٹیں
image-select = مستطیل انتخاب
image-crop = انتخاب تک تراشیں
image-adjust-size-tool = سائز ایڈجسٹ کریں
image-adjust-color-tool = رنگ ایڈجسٹ کریں
image-inspector = انسپکٹر
image-markup = مارک اپ
image-export = برآمد کریں
image-settings = ترتیبات
image-adjust-color = رنگ ایڈجسٹ کریں
image-adjust-size = سائز ایڈجسٹ کریں
image-exposure = ایکسپوژر
image-contrast = کنٹراسٹ
image-saturation = سیچوریشن
image-temperature = درجۂ حرارت
image-tint = رنگت
image-sepia = سیپیا
image-sharpness = شارپنیس
image-levels = لیولز
image-black-point = سیاہ نقطہ
image-midtones = درمیانی ٹونز
image-white-point = سفید نقطہ
image-reset-all = سب ری سیٹ کریں
image-current-size = موجودہ سائز: { $width } × { $height } پکسلز
image-width = چوڑائی
image-height = اونچائی
image-scale-proportionally = تناسب کے ساتھ اسکیل کریں
image-resize = سائز تبدیل کریں
image-inspector-loading = لوڈ ہو رہا ہے…
image-file = فائل
image-format = فارمیٹ
image-dimensions-label = ابعاد
image-pixels = { $width } × { $height } پکسلز
image-no-camera = کیمرے کی کوئی معلومات نہیں۔
image-location = مقام
image-remove-location = مقام کی معلومات ہٹائیں
image-no-location = مقام کی کوئی معلومات نہیں۔
image-keywords-description = کلیدی الفاظ اور تفصیل
image-keywords-hint = کلیدی الفاظ، کوما سے الگ کیے ہوئے
image-description = تفصیل
image-keywords-unsupported = کلیدی الفاظ JPEG، PNG اور WebP فائلوں میں محفوظ کیے جا سکتے ہیں۔
image-revert-to = کسی ورژن پر بحال کریں
image-no-versions = کوئی پچھلا ورژن نہیں۔
image-revert = بحال کریں
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = مارک اپ برآمد کیے بغیر بند کریں؟
image-close-body = { $count ->
    [one] تصویر پر مارک اپ صرف اس وقت تک رہتا ہے جب تک اس کی ونڈو کھلی ہو۔ اسے رکھنے کے لیے تصویر برآمد کریں: مارک اپ اس کاپی میں شامل ہو جاتا ہے جو آپ محفوظ کرتے ہیں۔
   *[other] تصاویر پر مارک اپ صرف اس وقت تک رہتا ہے جب تک ان کی ونڈو کھلی ہو۔ اسے رکھنے کے لیے ہر تصویر برآمد کریں: مارک اپ اس کاپی میں شامل ہو جاتا ہے جو آپ محفوظ کرتے ہیں۔
}
image-close-anyway = پھر بھی بند کریں

## Markdown

markdown-reading-stopped = پڑھنا رک گیا
markdown-read-failed = prev یہ فائل نہیں پڑھ سکتا
markdown-draw-failed = دستاویز رینڈر نہیں کی جا سکی
markdown-export-size = پوری دستاویز، { $width } × { $height } پکسلز
markdown-not-found = نہیں ملا
markdown-match = { $total } میں سے { $current }
markdown-search = تلاش کریں
markdown-smaller-text = چھوٹا متن
markdown-larger-text = بڑا متن
markdown-zoom = { $percent }%
markdown-actual-size = اصل سائز
markdown-inspector = انسپکٹر
markdown-export = برآمد کریں
markdown-settings = ترتیبات
markdown-file = فائل
markdown-document = دستاویز
markdown-words = الفاظ
markdown-lines = سطریں
markdown-pictures = تصاویر

## Image details

image-meta-camera = کیمرا
image-meta-exposure = ایکسپوژر
image-meta-image = تصویر
image-meta-make = بنانے والا
image-meta-model = ماڈل
image-meta-lens = لینز
image-meta-exposure-time = ایکسپوژر کا وقت
image-meta-f-number = F نمبر
image-meta-iso = ISO
image-meta-focal-length = فوکل لمبائی
image-meta-exposure-bias = ایکسپوژر بائس
image-meta-flash = فلیش
image-meta-date-taken = تصویر لینے کی تاریخ
image-meta-orientation = سمت بندی
image-meta-color-space = کلر اسپیس
image-meta-software = سافٹ ویئر
image-meta-artist = فنکار
image-meta-copyright = کاپی رائٹ
image-meta-seconds = { $value } سیکنڈ
image-meta-millimeters = { $value } ملی میٹر
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] عام
    [2] افقی طور پر الٹا ہوا
    [3] 180° گھمایا ہوا
    [4] عمودی طور پر الٹا ہوا
    [5] افقی طور پر الٹا ہوا، 90° گھڑی کی مخالف سمت میں گھمایا ہوا
    [6] 90° گھڑی کی سمت میں گھمایا ہوا
    [7] افقی طور پر الٹا ہوا، 90° گھڑی کی سمت میں گھمایا ہوا
    [8] 90° گھڑی کی مخالف سمت میں گھمایا ہوا
   *[other] نامعلوم ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] چلا
   *[no] نہیں چلا
}{ $mode ->
    [on] ، لازمی آن
    [off] ، آف
    [auto] ، خودکار
   *[unknown] {""}
}{ $redeye ->
    [yes] ، سرخ آنکھ میں کمی
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] غیر کیلیبریٹڈ
   *[other] دیگر ({ $code })
}

## Errors

error-pdf-open = دستاویز نہیں کھل سکتی: { $detail }
error-pdf-page-out-of-range = صفحہ { $page } موجود نہیں ہے
error-pdf-password-protected = دستاویز پاس ورڈ سے محفوظ ہے؛ اس کے بجائے اسے کھولیں اور اس کے صفحات کاپی کریں
error-pdf-no-pages = نکالنے کے لیے کوئی صفحات نہیں
error-pdf-crop-outside = تراشنے کا حصہ صفحے سے باہر ہے
error-pdf-closed = دستاویز بند ہو گئی
error-pdf-saved-unreadable = محفوظ کی گئی دستاویز اب نہیں کھلتی
error-image-read = فائل نہیں پڑھی جا سکتی: { $detail }
error-image-invalid = تصویر خراب یا غلط ہے: { $detail }
error-image-missing-library = اس فارمیٹ کو کھولنے کے لیے { $library } درکار ہے، جو انسٹال نہیں ہے
error-image-unsupported = { $format } تصاویر ابھی معاون نہیں ہیں
error-image-encode = تصویر انکوڈ نہیں کی جا سکتی: { $detail }
error-exif-malformed = EXIF ڈیٹا خراب ہے
error-settings-read = ترتیبات نہیں پڑھی جا سکتیں: { $detail }
error-settings-invalid = غلط ترتیبات: { $detail }
error-remove-location = مقام نہیں ہٹایا جا سکا: { $error }
error-location-unsupported = مقام کی معلومات JPEG، PNG، WebP اور TIFF فائلوں سے ہٹائی جا سکتی ہیں
error-xmp-unsupported = کلیدی الفاظ اور تفصیلات صرف JPEG، PNG اور WebP فائلوں میں محفوظ کی جا سکتی ہیں

## Formats

format-camera-raw = کیمرا RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev کے بارے میں
menu-settings = ترتیبات…
menu-services = خدمات
menu-hide = prev چھپائیں
menu-hide-others = دیگر چھپائیں
menu-show-all = سب دکھائیں
menu-quit = prev بند کریں
menu-file = فائل
menu-open = کھولیں…
menu-close = ونڈو بند کریں
menu-export = برآمد کریں…
menu-print = پرنٹ کریں…
menu-edit = ترمیم
menu-undo = کالعدم کریں
menu-redo = دوبارہ کریں
menu-cut = کاٹیں
menu-copy = کاپی کریں
menu-paste = چسپاں کریں
menu-select-all = سب منتخب کریں
menu-find = تلاش کریں
menu-find-next = اگلا تلاش کریں
menu-find-previous = پچھلا تلاش کریں
menu-view = منظر
menu-hide-sidebar = سائڈ بار چھپائیں
menu-thumbnails = تھمب نیلز
menu-contents = فہرست مضامین
menu-notes = نمایاں کاریاں اور نوٹس
menu-bookmarks = بک مارکس
menu-zoom-in = زوم ان
menu-zoom-out = زوم آؤٹ
menu-actual-size = اصل سائز
menu-zoom-to-fit = فٹ کرنے کے لیے زوم کریں
menu-inspector = انسپکٹر دکھائیں
menu-slideshow = سلائیڈ شو
menu-full-screen = پوری اسکرین پر جائیں
menu-go = جائیں
menu-next-page = اگلا صفحہ
menu-previous-page = پچھلا صفحہ
menu-go-to-page = صفحے پر جائیں…
menu-bookmark = بک مارک شامل کریں
menu-tools = ٹولز
menu-markup = مارک اپ ٹول بار دکھائیں
menu-rotate-left = بائیں گھمائیں
menu-rotate-right = دائیں گھمائیں
menu-crop = تراشیں
menu-adjust-color = رنگ ایڈجسٹ کریں…
menu-window = ونڈو
menu-minimize = چھوٹا کریں
menu-zoom = زوم
menu-bring-all-to-front = سب کو سامنے لائیں

## Outside control

settings-outside-control = بیرونی کنٹرول
settings-allow-outside-control = بیرونی کنٹرول کی اجازت دیں
settings-allow-outside-control-note = Claude Code جیسے AI ایجنٹ prev --mcp کے ذریعے prev میں آپ کی فائلیں پڑھ اور بدل سکتے ہیں۔ ہر نئے ایجنٹ سے پہلے prev پوچھتا ہے۔
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = اجازت یافتہ: { $agents }
settings-forget-agents = بھول جائیں
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = کیا { $agent } کو prev کنٹرول کرنے کی اجازت دیں؟
agent-prompt-body = { $agent } آپ کی کھلی فائلیں پڑھنے اور بدلنے کے لیے prev کا بیرونی کنٹرول استعمال کرنا چاہتا ہے۔ آپ بیرونی کنٹرول کو ترتیبات میں بند کر سکتے ہیں۔
agent-prompt-allow = اجازت دیں
agent-prompt-deny = اجازت نہ دیں
settings-ask-before-note = ایجنٹ کے یہ کام کرنے سے پہلے پوچھیں:
settings-ask-reading = فائل پڑھنا
settings-ask-viewing = منظر یا ونڈو بدلنا
settings-ask-marking-up = فائل پر مارک اپ کرنا
settings-ask-editing = فائل میں ترمیم کرنا
settings-ask-signing = فائل پر دستخط کرنا
settings-ask-redacting = پوشیدگیاں لاگو کرنا
settings-ask-exporting = فائل برآمد کرنا
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = کیا { $agent } کو یہ فائل پڑھنے دیں؟
agent-ask-view = کیا { $agent } کو منظر بدلنے دیں؟
agent-ask-markup = کیا { $agent } کو اس فائل پر مارک اپ کرنے دیں؟
agent-ask-edit = کیا { $agent } کو اس فائل میں ترمیم کرنے دیں؟
agent-ask-sign = کیا { $agent } کو اس فائل پر دستخط کرنے دیں؟
agent-ask-redact = کیا { $agent } کو پوشیدگیاں لاگو کرنے دیں؟
agent-ask-export = کیا { $agent } کو یہ فائل برآمد کرنے دیں؟
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } “{ $tool }” استعمال کرنا چاہتا ہے۔ prev کن باتوں پر پوچھے، یہ ترتیبات میں منتخب کیا جاتا ہے۔
agent-ask-final = اسے کالعدم نہیں کیا جا سکتا۔
