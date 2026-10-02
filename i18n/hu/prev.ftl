# prev's interface text in Hungarian (Magyar), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = jelölés, note = jegyzet, highlight = kiemelés,
# annotation = annotáció, redact/redaction = kitakarás, inspector = felügyelő,
# zoom in/out = nagyítás/kicsinyítés, bookmark = könyvjelző, page = oldal.
# redaction mark = kitakarás (not jelölés), export = exportálás, crop = körülvágás.
# Buttons and menu items use the action noun (Mentés, Mégse, Bezárás); the user
# is addressed formally (Ön). Suffixes are kept off `{ $name }` values.

## Language

language-name = Magyar

## Common

common-cancel = Mégse
common-close = Bezárás
common-save = Mentés

## Settings

settings-title = Beállítások
settings-appearance = Megjelenés
settings-colors = Színek
settings-windows = Ablakok
settings-default-app = Alapértelmezett alkalmazás
settings-default-app-label = A fájlokat a prev nyissa meg
settings-default-app-note = A prev legyen az az alkalmazás, amely a PDF-eket, képeket, SVG-rajzokat és Markdown-fájlokat megnyitja.
settings-default-app-note-windows = A Windowsban az alapértelmezett alkalmazások csak a rendszer saját Beállítások alkalmazásában választhatók ki. Ez ott megnyitja a prev oldalát.
settings-default-app-note-macos = A macOS minden típushoz megerősítést kér: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP és AVIF.
settings-default-app-status = { $total } fájltípusból { $set } nyílik meg a prevvel.
settings-default-app-button = Beállítás alapértelmezettként
settings-default-app-button-windows = Beállítások megnyitása
settings-default-app-no-entry = A prev asztali bejegyzése (desktop entry) nincs telepítve, így a rendszer nem tud vele fájlokat megnyitni. Telepítse a prevet csomagból vagy a scripts/install.sh segítségével.
settings-default-app-no-bundle = Ahhoz, hogy a prev alapértelmezett legyen, a prev.app-ból indítsa el.
settings-default-app-failed = Nem sikerült a prevet alapértelmezetté tenni: { $error }
settings-storage = Tárolás
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (fejlesztői változat, { $build })

## Markup toolbar

markup-tool-select = Kijelölés
markup-tool-area = Téglalap alakú kijelölés
markup-tool-sketch = Vázlat
markup-tool-draw = Rajzolás
markup-tool-shapes = Alakzatok
markup-tool-text-box = Szövegdoboz
markup-tool-highlight = Kiemelés
markup-tool-note = Jegyzet
markup-tool-sign = Aláírás
markup-tool-redact = Kitakarás
markup-apply = Alkalmaz
markup-apply-redactions = Kitakarások alkalmazása
markup-shape-style = Alakzatstílus
markup-border-color = Szegély színe
markup-fill-color = Kitöltés színe
markup-text-style = Szövegstílus
markup-delete = Törlés
markup-undo = Visszavonás
markup-redo = Ismétlés

## Markup menus

markup-shape-rectangle = Téglalap
markup-shape-rounded-rectangle = Lekerekített téglalap
markup-shape-oval = Ellipszis
markup-shape-line = Vonal
markup-shape-arrow = Nyíl
markup-shape-star = Csillag
markup-shape-polygon = Sokszög
markup-shape-speech-bubble = Szövegbuborék
markup-shape-loupe = Nagyító
markup-shape-mask = Maszk
markup-style-highlight = Kiemelés
markup-style-underline = Aláhúzás
markup-style-strikethrough = Áthúzás
markup-style-squiggly = Hullámos aláhúzás
markup-menu-color = Szín
markup-menu-font = Betűtípus
markup-menu-size = Méret
markup-menu-alignment = Igazítás
markup-line-width = { $width } pt
markup-dashed = Szaggatott

## Notes

markup-kind-note = Jegyzet
markup-kind-text-box = Szövegdoboz
markup-kind-stamp = Bélyegző
markup-kind-redaction = Kitakarás
markup-kind-shape = Alakzat
markup-note-delete = Jegyzet törlése
markup-note-done = Kész
markup-note-placeholder = Írjon jegyzetet
markup-notes-empty = Nincsenek kiemelések vagy jegyzetek
markup-notes-empty-hint = A kiemelések, jegyzetek és szövegdobozok itt jelennek meg.
markup-notes-page = { $page }. oldal

## Markup errors

markup-change-failed = Nem sikerült módosítani a dokumentumot: { $error }
markup-copy-area-failed = Nem sikerült másolni a területet: { $error }
markup-document-closed = a dokumentum bezárult
markup-render-area-failed = nem sikerült megjeleníteni a területet
markup-copy-stopped = a másolás leállt

## Signatures

signature-menu-empty = Még nincsenek aláírások.
signature-delete = Aláírás törlése
signature-create = Aláírás létrehozása…
signature-dialog-title = Aláírás létrehozása
signature-tab-draw = Rajzolás
signature-tab-type = Gépelés
signature-tab-image = Kép
signature-draw-hint = Írjon alá a vonalon egérrel, tollal vagy érintőpaddal.
signature-your-name = Az Ön neve
signature-image-hint = Válasszon fényképet vagy szkennelt képet a fehér papírra írt aláírásáról.
signature-choose-image = Kép kiválasztása…
signature-description = Leírás, például Teljes név vagy Monogram
signature-clear = Ürítés
signature-ink = Tinta
signature-thickness = Vastagság
signature-sign-first = Előbb írjon alá, aztán mentsen.
signature-default-name = Aláírás { $number }
signature-change-failed = Nem sikerült módosítani az aláírásokat: { $error }
signature-no-data-folder = nincs adatmappa: a HOME nincs beállítva
signature-removing-stopped = az eltávolítás leállt
signature-saving-stopped = a mentés leállt
signature-reading-stopped = az olvasás leállt
signature-not-an-image = a fájl nem olyan kép, amelyet a prev be tud olvasni
signature-no-frames = a képnek nincsenek képkockái
signature-not-found = a képen nem található aláírás

## Dragging

drag-pages-need-document = Oldalakat csak dokumentumra lehet ejteni.
drag-image-unsupported = A prev nem tudja megnyitni ezt a képet.
drag-area-failed = Nem sikerült áthúzni a területet: { $error }
drag-pages-failed = Nem sikerült áthúzni az oldalakat: { $error }
drag-start-failed = Nem sikerült elkezdeni a húzást.
drag-file-pages = Oldalak
drag-file-one-page = { $name } ({ $page }. oldal)
drag-file-page-range = { $name } ({ $first }–{ $last }. oldal)
drag-file-image = Kép
drop-pdf-title = Hozzáadja ehhez a dokumentumhoz?
drop-pdf-body = „{ $name }”: hozzáfűzi ennek a dokumentumnak a végéhez, vagy saját ablakban nyitja meg?
drop-pdfs-body = { $count ->
    [one] { $count } PDF-fájl: hozzáfűzi ennek a dokumentumnak a végéhez, vagy saját ablakban nyitja meg?
   *[other] { $count } PDF-fájl: hozzáfűzi őket ennek a dokumentumnak a végéhez, vagy mindegyiket saját ablakban nyitja meg?
}
drop-pdf-add = Hozzáfűzés a végéhez
drop-pdf-open = Megnyitás külön

## PDF window

pdf-opening = Megnyitás…
pdf-open-failed = A prev nem tudja megnyitni ezt a dokumentumot
pdf-no-pages = A dokumentumnak nincsenek oldalai.
pdf-document-closed = a dokumentum bezárult
pdf-keep-original-failed = nem sikerült megtartani az eredeti változatot: { $error }
pdf-save-failed = Nem sikerült menteni: { $error }
pdf-nothing-to-paste = Nincs mit beilleszteni.
pdf-pasting-stopped = a beillesztés leállt
pdf-file-dialog-failed = Nem sikerült megjeleníteni a fájlválasztó ablakot: { $error }
pdf-bookmarks-no-home = A könyvjelzők nem menthetők: a HOME nincs beállítva
pdf-bookmarks-save-failed = Nem sikerült menteni a könyvjelzőket: { $error }
pdf-bookmark-page = { $page }. oldal

pdf-password-protected = Ez a fájl jelszóval védett: „{ $name }”
pdf-password = Jelszó
pdf-password-wrong = Helytelen jelszó. Próbálja újra.
pdf-unlock = Feloldás

pdf-sidebar = Oldalsáv
pdf-page-of = / { $count }
pdf-zoom-out = Kicsinyítés
pdf-zoom-in = Nagyítás
pdf-zoom-percent = { $percent }%
pdf-fit-page = Oldalhoz igazítás
pdf-fit-width = Szélességhez igazítás
pdf-actual-size = Tényleges méret
pdf-view-continuous = Folyamatos görgetés
pdf-view-single-page = Egy oldal
pdf-view-two-pages = Két oldal
pdf-undo = Visszavonás
pdf-redo = Ismétlés
pdf-rotate-left = Forgatás balra
pdf-rotate-right = Forgatás jobbra
pdf-inspector = Felügyelő
pdf-markup = Jelölés
pdf-export = Exportálás
pdf-settings = Beállítások

pdf-search = Keresés
pdf-search-not-found = Nincs találat
pdf-searching = Keresés…
pdf-search-match = { $current } / { $total }
pdf-search-match-more = { $current } / { $total }+

pdf-inspector-file = Fájl
pdf-inspector-document = Dokumentum
pdf-inspector-pages = Oldalak
pdf-inspector-title = Cím
pdf-inspector-author = Szerző
pdf-inspector-subject = Tárgy
pdf-inspector-keywords = Kulcsszavak
pdf-inspector-created = Létrehozva
pdf-inspector-modified = Módosítva
pdf-inspector-application = Alkalmazás
pdf-inspector-producer = PDF-előállító
pdf-inspector-version = Verzió
pdf-inspector-security = Biztonság
pdf-inspector-not-encrypted = Nincs titkosítva
pdf-inspector-encrypted = Titkosítva ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } oldal
   *[other] { $count } oldal
}
pdf-inspector-page-size = Oldalméret
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } hüvelyk)
pdf-loading = Betöltés…

pdf-tab-pages = Oldalak
pdf-tab-contents = Tartalom
pdf-tab-notes = Kiemelések és jegyzetek
pdf-tab-bookmarks = Könyvjelzők
pdf-no-outline = Nincs tartalomjegyzék
pdf-no-outline-detail = Ez a dokumentum nem tartalmaz tartalomjegyzéket.
pdf-no-bookmarks = Nincsenek könyvjelzők
pdf-no-bookmarks-detail = Könyvjelző hozzáadása egy oldalhoz: { $keys }
pdf-no-bookmarks-detail-unbound = A könyvjelzővel ellátott oldalak itt jelennek meg.
pdf-remove-bookmark = Könyvjelző eltávolítása

## Page editing

pages-menu = Oldalak
pages-insert-blank = Üres oldal beszúrása
pages-insert-file = Beszúrás fájlból…
pages-copy = { $count ->
    [one] Oldal másolása
   *[other] Oldalak másolása
}
pages-paste = { $count ->
    [one] Oldal beillesztése
   *[other] { $count } oldal beillesztése
}
pages-crop = Körülvágás a kijelölésre
pages-select-all = Összes oldal kijelölése
pages-delete = { $count ->
    [one] Oldal törlése
   *[other] Oldalak törlése
}
pages-apply-redactions = Kitakarások alkalmazása…
pages-no-copied = Nincsenek beilleszthető másolt oldalak.
pages-copied = { $count ->
    [one] { $count } oldal másolva.
   *[other] { $count } oldal másolva.
}
pages-copy-failed = Nem sikerült másolni az oldalakat: { $error }
pages-reading-stopped = az olvasás leállt
pages-image-unreadable = nem olyan kép, amelyet a prev be tud olvasni
pages-read-failed = Nem sikerült beolvasni a fájlt: { $error }
pages-at-least-one = A dokumentumnak legalább egy oldalt tartalmaznia kell.
pages-crop-needs-area = Előbb jelöljön ki egy területet a téglalap alakú kijelölés eszközzel.
pages-change-failed = Nem sikerült módosítani az oldalakat: { $error }
pages-no-redactions = Nem volt alkalmazandó kitakarás.
pages-redactions-applied = { $count ->
    [one] { $count } kitakarás alkalmazva.
   *[other] { $count } kitakarás alkalmazva.
}
pages-forget-versions-failed = Nem sikerült törölni a korábbi változatokat: { $error }
pages-redact-title = Alkalmazza a kitakarásokat?
pages-redact-body = { $count ->
    [one] A kitakarás alatti szöveg, képek és rajzok véglegesen törlődnek a dokumentumból, a kitakarás pedig fekete téglalappá válik. Ez nem vonható vissza, és a fájl prev által megőrzött korábbi változatai is törlődnek.
   *[other] A kitakarások alatti szöveg, képek és rajzok ({ $count } helyen) véglegesen törlődnek a dokumentumból, a kitakarások pedig fekete téglalapokká válnak. Ez nem vonható vissza, és a fájl prev által megőrzött korábbi változatai is törlődnek.
}
pages-redact-apply = Alkalmaz

## PDF export

pages-export-title = Exportálás
pages-export-format = Formátum
pages-export-reduce = Fájlméret csökkentése (képek 150 dpi felbontással)
pages-export-flatten = Annotációk és űrlapmezők véglegesítése
pages-export-flatten-detail = A jelölések és a kitöltött mezők az oldalak részévé válnak, és többé nem szerkeszthetők. A még nem alkalmazott kitakarások kimaradnak.
pages-export-encrypt = Titkosítás jelszóval
pages-export-password = Jelszó
pages-export-verify-password = Jelszó megerősítése
pages-export-resolution = Felbontás
pages-export-dpi = { $dpi } dpi
pages-export-quality = Minőség
pages-export-quality-low = Alacsony
pages-export-quality-medium = Közepes
pages-export-quality-high = Magas
pages-export-quality-best = Legjobb
pages-export-one-file = Minden oldal egy fájlba kerül.
pages-export-file-per-page = Minden oldal külön fájlba kerül, a választott név után számozva.
pages-export-selected-only = { $count ->
    [one] Csak a kijelölt oldal
   *[other] Csak a kijelölt { $count } oldal
}
pages-export-choose = Exportálás…
pages-export-no-password = Adjon meg egy jelszót.
pages-export-password-mismatch = A jelszavak nem egyeznek.
pages-export-file-name = { $name } (exportált)
pages-export-untitled = dokumentum
pages-export-same-file = Exportáljon új fájlba; ez a dokumentum magától menti magát.
pages-export-exporting = Exportálás: „{ $name }”…
pages-export-done = Exportálva: „{ $name }”.
pages-export-done-images = { $count } kép exportálva.
pages-export-failed = Nem sikerült exportálni: { $error }
pages-export-stopped = az exportálás leállt

## Start window

app-start-hint = Nyisson meg vagy húzzon ide egy PDF-, kép-, SVG- vagy Markdown-fájlt.
app-start-open = Megnyitás…
app-title-dev = { $title } (fejlesztői)
app-viewer-missing = { $kind }: ez a megjelenítő még nem készült el.
app-cannot-open = A prev nem tudja megnyitni ezt a fájltípust.
app-cannot-read = A prev nem tudja beolvasni ezt a fájlt: { $error }
app-kind-pdf = PDF-dokumentum
app-kind-image = { $format }-kép
app-kind-svg = SVG-rajz
app-kind-markdown = Markdown-dokumentum
app-file-dialog-failed = Nem sikerült megjeleníteni a fájlválasztó ablakot: { $error }

## Actions

action-open = Megnyitás
action-settings = Beállítások

## Toolbar

app-toolbar-keep-shown = Az eszköztár mindig látható
app-toolbar-auto-hide = Az eszköztár elrejtése, amikor a mutató elhagyja az ablakot
app-toolbar-more = Továbbiak

## File facts

app-fact-name = Név
app-fact-folder = Mappa
app-fact-size = Méret
app-fact-modified = Módosítva
app-size-bytes = { $count } bájt
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Érvénytelen hivatkozás ({ $uri }): { $error }
app-link-open-failed = Nem sikerült megnyitni a hivatkozást ({ $uri }): { $error }
app-paste-needs-wl-clipboard = képek beillesztéséhez telepítse a wl-clipboard csomagot
app-copy-needs-wl-clipboard = képek másolásához telepítse a wl-clipboard csomagot
app-copy-no-pixels = a területen nincsenek képpontok
app-copy-no-input = a wl-copy nem kapott bemenetet
app-copy-failed = a wl-copy hibát jelzett
app-clipboard-open-failed = Nem sikerült megnyitni a vágólapot: { $error }
app-copy-image-failed = Nem sikerült másolni a képet: { $error }

## Printing

print-failed = Nem sikerült nyomtatni: { $error }
print-stopped = A nyomtatás leállt
print-unavailable = A nyomtatás ezen a rendszeren még nem érhető el.
print-no-window = Nem sikerült nyomtatni: nincs ablak, amely fölött a nyomtatási párbeszédpanel megjelenhetne
print-dialog-failed = Nem sikerült megjeleníteni a nyomtatási párbeszédpanelt: { $error }
print-job-not-started = a nyomtató nem indította el a feladatot
print-printer-stopped = a nyomtató leállt

## File dialogs

dialog-open = Megnyitás
dialog-filter-all = Minden támogatott fájl
dialog-filter-pdf = PDF-dokumentumok
dialog-filter-images = Képek
dialog-filter-svg = SVG-rajzok
dialog-filter-markdown = Markdown-fájlok
dialog-choose-signatures = Válassza ki az aláírások mappáját
dialog-choose-versions = Válassza ki a verzióelőzmények mappáját
dialog-choose-bookmarks = Válassza ki a könyvjelzőfájlt

## Command line

usage-help =
    Használat: prev [FILE]...
               prev --mcp

    PDF-ek és képek megtekintése és szerkesztése. A fájlok a futó prev
    ablakaiban nyílnak meg; a prev szükség esetén elindul.

    Kapcsolók:
      -h, --help     A súgó megjelenítése
      -V, --version  A verzió megjelenítése
          --mcp      MCP kiszolgálása stdin és stdout útján, hogy MI-ügynökök
                     vezérelhessék a futó prevet

## Settings, continued

settings-language = Nyelv
settings-language-system = Rendszer alapértelmezése: { $language }
settings-input-language = Beviteli nyelv
settings-input-language-system = A billentyűzetkiosztás szerint
settings-input-language-note = Meghatározza, melyik oldalon kezdődik egy üres szövegmező. A begépelt szöveg megtartja a saját irányát.

settings-appearance-system = Rendszer
settings-appearance-light = Világos
settings-appearance-dark = Sötét
settings-system-accent = A rendszer kiemelőszínének használata
settings-omarchy-note = A színek az Omarchy-téma („{ $theme }”) kiemelőszínéből készülnek.
settings-system-accent-note = A színek a rendszer kiemelőszínéből készülnek.
settings-system-accent-none = A rendszernek nincs kiemelőszíne, ezért a prev az alább kiválasztott színt használja.
settings-accent-chosen-note = A színek az alább kiválasztott színből készülnek.
settings-auto-hide = Az eszköztár elrejtése, amikor a mutató elhagyja az ablakot
settings-auto-hide-note = Az eszköztár a dokumentum fölött lebeg, és eltűnik, amíg a mutató az ablakon kívül van.
settings-animations = Animációk
settings-animations-note = Becsúszó sávok és panelek, kinyíló párbeszédpanelek és rugalmas gombok.
settings-animations-reduced = Kikapcsolva, amíg a rendszer csökkentett mozgást kér.
settings-corner-radius = Sarkok lekerekítése
settings-corner-radius-note = A párbeszédpanelekhez és a lebegő eszköztárhoz.
settings-corner-radius-value = { $radius } px
settings-overlay = Átfedés átlátszósága
settings-overlay-note = Mennyire látszik át az oldal a lebegő eszköztáron.
settings-overlay-value = { $percent }%
settings-storage-signatures = Aláírások mappája
settings-storage-versions = Verzióelőzmények mappája
settings-storage-bookmarks = Könyvjelzőfájl
settings-storage-apply = Alkalmaz
settings-storage-choose = Tallózás…
settings-storage-note = A régi helyen tárolt fájlok ott maradnak; helyezze át őket, ha továbbra is használni szeretné őket. A prev alkalmazás beállításainak helye: { $file }.
settings-save-failed = Nem sikerült menteni a beállításokat: { $error }
settings-no-location = Nincs hely a beállítások számára: a HOME nincs beállítva
settings-full-path = Teljes elérési utat adjon meg, például ~/Documents/prev.
settings-path-is-folder = Ez mappa, nem fájl: { $path }
settings-folder-missing = Nincs ilyen mappa: { $path }. Előbb hozza létre, vagy válasszon egy másikat.
settings-path-is-file = Ez fájl, nem mappa: { $path }
settings-cannot-write = A prev nem tud írni ide: { $path } – { $error }.

## Export dialog

export-title = Exportálás
export-format = Formátum
export-quality = Minőség
export-size = Méret
export-choose = Exportálás…
export-format-webp = WebP (veszteségmentes)
export-format-unknown = kép
export-quality-low = Alacsony
export-quality-medium = Közepes
export-quality-high = Magas
export-quality-best = Legjobb
export-size-actual = Tényleges méret
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } képpont
export-dialog-failed = Nem sikerült megjeleníteni a mentési párbeszédpanelt: { $error }
export-done = Exportálva: { $path }
export-failed = Nem sikerült exportálni: { $error }
export-stopped = az exportálás leállt

## Image window

image-marked-no-edit = A jelöléseket tartalmazó képek nem szerkeszthetők. A jelölések megtartásához exportálja a képet, vagy törölje a jelöléseket, és zárja be a Jelölés eszközsort.

image-loading-stopped = a betöltés leállt
image-reverting-stopped = a visszaállítás leállt
image-rendering-stopped = a megjelenítés leállt
image-saving-stopped = a mentés leállt
image-markup-stopped = a jelölés leállt
image-no-version-store = Nincs hely a változatok tárolására
image-revert-failed = Nem sikerült visszaállítani: { $error }
image-read-failed = Nem sikerült beolvasni a fájlt ({ $path }): { $error }
image-keep-original-failed = Nem sikerült megtartani az eredeti változatot: { $error }
image-save-failed = Nem sikerült menteni a fájlt ({ $path }): { $error }
image-markup-start-failed = Nem sikerült elindítani a jelölést: { $error }
image-cannot-edit = Animációk és SVG-rajzok nem szerkeszthetők.
image-cannot-mark-up = Animációkra és SVG-rajzokra nem lehet jelölést tenni.
image-mark-up-wait = Várja meg, amíg a szerkesztés befejeződik, majd jelöljön.
image-crop-needs-selection = Előbb húzzon ki egy kijelölést (Kijelölés eszköz), aztán vágja körül.
image-size-needed = Adja meg a szélességet és a magasságot képpontban.
image-cannot-save-format = „{ $name }”: a módosítások nem menthetők a fájl formátumában. Használja az Exportálást ({ $keys }).
image-cannot-save-format-unbound = „{ $name }”: a módosítások nem menthetők a fájl formátumában. Használja az Exportálást.
image-cannot-export-animation = Animációk még nem exportálhatók.
image-drop-pages = Oldalakat csak dokumentumra lehet ejteni.
image-drag-failed = Nem sikerült elkezdeni a húzást.
image-picture-save-failed = Nem sikerült menteni a képet a Letöltések mappába.
image-open-failed = A prev nem tudja megnyitni ezt a képet
image-opening = Megnyitás…
image-name-mismatch-title = A név nem egyezik a formátummal
image-name-mismatch = A fájl („{ $name }”) { $format } formátumban lesz mentve, de a neve .{ $extension } végződésű. Előfordulhat, hogy más alkalmazások nem tudják megnyitni.
image-name-mismatch-no-extension = A fájl („{ $name }”) { $format } formátumban lesz mentve, de a nevének nincs kiterjesztése. Előfordulhat, hogy más alkalmazások nem tudják megnyitni.
image-choose-again = Újraválasztás
image-save-as-is = Mentés így
image-dimensions = { $width } × { $height }
image-frame-position = képkocka: { $current } / { $total }
image-position = { $current } / { $total }
image-edited = szerkesztve
image-sidebar = Oldalsáv
image-zoom-out = Kicsinyítés
image-zoom-in = Nagyítás
image-zoom = { $percent }%
image-fit = Ablakhoz igazítás
image-actual-size = Tényleges méret
image-undo = Visszavonás
image-redo = Ismétlés
image-rotate-left = Forgatás balra
image-rotate-right = Forgatás jobbra
image-flip-horizontal = Vízszintes tükrözés
image-flip-vertical = Függőleges tükrözés
image-select = Téglalap alakú kijelölés
image-crop = Körülvágás a kijelölésre
image-adjust-size-tool = Méret módosítása
image-adjust-color-tool = Színek módosítása
image-inspector = Felügyelő
image-markup = Jelölés
image-export = Exportálás
image-settings = Beállítások
image-adjust-color = Színek módosítása
image-adjust-size = Méret módosítása
image-exposure = Expozíció
image-contrast = Kontraszt
image-saturation = Telítettség
image-temperature = Színhőmérséklet
image-tint = Színárnyalat
image-sepia = Szépia
image-sharpness = Élesség
image-levels = Szintek
image-black-point = Fekete pont
image-midtones = Középtónusok
image-white-point = Fehér pont
image-reset-all = Összes visszaállítása
image-current-size = Jelenlegi méret: { $width } × { $height } képpont
image-width = Szélesség
image-height = Magasság
image-scale-proportionally = Arányos méretezés
image-resize = Átméretezés
image-inspector-loading = Betöltés…
image-file = Fájl
image-format = Formátum
image-dimensions-label = Méretek
image-pixels = { $width } × { $height } képpont
image-no-camera = Nincsenek fényképezőgép-adatok.
image-location = Hely
image-remove-location = Helyadatok eltávolítása
image-no-location = Nincsenek helyadatok.
image-keywords-description = Kulcsszavak és leírás
image-keywords-hint = Kulcsszavak, vesszővel elválasztva
image-description = Leírás
image-keywords-unsupported = Kulcsszavak JPEG-, PNG- és WebP-fájlokba menthetők.
image-revert-to = Visszaállítás korábbi változatra
image-no-versions = Nincsenek korábbi változatok.
image-revert = Visszaállítás
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Bezárja a jelölések exportálása nélkül?
image-close-body = { $count ->
    [one] A képen lévő jelölések csak addig maradnak meg, amíg az ablaka nyitva van. Megtartásukhoz exportálja a képet: a jelölések a mentett másolatba kerülnek.
   *[other] A képeken lévő jelölések csak addig maradnak meg, amíg az ablakuk nyitva van. Megtartásukhoz exportálja az egyes képeket: a jelölések a mentett másolatba kerülnek.
}
image-close-anyway = Bezárás mindenképp

## Markdown

markdown-reading-stopped = az olvasás leállt
markdown-read-failed = A prev nem tudja beolvasni ezt a fájlt
markdown-draw-failed = Nem sikerült megjeleníteni a dokumentumot
markdown-export-size = A teljes dokumentum, { $width } × { $height } képpont
markdown-not-found = Nincs találat
markdown-match = { $current } / { $total }
markdown-search = Keresés
markdown-smaller-text = Kisebb szöveg
markdown-larger-text = Nagyobb szöveg
markdown-zoom = { $percent }%
markdown-actual-size = Tényleges méret
markdown-inspector = Felügyelő
markdown-export = Exportálás
markdown-settings = Beállítások
markdown-file = Fájl
markdown-document = Dokumentum
markdown-words = Szavak
markdown-lines = Sorok
markdown-pictures = Képek

## Image details

image-meta-camera = Fényképezőgép
image-meta-exposure = Expozíció
image-meta-image = Kép
image-meta-make = Gyártó
image-meta-model = Modell
image-meta-lens = Objektív
image-meta-exposure-time = Expozíciós idő
image-meta-f-number = Rekeszérték
image-meta-iso = ISO
image-meta-focal-length = Gyújtótávolság
image-meta-exposure-bias = Expozíciókorrekció
image-meta-flash = Vaku
image-meta-date-taken = Készítés dátuma
image-meta-orientation = Tájolás
image-meta-color-space = Színtér
image-meta-software = Szoftver
image-meta-artist = Alkotó
image-meta-copyright = Szerzői jog
image-meta-seconds = { $value } mp
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normál
    [2] Vízszintesen tükrözve
    [3] 180°-kal elforgatva
    [4] Függőlegesen tükrözve
    [5] Vízszintesen tükrözve, 90°-kal elforgatva az óramutató járásával ellentétesen
    [6] 90°-kal elforgatva az óramutató járásával megegyezően
    [7] Vízszintesen tükrözve, 90°-kal elforgatva az óramutató járásával megegyezően
    [8] 90°-kal elforgatva az óramutató járásával ellentétesen
   *[other] Ismeretlen ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Villant
   *[no] Nem villant
}{ $mode ->
    [on] , kényszerített mód
    [off] , kikapcsolt mód
    [auto] , automatikus mód
   *[unknown] {""}
}{ $redeye ->
    [yes] , vörösszemhatás-csökkentés
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Kalibrálatlan
   *[other] Egyéb ({ $code })
}

## Errors

error-pdf-open = a dokumentum nem nyitható meg: { $detail }
error-pdf-page-out-of-range = nincs { $page }. oldal
error-pdf-password-protected = a dokumentum jelszóval védett; nyissa meg, és inkább az oldalait másolja
error-pdf-no-pages = nincsenek kinyerhető oldalak
error-pdf-crop-outside = a körülvágási terület az oldalon kívül esik
error-pdf-closed = a dokumentum bezárult
error-pdf-saved-unreadable = a mentett dokumentum már nem nyitható meg
error-image-read = a fájl nem olvasható: { $detail }
error-image-invalid = a kép sérült vagy érvénytelen: { $detail }
error-image-missing-library = a formátum megnyitásához szükséges könyvtár nincs telepítve: { $library }
error-image-unsupported = ez a képformátum még nem támogatott: { $format }
error-image-encode = a kép nem kódolható: { $detail }
error-exif-malformed = az EXIF-adatok hibásak
error-settings-read = a beállítások nem olvashatók: { $detail }
error-settings-invalid = érvénytelen beállítások: { $detail }
error-remove-location = nem sikerült eltávolítani a helyadatokat: { $error }
error-location-unsupported = helyadatok JPEG-, PNG-, WebP- és TIFF-fájlokból távolíthatók el
error-xmp-unsupported = kulcsszavak és leírások csak JPEG-, PNG- és WebP-fájlokba menthetők

## Formats

format-camera-raw = Nyers RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = A prev névjegye
menu-settings = Beállítások…
menu-services = Szolgáltatások
menu-hide = prev elrejtése
menu-hide-others = Többi elrejtése
menu-show-all = Az összes megjelenítése
menu-quit = Kilépés a prevből
menu-file = Fájl
menu-open = Megnyitás…
menu-close = Ablak bezárása
menu-export = Exportálás…
menu-print = Nyomtatás…
menu-edit = Szerkesztés
menu-undo = Visszavonás
menu-redo = Ismétlés
menu-cut = Kivágás
menu-copy = Másolás
menu-paste = Beillesztés
menu-select-all = Az összes kijelölése
menu-find = Keresés
menu-find-next = Következő keresése
menu-find-previous = Előző keresése
menu-view = Nézet
menu-hide-sidebar = Oldalsáv elrejtése
menu-thumbnails = Miniatűrök
menu-contents = Tartalomjegyzék
menu-notes = Kiemelések és jegyzetek
menu-bookmarks = Könyvjelzők
menu-zoom-in = Nagyítás
menu-zoom-out = Kicsinyítés
menu-actual-size = Tényleges méret
menu-zoom-to-fit = Méretezés az ablakhoz
menu-inspector = Felügyelő megjelenítése
menu-slideshow = Diavetítés
menu-full-screen = Teljes képernyő
menu-go = Ugrás
menu-next-page = Következő oldal
menu-previous-page = Előző oldal
menu-go-to-page = Ugrás oldalra…
menu-bookmark = Könyvjelző hozzáadása
menu-tools = Eszközök
menu-markup = Jelölés eszközsor megjelenítése
menu-rotate-left = Forgatás balra
menu-rotate-right = Forgatás jobbra
menu-crop = Körülvágás
menu-adjust-color = Színek módosítása…
menu-window = Ablak
menu-minimize = Kis méret
menu-zoom = Nagyítás/kicsinyítés
menu-bring-all-to-front = Az összes előtérbe hozása

## Outside control

settings-outside-control = Külső vezérlés
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = Általános
settings-tab-agents = Ügynökök
settings-allow-outside-control = Külső vezérlés engedélyezése
settings-allow-outside-control-note = Az MI-ügynökök, például a Claude Code, a prev --mcp segítségével olvashatják és módosíthatják a prevben lévő fájljait. A prev minden új ügynök előtt rákérdez.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Engedélyezve: { $agents }
settings-forget-agents = Elfelejtés
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Engedélyezi, hogy { $agent } vezérelje a prevet?
agent-prompt-body = { $agent } szeretné használni a prev külső vezérlését, hogy olvassa és módosítsa a megnyitott fájljait. A külső vezérlést a Beállításokban kapcsolhatja ki.
agent-prompt-allow = Engedélyezés
agent-prompt-deny = Tiltás
settings-ask-before-note = Rákérdezés az ügynök alábbi műveletei előtt:
settings-ask-reading = Fájl olvasása
settings-ask-viewing = A nézet vagy egy ablak módosítása
settings-ask-marking-up = Fájl jelölése
settings-ask-editing = Fájl szerkesztése
settings-ask-signing = Fájl aláírása
settings-ask-redacting = Kitakarások alkalmazása
settings-ask-exporting = Fájl exportálása
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Engedélyezi, hogy { $agent } beolvassa ezt a fájlt?
agent-ask-view = Engedélyezi, hogy { $agent } módosítsa a nézetet?
agent-ask-markup = Engedélyezi, hogy { $agent } jelöléseket tegyen erre a fájlra?
agent-ask-edit = Engedélyezi, hogy { $agent } szerkessze ezt a fájlt?
agent-ask-sign = Engedélyezi, hogy { $agent } aláírja ezt a fájlt?
agent-ask-redact = Engedélyezi, hogy { $agent } kitakarásokat alkalmazzon?
agent-ask-export = Engedélyezi, hogy { $agent } exportálja ezt a fájlt?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } szeretné használni ezt: „{ $tool }”. A Beállításokban választhatja ki, mire kérdez rá a prev.
agent-ask-final = Ez nem vonható vissza.
