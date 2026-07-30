# XIMOD Architect - translation metadata
# @language = ces
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Čeština
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Verze { $version }

# Status messages
status-ready = Připraveno
msg-save-success = FOMOD byl úspěšně uložen
msg-save-error = Chyba při ukládání FOMOD
msg-export-success = Distribuční archiv byl vytvořen ({ $count } souborů): { $path }
msg-export-error = Chyba při vytváření distribučního archivu: { $error }
msg-load-success = FOMOD byl úspěšně načten
msg-load-error = Chyba při načítání FOMOD
msg-merge-success = FOMOD byl úspěšně sloučen
msg-merge-error = Chyba při slučování FOMOD
msg-no-root-selected = Nejprve vyberte kořenový adresář
msg-no-fomod-folder = Složka „fomod“ nenalezena. Vytvořit ji?
msg-file-outside-root = Soubor je mimo kořenový adresář

# Menu - File
menu-file = Soubor
menu-new = Nový
menu-open = Otevřít složku…
menu-open-file = Otevřít soubor…
menu-save = Uložit
menu-recent = Nedávné
menu-exit = Ukončit
menu-merge = Sloučit FOMOD…
menu-export = Exportovat distribuční archiv…

# Menu - Options
menu-options = Možnosti
menu-settings = Nastavení
menu-pre-save-script = Skript před uložením…
menu-post-save-script = Skript po uložení…
menu-translation = Překlad…

# Menu - Help
menu-help = Nápověda
menu-about = O aplikaci

# Tabs
tab-info = Informace o modu
tab-steps = Kroky instalace
tab-required = Povinné instalace
tab-conditional = Podmíněné instalace

# Info Tab
label-workspace = Pracovní prostor
label-root-dir = Kořenový adresář:
label-mod-name = Název modu:
label-author = Autor:
label-version = Verze:
label-game-name = Název hry:
label-category = Kategorie:
label-url = Adresa URL webu:
label-header-image = Záhlaví (obrázek):
label-description = Popis:
placeholder-select-dir = (Vyberte adresář)
placeholder-select-game = (Vyberte hru)

# Steps Tab
label-step-name = Název kroku:
label-group-name = Název skupiny:
label-group-type = Typ skupiny:
label-plugin-name = Název pluginu:
label-plugin-desc = Popis:
label-plugin-type = Výchozí typ:
label-plugin-image = Obrázek:
label-visibility = Podmínky viditelnosti
label-operator = Operátor:

# Buttons
btn-browse = Procházet…
btn-clear = Vymazat
btn-add = Přidat
btn-remove = Odebrat
btn-add-step = Nový krok
btn-delete-step = Smazat krok
btn-add-group = Přidat skupinu
btn-remove-group = Odebrat skupinu
btn-add-plugin = Přidat plugin
btn-remove-plugin = Odebrat plugin
btn-add-file = Přidat soubor
btn-add-folder = Přidat složku
btn-remove-file = Odebrat
btn-add-flag = Přidat příznak
btn-remove-flag = Odebrat příznak
btn-add-condition = Přidat podmínku
btn-remove-condition = Odebrat podmínku
btn-add-dependency = Přidat závislost
btn-remove-dependency = Odebrat závislost
btn-add-pattern = Nový vzor
btn-remove-pattern = Smazat vzor
btn-save = Uložit
btn-cancel = Zrušit
btn-ok = OK
btn-yes = Ano
btn-no = Ne

# Condition/Dependency Labels
label-flag-name = Název příznaku:
label-flag-value = Hodnota:
label-condition-type = Typ:
label-condition-name = Název:
label-condition-value = Hodnota:
label-dep-type = Typ závislosti:
label-dep-name = Název/soubor:
label-dep-value = Hodnota/stav:

# Files
label-source = Zdroj
label-destination = Cíl
label-priority = Priorita
label-file-type = Typ
label-files = Soubory
label-dependencies = Závislosti

# Settings Dialog
settings-title = Nastavení
settings-tab-general = Obecné
settings-tab-recent-files = Nedávné soubory
settings-language = Jazyk:
settings-theme = Motiv:
settings-font-size = Velikost písma:
settings-replace-newlines = Zpracovat konce řádků v popisech
settings-max-recent = Max. nedávných souborů:
settings-window-width = Šířka okna:
settings-window-height = Výška okna:
settings-no-recent-files = Žádné nedávné soubory.

# Status messages for settings
status-settings-saved = Nastavení bylo úspěšně uloženo

# About Dialog
about-title = O aplikaci XIMOD Architect
about-description = Multiplatformní nástroj pro tvorbu instalátorů FOMOD pro mody her Bethesda.
about-license = Licencováno pod licencí MIT
about-copyright = © 2024 XIMOD Team
about-credit = Port původního nástroje od Wenderer ve verzi Rust:

# Script Dialog
script-title = Upravit skript
script-info = Skripty se spouštějí před uložením nebo po něm. Můžete použít následující makra:
script-macros = Dostupná makra:
macro-modname = $MODNAME$ - Název modu
macro-modauthor = $MODAUTHOR$ - Jméno autora
macro-modversion = $MODVERSION$ - Verze modu
macro-modroot = $MODROOT$ - Cesta ke kořenovému adresáři
macro-date = $DATE$ - Aktuální datum (RRRR-MM-DD)
macro-time = $TIME$ - Aktuální čas (HH:MM:SS)
macro-random = $RANDOM$ - Náhodné číslo

# Plugin Dependencies
label-default-type = Výchozí typ:
label-pattern-type = Typ vzoru:
label-pattern-operator = Operátor vzoru:

# Conditional Files
label-pattern = Vzor

# Validation Messages
validation-no-name = Název modu je povinný
validation-no-steps = Je potřeba alespoň jeden krok nebo povinný soubor
validation-empty-step = Krok { $num } nemá název
validation-empty-group = Krok { $step }, skupina { $group } nemá název
validation-no-plugins = Krok { $step }, skupina „{ $name }“ nemá žádné pluginy

# File States
state-active = Aktivní
state-inactive = Neaktivní
state-missing = Chybí

# Confirmation
confirm-title = Potvrzení
confirm-delete = Opravdu chcete tuto položku smazat?
confirm-discard = Máte neuložené změny. Zahodit je a pokračovat?
confirm-unsaved = Máte neuložené změny. Chcete je před zavřením uložit?
confirm-save-issues = Projekt má následující problémy:
confirm-save-anyway = Přesto uložit?

# Errors
error-invalid-xml = Neplatný soubor XML
error-parse-failed = Nepodařilo se zpracovat FOMOD
error-write-failed = Nepodařilo se zapsat soubor
error-create-dir = Nepodařilo se vytvořit adresář

# Default names (generated when creating new items)
default-step-name = Krok { $num }
default-group-name = Skupina { $num }
default-plugin-name = Plugin { $num }
pattern-label = Vzor { $num }

# Selection prompts
msg-select-group-first = Nejprve vyberte skupinu.
msg-select-plugin-edit = Vyberte plugin k úpravě.
label-empty = (prázdné)
image-no-image = Žádný obrázek

# File dialog filters
filter-images = Obrázky
filter-xml = XML

# Dependency types
dep-type-flag = Příznak
dep-type-file = Soubor

# Status bar
status-modified = Změněno

# Status messages (errors)
msg-settings-save-error = Chyba při ukládání nastavení
msg-script-save-error = Chyba při ukládání skriptu

# Translation editor
trans-title = Editor překladů
trans-source-lang = Zobrazený jazyk:
trans-target-lang = Jazyk k překladu:
trans-col-key = Klíč
trans-col-source = Popisek
trans-col-target = Překlad
trans-saved = Překlad byl uložen
trans-save-error = Chyba při ukládání překladu

# XML editor
xml-editor-title = Editor XML
xml-editor-edit = Upravit
xml-editor-apply = Použít
xml-editor-revert = Zrušit
xml-editor-readonly = Jen pro čtení
xml-editor-editing = Úpravy — grafické karty jsou uzamčeny
xml-editor-error = Chyba:
xml-editor-applied = Změny XML byly použity
xml-editor-wellformed = Správně strukturované XML
xml-editor-error-at = Řádek { $line }, sloupec { $col }: { $msg }

# Country / flag picker
settings-country-name = Název země:
settings-pick-country = Klikněte pro výběr své země
flags-title = Vyberte zemi
flags-filter = Filtr:
flags-none = Žádná vlajka nenalezena

# Translation editor: country & font
trans-endonym = Endonym země:
trans-font = Písmo:
trans-no-font = (žádné)
trans-browse = Procházet…
trans-google-fonts = Google Fonts
trans-pick-country = Klikněte pro výběr země
trans-font-outside = Písmo musí být nejprve nainstalováno do assets/fonts.
trans-font-dir-missing = Složka assets/fonts nebyla nalezena.

# Translation submission
trans-lang-endonym = Endonym jazyka:
trans-author = Autor:
trans-submit = Odeslat…
trans-submit-hint = Vytvořit zip a otevřít předvyplněný e-mail
trans-data-updated = Referenční data byla aktualizována (Languages.json / Countries.json)
trans-package-ready = Archiv připraven:
trans-package-error = Archiv se nepodařilo vytvořit:

# ISO 639-3 requirement
trans-lang-not-iso = Překlad je možný pouze pro jazyk s kódem ISO 639-3.

# FOMOD installer preview
menu-preview = Náhled instalátoru…
preview-title = Náhled instalátoru FOMOD
preview-refresh = Obnovit
preview-assumptions = Předpoklady o souborech
preview-details = Podrobnosti
preview-back = Zpět
preview-next = Další
preview-install = Instalovat
preview-close = Zavřít
preview-restart = Restartovat
preview-summary-title = Soubory, které budou nainstalovány
preview-empty = Žádný soubor by nebyl nainstalován.
preview-none-option = (žádné)
preview-invalid = Pro pokračování dokončete povinné volby.
preview-no-steps = Není viditelný žádný krok; viz souhrn instalace.
preview-select-hint = Vyberte možnost pro zobrazení jejího popisu.
preview-col-source = Zdroj
preview-col-dest = Cíl
preview-col-priority = Priorita
preview-sel-exactlyone = Vyberte přesně jednu možnost.
preview-sel-atmostone = Vyberte nejvýše jednu možnost.
preview-sel-any = Vyberte libovolný počet možností.
preview-sel-all = Všechny možnosti jsou nainstalovány.
preview-sel-atleastone = Vyberte alespoň jednu možnost.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Ověřit FOMOD
validate-report-title = Ověření FOMOD
validate-ok = Nebyl nalezen žádný problém. FOMOD odpovídá schématu.
xml-editor-schema-ok = Odpovídá schématu ModConfig 5.0.
xml-editor-schema-issues = Problémy se schématem:
schema-line-col = Řádek { $line }, sl. { $col }: { $msg }
schema-wrong-root = Neočekávaný kořen „{ $found }“ (očekáváno „{ $expected }“).
schema-unknown = Neočekávaný prvek „{ $element }“ v „{ $parent }“.
schema-missing = „{ $parent }“ musí obsahovat „{ $child }“.
schema-needs-one = „{ $parent }“ musí obsahovat alespoň jeden „{ $child }“.
schema-too-many = „{ $child }“ se smí v „{ $parent }“ vyskytovat pouze jednou.
schema-missing-attr = Atribut „{ $attr }“ je u „{ $element }“ povinný.
schema-bad-enum = Neplatná hodnota „{ $value }“ pro { $element }/@{ $attr } (očekáváno: { $allowed }).
schema-choose-one = „{ $parent }“ musí obsahovat přesně jeden z: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Přesunout před
reorder-after = Přesunout za

# Country / language database explorer (Properties)
menu-properties = Vlastnosti…
prop-title = Databáze zemí / jazyků
prop-tab-countries = Země
prop-tab-languages = Jazyky
prop-filter = Filtr:
prop-official-langs = Úřední jazyky
prop-spoken-langs = Používané jazyky
prop-endonym = Endonym země
prop-font = Písmo
prop-spoken-in = Používá se v
prop-select-country = Vyberte zemi pro zobrazení jejích podrobností.
prop-select-lang = Vyberte jazyk pro zobrazení jeho podrobností.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Otevřít stránku hry na Nexus Mods
