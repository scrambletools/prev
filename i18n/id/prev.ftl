# prev's interface text in Indonesian (Bahasa Indonesia), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = markah, note = catatan, highlight = sorotan,
# annotation = anotasi, redact/redaction = sensor, inspector = inspektur,
# bookmark = penanda, page = halaman, file = berkas, settings = pengaturan,
# zoom in/out = perbesar/perkecil, undo/redo = urungkan/ulangi,
# default = bawaan, crop = pangkas, export = ekspor, drop = letakkan,
# SVG drawing = grafik SVG, path = jalur.
# Buttons and menu items use the imperative verb (Simpan, Batal, Tutup).

## Language

language-name = Bahasa Indonesia

## Common

common-cancel = Batal
common-close = Tutup
common-save = Simpan

## Settings

settings-title = Pengaturan
settings-appearance = Tampilan
settings-colors = Warna
settings-windows = Jendela
settings-default-app = Aplikasi bawaan
settings-default-app-label = Buka berkas dengan prev
settings-default-app-note = Jadikan prev sebagai aplikasi yang membuka PDF, gambar, grafik SVG, dan berkas Markdown.
settings-default-app-note-windows = Windows hanya mengizinkan pemilihan aplikasi bawaan di Pengaturannya sendiri. Tombol ini membuka halaman prev di sana.
settings-default-app-note-macos = macOS meminta Anda mengonfirmasi setiap jenis: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP, dan AVIF.
settings-default-app-status = { $set } dari { $total } jenis berkas dibuka dengan prev.
settings-default-app-button = Jadikan Bawaan
settings-default-app-button-windows = Buka Pengaturan
settings-default-app-no-entry = Entri desktop prev belum terpasang, sehingga sistem tidak dapat membuka berkas dengannya. Pasang prev dari paket atau dengan scripts/install.sh.
settings-default-app-no-bundle = Buka prev dari prev.app untuk menjadikannya aplikasi bawaan.
settings-default-app-failed = Tidak dapat menjadikan prev sebagai aplikasi bawaan: { $error }
settings-storage = Penyimpanan
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (versi pengembangan, { $build })

## Markup toolbar

markup-tool-select = Pilih
markup-tool-area = Seleksi persegi panjang
markup-tool-sketch = Sketsa
markup-tool-draw = Gambar
markup-tool-shapes = Bentuk
markup-tool-text-box = Kotak teks
markup-tool-highlight = Sorot
markup-tool-note = Catatan
markup-tool-sign = Tanda tangani
markup-tool-redact = Sensor
markup-apply = Terapkan
markup-apply-redactions = Terapkan sensor
markup-shape-style = Gaya bentuk
markup-border-color = Warna garis tepi
markup-fill-color = Warna isian
markup-text-style = Gaya teks
markup-delete = Hapus
markup-undo = Urungkan
markup-redo = Ulangi

## Markup menus

markup-shape-rectangle = Persegi Panjang
markup-shape-rounded-rectangle = Persegi Panjang Membulat
markup-shape-oval = Lonjong
markup-shape-line = Garis
markup-shape-arrow = Panah
markup-shape-star = Bintang
markup-shape-polygon = Poligon
markup-shape-speech-bubble = Balon Ucapan
markup-shape-loupe = Kaca Pembesar
markup-shape-mask = Masker
markup-style-highlight = Sorotan
markup-style-underline = Garis bawah
markup-style-strikethrough = Coret
markup-style-squiggly = Garis bergelombang
markup-menu-color = Warna
markup-menu-font = Fon
markup-menu-size = Ukuran
markup-menu-alignment = Perataan
markup-line-width = { $width } pt
markup-dashed = Putus-putus

## Notes

markup-kind-note = Catatan
markup-kind-text-box = Kotak teks
markup-kind-stamp = Stempel
markup-kind-redaction = Sensor
markup-kind-shape = Bentuk
markup-note-delete = Hapus catatan
markup-note-done = Selesai
markup-note-placeholder = Ketik catatan
markup-notes-empty = Tidak ada sorotan atau catatan
markup-notes-empty-hint = Sorotan, catatan, dan kotak teks muncul di sini.
markup-notes-page = Halaman { $page }

## Markup errors

markup-change-failed = Tidak dapat mengubah dokumen: { $error }
markup-copy-area-failed = Tidak dapat menyalin area: { $error }
markup-document-closed = dokumen ditutup
markup-render-area-failed = tidak dapat merender area
markup-copy-stopped = penyalinan terhenti

## Signatures

signature-menu-empty = Belum ada tanda tangan.
signature-delete = Hapus tanda tangan
signature-create = Buat Tanda Tangan…
signature-dialog-title = Buat Tanda Tangan
signature-tab-draw = Gambar
signature-tab-type = Ketik
signature-tab-image = Berkas gambar
signature-draw-hint = Tanda tangani di atas garis dengan mouse, pena, atau touchpad.
signature-your-name = Nama Anda
signature-image-hint = Pilih foto atau pindaian tanda tangan Anda di atas kertas putih.
signature-choose-image = Pilih Gambar…
signature-description = Deskripsi, misalnya Nama lengkap atau Inisial
signature-clear = Bersihkan
signature-ink = Tinta
signature-thickness = Ketebalan
signature-sign-first = Tanda tangani dulu, lalu simpan.
signature-default-name = Tanda Tangan { $number }
signature-change-failed = Tidak dapat mengubah tanda tangan: { $error }
signature-no-data-folder = tidak ada folder data: HOME tidak diatur
signature-removing-stopped = penghapusan terhenti
signature-saving-stopped = penyimpanan terhenti
signature-reading-stopped = pembacaan terhenti
signature-not-an-image = berkas tersebut bukan gambar yang dapat dibaca prev
signature-no-frames = gambar tidak memiliki bingkai
signature-not-found = tidak ada tanda tangan yang ditemukan di gambar

## Dragging

drag-pages-need-document = Halaman hanya dapat diletakkan di dokumen.
drag-image-unsupported = prev tidak dapat membuka gambar ini.
drag-area-failed = Tidak dapat menyeret area: { $error }
drag-pages-failed = Tidak dapat menyeret halaman: { $error }
drag-start-failed = Tidak dapat mulai menyeret.
drag-file-pages = Halaman
drag-file-one-page = { $name } (halaman { $page })
drag-file-page-range = { $name } (halaman { $first }–{ $last })
drag-file-image = Gambar
drop-pdf-title = Tambahkan ke dokumen ini?
drop-pdf-body = Tambahkan “{ $name }” ke akhir dokumen ini, atau buka di jendelanya sendiri?
drop-pdfs-body = Tambahkan { $count } PDF ini ke akhir dokumen ini, atau buka masing-masing di jendelanya sendiri?
drop-pdf-add = Tambahkan ke Akhir
drop-pdf-open = Buka Terpisah

## PDF window

pdf-opening = Membuka…
pdf-open-failed = prev tidak dapat membuka dokumen ini
pdf-no-pages = Dokumen tidak memiliki halaman.
pdf-document-closed = dokumen ditutup
pdf-keep-original-failed = tidak dapat menyimpan versi asli: { $error }
pdf-save-failed = Tidak dapat menyimpan: { $error }
pdf-nothing-to-paste = Tidak ada yang dapat ditempel.
pdf-pasting-stopped = penempelan terhenti
pdf-file-dialog-failed = Tidak dapat menampilkan dialog berkas: { $error }
pdf-bookmarks-no-home = Penanda tidak dapat disimpan: HOME tidak diatur
pdf-bookmarks-save-failed = Tidak dapat menyimpan penanda: { $error }
pdf-bookmark-page = Halaman { $page }

pdf-password-protected = “{ $name }” dilindungi kata sandi
pdf-password = Kata sandi
pdf-password-wrong = Kata sandi salah. Coba lagi.
pdf-unlock = Buka Kunci

pdf-sidebar = Bilah samping
pdf-page-of = dari { $count }
pdf-zoom-out = Perkecil
pdf-zoom-in = Perbesar
pdf-zoom-percent = { $percent }%
pdf-fit-page = Paskan halaman
pdf-fit-width = Paskan lebar
pdf-actual-size = Ukuran sebenarnya
pdf-view-continuous = Gulir bersambung
pdf-view-single-page = Satu halaman
pdf-view-two-pages = Dua halaman
pdf-undo = Urungkan
pdf-redo = Ulangi
pdf-rotate-left = Putar ke kiri
pdf-rotate-right = Putar ke kanan
pdf-inspector = Inspektur
pdf-markup = Markah
pdf-export = Ekspor
pdf-settings = Pengaturan

pdf-search = Cari
pdf-search-not-found = Tidak ditemukan
pdf-searching = Mencari…
pdf-search-match = { $current } dari { $total }
pdf-search-match-more = { $current } dari { $total }+

pdf-inspector-file = Berkas
pdf-inspector-document = Dokumen
pdf-inspector-pages = Halaman
pdf-inspector-title = Judul
pdf-inspector-author = Penulis
pdf-inspector-subject = Subjek
pdf-inspector-keywords = Kata kunci
pdf-inspector-created = Dibuat
pdf-inspector-modified = Diubah
pdf-inspector-application = Aplikasi
pdf-inspector-producer = Pembuat PDF
pdf-inspector-version = Versi
pdf-inspector-security = Keamanan
pdf-inspector-not-encrypted = Tidak dienkripsi
pdf-inspector-encrypted = Dienkripsi ({ $method })
pdf-inspector-page-count = { $count ->
   *[other] { $count } halaman
}
pdf-inspector-page-size = Ukuran halaman
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inci)
pdf-loading = Memuat…

pdf-tab-pages = Halaman
pdf-tab-contents = Daftar isi
pdf-tab-notes = Sorotan dan catatan
pdf-tab-bookmarks = Penanda
pdf-no-outline = Tidak ada daftar isi
pdf-no-outline-detail = Dokumen ini tidak memiliki kerangka.
pdf-no-bookmarks = Tidak ada penanda
pdf-no-bookmarks-detail = Tekan { $keys } untuk menandai halaman.
pdf-no-bookmarks-detail-unbound = Halaman yang ditandai muncul di sini.
pdf-remove-bookmark = Hapus penanda

## Page editing

pages-menu = Halaman
pages-insert-blank = Sisipkan Halaman Kosong
pages-insert-file = Sisipkan dari Berkas…
pages-copy = { $count ->
   *[other] Salin Halaman
}
pages-paste = { $count ->
   *[other] Tempel { $count } Halaman
}
pages-crop = Pangkas ke Seleksi
pages-select-all = Pilih Semua Halaman
pages-delete = { $count ->
   *[other] Hapus Halaman
}
pages-apply-redactions = Terapkan Sensor…
pages-no-copied = Tidak ada halaman tersalin untuk ditempel.
pages-copied = { $count ->
   *[other] { $count } halaman disalin.
}
pages-copy-failed = Tidak dapat menyalin halaman: { $error }
pages-reading-stopped = pembacaan terhenti
pages-image-unreadable = bukan gambar yang dapat dibaca prev
pages-read-failed = Tidak dapat membaca berkas: { $error }
pages-at-least-one = Dokumen memerlukan setidaknya satu halaman.
pages-crop-needs-area = Pilih area dengan alat seleksi persegi panjang terlebih dahulu.
pages-change-failed = Tidak dapat mengubah halaman: { $error }
pages-no-redactions = Tidak ada sensor yang perlu diterapkan.
pages-redactions-applied = { $count ->
   *[other] { $count } sensor diterapkan.
}
pages-forget-versions-failed = Tidak dapat menghapus versi sebelumnya: { $error }
pages-redact-title = Terapkan sensor?
pages-redact-body = { $count ->
   *[other] Teks, gambar, dan coretan di bawah { $count } tanda sensor akan dihapus dari dokumen secara permanen, dan tanda tersebut menjadi kotak hitam. Tindakan ini tidak dapat diurungkan, dan versi sebelumnya dari berkas ini yang disimpan prev akan dihapus.
}
pages-redact-apply = Terapkan

## PDF export

pages-export-title = Ekspor
pages-export-format = Format berkas
pages-export-reduce = Perkecil ukuran berkas (gambar pada 150 dpi)
pages-export-flatten = Ratakan anotasi dan kolom formulir
pages-export-flatten-detail = Markah dan kolom yang sudah diisi menjadi bagian dari halaman dan tidak dapat diedit lagi. Tanda sensor yang belum diterapkan tidak disertakan.
pages-export-encrypt = Enkripsi dengan kata sandi
pages-export-password = Kata sandi
pages-export-verify-password = Verifikasi kata sandi
pages-export-resolution = Resolusi
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kualitas
pages-export-quality-low = Rendah
pages-export-quality-medium = Sedang
pages-export-quality-high = Tinggi
pages-export-quality-best = Terbaik
pages-export-one-file = Semua halaman masuk ke satu berkas.
pages-export-file-per-page = Setiap halaman disimpan sebagai berkas tersendiri, dengan nama yang Anda pilih diikuti nomor urut.
pages-export-selected-only = { $count ->
   *[other] Hanya { $count } halaman yang dipilih
}
pages-export-choose = Ekspor…
pages-export-no-password = Masukkan kata sandi.
pages-export-password-mismatch = Kata sandi tidak cocok.
pages-export-file-name = { $name } (diekspor)
pages-export-untitled = dokumen
pages-export-same-file = Ekspor ke berkas baru; dokumen ini tersimpan otomatis.
pages-export-exporting = Mengekspor “{ $name }”…
pages-export-done = “{ $name }” telah diekspor.
pages-export-done-images = { $count } gambar telah diekspor.
pages-export-failed = Tidak dapat mengekspor: { $error }
pages-export-stopped = ekspor terhenti

## Start window

app-start-hint = Buka atau letakkan berkas PDF, gambar, SVG, atau Markdown.
app-start-open = Buka…
app-title-dev = { $title } (pengembangan)
app-viewer-missing = { $kind }: penampil ini belum dibuat.
app-cannot-open = prev tidak dapat membuka jenis berkas ini.
app-cannot-read = prev tidak dapat membaca berkas ini: { $error }
app-kind-pdf = Dokumen PDF
app-kind-image = Gambar { $format }
app-kind-svg = Grafik SVG
app-kind-markdown = Dokumen Markdown
app-file-dialog-failed = Tidak dapat menampilkan dialog berkas: { $error }

## Actions

action-open = Buka
action-settings = Pengaturan

## Toolbar

app-toolbar-keep-shown = Selalu tampilkan bilah alat
app-toolbar-auto-hide = Sembunyikan bilah alat saat penunjuk keluar
app-toolbar-more = Lainnya

## File facts

app-fact-name = Nama
app-fact-folder = Folder
app-fact-size = Ukuran
app-fact-modified = Diubah
app-size-bytes = { $count } byte
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Tautan tidak valid { $uri }: { $error }
app-link-open-failed = Tidak dapat membuka { $uri }: { $error }
app-paste-needs-wl-clipboard = pasang wl-clipboard untuk menempel gambar
app-copy-needs-wl-clipboard = pasang wl-clipboard untuk menyalin gambar
app-copy-no-pixels = area tidak memiliki piksel
app-copy-no-input = wl-copy tidak menerima masukan
app-copy-failed = wl-copy gagal
app-clipboard-open-failed = Tidak dapat membuka papan klip: { $error }
app-copy-image-failed = Tidak dapat menyalin gambar: { $error }

## Printing

print-failed = Tidak dapat mencetak: { $error }
print-stopped = Pencetakan terhenti
print-unavailable = Pencetakan belum tersedia di sistem ini.
print-no-window = Tidak dapat mencetak: tidak ada jendela untuk menampilkan dialog cetak
print-dialog-failed = Tidak dapat menampilkan dialog cetak: { $error }
print-job-not-started = printer tidak memulai pekerjaan cetak
print-printer-stopped = printer berhenti

## File dialogs

dialog-open = Buka
dialog-filter-all = Semua berkas yang didukung
dialog-filter-pdf = Dokumen PDF
dialog-filter-images = Gambar
dialog-filter-svg = Grafik SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Pilih folder tanda tangan
dialog-choose-versions = Pilih folder riwayat versi
dialog-choose-bookmarks = Pilih berkas penanda

## Command line

usage-help =
    Penggunaan: prev [FILE]...
                prev --mcp

    Lihat dan edit PDF dan gambar. Berkas dibuka di jendela prev yang sedang
    berjalan, yang akan dijalankan bila perlu.

    Opsi:
      -h, --help     Tampilkan bantuan ini
      -V, --version  Tampilkan versi
          --mcp      Sajikan MCP di stdin dan stdout, agar agen AI dapat mengendalikan
                     prev yang sedang berjalan

## Settings, continued

settings-language = Bahasa
settings-language-system = Bawaan sistem: { $language }
settings-input-language = Bahasa input
settings-input-language-system = Ikuti tata letak keyboard
settings-input-language-note = Menentukan sisi awal kolom teks yang kosong. Teks yang Anda ketik tetap mengikuti arahnya sendiri.

settings-appearance-system = Sistem
settings-appearance-light = Terang
settings-appearance-dark = Gelap
settings-system-accent = Gunakan warna aksen sistem
settings-omarchy-note = Warna dibuat dari aksen “{ $theme }”.
settings-system-accent-note = Warna dibuat dari warna aksen sistem.
settings-system-accent-none = Sistem tidak memiliki warna aksen, sehingga prev memakai warna yang dipilih di bawah.
settings-accent-chosen-note = Warna dibuat dari warna yang dipilih di bawah.
settings-auto-hide = Sembunyikan bilah alat saat penunjuk keluar
settings-auto-hide-note = Bilah alat melayang di atas dokumen dan bergeser menyingkir saat penunjuk berada di luar jendela.
settings-animations = Animasi
settings-animations-note = Bilah dan panel yang bergeser, dialog yang membesar, dan tombol yang memantul.
settings-animations-reduced = Nonaktif selama sistem meminta gerakan dikurangi.
settings-corner-radius = Radius sudut
settings-corner-radius-note = Untuk dialog dan bilah alat melayang.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparansi lapisan
settings-overlay-note = Seberapa banyak halaman yang terlihat menembus bilah alat melayang.
settings-overlay-value = { $percent }%
settings-storage-signatures = Folder tanda tangan
settings-storage-versions = Folder riwayat versi
settings-storage-bookmarks = Berkas penanda
settings-storage-apply = Terapkan
settings-storage-choose = Pilih…
settings-storage-note = Berkas yang sudah disimpan di tempat lama tetap berada di sana; pindahkan agar tetap dapat digunakan. Pengaturan aplikasi prev disimpan di { $file }.
settings-save-failed = Tidak dapat menyimpan pengaturan: { $error }
settings-no-location = Tidak ada lokasi pengaturan: HOME tidak diatur
settings-full-path = Gunakan jalur lengkap, misalnya ~/Documents/prev.
settings-path-is-folder = { $path } adalah folder, bukan berkas.
settings-folder-missing = Folder { $path } tidak ada. Buat terlebih dahulu, atau pilih folder lain.
settings-path-is-file = { $path } adalah berkas, bukan folder.
settings-cannot-write = prev tidak dapat menulis di { $path }: { $error }.

## Export dialog

export-title = Ekspor
export-format = Format berkas
export-quality = Kualitas
export-size = Ukuran
export-choose = Ekspor…
export-format-webp = WebP (tanpa penurunan kualitas)
export-format-unknown = gambar
export-quality-low = Rendah
export-quality-medium = Sedang
export-quality-high = Tinggi
export-quality-best = Terbaik
export-size-actual = Ukuran sebenarnya
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } piksel
export-dialog-failed = Tidak dapat menampilkan dialog simpan: { $error }
export-done = { $path } telah diekspor
export-failed = Tidak dapat mengekspor: { $error }
export-stopped = ekspor terhenti

## Image window

image-marked-no-edit = Gambar dengan markah tidak dapat diedit. Ekspor untuk mempertahankan markah, atau hapus markah lalu tutup bilah markah.

image-loading-stopped = pemuatan terhenti
image-reverting-stopped = pengembalian terhenti
image-rendering-stopped = perenderan terhenti
image-saving-stopped = penyimpanan terhenti
image-markup-stopped = markah terhenti
image-no-version-store = Tidak ada tempat untuk menyimpan versi
image-revert-failed = Tidak dapat mengembalikan: { $error }
image-read-failed = Tidak dapat membaca { $path }: { $error }
image-keep-original-failed = Tidak dapat menyimpan versi asli: { $error }
image-save-failed = Tidak dapat menyimpan { $path }: { $error }
image-markup-start-failed = Tidak dapat memulai markah: { $error }
image-cannot-edit = Animasi dan grafik SVG tidak dapat diedit.
image-cannot-mark-up = Animasi dan grafik SVG tidak dapat diberi markah.
image-mark-up-wait = Tunggu hingga pengeditan selesai, lalu beri markah.
image-crop-needs-selection = Seret seleksi terlebih dahulu (alat Pilih), lalu pangkas.
image-size-needed = Masukkan lebar dan tinggi dalam piksel.
image-cannot-save-format = Perubahan pada “{ $name }” tidak dapat disimpan dalam formatnya. Gunakan Ekspor ({ $keys }).
image-cannot-save-format-unbound = Perubahan pada “{ $name }” tidak dapat disimpan dalam formatnya. Gunakan Ekspor.
image-cannot-export-animation = Animasi belum dapat diekspor.
image-drop-pages = Halaman hanya dapat diletakkan di dokumen.
image-drag-failed = Tidak dapat mulai menyeret.
image-picture-save-failed = Tidak dapat menyimpan gambar di folder Unduhan Anda.
image-open-failed = prev tidak dapat membuka gambar ini
image-opening = Membuka…
image-name-mismatch-title = Nama tidak sesuai dengan format
image-name-mismatch = “{ $name }” akan disimpan sebagai berkas { $format }, tetapi namanya berakhiran .{ $extension }. Aplikasi lain mungkin tidak dapat membukanya.
image-name-mismatch-no-extension = “{ $name }” akan disimpan sebagai berkas { $format }, tetapi namanya tidak memiliki ekstensi. Aplikasi lain mungkin tidak dapat membukanya.
image-choose-again = Pilih Lagi
image-save-as-is = Simpan Apa Adanya
image-dimensions = { $width } × { $height }
image-frame-position = bingkai { $current } dari { $total }
image-position = { $current } dari { $total }
image-edited = diedit
image-sidebar = Bilah samping
image-zoom-out = Perkecil
image-zoom-in = Perbesar
image-zoom = { $percent }%
image-fit = Paskan ke jendela
image-actual-size = Ukuran sebenarnya
image-undo = Urungkan
image-redo = Ulangi
image-rotate-left = Putar ke kiri
image-rotate-right = Putar ke kanan
image-flip-horizontal = Balik horizontal
image-flip-vertical = Balik vertikal
image-select = Seleksi persegi panjang
image-crop = Pangkas ke seleksi
image-adjust-size-tool = Sesuaikan ukuran
image-adjust-color-tool = Sesuaikan warna
image-inspector = Inspektur
image-markup = Markah
image-export = Ekspor
image-settings = Pengaturan
image-adjust-color = Sesuaikan Warna
image-adjust-size = Sesuaikan Ukuran
image-exposure = Eksposur
image-contrast = Kontras
image-saturation = Saturasi
image-temperature = Suhu
image-tint = Rona
image-sepia = Sepia
image-sharpness = Ketajaman
image-levels = Level
image-black-point = Titik hitam
image-midtones = Nada tengah
image-white-point = Titik putih
image-reset-all = Atur Ulang Semua
image-current-size = Ukuran saat ini: { $width } × { $height } piksel
image-width = Lebar
image-height = Tinggi
image-scale-proportionally = Skalakan secara proporsional
image-resize = Ubah Ukuran
image-inspector-loading = Memuat…
image-file = Berkas
image-format = Format berkas
image-dimensions-label = Dimensi
image-pixels = { $width } × { $height } piksel
image-no-camera = Tidak ada informasi kamera.
image-location = Lokasi
image-remove-location = Hapus Info Lokasi
image-no-location = Tidak ada informasi lokasi.
image-keywords-description = Kata Kunci dan Deskripsi
image-keywords-hint = Kata kunci, dipisahkan dengan koma
image-description = Deskripsi
image-keywords-unsupported = Kata kunci dapat disimpan di berkas JPEG, PNG, dan WebP.
image-revert-to = Kembalikan ke
image-no-versions = Tidak ada versi sebelumnya.
image-revert = Kembalikan
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Tutup tanpa mengekspor markah?
image-close-body = { $count ->
   *[other] Markah pada gambar hanya bertahan selama jendelanya terbuka. Ekspor gambar untuk menyimpan markah: markah digambar ke salinan yang Anda simpan.
}
image-close-anyway = Tetap Tutup

## Markdown

markdown-reading-stopped = pembacaan terhenti
markdown-read-failed = prev tidak dapat membaca berkas ini
markdown-draw-failed = Tidak dapat menggambar dokumen
markdown-export-size = Seluruh dokumen, { $width } × { $height } piksel
markdown-not-found = Tidak ditemukan
markdown-match = { $current } dari { $total }
markdown-search = Cari
markdown-smaller-text = Teks lebih kecil
markdown-larger-text = Teks lebih besar
markdown-zoom = { $percent }%
markdown-actual-size = Ukuran sebenarnya
markdown-inspector = Inspektur
markdown-export = Ekspor
markdown-settings = Pengaturan
markdown-file = Berkas
markdown-document = Dokumen
markdown-words = Kata
markdown-lines = Baris
markdown-pictures = Gambar

## Image details

image-meta-camera = Kamera
image-meta-exposure = Eksposur
image-meta-image = Gambar
image-meta-make = Merek
image-meta-model = Tipe
image-meta-lens = Lensa
image-meta-exposure-time = Waktu eksposur
image-meta-f-number = Angka-f
image-meta-iso = ISO
image-meta-focal-length = Panjang fokus
image-meta-exposure-bias = Bias eksposur
image-meta-flash = Lampu kilat
image-meta-date-taken = Tanggal diambil
image-meta-orientation = Orientasi
image-meta-color-space = Ruang warna
image-meta-software = Perangkat lunak
image-meta-artist = Seniman
image-meta-copyright = Hak cipta
image-meta-seconds = { $value } dtk
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Dicerminkan secara horizontal
    [3] Diputar 180°
    [4] Dicerminkan secara vertikal
    [5] Dicerminkan secara horizontal, diputar 90° berlawanan arah jarum jam
    [6] Diputar 90° searah jarum jam
    [7] Dicerminkan secara horizontal, diputar 90° searah jarum jam
    [8] Diputar 90° berlawanan arah jarum jam
   *[other] Tidak diketahui ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Menyala
   *[no] Tidak menyala
}{ $mode ->
    [on] , dipaksa menyala
    [off] , mati
    [auto] , otomatis
   *[unknown] {""}
}{ $redeye ->
    [yes] , pengurangan mata merah
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Tidak terkalibrasi
   *[other] Lainnya ({ $code })
}

## Errors

error-pdf-open = tidak dapat membuka dokumen: { $detail }
error-pdf-page-out-of-range = halaman { $page } tidak ada
error-pdf-password-protected = dokumen dilindungi kata sandi; buka dokumen tersebut lalu salin halamannya
error-pdf-no-pages = tidak ada halaman untuk diekstrak
error-pdf-crop-outside = area pangkas berada di luar halaman
error-pdf-closed = dokumen ditutup
error-pdf-saved-unreadable = dokumen yang disimpan tidak dapat dibuka lagi
error-image-read = tidak dapat membaca berkas: { $detail }
error-image-invalid = gambar rusak atau tidak valid: { $detail }
error-image-missing-library = membuka format ini memerlukan { $library }, yang belum terpasang
error-image-unsupported = gambar { $format } belum didukung
error-image-encode = tidak dapat mengodekan gambar: { $detail }
error-exif-malformed = data EXIF rusak
error-settings-read = tidak dapat membaca pengaturan: { $detail }
error-settings-invalid = pengaturan tidak valid: { $detail }
error-remove-location = tidak dapat menghapus lokasi: { $error }
error-location-unsupported = info lokasi dapat dihapus dari berkas JPEG, PNG, WebP, dan TIFF
error-xmp-unsupported = kata kunci dan deskripsi hanya dapat disimpan di berkas JPEG, PNG, dan WebP

## Formats

format-camera-raw = RAW Kamera

## The macOS menu bar, named as in macOS's own apps.
menu-about = Tentang prev
menu-settings = Pengaturan…
menu-services = Layanan
menu-hide = Sembunyikan prev
menu-hide-others = Sembunyikan Lainnya
menu-show-all = Tampilkan Semua
menu-quit = Keluar dari prev
menu-file = Berkas
menu-open = Buka…
menu-close = Tutup Jendela
menu-export = Ekspor…
menu-print = Cetak…
menu-edit = Edit
menu-undo = Urungkan
menu-redo = Ulangi
menu-cut = Potong
menu-copy = Salin
menu-paste = Tempel
menu-select-all = Pilih Semua
menu-find = Cari
menu-find-next = Cari Berikutnya
menu-find-previous = Cari Sebelumnya
menu-view = Tampilan
menu-hide-sidebar = Sembunyikan Bilah Samping
menu-thumbnails = Gambar Mini
menu-contents = Daftar Isi
menu-notes = Sorotan dan Catatan
menu-bookmarks = Penanda
menu-zoom-in = Perbesar
menu-zoom-out = Perkecil
menu-actual-size = Ukuran Sebenarnya
menu-zoom-to-fit = Perbesar agar Pas
menu-inspector = Tampilkan Inspektur
menu-slideshow = Peragaan Slide
menu-full-screen = Masuk Layar Penuh
menu-go = Pergi
menu-next-page = Halaman Berikutnya
menu-previous-page = Halaman Sebelumnya
menu-go-to-page = Pergi ke Halaman…
menu-bookmark = Tambah Penanda
menu-tools = Alat
menu-markup = Tampilkan Bilah Alat Markah
menu-rotate-left = Putar ke Kiri
menu-rotate-right = Putar ke Kanan
menu-crop = Pangkas
menu-adjust-color = Sesuaikan Warna…
menu-window = Jendela
menu-minimize = Minimalkan
menu-zoom = Maksimalkan
menu-bring-all-to-front = Bawa Semua ke Depan

## Outside control

settings-outside-control = Kendali luar
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = Umum
settings-tab-agents = Agen
settings-allow-outside-control = Izinkan kendali luar
settings-allow-outside-control-note = Agen AI seperti Claude Code dapat membaca dan mengubah berkas Anda di prev melalui prev --mcp. prev akan bertanya sebelum setiap agen baru.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Diizinkan: { $agents }
settings-forget-agents = Lupakan
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Izinkan { $agent } mengendalikan prev?
agent-prompt-body = { $agent } meminta untuk memakai kendali luar prev, untuk membaca dan mengubah berkas Anda yang terbuka. Anda dapat menonaktifkan kendali luar di Pengaturan.
agent-prompt-allow = Izinkan
agent-prompt-deny = Jangan izinkan
settings-ask-before-note = Tanya dulu sebelum agen:
settings-ask-reading = Membaca berkas
settings-ask-viewing = Mengubah tampilan atau jendela
settings-ask-marking-up = Memberi markah pada berkas
settings-ask-editing = Mengedit berkas
settings-ask-signing = Menandatangani berkas
settings-ask-redacting = Menerapkan sensor
settings-ask-exporting = Mengekspor berkas
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Izinkan { $agent } membaca berkas ini?
agent-ask-view = Izinkan { $agent } mengubah tampilan?
agent-ask-markup = Izinkan { $agent } memberi markah pada berkas ini?
agent-ask-edit = Izinkan { $agent } mengedit berkas ini?
agent-ask-sign = Izinkan { $agent } menandatangani berkas ini?
agent-ask-redact = Izinkan { $agent } menerapkan sensor?
agent-ask-export = Izinkan { $agent } mengekspor berkas ini?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } meminta untuk memakai “{ $tool }”. Pengaturan menentukan apa saja yang ditanyakan prev.
agent-ask-final = Ini tidak dapat diurungkan.

## The assistant
settings-tab-assistant = Asisten
settings-assistant-note = Model yang dapat diajak bicara oleh panel asisten. Kunci disimpan di penyimpanan kredensial sistem.
settings-assistant-none = Belum ada model. Tambahkan satu di bawah: model lokal, seperti milik Ollama, tetap berada di komputer ini; model cloud memerlukan kunci API dari penyedianya.
settings-assistant-in-use = Sedang dipakai
settings-assistant-use = Pakai
settings-assistant-remove = Hapus
settings-assistant-add = Tambahkan model
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Server yang kompatibel dengan OpenAI
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Model, misalnya { $example }
settings-assistant-key = Kunci API
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Alamat, misalnya { $example }
# The menu of how much a local model reads at once.
settings-assistant-context = Konteks
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = { $thousands }K token
settings-assistant-context-note = Konteks yang lebih besar membuat asisten dapat membaca lebih banyak isi berkas dalam satu obrolan, tetapi model memakai lebih banyak memori dan mungkin menjawab lebih lambat.
settings-assistant-add-button = Tambah
settings-assistant-use-key = Lanjutkan
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Buat kunci di situs { $provider } lalu tempelkan di sini.
settings-assistant-get-key = Dapatkan kunci API
settings-assistant-key-kept = Kunci { $provider } Anda disimpan di penyimpanan kredensial sistem.
settings-assistant-change-key = Ganti kunci
settings-assistant-key-refused = { $provider } menolak kunci ini. Pastikan kunci disalin utuh, dari akun yang benar.
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } berjalan di { $address }.
settings-assistant-no-server = prev tidak menemukan { $provider } yang berjalan di komputer ini. Jalankan, atau masukkan alamatnya di bawah.
settings-assistant-get-server = Dapatkan { $provider }
settings-assistant-look-again = Cari lagi
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Gunakan alamat lain
settings-assistant-looking = Mencari model…
settings-assistant-found-none = { $provider } belum memiliki model. Unduh satu model dengannya, lalu cari lagi.
settings-assistant-recommended = Disarankan
settings-assistant-uses-tools = Memakai alat
settings-assistant-sees = Dapat melihat gambar
settings-assistant-no-tools = Tidak dapat memakai alat, yang dibutuhkan asisten
settings-assistant-added-tag = Ditambahkan
settings-assistant-trying = Mencoba…
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Buka halaman
# $model is the model's name.
settings-assistant-added = { $model } menjawab dan telah ditambahkan.
settings-assistant-key-needed = Model ini memerlukan kunci API.
# $error is what the keychain said.
settings-assistant-key-failed = Kunci tidak dapat disimpan di penyimpanan kredensial: { $error }
assistant-title = Asisten
assistant-new-chat = Obrolan baru
assistant-ask = Tanyakan tentang berkas ini
assistant-send = Kirim
assistant-stop = Hentikan
assistant-thinking = Berpikir…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Pemikiran
assistant-running = Berjalan…
assistant-stopped = Dihentikan.
assistant-no-model = Tambahkan model di Pengaturan terlebih dahulu.
assistant-add-model = Asisten memerlukan model: model cloud dengan kunci API-nya, atau model lokal.
assistant-open-settings = Tambahkan model
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Sekarang berbicara dengan { $model }.
assistant-add-another = Tambahkan model…
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } di komputer ini
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } di { $host }
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Percakapan ini tidak lagi muat dalam jumlah yang dapat dibaca { $model } sekaligus. Mulai obrolan baru, atau pilih model yang dapat membaca lebih banyak.
assistant-problem-key = { $provider } menolak kunci API. Periksa di Pengaturan.
assistant-problem-rate = { $provider } meminta Anda memperlambat. Coba lagi sebentar lagi.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } meminta Anda memperlambat: { $message }
assistant-problem-credit = { $provider } menyatakan akun ini tidak memiliki kredit. Akun baru perlu membeli kredit di situs { $provider } sebelum kuncinya dapat dipakai; setelah itu coba lagi.
assistant-problem-model = { $provider } tidak memiliki model bernama { $model }. Periksa namanya di Pengaturan.
assistant-problem-unavailable = { $provider } sedang sibuk atau mengalami masalah. Coba lagi sebentar lagi.
assistant-problem-unreachable = prev tidak dapat menghubungi { $provider }. Periksa koneksi Anda, atau pastikan server sedang berjalan.
assistant-problem-refused = { $model } menolak menjawab.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } tidak menjawab: { $message }
