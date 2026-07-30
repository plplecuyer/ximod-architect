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
menu-settings = Einstellungen
menu-pre-save-script = Skript vor dem Speichern…
menu-post-save-script = Skript nach dem Speichern…
menu-translation = Übersetzung…

# Menu - Help
menu-help = Hilfe
menu-about = Über

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
label-plugin-name = Plugin-Name:
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
btn-add-plugin = Plugin hinzufügen
btn-remove-plugin = Plugin entfernen
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
validation-no-plugins = Schritt { $step }, Gruppe „{ $name }“ hat keine Plugins

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
default-plugin-name = Plugin { $num }
pattern-label = Muster { $num }

# Selection prompts
msg-select-group-first = Zuerst eine Gruppe auswählen.
msg-select-plugin-edit = Ein Plugin zum Bearbeiten auswählen.
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
