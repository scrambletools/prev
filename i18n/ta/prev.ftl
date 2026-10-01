# prev's interface text in Tamil (தமிழ்), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = மார்க்அப், note = குறிப்பு, highlight = தனிப்படுத்து/தனிப்படுத்தல்,
# annotation = சிறுகுறிப்பு, redact/redaction = தகவலை மறை/மறைப்பு,
# inspector = ஆய்வி, zoom in/out = பெரிதாக்கு/சிறிதாக்கு, bookmark = புத்தகக்குறி,
# page = பக்கம், document = ஆவணம், image = படம், export = ஏற்றுமதி, crop = செதுக்கு,
# encrypt = மறையாக்கு, system = கணினி, app = பயன்பாடு, default = இயல்புநிலை,
# revert = முந்தைய பதிப்புக்கு மீட்டமை, undo/redo = செயல்தவிர்/மீண்டும் செய்.
# Buttons and menu items use the plain imperative (சேமி, ரத்துசெய், மூடு), as
# Windows and macOS do in Tamil; labels use nouns.

## Language

language-name = தமிழ்

## Common

common-cancel = ரத்துசெய்
common-close = மூடு
common-save = சேமி

## Settings

settings-title = அமைப்புகள்
settings-appearance = தோற்றம்
settings-colors = வண்ணங்கள்
settings-windows = சாளரங்கள்
settings-default-app = இயல்புநிலைப் பயன்பாடு
settings-default-app-label = கோப்புகளை prev மூலம் திற
settings-default-app-note = PDFகள், படங்கள், SVG வரைபடங்கள், Markdown கோப்புகளைத் திறக்கும் பயன்பாடாக prev-ஐ அமைக்கவும்.
settings-default-app-note-windows = Windows அதன் சொந்த அமைப்புகளில் மட்டுமே இயல்புநிலைப் பயன்பாடுகளைத் தேர்வுசெய்ய அனுமதிக்கிறது. இது அங்கே prev-இன் பக்கத்தைத் திறக்கும்.
settings-default-app-note-macos = macOS ஒவ்வொரு வகையையும் உறுதிப்படுத்தக் கேட்கும்: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP, AVIF.
settings-default-app-status = { $total } கோப்பு வகைகளில் { $set } prev மூலம் திறக்கின்றன.
settings-default-app-button = இயல்புநிலையாக்கு
settings-default-app-button-windows = அமைப்புகளைத் திற
settings-default-app-no-entry = prev-இன் டெஸ்க்டாப் உள்ளீடு நிறுவப்படவில்லை, எனவே கணினியால் அதைக் கொண்டு கோப்புகளைத் திறக்க முடியாது. prev-ஐ ஒரு தொகுப்பிலிருந்தோ scripts/install.sh மூலமோ நிறுவவும்.
settings-default-app-no-bundle = prev-ஐ இயல்புநிலையாக்க, அதை prev.app-இலிருந்து திறக்கவும்.
settings-default-app-failed = prev-ஐ இயல்புநிலையாக்க முடியவில்லை: { $error }
settings-storage = சேமிப்பிடம்
settings-version = prev { $version }
settings-version-development = prev { $version } (மேம்பாட்டுப் பதிப்பு)

## Markup toolbar

markup-tool-select = தேர்ந்தெடு
markup-tool-area = செவ்வகத் தேர்வு
markup-tool-sketch = ஸ்கெட்ச்
markup-tool-draw = வரை
markup-tool-shapes = வடிவங்கள்
markup-tool-text-box = உரைப் பெட்டி
markup-tool-highlight = தனிப்படுத்து
markup-tool-note = குறிப்பு
markup-tool-sign = கையொப்பமிடு
markup-tool-redact = தகவலை மறை
markup-apply = பயன்படுத்து
markup-apply-redactions = மறைப்புகளைப் பயன்படுத்து
markup-shape-style = வடிவ நடை
markup-border-color = எல்லை வண்ணம்
markup-fill-color = நிரப்பு வண்ணம்
markup-text-style = உரை நடை
markup-delete = நீக்கு
markup-undo = செயல்தவிர்
markup-redo = மீண்டும் செய்

## Markup menus

markup-shape-rectangle = செவ்வகம்
markup-shape-rounded-rectangle = வளைந்த மூலைச் செவ்வகம்
markup-shape-oval = நீள்வட்டம்
markup-shape-line = கோடு
markup-shape-arrow = அம்புக்குறி
markup-shape-star = நட்சத்திரம்
markup-shape-polygon = பலகோணம்
markup-shape-speech-bubble = பேச்சுக் குமிழ்
markup-shape-loupe = உருப்பெருக்கி
markup-shape-mask = மாஸ்க்
markup-style-highlight = தனிப்படுத்தல்
markup-style-underline = அடிக்கோடு
markup-style-strikethrough = குறுக்குக்கோடு
markup-style-squiggly = அலைக்கோடு
markup-menu-color = வண்ணம்
markup-menu-font = எழுத்துரு
markup-menu-size = அளவு
markup-menu-alignment = சீரமைப்பு
markup-line-width = { $width } pt
markup-dashed = துண்டுக்கோடு

## Notes

markup-kind-note = குறிப்பு
markup-kind-text-box = உரைப் பெட்டி
markup-kind-stamp = முத்திரை
markup-kind-redaction = மறைப்பு
markup-kind-shape = வடிவம்
markup-note-delete = குறிப்பை நீக்கு
markup-note-done = முடிந்தது
markup-note-placeholder = குறிப்பைத் தட்டச்சிடவும்
markup-notes-empty = தனிப்படுத்தல்களோ குறிப்புகளோ இல்லை
markup-notes-empty-hint = தனிப்படுத்தல்கள், குறிப்புகள், உரைப் பெட்டிகள் இங்கே தோன்றும்.
markup-notes-page = பக்கம் { $page }

## Markup errors

markup-change-failed = ஆவணத்தை மாற்ற முடியவில்லை: { $error }
markup-copy-area-failed = பகுதியை நகலெடுக்க முடியவில்லை: { $error }
markup-document-closed = ஆவணம் மூடப்பட்டது
markup-render-area-failed = பகுதியை வரைய முடியவில்லை
markup-copy-stopped = நகலெடுத்தல் நின்றுவிட்டது

## Signatures

signature-menu-empty = இன்னும் கையொப்பங்கள் இல்லை.
signature-delete = கையொப்பத்தை நீக்கு
signature-create = கையொப்பத்தை உருவாக்கு…
signature-dialog-title = கையொப்பத்தை உருவாக்கு
signature-tab-draw = வரை
signature-tab-type = தட்டச்சிடு
signature-tab-image = படம்
signature-draw-hint = கோட்டின் மேல் உங்கள் மவுஸ், பேனா அல்லது டச்பேட் மூலம் கையொப்பமிடவும்.
signature-your-name = உங்கள் பெயர்
signature-image-hint = வெள்ளைத் தாளில் இட்ட உங்கள் கையொப்பத்தின் புகைப்படம் அல்லது ஸ்கேனைத் தேர்வுசெய்யவும்.
signature-choose-image = படத்தைத் தேர்வுசெய்…
signature-description = விளக்கம், எ.கா. முழுப் பெயர் அல்லது தலைப்பெழுத்துகள்
signature-clear = அழி
signature-ink = மை
signature-thickness = தடிமன்
signature-sign-first = முதலில் கையொப்பமிட்டு, பிறகு சேமிக்கவும்.
signature-default-name = கையொப்பம் { $number }
signature-change-failed = கையொப்பங்களை மாற்ற முடியவில்லை: { $error }
signature-no-data-folder = தரவுக் கோப்புறை இல்லை: HOME அமைக்கப்படவில்லை
signature-removing-stopped = அகற்றுதல் நின்றுவிட்டது
signature-saving-stopped = சேமித்தல் நின்றுவிட்டது
signature-reading-stopped = படித்தல் நின்றுவிட்டது
signature-not-an-image = அந்தக் கோப்பு prev படிக்கக்கூடிய படம் அல்ல
signature-no-frames = படத்தில் சட்டகங்கள் இல்லை
signature-not-found = படத்தில் கையொப்பம் எதுவும் கிடைக்கவில்லை

## Dragging

drag-pages-need-document = பக்கங்களை ஓர் ஆவணத்தின் மீது மட்டுமே விட முடியும்.
drag-image-unsupported = prev-ஆல் இந்தப் படத்தைத் திறக்க முடியாது.
drag-area-failed = பகுதியை இழுக்க முடியவில்லை: { $error }
drag-pages-failed = பக்கங்களை இழுக்க முடியவில்லை: { $error }
drag-start-failed = இழுக்கத் தொடங்க முடியவில்லை.
drag-file-pages = பக்கங்கள்
drag-file-one-page = { $name } (பக்கம் { $page })
drag-file-page-range = { $name } (பக்கங்கள் { $first }–{ $last })
drag-file-image = படம்
drop-pdf-title = இந்த ஆவணத்தில் சேர்க்கவா?
drop-pdf-body = “{ $name }” ஐ இந்த ஆவணத்தின் இறுதியில் சேர்க்கவா, அல்லது அதைத் தனிச் சாளரத்தில் திறக்கவா?
drop-pdfs-body = { $count ->
    [one] இந்த PDF-ஐ இந்த ஆவணத்தின் இறுதியில் சேர்க்கவா, அல்லது அதைத் தனிச் சாளரத்தில் திறக்கவா?
   *[other] இந்த { $count } PDF-களை இந்த ஆவணத்தின் இறுதியில் சேர்க்கவா, அல்லது அவற்றைத் தனித்தனிச் சாளரங்களில் திறக்கவா?
}
drop-pdf-add = இறுதியில் சேர்
drop-pdf-open = தனியாகத் திற

## PDF window

pdf-opening = திறக்கிறது…
pdf-open-failed = prev-ஆல் இந்த ஆவணத்தைத் திறக்க முடியாது
pdf-no-pages = ஆவணத்தில் பக்கங்கள் இல்லை.
pdf-document-closed = ஆவணம் மூடப்பட்டது
pdf-keep-original-failed = அசல் பதிப்பை வைத்திருக்க முடியவில்லை: { $error }
pdf-save-failed = சேமிக்க முடியவில்லை: { $error }
pdf-nothing-to-paste = ஒட்டுவதற்கு எதுவும் இல்லை.
pdf-pasting-stopped = ஒட்டுதல் நின்றுவிட்டது
pdf-file-dialog-failed = கோப்பு உரையாடலைக் காட்ட முடியவில்லை: { $error }
pdf-bookmarks-no-home = புத்தகக்குறிகளைச் சேமிக்க முடியாது: HOME அமைக்கப்படவில்லை
pdf-bookmarks-save-failed = புத்தகக்குறிகளைச் சேமிக்க முடியவில்லை: { $error }
pdf-bookmark-page = பக்கம் { $page }

pdf-password-protected = “{ $name }” கடவுச்சொல்லால் பாதுகாக்கப்பட்டுள்ளது
pdf-password = கடவுச்சொல்
pdf-password-wrong = தவறான கடவுச்சொல். மீண்டும் முயலவும்.
pdf-unlock = பூட்டைத் திற

pdf-sidebar = பக்கப்பட்டி
pdf-page-of = / { $count }
pdf-zoom-out = சிறிதாக்கு
pdf-zoom-in = பெரிதாக்கு
pdf-zoom-percent = { $percent }%
pdf-fit-page = பக்கத்திற்குப் பொருத்து
pdf-fit-width = அகலத்திற்குப் பொருத்து
pdf-actual-size = உண்மையான அளவு
pdf-view-continuous = தொடர் உருட்டல்
pdf-view-single-page = ஒற்றைப் பக்கம்
pdf-view-two-pages = இரண்டு பக்கங்கள்
pdf-undo = செயல்தவிர்
pdf-redo = மீண்டும் செய்
pdf-rotate-left = இடப்புறம் சுழற்று
pdf-rotate-right = வலப்புறம் சுழற்று
pdf-inspector = ஆய்வி
pdf-markup = மார்க்அப்
pdf-export = ஏற்றுமதி செய்
pdf-settings = அமைப்புகள்

pdf-search = தேடு
pdf-search-not-found = கிடைக்கவில்லை
pdf-searching = தேடுகிறது…
pdf-search-match = { $total } இல் { $current }
pdf-search-match-more = { $total }+ இல் { $current }

pdf-inspector-file = கோப்பு
pdf-inspector-document = ஆவணம்
pdf-inspector-pages = பக்கங்கள்
pdf-inspector-title = தலைப்பு
pdf-inspector-author = ஆசிரியர்
pdf-inspector-subject = பொருள்
pdf-inspector-keywords = முக்கியச் சொற்கள்
pdf-inspector-created = உருவாக்கப்பட்டது
pdf-inspector-modified = மாற்றப்பட்டது
pdf-inspector-application = பயன்பாடு
pdf-inspector-producer = PDF தயாரிப்பான்
pdf-inspector-version = பதிப்பு
pdf-inspector-security = பாதுகாப்பு
pdf-inspector-not-encrypted = மறையாக்கம் செய்யப்படவில்லை
pdf-inspector-encrypted = மறையாக்கம் செய்யப்பட்டது ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } பக்கம்
   *[other] { $count } பக்கங்கள்
}
pdf-inspector-page-size = பக்க அளவு
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } மி.மீ. ({ $width_in } × { $height_in } அங்.)
pdf-loading = ஏற்றுகிறது…

pdf-tab-pages = பக்கங்கள்
pdf-tab-contents = உள்ளடக்கம்
pdf-tab-notes = தனிப்படுத்தல்களும் குறிப்புகளும்
pdf-tab-bookmarks = புத்தகக்குறிகள்
pdf-no-outline = உள்ளடக்க அட்டவணை இல்லை
pdf-no-outline-detail = இந்த ஆவணத்தில் வெளிக்கோடு இல்லை.
pdf-no-bookmarks = புத்தகக்குறிகள் இல்லை
pdf-no-bookmarks-detail = ஒரு பக்கத்திற்குப் புத்தகக்குறியிட { $keys } அழுத்தவும்.
pdf-no-bookmarks-detail-unbound = புத்தகக்குறியிட்ட பக்கங்கள் இங்கே தோன்றும்.
pdf-remove-bookmark = புத்தகக்குறியை அகற்று

## Page editing

pages-menu = பக்கங்கள்
pages-insert-blank = வெற்றுப் பக்கத்தைச் செருகு
pages-insert-file = கோப்பிலிருந்து செருகு…
pages-copy = { $count ->
    [one] பக்கத்தை நகலெடு
   *[other] பக்கங்களை நகலெடு
}
pages-paste = { $count ->
    [one] பக்கத்தை ஒட்டு
   *[other] { $count } பக்கங்களை ஒட்டு
}
pages-crop = தேர்வுக்கேற்பச் செதுக்கு
pages-select-all = எல்லாப் பக்கங்களையும் தேர்ந்தெடு
pages-delete = { $count ->
    [one] பக்கத்தை நீக்கு
   *[other] பக்கங்களை நீக்கு
}
pages-apply-redactions = மறைப்புகளைப் பயன்படுத்து…
pages-no-copied = ஒட்டுவதற்கு நகலெடுத்த பக்கங்கள் இல்லை.
pages-copied = { $count ->
    [one] { $count } பக்கம் நகலெடுக்கப்பட்டது.
   *[other] { $count } பக்கங்கள் நகலெடுக்கப்பட்டன.
}
pages-copy-failed = பக்கங்களை நகலெடுக்க முடியவில்லை: { $error }
pages-reading-stopped = படித்தல் நின்றுவிட்டது
pages-image-unreadable = prev படிக்கக்கூடிய படம் அல்ல
pages-read-failed = கோப்பைப் படிக்க முடியவில்லை: { $error }
pages-at-least-one = ஓர் ஆவணத்தில் குறைந்தது ஒரு பக்கமாவது இருக்க வேண்டும்.
pages-crop-needs-area = முதலில் செவ்வகத் தேர்வுக் கருவி மூலம் ஒரு பகுதியைத் தேர்ந்தெடுக்கவும்.
pages-change-failed = பக்கங்களை மாற்ற முடியவில்லை: { $error }
pages-no-redactions = பயன்படுத்த மறைப்புகள் எதுவும் இல்லை.
pages-redactions-applied = { $count ->
    [one] { $count } மறைப்பு பயன்படுத்தப்பட்டது.
   *[other] { $count } மறைப்புகள் பயன்படுத்தப்பட்டன.
}
pages-forget-versions-failed = முந்தைய பதிப்புகளை நீக்க முடியவில்லை: { $error }
pages-redact-title = மறைப்புகளைப் பயன்படுத்தவா?
pages-redact-body = { $count ->
    [one] குறியின் கீழுள்ள உரை, படங்கள், வரைபடங்கள் ஆவணத்திலிருந்து நிரந்தரமாக அகற்றப்படும், குறி கருப்புப் பெட்டியாக மாறும். இதைச் செயல்தவிர்க்க முடியாது, மேலும் prev வைத்திருக்கும் இந்தக் கோப்பின் முந்தைய பதிப்புகள் நீக்கப்படும்.
   *[other] { $count } குறிகளின் கீழுள்ள உரை, படங்கள், வரைபடங்கள் ஆவணத்திலிருந்து நிரந்தரமாக அகற்றப்படும், குறிகள் கருப்புப் பெட்டிகளாக மாறும். இதைச் செயல்தவிர்க்க முடியாது, மேலும் prev வைத்திருக்கும் இந்தக் கோப்பின் முந்தைய பதிப்புகள் நீக்கப்படும்.
}
pages-redact-apply = பயன்படுத்து

## PDF export

pages-export-title = ஏற்றுமதி செய்
pages-export-format = வடிவமைப்பு
pages-export-reduce = கோப்பு அளவைக் குறை (150 dpi இல் படங்கள்)
pages-export-flatten = சிறுகுறிப்புகளையும் படிவப் புலங்களையும் தட்டையாக்கு
pages-export-flatten-detail = மார்க்அப்பும் நிரப்பிய புலங்களும் பக்கங்களின் பகுதியாகிவிடும், அவற்றை இனி திருத்த முடியாது. இன்னும் பயன்படுத்தப்படாத மறைப்புக் குறிகள் விடப்படும்.
pages-export-encrypt = கடவுச்சொல்லுடன் மறையாக்கு
pages-export-password = கடவுச்சொல்
pages-export-verify-password = கடவுச்சொல்லை உறுதிப்படுத்து
pages-export-resolution = தெளிவுத்திறன்
pages-export-dpi = { $dpi } dpi
pages-export-quality = தரம்
pages-export-quality-low = குறைவு
pages-export-quality-medium = நடுத்தரம்
pages-export-quality-high = அதிகம்
pages-export-quality-best = மிகச் சிறந்தது
pages-export-one-file = எல்லாப் பக்கங்களும் ஒரே கோப்பில் செல்லும்.
pages-export-file-per-page = ஒவ்வொரு பக்கமும் தனிக் கோப்பாகச் சேமிக்கப்படும், நீங்கள் தேர்வுசெய்யும் பெயருக்குப் பின் எண்ணிடப்படும்.
pages-export-selected-only = { $count ->
    [one] தேர்ந்தெடுத்த பக்கம் மட்டும்
   *[other] தேர்ந்தெடுத்த { $count } பக்கங்கள் மட்டும்
}
pages-export-choose = ஏற்றுமதி செய்…
pages-export-no-password = கடவுச்சொல்லை உள்ளிடவும்.
pages-export-password-mismatch = கடவுச்சொற்கள் பொருந்தவில்லை.
pages-export-file-name = { $name } (ஏற்றுமதி செய்யப்பட்டது)
pages-export-untitled = ஆவணம்
pages-export-same-file = புதிய கோப்புக்கு ஏற்றுமதி செய்யவும்; இந்த ஆவணம் தானாகவே சேமிக்கப்படும்.
pages-export-exporting = “{ $name }” ஏற்றுமதி செய்யப்படுகிறது…
pages-export-done = “{ $name }” ஏற்றுமதி செய்யப்பட்டது.
pages-export-done-images = { $count } படங்கள் ஏற்றுமதி செய்யப்பட்டன.
pages-export-failed = ஏற்றுமதி செய்ய முடியவில்லை: { $error }
pages-export-stopped = ஏற்றுமதி நின்றுவிட்டது

## Start window

app-start-hint = PDF, படம், SVG அல்லது Markdown கோப்பைத் திறக்கவும் அல்லது இங்கே விடவும்.
app-start-open = திற…
app-title-dev = { $title } (மேம்பாடு)
app-viewer-missing = { $kind }: இந்தக் காட்டி இன்னும் உருவாக்கப்படவில்லை.
app-cannot-open = prev-ஆல் இந்த வகைக் கோப்பைத் திறக்க முடியாது.
app-cannot-read = prev-ஆல் இந்தக் கோப்பைப் படிக்க முடியாது: { $error }
app-kind-pdf = PDF ஆவணம்
app-kind-image = { $format } படம்
app-kind-svg = SVG வரைபடம்
app-kind-markdown = Markdown ஆவணம்
app-file-dialog-failed = கோப்பு உரையாடலைக் காட்ட முடியவில்லை: { $error }

## Actions

action-open = திற
action-settings = அமைப்புகள்

## Toolbar

app-toolbar-keep-shown = கருவிப்பட்டியை எப்போதும் காட்டு
app-toolbar-auto-hide = சுட்டி வெளியேறும்போது கருவிப்பட்டியை மறை
app-toolbar-more = மேலும்

## File facts

app-fact-name = பெயர்
app-fact-folder = கோப்புறை
app-fact-size = அளவு
app-fact-modified = மாற்றப்பட்டது
app-size-bytes = { $count } பைட்டுகள்
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = தவறான இணைப்பு { $uri }: { $error }
app-link-open-failed = { $uri } ஐத் திறக்க முடியவில்லை: { $error }
app-paste-needs-wl-clipboard = படங்களை ஒட்ட wl-clipboard-ஐ நிறுவவும்
app-copy-needs-wl-clipboard = படங்களை நகலெடுக்க wl-clipboard-ஐ நிறுவவும்
app-copy-no-pixels = பகுதியில் பிக்சல்கள் இல்லை
app-copy-no-input = wl-copy-க்கு உள்ளீடு இல்லை
app-copy-failed = wl-copy தோல்வியடைந்தது
app-clipboard-open-failed = கிளிப்போர்டைத் திறக்க முடியவில்லை: { $error }
app-copy-image-failed = படத்தை நகலெடுக்க முடியவில்லை: { $error }

## Printing

print-failed = அச்சிட முடியவில்லை: { $error }
print-stopped = அச்சிடுதல் நின்றுவிட்டது
print-unavailable = இந்தக் கணினியில் இன்னும் அச்சிட முடியாது.
print-no-window = அச்சிட முடியவில்லை: அச்சு உரையாடலைக் காட்ட சாளரம் இல்லை
print-dialog-failed = அச்சு உரையாடலைக் காட்ட முடியவில்லை: { $error }
print-job-not-started = அச்சுப்பொறி பணியைத் தொடங்கவில்லை
print-printer-stopped = அச்சுப்பொறி நின்றுவிட்டது

## File dialogs

dialog-open = திற
dialog-filter-all = ஆதரிக்கப்படும் எல்லாக் கோப்புகளும்
dialog-filter-pdf = PDF ஆவணங்கள்
dialog-filter-images = படங்கள்
dialog-filter-svg = SVG வரைபடங்கள்
dialog-filter-markdown = Markdown
dialog-choose-signatures = கையொப்பக் கோப்புறையைத் தேர்வுசெய்
dialog-choose-versions = பதிப்பு வரலாற்றுக் கோப்புறையைத் தேர்வுசெய்
dialog-choose-bookmarks = புத்தகக்குறிக் கோப்பைத் தேர்வுசெய்

## Command line

usage-help =
    பயன்பாடு: prev [FILE]...

    PDFகளையும் படங்களையும் பார்க்கவும் திருத்தவும். கோப்புகள் இயங்கும் prev-இன்
    சாளரங்களில் திறக்கும்; தேவைப்பட்டால் prev தானாகத் தொடங்கும்.

    விருப்பங்கள்:
      -h, --help     இந்த உதவியைக் காட்டு
      -V, --version  பதிப்பைக் காட்டு

## Settings, continued

settings-language = மொழி
settings-language-system = கணினி இயல்புநிலை: { $language }
settings-input-language = உள்ளீட்டு மொழி
settings-input-language-system = விசைப்பலகை தளவமைப்பைப் பின்பற்று
settings-input-language-note = வெற்று உரைப் புலம் எந்தப் பக்கத்திலிருந்து தொடங்கும் என்பதை அமைக்கும். நீங்கள் தட்டச்சிடும் உரை அதன் சொந்தத் திசையைத் தக்கவைக்கும்.

settings-appearance-system = கணினி
settings-appearance-light = வெளிர்
settings-appearance-dark = அடர்
settings-system-accent = கணினியின் அக்சென்ட் வண்ணத்தைப் பயன்படுத்து
settings-omarchy-note = வண்ணங்கள் “{ $theme }” இன் அக்சென்ட்டிலிருந்து உருவாக்கப்படுகின்றன.
settings-system-accent-note = வண்ணங்கள் கணினியின் அக்சென்ட் வண்ணத்திலிருந்து உருவாக்கப்படுகின்றன.
settings-system-accent-none = கணினியில் அக்சென்ட் வண்ணம் இல்லை, எனவே prev தனது சொந்த வண்ணத்தைப் பயன்படுத்துகிறது.
settings-auto-hide = சுட்டி வெளியேறும்போது கருவிப்பட்டியை மறை
settings-auto-hide-note = கருவிப்பட்டி ஆவணத்தின் மேல் மிதக்கும், சுட்டி சாளரத்திற்கு வெளியே இருக்கும்போது நகர்ந்து மறையும்.
settings-animations = அனிமேஷன்கள்
settings-animations-note = நகரும் பட்டிகளும் பலகங்களும், விரியும் உரையாடல்களும், துள்ளும் பொத்தான்களும்.
settings-animations-reduced = கணினி குறைந்த இயக்கத்தைக் கோரும்போது முடக்கப்படும்.
settings-corner-radius = மூலை ஆரம்
settings-corner-radius-note = உரையாடல்களுக்கும் மிதக்கும் கருவிப்பட்டிக்கும்.
settings-corner-radius-value = { $radius } px
settings-overlay = மேலடுக்கு ஒளிபுகுதன்மை
settings-overlay-note = மிதக்கும் கருவிப்பட்டியின் வழியே பக்கம் எவ்வளவு தெரியும்.
settings-overlay-value = { $percent }%
settings-storage-signatures = கையொப்பக் கோப்புறை
settings-storage-versions = பதிப்பு வரலாற்றுக் கோப்புறை
settings-storage-bookmarks = புத்தகக்குறிக் கோப்பு
settings-storage-apply = பயன்படுத்து
settings-storage-choose = தேர்வுசெய்…
settings-storage-note = பழைய இடத்தில் ஏற்கெனவே உள்ள கோப்புகள் அங்கேயே இருக்கும்; அவற்றைத் தொடர்ந்து பயன்படுத்த, புதிய இடத்துக்கு நகர்த்தவும். prev பயன்பாட்டு அமைப்புகள் { $file } இல் சேமிக்கப்படுகின்றன.
settings-save-failed = அமைப்புகளைச் சேமிக்க முடியவில்லை: { $error }
settings-no-location = அமைப்புகளுக்கான இடம் இல்லை: HOME அமைக்கப்படவில்லை
settings-full-path = முழுப் பாதையைப் பயன்படுத்தவும், எ.கா. ~/Documents/prev.
settings-path-is-folder = { $path } ஒரு கோப்புறை, கோப்பு அல்ல.
settings-folder-missing = { $path } என்ற கோப்புறை இல்லை. முதலில் அதை உருவாக்கவும், அல்லது ஒன்றைத் தேர்வுசெய்யவும்.
settings-path-is-file = { $path } ஒரு கோப்பு, கோப்புறை அல்ல.
settings-cannot-write = prev-ஆல் { $path } இல் எழுத முடியாது: { $error }.

## Export dialog

export-title = ஏற்றுமதி செய்
export-format = வடிவமைப்பு
export-quality = தரம்
export-size = அளவு
export-choose = ஏற்றுமதி செய்…
export-format-webp = WebP (இழப்பற்றது)
export-format-unknown = படம்
export-quality-low = குறைவு
export-quality-medium = நடுத்தரம்
export-quality-high = அதிகம்
export-quality-best = மிகச் சிறந்தது
export-size-actual = உண்மையான அளவு
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } பிக்சல்கள்
export-dialog-failed = சேமிப்பு உரையாடலைக் காட்ட முடியவில்லை: { $error }
export-done = { $path } க்கு ஏற்றுமதி செய்யப்பட்டது
export-failed = ஏற்றுமதி செய்ய முடியவில்லை: { $error }
export-stopped = ஏற்றுமதி நின்றுவிட்டது

## Image window

image-marked-no-edit = மார்க்அப் உள்ள படங்களைத் திருத்த முடியாது. மார்க்அப்பை வைத்திருக்க ஏற்றுமதி செய்யவும், அல்லது அதை நீக்கி மார்க்அப் பட்டியை மூடவும்.

image-loading-stopped = ஏற்றுதல் நின்றுவிட்டது
image-reverting-stopped = மீட்டமைத்தல் நின்றுவிட்டது
image-rendering-stopped = வரைதல் நின்றுவிட்டது
image-saving-stopped = சேமித்தல் நின்றுவிட்டது
image-markup-stopped = மார்க்அப் நின்றுவிட்டது
image-no-version-store = பதிப்புகளை வைத்திருக்க இடம் இல்லை
image-revert-failed = மீட்டமைக்க முடியவில்லை: { $error }
image-read-failed = { $path } ஐப் படிக்க முடியவில்லை: { $error }
image-keep-original-failed = அசல் பதிப்பை வைத்திருக்க முடியவில்லை: { $error }
image-save-failed = { $path } ஐச் சேமிக்க முடியவில்லை: { $error }
image-markup-start-failed = மார்க்அப்பைத் தொடங்க முடியவில்லை: { $error }
image-cannot-edit = அனிமேஷன்களையும் SVG வரைபடங்களையும் திருத்த முடியாது.
image-cannot-mark-up = அனிமேஷன்களிலும் SVG வரைபடங்களிலும் மார்க்அப் செய்ய முடியாது.
image-mark-up-wait = திருத்தம் முடியும் வரை காத்திருந்து, பிறகு மார்க்அப் செய்யவும்.
image-crop-needs-selection = முதலில் ஒரு தேர்வை இழுக்கவும் (தேர்வுக் கருவி), பிறகு செதுக்கவும்.
image-size-needed = அகலத்தையும் உயரத்தையும் பிக்சல்களில் உள்ளிடவும்.
image-cannot-save-format = “{ $name }” இல் செய்த மாற்றங்களை அதன் வடிவமைப்பில் சேமிக்க முடியாது. ஏற்றுமதியைப் ({ $keys }) பயன்படுத்தவும்.
image-cannot-save-format-unbound = “{ $name }” இல் செய்த மாற்றங்களை அதன் வடிவமைப்பில் சேமிக்க முடியாது. ஏற்றுமதியைப் பயன்படுத்தவும்.
image-cannot-export-animation = அனிமேஷன்களை இன்னும் ஏற்றுமதி செய்ய முடியாது.
image-drop-pages = பக்கங்களை ஓர் ஆவணத்தின் மீது மட்டுமே விட முடியும்.
image-drag-failed = இழுக்கத் தொடங்க முடியவில்லை.
image-picture-save-failed = உங்கள் பதிவிறக்கங்கள் கோப்புறையில் படத்தைச் சேமிக்க முடியவில்லை.
image-open-failed = prev-ஆல் இந்தப் படத்தைத் திறக்க முடியாது
image-opening = திறக்கிறது…
image-name-mismatch-title = பெயர் வடிவமைப்புடன் பொருந்தவில்லை
image-name-mismatch = “{ $name }” ஒரு { $format } கோப்பாகச் சேமிக்கப்படும், ஆனால் அதன் பெயர் .{ $extension } இல் முடிகிறது. பிற பயன்பாடுகள் இதைத் திறக்காமல் போகலாம்.
image-name-mismatch-no-extension = “{ $name }” ஒரு { $format } கோப்பாகச் சேமிக்கப்படும், ஆனால் அதன் பெயரில் நீட்டிப்பு இல்லை. பிற பயன்பாடுகள் இதைத் திறக்காமல் போகலாம்.
image-choose-again = மீண்டும் தேர்வுசெய்
image-save-as-is = உள்ளபடியே சேமி
image-dimensions = { $width } × { $height }
image-frame-position = சட்டகம் { $total } இல் { $current }
image-position = { $total } இல் { $current }
image-edited = திருத்தப்பட்டது
image-sidebar = பக்கப்பட்டி
image-zoom-out = சிறிதாக்கு
image-zoom-in = பெரிதாக்கு
image-zoom = { $percent }%
image-fit = சாளரத்திற்குப் பொருத்து
image-actual-size = உண்மையான அளவு
image-undo = செயல்தவிர்
image-redo = மீண்டும் செய்
image-rotate-left = இடப்புறம் சுழற்று
image-rotate-right = வலப்புறம் சுழற்று
image-flip-horizontal = கிடைமட்டமாகப் புரட்டு
image-flip-vertical = செங்குத்தாகப் புரட்டு
image-select = செவ்வகத் தேர்வு
image-crop = தேர்வுக்கேற்பச் செதுக்கு
image-adjust-size-tool = அளவைச் சரிசெய்
image-adjust-color-tool = வண்ணத்தைச் சரிசெய்
image-inspector = ஆய்வி
image-markup = மார்க்அப்
image-export = ஏற்றுமதி செய்
image-settings = அமைப்புகள்
image-adjust-color = வண்ணத்தைச் சரிசெய்
image-adjust-size = அளவைச் சரிசெய்
image-exposure = வெளிப்பாடு
image-contrast = மாறுபாடு
image-saturation = செறிவு
image-temperature = வெப்பநிலை
image-tint = சாயல்
image-sepia = செபியா
image-sharpness = கூர்மை
image-levels = நிலைகள்
image-black-point = கருப்புப் புள்ளி
image-midtones = நடுத்தொனிகள்
image-white-point = வெள்ளைப் புள்ளி
image-reset-all = அனைத்தையும் மீட்டமை
image-current-size = தற்போதைய அளவு: { $width } × { $height } பிக்சல்கள்
image-width = அகலம்
image-height = உயரம்
image-scale-proportionally = விகிதப்படி அளவிடு
image-resize = அளவை மாற்று
image-inspector-loading = ஏற்றுகிறது…
image-file = கோப்பு
image-format = வடிவமைப்பு
image-dimensions-label = பரிமாணங்கள்
image-pixels = { $width } × { $height } பிக்சல்கள்
image-no-camera = கேமரா தகவல் இல்லை.
image-location = இருப்பிடம்
image-remove-location = இருப்பிடத் தகவலை அகற்று
image-no-location = இருப்பிடத் தகவல் இல்லை.
image-keywords-description = முக்கியச் சொற்களும் விளக்கமும்
image-keywords-hint = முக்கியச் சொற்கள், காற்புள்ளிகளால் பிரிக்கப்பட்டவை
image-description = விளக்கம்
image-keywords-unsupported = முக்கியச் சொற்களை JPEG, PNG, WebP கோப்புகளில் சேமிக்கலாம்.
image-revert-to = முந்தைய பதிப்புக்கு மீட்டமை
image-no-versions = முந்தைய பதிப்புகள் இல்லை.
image-revert = மீட்டமை
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = மார்க்அப்பை ஏற்றுமதி செய்யாமல் மூடவா?
image-close-body = { $count ->
    [one] படத்தின் மீதான மார்க்அப் அதன் சாளரம் திறந்திருக்கும் வரை மட்டுமே இருக்கும். அதை வைத்திருக்கப் படத்தை ஏற்றுமதி செய்யவும்: நீங்கள் சேமிக்கும் நகலில் மார்க்அப் வரையப்படும்.
   *[other] படங்களின் மீதான மார்க்அப் அவற்றின் சாளரம் திறந்திருக்கும் வரை மட்டுமே இருக்கும். அதை வைத்திருக்க ஒவ்வொரு படத்தையும் ஏற்றுமதி செய்யவும்: நீங்கள் சேமிக்கும் நகலில் மார்க்அப் வரையப்படும்.
}
image-close-anyway = பரவாயில்லை, மூடு

## Markdown

markdown-reading-stopped = படித்தல் நின்றுவிட்டது
markdown-read-failed = prev-ஆல் இந்தக் கோப்பைப் படிக்க முடியாது
markdown-draw-failed = ஆவணத்தை வரைய முடியவில்லை
markdown-export-size = முழு ஆவணம், { $width } × { $height } பிக்சல்கள்
markdown-not-found = கிடைக்கவில்லை
markdown-match = { $total } இல் { $current }
markdown-search = தேடு
markdown-smaller-text = சிறிய உரை
markdown-larger-text = பெரிய உரை
markdown-zoom = { $percent }%
markdown-actual-size = உண்மையான அளவு
markdown-inspector = ஆய்வி
markdown-export = ஏற்றுமதி செய்
markdown-settings = அமைப்புகள்
markdown-file = கோப்பு
markdown-document = ஆவணம்
markdown-words = சொற்கள்
markdown-lines = வரிகள்
markdown-pictures = படங்கள்

## Image details

image-meta-camera = கேமரா
image-meta-exposure = வெளிப்பாடு
image-meta-image = படம்
image-meta-make = தயாரிப்பாளர்
image-meta-model = மாடல்
image-meta-lens = லென்ஸ்
image-meta-exposure-time = வெளிப்பாட்டு நேரம்
image-meta-f-number = F-எண்
image-meta-iso = ISO
image-meta-focal-length = குவிய நீளம்
image-meta-exposure-bias = வெளிப்பாட்டுச் சார்பு
image-meta-flash = ஃபிளாஷ்
image-meta-date-taken = எடுத்த தேதி
image-meta-orientation = திசையமைவு
image-meta-color-space = வண்ண வெளி
image-meta-software = மென்பொருள்
image-meta-artist = கலைஞர்
image-meta-copyright = பதிப்புரிமை
image-meta-seconds = { $value } வி.
image-meta-millimeters = { $value } மி.மீ.
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] இயல்பானது
    [2] கிடைமட்டமாகப் பிரதிபலிக்கப்பட்டது
    [3] 180° சுழற்றப்பட்டது
    [4] செங்குத்தாகப் பிரதிபலிக்கப்பட்டது
    [5] கிடைமட்டமாகப் பிரதிபலிக்கப்பட்டு, இடஞ்சுழியாக 90° சுழற்றப்பட்டது
    [6] வலஞ்சுழியாக 90° சுழற்றப்பட்டது
    [7] கிடைமட்டமாகப் பிரதிபலிக்கப்பட்டு, வலஞ்சுழியாக 90° சுழற்றப்பட்டது
    [8] இடஞ்சுழியாக 90° சுழற்றப்பட்டது
   *[other] தெரியாதது ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] ஒளிர்ந்தது
   *[no] ஒளிரவில்லை
}{ $mode ->
    [on] , கட்டாயமாக இயக்கத்தில்
    [off] , முடக்கத்தில்
    [auto] , தானியங்கி
   *[unknown] {""}
}{ $redeye ->
    [yes] , சிவப்புக்கண் குறைப்பு
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] அளவீடு செய்யப்படாதது
   *[other] பிற ({ $code })
}

## Errors

error-pdf-open = ஆவணத்தைத் திறக்க முடியாது: { $detail }
error-pdf-page-out-of-range = பக்கம் { $page } இல்லை
error-pdf-password-protected = ஆவணம் கடவுச்சொல்லால் பாதுகாக்கப்பட்டுள்ளது; அதைத் திறந்து, அதற்குப் பதிலாக அதன் பக்கங்களை நகலெடுக்கவும்
error-pdf-no-pages = பிரித்தெடுக்கப் பக்கங்கள் இல்லை
error-pdf-crop-outside = செதுக்கும் பகுதி பக்கத்திற்கு வெளியே உள்ளது
error-pdf-closed = ஆவணம் மூடப்பட்டது
error-pdf-saved-unreadable = சேமித்த ஆவணம் இனி திறக்கவில்லை
error-image-read = கோப்பைப் படிக்க முடியாது: { $detail }
error-image-invalid = படம் சேதமடைந்துள்ளது அல்லது தவறானது: { $detail }
error-image-missing-library = இந்த வடிவமைப்பைத் திறக்க { $library } தேவை, அது நிறுவப்படவில்லை
error-image-unsupported = { $format } படங்கள் இன்னும் ஆதரிக்கப்படவில்லை
error-image-encode = படத்தைக் குறியாக்க முடியாது: { $detail }
error-exif-malformed = EXIF தரவு சிதைந்துள்ளது
error-settings-read = அமைப்புகளைப் படிக்க முடியாது: { $detail }
error-settings-invalid = தவறான அமைப்புகள்: { $detail }
error-remove-location = இருப்பிடத்தை அகற்ற முடியவில்லை: { $error }
error-location-unsupported = இருப்பிடத் தகவலை JPEG, PNG, WebP, TIFF கோப்புகளிலிருந்து அகற்றலாம்
error-xmp-unsupported = முக்கியச் சொற்களையும் விளக்கங்களையும் JPEG, PNG, WebP கோப்புகளில் மட்டுமே சேமிக்க முடியும்

## Formats

format-camera-raw = கேமரா RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev பற்றி
menu-settings = அமைப்புகள்…
menu-services = சேவைகள்
menu-hide = prev-ஐ மறை
menu-hide-others = மற்றவற்றை மறை
menu-show-all = அனைத்தையும் காட்டு
menu-quit = prev-இலிருந்து வெளியேறு
menu-file = கோப்பு
menu-open = திற…
menu-close = சாளரத்தை மூடு
menu-export = ஏற்றுமதி செய்…
menu-print = அச்சிடு…
menu-edit = திருத்து
menu-undo = செயல்தவிர்
menu-redo = மீண்டும் செய்
menu-cut = வெட்டு
menu-copy = நகலெடு
menu-paste = ஒட்டு
menu-select-all = அனைத்தையும் தேர்ந்தெடு
menu-find = கண்டுபிடி
menu-find-next = அடுத்ததைக் கண்டுபிடி
menu-find-previous = முந்தையதைக் கண்டுபிடி
menu-view = காட்சி
menu-hide-sidebar = பக்கப்பட்டியை மறை
menu-thumbnails = சிறுபடங்கள்
menu-contents = உள்ளடக்க அட்டவணை
menu-notes = தனிப்படுத்தல்களும் குறிப்புகளும்
menu-bookmarks = புத்தகக்குறிகள்
menu-zoom-in = பெரிதாக்கு
menu-zoom-out = சிறிதாக்கு
menu-actual-size = உண்மையான அளவு
menu-zoom-to-fit = பொருந்தும்படி பெரிதாக்கு
menu-inspector = ஆய்வியைக் காட்டு
menu-slideshow = ஸ்லைடுஷோ
menu-full-screen = முழுத்திரைக்குச் செல்
menu-go = செல்
menu-next-page = அடுத்த பக்கம்
menu-previous-page = முந்தைய பக்கம்
menu-go-to-page = பக்கத்திற்குச் செல்…
menu-bookmark = புத்தகக்குறியைச் சேர்
menu-tools = கருவிகள்
menu-markup = மார்க்அப் கருவிப்பட்டியைக் காட்டு
menu-rotate-left = இடப்புறம் சுழற்று
menu-rotate-right = வலப்புறம் சுழற்று
menu-crop = செதுக்கு
menu-adjust-color = வண்ணத்தைச் சரிசெய்…
menu-window = சாளரம்
menu-minimize = குறுக்கு
menu-zoom = சாளரத்தைப் பெரிதாக்கு
menu-bring-all-to-front = அனைத்தையும் முன்னால் கொண்டுவா
