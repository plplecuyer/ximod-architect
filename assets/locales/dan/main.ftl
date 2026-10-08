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
menu-settings = Indstillinger…
menu-pre-save-script = Script før lagring…
menu-post-save-script = Script efter lagring…
menu-translation = Oversæt brugerfladen…
# Menu - Help
menu-help = Hjælp
menu-check-updates = Søg efter opdateringer…
menu-about = Om

# Update check
update-checking = Søger efter opdateringer…
update-up-to-date = XIMOD Architect er opdateret.
update-check-failed = Kunne ikke søge efter opdateringer. Prøv igen senere.
update-available-status = Version { $version } er tilgængelig.
update-banner-text = XIMOD Architect { $version } er tilgængelig.
update-download = Download:
update-skip = Spring denne version over
update-later = Senere

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
label-plugin-name = Valgmulighedens navn:
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
btn-add-plugin = Tilføj valgmulighed
btn-remove-plugin = Fjern valgmulighed
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Destination for hele gruppen
label-page-dest = Installationsdestination (hele siden)
btn-apply-group-dest = Anvend på alle valgmuligheder i denne gruppe
btn-apply-page-dest = Anvend på alle valgmuligheder på denne side
group-dest-hint = Angiver én installationsdestination for hver fil i hver valgmulighed i denne gruppe.
page-dest-hint = Angiver én installationsdestination for hver fil i hver valgmulighed på denne side (alle grupper).
bulk-dest-nofiles = Ingen filer at opdatere endnu — tilføj først filer til valgmulighederne.
status-dest-applied = Destination anvendt på { $num } fil(er).
preview-hidden-steps = { $num } trin skjult af aktuelle valg.
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
settings-check-updates = Søg efter opdateringer ved opstart
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
label-plugin-dependencies = Valgmulighedens afhængigheder
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
validation-no-plugins = Trin { $step }, gruppe "{ $name }" har ingen valgmuligheder

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
default-plugin-name = Valgmulighed { $num }
pattern-label = Mønster { $num }

# Selection prompts
msg-select-group-first = Vælg først en gruppe.
msg-select-plugin-edit = Vælg en valgmulighed, der skal redigeres.
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

# Referenced-file verification (V2)
verify-no-root = Filkontrol sprunget over: ingen rodmappe er angivet
loc-header = headerbillede
loc-required = påkrævede filer
loc-conditional = betinget sæt { $num }
loc-plugin = trin { $step }, gruppe { $group }, valgmulighed »{ $plugin }«
verify-missing-file = Manglende fil: { $path } ({ $loc })
verify-missing-folder = Manglende mappe: { $path } ({ $loc })
verify-missing-image = Manglende billede: { $path } ({ $loc })
verify-absolute = Absolut sti (ikke flytbar): { $path } ({ $loc })
verify-outside = Stien forlader rodmappen: { $path } ({ $loc })
verify-orphan = Forældreløs fil (ikke refereret af nogen valgmulighed): { $path }
conflict-certain = Destinationskonflikt: „{ $path }“ skrives af { $count } valg ({ $locs }) – de overskriver hinanden.
conflict-potential = Mulig destinationskonflikt: „{ $path }“ er mål for { $count } referencer ({ $locs }) – overskrivning afhænger af valg/betingelser.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Luk FOMOD
menu-close-all-fomods = Luk alle FOMOD'er
tab-untitled = (uden titel)
msg-drop-not-fomod = Det slupne element er ikke et FOMOD (ingen »fomod«-mappe fundet)
exit-title = Ikke-gemte ændringer
exit-unsaved = Et FOMOD er ikke gemt. Vil du gemme det?
tab-close-hint = Luk dette FOMOD
menu-new-from-folder = Ny fra mappe…
menu-templates = Skabeloner…
templates-title = Genbrugelige skabeloner
templates-empty = Ingen skabeloner gemt endnu. Gem det valgte trin ovenfor for at oprette en.
templates-insert = Indsæt
templates-save-step = Gem valgt trin
templates-name-hint = Skabelonnavn (valgfrit)
msg-wizard-success = Skelet oprettet fra mappe: { $num } valgmulighed(er).
msg-wizard-error = Fejl: { $error }
msg-template-saved = Skabelon gemt: { $name }
msg-template-inserted = Skabelon indsat i projektet.
msg-template-no-step = Vælg først et trin for at gemme det som en skabelon.
msg-template-no-dir = Kunne ikke finde skabelonmappen.
msg-drop-assigned = { $added } kilde(r) tilføjet til valgmuligheden ({ $rejected } uden for roden ignoreret).
menu-compare = Sammenlign med…
compare-title = FOMOD-sammenligning
compare-none = Ingen forskelle.
btn-optimize-image = Optimer billede
msg-image-optimized = Overskriftsbillede optimeret.
msg-image-ok = Overskriftsbilledet er allerede inden for grænserne.
msg-no-header-image = Intet overskriftsbillede at optimere.
verify-image-large = Billedet er for stort ({ $width }×{ $height }): { $path }
verify-image-format = Ikke-understøttet billedformat (.{ $ext }): { $path }
verify-image-unreadable = Ulæseligt billede: { $path }
menu-condition-editor = Betingelseseditor…
condeditor-title = Betingelseseditor
condeditor-set-by = Sat af:
condeditor-used-by = Brugt af:
condeditor-filedeps = Filafhængigheder
condeditor-empty = Ingen flag eller afhængigheder i dette projekt.
condeditor-orphan-set = sat, men aldrig brugt
condeditor-orphan-used = brugt, men aldrig sat
msg-img-optimized = Billede optimeret.
msg-img-ok = Billedet er allerede inden for grænserne.
msg-img-none = Intet billede at optimere.
msg-crash-recovery = Den forrige session blev afsluttet uventet. En sikkerhedskopi af dit projekt er gemt i { $path }
export-progress-title = Opretter distributionsarkivet…
export-progress-files = { $done } / { $total } filer
msg-export-cancelled = Eksport annulleret; det ufuldstændige arkiv blev fjernet.
verify-running = Kontrollerer filer på disken…
verify-stale = Bemærk: projektet blev ændret, mens filerne blev kontrolleret; kør valideringen igen.
prop-col-name = Navn
menu-save-as = Gem som…
menu-project = Projekt
menu-tools = Værktøjer
menu-manual = Brugervejledning
msg-manual-missing = Brugervejledningen (PDF) blev ikke fundet ved siden af programmet.
toolbar-new = Ny
toolbar-open = Åbn
toolbar-save = Gem
toolbar-validate = Validér
toolbar-preview = Forhåndsvisning
toolbar-export = Eksportér
dialog-choose-root = Vælg moddens rodmappe
exit-unsaved-docs = Ikke gemt: { $names }
status-summary = { $steps } trin · { $options } valgmuligheder
section-groups = Grupper
section-options = Valgmuligheder
section-flags = Betingelsesflag
section-files = Filer til installation
hint-group-type = Hvordan installationsprogrammet lader brugeren vælge valgmuligheder i denne gruppe.
hint-default-type = Hvordan valgmuligheden tilbydes, når ingen af dens afhængighedsmønstre passer: påkrævet, valgfri, anbefalet, ikke anvendelig…
hint-operator = Alle betingelser skal være sande (OG), eller blot én af dem (ELLER).
hint-flags = Flag er navngivne værdier, som denne valgmulighed sætter, når den vælges. Andre trin og valgmuligheder kan teste dem for at vise, skjule eller kræve sig selv.
hint-plugin-dependencies = Mønstre der ændrer valgmulighedens type ud fra flag eller filer i spillet: for eksempel “Påkrævet”, når en anden mod er installeret.
hint-files = Filer og mapper, der kopieres til spillets Data-mappe, når denne valgmulighed vælges. Destinationen er relativ til Data; højeste prioritet vinder ved konflikt.
hint-visibility = Betingelser der skal være opfyldt, for at dette trin vises. Lad stå tomt for altid at vise det.
seltype-exactly-one = Præcis én (påkrævet)
seltype-at-most-one = Højst én
seltype-any = Vilkårligt antal
seltype-all = Alle (intet valg)
seltype-at-least-one = Mindst én
plugtype-required = Påkrævet
plugtype-optional = Valgfri
plugtype-recommended = Anbefalet
plugtype-not-usable = Ikke anvendelig
plugtype-could-be-usable = Måske anvendelig
plugtype-required-hint = Installeres altid; brugeren kan ikke fravælge den.
plugtype-optional-hint = Tilbydes uden markering; brugeren bestemmer.
plugtype-recommended-hint = Tilbydes markeret; brugeren kan fravælge den.
plugtype-not-usable-hint = Vises nedtonet og kan ikke vælges.
plugtype-could-be-usable-hint = Kan vælges, men installationsprogrammet advarer om, at den måske ikke virker.
op-and = Alle betingelser (OG)
op-or = Enhver betingelse (ELLER)
theme-dark = Mørk
theme-light = Lys
theme-system = Følg systemet
condeditor-setter-loc = Trin { "{step}" } / Gruppe { "{group}" } / »{ "{name}" }«
condeditor-pattern-of = Mønster for »{ "{name}" }« → { "{type}" }
condeditor-visibility-of = Synlighed af trin { "{step}" }
condeditor-cond-set = Betinget sæt { "{num}" }
condeditor-needs = { "{ctx}" } (kræver = { "{value}" })
condeditor-file-dep = { "{ctx}" }: fil »{ "{name}" }« ({ "{state}" })
menu-translate-fomod = Oversæt et FOMOD…
ftr-title = Oversæt et FOMOD
ftr-open-folder = Åbn en mod-mappe…
ftr-from-active = Fra det aktive projekt
ftr-from-active-hint = Oversætter FOMOD'et for det projekt, der er åbent i hovedvinduet (det skal først gemmes).
ftr-no-fomod = Intet FOMOD indlæst.
ftr-encoding = De oprindelige filers tegnkodning; de oversatte filer skrives med samme tegnkodning.
ftr-source-lang = Fra
ftr-target-lang = til
ftr-lang-locked = (sprogene ligger fast, når et FOMOD er indlæst)
ftr-translator = Oversætter:
ftr-save = Gem oversættelse
ftr-export = Eksportér de oversatte filer
ftr-export-sibling = Til en mappe fomod_<sprog>
ftr-export-sibling-hint = Skriver de oversatte info.xml og ModuleConfig.xml ved siden af den oprindelige fomod-mappe; de oprindelige filer røres ikke.
ftr-export-inplace = Oven i de oprindelige filer
ftr-export-inplace-hint = Erstatter fomod/info.xml og fomod/ModuleConfig.xml efter at have lavet en tidsstemplet .bak-kopi af hver.
ftr-force-explicit-order = Bevar den oprindelige rækkefølge
ftr-warn-order = Lister sorteret efter navn (order="Ascending") ville blive sorteret om efter de oversatte navne i mod-manageren. Dette gennemtvinger order="Explicit", så valgmulighederne beholder deres nuværende rækkefølge.
ftr-update = Opdatér fra mappe
ftr-update-hint = Genindlæser FOMOD'et fra disken og fletter oversættelsen med det: nye, ændrede og fjernede strenge rapporteres.
ftr-preview-translated = Oversat forhåndsvisning
ftr-progress = { $done } / { $total } oversat
ftr-filter-all = Alle
ftr-filter-untranslated = Uoversatte
ftr-filter-review = Skal gennemses
ftr-filter-issues = Med problemer
ftr-filter-locked = Låste
ftr-type-all = Alle felter
ftr-type-names = Navne
ftr-type-descriptions = Beskrivelser
ftr-type-meta = Mod-oplysninger
ftr-search-hint = Søg i kilde, oversættelse eller kontekst…
ftr-next-untranslated = Næste uoversatte
ftr-show-whitespace = Vis mellemrum og linjeskift
ftr-discard-question = Den aktuelle oversættelse har ugemte ændringer. Kassér dem og indlæs det andet FOMOD?
ftr-discard-yes = Kassér
ftr-unsaved-close = Oversættelsen har ugemte ændringer.
ftr-col-num = Nr.
ftr-col-status = { "" }
ftr-col-context = Kontekst
ftr-col-source = Kilde
ftr-col-target = Oversættelse
ftr-col-issues = { "" }
ftr-empty-hint = Åbn en mod-mappe, eller indlæs det aktive projekt, for at få vist dets oversættelige strenge.
ftr-empty-filter = Ingen streng matcher det aktuelle filter.
ftr-select-row = Vælg en række for at redigere dens oversættelse.
ftr-copy-source = Kopiér kilde
ftr-clear-target = Ryd
ftr-lock = Oversæt ikke
ftr-lock-hint = Låste strenge skrives uændret (forfatter, websted, egennavne…).
ftr-note = Note:
ftr-status-untranslated = Uoversat
ftr-status-translated = Oversat
ftr-status-auto = Udfyldt automatisk — gennemse venligst
ftr-status-fuzzy = Kildeteksten er ændret, siden dette blev oversat — gennemse venligst
ftr-status-obsolete = Findes ikke længere i FOMOD'et
ftr-status-locked = Låst (skrives uændret)
ftr-field-info-name = Mod-navn (info.xml)
ftr-field-module-name = Installationsprogrammets titel (ModuleConfig.xml)
ftr-field-author = Forfatter
ftr-field-website = Websted
ftr-field-description = Mod-beskrivelse
ftr-field-step = Trinnavn
ftr-field-group = Gruppenavn
ftr-field-plugin = Valgmulighedens navn
ftr-field-plugin-desc = Valgmulighedens beskrivelse
ftr-issue-empty = Tom oversættelse
ftr-issue-whitespace = Oversættelsen indeholder kun mellemrum
ftr-issue-edge-whitespace = Mellemrum i starten eller slutningen afviger fra kilden
ftr-issue-token = Beskyttede tokens afviger — mangler: { $missing } ; overskydende: { $extra }
ftr-issue-newline-name = Et navn må ikke indeholde linjeskift
ftr-issue-control = Indeholder tegn, som XML ikke kan gemme
ftr-issue-length = Usædvanlig længde i forhold til kilden (×{ $ratio })
ftr-issue-identical = Identisk med kilden
ftr-issue-duplicate = Samme kildetekst er oversat anderledes i { $key }
ftr-issue-cdata = Sekvensen ]]> er ikke tilladt her
ftr-load-error = FOMOD'et kunne ikke indlæses: { $error }
ftr-extracted = { $num } oversættelige strenge fundet.
ftr-sidecar-found = Eksisterende oversættelse indlæst og flettet: { $new } nye, { $changed } ændrede, { $removed } fjernede.
ftr-saved = Oversættelse gemt i { $path }
ftr-save-error = Oversættelsen kunne ikke gemmes: { $error }
ftr-save-first = Gem først projektet, og oversæt det derefter.
ftr-export-success = { $count } strenge skrevet til { $path }
ftr-export-error = Eksport mislykkedes: { $error }
ftr-export-blocked = { $num } blokerende problemer skal rettes før eksport.
ftr-export-stale = { $num } strenge blev sprunget over, fordi FOMOD'et er ændret; brug »Opdatér fra mappe«.
ftr-update-report = Opdateret: { $new } nye, { $changed } ændrede, { $moved } flyttede, { $removed } fjernede, { $unchanged } uændrede.
menu-edit = Rediger
menu-undo = Fortryd
menu-redo = Gentag
tree-title = Projekt
tree-mod-info = Mod-oplysninger
tree-steps = Installationstrin
tree-required = Påkrævede filer
tree-conditional = Betingede installationer
tree-empty-steps = Ingen trin endnu — klik på + for at tilføje et.
tree-duplicate = Dupliker
tree-delete = Slet
tree-save-template = Gem som skabelon…
tree-drop-hint = Slip her for at flytte
cond-set-label = Betinget sæt { $num }
inspector-empty = Vælg et element i projekttræet, eller tilføj et trin for at komme i gang.
count-options = { $num } valgmuligheder
count-files = { $num } filer
msg-deleted-undo = Slettet. Brug Fortryd (Ctrl+Z) for at gendanne det.
problems-title = Problemer
problems-errors = { $num } fejl
problems-warnings = { $num } advarsler
btn-close = Luk
ftr-export-package = Som oversættelsespakke (arkiv)
ftr-export-package-hint = Opretter en .zip eller .7z, der er klar til upload: de oversatte info.xml og ModuleConfig.xml plus en README (kun patch), eller hele mod'et med de oversatte filer (komplet).
ftr-package-full = Hele mod'et
ftr-package-full-hint = Medtag alle mod'ets filer i arkivet, ikke kun de to oversatte XML-filer. Sørg for, at forfatteren tillader videredistribution.
ftr-package-name-template = Navn:
ftr-readme-patch = Dette arkiv indeholder oversættelsen ({ $langname }) af installationsprogrammet til »{ $name }« (fomod/info.xml og fomod/ModuleConfig.xml). Installér det oven på det oprindelige mod, eller lad din mod-manager flette det, så de oversatte filer erstatter de oprindelige. Kun installationsprogrammets tekster ændres; selve mod-filerne er ikke inkluderet. Lavet med XIMOD Architect.
ftr-readme-full = Dette arkiv indeholder »{ $name }« med installationsprogrammet oversat ({ $langname }; fomod/info.xml og fomod/ModuleConfig.xml). Installér det på samme måde som det oprindelige mod. Kun installationsprogrammets tekster er ændret. Lavet med XIMOD Architect.
ftr-apply-memory = Udfyld fra hukommelsen
ftr-memory-size = Oversættelseshukommelse: { $num } poster for dette sprogpar. Alle gemte oversættelser føjes til den.
ftr-memory-applied = { $num } strenge udfyldt fra oversættelseshukommelsen (markeret »skal gennemses«).
ftr-memory-suggestion = Hukommelsen foreslår:
ftr-use-suggestion = Brug
ftr-propagate = Overfør til identiske
ftr-propagate-hint = Kopiér denne oversættelse til alle andre strenge med samme kildetekst, som stadig er uoversatte.
ftr-propagated = { $num } identiske strenge udfyldt.
ftr-csv-export = Eksportér CSV…
ftr-csv-import = Importér CSV…
ftr-csv-imported = { $num } strenge opdateret fra CSV-filen.
ftr-csv-error = CSV-fejl: { $error }
ftr-glossary = Ordliste
ftr-glossary-source = Term
ftr-glossary-target = Oversættelse
ftr-glossary-case = Store/små bogstaver
ftr-glossary-dnt = Bevar
ftr-glossary-add = Tilføj term
ftr-issue-glossary = Ordliste: »{ $term }« er ikke oversat som forventet

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Åbn arkiv…
filter-archive = Mod-arkiver (zip, 7z)
msg-archive-opened = Arkiv åbnet ({ $num } filer udpakket): { $path }
msg-archive-reused = Arkivet er allerede udpakket, genbruger { $path }
msg-archive-unsupported = Arkivformatet ».{ $ext }« understøttes ikke; udpak det først med 7-Zip (kun .zip og .7z kan åbnes).
msg-archive-error = Fejl ved åbning af arkivet: { $error }
msg-archive-no-fomod = Ingen »fomod«-mappe fundet i arkivet ({ $path })
msg-archive-extracting = Udpakker arkivet…
ftr-open-archive = Åbn et mod-arkiv…
ftr-package-full-partial = Mod'et blev åbnet fra et arkiv, der kun indeholder dets fomod-mappe; komplette pakker kræver det udpakkede mod.
info-module-deps = Mod-krav
info-module-deps-hint = Filer eller flag, som hele mod'et kræver, før installationsprogrammet kører (moduleDependencies). Lad feltet stå tomt, hvis der ingen er.
info-header-advanced = Avanceret sidehoved
info-title-position = Titelplacering
info-title-colour = Titelfarve
info-title-colour-hint = Forventet: seks hexadecimale cifre (RRGGBB)
info-image-show = Vis sidehovedbillede
info-image-fade = Udton sidehovedbillede
info-image-height = Højde på sidehovedbillede
info-attr-default = (standard)
file-always-install = Altid
file-always-install-hint = Installér altid denne fil, også når valgmuligheden ikke er valgt (alwaysInstall).
file-install-if-usable = Hvis brugbar
file-install-if-usable-hint = Installér denne fil, når valgmuligheden er brugbar, også når den ikke er valgt (installIfUsable).
msg-import-lossy = Denne FOMOD indeholder { $num } konstruktioner, som XIMOD ikke kan redigere; de går tabt, når projektet gemmes.
fidelity-nested-deps = Indlejret afhængighedsgruppe i { $context } (kun ét niveau understøttes)
fidelity-game-dep = Krav om spilversion { $version } i { $context }
fidelity-fomm-dep = Krav om mod-manager-version { $version } i { $context }
fidelity-unknown = Elementet »{ $element }« i »{ $parent }« understøttes ikke ({ $context })
loc-module = mod-kravene
loc-step = trin { $step } »{ $name }«
loc-installer = installationsprogrammet

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Gendan en sikkerhedskopi…
backups-title = Gendan en sikkerhedskopi
backups-empty = Dette projekt har endnu ingen sikkerhedskopi. Der oprettes én, hver gang projektet gemmes oven i en tidligere version.
backups-changes = { $num } ændring(er) i forhold til det aktuelle projekt
btn-compare = Sammenlign
btn-restore = Gendan
btn-delete-backups = Slet alle sikkerhedskopier
btn-delete-backups-confirm = Klik igen for at slette alle sikkerhedskopier
msg-backup-restored = Sikkerhedskopi fra { $time } gendannet i editoren (ikke gemt endnu; Fortryd ruller den tilbage)
msg-backups-deleted = { $num } sikkerhedskopi(er) slettet
settings-backup-count = Sikkerhedskopier at beholde:
settings-backup-count-hint = Antal tidligere versioner af FOMOD-XML'en, der beholdes under fomod/backups ved gem (0 = ingen sikkerhedskopier).
settings-autosave-minutes = Gem automatisk en gendannelseskopi hvert (minutter):
settings-autosave-minutes-hint = Med dette interval skrives en gendannelseskopi af hvert ændret projekt i konfigurationsmappen; den tilbydes ved næste start kun efter en unormal afslutning (0 = fra).
settings-auto-masters = Tilføj et plugins masterfiler som betingelser
settings-auto-masters-hint = Når et plugin (.esp/.esm/.esl) føjes til en valgmulighed, bliver de masterfiler, det kræver, og som hverken spillet eller dette mod leverer, til »Active«-filbetingelser for valgmuligheden.
msg-author-from-plugin = Forfatter udfyldt fra plugin-headeren: { $author }
msg-masters-added = { $num } masterfil(er) for { $plugin } tilføjet som filbetingelse(r)
issue-missing-master = { $plugin } kræver { $master }, som hverken findes i dette mod eller er angivet som afhængighed
issue-esl-mismatch-flag = { $plugin } har filendelsen .esl, men dets light-flag (ESL) er ikke sat
issue-esl-eligible = { $plugin } kunne markeres som light ({ $num } nye records, grænse { $limit })
issue-esl-too-big = { $plugin } er markeret som light, men opfylder ikke reglerne for light-plugins ({ $num } nye records, grænse { $limit }, eller et FormID uden for det tilladte interval)
menu-plugin-report = Plugin-rapport…
plugins-title = Plugin-rapport
plugins-file = Fil
plugins-kind = Type
plugins-light = Light-flag
plugins-masters = Masterfiler
plugins-new-records = Nye records / grænse
plugins-eligible = Light-egnet
plugins-empty = Dette projekt installerer ingen plugin-fil (.esp, .esm eller .esl).
plugins-unreadable = ulæselig

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Endeligt filtræ
preview-total-size = Samlet installationsstørrelse: { $size }
preview-tree-truncated = Træet er afkortet: for mange filer at udfolde (størrelserne ovenfor er delvise).
preview-overwritten-by = Overskrevet af { $plugin }
preview-scenario = Scenarie:
preview-scenario-load = Indlæs
preview-scenario-save = Gem…
preview-scenario-delete = Slet
preview-scenario-name = Scenarienavn
preview-scenario-saved = Scenariet "{ $name }" er gemt under fomod/scenarios
preview-scenario-unresolved = { $num } valg i scenariet matcher ingen valgmulighed i dette projekt (omdøbt eller fjernet)
preview-scenario-none = (intet scenarie)
issue-unreachable-step = Trinnet "{ $step }" kan aldrig vises: dets synlighedsbetingelser tester en flagværdi, som ingen tidligere valgmulighed sætter
issue-unreachable-option = Valgmuligheden "{ $plugin }" kan aldrig vælges: dens mønstre for anvendelig type tester en flagværdi, som ingen valgmulighed sætter
issue-unreachable-cond = Det betingede filsæt { $num } kan aldrig anvendes: dets betingelser tester en flagværdi, som ingen valgmulighed sætter
size-option = Installationsstørrelse: { $size } ({ $num } fil(er))
size-missing = { $num } manglende kilde(r)
size-unknown = Installationsstørrelse: — (kør Valider for at måle)
menu-nexus-desc = Nexus-beskrivelse…
nexus-title = Nexus Mods-beskrivelse
nexus-format = Format:
nexus-include-requirements = Krav
nexus-include-options = Installationsvalg
nexus-include-install = Installation
nexus-include-changelog = Ændringslog
nexus-previous = Forrige version…
nexus-previous-none = (ingen forrige version: ingen ændringslog)
nexus-language = Sprog:
nexus-language-source = (kilde)
nexus-sec-requirements = Krav
nexus-sec-options = Installationsvalg
nexus-sec-install = Installation
nexus-sec-changelog = Ændringslog
nexus-install-text = Dette mod leveres med et FOMOD-installationsprogram: installer det med en mod-manager (Vortex, Mod Organizer 2) og vælg dine valgmuligheder i installationsprogrammet.
nexus-requires = Kræver
nexus-step = Trin
nexus-added = Tilføjet
nexus-removed = Fjernet
nexus-changed = Ændret
btn-copy = Kopiér
btn-save-as = Gem som…
msg-copied = Kopieret til udklipsholderen
msg-saved-to = Gemt i { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Omdøb…
condeditor-rename-exists = Der findes allerede et flag med navnet "{ $name }"
condeditor-renamed = Flaget "{ $from }" er omdøbt til "{ $to }" ({ $num } forekomst(er))
condeditor-delete-uses = Slet alle anvendelser
condeditor-deleted-uses = Flaget "{ $name }" er fjernet overalt ({ $num } forekomst(er))
condeditor-values-set = Satte værdier:
condeditor-values-tested = Testede værdier:
condeditor-value-never-set = { $value } — testet, men aldrig sat
condeditor-value-never-tested = { $value } — sat, men aldrig testet
condeditor-builder = Betingelsesbygger
condeditor-builder-none = Vælg et trin, en valgmulighed, et betinget filsæt eller mod-oplysningerne i hovedvinduet for at redigere betingelserne her.
condeditor-builder-pattern = Mønster:
condeditor-sentence-if = HVIS
condeditor-sentence-and = OG
condeditor-sentence-or = ELLER
condeditor-sentence-flag = flaget { "{name}" } = { "{value}" }
condeditor-sentence-file = filen { "{name}" } er { "{value}" }
condeditor-sentence-empty = (ingen betingelse: altid sand)
condeditor-sentence-then-visible = SÅ vises trinnet
condeditor-sentence-then-type = SÅ bliver valgmuligheden { $type }
condeditor-sentence-then-install = SÅ installeres filerne
condeditor-sentence-then-module = SÅ kan installationsprogrammet køre (kontrolleres før det starter)
issue-flag-value-never-set = Flaget "{ $flag }" testes med værdien "{ $value }", som ingen valgmulighed sætter
issue-flag-never-used = Flaget "{ $flag }" sættes, men testes aldrig nogen steder
menu-project-strings = Projektstrenge…
strings-title = Projektstrenge
strings-search = Søg efter tekst, placering eller nøgle…
strings-kind-all = Alle
strings-kind-names = Navne
strings-kind-descriptions = Beskrivelser
strings-duplicates-only = Kun dubletter
strings-replace-with = Erstat med:
strings-case = Forskel på store og små bogstaver
strings-whole-word = Hele ord
strings-replace-current = Erstat
strings-replace-all = Erstat alle
strings-replaced = { $num } streng(e) erstattet
strings-dup-badge = ×{ $num }
strings-dup-hover = Samme tekst som:
strings-count = { $num } streng(e) · { $dups } dubletgruppe(r)
strings-col-location = Placering
strings-col-field = Felt
strings-col-text = Tekst

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Arkivindhold…
filter-bethesda-archive = Bethesda-arkiver (bsa, ba2)
archive-view-title = Arkivets indhold
archive-view-format = Format:
archive-view-entries = { $num } poster
archive-view-size = { $size } udpakket
archive-view-search = Søg efter en sti…
archive-view-col-path = Sti
archive-view-col-size = Størrelse
archive-view-col-compressed = Komprimeret
archive-view-truncated = Kun de første { $num } matchende poster vises – indsnævr søgningen.
archive-view-error = Dette arkiv kan ikke læses: { $error }
archive-view-hint = Vis indholdet af dette arkiv
issue-conflict-archive = Samme ressource i flere arkiver: „{ $path }“ er pakket af { $count } referencer ({ $locs }) – spillets indlæsningsrækkefølge for arkiver afgør, hvilken der bruges.
issue-conflict-archive-loose = Arkiv mod løs fil: „{ $path }“ er både pakket i et arkiv og installeret som løs fil ({ $locs }) – den løse fil vinder over den arkiverede.
preview-in-archive = (i arkiv)
preview-archived-size = heraf { $size } pakket i arkiver

# --- Project tree: expand / collapse menus
tree-expand = Udfold
tree-collapse = Fold sammen
tree-expand-all = Udfold alle
tree-expand-selected = Udfold valgte
tree-expand-from = Udfold fra valgte
tree-collapse-all = Fold alle sammen
tree-collapse-selected = Fold valgte sammen
tree-collapse-from = Fold sammen fra valgte
tree-expand-all-hint = Udfolder alle overskrifter
tree-expand-selected-hint = Udfolder kun den valgte overskrift
tree-expand-from-hint = Udfolder den valgte overskrift og alt under den
tree-collapse-all-hint = Folder alle overskrifter sammen
tree-collapse-selected-hint = Folder kun den valgte overskrift sammen
tree-collapse-from-hint = Folder den valgte overskrift og alt under den sammen

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Tilføj gruppe
btn-remove-group-cond = Fjern gruppe
dep-type-game = Spilversion
dep-type-fomm = Mod-manager-version
dep-group-hint = En gruppe af betingelser kombineret med OG / ELLER; grupper kan indlejres.
condeditor-sentence-game = spilversionen ≥ { "{value}" }
condeditor-sentence-fomm = mod-manager-versionen ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unikke tekster
ftr-uniques-hint = Vis én række pr. særskilt kildetekst. Oversættes denne række, oversættes alle strenge med samme tekst på én gang.
ftr-uniques-synced = { $num } identiske strenge opdateret.
ftr-uniques-group = { $num } strenge deler denne tekst; oversættelsen gælder for dem alle.
