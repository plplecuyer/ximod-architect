# XIMOD Architect - translation metadata
# @language = por
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Português
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versão { $version }

# Status messages
status-ready = Pronto
msg-save-success = FOMOD guardado com sucesso
msg-save-error = Erro ao guardar o FOMOD
msg-export-success = Arquivo de distribuição criado ({ $count } ficheiros): { $path }
msg-export-error = Erro ao criar o arquivo de distribuição: { $error }
msg-load-success = FOMOD carregado com sucesso
msg-load-error = Erro ao carregar o FOMOD
msg-merge-success = FOMOD combinado com sucesso
msg-merge-error = Erro ao combinar o FOMOD
msg-no-root-selected = Selecione primeiro um diretório raiz
msg-no-fomod-folder = Pasta «fomod» não encontrada. Criar uma?
msg-file-outside-root = O ficheiro está fora do diretório raiz

# Menu - File
menu-file = Ficheiro
menu-new = Novo
menu-open = Abrir pasta…
menu-open-file = Abrir ficheiro…
menu-save = Guardar
menu-recent = Recentes
menu-exit = Sair
menu-merge = Unir FOMOD…
menu-export = Exportar arquivo de distribuição...

# Menu - Options
menu-options = Opções
menu-settings = Definições
menu-pre-save-script = Script antes de guardar…
menu-post-save-script = Script depois de guardar…
menu-translation = Tradução...

# Menu - Help
menu-help = Ajuda
menu-about = Acerca de

# Tabs
tab-info = Informações do mod
tab-steps = Passos de instalação
tab-required = Instalações obrigatórias
tab-conditional = Instalações condicionais

# Info Tab
label-workspace = Área de trabalho
label-root-dir = Diretório raiz:
label-mod-name = Nome do mod:
label-author = Autor:
label-version = Versão:
label-game-name = Nome do jogo:
label-category = Categoria:
label-url = URL do site:
label-header-image = Imagem de cabeçalho:
label-description = Descrição:
placeholder-select-dir = (Selecione um diretório)
placeholder-select-game = (Selecione um jogo)

# Steps Tab
label-step-name = Nome do passo:
label-group-name = Nome do grupo:
label-group-type = Tipo de grupo:
label-plugin-name = Nome do plugin:
label-plugin-desc = Descrição:
label-plugin-type = Tipo predefinido:
label-plugin-image = Imagem:
label-visibility = Condições de visibilidade
label-operator = Operador:

# Buttons
btn-browse = Procurar…
btn-clear = Limpar
btn-add = Adicionar
btn-remove = Remover
btn-add-step = Novo passo
btn-delete-step = Eliminar passo
btn-add-group = Adicionar grupo
btn-remove-group = Remover grupo
btn-add-plugin = Adicionar plugin
btn-remove-plugin = Remover plugin
btn-add-file = Adicionar ficheiro
btn-add-folder = Adicionar pasta
btn-remove-file = Remover
btn-add-flag = Adicionar flag
btn-remove-flag = Remover flag
btn-add-condition = Adicionar condição
btn-remove-condition = Remover condição
btn-add-dependency = Adicionar dependência
btn-remove-dependency = Remover dependência
btn-add-pattern = Novo padrão
btn-remove-pattern = Eliminar padrão
btn-save = Guardar
btn-cancel = Cancelar
btn-ok = OK
btn-yes = Sim
btn-no = Não

# Condition/Dependency Labels
label-flag-name = Nome da flag:
label-flag-value = Valor:
label-condition-type = Tipo:
label-condition-name = Nome:
label-condition-value = Valor:
label-dep-type = Tipo de dependência:
label-dep-name = Nome/ficheiro:
label-dep-value = Valor/estado:

# Files
label-source = Origem
label-destination = Destino
label-priority = Prioridade
label-file-type = Tipo
label-files = Ficheiros
label-dependencies = Dependências

# Settings Dialog
settings-title = Definições
settings-tab-general = Geral
settings-tab-recent-files = Ficheiros recentes
settings-language = Idioma:
settings-theme = Tema:
settings-font-size = Tamanho do tipo de letra:
settings-replace-newlines = Processar quebras de linha nas descrições
settings-max-recent = Máx. de ficheiros recentes:
settings-window-width = Largura da janela:
settings-window-height = Altura da janela:
settings-no-recent-files = Sem ficheiros recentes.

# Status messages for settings
status-settings-saved = Definições guardadas com sucesso

# About Dialog
about-title = Acerca do XIMOD Architect
about-description = Uma ferramenta multiplataforma para criar instaladores FOMOD para mods de jogos da Bethesda.
about-license = Licenciado sob a licença MIT
about-copyright = © 2024 XIMOD Team
about-credit = Portabilidade para Rust da ferramenta original de Wenderer:

# Script Dialog
script-title = Editar script
script-info = Os scripts são executados antes ou depois de guardar. Pode utilizar as seguintes macros:
script-macros = Macros disponíveis:
macro-modname = $MODNAME$ - Nome do mod
macro-modauthor = $MODAUTHOR$ - Nome do autor
macro-modversion = $MODVERSION$ - Versão do mod
macro-modroot = $MODROOT$ - Caminho do diretório raiz
macro-date = $DATE$ - Data atual (AAAA-MM-DD)
macro-time = $TIME$ - Hora atual (HH:MM:SS)
macro-random = $RANDOM$ - Número aleatório

# Plugin Dependencies
label-default-type = Tipo predefinido:
label-pattern-type = Tipo de padrão:
label-pattern-operator = Operador do padrão:

# Conditional Files
label-pattern = Padrão

# Validation Messages
validation-no-name = O nome do mod é obrigatório
validation-no-steps = É necessário pelo menos um passo ou ficheiro obrigatório
validation-empty-step = O passo { $num } não tem nome
validation-empty-group = O passo { $step }, grupo { $group } não tem nome
validation-no-plugins = O passo { $step }, grupo «{ $name }» não tem plugins

# File States
state-active = Ativo
state-inactive = Inativo
state-missing = Em falta

# Confirmation
confirm-title = Confirmação
confirm-delete = Tem a certeza de que quer eliminar este item?
confirm-discard = Tem alterações não guardadas. Descartá-las e continuar?
confirm-unsaved = Tem alterações não guardadas. Quer guardar antes de fechar?
confirm-save-issues = O projeto tem os seguintes problemas:
confirm-save-anyway = Guardar mesmo assim?

# Errors
error-invalid-xml = Ficheiro XML inválido
error-parse-failed = Falha ao analisar o FOMOD
error-write-failed = Falha ao escrever o ficheiro
error-create-dir = Falha ao criar o diretório

# Default names (generated when creating new items)
default-step-name = Passo { $num }
default-group-name = Grupo { $num }
default-plugin-name = Plugin { $num }
pattern-label = Padrão { $num }

# Selection prompts
msg-select-group-first = Selecione primeiro um grupo.
msg-select-plugin-edit = Selecione um plugin para editar.
label-empty = (vazio)
image-no-image = Sem imagem

# File dialog filters
filter-images = Imagens
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = Ficheiro

# Status bar
status-modified = Modificado

# Status messages (errors)
msg-settings-save-error = Erro ao guardar as definições
msg-script-save-error = Erro ao guardar o script

# Translation editor
trans-title = Editor de Tradução
trans-source-lang = Idioma apresentado:
trans-target-lang = Idioma a traduzir:
trans-col-key = Chave
trans-col-source = Etiqueta
trans-col-target = Tradução
trans-saved = Tradução guardada
trans-save-error = Erro ao guardar a tradução

# XML editor
xml-editor-title = Editor XML
xml-editor-edit = Editar
xml-editor-apply = Aplicar
xml-editor-revert = Cancelar
xml-editor-readonly = Só de leitura
xml-editor-editing = A editar — os separadores gráficos estão bloqueados
xml-editor-error = Erro:
xml-editor-applied = Alterações XML aplicadas
xml-editor-wellformed = XML bem formado
xml-editor-error-at = Linha { $line }, coluna { $col }: { $msg }

# Country / flag picker
settings-country-name = Nome do país:
settings-pick-country = Clique para escolher o seu país
flags-title = Escolha um país
flags-filter = Filtro:
flags-none = Nenhuma bandeira encontrada

# Translation editor: country & font
trans-endonym = Endónimo do país:
trans-font = Tipo de letra:
trans-no-font = (nenhum)
trans-browse = Procurar…
trans-google-fonts = Google Fonts
trans-pick-country = Clique para escolher o país
trans-font-outside = O tipo de letra deve primeiro ser instalado em assets/fonts.
trans-font-dir-missing = A pasta assets/fonts não foi encontrada.

# Translation submission
trans-lang-endonym = Endónimo do idioma:
trans-author = Autor:
trans-submit = Enviar…
trans-submit-hint = Criar um .zip e abrir um e-mail pré-preenchido
trans-data-updated = Dados de referência atualizados (Languages.json / Countries.json)
trans-package-ready = Arquivo pronto:
trans-package-error = Não foi possível criar o arquivo:

# ISO 639-3 requirement
trans-lang-not-iso = A tradução só é possível para um idioma com um código ISO 639-3.

# FOMOD installer preview
menu-preview = Pré-visualizar instalador…
preview-title = Pré-visualização do instalador FOMOD
preview-refresh = Atualizar
preview-assumptions = Pressupostos de ficheiros
preview-details = Detalhes
preview-back = Anterior
preview-next = Seguinte
preview-install = Instalar
preview-close = Fechar
preview-restart = Reiniciar
preview-summary-title = Ficheiros que serão instalados
preview-empty = Nenhum ficheiro seria instalado.
preview-none-option = (nenhum)
preview-invalid = Complete as escolhas obrigatórias para continuar.
preview-no-steps = Nenhum passo está visível; consulte o resumo da instalação.
preview-select-hint = Selecione uma opção para ver a sua descrição.
preview-col-source = Origem
preview-col-dest = Destino
preview-col-priority = Prioridade
preview-sel-exactlyone = Escolha exatamente uma opção.
preview-sel-atmostone = Escolha no máximo uma opção.
preview-sel-any = Escolha qualquer número de opções.
preview-sel-all = Todas as opções são instaladas.
preview-sel-atleastone = Escolha pelo menos uma opção.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validar FOMOD
validate-report-title = Validação do FOMOD
validate-ok = Nenhum problema encontrado. O FOMOD está em conformidade com o esquema.
xml-editor-schema-ok = Em conformidade com o esquema ModConfig 5.0.
xml-editor-schema-issues = Problemas do esquema:
schema-line-col = Linha { $line }, col. { $col }: { $msg }
schema-wrong-root = Raiz inesperada "{ $found }" (esperada "{ $expected }").
schema-unknown = Elemento inesperado "{ $element }" em "{ $parent }".
schema-missing = "{ $parent }" deve conter "{ $child }".
schema-needs-one = "{ $parent }" deve conter pelo menos um "{ $child }".
schema-too-many = "{ $child }" só pode aparecer uma vez em "{ $parent }".
schema-missing-attr = O atributo "{ $attr }" é obrigatório em "{ $element }".
schema-bad-enum = Valor inválido "{ $value }" para { $element }/@{ $attr } (esperado: { $allowed }).
schema-choose-one = "{ $parent }" deve conter exatamente um de: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Mover para antes
reorder-after = Mover para depois

# Country / language database explorer (Properties)
menu-properties = Propriedades…
prop-title = Base de dados de países / idiomas
prop-tab-countries = Países
prop-tab-languages = Idiomas
prop-filter = Filtro:
prop-official-langs = Idiomas oficiais
prop-spoken-langs = Idiomas falados
prop-endonym = Endónimo do país
prop-font = Tipo de letra
prop-spoken-in = Falado em
prop-select-country = Selecione um país para ver os seus detalhes.
prop-select-lang = Selecione um idioma para ver os seus detalhes.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Abrir a página do jogo no Nexus Mods
