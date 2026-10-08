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
menu-open = Otvori mapu…
menu-open-file = Otvori datoteku…
menu-save = Spremi
menu-recent = Nedavno
menu-exit = Izlaz
menu-merge = Spoj FOMOD…
menu-export = Izvezi arhivu distribucije…
# Menu - Options
menu-options = Opcije
menu-settings = Postavke…
menu-pre-save-script = Skripta za spremanje…
menu-post-save-script = Skripta nakon spremanja…
menu-translation = Prevedi sučelje…
# Menu - Help
menu-help = Pomoć
menu-check-updates = Provjeri ažuriranja…
menu-about = O programu

# Update check
update-checking = Provjera ažuriranja…
update-up-to-date = XIMOD Architect je ažuran.
update-check-failed = Nije moguće provjeriti ažuriranja. Pokušajte ponovno kasnije.
update-available-status = Dostupna je verzija { $version }.
update-banner-text = XIMOD Architect { $version } je dostupan.
update-download = Preuzmi:
update-skip = Preskoči ovu verziju
update-later = Kasnije

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
label-plugin-name = Naziv opcije:
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
btn-add-plugin = Dodaj opciju
btn-remove-plugin = Ukloni opciju
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Odredište za cijelu grupu
label-page-dest = Odredište instalacije (cijela stranica)
btn-apply-group-dest = Primijeni na sve opcije u ovoj grupi
btn-apply-page-dest = Primijeni na sve opcije na ovoj stranici
group-dest-hint = Postavlja jedno odredište instalacije za svaku datoteku svake opcije u ovoj grupi.
page-dest-hint = Postavlja jedno odredište instalacije za svaku datoteku svake opcije na ovoj stranici (sve grupe).
bulk-dest-nofiles = Još nema datoteka za ažuriranje — najprije dodajte datoteke opcijama.
status-dest-applied = Odredište primijenjeno na { $num } datoteku(a).
preview-hidden-steps = { $num } korak(a) skriveno trenutnim odabirom.
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
settings-check-updates = Provjeri ažuriranja pri pokretanju
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
label-plugin-dependencies = Ovisnosti opcije
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
validation-no-plugins = Korak { $step }, grupa "{ $name }" nema opcije

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
default-plugin-name = Opcija { $num }
pattern-label = Uzorak { $num }

# Selection prompts
msg-select-group-first = Prvo odaberite grupu.
msg-select-plugin-edit = Odaberite opciju za uređivanje.
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

# Referenced-file verification (V2)
verify-no-root = Provjera datoteka preskočena: korijenska mapa nije postavljena
loc-header = slika zaglavlja
loc-required = obavezne datoteke
loc-conditional = uvjetni skup { $num }
loc-plugin = korak { $step }, grupa { $group }, opcija „{ $plugin }“
verify-missing-file = Nedostaje datoteka: { $path } ({ $loc })
verify-missing-folder = Nedostaje mapa: { $path } ({ $loc })
verify-missing-image = Nedostaje slika: { $path } ({ $loc })
verify-absolute = Apsolutna putanja (nije prenosiva): { $path } ({ $loc })
verify-outside = Putanja izlazi iz korijenske mape: { $path } ({ $loc })
verify-orphan = Napuštena datoteka (ne koristi je nijedna opcija): { $path }
conflict-certain = Sukob odredišta: „{ $path }“ zapisuje { $count } opcija ({ $locs }) — međusobno se prepisuju.
conflict-potential = Mogući sukob odredišta: „{ $path }“ je odredište { $count } referenci ({ $locs }) — prepisivanje ovisi o odabiru/uvjetima.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Zatvori FOMOD
menu-close-all-fomods = Zatvori sve FOMOD-e
tab-untitled = (bez naslova)
msg-drop-not-fomod = Ispuštena stavka nije FOMOD (mapa „fomod“ nije pronađena)
exit-title = Nespremljene promjene
exit-unsaved = FOMOD nije spremljen. Želite li ga spremiti?
tab-close-hint = Zatvori ovaj FOMOD
menu-new-from-folder = Novo iz mape…
menu-templates = Predlošci…
templates-title = Predlošci za višekratnu upotrebu
templates-empty = Još nema spremljenih predložaka. Spremite gore odabrani korak da biste stvorili predložak.
templates-insert = Umetni
templates-save-step = Spremi odabrani korak
templates-name-hint = Naziv predloška (neobavezno)
msg-wizard-success = Kostur stvoren iz mape: { $num } opcija.
msg-wizard-error = Pogreška: { $error }
msg-template-saved = Predložak spremljen: { $name }
msg-template-inserted = Predložak umetnut u projekt.
msg-template-no-step = Najprije odaberite korak da biste ga spremili kao predložak.
msg-template-no-dir = Nije moguće pronaći mapu predložaka.
msg-drop-assigned = Opciji dodano { $added } izvor(a) ({ $rejected } izvan korijena zanemareno).
menu-compare = Usporedi s…
compare-title = Usporedba FOMOD-a
compare-none = Nema razlika.
btn-optimize-image = Optimiziraj sliku
msg-image-optimized = Slika zaglavlja optimizirana.
msg-image-ok = Slika zaglavlja već je unutar ograničenja.
msg-no-header-image = Nema slike zaglavlja za optimizaciju.
verify-image-large = Slika prevelika ({ $width }×{ $height }): { $path }
verify-image-format = Nepodržani format slike (.{ $ext }): { $path }
verify-image-unreadable = Nečitljiva slika: { $path }
menu-condition-editor = Uređivač uvjeta…
condeditor-title = Uređivač uvjeta
condeditor-set-by = Postavlja:
condeditor-used-by = Koristi:
condeditor-filedeps = Ovisnosti datoteka
condeditor-empty = U ovom projektu nema zastavica ni ovisnosti.
condeditor-orphan-set = postavljeno, ali nikad korišteno
condeditor-orphan-used = korišteno, ali nikad postavljeno
msg-img-optimized = Slika optimizirana.
msg-img-ok = Slika je već unutar ograničenja.
msg-img-none = Nema slike za optimizaciju.
msg-crash-recovery = Prethodna sesija neočekivano je završila. Sigurnosna kopija vašeg projekta spremljena je u { $path }
export-progress-title = Stvaranje distribucijske arhive…
export-progress-files = { $done } / { $total } datoteka
msg-export-cancelled = Izvoz je otkazan; djelomična arhiva je uklonjena.
verify-running = Provjera datoteka na disku…
verify-stale = Napomena: projekt je promijenjen tijekom provjere datoteka; ponovno pokrenite provjeru.
prop-col-name = Naziv
menu-save-as = Spremi kao…
menu-project = Projekt
menu-tools = Alati
menu-manual = Korisnički priručnik
msg-manual-missing = Korisnički priručnik (PDF) nije pronađen uz aplikaciju.
toolbar-new = Novo
toolbar-open = Otvori
toolbar-save = Spremi
toolbar-validate = Provjeri
toolbar-preview = Pregled
toolbar-export = Izvezi
dialog-choose-root = Odaberite korijensku mapu moda
exit-unsaved-docs = Nespremljeno: { $names }
status-summary = { $steps } koraka · { $options } opcija
section-groups = Grupe
section-options = Opcije
section-flags = Zastavice uvjeta
section-files = Datoteke za instalaciju
hint-group-type = Kako instalacijski program korisniku dopušta odabir opcija u ovoj grupi.
hint-default-type = Kako se opcija nudi kada ne odgovara nijedan uzorak ovisnosti: obavezna, neobavezna, preporučena, neupotrebljiva…
hint-operator = Svi uvjeti moraju biti istiniti (I) ili bilo koji od njih (ILI).
hint-flags = Zastavice su imenovane vrijednosti koje ova opcija postavlja kad je odabrana. Drugi koraci i opcije mogu ih provjeravati kako bi se prikazali, sakrili ili postali obavezni.
hint-plugin-dependencies = Uzorci koji mijenjaju vrstu opcije ovisno o zastavicama ili datotekama u igri: na primjer „Obavezna“ kada je instaliran drugi mod.
hint-files = Datoteke i mape kopirane u mapu Data igre kad je ova opcija odabrana. Odredište je relativno u odnosu na Data; kod sukoba pobjeđuje viši prioritet.
hint-visibility = Uvjeti koji moraju biti ispunjeni da bi se ovaj korak uopće prikazao. Ostavite prazno za stalni prikaz.
seltype-exactly-one = Točno jedna (obavezno)
seltype-at-most-one = Najviše jedna
seltype-any = Bilo koji broj
seltype-all = Sve (bez izbora)
seltype-at-least-one = Najmanje jedna
plugtype-required = Obavezna
plugtype-optional = Neobavezna
plugtype-recommended = Preporučena
plugtype-not-usable = Neupotrebljiva
plugtype-could-be-usable = Možda upotrebljiva
plugtype-required-hint = Uvijek se instalira; korisnik je ne može odznačiti.
plugtype-optional-hint = Ponuđena bez oznake; korisnik odlučuje.
plugtype-recommended-hint = Ponuđena označena; korisnik je može odznačiti.
plugtype-not-usable-hint = Prikazana zasivljeno i ne može se odabrati.
plugtype-could-be-usable-hint = Može se odabrati, ali instalacijski program upozorava da možda neće raditi.
op-and = Svi uvjeti (I)
op-or = Bilo koji uvjet (ILI)
theme-dark = Tamna
theme-light = Svijetla
theme-system = Prati sustav
condeditor-setter-loc = Korak { "{step}" } / Grupa { "{group}" } / „{ "{name}" }“
condeditor-pattern-of = Uzorak „{ "{name}" }“ → { "{type}" }
condeditor-visibility-of = Vidljivost koraka { "{step}" }
condeditor-cond-set = Uvjetni skup { "{num}" }
condeditor-needs = { "{ctx}" } (zahtijeva = { "{value}" })
condeditor-file-dep = { "{ctx}" }: datoteka „{ "{name}" }“ ({ "{state}" })
menu-translate-fomod = Prevedi FOMOD…
ftr-title = Prevedi FOMOD
ftr-open-folder = Otvori mapu moda…
ftr-from-active = Iz aktivnog projekta
ftr-from-active-hint = Prevodi FOMOD projekta otvorenog u glavnom prozoru (najprije ga treba spremiti).
ftr-no-fomod = Nije učitan nijedan FOMOD.
ftr-encoding = Kodiranje izvornih datoteka; prevedene datoteke zapisuju se istim kodiranjem.
ftr-source-lang = S
ftr-target-lang = na
ftr-lang-locked = (jezici su nepromjenjivi nakon učitavanja FOMOD-a)
ftr-translator = Prevoditelj:
ftr-save = Spremi prijevod
ftr-export = Izvezi prevedene datoteke
ftr-export-sibling = U mapu fomod_<jezik>
ftr-export-sibling-hint = Zapisuje prevedene datoteke info.xml i ModuleConfig.xml pokraj izvorne mape fomod; izvorne datoteke ostaju netaknute.
ftr-export-inplace = Preko izvornih datoteka
ftr-export-inplace-hint = Zamjenjuje fomod/info.xml i fomod/ModuleConfig.xml nakon izrade .bak kopije svake datoteke s vremenskom oznakom.
ftr-force-explicit-order = Zadrži izvorni redoslijed
ftr-warn-order = Popise poredane po nazivu (order="Ascending") upravitelj modova ponovno bi poredao prema prevedenim nazivima. Ova opcija nameće order="Explicit" kako bi opcije zadržale trenutačni redoslijed.
ftr-update = Ažuriraj iz mape
ftr-update-hint = Ponovno učitava FOMOD s diska i spaja prijevod s njim: prijavljuju se novi, izmijenjeni i uklonjeni nizovi.
ftr-preview-translated = Prevedeni pregled
ftr-progress = Prevedeno { $done } / { $total }
ftr-filter-all = Svi
ftr-filter-untranslated = Neprevedeni
ftr-filter-review = Za pregled
ftr-filter-issues = S problemima
ftr-filter-locked = Zaključani
ftr-type-all = Sva polja
ftr-type-names = Nazivi
ftr-type-descriptions = Opisi
ftr-type-meta = Podaci o modu
ftr-search-hint = Traži u izvoru, prijevodu ili kontekstu…
ftr-next-untranslated = Sljedeći neprevedeni
ftr-show-whitespace = Prikaži razmake i prijelome redaka
ftr-discard-question = Trenutačni prijevod sadrži nespremljene izmjene. Odbaciti ih i učitati drugi FOMOD?
ftr-discard-yes = Odbaci
ftr-unsaved-close = Prijevod sadrži nespremljene izmjene.
ftr-col-num = Br.
ftr-col-status = { "" }
ftr-col-context = Kontekst
ftr-col-source = Izvor
ftr-col-target = Prijevod
ftr-col-issues = { "" }
ftr-empty-hint = Otvorite mapu moda ili učitajte aktivni projekt da biste prikazali njegove prevodive nizove.
ftr-empty-filter = Nijedan niz ne odgovara trenutačnom filtru.
ftr-select-row = Odaberite redak da biste uredili njegov prijevod.
ftr-copy-source = Kopiraj izvor
ftr-clear-target = Očisti
ftr-lock = Ne prevodi
ftr-lock-hint = Zaključani nizovi zapisuju se nepromijenjeni (autor, web-stranica, vlastita imena…).
ftr-note = Napomena:
ftr-status-untranslated = Neprevedeno
ftr-status-translated = Prevedeno
ftr-status-auto = Automatski unaprijed popunjeno — pregledajte
ftr-status-fuzzy = Izvorni tekst promijenio se nakon prijevoda — pregledajte
ftr-status-obsolete = Više ne postoji u FOMOD-u
ftr-status-locked = Zaključano (zapisuje se nepromijenjeno)
ftr-field-info-name = Naziv moda (info.xml)
ftr-field-module-name = Naslov instalatera (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Web-stranica
ftr-field-description = Opis moda
ftr-field-step = Naziv koraka
ftr-field-group = Naziv grupe
ftr-field-plugin = Naziv opcije
ftr-field-plugin-desc = Opis opcije
ftr-issue-empty = Prazan prijevod
ftr-issue-whitespace = Prijevod sadrži samo razmake
ftr-issue-edge-whitespace = Razmaci na početku ili na kraju razlikuju se od izvora
ftr-issue-token = Zaštićeni tokeni se razlikuju — nedostaju: { $missing } ; suvišni: { $extra }
ftr-issue-newline-name = Naziv ne smije sadržavati prijelom retka
ftr-issue-control = Sadrži znakove koje XML ne može pohraniti
ftr-issue-length = Neuobičajena duljina u odnosu na izvor (×{ $ratio })
ftr-issue-identical = Istovjetno izvoru
ftr-issue-duplicate = Isti izvorni tekst drukčije je preveden u { $key }
ftr-issue-cdata = Slijed ]]> ovdje nije dopušten
ftr-load-error = FOMOD nije moguće učitati: { $error }
ftr-extracted = Pronađeno prevodivih nizova: { $num }.
ftr-sidecar-found = Postojeći prijevod učitan i spojen: novih { $new }, izmijenjenih { $changed }, uklonjenih { $removed }.
ftr-saved = Prijevod spremljen u { $path }
ftr-save-error = Prijevod nije moguće spremiti: { $error }
ftr-save-first = Najprije spremite projekt, a zatim ga prevedite.
ftr-export-success = Nizova zapisanih u { $path }: { $count }
ftr-export-error = Izvoz nije uspio: { $error }
ftr-export-blocked = Blokirajućih problema koje treba ispraviti prije izvoza: { $num }.
ftr-export-stale = Preskočeno nizova jer se FOMOD promijenio: { $num }; upotrijebite „Ažuriraj iz mape“.
ftr-update-report = Ažurirano: novih { $new }, izmijenjenih { $changed }, premještenih { $moved }, uklonjenih { $removed }, nepromijenjenih { $unchanged }.
menu-edit = Uredi
menu-undo = Poništi
menu-redo = Ponovi
tree-title = Projekt
tree-mod-info = Informacije o modu
tree-steps = Koraci instalacije
tree-required = Obavezne datoteke
tree-conditional = Uvjetne instalacije
tree-empty-steps = Još nema koraka — kliknite + da biste dodali korak.
tree-duplicate = Dupliciraj
tree-delete = Izbriši
tree-save-template = Spremi kao predložak…
tree-drop-hint = Ispustite ovdje za premještanje
cond-set-label = Uvjetni skup { $num }
inspector-empty = Odaberite stavku u stablu projekta ili dodajte korak za početak.
count-options = Opcije: { $num }
count-files = Datoteke: { $num }
msg-deleted-undo = Izbrisano. Upotrijebite Poništi (Ctrl+Z) za vraćanje.
problems-title = Problemi
problems-errors = Pogreške: { $num }
problems-warnings = Upozorenja: { $num }
btn-close = Zatvori
ftr-export-package = Kao prijevodni paket (arhiva)
ftr-export-package-hint = Izrađuje .zip ili .7z spreman za prijenos: prevedene datoteke info.xml i ModuleConfig.xml te README (samo zakrpa) ili cijeli mod s prevedenim datotekama (potpuni).
ftr-package-full = Cijeli mod
ftr-package-full-hint = U arhivu uključuje sve datoteke moda, a ne samo dvije prevedene XML datoteke. Provjerite dopušta li autor daljnju distribuciju.
ftr-package-name-template = Naziv:
ftr-readme-patch = Ova arhiva sadrži prijevod instalatera moda „{ $name }“ (jezik: { $langname }; datoteke fomod/info.xml i fomod/ModuleConfig.xml). Instalirajte je preko izvornog moda ili prepustite upravitelju modova da je spoji, kako bi prevedene datoteke zamijenile izvorne. Mijenjaju se samo tekstovi instalatera; same datoteke moda nisu uključene. Izrađeno u programu XIMOD Architect.
ftr-readme-full = Ova arhiva sadrži mod „{ $name }“ s prevedenim instalaterom (jezik: { $langname }; datoteke fomod/info.xml i fomod/ModuleConfig.xml). Instalirajte je kao izvorni mod. Promijenjeni su samo tekstovi instalatera. Izrađeno u programu XIMOD Architect.
ftr-apply-memory = Popuni iz memorije
ftr-memory-size = Prijevodna memorija — unosa za ovaj jezični par: { $num }. U nju se dodaje svaki spremljeni prijevod.
ftr-memory-applied = Nizova popunjenih iz prijevodne memorije (označeni „za pregled“): { $num }.
ftr-memory-suggestion = Memorija predlaže:
ftr-use-suggestion = Upotrijebi
ftr-propagate = Proširi na identične
ftr-propagate-hint = Kopira ovaj prijevod u sve ostale još neprevedene nizove s istim izvornim tekstom.
ftr-propagated = Popunjeno identičnih nizova: { $num }.
ftr-csv-export = Izvezi CSV…
ftr-csv-import = Uvezi CSV…
ftr-csv-imported = Nizova ažuriranih iz CSV datoteke: { $num }.
ftr-csv-error = CSV pogreška: { $error }
ftr-glossary = Pojmovnik
ftr-glossary-source = Pojam
ftr-glossary-target = Prijevod
ftr-glossary-case = Velika/mala slova
ftr-glossary-dnt = Zadrži
ftr-glossary-add = Dodaj pojam
ftr-issue-glossary = Pojmovnik: „{ $term }“ nije preveden prema očekivanju

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Otvori arhivu…
filter-archive = Arhive modova (zip, 7z)
msg-archive-opened = Arhiva otvorena (raspakiranih datoteka: { $num }): { $path }
msg-archive-reused = Arhiva je već raspakirana, ponovno se koristi { $path }
msg-archive-unsupported = Format arhive „.{ $ext }“ nije podržan; najprije je raspakirajte pomoću 7-Zip (mogu se otvoriti samo .zip i .7z).
msg-archive-error = Greška pri otvaranju arhive: { $error }
msg-archive-no-fomod = U arhivi nije pronađena mapa „fomod“ ({ $path })
msg-archive-extracting = Raspakiravanje arhive…
ftr-open-archive = Otvori arhivu moda…
ftr-package-full-partial = Mod je otvoren iz arhive koja sadrži samo njegovu mapu fomod; potpuni paketi zahtijevaju raspakirani mod.
info-module-deps = Zahtjevi moda
info-module-deps-hint = Datoteke ili zastavice koje cijeli mod zahtijeva prije pokretanja instalacijskog programa (moduleDependencies). Ostavite prazno ako ih nema.
info-header-advanced = Napredno zaglavlje
info-title-position = Položaj naslova
info-title-colour = Boja naslova
info-title-colour-hint = Očekuje se: šest heksadecimalnih znamenki (RRGGBB)
info-image-show = Prikaži sliku zaglavlja
info-image-fade = Pretapanje slike zaglavlja
info-image-height = Visina slike zaglavlja
info-attr-default = (zadano)
file-always-install = Uvijek
file-always-install-hint = Uvijek instaliraj ovu datoteku, čak i kada opcija nije odabrana (alwaysInstall).
file-install-if-usable = Ako može
file-install-if-usable-hint = Instaliraj ovu datoteku kad god je opcija upotrebljiva, čak i kada nije odabrana (installIfUsable).
msg-import-lossy = Ovaj FOMOD sadrži konstrukte koje XIMOD ne može uređivati (broj: { $num }); bit će odbačeni pri spremanju projekta.
fidelity-nested-deps = Ugniježđena grupa ovisnosti, mjesto: { $context } (podržana je samo jedna razina)
fidelity-game-dep = Zahtjev za verziju igre { $version }, mjesto: { $context }
fidelity-fomm-dep = Zahtjev za verziju upravitelja modova { $version }, mjesto: { $context }
fidelity-unknown = Element „{ $element }“ u „{ $parent }“ nije podržan ({ $context })
loc-module = zahtjevi moda
loc-step = korak { $step } „{ $name }“
loc-installer = instalacijski program

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Vrati sigurnosnu kopiju…
backups-title = Vrati sigurnosnu kopiju
backups-empty = Ovaj projekt još nema sigurnosnu kopiju. Stvara se pri svakom spremanju projekta preko prethodne verzije.
backups-changes = Broj promjena u odnosu na trenutni projekt: { $num }
btn-compare = Usporedi
btn-restore = Vrati
btn-delete-backups = Izbriši sve sigurnosne kopije
btn-delete-backups-confirm = Kliknite ponovno za brisanje svih sigurnosnih kopija
msg-backup-restored = Sigurnosna kopija od { $time } vraćena u uređivač (još nije spremljeno; Poništi je vraća)
msg-backups-deleted = Izbrisanih sigurnosnih kopija: { $num }
settings-backup-count = Broj sigurnosnih kopija za čuvanje:
settings-backup-count-hint = Broj prethodnih verzija FOMOD XML datoteka koje se pri spremanju čuvaju u fomod/backups (0 = bez sigurnosnih kopija).
settings-autosave-minutes = Automatski spremaj kopiju za oporavak svakih (minuta):
settings-autosave-minutes-hint = U ovom se intervalu u mapu konfiguracije zapisuje kopija za oporavak svakog izmijenjenog projekta; pri sljedećem pokretanju nudi se samo nakon neispravnog izlaza (0 = isključeno).
settings-auto-masters = Dodaj mastere plugina kao uvjete
settings-auto-masters-hint = Kada se plugin (.esp/.esm/.esl) doda opciji, masteri koje zahtijeva, a koje ne pružaju ni igra ni ovaj mod, postaju datotečni uvjeti „Active“ te opcije.
msg-author-from-plugin = Autor popunjen iz zaglavlja plugina: { $author }
msg-masters-added = Broj mastera plugina { $plugin } dodanih kao datotečni uvjeti: { $num }
issue-missing-master = { $plugin } zahtijeva { $master }, koji nije u ovom modu niti je deklariran kao ovisnost
issue-esl-mismatch-flag = { $plugin } ima nastavak .esl, ali njegova light (ESL) zastavica nije postavljena
issue-esl-eligible = { $plugin } mogao bi se označiti kao light (novih zapisa: { $num }, limit { $limit })
issue-esl-too-big = { $plugin } označen je kao light, ali ne zadovoljava pravila za light plugine (novih zapisa: { $num }, limit { $limit }, ili FormID izvan dopuštenog raspona)
menu-plugin-report = Izvještaj o pluginima…
plugins-title = Izvještaj o pluginima
plugins-file = Datoteka
plugins-kind = Vrsta
plugins-light = Light zastavica
plugins-masters = Masteri
plugins-new-records = Novi zapisi / limit
plugins-eligible = Može biti light
plugins-empty = Ovaj projekt ne instalira nijednu datoteku plugina (.esp, .esm ili .esl).
plugins-unreadable = nečitljivo

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Konačno stablo datoteka
preview-total-size = Ukupna veličina instalacije: { $size }
preview-tree-truncated = Stablo je skraćeno: previše datoteka za prikaz (gornje veličine su djelomične).
preview-overwritten-by = Prepisano opcijom { $plugin }
preview-scenario = Scenarij:
preview-scenario-load = Učitaj
preview-scenario-save = Spremi…
preview-scenario-delete = Izbriši
preview-scenario-name = Naziv scenarija
preview-scenario-saved = Scenarij "{ $name }" spremljen u fomod/scenarios
preview-scenario-unresolved = Odabira iz scenarija koji ne odgovaraju nijednoj opciji ovog projekta (preimenovana ili uklonjena): { $num }
preview-scenario-none = (nema scenarija)
issue-unreachable-step = Korak "{ $step }" nikada se ne može prikazati: njegovi uvjeti vidljivosti provjeravaju vrijednost zastavice koju nijedna ranija opcija ne postavlja
issue-unreachable-option = Opcija "{ $plugin }" nikada se ne može odabrati: njezini uzorci upotrebljivog tipa provjeravaju vrijednost zastavice koju nijedna opcija ne postavlja
issue-unreachable-cond = Uvjetni skup datoteka { $num } nikada se ne može primijeniti: njegovi uvjeti provjeravaju vrijednost zastavice koju nijedna opcija ne postavlja
size-option = Veličina instalacije: { $size } (datoteka: { $num })
size-missing = Nedostajućih izvora: { $num }
size-unknown = Veličina instalacije: — (pokrenite Validiraj za mjerenje)
menu-nexus-desc = Opis za Nexus…
nexus-title = Opis za Nexus Mods
nexus-format = Format:
nexus-include-requirements = Zahtjevi
nexus-include-options = Opcije instalacije
nexus-include-install = Instalacija
nexus-include-changelog = Popis promjena
nexus-previous = Prethodna verzija…
nexus-previous-none = (nema prethodne verzije: bez popisa promjena)
nexus-language = Jezik:
nexus-language-source = (izvor)
nexus-sec-requirements = Zahtjevi
nexus-sec-options = Opcije instalacije
nexus-sec-install = Instalacija
nexus-sec-changelog = Popis promjena
nexus-install-text = Ovaj mod dolazi s FOMOD instalaterom: instalirajte ga upraviteljem modova (Vortex, Mod Organizer 2) i odaberite svoje opcije u instalateru.
nexus-requires = Zahtijeva
nexus-step = Korak
nexus-added = Dodano
nexus-removed = Uklonjeno
nexus-changed = Promijenjeno
btn-copy = Kopiraj
btn-save-as = Spremi kao…
msg-copied = Kopirano u međuspremnik
msg-saved-to = Spremljeno u { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Preimenuj…
condeditor-rename-exists = Zastavica naziva "{ $name }" već postoji
condeditor-renamed = Zastavica "{ $from }" preimenovana u "{ $to }" (pojavljivanja: { $num })
condeditor-delete-uses = Izbriši sve upotrebe
condeditor-deleted-uses = Zastavica "{ $name }" uklonjena posvuda (pojavljivanja: { $num })
condeditor-values-set = Postavljene vrijednosti:
condeditor-values-tested = Provjeravane vrijednosti:
condeditor-value-never-set = { $value } — provjerava se, ali nikad ne postavlja
condeditor-value-never-tested = { $value } — postavlja se, ali nikad ne provjerava
condeditor-builder = Graditelj uvjeta
condeditor-builder-none = Odaberite korak, opciju, uvjetni skup datoteka ili podatke o modu u glavnom prozoru kako biste ovdje uredili njihove uvjete.
condeditor-builder-pattern = Uzorak:
condeditor-sentence-if = AKO
condeditor-sentence-and = I
condeditor-sentence-or = ILI
condeditor-sentence-flag = zastavica { "{name}" } = { "{value}" }
condeditor-sentence-file = datoteka { "{name}" } je { "{value}" }
condeditor-sentence-empty = (nema uvjeta: uvijek istinito)
condeditor-sentence-then-visible = ONDA se korak prikazuje
condeditor-sentence-then-type = ONDA opcija postaje { $type }
condeditor-sentence-then-install = ONDA se datoteke instaliraju
condeditor-sentence-then-module = ONDA se instalater može pokrenuti (provjerava se prije pokretanja)
issue-flag-value-never-set = Zastavica "{ $flag }" provjerava se s vrijednošću "{ $value }" koju nijedna opcija ne postavlja
issue-flag-never-used = Zastavica "{ $flag }" postavlja se, ali se nigdje ne provjerava
menu-project-strings = Tekstovi projekta…
strings-title = Tekstovi projekta
strings-search = Pretraži tekst, lokaciju ili ključ…
strings-kind-all = Sve
strings-kind-names = Nazivi
strings-kind-descriptions = Opisi
strings-duplicates-only = Samo duplikati
strings-replace-with = Zamijeni s:
strings-case = Razlikuj velika i mala slova
strings-whole-word = Cijela riječ
strings-replace-current = Zamijeni
strings-replace-all = Zamijeni sve
strings-replaced = Zamijenjenih tekstova: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Isti tekst kao:
strings-count = Tekstova: { $num } · grupa duplikata: { $dups }
strings-col-location = Lokacija
strings-col-field = Polje
strings-col-text = Tekst

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Sadržaj arhive…
filter-bethesda-archive = Bethesda arhive (bsa, ba2)
archive-view-title = Sadržaj arhive
archive-view-format = Format:
archive-view-entries = Stavke: { $num }
archive-view-size = { $size } raspakirano
archive-view-search = Pretraži putanju…
archive-view-col-path = Putanja
archive-view-col-size = Veličina
archive-view-col-compressed = Komprimirano
archive-view-truncated = Prikazano je samo prvih { $num } odgovarajućih stavki — suzite pretragu.
archive-view-error = Ovu arhivu nije moguće pročitati: { $error }
archive-view-hint = Prikaži sadržaj ove arhive
issue-conflict-archive = Isti resurs u više arhiva: „{ $path }“ pakiran je putem { $count } referenci ({ $locs }) — redoslijed učitavanja arhiva u igri odlučuje koja se koristi.
issue-conflict-archive-loose = Arhiva nasuprot slobodnoj datoteci: „{ $path }“ istovremeno je pakiran u arhivu i instaliran kao slobodna datoteka ({ $locs }) — slobodna datoteka ima prednost pred arhiviranom.
preview-in-archive = (u arhivi)
preview-archived-size = od čega { $size } pakirano u arhivama

# --- Project tree: expand / collapse menus
tree-expand = Proširi
tree-collapse = Sažmi
tree-expand-all = Proširi sve
tree-expand-selected = Proširi odabrano
tree-expand-from = Proširi od odabranog
tree-collapse-all = Sažmi sve
tree-collapse-selected = Sažmi odabrano
tree-collapse-from = Sažmi od odabranog
tree-expand-all-hint = Proširuje sve naslove
tree-expand-selected-hint = Proširuje samo odabrani naslov
tree-expand-from-hint = Proširuje odabrani naslov i sve ispod njega
tree-collapse-all-hint = Sažima sve naslove
tree-collapse-selected-hint = Sažima samo odabrani naslov
tree-collapse-from-hint = Sažima odabrani naslov i sve ispod njega

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Dodaj grupu
btn-remove-group-cond = Ukloni grupu
dep-type-game = Verzija igre
dep-type-fomm = Verzija upravitelja modova
dep-group-hint = Grupa uvjeta povezanih s I / ILI; grupe se mogu ugnijezditi.
condeditor-sentence-game = verzija igre ≥ { "{value}" }
condeditor-sentence-fomm = verzija upravitelja modova ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Jedinstveni tekstovi
ftr-uniques-hint = Prikazuje jedan redak po različitom izvornom tekstu. Prijevod tog retka odjednom prevodi sve nizove s istim tekstom.
ftr-uniques-synced = Ažurirano identičnih nizova: { $num }.
ftr-uniques-group = Nizova s ovim tekstom: { $num }; njegov prijevod primjenjuje se na sve njih.
