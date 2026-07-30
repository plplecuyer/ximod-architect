# XIMOD Architect - translation metadata
# @language = nld
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Nederlands
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versie { $version }

# Status messages
status-ready = Gereed
msg-save-success = FOMOD is opgeslagen
msg-save-error = Fout bij het opslaan van FOMOD
msg-export-success = Distributiearchief aangemaakt ({ $count } bestanden): { $path }
msg-export-error = Fout bij het aanmaken van het distributiearchief: { $error }
msg-load-success = FOMOD is geladen
msg-load-error = Fout bij het laden van FOMOD
msg-merge-success = FOMOD succesvol samengevoegd
msg-merge-error = Fout bij het samenvoegen van FOMOD
msg-no-root-selected = Selecteer eerst een hoofdmap
msg-no-fomod-folder = Geen ‘fomod’-map gevonden. Er een maken?
msg-file-outside-root = Bestand bevindt zich buiten de hoofdmap

# Menu - File
menu-file = Bestand
menu-new = Nieuw
menu-open = Map openen…
menu-open-file = Bestand openen…
menu-save = Opslaan
menu-recent = Recent
menu-exit = Afsluiten
menu-merge = FOMOD samenvoegen…
menu-export = Distributiearchief exporteren...

# Menu - Options
menu-options = Opties
menu-settings = Instellingen
menu-pre-save-script = Script vóór opslaan…
menu-post-save-script = Script na opslaan…
menu-translation = Vertaling...

# Menu - Help
menu-help = Help
menu-about = Over

# Tabs
tab-info = Mod-info
tab-steps = Installatiestappen
tab-required = Vereiste installaties
tab-conditional = Voorwaardelijke installaties

# Info Tab
label-workspace = Werkruimte
label-root-dir = Hoofdmap:
label-mod-name = Modnaam:
label-author = Auteur:
label-version = Versie:
label-game-name = Spelnaam:
label-category = Categorie:
label-url = Website-URL:
label-header-image = Kopafbeelding:
label-description = Beschrijving:
placeholder-select-dir = (Selecteer een map)
placeholder-select-game = (Selecteer een spel)

# Steps Tab
label-step-name = Stapnaam:
label-group-name = Groepsnaam:
label-group-type = Groepstype:
label-plugin-name = Pluginnaam:
label-plugin-desc = Beschrijving:
label-plugin-type = Standaardtype:
label-plugin-image = Afbeelding:
label-visibility = Zichtbaarheidsvoorwaarden
label-operator = Operator:

# Buttons
btn-browse = Bladeren…
btn-clear = Wissen
btn-add = Toevoegen
btn-remove = Verwijderen
btn-add-step = Nieuwe stap
btn-delete-step = Stap verwijderen
btn-add-group = Groep toevoegen
btn-remove-group = Groep verwijderen
btn-add-plugin = Plugin toevoegen
btn-remove-plugin = Plugin verwijderen
btn-add-file = Bestand toevoegen
btn-add-folder = Map toevoegen
btn-remove-file = Verwijderen
btn-add-flag = Vlag toevoegen
btn-remove-flag = Vlag verwijderen
btn-add-condition = Voorwaarde toevoegen
btn-remove-condition = Voorwaarde verwijderen
btn-add-dependency = Afhankelijkheid toevoegen
btn-remove-dependency = Afhankelijkheid verwijderen
btn-add-pattern = Nieuw patroon
btn-remove-pattern = Patroon verwijderen
btn-save = Opslaan
btn-cancel = Annuleren
btn-ok = OK
btn-yes = Ja
btn-no = Nee

# Condition/Dependency Labels
label-flag-name = Vlagnaam:
label-flag-value = Waarde:
label-condition-type = Type:
label-condition-name = Naam:
label-condition-value = Waarde:
label-dep-type = Afhankelijkheidstype:
label-dep-name = Naam/bestand:
label-dep-value = Waarde/status:

# Files
label-source = Bron
label-destination = Bestemming
label-priority = Prioriteit
label-file-type = Type
label-files = Bestanden
label-dependencies = Afhankelijkheden

# Settings Dialog
settings-title = Instellingen
settings-tab-general = Algemeen
settings-tab-recent-files = Recente bestanden
settings-language = Taal:
settings-theme = Thema:
settings-font-size = Lettergrootte:
settings-replace-newlines = Nieuwe regels in beschrijvingen verwerken
settings-max-recent = Max. recente bestanden:
settings-window-width = Vensterbreedte:
settings-window-height = Vensterhoogte:
settings-no-recent-files = Geen recente bestanden.

# Status messages for settings
status-settings-saved = Instellingen zijn opgeslagen

# About Dialog
about-title = Over XIMOD Architect
about-description = Een platformonafhankelijk hulpmiddel om FOMOD-installatieprogramma’s voor mods van Bethesda-spellen te maken.
about-license = Gelicentieerd onder de MIT-licentie
about-copyright = © 2024 XIMOD Team
about-credit = Rust-port van de oorspronkelijke tool door Wenderer:

# Script Dialog
script-title = Script bewerken
script-info = Scripts worden vóór of na het opslaan uitgevoerd. U kunt de volgende macro’s gebruiken:
script-macros = Beschikbare macro’s:
macro-modname = $MODNAME$ - Modnaam
macro-modauthor = $MODAUTHOR$ - Auteursnaam
macro-modversion = $MODVERSION$ - Modversie
macro-modroot = $MODROOT$ - Pad naar hoofdmap
macro-date = $DATE$ - Huidige datum (JJJJ-MM-DD)
macro-time = $TIME$ - Huidige tijd (UU:MM:SS)
macro-random = $RANDOM$ - Willekeurig getal

# Plugin Dependencies
label-default-type = Standaardtype:
label-pattern-type = Patroontype:
label-pattern-operator = Patroonoperator:

# Conditional Files
label-pattern = Patroon

# Validation Messages
validation-no-name = Modnaam is vereist
validation-no-steps = Er is minstens één stap of vereist bestand nodig
validation-empty-step = Stap { $num } heeft geen naam
validation-empty-group = Stap { $step }, groep { $group } heeft geen naam
validation-no-plugins = Stap { $step }, groep ‘{ $name }’ heeft geen plugins

# File States
state-active = Actief
state-inactive = Inactief
state-missing = Ontbreekt

# Confirmation
confirm-title = Bevestiging
confirm-delete = Weet u zeker dat u dit item wilt verwijderen?
confirm-discard = U hebt niet-opgeslagen wijzigingen. Negeren en doorgaan?
confirm-unsaved = U hebt niet-opgeslagen wijzigingen. Wilt u opslaan voordat u sluit?
confirm-save-issues = Het project heeft de volgende problemen:
confirm-save-anyway = Toch opslaan?

# Errors
error-invalid-xml = Ongeldig XML-bestand
error-parse-failed = Kan FOMOD niet verwerken
error-write-failed = Kan bestand niet schrijven
error-create-dir = Kan map niet maken

# Default names (generated when creating new items)
default-step-name = Stap { $num }
default-group-name = Groep { $num }
default-plugin-name = Plugin { $num }
pattern-label = Patroon { $num }

# Selection prompts
msg-select-group-first = Selecteer eerst een groep.
msg-select-plugin-edit = Selecteer een plugin om te bewerken.
label-empty = (leeg)
image-no-image = Geen afbeelding

# File dialog filters
filter-images = Afbeeldingen
filter-xml = XML

# Dependency types
dep-type-flag = Vlag
dep-type-file = Bestand

# Status bar
status-modified = Gewijzigd

# Status messages (errors)
msg-settings-save-error = Fout bij het opslaan van instellingen
msg-script-save-error = Fout bij het opslaan van script

# Translation editor
trans-title = Vertaaleditor
trans-source-lang = Weergegeven taal:
trans-target-lang = Te vertalen taal:
trans-col-key = Sleutel
trans-col-source = Label
trans-col-target = Vertaling
trans-saved = Vertaling opgeslagen
trans-save-error = Fout bij het opslaan van de vertaling

# XML editor
xml-editor-title = XML-editor
xml-editor-edit = Bewerken
xml-editor-apply = Toepassen
xml-editor-revert = Annuleren
xml-editor-readonly = Alleen-lezen
xml-editor-editing = Bewerken — grafische tabbladen zijn vergrendeld
xml-editor-error = Fout:
xml-editor-applied = XML-wijzigingen toegepast
xml-editor-wellformed = Correct opgemaakte XML
xml-editor-error-at = Regel { $line }, kolom { $col }: { $msg }

# Country / flag picker
settings-country-name = Landnaam:
settings-pick-country = Klik om je land te kiezen
flags-title = Kies een land
flags-filter = Filter:
flags-none = Geen vlag gevonden

# Translation editor: country & font
trans-endonym = Endoniem van het land:
trans-font = Lettertype:
trans-no-font = (geen)
trans-browse = Bladeren…
trans-google-fonts = Google Fonts
trans-pick-country = Klik om het land te kiezen
trans-font-outside = Het lettertype moet eerst in assets/fonts geïnstalleerd zijn.
trans-font-dir-missing = De map assets/fonts is niet gevonden.

# Translation submission
trans-lang-endonym = Endoniem van de taal:
trans-author = Auteur:
trans-submit = Verzenden…
trans-submit-hint = Bouw een zip en open een vooraf ingevulde e-mail
trans-data-updated = Referentiegegevens bijgewerkt (Languages.json / Countries.json)
trans-package-ready = Archief gereed:
trans-package-error = Kon het archief niet bouwen:

# ISO 639-3 requirement
trans-lang-not-iso = Vertaling is alleen mogelijk voor een taal met een ISO 639-3-code.

# FOMOD installer preview
menu-preview = Installatievoorbeeld…
preview-title = Voorbeeld van FOMOD-installatieprogramma
preview-refresh = Vernieuwen
preview-assumptions = Bestandsaannames
preview-details = Details
preview-back = Terug
preview-next = Volgende
preview-install = Installeren
preview-close = Sluiten
preview-restart = Opnieuw starten
preview-summary-title = Bestanden die geïnstalleerd worden
preview-empty = Er zou geen bestand geïnstalleerd worden.
preview-none-option = (geen)
preview-invalid = Vul de vereiste keuzes in om door te gaan.
preview-no-steps = Er is geen stap zichtbaar; zie het installatieoverzicht.
preview-select-hint = Selecteer een optie om de beschrijving te zien.
preview-col-source = Bron
preview-col-dest = Bestemming
preview-col-priority = Prioriteit
preview-sel-exactlyone = Kies precies één optie.
preview-sel-atmostone = Kies hoogstens één optie.
preview-sel-any = Kies een willekeurig aantal opties.
preview-sel-all = Alle opties worden geïnstalleerd.
preview-sel-atleastone = Kies ten minste één optie.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = FOMOD valideren
validate-report-title = FOMOD-validatie
validate-ok = Geen probleem gevonden. De FOMOD voldoet aan het schema.
xml-editor-schema-ok = Voldoet aan het ModConfig 5.0-schema.
xml-editor-schema-issues = Schemaproblemen:
schema-line-col = Regel { $line }, kol. { $col }: { $msg }
schema-wrong-root = Onverwachte root "{ $found }" (verwacht "{ $expected }").
schema-unknown = Onverwacht element "{ $element }" in "{ $parent }".
schema-missing = "{ $parent }" moet "{ $child }" bevatten.
schema-needs-one = "{ $parent }" moet ten minste één "{ $child }" bevatten.
schema-too-many = "{ $child }" mag slechts eenmaal voorkomen in "{ $parent }".
schema-missing-attr = Attribuut "{ $attr }" is vereist op "{ $element }".
schema-bad-enum = Ongeldige waarde "{ $value }" voor { $element }/@{ $attr } (verwacht: { $allowed }).
schema-choose-one = "{ $parent }" moet precies één van de volgende bevatten: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Ervoor plaatsen
reorder-after = Erna plaatsen

# Country / language database explorer (Properties)
menu-properties = Eigenschappen…
prop-title = Land-/taaldatabase
prop-tab-countries = Landen
prop-tab-languages = Talen
prop-filter = Filter:
prop-official-langs = Officiële talen
prop-spoken-langs = Gesproken talen
prop-endonym = Endoniem van het land
prop-font = Lettertype
prop-spoken-in = Gesproken in
prop-select-country = Selecteer een land om de details te zien.
prop-select-lang = Selecteer een taal om de details te zien.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Open de Nexus Mods-pagina van het spel
