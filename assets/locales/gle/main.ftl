# XIMOD Architect - translation metadata
# @language = gle
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Gaeilge
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Leagan { $version }

# Status messages
status-ready = Réidh
msg-save-success = Sábháilte FOMOD go rathúil
msg-save-error = Earráid agus FOMOD á shábháil
msg-export-success = Cartlann dáilte cruthaithe ({ $count } comhad): { $path }
msg-export-error = Earráid agus an cartlann dáilte á chruthú: { $error }
msg-load-success = Lódáilte FOMOD go rathúil
msg-load-error = Earráid ag lódáil FOMOD
msg-merge-success = Comhcheanglaíodh FOMOD go rathúil
msg-merge-error = Earráid ag comhcheangal FOMOD
msg-no-root-selected = Roghnaigh eolaire fréimhe ar dtús, le do thoil
msg-no-fomod-folder = Níl aon eolaire 'fomod' aimsithe. Ceann a chruthú?
msg-file-outside-root = Tá an comhad lasmuigh den eolaire fréimhe

# Menu - File
menu-file = Comhad
menu-new = Nua
menu-open = Oscail Fillteán...
menu-open-file = Oscail Comhad...
menu-save = Sábháil
menu-recent = Is Déanaí
menu-exit = Scoir
menu-merge = Cumaisc FOMOD...
menu-export = Easpórtáil cartlann dáilte...

# Menu - Options
menu-options = Roghanna
menu-settings = Socruithe
menu-pre-save-script = Scriptiúr Réamhshábhála...
menu-post-save-script = Scriptiúr Iarshábhála...
menu-translation = Aistriúchán...

# Menu - Help
menu-help = Cabhair
menu-about = Maidir le

# Tabs
tab-info = Eolas faoin Mod
tab-steps = Céimeanna Suiteála
tab-required = Suiteálacha Riachtanacha
tab-conditional = Suiteálacha Coinníollacha

# Info Tab
label-workspace = Spás Oibre
label-root-dir = Eolaire Fréimhe:
label-mod-name = Ainm an Mhod:
label-author = Údar:
label-version = Leagan:
label-game-name = Ainm an Chluiche:
label-category = Catagóir:
label-url = URL an tSuímh Ghréasáin:
label-header-image = Íomhá Ceanntásc:
label-description = Cur síos:
placeholder-select-dir = (Roghnaigh eolaire)
placeholder-select-game = (Roghnaigh cluiche)

# Steps Tab
label-step-name = Ainm na Céime:
label-group-name = Ainm an Ghrúpa:
label-group-type = Cineál an Ghrúpa:
label-plugin-name = Ainm an Phlugáin:
label-plugin-desc = Cur síos:
label-plugin-type = Cineál Réamhshocraithe:
label-plugin-image = Íomhá:
label-visibility = Coinníollacha Infheictheachta
label-operator = Oibritheoir:

# Buttons
btn-browse = Brabhsáil...
btn-clear = Glan
btn-add = Cuir Leis
btn-remove = Bain
btn-add-step = Céim Nua
btn-delete-step = Scrios Céim
btn-add-group = Cuir Grúpa Leis
btn-remove-group = Bain Grúpa
btn-add-plugin = Cuir Breiseán Leis
btn-remove-plugin = Bain an Breiseán
btn-add-file = Cuir Comhad Leis
btn-add-folder = Cuir Fillteán Leis
btn-remove-file = Bain
btn-add-flag = Cuir Bratach Leis
btn-remove-flag = Bain an Bhratach
btn-add-condition = Cuir Coinníoll Leis
btn-remove-condition = Bain an Coinníoll
btn-add-dependency = Cuir Spleáchas Leis
btn-remove-dependency = Bain an Spleáchas
btn-add-pattern = Patrún Nua
btn-remove-pattern = Scrios an Patrún
btn-save = Sábháil
btn-cancel = Cealaigh
btn-ok = Ceart
btn-yes = Tá
btn-no = Níl

# Condition/Dependency Labels
label-flag-name = Ainm an Bhrataigh:
label-flag-value = Luach:
label-condition-type = Cineál:
label-condition-name = Ainm:
label-condition-value = Luach:
label-dep-type = Cineál Spleáchais:
label-dep-name = Ainm/Comhad:
label-dep-value = Luach/Stádas:

# Files
label-source = Foinse
label-destination = Ceann scríbe
label-priority = Tosaíocht
label-file-type = Cineál
label-files = Comhaid
label-dependencies = Spleáchais

# Settings Dialog
settings-title = Socruithe
settings-tab-general = Ginearálta
settings-tab-recent-files = Comhaid is Déanaí
settings-language = Teanga:
settings-theme = Téama:
settings-font-size = Méid an Chló:
settings-replace-newlines = Próiseáil líne nua i gcur síos
settings-max-recent = Uasmhéid Comhad Nuashonraithe:
settings-window-width = Leathan an Fhuinneoige:
settings-window-height = Airde an Fhuinneoige:
settings-no-recent-files = Níl aon chomhaid nuashonraithe.

# Status messages for settings
status-settings-saved = Sábháilte na socruithe go rathúil

# About Dialog
about-title = Faoi XIMOD Architect
about-description = Uirlis tras-ardáin chun suiteálaithe FOMOD a chruthú do mhodhnuithe ar chluichí Bethesda.
about-license = Ceadúnaithe faoin gCeadúnas MIT
about-copyright = © 2025-2026 Foireann XIMOD
about-credit = Port Rust den uirlis bhunaidh ag Wenderer:

# Script Dialog
script-title = Eagar a chur ar an Scrípt
script-info = Ritheann scripteanna roimh shábháil nó ina diaidh. Is féidir leat na macraí seo a leanas a úsáid:
script-macros = Macraí ar Fáil:
macro-modname = $MODNAME$ - Ainm an mhod
macro-modauthor = $MODAUTHOR$ - Ainm an údair
macro-modversion = $MODVERSION$ - Leagan an mhod
macro-modroot = $MODROOT$ - Conair an fhreastalaí fhréamh
macro-date = $DATE$ - An dáta reatha (BBBB-MM-LL)
macro-time = $TIME$ - An t-am reatha (HH:MM:SS)
macro-random = $RANDOM$ - Uimhir randamach

# Plugin Dependencies
label-default-type = Cineál Réamhshocraithe:
label-pattern-type = Cineál Patrúin:
label-pattern-operator = Oibritheoir Patrúin:

# Conditional Files
label-pattern = Patrún

# Validation Messages
validation-no-name = Tá ainm an mhodha riachtanach
validation-no-steps = Tá céim amháin ar a laghad nó comhad riachtanach de dhíth
validation-empty-step = Níl ainm ar chéim { $num }
validation-empty-group = Níl ainm ar chéim { $step }, ar ghrúpa { $group }
validation-no-plugins = Níl breiseáin ar chéim { $step }, ar ghrúpa "{ $name }"

# File States
state-active = Gníomhach
state-inactive = Neamhghníomhach
state-missing = Ineas

# Confirmation
confirm-title = Deimhniú
confirm-delete = An bhfuil tú cinnte gur mian leat an t-ítim seo a scriosadh?
confirm-discard = Tá athruithe gan shábháil agat. An gcaillfear iad agus an leanfaidh tú ar aghaidh?
confirm-unsaved = Tá athruithe gan shábháil agat. Ar mhaith leat iad a shábháil sula ndúnann tú?
confirm-save-issues = Tá na fadhbanna seo a leanas ag an tionscadal:
confirm-save-anyway = Sábháil ar aon nós?

# Errors
error-invalid-xml = Comhad XML neamhbhailí
error-parse-failed = Theip ar FOMOD a anailísiú
error-write-failed = Theip ar an gcomhad a scríobh
error-create-dir = Theip ar an eolaire a chruthú

# Default names (generated when creating new items)
default-step-name = Céim { $num }
default-group-name = Grúpa { $num }
default-plugin-name = Breiseán { $num }
pattern-label = Patrún { $num }

# Selection prompts
msg-select-group-first = Roghnaigh grúpa ar dtús.
msg-select-plugin-edit = Roghnaigh breiseán le cur in eagar.
label-empty = (folamh)
image-no-image = Níl íomhá ann

# File dialog filters
filter-images = Íomhánna
filter-xml = XML

# Dependency types
dep-type-flag = Bratach
dep-type-file = Comhad

# Status bar
status-modified = Modhnaithe

# Status messages (errors)
msg-settings-save-error = Earráid agus socruithe á sábháil
msg-script-save-error = Earráid agus script á sábháil

# Translation editor
trans-title = Eagarthóir Aistriúcháin
trans-source-lang = Teanga thaispeáinte:
trans-target-lang = Teanga le haistriú:
trans-col-key = Eochair
trans-col-source = Lipéad
trans-col-target = Aistriúchán
trans-saved = Aistriúchán sábháilte
trans-save-error = Earráid agus an t-aistriúchán á shábháil

# XML editor
xml-editor-title = Eagarthóir XML
xml-editor-edit = Cuir in eagar
xml-editor-apply = Cuir i bhfeidhm
xml-editor-revert = Cealaigh
xml-editor-readonly = Léite amháin
xml-editor-editing = Ag cur in eagar — tá na cluaisíní grafacha faoi ghlas
xml-editor-error = Earráid:
xml-editor-applied = Athruithe XML curtha i bhfeidhm
xml-editor-wellformed = XML dea-fhoirmithe
xml-editor-error-at = Líne { $line }, colún { $col }: { $msg }

# Country / flag picker
settings-country-name = Ainm na tíre:
settings-pick-country = Cliceáil chun do thír a roghnú
flags-title = Roghnaigh tír
flags-filter = Scagaire:
flags-none = Níl aon bhratach aimsithe

# Translation editor: country & font
trans-endonym = Endainim na tíre:
trans-font = Cló:
trans-no-font = (gan aon cheann)
trans-browse = Brabhsáil…
trans-google-fonts = Clónna Google
trans-pick-country = Cliceáil chun an tír a roghnú
trans-font-outside = Caithfear an cló a shuiteáil in assets/fonts ar dtús.
trans-font-dir-missing = Níor aimsíodh an fillteán assets/fonts.

# Translation submission
trans-lang-endonym = Endainim na teanga:
trans-author = Údar:
trans-submit = Seol…
trans-submit-hint = Cruthaigh comhad zip agus oscail ríomhphost réamh-líonta
trans-data-updated = Tuarascálacha nuashonraithe (Languages.json / Countries.json)
trans-package-ready = Cartlann réidh:
trans-package-error = Níorbh fhéidir an cartlann a chruthú:

# ISO 639-3 requirement
trans-lang-not-iso = Níl an aistriúchán indéanta ach amháin do theanga a bhfuil cód ISO 639-3 aici.

# FOMOD installer preview
menu-preview = Réamhamharc ar an suiteálaí…
preview-title = Réamhamharc suiteálaí FOMOD
preview-refresh = Athnuachan
preview-assumptions = Toimhdí comhaid
preview-details = Sonraí
preview-back = Ar ais
preview-next = Ar aghaidh
preview-install = Suiteáil
preview-close = Dún
preview-restart = Atosaigh
preview-summary-title = Comhaid a shuiteálfar
preview-empty = Ní shuiteálfaí aon chomhad.
preview-none-option = (nialasach)
preview-invalid = Comhlánaigh na roghanna riachtanacha chun leanúint ar aghaidh.
preview-no-steps = Níl aon chéim le feiceáil; féach ar an achoimre suiteála.
preview-select-hint = Roghnaigh rogha chun a cur síos a fheiceáil.
preview-col-source = Foinse
preview-col-dest = Ceann scríbe
preview-col-priority = Tosaíocht
preview-sel-exactlyone = Roghnaigh go díreach rogha amháin.
preview-sel-atmostone = Roghnaigh suas le rogha amháin.
preview-sel-any = Roghnaigh líon ar bith roghanna.
preview-sel-all = Tá na roghanna go léir suiteáilte.
preview-sel-atleastone = Roghnaigh roghanna amháin ar a laghad.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Fíoraigh FOMOD
validate-report-title = Fíorú FOMOD
validate-ok = Níor aimsíodh aon fhadhb. Comhlíonann an FOMOD an scéimre.
xml-editor-schema-ok = Comhlíonann sé scéimre ModConfig 5.0.
xml-editor-schema-issues = Fadhbanna scéime:
schema-line-col = Líne { $line }, col. { $col }: { $msg }
schema-wrong-root = Fréamh gan choinne "{ $found }" (a rabhthas ag súil le "{ $expected }").
schema-unknown = Eilimint neamhionchais "{ $element }" in "{ $parent }".
schema-missing = Caithfidh "{ $parent }" "{ $child }" a bheith ann.
schema-needs-one = Caithfidh "{ $parent }" ar a laghad "{ $child }" amháin a bheith ann.
schema-too-many = Ní fhéadfaidh "{ $child }" a bheith le feiceáil ach uair amháin in "{ $parent }".
schema-missing-attr = Tá an tréith "{ $attr }" riachtanach ar "{ $element }".
schema-bad-enum = Luach neamhbhailí "{ $value }" do { $element }/@{ $attr } (a rabhthas ag súil leis: { $allowed }).
schema-choose-one = Caithfidh "{ $parent }" a bheith ann go díreach ceann amháin de: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Bogadh roimh
reorder-after = Bogadh i ndiaidh

# Country / language database explorer (Properties)
menu-properties = Airíonna…
prop-title = Bunachar sonraí tíre / teanga
prop-tab-countries = Tíortha
prop-tab-languages = Teangacha
prop-filter = Scagaire:
prop-official-langs = Teangacha oifigiúla
prop-spoken-langs = Teangacha labhartha
prop-endonym = Endainim tíre
prop-font = Cló
prop-spoken-in = Labhartha in
prop-select-country = Roghnaigh tír chun a sonraí a fheiceáil.
prop-select-lang = Roghnaigh teanga chun a sonraí a fheiceáil.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Oscail leathanach Nexus Mods an chluiche
