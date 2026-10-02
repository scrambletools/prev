# prev's interface text in Thai (ไทย), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = มาร์กอัป, note = โน้ต, highlight = ไฮไลท์,
# annotation = คำอธิบายประกอบ, redact/redaction = ปกปิดข้อมูล/การปกปิดข้อมูล,
# inspector = ตัวตรวจสอบ, zoom in/out = ซูมเข้า/ซูมออก, bookmark = บุ๊กมาร์ก,
# page = หน้า, export = ส่งออก, revert = ย้อนกลับ, undo/redo = เลิกทำ/ทำซ้ำ.
# Buttons and menu items use the plain verb (บันทึก, ยกเลิก, ปิด), as Windows
# and macOS do in Thai.

## Language

language-name = ไทย

## Common

common-cancel = ยกเลิก
common-close = ปิด
common-save = บันทึก

## Settings

settings-title = การตั้งค่า
settings-appearance = ลักษณะที่ปรากฏ
settings-colors = สี
settings-windows = หน้าต่าง
settings-default-app = แอปเริ่มต้น
settings-default-app-label = เปิดไฟล์ด้วย prev
settings-default-app-note = ให้ prev เป็นแอปที่เปิดไฟล์ PDF รูปภาพ ภาพวาด SVG และไฟล์ Markdown
settings-default-app-note-windows = Windows ให้เลือกแอปเริ่มต้นได้เฉพาะในการตั้งค่าของ Windows เอง ปุ่มนี้จะเปิดหน้าของ prev ในนั้น
settings-default-app-note-macos = macOS จะขอให้คุณยืนยันทีละประเภท: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP และ AVIF
settings-default-app-status = { $set } จาก { $total } ประเภทไฟล์เปิดด้วย prev
settings-default-app-button = ตั้งเป็นค่าเริ่มต้น
settings-default-app-button-windows = เปิดการตั้งค่า
settings-default-app-no-entry = ไม่ได้ติดตั้งรายการเดสก์ท็อปของ prev ระบบจึงเปิดไฟล์ด้วย prev ไม่ได้ ติดตั้ง prev จากแพ็กเกจหรือด้วย scripts/install.sh
settings-default-app-no-bundle = เปิด prev จาก prev.app เพื่อตั้งเป็นค่าเริ่มต้น
settings-default-app-failed = ไม่สามารถตั้ง prev เป็นค่าเริ่มต้นได้: { $error }
settings-storage = ที่จัดเก็บ
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (บิลด์สำหรับการพัฒนา, { $build })

## Markup toolbar

markup-tool-select = เลือก
markup-tool-area = การเลือกแบบสี่เหลี่ยม
markup-tool-sketch = ร่างภาพ
markup-tool-draw = วาด
markup-tool-shapes = รูปทรง
markup-tool-text-box = กล่องข้อความ
markup-tool-highlight = ไฮไลท์
markup-tool-note = โน้ต
markup-tool-sign = ลงชื่อ
markup-tool-redact = ปกปิดข้อมูล
markup-apply = นำไปใช้
markup-apply-redactions = ใช้การปกปิดข้อมูล
markup-shape-style = ลักษณะรูปทรง
markup-border-color = สีเส้นขอบ
markup-fill-color = สีเติม
markup-text-style = ลักษณะข้อความ
markup-delete = ลบ
markup-undo = เลิกทำ
markup-redo = ทำซ้ำ

## Markup menus

markup-shape-rectangle = สี่เหลี่ยมผืนผ้า
markup-shape-rounded-rectangle = สี่เหลี่ยมมุมมน
markup-shape-oval = วงรี
markup-shape-line = เส้น
markup-shape-arrow = ลูกศร
markup-shape-star = ดาว
markup-shape-polygon = รูปหลายเหลี่ยม
markup-shape-speech-bubble = บอลลูนคำพูด
markup-shape-loupe = แว่นขยาย
markup-shape-mask = มาสก์
markup-style-highlight = ไฮไลท์
markup-style-underline = ขีดเส้นใต้
markup-style-strikethrough = ขีดทับ
markup-style-squiggly = เส้นหยัก
markup-menu-color = สี
markup-menu-font = แบบอักษร
markup-menu-size = ขนาด
markup-menu-alignment = การจัดแนว
markup-line-width = { $width } pt
markup-dashed = เส้นประ

## Notes

markup-kind-note = โน้ต
markup-kind-text-box = กล่องข้อความ
markup-kind-stamp = ตราประทับ
markup-kind-redaction = การปกปิดข้อมูล
markup-kind-shape = รูปทรง
markup-note-delete = ลบโน้ต
markup-note-done = เสร็จสิ้น
markup-note-placeholder = พิมพ์โน้ต
markup-notes-empty = ไม่มีไฮไลท์หรือโน้ต
markup-notes-empty-hint = ไฮไลท์ โน้ต และกล่องข้อความจะแสดงที่นี่
markup-notes-page = หน้า { $page }

## Markup errors

markup-change-failed = ไม่สามารถเปลี่ยนแปลงเอกสารได้: { $error }
markup-copy-area-failed = ไม่สามารถคัดลอกพื้นที่ได้: { $error }
markup-document-closed = เอกสารถูกปิดแล้ว
markup-render-area-failed = ไม่สามารถแสดงผลพื้นที่ได้
markup-copy-stopped = การคัดลอกหยุดลง

## Signatures

signature-menu-empty = ยังไม่มีลายเซ็น
signature-delete = ลบลายเซ็น
signature-create = สร้างลายเซ็น…
signature-dialog-title = สร้างลายเซ็น
signature-tab-draw = วาด
signature-tab-type = พิมพ์
signature-tab-image = รูปภาพ
signature-draw-hint = เซ็นชื่อบนเส้นด้วยเมาส์ ปากกา หรือทัชแพด
signature-your-name = ชื่อของคุณ
signature-image-hint = เลือกรูปถ่ายหรือภาพสแกนลายเซ็นของคุณบนกระดาษสีขาว
signature-choose-image = เลือกรูปภาพ…
signature-description = คำอธิบาย เช่น ชื่อเต็มหรือชื่อย่อ
signature-clear = ล้าง
signature-ink = หมึก
signature-thickness = ความหนา
signature-sign-first = เซ็นชื่อก่อน แล้วจึงบันทึก
signature-default-name = ลายเซ็น { $number }
signature-change-failed = ไม่สามารถเปลี่ยนแปลงลายเซ็นได้: { $error }
signature-no-data-folder = ไม่มีโฟลเดอร์ข้อมูล: ไม่ได้ตั้งค่า HOME
signature-removing-stopped = การลบหยุดลง
signature-saving-stopped = การบันทึกหยุดลง
signature-reading-stopped = การอ่านหยุดลง
signature-not-an-image = ไฟล์นั้นไม่ใช่รูปภาพที่ prev อ่านได้
signature-no-frames = รูปภาพไม่มีเฟรม
signature-not-found = ไม่พบลายเซ็นในรูปภาพ

## Dragging

drag-pages-need-document = วางหน้าได้บนเอกสารเท่านั้น
drag-image-unsupported = prev เปิดรูปภาพนี้ไม่ได้
drag-area-failed = ไม่สามารถลากพื้นที่ได้: { $error }
drag-pages-failed = ไม่สามารถลากหน้าได้: { $error }
drag-start-failed = ไม่สามารถเริ่มลากได้
drag-file-pages = หน้า
drag-file-one-page = { $name } (หน้า { $page })
drag-file-page-range = { $name } (หน้า { $first }–{ $last })
drag-file-image = รูปภาพ
drop-pdf-title = เพิ่มลงในเอกสารนี้หรือไม่
drop-pdf-body = จะเพิ่ม “{ $name }” ต่อท้ายเอกสารนี้ หรือเปิดในหน้าต่างแยก
drop-pdfs-body = จะเพิ่ม PDF { $count } ไฟล์นี้ต่อท้ายเอกสารนี้ หรือเปิดแต่ละไฟล์ในหน้าต่างแยก
drop-pdf-add = เพิ่มต่อท้าย
drop-pdf-open = เปิดแยก

## PDF window

pdf-opening = กำลังเปิด…
pdf-open-failed = prev เปิดเอกสารนี้ไม่ได้
pdf-no-pages = เอกสารไม่มีหน้า
pdf-document-closed = เอกสารถูกปิดแล้ว
pdf-keep-original-failed = ไม่สามารถเก็บเวอร์ชันต้นฉบับไว้ได้: { $error }
pdf-save-failed = ไม่สามารถบันทึกได้: { $error }
pdf-nothing-to-paste = ไม่มีอะไรให้วาง
pdf-pasting-stopped = การวางหยุดลง
pdf-file-dialog-failed = ไม่สามารถแสดงกล่องโต้ตอบไฟล์ได้: { $error }
pdf-bookmarks-no-home = บันทึกบุ๊กมาร์กไม่ได้: ไม่ได้ตั้งค่า HOME
pdf-bookmarks-save-failed = ไม่สามารถบันทึกบุ๊กมาร์กได้: { $error }
pdf-bookmark-page = หน้า { $page }

pdf-password-protected = “{ $name }” ได้รับการป้องกันด้วยรหัสผ่าน
pdf-password = รหัสผ่าน
pdf-password-wrong = รหัสผ่านไม่ถูกต้อง ลองอีกครั้ง
pdf-unlock = ปลดล็อก

pdf-sidebar = แถบด้านข้าง
pdf-page-of = จาก { $count }
pdf-zoom-out = ซูมออก
pdf-zoom-in = ซูมเข้า
pdf-zoom-percent = { $percent }%
pdf-fit-page = พอดีหน้า
pdf-fit-width = พอดีความกว้าง
pdf-actual-size = ขนาดจริง
pdf-view-continuous = เลื่อนต่อเนื่อง
pdf-view-single-page = หน้าเดียว
pdf-view-two-pages = สองหน้า
pdf-undo = เลิกทำ
pdf-redo = ทำซ้ำ
pdf-rotate-left = หมุนซ้าย
pdf-rotate-right = หมุนขวา
pdf-inspector = ตัวตรวจสอบ
pdf-markup = มาร์กอัป
pdf-export = ส่งออก
pdf-settings = การตั้งค่า

pdf-search = ค้นหา
pdf-search-not-found = ไม่พบ
pdf-searching = กำลังค้นหา…
pdf-search-match = { $current } จาก { $total }
pdf-search-match-more = { $current } จาก { $total }+

pdf-inspector-file = ไฟล์
pdf-inspector-document = เอกสาร
pdf-inspector-pages = หน้า
pdf-inspector-title = ชื่อเรื่อง
pdf-inspector-author = ผู้เขียน
pdf-inspector-subject = หัวเรื่อง
pdf-inspector-keywords = คำสำคัญ
pdf-inspector-created = สร้างเมื่อ
pdf-inspector-modified = แก้ไขเมื่อ
pdf-inspector-application = แอปพลิเคชัน
pdf-inspector-producer = ตัวสร้าง PDF
pdf-inspector-version = เวอร์ชัน
pdf-inspector-security = ความปลอดภัย
pdf-inspector-not-encrypted = ไม่ได้เข้ารหัส
pdf-inspector-encrypted = เข้ารหัสแล้ว ({ $method })
pdf-inspector-page-count = { $count ->
   *[other] { $count } หน้า
}
pdf-inspector-page-size = ขนาดหน้า
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } มม. ({ $width_in } × { $height_in } นิ้ว)
pdf-loading = กำลังโหลด…

pdf-tab-pages = หน้า
pdf-tab-contents = สารบัญ
pdf-tab-notes = ไฮไลท์และโน้ต
pdf-tab-bookmarks = บุ๊กมาร์ก
pdf-no-outline = ไม่มีสารบัญ
pdf-no-outline-detail = เอกสารนี้ไม่มีโครงร่าง
pdf-no-bookmarks = ไม่มีบุ๊กมาร์ก
pdf-no-bookmarks-detail = กด { $keys } เพื่อบุ๊กมาร์กหน้า
pdf-no-bookmarks-detail-unbound = หน้าที่บุ๊กมาร์กไว้จะแสดงที่นี่
pdf-remove-bookmark = เอาบุ๊กมาร์กออก

## Page editing

pages-menu = หน้า
pages-insert-blank = แทรกหน้าว่าง
pages-insert-file = แทรกจากไฟล์…
pages-copy = { $count ->
   *[other] คัดลอกหน้า
}
pages-paste = { $count ->
   *[other] วาง { $count } หน้า
}
pages-crop = ครอบตัดตามส่วนที่เลือก
pages-select-all = เลือกทุกหน้า
pages-delete = { $count ->
   *[other] ลบหน้า
}
pages-apply-redactions = ใช้การปกปิดข้อมูล…
pages-no-copied = ไม่มีหน้าที่คัดลอกไว้ให้วาง
pages-copied = { $count ->
   *[other] คัดลอก { $count } หน้าแล้ว
}
pages-copy-failed = ไม่สามารถคัดลอกหน้าได้: { $error }
pages-reading-stopped = การอ่านหยุดลง
pages-image-unreadable = ไม่ใช่รูปภาพที่ prev อ่านได้
pages-read-failed = ไม่สามารถอ่านไฟล์ได้: { $error }
pages-at-least-one = เอกสารต้องมีอย่างน้อยหนึ่งหน้า
pages-crop-needs-area = เลือกพื้นที่ด้วยเครื่องมือการเลือกแบบสี่เหลี่ยมก่อน
pages-change-failed = ไม่สามารถเปลี่ยนแปลงหน้าได้: { $error }
pages-no-redactions = ไม่มีการปกปิดข้อมูลให้นำไปใช้
pages-redactions-applied = { $count ->
   *[other] ใช้การปกปิดข้อมูล { $count } รายการแล้ว
}
pages-forget-versions-failed = ไม่สามารถลบเวอร์ชันก่อนหน้าได้: { $error }
pages-redact-title = ใช้การปกปิดข้อมูลหรือไม่
pages-redact-body = { $count ->
   *[other] ข้อความ รูปภาพ และภาพวาดใต้เครื่องหมาย { $count } จุดจะถูกลบออกจากเอกสารอย่างถาวร และเครื่องหมายจะกลายเป็นกล่องสีดำ การดำเนินการนี้เลิกทำไม่ได้ และเวอร์ชันก่อนหน้าของไฟล์นี้ที่ prev เก็บไว้จะถูกลบ
}
pages-redact-apply = นำไปใช้

## PDF export

pages-export-title = ส่งออก
pages-export-format = รูปแบบ
pages-export-reduce = ลดขนาดไฟล์ (รูปภาพที่ 150 dpi)
pages-export-flatten = รวมคำอธิบายประกอบและช่องฟอร์มเข้ากับหน้า
pages-export-flatten-detail = มาร์กอัปและช่องที่กรอกแล้วจะกลายเป็นส่วนหนึ่งของหน้าและแก้ไขไม่ได้อีก เครื่องหมายปกปิดข้อมูลที่ยังไม่ได้นำไปใช้จะไม่ถูกรวมไว้
pages-export-encrypt = เข้ารหัสด้วยรหัสผ่าน
pages-export-password = รหัสผ่าน
pages-export-verify-password = ยืนยันรหัสผ่าน
pages-export-resolution = ความละเอียด
pages-export-dpi = { $dpi } dpi
pages-export-quality = คุณภาพ
pages-export-quality-low = ต่ำ
pages-export-quality-medium = ปานกลาง
pages-export-quality-high = สูง
pages-export-quality-best = ดีที่สุด
pages-export-one-file = ทุกหน้าจะอยู่ในไฟล์เดียว
pages-export-file-per-page = แต่ละหน้าจะบันทึกเป็นไฟล์แยก โดยใส่หมายเลขต่อท้ายชื่อที่คุณเลือก
pages-export-selected-only = { $count ->
   *[other] เฉพาะหน้าที่เลือก { $count } หน้า
}
pages-export-choose = ส่งออก…
pages-export-no-password = ใส่รหัสผ่าน
pages-export-password-mismatch = รหัสผ่านไม่ตรงกัน
pages-export-file-name = { $name } (ส่งออกแล้ว)
pages-export-untitled = เอกสาร
pages-export-same-file = ส่งออกเป็นไฟล์ใหม่ เอกสารนี้บันทึกโดยอัตโนมัติอยู่แล้ว
pages-export-exporting = กำลังส่งออก “{ $name }”…
pages-export-done = ส่งออก “{ $name }” แล้ว
pages-export-done-images = ส่งออกรูปภาพ { $count } รูปแล้ว
pages-export-failed = ไม่สามารถส่งออกได้: { $error }
pages-export-stopped = การส่งออกหยุดลง

## Start window

app-start-hint = เปิดหรือวางไฟล์ PDF รูปภาพ SVG หรือ Markdown
app-start-open = เปิด…
app-title-dev = { $title } (รุ่นพัฒนา)
app-viewer-missing = { $kind }: ยังไม่มีตัวแสดงผลนี้
app-cannot-open = prev เปิดไฟล์ประเภทนี้ไม่ได้
app-cannot-read = prev อ่านไฟล์นี้ไม่ได้: { $error }
app-kind-pdf = เอกสาร PDF
app-kind-image = รูปภาพ { $format }
app-kind-svg = ภาพวาด SVG
app-kind-markdown = เอกสาร Markdown
app-file-dialog-failed = ไม่สามารถแสดงกล่องโต้ตอบไฟล์ได้: { $error }

## Actions

action-open = เปิด
action-settings = การตั้งค่า

## Toolbar

app-toolbar-keep-shown = แสดงแถบเครื่องมือไว้เสมอ
app-toolbar-auto-hide = ซ่อนแถบเครื่องมือเมื่อตัวชี้ออกไป
app-toolbar-more = เพิ่มเติม

## File facts

app-fact-name = ชื่อ
app-fact-folder = โฟลเดอร์
app-fact-size = ขนาด
app-fact-modified = แก้ไขเมื่อ
app-size-bytes = { $count } ไบต์
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = ลิงก์ไม่ถูกต้อง { $uri }: { $error }
app-link-open-failed = ไม่สามารถเปิด { $uri } ได้: { $error }
app-paste-needs-wl-clipboard = ติดตั้ง wl-clipboard เพื่อวางรูปภาพ
app-copy-needs-wl-clipboard = ติดตั้ง wl-clipboard เพื่อคัดลอกรูปภาพ
app-copy-no-pixels = พื้นที่นี้ไม่มีพิกเซล
app-copy-no-input = wl-copy ไม่ได้รับข้อมูล
app-copy-failed = wl-copy ล้มเหลว
app-clipboard-open-failed = ไม่สามารถเปิดคลิปบอร์ดได้: { $error }
app-copy-image-failed = ไม่สามารถคัดลอกรูปภาพได้: { $error }

## Printing

print-failed = ไม่สามารถพิมพ์ได้: { $error }
print-stopped = การพิมพ์หยุดลง
print-unavailable = ยังพิมพ์บนระบบนี้ไม่ได้
print-no-window = ไม่สามารถพิมพ์ได้: ไม่มีหน้าต่างสำหรับแสดงกล่องโต้ตอบการพิมพ์
print-dialog-failed = ไม่สามารถแสดงกล่องโต้ตอบการพิมพ์ได้: { $error }
print-job-not-started = เครื่องพิมพ์ไม่ได้เริ่มงานพิมพ์
print-printer-stopped = เครื่องพิมพ์หยุดทำงาน

## File dialogs

dialog-open = เปิด
dialog-filter-all = ไฟล์ที่รองรับทั้งหมด
dialog-filter-pdf = เอกสาร PDF
dialog-filter-images = รูปภาพ
dialog-filter-svg = ภาพวาด SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = เลือกโฟลเดอร์ลายเซ็น
dialog-choose-versions = เลือกโฟลเดอร์ประวัติเวอร์ชัน
dialog-choose-bookmarks = เลือกไฟล์บุ๊กมาร์ก

## Command line

usage-help =
    วิธีใช้: prev [FILE]...
          prev --mcp

    ดูและแก้ไข PDF และรูปภาพ ไฟล์จะเปิดในหน้าต่างของ prev ที่กำลังทำงานอยู่
    ซึ่งจะเริ่มทำงานเองหากจำเป็น

    ตัวเลือก:
      -h, --help     แสดงวิธีใช้นี้
      -V, --version  แสดงเวอร์ชัน
          --mcp      ให้บริการ MCP ผ่าน stdin และ stdout เพื่อให้เอเจนต์ AI
                     ควบคุม prev ที่กำลังทำงานอยู่

## Settings, continued

settings-language = ภาษา
settings-language-system = ค่าเริ่มต้นของระบบ: { $language }
settings-input-language = ภาษาสำหรับป้อนข้อความ
settings-input-language-system = ตามรูปแบบแป้นพิมพ์
settings-input-language-note = กำหนดว่าช่องข้อความว่างจะเริ่มจากด้านใด ข้อความที่คุณพิมพ์จะคงทิศทางของตัวเองไว้

settings-appearance-system = ระบบ
settings-appearance-light = สว่าง
settings-appearance-dark = มืด
settings-system-accent = ใช้สีเน้นของระบบ
settings-omarchy-note = สีสร้างจากสีเน้นของ “{ $theme }”
settings-system-accent-note = สีสร้างจากสีเน้นของระบบ
settings-system-accent-none = ระบบไม่มีสีเน้น prev จึงใช้สีที่เลือกไว้ด้านล่าง
settings-accent-chosen-note = สีสร้างจากสีที่เลือกไว้ด้านล่าง
settings-auto-hide = ซ่อนแถบเครื่องมือเมื่อตัวชี้ออกไป
settings-auto-hide-note = แถบเครื่องมือจะลอยอยู่เหนือเอกสารและเลื่อนหายไปเมื่อตัวชี้อยู่นอกหน้าต่าง
settings-animations = ภาพเคลื่อนไหว
settings-animations-note = แถบและแผงที่เลื่อนเข้าออก กล่องโต้ตอบที่ขยายขึ้น และปุ่มที่เด้งได้
settings-animations-reduced = ปิดอยู่ขณะที่ระบบขอให้ลดการเคลื่อนไหว
settings-corner-radius = รัศมีมุม
settings-corner-radius-note = สำหรับกล่องโต้ตอบและแถบเครื่องมือแบบลอย
settings-corner-radius-value = { $radius } px
settings-overlay = ความโปร่งใสของโอเวอร์เลย์
settings-overlay-note = หน้าเอกสารจะมองเห็นผ่านแถบเครื่องมือแบบลอยได้มากเพียงใด
settings-overlay-value = { $percent }%
settings-storage-signatures = โฟลเดอร์ลายเซ็น
settings-storage-versions = โฟลเดอร์ประวัติเวอร์ชัน
settings-storage-bookmarks = ไฟล์บุ๊กมาร์ก
settings-storage-apply = นำไปใช้
settings-storage-choose = เลือก…
settings-storage-note = ไฟล์ที่เก็บไว้ในตำแหน่งเดิมจะยังอยู่ที่นั่น ย้ายไฟล์เหล่านั้นมาหากต้องการใช้ต่อ การตั้งค่าแอป prev บันทึกไว้ใน { $file }
settings-save-failed = ไม่สามารถบันทึกการตั้งค่าได้: { $error }
settings-no-location = ไม่มีตำแหน่งสำหรับการตั้งค่า: ไม่ได้ตั้งค่า HOME
settings-full-path = ใช้พาธแบบเต็ม เช่น ~/Documents/prev
settings-path-is-folder = { $path } เป็นโฟลเดอร์ ไม่ใช่ไฟล์
settings-folder-missing = ไม่มีโฟลเดอร์ { $path } สร้างโฟลเดอร์นี้ก่อน หรือเลือกโฟลเดอร์อื่น
settings-path-is-file = { $path } เป็นไฟล์ ไม่ใช่โฟลเดอร์
settings-cannot-write = prev เขียนลงใน { $path } ไม่ได้: { $error }

## Export dialog

export-title = ส่งออก
export-format = รูปแบบ
export-quality = คุณภาพ
export-size = ขนาด
export-choose = ส่งออก…
export-format-webp = WebP (ไม่สูญเสียคุณภาพ)
export-format-unknown = รูปภาพ
export-quality-low = ต่ำ
export-quality-medium = ปานกลาง
export-quality-high = สูง
export-quality-best = ดีที่สุด
export-size-actual = ขนาดจริง
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } พิกเซล
export-dialog-failed = ไม่สามารถแสดงกล่องโต้ตอบการบันทึกได้: { $error }
export-done = ส่งออก { $path } แล้ว
export-failed = ไม่สามารถส่งออกได้: { $error }
export-stopped = การส่งออกหยุดลง

## Image window

image-marked-no-edit = รูปภาพที่มีมาร์กอัปแก้ไขไม่ได้ ส่งออกเพื่อเก็บมาร์กอัปไว้ หรือลบมาร์กอัปแล้วปิดแถบมาร์กอัป

image-loading-stopped = การโหลดหยุดลง
image-reverting-stopped = การย้อนกลับหยุดลง
image-rendering-stopped = การแสดงผลหยุดลง
image-saving-stopped = การบันทึกหยุดลง
image-markup-stopped = มาร์กอัปหยุดทำงาน
image-no-version-store = ไม่มีที่สำหรับเก็บเวอร์ชัน
image-revert-failed = ไม่สามารถย้อนกลับได้: { $error }
image-read-failed = ไม่สามารถอ่าน { $path } ได้: { $error }
image-keep-original-failed = ไม่สามารถเก็บเวอร์ชันต้นฉบับไว้ได้: { $error }
image-save-failed = ไม่สามารถบันทึก { $path } ได้: { $error }
image-markup-start-failed = ไม่สามารถเริ่มมาร์กอัปได้: { $error }
image-cannot-edit = ภาพเคลื่อนไหวและภาพวาด SVG แก้ไขไม่ได้
image-cannot-mark-up = ภาพเคลื่อนไหวและภาพวาด SVG ใส่มาร์กอัปไม่ได้
image-mark-up-wait = รอให้การแก้ไขเสร็จก่อน แล้วจึงใส่มาร์กอัป
image-crop-needs-selection = ลากเพื่อเลือกก่อน (เครื่องมือเลือก) แล้วจึงครอบตัด
image-size-needed = ใส่ความกว้างและความสูงเป็นพิกเซล
image-cannot-save-format = บันทึกการเปลี่ยนแปลงของ “{ $name }” ในรูปแบบของไฟล์นี้ไม่ได้ ใช้การส่งออก ({ $keys })
image-cannot-save-format-unbound = บันทึกการเปลี่ยนแปลงของ “{ $name }” ในรูปแบบของไฟล์นี้ไม่ได้ ใช้การส่งออก
image-cannot-export-animation = ยังส่งออกภาพเคลื่อนไหวไม่ได้
image-drop-pages = วางหน้าได้บนเอกสารเท่านั้น
image-drag-failed = ไม่สามารถเริ่มลากได้
image-picture-save-failed = ไม่สามารถบันทึกรูปภาพในโฟลเดอร์ดาวน์โหลดได้
image-open-failed = prev เปิดรูปภาพนี้ไม่ได้
image-opening = กำลังเปิด…
image-name-mismatch-title = ชื่อไม่ตรงกับรูปแบบ
image-name-mismatch = “{ $name }” จะถูกบันทึกเป็นไฟล์ { $format } แต่ชื่อลงท้ายด้วย .{ $extension } แอปอื่นอาจเปิดไฟล์นี้ไม่ได้
image-name-mismatch-no-extension = “{ $name }” จะถูกบันทึกเป็นไฟล์ { $format } แต่ชื่อไม่มีนามสกุล แอปอื่นอาจเปิดไฟล์นี้ไม่ได้
image-choose-again = เลือกอีกครั้ง
image-save-as-is = บันทึกตามเดิม
image-dimensions = { $width } × { $height }
image-frame-position = เฟรม { $current } จาก { $total }
image-position = { $current } จาก { $total }
image-edited = แก้ไขแล้ว
image-sidebar = แถบด้านข้าง
image-zoom-out = ซูมออก
image-zoom-in = ซูมเข้า
image-zoom = { $percent }%
image-fit = พอดีหน้าต่าง
image-actual-size = ขนาดจริง
image-undo = เลิกทำ
image-redo = ทำซ้ำ
image-rotate-left = หมุนซ้าย
image-rotate-right = หมุนขวา
image-flip-horizontal = พลิกแนวนอน
image-flip-vertical = พลิกแนวตั้ง
image-select = การเลือกแบบสี่เหลี่ยม
image-crop = ครอบตัดตามส่วนที่เลือก
image-adjust-size-tool = ปรับขนาด
image-adjust-color-tool = ปรับสี
image-inspector = ตัวตรวจสอบ
image-markup = มาร์กอัป
image-export = ส่งออก
image-settings = การตั้งค่า
image-adjust-color = ปรับสี
image-adjust-size = ปรับขนาด
image-exposure = การรับแสง
image-contrast = คอนทราสต์
image-saturation = ความอิ่มตัวของสี
image-temperature = อุณหภูมิสี
image-tint = โทนสี
image-sepia = ซีเปีย
image-sharpness = ความคมชัด
image-levels = ระดับ
image-black-point = จุดดำ
image-midtones = โทนกลาง
image-white-point = จุดขาว
image-reset-all = รีเซ็ตทั้งหมด
image-current-size = ขนาดปัจจุบัน: { $width } × { $height } พิกเซล
image-width = ความกว้าง
image-height = ความสูง
image-scale-proportionally = ปรับขนาดตามสัดส่วน
image-resize = เปลี่ยนขนาด
image-inspector-loading = กำลังโหลด…
image-file = ไฟล์
image-format = รูปแบบ
image-dimensions-label = ขนาดภาพ
image-pixels = { $width } × { $height } พิกเซล
image-no-camera = ไม่มีข้อมูลกล้อง
image-location = ตำแหน่งที่ตั้ง
image-remove-location = ลบข้อมูลตำแหน่งที่ตั้ง
image-no-location = ไม่มีข้อมูลตำแหน่งที่ตั้ง
image-keywords-description = คำสำคัญและคำอธิบาย
image-keywords-hint = คำสำคัญ คั่นด้วยจุลภาค
image-description = คำอธิบาย
image-keywords-unsupported = บันทึกคำสำคัญได้ในไฟล์ JPEG, PNG และ WebP
image-revert-to = ย้อนกลับเป็น
image-no-versions = ไม่มีเวอร์ชันก่อนหน้า
image-revert = ย้อนกลับ
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = ปิดโดยไม่ส่งออกมาร์กอัปหรือไม่
image-close-body = { $count ->
   *[other] มาร์กอัปบนรูปภาพจะอยู่เฉพาะขณะที่หน้าต่างของรูปภาพเปิดอยู่ ส่งออกรูปภาพเพื่อเก็บมาร์กอัปไว้ มาร์กอัปจะถูกวาดลงในสำเนาที่คุณบันทึก
}
image-close-anyway = ปิดเลย

## Markdown

markdown-reading-stopped = การอ่านหยุดลง
markdown-read-failed = prev อ่านไฟล์นี้ไม่ได้
markdown-draw-failed = ไม่สามารถวาดเอกสารได้
markdown-export-size = ทั้งเอกสาร { $width } × { $height } พิกเซล
markdown-not-found = ไม่พบ
markdown-match = { $current } จาก { $total }
markdown-search = ค้นหา
markdown-smaller-text = ข้อความเล็กลง
markdown-larger-text = ข้อความใหญ่ขึ้น
markdown-zoom = { $percent }%
markdown-actual-size = ขนาดจริง
markdown-inspector = ตัวตรวจสอบ
markdown-export = ส่งออก
markdown-settings = การตั้งค่า
markdown-file = ไฟล์
markdown-document = เอกสาร
markdown-words = คำ
markdown-lines = บรรทัด
markdown-pictures = รูปภาพ

## Image details

image-meta-camera = กล้อง
image-meta-exposure = การรับแสง
image-meta-image = รูปภาพ
image-meta-make = ผู้ผลิต
image-meta-model = รุ่น
image-meta-lens = เลนส์
image-meta-exposure-time = เวลารับแสง
image-meta-f-number = ค่า F
image-meta-iso = ISO
image-meta-focal-length = ทางยาวโฟกัส
image-meta-exposure-bias = การชดเชยแสง
image-meta-flash = แฟลช
image-meta-date-taken = วันที่ถ่าย
image-meta-orientation = การวางแนว
image-meta-color-space = ปริภูมิสี
image-meta-software = ซอฟต์แวร์
image-meta-artist = ศิลปิน
image-meta-copyright = ลิขสิทธิ์
image-meta-seconds = { $value } วินาที
image-meta-millimeters = { $value } มม.
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] ปกติ
    [2] กลับด้านแนวนอน
    [3] หมุน 180°
    [4] กลับด้านแนวตั้ง
    [5] กลับด้านแนวนอน หมุนทวนเข็มนาฬิกา 90°
    [6] หมุนตามเข็มนาฬิกา 90°
    [7] กลับด้านแนวนอน หมุนตามเข็มนาฬิกา 90°
    [8] หมุนทวนเข็มนาฬิกา 90°
   *[other] ไม่ทราบ ({ $value })
}
image-meta-flash-value = { $fired ->
    [yes] แฟลชทำงาน
   *[no] แฟลชไม่ทำงาน
}{ $mode ->
    [on] , บังคับเปิด
    [off] , ปิด
    [auto] , อัตโนมัติ
   *[unknown] {""}
}{ $redeye ->
    [yes] , ลดตาแดง
   *[no] {""}
}
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] ไม่ได้ปรับเทียบ
   *[other] อื่นๆ ({ $code })
}

## Errors

error-pdf-open = เปิดเอกสารไม่ได้: { $detail }
error-pdf-page-out-of-range = ไม่มีหน้า { $page }
error-pdf-password-protected = เอกสารได้รับการป้องกันด้วยรหัสผ่าน ให้เปิดเอกสารแล้วคัดลอกหน้าแทน
error-pdf-no-pages = ไม่มีหน้าให้แยกออก
error-pdf-crop-outside = พื้นที่ครอบตัดอยู่นอกหน้า
error-pdf-closed = เอกสารถูกปิดแล้ว
error-pdf-saved-unreadable = เอกสารที่บันทึกไว้เปิดไม่ได้อีกต่อไป
error-image-read = อ่านไฟล์ไม่ได้: { $detail }
error-image-invalid = รูปภาพเสียหายหรือไม่ถูกต้อง: { $detail }
error-image-missing-library = การเปิดรูปแบบนี้ต้องใช้ { $library } ซึ่งยังไม่ได้ติดตั้ง
error-image-unsupported = ยังไม่รองรับรูปภาพ { $format }
error-image-encode = เข้ารหัสรูปภาพไม่ได้: { $detail }
error-exif-malformed = ข้อมูล EXIF มีรูปแบบไม่ถูกต้อง
error-settings-read = อ่านการตั้งค่าไม่ได้: { $detail }
error-settings-invalid = การตั้งค่าไม่ถูกต้อง: { $detail }
error-remove-location = ลบตำแหน่งที่ตั้งไม่ได้: { $error }
error-location-unsupported = ลบข้อมูลตำแหน่งที่ตั้งได้จากไฟล์ JPEG, PNG, WebP และ TIFF
error-xmp-unsupported = บันทึกคำสำคัญและคำอธิบายได้เฉพาะในไฟล์ JPEG, PNG และ WebP

## Formats

format-camera-raw = RAW จากกล้อง

## The macOS menu bar, named as in macOS's own apps.
menu-about = เกี่ยวกับ prev
menu-settings = การตั้งค่า…
menu-services = บริการ
menu-hide = ซ่อน prev
menu-hide-others = ซ่อนแอปอื่น
menu-show-all = แสดงทั้งหมด
menu-quit = ออกจาก prev
menu-file = ไฟล์
menu-open = เปิด…
menu-close = ปิดหน้าต่าง
menu-export = ส่งออก…
menu-print = พิมพ์…
menu-edit = แก้ไข
menu-undo = เลิกทำ
menu-redo = ทำซ้ำ
menu-cut = ตัด
menu-copy = คัดลอก
menu-paste = วาง
menu-select-all = เลือกทั้งหมด
menu-find = ค้นหา
menu-find-next = ค้นหาถัดไป
menu-find-previous = ค้นหาก่อนหน้า
menu-view = มุมมอง
menu-hide-sidebar = ซ่อนแถบด้านข้าง
menu-thumbnails = ภาพย่อ
menu-contents = สารบัญ
menu-notes = ไฮไลท์และโน้ต
menu-bookmarks = บุ๊กมาร์ก
menu-zoom-in = ซูมเข้า
menu-zoom-out = ซูมออก
menu-actual-size = ขนาดจริง
menu-zoom-to-fit = ซูมให้พอดี
menu-inspector = แสดงตัวตรวจสอบ
menu-slideshow = สไลด์โชว์
menu-full-screen = เข้าสู่โหมดเต็มหน้าจอ
menu-go = ไป
menu-next-page = หน้าถัดไป
menu-previous-page = หน้าก่อนหน้า
menu-go-to-page = ไปที่หน้า…
menu-bookmark = เพิ่มบุ๊กมาร์ก
menu-tools = เครื่องมือ
menu-markup = แสดงแถบเครื่องมือมาร์กอัป
menu-rotate-left = หมุนซ้าย
menu-rotate-right = หมุนขวา
menu-crop = ครอบตัด
menu-adjust-color = ปรับสี…
menu-window = หน้าต่าง
menu-minimize = ย่อ
menu-zoom = ซูม
menu-bring-all-to-front = นำทั้งหมดมาไว้ด้านหน้า

## Outside control

settings-outside-control = การควบคุมจากภายนอก
settings-allow-outside-control = อนุญาตการควบคุมจากภายนอก
settings-allow-outside-control-note = เอเจนต์ AI เช่น Claude Code สามารถอ่านและแก้ไขไฟล์ของคุณใน prev ได้ผ่าน prev --mcp โดย prev จะถามก่อนทุกครั้งที่มีเอเจนต์ใหม่
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = อนุญาตแล้ว: { $agents }
settings-forget-agents = ลืม
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = อนุญาตให้ { $agent } ควบคุม prev หรือไม่
agent-prompt-body = { $agent } ขอใช้การควบคุมจากภายนอกของ prev เพื่ออ่านและแก้ไขไฟล์ที่คุณเปิดอยู่ คุณปิดการควบคุมจากภายนอกได้ในการตั้งค่า
agent-prompt-allow = อนุญาต
agent-prompt-deny = ไม่อนุญาต
