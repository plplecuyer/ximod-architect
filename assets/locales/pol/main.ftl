# XIMOD Architect - translation metadata
# @language = pol
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Polski
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Wersja { $version }

# Status messages
status-ready = Gotowe
msg-save-success = Pomyślnie zapisano FOMOD
msg-save-error = Błąd podczas zapisywania FOMOD
msg-export-success = Utworzono archiwum dystrybucyjne ({ $count } plików): { $path }
msg-export-error = Błąd podczas tworzenia archiwum dystrybucyjnego: { $error }
msg-load-success = Pomyślnie wczytano FOMOD
msg-load-error = Błąd podczas wczytywania FOMOD
msg-merge-success = Pomyślnie scalono FOMOD
msg-merge-error = Błąd podczas scalania FOMOD
msg-no-root-selected = Najpierw wybierz katalog główny
msg-no-fomod-folder = Nie znaleziono folderu „fomod”. Utworzyć go?
msg-file-outside-root = Plik znajduje się poza katalogiem głównym

# Menu - File
menu-file = Plik
menu-new = Nowy
menu-open = Otwórz folder…
menu-open-file = Otwórz plik…
menu-save = Zapisz
menu-recent = Ostatnie
menu-exit = Zakończ
menu-merge = Scal FOMOD…
menu-export = Eksportuj archiwum dystrybucyjne...

# Menu - Options
menu-options = Opcje
menu-settings = Ustawienia
menu-pre-save-script = Skrypt przed zapisem…
menu-post-save-script = Skrypt po zapisie…
menu-translation = Tłumaczenie...

# Menu - Help
menu-help = Pomoc
menu-about = O programie

# Tabs
tab-info = Informacje o modzie
tab-steps = Kroki instalacji
tab-required = Instalacje wymagane
tab-conditional = Instalacje warunkowe

# Info Tab
label-workspace = Obszar roboczy
label-root-dir = Katalog główny:
label-mod-name = Nazwa moda:
label-author = Autor:
label-version = Wersja:
label-game-name = Nazwa gry:
label-category = Kategoria:
label-url = Adres URL witryny:
label-header-image = Obraz nagłówka:
label-description = Opis:
placeholder-select-dir = (Wybierz katalog)
placeholder-select-game = (Wybierz grę)

# Steps Tab
label-step-name = Nazwa kroku:
label-group-name = Nazwa grupy:
label-group-type = Typ grupy:
label-plugin-name = Nazwa wtyczki:
label-plugin-desc = Opis:
label-plugin-type = Typ domyślny:
label-plugin-image = Obraz:
label-visibility = Warunki widoczności
label-operator = Operator:

# Buttons
btn-browse = Przeglądaj…
btn-clear = Wyczyść
btn-add = Dodaj
btn-remove = Usuń
btn-add-step = Nowy krok
btn-delete-step = Usuń krok
btn-add-group = Dodaj grupę
btn-remove-group = Usuń grupę
btn-add-plugin = Dodaj wtyczkę
btn-remove-plugin = Usuń wtyczkę
btn-add-file = Dodaj plik
btn-add-folder = Dodaj folder
btn-remove-file = Usuń
btn-add-flag = Dodaj flagę
btn-remove-flag = Usuń flagę
btn-add-condition = Dodaj warunek
btn-remove-condition = Usuń warunek
btn-add-dependency = Dodaj zależność
btn-remove-dependency = Usuń zależność
btn-add-pattern = Nowy wzorzec
btn-remove-pattern = Usuń wzorzec
btn-save = Zapisz
btn-cancel = Anuluj
btn-ok = OK
btn-yes = Tak
btn-no = Nie

# Condition/Dependency Labels
label-flag-name = Nazwa flagi:
label-flag-value = Wartość:
label-condition-type = Typ:
label-condition-name = Nazwa:
label-condition-value = Wartość:
label-dep-type = Typ zależności:
label-dep-name = Nazwa/plik:
label-dep-value = Wartość/stan:

# Files
label-source = Źródło
label-destination = Miejsce docelowe
label-priority = Priorytet
label-file-type = Typ
label-files = Pliki
label-dependencies = Zależności

# Settings Dialog
settings-title = Ustawienia
settings-tab-general = Ogólne
settings-tab-recent-files = Ostatnie pliki
settings-language = Język:
settings-theme = Motyw:
settings-font-size = Rozmiar czcionki:
settings-replace-newlines = Przetwarzaj znaki nowej linii w opisach
settings-max-recent = Maks. ostatnich plików:
settings-window-width = Szerokość okna:
settings-window-height = Wysokość okna:
settings-no-recent-files = Brak ostatnich plików.

# Status messages for settings
status-settings-saved = Pomyślnie zapisano ustawienia

# About Dialog
about-title = O programie XIMOD Architect
about-description = Wieloplatformowe narzędzie do tworzenia instalatorów FOMOD dla modów gier Bethesdy.
about-license = Na licencji MIT
about-copyright = © 2024 XIMOD Team
about-credit = Port oryginalnego narzędzia autorstwa Wenderer na język Rust:

# Script Dialog
script-title = Edytuj skrypt
script-info = Skrypty są uruchamiane przed zapisem lub po nim. Możesz użyć następujących makr:
script-macros = Dostępne makra:
macro-modname = $MODNAME$ - Nazwa moda
macro-modauthor = $MODAUTHOR$ - Nazwa autora
macro-modversion = $MODVERSION$ - Wersja moda
macro-modroot = $MODROOT$ - Ścieżka katalogu głównego
macro-date = $DATE$ - Bieżąca data (RRRR-MM-DD)
macro-time = $TIME$ - Bieżący czas (GG:MM:SS)
macro-random = $RANDOM$ - Liczba losowa

# Plugin Dependencies
label-default-type = Typ domyślny:
label-pattern-type = Typ wzorca:
label-pattern-operator = Operator wzorca:

# Conditional Files
label-pattern = Wzorzec

# Validation Messages
validation-no-name = Nazwa moda jest wymagana
validation-no-steps = Wymagany jest co najmniej jeden krok lub wymagany plik
validation-empty-step = Krok { $num } nie ma nazwy
validation-empty-group = Krok { $step }, grupa { $group } nie ma nazwy
validation-no-plugins = Krok { $step }, grupa „{ $name }” nie ma wtyczek

# File States
state-active = Aktywny
state-inactive = Nieaktywny
state-missing = Brak

# Confirmation
confirm-title = Potwierdzenie
confirm-delete = Czy na pewno chcesz usunąć ten element?
confirm-discard = Masz niezapisane zmiany. Odrzucić je i kontynuować?
confirm-unsaved = Masz niezapisane zmiany. Czy chcesz zapisać przed zamknięciem?
confirm-save-issues = Projekt ma następujące problemy:
confirm-save-anyway = Zapisać mimo to?

# Errors
error-invalid-xml = Nieprawidłowy plik XML
error-parse-failed = Nie udało się przeanalizować FOMOD
error-write-failed = Nie udało się zapisać pliku
error-create-dir = Nie udało się utworzyć katalogu

# Default names (generated when creating new items)
default-step-name = Krok { $num }
default-group-name = Grupa { $num }
default-plugin-name = Wtyczka { $num }
pattern-label = Wzorzec { $num }

# Selection prompts
msg-select-group-first = Najpierw wybierz grupę.
msg-select-plugin-edit = Wybierz wtyczkę do edycji.
label-empty = (puste)
image-no-image = Brak obrazu

# File dialog filters
filter-images = Obrazy
filter-xml = XML

# Dependency types
dep-type-flag = Flaga
dep-type-file = Plik

# Status bar
status-modified = Zmodyfikowano

# Status messages (errors)
msg-settings-save-error = Błąd podczas zapisywania ustawień
msg-script-save-error = Błąd podczas zapisywania skryptu

# Translation editor
trans-title = Edytor tłumaczeń
trans-source-lang = Wyświetlany język:
trans-target-lang = Język do przetłumaczenia:
trans-col-key = Klucz
trans-col-source = Etykieta
trans-col-target = Tłumaczenie
trans-saved = Zapisano tłumaczenie
trans-save-error = Błąd podczas zapisywania tłumaczenia

# XML editor
xml-editor-title = Edytor XML
xml-editor-edit = Edytuj
xml-editor-apply = Zastosuj
xml-editor-revert = Anuluj
xml-editor-readonly = Tylko do odczytu
xml-editor-editing = Edycja — karty graficzne są zablokowane
xml-editor-error = Błąd:
xml-editor-applied = Zastosowano zmiany XML
xml-editor-wellformed = Poprawnie sformułowany XML
xml-editor-error-at = Wiersz { $line }, kolumna { $col }: { $msg }

# Country / flag picker
settings-country-name = Nazwa kraju:
settings-pick-country = Kliknij, aby wybrać swój kraj
flags-title = Wybierz kraj
flags-filter = Filtr:
flags-none = Nie znaleziono flagi

# Translation editor: country & font
trans-endonym = Endonim kraju:
trans-font = Czcionka:
trans-no-font = (brak)
trans-browse = Przeglądaj…
trans-google-fonts = Google Fonts
trans-pick-country = Kliknij, aby wybrać kraj
trans-font-outside = Czcionka musi być najpierw zainstalowana w assets/fonts.
trans-font-dir-missing = Nie znaleziono folderu assets/fonts.

# Translation submission
trans-lang-endonym = Endonim języka:
trans-author = Autor:
trans-submit = Wyślij…
trans-submit-hint = Zbuduj archiwum .zip i otwórz wstępnie wypełnioną wiadomość e-mail
trans-data-updated = Zaktualizowano dane referencyjne (Languages.json / Countries.json)
trans-package-ready = Archiwum gotowe:
trans-package-error = Nie udało się zbudować archiwum:

# ISO 639-3 requirement
trans-lang-not-iso = Tłumaczenie jest możliwe tylko dla języka z kodem ISO 639-3.

# FOMOD installer preview
menu-preview = Podgląd instalatora…
preview-title = Podgląd instalatora FOMOD
preview-refresh = Odśwież
preview-assumptions = Założenia dotyczące plików
preview-details = Szczegóły
preview-back = Wstecz
preview-next = Dalej
preview-install = Instaluj
preview-close = Zamknij
preview-restart = Uruchom ponownie
preview-summary-title = Pliki, które zostaną zainstalowane
preview-empty = Żaden plik nie zostałby zainstalowany.
preview-none-option = (brak)
preview-invalid = Uzupełnij wymagane wybory, aby kontynuować.
preview-no-steps = Żaden krok nie jest widoczny; zobacz podsumowanie instalacji.
preview-select-hint = Wybierz opcję, aby zobaczyć jej opis.
preview-col-source = Źródło
preview-col-dest = Miejsce docelowe
preview-col-priority = Priorytet
preview-sel-exactlyone = Wybierz dokładnie jedną opcję.
preview-sel-atmostone = Wybierz co najwyżej jedną opcję.
preview-sel-any = Wybierz dowolną liczbę opcji.
preview-sel-all = Wszystkie opcje są instalowane.
preview-sel-atleastone = Wybierz co najmniej jedną opcję.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Sprawdź poprawność FOMOD
validate-report-title = Walidacja FOMOD
validate-ok = Nie znaleziono żadnych problemów. FOMOD jest zgodny ze schematem.
xml-editor-schema-ok = Zgodny ze schematem ModConfig 5.0.
xml-editor-schema-issues = Problemy ze schematem:
schema-line-col = Wiersz { $line }, kol. { $col }: { $msg }
schema-wrong-root = Nieoczekiwany element główny "{ $found }" (oczekiwano "{ $expected }").
schema-unknown = Nieoczekiwany element "{ $element }" w "{ $parent }".
schema-missing = "{ $parent }" musi zawierać "{ $child }".
schema-needs-one = "{ $parent }" musi zawierać co najmniej jeden "{ $child }".
schema-too-many = "{ $child }" może wystąpić tylko raz w "{ $parent }".
schema-missing-attr = Atrybut "{ $attr }" jest wymagany w "{ $element }".
schema-bad-enum = Nieprawidłowa wartość "{ $value }" dla { $element }/@{ $attr } (oczekiwano: { $allowed }).
schema-choose-one = "{ $parent }" musi zawierać dokładnie jeden z: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Przenieś przed
reorder-after = Przenieś za

# Country / language database explorer (Properties)
menu-properties = Właściwości…
prop-title = Baza danych krajów / języków
prop-tab-countries = Kraje
prop-tab-languages = Języki
prop-filter = Filtr:
prop-official-langs = Języki urzędowe
prop-spoken-langs = Języki używane
prop-endonym = Endonim kraju
prop-font = Czcionka
prop-spoken-in = Używany w
prop-select-country = Wybierz kraj, aby zobaczyć jego szczegóły.
prop-select-lang = Wybierz język, aby zobaczyć jego szczegóły.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Otwórz stronę gry w serwisie Nexus Mods
