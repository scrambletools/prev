# prev's interface text in Hindi (हिन्दी), translated from i18n/en/prev.ftl.
#
# Each line is `key = text`. Keys stay the same in every language; only the
# text after `=` is translated. `{ $name }` is a value prev fills in, such
# as a page number or a file name; keep it, and move it where the
# language needs it. Plural and other variants use Fluent's selectors:
# https://projectfluent.org/fluent/guide/selectors.html
#
# Sections follow the parts of the interface. Keys shared by several
# parts are under "Common".
#
# A first draft, open to further review. Terms used throughout: markup = मार्कअप,
# note = नोट, highlight = हाइलाइट, annotation = एनोटेशन,
# redact/redaction = रिडैक्ट/रिडैक्शन, inspector = इंस्पेक्टर,
# zoom in/out = ज़ूम इन/ज़ूम आउट, bookmark = बुकमार्क, page = पृष्ठ,
# export = एक्सपोर्ट, revert = पिछले संस्करण पर लौटना, window = विंडो,
# print = प्रिंट, undo/redo = पूर्ववत करें/फिर से करें. Buttons and menu items use the polite imperative
# (सहेजें, रद्द करें); labels use nouns.

## Language

# This language's name in itself, as the Settings language list shows it,
# such as English, Deutsch or עברית.
language-name = हिन्दी

## Common

common-cancel = रद्द करें
common-close = बंद करें
common-save = सहेजें

## Settings

settings-title = सेटिंग्स
settings-appearance = दिखावट
settings-colors = रंग
settings-windows = विंडो
settings-storage = स्टोरेज
settings-version = prev { $version }
settings-version-development = prev { $version } (डेवलपमेंट बिल्ड)

## Markup toolbar

markup-tool-select = चुनें
markup-tool-area = आयताकार चयन
markup-tool-sketch = स्केच
markup-tool-draw = ड्रॉ करें
markup-tool-shapes = आकृतियाँ
markup-tool-text-box = टेक्स्ट बॉक्स
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = हाइलाइट करें
markup-tool-note = नोट
# Opens the menu of saved signatures (a verb).
markup-tool-sign = हस्ताक्षर करें
# A verb: the tool that marks areas to black out.
markup-tool-redact = रिडैक्ट करें
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = लागू करें
markup-apply-redactions = रिडैक्शन लागू करें
markup-shape-style = आकृति शैली
markup-border-color = बॉर्डर का रंग
markup-fill-color = भरण रंग
markup-text-style = टेक्स्ट शैली
markup-delete = हटाएँ
markup-undo = पूर्ववत करें
markup-redo = फिर से करें

## Markup menus

markup-shape-rectangle = आयत
markup-shape-rounded-rectangle = गोलाकार कोनों वाला आयत
markup-shape-oval = अंडाकार
markup-shape-line = रेखा
markup-shape-arrow = तीर
markup-shape-star = तारा
markup-shape-polygon = बहुभुज
markup-shape-speech-bubble = स्पीच बबल
# A shape that magnifies the part of the page under it.
markup-shape-loupe = आवर्धक लेंस
# A shape that darkens the page around it.
markup-shape-mask = मास्क
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = हाइलाइट
markup-style-underline = रेखांकन
markup-style-strikethrough = स्ट्राइकथ्रू
markup-style-squiggly = लहरदार रेखा
# Menu section headings.
markup-menu-color = रंग
markup-menu-font = फ़ॉन्ट
markup-menu-size = आकार
markup-menu-alignment = संरेखण
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = डैश वाली रेखा

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = नोट
markup-kind-text-box = टेक्स्ट बॉक्स
markup-kind-stamp = स्टैम्प
markup-kind-redaction = रिडैक्शन
markup-kind-shape = आकृति
# Tooltips on a note being edited.
markup-note-delete = नोट हटाएँ
markup-note-done = हो गया
markup-note-placeholder = नोट लिखें
markup-notes-empty = कोई हाइलाइट या नोट नहीं
markup-notes-empty-hint = हाइलाइट, नोट और टेक्स्ट बॉक्स यहाँ दिखाई देते हैं।
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = पृष्ठ { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = दस्तावेज़ बदला नहीं जा सका: { $error }
markup-copy-area-failed = क्षेत्र कॉपी नहीं किया जा सका: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = दस्तावेज़ बंद हो गया
markup-render-area-failed = क्षेत्र रेंडर नहीं किया जा सका
markup-copy-stopped = कॉपी करना रुक गया

## Signatures

signature-menu-empty = अभी कोई हस्ताक्षर नहीं है।
signature-delete = हस्ताक्षर हटाएँ
signature-create = हस्ताक्षर बनाएँ…
signature-dialog-title = हस्ताक्षर बनाएँ
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = ड्रॉ करें
signature-tab-type = टाइप करें
signature-tab-image = छवि
signature-draw-hint = रेखा पर अपने माउस, पेन या टचपैड से हस्ताक्षर करें।
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = आपका नाम
signature-image-hint = सफ़ेद काग़ज़ पर किए गए अपने हस्ताक्षर की फ़ोटो या स्कैन चुनें।
signature-choose-image = छवि चुनें…
# Placeholder of the field naming the signature in the library.
signature-description = विवरण, जैसे पूरा नाम या आद्याक्षर
# Clears the drawing, typed name or image.
signature-clear = साफ़ करें
# The color the signature is drawn or typed in.
signature-ink = स्याही
# The pen's width, for drawing.
signature-thickness = मोटाई
signature-sign-first = पहले हस्ताक्षर करें, फिर सहेजें।
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = हस्ताक्षर { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = हस्ताक्षर बदले नहीं जा सके: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = कोई डेटा फ़ोल्डर नहीं: HOME सेट नहीं है
signature-removing-stopped = हटाना रुक गया
signature-saving-stopped = सहेजना रुक गया
signature-reading-stopped = पढ़ना रुक गया
signature-not-an-image = यह फ़ाइल ऐसी छवि नहीं है जिसे prev पढ़ सके
signature-no-frames = छवि में कोई फ़्रेम नहीं है
signature-not-found = छवि में कोई हस्ताक्षर नहीं मिला

## Dragging

drag-pages-need-document = पृष्ठ केवल किसी दस्तावेज़ पर छोड़े जा सकते हैं।
drag-image-unsupported = prev यह छवि नहीं खोल सकता।
# $error is a lowercase reason or a technical message.
drag-area-failed = क्षेत्र ड्रैग नहीं किया जा सका: { $error }
drag-pages-failed = पृष्ठ ड्रैग नहीं किए जा सके: { $error }
drag-start-failed = ड्रैग करना शुरू नहीं किया जा सका।
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = पृष्ठ
drag-file-one-page = { $name } (पृष्ठ { $page })
drag-file-page-range = { $name } (पृष्ठ { $first }–{ $last })

## PDF window

pdf-opening = खोला जा रहा है…
pdf-open-failed = prev यह दस्तावेज़ नहीं खोल सकता
pdf-no-pages = दस्तावेज़ में कोई पृष्ठ नहीं है।
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = दस्तावेज़ बंद हो गया
pdf-keep-original-failed = मूल संस्करण रखा नहीं जा सका: { $error }
pdf-save-failed = सहेजा नहीं जा सका: { $error }
pdf-nothing-to-paste = पेस्ट करने के लिए कुछ नहीं है।
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = पेस्ट करना रुक गया
pdf-file-dialog-failed = फ़ाइल डायलॉग नहीं दिखाया जा सका: { $error }
pdf-bookmarks-no-home = बुकमार्क सहेजे नहीं जा सकते: HOME सेट नहीं है
pdf-bookmarks-save-failed = बुकमार्क सहेजे नहीं जा सके: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = पृष्ठ { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” पासवर्ड से सुरक्षित है
pdf-password = पासवर्ड
pdf-password-wrong = पासवर्ड ग़लत है। फिर से कोशिश करें।
# Button that opens a locked document.
pdf-unlock = अनलॉक करें

# Toolbar tooltips and labels.
pdf-sidebar = साइडबार
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = / { $count }
pdf-zoom-out = ज़ूम आउट करें
pdf-zoom-in = ज़ूम इन करें
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = पृष्ठ में फ़िट करें
pdf-fit-width = चौड़ाई में फ़िट करें
pdf-actual-size = वास्तविक आकार
pdf-view-continuous = लगातार स्क्रॉल
pdf-view-single-page = एकल पृष्ठ
pdf-view-two-pages = दो पृष्ठ
pdf-undo = पूर्ववत करें
pdf-redo = फिर से करें
pdf-rotate-left = बाएँ घुमाएँ
pdf-rotate-right = दाएँ घुमाएँ
pdf-inspector = इंस्पेक्टर
pdf-markup = मार्कअप
# Tooltip of the button that opens the export dialog.
pdf-export = एक्सपोर्ट करें
pdf-settings = सेटिंग्स

# Search field.
pdf-search = खोजें
pdf-search-not-found = नहीं मिला
pdf-searching = खोजा जा रहा है…
# The match shown, of all matches found.
pdf-search-match = { $total } में से { $current }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $total }+ में से { $current }

# Inspector: section headings.
pdf-inspector-file = फ़ाइल
pdf-inspector-document = दस्तावेज़
pdf-inspector-pages = पृष्ठ
# Inspector: fact labels and values.
pdf-inspector-title = शीर्षक
pdf-inspector-author = लेखक
pdf-inspector-subject = विषय
pdf-inspector-keywords = कीवर्ड
pdf-inspector-created = बनाया गया
pdf-inspector-modified = संशोधित
pdf-inspector-application = ऐप्लिकेशन
pdf-inspector-producer = PDF निर्माता
pdf-inspector-version = संस्करण
pdf-inspector-security = सुरक्षा
pdf-inspector-not-encrypted = एन्क्रिप्ट नहीं किया गया
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = एन्क्रिप्ट किया गया ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } पृष्ठ
   *[other] { $count } पृष्ठ
}
pdf-inspector-page-size = पृष्ठ का आकार
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } in)
pdf-loading = लोड हो रहा है…

# Sidebar tabs and lists.
pdf-tab-pages = पृष्ठ
pdf-tab-contents = विषय-सूची
pdf-tab-notes = हाइलाइट और नोट
pdf-tab-bookmarks = बुकमार्क
pdf-no-outline = कोई विषय-सूची नहीं
pdf-no-outline-detail = इस दस्तावेज़ में कोई रूपरेखा नहीं है।
pdf-no-bookmarks = कोई बुकमार्क नहीं
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = किसी पृष्ठ को बुकमार्क करने के लिए { $keys } दबाएँ।
pdf-remove-bookmark = बुकमार्क निकालें

## Page editing

# Tooltip of the Pages menu button.
pages-menu = पृष्ठ
pages-insert-blank = रिक्त पृष्ठ डालें
pages-insert-file = फ़ाइल से डालें…
pages-copy = { $count ->
    [one] पृष्ठ कॉपी करें
   *[other] पृष्ठ कॉपी करें
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] पृष्ठ पेस्ट करें
   *[other] { $count } पृष्ठ पेस्ट करें
}
pages-crop = चयन के अनुसार क्रॉप करें
pages-select-all = सभी पृष्ठ चुनें
pages-delete = { $count ->
    [one] पृष्ठ हटाएँ
   *[other] पृष्ठ हटाएँ
}
pages-apply-redactions = रिडैक्शन लागू करें…
pages-no-copied = पेस्ट करने के लिए कोई कॉपी किया गया पृष्ठ नहीं है।
pages-copied = { $count ->
    [one] { $count } पृष्ठ कॉपी किया गया।
   *[other] { $count } पृष्ठ कॉपी किए गए।
}
pages-copy-failed = पृष्ठ कॉपी नहीं किए जा सके: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = पढ़ना रुक गया
pages-read-failed = फ़ाइल पढ़ी नहीं जा सकी: { $error }
pages-at-least-one = किसी दस्तावेज़ में कम से कम एक पृष्ठ होना चाहिए।
pages-crop-needs-area = पहले आयताकार चयन टूल से कोई क्षेत्र चुनें।
pages-change-failed = पृष्ठ बदले नहीं जा सके: { $error }
pages-no-redactions = लागू करने के लिए कोई रिडैक्शन नहीं था।
pages-redactions-applied = { $count ->
    [one] { $count } रिडैक्शन लागू किया गया।
   *[other] { $count } रिडैक्शन लागू किए गए।
}
pages-forget-versions-failed = पुराने संस्करण हटाए नहीं जा सके: { $error }
pages-redact-title = रिडैक्शन लागू करें?
pages-redact-body = { $count ->
    [one] चिह्न के नीचे मौजूद टेक्स्ट, छवियाँ और ड्रॉइंग दस्तावेज़ से हमेशा के लिए हटा दिए जाते हैं, और चिह्न काला बॉक्स बन जाता है। इसे पूर्ववत नहीं किया जा सकता, और prev द्वारा रखे गए इस फ़ाइल के पुराने संस्करण हटा दिए जाते हैं।
   *[other] { $count } चिह्नों के नीचे मौजूद टेक्स्ट, छवियाँ और ड्रॉइंग दस्तावेज़ से हमेशा के लिए हटा दिए जाते हैं, और चिह्न काले बॉक्स बन जाते हैं। इसे पूर्ववत नहीं किया जा सकता, और prev द्वारा रखे गए इस फ़ाइल के पुराने संस्करण हटा दिए जाते हैं।
}
# Button that applies redactions.
pages-redact-apply = लागू करें

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = एक्सपोर्ट करें
pages-export-format = फ़ॉर्मेट
pages-export-reduce = फ़ाइल का आकार घटाएँ (150 dpi पर छवियाँ)
pages-export-flatten = एनोटेशन और फ़ॉर्म फ़ील्ड फ़्लैट करें
pages-export-flatten-detail = मार्कअप और भरे गए फ़ील्ड पृष्ठों का हिस्सा बन जाते हैं और उन्हें फिर संपादित नहीं किया जा सकता। जो रिडैक्शन चिह्न अभी लागू नहीं किए गए हैं, वे छोड़ दिए जाते हैं।
pages-export-encrypt = पासवर्ड से एन्क्रिप्ट करें
pages-export-password = पासवर्ड
pages-export-verify-password = पासवर्ड की पुष्टि करें
pages-export-resolution = रिज़ॉल्यूशन
pages-export-dpi = { $dpi } dpi
pages-export-quality = गुणवत्ता
# JPEG quality choices.
pages-export-quality-low = निम्न
pages-export-quality-medium = मध्यम
pages-export-quality-high = उच्च
pages-export-quality-best = सर्वश्रेष्ठ
pages-export-one-file = सभी पृष्ठ एक ही फ़ाइल में जाते हैं।
pages-export-file-per-page = हर पृष्ठ अलग फ़ाइल के रूप में सहेजा जाता है, और आपके चुने गए नाम के बाद उसकी संख्या जुड़ जाती है।
pages-export-selected-only = { $count ->
    [one] केवल चयनित पृष्ठ
   *[other] केवल { $count } चयनित पृष्ठ
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = एक्सपोर्ट करें…
pages-export-no-password = पासवर्ड दर्ज करें।
pages-export-password-mismatch = पासवर्ड मेल नहीं खाते।
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (एक्सपोर्ट किया गया)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = दस्तावेज़
pages-export-same-file = नई फ़ाइल में एक्सपोर्ट करें; यह दस्तावेज़ अपने आप सहेजा जाता है।
pages-export-exporting = “{ $name }” एक्सपोर्ट किया जा रहा है…
pages-export-done = “{ $name }” एक्सपोर्ट किया गया।
pages-export-done-images = { $count } छवियाँ एक्सपोर्ट की गईं।
pages-export-failed = एक्सपोर्ट नहीं किया जा सका: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = एक्सपोर्ट करना रुक गया

## Start window

# Under the app name in a window with no file open.
app-start-hint = कोई PDF, छवि, SVG या Markdown फ़ाइल खोलें या यहाँ छोड़ें।
app-start-open = खोलें…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (डेवलपमेंट)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: यह व्यूअर अभी बनाया नहीं गया है।
app-cannot-open = prev इस प्रकार की फ़ाइल नहीं खोल सकता।
app-cannot-read = prev यह फ़ाइल नहीं पढ़ सकता: { $error }
app-kind-pdf = PDF दस्तावेज़
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format } छवि
app-kind-svg = SVG ड्रॉइंग
app-kind-markdown = Markdown दस्तावेज़
app-file-dialog-failed = फ़ाइल डायलॉग नहीं दिखाया जा सका: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = खोलें
action-settings = सेटिंग्स

## Toolbar

app-toolbar-keep-shown = टूलबार दिखाते रहें
app-toolbar-auto-hide = पॉइंटर हटने पर टूलबार छिपाएँ
# The button that shows the toolbar's hidden tools.
app-toolbar-more = अधिक

## File facts

# Labels in a file's inspector.
app-fact-name = नाम
app-fact-folder = फ़ोल्डर
app-fact-size = आकार
app-fact-modified = संशोधित
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } बाइट
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = अमान्य लिंक { $uri }: { $error }
app-link-open-failed = { $uri } नहीं खोला जा सका: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = छवियाँ पेस्ट करने के लिए wl-clipboard इंस्टॉल करें
app-copy-needs-wl-clipboard = छवियाँ कॉपी करने के लिए wl-clipboard इंस्टॉल करें
app-copy-no-pixels = क्षेत्र में कोई पिक्सेल नहीं है
# wl-copy is a program's name.
app-copy-no-input = wl-copy को कोई इनपुट नहीं मिला
app-copy-failed = wl-copy विफल रहा
app-clipboard-open-failed = क्लिपबोर्ड नहीं खोला जा सका: { $error }
app-copy-image-failed = छवि कॉपी नहीं की जा सकी: { $error }

## Printing

print-failed = प्रिंट नहीं किया जा सका: { $error }
print-stopped = प्रिंट करना रुक गया
print-unavailable = इस सिस्टम पर अभी प्रिंट करना उपलब्ध नहीं है।
print-no-window = प्रिंट नहीं किया जा सका: प्रिंट डायलॉग दिखाने के लिए कोई विंडो नहीं है
print-dialog-failed = प्रिंट डायलॉग नहीं दिखाया जा सका: { $error }
# Shown after "Could not print:".
print-job-not-started = प्रिंटर ने जॉब शुरू नहीं किया
# Shown after "Could not print:".
print-printer-stopped = प्रिंटर रुक गया

## File dialogs

dialog-open = खोलें
dialog-filter-all = सभी समर्थित फ़ाइलें
dialog-filter-pdf = PDF दस्तावेज़
dialog-filter-images = छवियाँ
dialog-filter-svg = SVG ड्रॉइंग
dialog-filter-markdown = Markdown
dialog-choose-signatures = हस्ताक्षर फ़ोल्डर चुनें
dialog-choose-versions = संस्करण इतिहास फ़ोल्डर चुनें
dialog-choose-bookmarks = बुकमार्क फ़ाइल चुनें

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    उपयोग: prev [FILE]...

    PDF और छवियाँ देखें और संपादित करें। फ़ाइलें चल रहे prev की विंडो में
    खुलती हैं, जो ज़रूरत पड़ने पर शुरू हो जाता है।

    विकल्प:
      -h, --help     यह सहायता दिखाएँ
      -V, --version  संस्करण दिखाएँ

## Settings, continued

settings-language = भाषा
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = सिस्टम डिफ़ॉल्ट: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = इनपुट भाषा
settings-input-language-system = कीबोर्ड लेआउट का पालन करें
settings-input-language-note = यह तय करता है कि खाली टेक्स्ट फ़ील्ड किस ओर से शुरू हो। आपका टाइप किया गया टेक्स्ट अपनी दिशा बनाए रखता है।

settings-appearance-system = सिस्टम
settings-appearance-light = लाइट
settings-appearance-dark = डार्क
settings-omarchy-accent = Omarchy एक्सेंट रंग का उपयोग करें
# $theme is the Omarchy theme's name.
settings-omarchy-note = रंग “{ $theme }” के एक्सेंट से बनाए जाते हैं।
settings-omarchy-none = कोई Omarchy थीम सक्रिय नहीं है।
settings-auto-hide = पॉइंटर हटने पर टूलबार छिपाएँ
settings-auto-hide-note = टूलबार दस्तावेज़ के ऊपर तैरता है और पॉइंटर के विंडो से बाहर रहने पर खिसककर हट जाता है।
settings-animations = एनिमेशन
settings-animations-note = खिसकते बार और पैनल, बढ़ते डायलॉग और उछलते बटन।
settings-animations-reduced = जब सिस्टम कम गति माँगता है, तब बंद रहता है।
settings-corner-radius = कोने की त्रिज्या
settings-corner-radius-note = डायलॉग और तैरते टूलबार के लिए।
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = ओवरले पारदर्शिता
settings-overlay-note = तैरते टूलबार के आर-पार पृष्ठ कितना दिखाई देता है।
settings-overlay-value = { $percent }%
settings-storage-signatures = हस्ताक्षर फ़ोल्डर
settings-storage-versions = संस्करण इतिहास फ़ोल्डर
settings-storage-bookmarks = बुकमार्क फ़ाइल
settings-storage-apply = लागू करें
settings-storage-choose = चुनें…
# $file is where the settings file is.
settings-storage-note = पुरानी जगह पर पहले से रखी फ़ाइलें वहीं रहती हैं; उनका उपयोग जारी रखने के लिए उन्हें नई जगह पर ले जाएँ। prev ऐप की सेटिंग्स { $file } में सहेजी जाती हैं।
settings-save-failed = सेटिंग्स सहेजी नहीं जा सकीं: { $error }
settings-no-location = सेटिंग्स के लिए कोई स्थान नहीं: HOME सेट नहीं है
settings-full-path = पूरा पाथ इस्तेमाल करें, जैसे ~/Documents/prev।
settings-path-is-folder = { $path } एक फ़ोल्डर है, फ़ाइल नहीं।
settings-folder-missing = { $path } नाम का कोई फ़ोल्डर नहीं है। पहले उसे बनाएँ, या कोई फ़ोल्डर चुनें।
settings-path-is-file = { $path } एक फ़ाइल है, फ़ोल्डर नहीं।
settings-cannot-write = prev { $path } में नहीं लिख सकता: { $error }।

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = एक्सपोर्ट करें
# Section headings in the export dialog.
export-format = फ़ॉर्मेट
export-quality = गुणवत्ता
export-size = आकार
# Button that goes on to choose where to save the export.
export-choose = एक्सपोर्ट करें…
# Format choice; the format name stays as it is.
export-format-webp = WebP (लॉसलेस)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = छवि
# JPEG quality choices.
export-quality-low = निम्न
export-quality-medium = मध्यम
export-quality-high = उच्च
export-quality-best = सर्वश्रेष्ठ
# Size choices: the picture at its own size, or scaled up.
export-size-actual = वास्तविक आकार
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } पिक्सेल
# $error is the system's reason.
export-dialog-failed = सहेजें डायलॉग नहीं दिखाया जा सका: { $error }
# $path is where the file was saved.
export-done = { $path } में एक्सपोर्ट किया गया
export-failed = एक्सपोर्ट नहीं किया जा सका: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = एक्सपोर्ट करना रुक गया

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = मार्कअप वाली छवियाँ संपादित नहीं की जा सकतीं। मार्कअप रखने के लिए एक्सपोर्ट करें, या उसे हटाकर मार्कअप बार बंद करें।

# Shown if a background task ends unexpectedly.
image-loading-stopped = लोड करना रुक गया
image-reverting-stopped = पिछले संस्करण पर लौटना रुक गया
image-rendering-stopped = रेंडर करना रुक गया
image-saving-stopped = सहेजना रुक गया
image-markup-stopped = मार्कअप रुक गया
image-no-version-store = संस्करण रखने के लिए कोई जगह नहीं है
image-revert-failed = पिछले संस्करण पर नहीं लौटा जा सका: { $error }
image-read-failed = { $path } पढ़ा नहीं जा सका: { $error }
image-keep-original-failed = मूल संस्करण रखा नहीं जा सका: { $error }
image-save-failed = { $path } सहेजा नहीं जा सका: { $error }
image-markup-start-failed = मार्कअप शुरू नहीं किया जा सका: { $error }
image-cannot-edit = एनिमेशन और SVG ड्रॉइंग संपादित नहीं की जा सकतीं।
image-cannot-mark-up = एनिमेशन और SVG ड्रॉइंग पर मार्कअप नहीं किया जा सकता।
image-mark-up-wait = संपादन पूरा होने तक प्रतीक्षा करें, फिर मार्कअप करें।
image-crop-needs-selection = पहले एक चयन ड्रैग करें (चयन टूल), फिर क्रॉप करें।
image-size-needed = पिक्सेल में चौड़ाई और ऊँचाई दर्ज करें।
# $name is a file name.
image-cannot-save-format = “{ $name }” में किए गए बदलाव उसके फ़ॉर्मेट में सहेजे नहीं जा सकते। एक्सपोर्ट ({ $keys }) का उपयोग करें।
image-cannot-export-animation = एनिमेशन अभी एक्सपोर्ट नहीं किए जा सकते।
image-drop-pages = पृष्ठ केवल किसी दस्तावेज़ पर छोड़े जा सकते हैं।
image-drag-failed = ड्रैग करना शुरू नहीं किया जा सका।
image-open-failed = prev यह छवि नहीं खोल सकता
image-opening = खोला जा रहा है…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = नाम फ़ॉर्मेट से मेल नहीं खाता
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” को { $format } फ़ाइल के रूप में सहेजा जाएगा, लेकिन इसका नाम .{ $extension } पर समाप्त होता है। हो सकता है अन्य ऐप इसे न खोलें।
image-name-mismatch-no-extension = “{ $name }” को { $format } फ़ाइल के रूप में सहेजा जाएगा, लेकिन इसके नाम में कोई एक्सटेंशन नहीं है। हो सकता है अन्य ऐप इसे न खोलें।
image-choose-again = फिर से चुनें
image-save-as-is = जैसा है वैसा सहेजें
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = फ़्रेम { $total } में से { $current }
image-position = { $total } में से { $current }
image-edited = संपादित
# Toolbar tooltips.
image-sidebar = साइडबार
image-zoom-out = ज़ूम आउट करें
image-zoom-in = ज़ूम इन करें
image-zoom = { $percent }%
image-fit = विंडो में फ़िट करें
image-actual-size = वास्तविक आकार
image-undo = पूर्ववत करें
image-redo = फिर से करें
image-rotate-left = बाएँ घुमाएँ
image-rotate-right = दाएँ घुमाएँ
image-flip-horizontal = क्षैतिज रूप से पलटें
image-flip-vertical = लंबवत रूप से पलटें
image-select = आयताकार चयन
image-crop = चयन के अनुसार क्रॉप करें
image-adjust-size-tool = आकार समायोजित करें
image-adjust-color-tool = रंग समायोजित करें
# Tooltip and panel title.
image-inspector = इंस्पेक्टर
image-markup = मार्कअप
image-export = एक्सपोर्ट करें
image-settings = सेटिंग्स
# Panel titles.
image-adjust-color = रंग समायोजित करें
image-adjust-size = आकार समायोजित करें
# Adjust Color sliders.
image-exposure = एक्सपोज़र
image-contrast = कंट्रास्ट
image-saturation = सैचुरेशन
image-temperature = तापमान
image-tint = टिंट
image-sepia = सीपिया
image-sharpness = शार्पनेस
image-levels = लेवल
image-black-point = ब्लैक पॉइंट
image-midtones = मिडटोन
image-white-point = व्हाइट पॉइंट
image-reset-all = सभी रीसेट करें
# Adjust Size panel.
image-current-size = वर्तमान आकार: { $width } × { $height } पिक्सेल
image-width = चौड़ाई
image-height = ऊँचाई
image-scale-proportionally = अनुपात में स्केल करें
# Button that applies the new size.
image-resize = आकार बदलें
# Inspector panel.
image-inspector-loading = लोड हो रहा है…
image-file = फ़ाइल
image-format = फ़ॉर्मेट
image-dimensions-label = आयाम
image-pixels = { $width } × { $height } पिक्सेल
image-no-camera = कैमरे की कोई जानकारी नहीं।
image-location = स्थान
image-remove-location = स्थान की जानकारी हटाएँ
image-no-location = स्थान की कोई जानकारी नहीं।
image-keywords-description = कीवर्ड और विवरण
image-keywords-hint = कीवर्ड (अल्पविराम से अलग करें)
image-description = विवरण
image-keywords-unsupported = कीवर्ड JPEG, PNG और WebP फ़ाइलों में सहेजे जा सकते हैं।
# Heading over the earlier versions of the file.
image-revert-to = पिछले संस्करण पर लौटें
image-no-versions = कोई पुराना संस्करण नहीं।
image-revert = लौटें
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = मार्कअप एक्सपोर्ट किए बिना बंद करें?
image-close-body = { $count ->
    [one] किसी छवि पर मार्कअप तभी तक रहता है जब तक उसकी विंडो खुली रहती है। इसे रखने के लिए छवि एक्सपोर्ट करें: मार्कअप आपकी सहेजी गई कॉपी में बना दिया जाता है।
   *[other] छवियों पर मार्कअप तभी तक रहता है जब तक उनकी विंडो खुली रहती है। इसे रखने के लिए हर छवि एक्सपोर्ट करें: मार्कअप आपकी सहेजी गई कॉपी में बना दिया जाता है।
}
image-close-anyway = फिर भी बंद करें

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = पढ़ना रुक गया
markdown-read-failed = prev यह फ़ाइल नहीं पढ़ सकता
markdown-draw-failed = दस्तावेज़ रेंडर नहीं किया जा सका
# Under the export's size choices.
markdown-export-size = पूरा दस्तावेज़, { $width } × { $height } पिक्सेल
# Search results.
markdown-not-found = नहीं मिला
markdown-match = { $total } में से { $current }
# Placeholder of the search field.
markdown-search = खोजें
# Toolbar tooltips.
markdown-smaller-text = छोटा टेक्स्ट
markdown-larger-text = बड़ा टेक्स्ट
markdown-zoom = { $percent }%
markdown-actual-size = वास्तविक आकार
# Tooltip and panel title.
markdown-inspector = इंस्पेक्टर
markdown-export = एक्सपोर्ट करें
markdown-settings = सेटिंग्स
# Inspector headings and labels.
markdown-file = फ़ाइल
markdown-document = दस्तावेज़
markdown-words = शब्द
markdown-lines = पंक्तियाँ
markdown-pictures = छवियाँ

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = कैमरा
image-meta-exposure = एक्सपोज़र
image-meta-image = छवि
image-meta-make = निर्माता
image-meta-model = मॉडल
image-meta-lens = लेंस
image-meta-exposure-time = एक्सपोज़र समय
# The lens aperture, written like f/2.8.
image-meta-f-number = F-नंबर
image-meta-iso = ISO
image-meta-focal-length = फ़ोकल लंबाई
image-meta-exposure-bias = एक्सपोज़र बायस
image-meta-flash = फ़्लैश
image-meta-date-taken = खींचने की तारीख
image-meta-orientation = ओरिएंटेशन
image-meta-color-space = कलर स्पेस
image-meta-software = सॉफ़्टवेयर
image-meta-artist = कलाकार
image-meta-copyright = कॉपीराइट
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } सेकंड
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] सामान्य
    [2] क्षैतिज रूप से मिरर किया गया
    [3] 180° घुमाया गया
    [4] लंबवत रूप से मिरर किया गया
    [5] क्षैतिज रूप से मिरर किया गया, 90° वामावर्त घुमाया गया
    [6] 90° दक्षिणावर्त घुमाया गया
    [7] क्षैतिज रूप से मिरर किया गया, 90° दक्षिणावर्त घुमाया गया
    [8] 90° वामावर्त घुमाया गया
   *[other] अज्ञात ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] चला
   *[no] नहीं चला
}{ $mode ->
    [on] , अनिवार्य रूप से चालू
    [off] , बंद
    [auto] , स्वचालित
   *[unknown] {""}
}{ $redeye ->
    [yes] , रेड-आई रिडक्शन
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] कैलिब्रेट नहीं किया गया
   *[other] अन्य ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = दस्तावेज़ नहीं खोला जा सकता: { $detail }
error-pdf-page-out-of-range = पृष्ठ { $page } मौजूद नहीं है
error-pdf-password-protected = दस्तावेज़ पासवर्ड से सुरक्षित है; इसे खोलें और इसके बजाय इसके पृष्ठ कॉपी करें
error-pdf-no-pages = निकालने के लिए कोई पृष्ठ नहीं
error-pdf-crop-outside = क्रॉप क्षेत्र पृष्ठ से बाहर है
error-pdf-closed = दस्तावेज़ बंद हो गया
error-pdf-saved-unreadable = सहेजा गया दस्तावेज़ अब नहीं खुलता
error-image-read = फ़ाइल नहीं पढ़ी जा सकती: { $detail }
error-image-invalid = छवि क्षतिग्रस्त या अमान्य है: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = इस फ़ॉर्मेट को खोलने के लिए { $library } चाहिए, जो इंस्टॉल नहीं है
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format } छवियाँ अभी समर्थित नहीं हैं
error-image-encode = छवि एन्कोड नहीं की जा सकती: { $detail }
error-exif-malformed = EXIF डेटा दोषपूर्ण है
error-settings-read = सेटिंग्स नहीं पढ़ी जा सकतीं: { $detail }
error-settings-invalid = अमान्य सेटिंग्स: { $detail }
error-remove-location = स्थान हटाया नहीं जा सका: { $error }
error-location-unsupported = स्थान की जानकारी केवल JPEG, PNG, WebP और TIFF फ़ाइलों से हटाई जा सकती है
error-xmp-unsupported = कीवर्ड और विवरण केवल JPEG, PNG और WebP फ़ाइलों में सहेजे जा सकते हैं

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = कैमरा RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev के बारे में
menu-settings = सेटिंग्स…
menu-services = सेवाएँ
menu-hide = prev छिपाएँ
menu-hide-others = अन्य को छिपाएँ
menu-show-all = सभी दिखाएँ
menu-quit = prev बंद करें
menu-file = फ़ाइल
menu-open = खोलें…
menu-close = विंडो बंद करें
menu-export = एक्सपोर्ट करें…
menu-print = प्रिंट करें…
menu-edit = संपादित करें
menu-undo = पूर्ववत करें
menu-redo = फिर से करें
menu-cut = कट करें
menu-copy = कॉपी करें
menu-paste = पेस्ट करें
menu-select-all = सभी चुनें
menu-find = ढूँढें
menu-find-next = अगला ढूँढें
menu-find-previous = पिछला ढूँढें
menu-view = देखें
menu-hide-sidebar = साइडबार छिपाएँ
menu-thumbnails = थंबनेल
menu-contents = विषय-सूची
menu-notes = हाइलाइट और नोट
menu-bookmarks = बुकमार्क
menu-zoom-in = ज़ूम इन करें
menu-zoom-out = ज़ूम आउट करें
menu-actual-size = वास्तविक आकार
menu-zoom-to-fit = फ़िट करने के लिए ज़ूम करें
menu-inspector = इंस्पेक्टर दिखाएँ
menu-slideshow = स्लाइडशो
menu-full-screen = पूर्ण स्क्रीन में प्रवेश करें
menu-go = जाएँ
menu-next-page = अगला पृष्ठ
menu-previous-page = पिछला पृष्ठ
menu-go-to-page = पृष्ठ पर जाएँ…
menu-bookmark = बुकमार्क जोड़ें
menu-tools = टूल
menu-markup = मार्कअप टूलबार दिखाएँ
menu-rotate-left = बाएँ घुमाएँ
menu-rotate-right = दाएँ घुमाएँ
menu-crop = क्रॉप करें
menu-adjust-color = रंग समायोजित करें…
menu-window = विंडो
menu-minimize = छोटा करें
menu-zoom = ज़ूम करें
menu-bring-all-to-front = सभी को सामने लाएँ
