# XIMOD Architect - translation metadata
# @language = spa
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Español
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versión { $version }

# Status messages
status-ready = Listo
msg-save-success = FOMOD guardado correctamente
msg-save-error = Error al guardar el FOMOD
msg-export-success = Archivo de distribución creado ({ $count } archivos): { $path }
msg-export-error = Error al crear el archivo de distribución: { $error }
msg-load-success = FOMOD cargado correctamente
msg-load-error = Error al cargar el FOMOD
msg-merge-success = FOMOD combinado correctamente
msg-merge-error = Error al combinar el FOMOD
msg-no-root-selected = Seleccione primero un directorio raíz
msg-no-fomod-folder = No se encontró la carpeta «fomod». ¿Crear una?
msg-file-outside-root = El archivo está fuera del directorio raíz

# Menu - File
menu-file = Archivo
menu-new = Nuevo
menu-open = Abrir carpeta…
menu-open-file = Abrir archivo…
menu-save = Guardar
menu-recent = Recientes
menu-exit = Salir
menu-merge = Combinar FOMOD…
menu-export = Exportar archivo de distribución...

# Menu - Options
menu-options = Opciones
menu-settings = Configuración
menu-pre-save-script = Script previo al guardado…
menu-post-save-script = Script posterior al guardado…
menu-translation = Traducción...

# Menu - Help
menu-help = Ayuda
menu-about = Acerca de

# Tabs
tab-info = Información del mod
tab-steps = Pasos de instalación
tab-required = Instalaciones obligatorias
tab-conditional = Instalaciones condicionales

# Info Tab
label-workspace = Espacio de trabajo
label-root-dir = Directorio raíz:
label-mod-name = Nombre del mod:
label-author = Autor:
label-version = Versión:
label-game-name = Nombre del juego:
label-category = Categoría:
label-url = URL del sitio web:
label-header-image = Imagen de encabezado:
label-description = Descripción:
placeholder-select-dir = (Seleccione un directorio)
placeholder-select-game = (Seleccione un juego)

# Steps Tab
label-step-name = Nombre del paso:
label-group-name = Nombre del grupo:
label-group-type = Tipo de grupo:
label-plugin-name = Nombre del plugin:
label-plugin-desc = Descripción:
label-plugin-type = Tipo predeterminado:
label-plugin-image = Imagen:
label-visibility = Condiciones de visibilidad
label-operator = Operador:

# Buttons
btn-browse = Examinar…
btn-clear = Borrar
btn-add = Añadir
btn-remove = Quitar
btn-add-step = Nuevo paso
btn-delete-step = Eliminar paso
btn-add-group = Añadir grupo
btn-remove-group = Quitar grupo
btn-add-plugin = Añadir plugin
btn-remove-plugin = Quitar plugin
btn-add-file = Añadir archivo
btn-add-folder = Añadir carpeta
btn-remove-file = Quitar
btn-add-flag = Añadir marca
btn-remove-flag = Quitar marca
btn-add-condition = Añadir condición
btn-remove-condition = Quitar condición
btn-add-dependency = Añadir dependencia
btn-remove-dependency = Quitar dependencia
btn-add-pattern = Nuevo patrón
btn-remove-pattern = Eliminar patrón
btn-save = Guardar
btn-cancel = Cancelar
btn-ok = Aceptar
btn-yes = Sí
btn-no = No

# Condition/Dependency Labels
label-flag-name = Nombre de la marca:
label-flag-value = Valor:
label-condition-type = Tipo:
label-condition-name = Nombre:
label-condition-value = Valor:
label-dep-type = Tipo de dependencia:
label-dep-name = Nombre/archivo:
label-dep-value = Valor/estado:

# Files
label-source = Origen
label-destination = Destino
label-priority = Prioridad
label-file-type = Tipo
label-files = Archivos
label-dependencies = Dependencias

# Settings Dialog
settings-title = Configuración
settings-tab-general = General
settings-tab-recent-files = Archivos recientes
settings-language = Idioma:
settings-theme = Tema:
settings-font-size = Tamaño de fuente:
settings-replace-newlines = Procesar saltos de línea en las descripciones
settings-max-recent = Máx. archivos recientes:
settings-window-width = Ancho de ventana:
settings-window-height = Alto de ventana:
settings-no-recent-files = No hay archivos recientes.

# Status messages for settings
status-settings-saved = Configuración guardada correctamente

# About Dialog
about-title = Acerca de XIMOD Architect
about-description = Una herramienta multiplataforma para crear instaladores FOMOD para mods de juegos de Bethesda.
about-license = Con licencia MIT
about-copyright = © 2024 XIMOD Team
about-credit = Adaptación a Rust de la herramienta original de Wenderer:

# Script Dialog
script-title = Editar script
script-info = Los scripts se ejecutan antes o después de guardar. Puede usar las siguientes macros:
script-macros = Macros disponibles:
macro-modname = $MODNAME$ - Nombre del mod
macro-modauthor = $MODAUTHOR$ - Nombre del autor
macro-modversion = $MODVERSION$ - Versión del mod
macro-modroot = $MODROOT$ - Ruta del directorio raíz
macro-date = $DATE$ - Fecha actual (AAAA-MM-DD)
macro-time = $TIME$ - Hora actual (HH:MM:SS)
macro-random = $RANDOM$ - Número aleatorio

# Plugin Dependencies
label-default-type = Tipo predeterminado:
label-pattern-type = Tipo de patrón:
label-pattern-operator = Operador de patrón:

# Conditional Files
label-pattern = Patrón

# Validation Messages
validation-no-name = El nombre del mod es obligatorio
validation-no-steps = Se necesita al menos un paso o un archivo obligatorio
validation-empty-step = El paso { $num } no tiene nombre
validation-empty-group = El paso { $step }, grupo { $group } no tiene nombre
validation-no-plugins = El paso { $step }, grupo «{ $name }» no tiene plugins

# File States
state-active = Activo
state-inactive = Inactivo
state-missing = Falta

# Confirmation
confirm-title = Confirmación
confirm-delete = ¿Seguro que quiere eliminar este elemento?
confirm-discard = Tiene cambios sin guardar. ¿Descartarlos y continuar?
confirm-unsaved = Tiene cambios sin guardar. ¿Quiere guardar antes de cerrar?
confirm-save-issues = El proyecto presenta los siguientes problemas:
confirm-save-anyway = ¿Guardar de todos modos?

# Errors
error-invalid-xml = Archivo XML no válido
error-parse-failed = No se pudo analizar el FOMOD
error-write-failed = No se pudo escribir el archivo
error-create-dir = No se pudo crear el directorio

# Default names (generated when creating new items)
default-step-name = Paso { $num }
default-group-name = Grupo { $num }
default-plugin-name = Plugin { $num }
pattern-label = Patrón { $num }

# Selection prompts
msg-select-group-first = Seleccione primero un grupo.
msg-select-plugin-edit = Seleccione un plugin para editar.
label-empty = (vacío)
image-no-image = Sin imagen

# File dialog filters
filter-images = Imágenes
filter-xml = XML

# Dependency types
dep-type-flag = Marca
dep-type-file = Archivo

# Status bar
status-modified = Modificado

# Status messages (errors)
msg-settings-save-error = Error al guardar la configuración
msg-script-save-error = Error al guardar el script

# Translation editor
trans-title = Editor de traducción
trans-source-lang = Idioma mostrado:
trans-target-lang = Idioma a traducir:
trans-col-key = Clave
trans-col-source = Etiqueta
trans-col-target = Traducción
trans-saved = Traducción guardada
trans-save-error = Error al guardar la traducción

# XML editor
xml-editor-title = Editor XML
xml-editor-edit = Editar
xml-editor-apply = Aplicar
xml-editor-revert = Cancelar
xml-editor-readonly = Solo lectura
xml-editor-editing = Editando — las pestañas gráficas están bloqueadas
xml-editor-error = Error:
xml-editor-applied = Cambios XML aplicados
xml-editor-wellformed = XML bien formado
xml-editor-error-at = Línea { $line }, columna { $col }: { $msg }

# Country / flag picker
settings-country-name = Nombre del país:
settings-pick-country = Haz clic para elegir tu país
flags-title = Elige un país
flags-filter = Filtro:
flags-none = No se encontró ninguna bandera

# Translation editor: country & font
trans-endonym = Endónimo del país:
trans-font = Fuente:
trans-no-font = (ninguna)
trans-browse = Examinar…
trans-google-fonts = Google Fonts
trans-pick-country = Haz clic para elegir el país
trans-font-outside = La fuente debe instalarse primero en assets/fonts.
trans-font-dir-missing = No se encontró la carpeta assets/fonts.

# Translation submission
trans-lang-endonym = Endónimo del idioma:
trans-author = Autor:
trans-submit = Enviar…
trans-submit-hint = Crear un zip y abrir un correo electrónico prerrellenado
trans-data-updated = Datos de referencia actualizados (Languages.json / Countries.json)
trans-package-ready = Archivo listo:
trans-package-error = No se pudo crear el archivo:

# ISO 639-3 requirement
trans-lang-not-iso = La traducción solo es posible para un idioma con código ISO 639-3.

# FOMOD installer preview
menu-preview = Previsualizar instalador…
preview-title = Vista previa del instalador FOMOD
preview-refresh = Actualizar
preview-assumptions = Supuestos de archivos
preview-details = Detalles
preview-back = Atrás
preview-next = Siguiente
preview-install = Instalar
preview-close = Cerrar
preview-restart = Reiniciar
preview-summary-title = Archivos que se instalarán
preview-empty = No se instalaría ningún archivo.
preview-none-option = (ninguna)
preview-invalid = Completa las opciones requeridas para continuar.
preview-no-steps = No hay ningún paso visible; consulta el resumen de instalación.
preview-select-hint = Selecciona una opción para ver su descripción.
preview-col-source = Origen
preview-col-dest = Destino
preview-col-priority = Prioridad
preview-sel-exactlyone = Elige exactamente una opción.
preview-sel-atmostone = Elige como máximo una opción.
preview-sel-any = Elige cualquier número de opciones.
preview-sel-all = Todas las opciones se instalan.
preview-sel-atleastone = Elige al menos una opción.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validar FOMOD
validate-report-title = Validación del FOMOD
validate-ok = No se encontró ningún problema. El FOMOD cumple con el esquema.
xml-editor-schema-ok = Cumple con el esquema ModConfig 5.0.
xml-editor-schema-issues = Problemas del esquema:
schema-line-col = Línea { $line }, col. { $col }: { $msg }
schema-wrong-root = Raíz inesperada "{ $found }" (se esperaba "{ $expected }").
schema-unknown = Elemento inesperado "{ $element }" en "{ $parent }".
schema-missing = "{ $parent }" debe contener "{ $child }".
schema-needs-one = "{ $parent }" debe contener al menos un "{ $child }".
schema-too-many = "{ $child }" solo puede aparecer una vez en "{ $parent }".
schema-missing-attr = El atributo "{ $attr }" es obligatorio en "{ $element }".
schema-bad-enum = Valor no válido "{ $value }" para { $element }/@{ $attr } (se esperaba: { $allowed }).
schema-choose-one = "{ $parent }" debe contener exactamente uno de: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Mover antes
reorder-after = Mover después

# Country / language database explorer (Properties)
menu-properties = Propiedades…
prop-title = Base de datos de países / idiomas
prop-tab-countries = Países
prop-tab-languages = Idiomas
prop-filter = Filtro:
prop-official-langs = Idiomas oficiales
prop-spoken-langs = Idiomas hablados
prop-endonym = Endónimo del país
prop-font = Fuente
prop-spoken-in = Hablado en
prop-select-country = Selecciona un país para ver sus detalles.
prop-select-lang = Selecciona un idioma para ver sus detalles.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Abrir la página de Nexus Mods del juego
