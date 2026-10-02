# prev's interface text in Finnish (Suomi), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = merkinnät, note = muistiinpano, highlight = korostus,
# annotation = huomautus, redact/redaction = mustaa/mustaus, inspector = tiedot (tietopaneeli),
# selection = valinta, bookmark = kirjanmerkki, page = sivu.
# Error messages use the pattern "X epäonnistui" so placeholders need no case endings.

## Language

language-name = Suomi

## Common

common-cancel = Peruuta
common-close = Sulje
common-save = Tallenna

## Settings

settings-title = Asetukset
settings-appearance = Ulkoasu
settings-colors = Värit
settings-windows = Ikkunat
settings-default-app = Oletussovellus
settings-default-app-label = Avaa tiedostot previllä
settings-default-app-note = Tee previstä sovellus, joka avaa PDF-tiedostot, kuvat, SVG-piirrokset ja Markdown-tiedostot.
settings-default-app-note-windows = Windowsissa oletussovellukset voi valita vain sen omista asetuksista. Tämä avaa previn sivun siellä.
settings-default-app-note-macos = macOS pyytää vahvistamaan jokaisen tyypin: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP ja AVIF.
settings-default-app-status = { $set }/{ $total } tiedostotyyppiä avautuu previllä.
settings-default-app-button = Aseta oletukseksi
settings-default-app-button-windows = Avaa asetukset
settings-default-app-no-entry = Järjestelmä ei voi avata tiedostoja previllä, koska sen työpöytätiedostoa ei ole asennettu. Asenna prev paketista tai komennolla scripts/install.sh.
settings-default-app-no-bundle = Avaa prev kohteesta prev.app, jotta voit asettaa sen oletukseksi.
settings-default-app-failed = previn asettaminen oletukseksi epäonnistui: { $error }
settings-storage = Tallennus
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (kehitysversio, { $build })

## Markup toolbar

markup-tool-select = Valitse
markup-tool-area = Suorakulmainen valinta
markup-tool-sketch = Luonnos
markup-tool-draw = Piirrä
markup-tool-shapes = Muodot
markup-tool-text-box = Tekstiruutu
markup-tool-highlight = Korosta
markup-tool-note = Muistiinpano
markup-tool-sign = Allekirjoita
markup-tool-redact = Mustaa
markup-apply = Toteuta
markup-apply-redactions = Toteuta mustaukset
markup-shape-style = Muodon tyyli
markup-border-color = Reunan väri
markup-fill-color = Täyttöväri
markup-text-style = Tekstin tyyli
markup-delete = Poista
markup-undo = Kumoa
markup-redo = Tee uudelleen

## Markup menus

markup-shape-rectangle = Suorakulmio
markup-shape-rounded-rectangle = Pyöristetty suorakulmio
markup-shape-oval = Soikio
markup-shape-line = Viiva
markup-shape-arrow = Nuoli
markup-shape-star = Tähti
markup-shape-polygon = Monikulmio
markup-shape-speech-bubble = Puhekupla
markup-shape-loupe = Suurennuslasi
markup-shape-mask = Maski
markup-style-highlight = Korostus
markup-style-underline = Alleviivaus
markup-style-strikethrough = Yliviivaus
markup-style-squiggly = Aaltoviiva
markup-menu-color = Väri
markup-menu-font = Fontti
markup-menu-size = Koko
markup-menu-alignment = Tasaus
markup-line-width = { $width } pt
markup-dashed = Katkoviiva

## Notes

markup-kind-note = Muistiinpano
markup-kind-text-box = Tekstiruutu
markup-kind-stamp = Leima
markup-kind-redaction = Mustaus
markup-kind-shape = Muoto
markup-note-delete = Poista muistiinpano
markup-note-done = Valmis
markup-note-placeholder = Kirjoita muistiinpano
markup-notes-empty = Ei korostuksia tai muistiinpanoja
markup-notes-empty-hint = Korostukset, muistiinpanot ja tekstiruudut näkyvät tässä.
markup-notes-page = Sivu { $page }

## Markup errors

markup-change-failed = Asiakirjan muuttaminen epäonnistui: { $error }
markup-copy-area-failed = Alueen kopioiminen epäonnistui: { $error }
markup-document-closed = asiakirja suljettiin
markup-render-area-failed = alueen piirtäminen epäonnistui
markup-copy-stopped = kopioiminen keskeytyi

## Signatures

signature-menu-empty = Ei vielä allekirjoituksia.
signature-delete = Poista allekirjoitus
signature-create = Luo allekirjoitus…
signature-dialog-title = Luo allekirjoitus
signature-tab-draw = Piirrä
signature-tab-type = Kirjoita
signature-tab-image = Kuva
signature-draw-hint = Allekirjoita viivalle hiirellä, kynällä tai ohjauslevyllä.
signature-your-name = Nimesi
signature-image-hint = Valitse valokuva tai skannaus allekirjoituksestasi valkoisella paperilla.
signature-choose-image = Valitse kuva…
signature-description = Kuvaus, esimerkiksi Koko nimi tai Nimikirjaimet
signature-clear = Tyhjennä
signature-ink = Muste
signature-thickness = Paksuus
signature-sign-first = Allekirjoita ensin ja tallenna sitten.
signature-default-name = Allekirjoitus { $number }
signature-change-failed = Allekirjoitusten muuttaminen epäonnistui: { $error }
signature-no-data-folder = ei datakansiota: HOME-arvoa ei ole asetettu
signature-removing-stopped = poistaminen keskeytyi
signature-saving-stopped = tallentaminen keskeytyi
signature-reading-stopped = lukeminen keskeytyi
signature-not-an-image = tiedosto ei ole kuva, jota prev osaa lukea
signature-no-frames = kuvassa ei ole ruutuja
signature-not-found = kuvasta ei löytynyt allekirjoitusta

## Dragging

drag-pages-need-document = Sivuja voi pudottaa asiakirjaan.
drag-image-unsupported = prev ei voi avata tätä kuvaa.
drag-area-failed = Alueen vetäminen epäonnistui: { $error }
drag-pages-failed = Sivujen vetäminen epäonnistui: { $error }
drag-start-failed = Vetämisen aloittaminen epäonnistui.
drag-file-pages = Sivut
drag-file-one-page = { $name } (sivu { $page })
drag-file-page-range = { $name } (sivut { $first }–{ $last })
drag-file-image = Kuva
drop-pdf-title = Lisätäänkö tähän asiakirjaan?
drop-pdf-body = Lisätäänkö ”{ $name }” tämän asiakirjan loppuun vai avataanko se omaan ikkunaansa?
drop-pdfs-body = { $count ->
    [one] Lisätäänkö tämä PDF-tiedosto tämän asiakirjan loppuun vai avataanko se omaan ikkunaansa?
   *[other] Lisätäänkö nämä { $count } PDF-tiedostoa tämän asiakirjan loppuun vai avataanko ne omiin ikkunoihinsa?
}
drop-pdf-add = Lisää loppuun
drop-pdf-open = Avaa erikseen

## PDF window

pdf-opening = Avataan…
pdf-open-failed = prev ei voi avata tätä asiakirjaa
pdf-no-pages = Asiakirjassa ei ole sivuja.
pdf-document-closed = asiakirja suljettiin
pdf-keep-original-failed = alkuperäisen version säilyttäminen epäonnistui: { $error }
pdf-save-failed = Tallentaminen epäonnistui: { $error }
pdf-nothing-to-paste = Ei mitään liitettävää.
pdf-pasting-stopped = liittäminen keskeytyi
pdf-file-dialog-failed = Tiedostoikkunan näyttäminen epäonnistui: { $error }
pdf-bookmarks-no-home = Kirjanmerkkejä ei voi tallentaa: HOME-arvoa ei ole asetettu
pdf-bookmarks-save-failed = Kirjanmerkkien tallentaminen epäonnistui: { $error }
pdf-bookmark-page = Sivu { $page }

pdf-password-protected = ”{ $name }” on suojattu salasanalla
pdf-password = Salasana
pdf-password-wrong = Väärä salasana. Yritä uudelleen.
pdf-unlock = Avaa lukitus

pdf-sidebar = Sivupalkki
pdf-page-of = / { $count }
pdf-zoom-out = Loitonna
pdf-zoom-in = Lähennä
pdf-zoom-percent = { $percent } %
pdf-fit-page = Sovita sivu
pdf-fit-width = Sovita leveys
pdf-actual-size = Todellinen koko
pdf-view-continuous = Jatkuva vieritys
pdf-view-single-page = Yksi sivu
pdf-view-two-pages = Kaksi sivua
pdf-undo = Kumoa
pdf-redo = Tee uudelleen
pdf-rotate-left = Kierrä vasemmalle
pdf-rotate-right = Kierrä oikealle
pdf-inspector = Tiedot
pdf-markup = Merkinnät
pdf-export = Vie
pdf-settings = Asetukset

pdf-search = Etsi
pdf-search-not-found = Ei löytynyt
pdf-searching = Etsitään…
pdf-search-match = { $current }/{ $total }
pdf-search-match-more = { $current }/{ $total }+

pdf-inspector-file = Tiedosto
pdf-inspector-document = Asiakirja
pdf-inspector-pages = Sivut
pdf-inspector-title = Otsikko
pdf-inspector-author = Tekijä
pdf-inspector-subject = Aihe
pdf-inspector-keywords = Avainsanat
pdf-inspector-created = Luotu
pdf-inspector-modified = Muokattu
pdf-inspector-application = Sovellus
pdf-inspector-producer = PDF-tuottaja
pdf-inspector-version = Versio
pdf-inspector-security = Suojaus
pdf-inspector-not-encrypted = Ei salattu
pdf-inspector-encrypted = Salattu ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } sivu
   *[other] { $count } sivua
}
pdf-inspector-page-size = Sivun koko
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } tuumaa)
pdf-loading = Ladataan…

pdf-tab-pages = Sivut
pdf-tab-contents = Sisältö
pdf-tab-notes = Korostukset ja muistiinpanot
pdf-tab-bookmarks = Kirjanmerkit
pdf-no-outline = Ei sisällysluetteloa
pdf-no-outline-detail = Tässä asiakirjassa ei ole jäsennystä.
pdf-no-bookmarks = Ei kirjanmerkkejä
pdf-no-bookmarks-detail = Lisää sivulle kirjanmerkki painamalla { $keys }.
pdf-no-bookmarks-detail-unbound = Kirjanmerkityt sivut näkyvät tässä.
pdf-remove-bookmark = Poista kirjanmerkki

## Page editing

pages-menu = Sivut
pages-insert-blank = Lisää tyhjä sivu
pages-insert-file = Lisää tiedostosta…
pages-copy = { $count ->
    [one] Kopioi sivu
   *[other] Kopioi sivut
}
pages-paste = { $count ->
    [one] Liitä sivu
   *[other] Liitä { $count } sivua
}
pages-crop = Rajaa valintaan
pages-select-all = Valitse kaikki sivut
pages-delete = { $count ->
    [one] Poista sivu
   *[other] Poista sivut
}
pages-apply-redactions = Toteuta mustaukset…
pages-no-copied = Liitettäviä kopioituja sivuja ei ole.
pages-copied = { $count ->
    [one] Kopioitiin { $count } sivu.
   *[other] Kopioitiin { $count } sivua.
}
pages-copy-failed = Sivujen kopioiminen epäonnistui: { $error }
pages-reading-stopped = lukeminen keskeytyi
pages-image-unreadable = ei ole kuva, jota prev osaa lukea
pages-read-failed = Tiedoston lukeminen epäonnistui: { $error }
pages-at-least-one = Asiakirjassa on oltava vähintään yksi sivu.
pages-crop-needs-area = Valitse ensin alue suorakulmaisen valinnan työkalulla.
pages-change-failed = Sivujen muuttaminen epäonnistui: { $error }
pages-no-redactions = Toteutettavia mustauksia ei ollut.
pages-redactions-applied = { $count ->
    [one] Toteutettiin { $count } mustaus.
   *[other] Toteutettiin { $count } mustausta.
}
pages-forget-versions-failed = Aiempien versioiden poistaminen epäonnistui: { $error }
pages-redact-title = Toteutetaanko mustaukset?
pages-redact-body = { $count ->
    [one] Mustausmerkinnän alla olevat tekstit, kuvat ja piirrokset poistetaan asiakirjasta pysyvästi, ja merkinnästä tulee musta laatikko. Tätä ei voi kumota, ja previn säilyttämät tämän tiedoston aiemmat versiot poistetaan.
   *[other] { $count } mustausmerkinnän alla olevat tekstit, kuvat ja piirrokset poistetaan asiakirjasta pysyvästi, ja merkinnöistä tulee mustia laatikoita. Tätä ei voi kumota, ja previn säilyttämät tämän tiedoston aiemmat versiot poistetaan.
}
pages-redact-apply = Toteuta

## PDF export

pages-export-title = Vie
pages-export-format = Tiedostomuoto
pages-export-reduce = Pienennä tiedostokokoa (kuvat 150 dpi:n tarkkuudella)
pages-export-flatten = Litistä huomautukset ja lomakekentät
pages-export-flatten-detail = Merkinnät ja täytetyt kentät tulevat osaksi sivuja, eikä niitä voi enää muokata. Toteuttamattomat mustausmerkinnät jätetään pois.
pages-export-encrypt = Salaa salasanalla
pages-export-password = Salasana
pages-export-verify-password = Vahvista salasana
pages-export-resolution = Tarkkuus
pages-export-dpi = { $dpi } dpi
pages-export-quality = Laatu
pages-export-quality-low = Matala
pages-export-quality-medium = Keskitaso
pages-export-quality-high = Korkea
pages-export-quality-best = Paras
pages-export-one-file = Kaikki sivut tallennetaan yhteen tiedostoon.
pages-export-file-per-page = Jokainen sivu tallennetaan omaksi tiedostokseen, jonka nimi on valitsemasi nimi ja juokseva numero.
pages-export-selected-only = { $count ->
    [one] Vain valittu sivu
   *[other] Vain { $count } valittua sivua
}
pages-export-choose = Vie…
pages-export-no-password = Anna salasana.
pages-export-password-mismatch = Salasanat eivät täsmää.
pages-export-file-name = { $name } (viety)
pages-export-untitled = asiakirja
pages-export-same-file = Vie uuteen tiedostoon; tämä asiakirja tallentuu itsestään.
pages-export-exporting = Viedään: ”{ $name }”…
pages-export-done = Vietiin: ”{ $name }”.
pages-export-done-images = Vietiin { $count } kuvaa.
pages-export-failed = Vieminen epäonnistui: { $error }
pages-export-stopped = vieminen keskeytyi

## Start window

app-start-hint = Avaa tai pudota tähän PDF-, kuva-, SVG- tai Markdown-tiedosto.
app-start-open = Avaa…
app-title-dev = { $title } (kehitys)
app-viewer-missing = { $kind }: tätä katseluohjelmaa ei ole vielä tehty.
app-cannot-open = prev ei voi avata tämäntyyppistä tiedostoa.
app-cannot-read = prev ei voi lukea tätä tiedostoa: { $error }
app-kind-pdf = PDF-asiakirja
app-kind-image = { $format }-kuva
app-kind-svg = SVG-piirros
app-kind-markdown = Markdown-asiakirja
app-file-dialog-failed = Tiedostoikkunan näyttäminen epäonnistui: { $error }

## Actions

action-open = Avaa
action-settings = Asetukset

## Toolbar

app-toolbar-keep-shown = Pidä työkalupalkki näkyvissä
app-toolbar-auto-hide = Piilota työkalupalkki, kun osoitin poistuu ikkunasta
app-toolbar-more = Lisää

## File facts

app-fact-name = Nimi
app-fact-folder = Kansio
app-fact-size = Koko
app-fact-modified = Muokattu
app-size-bytes = { $count } tavua
app-size-kb = { $size } kt
app-size-mb = { $size } Mt
app-size-gb = { $size } Gt
app-size-tb = { $size } Tt

## Links and clipboard

app-link-invalid = Virheellinen linkki { $uri }: { $error }
app-link-open-failed = Kohteen { $uri } avaaminen epäonnistui: { $error }
app-paste-needs-wl-clipboard = asenna wl-clipboard, jotta voit liittää kuvia
app-copy-needs-wl-clipboard = asenna wl-clipboard, jotta voit kopioida kuvia
app-copy-no-pixels = alueella ei ole pikseleitä
app-copy-no-input = wl-copy ei saanut syötettä
app-copy-failed = wl-copy epäonnistui
app-clipboard-open-failed = Leikepöydän avaaminen epäonnistui: { $error }
app-copy-image-failed = Kuvan kopioiminen epäonnistui: { $error }

## Printing

print-failed = Tulostaminen epäonnistui: { $error }
print-stopped = Tulostus keskeytyi
print-unavailable = Tulostus ei ole vielä käytettävissä tässä järjestelmässä.
print-no-window = Tulostaminen epäonnistui: ei ikkunaa, jonka päällä tulostusikkunan voisi näyttää
print-dialog-failed = Tulostusikkunan näyttäminen epäonnistui: { $error }
print-job-not-started = tulostin ei aloittanut työtä
print-printer-stopped = tulostin pysähtyi

## File dialogs

dialog-open = Avaa
dialog-filter-all = Kaikki tuetut tiedostot
dialog-filter-pdf = PDF-asiakirjat
dialog-filter-images = Kuvat
dialog-filter-svg = SVG-piirrokset
dialog-filter-markdown = Markdown-tiedostot
dialog-choose-signatures = Valitse allekirjoitusten kansio
dialog-choose-versions = Valitse versiohistorian kansio
dialog-choose-bookmarks = Valitse kirjanmerkkitiedosto

## Command line

usage-help =
    Käyttö: prev [FILE]...
            prev --mcp

    Katsele ja muokkaa PDF-tiedostoja ja kuvia. Tiedostot avautuvat käynnissä
    olevan previn ikkunoihin, ja prev käynnistyy tarvittaessa.

    Valitsimet:
      -h, --help     Näytä tämä ohje
      -V, --version  Näytä versio
          --mcp      Tarjoa MCP stdinissä ja stdoutissa, jotta tekoälyagentit voivat
                     ohjata käynnissä olevaa previä

## Settings, continued

settings-language = Kieli
settings-language-system = Järjestelmän oletus: { $language }
settings-input-language = Syöttökieli
settings-input-language-system = Seuraa näppäimistöasettelua
settings-input-language-note = Määrittää, kummasta reunasta tyhjä tekstikenttä alkaa. Kirjoittamasi teksti säilyttää oman suuntansa.

settings-appearance-system = Järjestelmä
settings-appearance-light = Vaalea
settings-appearance-dark = Tumma
settings-system-accent = Käytä järjestelmän korostusväriä
settings-omarchy-note = Värit muodostetaan teeman ”{ $theme }” korostusväristä.
settings-system-accent-note = Värit muodostetaan järjestelmän korostusväristä.
settings-system-accent-none = Järjestelmässä ei ole korostusväriä, joten prev käyttää alla valittua.
settings-accent-chosen-note = Värit muodostetaan alla valitusta väristä.
settings-auto-hide = Piilota työkalupalkki, kun osoitin poistuu ikkunasta
settings-auto-hide-note = Työkalupalkki kelluu asiakirjan päällä ja liukuu pois, kun osoitin on ikkunan ulkopuolella.
settings-animations = Animaatiot
settings-animations-note = Liukuvat palkit ja paneelit, kasvavat valintaikkunat ja joustavat painikkeet.
settings-animations-reduced = Pois käytöstä, kun järjestelmä pyytää vähentämään liikettä.
settings-corner-radius = Kulmien pyöristys
settings-corner-radius-note = Valintaikkunoille ja kelluvalle työkalupalkille.
settings-corner-radius-value = { $radius } px
settings-overlay = Peittokuvan läpinäkyvyys
settings-overlay-note = Kuinka paljon sivusta näkyy kelluvan työkalupalkin läpi.
settings-overlay-value = { $percent } %
settings-storage-signatures = Allekirjoitusten kansio
settings-storage-versions = Versiohistorian kansio
settings-storage-bookmarks = Kirjanmerkkitiedosto
settings-storage-apply = Käytä
settings-storage-choose = Valitse…
settings-storage-note = Vanhaan paikkaan jo tallennetut tiedostot jäävät sinne; siirrä ne uuteen paikkaan, jos haluat käyttää niitä edelleen. previn asetukset tallennetaan tiedostoon { $file }.
settings-save-failed = Asetusten tallentaminen epäonnistui: { $error }
settings-no-location = Asetuksille ei ole sijaintia: HOME-arvoa ei ole asetettu
settings-full-path = Käytä koko polkua, esimerkiksi ~/Documents/prev.
settings-path-is-folder = { $path } on kansio, ei tiedosto.
settings-folder-missing = Kansiota { $path } ei ole. Luo se ensin tai valitse jokin muu.
settings-path-is-file = { $path } on tiedosto, ei kansio.
settings-cannot-write = prev ei voi kirjoittaa kansioon { $path }: { $error }.

## Export dialog

export-title = Vie
export-format = Tiedostomuoto
export-quality = Laatu
export-size = Koko
export-choose = Vie…
export-format-webp = WebP (häviötön)
export-format-unknown = kuva
export-quality-low = Matala
export-quality-medium = Keskitaso
export-quality-high = Korkea
export-quality-best = Paras
export-size-actual = Todellinen koko
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } pikseliä
export-dialog-failed = Tallennusikkunan näyttäminen epäonnistui: { $error }
export-done = Vietiin: { $path }
export-failed = Vieminen epäonnistui: { $error }
export-stopped = vieminen keskeytyi

## Image window

image-marked-no-edit = Kuvia, joissa on merkintöjä, ei voi muokata. Säilytä merkinnät viemällä kuva tai poista ne ja sulje merkintäpalkki.

image-loading-stopped = lataaminen keskeytyi
image-reverting-stopped = palauttaminen keskeytyi
image-rendering-stopped = piirtäminen keskeytyi
image-saving-stopped = tallentaminen keskeytyi
image-markup-stopped = merkintä keskeytyi
image-no-version-store = Versioille ei ole tallennuspaikkaa
image-revert-failed = Palauttaminen epäonnistui: { $error }
image-read-failed = Tiedoston { $path } lukeminen epäonnistui: { $error }
image-keep-original-failed = Alkuperäisen version säilyttäminen epäonnistui: { $error }
image-save-failed = Tiedoston { $path } tallentaminen epäonnistui: { $error }
image-markup-start-failed = Merkinnän aloittaminen epäonnistui: { $error }
image-cannot-edit = Animaatioita ja SVG-piirroksia ei voi muokata.
image-cannot-mark-up = Animaatioihin ja SVG-piirroksiin ei voi lisätä merkintöjä.
image-mark-up-wait = Odota, että muokkaus valmistuu, ja lisää sitten merkinnät.
image-crop-needs-selection = Vedä ensin valinta (Valitse-työkalu) ja rajaa sitten.
image-size-needed = Anna leveys ja korkeus pikseleinä.
image-cannot-save-format = Tiedoston ”{ $name }” muutoksia ei voi tallentaa sen muodossa. Käytä Vie-toimintoa ({ $keys }).
image-cannot-save-format-unbound = Tiedoston ”{ $name }” muutoksia ei voi tallentaa sen muodossa. Käytä Vie-toimintoa.
image-cannot-export-animation = Animaatioita ei voi vielä viedä.
image-drop-pages = Sivuja voi pudottaa asiakirjaan.
image-drag-failed = Vetämisen aloittaminen epäonnistui.
image-picture-save-failed = Kuvan tallentaminen Lataukset-kansioon epäonnistui.
image-open-failed = prev ei voi avata tätä kuvaa
image-opening = Avataan…
image-name-mismatch-title = Nimi ei vastaa tiedostomuotoa
image-name-mismatch = ”{ $name }” tallennetaan { $format }-tiedostona, mutta nimen pääte on .{ $extension }. Muut sovellukset eivät ehkä pysty avaamaan sitä.
image-name-mismatch-no-extension = ”{ $name }” tallennetaan { $format }-tiedostona, mutta nimessä ei ole päätettä. Muut sovellukset eivät ehkä pysty avaamaan sitä.
image-choose-again = Valitse uudelleen
image-save-as-is = Tallenna sellaisenaan
image-dimensions = { $width } × { $height }
image-frame-position = ruutu { $current }/{ $total }
image-position = { $current }/{ $total }
image-edited = muokattu
image-sidebar = Sivupalkki
image-zoom-out = Loitonna
image-zoom-in = Lähennä
image-zoom = { $percent } %
image-fit = Sovita ikkunaan
image-actual-size = Todellinen koko
image-undo = Kumoa
image-redo = Tee uudelleen
image-rotate-left = Kierrä vasemmalle
image-rotate-right = Kierrä oikealle
image-flip-horizontal = Peilaa vaakasuunnassa
image-flip-vertical = Peilaa pystysuunnassa
image-select = Suorakulmainen valinta
image-crop = Rajaa valintaan
image-adjust-size-tool = Säädä kokoa
image-adjust-color-tool = Säädä värejä
image-inspector = Tiedot
image-markup = Merkinnät
image-export = Vie
image-settings = Asetukset
image-adjust-color = Säädä värejä
image-adjust-size = Säädä kokoa
image-exposure = Valotus
image-contrast = Kontrasti
image-saturation = Kylläisyys
image-temperature = Lämpötila
image-tint = Sävy
image-sepia = Seepia
image-sharpness = Terävyys
image-levels = Tasot
image-black-point = Mustapiste
image-midtones = Keskisävyt
image-white-point = Valkopiste
image-reset-all = Palauta kaikki
image-current-size = Nykyinen koko: { $width } × { $height } pikseliä
image-width = Leveys
image-height = Korkeus
image-scale-proportionally = Skaalaa suhteellisesti
image-resize = Muuta kokoa
image-inspector-loading = Ladataan…
image-file = Tiedosto
image-format = Tiedostomuoto
image-dimensions-label = Mitat
image-pixels = { $width } × { $height } pikseliä
image-no-camera = Ei kameratietoja.
image-location = Sijainti
image-remove-location = Poista sijaintitiedot
image-no-location = Ei sijaintitietoja.
image-keywords-description = Avainsanat ja kuvaus
image-keywords-hint = Avainsanat pilkuilla erotettuina
image-description = Kuvaus
image-keywords-unsupported = Avainsanoja voi tallentaa JPEG-, PNG- ja WebP-tiedostoihin.
image-revert-to = Palauta versioon
image-no-versions = Ei aiempia versioita.
image-revert = Palauta
image-size-kb = { $size } kt
image-size-mb = { $size } Mt
image-close-title = Suljetaanko viemättä merkintöjä?
image-close-body = { $count ->
    [one] Kuvan merkinnät säilyvät vain niin kauan kuin sen ikkuna on auki. Säilytä ne viemällä kuva: merkinnät piirretään tallentamaasi kopioon.
   *[other] Kuvien merkinnät säilyvät vain niin kauan kuin niiden ikkuna on auki. Säilytä ne viemällä jokainen kuva: merkinnät piirretään tallentamaasi kopioon.
}
image-close-anyway = Sulje silti

## Markdown

markdown-reading-stopped = lukeminen keskeytyi
markdown-read-failed = prev ei voi lukea tätä tiedostoa
markdown-draw-failed = Asiakirjan piirtäminen epäonnistui
markdown-export-size = Koko asiakirja, { $width } × { $height } pikseliä
markdown-not-found = Ei löytynyt
markdown-match = { $current }/{ $total }
markdown-search = Etsi
markdown-smaller-text = Pienempi teksti
markdown-larger-text = Suurempi teksti
markdown-zoom = { $percent } %
markdown-actual-size = Todellinen koko
markdown-inspector = Tiedot
markdown-export = Vie
markdown-settings = Asetukset
markdown-file = Tiedosto
markdown-document = Asiakirja
markdown-words = Sanat
markdown-lines = Rivit
markdown-pictures = Kuvat

## Image details

image-meta-camera = Kamera
image-meta-exposure = Valotus
image-meta-image = Kuva
image-meta-make = Valmistaja
image-meta-model = Malli
image-meta-lens = Objektiivi
image-meta-exposure-time = Valotusaika
image-meta-f-number = Aukko
image-meta-iso = ISO
image-meta-focal-length = Polttoväli
image-meta-exposure-bias = Valotuksen korjaus
image-meta-flash = Salama
image-meta-date-taken = Kuvauspäivä
image-meta-orientation = Suunta
image-meta-color-space = Väriavaruus
image-meta-software = Ohjelmisto
image-meta-artist = Kuvaaja
image-meta-copyright = Tekijänoikeus
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normaali
    [2] Peilattu vaakasuunnassa
    [3] Kierretty 180°
    [4] Peilattu pystysuunnassa
    [5] Peilattu vaakasuunnassa, kierretty 90° vastapäivään
    [6] Kierretty 90° myötäpäivään
    [7] Peilattu vaakasuunnassa, kierretty 90° myötäpäivään
    [8] Kierretty 90° vastapäivään
   *[other] Tuntematon ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Laukesi
   *[no] Ei lauennut
}{ $mode ->
    [on] , pakotettu päälle
    [off] , pois
    [auto] , automaattinen
   *[unknown] {""}
}{ $redeye ->
    [yes] , punasilmäisyyden vähennys
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Kalibroimaton
   *[other] Muu ({ $code })
}

## Errors

error-pdf-open = asiakirjaa ei voi avata: { $detail }
error-pdf-page-out-of-range = sivua { $page } ei ole
error-pdf-password-protected = asiakirja on suojattu salasanalla; avaa se ensin ja kopioi sitten sen sivut
error-pdf-no-pages = ei poimittavia sivuja
error-pdf-crop-outside = rajausalue on sivun ulkopuolella
error-pdf-closed = asiakirja on suljettu
error-pdf-saved-unreadable = tallennettu asiakirja ei enää avaudu
error-image-read = tiedostoa ei voi lukea: { $detail }
error-image-invalid = kuva on vioittunut tai virheellinen: { $detail }
error-image-missing-library = tämän muodon avaamiseen tarvitaan { $library }, jota ei ole asennettu
error-image-unsupported = { $format }-kuvia ei vielä tueta
error-image-encode = kuvan koodaaminen epäonnistui: { $detail }
error-exif-malformed = EXIF-tiedot ovat virheellisiä
error-settings-read = asetuksia ei voi lukea: { $detail }
error-settings-invalid = virheelliset asetukset: { $detail }
error-remove-location = sijainnin poistaminen epäonnistui: { $error }
error-location-unsupported = sijaintitiedot voi poistaa JPEG-, PNG-, WebP- ja TIFF-tiedostoista
error-xmp-unsupported = avainsanoja ja kuvauksia voi tallentaa vain JPEG-, PNG- ja WebP-tiedostoihin

## Formats

format-camera-raw = Kameran RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = Tietoja: prev
menu-settings = Asetukset…
menu-services = Palvelut
menu-hide = Kätke prev
menu-hide-others = Kätke muut
menu-show-all = Näytä kaikki
menu-quit = Lopeta prev
menu-file = Arkisto
menu-open = Avaa…
menu-close = Sulje ikkuna
menu-export = Vie…
menu-print = Tulosta…
menu-edit = Muokkaa
menu-undo = Kumoa
menu-redo = Tee uudelleen
menu-cut = Leikkaa
menu-copy = Kopioi
menu-paste = Sijoita
menu-select-all = Valitse kaikki
menu-find = Etsi
menu-find-next = Etsi seuraava
menu-find-previous = Etsi edellinen
menu-view = Näytä
menu-hide-sidebar = Kätke sivupalkki
menu-thumbnails = Pienoiskuvat
menu-contents = Sisällysluettelo
menu-notes = Korostukset ja muistiinpanot
menu-bookmarks = Kirjanmerkit
menu-zoom-in = Lähennä
menu-zoom-out = Loitonna
menu-actual-size = Todellinen koko
menu-zoom-to-fit = Sovita ikkunaan
menu-inspector = Näytä tiedot
menu-slideshow = Diaesitys
menu-full-screen = Siirry koko näytön tilaan
menu-go = Siirry
menu-next-page = Seuraava sivu
menu-previous-page = Edellinen sivu
menu-go-to-page = Siirry sivulle…
menu-bookmark = Lisää kirjanmerkki
menu-tools = Työkalut
menu-markup = Näytä merkintätyökalupalkki
menu-rotate-left = Kierrä vasemmalle
menu-rotate-right = Kierrä oikealle
menu-crop = Rajaa
menu-adjust-color = Säädä värejä…
menu-window = Ikkuna
menu-minimize = Pienennä
menu-zoom = Zoomaa
menu-bring-all-to-front = Tuo kaikki eteen

## Outside control

settings-outside-control = Ulkoinen ohjaus
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = Yleiset
settings-tab-agents = Agentit
settings-allow-outside-control = Salli ulkoinen ohjaus
settings-allow-outside-control-note = Tekoälyagentit, kuten Claude Code, voivat lukea ja muuttaa tiedostojasi previssä komennon prev --mcp kautta. prev kysyy ennen jokaista uutta agenttia.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Sallitut: { $agents }
settings-forget-agents = Unohda
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Saako { $agent } ohjata previä?
agent-prompt-body = { $agent } pyytää käyttää previn ulkoista ohjausta avoimien tiedostojesi lukemiseen ja muuttamiseen. Voit poistaa ulkoisen ohjauksen käytöstä Asetuksissa.
agent-prompt-allow = Salli
agent-prompt-deny = Älä salli
settings-ask-before-note = Kysy ennen kuin agentti:
settings-ask-reading = Lukee tiedoston
settings-ask-viewing = Muuttaa näkymää tai ikkunaa
settings-ask-marking-up = Lisää tiedostoon merkintöjä
settings-ask-editing = Muokkaa tiedostoa
settings-ask-signing = Allekirjoittaa tiedoston
settings-ask-redacting = Toteuttaa mustaukset
settings-ask-exporting = Vie tiedoston
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Saako { $agent } lukea tämän tiedoston?
agent-ask-view = Saako { $agent } muuttaa näkymää?
agent-ask-markup = Saako { $agent } lisätä tähän tiedostoon merkintöjä?
agent-ask-edit = Saako { $agent } muokata tätä tiedostoa?
agent-ask-sign = Saako { $agent } allekirjoittaa tämän tiedoston?
agent-ask-redact = Saako { $agent } toteuttaa mustaukset?
agent-ask-export = Saako { $agent } viedä tämän tiedoston?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } pyytää käyttää toimintoa ”{ $tool }”. Asetuksissa valitset, mistä prev kysyy.
agent-ask-final = Tätä ei voi kumota.
