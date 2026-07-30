# XIMOD Architect - translation metadata
# @language = hrv
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Hrvatski
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Verzija { $version }

# Status messages
status-ready = Spremno
msg-save-success = FOMOD uspješno spremljen
msg-save-error = Pogreška pri spremanju FOMOD-a
msg-export-success = Arhiva za distribuciju stvorena ({ $count } datoteka): { $path }
msg-export-error = Pogreška pri stvaranju arhive za distribuciju: { $error }
msg-load-success = FOMOD uspješno učitano
msg-load-error = Pogreška pri učitavanju FOMOD-a
msg-merge-success = FOMOD uspješno spojljen
msg-merge-error = Pogreška pri spajanju FOMOD-a
msg-no-root-selected = Molimo prvo odaberite korijenski direktorij
msg-no-fomod-folder = Nije pronađen direktorij 'fomod'. Želite li ga stvoriti?
msg-file-outside-root = Datoteka se nalazi izvan korijenskog direktorija

# Menu - File
menu-file = Datoteka
menu-new = Novo
menu-open = Otvori mapu...
menu-open-file = Otvori datoteku...
menu-save = Spremi
menu-recent = Nedavno
menu-exit = Izlaz
menu-merge = Spoj FOMOD...
menu-export = Izvezi arhivu distribucije...

# Menu - Options
menu-options = Opcije
menu-settings = Postavke
menu-pre-save-script = Skripta za spremanje...
menu-post-save-script = Skripta nakon spremanja...
menu-translation = Prevod...

# Menu - Help
menu-help = Pomoć
menu-about = O programu

# Tabs
tab-info = Informacije o modu
tab-steps = Koraci instalacije
tab-required = Potrebne instalacije
tab-conditional = Uvjetne instalacije

# Info Tab
label-workspace = Radni prostor
label-root-dir = Korijenski direktorij:
label-mod-name = Naziv modifikacije:
label-author = Autor:
label-version = Verzija:
label-game-name = Naziv igre:
label-category = Kategorija:
label-url = URL web-stranice:
label-header-image = Slika naslova:
label-description = Opis:
placeholder-select-dir = (Odaberite direktorij)
placeholder-select-game = (Odaberite igru)

# Steps Tab
label-step-name = Naziv koraka:
label-group-name = Naziv grupe:
label-group-type = Vrsta grupe:
label-plugin-name = Naziv dodatka:
label-plugin-desc = Opis:
label-plugin-type = Zadana vrsta:
label-plugin-image = Slika:
label-visibility = Uvjeti vidljivosti
label-operator = Operator:

# Buttons
btn-browse = Pregledaj...
btn-clear = Očisti
btn-add = Dodaj
btn-remove = Ukloni
btn-add-step = Novi korak
btn-delete-step = Izbriši korak
btn-add-group = Dodaj grupu
btn-remove-group = Ukloni grupu
btn-add-plugin = Dodaj dodatak
btn-remove-plugin = Ukloni dodatak
btn-add-file = Dodaj datoteku
btn-add-folder = Dodaj mapu
btn-remove-file = Ukloni
btn-add-flag = Dodaj zastavicu
btn-remove-flag = Ukloni zastavicu
btn-add-condition = Dodaj uvjet
btn-remove-condition = Ukloni uvjet
btn-add-dependency = Dodaj ovisnost
btn-remove-dependency = Ukloni ovisnost
btn-add-pattern = Novi obrazac
btn-remove-pattern = Izbriši obrazac
btn-save = Spremi
btn-cancel = Otkaži
btn-ok = U redu
btn-yes = Da
btn-no = Ne

# Condition/Dependency Labels
label-flag-name = Naziv zastavice:
label-flag-value = Vrijednost:
label-condition-type = Vrsta:
label-condition-name = Naziv:
label-condition-value = Vrijednost:
label-dep-type = Vrsta ovisnosti:
label-dep-name = Naziv/Datoteka:
label-dep-value = Vrijednost/Stanje:

# Files
label-source = Izvor
label-destination = Odredište
label-priority = Prioritet
label-file-type = Tip
label-files = Datoteke
label-dependencies = Ovisnosti

# Settings Dialog
settings-title = Postavke
settings-tab-general = Općenito
settings-tab-recent-files = Nedavne datoteke
settings-language = Jezik:
settings-theme = Tema:
settings-font-size = Veličina fonta:
settings-replace-newlines = Obradi nove redove u opisima
settings-max-recent = Maks. nedavne datoteke:
settings-window-width = Širina prozora:
settings-window-height = Visina prozora:
settings-no-recent-files = Nema nedavnih datoteka.

# Status messages for settings
status-settings-saved = Postavke su uspješno spremljene

# About Dialog
about-title = O programu XIMOD Architect
about-description = Višeplatformski alat za izradu FOMOD instalatera za modove za igre tvrtke Bethesda.
about-license = Pod MIT licencom
about-copyright = © 2025-2026 XIMOD Team
about-credit = Rust port originalnog alata od Wenderer:

# Script Dialog
script-title = Uređivanje skripte
script-info = Skripte se izvršavaju prije ili nakon spremanja. Možete koristiti sljedeće makroze:
script-macros = Dostupni makroi:
macro-modname = $MODNAME$ - Naziv moda
macro-modauthor = $MODAUTHOR$ - Ime autora
macro-modversion = $MODVERSION$ - Verzija moda
macro-modroot = $MODROOT$ - Put do korijenskog direktorija
macro-date = $DATE$ - Trenutni datum (GGGG-MM-DD)
macro-time = $TIME$ - Trenutno vrijeme (HH:MM:SS)
macro-random = $RANDOM$ - Slučajan broj

# Plugin Dependencies
label-default-type = Zadani tip:
label-pattern-type = Tip uzorka:
label-pattern-operator = Operator uzorka:

# Conditional Files
label-pattern = Uzorak

# Validation Messages
validation-no-name = Naziv modula je obavezan
validation-no-steps = Potrebno je najmanje jedan korak ili obavezna datoteka
validation-empty-step = Korak { $num } nema naziv
validation-empty-group = Korak { $step }, grupa { $group } nema naziv
validation-no-plugins = Korak { $step }, grupa "{ $name }" nema dodatke

# File States
state-active = Aktivno
state-inactive = Neaktivno
state-missing = Nedostaje

# Confirmation
confirm-title = Potvrda
confirm-delete = Jeste li sigurni da želite izbrisati ovaj element?
confirm-discard = Imate nepohranjene promjene. Želite li ih odbaciti i nastaviti?
confirm-unsaved = Imate nepohranjene promjene. Želite li spremiti prije zatvaranja?
confirm-save-issues = Projekt ima sljedeće probleme:
confirm-save-anyway = Spremiti unatoč svemu?

# Errors
error-invalid-xml = Neispravna XML datoteka
error-parse-failed = Neuspjelo parsiranje FOMOD-a
error-write-failed = Neuspjelo pisanje datoteke
error-create-dir = Neuspjelo stvaranje direktorija

# Default names (generated when creating new items)
default-step-name = Korak { $num }
default-group-name = Grupa { $num }
default-plugin-name = Plugin { $num }
pattern-label = Uzorak { $num }

# Selection prompts
msg-select-group-first = Prvo odaberite grupu.
msg-select-plugin-edit = Odaberite dodatak za uređivanje.
label-empty = (prazno)
image-no-image = Nema slike

# File dialog filters
filter-images = Slike
filter-xml = XML

# Dependency types
dep-type-flag = Zastavica
dep-type-file = Datoteka

# Status bar
status-modified = Modificirano

# Status messages (errors)
msg-settings-save-error = Greška pri spremanju postavki
msg-script-save-error = Greška pri spremanju skripte

# Translation editor
trans-title = Uređivač prijevoda
trans-source-lang = Prikazani jezik:
trans-target-lang = Jezik za prijevod:
trans-col-key = Ključ
trans-col-source = Izvorni tekst
trans-col-target = Prijevod
trans-saved = Prijevod spremljen
trans-save-error = Pogreška pri spremanju prijevoda

# XML editor
xml-editor-title = XML uređivač
xml-editor-edit = Uredi
xml-editor-apply = Primijeni
xml-editor-revert = Otkaži
xml-editor-readonly = Samo za čitanje
xml-editor-editing = Uređivanje — grafički kartici su zaključani
xml-editor-error = Pogreška:
xml-editor-applied = XML promjene primijenjene
xml-editor-wellformed = Dobro oblikovan XML
xml-editor-error-at = Redak { $line }, stupac { $col }: { $msg }

# Country / flag picker
settings-country-name = Naziv zemlje:
settings-pick-country = Kliknite za odabir svoje zemlje
flags-title = Odaberite zemlju
flags-filter = Filtriraj:
flags-none = Nije pronađena zastava

# Translation editor: country & font
trans-endonym = Endonim zemlje:
trans-font = Font:
trans-no-font = (nema)
trans-browse = Pregledaj…
trans-google-fonts = Google Fonts
trans-pick-country = Kliknite za odabir zemlje
trans-font-outside = Font prvo mora biti instaliran u assets/fonts.
trans-font-dir-missing = Mapu assets/fonts nije moguće pronaći.

# Translation submission
trans-lang-endonym = Endonim jezika:
trans-author = Autor:
trans-submit = Pošalji…
trans-submit-hint = Izradite zip i otvorite unaprijed popunjenu e-poštu
trans-data-updated = Referentni podaci ažurirani (Languages.json / Countries.json)
trans-package-ready = Arhiva je spremna:
trans-package-error = Nije moguće izraditi arhivu:

# ISO 639-3 requirement
trans-lang-not-iso = Prevod je moguć samo za jezik s kodom ISO 639-3.

# FOMOD installer preview
menu-preview = Pregled instalatera…
preview-title = Pregled FOMOD instalatera
preview-refresh = Osvježi
preview-assumptions = Pretpostavke o datotekama
preview-details = Detalji
preview-back = Natrag
preview-next = Sljedeće
preview-install = Instaliraj
preview-close = Zatvori
preview-restart = Ponovno pokreni
preview-summary-title = Datoteke koje će biti instalirane
preview-empty = Niti jedna datoteka neće biti instalirana.
preview-none-option = (ništa)
preview-invalid = Dovršite potrebne odabire da biste nastavili.
preview-no-steps = Nema vidljivih koraka; pogledajte sažetak instalacije.
preview-select-hint = Odaberite opciju da biste vidjeli njezin opis.
preview-col-source = Izvor
preview-col-dest = Odredište
preview-col-priority = Prioritet
preview-sel-exactlyone = Odaberite točno jednu opciju.
preview-sel-atmostone = Odaberite najviše jednu opciju.
preview-sel-any = Odaberite bilo koji broj opcija.
preview-sel-all = Sve su opcije instalirane.
preview-sel-atleastone = Odaberite barem jednu opciju.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validiraj FOMOD
validate-report-title = FOMOD validacija
validate-ok = Nije pronađen problem. FOMOD je u skladu sa šemom.
xml-editor-schema-ok = U skladu je sa šemom ModConfig 5.0.
xml-editor-schema-issues = Problemi sa shemom:
schema-line-col = Redak { $line }, stupac { $col }: { $msg }
schema-wrong-root = Neočekivani korijen "{ $found }" (očekivano "{ $expected }").
schema-unknown = Neočekivani element "{ $element }" u "{ $parent }".
schema-missing = "{ $parent }" mora sadržavati "{ $child }".
schema-needs-one = "{ $parent }" mora sadržavati najmanje jedan "{ $child }".
schema-too-many = "{ $child }" se može pojaviti samo jednom u "{ $parent }".
schema-missing-attr = Atribut "{ $attr }" je obavezan za "{ $element }".
schema-bad-enum = Neispravna vrijednost "{ $value }" za { $element }/@{ $attr } (očekivano: { $allowed }).
schema-choose-one = "{ $parent }" mora sadržavati točno jedan od: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Pomakni prije
reorder-after = Pomakni poslije

# Country / language database explorer (Properties)
menu-properties = Svojstva…
prop-title = Baza podataka o zemljama / jezicima
prop-tab-countries = Zemlje
prop-tab-languages = Jezici
prop-filter = Filtriraj:
prop-official-langs = Službeni jezici
prop-spoken-langs = Govorni jezici
prop-endonym = Endonim države
prop-font = Font
prop-spoken-in = Govori se u
prop-select-country = Odaberite državu da biste vidjeli njezine detalje.
prop-select-lang = Odaberite jezik da biste vidjeli njegove detalje.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Otvori stranicu igre na Nexus Modsu
