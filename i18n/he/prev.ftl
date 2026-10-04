# prev's interface text in Hebrew (עברית), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = סימון, note = פתק, highlight = הדגשה,
# annotation = הערה, redact/redaction = השחרה, inspector = פרטים,
# zoom in/out = הגדלה/הקטנה, bookmark = סימנייה, page = עמוד.
# Buttons and menu items use the action noun (שמירה, ביטול, סגירה).

## Language

language-name = עברית

## Common

common-cancel = ביטול
common-close = סגירה
common-save = שמירה

## Settings

settings-title = הגדרות
settings-appearance = מראה
settings-colors = צבעים
settings-windows = חלונות
settings-default-app = אפליקציית ברירת מחדל
settings-default-app-label = פתיחת קבצים ב-prev
settings-default-app-note = להפוך את prev לאפליקציה שפותחת קובצי PDF, תמונות, ציורי SVG וקובצי Markdown.
settings-default-app-note-windows = ב-Windows אפשר לבחור אפליקציות ברירת מחדל רק בהגדרות שלה. הכפתור פותח שם את הדף של prev.
settings-default-app-note-macos = macOS מבקשת לאשר כל סוג: PDF,‏ PNG,‏ JPEG,‏ HEIC,‏ GIF,‏ TIFF,‏ WebP ו-AVIF.
settings-default-app-status = { $set } מתוך { $total } סוגי קבצים נפתחים ב-prev.
settings-default-app-button = הגדרה כברירת מחדל
settings-default-app-button-windows = פתיחת ההגדרות
settings-default-app-no-entry = רשומת שולחן העבודה של prev לא מותקנת, ולכן המערכת לא יכולה לפתוח בה קבצים. אפשר להתקין את prev מחבילה או באמצעות scripts/install.sh.
settings-default-app-no-bundle = כדי להגדיר את prev כברירת מחדל, יש לפתוח אותה מ-prev.app.
settings-default-app-failed = לא ניתן להגדיר את prev כברירת מחדל: { $error }
settings-storage = אחסון
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (גרסת פיתוח, { $build })

## Markup toolbar

markup-tool-select = בחירה
markup-tool-area = בחירה מלבנית
markup-tool-sketch = סקיצה
markup-tool-draw = ציור
markup-tool-shapes = צורות
markup-tool-text-box = תיבת טקסט
markup-tool-highlight = הדגשה
markup-tool-note = פתק
markup-tool-sign = חתימה
markup-tool-redact = השחרה
markup-apply = החלה
markup-apply-redactions = החלת השחרות
markup-shape-style = סגנון צורה
markup-border-color = צבע מסגרת
markup-fill-color = צבע מילוי
markup-text-style = סגנון טקסט
markup-delete = מחיקה
markup-undo = ביטול פעולה
markup-redo = ביצוע חוזר

## Markup menus

markup-shape-rectangle = מלבן
markup-shape-rounded-rectangle = מלבן מעוגל
markup-shape-oval = אליפסה
markup-shape-line = קו
markup-shape-arrow = חץ
markup-shape-star = כוכב
markup-shape-polygon = מצולע
markup-shape-speech-bubble = בועת דיבור
markup-shape-loupe = זכוכית מגדלת
markup-shape-mask = מסכה
markup-style-highlight = הדגשה
markup-style-underline = קו תחתון
markup-style-strikethrough = קו חוצה
markup-style-squiggly = קו גלי
markup-menu-color = צבע
markup-menu-font = גופן
markup-menu-size = גודל
markup-menu-alignment = יישור
markup-line-width = { $width } נק׳
markup-dashed = מקווקו

## Notes

markup-kind-note = פתק
markup-kind-text-box = תיבת טקסט
markup-kind-stamp = חותמת
markup-kind-redaction = השחרה
markup-kind-shape = צורה
markup-note-delete = מחיקת הפתק
markup-note-done = סיום
markup-note-placeholder = כאן כותבים פתק
markup-notes-empty = אין הדגשות או פתקים
markup-notes-empty-hint = הדגשות, פתקים ותיבות טקסט יופיעו כאן.
markup-notes-page = עמוד { $page }

## Markup errors

markup-change-failed = לא ניתן לשנות את המסמך: { $error }
markup-copy-area-failed = לא ניתן להעתיק את האזור: { $error }
markup-document-closed = המסמך נסגר
markup-render-area-failed = לא ניתן לעבד את האזור
markup-copy-stopped = ההעתקה הופסקה

## Signatures

signature-menu-empty = עדיין אין חתימות.
signature-delete = מחיקת החתימה
signature-create = יצירת חתימה…
signature-dialog-title = יצירת חתימה
signature-tab-draw = ציור
signature-tab-type = הקלדה
signature-tab-image = תמונה
signature-draw-hint = אפשר לחתום על הקו בעכבר, בעט או במשטח המגע.
signature-your-name = השם שלך
signature-image-hint = יש לבחור תמונה או סריקה של החתימה על דף לבן.
signature-choose-image = בחירת תמונה…
signature-description = תיאור, למשל שם מלא או ראשי תיבות
signature-clear = ניקוי
signature-ink = דיו
signature-thickness = עובי
signature-sign-first = קודם חותמים, ואז שומרים.
signature-default-name = חתימה { $number }
signature-change-failed = לא ניתן לשנות את החתימות: { $error }
signature-no-data-folder = אין תיקיית נתונים: HOME לא מוגדר
signature-removing-stopped = ההסרה הופסקה
signature-saving-stopped = השמירה הופסקה
signature-reading-stopped = הקריאה הופסקה
signature-not-an-image = הקובץ אינו תמונה ש-prev יכול לקרוא
signature-no-frames = אין בתמונה פריימים
signature-not-found = לא נמצאה חתימה בתמונה

## Dragging

drag-pages-need-document = אפשר לשחרר עמודים רק על מסמך.
drag-image-unsupported = prev לא יכול לפתוח את התמונה הזו.
drag-area-failed = לא ניתן לגרור את האזור: { $error }
drag-pages-failed = לא ניתן לגרור את העמודים: { $error }
drag-start-failed = לא ניתן להתחיל בגרירה.
drag-file-pages = עמודים
drag-file-one-page = { $name } (עמוד { $page })
drag-file-page-range = { $name } (עמודים { $first }–{ $last })
drag-file-image = תמונה
drop-pdf-title = להוסיף למסמך הזה?
drop-pdf-body = להוסיף את "{ $name }" לסוף המסמך הזה, או לפתוח אותו בחלון נפרד?
drop-pdfs-body = { $count ->
    [one] להוסיף את קובץ ה-PDF הזה לסוף המסמך הזה, או לפתוח אותו בחלון נפרד?
    [two] להוסיף את שני קובצי ה-PDF האלה לסוף המסמך הזה, או לפתוח אותם בחלונות נפרדים?
   *[other] להוסיף את { $count } קובצי ה-PDF האלה לסוף המסמך הזה, או לפתוח אותם בחלונות נפרדים?
}
drop-pdf-add = הוספה לסוף
drop-pdf-open = פתיחה בנפרד

## PDF window

pdf-opening = בפתיחה…
pdf-open-failed = prev לא יכול לפתוח את המסמך הזה
pdf-no-pages = אין במסמך עמודים.
pdf-document-closed = המסמך נסגר
pdf-keep-original-failed = לא ניתן לשמור את הגרסה המקורית: { $error }
pdf-save-failed = לא ניתן לשמור: { $error }
pdf-nothing-to-paste = אין מה להדביק.
pdf-pasting-stopped = ההדבקה הופסקה
pdf-file-dialog-failed = לא ניתן להציג את חלון בחירת הקבצים: { $error }
pdf-bookmarks-no-home = לא ניתן לשמור סימניות: HOME לא מוגדר
pdf-bookmarks-save-failed = לא ניתן לשמור את הסימניות: { $error }
pdf-bookmark-page = עמוד { $page }

pdf-password-protected = הקובץ "{ $name }" מוגן בסיסמה
pdf-password = סיסמה
pdf-password-wrong = הסיסמה שגויה. אפשר לנסות שוב.
pdf-unlock = ביטול נעילה

pdf-sidebar = סרגל צד
pdf-page-of = מתוך { $count }
pdf-zoom-out = הקטנה
pdf-zoom-in = הגדלה
pdf-zoom-percent = { $percent }%
pdf-fit-page = התאמה לעמוד
pdf-fit-width = התאמה לרוחב
pdf-actual-size = גודל אמיתי
pdf-view-continuous = גלילה רציפה
pdf-view-single-page = עמוד בודד
pdf-view-two-pages = שני עמודים
pdf-undo = ביטול פעולה
pdf-redo = ביצוע חוזר
pdf-rotate-left = סיבוב שמאלה
pdf-rotate-right = סיבוב ימינה
pdf-inspector = פרטים
pdf-markup = סימון
pdf-export = ייצוא
pdf-settings = הגדרות

pdf-search = חיפוש
pdf-search-not-found = לא נמצא
pdf-searching = בחיפוש…
pdf-search-match = { $current } מתוך { $total }
pdf-search-match-more = { $current } מתוך { $total }+

pdf-inspector-file = קובץ
pdf-inspector-document = מסמך
pdf-inspector-pages = עמודים
pdf-inspector-title = כותרת
pdf-inspector-author = מחבר
pdf-inspector-subject = נושא
pdf-inspector-keywords = מילות מפתח
pdf-inspector-created = נוצר
pdf-inspector-modified = שונה
pdf-inspector-application = יישום
pdf-inspector-producer = יוצר ה-PDF
pdf-inspector-version = גרסה
pdf-inspector-security = אבטחה
pdf-inspector-not-encrypted = לא מוצפן
pdf-inspector-encrypted = מוצפן ({ $method })
pdf-inspector-page-count = { $count ->
    [one] עמוד אחד
    [two] שני עמודים
   *[other] { $count } עמודים
}
pdf-inspector-page-size = גודל עמוד
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } מ״מ ({ $width_in } × { $height_in } אינץ׳)
pdf-loading = בטעינה…

pdf-tab-pages = עמודים
pdf-tab-contents = תוכן עניינים
pdf-tab-notes = הדגשות ופתקים
pdf-tab-bookmarks = סימניות
pdf-no-outline = אין תוכן עניינים
pdf-no-outline-detail = למסמך הזה אין תוכן עניינים.
pdf-no-bookmarks = אין סימניות
pdf-no-bookmarks-detail = אפשר להקיש { $keys } כדי לסמן עמוד בסימנייה.
pdf-no-bookmarks-detail-unbound = עמודים שסומנו בסימנייה יופיעו כאן.
pdf-remove-bookmark = הסרת הסימנייה

## Page editing

pages-menu = עמודים
pages-insert-blank = הוספת עמוד ריק
pages-insert-file = הוספה מקובץ…
pages-copy = { $count ->
    [one] העתקת העמוד
   *[other] העתקת העמודים
}
pages-paste = { $count ->
    [one] הדבקת עמוד
    [two] הדבקת שני עמודים
   *[other] הדבקת { $count } עמודים
}
pages-crop = חיתוך לפי הבחירה
pages-select-all = בחירת כל העמודים
pages-delete = { $count ->
    [one] מחיקת העמוד
   *[other] מחיקת העמודים
}
pages-apply-redactions = החלת השחרות…
pages-no-copied = אין עמודים מועתקים להדבקה.
pages-copied = { $count ->
    [one] הועתק עמוד אחד.
    [two] הועתקו שני עמודים.
   *[other] הועתקו { $count } עמודים.
}
pages-copy-failed = לא ניתן להעתיק את העמודים: { $error }
pages-reading-stopped = הקריאה הופסקה
pages-image-unreadable = אינו תמונה ש-prev יכול לקרוא
pages-read-failed = לא ניתן לקרוא את הקובץ: { $error }
pages-at-least-one = במסמך חייב להיות לפחות עמוד אחד.
pages-crop-needs-area = קודם צריך לבחור אזור בכלי הבחירה המלבנית.
pages-change-failed = לא ניתן לשנות את העמודים: { $error }
pages-no-redactions = לא היו השחרות להחלה.
pages-redactions-applied = { $count ->
    [one] הוחלה השחרה אחת.
    [two] הוחלו שתי השחרות.
   *[other] הוחלו { $count } השחרות.
}
pages-forget-versions-failed = לא ניתן למחוק גרסאות קודמות: { $error }
pages-redact-title = להחיל את ההשחרות?
pages-redact-body = { $count ->
    [one] טקסט, תמונות ואיורים שמתחת לסימון יוסרו מהמסמך לצמיתות, והסימון יהפוך למלבן שחור. אי אפשר לבטל את הפעולה, והגרסאות הקודמות של הקובץ ש-prev שומר יימחקו.
   *[other] טקסט, תמונות ואיורים שמתחת ל-{ $count } הסימונים יוסרו מהמסמך לצמיתות, והסימונים יהפכו למלבנים שחורים. אי אפשר לבטל את הפעולה, והגרסאות הקודמות של הקובץ ש-prev שומר יימחקו.
}
pages-redact-apply = החלה

## PDF export

pages-export-title = ייצוא
pages-export-format = פורמט
pages-export-reduce = הקטנת גודל הקובץ (תמונות ב-150 dpi)
pages-export-flatten = שיטוח הערות ושדות טופס
pages-export-flatten-detail = הסימונים והשדות שמולאו יהפכו לחלק מהעמודים ולא יהיה אפשר לערוך אותם עוד. סימוני השחרה שעדיין לא הוחלו לא ייכללו.
pages-export-encrypt = הצפנה עם סיסמה
pages-export-password = סיסמה
pages-export-verify-password = אימות סיסמה
pages-export-resolution = רזולוציה
pages-export-dpi = { $dpi } dpi
pages-export-quality = איכות
pages-export-quality-low = נמוכה
pages-export-quality-medium = בינונית
pages-export-quality-high = גבוהה
pages-export-quality-best = הטובה ביותר
pages-export-one-file = כל העמודים ייכנסו לקובץ אחד.
pages-export-file-per-page = כל עמוד יישמר כקובץ נפרד, ממוספר לפי השם שנבחר.
pages-export-selected-only = { $count ->
    [one] רק העמוד שנבחר
    [two] רק שני העמודים שנבחרו
   *[other] רק { $count } העמודים שנבחרו
}
pages-export-choose = ייצוא…
pages-export-no-password = יש להזין סיסמה.
pages-export-password-mismatch = הסיסמאות לא תואמות.
pages-export-file-name = { $name } (מיוצא)
pages-export-untitled = מסמך
pages-export-same-file = יש לייצא לקובץ חדש; המסמך הזה נשמר מעצמו.
pages-export-exporting = בייצוא של "{ $name }"…
pages-export-done = הקובץ "{ $name }" יוצא.
pages-export-done-images = יוצאו { $count } תמונות.
pages-export-failed = לא ניתן לייצא: { $error }
pages-export-stopped = הייצוא הופסק

## Start window

app-start-hint = אפשר לפתוח או לשחרר כאן קובץ PDF, תמונה, SVG או Markdown.
app-start-open = פתיחה…
app-title-dev = { $title } (פיתוח)
app-viewer-missing = { $kind }: המציג הזה עדיין לא קיים.
app-cannot-open = prev לא יכול לפתוח קבצים מהסוג הזה.
app-cannot-read = prev לא יכול לקרוא את הקובץ: { $error }
app-kind-pdf = מסמך PDF
app-kind-image = תמונת { $format }
app-kind-svg = ציור SVG
app-kind-markdown = מסמך Markdown
app-file-dialog-failed = לא ניתן להציג את חלון בחירת הקבצים: { $error }

## Actions

action-open = פתיחה
action-settings = הגדרות

## Toolbar

app-toolbar-keep-shown = הצגה קבועה של סרגל הכלים
app-toolbar-auto-hide = הסתרת סרגל הכלים כשהסמן יוצא
app-toolbar-more = עוד

## File facts

app-fact-name = שם
app-fact-folder = תיקייה
app-fact-size = גודל
app-fact-modified = שונה
app-size-bytes = { $count } בתים
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = קישור לא תקין { $uri }: { $error }
app-link-open-failed = לא ניתן לפתוח את { $uri }: { $error }
app-paste-needs-wl-clipboard = יש להתקין את wl-clipboard כדי להדביק תמונות
app-copy-needs-wl-clipboard = יש להתקין את wl-clipboard כדי להעתיק תמונות
app-copy-no-pixels = אין באזור פיקסלים
app-copy-no-input = ל-wl-copy אין קלט
app-copy-failed = wl-copy נכשל
app-clipboard-open-failed = לא ניתן לפתוח את הלוח: { $error }
app-copy-image-failed = לא ניתן להעתיק את התמונה: { $error }

## Printing

print-failed = לא ניתן להדפיס: { $error }
print-stopped = ההדפסה הופסקה
print-unavailable = הדפסה עדיין לא זמינה במערכת הזו.
print-no-window = לא ניתן להדפיס: אין חלון שמעליו אפשר להציג את חלון ההדפסה
print-dialog-failed = לא ניתן להציג את חלון ההדפסה: { $error }
print-job-not-started = המדפסת לא התחילה את העבודה
print-printer-stopped = המדפסת הפסיקה

## File dialogs

dialog-open = פתיחה
dialog-filter-all = כל הקבצים הנתמכים
dialog-filter-pdf = מסמכי PDF
dialog-filter-images = תמונות
dialog-filter-svg = ציורי SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = בחירת תיקיית החתימות
dialog-choose-versions = בחירת תיקיית היסטוריית הגרסאות
dialog-choose-bookmarks = בחירת קובץ הסימניות

## Command line

usage-help =
    שימוש: prev [FILE]...
           prev --mcp

    הצגה ועריכה של קובצי PDF ותמונות. הקבצים נפתחים בחלונות של prev
    שכבר פועל, והוא מופעל אם צריך.

    אפשרויות:
      -h, --help     הצגת העזרה הזו
      -V, --version  הצגת הגרסה
          --mcp      הגשת MCP דרך stdin ו-stdout, כדי שסוכני AI ישלטו
                     ב-prev שכבר פועל

## Settings, continued

settings-language = שפה
settings-language-system = ברירת המחדל של המערכת: { $language }
settings-input-language = שפת הקלדה
settings-input-language-system = לפי פריסת המקלדת
settings-input-language-note = קובעת מאיזה צד מתחיל שדה טקסט ריק. טקסט שמקלידים נשאר בכיוון שלו.

settings-appearance-system = מערכת
settings-appearance-light = בהיר
settings-appearance-dark = כהה
settings-system-accent = שימוש בצבע ההדגשה של המערכת
settings-omarchy-note = הצבעים נבנים מצבע ההדגשה של "{ $theme }".
settings-system-accent-note = הצבעים נבנים מצבע ההדגשה של המערכת.
settings-system-accent-none = למערכת אין צבע הדגשה, ולכן prev משתמש בצבע שנבחר למטה.
settings-accent-chosen-note = הצבעים נבנים מהצבע שנבחר למטה.
settings-auto-hide = הסתרת סרגל הכלים כשהסמן יוצא
settings-auto-hide-note = סרגל הכלים צף מעל המסמך ומחליק הצידה כשהסמן מחוץ לחלון.
settings-animations = אנימציות
settings-animations-note = סרגלים וחלוניות שמחליקים, חלונות דו-שיח שגדלים וכפתורים קפיציים.
settings-animations-reduced = כבויות כשהמערכת מבקשת להפחית תנועה.
settings-corner-radius = רדיוס פינות
settings-corner-radius-note = לחלונות דו-שיח ולסרגל הכלים הצף.
settings-corner-radius-value = { $radius } פיקסלים
settings-overlay = שקיפות השכבה העליונה
settings-overlay-note = כמה מהעמוד נראה דרך סרגל הכלים הצף.
settings-overlay-value = { $percent }%
settings-storage-signatures = תיקיית החתימות
settings-storage-versions = תיקיית היסטוריית הגרסאות
settings-storage-bookmarks = קובץ הסימניות
settings-storage-apply = החלה
settings-storage-choose = בחירה…
settings-storage-note = קבצים שכבר נשמרו במיקום הישן נשארים שם; כדי להמשיך להשתמש בהם צריך להעביר אותם. הגדרות האפליקציה prev נשמרות ב-{ $file }.
settings-save-failed = לא ניתן לשמור את ההגדרות: { $error }
settings-no-location = אין מיקום להגדרות: HOME לא מוגדר
settings-full-path = יש להשתמש בנתיב מלא, למשל ~/Documents/prev.
settings-path-is-folder = { $path } הוא תיקייה, לא קובץ.
settings-folder-missing = התיקייה { $path } לא קיימת. אפשר ליצור אותה קודם, או לבחור תיקייה אחרת.
settings-path-is-file = { $path } הוא קובץ, לא תיקייה.
settings-cannot-write = prev לא יכול לכתוב ב-{ $path }: { $error }.

## Export dialog

export-title = ייצוא
export-format = פורמט
export-quality = איכות
export-size = גודל
export-choose = ייצוא…
export-format-webp = WebP (ללא אובדן)
export-format-unknown = תמונה
export-quality-low = נמוכה
export-quality-medium = בינונית
export-quality-high = גבוהה
export-quality-best = הטובה ביותר
export-size-actual = גודל אמיתי
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } פיקסלים
export-dialog-failed = לא ניתן להציג את חלון השמירה: { $error }
export-done = יוצא אל { $path }
export-failed = לא ניתן לייצא: { $error }
export-stopped = הייצוא הופסק

## Image window

image-marked-no-edit = אי אפשר לערוך תמונות עם סימונים. כדי לשמור את הסימונים יש לייצא את התמונה, או למחוק אותם ולסגור את סרגל הסימון.

image-loading-stopped = הטעינה הופסקה
image-reverting-stopped = השחזור הופסק
image-rendering-stopped = העיבוד הופסק
image-saving-stopped = השמירה הופסקה
image-markup-stopped = הסימון הופסק
image-no-version-store = אין מקום לשמירת גרסאות
image-revert-failed = לא ניתן לשחזר: { $error }
image-read-failed = לא ניתן לקרוא את { $path }: { $error }
image-keep-original-failed = לא ניתן לשמור את הגרסה המקורית: { $error }
image-save-failed = לא ניתן לשמור את { $path }: { $error }
image-markup-start-failed = לא ניתן להתחיל בסימון: { $error }
image-cannot-edit = אי אפשר לערוך אנימציות וציורי SVG.
image-cannot-mark-up = אי אפשר להוסיף סימונים לאנימציות ולציורי SVG.
image-mark-up-wait = יש להמתין לסיום העריכה, ואז לסמן.
image-crop-needs-selection = קודם צריך לגרור כדי לבחור אזור (בכלי הבחירה), ואז לחתוך.
image-size-needed = יש להזין רוחב וגובה בפיקסלים.
image-cannot-save-format = אי אפשר לשמור את השינויים ב-"{ $name }" בפורמט שלו. אפשר להשתמש בייצוא ({ $keys }).
image-cannot-save-format-unbound = אי אפשר לשמור את השינויים ב-"{ $name }" בפורמט שלו. אפשר להשתמש בייצוא.
image-cannot-export-animation = עדיין אי אפשר לייצא אנימציות.
image-drop-pages = אפשר לשחרר עמודים רק על מסמך.
image-drag-failed = לא ניתן להתחיל בגרירה.
image-picture-save-failed = לא ניתן לשמור את התמונה בתיקיית ההורדות.
image-open-failed = prev לא יכול לפתוח את התמונה הזו
image-opening = בפתיחה…
image-name-mismatch-title = השם לא תואם לפורמט
image-name-mismatch = "{ $name }" יישמר כקובץ { $format }, אבל השם שלו מסתיים ב-.{ $extension }. ייתכן שאפליקציות אחרות לא יפתחו אותו.
image-name-mismatch-no-extension = "{ $name }" יישמר כקובץ { $format }, אבל לשם שלו אין סיומת. ייתכן שאפליקציות אחרות לא יפתחו אותו.
image-choose-again = בחירה מחדש
image-save-as-is = שמירה כמו שהוא
image-dimensions = { $width } × { $height }
image-frame-position = פריים { $current } מתוך { $total }
image-position = { $current } מתוך { $total }
image-edited = נערך
image-sidebar = סרגל צד
image-zoom-out = הקטנה
image-zoom-in = הגדלה
image-zoom = { $percent }%
image-fit = התאמה לחלון
image-actual-size = גודל אמיתי
image-undo = ביטול פעולה
image-redo = ביצוע חוזר
image-rotate-left = סיבוב שמאלה
image-rotate-right = סיבוב ימינה
image-flip-horizontal = היפוך אופקי
image-flip-vertical = היפוך אנכי
image-select = בחירה מלבנית
image-crop = חיתוך לפי הבחירה
image-adjust-size-tool = שינוי גודל
image-adjust-color-tool = כוונון צבע
image-inspector = פרטים
image-markup = סימון
image-export = ייצוא
image-settings = הגדרות
image-adjust-color = כוונון צבע
image-adjust-size = שינוי גודל
image-exposure = חשיפה
image-contrast = ניגודיות
image-saturation = רוויה
image-temperature = טמפרטורה
image-tint = גוון
image-sepia = ספיה
image-sharpness = חדות
image-levels = רמות
image-black-point = נקודת שחור
image-midtones = גוונים אמצעיים
image-white-point = נקודת לבן
image-reset-all = איפוס הכול
image-current-size = הגודל הנוכחי: { $width } × { $height } פיקסלים
image-width = רוחב
image-height = גובה
image-scale-proportionally = שינוי גודל יחסי
image-resize = שינוי גודל
image-inspector-loading = בטעינה…
image-file = קובץ
image-format = פורמט
image-dimensions-label = ממדים
image-pixels = { $width } × { $height } פיקסלים
image-no-camera = אין מידע על המצלמה.
image-location = מיקום
image-remove-location = הסרת פרטי המיקום
image-no-location = אין מידע על המיקום.
image-keywords-description = מילות מפתח ותיאור
image-keywords-hint = מילות מפתח, מופרדות בפסיקים
image-description = תיאור
image-keywords-unsupported = אפשר לשמור מילות מפתח בקובצי JPEG, PNG ו-WebP.
image-revert-to = שחזור לגרסה
image-no-versions = אין גרסאות קודמות.
image-revert = שחזור
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = לסגור בלי לייצא את הסימונים?
image-close-body = { $count ->
    [one] סימונים על תמונה נשמרים רק כל עוד החלון שלה פתוח. כדי לשמור אותם יש לייצא את התמונה: הסימונים יצוירו בעותק שנשמר.
   *[other] סימונים על תמונות נשמרים רק כל עוד החלון שלהן פתוח. כדי לשמור אותם יש לייצא כל תמונה: הסימונים יצוירו בעותק שנשמר.
}
image-close-anyway = סגירה בכל זאת

## Markdown

markdown-reading-stopped = הקריאה הופסקה
markdown-read-failed = prev לא יכול לקרוא את הקובץ הזה
markdown-draw-failed = לא ניתן לצייר את המסמך
markdown-export-size = המסמך כולו, { $width } × { $height } פיקסלים
markdown-not-found = לא נמצא
markdown-match = { $current } מתוך { $total }
markdown-search = חיפוש
markdown-smaller-text = טקסט קטן יותר
markdown-larger-text = טקסט גדול יותר
markdown-zoom = { $percent }%
markdown-actual-size = גודל אמיתי
markdown-limit-width = הגבלת רוחב הטקסט
markdown-inspector = פרטים
markdown-export = ייצוא
markdown-settings = הגדרות
markdown-file = קובץ
markdown-document = מסמך
markdown-words = מילים
markdown-lines = שורות
markdown-pictures = תמונות

## Image details

image-meta-camera = מצלמה
image-meta-exposure = חשיפה
image-meta-image = תמונה
image-meta-make = יצרן
image-meta-model = דגם
image-meta-lens = עדשה
image-meta-exposure-time = זמן חשיפה
image-meta-f-number = מספר f
image-meta-iso = ISO
image-meta-focal-length = אורך מוקד
image-meta-exposure-bias = פיצוי חשיפה
image-meta-flash = מבזק
image-meta-date-taken = תאריך צילום
image-meta-orientation = כיוון
image-meta-color-space = מרחב צבע
image-meta-software = תוכנה
image-meta-artist = צלם
image-meta-copyright = זכויות יוצרים
image-meta-seconds = { $value } ש׳
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] רגיל
    [2] שיקוף אופקי
    [3] סיבוב ב-180°
    [4] שיקוף אנכי
    [5] שיקוף אופקי, סיבוב ב-90° נגד כיוון השעון
    [6] סיבוב ב-90° בכיוון השעון
    [7] שיקוף אופקי, סיבוב ב-90° בכיוון השעון
    [8] סיבוב ב-90° נגד כיוון השעון
   *[other] לא ידוע ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] הופעל
   *[no] לא הופעל
}{ $mode ->
    [on] , מאולץ
    [off] , כבוי
    [auto] , אוטומטי
   *[unknown] {""}
}{ $redeye ->
    [yes] , הפחתת עיניים אדומות
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] לא מכויל
   *[other] אחר ({ $code })
}

## Errors

error-pdf-open = לא ניתן לפתוח את המסמך: { $detail }
error-pdf-page-out-of-range = עמוד { $page } לא קיים
error-pdf-password-protected = המסמך מוגן בסיסמה; יש לפתוח אותו ולהעתיק ממנו את העמודים
error-pdf-no-pages = אין עמודים לחילוץ
error-pdf-crop-outside = אזור החיתוך נמצא מחוץ לעמוד
error-pdf-closed = המסמך נסגר
error-pdf-saved-unreadable = המסמך שנשמר כבר לא נפתח
error-image-read = לא ניתן לקרוא את הקובץ: { $detail }
error-image-invalid = התמונה פגומה או לא תקינה: { $detail }
error-image-missing-library = פתיחת הפורמט הזה דורשת את { $library }, שאינו מותקן
error-image-unsupported = תמונות { $format } עדיין לא נתמכות
error-image-encode = לא ניתן לקודד את התמונה: { $detail }
error-exif-malformed = נתוני ה-EXIF פגומים
error-settings-read = לא ניתן לקרוא את ההגדרות: { $detail }
error-settings-invalid = הגדרות לא תקינות: { $detail }
error-remove-location = לא ניתן להסיר את המיקום: { $error }
error-location-unsupported = אפשר להסיר פרטי מיקום רק מקובצי JPEG, PNG, WebP ו-TIFF
error-xmp-unsupported = אפשר לשמור מילות מפתח ותיאורים רק בקובצי JPEG, PNG ו-WebP

## Formats

format-camera-raw = Camera RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = אודות prev
menu-settings = הגדרות…
menu-services = שירותים
menu-hide = הסתרת prev
menu-hide-others = הסתרת האחרים
menu-show-all = הצגת הכול
menu-quit = יציאה מ-prev
menu-file = קובץ
menu-open = פתיחה…
menu-close = סגירת החלון
menu-export = ייצוא…
menu-print = הדפסה…
menu-edit = עריכה
menu-undo = ביטול
menu-redo = ביצוע חוזר
menu-cut = גזירה
menu-copy = העתקה
menu-paste = הדבקה
menu-select-all = בחירת הכול
menu-find = חיפוש
menu-find-next = חיפוש הבא
menu-find-previous = חיפוש הקודם
menu-view = תצוגה
menu-hide-sidebar = הסתרת סרגל הצד
menu-thumbnails = תמונות ממוזערות
menu-contents = תוכן העניינים
menu-notes = הדגשות והערות
menu-bookmarks = סימניות
menu-zoom-in = הגדלה
menu-zoom-out = הקטנה
menu-actual-size = גודל אמיתי
menu-zoom-to-fit = התאמה לחלון
menu-inspector = הצגת החלונית פרטים
menu-slideshow = מצגת
menu-full-screen = כניסה למסך מלא
menu-go = מעבר
menu-next-page = העמוד הבא
menu-previous-page = העמוד הקודם
menu-go-to-page = מעבר לעמוד…
menu-bookmark = הוספת סימנייה
menu-tools = כלים
menu-markup = הצגת סרגל הסימון
menu-rotate-left = סיבוב שמאלה
menu-rotate-right = סיבוב ימינה
menu-crop = חיתוך
menu-adjust-color = כוונון צבע…
menu-window = חלון
menu-minimize = מזעור
menu-zoom = הגדלת החלון
menu-bring-all-to-front = הבאת הכול לחזית

## Outside control

settings-outside-control = שליטה מבחוץ
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = כללי
settings-tab-agents = סוכנים
settings-allow-outside-control = התרת שליטה מבחוץ
settings-allow-outside-control-note = סוכני AI כמו Claude Code יכולים לקרוא ולשנות קבצים ב-prev, דרך prev --mcp. prev שואל לפני כל סוכן חדש.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = מורשים: { $agents }
settings-forget-agents = הסרה
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = לאפשר ל-{ $agent } לשלוט ב-prev?
agent-prompt-body = { $agent } מבקש להשתמש בשליטה מבחוץ של prev, כדי לקרוא את הקבצים הפתוחים ולשנות אותם. אפשר לכבות את השליטה מבחוץ בהגדרות.
agent-prompt-allow = אישור
agent-prompt-deny = דחייה
settings-ask-before-note = לשאול לפני פעולות אלה של סוכן:
settings-ask-reading = קריאת קובץ
settings-ask-viewing = שינוי התצוגה או חלון
settings-ask-marking-up = הוספת סימונים לקובץ
settings-ask-editing = עריכת קובץ
settings-ask-signing = חתימה על קובץ
settings-ask-redacting = החלת השחרות
settings-ask-exporting = ייצוא קובץ
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = לאפשר ל-{ $agent } לקרוא את הקובץ הזה?
agent-ask-view = לאפשר ל-{ $agent } לשנות את התצוגה?
agent-ask-markup = לאפשר ל-{ $agent } להוסיף סימונים לקובץ הזה?
agent-ask-edit = לאפשר ל-{ $agent } לערוך את הקובץ הזה?
agent-ask-sign = לאפשר ל-{ $agent } לחתום על הקובץ הזה?
agent-ask-redact = לאפשר ל-{ $agent } להחיל השחרות?
agent-ask-export = לאפשר ל-{ $agent } לייצא את הקובץ הזה?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } מבקש להשתמש ב-"{ $tool }". בהגדרות אפשר לבחור על מה prev שואל.
agent-ask-final = אי אפשר לבטל את הפעולה הזו.

## The assistant
settings-tab-assistant = עוזר
settings-assistant-note = המודלים שחלונית העוזר יכולה לדבר איתם. המפתחות נשמרים בצרור המפתחות של המערכת.
settings-assistant-none = אין עדיין מודלים. אפשר להוסיף אחד למטה: מודל מקומי, כמו של Ollama, נשאר במחשב הזה; מודל בענן צריך מפתח API מהספק שלו.
settings-assistant-in-use = בשימוש
settings-assistant-use = שימוש
settings-assistant-remove = הסרה
settings-assistant-add = הוספת מודל
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = שרת תואם OpenAI
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = מודל, למשל { $example }
settings-assistant-key = מפתח API
# $example is an address, such as http://localhost:11434.
settings-assistant-address = כתובת, למשל { $example }
# The menu of how much a local model reads at once.
settings-assistant-context = הקשר
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K טוקנים
settings-assistant-context-note = ערך גבוה יותר מאפשר לעוזר לקרוא חלק גדול יותר מהקובץ בשיחה אחת, אבל המודל צורך יותר זיכרון ועשוי לענות לאט יותר.
settings-assistant-add-button = הוספה
settings-assistant-use-key = המשך
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = יש ליצור מפתח באתר של { $provider } ולהדביק אותו כאן.
settings-assistant-get-key = קבלת מפתח API
settings-assistant-key-kept = המפתח שלך ל-{ $provider } נשמר בצרור המפתחות של המערכת.
settings-assistant-change-key = החלפת המפתח
settings-assistant-key-refused = { $provider } דחה את המפתח הזה. כדאי לבדוק שהוא הועתק במלואו ומהחשבון הנכון.
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } פועל בכתובת { $address }.
settings-assistant-no-server = prev לא מצא { $provider } שפועל במחשב הזה. אפשר להפעיל אותו, או לתת את הכתובת שלו למטה.
settings-assistant-get-server = הורדת { $provider }
settings-assistant-look-again = חיפוש חוזר
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = שימוש בכתובת אחרת
settings-assistant-looking = מחפש מודלים…
settings-assistant-found-none = אין עדיין מודלים ב-{ $provider }. אפשר להוריד מודל דרכו ואז לחפש שוב.
settings-assistant-recommended = מומלץ
settings-assistant-uses-tools = משתמש בכלים
settings-assistant-sees = רואה תמונות
settings-assistant-no-tools = לא יכול להשתמש בכלים, והעוזר זקוק להם
settings-assistant-added-tag = נוסף
settings-assistant-trying = מנסה…
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = פתיחת הדף
# $model is the model's name.
settings-assistant-added = { $model } ענה ונוסף.
settings-assistant-key-needed = המודל הזה צריך מפתח API.
# $error is what the keychain said.
settings-assistant-key-failed = לא ניתן לשמור את המפתח בצרור המפתחות: { $error }
assistant-title = עוזר
assistant-new-chat = שיחה חדשה
assistant-ask = שאלה על הקובץ הזה
assistant-send = שליחה
assistant-stop = עצירה
assistant-thinking = חושב…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = מחשבות
assistant-running = פועל…
assistant-stopped = נעצר.
assistant-no-model = קודם צריך להוסיף מודל בהגדרות.
assistant-add-model = העוזר צריך מודל: מודל בענן עם מפתח ה-API שלו, או מודל מקומי.
assistant-open-settings = הוספת מודל
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = השיחה עכשיו עם { $model }.
assistant-add-another = הוספת מודל…
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } במחשב הזה
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } בכתובת { $host }
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = השיחה כבר גדולה ממה ש-{ $model } יכול לקרוא בבת אחת. אפשר להתחיל שיחה חדשה, או לבחור מודל שיכול לקרוא יותר.
assistant-problem-key = { $provider } דחה את מפתח ה-API. אפשר לבדוק אותו בהגדרות.
assistant-problem-rate = { $provider } מבקש להאט. אפשר לנסות שוב בעוד רגע.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } מבקש להאט: { $message }
assistant-problem-credit = לפי { $provider }, אין בחשבון קרדיט. בחשבון חדש צריך לקנות קרדיט באתר של { $provider } לפני שהמפתח יעבוד; אחר כך אפשר לנסות שוב.
assistant-problem-model = ל-{ $provider } אין מודל בשם { $model }. אפשר לבדוק את השם שלו בהגדרות.
assistant-problem-unavailable = { $provider } עמוס או שיש בו תקלה. אפשר לנסות שוב בעוד רגע.
assistant-problem-unreachable = prev לא הצליח להתחבר ל-{ $provider }. כדאי לבדוק את החיבור, או שהשרת פועל.
assistant-problem-refused = { $model } סירב לענות.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } לא ענה: { $message }
