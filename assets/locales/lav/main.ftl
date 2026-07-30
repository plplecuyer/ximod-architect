# XIMOD Architect - translation metadata
# @language = lav
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = latviešu valoda
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versija { $version }

# Status messages
status-ready = Gatavs
msg-save-success = FOMOD veiksmīgi saglabāts
msg-save-error = Kļūda, saglabājot FOMOD
msg-export-success = Izplatīšanas arhīvs izveidots ({ $count } faili): { $path }
msg-export-error = Kļūda, izveidojot izplatīšanas arhīvu: { $error }
msg-load-success = FOMOD veiksmīgi ielādēts
msg-load-error = Kļūda, ielādējot FOMOD
msg-merge-success = FOMOD veiksmīgi apvienots
msg-merge-error = Kļūda, apvienojot FOMOD
msg-no-root-selected = Lūdzu, vispirms izvēlieties saknes direktoriju
msg-no-fomod-folder = Nav atrasta 'fomod' mape. Izveidot?
msg-file-outside-root = Fails atrodas ārpus saknes direktorijas

# Menu - File
menu-file = Fails
menu-new = Jauns
menu-open = Atvērt mapi...
menu-open-file = Atvērt failu...
menu-save = Saglabāt
menu-recent = Nesenie
menu-exit = Iziet
menu-merge = Apvienot FOMOD...
menu-export = Eksportēt izplatīšanas arhīvu...

# Menu - Options
menu-options = Opcijas
menu-settings = Iestatījumi
menu-pre-save-script = Skripts pirms saglabāšanas...
menu-post-save-script = Skripts pēc saglabāšanas...
menu-translation = Tulkojums...

# Menu - Help
menu-help = Palīdzība
menu-about = Par programmu

# Tabs
tab-info = Modifikācijas informācija
tab-steps = Instalēšanas soļi
tab-required = Obligātās instalācijas
tab-conditional = Nosacītās instalācijas

# Info Tab
label-workspace = Darba vide
label-root-dir = Saknes katalogs:
label-mod-name = Modifikācijas nosaukums:
label-author = Autors:
label-version = Versija:
label-game-name = Spēles nosaukums:
label-category = Kategorija:
label-url = Tīmekļa vietnes URL:
label-header-image = Galvenes attēls:
label-description = Apraksts:
placeholder-select-dir = (Izvēlieties direktoriju)
placeholder-select-game = (Izvēlieties spēli)

# Steps Tab
label-step-name = Soļa nosaukums:
label-group-name = Grupas nosaukums:
label-group-type = Grupas tips:
label-plugin-name = Plugina nosaukums:
label-plugin-desc = Apraksts:
label-plugin-type = Noklusējuma tips:
label-plugin-image = Attēls:
label-visibility = Redzamības nosacījumi
label-operator = Operators:

# Buttons
btn-browse = Pārlūkot...
btn-clear = Dzēst
btn-add = Pievienot
btn-remove = Noņemt
btn-add-step = Jauns solis
btn-delete-step = Dzēst soli
btn-add-group = Pievienot grupu
btn-remove-group = Noņemt grupu
btn-add-plugin = Pievienot spraudni
btn-remove-plugin = Noņemt spraudni
btn-add-file = Pievienot failu
btn-add-folder = Pievienot mapi
btn-remove-file = Noņemt
btn-add-flag = Pievienot atzīmi
btn-remove-flag = Noņemt atzīmi
btn-add-condition = Pievienot nosacījumu
btn-remove-condition = Noņemt nosacījumu
btn-add-dependency = Pievienot atkarību
btn-remove-dependency = Noņemt atkarību
btn-add-pattern = Jauns paraugs
btn-remove-pattern = Dzēst paraugu
btn-save = Saglabāt
btn-cancel = Atcelt
btn-ok = OK
btn-yes = Jā
btn-no = Nē

# Condition/Dependency Labels
label-flag-name = Karoga nosaukums:
label-flag-value = Vērtība:
label-condition-type = Tips:
label-condition-name = Nosaukums:
label-condition-value = Vērtība:
label-dep-type = Atkarības tips:
label-dep-name = Nosaukums/fails:
label-dep-value = Vērtība/stāvoklis:

# Files
label-source = Avots
label-destination = Galamērķis
label-priority = Prioritāte
label-file-type = Tips
label-files = Fails
label-dependencies = Atkarības

# Settings Dialog
settings-title = Iestatījumi
settings-tab-general = Vispārīgi
settings-tab-recent-files = Nesenie faili
settings-language = Valoda:
settings-theme = Tēma:
settings-font-size = Fontu izmērs:
settings-replace-newlines = Apstrādāt rindu pārtraukumus aprakstos
settings-max-recent = Maksimālais nesenu failu skaits:
settings-window-width = Loga platums:
settings-window-height = Loga augstums:
settings-no-recent-files = Nav nesenu failu.

# Status messages for settings
status-settings-saved = Iestatījumi veiksmīgi saglabāti

# About Dialog
about-title = Par XIMOD Architect
about-description = Daudzplatformu rīks FOMOD instalētāju izveidei Bethesda spēļu modifikācijām.
about-license = Licencēts saskaņā ar MIT licenci
about-copyright = © 2025–2026 XIMOD Team
about-credit = Wenderer oriģinālā instrumenta Rust ports:

# Script Dialog
script-title = Rediģēt skriptu
script-info = Skripti tiek izpildīti pirms vai pēc saglabāšanas. Jūs varat izmantot šādus makrous:
script-macros = Pieejamie makroi:
macro-modname = $MODNAME$ — modifikācijas nosaukums
macro-modauthor = $MODAUTHOR$ — autora vārds
macro-modversion = $MODVERSION$ — modifikācijas versija
macro-modroot = $MODROOT$ — galvenā direktorija ceļš
macro-date = $DATE$ — pašreizējais datums (GGGG-MM-DD)
macro-time = $TIME$ — pašreizējais laiks (HH:MM:SS)
macro-random = $RANDOM$ — nejaušs skaitlis

# Plugin Dependencies
label-default-type = Noklusējuma tips:
label-pattern-type = Veidnes tips:
label-pattern-operator = Veidnes operators:

# Conditional Files
label-pattern = Veidne

# Validation Messages
validation-no-name = Nepieciešams moduļa nosaukums
validation-no-steps = Nepieciešams vismaz viens solis vai obligātais fails
validation-empty-step = Solim { $num } nav nosaukuma
validation-empty-group = Solim { $step }, grupai { $group } nav nosaukuma
validation-no-plugins = Solim { $step }, grupai "{ $name }" nav spraudņu

# File States
state-active = Aktīvs
state-inactive = Neaktīvs
state-missing = Trūkst

# Confirmation
confirm-title = Apstiprinājums
confirm-delete = Vai tiešām vēlaties dzēst šo elementu?
confirm-discard = Jums ir nesaglabātas izmaiņas. Vai vēlaties tās atcelt un turpināt?
confirm-unsaved = Jums ir nesaglabātas izmaiņas. Vai vēlaties tās saglabāt pirms aizvēršanas?
confirm-save-issues = Projektā ir šādas problēmas:
confirm-save-anyway = Saglabāt tomēr?

# Errors
error-invalid-xml = Nepareizs XML fails
error-parse-failed = Neizdevās analizēt FOMOD
error-write-failed = Neizdevās ierakstīt failu
error-create-dir = Neizdevās izveidot direktoriju

# Default names (generated when creating new items)
default-step-name = Solis { $num }
default-group-name = Grupa { $num }
default-plugin-name = Plugins { $num }
pattern-label = Šablons { $num }

# Selection prompts
msg-select-group-first = Vispirms izvēlieties grupu.
msg-select-plugin-edit = Izvēlieties spraudni, kuru vēlaties rediģēt.
label-empty = (tukšs)
image-no-image = Nav attēla

# File dialog filters
filter-images = Attēli
filter-xml = XML

# Dependency types
dep-type-flag = Karodziņš
dep-type-file = Fails

# Status bar
status-modified = Pārveidots

# Status messages (errors)
msg-settings-save-error = Kļūda, saglabājot iestatījumus
msg-script-save-error = Kļūda, saglabājot skriptu

# Translation editor
trans-title = Tulkojumu redaktors
trans-source-lang = Parādītā valoda:
trans-target-lang = Valoda, uz kuru tulkot:
trans-col-key = Atslēgvārds
trans-col-source = Nosaukums
trans-col-target = Tulkojums
trans-saved = Tulkojums saglabāts
trans-save-error = Kļūda, saglabājot tulkojumu

# XML editor
xml-editor-title = XML redaktors
xml-editor-edit = Rediģēt
xml-editor-apply = Piemērot
xml-editor-revert = Atcelt
xml-editor-readonly = Tikai lasīšanai
xml-editor-editing = Rediģēšana — grafiskās cilnes ir bloķētas
xml-editor-error = Kļūda:
xml-editor-applied = XML izmaiņas piemērotas
xml-editor-wellformed = Korrekti veidots XML
xml-editor-error-at = Rinda { $line }, kolonna { $col }: { $msg }

# Country / flag picker
settings-country-name = Valsts nosaukums:
settings-pick-country = Noklikšķiniet, lai izvēlētos savu valsti
flags-title = Izvēlieties valsti
flags-filter = Filtrs:
flags-none = Karogs nav atrasts

# Translation editor: country & font
trans-endonym = Valsts endonīms:
trans-font = Fonts:
trans-no-font = (nav)
trans-browse = Pārlūkot…
trans-google-fonts = Google Fonts
trans-pick-country = Noklikšķiniet, lai izvēlētos valsti
trans-font-outside = Fonts vispirms jāinstalē mapē assets/fonts.
trans-font-dir-missing = Mapes „assets/fonts” nav atrasta.

# Translation submission
trans-lang-endonym = Valodas nosaukums:
trans-author = Autors:
trans-submit = Nosūtīt…
trans-submit-hint = Izveidojiet zip failu un atveriet iepriekš aizpildītu e-pastu
trans-data-updated = Atsauces dati atjaunināti (Languages.json / Countries.json)
trans-package-ready = Arhīvs gatavs:
trans-package-error = Neizdevās izveidot arhīvu:

# ISO 639-3 requirement
trans-lang-not-iso = Tulkojums ir iespējams tikai valodai ar ISO 639-3 kodu.

# FOMOD installer preview
menu-preview = Instalētāja priekšskatījums…
preview-title = FOMOD instalētāja priekšskatījums
preview-refresh = Atjaunināt
preview-assumptions = Failu pieņēmumi
preview-details = Detalizēta informācija
preview-back = Atpakaļ
preview-next = Tālāk
preview-install = Instalēt
preview-close = Aizvērt
preview-restart = Pārstartēt
preview-summary-title = Faili, kas tiks instalēti
preview-empty = Netiks instalēts neviens fails.
preview-none-option = (nav)
preview-invalid = Lai turpinātu, veiciet nepieciešamās izvēles.
preview-no-steps = Nav redzams neviens solis; skatiet instalēšanas kopsavilkumu.
preview-select-hint = Izvēlieties opciju, lai redzētu tās aprakstu.
preview-col-source = Avots
preview-col-dest = Galamērķis
preview-col-priority = Prioritāte
preview-sel-exactlyone = Izvēlieties tieši vienu opciju.
preview-sel-atmostone = Izvēlieties ne vairāk kā vienu opciju.
preview-sel-any = Izvēlieties jebkuru opciju skaitu.
preview-sel-all = Visas opcijas ir instalētas.
preview-sel-atleastone = Izvēlieties vismaz vienu opciju.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validēt FOMOD
validate-report-title = FOMOD validācija
validate-ok = Problēmas nav atrastas. FOMOD atbilst shēmai.
xml-editor-schema-ok = Atbilst ModConfig 5.0 shēmai.
xml-editor-schema-issues = Shēmas problēmas:
schema-line-col = Rinda { $line }, kolonna { $col }: { $msg }
schema-wrong-root = Negaidīts saknes elements "{ $found }" (gaidīts "{ $expected }").
schema-unknown = Negaidīts elements "{ $element }" elementā "{ $parent }".
schema-missing = "{ $parent }" jāietver "{ $child }".
schema-needs-one = "{ $parent }" jāietver vismaz viens "{ $child }".
schema-too-many = „{ $child }“ drīkst parādīties tikai vienu reizi „{ $parent }“.
schema-missing-attr = Atribūtam „{ $element }“ ir obligāts atribūts „{ $attr }“.
schema-bad-enum = Nepareiza vērtība „{ $value }” elementam { $element }/@{ $attr } (paredzēts: { $allowed }).
schema-choose-one = „{ $parent }” jāietver tieši viens no: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Pārvietot pirms
reorder-after = Pārvietot pēc

# Country / language database explorer (Properties)
menu-properties = Īpašības…
prop-title = Valstu / valodu datu bāze
prop-tab-countries = Valstis
prop-tab-languages = Valodas
prop-filter = Filtrs:
prop-official-langs = Oficiālās valodas
prop-spoken-langs = Runātās valodas
prop-endonym = Valsts endonīms
prop-font = Fonti
prop-spoken-in = Runā
prop-select-country = Izvēlieties valsti, lai apskatītu tās informāciju.
prop-select-lang = Izvēlieties valodu, lai apskatītu tās informāciju.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Atveriet spēles Nexus Mods lapu
