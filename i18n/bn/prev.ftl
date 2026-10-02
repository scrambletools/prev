# prev's interface text in Bengali (বাংলা), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = মার্কআপ, note = নোট, highlight = হাইলাইট,
# annotation = টীকা, redact/redaction = তথ্য গোপন করা/গোপন-চিহ্ন,
# inspector = পরিদর্শক, zoom in/out = জুম বাড়ান/জুম কমান, bookmark = বুকমার্ক,
# page = পৃষ্ঠা, document = নথি, image = ছবি, export = রপ্তানি,
# revert = আগের সংস্করণে ফেরানো, undo/redo = পূর্বাবস্থায় ফেরান/পুনরায় করুন.
# Buttons and menu items use the polite imperative (সংরক্ষণ করুন, বাতিল করুন);
# labels use nouns.

## Language

language-name = বাংলা

## Common

common-cancel = বাতিল করুন
common-close = বন্ধ করুন
common-save = সংরক্ষণ করুন

## Settings

settings-title = সেটিংস
settings-appearance = চেহারা
settings-colors = রং
settings-windows = উইন্ডো
settings-default-app = ডিফল্ট অ্যাপ
settings-default-app-label = prev দিয়ে ফাইল খুলুন
settings-default-app-note = PDF, ছবি, SVG অঙ্কন এবং Markdown ফাইল খোলার অ্যাপ হিসেবে prev-কে সেট করুন।
settings-default-app-note-windows = Windows শুধু নিজের সেটিংসেই ডিফল্ট অ্যাপ বেছে নিতে দেয়। এটি সেখানে prev-এর পৃষ্ঠা খোলে।
settings-default-app-note-macos = macOS প্রতিটি ধরনের জন্য আপনার নিশ্চিতকরণ চায়: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP এবং AVIF।
settings-default-app-status = { $total }টির মধ্যে { $set }টি ফাইলের ধরন prev দিয়ে খোলে।
settings-default-app-button = ডিফল্ট করুন
settings-default-app-button-windows = সেটিংস খুলুন
settings-default-app-no-entry = prev-এর ডেস্কটপ এন্ট্রি ইনস্টল করা নেই, তাই সিস্টেম এটি দিয়ে ফাইল খুলতে পারে না। কোনো প্যাকেজ থেকে বা scripts/install.sh দিয়ে prev ইনস্টল করুন।
settings-default-app-no-bundle = prev-কে ডিফল্ট করতে এটি prev.app থেকে খুলুন।
settings-default-app-failed = prev-কে ডিফল্ট করা যায়নি: { $error }
settings-storage = স্টোরেজ
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (ডেভেলপমেন্ট বিল্ড, { $build })

## Markup toolbar

markup-tool-select = নির্বাচন করুন
markup-tool-area = আয়তাকার নির্বাচন
markup-tool-sketch = স্কেচ
markup-tool-draw = আঁকুন
markup-tool-shapes = আকৃতি
markup-tool-text-box = টেক্সট বক্স
markup-tool-highlight = হাইলাইট করুন
markup-tool-note = নোট
markup-tool-sign = স্বাক্ষর করুন
markup-tool-redact = তথ্য গোপন করুন
markup-apply = প্রয়োগ করুন
markup-apply-redactions = গোপন-চিহ্ন প্রয়োগ করুন
markup-shape-style = আকৃতির শৈলী
markup-border-color = সীমানার রং
markup-fill-color = ভরাটের রং
markup-text-style = টেক্সটের শৈলী
markup-delete = মুছুন
markup-undo = পূর্বাবস্থায় ফেরান
markup-redo = পুনরায় করুন

## Markup menus

markup-shape-rectangle = আয়তক্ষেত্র
markup-shape-rounded-rectangle = গোলাকার কোণের আয়তক্ষেত্র
markup-shape-oval = ডিম্বাকৃতি
markup-shape-line = রেখা
markup-shape-arrow = তীর
markup-shape-star = তারা
markup-shape-polygon = বহুভুজ
markup-shape-speech-bubble = কথার বুদবুদ
markup-shape-loupe = বিবর্ধক কাচ
markup-shape-mask = মাস্ক
markup-style-highlight = হাইলাইট
markup-style-underline = নিম্নরেখা
markup-style-strikethrough = কাটা দাগ
markup-style-squiggly = ঢেউখেলানো দাগ
markup-menu-color = রং
markup-menu-font = ফন্ট
markup-menu-size = আকার
markup-menu-alignment = সারিবদ্ধকরণ
markup-line-width = { $width } pt
markup-dashed = ড্যাশযুক্ত

## Notes

markup-kind-note = নোট
markup-kind-text-box = টেক্সট বক্স
markup-kind-stamp = স্ট্যাম্প
markup-kind-redaction = গোপন-চিহ্ন
markup-kind-shape = আকৃতি
markup-note-delete = নোট মুছুন
markup-note-done = সম্পন্ন
markup-note-placeholder = একটি নোট লিখুন
markup-notes-empty = কোনো হাইলাইট বা নোট নেই
markup-notes-empty-hint = হাইলাইট, নোট এবং টেক্সট বক্স এখানে দেখা যায়।
markup-notes-page = পৃষ্ঠা { $page }

## Markup errors

markup-change-failed = নথিটি পরিবর্তন করা যায়নি: { $error }
markup-copy-area-failed = এলাকাটি কপি করা যায়নি: { $error }
markup-document-closed = নথিটি বন্ধ হয়ে গেছে
markup-render-area-failed = এলাকাটি রেন্ডার করা যায়নি
markup-copy-stopped = কপি করা থেমে গেছে

## Signatures

signature-menu-empty = এখনও কোনো স্বাক্ষর নেই।
signature-delete = স্বাক্ষর মুছুন
signature-create = স্বাক্ষর তৈরি করুন…
signature-dialog-title = স্বাক্ষর তৈরি করুন
signature-tab-draw = আঁকুন
signature-tab-type = টাইপ করুন
signature-tab-image = ছবি
signature-draw-hint = রেখার উপরে আপনার মাউস, পেন বা টাচপ্যাড দিয়ে স্বাক্ষর করুন।
signature-your-name = আপনার নাম
signature-image-hint = সাদা কাগজে করা আপনার স্বাক্ষরের একটি ছবি বা স্ক্যান বেছে নিন।
signature-choose-image = ছবি বেছে নিন…
signature-description = বিবরণ, যেমন পুরো নাম বা নামের আদ্যক্ষর
signature-clear = পরিষ্কার করুন
signature-ink = কালি
signature-thickness = পুরুত্ব
signature-sign-first = আগে স্বাক্ষর করুন, তারপর সংরক্ষণ করুন।
signature-default-name = স্বাক্ষর { $number }
signature-change-failed = স্বাক্ষর পরিবর্তন করা যায়নি: { $error }
signature-no-data-folder = কোনো ডেটা ফোল্ডার নেই: HOME সেট করা নেই
signature-removing-stopped = সরানো থেমে গেছে
signature-saving-stopped = সংরক্ষণ থেমে গেছে
signature-reading-stopped = পড়া থেমে গেছে
signature-not-an-image = ফাইলটি এমন কোনো ছবি নয় যা prev পড়তে পারে
signature-no-frames = ছবিটিতে কোনো ফ্রেম নেই
signature-not-found = ছবিতে কোনো স্বাক্ষর পাওয়া যায়নি

## Dragging

drag-pages-need-document = পৃষ্ঠা শুধু কোনো নথির উপরে ছাড়া যায়।
drag-image-unsupported = prev এই ছবিটি খুলতে পারে না।
drag-area-failed = এলাকাটি টেনে আনা যায়নি: { $error }
drag-pages-failed = পৃষ্ঠাগুলো টেনে আনা যায়নি: { $error }
drag-start-failed = টেনে আনা শুরু করা যায়নি।
drag-file-pages = পৃষ্ঠা
drag-file-one-page = { $name } (পৃষ্ঠা { $page })
drag-file-page-range = { $name } (পৃষ্ঠা { $first }–{ $last })
drag-file-image = ছবি
drop-pdf-title = এই নথিতে যোগ করবেন?
drop-pdf-body = “{ $name }” এই নথির শেষে যোগ করবেন, নাকি এটি আলাদা উইন্ডোতে খুলবেন?
drop-pdfs-body = { $count ->
    [one] এই { $count }টি PDF এই নথির শেষে যোগ করবেন, নাকি এটি আলাদা উইন্ডোতে খুলবেন?
   *[other] এই { $count }টি PDF এই নথির শেষে যোগ করবেন, নাকি সেগুলো আলাদা আলাদা উইন্ডোতে খুলবেন?
}
drop-pdf-add = শেষে যোগ করুন
drop-pdf-open = আলাদাভাবে খুলুন

## PDF window

pdf-opening = খোলা হচ্ছে…
pdf-open-failed = prev এই নথিটি খুলতে পারে না
pdf-no-pages = নথিটিতে কোনো পৃষ্ঠা নেই।
pdf-document-closed = নথিটি বন্ধ হয়ে গেছে
pdf-keep-original-failed = মূল সংস্করণটি রাখা যায়নি: { $error }
pdf-save-failed = সংরক্ষণ করা যায়নি: { $error }
pdf-nothing-to-paste = পেস্ট করার মতো কিছু নেই।
pdf-pasting-stopped = পেস্ট করা থেমে গেছে
pdf-file-dialog-failed = ফাইল ডায়ালগ দেখানো যায়নি: { $error }
pdf-bookmarks-no-home = বুকমার্ক সংরক্ষণ করা যাবে না: HOME সেট করা নেই
pdf-bookmarks-save-failed = বুকমার্ক সংরক্ষণ করা যায়নি: { $error }
pdf-bookmark-page = পৃষ্ঠা { $page }

pdf-password-protected = “{ $name }” পাসওয়ার্ড দিয়ে সুরক্ষিত
pdf-password = পাসওয়ার্ড
pdf-password-wrong = পাসওয়ার্ড ভুল। আবার চেষ্টা করুন।
pdf-unlock = আনলক করুন

pdf-sidebar = সাইডবার
pdf-page-of = / { $count }
pdf-zoom-out = জুম কমান
pdf-zoom-in = জুম বাড়ান
pdf-zoom-percent = { $percent }%
pdf-fit-page = পৃষ্ঠায় মানানসই
pdf-fit-width = প্রস্থে মানানসই
pdf-actual-size = প্রকৃত আকার
pdf-view-continuous = একটানা স্ক্রল
pdf-view-single-page = একক পৃষ্ঠা
pdf-view-two-pages = দুই পৃষ্ঠা
pdf-undo = পূর্বাবস্থায় ফেরান
pdf-redo = পুনরায় করুন
pdf-rotate-left = বাঁয়ে ঘোরান
pdf-rotate-right = ডানে ঘোরান
pdf-inspector = পরিদর্শক
pdf-markup = মার্কআপ
pdf-export = রপ্তানি করুন
pdf-settings = সেটিংস

pdf-search = অনুসন্ধান
pdf-search-not-found = পাওয়া যায়নি
pdf-searching = অনুসন্ধান করা হচ্ছে…
pdf-search-match = { $total }টির মধ্যে { $current }
pdf-search-match-more = { $total }+টির মধ্যে { $current }

pdf-inspector-file = ফাইল
pdf-inspector-document = নথি
pdf-inspector-pages = পৃষ্ঠা
pdf-inspector-title = শিরোনাম
pdf-inspector-author = লেখক
pdf-inspector-subject = বিষয়
pdf-inspector-keywords = কীওয়ার্ড
pdf-inspector-created = তৈরি হয়েছে
pdf-inspector-modified = পরিবর্তিত
pdf-inspector-application = অ্যাপ্লিকেশন
pdf-inspector-producer = PDF প্রস্তুতকারক
pdf-inspector-version = সংস্করণ
pdf-inspector-security = নিরাপত্তা
pdf-inspector-not-encrypted = এনক্রিপ্ট করা নেই
pdf-inspector-encrypted = এনক্রিপ্ট করা ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } পৃষ্ঠা
   *[other] { $count } পৃষ্ঠা
}
pdf-inspector-page-size = পৃষ্ঠার আকার
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } মিমি ({ $width_in } × { $height_in } ইঞ্চি)
pdf-loading = লোড হচ্ছে…

pdf-tab-pages = পৃষ্ঠা
pdf-tab-contents = সূচিপত্র
pdf-tab-notes = হাইলাইট ও নোট
pdf-tab-bookmarks = বুকমার্ক
pdf-no-outline = কোনো সূচিপত্র নেই
pdf-no-outline-detail = এই নথিতে কোনো রূপরেখা নেই।
pdf-no-bookmarks = কোনো বুকমার্ক নেই
pdf-no-bookmarks-detail = কোনো পৃষ্ঠা বুকমার্ক করতে { $keys } চাপুন।
pdf-no-bookmarks-detail-unbound = বুকমার্ক করা পৃষ্ঠাগুলো এখানে দেখা যায়।
pdf-remove-bookmark = বুকমার্ক সরান

## Page editing

pages-menu = পৃষ্ঠা
pages-insert-blank = ফাঁকা পৃষ্ঠা যোগ করুন
pages-insert-file = ফাইল থেকে যোগ করুন…
pages-copy = { $count ->
    [one] পৃষ্ঠা কপি করুন
   *[other] পৃষ্ঠাগুলো কপি করুন
}
pages-paste = { $count ->
    [one] পৃষ্ঠা পেস্ট করুন
   *[other] { $count }টি পৃষ্ঠা পেস্ট করুন
}
pages-crop = নির্বাচন অনুযায়ী ক্রপ করুন
pages-select-all = সব পৃষ্ঠা নির্বাচন করুন
pages-delete = { $count ->
    [one] পৃষ্ঠা মুছুন
   *[other] পৃষ্ঠাগুলো মুছুন
}
pages-apply-redactions = গোপন-চিহ্ন প্রয়োগ করুন…
pages-no-copied = পেস্ট করার মতো কোনো কপি করা পৃষ্ঠা নেই।
pages-copied = { $count ->
    [one] { $count }টি পৃষ্ঠা কপি করা হয়েছে।
   *[other] { $count }টি পৃষ্ঠা কপি করা হয়েছে।
}
pages-copy-failed = পৃষ্ঠাগুলো কপি করা যায়নি: { $error }
pages-reading-stopped = পড়া থেমে গেছে
pages-image-unreadable = এমন কোনো ছবি নয় যা prev পড়তে পারে
pages-read-failed = ফাইলটি পড়া যায়নি: { $error }
pages-at-least-one = একটি নথিতে অন্তত একটি পৃষ্ঠা থাকতে হবে।
pages-crop-needs-area = আগে আয়তাকার নির্বাচন টুল দিয়ে একটি এলাকা বেছে নিন।
pages-change-failed = পৃষ্ঠাগুলো পরিবর্তন করা যায়নি: { $error }
pages-no-redactions = প্রয়োগ করার মতো কোনো গোপন-চিহ্ন ছিল না।
pages-redactions-applied = { $count ->
    [one] { $count }টি গোপন-চিহ্ন প্রয়োগ করা হয়েছে।
   *[other] { $count }টি গোপন-চিহ্ন প্রয়োগ করা হয়েছে।
}
pages-forget-versions-failed = আগের সংস্করণগুলো মোছা যায়নি: { $error }
pages-redact-title = গোপন-চিহ্ন প্রয়োগ করবেন?
pages-redact-body = { $count ->
    [one] চিহ্নের নিচের টেক্সট, ছবি ও অঙ্কন নথি থেকে চিরতরে সরিয়ে ফেলা হয়, এবং চিহ্নটি একটি কালো বাক্সে পরিণত হয়। এটি পূর্বাবস্থায় ফেরানো যায় না, এবং prev এই ফাইলের যে আগের সংস্করণগুলো রাখে সেগুলো মুছে ফেলা হয়।
   *[other] { $count }টি চিহ্নের নিচের টেক্সট, ছবি ও অঙ্কন নথি থেকে চিরতরে সরিয়ে ফেলা হয়, এবং চিহ্নগুলো কালো বাক্সে পরিণত হয়। এটি পূর্বাবস্থায় ফেরানো যায় না, এবং prev এই ফাইলের যে আগের সংস্করণগুলো রাখে সেগুলো মুছে ফেলা হয়।
}
pages-redact-apply = প্রয়োগ করুন

## PDF export

pages-export-title = রপ্তানি করুন
pages-export-format = ফরম্যাট
pages-export-reduce = ফাইলের আকার কমান (150 dpi-তে ছবি)
pages-export-flatten = টীকা ও ফর্মের ঘর পৃষ্ঠার সঙ্গে মিশিয়ে দিন
pages-export-flatten-detail = মার্কআপ ও পূরণ করা ঘরগুলো পৃষ্ঠার অংশ হয়ে যায় এবং আর সম্পাদনা করা যায় না। যে গোপন-চিহ্ন এখনও প্রয়োগ করা হয়নি সেগুলো বাদ দেওয়া হয়।
pages-export-encrypt = পাসওয়ার্ড দিয়ে এনক্রিপ্ট করুন
pages-export-password = পাসওয়ার্ড
pages-export-verify-password = পাসওয়ার্ড যাচাই করুন
pages-export-resolution = রেজোলিউশন
pages-export-dpi = { $dpi } dpi
pages-export-quality = গুণমান
pages-export-quality-low = নিম্ন
pages-export-quality-medium = মাঝারি
pages-export-quality-high = উচ্চ
pages-export-quality-best = সর্বোত্তম
pages-export-one-file = সব পৃষ্ঠা একটি ফাইলে যায়।
pages-export-file-per-page = প্রতিটি পৃষ্ঠা আলাদা ফাইল হিসেবে সংরক্ষিত হয়, আপনার বেছে নেওয়া নামের পরে নম্বর দিয়ে।
pages-export-selected-only = { $count ->
    [one] শুধু নির্বাচিত পৃষ্ঠা
   *[other] শুধু নির্বাচিত { $count }টি পৃষ্ঠা
}
pages-export-choose = রপ্তানি করুন…
pages-export-no-password = একটি পাসওয়ার্ড লিখুন।
pages-export-password-mismatch = পাসওয়ার্ড মিলছে না।
pages-export-file-name = { $name } (রপ্তানি করা)
pages-export-untitled = নথি
pages-export-same-file = একটি নতুন ফাইলে রপ্তানি করুন; এই নথিটি নিজে থেকেই সংরক্ষিত হয়।
pages-export-exporting = “{ $name }” রপ্তানি করা হচ্ছে…
pages-export-done = “{ $name }” রপ্তানি করা হয়েছে।
pages-export-done-images = { $count }টি ছবি রপ্তানি করা হয়েছে।
pages-export-failed = রপ্তানি করা যায়নি: { $error }
pages-export-stopped = রপ্তানি থেমে গেছে

## Start window

app-start-hint = একটি PDF, ছবি, SVG বা Markdown ফাইল খুলুন অথবা এখানে ছেড়ে দিন।
app-start-open = খুলুন…
app-title-dev = { $title } (ডেভেলপমেন্ট)
app-viewer-missing = { $kind }: এই ভিউয়ার এখনও তৈরি হয়নি।
app-cannot-open = prev এই ধরনের ফাইল খুলতে পারে না।
app-cannot-read = prev এই ফাইলটি পড়তে পারে না: { $error }
app-kind-pdf = PDF নথি
app-kind-image = { $format } ছবি
app-kind-svg = SVG অঙ্কন
app-kind-markdown = Markdown নথি
app-file-dialog-failed = ফাইল ডায়ালগ দেখানো যায়নি: { $error }

## Actions

action-open = খুলুন
action-settings = সেটিংস

## Toolbar

app-toolbar-keep-shown = টুলবার সবসময় দেখান
app-toolbar-auto-hide = পয়েন্টার সরে গেলে টুলবার লুকান
app-toolbar-more = আরও

## File facts

app-fact-name = নাম
app-fact-folder = ফোল্ডার
app-fact-size = আকার
app-fact-modified = পরিবর্তিত
app-size-bytes = { $count } বাইট
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = অবৈধ লিঙ্ক { $uri }: { $error }
app-link-open-failed = { $uri } খোলা যায়নি: { $error }
app-paste-needs-wl-clipboard = ছবি পেস্ট করতে wl-clipboard ইনস্টল করুন
app-copy-needs-wl-clipboard = ছবি কপি করতে wl-clipboard ইনস্টল করুন
app-copy-no-pixels = এলাকাটিতে কোনো পিক্সেল নেই
app-copy-no-input = wl-copy কোনো ইনপুট পায়নি
app-copy-failed = wl-copy ব্যর্থ হয়েছে
app-clipboard-open-failed = ক্লিপবোর্ড খোলা যায়নি: { $error }
app-copy-image-failed = ছবিটি কপি করা যায়নি: { $error }

## Printing

print-failed = প্রিন্ট করা যায়নি: { $error }
print-stopped = প্রিন্ট করা থেমে গেছে
print-unavailable = এই সিস্টেমে এখনও প্রিন্ট করা যায় না।
print-no-window = প্রিন্ট করা যায়নি: প্রিন্ট ডায়ালগ দেখানোর মতো কোনো উইন্ডো নেই
print-dialog-failed = প্রিন্ট ডায়ালগ দেখানো যায়নি: { $error }
print-job-not-started = প্রিন্টার কাজটি শুরু করেনি
print-printer-stopped = প্রিন্টার থেমে গেছে

## File dialogs

dialog-open = খুলুন
dialog-filter-all = সব সমর্থিত ফাইল
dialog-filter-pdf = PDF নথি
dialog-filter-images = ছবি
dialog-filter-svg = SVG অঙ্কন
dialog-filter-markdown = Markdown
dialog-choose-signatures = স্বাক্ষরের ফোল্ডার বেছে নিন
dialog-choose-versions = সংস্করণ ইতিহাসের ফোল্ডার বেছে নিন
dialog-choose-bookmarks = বুকমার্ক ফাইল বেছে নিন

## Command line

usage-help =
    ব্যবহার: prev [FILE]...
            prev --mcp

    PDF ও ছবি দেখুন এবং সম্পাদনা করুন। ফাইলগুলো চালু থাকা prev-এর উইন্ডোতে
    খোলে, যা প্রয়োজনে নিজেই চালু হয়।

    বিকল্প:
      -h, --help     এই সহায়তা দেখান
      -V, --version  সংস্করণ দেখান
          --mcp      stdin ও stdout-এ MCP চালান, যাতে AI এজেন্ট চালু থাকা prev
                     নিয়ন্ত্রণ করতে পারে

## Settings, continued

settings-language = ভাষা
settings-language-system = সিস্টেম ডিফল্ট: { $language }
settings-input-language = ইনপুট ভাষা
settings-input-language-system = কীবোর্ড লেআউট অনুসরণ করুন
settings-input-language-note = খালি টেক্সট ফিল্ড কোন দিক থেকে শুরু হবে তা ঠিক করে। আপনার টাইপ করা টেক্সট নিজের দিক বজায় রাখে।

settings-appearance-system = সিস্টেম
settings-appearance-light = হালকা
settings-appearance-dark = গাঢ়
settings-system-accent = সিস্টেমের অ্যাকসেন্ট রং ব্যবহার করুন
settings-omarchy-note = রংগুলো “{ $theme }”-এর অ্যাকসেন্ট থেকে তৈরি।
settings-system-accent-note = রংগুলো সিস্টেমের অ্যাকসেন্ট রং থেকে তৈরি।
settings-system-accent-none = সিস্টেমে কোনো অ্যাকসেন্ট রং নেই, তাই prev নিচে বেছে নেওয়া রং ব্যবহার করে।
settings-accent-chosen-note = রংগুলো নিচে বেছে নেওয়া রং থেকে তৈরি।
settings-auto-hide = পয়েন্টার সরে গেলে টুলবার লুকান
settings-auto-hide-note = টুলবার নথির উপরে ভেসে থাকে এবং পয়েন্টার উইন্ডোর বাইরে থাকলে সরে যায়।
settings-animations = অ্যানিমেশন
settings-animations-note = সরে যাওয়া বার ও প্যানেল, বড় হয়ে ওঠা ডায়ালগ এবং লাফিয়ে ওঠা বোতাম।
settings-animations-reduced = সিস্টেম কম গতি চাইলে বন্ধ থাকে।
settings-corner-radius = কোণের ব্যাসার্ধ
settings-corner-radius-note = ডায়ালগ ও ভাসমান টুলবারের জন্য।
settings-corner-radius-value = { $radius } px
settings-overlay = ওভারলের স্বচ্ছতা
settings-overlay-note = ভাসমান টুলবারের ভেতর দিয়ে পৃষ্ঠার কতটা দেখা যায়।
settings-overlay-value = { $percent }%
settings-storage-signatures = স্বাক্ষরের ফোল্ডার
settings-storage-versions = সংস্করণ ইতিহাসের ফোল্ডার
settings-storage-bookmarks = বুকমার্ক ফাইল
settings-storage-apply = প্রয়োগ করুন
settings-storage-choose = বেছে নিন…
settings-storage-note = পুরোনো জায়গায় আগে থেকে রাখা ফাইলগুলো সেখানেই থাকে; সেগুলো ব্যবহার চালিয়ে যেতে নতুন জায়গায় সরিয়ে নিন। prev অ্যাপের সেটিংস { $file }-এ সংরক্ষিত হয়।
settings-save-failed = সেটিংস সংরক্ষণ করা যায়নি: { $error }
settings-no-location = সেটিংসের কোনো অবস্থান নেই: HOME সেট করা নেই
settings-full-path = পুরো পাথ ব্যবহার করুন, যেমন ~/Documents/prev।
settings-path-is-folder = { $path } একটি ফোল্ডার, ফাইল নয়।
settings-folder-missing = { $path } নামে কোনো ফোল্ডার নেই। আগে এটি তৈরি করুন, অথবা একটি ফোল্ডার বেছে নিন।
settings-path-is-file = { $path } একটি ফাইল, ফোল্ডার নয়।
settings-cannot-write = prev { $path }-এ লিখতে পারে না: { $error }।

## Export dialog

export-title = রপ্তানি করুন
export-format = ফরম্যাট
export-quality = গুণমান
export-size = আকার
export-choose = রপ্তানি করুন…
export-format-webp = WebP (ক্ষতিহীন)
export-format-unknown = ছবি
export-quality-low = নিম্ন
export-quality-medium = মাঝারি
export-quality-high = উচ্চ
export-quality-best = সর্বোত্তম
export-size-actual = প্রকৃত আকার
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } পিক্সেল
export-dialog-failed = সংরক্ষণ ডায়ালগ দেখানো যায়নি: { $error }
export-done = { $path }-এ রপ্তানি করা হয়েছে
export-failed = রপ্তানি করা যায়নি: { $error }
export-stopped = রপ্তানি থেমে গেছে

## Image window

image-marked-no-edit = মার্কআপ থাকা ছবি সম্পাদনা করা যায় না। মার্কআপ রাখতে রপ্তানি করুন, অথবা সেটি মুছে মার্কআপ বার বন্ধ করুন।

image-loading-stopped = লোড করা থেমে গেছে
image-reverting-stopped = আগের সংস্করণে ফেরানো থেমে গেছে
image-rendering-stopped = রেন্ডার করা থেমে গেছে
image-saving-stopped = সংরক্ষণ থেমে গেছে
image-markup-stopped = মার্কআপ থেমে গেছে
image-no-version-store = সংস্করণ রাখার কোনো জায়গা নেই
image-revert-failed = আগের সংস্করণে ফেরানো যায়নি: { $error }
image-read-failed = { $path } পড়া যায়নি: { $error }
image-keep-original-failed = মূল সংস্করণটি রাখা যায়নি: { $error }
image-save-failed = { $path } সংরক্ষণ করা যায়নি: { $error }
image-markup-start-failed = মার্কআপ শুরু করা যায়নি: { $error }
image-cannot-edit = অ্যানিমেশন ও SVG অঙ্কন সম্পাদনা করা যায় না।
image-cannot-mark-up = অ্যানিমেশন ও SVG অঙ্কনে মার্কআপ করা যায় না।
image-mark-up-wait = সম্পাদনা শেষ হওয়া পর্যন্ত অপেক্ষা করুন, তারপর মার্কআপ করুন।
image-crop-needs-selection = আগে টেনে একটি এলাকা নির্বাচন করুন (নির্বাচন টুল), তারপর ক্রপ করুন।
image-size-needed = পিক্সেলে প্রস্থ ও উচ্চতা লিখুন।
image-cannot-save-format = “{ $name }”-এর পরিবর্তনগুলো এর ফরম্যাটে সংরক্ষণ করা যায় না। রপ্তানি ({ $keys }) ব্যবহার করুন।
image-cannot-save-format-unbound = “{ $name }”-এর পরিবর্তনগুলো এর ফরম্যাটে সংরক্ষণ করা যায় না। রপ্তানি ব্যবহার করুন।
image-cannot-export-animation = অ্যানিমেশন এখনও রপ্তানি করা যায় না।
image-drop-pages = পৃষ্ঠা শুধু কোনো নথির উপরে ছাড়া যায়।
image-drag-failed = টেনে আনা শুরু করা যায়নি।
image-picture-save-failed = আপনার ডাউনলোড ফোল্ডারে ছবিটি সংরক্ষণ করা যায়নি।
image-open-failed = prev এই ছবিটি খুলতে পারে না
image-opening = খোলা হচ্ছে…
image-name-mismatch-title = নামটি ফরম্যাটের সাথে মেলে না
image-name-mismatch = “{ $name }” একটি { $format } ফাইল হিসেবে সংরক্ষিত হবে, কিন্তু এর নাম .{ $extension } দিয়ে শেষ হয়েছে। অন্য অ্যাপ এটি নাও খুলতে পারে।
image-name-mismatch-no-extension = “{ $name }” একটি { $format } ফাইল হিসেবে সংরক্ষিত হবে, কিন্তু এর নামে কোনো এক্সটেনশন নেই। অন্য অ্যাপ এটি নাও খুলতে পারে।
image-choose-again = আবার বেছে নিন
image-save-as-is = যেমন আছে তেমন সংরক্ষণ করুন
image-dimensions = { $width } × { $height }
image-frame-position = ফ্রেম { $total }টির মধ্যে { $current }
image-position = { $total }টির মধ্যে { $current }
image-edited = সম্পাদিত
image-sidebar = সাইডবার
image-zoom-out = জুম কমান
image-zoom-in = জুম বাড়ান
image-zoom = { $percent }%
image-fit = উইন্ডোতে মানানসই
image-actual-size = প্রকৃত আকার
image-undo = পূর্বাবস্থায় ফেরান
image-redo = পুনরায় করুন
image-rotate-left = বাঁয়ে ঘোরান
image-rotate-right = ডানে ঘোরান
image-flip-horizontal = অনুভূমিকভাবে উল্টান
image-flip-vertical = উল্লম্বভাবে উল্টান
image-select = আয়তাকার নির্বাচন
image-crop = নির্বাচন অনুযায়ী ক্রপ করুন
image-adjust-size-tool = আকার সামঞ্জস্য করুন
image-adjust-color-tool = রং সামঞ্জস্য করুন
image-inspector = পরিদর্শক
image-markup = মার্কআপ
image-export = রপ্তানি করুন
image-settings = সেটিংস
image-adjust-color = রং সামঞ্জস্য করুন
image-adjust-size = আকার সামঞ্জস্য করুন
image-exposure = এক্সপোজার
image-contrast = কনট্রাস্ট
image-saturation = স্যাচুরেশন
image-temperature = তাপমাত্রা
image-tint = টিন্ট
image-sepia = সেপিয়া
image-sharpness = তীক্ষ্ণতা
image-levels = লেভেল
image-black-point = কালো বিন্দু
image-midtones = মিডটোন
image-white-point = সাদা বিন্দু
image-reset-all = সব রিসেট করুন
image-current-size = বর্তমান আকার: { $width } × { $height } পিক্সেল
image-width = প্রস্থ
image-height = উচ্চতা
image-scale-proportionally = অনুপাত বজায় রেখে বদলান
image-resize = আকার বদলান
image-inspector-loading = লোড হচ্ছে…
image-file = ফাইল
image-format = ফরম্যাট
image-dimensions-label = মাত্রা
image-pixels = { $width } × { $height } পিক্সেল
image-no-camera = ক্যামেরার কোনো তথ্য নেই।
image-location = অবস্থান
image-remove-location = অবস্থানের তথ্য সরান
image-no-location = অবস্থানের কোনো তথ্য নেই।
image-keywords-description = কীওয়ার্ড ও বিবরণ
image-keywords-hint = কীওয়ার্ড, কমা দিয়ে আলাদা করা
image-description = বিবরণ
image-keywords-unsupported = কীওয়ার্ড JPEG, PNG ও WebP ফাইলে সংরক্ষণ করা যায়।
image-revert-to = আগের সংস্করণে ফেরান
image-no-versions = কোনো আগের সংস্করণ নেই।
image-revert = ফেরান
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = মার্কআপ রপ্তানি না করেই বন্ধ করবেন?
image-close-body = { $count ->
    [one] ছবির মার্কআপ শুধু তার উইন্ডো খোলা থাকা পর্যন্ত থাকে। এটি রাখতে ছবিটি রপ্তানি করুন: মার্কআপটি আপনার সংরক্ষিত কপিতে এঁকে দেওয়া হয়।
   *[other] ছবিগুলোর মার্কআপ শুধু তাদের উইন্ডো খোলা থাকা পর্যন্ত থাকে। এটি রাখতে প্রতিটি ছবি রপ্তানি করুন: মার্কআপটি আপনার সংরক্ষিত কপিতে এঁকে দেওয়া হয়।
}
image-close-anyway = তবুও বন্ধ করুন

## Markdown

markdown-reading-stopped = পড়া থেমে গেছে
markdown-read-failed = prev এই ফাইলটি পড়তে পারে না
markdown-draw-failed = নথিটি আঁকা যায়নি
markdown-export-size = পুরো নথি, { $width } × { $height } পিক্সেল
markdown-not-found = পাওয়া যায়নি
markdown-match = { $total }টির মধ্যে { $current }
markdown-search = অনুসন্ধান
markdown-smaller-text = ছোট লেখা
markdown-larger-text = বড় লেখা
markdown-zoom = { $percent }%
markdown-actual-size = প্রকৃত আকার
markdown-inspector = পরিদর্শক
markdown-export = রপ্তানি করুন
markdown-settings = সেটিংস
markdown-file = ফাইল
markdown-document = নথি
markdown-words = শব্দ
markdown-lines = লাইন
markdown-pictures = ছবি

## Image details

image-meta-camera = ক্যামেরা
image-meta-exposure = এক্সপোজার
image-meta-image = ছবি
image-meta-make = নির্মাতা
image-meta-model = মডেল
image-meta-lens = লেন্স
image-meta-exposure-time = এক্সপোজারের সময়
image-meta-f-number = F-নম্বর
image-meta-iso = ISO
image-meta-focal-length = ফোকাল দৈর্ঘ্য
image-meta-exposure-bias = এক্সপোজার বায়াস
image-meta-flash = ফ্ল্যাশ
image-meta-date-taken = তোলার তারিখ
image-meta-orientation = অভিমুখ
image-meta-color-space = কালার স্পেস
image-meta-software = সফটওয়্যার
image-meta-artist = শিল্পী
image-meta-copyright = কপিরাইট
image-meta-seconds = { $value } সেকেন্ড
image-meta-millimeters = { $value } মিমি
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] স্বাভাবিক
    [2] অনুভূমিকভাবে মিরর করা
    [3] 180° ঘোরানো
    [4] উল্লম্বভাবে মিরর করা
    [5] অনুভূমিকভাবে মিরর করা, 90° ঘড়ির কাঁটার বিপরীতে ঘোরানো
    [6] 90° ঘড়ির কাঁটার দিকে ঘোরানো
    [7] অনুভূমিকভাবে মিরর করা, 90° ঘড়ির কাঁটার দিকে ঘোরানো
    [8] 90° ঘড়ির কাঁটার বিপরীতে ঘোরানো
   *[other] অজানা ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] জ্বলেছে
   *[no] জ্বলেনি
}{ $mode ->
    [on] , জোর করে চালু
    [off] , বন্ধ
    [auto] , স্বয়ংক্রিয়
   *[unknown] {""}
}{ $redeye ->
    [yes] , রেড-আই হ্রাস
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] ক্যালিব্রেট করা নয়
   *[other] অন্যান্য ({ $code })
}

## Errors

error-pdf-open = নথি খোলা যায় না: { $detail }
error-pdf-page-out-of-range = পৃষ্ঠা { $page } নেই
error-pdf-password-protected = নথিটি পাসওয়ার্ড দিয়ে সুরক্ষিত; এটি খুলে এর বদলে এর পৃষ্ঠাগুলো কপি করুন
error-pdf-no-pages = বের করার মতো কোনো পৃষ্ঠা নেই
error-pdf-crop-outside = ক্রপ এলাকাটি পৃষ্ঠার বাইরে
error-pdf-closed = নথি বন্ধ হয়ে গেছে
error-pdf-saved-unreadable = সংরক্ষিত নথিটি আর খোলে না
error-image-read = ফাইলটি পড়া যায় না: { $detail }
error-image-invalid = ছবিটি ক্ষতিগ্রস্ত বা অবৈধ: { $detail }
error-image-missing-library = এই ফরম্যাট খুলতে { $library } দরকার, যা ইনস্টল করা নেই
error-image-unsupported = { $format } ছবি এখনও সমর্থিত নয়
error-image-encode = ছবিটি এনকোড করা যায় না: { $detail }
error-exif-malformed = EXIF ডেটা ত্রুটিপূর্ণ
error-settings-read = সেটিংস পড়া যায় না: { $detail }
error-settings-invalid = অবৈধ সেটিংস: { $detail }
error-remove-location = অবস্থান সরানো যায়নি: { $error }
error-location-unsupported = অবস্থানের তথ্য JPEG, PNG, WebP ও TIFF ফাইল থেকে সরানো যায়
error-xmp-unsupported = কীওয়ার্ড ও বিবরণ শুধু JPEG, PNG ও WebP ফাইলে সংরক্ষণ করা যায়

## Formats

format-camera-raw = ক্যামেরা RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev সম্পর্কে
menu-settings = সেটিংস…
menu-services = পরিষেবা
menu-hide = prev লুকান
menu-hide-others = অন্যগুলো লুকান
menu-show-all = সব দেখান
menu-quit = prev থেকে প্রস্থান করুন
menu-file = ফাইল
menu-open = খুলুন…
menu-close = উইন্ডো বন্ধ করুন
menu-export = রপ্তানি করুন…
menu-print = প্রিন্ট করুন…
menu-edit = সম্পাদনা
menu-undo = পূর্বাবস্থায় ফেরান
menu-redo = পুনরায় করুন
menu-cut = কাট করুন
menu-copy = কপি করুন
menu-paste = পেস্ট করুন
menu-select-all = সব নির্বাচন করুন
menu-find = খুঁজুন
menu-find-next = পরেরটি খুঁজুন
menu-find-previous = আগেরটি খুঁজুন
menu-view = দেখুন
menu-hide-sidebar = সাইডবার লুকান
menu-thumbnails = থাম্বনেইল
menu-contents = সূচিপত্র
menu-notes = হাইলাইট ও নোট
menu-bookmarks = বুকমার্ক
menu-zoom-in = জুম বাড়ান
menu-zoom-out = জুম কমান
menu-actual-size = প্রকৃত আকার
menu-zoom-to-fit = মানানসই জুম
menu-inspector = পরিদর্শক দেখান
menu-slideshow = স্লাইডশো
menu-full-screen = পূর্ণ স্ক্রিনে যান
menu-go = যান
menu-next-page = পরের পৃষ্ঠা
menu-previous-page = আগের পৃষ্ঠা
menu-go-to-page = পৃষ্ঠায় যান…
menu-bookmark = বুকমার্ক যোগ করুন
menu-tools = টুল
menu-markup = মার্কআপ টুলবার দেখান
menu-rotate-left = বাঁয়ে ঘোরান
menu-rotate-right = ডানে ঘোরান
menu-crop = ক্রপ করুন
menu-adjust-color = রং সামঞ্জস্য করুন…
menu-window = উইন্ডো
menu-minimize = ছোট করুন
menu-zoom = জুম
menu-bring-all-to-front = সব সামনে আনুন

## Outside control

settings-outside-control = বাইরের নিয়ন্ত্রণ
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = সাধারণ
settings-tab-agents = এজেন্ট
settings-allow-outside-control = বাইরের নিয়ন্ত্রণের অনুমতি দিন
settings-allow-outside-control-note = Claude Code-এর মতো AI এজেন্ট prev --mcp ব্যবহার করে prev-এ আপনার ফাইল পড়তে ও পরিবর্তন করতে পারে। প্রতিটি নতুন এজেন্টের আগে prev জিজ্ঞাসা করে।
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = অনুমোদিত: { $agents }
settings-forget-agents = ভুলে যান
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent }-কে prev নিয়ন্ত্রণ করার অনুমতি দেবেন?
agent-prompt-body = { $agent } prev-এর বাইরের নিয়ন্ত্রণ ব্যবহার করে আপনার খোলা ফাইল পড়তে ও পরিবর্তন করতে চায়। সেটিংসে গিয়ে বাইরের নিয়ন্ত্রণ বন্ধ করতে পারেন।
agent-prompt-allow = অনুমতি দিন
agent-prompt-deny = অনুমতি দেবেন না
settings-ask-before-note = কোনো এজেন্ট এগুলো করার আগে জিজ্ঞাসা করুন:
settings-ask-reading = ফাইল পড়া
settings-ask-viewing = দৃশ্য বা উইন্ডো পরিবর্তন করা
settings-ask-marking-up = ফাইলে মার্কআপ করা
settings-ask-editing = ফাইল সম্পাদনা করা
settings-ask-signing = ফাইলে স্বাক্ষর করা
settings-ask-redacting = গোপন-চিহ্ন প্রয়োগ করা
settings-ask-exporting = ফাইল রপ্তানি করা
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = { $agent }-কে এই ফাইলটি পড়তে দেবেন?
agent-ask-view = { $agent }-কে দৃশ্য পরিবর্তন করতে দেবেন?
agent-ask-markup = { $agent }-কে এই ফাইলে মার্কআপ করতে দেবেন?
agent-ask-edit = { $agent }-কে এই ফাইলটি সম্পাদনা করতে দেবেন?
agent-ask-sign = { $agent }-কে এই ফাইলে স্বাক্ষর করতে দেবেন?
agent-ask-redact = { $agent }-কে গোপন-চিহ্ন প্রয়োগ করতে দেবেন?
agent-ask-export = { $agent }-কে এই ফাইলটি রপ্তানি করতে দেবেন?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } “{ $tool }” ব্যবহার করতে চায়। prev কোন বিষয়ে জিজ্ঞাসা করবে, তা সেটিংসে বেছে নেওয়া যায়।
agent-ask-final = এটি পূর্বাবস্থায় ফেরানো যাবে না।
