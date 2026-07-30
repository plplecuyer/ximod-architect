# XIMOD Architect - translation metadata
# @language = bul
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Български
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Версия { $version }

# Status messages
status-ready = Готов
msg-save-success = FOMOD е запазен успешно
msg-save-error = Грешка при запазването на FOMOD
msg-export-success = Създаден е архив за разпространение ({ $count } файла): { $path }
msg-export-error = Грешка при създаването на архива за разпространение: { $error }
msg-load-success = FOMOD е зареден успешно
msg-load-error = Грешка при зареждането на FOMOD
msg-merge-success = FOMOD беше обединен успешно
msg-merge-error = Грешка при обединяването на FOMOD
msg-no-root-selected = Моля, първо изберете коренова директория
msg-no-fomod-folder = Не беше намерена папка „fomod“. Да се създаде такава?
msg-file-outside-root = Файлът е извън кореновата директория

# Menu - File
menu-file = Файл
menu-new = Нов
menu-open = Отвори папка...
menu-open-file = Отвори файл...
menu-save = Запази
menu-recent = Последни
menu-exit = Изход
menu-merge = Обедини FOMOD...
menu-export = Експортирай архив на дистрибуцията...

# Menu - Options
menu-options = Опции
menu-settings = Настройки
menu-pre-save-script = Скрипт преди запазване...
menu-post-save-script = Скрипт след запазване...
menu-translation = Превод...

# Menu - Help
menu-help = Помощ
menu-about = За програмата

# Tabs
tab-info = Информация за мода
tab-steps = Стъпки за инсталиране
tab-required = Задължителни инсталации
tab-conditional = Условни инсталации

# Info Tab
label-workspace = Работна среда
label-root-dir = Коренна директория:
label-mod-name = Име на мода:
label-author = Автор:
label-version = Версия:
label-game-name = Име на играта:
label-category = Категория:
label-url = URL на уебсайта:
label-header-image = Изображение в заглавната част:
label-description = Описание:
placeholder-select-dir = (Изберете директория)
placeholder-select-game = (Изберете игра)

# Steps Tab
label-step-name = Име на стъпката:
label-group-name = Име на групата:
label-group-type = Тип на групата:
label-plugin-name = Име на плъгина:
label-plugin-desc = Описание:
label-plugin-type = Тип по подразбиране:
label-plugin-image = Изображение:
label-visibility = Условия за видимост
label-operator = Оператор:

# Buttons
btn-browse = Преглед...
btn-clear = Изчисти
btn-add = Добави
btn-remove = Премахни
btn-add-step = Нова стъпка
btn-delete-step = Изтрий стъпка
btn-add-group = Добави група
btn-remove-group = Премахни група
btn-add-plugin = Добави плъгин
btn-remove-plugin = Премахни плъгин
btn-add-file = Добави файл
btn-add-folder = Добави папка
btn-remove-file = Премахни
btn-add-flag = Добави маркер
btn-remove-flag = Премахване на маркер
btn-add-condition = Добавяне на условие
btn-remove-condition = Премахване на условие
btn-add-dependency = Добавяне на зависимост
btn-remove-dependency = Премахване на зависимост
btn-add-pattern = Нов шаблон
btn-remove-pattern = Изтриване на шаблон
btn-save = Запази
btn-cancel = Отказ
btn-ok = OK
btn-yes = Да
btn-no = Не

# Condition/Dependency Labels
label-flag-name = Име на флаг:
label-flag-value = Стойност:
label-condition-type = Тип:
label-condition-name = Име:
label-condition-value = Стойност:
label-dep-type = Тип на зависимостта:
label-dep-name = Име/Файл:
label-dep-value = Стойност/Състояние:

# Files
label-source = Източник
label-destination = Дестинация
label-priority = Приоритет
label-file-type = Тип
label-files = Файлове
label-dependencies = Зависимости

# Settings Dialog
settings-title = Настройки
settings-tab-general = Общи
settings-tab-recent-files = Последни файлове
settings-language = Език:
settings-theme = Тема:
settings-font-size = Размер на шрифта:
settings-replace-newlines = Обработка на символите за нов ред в описанията
settings-max-recent = Максимален брой скорошни файлове:
settings-window-width = Ширина на прозореца:
settings-window-height = Височина на прозореца:
settings-no-recent-files = Няма скорошни файлове.

# Status messages for settings
status-settings-saved = Настройките бяха запазени успешно

# About Dialog
about-title = За XIMOD Architect
about-description = Мултиплатформен инструмент за създаване на FOMOD инсталатори за модификации на игри на Bethesda.
about-license = Лицензиран под лиценза MIT
about-copyright = © 2025-2026 Екипът на XIMOD
about-credit = Rust порт на оригиналния инструмент от Wenderer:

# Script Dialog
script-title = Редактиране на скрипт
script-info = Скриптовете се изпълняват преди или след запазването. Можете да използвате следните макроси:
script-macros = Налични макроси:
macro-modname = $MODNAME$ – Име на мода
macro-modauthor = $MODAUTHOR$ – Име на автора
macro-modversion = $MODVERSION$ – Версия на мода
macro-modroot = $MODROOT$ – Път към кореновата директория
macro-date = $DATE$ – Текуща дата (ГГГГ-ММ-ДД)
macro-time = $TIME$ – Текущо време (ЧЧ:ММ:СС)
macro-random = $RANDOM$ – Случайно число

# Plugin Dependencies
label-default-type = Тип по подразбиране:
label-pattern-type = Тип на шаблона:
label-pattern-operator = Оператор на шаблона:

# Conditional Files
label-pattern = Шаблон

# Validation Messages
validation-no-name = Името на модула е задължително
validation-no-steps = Необходима е поне една стъпка или задължителен файл
validation-empty-step = Стъпка { $num } няма име
validation-empty-group = Стъпка { $step }, група { $group } няма име
validation-no-plugins = Стъпка { $step }, група "{ $name }" няма плъгини

# File States
state-active = Активно
state-inactive = Неактивно
state-missing = Липсва

# Confirmation
confirm-title = Потвърждение
confirm-delete = Сигурен ли сте, че искате да изтриете този елемент?
confirm-discard = Имате незапазени промени. Да ги отхвърлите и да продължите?
confirm-unsaved = Имате незапазени промени. Искате ли да ги запазите преди затваряне?
confirm-save-issues = Проектът има следните проблеми:
confirm-save-anyway = Да запазите въпреки това?

# Errors
error-invalid-xml = Невалиден XML файл
error-parse-failed = Неуспешен анализ на FOMOD
error-write-failed = Неуспешно записване на файл
error-create-dir = Неуспешно създаване на директория

# Default names (generated when creating new items)
default-step-name = Стъпка { $num }
default-group-name = Група { $num }
default-plugin-name = Плъгин { $num }
pattern-label = Шаблон { $num }

# Selection prompts
msg-select-group-first = Първо изберете група.
msg-select-plugin-edit = Изберете плъгин за редактиране.
label-empty = (празно)
image-no-image = Няма изображение

# File dialog filters
filter-images = Изображения
filter-xml = XML

# Dependency types
dep-type-flag = Флаг
dep-type-file = Файл

# Status bar
status-modified = Променено

# Status messages (errors)
msg-settings-save-error = Грешка при запазване на настройките
msg-script-save-error = Грешка при запазване на скрипта

# Translation editor
trans-title = Редактор за превод
trans-source-lang = Показан език:
trans-target-lang = Език за превод:
trans-col-key = Ключ
trans-col-source = Етикет
trans-col-target = Превод
trans-saved = Преводът е запазен
trans-save-error = Грешка при запазване на превода

# XML editor
xml-editor-title = XML редактор
xml-editor-edit = Редактиране
xml-editor-apply = Прилагане
xml-editor-revert = Отказ
xml-editor-readonly = Само за четене
xml-editor-editing = Редактиране — графичните раздели са заключени
xml-editor-error = Грешка:
xml-editor-applied = Промените в XML са приложени
xml-editor-wellformed = Правилно оформен XML
xml-editor-error-at = Ред { $line }, колона { $col }: { $msg }

# Country / flag picker
settings-country-name = Име на държавата:
settings-pick-country = Кликнете, за да изберете държавата си
flags-title = Изберете държава
flags-filter = Филтър:
flags-none = Не е намерено знаме

# Translation editor: country & font
trans-endonym = Ендоним на държавата:
trans-font = Шрифт:
trans-no-font = (няма)
trans-browse = Преглед…
trans-google-fonts = Google Fonts
trans-pick-country = Кликнете, за да изберете държавата
trans-font-outside = Шрифтът трябва първо да бъде инсталиран в папката assets/fonts.
trans-font-dir-missing = Папката assets/fonts не беше намерена.

# Translation submission
trans-lang-endonym = Ендином на езика:
trans-author = Автор:
trans-submit = Изпрати…
trans-submit-hint = Създайте ZIP файл и отворете предварително попълнен имейл
trans-data-updated = Референтните данни са актуализирани (Languages.json / Countries.json)
trans-package-ready = Архивът е готов:
trans-package-error = Не можа да се създаде архивът:

# ISO 639-3 requirement
trans-lang-not-iso = Преводът е възможен само за език с код по ISO 639-3.

# FOMOD installer preview
menu-preview = Предварителен преглед на инсталатора…
preview-title = Предварителен преглед на инсталатора на FOMOD
preview-refresh = Опресни
preview-assumptions = Предположения за файловете
preview-details = Подробности
preview-back = Назад
preview-next = Напред
preview-install = Инсталирай
preview-close = Затвори
preview-restart = Рестартирай
preview-summary-title = Файлове, които ще бъдат инсталирани
preview-empty = Няма да бъде инсталиран нито един файл.
preview-none-option = (няма)
preview-invalid = Попълнете задължителните полета, за да продължите.
preview-no-steps = Няма видими стъпки; вижте обобщението на инсталацията.
preview-select-hint = Изберете опция, за да видите нейното описание.
preview-col-source = Източник
preview-col-dest = Дестинация
preview-col-priority = Приоритет
preview-sel-exactlyone = Изберете точно една опция.
preview-sel-atmostone = Изберете най-много една опция.
preview-sel-any = Изберете произволен брой опции.
preview-sel-all = Всички опции са инсталирани.
preview-sel-atleastone = Изберете поне една опция.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Валидиране на FOMOD
validate-report-title = Валидиране на FOMOD
validate-ok = Не са открити проблеми. FOMOD отговаря на схемата.
xml-editor-schema-ok = Отговаря на схемата ModConfig 5.0.
xml-editor-schema-issues = Проблеми със схемата:
schema-line-col = Ред { $line }, колона { $col }: { $msg }
schema-wrong-root = Неочакван корен "{ $found }" (очакваше се "{ $expected }").
schema-unknown = Неочакван елемент „{ $element }“ в „{ $parent }“.
schema-missing = „{ $parent }“ трябва да съдържа „{ $child }“.
schema-needs-one = „{ $parent }“ трябва да съдържа поне един „{ $child }“.
schema-too-many = „{ $child }“ може да се появи само веднъж в „{ $parent }“.
schema-missing-attr = Атрибутът „{ $attr }“ е задължителен за „{ $element }“.
schema-bad-enum = Невалидна стойност „{ $value }“ за { $element }/@{ $attr } (очаквано: { $allowed }).
schema-choose-one = „{ $parent }“ трябва да съдържа точно едно от: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Премести преди
reorder-after = Премести след

# Country / language database explorer (Properties)
menu-properties = Свойства…
prop-title = База данни за държави/езици
prop-tab-countries = Държави
prop-tab-languages = Езици
prop-filter = Филтър:
prop-official-langs = Официални езици
prop-spoken-langs = Говорени езици
prop-endonym = Ендином на държавата
prop-font = Шрифт
prop-spoken-in = Говори се в
prop-select-country = Изберете държава, за да видите подробностите за нея.
prop-select-lang = Изберете език, за да видите подробностите за него.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Отвори страницата на играта в Nexus Mods
