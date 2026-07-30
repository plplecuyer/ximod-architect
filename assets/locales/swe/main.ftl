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
menu-settings = Inställningar
menu-pre-save-script = Skript före sparande…
menu-post-save-script = Skript efter sparande…
menu-translation = Översättning…

# Menu - Help
menu-help = Hjälp
menu-about = Om

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
label-plugin-name = Plugin-namn:
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
btn-add-plugin = Lägg till plugin
btn-remove-plugin = Ta bort plugin
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
validation-no-plugins = Steg { $step }, grupp ”{ $name }” har inga plugins

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
default-plugin-name = Plugin { $num }
pattern-label = Mönster { $num }

# Selection prompts
msg-select-group-first = Välj först en grupp.
msg-select-plugin-edit = Välj ett plugin att redigera.
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
