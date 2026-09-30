# prev's interface text in Hebrew (עברית), translated from i18n/en/prev.ftl.
#
# Machine-assisted translation: it should be reviewed by a native Hebrew
# speaker before release. Keys, section headings and `{ $name }` values
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
settings-storage = אחסון

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

    הצגה ועריכה של קובצי PDF ותמונות. הקבצים נפתחים בחלונות של prev
    שכבר פועל, והוא מופעל אם צריך.

    אפשרויות:
      -h, --help     הצגת העזרה הזו
      -V, --version  הצגת הגרסה

## Settings, continued

settings-language = שפה
settings-language-system = ברירת המחדל של המערכת ({ $language })
settings-input-language = שפת הקלדה
settings-input-language-system = לפי פריסת המקלדת
settings-input-language-note = קובעת מאיזה צד מתחיל שדה טקסט ריק. טקסט שמקלידים נשאר בכיוון שלו.

settings-appearance-system = מערכת
settings-appearance-light = בהיר
settings-appearance-dark = כהה
settings-omarchy-accent = שימוש בצבע ההדגשה של Omarchy
settings-omarchy-note = הצבעים נבנים מצבע ההדגשה של "{ $theme }".
settings-omarchy-none = אין ערכת נושא פעילה של Omarchy.
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
image-cannot-export-animation = עדיין אי אפשר לייצא אנימציות.
image-drop-pages = אפשר לשחרר עמודים רק על מסמך.
image-drag-failed = לא ניתן להתחיל בגרירה.
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
