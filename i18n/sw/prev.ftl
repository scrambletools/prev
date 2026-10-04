# prev's interface text in Swahili (Kiswahili), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = alama, note = dokezo (plural madokezo),
# highlight = kiangazio (verb angazia), annotation = ufafanuzi,
# redact/redaction = ficha/ufichaji, inspector = kikaguzi,
# zoom in/out = vuta karibu/sogeza mbali, bookmark = alamisho,
# page = ukurasa (plural kurasa), export = hamisha, revert = rejesha,
# undo/redo = tendua/rudia. Buttons and menu items use the imperative
# (Hifadhi, Ghairi); labels use nouns. Units follow the number's noun
# order of Swahili (kurasa 3, pikseli 12).

## Language

language-name = Kiswahili

## Common

common-cancel = Ghairi
common-close = Funga
common-save = Hifadhi

## Settings

settings-title = Mipangilio
settings-appearance = Mwonekano
settings-colors = Rangi
settings-windows = Madirisha
settings-default-app = Programu chaguomsingi
settings-default-app-label = Fungua faili kwa prev
settings-default-app-note = Fanya prev iwe programu inayofungua PDF, picha, michoro ya SVG na faili za Markdown.
settings-default-app-note-windows = Windows hukuruhusu kuchagua programu chaguomsingi katika Mipangilio yake tu. Kitufe hiki kinafungua ukurasa wa prev huko.
settings-default-app-note-macos = macOS hukuomba uthibitishe kila aina: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP na AVIF.
settings-default-app-status = Aina za faili { $set } kati ya { $total } hufunguka kwa prev.
settings-default-app-button = Fanya Chaguomsingi
settings-default-app-button-windows = Fungua Mipangilio
settings-default-app-no-entry = Kiingizo cha eneo-kazi cha prev hakijasakinishwa, kwa hivyo mfumo hauwezi kufungua faili kwacho. Sakinisha prev kutoka kwa kifurushi au kwa scripts/install.sh.
settings-default-app-no-bundle = Fungua prev kutoka prev.app ili uifanye kuwa chaguomsingi.
settings-default-app-failed = Imeshindwa kufanya prev kuwa chaguomsingi: { $error }
settings-storage = Hifadhi ya faili
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (toleo la usanidi, { $build })

## Markup toolbar

markup-tool-select = Chagua
markup-tool-area = Uteuzi wa mstatili
markup-tool-sketch = Mchoro huru
markup-tool-draw = Chora
markup-tool-shapes = Maumbo
markup-tool-text-box = Kisanduku cha maandishi
markup-tool-highlight = Angazia
markup-tool-note = Dokezo
markup-tool-sign = Weka sahihi
markup-tool-redact = Ficha
markup-apply = Tekeleza
markup-apply-redactions = Tekeleza ufichaji
markup-shape-style = Mtindo wa umbo
markup-border-color = Rangi ya mpaka
markup-fill-color = Rangi ya kujaza
markup-text-style = Mtindo wa maandishi
markup-delete = Futa
markup-undo = Tendua
markup-redo = Rudia

## Markup menus

markup-shape-rectangle = Mstatili
markup-shape-rounded-rectangle = Mstatili wenye Pembe za Mviringo
markup-shape-oval = Duaradufu
markup-shape-line = Mstari
markup-shape-arrow = Mshale
markup-shape-star = Nyota
markup-shape-polygon = Poligoni
markup-shape-speech-bubble = Kiputo cha Maneno
markup-shape-loupe = Lenzi ya kukuza
markup-shape-mask = Kifuniko
markup-style-highlight = Kiangazio
markup-style-underline = Mstari chini
markup-style-strikethrough = Mstari katikati
markup-style-squiggly = Mstari wa mawimbi
markup-menu-color = Rangi
markup-menu-font = Fonti
markup-menu-size = Ukubwa
markup-menu-alignment = Mpangilio
markup-line-width = pointi { $width }
markup-dashed = Vistari

## Notes

markup-kind-note = Dokezo
markup-kind-text-box = Kisanduku cha maandishi
markup-kind-stamp = Muhuri
markup-kind-redaction = Ufichaji
markup-kind-shape = Umbo
markup-note-delete = Futa dokezo
markup-note-done = Nimemaliza
markup-note-placeholder = Andika dokezo
markup-notes-empty = Hakuna viangazio wala madokezo
markup-notes-empty-hint = Viangazio, madokezo na visanduku vya maandishi huonekana hapa.
markup-notes-page = Ukurasa { $page }

## Markup errors

markup-change-failed = Imeshindwa kubadilisha hati: { $error }
markup-copy-area-failed = Imeshindwa kunakili eneo: { $error }
markup-document-closed = hati imefungwa
markup-render-area-failed = imeshindwa kuonyesha eneo
markup-copy-stopped = kunakili kumesimama

## Signatures

signature-menu-empty = Bado hakuna sahihi.
signature-delete = Futa sahihi
signature-create = Unda Sahihi…
signature-dialog-title = Unda Sahihi
signature-tab-draw = Chora
signature-tab-type = Andika
signature-tab-image = Picha
signature-draw-hint = Weka sahihi kwenye mstari kwa kipanya, kalamu au padi ya kugusa.
signature-your-name = Jina lako
signature-image-hint = Chagua picha au skani ya sahihi yako kwenye karatasi nyeupe.
signature-choose-image = Chagua Picha…
signature-description = Maelezo, kama Jina kamili au Herufi za mwanzo
signature-clear = Futa yote
signature-ink = Wino
signature-thickness = Unene
signature-sign-first = Weka sahihi kwanza, kisha uhifadhi.
signature-default-name = Sahihi { $number }
signature-change-failed = Imeshindwa kubadilisha sahihi: { $error }
signature-no-data-folder = hakuna folda ya data: HOME haijawekwa
signature-removing-stopped = kuondoa kumesimama
signature-saving-stopped = kuhifadhi kumesimama
signature-reading-stopped = kusoma kumesimama
signature-not-an-image = faili hiyo si picha ambayo prev inaweza kusoma
signature-no-frames = picha haina fremu
signature-not-found = hakuna sahihi iliyopatikana kwenye picha

## Dragging

drag-pages-need-document = Kurasa zinaweza kudondoshwa kwenye hati.
drag-image-unsupported = prev haiwezi kufungua picha hii.
drag-area-failed = Imeshindwa kuburuta eneo: { $error }
drag-pages-failed = Imeshindwa kuburuta kurasa: { $error }
drag-start-failed = Imeshindwa kuanza kuburuta.
drag-file-pages = Kurasa
drag-file-one-page = { $name } (ukurasa { $page })
drag-file-page-range = { $name } (kurasa { $first }–{ $last })
drag-file-image = Picha
drop-pdf-title = Ongeza kwenye hati hii?
drop-pdf-body = Ongeza “{ $name }” mwishoni mwa hati hii, au uifungue katika dirisha lake?
drop-pdfs-body = { $count ->
    [one] Ongeza PDF hii mwishoni mwa hati hii, au uifungue katika dirisha lake?
   *[other] Ongeza PDF hizi { $count } mwishoni mwa hati hii, au ufungue kila moja katika dirisha lake?
}
drop-pdf-add = Ongeza Mwishoni
drop-pdf-open = Fungua Kando

## PDF window

pdf-opening = Inafungua…
pdf-open-failed = prev haiwezi kufungua hati hii
pdf-no-pages = Hati haina kurasa.
pdf-document-closed = hati imefungwa
pdf-keep-original-failed = imeshindwa kuhifadhi toleo asili: { $error }
pdf-save-failed = Imeshindwa kuhifadhi: { $error }
pdf-nothing-to-paste = Hakuna kitu cha kubandika.
pdf-pasting-stopped = kubandika kumesimama
pdf-file-dialog-failed = Imeshindwa kuonyesha kidirisha cha faili: { $error }
pdf-bookmarks-no-home = Alamisho haziwezi kuhifadhiwa: HOME haijawekwa
pdf-bookmarks-save-failed = Imeshindwa kuhifadhi alamisho: { $error }
pdf-bookmark-page = Ukurasa { $page }

pdf-password-protected = “{ $name }” imelindwa kwa nenosiri
pdf-password = Nenosiri
pdf-password-wrong = Nenosiri si sahihi. Jaribu tena.
pdf-unlock = Fungua kufuli

pdf-sidebar = Upau wa pembeni
pdf-page-of = kati ya { $count }
pdf-zoom-out = Sogeza mbali
pdf-zoom-in = Vuta karibu
pdf-zoom-percent = { $percent }%
pdf-fit-page = Tosheleza ukurasa
pdf-fit-width = Tosheleza upana
pdf-actual-size = Ukubwa halisi
pdf-view-continuous = Usogezaji endelevu
pdf-view-single-page = Ukurasa mmoja
pdf-view-two-pages = Kurasa mbili
pdf-undo = Tendua
pdf-redo = Rudia
pdf-rotate-left = Zungusha kushoto
pdf-rotate-right = Zungusha kulia
pdf-inspector = Kikaguzi
pdf-markup = Alama
pdf-export = Hamisha
pdf-settings = Mipangilio

pdf-search = Tafuta
pdf-search-not-found = Haikupatikana
pdf-searching = Inatafuta…
pdf-search-match = { $current } kati ya { $total }
pdf-search-match-more = { $current } kati ya { $total }+

pdf-inspector-file = Faili
pdf-inspector-document = Hati
pdf-inspector-pages = Kurasa
pdf-inspector-title = Kichwa
pdf-inspector-author = Mwandishi
pdf-inspector-subject = Mada
pdf-inspector-keywords = Maneno muhimu
pdf-inspector-created = Iliundwa
pdf-inspector-modified = Ilibadilishwa
pdf-inspector-application = Programu
pdf-inspector-producer = Kitengeneza PDF
pdf-inspector-version = Toleo
pdf-inspector-security = Usalama
pdf-inspector-not-encrypted = Haijasimbwa kwa njia fiche
pdf-inspector-encrypted = Imesimbwa kwa njia fiche ({ $method })
pdf-inspector-page-count = { $count ->
    [one] ukurasa { $count }
   *[other] kurasa { $count }
}
pdf-inspector-page-size = Ukubwa wa ukurasa
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inchi)
pdf-loading = Inapakia…

pdf-tab-pages = Kurasa
pdf-tab-contents = Yaliyomo
pdf-tab-notes = Viangazio na madokezo
pdf-tab-bookmarks = Alamisho
pdf-no-outline = Hakuna jedwali la yaliyomo
pdf-no-outline-detail = Hati hii haina orodha ya vichwa.
pdf-no-bookmarks = Hakuna alamisho
pdf-no-bookmarks-detail = Bonyeza { $keys } ili kuweka alamisho kwenye ukurasa.
pdf-no-bookmarks-detail-unbound = Kurasa zenye alamisho huonekana hapa.
pdf-remove-bookmark = Ondoa alamisho

## Page editing

pages-menu = Kurasa
pages-insert-blank = Ingiza Ukurasa Tupu
pages-insert-file = Ingiza kutoka kwa Faili…
pages-copy = { $count ->
    [one] Nakili Ukurasa
   *[other] Nakili Kurasa
}
pages-paste = { $count ->
    [one] Bandika Ukurasa
   *[other] Bandika Kurasa { $count }
}
pages-crop = Punguza hadi Uteuzi
pages-select-all = Chagua Kurasa Zote
pages-delete = { $count ->
    [one] Futa Ukurasa
   *[other] Futa Kurasa
}
pages-apply-redactions = Tekeleza Ufichaji…
pages-no-copied = Hakuna kurasa zilizonakiliwa za kubandika.
pages-copied = { $count ->
    [one] Ukurasa { $count } umenakiliwa.
   *[other] Kurasa { $count } zimenakiliwa.
}
pages-copy-failed = Imeshindwa kunakili kurasa: { $error }
pages-reading-stopped = kusoma kumesimama
pages-image-unreadable = si picha ambayo prev inaweza kusoma
pages-read-failed = Imeshindwa kusoma faili: { $error }
pages-at-least-one = Hati inahitaji angalau ukurasa mmoja.
pages-crop-needs-area = Kwanza chagua eneo kwa zana ya uteuzi wa mstatili.
pages-change-failed = Imeshindwa kubadilisha kurasa: { $error }
pages-no-redactions = Hakukuwa na ufichaji wa kutekeleza.
pages-redactions-applied = { $count ->
    [one] Alama { $count } ya ufichaji imetekelezwa.
   *[other] Alama { $count } za ufichaji zimetekelezwa.
}
pages-forget-versions-failed = Imeshindwa kufuta matoleo ya awali: { $error }
pages-redact-title = Tekeleza ufichaji?
pages-redact-body = { $count ->
    [one] Maandishi, picha na michoro iliyo chini ya alama inaondolewa kwenye hati kabisa, na alama inakuwa kisanduku cheusi. Hatua hii haiwezi kutenduliwa, na matoleo ya awali ya faili hii ambayo prev huhifadhi yanafutwa.
   *[other] Maandishi, picha na michoro iliyo chini ya alama { $count } inaondolewa kwenye hati kabisa, na alama zinakuwa visanduku vyeusi. Hatua hii haiwezi kutenduliwa, na matoleo ya awali ya faili hii ambayo prev huhifadhi yanafutwa.
}
pages-redact-apply = Tekeleza

## PDF export

pages-export-title = Hamisha
pages-export-format = Umbizo
pages-export-reduce = Punguza ukubwa wa faili (picha kwa 150 dpi)
pages-export-flatten = Unganisha ufafanuzi na sehemu za fomu kwenye kurasa
pages-export-flatten-detail = Alama na sehemu zilizojazwa zinakuwa sehemu ya kurasa na haziwezi kuhaririwa tena. Alama za ufichaji ambazo bado hazijatekelezwa zinaachwa nje.
pages-export-encrypt = Simba kwa njia fiche kwa nenosiri
pages-export-password = Nenosiri
pages-export-verify-password = Thibitisha nenosiri
pages-export-resolution = Msongo wa picha
pages-export-dpi = { $dpi } dpi
pages-export-quality = Ubora
pages-export-quality-low = Chini
pages-export-quality-medium = Wastani
pages-export-quality-high = Juu
pages-export-quality-best = Bora zaidi
pages-export-one-file = Kurasa zote zinaingia kwenye faili moja.
pages-export-file-per-page = Kila ukurasa unahifadhiwa kama faili yake, yenye nambari baada ya jina unalochagua.
pages-export-selected-only = { $count ->
    [one] Ukurasa uliochaguliwa pekee
   *[other] Kurasa { $count } zilizochaguliwa pekee
}
pages-export-choose = Hamisha…
pages-export-no-password = Weka nenosiri.
pages-export-password-mismatch = Manenosiri hayalingani.
pages-export-file-name = { $name } (imehamishwa)
pages-export-untitled = hati
pages-export-same-file = Hamisha kwenye faili mpya; hati hii hujihifadhi yenyewe.
pages-export-exporting = Inahamisha “{ $name }”…
pages-export-done = “{ $name }” imehamishwa.
pages-export-done-images = { $count ->
    [one] Picha { $count } imehamishwa.
   *[other] Picha { $count } zimehamishwa.
}
pages-export-failed = Imeshindwa kuhamisha: { $error }
pages-export-stopped = kuhamisha kumesimama

## Start window

app-start-hint = Fungua au dondosha faili ya PDF, picha, SVG au Markdown.
app-start-open = Fungua…
app-title-dev = { $title } (usanidi)
app-viewer-missing = { $kind }: kitazamaji hiki bado hakijatengenezwa.
app-cannot-open = prev haiwezi kufungua aina hii ya faili.
app-cannot-read = prev haiwezi kusoma faili hii: { $error }
app-kind-pdf = Hati ya PDF
app-kind-image = Picha ya { $format }
app-kind-svg = Mchoro wa SVG
app-kind-markdown = Hati ya Markdown
app-file-dialog-failed = Imeshindwa kuonyesha kidirisha cha faili: { $error }

## Actions

action-open = Fungua
action-settings = Mipangilio

## Toolbar

app-toolbar-keep-shown = Onyesha upau wa zana kila wakati
app-toolbar-auto-hide = Ficha upau wa zana kielekezi kinapoondoka
app-toolbar-more = Zaidi

## File facts

app-fact-name = Jina
app-fact-folder = Folda
app-fact-size = Ukubwa
app-fact-modified = Ilibadilishwa
app-size-bytes = baiti { $count }
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Kiungo { $uri } si sahihi: { $error }
app-link-open-failed = Imeshindwa kufungua { $uri }: { $error }
app-paste-needs-wl-clipboard = sakinisha wl-clipboard ili kubandika picha
app-copy-needs-wl-clipboard = sakinisha wl-clipboard ili kunakili picha
app-copy-no-pixels = eneo halina pikseli
app-copy-no-input = wl-copy haina ingizo
app-copy-failed = wl-copy imeshindwa
app-clipboard-open-failed = Imeshindwa kufungua ubao wa kunakili: { $error }
app-copy-image-failed = Imeshindwa kunakili picha: { $error }

## Printing

print-failed = Imeshindwa kuchapisha: { $error }
print-stopped = Uchapishaji umesimama
print-unavailable = Uchapishaji bado haupatikani kwenye mfumo huu.
print-no-window = Imeshindwa kuchapisha: hakuna dirisha la kuonyesha kidirisha cha kuchapisha juu yake
print-dialog-failed = Imeshindwa kuonyesha kidirisha cha kuchapisha: { $error }
print-job-not-started = printa haikuanza kazi
print-printer-stopped = printa imesimama

## File dialogs

dialog-open = Fungua
dialog-filter-all = Faili zote zinazotumika
dialog-filter-pdf = Hati za PDF
dialog-filter-images = Picha
dialog-filter-svg = Michoro ya SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Chagua folda ya sahihi
dialog-choose-versions = Chagua folda ya historia ya matoleo
dialog-choose-bookmarks = Chagua faili ya alamisho

## Command line

usage-help =
    Matumizi: prev [FILE]...
              prev --mcp

    Tazama na uhariri PDF na picha. Faili hufunguka katika madirisha ya prev
    inayoendeshwa, ambayo huanza ikihitajika.

    Chaguo:
      -h, --help     Onyesha msaada huu
      -V, --version  Onyesha toleo
          --mcp      Toa MCP kupitia stdin na stdout, ili mawakala wa AI waidhibiti
                     prev inayoendeshwa

## Settings, continued

settings-language = Lugha
settings-language-system = Chaguomsingi la mfumo: { $language }
settings-input-language = Lugha ya kuingiza
settings-input-language-system = Fuata mpangilio wa kibodi
settings-input-language-note = Huweka upande ambao sehemu tupu ya maandishi inaanzia. Maandishi unayoandika hubaki na mwelekeo wake.

settings-appearance-system = Mfumo
settings-appearance-light = Angavu
settings-appearance-dark = Giza
settings-system-accent = Tumia rangi ya msisitizo ya mfumo
settings-omarchy-note = Rangi zinatokana na rangi ya msisitizo ya “{ $theme }”.
settings-system-accent-note = Rangi zinatokana na rangi ya msisitizo ya mfumo.
settings-system-accent-none = Mfumo hauna rangi ya msisitizo, kwa hivyo prev inatumia rangi iliyochaguliwa hapa chini.
settings-accent-chosen-note = Rangi zinatokana na rangi iliyochaguliwa hapa chini.
settings-auto-hide = Ficha upau wa zana kielekezi kinapoondoka
settings-auto-hide-note = Upau wa zana huelea juu ya hati na huteleza mbali wakati kielekezi kiko nje ya dirisha.
settings-animations = Uhuishaji
settings-animations-note = Pau na paneli zinazoteleza, vidirisha vinavyokua na vitufe vinavyodunda.
settings-animations-reduced = Umezimwa wakati mfumo unaomba mwendo uliopunguzwa.
settings-corner-radius = Mviringo wa pembe
settings-corner-radius-note = Kwa vidirisha na upau wa zana unaoelea.
settings-corner-radius-value = pikseli { $radius }
settings-overlay = Uwazi wa kiwekelezo
settings-overlay-note = Kiasi cha ukurasa kinachoonekana kupitia upau wa zana unaoelea.
settings-overlay-value = { $percent }%
settings-storage-signatures = Folda ya sahihi
settings-storage-versions = Folda ya historia ya matoleo
settings-storage-bookmarks = Faili ya alamisho
settings-storage-apply = Tekeleza
settings-storage-choose = Chagua…
settings-storage-note = Faili ambazo tayari ziko mahali pa zamani zinabaki pale; zihamishe ili uendelee kuzitumia. Mipangilio ya programu ya prev huhifadhiwa katika { $file }.
settings-save-failed = Imeshindwa kuhifadhi mipangilio: { $error }
settings-no-location = Hakuna mahali pa mipangilio: HOME haijawekwa
settings-full-path = Tumia njia kamili, kama ~/Documents/prev.
settings-path-is-folder = { $path } ni folda, si faili.
settings-folder-missing = Hakuna folda { $path }. Iunde kwanza, au uchague nyingine.
settings-path-is-file = { $path } ni faili, si folda.
settings-cannot-write = prev haiwezi kuandika katika { $path }: { $error }.

## Export dialog

export-title = Hamisha
export-format = Umbizo
export-quality = Ubora
export-size = Ukubwa
export-choose = Hamisha…
export-format-webp = WebP (bila upotevu)
export-format-unknown = picha
export-quality-low = Chini
export-quality-medium = Wastani
export-quality-high = Juu
export-quality-best = Bora zaidi
export-size-actual = Ukubwa halisi
export-size-scale = { $scale }×
export-size-pixels = pikseli { $width } × { $height }
export-dialog-failed = Imeshindwa kuonyesha kidirisha cha kuhifadhi: { $error }
export-done = { $path } imehamishwa
export-failed = Imeshindwa kuhamisha: { $error }
export-stopped = kuhamisha kumesimama

## Image window

image-marked-no-edit = Picha zenye alama haziwezi kuhaririwa. Hamisha ili kuhifadhi alama, au zifute na ufunge upau wa alama.

image-loading-stopped = kupakia kumesimama
image-reverting-stopped = kurejesha kumesimama
image-rendering-stopped = kuonyesha kumesimama
image-saving-stopped = kuhifadhi kumesimama
image-markup-stopped = uwekaji alama umesimama
image-no-version-store = Hakuna mahali pa kuhifadhi matoleo
image-revert-failed = Imeshindwa kurejesha: { $error }
image-read-failed = Imeshindwa kusoma { $path }: { $error }
image-keep-original-failed = Imeshindwa kuhifadhi toleo asili: { $error }
image-save-failed = Imeshindwa kuhifadhi { $path }: { $error }
image-markup-start-failed = Imeshindwa kuanzisha uwekaji alama: { $error }
image-cannot-edit = Uhuishaji na michoro ya SVG haiwezi kuhaririwa.
image-cannot-mark-up = Uhuishaji na michoro ya SVG haiwezi kuwekewa alama.
image-mark-up-wait = Subiri uhariri ukamilike, kisha uweke alama.
image-crop-needs-selection = Kwanza buruta uteuzi (zana ya Chagua), kisha upunguze.
image-size-needed = Weka upana na urefu kwa pikseli.
image-cannot-save-format = Mabadiliko ya “{ $name }” hayawezi kuhifadhiwa katika umbizo lake. Tumia Hamisha ({ $keys }).
image-cannot-save-format-unbound = Mabadiliko ya “{ $name }” hayawezi kuhifadhiwa katika umbizo lake. Tumia Hamisha.
image-cannot-export-animation = Uhuishaji bado hauwezi kuhamishwa.
image-drop-pages = Kurasa zinaweza kudondoshwa kwenye hati.
image-drag-failed = Imeshindwa kuanza kuburuta.
image-picture-save-failed = Imeshindwa kuhifadhi picha kwenye folda yako ya Vipakuliwa.
image-open-failed = prev haiwezi kufungua picha hii
image-opening = Inafungua…
image-name-mismatch-title = Jina halilingani na umbizo
image-name-mismatch = “{ $name }” itahifadhiwa kama faili ya { $format }, lakini jina lake linaishia na .{ $extension }. Huenda programu nyingine zisiifungue.
image-name-mismatch-no-extension = “{ $name }” itahifadhiwa kama faili ya { $format }, lakini jina lake halina kiendelezi. Huenda programu nyingine zisiifungue.
image-choose-again = Chagua Tena
image-save-as-is = Hifadhi Ilivyo
image-dimensions = { $width } × { $height }
image-frame-position = fremu { $current } kati ya { $total }
image-position = { $current } kati ya { $total }
image-edited = imehaririwa
image-sidebar = Upau wa pembeni
image-zoom-out = Sogeza mbali
image-zoom-in = Vuta karibu
image-zoom = { $percent }%
image-fit = Tosheleza dirisha
image-actual-size = Ukubwa halisi
image-undo = Tendua
image-redo = Rudia
image-rotate-left = Zungusha kushoto
image-rotate-right = Zungusha kulia
image-flip-horizontal = Geuza mlalo
image-flip-vertical = Geuza wima
image-select = Uteuzi wa mstatili
image-crop = Punguza hadi uteuzi
image-adjust-size-tool = Rekebisha ukubwa
image-adjust-color-tool = Rekebisha rangi
image-inspector = Kikaguzi
image-markup = Alama
image-export = Hamisha
image-settings = Mipangilio
image-adjust-color = Rekebisha Rangi
image-adjust-size = Rekebisha Ukubwa
image-exposure = Mwangaza
image-contrast = Utofautishaji
image-saturation = Ukolezaji
image-temperature = Joto
image-tint = Kivuli cha rangi
image-sepia = Sepia
image-sharpness = Ukali
image-levels = Viwango
image-black-point = Kiwango cha weusi
image-midtones = Toni za kati
image-white-point = Kiwango cha weupe
image-reset-all = Weka Upya Zote
image-current-size = Ukubwa wa sasa: pikseli { $width } × { $height }
image-width = Upana
image-height = Urefu
image-scale-proportionally = Badilisha kwa uwiano
image-resize = Badilisha ukubwa
image-inspector-loading = Inapakia…
image-file = Faili
image-format = Umbizo
image-dimensions-label = Vipimo
image-pixels = pikseli { $width } × { $height }
image-no-camera = Hakuna taarifa za kamera.
image-location = Mahali
image-remove-location = Ondoa Taarifa za Mahali
image-no-location = Hakuna taarifa za mahali.
image-keywords-description = Maneno Muhimu na Maelezo
image-keywords-hint = Maneno muhimu, yakitenganishwa kwa koma
image-description = Maelezo
image-keywords-unsupported = Maneno muhimu yanaweza kuhifadhiwa katika faili za JPEG, PNG na WebP.
image-revert-to = Rejesha Hadi
image-no-versions = Hakuna matoleo ya awali.
image-revert = Rejesha
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Funga bila kuhamisha alama?
image-close-body = { $count ->
    [one] Alama kwenye picha hudumu tu wakati dirisha lake liko wazi. Hamisha picha ili kuzihifadhi: alama huchorwa kwenye nakala unayohifadhi.
   *[other] Alama kwenye picha hudumu tu wakati madirisha yake yako wazi. Hamisha kila picha ili kuzihifadhi: alama huchorwa kwenye nakala unayohifadhi.
}
image-close-anyway = Funga Hata Hivyo

## Markdown

markdown-reading-stopped = kusoma kumesimama
markdown-read-failed = prev haiwezi kusoma faili hii
markdown-draw-failed = Imeshindwa kuchora hati
markdown-export-size = Hati nzima, pikseli { $width } × { $height }
markdown-not-found = Haikupatikana
markdown-match = { $current } kati ya { $total }
markdown-search = Tafuta
markdown-smaller-text = Maandishi madogo zaidi
markdown-larger-text = Maandishi makubwa zaidi
markdown-zoom = { $percent }%
markdown-actual-size = Ukubwa halisi
markdown-limit-width = Weka kikomo cha upana wa maandishi
markdown-inspector = Kikaguzi
markdown-export = Hamisha
markdown-settings = Mipangilio
markdown-file = Faili
markdown-document = Hati
markdown-words = Maneno
markdown-lines = Mistari
markdown-pictures = Picha

## Image details

image-meta-camera = Kamera
image-meta-exposure = Mwangaza
image-meta-image = Picha
image-meta-make = Mtengenezaji
image-meta-model = Muundo
image-meta-lens = Lenzi
image-meta-exposure-time = Muda wa mwangaza
image-meta-f-number = Namba ya F
image-meta-iso = ISO
image-meta-focal-length = Urefu wa fokasi
image-meta-exposure-bias = Fidia ya mwangaza
image-meta-flash = Fleshi
image-meta-date-taken = Tarehe ilipopigwa
image-meta-orientation = Mkao
image-meta-color-space = Nafasi ya rangi
image-meta-software = Programu
image-meta-artist = Msanii
image-meta-copyright = Hakimiliki
image-meta-seconds = sekunde { $value }
image-meta-millimeters = milimita { $value }
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Kawaida
    [2] Imegeuzwa kwa mlalo
    [3] Imezungushwa 180°
    [4] Imegeuzwa kwa wima
    [5] Imegeuzwa kwa mlalo, imezungushwa 90° kinyume cha saa
    [6] Imezungushwa 90° kisaa
    [7] Imegeuzwa kwa mlalo, imezungushwa 90° kisaa
    [8] Imezungushwa 90° kinyume cha saa
   *[other] Haijulikani ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Iliwaka
   *[no] Haikuwaka
}{ $mode ->
    [on] , imelazimishwa kuwaka
    [off] , imezimwa
    [auto] , otomatiki
   *[unknown] {""}
}{ $redeye ->
    [yes] , kupunguza macho mekundu
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Haijasawazishwa
   *[other] Nyingine ({ $code })
}

## Errors

error-pdf-open = haiwezi kufungua hati: { $detail }
error-pdf-page-out-of-range = ukurasa { $page } haupo
error-pdf-password-protected = hati imelindwa kwa nenosiri; ifungue na unakili kurasa zake badala yake
error-pdf-no-pages = hakuna kurasa za kutoa
error-pdf-crop-outside = eneo la kupunguza liko nje ya ukurasa
error-pdf-closed = hati imefungwa
error-pdf-saved-unreadable = hati iliyohifadhiwa haifunguki tena
error-image-read = haiwezi kusoma faili: { $detail }
error-image-invalid = picha imeharibika au si sahihi: { $detail }
error-image-missing-library = kufungua umbizo hili kunahitaji { $library }, ambayo haijasakinishwa
error-image-unsupported = picha za { $format } bado hazitumiki
error-image-encode = haiwezi kusimba picha: { $detail }
error-exif-malformed = data ya EXIF ina hitilafu
error-settings-read = haiwezi kusoma mipangilio: { $detail }
error-settings-invalid = mipangilio si sahihi: { $detail }
error-remove-location = imeshindwa kuondoa mahali: { $error }
error-location-unsupported = taarifa za mahali zinaweza kuondolewa kwenye faili za JPEG, PNG, WebP na TIFF
error-xmp-unsupported = maneno muhimu na maelezo yanaweza kuhifadhiwa katika faili za JPEG, PNG na WebP pekee

## Formats

format-camera-raw = RAW ya kamera

## The macOS menu bar, named as in macOS's own apps.
menu-about = Kuhusu prev
menu-settings = Mipangilio…
menu-services = Huduma
menu-hide = Ficha prev
menu-hide-others = Ficha Nyingine
menu-show-all = Onyesha Zote
menu-quit = Ondoka kwenye prev
menu-file = Faili
menu-open = Fungua…
menu-close = Funga Dirisha
menu-export = Hamisha…
menu-print = Chapisha…
menu-edit = Hariri
menu-undo = Tendua
menu-redo = Rudia
menu-cut = Kata
menu-copy = Nakili
menu-paste = Bandika
menu-select-all = Chagua Zote
menu-find = Tafuta
menu-find-next = Tafuta Kinachofuata
menu-find-previous = Tafuta Kilichotangulia
menu-view = Mwonekano
menu-hide-sidebar = Ficha Upau wa Pembeni
menu-thumbnails = Vijipicha
menu-contents = Jedwali la Yaliyomo
menu-notes = Viangazio na Madokezo
menu-bookmarks = Alamisho
menu-zoom-in = Vuta Karibu
menu-zoom-out = Sogeza Mbali
menu-actual-size = Ukubwa Halisi
menu-zoom-to-fit = Kuza Kutosheleza
menu-inspector = Onyesha Kikaguzi
menu-slideshow = Onyesho la Slaidi
menu-full-screen = Ingia Skrini Nzima
menu-go = Nenda
menu-next-page = Ukurasa Unaofuata
menu-previous-page = Ukurasa Uliotangulia
menu-go-to-page = Nenda kwenye Ukurasa…
menu-bookmark = Ongeza Alamisho
menu-tools = Zana
menu-markup = Onyesha Upau wa Zana za Alama
menu-rotate-left = Zungusha Kushoto
menu-rotate-right = Zungusha Kulia
menu-crop = Punguza
menu-adjust-color = Rekebisha Rangi…
menu-window = Dirisha
menu-minimize = Punguza Dirisha
menu-zoom = Kuza
menu-bring-all-to-front = Leta Zote Mbele

## Outside control

settings-outside-control = Udhibiti wa nje
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = Jumla
settings-tab-agents = Mawakala
settings-allow-outside-control = Ruhusu udhibiti wa nje
settings-allow-outside-control-note = Mawakala wa AI kama Claude Code wanaweza kusoma na kubadilisha faili zako katika prev, kupitia prev --mcp. prev huuliza kabla ya kila wakala mpya.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Wanaoruhusiwa: { $agents }
settings-forget-agents = Sahau
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Ruhusu { $agent } kudhibiti prev?
agent-prompt-body = { $agent } anaomba kutumia udhibiti wa nje wa prev, ili kusoma faili zako zilizofunguliwa na kuzibadilisha. Unaweza kuzima udhibiti wa nje katika Mipangilio.
agent-prompt-allow = Ruhusu
agent-prompt-deny = Usiruhusu
settings-ask-before-note = Uliza kwanza wakala anapotaka:
settings-ask-reading = Kusoma faili
settings-ask-viewing = Kubadilisha mwonekano au dirisha
settings-ask-marking-up = Kuweka alama kwenye faili
settings-ask-editing = Kuhariri faili
settings-ask-signing = Kuweka sahihi kwenye faili
settings-ask-redacting = Kutekeleza ufichaji
settings-ask-exporting = Kuhamisha faili
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Ruhusu { $agent } kusoma faili hii?
agent-ask-view = Ruhusu { $agent } kubadilisha mwonekano?
agent-ask-markup = Ruhusu { $agent } kuweka alama kwenye faili hii?
agent-ask-edit = Ruhusu { $agent } kuhariri faili hii?
agent-ask-sign = Ruhusu { $agent } kuweka sahihi kwenye faili hii?
agent-ask-redact = Ruhusu { $agent } kutekeleza ufichaji?
agent-ask-export = Ruhusu { $agent } kuhamisha faili hii?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } anaomba kutumia “{ $tool }”. Katika Mipangilio unachagua mambo ambayo prev huuliza.
agent-ask-final = Hili haliwezi kutenduliwa.

## The assistant
settings-tab-assistant = Msaidizi
settings-assistant-note = Modeli ambazo kidirisha cha msaidizi kinaweza kuzungumza nazo. Funguo huhifadhiwa katika hifadhi ya funguo ya mfumo.
settings-assistant-none = Bado hakuna modeli. Ongeza moja hapa chini: modeli ya ndani, kama ile ya Ollama, hubaki kwenye kompyuta hii; modeli ya wingu inahitaji ufunguo wa API kutoka kwa mtoa huduma wake.
settings-assistant-in-use = Inatumika
settings-assistant-use = Tumia
settings-assistant-remove = Ondoa
settings-assistant-add = Ongeza modeli
# The menu entry for a server that speaks OpenAI's API.
settings-assistant-compatible = Seva inayooana na OpenAI
# $example is a model name, such as claude-sonnet-5-5.
settings-assistant-model = Modeli, kwa mfano { $example }
settings-assistant-key = Ufunguo wa API
# $example is an address, such as http://localhost:11434.
settings-assistant-address = Anwani, kwa mfano { $example }
# The menu of how much a local model reads at once.
settings-assistant-context = Muktadha
# $thousands is the size in thousands of tokens, such as 32.
settings-assistant-context-size = Tokeni { $thousands }K
settings-assistant-context-note = Muktadha mkubwa zaidi humwezesha msaidizi kusoma sehemu kubwa zaidi ya faili katika gumzo moja, lakini modeli hutumia kumbukumbu zaidi na huenda ikajibu polepole zaidi.
settings-assistant-add-button = Ongeza
settings-assistant-use-key = Endelea
# $provider is a cloud provider, such as Anthropic.
settings-assistant-key-where = Tengeneza ufunguo kwenye tovuti ya { $provider } kisha uubandike hapa.
settings-assistant-get-key = Pata ufunguo wa API
settings-assistant-key-kept = Ufunguo wako wa { $provider } umehifadhiwa katika hifadhi ya funguo ya mfumo.
settings-assistant-change-key = Badilisha ufunguo
settings-assistant-key-refused = { $provider } imekataa ufunguo huu. Hakikisha umenakiliwa wote, kutoka kwa akaunti sahihi.
# $provider is a local server, such as Ollama; $address is where it answered.
settings-assistant-found-at = { $provider } inaendeshwa kwenye { $address }.
settings-assistant-no-server = prev haikupata { $provider } ikiendeshwa kwenye kompyuta hii. Iwashe, au weka anwani yake hapa chini.
settings-assistant-get-server = Pata { $provider }
settings-assistant-look-again = Tafuta tena
# Shows the address field, to use a server on another computer.
settings-assistant-other-address = Tumia anwani nyingine
settings-assistant-looking = Inatafuta modeli…
settings-assistant-found-none = { $provider } bado haina modeli. Pakua moja kwa kuitumia, kisha utafute tena.
settings-assistant-recommended = Inapendekezwa
settings-assistant-uses-tools = Hutumia zana
settings-assistant-sees = Huona picha
settings-assistant-no-tools = Haiwezi kutumia zana, ambazo msaidizi anahitaji
settings-assistant-added-tag = Imeongezwa
settings-assistant-trying = Inajaribu…
# Opens the provider's page that fixes the problem shown, such as billing.
settings-assistant-fix-it = Fungua ukurasa
# $model is the model's name.
settings-assistant-added = { $model } imejibu na imeongezwa.
settings-assistant-key-needed = Modeli hii inahitaji ufunguo wa API.
# $error is what the keychain said.
settings-assistant-key-failed = Ufunguo haukuweza kuhifadhiwa katika hifadhi ya funguo: { $error }
assistant-title = Msaidizi
assistant-new-chat = Gumzo jipya
assistant-ask = Uliza kuhusu faili hii
assistant-send = Tuma
assistant-stop = Simamisha
assistant-thinking = Inafikiri…
# Folded away above a reply: what the model thought before it.
assistant-thoughts = Mawazo
assistant-running = Inaendesha…
assistant-stopped = Imesimamishwa.
assistant-no-model = Ongeza modeli katika Mipangilio kwanza.
assistant-add-model = Msaidizi anahitaji modeli: modeli ya wingu pamoja na ufunguo wake wa API, au modeli ya ndani.
assistant-open-settings = Ongeza modeli
# $model is the model's name, such as claude-sonnet-5.
assistant-switched = Sasa unazungumza na { $model }.
assistant-add-another = Ongeza modeli…
# A heading in the model menu for models on this computer; $provider is
# the server, such as Ollama.
assistant-group-local = { $provider } kwenye kompyuta hii
# A heading for models on another computer; $host is its address, such
# as 192.168.4.61.
assistant-group-remote = { $provider } kwenye { $host }
# Why the assistant's model did not answer. $model is the model's name,
# such as qwen3.8; $provider is who serves it, such as Anthropic or Ollama.
assistant-problem-context = Mazungumzo hayatoshi tena katika kiasi ambacho { $model } inaweza kusoma kwa wakati mmoja. Anzisha gumzo jipya, au chagua modeli inayoweza kusoma zaidi.
assistant-problem-key = { $provider } imekataa ufunguo wa API. Uangalie katika Mipangilio.
assistant-problem-rate = { $provider } inaomba upunguze kasi. Jaribu tena baada ya muda mfupi.
# $message is the provider's own words, untranslated, such as which limit
# was reached and when to try again.
assistant-problem-rate-said = { $provider } inaomba upunguze kasi: { $message }
assistant-problem-credit = { $provider } inasema akaunti haina salio. Akaunti mpya inahitaji kununua salio kwenye tovuti ya { $provider } kabla ufunguo wake haujafanya kazi; kisha jaribu tena.
assistant-problem-model = { $provider } haina modeli inayoitwa { $model }. Angalia jina lake katika Mipangilio.
assistant-problem-unavailable = { $provider } ina shughuli nyingi au ina tatizo. Jaribu tena baada ya muda mfupi.
assistant-problem-unreachable = prev haikuweza kufikia { $provider }. Angalia muunganisho wako, au kama seva inaendeshwa.
assistant-problem-refused = { $model } imekataa kujibu.
# $message is what the provider said, untranslated.
assistant-problem-other = { $model } haikujibu: { $message }
