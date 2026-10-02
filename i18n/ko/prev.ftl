# prev's interface text in Korean (한국어), translated from i18n/en/prev.ftl.
#
# Each line is `key = text`. Keys stay the same in every language; only the
# text after `=` is translated. `{ $name }` is a value prev fills in, such
# as a page number or a file name; keep it, and move it where the
# language needs it. Plural and other variants use Fluent's selectors:
# https://projectfluent.org/fluent/guide/selectors.html
#
# Sections follow the parts of the interface. Keys shared by several
# parts are under "Common".
#
# A first draft, open to further review. Terms used throughout: markup = 마크업,
# note = 메모, highlight = 하이라이트, annotation = 주석,
# redact/redaction = 가리기/가림 표시, inspector = 속성,
# zoom in/out = 확대/축소, bookmark = 책갈피, page = 페이지, hide = 숨기기,
# print = 인쇄, redo = 다시 실행, window = 창. The macOS menu bar (menu-*)
# keeps Apple's own words instead: 가리기, 프린트, 실행 복귀, 윈도우.
# Redact is 가리기 outside the menus; since Apple's menus use 가리기 for
# Hide, hiding elsewhere is 숨기기 so the two never meet.

## Language

# This language's name in itself, as the Settings language list shows it,
# such as English, Deutsch or עברית.
language-name = 한국어

## Common

common-cancel = 취소
common-close = 닫기
common-save = 저장

## Settings

settings-title = 설정
settings-appearance = 화면 모드
settings-colors = 색상
settings-windows = 창
settings-default-app = 기본 앱
settings-default-app-label = prev로 파일 열기
settings-default-app-note = PDF, 이미지, SVG 그림, Markdown 파일을 prev로 열도록 합니다.
settings-default-app-note-windows = Windows에서는 기본 앱을 Windows 설정에서만 선택할 수 있습니다. 여기서 설정의 prev 페이지를 엽니다.
settings-default-app-note-macos = macOS는 유형마다 확인을 요청합니다: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP, AVIF.
settings-default-app-status = 파일 유형 { $total }개 중 { $set }개가 prev로 열립니다.
settings-default-app-button = 기본으로 설정
settings-default-app-button-windows = 설정 열기
settings-default-app-no-entry = prev의 데스크톱 항목이 설치되어 있지 않아 시스템이 prev로 파일을 열 수 없습니다. 패키지나 scripts/install.sh로 prev를 설치하세요.
settings-default-app-no-bundle = prev를 기본으로 설정하려면 prev.app에서 여세요.
settings-default-app-failed = prev를 기본으로 설정할 수 없습니다: { $error }
settings-storage = 저장 위치
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (개발 빌드, { $build })

## Markup toolbar

markup-tool-select = 선택
markup-tool-area = 사각형 선택
markup-tool-sketch = 스케치
markup-tool-draw = 그리기
markup-tool-shapes = 모양
markup-tool-text-box = 텍스트 상자
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = 하이라이트
markup-tool-note = 메모
# Opens the menu of saved signatures (a verb).
markup-tool-sign = 서명
# A verb: the tool that marks areas to black out.
markup-tool-redact = 가리기
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = 적용
markup-apply-redactions = 가림 표시 적용
markup-shape-style = 모양 스타일
markup-border-color = 테두리 색상
markup-fill-color = 채우기 색상
markup-text-style = 텍스트 스타일
markup-delete = 삭제
markup-undo = 실행 취소
markup-redo = 다시 실행

## Markup menus

markup-shape-rectangle = 사각형
markup-shape-rounded-rectangle = 둥근 사각형
markup-shape-oval = 타원
markup-shape-line = 선
markup-shape-arrow = 화살표
markup-shape-star = 별
markup-shape-polygon = 다각형
markup-shape-speech-bubble = 말풍선
# A shape that magnifies the part of the page under it.
markup-shape-loupe = 돋보기
# A shape that darkens the page around it.
markup-shape-mask = 마스크
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = 하이라이트
markup-style-underline = 밑줄
markup-style-strikethrough = 취소선
markup-style-squiggly = 물결선
# Menu section headings.
markup-menu-color = 색상
markup-menu-font = 서체
markup-menu-size = 크기
markup-menu-alignment = 정렬
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width }pt
markup-dashed = 점선

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = 메모
markup-kind-text-box = 텍스트 상자
markup-kind-stamp = 스탬프
markup-kind-redaction = 가림 표시
markup-kind-shape = 모양
# Tooltips on a note being edited.
markup-note-delete = 메모 삭제
markup-note-done = 완료
markup-note-placeholder = 메모 입력
markup-notes-empty = 하이라이트 또는 메모 없음
markup-notes-empty-hint = 하이라이트, 메모 및 텍스트 상자가 여기에 표시됩니다.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = { $page }페이지

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = 문서를 변경할 수 없습니다: { $error }
markup-copy-area-failed = 영역을 복사할 수 없습니다: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = 문서가 닫힘
markup-render-area-failed = 영역을 렌더링할 수 없음
markup-copy-stopped = 복사가 중단됨

## Signatures

signature-menu-empty = 아직 서명이 없습니다.
signature-delete = 서명 삭제
signature-create = 서명 생성…
signature-dialog-title = 서명 생성
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = 그리기
signature-tab-type = 입력
signature-tab-image = 이미지
signature-draw-hint = 마우스, 펜 또는 터치패드로 선 위에 서명하세요.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = 이름
signature-image-hint = 흰 종이에 쓴 서명을 찍은 사진이나 스캔 이미지를 선택하세요.
signature-choose-image = 이미지 선택…
# Placeholder of the field naming the signature in the library.
signature-description = 설명(예: 전체 이름 또는 이니셜)
# Clears the drawing, typed name or image.
signature-clear = 지우기
# The color the signature is drawn or typed in.
signature-ink = 잉크
# The pen's width, for drawing.
signature-thickness = 두께
signature-sign-first = 먼저 서명한 다음 저장하세요.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = 서명 { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = 서명을 변경할 수 없습니다: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = 데이터 폴더 없음: HOME이 설정되지 않음
signature-removing-stopped = 제거가 중단됨
signature-saving-stopped = 저장이 중단됨
signature-reading-stopped = 읽기가 중단됨
signature-not-an-image = prev에서 읽을 수 있는 이미지 파일이 아님
signature-no-frames = 이미지에 프레임이 없음
signature-not-found = 이미지에서 서명을 찾을 수 없음

## Dragging

drag-pages-need-document = 페이지는 문서에만 놓을 수 있습니다.
drag-image-unsupported = prev에서 이 이미지를 열 수 없습니다.
# $error is a lowercase reason or a technical message.
drag-area-failed = 영역을 드래그할 수 없습니다: { $error }
drag-pages-failed = 페이지를 드래그할 수 없습니다: { $error }
drag-start-failed = 드래그를 시작할 수 없습니다.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = 페이지
drag-file-one-page = { $name } ({ $page }페이지)
drag-file-page-range = { $name } ({ $first }–{ $last }페이지)
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = 이미지
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = 이 문서에 추가하겠습니까?
drop-pdf-body = “{ $name }”을(를) 이 문서의 끝에 추가하겠습니까, 아니면 별도의 창에서 열겠습니까?
drop-pdfs-body = 이 PDF { $count }개를 이 문서의 끝에 추가하겠습니까, 아니면 각각 별도의 창에서 열겠습니까?
drop-pdf-add = 끝에 추가
drop-pdf-open = 따로 열기

## PDF window

pdf-opening = 여는 중…
pdf-open-failed = prev에서 이 문서를 열 수 없습니다
pdf-no-pages = 문서에 페이지가 없습니다.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = 문서가 닫힘
pdf-keep-original-failed = 원본 버전을 보관할 수 없음: { $error }
pdf-save-failed = 저장할 수 없습니다: { $error }
pdf-nothing-to-paste = 붙여넣을 항목이 없습니다.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = 붙여넣기가 중단됨
pdf-file-dialog-failed = 파일 대화상자를 표시할 수 없습니다: { $error }
pdf-bookmarks-no-home = 책갈피를 저장할 수 없습니다: HOME이 설정되지 않음
pdf-bookmarks-save-failed = 책갈피를 저장할 수 없습니다: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = { $page }페이지

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }”은(는) 암호로 보호되어 있습니다
pdf-password = 암호
pdf-password-wrong = 암호가 올바르지 않습니다. 다시 시도하세요.
# Button that opens a locked document.
pdf-unlock = 잠금 해제

# Toolbar tooltips and labels.
pdf-sidebar = 사이드바
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = / { $count }
pdf-zoom-out = 축소
pdf-zoom-in = 확대
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = 페이지에 맞추기
pdf-fit-width = 너비에 맞추기
pdf-actual-size = 실제 크기
pdf-view-continuous = 연속 스크롤
pdf-view-single-page = 단일 페이지
pdf-view-two-pages = 두 페이지
pdf-undo = 실행 취소
pdf-redo = 다시 실행
pdf-rotate-left = 왼쪽으로 회전
pdf-rotate-right = 오른쪽으로 회전
pdf-inspector = 속성
pdf-markup = 마크업
# Tooltip of the button that opens the export dialog.
pdf-export = 내보내기
pdf-settings = 설정

# Search field.
pdf-search = 검색
pdf-search-not-found = 찾을 수 없음
pdf-searching = 검색 중…
# The match shown, of all matches found.
pdf-search-match = { $current }/{ $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current }/{ $total }+

# Inspector: section headings.
pdf-inspector-file = 파일
pdf-inspector-document = 문서
pdf-inspector-pages = 페이지
# Inspector: fact labels and values.
pdf-inspector-title = 제목
pdf-inspector-author = 작성자
pdf-inspector-subject = 주제
pdf-inspector-keywords = 키워드
pdf-inspector-created = 생성일
pdf-inspector-modified = 수정일
pdf-inspector-application = 응용 프로그램
pdf-inspector-producer = PDF 생성기
pdf-inspector-version = 버전
pdf-inspector-security = 보안
pdf-inspector-not-encrypted = 암호화되지 않음
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = 암호화됨({ $method })
pdf-inspector-page-count = { $count }페이지
pdf-inspector-page-size = 페이지 크기
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm }mm({ $width_in } × { $height_in }in)
pdf-loading = 불러오는 중…

# Sidebar tabs and lists.
pdf-tab-pages = 페이지
pdf-tab-contents = 목차
pdf-tab-notes = 하이라이트 및 메모
pdf-tab-bookmarks = 책갈피
pdf-no-outline = 목차 없음
pdf-no-outline-detail = 이 문서에는 개요가 없습니다.
pdf-no-bookmarks = 책갈피 없음
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = 페이지에 책갈피를 추가하려면 { $keys }을(를) 누르세요.
pdf-no-bookmarks-detail-unbound = 책갈피를 추가한 페이지가 여기에 표시됩니다.
pdf-remove-bookmark = 책갈피 제거

## Page editing

# Tooltip of the Pages menu button.
pages-menu = 페이지
pages-insert-blank = 빈 페이지 삽입
pages-insert-file = 파일에서 삽입…
pages-copy = 페이지 복사
# $count is the number of pages copied earlier.
pages-paste = { $count ->
   *[other] 페이지 { $count }개 붙여넣기
}
pages-crop = 선택 영역으로 자르기
pages-select-all = 모든 페이지 선택
pages-delete = 페이지 삭제
pages-apply-redactions = 가림 표시 적용…
pages-no-copied = 붙여넣을 복사된 페이지가 없습니다.
pages-copied = { $count ->
   *[other] 페이지 { $count }개를 복사했습니다.
}
pages-copy-failed = 페이지를 복사할 수 없습니다: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = 읽기가 중단됨
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = prev에서 읽을 수 있는 이미지가 아님
pages-read-failed = 파일을 읽을 수 없습니다: { $error }
pages-at-least-one = 문서에는 페이지가 한 개 이상 있어야 합니다.
pages-crop-needs-area = 먼저 사각형 선택 도구로 영역을 선택하세요.
pages-change-failed = 페이지를 변경할 수 없습니다: { $error }
pages-no-redactions = 적용할 가림 표시가 없습니다.
pages-redactions-applied = { $count ->
   *[other] 가림 표시 { $count }개를 적용했습니다.
}
pages-forget-versions-failed = 이전 버전을 삭제할 수 없습니다: { $error }
pages-redact-title = 가림 표시를 적용하겠습니까?
pages-redact-body = { $count ->
   *[other] 표시 아래의 텍스트, 이미지 및 그림이 문서에서 영구적으로 제거되고, 표시는 검은색 상자로 바뀝니다. 이 작업은 실행 취소할 수 없으며, prev가 보관하는 이 파일의 이전 버전도 삭제됩니다.
}
# Button that applies redactions.
pages-redact-apply = 적용

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = 내보내기
pages-export-format = 포맷
pages-export-reduce = 파일 크기 줄이기(이미지 150dpi)
pages-export-flatten = 주석 및 양식 필드 평면화
pages-export-flatten-detail = 마크업과 입력된 필드가 페이지의 일부가 되어 더 이상 편집할 수 없습니다. 아직 적용하지 않은 가림 표시는 제외됩니다.
pages-export-encrypt = 암호를 사용하여 암호화
pages-export-password = 암호
pages-export-verify-password = 암호 확인
pages-export-resolution = 해상도
pages-export-dpi = { $dpi }dpi
pages-export-quality = 품질
# JPEG quality choices.
pages-export-quality-low = 낮음
pages-export-quality-medium = 중간
pages-export-quality-high = 높음
pages-export-quality-best = 최고
pages-export-one-file = 모든 페이지가 하나의 파일로 저장됩니다.
pages-export-file-per-page = 각 페이지가 별도의 파일로 저장되며, 선택한 이름 뒤에 번호가 붙습니다.
pages-export-selected-only = { $count ->
   *[other] 선택한 페이지만({ $count }개)
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = 내보내기…
pages-export-no-password = 암호를 입력하세요.
pages-export-password-mismatch = 암호가 일치하지 않습니다.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name }(내보냄)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = 문서
pages-export-same-file = 새 파일로 내보내세요. 이 문서는 자동으로 저장됩니다.
pages-export-exporting = “{ $name }” 내보내는 중…
pages-export-done = “{ $name }”을(를) 내보냈습니다.
pages-export-done-images = 이미지 { $count }개를 내보냈습니다.
pages-export-failed = 내보낼 수 없습니다: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = 내보내기가 중단됨

## Start window

# Under the app name in a window with no file open.
app-start-hint = PDF, 이미지, SVG 또는 Markdown 파일을 열거나 여기에 놓으세요.
app-start-open = 열기…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (개발)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: 이 뷰어는 아직 만들어지지 않았습니다.
app-cannot-open = prev에서 이 종류의 파일을 열 수 없습니다.
app-cannot-read = prev에서 이 파일을 읽을 수 없습니다: { $error }
app-kind-pdf = PDF 문서
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = { $format } 이미지
app-kind-svg = SVG 드로잉
app-kind-markdown = Markdown 문서
app-file-dialog-failed = 파일 대화상자를 표시할 수 없습니다: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = 열기
action-settings = 설정

## Toolbar

app-toolbar-keep-shown = 도구 막대 항상 표시
app-toolbar-auto-hide = 포인터가 벗어나면 도구 막대 숨기기
# The button that shows the toolbar's hidden tools.
app-toolbar-more = 더 보기

## File facts

# Labels in a file's inspector.
app-fact-name = 이름
app-fact-folder = 폴더
app-fact-size = 크기
app-fact-modified = 수정일
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count }바이트
app-size-kb = { $size }KB
app-size-mb = { $size }MB
app-size-gb = { $size }GB
app-size-tb = { $size }TB

## Links and clipboard

app-link-invalid = 잘못된 링크 { $uri }: { $error }
app-link-open-failed = { $uri }을(를) 열 수 없습니다: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = 이미지를 붙여넣으려면 wl-clipboard를 설치하세요
app-copy-needs-wl-clipboard = 이미지를 복사하려면 wl-clipboard를 설치하세요
app-copy-no-pixels = 영역에 픽셀이 없음
# wl-copy is a program's name.
app-copy-no-input = wl-copy에 입력이 없음
app-copy-failed = wl-copy 실패
app-clipboard-open-failed = 클립보드를 열 수 없습니다: { $error }
app-copy-image-failed = 이미지를 복사할 수 없습니다: { $error }

## Printing

print-failed = 인쇄할 수 없습니다: { $error }
print-stopped = 인쇄가 중단되었습니다
print-unavailable = 이 시스템에서는 아직 인쇄를 사용할 수 없습니다.
print-no-window = 인쇄할 수 없습니다: 인쇄 대화상자를 표시할 창이 없음
print-dialog-failed = 인쇄 대화상자를 표시할 수 없습니다: { $error }
# Shown after "Could not print:".
print-job-not-started = 프린터가 작업을 시작하지 않음
# Shown after "Could not print:".
print-printer-stopped = 프린터가 중지됨

## File dialogs

dialog-open = 열기
dialog-filter-all = 지원되는 모든 파일
dialog-filter-pdf = PDF 문서
dialog-filter-images = 이미지
dialog-filter-svg = SVG 드로잉
dialog-filter-markdown = Markdown
dialog-choose-signatures = 서명 폴더 선택
dialog-choose-versions = 버전 기록 폴더 선택
dialog-choose-bookmarks = 책갈피 파일 선택

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    사용법: prev [FILE]...
            prev --mcp

    PDF와 이미지를 보고 편집합니다. 파일은 실행 중인 prev의 창에서 열리며,
    필요하면 prev가 시작됩니다.

    옵션:
      -h, --help     이 도움말 보기
      -V, --version  버전 보기
          --mcp      stdin과 stdout으로 MCP를 제공하여 AI 에이전트가 실행 중인
                     prev를 제어하게 합니다

## Settings, continued

settings-language = 언어
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = 시스템 기본값: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = 입력 언어
settings-input-language-system = 키보드 레이아웃 따르기
settings-input-language-note = 빈 텍스트 필드에서 입력이 시작되는 쪽을 설정합니다. 입력한 텍스트는 원래 방향을 유지합니다.

settings-appearance-system = 시스템
settings-appearance-light = 라이트
settings-appearance-dark = 다크
settings-system-accent = 시스템 강조 색상 사용
# $theme is the Omarchy theme's name.
settings-omarchy-note = “{ $theme }”의 강조 색상으로 색상을 만듭니다.
settings-system-accent-note = 시스템 강조 색상으로 색상을 만듭니다.
settings-system-accent-none = 시스템에 강조 색상이 없어 prev는 아래에서 선택한 색상을 사용합니다.
settings-accent-chosen-note = 아래에서 선택한 색상으로 색상을 만듭니다.
settings-auto-hide = 포인터가 벗어나면 도구 막대 숨기기
settings-auto-hide-note = 도구 막대가 문서 위에 떠 있으며, 포인터가 창 밖에 있는 동안에는 밀려나듯 사라집니다.
settings-animations = 애니메이션
settings-animations-note = 미끄러지는 막대와 패널, 커지는 대화상자, 튕기는 버튼.
settings-animations-reduced = 시스템에서 동작 줄이기를 요청하는 동안에는 꺼집니다.
settings-corner-radius = 모서리 반경
settings-corner-radius-note = 대화상자와 떠 있는 도구 막대에 적용됩니다.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius }px
settings-overlay = 오버레이 투명도
settings-overlay-note = 떠 있는 도구 막대를 통해 페이지가 비치는 정도입니다.
settings-overlay-value = { $percent }%
settings-storage-signatures = 서명 폴더
settings-storage-versions = 버전 기록 폴더
settings-storage-bookmarks = 책갈피 파일
settings-storage-apply = 적용
settings-storage-choose = 선택…
# $file is where the settings file is.
settings-storage-note = 이전 위치에 이미 보관된 파일은 그대로 남습니다. 계속 사용하려면 새 위치로 옮기세요. prev 앱 설정은 { $file }에 저장됩니다.
settings-save-failed = 설정을 저장할 수 없습니다: { $error }
settings-no-location = 설정 위치 없음: HOME이 설정되지 않음
settings-full-path = ~/Documents/prev와 같은 전체 경로를 사용하세요.
settings-path-is-folder = { $path }은(는) 파일이 아니라 폴더입니다.
settings-folder-missing = { $path } 폴더가 없습니다. 먼저 폴더를 생성하거나 다른 폴더를 선택하세요.
settings-path-is-file = { $path }은(는) 폴더가 아니라 파일입니다.
settings-cannot-write = prev에서 { $path }에 쓸 수 없습니다: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = 내보내기
# Section headings in the export dialog.
export-format = 포맷
export-quality = 품질
export-size = 크기
# Button that goes on to choose where to save the export.
export-choose = 내보내기…
# Format choice; the format name stays as it is.
export-format-webp = WebP(무손실)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = 이미지
# JPEG quality choices.
export-quality-low = 낮음
export-quality-medium = 중간
export-quality-high = 높음
export-quality-best = 최고
# Size choices: the picture at its own size, or scaled up.
export-size-actual = 실제 크기
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height }픽셀
# $error is the system's reason.
export-dialog-failed = 저장 대화상자를 표시할 수 없습니다: { $error }
# $path is where the file was saved.
export-done = { $path }(으)로 내보냈습니다
export-failed = 내보낼 수 없습니다: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = 내보내기가 중단됨

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = 마크업이 있는 이미지는 편집할 수 없습니다. 마크업을 보존하려면 내보내거나, 마크업을 삭제하고 마크업 막대를 닫으세요.

# Shown if a background task ends unexpectedly.
image-loading-stopped = 불러오기가 중단됨
image-reverting-stopped = 되돌리기가 중단됨
image-rendering-stopped = 렌더링이 중단됨
image-saving-stopped = 저장이 중단됨
image-markup-stopped = 마크업이 중단됨
image-no-version-store = 버전을 보관할 위치가 없습니다
image-revert-failed = 되돌릴 수 없습니다: { $error }
image-read-failed = { $path }을(를) 읽을 수 없습니다: { $error }
image-keep-original-failed = 원본 버전을 보관할 수 없습니다: { $error }
image-save-failed = { $path }을(를) 저장할 수 없습니다: { $error }
image-markup-start-failed = 마크업을 시작할 수 없습니다: { $error }
image-cannot-edit = 애니메이션과 SVG 드로잉은 편집할 수 없습니다.
image-cannot-mark-up = 애니메이션과 SVG 드로잉에는 마크업을 할 수 없습니다.
image-mark-up-wait = 편집이 끝날 때까지 기다린 다음 마크업하세요.
image-crop-needs-selection = 먼저 선택 도구로 영역을 드래그한 다음 자르세요.
image-size-needed = 너비와 높이를 픽셀 단위로 입력하세요.
# $name is a file name.
image-cannot-save-format = “{ $name }”의 변경 사항을 해당 포맷으로 저장할 수 없습니다. 내보내기({ $keys })를 사용하세요.
image-cannot-save-format-unbound = “{ $name }”의 변경 사항을 해당 포맷으로 저장할 수 없습니다. 내보내기를 사용하세요.
image-cannot-export-animation = 애니메이션은 아직 내보낼 수 없습니다.
image-drop-pages = 페이지는 문서에만 놓을 수 있습니다.
image-drag-failed = 드래그를 시작할 수 없습니다.
image-picture-save-failed = 다운로드 폴더에 이미지를 저장할 수 없습니다.
image-open-failed = prev에서 이 이미지를 열 수 없습니다
image-opening = 여는 중…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = 이름이 포맷과 일치하지 않습니다
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }”은(는) { $format } 파일로 저장되지만 이름이 .{ $extension }(으)로 끝납니다. 다른 앱에서 열리지 않을 수 있습니다.
image-name-mismatch-no-extension = “{ $name }”은(는) { $format } 파일로 저장되지만 이름에 확장자가 없습니다. 다른 앱에서 열리지 않을 수 있습니다.
image-choose-again = 다시 선택
image-save-as-is = 그대로 저장
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = 프레임 { $current }/{ $total }
image-position = { $current }/{ $total }
image-edited = 편집됨
# Toolbar tooltips.
image-sidebar = 사이드바
image-zoom-out = 축소
image-zoom-in = 확대
image-zoom = { $percent }%
image-fit = 창에 맞추기
image-actual-size = 실제 크기
image-undo = 실행 취소
image-redo = 다시 실행
image-rotate-left = 왼쪽으로 회전
image-rotate-right = 오른쪽으로 회전
image-flip-horizontal = 수평으로 뒤집기
image-flip-vertical = 수직으로 뒤집기
image-select = 사각형 선택
image-crop = 선택 영역으로 자르기
image-adjust-size-tool = 크기 조절
image-adjust-color-tool = 색상 조절
# Tooltip and panel title.
image-inspector = 속성
image-markup = 마크업
image-export = 내보내기
image-settings = 설정
# Panel titles.
image-adjust-color = 색상 조절
image-adjust-size = 크기 조절
# Adjust Color sliders.
image-exposure = 노출
image-contrast = 대비
image-saturation = 채도
image-temperature = 색온도
image-tint = 색조
image-sepia = 세피아
image-sharpness = 선명도
image-levels = 레벨
image-black-point = 검정점
image-midtones = 중간 톤
image-white-point = 흰색점
image-reset-all = 모두 재설정
# Adjust Size panel.
image-current-size = 현재 크기: { $width } × { $height }픽셀
image-width = 너비
image-height = 높이
image-scale-proportionally = 비례적으로 크기 조절
# Button that applies the new size.
image-resize = 크기 조절
# Inspector panel.
image-inspector-loading = 불러오는 중…
image-file = 파일
image-format = 포맷
image-dimensions-label = 치수
image-pixels = { $width } × { $height }픽셀
image-no-camera = 카메라 정보가 없습니다.
image-location = 위치
image-remove-location = 위치 정보 제거
image-no-location = 위치 정보가 없습니다.
image-keywords-description = 키워드 및 설명
image-keywords-hint = 키워드(쉼표로 구분)
image-description = 설명
image-keywords-unsupported = 키워드는 JPEG, PNG 및 WebP 파일에 저장할 수 있습니다.
# Heading over the earlier versions of the file.
image-revert-to = 되돌릴 버전
image-no-versions = 이전 버전이 없습니다.
image-revert = 되돌리기
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size }KB
image-size-mb = { $size }MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = 마크업을 내보내지 않고 닫겠습니까?
image-close-body = { $count ->
   *[other] 이미지의 마크업은 창이 열려 있는 동안에만 유지됩니다. 마크업을 보존하려면 이미지를 내보내세요. 저장하는 사본에 마크업이 그려집니다.
}
image-close-anyway = 그래도 닫기

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = 읽기가 중단됨
markdown-read-failed = prev에서 이 파일을 읽을 수 없습니다
markdown-draw-failed = 문서를 그릴 수 없습니다
# Under the export's size choices.
markdown-export-size = 문서 전체, { $width } × { $height }픽셀
# Search results.
markdown-not-found = 찾을 수 없음
markdown-match = { $current }/{ $total }
# Placeholder of the search field.
markdown-search = 검색
# Toolbar tooltips.
markdown-smaller-text = 텍스트 작게
markdown-larger-text = 텍스트 크게
markdown-zoom = { $percent }%
markdown-actual-size = 실제 크기
# Tooltip and panel title.
markdown-inspector = 속성
markdown-export = 내보내기
markdown-settings = 설정
# Inspector headings and labels.
markdown-file = 파일
markdown-document = 문서
markdown-words = 단어
markdown-lines = 줄
markdown-pictures = 이미지

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = 카메라
image-meta-exposure = 노출
image-meta-image = 이미지
image-meta-make = 제조사
image-meta-model = 모델
image-meta-lens = 렌즈
image-meta-exposure-time = 노출 시간
# The lens aperture, written like f/2.8.
image-meta-f-number = 조리개 값
image-meta-iso = ISO
image-meta-focal-length = 초점 거리
image-meta-exposure-bias = 노출 보정
image-meta-flash = 플래시
image-meta-date-taken = 촬영 날짜
image-meta-orientation = 방향
image-meta-color-space = 색 공간
image-meta-software = 소프트웨어
image-meta-artist = 작가
image-meta-copyright = 저작권
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value }초
image-meta-millimeters = { $value }mm
image-meta-ev = { $value }EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] 일반
    [2] 수평으로 반전됨
    [3] 180° 회전됨
    [4] 수직으로 반전됨
    [5] 수평으로 반전됨, 반시계 방향으로 90° 회전됨
    [6] 시계 방향으로 90° 회전됨
    [7] 수평으로 반전됨, 시계 방향으로 90° 회전됨
    [8] 반시계 방향으로 90° 회전됨
   *[other] 알 수 없음({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] 발광함
   *[no] 발광 안 함
}{ $mode ->
    [on] , 강제 발광
    [off] , 끔
    [auto] , 자동
   *[unknown] {""}
}{ $redeye ->
    [yes] , 적목 현상 감소
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] 보정되지 않음
   *[other] 기타({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = 문서를 열 수 없음: { $detail }
error-pdf-page-out-of-range = { $page }페이지가 존재하지 않음
error-pdf-password-protected = 문서가 암호로 보호되어 있음. 문서를 연 다음 페이지를 복사하세요
error-pdf-no-pages = 추출할 페이지가 없음
error-pdf-crop-outside = 자르기 영역이 페이지 밖에 있음
error-pdf-closed = 문서가 닫힘
error-pdf-saved-unreadable = 저장된 문서가 더 이상 열리지 않음
error-image-read = 파일을 읽을 수 없음: { $detail }
error-image-invalid = 이미지가 손상되었거나 유효하지 않음: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = 이 포맷을 열려면 { $library }이(가) 필요하지만 설치되어 있지 않음
# $format is an image format name, such as HEIC.
error-image-unsupported = { $format } 이미지는 아직 지원되지 않음
error-image-encode = 이미지를 인코딩할 수 없음: { $detail }
error-exif-malformed = EXIF 데이터 형식이 잘못됨
error-settings-read = 설정을 읽을 수 없음: { $detail }
error-settings-invalid = 잘못된 설정: { $detail }
error-remove-location = 위치를 제거할 수 없음: { $error }
error-location-unsupported = 위치 정보는 JPEG, PNG, WebP 및 TIFF 파일에서만 제거할 수 있음
error-xmp-unsupported = 키워드와 설명은 JPEG, PNG 및 WebP 파일에만 저장할 수 있음

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = 카메라 RAW

## The macOS menu bar, named as in macOS's own apps.
menu-about = prev에 관하여
menu-settings = 설정…
menu-services = 서비스
menu-hide = prev 가리기
menu-hide-others = 기타 가리기
menu-show-all = 모두 보기
menu-quit = prev 종료
menu-file = 파일
menu-open = 열기…
menu-close = 윈도우 닫기
menu-export = 내보내기…
menu-print = 프린트…
menu-edit = 편집
menu-undo = 실행 취소
menu-redo = 실행 복귀
menu-cut = 오려두기
menu-copy = 복사하기
menu-paste = 붙여넣기
menu-select-all = 모두 선택
menu-find = 찾기
menu-find-next = 다음 찾기
menu-find-previous = 이전 찾기
menu-view = 보기
menu-hide-sidebar = 사이드바 가리기
menu-thumbnails = 축소판
menu-contents = 목차
menu-notes = 하이라이트 및 메모
menu-bookmarks = 책갈피
menu-zoom-in = 확대
menu-zoom-out = 축소
menu-actual-size = 실제 크기
menu-zoom-to-fit = 크기에 맞게 확대/축소
menu-inspector = 속성 보기
menu-slideshow = 슬라이드쇼
menu-full-screen = 전체 화면 시작
menu-go = 이동
menu-next-page = 다음 페이지
menu-previous-page = 이전 페이지
menu-go-to-page = 페이지로 이동…
menu-bookmark = 책갈피 추가
menu-tools = 도구
menu-markup = 마크업 도구 막대 보기
menu-rotate-left = 왼쪽으로 회전
menu-rotate-right = 오른쪽으로 회전
menu-crop = 자르기
menu-adjust-color = 색상 조절…
menu-window = 윈도우
menu-minimize = 최소화
menu-zoom = 확대/축소
menu-bring-all-to-front = 모두 앞으로 가져오기

## Outside control

settings-outside-control = 외부 제어
# Settings tabs; Appearance and Storage use settings-appearance and
# settings-storage.
settings-tab-general = 일반
settings-tab-agents = 에이전트
settings-allow-outside-control = 외부 제어 허용
settings-allow-outside-control-note = Claude Code 같은 AI 에이전트가 prev --mcp를 통해 prev에서 파일을 읽고 변경할 수 있습니다. 새 에이전트마다 prev가 먼저 묻습니다.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = 허용됨: { $agents }
settings-forget-agents = 지우기
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = { $agent }이(가) prev를 제어하도록 허용하시겠습니까?
agent-prompt-body = { $agent }이(가) 열려 있는 파일을 읽고 변경하기 위해 prev의 외부 제어를 사용하려고 합니다. 외부 제어는 설정에서 끌 수 있습니다.
agent-prompt-allow = 허용
agent-prompt-deny = 허용 안 함
settings-ask-before-note = 에이전트가 다음 작업을 하기 전에 묻기:
settings-ask-reading = 파일 읽기
settings-ask-viewing = 보기 또는 창 변경
settings-ask-marking-up = 파일에 마크업
settings-ask-editing = 파일 편집
settings-ask-signing = 파일에 서명
settings-ask-redacting = 가림 표시 적용
settings-ask-exporting = 파일 내보내기
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = { $agent }이(가) 이 파일을 읽도록 허용하시겠습니까?
agent-ask-view = { $agent }이(가) 보기를 변경하도록 허용하시겠습니까?
agent-ask-markup = { $agent }이(가) 이 파일에 마크업하도록 허용하시겠습니까?
agent-ask-edit = { $agent }이(가) 이 파일을 편집하도록 허용하시겠습니까?
agent-ask-sign = { $agent }이(가) 이 파일에 서명하도록 허용하시겠습니까?
agent-ask-redact = { $agent }이(가) 가림 표시를 적용하도록 허용하시겠습니까?
agent-ask-export = { $agent }이(가) 이 파일을 내보내도록 허용하시겠습니까?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent }이(가) “{ $tool }”을(를) 사용하려고 합니다. prev가 무엇을 물을지는 설정에서 선택합니다.
agent-ask-final = 이 작업은 실행 취소할 수 없습니다.
