# XIMOD Architect - translation metadata
# @language = ita
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Italiano
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versione { $version }

# Status messages
status-ready = Pronto
msg-save-success = FOMOD salvato correttamente
msg-save-error = Errore durante il salvataggio del FOMOD
msg-export-success = Archivio di distribuzione creato ({ $count } file): { $path }
msg-export-error = Errore durante la creazione dell'archivio di distribuzione: { $error }
msg-load-success = FOMOD caricato correttamente
msg-load-error = Errore durante il caricamento del FOMOD
msg-merge-success = FOMOD unito correttamente
msg-merge-error = Errore durante l'unione del FOMOD
msg-no-root-selected = Seleziona prima una cartella radice
msg-no-fomod-folder = Nessuna cartella «fomod» trovata. Crearla?
msg-file-outside-root = Il file è fuori dalla cartella radice

# Menu - File
menu-file = File
menu-new = Nuovo
menu-open = Apri cartella…
menu-open-file = Apri file…
menu-save = Salva
menu-recent = Recenti
menu-exit = Esci
menu-merge = Unisci FOMOD…
menu-export = Esporta archivio di distribuzione…
# Menu - Options
menu-options = Opzioni
menu-settings = Impostazioni…
menu-pre-save-script = Script pre-salvataggio…
menu-post-save-script = Script post-salvataggio…
menu-translation = Traduci l'interfaccia…
# Menu - Help
menu-help = Aiuto
menu-check-updates = Controlla aggiornamenti…
menu-about = Informazioni

# Update check
update-checking = Ricerca aggiornamenti…
update-up-to-date = XIMOD Architect è aggiornato.
update-check-failed = Impossibile controllare gli aggiornamenti. Riprova più tardi.
update-available-status = È disponibile la versione { $version }.
update-banner-text = È disponibile XIMOD Architect { $version }.
update-download = Scarica:
update-skip = Ignora questa versione
update-later = Più tardi

# Tabs
tab-info = Info mod
tab-steps = Fasi di installazione
tab-required = Installazioni obbligatorie
tab-conditional = Installazioni condizionali

# Info Tab
label-workspace = Area di lavoro
label-root-dir = Cartella radice:
label-mod-name = Nome mod:
label-author = Autore:
label-version = Versione:
label-game-name = Nome gioco:
label-category = Categoria:
label-url = URL sito web:
label-header-image = Immagine intestazione:
label-description = Descrizione:
placeholder-select-dir = (Seleziona una cartella)
placeholder-select-game = (Seleziona un gioco)

# Steps Tab
label-step-name = Nome fase:
label-group-name = Nome gruppo:
label-group-type = Tipo di gruppo:
label-plugin-name = Nome dell'opzione:
label-plugin-desc = Descrizione:
label-plugin-type = Tipo predefinito:
label-plugin-image = Immagine:
label-visibility = Condizioni di visibilità
label-operator = Operatore:

# Buttons
btn-browse = Sfoglia…
btn-clear = Cancella
btn-add = Aggiungi
btn-remove = Rimuovi
btn-add-step = Nuova fase
btn-delete-step = Elimina fase
btn-add-group = Aggiungi gruppo
btn-remove-group = Rimuovi gruppo
btn-add-plugin = Aggiungi opzione
btn-remove-plugin = Rimuovi opzione
btn-add-file = Aggiungi file
btn-add-folder = Aggiungi cartella
btn-remove-file = Rimuovi
btn-add-flag = Aggiungi flag
btn-remove-flag = Rimuovi flag
btn-add-condition = Aggiungi condizione
btn-remove-condition = Rimuovi condizione
btn-add-dependency = Aggiungi dipendenza
btn-remove-dependency = Rimuovi dipendenza
btn-add-pattern = Nuovo schema
btn-remove-pattern = Elimina schema
btn-save = Salva
btn-cancel = Annulla
btn-ok = OK
btn-yes = Sì
btn-no = No

# Condition/Dependency Labels
label-flag-name = Nome flag:
label-flag-value = Valore:
label-condition-type = Tipo:
label-condition-name = Nome:
label-condition-value = Valore:
label-dep-type = Tipo di dipendenza:
label-dep-name = Nome/file:
label-dep-value = Valore/stato:

# Files
label-source = Origine
label-destination = Destinazione
label-priority = Priorità
label-file-type = Tipo

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Destinazione per l'intero gruppo
label-page-dest = Destinazione di installazione (intera pagina)
btn-apply-group-dest = Applica a tutte le opzioni di questo gruppo
btn-apply-page-dest = Applica a tutte le opzioni di questa pagina
group-dest-hint = Imposta un'unica destinazione di installazione per ogni file di ogni opzione di questo gruppo.
page-dest-hint = Imposta un'unica destinazione di installazione per ogni file di ogni opzione di questa pagina (tutti i gruppi).
bulk-dest-nofiles = Nessun file da aggiornare — aggiungi prima dei file alle opzioni.
status-dest-applied = Destinazione applicata a { $num } file.
preview-hidden-steps = { $num } passaggi nascosti dalle selezioni attuali.
label-files = File
label-dependencies = Dipendenze

# Settings Dialog
settings-title = Impostazioni
settings-tab-general = Generali
settings-tab-recent-files = File recenti
settings-language = Lingua:
settings-theme = Tema:
settings-font-size = Dimensione carattere:
settings-replace-newlines = Elabora gli a capo nelle descrizioni
settings-check-updates = Controlla aggiornamenti all'avvio
settings-max-recent = Max file recenti:
settings-window-width = Larghezza finestra:
settings-window-height = Altezza finestra:
settings-no-recent-files = Nessun file recente.

# Status messages for settings
status-settings-saved = Impostazioni salvate correttamente

# About Dialog
about-title = Informazioni su XIMOD Architect
about-description = Uno strumento multipiattaforma per creare installer FOMOD per le mod dei giochi Bethesda.
about-license = Concesso in licenza con licenza MIT
about-copyright = © 2024 XIMOD Team
about-credit = Porting in Rust dello strumento originale di Wenderer:

# Script Dialog
script-title = Modifica script
script-info = Gli script vengono eseguiti prima o dopo il salvataggio. Puoi usare le seguenti macro:
script-macros = Macro disponibili:
macro-modname = $MODNAME$ - Nome mod
macro-modauthor = $MODAUTHOR$ - Nome autore
macro-modversion = $MODVERSION$ - Versione mod
macro-modroot = $MODROOT$ - Percorso cartella radice
macro-date = $DATE$ - Data corrente (AAAA-MM-GG)
macro-time = $TIME$ - Ora corrente (HH:MM:SS)
macro-random = $RANDOM$ - Numero casuale

# Plugin Dependencies
label-plugin-dependencies = Dipendenze dell'opzione
label-default-type = Tipo predefinito:
label-pattern-type = Tipo di schema:
label-pattern-operator = Operatore schema:

# Conditional Files
label-pattern = Schema

# Validation Messages
validation-no-name = Il nome della mod è obbligatorio
validation-no-steps = È necessaria almeno una fase o un file obbligatorio
validation-empty-step = La fase { $num } non ha un nome
validation-empty-group = La fase { $step }, gruppo { $group } non ha un nome
validation-no-plugins = La fase { $step }, gruppo «{ $name }» non ha opzioni

# File States
state-active = Attivo
state-inactive = Inattivo
state-missing = Mancante

# Confirmation
confirm-title = Conferma
confirm-delete = Vuoi davvero eliminare questo elemento?
confirm-discard = Hai modifiche non salvate. Scartarle e continuare?
confirm-unsaved = Hai modifiche non salvate. Vuoi salvare prima di chiudere?
confirm-save-issues = Il progetto presenta i seguenti problemi:
confirm-save-anyway = Salvare comunque?

# Errors
error-invalid-xml = File XML non valido
error-parse-failed = Analisi del FOMOD non riuscita
error-write-failed = Scrittura del file non riuscita
error-create-dir = Creazione della cartella non riuscita

# Default names (generated when creating new items)
default-step-name = Fase { $num }
default-group-name = Gruppo { $num }
default-plugin-name = Opzione { $num }
pattern-label = Schema { $num }

# Selection prompts
msg-select-group-first = Seleziona prima un gruppo.
msg-select-plugin-edit = Seleziona un'opzione da modificare.
label-empty = (vuoto)
image-no-image = Nessuna immagine

# File dialog filters
filter-images = Immagini
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = File

# Status bar
status-modified = Modificato

# Status messages (errors)
msg-settings-save-error = Errore durante il salvataggio delle impostazioni
msg-script-save-error = Errore durante il salvataggio dello script

# Translation editor
trans-title = Editor di traduzione
trans-source-lang = Lingua visualizzata:
trans-target-lang = Lingua da tradurre:
trans-col-key = Chiave
trans-col-source = Etichetta
trans-col-target = Traduzione
trans-saved = Traduzione salvata
trans-save-error = Errore durante il salvataggio della traduzione

# XML editor
xml-editor-title = Editor XML
xml-editor-edit = Modifica
xml-editor-apply = Applica
xml-editor-revert = Annulla
xml-editor-readonly = Sola lettura
xml-editor-editing = Modifica in corso — le schede grafiche sono bloccate
xml-editor-error = Errore:
xml-editor-applied = Modifiche XML applicate
xml-editor-wellformed = XML ben formato
xml-editor-error-at = Riga { $line }, colonna { $col }: { $msg }

# Country / flag picker
settings-country-name = Nome paese:
settings-pick-country = Fai clic per scegliere il tuo paese
flags-title = Scegli un paese
flags-filter = Filtro:
flags-none = Nessuna bandiera trovata

# Translation editor: country & font
trans-endonym = Endonimo del paese:
trans-font = Carattere:
trans-no-font = (nessuno)
trans-browse = Sfoglia…
trans-google-fonts = Google Fonts
trans-pick-country = Fai clic per scegliere il paese
trans-font-outside = Il carattere deve prima essere installato in assets/fonts.
trans-font-dir-missing = La cartella assets/fonts non è stata trovata.

# Translation submission
trans-lang-endonym = Endonimo della lingua:
trans-author = Autore:
trans-submit = Invia…
trans-submit-hint = Crea un file zip e apre un'e-mail precompilata
trans-data-updated = Dati di riferimento aggiornati (Languages.json / Countries.json)
trans-package-ready = Archivio pronto:
trans-package-error = Impossibile creare l'archivio:

# ISO 639-3 requirement
trans-lang-not-iso = La traduzione è possibile solo per una lingua con un codice ISO 639-3.

# FOMOD installer preview
menu-preview = Anteprima installer…
preview-title = Anteprima installer FOMOD
preview-refresh = Aggiorna
preview-assumptions = Ipotesi sui file
preview-details = Dettagli
preview-back = Indietro
preview-next = Avanti
preview-install = Installa
preview-close = Chiudi
preview-restart = Ricomincia
preview-summary-title = File che verranno installati
preview-empty = Nessun file verrebbe installato.
preview-none-option = (nessuno)
preview-invalid = Completa le scelte obbligatorie per continuare.
preview-no-steps = Nessuna fase è visibile; consulta il riepilogo dell'installazione.
preview-select-hint = Seleziona un'opzione per vederne la descrizione.
preview-col-source = Origine
preview-col-dest = Destinazione
preview-col-priority = Priorità
preview-sel-exactlyone = Scegli esattamente un'opzione.
preview-sel-atmostone = Scegli al massimo un'opzione.
preview-sel-any = Scegli un numero qualsiasi di opzioni.
preview-sel-all = Tutte le opzioni vengono installate.
preview-sel-atleastone = Scegli almeno un'opzione.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Convalida FOMOD
validate-report-title = Convalida FOMOD
validate-ok = Nessun problema rilevato. Il FOMOD è conforme allo schema.
xml-editor-schema-ok = Conforme allo schema ModConfig 5.0.
xml-editor-schema-issues = Problemi con lo schema:
schema-line-col = Riga { $line }, col. { $col }: { $msg }
schema-wrong-root = Elemento radice imprevisto "{ $found }" (previsto "{ $expected }").
schema-unknown = Elemento imprevisto "{ $element }" in "{ $parent }".
schema-missing = "{ $parent }" deve contenere "{ $child }".
schema-needs-one = "{ $parent }" deve contenere almeno un "{ $child }".
schema-too-many = "{ $child }" può comparire una sola volta in "{ $parent }".
schema-missing-attr = L'attributo "{ $attr }" è obbligatorio in "{ $element }".
schema-bad-enum = Valore non valido "{ $value }" per { $element }/@{ $attr } (previsto: { $allowed }).
schema-choose-one = "{ $parent }" deve contenere esattamente uno tra: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Sposta prima
reorder-after = Sposta dopo

# Country / language database explorer (Properties)
menu-properties = Proprietà…
prop-title = Database paesi / lingue
prop-tab-countries = Paesi
prop-tab-languages = Lingue
prop-filter = Filtro:
prop-official-langs = Lingue ufficiali
prop-spoken-langs = Lingue parlate
prop-endonym = Endonimo del paese
prop-font = Carattere
prop-spoken-in = Parlata in
prop-select-country = Seleziona un paese per vederne i dettagli.
prop-select-lang = Seleziona una lingua per vederne i dettagli.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Apri la pagina Nexus Mods del gioco

# Referenced-file verification (V2)
verify-no-root = Verifica dei file saltata: nessuna cartella principale impostata
loc-header = immagine di intestazione
loc-required = file richiesti
loc-conditional = insieme condizionale { $num }
loc-plugin = passaggio { $step }, gruppo { $group }, opzione «{ $plugin }»
verify-missing-file = File mancante: { $path } ({ $loc })
verify-missing-folder = Cartella mancante: { $path } ({ $loc })
verify-missing-image = Immagine mancante: { $path } ({ $loc })
verify-absolute = Percorso assoluto (non portabile): { $path } ({ $loc })
verify-outside = Il percorso esce dalla cartella principale: { $path } ({ $loc })
verify-orphan = File orfano (non usato da nessuna opzione): { $path }
conflict-certain = Conflitto di destinazione: «{ $path }» è scritto da { $count } opzioni ({ $locs }): si sovrascrivono a vicenda.
conflict-potential = Possibile conflitto di destinazione: «{ $path }» è destinazione di { $count } riferimenti ({ $locs }): la sovrascrittura dipende dalla selezione/condizioni.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Chiudi FOMOD
menu-close-all-fomods = Chiudi tutti i FOMOD
tab-untitled = (senza titolo)
msg-drop-not-fomod = L'elemento trascinato non è un FOMOD (cartella «fomod» non trovata)
exit-title = Modifiche non salvate
exit-unsaved = Un FOMOD non è stato salvato. Vuoi salvarlo?
tab-close-hint = Chiudi questo FOMOD
menu-new-from-folder = Nuovo da cartella…
menu-templates = Modelli…
templates-title = Modelli riutilizzabili
templates-empty = Nessun modello salvato. Salva sopra il passaggio selezionato per crearne uno.
templates-insert = Inserisci
templates-save-step = Salva il passaggio selezionato
templates-name-hint = Nome del modello (facoltativo)
msg-wizard-success = Struttura creata dalla cartella: { $num } opzione/i.
msg-wizard-error = Errore: { $error }
msg-template-saved = Modello salvato: { $name }
msg-template-inserted = Modello inserito nel progetto.
msg-template-no-step = Seleziona prima un passaggio per salvarlo come modello.
msg-template-no-dir = Impossibile individuare la cartella dei modelli.
msg-drop-assigned = { $added } sorgente/i aggiunta/e all'opzione ({ $rejected } fuori dalla radice ignorata/e).
menu-compare = Confronta con…
compare-title = Confronto FOMOD
compare-none = Nessuna differenza.
btn-optimize-image = Ottimizza immagine
msg-image-optimized = Immagine di intestazione ottimizzata.
msg-image-ok = L'immagine di intestazione è già entro i limiti.
msg-no-header-image = Nessuna immagine di intestazione da ottimizzare.
verify-image-large = Immagine troppo grande ({ $width }×{ $height }): { $path }
verify-image-format = Formato immagine non supportato (.{ $ext }): { $path }
verify-image-unreadable = Immagine illeggibile: { $path }
menu-condition-editor = Editor delle condizioni…
condeditor-title = Editor delle condizioni
condeditor-set-by = Impostato da:
condeditor-used-by = Usato da:
condeditor-filedeps = Dipendenze di file
condeditor-empty = Nessun flag o dipendenza in questo progetto.
condeditor-orphan-set = impostato ma mai usato
condeditor-orphan-used = usato ma mai impostato
msg-img-optimized = Immagine ottimizzata.
msg-img-ok = Immagine già entro i limiti.
msg-img-none = Nessuna immagine da ottimizzare.
msg-crash-recovery = La sessione precedente è terminata in modo imprevisto. Una copia di backup del progetto è stata salvata in { $path }
export-progress-title = Creazione dell'archivio di distribuzione…
export-progress-files = { $done } / { $total } file
msg-export-cancelled = Esportazione annullata; l'archivio parziale è stato rimosso.
verify-running = Verifica dei file su disco…
verify-stale = Nota: il progetto è cambiato durante la verifica dei file; eseguire di nuovo la convalida.
prop-col-name = Nome
menu-save-as = Salva con nome…
menu-project = Progetto
menu-tools = Strumenti
menu-manual = Manuale utente
msg-manual-missing = Il manuale utente (PDF) non è stato trovato accanto all'applicazione.
toolbar-new = Nuovo
toolbar-open = Apri
toolbar-save = Salva
toolbar-validate = Convalida
toolbar-preview = Anteprima
toolbar-export = Esporta
dialog-choose-root = Scegli la cartella principale della mod
exit-unsaved-docs = Non salvati: { $names }
status-summary = { $steps } passaggi · { $options } opzioni
section-groups = Gruppi
section-options = Opzioni
section-flags = Flag di condizione
section-files = File da installare
hint-group-type = Come l'installer lascia scegliere all'utente le opzioni di questo gruppo.
hint-default-type = Come viene proposta l'opzione quando nessuno dei suoi schemi di dipendenza corrisponde: richiesta, facoltativa, consigliata, non utilizzabile…
hint-operator = Tutte le condizioni devono essere vere (E), oppure ne basta una (O).
hint-flags = I flag sono valori con nome impostati da questa opzione quando viene scelta. Altri passaggi e opzioni possono verificarli per mostrarsi, nascondersi o diventare obbligatori.
hint-plugin-dependencies = Schemi che cambiano il tipo dell'opzione in base a flag o a file presenti nel gioco: per esempio “Richiesta” quando è installata un'altra mod.
hint-files = File e cartelle copiati nella cartella Data del gioco quando questa opzione è scelta. La destinazione è relativa a Data; in caso di conflitto vince la priorità più alta.
hint-visibility = Condizioni da soddisfare perché questo passaggio venga mostrato. Lasciare vuoto per mostrarlo sempre.
seltype-exactly-one = Esattamente una (obbligatoria)
seltype-at-most-one = Al massimo una
seltype-any = Qualsiasi numero
seltype-all = Tutte (nessuna scelta)
seltype-at-least-one = Almeno una
plugtype-required = Richiesta
plugtype-optional = Facoltativa
plugtype-recommended = Consigliata
plugtype-not-usable = Non utilizzabile
plugtype-could-be-usable = Forse utilizzabile
plugtype-required-hint = Sempre installata; l'utente non può deselezionarla.
plugtype-optional-hint = Proposta non selezionata; decide l'utente.
plugtype-recommended-hint = Proposta già selezionata; l'utente può deselezionarla.
plugtype-not-usable-hint = Mostrata in grigio, non selezionabile.
plugtype-could-be-usable-hint = Selezionabile, ma l'installer avvisa che potrebbe non funzionare.
op-and = Tutte le condizioni (E)
op-or = Una condizione qualsiasi (O)
theme-dark = Scuro
theme-light = Chiaro
theme-system = Come il sistema
condeditor-setter-loc = Passaggio { "{step}" } / Gruppo { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Schema di «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Visibilità del passaggio { "{step}" }
condeditor-cond-set = Insieme condizionale { "{num}" }
condeditor-needs = { "{ctx}" } (richiede = { "{value}" })
condeditor-file-dep = { "{ctx}" }: file «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = Traduci un FOMOD…
ftr-title = Traduci un FOMOD
ftr-open-folder = Apri una cartella di mod…
ftr-from-active = Dal progetto attivo
ftr-from-active-hint = Traduce il FOMOD del progetto aperto nella finestra principale (deve prima essere salvato).
ftr-no-fomod = Nessun FOMOD caricato.
ftr-encoding = Codifica dei file originali; i file tradotti vengono scritti con la stessa codifica.
ftr-source-lang = Da
ftr-target-lang = a
ftr-lang-locked = (le lingue restano fisse dopo il caricamento di un FOMOD)
ftr-translator = Traduttore:
ftr-save = Salva la traduzione
ftr-export = Esporta i file tradotti
ftr-export-sibling = In una cartella fomod_<lingua>
ftr-export-sibling-hint = Scrive info.xml e ModuleConfig.xml tradotti accanto alla cartella fomod originale; i file originali non vengono toccati.
ftr-export-inplace = Sopra i file originali
ftr-export-inplace-hint = Sostituisce fomod/info.xml e fomod/ModuleConfig.xml dopo aver creato di ciascuno una copia .bak con data e ora.
ftr-force-explicit-order = Mantieni l'ordine originale
ftr-warn-order = Gli elenchi ordinati per nome (order="Ascending") verrebbero riordinati dal gestore di mod in base ai nomi tradotti. Questa opzione forza order="Explicit" in modo che le opzioni mantengano l'ordine attuale.
ftr-update = Aggiorna dalla cartella
ftr-update-hint = Rilegge il FOMOD dal disco e vi unisce la traduzione: vengono segnalate le stringhe nuove, modificate e rimosse.
ftr-preview-translated = Anteprima tradotta
ftr-progress = { $done } / { $total } tradotte
ftr-filter-all = Tutte
ftr-filter-untranslated = Non tradotte
ftr-filter-review = Da rivedere
ftr-filter-issues = Con problemi
ftr-filter-locked = Bloccate
ftr-type-all = Tutti i campi
ftr-type-names = Nomi
ftr-type-descriptions = Descrizioni
ftr-type-meta = Informazioni sulla mod
ftr-search-hint = Cerca nell'originale, nella traduzione o nel contesto…
ftr-next-untranslated = Prossima non tradotta
ftr-show-whitespace = Mostra spazi e ritorni a capo
ftr-discard-question = La traduzione corrente contiene modifiche non salvate. Scartarle e caricare l'altro FOMOD?
ftr-discard-yes = Scarta
ftr-unsaved-close = La traduzione contiene modifiche non salvate.
ftr-col-num = N.
ftr-col-status = { "" }
ftr-col-context = Contesto
ftr-col-source = Originale
ftr-col-target = Traduzione
ftr-col-issues = { "" }
ftr-empty-hint = Apri una cartella di mod, o carica il progetto attivo, per elencarne le stringhe traducibili.
ftr-empty-filter = Nessuna stringa corrisponde al filtro corrente.
ftr-select-row = Seleziona una riga per modificarne la traduzione.
ftr-copy-source = Copia l'originale
ftr-clear-target = Cancella
ftr-lock = Non tradurre
ftr-lock-hint = Le stringhe bloccate vengono scritte senza modifiche (autore, sito web, nomi propri…).
ftr-note = Nota:
ftr-status-untranslated = Non tradotta
ftr-status-translated = Tradotta
ftr-status-auto = Precompilata automaticamente — da rivedere
ftr-status-fuzzy = Il testo originale è cambiato dopo la traduzione — da rivedere
ftr-status-obsolete = Non più presente nel FOMOD
ftr-status-locked = Bloccata (scritta senza modifiche)
ftr-field-info-name = Nome della mod (info.xml)
ftr-field-module-name = Titolo dell'installer (ModuleConfig.xml)
ftr-field-author = Autore
ftr-field-website = Sito web
ftr-field-description = Descrizione della mod
ftr-field-step = Nome del passaggio
ftr-field-group = Nome del gruppo
ftr-field-plugin = Nome dell'opzione
ftr-field-plugin-desc = Descrizione dell'opzione
ftr-issue-empty = Traduzione vuota
ftr-issue-whitespace = La traduzione contiene solo spazi
ftr-issue-edge-whitespace = Gli spazi iniziali o finali differiscono dall'originale
ftr-issue-token = I token protetti differiscono — mancanti: { $missing } ; in eccesso: { $extra }
ftr-issue-newline-name = Un nome non può contenere un ritorno a capo
ftr-issue-control = Contiene caratteri che XML non può memorizzare
ftr-issue-length = Lunghezza insolita rispetto all'originale (×{ $ratio })
ftr-issue-identical = Identica all'originale
ftr-issue-duplicate = Lo stesso testo originale è tradotto diversamente in { $key }
ftr-issue-cdata = La sequenza ]]> non è consentita qui
ftr-load-error = Impossibile caricare il FOMOD: { $error }
ftr-extracted = { $num } stringhe traducibili trovate.
ftr-sidecar-found = Traduzione esistente caricata e unita: { $new } nuove, { $changed } modificate, { $removed } rimosse.
ftr-saved = Traduzione salvata in { $path }
ftr-save-error = Impossibile salvare la traduzione: { $error }
ftr-save-first = Salva prima il progetto, poi traducilo.
ftr-export-success = { $count } stringhe scritte in { $path }
ftr-export-error = Esportazione non riuscita: { $error }
ftr-export-blocked = { $num } problemi bloccanti devono essere corretti prima dell'esportazione.
ftr-export-stale = { $num } stringhe sono state ignorate perché il FOMOD è cambiato; usa «Aggiorna dalla cartella».
ftr-update-report = Aggiornamento: { $new } nuove, { $changed } modificate, { $moved } spostate, { $removed } rimosse, { $unchanged } invariate.
menu-edit = Modifica
menu-undo = Annulla
menu-redo = Ripeti
tree-title = Progetto
tree-mod-info = Informazioni sul mod
tree-steps = Fasi di installazione
tree-required = File richiesti
tree-conditional = Installazioni condizionali
tree-empty-steps = Ancora nessuna fase — fai clic su + per aggiungerne una.
tree-duplicate = Duplica
tree-delete = Elimina
tree-save-template = Salva come modello…
tree-drop-hint = Rilascia qui per spostare
cond-set-label = Insieme condizionale { $num }
inspector-empty = Seleziona un elemento nell'albero del progetto oppure aggiungi una fase per iniziare.
count-options = { $num } opzioni
count-files = { $num } file
msg-deleted-undo = Eliminato. Usa Annulla (Ctrl+Z) per ripristinarlo.
problems-title = Problemi
problems-errors = { $num } errori
problems-warnings = { $num } avvisi
btn-close = Chiudi
ftr-export-package = Come pacchetto di traduzione (archivio)
ftr-export-package-hint = Crea un file .zip o .7z pronto da caricare: info.xml e ModuleConfig.xml tradotti più un README (solo patch), oppure l'intera mod con i file tradotti (completo).
ftr-package-full = Mod completa
ftr-package-full-hint = Include nell'archivio tutti i file della mod, non solo i due file XML tradotti. Assicurati che l'autore consenta la ridistribuzione.
ftr-package-name-template = Nome:
ftr-readme-patch = Questo archivio contiene la traduzione ({ $langname }) dell'installer di «{ $name }» (fomod/info.xml e fomod/ModuleConfig.xml). Installalo sopra la mod originale, oppure lascia che il tuo gestore di mod lo unisca, in modo che i file tradotti sostituiscano gli originali. Cambiano solo i testi dell'installer; i file della mod non sono inclusi. Realizzato con XIMOD Architect.
ftr-readme-full = Questo archivio contiene «{ $name }» con l'installer tradotto ({ $langname }; fomod/info.xml e fomod/ModuleConfig.xml). Installalo come la mod originale. Sono stati modificati solo i testi dell'installer. Realizzato con XIMOD Architect.
ftr-apply-memory = Compila dalla memoria
ftr-memory-size = Memoria di traduzione: { $num } voci per questa coppia di lingue. Ogni traduzione salvata vi viene aggiunta.
ftr-memory-applied = { $num } stringhe compilate dalla memoria di traduzione (contrassegnate «da rivedere»).
ftr-memory-suggestion = La memoria suggerisce:
ftr-use-suggestion = Usa
ftr-propagate = Propaga alle identiche
ftr-propagate-hint = Copia questa traduzione in tutte le altre stringhe con lo stesso testo originale ancora non tradotte.
ftr-propagated = { $num } stringhe identiche compilate.
ftr-csv-export = Esporta CSV…
ftr-csv-import = Importa CSV…
ftr-csv-imported = { $num } stringhe aggiornate dal file CSV.
ftr-csv-error = Errore CSV: { $error }
ftr-glossary = Glossario
ftr-glossary-source = Termine
ftr-glossary-target = Traduzione
ftr-glossary-case = Maiuscole/minuscole
ftr-glossary-dnt = Mantieni
ftr-glossary-add = Aggiungi termine
ftr-issue-glossary = Glossario: «{ $term }» non è tradotto come previsto

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Apri archivio…
filter-archive = Archivi di mod (zip, 7z)
msg-archive-opened = Archivio aperto ({ $num } file estratti): { $path }
msg-archive-reused = Archivio già estratto, riutilizzo di { $path }
msg-archive-unsupported = Il formato di archivio “.{ $ext }” non è supportato; estrailo prima con 7-Zip (è possibile aprire solo .zip e .7z).
msg-archive-error = Errore durante l'apertura dell'archivio: { $error }
msg-archive-no-fomod = Nessuna cartella “fomod” trovata nell'archivio ({ $path })
msg-archive-extracting = Estrazione dell'archivio…
ftr-open-archive = Apri un archivio di mod…
ftr-package-full-partial = La mod è stata aperta da un archivio contenente solo la cartella fomod; i pacchetti completi richiedono la mod estratta.
info-module-deps = Requisiti della mod
info-module-deps-hint = File o flag richiesti dall'intera mod prima dell'avvio dell'installer (moduleDependencies). Lascia vuoto se non ce ne sono.
info-header-advanced = Intestazione avanzata
info-title-position = Posizione del titolo
info-title-colour = Colore del titolo
info-title-colour-hint = Atteso: sei cifre esadecimali (RRGGBB)
info-image-show = Mostra immagine di intestazione
info-image-fade = Dissolvi immagine di intestazione
info-image-height = Altezza immagine di intestazione
info-attr-default = (predefinito)
file-always-install = Sempre
file-always-install-hint = Installa sempre questo file, anche quando l'opzione non è selezionata (alwaysInstall).
file-install-if-usable = Se usabile
file-install-if-usable-hint = Installa questo file ogni volta che l'opzione è usabile, anche quando non è selezionata (installIfUsable).
msg-import-lossy = Questo FOMOD contiene { $num } costrutti che XIMOD non può modificare; verranno eliminati al salvataggio del progetto.
fidelity-nested-deps = Gruppo di dipendenze annidato in { $context } (è supportato un solo livello)
fidelity-game-dep = Requisito di versione del gioco { $version } in { $context }
fidelity-fomm-dep = Requisito di versione del mod manager { $version } in { $context }
fidelity-unknown = L'elemento “{ $element }” in “{ $parent }” non è supportato ({ $context })
loc-module = i requisiti della mod
loc-step = il passaggio { $step } “{ $name }”
loc-installer = l'installer

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Ripristina un backup…
backups-title = Ripristina un backup
backups-empty = Questo progetto non ha ancora alcun backup. Ne viene creato uno ogni volta che il progetto viene salvato sopra una versione precedente.
backups-changes = { $num } modifica(e) rispetto al progetto attuale
btn-compare = Confronta
btn-restore = Ripristina
btn-delete-backups = Elimina tutti i backup
btn-delete-backups-confirm = Fai di nuovo clic per eliminare tutti i backup
msg-backup-restored = Backup del { $time } ripristinato nell'editor (non ancora salvato; Annulla ripristina lo stato precedente)
msg-backups-deleted = { $num } backup eliminato/i
settings-backup-count = Backup da conservare:
settings-backup-count-hint = Numero di versioni precedenti dell'XML del FOMOD conservate in fomod/backups al salvataggio (0 = nessun backup).
settings-autosave-minutes = Salva automaticamente una copia di recupero ogni (minuti):
settings-autosave-minutes-hint = A questo intervallo viene scritta nella cartella di configurazione una copia di recupero di ogni progetto modificato; viene proposta all'avvio successivo solo dopo una chiusura anomala (0 = disattivato).
settings-auto-masters = Aggiungi i master di un plugin come condizioni
settings-auto-masters-hint = Quando un plugin (.esp/.esm/.esl) viene aggiunto a un'opzione, i master che richiede e che né il gioco né questa mod forniscono diventano condizioni di file “Active” dell'opzione.
msg-author-from-plugin = Autore compilato dall'intestazione del plugin: { $author }
msg-masters-added = { $num } master di { $plugin } aggiunto/i come condizione/i di file
issue-missing-master = { $plugin } richiede { $master }, che non è né in questa mod né dichiarato come dipendenza
issue-esl-mismatch-flag = { $plugin } ha l'estensione .esl ma il suo flag light (ESL) non è impostato
issue-esl-eligible = { $plugin } potrebbe essere contrassegnato come light ({ $num } nuovi record, limite { $limit })
issue-esl-too-big = { $plugin } è contrassegnato come light ma non rispetta le regole dei plugin light ({ $num } nuovi record, limite { $limit }, oppure un FormID fuori dall'intervallo consentito)
menu-plugin-report = Rapporto sui plugin…
plugins-title = Rapporto sui plugin
plugins-file = File
plugins-kind = Tipo
plugins-light = Flag light
plugins-masters = Master
plugins-new-records = Nuovi record / limite
plugins-eligible = Idoneo light
plugins-empty = Questo progetto non installa alcun file plugin (.esp, .esm o .esl).
plugins-unreadable = illeggibile

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Albero finale dei file
preview-total-size = Dimensione totale dell'installazione: { $size }
preview-tree-truncated = L'albero è troncato: troppi file da espandere (le dimensioni sopra sono parziali).
preview-overwritten-by = Sovrascritto da { $plugin }
preview-scenario = Scenario:
preview-scenario-load = Carica
preview-scenario-save = Salva…
preview-scenario-delete = Elimina
preview-scenario-name = Nome dello scenario
preview-scenario-saved = Scenario «{ $name }» salvato in fomod/scenarios
preview-scenario-unresolved = { $num } selezione/i dello scenario senza corrispondenza con alcuna opzione di questo progetto (rinominata o rimossa)
preview-scenario-none = (nessuno scenario)
issue-unreachable-step = La fase «{ $step }» non può mai essere mostrata: le sue condizioni di visibilità verificano un valore di flag che nessuna opzione precedente imposta
issue-unreachable-option = L'opzione «{ $plugin }» non può mai essere selezionata: i suoi pattern di tipo utilizzabile verificano un valore di flag che nessuna opzione imposta
issue-unreachable-cond = L'insieme condizionale di file { $num } non può mai applicarsi: le sue condizioni verificano un valore di flag che nessuna opzione imposta
size-option = Dimensione installata: { $size } ({ $num } file)
size-missing = { $num } sorgente/i mancante/i
size-unknown = Dimensione installata: — (esegui Convalida per misurarla)
menu-nexus-desc = Descrizione Nexus…
nexus-title = Descrizione Nexus Mods
nexus-format = Formato:
nexus-include-requirements = Requisiti
nexus-include-options = Opzioni di installazione
nexus-include-install = Installazione
nexus-include-changelog = Registro delle modifiche
nexus-previous = Versione precedente…
nexus-previous-none = (nessuna versione precedente: nessun registro delle modifiche)
nexus-language = Lingua:
nexus-language-source = (origine)
nexus-sec-requirements = Requisiti
nexus-sec-options = Opzioni di installazione
nexus-sec-install = Installazione
nexus-sec-changelog = Registro delle modifiche
nexus-install-text = Questa mod include un installer FOMOD: installala con un mod manager (Vortex, Mod Organizer 2) e scegli le opzioni nell'installer.
nexus-requires = Richiede
nexus-step = Fase
nexus-added = Aggiunto
nexus-removed = Rimosso
nexus-changed = Modificato
btn-copy = Copia
btn-save-as = Salva con nome…
msg-copied = Copiato negli appunti
msg-saved-to = Salvato in { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Rinomina…
condeditor-rename-exists = Esiste già un flag chiamato «{ $name }»
condeditor-renamed = Flag «{ $from }» rinominato in «{ $to }» ({ $num } occorrenza/e)
condeditor-delete-uses = Elimina tutti gli usi
condeditor-deleted-uses = Flag «{ $name }» rimosso ovunque ({ $num } occorrenza/e)
condeditor-values-set = Valori impostati:
condeditor-values-tested = Valori verificati:
condeditor-value-never-set = { $value } — verificato ma mai impostato
condeditor-value-never-tested = { $value } — impostato ma mai verificato
condeditor-builder = Costruttore di condizioni
condeditor-builder-none = Seleziona una fase, un'opzione, un insieme condizionale di file o le informazioni della mod nella finestra principale per modificarne qui le condizioni.
condeditor-builder-pattern = Schema:
condeditor-sentence-if = SE
condeditor-sentence-and = E
condeditor-sentence-or = O
condeditor-sentence-flag = il flag { "{name}" } = { "{value}" }
condeditor-sentence-file = il file { "{name}" } è { "{value}" }
condeditor-sentence-empty = (nessuna condizione: sempre vera)
condeditor-sentence-then-visible = ALLORA la fase viene mostrata
condeditor-sentence-then-type = ALLORA l'opzione diventa { $type }
condeditor-sentence-then-install = ALLORA i file vengono installati
condeditor-sentence-then-module = ALLORA l'installer può essere eseguito (verificato prima dell'avvio)
issue-flag-value-never-set = Il flag «{ $flag }» viene verificato con il valore «{ $value }», che nessuna opzione imposta
issue-flag-never-used = Il flag «{ $flag }» viene impostato ma non è mai verificato
menu-project-strings = Stringhe del progetto…
strings-title = Stringhe del progetto
strings-search = Cerca testo, posizione o chiave…
strings-kind-all = Tutte
strings-kind-names = Nomi
strings-kind-descriptions = Descrizioni
strings-duplicates-only = Solo duplicati
strings-replace-with = Sostituisci con:
strings-case = Maiuscole/minuscole
strings-whole-word = Parola intera
strings-replace-current = Sostituisci
strings-replace-all = Sostituisci tutto
strings-replaced = { $num } stringa/e sostituita/e
strings-dup-badge = ×{ $num }
strings-dup-hover = Stesso testo di:
strings-count = { $num } stringa/e · { $dups } gruppo/i di duplicati
strings-col-location = Posizione
strings-col-field = Campo
strings-col-text = Testo

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Contenuto di un archivio…
filter-bethesda-archive = Archivi Bethesda (bsa, ba2)
archive-view-title = Contenuto dell'archivio
archive-view-format = Formato:
archive-view-entries = { $num } voci
archive-view-size = { $size } decompressi
archive-view-search = Cerca un percorso…
archive-view-col-path = Percorso
archive-view-col-size = Dimensione
archive-view-col-compressed = Compresso
archive-view-truncated = Sono mostrate solo le prime { $num } voci corrispondenti: affina la ricerca.
archive-view-error = Impossibile leggere questo archivio: { $error }
archive-view-hint = Visualizza il contenuto di questo archivio
issue-conflict-archive = Stessa risorsa in più archivi: «{ $path }» è impacchettato da { $count } riferimenti ({ $locs }): l'ordine di caricamento degli archivi del gioco decide quale viene usato.
issue-conflict-archive-loose = Archivio contro file sciolto: «{ $path }» è sia impacchettato in un archivio sia installato come file sciolto ({ $locs }): il file sciolto prevale su quello archiviato.
preview-in-archive = (in archivio)
preview-archived-size = di cui { $size } impacchettati in archivi

# --- Project tree: expand / collapse menus
tree-expand = Espandi
tree-collapse = Comprimi
tree-expand-all = Espandi tutto
tree-expand-selected = Espandi selezione
tree-expand-from = Espandi dalla selezione
tree-collapse-all = Comprimi tutto
tree-collapse-selected = Comprimi selezione
tree-collapse-from = Comprimi dalla selezione
tree-expand-all-hint = Espande tutte le intestazioni
tree-expand-selected-hint = Espande solo l'intestazione selezionata
tree-expand-from-hint = Espande l'intestazione selezionata e tutto ciò che contiene
tree-collapse-all-hint = Comprime tutte le intestazioni
tree-collapse-selected-hint = Comprime solo l'intestazione selezionata
tree-collapse-from-hint = Comprime l'intestazione selezionata e tutto ciò che contiene

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Aggiungi gruppo
btn-remove-group-cond = Rimuovi gruppo
dep-type-game = Versione del gioco
dep-type-fomm = Versione del mod manager
dep-group-hint = Un gruppo di condizioni combinate con E / O; i gruppi possono essere annidati.
condeditor-sentence-game = la versione del gioco ≥ { "{value}" }
condeditor-sentence-fomm = la versione del mod manager ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Testi unici
ftr-uniques-hint = Mostra una riga per ogni testo originale distinto. Tradurre quella riga traduce in una volta tutte le stringhe con lo stesso testo.
ftr-uniques-synced = { $num } stringhe identiche aggiornate.
ftr-uniques-group = { $num } stringhe condividono questo testo; la sua traduzione si applica a tutte.
