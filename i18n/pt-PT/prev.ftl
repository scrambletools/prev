# prev's interface text in European Portuguese (Português (Portugal)), translated from i18n/en/prev.ftl.
#
# A first draft, open to further review. Keys, section headings and `{ $name }` values
# stay as in English; only the text after `=` is translated.
#
# Terms used throughout: markup = marcações, note = nota, highlight = realce,
# annotation = anotação, redact/redaction = rasurar/rasura (strikethrough = riscado,
# to keep it apart), inspector = inspetor,
# zoom in/out = ampliar/reduzir, bookmark = marcador, page = página,
# file = ficheiro, settings = definições, password = palavra-passe.
# Buttons and menu items use the infinitive (Guardar, Cancelar, Fechar);
# messages address the user with the third-person imperative (Introduza, Escolha).

## Language

language-name = Português (Portugal)

## Common

common-cancel = Cancelar
common-close = Fechar
common-save = Guardar

## Settings

settings-title = Definições
settings-appearance = Aspeto
settings-colors = Cores
settings-windows = Janelas
settings-default-app = Aplicação predefinida
settings-default-app-label = Abrir ficheiros com o prev
settings-default-app-note = Torne o prev a aplicação que abre PDF, imagens, desenhos SVG e ficheiros Markdown.
settings-default-app-note-windows = O Windows só permite escolher as aplicações predefinidas nas suas próprias Definições. Este botão abre lá a página do prev.
settings-default-app-note-macos = O macOS pede que confirme cada tipo: PDF, PNG, JPEG, HEIC, GIF, TIFF, WebP e AVIF.
settings-default-app-status = { $set } de { $total } tipos de ficheiro abrem com o prev.
settings-default-app-button = Tornar predefinida
settings-default-app-button-windows = Abrir Definições
settings-default-app-no-entry = A entrada de ambiente de trabalho do prev não está instalada, pelo que o sistema não consegue abrir ficheiros com ele. Instale o prev a partir de um pacote ou com scripts/install.sh.
settings-default-app-no-bundle = Abra o prev a partir de prev.app para o tornar predefinido.
settings-default-app-failed = Não foi possível tornar o prev predefinido: { $error }
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
markup-tool-highlight = Realçar
markup-tool-note = Nota
markup-tool-sign = Assinar
markup-tool-redact = Rasurar
markup-apply = Aplicar
markup-apply-redactions = Aplicar rasuras
markup-shape-style = Estilo da forma
markup-border-color = Cor do contorno
markup-fill-color = Cor de preenchimento
markup-text-style = Estilo do texto
markup-delete = Eliminar
markup-undo = Anular
markup-redo = Refazer

## Markup menus

markup-shape-rectangle = Retângulo
markup-shape-rounded-rectangle = Retângulo arredondado
markup-shape-oval = Oval
markup-shape-line = Linha
markup-shape-arrow = Seta
markup-shape-star = Estrela
markup-shape-polygon = Polígono
markup-shape-speech-bubble = Balão de fala
markup-shape-loupe = Lupa
markup-shape-mask = Máscara
markup-style-highlight = Realce
markup-style-underline = Sublinhado
markup-style-strikethrough = Riscado
markup-style-squiggly = Ondulado
markup-menu-color = Cor
markup-menu-font = Tipo de letra
markup-menu-size = Tamanho
markup-menu-alignment = Alinhamento
markup-line-width = { $width } pt
markup-dashed = Tracejada

## Notes

markup-kind-note = Nota
markup-kind-text-box = Caixa de texto
markup-kind-stamp = Carimbo
markup-kind-redaction = Rasura
markup-kind-shape = Forma
markup-note-delete = Eliminar nota
markup-note-done = Concluído
markup-note-placeholder = Escreva uma nota
markup-notes-empty = Sem realces nem notas
markup-notes-empty-hint = Os realces, as notas e as caixas de texto aparecem aqui.
markup-notes-page = Página { $page }

## Markup errors

markup-change-failed = Não foi possível alterar o documento: { $error }
markup-copy-area-failed = Não foi possível copiar a área: { $error }
markup-document-closed = o documento foi fechado
markup-render-area-failed = não foi possível compor a área
markup-copy-stopped = a cópia foi interrompida

## Signatures

signature-menu-empty = Ainda não há assinaturas.
signature-delete = Eliminar assinatura
signature-create = Criar assinatura…
signature-dialog-title = Criar assinatura
signature-tab-draw = Desenhar
signature-tab-type = Escrever
signature-tab-image = Imagem
signature-draw-hint = Assine sobre a linha com o rato, a caneta ou o painel tátil.
signature-your-name = O seu nome
signature-image-hint = Escolha uma fotografia ou digitalização da sua assinatura em papel branco.
signature-choose-image = Escolher imagem…
signature-description = Descrição, por exemplo Nome completo ou Iniciais
signature-clear = Limpar
signature-ink = Tinta
signature-thickness = Espessura
signature-sign-first = Assine primeiro e depois guarde.
signature-default-name = Assinatura { $number }
signature-change-failed = Não foi possível alterar as assinaturas: { $error }
signature-no-data-folder = não há pasta de dados: HOME não está definido
signature-removing-stopped = a remoção foi interrompida
signature-saving-stopped = a gravação foi interrompida
signature-reading-stopped = a leitura foi interrompida
signature-not-an-image = esse ficheiro não é uma imagem que o prev consiga ler
signature-no-frames = a imagem não tem fotogramas
signature-not-found = não foi encontrada nenhuma assinatura na imagem

## Dragging

drag-pages-need-document = As páginas podem ser largadas num documento.
drag-image-unsupported = O prev não consegue abrir esta imagem.
drag-area-failed = Não foi possível arrastar a área: { $error }
drag-pages-failed = Não foi possível arrastar as páginas: { $error }
drag-start-failed = Não foi possível começar a arrastar.
drag-file-pages = Páginas
drag-file-one-page = { $name } (página { $page })
drag-file-page-range = { $name } (páginas { $first }–{ $last })
drag-file-image = Imagem
drop-pdf-title = Adicionar a este documento?
drop-pdf-body = Adicionar «{ $name }» ao fim deste documento ou abri-lo numa janela própria?
drop-pdfs-body = { $count ->
    [one] Adicionar este PDF ao fim deste documento ou abri-lo numa janela própria?
   *[other] Adicionar estes { $count } PDF ao fim deste documento ou abri-los em janelas próprias?
}
drop-pdf-add = Adicionar ao fim
drop-pdf-open = Abrir em separado

## PDF window

pdf-opening = A abrir…
pdf-open-failed = O prev não consegue abrir este documento
pdf-no-pages = O documento não tem páginas.
pdf-document-closed = o documento foi fechado
pdf-keep-original-failed = não foi possível manter a versão original: { $error }
pdf-save-failed = Não foi possível guardar: { $error }
pdf-nothing-to-paste = Não há nada para colar.
pdf-pasting-stopped = a colagem foi interrompida
pdf-file-dialog-failed = Não foi possível mostrar a caixa de diálogo de ficheiros: { $error }
pdf-bookmarks-no-home = Não é possível guardar os marcadores: HOME não está definido
pdf-bookmarks-save-failed = Não foi possível guardar os marcadores: { $error }
pdf-bookmark-page = Página { $page }

pdf-password-protected = «{ $name }» está protegido por palavra-passe
pdf-password = Palavra-passe
pdf-password-wrong = Palavra-passe incorreta. Tente novamente.
pdf-unlock = Desbloquear

pdf-sidebar = Barra lateral
pdf-page-of = de { $count }
pdf-zoom-out = Reduzir
pdf-zoom-in = Ampliar
pdf-zoom-percent = { $percent }%
pdf-fit-page = Ajustar à página
pdf-fit-width = Ajustar à largura
pdf-actual-size = Tamanho real
pdf-view-continuous = Deslocamento contínuo
pdf-view-single-page = Página única
pdf-view-two-pages = Duas páginas
pdf-undo = Anular
pdf-redo = Refazer
pdf-rotate-left = Rodar para a esquerda
pdf-rotate-right = Rodar para a direita
pdf-inspector = Inspetor
pdf-markup = Marcações
pdf-export = Exportar
pdf-settings = Definições

pdf-search = Pesquisar
pdf-search-not-found = Não encontrado
pdf-searching = A pesquisar…
pdf-search-match = { $current } de { $total }
pdf-search-match-more = { $current } de { $total }+

pdf-inspector-file = Ficheiro
pdf-inspector-document = Documento
pdf-inspector-pages = Páginas
pdf-inspector-title = Título
pdf-inspector-author = Autor
pdf-inspector-subject = Assunto
pdf-inspector-keywords = Palavras-chave
pdf-inspector-created = Criação
pdf-inspector-modified = Modificação
pdf-inspector-application = Aplicação
pdf-inspector-producer = Produtor do PDF
pdf-inspector-version = Versão
pdf-inspector-security = Segurança
pdf-inspector-not-encrypted = Não encriptado
pdf-inspector-encrypted = Encriptado ({ $method })
pdf-inspector-page-count = { $count ->
    [one] { $count } página
   *[other] { $count } páginas
}
pdf-inspector-page-size = Tamanho da página
pdf-inspector-page-size-value = { $width_mm } × { $height_mm } mm ({ $width_in } × { $height_in } pol.)
pdf-loading = A carregar…

pdf-tab-pages = Páginas
pdf-tab-contents = Índice
pdf-tab-notes = Realces e notas
pdf-tab-bookmarks = Marcadores
pdf-no-outline = Sem índice
pdf-no-outline-detail = Este documento não tem índice.
pdf-no-bookmarks = Sem marcadores
pdf-no-bookmarks-detail = Prima { $keys } para adicionar um marcador a uma página.
pdf-no-bookmarks-detail-unbound = As páginas com marcador aparecem aqui.
pdf-remove-bookmark = Remover marcador

## Page editing

pages-menu = Páginas
pages-insert-blank = Inserir página em branco
pages-insert-file = Inserir a partir de ficheiro…
pages-copy = { $count ->
    [one] Copiar página
   *[other] Copiar páginas
}
pages-paste = { $count ->
    [one] Colar página
   *[other] Colar { $count } páginas
}
pages-crop = Recortar à seleção
pages-select-all = Selecionar todas as páginas
pages-delete = { $count ->
    [one] Eliminar página
   *[other] Eliminar páginas
}
pages-apply-redactions = Aplicar rasuras…
pages-no-copied = Não há páginas copiadas para colar.
pages-copied = { $count ->
    [one] { $count } página copiada.
   *[other] { $count } páginas copiadas.
}
pages-copy-failed = Não foi possível copiar as páginas: { $error }
pages-reading-stopped = a leitura foi interrompida
pages-image-unreadable = não é uma imagem que o prev consiga ler
pages-read-failed = Não foi possível ler o ficheiro: { $error }
pages-at-least-one = Um documento precisa de ter pelo menos uma página.
pages-crop-needs-area = Escolha primeiro uma área com a ferramenta de seleção retangular.
pages-change-failed = Não foi possível alterar as páginas: { $error }
pages-no-redactions = Não havia rasuras para aplicar.
pages-redactions-applied = { $count ->
    [one] { $count } rasura aplicada.
   *[other] { $count } rasuras aplicadas.
}
pages-forget-versions-failed = Não foi possível eliminar as versões anteriores: { $error }
pages-redact-title = Aplicar as rasuras?
pages-redact-body = { $count ->
    [one] O texto, as imagens e os desenhos sob a marca são removidos definitivamente do documento, e a marca passa a ser uma caixa preta. Esta ação não pode ser anulada, e as versões anteriores deste ficheiro que o prev guarda são eliminadas.
   *[other] O texto, as imagens e os desenhos sob as { $count } marcas são removidos definitivamente do documento, e as marcas passam a ser caixas pretas. Esta ação não pode ser anulada, e as versões anteriores deste ficheiro que o prev guarda são eliminadas.
}
pages-redact-apply = Aplicar

## PDF export

pages-export-title = Exportar
pages-export-format = Formato
pages-export-reduce = Reduzir o tamanho do ficheiro (imagens a 150 ppp)
pages-export-flatten = Achatar anotações e campos de formulário
pages-export-flatten-detail = As marcações e os campos preenchidos passam a fazer parte das páginas e deixam de poder ser editados. As rasuras ainda não aplicadas ficam de fora.
pages-export-encrypt = Encriptar com palavra-passe
pages-export-password = Palavra-passe
pages-export-verify-password = Confirmar palavra-passe
pages-export-resolution = Resolução
pages-export-dpi = { $dpi } ppp
pages-export-quality = Qualidade
pages-export-quality-low = Baixa
pages-export-quality-medium = Média
pages-export-quality-high = Alta
pages-export-quality-best = Máxima
pages-export-one-file = Todas as páginas vão para um único ficheiro.
pages-export-file-per-page = Cada página é guardada num ficheiro próprio, numerado a partir do nome que escolher.
pages-export-selected-only = { $count ->
    [one] Apenas a página selecionada
   *[other] Apenas as { $count } páginas selecionadas
}
pages-export-choose = Exportar…
pages-export-no-password = Introduza uma palavra-passe.
pages-export-password-mismatch = As palavras-passe não coincidem.
pages-export-file-name = { $name } (exportado)
pages-export-untitled = documento
pages-export-same-file = Exporte para um novo ficheiro; este documento é guardado automaticamente.
pages-export-exporting = A exportar «{ $name }»…
pages-export-done = «{ $name }» exportado.
pages-export-done-images = { $count ->
    [one] { $count } imagem exportada.
   *[other] { $count } imagens exportadas.
}
pages-export-failed = Não foi possível exportar: { $error }
pages-export-stopped = a exportação foi interrompida

## Start window

app-start-hint = Abra ou largue aqui um ficheiro PDF, de imagem, SVG ou Markdown.
app-start-open = Abrir…
app-title-dev = { $title } (desenvolvimento)
app-viewer-missing = { $kind }: este visualizador ainda não está feito.
app-cannot-open = O prev não consegue abrir este tipo de ficheiro.
app-cannot-read = O prev não consegue ler este ficheiro: { $error }
app-kind-pdf = Documento PDF
app-kind-image = Imagem { $format }
app-kind-svg = Desenho SVG
app-kind-markdown = Documento Markdown
app-file-dialog-failed = Não foi possível mostrar a caixa de diálogo de ficheiros: { $error }

## Actions

action-open = Abrir
action-settings = Definições

## Toolbar

app-toolbar-keep-shown = Manter a barra de ferramentas visível
app-toolbar-auto-hide = Ocultar a barra de ferramentas quando o ponteiro sai
app-toolbar-more = Mais

## File facts

app-fact-name = Nome
app-fact-folder = Pasta
app-fact-size = Tamanho
app-fact-modified = Modificação
app-size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } bytes
}
app-size-kb = { $size } KB
app-size-mb = { $size } MB
app-size-gb = { $size } GB
app-size-tb = { $size } TB

## Links and clipboard

app-link-invalid = Ligação inválida { $uri }: { $error }
app-link-open-failed = Não foi possível abrir { $uri }: { $error }
app-paste-needs-wl-clipboard = instale o wl-clipboard para colar imagens
app-copy-needs-wl-clipboard = instale o wl-clipboard para copiar imagens
app-copy-no-pixels = a área não tem píxeis
app-copy-no-input = o wl-copy não recebeu dados
app-copy-failed = o wl-copy falhou
app-clipboard-open-failed = Não foi possível abrir a área de transferência: { $error }
app-copy-image-failed = Não foi possível copiar a imagem: { $error }

## Printing

print-failed = Não foi possível imprimir: { $error }
print-stopped = A impressão foi interrompida
print-unavailable = A impressão ainda não está disponível neste sistema.
print-no-window = Não foi possível imprimir: não há nenhuma janela sobre a qual mostrar a caixa de diálogo de impressão
print-dialog-failed = Não foi possível mostrar a caixa de diálogo de impressão: { $error }
print-job-not-started = a impressora não iniciou o trabalho
print-printer-stopped = a impressora parou

## File dialogs

dialog-open = Abrir
dialog-filter-all = Todos os ficheiros suportados
dialog-filter-pdf = Documentos PDF
dialog-filter-images = Imagens
dialog-filter-svg = Desenhos SVG
dialog-filter-markdown = Markdown
dialog-choose-signatures = Escolha a pasta das assinaturas
dialog-choose-versions = Escolha a pasta do histórico de versões
dialog-choose-bookmarks = Escolha o ficheiro de marcadores

## Command line

usage-help =
    Utilização: prev [FILE]...
                prev --mcp

    Visualiza e edita PDF e imagens. Os ficheiros abrem em janelas do prev
    em execução, que é iniciado se for preciso.

    Opções:
      -h, --help     Mostra esta ajuda
      -V, --version  Mostra a versão
          --mcp      Serve MCP por stdin e stdout, para que agentes de IA controlem
                     o prev em execução

## Settings, continued

settings-language = Idioma
settings-language-system = Predefinição do sistema: { $language }
settings-input-language = Idioma de introdução
settings-input-language-system = Seguir o esquema de teclado
settings-input-language-note = Define o lado em que começa um campo de texto vazio. O texto escrito mantém a sua própria direção.

settings-appearance-system = Sistema
settings-appearance-light = Claro
settings-appearance-dark = Escuro
settings-system-accent = Utilizar a cor de destaque do sistema
settings-omarchy-note = As cores são criadas a partir da cor de destaque de «{ $theme }».
settings-system-accent-note = As cores são criadas a partir da cor de destaque do sistema.
settings-system-accent-none = O sistema não tem cor de destaque, pelo que o prev utiliza a cor escolhida abaixo.
settings-accent-chosen-note = As cores são criadas a partir da cor escolhida abaixo.
settings-auto-hide = Ocultar a barra de ferramentas quando o ponteiro sai
settings-auto-hide-note = A barra de ferramentas flutua sobre o documento e recolhe-se enquanto o ponteiro está fora da janela.
settings-animations = Animações
settings-animations-note = Barras e painéis deslizantes, caixas de diálogo que crescem e botões com efeito de mola.
settings-animations-reduced = Desativadas enquanto o sistema pedir movimento reduzido.
settings-corner-radius = Raio dos cantos
settings-corner-radius-note = Para caixas de diálogo e a barra de ferramentas flutuante.
settings-corner-radius-value = { $radius } px
settings-overlay = Transparência da sobreposição
settings-overlay-note = Quanto da página se vê através da barra de ferramentas flutuante.
settings-overlay-value = { $percent }%
settings-storage-signatures = Pasta das assinaturas
settings-storage-versions = Pasta do histórico de versões
settings-storage-bookmarks = Ficheiro de marcadores
settings-storage-apply = Aplicar
settings-storage-choose = Escolher…
settings-storage-note = Os ficheiros já guardados num local antigo ficam lá; mova-os para continuar a utilizá-los. As definições do prev são guardadas em { $file }.
settings-save-failed = Não foi possível guardar as definições: { $error }
settings-no-location = Não há local para as definições: HOME não está definido
settings-full-path = Utilize um caminho completo, por exemplo ~/Documents/prev.
settings-path-is-folder = { $path } é uma pasta, não um ficheiro.
settings-folder-missing = A pasta { $path } não existe. Crie-a primeiro ou escolha outra.
settings-path-is-file = { $path } é um ficheiro, não uma pasta.
settings-cannot-write = O prev não consegue escrever em { $path }: { $error }.

## Export dialog

export-title = Exportar
export-format = Formato
export-quality = Qualidade
export-size = Tamanho
export-choose = Exportar…
export-format-webp = WebP (sem perdas)
export-format-unknown = imagem
export-quality-low = Baixa
export-quality-medium = Média
export-quality-high = Alta
export-quality-best = Máxima
export-size-actual = Tamanho real
export-size-scale = { $scale }×
export-size-pixels = { $width } × { $height } píxeis
export-dialog-failed = Não foi possível mostrar a caixa de diálogo para guardar: { $error }
export-done = Exportado para { $path }
export-failed = Não foi possível exportar: { $error }
export-stopped = a exportação foi interrompida

## Image window

image-marked-no-edit = As imagens com marcações não podem ser editadas. Exporte a imagem para manter as marcações, ou elimine-as e feche a barra de marcações.

image-loading-stopped = o carregamento foi interrompido
image-reverting-stopped = a reversão foi interrompida
image-rendering-stopped = a composição foi interrompida
image-saving-stopped = a gravação foi interrompida
image-markup-stopped = a marcação foi interrompida
image-no-version-store = Não há local para guardar versões
image-revert-failed = Não foi possível reverter: { $error }
image-read-failed = Não foi possível ler { $path }: { $error }
image-keep-original-failed = Não foi possível manter a versão original: { $error }
image-save-failed = Não foi possível guardar { $path }: { $error }
image-markup-start-failed = Não foi possível iniciar a marcação: { $error }
image-cannot-edit = As animações e os desenhos SVG não podem ser editados.
image-cannot-mark-up = As animações e os desenhos SVG não podem receber marcações.
image-mark-up-wait = Aguarde que a edição termine e depois adicione as marcações.
image-crop-needs-selection = Arraste primeiro uma seleção (ferramenta Selecionar) e depois recorte.
image-size-needed = Introduza uma largura e uma altura em píxeis.
image-cannot-save-format = As alterações a «{ $name }» não podem ser guardadas no formato do ficheiro. Utilize Exportar ({ $keys }).
image-cannot-save-format-unbound = As alterações a «{ $name }» não podem ser guardadas no formato do ficheiro. Utilize Exportar.
image-cannot-export-animation = Ainda não é possível exportar animações.
image-drop-pages = As páginas podem ser largadas num documento.
image-drag-failed = Não foi possível começar a arrastar.
image-picture-save-failed = Não foi possível guardar a imagem na pasta Transferências.
image-open-failed = O prev não consegue abrir esta imagem
image-opening = A abrir…
image-name-mismatch-title = O nome não corresponde ao formato
image-name-mismatch = «{ $name }» vai ser guardado como ficheiro { $format }, mas o nome termina em .{ $extension }. Outras aplicações podem não conseguir abri-lo.
image-name-mismatch-no-extension = «{ $name }» vai ser guardado como ficheiro { $format }, mas o nome não tem extensão. Outras aplicações podem não conseguir abri-lo.
image-choose-again = Escolher novamente
image-save-as-is = Guardar assim
image-dimensions = { $width } × { $height }
image-frame-position = fotograma { $current } de { $total }
image-position = { $current } de { $total }
image-edited = editada
image-sidebar = Barra lateral
image-zoom-out = Reduzir
image-zoom-in = Ampliar
image-zoom = { $percent }%
image-fit = Ajustar à janela
image-actual-size = Tamanho real
image-undo = Anular
image-redo = Refazer
image-rotate-left = Rodar para a esquerda
image-rotate-right = Rodar para a direita
image-flip-horizontal = Inverter na horizontal
image-flip-vertical = Inverter na vertical
image-select = Seleção retangular
image-crop = Recortar à seleção
image-adjust-size-tool = Ajustar tamanho
image-adjust-color-tool = Ajustar cor
image-inspector = Inspetor
image-markup = Marcações
image-export = Exportar
image-settings = Definições
image-adjust-color = Ajustar cor
image-adjust-size = Ajustar tamanho
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
image-reset-all = Repor tudo
image-current-size = Tamanho atual: { $width } × { $height } píxeis
image-width = Largura
image-height = Altura
image-scale-proportionally = Redimensionar proporcionalmente
image-resize = Redimensionar
image-inspector-loading = A carregar…
image-file = Ficheiro
image-format = Formato
image-dimensions-label = Dimensões
image-pixels = { $width } × { $height } píxeis
image-no-camera = Sem informações da câmara.
image-location = Localização
image-remove-location = Remover informações de localização
image-no-location = Sem informações de localização.
image-keywords-description = Palavras-chave e descrição
image-keywords-hint = Palavras-chave, separadas por vírgulas
image-description = Descrição
image-keywords-unsupported = As palavras-chave podem ser guardadas em ficheiros JPEG, PNG e WebP.
image-revert-to = Reverter para
image-no-versions = Sem versões anteriores.
image-revert = Reverter
image-size-kb = { $size } KB
image-size-mb = { $size } MB
image-close-title = Fechar sem exportar as marcações?
image-close-body = { $count ->
    [one] As marcações numa imagem só duram enquanto a respetiva janela estiver aberta. Exporte a imagem para as manter: as marcações são desenhadas na cópia que guardar.
   *[other] As marcações nas imagens só duram enquanto a respetiva janela estiver aberta. Exporte cada imagem para as manter: as marcações são desenhadas na cópia que guardar.
}
image-close-anyway = Fechar mesmo assim

## Markdown

markdown-reading-stopped = a leitura foi interrompida
markdown-read-failed = O prev não consegue ler este ficheiro
markdown-draw-failed = Não foi possível desenhar o documento
markdown-export-size = O documento inteiro, { $width } × { $height } píxeis
markdown-not-found = Não encontrado
markdown-match = { $current } de { $total }
markdown-search = Pesquisar
markdown-smaller-text = Texto mais pequeno
markdown-larger-text = Texto maior
markdown-zoom = { $percent }%
markdown-actual-size = Tamanho real
markdown-inspector = Inspetor
markdown-export = Exportar
markdown-settings = Definições
markdown-file = Ficheiro
markdown-document = Documento
markdown-words = Palavras
markdown-lines = Linhas
markdown-pictures = Imagens

## Image details

image-meta-camera = Câmara
image-meta-exposure = Exposição
image-meta-image = Imagem
image-meta-make = Fabricante
image-meta-model = Modelo
image-meta-lens = Objetiva
image-meta-exposure-time = Tempo de exposição
image-meta-f-number = Número f
image-meta-iso = ISO
image-meta-focal-length = Distância focal
image-meta-exposure-bias = Compensação da exposição
image-meta-flash = Flash
image-meta-date-taken = Data da fotografia
image-meta-orientation = Orientação
image-meta-color-space = Espaço de cor
image-meta-software = Software
image-meta-artist = Autor
image-meta-copyright = Direitos de autor
image-meta-seconds = { $value } s
image-meta-millimeters = { $value } mm
image-meta-ev = { $value } EV
image-meta-orientation-value = { $value ->
    [1] Normal
    [2] Espelhada na horizontal
    [3] Rodada 180°
    [4] Espelhada na vertical
    [5] Espelhada na horizontal, rodada 90° no sentido anti-horário
    [6] Rodada 90° no sentido horário
    [7] Espelhada na horizontal, rodada 90° no sentido horário
    [8] Rodada 90° no sentido anti-horário
   *[other] Desconhecida ({ $value })
}
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
image-meta-color-space-value = { $space ->
    [srgb] sRGB
    [adobe] Adobe RGB
    [uncalibrated] Não calibrado
   *[other] Outro ({ $code })
}

## Errors

error-pdf-open = não é possível abrir o documento: { $detail }
error-pdf-page-out-of-range = a página { $page } não existe
error-pdf-password-protected = o documento está protegido por palavra-passe; abra-o e copie as respetivas páginas
error-pdf-no-pages = não há páginas para extrair
error-pdf-crop-outside = a área de recorte está fora da página
error-pdf-closed = documento fechado
error-pdf-saved-unreadable = o documento guardado já não abre
error-image-read = não é possível ler o ficheiro: { $detail }
error-image-invalid = a imagem está danificada ou é inválida: { $detail }
error-image-missing-library = abrir este formato requer { $library }, que não está instalado
error-image-unsupported = as imagens { $format } ainda não são suportadas
error-image-encode = não é possível codificar a imagem: { $detail }
error-exif-malformed = os dados EXIF estão mal formados
error-settings-read = não é possível ler as definições: { $detail }
error-settings-invalid = definições inválidas: { $detail }
error-remove-location = não foi possível remover a localização: { $error }
error-location-unsupported = as informações de localização podem ser removidas de ficheiros JPEG, PNG, WebP e TIFF
error-xmp-unsupported = as palavras-chave e as descrições só podem ser guardadas em ficheiros JPEG, PNG e WebP

## Formats

format-camera-raw = RAW de câmara

## The macOS menu bar, named as in macOS's own apps.
menu-about = Acerca do prev
menu-settings = Definições…
menu-services = Serviços
menu-hide = Ocultar prev
menu-hide-others = Ocultar outros
menu-show-all = Mostrar tudo
menu-quit = Sair do prev
menu-file = Ficheiro
menu-open = Abrir…
menu-close = Fechar janela
menu-export = Exportar…
menu-print = Imprimir…
menu-edit = Edição
menu-undo = Anular
menu-redo = Refazer
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Colar
menu-select-all = Selecionar tudo
menu-find = Procurar
menu-find-next = Procurar seguinte
menu-find-previous = Procurar anterior
menu-view = Visualização
menu-hide-sidebar = Ocultar barra lateral
menu-thumbnails = Miniaturas
menu-contents = Índice
menu-notes = Realces e notas
menu-bookmarks = Marcadores
menu-zoom-in = Ampliar
menu-zoom-out = Reduzir
menu-actual-size = Tamanho real
menu-zoom-to-fit = Ajustar à janela
menu-inspector = Mostrar inspetor
menu-slideshow = Apresentação de diapositivos
menu-full-screen = Entrar em ecrã completo
menu-go = Ir
menu-next-page = Página seguinte
menu-previous-page = Página anterior
menu-go-to-page = Ir para a página…
menu-bookmark = Adicionar marcador
menu-tools = Ferramentas
menu-markup = Mostrar barra de marcações
menu-rotate-left = Rodar para a esquerda
menu-rotate-right = Rodar para a direita
menu-crop = Recortar
menu-adjust-color = Ajustar cor…
menu-window = Janela
menu-minimize = Minimizar
menu-zoom = Redimensionar
menu-bring-all-to-front = Passar tudo para a frente

## Outside control

settings-outside-control = Controlo externo
settings-allow-outside-control = Permitir controlo externo
settings-allow-outside-control-note = Agentes de IA como o Claude Code podem ler e alterar os seus ficheiros no prev, através do prev --mcp. O prev pergunta antes de cada novo agente.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Esquecer
# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Permitir que { $agent } controle o prev?
agent-prompt-body = { $agent } pede para usar o controlo externo do prev, para ler os seus ficheiros abertos e alterá-los. Pode desativar o controlo externo nas Definições.
agent-prompt-allow = Permitir
agent-prompt-deny = Não permitir
settings-ask-before-note = Perguntar antes que um agente:
settings-ask-reading = Leia um ficheiro
settings-ask-viewing = Mude a visualização ou uma janela
settings-ask-marking-up = Faça marcações num ficheiro
settings-ask-editing = Edite um ficheiro
settings-ask-signing = Assine um ficheiro
settings-ask-redacting = Aplique rasuras
settings-ask-exporting = Exporte um ficheiro
# The prompt before an agent's tool runs; $agent is the agent's name,
# such as Claude Code.
agent-ask-read = Permitir que { $agent } leia este ficheiro?
agent-ask-view = Permitir que { $agent } mude a visualização?
agent-ask-markup = Permitir que { $agent } faça marcações neste ficheiro?
agent-ask-edit = Permitir que { $agent } edite este ficheiro?
agent-ask-sign = Permitir que { $agent } assine este ficheiro?
agent-ask-redact = Permitir que { $agent } aplique rasuras?
agent-ask-export = Permitir que { $agent } exporte este ficheiro?
# $tool is the name of what the agent asks to do, such as Highlight text.
agent-ask-body = { $agent } pede para usar «{ $tool }». Nas Definições, escolhe sobre o que o prev pergunta.
agent-ask-final = Isto não pode ser anulado.
