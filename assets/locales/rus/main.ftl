# XIMOD Architect - translation metadata
# @language = rus
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Русский
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Версия { $version }

# Status messages
status-ready = Готово
msg-save-success = FOMOD успешно сохранен
msg-save-error = Ошибка при сохранении FOMOD
msg-export-success = Создан архив дистрибутива ({ $count } файлов): { $path }
msg-export-error = Ошибка при создании архива дистрибутива: { $error }
msg-load-success = FOMOD успешно загружен
msg-load-error = Ошибка при загрузке FOMOD
msg-merge-success = FOMOD успешно объединен
msg-merge-error = Ошибка при объединении FOMOD
msg-no-root-selected = Сначала выберите корневой каталог
msg-no-fomod-folder = Папка «fomod» не найдена. Создать?
msg-file-outside-root = Файл находится вне корневого каталога

# Menu - File
menu-file = Файл
menu-new = Новый
menu-open = Открыть папку…
menu-open-file = Открыть файл…
menu-save = Сохранить
menu-recent = Недавние
menu-exit = Выход
menu-merge = Объединить FOMOD…
menu-export = Экспортировать архив дистрибутива…
# Menu - Options
menu-options = Параметры
menu-settings = Настройки…
menu-pre-save-script = Скрипт перед сохранением…
menu-post-save-script = Скрипт после сохранения…
menu-translation = Перевести интерфейс…
# Menu - Help
menu-help = Справка
menu-check-updates = Проверить обновления…
menu-about = О программе

# Update check
update-checking = Проверка обновлений…
update-up-to-date = XIMOD Architect обновлён до последней версии.
update-check-failed = Не удалось проверить обновления. Повторите попытку позже.
update-available-status = Доступна версия { $version }.
update-banner-text = Доступен XIMOD Architect { $version }.
update-download = Скачать:
update-skip = Пропустить эту версию
update-later = Позже

# Tabs
tab-info = Информация о моде
tab-steps = Этапы установки
tab-required = Обязательные установки
tab-conditional = Условные установки

# Info Tab
label-workspace = Рабочая область
label-root-dir = Корневой каталог:
label-mod-name = Название мода:
label-author = Автор:
label-version = Версия:
label-game-name = Название игры:
label-category = Категория:
label-url = URL сайта:
label-header-image = Изображение заголовка:
label-description = Описание:
placeholder-select-dir = (Выберите каталог)
placeholder-select-game = (Выберите игру)

# Steps Tab
label-step-name = Название шага:
label-group-name = Название группы:
label-group-type = Тип группы:
label-plugin-name = Название опции:
label-plugin-desc = Описание:
label-plugin-type = Тип по умолчанию:
label-plugin-image = Изображение:
label-visibility = Условия видимости
label-operator = Оператор:

# Buttons
btn-browse = Обзор...
btn-clear = Очистить
btn-add = Добавить
btn-remove = Удалить
btn-add-step = Новый шаг
btn-delete-step = Удалить шаг
btn-add-group = Добавить группу
btn-remove-group = Удалить группу
btn-add-plugin = Добавить опцию
btn-remove-plugin = Удалить опцию
btn-add-file = Добавить файл
btn-add-folder = Добавить папку
btn-remove-file = Удалить
btn-add-flag = Добавить флаг
btn-remove-flag = Удалить флаг
btn-add-condition = Добавить условие
btn-remove-condition = Удалить условие
btn-add-dependency = Добавить зависимость
btn-remove-dependency = Удалить зависимость
btn-add-pattern = Новый шаблон
btn-remove-pattern = Удалить шаблон
btn-save = Сохранить
btn-cancel = Отменить
btn-ok = ОК
btn-yes = Да
btn-no = Нет

# Condition/Dependency Labels
label-flag-name = Имя флага:
label-flag-value = Значение:
label-condition-type = Тип:
label-condition-name = Имя:
label-condition-value = Значение:
label-dep-type = Тип зависимости:
label-dep-name = Имя/Файл:
label-dep-value = Значение/Состояние:

# Files
label-source = Источник
label-destination = Пункт назначения
label-priority = Приоритет
label-file-type = Тип

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Назначение для всей группы
label-page-dest = Назначение установки (вся страница)
btn-apply-group-dest = Применить ко всем опциям этой группы
btn-apply-page-dest = Применить ко всем опциям этой страницы
group-dest-hint = Задаёт одно место установки для каждого файла каждой опции этой группы.
page-dest-hint = Задаёт одно место установки для каждого файла каждой опции этой страницы (все группы).
bulk-dest-nofiles = Пока нет файлов для обновления — сначала добавьте файлы в опции.
status-dest-applied = Назначение применено к { $num } файлу(ам).
preview-hidden-steps = { $num } шаг(ов) скрыто текущим выбором.
label-files = Файлы
label-dependencies = Зависимости

# Settings Dialog
settings-title = Настройки
settings-tab-general = Общие
settings-tab-recent-files = Недавние файлы
settings-language = Язык:
settings-theme = Тема:
settings-font-size = Размер шрифта:
settings-replace-newlines = Обрабатывать символы новой строки в описаниях
settings-check-updates = Проверять обновления при запуске
settings-max-recent = Максимальное количество недавних файлов:
settings-window-width = Ширина окна:
settings-window-height = Высота окна:
settings-no-recent-files = Нет недавних файлов.

# Status messages for settings
status-settings-saved = Настройки успешно сохранены

# About Dialog
about-title = О программе XIMOD Architect
about-description = Кроссплатформенный инструмент для создания установщиков FOMOD для модификаций игр Bethesda.
about-license = Лицензировано по лицензии MIT
about-copyright = © 2025-2026 Команда XIMOD
about-credit = Порт оригинального инструмента от Wenderer на Rust:

# Script Dialog
script-title = Редактировать скрипт
script-info = Скрипты выполняются до или после сохранения. Вы можете использовать следующие макросы:
script-macros = Доступные макросы:
macro-modname = $MODNAME$ - Название мода
macro-modauthor = $MODAUTHOR$ - Имя автора
macro-modversion = $MODVERSION$ - Версия мода
macro-modroot = $MODROOT$ - Путь к корневому каталогу
macro-date = $DATE$ - Текущая дата (ГГГГ-ММ-ДД)
macro-time = $TIME$ - текущее время (ЧЧ:ММ:СС)
macro-random = $RANDOM$ - случайное число

# Plugin Dependencies
label-plugin-dependencies = Зависимости опции
label-default-type = Тип по умолчанию:
label-pattern-type = Тип шаблона:
label-pattern-operator = Оператор шаблона:

# Conditional Files
label-pattern = Шаблон

# Validation Messages
validation-no-name = Требуется указать название модуля
validation-no-steps = Требуется как минимум один шаг или обязательный файл
validation-empty-step = Шаг { $num } не имеет названия
validation-empty-group = Шаг { $step }, группа { $group } не имеет названия
validation-no-plugins = Шаг { $step }, группа "{ $name }" не имеет опций

# File States
state-active = Активен
state-inactive = Неактивен
state-missing = Отсутствует

# Confirmation
confirm-title = Подтверждение
confirm-delete = Вы действительно хотите удалить этот элемент?
confirm-discard = У вас есть несохраненные изменения. Отменить их и продолжить?
confirm-unsaved = У вас есть несохраненные изменения. Хотите сохранить перед закрытием?
confirm-save-issues = В проекте имеются следующие проблемы:
confirm-save-anyway = Сохранить все равно?

# Errors
error-invalid-xml = Недопустимый XML-файл
error-parse-failed = Не удалось проанализировать FOMOD
error-write-failed = Не удалось записать файл
error-create-dir = Не удалось создать каталог

# Default names (generated when creating new items)
default-step-name = Шаг { $num }
default-group-name = Группа { $num }
default-plugin-name = Опция { $num }
pattern-label = Шаблон { $num }

# Selection prompts
msg-select-group-first = Сначала выберите группу.
msg-select-plugin-edit = Выберите опцию для редактирования.
label-empty = (пусто)
image-no-image = Изображение отсутствует

# File dialog filters
filter-images = Изображения
filter-xml = XML

# Dependency types
dep-type-flag = Флаг
dep-type-file = Файл

# Status bar
status-modified = Изменено

# Status messages (errors)
msg-settings-save-error = Ошибка при сохранении настроек
msg-script-save-error = Ошибка при сохранении скрипта

# Translation editor
trans-title = Редактор переводов
trans-source-lang = Язык отображения:
trans-target-lang = Язык перевода:
trans-col-key = Ключ
trans-col-source = Метка
trans-col-target = Перевод
trans-saved = Перевод сохранен
trans-save-error = Ошибка при сохранении перевода

# XML editor
xml-editor-title = Редактор XML
xml-editor-edit = Редактировать
xml-editor-apply = Применить
xml-editor-revert = Отменить
xml-editor-readonly = Только для чтения
xml-editor-editing = Режим редактирования — графические вкладки заблокированы
xml-editor-error = Ошибка:
xml-editor-applied = Изменения в XML применены
xml-editor-wellformed = XML имеет правильную структуру
xml-editor-error-at = Строка { $line }, столбец { $col }: { $msg }

# Country / flag picker
settings-country-name = Название страны:
settings-pick-country = Нажмите, чтобы выбрать страну
flags-title = Выбрать страну
flags-filter = Фильтр:
flags-none = Флаг не найден

# Translation editor: country & font
trans-endonym = Эндоним страны:
trans-font = Шрифт:
trans-no-font = (нет)
trans-browse = Обзор…
trans-google-fonts = Шрифты Google
trans-pick-country = Нажмите, чтобы выбрать страну
trans-font-outside = Шрифт сначала необходимо установить в папку assets/fonts.
trans-font-dir-missing = Папка assets/fonts не найдена.

# Translation submission
trans-lang-endonym = Эндоним языка:
trans-author = Автор:
trans-submit = Отправить…
trans-submit-hint = Создать ZIP-архив и открыть предварительно заполненное электронное письмо
trans-data-updated = Справочные данные обновлены (Languages.json / Countries.json)
trans-package-ready = Архив готов:
trans-package-error = Не удалось создать архив:

# ISO 639-3 requirement
trans-lang-not-iso = Перевод возможен только для языков с кодом ISO 639-3.

# FOMOD installer preview
menu-preview = Предварительный просмотр установщика…
preview-title = Предварительный просмотр установщика FOMOD
preview-refresh = Обновить
preview-assumptions = Предположения о файлах
preview-details = Подробности
preview-back = Назад
preview-next = Далее
preview-install = Установить
preview-close = Закрыть
preview-restart = Перезапустить
preview-summary-title = Файлы, которые будут установлены
preview-empty = Ни один файл не будет установлен.
preview-none-option = (нет)
preview-invalid = Выполните необходимые настройки, чтобы продолжить.
preview-no-steps = Шагов не отображается; см. сводку установки.
preview-select-hint = Выберите опцию, чтобы увидеть её описание.
preview-col-source = Источник
preview-col-dest = Место назначения
preview-col-priority = Приоритет
preview-sel-exactlyone = Выберите ровно одну опцию.
preview-sel-atmostone = Выберите не более одной опции.
preview-sel-any = Выберите любое количество вариантов.
preview-sel-all = Установлены все варианты.
preview-sel-atleastone = Выберите как минимум один вариант.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Проверить FOMOD
validate-report-title = Проверка FOMOD
validate-ok = Проблем не обнаружено. FOMOD соответствует схеме.
xml-editor-schema-ok = Соответствует схеме ModConfig 5.0.
xml-editor-schema-issues = Проблемы со схемой:
schema-line-col = Строка { $line }, столбец. { $col }: { $msg }
schema-wrong-root = Неожиданный корневой элемент «{ $found }» (ожидался «{ $expected }»).
schema-unknown = Неожиданный элемент «{ $element }» в «{ $parent }».
schema-missing = «{ $parent }» должен содержать «{ $child }».
schema-needs-one = «{ $parent }» должен содержать как минимум один «{ $child }».
schema-too-many = «{ $child }» может встречаться в «{ $parent }» только один раз.
schema-missing-attr = Атрибут «{ $attr }» является обязательным для «{ $element }».
schema-bad-enum = Недопустимое значение «{ $value }» для { $element }/@{ $attr } (ожидается: { $allowed }).
schema-choose-one = «{ $parent }» должен содержать ровно один из элементов: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Переместить вперед
reorder-after = Переместить назад

# Country / language database explorer (Properties)
menu-properties = Свойства…
prop-title = База данных стран / языков
prop-tab-countries = Страны
prop-tab-languages = Языки
prop-filter = Фильтр:
prop-official-langs = Официальные языки
prop-spoken-langs = Языки, на которых говорят
prop-endonym = Эндоним страны
prop-font = Шрифт
prop-spoken-in = Используется в
prop-select-country = Выберите страну, чтобы просмотреть её сведения.
prop-select-lang = Выберите язык, чтобы просмотреть его сведения.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Открыть страницу модификаций игры на Nexus

# Referenced-file verification (V2)
verify-no-root = Проверка файлов пропущена: корневая папка не задана
loc-header = изображение заголовка
loc-required = обязательные файлы
loc-conditional = условный набор { $num }
loc-plugin = шаг { $step }, группа { $group }, опция «{ $plugin }»
verify-missing-file = Отсутствует файл: { $path } ({ $loc })
verify-missing-folder = Отсутствует папка: { $path } ({ $loc })
verify-missing-image = Отсутствует изображение: { $path } ({ $loc })
verify-absolute = Абсолютный путь (непереносимый): { $path } ({ $loc })
verify-outside = Путь выходит за пределы корневой папки: { $path } ({ $loc })
verify-orphan = Файл-сирота (не используется ни одной опцией): { $path }
conflict-certain = Конфликт назначения: «{ $path }» записывают { $count } вариантов ({ $locs }) — они перезаписывают друг друга.
conflict-potential = Возможный конфликт назначения: «{ $path }» — цель { $count } ссылок ({ $locs }); перезапись зависит от выбора/условий.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Закрыть FOMOD
menu-close-all-fomods = Закрыть все FOMOD
tab-untitled = (без названия)
msg-drop-not-fomod = Перетащенный элемент не является FOMOD (папка «fomod» не найдена)
exit-title = Несохранённые изменения
exit-unsaved = FOMOD не сохранён. Сохранить его?
tab-close-hint = Закрыть этот FOMOD
menu-new-from-folder = Создать из папки…
menu-templates = Шаблоны…
templates-title = Многоразовые шаблоны
templates-empty = Пока нет сохранённых шаблонов. Сохраните выбранный шаг выше, чтобы создать шаблон.
templates-insert = Вставить
templates-save-step = Сохранить выбранный шаг
templates-name-hint = Имя шаблона (необязательно)
msg-wizard-success = Каркас создан из папки: { $num } вариант(ов).
msg-wizard-error = Ошибка: { $error }
msg-template-saved = Шаблон сохранён: { $name }
msg-template-inserted = Шаблон вставлен в проект.
msg-template-no-step = Сначала выберите шаг, чтобы сохранить его как шаблон.
msg-template-no-dir = Не удалось найти папку шаблонов.
msg-drop-assigned = К варианту добавлено { $added } источник(ов) ({ $rejected } вне корня пропущено).
menu-compare = Сравнить с…
compare-title = Сравнение FOMOD
compare-none = Нет различий.
btn-optimize-image = Оптимизировать изображение
msg-image-optimized = Изображение-заголовок оптимизировано.
msg-image-ok = Изображение-заголовок уже в пределах.
msg-no-header-image = Нет изображения-заголовка для оптимизации.
verify-image-large = Изображение слишком большое ({ $width }×{ $height }): { $path }
verify-image-format = Неподдерживаемый формат изображения (.{ $ext }): { $path }
verify-image-unreadable = Нечитаемое изображение: { $path }
menu-condition-editor = Редактор условий…
condeditor-title = Редактор условий
condeditor-set-by = Устанавливается:
condeditor-used-by = Используется:
condeditor-filedeps = Зависимости от файлов
condeditor-empty = В этом проекте нет флагов или зависимостей.
condeditor-orphan-set = задан, но не используется
condeditor-orphan-used = используется, но не задан
msg-img-optimized = Изображение оптимизировано.
msg-img-ok = Изображение уже в пределах.
msg-img-none = Нет изображения для оптимизации.
msg-crash-recovery = Предыдущий сеанс завершился непредвиденно. Резервная копия вашего проекта сохранена в { $path }
export-progress-title = Создание архива для распространения…
export-progress-files = { $done } / { $total } файлов
msg-export-cancelled = Экспорт отменён; частичный архив удалён.
verify-running = Проверка файлов на диске…
verify-stale = Примечание: проект изменился во время проверки файлов; запустите проверку заново.
prop-col-name = Название
menu-save-as = Сохранить как…
menu-project = Проект
menu-tools = Инструменты
menu-manual = Руководство пользователя
msg-manual-missing = Руководство пользователя (PDF) не найдено рядом с приложением.
toolbar-new = Создать
toolbar-open = Открыть
toolbar-save = Сохранить
toolbar-validate = Проверить
toolbar-preview = Предпросмотр
toolbar-export = Экспорт
dialog-choose-root = Выберите корневую папку мода
exit-unsaved-docs = Не сохранено: { $names }
status-summary = { $steps } шагов · { $options } опций
section-groups = Группы
section-options = Опции
section-flags = Флаги условий
section-files = Устанавливаемые файлы
hint-group-type = Как установщик позволяет пользователю выбирать опции в этой группе.
hint-default-type = Как опция предлагается, когда ни один шаблон зависимостей не совпал: обязательная, необязательная, рекомендуемая, недоступная…
hint-operator = Все условия должны быть истинны (И) или любое одно из них (ИЛИ).
hint-flags = Флаги — это именованные значения, которые эта опция задаёт при выборе. Другие шаги и опции могут проверять их, чтобы показываться, скрываться или становиться обязательными.
hint-plugin-dependencies = Шаблоны, меняющие тип опции в зависимости от флагов или файлов в игре: например, «Обязательная», когда установлен другой мод.
hint-files = Файлы и папки, копируемые в папку Data игры при выборе этой опции. Назначение задаётся относительно Data; при конфликте побеждает более высокий приоритет.
hint-visibility = Условия, которые должны выполняться, чтобы этот шаг вообще показывался. Оставьте пустым, чтобы показывать всегда.
seltype-exactly-one = Ровно одна (обязательно)
seltype-at-most-one = Не более одной
seltype-any = Любое количество
seltype-all = Все (без выбора)
seltype-at-least-one = Не менее одной
plugtype-required = Обязательная
plugtype-optional = Необязательная
plugtype-recommended = Рекомендуемая
plugtype-not-usable = Недоступная
plugtype-could-be-usable = Возможно доступная
plugtype-required-hint = Всегда устанавливается; пользователь не может снять отметку.
plugtype-optional-hint = Предлагается без отметки; решает пользователь.
plugtype-recommended-hint = Предлагается с отметкой; пользователь может её снять.
plugtype-not-usable-hint = Показывается серой, выбрать нельзя.
plugtype-could-be-usable-hint = Можно выбрать, но установщик предупреждает, что она может не работать.
op-and = Все условия (И)
op-or = Любое условие (ИЛИ)
theme-dark = Тёмная
theme-light = Светлая
theme-system = Как в системе
condeditor-setter-loc = Шаг { "{step}" } / Группа { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Шаблон «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Видимость шага { "{step}" }
condeditor-cond-set = Условный набор { "{num}" }
condeditor-needs = { "{ctx}" } (требует = { "{value}" })
condeditor-file-dep = { "{ctx}" }: файл «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = Перевести FOMOD…
ftr-title = Перевод FOMOD
ftr-open-folder = Открыть папку мода…
ftr-from-active = Из активного проекта
ftr-from-active-hint = Перевести FOMOD проекта, открытого в главном окне (сначала его нужно сохранить).
ftr-no-fomod = FOMOD не загружен.
ftr-encoding = Кодировка исходных файлов; переведённые файлы записываются в той же кодировке.
ftr-source-lang = С
ftr-target-lang = на
ftr-lang-locked = (после загрузки FOMOD языки изменить нельзя)
ftr-translator = Переводчик:
ftr-save = Сохранить перевод
ftr-export = Экспортировать переведённые файлы
ftr-export-sibling = В папку fomod_<язык>
ftr-export-sibling-hint = Записывает переведённые info.xml и ModuleConfig.xml рядом с исходной папкой fomod; исходные файлы не затрагиваются.
ftr-export-inplace = Поверх исходных файлов
ftr-export-inplace-hint = Заменяет fomod/info.xml и fomod/ModuleConfig.xml, предварительно создав для каждого копию .bak с отметкой времени.
ftr-force-explicit-order = Сохранить исходный порядок
ftr-warn-order = Списки, отсортированные по названию (order="Ascending"), менеджер модов пересортировал бы по переведённым названиям. Эта опция принудительно задаёт order="Explicit", чтобы опции сохранили текущий порядок.
ftr-update = Обновить из папки
ftr-update-hint = Заново читает FOMOD с диска и объединяет с ним перевод: сообщается о новых, изменённых и удалённых строках.
ftr-preview-translated = Предпросмотр перевода
ftr-progress = Переведено { $done } / { $total }
ftr-filter-all = Все
ftr-filter-untranslated = Непереведённые
ftr-filter-review = На проверку
ftr-filter-issues = С проблемами
ftr-filter-locked = Заблокированные
ftr-type-all = Все поля
ftr-type-names = Названия
ftr-type-descriptions = Описания
ftr-type-meta = Сведения о моде
ftr-search-hint = Поиск в оригинале, переводе или контексте…
ftr-next-untranslated = Следующая непереведённая
ftr-show-whitespace = Показывать пробелы и переводы строк
ftr-discard-question = В текущем переводе есть несохранённые изменения. Отменить их и загрузить другой FOMOD?
ftr-discard-yes = Отменить изменения
ftr-unsaved-close = В переводе есть несохранённые изменения.
ftr-col-num = №
ftr-col-status = { "" }
ftr-col-context = Контекст
ftr-col-source = Оригинал
ftr-col-target = Перевод
ftr-col-issues = { "" }
ftr-empty-hint = Откройте папку мода или загрузите активный проект, чтобы увидеть его переводимые строки.
ftr-empty-filter = Ни одна строка не соответствует текущему фильтру.
ftr-select-row = Выберите строку таблицы, чтобы изменить её перевод.
ftr-copy-source = Копировать оригинал
ftr-clear-target = Очистить
ftr-lock = Не переводить
ftr-lock-hint = Заблокированные строки записываются без изменений (автор, сайт, имена собственные…).
ftr-note = Примечание:
ftr-status-untranslated = Не переведена
ftr-status-translated = Переведена
ftr-status-auto = Заполнена автоматически — проверьте
ftr-status-fuzzy = Исходный текст изменился после перевода — проверьте
ftr-status-obsolete = Больше не существует в FOMOD
ftr-status-locked = Заблокирована (записывается без изменений)
ftr-field-info-name = Название мода (info.xml)
ftr-field-module-name = Заголовок установщика (ModuleConfig.xml)
ftr-field-author = Автор
ftr-field-website = Сайт
ftr-field-description = Описание мода
ftr-field-step = Название шага
ftr-field-group = Название группы
ftr-field-plugin = Название опции
ftr-field-plugin-desc = Описание опции
ftr-issue-empty = Пустой перевод
ftr-issue-whitespace = Перевод состоит только из пробелов
ftr-issue-edge-whitespace = Пробелы в начале или в конце отличаются от оригинала
ftr-issue-token = Защищённые токены различаются — отсутствуют: { $missing } ; лишние: { $extra }
ftr-issue-newline-name = Название не может содержать перевод строки
ftr-issue-control = Содержит символы, которые нельзя сохранить в XML
ftr-issue-length = Необычная длина по сравнению с оригиналом (×{ $ratio })
ftr-issue-identical = Совпадает с оригиналом
ftr-issue-duplicate = Тот же исходный текст переведён иначе в { $key }
ftr-issue-cdata = Последовательность ]]> здесь недопустима
ftr-load-error = Не удалось загрузить FOMOD: { $error }
ftr-extracted = Найдено переводимых строк: { $num }.
ftr-sidecar-found = Существующий перевод загружен и объединён: новых — { $new }, изменённых — { $changed }, удалённых — { $removed }.
ftr-saved = Перевод сохранён в { $path }
ftr-save-error = Не удалось сохранить перевод: { $error }
ftr-save-first = Сначала сохраните проект, затем переводите его.
ftr-export-success = Строк записано в { $path }: { $count }
ftr-export-error = Ошибка экспорта: { $error }
ftr-export-blocked = Блокирующих проблем, которые нужно исправить перед экспортом: { $num }.
ftr-export-stale = Пропущено строк из-за изменения FOMOD: { $num }; используйте «Обновить из папки».
ftr-update-report = Обновлено: новых — { $new }, изменённых — { $changed }, перемещённых — { $moved }, удалённых — { $removed }, без изменений — { $unchanged }.
menu-edit = Правка
menu-undo = Отменить
menu-redo = Повторить
tree-title = Проект
tree-mod-info = Информация о моде
tree-steps = Этапы установки
tree-required = Обязательные файлы
tree-conditional = Условные установки
tree-empty-steps = Шагов пока нет — нажмите +, чтобы добавить.
tree-duplicate = Дублировать
tree-delete = Удалить
tree-save-template = Сохранить как шаблон…
tree-drop-hint = Отпустите здесь, чтобы переместить
cond-set-label = Условный набор { $num }
inspector-empty = Выберите элемент в дереве проекта или добавьте шаг, чтобы начать.
count-options = Опции: { $num }
count-files = Файлы: { $num }
msg-deleted-undo = Удалено. Чтобы восстановить, используйте «Отменить» (Ctrl+Z).
problems-title = Проблемы
problems-errors = Ошибки: { $num }
problems-warnings = Предупреждения: { $num }
btn-close = Закрыть
ftr-export-package = Как пакет перевода (архив)
ftr-export-package-hint = Создаёт .zip или .7z, готовый к загрузке: переведённые info.xml и ModuleConfig.xml плюс README (только патч) либо весь мод с переведёнными файлами (полный).
ftr-package-full = Мод целиком
ftr-package-full-hint = Включить в архив все файлы мода, а не только два переведённых XML-файла. Убедитесь, что автор разрешает распространение.
ftr-package-name-template = Имя:
ftr-readme-patch = Этот архив содержит перевод установщика мода «{ $name }» (язык: { $langname }; файлы fomod/info.xml и fomod/ModuleConfig.xml). Установите его поверх оригинального мода или позвольте менеджеру модов объединить его, чтобы переведённые файлы заменили исходные. Меняются только тексты установщика; сами файлы мода в архив не входят. Создано в XIMOD Architect.
ftr-readme-full = Этот архив содержит мод «{ $name }» с переведённым установщиком (язык: { $langname }; файлы fomod/info.xml и fomod/ModuleConfig.xml). Установите его так же, как оригинальный мод. Изменены только тексты установщика. Создано в XIMOD Architect.
ftr-apply-memory = Заполнить из памяти
ftr-memory-size = Память переводов — записей для этой языковой пары: { $num }. В неё добавляется каждый сохранённый перевод.
ftr-memory-applied = Строк, заполненных из памяти переводов (помечены «на проверку»): { $num }.
ftr-memory-suggestion = Память предлагает:
ftr-use-suggestion = Использовать
ftr-propagate = Применить к одинаковым
ftr-propagate-hint = Скопировать этот перевод во все остальные ещё не переведённые строки с тем же исходным текстом.
ftr-propagated = Заполнено одинаковых строк: { $num }.
ftr-csv-export = Экспортировать CSV…
ftr-csv-import = Импортировать CSV…
ftr-csv-imported = Строк, обновлённых из файла CSV: { $num }.
ftr-csv-error = Ошибка CSV: { $error }
ftr-glossary = Глоссарий
ftr-glossary-source = Термин
ftr-glossary-target = Перевод
ftr-glossary-case = Регистр
ftr-glossary-dnt = Оставить
ftr-glossary-add = Добавить термин
ftr-issue-glossary = Глоссарий: «{ $term }» переведён не так, как ожидается

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Открыть архив…
filter-archive = Архивы модов (zip, 7z)
msg-archive-opened = Архив открыт (извлечено файлов: { $num }): { $path }
msg-archive-reused = Архив уже извлечён, повторно используется { $path }
msg-archive-unsupported = Формат архива «.{ $ext }» не поддерживается; сначала распакуйте его с помощью 7-Zip (открыть можно только .zip и .7z).
msg-archive-error = Ошибка при открытии архива: { $error }
msg-archive-no-fomod = В архиве не найдена папка «fomod» ({ $path })
msg-archive-extracting = Извлечение архива…
ftr-open-archive = Открыть архив мода…
ftr-package-full-partial = Мод открыт из архива, содержащего только папку fomod; для полных пакетов нужен распакованный мод.
info-module-deps = Требования мода
info-module-deps-hint = Файлы или флаги, необходимые всему моду до запуска установщика (moduleDependencies). Оставьте пустым, если их нет.
info-header-advanced = Расширенный заголовок
info-title-position = Положение названия
info-title-colour = Цвет названия
info-title-colour-hint = Ожидается: шесть шестнадцатеричных цифр (RRGGBB)
info-image-show = Показывать изображение заголовка
info-image-fade = Затухание изображения заголовка
info-image-height = Высота изображения заголовка
info-attr-default = (по умолчанию)
file-always-install = Всегда
file-always-install-hint = Всегда устанавливать этот файл, даже если опция не выбрана (alwaysInstall).
file-install-if-usable = Если можно
file-install-if-usable-hint = Устанавливать этот файл всякий раз, когда опция доступна для использования, даже если она не выбрана (installIfUsable).
msg-import-lossy = Этот FOMOD содержит конструкции, которые XIMOD не может редактировать (количество: { $num }); при сохранении проекта они будут отброшены.
fidelity-nested-deps = Вложенная группа зависимостей, расположение: { $context } (поддерживается только один уровень)
fidelity-game-dep = Требование к версии игры { $version }, расположение: { $context }
fidelity-fomm-dep = Требование к версии менеджера модов { $version }, расположение: { $context }
fidelity-unknown = Элемент «{ $element }» в «{ $parent }» не поддерживается ({ $context })
loc-module = требования мода
loc-step = шаг { $step } «{ $name }»
loc-installer = установщик

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Восстановить резервную копию…
backups-title = Восстановить резервную копию
backups-empty = У этого проекта пока нет резервных копий. Копия создаётся при каждом сохранении проекта поверх предыдущей версии.
backups-changes = Отличий от текущего проекта: { $num }
btn-compare = Сравнить
btn-restore = Восстановить
btn-delete-backups = Удалить все резервные копии
btn-delete-backups-confirm = Нажмите ещё раз, чтобы удалить все резервные копии
msg-backup-restored = Резервная копия от { $time } восстановлена в редакторе (ещё не сохранено; «Отменить» вернёт прежнее состояние)
msg-backups-deleted = Удалено резервных копий: { $num }
settings-backup-count = Хранить резервных копий:
settings-backup-count-hint = Количество предыдущих версий XML-файлов FOMOD, сохраняемых в fomod/backups при сохранении (0 = без резервных копий).
settings-autosave-minutes = Автосохранение копии для восстановления каждые (минут):
settings-autosave-minutes-hint = С этим интервалом в папку конфигурации записывается копия для восстановления каждого изменённого проекта; при следующем запуске она предлагается только после аварийного завершения (0 = отключено).
settings-auto-masters = Добавлять мастер-файлы плагина как условия
settings-auto-masters-hint = Когда плагин (.esp/.esm/.esl) добавляется к опции, требуемые им мастер-файлы, которые не предоставляют ни игра, ни этот мод, становятся файловыми условиями «Active» этой опции.
msg-author-from-plugin = Автор заполнен из заголовка плагина: { $author }
msg-masters-added = Мастер-файлов плагина { $plugin } добавлено как файловые условия: { $num }
issue-missing-master = { $plugin } требует { $master }, которого нет ни в этом моде, ни в объявленных зависимостях
issue-esl-mismatch-flag = { $plugin } имеет расширение .esl, но его флаг light (ESL) не установлен
issue-esl-eligible = { $plugin } можно пометить как light (новых записей: { $num }, предел { $limit })
issue-esl-too-big = { $plugin } помечен как light, но не соответствует правилам для light-плагинов (новых записей: { $num }, предел { $limit }, либо FormID вне допустимого диапазона)
menu-plugin-report = Отчёт о плагинах…
plugins-title = Отчёт о плагинах
plugins-file = Файл
plugins-kind = Тип
plugins-light = Флаг light
plugins-masters = Мастер-файлы
plugins-new-records = Новых записей / предел
plugins-eligible = Подходит для light
plugins-empty = Этот проект не устанавливает ни одного файла плагина (.esp, .esm или .esl).
plugins-unreadable = не читается

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Итоговое дерево файлов
preview-total-size = Общий размер установки: { $size }
preview-tree-truncated = Дерево усечено: слишком много файлов для раскрытия (размеры выше неполные).
preview-overwritten-by = Перезаписано опцией { $plugin }
preview-scenario = Сценарий:
preview-scenario-load = Загрузить
preview-scenario-save = Сохранить…
preview-scenario-delete = Удалить
preview-scenario-name = Название сценария
preview-scenario-saved = Сценарий «{ $name }» сохранён в fomod/scenarios
preview-scenario-unresolved = Выборов сценария, не соответствующих ни одной опции этого проекта (переименована или удалена): { $num }
preview-scenario-none = (нет сценария)
issue-unreachable-step = Шаг «{ $step }» никогда не будет показан: его условия видимости проверяют значение флага, которое не задаёт ни одна предыдущая опция
issue-unreachable-option = Опция «{ $plugin }» никогда не может быть выбрана: её шаблоны допустимого типа проверяют значение флага, которое не задаёт ни одна опция
issue-unreachable-cond = Условный набор файлов { $num } никогда не может быть применён: его условия проверяют значение флага, которое не задаёт ни одна опция
size-option = Размер установки: { $size } (файлов: { $num })
size-missing = Отсутствующих источников: { $num }
size-unknown = Размер установки: — (запустите Проверить, чтобы измерить)
menu-nexus-desc = Описание для Nexus…
nexus-title = Описание для Nexus Mods
nexus-format = Формат:
nexus-include-requirements = Требования
nexus-include-options = Опции установки
nexus-include-install = Установка
nexus-include-changelog = Список изменений
nexus-previous = Предыдущая версия…
nexus-previous-none = (нет предыдущей версии: без списка изменений)
nexus-language = Язык:
nexus-language-source = (исходный)
nexus-sec-requirements = Требования
nexus-sec-options = Опции установки
nexus-sec-install = Установка
nexus-sec-changelog = Список изменений
nexus-install-text = Этот мод поставляется с установщиком FOMOD: установите его через менеджер модов (Vortex, Mod Organizer 2) и выберите нужные опции в установщике.
nexus-requires = Требует
nexus-step = Шаг
nexus-added = Добавлено
nexus-removed = Удалено
nexus-changed = Изменено
btn-copy = Копировать
btn-save-as = Сохранить как…
msg-copied = Скопировано в буфер обмена
msg-saved-to = Сохранено в { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Переименовать…
condeditor-rename-exists = Флаг с именем «{ $name }» уже существует
condeditor-renamed = Флаг «{ $from }» переименован в «{ $to }» (вхождений: { $num })
condeditor-delete-uses = Удалить все использования
condeditor-deleted-uses = Флаг «{ $name }» удалён отовсюду (вхождений: { $num })
condeditor-values-set = Задаваемые значения:
condeditor-values-tested = Проверяемые значения:
condeditor-value-never-set = { $value } — проверяется, но нигде не задаётся
condeditor-value-never-tested = { $value } — задаётся, но нигде не проверяется
condeditor-builder = Конструктор условий
condeditor-builder-none = Выберите в главном окне шаг, опцию, условный набор файлов или сведения о моде, чтобы редактировать их условия здесь.
condeditor-builder-pattern = Шаблон:
condeditor-sentence-if = ЕСЛИ
condeditor-sentence-and = И
condeditor-sentence-or = ИЛИ
condeditor-sentence-flag = флаг { "{name}" } = { "{value}" }
condeditor-sentence-file = файл { "{name}" } — { "{value}" }
condeditor-sentence-empty = (нет условия: всегда истинно)
condeditor-sentence-then-visible = ТОГДА шаг показывается
condeditor-sentence-then-type = ТОГДА опция становится { $type }
condeditor-sentence-then-install = ТОГДА файлы устанавливаются
condeditor-sentence-then-module = ТОГДА установщик может быть запущен (проверяется перед его запуском)
issue-flag-value-never-set = Флаг «{ $flag }» проверяется со значением «{ $value }», которое не задаёт ни одна опция
issue-flag-never-used = Флаг «{ $flag }» задаётся, но нигде не проверяется
menu-project-strings = Строки проекта…
strings-title = Строки проекта
strings-search = Поиск по тексту, расположению или ключу…
strings-kind-all = Все
strings-kind-names = Названия
strings-kind-descriptions = Описания
strings-duplicates-only = Только дубликаты
strings-replace-with = Заменить на:
strings-case = Учитывать регистр
strings-whole-word = Слово целиком
strings-replace-current = Заменить
strings-replace-all = Заменить все
strings-replaced = Заменено строк: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Тот же текст, что и:
strings-count = Строк: { $num } · групп дубликатов: { $dups }
strings-col-location = Расположение
strings-col-field = Поле
strings-col-text = Текст

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Содержимое архива…
filter-bethesda-archive = Архивы Bethesda (bsa, ba2)
archive-view-title = Содержимое архива
archive-view-format = Формат:
archive-view-entries = Записей: { $num }
archive-view-size = { $size } в распакованном виде
archive-view-search = Поиск пути…
archive-view-col-path = Путь
archive-view-col-size = Размер
archive-view-col-compressed = Сжат
archive-view-truncated = Показаны только первые { $num } подходящих записей — уточните запрос.
archive-view-error = Не удалось прочитать этот архив: { $error }
archive-view-hint = Просмотреть содержимое этого архива
issue-conflict-archive = Один и тот же ресурс в нескольких архивах: «{ $path }» упакован { $count } ссылками ({ $locs }); какой из них будет использован, решает порядок загрузки архивов в игре.
issue-conflict-archive-loose = Архив против свободного файла: «{ $path }» одновременно упакован в архив и установлен как свободный файл ({ $locs }); свободный файл имеет приоритет над архивным.
preview-in-archive = (в архиве)
preview-archived-size = из них { $size } упаковано в архивы

# --- Project tree: expand / collapse menus
tree-expand = Развернуть
tree-collapse = Свернуть
tree-expand-all = Развернуть всё
tree-expand-selected = Развернуть выбранное
tree-expand-from = Развернуть начиная с выбранного
tree-collapse-all = Свернуть всё
tree-collapse-selected = Свернуть выбранное
tree-collapse-from = Свернуть начиная с выбранного
tree-expand-all-hint = Разворачивает все заголовки
tree-expand-selected-hint = Разворачивает только выбранный заголовок
tree-expand-from-hint = Разворачивает выбранный заголовок и всё, что под ним
tree-collapse-all-hint = Сворачивает все заголовки
tree-collapse-selected-hint = Сворачивает только выбранный заголовок
tree-collapse-from-hint = Сворачивает выбранный заголовок и всё, что под ним

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Добавить группу
btn-remove-group-cond = Удалить группу
dep-type-game = Версия игры
dep-type-fomm = Версия менеджера модов
dep-group-hint = Группа условий, объединённых через И / ИЛИ; группы могут быть вложенными.
condeditor-sentence-game = версия игры ≥ { "{value}" }
condeditor-sentence-fomm = версия менеджера модов ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Уникальные тексты
ftr-uniques-hint = Показывать одну запись на каждый уникальный исходный текст. Перевод этой записи сразу переводит все строки с тем же текстом.
ftr-uniques-synced = Обновлено одинаковых строк: { $num }.
ftr-uniques-group = Строк с этим текстом: { $num }; его перевод применяется ко всем.
