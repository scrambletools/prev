# prev's interface text in Greek (Ελληνικά), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = σήμανση, note = σημείωση, highlight = επισήμανση,
# annotation = σχολιασμός, redact/redaction = απόκρυψη, inspector = επιθεωρητής,
# zoom in/out = μεγέθυνση/σμίκρυνση, bookmark = σελιδοδείκτης, page = σελίδα,
# redaction mark = σήμανση απόκρυψης, export = εξαγωγή, crop = περικοπή.
# Buttons and menu items use the action noun (Αποθήκευση, Ακύρωση, Κλείσιμο);
# questions end with the Greek question mark (;).

## Language

language-name = Ελληνικά

## Common

common-cancel = Ακύρωση
common-close = Κλείσιμο
common-save = Αποθήκευση

## Settings

settings-title = Ρυθμίσεις
settings-appearance = Εμφάνιση
settings-colors = Χρώματα
settings-windows = Παράθυρα
settings-default-app = Προεπιλεγμένη εφαρμογή
settings-default-app-label = Άνοιγμα αρχείων με το prev
settings-default-app-note = Ορισμός του prev ως της εφαρμογής που ανοίγει αρχεία PDF, εικόνες, σχέδια SVG και αρχεία Markdown.
settings-default-app-note-windows = Στα Windows οι προεπιλεγμένες εφαρμογές ορίζονται μόνο από τις Ρυθμίσεις του συστήματος. Με αυτό ανοίγει εκεί η σελίδα του prev.
settings-default-app-note-macos = Το macOS ζητά επιβεβαίωση για κάθε τύπο: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP και AVIF.
settings-default-app-status = Τύποι αρχείων που ανοίγουν με το prev: { $set } από { $total }.
settings-default-app-button = Ορισμός ως προεπιλογής
settings-default-app-button-windows = Άνοιγμα Ρυθμίσεων
settings-default-app-no-entry = Η καταχώριση επιφάνειας εργασίας (desktop entry) του prev δεν είναι εγκατεστημένη, οπότε το σύστημα δεν μπορεί να ανοίξει αρχεία με αυτό. Εγκαταστήστε το prev από πακέτο ή με το scripts/install.sh.
settings-default-app-no-bundle = Ανοίξτε το prev από το prev.app για να το ορίσετε ως προεπιλογή.
settings-default-app-failed = Δεν ήταν δυνατός ο ορισμός του prev ως προεπιλογής: { $error }
settings-storage = Χώρος αποθήκευσης
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (έκδοση ανάπτυξης, { $build })

## Markup toolbar

markup-tool-select = Επιλογή
markup-tool-area = Ορθογώνια επιλογή
markup-tool-sketch = Σκίτσο
markup-tool-draw = Σχεδίαση
markup-tool-shapes = Σχήματα
markup-tool-text-box = Πλαίσιο κειμένου
markup-tool-highlight = Επισήμανση
markup-tool-note = Σημείωση
markup-tool-sign = Υπογραφή
markup-tool-redact = Απόκρυψη
markup-apply = Εφαρμογή
markup-apply-redactions = Εφαρμογή αποκρύψεων
markup-shape-style = Στιλ σχήματος
markup-border-color = Χρώμα περιγράμματος
markup-fill-color = Χρώμα γεμίσματος
markup-text-style = Στιλ κειμένου
markup-delete = Διαγραφή
markup-undo = Αναίρεση
markup-redo = Επανάληψη

## Markup menus

markup-shape-rectangle = Ορθογώνιο
markup-shape-rounded-rectangle = Στρογγυλεμένο ορθογώνιο
markup-shape-oval = Οβάλ
markup-shape-line = Γραμμή
markup-shape-arrow = Βέλος
markup-shape-star = Αστέρι
markup-shape-polygon = Πολύγωνο
markup-shape-speech-bubble = Συννεφάκι ομιλίας
markup-shape-loupe = Μεγεθυντικός φακός
markup-shape-mask = Μάσκα
markup-style-highlight = Επισήμανση
markup-style-underline = Υπογράμμιση
markup-style-strikethrough = Διακριτή διαγραφή
markup-style-squiggly = Κυματιστή υπογράμμιση
markup-menu-color = Χρώμα
markup-menu-font = Γραμματοσειρά
markup-menu-size = Μέγεθος
markup-menu-alignment = Στοίχιση
markup-line-width = { $width } στ.
markup-dashed = Διακεκομμένη

## Notes

markup-kind-note = Σημείωση
markup-kind-text-box = Πλαίσιο κειμένου
markup-kind-stamp = Σφραγίδα
markup-kind-redaction = Απόκρυψη
markup-kind-shape = Σχήμα
markup-note-delete = Διαγραφή σημείωσης
markup-note-done = Τέλος
markup-note-placeholder = Πληκτρολογήστε μια σημείωση
markup-notes-empty = Δεν υπάρχουν επισημάνσεις ή σημειώσεις
markup-notes-empty-hint = Οι επισημάνσεις, οι σημειώσεις και τα πλαίσια κειμένου εμφανίζονται εδώ.
markup-notes-page = Σελίδα { $page }

## Markup errors

markup-change-failed = Δεν ήταν δυνατή η αλλαγή του εγγράφου: { $error }
markup-copy-area-failed = Δεν ήταν δυνατή η αντιγραφή της περιοχής: { $error }
markup-document-closed = το έγγραφο έκλεισε
markup-render-area-failed = δεν ήταν δυνατή η απόδοση της περιοχής
markup-copy-stopped = η αντιγραφή διακόπηκε

## Signatures

signature-menu-empty = Δεν υπάρχουν ακόμη υπογραφές.
signature-delete = Διαγραφή υπογραφής
signature-create = Δημιουργία υπογραφής…
signature-dialog-title = Δημιουργία υπογραφής
signature-tab-draw = Σχεδίαση
signature-tab-type = Πληκτρολόγηση
signature-tab-image = Εικόνα
signature-draw-hint = Υπογράψτε πάνω στη γραμμή με το ποντίκι, τη γραφίδα ή την επιφάνεια αφής.
signature-your-name = Το όνομά σας
signature-image-hint = Επιλέξτε μια φωτογραφία ή σάρωση της υπογραφής σας σε λευκό χαρτί.
signature-choose-image = Επιλογή εικόνας…
signature-description = Περιγραφή, π.χ. Ονοματεπώνυμο ή Αρχικά
signature-clear = Απαλοιφή
signature-ink = Μελάνι
signature-thickness = Πάχος
signature-sign-first = Υπογράψτε πρώτα και μετά αποθηκεύστε.
signature-default-name = Υπογραφή { $number }
signature-change-failed = Δεν ήταν δυνατή η αλλαγή των υπογραφών: { $error }
signature-no-data-folder = δεν υπάρχει φάκελος δεδομένων: η μεταβλητή HOME δεν έχει οριστεί
signature-removing-stopped = η αφαίρεση διακόπηκε
signature-saving-stopped = η αποθήκευση διακόπηκε
signature-reading-stopped = η ανάγνωση διακόπηκε
signature-not-an-image = το αρχείο δεν είναι εικόνα που μπορεί να διαβάσει το prev
signature-no-frames = η εικόνα δεν έχει καρέ
signature-not-found = δεν βρέθηκε υπογραφή στην εικόνα

## Dragging

drag-pages-need-document = Οι σελίδες μπορούν να αφεθούν μόνο σε έγγραφο.
drag-image-unsupported = Το prev δεν μπορεί να ανοίξει αυτήν την εικόνα.
drag-area-failed = Δεν ήταν δυνατή η μεταφορά της περιοχής: { $error }
drag-pages-failed = Δεν ήταν δυνατή η μεταφορά των σελίδων: { $error }
drag-start-failed = Δεν ήταν δυνατή η έναρξη της μεταφοράς.
drag-file-pages = Σελίδες
drag-file-one-page = { $name } (σελίδα { $page })
drag-file-page-range = { $name } (σελίδες { $first }–{ $last })
drag-file-image = Εικόνα
drop-pdf-title = Προσθήκη σε αυτό το έγγραφο;
drop-pdf-body = Προσθήκη του «{ $name }» στο τέλος αυτού του εγγράφου ή άνοιγμά του σε ξεχωριστό παράθυρο;
drop-pdfs-body = { $count ->
    [one] Προσθήκη αυτού του PDF στο τέλος αυτού του εγγράφου ή άνοιγμά του σε ξεχωριστό παράθυρο;
   *[other] Προσθήκη αυτών των { $count } PDF στο τέλος αυτού του εγγράφου ή άνοιγμά τους σε ξεχωριστά παράθυρα;
}
drop-pdf-add = Προσθήκη στο τέλος
drop-pdf-open = Άνοιγμα χωριστά

## PDF window

pdf-opening = Άνοιγμα…
pdf-open-failed = Το prev δεν μπορεί να ανοίξει αυτό το έγγραφο
pdf-no-pages = Το έγγραφο δεν έχει σελίδες.
pdf-document-closed = το έγγραφο έκλεισε
pdf-keep-original-failed = δεν ήταν δυνατή η διατήρηση της αρχικής έκδοσης: { $error }
pdf-save-failed = Δεν ήταν δυνατή η αποθήκευση: { $error }
pdf-nothing-to-paste = Δεν υπάρχει τίποτα για επικόλληση.
pdf-pasting-stopped = η επικόλληση διακόπηκε
pdf-file-dialog-failed = Δεν ήταν δυνατή η εμφάνιση του παραθύρου αρχείων: { $error }
pdf-bookmarks-no-home = Δεν είναι δυνατή η αποθήκευση σελιδοδεικτών: η μεταβλητή HOME δεν έχει οριστεί
pdf-bookmarks-save-failed = Δεν ήταν δυνατή η αποθήκευση των σελιδοδεικτών: { $error }
pdf-bookmark-page = Σελίδα { $page }

pdf-password-protected = Το «{ $name }» προστατεύεται με κωδικό πρόσβασης
pdf-password = Κωδικός πρόσβασης
pdf-password-wrong = Λανθασμένος κωδικός πρόσβασης. Δοκιμάστε ξανά.
pdf-unlock = Ξεκλείδωμα

pdf-sidebar = Πλαϊνή στήλη
pdf-page-of = από { $count }
pdf-zoom-out = Σμίκρυνση
pdf-zoom-in = Μεγέθυνση
pdf-zoom-percent = { $percent }%
pdf-fit-page = Προσαρμογή στη σελίδα
pdf-fit-width = Προσαρμογή στο πλάτος
pdf-actual-size = Πραγματικό μέγεθος
pdf-view-continuous = Συνεχής κύλιση
pdf-view-single-page = Μία σελίδα
pdf-view-two-pages = Δύο σελίδες
pdf-undo = Αναίρεση
pdf-redo = Επανάληψη
pdf-rotate-left = Περιστροφή αριστερά
pdf-rotate-right = Περιστροφή δεξιά
pdf-inspector = Επιθεωρητής
pdf-markup = Σήμανση
pdf-export = Εξαγωγή
pdf-settings = Ρυθμίσεις

pdf-search = Αναζήτηση
pdf-search-not-found = Δεν βρέθηκε
pdf-searching = Αναζήτηση…
pdf-search-match = { $current } από { $total }
pdf-search-match-more = { $current } από { $total }+

pdf-inspector-file = Αρχείο
pdf-inspector-document = Έγγραφο
pdf-inspector-pages = Σελίδες
pdf-inspector-title = Τίτλος
pdf-inspector-author = Συντάκτης
pdf-inspector-subject = Θέμα
pdf-inspector-keywords = Λέξεις-κλειδιά
pdf-inspector-created = Δημιουργία
pdf-inspector-modified = Τροποποίηση
pdf-inspector-application = Εφαρμογή
pdf-inspector-producer = Παραγωγός PDF
pdf-inspector-version = Έκδοση
pdf-inspector-security = Ασφάλεια
pdf-inspector-not-encrypted = Χωρίς κρυπτογράφηση
pdf-inspector-encrypted = Κρυπτογραφημένο ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } σελίδα
   *[other] { $count } σελίδες
}
pdf-inspector-page-size = Μέγεθος σελίδας
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } ίντσες)
pdf-loading = Φόρτωση…

pdf-tab-pages = Σελίδες
pdf-tab-contents = Περιεχόμενα
pdf-tab-notes = Επισημάνσεις και σημειώσεις
pdf-tab-bookmarks = Σελιδοδείκτες
pdf-no-outline = Χωρίς πίνακα περιεχομένων
pdf-no-outline-detail = Αυτό το έγγραφο δεν έχει διάρθρωση.
pdf-no-bookmarks = Χωρίς σελιδοδείκτες
pdf-no-bookmarks-detail = Πατήστε { $keys } για να προσθέσετε σελιδοδείκτη σε μια σελίδα.
pdf-no-bookmarks-detail-unbound = Οι σελίδες με σελιδοδείκτη εμφανίζονται εδώ.
pdf-remove-bookmark = Αφαίρεση σελιδοδείκτη

## Page editing

pages-menu = Σελίδες
pages-insert-blank = Εισαγωγή κενής σελίδας
pages-insert-file = Εισαγωγή από αρχείο…
pages-copy = { $count ->
    [one] Αντιγραφή σελίδας
   *[other] Αντιγραφή σελίδων
}
pages-paste = { $count ->
    [one] Επικόλληση σελίδας
   *[other] Επικόλληση { $count } σελίδων
}
pages-crop = Περικοπή στην επιλογή
pages-select-all = Επιλογή όλων των σελίδων
pages-delete = { $count ->
    [one] Διαγραφή σελίδας
   *[other] Διαγραφή σελίδων
}
pages-apply-redactions = Εφαρμογή αποκρύψεων…
pages-no-copied = Δεν υπάρχουν αντιγραμμένες σελίδες για επικόλληση.
pages-copied = { $count ->
    [one] Αντιγράφηκε { $count } σελίδα.
   *[other] Αντιγράφηκαν { $count } σελίδες.
}
pages-copy-failed = Δεν ήταν δυνατή η αντιγραφή των σελίδων: { $error }
pages-reading-stopped = η ανάγνωση διακόπηκε
pages-image-unreadable = δεν είναι εικόνα που μπορεί να διαβάσει το prev
pages-read-failed = Δεν ήταν δυνατή η ανάγνωση του αρχείου: { $error }
pages-at-least-one = Ένα έγγραφο πρέπει να έχει τουλάχιστον μία σελίδα.
pages-crop-needs-area = Επιλέξτε πρώτα μια περιοχή με το εργαλείο ορθογώνιας επιλογής.
pages-change-failed = Δεν ήταν δυνατή η αλλαγή των σελίδων: { $error }
pages-no-redactions = Δεν υπήρχαν αποκρύψεις για εφαρμογή.
pages-redactions-applied = { $count ->
    [one] Εφαρμόστηκε { $count } απόκρυψη.
   *[other] Εφαρμόστηκαν { $count } αποκρύψεις.
}
pages-forget-versions-failed = Δεν ήταν δυνατή η διαγραφή των προηγούμενων εκδόσεων: { $error }
pages-redact-title = Εφαρμογή αποκρύψεων;
pages-redact-body = { $count ->
    [one] Το κείμενο, οι εικόνες και τα σχέδια κάτω από τη σήμανση απόκρυψης αφαιρούνται οριστικά από το έγγραφο και η σήμανση γίνεται μαύρο πλαίσιο. Αυτό δεν αναιρείται και οι προηγούμενες εκδόσεις αυτού του αρχείου που διατηρεί το prev διαγράφονται.
   *[other] Το κείμενο, οι εικόνες και τα σχέδια κάτω από τις { $count } σημάνσεις απόκρυψης αφαιρούνται οριστικά από το έγγραφο και οι σημάνσεις γίνονται μαύρα πλαίσια. Αυτό δεν αναιρείται και οι προηγούμενες εκδόσεις αυτού του αρχείου που διατηρεί το prev διαγράφονται.
}
pages-redact-apply = Εφαρμογή

## PDF export

pages-export-title = Εξαγωγή
pages-export-format = Μορφή
pages-export-reduce = Μείωση μεγέθους αρχείου (εικόνες στα 150 dpi)
pages-export-flatten = Ενσωμάτωση σχολιασμών και πεδίων φόρμας
pages-export-flatten-detail = Η σήμανση και τα συμπληρωμένα πεδία γίνονται μέρος των σελίδων και δεν μπορούν πλέον να τροποποιηθούν. Οι σημάνσεις απόκρυψης που δεν έχουν εφαρμοστεί παραλείπονται.
pages-export-encrypt = Κρυπτογράφηση με κωδικό πρόσβασης
pages-export-password = Κωδικός πρόσβασης
pages-export-verify-password = Επιβεβαίωση κωδικού πρόσβασης
pages-export-resolution = Ανάλυση
pages-export-dpi = { $dpi } dpi
pages-export-quality = Ποιότητα
pages-export-quality-low = Χαμηλή
pages-export-quality-medium = Μέτρια
pages-export-quality-high = Υψηλή
pages-export-quality-best = Βέλτιστη
pages-export-one-file = Όλες οι σελίδες μπαίνουν σε ένα αρχείο.
pages-export-file-per-page = Κάθε σελίδα αποθηκεύεται ως ξεχωριστό αρχείο, αριθμημένο με βάση το όνομα που επιλέγετε.
pages-export-selected-only = { $count ->
    [one] Μόνο η επιλεγμένη σελίδα
   *[other] Μόνο οι { $count } επιλεγμένες σελίδες
}
pages-export-choose = Εξαγωγή…
pages-export-no-password = Εισαγάγετε κωδικό πρόσβασης.
pages-export-password-mismatch = Οι κωδικοί πρόσβασης δεν ταιριάζουν.
pages-export-file-name = { $name } (εξαγωγή)
pages-export-untitled = έγγραφο
pages-export-same-file = Εξαγάγετε σε νέο αρχείο· αυτό το έγγραφο αποθηκεύεται αυτόματα.
pages-export-exporting = Εξαγωγή του «{ $name }»…
pages-export-done = Ολοκληρώθηκε η εξαγωγή του «{ $name }».
pages-export-done-images = Εικόνες που εξήχθησαν: { $count }.
pages-export-failed = Δεν ήταν δυνατή η εξαγωγή: { $error }
pages-export-stopped = η εξαγωγή διακόπηκε

## Start window

app-start-hint = Ανοίξτε ή αφήστε εδώ ένα αρχείο PDF, εικόνας, SVG ή Markdown.
app-start-open = Άνοιγμα…
app-title-dev = { $title } (ανάπτυξη)
app-viewer-missing = { $kind }: αυτό το πρόγραμμα προβολής δεν έχει υλοποιηθεί ακόμη.
app-cannot-open = Το prev δεν μπορεί να ανοίξει αυτό το είδος αρχείου.
app-cannot-read = Το prev δεν μπορεί να διαβάσει αυτό το αρχείο: { $error }
app-kind-pdf = Έγγραφο PDF
app-kind-image = Εικόνα { $format }
app-kind-svg = Σχέδιο SVG
app-kind-markdown = Έγγραφο Markdown
app-file-dialog-failed = Δεν ήταν δυνατή η εμφάνιση του παραθύρου αρχείων: { $error }

## Actions

action-open = Άνοιγμα
action-settings = Ρυθμίσεις

## Toolbar

app-toolbar-keep-shown = Μόνιμη εμφάνιση της γραμμής εργαλείων
app-toolbar-auto-hide = Απόκρυψη της γραμμής εργαλείων όταν ο δείκτης βγαίνει από το παράθυρο
app-toolbar-more = Περισσότερα

## File facts

app-fact-name = Όνομα
app-fact-folder = Φάκελος
app-fact-size = Μέγεθος
app-fact-modified = Τροποποίηση
app-size-bytes = { $count } byte
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Μη έγκυρος σύνδεσμος { $uri }: { $error }
app-link-open-failed = Δεν ήταν δυνατό το άνοιγμα του { $uri }: { $error }
app-paste-needs-wl-clipboard = εγκαταστήστε το wl-clipboard για επικόλληση εικόνων
app-copy-needs-wl-clipboard = εγκαταστήστε το wl-clipboard για αντιγραφή εικόνων
app-copy-no-pixels = η περιοχή δεν έχει εικονοστοιχεία
app-copy-no-input = το wl-copy δεν έχει είσοδο
app-copy-failed = το wl-copy απέτυχε
app-clipboard-open-failed = Δεν ήταν δυνατό το άνοιγμα του προχείρου: { $error }
app-copy-image-failed = Δεν ήταν δυνατή η αντιγραφή της εικόνας: { $error }

## Printing

print-failed = Δεν ήταν δυνατή η εκτύπωση: { $error }
print-stopped = Η εκτύπωση διακόπηκε
print-unavailable = Η εκτύπωση δεν είναι ακόμη διαθέσιμη σε αυτό το σύστημα.
print-no-window = Δεν ήταν δυνατή η εκτύπωση: δεν υπάρχει παράθυρο πάνω από το οποίο να εμφανιστεί το παράθυρο διαλόγου εκτύπωσης
print-dialog-failed = Δεν ήταν δυνατή η εμφάνιση του παραθύρου διαλόγου εκτύπωσης: { $error }
print-job-not-started = ο εκτυπωτής δεν ξεκίνησε την εργασία
print-printer-stopped = ο εκτυπωτής σταμάτησε

## File dialogs

dialog-open = Άνοιγμα
dialog-filter-all = Όλα τα υποστηριζόμενα αρχεία
dialog-filter-pdf = Έγγραφα PDF
dialog-filter-images = Εικόνες
dialog-filter-svg = Σχέδια SVG
dialog-filter-markdown = Αρχεία Markdown
dialog-choose-signatures = Επιλέξτε τον φάκελο υπογραφών
dialog-choose-versions = Επιλέξτε τον φάκελο ιστορικού εκδόσεων
dialog-choose-bookmarks = Επιλέξτε το αρχείο σελιδοδεικτών

## Command line

usage-help =
    Χρήση: prev [FILE]...
           prev --mcp

    Προβολή και επεξεργασία PDF και εικόνων. Τα αρχεία ανοίγουν σε παράθυρα
    του prev που εκτελείται, το οποίο ξεκινά αν χρειάζεται.

    Επιλογές:
      -h, --help     Εμφάνιση αυτής της βοήθειας
      -V, --version  Εμφάνιση της έκδοσης
          --mcp      Εξυπηρέτηση MCP μέσω stdin και stdout, ώστε πράκτορες AI
                     να ελέγχουν το prev που εκτελείται

## Settings, continued

settings-language = Γλώσσα
settings-language-system = Προεπιλογή συστήματος: { $language }
settings-input-language = Γλώσσα εισαγωγής
settings-input-language-system = Σύμφωνα με τη διάταξη πληκτρολογίου
settings-input-language-note = Ορίζει από ποια πλευρά ξεκινά ένα κενό πεδίο κειμένου. Το κείμενο που πληκτρολογείτε διατηρεί τη δική του κατεύθυνση.

settings-appearance-system = Σύστημα
settings-appearance-light = Φωτεινή
settings-appearance-dark = Σκούρα
settings-system-accent = Χρήση του χρώματος έμφασης του συστήματος
settings-omarchy-note = Τα χρώματα προκύπτουν από το χρώμα έμφασης του θέματος «{ $theme }».
settings-system-accent-note = Τα χρώματα προκύπτουν από το χρώμα έμφασης του συστήματος.
settings-system-accent-none = Το σύστημα δεν έχει χρώμα έμφασης, οπότε το prev χρησιμοποιεί αυτό που έχει επιλεγεί παρακάτω.
settings-accent-chosen-note = Τα χρώματα προκύπτουν από το χρώμα που έχει επιλεγεί παρακάτω.
settings-auto-hide = Απόκρυψη της γραμμής εργαλείων όταν ο δείκτης βγαίνει από το παράθυρο
settings-auto-hide-note = Η γραμμή εργαλείων αιωρείται πάνω από το έγγραφο και αποσύρεται όσο ο δείκτης βρίσκεται έξω από το παράθυρο.
settings-animations = Κινούμενα εφέ
settings-animations-note = Γραμμές και πλαίσια που γλιστρούν, παράθυρα διαλόγου που μεγαλώνουν και ελαστικά κουμπιά.
settings-animations-reduced = Ανενεργά όσο το σύστημα ζητά μειωμένη κίνηση.
settings-corner-radius = Ακτίνα γωνιών
settings-corner-radius-note = Για τα παράθυρα διαλόγου και την αιωρούμενη γραμμή εργαλείων.
settings-corner-radius-value = { $radius } px
settings-overlay = Διαφάνεια επικάλυψης
settings-overlay-note = Πόσο διακρίνεται η σελίδα μέσα από την αιωρούμενη γραμμή εργαλείων.
settings-overlay-value = { $percent }%
settings-storage-signatures = Φάκελος υπογραφών
settings-storage-versions = Φάκελος ιστορικού εκδόσεων
settings-storage-bookmarks = Αρχείο σελιδοδεικτών
settings-storage-apply = Εφαρμογή
settings-storage-choose = Επιλογή…
settings-storage-note = Τα αρχεία που βρίσκονται ήδη σε παλιά θέση μένουν εκεί· μετακινήστε τα για να συνεχίσετε να τα χρησιμοποιείτε. Οι ρυθμίσεις της εφαρμογής prev αποθηκεύονται στο { $file }.
settings-save-failed = Δεν ήταν δυνατή η αποθήκευση των ρυθμίσεων: { $error }
settings-no-location = Δεν υπάρχει θέση για τις ρυθμίσεις: η μεταβλητή HOME δεν έχει οριστεί
settings-full-path = Χρησιμοποιήστε πλήρη διαδρομή, π.χ. ~/Documents/prev.
settings-path-is-folder = Το { $path } είναι φάκελος, όχι αρχείο.
settings-folder-missing = Ο φάκελος { $path } δεν υπάρχει. Δημιουργήστε τον πρώτα ή επιλέξτε έναν άλλο.
settings-path-is-file = Το { $path } είναι αρχείο, όχι φάκελος.
settings-cannot-write = Το prev δεν μπορεί να γράψει στο { $path }: { $error }.

## Export dialog

export-title = Εξαγωγή
export-format = Μορφή
export-quality = Ποιότητα
export-size = Μέγεθος
export-choose = Εξαγωγή…
export-format-webp = WebP (χωρίς απώλειες)
export-format-unknown = εικόνα
export-quality-low = Χαμηλή
export-quality-medium = Μέτρια
export-quality-high = Υψηλή
export-quality-best = Βέλτιστη
export-size-actual = Πραγματικό μέγεθος
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } εικονοστοιχεία
export-dialog-failed = Δεν ήταν δυνατή η εμφάνιση του παραθύρου αποθήκευσης: { $error }
export-done = Ολοκληρώθηκε η εξαγωγή: { $path }
export-failed = Δεν ήταν δυνατή η εξαγωγή: { $error }
export-stopped = η εξαγωγή διακόπηκε

## Image window

image-marked-no-edit = Δεν είναι δυνατή η επεξεργασία εικόνων με σήμανση. Εξαγάγετε την εικόνα για να κρατήσετε τη σήμανση ή διαγράψτε τη σήμανση και κλείστε τη γραμμή σήμανσης.

image-loading-stopped = η φόρτωση διακόπηκε
image-reverting-stopped = η επαναφορά διακόπηκε
image-rendering-stopped = η απόδοση διακόπηκε
image-saving-stopped = η αποθήκευση διακόπηκε
image-markup-stopped = η σήμανση διακόπηκε
image-no-version-store = Δεν υπάρχει θέση για τη διατήρηση εκδόσεων
image-revert-failed = Δεν ήταν δυνατή η επαναφορά: { $error }
image-read-failed = Δεν ήταν δυνατή η ανάγνωση του { $path }: { $error }
image-keep-original-failed = Δεν ήταν δυνατή η διατήρηση της αρχικής έκδοσης: { $error }
image-save-failed = Δεν ήταν δυνατή η αποθήκευση του { $path }: { $error }
image-markup-start-failed = Δεν ήταν δυνατή η έναρξη της σήμανσης: { $error }
image-cannot-edit = Δεν είναι δυνατή η επεξεργασία κινούμενων εικόνων και σχεδίων SVG.
image-cannot-mark-up = Δεν είναι δυνατή η σήμανση σε κινούμενες εικόνες και σχέδια SVG.
image-mark-up-wait = Περιμένετε να ολοκληρωθεί η επεξεργασία και μετά προσθέστε σήμανση.
image-crop-needs-selection = Σύρετε πρώτα μια επιλογή (εργαλείο Επιλογή) και μετά περικόψτε.
image-size-needed = Εισαγάγετε πλάτος και ύψος σε εικονοστοιχεία.
image-cannot-save-format = Οι αλλαγές στο «{ $name }» δεν μπορούν να αποθηκευτούν στη μορφή του. Χρησιμοποιήστε την Εξαγωγή ({ $keys }).
image-cannot-save-format-unbound = Οι αλλαγές στο «{ $name }» δεν μπορούν να αποθηκευτούν στη μορφή του. Χρησιμοποιήστε την Εξαγωγή.
image-cannot-export-animation = Οι κινούμενες εικόνες δεν μπορούν ακόμη να εξαχθούν.
image-drop-pages = Οι σελίδες μπορούν να αφεθούν μόνο σε έγγραφο.
image-drag-failed = Δεν ήταν δυνατή η έναρξη της μεταφοράς.
image-picture-save-failed = Δεν ήταν δυνατή η αποθήκευση της εικόνας στον φάκελο Λήψεις.
image-open-failed = Το prev δεν μπορεί να ανοίξει αυτήν την εικόνα
image-opening = Άνοιγμα…
image-name-mismatch-title = Το όνομα δεν ταιριάζει με τη μορφή
image-name-mismatch = Το «{ $name }» θα αποθηκευτεί ως αρχείο { $format }, αλλά το όνομά του τελειώνει σε .{ $extension }. Άλλες εφαρμογές ίσως να μην μπορούν να το ανοίξουν.
image-name-mismatch-no-extension = Το «{ $name }» θα αποθηκευτεί ως αρχείο { $format }, αλλά το όνομά του δεν έχει επέκταση. Άλλες εφαρμογές ίσως να μην μπορούν να το ανοίξουν.
image-choose-again = Νέα επιλογή
image-save-as-is = Αποθήκευση ως έχει
image-dimensions = { $width } × { $height }
image-frame-position = καρέ { $current } από { $total }
image-position = { $current } από { $total }
image-edited = επεξεργασμένη
image-sidebar = Πλαϊνή στήλη
image-zoom-out = Σμίκρυνση
image-zoom-in = Μεγέθυνση
image-zoom = { $percent }%
image-fit = Προσαρμογή στο παράθυρο
image-actual-size = Πραγματικό μέγεθος
image-undo = Αναίρεση
image-redo = Επανάληψη
image-rotate-left = Περιστροφή αριστερά
image-rotate-right = Περιστροφή δεξιά
image-flip-horizontal = Οριζόντια αναστροφή
image-flip-vertical = Κατακόρυφη αναστροφή
image-select = Ορθογώνια επιλογή
image-crop = Περικοπή στην επιλογή
image-adjust-size-tool = Προσαρμογή μεγέθους
image-adjust-color-tool = Προσαρμογή χρώματος
image-inspector = Επιθεωρητής
image-markup = Σήμανση
image-export = Εξαγωγή
image-settings = Ρυθμίσεις
image-adjust-color = Προσαρμογή χρώματος
image-adjust-size = Προσαρμογή μεγέθους
image-exposure = Έκθεση
image-contrast = Αντίθεση
image-saturation = Κορεσμός
image-temperature = Θερμοκρασία
image-tint = Απόχρωση
image-sepia = Σέπια
image-sharpness = Οξύτητα
image-levels = Επίπεδα
image-black-point = Μαύρο σημείο
image-midtones = Μεσαίοι τόνοι
image-white-point = Λευκό σημείο
image-reset-all = Επαναφορά όλων
image-current-size = Τρέχον μέγεθος: { $width } × { $height } εικονοστοιχεία
image-width = Πλάτος
image-height = Ύψος
image-scale-proportionally = Αναλογική κλιμάκωση
image-resize = Αλλαγή μεγέθους
image-inspector-loading = Φόρτωση…
image-file = Αρχείο
image-format = Μορφή
image-dimensions-label = Διαστάσεις
image-pixels = { $width } × { $height } εικονοστοιχεία
image-no-camera = Δεν υπάρχουν πληροφορίες κάμερας.
image-location = Τοποθεσία
image-remove-location = Αφαίρεση πληροφοριών τοποθεσίας
image-no-location = Δεν υπάρχουν πληροφορίες τοποθεσίας.
image-keywords-description = Λέξεις-κλειδιά και περιγραφή
image-keywords-hint = Λέξεις-κλειδιά, χωρισμένες με κόμματα
image-description = Περιγραφή
image-keywords-unsupported = Οι λέξεις-κλειδιά μπορούν να αποθηκευτούν σε αρχεία JPEG, PNG και WebP.
image-revert-to = Επαναφορά σε έκδοση
image-no-versions = Δεν υπάρχουν προηγούμενες εκδόσεις.
image-revert = Επαναφορά
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Κλείσιμο χωρίς εξαγωγή της σήμανσης;
image-close-body = { $count ->
    [one] Η σήμανση σε μια εικόνα διαρκεί μόνο όσο είναι ανοιχτό το παράθυρό της. Εξαγάγετε την εικόνα για να την κρατήσετε: η σήμανση σχεδιάζεται στο αντίγραφο που αποθηκεύετε.
   *[other] Η σήμανση στις εικόνες διαρκεί μόνο όσο είναι ανοιχτό το παράθυρό τους. Εξαγάγετε κάθε εικόνα για να την κρατήσετε: η σήμανση σχεδιάζεται στο αντίγραφο που αποθηκεύετε.
}
image-close-anyway = Κλείσιμο παρ’ όλα αυτά

## Markdown

markdown-reading-stopped = η ανάγνωση διακόπηκε
markdown-read-failed = Το prev δεν μπορεί να διαβάσει αυτό το αρχείο
markdown-draw-failed = Δεν ήταν δυνατή η σχεδίαση του εγγράφου
markdown-export-size = Ολόκληρο το έγγραφο, { $width } × { $height } εικονοστοιχεία
markdown-not-found = Δεν βρέθηκε
markdown-match = { $current } από { $total }
markdown-search = Αναζήτηση
markdown-smaller-text = Μικρότερο κείμενο
markdown-larger-text = Μεγαλύτερο κείμενο
markdown-zoom = { $percent }%
markdown-actual-size = Πραγματικό μέγεθος
markdown-limit-width = Περιορισμός πλάτους κειμένου
markdown-inspector = Επιθεωρητής
markdown-export = Εξαγωγή
markdown-settings = Ρυθμίσεις
markdown-file = Αρχείο
markdown-document = Έγγραφο
markdown-words = Λέξεις
markdown-lines = Γραμμές
markdown-pictures = Εικόνες

## Image details

image-meta-camera = Κάμερα
image-meta-exposure = Έκθεση
image-meta-image = Εικόνα
image-meta-make = Κατασκευαστής
image-meta-model = Μοντέλο
image-meta-lens = Φακός
image-meta-exposure-time = Χρόνος έκθεσης
image-meta-f-number = Αριθμός f
image-meta-iso = ISO
image-meta-focal-length = Εστιακή απόσταση
image-meta-exposure-bias = Αντιστάθμιση έκθεσης
image-meta-flash = Φλας
image-meta-date-taken = Ημερομηνία λήψης
image-meta-orientation = Προσανατολισμός
image-meta-color-space = Χρωματικός χώρος
image-meta-software = Λογισμικό
image-meta-artist = Δημιουργός
image-meta-copyright = Πνευματικά δικαιώματα
image-meta-seconds = { $value } δευτ.
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Κανονικός
    [2] Κατοπτρισμός οριζόντια
    [3] Περιστροφή 180°
    [4] Κατοπτρισμός κατακόρυφα
    [5] Κατοπτρισμός οριζόντια, περιστροφή 90° αριστερόστροφα
    [6] Περιστροφή 90° δεξιόστροφα
    [7] Κατοπτρισμός οριζόντια, περιστροφή 90° δεξιόστροφα
    [8] Περιστροφή 90° αριστερόστροφα
   *[other] Άγνωστος ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Ενεργοποιήθηκε
   *[no] Δεν ενεργοποιήθηκε
}{ $mode ->
    [on] , υποχρεωτικό
    [off] , ανενεργό
    [auto] , αυτόματο
   *[unknown] {""}
}{ $redeye ->
    [yes] , μείωση κόκκινων ματιών
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Χωρίς βαθμονόμηση
   *[other] Άλλος ({ $code })
}

## Errors

error-pdf-open = δεν είναι δυνατό το άνοιγμα του εγγράφου: { $detail }
error-pdf-page-out-of-range = η σελίδα { $page } δεν υπάρχει
error-pdf-password-protected = το έγγραφο προστατεύεται με κωδικό πρόσβασης· ανοίξτε το και αντιγράψτε αντ’ αυτού τις σελίδες του
error-pdf-no-pages = δεν υπάρχουν σελίδες για απόσπαση
error-pdf-crop-outside = η περιοχή περικοπής βρίσκεται εκτός της σελίδας
error-pdf-closed = το έγγραφο έκλεισε
error-pdf-saved-unreadable = το αποθηκευμένο έγγραφο δεν ανοίγει πλέον
error-image-read = δεν είναι δυνατή η ανάγνωση του αρχείου: { $detail }
error-image-invalid = η εικόνα είναι κατεστραμμένη ή μη έγκυρη: { $detail }
error-image-missing-library = για το άνοιγμα αυτής της μορφής απαιτείται το { $library }, το οποίο δεν είναι εγκατεστημένο
error-image-unsupported = οι εικόνες { $format } δεν υποστηρίζονται ακόμη
error-image-encode = δεν είναι δυνατή η κωδικοποίηση της εικόνας: { $detail }
error-exif-malformed = τα δεδομένα EXIF είναι κατεστραμμένα
error-settings-read = δεν είναι δυνατή η ανάγνωση των ρυθμίσεων: { $detail }
error-settings-invalid = μη έγκυρες ρυθμίσεις: { $detail }
error-remove-location = δεν ήταν δυνατή η αφαίρεση της τοποθεσίας: { $error }
error-location-unsupported = οι πληροφορίες τοποθεσίας μπορούν να αφαιρεθούν από αρχεία JPEG, PNG, WebP και TIFF
error-xmp-unsupported = οι λέξεις-κλειδιά και οι περιγραφές μπορούν να αποθηκευτούν μόνο σε αρχεία JPEG, PNG και WebP

## Formats

format-camera-raw = RAW κάμερας

## The macOS menu bar, named as in macOS's own apps.
menu-about = Πληροφορίες για το prev
menu-settings = Ρυθμίσεις…
menu-services = Υπηρεσίες
menu-hide = Απόκρυψη prev
menu-hide-others = Απόκρυψη άλλων
menu-show-all = Εμφάνιση όλων
menu-quit = Τερματισμός prev
menu-file = Αρχείο
menu-open = Άνοιγμα…
menu-close = Κλείσιμο παραθύρου
menu-export = Εξαγωγή…
menu-print = Εκτύπωση…
menu-edit = Επεξεργασία
menu-undo = Αναίρεση
menu-redo = Επανάληψη
menu-cut = Αποκοπή
menu-copy = Αντιγραφή
menu-paste = Επικόλληση
menu-select-all = Επιλογή όλων
menu-find = Εύρεση
menu-find-next = Εύρεση επόμενου
menu-find-previous = Εύρεση προηγούμενου
menu-view = Προβολή
menu-hide-sidebar = Απόκρυψη πλαϊνής στήλης
menu-thumbnails = Μικρογραφίες
menu-contents = Πίνακας περιεχομένων
menu-notes = Επισημάνσεις και σημειώσεις
menu-bookmarks = Σελιδοδείκτες
menu-zoom-in = Μεγέθυνση
menu-zoom-out = Σμίκρυνση
menu-actual-size = Πραγματικό μέγεθος
menu-zoom-to-fit = Ζουμ για προσαρμογή
menu-inspector = Εμφάνιση επιθεωρητή
menu-slideshow = Παρουσίαση
menu-full-screen = Είσοδος σε πλήρη οθόνη
menu-go = Μετάβαση
menu-next-page = Επόμενη σελίδα
menu-previous-page = Προηγούμενη σελίδα
menu-go-to-page = Μετάβαση σε σελίδα…
menu-bookmark = Προσθήκη σελιδοδείκτη
menu-tools = Εργαλεία
menu-markup = Εμφάνιση γραμμής εργαλείων σήμανσης
menu-rotate-left = Περιστροφή αριστερά
menu-rotate-right = Περιστροφή δεξιά
menu-crop = Περικοπή
menu-adjust-color = Προσαρμογή χρώματος…
menu-window = Παράθυρο
menu-minimize = Ελαχιστοποίηση
menu-zoom = Ζουμ
menu-bring-all-to-front = Μεταφορά όλων μπροστά

## Outside control

settings-outside-control = Εξωτερικός έλεγχος
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = Γενικά
settings-tab-agents = Πράκτορες
settings-allow-outside-control = Να επιτρέπεται ο εξωτερικός έλεγχος
settings-allow-outside-control-note = Πράκτορες AI όπως το Claude Code μπορούν να διαβάζουν και να αλλάζουν τα αρχεία σας στο prev, μέσω του prev --mcp. Το prev ρωτά πριν από κάθε νέο πράκτορα.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Επιτρέπονται: { $agents }
settings-forget-agents = Διαγραφή
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Να επιτραπεί στο { $agent } να ελέγχει το prev;
agent-prompt-body = Το { $agent } ζητά να χρησιμοποιήσει τον εξωτερικό έλεγχο του prev, για να διαβάζει τα ανοιχτά αρχεία σας και να τα αλλάζει. Μπορείτε να απενεργοποιήσετε τον εξωτερικό έλεγχο στις Ρυθμίσεις.
agent-prompt-allow = Να επιτραπεί
agent-prompt-deny = Να μην επιτραπεί
settings-ask-before-note = Ερώτηση πριν από τις εξής ενέργειες ενός πράκτορα:
settings-ask-reading = Ανάγνωση αρχείου
settings-ask-viewing = Αλλαγή προβολής ή παραθύρου
settings-ask-marking-up = Σήμανση αρχείου
settings-ask-editing = Επεξεργασία αρχείου
settings-ask-signing = Υπογραφή αρχείου
settings-ask-redacting = Εφαρμογή αποκρύψεων
settings-ask-exporting = Εξαγωγή αρχείου
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Να επιτραπεί στο { $agent } να διαβάσει αυτό το αρχείο;
agent-ask-view = Να επιτραπεί στο { $agent } να αλλάξει την προβολή;
agent-ask-markup = Να επιτραπεί στο { $agent } να προσθέσει σήμανση σε αυτό το αρχείο;
agent-ask-edit = Να επιτραπεί στο { $agent } να επεξεργαστεί αυτό το αρχείο;
agent-ask-sign = Να επιτραπεί στο { $agent } να υπογράψει αυτό το αρχείο;
agent-ask-redact = Να επιτραπεί στο { $agent } να εφαρμόσει αποκρύψεις;
agent-ask-export = Να επιτραπεί στο { $agent } να εξαγάγει αυτό το αρχείο;
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = Το { $agent } ζητά να χρησιμοποιήσει το «{ $tool }». Στις Ρυθμίσεις επιλέγετε για τι ρωτά το prev.
agent-ask-final = Αυτό δεν μπορεί να αναιρεθεί.

## The assistant
settings-tab-assistant = Βοηθός
settings-assistant-note = Τα μοντέλα με τα οποία μπορεί να μιλά ο πίνακας του βοηθού. Τα κλειδιά φυλάσσονται στην κλειδοθήκη του συστήματος.
settings-assistant-none = Δεν υπάρχουν ακόμη μοντέλα. Προσθέστε ένα παρακάτω: ένα τοπικό μοντέλο, όπως ένα του Ollama, μένει σε αυτόν τον υπολογιστή· ένα μοντέλο cloud χρειάζεται κλειδί API από τον πάροχό του.
settings-assistant-in-use = Σε χρήση
settings-assistant-use = Χρήση
settings-assistant-remove = Αφαίρεση
settings-assistant-add = Προσθήκη μοντέλου
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Διακομιστής συμβατός με OpenAI
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Μοντέλο, π.χ. { $example }
settings-assistant-key = Κλειδί API
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Διεύθυνση, π.χ. { $example }
# The menu of how much a local model reads at once.
settings-assistant-context = Παράθυρο περιβάλλοντος
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands } χιλ. tokens
settings-assistant-context-note = Μεγαλύτερο παράθυρο επιτρέπει στον βοηθό να διαβάσει μεγαλύτερο μέρος ενός αρχείου σε μία συνομιλία, αλλά το μοντέλο καταλαμβάνει περισσότερη μνήμη και μπορεί να απαντά πιο αργά.
settings-assistant-add-button = Προσθήκη
settings-assistant-use-key = Συνέχεια
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Δημιουργήστε ένα κλειδί στον ιστότοπο του παρόχου { $provider } και επικολλήστε το εδώ.
settings-assistant-get-key = Λήψη κλειδιού API
settings-assistant-key-kept = Το κλειδί σας για τον πάροχο { $provider } φυλάσσεται στην κλειδοθήκη του συστήματος.
settings-assistant-change-key = Αλλαγή κλειδιού
settings-assistant-key-refused = Ο πάροχος { $provider } απέρριψε αυτό το κλειδί. Ελέγξτε ότι αντιγράφηκε ολόκληρο, από τον σωστό λογαριασμό.
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = Ο διακομιστής { $provider } εκτελείται στη διεύθυνση { $address }.
settings-assistant-no-server = Το prev δεν βρήκε διακομιστή { $provider } σε λειτουργία σε αυτόν τον υπολογιστή. Ξεκινήστε τον ή δώστε τη διεύθυνσή του παρακάτω.
settings-assistant-get-server = Λήψη { $provider }
settings-assistant-look-again = Νέα αναζήτηση
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Χρήση άλλης διεύθυνσης
settings-assistant-looking = Αναζήτηση μοντέλων…
settings-assistant-found-none = Ο διακομιστής { $provider } δεν έχει ακόμη μοντέλα. Κατεβάστε ένα μέσω αυτού και αναζητήστε ξανά.
settings-assistant-recommended = Προτείνεται
settings-assistant-uses-tools = Χρησιμοποιεί εργαλεία
settings-assistant-sees = Βλέπει εικόνες
settings-assistant-no-tools = Δεν μπορεί να χρησιμοποιήσει εργαλεία, που τα χρειάζεται ο βοηθός
settings-assistant-added-tag = Προστέθηκε
settings-assistant-trying = Δοκιμή…
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Άνοιγμα σελίδας
# $model is the model's name.
settings-assistant-added = Το μοντέλο { $model } απάντησε και προστέθηκε.
settings-assistant-key-needed = Αυτό το μοντέλο χρειάζεται κλειδί API.
# $error is what the keychain said.
settings-assistant-key-failed = Δεν ήταν δυνατή η φύλαξη του κλειδιού στην κλειδοθήκη: { $error }
assistant-title = Βοηθός
assistant-new-chat = Νέα συνομιλία
assistant-ask = Ρωτήστε για αυτό το αρχείο
assistant-send = Αποστολή
assistant-stop = Διακοπή
assistant-thinking = Σκέφτεται…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Σκέψεις
assistant-running = Εκτελείται…
assistant-stopped = Διακόπηκε.
assistant-no-model = Προσθέστε πρώτα ένα μοντέλο στις Ρυθμίσεις.
assistant-add-model = Ο βοηθός χρειάζεται ένα μοντέλο: ένα μοντέλο cloud με το κλειδί API του ή ένα τοπικό μοντέλο.
assistant-open-settings = Προσθήκη μοντέλου
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Τώρα μιλάτε με το μοντέλο { $model }.
assistant-add-another = Προσθήκη μοντέλου…
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } σε αυτόν τον υπολογιστή
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } στη διεύθυνση { $host }
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Η συνομιλία δεν χωρά πλέον σε όσα μπορεί να διαβάσει μονομιάς το μοντέλο { $model }. Ξεκινήστε νέα συνομιλία ή επιλέξτε ένα μοντέλο που διαβάζει περισσότερα.
assistant-problem-key = Ο πάροχος { $provider } απέρριψε το κλειδί API. Ελέγξτε το στις Ρυθμίσεις.
assistant-problem-rate = Ο πάροχος { $provider } ζητά να μειωθεί ο ρυθμός των αιτημάτων. Δοκιμάστε ξανά σε λίγο.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = Ο πάροχος { $provider } ζητά να μειωθεί ο ρυθμός των αιτημάτων: { $message }
assistant-problem-credit = Ο πάροχος { $provider } αναφέρει ότι ο λογαριασμός δεν έχει υπόλοιπο πίστωσης. Σε νέο λογαριασμό πρέπει πρώτα να αγοράσετε πίστωση στον ιστότοπο του παρόχου { $provider } για να λειτουργήσει το κλειδί· έπειτα δοκιμάστε ξανά.
assistant-problem-model = Ο πάροχος { $provider } δεν έχει μοντέλο με το όνομα { $model }. Ελέγξτε το όνομά του στις Ρυθμίσεις.
assistant-problem-unavailable = Ο πάροχος { $provider } είναι απασχολημένος ή αντιμετωπίζει προβλήματα. Δοκιμάστε ξανά σε λίγο.
assistant-problem-unreachable = Το prev δεν μπόρεσε να συνδεθεί με τον πάροχο { $provider }. Ελέγξτε τη σύνδεσή σας ή ότι ο διακομιστής λειτουργεί.
assistant-problem-refused = Το μοντέλο { $model } αρνήθηκε να απαντήσει.
# $message is what the provider said, untranslated.
assistant-problem-other = Το μοντέλο { $model } δεν απάντησε: { $message }
