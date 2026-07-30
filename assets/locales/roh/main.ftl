# XIMOD Architect - translation metadata
# @language = roh
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Rumantsch Grischun
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versiun { $version }

# Status messages
status-ready = Pront
msg-save-success = FOMOD memorisà cun success
msg-save-error = Errur durant la memorisaziun da FOMOD
msg-export-success = Archiv da distribuziun creà ({ $count } datotecas): { $path }
msg-export-error = Errur durant la creaziun da l'archiv da distribuziun: { $error }
msg-load-success = FOMOD chargià cun success
msg-load-error = Errur durant il chargiar FOMOD
msg-merge-success = FOMOD fusiunà cun success
msg-merge-error = Errur durant la fusiun da FOMOD
msg-no-root-selected = Tscherna l'emprim in ordinatur da basa
msg-no-fomod-folder = Nagin ordinatur 'fomod' chattà. Crear in?
msg-file-outside-root = La datoteca è ordaifer l'ordinatur da basa

# Menu - File
menu-file = Datoteca
menu-new = Nov
menu-open = Avrir ordinatur...
menu-open-file = Avrir datoteca...
menu-save = Memorisar
menu-recent = Recents
menu-exit = Terminar
menu-merge = Fusiunar FOMOD...
menu-export = Exportar archiv da distribuziun...

# Menu - Options
menu-options = Opziuns
menu-settings = Configuraziuns
menu-pre-save-script = Script avant memorisar...
menu-post-save-script = Script suenter memorisar...
menu-translation = Translaziun...

# Menu - Help
menu-help = Agid
menu-about = Davart

# Tabs
tab-info = Infurmaziuns dal mod
tab-steps = Pass d'installaziun
tab-required = Installaziuns obligatoricas
tab-conditional = Installaziuns cundiziunalas

# Info Tab
label-workspace = Spazi da lavur
label-root-dir = Ordinatur da basa:
label-mod-name = Num dal mod:
label-author = Autur:
label-version = Versiun:
label-game-name = Num dal gieu:
label-category = Categoria:
label-url = URL da la pagina d’internet:
label-header-image = Maletg da chau:
label-description = Descripziun:
placeholder-select-dir = (Tscherner in ordinatur)
placeholder-select-game = (Tscherner in gieu)

# Steps Tab
label-step-name = Num dal pass:
label-group-name = Num da la gruppa:
label-group-type = Tip da gruppa:
label-plugin-name = Num dal plugin:
label-plugin-desc = Descripziun:
label-plugin-type = Tip standard:
label-plugin-image = Maletg:
label-visibility = Cundiziuns da visibilitad
label-operator = Operatur:

# Buttons
btn-browse = Tschertgar...
btn-clear = Svidar
btn-add = Agiuntar
btn-remove = Allontanar
btn-add-step = Nov pass
btn-delete-step = Stizzar pass
btn-add-group = Agiuntar gruppa
btn-remove-group = Allontanar gruppa
btn-add-plugin = Agiuntar plugin
btn-remove-plugin = Allontanar plugin
btn-add-file = Agiuntar datoteca
btn-add-folder = Agiuntar ordinatur
btn-remove-file = Allontanar
btn-add-flag = Agiuntar flag
btn-remove-flag = Allontanar flag
btn-add-condition = Agiuntar cundiziun
btn-remove-condition = Allontanar cundiziun
btn-add-dependency = Agiuntar dependenza
btn-remove-dependency = Allontanar dependenza
btn-add-pattern = Nov muster
btn-remove-pattern = Stizzar muster
btn-save = Memorisar
btn-cancel = Annullar
btn-ok = OK
btn-yes = Gea
btn-no = Na

# Condition/Dependency Labels
label-flag-name = Num dal flag:
label-flag-value = Valur:
label-condition-type = Tip:
label-condition-name = Num:
label-condition-value = Valur:
label-dep-type = Tip da dependenza:
label-dep-name = Num/datoteca:
label-dep-value = Valur/stadi:

# Files
label-source = Funtauna
label-destination = Destinaziun
label-priority = Prioritad
label-file-type = Tip
label-files = Datotecas
label-dependencies = Dependenzas

# Settings Dialog
settings-title = Configuraziuns
settings-tab-general = General
settings-tab-recent-files = Datotecas recentas
settings-language = Lingua:
settings-theme = Tema:
settings-font-size = Grondezza da scrittira:
settings-replace-newlines = Elavurar rupturas da lingia en las descripziuns
settings-max-recent = Dumber maximal da datotecas recentas:
settings-window-width = Ladezza da la fanestra:
settings-window-height = Autezza da la fanestra:
settings-no-recent-files = Naginas datotecas recentas.

# Status messages for settings
status-settings-saved = Configuraziuns memorisadas cun success

# About Dialog
about-title = Davart XIMOD Architect
about-description = In instrument multiplattafurma per crear installaders FOMOD per mods da gieus Bethesda.
about-license = Licenzià tenor la licenza MIT
about-copyright = © 2025-2026 XIMOD Team
about-credit = Versiun en Rust da l'utensil original da Wenderer:

# Script Dialog
script-title = Modifitgar script
script-info = Ils scripts vegnan executads avant u suenter la memorisaziun. Ti pos duvrar las suandantas macros:
script-macros = Macros disponiblas:
macro-modname = $MODNAME$ - Num dal mod
macro-modauthor = $MODAUTHOR$ - Num da l’autur
macro-modversion = $MODVERSION$ - Versiun dal mod
macro-modroot = $MODROOT$ - Via da l’ordinatur da basa
macro-date = $DATE$ - Data actuala (YYYY-MM-DD)
macro-time = $TIME$ - Ura actuala (HH:MM:SS)
macro-random = $RANDOM$ - Numer casual

# Plugin Dependencies
label-default-type = Tip standard:
label-pattern-type = Tip da muster:
label-pattern-operator = Operatur dal muster:

# Conditional Files
label-pattern = Muster

# Validation Messages
validation-no-name = Il num dal mod è obligatoric
validation-no-steps = I dovra almain in pass u ina datoteca obligatorica
validation-empty-step = Il pass { $num } n’ha nagin num
validation-empty-group = Il pass { $step }, gruppa { $group }, n’ha nagin num
validation-no-plugins = Il pass { $step }, gruppa "{ $name }", n’ha nagins plugins

# File States
state-active = Activ
state-inactive = Inactiv
state-missing = Mancant

# Confirmation
confirm-title = Conferma
confirm-delete = Es ti segir che ti vuls stizzar quest element?
confirm-discard = Ti has midadas betg memorisadas. Las descartar e cuntinuar?
confirm-unsaved = Ti has midadas betg memorisadas. Vuls ti memorisar avant che serrar?
confirm-save-issues = Il project ha ils suandants problems:
confirm-save-anyway = Memorisar tuttina?

# Errors
error-invalid-xml = Datoteca XML nunvalida
error-parse-failed = Impussibel d’analisar FOMOD
error-write-failed = Impussibel da scriver la datoteca
error-create-dir = Impussibel da crear l’ordinatur

# Default names (generated when creating new items)
default-step-name = Pass { $num }
default-group-name = Gruppa { $num }
default-plugin-name = Plugin { $num }
pattern-label = Muster { $num }

# Selection prompts
msg-select-group-first = Tscherna l'emprim ina gruppa.
msg-select-plugin-edit = Tscherna in plugin da modifitgar.
label-empty = (vid)
image-no-image = Nagin maletg

# File dialog filters
filter-images = Maletgs
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = Datoteca

# Status bar
status-modified = Modifitgà

# Status messages (errors)
msg-settings-save-error = Errur durant la memorisaziun da las configuraziuns
msg-script-save-error = Errur durant la memorisaziun dal script

# Translation editor
trans-title = Editur da translaziuns
trans-source-lang = Lingua mussada:
trans-target-lang = Lingua da translatar:
trans-col-key = Clav
trans-col-source = Inscripziun
trans-col-target = Translaziun
trans-saved = Translaziun memorisada
trans-save-error = Errur durant la memorisaziun da la translaziun

# XML editor
xml-editor-title = Editur XML
xml-editor-edit = Modifitgar
xml-editor-apply = Applitgar
xml-editor-revert = Annullar
xml-editor-readonly = Mo lectura
xml-editor-editing = Modificaziun — ils tabs grafics èn bloccads
xml-editor-error = Errur:
xml-editor-applied = Midadas XML applitgadas
xml-editor-wellformed = XML bain furmà
xml-editor-error-at = Lingia { $line }, colonna { $col }: { $msg }

# Country / flag picker
settings-country-name = Num dal pajais:
settings-pick-country = Cliccar per tscherner il pajais
flags-title = Tscherner in pajais
flags-filter = Filter:
flags-none = Nagina bandiera chattada

# Translation editor: country & font
trans-endonym = Endonim dal pajais:
trans-font = Font:
trans-no-font = (nagin)
trans-browse = Tschertgar…
trans-google-fonts = Google Fonts
trans-pick-country = Cliccar per tscherner il pajais
trans-font-outside = Il font sto vegnir installà l’emprim en assets/fonts.
trans-font-dir-missing = L’ordinatur assets/fonts n’è betg vegnì chattà.

# Translation submission
trans-lang-endonym = Endonim da la lingua:
trans-author = Autur:
trans-submit = Trametter…
trans-submit-hint = Crear in archiv ZIP ed avrir in e-mail emplenì ordavant
trans-data-updated = Datas da referenza actualisadas (Languages.json / Countries.json)
trans-package-ready = Archiv pront:
trans-package-error = Impussibel da crear l’archiv:

# ISO 639-3 requirement
trans-lang-not-iso = La translaziun è pussaivla mo per ina lingua cun in code ISO 639-3.

# FOMOD installer preview
menu-preview = Prevista da l’installader…
preview-title = Prevista da l’installader FOMOD
preview-refresh = Actualisar
preview-assumptions = Premissas davart las datotecas
preview-details = Detagls
preview-back = Enavos
preview-next = Enavant
preview-install = Installar
preview-close = Serrar
preview-restart = Cumenzar da nov
preview-summary-title = Datotecas che vegnan installadas
preview-empty = Nagina datoteca vegniss installada.
preview-none-option = (nagina)
preview-invalid = Complettar las selecziuns obligatoricas per cuntinuar.
preview-no-steps = Nagin pass è visibel; vesair il resumaziun da l’installaziun.
preview-select-hint = Tscherna ina opziun per vesair sia descripziun.
preview-col-source = Funtauna
preview-col-dest = Destinaziun
preview-col-priority = Prioritad
preview-sel-exactlyone = Tscherna exactamain ina opziun.
preview-sel-atmostone = Tscherna maximalmain ina opziun.
preview-sel-any = Tscherna tantas opziuns sco giavischà.
preview-sel-all = Tut las opziuns vegnan installadas.
preview-sel-atleastone = Tscherna almain ina opziun.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validar FOMOD
validate-report-title = Validaziun da FOMOD
validate-ok = Nagin problem chattà. Il FOMOD correspunda al schema.
xml-editor-schema-ok = Correspunda al schema ModConfig 5.0.
xml-editor-schema-issues = Problems dal schema:
schema-line-col = Lingia { $line }, col. { $col }: { $msg }
schema-wrong-root = Ragisch nunspetgada "{ $found }" (spetgà "{ $expected }").
schema-unknown = Element nunspetgà "{ $element }" en "{ $parent }".
schema-missing = "{ $parent }" sto cuntegnair "{ $child }".
schema-needs-one = "{ $parent }" sto cuntegnair almain in "{ $child }".
schema-too-many = "{ $child }" dastga cumparair mo ina giada en "{ $parent }".
schema-missing-attr = L’attribut "{ $attr }" è obligatoric en "{ $element }".
schema-bad-enum = Valur nunvalida "{ $value }" per { $element }/@{ $attr } (spetgà: { $allowed }).
schema-choose-one = "{ $parent }" sto cuntegnair exactamain in da: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Spustar avant
reorder-after = Spustar suenter

# Country / language database explorer (Properties)
menu-properties = Proprietads…
prop-title = Banca da datas da pajais e linguas
prop-tab-countries = Pajais
prop-tab-languages = Linguas
prop-filter = Filter:
prop-official-langs = Linguas uffizialas
prop-spoken-langs = Linguas discurridas
prop-endonym = Endonim dal pajais
prop-font = Font
prop-spoken-in = Discurrida en
prop-select-country = Tscherna in pajais per vesair ses detagls.
prop-select-lang = Tscherna ina lingua per vesair ses detagls.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Avrir la pagina Nexus Mods dal gieu
