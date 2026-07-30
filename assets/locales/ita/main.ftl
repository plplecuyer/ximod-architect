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
menu-settings = Impostazioni
menu-pre-save-script = Script pre-salvataggio…
menu-post-save-script = Script post-salvataggio…
menu-translation = Traduzione…

# Menu - Help
menu-help = Aiuto
menu-about = Informazioni

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
label-plugin-name = Nome plugin:
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
btn-add-plugin = Aggiungi plugin
btn-remove-plugin = Rimuovi plugin
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
validation-no-plugins = La fase { $step }, gruppo «{ $name }» non ha plugin

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
default-plugin-name = Plugin { $num }
pattern-label = Schema { $num }

# Selection prompts
msg-select-group-first = Seleziona prima un gruppo.
msg-select-plugin-edit = Seleziona un plugin da modificare.
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
