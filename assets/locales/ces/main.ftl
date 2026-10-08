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
menu-settings = Nastavení…
menu-pre-save-script = Skript před uložením…
menu-post-save-script = Skript po uložení…
menu-translation = Přeložit rozhraní…
# Menu - Help
menu-help = Nápověda
menu-check-updates = Zkontrolovat aktualizace…
menu-about = O aplikaci

# Update check
update-checking = Kontrola aktualizací…
update-up-to-date = XIMOD Architect je aktuální.
update-check-failed = Nelze zkontrolovat aktualizace. Zkuste to později.
update-available-status = Je k dispozici verze { $version }.
update-banner-text = XIMOD Architect { $version } je k dispozici.
update-download = Stáhnout:
update-skip = Přeskočit tuto verzi
update-later = Později

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
label-plugin-name = Název možnosti:
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
btn-add-plugin = Přidat možnost
btn-remove-plugin = Odebrat možnost
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Cíl pro celou skupinu
label-page-dest = Cíl instalace (celá stránka)
btn-apply-group-dest = Použít na všechny možnosti v této skupině
btn-apply-page-dest = Použít na všechny možnosti na této stránce
group-dest-hint = Nastaví jeden cíl instalace pro každý soubor každé možnosti v této skupině.
page-dest-hint = Nastaví jeden cíl instalace pro každý soubor každé možnosti na této stránce (všechny skupiny).
bulk-dest-nofiles = Zatím žádné soubory k aktualizaci — nejprve přidejte soubory do možností.
status-dest-applied = Cíl použit na { $num } soubor(ů).
preview-hidden-steps = { $num } krok(ů) skryto aktuálním výběrem.
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
settings-check-updates = Kontrolovat aktualizace při spuštění
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
label-plugin-dependencies = Závislosti možnosti
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
validation-no-plugins = Krok { $step }, skupina „{ $name }“ nemá žádné možnosti

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
default-plugin-name = Možnost { $num }
pattern-label = Vzor { $num }

# Selection prompts
msg-select-group-first = Nejprve vyberte skupinu.
msg-select-plugin-edit = Vyberte možnost k úpravě.
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

# Referenced-file verification (V2)
verify-no-root = Kontrola souborů přeskočena: není nastavena kořenová složka
loc-header = záhlaví (obrázek)
loc-required = povinné soubory
loc-conditional = podmíněná sada { $num }
loc-plugin = krok { $step }, skupina { $group }, možnost „{ $plugin }“
verify-missing-file = Chybějící soubor: { $path } ({ $loc })
verify-missing-folder = Chybějící složka: { $path } ({ $loc })
verify-missing-image = Chybějící obrázek: { $path } ({ $loc })
verify-absolute = Absolutní cesta (nepřenositelná): { $path } ({ $loc })
verify-outside = Cesta opouští kořenovou složku: { $path } ({ $loc })
verify-orphan = Osiřelý soubor (neodkazuje na něj žádná možnost): { $path }
conflict-certain = Konflikt cíle: „{ $path }“ zapisuje { $count } možností ({ $locs }) — vzájemně se přepisují.
conflict-potential = Možný konflikt cíle: „{ $path }“ je cílem { $count } odkazů ({ $locs }) — přepsání závisí na výběru/podmínkách.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Zavřít FOMOD
menu-close-all-fomods = Zavřít všechny FOMODy
tab-untitled = (bez názvu)
msg-drop-not-fomod = Přetažená položka není FOMOD (složka „fomod“ nenalezena)
exit-title = Neuložené změny
exit-unsaved = FOMOD nebyl uložen. Chcete jej uložit?
tab-close-hint = Zavřít tento FOMOD
menu-new-from-folder = Nový ze složky…
menu-templates = Šablony…
templates-title = Znovupoužitelné šablony
templates-empty = Zatím nejsou uloženy žádné šablony. Uložte výše vybraný krok a vytvořte šablonu.
templates-insert = Vložit
templates-save-step = Uložit vybraný krok
templates-name-hint = Název šablony (volitelné)
msg-wizard-success = Kostra vytvořena ze složky: { $num } možnost(í).
msg-wizard-error = Chyba: { $error }
msg-template-saved = Šablona uložena: { $name }
msg-template-inserted = Šablona vložena do projektu.
msg-template-no-step = Nejprve vyberte krok, abyste jej uložili jako šablonu.
msg-template-no-dir = Nelze najít složku se šablonami.
msg-drop-assigned = K možnosti přidáno { $added } zdroj(ů) ({ $rejected } mimo kořen ignorováno).
menu-compare = Porovnat s…
compare-title = Porovnání FOMOD
compare-none = Žádné rozdíly.
btn-optimize-image = Optimalizovat obrázek
msg-image-optimized = Obrázek záhlaví optimalizován.
msg-image-ok = Obrázek záhlaví je již v mezích.
msg-no-header-image = Žádný obrázek záhlaví k optimalizaci.
verify-image-large = Obrázek je příliš velký ({ $width }×{ $height }): { $path }
verify-image-format = Nepodporovaný formát obrázku (.{ $ext }): { $path }
verify-image-unreadable = Nečitelný obrázek: { $path }
menu-condition-editor = Editor podmínek…
condeditor-title = Editor podmínek
condeditor-set-by = Nastaveno:
condeditor-used-by = Použito:
condeditor-filedeps = Závislosti souborů
condeditor-empty = V tomto projektu nejsou žádné příznaky ani závislosti.
condeditor-orphan-set = nastaveno, ale nikdy nepoužito
condeditor-orphan-used = použito, ale nikdy nenastaveno
msg-img-optimized = Obrázek optimalizován.
msg-img-ok = Obrázek je již v mezích.
msg-img-none = Žádný obrázek k optimalizaci.
msg-crash-recovery = Předchozí relace skončila neočekávaně. Záloha vašeho projektu byla uložena do { $path }
export-progress-title = Vytváření distribučního archivu…
export-progress-files = { $done } / { $total } souborů
msg-export-cancelled = Export zrušen; částečný archiv byl odstraněn.
verify-running = Kontrola souborů na disku…
verify-stale = Poznámka: projekt se během kontroly souborů změnil; spusťte ověření znovu.
prop-col-name = Název
menu-save-as = Uložit jako…
menu-project = Projekt
menu-tools = Nástroje
menu-manual = Uživatelská příručka
msg-manual-missing = Uživatelská příručka (PDF) nebyla nalezena vedle aplikace.
toolbar-new = Nový
toolbar-open = Otevřít
toolbar-save = Uložit
toolbar-validate = Ověřit
toolbar-preview = Náhled
toolbar-export = Exportovat
dialog-choose-root = Vyberte kořenovou složku modu
exit-unsaved-docs = Neuloženo: { $names }
status-summary = { $steps } kroků · { $options } možností
section-groups = Skupiny
section-options = Možnosti
section-flags = Příznaky podmínek
section-files = Soubory k instalaci
hint-group-type = Jak instalátor nechá uživatele vybírat možnosti v této skupině.
hint-default-type = Jak je možnost nabízena, když neodpovídá žádný vzor závislostí: povinná, volitelná, doporučená, nepoužitelná…
hint-operator = Všechny podmínky musí platit (A), nebo stačí kterákoli (NEBO).
hint-flags = Příznaky jsou pojmenované hodnoty, které tato možnost nastaví při výběru. Jiné kroky a možnosti je mohou testovat, aby se zobrazily, skryly nebo staly povinnými.
hint-plugin-dependencies = Vzory, které mění typ možnosti podle příznaků nebo souborů ve hře: například „Povinná“, když je nainstalován jiný mod.
hint-files = Soubory a složky zkopírované do složky Data hry při výběru této možnosti. Cíl je relativní k Data; při konfliktu vítězí vyšší priorita.
hint-visibility = Podmínky, které musí platit, aby se tento krok vůbec zobrazil. Nechte prázdné pro trvalé zobrazení.
seltype-exactly-one = Právě jedna (povinná)
seltype-at-most-one = Nejvýše jedna
seltype-any = Libovolný počet
seltype-all = Všechny (bez výběru)
seltype-at-least-one = Alespoň jedna
plugtype-required = Povinná
plugtype-optional = Volitelná
plugtype-recommended = Doporučená
plugtype-not-usable = Nepoužitelná
plugtype-could-be-usable = Možná použitelná
plugtype-required-hint = Vždy nainstalována; uživatel ji nemůže odškrtnout.
plugtype-optional-hint = Nabízena neodškrtnutá; rozhoduje uživatel.
plugtype-recommended-hint = Nabízena zaškrtnutá; uživatel ji může odškrtnout.
plugtype-not-usable-hint = Zobrazena šedě, nelze ji vybrat.
plugtype-could-be-usable-hint = Lze vybrat, ale instalátor varuje, že nemusí fungovat.
op-and = Všechny podmínky (A)
op-or = Kterákoli podmínka (NEBO)
theme-dark = Tmavý
theme-light = Světlý
theme-system = Podle systému
condeditor-setter-loc = Krok { "{step}" } / Skupina { "{group}" } / „{ "{name}" }“
condeditor-pattern-of = Vzor „{ "{name}" }“ → { "{type}" }
condeditor-visibility-of = Viditelnost kroku { "{step}" }
condeditor-cond-set = Podmíněná sada { "{num}" }
condeditor-needs = { "{ctx}" } (vyžaduje = { "{value}" })
condeditor-file-dep = { "{ctx}" }: soubor „{ "{name}" }“ ({ "{state}" })
menu-translate-fomod = Přeložit FOMOD…
ftr-title = Přeložit FOMOD
ftr-open-folder = Otevřít složku modu…
ftr-from-active = Z aktivního projektu
ftr-from-active-hint = Přeloží FOMOD projektu otevřeného v hlavním okně (nejprve musí být uložen).
ftr-no-fomod = Není načten žádný FOMOD.
ftr-encoding = Kódování původních souborů; přeložené soubory se zapisují ve stejném kódování.
ftr-source-lang = Z
ftr-target-lang = do
ftr-lang-locked = (po načtení FOMOD jsou jazyky pevně dané)
ftr-translator = Překladatel:
ftr-save = Uložit překlad
ftr-export = Exportovat přeložené soubory
ftr-export-sibling = Do složky fomod_<jazyk>
ftr-export-sibling-hint = Zapíše přeložené soubory info.xml a ModuleConfig.xml vedle původní složky fomod; původní soubory zůstanou nedotčené.
ftr-export-inplace = Přes původní soubory
ftr-export-inplace-hint = Nahradí fomod/info.xml a fomod/ModuleConfig.xml poté, co pro každý vytvoří kopii .bak s časovým razítkem.
ftr-force-explicit-order = Zachovat původní pořadí
ftr-warn-order = Seznamy řazené podle názvu (order="Ascending") by správce modů přeřadil podle přeložených názvů. Tato volba vynutí order="Explicit", aby si možnosti zachovaly současné pořadí.
ftr-update = Aktualizovat ze složky
ftr-update-hint = Znovu načte FOMOD z disku a sloučí s ním překlad: nové, změněné a odstraněné řetězce jsou nahlášeny.
ftr-preview-translated = Přeložený náhled
ftr-progress = Přeloženo { $done } / { $total }
ftr-filter-all = Všechny
ftr-filter-untranslated = Nepřeložené
ftr-filter-review = Ke kontrole
ftr-filter-issues = S problémy
ftr-filter-locked = Zamčené
ftr-type-all = Všechna pole
ftr-type-names = Názvy
ftr-type-descriptions = Popisy
ftr-type-meta = Informace o modu
ftr-search-hint = Hledat ve zdroji, překladu nebo kontextu…
ftr-next-untranslated = Další nepřeložený
ftr-show-whitespace = Zobrazit mezery a konce řádků
ftr-discard-question = Aktuální překlad obsahuje neuložené úpravy. Zahodit je a načíst druhý FOMOD?
ftr-discard-yes = Zahodit
ftr-unsaved-close = Překlad obsahuje neuložené úpravy.
ftr-col-num = Č.
ftr-col-status = { "" }
ftr-col-context = Kontext
ftr-col-source = Zdroj
ftr-col-target = Překlad
ftr-col-issues = { "" }
ftr-empty-hint = Otevřete složku modu nebo načtěte aktivní projekt, aby se zobrazily jeho přeložitelné řetězce.
ftr-empty-filter = Aktuálnímu filtru neodpovídá žádný řetězec.
ftr-select-row = Vyberte řádek a upravte jeho překlad.
ftr-copy-source = Kopírovat zdroj
ftr-clear-target = Vymazat
ftr-lock = Nepřekládat
ftr-lock-hint = Zamčené řetězce se zapisují beze změny (autor, web, vlastní jména…).
ftr-note = Poznámka:
ftr-status-untranslated = Nepřeloženo
ftr-status-translated = Přeloženo
ftr-status-auto = Předvyplněno automaticky — zkontrolujte
ftr-status-fuzzy = Zdrojový text se od překladu změnil — zkontrolujte
ftr-status-obsolete = Ve FOMOD již neexistuje
ftr-status-locked = Zamčeno (zapisuje se beze změny)
ftr-field-info-name = Název modu (info.xml)
ftr-field-module-name = Titulek instalátoru (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Web
ftr-field-description = Popis modu
ftr-field-step = Název kroku
ftr-field-group = Název skupiny
ftr-field-plugin = Název možnosti
ftr-field-plugin-desc = Popis možnosti
ftr-issue-empty = Prázdný překlad
ftr-issue-whitespace = Překlad obsahuje pouze mezery
ftr-issue-edge-whitespace = Mezery na začátku nebo na konci se liší od zdroje
ftr-issue-token = Chráněné tokeny se liší — chybí: { $missing } ; navíc: { $extra }
ftr-issue-newline-name = Název nesmí obsahovat konec řádku
ftr-issue-control = Obsahuje znaky, které XML nedokáže uložit
ftr-issue-length = Neobvyklá délka oproti zdroji (×{ $ratio })
ftr-issue-identical = Shodné se zdrojem
ftr-issue-duplicate = Stejný zdrojový text je v { $key } přeložen jinak
ftr-issue-cdata = Sekvence ]]> zde není povolena
ftr-load-error = FOMOD se nepodařilo načíst: { $error }
ftr-extracted = Nalezeno přeložitelných řetězců: { $num }.
ftr-sidecar-found = Existující překlad načten a sloučen: nové { $new }, změněné { $changed }, odstraněné { $removed }.
ftr-saved = Překlad uložen do { $path }
ftr-save-error = Překlad se nepodařilo uložit: { $error }
ftr-save-first = Nejprve projekt uložte a poté jej přeložte.
ftr-export-success = Řetězců zapsaných do { $path }: { $count }
ftr-export-error = Export se nezdařil: { $error }
ftr-export-blocked = Před exportem je třeba opravit blokující problémy: { $num }.
ftr-export-stale = Přeskočeno řetězců, protože se FOMOD změnil: { $num }; použijte „Aktualizovat ze složky“.
ftr-update-report = Aktualizováno: nové { $new }, změněné { $changed }, přesunuté { $moved }, odstraněné { $removed }, beze změny { $unchanged }.
menu-edit = Úpravy
menu-undo = Zpět
menu-redo = Znovu
tree-title = Projekt
tree-mod-info = Informace o modu
tree-steps = Kroky instalace
tree-required = Povinné soubory
tree-conditional = Podmíněné instalace
tree-empty-steps = Zatím žádný krok — kliknutím na + jej přidáte.
tree-duplicate = Duplikovat
tree-delete = Smazat
tree-save-template = Uložit jako šablonu…
tree-drop-hint = Přesuňte puštěním sem
cond-set-label = Podmíněná sada { $num }
inspector-empty = Vyberte položku ve stromu projektu nebo začněte přidáním kroku.
count-options = Možnosti: { $num }
count-files = Soubory: { $num }
msg-deleted-undo = Smazáno. Příkazem Zpět (Ctrl+Z) položku obnovíte.
problems-title = Problémy
problems-errors = Chyby: { $num }
problems-warnings = Varování: { $num }
btn-close = Zavřít
ftr-export-package = Jako překladový balíček (archiv)
ftr-export-package-hint = Vytvoří soubor .zip nebo .7z připravený k nahrání: přeložené soubory info.xml a ModuleConfig.xml a README (pouze záplata), nebo celý mod s přeloženými soubory (úplný).
ftr-package-full = Celý mod
ftr-package-full-hint = Zahrne do archivu všechny soubory modu, nejen dva přeložené soubory XML. Ověřte, že autor povoluje další šíření.
ftr-package-name-template = Název:
ftr-readme-patch = Tento archiv obsahuje překlad instalátoru modu „{ $name }“ (jazyk: { $langname }; soubory fomod/info.xml a fomod/ModuleConfig.xml). Nainstalujte jej přes původní mod nebo jej nechte sloučit správcem modů, aby přeložené soubory nahradily původní. Mění se pouze texty instalátoru; samotné soubory modu součástí nejsou. Vytvořeno v aplikaci XIMOD Architect.
ftr-readme-full = Tento archiv obsahuje mod „{ $name }“ s přeloženým instalátorem (jazyk: { $langname }; soubory fomod/info.xml a fomod/ModuleConfig.xml). Nainstalujte jej stejně jako původní mod. Změněny byly pouze texty instalátoru. Vytvořeno v aplikaci XIMOD Architect.
ftr-apply-memory = Vyplnit z paměti
ftr-memory-size = Překladová paměť — záznamů pro tento jazykový pár: { $num }. Každý uložený překlad se do ní přidá.
ftr-memory-applied = Řetězců vyplněných z překladové paměti (označeny „ke kontrole“): { $num }.
ftr-memory-suggestion = Paměť navrhuje:
ftr-use-suggestion = Použít
ftr-propagate = Rozšířit na shodné
ftr-propagate-hint = Zkopíruje tento překlad do všech ostatních dosud nepřeložených řetězců se stejným zdrojovým textem.
ftr-propagated = Vyplněno shodných řetězců: { $num }.
ftr-csv-export = Exportovat CSV…
ftr-csv-import = Importovat CSV…
ftr-csv-imported = Řetězců aktualizovaných ze souboru CSV: { $num }.
ftr-csv-error = Chyba CSV: { $error }
ftr-glossary = Glosář
ftr-glossary-source = Termín
ftr-glossary-target = Překlad
ftr-glossary-case = Velikost písmen
ftr-glossary-dnt = Zachovat
ftr-glossary-add = Přidat termín
ftr-issue-glossary = Glosář: „{ $term }“ není přeložen podle očekávání

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Otevřít archiv…
filter-archive = Archivy modů (zip, 7z)
msg-archive-opened = Archiv otevřen (rozbalených souborů: { $num }): { $path }
msg-archive-reused = Archiv je již rozbalen, znovu se použije { $path }
msg-archive-unsupported = Formát archivu „.{ $ext }“ není podporován; nejprve jej rozbalte pomocí 7-Zip (otevřít lze pouze .zip a .7z).
msg-archive-error = Chyba při otevírání archivu: { $error }
msg-archive-no-fomod = V archivu nebyla nalezena složka „fomod“ ({ $path })
msg-archive-extracting = Rozbalování archivu…
ftr-open-archive = Otevřít archiv modu…
ftr-package-full-partial = Mod byl otevřen z archivu obsahujícího pouze složku fomod; úplné balíčky vyžadují rozbalený mod.
info-module-deps = Požadavky modu
info-module-deps-hint = Soubory nebo příznaky, které celý mod vyžaduje před spuštěním instalátoru (moduleDependencies). Pokud žádné nejsou, ponechte prázdné.
info-header-advanced = Rozšířené záhlaví
info-title-position = Umístění titulku
info-title-colour = Barva titulku
info-title-colour-hint = Očekává se: šest šestnáctkových číslic (RRGGBB)
info-image-show = Zobrazit obrázek záhlaví
info-image-fade = Prolínat obrázek záhlaví
info-image-height = Výška obrázku záhlaví
info-attr-default = (výchozí)
file-always-install = Vždy
file-always-install-hint = Vždy nainstalovat tento soubor, i když volba není vybrána (alwaysInstall).
file-install-if-usable = Pokud lze
file-install-if-usable-hint = Nainstalovat tento soubor, kdykoli je volba použitelná, i když není vybrána (installIfUsable).
msg-import-lossy = Tento FOMOD obsahuje konstrukce, které XIMOD nedokáže upravovat (počet: { $num }); při uložení projektu budou zahozeny.
fidelity-nested-deps = Vnořená skupina závislostí, umístění: { $context } (podporována je pouze jedna úroveň)
fidelity-game-dep = Požadavek na verzi hry { $version }, umístění: { $context }
fidelity-fomm-dep = Požadavek na verzi správce modů { $version }, umístění: { $context }
fidelity-unknown = Prvek „{ $element }“ v „{ $parent }“ není podporován ({ $context })
loc-module = požadavky modu
loc-step = krok { $step } „{ $name }“
loc-installer = instalátor

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Obnovit zálohu…
backups-title = Obnovit zálohu
backups-empty = Tento projekt zatím nemá žádnou zálohu. Záloha se vytvoří při každém uložení projektu přes předchozí verzi.
backups-changes = Počet změn oproti aktuálnímu projektu: { $num }
btn-compare = Porovnat
btn-restore = Obnovit
btn-delete-backups = Smazat všechny zálohy
btn-delete-backups-confirm = Kliknutím znovu smažete všechny zálohy
msg-backup-restored = Záloha z { $time } obnovena do editoru (zatím neuloženo; akce Zpět ji vrátí)
msg-backups-deleted = Smazaných záloh: { $num }
settings-backup-count = Počet uchovávaných záloh:
settings-backup-count-hint = Počet předchozích verzí XML souborů FOMOD uchovávaných ve složce fomod/backups při ukládání (0 = žádné zálohy).
settings-autosave-minutes = Automaticky ukládat kopii pro obnovení každých (minut):
settings-autosave-minutes-hint = V tomto intervalu se do složky s konfigurací zapisuje kopie pro obnovení každého změněného projektu; při příštím spuštění je nabídnuta pouze po neobvyklém ukončení (0 = vypnuto).
settings-auto-masters = Přidat mastery pluginu jako podmínky
settings-auto-masters-hint = Když je plugin (.esp/.esm/.esl) přidán k volbě, mastery, které vyžaduje a které neposkytuje hra ani tento mod, se stanou souborovými podmínkami „Active“ dané volby.
msg-author-from-plugin = Autor doplněn z hlavičky pluginu: { $author }
msg-masters-added = Počet masterů pluginu { $plugin } přidaných jako souborové podmínky: { $num }
issue-missing-master = { $plugin } vyžaduje { $master }, který není v tomto modu ani není deklarován jako závislost
issue-esl-mismatch-flag = { $plugin } má příponu .esl, ale jeho příznak light (ESL) není nastaven
issue-esl-eligible = { $plugin } by mohl být označen jako light (nových záznamů: { $num }, limit { $limit })
issue-esl-too-big = { $plugin } je označen jako light, ale nesplňuje pravidla pro light pluginy (nových záznamů: { $num }, limit { $limit }, nebo FormID mimo povolený rozsah)
menu-plugin-report = Zpráva o pluginech…
plugins-title = Zpráva o pluginech
plugins-file = Soubor
plugins-kind = Typ
plugins-light = Příznak light
plugins-masters = Mastery
plugins-new-records = Nové záznamy / limit
plugins-eligible = Vhodný pro light
plugins-empty = Tento projekt neinstaluje žádný soubor pluginu (.esp, .esm ani .esl).
plugins-unreadable = nečitelný

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Výsledný strom souborů
preview-total-size = Celková velikost instalace: { $size }
preview-tree-truncated = Strom je zkrácen: příliš mnoho souborů k rozbalení (velikosti výše jsou částečné).
preview-overwritten-by = Přepsáno možností { $plugin }
preview-scenario = Scénář:
preview-scenario-load = Načíst
preview-scenario-save = Uložit…
preview-scenario-delete = Smazat
preview-scenario-name = Název scénáře
preview-scenario-saved = Scénář „{ $name }“ uložen do fomod/scenarios
preview-scenario-unresolved = Výběrů scénáře, které neodpovídají žádné možnosti tohoto projektu (přejmenována nebo odstraněna): { $num }
preview-scenario-none = (žádný scénář)
issue-unreachable-step = Krok „{ $step }“ se nikdy nemůže zobrazit: jeho podmínky viditelnosti testují hodnotu příznaku, kterou žádná předchozí možnost nenastavuje
issue-unreachable-option = Možnost „{ $plugin }“ nelze nikdy vybrat: její vzory použitelného typu testují hodnotu příznaku, kterou žádná možnost nenastavuje
issue-unreachable-cond = Podmíněná sada souborů { $num } se nikdy nemůže uplatnit: její podmínky testují hodnotu příznaku, kterou žádná možnost nenastavuje
size-option = Velikost instalace: { $size } (souborů: { $num })
size-missing = Chybějících zdrojů: { $num }
size-unknown = Velikost instalace: — (spusťte Ověřit pro změření)
menu-nexus-desc = Popis pro Nexus…
nexus-title = Popis pro Nexus Mods
nexus-format = Formát:
nexus-include-requirements = Požadavky
nexus-include-options = Možnosti instalace
nexus-include-install = Instalace
nexus-include-changelog = Seznam změn
nexus-previous = Předchozí verze…
nexus-previous-none = (žádná předchozí verze: bez seznamu změn)
nexus-language = Jazyk:
nexus-language-source = (zdroj)
nexus-sec-requirements = Požadavky
nexus-sec-options = Možnosti instalace
nexus-sec-install = Instalace
nexus-sec-changelog = Seznam změn
nexus-install-text = Tento mod obsahuje instalátor FOMOD: nainstalujte jej správcem modů (Vortex, Mod Organizer 2) a vyberte své možnosti v instalátoru.
nexus-requires = Vyžaduje
nexus-step = Krok
nexus-added = Přidáno
nexus-removed = Odstraněno
nexus-changed = Změněno
btn-copy = Kopírovat
btn-save-as = Uložit jako…
msg-copied = Zkopírováno do schránky
msg-saved-to = Uloženo do { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Přejmenovat…
condeditor-rename-exists = Příznak s názvem „{ $name }“ již existuje
condeditor-renamed = Příznak „{ $from }“ přejmenován na „{ $to }“ (výskytů: { $num })
condeditor-delete-uses = Smazat všechna použití
condeditor-deleted-uses = Příznak „{ $name }“ odstraněn všude (výskytů: { $num })
condeditor-values-set = Nastavované hodnoty:
condeditor-values-tested = Testované hodnoty:
condeditor-value-never-set = { $value } — testována, ale nikdy nenastavena
condeditor-value-never-tested = { $value } — nastavena, ale nikdy netestována
condeditor-builder = Tvůrce podmínek
condeditor-builder-none = Vyberte v hlavním okně krok, možnost, podmíněnou sadu souborů nebo informace o modu a upravte zde jejich podmínky.
condeditor-builder-pattern = Vzor:
condeditor-sentence-if = POKUD
condeditor-sentence-and = A
condeditor-sentence-or = NEBO
condeditor-sentence-flag = příznak { "{name}" } = { "{value}" }
condeditor-sentence-file = soubor { "{name}" } je { "{value}" }
condeditor-sentence-empty = (žádná podmínka: vždy pravda)
condeditor-sentence-then-visible = PAK se krok zobrazí
condeditor-sentence-then-type = PAK se možnost stane { $type }
condeditor-sentence-then-install = PAK se soubory nainstalují
condeditor-sentence-then-module = PAK lze instalátor spustit (ověřuje se před jeho spuštěním)
issue-flag-value-never-set = Příznak „{ $flag }“ je testován s hodnotou „{ $value }“, kterou žádná možnost nenastavuje
issue-flag-never-used = Příznak „{ $flag }“ je nastaven, ale nikde není testován
menu-project-strings = Řetězce projektu…
strings-title = Řetězce projektu
strings-search = Hledat text, umístění nebo klíč…
strings-kind-all = Vše
strings-kind-names = Názvy
strings-kind-descriptions = Popisy
strings-duplicates-only = Pouze duplicity
strings-replace-with = Nahradit čím:
strings-case = Rozlišovat velikost písmen
strings-whole-word = Celé slovo
strings-replace-current = Nahradit
strings-replace-all = Nahradit vše
strings-replaced = Nahrazených řetězců: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Stejný text jako:
strings-count = Řetězců: { $num } · skupin duplicit: { $dups }
strings-col-location = Umístění
strings-col-field = Pole
strings-col-text = Text

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Obsah archivu…
filter-bethesda-archive = Archivy Bethesda (bsa, ba2)
archive-view-title = Obsah archivu
archive-view-format = Formát:
archive-view-entries = Položek: { $num }
archive-view-size = { $size } po rozbalení
archive-view-search = Hledat cestu…
archive-view-col-path = Cesta
archive-view-col-size = Velikost
archive-view-col-compressed = Komprimováno
archive-view-truncated = Zobrazeno je pouze prvních { $num } odpovídajících položek — upřesněte hledání.
archive-view-error = Tento archiv nelze přečíst: { $error }
archive-view-hint = Zobrazit obsah tohoto archivu
issue-conflict-archive = Stejný prostředek v několika archivech: „{ $path }“ je zabalen { $count } odkazy ({ $locs }) — o tom, který se použije, rozhoduje pořadí načítání archivů ve hře.
issue-conflict-archive-loose = Archiv versus volný soubor: „{ $path }“ je zároveň zabalen v archivu i nainstalován jako volný soubor ({ $locs }) — volný soubor má přednost před tím v archivu.
preview-in-archive = (v archivu)
preview-archived-size = z toho { $size } zabaleno v archivech

# --- Project tree: expand / collapse menus
tree-expand = Rozbalit
tree-collapse = Sbalit
tree-expand-all = Rozbalit vše
tree-expand-selected = Rozbalit vybrané
tree-expand-from = Rozbalit od vybraného
tree-collapse-all = Sbalit vše
tree-collapse-selected = Sbalit vybrané
tree-collapse-from = Sbalit od vybraného
tree-expand-all-hint = Rozbalí všechny nadpisy
tree-expand-selected-hint = Rozbalí pouze vybraný nadpis
tree-expand-from-hint = Rozbalí vybraný nadpis a vše pod ním
tree-collapse-all-hint = Sbalí všechny nadpisy
tree-collapse-selected-hint = Sbalí pouze vybraný nadpis
tree-collapse-from-hint = Sbalí vybraný nadpis a vše pod ním

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Přidat skupinu
btn-remove-group-cond = Odebrat skupinu
dep-type-game = Verze hry
dep-type-fomm = Verze správce modů
dep-group-hint = Skupina podmínek spojených pomocí A / NEBO; skupiny lze vnořovat.
condeditor-sentence-game = verze hry ≥ { "{value}" }
condeditor-sentence-fomm = verze správce modů ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Jedinečné texty
ftr-uniques-hint = Zobrazí jeden řádek pro každý odlišný zdrojový text. Překlad tohoto řádku přeloží najednou všechny řetězce se stejným textem.
ftr-uniques-synced = Aktualizováno shodných řetězců: { $num }.
ftr-uniques-group = Řetězců se stejným textem: { $num }; jeho překlad se použije na všechny.
