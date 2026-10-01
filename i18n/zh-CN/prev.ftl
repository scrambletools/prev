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
language-name = 简体中文

## Common

common-cancel = 取消
common-close = 关闭
common-save = 保存

## Settings

settings-title = 设置
settings-appearance = 外观
settings-colors = 颜色
settings-windows = 窗口
settings-default-app = 默认应用
settings-default-app-label = 用 prev 打开文件
settings-default-app-note = 让 prev 成为打开 PDF、图像、SVG 图形和 Markdown 文件的应用。
settings-default-app-note-windows = Windows 只允许在其自身的“设置”中选择默认应用。此按钮会在那里打开 prev 的页面。
settings-default-app-note-macos = macOS 会要求逐一确认每种类型：PDF、PNG、JPEG、HEIC、GIF、TIFF、WebP 和 AVIF。
settings-default-app-status = { $total } 种文件类型中有 { $set } 种用 prev 打开。
settings-default-app-button = 设为默认
settings-default-app-button-windows = 打开设置
settings-default-app-no-entry = prev 的桌面条目未安装，因此系统无法用它打开文件。请通过软件包或 scripts/install.sh 安装 prev。
settings-default-app-no-bundle = 请从 prev.app 打开 prev，才能将其设为默认。
settings-default-app-failed = 无法将 prev 设为默认：{ $error }
settings-storage = 存储位置
settings-version = prev { $version }
settings-version-development = prev { $version }（开发版）

## Markup toolbar

markup-tool-select = 选择
markup-tool-area = 矩形选择
markup-tool-sketch = 草绘
markup-tool-draw = 绘制
markup-tool-shapes = 形状
markup-tool-text-box = 文本框
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = 高亮
markup-tool-note = 备注
# Opens the menu of saved signatures (a verb).
markup-tool-sign = 签名
# A verb: the tool that marks areas to black out.
markup-tool-redact = 涂黑
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = 应用
markup-apply-redactions = 应用涂黑
markup-shape-style = 形状样式
markup-border-color = 边框颜色
markup-fill-color = 填充颜色
markup-text-style = 文本样式
markup-delete = 删除
markup-undo = 撤销
markup-redo = 重做

## Markup menus

markup-shape-rectangle = 矩形
markup-shape-rounded-rectangle = 圆角矩形
markup-shape-oval = 椭圆
markup-shape-line = 直线
markup-shape-arrow = 箭头
markup-shape-star = 星形
markup-shape-polygon = 多边形
markup-shape-speech-bubble = 对话气泡
# A shape that magnifies the part of the page under it.
markup-shape-loupe = 放大镜
# A shape that darkens the page around it.
markup-shape-mask = 遮罩
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = 高亮
markup-style-underline = 下划线
markup-style-strikethrough = 删除线
markup-style-squiggly = 波浪线
# Menu section headings.
markup-menu-color = 颜色
markup-menu-font = 字体
markup-menu-size = 大小
markup-menu-alignment = 对齐
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = 虚线

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = 备注
markup-kind-text-box = 文本框
markup-kind-stamp = 图章
markup-kind-redaction = 涂黑
markup-kind-shape = 形状
# Tooltips on a note being edited.
markup-note-delete = 删除备注
markup-note-done = 完成
markup-note-placeholder = 输入备注
markup-notes-empty = 没有高亮或备注
markup-notes-empty-hint = 高亮、备注和文本框会显示在这里。
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = 第 { $page } 页

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = 无法更改文档：{ $error }
markup-copy-area-failed = 无法复制该区域：{ $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = 文档已关闭
markup-render-area-failed = 无法渲染该区域
markup-copy-stopped = 复制已中断

## Signatures

signature-menu-empty = 还没有签名。
signature-delete = 删除签名
signature-create = 创建签名…
signature-dialog-title = 创建签名
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = 手写
signature-tab-type = 键入
signature-tab-image = 图像
signature-draw-hint = 使用鼠标、手写笔或触控板在线上签名。
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = 你的姓名
signature-image-hint = 选择一张写在白纸上的签名照片或扫描件。
signature-choose-image = 选择图像…
# Placeholder of the field naming the signature in the library.
signature-description = 描述，例如全名或姓名首字母
# Clears the drawing, typed name or image.
signature-clear = 清除
# The color the signature is drawn or typed in.
signature-ink = 墨水
# The pen's width, for drawing.
signature-thickness = 粗细
signature-sign-first = 请先签名，然后保存。
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = 签名 { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = 无法更改签名：{ $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = 没有数据文件夹：未设置 HOME
signature-removing-stopped = 移除已中断
signature-saving-stopped = 保存已中断
signature-reading-stopped = 读取已中断
signature-not-an-image = 该文件不是 prev 可以读取的图像
signature-no-frames = 该图像没有帧
signature-not-found = 在图像中未找到签名

## Dragging

drag-pages-need-document = 页面可以拖放到文档上。
drag-image-unsupported = prev 无法打开此图像。
# $error is a lowercase reason or a technical message.
drag-area-failed = 无法拖移该区域：{ $error }
drag-pages-failed = 无法拖移页面：{ $error }
drag-start-failed = 无法开始拖移。
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = 页面
drag-file-one-page = { $name }（第 { $page } 页）
drag-file-page-range = { $name }（第 { $first }–{ $last } 页）
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = 图像
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = 要添加到此文档吗？
drop-pdf-body = 要将“{ $name }”添加到此文档的末尾，还是在单独的窗口中打开？
drop-pdfs-body = 要将这 { $count } 个 PDF 添加到此文档的末尾，还是分别在单独的窗口中打开？
drop-pdf-add = 添加到末尾
drop-pdf-open = 单独打开

## PDF window

pdf-opening = 正在打开…
pdf-open-failed = prev 无法打开此文档
pdf-no-pages = 此文档没有页面。
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = 文档已关闭
pdf-keep-original-failed = 无法保留原始版本：{ $error }
pdf-save-failed = 无法保存：{ $error }
pdf-nothing-to-paste = 没有可粘贴的内容。
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = 粘贴已中断
pdf-file-dialog-failed = 无法显示文件对话框：{ $error }
pdf-bookmarks-no-home = 无法保存书签：未设置 HOME
pdf-bookmarks-save-failed = 无法保存书签：{ $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = 第 { $page } 页

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }”受密码保护
pdf-password = 密码
pdf-password-wrong = 密码不正确，请重试。
# Button that opens a locked document.
pdf-unlock = 解锁

# Toolbar tooltips and labels.
pdf-sidebar = 边栏
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = / { $count }
pdf-zoom-out = 缩小
pdf-zoom-in = 放大
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = 适合页面
pdf-fit-width = 适合宽度
pdf-actual-size = 实际大小
pdf-view-continuous = 连续滚动
pdf-view-single-page = 单页
pdf-view-two-pages = 双页
pdf-undo = 撤销
pdf-redo = 重做
pdf-rotate-left = 向左旋转
pdf-rotate-right = 向右旋转
pdf-inspector = 检查器
pdf-markup = 标记
# Tooltip of the button that opens the export dialog.
pdf-export = 导出
pdf-settings = 设置

# Search field.
pdf-search = 搜索
pdf-search-not-found = 未找到
pdf-searching = 正在搜索…
# The match shown, of all matches found.
pdf-search-match = { $current } / { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } / { $total }+

# Inspector: section headings.
pdf-inspector-file = 文件
pdf-inspector-document = 文档
pdf-inspector-pages = 页面
# Inspector: fact labels and values.
pdf-inspector-title = 标题
pdf-inspector-author = 作者
pdf-inspector-subject = 主题
pdf-inspector-keywords = 关键词
pdf-inspector-created = 创建时间
pdf-inspector-modified = 修改时间
pdf-inspector-application = 应用程序
pdf-inspector-producer = PDF 生成器
pdf-inspector-version = 版本
pdf-inspector-security = 安全性
pdf-inspector-not-encrypted = 未加密
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = 已加密（{ $method }）
pdf-inspector-page-count = { $count } 页
pdf-inspector-page-size = 页面大小
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm（{ $width_in } × { $height_in } in）
pdf-loading = 正在载入…

# Sidebar tabs and lists.
pdf-tab-pages = 页面
pdf-tab-contents = 目录
pdf-tab-notes = 高亮和备注
pdf-tab-bookmarks = 书签
pdf-no-outline = 没有目录
pdf-no-outline-detail = 此文档没有大纲。
pdf-no-bookmarks = 没有书签
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = 按 { $keys } 可为页面添加书签。
pdf-no-bookmarks-detail-unbound = 添加了书签的页面会显示在这里。
pdf-remove-bookmark = 移除书签

## Page editing

# Tooltip of the Pages menu button.
pages-menu = 页面
pages-insert-blank = 插入空白页
pages-insert-file = 从文件插入…
pages-copy = 复制页面
# $count is the number of pages copied earlier.
pages-paste = 粘贴 { $count } 个页面
pages-crop = 裁剪到所选范围
pages-select-all = 选择全部页面
pages-delete = 删除页面
pages-apply-redactions = 应用涂黑…
pages-no-copied = 没有可粘贴的已复制页面。
pages-copied = 已复制 { $count } 个页面。
pages-copy-failed = 无法复制页面：{ $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = 读取已中断
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = 不是 prev 可以读取的图像
pages-read-failed = 无法读取文件：{ $error }
pages-at-least-one = 文档至少需要一个页面。
pages-crop-needs-area = 请先用矩形选择工具选择一个区域。
pages-change-failed = 无法更改页面：{ $error }
pages-no-redactions = 没有可应用的涂黑。
pages-redactions-applied = 已应用 { $count } 处涂黑。
pages-forget-versions-failed = 无法删除早期版本：{ $error }
pages-redact-title = 要应用涂黑吗？
pages-redact-body = { $count } 处标记下的文本、图像和绘图将从文档中永久移除，标记会变成黑色方块。此操作无法撤销，并且 prev 保留的此文件的早期版本也会被删除。
# Button that applies redactions.
pages-redact-apply = 应用

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = 导出
pages-export-format = 格式
pages-export-reduce = 减小文件大小（图像为 150 dpi）
pages-export-flatten = 拼合注释和表单域
pages-export-flatten-detail = 标记和已填写的表单域将成为页面的一部分，无法再编辑。尚未应用的涂黑标记不会包含在内。
pages-export-encrypt = 使用密码加密
pages-export-password = 密码
pages-export-verify-password = 验证密码
pages-export-resolution = 分辨率
pages-export-dpi = { $dpi } dpi
pages-export-quality = 质量
# JPEG quality choices.
pages-export-quality-low = 低
pages-export-quality-medium = 中
pages-export-quality-high = 高
pages-export-quality-best = 最佳
pages-export-one-file = 所有页面导出到一个文件中。
pages-export-file-per-page = 每个页面保存为单独的文件，并在你选择的名称后编号。
pages-export-selected-only = 仅所选的 { $count } 页
# Button that opens the file dialog to choose where to export.
pages-export-choose = 导出…
pages-export-no-password = 请输入密码。
pages-export-password-mismatch = 密码不匹配。
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name }（已导出）
# Suggested file name when the document's own name is unknown.
pages-export-untitled = 文档
pages-export-same-file = 请导出到新文件；此文档会自动保存。
pages-export-exporting = 正在导出“{ $name }”…
pages-export-done = 已导出“{ $name }”。
pages-export-done-images = 已导出 { $count } 张图像。
pages-export-failed = 无法导出：{ $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = 导出已中断

## Start window

# Under the app name in a window with no file open.
app-start-hint = 打开或拖入 PDF、图像、SVG 或 Markdown 文件。
app-start-open = 打开…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title }（开发版）
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }：此查看器尚未完成。
app-cannot-open = prev 无法打开此类文件。
app-cannot-read = prev 无法读取此文件：{ $error }
app-kind-pdf = PDF 文档
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format } 图像
app-kind-svg = SVG 绘图
app-kind-markdown = Markdown 文档
app-file-dialog-failed = 无法显示文件对话框：{ $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = 打开
action-settings = 设置

## Toolbar

app-toolbar-keep-shown = 始终显示工具栏
app-toolbar-auto-hide = 指针离开时隐藏工具栏
# The button that shows the toolbar's hidden tools.
app-toolbar-more = 更多

## File facts

# Labels in a file's inspector.
app-fact-name = 名称
app-fact-folder = 文件夹
app-fact-size = 大小
app-fact-modified = 修改时间
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } 字节
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = 无效链接 { $uri }：{ $error }
app-link-open-failed = 无法打开 { $uri }：{ $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = 请安装 wl-clipboard 以粘贴图像
app-copy-needs-wl-clipboard = 请安装 wl-clipboard 以复制图像
app-copy-no-pixels = 该区域没有像素
# wl-copy is a program's name.
app-copy-no-input = wl-copy 没有输入
app-copy-failed = wl-copy 失败
app-clipboard-open-failed = 无法打开剪贴板：{ $error }
app-copy-image-failed = 无法复制图像：{ $error }

## Printing

print-failed = 无法打印：{ $error }
print-stopped = 打印已中断
print-unavailable = 此系统暂不支持打印。
print-no-window = 无法打印：没有可显示打印对话框的窗口
print-dialog-failed = 无法显示打印对话框：{ $error }
# Shown after "Could not print:".
print-job-not-started = 打印机未开始打印任务
# Shown after "Could not print:".
print-printer-stopped = 打印机已停止

## File dialogs

dialog-open = 打开
dialog-filter-all = 所有支持的文件
dialog-filter-pdf = PDF 文档
dialog-filter-images = 图像
dialog-filter-svg = SVG 绘图
dialog-filter-markdown = Markdown
dialog-choose-signatures = 选择签名文件夹
dialog-choose-versions = 选择版本历史记录文件夹
dialog-choose-bookmarks = 选择书签文件

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    用法：prev [FILE]...

    查看和编辑 PDF 与图像。文件会在正在运行的 prev 的窗口中打开，
    如有需要会先启动 prev。

    选项：
      -h, --help     显示此帮助
      -V, --version  显示版本

## Settings, continued

settings-language = 语言
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = 系统默认：{ $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = 输入语言
settings-input-language-system = 跟随键盘布局
settings-input-language-note = 决定空文本框从哪一侧开始。你键入的文本保持其自身的方向。

settings-appearance-system = 系统
settings-appearance-light = 浅色
settings-appearance-dark = 深色
settings-system-accent = 使用系统强调色
# $theme is the Omarchy theme's name.
settings-omarchy-note = 颜色根据“{ $theme }”的强调色生成。
settings-system-accent-note = 颜色根据系统的强调色生成。
settings-system-accent-none = 系统没有强调色，因此 prev 使用自己的颜色。
settings-auto-hide = 指针离开时隐藏工具栏
settings-auto-hide-note = 工具栏浮在文档上方，指针位于窗口外时会滑出隐藏。
settings-animations = 动画
settings-animations-note = 滑动的栏和面板、展开的对话框以及有弹性的按钮。
settings-animations-reduced = 系统要求减弱动态效果时关闭。
settings-corner-radius = 圆角半径
settings-corner-radius-note = 用于对话框和浮动工具栏。
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = 叠层透明度
settings-overlay-note = 透过浮动工具栏能看到多少页面内容。
settings-overlay-value = { $percent }%
settings-storage-signatures = 签名文件夹
settings-storage-versions = 版本历史记录文件夹
settings-storage-bookmarks = 书签文件
settings-storage-apply = 应用
settings-storage-choose = 选择…
# $file is where the settings file is.
settings-storage-note = 已存放在旧位置的文件会留在原处；如需继续使用，请将它们移过去。prev 的应用设置保存在 { $file } 中。
settings-save-failed = 无法保存设置：{ $error }
settings-no-location = 没有存放设置的位置：未设置 HOME
settings-full-path = 请使用完整路径，例如 ~/Documents/prev。
settings-path-is-folder = { $path } 是文件夹，不是文件。
settings-folder-missing = 文件夹 { $path } 不存在。请先创建它，或选择一个文件夹。
settings-path-is-file = { $path } 是文件，不是文件夹。
settings-cannot-write = prev 无法写入 { $path }：{ $error }。

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = 导出
# Section headings in the export dialog.
export-format = 格式
export-quality = 质量
export-size = 大小
# Button that goes on to choose where to save the export.
export-choose = 导出…
# Format choice; the format name stays as it is.
export-format-webp = WebP（无损）
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = 图像
# JPEG quality choices.
export-quality-low = 低
export-quality-medium = 中
export-quality-high = 高
export-quality-best = 最佳
# Size choices: the picture at its own size, or scaled up.
export-size-actual = 实际大小
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } 像素
# $error is the system's reason.
export-dialog-failed = 无法显示保存对话框：{ $error }
# $path is where the file was saved.
export-done = 已导出 { $path }
export-failed = 无法导出：{ $error }
# Shown if exporting ends unexpectedly.
export-stopped = 导出已中断

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = 带有标记的图像无法编辑。请导出以保留标记，或删除标记并关闭标记栏。

# Shown if a background task ends unexpectedly.
image-loading-stopped = 载入已中断
image-reverting-stopped = 复原已中断
image-rendering-stopped = 渲染已中断
image-saving-stopped = 保存已中断
image-markup-stopped = 标记已中断
image-no-version-store = 没有保存版本的位置
image-revert-failed = 无法复原：{ $error }
image-read-failed = 无法读取 { $path }：{ $error }
image-keep-original-failed = 无法保留原始版本：{ $error }
image-save-failed = 无法保存 { $path }：{ $error }
image-markup-start-failed = 无法开始标记：{ $error }
image-cannot-edit = 动画和 SVG 绘图无法编辑。
image-cannot-mark-up = 动画和 SVG 绘图无法添加标记。
image-mark-up-wait = 请等待编辑完成，然后再添加标记。
image-crop-needs-selection = 请先拖出一个选区（选择工具），然后再裁剪。
image-size-needed = 请输入以像素为单位的宽度和高度。
# $name is a file name.
image-cannot-save-format = 对“{ $name }”的更改无法以其格式保存。请使用导出（{ $keys }）。
image-cannot-save-format-unbound = 对“{ $name }”的更改无法以其格式保存。请使用导出。
image-cannot-export-animation = 暂时无法导出动画。
image-drop-pages = 页面可以拖放到文档上。
image-drag-failed = 无法开始拖移。
image-picture-save-failed = 无法将图像保存到“下载”文件夹。
image-open-failed = prev 无法打开此图像
image-opening = 正在打开…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = 名称与格式不符
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }”将保存为 { $format } 文件，但其名称以 .{ $extension } 结尾。其他应用可能无法打开它。
image-name-mismatch-no-extension = “{ $name }”将保存为 { $format } 文件，但其名称没有扩展名。其他应用可能无法打开它。
image-choose-again = 重新选择
image-save-as-is = 按原样保存
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = 帧 { $current } / { $total }
image-position = { $current } / { $total }
image-edited = 已编辑
# Toolbar tooltips.
image-sidebar = 边栏
image-zoom-out = 缩小
image-zoom-in = 放大
image-zoom = { $percent }%
image-fit = 适合窗口
image-actual-size = 实际大小
image-undo = 撤销
image-redo = 重做
image-rotate-left = 向左旋转
image-rotate-right = 向右旋转
image-flip-horizontal = 水平翻转
image-flip-vertical = 垂直翻转
image-select = 矩形选择
image-crop = 裁剪到所选范围
image-adjust-size-tool = 调整大小
image-adjust-color-tool = 调整颜色
# Tooltip and panel title.
image-inspector = 检查器
image-markup = 标记
image-export = 导出
image-settings = 设置
# Panel titles.
image-adjust-color = 调整颜色
image-adjust-size = 调整大小
# Adjust Color sliders.
image-exposure = 曝光
image-contrast = 对比度
image-saturation = 饱和度
image-temperature = 色温
image-tint = 色调
image-sepia = 棕褐色
image-sharpness = 锐度
image-levels = 色阶
image-black-point = 黑点
image-midtones = 中间调
image-white-point = 白点
image-reset-all = 全部复位
# Adjust Size panel.
image-current-size = 当前大小：{ $width } × { $height } 像素
image-width = 宽度
image-height = 高度
image-scale-proportionally = 按比例缩放
# Button that applies the new size.
image-resize = 调整大小
# Inspector panel.
image-inspector-loading = 正在载入…
image-file = 文件
image-format = 格式
image-dimensions-label = 尺寸
image-pixels = { $width } × { $height } 像素
image-no-camera = 没有相机信息。
image-location = 位置
image-remove-location = 移除位置信息
image-no-location = 没有位置信息。
image-keywords-description = 关键词和描述
image-keywords-hint = 关键词，以逗号分隔
image-description = 描述
image-keywords-unsupported = 关键词可以保存在 JPEG、PNG 和 WebP 文件中。
# Heading over the earlier versions of the file.
image-revert-to = 复原到
image-no-versions = 没有早期版本。
image-revert = 复原
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = 不导出标记就关闭吗？
image-close-body = 图像上的标记仅在其窗口打开期间保留。要保留标记，请导出图像：标记会绘制到你保存的副本中。
image-close-anyway = 仍然关闭

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = 读取已中断
markdown-read-failed = prev 无法读取此文件
markdown-draw-failed = 无法绘制文档
# Under the export's size choices.
markdown-export-size = 整个文档，{ $width } × { $height } 像素
# Search results.
markdown-not-found = 未找到
markdown-match = { $current } / { $total }
# Placeholder of the search field.
markdown-search = 搜索
# Toolbar tooltips.
markdown-smaller-text = 缩小文字
markdown-larger-text = 放大文字
markdown-zoom = { $percent }%
markdown-actual-size = 实际大小
# Tooltip and panel title.
markdown-inspector = 检查器
markdown-export = 导出
markdown-settings = 设置
# Inspector headings and labels.
markdown-file = 文件
markdown-document = 文档
markdown-words = 字数
markdown-lines = 行数
markdown-pictures = 图片

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = 相机
image-meta-exposure = 曝光
image-meta-image = 图像
image-meta-make = 制造商
image-meta-model = 型号
image-meta-lens = 镜头
image-meta-exposure-time = 曝光时间
# The lens aperture, written like f/2.8.
image-meta-f-number = 光圈值
image-meta-iso = ISO
image-meta-focal-length = 焦距
image-meta-exposure-bias = 曝光补偿
image-meta-flash = 闪光灯
image-meta-date-taken = 拍摄日期
image-meta-orientation = 方向
image-meta-color-space = 色彩空间
image-meta-software = 软件
image-meta-artist = 作者
image-meta-copyright = 版权
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] 正常
    [2] 水平镜像
    [3] 旋转 180°
    [4] 垂直镜像
    [5] 水平镜像，逆时针旋转 90°
    [6] 顺时针旋转 90°
    [7] 水平镜像，顺时针旋转 90°
    [8] 逆时针旋转 90°
   *[other] 未知（{ $value }）
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] 已闪光
   *[no] 未闪光
}{ $mode ->
    [on] ，强制闪光
    [off] ，关闭
    [auto] ，自动
   *[unknown] {""}
}{ $redeye ->
    [yes] ，防红眼
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] 未校准
   *[other] 其他（{ $code }）
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = 无法打开文档：{ $detail }
error-pdf-page-out-of-range = 第 { $page } 页不存在
error-pdf-password-protected = 文档受密码保护；请打开它，然后复制其页面
error-pdf-no-pages = 没有可提取的页面
error-pdf-crop-outside = 裁剪区域在页面之外
error-pdf-closed = 文档已关闭
error-pdf-saved-unreadable = 保存的文档无法再打开
error-image-read = 无法读取文件：{ $detail }
error-image-invalid = 图像已损坏或无效：{ $detail }
# $library is a program name, such as libheif.
error-image-missing-library = 打开此格式需要 { $library }，但它尚未安装
# $format is an image format name, such as HEIC.
error-image-unsupported = 暂不支持 { $format } 图像
error-image-encode = 无法编码图像：{ $detail }
error-exif-malformed = EXIF 数据格式错误
error-settings-read = 无法读取设置：{ $detail }
error-settings-invalid = 设置无效：{ $detail }
error-remove-location = 无法移除位置：{ $error }
error-location-unsupported = 只能从 JPEG、PNG、WebP 和 TIFF 文件中移除位置信息
error-xmp-unsupported = 关键词和描述只能保存在 JPEG、PNG 和 WebP 文件中

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = 相机 RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = 关于 prev
menu-settings = 设置…
menu-services = 服务
menu-hide = 隐藏 prev
menu-hide-others = 隐藏其他
menu-show-all = 全部显示
menu-quit = 退出 prev
menu-file = 文件
menu-open = 打开…
menu-close = 关闭窗口
menu-export = 导出…
menu-print = 打印…
menu-edit = 编辑
menu-undo = 撤销
menu-redo = 重做
menu-cut = 剪切
menu-copy = 拷贝
menu-paste = 粘贴
menu-select-all = 全选
menu-find = 查找
menu-find-next = 查找下一个
menu-find-previous = 查找上一个
menu-view = 显示
menu-hide-sidebar = 隐藏边栏
menu-thumbnails = 缩略图
menu-contents = 目录
menu-notes = 高亮和备注
menu-bookmarks = 书签
menu-zoom-in = 放大
menu-zoom-out = 缩小
menu-actual-size = 实际大小
menu-zoom-to-fit = 缩放以适合
menu-inspector = 显示检查器
menu-slideshow = 幻灯片放映
menu-full-screen = 进入全屏幕
menu-go = 前往
menu-next-page = 下一页
menu-previous-page = 上一页
menu-go-to-page = 前往页面…
menu-bookmark = 添加书签
menu-tools = 工具
menu-markup = 显示标记工具栏
menu-rotate-left = 向左旋转
menu-rotate-right = 向右旋转
menu-crop = 裁剪
menu-adjust-color = 调整颜色…
menu-window = 窗口
menu-minimize = 最小化
menu-zoom = 缩放
menu-bring-all-to-front = 前置全部窗口
