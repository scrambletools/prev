# prev's interface text in English, the source for every other language.
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
language-name = 繁體中文

## Common

common-cancel = 取消
common-close = 關閉
common-save = 儲存

## Settings

settings-title = 設定
settings-appearance = 外觀
settings-colors = 顏色
settings-windows = 視窗
settings-default-app = 預設 App
settings-default-app-label = 使用 prev 開啟檔案
settings-default-app-note = 讓 prev 成為開啟 PDF、影像、SVG 繪圖和 Markdown 檔案的 App。
settings-default-app-note-windows = Windows 只允許在其自身的「設定」中選擇預設 App。此按鈕會在那裡開啟 prev 的頁面。
settings-default-app-note-macos = macOS 會要求逐一確認每種類型：PDF、PNG、JPEG、HEIC、GIF、TIFF、WebP 和 AVIF。
settings-default-app-status = { $total } 種檔案類型中有 { $set } 種使用 prev 開啟。
settings-default-app-button = 設為預設
settings-default-app-button-windows = 開啟設定
settings-default-app-no-entry = prev 的桌面項目尚未安裝，因此系統無法用它開啟檔案。請透過套件或 scripts/install.sh 安裝 prev。
settings-default-app-no-bundle = 請從 prev.app 開啟 prev，才能將其設為預設。
settings-default-app-failed = 無法將 prev 設為預設：{ $error }
settings-storage = 儲存位置
settings-version = prev { $version }
settings-version-development = prev { $version }（開發版）

## Markup toolbar

markup-tool-select = 選取
markup-tool-area = 矩形選取
markup-tool-sketch = 素描
markup-tool-draw = 繪圖
markup-tool-shapes = 形狀
markup-tool-text-box = 文字框
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = 標示重點
markup-tool-note = 附註
# Opens the menu of saved signatures (a verb).
markup-tool-sign = 簽名
# A verb: the tool that marks areas to black out.
markup-tool-redact = 塗黑
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = 套用
markup-apply-redactions = 套用塗黑
markup-shape-style = 形狀樣式
markup-border-color = 邊框顏色
markup-fill-color = 填滿顏色
markup-text-style = 文字樣式
markup-delete = 刪除
markup-undo = 還原
markup-redo = 重做

## Markup menus

markup-shape-rectangle = 矩形
markup-shape-rounded-rectangle = 圓角矩形
markup-shape-oval = 橢圓形
markup-shape-line = 線條
markup-shape-arrow = 箭頭
markup-shape-star = 星形
markup-shape-polygon = 多邊形
markup-shape-speech-bubble = 對話泡泡
# A shape that magnifies the part of the page under it.
markup-shape-loupe = 放大鏡
# A shape that darkens the page around it.
markup-shape-mask = 遮罩
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = 標示重點
markup-style-underline = 底線
markup-style-strikethrough = 刪除線
markup-style-squiggly = 波浪線
# Menu section headings.
markup-menu-color = 顏色
markup-menu-font = 字體
markup-menu-size = 大小
markup-menu-alignment = 對齊
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = 虛線

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = 附註
markup-kind-text-box = 文字框
markup-kind-stamp = 圖章
markup-kind-redaction = 塗黑
markup-kind-shape = 形狀
# Tooltips on a note being edited.
markup-note-delete = 刪除附註
markup-note-done = 完成
markup-note-placeholder = 輸入附註
markup-notes-empty = 沒有重點標示或附註
markup-notes-empty-hint = 重點標示、附註和文字框會顯示在這裡。
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = 第 { $page } 頁

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = 無法更改文件：{ $error }
markup-copy-area-failed = 無法複製該區域：{ $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = 文件已關閉
markup-render-area-failed = 無法繪製該區域
markup-copy-stopped = 複製已中斷

## Signatures

signature-menu-empty = 尚無簽名。
signature-delete = 刪除簽名
signature-create = 製作簽名…
signature-dialog-title = 製作簽名
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = 手寫
signature-tab-type = 鍵入
signature-tab-image = 影像
signature-draw-hint = 使用滑鼠、觸控筆或觸控板在線上簽名。
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = 你的姓名
signature-image-hint = 選擇一張寫在白紙上的簽名照片或掃描檔。
signature-choose-image = 選擇影像…
# Placeholder of the field naming the signature in the library.
signature-description = 描述，例如全名或姓名縮寫
# Clears the drawing, typed name or image.
signature-clear = 清除
# The color the signature is drawn or typed in.
signature-ink = 墨水
# The pen's width, for drawing.
signature-thickness = 粗細
signature-sign-first = 請先簽名，再儲存。
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = 簽名 { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = 無法更改簽名：{ $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = 沒有資料夾可存放資料：未設定 HOME
signature-removing-stopped = 移除已中斷
signature-saving-stopped = 儲存已中斷
signature-reading-stopped = 讀取已中斷
signature-not-an-image = 該檔案不是 prev 可讀取的影像
signature-no-frames = 該影像沒有影格
signature-not-found = 影像中找不到簽名

## Dragging

drag-pages-need-document = 頁面可以拖放到文件上。
drag-image-unsupported = prev 無法開啟此影像。
# $error is a lowercase reason or a technical message.
drag-area-failed = 無法拖移該區域：{ $error }
drag-pages-failed = 無法拖移頁面：{ $error }
drag-start-failed = 無法開始拖移。
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = 頁面
drag-file-one-page = { $name }（第 { $page } 頁）
drag-file-page-range = { $name }（第 { $first }–{ $last } 頁）
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = 影像
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = 要加入此文件嗎？
drop-pdf-body = 要將「{ $name }」加到此文件的結尾，還是在獨立的視窗中開啟？
drop-pdfs-body = 要將這 { $count } 個 PDF 加到此文件的結尾，還是分別在獨立的視窗中開啟？
drop-pdf-add = 加到結尾
drop-pdf-open = 單獨開啟

## PDF window

pdf-opening = 正在開啟…
pdf-open-failed = prev 無法開啟此文件
pdf-no-pages = 此文件沒有頁面。
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = 文件已關閉
pdf-keep-original-failed = 無法保留原始版本：{ $error }
pdf-save-failed = 無法儲存：{ $error }
pdf-nothing-to-paste = 沒有可貼上的內容。
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = 貼上已中斷
pdf-file-dialog-failed = 無法顯示檔案對話框：{ $error }
pdf-bookmarks-no-home = 無法儲存書籤：未設定 HOME
pdf-bookmarks-save-failed = 無法儲存書籤：{ $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = 第 { $page } 頁

# Password prompt. $name is the file name.
pdf-password-protected = 「{ $name }」受密碼保護
pdf-password = 密碼
pdf-password-wrong = 密碼不正確，請再試一次。
# Button that opens a locked document.
pdf-unlock = 解鎖

# Toolbar tooltips and labels.
pdf-sidebar = 側邊欄
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = / { $count }
pdf-zoom-out = 縮小
pdf-zoom-in = 放大
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = 符合頁面
pdf-fit-width = 符合寬度
pdf-actual-size = 實際大小
pdf-view-continuous = 連續捲動
pdf-view-single-page = 單頁
pdf-view-two-pages = 雙頁
pdf-undo = 還原
pdf-redo = 重做
pdf-rotate-left = 向左旋轉
pdf-rotate-right = 向右旋轉
pdf-inspector = 檢閱器
pdf-markup = 標示
# Tooltip of the button that opens the export dialog.
pdf-export = 輸出
pdf-settings = 設定

# Search field.
pdf-search = 搜尋
pdf-search-not-found = 找不到
pdf-searching = 正在搜尋…
# The match shown, of all matches found.
pdf-search-match = { $current } / { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } / { $total }+

# Inspector: section headings.
pdf-inspector-file = 檔案
pdf-inspector-document = 文件
pdf-inspector-pages = 頁面
# Inspector: fact labels and values.
pdf-inspector-title = 標題
pdf-inspector-author = 作者
pdf-inspector-subject = 主旨
pdf-inspector-keywords = 關鍵字
pdf-inspector-created = 建立日期
pdf-inspector-modified = 修改日期
pdf-inspector-application = 應用程式
pdf-inspector-producer = PDF 製作程式
pdf-inspector-version = 版本
pdf-inspector-security = 安全性
pdf-inspector-not-encrypted = 未加密
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = 已加密（{ $method }）
pdf-inspector-page-count = { $count } 頁
pdf-inspector-page-size = 頁面大小
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm（{ $width_in } × { $height_in } in）
pdf-loading = 正在載入…

# Sidebar tabs and lists.
pdf-tab-pages = 頁面
pdf-tab-contents = 目錄
pdf-tab-notes = 重點標示與附註
pdf-tab-bookmarks = 書籤
pdf-no-outline = 沒有目錄
pdf-no-outline-detail = 此文件沒有大綱。
pdf-no-bookmarks = 沒有書籤
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = 按下 { $keys } 即可將頁面加入書籤。
pdf-no-bookmarks-detail-unbound = 加入書籤的頁面會顯示在這裡。
pdf-remove-bookmark = 移除書籤

## Page editing

# Tooltip of the Pages menu button.
pages-menu = 頁面
pages-insert-blank = 插入空白頁面
pages-insert-file = 從檔案插入…
pages-copy = 複製頁面
# $count is the number of pages copied earlier.
pages-paste = 貼上 { $count } 個頁面
pages-crop = 裁切為選取範圍
pages-select-all = 選取所有頁面
pages-delete = 刪除頁面
pages-apply-redactions = 套用塗黑…
pages-no-copied = 沒有可貼上的已複製頁面。
pages-copied = 已複製 { $count } 個頁面。
pages-copy-failed = 無法複製頁面：{ $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = 讀取已中斷
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = 不是 prev 可讀取的影像
pages-read-failed = 無法讀取檔案：{ $error }
pages-at-least-one = 文件至少需要一個頁面。
pages-crop-needs-area = 請先使用矩形選取工具選取一個區域。
pages-change-failed = 無法更改頁面：{ $error }
pages-no-redactions = 沒有可套用的塗黑。
pages-redactions-applied = 已套用 { $count } 處塗黑。
pages-forget-versions-failed = 無法刪除先前的版本：{ $error }
pages-redact-title = 要套用塗黑嗎？
pages-redact-body = { $count } 處標記下的文字、影像和繪圖將從文件中永久移除，標記會變成黑色方塊。此動作無法還原，而且 prev 保留的此檔案先前版本也會被刪除。
# Button that applies redactions.
pages-redact-apply = 套用

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = 輸出
pages-export-format = 格式
pages-export-reduce = 縮減檔案大小（影像為 150 dpi）
pages-export-flatten = 平面化註解和表單欄位
pages-export-flatten-detail = 標示和已填寫的欄位會成為頁面的一部分，無法再編輯。尚未套用的塗黑標記將不會包含在內。
pages-export-encrypt = 使用密碼加密
pages-export-password = 密碼
pages-export-verify-password = 確認密碼
pages-export-resolution = 解析度
pages-export-dpi = { $dpi } dpi
pages-export-quality = 品質
# JPEG quality choices.
pages-export-quality-low = 低
pages-export-quality-medium = 中
pages-export-quality-high = 高
pages-export-quality-best = 最佳
pages-export-one-file = 所有頁面會輸出成一個檔案。
pages-export-file-per-page = 每個頁面會儲存為獨立的檔案，並在你選擇的名稱後加上編號。
pages-export-selected-only = 僅所選的 { $count } 頁
# Button that opens the file dialog to choose where to export.
pages-export-choose = 輸出…
pages-export-no-password = 請輸入密碼。
pages-export-password-mismatch = 密碼不相符。
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name }（已輸出）
# Suggested file name when the document's own name is unknown.
pages-export-untitled = 文件
pages-export-same-file = 請輸出至新檔案；此文件會自動儲存。
pages-export-exporting = 正在輸出「{ $name }」…
pages-export-done = 已輸出「{ $name }」。
pages-export-done-images = 已輸出 { $count } 張影像。
pages-export-failed = 無法輸出：{ $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = 輸出已中斷

## Start window

# Under the app name in a window with no file open.
app-start-hint = 開啟或拖入 PDF、影像、SVG 或 Markdown 檔案。
app-start-open = 開啟…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title }（開發版）
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }：此檢視器尚未完成。
app-cannot-open = prev 無法開啟此類檔案。
app-cannot-read = prev 無法讀取此檔案：{ $error }
app-kind-pdf = PDF 文件
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format } 影像
app-kind-svg = SVG 繪圖
app-kind-markdown = Markdown 文件
app-file-dialog-failed = 無法顯示檔案對話框：{ $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = 開啟
action-settings = 設定

## Toolbar

app-toolbar-keep-shown = 永遠顯示工具列
app-toolbar-auto-hide = 指標離開時隱藏工具列
# The button that shows the toolbar's hidden tools.
app-toolbar-more = 更多

## File facts

# Labels in a file's inspector.
app-fact-name = 名稱
app-fact-folder = 資料夾
app-fact-size = 大小
app-fact-modified = 修改日期
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } 位元組
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = 無效的連結 { $uri }：{ $error }
app-link-open-failed = 無法開啟 { $uri }：{ $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = 請安裝 wl-clipboard 以貼上影像
app-copy-needs-wl-clipboard = 請安裝 wl-clipboard 以複製影像
app-copy-no-pixels = 該區域沒有像素
# wl-copy is a program's name.
app-copy-no-input = wl-copy 沒有輸入內容
app-copy-failed = wl-copy 失敗
app-clipboard-open-failed = 無法開啟剪貼簿：{ $error }
app-copy-image-failed = 無法複製影像：{ $error }

## Printing

print-failed = 無法列印：{ $error }
print-stopped = 列印已中斷
print-unavailable = 此系統尚不支援列印。
print-no-window = 無法列印：沒有可顯示列印對話框的視窗
print-dialog-failed = 無法顯示列印對話框：{ $error }
# Shown after "Could not print:".
print-job-not-started = 印表機未開始列印工作
# Shown after "Could not print:".
print-printer-stopped = 印表機已停止

## File dialogs

dialog-open = 開啟
dialog-filter-all = 所有支援的檔案
dialog-filter-pdf = PDF 文件
dialog-filter-images = 影像
dialog-filter-svg = SVG 繪圖
dialog-filter-markdown = Markdown
dialog-choose-signatures = 選擇簽名資料夾
dialog-choose-versions = 選擇版本記錄資料夾
dialog-choose-bookmarks = 選擇書籤檔案

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    用法：prev [FILE]...

    檢視和編輯 PDF 與影像。檔案會在執行中的 prev 視窗內開啟，
    必要時會先啟動 prev。

    選項：
      -h, --help     顯示此說明
      -V, --version  顯示版本

## Settings, continued

settings-language = 語言
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = 系統預設：{ $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = 輸入語言
settings-input-language-system = 依照鍵盤配置
settings-input-language-note = 決定空白文字欄位從哪一側開始。你輸入的文字會保有其本身的方向。

settings-appearance-system = 系統
settings-appearance-light = 淺色
settings-appearance-dark = 深色
settings-omarchy-accent = 使用 Omarchy 強調色
# $theme is the Omarchy theme's name.
settings-omarchy-note = 顏色是根據「{ $theme }」的強調色產生。
settings-omarchy-none = 目前沒有啟用的 Omarchy 主題。
settings-auto-hide = 指標離開時隱藏工具列
settings-auto-hide-note = 工具列會浮在文件上方，指標移到視窗外時會滑出隱藏。
settings-animations = 動畫
settings-animations-note = 滑動的列與面板、展開的對話框和有彈性的按鈕。
settings-animations-reduced = 系統要求減少動態效果時會關閉。
settings-corner-radius = 圓角半徑
settings-corner-radius-note = 用於對話框和浮動工具列。
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = 覆疊透明度
settings-overlay-note = 透過浮動工具列能看到多少頁面內容。
settings-overlay-value = { $percent }%
settings-storage-signatures = 簽名資料夾
settings-storage-versions = 版本記錄資料夾
settings-storage-bookmarks = 書籤檔案
settings-storage-apply = 套用
settings-storage-choose = 選擇…
# $file is where the settings file is.
settings-storage-note = 已存放在舊位置的檔案會留在原處；若要繼續使用，請將它們搬移過去。prev 的 App 設定儲存在 { $file }。
settings-save-failed = 無法儲存設定：{ $error }
settings-no-location = 沒有存放設定的位置：未設定 HOME
settings-full-path = 請使用完整路徑，例如 ~/Documents/prev。
settings-path-is-folder = { $path } 是資料夾，不是檔案。
settings-folder-missing = 資料夾 { $path } 不存在。請先建立它，或選擇一個資料夾。
settings-path-is-file = { $path } 是檔案，不是資料夾。
settings-cannot-write = prev 無法寫入 { $path }：{ $error }。

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = 輸出
# Section headings in the export dialog.
export-format = 格式
export-quality = 品質
export-size = 大小
# Button that goes on to choose where to save the export.
export-choose = 輸出…
# Format choice; the format name stays as it is.
export-format-webp = WebP（無損）
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = 影像
# JPEG quality choices.
export-quality-low = 低
export-quality-medium = 中
export-quality-high = 高
export-quality-best = 最佳
# Size choices: the picture at its own size, or scaled up.
export-size-actual = 實際大小
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } 像素
# $error is the system's reason.
export-dialog-failed = 無法顯示儲存對話框：{ $error }
# $path is where the file was saved.
export-done = 已輸出 { $path }
export-failed = 無法輸出：{ $error }
# Shown if exporting ends unexpectedly.
export-stopped = 輸出已中斷

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = 含有標示的影像無法編輯。請輸出以保留標示，或刪除標示並關閉標示列。

# Shown if a background task ends unexpectedly.
image-loading-stopped = 載入已中斷
image-reverting-stopped = 回復已中斷
image-rendering-stopped = 繪製已中斷
image-saving-stopped = 儲存已中斷
image-markup-stopped = 標示已中斷
image-no-version-store = 沒有可保留版本的位置
image-revert-failed = 無法回復：{ $error }
image-read-failed = 無法讀取 { $path }：{ $error }
image-keep-original-failed = 無法保留原始版本：{ $error }
image-save-failed = 無法儲存 { $path }：{ $error }
image-markup-start-failed = 無法開始標示：{ $error }
image-cannot-edit = 動畫和 SVG 繪圖無法編輯。
image-cannot-mark-up = 動畫和 SVG 繪圖無法加上標示。
image-mark-up-wait = 請等編輯完成後再加上標示。
image-crop-needs-selection = 請先拖出一個選取範圍（選取工具），再進行裁切。
image-size-needed = 請輸入以像素為單位的寬度和高度。
# $name is a file name.
image-cannot-save-format = 對「{ $name }」的更改無法以其格式儲存。請使用輸出（{ $keys }）。
image-cannot-save-format-unbound = 對「{ $name }」的更改無法以其格式儲存。請使用輸出。
image-cannot-export-animation = 目前尚無法輸出動畫。
image-drop-pages = 頁面可以拖放到文件上。
image-drag-failed = 無法開始拖移。
image-picture-save-failed = 無法將影像儲存到「下載」資料夾。
image-open-failed = prev 無法開啟此影像
image-opening = 正在開啟…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = 名稱與格式不符
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = 「{ $name }」將儲存為 { $format } 檔案，但其名稱結尾為 .{ $extension }。其他 App 可能無法開啟。
image-name-mismatch-no-extension = 「{ $name }」將儲存為 { $format } 檔案，但其名稱沒有副檔名。其他 App 可能無法開啟。
image-choose-again = 重新選擇
image-save-as-is = 照原樣儲存
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = 影格 { $current } / { $total }
image-position = { $current } / { $total }
image-edited = 已編輯
# Toolbar tooltips.
image-sidebar = 側邊欄
image-zoom-out = 縮小
image-zoom-in = 放大
image-zoom = { $percent }%
image-fit = 符合視窗
image-actual-size = 實際大小
image-undo = 還原
image-redo = 重做
image-rotate-left = 向左旋轉
image-rotate-right = 向右旋轉
image-flip-horizontal = 水平翻轉
image-flip-vertical = 垂直翻轉
image-select = 矩形選取
image-crop = 裁切為選取範圍
image-adjust-size-tool = 調整大小
image-adjust-color-tool = 調整顏色
# Tooltip and panel title.
image-inspector = 檢閱器
image-markup = 標示
image-export = 輸出
image-settings = 設定
# Panel titles.
image-adjust-color = 調整顏色
image-adjust-size = 調整大小
# Adjust Color sliders.
image-exposure = 曝光
image-contrast = 對比
image-saturation = 飽和度
image-temperature = 色溫
image-tint = 色調
image-sepia = 棕褐色
image-sharpness = 銳利度
image-levels = 色階
image-black-point = 黑點
image-midtones = 中間調
image-white-point = 白點
image-reset-all = 全部重置
# Adjust Size panel.
image-current-size = 目前大小：{ $width } × { $height } 像素
image-width = 寬度
image-height = 高度
image-scale-proportionally = 等比例縮放
# Button that applies the new size.
image-resize = 調整大小
# Inspector panel.
image-inspector-loading = 正在載入…
image-file = 檔案
image-format = 格式
image-dimensions-label = 尺寸
image-pixels = { $width } × { $height } 像素
image-no-camera = 沒有相機資訊。
image-location = 位置
image-remove-location = 移除位置資訊
image-no-location = 沒有位置資訊。
image-keywords-description = 關鍵字與描述
image-keywords-hint = 關鍵字，以逗號分隔
image-description = 描述
image-keywords-unsupported = 關鍵字可儲存在 JPEG、PNG 和 WebP 檔案中。
# Heading over the earlier versions of the file.
image-revert-to = 回復成
image-no-versions = 沒有先前的版本。
image-revert = 回復
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = 要關閉而不輸出標示嗎？
image-close-body = 影像上的標示只會在其視窗開啟期間保留。若要保留標示，請輸出影像：標示會繪製到你儲存的副本中。
image-close-anyway = 仍要關閉

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = 讀取已中斷
markdown-read-failed = prev 無法讀取此檔案
markdown-draw-failed = 無法繪製文件
# Under the export's size choices.
markdown-export-size = 整份文件，{ $width } × { $height } 像素
# Search results.
markdown-not-found = 找不到
markdown-match = { $current } / { $total }
# Placeholder of the search field.
markdown-search = 搜尋
# Toolbar tooltips.
markdown-smaller-text = 縮小文字
markdown-larger-text = 放大文字
markdown-zoom = { $percent }%
markdown-actual-size = 實際大小
# Tooltip and panel title.
markdown-inspector = 檢閱器
markdown-export = 輸出
markdown-settings = 設定
# Inspector headings and labels.
markdown-file = 檔案
markdown-document = 文件
markdown-words = 字數
markdown-lines = 行數
markdown-pictures = 圖片

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = 相機
image-meta-exposure = 曝光
image-meta-image = 影像
image-meta-make = 製造商
image-meta-model = 型號
image-meta-lens = 鏡頭
image-meta-exposure-time = 曝光時間
# The lens aperture, written like f/2.8.
image-meta-f-number = 光圈值
image-meta-iso = ISO
image-meta-focal-length = 焦距
image-meta-exposure-bias = 曝光補償
image-meta-flash = 閃光燈
image-meta-date-taken = 拍攝日期
image-meta-orientation = 方向
image-meta-color-space = 色彩空間
image-meta-software = 軟體
image-meta-artist = 作者
image-meta-copyright = 版權
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] 正常
    [2] 水平鏡像
    [3] 旋轉 180°
    [4] 垂直鏡像
    [5] 水平鏡像，逆時針旋轉 90°
    [6] 順時針旋轉 90°
    [7] 水平鏡像，順時針旋轉 90°
    [8] 逆時針旋轉 90°
   *[other] 未知（{ $value }）
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] 已閃光
   *[no] 未閃光
}{ $mode ->
    [on] ，強制閃光
    [off] ，關閉
    [auto] ，自動
   *[unknown] {""}
}{ $redeye ->
    [yes] ，防紅眼
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] 未校準
   *[other] 其他（{ $code }）
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = 無法開啟文件：{ $detail }
error-pdf-page-out-of-range = 第 { $page } 頁不存在
error-pdf-password-protected = 文件受密碼保護；請先開啟文件，再複製其頁面
error-pdf-no-pages = 沒有可擷取的頁面
error-pdf-crop-outside = 裁切區域位於頁面之外
error-pdf-closed = 文件已關閉
error-pdf-saved-unreadable = 儲存的文件已無法開啟
error-image-read = 無法讀取檔案：{ $detail }
error-image-invalid = 影像已損毀或無效：{ $detail }
# $library is a program name, such as libheif.
error-image-missing-library = 開啟此格式需要 { $library }，但尚未安裝
# $format is an image format name, such as HEIC.
error-image-unsupported = 尚不支援 { $format } 影像
error-image-encode = 無法編碼影像：{ $detail }
error-exif-malformed = EXIF 資料格式錯誤
error-settings-read = 無法讀取設定：{ $detail }
error-settings-invalid = 設定無效：{ $detail }
error-remove-location = 無法移除位置：{ $error }
error-location-unsupported = 只能從 JPEG、PNG、WebP 和 TIFF 檔案移除位置資訊
error-xmp-unsupported = 關鍵字和描述只能儲存在 JPEG、PNG 和 WebP 檔案中

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = 相機 RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = 關於 prev
menu-settings = 設定…
menu-services = 服務
menu-hide = 隱藏 prev
menu-hide-others = 隱藏其他
menu-show-all = 顯示全部
menu-quit = 結束 prev
menu-file = 檔案
menu-open = 打開…
menu-close = 關閉視窗
menu-export = 輸出…
menu-print = 列印…
menu-edit = 編輯
menu-undo = 還原
menu-redo = 重做
menu-cut = 剪下
menu-copy = 拷貝
menu-paste = 貼上
menu-select-all = 全選
menu-find = 尋找
menu-find-next = 尋找下一個
menu-find-previous = 尋找上一個
menu-view = 顯示方式
menu-hide-sidebar = 隱藏側邊欄
menu-thumbnails = 縮覽圖
menu-contents = 目錄
menu-notes = 重點標示與附註
menu-bookmarks = 書籤
menu-zoom-in = 放大
menu-zoom-out = 縮小
menu-actual-size = 實際大小
menu-zoom-to-fit = 縮放至符合大小
menu-inspector = 顯示檢閱器
menu-slideshow = 幻燈片秀
menu-full-screen = 進入全螢幕
menu-go = 前往
menu-next-page = 下一頁
menu-previous-page = 上一頁
menu-go-to-page = 前往頁面…
menu-bookmark = 加入書籤
menu-tools = 工具
menu-markup = 顯示標示工具列
menu-rotate-left = 向左旋轉
menu-rotate-right = 向右旋轉
menu-crop = 裁切
menu-adjust-color = 調整顏色…
menu-window = 視窗
menu-minimize = 縮到最小
menu-zoom = 縮放
menu-bring-all-to-front = 將此程式所有視窗移至最前
