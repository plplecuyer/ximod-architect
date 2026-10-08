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
menu-open = Отвори папка…
menu-open-file = Отвори файл…
menu-save = Запази
menu-recent = Последни
menu-exit = Изход
menu-merge = Обедини FOMOD…
menu-export = Експортирай архив на дистрибуцията…
# Menu - Options
menu-options = Опции
menu-settings = Настройки…
menu-pre-save-script = Скрипт преди запазване…
menu-post-save-script = Скрипт след запазване…
menu-translation = Преведи интерфейса…
# Menu - Help
menu-help = Помощ
menu-check-updates = Проверка за обновления…
menu-about = За програмата

# Update check
update-checking = Проверка за обновления…
update-up-to-date = XIMOD Architect е актуален.
update-check-failed = Проверката за обновления не бе възможна. Опитайте по-късно.
update-available-status = Налична е версия { $version }.
update-banner-text = XIMOD Architect { $version } е наличен.
update-download = Изтегляне:
update-skip = Пропусни тази версия
update-later = По-късно

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
label-plugin-name = Име на опцията:
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
btn-add-plugin = Добави опция
btn-remove-plugin = Премахни опция
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Местоназначение за цялата група
label-page-dest = Местоназначение за инсталиране (цялата страница)
btn-apply-group-dest = Приложи към всички опции в тази група
btn-apply-page-dest = Приложи към всички опции на тази страница
group-dest-hint = Задава едно местоназначение за инсталиране за всеки файл на всяка опция в тази група.
page-dest-hint = Задава едно местоназначение за инсталиране за всеки файл на всяка опция на тази страница (всички групи).
bulk-dest-nofiles = Все още няма файлове за актуализиране — първо добавете файлове към опциите.
status-dest-applied = Местоназначението е приложено към { $num } файл(а).
preview-hidden-steps = { $num } стъпка(и), скрити от текущите избори.
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
settings-check-updates = Проверка за обновления при стартиране
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
label-plugin-dependencies = Зависимости на опцията
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
validation-no-plugins = Стъпка { $step }, група "{ $name }" няма опции

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
default-plugin-name = Опция { $num }
pattern-label = Шаблон { $num }

# Selection prompts
msg-select-group-first = Първо изберете група.
msg-select-plugin-edit = Изберете опция за редактиране.
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

# Referenced-file verification (V2)
verify-no-root = Проверката на файловете е пропусната: не е зададена коренна папка
loc-header = заглавно изображение
loc-required = задължителни файлове
loc-conditional = условен набор { $num }
loc-plugin = стъпка { $step }, група { $group }, опция „{ $plugin }“
verify-missing-file = Липсващ файл: { $path } ({ $loc })
verify-missing-folder = Липсваща папка: { $path } ({ $loc })
verify-missing-image = Липсващо изображение: { $path } ({ $loc })
verify-absolute = Абсолютен път (не е преносим): { $path } ({ $loc })
verify-outside = Пътят излиза извън коренната папка: { $path } ({ $loc })
verify-orphan = Осиротял файл (не се използва от никоя опция): { $path }
conflict-certain = Конфликт на местоназначение: „{ $path }“ се записва от { $count } опции ({ $locs }) — те се презаписват взаимно.
conflict-potential = Възможен конфликт на местоназначение: „{ $path }“ е цел на { $count } препратки ({ $locs }) — презаписването зависи от избора/условията.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Затваряне на FOMOD
menu-close-all-fomods = Затваряне на всички FOMOD
tab-untitled = (без име)
msg-drop-not-fomod = Пуснатият елемент не е FOMOD (не е открита папка „fomod“)
exit-title = Незапазени промени
exit-unsaved = Един FOMOD не е запазен. Искате ли да го запазите?
tab-close-hint = Затваряне на този FOMOD
menu-new-from-folder = Нов от папка…
menu-templates = Шаблони…
templates-title = Шаблони за многократна употреба
templates-empty = Все още няма запазени шаблони. Запазете избраната стъпка по-горе, за да създадете шаблон.
templates-insert = Вмъкване
templates-save-step = Запазване на избраната стъпка
templates-name-hint = Име на шаблона (незадължително)
msg-wizard-success = Скелет, създаден от папка: { $num } опция(и).
msg-wizard-error = Грешка: { $error }
msg-template-saved = Шаблонът е запазен: { $name }
msg-template-inserted = Шаблонът е вмъкнат в проекта.
msg-template-no-step = Първо изберете стъпка, за да я запазите като шаблон.
msg-template-no-dir = Папката с шаблони не е намерена.
msg-drop-assigned = Към опцията са добавени { $added } източник(а) ({ $rejected } извън корена са пренебрегнати).
menu-compare = Сравни с…
compare-title = Сравнение на FOMOD
compare-none = Няма разлики.
btn-optimize-image = Оптимизирай изображението
msg-image-optimized = Заглавното изображение е оптимизирано.
msg-image-ok = Заглавното изображение вече е в границите.
msg-no-header-image = Няма заглавно изображение за оптимизиране.
verify-image-large = Изображението е твърде голямо ({ $width }×{ $height }): { $path }
verify-image-format = Неподдържан формат на изображение (.{ $ext }): { $path }
verify-image-unreadable = Нечетимо изображение: { $path }
menu-condition-editor = Редактор на условия…
condeditor-title = Редактор на условия
condeditor-set-by = Задава се от:
condeditor-used-by = Използва се от:
condeditor-filedeps = Зависимости от файлове
condeditor-empty = Няма флагове или зависимости в този проект.
condeditor-orphan-set = зададен, но никога неизползван
condeditor-orphan-used = използван, но никога незададен
msg-img-optimized = Изображението е оптимизирано.
msg-img-ok = Изображението вече е в границите.
msg-img-none = Няма изображение за оптимизиране.
msg-crash-recovery = Предишната сесия приключи неочаквано. Резервно копие на проекта ви беше записано в { $path }
export-progress-title = Създаване на архива за разпространение…
export-progress-files = { $done } / { $total } файла
msg-export-cancelled = Експортирането е отменено; частичният архив беше премахнат.
verify-running = Проверка на файловете на диска…
verify-stale = Забележка: проектът беше променен по време на проверката на файловете; стартирайте проверката отново.
prop-col-name = Име
menu-save-as = Запазване като…
menu-project = Проект
menu-tools = Инструменти
menu-manual = Ръководство за потребителя
msg-manual-missing = Ръководството за потребителя (PDF) не беше намерено до приложението.
toolbar-new = Нов
toolbar-open = Отваряне
toolbar-save = Запазване
toolbar-validate = Проверка
toolbar-preview = Преглед
toolbar-export = Експорт
dialog-choose-root = Изберете коренната папка на мода
exit-unsaved-docs = Незапазени: { $names }
status-summary = { $steps } стъпки · { $options } опции
section-groups = Групи
section-options = Опции
section-flags = Флагове за условие
section-files = Файлове за инсталиране
hint-group-type = Как инсталаторът позволява на потребителя да избира опции в тази група.
hint-default-type = Как се предлага опцията, когато никой от шаблоните за зависимост не съвпада: задължителна, по избор, препоръчителна, неизползваема…
hint-operator = Всички условия трябва да са верни (И) или поне едно (ИЛИ).
hint-flags = Флаговете са именувани стойности, които тази опция задава при избор. Други стъпки и опции могат да ги проверяват, за да се покажат, скрият или станат задължителни.
hint-plugin-dependencies = Шаблони, които променят типа на опцията според флагове или файлове в играта: например „Задължителна“, когато е инсталиран друг мод.
hint-files = Файлове и папки, копирани в папката Data на играта при избор на тази опция. Местоназначението е спрямо Data; при конфликт печели по-високият приоритет.
hint-visibility = Условия, които трябва да са изпълнени, за да се покаже тази стъпка. Оставете празно, за да се показва винаги.
seltype-exactly-one = Точно една (задължително)
seltype-at-most-one = Най-много една
seltype-any = Произволен брой
seltype-all = Всички (без избор)
seltype-at-least-one = Поне една
plugtype-required = Задължителна
plugtype-optional = По избор
plugtype-recommended = Препоръчителна
plugtype-not-usable = Неизползваема
plugtype-could-be-usable = Може да е използваема
plugtype-required-hint = Винаги се инсталира; потребителят не може да я размаркира.
plugtype-optional-hint = Предлага се немаркирана; потребителят решава.
plugtype-recommended-hint = Предлага се маркирана; потребителят може да я размаркира.
plugtype-not-usable-hint = Показва се в сиво и не може да бъде избрана.
plugtype-could-be-usable-hint = Може да се избере, но инсталаторът предупреждава, че може да не работи.
op-and = Всички условия (И)
op-or = Което и да е условие (ИЛИ)
theme-dark = Тъмна
theme-light = Светла
theme-system = Според системата
condeditor-setter-loc = Стъпка { "{step}" } / Група { "{group}" } / „{ "{name}" }“
condeditor-pattern-of = Шаблон на „{ "{name}" }“ → { "{type}" }
condeditor-visibility-of = Видимост на стъпка { "{step}" }
condeditor-cond-set = Условен набор { "{num}" }
condeditor-needs = { "{ctx}" } (изисква = { "{value}" })
condeditor-file-dep = { "{ctx}" }: файл „{ "{name}" }“ ({ "{state}" })
menu-translate-fomod = Преведи FOMOD…
ftr-title = Превод на FOMOD
ftr-open-folder = Отвори папка на мод…
ftr-from-active = От активния проект
ftr-from-active-hint = Превежда FOMOD на проекта, отворен в главния прозорец (първо трябва да бъде запазен).
ftr-no-fomod = Няма зареден FOMOD.
ftr-encoding = Кодиране на оригиналните файлове; преведените файлове се записват със същото кодиране.
ftr-source-lang = От
ftr-target-lang = на
ftr-lang-locked = (езиците са фиксирани, след като е зареден FOMOD)
ftr-translator = Преводач:
ftr-save = Запази превода
ftr-export = Експортирай преведените файлове
ftr-export-sibling = В папка fomod_<език>
ftr-export-sibling-hint = Записва преведените info.xml и ModuleConfig.xml до оригиналната папка fomod; оригиналните файлове не се променят.
ftr-export-inplace = Върху оригиналните файлове
ftr-export-inplace-hint = Заменя fomod/info.xml и fomod/ModuleConfig.xml, след като направи .bak копие с времеви печат на всеки от тях.
ftr-force-explicit-order = Запази оригиналния ред
ftr-warn-order = Списъците, сортирани по име (order="Ascending"), биха били пренаредени по преведените имена в мениджъра на модове. Тази опция налага order="Explicit", за да запазят опциите текущия си ред.
ftr-update = Обнови от папката
ftr-update-hint = Прочита отново FOMOD от диска и обединява превода с него: съобщава се за нови, променени и премахнати низове.
ftr-preview-translated = Преглед на превода
ftr-progress = Преведени { $done } / { $total }
ftr-filter-all = Всички
ftr-filter-untranslated = Непреведени
ftr-filter-review = За преглед
ftr-filter-issues = С проблеми
ftr-filter-locked = Заключени
ftr-type-all = Всички полета
ftr-type-names = Имена
ftr-type-descriptions = Описания
ftr-type-meta = Информация за мода
ftr-search-hint = Търсене в оригинала, превода или контекста…
ftr-next-untranslated = Следващ непреведен
ftr-show-whitespace = Показвай интервалите и новите редове
ftr-discard-question = Текущият превод има незапазени промени. Да бъдат ли отхвърлени и да се зареди другият FOMOD?
ftr-discard-yes = Отхвърли
ftr-unsaved-close = Преводът има незапазени промени.
ftr-col-num = №
ftr-col-status = { "" }
ftr-col-context = Контекст
ftr-col-source = Оригинал
ftr-col-target = Превод
ftr-col-issues = { "" }
ftr-empty-hint = Отворете папка на мод или заредете активния проект, за да се покажат преводимите му низове.
ftr-empty-filter = Няма низ, който да отговаря на текущия филтър.
ftr-select-row = Изберете ред, за да редактирате превода му.
ftr-copy-source = Копирай оригинала
ftr-clear-target = Изчисти
ftr-lock = Не превеждай
ftr-lock-hint = Заключените низове се записват непроменени (автор, уебсайт, собствени имена…).
ftr-note = Бележка:
ftr-status-untranslated = Непреведен
ftr-status-translated = Преведен
ftr-status-auto = Попълнен автоматично — моля, прегледайте
ftr-status-fuzzy = Изходният текст е променен след превода — моля, прегледайте
ftr-status-obsolete = Вече не съществува във FOMOD
ftr-status-locked = Заключен (записва се непроменен)
ftr-field-info-name = Име на мода (info.xml)
ftr-field-module-name = Заглавие на инсталатора (ModuleConfig.xml)
ftr-field-author = Автор
ftr-field-website = Уебсайт
ftr-field-description = Описание на мода
ftr-field-step = Име на стъпката
ftr-field-group = Име на групата
ftr-field-plugin = Име на опцията
ftr-field-plugin-desc = Описание на опцията
ftr-issue-empty = Празен превод
ftr-issue-whitespace = Преводът съдържа само интервали
ftr-issue-edge-whitespace = Интервалите в началото или в края се различават от оригинала
ftr-issue-token = Защитените токени се различават — липсващи: { $missing } ; излишни: { $extra }
ftr-issue-newline-name = Името не може да съдържа нов ред
ftr-issue-control = Съдържа знаци, които XML не може да съхрани
ftr-issue-length = Необичайна дължина спрямо оригинала (×{ $ratio })
ftr-issue-identical = Еднакъв с оригинала
ftr-issue-duplicate = Същият изходен текст е преведен различно в { $key }
ftr-issue-cdata = Последователността ]]> не е разрешена тук
ftr-load-error = FOMOD не можа да бъде зареден: { $error }
ftr-extracted = Намерени преводими низове: { $num }.
ftr-sidecar-found = Съществуващият превод е зареден и обединен: нови { $new }, променени { $changed }, премахнати { $removed }.
ftr-saved = Преводът е запазен в { $path }
ftr-save-error = Преводът не можа да бъде запазен: { $error }
ftr-save-first = Първо запазете проекта, след това го преведете.
ftr-export-success = Низове, записани в { $path }: { $count }
ftr-export-error = Експортирането е неуспешно: { $error }
ftr-export-blocked = Блокиращи проблеми, които трябва да се отстранят преди експортиране: { $num }.
ftr-export-stale = Пропуснати низове, защото FOMOD е променен: { $num }; използвайте „Обнови от папката“.
ftr-update-report = Обновено: нови { $new }, променени { $changed }, преместени { $moved }, премахнати { $removed }, непроменени { $unchanged }.
menu-edit = Редактиране
menu-undo = Отмяна
menu-redo = Възстановяване
tree-title = Проект
tree-mod-info = Информация за мода
tree-steps = Стъпки за инсталиране
tree-required = Задължителни файлове
tree-conditional = Условни инсталации
tree-empty-steps = Все още няма стъпки — щракнете върху +, за да добавите.
tree-duplicate = Дублиране
tree-delete = Изтриване
tree-save-template = Запазване като шаблон…
tree-drop-hint = Пуснете тук, за да преместите
cond-set-label = Условен набор { $num }
inspector-empty = Изберете елемент в дървото на проекта или добавете стъпка, за да започнете.
count-options = Опции: { $num }
count-files = Файлове: { $num }
msg-deleted-undo = Изтрито. Използвайте „Отмяна“ (Ctrl+Z), за да го върнете.
problems-title = Проблеми
problems-errors = Грешки: { $num }
problems-warnings = Предупреждения: { $num }
btn-close = Затвори
ftr-export-package = Като пакет с превод (архив)
ftr-export-package-hint = Създава .zip или .7z, готов за качване: преведените info.xml и ModuleConfig.xml плюс README (само пач) или целия мод с преведените файлове (пълен).
ftr-package-full = Целият мод
ftr-package-full-hint = Включва всички файлове на мода в архива, а не само двата преведени XML файла. Уверете се, че авторът разрешава повторното разпространение.
ftr-package-name-template = Име:
ftr-readme-patch = Този архив съдържа превода (език: { $langname }) на инсталатора на „{ $name }“ (fomod/info.xml и fomod/ModuleConfig.xml). Инсталирайте го върху оригиналния мод или оставете мениджъра на модове да го обедини, за да заменят преведените файлове оригиналните. Променят се само текстовете на инсталатора; самите файлове на мода не са включени. Създадено с XIMOD Architect.
ftr-readme-full = Този архив съдържа „{ $name }“ с преведен инсталатор (език: { $langname }; fomod/info.xml и fomod/ModuleConfig.xml). Инсталирайте го като оригиналния мод. Променени са само текстовете на инсталатора. Създадено с XIMOD Architect.
ftr-apply-memory = Попълни от паметта
ftr-memory-size = Преводна памет — записи за тази езикова двойка: { $num }. Всеки запазен превод се добавя към нея.
ftr-memory-applied = Низове, попълнени от преводната памет (отбелязани „за преглед“): { $num }.
ftr-memory-suggestion = Паметта предлага:
ftr-use-suggestion = Използвай
ftr-propagate = Разпространи към еднаквите
ftr-propagate-hint = Копира този превод във всеки друг все още непреведен низ със същия изходен текст.
ftr-propagated = Попълнени еднакви низове: { $num }.
ftr-csv-export = Експортирай CSV…
ftr-csv-import = Импортирай CSV…
ftr-csv-imported = Низове, обновени от CSV файла: { $num }.
ftr-csv-error = Грешка в CSV: { $error }
ftr-glossary = Глосар
ftr-glossary-source = Термин
ftr-glossary-target = Превод
ftr-glossary-case = Регистър
ftr-glossary-dnt = Остави
ftr-glossary-add = Добави термин
ftr-issue-glossary = Глосар: „{ $term }“ не е преведен според очакваното

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Отвори архив…
filter-archive = Архиви на модове (zip, 7z)
msg-archive-opened = Архивът е отворен (извлечени файлове: { $num }): { $path }
msg-archive-reused = Архивът вече е извлечен, използва се повторно { $path }
msg-archive-unsupported = Форматът на архива „.{ $ext }“ не се поддържа; първо го извлечете със 7-Zip (могат да се отварят само .zip и .7z).
msg-archive-error = Грешка при отваряне на архива: { $error }
msg-archive-no-fomod = В архива не е намерена папка „fomod“ ({ $path })
msg-archive-extracting = Извличане на архива…
ftr-open-archive = Отвори архив на мод…
ftr-package-full-partial = Модът е отворен от архив само с папката fomod; пълните пакети изискват извлечения мод.
info-module-deps = Изисквания на мода
info-module-deps-hint = Файлове или флагове, които целият мод изисква, преди инсталаторът да се стартира (moduleDependencies). Оставете празно, ако няма такива.
info-header-advanced = Разширена заглавна част
info-title-position = Позиция на заглавието
info-title-colour = Цвят на заглавието
info-title-colour-hint = Очаква се: шест шестнадесетични цифри (RRGGBB)
info-image-show = Показвай заглавното изображение
info-image-fade = Избледняване на заглавното изображение
info-image-height = Височина на заглавното изображение
info-attr-default = (по подразбиране)
file-always-install = Винаги
file-always-install-hint = Винаги инсталирай този файл, дори когато опцията не е избрана (alwaysInstall).
file-install-if-usable = Ако е годен
file-install-if-usable-hint = Инсталирай този файл винаги, когато опцията е използваема, дори да не е избрана (installIfUsable).
msg-import-lossy = Този FOMOD съдържа конструкции, които XIMOD не може да редактира (брой: { $num }); те ще бъдат отхвърлени при запазване на проекта.
fidelity-nested-deps = Вложена група зависимости в { $context } (поддържа се само едно ниво)
fidelity-game-dep = Изискване за версия на играта { $version } в { $context }
fidelity-fomm-dep = Изискване за версия на мениджъра на модове { $version } в { $context }
fidelity-unknown = Елементът „{ $element }“ в „{ $parent }“ не се поддържа ({ $context })
loc-module = изискванията на мода
loc-step = стъпка { $step } „{ $name }“
loc-installer = инсталатора

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Възстанови резервно копие…
backups-title = Възстановяване на резервно копие
backups-empty = Този проект все още няма резервно копие. Такова се създава при всяко записване на проекта върху предишна версия.
backups-changes = Брой промени спрямо текущия проект: { $num }
btn-compare = Сравни
btn-restore = Възстанови
btn-delete-backups = Изтрий всички резервни копия
btn-delete-backups-confirm = Щракнете отново, за да изтриете всички резервни копия
msg-backup-restored = Резервното копие от { $time } е възстановено в редактора (още не е записано; „Отмяна“ го връща)
msg-backups-deleted = Изтрити резервни копия: { $num }
settings-backup-count = Резервни копия за пазене:
settings-backup-count-hint = Брой предишни версии на XML файловете на FOMOD, пазени във fomod/backups при записване (0 = без резервни копия).
settings-autosave-minutes = Автоматично записвай копие за възстановяване на всеки (минути):
settings-autosave-minutes-hint = През този интервал в папката с конфигурацията се записва копие за възстановяване на всеки променен проект; при следващото стартиране то се предлага само след необичайно затваряне (0 = изключено).
settings-auto-masters = Добавяй мастър файловете на плъгин като условия
settings-auto-masters-hint = Когато плъгин (.esp/.esm/.esl) бъде добавен към опция, изискваните от него мастър файлове, които нито играта, нито този мод предоставят, стават файлови условия „Active“ на опцията.
msg-author-from-plugin = Авторът е попълнен от заглавната част на плъгина: { $author }
msg-masters-added = Мастър файлове на { $plugin }, добавени като файлови условия: { $num }
issue-missing-master = { $plugin } изисква { $master }, който нито е в този мод, нито е деклариран като зависимост
issue-esl-mismatch-flag = { $plugin } има разширение .esl, но неговият флаг light (ESL) не е зададен
issue-esl-eligible = { $plugin } може да бъде маркиран като light (нови записи: { $num }, лимит { $limit })
issue-esl-too-big = { $plugin } е маркиран като light, но не отговаря на правилата за light плъгини (нови записи: { $num }, лимит { $limit }, или FormID извън позволения диапазон)
menu-plugin-report = Отчет за плъгините…
plugins-title = Отчет за плъгините
plugins-file = Файл
plugins-kind = Вид
plugins-light = Флаг light
plugins-masters = Мастър файлове
plugins-new-records = Нови записи / лимит
plugins-eligible = Подходящ за light
plugins-empty = Този проект не инсталира нито един файл на плъгин (.esp, .esm или .esl).
plugins-unreadable = нечетим

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Крайно дърво на файловете
preview-total-size = Общ размер на инсталацията: { $size }
preview-tree-truncated = Дървото е съкратено: твърде много файлове за показване (размерите по-горе са частични).
preview-overwritten-by = Презаписан от { $plugin }
preview-scenario = Сценарий:
preview-scenario-load = Зареди
preview-scenario-save = Запази…
preview-scenario-delete = Изтрий
preview-scenario-name = Име на сценария
preview-scenario-saved = Сценарият „{ $name }“ е запазен в fomod/scenarios
preview-scenario-unresolved = Избори от сценария, които не съответстват на нито една опция в този проект (преименувана или премахната): { $num }
preview-scenario-none = (няма сценарий)
issue-unreachable-step = Стъпка „{ $step }“ никога не може да бъде показана: условията ѝ за видимост проверяват стойност на флаг, която никоя предишна опция не задава
issue-unreachable-option = Опция „{ $plugin }“ никога не може да бъде избрана: шаблоните ѝ за използваем тип проверяват стойност на флаг, която никоя опция не задава
issue-unreachable-cond = Условният набор от файлове { $num } никога не може да се приложи: условията му проверяват стойност на флаг, която никоя опция не задава
size-option = Размер на инсталацията: { $size } (файлове: { $num })
size-missing = Липсващи източници: { $num }
size-unknown = Размер на инсталацията: — (стартирайте Валидиране, за да бъде измерен)
menu-nexus-desc = Описание за Nexus…
nexus-title = Описание за Nexus Mods
nexus-format = Формат:
nexus-include-requirements = Изисквания
nexus-include-options = Опции за инсталиране
nexus-include-install = Инсталиране
nexus-include-changelog = Списък с промени
nexus-previous = Предишна версия…
nexus-previous-none = (няма предишна версия: без списък с промени)
nexus-language = Език:
nexus-language-source = (източник)
nexus-sec-requirements = Изисквания
nexus-sec-options = Опции за инсталиране
nexus-sec-install = Инсталиране
nexus-sec-changelog = Списък с промени
nexus-install-text = Този мод се доставя с инсталатор FOMOD: инсталирайте го с мениджър на модове (Vortex, Mod Organizer 2) и изберете опциите си в инсталатора.
nexus-requires = Изисква
nexus-step = Стъпка
nexus-added = Добавено
nexus-removed = Премахнато
nexus-changed = Променено
btn-copy = Копирай
btn-save-as = Запазване като…
msg-copied = Копирано в клипборда
msg-saved-to = Запазено в { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Преименувай…
condeditor-rename-exists = Вече съществува флаг с име „{ $name }“
condeditor-renamed = Флагът „{ $from }“ е преименуван на „{ $to }“ (срещания: { $num })
condeditor-delete-uses = Изтрий всички употреби
condeditor-deleted-uses = Флагът „{ $name }“ е премахнат отвсякъде (срещания: { $num })
condeditor-values-set = Зададени стойности:
condeditor-values-tested = Проверявани стойности:
condeditor-value-never-set = { $value } — проверява се, но никога не се задава
condeditor-value-never-tested = { $value } — задава се, но никога не се проверява
condeditor-builder = Конструктор на условия
condeditor-builder-none = Изберете стъпка, опция, условен набор от файлове или информацията за мода в главния прозорец, за да редактирате условията им тук.
condeditor-builder-pattern = Шаблон:
condeditor-sentence-if = АКО
condeditor-sentence-and = И
condeditor-sentence-or = ИЛИ
condeditor-sentence-flag = флагът { "{name}" } = { "{value}" }
condeditor-sentence-file = файлът { "{name}" } е { "{value}" }
condeditor-sentence-empty = (няма условие: винаги вярно)
condeditor-sentence-then-visible = ТОГАВА стъпката се показва
condeditor-sentence-then-type = ТОГАВА опцията става { $type }
condeditor-sentence-then-install = ТОГАВА файловете се инсталират
condeditor-sentence-then-module = ТОГАВА инсталаторът може да се стартира (проверява се преди стартирането му)
issue-flag-value-never-set = Флагът „{ $flag }“ се проверява със стойност „{ $value }“, която никоя опция не задава
issue-flag-never-used = Флагът „{ $flag }“ се задава, но никъде не се проверява
menu-project-strings = Низове на проекта…
strings-title = Низове на проекта
strings-search = Търсене по текст, местоположение или ключ…
strings-kind-all = Всички
strings-kind-names = Имена
strings-kind-descriptions = Описания
strings-duplicates-only = Само дубликати
strings-replace-with = Замени с:
strings-case = Съвпадение на регистъра
strings-whole-word = Цяла дума
strings-replace-current = Замени
strings-replace-all = Замени всички
strings-replaced = Заменени низове: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Същият текст като:
strings-count = Низове: { $num } · групи дубликати: { $dups }
strings-col-location = Местоположение
strings-col-field = Поле
strings-col-text = Текст

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Съдържание на архив…
filter-bethesda-archive = Архиви на Bethesda (bsa, ba2)
archive-view-title = Съдържание на архива
archive-view-format = Формат:
archive-view-entries = Записи: { $num }
archive-view-size = { $size } разопаковани
archive-view-search = Търсене на път…
archive-view-col-path = Път
archive-view-col-size = Размер
archive-view-col-compressed = Компресиран
archive-view-truncated = Показани са само първите { $num } съвпадащи записа — прецизирайте търсенето.
archive-view-error = Този архив не може да бъде прочетен: { $error }
archive-view-hint = Преглед на съдържанието на този архив
issue-conflict-archive = Един и същ ресурс в няколко архива: „{ $path }“ е пакетиран от { $count } препратки ({ $locs }) — редът на зареждане на архивите в играта решава кой ще се използва.
issue-conflict-archive-loose = Архив срещу свободен файл: „{ $path }“ е едновременно пакетиран в архив и инсталиран като свободен файл ({ $locs }) — свободният файл има предимство пред този в архива.
preview-in-archive = (в архив)
preview-archived-size = от които { $size } пакетирани в архиви

# --- Project tree: expand / collapse menus
tree-expand = Разгъване
tree-collapse = Свиване
tree-expand-all = Разгъни всички
tree-expand-selected = Разгъни избраното
tree-expand-from = Разгъни от избраното
tree-collapse-all = Свий всички
tree-collapse-selected = Свий избраното
tree-collapse-from = Свий от избраното
tree-expand-all-hint = Разгъва всички заглавия
tree-expand-selected-hint = Разгъва само избраното заглавие
tree-expand-from-hint = Разгъва избраното заглавие и всичко под него
tree-collapse-all-hint = Свива всички заглавия
tree-collapse-selected-hint = Свива само избраното заглавие
tree-collapse-from-hint = Свива избраното заглавие и всичко под него

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Добавяне на група
btn-remove-group-cond = Премахване на групата
dep-type-game = Версия на играта
dep-type-fomm = Версия на мениджъра на модове
dep-group-hint = Група от условия, съчетани с И / ИЛИ; групите могат да се влагат една в друга.
condeditor-sentence-game = версията на играта ≥ { "{value}" }
condeditor-sentence-fomm = версията на мениджъра на модове ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Уникални текстове
ftr-uniques-hint = Показва по един ред за всеки различен изходен текст. Преводът на този ред превежда наведнъж всички низове със същия текст.
ftr-uniques-synced = Обновени еднакви низове: { $num }.
ftr-uniques-group = Низове с този текст: { $num }; преводът му се прилага към всички тях.
