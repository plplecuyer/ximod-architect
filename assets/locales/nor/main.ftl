# XIMOD Architect - translation metadata
# @language = nor
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Norsk
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versjon { $version }

# Status messages
status-ready = Klar
msg-save-success = FOMOD ble lagret
msg-save-error = Feil ved lagring av FOMOD
msg-export-success = Distribusjonsarkiv opprettet ({ $count } filer): { $path }
msg-export-error = Feil ved oppretting av distribusjonsarkivet: { $error }
msg-load-success = FOMOD ble lastet inn
msg-load-error = Feil ved innlasting av FOMOD
msg-merge-success = FOMOD ble slått sammen
msg-merge-error = Feil ved sammenslåing av FOMOD
msg-no-root-selected = Velg først en rotmappe
msg-no-fomod-folder = Fant ingen «fomod»-mappe. Opprette en?
msg-file-outside-root = Filen er utenfor rotmappen

# Menu - File
menu-file = Fil
menu-new = Ny
menu-open = Åpne mappe…
menu-open-file = Åpne fil…
menu-save = Lagre
menu-recent = Nylige
menu-exit = Avslutt
menu-merge = Slå sammen FOMOD…
menu-export = Eksporter distribusjonsarkiv …

# Menu - Options
menu-options = Alternativer
menu-settings = Innstillinger
menu-pre-save-script = Skript før lagring…
menu-post-save-script = Skript etter lagring…
menu-translation = Oversettelse…

# Menu - Help
menu-help = Hjelp
menu-about = Om

# Tabs
tab-info = Mod-info
tab-steps = Installasjonstrinn
tab-required = Obligatoriske installasjoner
tab-conditional = Betingede installasjoner

# Info Tab
label-workspace = Arbeidsområde
label-root-dir = Rotmappe:
label-mod-name = Mod-navn:
label-author = Forfatter:
label-version = Versjon:
label-game-name = Spillnavn:
label-category = Kategori:
label-url = Nettsted-URL:
label-header-image = Toppbilde:
label-description = Beskrivelse:
placeholder-select-dir = (Velg en mappe)
placeholder-select-game = (Velg et spill)

# Steps Tab
label-step-name = Trinnavn:
label-group-name = Gruppenavn:
label-group-type = Gruppetype:
label-plugin-name = Plugin-navn:
label-plugin-desc = Beskrivelse:
label-plugin-type = Standardtype:
label-plugin-image = Bilde:
label-visibility = Synlighetsbetingelser
label-operator = Operator:

# Buttons
btn-browse = Bla gjennom…
btn-clear = Tøm
btn-add = Legg til
btn-remove = Fjern
btn-add-step = Nytt trinn
btn-delete-step = Slett trinn
btn-add-group = Legg til gruppe
btn-remove-group = Fjern gruppe
btn-add-plugin = Legg til plugin
btn-remove-plugin = Fjern plugin
btn-add-file = Legg til fil
btn-add-folder = Legg til mappe
btn-remove-file = Fjern
btn-add-flag = Legg til flagg
btn-remove-flag = Fjern flagg
btn-add-condition = Legg til betingelse
btn-remove-condition = Fjern betingelse
btn-add-dependency = Legg til avhengighet
btn-remove-dependency = Fjern avhengighet
btn-add-pattern = Nytt mønster
btn-remove-pattern = Slett mønster
btn-save = Lagre
btn-cancel = Avbryt
btn-ok = OK
btn-yes = Ja
btn-no = Nei

# Condition/Dependency Labels
label-flag-name = Flaggnavn:
label-flag-value = Verdi:
label-condition-type = Type:
label-condition-name = Navn:
label-condition-value = Verdi:
label-dep-type = Avhengighetstype:
label-dep-name = Navn/fil:
label-dep-value = Verdi/tilstand:

# Files
label-source = Kilde
label-destination = Mål
label-priority = Prioritet
label-file-type = Type
label-files = Filer
label-dependencies = Avhengigheter

# Settings Dialog
settings-title = Innstillinger
settings-tab-general = Generelt
settings-tab-recent-files = Nylige filer
settings-language = Språk:
settings-theme = Tema:
settings-font-size = Skriftstørrelse:
settings-replace-newlines = Behandle linjeskift i beskrivelser
settings-max-recent = Maks nylige filer:
settings-window-width = Vindusbredde:
settings-window-height = Vindushøyde:
settings-no-recent-files = Ingen nylige filer.

# Status messages for settings
status-settings-saved = Innstillingene ble lagret

# About Dialog
about-title = Om XIMOD Architect
about-description = Et plattformuavhengig verktøy for å lage FOMOD-installatører for Bethesda-spillmodder.
about-license = Lisensiert under MIT-lisensen
about-copyright = © 2024 XIMOD Team
about-credit = Rust-portering av det originale verktøyet av Wenderer:

# Script Dialog
script-title = Rediger skript
script-info = Skript kjøres før eller etter lagring. Du kan bruke følgende makroer:
script-macros = Tilgjengelige makroer:
macro-modname = $MODNAME$ - Mod-navn
macro-modauthor = $MODAUTHOR$ - Forfatternavn
macro-modversion = $MODVERSION$ - Mod-versjon
macro-modroot = $MODROOT$ - Sti til rotmappe
macro-date = $DATE$ - Gjeldende dato (ÅÅÅÅ-MM-DD)
macro-time = $TIME$ - Gjeldende tid (TT:MM:SS)
macro-random = $RANDOM$ - Tilfeldig tall

# Plugin Dependencies
label-default-type = Standardtype:
label-pattern-type = Mønstertype:
label-pattern-operator = Mønsteroperator:

# Conditional Files
label-pattern = Mønster

# Validation Messages
validation-no-name = Mod-navn er påkrevd
validation-no-steps = Minst ett trinn eller én obligatorisk fil kreves
validation-empty-step = Trinn { $num } har ikke noe navn
validation-empty-group = Trinn { $step }, gruppe { $group } har ikke noe navn
validation-no-plugins = Trinn { $step }, gruppe «{ $name }» har ingen plugins

# File States
state-active = Aktiv
state-inactive = Inaktiv
state-missing = Mangler

# Confirmation
confirm-title = Bekreftelse
confirm-delete = Er du sikker på at du vil slette dette elementet?
confirm-discard = Du har ulagrede endringer. Forkaste dem og fortsette?
confirm-unsaved = Du har ulagrede endringer. Vil du lagre før du lukker?
confirm-save-issues = Prosjektet har følgende problemer:
confirm-save-anyway = Lagre likevel?

# Errors
error-invalid-xml = Ugyldig XML-fil
error-parse-failed = Kunne ikke tolke FOMOD
error-write-failed = Kunne ikke skrive filen
error-create-dir = Kunne ikke opprette mappe

# Default names (generated when creating new items)
default-step-name = Trinn { $num }
default-group-name = Gruppe { $num }
default-plugin-name = Plugin { $num }
pattern-label = Mønster { $num }

# Selection prompts
msg-select-group-first = Velg først en gruppe.
msg-select-plugin-edit = Velg en plugin å redigere.
label-empty = (tom)
image-no-image = Ingen bilde

# File dialog filters
filter-images = Bilder
filter-xml = XML

# Dependency types
dep-type-flag = Flagg
dep-type-file = Fil

# Status bar
status-modified = Endret

# Status messages (errors)
msg-settings-save-error = Feil ved lagring av innstillinger
msg-script-save-error = Feil ved lagring av skript

# Translation editor
trans-title = Oversettelseseditor
trans-source-lang = Vist språk:
trans-target-lang = Språk å oversette:
trans-col-key = Nøkkel
trans-col-source = Etikett
trans-col-target = Oversettelse
trans-saved = Oversettelse lagret
trans-save-error = Feil ved lagring av oversettelse

# XML editor
xml-editor-title = XML-editor
xml-editor-edit = Rediger
xml-editor-apply = Bruk
xml-editor-revert = Avbryt
xml-editor-readonly = Skrivebeskyttet
xml-editor-editing = Redigerer — de grafiske fanene er låst
xml-editor-error = Feil:
xml-editor-applied = XML-endringene er tatt i bruk
xml-editor-wellformed = Velformet XML
xml-editor-error-at = Linje { $line }, kolonne { $col }: { $msg }

# Country / flag picker
settings-country-name = Landnavn:
settings-pick-country = Klikk for å velge landet ditt
flags-title = Velg et land
flags-filter = Filter:
flags-none = Fant ingen flagg

# Translation editor: country & font
trans-endonym = Landets endonym:
trans-font = Skrift:
trans-no-font = (ingen)
trans-browse = Bla gjennom …
trans-google-fonts = Google Fonts
trans-pick-country = Klikk for å velge landet
trans-font-outside = Skriften må først installeres i assets/fonts.
trans-font-dir-missing = Fant ikke mappen assets/fonts.

# Translation submission
trans-lang-endonym = Språkets endonym:
trans-author = Forfatter:
trans-submit = Send …
trans-submit-hint = Bygg en zip og åpne en forhåndsutfylt e-post
trans-data-updated = Referansedata oppdatert (Languages.json / Countries.json)
trans-package-ready = Arkiv klart:
trans-package-error = Kunne ikke bygge arkivet:

# ISO 639-3 requirement
trans-lang-not-iso = Oversettelse er bare mulig for et språk med en ISO 639-3-kode.

# FOMOD installer preview
menu-preview = Forhåndsvis installasjonsprogram …
preview-title = Forhåndsvisning av FOMOD-installasjonsprogram
preview-refresh = Oppdater
preview-assumptions = Filantakelser
preview-details = Detaljer
preview-back = Tilbake
preview-next = Neste
preview-install = Installer
preview-close = Lukk
preview-restart = Start på nytt
preview-summary-title = Filer som blir installert
preview-empty = Ingen fil ville blitt installert.
preview-none-option = (ingen)
preview-invalid = Fullfør de obligatoriske valgene for å fortsette.
preview-no-steps = Ingen trinn er synlig; se installasjonssammendraget.
preview-select-hint = Velg et alternativ for å se beskrivelsen.
preview-col-source = Kilde
preview-col-dest = Mål
preview-col-priority = Prioritet
preview-sel-exactlyone = Velg nøyaktig ett alternativ.
preview-sel-atmostone = Velg høyst ett alternativ.
preview-sel-any = Velg et vilkårlig antall alternativer.
preview-sel-all = Alle alternativene installeres.
preview-sel-atleastone = Velg minst ett alternativ.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Valider FOMOD
validate-report-title = FOMOD-validering
validate-ok = Ingen problemer funnet. FOMOD samsvarer med skjemaet.
xml-editor-schema-ok = Samsvarer med ModConfig 5.0-skjemaet.
xml-editor-schema-issues = Skjemaproblemer:
schema-line-col = Linje { $line }, kol. { $col }: { $msg }
schema-wrong-root = Uventet rot «{ $found }» (forventet «{ $expected }»).
schema-unknown = Uventet element «{ $element }» i «{ $parent }».
schema-missing = «{ $parent }» må inneholde «{ $child }».
schema-needs-one = «{ $parent }» må inneholde minst én «{ $child }».
schema-too-many = «{ $child }» kan bare forekomme én gang i «{ $parent }».
schema-missing-attr = Attributtet «{ $attr }» er påkrevd på «{ $element }».
schema-bad-enum = Ugyldig verdi «{ $value }» for { $element }/@{ $attr } (forventet: { $allowed }).
schema-choose-one = «{ $parent }» må inneholde nøyaktig én av: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Flytt før
reorder-after = Flytt etter

# Country / language database explorer (Properties)
menu-properties = Egenskaper …
prop-title = Database over land og språk
prop-tab-countries = Land
prop-tab-languages = Språk
prop-filter = Filter:
prop-official-langs = Offisielle språk
prop-spoken-langs = Talte språk
prop-endonym = Landets endonym
prop-font = Skrift
prop-spoken-in = Snakkes i
prop-select-country = Velg et land for å se detaljene.
prop-select-lang = Velg et språk for å se detaljene.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Åpne spillets Nexus Mods-side
