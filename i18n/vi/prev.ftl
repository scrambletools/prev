# prev's interface text in Vietnamese (Tiếng Việt), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = đánh dấu, note = ghi chú, highlight = tô sáng,
# annotation = chú thích, redact/redaction = che/vùng che thông tin,
# inspector = bộ kiểm tra, bookmark = dấu trang, page = trang,
# file = tệp, folder = thư mục, settings = cài đặt,
# zoom in/out = phóng to/thu nhỏ, undo/redo = hoàn tác/làm lại,
# export = xuất, crop = cắt, pixel = pixel.
# Buttons and menu items use sentence case (Lưu, Hủy, Tạo chữ ký…).

## Language

language-name = Tiếng Việt

## Common

common-cancel = Hủy
common-close = Đóng
common-save = Lưu

## Settings

settings-title = Cài đặt
settings-appearance = Giao diện
settings-colors = Màu sắc
settings-windows = Cửa sổ
settings-default-app = Ứng dụng mặc định
settings-default-app-label = Mở tệp bằng prev
settings-default-app-note = Đặt prev làm ứng dụng mở tệp PDF, hình ảnh, bản vẽ SVG và tệp Markdown.
settings-default-app-note-windows = Windows chỉ cho phép chọn ứng dụng mặc định trong phần Cài đặt của chính nó. Nút này sẽ mở trang của prev ở đó.
settings-default-app-note-macos = macOS yêu cầu bạn xác nhận từng loại: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP và AVIF.
settings-default-app-status = { $set } trên { $total } loại tệp được mở bằng prev.
settings-default-app-button = Đặt làm mặc định
settings-default-app-button-windows = Mở Cài đặt
settings-default-app-no-entry = Tệp .desktop của prev chưa được cài đặt nên hệ thống không thể mở tệp bằng prev. Hãy cài đặt prev từ một gói hoặc bằng scripts/install.sh.
settings-default-app-no-bundle = Mở prev từ prev.app để đặt làm mặc định.
settings-default-app-failed = Không thể đặt prev làm mặc định: { $error }
settings-storage = Lưu trữ
settings-version = prev { $version }
settings-version-development = prev { $version } (bản phát triển)

## Markup toolbar

markup-tool-select = Chọn
markup-tool-area = Vùng chọn chữ nhật
markup-tool-sketch = Phác thảo
markup-tool-draw = Vẽ
markup-tool-shapes = Hình dạng
markup-tool-text-box = Hộp văn bản
markup-tool-highlight = Tô sáng
markup-tool-note = Ghi chú
markup-tool-sign = Ký
markup-tool-redact = Che thông tin
markup-apply = Áp dụng
markup-apply-redactions = Áp dụng vùng che
markup-shape-style = Kiểu hình
markup-border-color = Màu viền
markup-fill-color = Màu tô
markup-text-style = Kiểu văn bản
markup-delete = Xóa
markup-undo = Hoàn tác
markup-redo = Làm lại

## Markup menus

markup-shape-rectangle = Hình chữ nhật
markup-shape-rounded-rectangle = Hình chữ nhật bo góc
markup-shape-oval = Hình bầu dục
markup-shape-line = Đường thẳng
markup-shape-arrow = Mũi tên
markup-shape-star = Ngôi sao
markup-shape-polygon = Đa giác
markup-shape-speech-bubble = Bong bóng lời thoại
markup-shape-loupe = Kính lúp
markup-shape-mask = Mặt nạ
markup-style-highlight = Tô sáng
markup-style-underline = Gạch chân
markup-style-strikethrough = Gạch ngang
markup-style-squiggly = Gạch lượn sóng
markup-menu-color = Màu
markup-menu-font = Phông chữ
markup-menu-size = Cỡ
markup-menu-alignment = Căn chỉnh
markup-line-width = { $width } pt
markup-dashed = Nét đứt

## Notes

markup-kind-note = Ghi chú
markup-kind-text-box = Hộp văn bản
markup-kind-stamp = Con dấu
markup-kind-redaction = Vùng che
markup-kind-shape = Hình dạng
markup-note-delete = Xóa ghi chú
markup-note-done = Xong
markup-note-placeholder = Nhập ghi chú
markup-notes-empty = Không có nội dung tô sáng hoặc ghi chú
markup-notes-empty-hint = Nội dung tô sáng, ghi chú và hộp văn bản sẽ xuất hiện ở đây.
markup-notes-page = Trang { $page }

## Markup errors

markup-change-failed = Không thể thay đổi tài liệu: { $error }
markup-copy-area-failed = Không thể sao chép vùng: { $error }
markup-document-closed = tài liệu đã đóng
markup-render-area-failed = không thể kết xuất vùng
markup-copy-stopped = việc sao chép đã dừng

## Signatures

signature-menu-empty = Chưa có chữ ký nào.
signature-delete = Xóa chữ ký
signature-create = Tạo chữ ký…
signature-dialog-title = Tạo chữ ký
signature-tab-draw = Vẽ
signature-tab-type = Nhập
signature-tab-image = Hình ảnh
signature-draw-hint = Ký lên đường kẻ bằng chuột, bút hoặc bàn di chuột.
signature-your-name = Tên của bạn
signature-image-hint = Chọn ảnh chụp hoặc bản quét chữ ký của bạn trên giấy trắng.
signature-choose-image = Chọn hình ảnh…
signature-description = Mô tả, chẳng hạn Họ tên đầy đủ hoặc Tên viết tắt
signature-clear = Xóa hết
signature-ink = Mực
signature-thickness = Độ dày
signature-sign-first = Hãy ký trước, sau đó lưu.
signature-default-name = Chữ ký { $number }
signature-change-failed = Không thể thay đổi chữ ký: { $error }
signature-no-data-folder = không có thư mục dữ liệu: HOME chưa được đặt
signature-removing-stopped = việc xóa đã dừng
signature-saving-stopped = việc lưu đã dừng
signature-reading-stopped = việc đọc đã dừng
signature-not-an-image = tệp đó không phải là hình ảnh mà prev có thể đọc
signature-no-frames = hình ảnh không có khung hình nào
signature-not-found = không tìm thấy chữ ký trong hình ảnh

## Dragging

drag-pages-need-document = Chỉ có thể thả trang vào tài liệu.
drag-image-unsupported = prev không thể mở hình ảnh này.
drag-area-failed = Không thể kéo vùng: { $error }
drag-pages-failed = Không thể kéo các trang: { $error }
drag-start-failed = Không thể bắt đầu kéo.
drag-file-pages = Các trang
drag-file-one-page = { $name } (trang { $page })
drag-file-page-range = { $name } (trang { $first }–{ $last })
drag-file-image = Hình ảnh
drop-pdf-title = Thêm vào tài liệu này?
drop-pdf-body = Thêm “{ $name }” vào cuối tài liệu này hay mở trong cửa sổ riêng?
drop-pdfs-body = Thêm { $count } tệp PDF này vào cuối tài liệu này hay mở mỗi tệp trong một cửa sổ riêng?
drop-pdf-add = Thêm vào cuối
drop-pdf-open = Mở riêng

## PDF window

pdf-opening = Đang mở…
pdf-open-failed = prev không thể mở tài liệu này
pdf-no-pages = Tài liệu không có trang nào.
pdf-document-closed = tài liệu đã đóng
pdf-keep-original-failed = không thể giữ phiên bản gốc: { $error }
pdf-save-failed = Không thể lưu: { $error }
pdf-nothing-to-paste = Không có gì để dán.
pdf-pasting-stopped = việc dán đã dừng
pdf-file-dialog-failed = Không thể hiển thị hộp thoại tệp: { $error }
pdf-bookmarks-no-home = Không thể lưu dấu trang: HOME chưa được đặt
pdf-bookmarks-save-failed = Không thể lưu dấu trang: { $error }
pdf-bookmark-page = Trang { $page }

pdf-password-protected = “{ $name }” được bảo vệ bằng mật khẩu
pdf-password = Mật khẩu
pdf-password-wrong = Mật khẩu không đúng. Hãy thử lại.
pdf-unlock = Mở khóa

pdf-sidebar = Thanh bên
pdf-page-of = / { $count }
pdf-zoom-out = Thu nhỏ
pdf-zoom-in = Phóng to
pdf-zoom-percent = { $percent }%
pdf-fit-page = Vừa trang
pdf-fit-width = Vừa chiều rộng
pdf-actual-size = Kích thước thực
pdf-view-continuous = Cuộn liên tục
pdf-view-single-page = Một trang
pdf-view-two-pages = Hai trang
pdf-undo = Hoàn tác
pdf-redo = Làm lại
pdf-rotate-left = Xoay trái
pdf-rotate-right = Xoay phải
pdf-inspector = Bộ kiểm tra
pdf-markup = Đánh dấu
pdf-export = Xuất
pdf-settings = Cài đặt

pdf-search = Tìm kiếm
pdf-search-not-found = Không tìm thấy
pdf-searching = Đang tìm kiếm…
pdf-search-match = { $current } trên { $total }
pdf-search-match-more = { $current } trên { $total }+

pdf-inspector-file = Tệp
pdf-inspector-document = Tài liệu
pdf-inspector-pages = Trang
pdf-inspector-title = Tiêu đề
pdf-inspector-author = Tác giả
pdf-inspector-subject = Chủ đề
pdf-inspector-keywords = Từ khóa
pdf-inspector-created = Ngày tạo
pdf-inspector-modified = Ngày sửa đổi
pdf-inspector-application = Ứng dụng
pdf-inspector-producer = Trình tạo PDF
pdf-inspector-version = Phiên bản
pdf-inspector-security = Bảo mật
pdf-inspector-not-encrypted = Không mã hóa
pdf-inspector-encrypted = Đã mã hóa ({ $method })
pdf-inspector-page-count = { $count ->
   *[other] { $count } trang
}
pdf-inspector-page-size = Kích thước trang
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } inch)
pdf-loading = Đang tải…

pdf-tab-pages = Trang
pdf-tab-contents = Mục lục
pdf-tab-notes = Tô sáng và ghi chú
pdf-tab-bookmarks = Dấu trang
pdf-no-outline = Không có mục lục
pdf-no-outline-detail = Tài liệu này không có dàn ý.
pdf-no-bookmarks = Không có dấu trang
pdf-no-bookmarks-detail = Nhấn { $keys } để thêm dấu trang.
pdf-no-bookmarks-detail-unbound = Các trang có dấu trang sẽ hiển thị ở đây.
pdf-remove-bookmark = Xóa dấu trang

## Page editing

pages-menu = Trang
pages-insert-blank = Chèn trang trống
pages-insert-file = Chèn từ tệp…
pages-copy = { $count ->
   *[other] Sao chép trang
}
pages-paste = { $count ->
   *[other] Dán { $count } trang
}
pages-crop = Cắt theo vùng chọn
pages-select-all = Chọn tất cả các trang
pages-delete = { $count ->
   *[other] Xóa trang
}
pages-apply-redactions = Áp dụng vùng che…
pages-no-copied = Không có trang đã sao chép nào để dán.
pages-copied = { $count ->
   *[other] Đã sao chép { $count } trang.
}
pages-copy-failed = Không thể sao chép các trang: { $error }
pages-reading-stopped = việc đọc đã dừng
pages-image-unreadable = không phải là hình ảnh mà prev có thể đọc
pages-read-failed = Không thể đọc tệp: { $error }
pages-at-least-one = Tài liệu cần có ít nhất một trang.
pages-crop-needs-area = Trước tiên, hãy chọn một vùng bằng công cụ vùng chọn chữ nhật.
pages-change-failed = Không thể thay đổi các trang: { $error }
pages-no-redactions = Không có vùng che nào để áp dụng.
pages-redactions-applied = { $count ->
   *[other] Đã áp dụng { $count } vùng che.
}
pages-forget-versions-failed = Không thể xóa các phiên bản trước: { $error }
pages-redact-title = Áp dụng vùng che?
pages-redact-body = { $count ->
   *[other] Văn bản, hình ảnh và nét vẽ bên dưới { $count } vùng che sẽ bị xóa vĩnh viễn khỏi tài liệu, và vùng che sẽ trở thành ô màu đen. Không thể hoàn tác thao tác này, và các phiên bản trước của tệp này mà prev lưu giữ sẽ bị xóa.
}
pages-redact-apply = Áp dụng

## PDF export

pages-export-title = Xuất
pages-export-format = Định dạng
pages-export-reduce = Giảm kích thước tệp (hình ảnh ở 150 dpi)
pages-export-flatten = Làm phẳng chú thích và trường biểu mẫu
pages-export-flatten-detail = Nội dung đánh dấu và các trường đã điền trở thành một phần của trang và không thể sửa được nữa. Các vùng che chưa được áp dụng sẽ bị bỏ qua.
pages-export-encrypt = Mã hóa bằng mật khẩu
pages-export-password = Mật khẩu
pages-export-verify-password = Xác minh mật khẩu
pages-export-resolution = Độ phân giải
pages-export-dpi = { $dpi } dpi
pages-export-quality = Chất lượng
pages-export-quality-low = Thấp
pages-export-quality-medium = Trung bình
pages-export-quality-high = Cao
pages-export-quality-best = Tốt nhất
pages-export-one-file = Tất cả các trang được gộp vào một tệp.
pages-export-file-per-page = Mỗi trang được lưu thành một tệp riêng, đánh số theo sau tên bạn chọn.
pages-export-selected-only = { $count ->
   *[other] Chỉ { $count } trang đã chọn
}
pages-export-choose = Xuất…
pages-export-no-password = Hãy nhập mật khẩu.
pages-export-password-mismatch = Mật khẩu không khớp.
pages-export-file-name = { $name } (đã xuất)
pages-export-untitled = tài liệu
pages-export-same-file = Hãy xuất ra một tệp mới; tài liệu này được tự động lưu.
pages-export-exporting = Đang xuất “{ $name }”…
pages-export-done = Đã xuất “{ $name }”.
pages-export-done-images = Đã xuất { $count } hình ảnh.
pages-export-failed = Không thể xuất: { $error }
pages-export-stopped = việc xuất đã dừng

## Start window

app-start-hint = Mở hoặc thả tệp PDF, hình ảnh, SVG hoặc Markdown.
app-start-open = Mở…
app-title-dev = { $title } (phát triển)
app-viewer-missing = { $kind }: trình xem này chưa được xây dựng.
app-cannot-open = prev không thể mở loại tệp này.
app-cannot-read = prev không thể đọc tệp này: { $error }
app-kind-pdf = Tài liệu PDF
app-kind-image = Hình ảnh { $format }
app-kind-svg = Bản vẽ SVG
app-kind-markdown = Tài liệu Markdown
app-file-dialog-failed = Không thể hiển thị hộp thoại tệp: { $error }

## Actions

action-open = Mở
action-settings = Cài đặt

## Toolbar

app-toolbar-keep-shown = Luôn hiển thị thanh công cụ
app-toolbar-auto-hide = Ẩn thanh công cụ khi con trỏ rời đi
app-toolbar-more = Thêm

## File facts

app-fact-name = Tên
app-fact-folder = Thư mục
app-fact-size = Kích thước
app-fact-modified = Ngày sửa đổi
app-size-bytes = { $count } byte
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Liên kết không hợp lệ { $uri }: { $error }
app-link-open-failed = Không thể mở { $uri }: { $error }
app-paste-needs-wl-clipboard = hãy cài đặt wl-clipboard để dán hình ảnh
app-copy-needs-wl-clipboard = hãy cài đặt wl-clipboard để sao chép hình ảnh
app-copy-no-pixels = vùng này không có pixel nào
app-copy-no-input = wl-copy không có dữ liệu đầu vào
app-copy-failed = wl-copy thất bại
app-clipboard-open-failed = Không thể mở bảng tạm: { $error }
app-copy-image-failed = Không thể sao chép hình ảnh: { $error }

## Printing

print-failed = Không thể in: { $error }
print-stopped = Việc in đã dừng
print-unavailable = Chưa thể in trên hệ thống này.
print-no-window = Không thể in: không có cửa sổ để hiển thị hộp thoại in
print-dialog-failed = Không thể hiển thị hộp thoại in: { $error }
print-job-not-started = máy in không bắt đầu lệnh in
print-printer-stopped = máy in đã dừng

## File dialogs

dialog-open = Mở
dialog-filter-all = Tất cả các tệp được hỗ trợ
dialog-filter-pdf = Tài liệu PDF
dialog-filter-images = Hình ảnh
dialog-filter-svg = Bản vẽ SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Chọn thư mục chữ ký
dialog-choose-versions = Chọn thư mục lịch sử phiên bản
dialog-choose-bookmarks = Chọn tệp dấu trang

## Command line

usage-help =
    Cách dùng: prev [FILE]...

    Xem và chỉnh sửa PDF và hình ảnh. Tệp được mở trong cửa sổ của prev đang
    chạy, và prev sẽ khởi động nếu cần.

    Tùy chọn:
      -h, --help     Hiển thị trợ giúp này
      -V, --version  Hiển thị phiên bản

## Settings, continued

settings-language = Ngôn ngữ
settings-language-system = Mặc định của hệ thống: { $language }
settings-input-language = Ngôn ngữ nhập
settings-input-language-system = Theo bố cục bàn phím
settings-input-language-note = Đặt phía bắt đầu của trường văn bản trống. Văn bản bạn nhập vẫn giữ hướng riêng của nó.

settings-appearance-system = Hệ thống
settings-appearance-light = Sáng
settings-appearance-dark = Tối
settings-omarchy-accent = Dùng màu nhấn của Omarchy
settings-omarchy-note = Màu sắc được tạo từ màu nhấn của “{ $theme }”.
settings-omarchy-none = Không có chủ đề Omarchy nào đang hoạt động.
settings-auto-hide = Ẩn thanh công cụ khi con trỏ rời đi
settings-auto-hide-note = Thanh công cụ nổi phía trên tài liệu và trượt đi khi con trỏ ở ngoài cửa sổ.
settings-animations = Hiệu ứng động
settings-animations-note = Thanh và bảng trượt, hộp thoại phóng to và nút bấm nảy.
settings-animations-reduced = Tắt khi hệ thống yêu cầu giảm chuyển động.
settings-corner-radius = Bán kính góc
settings-corner-radius-note = Cho hộp thoại và thanh công cụ nổi.
settings-corner-radius-value = { $radius } px
settings-overlay = Độ trong suốt lớp phủ
settings-overlay-note = Mức độ trang hiện xuyên qua thanh công cụ nổi.
settings-overlay-value = { $percent }%
settings-storage-signatures = Thư mục chữ ký
settings-storage-versions = Thư mục lịch sử phiên bản
settings-storage-bookmarks = Tệp dấu trang
settings-storage-apply = Áp dụng
settings-storage-choose = Chọn…
settings-storage-note = Các tệp đã lưu ở vị trí cũ vẫn ở đó; hãy di chuyển chúng sang để tiếp tục sử dụng. Cài đặt ứng dụng prev được lưu trong { $file }.
settings-save-failed = Không thể lưu cài đặt: { $error }
settings-no-location = Không có vị trí lưu cài đặt: HOME chưa được đặt
settings-full-path = Hãy dùng đường dẫn đầy đủ, chẳng hạn ~/Documents/prev.
settings-path-is-folder = { $path } là thư mục, không phải tệp.
settings-folder-missing = Không có thư mục { $path }. Hãy tạo thư mục trước hoặc chọn một thư mục khác.
settings-path-is-file = { $path } là tệp, không phải thư mục.
settings-cannot-write = prev không thể ghi vào { $path }: { $error }.

## Export dialog

export-title = Xuất
export-format = Định dạng
export-quality = Chất lượng
export-size = Kích thước
export-choose = Xuất…
export-format-webp = WebP (không mất dữ liệu)
export-format-unknown = hình ảnh
export-quality-low = Thấp
export-quality-medium = Trung bình
export-quality-high = Cao
export-quality-best = Tốt nhất
export-size-actual = Kích thước thực
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } pixel
export-dialog-failed = Không thể hiển thị hộp thoại lưu: { $error }
export-done = Đã xuất { $path }
export-failed = Không thể xuất: { $error }
export-stopped = việc xuất đã dừng

## Image window

image-marked-no-edit = Không thể chỉnh sửa hình ảnh có đánh dấu. Hãy xuất để giữ lại phần đánh dấu, hoặc xóa phần đánh dấu rồi đóng thanh đánh dấu.

image-loading-stopped = việc tải đã dừng
image-reverting-stopped = việc khôi phục đã dừng
image-rendering-stopped = việc kết xuất đã dừng
image-saving-stopped = việc lưu đã dừng
image-markup-stopped = việc đánh dấu đã dừng
image-no-version-store = Không có nơi lưu giữ phiên bản
image-revert-failed = Không thể khôi phục: { $error }
image-read-failed = Không thể đọc { $path }: { $error }
image-keep-original-failed = Không thể giữ phiên bản gốc: { $error }
image-save-failed = Không thể lưu { $path }: { $error }
image-markup-start-failed = Không thể bắt đầu đánh dấu: { $error }
image-cannot-edit = Không thể chỉnh sửa ảnh động và bản vẽ SVG.
image-cannot-mark-up = Không thể đánh dấu ảnh động và bản vẽ SVG.
image-mark-up-wait = Hãy đợi chỉnh sửa hoàn tất rồi mới đánh dấu.
image-crop-needs-selection = Trước tiên, hãy kéo tạo vùng chọn (công cụ Chọn), rồi cắt.
image-size-needed = Hãy nhập chiều rộng và chiều cao theo pixel.
image-cannot-save-format = Không thể lưu các thay đổi của “{ $name }” ở định dạng của nó. Hãy dùng Xuất ({ $keys }).
image-cannot-save-format-unbound = Không thể lưu các thay đổi của “{ $name }” ở định dạng của nó. Hãy dùng Xuất.
image-cannot-export-animation = Chưa thể xuất ảnh động.
image-drop-pages = Chỉ có thể thả trang vào tài liệu.
image-drag-failed = Không thể bắt đầu kéo.
image-picture-save-failed = Không thể lưu hình ảnh vào thư mục Tải xuống của bạn.
image-open-failed = prev không thể mở hình ảnh này
image-opening = Đang mở…
image-name-mismatch-title = Tên không khớp với định dạng
image-name-mismatch = “{ $name }” sẽ được lưu dưới dạng tệp { $format }, nhưng tên của nó kết thúc bằng .{ $extension }. Các ứng dụng khác có thể không mở được tệp này.
image-name-mismatch-no-extension = “{ $name }” sẽ được lưu dưới dạng tệp { $format }, nhưng tên của nó không có phần mở rộng. Các ứng dụng khác có thể không mở được tệp này.
image-choose-again = Chọn lại
image-save-as-is = Vẫn lưu
image-dimensions = { $width } × { $height }
image-frame-position = khung hình { $current } trên { $total }
image-position = { $current } trên { $total }
image-edited = đã sửa
image-sidebar = Thanh bên
image-zoom-out = Thu nhỏ
image-zoom-in = Phóng to
image-zoom = { $percent }%
image-fit = Vừa cửa sổ
image-actual-size = Kích thước thực
image-undo = Hoàn tác
image-redo = Làm lại
image-rotate-left = Xoay trái
image-rotate-right = Xoay phải
image-flip-horizontal = Lật ngang
image-flip-vertical = Lật dọc
image-select = Vùng chọn chữ nhật
image-crop = Cắt theo vùng chọn
image-adjust-size-tool = Điều chỉnh kích thước
image-adjust-color-tool = Điều chỉnh màu
image-inspector = Bộ kiểm tra
image-markup = Đánh dấu
image-export = Xuất
image-settings = Cài đặt
image-adjust-color = Điều chỉnh màu
image-adjust-size = Điều chỉnh kích thước
image-exposure = Phơi sáng
image-contrast = Độ tương phản
image-saturation = Độ bão hòa
image-temperature = Nhiệt độ màu
image-tint = Sắc độ
image-sepia = Nâu cổ điển
image-sharpness = Độ sắc nét
image-levels = Mức sáng
image-black-point = Điểm đen
image-midtones = Tông trung
image-white-point = Điểm trắng
image-reset-all = Đặt lại tất cả
image-current-size = Kích thước hiện tại: { $width } × { $height } pixel
image-width = Chiều rộng
image-height = Chiều cao
image-scale-proportionally = Co giãn theo tỷ lệ
image-resize = Đổi kích thước
image-inspector-loading = Đang tải…
image-file = Tệp
image-format = Định dạng
image-dimensions-label = Kích thước ảnh
image-pixels = { $width } × { $height } pixel
image-no-camera = Không có thông tin máy ảnh.
image-location = Vị trí
image-remove-location = Xóa thông tin vị trí
image-no-location = Không có thông tin vị trí.
image-keywords-description = Từ khóa và mô tả
image-keywords-hint = Từ khóa, phân tách bằng dấu phẩy
image-description = Mô tả
image-keywords-unsupported = Có thể lưu từ khóa trong tệp JPEG, PNG và WebP.
image-revert-to = Khôi phục về
image-no-versions = Không có phiên bản trước.
image-revert = Khôi phục
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Đóng mà không xuất phần đánh dấu?
image-close-body = { $count ->
   *[other] Phần đánh dấu trên hình ảnh chỉ tồn tại khi cửa sổ còn mở. Hãy xuất hình ảnh để giữ lại: phần đánh dấu sẽ được vẽ vào bản sao bạn lưu.
}
image-close-anyway = Vẫn đóng

## Markdown

markdown-reading-stopped = việc đọc đã dừng
markdown-read-failed = prev không thể đọc tệp này
markdown-draw-failed = Không thể vẽ tài liệu
markdown-export-size = Toàn bộ tài liệu, { $width } × { $height } pixel
markdown-not-found = Không tìm thấy
markdown-match = { $current } trên { $total }
markdown-search = Tìm kiếm
markdown-smaller-text = Chữ nhỏ hơn
markdown-larger-text = Chữ lớn hơn
markdown-zoom = { $percent }%
markdown-actual-size = Kích thước thực
markdown-inspector = Bộ kiểm tra
markdown-export = Xuất
markdown-settings = Cài đặt
markdown-file = Tệp
markdown-document = Tài liệu
markdown-words = Số từ
markdown-lines = Số dòng
markdown-pictures = Hình ảnh

## Image details

image-meta-camera = Máy ảnh
image-meta-exposure = Phơi sáng
image-meta-image = Hình ảnh
image-meta-make = Hãng
image-meta-model = Kiểu máy
image-meta-lens = Ống kính
image-meta-exposure-time = Thời gian phơi sáng
image-meta-f-number = Khẩu độ
image-meta-iso = ISO
image-meta-focal-length = Tiêu cự
image-meta-exposure-bias = Bù phơi sáng
image-meta-flash = Đèn flash
image-meta-date-taken = Ngày chụp
image-meta-orientation = Hướng
image-meta-color-space = Không gian màu
image-meta-software = Phần mềm
image-meta-artist = Tác giả
image-meta-copyright = Bản quyền
image-meta-seconds = { $value } giây
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Bình thường
    [2] Lật ngang
    [3] Xoay 180°
    [4] Lật dọc
    [5] Lật ngang, xoay 90° ngược chiều kim đồng hồ
    [6] Xoay 90° theo chiều kim đồng hồ
    [7] Lật ngang, xoay 90° theo chiều kim đồng hồ
    [8] Xoay 90° ngược chiều kim đồng hồ
   *[other] Không xác định ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] Có nháy
   *[no] Không nháy
}{ $mode ->
    [on] , bắt buộc bật
    [off] , tắt
    [auto] , tự động
   *[unknown] {""}
}{ $redeye ->
    [yes] , giảm mắt đỏ
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Chưa hiệu chuẩn
   *[other] Khác ({ $code })
}

## Errors

error-pdf-open = không thể mở tài liệu: { $detail }
error-pdf-page-out-of-range = trang { $page } không tồn tại
error-pdf-password-protected = tài liệu được bảo vệ bằng mật khẩu; hãy mở tài liệu rồi sao chép các trang của nó
error-pdf-no-pages = không có trang nào để trích xuất
error-pdf-crop-outside = vùng cắt nằm ngoài trang
error-pdf-closed = tài liệu đã đóng
error-pdf-saved-unreadable = tài liệu đã lưu không còn mở được
error-image-read = không thể đọc tệp: { $detail }
error-image-invalid = hình ảnh bị hỏng hoặc không hợp lệ: { $detail }
error-image-missing-library = để mở định dạng này cần có { $library }, nhưng thư viện này chưa được cài đặt
error-image-unsupported = hình ảnh { $format } chưa được hỗ trợ
error-image-encode = không thể mã hóa hình ảnh: { $detail }
error-exif-malformed = dữ liệu EXIF bị lỗi
error-settings-read = không thể đọc cài đặt: { $detail }
error-settings-invalid = cài đặt không hợp lệ: { $detail }
error-remove-location = không thể xóa vị trí: { $error }
error-location-unsupported = có thể xóa thông tin vị trí khỏi tệp JPEG, PNG, WebP và TIFF
error-xmp-unsupported = chỉ có thể lưu từ khóa và mô tả trong tệp JPEG, PNG và WebP

## Formats

format-camera-raw = RAW máy ảnh

## The macOS menu bar, named as in macOS's own apps.
menu-about = Giới thiệu về prev
menu-settings = Cài đặt…
menu-services = Dịch vụ
menu-hide = Ẩn prev
menu-hide-others = Ẩn các ứng dụng khác
menu-show-all = Hiển thị tất cả
menu-quit = Thoát prev
menu-file = Tệp
menu-open = Mở…
menu-close = Đóng cửa sổ
menu-export = Xuất…
menu-print = In…
menu-edit = Sửa
menu-undo = Hoàn tác
menu-redo = Làm lại
menu-cut = Cắt
menu-copy = Sao chép
menu-paste = Dán
menu-select-all = Chọn tất cả
menu-find = Tìm
menu-find-next = Tìm tiếp
menu-find-previous = Tìm trước
menu-view = Xem
menu-hide-sidebar = Ẩn thanh bên
menu-thumbnails = Hình thu nhỏ
menu-contents = Mục lục
menu-notes = Tô sáng và ghi chú
menu-bookmarks = Dấu trang
menu-zoom-in = Phóng to
menu-zoom-out = Thu nhỏ
menu-actual-size = Kích thước thực
menu-zoom-to-fit = Thu phóng vừa khít
menu-inspector = Hiển thị bộ kiểm tra
menu-slideshow = Trình chiếu
menu-full-screen = Vào chế độ toàn màn hình
menu-go = Đi
menu-next-page = Trang tiếp theo
menu-previous-page = Trang trước
menu-go-to-page = Đi tới trang…
menu-bookmark = Thêm dấu trang
menu-tools = Công cụ
menu-markup = Hiển thị thanh công cụ đánh dấu
menu-rotate-left = Xoay trái
menu-rotate-right = Xoay phải
menu-crop = Cắt
menu-adjust-color = Điều chỉnh màu…
menu-window = Cửa sổ
menu-minimize = Thu nhỏ cửa sổ
menu-zoom = Thu phóng
menu-bring-all-to-front = Đưa tất cả ra trước
