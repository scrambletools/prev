# prev's interface text in Czech (Čeština), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = anotace (panel Anotace), note = poznámka,
# highlight = zvýraznění, annotation = anotace (in a PDF the markup is stored as its
# annotations, so the two share one word on purpose), redact/redaction =
# začernit/začernění, redaction mark = značka,
# inspector = inspektor, zoom in/out = přiblížit/oddálit, bookmark = záložka,
# page = stránka, export = exportovat, crop = oříznout. Buttons and menu items use the infinitive (Uložit, Zrušit, Zavřít).

## Language

language-name = Čeština

## Common

common-cancel = Zrušit
common-close = Zavřít
common-save = Uložit

## Settings

settings-title = Nastavení
settings-appearance = Vzhled
settings-colors = Barvy
settings-windows = Okna
settings-default-app = Výchozí aplikace
settings-default-app-label = Otevírat soubory v prev
settings-default-app-note = Nastavit prev jako aplikaci, která otevírá soubory PDF, obrázky, kresby SVG a soubory Markdown.
settings-default-app-note-windows = Windows umožňují vybrat výchozí aplikace jen ve vlastním Nastavení. Tímto tam otevřete stránku aplikace prev.
settings-default-app-note-macos = macOS vás požádá o potvrzení každého typu: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP a AVIF.
settings-default-app-status = V prev se otevírá { $set } z { $total } typů souborů.
settings-default-app-button = Nastavit jako výchozí
settings-default-app-button-windows = Otevřít Nastavení
settings-default-app-no-entry = Soubor .desktop aplikace prev není nainstalovaný, proto v ní systém nemůže otevírat soubory. Nainstalujte prev z balíčku nebo pomocí scripts/install.sh.
settings-default-app-no-bundle = Chcete-li prev nastavit jako výchozí, spusťte jej z prev.app.
settings-default-app-failed = prev se nepodařilo nastavit jako výchozí: { $error }
settings-storage = Úložiště
settings-version = prev { $version }
settings-version-development = prev { $version } (vývojové sestavení)

## Markup toolbar

markup-tool-select = Vybrat
markup-tool-area = Obdélníkový výběr
markup-tool-sketch = Skica
markup-tool-draw = Kreslit
markup-tool-shapes = Tvary
markup-tool-text-box = Textové pole
markup-tool-highlight = Zvýraznit
markup-tool-note = Poznámka
markup-tool-sign = Podepsat
markup-tool-redact = Začernit
markup-apply = Použít
markup-apply-redactions = Použít začernění
markup-shape-style = Styl tvaru
markup-border-color = Barva okraje
markup-fill-color = Barva výplně
markup-text-style = Styl textu
markup-delete = Smazat
markup-undo = Zpět
markup-redo = Znovu

## Markup menus

markup-shape-rectangle = Obdélník
markup-shape-rounded-rectangle = Zaoblený obdélník
markup-shape-oval = Ovál
markup-shape-line = Čára
markup-shape-arrow = Šipka
markup-shape-star = Hvězda
markup-shape-polygon = Mnohoúhelník
markup-shape-speech-bubble = Bublina
markup-shape-loupe = Lupa
markup-shape-mask = Maska
markup-style-highlight = Zvýraznění
markup-style-underline = Podtržení
markup-style-strikethrough = Přeškrtnutí
markup-style-squiggly = Vlnovka
markup-menu-color = Barva
markup-menu-font = Písmo
markup-menu-size = Velikost
markup-menu-alignment = Zarovnání
markup-line-width = { $width } b.
markup-dashed = Přerušovaná

## Notes

markup-kind-note = Poznámka
markup-kind-text-box = Textové pole
markup-kind-stamp = Razítko
markup-kind-redaction = Začernění
markup-kind-shape = Tvar
markup-note-delete = Smazat poznámku
markup-note-done = Hotovo
markup-note-placeholder = Napište poznámku
markup-notes-empty = Žádná zvýraznění ani poznámky
markup-notes-empty-hint = Zde se zobrazí zvýraznění, poznámky a textová pole.
markup-notes-page = Stránka { $page }

## Markup errors

markup-change-failed = Dokument se nepodařilo změnit: { $error }
markup-copy-area-failed = Oblast se nepodařilo zkopírovat: { $error }
markup-document-closed = dokument byl zavřen
markup-render-area-failed = oblast se nepodařilo vykreslit
markup-copy-stopped = kopírování bylo přerušeno

## Signatures

signature-menu-empty = Zatím žádné podpisy.
signature-delete = Smazat podpis
signature-create = Vytvořit podpis…
signature-dialog-title = Vytvořit podpis
signature-tab-draw = Kreslit
signature-tab-type = Napsat
signature-tab-image = Obrázek
signature-draw-hint = Podepište se na čáru myší, perem nebo touchpadem.
signature-your-name = Vaše jméno
signature-image-hint = Vyberte fotku nebo sken svého podpisu na bílém papíře.
signature-choose-image = Vybrat obrázek…
signature-description = Popis, např. Celé jméno nebo Iniciály
signature-clear = Vymazat
signature-ink = Inkoust
signature-thickness = Tloušťka
signature-sign-first = Nejprve se podepište, pak uložte.
signature-default-name = Podpis { $number }
signature-change-failed = Podpisy se nepodařilo změnit: { $error }
signature-no-data-folder = chybí složka pro data: proměnná HOME není nastavena
signature-removing-stopped = odstraňování bylo přerušeno
signature-saving-stopped = ukládání bylo přerušeno
signature-reading-stopped = čtení bylo přerušeno
signature-not-an-image = tento soubor není obrázek, který prev umí přečíst
signature-no-frames = obrázek nemá žádné snímky
signature-not-found = v obrázku nebyl nalezen žádný podpis

## Dragging

drag-pages-need-document = Stránky lze přetáhnout do dokumentu.
drag-image-unsupported = prev tento obrázek neumí otevřít.
drag-area-failed = Oblast se nepodařilo přetáhnout: { $error }
drag-pages-failed = Stránky se nepodařilo přetáhnout: { $error }
drag-start-failed = Přetahování se nepodařilo zahájit.
drag-file-pages = Stránky
drag-file-one-page = { $name } (stránka { $page })
drag-file-page-range = { $name } (stránky { $first }–{ $last })
drag-file-image = Obrázek
drop-pdf-title = Přidat do tohoto dokumentu?
drop-pdf-body = Přidat soubor „{ $name }“ na konec tohoto dokumentu, nebo ho otevřít v samostatném okně?
drop-pdfs-body = { $count ->
    [one] Přidat { $count } soubor PDF na konec tohoto dokumentu, nebo ho otevřít v samostatném okně?
    [few] Přidat tyto { $count } soubory PDF na konec tohoto dokumentu, nebo je otevřít v samostatných oknech?
    [many] Přidat těchto { $count } souboru PDF na konec tohoto dokumentu, nebo je otevřít v samostatných oknech?
   *[other] Přidat těchto { $count } souborů PDF na konec tohoto dokumentu, nebo je otevřít v samostatných oknech?
}
drop-pdf-add = Přidat na konec
drop-pdf-open = Otevřít samostatně

## PDF window

pdf-opening = Otevírání…
pdf-open-failed = prev tento dokument neumí otevřít
pdf-no-pages = Dokument nemá žádné stránky.
pdf-document-closed = dokument byl zavřen
pdf-keep-original-failed = původní verzi se nepodařilo zachovat: { $error }
pdf-save-failed = Nepodařilo se uložit: { $error }
pdf-nothing-to-paste = Není co vložit.
pdf-pasting-stopped = vkládání bylo přerušeno
pdf-file-dialog-failed = Dialog souborů se nepodařilo zobrazit: { $error }
pdf-bookmarks-no-home = Záložky nelze uložit: proměnná HOME není nastavena
pdf-bookmarks-save-failed = Záložky se nepodařilo uložit: { $error }
pdf-bookmark-page = Stránka { $page }

pdf-password-protected = Soubor „{ $name }“ je chráněn heslem
pdf-password = Heslo
pdf-password-wrong = Nesprávné heslo. Zkuste to znovu.
pdf-unlock = Odemknout

pdf-sidebar = Postranní panel
pdf-page-of = z { $count }
pdf-zoom-out = Oddálit
pdf-zoom-in = Přiblížit
pdf-zoom-percent = { $percent } %
pdf-fit-page = Přizpůsobit stránce
pdf-fit-width = Přizpůsobit šířce
pdf-actual-size = Skutečná velikost
pdf-view-continuous = Plynulé posouvání
pdf-view-single-page = Jedna stránka
pdf-view-two-pages = Dvě stránky
pdf-undo = Zpět
pdf-redo = Znovu
pdf-rotate-left = Otočit doleva
pdf-rotate-right = Otočit doprava
pdf-inspector = Inspektor
pdf-markup = Anotace
pdf-export = Exportovat
pdf-settings = Nastavení

pdf-search = Hledat
pdf-search-not-found = Nenalezeno
pdf-searching = Hledání…
pdf-search-match = { $current } z { $total }
pdf-search-match-more = { $current } z { $total }+

pdf-inspector-file = Soubor
pdf-inspector-document = Dokument
pdf-inspector-pages = Stránky
pdf-inspector-title = Název
pdf-inspector-author = Autor
pdf-inspector-subject = Předmět
pdf-inspector-keywords = Klíčová slova
pdf-inspector-created = Vytvořeno
pdf-inspector-modified = Změněno
pdf-inspector-application = Aplikace
pdf-inspector-producer = Tvůrce PDF
pdf-inspector-version = Verze
pdf-inspector-security = Zabezpečení
pdf-inspector-not-encrypted = Nešifrováno
pdf-inspector-encrypted = Šifrováno ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } stránka
    [few] { $count } stránky
    [many] { $count } stránky
   *[other] { $count } stránek
}
pdf-inspector-page-size = Velikost stránky
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } in)
pdf-loading = Načítání…

pdf-tab-pages = Stránky
pdf-tab-contents = Obsah
pdf-tab-notes = Zvýraznění a poznámky
pdf-tab-bookmarks = Záložky
pdf-no-outline = Žádný obsah
pdf-no-outline-detail = Tento dokument nemá osnovu.
pdf-no-bookmarks = Žádné záložky
pdf-no-bookmarks-detail = Stisknutím { $keys } přidáte stránku do záložek.
pdf-no-bookmarks-detail-unbound = Zde se zobrazí stránky se záložkou.
pdf-remove-bookmark = Odebrat záložku

## Page editing

pages-menu = Stránky
pages-insert-blank = Vložit prázdnou stránku
pages-insert-file = Vložit ze souboru…
pages-copy = { $count ->
    [one] Kopírovat stránku
    [few] Kopírovat stránky
    [many] Kopírovat stránky
   *[other] Kopírovat stránky
}
pages-paste = { $count ->
    [one] Vložit stránku
    [few] Vložit { $count } stránky
    [many] Vložit { $count } stránky
   *[other] Vložit { $count } stránek
}
pages-crop = Oříznout podle výběru
pages-select-all = Vybrat všechny stránky
pages-delete = { $count ->
    [one] Smazat stránku
    [few] Smazat stránky
    [many] Smazat stránky
   *[other] Smazat stránky
}
pages-apply-redactions = Použít začernění…
pages-no-copied = Nejsou zkopírované žádné stránky, které by šlo vložit.
pages-copied = { $count ->
    [one] Zkopírována { $count } stránka.
    [few] Zkopírovány { $count } stránky.
    [many] Zkopírováno { $count } stránky.
   *[other] Zkopírováno { $count } stránek.
}
pages-copy-failed = Stránky se nepodařilo zkopírovat: { $error }
pages-reading-stopped = čtení bylo přerušeno
pages-image-unreadable = není obrázek, který prev umí přečíst
pages-read-failed = Soubor se nepodařilo přečíst: { $error }
pages-at-least-one = Dokument musí mít alespoň jednu stránku.
pages-crop-needs-area = Nejprve vyberte oblast nástrojem Obdélníkový výběr.
pages-change-failed = Stránky se nepodařilo změnit: { $error }
pages-no-redactions = Nebylo co začernit.
pages-redactions-applied = { $count ->
    [one] Použito { $count } začernění.
    [few] Použita { $count } začernění.
    [many] Použito { $count } začernění.
   *[other] Použito { $count } začernění.
}
pages-forget-versions-failed = Starší verze se nepodařilo smazat: { $error }
pages-redact-title = Použít začernění?
pages-redact-body = { $count ->
    [one] Text, obrázky a kresby pod značkou budou z dokumentu trvale odstraněny a značka se změní v černý obdélník. Tuto akci nelze vrátit zpět a starší verze tohoto souboru, které prev uchovává, budou smazány.
    [few] Text, obrázky a kresby pod { $count } značkami budou z dokumentu trvale odstraněny a značky se změní v černé obdélníky. Tuto akci nelze vrátit zpět a starší verze tohoto souboru, které prev uchovává, budou smazány.
    [many] Text, obrázky a kresby pod { $count } značkami budou z dokumentu trvale odstraněny a značky se změní v černé obdélníky. Tuto akci nelze vrátit zpět a starší verze tohoto souboru, které prev uchovává, budou smazány.
   *[other] Text, obrázky a kresby pod { $count } značkami budou z dokumentu trvale odstraněny a značky se změní v černé obdélníky. Tuto akci nelze vrátit zpět a starší verze tohoto souboru, které prev uchovává, budou smazány.
}
pages-redact-apply = Použít

## PDF export

pages-export-title = Exportovat
pages-export-format = Formát
pages-export-reduce = Zmenšit velikost souboru (obrázky v rozlišení 150 dpi)
pages-export-flatten = Sloučit anotace a pole formulářů
pages-export-flatten-detail = Anotace a vyplněná pole se stanou součástí stránek a už je nebude možné upravit. Dosud nepoužitá začernění budou vynechána.
pages-export-encrypt = Zašifrovat heslem
pages-export-password = Heslo
pages-export-verify-password = Potvrdit heslo
pages-export-resolution = Rozlišení
pages-export-dpi = { $dpi } dpi
pages-export-quality = Kvalita
pages-export-quality-low = Nízká
pages-export-quality-medium = Střední
pages-export-quality-high = Vysoká
pages-export-quality-best = Nejlepší
pages-export-one-file = Všechny stránky budou v jednom souboru.
pages-export-file-per-page = Každá stránka se uloží jako samostatný soubor se zvoleným názvem a pořadovým číslem.
pages-export-selected-only = { $count ->
    [one] Jen vybraná stránka
    [few] Jen { $count } vybrané stránky
    [many] Jen { $count } vybrané stránky
   *[other] Jen { $count } vybraných stránek
}
pages-export-choose = Exportovat…
pages-export-no-password = Zadejte heslo.
pages-export-password-mismatch = Hesla se neshodují.
pages-export-file-name = { $name } (exportováno)
pages-export-untitled = dokument
pages-export-same-file = Exportujte do nového souboru; tento dokument se ukládá sám.
pages-export-exporting = Exportování souboru „{ $name }“…
pages-export-done = Soubor „{ $name }“ byl exportován.
pages-export-done-images = Exportované obrázky: { $count }.
pages-export-failed = Nepodařilo se exportovat: { $error }
pages-export-stopped = export byl přerušen

## Start window

app-start-hint = Otevřete nebo sem přetáhněte soubor PDF, obrázek, SVG nebo Markdown.
app-start-open = Otevřít…
app-title-dev = { $title } (vývoj)
app-viewer-missing = { $kind }: tento prohlížeč zatím není hotový.
app-cannot-open = prev tento typ souboru neumí otevřít.
app-cannot-read = prev tento soubor neumí přečíst: { $error }
app-kind-pdf = Dokument PDF
app-kind-image = Obrázek { $format }
app-kind-svg = Kresba SVG
app-kind-markdown = Dokument Markdown
app-file-dialog-failed = Dialog souborů se nepodařilo zobrazit: { $error }

## Actions

action-open = Otevřít
action-settings = Nastavení

## Toolbar

app-toolbar-keep-shown = Stále zobrazovat panel nástrojů
app-toolbar-auto-hide = Skrýt panel nástrojů, když ukazatel opustí okno
app-toolbar-more = Další

## File facts

app-fact-name = Název
app-fact-folder = Složka
app-fact-size = Velikost
app-fact-modified = Změněno
app-size-bytes = { $count ->
    [one] { $count } bajt
    [few] { $count } bajty
    [many] { $count } bajtu
   *[other] { $count } bajtů
}
app-size-kb = { $size } kB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Neplatný odkaz { $uri }: { $error }
app-link-open-failed = Nepodařilo se otevřít { $uri }: { $error }
app-paste-needs-wl-clipboard = pro vkládání obrázků nainstalujte wl-clipboard
app-copy-needs-wl-clipboard = pro kopírování obrázků nainstalujte wl-clipboard
app-copy-no-pixels = oblast nemá žádné pixely
app-copy-no-input = wl-copy nemá žádný vstup
app-copy-failed = wl-copy selhal
app-clipboard-open-failed = Schránku se nepodařilo otevřít: { $error }
app-copy-image-failed = Obrázek se nepodařilo zkopírovat: { $error }

## Printing

print-failed = Nepodařilo se tisknout: { $error }
print-stopped = Tisk byl přerušen
print-unavailable = Tisk zatím v tomto systému není k dispozici.
print-no-window = Nepodařilo se tisknout: chybí okno, nad kterým by se zobrazil dialog tisku
print-dialog-failed = Dialog tisku se nepodařilo zobrazit: { $error }
print-job-not-started = tiskárna úlohu nespustila
print-printer-stopped = tiskárna se zastavila

## File dialogs

dialog-open = Otevřít
dialog-filter-all = Všechny podporované soubory
dialog-filter-pdf = Dokumenty PDF
dialog-filter-images = Obrázky
dialog-filter-svg = Kresby SVG
dialog-filter-markdown = Soubory Markdown
dialog-choose-signatures = Vyberte složku s podpisy
dialog-choose-versions = Vyberte složku historie verzí
dialog-choose-bookmarks = Vyberte soubor záložek

## Command line

usage-help =
    Použití: prev [FILE]...

    Zobrazení a úpravy souborů PDF a obrázků. Soubory se otevírají v oknech
    spuštěné aplikace prev, která se v případě potřeby spustí.

    Možnosti:
      -h, --help     Zobrazí tuto nápovědu
      -V, --version  Zobrazí verzi

## Settings, continued

settings-language = Jazyk
settings-language-system = Podle systému: { $language }
settings-input-language = Jazyk zadávání
settings-input-language-system = Podle rozložení klávesnice
settings-input-language-note = Určuje, na které straně začíná prázdné textové pole. Napsaný text si zachovává vlastní směr.

settings-appearance-system = Podle systému
settings-appearance-light = Světlý
settings-appearance-dark = Tmavý
settings-omarchy-accent = Použít zvýrazňující barvu Omarchy
settings-omarchy-note = Barvy vycházejí ze zvýrazňující barvy motivu „{ $theme }“.
settings-omarchy-none = Není aktivní žádný motiv Omarchy.
settings-auto-hide = Skrýt panel nástrojů, když ukazatel opustí okno
settings-auto-hide-note = Panel nástrojů se vznáší nad dokumentem a zasune se, když je ukazatel mimo okno.
settings-animations = Animace
settings-animations-note = Vysouvací lišty a panely, zvětšující se dialogy a pružná tlačítka.
settings-animations-reduced = Vypnuto, dokud systém požaduje omezení pohybu.
settings-corner-radius = Zaoblení rohů
settings-corner-radius-note = Pro dialogy a plovoucí panel nástrojů.
settings-corner-radius-value = { $radius } px
settings-overlay = Průhlednost překryvu
settings-overlay-note = Nakolik stránka prosvítá plovoucím panelem nástrojů.
settings-overlay-value = { $percent } %
settings-storage-signatures = Složka podpisů
settings-storage-versions = Složka historie verzí
settings-storage-bookmarks = Soubor záložek
settings-storage-apply = Použít
settings-storage-choose = Vybrat…
settings-storage-note = Soubory uložené na původním místě tam zůstanou; chcete-li je dál používat, přesuňte je. Nastavení aplikace prev se ukládá do { $file }.
settings-save-failed = Nastavení se nepodařilo uložit: { $error }
settings-no-location = Chybí umístění nastavení: proměnná HOME není nastavena
settings-full-path = Použijte úplnou cestu, např. ~/Documents/prev.
settings-path-is-folder = { $path } je složka, ne soubor.
settings-folder-missing = Složka { $path } neexistuje. Nejprve ji vytvořte, nebo vyberte jinou.
settings-path-is-file = { $path } je soubor, ne složka.
settings-cannot-write = prev nemůže zapisovat do { $path }: { $error }.

## Export dialog

export-title = Exportovat
export-format = Formát
export-quality = Kvalita
export-size = Velikost
export-choose = Exportovat…
export-format-webp = WebP (bezeztrátový)
export-format-unknown = obrázek
export-quality-low = Nízká
export-quality-medium = Střední
export-quality-high = Vysoká
export-quality-best = Nejlepší
export-size-actual = Skutečná velikost
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } pixelů
export-dialog-failed = Dialog uložení se nepodařilo zobrazit: { $error }
export-done = Exportováno: { $path }
export-failed = Nepodařilo se exportovat: { $error }
export-stopped = export byl přerušen

## Image window

image-marked-no-edit = Obrázky s anotacemi nelze upravovat. Chcete-li anotace zachovat, obrázek exportujte, nebo anotace smažte a zavřete panel Anotace.

image-loading-stopped = načítání bylo přerušeno
image-reverting-stopped = obnovování bylo přerušeno
image-rendering-stopped = vykreslování bylo přerušeno
image-saving-stopped = ukládání bylo přerušeno
image-markup-stopped = anotování bylo přerušeno
image-no-version-store = Chybí místo pro ukládání verzí
image-revert-failed = Nepodařilo se obnovit: { $error }
image-read-failed = Nepodařilo se přečíst { $path }: { $error }
image-keep-original-failed = Původní verzi se nepodařilo zachovat: { $error }
image-save-failed = Nepodařilo se uložit { $path }: { $error }
image-markup-start-failed = Anotování se nepodařilo spustit: { $error }
image-cannot-edit = Animace a kresby SVG nelze upravovat.
image-cannot-mark-up = Animace a kresby SVG nelze anotovat.
image-mark-up-wait = Počkejte na dokončení úpravy a pak anotujte.
image-crop-needs-selection = Nejprve tažením vytvořte výběr (nástroj Vybrat), pak ořízněte.
image-size-needed = Zadejte šířku a výšku v pixelech.
image-cannot-save-format = Změny souboru „{ $name }“ nelze uložit v jeho formátu. Použijte export ({ $keys }).
image-cannot-save-format-unbound = Změny souboru „{ $name }“ nelze uložit v jeho formátu. Použijte export.
image-cannot-export-animation = Animace zatím nelze exportovat.
image-drop-pages = Stránky lze přetáhnout do dokumentu.
image-drag-failed = Přetahování se nepodařilo zahájit.
image-picture-save-failed = Obrázek se nepodařilo uložit do složky Stažené.
image-open-failed = prev tento obrázek neumí otevřít
image-opening = Otevírání…
image-name-mismatch-title = Název neodpovídá formátu
image-name-mismatch = Soubor „{ $name }“ bude uložen ve formátu { $format }, ale jeho název končí na .{ $extension }. Jiné aplikace ho nemusí otevřít.
image-name-mismatch-no-extension = Soubor „{ $name }“ bude uložen ve formátu { $format }, ale jeho název nemá příponu. Jiné aplikace ho nemusí otevřít.
image-choose-again = Vybrat znovu
image-save-as-is = Uložit takto
image-dimensions = { $width } × { $height }
image-frame-position = snímek { $current } z { $total }
image-position = { $current } z { $total }
image-edited = upraveno
image-sidebar = Postranní panel
image-zoom-out = Oddálit
image-zoom-in = Přiblížit
image-zoom = { $percent } %
image-fit = Přizpůsobit oknu
image-actual-size = Skutečná velikost
image-undo = Zpět
image-redo = Znovu
image-rotate-left = Otočit doleva
image-rotate-right = Otočit doprava
image-flip-horizontal = Převrátit vodorovně
image-flip-vertical = Převrátit svisle
image-select = Obdélníkový výběr
image-crop = Oříznout podle výběru
image-adjust-size-tool = Upravit velikost
image-adjust-color-tool = Upravit barvy
image-inspector = Inspektor
image-markup = Anotace
image-export = Exportovat
image-settings = Nastavení
image-adjust-color = Upravit barvy
image-adjust-size = Upravit velikost
image-exposure = Expozice
image-contrast = Kontrast
image-saturation = Sytost
image-temperature = Teplota
image-tint = Odstín
image-sepia = Sépie
image-sharpness = Ostrost
image-levels = Úrovně
image-black-point = Černý bod
image-midtones = Střední tóny
image-white-point = Bílý bod
image-reset-all = Obnovit vše
image-current-size = Aktuální velikost: { $width } × { $height } pixelů
image-width = Šířka
image-height = Výška
image-scale-proportionally = Měnit velikost proporcionálně
image-resize = Změnit velikost
image-inspector-loading = Načítání…
image-file = Soubor
image-format = Formát
image-dimensions-label = Rozměry
image-pixels = { $width } × { $height } pixelů
image-no-camera = Žádné informace o fotoaparátu.
image-location = Poloha
image-remove-location = Odstranit informace o poloze
image-no-location = Žádné informace o poloze.
image-keywords-description = Klíčová slova a popis
image-keywords-hint = Klíčová slova oddělená čárkami
image-description = Popis
image-keywords-unsupported = Klíčová slova lze uložit do souborů JPEG, PNG a WebP.
image-revert-to = Vrátit k verzi
image-no-versions = Žádné starší verze.
image-revert = Obnovit
image-size-kb = { $size } kB
image-size-mb = { $size } MB
image-close-title = Zavřít bez exportu anotací?
image-close-body = { $count ->
    [one] Anotace na obrázku trvají jen do zavření jeho okna. Chcete-li je zachovat, obrázek exportujte: anotace se vykreslí do uložené kopie.
    [few] Anotace na obrázcích trvají jen do zavření jejich oken. Chcete-li je zachovat, exportujte každý obrázek: anotace se vykreslí do uložené kopie.
    [many] Anotace na obrázcích trvají jen do zavření jejich oken. Chcete-li je zachovat, exportujte každý obrázek: anotace se vykreslí do uložené kopie.
   *[other] Anotace na obrázcích trvají jen do zavření jejich oken. Chcete-li je zachovat, exportujte každý obrázek: anotace se vykreslí do uložené kopie.
}
image-close-anyway = Přesto zavřít

## Markdown

markdown-reading-stopped = čtení bylo přerušeno
markdown-read-failed = prev tento soubor neumí přečíst
markdown-draw-failed = Dokument se nepodařilo vykreslit
markdown-export-size = Celý dokument, { $width } × { $height } pixelů
markdown-not-found = Nenalezeno
markdown-match = { $current } z { $total }
markdown-search = Hledat
markdown-smaller-text = Menší text
markdown-larger-text = Větší text
markdown-zoom = { $percent } %
markdown-actual-size = Skutečná velikost
markdown-inspector = Inspektor
markdown-export = Exportovat
markdown-settings = Nastavení
markdown-file = Soubor
markdown-document = Dokument
markdown-words = Slova
markdown-lines = Řádky
markdown-pictures = Obrázky

## Image details

image-meta-camera = Fotoaparát
image-meta-exposure = Expozice
image-meta-image = Obrázek
image-meta-make = Výrobce
image-meta-model = Model
image-meta-lens = Objektiv
image-meta-exposure-time = Expoziční čas
image-meta-f-number = Clonové číslo
image-meta-iso = ISO
image-meta-focal-length = Ohnisková vzdálenost
image-meta-exposure-bias = Korekce expozice
image-meta-flash = Blesk
image-meta-date-taken = Datum pořízení
image-meta-orientation = Orientace
image-meta-color-space = Barevný prostor
image-meta-software = Software
image-meta-artist = Autor
image-meta-copyright = Autorská práva
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normální
    [2] Zrcadleno vodorovně
    [3] Otočeno o 180°
    [4] Zrcadleno svisle
    [5] Zrcadleno vodorovně, otočeno o 90° proti směru hodinových ručiček
    [6] Otočeno o 90° po směru hodinových ručiček
    [7] Zrcadleno vodorovně, otočeno o 90° po směru hodinových ručiček
    [8] Otočeno o 90° proti směru hodinových ručiček
   *[other] Neznámá ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Blesk použit
   *[no] Blesk nepoužit
}{ $mode ->
    [on] , vynucený
    [off] , vypnutý
    [auto] , automatický
   *[unknown] {""}
}{ $redeye ->
    [yes] , redukce červených očí
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Nekalibrovaný
   *[other] Jiný ({ $code })
}

## Errors

error-pdf-open = dokument nelze otevřít: { $detail }
error-pdf-page-out-of-range = stránka { $page } neexistuje
error-pdf-password-protected = dokument je chráněn heslem; otevřete ho a zkopírujte místo toho jeho stránky
error-pdf-no-pages = žádné stránky k extrakci
error-pdf-crop-outside = oblast oříznutí leží mimo stránku
error-pdf-closed = dokument byl zavřen
error-pdf-saved-unreadable = uložený dokument už nejde otevřít
error-image-read = soubor nelze přečíst: { $detail }
error-image-invalid = obrázek je poškozený nebo neplatný: { $detail }
error-image-missing-library = otevření tohoto formátu vyžaduje knihovnu { $library }, která není nainstalována
error-image-unsupported = obrázky { $format } zatím nejsou podporovány
error-image-encode = obrázek nelze zakódovat: { $detail }
error-exif-malformed = data EXIF jsou poškozená
error-settings-read = nastavení nelze přečíst: { $detail }
error-settings-invalid = neplatné nastavení: { $detail }
error-remove-location = polohu se nepodařilo odstranit: { $error }
error-location-unsupported = informace o poloze lze odstranit ze souborů JPEG, PNG, WebP a TIFF
error-xmp-unsupported = klíčová slova a popisy lze uložit jen do souborů JPEG, PNG a WebP

## Formats

format-camera-raw = RAW z fotoaparátu

## The macOS menu bar, named as in macOS's own apps.
menu-about = O aplikaci prev
menu-settings = Nastavení…
menu-services = Služby
menu-hide = Skrýt prev
menu-hide-others = Skrýt ostatní
menu-show-all = Zobrazit vše
menu-quit = Ukončit prev
menu-file = Soubor
menu-open = Otevřít…
menu-close = Zavřít okno
menu-export = Exportovat…
menu-print = Tisknout…
menu-edit = Úpravy
menu-undo = Zpět
menu-redo = Znovu
menu-cut = Vyjmout
menu-copy = Kopírovat
menu-paste = Vložit
menu-select-all = Vybrat vše
menu-find = Najít
menu-find-next = Najít další
menu-find-previous = Najít předchozí
menu-view = Zobrazení
menu-hide-sidebar = Skrýt postranní panel
menu-thumbnails = Miniatury
menu-contents = Obsah
menu-notes = Zvýraznění a poznámky
menu-bookmarks = Záložky
menu-zoom-in = Přiblížit
menu-zoom-out = Oddálit
menu-actual-size = Skutečná velikost
menu-zoom-to-fit = Přizpůsobit oknu
menu-inspector = Zobrazit inspektor
menu-slideshow = Prezentace
menu-full-screen = Přejít do režimu celé obrazovky
menu-go = Přejít
menu-next-page = Další stránka
menu-previous-page = Předchozí stránka
menu-go-to-page = Přejít na stránku…
menu-bookmark = Přidat záložku
menu-tools = Nástroje
menu-markup = Zobrazit panel Anotace
menu-rotate-left = Otočit doleva
menu-rotate-right = Otočit doprava
menu-crop = Oříznout
menu-adjust-color = Upravit barvy…
menu-window = Okno
menu-minimize = Minimalizovat
menu-zoom = Zvětšit/zmenšit
menu-bring-all-to-front = Přenést vše do popředí
