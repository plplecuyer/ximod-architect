# XIMOD Architect - translation metadata
# @language = slk
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Slovenčina
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Verzia { $version }

# Status messages
status-ready = Pripravené
msg-save-success = FOMOD bol úspešne uložený
msg-save-error = Chyba pri ukladaní FOMOD
msg-export-success = Distribučný archív vytvorený ({ $count } súborov): { $path }
msg-export-error = Chyba pri vytváraní distribučného archívu: { $error }
msg-load-success = FOMOD bol úspešne načítaný
msg-load-error = Chyba pri načítaní FOMOD
msg-merge-success = FOMOD úspešne zlúčený
msg-merge-error = Chyba pri zlučovaní FOMOD
msg-no-root-selected = Najprv vyberte koreňový adresár
msg-no-fomod-folder = Priečinok „fomod“ sa nenašiel. Vytvoriť ho?
msg-file-outside-root = Súbor je mimo koreňového adresára

# Menu - File
menu-file = Súbor
menu-new = Nový
menu-open = Otvoriť priečinok…
menu-open-file = Otvoriť súbor…
menu-save = Uložiť
menu-recent = Nedávne
menu-exit = Ukončiť
menu-merge = Zlúčiť FOMOD…
menu-export = Exportovať distribučný archív...

# Menu - Options
menu-options = Možnosti
menu-settings = Nastavenia
menu-pre-save-script = Skript pred uložením…
menu-post-save-script = Skript po uložení…
menu-translation = Preklad...

# Menu - Help
menu-help = Pomocník
menu-about = O aplikácii

# Tabs
tab-info = Informácie o mode
tab-steps = Kroky inštalácie
tab-required = Povinné inštalácie
tab-conditional = Podmienené inštalácie

# Info Tab
label-workspace = Pracovný priestor
label-root-dir = Koreňový adresár:
label-mod-name = Názov modu:
label-author = Autor:
label-version = Verzia:
label-game-name = Názov hry:
label-category = Kategória:
label-url = URL adresa webu:
label-header-image = Obrázok hlavičky:
label-description = Popis:
placeholder-select-dir = (Vyberte adresár)
placeholder-select-game = (Vyberte hru)

# Steps Tab
label-step-name = Názov kroku:
label-group-name = Názov skupiny:
label-group-type = Typ skupiny:
label-plugin-name = Názov pluginu:
label-plugin-desc = Popis:
label-plugin-type = Predvolený typ:
label-plugin-image = Obrázok:
label-visibility = Podmienky viditeľnosti
label-operator = Operátor:

# Buttons
btn-browse = Prehľadávať…
btn-clear = Vymazať
btn-add = Pridať
btn-remove = Odstrániť
btn-add-step = Nový krok
btn-delete-step = Odstrániť krok
btn-add-group = Pridať skupinu
btn-remove-group = Odstrániť skupinu
btn-add-plugin = Pridať plugin
btn-remove-plugin = Odstrániť plugin
btn-add-file = Pridať súbor
btn-add-folder = Pridať priečinok
btn-remove-file = Odstrániť
btn-add-flag = Pridať príznak
btn-remove-flag = Odstrániť príznak
btn-add-condition = Pridať podmienku
btn-remove-condition = Odstrániť podmienku
btn-add-dependency = Pridať závislosť
btn-remove-dependency = Odstrániť závislosť
btn-add-pattern = Nový vzor
btn-remove-pattern = Odstrániť vzor
btn-save = Uložiť
btn-cancel = Zrušiť
btn-ok = OK
btn-yes = Áno
btn-no = Nie

# Condition/Dependency Labels
label-flag-name = Názov príznaku:
label-flag-value = Hodnota:
label-condition-type = Typ:
label-condition-name = Názov:
label-condition-value = Hodnota:
label-dep-type = Typ závislosti:
label-dep-name = Názov/súbor:
label-dep-value = Hodnota/stav:

# Files
label-source = Zdroj
label-destination = Cieľ
label-priority = Priorita
label-file-type = Typ
label-files = Súbory
label-dependencies = Závislosti

# Settings Dialog
settings-title = Nastavenia
settings-tab-general = Všeobecné
settings-tab-recent-files = Nedávne súbory
settings-language = Jazyk:
settings-theme = Motív:
settings-font-size = Veľkosť písma:
settings-replace-newlines = Spracovať konce riadkov v popisoch
settings-max-recent = Max. nedávnych súborov:
settings-window-width = Šírka okna:
settings-window-height = Výška okna:
settings-no-recent-files = Žiadne nedávne súbory.

# Status messages for settings
status-settings-saved = Nastavenia boli úspešne uložené

# About Dialog
about-title = O aplikácii XIMOD Architect
about-description = Multiplatformový nástroj na tvorbu inštalátorov FOMOD pre módy hier Bethesda.
about-license = Licencované pod licenciou MIT
about-copyright = © 2024 XIMOD Team
about-credit = Port pôvodného nástroja od Wenderera na Rust:

# Script Dialog
script-title = Upraviť skript
script-info = Skripty sa spúšťajú pred uložením alebo po ňom. Môžete použiť nasledujúce makrá:
script-macros = Dostupné makrá:
macro-modname = $MODNAME$ - Názov modu
macro-modauthor = $MODAUTHOR$ - Meno autora
macro-modversion = $MODVERSION$ - Verzia modu
macro-modroot = $MODROOT$ - Cesta ku koreňovému adresáru
macro-date = $DATE$ - Aktuálny dátum (RRRR-MM-DD)
macro-time = $TIME$ - Aktuálny čas (HH:MM:SS)
macro-random = $RANDOM$ - Náhodné číslo

# Plugin Dependencies
label-default-type = Predvolený typ:
label-pattern-type = Typ vzoru:
label-pattern-operator = Operátor vzoru:

# Conditional Files
label-pattern = Vzor

# Validation Messages
validation-no-name = Názov modu je povinný
validation-no-steps = Vyžaduje sa aspoň jeden krok alebo povinný súbor
validation-empty-step = Krok { $num } nemá názov
validation-empty-group = Krok { $step }, skupina { $group } nemá názov
validation-no-plugins = Krok { $step }, skupina „{ $name }“ nemá žiadne pluginy

# File States
state-active = Aktívny
state-inactive = Neaktívny
state-missing = Chýba

# Confirmation
confirm-title = Potvrdenie
confirm-delete = Naozaj chcete odstrániť túto položku?
confirm-discard = Máte neuložené zmeny. Zahodiť ich a pokračovať?
confirm-unsaved = Máte neuložené zmeny. Chcete ich pred zatvorením uložiť?
confirm-save-issues = Projekt má tieto problémy:
confirm-save-anyway = Uložiť napriek tomu?

# Errors
error-invalid-xml = Neplatný súbor XML
error-parse-failed = Nepodarilo sa spracovať FOMOD
error-write-failed = Nepodarilo sa zapísať súbor
error-create-dir = Nepodarilo sa vytvoriť adresár

# Default names (generated when creating new items)
default-step-name = Krok { $num }
default-group-name = Skupina { $num }
default-plugin-name = Plugin { $num }
pattern-label = Vzor { $num }

# Selection prompts
msg-select-group-first = Najprv vyberte skupinu.
msg-select-plugin-edit = Vyberte plugin na úpravu.
label-empty = (prázdne)
image-no-image = Žiadny obrázok

# File dialog filters
filter-images = Obrázky
filter-xml = XML

# Dependency types
dep-type-flag = Príznak
dep-type-file = Súbor

# Status bar
status-modified = Zmenené

# Status messages (errors)
msg-settings-save-error = Chyba pri ukladaní nastavení
msg-script-save-error = Chyba pri ukladaní skriptu

# Translation editor
trans-title = Editor prekladov
trans-source-lang = Zobrazený jazyk:
trans-target-lang = Jazyk na preloženie:
trans-col-key = Kľúč
trans-col-source = Popis
trans-col-target = Preklad
trans-saved = Preklad uložený
trans-save-error = Chyba pri ukladaní prekladu

# XML editor
xml-editor-title = XML Editor
xml-editor-edit = Upraviť
xml-editor-apply = Použiť
xml-editor-revert = Zrušiť
xml-editor-readonly = Iba na čítanie
xml-editor-editing = Úpravy — grafické karty sú uzamknuté
xml-editor-error = Chyba:
xml-editor-applied = Zmeny XML použité
xml-editor-wellformed = Správne štruktúrované XML
xml-editor-error-at = Riadok { $line }, stĺpec { $col }: { $msg }

# Country / flag picker
settings-country-name = Názov krajiny:
settings-pick-country = Kliknutím vyberte svoju krajinu
flags-title = Vyberte krajinu
flags-filter = Filter:
flags-none = Nenašla sa žiadna vlajka

# Translation editor: country & font
trans-endonym = Endoným krajiny:
trans-font = Písmo:
trans-no-font = (žiadne)
trans-browse = Prehľadávať…
trans-google-fonts = Google Fonts
trans-pick-country = Kliknutím vyberte krajinu
trans-font-outside = Písmo musí byť najprv nainštalované v assets/fonts.
trans-font-dir-missing = Priečinok assets/fonts sa nenašiel.

# Translation submission
trans-lang-endonym = Endoným jazyka:
trans-author = Autor:
trans-submit = Odoslať…
trans-submit-hint = Vytvorí zip a otvorí predvyplnený e-mail
trans-data-updated = Referenčné údaje aktualizované (Languages.json / Countries.json)
trans-package-ready = Archív pripravený:
trans-package-error = Archív sa nepodarilo vytvoriť:

# ISO 639-3 requirement
trans-lang-not-iso = Preklad je možný len pre jazyk s kódom ISO 639-3.

# FOMOD installer preview
menu-preview = Náhľad inštalátora…
preview-title = Náhľad inštalátora FOMOD
preview-refresh = Obnoviť
preview-assumptions = Predpoklady súborov
preview-details = Podrobnosti
preview-back = Späť
preview-next = Ďalej
preview-install = Inštalovať
preview-close = Zavrieť
preview-restart = Reštartovať
preview-summary-title = Súbory, ktoré budú nainštalované
preview-empty = Nenainštaloval by sa žiadny súbor.
preview-none-option = (žiadne)
preview-invalid = Na pokračovanie dokončite požadované voľby.
preview-no-steps = Nie je viditeľný žiadny krok; pozrite si súhrn inštalácie.
preview-select-hint = Vyberte možnosť, aby ste videli jej popis.
preview-col-source = Zdroj
preview-col-dest = Cieľ
preview-col-priority = Priorita
preview-sel-exactlyone = Vyberte presne jednu možnosť.
preview-sel-atmostone = Vyberte najviac jednu možnosť.
preview-sel-any = Vyberte ľubovoľný počet možností.
preview-sel-all = Nainštalované sú všetky možnosti.
preview-sel-atleastone = Vyberte aspoň jednu možnosť.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Overiť FOMOD
validate-report-title = Overenie FOMOD
validate-ok = Nenašiel sa žiadny problém. FOMOD zodpovedá schéme.
xml-editor-schema-ok = Zodpovedá schéme ModConfig 5.0.
xml-editor-schema-issues = Problémy so schémou:
schema-line-col = Riadok { $line }, stĺp. { $col }: { $msg }
schema-wrong-root = Neočakávaný koreň "{ $found }" (očakávalo sa "{ $expected }").
schema-unknown = Neočakávaný prvok "{ $element }" v "{ $parent }".
schema-missing = "{ $parent }" musí obsahovať "{ $child }".
schema-needs-one = "{ $parent }" musí obsahovať aspoň jeden "{ $child }".
schema-too-many = "{ $child }" sa môže vyskytnúť iba raz v "{ $parent }".
schema-missing-attr = Atribút "{ $attr }" je povinný pre "{ $element }".
schema-bad-enum = Neplatná hodnota "{ $value }" pre { $element }/@{ $attr } (očakávalo sa: { $allowed }).
schema-choose-one = "{ $parent }" musí obsahovať presne jedno z: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Presunúť pred
reorder-after = Presunúť za

# Country / language database explorer (Properties)
menu-properties = Vlastnosti…
prop-title = Databáza krajín / jazykov
prop-tab-countries = Krajiny
prop-tab-languages = Jazyky
prop-filter = Filter:
prop-official-langs = Úradné jazyky
prop-spoken-langs = Používané jazyky
prop-endonym = Endoným krajiny
prop-font = Písmo
prop-spoken-in = Používa sa v
prop-select-country = Vyberte krajinu, aby ste videli jej podrobnosti.
prop-select-lang = Vyberte jazyk, aby ste videli jeho podrobnosti.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Otvoriť stránku hry na Nexus Mods
