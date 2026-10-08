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
menu-export = Exportă arhiva de distribuție…
# Menu - Options
menu-options = Opțiuni
menu-settings = Setări…
menu-pre-save-script = Script înainte de salvare…
menu-post-save-script = Script după salvare…
menu-translation = Traduce interfața…
# Menu - Help
menu-help = Ajutor
menu-check-updates = Caută actualizări…
menu-about = Despre

# Update check
update-checking = Se caută actualizări…
update-up-to-date = XIMOD Architect este actualizat.
update-check-failed = Nu s-au putut verifica actualizările. Încercați din nou mai târziu.
update-available-status = Versiunea { $version } este disponibilă.
update-banner-text = XIMOD Architect { $version } este disponibil.
update-download = Descarcă:
update-skip = Omite această versiune
update-later = Mai târziu

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
label-plugin-name = Nume opțiune:
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
btn-add-plugin = Adaugă opțiune
btn-remove-plugin = Elimină opțiunea
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Destinație pentru tot grupul
label-page-dest = Destinație de instalare (toată pagina)
btn-apply-group-dest = Aplică la toate opțiunile din acest grup
btn-apply-page-dest = Aplică la toate opțiunile de pe această pagină
group-dest-hint = Setează o singură destinație de instalare pentru fiecare fișier al fiecărei opțiuni din acest grup.
page-dest-hint = Setează o singură destinație de instalare pentru fiecare fișier al fiecărei opțiuni de pe această pagină (toate grupurile).
bulk-dest-nofiles = Încă nu există fișiere de actualizat — adăugați mai întâi fișiere la opțiuni.
status-dest-applied = Destinație aplicată la { $num } fișier(e).
preview-hidden-steps = { $num } pas(i) ascunși de selecțiile curente.
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
settings-check-updates = Caută actualizări la pornire
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
label-plugin-dependencies = Dependențe ale opțiunii
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
validation-no-plugins = Pasul { $step }, grupul „{ $name }” nu are opțiuni

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
default-plugin-name = Opțiune { $num }
pattern-label = Model { $num }

# Selection prompts
msg-select-group-first = Selectați mai întâi un grup.
msg-select-plugin-edit = Selectați o opțiune pentru editare.
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

# Referenced-file verification (V2)
verify-no-root = Verificarea fișierelor omisă: niciun folder rădăcină setat
loc-header = imaginea de antet
loc-required = fișiere obligatorii
loc-conditional = set condiționat { $num }
loc-plugin = pasul { $step }, grupul { $group }, opțiunea „{ $plugin }”
verify-missing-file = Fișier lipsă: { $path } ({ $loc })
verify-missing-folder = Folder lipsă: { $path } ({ $loc })
verify-missing-image = Imagine lipsă: { $path } ({ $loc })
verify-absolute = Cale absolută (neportabilă): { $path } ({ $loc })
verify-outside = Calea iese din folderul rădăcină: { $path } ({ $loc })
verify-orphan = Fișier orfan (nereferențiat de nicio opțiune): { $path }
conflict-certain = Conflict de destinație: „{ $path }“ este scris de { $count } opțiuni ({ $locs }) — se suprascriu reciproc.
conflict-potential = Posibil conflict de destinație: „{ $path }“ este ținta a { $count } referințe ({ $locs }) — suprascrierea depinde de selecție/condiții.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Închide FOMOD
menu-close-all-fomods = Închide toate FOMOD-urile
tab-untitled = (fără titlu)
msg-drop-not-fomod = Elementul plasat nu este un FOMOD (nu s-a găsit folderul „fomod”)
exit-title = Modificări nesalvate
exit-unsaved = Un FOMOD nu a fost salvat. Doriți să îl salvați?
tab-close-hint = Închide acest FOMOD
menu-new-from-folder = Nou din folder…
menu-templates = Șabloane…
templates-title = Șabloane reutilizabile
templates-empty = Niciun șablon salvat încă. Salvați pasul selectat de mai sus pentru a crea unul.
templates-insert = Inserare
templates-save-step = Salvează pasul selectat
templates-name-hint = Numele șablonului (opțional)
msg-wizard-success = Schelet creat din folder: { $num } opțiune(i).
msg-wizard-error = Eroare: { $error }
msg-template-saved = Șablon salvat: { $name }
msg-template-inserted = Șablon inserat în proiect.
msg-template-no-step = Selectați mai întâi un pas pentru a-l salva ca șablon.
msg-template-no-dir = Nu s-a putut localiza folderul de șabloane.
msg-drop-assigned = { $added } sursă/surse adăugată(e) la opțiune ({ $rejected } în afara rădăcinii ignorată(e)).
menu-compare = Comparați cu…
compare-title = Comparație FOMOD
compare-none = Nicio diferență.
btn-optimize-image = Optimizează imaginea
msg-image-optimized = Imaginea de antet optimizată.
msg-image-ok = Imaginea de antet este deja în limite.
msg-no-header-image = Nicio imagine de antet de optimizat.
verify-image-large = Imagine prea mare ({ $width }×{ $height }): { $path }
verify-image-format = Format de imagine neacceptat (.{ $ext }): { $path }
verify-image-unreadable = Imagine ilizibilă: { $path }
menu-condition-editor = Editor de condiții…
condeditor-title = Editor de condiții
condeditor-set-by = Setat de:
condeditor-used-by = Folosit de:
condeditor-filedeps = Dependențe de fișiere
condeditor-empty = Niciun indicator sau dependență în acest proiect.
condeditor-orphan-set = setat, dar niciodată folosit
condeditor-orphan-used = folosit, dar niciodată setat
msg-img-optimized = Imagine optimizată.
msg-img-ok = Imaginea este deja în limite.
msg-img-none = Nicio imagine de optimizat.
msg-crash-recovery = Sesiunea anterioară s-a încheiat în mod neașteptat. O copie de rezervă a proiectului dvs. a fost salvată în { $path }
export-progress-title = Se creează arhiva de distribuție…
export-progress-files = { $done } / { $total } fișiere
msg-export-cancelled = Export anulat; arhiva parțială a fost eliminată.
verify-running = Se verifică fișierele de pe disc…
verify-stale = Notă: proiectul s-a schimbat în timpul verificării fișierelor; rulați din nou validarea.
prop-col-name = Nume
menu-save-as = Salvează ca…
menu-project = Proiect
menu-tools = Instrumente
menu-manual = Manual de utilizare
msg-manual-missing = Manualul de utilizare (PDF) nu a fost găsit lângă aplicație.
toolbar-new = Nou
toolbar-open = Deschide
toolbar-save = Salvează
toolbar-validate = Validează
toolbar-preview = Previzualizare
toolbar-export = Exportă
dialog-choose-root = Alegeți folderul rădăcină al modului
exit-unsaved-docs = Nesalvate: { $names }
status-summary = { $steps } pași · { $options } opțiuni
section-groups = Grupuri
section-options = Opțiuni
section-flags = Indicatori de condiție
section-files = Fișiere de instalat
hint-group-type = Cum permite programul de instalare utilizatorului să aleagă opțiuni din acest grup.
hint-default-type = Cum este oferită opțiunea când niciun tipar de dependență nu se potrivește: obligatorie, opțională, recomandată, inutilizabilă…
hint-operator = Toate condițiile trebuie să fie adevărate (ȘI) sau oricare dintre ele (SAU).
hint-flags = Indicatorii sunt valori denumite pe care această opțiune le setează când este aleasă. Alți pași și alte opțiuni le pot verifica pentru a se afișa, ascunde sau deveni obligatorii.
hint-plugin-dependencies = Tipare care schimbă tipul opțiunii în funcție de indicatori sau de fișiere prezente în joc: de exemplu „Obligatorie” când un alt mod este instalat.
hint-files = Fișiere și foldere copiate în folderul Data al jocului când această opțiune este aleasă. Destinația este relativă la Data; în caz de conflict câștigă prioritatea mai mare.
hint-visibility = Condiții care trebuie îndeplinite pentru ca acest pas să fie afișat. Lăsați gol pentru a-l afișa mereu.
seltype-exactly-one = Exact una (obligatoriu)
seltype-at-most-one = Cel mult una
seltype-any = Oricâte
seltype-all = Toate (fără alegere)
seltype-at-least-one = Cel puțin una
plugtype-required = Obligatorie
plugtype-optional = Opțională
plugtype-recommended = Recomandată
plugtype-not-usable = Inutilizabilă
plugtype-could-be-usable = Posibil utilizabilă
plugtype-required-hint = Instalată întotdeauna; utilizatorul nu o poate debifa.
plugtype-optional-hint = Oferită nebifată; utilizatorul decide.
plugtype-recommended-hint = Oferită bifată; utilizatorul o poate debifa.
plugtype-not-usable-hint = Afișată gri și nu poate fi selectată.
plugtype-could-be-usable-hint = Selectabilă, dar programul de instalare avertizează că s-ar putea să nu funcționeze.
op-and = Toate condițiile (ȘI)
op-or = Orice condiție (SAU)
theme-dark = Întunecată
theme-light = Luminoasă
theme-system = Conform sistemului
condeditor-setter-loc = Pasul { "{step}" } / Grupul { "{group}" } / „{ "{name}" }”
condeditor-pattern-of = Tiparul „{ "{name}" }” → { "{type}" }
condeditor-visibility-of = Vizibilitatea pasului { "{step}" }
condeditor-cond-set = Set condiționat { "{num}" }
condeditor-needs = { "{ctx}" } (necesită = { "{value}" })
condeditor-file-dep = { "{ctx}" }: fișier „{ "{name}" }” ({ "{state}" })
menu-translate-fomod = Traduce un FOMOD…
ftr-title = Traduce un FOMOD
ftr-open-folder = Deschide un folder de mod…
ftr-from-active = Din proiectul activ
ftr-from-active-hint = Traduce FOMOD-ul proiectului deschis în fereastra principală (trebuie mai întâi salvat).
ftr-no-fomod = Niciun FOMOD încărcat.
ftr-encoding = Codificarea fișierelor originale; fișierele traduse sunt scrise cu aceeași codificare.
ftr-source-lang = Din
ftr-target-lang = în
ftr-lang-locked = (limbile rămân fixe după încărcarea unui FOMOD)
ftr-translator = Traducător:
ftr-save = Salvează traducerea
ftr-export = Exportă fișierele traduse
ftr-export-sibling = Într-un folder fomod_<limbă>
ftr-export-sibling-hint = Scrie fișierele info.xml și ModuleConfig.xml traduse lângă folderul fomod original; fișierele originale nu sunt atinse.
ftr-export-inplace = Peste fișierele originale
ftr-export-inplace-hint = Înlocuiește fomod/info.xml și fomod/ModuleConfig.xml după ce face o copie .bak cu marcaj de timp pentru fiecare.
ftr-force-explicit-order = Păstrează ordinea originală
ftr-warn-order = Listele sortate după nume (order="Ascending") ar fi resortate după numele traduse în managerul de moduri. Această opțiune forțează order="Explicit", astfel încât opțiunile să își păstreze ordinea actuală.
ftr-update = Actualizează din folder
ftr-update-hint = Recitește FOMOD-ul de pe disc și îmbină traducerea cu el: șirurile noi, modificate și eliminate sunt semnalate.
ftr-preview-translated = Previzualizare tradusă
ftr-progress = { $done } / { $total } traduse
ftr-filter-all = Toate
ftr-filter-untranslated = Netraduse
ftr-filter-review = De revizuit
ftr-filter-issues = Cu probleme
ftr-filter-locked = Blocate
ftr-type-all = Toate câmpurile
ftr-type-names = Nume
ftr-type-descriptions = Descrieri
ftr-type-meta = Informații despre mod
ftr-search-hint = Caută în sursă, traducere sau context…
ftr-next-untranslated = Următorul netradus
ftr-show-whitespace = Afișează spațiile și sfârșiturile de linie
ftr-discard-question = Traducerea curentă are modificări nesalvate. Le eliminați și încărcați celălalt FOMOD?
ftr-discard-yes = Elimină
ftr-unsaved-close = Traducerea are modificări nesalvate.
ftr-col-num = Nr.
ftr-col-status = { "" }
ftr-col-context = Context
ftr-col-source = Sursă
ftr-col-target = Traducere
ftr-col-issues = { "" }
ftr-empty-hint = Deschideți un folder de mod sau încărcați proiectul activ pentru a lista șirurile sale traductibile.
ftr-empty-filter = Niciun șir nu corespunde filtrului curent.
ftr-select-row = Selectați un rând pentru a-i edita traducerea.
ftr-copy-source = Copiază sursa
ftr-clear-target = Golește
ftr-lock = Nu traduce
ftr-lock-hint = Șirurile blocate sunt scrise neschimbate (autor, site web, nume proprii…).
ftr-note = Notă:
ftr-status-untranslated = Netradus
ftr-status-translated = Tradus
ftr-status-auto = Precompletat automat — de revizuit
ftr-status-fuzzy = Textul sursă s-a schimbat de la traducere — de revizuit
ftr-status-obsolete = Nu mai există în FOMOD
ftr-status-locked = Blocat (scris neschimbat)
ftr-field-info-name = Numele modului (info.xml)
ftr-field-module-name = Titlul instalatorului (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Site web
ftr-field-description = Descrierea modului
ftr-field-step = Nume pas
ftr-field-group = Nume grup
ftr-field-plugin = Nume opțiune
ftr-field-plugin-desc = Descriere opțiune
ftr-issue-empty = Traducere goală
ftr-issue-whitespace = Traducerea conține doar spații
ftr-issue-edge-whitespace = Spațiile de la început sau de la sfârșit diferă de sursă
ftr-issue-token = Tokenurile protejate diferă — lipsă: { $missing } ; în plus: { $extra }
ftr-issue-newline-name = Un nume nu poate conține un sfârșit de linie
ftr-issue-control = Conține caractere pe care XML nu le poate stoca
ftr-issue-length = Lungime neobișnuită față de sursă (×{ $ratio })
ftr-issue-identical = Identic cu sursa
ftr-issue-duplicate = Același text sursă este tradus diferit în { $key }
ftr-issue-cdata = Secvența ]]> nu este permisă aici
ftr-load-error = FOMOD-ul nu a putut fi încărcat: { $error }
ftr-extracted = Șiruri traductibile găsite: { $num }.
ftr-sidecar-found = Traducere existentă încărcată și îmbinată: { $new } noi, { $changed } modificate, { $removed } eliminate.
ftr-saved = Traducere salvată în { $path }
ftr-save-error = Traducerea nu a putut fi salvată: { $error }
ftr-save-first = Salvați mai întâi proiectul, apoi traduceți-l.
ftr-export-success = Șiruri scrise în { $path }: { $count }
ftr-export-error = Exportul a eșuat: { $error }
ftr-export-blocked = Probleme blocante de corectat înainte de export: { $num }.
ftr-export-stale = Șiruri omise deoarece FOMOD-ul s-a schimbat: { $num }; folosiți „Actualizează din folder”.
ftr-update-report = Actualizat: { $new } noi, { $changed } modificate, { $moved } mutate, { $removed } eliminate, { $unchanged } neschimbate.
menu-edit = Editare
menu-undo = Anulează
menu-redo = Refă
tree-title = Proiect
tree-mod-info = Informații despre mod
tree-steps = Pași de instalare
tree-required = Fișiere obligatorii
tree-conditional = Instalări condiționate
tree-empty-steps = Încă nu există niciun pas — faceți clic pe + pentru a adăuga unul.
tree-duplicate = Duplică
tree-delete = Șterge
tree-save-template = Salvează ca șablon…
tree-drop-hint = Plasați aici pentru a muta
cond-set-label = Set condiționat { $num }
inspector-empty = Selectați un element din arborele proiectului sau adăugați un pas pentru a începe.
count-options = Opțiuni: { $num }
count-files = Fișiere: { $num }
msg-deleted-undo = Șters. Folosiți „Anulează” (Ctrl+Z) pentru a-l restaura.
problems-title = Probleme
problems-errors = Erori: { $num }
problems-warnings = Avertismente: { $num }
btn-close = Închide
ftr-export-package = Ca pachet de traducere (arhivă)
ftr-export-package-hint = Creează un fișier .zip sau .7z gata de încărcat: fișierele info.xml și ModuleConfig.xml traduse plus un README (doar corecție) sau întregul mod cu fișierele traduse (complet).
ftr-package-full = Mod complet
ftr-package-full-hint = Include în arhivă toate fișierele modului, nu doar cele două fișiere XML traduse. Asigurați-vă că autorul permite redistribuirea.
ftr-package-name-template = Nume:
ftr-readme-patch = Această arhivă conține traducerea (limba: { $langname }) a instalatorului modului „{ $name }” (fomod/info.xml și fomod/ModuleConfig.xml). Instalați-o peste modul original sau lăsați managerul de moduri să o îmbine, astfel încât fișierele traduse să le înlocuiască pe cele originale. Se schimbă doar textele instalatorului; fișierele propriu-zise ale modului nu sunt incluse. Realizat cu XIMOD Architect.
ftr-readme-full = Această arhivă conține modul „{ $name }” cu instalatorul tradus (limba: { $langname }; fomod/info.xml și fomod/ModuleConfig.xml). Instalați-o la fel ca modul original. Au fost modificate doar textele instalatorului. Realizat cu XIMOD Architect.
ftr-apply-memory = Completează din memorie
ftr-memory-size = Memorie de traducere — intrări pentru această pereche de limbi: { $num }. Fiecare traducere salvată este adăugată în ea.
ftr-memory-applied = Șiruri completate din memoria de traducere (marcate „de revizuit”): { $num }.
ftr-memory-suggestion = Memoria sugerează:
ftr-use-suggestion = Folosește
ftr-propagate = Propagă la cele identice
ftr-propagate-hint = Copiază această traducere în toate celelalte șiruri cu același text sursă care sunt încă netraduse.
ftr-propagated = Șiruri identice completate: { $num }.
ftr-csv-export = Exportă CSV…
ftr-csv-import = Importă CSV…
ftr-csv-imported = Șiruri actualizate din fișierul CSV: { $num }.
ftr-csv-error = Eroare CSV: { $error }
ftr-glossary = Glosar
ftr-glossary-source = Termen
ftr-glossary-target = Traducere
ftr-glossary-case = Majuscule/minuscule
ftr-glossary-dnt = Păstrează
ftr-glossary-add = Adaugă termen
ftr-issue-glossary = Glosar: „{ $term }” nu este tradus conform așteptărilor

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Deschide arhiva…
filter-archive = Arhive de mod (zip, 7z)
msg-archive-opened = Arhivă deschisă (fișiere extrase: { $num }): { $path }
msg-archive-reused = Arhiva este deja extrasă, se reutilizează { $path }
msg-archive-unsupported = Formatul de arhivă „.{ $ext }” nu este acceptat; extrageți-o mai întâi cu 7-Zip (pot fi deschise doar .zip și .7z).
msg-archive-error = Eroare la deschiderea arhivei: { $error }
msg-archive-no-fomod = Nu s-a găsit niciun dosar „fomod” în arhivă ({ $path })
msg-archive-extracting = Se extrage arhiva…
ftr-open-archive = Deschide o arhivă de mod…
ftr-package-full-partial = Modul a fost deschis dintr-o arhivă care conține doar dosarul său fomod; pachetele complete necesită modul extras.
info-module-deps = Cerințele modului
info-module-deps-hint = Fișiere sau indicatori de care are nevoie întregul mod înainte de rularea instalatorului (moduleDependencies). Lăsați gol dacă nu există.
info-header-advanced = Antet avansat
info-title-position = Poziția titlului
info-title-colour = Culoarea titlului
info-title-colour-hint = Așteptat: șase cifre hexazecimale (RRGGBB)
info-image-show = Afișează imaginea antetului
info-image-fade = Estompează imaginea antetului
info-image-height = Înălțimea imaginii antetului
info-attr-default = (implicit)
file-always-install = Întotdeauna
file-always-install-hint = Instalează întotdeauna acest fișier, chiar dacă opțiunea nu este selectată (alwaysInstall).
file-install-if-usable = Dacă e util
file-install-if-usable-hint = Instalează acest fișier ori de câte ori opțiunea este utilizabilă, chiar dacă nu este selectată (installIfUsable).
msg-import-lossy = Acest FOMOD conține construcții pe care XIMOD nu le poate edita (număr: { $num }); ele vor fi eliminate la salvarea proiectului.
fidelity-nested-deps = Grup de dependențe imbricat în { $context } (este acceptat un singur nivel)
fidelity-game-dep = Cerință de versiune a jocului { $version } în { $context }
fidelity-fomm-dep = Cerință de versiune a managerului de moduri { $version } în { $context }
fidelity-unknown = Elementul „{ $element }” din „{ $parent }” nu este acceptat ({ $context })
loc-module = cerințele modului
loc-step = pasul { $step } „{ $name }”
loc-installer = instalator

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Restaurează o copie de rezervă…
backups-title = Restaurare copie de rezervă
backups-empty = Acest proiect nu are încă nicio copie de rezervă. Una este creată de fiecare dată când proiectul este salvat peste o versiune anterioară.
backups-changes = Modificări față de proiectul curent: { $num }
btn-compare = Compară
btn-restore = Restaurează
btn-delete-backups = Șterge toate copiile de rezervă
btn-delete-backups-confirm = Dați clic din nou pentru a șterge toate copiile de rezervă
msg-backup-restored = Copia de rezervă din { $time } a fost restaurată în editor (încă nesalvată; acțiunea Anulează o revocă)
msg-backups-deleted = Copii de rezervă șterse: { $num }
settings-backup-count = Copii de rezervă de păstrat:
settings-backup-count-hint = Numărul de versiuni anterioare ale XML-ului FOMOD păstrate în fomod/backups la salvare (0 = fără copii de rezervă).
settings-autosave-minutes = Salvează automat o copie de recuperare la fiecare (minute):
settings-autosave-minutes-hint = La acest interval, o copie de recuperare a fiecărui proiect modificat este scrisă în dosarul de configurare; este propusă la următoarea pornire doar după o închidere anormală (0 = dezactivat).
settings-auto-masters = Adaugă masterele unui plugin drept condiții
settings-auto-masters-hint = Când un plugin (.esp/.esm/.esl) este adăugat unei opțiuni, masterele pe care le necesită și pe care nu le oferă nici jocul, nici acest mod devin condiții de fișier „Active” ale opțiunii.
msg-author-from-plugin = Autor completat din antetul pluginului: { $author }
msg-masters-added = Mastere ale { $plugin } adăugate drept condiții de fișier: { $num }
issue-missing-master = { $plugin } necesită { $master }, care nu se află în acest mod și nici nu este declarat ca dependență
issue-esl-mismatch-flag = { $plugin } are extensia .esl, dar fanionul light (ESL) nu este setat
issue-esl-eligible = { $plugin } ar putea fi marcat ca light (înregistrări noi: { $num }, limită { $limit })
issue-esl-too-big = { $plugin } este marcat ca light, dar nu respectă regulile pluginurilor light (înregistrări noi: { $num }, limită { $limit }, sau un FormID în afara intervalului permis)
menu-plugin-report = Raport despre pluginuri…
plugins-title = Raport despre pluginuri
plugins-file = Fișier
plugins-kind = Tip
plugins-light = Fanion light
plugins-masters = Mastere
plugins-new-records = Înregistrări noi / limită
plugins-eligible = Eligibil light
plugins-empty = Acest proiect nu instalează niciun fișier de plugin (.esp, .esm sau .esl).
plugins-unreadable = ilizibil

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Arborele final de fișiere
preview-total-size = Dimensiunea totală a instalării: { $size }
preview-tree-truncated = Arborele este trunchiat: prea multe fișiere de extins (dimensiunile de mai sus sunt parțiale).
preview-overwritten-by = Suprascris de { $plugin }
preview-scenario = Scenariu:
preview-scenario-load = Încarcă
preview-scenario-save = Salvează…
preview-scenario-delete = Șterge
preview-scenario-name = Numele scenariului
preview-scenario-saved = Scenariul „{ $name }” a fost salvat în fomod/scenarios
preview-scenario-unresolved = Selecții ale scenariului care nu corespund niciunei opțiuni din acest proiect (redenumită sau eliminată): { $num }
preview-scenario-none = (niciun scenariu)
issue-unreachable-step = Pasul „{ $step }” nu poate fi afișat niciodată: condițiile sale de vizibilitate testează o valoare de marcaj pe care nicio opțiune anterioară nu o setează
issue-unreachable-option = Opțiunea „{ $plugin }” nu poate fi selectată niciodată: tiparele sale de tip utilizabil testează o valoare de marcaj pe care nicio opțiune nu o setează
issue-unreachable-cond = Setul condiționat de fișiere { $num } nu se poate aplica niciodată: condițiile sale testează o valoare de marcaj pe care nicio opțiune nu o setează
size-option = Dimensiunea instalării: { $size } (fișiere: { $num })
size-missing = Surse lipsă: { $num }
size-unknown = Dimensiunea instalării: — (rulați Validează pentru a o măsura)
menu-nexus-desc = Descriere pentru Nexus…
nexus-title = Descriere pentru Nexus Mods
nexus-format = Format:
nexus-include-requirements = Cerințe
nexus-include-options = Opțiuni de instalare
nexus-include-install = Instalare
nexus-include-changelog = Jurnal de modificări
nexus-previous = Versiunea anterioară…
nexus-previous-none = (nicio versiune anterioară: fără jurnal de modificări)
nexus-language = Limbă:
nexus-language-source = (sursă)
nexus-sec-requirements = Cerințe
nexus-sec-options = Opțiuni de instalare
nexus-sec-install = Instalare
nexus-sec-changelog = Jurnal de modificări
nexus-install-text = Acest mod include un program de instalare FOMOD: instalați-l cu un manager de moduri (Vortex, Mod Organizer 2) și alegeți opțiunile în programul de instalare.
nexus-requires = Necesită
nexus-step = Pas
nexus-added = Adăugat
nexus-removed = Eliminat
nexus-changed = Modificat
btn-copy = Copiază
btn-save-as = Salvează ca…
msg-copied = Copiat în clipboard
msg-saved-to = Salvat în { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Redenumește…
condeditor-rename-exists = Există deja un marcaj cu numele „{ $name }”
condeditor-renamed = Marcajul „{ $from }” a fost redenumit în „{ $to }” (apariții: { $num })
condeditor-delete-uses = Șterge toate utilizările
condeditor-deleted-uses = Marcajul „{ $name }” a fost eliminat de peste tot (apariții: { $num })
condeditor-values-set = Valori setate:
condeditor-values-tested = Valori testate:
condeditor-value-never-set = { $value } — testată, dar niciodată setată
condeditor-value-never-tested = { $value } — setată, dar niciodată testată
condeditor-builder = Constructor de condiții
condeditor-builder-none = Selectați un pas, o opțiune, un set condițional de fișiere sau informațiile modului în fereastra principală pentru a-i edita condițiile aici.
condeditor-builder-pattern = Model:
condeditor-sentence-if = DACĂ
condeditor-sentence-and = ȘI
condeditor-sentence-or = SAU
condeditor-sentence-flag = marcajul { "{name}" } = { "{value}" }
condeditor-sentence-file = fișierul { "{name}" } este { "{value}" }
condeditor-sentence-empty = (nicio condiție: întotdeauna adevărat)
condeditor-sentence-then-visible = ATUNCI pasul este afișat
condeditor-sentence-then-type = ATUNCI opțiunea devine { $type }
condeditor-sentence-then-install = ATUNCI fișierele sunt instalate
condeditor-sentence-then-module = ATUNCI programul de instalare poate rula (verificat înainte de pornire)
issue-flag-value-never-set = Marcajul „{ $flag }” este testat cu valoarea „{ $value }”, pe care nicio opțiune nu o setează
issue-flag-never-used = Marcajul „{ $flag }” este setat, dar nu este testat nicăieri
menu-project-strings = Textele proiectului…
strings-title = Textele proiectului
strings-search = Caută text, locație sau cheie…
strings-kind-all = Toate
strings-kind-names = Nume
strings-kind-descriptions = Descrieri
strings-duplicates-only = Doar duplicate
strings-replace-with = Înlocuiește cu:
strings-case = Potrivire majuscule/minuscule
strings-whole-word = Cuvânt întreg
strings-replace-current = Înlocuiește
strings-replace-all = Înlocuiește tot
strings-replaced = Texte înlocuite: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Același text ca:
strings-count = Texte: { $num } · grupuri de duplicate: { $dups }
strings-col-location = Locație
strings-col-field = Câmp
strings-col-text = Text

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Conținutul unei arhive…
filter-bethesda-archive = Arhive Bethesda (bsa, ba2)
archive-view-title = Conținutul arhivei
archive-view-format = Format:
archive-view-entries = Intrări: { $num }
archive-view-size = { $size } dezarhivat
archive-view-search = Caută o cale…
archive-view-col-path = Cale
archive-view-col-size = Dimensiune
archive-view-col-compressed = Comprimat
archive-view-truncated = Sunt afișate doar primele { $num } intrări care se potrivesc — rafinați căutarea.
archive-view-error = Această arhivă nu poate fi citită: { $error }
archive-view-hint = Vezi conținutul acestei arhive
issue-conflict-archive = Aceeași resursă în mai multe arhive: „{ $path }“ este împachetată de { $count } referințe ({ $locs }) — ordinea de încărcare a arhivelor din joc decide care este folosită.
issue-conflict-archive-loose = Arhivă versus fișier liber: „{ $path }“ este atât împachetat într-o arhivă, cât și instalat ca fișier liber ({ $locs }) — fișierul liber are prioritate față de cel arhivat.
preview-in-archive = (în arhivă)
preview-archived-size = din care { $size } împachetate în arhive

# --- Project tree: expand / collapse menus
tree-expand = Extindere
tree-collapse = Restrângere
tree-expand-all = Extinde tot
tree-expand-selected = Extinde selecția
tree-expand-from = Extinde de la selecție
tree-collapse-all = Restrânge tot
tree-collapse-selected = Restrânge selecția
tree-collapse-from = Restrânge de la selecție
tree-expand-all-hint = Extinde toate titlurile
tree-expand-selected-hint = Extinde doar titlul selectat
tree-expand-from-hint = Extinde titlul selectat și tot ce se află sub el
tree-collapse-all-hint = Restrânge toate titlurile
tree-collapse-selected-hint = Restrânge doar titlul selectat
tree-collapse-from-hint = Restrânge titlul selectat și tot ce se află sub el

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Adaugă grup
btn-remove-group-cond = Elimină grupul
dep-type-game = Versiunea jocului
dep-type-fomm = Versiunea managerului de moduri
dep-group-hint = Un grup de condiții combinate cu ȘI / SAU; grupurile pot fi imbricate.
condeditor-sentence-game = versiunea jocului ≥ { "{value}" }
condeditor-sentence-fomm = versiunea managerului de moduri ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Texte unice
ftr-uniques-hint = Afișează un singur rând pentru fiecare text sursă distinct. Traducerea acelui rând traduce dintr-odată toate șirurile cu același text.
ftr-uniques-synced = Șiruri identice actualizate: { $num }.
ftr-uniques-group = Șiruri care au acest text: { $num }; traducerea lui se aplică tuturor.
