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
language-name = 日本語

## Common

common-cancel = キャンセル
common-close = 閉じる
common-save = 保存

## Settings

settings-title = 設定
settings-appearance = 外観
settings-colors = カラー
settings-windows = ウインドウ
settings-storage = 保存場所

## Markup toolbar

markup-tool-select = 選択
markup-tool-area = 長方形で選択
markup-tool-sketch = スケッチ
markup-tool-draw = 描画
markup-tool-shapes = 図形
markup-tool-text-box = テキストボックス
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = ハイライト
markup-tool-note = メモ
# Opens the menu of saved signatures (a verb).
markup-tool-sign = 署名
# A verb: the tool that marks areas to black out.
markup-tool-redact = 墨消し
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = 適用
markup-apply-redactions = 墨消しを適用
markup-shape-style = 図形のスタイル
markup-border-color = 枠線の色
markup-fill-color = 塗りつぶしの色
markup-text-style = テキストのスタイル
markup-delete = 削除
markup-undo = 取り消す
markup-redo = やり直す

## Markup menus

markup-shape-rectangle = 長方形
markup-shape-rounded-rectangle = 角丸長方形
markup-shape-oval = 楕円
markup-shape-line = 線
markup-shape-arrow = 矢印
markup-shape-star = 星
markup-shape-polygon = 多角形
markup-shape-speech-bubble = 吹き出し
# A shape that magnifies the part of the page under it.
markup-shape-loupe = ルーペ
# A shape that darkens the page around it.
markup-shape-mask = マスク
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = ハイライト
markup-style-underline = 下線
markup-style-strikethrough = 取り消し線
markup-style-squiggly = 波線
# Menu section headings.
markup-menu-color = カラー
markup-menu-font = フォント
markup-menu-size = サイズ
markup-menu-alignment = 配置
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = 破線

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = メモ
markup-kind-text-box = テキストボックス
markup-kind-stamp = スタンプ
markup-kind-redaction = 墨消し
markup-kind-shape = 図形
# Tooltips on a note being edited.
markup-note-delete = メモを削除
markup-note-done = 完了
markup-note-placeholder = メモを入力
markup-notes-empty = ハイライトやメモはありません
markup-notes-empty-hint = ハイライト、メモ、テキストボックスがここに表示されます。
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = ページ { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = 書類を変更できませんでした：{ $error }
markup-copy-area-failed = 領域をコピーできませんでした：{ $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = 書類が閉じられました
markup-render-area-failed = 領域をレンダリングできませんでした
markup-copy-stopped = コピーが中断されました

## Signatures

signature-menu-empty = 署名はまだありません。
signature-delete = 署名を削除
signature-create = 署名を作成…
signature-dialog-title = 署名を作成
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = 手書き
signature-tab-type = 入力
signature-tab-image = 画像
signature-draw-hint = マウス、ペン、またはタッチパッドで線の上に署名してください。
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = 名前
signature-image-hint = 白い紙に書いた署名の写真またはスキャン画像を選択してください。
signature-choose-image = 画像を選択…
# Placeholder of the field naming the signature in the library.
signature-description = 説明（フルネーム、イニシャルなど）
# Clears the drawing, typed name or image.
signature-clear = 消去
# The color the signature is drawn or typed in.
signature-ink = インク
# The pen's width, for drawing.
signature-thickness = 太さ
signature-sign-first = 署名してから保存してください。
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = 署名 { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = 署名を変更できませんでした：{ $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = データフォルダがありません：HOMEが設定されていません
signature-removing-stopped = 削除が中断されました
signature-saving-stopped = 保存が中断されました
signature-reading-stopped = 読み込みが中断されました
signature-not-an-image = このファイルはprevで読み込める画像ではありません
signature-no-frames = 画像にフレームがありません
signature-not-found = 画像に署名が見つかりません

## Dragging

drag-pages-need-document = ページは書類にドロップできます。
drag-image-unsupported = prevではこの画像を開けません。
# $error is a lowercase reason or a technical message.
drag-area-failed = 領域をドラッグできませんでした：{ $error }
drag-pages-failed = ページをドラッグできませんでした：{ $error }
drag-start-failed = ドラッグを開始できませんでした。
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = ページ
drag-file-one-page = { $name }（ページ { $page }）
drag-file-page-range = { $name }（ページ { $first }–{ $last }）

## PDF window

pdf-opening = 開いています…
pdf-open-failed = prevではこの書類を開けません
pdf-no-pages = この書類にはページがありません。
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = 書類が閉じられました
pdf-keep-original-failed = 元のバージョンを保持できませんでした：{ $error }
pdf-save-failed = 保存できませんでした：{ $error }
pdf-nothing-to-paste = 貼り付けるものがありません。
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = 貼り付けが中断されました
pdf-file-dialog-failed = ファイルダイアログを表示できませんでした：{ $error }
pdf-bookmarks-no-home = ブックマークを保存できません：HOMEが設定されていません
pdf-bookmarks-save-failed = ブックマークを保存できませんでした：{ $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = ページ { $page }

# Password prompt. $name is the file name.
pdf-password-protected = 「{ $name }」はパスワードで保護されています
pdf-password = パスワード
pdf-password-wrong = パスワードが正しくありません。もう一度お試しください。
# Button that opens a locked document.
pdf-unlock = ロック解除

# Toolbar tooltips and labels.
pdf-sidebar = サイドバー
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = / { $count }
pdf-zoom-out = 縮小
pdf-zoom-in = 拡大
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = ページに合わせる
pdf-fit-width = 幅に合わせる
pdf-actual-size = 実際のサイズ
pdf-view-continuous = 連続スクロール
pdf-view-single-page = 単一ページ
pdf-view-two-pages = 2ページ
pdf-undo = 取り消す
pdf-redo = やり直す
pdf-rotate-left = 左に回転
pdf-rotate-right = 右に回転
pdf-inspector = インスペクタ
pdf-markup = マークアップ
# Tooltip of the button that opens the export dialog.
pdf-export = 書き出す
pdf-settings = 設定

# Search field.
pdf-search = 検索
pdf-search-not-found = 見つかりません
pdf-searching = 検索中…
# The match shown, of all matches found.
pdf-search-match = { $current }/{ $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current }/{ $total }+

# Inspector: section headings.
pdf-inspector-file = ファイル
pdf-inspector-document = 書類
pdf-inspector-pages = ページ
# Inspector: fact labels and values.
pdf-inspector-title = タイトル
pdf-inspector-author = 作成者
pdf-inspector-subject = 主題
pdf-inspector-keywords = キーワード
pdf-inspector-created = 作成日
pdf-inspector-modified = 変更日
pdf-inspector-application = アプリケーション
pdf-inspector-producer = PDF変換
pdf-inspector-version = バージョン
pdf-inspector-security = セキュリティ
pdf-inspector-not-encrypted = 暗号化なし
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = 暗号化あり（{ $method }）
pdf-inspector-page-count = { $count }ページ
pdf-inspector-page-size = ページサイズ
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm（{ $width_in } × { $height_in } in）
pdf-loading = 読み込み中…

# Sidebar tabs and lists.
pdf-tab-pages = ページ
pdf-tab-contents = 目次
pdf-tab-notes = ハイライトとメモ
pdf-tab-bookmarks = ブックマーク
pdf-no-outline = 目次がありません
pdf-no-outline-detail = この書類にはアウトラインがありません。
pdf-no-bookmarks = ブックマークはありません
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = { $keys }を押すとページをブックマークできます。
pdf-remove-bookmark = ブックマークを削除

## Page editing

# Tooltip of the Pages menu button.
pages-menu = ページ
pages-insert-blank = 空白ページを挿入
pages-insert-file = ファイルから挿入…
pages-copy = ページをコピー
# $count is the number of pages copied earlier.
pages-paste = { $count }ページを貼り付け
pages-crop = 選択範囲で切り取る
pages-select-all = すべてのページを選択
pages-delete = ページを削除
pages-apply-redactions = 墨消しを適用…
pages-no-copied = 貼り付けるコピー済みのページがありません。
pages-copied = { $count }ページをコピーしました。
pages-copy-failed = ページをコピーできませんでした：{ $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = 読み込みが中断されました
pages-read-failed = ファイルを読み込めませんでした：{ $error }
pages-at-least-one = 書類には少なくとも1ページが必要です。
pages-crop-needs-area = 先に長方形選択ツールで領域を選択してください。
pages-change-failed = ページを変更できませんでした：{ $error }
pages-no-redactions = 適用する墨消しはありませんでした。
pages-redactions-applied = { $count }か所の墨消しを適用しました。
pages-forget-versions-failed = 以前のバージョンを削除できませんでした：{ $error }
pages-redact-title = 墨消しを適用しますか？
pages-redact-body = { $count }か所のマークの下にあるテキスト、画像、描画は書類から完全に削除され、マークは黒い四角になります。この操作は取り消せません。また、prevが保持しているこのファイルの以前のバージョンも削除されます。
# Button that applies redactions.
pages-redact-apply = 適用

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = 書き出す
pages-export-format = フォーマット
pages-export-reduce = ファイルサイズを縮小（画像を150 dpiに）
pages-export-flatten = 注釈とフォームフィールドをフラット化
pages-export-flatten-detail = マークアップと入力済みのフィールドはページの一部になり、編集できなくなります。まだ適用していない墨消しマークは含まれません。
pages-export-encrypt = パスワードで暗号化
pages-export-password = パスワード
pages-export-verify-password = パスワードの確認
pages-export-resolution = 解像度
pages-export-dpi = { $dpi } dpi
pages-export-quality = 画質
# JPEG quality choices.
pages-export-quality-low = 低
pages-export-quality-medium = 中
pages-export-quality-high = 高
pages-export-quality-best = 最高
pages-export-one-file = すべてのページを1つのファイルにまとめます。
pages-export-file-per-page = 各ページを個別のファイルとして保存し、指定した名前のあとに番号を付けます。
pages-export-selected-only = 選択した{ $count }ページのみ
# Button that opens the file dialog to choose where to export.
pages-export-choose = 書き出す…
pages-export-no-password = パスワードを入力してください。
pages-export-password-mismatch = パスワードが一致しません。
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name }（書き出し）
# Suggested file name when the document's own name is unknown.
pages-export-untitled = 書類
pages-export-same-file = 新しいファイルに書き出してください。この書類は自動的に保存されます。
pages-export-exporting = 「{ $name }」を書き出しています…
pages-export-done = 「{ $name }」を書き出しました。
pages-export-done-images = { $count }枚の画像を書き出しました。
pages-export-failed = 書き出せませんでした：{ $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = 書き出しが中断されました

## Start window

# Under the app name in a window with no file open.
app-start-hint = PDF、画像、SVG、Markdownファイルを開くか、ドロップしてください。
app-start-open = 開く…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title }（開発版）
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }：このビューアはまだ作成されていません。
app-cannot-open = prevではこの種類のファイルを開けません。
app-cannot-read = prevではこのファイルを読み込めません：{ $error }
app-kind-pdf = PDF書類
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format }画像
app-kind-svg = SVGドローイング
app-kind-markdown = Markdown書類
app-file-dialog-failed = ファイルダイアログを表示できませんでした：{ $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = 開く
action-settings = 設定

## Toolbar

app-toolbar-keep-shown = ツールバーを常に表示
app-toolbar-auto-hide = ポインタが離れたらツールバーを隠す
# The button that shows the toolbar's hidden tools.
app-toolbar-more = その他

## File facts

# Labels in a file's inspector.
app-fact-name = 名前
app-fact-folder = フォルダ
app-fact-size = サイズ
app-fact-modified = 変更日
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count }バイト
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = リンク{ $uri }は無効です：{ $error }
app-link-open-failed = { $uri }を開けませんでした：{ $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = 画像を貼り付けるにはwl-clipboardをインストールしてください
app-copy-needs-wl-clipboard = 画像をコピーするにはwl-clipboardをインストールしてください
app-copy-no-pixels = 領域にピクセルがありません
# wl-copy is a program's name.
app-copy-no-input = wl-copyに入力がありません
app-copy-failed = wl-copyが失敗しました
app-clipboard-open-failed = クリップボードを開けませんでした：{ $error }
app-copy-image-failed = 画像をコピーできませんでした：{ $error }

## Printing

print-failed = プリントできませんでした：{ $error }
print-stopped = プリントが中断されました
print-unavailable = このシステムではまだプリントを利用できません。
print-no-window = プリントできませんでした：プリントダイアログを表示するウインドウがありません
print-dialog-failed = プリントダイアログを表示できませんでした：{ $error }
# Shown after "Could not print:".
print-job-not-started = プリンタがジョブを開始しませんでした
# Shown after "Could not print:".
print-printer-stopped = プリンタが停止しました

## File dialogs

dialog-open = 開く
dialog-filter-all = サポートされているすべてのファイル
dialog-filter-pdf = PDF書類
dialog-filter-images = 画像
dialog-filter-svg = SVGドローイング
dialog-filter-markdown = Markdown
dialog-choose-signatures = 署名フォルダを選択
dialog-choose-versions = バージョン履歴フォルダを選択
dialog-choose-bookmarks = ブックマークファイルを選択

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    使い方：prev [FILE]...

    PDFと画像を表示・編集します。ファイルは実行中のprevのウインドウで
    開きます。prevが起動していない場合は起動します。

    オプション：
      -h, --help     このヘルプを表示
      -V, --version  バージョンを表示

## Settings, continued

settings-language = 言語
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = システムのデフォルト：{ $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = 入力言語
settings-input-language-system = キーボードレイアウトに従う
settings-input-language-note = 空のテキストフィールドで入力を始める側を決めます。入力したテキストはそれぞれの方向を保ちます。

settings-appearance-system = システム
settings-appearance-light = ライト
settings-appearance-dark = ダーク
settings-omarchy-accent = Omarchyのアクセントカラーを使用
# $theme is the Omarchy theme's name.
settings-omarchy-note = 「{ $theme }」のアクセントからカラーを作成しています。
settings-omarchy-none = 有効なOmarchyテーマがありません。
settings-auto-hide = ポインタが離れたらツールバーを隠す
settings-auto-hide-note = ツールバーは書類の上に浮かび、ポインタがウインドウの外にあるあいだはスライドして隠れます。
settings-animations = アニメーション
settings-animations-note = バーやパネルのスライド、ダイアログの拡大、弾むボタン。
settings-animations-reduced = システムで動きを減らす設定がオンのあいだはオフになります。
settings-corner-radius = 角の半径
settings-corner-radius-note = ダイアログとフローティングツールバーに適用されます。
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = オーバーレイの透明度
settings-overlay-note = フローティングツールバー越しにページが透けて見える度合いです。
settings-overlay-value = { $percent }%
settings-storage-signatures = 署名フォルダ
settings-storage-versions = バージョン履歴フォルダ
settings-storage-bookmarks = ブックマークファイル
settings-storage-apply = 適用
settings-storage-choose = 選択…
# $file is where the settings file is.
settings-storage-note = 以前の場所に保存されているファイルはそのまま残ります。引き続き使うには移動してください。prevのアプリ設定は{ $file }に保存されます。
settings-save-failed = 設定を保存できませんでした：{ $error }
settings-no-location = 設定の保存場所がありません：HOMEが設定されていません
settings-full-path = ~/Documents/prevのようなフルパスを使用してください。
settings-path-is-folder = { $path }はフォルダです。ファイルではありません。
settings-folder-missing = { $path }というフォルダはありません。先に作成するか、フォルダを選択してください。
settings-path-is-file = { $path }はファイルです。フォルダではありません。
settings-cannot-write = prevは{ $path }に書き込めません：{ $error }。

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = 書き出す
# Section headings in the export dialog.
export-format = フォーマット
export-quality = 画質
export-size = サイズ
# Button that goes on to choose where to save the export.
export-choose = 書き出す…
# Format choice; the format name stays as it is.
export-format-webp = WebP（ロスレス）
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = 画像
# JPEG quality choices.
export-quality-low = 低
export-quality-medium = 中
export-quality-high = 高
export-quality-best = 最高
# Size choices: the picture at its own size, or scaled up.
export-size-actual = 実際のサイズ
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height }ピクセル
# $error is the system's reason.
export-dialog-failed = 保存ダイアログを表示できませんでした：{ $error }
# $path is where the file was saved.
export-done = { $path }を書き出しました
export-failed = 書き出せませんでした：{ $error }
# Shown if exporting ends unexpectedly.
export-stopped = 書き出しが中断されました

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = マークアップのある画像は編集できません。マークアップを残す場合は書き出してください。残さない場合はマークアップを削除して、マークアップバーを閉じてください。

# Shown if a background task ends unexpectedly.
image-loading-stopped = 読み込みが中断されました
image-reverting-stopped = 復元が中断されました
image-rendering-stopped = レンダリングが中断されました
image-saving-stopped = 保存が中断されました
image-markup-stopped = マークアップが中断されました
image-no-version-store = バージョンを保存する場所がありません
image-revert-failed = 復元できませんでした：{ $error }
image-read-failed = { $path }を読み込めませんでした：{ $error }
image-keep-original-failed = 元のバージョンを保持できませんでした：{ $error }
image-save-failed = { $path }を保存できませんでした：{ $error }
image-markup-start-failed = マークアップを開始できませんでした：{ $error }
image-cannot-edit = アニメーションとSVGドローイングは編集できません。
image-cannot-mark-up = アニメーションとSVGドローイングにはマークアップできません。
image-mark-up-wait = 編集が終わってからマークアップしてください。
image-crop-needs-selection = 先に（選択ツールで）選択範囲をドラッグしてから切り取ってください。
image-size-needed = 幅と高さをピクセルで入力してください。
# $name is a file name.
image-cannot-save-format = 「{ $name }」への変更はこのフォーマットでは保存できません。書き出し（{ $keys }）を使用してください。
image-cannot-export-animation = アニメーションはまだ書き出せません。
image-drop-pages = ページは書類にドロップできます。
image-drag-failed = ドラッグを開始できませんでした。
image-open-failed = prevではこの画像を開けません
image-opening = 開いています…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = 名前がフォーマットと一致しません
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = 「{ $name }」は{ $format }ファイルとして保存されますが、名前の末尾が.{ $extension }です。ほかのアプリで開けない場合があります。
image-name-mismatch-no-extension = 「{ $name }」は{ $format }ファイルとして保存されますが、名前に拡張子がありません。ほかのアプリで開けない場合があります。
image-choose-again = 選び直す
image-save-as-is = このまま保存
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = フレーム { $current }/{ $total }
image-position = { $current }/{ $total }
image-edited = 編集済み
# Toolbar tooltips.
image-sidebar = サイドバー
image-zoom-out = 縮小
image-zoom-in = 拡大
image-zoom = { $percent }%
image-fit = ウインドウに合わせる
image-actual-size = 実際のサイズ
image-undo = 取り消す
image-redo = やり直す
image-rotate-left = 左に回転
image-rotate-right = 右に回転
image-flip-horizontal = 左右反転
image-flip-vertical = 上下反転
image-select = 長方形で選択
image-crop = 選択範囲で切り取る
image-adjust-size-tool = サイズを調整
image-adjust-color-tool = カラーを調整
# Tooltip and panel title.
image-inspector = インスペクタ
image-markup = マークアップ
image-export = 書き出す
image-settings = 設定
# Panel titles.
image-adjust-color = カラーを調整
image-adjust-size = サイズを調整
# Adjust Color sliders.
image-exposure = 露出
image-contrast = コントラスト
image-saturation = 彩度
image-temperature = 色温度
image-tint = 色合い
image-sepia = セピア
image-sharpness = シャープネス
image-levels = レベル
image-black-point = ブラックポイント
image-midtones = 中間調
image-white-point = ホワイトポイント
image-reset-all = すべてリセット
# Adjust Size panel.
image-current-size = 現在のサイズ：{ $width } × { $height }ピクセル
image-width = 幅
image-height = 高さ
image-scale-proportionally = 比例して拡大／縮小
# Button that applies the new size.
image-resize = サイズを変更
# Inspector panel.
image-inspector-loading = 読み込み中…
image-file = ファイル
image-format = フォーマット
image-dimensions-label = 寸法
image-pixels = { $width } × { $height }ピクセル
image-no-camera = カメラ情報はありません。
image-location = 位置情報
image-remove-location = 位置情報を削除
image-no-location = 位置情報はありません。
image-keywords-description = キーワードと説明
image-keywords-hint = キーワード（カンマ区切り）
image-description = 説明
image-keywords-unsupported = キーワードはJPEG、PNG、WebPファイルに保存できます。
# Heading over the earlier versions of the file.
image-revert-to = バージョンを戻す
image-no-versions = 以前のバージョンはありません。
image-revert = 復元
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = マークアップを書き出さずに閉じますか？
image-close-body = 画像のマークアップはウインドウが開いているあいだだけ保持されます。残すには画像を書き出してください。保存するコピーにマークアップが描き込まれます。
image-close-anyway = このまま閉じる

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = 読み込みが中断されました
markdown-read-failed = prevではこのファイルを読み込めません
markdown-draw-failed = 書類を描画できませんでした
# Under the export's size choices.
markdown-export-size = 書類全体、{ $width } × { $height }ピクセル
# Search results.
markdown-not-found = 見つかりません
markdown-match = { $current }/{ $total }
# Placeholder of the search field.
markdown-search = 検索
# Toolbar tooltips.
markdown-smaller-text = 文字を小さく
markdown-larger-text = 文字を大きく
markdown-zoom = { $percent }%
markdown-actual-size = 実際のサイズ
# Tooltip and panel title.
markdown-inspector = インスペクタ
markdown-export = 書き出す
markdown-settings = 設定
# Inspector headings and labels.
markdown-file = ファイル
markdown-document = 書類
markdown-words = 単語数
markdown-lines = 行数
markdown-pictures = 画像

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = カメラ
image-meta-exposure = 露出
image-meta-image = 画像
image-meta-make = メーカー
image-meta-model = モデル
image-meta-lens = レンズ
image-meta-exposure-time = 露出時間
# The lens aperture, written like f/2.8.
image-meta-f-number = F値
image-meta-iso = ISO
image-meta-focal-length = 焦点距離
image-meta-exposure-bias = 露出補正
image-meta-flash = フラッシュ
image-meta-date-taken = 撮影日時
image-meta-orientation = 向き
image-meta-color-space = 色空間
image-meta-software = ソフトウェア
image-meta-artist = 作者
image-meta-copyright = 著作権
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] 標準
    [2] 左右反転
    [3] 180°回転
    [4] 上下反転
    [5] 左右反転、反時計回りに90°回転
    [6] 時計回りに90°回転
    [7] 左右反転、時計回りに90°回転
    [8] 反時計回りに90°回転
   *[other] 不明（{ $value }）
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] 発光
   *[no] 発光なし
}{ $mode ->
    [on] 、強制発光
    [off] 、オフ
    [auto] 、自動
   *[unknown] {""}
}{ $redeye ->
    [yes] 、赤目軽減
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] キャリブレーションなし
   *[other] その他（{ $code }）
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = 書類を開けません：{ $detail }
error-pdf-page-out-of-range = ページ { $page }は存在しません
error-pdf-password-protected = 書類はパスワードで保護されています。書類を開いてからページをコピーしてください
error-pdf-no-pages = 抽出するページがありません
error-pdf-crop-outside = 切り取り範囲がページの外にあります
error-pdf-closed = 書類が閉じられました
error-pdf-saved-unreadable = 保存した書類が開けなくなりました
error-image-read = ファイルを読み込めません：{ $detail }
error-image-invalid = 画像が破損しているか無効です：{ $detail }
# $library is a program name, such as libheif.
error-image-missing-library = このフォーマットを開くには{ $library }が必要ですが、インストールされていません
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format }画像にはまだ対応していません
error-image-encode = 画像をエンコードできません：{ $detail }
error-exif-malformed = EXIFデータの形式が正しくありません
error-settings-read = 設定を読み込めません：{ $detail }
error-settings-invalid = 設定が無効です：{ $detail }
error-remove-location = 位置情報を削除できませんでした：{ $error }
error-location-unsupported = 位置情報を削除できるのはJPEG、PNG、WebP、TIFFファイルです
error-xmp-unsupported = キーワードと説明はJPEG、PNG、WebPファイルにのみ保存できます

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = カメラRAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prevについて
menu-settings = 設定…
menu-services = サービス
menu-hide = prevを非表示
menu-hide-others = ほかを非表示
menu-show-all = すべてを表示
menu-quit = prevを終了
menu-file = ファイル
menu-open = 開く…
menu-close = ウインドウを閉じる
menu-export = 書き出す…
menu-print = プリント…
menu-edit = 編集
menu-undo = 取り消す
menu-redo = やり直す
menu-cut = カット
menu-copy = コピー
menu-paste = ペースト
menu-select-all = すべてを選択
menu-find = 検索
menu-find-next = 次を検索
menu-find-previous = 前を検索
menu-view = 表示
menu-hide-sidebar = サイドバーを非表示
menu-thumbnails = サムネール
menu-contents = 目次
menu-notes = ハイライトとメモ
menu-bookmarks = ブックマーク
menu-zoom-in = 拡大
menu-zoom-out = 縮小
menu-actual-size = 実際のサイズ
menu-zoom-to-fit = ウインドウサイズに合わせる
menu-inspector = インスペクタを表示
menu-slideshow = スライドショー
menu-full-screen = フルスクリーンにする
menu-go = 移動
menu-next-page = 次のページ
menu-previous-page = 前のページ
menu-go-to-page = ページに移動…
menu-bookmark = ブックマークを追加
menu-tools = ツール
menu-markup = マークアップツールバーを表示
menu-rotate-left = 左に回転
menu-rotate-right = 右に回転
menu-crop = 切り取る
menu-adjust-color = カラーを調整…
menu-window = ウインドウ
menu-minimize = しまう
menu-zoom = 拡大／縮小
menu-bring-all-to-front = すべてを手前に移動
