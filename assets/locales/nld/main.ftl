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
menu-export = Distributiearchief exporteren…
# Menu - Options
menu-options = Opties
menu-settings = Instellingen…
menu-pre-save-script = Script vóór opslaan…
menu-post-save-script = Script na opslaan…
menu-translation = Interface vertalen…
# Menu - Help
menu-help = Help
menu-check-updates = Controleren op updates…
menu-about = Over

# Update check
update-checking = Controleren op updates…
update-up-to-date = XIMOD Architect is up-to-date.
update-check-failed = Kon niet controleren op updates. Probeer het later opnieuw.
update-available-status = Versie { $version } is beschikbaar.
update-banner-text = XIMOD Architect { $version } is beschikbaar.
update-download = Downloaden:
update-skip = Deze versie overslaan
update-later = Later

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
label-plugin-name = Optienaam:
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
btn-add-plugin = Optie toevoegen
btn-remove-plugin = Optie verwijderen
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Bestemming voor de hele groep
label-page-dest = Installatiebestemming (hele pagina)
btn-apply-group-dest = Toepassen op alle opties in deze groep
btn-apply-page-dest = Toepassen op alle opties op deze pagina
group-dest-hint = Stelt één installatiebestemming in voor elk bestand van elke optie in deze groep.
page-dest-hint = Stelt één installatiebestemming in voor elk bestand van elke optie op deze pagina (alle groepen).
bulk-dest-nofiles = Nog geen bestanden om bij te werken — voeg eerst bestanden toe aan de opties.
status-dest-applied = Bestemming toegepast op { $num } bestand(en).
preview-hidden-steps = { $num } stap(pen) verborgen door huidige selectie.
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
settings-check-updates = Bij het opstarten controleren op updates
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
label-plugin-dependencies = Optie-afhankelijkheden
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
validation-no-plugins = Stap { $step }, groep ‘{ $name }’ heeft geen opties

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
default-plugin-name = Optie { $num }
pattern-label = Patroon { $num }

# Selection prompts
msg-select-group-first = Selecteer eerst een groep.
msg-select-plugin-edit = Selecteer een optie om te bewerken.
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

# Referenced-file verification (V2)
verify-no-root = Bestandscontrole overgeslagen: geen hoofdmap ingesteld
loc-header = headerafbeelding
loc-required = vereiste bestanden
loc-conditional = voorwaardelijke set { $num }
loc-plugin = stap { $step }, groep { $group }, optie "{ $plugin }"
verify-missing-file = Ontbrekend bestand: { $path } ({ $loc })
verify-missing-folder = Ontbrekende map: { $path } ({ $loc })
verify-missing-image = Ontbrekende afbeelding: { $path } ({ $loc })
verify-absolute = Absoluut pad (niet overdraagbaar): { $path } ({ $loc })
verify-outside = Pad verlaat de hoofdmap: { $path } ({ $loc })
verify-orphan = Weesbestand (door geen enkele optie gebruikt): { $path }
conflict-certain = Bestemmingsconflict: “{ $path }” wordt door { $count } opties ({ $locs }) geschreven — ze overschrijven elkaar.
conflict-potential = Mogelijk bestemmingsconflict: “{ $path }” is doel van { $count } verwijzingen ({ $locs }) — overschrijven hangt af van de selectie/voorwaarden.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = FOMOD sluiten
menu-close-all-fomods = Alle FOMODs sluiten
tab-untitled = (naamloos)
msg-drop-not-fomod = Het neergezette item is geen FOMOD (geen map "fomod" gevonden)
exit-title = Niet-opgeslagen wijzigingen
exit-unsaved = Een FOMOD is niet opgeslagen. Wilt u het opslaan?
tab-close-hint = Deze FOMOD sluiten
menu-new-from-folder = Nieuw vanuit map…
menu-templates = Sjablonen…
templates-title = Herbruikbare sjablonen
templates-empty = Nog geen sjablonen opgeslagen. Sla hierboven de geselecteerde stap op om er een te maken.
templates-insert = Invoegen
templates-save-step = Geselecteerde stap opslaan
templates-name-hint = Sjabloonnaam (optioneel)
msg-wizard-success = Structuur gemaakt vanuit map: { $num } optie(s).
msg-wizard-error = Fout: { $error }
msg-template-saved = Sjabloon opgeslagen: { $name }
msg-template-inserted = Sjabloon in het project ingevoegd.
msg-template-no-step = Selecteer eerst een stap om deze als sjabloon op te slaan.
msg-template-no-dir = Kon de sjablonenmap niet vinden.
msg-drop-assigned = { $added } bron(nen) toegevoegd aan de optie ({ $rejected } buiten de hoofdmap genegeerd).
menu-compare = Vergelijken met…
compare-title = FOMOD-vergelijking
compare-none = Geen verschillen.
btn-optimize-image = Afbeelding optimaliseren
msg-image-optimized = Kopafbeelding geoptimaliseerd.
msg-image-ok = Kopafbeelding valt al binnen de limieten.
msg-no-header-image = Geen kopafbeelding om te optimaliseren.
verify-image-large = Afbeelding te groot ({ $width }×{ $height }): { $path }
verify-image-format = Niet-ondersteund afbeeldingsformaat (.{ $ext }): { $path }
verify-image-unreadable = Onleesbare afbeelding: { $path }
menu-condition-editor = Voorwaarden-editor…
condeditor-title = Voorwaarden-editor
condeditor-set-by = Ingesteld door:
condeditor-used-by = Gebruikt door:
condeditor-filedeps = Bestandsafhankelijkheden
condeditor-empty = Geen vlaggen of afhankelijkheden in dit project.
condeditor-orphan-set = ingesteld maar nooit gebruikt
condeditor-orphan-used = gebruikt maar nooit ingesteld
msg-img-optimized = Afbeelding geoptimaliseerd.
msg-img-ok = Afbeelding valt al binnen de limieten.
msg-img-none = Geen afbeelding om te optimaliseren.
msg-crash-recovery = De vorige sessie is onverwacht beëindigd. Een back-up van uw project is opgeslagen in { $path }
export-progress-title = Distributiearchief wordt aangemaakt…
export-progress-files = { $done } / { $total } bestanden
msg-export-cancelled = Export geannuleerd; het onvolledige archief is verwijderd.
verify-running = Bestanden op schijf worden gecontroleerd…
verify-stale = Opmerking: het project is gewijzigd tijdens de bestandscontrole; voer de validatie opnieuw uit.
prop-col-name = Naam
menu-save-as = Opslaan als…
menu-project = Project
menu-tools = Extra
menu-manual = Gebruikershandleiding
msg-manual-missing = De gebruikershandleiding (PDF) is niet gevonden naast de toepassing.
toolbar-new = Nieuw
toolbar-open = Openen
toolbar-save = Opslaan
toolbar-validate = Valideren
toolbar-preview = Voorbeeld
toolbar-export = Exporteren
dialog-choose-root = Kies de hoofdmap van de mod
exit-unsaved-docs = Niet opgeslagen: { $names }
status-summary = { $steps } stappen · { $options } opties
section-groups = Groepen
section-options = Opties
section-flags = Voorwaardevlaggen
section-files = Te installeren bestanden
hint-group-type = Hoe het installatieprogramma de gebruiker opties in deze groep laat kiezen.
hint-default-type = Hoe de optie wordt aangeboden als geen van de afhankelijkheidspatronen past: vereist, optioneel, aanbevolen, niet bruikbaar…
hint-operator = Alle voorwaarden moeten waar zijn (EN), of één ervan volstaat (OF).
hint-flags = Vlaggen zijn benoemde waarden die deze optie instelt wanneer ze wordt gekozen. Andere stappen en opties kunnen ze testen om zichzelf te tonen, te verbergen of verplicht te maken.
hint-plugin-dependencies = Patronen die het type van de optie wijzigen op basis van vlaggen of bestanden in het spel: bijvoorbeeld “Vereist” als een andere mod geïnstalleerd is.
hint-files = Bestanden en mappen die naar de Data-map van het spel worden gekopieerd wanneer deze optie is gekozen. De bestemming is relatief aan Data; bij een conflict wint de hoogste prioriteit.
hint-visibility = Voorwaarden waaraan moet worden voldaan om deze stap überhaupt te tonen. Laat leeg om hem altijd te tonen.
seltype-exactly-one = Precies één (verplicht)
seltype-at-most-one = Hoogstens één
seltype-any = Willekeurig aantal
seltype-all = Alle (geen keuze)
seltype-at-least-one = Minstens één
plugtype-required = Vereist
plugtype-optional = Optioneel
plugtype-recommended = Aanbevolen
plugtype-not-usable = Niet bruikbaar
plugtype-could-be-usable = Mogelijk bruikbaar
plugtype-required-hint = Wordt altijd geïnstalleerd; de gebruiker kan het niet uitvinken.
plugtype-optional-hint = Aangeboden zonder vinkje; de gebruiker beslist.
plugtype-recommended-hint = Aangeboden met vinkje; de gebruiker kan het uitvinken.
plugtype-not-usable-hint = Grijs weergegeven en niet selecteerbaar.
plugtype-could-be-usable-hint = Selecteerbaar, maar het installatieprogramma waarschuwt dat het mogelijk niet werkt.
op-and = Alle voorwaarden (EN)
op-or = Een willekeurige voorwaarde (OF)
theme-dark = Donker
theme-light = Licht
theme-system = Systeem volgen
condeditor-setter-loc = Stap { "{step}" } / Groep { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Patroon van «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Zichtbaarheid van stap { "{step}" }
condeditor-cond-set = Voorwaardelijke set { "{num}" }
condeditor-needs = { "{ctx}" } (vereist = { "{value}" })
condeditor-file-dep = { "{ctx}" }: bestand «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = FOMOD vertalen…
ftr-title = FOMOD vertalen
ftr-open-folder = Modmap openen…
ftr-from-active = Vanuit het actieve project
ftr-from-active-hint = Vertaalt de FOMOD van het project dat in het hoofdvenster is geopend (het moet eerst worden opgeslagen).
ftr-no-fomod = Geen FOMOD geladen.
ftr-encoding = Codering van de oorspronkelijke bestanden; de vertaalde bestanden worden met dezelfde codering geschreven.
ftr-source-lang = Van
ftr-target-lang = naar
ftr-lang-locked = (de talen liggen vast zodra een FOMOD is geladen)
ftr-translator = Vertaler:
ftr-save = Vertaling opslaan
ftr-export = Vertaalde bestanden exporteren
ftr-export-sibling = Naar een map fomod_<taal>
ftr-export-sibling-hint = Schrijft de vertaalde info.xml en ModuleConfig.xml naast de oorspronkelijke fomod-map; de oorspronkelijke bestanden blijven ongemoeid.
ftr-export-inplace = Over de oorspronkelijke bestanden heen
ftr-export-inplace-hint = Vervangt fomod/info.xml en fomod/ModuleConfig.xml nadat van elk een .bak-kopie met tijdstempel is gemaakt.
ftr-force-explicit-order = Oorspronkelijke volgorde behouden
ftr-warn-order = Lijsten die op naam zijn gesorteerd (order="Ascending") zouden door de modmanager opnieuw worden gesorteerd op de vertaalde namen. Deze optie dwingt order="Explicit" af, zodat de opties hun huidige volgorde behouden.
ftr-update = Bijwerken vanuit map
ftr-update-hint = Leest de FOMOD opnieuw van schijf en voegt de vertaling ermee samen: nieuwe, gewijzigde en verwijderde tekenreeksen worden gemeld.
ftr-preview-translated = Vertaald voorbeeld
ftr-progress = { $done } / { $total } vertaald
ftr-filter-all = Alle
ftr-filter-untranslated = Niet vertaald
ftr-filter-review = Te controleren
ftr-filter-issues = Met problemen
ftr-filter-locked = Vergrendeld
ftr-type-all = Alle velden
ftr-type-names = Namen
ftr-type-descriptions = Beschrijvingen
ftr-type-meta = Modinformatie
ftr-search-hint = Zoeken in bron, vertaling of context…
ftr-next-untranslated = Volgende niet-vertaalde
ftr-show-whitespace = Spaties en regeleinden tonen
ftr-discard-question = De huidige vertaling bevat niet-opgeslagen wijzigingen. Deze negeren en de andere FOMOD laden?
ftr-discard-yes = Negeren
ftr-unsaved-close = De vertaling bevat niet-opgeslagen wijzigingen.
ftr-col-num = Nr.
ftr-col-status = { "" }
ftr-col-context = Context
ftr-col-source = Bron
ftr-col-target = Vertaling
ftr-col-issues = { "" }
ftr-empty-hint = Open een modmap of laad het actieve project om de vertaalbare tekenreeksen te tonen.
ftr-empty-filter = Geen tekenreeks komt overeen met het huidige filter.
ftr-select-row = Selecteer een rij om de vertaling te bewerken.
ftr-copy-source = Bron kopiëren
ftr-clear-target = Wissen
ftr-lock = Niet vertalen
ftr-lock-hint = Vergrendelde tekenreeksen worden ongewijzigd geschreven (auteur, website, eigennamen…).
ftr-note = Notitie:
ftr-status-untranslated = Niet vertaald
ftr-status-translated = Vertaald
ftr-status-auto = Automatisch vooraf ingevuld — graag controleren
ftr-status-fuzzy = De brontekst is gewijzigd sinds de vertaling — graag controleren
ftr-status-obsolete = Niet meer aanwezig in de FOMOD
ftr-status-locked = Vergrendeld (wordt ongewijzigd geschreven)
ftr-field-info-name = Modnaam (info.xml)
ftr-field-module-name = Titel van het installatieprogramma (ModuleConfig.xml)
ftr-field-author = Auteur
ftr-field-website = Website
ftr-field-description = Modbeschrijving
ftr-field-step = Stapnaam
ftr-field-group = Groepsnaam
ftr-field-plugin = Optienaam
ftr-field-plugin-desc = Optiebeschrijving
ftr-issue-empty = Lege vertaling
ftr-issue-whitespace = De vertaling bevat alleen spaties
ftr-issue-edge-whitespace = Spaties aan het begin of einde wijken af van de bron
ftr-issue-token = Beschermde tokens wijken af — ontbrekend: { $missing } ; te veel: { $extra }
ftr-issue-newline-name = Een naam mag geen regeleinde bevatten
ftr-issue-control = Bevat tekens die XML niet kan opslaan
ftr-issue-length = Ongebruikelijke lengte vergeleken met de bron (×{ $ratio })
ftr-issue-identical = Identiek aan de bron
ftr-issue-duplicate = Dezelfde brontekst is anders vertaald in { $key }
ftr-issue-cdata = De reeks ]]> is hier niet toegestaan
ftr-load-error = Kan de FOMOD niet laden: { $error }
ftr-extracted = { $num } vertaalbare tekenreeksen gevonden.
ftr-sidecar-found = Bestaande vertaling geladen en samengevoegd: { $new } nieuw, { $changed } gewijzigd, { $removed } verwijderd.
ftr-saved = Vertaling opgeslagen in { $path }
ftr-save-error = Kan de vertaling niet opslaan: { $error }
ftr-save-first = Sla eerst het project op en vertaal het daarna.
ftr-export-success = { $count } tekenreeksen geschreven naar { $path }
ftr-export-error = Exporteren mislukt: { $error }
ftr-export-blocked = { $num } blokkerende problemen moeten vóór het exporteren worden opgelost.
ftr-export-stale = { $num } tekenreeksen zijn overgeslagen omdat de FOMOD is gewijzigd; gebruik “Bijwerken vanuit map”.
ftr-update-report = Bijgewerkt: { $new } nieuw, { $changed } gewijzigd, { $moved } verplaatst, { $removed } verwijderd, { $unchanged } ongewijzigd.
menu-edit = Bewerken
menu-undo = Ongedaan maken
menu-redo = Opnieuw
tree-title = Project
tree-mod-info = Mod-informatie
tree-steps = Installatiestappen
tree-required = Vereiste bestanden
tree-conditional = Voorwaardelijke installaties
tree-empty-steps = Nog geen stap — klik op + om er een toe te voegen.
tree-duplicate = Dupliceren
tree-delete = Verwijderen
tree-save-template = Opslaan als sjabloon…
tree-drop-hint = Hier neerzetten om te verplaatsen
cond-set-label = Voorwaardelijke set { $num }
inspector-empty = Selecteer een item in de projectboom of voeg een stap toe om te beginnen.
count-options = { $num } opties
count-files = { $num } bestanden
msg-deleted-undo = Verwijderd. Gebruik Ongedaan maken (Ctrl+Z) om het te herstellen.
problems-title = Problemen
problems-errors = { $num } fouten
problems-warnings = { $num } waarschuwingen
btn-close = Sluiten
ftr-export-package = Als vertaalpakket (archief)
ftr-export-package-hint = Maakt een .zip of .7z die klaar is om te uploaden: de vertaalde info.xml en ModuleConfig.xml plus een README (alleen patch), of de volledige mod met de vertaalde bestanden (volledig).
ftr-package-full = Volledige mod
ftr-package-full-hint = Neem alle bestanden van de mod op in het archief, niet alleen de twee vertaalde XML-bestanden. Controleer of de auteur herdistributie toestaat.
ftr-package-name-template = Naam:
ftr-readme-patch = Dit archief bevat de vertaling ({ $langname }) van het installatieprogramma van “{ $name }” (fomod/info.xml en fomod/ModuleConfig.xml). Installeer het over de oorspronkelijke mod heen, of laat je modmanager het samenvoegen, zodat de vertaalde bestanden de oorspronkelijke vervangen. Alleen de teksten van het installatieprogramma veranderen; de bestanden van de mod zelf zijn niet inbegrepen. Gemaakt met XIMOD Architect.
ftr-readme-full = Dit archief bevat “{ $name }” met een vertaald installatieprogramma ({ $langname }; fomod/info.xml en fomod/ModuleConfig.xml). Installeer het zoals de oorspronkelijke mod. Alleen de teksten van het installatieprogramma zijn gewijzigd. Gemaakt met XIMOD Architect.
ftr-apply-memory = Invullen vanuit geheugen
ftr-memory-size = Vertaalgeheugen: { $num } items voor dit talenpaar. Elke opgeslagen vertaling wordt eraan toegevoegd.
ftr-memory-applied = { $num } tekenreeksen ingevuld vanuit het vertaalgeheugen (gemarkeerd als “te controleren”).
ftr-memory-suggestion = Het geheugen stelt voor:
ftr-use-suggestion = Gebruiken
ftr-propagate = Doorvoeren naar identieke
ftr-propagate-hint = Kopieer deze vertaling naar elke andere, nog niet vertaalde tekenreeks met dezelfde brontekst.
ftr-propagated = { $num } identieke tekenreeksen ingevuld.
ftr-csv-export = CSV exporteren…
ftr-csv-import = CSV importeren…
ftr-csv-imported = { $num } tekenreeksen bijgewerkt vanuit het CSV-bestand.
ftr-csv-error = CSV-fout: { $error }
ftr-glossary = Woordenlijst
ftr-glossary-source = Term
ftr-glossary-target = Vertaling
ftr-glossary-case = Hoofdlettergebruik
ftr-glossary-dnt = Behouden
ftr-glossary-add = Term toevoegen
ftr-issue-glossary = Woordenlijst: “{ $term }” is niet vertaald zoals verwacht

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Archief openen…
filter-archive = Mod-archieven (zip, 7z)
msg-archive-opened = Archief geopend ({ $num } bestanden uitgepakt): { $path }
msg-archive-reused = Archief is al uitgepakt, { $path } wordt hergebruikt
msg-archive-unsupported = Archiefformaat “.{ $ext }” wordt niet ondersteund; pak het eerst uit met 7-Zip (alleen .zip en .7z kunnen worden geopend).
msg-archive-error = Fout bij het openen van het archief: { $error }
msg-archive-no-fomod = Geen map “fomod” gevonden in het archief ({ $path })
msg-archive-extracting = Archief wordt uitgepakt…
ftr-open-archive = Een mod-archief openen…
ftr-package-full-partial = De mod is geopend vanuit een archief met alleen de fomod-map; volledige pakketten vereisen de uitgepakte mod.
info-module-deps = Mod-vereisten
info-module-deps-hint = Bestanden of vlaggen die de hele mod vereist voordat het installatieprogramma start (moduleDependencies). Laat leeg als er geen zijn.
info-header-advanced = Geavanceerde koptekst
info-title-position = Titelpositie
info-title-colour = Titelkleur
info-title-colour-hint = Verwacht: zes hexadecimale cijfers (RRGGBB)
info-image-show = Kopafbeelding tonen
info-image-fade = Kopafbeelding vervagen
info-image-height = Hoogte kopafbeelding
info-attr-default = (standaard)
file-always-install = Altijd
file-always-install-hint = Dit bestand altijd installeren, ook als de optie niet is geselecteerd (alwaysInstall).
file-install-if-usable = Als bruikbaar
file-install-if-usable-hint = Dit bestand installeren zodra de optie bruikbaar is, ook als deze niet is geselecteerd (installIfUsable).
msg-import-lossy = Deze FOMOD bevat { $num } constructies die XIMOD niet kan bewerken; ze gaan verloren wanneer het project wordt opgeslagen.
fidelity-nested-deps = Geneste afhankelijkheidsgroep in { $context } (slechts één niveau wordt ondersteund)
fidelity-game-dep = Vereiste spelversie { $version } in { $context }
fidelity-fomm-dep = Vereiste mod-manager-versie { $version } in { $context }
fidelity-unknown = Element “{ $element }” in “{ $parent }” wordt niet ondersteund ({ $context })
loc-module = de mod-vereisten
loc-step = stap { $step } “{ $name }”
loc-installer = het installatieprogramma

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Een back-up terugzetten…
backups-title = Een back-up terugzetten
backups-empty = Dit project heeft nog geen back-up. Er wordt er een gemaakt telkens wanneer het project over een vorige versie heen wordt opgeslagen.
backups-changes = { $num } wijziging(en) ten opzichte van het huidige project
btn-compare = Vergelijken
btn-restore = Terugzetten
btn-delete-backups = Alle back-ups verwijderen
btn-delete-backups-confirm = Klik nogmaals om alle back-ups te verwijderen
msg-backup-restored = Back-up van { $time } teruggezet in de editor (nog niet opgeslagen; Ongedaan maken draait dit terug)
msg-backups-deleted = { $num } back-up(s) verwijderd
settings-backup-count = Te bewaren back-ups:
settings-backup-count-hint = Aantal vorige versies van de FOMOD-XML dat bij het opslaan onder fomod/backups wordt bewaard (0 = geen back-ups).
settings-autosave-minutes = Herstelkopie automatisch opslaan elke (minuten):
settings-autosave-minutes-hint = Met dit interval wordt van elk gewijzigd project een herstelkopie in de configuratiemap geschreven; deze wordt bij de volgende start alleen aangeboden na een abnormale afsluiting (0 = uit).
settings-auto-masters = De masters van een plugin als voorwaarden toevoegen
settings-auto-masters-hint = Wanneer een plugin (.esp/.esm/.esl) aan een optie wordt toegevoegd, worden de masters die hij vereist en die noch het spel noch deze mod levert, “Active”-bestandsvoorwaarden van de optie.
msg-author-from-plugin = Auteur ingevuld vanuit de plugin-header: { $author }
msg-masters-added = { $num } master(s) van { $plugin } toegevoegd als bestandsvoorwaarde(n)
issue-missing-master = { $plugin } vereist { $master }, dat noch in deze mod zit, noch als afhankelijkheid is opgegeven
issue-esl-mismatch-flag = { $plugin } heeft de extensie .esl, maar de light-vlag (ESL) ervan is niet ingesteld
issue-esl-eligible = { $plugin } zou als light gemarkeerd kunnen worden ({ $num } nieuwe records, limiet { $limit })
issue-esl-too-big = { $plugin } is als light gemarkeerd, maar voldoet niet aan de regels voor light-plugins ({ $num } nieuwe records, limiet { $limit }, of een FormID buiten het toegestane bereik)
menu-plugin-report = Pluginrapport…
plugins-title = Pluginrapport
plugins-file = Bestand
plugins-kind = Soort
plugins-light = Light-vlag
plugins-masters = Masters
plugins-new-records = Nieuwe records / limiet
plugins-eligible = Light-geschikt
plugins-empty = Dit project installeert geen pluginbestand (.esp, .esm of .esl).
plugins-unreadable = onleesbaar

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Definitieve bestandsboom
preview-total-size = Totale installatiegrootte: { $size }
preview-tree-truncated = De boom is afgekapt: te veel bestanden om uit te vouwen (de groottes hierboven zijn onvolledig).
preview-overwritten-by = Overschreven door { $plugin }
preview-scenario = Scenario:
preview-scenario-load = Laden
preview-scenario-save = Opslaan…
preview-scenario-delete = Verwijderen
preview-scenario-name = Scenarionaam
preview-scenario-saved = Scenario ‘{ $name }’ opgeslagen onder fomod/scenarios
preview-scenario-unresolved = { $num } selectie(s) van het scenario komen met geen enkele optie van dit project overeen (hernoemd of verwijderd)
preview-scenario-none = (geen scenario)
issue-unreachable-step = Stap ‘{ $step }’ kan nooit worden getoond: de zichtbaarheidsvoorwaarden ervan testen een vlagwaarde die geen enkele eerdere optie instelt
issue-unreachable-option = Optie ‘{ $plugin }’ kan nooit worden geselecteerd: de patronen voor het bruikbare type ervan testen een vlagwaarde die geen enkele optie instelt
issue-unreachable-cond = Voorwaardelijke bestandsset { $num } kan nooit van toepassing zijn: de voorwaarden ervan testen een vlagwaarde die geen enkele optie instelt
size-option = Installatiegrootte: { $size } ({ $num } bestand(en))
size-missing = { $num } ontbrekende bron(nen)
size-unknown = Installatiegrootte: — (voer Valideren uit om te meten)
menu-nexus-desc = Nexus-beschrijving…
nexus-title = Nexus Mods-beschrijving
nexus-format = Indeling:
nexus-include-requirements = Vereisten
nexus-include-options = Installatieopties
nexus-include-install = Installatie
nexus-include-changelog = Wijzigingslogboek
nexus-previous = Vorige versie…
nexus-previous-none = (geen vorige versie: geen wijzigingslogboek)
nexus-language = Taal:
nexus-language-source = (bron)
nexus-sec-requirements = Vereisten
nexus-sec-options = Installatieopties
nexus-sec-install = Installatie
nexus-sec-changelog = Wijzigingslogboek
nexus-install-text = Deze mod wordt geleverd met een FOMOD-installatieprogramma: installeer hem met een modmanager (Vortex, Mod Organizer 2) en kies je opties in het installatieprogramma.
nexus-requires = Vereist
nexus-step = Stap
nexus-added = Toegevoegd
nexus-removed = Verwijderd
nexus-changed = Gewijzigd
btn-copy = Kopiëren
btn-save-as = Opslaan als…
msg-copied = Gekopieerd naar het klembord
msg-saved-to = Opgeslagen in { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Hernoemen…
condeditor-rename-exists = Er bestaat al een vlag met de naam ‘{ $name }’
condeditor-renamed = Vlag ‘{ $from }’ hernoemd naar ‘{ $to }’ ({ $num } keer)
condeditor-delete-uses = Alle gebruiken verwijderen
condeditor-deleted-uses = Vlag ‘{ $name }’ overal verwijderd ({ $num } keer)
condeditor-values-set = Ingestelde waarden:
condeditor-values-tested = Getoetste waarden:
condeditor-value-never-set = { $value } — getoetst maar nooit ingesteld
condeditor-value-never-tested = { $value } — ingesteld maar nooit getoetst
condeditor-builder = Voorwaardenbouwer
condeditor-builder-none = Selecteer in het hoofdvenster een stap, een optie, een voorwaardelijke bestandsset of de modinformatie om de voorwaarden ervan hier te bewerken.
condeditor-builder-pattern = Patroon:
condeditor-sentence-if = ALS
condeditor-sentence-and = EN
condeditor-sentence-or = OF
condeditor-sentence-flag = vlag { "{name}" } = { "{value}" }
condeditor-sentence-file = bestand { "{name}" } is { "{value}" }
condeditor-sentence-empty = (geen voorwaarde: altijd waar)
condeditor-sentence-then-visible = DAN wordt de stap getoond
condeditor-sentence-then-type = DAN wordt de optie { $type }
condeditor-sentence-then-install = DAN worden de bestanden geïnstalleerd
condeditor-sentence-then-module = DAN kan het installatieprogramma draaien (gecontroleerd voordat het start)
issue-flag-value-never-set = Vlag ‘{ $flag }’ wordt getoetst met de waarde ‘{ $value }’, die geen enkele optie instelt
issue-flag-never-used = Vlag ‘{ $flag }’ wordt ingesteld maar nergens getoetst
menu-project-strings = Projectteksten…
strings-title = Projectteksten
strings-search = Zoek tekst, locatie of sleutel…
strings-kind-all = Alle
strings-kind-names = Namen
strings-kind-descriptions = Beschrijvingen
strings-duplicates-only = Alleen duplicaten
strings-replace-with = Vervangen door:
strings-case = Hoofdlettergevoelig
strings-whole-word = Heel woord
strings-replace-current = Vervangen
strings-replace-all = Alles vervangen
strings-replaced = { $num } tekst(en) vervangen
strings-dup-badge = ×{ $num }
strings-dup-hover = Dezelfde tekst als:
strings-count = { $num } tekst(en) · { $dups } duplicaatgroep(en)
strings-col-location = Locatie
strings-col-field = Veld
strings-col-text = Tekst

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Archiefinhoud…
filter-bethesda-archive = Bethesda-archieven (bsa, ba2)
archive-view-title = Archiefinhoud
archive-view-format = Indeling:
archive-view-entries = { $num } items
archive-view-size = { $size } uitgepakt
archive-view-search = Zoek een pad…
archive-view-col-path = Pad
archive-view-col-size = Grootte
archive-view-col-compressed = Gecomprimeerd
archive-view-truncated = Alleen de eerste { $num } overeenkomende items worden getoond — verfijn de zoekopdracht.
archive-view-error = Dit archief kan niet worden gelezen: { $error }
archive-view-hint = Bekijk de inhoud van dit archief
issue-conflict-archive = Zelfde asset in meerdere archieven: “{ $path }” wordt ingepakt door { $count } verwijzingen ({ $locs }) — de archieflaadvolgorde van het spel bepaalt welke wordt gebruikt.
issue-conflict-archive-loose = Archief tegenover los bestand: “{ $path }” is zowel ingepakt in een archief als los geïnstalleerd ({ $locs }) — het losse bestand wint van het gearchiveerde.
preview-in-archive = (in archief)
preview-archived-size = waarvan { $size } ingepakt in archieven

# --- Project tree: expand / collapse menus
tree-expand = Uitvouwen
tree-collapse = Samenvouwen
tree-expand-all = Alles uitvouwen
tree-expand-selected = Selectie uitvouwen
tree-expand-from = Uitvouwen vanaf selectie
tree-collapse-all = Alles samenvouwen
tree-collapse-selected = Selectie samenvouwen
tree-collapse-from = Samenvouwen vanaf selectie
tree-expand-all-hint = Vouwt alle koppen uit
tree-expand-selected-hint = Vouwt alleen de geselecteerde kop uit
tree-expand-from-hint = Vouwt de geselecteerde kop en alles eronder uit
tree-collapse-all-hint = Vouwt alle koppen samen
tree-collapse-selected-hint = Vouwt alleen de geselecteerde kop samen
tree-collapse-from-hint = Vouwt de geselecteerde kop en alles eronder samen

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Groep toevoegen
btn-remove-group-cond = Groep verwijderen
dep-type-game = Spelversie
dep-type-fomm = Mod-manager-versie
dep-group-hint = Een groep voorwaarden gecombineerd met EN / OF; groepen kunnen worden genest.
condeditor-sentence-game = spelversie ≥ { "{value}" }
condeditor-sentence-fomm = mod-manager-versie ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unieke teksten
ftr-uniques-hint = Toon één rij per afzonderlijke brontekst. Door die rij te vertalen, vertaalt u in één keer alle tekenreeksen met dezelfde tekst.
ftr-uniques-synced = { $num } identieke tekenreeksen bijgewerkt.
ftr-uniques-group = { $num } tekenreeksen delen deze tekst; de vertaling ervan geldt voor allemaal.
