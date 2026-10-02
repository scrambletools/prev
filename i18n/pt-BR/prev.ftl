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
language-name = Português (Brasil)

## Common

common-cancel = Cancelar
common-close = Fechar
common-save = Salvar

## Settings

settings-title = Ajustes
settings-appearance = Aparência
settings-colors = Cores
settings-windows = Janelas
settings-default-app = App padrão
settings-default-app-label = Abrir arquivos com o prev
settings-default-app-note = Faça do prev o app que abre PDFs, imagens, desenhos SVG e arquivos Markdown.
settings-default-app-note-windows = O Windows só permite escolher apps padrão nas próprias Configurações. Isto abre a página do prev lá.
settings-default-app-note-macos = O macOS pede para confirmar cada tipo: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP e AVIF.
settings-default-app-status = { $set } de { $total } tipos de arquivo abrem com o prev.
settings-default-app-button = Definir como padrão
settings-default-app-button-windows = Abrir Configurações
settings-default-app-no-entry = A entrada de área de trabalho do prev não está instalada, então o sistema não pode abrir arquivos com ele. Instale o prev de um pacote ou com scripts/install.sh.
settings-default-app-no-bundle = Abra o prev pelo prev.app para defini-lo como padrão.
settings-default-app-failed = Não foi possível definir o prev como padrão: { $error }
settings-storage = Armazenamento
settings-version = prev { $version } ({ $build })
settings-version-development = prev { $version } (versão de desenvolvimento, { $build })

## Markup toolbar

markup-tool-select = Selecionar
markup-tool-area = Seleção retangular
markup-tool-sketch = Esboço
markup-tool-draw = Desenhar
markup-tool-shapes = Formas
markup-tool-text-box = Caixa de texto
# The button that opens the menu of highlight, underline and strikethrough.
markup-tool-highlight = Realçar
markup-tool-note = Nota
# Opens the menu of saved signatures (a verb).
markup-tool-sign = Assinar
# A verb: the tool that marks areas to black out.
markup-tool-redact = Tarjar
# A short button next to the Redact tool; its tooltip is markup-apply-redactions.
markup-apply = Aplicar
markup-apply-redactions = Aplicar tarjas
markup-shape-style = Estilo da forma
markup-border-color = Cor da borda
markup-fill-color = Cor de preenchimento
markup-text-style = Estilo do texto
markup-delete = Apagar
markup-undo = Desfazer
markup-redo = Refazer

## Markup menus

markup-shape-rectangle = Retângulo
markup-shape-rounded-rectangle = Retângulo Arredondado
markup-shape-oval = Oval
markup-shape-line = Linha
markup-shape-arrow = Seta
markup-shape-star = Estrela
markup-shape-polygon = Polígono
markup-shape-speech-bubble = Balão de Fala
# A shape that magnifies the part of the page under it.
markup-shape-loupe = Lupa
# A shape that darkens the page around it.
markup-shape-mask = Máscara
# Ways to mark text; also shown in the notes sidebar for marks without text.
markup-style-highlight = Realce
markup-style-underline = Sublinhado
markup-style-strikethrough = Tachado
markup-style-squiggly = Ondulado
# Menu section headings.
markup-menu-color = Cor
markup-menu-font = Fonte
markup-menu-size = Tamanho
markup-menu-alignment = Alinhamento
# A line thickness; $width is a number of points (1/72 inch).
markup-line-width = { $width } pt
markup-dashed = Tracejada

## Notes

# The heading of a note being edited, when it has no author, and the
# notes sidebar's name for an empty note.
markup-kind-note = Nota
markup-kind-text-box = Caixa de texto
markup-kind-stamp = Carimbo
markup-kind-redaction = Tarja
markup-kind-shape = Forma
# Tooltips on a note being edited.
markup-note-delete = Apagar nota
markup-note-done = OK
markup-note-placeholder = Digite uma nota
markup-notes-empty = Nenhum realce ou nota
markup-notes-empty-hint = Realces, notas e caixas de texto aparecem aqui.
# Above each entry in the notes sidebar; $page is the page's label, such as 3 or iv.
markup-notes-page = Página { $page }

## Markup errors

# $error is one of the lowercase reasons below, or a technical message.
markup-change-failed = Não foi possível alterar o documento: { $error }
markup-copy-area-failed = Não foi possível copiar a área: { $error }
# Reasons placed in the messages above and in drag-area-failed.
markup-document-closed = o documento foi fechado
markup-render-area-failed = não foi possível renderizar a área
markup-copy-stopped = a cópia foi interrompida

## Signatures

signature-menu-empty = Nenhuma assinatura ainda.
signature-delete = Apagar assinatura
signature-create = Criar Assinatura…
signature-dialog-title = Criar Assinatura
# Tabs of the Create Signature dialog: ways to make a signature.
signature-tab-draw = Desenhar
signature-tab-type = Digitar
signature-tab-image = Imagem
signature-draw-hint = Assine na linha com o mouse, a caneta ou o touchpad.
# The placeholder of the field where a signature is typed, and its preview.
signature-your-name = Seu nome
signature-image-hint = Escolha uma foto ou digitalização da sua assinatura em papel branco.
signature-choose-image = Escolher Imagem…
# Placeholder of the field naming the signature in the library.
signature-description = Descrição, como Nome completo ou Iniciais
# Clears the drawing, typed name or image.
signature-clear = Limpar
# The color the signature is drawn or typed in.
signature-ink = Tinta
# The pen's width, for drawing.
signature-thickness = Espessura
signature-sign-first = Assine primeiro e depois salve.
# The name a signature gets when the user gives none; $number counts up from 1.
signature-default-name = Assinatura { $number }
# $error is one of the lowercase reasons below, or a technical message.
signature-change-failed = Não foi possível alterar as assinaturas: { $error }
# Reasons shown in the dialog or placed in signature-change-failed. HOME is
# the name of a setting and stays as it is.
signature-no-data-folder = nenhuma pasta de dados: HOME não está definido
signature-removing-stopped = a remoção foi interrompida
signature-saving-stopped = o salvamento foi interrompido
signature-reading-stopped = a leitura foi interrompida
signature-not-an-image = esse arquivo não é uma imagem que o prev consiga ler
signature-no-frames = a imagem não tem quadros
signature-not-found = nenhuma assinatura encontrada na imagem

## Dragging

drag-pages-need-document = As páginas podem ser soltas em um documento.
drag-image-unsupported = O prev não consegue abrir esta imagem.
# $error is a lowercase reason or a technical message.
drag-area-failed = Não foi possível arrastar a área: { $error }
drag-pages-failed = Não foi possível arrastar as páginas: { $error }
drag-start-failed = Não foi possível começar a arrastar.
# File names for pages dragged out to a file manager; ".pdf" is added
# after them. $name is the document's file name without its extension;
# $page, $first and $last are page labels, such as 3 or iv.
drag-file-pages = Páginas
drag-file-one-page = { $name } (página { $page })
drag-file-page-range = { $name } (páginas { $first }–{ $last })
# File name, before ".png", for an image annotation dragged to an image
# window's sidebar and saved in Downloads.
drag-file-image = Imagem
# Asked when PDF files are dropped on a document's page. $name is a
# file name; $count is 2 or more.
drop-pdf-title = Adicionar a este documento?
drop-pdf-body = Adicionar “{ $name }” ao final deste documento ou abri-lo em uma janela própria?
drop-pdfs-body = { $count ->
    [one] Adicionar este PDF ao final deste documento ou abri-lo em uma janela própria?
   *[other] Adicionar estes { $count } PDFs ao final deste documento ou abri-los em janelas próprias?
}
drop-pdf-add = Adicionar ao Final
drop-pdf-open = Abrir Separadamente

## PDF window

pdf-opening = Abrindo…
pdf-open-failed = O prev não consegue abrir este documento
pdf-no-pages = O documento não tem páginas.
# Shown after "Could not save:" and similar when the window closed first.
pdf-document-closed = o documento foi fechado
pdf-keep-original-failed = não foi possível manter a versão original: { $error }
pdf-save-failed = Não foi possível salvar: { $error }
pdf-nothing-to-paste = Não há nada para colar.
# Shown after an error prefix when pasting was interrupted.
pdf-pasting-stopped = a colagem foi interrompida
pdf-file-dialog-failed = Não foi possível mostrar o diálogo de arquivos: { $error }
pdf-bookmarks-no-home = Não é possível salvar os marcadores: HOME não está definido
pdf-bookmarks-save-failed = Não foi possível salvar os marcadores: { $error }
# Title of a bookmark on a page outside any table of contents entry;
# $page is the page's label, usually its number.
pdf-bookmark-page = Página { $page }

# Password prompt. $name is the file name.
pdf-password-protected = “{ $name }” está protegido por senha
pdf-password = Senha
pdf-password-wrong = Senha incorreta. Tente novamente.
# Button that opens a locked document.
pdf-unlock = Desbloquear

# Toolbar tooltips and labels.
pdf-sidebar = Barra lateral
# After the page number box: "3 of 12". $count is the number of pages.
pdf-page-of = de { $count }
pdf-zoom-out = Reduzir
pdf-zoom-in = Ampliar
# The zoom level, such as 150%.
pdf-zoom-percent = { $percent }%
pdf-fit-page = Ajustar à página
pdf-fit-width = Ajustar à largura
pdf-actual-size = Tamanho real
pdf-view-continuous = Rolagem contínua
pdf-view-single-page = Página única
pdf-view-two-pages = Duas páginas
pdf-undo = Desfazer
pdf-redo = Refazer
pdf-rotate-left = Girar para a esquerda
pdf-rotate-right = Girar para a direita
pdf-inspector = Inspetor
pdf-markup = Marcação
# Tooltip of the button that opens the export dialog.
pdf-export = Exportar
pdf-settings = Ajustes

# Search field.
pdf-search = Buscar
pdf-search-not-found = Não encontrado
pdf-searching = Buscando…
# The match shown, of all matches found.
pdf-search-match = { $current } de { $total }
# The same while the search goes on, so more matches may come.
pdf-search-match-more = { $current } de { $total }+

# Inspector: section headings.
pdf-inspector-file = Arquivo
pdf-inspector-document = Documento
pdf-inspector-pages = Páginas
# Inspector: fact labels and values.
pdf-inspector-title = Título
pdf-inspector-author = Autor
pdf-inspector-subject = Assunto
pdf-inspector-keywords = Palavras-chave
pdf-inspector-created = Criação
pdf-inspector-modified = Modificação
pdf-inspector-application = Aplicativo
pdf-inspector-producer = Produtor do PDF
pdf-inspector-version = Versão
pdf-inspector-security = Segurança
pdf-inspector-not-encrypted = Sem criptografia
# $method is the encryption method, such as AES-256.
pdf-inspector-encrypted = Criptografado ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } página
   *[other] { $count } páginas
}
pdf-inspector-page-size = Tamanho da página
# The current page's size in millimetres and inches.
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } pol)
pdf-loading = Carregando…

# Sidebar tabs and lists.
pdf-tab-pages = Páginas
pdf-tab-contents = Índice
pdf-tab-notes = Realces e notas
pdf-tab-bookmarks = Marcadores
pdf-no-outline = Sem índice
pdf-no-outline-detail = Este documento não tem índice.
pdf-no-bookmarks = Nenhum marcador
# $keys is the shortcut as the platform writes it, such as Ctrl+D.
pdf-no-bookmarks-detail = Pressione { $keys } para adicionar uma página aos marcadores.
pdf-no-bookmarks-detail-unbound = As páginas com marcador aparecem aqui.
pdf-remove-bookmark = Remover marcador

## Page editing

# Tooltip of the Pages menu button.
pages-menu = Páginas
pages-insert-blank = Inserir Página em Branco
pages-insert-file = Inserir de Arquivo…
pages-copy = { $count ->
    [one] Copiar Página
   *[other] Copiar Páginas
}
# $count is the number of pages copied earlier.
pages-paste = { $count ->
    [one] Colar Página
   *[other] Colar { $count } Páginas
}
pages-crop = Recortar para a Seleção
pages-select-all = Selecionar Todas as Páginas
pages-delete = { $count ->
    [one] Apagar Página
   *[other] Apagar Páginas
}
pages-apply-redactions = Aplicar Tarjas…
pages-no-copied = Não há páginas copiadas para colar.
pages-copied = { $count ->
    [one] { $count } página copiada.
   *[other] { $count } páginas copiadas.
}
pages-copy-failed = Não foi possível copiar as páginas: { $error }
# Shown after an error prefix when reading files was interrupted.
pages-reading-stopped = a leitura foi interrompida
# Shown after a file name when an image dropped among the pages could
# not be read.
pages-image-unreadable = não é uma imagem que o prev consiga ler
pages-read-failed = Não foi possível ler o arquivo: { $error }
pages-at-least-one = Um documento precisa de pelo menos uma página.
pages-crop-needs-area = Primeiro escolha uma área com a ferramenta de seleção retangular.
pages-change-failed = Não foi possível alterar as páginas: { $error }
pages-no-redactions = Não havia tarjas para aplicar.
pages-redactions-applied = { $count ->
    [one] { $count } tarja aplicada.
   *[other] { $count } tarjas aplicadas.
}
pages-forget-versions-failed = Não foi possível apagar as versões anteriores: { $error }
pages-redact-title = Aplicar as tarjas?
pages-redact-body = { $count ->
    [one] O texto, as imagens e os desenhos sob a marca são removidos do documento para sempre, e a marca vira uma caixa preta. Isso não pode ser desfeito, e as versões anteriores deste arquivo que o prev guarda são apagadas.
   *[other] O texto, as imagens e os desenhos sob as { $count } marcas são removidos do documento para sempre, e as marcas viram caixas pretas. Isso não pode ser desfeito, e as versões anteriores deste arquivo que o prev guarda são apagadas.
}
# Button that applies redactions.
pages-redact-apply = Aplicar

## PDF export

# Heading of the export dialog and title of its file dialog.
pages-export-title = Exportar
pages-export-format = Formato
pages-export-reduce = Reduzir o tamanho do arquivo (imagens a 150 dpi)
pages-export-flatten = Nivelar anotações e campos de formulário
pages-export-flatten-detail = As marcações e os campos preenchidos passam a fazer parte das páginas e não podem mais ser editados. As tarjas ainda não aplicadas ficam de fora.
pages-export-encrypt = Criptografar com senha
pages-export-password = Senha
pages-export-verify-password = Verificar senha
pages-export-resolution = Resolução
pages-export-dpi = { $dpi } dpi
pages-export-quality = Qualidade
# JPEG quality choices.
pages-export-quality-low = Baixa
pages-export-quality-medium = Média
pages-export-quality-high = Alta
pages-export-quality-best = Máxima
pages-export-one-file = Todas as páginas vão para um único arquivo.
pages-export-file-per-page = Cada página é salva em um arquivo próprio, numerado a partir do nome que você escolher.
pages-export-selected-only = { $count ->
    [one] Somente a página selecionada
   *[other] Somente as { $count } páginas selecionadas
}
# Button that opens the file dialog to choose where to export.
pages-export-choose = Exportar…
pages-export-no-password = Digite uma senha.
pages-export-password-mismatch = As senhas não coincidem.
# Suggested file name for a whole document exported as PDF; the
# extension is added after it.
pages-export-file-name = { $name } (exportado)
# Suggested file name when the document's own name is unknown.
pages-export-untitled = documento
pages-export-same-file = Exporte para um novo arquivo; este documento é salvo sozinho.
pages-export-exporting = Exportando “{ $name }”…
pages-export-done = “{ $name }” exportado.
pages-export-done-images = { $count } imagens exportadas.
pages-export-failed = Não foi possível exportar: { $error }
# Shown after "Could not export:" when exporting was interrupted.
pages-export-stopped = a exportação foi interrompida

## Start window

# Under the app name in a window with no file open.
app-start-hint = Abra ou solte um arquivo PDF, de imagem, SVG ou Markdown.
app-start-open = Abrir…
# A window title in a development build; $title is the usual title.
app-title-dev = { $title } (desenvolvimento)
# $kind is one of the app-kind- names below.
app-viewer-missing = { $kind }: este visualizador ainda não foi feito.
app-cannot-open = O prev não consegue abrir este tipo de arquivo.
app-cannot-read = O prev não consegue ler este arquivo: { $error }
app-kind-pdf = Documento PDF
# $format is an image format's name, such as Png or Jpeg.
app-kind-image = Imagem { $format }
app-kind-svg = Desenho SVG
app-kind-markdown = Documento Markdown
app-file-dialog-failed = Não foi possível mostrar o diálogo de arquivos: { $error }

## Actions

# Names of actions, shown beside their keyboard shortcuts.
action-open = Abrir
action-settings = Ajustes

## Toolbar

app-toolbar-keep-shown = Manter a barra de ferramentas visível
app-toolbar-auto-hide = Ocultar a barra de ferramentas quando o ponteiro sair
# The button that shows the toolbar's hidden tools.
app-toolbar-more = Mais

## File facts

# Labels in a file's inspector.
app-fact-name = Nome
app-fact-folder = Pasta
app-fact-size = Tamanho
app-fact-modified = Modificação
# File sizes; $size is a number such as 1.5 or 73.
app-size-bytes = { $count } bytes
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Link inválido { $uri }: { $error }
app-link-open-failed = Não foi possível abrir { $uri }: { $error }
# Shown after a message such as "Could not paste"; wl-clipboard is a program's name.
app-paste-needs-wl-clipboard = instale o wl-clipboard para colar imagens
app-copy-needs-wl-clipboard = instale o wl-clipboard para copiar imagens
app-copy-no-pixels = a área não tem pixels
# wl-copy is a program's name.
app-copy-no-input = o wl-copy não recebeu entrada
app-copy-failed = o wl-copy falhou
app-clipboard-open-failed = Não foi possível abrir a área de transferência: { $error }
app-copy-image-failed = Não foi possível copiar a imagem: { $error }

## Printing

print-failed = Não foi possível imprimir: { $error }
print-stopped = A impressão foi interrompida
print-unavailable = A impressão ainda não está disponível neste sistema.
print-no-window = Não foi possível imprimir: não há janela sobre a qual mostrar o diálogo de impressão
print-dialog-failed = Não foi possível mostrar o diálogo de impressão: { $error }
# Shown after "Could not print:".
print-job-not-started = a impressora não iniciou o trabalho
# Shown after "Could not print:".
print-printer-stopped = a impressora parou

## File dialogs

dialog-open = Abrir
dialog-filter-all = Todos os arquivos compatíveis
dialog-filter-pdf = Documentos PDF
dialog-filter-images = Imagens
dialog-filter-svg = Desenhos SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Escolha a pasta de assinaturas
dialog-choose-versions = Escolha a pasta do histórico de versões
dialog-choose-bookmarks = Escolha o arquivo de marcadores

## Command line

# The text of prev --help. Keep the option names (-h, --help, -V,
# --version) and "prev [FILE]..." as they are; reflow the rest freely.
usage-help =
    Uso: prev [FILE]...

    Visualiza e edita PDFs e imagens. Os arquivos abrem em janelas do prev
    em execução, que é iniciado se necessário.

    Opções:
      -h, --help     Mostra esta ajuda
      -V, --version  Mostra a versão

## Settings, continued

settings-language = Idioma
# The first entry of the language list; $language is the language that
# following the system gives, named in itself, such as English.
settings-language-system = Padrão do sistema: { $language }
# The language typed into text fields, which can differ from the
# interface's.
settings-input-language = Idioma de entrada
settings-input-language-system = Seguir o layout do teclado
settings-input-language-note = Define o lado em que um campo de texto vazio começa. O texto digitado mantém a própria direção.

settings-appearance-system = Sistema
settings-appearance-light = Claro
settings-appearance-dark = Escuro
settings-system-accent = Usar a cor de destaque do sistema
# $theme is the Omarchy theme's name.
settings-omarchy-note = As cores são criadas a partir da cor de destaque de “{ $theme }”.
settings-system-accent-note = As cores são criadas a partir da cor de destaque do sistema.
settings-system-accent-none = O sistema não tem cor de destaque, então o prev usa a cor escolhida abaixo.
settings-accent-chosen-note = As cores são criadas a partir da cor escolhida abaixo.
settings-auto-hide = Ocultar a barra de ferramentas quando o ponteiro sair
settings-auto-hide-note = A barra de ferramentas flutua sobre o documento e se recolhe enquanto o ponteiro está fora da janela.
settings-animations = Animações
settings-animations-note = Barras e painéis deslizantes, diálogos que crescem e botões com efeito de mola.
settings-animations-reduced = Desativadas enquanto o sistema pedir movimento reduzido.
settings-corner-radius = Raio dos cantos
settings-corner-radius-note = Para diálogos e a barra de ferramentas flutuante.
# $radius is a whole number of pixels.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparência da sobreposição
settings-overlay-note = Quanto da página aparece através da barra de ferramentas flutuante.
settings-overlay-value = { $percent }%
settings-storage-signatures = Pasta de assinaturas
settings-storage-versions = Pasta do histórico de versões
settings-storage-bookmarks = Arquivo de marcadores
settings-storage-apply = Aplicar
settings-storage-choose = Escolher…
# $file is where the settings file is.
settings-storage-note = Os arquivos já guardados em um local antigo continuam lá; mova-os para continuar usando. Os ajustes do prev são salvos em { $file }.
settings-save-failed = Não foi possível salvar os ajustes: { $error }
settings-no-location = Nenhum local para os ajustes: HOME não está definido
settings-full-path = Use um caminho completo, como ~/Documents/prev.
settings-path-is-folder = { $path } é uma pasta, não um arquivo.
settings-folder-missing = A pasta { $path } não existe. Crie-a primeiro ou escolha outra.
settings-path-is-file = { $path } é um arquivo, não uma pasta.
settings-cannot-write = O prev não consegue gravar em { $path }: { $error }.

## Export dialog

# Title of the export dialog, and of the save dialog it opens.
export-title = Exportar
# Section headings in the export dialog.
export-format = Formato
export-quality = Qualidade
export-size = Tamanho
# Button that goes on to choose where to save the export.
export-choose = Exportar…
# Format choice; the format name stays as it is.
export-format-webp = WebP (sem perdas)
# Stands in for a format name in a sentence when the format is unknown.
export-format-unknown = imagem
# JPEG quality choices.
export-quality-low = Baixa
export-quality-medium = Média
export-quality-high = Alta
export-quality-best = Máxima
# Size choices: the picture at its own size, or scaled up.
export-size-actual = Tamanho real
# $scale is how many times larger, such as 2.
export-size-scale = { $scale }×
# Under the size choices: the exported picture's size.
export-size-pixels = { $width } × { $height } pixels
# $error is the system's reason.
export-dialog-failed = Não foi possível mostrar o diálogo para salvar: { $error }
# $path is where the file was saved.
export-done = Exportado para { $path }
export-failed = Não foi possível exportar: { $error }
# Shown if exporting ends unexpectedly.
export-stopped = a exportação foi interrompida

## Image window

# Shown when the user tries to edit (crop, rotate, adjust) an image that has markup.
image-marked-no-edit = Imagens com marcações não podem ser editadas. Exporte para manter as marcações, ou apague-as e feche a barra de marcação.

# Shown if a background task ends unexpectedly.
image-loading-stopped = o carregamento foi interrompido
image-reverting-stopped = a reversão foi interrompida
image-rendering-stopped = a renderização foi interrompida
image-saving-stopped = o salvamento foi interrompido
image-markup-stopped = a marcação foi interrompida
image-no-version-store = Nenhum local para guardar versões
image-revert-failed = Não foi possível reverter: { $error }
image-read-failed = Não foi possível ler { $path }: { $error }
image-keep-original-failed = Não foi possível manter a versão original: { $error }
image-save-failed = Não foi possível salvar { $path }: { $error }
image-markup-start-failed = Não foi possível iniciar a marcação: { $error }
image-cannot-edit = Animações e desenhos SVG não podem ser editados.
image-cannot-mark-up = Animações e desenhos SVG não podem receber marcações.
image-mark-up-wait = Aguarde a edição terminar e depois faça as marcações.
image-crop-needs-selection = Primeiro arraste uma seleção (ferramenta Selecionar) e depois recorte.
image-size-needed = Digite uma largura e uma altura em pixels.
# $name is a file name.
image-cannot-save-format = As alterações em “{ $name }” não podem ser salvas no formato dele. Use Exportar ({ $keys }).
image-cannot-save-format-unbound = As alterações em “{ $name }” não podem ser salvas no formato dele. Use Exportar.
image-cannot-export-animation = Ainda não é possível exportar animações.
image-drop-pages = As páginas podem ser soltas em um documento.
image-drag-failed = Não foi possível começar a arrastar.
image-picture-save-failed = Não foi possível salvar a imagem na sua pasta Downloads.
image-open-failed = O prev não consegue abrir esta imagem
image-opening = Abrindo…
# Asked when an export's file name has another format's extension.
image-name-mismatch-title = O nome não corresponde ao formato
# $name is the file name, $format a format name such as PNG, $extension
# the name's extension without the dot.
image-name-mismatch = “{ $name }” será salvo como arquivo { $format }, mas o nome termina em .{ $extension }. Outros apps podem não abri-lo.
image-name-mismatch-no-extension = “{ $name }” será salvo como arquivo { $format }, mas o nome não tem extensão. Outros apps podem não abri-lo.
image-choose-again = Escolher Novamente
image-save-as-is = Salvar Assim Mesmo
# Toolbar details, shown one after another: the image's size in pixels,
# the frame of an animation, the image's place among the window's, and
# whether it has unsaved edits.
image-dimensions = { $width } × { $height }
image-frame-position = quadro { $current } de { $total }
image-position = { $current } de { $total }
image-edited = editada
# Toolbar tooltips.
image-sidebar = Barra lateral
image-zoom-out = Reduzir
image-zoom-in = Ampliar
image-zoom = { $percent }%
image-fit = Ajustar à janela
image-actual-size = Tamanho real
image-undo = Desfazer
image-redo = Refazer
image-rotate-left = Girar para a esquerda
image-rotate-right = Girar para a direita
image-flip-horizontal = Inverter horizontalmente
image-flip-vertical = Inverter verticalmente
image-select = Seleção retangular
image-crop = Recortar para a seleção
image-adjust-size-tool = Ajustar tamanho
image-adjust-color-tool = Ajustar cor
# Tooltip and panel title.
image-inspector = Inspetor
image-markup = Marcação
image-export = Exportar
image-settings = Ajustes
# Panel titles.
image-adjust-color = Ajustar Cor
image-adjust-size = Ajustar Tamanho
# Adjust Color sliders.
image-exposure = Exposição
image-contrast = Contraste
image-saturation = Saturação
image-temperature = Temperatura
image-tint = Tonalidade
image-sepia = Sépia
image-sharpness = Nitidez
image-levels = Níveis
image-black-point = Ponto preto
image-midtones = Meios-tons
image-white-point = Ponto branco
image-reset-all = Redefinir Tudo
# Adjust Size panel.
image-current-size = Tamanho atual: { $width } × { $height } pixels
image-width = Largura
image-height = Altura
image-scale-proportionally = Redimensionar proporcionalmente
# Button that applies the new size.
image-resize = Redimensionar
# Inspector panel.
image-inspector-loading = Carregando…
image-file = Arquivo
image-format = Formato
image-dimensions-label = Dimensões
image-pixels = { $width } × { $height } pixels
image-no-camera = Nenhuma informação da câmera.
image-location = Localização
image-remove-location = Remover Informações de Localização
image-no-location = Nenhuma informação de localização.
image-keywords-description = Palavras-chave e Descrição
image-keywords-hint = Palavras-chave, separadas por vírgulas
image-description = Descrição
image-keywords-unsupported = As palavras-chave podem ser salvas em arquivos JPEG, PNG e WebP.
# Heading over the earlier versions of the file.
image-revert-to = Reverter Para
image-no-versions = Nenhuma versão anterior.
image-revert = Reverter
# A version's file size; $size is a number such as 512 or 1.5.
image-size-kb = { $size } KB
image-size-mb = { $size } MB
# Asked when closing a window whose markup was not exported. $count is
# how many images have such markup.
image-close-title = Fechar sem exportar as marcações?
image-close-body = { $count ->
    [one] As marcações em uma imagem só duram enquanto a janela dela estiver aberta. Exporte a imagem para mantê-las: as marcações são desenhadas na cópia que você salvar.
   *[other] As marcações em imagens só duram enquanto a janela delas estiver aberta. Exporte cada imagem para mantê-las: as marcações são desenhadas na cópia que você salvar.
}
image-close-anyway = Fechar Mesmo Assim

## Markdown

# Shown if reading the file ends unexpectedly.
markdown-reading-stopped = a leitura foi interrompida
markdown-read-failed = O prev não consegue ler este arquivo
markdown-draw-failed = Não foi possível desenhar o documento
# Under the export's size choices.
markdown-export-size = O documento inteiro, { $width } × { $height } pixels
# Search results.
markdown-not-found = Não encontrado
markdown-match = { $current } de { $total }
# Placeholder of the search field.
markdown-search = Buscar
# Toolbar tooltips.
markdown-smaller-text = Texto menor
markdown-larger-text = Texto maior
markdown-zoom = { $percent }%
markdown-actual-size = Tamanho real
# Tooltip and panel title.
markdown-inspector = Inspetor
markdown-export = Exportar
markdown-settings = Ajustes
# Inspector headings and labels.
markdown-file = Arquivo
markdown-document = Documento
markdown-words = Palavras
markdown-lines = Linhas
markdown-pictures = Imagens

## Image details
# The camera's data in the image inspector: group headings, field names
# and the values prev words itself.

image-meta-camera = Câmera
image-meta-exposure = Exposição
image-meta-image = Imagem
image-meta-make = Fabricante
image-meta-model = Modelo
image-meta-lens = Lente
image-meta-exposure-time = Tempo de exposição
# The lens aperture, written like f/2.8.
image-meta-f-number = Número f
image-meta-iso = ISO
image-meta-focal-length = Distância focal
image-meta-exposure-bias = Compensação de exposição
image-meta-flash = Flash
image-meta-date-taken = Data da captura
image-meta-orientation = Orientação
image-meta-color-space = Espaço de cor
image-meta-software = Software
image-meta-artist = Artista
image-meta-copyright = Direitos autorais
# $value is a number or fraction of seconds, such as 1/200.
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
# How the camera was held; $value is the EXIF orientation, 1 to 8.
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Espelhada horizontalmente
    [3] Girada 180°
    [4] Espelhada verticalmente
    [5] Espelhada horizontalmente, girada 90° no sentido anti-horário
    [6] Girada 90° no sentido horário
    [7] Espelhada horizontalmente, girada 90° no sentido horário
    [8] Girada 90° no sentido anti-horário
   *[other] Desconhecida ({ $value })
}
# $fired is yes or no; $mode is on (forced), off, auto or unknown;
# $redeye is yes when red-eye reduction was used.
image-meta-flash-value = { $fired ->
    [yes] Disparado
   *[no] Não disparado
}{ $mode ->
    [on] , forçado
    [off] , desligado
    [auto] , automático
   *[unknown] {""}
}{ $redeye ->
    [yes] , redução de olhos vermelhos
   *[no] {""}
}
# $space names the color space; $code is its EXIF number, for other ones.
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Não calibrado
   *[other] Outro ({ $code })
}

## Errors
# Reasons from prev's PDF, image and settings code. They are shown inside
# other messages, after a colon, such as "Could not save: { $error }", so
# they start in lower case. $detail is the underlying error, as MuPDF or
# the system gives it, often in English.

error-pdf-open = não é possível abrir o documento: { $detail }
error-pdf-page-out-of-range = a página { $page } não existe
error-pdf-password-protected = o documento está protegido por senha; abra-o e copie as páginas dele
error-pdf-no-pages = não há páginas para extrair
error-pdf-crop-outside = a área de recorte está fora da página
error-pdf-closed = documento fechado
error-pdf-saved-unreadable = o documento salvo não abre mais
error-image-read = não é possível ler o arquivo: { $detail }
error-image-invalid = a imagem está danificada ou é inválida: { $detail }
# $library is a program name, such as libheif.
error-image-missing-library = abrir este formato requer { $library }, que não está instalado
# $format is an image format name, such as HEIC.
error-image-unsupported = imagens { $format } ainda não são compatíveis
error-image-encode = não é possível codificar a imagem: { $detail }
error-exif-malformed = os dados EXIF estão malformados
error-settings-read = não é possível ler os ajustes: { $detail }
error-settings-invalid = ajustes inválidos: { $detail }
error-remove-location = não foi possível remover a localização: { $error }
error-location-unsupported = as informações de localização podem ser removidas de arquivos JPEG, PNG, WebP e TIFF
error-xmp-unsupported = palavras-chave e descrições só podem ser salvas em arquivos JPEG, PNG e WebP

## Formats
# Image format names are the same in every language; this one has a word.

format-camera-raw = RAW de câmera

## The macOS menu bar, named as in macOS's own apps.
menu-about = Sobre o prev
menu-settings = Ajustes…
menu-services = Serviços
menu-hide = Ocultar prev
menu-hide-others = Ocultar Outros
menu-show-all = Mostrar Tudo
menu-quit = Encerrar prev
menu-file = Arquivo
menu-open = Abrir…
menu-close = Fechar Janela
menu-export = Exportar…
menu-print = Imprimir…
menu-edit = Editar
menu-undo = Desfazer
menu-redo = Refazer
menu-cut = Recortar
menu-copy = Copiar
menu-paste = Colar
menu-select-all = Selecionar Tudo
menu-find = Buscar
menu-find-next = Buscar Seguinte
menu-find-previous = Buscar Anterior
menu-view = Visualizar
menu-hide-sidebar = Ocultar Barra Lateral
menu-thumbnails = Miniaturas
menu-contents = Índice
menu-notes = Realces e Notas
menu-bookmarks = Marcadores
menu-zoom-in = Ampliar
menu-zoom-out = Reduzir
menu-actual-size = Tamanho Real
menu-zoom-to-fit = Ajustar à Janela
menu-inspector = Mostrar Inspetor
menu-slideshow = Apresentação de Slides
menu-full-screen = Entrar em Tela Cheia
menu-go = Ir
menu-next-page = Próxima Página
menu-previous-page = Página Anterior
menu-go-to-page = Ir para a Página…
menu-bookmark = Adicionar Marcador
menu-tools = Ferramentas
menu-markup = Mostrar Barra de Ferramentas de Marcação
menu-rotate-left = Girar para a Esquerda
menu-rotate-right = Girar para a Direita
menu-crop = Recortar
menu-adjust-color = Ajustar Cor…
menu-window = Janela
menu-minimize = Minimizar
menu-zoom = Zoom
menu-bring-all-to-front = Trazer Todas para a Frente
