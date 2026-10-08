# XIMOD Architect - translation metadata
# @language = deu
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Deutsch
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Version { $version }

# Status messages
status-ready = Bereit
msg-save-success = FOMOD erfolgreich gespeichert
msg-save-error = Fehler beim Speichern des FOMOD
msg-export-success = Distributionsarchiv erstellt ({ $count } Dateien): { $path }
msg-export-error = Fehler beim Erstellen des Distributionsarchivs: { $error }
msg-load-success = FOMOD erfolgreich geladen
msg-load-error = Fehler beim Laden des FOMOD
msg-merge-success = FOMOD erfolgreich zusammengeführt
msg-merge-error = Fehler beim Zusammenführen des FOMOD
msg-no-root-selected = Bitte zuerst ein Stammverzeichnis auswählen
msg-no-fomod-folder = Kein „fomod“-Ordner gefunden. Einen erstellen?
msg-file-outside-root = Datei liegt außerhalb des Stammverzeichnisses

# Menu - File
menu-file = Datei
menu-new = Neu
menu-open = Ordner öffnen…
menu-open-file = Datei öffnen…
menu-save = Speichern
menu-recent = Zuletzt verwendet
menu-exit = Beenden
menu-merge = FOMOD zusammenführen…
menu-export = Distributionsarchiv exportieren…
# Menu - Options
menu-options = Optionen
menu-settings = Einstellungen…
menu-pre-save-script = Skript vor dem Speichern…
menu-post-save-script = Skript nach dem Speichern…
menu-translation = Oberfläche übersetzen…
# Menu - Help
menu-help = Hilfe
menu-check-updates = Nach Updates suchen…
menu-about = Über

# Update check
update-checking = Suche nach Updates…
update-up-to-date = XIMOD Architect ist auf dem neuesten Stand.
update-check-failed = Updates konnten nicht geprüft werden. Bitte später erneut versuchen.
update-available-status = Version { $version } ist verfügbar.
update-banner-text = XIMOD Architect { $version } ist verfügbar.
update-download = Herunterladen:
update-skip = Diese Version überspringen
update-later = Später

# Tabs
tab-info = Mod-Info
tab-steps = Installationsschritte
tab-required = Erforderliche Installationen
tab-conditional = Bedingte Installationen

# Info Tab
label-workspace = Arbeitsbereich
label-root-dir = Stammverzeichnis:
label-mod-name = Mod-Name:
label-author = Autor:
label-version = Version:
label-game-name = Spielname:
label-category = Kategorie:
label-url = Website-URL:
label-header-image = Titelbild:
label-description = Beschreibung:
placeholder-select-dir = (Verzeichnis auswählen)
placeholder-select-game = (Spiel auswählen)

# Steps Tab
label-step-name = Schrittname:
label-group-name = Gruppenname:
label-group-type = Gruppentyp:
label-plugin-name = Optionsname:
label-plugin-desc = Beschreibung:
label-plugin-type = Standardtyp:
label-plugin-image = Bild:
label-visibility = Sichtbarkeitsbedingungen
label-operator = Operator:

# Buttons
btn-browse = Durchsuchen…
btn-clear = Leeren
btn-add = Hinzufügen
btn-remove = Entfernen
btn-add-step = Neuer Schritt
btn-delete-step = Schritt löschen
btn-add-group = Gruppe hinzufügen
btn-remove-group = Gruppe entfernen
btn-add-plugin = Option hinzufügen
btn-remove-plugin = Option entfernen
btn-add-file = Datei hinzufügen
btn-add-folder = Ordner hinzufügen
btn-remove-file = Entfernen
btn-add-flag = Flag hinzufügen
btn-remove-flag = Flag entfernen
btn-add-condition = Bedingung hinzufügen
btn-remove-condition = Bedingung entfernen
btn-add-dependency = Abhängigkeit hinzufügen
btn-remove-dependency = Abhängigkeit entfernen
btn-add-pattern = Neues Muster
btn-remove-pattern = Muster löschen
btn-save = Speichern
btn-cancel = Abbrechen
btn-ok = OK
btn-yes = Ja
btn-no = Nein

# Condition/Dependency Labels
label-flag-name = Flag-Name:
label-flag-value = Wert:
label-condition-type = Typ:
label-condition-name = Name:
label-condition-value = Wert:
label-dep-type = Abhängigkeitstyp:
label-dep-name = Name/Datei:
label-dep-value = Wert/Status:

# Files
label-source = Quelle
label-destination = Ziel
label-priority = Priorität
label-file-type = Typ

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Ziel für die ganze Gruppe
label-page-dest = Installationsziel (ganze Seite)
btn-apply-group-dest = Auf alle Optionen dieser Gruppe anwenden
btn-apply-page-dest = Auf alle Optionen dieser Seite anwenden
group-dest-hint = Legt ein Installationsziel für jede Datei jeder Option dieser Gruppe fest.
page-dest-hint = Legt ein Installationsziel für jede Datei jeder Option dieser Seite fest (alle Gruppen).
bulk-dest-nofiles = Noch keine Dateien zum Aktualisieren — fügen Sie zuerst Dateien zu den Optionen hinzu.
status-dest-applied = Ziel auf { $num } Datei(en) angewendet.
preview-hidden-steps = { $num } Schritt(e) durch aktuelle Auswahl ausgeblendet.
label-files = Dateien
label-dependencies = Abhängigkeiten

# Settings Dialog
settings-title = Einstellungen
settings-tab-general = Allgemein
settings-tab-recent-files = Zuletzt verwendete Dateien
settings-language = Sprache:
settings-theme = Design:
settings-font-size = Schriftgröße:
settings-replace-newlines = Zeilenumbrüche in Beschreibungen verarbeiten
settings-check-updates = Beim Start nach Updates suchen
settings-max-recent = Max. zuletzt verwendete Dateien:
settings-window-width = Fensterbreite:
settings-window-height = Fensterhöhe:
settings-no-recent-files = Keine zuletzt verwendeten Dateien.

# Status messages for settings
status-settings-saved = Einstellungen erfolgreich gespeichert

# About Dialog
about-title = Über XIMOD Architect
about-description = Ein plattformübergreifendes Werkzeug zur Erstellung von FOMOD-Installern für Bethesda-Spielmods.
about-license = Lizenziert unter der MIT-Lizenz
about-copyright = © 2024 XIMOD-Team
about-credit = Rust-Portierung des Original-Tools von Wenderer:

# Script Dialog
script-title = Skript bearbeiten
script-info = Skripte werden vor oder nach dem Speichern ausgeführt. Sie können die folgenden Makros verwenden:
script-macros = Verfügbare Makros:
macro-modname = $MODNAME$ - Mod-Name
macro-modauthor = $MODAUTHOR$ - Name des Autors
macro-modversion = $MODVERSION$ - Mod-Version
macro-modroot = $MODROOT$ - Pfad zum Stammverzeichnis
macro-date = $DATE$ - Aktuelles Datum (JJJJ-MM-TT)
macro-time = $TIME$ - Aktuelle Uhrzeit (HH:MM:SS)
macro-random = $RANDOM$ - Zufallszahl

# Plugin Dependencies
label-plugin-dependencies = Optionsabhängigkeiten
label-default-type = Standardtyp:
label-pattern-type = Mustertyp:
label-pattern-operator = Musteroperator:

# Conditional Files
label-pattern = Muster

# Validation Messages
validation-no-name = Mod-Name ist erforderlich
validation-no-steps = Mindestens ein Schritt oder eine erforderliche Datei wird benötigt
validation-empty-step = Schritt { $num } hat keinen Namen
validation-empty-group = Schritt { $step }, Gruppe { $group } hat keinen Namen
validation-no-plugins = Schritt { $step }, Gruppe „{ $name }“ hat keine Optionen

# File States
state-active = Aktiv
state-inactive = Inaktiv
state-missing = Fehlt

# Confirmation
confirm-title = Bestätigung
confirm-delete = Möchten Sie dieses Element wirklich löschen?
confirm-discard = Sie haben ungespeicherte Änderungen. Verwerfen und fortfahren?
confirm-unsaved = Sie haben ungespeicherte Änderungen. Möchten Sie vor dem Schließen speichern?
confirm-save-issues = Das Projekt weist die folgenden Probleme auf:
confirm-save-anyway = Trotzdem speichern?

# Errors
error-invalid-xml = Ungültige XML-Datei
error-parse-failed = FOMOD konnte nicht verarbeitet werden
error-write-failed = Datei konnte nicht geschrieben werden
error-create-dir = Verzeichnis konnte nicht erstellt werden

# Default names (generated when creating new items)
default-step-name = Schritt { $num }
default-group-name = Gruppe { $num }
default-plugin-name = Option { $num }
pattern-label = Muster { $num }

# Selection prompts
msg-select-group-first = Zuerst eine Gruppe auswählen.
msg-select-plugin-edit = Eine Option zum Bearbeiten auswählen.
label-empty = (leer)
image-no-image = Kein Bild

# File dialog filters
filter-images = Bilder
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = Datei

# Status bar
status-modified = Geändert

# Status messages (errors)
msg-settings-save-error = Fehler beim Speichern der Einstellungen
msg-script-save-error = Fehler beim Speichern des Skripts

# Translation editor
trans-title = Übersetzungseditor
trans-source-lang = Angezeigte Sprache:
trans-target-lang = Zu übersetzende Sprache:
trans-col-key = Schlüssel
trans-col-source = Bezeichnung
trans-col-target = Übersetzung
trans-saved = Übersetzung gespeichert
trans-save-error = Fehler beim Speichern der Übersetzung

# XML editor
xml-editor-title = XML-Editor
xml-editor-edit = Bearbeiten
xml-editor-apply = Anwenden
xml-editor-revert = Abbrechen
xml-editor-readonly = Schreibgeschützt
xml-editor-editing = Bearbeitung — grafische Registerkarten sind gesperrt
xml-editor-error = Fehler:
xml-editor-applied = XML-Änderungen angewendet
xml-editor-wellformed = Wohlgeformtes XML
xml-editor-error-at = Zeile { $line }, Spalte { $col }: { $msg }

# Country / flag picker
settings-country-name = Ländername:
settings-pick-country = Klicken, um Ihr Land auszuwählen
flags-title = Ein Land auswählen
flags-filter = Filter:
flags-none = Keine Flagge gefunden

# Translation editor: country & font
trans-endonym = Endonym des Landes:
trans-font = Schriftart:
trans-no-font = (keine)
trans-browse = Durchsuchen…
trans-google-fonts = Google Fonts
trans-pick-country = Klicken, um das Land auszuwählen
trans-font-outside = Die Schriftart muss zuerst in assets/fonts installiert werden.
trans-font-dir-missing = Der Ordner assets/fonts wurde nicht gefunden.

# Translation submission
trans-lang-endonym = Endonym der Sprache:
trans-author = Autor:
trans-submit = Senden…
trans-submit-hint = Ein zip erstellen und eine vorausgefüllte E-Mail öffnen
trans-data-updated = Referenzdaten aktualisiert (Languages.json / Countries.json)
trans-package-ready = Archiv bereit:
trans-package-error = Archiv konnte nicht erstellt werden:

# ISO 639-3 requirement
trans-lang-not-iso = Eine Übersetzung ist nur für eine Sprache mit einem ISO 639-3-Code möglich.

# FOMOD installer preview
menu-preview = Installer-Vorschau…
preview-title = Vorschau des FOMOD-Installers
preview-refresh = Aktualisieren
preview-assumptions = Dateiannahmen
preview-details = Details
preview-back = Zurück
preview-next = Weiter
preview-install = Installieren
preview-close = Schließen
preview-restart = Neu starten
preview-summary-title = Dateien, die installiert werden
preview-empty = Es würde keine Datei installiert.
preview-none-option = (keine)
preview-invalid = Vervollständigen Sie die erforderlichen Auswahlen, um fortzufahren.
preview-no-steps = Kein Schritt ist sichtbar; siehe die Installationszusammenfassung.
preview-select-hint = Wählen Sie eine Option, um ihre Beschreibung anzuzeigen.
preview-col-source = Quelle
preview-col-dest = Ziel
preview-col-priority = Priorität
preview-sel-exactlyone = Wählen Sie genau eine Option.
preview-sel-atmostone = Wählen Sie höchstens eine Option.
preview-sel-any = Wählen Sie eine beliebige Anzahl von Optionen.
preview-sel-all = Alle Optionen werden installiert.
preview-sel-atleastone = Wählen Sie mindestens eine Option.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = FOMOD validieren
validate-report-title = FOMOD-Validierung
validate-ok = Kein Problem gefunden. Das FOMOD entspricht dem Schema.
xml-editor-schema-ok = Entspricht dem ModConfig 5.0-Schema.
xml-editor-schema-issues = Schemaprobleme:
schema-line-col = Zeile { $line }, Sp. { $col }: { $msg }
schema-wrong-root = Unerwartetes Wurzelelement „{ $found }“ (erwartet „{ $expected }“).
schema-unknown = Unerwartetes Element „{ $element }“ in „{ $parent }“.
schema-missing = „{ $parent }“ muss „{ $child }“ enthalten.
schema-needs-one = „{ $parent }“ muss mindestens ein „{ $child }“ enthalten.
schema-too-many = „{ $child }“ darf nur einmal in „{ $parent }“ vorkommen.
schema-missing-attr = Das Attribut „{ $attr }“ ist für „{ $element }“ erforderlich.
schema-bad-enum = Ungültiger Wert „{ $value }“ für { $element }/@{ $attr } (erwartet: { $allowed }).
schema-choose-one = „{ $parent }“ muss genau eines der folgenden enthalten: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Davor verschieben
reorder-after = Danach verschieben

# Country / language database explorer (Properties)
menu-properties = Eigenschaften…
prop-title = Länder-/Sprachdatenbank
prop-tab-countries = Länder
prop-tab-languages = Sprachen
prop-filter = Filter:
prop-official-langs = Amtssprachen
prop-spoken-langs = Gesprochene Sprachen
prop-endonym = Endonym des Landes
prop-font = Schriftart
prop-spoken-in = Gesprochen in
prop-select-country = Wählen Sie ein Land, um seine Details anzuzeigen.
prop-select-lang = Wählen Sie eine Sprache, um ihre Details anzuzeigen.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Die Nexus Mods-Seite des Spiels öffnen

# Referenced-file verification (V2)
verify-no-root = Dateiprüfung übersprungen: kein Stammordner festgelegt
loc-header = Kopfbild
loc-required = erforderliche Dateien
loc-conditional = bedingter Satz { $num }
loc-plugin = Schritt { $step }, Gruppe { $group }, Option „{ $plugin }“
verify-missing-file = Fehlende Datei: { $path } ({ $loc })
verify-missing-folder = Fehlender Ordner: { $path } ({ $loc })
verify-missing-image = Fehlendes Bild: { $path } ({ $loc })
verify-absolute = Absoluter Pfad (nicht portabel): { $path } ({ $loc })
verify-outside = Pfad verlässt den Stammordner: { $path } ({ $loc })
verify-orphan = Verwaiste Datei (von keiner Option referenziert): { $path }
conflict-certain = Zielkonflikt: „{ $path }“ wird von { $count } Optionen ({ $locs }) geschrieben – sie überschreiben sich gegenseitig.
conflict-potential = Möglicher Zielkonflikt: „{ $path }“ wird von { $count } Verweisen ({ $locs }) angesteuert – das Überschreiben hängt von Auswahl/Bedingungen ab.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = FOMOD schließen
menu-close-all-fomods = Alle FOMODs schließen
tab-untitled = (ohne Titel)
msg-drop-not-fomod = Das abgelegte Element ist kein FOMOD (kein Ordner „fomod“ gefunden)
exit-title = Ungespeicherte Änderungen
exit-unsaved = Ein FOMOD wurde nicht gespeichert. Möchten Sie es speichern?
tab-close-hint = Dieses FOMOD schließen
menu-new-from-folder = Neu aus Ordner…
menu-templates = Vorlagen…
templates-title = Wiederverwendbare Vorlagen
templates-empty = Noch keine Vorlagen gespeichert. Speichern Sie oben den ausgewählten Schritt, um eine zu erstellen.
templates-insert = Einfügen
templates-save-step = Ausgewählten Schritt speichern
templates-name-hint = Vorlagenname (optional)
msg-wizard-success = Gerüst aus Ordner erstellt: { $num } Option(en).
msg-wizard-error = Fehler: { $error }
msg-template-saved = Vorlage gespeichert: { $name }
msg-template-inserted = Vorlage ins Projekt eingefügt.
msg-template-no-step = Wählen Sie zuerst einen Schritt aus, um ihn als Vorlage zu speichern.
msg-template-no-dir = Vorlagenverzeichnis konnte nicht gefunden werden.
msg-drop-assigned = { $added } Quelle(n) zur Option hinzugefügt ({ $rejected } außerhalb des Stammordners ignoriert).
menu-compare = Vergleichen mit…
compare-title = FOMOD-Vergleich
compare-none = Keine Unterschiede.
btn-optimize-image = Bild optimieren
msg-image-optimized = Titelbild optimiert.
msg-image-ok = Titelbild liegt bereits innerhalb der Grenzen.
msg-no-header-image = Kein Titelbild zum Optimieren.
verify-image-large = Bild zu groß ({ $width }×{ $height }): { $path }
verify-image-format = Nicht unterstütztes Bildformat (.{ $ext }): { $path }
verify-image-unreadable = Unlesbares Bild: { $path }
menu-condition-editor = Bedingungseditor…
condeditor-title = Bedingungseditor
condeditor-set-by = Gesetzt von:
condeditor-used-by = Verwendet von:
condeditor-filedeps = Dateiabhängigkeiten
condeditor-empty = Keine Flags oder Abhängigkeiten in diesem Projekt.
condeditor-orphan-set = gesetzt, aber nie verwendet
condeditor-orphan-used = verwendet, aber nie gesetzt
msg-img-optimized = Bild optimiert.
msg-img-ok = Bild bereits innerhalb der Grenzen.
msg-img-none = Kein Bild zum Optimieren.
msg-crash-recovery = Die vorherige Sitzung wurde unerwartet beendet. Eine Sicherung Ihres Projekts wurde unter { $path } gespeichert
export-progress-title = Distributionsarchiv wird erstellt…
export-progress-files = { $done } / { $total } Dateien
msg-export-cancelled = Export abgebrochen; das unvollständige Archiv wurde entfernt.
verify-running = Dateien auf dem Datenträger werden geprüft…
verify-stale = Hinweis: Das Projekt wurde während der Dateiprüfung geändert; bitte die Validierung erneut ausführen.
prop-col-name = Name
menu-save-as = Speichern unter…
menu-project = Projekt
menu-tools = Werkzeuge
menu-manual = Benutzerhandbuch
msg-manual-missing = Das Benutzerhandbuch (PDF) wurde neben der Anwendung nicht gefunden.
toolbar-new = Neu
toolbar-open = Öffnen
toolbar-save = Speichern
toolbar-validate = Prüfen
toolbar-preview = Vorschau
toolbar-export = Exportieren
dialog-choose-root = Stammordner der Mod wählen
exit-unsaved-docs = Nicht gespeichert: { $names }
status-summary = { $steps } Schritte · { $options } Optionen
section-groups = Gruppen
section-options = Optionen
section-flags = Bedingungs-Flags
section-files = Zu installierende Dateien
hint-group-type = Wie der Installer den Benutzer Optionen in dieser Gruppe auswählen lässt.
hint-default-type = Wie die Option angeboten wird, wenn keines ihrer Abhängigkeitsmuster zutrifft: erforderlich, optional, empfohlen, nicht verwendbar…
hint-operator = Alle Bedingungen müssen zutreffen (UND) oder eine beliebige davon (ODER).
hint-flags = Flags sind benannte Werte, die diese Option bei Auswahl setzt. Andere Schritte und Optionen können sie prüfen, um sich anzuzeigen, auszublenden oder erforderlich zu machen.
hint-plugin-dependencies = Muster, die den Typ der Option je nach Flags oder im Spiel vorhandenen Dateien ändern: zum Beispiel „Erforderlich“, wenn eine andere Mod installiert ist.
hint-files = Dateien und Ordner, die bei Auswahl dieser Option in den Data-Ordner des Spiels kopiert werden. Das Ziel ist relativ zu Data; bei Konflikten gewinnt die höhere Priorität.
hint-visibility = Bedingungen, die erfüllt sein müssen, damit dieser Schritt überhaupt angezeigt wird. Leer lassen, um ihn immer anzuzeigen.
seltype-exactly-one = Genau eine (erforderlich)
seltype-at-most-one = Höchstens eine
seltype-any = Beliebig viele
seltype-all = Alle (keine Auswahl)
seltype-at-least-one = Mindestens eine
plugtype-required = Erforderlich
plugtype-optional = Optional
plugtype-recommended = Empfohlen
plugtype-not-usable = Nicht verwendbar
plugtype-could-be-usable = Eventuell verwendbar
plugtype-required-hint = Wird immer installiert; der Benutzer kann sie nicht abwählen.
plugtype-optional-hint = Wird abgewählt angeboten; der Benutzer entscheidet.
plugtype-recommended-hint = Wird vorausgewählt angeboten; der Benutzer kann sie abwählen.
plugtype-not-usable-hint = Wird ausgegraut angezeigt und kann nicht gewählt werden.
plugtype-could-be-usable-hint = Wählbar, aber der Installer warnt, dass sie möglicherweise nicht funktioniert.
op-and = Alle Bedingungen (UND)
op-or = Eine beliebige Bedingung (ODER)
theme-dark = Dunkel
theme-light = Hell
theme-system = System folgen
condeditor-setter-loc = Schritt { "{step}" } / Gruppe { "{group}" } / „{ "{name}" }“
condeditor-pattern-of = Muster von „{ "{name}" }“ → { "{type}" }
condeditor-visibility-of = Sichtbarkeit von Schritt { "{step}" }
condeditor-cond-set = Bedingter Satz { "{num}" }
condeditor-needs = { "{ctx}" } (erwartet = { "{value}" })
condeditor-file-dep = { "{ctx}" }: Datei „{ "{name}" }“ ({ "{state}" })
menu-translate-fomod = FOMOD übersetzen…
ftr-title = FOMOD übersetzen
ftr-open-folder = Mod-Ordner öffnen…
ftr-from-active = Aus dem aktiven Projekt
ftr-from-active-hint = Übersetzt das FOMOD des im Hauptfenster geöffneten Projekts (es muss zuerst gespeichert werden).
ftr-no-fomod = Kein FOMOD geladen.
ftr-encoding = Kodierung der Originaldateien; die übersetzten Dateien werden mit derselben Kodierung geschrieben.
ftr-source-lang = Von
ftr-target-lang = nach
ftr-lang-locked = (die Sprachen stehen fest, sobald ein FOMOD geladen ist)
ftr-translator = Übersetzer:
ftr-save = Übersetzung speichern
ftr-export = Übersetzte Dateien exportieren
ftr-export-sibling = In einen Ordner fomod_<Sprache>
ftr-export-sibling-hint = Schreibt die übersetzten Dateien info.xml und ModuleConfig.xml neben den ursprünglichen fomod-Ordner; die Originaldateien bleiben unberührt.
ftr-export-inplace = Über die Originaldateien
ftr-export-inplace-hint = Ersetzt fomod/info.xml und fomod/ModuleConfig.xml, nachdem von jeder Datei eine .bak-Kopie mit Zeitstempel angelegt wurde.
ftr-force-explicit-order = Ursprüngliche Reihenfolge beibehalten
ftr-warn-order = Nach Namen sortierte Listen (order="Ascending") würden vom Mod-Manager nach den übersetzten Namen neu sortiert. Diese Option erzwingt order="Explicit", damit die Optionen ihre aktuelle Reihenfolge behalten.
ftr-update = Aus Ordner aktualisieren
ftr-update-hint = Liest das FOMOD erneut vom Datenträger und führt die Übersetzung damit zusammen: neue, geänderte und entfernte Texte werden gemeldet.
ftr-preview-translated = Übersetzte Vorschau
ftr-progress = { $done } / { $total } übersetzt
ftr-filter-all = Alle
ftr-filter-untranslated = Nicht übersetzt
ftr-filter-review = Zu prüfen
ftr-filter-issues = Mit Problemen
ftr-filter-locked = Gesperrt
ftr-type-all = Alle Felder
ftr-type-names = Namen
ftr-type-descriptions = Beschreibungen
ftr-type-meta = Mod-Informationen
ftr-search-hint = In Quelltext, Übersetzung oder Kontext suchen…
ftr-next-untranslated = Nächster nicht übersetzter Text
ftr-show-whitespace = Leerzeichen und Zeilenumbrüche anzeigen
ftr-discard-question = Die aktuelle Übersetzung enthält ungespeicherte Änderungen. Verwerfen und das andere FOMOD laden?
ftr-discard-yes = Verwerfen
ftr-unsaved-close = Die Übersetzung enthält ungespeicherte Änderungen.
ftr-col-num = Nr.
ftr-col-status = { "" }
ftr-col-context = Kontext
ftr-col-source = Quelltext
ftr-col-target = Übersetzung
ftr-col-issues = { "" }
ftr-empty-hint = Öffnen Sie einen Mod-Ordner oder laden Sie das aktive Projekt, um dessen übersetzbare Texte aufzulisten.
ftr-empty-filter = Kein Text entspricht dem aktuellen Filter.
ftr-select-row = Wählen Sie eine Zeile aus, um ihre Übersetzung zu bearbeiten.
ftr-copy-source = Quelltext kopieren
ftr-clear-target = Leeren
ftr-lock = Nicht übersetzen
ftr-lock-hint = Gesperrte Texte werden unverändert geschrieben (Autor, Website, Eigennamen…).
ftr-note = Notiz:
ftr-status-untranslated = Nicht übersetzt
ftr-status-translated = Übersetzt
ftr-status-auto = Automatisch vorausgefüllt — bitte prüfen
ftr-status-fuzzy = Der Quelltext hat sich seit der Übersetzung geändert — bitte prüfen
ftr-status-obsolete = Im FOMOD nicht mehr vorhanden
ftr-status-locked = Gesperrt (wird unverändert geschrieben)
ftr-field-info-name = Mod-Name (info.xml)
ftr-field-module-name = Titel des Installers (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Website
ftr-field-description = Mod-Beschreibung
ftr-field-step = Schrittname
ftr-field-group = Gruppenname
ftr-field-plugin = Optionsname
ftr-field-plugin-desc = Optionsbeschreibung
ftr-issue-empty = Leere Übersetzung
ftr-issue-whitespace = Die Übersetzung besteht nur aus Leerzeichen
ftr-issue-edge-whitespace = Leerzeichen am Anfang oder Ende weichen vom Quelltext ab
ftr-issue-token = Geschützte Token weichen ab — fehlend: { $missing } ; überzählig: { $extra }
ftr-issue-newline-name = Ein Name darf keinen Zeilenumbruch enthalten
ftr-issue-control = Enthält Zeichen, die XML nicht speichern kann
ftr-issue-length = Ungewöhnliche Länge im Vergleich zum Quelltext (×{ $ratio })
ftr-issue-identical = Identisch mit dem Quelltext
ftr-issue-duplicate = Derselbe Quelltext ist in { $key } anders übersetzt
ftr-issue-cdata = Die Zeichenfolge ]]> ist hier nicht zulässig
ftr-load-error = Das FOMOD konnte nicht geladen werden: { $error }
ftr-extracted = { $num } übersetzbare Texte gefunden.
ftr-sidecar-found = Vorhandene Übersetzung geladen und zusammengeführt: { $new } neu, { $changed } geändert, { $removed } entfernt.
ftr-saved = Übersetzung gespeichert in { $path }
ftr-save-error = Die Übersetzung konnte nicht gespeichert werden: { $error }
ftr-save-first = Speichern Sie zuerst das Projekt und übersetzen Sie es dann.
ftr-export-success = { $count } Texte nach { $path } geschrieben
ftr-export-error = Export fehlgeschlagen: { $error }
ftr-export-blocked = { $num } blockierende Probleme müssen vor dem Export behoben werden.
ftr-export-stale = { $num } Texte wurden übersprungen, weil sich das FOMOD geändert hat; verwenden Sie „Aus Ordner aktualisieren“.
ftr-update-report = Aktualisiert: { $new } neu, { $changed } geändert, { $moved } verschoben, { $removed } entfernt, { $unchanged } unverändert.
menu-edit = Bearbeiten
menu-undo = Rückgängig
menu-redo = Wiederholen
tree-title = Projekt
tree-mod-info = Mod-Informationen
tree-steps = Installationsschritte
tree-required = Erforderliche Dateien
tree-conditional = Bedingte Installationen
tree-empty-steps = Noch kein Schritt — klicken Sie auf +, um einen hinzuzufügen.
tree-duplicate = Duplizieren
tree-delete = Löschen
tree-save-template = Als Vorlage speichern…
tree-drop-hint = Zum Verschieben hier ablegen
cond-set-label = Bedingter Satz { $num }
inspector-empty = Wählen Sie ein Element im Projektbaum aus oder fügen Sie einen Schritt hinzu, um zu beginnen.
count-options = { $num } Optionen
count-files = { $num } Dateien
msg-deleted-undo = Gelöscht. Mit „Rückgängig“ (Ctrl+Z) stellen Sie es wieder her.
problems-title = Probleme
problems-errors = { $num } Fehler
problems-warnings = { $num } Warnungen
btn-close = Schließen
ftr-export-package = Als Übersetzungspaket (Archiv)
ftr-export-package-hint = Erstellt ein .zip oder .7z zum Hochladen: die übersetzten Dateien info.xml und ModuleConfig.xml plus eine README (nur Patch) oder die gesamte Mod mit den übersetzten Dateien (vollständig).
ftr-package-full = Vollständige Mod
ftr-package-full-hint = Alle Dateien der Mod in das Archiv aufnehmen, nicht nur die beiden übersetzten XML-Dateien. Stellen Sie sicher, dass der Autor die Weiterverbreitung erlaubt.
ftr-package-name-template = Name:
ftr-readme-patch = Dieses Archiv enthält die Übersetzung ({ $langname }) des Installers von „{ $name }“ (fomod/info.xml und fomod/ModuleConfig.xml). Installieren Sie es über die ursprüngliche Mod oder lassen Sie es von Ihrem Mod-Manager zusammenführen, damit die übersetzten Dateien die Originale ersetzen. Nur die Texte des Installers ändern sich; die Dateien der Mod selbst sind nicht enthalten. Erstellt mit XIMOD Architect.
ftr-readme-full = Dieses Archiv enthält „{ $name }“ mit übersetztem Installer ({ $langname }; fomod/info.xml und fomod/ModuleConfig.xml). Installieren Sie es wie die ursprüngliche Mod. Nur die Texte des Installers wurden geändert. Erstellt mit XIMOD Architect.
ftr-apply-memory = Aus Speicher ausfüllen
ftr-memory-size = Übersetzungsspeicher: { $num } Einträge für dieses Sprachpaar. Jede gespeicherte Übersetzung wird hinzugefügt.
ftr-memory-applied = { $num } Texte aus dem Übersetzungsspeicher ausgefüllt (als „zu prüfen“ markiert).
ftr-memory-suggestion = Vorschlag aus dem Speicher:
ftr-use-suggestion = Übernehmen
ftr-propagate = Auf identische übertragen
ftr-propagate-hint = Kopiert diese Übersetzung in alle anderen noch nicht übersetzten Texte mit demselben Quelltext.
ftr-propagated = { $num } identische Texte ausgefüllt.
ftr-csv-export = CSV exportieren…
ftr-csv-import = CSV importieren…
ftr-csv-imported = { $num } Texte aus der CSV-Datei aktualisiert.
ftr-csv-error = CSV-Fehler: { $error }
ftr-glossary = Glossar
ftr-glossary-source = Begriff
ftr-glossary-target = Übersetzung
ftr-glossary-case = Groß-/Kleinschreibung
ftr-glossary-dnt = Beibehalten
ftr-glossary-add = Begriff hinzufügen
ftr-issue-glossary = Glossar: „{ $term }“ ist nicht wie erwartet übersetzt

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Archiv öffnen…
filter-archive = Mod-Archive (zip, 7z)
msg-archive-opened = Archiv geöffnet ({ $num } Dateien entpackt): { $path }
msg-archive-reused = Archiv bereits entpackt, { $path } wird wiederverwendet
msg-archive-unsupported = Das Archivformat „.{ $ext }“ wird nicht unterstützt; entpacken Sie es zuerst mit 7-Zip (nur .zip und .7z können geöffnet werden).
msg-archive-error = Fehler beim Öffnen des Archivs: { $error }
msg-archive-no-fomod = Kein „fomod“-Ordner im Archiv gefunden ({ $path })
msg-archive-extracting = Archiv wird entpackt…
ftr-open-archive = Mod-Archiv öffnen…
ftr-package-full-partial = Die Mod wurde aus einem Archiv geöffnet, das nur ihren fomod-Ordner enthält; vollständige Pakete benötigen die entpackte Mod.
info-module-deps = Mod-Voraussetzungen
info-module-deps-hint = Dateien oder Flags, die die gesamte Mod benötigt, bevor der Installer startet (moduleDependencies). Leer lassen, wenn keine nötig sind.
info-header-advanced = Erweiterte Kopfzeile
info-title-position = Titelposition
info-title-colour = Titelfarbe
info-title-colour-hint = Erwartet: sechs Hexadezimalziffern (RRGGBB)
info-image-show = Kopfbild anzeigen
info-image-fade = Kopfbild ausblenden
info-image-height = Höhe des Kopfbilds
info-attr-default = (Standard)
file-always-install = Immer
file-always-install-hint = Diese Datei immer installieren, auch wenn die Option nicht ausgewählt ist (alwaysInstall).
file-install-if-usable = Wenn nutzbar
file-install-if-usable-hint = Diese Datei installieren, sobald die Option nutzbar ist, auch wenn sie nicht ausgewählt ist (installIfUsable).
msg-import-lossy = Diese FOMOD enthält { $num } Konstrukte, die XIMOD nicht bearbeiten kann; sie gehen beim Speichern des Projekts verloren.
fidelity-nested-deps = Verschachtelte Abhängigkeitsgruppe in { $context } (nur eine Ebene wird unterstützt)
fidelity-game-dep = Anforderung an die Spielversion { $version } in { $context }
fidelity-fomm-dep = Anforderung an die Mod-Manager-Version { $version } in { $context }
fidelity-unknown = Das Element „{ $element }“ in „{ $parent }“ wird nicht unterstützt ({ $context })
loc-module = den Mod-Voraussetzungen
loc-step = Schritt { $step } „{ $name }“
loc-installer = dem Installer

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Sicherung wiederherstellen…
backups-title = Sicherung wiederherstellen
backups-empty = Dieses Projekt hat noch keine Sicherung. Eine wird bei jedem Speichern über eine vorherige Version angelegt.
backups-changes = { $num } Änderung(en) gegenüber dem aktuellen Projekt
btn-compare = Vergleichen
btn-restore = Wiederherstellen
btn-delete-backups = Alle Sicherungen löschen
btn-delete-backups-confirm = Erneut klicken, um alle Sicherungen zu löschen
msg-backup-restored = Sicherung vom { $time } im Editor wiederhergestellt (noch nicht gespeichert; „Rückgängig“ stellt den vorherigen Stand wieder her)
msg-backups-deleted = { $num } Sicherung(en) gelöscht
settings-backup-count = Zu behaltende Sicherungen:
settings-backup-count-hint = Anzahl der vorherigen Versionen des FOMOD-XML, die beim Speichern unter fomod/backups behalten werden (0 = keine Sicherungen).
settings-autosave-minutes = Wiederherstellungskopie automatisch speichern alle (Minuten):
settings-autosave-minutes-hint = In diesem Intervall wird von jedem geänderten Projekt eine Wiederherstellungskopie im Konfigurationsordner geschrieben; sie wird beim nächsten Start nur nach einem unerwarteten Beenden angeboten (0 = aus).
settings-auto-masters = Die Master eines Plugins als Bedingungen hinzufügen
settings-auto-masters-hint = Wenn ein Plugin (.esp/.esm/.esl) zu einer Option hinzugefügt wird, werden die von ihm benötigten Master, die weder das Spiel noch diese Mod bereitstellt, zu „Active“-Dateibedingungen der Option.
msg-author-from-plugin = Autor aus dem Plugin-Header übernommen: { $author }
msg-masters-added = { $num } Master von { $plugin } als Dateibedingung(en) hinzugefügt
issue-missing-master = { $plugin } benötigt { $master }, das weder in dieser Mod enthalten noch als Abhängigkeit deklariert ist
issue-esl-mismatch-flag = { $plugin } hat die Endung .esl, aber sein Light-Flag (ESL) ist nicht gesetzt
issue-esl-eligible = { $plugin } könnte als Light markiert werden ({ $num } neue Datensätze, Limit { $limit })
issue-esl-too-big = { $plugin } ist als Light markiert, erfüllt aber die Regeln für Light-Plugins nicht ({ $num } neue Datensätze, Limit { $limit }, oder eine FormID außerhalb des erlaubten Bereichs)
menu-plugin-report = Plugin-Bericht…
plugins-title = Plugin-Bericht
plugins-file = Datei
plugins-kind = Art
plugins-light = Light-Flag
plugins-masters = Master
plugins-new-records = Neue Datensätze / Limit
plugins-eligible = Light-geeignet
plugins-empty = Dieses Projekt installiert keine Plugin-Datei (.esp, .esm oder .esl).
plugins-unreadable = unlesbar

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Endgültiger Dateibaum
preview-total-size = Gesamte Installationsgröße: { $size }
preview-tree-truncated = Der Baum ist gekürzt: zu viele Dateien zum Aufklappen (die Größen oben sind unvollständig).
preview-overwritten-by = Überschrieben durch { $plugin }
preview-scenario = Szenario:
preview-scenario-load = Laden
preview-scenario-save = Speichern…
preview-scenario-delete = Löschen
preview-scenario-name = Szenarioname
preview-scenario-saved = Szenario „{ $name }“ unter fomod/scenarios gespeichert
preview-scenario-unresolved = { $num } Auswahl(en) des Szenarios passen zu keiner Option dieses Projekts (umbenannt oder entfernt)
preview-scenario-none = (kein Szenario)
issue-unreachable-step = Schritt „{ $step }“ kann nie angezeigt werden: seine Sichtbarkeitsbedingungen prüfen einen Flag-Wert, den keine frühere Option setzt
issue-unreachable-option = Option „{ $plugin }“ kann nie ausgewählt werden: ihre Muster für den nutzbaren Typ prüfen einen Flag-Wert, den keine Option setzt
issue-unreachable-cond = Der bedingte Dateisatz { $num } kann nie greifen: seine Bedingungen prüfen einen Flag-Wert, den keine Option setzt
size-option = Installationsgröße: { $size } ({ $num } Datei(en))
size-missing = { $num } fehlende Quelle(n)
size-unknown = Installationsgröße: — (zum Messen „Validieren“ ausführen)
menu-nexus-desc = Nexus-Beschreibung…
nexus-title = Nexus Mods-Beschreibung
nexus-format = Format:
nexus-include-requirements = Anforderungen
nexus-include-options = Installationsoptionen
nexus-include-install = Installation
nexus-include-changelog = Änderungsprotokoll
nexus-previous = Vorherige Version…
nexus-previous-none = (keine vorherige Version: kein Änderungsprotokoll)
nexus-language = Sprache:
nexus-language-source = (Quelle)
nexus-sec-requirements = Anforderungen
nexus-sec-options = Installationsoptionen
nexus-sec-install = Installation
nexus-sec-changelog = Änderungsprotokoll
nexus-install-text = Diese Mod wird mit einem FOMOD-Installer ausgeliefert: mit einem Mod-Manager (Vortex, Mod Organizer 2) installieren und die gewünschten Optionen im Installer auswählen.
nexus-requires = Benötigt
nexus-step = Schritt
nexus-added = Hinzugefügt
nexus-removed = Entfernt
nexus-changed = Geändert
btn-copy = Kopieren
btn-save-as = Speichern unter…
msg-copied = In die Zwischenablage kopiert
msg-saved-to = Gespeichert unter { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Umbenennen…
condeditor-rename-exists = Ein Flag namens „{ $name }“ existiert bereits
condeditor-renamed = Flag „{ $from }“ in „{ $to }“ umbenannt ({ $num } Vorkommen)
condeditor-delete-uses = Alle Verwendungen löschen
condeditor-deleted-uses = Flag „{ $name }“ überall entfernt ({ $num } Vorkommen)
condeditor-values-set = Gesetzte Werte:
condeditor-values-tested = Geprüfte Werte:
condeditor-value-never-set = { $value } — geprüft, aber nie gesetzt
condeditor-value-never-tested = { $value } — gesetzt, aber nie geprüft
condeditor-builder = Bedingungsbaukasten
condeditor-builder-none = Wählen Sie im Hauptfenster einen Schritt, eine Option, einen bedingten Dateisatz oder die Mod-Informationen aus, um deren Bedingungen hier zu bearbeiten.
condeditor-builder-pattern = Muster:
condeditor-sentence-if = WENN
condeditor-sentence-and = UND
condeditor-sentence-or = ODER
condeditor-sentence-flag = Flag { "{name}" } = { "{value}" }
condeditor-sentence-file = Datei { "{name}" } ist { "{value}" }
condeditor-sentence-empty = (keine Bedingung: immer wahr)
condeditor-sentence-then-visible = DANN wird der Schritt angezeigt
condeditor-sentence-then-type = DANN wird die Option { $type }
condeditor-sentence-then-install = DANN werden die Dateien installiert
condeditor-sentence-then-module = DANN kann der Installer laufen (wird vor dem Start geprüft)
issue-flag-value-never-set = Flag „{ $flag }“ wird mit dem Wert „{ $value }“ geprüft, den keine Option setzt
issue-flag-never-used = Flag „{ $flag }“ wird gesetzt, aber nirgends geprüft
menu-project-strings = Projekttexte…
strings-title = Projekttexte
strings-search = Text, Ort oder Schlüssel suchen…
strings-kind-all = Alle
strings-kind-names = Namen
strings-kind-descriptions = Beschreibungen
strings-duplicates-only = Nur Duplikate
strings-replace-with = Ersetzen durch:
strings-case = Groß-/Kleinschreibung beachten
strings-whole-word = Ganzes Wort
strings-replace-current = Ersetzen
strings-replace-all = Alle ersetzen
strings-replaced = { $num } Text(e) ersetzt
strings-dup-badge = ×{ $num }
strings-dup-hover = Gleicher Text wie:
strings-count = { $num } Text(e) · { $dups } Duplikatgruppe(n)
strings-col-location = Ort
strings-col-field = Feld
strings-col-text = Text

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Archivinhalt…
filter-bethesda-archive = Bethesda-Archive (bsa, ba2)
archive-view-title = Archivinhalt
archive-view-format = Format:
archive-view-entries = { $num } Einträge
archive-view-size = { $size } entpackt
archive-view-search = Pfad suchen…
archive-view-col-path = Pfad
archive-view-col-size = Größe
archive-view-col-compressed = Komprimiert
archive-view-truncated = Nur die ersten { $num } passenden Einträge werden angezeigt – Suche eingrenzen.
archive-view-error = Dieses Archiv kann nicht gelesen werden: { $error }
archive-view-hint = Inhalt dieses Archivs anzeigen
issue-conflict-archive = Gleiches Asset in mehreren Archiven: „{ $path }“ wird von { $count } Verweisen ({ $locs }) gepackt – die Archiv-Ladereihenfolge des Spiels entscheidet, welches verwendet wird.
issue-conflict-archive-loose = Archiv gegen lose Datei: „{ $path }“ ist sowohl in einem Archiv gepackt als auch lose installiert ({ $locs }) – die lose Datei hat Vorrang vor der archivierten.
preview-in-archive = (im Archiv)
preview-archived-size = davon { $size } in Archiven gepackt

# --- Project tree: expand / collapse menus
tree-expand = Aufklappen
tree-collapse = Zuklappen
tree-expand-all = Alle aufklappen
tree-expand-selected = Auswahl aufklappen
tree-expand-from = Ab Auswahl aufklappen
tree-collapse-all = Alle zuklappen
tree-collapse-selected = Auswahl zuklappen
tree-collapse-from = Ab Auswahl zuklappen
tree-expand-all-hint = Klappt alle Überschriften auf
tree-expand-selected-hint = Klappt nur die ausgewählte Überschrift auf
tree-expand-from-hint = Klappt die ausgewählte Überschrift und alles darunter auf
tree-collapse-all-hint = Klappt alle Überschriften zu
tree-collapse-selected-hint = Klappt nur die ausgewählte Überschrift zu
tree-collapse-from-hint = Klappt die ausgewählte Überschrift und alles darunter zu

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Gruppe hinzufügen
btn-remove-group-cond = Gruppe entfernen
dep-type-game = Spielversion
dep-type-fomm = Mod-Manager-Version
dep-group-hint = Eine Gruppe von Bedingungen, die mit UND / ODER verknüpft sind; Gruppen können verschachtelt werden.
condeditor-sentence-game = Spielversion ≥ { "{value}" }
condeditor-sentence-fomm = Mod-Manager-Version ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Eindeutige Texte
ftr-uniques-hint = Zeigt eine Zeile pro unterschiedlichem Quelltext. Wird diese Zeile übersetzt, werden alle Texte mit demselben Inhalt auf einmal übersetzt.
ftr-uniques-synced = { $num } identische Texte aktualisiert.
ftr-uniques-group = { $num } Texte teilen diesen Inhalt; die Übersetzung gilt für alle.
