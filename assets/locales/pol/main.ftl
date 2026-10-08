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
menu-export = Eksportuj archiwum dystrybucyjne…
# Menu - Options
menu-options = Opcje
menu-settings = Ustawienia…
menu-pre-save-script = Skrypt przed zapisem…
menu-post-save-script = Skrypt po zapisie…
menu-translation = Przetłumacz interfejs…
# Menu - Help
menu-help = Pomoc
menu-check-updates = Sprawdź aktualizacje…
menu-about = O programie

# Update check
update-checking = Sprawdzanie aktualizacji…
update-up-to-date = XIMOD Architect jest aktualny.
update-check-failed = Nie udało się sprawdzić aktualizacji. Spróbuj ponownie później.
update-available-status = Dostępna jest wersja { $version }.
update-banner-text = Dostępny jest XIMOD Architect { $version }.
update-download = Pobierz:
update-skip = Pomiń tę wersję
update-later = Później

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
label-plugin-name = Nazwa opcji:
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
btn-add-plugin = Dodaj opcję
btn-remove-plugin = Usuń opcję
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Miejsce docelowe dla całej grupy
label-page-dest = Miejsce docelowe instalacji (cała strona)
btn-apply-group-dest = Zastosuj do wszystkich opcji w tej grupie
btn-apply-page-dest = Zastosuj do wszystkich opcji na tej stronie
group-dest-hint = Ustawia jedno miejsce docelowe instalacji dla każdego pliku każdej opcji w tej grupie.
page-dest-hint = Ustawia jedno miejsce docelowe instalacji dla każdego pliku każdej opcji na tej stronie (wszystkie grupy).
bulk-dest-nofiles = Brak plików do zaktualizowania — najpierw dodaj pliki do opcji.
status-dest-applied = Zastosowano miejsce docelowe do { $num } plik(ów).
preview-hidden-steps = { $num } krok(ów) ukrytych przez bieżące wybory.
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
settings-check-updates = Sprawdzaj aktualizacje przy uruchomieniu
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
label-plugin-dependencies = Zależności opcji
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
validation-no-plugins = Krok { $step }, grupa „{ $name }” nie ma opcji

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
default-plugin-name = Opcja { $num }
pattern-label = Wzorzec { $num }

# Selection prompts
msg-select-group-first = Najpierw wybierz grupę.
msg-select-plugin-edit = Wybierz opcję do edycji.
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

# Referenced-file verification (V2)
verify-no-root = Pominięto sprawdzanie plików: nie ustawiono folderu głównego
loc-header = obraz nagłówka
loc-required = pliki wymagane
loc-conditional = zestaw warunkowy { $num }
loc-plugin = krok { $step }, grupa { $group }, opcja „{ $plugin }”
verify-missing-file = Brak pliku: { $path } ({ $loc })
verify-missing-folder = Brak folderu: { $path } ({ $loc })
verify-missing-image = Brak obrazu: { $path } ({ $loc })
verify-absolute = Ścieżka bezwzględna (nieprzenośna): { $path } ({ $loc })
verify-outside = Ścieżka wychodzi poza folder główny: { $path } ({ $loc })
verify-orphan = Plik osierocony (nieużywany przez żadną opcję): { $path }
conflict-certain = Konflikt miejsca docelowego: „{ $path }“ zapisuje { $count } opcji ({ $locs }) — nadpisują się nawzajem.
conflict-potential = Możliwy konflikt miejsca docelowego: „{ $path }“ jest celem { $count } odwołań ({ $locs }) — nadpisanie zależy od wyboru/warunków.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Zamknij FOMOD
menu-close-all-fomods = Zamknij wszystkie FOMOD-y
tab-untitled = (bez tytułu)
msg-drop-not-fomod = Upuszczony element nie jest FOMOD-em (nie znaleziono folderu „fomod”)
exit-title = Niezapisane zmiany
exit-unsaved = FOMOD nie został zapisany. Czy chcesz go zapisać?
tab-close-hint = Zamknij ten FOMOD
menu-new-from-folder = Nowy z folderu…
menu-templates = Szablony…
templates-title = Szablony wielokrotnego użytku
templates-empty = Brak zapisanych szablonów. Zapisz powyżej wybrany krok, aby utworzyć szablon.
templates-insert = Wstaw
templates-save-step = Zapisz wybrany krok
templates-name-hint = Nazwa szablonu (opcjonalnie)
msg-wizard-success = Szkielet utworzony z folderu: { $num } opcja(e).
msg-wizard-error = Błąd: { $error }
msg-template-saved = Szablon zapisany: { $name }
msg-template-inserted = Szablon wstawiony do projektu.
msg-template-no-step = Najpierw wybierz krok, aby zapisać go jako szablon.
msg-template-no-dir = Nie można znaleźć folderu szablonów.
msg-drop-assigned = Dodano { $added } źródło(a) do opcji ({ $rejected } poza katalogiem głównym zignorowano).
menu-compare = Porównaj z…
compare-title = Porównanie FOMOD
compare-none = Brak różnic.
btn-optimize-image = Optymalizuj obraz
msg-image-optimized = Obraz nagłówka zoptymalizowany.
msg-image-ok = Obraz nagłówka już mieści się w limitach.
msg-no-header-image = Brak obrazu nagłówka do optymalizacji.
verify-image-large = Obraz zbyt duży ({ $width }×{ $height }): { $path }
verify-image-format = Nieobsługiwany format obrazu (.{ $ext }): { $path }
verify-image-unreadable = Nieczytelny obraz: { $path }
menu-condition-editor = Edytor warunków…
condeditor-title = Edytor warunków
condeditor-set-by = Ustawiane przez:
condeditor-used-by = Używane przez:
condeditor-filedeps = Zależności plików
condeditor-empty = Brak flag lub zależności w tym projekcie.
condeditor-orphan-set = ustawiona, ale nigdy nieużywana
condeditor-orphan-used = używana, ale nigdy nieustawiona
msg-img-optimized = Obraz zoptymalizowany.
msg-img-ok = Obraz już mieści się w limitach.
msg-img-none = Brak obrazu do optymalizacji.
msg-crash-recovery = Poprzednia sesja zakończyła się nieoczekiwanie. Kopia zapasowa projektu została zapisana w { $path }
export-progress-title = Tworzenie archiwum dystrybucyjnego…
export-progress-files = { $done } / { $total } plików
msg-export-cancelled = Eksport anulowany; częściowe archiwum zostało usunięte.
verify-running = Sprawdzanie plików na dysku…
verify-stale = Uwaga: projekt zmienił się podczas sprawdzania plików; uruchom walidację ponownie.
prop-col-name = Nazwa
menu-save-as = Zapisz jako…
menu-project = Projekt
menu-tools = Narzędzia
menu-manual = Podręcznik użytkownika
msg-manual-missing = Nie znaleziono podręcznika użytkownika (PDF) obok aplikacji.
toolbar-new = Nowy
toolbar-open = Otwórz
toolbar-save = Zapisz
toolbar-validate = Sprawdź
toolbar-preview = Podgląd
toolbar-export = Eksportuj
dialog-choose-root = Wybierz folder główny moda
exit-unsaved-docs = Niezapisane: { $names }
status-summary = { $steps } kroków · { $options } opcji
section-groups = Grupy
section-options = Opcje
section-flags = Flagi warunków
section-files = Pliki do zainstalowania
hint-group-type = Jak instalator pozwala użytkownikowi wybierać opcje w tej grupie.
hint-default-type = Jak opcja jest oferowana, gdy żaden wzorzec zależności nie pasuje: wymagana, opcjonalna, zalecana, nieużyteczna…
hint-operator = Wszystkie warunki muszą być spełnione (I) lub dowolny z nich (LUB).
hint-flags = Flagi to nazwane wartości ustawiane przez tę opcję po jej wybraniu. Inne kroki i opcje mogą je sprawdzać, aby się pokazać, ukryć lub stać się wymaganymi.
hint-plugin-dependencies = Wzorce zmieniające typ opcji w zależności od flag lub plików obecnych w grze: na przykład „Wymagana”, gdy zainstalowany jest inny mod.
hint-files = Pliki i foldery kopiowane do folderu Data gry po wybraniu tej opcji. Miejsce docelowe jest względne wobec Data; przy konflikcie wygrywa wyższy priorytet.
hint-visibility = Warunki, które muszą być spełnione, aby ten krok w ogóle się pokazał. Zostaw puste, aby zawsze go pokazywać.
seltype-exactly-one = Dokładnie jedna (wymagana)
seltype-at-most-one = Co najwyżej jedna
seltype-any = Dowolna liczba
seltype-all = Wszystkie (bez wyboru)
seltype-at-least-one = Co najmniej jedna
plugtype-required = Wymagana
plugtype-optional = Opcjonalna
plugtype-recommended = Zalecana
plugtype-not-usable = Nieużyteczna
plugtype-could-be-usable = Możliwie użyteczna
plugtype-required-hint = Zawsze instalowana; użytkownik nie może jej odznaczyć.
plugtype-optional-hint = Oferowana bez zaznaczenia; użytkownik decyduje.
plugtype-recommended-hint = Oferowana zaznaczona; użytkownik może ją odznaczyć.
plugtype-not-usable-hint = Wyszarzona, nie można jej wybrać.
plugtype-could-be-usable-hint = Można ją wybrać, ale instalator ostrzega, że może nie działać.
op-and = Wszystkie warunki (I)
op-or = Dowolny warunek (LUB)
theme-dark = Ciemny
theme-light = Jasny
theme-system = Zgodnie z systemem
condeditor-setter-loc = Krok { "{step}" } / Grupa { "{group}" } / „{ "{name}" }”
condeditor-pattern-of = Wzorzec „{ "{name}" }” → { "{type}" }
condeditor-visibility-of = Widoczność kroku { "{step}" }
condeditor-cond-set = Zestaw warunkowy { "{num}" }
condeditor-needs = { "{ctx}" } (wymaga = { "{value}" })
condeditor-file-dep = { "{ctx}" }: plik „{ "{name}" }” ({ "{state}" })
menu-translate-fomod = Przetłumacz FOMOD…
ftr-title = Przetłumacz FOMOD
ftr-open-folder = Otwórz folder moda…
ftr-from-active = Z aktywnego projektu
ftr-from-active-hint = Tłumaczy FOMOD projektu otwartego w oknie głównym (najpierw trzeba go zapisać).
ftr-no-fomod = Nie wczytano żadnego FOMOD.
ftr-encoding = Kodowanie plików oryginalnych; przetłumaczone pliki są zapisywane w tym samym kodowaniu.
ftr-source-lang = Z
ftr-target-lang = na
ftr-lang-locked = (po wczytaniu FOMOD języków nie można zmienić)
ftr-translator = Tłumacz:
ftr-save = Zapisz tłumaczenie
ftr-export = Eksportuj przetłumaczone pliki
ftr-export-sibling = Do folderu fomod_<język>
ftr-export-sibling-hint = Zapisuje przetłumaczone pliki info.xml i ModuleConfig.xml obok oryginalnego folderu fomod; oryginalne pliki pozostają nietknięte.
ftr-export-inplace = W miejsce oryginalnych plików
ftr-export-inplace-hint = Zastępuje fomod/info.xml i fomod/ModuleConfig.xml po utworzeniu kopii .bak każdego z nich ze znacznikiem czasu.
ftr-force-explicit-order = Zachowaj oryginalną kolejność
ftr-warn-order = Listy sortowane według nazwy (order="Ascending") zostałyby w menedżerze modów posortowane ponownie według przetłumaczonych nazw. Ta opcja wymusza order="Explicit", aby opcje zachowały obecną kolejność.
ftr-update = Aktualizuj z folderu
ftr-update-hint = Ponownie wczytuje FOMOD z dysku i scala z nim tłumaczenie: nowe, zmienione i usunięte ciągi są zgłaszane.
ftr-preview-translated = Podgląd tłumaczenia
ftr-progress = Przetłumaczono { $done } / { $total }
ftr-filter-all = Wszystkie
ftr-filter-untranslated = Nieprzetłumaczone
ftr-filter-review = Do sprawdzenia
ftr-filter-issues = Z problemami
ftr-filter-locked = Zablokowane
ftr-type-all = Wszystkie pola
ftr-type-names = Nazwy
ftr-type-descriptions = Opisy
ftr-type-meta = Informacje o modzie
ftr-search-hint = Szukaj w źródle, tłumaczeniu lub kontekście…
ftr-next-untranslated = Następny nieprzetłumaczony
ftr-show-whitespace = Pokaż spacje i znaki końca wiersza
ftr-discard-question = Bieżące tłumaczenie zawiera niezapisane zmiany. Odrzucić je i wczytać drugi FOMOD?
ftr-discard-yes = Odrzuć
ftr-unsaved-close = Tłumaczenie zawiera niezapisane zmiany.
ftr-col-num = Nr
ftr-col-status = { "" }
ftr-col-context = Kontekst
ftr-col-source = Źródło
ftr-col-target = Tłumaczenie
ftr-col-issues = { "" }
ftr-empty-hint = Otwórz folder moda lub wczytaj aktywny projekt, aby wyświetlić jego ciągi do przetłumaczenia.
ftr-empty-filter = Żaden ciąg nie pasuje do bieżącego filtra.
ftr-select-row = Wybierz wiersz, aby edytować jego tłumaczenie.
ftr-copy-source = Kopiuj źródło
ftr-clear-target = Wyczyść
ftr-lock = Nie tłumacz
ftr-lock-hint = Zablokowane ciągi są zapisywane bez zmian (autor, witryna, nazwy własne…).
ftr-note = Notatka:
ftr-status-untranslated = Nieprzetłumaczony
ftr-status-translated = Przetłumaczony
ftr-status-auto = Wypełniony automatycznie — do sprawdzenia
ftr-status-fuzzy = Tekst źródłowy zmienił się od czasu tłumaczenia — do sprawdzenia
ftr-status-obsolete = Nie występuje już w FOMOD
ftr-status-locked = Zablokowany (zapisywany bez zmian)
ftr-field-info-name = Nazwa moda (info.xml)
ftr-field-module-name = Tytuł instalatora (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Witryna
ftr-field-description = Opis moda
ftr-field-step = Nazwa kroku
ftr-field-group = Nazwa grupy
ftr-field-plugin = Nazwa opcji
ftr-field-plugin-desc = Opis opcji
ftr-issue-empty = Puste tłumaczenie
ftr-issue-whitespace = Tłumaczenie zawiera tylko spacje
ftr-issue-edge-whitespace = Spacje na początku lub na końcu różnią się od źródła
ftr-issue-token = Chronione tokeny się różnią — brakujące: { $missing } ; nadmiarowe: { $extra }
ftr-issue-newline-name = Nazwa nie może zawierać znaku końca wiersza
ftr-issue-control = Zawiera znaki, których XML nie może przechować
ftr-issue-length = Nietypowa długość w porównaniu ze źródłem (×{ $ratio })
ftr-issue-identical = Identyczne ze źródłem
ftr-issue-duplicate = Ten sam tekst źródłowy przetłumaczono inaczej w { $key }
ftr-issue-cdata = Sekwencja ]]> nie jest tu dozwolona
ftr-load-error = Nie można wczytać FOMOD: { $error }
ftr-extracted = Znalezione ciągi do przetłumaczenia: { $num }.
ftr-sidecar-found = Wczytano i scalono istniejące tłumaczenie: nowe { $new }, zmienione { $changed }, usunięte { $removed }.
ftr-saved = Tłumaczenie zapisano w { $path }
ftr-save-error = Nie można zapisać tłumaczenia: { $error }
ftr-save-first = Najpierw zapisz projekt, a potem go przetłumacz.
ftr-export-success = Ciągi zapisane w { $path }: { $count }
ftr-export-error = Eksport nie powiódł się: { $error }
ftr-export-blocked = Problemy blokujące do naprawienia przed eksportem: { $num }.
ftr-export-stale = Ciągi pominięte z powodu zmiany FOMOD: { $num }; użyj „Aktualizuj z folderu”.
ftr-update-report = Zaktualizowano: nowe { $new }, zmienione { $changed }, przeniesione { $moved }, usunięte { $removed }, bez zmian { $unchanged }.
menu-edit = Edycja
menu-undo = Cofnij
menu-redo = Ponów
tree-title = Projekt
tree-mod-info = Informacje o modzie
tree-steps = Kroki instalacji
tree-required = Pliki wymagane
tree-conditional = Instalacje warunkowe
tree-empty-steps = Nie ma jeszcze żadnego kroku — kliknij +, aby go dodać.
tree-duplicate = Duplikuj
tree-delete = Usuń
tree-save-template = Zapisz jako szablon…
tree-drop-hint = Upuść tutaj, aby przenieść
cond-set-label = Zestaw warunkowy { $num }
inspector-empty = Wybierz element w drzewie projektu albo dodaj krok, aby zacząć.
count-options = Opcje: { $num }
count-files = Pliki: { $num }
msg-deleted-undo = Usunięto. Użyj polecenia Cofnij (Ctrl+Z), aby przywrócić.
problems-title = Problemy
problems-errors = Błędy: { $num }
problems-warnings = Ostrzeżenia: { $num }
btn-close = Zamknij
ftr-export-package = Jako pakiet tłumaczenia (archiwum)
ftr-export-package-hint = Tworzy plik .zip lub .7z gotowy do przesłania: przetłumaczone pliki info.xml i ModuleConfig.xml oraz README (sama łatka) albo cały mod z przetłumaczonymi plikami (pełny).
ftr-package-full = Cały mod
ftr-package-full-hint = Dołącz do archiwum wszystkie pliki moda, a nie tylko dwa przetłumaczone pliki XML. Upewnij się, że autor zezwala na redystrybucję.
ftr-package-name-template = Nazwa:
ftr-readme-patch = To archiwum zawiera tłumaczenie instalatora moda „{ $name }” (język: { $langname }; pliki fomod/info.xml i fomod/ModuleConfig.xml). Zainstaluj je na oryginalnym modzie albo pozwól menedżerowi modów je scalić, aby przetłumaczone pliki zastąpiły oryginalne. Zmieniają się tylko teksty instalatora; same pliki moda nie są dołączone. Utworzono w programie XIMOD Architect.
ftr-readme-full = To archiwum zawiera mod „{ $name }” z przetłumaczonym instalatorem (język: { $langname }; pliki fomod/info.xml i fomod/ModuleConfig.xml). Zainstaluj je tak jak oryginalny mod. Zmieniono tylko teksty instalatora. Utworzono w programie XIMOD Architect.
ftr-apply-memory = Wypełnij z pamięci
ftr-memory-size = Pamięć tłumaczeń — wpisy dla tej pary języków: { $num }. Trafia do niej każde zapisane tłumaczenie.
ftr-memory-applied = Ciągi wypełnione z pamięci tłumaczeń (oznaczone „do sprawdzenia”): { $num }.
ftr-memory-suggestion = Pamięć podpowiada:
ftr-use-suggestion = Użyj
ftr-propagate = Zastosuj do identycznych
ftr-propagate-hint = Kopiuje to tłumaczenie do wszystkich pozostałych, jeszcze nieprzetłumaczonych ciągów o tym samym tekście źródłowym.
ftr-propagated = Wypełnione identyczne ciągi: { $num }.
ftr-csv-export = Eksportuj CSV…
ftr-csv-import = Importuj CSV…
ftr-csv-imported = Ciągi zaktualizowane z pliku CSV: { $num }.
ftr-csv-error = Błąd CSV: { $error }
ftr-glossary = Glosariusz
ftr-glossary-source = Termin
ftr-glossary-target = Tłumaczenie
ftr-glossary-case = Wielkość liter
ftr-glossary-dnt = Zachowaj
ftr-glossary-add = Dodaj termin
ftr-issue-glossary = Glosariusz: „{ $term }” nie został przetłumaczony zgodnie z oczekiwaniami

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Otwórz archiwum…
filter-archive = Archiwa modów (zip, 7z)
msg-archive-opened = Archiwum otwarte (wyodrębnionych plików: { $num }): { $path }
msg-archive-reused = Archiwum zostało już wyodrębnione, ponowne użycie { $path }
msg-archive-unsupported = Format archiwum „.{ $ext }” nie jest obsługiwany; najpierw wyodrębnij je programem 7-Zip (można otwierać tylko .zip i .7z).
msg-archive-error = Błąd podczas otwierania archiwum: { $error }
msg-archive-no-fomod = Nie znaleziono folderu „fomod” w archiwum ({ $path })
msg-archive-extracting = Wyodrębnianie archiwum…
ftr-open-archive = Otwórz archiwum moda…
ftr-package-full-partial = Mod został otwarty z archiwum zawierającego tylko jego folder fomod; pełne pakiety wymagają wyodrębnionego moda.
info-module-deps = Wymagania moda
info-module-deps-hint = Pliki lub flagi wymagane przez cały mod przed uruchomieniem instalatora (moduleDependencies). Pozostaw puste, jeśli nie ma żadnych.
info-header-advanced = Nagłówek zaawansowany
info-title-position = Położenie tytułu
info-title-colour = Kolor tytułu
info-title-colour-hint = Oczekiwane: sześć cyfr szesnastkowych (RRGGBB)
info-image-show = Pokaż obraz nagłówka
info-image-fade = Wygaszaj obraz nagłówka
info-image-height = Wysokość obrazu nagłówka
info-attr-default = (domyślnie)
file-always-install = Zawsze
file-always-install-hint = Zawsze instaluj ten plik, nawet gdy opcja nie jest zaznaczona (alwaysInstall).
file-install-if-usable = Jeśli można
file-install-if-usable-hint = Instaluj ten plik zawsze, gdy opcja jest użyteczna, nawet jeśli nie jest zaznaczona (installIfUsable).
msg-import-lossy = Ten FOMOD zawiera konstrukcje, których XIMOD nie może edytować (liczba: { $num }); zostaną one odrzucone przy zapisie projektu.
fidelity-nested-deps = Zagnieżdżona grupa zależności, lokalizacja: { $context } (obsługiwany jest tylko jeden poziom)
fidelity-game-dep = Wymaganie wersji gry { $version }, lokalizacja: { $context }
fidelity-fomm-dep = Wymaganie wersji menedżera modów { $version }, lokalizacja: { $context }
fidelity-unknown = Element „{ $element }” w „{ $parent }” nie jest obsługiwany ({ $context })
loc-module = wymagania moda
loc-step = krok { $step } „{ $name }”
loc-installer = instalator

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Przywróć kopię zapasową…
backups-title = Przywróć kopię zapasową
backups-empty = Ten projekt nie ma jeszcze kopii zapasowej. Kopia jest tworzona przy każdym zapisie projektu nadpisującym poprzednią wersję.
backups-changes = Liczba zmian względem bieżącego projektu: { $num }
btn-compare = Porównaj
btn-restore = Przywróć
btn-delete-backups = Usuń wszystkie kopie zapasowe
btn-delete-backups-confirm = Kliknij ponownie, aby usunąć wszystkie kopie zapasowe
msg-backup-restored = Kopia zapasowa z { $time } przywrócona do edytora (jeszcze nie zapisano; Cofnij ją wycofa)
msg-backups-deleted = Usuniętych kopii zapasowych: { $num }
settings-backup-count = Liczba przechowywanych kopii zapasowych:
settings-backup-count-hint = Liczba poprzednich wersji plików XML FOMOD przechowywanych w fomod/backups przy zapisie (0 = brak kopii zapasowych).
settings-autosave-minutes = Automatycznie zapisuj kopię odzyskiwania co (minuty):
settings-autosave-minutes-hint = W tym odstępie czasu w folderze konfiguracji zapisywana jest kopia odzyskiwania każdego zmodyfikowanego projektu; przy następnym uruchomieniu jest proponowana tylko po nieprawidłowym zamknięciu (0 = wyłączone).
settings-auto-masters = Dodawaj pliki master wtyczki jako warunki
settings-auto-masters-hint = Gdy wtyczka (.esp/.esm/.esl) zostaje dodana do opcji, wymagane przez nią pliki master, których nie dostarcza ani gra, ani ten mod, stają się warunkami plikowymi „Active” tej opcji.
msg-author-from-plugin = Autor uzupełniony z nagłówka wtyczki: { $author }
msg-masters-added = Liczba plików master wtyczki { $plugin } dodanych jako warunki plikowe: { $num }
issue-missing-master = { $plugin } wymaga { $master }, którego nie ma w tym modzie ani nie zadeklarowano jako zależności
issue-esl-mismatch-flag = { $plugin } ma rozszerzenie .esl, ale jego flaga light (ESL) nie jest ustawiona
issue-esl-eligible = { $plugin } można by oznaczyć jako light (nowych rekordów: { $num }, limit { $limit })
issue-esl-too-big = { $plugin } jest oznaczony jako light, ale nie spełnia zasad dla wtyczek light (nowych rekordów: { $num }, limit { $limit }, lub FormID poza dozwolonym zakresem)
menu-plugin-report = Raport wtyczek…
plugins-title = Raport wtyczek
plugins-file = Plik
plugins-kind = Rodzaj
plugins-light = Flaga light
plugins-masters = Pliki master
plugins-new-records = Nowe rekordy / limit
plugins-eligible = Może być light
plugins-empty = Ten projekt nie instaluje żadnego pliku wtyczki (.esp, .esm ani .esl).
plugins-unreadable = nieczytelny

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Końcowe drzewo plików
preview-total-size = Łączny rozmiar instalacji: { $size }
preview-tree-truncated = Drzewo jest skrócone: zbyt wiele plików do rozwinięcia (powyższe rozmiary są częściowe).
preview-overwritten-by = Nadpisane przez { $plugin }
preview-scenario = Scenariusz:
preview-scenario-load = Wczytaj
preview-scenario-save = Zapisz…
preview-scenario-delete = Usuń
preview-scenario-name = Nazwa scenariusza
preview-scenario-saved = Scenariusz „{ $name }” zapisano w fomod/scenarios
preview-scenario-unresolved = Wyborów scenariusza niepasujących do żadnej opcji tego projektu (zmieniona nazwa lub usunięta): { $num }
preview-scenario-none = (brak scenariusza)
issue-unreachable-step = Krok „{ $step }” nigdy nie może zostać wyświetlony: jego warunki widoczności sprawdzają wartość flagi, której nie ustawia żadna wcześniejsza opcja
issue-unreachable-option = Opcja „{ $plugin }” nigdy nie może zostać wybrana: jej wzorce typu użytecznego sprawdzają wartość flagi, której nie ustawia żadna opcja
issue-unreachable-cond = Warunkowy zestaw plików { $num } nigdy nie może mieć zastosowania: jego warunki sprawdzają wartość flagi, której nie ustawia żadna opcja
size-option = Rozmiar instalacji: { $size } (plików: { $num })
size-missing = Brakujących źródeł: { $num }
size-unknown = Rozmiar instalacji: — (uruchom Sprawdź poprawność, aby zmierzyć)
menu-nexus-desc = Opis dla Nexus…
nexus-title = Opis dla Nexus Mods
nexus-format = Format:
nexus-include-requirements = Wymagania
nexus-include-options = Opcje instalacji
nexus-include-install = Instalacja
nexus-include-changelog = Lista zmian
nexus-previous = Poprzednia wersja…
nexus-previous-none = (brak poprzedniej wersji: brak listy zmian)
nexus-language = Język:
nexus-language-source = (źródło)
nexus-sec-requirements = Wymagania
nexus-sec-options = Opcje instalacji
nexus-sec-install = Instalacja
nexus-sec-changelog = Lista zmian
nexus-install-text = Ten mod zawiera instalator FOMOD: zainstaluj go menedżerem modów (Vortex, Mod Organizer 2) i wybierz opcje w instalatorze.
nexus-requires = Wymaga
nexus-step = Krok
nexus-added = Dodano
nexus-removed = Usunięto
nexus-changed = Zmieniono
btn-copy = Kopiuj
btn-save-as = Zapisz jako…
msg-copied = Skopiowano do schowka
msg-saved-to = Zapisano w { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Zmień nazwę…
condeditor-rename-exists = Flaga o nazwie „{ $name }” już istnieje
condeditor-renamed = Nazwa flagi „{ $from }” zmieniona na „{ $to }” (wystąpień: { $num })
condeditor-delete-uses = Usuń wszystkie użycia
condeditor-deleted-uses = Flaga „{ $name }” usunięta wszędzie (wystąpień: { $num })
condeditor-values-set = Ustawiane wartości:
condeditor-values-tested = Sprawdzane wartości:
condeditor-value-never-set = { $value } — sprawdzana, ale nigdy nieustawiana
condeditor-value-never-tested = { $value } — ustawiana, ale nigdy niesprawdzana
condeditor-builder = Kreator warunków
condeditor-builder-none = Wybierz w oknie głównym krok, opcję, warunkowy zestaw plików lub informacje o modzie, aby edytować tutaj ich warunki.
condeditor-builder-pattern = Wzorzec:
condeditor-sentence-if = JEŻELI
condeditor-sentence-and = I
condeditor-sentence-or = LUB
condeditor-sentence-flag = flaga { "{name}" } = { "{value}" }
condeditor-sentence-file = plik { "{name}" } jest { "{value}" }
condeditor-sentence-empty = (brak warunku: zawsze prawda)
condeditor-sentence-then-visible = WTEDY krok jest wyświetlany
condeditor-sentence-then-type = WTEDY opcja staje się { $type }
condeditor-sentence-then-install = WTEDY pliki są instalowane
condeditor-sentence-then-module = WTEDY instalator może zostać uruchomiony (sprawdzane przed jego startem)
issue-flag-value-never-set = Flaga „{ $flag }” jest sprawdzana z wartością „{ $value }”, której nie ustawia żadna opcja
issue-flag-never-used = Flaga „{ $flag }” jest ustawiana, ale nigdzie nie jest sprawdzana
menu-project-strings = Teksty projektu…
strings-title = Teksty projektu
strings-search = Szukaj tekstu, lokalizacji lub klucza…
strings-kind-all = Wszystkie
strings-kind-names = Nazwy
strings-kind-descriptions = Opisy
strings-duplicates-only = Tylko duplikaty
strings-replace-with = Zamień na:
strings-case = Uwzględnij wielkość liter
strings-whole-word = Całe słowo
strings-replace-current = Zamień
strings-replace-all = Zamień wszystko
strings-replaced = Zamienionych tekstów: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Ten sam tekst co:
strings-count = Tekstów: { $num } · grup duplikatów: { $dups }
strings-col-location = Lokalizacja
strings-col-field = Pole
strings-col-text = Tekst

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Zawartość archiwum…
filter-bethesda-archive = Archiwa Bethesda (bsa, ba2)
archive-view-title = Zawartość archiwum
archive-view-format = Format:
archive-view-entries = Wpisy: { $num }
archive-view-size = { $size } po rozpakowaniu
archive-view-search = Szukaj ścieżki…
archive-view-col-path = Ścieżka
archive-view-col-size = Rozmiar
archive-view-col-compressed = Skompresowany
archive-view-truncated = Wyświetlanych jest tylko pierwszych { $num } pasujących wpisów — zawęź wyszukiwanie.
archive-view-error = Nie można odczytać tego archiwum: { $error }
archive-view-hint = Wyświetl zawartość tego archiwum
issue-conflict-archive = Ten sam zasób w kilku archiwach: „{ $path }“ jest pakowany przez { $count } odwołań ({ $locs }) — o tym, który zostanie użyty, decyduje kolejność wczytywania archiwów w grze.
issue-conflict-archive-loose = Archiwum kontra luźny plik: „{ $path }“ jest zarówno spakowany w archiwum, jak i zainstalowany jako luźny plik ({ $locs }) — luźny plik ma pierwszeństwo przed tym z archiwum.
preview-in-archive = (w archiwum)
preview-archived-size = w tym { $size } spakowane w archiwach

# --- Project tree: expand / collapse menus
tree-expand = Rozwiń
tree-collapse = Zwiń
tree-expand-all = Rozwiń wszystko
tree-expand-selected = Rozwiń zaznaczone
tree-expand-from = Rozwiń od zaznaczonego
tree-collapse-all = Zwiń wszystko
tree-collapse-selected = Zwiń zaznaczone
tree-collapse-from = Zwiń od zaznaczonego
tree-expand-all-hint = Rozwija wszystkie nagłówki
tree-expand-selected-hint = Rozwija tylko zaznaczony nagłówek
tree-expand-from-hint = Rozwija zaznaczony nagłówek i wszystko pod nim
tree-collapse-all-hint = Zwija wszystkie nagłówki
tree-collapse-selected-hint = Zwija tylko zaznaczony nagłówek
tree-collapse-from-hint = Zwija zaznaczony nagłówek i wszystko pod nim

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Dodaj grupę
btn-remove-group-cond = Usuń grupę
dep-type-game = Wersja gry
dep-type-fomm = Wersja menedżera modów
dep-group-hint = Grupa warunków połączonych operatorem I / LUB; grupy można zagnieżdżać.
condeditor-sentence-game = wersja gry ≥ { "{value}" }
condeditor-sentence-fomm = wersja menedżera modów ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unikatowe teksty
ftr-uniques-hint = Pokazuje jeden wiersz dla każdego odrębnego tekstu źródłowego. Przetłumaczenie tego wiersza tłumaczy naraz wszystkie ciągi o tym samym tekście.
ftr-uniques-synced = Zaktualizowane identyczne ciągi: { $num }.
ftr-uniques-group = Ciągi o tym samym tekście: { $num }; jego tłumaczenie dotyczy ich wszystkich.
