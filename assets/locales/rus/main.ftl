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
menu-open = Открыть папку...
menu-open-file = Открыть файл...
menu-save = Сохранить
menu-recent = Недавние
menu-exit = Выход
menu-merge = Объединить FOMOD...
menu-export = Экспортировать архив дистрибутива...

# Menu - Options
menu-options = Параметры
menu-settings = Настройки
menu-pre-save-script = Скрипт перед сохранением...
menu-post-save-script = Скрипт после сохранения...
menu-translation = Перевод...

# Menu - Help
menu-help = Справка
menu-about = О программе

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
label-plugin-name = Название плагина:
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
btn-add-plugin = Добавить плагин
btn-remove-plugin = Удалить плагин
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
validation-no-plugins = Шаг { $step }, группа "{ $name }" не имеет плагинов

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
default-plugin-name = Плагин { $num }
pattern-label = Шаблон { $num }

# Selection prompts
msg-select-group-first = Сначала выберите группу.
msg-select-plugin-edit = Выберите плагин для редактирования.
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
