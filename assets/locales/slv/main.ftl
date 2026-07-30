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
menu-open = Odpri mapo...
menu-open-file = Odpri datoteko...
menu-save = Shrani
menu-recent = Zadnje
menu-exit = Izhod
menu-merge = Združi FOMOD...
menu-export = Izvozi distribucijski arhiv...

# Menu - Options
menu-options = Možnosti
menu-settings = Nastavitve
menu-pre-save-script = Skript pred shranjevanjem...
menu-post-save-script = Skript po shranjevanju...
menu-translation = Prevod...

# Menu - Help
menu-help = Pomoč
menu-about = O programu

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
label-plugin-name = Ime vtičnika:
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
btn-add-plugin = Dodaj vtičnik
btn-remove-plugin = Odstrani vtičnik
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
validation-no-plugins = Korak { $step }, skupina "{ $name }" nima vtičnikov

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
default-plugin-name = Vtičnik { $num }
pattern-label = Vzorec { $num }

# Selection prompts
msg-select-group-first = Najprej izberite skupino.
msg-select-plugin-edit = Izberite vtičnik za urejanje.
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
