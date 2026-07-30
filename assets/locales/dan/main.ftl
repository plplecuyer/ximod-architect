# XIMOD Architect - translation metadata
# @language = dan
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Dansk
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Version { $version }

# Status messages
status-ready = Klar
msg-save-success = FOMOD blev gemt
msg-save-error = Fejl ved lagring af FOMOD
msg-export-success = Distributionsarkiv oprettet ({ $count } filer): { $path }
msg-export-error = Fejl ved oprettelse af distributionsarkivet: { $error }
msg-load-success = FOMOD blev indlæst
msg-load-error = Fejl ved indlæsning af FOMOD
msg-merge-success = FOMOD blev flettet
msg-merge-error = Fejl ved fletning af FOMOD
msg-no-root-selected = Vælg først en rodmappe
msg-no-fomod-folder = Ingen "fomod"-mappe fundet. Opret en?
msg-file-outside-root = Filen er uden for rodmappen

# Menu - File
menu-file = Filer
menu-new = Ny
menu-open = Åbn mappe…
menu-open-file = Åbn fil…
menu-save = Gem
menu-recent = Seneste
menu-exit = Afslut
menu-merge = Flet FOMOD…
menu-export = Eksportér distributionsarkiv…

# Menu - Options
menu-options = Indstillinger
menu-settings = Indstillinger
menu-pre-save-script = Script før lagring…
menu-post-save-script = Script efter lagring…
menu-translation = Oversættelse…

# Menu - Help
menu-help = Hjælp
menu-about = Om

# Tabs
tab-info = Mod-info
tab-steps = Installationstrin
tab-required = Påkrævede installationer
tab-conditional = Betingede installationer

# Info Tab
label-workspace = Arbejdsområde
label-root-dir = Rodmappe:
label-mod-name = Mod-navn:
label-author = Forfatter:
label-version = Version:
label-game-name = Spilnavn:
label-category = Kategori:
label-url = Websteds-URL:
label-header-image = Overskriftsbillede:
label-description = Beskrivelse:
placeholder-select-dir = (Vælg en mappe)
placeholder-select-game = (Vælg et spil)

# Steps Tab
label-step-name = Trinnavn:
label-group-name = Gruppenavn:
label-group-type = Gruppetype:
label-plugin-name = Plugin-navn:
label-plugin-desc = Beskrivelse:
label-plugin-type = Standardtype:
label-plugin-image = Billede:
label-visibility = Synlighedsbetingelser
label-operator = Operator:

# Buttons
btn-browse = Gennemse…
btn-clear = Ryd
btn-add = Tilføj
btn-remove = Fjern
btn-add-step = Nyt trin
btn-delete-step = Slet trin
btn-add-group = Tilføj gruppe
btn-remove-group = Fjern gruppe
btn-add-plugin = Tilføj plugin
btn-remove-plugin = Fjern plugin
btn-add-file = Tilføj fil
btn-add-folder = Tilføj mappe
btn-remove-file = Fjern
btn-add-flag = Tilføj flag
btn-remove-flag = Fjern flag
btn-add-condition = Tilføj betingelse
btn-remove-condition = Fjern betingelse
btn-add-dependency = Tilføj afhængighed
btn-remove-dependency = Fjern afhængighed
btn-add-pattern = Nyt mønster
btn-remove-pattern = Slet mønster
btn-save = Gem
btn-cancel = Annuller
btn-ok = OK
btn-yes = Ja
btn-no = Nej

# Condition/Dependency Labels
label-flag-name = Flagnavn:
label-flag-value = Værdi:
label-condition-type = Type:
label-condition-name = Navn:
label-condition-value = Værdi:
label-dep-type = Afhængighedstype:
label-dep-name = Navn/fil:
label-dep-value = Værdi/tilstand:

# Files
label-source = Kilde
label-destination = Destination
label-priority = Prioritet
label-file-type = Type
label-files = Filer
label-dependencies = Afhængigheder

# Settings Dialog
settings-title = Indstillinger
settings-tab-general = Generelt
settings-tab-recent-files = Seneste filer
settings-language = Sprog:
settings-theme = Tema:
settings-font-size = Skriftstørrelse:
settings-replace-newlines = Behandl linjeskift i beskrivelser
settings-max-recent = Maks. seneste filer:
settings-window-width = Vinduesbredde:
settings-window-height = Vindueshøjde:
settings-no-recent-files = Ingen seneste filer.

# Status messages for settings
status-settings-saved = Indstillingerne blev gemt

# About Dialog
about-title = Om XIMOD Architect
about-description = Et tværplatformsværktøj til oprettelse af FOMOD-installationsprogrammer til Bethesda-spilmods.
about-license = Licenseret under MIT-licensen
about-copyright = © 2024 XIMOD Team
about-credit = Rust port af det originale værktøj af Wenderer:

# Script Dialog
script-title = Rediger script
script-info = Scripts køres før eller efter lagring. Du kan bruge følgende makroer:
script-macros = Tilgængelige makroer:
macro-modname = $MODNAME$ - Mod-navn
macro-modauthor = $MODAUTHOR$ - Forfatternavn
macro-modversion = $MODVERSION$ - Mod-version
macro-modroot = $MODROOT$ - Sti til rodmappe
macro-date = $DATE$ - Aktuel dato (ÅÅÅÅ-MM-DD)
macro-time = $TIME$ - Aktuelt klokkeslæt (TT:MM:SS)
macro-random = $RANDOM$ - Tilfældigt tal

# Plugin Dependencies
label-default-type = Standardtype:
label-pattern-type = Mønstertype:
label-pattern-operator = Mønsteroperator:

# Conditional Files
label-pattern = Mønster

# Validation Messages
validation-no-name = Mod-navn er påkrævet
validation-no-steps = Der kræves mindst ét trin eller én påkrævet fil
validation-empty-step = Trin { $num } har intet navn
validation-empty-group = Trin { $step }, gruppe { $group } har intet navn
validation-no-plugins = Trin { $step }, gruppe "{ $name }" har ingen plugins

# File States
state-active = Aktiv
state-inactive = Inaktiv
state-missing = Mangler

# Confirmation
confirm-title = Bekræftelse
confirm-delete = Er du sikker på, at du vil slette dette element?
confirm-discard = Du har ugemte ændringer. Kassér dem og fortsæt?
confirm-unsaved = Du har ugemte ændringer. Vil du gemme før lukning?
confirm-save-issues = Projektet har følgende problemer:
confirm-save-anyway = Gem alligevel?

# Errors
error-invalid-xml = Ugyldig XML-fil
error-parse-failed = Kunne ikke fortolke FOMOD
error-write-failed = Kunne ikke skrive filen
error-create-dir = Kunne ikke oprette mappe

# Default names (generated when creating new items)
default-step-name = Trin { $num }
default-group-name = Gruppe { $num }
default-plugin-name = Plugin { $num }
pattern-label = Mønster { $num }

# Selection prompts
msg-select-group-first = Vælg først en gruppe.
msg-select-plugin-edit = Vælg et plugin, der skal redigeres.
label-empty = (tom)
image-no-image = Intet billede

# File dialog filters
filter-images = Billeder
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = Fil

# Status bar
status-modified = Ændret

# Status messages (errors)
msg-settings-save-error = Fejl ved lagring af indstillinger
msg-script-save-error = Fejl ved lagring af script

# Translation editor
trans-title = Oversættelseseditor
trans-source-lang = Vist sprog:
trans-target-lang = Sprog der skal oversættes:
trans-col-key = Nøgle
trans-col-source = Etiket
trans-col-target = Oversættelse
trans-saved = Oversættelse gemt
trans-save-error = Fejl ved lagring af oversættelse

# XML editor
xml-editor-title = XML-editor
xml-editor-edit = Rediger
xml-editor-apply = Anvend
xml-editor-revert = Annuller
xml-editor-readonly = Skrivebeskyttet
xml-editor-editing = Redigerer — grafiske faneblade er låst
xml-editor-error = Fejl:
xml-editor-applied = XML-ændringer anvendt
xml-editor-wellformed = Velformet XML
xml-editor-error-at = Linje { $line }, kolonne { $col }: { $msg }

# Country / flag picker
settings-country-name = Landenavn:
settings-pick-country = Klik for at vælge dit land
flags-title = Vælg et land
flags-filter = Filter:
flags-none = Intet flag fundet

# Translation editor: country & font
trans-endonym = Landets endonym:
trans-font = Skrifttype:
trans-no-font = (ingen)
trans-browse = Gennemse…
trans-google-fonts = Google Fonts
trans-pick-country = Klik for at vælge landet
trans-font-outside = Skrifttypen skal først installeres i assets/fonts.
trans-font-dir-missing = Mappen assets/fonts blev ikke fundet.

# Translation submission
trans-lang-endonym = Sprogets endonym:
trans-author = Forfatter:
trans-submit = Send…
trans-submit-hint = Byg en zip og åbn en forududfyldt e-mail
trans-data-updated = Referencedata opdateret (Languages.json / Countries.json)
trans-package-ready = Arkiv klar:
trans-package-error = Kunne ikke bygge arkivet:

# ISO 639-3 requirement
trans-lang-not-iso = Oversættelse er kun mulig for et sprog med en ISO 639-3-kode.

# FOMOD installer preview
menu-preview = Forhåndsvis installationsprogram…
preview-title = Forhåndsvisning af FOMOD-installationsprogram
preview-refresh = Opdater
preview-assumptions = Filantagelser
preview-details = Detaljer
preview-back = Tilbage
preview-next = Næste
preview-install = Installer
preview-close = Luk
preview-restart = Genstart
preview-summary-title = Filer der vil blive installeret
preview-empty = Ingen fil ville blive installeret.
preview-none-option = (ingen)
preview-invalid = Udfyld de påkrævede valg for at fortsætte.
preview-no-steps = Intet trin er synligt; se installationsoversigten.
preview-select-hint = Vælg en indstilling for at se dens beskrivelse.
preview-col-source = Kilde
preview-col-dest = Destination
preview-col-priority = Prioritet
preview-sel-exactlyone = Vælg præcis én indstilling.
preview-sel-atmostone = Vælg højst én indstilling.
preview-sel-any = Vælg et vilkårligt antal indstillinger.
preview-sel-all = Alle indstillinger installeres.
preview-sel-atleastone = Vælg mindst én indstilling.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Valider FOMOD
validate-report-title = FOMOD-validering
validate-ok = Intet problem fundet. FOMOD'en overholder skemaet.
xml-editor-schema-ok = Overholder ModConfig 5.0-skemaet.
xml-editor-schema-issues = Skemaproblemer:
schema-line-col = Linje { $line }, kol. { $col }: { $msg }
schema-wrong-root = Uventet rod "{ $found }" (forventet "{ $expected }").
schema-unknown = Uventet element "{ $element }" i "{ $parent }".
schema-missing = "{ $parent }" skal indeholde "{ $child }".
schema-needs-one = "{ $parent }" skal indeholde mindst én "{ $child }".
schema-too-many = "{ $child }" må kun forekomme én gang i "{ $parent }".
schema-missing-attr = Attributten "{ $attr }" er påkrævet på "{ $element }".
schema-bad-enum = Ugyldig værdi "{ $value }" for { $element }/@{ $attr } (forventet: { $allowed }).
schema-choose-one = "{ $parent }" skal indeholde præcis én af: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Flyt før
reorder-after = Flyt efter

# Country / language database explorer (Properties)
menu-properties = Egenskaber…
prop-title = Land-/sprogdatabase
prop-tab-countries = Lande
prop-tab-languages = Sprog
prop-filter = Filter:
prop-official-langs = Officielle sprog
prop-spoken-langs = Talte sprog
prop-endonym = Landets endonym
prop-font = Skrifttype
prop-spoken-in = Talt i
prop-select-country = Vælg et land for at se dets detaljer.
prop-select-lang = Vælg et sprog for at se dets detaljer.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Åbn spillets Nexus Mods-side
