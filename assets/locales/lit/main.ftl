# XIMOD Architect - translation metadata
# @language = lit
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = lietuvių kalba
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versija { $version }

# Status messages
status-ready = Paruošta
msg-save-success = FOMOD sėkmingai išsaugotas
msg-save-error = Klaida išsaugant FOMOD
msg-export-success = Sukurtas platinimo archyvas ({ $count } failų): { $path }
msg-export-error = Klaida kuriant platinimo archyvą: { $error }
msg-load-success = FOMOD sėkmingai įkeltas
msg-load-error = Įkeliant FOMOD įvyko klaida
msg-merge-success = FOMOD sėkmingai sujungtas
msg-merge-error = Sujungiant FOMOD įvyko klaida
msg-no-root-selected = Pirmiausia pasirinkite šakninį katalogą
msg-no-fomod-folder = Nerastas „fomod“ aplankas. Sukurti?
msg-file-outside-root = Failas yra už šakninio katalogo ribų

# Menu - File
menu-file = Failas
menu-new = Naujas
menu-open = Atidaryti aplanką...
menu-open-file = Atidaryti failą...
menu-save = Išsaugoti
menu-recent = Neseniai naudoti
menu-exit = Išeiti
menu-merge = Sujungti FOMOD...
menu-export = Eksportuoti platinimo archyvą...

# Menu - Options
menu-options = Parinktys
menu-settings = Nustatymai
menu-pre-save-script = Skriptas prieš išsaugojimą...
menu-post-save-script = Skriptas po išsaugojimo...
menu-translation = Vertimas...

# Menu - Help
menu-help = Pagalba
menu-about = Apie

# Tabs
tab-info = Modifikacijos informacija
tab-steps = Įdiegimo žingsniai
tab-required = Būtini įdiegimai
tab-conditional = Sąlyginiai įdiegimai

# Info Tab
label-workspace = Darbo erdvė
label-root-dir = Pagrindinis katalogas:
label-mod-name = Modifikacijos pavadinimas:
label-author = Autorius:
label-version = Versija:
label-game-name = Žaidimo pavadinimas:
label-category = Kategorija:
label-url = Svetainės URL:
label-header-image = Antraštės paveikslėlis:
label-description = Aprašymas:
placeholder-select-dir = (Pasirinkite katalogą)
placeholder-select-game = (Pasirinkite žaidimą)

# Steps Tab
label-step-name = Žingsnio pavadinimas:
label-group-name = Grupės pavadinimas:
label-group-type = Grupės tipas:
label-plugin-name = Įskiepio pavadinimas:
label-plugin-desc = Aprašymas:
label-plugin-type = Numatytasis tipas:
label-plugin-image = Vaizdas:
label-visibility = Matomumo sąlygos
label-operator = Operatorius:

# Buttons
btn-browse = Naršyti...
btn-clear = Išvalyti
btn-add = Pridėti
btn-remove = Pašalinti
btn-add-step = Naujas žingsnis
btn-delete-step = Ištrinti žingsnį
btn-add-group = Pridėti grupę
btn-remove-group = Pašalinti grupę
btn-add-plugin = Pridėti įskiepį
btn-remove-plugin = Pašalinti įskiepį
btn-add-file = Pridėti failą
btn-add-folder = Pridėti aplanką
btn-remove-file = Pašalinti
btn-add-flag = Pridėti žymę
btn-remove-flag = Pašalinti žymę
btn-add-condition = Pridėti sąlygą
btn-remove-condition = Pašalinti sąlygą
btn-add-dependency = Pridėti priklausomybę
btn-remove-dependency = Pašalinti priklausomybę
btn-add-pattern = Naujas šablonas
btn-remove-pattern = Ištrinti šabloną
btn-save = Išsaugoti
btn-cancel = Atšaukti
btn-ok = Gerai
btn-yes = Taip
btn-no = Ne

# Condition/Dependency Labels
label-flag-name = Žymos pavadinimas:
label-flag-value = Vertė:
label-condition-type = Tipas:
label-condition-name = Pavadinimas:
label-condition-value = Vertė:
label-dep-type = Priklausomybės tipas:
label-dep-name = Pavadinimas/Failas:
label-dep-value = Vertė/Būklė:

# Files
label-source = Šaltinis
label-destination = Paskirties vieta
label-priority = Prioritetas
label-file-type = Tipas
label-files = Failai
label-dependencies = Priklausomybės

# Settings Dialog
settings-title = Nustatymai
settings-tab-general = Bendrieji
settings-tab-recent-files = Neseniai naudoti failai
settings-language = Kalba:
settings-theme = Tema:
settings-font-size = Šrifto dydis:
settings-replace-newlines = Aprašymuose apdoroti naujų eilučių simbolius
settings-max-recent = Maksimalus neseniai naudotų failų skaičius:
settings-window-width = Lango plotis:
settings-window-height = Lango aukštis:
settings-no-recent-files = Nėra neseniai naudotų failų.

# Status messages for settings
status-settings-saved = Nustatymai sėkmingai išsaugoti

# About Dialog
about-title = Apie „XIMOD Architect“
about-description = Daugiaplatformis „FOMOD“ diegimo programos kūrimo įrankis, skirtas „Bethesda“ žaidimų modams.
about-license = Licencijuota pagal MIT licenciją
about-copyright = © 2025–2026 „XIMOD“ komanda
about-credit = Originalaus įrankio Rust perkėlimas, atliktas Wenderer:

# Script Dialog
script-title = Redaguoti skriptą
script-info = Skriptai vykdomi prieš arba po išsaugojimo. Galite naudoti šiuos makrokomandas:
script-macros = Galimi makrokomandos:
macro-modname = $MODNAME$ – modifikacijos pavadinimas
macro-modauthor = $MODAUTHOR$ – autoriaus vardas
macro-modversion = $MODVERSION$ – modifikacijos versija
macro-modroot = $MODROOT$ – pagrindinio katalogo kelias
macro-date = $DATE$ – dabartinė data (MMMM-MM-DD)
macro-time = $TIME$ – Dabartinis laikas (HH:MM:SS)
macro-random = $RANDOM$ – Atsitiktinis skaičius

# Plugin Dependencies
label-default-type = Numatytasis tipas:
label-pattern-type = Šablono tipas:
label-pattern-operator = Šablono operatorius:

# Conditional Files
label-pattern = Šablonas

# Validation Messages
validation-no-name = Reikalingas modifikatoriaus pavadinimas
validation-no-steps = Reikalingas bent vienas žingsnis arba privalomas failas
validation-empty-step = Žingsnis { $num } neturi pavadinimo
validation-empty-group = Žingsnis { $step }, grupė { $group } neturi pavadinimo
validation-no-plugins = Žingsnis { $step }, grupė „{ $name }“ neturi papildinių

# File States
state-active = Aktyvi
state-inactive = Neaktyvi
state-missing = Trūksta

# Confirmation
confirm-title = Patvirtinimas
confirm-delete = Ar tikrai norite ištrinti šį elementą?
confirm-discard = Turite neišsaugotų pakeitimų. Atmesti juos ir tęsti?
confirm-unsaved = Turite neišsaugotų pakeitimų. Ar norite išsaugoti prieš uždarant?
confirm-save-issues = Projekte yra šios problemos:
confirm-save-anyway = Vis tiek išsaugoti?

# Errors
error-invalid-xml = Neteisingas XML failas
error-parse-failed = Nepavyko išanalizuoti FOMOD
error-write-failed = Nepavyko įrašyti failo
error-create-dir = Nepavyko sukurti katalogo

# Default names (generated when creating new items)
default-step-name = Žingsnis { $num }
default-group-name = Grupė { $num }
default-plugin-name = Įskiepis { $num }
pattern-label = Šablonas { $num }

# Selection prompts
msg-select-group-first = Pirmiausia pasirinkite grupę.
msg-select-plugin-edit = Pasirinkite papildinį, kurį norite redaguoti.
label-empty = (tuščia)
image-no-image = Nėra paveikslėlio

# File dialog filters
filter-images = Vaizdai
filter-xml = XML

# Dependency types
dep-type-flag = Vėliava
dep-type-file = Failas

# Status bar
status-modified = Pakeista

# Status messages (errors)
msg-settings-save-error = Klaida išsaugant nustatymus
msg-script-save-error = Klaida išsaugant scenarijų

# Translation editor
trans-title = Vertimų redaktorius
trans-source-lang = Rodoma kalba:
trans-target-lang = Vertimo kalba:
trans-col-key = Raktinis žodis
trans-col-source = Etiketė
trans-col-target = Vertimas
trans-saved = Vertimas išsaugotas
trans-save-error = Klaida išsaugant vertimą

# XML editor
xml-editor-title = XML redaktorius
xml-editor-edit = Redaguoti
xml-editor-apply = Taikyti
xml-editor-revert = Atšaukti
xml-editor-readonly = Tik skaityti
xml-editor-editing = Redaguojama — grafinės kortelės užrakintos
xml-editor-error = Klaida:
xml-editor-applied = XML pakeitimai pritaikyti
xml-editor-wellformed = Teisingai suformuotas XML
xml-editor-error-at = Eilutė { $line }, stulpelis { $col }: { $msg }

# Country / flag picker
settings-country-name = Šalies pavadinimas:
settings-pick-country = Spustelėkite, kad pasirinkite savo šalį
flags-title = Pasirinkite šalį
flags-filter = Filtras:
flags-none = Vėliava nerasta

# Translation editor: country & font
trans-endonym = Šalies endonimas:
trans-font = Šriftas:
trans-no-font = (nėra)
trans-browse = Naršyti…
trans-google-fonts = „Google Fonts“
trans-pick-country = Spustelėkite, kad pasirinkite šalį
trans-font-outside = Šriftas pirmiausia turi būti įdiegtas į „assets/fonts“ aplanką.
trans-font-dir-missing = Aplankas „assets/fonts“ nerastas.

# Translation submission
trans-lang-endonym = Kalbos pavadinimas:
trans-author = Autorius:
trans-submit = Siųsti…
trans-submit-hint = Sukurkite ZIP failą ir atidarykite iš anksto užpildytą el. laišką
trans-data-updated = Atnaujinti nuorodų duomenys (Languages.json / Countries.json)
trans-package-ready = Archyvas paruoštas:
trans-package-error = Nepavyko sukurti archyvo:

# ISO 639-3 requirement
trans-lang-not-iso = Vertimas galimas tik kalbai, turinčiai ISO 639-3 kodą.

# FOMOD installer preview
menu-preview = Peržiūrėti diegimo programą…
preview-title = FOMOD diegimo programos peržiūra
preview-refresh = Atnaujinti
preview-assumptions = Failų prielaidos
preview-details = Išsamiau
preview-back = Atgal
preview-next = Toliau
preview-install = Įdiegti
preview-close = Uždaryti
preview-restart = Paleisti iš naujo
preview-summary-title = Failai, kurie bus įdiegti
preview-empty = Nebus įdiegtas joks failas.
preview-none-option = (nėra)
preview-invalid = Norėdami tęsti, užpildykite privalomus laukelius.
preview-no-steps = Nėra matomų žingsnių; žr. diegimo santrauką.
preview-select-hint = Pasirinkite parinktį, kad pamatytumėte jos aprašymą.
preview-col-source = Šaltinis
preview-col-dest = Paskirties vieta
preview-col-priority = Prioritetas
preview-sel-exactlyone = Pasirinkite tiksliai vieną parinktį.
preview-sel-atmostone = Pasirinkite ne daugiau kaip vieną parinktį.
preview-sel-any = Pasirinkite bet kokį parinkčių skaičių.
preview-sel-all = Įdiegiamos visos parinktys.
preview-sel-atleastone = Pasirinkite bent vieną parinktį.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Patikrinti FOMOD
validate-report-title = FOMOD tikrinimas
validate-ok = Problemų nerasta. FOMOD atitinka schemą.
xml-editor-schema-ok = Atitinka ModConfig 5.0 schemą.
xml-editor-schema-issues = Schemos problemos:
schema-line-col = Eilutė { $line }, stulpelis { $col }: { $msg }
schema-wrong-root = Netikėtas šaknis „{ $found }“ (lauktas „{ $expected }“).
schema-unknown = Netikėtas elementas „{ $element }“ elemente „{ $parent }“.
schema-missing = „{ $parent }“ turi turėti „{ $child }“.
schema-needs-one = „{ $parent }“ turi turėti bent vieną „{ $child }“.
schema-too-many = „{ $child }“ gali pasirodyti tik vieną kartą „{ $parent }“.
schema-missing-attr = Atributas „{ $attr }“ yra privalomas elemente „{ $element }“.
schema-bad-enum = Netinkama reikšmė „{ $value }“ elementui { $element }/@{ $attr } (laukiama: { $allowed }).
schema-choose-one = „{ $parent }“ turi turėti būtent vieną iš: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Perkelti prieš
reorder-after = Perkelti po

# Country / language database explorer (Properties)
menu-properties = Savybės…
prop-title = Šalių / kalbų duomenų bazė
prop-tab-countries = Šalys
prop-tab-languages = Kalbos
prop-filter = Filtras:
prop-official-langs = Oficialios kalbos
prop-spoken-langs = Kalbos, kuriomis kalbama
prop-endonym = Šalies endonimas
prop-font = Šriftas
prop-spoken-in = Kalbama
prop-select-country = Pasirinkite šalį, kad pamatytumėte jos informaciją.
prop-select-lang = Pasirinkite kalbą, kad pamatytumėte jos informaciją.

# Direct link to Nexus Mods (game slug)
btn-nexus = „Nexus“ ↗
nexus-open-hint = Atidaryti žaidimo „Nexus Mods“ puslapį
