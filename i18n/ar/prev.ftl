# prev's interface text in Arabic (العربية, Modern Standard Arabic), translated from i18n/en/prev.ftl.
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
language-name = العربية

## Common

common-cancel = إلغاء
common-close = إغلاق
common-save = حفظ

## Settings

settings-title = الإعدادات
settings-appearance = المظهر
settings-colors = الألوان
settings-windows = النوافذ
settings-default-app = التطبيق الافتراضي
settings-default-app-label = فتح الملفات باستخدام prev
settings-default-app-note = اجعل prev التطبيق الذي يفتح ملفات PDF والصور ورسومات SVG وملفات Markdown.
settings-default-app-note-windows = يتيح Windows اختيار التطبيقات الافتراضية في إعداداته فقط. يفتح هذا صفحة prev هناك.
settings-default-app-note-macos = يطلب macOS تأكيد كل نوع: PDF وPNG وJPEG وHEIC وGIF وTIFF وWebP وAVIF.
settings-default-app-status = { $set } من { $total } من أنواع الملفات تُفتح باستخدام prev.
settings-default-app-button = تعيين كافتراضي
settings-default-app-button-windows = فتح الإعدادات
settings-default-app-no-entry = إدخال سطح المكتب الخاص بـ prev غير مثبّت، لذا لا يستطيع النظام فتح الملفات به. ثبّت prev من حزمة أو باستخدام scripts/install.sh.
settings-default-app-no-bundle = افتح prev من prev.app لجعله التطبيق الافتراضي.
settings-default-app-failed = تعذّر جعل prev التطبيق الافتراضي: { $error }
settings-storage = التخزين
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (إصدار تطويري، { $build })

## Markup toolbar

markup-tool-select = تحديد
markup-tool-area = تحديد مستطيل
markup-tool-sketch = تخطيط
markup-tool-draw = رسم
markup-tool-shapes = الأشكال
markup-tool-text-box = مربع نص
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = تمييز
markup-tool-note = ملاحظة
# Opens the menu of saved signatures (a verb).
markup-tool-sign = توقيع
# A verb: the tool that marks areas to black out.
markup-tool-redact = تنقيح
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = تطبيق
markup-apply-redactions = تطبيق التنقيحات
markup-shape-style = نمط الشكل
markup-border-color = لون الحد
markup-fill-color = لون التعبئة
markup-text-style = نمط النص
markup-delete = حذف
markup-undo = تراجع
markup-redo = إعادة

## Markup menus

markup-shape-rectangle = مستطيل
markup-shape-rounded-rectangle = مستطيل مستدير الزوايا
markup-shape-oval = شكل بيضاوي
markup-shape-line = خط
markup-shape-arrow = سهم
markup-shape-star = نجمة
markup-shape-polygon = مضلع
markup-shape-speech-bubble = فقاعة كلام
# A shape that magnifies the part of the page under it.
markup-shape-loupe = عدسة مكبرة
# A shape that darkens the page around it.
markup-shape-mask = قناع
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = تمييز
markup-style-underline = تسطير
markup-style-strikethrough = يتوسطه خط
markup-style-squiggly = خط متموج
# Menu section headings.
markup-menu-color = اللون
markup-menu-font = الخط
markup-menu-size = الحجم
markup-menu-alignment = المحاذاة
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } نقطة
markup-dashed = متقطع

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = ملاحظة
markup-kind-text-box = مربع نص
markup-kind-stamp = ختم
markup-kind-redaction = تنقيح
markup-kind-shape = شكل
# Tooltips on a note being edited.
markup-note-delete = حذف الملاحظة
markup-note-done = تم
markup-note-placeholder = اكتب ملاحظة
markup-notes-empty = لا توجد تمييزات أو ملاحظات
markup-notes-empty-hint = تظهر هنا التمييزات والملاحظات ومربعات النص.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = الصفحة { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = تعذّر تغيير المستند: { $error }
markup-copy-area-failed = تعذّر نسخ المنطقة: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = أُغلق المستند
markup-render-area-failed = تعذّر عرض المنطقة
markup-copy-stopped = توقف النسخ

## Signatures

signature-menu-empty = لا توجد توقيعات بعد.
signature-delete = حذف التوقيع
signature-create = إنشاء توقيع…
signature-dialog-title = إنشاء توقيع
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = رسم
signature-tab-type = كتابة
signature-tab-image = صورة
signature-draw-hint = وقّع على الخط باستخدام الماوس أو القلم أو لوحة اللمس.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = اسمك
signature-image-hint = اختر صورة أو مسحًا ضوئيًا لتوقيعك على ورقة بيضاء.
signature-choose-image = اختيار صورة…
# Placeholder of the field naming the signature in the library.
signature-description = الوصف، مثل الاسم الكامل أو الأحرف الأولى
# Clears the drawing, typed name or image.
signature-clear = مسح
# The color the signature is drawn or typed in.
signature-ink = الحبر
# The pen's width, for drawing.
signature-thickness = السُّمك
signature-sign-first = وقّع أولًا، ثم احفظ.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = التوقيع { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = تعذّر تغيير التوقيعات: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = لا يوجد مجلد بيانات: لم يتم تعيين HOME
signature-removing-stopped = توقفت الإزالة
signature-saving-stopped = توقف الحفظ
signature-reading-stopped = توقفت القراءة
signature-not-an-image = هذا الملف ليس صورة يمكن لـ prev قراءتها
signature-no-frames = لا تحتوي الصورة على أي إطار
signature-not-found = لم يُعثر على توقيع في الصورة

## Dragging

drag-pages-need-document = يمكن إفلات الصفحات على مستند.
drag-image-unsupported = لا يمكن لـ prev فتح هذه الصورة.
# $error is a lowercase reason or a technical message.
drag-area-failed = تعذّر سحب المنطقة: { $error }
drag-pages-failed = تعذّر سحب الصفحات: { $error }
drag-start-failed = تعذّر بدء السحب.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = الصفحات
drag-file-one-page = { $name } (الصفحة { $page })
drag-file-page-range = { $name } (الصفحات { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = صورة
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = هل تريد الإضافة إلى هذا المستند؟
drop-pdf-body = هل تريد إضافة “{ $name }” إلى نهاية هذا المستند، أم فتحه في نافذة مستقلة؟
drop-pdfs-body = { $count ->
    [zero] هل تريد إضافة ملفات PDF هذه وعددها { $count } إلى نهاية هذا المستند، أم فتحها في نوافذ مستقلة؟
    [one] هل تريد إضافة ملف PDF هذا إلى نهاية هذا المستند، أم فتحه في نافذة مستقلة؟
    [two] هل تريد إضافة ملفَي PDF هذين إلى نهاية هذا المستند، أم فتحهما في نافذتين مستقلتين؟
    [few] هل تريد إضافة ملفات PDF هذه وعددها { $count } ملفات إلى نهاية هذا المستند، أم فتحها في نوافذ مستقلة؟
    [many] هل تريد إضافة ملفات PDF هذه وعددها { $count } ملفًا إلى نهاية هذا المستند، أم فتحها في نوافذ مستقلة؟
   *[other] هل تريد إضافة ملفات PDF هذه وعددها { $count } ملف إلى نهاية هذا المستند، أم فتحها في نوافذ مستقلة؟
}
drop-pdf-add = إضافة إلى النهاية
drop-pdf-open = فتح بشكل منفصل

## PDF window

pdf-opening = جارٍ الفتح…
pdf-open-failed = لا يمكن لـ prev فتح هذا المستند
pdf-no-pages = لا يحتوي المستند على أي صفحة.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = أُغلق المستند
pdf-keep-original-failed = تعذّر الاحتفاظ بالإصدار الأصلي: { $error }
pdf-save-failed = تعذّر الحفظ: { $error }
pdf-nothing-to-paste = لا يوجد شيء للصقه.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = توقف اللصق
pdf-file-dialog-failed = تعذّر عرض مربع حوار الملفات: { $error }
pdf-bookmarks-no-home = لا يمكن حفظ الإشارات المرجعية: لم يتم تعيين HOME
pdf-bookmarks-save-failed = تعذّر حفظ الإشارات المرجعية: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = الصفحة { $page }

# Password prompt. $name is the file name.
pdf-password-protected = الملف “{ $name }” محمي بكلمة سر
pdf-password = كلمة السر
pdf-password-wrong = كلمة السر غير صحيحة. حاول مرة أخرى.
# Button that opens a locked document.
pdf-unlock = فتح القفل

# Toolbar tooltips and labels.
pdf-sidebar = الشريط الجانبي
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = من { $count }
pdf-zoom-out = تصغير
pdf-zoom-in = تكبير
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = ملاءمة الصفحة
pdf-fit-width = ملاءمة العرض
pdf-actual-size = الحجم الفعلي
pdf-view-continuous = تمرير مستمر
pdf-view-single-page = صفحة مفردة
pdf-view-two-pages = صفحتان
pdf-undo = تراجع
pdf-redo = إعادة
pdf-rotate-left = تدوير إلى اليسار
pdf-rotate-right = تدوير إلى اليمين
pdf-inspector = المراقب
pdf-markup = التوصيف
# Tooltip of the button that opens the export dialog.
pdf-export = تصدير
pdf-settings = الإعدادات

# Search field.
pdf-search = بحث
pdf-search-not-found = لم يُعثر على نتائج
pdf-searching = جارٍ البحث…
# The match shown, of all matches found.
pdf-search-match = { $current } من { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } من { $total }+

# Inspector: section headings.
pdf-inspector-file = الملف
pdf-inspector-document = المستند
pdf-inspector-pages = الصفحات
# Inspector: fact labels and values.
pdf-inspector-title = العنوان
pdf-inspector-author = المؤلف
pdf-inspector-subject = الموضوع
pdf-inspector-keywords = الكلمات الرئيسية
pdf-inspector-created = تاريخ الإنشاء
pdf-inspector-modified = تاريخ التعديل
pdf-inspector-application = التطبيق
pdf-inspector-producer = منتج PDF
pdf-inspector-version = الإصدار
pdf-inspector-security = الأمان
pdf-inspector-not-encrypted = غير مشفّر
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = مشفّر ({ $method })
pdf-inspector-page-count = { $count ->
    [zero] { $count } صفحة
    [one] صفحة واحدة
    [two] صفحتان
    [few] { $count } صفحات
    [many] { $count } صفحة
   *[other] { $count } صفحة
}
pdf-inspector-page-size = حجم الصفحة
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } مم ({ $width_in } × { $height_in } بوصة)
pdf-loading = جارٍ التحميل…

# Sidebar tabs and lists.
pdf-tab-pages = الصفحات
pdf-tab-contents = المحتويات
pdf-tab-notes = التمييزات والملاحظات
pdf-tab-bookmarks = الإشارات المرجعية
pdf-no-outline = لا يوجد جدول محتويات
pdf-no-outline-detail = لا يحتوي هذا المستند على مخطط تفصيلي.
pdf-no-bookmarks = لا توجد إشارات مرجعية
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = اضغط على { $keys } لإضافة إشارة مرجعية إلى صفحة.
pdf-no-bookmarks-detail-unbound = تظهر هنا الصفحات التي تضيف إليها إشارة مرجعية.
pdf-remove-bookmark = إزالة الإشارة المرجعية

## Page editing

# Tooltip of the Pages menu button.
pages-menu = الصفحات
pages-insert-blank = إدراج صفحة فارغة
pages-insert-file = إدراج من ملف…
pages-copy = { $count ->
    [zero] نسخ الصفحات
    [one] نسخ الصفحة
    [two] نسخ الصفحتين
    [few] نسخ الصفحات
    [many] نسخ الصفحات
   *[other] نسخ الصفحات
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [zero] لصق { $count } صفحة
    [one] لصق الصفحة
    [two] لصق الصفحتين
    [few] لصق { $count } صفحات
    [many] لصق { $count } صفحة
   *[other] لصق { $count } صفحة
}
pages-crop = اقتصاص إلى التحديد
pages-select-all = تحديد كل الصفحات
pages-delete = { $count ->
    [zero] حذف الصفحات
    [one] حذف الصفحة
    [two] حذف الصفحتين
    [few] حذف الصفحات
    [many] حذف الصفحات
   *[other] حذف الصفحات
}
pages-apply-redactions = تطبيق التنقيحات…
pages-no-copied = لا توجد صفحات منسوخة للصقها.
pages-copied = { $count ->
    [zero] تم نسخ { $count } صفحة.
    [one] تم نسخ صفحة واحدة.
    [two] تم نسخ صفحتين.
    [few] تم نسخ { $count } صفحات.
    [many] تم نسخ { $count } صفحة.
   *[other] تم نسخ { $count } صفحة.
}
pages-copy-failed = تعذّر نسخ الصفحات: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = توقفت القراءة
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = ليس صورة يمكن لـ prev قراءتها
pages-read-failed = تعذّرت قراءة الملف: { $error }
pages-at-least-one = يجب أن يحتوي المستند على صفحة واحدة على الأقل.
pages-crop-needs-area = اختر منطقة باستخدام أداة التحديد المستطيل أولًا.
pages-change-failed = تعذّر تغيير الصفحات: { $error }
pages-no-redactions = لم تكن هناك تنقيحات لتطبيقها.
pages-redactions-applied = { $count ->
    [zero] تم تطبيق { $count } تنقيح.
    [one] تم تطبيق تنقيح واحد.
    [two] تم تطبيق تنقيحين.
    [few] تم تطبيق { $count } تنقيحات.
    [many] تم تطبيق { $count } تنقيحًا.
   *[other] تم تطبيق { $count } تنقيح.
}
pages-forget-versions-failed = تعذّر حذف الإصدارات السابقة: { $error }
pages-redact-title = هل تريد تطبيق التنقيحات؟
pages-redact-body = { $count ->
    [zero] ستُزال النصوص والصور والرسومات الموجودة تحت العلامات من المستند نهائيًا، وتتحول العلامات إلى مربعات سوداء. لا يمكن التراجع عن ذلك، وستُحذف الإصدارات السابقة من هذا الملف التي يحتفظ بها prev.
    [one] ستُزال النصوص والصور والرسومات الموجودة تحت العلامة من المستند نهائيًا، وتتحول العلامة إلى مربع أسود. لا يمكن التراجع عن ذلك، وستُحذف الإصدارات السابقة من هذا الملف التي يحتفظ بها prev.
    [two] ستُزال النصوص والصور والرسومات الموجودة تحت العلامتين من المستند نهائيًا، وتتحول العلامتان إلى مربعين أسودين. لا يمكن التراجع عن ذلك، وستُحذف الإصدارات السابقة من هذا الملف التي يحتفظ بها prev.
    [few] ستُزال النصوص والصور والرسومات الموجودة تحت العلامات الـ { $count } من المستند نهائيًا، وتتحول العلامات إلى مربعات سوداء. لا يمكن التراجع عن ذلك، وستُحذف الإصدارات السابقة من هذا الملف التي يحتفظ بها prev.
    [many] ستُزال النصوص والصور والرسومات الموجودة تحت العلامات الـ { $count } من المستند نهائيًا، وتتحول العلامات إلى مربعات سوداء. لا يمكن التراجع عن ذلك، وستُحذف الإصدارات السابقة من هذا الملف التي يحتفظ بها prev.
   *[other] ستُزال النصوص والصور والرسومات الموجودة تحت العلامات الـ { $count } من المستند نهائيًا، وتتحول العلامات إلى مربعات سوداء. لا يمكن التراجع عن ذلك، وستُحذف الإصدارات السابقة من هذا الملف التي يحتفظ بها prev.
}
# Button that applies redactions.
pages-redact-apply = تطبيق

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = تصدير
pages-export-format = التنسيق
pages-export-reduce = تقليل حجم الملف (الصور بدقة 150 dpi)
pages-export-flatten = دمج التعليقات التوضيحية وحقول النماذج في الصفحات
pages-export-flatten-detail = يصبح التوصيف والحقول المعبأة جزءًا من الصفحات ولا يمكن تحريرها بعد ذلك. تُستبعد علامات التنقيح التي لم تُطبَّق بعد.
pages-export-encrypt = التشفير بكلمة سر
pages-export-password = كلمة السر
pages-export-verify-password = تأكيد كلمة السر
pages-export-resolution = الدقة
pages-export-dpi = { $dpi } dpi
pages-export-quality = الجودة
# JPEG quality choices.
pages-export-quality-low = منخفضة
pages-export-quality-medium = متوسطة
pages-export-quality-high = عالية
pages-export-quality-best = الأفضل
pages-export-one-file = توضع كل الصفحات في ملف واحد.
pages-export-file-per-page = تُحفظ كل صفحة في ملف مستقل يحمل الاسم الذي تختاره متبوعًا برقم.
pages-export-selected-only = { $count ->
    [zero] { $count } صفحة محددة فقط
    [one] الصفحة المحددة فقط
    [two] الصفحتان المحددتان فقط
    [few] الصفحات الـ { $count } المحددة فقط
    [many] الصفحات الـ { $count } المحددة فقط
   *[other] الصفحات الـ { $count } المحددة فقط
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = تصدير…
pages-export-no-password = أدخل كلمة سر.
pages-export-password-mismatch = كلمتا السر غير متطابقتين.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (مُصدَّر)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = مستند
pages-export-same-file = صدّر إلى ملف جديد؛ فهذا المستند يُحفظ تلقائيًا.
pages-export-exporting = جارٍ تصدير “{ $name }”…
pages-export-done = تم تصدير “{ $name }”.
pages-export-done-images = عدد الصور المُصدَّرة: { $count }.
pages-export-failed = تعذّر التصدير: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = توقف التصدير

## Start window

# Under the app name in a window with no file open.
app-start-hint = افتح ملف PDF أو صورة أو SVG أو Markdown، أو أفلته هنا.
app-start-open = فتح…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (إصدار التطوير)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: لم يُنجَز هذا العارض بعد.
app-cannot-open = لا يمكن لـ prev فتح هذا النوع من الملفات.
app-cannot-read = لا يمكن لـ prev قراءة هذا الملف: { $error }
app-kind-pdf = مستند PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = صورة { $format }
app-kind-svg = رسم SVG
app-kind-markdown = مستند Markdown
app-file-dialog-failed = تعذّر عرض مربع حوار الملفات: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = فتح
action-settings = الإعدادات

## Toolbar

app-toolbar-keep-shown = إبقاء شريط الأدوات ظاهرًا
app-toolbar-auto-hide = إخفاء شريط الأدوات عند مغادرة المؤشر
# The button that shows the toolbar's hidden tools.
app-toolbar-more = المزيد

## File facts

# Labels in a file's inspector.
app-fact-name = الاسم
app-fact-folder = المجلد
app-fact-size = الحجم
app-fact-modified = تاريخ التعديل
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } بايت
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = رابط غير صالح { $uri }: { $error }
app-link-open-failed = تعذّر فتح { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = ثبّت wl-clipboard للصق الصور
app-copy-needs-wl-clipboard = ثبّت wl-clipboard لنسخ الصور
app-copy-no-pixels = لا تحتوي المنطقة على أي بكسل
# wl-copy is a program's name.
app-copy-no-input = لا يوجد إدخال لـ wl-copy
app-copy-failed = فشل wl-copy
app-clipboard-open-failed = تعذّر فتح الحافظة: { $error }
app-copy-image-failed = تعذّر نسخ الصورة: { $error }

## Printing

print-failed = تعذّرت الطباعة: { $error }
print-stopped = توقفت الطباعة
print-unavailable = الطباعة غير متاحة على هذا النظام بعد.
print-no-window = تعذّرت الطباعة: لا توجد نافذة لعرض مربع حوار الطباعة فوقها
print-dialog-failed = تعذّر عرض مربع حوار الطباعة: { $error }
# Shown after "Could not print:".
print-job-not-started = لم تبدأ الطابعة المهمة
# Shown after "Could not print:".
print-printer-stopped = توقفت الطابعة

## File dialogs

dialog-open = فتح
dialog-filter-all = كل الملفات المدعومة
dialog-filter-pdf = مستندات PDF
dialog-filter-images = الصور
dialog-filter-svg = رسومات SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = اختر مجلد التوقيعات
dialog-choose-versions = اختر مجلد سجل الإصدارات
dialog-choose-bookmarks = اختر ملف الإشارات المرجعية

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    الاستخدام: prev [FILE]...
               prev --mcp

    عرض ملفات PDF والصور وتحريرها. تُفتح الملفات في نوافذ prev قيد التشغيل،
    والذي يبدأ عند الحاجة.

    الخيارات:
      -h, --help     عرض هذه المساعدة
      -V, --version  عرض الإصدار
          --mcp      تقديم MCP عبر stdin وstdout، ليتحكم وكلاء الذكاء الاصطناعي
                     في prev قيد التشغيل

## Settings, continued

settings-language = اللغة
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = الافتراضي للنظام: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = لغة الإدخال
settings-input-language-system = اتباع تخطيط لوحة المفاتيح
settings-input-language-note = يحدد الجانب الذي يبدأ منه حقل النص الفارغ. يحتفظ النص الذي تكتبه باتجاهه الخاص.

settings-appearance-system = النظام
settings-appearance-light = فاتح
settings-appearance-dark = داكن
settings-system-accent = استخدام لون التمييز في النظام
# $theme is the Omarchy theme's name.
settings-omarchy-note = الألوان مبنية على لون التمييز في سمة “{ $theme }”.
settings-system-accent-note = الألوان مبنية على لون التمييز في النظام.
settings-system-accent-none = لا يوجد لون تمييز في النظام، لذا يستخدم prev اللون المختار أدناه.
settings-accent-chosen-note = الألوان مبنية على اللون المختار أدناه.
settings-auto-hide = إخفاء شريط الأدوات عند مغادرة المؤشر
settings-auto-hide-note = يطفو شريط الأدوات فوق المستند وينزلق بعيدًا عندما يكون المؤشر خارج النافذة.
settings-animations = الحركات
settings-animations-note = أشرطة ولوحات منزلقة، ومربعات حوار تتسع، وأزرار نابضة.
settings-animations-reduced = متوقفة عندما يطلب النظام تقليل الحركة.
settings-corner-radius = نصف قطر الزوايا
settings-corner-radius-note = لمربعات الحوار وشريط الأدوات العائم.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } بكسل
settings-overlay = شفافية الطبقة العائمة
settings-overlay-note = مقدار ما يظهر من الصفحة عبر شريط الأدوات العائم.
settings-overlay-value = { $percent }%
settings-storage-signatures = مجلد التوقيعات
settings-storage-versions = مجلد سجل الإصدارات
settings-storage-bookmarks = ملف الإشارات المرجعية
settings-storage-apply = تطبيق
settings-storage-choose = اختيار…
# $file is where the settings file is.
settings-storage-note = تبقى الملفات المحفوظة في المكان القديم هناك؛ انقلها لمواصلة استخدامها. تُحفظ إعدادات تطبيق prev في { $file }.
settings-save-failed = تعذّر حفظ الإعدادات: { $error }
settings-no-location = لا يوجد موقع للإعدادات: لم يتم تعيين HOME
settings-full-path = استخدم مسارًا كاملًا، مثل ~/Documents/prev.
settings-path-is-folder = { $path } مجلد وليس ملفًا.
settings-folder-missing = لا يوجد مجلد باسم { $path }. أنشئه أولًا، أو اختر مجلدًا آخر.
settings-path-is-file = { $path } ملف وليس مجلدًا.
settings-cannot-write = لا يمكن لـ prev الكتابة في { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = تصدير
# Section headings in the export dialog.
export-format = التنسيق
export-quality = الجودة
export-size = الحجم
# Button that goes on to choose where to save the export.
export-choose = تصدير…
# Format choice; the format name stays as it is.
export-format-webp = WebP (بلا فقدان)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = صورة
# JPEG quality choices.
export-quality-low = منخفضة
export-quality-medium = متوسطة
export-quality-high = عالية
export-quality-best = الأفضل
# Size choices: the picture at its own size, or scaled up.
export-size-actual = الحجم الفعلي
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } بكسل
# $error is the system's reason.
export-dialog-failed = تعذّر عرض مربع حوار الحفظ: { $error }
# $path is where the file was saved.
export-done = تم تصدير { $path }
export-failed = تعذّر التصدير: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = توقف التصدير

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = لا يمكن تحرير الصور التي تحتوي على توصيف. صدّر الصورة للاحتفاظ بالتوصيف، أو احذفه وأغلق شريط التوصيف.

# Shown if a background task ends unexpectedly.
image-loading-stopped = توقف التحميل
image-reverting-stopped = توقف الرجوع
image-rendering-stopped = توقفت المعالجة
image-saving-stopped = توقف الحفظ
image-markup-stopped = توقف التوصيف
image-no-version-store = لا يوجد مكان للاحتفاظ بالإصدارات
image-revert-failed = تعذّر الرجوع: { $error }
image-read-failed = تعذّرت قراءة { $path }: { $error }
image-keep-original-failed = تعذّر الاحتفاظ بالإصدار الأصلي: { $error }
image-save-failed = تعذّر حفظ { $path }: { $error }
image-markup-start-failed = تعذّر بدء التوصيف: { $error }
image-cannot-edit = لا يمكن تحرير الصور المتحركة ورسومات SVG.
image-cannot-mark-up = لا يمكن إضافة توصيف إلى الصور المتحركة ورسومات SVG.
image-mark-up-wait = انتظر حتى ينتهي التعديل، ثم أضف التوصيف.
image-crop-needs-selection = اسحب لتحديد منطقة أولًا (أداة التحديد)، ثم اقتصص.
image-size-needed = أدخل عرضًا وارتفاعًا بالبكسل.
# $name is a file name.
image-cannot-save-format = لا يمكن حفظ التغييرات على “{ $name }” بتنسيقه. استخدم التصدير ({ $keys }).
image-cannot-save-format-unbound = لا يمكن حفظ التغييرات على “{ $name }” بتنسيقه. استخدم التصدير.
image-cannot-export-animation = لا يمكن تصدير الصور المتحركة بعد.
image-drop-pages = يمكن إفلات الصفحات على مستند.
image-drag-failed = تعذّر بدء السحب.
image-picture-save-failed = تعذّر حفظ الصورة في مجلد التنزيلات.
image-open-failed = لا يمكن لـ prev فتح هذه الصورة
image-opening = جارٍ الفتح…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = الاسم لا يطابق التنسيق
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = سيُحفظ “{ $name }” كملف { $format }، لكن اسمه ينتهي بـ .{ $extension }. قد لا تتمكن التطبيقات الأخرى من فتحه.
image-name-mismatch-no-extension = سيُحفظ “{ $name }” كملف { $format }، لكن اسمه بلا امتداد. قد لا تتمكن التطبيقات الأخرى من فتحه.
image-choose-again = الاختيار مجددًا
image-save-as-is = الحفظ كما هو
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = الإطار { $current } من { $total }
image-position = { $current } من { $total }
image-edited = مُعدَّلة
# Toolbar tooltips.
image-sidebar = الشريط الجانبي
image-zoom-out = تصغير
image-zoom-in = تكبير
image-zoom = { $percent }%
image-fit = ملاءمة النافذة
image-actual-size = الحجم الفعلي
image-undo = تراجع
image-redo = إعادة
image-rotate-left = تدوير إلى اليسار
image-rotate-right = تدوير إلى اليمين
image-flip-horizontal = قلب أفقي
image-flip-vertical = قلب عمودي
image-select = تحديد مستطيل
image-crop = اقتصاص إلى التحديد
image-adjust-size-tool = ضبط الحجم
image-adjust-color-tool = ضبط اللون
# Tooltip and panel title.
image-inspector = المراقب
image-markup = التوصيف
image-export = تصدير
image-settings = الإعدادات
# Panel titles.
image-adjust-color = ضبط اللون
image-adjust-size = ضبط الحجم
# Adjust Color sliders.
image-exposure = التعريض
image-contrast = التباين
image-saturation = التشبع
image-temperature = درجة الحرارة
image-tint = الصبغة
image-sepia = بني داكن
image-sharpness = الحدة
image-levels = المستويات
image-black-point = النقطة السوداء
image-midtones = الدرجات المتوسطة
image-white-point = النقطة البيضاء
image-reset-all = إعادة تعيين الكل
# Adjust Size panel.
image-current-size = الحجم الحالي: { $width } × { $height } بكسل
image-width = العرض
image-height = الارتفاع
image-scale-proportionally = تحجيم نسبي
# Button that applies the new size.
image-resize = تغيير الحجم
# Inspector panel.
image-inspector-loading = جارٍ التحميل…
image-file = الملف
image-format = التنسيق
image-dimensions-label = الأبعاد
image-pixels = { $width } × { $height } بكسل
image-no-camera = لا توجد معلومات عن الكاميرا.
image-location = الموقع
image-remove-location = إزالة معلومات الموقع
image-no-location = لا توجد معلومات عن الموقع.
image-keywords-description = الكلمات الرئيسية والوصف
image-keywords-hint = كلمات رئيسية، مفصولة بفواصل
image-description = الوصف
image-keywords-unsupported = يمكن حفظ الكلمات الرئيسية في ملفات JPEG وPNG وWebP.
# Heading over the earlier versions of the file.
image-revert-to = الرجوع إلى
image-no-versions = لا توجد إصدارات سابقة.
image-revert = رجوع
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = هل تريد الإغلاق دون تصدير التوصيف؟
image-close-body = { $count ->
    [zero] لا يبقى التوصيف على الصور إلا ما دامت نافذتها مفتوحة. صدّر كل صورة للاحتفاظ به: يُرسم التوصيف في النسخة التي تحفظها.
    [one] لا يبقى التوصيف على الصورة إلا ما دامت نافذتها مفتوحة. صدّر الصورة للاحتفاظ به: يُرسم التوصيف في النسخة التي تحفظها.
    [two] لا يبقى التوصيف على الصورتين إلا ما دامت نافذتهما مفتوحة. صدّر كل صورة للاحتفاظ به: يُرسم التوصيف في النسخة التي تحفظها.
    [few] لا يبقى التوصيف على الصور إلا ما دامت نافذتها مفتوحة. صدّر كل صورة للاحتفاظ به: يُرسم التوصيف في النسخة التي تحفظها.
    [many] لا يبقى التوصيف على الصور إلا ما دامت نافذتها مفتوحة. صدّر كل صورة للاحتفاظ به: يُرسم التوصيف في النسخة التي تحفظها.
   *[other] لا يبقى التوصيف على الصور إلا ما دامت نافذتها مفتوحة. صدّر كل صورة للاحتفاظ به: يُرسم التوصيف في النسخة التي تحفظها.
}
image-close-anyway = الإغلاق على أي حال

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = توقفت القراءة
markdown-read-failed = لا يمكن لـ prev قراءة هذا الملف
markdown-draw-failed = تعذّر رسم المستند
# Under the export's size choices.
markdown-export-size = المستند بأكمله، { $width } × { $height } بكسل
# Search results.
markdown-not-found = لم يُعثر على نتائج
markdown-match = { $current } من { $total }
# Placeholder of the search field.
markdown-search = بحث
# Toolbar tooltips.
markdown-smaller-text = نص أصغر
markdown-larger-text = نص أكبر
markdown-zoom = { $percent }%
markdown-actual-size = الحجم الفعلي
# Tooltip and panel title.
markdown-inspector = المراقب
markdown-export = تصدير
markdown-settings = الإعدادات
# Inspector headings and labels.
markdown-file = الملف
markdown-document = المستند
markdown-words = الكلمات
markdown-lines = الأسطر
markdown-pictures = الصور

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = الكاميرا
image-meta-exposure = التعريض
image-meta-image = الصورة
image-meta-make = الشركة المصنّعة
image-meta-model = الطراز
image-meta-lens = العدسة
image-meta-exposure-time = مدة التعريض
# The lens aperture, written like f/2.8.
image-meta-f-number = الرقم البؤري
image-meta-iso = ISO
image-meta-focal-length = البعد البؤري
image-meta-exposure-bias = تعويض التعريض
image-meta-flash = الفلاش
image-meta-date-taken = تاريخ الالتقاط
image-meta-orientation = الاتجاه
image-meta-color-space = مساحة الألوان
image-meta-software = البرنامج
image-meta-artist = الفنان
image-meta-copyright = حقوق النشر
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } ث
image-meta-millimeters = { $value } مم
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] عادي
    [2] معكوس أفقيًا
    [3] مُدار 180°
    [4] معكوس عموديًا
    [5] معكوس أفقيًا، ومُدار 90° عكس اتجاه عقارب الساعة
    [6] مُدار 90° في اتجاه عقارب الساعة
    [7] معكوس أفقيًا، ومُدار 90° في اتجاه عقارب الساعة
    [8] مُدار 90° عكس اتجاه عقارب الساعة
   *[other] غير معروف ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] انطلق
   *[no] لم ينطلق
}{ $mode ->
    [on] ، تشغيل إجباري
    [off] ، متوقف
    [auto] ، تلقائي
   *[unknown] {""}
}{ $redeye ->
    [yes] ، تقليل احمرار العين
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] غير معايَر
   *[other] أخرى ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = لا يمكن فتح المستند: { $detail }
error-pdf-page-out-of-range = الصفحة { $page } غير موجودة
error-pdf-password-protected = المستند محمي بكلمة سر؛ افتحه وانسخ صفحاته بدلًا من ذلك
error-pdf-no-pages = لا توجد صفحات لاستخراجها
error-pdf-crop-outside = منطقة الاقتصاص خارج الصفحة
error-pdf-closed = أُغلق المستند
error-pdf-saved-unreadable = لم يعد المستند المحفوظ يُفتح
error-image-read = لا يمكن قراءة الملف: { $detail }
error-image-invalid = الصورة تالفة أو غير صالحة: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = يتطلب فتح هذا التنسيق { $library }، وهو غير مثبّت
# $format is an image format name, such as HEIC.
error-image-unsupported = صور { $format } غير مدعومة بعد
error-image-encode = لا يمكن ترميز الصورة: { $detail }
error-exif-malformed = بيانات EXIF تالفة
error-settings-read = لا يمكن قراءة الإعدادات: { $detail }
error-settings-invalid = إعدادات غير صالحة: { $detail }
error-remove-location = تعذّرت إزالة الموقع: { $error }
error-location-unsupported = يمكن إزالة معلومات الموقع من ملفات JPEG وPNG وWebP وTIFF
error-xmp-unsupported = لا يمكن حفظ الكلمات الرئيسية والأوصاف إلا في ملفات JPEG وPNG وWebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW للكاميرا

## The macOS menu bar, named as in macOS's own apps.
menu-about = حول prev
menu-settings = الإعدادات…
menu-services = الخدمات
menu-hide = إخفاء prev
menu-hide-others = إخفاء الآخرين
menu-show-all = إظهار الكل
menu-quit = إنهاء prev
menu-file = ملف
menu-open = فتح…
menu-close = إغلاق النافذة
menu-export = تصدير…
menu-print = طباعة…
menu-edit = تحرير
menu-undo = تراجع
menu-redo = إعادة
menu-cut = قص
menu-copy = نسخ
menu-paste = لصق
menu-select-all = تحديد الكل
menu-find = بحث
menu-find-next = بحث عن التالي
menu-find-previous = بحث عن السابق
menu-view = عرض
menu-hide-sidebar = إخفاء الشريط الجانبي
menu-thumbnails = صور مصغرة
menu-contents = جدول المحتويات
menu-notes = التمييزات والملاحظات
menu-bookmarks = الإشارات المرجعية
menu-zoom-in = تكبير
menu-zoom-out = تصغير
menu-actual-size = الحجم الفعلي
menu-zoom-to-fit = تكبير/تصغير للملاءمة
menu-inspector = إظهار المراقب
menu-slideshow = عرض الشرائح
menu-full-screen = الدخول إلى ملء الشاشة
menu-go = انتقال
menu-next-page = الصفحة التالية
menu-previous-page = الصفحة السابقة
menu-go-to-page = انتقال إلى صفحة…
menu-bookmark = إضافة إشارة مرجعية
menu-tools = أدوات
menu-markup = إظهار شريط أدوات التوصيف
menu-rotate-left = تدوير إلى اليسار
menu-rotate-right = تدوير إلى اليمين
menu-crop = اقتصاص
menu-adjust-color = ضبط اللون…
menu-window = نافذة
menu-minimize = تصغير
menu-zoom = تكبير/تصغير
menu-bring-all-to-front = إحضار الكل إلى الأمام

## Outside control

settings-outside-control = التحكم الخارجي
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = عام
settings-tab-agents = الوكلاء
settings-allow-outside-control = السماح بالتحكم الخارجي
settings-allow-outside-control-note = يمكن لوكلاء الذكاء الاصطناعي مثل Claude Code قراءة ملفاتك في prev وتغييرها، عبر prev --mcp. يسأل prev قبل كل وكيل جديد.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = المسموح لهم: { $agents }
settings-forget-agents = نسيان
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = هل تسمح لـ { $agent } بالتحكم في prev؟
agent-prompt-body = يطلب { $agent } استخدام التحكم الخارجي في prev، لقراءة ملفاتك المفتوحة وتغييرها. يمكنك إيقاف التحكم الخارجي من الإعدادات.
agent-prompt-allow = سماح
agent-prompt-deny = عدم السماح
settings-ask-before-note = السؤال قبل أن يقوم وكيل بما يلي:
settings-ask-reading = قراءة ملف
settings-ask-viewing = تغيير العرض أو نافذة
settings-ask-marking-up = إضافة توصيف إلى ملف
settings-ask-editing = تحرير ملف
settings-ask-signing = توقيع ملف
settings-ask-redacting = تطبيق التنقيحات
settings-ask-exporting = تصدير ملف
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = هل تسمح لـ { $agent } بقراءة هذا الملف؟
agent-ask-view = هل تسمح لـ { $agent } بتغيير العرض؟
agent-ask-markup = هل تسمح لـ { $agent } بإضافة توصيف إلى هذا الملف؟
agent-ask-edit = هل تسمح لـ { $agent } بتحرير هذا الملف؟
agent-ask-sign = هل تسمح لـ { $agent } بتوقيع هذا الملف؟
agent-ask-redact = هل تسمح لـ { $agent } بتطبيق التنقيحات؟
agent-ask-export = هل تسمح لـ { $agent } بتصدير هذا الملف؟
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = يطلب { $agent } استخدام “{ $tool }”. تحدد الإعدادات ما يسأل عنه prev.
agent-ask-final = لا يمكن التراجع عن هذا.
