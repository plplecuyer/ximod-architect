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
menu-settings = Innstillinger…
menu-pre-save-script = Skript før lagring…
menu-post-save-script = Skript etter lagring…
menu-translation = Oversett grensesnittet…
# Menu - Help
menu-help = Hjelp
menu-check-updates = Se etter oppdateringer…
menu-about = Om

# Update check
update-checking = Ser etter oppdateringer…
update-up-to-date = XIMOD Architect er oppdatert.
update-check-failed = Kunne ikke se etter oppdateringer. Prøv igjen senere.
update-available-status = Versjon { $version } er tilgjengelig.
update-banner-text = XIMOD Architect { $version } er tilgjengelig.
update-download = Last ned:
update-skip = Hopp over denne versjonen
update-later = Senere

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
label-plugin-name = Alternativnavn:
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
btn-add-plugin = Legg til alternativ
btn-remove-plugin = Fjern alternativ
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Mål for hele gruppen
label-page-dest = Installasjonsmål (hele siden)
btn-apply-group-dest = Bruk på alle alternativer i denne gruppen
btn-apply-page-dest = Bruk på alle alternativer på denne siden
group-dest-hint = Angir ett installasjonsmål for hver fil i hvert alternativ i denne gruppen.
page-dest-hint = Angir ett installasjonsmål for hver fil i hvert alternativ på denne siden (alle grupper).
bulk-dest-nofiles = Ingen filer å oppdatere ennå — legg til filer i alternativene først.
status-dest-applied = Mål brukt på { $num } fil(er).
preview-hidden-steps = { $num } trinn skjult av gjeldende valg.
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
settings-check-updates = Se etter oppdateringer ved oppstart
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
label-plugin-dependencies = Alternativavhengigheter
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
validation-no-plugins = Trinn { $step }, gruppe «{ $name }» har ingen alternativer

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
default-plugin-name = Alternativ { $num }
pattern-label = Mønster { $num }

# Selection prompts
msg-select-group-first = Velg først en gruppe.
msg-select-plugin-edit = Velg et alternativ å redigere.
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

# Referenced-file verification (V2)
verify-no-root = Filkontroll hoppet over: ingen rotmappe er angitt
loc-header = toppbilde
loc-required = påkrevde filer
loc-conditional = betinget sett { $num }
loc-plugin = trinn { $step }, gruppe { $group }, alternativ «{ $plugin }»
verify-missing-file = Manglende fil: { $path } ({ $loc })
verify-missing-folder = Manglende mappe: { $path } ({ $loc })
verify-missing-image = Manglende bilde: { $path } ({ $loc })
verify-absolute = Absolutt sti (ikke flyttbar): { $path } ({ $loc })
verify-outside = Stien går utenfor rotmappen: { $path } ({ $loc })
verify-orphan = Foreldreløs fil (ikke referert av noe alternativ): { $path }
conflict-certain = Målkonflikt: «{ $path }» skrives av { $count } alternativer ({ $locs }) – de overskriver hverandre.
conflict-potential = Mulig målkonflikt: «{ $path }» er mål for { $count } referanser ({ $locs }) – overskriving avhenger av valg/betingelser.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Lukk FOMOD
menu-close-all-fomods = Lukk alle FOMOD-er
tab-untitled = (uten tittel)
msg-drop-not-fomod = Elementet som ble sluppet er ikke en FOMOD (ingen «fomod»-mappe funnet)
exit-title = Ulagrede endringer
exit-unsaved = En FOMOD er ikke lagret. Vil du lagre den?
tab-close-hint = Lukk denne FOMOD-en
menu-new-from-folder = Ny fra mappe…
menu-templates = Maler…
templates-title = Gjenbrukbare maler
templates-empty = Ingen maler lagret ennå. Lagre det valgte trinnet ovenfor for å opprette en.
templates-insert = Sett inn
templates-save-step = Lagre valgt trinn
templates-name-hint = Malnavn (valgfritt)
msg-wizard-success = Skjelett opprettet fra mappe: { $num } alternativ(er).
msg-wizard-error = Feil: { $error }
msg-template-saved = Mal lagret: { $name }
msg-template-inserted = Mal satt inn i prosjektet.
msg-template-no-step = Velg først et trinn for å lagre det som en mal.
msg-template-no-dir = Fant ikke malmappen.
msg-drop-assigned = { $added } kilde(r) lagt til alternativet ({ $rejected } utenfor roten ignorert).
menu-compare = Sammenlign med…
compare-title = FOMOD-sammenligning
compare-none = Ingen forskjeller.
btn-optimize-image = Optimaliser bilde
msg-image-optimized = Toppbilde optimalisert.
msg-image-ok = Toppbildet er allerede innenfor grensene.
msg-no-header-image = Ingen toppbilde å optimalisere.
verify-image-large = Bildet er for stort ({ $width }×{ $height }): { $path }
verify-image-format = Bildeformat som ikke støttes (.{ $ext }): { $path }
verify-image-unreadable = Uleselig bilde: { $path }
menu-condition-editor = Betingelsesredigerer…
condeditor-title = Betingelsesredigerer
condeditor-set-by = Satt av:
condeditor-used-by = Brukt av:
condeditor-filedeps = Filavhengigheter
condeditor-empty = Ingen flagg eller avhengigheter i dette prosjektet.
condeditor-orphan-set = satt, men aldri brukt
condeditor-orphan-used = brukt, men aldri satt
msg-img-optimized = Bilde optimalisert.
msg-img-ok = Bildet er allerede innenfor grensene.
msg-img-none = Ingen bilde å optimalisere.
msg-crash-recovery = Forrige økt ble avsluttet uventet. En sikkerhetskopi av prosjektet ditt er lagret i { $path }
export-progress-title = Oppretter distribusjonsarkivet…
export-progress-files = { $done } / { $total } filer
msg-export-cancelled = Eksport avbrutt; det ufullstendige arkivet ble fjernet.
verify-running = Kontrollerer filer på disken…
verify-stale = Merk: prosjektet ble endret mens filene ble kontrollert; kjør valideringen på nytt.
prop-col-name = Navn
menu-save-as = Lagre som…
menu-project = Prosjekt
menu-tools = Verktøy
menu-manual = Brukerhåndbok
msg-manual-missing = Brukerhåndboken (PDF) ble ikke funnet ved siden av programmet.
toolbar-new = Ny
toolbar-open = Åpne
toolbar-save = Lagre
toolbar-validate = Valider
toolbar-preview = Forhåndsvisning
toolbar-export = Eksporter
dialog-choose-root = Velg moddens rotmappe
exit-unsaved-docs = Ikke lagret: { $names }
status-summary = { $steps } trinn · { $options } alternativer
section-groups = Grupper
section-options = Alternativer
section-flags = Betingelsesflagg
section-files = Filer som skal installeres
hint-group-type = Hvordan installasjonsprogrammet lar brukeren velge alternativer i denne gruppen.
hint-default-type = Hvordan alternativet tilbys når ingen av avhengighetsmønstrene passer: påkrevd, valgfritt, anbefalt, ikke brukbart…
hint-operator = Alle betingelser må være sanne (OG), eller én av dem (ELLER).
hint-flags = Flagg er navngitte verdier som dette alternativet setter når det velges. Andre trinn og alternativer kan teste dem for å vise, skjule eller kreve seg selv.
hint-plugin-dependencies = Mønstre som endrer alternativets type ut fra flagg eller filer i spillet: for eksempel «Påkrevd» når en annen mod er installert.
hint-files = Filer og mapper som kopieres til spillets Data-mappe når dette alternativet velges. Målet er relativt til Data; høyeste prioritet vinner ved konflikt.
hint-visibility = Betingelser som må oppfylles for at dette trinnet i det hele tatt skal vises. La stå tomt for alltid å vise det.
seltype-exactly-one = Nøyaktig én (påkrevd)
seltype-at-most-one = Høyst én
seltype-any = Valgfritt antall
seltype-all = Alle (ingen valg)
seltype-at-least-one = Minst én
plugtype-required = Påkrevd
plugtype-optional = Valgfritt
plugtype-recommended = Anbefalt
plugtype-not-usable = Ikke brukbart
plugtype-could-be-usable = Kanskje brukbart
plugtype-required-hint = Installeres alltid; brukeren kan ikke fjerne haken.
plugtype-optional-hint = Tilbys uten hake; brukeren bestemmer.
plugtype-recommended-hint = Tilbys med hake; brukeren kan fjerne den.
plugtype-not-usable-hint = Vises nedtonet og kan ikke velges.
plugtype-could-be-usable-hint = Kan velges, men installasjonsprogrammet advarer om at det kanskje ikke virker.
op-and = Alle betingelser (OG)
op-or = Hvilken som helst betingelse (ELLER)
theme-dark = Mørkt
theme-light = Lyst
theme-system = Følg systemet
condeditor-setter-loc = Trinn { "{step}" } / Gruppe { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Mønster for «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Synlighet for trinn { "{step}" }
condeditor-cond-set = Betinget sett { "{num}" }
condeditor-needs = { "{ctx}" } (krever = { "{value}" })
condeditor-file-dep = { "{ctx}" }: fil «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = Oversett en FOMOD…
ftr-title = Oversett en FOMOD
ftr-open-folder = Åpne en mod-mappe…
ftr-from-active = Fra det aktive prosjektet
ftr-from-active-hint = Oversetter FOMOD-en til prosjektet som er åpent i hovedvinduet (det må lagres først).
ftr-no-fomod = Ingen FOMOD lastet inn.
ftr-encoding = Tegnkodingen til originalfilene; de oversatte filene skrives med samme tegnkoding.
ftr-source-lang = Fra
ftr-target-lang = til
ftr-lang-locked = (språkene ligger fast når en FOMOD er lastet inn)
ftr-translator = Oversetter:
ftr-save = Lagre oversettelsen
ftr-export = Eksporter de oversatte filene
ftr-export-sibling = Til en mappe fomod_<språk>
ftr-export-sibling-hint = Skriver de oversatte filene info.xml og ModuleConfig.xml ved siden av den opprinnelige fomod-mappen; originalfilene røres ikke.
ftr-export-inplace = Over originalfilene
ftr-export-inplace-hint = Erstatter fomod/info.xml og fomod/ModuleConfig.xml etter å ha laget en tidsstemplet .bak-kopi av hver.
ftr-force-explicit-order = Behold den opprinnelige rekkefølgen
ftr-warn-order = Lister sortert etter navn (order="Ascending") ville blitt sortert på nytt etter de oversatte navnene i mod-behandleren. Dette tvinger frem order="Explicit", slik at alternativene beholder sin nåværende rekkefølge.
ftr-update = Oppdater fra mappe
ftr-update-hint = Leser FOMOD-en på nytt fra disken og fletter oversettelsen med den: nye, endrede og fjernede strenger rapporteres.
ftr-preview-translated = Oversatt forhåndsvisning
ftr-progress = { $done } / { $total } oversatt
ftr-filter-all = Alle
ftr-filter-untranslated = Uoversatte
ftr-filter-review = Må gjennomgås
ftr-filter-issues = Med problemer
ftr-filter-locked = Låste
ftr-type-all = Alle felt
ftr-type-names = Navn
ftr-type-descriptions = Beskrivelser
ftr-type-meta = Mod-informasjon
ftr-search-hint = Søk i kilde, oversettelse eller kontekst…
ftr-next-untranslated = Neste uoversatte
ftr-show-whitespace = Vis mellomrom og linjeskift
ftr-discard-question = Den gjeldende oversettelsen har ulagrede endringer. Forkaste dem og laste inn den andre FOMOD-en?
ftr-discard-yes = Forkast
ftr-unsaved-close = Oversettelsen har ulagrede endringer.
ftr-col-num = Nr.
ftr-col-status = { "" }
ftr-col-context = Kontekst
ftr-col-source = Kilde
ftr-col-target = Oversettelse
ftr-col-issues = { "" }
ftr-empty-hint = Åpne en mod-mappe, eller last inn det aktive prosjektet, for å liste opp de oversettbare strengene.
ftr-empty-filter = Ingen streng samsvarer med gjeldende filter.
ftr-select-row = Velg en rad for å redigere oversettelsen.
ftr-copy-source = Kopier kilde
ftr-clear-target = Tøm
ftr-lock = Ikke oversett
ftr-lock-hint = Låste strenger skrives uendret (forfatter, nettsted, egennavn…).
ftr-note = Merknad:
ftr-status-untranslated = Uoversatt
ftr-status-translated = Oversatt
ftr-status-auto = Forhåndsutfylt automatisk — vennligst gjennomgå
ftr-status-fuzzy = Kildeteksten er endret siden dette ble oversatt — vennligst gjennomgå
ftr-status-obsolete = Finnes ikke lenger i FOMOD-en
ftr-status-locked = Låst (skrives uendret)
ftr-field-info-name = Mod-navn (info.xml)
ftr-field-module-name = Installasjonsprogrammets tittel (ModuleConfig.xml)
ftr-field-author = Forfatter
ftr-field-website = Nettsted
ftr-field-description = Mod-beskrivelse
ftr-field-step = Trinnavn
ftr-field-group = Gruppenavn
ftr-field-plugin = Alternativnavn
ftr-field-plugin-desc = Alternativbeskrivelse
ftr-issue-empty = Tom oversettelse
ftr-issue-whitespace = Oversettelsen inneholder bare mellomrom
ftr-issue-edge-whitespace = Mellomrom i starten eller slutten avviker fra kilden
ftr-issue-token = Beskyttede symboler avviker — mangler: { $missing } ; overflødige: { $extra }
ftr-issue-newline-name = Et navn kan ikke inneholde linjeskift
ftr-issue-control = Inneholder tegn som XML ikke kan lagre
ftr-issue-length = Uvanlig lengde sammenlignet med kilden (×{ $ratio })
ftr-issue-identical = Identisk med kilden
ftr-issue-duplicate = Samme kildetekst er oversatt annerledes i { $key }
ftr-issue-cdata = Sekvensen ]]> er ikke tillatt her
ftr-load-error = Kunne ikke laste inn FOMOD-en: { $error }
ftr-extracted = Fant { $num } oversettbare strenger.
ftr-sidecar-found = Eksisterende oversettelse lastet inn og flettet: { $new } nye, { $changed } endrede, { $removed } fjernede.
ftr-saved = Oversettelsen er lagret i { $path }
ftr-save-error = Kunne ikke lagre oversettelsen: { $error }
ftr-save-first = Lagre prosjektet først, og oversett det deretter.
ftr-export-success = { $count } strenger skrevet til { $path }
ftr-export-error = Eksporten mislyktes: { $error }
ftr-export-blocked = { $num } blokkerende problemer må rettes før eksport.
ftr-export-stale = { $num } strenger ble hoppet over fordi FOMOD-en er endret; bruk «Oppdater fra mappe».
ftr-update-report = Oppdatert: { $new } nye, { $changed } endrede, { $moved } flyttede, { $removed } fjernede, { $unchanged } uendrede.
menu-edit = Rediger
menu-undo = Angre
menu-redo = Gjør om
tree-title = Prosjekt
tree-mod-info = Mod-informasjon
tree-steps = Installasjonstrinn
tree-required = Påkrevde filer
tree-conditional = Betingede installasjoner
tree-empty-steps = Ingen trinn ennå — klikk på + for å legge til et.
tree-duplicate = Dupliser
tree-delete = Slett
tree-save-template = Lagre som mal…
tree-drop-hint = Slipp her for å flytte
cond-set-label = Betinget sett { $num }
inspector-empty = Velg et element i prosjekttreet, eller legg til et trinn for å komme i gang.
count-options = { $num } alternativer
count-files = { $num } filer
msg-deleted-undo = Slettet. Bruk Angre (Ctrl+Z) for å gjenopprette det.
problems-title = Problemer
problems-errors = { $num } feil
problems-warnings = { $num } advarsler
btn-close = Lukk
ftr-export-package = Som oversettelsespakke (arkiv)
ftr-export-package-hint = Lager en .zip eller .7z som er klar til opplasting: de oversatte filene info.xml og ModuleConfig.xml pluss en README (kun patch), eller hele modden med de oversatte filene (komplett).
ftr-package-full = Hele modden
ftr-package-full-hint = Ta med alle filene i modden i arkivet, ikke bare de to oversatte XML-filene. Sørg for at forfatteren tillater videredistribusjon.
ftr-package-name-template = Navn:
ftr-readme-patch = Dette arkivet inneholder oversettelsen ({ $langname }) av installasjonsprogrammet til «{ $name }» (fomod/info.xml og fomod/ModuleConfig.xml). Installer det over den opprinnelige modden, eller la mod-behandleren flette det, slik at de oversatte filene erstatter originalene. Bare tekstene i installasjonsprogrammet endres; selve mod-filene er ikke inkludert. Laget med XIMOD Architect.
ftr-readme-full = Dette arkivet inneholder «{ $name }» med installasjonsprogrammet oversatt ({ $langname }; fomod/info.xml og fomod/ModuleConfig.xml). Installer det på samme måte som den opprinnelige modden. Bare tekstene i installasjonsprogrammet er endret. Laget med XIMOD Architect.
ftr-apply-memory = Fyll ut fra minnet
ftr-memory-size = Oversettelsesminne: { $num } oppføringer for dette språkparet. Alle lagrede oversettelser legges til i det.
ftr-memory-applied = { $num } strenger fylt ut fra oversettelsesminnet (merket «må gjennomgås»).
ftr-memory-suggestion = Minnet foreslår:
ftr-use-suggestion = Bruk
ftr-propagate = Overfør til identiske
ftr-propagate-hint = Kopier denne oversettelsen til alle andre strenger med samme kildetekst som fortsatt er uoversatte.
ftr-propagated = { $num } identiske strenger fylt ut.
ftr-csv-export = Eksporter CSV …
ftr-csv-import = Importer CSV …
ftr-csv-imported = { $num } strenger oppdatert fra CSV-filen.
ftr-csv-error = CSV-feil: { $error }
ftr-glossary = Ordliste
ftr-glossary-source = Term
ftr-glossary-target = Oversettelse
ftr-glossary-case = Store/små bokstaver
ftr-glossary-dnt = Behold
ftr-glossary-add = Legg til term
ftr-issue-glossary = Ordliste: «{ $term }» er ikke oversatt som forventet

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Åpne arkiv…
filter-archive = Mod-arkiver (zip, 7z)
msg-archive-opened = Arkiv åpnet ({ $num } filer pakket ut): { $path }
msg-archive-reused = Arkivet er allerede pakket ut, gjenbruker { $path }
msg-archive-unsupported = Arkivformatet «.{ $ext }» støttes ikke; pakk det ut med 7-Zip først (bare .zip og .7z kan åpnes).
msg-archive-error = Feil ved åpning av arkivet: { $error }
msg-archive-no-fomod = Fant ingen «fomod»-mappe i arkivet ({ $path })
msg-archive-extracting = Pakker ut arkivet…
ftr-open-archive = Åpne et mod-arkiv…
ftr-package-full-partial = Modden ble åpnet fra et arkiv som bare inneholder fomod-mappen; fullstendige pakker krever den utpakkede modden.
info-module-deps = Mod-krav
info-module-deps-hint = Filer eller flagg som hele modden krever før installasjonsprogrammet kjører (moduleDependencies). La stå tomt hvis ingen.
info-header-advanced = Avansert topptekst
info-title-position = Tittelplassering
info-title-colour = Tittelfarge
info-title-colour-hint = Forventet: seks heksadesimale sifre (RRGGBB)
info-image-show = Vis toppbilde
info-image-fade = Ton ut toppbilde
info-image-height = Høyde på toppbilde
info-attr-default = (standard)
file-always-install = Alltid
file-always-install-hint = Installer alltid denne filen, selv når alternativet ikke er valgt (alwaysInstall).
file-install-if-usable = Hvis brukbar
file-install-if-usable-hint = Installer denne filen når alternativet er brukbart, selv når det ikke er valgt (installIfUsable).
msg-import-lossy = Denne FOMOD-en inneholder { $num } konstruksjoner som XIMOD ikke kan redigere; de går tapt når prosjektet lagres.
fidelity-nested-deps = Nestet avhengighetsgruppe i { $context } (bare ett nivå støttes)
fidelity-game-dep = Krav til spillversjon { $version } i { $context }
fidelity-fomm-dep = Krav til mod-manager-versjon { $version } i { $context }
fidelity-unknown = Elementet «{ $element }» i «{ $parent }» støttes ikke ({ $context })
loc-module = mod-kravene
loc-step = trinn { $step } «{ $name }»
loc-installer = installasjonsprogrammet

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Gjenopprett en sikkerhetskopi…
backups-title = Gjenopprett en sikkerhetskopi
backups-empty = Dette prosjektet har ingen sikkerhetskopi ennå. Én lages hver gang prosjektet lagres over en tidligere versjon.
backups-changes = { $num } endring(er) i forhold til gjeldende prosjekt
btn-compare = Sammenlign
btn-restore = Gjenopprett
btn-delete-backups = Slett alle sikkerhetskopier
btn-delete-backups-confirm = Klikk igjen for å slette alle sikkerhetskopier
msg-backup-restored = Sikkerhetskopi fra { $time } gjenopprettet i redigeringsprogrammet (ikke lagret ennå; Angre tilbakestiller den)
msg-backups-deleted = { $num } sikkerhetskopi(er) slettet
settings-backup-count = Sikkerhetskopier å beholde:
settings-backup-count-hint = Antall tidligere versjoner av FOMOD-XML-en som beholdes under fomod/backups ved lagring (0 = ingen sikkerhetskopier).
settings-autosave-minutes = Lagre gjenopprettingskopi automatisk hvert (minutter):
settings-autosave-minutes-hint = Med dette intervallet skrives en gjenopprettingskopi av hvert endret prosjekt i konfigurasjonsmappen; den tilbys ved neste oppstart bare etter en unormal avslutning (0 = av).
settings-auto-masters = Legg til et plugins masterfiler som betingelser
settings-auto-masters-hint = Når et plugin (.esp/.esm/.esl) legges til et alternativ, blir masterfilene det krever, og som verken spillet eller denne modden leverer, til «Active»-filbetingelser for alternativet.
msg-author-from-plugin = Forfatter fylt ut fra plugin-headeren: { $author }
msg-masters-added = { $num } masterfil(er) for { $plugin } lagt til som filbetingelse(r)
issue-missing-master = { $plugin } krever { $master }, som verken finnes i denne modden eller er oppgitt som avhengighet
issue-esl-mismatch-flag = { $plugin } har filendelsen .esl, men light-flagget (ESL) er ikke satt
issue-esl-eligible = { $plugin } kunne merkes som light ({ $num } nye records, grense { $limit })
issue-esl-too-big = { $plugin } er merket som light, men oppfyller ikke reglene for light-plugins ({ $num } nye records, grense { $limit }, eller en FormID utenfor det tillatte området)
menu-plugin-report = Plugin-rapport…
plugins-title = Plugin-rapport
plugins-file = Fil
plugins-kind = Type
plugins-light = Light-flagg
plugins-masters = Masterfiler
plugins-new-records = Nye records / grense
plugins-eligible = Light-egnet
plugins-empty = Dette prosjektet installerer ingen plugin-fil (.esp, .esm eller .esl).
plugins-unreadable = uleselig

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Endelig filtre
preview-total-size = Total installasjonsstørrelse: { $size }
preview-tree-truncated = Treet er avkortet: for mange filer å utvide (størrelsene ovenfor er ufullstendige).
preview-overwritten-by = Overskrevet av { $plugin }
preview-scenario = Scenario:
preview-scenario-load = Last inn
preview-scenario-save = Lagre…
preview-scenario-delete = Slett
preview-scenario-name = Scenarionavn
preview-scenario-saved = Scenarioet «{ $name }» er lagret under fomod/scenarios
preview-scenario-unresolved = { $num } valg i scenarioet samsvarer ikke med noe alternativ i dette prosjektet (omdøpt eller fjernet)
preview-scenario-none = (ingen scenario)
issue-unreachable-step = Trinnet «{ $step }» kan aldri vises: synlighetsbetingelsene tester en flaggverdi som ingen tidligere alternativer setter
issue-unreachable-option = Alternativet «{ $plugin }» kan aldri velges: mønstrene for brukbar type tester en flaggverdi som ingen alternativer setter
issue-unreachable-cond = Det betingede filsettet { $num } kan aldri gjelde: betingelsene tester en flaggverdi som ingen alternativer setter
size-option = Installasjonsstørrelse: { $size } ({ $num } fil(er))
size-missing = { $num } manglende kilde(r)
size-unknown = Installasjonsstørrelse: — (kjør Valider for å måle)
menu-nexus-desc = Nexus-beskrivelse…
nexus-title = Nexus Mods-beskrivelse
nexus-format = Format:
nexus-include-requirements = Krav
nexus-include-options = Installasjonsalternativer
nexus-include-install = Installasjon
nexus-include-changelog = Endringslogg
nexus-previous = Forrige versjon…
nexus-previous-none = (ingen forrige versjon: ingen endringslogg)
nexus-language = Språk:
nexus-language-source = (kilde)
nexus-sec-requirements = Krav
nexus-sec-options = Installasjonsalternativer
nexus-sec-install = Installasjon
nexus-sec-changelog = Endringslogg
nexus-install-text = Denne modden leveres med et FOMOD-installasjonsprogram: installer den med en modbehandler (Vortex, Mod Organizer 2) og velg alternativene dine i installasjonsprogrammet.
nexus-requires = Krever
nexus-step = Trinn
nexus-added = Lagt til
nexus-removed = Fjernet
nexus-changed = Endret
btn-copy = Kopier
btn-save-as = Lagre som…
msg-copied = Kopiert til utklippstavlen
msg-saved-to = Lagret i { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Gi nytt navn…
condeditor-rename-exists = Det finnes allerede et flagg med navnet «{ $name }»
condeditor-renamed = Flagget «{ $from }» har fått nytt navn «{ $to }» ({ $num } forekomst(er))
condeditor-delete-uses = Slett alle bruk
condeditor-deleted-uses = Flagget «{ $name }» er fjernet overalt ({ $num } forekomst(er))
condeditor-values-set = Satte verdier:
condeditor-values-tested = Testede verdier:
condeditor-value-never-set = { $value } — testet, men aldri satt
condeditor-value-never-tested = { $value } — satt, men aldri testet
condeditor-builder = Betingelsesbygger
condeditor-builder-none = Velg et trinn, et alternativ, et betinget filsett eller mod-informasjonen i hovedvinduet for å redigere betingelsene her.
condeditor-builder-pattern = Mønster:
condeditor-sentence-if = HVIS
condeditor-sentence-and = OG
condeditor-sentence-or = ELLER
condeditor-sentence-flag = flagget { "{name}" } = { "{value}" }
condeditor-sentence-file = filen { "{name}" } er { "{value}" }
condeditor-sentence-empty = (ingen betingelse: alltid sann)
condeditor-sentence-then-visible = DA vises trinnet
condeditor-sentence-then-type = DA blir alternativet { $type }
condeditor-sentence-then-install = DA installeres filene
condeditor-sentence-then-module = DA kan installasjonsprogrammet kjøre (kontrolleres før det starter)
issue-flag-value-never-set = Flagget «{ $flag }» testes med verdien «{ $value }», som ingen alternativer setter
issue-flag-never-used = Flagget «{ $flag }» settes, men testes aldri noe sted
menu-project-strings = Prosjektstrenger…
strings-title = Prosjektstrenger
strings-search = Søk etter tekst, plassering eller nøkkel…
strings-kind-all = Alle
strings-kind-names = Navn
strings-kind-descriptions = Beskrivelser
strings-duplicates-only = Bare duplikater
strings-replace-with = Erstatt med:
strings-case = Skill mellom store og små bokstaver
strings-whole-word = Hele ord
strings-replace-current = Erstatt
strings-replace-all = Erstatt alle
strings-replaced = { $num } streng(er) erstattet
strings-dup-badge = ×{ $num }
strings-dup-hover = Samme tekst som:
strings-count = { $num } streng(er) · { $dups } duplikatgruppe(r)
strings-col-location = Plassering
strings-col-field = Felt
strings-col-text = Tekst

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Arkivinnhold…
filter-bethesda-archive = Bethesda-arkiver (bsa, ba2)
archive-view-title = Arkivinnhold
archive-view-format = Format:
archive-view-entries = { $num } oppføringer
archive-view-size = { $size } utpakket
archive-view-search = Søk etter en sti…
archive-view-col-path = Sti
archive-view-col-size = Størrelse
archive-view-col-compressed = Komprimert
archive-view-truncated = Bare de første { $num } samsvarende oppføringene vises – avgrens søket.
archive-view-error = Dette arkivet kan ikke leses: { $error }
archive-view-hint = Vis innholdet i dette arkivet
issue-conflict-archive = Samme ressurs i flere arkiver: «{ $path }» er pakket av { $count } referanser ({ $locs }) – spillets innlastingsrekkefølge for arkiver avgjør hvilken som brukes.
issue-conflict-archive-loose = Arkiv mot løs fil: «{ $path }» er både pakket i et arkiv og installert som løs fil ({ $locs }) – den løse filen vinner over den arkiverte.
preview-in-archive = (i arkiv)
preview-archived-size = hvorav { $size } pakket i arkiver

# --- Project tree: expand / collapse menus
tree-expand = Utvid
tree-collapse = Skjul
tree-expand-all = Utvid alle
tree-expand-selected = Utvid valgt
tree-expand-from = Utvid fra valgt
tree-collapse-all = Skjul alle
tree-collapse-selected = Skjul valgt
tree-collapse-from = Skjul fra valgt
tree-expand-all-hint = Utvider alle overskrifter
tree-expand-selected-hint = Utvider bare den valgte overskriften
tree-expand-from-hint = Utvider den valgte overskriften og alt under den
tree-collapse-all-hint = Skjuler alle overskrifter
tree-collapse-selected-hint = Skjuler bare den valgte overskriften
tree-collapse-from-hint = Skjuler den valgte overskriften og alt under den

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Legg til gruppe
btn-remove-group-cond = Fjern gruppe
dep-type-game = Spillversjon
dep-type-fomm = Mod-manager-versjon
dep-group-hint = En gruppe betingelser kombinert med OG / ELLER; grupper kan nøstes.
condeditor-sentence-game = spillversjonen ≥ { "{value}" }
condeditor-sentence-fomm = mod-manager-versjonen ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unike tekster
ftr-uniques-hint = Vis én rad per distinkt kildetekst. Oversettes denne raden, oversettes alle strenger med samme tekst på én gang.
ftr-uniques-synced = { $num } identiske strenger oppdatert.
ftr-uniques-group = { $num } strenger deler denne teksten; oversettelsen gjelder for alle.
