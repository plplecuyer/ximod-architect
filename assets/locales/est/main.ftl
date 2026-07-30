# XIMOD Architect - translation metadata
# @language = est
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Eesti
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versioon { $version }

# Status messages
status-ready = Valmis
msg-save-success = FOMOD salvestati edukalt
msg-save-error = Viga FOMOD-i salvestamisel
msg-export-success = Jaotamisarhiiv loodud ({ $count } faili): { $path }
msg-export-error = Viga jaotamisarhiivi loomisel: { $error }
msg-load-success = FOMOD laaditi edukalt
msg-load-error = FOMOD-i laadimisel tekkis viga
msg-merge-success = FOMOD ühendati edukalt
msg-merge-error = FOMOD-i ühendamisel tekkis viga
msg-no-root-selected = Palun vali esmalt juurkataloog
msg-no-fomod-folder = 'fomod'-kataloogi ei leitud. Kas soovid selle luua?
msg-file-outside-root = Fail asub väljaspool juurkataloogi

# Menu - File
menu-file = Fail
menu-new = Uus
menu-open = Ava kaust...
menu-open-file = Ava fail...
menu-save = Salvesta
menu-recent = Viimased
menu-exit = Välju
menu-merge = Ühenda FOMOD...
menu-export = Ekspordi levitusarhiiv...

# Menu - Options
menu-options = Valikud
menu-settings = Seaded
menu-pre-save-script = Skript enne salvestamist...
menu-post-save-script = Skript pärast salvestamist...
menu-translation = Tõlge...

# Menu - Help
menu-help = Abi
menu-about = Info

# Tabs
tab-info = Modi info
tab-steps = Paigaldamise sammud
tab-required = Nõutavad paigaldused
tab-conditional = Tingimuslikud paigaldused

# Info Tab
label-workspace = Töökeskkond
label-root-dir = Juurkataloog:
label-mod-name = Modi nimi:
label-author = Autor:
label-version = Versioon:
label-game-name = Mängu nimi:
label-category = Kategooria:
label-url = Veebisaidi URL:
label-header-image = Pealkirja pilt:
label-description = Kirjeldus:
placeholder-select-dir = (Vali kataloog)
placeholder-select-game = (Vali mäng)

# Steps Tab
label-step-name = Sammu nimi:
label-group-name = Rühma nimi:
label-group-type = Rühma tüüp:
label-plugin-name = Plugin nimi:
label-plugin-desc = Kirjeldus:
label-plugin-type = Vaikimisi tüüp:
label-plugin-image = Pilt:
label-visibility = Nähtavuse tingimused
label-operator = Operaator:

# Buttons
btn-browse = Sirvi...
btn-clear = Tühjenda
btn-add = Lisa
btn-remove = Eemalda
btn-add-step = Uus samm
btn-delete-step = Kustuta samm
btn-add-group = Lisa rühm
btn-remove-group = Eemalda rühm
btn-add-plugin = Lisa pistikprogramm
btn-remove-plugin = Eemalda pistikprogramm
btn-add-file = Lisa fail
btn-add-folder = Lisa kaust
btn-remove-file = Eemalda
btn-add-flag = Lisa märge
btn-remove-flag = Eemalda märge
btn-add-condition = Lisa tingimus
btn-remove-condition = Eemalda tingimus
btn-add-dependency = Lisa sõltuvus
btn-remove-dependency = Eemalda sõltuvus
btn-add-pattern = Uus muster
btn-remove-pattern = Kustuta muster
btn-save = Salvesta
btn-cancel = Tühista
btn-ok = OK
btn-yes = Jah
btn-no = Ei

# Condition/Dependency Labels
label-flag-name = Lipu nimi:
label-flag-value = Väärtus:
label-condition-type = Tüüp:
label-condition-name = Nimi:
label-condition-value = Väärtus:
label-dep-type = Sõltuvuse tüüp:
label-dep-name = Nimi/fail:
label-dep-value = Väärtus/seisund:

# Files
label-source = Allikas
label-destination = Sihtkoht
label-priority = Prioriteet
label-file-type = Tüüp
label-files = Failid
label-dependencies = Sõltuvused

# Settings Dialog
settings-title = Seaded
settings-tab-general = Üldine
settings-tab-recent-files = Viimased failid
settings-language = Keel:
settings-theme = Teema:
settings-font-size = Kirjasuurus:
settings-replace-newlines = Töötle kirjeldustes olevaid ridadevahetusi
settings-max-recent = Viimaste failide maksimaalne arv:
settings-window-width = Akna laius:
settings-window-height = Akna kõrgus:
settings-no-recent-files = Viimaseid faile pole.

# Status messages for settings
status-settings-saved = Seaded salvestati edukalt

# About Dialog
about-title = Teave XIMOD Architecti kohta
about-description = Platvormiülene FOMOD-installeri loomise tööriist Bethesda mängude modifikatsioonidele.
about-license = Litsentsitud MIT-litsentsi alusel
about-copyright = © 2025–2026 XIMOD Team
about-credit = Wenderer originaaltööriista Rust port:

# Script Dialog
script-title = Skripti redigeerimine
script-info = Skripte täidetakse enne või pärast salvestamist. Võite kasutada järgmisi makrosid:
script-macros = Saadaval olevad makrod:
macro-modname = $MODNAME$ – modifikatsiooni nimi
macro-modauthor = $MODAUTHOR$ – autori nimi
macro-modversion = $MODVERSION$ – modifikatsiooni versioon
macro-modroot = $MODROOT$ – juurkataloogi tee
macro-date = $DATE$ – praegune kuupäev (AAAA-KK-PP)
macro-time = $TIME$ – praegune kellaaeg (HH:MM:SS)
macro-random = $RANDOM$ – juhuslik number

# Plugin Dependencies
label-default-type = Vaikimisi tüüp:
label-pattern-type = Mustri tüüp:
label-pattern-operator = Mustri operaator:

# Conditional Files
label-pattern = Muster

# Validation Messages
validation-no-name = Mooduli nimi on kohustuslik
validation-no-steps = Vaja on vähemalt ühte sammu või kohustuslikku faili
validation-empty-step = Sammul { $num } puudub nimi
validation-empty-group = Sammul { $step }, rühmal { $group } puudub nimi
validation-no-plugins = Sammul { $step }, rühmal "{ $name }" puuduvad pistikprogrammid

# File States
state-active = Aktiivne
state-inactive = Mitteaktiivne
state-missing = Puudub

# Confirmation
confirm-title = Kinnitus
confirm-delete = Kas soovite seda elementi kindlasti kustutada?
confirm-discard = Teil on salvestamata muudatusi. Kas soovite need tühistada ja jätkata?
confirm-unsaved = Teil on salvestamata muudatusi. Kas soovite enne sulgemist salvestada?
confirm-save-issues = Projektil on järgmised probleemid:
confirm-save-anyway = Kas soovite ikkagi salvestada?

# Errors
error-invalid-xml = Kehtetu XML-fail
error-parse-failed = FOMOD-i analüüsimine ebaõnnestus
error-write-failed = Faili kirjutamine ebaõnnestus
error-create-dir = Kataloogi loomine ebaõnnestus

# Default names (generated when creating new items)
default-step-name = Samm { $num }
default-group-name = Rühm { $num }
default-plugin-name = Pistikprogramm { $num }
pattern-label = Muster { $num }

# Selection prompts
msg-select-group-first = Valige kõigepealt grupp.
msg-select-plugin-edit = Valige redigeeritav plugin.
label-empty = (tühi)
image-no-image = Pilt puudub

# File dialog filters
filter-images = Pildid
filter-xml = XML

# Dependency types
dep-type-flag = Lipuke
dep-type-file = Fail

# Status bar
status-modified = Muudetud

# Status messages (errors)
msg-settings-save-error = Viga seadete salvestamisel
msg-script-save-error = Viga skripti salvestamisel

# Translation editor
trans-title = Tõlkeeditor
trans-source-lang = Kuvatav keel:
trans-target-lang = Tõlgitav keel:
trans-col-key = Võti
trans-col-source = Silt
trans-col-target = Tõlge
trans-saved = Tõlge salvestatud
trans-save-error = Tõlke salvestamisel tekkis viga

# XML editor
xml-editor-title = XML-redaktor
xml-editor-edit = Redigeeri
xml-editor-apply = Rakenda
xml-editor-revert = Tühista
xml-editor-readonly = Ainult lugemiseks
xml-editor-editing = Redigeerimine — graafilised vahekaardid on lukustatud
xml-editor-error = Viga:
xml-editor-applied = XML-muudatused rakendatud
xml-editor-wellformed = Korrektselt vormindatud XML
xml-editor-error-at = Rida { $line }, veerg { $col }: { $msg }

# Country / flag picker
settings-country-name = Riigi nimi:
settings-pick-country = Klõpsa, et valida oma riik
flags-title = Vali riik
flags-filter = Filter:
flags-none = Lipu ei leitud

# Translation editor: country & font
trans-endonym = Riigi endonüüm:
trans-font = Font:
trans-no-font = (puudub)
trans-browse = Sirvi…
trans-google-fonts = Google Fonts
trans-pick-country = Klõpsake, et valida riik
trans-font-outside = Font tuleb esmalt installida kausta assets/fonts.
trans-font-dir-missing = Kausta assets/fonts ei leitud.

# Translation submission
trans-lang-endonym = Keele endonüüm:
trans-author = Autor:
trans-submit = Saada…
trans-submit-hint = Koosta zip-fail ja ava eeltäidetud e-kiri
trans-data-updated = Viited andmed uuendatud (Languages.json / Countries.json)
trans-package-ready = Arhiiv valmis:
trans-package-error = Arhiivi ei õnnestunud koostada:

# ISO 639-3 requirement
trans-lang-not-iso = Tõlkimine on võimalik ainult keelte puhul, millel on ISO 639-3 kood.

# FOMOD installer preview
menu-preview = Paigaldaja eelvaade…
preview-title = FOMODi paigaldaja eelvaade
preview-refresh = Värskenda
preview-assumptions = Failide eeldused
preview-details = Detailid
preview-back = Tagasi
preview-next = Edasi
preview-install = Paigalda
preview-close = Sulge
preview-restart = Käivita uuesti
preview-summary-title = Paigaldatavad failid
preview-empty = Ühtegi faili ei paigaldata.
preview-none-option = (puudub)
preview-invalid = Jätkamiseks täitke nõutud valikud.
preview-no-steps = Ühtegi sammu ei ole näha; vaata paigaldamise kokkuvõtet.
preview-select-hint = Vali valik, et näha selle kirjeldust.
preview-col-source = Allikas
preview-col-dest = Sihtkoht
preview-col-priority = Prioriteet
preview-sel-exactlyone = Vali täpselt üks valik.
preview-sel-atmostone = Vali maksimaalselt üks valik.
preview-sel-any = Valige suvaline arv valikuid.
preview-sel-all = Kõik valikud on paigaldatud.
preview-sel-atleastone = Valige vähemalt üks valik.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Valideeri FOMOD
validate-report-title = FOMOD-valideerimine
validate-ok = Probleeme ei leitud. FOMOD vastab skeemile.
xml-editor-schema-ok = Vastab ModConfig 5.0 skeemile.
xml-editor-schema-issues = Skeemiprobleemid:
schema-line-col = Rida { $line }, veerg { $col }: { $msg }
schema-wrong-root = Ootamatu juur „{ $found }“ (ootati „{ $expected }“).
schema-unknown = Ootamatu element „{ $element }” elemendis „{ $parent }”.
schema-missing = „{ $parent }” peab sisaldama „{ $child }”.
schema-needs-one = „{ $parent }” peab sisaldama vähemalt ühte „{ $child }”.
schema-too-many = „{ $child }“ võib esineda „{ $parent }“-is ainult üks kord.
schema-missing-attr = Atribuut „{ $attr }“ on „{ $element }“-is kohustuslik.
schema-bad-enum = Kehtetu väärtus „{ $value }“ elemendile { $element }/@{ $attr } (ootus: { $allowed }).
schema-choose-one = „{ $parent }“ peab sisaldama täpselt ühte järgmistest: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Liiguta ettepoole
reorder-after = Liiguta tahapoole

# Country / language database explorer (Properties)
menu-properties = Omadused…
prop-title = Riigi / keele andmebaas
prop-tab-countries = Riigid
prop-tab-languages = Keeled
prop-filter = Filter:
prop-official-langs = Ametlikud keeled
prop-spoken-langs = Räägitavad keeled
prop-endonym = Riigi endonüüm
prop-font = Font
prop-spoken-in = Räägitakse
prop-select-country = Vali riik, et näha selle üksikasju.
prop-select-lang = Vali keel, et näha selle üksikasju.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Ava mängu Nexus Modsi lehekülg
