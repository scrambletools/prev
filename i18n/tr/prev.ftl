# prev's interface text in Turkish (Türkçe), translated from i18n/en/prev.ftl.
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
language-name = Türkçe

## Common

common-cancel = Vazgeç
common-close = Kapat
common-save = Kaydet

## Settings

settings-title = Ayarlar
settings-appearance = Görünüm
settings-colors = Renkler
settings-windows = Pencereler
settings-default-app = Varsayılan uygulama
settings-default-app-label = Dosyaları prev ile aç
settings-default-app-note = PDF'leri, görüntüleri, SVG çizimlerini ve Markdown dosyalarını açan uygulama prev olsun.
settings-default-app-note-windows = Windows varsayılan uygulamaları yalnızca kendi Ayarlar'ında seçmenize izin verir. Bu, orada prev'in sayfasını açar.
settings-default-app-note-macos = macOS her tür için onay ister: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP ve AVIF.
settings-default-app-status = { $total } dosya türünden { $set } tanesi prev ile açılıyor.
settings-default-app-button = Varsayılan yap
settings-default-app-button-windows = Ayarlar'ı aç
settings-default-app-no-entry = prev'in masaüstü girdisi yüklü değil, bu yüzden sistem dosyaları onunla açamıyor. prev'i bir paketten veya scripts/install.sh ile yükleyin.
settings-default-app-no-bundle = prev'i varsayılan yapmak için prev.app'ten açın.
settings-default-app-failed = prev varsayılan yapılamadı: { $error }
settings-storage = Depolama
settings-version = prev { $version }
settings-version-development = prev { $version } (geliştirme sürümü)

## Markup toolbar

markup-tool-select = Seç
markup-tool-area = Dikdörtgen seçim
markup-tool-sketch = Karalama
markup-tool-draw = Çiz
markup-tool-shapes = Şekiller
markup-tool-text-box = Metin kutusu
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Vurgula
markup-tool-note = Not
# Opens the menu of saved signatures (a verb).
markup-tool-sign = İmzala
# A verb: the tool that marks areas to black out.
markup-tool-redact = Karart
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Uygula
markup-apply-redactions = Karartmaları uygula
markup-shape-style = Şekil stili
markup-border-color = Kenarlık rengi
markup-fill-color = Dolgu rengi
markup-text-style = Metin stili
markup-delete = Sil
markup-undo = Geri al
markup-redo = Yinele

## Markup menus

markup-shape-rectangle = Dikdörtgen
markup-shape-rounded-rectangle = Yuvarlak Köşeli Dikdörtgen
markup-shape-oval = Oval
markup-shape-line = Çizgi
markup-shape-arrow = Ok
markup-shape-star = Yıldız
markup-shape-polygon = Çokgen
markup-shape-speech-bubble = Konuşma Balonu
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Büyüteç
# A shape that darkens the page around it.
markup-shape-mask = Maske
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Vurgu
markup-style-underline = Altı çizili
markup-style-strikethrough = Üstü çizili
markup-style-squiggly = Dalgalı çizgi
# Menu section headings.
markup-menu-color = Renk
markup-menu-font = Yazı Tipi
markup-menu-size = Boyut
markup-menu-alignment = Hizalama
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Kesikli

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Not
markup-kind-text-box = Metin kutusu
markup-kind-stamp = Damga
markup-kind-redaction = Karartma
markup-kind-shape = Şekil
# Tooltips on a note being edited.
markup-note-delete = Notu sil
markup-note-done = Bitti
markup-note-placeholder = Not yazın
markup-notes-empty = Vurgu veya not yok
markup-notes-empty-hint = Vurgular, notlar ve metin kutuları burada görünür.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Sayfa { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Belge değiştirilemedi: { $error }
markup-copy-area-failed = Alan kopyalanamadı: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = belge kapandı
markup-render-area-failed = alan işlenemedi
markup-copy-stopped = kopyalama durdu

## Signatures

signature-menu-empty = Henüz imza yok.
signature-delete = İmzayı sil
signature-create = İmza Oluştur…
signature-dialog-title = İmza Oluştur
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Çiz
signature-tab-type = Yaz
signature-tab-image = Resim
signature-draw-hint = Çizginin üzerine farenizle, kaleminizle veya dokunmatik yüzeyinizle imzalayın.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Adınız
signature-image-hint = İmzanızın beyaz kâğıt üzerindeki bir fotoğrafını veya taramasını seçin.
signature-choose-image = Resim Seç…
# Placeholder of the field naming the signature in the library.
signature-description = Açıklama, örneğin Tam ad veya Baş harfler
# Clears the drawing, typed name or image.
signature-clear = Temizle
# The color the signature is drawn or typed in.
signature-ink = Mürekkep
# The pen's width, for drawing.
signature-thickness = Kalınlık
signature-sign-first = Önce imzalayın, sonra kaydedin.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = İmza { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = İmzalar değiştirilemedi: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = veri klasörü yok: HOME ayarlanmamış
signature-removing-stopped = kaldırma durdu
signature-saving-stopped = kaydetme durdu
signature-reading-stopped = okuma durdu
signature-not-an-image = bu dosya prev'in okuyabileceği bir resim değil
signature-no-frames = resimde hiç kare yok
signature-not-found = resimde imza bulunamadı

## Dragging

drag-pages-need-document = Sayfalar bir belgenin üzerine bırakılabilir.
drag-image-unsupported = prev bu resmi açamıyor.
# $error is a lowercase reason or a technical message.
drag-area-failed = Alan sürüklenemedi: { $error }
drag-pages-failed = Sayfalar sürüklenemedi: { $error }
drag-start-failed = Sürükleme başlatılamadı.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Sayfalar
drag-file-one-page = { $name } (sayfa { $page })
drag-file-page-range = { $name } (sayfa { $first }–{ $last })

## PDF window

pdf-opening = Açılıyor…
pdf-open-failed = prev bu belgeyi açamıyor
pdf-no-pages = Belgede hiç sayfa yok.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = belge kapandı
pdf-keep-original-failed = orijinal sürüm saklanamadı: { $error }
pdf-save-failed = Kaydedilemedi: { $error }
pdf-nothing-to-paste = Yapıştırılacak bir şey yok.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = yapıştırma durdu
pdf-file-dialog-failed = Dosya iletişim kutusu gösterilemedi: { $error }
pdf-bookmarks-no-home = Yer işaretleri kaydedilemiyor: HOME ayarlanmamış
pdf-bookmarks-save-failed = Yer işaretleri kaydedilemedi: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Sayfa { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” parola korumalı
pdf-password = Parola
pdf-password-wrong = Parola yanlış. Yeniden deneyin.
# Button that opens a locked document.
pdf-unlock = Kilidi Aç

# Toolbar tooltips and labels.
pdf-sidebar = Kenar çubuğu
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = / { $count }
pdf-zoom-out = Uzaklaştır
pdf-zoom-in = Yakınlaştır
# The zoom level, such as 150%.
pdf-zoom-percent = %{ $percent }
pdf-fit-page = Sayfayı sığdır
pdf-fit-width = Genişliği sığdır
pdf-actual-size = Gerçek boyut
pdf-view-continuous = Sürekli kaydırma
pdf-view-single-page = Tek sayfa
pdf-view-two-pages = İki sayfa
pdf-undo = Geri al
pdf-redo = Yinele
pdf-rotate-left = Sola döndür
pdf-rotate-right = Sağa döndür
pdf-inspector = Denetçi
pdf-markup = İşaretleme
# Tooltip of the button that opens the export dialog.
pdf-export = Dışa aktar
pdf-settings = Ayarlar

# Search field.
pdf-search = Ara
pdf-search-not-found = Bulunamadı
pdf-searching = Aranıyor…
# The match shown, of all matches found.
pdf-search-match = { $current } / { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } / { $total }+

# Inspector: section headings.
pdf-inspector-file = Dosya
pdf-inspector-document = Belge
pdf-inspector-pages = Sayfalar
# Inspector: fact labels and values.
pdf-inspector-title = Başlık
pdf-inspector-author = Yazar
pdf-inspector-subject = Konu
pdf-inspector-keywords = Anahtar sözcükler
pdf-inspector-created = Oluşturulma
pdf-inspector-modified = Değiştirilme
pdf-inspector-application = Uygulama
pdf-inspector-producer = PDF üreticisi
pdf-inspector-version = Sürüm
pdf-inspector-security = Güvenlik
pdf-inspector-not-encrypted = Şifrelenmemiş
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Şifrelenmiş ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } sayfa
   *[other] { $count } sayfa
}
pdf-inspector-page-size = Sayfa boyutu
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inç)
pdf-loading = Yükleniyor…

# Sidebar tabs and lists.
pdf-tab-pages = Sayfalar
pdf-tab-contents = İçindekiler
pdf-tab-notes = Vurgular ve notlar
pdf-tab-bookmarks = Yer işaretleri
pdf-no-outline = İçindekiler yok
pdf-no-outline-detail = Bu belgenin içindekiler tablosu yok.
pdf-no-bookmarks = Yer işareti yok
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Bir sayfaya yer işareti eklemek için { $keys } tuşlarına basın.
pdf-no-bookmarks-detail-unbound = Yer işareti eklenen sayfalar burada görünür.
pdf-remove-bookmark = Yer işaretini kaldır

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Sayfalar
pages-insert-blank = Boş Sayfa Ekle
pages-insert-file = Dosyadan Ekle…
pages-copy = { $count ->
    [one] Sayfayı Kopyala
   *[other] Sayfaları Kopyala
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Sayfayı Yapıştır
   *[other] { $count } Sayfayı Yapıştır
}
pages-crop = Seçime Göre Kırp
pages-select-all = Tüm Sayfaları Seç
pages-delete = { $count ->
    [one] Sayfayı Sil
   *[other] Sayfaları Sil
}
pages-apply-redactions = Karartmaları Uygula…
pages-no-copied = Yapıştırılacak kopyalanmış sayfa yok.
pages-copied = { $count ->
    [one] { $count } sayfa kopyalandı.
   *[other] { $count } sayfa kopyalandı.
}
pages-copy-failed = Sayfalar kopyalanamadı: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = okuma durdu
pages-read-failed = Dosya okunamadı: { $error }
pages-at-least-one = Bir belgede en az bir sayfa olmalıdır.
pages-crop-needs-area = Önce dikdörtgen seçim aracıyla bir alan seçin.
pages-change-failed = Sayfalar değiştirilemedi: { $error }
pages-no-redactions = Uygulanacak karartma yoktu.
pages-redactions-applied = { $count ->
    [one] { $count } karartma uygulandı.
   *[other] { $count } karartma uygulandı.
}
pages-forget-versions-failed = Önceki sürümler silinemedi: { $error }
pages-redact-title = Karartmalar uygulansın mı?
pages-redact-body = { $count ->
    [one] İşaretin altındaki metin, resimler ve çizimler belgeden kalıcı olarak kaldırılır ve işaret siyah bir kutuya dönüşür. Bu işlem geri alınamaz ve prev'in sakladığı bu dosyanın önceki sürümleri silinir.
   *[other] { $count } işaretin altındaki metin, resimler ve çizimler belgeden kalıcı olarak kaldırılır ve işaretler siyah kutulara dönüşür. Bu işlem geri alınamaz ve prev'in sakladığı bu dosyanın önceki sürümleri silinir.
}
# Button that applies redactions.
pages-redact-apply = Uygula

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Dışa Aktar
pages-export-format = Biçim
pages-export-reduce = Dosya boyutunu küçült (resimler 150 dpi)
pages-export-flatten = Ek açıklamaları ve form alanlarını düzleştir
pages-export-flatten-detail = İşaretlemeler ve doldurulmuş alanlar sayfaların parçası olur ve artık düzenlenemez. Henüz uygulanmamış karartma işaretleri dahil edilmez.
pages-export-encrypt = Parolayla şifrele
pages-export-password = Parola
pages-export-verify-password = Parolayı doğrula
pages-export-resolution = Çözünürlük
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kalite
# JPEG quality choices.
pages-export-quality-low = Düşük
pages-export-quality-medium = Orta
pages-export-quality-high = Yüksek
pages-export-quality-best = En iyi
pages-export-one-file = Tüm sayfalar tek bir dosyaya kaydedilir.
pages-export-file-per-page = Her sayfa, seçtiğiniz adın ardından numaralandırılarak ayrı bir dosya olarak kaydedilir.
pages-export-selected-only = { $count ->
    [one] Yalnızca seçili sayfa
   *[other] Yalnızca seçili { $count } sayfa
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Dışa Aktar…
pages-export-no-password = Bir parola girin.
pages-export-password-mismatch = Parolalar eşleşmiyor.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (dışa aktarıldı)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = belge
pages-export-same-file = Yeni bir dosyaya dışa aktarın; bu belge kendiliğinden kaydedilir.
pages-export-exporting = “{ $name }” dışa aktarılıyor…
pages-export-done = “{ $name }” dışa aktarıldı.
pages-export-done-images = { $count } resim dışa aktarıldı.
pages-export-failed = Dışa aktarılamadı: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = dışa aktarma durdu

## Start window

# Under the app name in a window with no file open.
app-start-hint = Bir PDF, resim, SVG veya Markdown dosyası açın ya da buraya bırakın.
app-start-open = Aç…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (geliştirme)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: bu görüntüleyici henüz hazır değil.
app-cannot-open = prev bu tür dosyaları açamıyor.
app-cannot-read = prev bu dosyayı okuyamıyor: { $error }
app-kind-pdf = PDF belgesi
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format } resmi
app-kind-svg = SVG çizimi
app-kind-markdown = Markdown belgesi
app-file-dialog-failed = Dosya iletişim kutusu gösterilemedi: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Aç
action-settings = Ayarlar

## Toolbar

app-toolbar-keep-shown = Araç çubuğunu her zaman göster
app-toolbar-auto-hide = İşaretçi ayrılınca araç çubuğunu gizle
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Diğer

## File facts

# Labels in a file's inspector.
app-fact-name = Ad
app-fact-folder = Klasör
app-fact-size = Boyut
app-fact-modified = Değiştirilme
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } bayt
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Geçersiz bağlantı { $uri }: { $error }
app-link-open-failed = { $uri } açılamadı: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = resim yapıştırmak için wl-clipboard paketini yükleyin
app-copy-needs-wl-clipboard = resim kopyalamak için wl-clipboard paketini yükleyin
app-copy-no-pixels = alanda hiç piksel yok
# wl-copy is a program's name.
app-copy-no-input = wl-copy için girdi yok
app-copy-failed = wl-copy başarısız oldu
app-clipboard-open-failed = Pano açılamadı: { $error }
app-copy-image-failed = Resim kopyalanamadı: { $error }

## Printing

print-failed = Yazdırılamadı: { $error }
print-stopped = Yazdırma durdu
print-unavailable = Bu sistemde yazdırma henüz kullanılamıyor.
print-no-window = Yazdırılamadı: yazdırma iletişim kutusunu gösterecek bir pencere yok
print-dialog-failed = Yazdırma iletişim kutusu gösterilemedi: { $error }
# Shown after "Could not print:".
print-job-not-started = yazıcı işi başlatmadı
# Shown after "Could not print:".
print-printer-stopped = yazıcı durdu

## File dialogs

dialog-open = Aç
dialog-filter-all = Desteklenen tüm dosyalar
dialog-filter-pdf = PDF belgeleri
dialog-filter-images = Resimler
dialog-filter-svg = SVG çizimleri
dialog-filter-markdown = Markdown
dialog-choose-signatures = İmzalar klasörünü seçin
dialog-choose-versions = Sürüm geçmişi klasörünü seçin
dialog-choose-bookmarks = Yer işaretleri dosyasını seçin

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Kullanım: prev [FILE]...

    PDF'leri ve resimleri görüntüleyin ve düzenleyin. Dosyalar, çalışan prev'in
    pencerelerinde açılır; prev gerekirse başlatılır.

    Seçenekler:
      -h, --help     Bu yardımı göster
      -V, --version  Sürümü göster

## Settings, continued

settings-language = Dil
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Sistem varsayılanı: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Giriş dili
settings-input-language-system = Klavye düzenine uy
settings-input-language-note = Boş bir metin alanının hangi taraftan başlayacağını belirler. Yazdığınız metin kendi yönünü korur.

settings-appearance-system = Sistem
settings-appearance-light = Açık
settings-appearance-dark = Koyu
settings-omarchy-accent = Omarchy vurgu rengini kullan
# $theme is the Omarchy theme's name.
settings-omarchy-note = Renkler “{ $theme }” temasının vurgu renginden oluşturulur.
settings-omarchy-none = Etkin bir Omarchy teması yok.
settings-auto-hide = İşaretçi ayrılınca araç çubuğunu gizle
settings-auto-hide-note = Araç çubuğu belgenin üzerinde durur ve işaretçi pencerenin dışındayken kayarak gizlenir.
settings-animations = Animasyonlar
settings-animations-note = Kayan çubuklar ve paneller, büyüyen iletişim kutuları ve yaylanan düğmeler.
settings-animations-reduced = Sistem hareketin azaltılmasını istediğinde kapalıdır.
settings-corner-radius = Köşe yarıçapı
settings-corner-radius-note = İletişim kutuları ve yüzen araç çubuğu için.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Katman saydamlığı
settings-overlay-note = Yüzen araç çubuğunun arkasından sayfanın ne kadarının görüneceği.
settings-overlay-value = %{ $percent }
settings-storage-signatures = İmzalar klasörü
settings-storage-versions = Sürüm geçmişi klasörü
settings-storage-bookmarks = Yer işaretleri dosyası
settings-storage-apply = Uygula
settings-storage-choose = Seç…
# $file is where the settings file is.
settings-storage-note = Eski bir konumda saklanan dosyalar orada kalır; kullanmaya devam etmek için onları yeni konuma taşıyın. prev uygulama ayarları { $file } dosyasına kaydedilir.
settings-save-failed = Ayarlar kaydedilemedi: { $error }
settings-no-location = Ayarlar için konum yok: HOME ayarlanmamış
settings-full-path = ~/Documents/prev gibi tam bir yol kullanın.
settings-path-is-folder = { $path } bir klasör, dosya değil.
settings-folder-missing = { $path } adlı bir klasör yok. Önce oluşturun ya da bir klasör seçin.
settings-path-is-file = { $path } bir dosya, klasör değil.
settings-cannot-write = prev, { $path } konumuna yazamıyor: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Dışa Aktar
# Section headings in the export dialog.
export-format = Biçim
export-quality = Kalite
export-size = Boyut
# Button that goes on to choose where to save the export.
export-choose = Dışa Aktar…
# Format choice; the format name stays as it is.
export-format-webp = WebP (kayıpsız)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = resim
# JPEG quality choices.
export-quality-low = Düşük
export-quality-medium = Orta
export-quality-high = Yüksek
export-quality-best = En iyi
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Gerçek boyut
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } piksel
# $error is the system's reason.
export-dialog-failed = Kaydetme iletişim kutusu gösterilemedi: { $error }
# $path is where the file was saved.
export-done = Dışa aktarıldı: { $path }
export-failed = Dışa aktarılamadı: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = dışa aktarma durdu

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = İşaretleme içeren resimler düzenlenemez. İşaretlemeyi korumak için dışa aktarın ya da işaretlemeyi silip işaretleme çubuğunu kapatın.

# Shown if a background task ends unexpectedly.
image-loading-stopped = yükleme durdu
image-reverting-stopped = geri döndürme durdu
image-rendering-stopped = işleme durdu
image-saving-stopped = kaydetme durdu
image-markup-stopped = işaretleme durdu
image-no-version-store = Sürümlerin saklanacağı bir yer yok
image-revert-failed = Geri döndürülemedi: { $error }
image-read-failed = { $path } okunamadı: { $error }
image-keep-original-failed = Orijinal sürüm saklanamadı: { $error }
image-save-failed = { $path } kaydedilemedi: { $error }
image-markup-start-failed = İşaretleme başlatılamadı: { $error }
image-cannot-edit = Animasyonlar ve SVG çizimleri düzenlenemez.
image-cannot-mark-up = Animasyonlar ve SVG çizimleri işaretlenemez.
image-mark-up-wait = Düzenlemenin bitmesini bekleyin, sonra işaretleyin.
image-crop-needs-selection = Önce bir seçim sürükleyin (Seç aracı), sonra kırpın.
image-size-needed = Piksel cinsinden bir genişlik ve yükseklik girin.
# $name is a file name.
image-cannot-save-format = “{ $name }” dosyasındaki değişiklikler kendi biçiminde kaydedilemez. Dışa Aktar'ı kullanın ({ $keys }).
image-cannot-save-format-unbound = “{ $name }” dosyasındaki değişiklikler kendi biçiminde kaydedilemez. Dışa Aktar'ı kullanın.
image-cannot-export-animation = Animasyonlar henüz dışa aktarılamıyor.
image-drop-pages = Sayfalar bir belgenin üzerine bırakılabilir.
image-drag-failed = Sürükleme başlatılamadı.
image-open-failed = prev bu resmi açamıyor
image-opening = Açılıyor…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = Ad, biçimle eşleşmiyor
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” bir { $format } dosyası olarak kaydedilecek, ancak adı .{ $extension } ile bitiyor. Diğer uygulamalar bu dosyayı açamayabilir.
image-name-mismatch-no-extension = “{ $name }” bir { $format } dosyası olarak kaydedilecek, ancak adının uzantısı yok. Diğer uygulamalar bu dosyayı açamayabilir.
image-choose-again = Yeniden Seç
image-save-as-is = Olduğu Gibi Kaydet
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = kare { $current } / { $total }
image-position = { $current } / { $total }
image-edited = düzenlendi
# Toolbar tooltips.
image-sidebar = Kenar çubuğu
image-zoom-out = Uzaklaştır
image-zoom-in = Yakınlaştır
image-zoom = %{ $percent }
image-fit = Pencereye sığdır
image-actual-size = Gerçek boyut
image-undo = Geri al
image-redo = Yinele
image-rotate-left = Sola döndür
image-rotate-right = Sağa döndür
image-flip-horizontal = Yatay çevir
image-flip-vertical = Dikey çevir
image-select = Dikdörtgen seçim
image-crop = Seçime göre kırp
image-adjust-size-tool = Boyutu ayarla
image-adjust-color-tool = Rengi ayarla
# Tooltip and panel title.
image-inspector = Denetçi
image-markup = İşaretleme
image-export = Dışa aktar
image-settings = Ayarlar
# Panel titles.
image-adjust-color = Rengi Ayarla
image-adjust-size = Boyutu Ayarla
# Adjust Color sliders.
image-exposure = Pozlama
image-contrast = Kontrast
image-saturation = Doygunluk
image-temperature = Sıcaklık
image-tint = Renk tonu
image-sepia = Sepya
image-sharpness = Keskinlik
image-levels = Düzeyler
image-black-point = Siyah noktası
image-midtones = Orta tonlar
image-white-point = Beyaz noktası
image-reset-all = Tümünü Sıfırla
# Adjust Size panel.
image-current-size = Geçerli boyut: { $width } × { $height } piksel
image-width = Genişlik
image-height = Yükseklik
image-scale-proportionally = Orantılı ölçekle
# Button that applies the new size.
image-resize = Yeniden Boyutlandır
# Inspector panel.
image-inspector-loading = Yükleniyor…
image-file = Dosya
image-format = Biçim
image-dimensions-label = Boyutlar
image-pixels = { $width } × { $height } piksel
image-no-camera = Kamera bilgisi yok.
image-location = Konum
image-remove-location = Konum Bilgisini Kaldır
image-no-location = Konum bilgisi yok.
image-keywords-description = Anahtar Sözcükler ve Açıklama
image-keywords-hint = Anahtar sözcükler, virgülle ayrılmış
image-description = Açıklama
image-keywords-unsupported = Anahtar sözcükler JPEG, PNG ve WebP dosyalarına kaydedilebilir.
# Heading over the earlier versions of the file.
image-revert-to = Şu Sürüme Geri Döndür
image-no-versions = Önceki sürüm yok.
image-revert = Geri Döndür
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = İşaretleme dışa aktarılmadan kapatılsın mı?
image-close-body = { $count ->
    [one] Bir resimdeki işaretleme yalnızca penceresi açık kaldığı sürece kalır. Korumak için resmi dışa aktarın: işaretleme, kaydettiğiniz kopyaya çizilir.
   *[other] Resimlerdeki işaretleme yalnızca pencereleri açık kaldığı sürece kalır. Korumak için her resmi dışa aktarın: işaretleme, kaydettiğiniz kopyaya çizilir.
}
image-close-anyway = Yine de Kapat

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = okuma durdu
markdown-read-failed = prev bu dosyayı okuyamıyor
markdown-draw-failed = Belge çizilemedi
# Under the export's size choices.
markdown-export-size = Belgenin tamamı, { $width } × { $height } piksel
# Search results.
markdown-not-found = Bulunamadı
markdown-match = { $current } / { $total }
# Placeholder of the search field.
markdown-search = Ara
# Toolbar tooltips.
markdown-smaller-text = Daha küçük metin
markdown-larger-text = Daha büyük metin
markdown-zoom = %{ $percent }
markdown-actual-size = Gerçek boyut
# Tooltip and panel title.
markdown-inspector = Denetçi
markdown-export = Dışa aktar
markdown-settings = Ayarlar
# Inspector headings and labels.
markdown-file = Dosya
markdown-document = Belge
markdown-words = Sözcükler
markdown-lines = Satırlar
markdown-pictures = Resimler

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Kamera
image-meta-exposure = Pozlama
image-meta-image = Görüntü
image-meta-make = Marka
image-meta-model = Model
image-meta-lens = Objektif
image-meta-exposure-time = Pozlama süresi
# The lens aperture, written like f/2.8.
image-meta-f-number = F değeri
image-meta-iso = ISO
image-meta-focal-length = Odak uzaklığı
image-meta-exposure-bias = Pozlama telafisi
image-meta-flash = Flaş
image-meta-date-taken = Çekim tarihi
image-meta-orientation = Yön
image-meta-color-space = Renk alanı
image-meta-software = Yazılım
image-meta-artist = Sanatçı
image-meta-copyright = Telif hakkı
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } sn
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Yatay olarak yansıtılmış
    [3] 180° döndürülmüş
    [4] Dikey olarak yansıtılmış
    [5] Yatay olarak yansıtılmış, saat yönünün tersine 90° döndürülmüş
    [6] Saat yönünde 90° döndürülmüş
    [7] Yatay olarak yansıtılmış, saat yönünde 90° döndürülmüş
    [8] Saat yönünün tersine 90° döndürülmüş
   *[other] Bilinmiyor ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Patladı
   *[no] Patlamadı
}{ $mode ->
    [on] , zorunlu açık
    [off] , kapalı
    [auto] , otomatik
   *[unknown] {""}
}{ $redeye ->
    [yes] , kırmızı göz azaltma
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Kalibre edilmemiş
   *[other] Diğer ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = belge açılamıyor: { $detail }
error-pdf-page-out-of-range = sayfa { $page } mevcut değil
error-pdf-password-protected = belge parola korumalı; bunun yerine belgeyi açıp sayfalarını kopyalayın
error-pdf-no-pages = çıkarılacak sayfa yok
error-pdf-crop-outside = kırpma alanı sayfanın dışında
error-pdf-closed = belge kapandı
error-pdf-saved-unreadable = kaydedilen belge artık açılmıyor
error-image-read = dosya okunamıyor: { $detail }
error-image-invalid = resim hasarlı veya geçersiz: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = bu biçimi açmak için { $library } gerekiyor, ancak yüklü değil
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format } resimleri henüz desteklenmiyor
error-image-encode = resim kodlanamıyor: { $detail }
error-exif-malformed = EXIF verileri hatalı biçimlendirilmiş
error-settings-read = ayarlar okunamıyor: { $detail }
error-settings-invalid = geçersiz ayarlar: { $detail }
error-remove-location = konum kaldırılamadı: { $error }
error-location-unsupported = konum bilgisi JPEG, PNG, WebP ve TIFF dosyalarından kaldırılabilir
error-xmp-unsupported = anahtar sözcükler ve açıklamalar yalnızca JPEG, PNG ve WebP dosyalarına kaydedilebilir

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = Kamera RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev Hakkında
menu-settings = Ayarlar…
menu-services = Hizmetler
menu-hide = prev'i Gizle
menu-hide-others = Diğerlerini Gizle
menu-show-all = Tümünü Göster
menu-quit = prev'den Çık
menu-file = Dosya
menu-open = Aç…
menu-close = Pencereyi Kapat
menu-export = Dışa Aktar…
menu-print = Yazdır…
menu-edit = Düzen
menu-undo = Geri Al
menu-redo = Yinele
menu-cut = Kes
menu-copy = Kopyala
menu-paste = Yapıştır
menu-select-all = Tümünü Seç
menu-find = Bul
menu-find-next = Sonrakini Bul
menu-find-previous = Öncekini Bul
menu-view = Görüntü
menu-hide-sidebar = Kenar Çubuğunu Gizle
menu-thumbnails = Küçük Resimler
menu-contents = İçindekiler
menu-notes = Vurgular ve Notlar
menu-bookmarks = Yer İşaretleri
menu-zoom-in = Yakınlaştır
menu-zoom-out = Uzaklaştır
menu-actual-size = Gerçek Boyut
menu-zoom-to-fit = Sığacak Şekilde Yakınlaştır
menu-inspector = Denetçiyi Göster
menu-slideshow = Slayt Gösterisi
menu-full-screen = Tam Ekrana Geç
menu-go = Git
menu-next-page = Sonraki Sayfa
menu-previous-page = Önceki Sayfa
menu-go-to-page = Sayfaya Git…
menu-bookmark = Yer İşareti Ekle
menu-tools = Araçlar
menu-markup = İşaretleme Araç Çubuğunu Göster
menu-rotate-left = Sola Döndür
menu-rotate-right = Sağa Döndür
menu-crop = Kırp
menu-adjust-color = Rengi Ayarla…
menu-window = Pencere
menu-minimize = Simge Durumuna Küçült
menu-zoom = Büyüt/Küçült
menu-bring-all-to-front = Tümünü Öne Getir
