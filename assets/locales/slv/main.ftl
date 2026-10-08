# XIMOD Architect - translation metadata
# @language = slv
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Slovenščina
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Različica { $version }

# Status messages
status-ready = Pripravljeno
msg-save-success = FOMOD uspešno shranjen
msg-save-error = Napaka pri shranjevanju FOMOD-a
msg-export-success = Arhiv za distribucijo ustvarjen ({ $count } datotek): { $path }
msg-export-error = Napaka pri ustvarjanju arhiva za distribucijo: { $error }
msg-load-success = FOMOD uspešno naložen
msg-load-error = Napaka pri nalaganju FOMOD-a
msg-merge-success = FOMOD je bil uspešno združen
msg-merge-error = Napaka pri združevanju FOMOD-a
msg-no-root-selected = Najprej izberite korenski imenik
msg-no-fomod-folder = Mape »fomod« ni bilo mogoče najti. Želite jo ustvariti?
msg-file-outside-root = Datoteka je zunaj korenskega imenika

# Menu - File
menu-file = Datoteka
menu-new = Novo
menu-open = Odpri mapo…
menu-open-file = Odpri datoteko…
menu-save = Shrani
menu-recent = Zadnje
menu-exit = Izhod
menu-merge = Združi FOMOD…
menu-export = Izvozi distribucijski arhiv…
# Menu - Options
menu-options = Možnosti
menu-settings = Nastavitve…
menu-pre-save-script = Skript pred shranjevanjem…
menu-post-save-script = Skript po shranjevanju…
menu-translation = Prevedi vmesnik…
# Menu - Help
menu-help = Pomoč
menu-check-updates = Preveri posodobitve…
menu-about = O programu

# Update check
update-checking = Preverjanje posodobitev…
update-up-to-date = XIMOD Architect je posodobljen.
update-check-failed = Posodobitev ni bilo mogoče preveriti. Poskusite znova pozneje.
update-available-status = Na voljo je različica { $version }.
update-banner-text = Na voljo je XIMOD Architect { $version }.
update-download = Prenesi:
update-skip = Preskoči to različico
update-later = Pozneje

# Tabs
tab-info = Informacije o modu
tab-steps = Koraki namestitve
tab-required = Obvezne namestitve
tab-conditional = Pogojne namestitve

# Info Tab
label-workspace = Delovno okolje
label-root-dir = Koreninski imenik:
label-mod-name = Ime modifikacije:
label-author = Avtor:
label-version = Različica:
label-game-name = Ime igre:
label-category = Kategorija:
label-url = URL spletne strani:
label-header-image = Slika v glavi:
label-description = Opis:
placeholder-select-dir = (Izberite imenik)
placeholder-select-game = (Izberite igro)

# Steps Tab
label-step-name = Ime koraka:
label-group-name = Ime skupine:
label-group-type = Vrsta skupine:
label-plugin-name = Ime možnosti:
label-plugin-desc = Opis:
label-plugin-type = Privzeta vrsta:
label-plugin-image = Slika:
label-visibility = Pogoji vidnosti
label-operator = Operator:

# Buttons
btn-browse = Brskaj...
btn-clear = Počisti
btn-add = Dodaj
btn-remove = Odstrani
btn-add-step = Nov korak
btn-delete-step = Izbriši korak
btn-add-group = Dodaj skupino
btn-remove-group = Odstrani skupino
btn-add-plugin = Dodaj možnost
btn-remove-plugin = Odstrani možnost
btn-add-file = Dodaj datoteko
btn-add-folder = Dodaj mapo
btn-remove-file = Odstrani
btn-add-flag = Dodaj oznako
btn-remove-flag = Odstrani oznako
btn-add-condition = Dodaj pogoj
btn-remove-condition = Odstrani pogoj
btn-add-dependency = Dodaj odvisnost
btn-remove-dependency = Odstrani odvisnost
btn-add-pattern = Nov vzorec
btn-remove-pattern = Izbriši vzorec
btn-save = Shrani
btn-cancel = Prekliči
btn-ok = OK
btn-yes = Da
btn-no = Ne

# Condition/Dependency Labels
label-flag-name = Ime oznake:
label-flag-value = Vrednost:
label-condition-type = Vrsta:
label-condition-name = Ime:
label-condition-value = Vrednost:
label-dep-type = Vrsta odvisnosti:
label-dep-name = Ime/datoteka:
label-dep-value = Vrednost/stanje:

# Files
label-source = Vir
label-destination = Cilj
label-priority = Prioriteta
label-file-type = Tip

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Cilj za celotno skupino
label-page-dest = Cilj namestitve (cela stran)
btn-apply-group-dest = Uporabi za vse možnosti v tej skupini
btn-apply-page-dest = Uporabi za vse možnosti na tej strani
group-dest-hint = Nastavi en cilj namestitve za vsako datoteko vsake možnosti v tej skupini.
page-dest-hint = Nastavi en cilj namestitve za vsako datoteko vsake možnosti na tej strani (vse skupine).
bulk-dest-nofiles = Zaenkrat ni datotek za posodobitev — najprej dodajte datoteke možnostim.
status-dest-applied = Cilj uporabljen za { $num } datotek(o).
preview-hidden-steps = { $num } korak(ov) skritih zaradi trenutnih izbir.
label-files = Datoteke
label-dependencies = Odvisnosti

# Settings Dialog
settings-title = Nastavitve
settings-tab-general = Splošno
settings-tab-recent-files = Nedavne datoteke
settings-language = Jezik:
settings-theme = Tema:
settings-font-size = Velikost pisave:
settings-replace-newlines = Obdelaj nove vrstice v opisih
settings-check-updates = Preveri posodobitve ob zagonu
settings-max-recent = Največ zadnjih datotek:
settings-window-width = Širina okna:
settings-window-height = Višina okna:
settings-no-recent-files = Ni zadnjih datotek.

# Status messages for settings
status-settings-saved = Nastavitve so bile uspešno shranjene

# About Dialog
about-title = O programu XIMOD Architect
about-description = Večplatformsko orodje za ustvarjanje namestitvenih datotek FOMOD za modifikacije iger Bethesda.
about-license = Licencirano pod licenco MIT
about-copyright = © 2025–2026 Ekipa XIMOD
about-credit = Rust-port originalnega orodja podjetja Wenderer:

# Script Dialog
script-title = Uredi skript
script-info = Skripti se izvedejo pred ali po shranjevanju. Uporabite lahko naslednje makre:
script-macros = Razpoložljivi makroji:
macro-modname = $MODNAME$ – Ime modifikacije
macro-modauthor = $MODAUTHOR$ – Ime avtorja
macro-modversion = $MODVERSION$ – Različica modifikacije
macro-modroot = $MODROOT$ – Pot do korenskega imenika
macro-date = $DATE$ – Trenutni datum (LLLL-MM-DD)
macro-time = $TIME$ – Trenutni čas (HH:MM:SS)
macro-random = $RANDOM$ – Naključno število

# Plugin Dependencies
label-plugin-dependencies = Odvisnosti možnosti
label-default-type = Privzeti tip:
label-pattern-type = Tip vzorca:
label-pattern-operator = Operator vzorca:

# Conditional Files
label-pattern = Vzorec

# Validation Messages
validation-no-name = Ime modula je obvezno
validation-no-steps = Potreben je vsaj en korak ali obvezna datoteka
validation-empty-step = Korak { $num } nima imena
validation-empty-group = Korak { $step }, skupina { $group } nima imena
validation-no-plugins = Korak { $step }, skupina "{ $name }" nima možnosti

# File States
state-active = Aktivno
state-inactive = Neaktivno
state-missing = Manjka

# Confirmation
confirm-title = Potrditev
confirm-delete = Ali res želite izbrisati ta element?
confirm-discard = Imate neshranjene spremembe. Ali jih želite zavreči in nadaljevati?
confirm-unsaved = Imate neshranjene spremembe. Ali želite shraniti pred zaprtjem?
confirm-save-issues = Projekt ima naslednje težave:
confirm-save-anyway = Shraniti kljub temu?

# Errors
error-invalid-xml = Neveljavna datoteka XML
error-parse-failed = Razčlenitev FOMOD ni uspela
error-write-failed = Pisanje datoteke ni uspelo
error-create-dir = Ustvarjanje mape ni uspelo

# Default names (generated when creating new items)
default-step-name = Korak { $num }
default-group-name = Skupina { $num }
default-plugin-name = Možnost { $num }
pattern-label = Vzorec { $num }

# Selection prompts
msg-select-group-first = Najprej izberite skupino.
msg-select-plugin-edit = Izberite možnost za urejanje.
label-empty = (prazno)
image-no-image = Brez slike

# File dialog filters
filter-images = Slike
filter-xml = XML

# Dependency types
dep-type-flag = Zastavica
dep-type-file = Datoteka

# Status bar
status-modified = Spremenjeno

# Status messages (errors)
msg-settings-save-error = Napaka pri shranjevanju nastavitev
msg-script-save-error = Napaka pri shranjevanju skripta

# Translation editor
trans-title = Urejevalnik prevodov
trans-source-lang = Prikazani jezik:
trans-target-lang = Jezik za prevod:
trans-col-key = Ključ
trans-col-source = Oznaka
trans-col-target = Prevod
trans-saved = Prevod shranjen
trans-save-error = Napaka pri shranjevanju prevoda

# XML editor
xml-editor-title = Urednik XML
xml-editor-edit = Uredi
xml-editor-apply = Uporabi
xml-editor-revert = Prekliči
xml-editor-readonly = Samo za branje
xml-editor-editing = Urejanje — grafični zavihki so zaklenjeni
xml-editor-error = Napaka:
xml-editor-applied = Spremembe XML-ja so bile uporabljene
xml-editor-wellformed = Pravilno oblikovan XML
xml-editor-error-at = Vrstica { $line }, stolpec { $col }: { $msg }

# Country / flag picker
settings-country-name = Ime države:
settings-pick-country = Kliknite, da izberete svojo državo
flags-title = Izberite državo
flags-filter = Filter:
flags-none = Zastava ni bila najdena

# Translation editor: country & font
trans-endonym = Endonim države:
trans-font = Pisava:
trans-no-font = (ni)
trans-browse = Brskaj…
trans-google-fonts = Google Fonts
trans-pick-country = Kliknite, da izberete državo
trans-font-outside = Pisavo je treba najprej namestiti v mapo assets/fonts.
trans-font-dir-missing = Mape assets/fonts ni bilo mogoče najti.

# Translation submission
trans-lang-endonym = Endonim jezika:
trans-author = Avtor:
trans-submit = Pošlji…
trans-submit-hint = Ustvarite datoteko zip in odprite vnaprej izpolnjeno e-poštno sporočilo
trans-data-updated = Referenčni podatki so posodobljeni (Languages.json / Countries.json)
trans-package-ready = Arhiv je pripravljen:
trans-package-error = Arhiva ni bilo mogoče ustvariti:

# ISO 639-3 requirement
trans-lang-not-iso = Prevod je mogoč le za jezik z oznako ISO 639-3.

# FOMOD installer preview
menu-preview = Predogled namestitvenega programa…
preview-title = Predogled namestitvenega programa FOMOD
preview-refresh = Osveži
preview-assumptions = Predpostavke o datotekah
preview-details = Podrobnosti
preview-back = Nazaj
preview-next = Naprej
preview-install = Namesti
preview-close = Zapri
preview-restart = Ponovni zagon
preview-summary-title = Datoteke, ki bodo nameščene
preview-empty = Nobena datoteka ne bo nameščena.
preview-none-option = (ni)
preview-invalid = Izpolnite obvezne izbire, da nadaljujete.
preview-no-steps = Ni vidnih korakov; glejte povzetek namestitve.
preview-select-hint = Izberite možnost, da si ogledate njen opis.
preview-col-source = Izvor
preview-col-dest = Cilj
preview-col-priority = Prioriteta
preview-sel-exactlyone = Izberite natanko eno možnost.
preview-sel-atmostone = Izberite največ eno možnost.
preview-sel-any = Izberite poljubno število možnosti.
preview-sel-all = Namestijo se vse možnosti.
preview-sel-atleastone = Izberite vsaj eno možnost.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Preveri FOMOD
validate-report-title = Preverjanje FOMOD
validate-ok = Ni bilo ugotovljenih težav. FOMOD je skladen s shemo.
xml-editor-schema-ok = Skladno s shemo ModConfig 5.0.
xml-editor-schema-issues = Težave s shemo:
schema-line-col = Vrstica { $line }, stolpec { $col }: { $msg }
schema-wrong-root = Nepričakovani koren »{ $found }« (pričakovano »{ $expected }«).
schema-unknown = Nepričakovan element »{ $element }« v »{ $parent }«.
schema-missing = »{ $parent }« mora vsebovati »{ $child }«.
schema-needs-one = »{ $parent }« mora vsebovati vsaj en »{ $child }«.
schema-too-many = »{ $child }« se sme pojaviti le enkrat v »{ $parent }«.
schema-missing-attr = Atribut »{ $attr }« je obvezen za »{ $element }«.
schema-bad-enum = Neveljavna vrednost „{ $value }“ za { $element }/@{ $attr } (pričakovano: { $allowed }).
schema-choose-one = „{ $parent }“ mora vsebovati natanko eno od: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Premakni pred
reorder-after = Premakni za

# Country / language database explorer (Properties)
menu-properties = Lastnosti…
prop-title = Baza podatkov držav/jezikov
prop-tab-countries = Države
prop-tab-languages = Jeziki
prop-filter = Filter:
prop-official-langs = Uradni jeziki
prop-spoken-langs = Govorjeni jeziki
prop-endonym = Endonim države
prop-font = Pisava
prop-spoken-in = Govori se v
prop-select-country = Izberite državo, da si ogledate podrobnosti.
prop-select-lang = Izberite jezik, da si ogledate podrobnosti.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Odpri stran igre na Nexus Mods

# Referenced-file verification (V2)
verify-no-root = Preverjanje datotek preskočeno: korenska mapa ni nastavljena
loc-header = slika glave
loc-required = obvezne datoteke
loc-conditional = pogojni nabor { $num }
loc-plugin = korak { $step }, skupina { $group }, možnost »{ $plugin }«
verify-missing-file = Manjkajoča datoteka: { $path } ({ $loc })
verify-missing-folder = Manjkajoča mapa: { $path } ({ $loc })
verify-missing-image = Manjkajoča slika: { $path } ({ $loc })
verify-absolute = Absolutna pot (ni prenosljiva): { $path } ({ $loc })
verify-outside = Pot zapušča korensko mapo: { $path } ({ $loc })
verify-orphan = Osirotela datoteka (nobena možnost je ne uporablja): { $path }
conflict-certain = Konflikt cilja: „{ $path }“ piše { $count } možnosti ({ $locs }) — prepišejo se med seboj.
conflict-potential = Možen konflikt cilja: „{ $path }“ je cilj { $count } sklicev ({ $locs }) — prepis je odvisen od izbire/pogojev.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Zapri FOMOD
menu-close-all-fomods = Zapri vse FOMOD-e
tab-untitled = (brez naslova)
msg-drop-not-fomod = Spuščeni element ni FOMOD (mapa »fomod« ni bila najdena)
exit-title = Neshranjene spremembe
exit-unsaved = FOMOD ni shranjen. Ali ga želite shraniti?
tab-close-hint = Zapri ta FOMOD
menu-new-from-folder = Nov iz mape…
menu-templates = Predloge…
templates-title = Predloge za večkratno uporabo
templates-empty = Shranjenih predlog še ni. Shranite zgoraj izbrani korak, da ustvarite predlogo.
templates-insert = Vstavi
templates-save-step = Shrani izbrani korak
templates-name-hint = Ime predloge (neobvezno)
msg-wizard-success = Ogrodje ustvarjeno iz mape: { $num } možnost(i).
msg-wizard-error = Napaka: { $error }
msg-template-saved = Predloga shranjena: { $name }
msg-template-inserted = Predloga vstavljena v projekt.
msg-template-no-step = Najprej izberite korak, da ga shranite kot predlogo.
msg-template-no-dir = Mape s predlogami ni mogoče najti.
msg-drop-assigned = Možnosti dodan(ih) { $added } vir(ov) ({ $rejected } zunaj korena prezrto).
menu-compare = Primerjaj z…
compare-title = Primerjava FOMOD
compare-none = Ni razlik.
btn-optimize-image = Optimiziraj sliko
msg-image-optimized = Slika glave optimizirana.
msg-image-ok = Slika glave je že znotraj omejitev.
msg-no-header-image = Ni slike glave za optimizacijo.
verify-image-large = Slika prevelika ({ $width }×{ $height }): { $path }
verify-image-format = Nepodprta oblika slike (.{ $ext }): { $path }
verify-image-unreadable = Neberljiva slika: { $path }
menu-condition-editor = Urejevalnik pogojev…
condeditor-title = Urejevalnik pogojev
condeditor-set-by = Nastavi:
condeditor-used-by = Uporablja:
condeditor-filedeps = Odvisnosti datotek
condeditor-empty = V tem projektu ni zastavic ali odvisnosti.
condeditor-orphan-set = nastavljeno, a nikoli uporabljeno
condeditor-orphan-used = uporabljeno, a nikoli nastavljeno
msg-img-optimized = Slika optimizirana.
msg-img-ok = Slika je že znotraj omejitev.
msg-img-none = Ni slike za optimizacijo.
msg-crash-recovery = Prejšnja seja se je nepričakovano končala. Varnostna kopija vašega projekta je bila shranjena v { $path }
export-progress-title = Ustvarjanje distribucijskega arhiva…
export-progress-files = { $done } / { $total } datotek
msg-export-cancelled = Izvoz preklican; delni arhiv je bil odstranjen.
verify-running = Preverjanje datotek na disku…
verify-stale = Opomba: projekt se je med preverjanjem datotek spremenil; ponovno zaženite preverjanje.
prop-col-name = Ime
menu-save-as = Shrani kot…
menu-project = Projekt
menu-tools = Orodja
menu-manual = Uporabniški priročnik
msg-manual-missing = Uporabniškega priročnika (PDF) ni bilo mogoče najti poleg aplikacije.
toolbar-new = Novo
toolbar-open = Odpri
toolbar-save = Shrani
toolbar-validate = Preveri
toolbar-preview = Predogled
toolbar-export = Izvozi
dialog-choose-root = Izberite korensko mapo moda
exit-unsaved-docs = Neshranjeno: { $names }
status-summary = { $steps } korakov · { $options } možnosti
section-groups = Skupine
section-options = Možnosti
section-flags = Zastavice pogojev
section-files = Datoteke za namestitev
hint-group-type = Kako namestitveni program uporabniku omogoča izbiro možnosti v tej skupini.
hint-default-type = Kako je možnost ponujena, ko se ne ujema noben vzorec odvisnosti: obvezna, izbirna, priporočena, neuporabna…
hint-operator = Vsi pogoji morajo biti izpolnjeni (IN) ali katerikoli od njih (ALI).
hint-flags = Zastavice so poimenovane vrednosti, ki jih ta možnost nastavi ob izbiri. Drugi koraki in možnosti jih lahko preverjajo, da se prikažejo, skrijejo ali postanejo obvezni.
hint-plugin-dependencies = Vzorci, ki spreminjajo vrsto možnosti glede na zastavice ali datoteke v igri: na primer »Obvezna«, ko je nameščen drug mod.
hint-files = Datoteke in mape, ki se ob izbiri te možnosti kopirajo v mapo Data igre. Cilj je relativen glede na Data; ob sporu zmaga višja prednost.
hint-visibility = Pogoji, ki morajo biti izpolnjeni, da se ta korak sploh prikaže. Pustite prazno, da se prikaže vedno.
seltype-exactly-one = Natanko ena (obvezno)
seltype-at-most-one = Največ ena
seltype-any = Poljubno število
seltype-all = Vse (brez izbire)
seltype-at-least-one = Vsaj ena
plugtype-required = Obvezna
plugtype-optional = Izbirna
plugtype-recommended = Priporočena
plugtype-not-usable = Neuporabna
plugtype-could-be-usable = Morda uporabna
plugtype-required-hint = Vedno nameščena; uporabnik je ne more odznačiti.
plugtype-optional-hint = Ponujena neoznačena; uporabnik se odloči.
plugtype-recommended-hint = Ponujena označena; uporabnik jo lahko odznači.
plugtype-not-usable-hint = Prikazana sivo in je ni mogoče izbrati.
plugtype-could-be-usable-hint = Izbirna, vendar namestitveni program opozori, da morda ne bo delovala.
op-and = Vsi pogoji (IN)
op-or = Katerikoli pogoj (ALI)
theme-dark = Temna
theme-light = Svetla
theme-system = Po sistemu
condeditor-setter-loc = Korak { "{step}" } / Skupina { "{group}" } / »{ "{name}" }«
condeditor-pattern-of = Vzorec »{ "{name}" }« → { "{type}" }
condeditor-visibility-of = Vidnost koraka { "{step}" }
condeditor-cond-set = Pogojni nabor { "{num}" }
condeditor-needs = { "{ctx}" } (zahteva = { "{value}" })
condeditor-file-dep = { "{ctx}" }: datoteka »{ "{name}" }« ({ "{state}" })
menu-translate-fomod = Prevedi FOMOD…
ftr-title = Prevedi FOMOD
ftr-open-folder = Odpri mapo moda…
ftr-from-active = Iz aktivnega projekta
ftr-from-active-hint = Prevede FOMOD projekta, odprtega v glavnem oknu (najprej ga je treba shraniti).
ftr-no-fomod = Noben FOMOD ni naložen.
ftr-encoding = Kodiranje izvirnih datotek; prevedene datoteke se zapišejo z enakim kodiranjem.
ftr-source-lang = Iz
ftr-target-lang = v
ftr-lang-locked = (jezika sta po nalaganju FOMOD-a določena)
ftr-translator = Prevajalec:
ftr-save = Shrani prevod
ftr-export = Izvozi prevedene datoteke
ftr-export-sibling = V mapo fomod_<jezik>
ftr-export-sibling-hint = Zapiše prevedeni datoteki info.xml in ModuleConfig.xml ob izvirno mapo fomod; izvirne datoteke ostanejo nedotaknjene.
ftr-export-inplace = Čez izvirne datoteke
ftr-export-inplace-hint = Zamenja fomod/info.xml in fomod/ModuleConfig.xml, potem ko za vsako ustvari kopijo .bak s časovnim žigom.
ftr-force-explicit-order = Ohrani izvirni vrstni red
ftr-warn-order = Sezname, razvrščene po imenu (order="Ascending"), bi upravitelj modov znova razvrstil po prevedenih imenih. Ta možnost vsili order="Explicit", da možnosti ohranijo trenutni vrstni red.
ftr-update = Posodobi iz mape
ftr-update-hint = Znova prebere FOMOD z diska in z njim združi prevod: novi, spremenjeni in odstranjeni nizi so sporočeni.
ftr-preview-translated = Prevedeni predogled
ftr-progress = Prevedeno { $done } / { $total }
ftr-filter-all = Vsi
ftr-filter-untranslated = Neprevedeni
ftr-filter-review = Za pregled
ftr-filter-issues = S težavami
ftr-filter-locked = Zaklenjeni
ftr-type-all = Vsa polja
ftr-type-names = Imena
ftr-type-descriptions = Opisi
ftr-type-meta = Podatki o modu
ftr-search-hint = Išči v izvoru, prevodu ali kontekstu…
ftr-next-untranslated = Naslednji neprevedeni
ftr-show-whitespace = Pokaži presledke in prelome vrstic
ftr-discard-question = Trenutni prevod vsebuje neshranjene spremembe. Ali jih želite zavreči in naložiti drugi FOMOD?
ftr-discard-yes = Zavrzi
ftr-unsaved-close = Prevod vsebuje neshranjene spremembe.
ftr-col-num = Št.
ftr-col-status = { "" }
ftr-col-context = Kontekst
ftr-col-source = Izvor
ftr-col-target = Prevod
ftr-col-issues = { "" }
ftr-empty-hint = Odprite mapo moda ali naložite aktivni projekt, da se prikažejo njegovi prevedljivi nizi.
ftr-empty-filter = Noben niz ne ustreza trenutnemu filtru.
ftr-select-row = Izberite vrstico, da uredite njen prevod.
ftr-copy-source = Kopiraj izvor
ftr-clear-target = Počisti
ftr-lock = Ne prevajaj
ftr-lock-hint = Zaklenjeni nizi se zapišejo nespremenjeni (avtor, spletna stran, lastna imena…).
ftr-note = Opomba:
ftr-status-untranslated = Neprevedeno
ftr-status-translated = Prevedeno
ftr-status-auto = Samodejno predizpolnjeno — preglejte
ftr-status-fuzzy = Izvorno besedilo se je po prevodu spremenilo — preglejte
ftr-status-obsolete = V FOMOD-u ne obstaja več
ftr-status-locked = Zaklenjeno (zapiše se nespremenjeno)
ftr-field-info-name = Ime moda (info.xml)
ftr-field-module-name = Naslov namestitvenega programa (ModuleConfig.xml)
ftr-field-author = Avtor
ftr-field-website = Spletna stran
ftr-field-description = Opis moda
ftr-field-step = Ime koraka
ftr-field-group = Ime skupine
ftr-field-plugin = Ime možnosti
ftr-field-plugin-desc = Opis možnosti
ftr-issue-empty = Prazen prevod
ftr-issue-whitespace = Prevod vsebuje samo presledke
ftr-issue-edge-whitespace = Presledki na začetku ali koncu se razlikujejo od izvora
ftr-issue-token = Zaščiteni žetoni se razlikujejo — manjkajo: { $missing } ; odveč: { $extra }
ftr-issue-newline-name = Ime ne sme vsebovati preloma vrstice
ftr-issue-control = Vsebuje znake, ki jih XML ne more shraniti
ftr-issue-length = Nenavadna dolžina v primerjavi z izvorom (×{ $ratio })
ftr-issue-identical = Enako izvoru
ftr-issue-duplicate = Isto izvorno besedilo je v { $key } prevedeno drugače
ftr-issue-cdata = Zaporedje ]]> tukaj ni dovoljeno
ftr-load-error = FOMOD-a ni bilo mogoče naložiti: { $error }
ftr-extracted = Najdenih prevedljivih nizov: { $num }.
ftr-sidecar-found = Obstoječi prevod naložen in združen: novih { $new }, spremenjenih { $changed }, odstranjenih { $removed }.
ftr-saved = Prevod shranjen v { $path }
ftr-save-error = Prevoda ni bilo mogoče shraniti: { $error }
ftr-save-first = Najprej shranite projekt, nato ga prevedite.
ftr-export-success = Nizov, zapisanih v { $path }: { $count }
ftr-export-error = Izvoz ni uspel: { $error }
ftr-export-blocked = Težav, ki jih je treba odpraviti pred izvozom: { $num }.
ftr-export-stale = Preskočenih nizov, ker se je FOMOD spremenil: { $num }; uporabite »Posodobi iz mape«.
ftr-update-report = Posodobljeno: novih { $new }, spremenjenih { $changed }, premaknjenih { $moved }, odstranjenih { $removed }, nespremenjenih { $unchanged }.
menu-edit = Uredi
menu-undo = Razveljavi
menu-redo = Uveljavi
tree-title = Projekt
tree-mod-info = Informacije o modu
tree-steps = Koraki namestitve
tree-required = Obvezne datoteke
tree-conditional = Pogojne namestitve
tree-empty-steps = Ni še nobenega koraka — kliknite +, da ga dodate.
tree-duplicate = Podvoji
tree-delete = Izbriši
tree-save-template = Shrani kot predlogo…
tree-drop-hint = Spustite tukaj za premik
cond-set-label = Pogojni nabor { $num }
inspector-empty = Izberite element v drevesu projekta ali za začetek dodajte korak.
count-options = Možnosti: { $num }
count-files = Datoteke: { $num }
msg-deleted-undo = Izbrisano. Za obnovitev uporabite Razveljavi (Ctrl+Z).
problems-title = Težave
problems-errors = Napake: { $num }
problems-warnings = Opozorila: { $num }
btn-close = Zapri
ftr-export-package = Kot prevodni paket (arhiv)
ftr-export-package-hint = Ustvari datoteko .zip ali .7z, pripravljeno za nalaganje: prevedeni datoteki info.xml in ModuleConfig.xml ter README (samo popravek) ali celoten mod s prevedenimi datotekami (poln).
ftr-package-full = Celoten mod
ftr-package-full-hint = V arhiv vključi vse datoteke moda, ne le dveh prevedenih datotek XML. Prepričajte se, da avtor dovoljuje nadaljnjo distribucijo.
ftr-package-name-template = Ime:
ftr-readme-patch = Ta arhiv vsebuje prevod namestitvenega programa moda »{ $name }« (jezik: { $langname }; datoteki fomod/info.xml in fomod/ModuleConfig.xml). Namestite ga čez izvirni mod ali pustite, da ga upravitelj modov združi, tako da prevedene datoteke nadomestijo izvirne. Spremenijo se samo besedila namestitvenega programa; datoteke samega moda niso vključene. Ustvarjeno s programom XIMOD Architect.
ftr-readme-full = Ta arhiv vsebuje mod »{ $name }« s prevedenim namestitvenim programom (jezik: { $langname }; datoteki fomod/info.xml in fomod/ModuleConfig.xml). Namestite ga enako kot izvirni mod. Spremenjena so samo besedila namestitvenega programa. Ustvarjeno s programom XIMOD Architect.
ftr-apply-memory = Izpolni iz pomnilnika
ftr-memory-size = Pomnilnik prevodov — vnosov za ta jezikovni par: { $num }. Vanj se doda vsak shranjeni prevod.
ftr-memory-applied = Nizov, izpolnjenih iz pomnilnika prevodov (označeni »za pregled«): { $num }.
ftr-memory-suggestion = Pomnilnik predlaga:
ftr-use-suggestion = Uporabi
ftr-propagate = Razširi na enake
ftr-propagate-hint = Kopira ta prevod v vse druge še neprevedene nize z enakim izvornim besedilom.
ftr-propagated = Izpolnjenih enakih nizov: { $num }.
ftr-csv-export = Izvozi CSV…
ftr-csv-import = Uvozi CSV…
ftr-csv-imported = Nizov, posodobljenih iz datoteke CSV: { $num }.
ftr-csv-error = Napaka CSV: { $error }
ftr-glossary = Glosar
ftr-glossary-source = Izraz
ftr-glossary-target = Prevod
ftr-glossary-case = Velike/male črke
ftr-glossary-dnt = Ohrani
ftr-glossary-add = Dodaj izraz
ftr-issue-glossary = Glosar: »{ $term }« ni preveden po pričakovanjih

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Odpri arhiv…
filter-archive = Arhivi modov (zip, 7z)
msg-archive-opened = Arhiv odprt (razširjenih datotek: { $num }): { $path }
msg-archive-reused = Arhiv je že razširjen, znova se uporabi { $path }
msg-archive-unsupported = Oblika arhiva „.{ $ext }“ ni podprta; najprej ga razširite s 7-Zip (odpreti je mogoče samo .zip in .7z).
msg-archive-error = Napaka pri odpiranju arhiva: { $error }
msg-archive-no-fomod = V arhivu ni bila najdena mapa „fomod“ ({ $path })
msg-archive-extracting = Razširjanje arhiva…
ftr-open-archive = Odpri arhiv moda…
ftr-package-full-partial = Mod je bil odprt iz arhiva, ki vsebuje samo njegovo mapo fomod; celotni paketi zahtevajo razširjen mod.
info-module-deps = Zahteve moda
info-module-deps-hint = Datoteke ali zastavice, ki jih celoten mod zahteva pred zagonom namestitvenega programa (moduleDependencies). Pustite prazno, če jih ni.
info-header-advanced = Napredna glava
info-title-position = Položaj naslova
info-title-colour = Barva naslova
info-title-colour-hint = Pričakovano: šest šestnajstiških števk (RRGGBB)
info-image-show = Pokaži sliko glave
info-image-fade = Pojemanje slike glave
info-image-height = Višina slike glave
info-attr-default = (privzeto)
file-always-install = Vedno
file-always-install-hint = Vedno namesti to datoteko, tudi če možnost ni izbrana (alwaysInstall).
file-install-if-usable = Če je mogoče
file-install-if-usable-hint = Namesti to datoteko, kadar koli je možnost uporabna, tudi če ni izbrana (installIfUsable).
msg-import-lossy = Ta FOMOD vsebuje konstrukte, ki jih XIMOD ne more urejati (število: { $num }); ob shranjevanju projekta bodo zavrženi.
fidelity-nested-deps = Ugnezdena skupina odvisnosti, mesto: { $context } (podprta je samo ena raven)
fidelity-game-dep = Zahteva za različico igre { $version }, mesto: { $context }
fidelity-fomm-dep = Zahteva za različico upravitelja modov { $version }, mesto: { $context }
fidelity-unknown = Element „{ $element }“ v „{ $parent }“ ni podprt ({ $context })
loc-module = zahteve moda
loc-step = korak { $step } „{ $name }“
loc-installer = namestitveni program

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Obnovi varnostno kopijo…
backups-title = Obnovi varnostno kopijo
backups-empty = Ta projekt še nima varnostne kopije. Ustvari se ob vsakem shranjevanju projekta prek prejšnje različice.
backups-changes = Število sprememb glede na trenutni projekt: { $num }
btn-compare = Primerjaj
btn-restore = Obnovi
btn-delete-backups = Izbriši vse varnostne kopije
btn-delete-backups-confirm = Kliknite znova, da izbrišete vse varnostne kopije
msg-backup-restored = Varnostna kopija z dne { $time } obnovljena v urejevalnik (še ni shranjeno; Razveljavi jo povrne)
msg-backups-deleted = Izbrisanih varnostnih kopij: { $num }
settings-backup-count = Število hranjenih varnostnih kopij:
settings-backup-count-hint = Število prejšnjih različic datotek FOMOD XML, ki se ob shranjevanju hranijo v fomod/backups (0 = brez varnostnih kopij).
settings-autosave-minutes = Samodejno shrani obnovitveno kopijo vsakih (minut):
settings-autosave-minutes-hint = V tem intervalu se v mapo s konfiguracijo zapiše obnovitvena kopija vsakega spremenjenega projekta; ob naslednjem zagonu se ponudi le po nepravilnem izhodu (0 = izklopljeno).
settings-auto-masters = Dodaj masterje vtičnika kot pogoje
settings-auto-masters-hint = Ko se vtičnik (.esp/.esm/.esl) doda možnosti, masterji, ki jih zahteva in jih ne zagotavljata niti igra niti ta mod, postanejo datotečni pogoji „Active“ te možnosti.
msg-author-from-plugin = Avtor izpolnjen iz glave vtičnika: { $author }
msg-masters-added = Število masterjev vtičnika { $plugin }, dodanih kot datotečni pogoji: { $num }
issue-missing-master = { $plugin } zahteva { $master }, ki ni v tem modu niti ni naveden kot odvisnost
issue-esl-mismatch-flag = { $plugin } ima končnico .esl, vendar njegova zastavica light (ESL) ni nastavljena
issue-esl-eligible = { $plugin } bi lahko bil označen kot light (novih zapisov: { $num }, omejitev { $limit })
issue-esl-too-big = { $plugin } je označen kot light, vendar ne ustreza pravilom za vtičnike light (novih zapisov: { $num }, omejitev { $limit }, ali FormID zunaj dovoljenega obsega)
menu-plugin-report = Poročilo o vtičnikih…
plugins-title = Poročilo o vtičnikih
plugins-file = Datoteka
plugins-kind = Vrsta
plugins-light = Zastavica light
plugins-masters = Masterji
plugins-new-records = Novi zapisi / omejitev
plugins-eligible = Primeren za light
plugins-empty = Ta projekt ne namesti nobene datoteke vtičnika (.esp, .esm ali .esl).
plugins-unreadable = neberljivo

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Končno drevo datotek
preview-total-size = Skupna velikost namestitve: { $size }
preview-tree-truncated = Drevo je okrnjeno: preveč datotek za razširitev (zgornje velikosti so delne).
preview-overwritten-by = Prepisano z možnostjo { $plugin }
preview-scenario = Scenarij:
preview-scenario-load = Naloži
preview-scenario-save = Shrani…
preview-scenario-delete = Izbriši
preview-scenario-name = Ime scenarija
preview-scenario-saved = Scenarij »{ $name }« shranjen v fomod/scenarios
preview-scenario-unresolved = Izbir scenarija, ki ne ustrezajo nobeni možnosti tega projekta (preimenovana ali odstranjena): { $num }
preview-scenario-none = (ni scenarija)
issue-unreachable-step = Korak »{ $step }« ne more biti nikoli prikazan: njegovi pogoji vidnosti preverjajo vrednost oznake, ki je ne nastavi nobena prejšnja možnost
issue-unreachable-option = Možnosti »{ $plugin }« ni mogoče nikoli izbrati: njeni vzorci uporabnega tipa preverjajo vrednost oznake, ki je ne nastavi nobena možnost
issue-unreachable-cond = Pogojni nabor datotek { $num } se ne more nikoli uveljaviti: njegovi pogoji preverjajo vrednost oznake, ki je ne nastavi nobena možnost
size-option = Velikost namestitve: { $size } (datotek: { $num })
size-missing = Manjkajočih virov: { $num }
size-unknown = Velikost namestitve: — (za meritev zaženite Preveri)
menu-nexus-desc = Opis za Nexus…
nexus-title = Opis za Nexus Mods
nexus-format = Oblika:
nexus-include-requirements = Zahteve
nexus-include-options = Možnosti namestitve
nexus-include-install = Namestitev
nexus-include-changelog = Dnevnik sprememb
nexus-previous = Prejšnja različica…
nexus-previous-none = (ni prejšnje različice: brez dnevnika sprememb)
nexus-language = Jezik:
nexus-language-source = (vir)
nexus-sec-requirements = Zahteve
nexus-sec-options = Možnosti namestitve
nexus-sec-install = Namestitev
nexus-sec-changelog = Dnevnik sprememb
nexus-install-text = Ta mod vsebuje namestitveni program FOMOD: namestite ga z upravljalnikom modov (Vortex, Mod Organizer 2) in izberite možnosti v namestitvenem programu.
nexus-requires = Zahteva
nexus-step = Korak
nexus-added = Dodano
nexus-removed = Odstranjeno
nexus-changed = Spremenjeno
btn-copy = Kopiraj
btn-save-as = Shrani kot…
msg-copied = Kopirano v odložišče
msg-saved-to = Shranjeno v { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Preimenuj…
condeditor-rename-exists = Oznaka z imenom »{ $name }« že obstaja
condeditor-renamed = Oznaka »{ $from }« preimenovana v »{ $to }« (pojavitev: { $num })
condeditor-delete-uses = Izbriši vse uporabe
condeditor-deleted-uses = Oznaka »{ $name }« odstranjena povsod (pojavitev: { $num })
condeditor-values-set = Nastavljene vrednosti:
condeditor-values-tested = Preverjane vrednosti:
condeditor-value-never-set = { $value } — se preverja, a se nikoli ne nastavi
condeditor-value-never-tested = { $value } — se nastavi, a se nikoli ne preverja
condeditor-builder = Graditelj pogojev
condeditor-builder-none = V glavnem oknu izberite korak, možnost, pogojni nabor datotek ali podatke o modu, da tukaj uredite njihove pogoje.
condeditor-builder-pattern = Vzorec:
condeditor-sentence-if = ČE
condeditor-sentence-and = IN
condeditor-sentence-or = ALI
condeditor-sentence-flag = oznaka { "{name}" } = { "{value}" }
condeditor-sentence-file = datoteka { "{name}" } je { "{value}" }
condeditor-sentence-empty = (ni pogoja: vedno res)
condeditor-sentence-then-visible = POTEM se korak prikaže
condeditor-sentence-then-type = POTEM možnost postane { $type }
condeditor-sentence-then-install = POTEM se datoteke namestijo
condeditor-sentence-then-module = POTEM se namestitveni program lahko zažene (preverjeno pred zagonom)
issue-flag-value-never-set = Oznaka »{ $flag }« se preverja z vrednostjo »{ $value }«, ki je ne nastavi nobena možnost
issue-flag-never-used = Oznaka »{ $flag }« se nastavi, a se nikjer ne preverja
menu-project-strings = Nizi projekta…
strings-title = Nizi projekta
strings-search = Išči besedilo, mesto ali ključ…
strings-kind-all = Vsi
strings-kind-names = Imena
strings-kind-descriptions = Opisi
strings-duplicates-only = Samo dvojniki
strings-replace-with = Zamenjaj z:
strings-case = Razlikuj velike in male črke
strings-whole-word = Cela beseda
strings-replace-current = Zamenjaj
strings-replace-all = Zamenjaj vse
strings-replaced = Zamenjanih nizov: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Enako besedilo kot:
strings-count = Nizov: { $num } · skupin dvojnikov: { $dups }
strings-col-location = Mesto
strings-col-field = Polje
strings-col-text = Besedilo

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Vsebina arhiva…
filter-bethesda-archive = Arhivi Bethesda (bsa, ba2)
archive-view-title = Vsebina arhiva
archive-view-format = Oblika:
archive-view-entries = Vnosi: { $num }
archive-view-size = { $size } razpakirano
archive-view-search = Išči pot…
archive-view-col-path = Pot
archive-view-col-size = Velikost
archive-view-col-compressed = Stisnjeno
archive-view-truncated = Prikazanih je le prvih { $num } ujemajočih se vnosov — zožite iskanje.
archive-view-error = Tega arhiva ni mogoče prebrati: { $error }
archive-view-hint = Prikaži vsebino tega arhiva
issue-conflict-archive = Isti vir v več arhivih: „{ $path }“ je zapakiran s { $count } sklici ({ $locs }) — kateri bo uporabljen, odloča vrstni red nalaganja arhivov v igri.
issue-conflict-archive-loose = Arhiv proti prosti datoteki: „{ $path }“ je hkrati zapakiran v arhiv in nameščen kot prosta datoteka ({ $locs }) — prosta datoteka ima prednost pred arhivirano.
preview-in-archive = (v arhivu)
preview-archived-size = od tega { $size } zapakirano v arhivih

# --- Project tree: expand / collapse menus
tree-expand = Razširi
tree-collapse = Strni
tree-expand-all = Razširi vse
tree-expand-selected = Razširi izbrano
tree-expand-from = Razširi od izbranega
tree-collapse-all = Strni vse
tree-collapse-selected = Strni izbrano
tree-collapse-from = Strni od izbranega
tree-expand-all-hint = Razširi vse naslove
tree-expand-selected-hint = Razširi samo izbrani naslov
tree-expand-from-hint = Razširi izbrani naslov in vse pod njim
tree-collapse-all-hint = Strne vse naslove
tree-collapse-selected-hint = Strne samo izbrani naslov
tree-collapse-from-hint = Strne izbrani naslov in vse pod njim

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Dodaj skupino
btn-remove-group-cond = Odstrani skupino
dep-type-game = Različica igre
dep-type-fomm = Različica upravitelja modov
dep-group-hint = Skupina pogojev, povezanih z IN / ALI; skupine so lahko gnezdene.
condeditor-sentence-game = različica igre ≥ { "{value}" }
condeditor-sentence-fomm = različica upravitelja modov ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Edinstvena besedila
ftr-uniques-hint = Prikaže eno vrstico za vsako različno izvorno besedilo. Prevod te vrstice naenkrat prevede vse nize z enakim besedilom.
ftr-uniques-synced = Posodobljenih enakih nizov: { $num }.
ftr-uniques-group = Nizov s tem besedilom: { $num }; njegov prevod velja za vse.
