# XIMOD Architect - translation metadata
# @language = ron
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Română
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versiunea { $version }

# Status messages
status-ready = Gata
msg-save-success = FOMOD salvat cu succes
msg-save-error = Eroare la salvarea FOMOD
msg-export-success = Arhivă de distribuție creată ({ $count } fișiere): { $path }
msg-export-error = Eroare la crearea arhivei de distribuție: { $error }
msg-load-success = FOMOD încărcat cu succes
msg-load-error = Eroare la încărcarea FOMOD
msg-merge-success = FOMOD îmbinat cu succes
msg-merge-error = Eroare la îmbinarea FOMOD
msg-no-root-selected = Selectați mai întâi un director rădăcină
msg-no-fomod-folder = Nu s-a găsit folderul „fomod”. Îl creați?
msg-file-outside-root = Fișierul este în afara directorului rădăcină

# Menu - File
menu-file = Fișier
menu-new = Nou
menu-open = Deschide folder…
menu-open-file = Deschide fișier…
menu-save = Salvează
menu-recent = Recente
menu-exit = Ieșire
menu-merge = Îmbină FOMOD…
menu-export = Exportă arhiva de distribuție...

# Menu - Options
menu-options = Opțiuni
menu-settings = Setări
menu-pre-save-script = Script înainte de salvare…
menu-post-save-script = Script după salvare…
menu-translation = Traducere...

# Menu - Help
menu-help = Ajutor
menu-about = Despre

# Tabs
tab-info = Informații mod
tab-steps = Pași de instalare
tab-required = Instalări obligatorii
tab-conditional = Instalări condiționate

# Info Tab
label-workspace = Spațiu de lucru
label-root-dir = Director rădăcină:
label-mod-name = Nume mod:
label-author = Autor:
label-version = Versiune:
label-game-name = Nume joc:
label-category = Categorie:
label-url = URL site web:
label-header-image = Imagine antet:
label-description = Descriere:
placeholder-select-dir = (Selectați un director)
placeholder-select-game = (Selectați un joc)

# Steps Tab
label-step-name = Nume pas:
label-group-name = Nume grup:
label-group-type = Tip grup:
label-plugin-name = Nume plugin:
label-plugin-desc = Descriere:
label-plugin-type = Tip implicit:
label-plugin-image = Imagine:
label-visibility = Condiții de vizibilitate
label-operator = Operator:

# Buttons
btn-browse = Răsfoiește…
btn-clear = Golește
btn-add = Adaugă
btn-remove = Elimină
btn-add-step = Pas nou
btn-delete-step = Șterge pasul
btn-add-group = Adaugă grup
btn-remove-group = Elimină grupul
btn-add-plugin = Adaugă plugin
btn-remove-plugin = Elimină pluginul
btn-add-file = Adaugă fișier
btn-add-folder = Adaugă folder
btn-remove-file = Elimină
btn-add-flag = Adaugă marcaj
btn-remove-flag = Elimină marcajul
btn-add-condition = Adaugă condiție
btn-remove-condition = Elimină condiția
btn-add-dependency = Adaugă dependență
btn-remove-dependency = Elimină dependența
btn-add-pattern = Model nou
btn-remove-pattern = Șterge modelul
btn-save = Salvează
btn-cancel = Anulează
btn-ok = OK
btn-yes = Da
btn-no = Nu

# Condition/Dependency Labels
label-flag-name = Nume marcaj:
label-flag-value = Valoare:
label-condition-type = Tip:
label-condition-name = Nume:
label-condition-value = Valoare:
label-dep-type = Tip dependență:
label-dep-name = Nume/fișier:
label-dep-value = Valoare/stare:

# Files
label-source = Sursă
label-destination = Destinație
label-priority = Prioritate
label-file-type = Tip
label-files = Fișiere
label-dependencies = Dependențe

# Settings Dialog
settings-title = Setări
settings-tab-general = General
settings-tab-recent-files = Fișiere recente
settings-language = Limbă:
settings-theme = Temă:
settings-font-size = Dimensiune font:
settings-replace-newlines = Procesează întreruperile de rând în descrieri
settings-max-recent = Max. fișiere recente:
settings-window-width = Lățime fereastră:
settings-window-height = Înălțime fereastră:
settings-no-recent-files = Niciun fișier recent.

# Status messages for settings
status-settings-saved = Setări salvate cu succes

# About Dialog
about-title = Despre XIMOD Architect
about-description = Un instrument multiplatformă pentru crearea de instalatoare FOMOD pentru moduri de jocuri Bethesda.
about-license = Licențiat sub licența MIT
about-copyright = © 2024 XIMOD Team
about-credit = Portare Rust a sculei originale de la Wenderer:

# Script Dialog
script-title = Editează scriptul
script-info = Scripturile sunt executate înainte sau după salvare. Puteți folosi următoarele macrocomenzi:
script-macros = Macrocomenzi disponibile:
macro-modname = $MODNAME$ - Nume mod
macro-modauthor = $MODAUTHOR$ - Nume autor
macro-modversion = $MODVERSION$ - Versiune mod
macro-modroot = $MODROOT$ - Cale director rădăcină
macro-date = $DATE$ - Data curentă (AAAA-LL-ZZ)
macro-time = $TIME$ - Ora curentă (HH:MM:SS)
macro-random = $RANDOM$ - Număr aleatoriu

# Plugin Dependencies
label-default-type = Tip implicit:
label-pattern-type = Tip model:
label-pattern-operator = Operator model:

# Conditional Files
label-pattern = Model

# Validation Messages
validation-no-name = Numele modului este obligatoriu
validation-no-steps = Este necesar cel puțin un pas sau un fișier obligatoriu
validation-empty-step = Pasul { $num } nu are nume
validation-empty-group = Pasul { $step }, grupul { $group } nu are nume
validation-no-plugins = Pasul { $step }, grupul „{ $name }” nu are pluginuri

# File States
state-active = Activ
state-inactive = Inactiv
state-missing = Lipsește

# Confirmation
confirm-title = Confirmare
confirm-delete = Sigur doriți să ștergeți acest element?
confirm-discard = Aveți modificări nesalvate. Le eliminați și continuați?
confirm-unsaved = Aveți modificări nesalvate. Doriți să salvați înainte de închidere?
confirm-save-issues = Proiectul are următoarele probleme:
confirm-save-anyway = Salvați oricum?

# Errors
error-invalid-xml = Fișier XML nevalid
error-parse-failed = Analizarea FOMOD a eșuat
error-write-failed = Scrierea fișierului a eșuat
error-create-dir = Crearea directorului a eșuat

# Default names (generated when creating new items)
default-step-name = Pasul { $num }
default-group-name = Grupul { $num }
default-plugin-name = Plugin { $num }
pattern-label = Model { $num }

# Selection prompts
msg-select-group-first = Selectați mai întâi un grup.
msg-select-plugin-edit = Selectați un plugin pentru editare.
label-empty = (gol)
image-no-image = Fără imagine

# File dialog filters
filter-images = Imagini
filter-xml = XML

# Dependency types
dep-type-flag = Marcaj
dep-type-file = Fișier

# Status bar
status-modified = Modificat

# Status messages (errors)
msg-settings-save-error = Eroare la salvarea setărilor
msg-script-save-error = Eroare la salvarea scriptului

# Translation editor
trans-title = Editor de traduceri
trans-source-lang = Limbă afișată:
trans-target-lang = Limbă de tradus:
trans-col-key = Cheie
trans-col-source = Etichetă
trans-col-target = Traducere
trans-saved = Traducere salvată
trans-save-error = Eroare la salvarea traducerii

# XML editor
xml-editor-title = Editor XML
xml-editor-edit = Editează
xml-editor-apply = Aplică
xml-editor-revert = Anulează
xml-editor-readonly = Doar citire
xml-editor-editing = Editare — filele grafice sunt blocate
xml-editor-error = Eroare:
xml-editor-applied = Modificările XML au fost aplicate
xml-editor-wellformed = XML bine format
xml-editor-error-at = Linia { $line }, coloana { $col }: { $msg }

# Country / flag picker
settings-country-name = Numele țării:
settings-pick-country = Faceți clic pentru a alege țara
flags-title = Alegeți o țară
flags-filter = Filtru:
flags-none = Niciun steag găsit

# Translation editor: country & font
trans-endonym = Endonimul țării:
trans-font = Font:
trans-no-font = (niciunul)
trans-browse = Răsfoiește…
trans-google-fonts = Google Fonts
trans-pick-country = Faceți clic pentru a alege țara
trans-font-outside = Fontul trebuie mai întâi instalat în assets/fonts.
trans-font-dir-missing = Folderul assets/fonts nu a fost găsit.

# Translation submission
trans-lang-endonym = Endonimul limbii:
trans-author = Autor:
trans-submit = Trimite…
trans-submit-hint = Construiește un zip și deschide un e-mail precompletat
trans-data-updated = Datele de referință au fost actualizate (Languages.json / Countries.json)
trans-package-ready = Arhivă gata:
trans-package-error = Nu s-a putut construi arhiva:

# ISO 639-3 requirement
trans-lang-not-iso = Traducerea este posibilă doar pentru o limbă cu un cod ISO 639-3.

# FOMOD installer preview
menu-preview = Previzualizează programul de instalare…
preview-title = Previzualizare program de instalare FOMOD
preview-refresh = Reîmprospătează
preview-assumptions = Presupuneri privind fișierele
preview-details = Detalii
preview-back = Înapoi
preview-next = Următorul
preview-install = Instalează
preview-close = Închide
preview-restart = Repornește
preview-summary-title = Fișiere care vor fi instalate
preview-empty = Niciun fișier nu ar fi instalat.
preview-none-option = (niciunul)
preview-invalid = Completați alegerile obligatorii pentru a continua.
preview-no-steps = Niciun pas nu este vizibil; consultați rezumatul instalării.
preview-select-hint = Selectați o opțiune pentru a-i vedea descrierea.
preview-col-source = Sursă
preview-col-dest = Destinație
preview-col-priority = Prioritate
preview-sel-exactlyone = Alegeți exact o opțiune.
preview-sel-atmostone = Alegeți cel mult o opțiune.
preview-sel-any = Alegeți oricâte opțiuni.
preview-sel-all = Toate opțiunile sunt instalate.
preview-sel-atleastone = Alegeți cel puțin o opțiune.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validează FOMOD
validate-report-title = Validare FOMOD
validate-ok = Nicio problemă găsită. FOMOD respectă schema.
xml-editor-schema-ok = Respectă schema ModConfig 5.0.
xml-editor-schema-issues = Probleme de schemă:
schema-line-col = Linia { $line }, col. { $col }: { $msg }
schema-wrong-root = Rădăcină neașteptată „{ $found }” (se aștepta „{ $expected }”).
schema-unknown = Element neașteptat „{ $element }” în „{ $parent }”.
schema-missing = „{ $parent }” trebuie să conțină „{ $child }”.
schema-needs-one = „{ $parent }” trebuie să conțină cel puțin un „{ $child }”.
schema-too-many = „{ $child }” poate apărea o singură dată în „{ $parent }”.
schema-missing-attr = Atributul „{ $attr }” este obligatoriu pentru „{ $element }”.
schema-bad-enum = Valoare invalidă „{ $value }” pentru { $element }/@{ $attr } (se aștepta: { $allowed }).
schema-choose-one = „{ $parent }” trebuie să conțină exact unul dintre: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Mută înainte
reorder-after = Mută după

# Country / language database explorer (Properties)
menu-properties = Proprietăți…
prop-title = Baza de date țări / limbi
prop-tab-countries = Țări
prop-tab-languages = Limbi
prop-filter = Filtru:
prop-official-langs = Limbi oficiale
prop-spoken-langs = Limbi vorbite
prop-endonym = Endonimul țării
prop-font = Font
prop-spoken-in = Vorbită în
prop-select-country = Selectați o țară pentru a-i vedea detaliile.
prop-select-lang = Selectați o limbă pentru a-i vedea detaliile.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Deschide pagina Nexus Mods a jocului
