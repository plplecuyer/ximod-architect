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
menu-export = Exportar arquivo de distribuição…
# Menu - Options
menu-options = Opções
menu-settings = Definições…
menu-pre-save-script = Script antes de guardar…
menu-post-save-script = Script depois de guardar…
menu-translation = Traduzir a interface…
# Menu - Help
menu-help = Ajuda
menu-check-updates = Procurar atualizações…
menu-about = Acerca de

# Update check
update-checking = A procurar atualizações…
update-up-to-date = O XIMOD Architect está atualizado.
update-check-failed = Não foi possível procurar atualizações. Tente novamente mais tarde.
update-available-status = A versão { $version } está disponível.
update-banner-text = O XIMOD Architect { $version } está disponível.
update-download = Transferir:
update-skip = Ignorar esta versão
update-later = Mais tarde

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
label-plugin-name = Nome da opção:
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
btn-add-plugin = Adicionar opção
btn-remove-plugin = Remover opção
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Destino para todo o grupo
label-page-dest = Destino de instalação (página inteira)
btn-apply-group-dest = Aplicar a todas as opções deste grupo
btn-apply-page-dest = Aplicar a todas as opções desta página
group-dest-hint = Define um único destino de instalação para cada ficheiro de cada opção deste grupo.
page-dest-hint = Define um único destino de instalação para cada ficheiro de cada opção desta página (todos os grupos).
bulk-dest-nofiles = Ainda não há ficheiros para atualizar — adicione primeiro ficheiros às opções.
status-dest-applied = Destino aplicado a { $num } ficheiro(s).
preview-hidden-steps = { $num } passo(s) ocultado(s) pelas seleções atuais.
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
settings-check-updates = Procurar atualizações ao iniciar
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
label-plugin-dependencies = Dependências da opção
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
validation-no-plugins = O passo { $step }, grupo «{ $name }» não tem opções

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
default-plugin-name = Opção { $num }
pattern-label = Padrão { $num }

# Selection prompts
msg-select-group-first = Selecione primeiro um grupo.
msg-select-plugin-edit = Selecione uma opção para editar.
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

# Referenced-file verification (V2)
verify-no-root = Verificação de ficheiros ignorada: nenhuma pasta raiz definida
loc-header = imagem de cabeçalho
loc-required = ficheiros obrigatórios
loc-conditional = conjunto condicional { $num }
loc-plugin = passo { $step }, grupo { $group }, opção «{ $plugin }»
verify-missing-file = Ficheiro em falta: { $path } ({ $loc })
verify-missing-folder = Pasta em falta: { $path } ({ $loc })
verify-missing-image = Imagem em falta: { $path } ({ $loc })
verify-absolute = Caminho absoluto (não portátil): { $path } ({ $loc })
verify-outside = O caminho sai da pasta raiz: { $path } ({ $loc })
verify-orphan = Ficheiro órfão (não referenciado por nenhuma opção): { $path }
conflict-certain = Conflito de destino: «{ $path }» é escrito por { $count } opções ({ $locs }) — sobrescrevem-se mutuamente.
conflict-potential = Possível conflito de destino: «{ $path }» é alvo de { $count } referências ({ $locs }) — a sobreposição depende da seleção/condições.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Fechar FOMOD
menu-close-all-fomods = Fechar todos os FOMOD
tab-untitled = (sem título)
msg-drop-not-fomod = O item largado não é um FOMOD (pasta «fomod» não encontrada)
exit-title = Alterações não guardadas
exit-unsaved = Um FOMOD não foi guardado. Quer guardá-lo?
tab-close-hint = Fechar este FOMOD
menu-new-from-folder = Novo a partir de pasta…
menu-templates = Modelos…
templates-title = Modelos reutilizáveis
templates-empty = Ainda não há modelos guardados. Guarde acima o passo selecionado para criar um.
templates-insert = Inserir
templates-save-step = Guardar o passo selecionado
templates-name-hint = Nome do modelo (opcional)
msg-wizard-success = Esqueleto criado a partir da pasta: { $num } opção(ões).
msg-wizard-error = Erro: { $error }
msg-template-saved = Modelo guardado: { $name }
msg-template-inserted = Modelo inserido no projeto.
msg-template-no-step = Selecione primeiro um passo para o guardar como modelo.
msg-template-no-dir = Não foi possível localizar a pasta de modelos.
msg-drop-assigned = { $added } fonte(s) adicionada(s) à opção ({ $rejected } fora da raiz ignorada(s)).
menu-compare = Comparar com…
compare-title = Comparação de FOMOD
compare-none = Sem diferenças.
btn-optimize-image = Otimizar imagem
msg-image-optimized = Imagem de cabeçalho otimizada.
msg-image-ok = A imagem de cabeçalho já está dentro dos limites.
msg-no-header-image = Nenhuma imagem de cabeçalho para otimizar.
verify-image-large = Imagem demasiado grande ({ $width }×{ $height }): { $path }
verify-image-format = Formato de imagem não suportado (.{ $ext }): { $path }
verify-image-unreadable = Imagem ilegível: { $path }
menu-condition-editor = Editor de condições…
condeditor-title = Editor de condições
condeditor-set-by = Definido por:
condeditor-used-by = Usado por:
condeditor-filedeps = Dependências de ficheiros
condeditor-empty = Nenhuma flag ou dependência neste projeto.
condeditor-orphan-set = definido mas nunca usado
condeditor-orphan-used = usado mas nunca definido
msg-img-optimized = Imagem otimizada.
msg-img-ok = Imagem já dentro dos limites.
msg-img-none = Nenhuma imagem para otimizar.
msg-crash-recovery = A sessão anterior terminou inesperadamente. Uma cópia de segurança do seu projeto foi guardada em { $path }
export-progress-title = A criar o arquivo de distribuição…
export-progress-files = { $done } / { $total } ficheiros
msg-export-cancelled = Exportação cancelada; o arquivo parcial foi removido.
verify-running = A verificar os ficheiros no disco…
verify-stale = Nota: o projeto foi alterado durante a verificação dos ficheiros; execute a validação novamente.
prop-col-name = Nome
menu-save-as = Guardar como…
menu-project = Projeto
menu-tools = Ferramentas
menu-manual = Manual do utilizador
msg-manual-missing = O manual do utilizador (PDF) não foi encontrado junto à aplicação.
toolbar-new = Novo
toolbar-open = Abrir
toolbar-save = Guardar
toolbar-validate = Validar
toolbar-preview = Pré-visualizar
toolbar-export = Exportar
dialog-choose-root = Escolha a pasta raiz do mod
exit-unsaved-docs = Não guardados: { $names }
status-summary = { $steps } passos · { $options } opções
section-groups = Grupos
section-options = Opções
section-flags = Sinalizadores de condição
section-files = Ficheiros a instalar
hint-group-type = Como o instalador deixa o utilizador escolher as opções deste grupo.
hint-default-type = Como a opção é proposta quando nenhum dos seus padrões de dependência corresponde: obrigatória, opcional, recomendada, inutilizável…
hint-operator = Todas as condições têm de ser verdadeiras (E), ou basta uma (OU).
hint-flags = Os sinalizadores são valores com nome que esta opção define quando é escolhida. Outros passos e opções podem testá-los para se mostrarem, ocultarem ou tornarem obrigatórios.
hint-plugin-dependencies = Padrões que alteram o tipo da opção consoante sinalizadores ou ficheiros presentes no jogo: por exemplo “Obrigatória” quando outro mod está instalado.
hint-files = Ficheiros e pastas copiados para a pasta Data do jogo quando esta opção é escolhida. O destino é relativo a Data; em caso de conflito vence a prioridade mais alta.
hint-visibility = Condições a cumprir para que este passo seja mostrado. Deixe vazio para o mostrar sempre.
seltype-exactly-one = Exatamente uma (obrigatória)
seltype-at-most-one = No máximo uma
seltype-any = Qualquer número
seltype-all = Todas (sem escolha)
seltype-at-least-one = Pelo menos uma
plugtype-required = Obrigatória
plugtype-optional = Opcional
plugtype-recommended = Recomendada
plugtype-not-usable = Inutilizável
plugtype-could-be-usable = Talvez utilizável
plugtype-required-hint = Sempre instalada; o utilizador não a pode desmarcar.
plugtype-optional-hint = Proposta desmarcada; o utilizador decide.
plugtype-recommended-hint = Proposta marcada; o utilizador pode desmarcá-la.
plugtype-not-usable-hint = Mostrada a cinzento, não pode ser selecionada.
plugtype-could-be-usable-hint = Selecionável, mas o instalador avisa que pode não funcionar.
op-and = Todas as condições (E)
op-or = Qualquer condição (OU)
theme-dark = Escuro
theme-light = Claro
theme-system = Seguir o sistema
condeditor-setter-loc = Passo { "{step}" } / Grupo { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Padrão de «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Visibilidade do passo { "{step}" }
condeditor-cond-set = Conjunto condicional { "{num}" }
condeditor-needs = { "{ctx}" } (requer = { "{value}" })
condeditor-file-dep = { "{ctx}" }: ficheiro «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = Traduzir um FOMOD…
ftr-title = Traduzir um FOMOD
ftr-open-folder = Abrir uma pasta de mod…
ftr-from-active = A partir do projeto ativo
ftr-from-active-hint = Traduz o FOMOD do projeto aberto na janela principal (tem de ser guardado primeiro).
ftr-no-fomod = Nenhum FOMOD carregado.
ftr-encoding = Codificação dos ficheiros originais; os ficheiros traduzidos são escritos com a mesma codificação.
ftr-source-lang = De
ftr-target-lang = para
ftr-lang-locked = (os idiomas ficam fixos depois de carregado um FOMOD)
ftr-translator = Tradutor:
ftr-save = Guardar a tradução
ftr-export = Exportar os ficheiros traduzidos
ftr-export-sibling = Para uma pasta fomod_<idioma>
ftr-export-sibling-hint = Escreve os ficheiros info.xml e ModuleConfig.xml traduzidos ao lado da pasta fomod original; os ficheiros originais não são alterados.
ftr-export-inplace = Por cima dos ficheiros originais
ftr-export-inplace-hint = Substitui fomod/info.xml e fomod/ModuleConfig.xml depois de criar uma cópia .bak com data e hora de cada um.
ftr-force-explicit-order = Manter a ordem original
ftr-warn-order = As listas ordenadas por nome (order="Ascending") seriam reordenadas pelos nomes traduzidos no gestor de mods. Esta opção força order="Explicit" para que as opções mantenham a ordem atual.
ftr-update = Atualizar a partir da pasta
ftr-update-hint = Volta a ler o FOMOD do disco e funde a tradução com ele: as cadeias novas, alteradas e removidas são assinaladas.
ftr-preview-translated = Pré-visualização traduzida
ftr-progress = { $done } / { $total } traduzidas
ftr-filter-all = Todas
ftr-filter-untranslated = Por traduzir
ftr-filter-review = A rever
ftr-filter-issues = Com problemas
ftr-filter-locked = Bloqueadas
ftr-type-all = Todos os campos
ftr-type-names = Nomes
ftr-type-descriptions = Descrições
ftr-type-meta = Informações do mod
ftr-search-hint = Procurar na origem, na tradução ou no contexto…
ftr-next-untranslated = Próxima por traduzir
ftr-show-whitespace = Mostrar espaços e quebras de linha
ftr-discard-question = A tradução atual tem alterações não guardadas. Descartá-las e carregar o outro FOMOD?
ftr-discard-yes = Descartar
ftr-unsaved-close = A tradução tem alterações não guardadas.
ftr-col-num = N.º
ftr-col-status = { "" }
ftr-col-context = Contexto
ftr-col-source = Origem
ftr-col-target = Tradução
ftr-col-issues = { "" }
ftr-empty-hint = Abra uma pasta de mod, ou carregue o projeto ativo, para listar as suas cadeias traduzíveis.
ftr-empty-filter = Nenhuma cadeia corresponde ao filtro atual.
ftr-select-row = Selecione uma linha para editar a sua tradução.
ftr-copy-source = Copiar a origem
ftr-clear-target = Limpar
ftr-lock = Não traduzir
ftr-lock-hint = As cadeias bloqueadas são escritas sem alterações (autor, site, nomes próprios…).
ftr-note = Nota:
ftr-status-untranslated = Por traduzir
ftr-status-translated = Traduzida
ftr-status-auto = Pré-preenchida automaticamente — a rever
ftr-status-fuzzy = O texto de origem mudou desde a tradução — a rever
ftr-status-obsolete = Já não existe no FOMOD
ftr-status-locked = Bloqueada (escrita sem alterações)
ftr-field-info-name = Nome do mod (info.xml)
ftr-field-module-name = Título do instalador (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Site
ftr-field-description = Descrição do mod
ftr-field-step = Nome do passo
ftr-field-group = Nome do grupo
ftr-field-plugin = Nome da opção
ftr-field-plugin-desc = Descrição da opção
ftr-issue-empty = Tradução vazia
ftr-issue-whitespace = A tradução contém apenas espaços
ftr-issue-edge-whitespace = Os espaços no início ou no fim diferem da origem
ftr-issue-token = Os tokens protegidos diferem — em falta: { $missing } ; a mais: { $extra }
ftr-issue-newline-name = Um nome não pode conter uma quebra de linha
ftr-issue-control = Contém caracteres que o XML não consegue armazenar
ftr-issue-length = Comprimento invulgar em relação à origem (×{ $ratio })
ftr-issue-identical = Idêntica à origem
ftr-issue-duplicate = O mesmo texto de origem está traduzido de forma diferente em { $key }
ftr-issue-cdata = A sequência ]]> não é permitida aqui
ftr-load-error = Não foi possível carregar o FOMOD: { $error }
ftr-extracted = { $num } cadeias traduzíveis encontradas.
ftr-sidecar-found = Tradução existente carregada e fundida: { $new } novas, { $changed } alteradas, { $removed } removidas.
ftr-saved = Tradução guardada em { $path }
ftr-save-error = Não foi possível guardar a tradução: { $error }
ftr-save-first = Guarde primeiro o projeto e depois traduza-o.
ftr-export-success = { $count } cadeias escritas em { $path }
ftr-export-error = A exportação falhou: { $error }
ftr-export-blocked = { $num } problemas bloqueantes têm de ser corrigidos antes de exportar.
ftr-export-stale = { $num } cadeias foram ignoradas porque o FOMOD mudou; use «Atualizar a partir da pasta».
ftr-update-report = Atualizado: { $new } novas, { $changed } alteradas, { $moved } movidas, { $removed } removidas, { $unchanged } inalteradas.
menu-edit = Editar
menu-undo = Anular
menu-redo = Refazer
tree-title = Projeto
tree-mod-info = Informações do mod
tree-steps = Passos de instalação
tree-required = Ficheiros obrigatórios
tree-conditional = Instalações condicionais
tree-empty-steps = Ainda sem passos — clique em + para adicionar um.
tree-duplicate = Duplicar
tree-delete = Eliminar
tree-save-template = Guardar como modelo…
tree-drop-hint = Largue aqui para mover
cond-set-label = Conjunto condicional { $num }
inspector-empty = Selecione um item na árvore do projeto ou adicione um passo para começar.
count-options = { $num } opções
count-files = { $num } ficheiros
msg-deleted-undo = Eliminado. Use Anular (Ctrl+Z) para o restaurar.
problems-title = Problemas
problems-errors = { $num } erros
problems-warnings = { $num } avisos
btn-close = Fechar
ftr-export-package = Como pacote de tradução (arquivo)
ftr-export-package-hint = Cria um .zip ou .7z pronto a publicar: os ficheiros info.xml e ModuleConfig.xml traduzidos mais um README (apenas correção), ou o mod inteiro com os ficheiros traduzidos (completo).
ftr-package-full = Mod completo
ftr-package-full-hint = Incluir todos os ficheiros do mod no arquivo, e não apenas os dois ficheiros XML traduzidos. Certifique-se de que o autor permite a redistribuição.
ftr-package-name-template = Nome:
ftr-readme-patch = Este arquivo contém a tradução ({ $langname }) do instalador de «{ $name }» (fomod/info.xml e fomod/ModuleConfig.xml). Instale-o por cima do mod original, ou deixe o seu gestor de mods fundi-lo, para que os ficheiros traduzidos substituam os originais. Só os textos do instalador mudam; os ficheiros do mod propriamente ditos não estão incluídos. Feito com XIMOD Architect.
ftr-readme-full = Este arquivo contém «{ $name }» com o instalador traduzido ({ $langname }; fomod/info.xml e fomod/ModuleConfig.xml). Instale-o como o mod original. Só os textos do instalador foram alterados. Feito com XIMOD Architect.
ftr-apply-memory = Preencher a partir da memória
ftr-memory-size = Memória de tradução: { $num } entradas para este par de línguas. Cada tradução guardada é-lhe adicionada.
ftr-memory-applied = { $num } cadeias preenchidas a partir da memória de tradução (marcadas «a rever»).
ftr-memory-suggestion = A memória sugere:
ftr-use-suggestion = Usar
ftr-propagate = Propagar às idênticas
ftr-propagate-hint = Copiar esta tradução para todas as outras cadeias com o mesmo texto de origem ainda por traduzir.
ftr-propagated = { $num } cadeias idênticas preenchidas.
ftr-csv-export = Exportar CSV…
ftr-csv-import = Importar CSV…
ftr-csv-imported = { $num } cadeias atualizadas a partir do ficheiro CSV.
ftr-csv-error = Erro de CSV: { $error }
ftr-glossary = Glossário
ftr-glossary-source = Termo
ftr-glossary-target = Tradução
ftr-glossary-case = Maiúsculas/minúsculas
ftr-glossary-dnt = Manter
ftr-glossary-add = Adicionar termo
ftr-issue-glossary = Glossário: «{ $term }» não está traduzido como esperado

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Abrir arquivo…
filter-archive = Arquivos de mod (zip, 7z)
msg-archive-opened = Arquivo aberto ({ $num } ficheiros extraídos): { $path }
msg-archive-reused = Arquivo já extraído, a reutilizar { $path }
msg-archive-unsupported = O formato de arquivo «.{ $ext }» não é suportado; extraia-o primeiro com o 7-Zip (só é possível abrir .zip e .7z).
msg-archive-error = Erro ao abrir o arquivo: { $error }
msg-archive-no-fomod = Nenhuma pasta «fomod» encontrada no arquivo ({ $path })
msg-archive-extracting = A extrair o arquivo…
ftr-open-archive = Abrir um arquivo de mod…
ftr-package-full-partial = O mod foi aberto a partir de um arquivo que contém apenas a pasta fomod; os pacotes completos precisam do mod extraído.
info-module-deps = Requisitos do mod
info-module-deps-hint = Ficheiros ou sinalizadores de que todo o mod precisa antes de o instalador ser executado (moduleDependencies). Deixe vazio se não houver.
info-header-advanced = Cabeçalho avançado
info-title-position = Posição do título
info-title-colour = Cor do título
info-title-colour-hint = Esperado: seis dígitos hexadecimais (RRGGBB)
info-image-show = Mostrar imagem do cabeçalho
info-image-fade = Esbater imagem do cabeçalho
info-image-height = Altura da imagem do cabeçalho
info-attr-default = (predefinição)
file-always-install = Sempre
file-always-install-hint = Instalar sempre este ficheiro, mesmo quando a opção não está selecionada (alwaysInstall).
file-install-if-usable = Se usável
file-install-if-usable-hint = Instalar este ficheiro sempre que a opção for utilizável, mesmo quando não está selecionada (installIfUsable).
msg-import-lossy = Este FOMOD contém { $num } construções que o XIMOD não consegue editar; serão descartadas ao guardar o projeto.
fidelity-nested-deps = Grupo de dependências aninhado em { $context } (só é suportado um nível)
fidelity-game-dep = Requisito de versão do jogo { $version } em { $context }
fidelity-fomm-dep = Requisito de versão do gestor de mods { $version } em { $context }
fidelity-unknown = O elemento «{ $element }» em «{ $parent }» não é suportado ({ $context })
loc-module = os requisitos do mod
loc-step = o passo { $step } «{ $name }»
loc-installer = o instalador

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Restaurar uma cópia de segurança…
backups-title = Restaurar uma cópia de segurança
backups-empty = Este projeto ainda não tem nenhuma cópia de segurança. É criada uma sempre que o projeto é guardado sobre uma versão anterior.
backups-changes = { $num } alteração(ões) em relação ao projeto atual
btn-compare = Comparar
btn-restore = Restaurar
btn-delete-backups = Eliminar todas as cópias de segurança
btn-delete-backups-confirm = Clique novamente para eliminar todas as cópias de segurança
msg-backup-restored = Cópia de segurança de { $time } restaurada no editor (ainda não guardada; Anular reverte-a)
msg-backups-deleted = { $num } cópia(s) de segurança eliminada(s)
settings-backup-count = Cópias de segurança a manter:
settings-backup-count-hint = Número de versões anteriores do XML do FOMOD mantidas em fomod/backups ao guardar (0 = sem cópias de segurança).
settings-autosave-minutes = Guardar automaticamente uma cópia de recuperação a cada (minutos):
settings-autosave-minutes-hint = A este intervalo é escrita na pasta de configuração uma cópia de recuperação de cada projeto modificado; só é proposta no arranque seguinte após um encerramento anormal (0 = desativado).
settings-auto-masters = Adicionar os masters de um plugin como condições
settings-auto-masters-hint = Quando um plugin (.esp/.esm/.esl) é adicionado a uma opção, os masters de que precisa e que nem o jogo nem este mod fornecem tornam-se condições de ficheiro «Active» da opção.
msg-author-from-plugin = Autor preenchido a partir do cabeçalho do plugin: { $author }
msg-masters-added = { $num } master(s) de { $plugin } adicionado(s) como condição(ões) de ficheiro
issue-missing-master = { $plugin } requer { $master }, que não está neste mod nem foi declarado como dependência
issue-esl-mismatch-flag = { $plugin } tem a extensão .esl mas o seu sinalizador light (ESL) não está ativo
issue-esl-eligible = { $plugin } poderia ser marcado como light ({ $num } registos novos, limite { $limit })
issue-esl-too-big = { $plugin } está marcado como light mas não cumpre as regras dos plugins light ({ $num } registos novos, limite { $limit }, ou um FormID fora do intervalo permitido)
menu-plugin-report = Relatório de plugins…
plugins-title = Relatório de plugins
plugins-file = Ficheiro
plugins-kind = Tipo
plugins-light = Sinalizador light
plugins-masters = Masters
plugins-new-records = Registos novos / limite
plugins-eligible = Elegível light
plugins-empty = Este projeto não instala nenhum ficheiro de plugin (.esp, .esm ou .esl).
plugins-unreadable = ilegível

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Árvore final de ficheiros
preview-total-size = Tamanho total da instalação: { $size }
preview-tree-truncated = A árvore está truncada: demasiados ficheiros para expandir (os tamanhos acima são parciais).
preview-overwritten-by = Substituído por { $plugin }
preview-scenario = Cenário:
preview-scenario-load = Carregar
preview-scenario-save = Guardar…
preview-scenario-delete = Eliminar
preview-scenario-name = Nome do cenário
preview-scenario-saved = Cenário «{ $name }» guardado em fomod/scenarios
preview-scenario-unresolved = { $num } seleção(ões) do cenário não corresponde(m) a nenhuma opção deste projeto (renomeada ou removida)
preview-scenario-none = (sem cenário)
issue-unreachable-step = O passo «{ $step }» nunca pode ser mostrado: as suas condições de visibilidade testam um valor de flag que nenhuma opção anterior define
issue-unreachable-option = A opção «{ $plugin }» nunca pode ser selecionada: os seus padrões de tipo utilizável testam um valor de flag que nenhuma opção define
issue-unreachable-cond = O conjunto condicional de ficheiros { $num } nunca pode aplicar-se: as suas condições testam um valor de flag que nenhuma opção define
size-option = Tamanho da instalação: { $size } ({ $num } ficheiro(s))
size-missing = { $num } origem(ns) em falta
size-unknown = Tamanho da instalação: — (execute Validar para medir)
menu-nexus-desc = Descrição para o Nexus…
nexus-title = Descrição para o Nexus Mods
nexus-format = Formato:
nexus-include-requirements = Requisitos
nexus-include-options = Opções de instalação
nexus-include-install = Instalação
nexus-include-changelog = Registo de alterações
nexus-previous = Versão anterior…
nexus-previous-none = (sem versão anterior: sem registo de alterações)
nexus-language = Idioma:
nexus-language-source = (origem)
nexus-sec-requirements = Requisitos
nexus-sec-options = Opções de instalação
nexus-sec-install = Instalação
nexus-sec-changelog = Registo de alterações
nexus-install-text = Este mod inclui um instalador FOMOD: instale-o com um gestor de mods (Vortex, Mod Organizer 2) e escolha as suas opções no instalador.
nexus-requires = Requer
nexus-step = Passo
nexus-added = Adicionado
nexus-removed = Removido
nexus-changed = Alterado
btn-copy = Copiar
btn-save-as = Guardar como…
msg-copied = Copiado para a área de transferência
msg-saved-to = Guardado em { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Mudar o nome…
condeditor-rename-exists = Já existe uma flag com o nome «{ $name }»
condeditor-renamed = Flag «{ $from }» renomeada para «{ $to }» ({ $num } ocorrência(s))
condeditor-delete-uses = Eliminar todas as utilizações
condeditor-deleted-uses = Flag «{ $name }» removida em todo o lado ({ $num } ocorrência(s))
condeditor-values-set = Valores definidos:
condeditor-values-tested = Valores testados:
condeditor-value-never-set = { $value } — testado mas nunca definido
condeditor-value-never-tested = { $value } — definido mas nunca testado
condeditor-builder = Construtor de condições
condeditor-builder-none = Selecione um passo, uma opção, um conjunto condicional de ficheiros ou as informações do mod na janela principal para editar aqui as suas condições.
condeditor-builder-pattern = Padrão:
condeditor-sentence-if = SE
condeditor-sentence-and = E
condeditor-sentence-or = OU
condeditor-sentence-flag = a flag { "{name}" } = { "{value}" }
condeditor-sentence-file = o ficheiro { "{name}" } está { "{value}" }
condeditor-sentence-empty = (sem condição: sempre verdadeiro)
condeditor-sentence-then-visible = ENTÃO o passo é mostrado
condeditor-sentence-then-type = ENTÃO a opção torna-se { $type }
condeditor-sentence-then-install = ENTÃO os ficheiros são instalados
condeditor-sentence-then-module = ENTÃO o instalador pode ser executado (verificado antes de iniciar)
issue-flag-value-never-set = A flag «{ $flag }» é testada com o valor «{ $value }», que nenhuma opção define
issue-flag-never-used = A flag «{ $flag }» é definida mas nunca é testada
menu-project-strings = Textos do projeto…
strings-title = Textos do projeto
strings-search = Pesquisar texto, localização ou chave…
strings-kind-all = Todos
strings-kind-names = Nomes
strings-kind-descriptions = Descrições
strings-duplicates-only = Apenas duplicados
strings-replace-with = Substituir por:
strings-case = Diferenciar maiúsculas/minúsculas
strings-whole-word = Palavra inteira
strings-replace-current = Substituir
strings-replace-all = Substituir tudo
strings-replaced = { $num } texto(s) substituído(s)
strings-dup-badge = ×{ $num }
strings-dup-hover = Mesmo texto que:
strings-count = { $num } texto(s) · { $dups } grupo(s) de duplicados
strings-col-location = Localização
strings-col-field = Campo
strings-col-text = Texto

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Conteúdo de um arquivo…
filter-bethesda-archive = Arquivos Bethesda (bsa, ba2)
archive-view-title = Conteúdo do arquivo
archive-view-format = Formato:
archive-view-entries = { $num } entradas
archive-view-size = { $size } descompactados
archive-view-search = Pesquisar um caminho…
archive-view-col-path = Caminho
archive-view-col-size = Tamanho
archive-view-col-compressed = Comprimido
archive-view-truncated = Apenas as primeiras { $num } entradas correspondentes são mostradas — refine a pesquisa.
archive-view-error = Não é possível ler este arquivo: { $error }
archive-view-hint = Ver o conteúdo deste arquivo
issue-conflict-archive = Mesmo recurso em vários arquivos: «{ $path }» é empacotado por { $count } referências ({ $locs }) — a ordem de carregamento dos arquivos do jogo decide qual é usado.
issue-conflict-archive-loose = Arquivo contra ficheiro solto: «{ $path }» está simultaneamente empacotado num arquivo e instalado como ficheiro solto ({ $locs }) — o ficheiro solto prevalece sobre o arquivado.
preview-in-archive = (no arquivo)
preview-archived-size = dos quais { $size } empacotados em arquivos

# --- Project tree: expand / collapse menus
tree-expand = Expandir
tree-collapse = Recolher
tree-expand-all = Expandir tudo
tree-expand-selected = Expandir seleção
tree-expand-from = Expandir a partir da seleção
tree-collapse-all = Recolher tudo
tree-collapse-selected = Recolher seleção
tree-collapse-from = Recolher a partir da seleção
tree-expand-all-hint = Expande todos os títulos
tree-expand-selected-hint = Expande apenas o título selecionado
tree-expand-from-hint = Expande o título selecionado e tudo o que está abaixo dele
tree-collapse-all-hint = Recolhe todos os títulos
tree-collapse-selected-hint = Recolhe apenas o título selecionado
tree-collapse-from-hint = Recolhe o título selecionado e tudo o que está abaixo dele

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Adicionar grupo
btn-remove-group-cond = Remover grupo
dep-type-game = Versão do jogo
dep-type-fomm = Versão do gestor de mods
dep-group-hint = Um grupo de condições combinadas com E / OU; os grupos podem ser aninhados.
condeditor-sentence-game = a versão do jogo ≥ { "{value}" }
condeditor-sentence-fomm = a versão do gestor de mods ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Textos únicos
ftr-uniques-hint = Mostrar uma linha por cada texto de origem distinto. Traduzir essa linha traduz de uma vez todas as cadeias com o mesmo texto.
ftr-uniques-synced = { $num } cadeias idênticas atualizadas.
ftr-uniques-group = { $num } cadeias partilham este texto; a sua tradução aplica-se a todas.
