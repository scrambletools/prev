# prev's interface text in Malay (Bahasa Melayu), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = penandaan, note = nota, highlight = serlahan,
# annotation = anotasi, redact/redaction = hitamkan/penghitaman,
# inspector = pemeriksa, bookmark = penanda buku, page = halaman,
# file = fail, image = imej, settings = tetapan, window = tetingkap,
# zoom in/out = zum masuk/keluar, undo/redo = buat asal/buat semula,
# default = lalai, crop = kerat, export = eksport, stamp = cop.
# Buttons and menu items use the imperative verb (Simpan, Batal, Tutup).

## Language

language-name = Bahasa Melayu

## Common

common-cancel = Batal
common-close = Tutup
common-save = Simpan

## Settings

settings-title = Tetapan
settings-appearance = Penampilan
settings-colors = Warna
settings-windows = Tetingkap
settings-default-app = Apl lalai
settings-default-app-label = Buka fail dengan prev
settings-default-app-note = Jadikan prev apl yang membuka PDF, imej, lukisan SVG dan fail Markdown.
settings-default-app-note-windows = Windows hanya membenarkan anda memilih apl lalai dalam Tetapannya sendiri. Butang ini membuka halaman prev di sana.
settings-default-app-note-macos = macOS meminta anda mengesahkan setiap jenis: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP dan AVIF.
settings-default-app-status = { $set } daripada { $total } jenis fail dibuka dengan prev.
settings-default-app-button = Jadikan Lalai
settings-default-app-button-windows = Buka Tetapan
settings-default-app-no-entry = Entri desktop prev tidak dipasang, jadi sistem tidak dapat membuka fail dengannya. Pasang prev daripada pakej atau dengan scripts/install.sh.
settings-default-app-no-bundle = Buka prev daripada prev.app untuk menjadikannya lalai.
settings-default-app-failed = Tidak dapat menjadikan prev lalai: { $error }
settings-storage = Storan
settings-version = prev { $version }
settings-version-development = prev { $version } (binaan pembangunan)

## Markup toolbar

markup-tool-select = Pilih
markup-tool-area = Pilihan segi empat
markup-tool-sketch = Lakar
markup-tool-draw = Lukis
markup-tool-shapes = Bentuk
markup-tool-text-box = Kotak teks
markup-tool-highlight = Serlah
markup-tool-note = Nota
markup-tool-sign = Tandatangani
markup-tool-redact = Hitamkan
markup-apply = Guna
markup-apply-redactions = Gunakan penghitaman
markup-shape-style = Gaya bentuk
markup-border-color = Warna sempadan
markup-fill-color = Warna isian
markup-text-style = Gaya teks
markup-delete = Padam
markup-undo = Buat asal
markup-redo = Buat semula

## Markup menus

markup-shape-rectangle = Segi Empat Tepat
markup-shape-rounded-rectangle = Segi Empat Tepat Bucu Bulat
markup-shape-oval = Bujur
markup-shape-line = Garisan
markup-shape-arrow = Anak Panah
markup-shape-star = Bintang
markup-shape-polygon = Poligon
markup-shape-speech-bubble = Belon Pertuturan
markup-shape-loupe = Kanta Pembesar
markup-shape-mask = Topeng
markup-style-highlight = Serlahan
markup-style-underline = Garis bawah
markup-style-strikethrough = Garis lorek
markup-style-squiggly = Garis berombak
markup-menu-color = Warna
markup-menu-font = Fon
markup-menu-size = Saiz
markup-menu-alignment = Penjajaran
markup-line-width = { $width } pt
markup-dashed = Putus-putus

## Notes

markup-kind-note = Nota
markup-kind-text-box = Kotak teks
markup-kind-stamp = Cop
markup-kind-redaction = Penghitaman
markup-kind-shape = Bentuk
markup-note-delete = Padam nota
markup-note-done = Selesai
markup-note-placeholder = Taip nota
markup-notes-empty = Tiada serlahan atau nota
markup-notes-empty-hint = Serlahan, nota dan kotak teks dipaparkan di sini.
markup-notes-page = Halaman { $page }

## Markup errors

markup-change-failed = Tidak dapat mengubah dokumen: { $error }
markup-copy-area-failed = Tidak dapat menyalin kawasan: { $error }
markup-document-closed = dokumen telah ditutup
markup-render-area-failed = tidak dapat memaparkan kawasan
markup-copy-stopped = penyalinan terhenti

## Signatures

signature-menu-empty = Belum ada tandatangan.
signature-delete = Padam tandatangan
signature-create = Cipta Tandatangan…
signature-dialog-title = Cipta Tandatangan
signature-tab-draw = Lukis
signature-tab-type = Taip
signature-tab-image = Imej
signature-draw-hint = Tandatangani pada garisan dengan tetikus, pen atau pad sentuh anda.
signature-your-name = Nama anda
signature-image-hint = Pilih foto atau imbasan tandatangan anda di atas kertas putih.
signature-choose-image = Pilih Imej…
signature-description = Keterangan, contohnya Nama penuh atau Parap
signature-clear = Kosongkan
signature-ink = Dakwat
signature-thickness = Ketebalan
signature-sign-first = Tandatangani dahulu, kemudian simpan.
signature-default-name = Tandatangan { $number }
signature-change-failed = Tidak dapat mengubah tandatangan: { $error }
signature-no-data-folder = tiada folder data: HOME tidak ditetapkan
signature-removing-stopped = pengalihan keluar terhenti
signature-saving-stopped = penyimpanan terhenti
signature-reading-stopped = pembacaan terhenti
signature-not-an-image = fail itu bukan imej yang boleh dibaca oleh prev
signature-no-frames = imej tidak mempunyai bingkai
signature-not-found = tiada tandatangan ditemui dalam imej

## Dragging

drag-pages-need-document = Halaman hanya boleh dilepaskan pada dokumen.
drag-image-unsupported = prev tidak dapat membuka imej ini.
drag-area-failed = Tidak dapat menyeret kawasan: { $error }
drag-pages-failed = Tidak dapat menyeret halaman: { $error }
drag-start-failed = Tidak dapat mula menyeret.
drag-file-pages = Halaman
drag-file-one-page = { $name } (halaman { $page })
drag-file-page-range = { $name } (halaman { $first }–{ $last })
drag-file-image = Imej
drop-pdf-title = Tambah pada dokumen ini?
drop-pdf-body = Tambah “{ $name }” ke hujung dokumen ini, atau buka dalam tetingkapnya sendiri?
drop-pdfs-body = Tambah { $count } PDF ini ke hujung dokumen ini, atau buka setiap satu dalam tetingkapnya sendiri?
drop-pdf-add = Tambah ke Hujung
drop-pdf-open = Buka Berasingan

## PDF window

pdf-opening = Membuka…
pdf-open-failed = prev tidak dapat membuka dokumen ini
pdf-no-pages = Dokumen ini tiada halaman.
pdf-document-closed = dokumen telah ditutup
pdf-keep-original-failed = tidak dapat mengekalkan versi asal: { $error }
pdf-save-failed = Tidak dapat menyimpan: { $error }
pdf-nothing-to-paste = Tiada apa-apa untuk ditampal.
pdf-pasting-stopped = penampalan terhenti
pdf-file-dialog-failed = Tidak dapat menunjukkan dialog fail: { $error }
pdf-bookmarks-no-home = Penanda buku tidak dapat disimpan: HOME tidak ditetapkan
pdf-bookmarks-save-failed = Tidak dapat menyimpan penanda buku: { $error }
pdf-bookmark-page = Halaman { $page }

pdf-password-protected = “{ $name }” dilindungi kata laluan
pdf-password = Kata laluan
pdf-password-wrong = Kata laluan salah. Cuba lagi.
pdf-unlock = Buka Kunci

pdf-sidebar = Bar sisi
pdf-page-of = daripada { $count }
pdf-zoom-out = Zum keluar
pdf-zoom-in = Zum masuk
pdf-zoom-percent = { $percent }%
pdf-fit-page = Muat halaman
pdf-fit-width = Muat lebar
pdf-actual-size = Saiz sebenar
pdf-view-continuous = Tatal berterusan
pdf-view-single-page = Satu halaman
pdf-view-two-pages = Dua halaman
pdf-undo = Buat asal
pdf-redo = Buat semula
pdf-rotate-left = Putar ke kiri
pdf-rotate-right = Putar ke kanan
pdf-inspector = Pemeriksa
pdf-markup = Penandaan
pdf-export = Eksport
pdf-settings = Tetapan

pdf-search = Cari
pdf-search-not-found = Tidak ditemui
pdf-searching = Mencari…
pdf-search-match = { $current } daripada { $total }
pdf-search-match-more = { $current } daripada { $total }+

pdf-inspector-file = Fail
pdf-inspector-document = Dokumen
pdf-inspector-pages = Halaman
pdf-inspector-title = Tajuk
pdf-inspector-author = Pengarang
pdf-inspector-subject = Subjek
pdf-inspector-keywords = Kata kunci
pdf-inspector-created = Dicipta
pdf-inspector-modified = Diubah suai
pdf-inspector-application = Aplikasi
pdf-inspector-producer = Penghasil PDF
pdf-inspector-version = Versi
pdf-inspector-security = Keselamatan
pdf-inspector-not-encrypted = Tidak disulitkan
pdf-inspector-encrypted = Disulitkan ({ $method })
pdf-inspector-page-count = { $count ->
   *[other] { $count } halaman
}
pdf-inspector-page-size = Saiz halaman
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inci)
pdf-loading = Memuatkan…

pdf-tab-pages = Halaman
pdf-tab-contents = Kandungan
pdf-tab-notes = Serlahan dan nota
pdf-tab-bookmarks = Penanda buku
pdf-no-outline = Tiada isi kandungan
pdf-no-outline-detail = Dokumen ini tiada rangka.
pdf-no-bookmarks = Tiada penanda buku
pdf-no-bookmarks-detail = Tekan { $keys } untuk menanda buku halaman.
pdf-no-bookmarks-detail-unbound = Halaman yang ditanda buku dipaparkan di sini.
pdf-remove-bookmark = Alih keluar penanda buku

## Page editing

pages-menu = Halaman
pages-insert-blank = Sisipkan Halaman Kosong
pages-insert-file = Sisipkan daripada Fail…
pages-copy = { $count ->
   *[other] Salin Halaman
}
pages-paste = { $count ->
   *[other] Tampal { $count } Halaman
}
pages-crop = Kerat mengikut Pilihan
pages-select-all = Pilih Semua Halaman
pages-delete = { $count ->
   *[other] Padam Halaman
}
pages-apply-redactions = Gunakan Penghitaman…
pages-no-copied = Tiada halaman yang disalin untuk ditampal.
pages-copied = { $count ->
   *[other] { $count } halaman telah disalin.
}
pages-copy-failed = Tidak dapat menyalin halaman: { $error }
pages-reading-stopped = pembacaan terhenti
pages-image-unreadable = bukan imej yang boleh dibaca oleh prev
pages-read-failed = Tidak dapat membaca fail: { $error }
pages-at-least-one = Dokumen memerlukan sekurang-kurangnya satu halaman.
pages-crop-needs-area = Pilih kawasan dengan alat pilihan segi empat dahulu.
pages-change-failed = Tidak dapat mengubah halaman: { $error }
pages-no-redactions = Tiada penghitaman untuk digunakan.
pages-redactions-applied = { $count ->
   *[other] { $count } penghitaman telah digunakan.
}
pages-forget-versions-failed = Tidak dapat memadam versi terdahulu: { $error }
pages-redact-title = Gunakan penghitaman?
pages-redact-body = { $count ->
   *[other] Teks, imej dan lukisan di bawah { $count } tanda penghitaman akan dialih keluar daripada dokumen secara kekal, dan tanda itu menjadi kotak hitam. Tindakan ini tidak boleh dibuat asal, dan versi terdahulu fail ini yang disimpan oleh prev akan dipadam.
}
pages-redact-apply = Guna

## PDF export

pages-export-title = Eksport
pages-export-format = Format fail
pages-export-reduce = Kecilkan saiz fail (imej pada 150 dpi)
pages-export-flatten = Leperkan anotasi dan medan borang
pages-export-flatten-detail = Penandaan dan medan yang telah diisi menjadi sebahagian daripada halaman dan tidak boleh diedit lagi. Tanda penghitaman yang belum digunakan akan ditinggalkan.
pages-export-encrypt = Sulitkan dengan kata laluan
pages-export-password = Kata laluan
pages-export-verify-password = Sahkan kata laluan
pages-export-resolution = Resolusi
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kualiti
pages-export-quality-low = Rendah
pages-export-quality-medium = Sederhana
pages-export-quality-high = Tinggi
pages-export-quality-best = Terbaik
pages-export-one-file = Semua halaman dimasukkan ke dalam satu fail.
pages-export-file-per-page = Setiap halaman disimpan sebagai failnya sendiri, dengan nama yang anda pilih diikuti nombor.
pages-export-selected-only = { $count ->
   *[other] Hanya { $count } halaman yang dipilih
}
pages-export-choose = Eksport…
pages-export-no-password = Masukkan kata laluan.
pages-export-password-mismatch = Kata laluan tidak sepadan.
pages-export-file-name = { $name } (dieksport)
pages-export-untitled = dokumen
pages-export-same-file = Eksport ke fail baharu; dokumen ini disimpan secara automatik.
pages-export-exporting = Mengeksport “{ $name }”…
pages-export-done = “{ $name }” telah dieksport.
pages-export-done-images = { $count } imej telah dieksport.
pages-export-failed = Tidak dapat mengeksport: { $error }
pages-export-stopped = pengeksportan terhenti

## Start window

app-start-hint = Buka atau lepaskan fail PDF, imej, SVG atau Markdown.
app-start-open = Buka…
app-title-dev = { $title } (pembangunan)
app-viewer-missing = { $kind }: pemapar ini belum dibina.
app-cannot-open = prev tidak dapat membuka jenis fail ini.
app-cannot-read = prev tidak dapat membaca fail ini: { $error }
app-kind-pdf = Dokumen PDF
app-kind-image = Imej { $format }
app-kind-svg = Lukisan SVG
app-kind-markdown = Dokumen Markdown
app-file-dialog-failed = Tidak dapat menunjukkan dialog fail: { $error }

## Actions

action-open = Buka
action-settings = Tetapan

## Toolbar

app-toolbar-keep-shown = Sentiasa tunjukkan bar alat
app-toolbar-auto-hide = Sembunyikan bar alat apabila penuding beredar
app-toolbar-more = Lagi

## File facts

app-fact-name = Nama
app-fact-folder = Folder
app-fact-size = Saiz
app-fact-modified = Diubah suai
app-size-bytes = { $count } bait
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Pautan tidak sah { $uri }: { $error }
app-link-open-failed = Tidak dapat membuka { $uri }: { $error }
app-paste-needs-wl-clipboard = pasang wl-clipboard untuk menampal imej
app-copy-needs-wl-clipboard = pasang wl-clipboard untuk menyalin imej
app-copy-no-pixels = kawasan itu tiada piksel
app-copy-no-input = wl-copy tiada input
app-copy-failed = wl-copy gagal
app-clipboard-open-failed = Tidak dapat membuka papan keratan: { $error }
app-copy-image-failed = Tidak dapat menyalin imej: { $error }

## Printing

print-failed = Tidak dapat mencetak: { $error }
print-stopped = Pencetakan terhenti
print-unavailable = Pencetakan belum tersedia pada sistem ini.
print-no-window = Tidak dapat mencetak: tiada tetingkap untuk menunjukkan dialog cetak
print-dialog-failed = Tidak dapat menunjukkan dialog cetak: { $error }
print-job-not-started = pencetak tidak memulakan kerja cetak
print-printer-stopped = pencetak terhenti

## File dialogs

dialog-open = Buka
dialog-filter-all = Semua fail yang disokong
dialog-filter-pdf = Dokumen PDF
dialog-filter-images = Imej
dialog-filter-svg = Lukisan SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Pilih folder tandatangan
dialog-choose-versions = Pilih folder sejarah versi
dialog-choose-bookmarks = Pilih fail penanda buku

## Command line

usage-help =
    Penggunaan: prev [FILE]...

    Lihat dan edit PDF dan imej. Fail dibuka dalam tetingkap prev yang sedang
    berjalan, yang akan dimulakan jika perlu.

    Pilihan:
      -h, --help     Tunjukkan bantuan ini
      -V, --version  Tunjukkan versi

## Settings, continued

settings-language = Bahasa
settings-language-system = Lalai sistem: { $language }
settings-input-language = Bahasa input
settings-input-language-system = Ikut susun atur papan kekunci
settings-input-language-note = Menetapkan sisi permulaan medan teks yang kosong. Teks yang anda taip mengekalkan arahnya sendiri.

settings-appearance-system = Sistem
settings-appearance-light = Cerah
settings-appearance-dark = Gelap
settings-omarchy-accent = Guna warna aksen Omarchy
settings-omarchy-note = Warna dibina daripada aksen “{ $theme }”.
settings-omarchy-none = Tiada tema Omarchy yang aktif.
settings-auto-hide = Sembunyikan bar alat apabila penuding beredar
settings-auto-hide-note = Bar alat terapung di atas dokumen dan menggelongsor pergi semasa penuding berada di luar tetingkap.
settings-animations = Animasi
settings-animations-note = Bar dan panel yang menggelongsor, dialog yang membesar dan butang yang melantun.
settings-animations-reduced = Dimatikan selagi sistem meminta pergerakan dikurangkan.
settings-corner-radius = Jejari bucu
settings-corner-radius-note = Untuk dialog dan bar alat terapung.
settings-corner-radius-value = { $radius } px
settings-overlay = Kelutsinaran tindanan
settings-overlay-note = Sejauh mana halaman kelihatan menembusi bar alat terapung.
settings-overlay-value = { $percent }%
settings-storage-signatures = Folder tandatangan
settings-storage-versions = Folder sejarah versi
settings-storage-bookmarks = Fail penanda buku
settings-storage-apply = Guna
settings-storage-choose = Pilih…
settings-storage-note = Fail yang sudah disimpan di tempat lama kekal di sana; alihkannya untuk terus menggunakannya. Tetapan apl prev disimpan dalam { $file }.
settings-save-failed = Tidak dapat menyimpan tetapan: { $error }
settings-no-location = Tiada lokasi tetapan: HOME tidak ditetapkan
settings-full-path = Gunakan laluan penuh, contohnya ~/Documents/prev.
settings-path-is-folder = { $path } ialah folder, bukan fail.
settings-folder-missing = Folder { $path } tidak wujud. Ciptanya dahulu, atau pilih folder lain.
settings-path-is-file = { $path } ialah fail, bukan folder.
settings-cannot-write = prev tidak dapat menulis dalam { $path }: { $error }.

## Export dialog

export-title = Eksport
export-format = Format fail
export-quality = Kualiti
export-size = Saiz
export-choose = Eksport…
export-format-webp = WebP (tanpa kehilangan kualiti)
export-format-unknown = imej
export-quality-low = Rendah
export-quality-medium = Sederhana
export-quality-high = Tinggi
export-quality-best = Terbaik
export-size-actual = Saiz sebenar
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } piksel
export-dialog-failed = Tidak dapat menunjukkan dialog simpan: { $error }
export-done = { $path } telah dieksport
export-failed = Tidak dapat mengeksport: { $error }
export-stopped = pengeksportan terhenti

## Image window

image-marked-no-edit = Imej yang mempunyai penandaan tidak boleh diedit. Eksport untuk mengekalkan penandaan, atau padamkan penandaan dan tutup bar penandaan.

image-loading-stopped = pemuatan terhenti
image-reverting-stopped = pengembalian terhenti
image-rendering-stopped = pemaparan terhenti
image-saving-stopped = penyimpanan terhenti
image-markup-stopped = penandaan terhenti
image-no-version-store = Tiada tempat untuk menyimpan versi
image-revert-failed = Tidak dapat mengembalikan: { $error }
image-read-failed = Tidak dapat membaca { $path }: { $error }
image-keep-original-failed = Tidak dapat mengekalkan versi asal: { $error }
image-save-failed = Tidak dapat menyimpan { $path }: { $error }
image-markup-start-failed = Tidak dapat memulakan penandaan: { $error }
image-cannot-edit = Animasi dan lukisan SVG tidak boleh diedit.
image-cannot-mark-up = Animasi dan lukisan SVG tidak boleh diberi penandaan.
image-mark-up-wait = Tunggu sehingga pengeditan selesai, kemudian buat penandaan.
image-crop-needs-selection = Seret pilihan dahulu (alat Pilih), kemudian kerat.
image-size-needed = Masukkan lebar dan tinggi dalam piksel.
image-cannot-save-format = Perubahan pada “{ $name }” tidak boleh disimpan dalam formatnya. Gunakan Eksport ({ $keys }).
image-cannot-save-format-unbound = Perubahan pada “{ $name }” tidak boleh disimpan dalam formatnya. Gunakan Eksport.
image-cannot-export-animation = Animasi belum boleh dieksport.
image-drop-pages = Halaman hanya boleh dilepaskan pada dokumen.
image-drag-failed = Tidak dapat mula menyeret.
image-picture-save-failed = Tidak dapat menyimpan imej dalam folder Muat Turun anda.
image-open-failed = prev tidak dapat membuka imej ini
image-opening = Membuka…
image-name-mismatch-title = Nama tidak sepadan dengan format
image-name-mismatch = “{ $name }” akan disimpan sebagai fail { $format }, tetapi namanya berakhir dengan .{ $extension }. Apl lain mungkin tidak dapat membukanya.
image-name-mismatch-no-extension = “{ $name }” akan disimpan sebagai fail { $format }, tetapi namanya tiada sambungan. Apl lain mungkin tidak dapat membukanya.
image-choose-again = Pilih Semula
image-save-as-is = Simpan Seperti Sedia Ada
image-dimensions = { $width } × { $height }
image-frame-position = bingkai { $current } daripada { $total }
image-position = { $current } daripada { $total }
image-edited = diedit
image-sidebar = Bar sisi
image-zoom-out = Zum keluar
image-zoom-in = Zum masuk
image-zoom = { $percent }%
image-fit = Muat dalam tetingkap
image-actual-size = Saiz sebenar
image-undo = Buat asal
image-redo = Buat semula
image-rotate-left = Putar ke kiri
image-rotate-right = Putar ke kanan
image-flip-horizontal = Balikkan mendatar
image-flip-vertical = Balikkan menegak
image-select = Pilihan segi empat
image-crop = Kerat mengikut pilihan
image-adjust-size-tool = Laraskan saiz
image-adjust-color-tool = Laraskan warna
image-inspector = Pemeriksa
image-markup = Penandaan
image-export = Eksport
image-settings = Tetapan
image-adjust-color = Laraskan Warna
image-adjust-size = Laraskan Saiz
image-exposure = Dedahan
image-contrast = Kontras
image-saturation = Ketepuan
image-temperature = Suhu
image-tint = Seri warna
image-sepia = Kesan sepia
image-sharpness = Ketajaman
image-levels = Aras
image-black-point = Titik hitam
image-midtones = Ton tengah
image-white-point = Titik putih
image-reset-all = Tetapkan Semula Semua
image-current-size = Saiz semasa: { $width } × { $height } piksel
image-width = Lebar
image-height = Tinggi
image-scale-proportionally = Skala secara berkadar
image-resize = Ubah Saiz
image-inspector-loading = Memuatkan…
image-file = Fail
image-format = Format fail
image-dimensions-label = Dimensi
image-pixels = { $width } × { $height } piksel
image-no-camera = Tiada maklumat kamera.
image-location = Lokasi
image-remove-location = Alih Keluar Maklumat Lokasi
image-no-location = Tiada maklumat lokasi.
image-keywords-description = Kata Kunci dan Keterangan
image-keywords-hint = Kata kunci, dipisahkan dengan koma
image-description = Keterangan
image-keywords-unsupported = Kata kunci boleh disimpan dalam fail JPEG, PNG dan WebP.
image-revert-to = Kembali kepada
image-no-versions = Tiada versi terdahulu.
image-revert = Kembalikan
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Tutup tanpa mengeksport penandaan?
image-close-body = { $count ->
   *[other] Penandaan pada imej hanya kekal selagi tetingkapnya terbuka. Eksport imej untuk menyimpan penandaan: penandaan dilukis ke dalam salinan yang anda simpan.
}
image-close-anyway = Tutup Juga

## Markdown

markdown-reading-stopped = pembacaan terhenti
markdown-read-failed = prev tidak dapat membaca fail ini
markdown-draw-failed = Tidak dapat melukis dokumen
markdown-export-size = Keseluruhan dokumen, { $width } × { $height } piksel
markdown-not-found = Tidak ditemui
markdown-match = { $current } daripada { $total }
markdown-search = Cari
markdown-smaller-text = Teks lebih kecil
markdown-larger-text = Teks lebih besar
markdown-zoom = { $percent }%
markdown-actual-size = Saiz sebenar
markdown-inspector = Pemeriksa
markdown-export = Eksport
markdown-settings = Tetapan
markdown-file = Fail
markdown-document = Dokumen
markdown-words = Perkataan
markdown-lines = Baris
markdown-pictures = Gambar

## Image details

image-meta-camera = Kamera
image-meta-exposure = Dedahan
image-meta-image = Imej
image-meta-make = Jenama
image-meta-model = Model kamera
image-meta-lens = Kanta
image-meta-exposure-time = Masa dedahan
image-meta-f-number = Nombor-f
image-meta-iso = ISO
image-meta-focal-length = Panjang fokus
image-meta-exposure-bias = Pincangan dedahan
image-meta-flash = Denyar
image-meta-date-taken = Tarikh diambil
image-meta-orientation = Orientasi
image-meta-color-space = Ruang warna
image-meta-software = Perisian
image-meta-artist = Artis
image-meta-copyright = Hak cipta
image-meta-seconds = { $value } saat
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Biasa
    [2] Dicerminkan secara mendatar
    [3] Diputar 180°
    [4] Dicerminkan secara menegak
    [5] Dicerminkan secara mendatar, diputar 90° lawan arah jam
    [6] Diputar 90° ikut arah jam
    [7] Dicerminkan secara mendatar, diputar 90° ikut arah jam
    [8] Diputar 90° lawan arah jam
   *[other] Tidak diketahui ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Menyala
   *[no] Tidak menyala
}{ $mode ->
    [on] , dipaksa hidup
    [off] , mati
    [auto] , automatik
   *[unknown] {""}
}{ $redeye ->
    [yes] , pengurangan mata merah
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Tidak ditentukur
   *[other] Lain-lain ({ $code })
}

## Errors

error-pdf-open = tidak dapat membuka dokumen: { $detail }
error-pdf-page-out-of-range = halaman { $page } tidak wujud
error-pdf-password-protected = dokumen dilindungi kata laluan; bukanya dan salin halamannya
error-pdf-no-pages = tiada halaman untuk diekstrak
error-pdf-crop-outside = kawasan keratan berada di luar halaman
error-pdf-closed = dokumen telah ditutup
error-pdf-saved-unreadable = dokumen yang disimpan tidak dapat dibuka lagi
error-image-read = tidak dapat membaca fail: { $detail }
error-image-invalid = imej rosak atau tidak sah: { $detail }
error-image-missing-library = membuka format ini memerlukan { $library }, yang tidak dipasang
error-image-unsupported = imej { $format } belum disokong
error-image-encode = tidak dapat mengekodkan imej: { $detail }
error-exif-malformed = data EXIF cacat
error-settings-read = tidak dapat membaca tetapan: { $detail }
error-settings-invalid = tetapan tidak sah: { $detail }
error-remove-location = tidak dapat mengalih keluar lokasi: { $error }
error-location-unsupported = maklumat lokasi boleh dialih keluar daripada fail JPEG, PNG, WebP dan TIFF
error-xmp-unsupported = kata kunci dan keterangan hanya boleh disimpan dalam fail JPEG, PNG dan WebP

## Formats

format-camera-raw = RAW Kamera

## The macOS menu bar, named as in macOS's own apps.
menu-about = Perihal prev
menu-settings = Tetapan…
menu-services = Perkhidmatan
menu-hide = Sembunyikan prev
menu-hide-others = Sembunyikan Yang Lain
menu-show-all = Tunjukkan Semua
menu-quit = Keluar prev
menu-file = Fail
menu-open = Buka…
menu-close = Tutup Tetingkap
menu-export = Eksport…
menu-print = Cetak…
menu-edit = Edit
menu-undo = Buat Asal
menu-redo = Buat Semula
menu-cut = Potong
menu-copy = Salin
menu-paste = Tampal
menu-select-all = Pilih Semua
menu-find = Cari
menu-find-next = Cari Seterusnya
menu-find-previous = Cari Sebelumnya
menu-view = Paparan
menu-hide-sidebar = Sembunyikan Bar Sisi
menu-thumbnails = Lakaran Kecil
menu-contents = Isi Kandungan
menu-notes = Serlahan dan Nota
menu-bookmarks = Penanda Buku
menu-zoom-in = Zum Masuk
menu-zoom-out = Zum Keluar
menu-actual-size = Saiz Sebenar
menu-zoom-to-fit = Zum untuk Muat
menu-inspector = Tunjukkan Pemeriksa
menu-slideshow = Tayangan Slaid
menu-full-screen = Masuk Skrin Penuh
menu-go = Pergi
menu-next-page = Halaman Seterusnya
menu-previous-page = Halaman Sebelumnya
menu-go-to-page = Pergi ke Halaman…
menu-bookmark = Tambah Penanda Buku
menu-tools = Alatan
menu-markup = Tunjukkan Bar Alat Penandaan
menu-rotate-left = Putar ke Kiri
menu-rotate-right = Putar ke Kanan
menu-crop = Kerat
menu-adjust-color = Laraskan Warna…
menu-window = Tetingkap
menu-minimize = Minimumkan
menu-zoom = Zum
menu-bring-all-to-front = Bawa Semua ke Hadapan
