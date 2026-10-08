# XIMOD Architect - translation metadata
# @language = swe
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Svenska
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Version { $version }

# Status messages
status-ready = Klar
msg-save-success = FOMOD sparades
msg-save-error = Fel vid sparande av FOMOD
msg-export-success = Distributionsarkiv skapat ({ $count } filer): { $path }
msg-export-error = Fel vid skapande av distributionsarkivet: { $error }
msg-load-success = FOMOD laddades
msg-load-error = Fel vid inläsning av FOMOD
msg-merge-success = FOMOD sammanslogs
msg-merge-error = Fel vid sammanslagning av FOMOD
msg-no-root-selected = Välj först en rotmapp
msg-no-fomod-folder = Ingen ”fomod”-mapp hittades. Skapa en?
msg-file-outside-root = Filen ligger utanför rotmappen

# Menu - File
menu-file = Arkiv
menu-new = Ny
menu-open = Öppna mapp…
menu-open-file = Öppna fil…
menu-save = Spara
menu-recent = Senaste
menu-exit = Avsluta
menu-merge = Slå samman FOMOD…
menu-export = Exportera distributionsarkiv…
# Menu - Options
menu-options = Alternativ
menu-settings = Inställningar…
menu-pre-save-script = Skript före sparande…
menu-post-save-script = Skript efter sparande…
menu-translation = Översätt gränssnittet…
# Menu - Help
menu-help = Hjälp
menu-check-updates = Sök efter uppdateringar…
menu-about = Om

# Update check
update-checking = Söker efter uppdateringar…
update-up-to-date = XIMOD Architect är uppdaterad.
update-check-failed = Kunde inte söka efter uppdateringar. Försök igen senare.
update-available-status = Version { $version } är tillgänglig.
update-banner-text = XIMOD Architect { $version } är tillgänglig.
update-download = Ladda ner:
update-skip = Hoppa över den här versionen
update-later = Senare

# Tabs
tab-info = Mod-info
tab-steps = Installationssteg
tab-required = Obligatoriska installationer
tab-conditional = Villkorliga installationer

# Info Tab
label-workspace = Arbetsyta
label-root-dir = Rotmapp:
label-mod-name = Mod-namn:
label-author = Upphovsperson:
label-version = Version:
label-game-name = Spelnamn:
label-category = Kategori:
label-url = Webbplats-URL:
label-header-image = Rubrikbild:
label-description = Beskrivning:
placeholder-select-dir = (Välj en mapp)
placeholder-select-game = (Välj ett spel)

# Steps Tab
label-step-name = Stegnamn:
label-group-name = Gruppnamn:
label-group-type = Grupptyp:
label-plugin-name = Alternativnamn:
label-plugin-desc = Beskrivning:
label-plugin-type = Standardtyp:
label-plugin-image = Bild:
label-visibility = Synlighetsvillkor
label-operator = Operator:

# Buttons
btn-browse = Bläddra…
btn-clear = Rensa
btn-add = Lägg till
btn-remove = Ta bort
btn-add-step = Nytt steg
btn-delete-step = Radera steg
btn-add-group = Lägg till grupp
btn-remove-group = Ta bort grupp
btn-add-plugin = Lägg till alternativ
btn-remove-plugin = Ta bort alternativ
btn-add-file = Lägg till fil
btn-add-folder = Lägg till mapp
btn-remove-file = Ta bort
btn-add-flag = Lägg till flagga
btn-remove-flag = Ta bort flagga
btn-add-condition = Lägg till villkor
btn-remove-condition = Ta bort villkor
btn-add-dependency = Lägg till beroende
btn-remove-dependency = Ta bort beroende
btn-add-pattern = Nytt mönster
btn-remove-pattern = Radera mönster
btn-save = Spara
btn-cancel = Avbryt
btn-ok = OK
btn-yes = Ja
btn-no = Nej

# Condition/Dependency Labels
label-flag-name = Flaggnamn:
label-flag-value = Värde:
label-condition-type = Typ:
label-condition-name = Namn:
label-condition-value = Värde:
label-dep-type = Beroendetyp:
label-dep-name = Namn/fil:
label-dep-value = Värde/tillstånd:

# Files
label-source = Källa
label-destination = Mål
label-priority = Prioritet
label-file-type = Typ

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Mål för hela gruppen
label-page-dest = Installationsmål (hela sidan)
btn-apply-group-dest = Tillämpa på alla alternativ i denna grupp
btn-apply-page-dest = Tillämpa på alla alternativ på denna sida
group-dest-hint = Anger ett installationsmål för varje fil i varje alternativ i denna grupp.
page-dest-hint = Anger ett installationsmål för varje fil i varje alternativ på denna sida (alla grupper).
bulk-dest-nofiles = Inga filer att uppdatera ännu — lägg först till filer i alternativen.
status-dest-applied = Mål tillämpat på { $num } fil(er).
preview-hidden-steps = { $num } steg dolda av aktuella val.
label-files = Filer
label-dependencies = Beroenden

# Settings Dialog
settings-title = Inställningar
settings-tab-general = Allmänt
settings-tab-recent-files = Senaste filer
settings-language = Språk:
settings-theme = Tema:
settings-font-size = Teckenstorlek:
settings-replace-newlines = Bearbeta radbrytningar i beskrivningar
settings-check-updates = Sök efter uppdateringar vid start
settings-max-recent = Max antal senaste filer:
settings-window-width = Fönsterbredd:
settings-window-height = Fönsterhöjd:
settings-no-recent-files = Inga senaste filer.

# Status messages for settings
status-settings-saved = Inställningarna sparades

# About Dialog
about-title = Om XIMOD Architect
about-description = Ett plattformsoberoende verktyg för att skapa FOMOD-installationsprogram för Bethesda-spelmoddar.
about-license = Licensierad under MIT-licensen
about-copyright = © 2024 XIMOD Team
about-credit = Rust-port av originalverktyget av Wenderer:

# Script Dialog
script-title = Redigera skript
script-info = Skript körs före eller efter sparande. Du kan använda följande makron:
script-macros = Tillgängliga makron:
macro-modname = $MODNAME$ - Mod-namn
macro-modauthor = $MODAUTHOR$ - Upphovspersonens namn
macro-modversion = $MODVERSION$ - Mod-version
macro-modroot = $MODROOT$ - Sökväg till rotmapp
macro-date = $DATE$ - Aktuellt datum (ÅÅÅÅ-MM-DD)
macro-time = $TIME$ - Aktuell tid (HH:MM:SS)
macro-random = $RANDOM$ - Slumptal

# Plugin Dependencies
label-plugin-dependencies = Alternativberoenden
label-default-type = Standardtyp:
label-pattern-type = Mönstertyp:
label-pattern-operator = Mönsteroperator:

# Conditional Files
label-pattern = Mönster

# Validation Messages
validation-no-name = Mod-namn krävs
validation-no-steps = Minst ett steg eller en obligatorisk fil krävs
validation-empty-step = Steg { $num } har inget namn
validation-empty-group = Steg { $step }, grupp { $group } har inget namn
validation-no-plugins = Steg { $step }, grupp ”{ $name }” har inga alternativ

# File States
state-active = Aktiv
state-inactive = Inaktiv
state-missing = Saknas

# Confirmation
confirm-title = Bekräftelse
confirm-delete = Är du säker på att du vill radera det här objektet?
confirm-discard = Du har osparade ändringar. Kassera dem och fortsätta?
confirm-unsaved = Du har osparade ändringar. Vill du spara innan du stänger?
confirm-save-issues = Projektet har följande problem:
confirm-save-anyway = Spara ändå?

# Errors
error-invalid-xml = Ogiltig XML-fil
error-parse-failed = Det gick inte att tolka FOMOD
error-write-failed = Det gick inte att skriva filen
error-create-dir = Det gick inte att skapa mappen

# Default names (generated when creating new items)
default-step-name = Steg { $num }
default-group-name = Grupp { $num }
default-plugin-name = Alternativ { $num }
pattern-label = Mönster { $num }

# Selection prompts
msg-select-group-first = Välj först en grupp.
msg-select-plugin-edit = Välj ett alternativ att redigera.
label-empty = (tom)
image-no-image = Ingen bild

# File dialog filters
filter-images = Bilder
filter-xml = XML

# Dependency types
dep-type-flag = Flagga
dep-type-file = Fil

# Status bar
status-modified = Ändrad

# Status messages (errors)
msg-settings-save-error = Fel vid sparande av inställningar
msg-script-save-error = Fel vid sparande av skript

# Translation editor
trans-title = Översättningsredigerare
trans-source-lang = Visat språk:
trans-target-lang = Språk att översätta:
trans-col-key = Nyckel
trans-col-source = Etikett
trans-col-target = Översättning
trans-saved = Översättningen sparades
trans-save-error = Fel vid sparande av översättning

# XML editor
xml-editor-title = XML-redigerare
xml-editor-edit = Redigera
xml-editor-apply = Verkställ
xml-editor-revert = Avbryt
xml-editor-readonly = Skrivskyddad
xml-editor-editing = Redigerar — grafiska flikar är låsta
xml-editor-error = Fel:
xml-editor-applied = XML-ändringarna verkställdes
xml-editor-wellformed = Välformad XML
xml-editor-error-at = Rad { $line }, kolumn { $col }: { $msg }

# Country / flag picker
settings-country-name = Landsnamn:
settings-pick-country = Klicka för att välja ditt land
flags-title = Välj ett land
flags-filter = Filter:
flags-none = Ingen flagga hittades

# Translation editor: country & font
trans-endonym = Landets endonym:
trans-font = Teckensnitt:
trans-no-font = (inget)
trans-browse = Bläddra…
trans-google-fonts = Google Fonts
trans-pick-country = Klicka för att välja landet
trans-font-outside = Teckensnittet måste först installeras i assets/fonts.
trans-font-dir-missing = Mappen assets/fonts hittades inte.

# Translation submission
trans-lang-endonym = Språkets endonym:
trans-author = Upphovsperson:
trans-submit = Skicka…
trans-submit-hint = Bygg en zip och öppna ett förifyllt e-postmeddelande
trans-data-updated = Referensdata uppdaterade (Languages.json / Countries.json)
trans-package-ready = Arkivet är klart:
trans-package-error = Det gick inte att bygga arkivet:

# ISO 639-3 requirement
trans-lang-not-iso = Översättning är endast möjlig för ett språk med en ISO 639-3-kod.

# FOMOD installer preview
menu-preview = Förhandsgranska installationsprogram…
preview-title = Förhandsgranskning av FOMOD-installationsprogram
preview-refresh = Uppdatera
preview-assumptions = Filantaganden
preview-details = Detaljer
preview-back = Tillbaka
preview-next = Nästa
preview-install = Installera
preview-close = Stäng
preview-restart = Starta om
preview-summary-title = Filer som kommer att installeras
preview-empty = Ingen fil skulle installeras.
preview-none-option = (inget)
preview-invalid = Slutför de obligatoriska valen för att fortsätta.
preview-no-steps = Inget steg är synligt; se installationssammanfattningen.
preview-select-hint = Välj ett alternativ för att se dess beskrivning.
preview-col-source = Källa
preview-col-dest = Mål
preview-col-priority = Prioritet
preview-sel-exactlyone = Välj exakt ett alternativ.
preview-sel-atmostone = Välj högst ett alternativ.
preview-sel-any = Välj valfritt antal alternativ.
preview-sel-all = Alla alternativ installeras.
preview-sel-atleastone = Välj minst ett alternativ.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validera FOMOD
validate-report-title = FOMOD-validering
validate-ok = Inga problem hittades. FOMOD följer schemat.
xml-editor-schema-ok = Följer ModConfig 5.0-schemat.
xml-editor-schema-issues = Schemaproblem:
schema-line-col = Rad { $line }, kol. { $col }: { $msg }
schema-wrong-root = Oväntad rot "{ $found }" (förväntade "{ $expected }").
schema-unknown = Oväntat element "{ $element }" i "{ $parent }".
schema-missing = "{ $parent }" måste innehålla "{ $child }".
schema-needs-one = "{ $parent }" måste innehålla minst ett "{ $child }".
schema-too-many = "{ $child }" får endast förekomma en gång i "{ $parent }".
schema-missing-attr = Attributet "{ $attr }" krävs på "{ $element }".
schema-bad-enum = Ogiltigt värde "{ $value }" för { $element }/@{ $attr } (förväntade: { $allowed }).
schema-choose-one = "{ $parent }" måste innehålla exakt ett av: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Flytta före
reorder-after = Flytta efter

# Country / language database explorer (Properties)
menu-properties = Egenskaper…
prop-title = Databas för länder/språk
prop-tab-countries = Länder
prop-tab-languages = Språk
prop-filter = Filter:
prop-official-langs = Officiella språk
prop-spoken-langs = Talade språk
prop-endonym = Landets endonym
prop-font = Teckensnitt
prop-spoken-in = Talas i
prop-select-country = Välj ett land för att se dess detaljer.
prop-select-lang = Välj ett språk för att se dess detaljer.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Öppna spelets Nexus Mods-sida

# Referenced-file verification (V2)
verify-no-root = Filkontroll hoppades över: ingen rotmapp är angiven
loc-header = sidhuvudsbild
loc-required = obligatoriska filer
loc-conditional = villkorlig uppsättning { $num }
loc-plugin = steg { $step }, grupp { $group }, alternativ ”{ $plugin }”
verify-missing-file = Saknad fil: { $path } ({ $loc })
verify-missing-folder = Saknad mapp: { $path } ({ $loc })
verify-missing-image = Saknad bild: { $path } ({ $loc })
verify-absolute = Absolut sökväg (inte portabel): { $path } ({ $loc })
verify-outside = Sökvägen lämnar rotmappen: { $path } ({ $loc })
verify-orphan = Föräldralös fil (refereras inte av något alternativ): { $path }
conflict-certain = Målkonflikt: ”{ $path }” skrivs av { $count } alternativ ({ $locs }) – de skriver över varandra.
conflict-potential = Möjlig målkonflikt: ”{ $path }” är mål för { $count } referenser ({ $locs }) – överskrivning beror på val/villkor.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Stäng FOMOD
menu-close-all-fomods = Stäng alla FOMOD
tab-untitled = (namnlös)
msg-drop-not-fomod = Det släppta objektet är inte en FOMOD (ingen ”fomod”-mapp hittades)
exit-title = Osparade ändringar
exit-unsaved = En FOMOD har inte sparats. Vill du spara den?
tab-close-hint = Stäng den här FOMOD
menu-new-from-folder = Nytt från mapp…
menu-templates = Mallar…
templates-title = Återanvändbara mallar
templates-empty = Inga mallar sparade än. Spara det markerade steget ovan för att skapa en.
templates-insert = Infoga
templates-save-step = Spara markerat steg
templates-name-hint = Mallnamn (valfritt)
msg-wizard-success = Struktur skapad från mapp: { $num } alternativ.
msg-wizard-error = Fel: { $error }
msg-template-saved = Mall sparad: { $name }
msg-template-inserted = Mall infogad i projektet.
msg-template-no-step = Välj först ett steg för att spara det som en mall.
msg-template-no-dir = Det gick inte att hitta mallmappen.
msg-drop-assigned = { $added } källa/källor tillagd(a) till alternativet ({ $rejected } utanför roten ignorerades).
menu-compare = Jämför med…
compare-title = FOMOD-jämförelse
compare-none = Inga skillnader.
btn-optimize-image = Optimera bild
msg-image-optimized = Rubrikbild optimerad.
msg-image-ok = Rubrikbilden ligger redan inom gränserna.
msg-no-header-image = Ingen rubrikbild att optimera.
verify-image-large = Bilden är för stor ({ $width }×{ $height }): { $path }
verify-image-format = Bildformat som inte stöds (.{ $ext }): { $path }
verify-image-unreadable = Oläsbar bild: { $path }
menu-condition-editor = Villkorsredigerare…
condeditor-title = Villkorsredigerare
condeditor-set-by = Satt av:
condeditor-used-by = Används av:
condeditor-filedeps = Filberoenden
condeditor-empty = Inga flaggor eller beroenden i detta projekt.
condeditor-orphan-set = satt men aldrig använd
condeditor-orphan-used = använd men aldrig satt
msg-img-optimized = Bild optimerad.
msg-img-ok = Bilden ligger redan inom gränserna.
msg-img-none = Ingen bild att optimera.
msg-crash-recovery = Föregående session avslutades oväntat. En säkerhetskopia av ditt projekt har sparats i { $path }
export-progress-title = Skapar distributionsarkivet…
export-progress-files = { $done } / { $total } filer
msg-export-cancelled = Exporten avbröts; det ofullständiga arkivet togs bort.
verify-running = Kontrollerar filer på disken…
verify-stale = Obs: projektet ändrades medan filerna kontrollerades; kör valideringen igen.
prop-col-name = Namn
menu-save-as = Spara som…
menu-project = Projekt
menu-tools = Verktyg
menu-manual = Användarhandbok
msg-manual-missing = Användarhandboken (PDF) hittades inte bredvid programmet.
toolbar-new = Ny
toolbar-open = Öppna
toolbar-save = Spara
toolbar-validate = Validera
toolbar-preview = Förhandsgranska
toolbar-export = Exportera
dialog-choose-root = Välj moddens rotmapp
exit-unsaved-docs = Osparat: { $names }
status-summary = { $steps } steg · { $options } alternativ
section-groups = Grupper
section-options = Alternativ
section-flags = Villkorsflaggor
section-files = Filer att installera
hint-group-type = Hur installationsprogrammet låter användaren välja alternativ i den här gruppen.
hint-default-type = Hur alternativet erbjuds när inget av dess beroendemönster matchar: obligatoriskt, valfritt, rekommenderat, oanvändbart…
hint-operator = Alla villkor måste vara sanna (OCH), eller något av dem (ELLER).
hint-flags = Flaggor är namngivna värden som det här alternativet sätter när det väljs. Andra steg och alternativ kan testa dem för att visas, döljas eller bli obligatoriska.
hint-plugin-dependencies = Mönster som ändrar alternativets typ beroende på flaggor eller filer i spelet: till exempel ”Obligatoriskt” när en annan modd är installerad.
hint-files = Filer och mappar som kopieras till spelets Data-mapp när det här alternativet väljs. Målet är relativt Data; vid konflikt vinner högsta prioritet.
hint-visibility = Villkor som måste uppfyllas för att det här steget ska visas alls. Lämna tomt för att alltid visa det.
seltype-exactly-one = Exakt ett (obligatoriskt)
seltype-at-most-one = Högst ett
seltype-any = Valfritt antal
seltype-all = Alla (inget val)
seltype-at-least-one = Minst ett
plugtype-required = Obligatoriskt
plugtype-optional = Valfritt
plugtype-recommended = Rekommenderat
plugtype-not-usable = Oanvändbart
plugtype-could-be-usable = Kanske användbart
plugtype-required-hint = Installeras alltid; användaren kan inte avmarkera det.
plugtype-optional-hint = Erbjuds omarkerat; användaren bestämmer.
plugtype-recommended-hint = Erbjuds förmarkerat; användaren kan avmarkera det.
plugtype-not-usable-hint = Visas nedtonat och kan inte väljas.
plugtype-could-be-usable-hint = Valbart, men installationsprogrammet varnar för att det kanske inte fungerar.
op-and = Alla villkor (OCH)
op-or = Något villkor (ELLER)
theme-dark = Mörkt
theme-light = Ljust
theme-system = Följ systemet
condeditor-setter-loc = Steg { "{step}" } / Grupp { "{group}" } / ”{ "{name}" }”
condeditor-pattern-of = Mönster för ”{ "{name}" }” → { "{type}" }
condeditor-visibility-of = Synlighet för steg { "{step}" }
condeditor-cond-set = Villkorlig uppsättning { "{num}" }
condeditor-needs = { "{ctx}" } (kräver = { "{value}" })
condeditor-file-dep = { "{ctx}" }: fil ”{ "{name}" }” ({ "{state}" })
menu-translate-fomod = Översätt en FOMOD…
ftr-title = Översätt en FOMOD
ftr-open-folder = Öppna en moddmapp…
ftr-from-active = Från det aktiva projektet
ftr-from-active-hint = Översätter FOMOD:en för projektet som är öppet i huvudfönstret (det måste sparas först).
ftr-no-fomod = Ingen FOMOD inläst.
ftr-encoding = Originalfilernas teckenkodning; de översatta filerna skrivs med samma teckenkodning.
ftr-source-lang = Från
ftr-target-lang = till
ftr-lang-locked = (språken är låsta när en FOMOD har lästs in)
ftr-translator = Översättare:
ftr-save = Spara översättningen
ftr-export = Exportera de översatta filerna
ftr-export-sibling = Till en mapp fomod_<språk>
ftr-export-sibling-hint = Skriver de översatta filerna info.xml och ModuleConfig.xml bredvid den ursprungliga fomod-mappen; originalfilerna rörs inte.
ftr-export-inplace = Över originalfilerna
ftr-export-inplace-hint = Ersätter fomod/info.xml och fomod/ModuleConfig.xml efter att ha gjort en tidsstämplad .bak-kopia av vardera.
ftr-force-explicit-order = Behåll den ursprungliga ordningen
ftr-warn-order = Listor sorterade efter namn (order="Ascending") skulle sorteras om efter de översatta namnen i moddhanteraren. Detta tvingar fram order="Explicit" så att alternativen behåller sin nuvarande ordning.
ftr-update = Uppdatera från mapp
ftr-update-hint = Läser in FOMOD:en från disken igen och sammanfogar översättningen med den: nya, ändrade och borttagna strängar rapporteras.
ftr-preview-translated = Översatt förhandsgranskning
ftr-progress = { $done } / { $total } översatta
ftr-filter-all = Alla
ftr-filter-untranslated = Oöversatta
ftr-filter-review = Att granska
ftr-filter-issues = Med problem
ftr-filter-locked = Låsta
ftr-type-all = Alla fält
ftr-type-names = Namn
ftr-type-descriptions = Beskrivningar
ftr-type-meta = Moddinformation
ftr-search-hint = Sök i källa, översättning eller sammanhang…
ftr-next-untranslated = Nästa oöversatta
ftr-show-whitespace = Visa blanksteg och radbrytningar
ftr-discard-question = Den aktuella översättningen har osparade ändringar. Kassera dem och läsa in den andra FOMOD:en?
ftr-discard-yes = Kassera
ftr-unsaved-close = Översättningen har osparade ändringar.
ftr-col-num = Nr
ftr-col-status = { "" }
ftr-col-context = Sammanhang
ftr-col-source = Källa
ftr-col-target = Översättning
ftr-col-issues = { "" }
ftr-empty-hint = Öppna en moddmapp, eller läs in det aktiva projektet, för att lista dess översättningsbara strängar.
ftr-empty-filter = Ingen sträng matchar det aktuella filtret.
ftr-select-row = Välj en rad för att redigera dess översättning.
ftr-copy-source = Kopiera källan
ftr-clear-target = Rensa
ftr-lock = Översätt inte
ftr-lock-hint = Låsta strängar skrivs oförändrade (upphovsperson, webbplats, egennamn…).
ftr-note = Anteckning:
ftr-status-untranslated = Oöversatt
ftr-status-translated = Översatt
ftr-status-auto = Förifylld automatiskt — granska gärna
ftr-status-fuzzy = Källtexten har ändrats sedan detta översattes — granska gärna
ftr-status-obsolete = Finns inte längre i FOMOD:en
ftr-status-locked = Låst (skrivs oförändrad)
ftr-field-info-name = Moddnamn (info.xml)
ftr-field-module-name = Installationsprogrammets titel (ModuleConfig.xml)
ftr-field-author = Upphovsperson
ftr-field-website = Webbplats
ftr-field-description = Moddbeskrivning
ftr-field-step = Stegnamn
ftr-field-group = Gruppnamn
ftr-field-plugin = Alternativnamn
ftr-field-plugin-desc = Alternativbeskrivning
ftr-issue-empty = Tom översättning
ftr-issue-whitespace = Översättningen innehåller bara blanksteg
ftr-issue-edge-whitespace = Blanksteg i början eller slutet skiljer sig från källan
ftr-issue-token = Skyddade token skiljer sig — saknas: { $missing } ; överflödiga: { $extra }
ftr-issue-newline-name = Ett namn får inte innehålla en radbrytning
ftr-issue-control = Innehåller tecken som XML inte kan lagra
ftr-issue-length = Ovanlig längd jämfört med källan (×{ $ratio })
ftr-issue-identical = Identisk med källan
ftr-issue-duplicate = Samma källtext är översatt annorlunda i { $key }
ftr-issue-cdata = Sekvensen ]]> är inte tillåten här
ftr-load-error = Det gick inte att läsa in FOMOD:en: { $error }
ftr-extracted = { $num } översättningsbara strängar hittades.
ftr-sidecar-found = Befintlig översättning inläst och sammanfogad: { $new } nya, { $changed } ändrade, { $removed } borttagna.
ftr-saved = Översättningen sparades i { $path }
ftr-save-error = Det gick inte att spara översättningen: { $error }
ftr-save-first = Spara projektet först och översätt det sedan.
ftr-export-success = { $count } strängar skrevs till { $path }
ftr-export-error = Exporten misslyckades: { $error }
ftr-export-blocked = { $num } blockerande problem måste åtgärdas före export.
ftr-export-stale = { $num } strängar hoppades över eftersom FOMOD:en har ändrats; använd ”Uppdatera från mapp”.
ftr-update-report = Uppdaterat: { $new } nya, { $changed } ändrade, { $moved } flyttade, { $removed } borttagna, { $unchanged } oförändrade.
menu-edit = Redigera
menu-undo = Ångra
menu-redo = Gör om
tree-title = Projekt
tree-mod-info = Mod-information
tree-steps = Installationssteg
tree-required = Obligatoriska filer
tree-conditional = Villkorliga installationer
tree-empty-steps = Inga steg än — klicka på + för att lägga till ett.
tree-duplicate = Duplicera
tree-delete = Radera
tree-save-template = Spara som mall…
tree-drop-hint = Släpp här för att flytta
cond-set-label = Villkorlig uppsättning { $num }
inspector-empty = Markera ett objekt i projektträdet eller lägg till ett steg för att börja.
count-options = { $num } alternativ
count-files = { $num } filer
msg-deleted-undo = Raderat. Använd Ångra (Ctrl+Z) för att återställa det.
problems-title = Problem
problems-errors = { $num } fel
problems-warnings = { $num } varningar
btn-close = Stäng
ftr-export-package = Som översättningspaket (arkiv)
ftr-export-package-hint = Skapar en .zip eller .7z som är klar att ladda upp: de översatta filerna info.xml och ModuleConfig.xml plus en README (endast patch), eller hela modden med de översatta filerna (komplett).
ftr-package-full = Hela modden
ftr-package-full-hint = Ta med moddens alla filer i arkivet, inte bara de två översatta XML-filerna. Kontrollera att upphovspersonen tillåter vidaredistribution.
ftr-package-name-template = Namn:
ftr-readme-patch = Det här arkivet innehåller översättningen ({ $langname }) av installationsprogrammet för ”{ $name }” (fomod/info.xml och fomod/ModuleConfig.xml). Installera det ovanpå den ursprungliga modden, eller låt din moddhanterare sammanfoga det, så att de översatta filerna ersätter originalen. Endast installationsprogrammets texter ändras; själva moddfilerna ingår inte. Skapat med XIMOD Architect.
ftr-readme-full = Det här arkivet innehåller ”{ $name }” med installationsprogrammet översatt ({ $langname }; fomod/info.xml och fomod/ModuleConfig.xml). Installera det på samma sätt som den ursprungliga modden. Endast installationsprogrammets texter har ändrats. Skapat med XIMOD Architect.
ftr-apply-memory = Fyll i från minnet
ftr-memory-size = Översättningsminne: { $num } poster för det här språkparet. Varje sparad översättning läggs till i det.
ftr-memory-applied = { $num } strängar ifyllda från översättningsminnet (markerade ”att granska”).
ftr-memory-suggestion = Minnet föreslår:
ftr-use-suggestion = Använd
ftr-propagate = Överför till identiska
ftr-propagate-hint = Kopiera den här översättningen till alla andra strängar med samma källtext som fortfarande är oöversatta.
ftr-propagated = { $num } identiska strängar ifyllda.
ftr-csv-export = Exportera CSV…
ftr-csv-import = Importera CSV…
ftr-csv-imported = { $num } strängar uppdaterade från CSV-filen.
ftr-csv-error = CSV-fel: { $error }
ftr-glossary = Ordlista
ftr-glossary-source = Term
ftr-glossary-target = Översättning
ftr-glossary-case = Skiftläge
ftr-glossary-dnt = Behåll
ftr-glossary-add = Lägg till term
ftr-issue-glossary = Ordlista: ”{ $term }” är inte översatt som förväntat

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Öppna arkiv…
filter-archive = Mod-arkiv (zip, 7z)
msg-archive-opened = Arkivet öppnat ({ $num } filer extraherade): { $path }
msg-archive-reused = Arkivet är redan extraherat, återanvänder { $path }
msg-archive-unsupported = Arkivformatet ”.{ $ext }” stöds inte; extrahera det först med 7-Zip (endast .zip och .7z kan öppnas).
msg-archive-error = Fel när arkivet öppnades: { $error }
msg-archive-no-fomod = Ingen ”fomod”-mapp hittades i arkivet ({ $path })
msg-archive-extracting = Extraherar arkivet…
ftr-open-archive = Öppna ett mod-arkiv…
ftr-package-full-partial = Modden öppnades från ett arkiv som bara innehåller dess fomod-mapp; fullständiga paket kräver den extraherade modden.
info-module-deps = Mod-krav
info-module-deps-hint = Filer eller flaggor som hela modden kräver innan installationsprogrammet körs (moduleDependencies). Lämna tomt om inga finns.
info-header-advanced = Avancerat sidhuvud
info-title-position = Titelplacering
info-title-colour = Titelfärg
info-title-colour-hint = Förväntat: sex hexadecimala siffror (RRGGBB)
info-image-show = Visa sidhuvudsbild
info-image-fade = Tona ut sidhuvudsbild
info-image-height = Sidhuvudsbildens höjd
info-attr-default = (standard)
file-always-install = Alltid
file-always-install-hint = Installera alltid den här filen, även när alternativet inte är valt (alwaysInstall).
file-install-if-usable = Om användbar
file-install-if-usable-hint = Installera den här filen när alternativet är användbart, även när det inte är valt (installIfUsable).
msg-import-lossy = Denna FOMOD innehåller { $num } konstruktioner som XIMOD inte kan redigera; de går förlorade när projektet sparas.
fidelity-nested-deps = Nästlad beroendegrupp i { $context } (endast en nivå stöds)
fidelity-game-dep = Krav på spelversion { $version } i { $context }
fidelity-fomm-dep = Krav på mod-manager-version { $version } i { $context }
fidelity-unknown = Elementet ”{ $element }” i ”{ $parent }” stöds inte ({ $context })
loc-module = mod-kraven
loc-step = steg { $step } ”{ $name }”
loc-installer = installationsprogrammet

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Återställ en säkerhetskopia…
backups-title = Återställ en säkerhetskopia
backups-empty = Det här projektet har ingen säkerhetskopia ännu. En skapas varje gång projektet sparas över en tidigare version.
backups-changes = { $num } ändring(ar) jämfört med det aktuella projektet
btn-compare = Jämför
btn-restore = Återställ
btn-delete-backups = Ta bort alla säkerhetskopior
btn-delete-backups-confirm = Klicka igen för att ta bort alla säkerhetskopior
msg-backup-restored = Säkerhetskopian från { $time } har återställts i redigeraren (inte sparad ännu; Ångra återställer den)
msg-backups-deleted = Borttagna säkerhetskopior: { $num }
settings-backup-count = Säkerhetskopior att behålla:
settings-backup-count-hint = Antal tidigare versioner av FOMOD-XML:en som behålls under fomod/backups vid sparande (0 = inga säkerhetskopior).
settings-autosave-minutes = Spara återställningskopia automatiskt var (minuter):
settings-autosave-minutes-hint = Med detta intervall skrivs en återställningskopia av varje ändrat projekt i konfigurationsmappen; den erbjuds vid nästa start endast efter en onormal avslutning (0 = av).
settings-auto-masters = Lägg till ett plugins masterfiler som villkor
settings-auto-masters-hint = När ett plugin (.esp/.esm/.esl) läggs till i ett alternativ blir de masterfiler det kräver, och som varken spelet eller denna mod tillhandahåller, ”Active”-filvillkor för alternativet.
msg-author-from-plugin = Författare ifylld från pluginets header: { $author }
msg-masters-added = { $num } masterfil(er) för { $plugin } tillagda som filvillkor
issue-missing-master = { $plugin } kräver { $master }, som varken finns i denna mod eller är angiven som beroende
issue-esl-mismatch-flag = { $plugin } har filändelsen .esl men dess light-flagga (ESL) är inte satt
issue-esl-eligible = { $plugin } skulle kunna markeras som light ({ $num } nya records, gräns { $limit })
issue-esl-too-big = { $plugin } är markerat som light men uppfyller inte reglerna för light-plugins ({ $num } nya records, gräns { $limit }, eller ett FormID utanför det tillåtna intervallet)
menu-plugin-report = Plugin-rapport…
plugins-title = Plugin-rapport
plugins-file = Fil
plugins-kind = Typ
plugins-light = Light-flagga
plugins-masters = Masterfiler
plugins-new-records = Nya records / gräns
plugins-eligible = Light-lämplig
plugins-empty = Det här projektet installerar ingen plugin-fil (.esp, .esm eller .esl).
plugins-unreadable = oläslig

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Slutgiltigt filträd
preview-total-size = Total installationsstorlek: { $size }
preview-tree-truncated = Trädet är avkortat: för många filer att expandera (storlekarna ovan är ofullständiga).
preview-overwritten-by = Överskriven av { $plugin }
preview-scenario = Scenario:
preview-scenario-load = Läs in
preview-scenario-save = Spara…
preview-scenario-delete = Ta bort
preview-scenario-name = Scenarionamn
preview-scenario-saved = Scenariot ”{ $name }” sparat under fomod/scenarios
preview-scenario-unresolved = { $num } val i scenariot matchar inget alternativ i detta projekt (omdöpt eller borttaget)
preview-scenario-none = (inget scenario)
issue-unreachable-step = Steget ”{ $step }” kan aldrig visas: dess synlighetsvillkor testar ett flaggvärde som inget tidigare alternativ sätter
issue-unreachable-option = Alternativet ”{ $plugin }” kan aldrig väljas: dess mönster för användbar typ testar ett flaggvärde som inget alternativ sätter
issue-unreachable-cond = Den villkorliga filuppsättningen { $num } kan aldrig gälla: dess villkor testar ett flaggvärde som inget alternativ sätter
size-option = Installationsstorlek: { $size } ({ $num } fil(er))
size-missing = Saknade källor: { $num }
size-unknown = Installationsstorlek: — (kör Validera för att mäta)
menu-nexus-desc = Nexus-beskrivning…
nexus-title = Nexus Mods-beskrivning
nexus-format = Format:
nexus-include-requirements = Krav
nexus-include-options = Installationsalternativ
nexus-include-install = Installation
nexus-include-changelog = Ändringslogg
nexus-previous = Föregående version…
nexus-previous-none = (ingen föregående version: ingen ändringslogg)
nexus-language = Språk:
nexus-language-source = (källa)
nexus-sec-requirements = Krav
nexus-sec-options = Installationsalternativ
nexus-sec-install = Installation
nexus-sec-changelog = Ändringslogg
nexus-install-text = Denna mod levereras med ett FOMOD-installationsprogram: installera den med en modhanterare (Vortex, Mod Organizer 2) och välj dina alternativ i installationsprogrammet.
nexus-requires = Kräver
nexus-step = Steg
nexus-added = Tillagt
nexus-removed = Borttaget
nexus-changed = Ändrat
btn-copy = Kopiera
btn-save-as = Spara som…
msg-copied = Kopierat till urklipp
msg-saved-to = Sparat i { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Byt namn…
condeditor-rename-exists = Det finns redan en flagga med namnet ”{ $name }”
condeditor-renamed = Flaggan ”{ $from }” har bytt namn till ”{ $to }” ({ $num } förekomst(er))
condeditor-delete-uses = Ta bort alla användningar
condeditor-deleted-uses = Flaggan ”{ $name }” har tagits bort överallt ({ $num } förekomst(er))
condeditor-values-set = Satta värden:
condeditor-values-tested = Testade värden:
condeditor-value-never-set = { $value } — testas men sätts aldrig
condeditor-value-never-tested = { $value } — sätts men testas aldrig
condeditor-builder = Villkorsbyggare
condeditor-builder-none = Välj ett steg, ett alternativ, en villkorad filuppsättning eller mod-informationen i huvudfönstret för att redigera dess villkor här.
condeditor-builder-pattern = Mönster:
condeditor-sentence-if = OM
condeditor-sentence-and = OCH
condeditor-sentence-or = ELLER
condeditor-sentence-flag = flaggan { "{name}" } = { "{value}" }
condeditor-sentence-file = filen { "{name}" } är { "{value}" }
condeditor-sentence-empty = (inget villkor: alltid sant)
condeditor-sentence-then-visible = DÅ visas steget
condeditor-sentence-then-type = DÅ blir alternativet { $type }
condeditor-sentence-then-install = DÅ installeras filerna
condeditor-sentence-then-module = DÅ kan installationsprogrammet köras (kontrolleras innan det startar)
issue-flag-value-never-set = Flaggan ”{ $flag }” testas med värdet ”{ $value }”, som inget alternativ sätter
issue-flag-never-used = Flaggan ”{ $flag }” sätts men testas aldrig någonstans
menu-project-strings = Projektsträngar…
strings-title = Projektsträngar
strings-search = Sök text, plats eller nyckel…
strings-kind-all = Alla
strings-kind-names = Namn
strings-kind-descriptions = Beskrivningar
strings-duplicates-only = Endast dubbletter
strings-replace-with = Ersätt med:
strings-case = Matcha skiftläge
strings-whole-word = Helt ord
strings-replace-current = Ersätt
strings-replace-all = Ersätt alla
strings-replaced = { $num } sträng(ar) ersatta
strings-dup-badge = ×{ $num }
strings-dup-hover = Samma text som:
strings-count = { $num } sträng(ar) · { $dups } dublettgrupp(er)
strings-col-location = Plats
strings-col-field = Fält
strings-col-text = Text

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Arkivinnehåll…
filter-bethesda-archive = Bethesda-arkiv (bsa, ba2)
archive-view-title = Arkivinnehåll
archive-view-format = Format:
archive-view-entries = { $num } poster
archive-view-size = { $size } uppackat
archive-view-search = Sök efter en sökväg…
archive-view-col-path = Sökväg
archive-view-col-size = Storlek
archive-view-col-compressed = Komprimerad
archive-view-truncated = Endast de första { $num } matchande posterna visas – begränsa sökningen.
archive-view-error = Det här arkivet kan inte läsas: { $error }
archive-view-hint = Visa innehållet i det här arkivet
issue-conflict-archive = Samma resurs i flera arkiv: ”{ $path }” packas av { $count } referenser ({ $locs }) – spelets laddningsordning för arkiv avgör vilken som används.
issue-conflict-archive-loose = Arkiv mot lös fil: ”{ $path }” är både packad i ett arkiv och installerad som lös fil ({ $locs }) – den lösa filen vinner över den arkiverade.
preview-in-archive = (i arkiv)
preview-archived-size = varav { $size } packat i arkiv

# --- Project tree: expand / collapse menus
tree-expand = Expandera
tree-collapse = Fäll ihop
tree-expand-all = Expandera alla
tree-expand-selected = Expandera markerad
tree-expand-from = Expandera från markerad
tree-collapse-all = Fäll ihop alla
tree-collapse-selected = Fäll ihop markerad
tree-collapse-from = Fäll ihop från markerad
tree-expand-all-hint = Expanderar alla rubriker
tree-expand-selected-hint = Expanderar endast den markerade rubriken
tree-expand-from-hint = Expanderar den markerade rubriken och allt under den
tree-collapse-all-hint = Fäller ihop alla rubriker
tree-collapse-selected-hint = Fäller ihop endast den markerade rubriken
tree-collapse-from-hint = Fäller ihop den markerade rubriken och allt under den

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Lägg till grupp
btn-remove-group-cond = Ta bort grupp
dep-type-game = Spelversion
dep-type-fomm = Mod-manager-version
dep-group-hint = En grupp villkor som kombineras med OCH / ELLER; grupper kan nästlas.
condeditor-sentence-game = spelversionen ≥ { "{value}" }
condeditor-sentence-fomm = mod-manager-versionen ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unika texter
ftr-uniques-hint = Visa en rad per distinkt källtext. Översätts den raden översätts alla strängar med samma text på en gång.
ftr-uniques-synced = { $num } identiska strängar uppdaterade.
ftr-uniques-group = { $num } strängar delar den här texten; översättningen gäller för alla.
