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
menu-export = Exportar archivo de distribución…
# Menu - Options
menu-options = Opciones
menu-settings = Configuración…
menu-pre-save-script = Script previo al guardado…
menu-post-save-script = Script posterior al guardado…
menu-translation = Traducir la interfaz…
# Menu - Help
menu-help = Ayuda
menu-check-updates = Buscar actualizaciones…
menu-about = Acerca de

# Update check
update-checking = Buscando actualizaciones…
update-up-to-date = XIMOD Architect está actualizado.
update-check-failed = No se pudieron comprobar las actualizaciones. Inténtelo más tarde.
update-available-status = La versión { $version } está disponible.
update-banner-text = XIMOD Architect { $version } está disponible.
update-download = Descargar:
update-skip = Omitir esta versión
update-later = Más tarde

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
label-plugin-name = Nombre de la opción:
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
btn-add-plugin = Añadir opción
btn-remove-plugin = Quitar opción
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Destino para todo el grupo
label-page-dest = Destino de instalación (toda la página)
btn-apply-group-dest = Aplicar a todas las opciones de este grupo
btn-apply-page-dest = Aplicar a todas las opciones de esta página
group-dest-hint = Establece un único destino de instalación para cada archivo de cada opción de este grupo.
page-dest-hint = Establece un único destino de instalación para cada archivo de cada opción de esta página (todos los grupos).
bulk-dest-nofiles = Aún no hay archivos que actualizar — añade primero archivos a las opciones.
status-dest-applied = Destino aplicado a { $num } archivo(s).
preview-hidden-steps = { $num } paso(s) ocultos por las selecciones actuales.
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
settings-check-updates = Buscar actualizaciones al iniciar
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
label-plugin-dependencies = Dependencias de la opción
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
validation-no-plugins = El paso { $step }, grupo «{ $name }» no tiene opciones

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
default-plugin-name = Opción { $num }
pattern-label = Patrón { $num }

# Selection prompts
msg-select-group-first = Seleccione primero un grupo.
msg-select-plugin-edit = Seleccione una opción para editar.
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

# Referenced-file verification (V2)
verify-no-root = Verificación de archivos omitida: no hay carpeta raíz definida
loc-header = imagen de cabecera
loc-required = archivos obligatorios
loc-conditional = conjunto condicional { $num }
loc-plugin = paso { $step }, grupo { $group }, opción «{ $plugin }»
verify-missing-file = Archivo ausente: { $path } ({ $loc })
verify-missing-folder = Carpeta ausente: { $path } ({ $loc })
verify-missing-image = Imagen ausente: { $path } ({ $loc })
verify-absolute = Ruta absoluta (no portátil): { $path } ({ $loc })
verify-outside = La ruta sale de la carpeta raíz: { $path } ({ $loc })
verify-orphan = Archivo huérfano (ninguna opción lo referencia): { $path }
conflict-certain = Conflicto de destino: «{ $path }» lo escriben { $count } opciones ({ $locs }): se sobrescriben entre sí.
conflict-potential = Posible conflicto de destino: «{ $path }» es destino de { $count } referencias ({ $locs }): la sobrescritura depende de la selección/condiciones.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Cerrar FOMOD
menu-close-all-fomods = Cerrar todos los FOMOD
tab-untitled = (sin título)
msg-drop-not-fomod = El elemento soltado no es un FOMOD (no se encontró la carpeta «fomod»)
exit-title = Cambios sin guardar
exit-unsaved = Un FOMOD no se ha guardado. ¿Desea guardarlo?
tab-close-hint = Cerrar este FOMOD
menu-new-from-folder = Nuevo desde carpeta…
menu-templates = Plantillas…
templates-title = Plantillas reutilizables
templates-empty = Aún no hay plantillas guardadas. Guarde arriba el paso seleccionado para crear una.
templates-insert = Insertar
templates-save-step = Guardar el paso seleccionado
templates-name-hint = Nombre de la plantilla (opcional)
msg-wizard-success = Esqueleto creado desde la carpeta: { $num } opción(es).
msg-wizard-error = Error: { $error }
msg-template-saved = Plantilla guardada: { $name }
msg-template-inserted = Plantilla insertada en el proyecto.
msg-template-no-step = Seleccione primero un paso para guardarlo como plantilla.
msg-template-no-dir = No se pudo localizar la carpeta de plantillas.
msg-drop-assigned = { $added } fuente(s) añadida(s) a la opción ({ $rejected } fuera de la raíz ignorada(s)).
menu-compare = Comparar con…
compare-title = Comparación de FOMOD
compare-none = Sin diferencias.
btn-optimize-image = Optimizar imagen
msg-image-optimized = Imagen de encabezado optimizada.
msg-image-ok = La imagen de encabezado ya está dentro de los límites.
msg-no-header-image = No hay imagen de encabezado para optimizar.
verify-image-large = Imagen demasiado grande ({ $width }×{ $height }): { $path }
verify-image-format = Formato de imagen no admitido (.{ $ext }): { $path }
verify-image-unreadable = Imagen ilegible: { $path }
menu-condition-editor = Editor de condiciones…
condeditor-title = Editor de condiciones
condeditor-set-by = Establecido por:
condeditor-used-by = Usado por:
condeditor-filedeps = Dependencias de archivos
condeditor-empty = No hay marcadores ni dependencias en este proyecto.
condeditor-orphan-set = establecido pero nunca usado
condeditor-orphan-used = usado pero nunca establecido
msg-img-optimized = Imagen optimizada.
msg-img-ok = La imagen ya está dentro de los límites.
msg-img-none = No hay imagen para optimizar.
msg-crash-recovery = La sesión anterior terminó de forma inesperada. Se guardó una copia de seguridad de su proyecto en { $path }
export-progress-title = Creando el archivo de distribución…
export-progress-files = { $done } / { $total } archivos
msg-export-cancelled = Exportación cancelada; el archivo parcial se eliminó.
verify-running = Comprobando los archivos en el disco…
verify-stale = Nota: el proyecto cambió mientras se comprobaban los archivos; vuelva a ejecutar la validación.
prop-col-name = Nombre
menu-save-as = Guardar como…
menu-project = Proyecto
menu-tools = Herramientas
menu-manual = Manual de usuario
msg-manual-missing = No se encontró el manual de usuario (PDF) junto a la aplicación.
toolbar-new = Nuevo
toolbar-open = Abrir
toolbar-save = Guardar
toolbar-validate = Validar
toolbar-preview = Vista previa
toolbar-export = Exportar
dialog-choose-root = Elija la carpeta raíz del mod
exit-unsaved-docs = Sin guardar: { $names }
status-summary = { $steps } pasos · { $options } opciones
section-groups = Grupos
section-options = Opciones
section-flags = Indicadores de condición
section-files = Archivos a instalar
hint-group-type = Cómo el instalador permite al usuario elegir opciones en este grupo.
hint-default-type = Cómo se ofrece la opción cuando ninguno de sus patrones de dependencia coincide: obligatoria, opcional, recomendada, no utilizable…
hint-operator = Todas las condiciones deben cumplirse (Y), o basta con una (O).
hint-flags = Los indicadores son valores con nombre que esta opción establece al elegirse. Otros pasos y opciones pueden comprobarlos para mostrarse, ocultarse o volverse obligatorios.
hint-plugin-dependencies = Patrones que cambian el tipo de la opción según indicadores o archivos presentes en el juego: por ejemplo «Obligatoria» cuando otro mod está instalado.
hint-files = Archivos y carpetas copiados a la carpeta Data del juego cuando se elige esta opción. El destino es relativo a Data; en caso de conflicto gana la prioridad más alta.
hint-visibility = Condiciones que deben cumplirse para que este paso se muestre. Déjelo vacío para mostrarlo siempre.
seltype-exactly-one = Exactamente una (obligatoria)
seltype-at-most-one = Como máximo una
seltype-any = Cualquier número
seltype-all = Todas (sin elección)
seltype-at-least-one = Al menos una
plugtype-required = Obligatoria
plugtype-optional = Opcional
plugtype-recommended = Recomendada
plugtype-not-usable = No utilizable
plugtype-could-be-usable = Quizá utilizable
plugtype-required-hint = Siempre instalada; el usuario no puede desmarcarla.
plugtype-optional-hint = Ofrecida sin marcar; el usuario decide.
plugtype-recommended-hint = Ofrecida marcada; el usuario puede desmarcarla.
plugtype-not-usable-hint = Se muestra en gris y no puede seleccionarse.
plugtype-could-be-usable-hint = Seleccionable, pero el instalador avisa de que puede no funcionar.
op-and = Todas las condiciones (Y)
op-or = Cualquier condición (O)
theme-dark = Oscuro
theme-light = Claro
theme-system = Seguir al sistema
condeditor-setter-loc = Paso { "{step}" } / Grupo { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Patrón de «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Visibilidad del paso { "{step}" }
condeditor-cond-set = Conjunto condicional { "{num}" }
condeditor-needs = { "{ctx}" } (requiere = { "{value}" })
condeditor-file-dep = { "{ctx}" }: archivo «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = Traducir un FOMOD…
ftr-title = Traducir un FOMOD
ftr-open-folder = Abrir una carpeta de mod…
ftr-from-active = Desde el proyecto activo
ftr-from-active-hint = Traduce el FOMOD del proyecto abierto en la ventana principal (primero debe guardarse).
ftr-no-fomod = Ningún FOMOD cargado.
ftr-encoding = Codificación de los archivos originales; los archivos traducidos se escriben con la misma codificación.
ftr-source-lang = De
ftr-target-lang = a
ftr-lang-locked = (los idiomas quedan fijados una vez cargado un FOMOD)
ftr-translator = Traductor:
ftr-save = Guardar la traducción
ftr-export = Exportar los archivos traducidos
ftr-export-sibling = A una carpeta fomod_<idioma>
ftr-export-sibling-hint = Escribe info.xml y ModuleConfig.xml traducidos junto a la carpeta fomod original; los archivos originales no se tocan.
ftr-export-inplace = Sobre los archivos originales
ftr-export-inplace-hint = Reemplaza fomod/info.xml y fomod/ModuleConfig.xml tras hacer una copia .bak con marca de tiempo de cada uno.
ftr-force-explicit-order = Conservar el orden original
ftr-warn-order = Las listas ordenadas por nombre (order="Ascending") serían reordenadas según los nombres traducidos por el gestor de mods. Esta opción fuerza order="Explicit" para que las opciones conserven su orden actual.
ftr-update = Actualizar desde la carpeta
ftr-update-hint = Vuelve a leer el FOMOD del disco y fusiona la traducción con él: se indican las cadenas nuevas, modificadas y eliminadas.
ftr-preview-translated = Vista previa traducida
ftr-progress = { $done } / { $total } traducidas
ftr-filter-all = Todas
ftr-filter-untranslated = Sin traducir
ftr-filter-review = Por revisar
ftr-filter-issues = Con problemas
ftr-filter-locked = Bloqueadas
ftr-type-all = Todos los campos
ftr-type-names = Nombres
ftr-type-descriptions = Descripciones
ftr-type-meta = Información del mod
ftr-search-hint = Buscar en el origen, la traducción o el contexto…
ftr-next-untranslated = Siguiente sin traducir
ftr-show-whitespace = Mostrar espacios y saltos de línea
ftr-discard-question = La traducción actual tiene cambios sin guardar. ¿Descartarlos y cargar el otro FOMOD?
ftr-discard-yes = Descartar
ftr-unsaved-close = La traducción tiene cambios sin guardar.
ftr-col-num = N.º
ftr-col-status = { "" }
ftr-col-context = Contexto
ftr-col-source = Origen
ftr-col-target = Traducción
ftr-col-issues = { "" }
ftr-empty-hint = Abra una carpeta de mod o cargue el proyecto activo para listar sus cadenas traducibles.
ftr-empty-filter = Ninguna cadena coincide con el filtro actual.
ftr-select-row = Seleccione una fila para editar su traducción.
ftr-copy-source = Copiar el origen
ftr-clear-target = Borrar
ftr-lock = No traducir
ftr-lock-hint = Las cadenas bloqueadas se escriben sin cambios (autor, sitio web, nombres propios…).
ftr-note = Nota:
ftr-status-untranslated = Sin traducir
ftr-status-translated = Traducida
ftr-status-auto = Rellenada automáticamente — por revisar
ftr-status-fuzzy = El texto de origen ha cambiado desde la traducción — por revisar
ftr-status-obsolete = Ya no existe en el FOMOD
ftr-status-locked = Bloqueada (se escribe sin cambios)
ftr-field-info-name = Nombre del mod (info.xml)
ftr-field-module-name = Título del instalador (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Sitio web
ftr-field-description = Descripción del mod
ftr-field-step = Nombre del paso
ftr-field-group = Nombre del grupo
ftr-field-plugin = Nombre de la opción
ftr-field-plugin-desc = Descripción de la opción
ftr-issue-empty = Traducción vacía
ftr-issue-whitespace = La traducción solo contiene espacios
ftr-issue-edge-whitespace = Los espacios al principio o al final difieren del origen
ftr-issue-token = Los tokens protegidos difieren — faltan: { $missing } ; sobran: { $extra }
ftr-issue-newline-name = Un nombre no puede contener un salto de línea
ftr-issue-control = Contiene caracteres que XML no puede almacenar
ftr-issue-length = Longitud inusual respecto al origen (×{ $ratio })
ftr-issue-identical = Idéntica al origen
ftr-issue-duplicate = El mismo texto de origen está traducido de otra forma en { $key }
ftr-issue-cdata = La secuencia ]]> no está permitida aquí
ftr-load-error = No se pudo cargar el FOMOD: { $error }
ftr-extracted = Se encontraron { $num } cadenas traducibles.
ftr-sidecar-found = Traducción existente cargada y fusionada: { $new } nuevas, { $changed } modificadas, { $removed } eliminadas.
ftr-saved = Traducción guardada en { $path }
ftr-save-error = No se pudo guardar la traducción: { $error }
ftr-save-first = Guarde primero el proyecto y luego tradúzcalo.
ftr-export-success = { $count } cadenas escritas en { $path }
ftr-export-error = Error de exportación: { $error }
ftr-export-blocked = Hay que corregir { $num } problemas bloqueantes antes de exportar.
ftr-export-stale = Se omitieron { $num } cadenas porque el FOMOD ha cambiado; use «Actualizar desde la carpeta».
ftr-update-report = Actualizado: { $new } nuevas, { $changed } modificadas, { $moved } movidas, { $removed } eliminadas, { $unchanged } sin cambios.
menu-edit = Editar
menu-undo = Deshacer
menu-redo = Rehacer
tree-title = Proyecto
tree-mod-info = Información del mod
tree-steps = Pasos de instalación
tree-required = Archivos obligatorios
tree-conditional = Instalaciones condicionales
tree-empty-steps = Aún no hay pasos — haga clic en + para añadir uno.
tree-duplicate = Duplicar
tree-delete = Eliminar
tree-save-template = Guardar como plantilla…
tree-drop-hint = Suelte aquí para mover
cond-set-label = Conjunto condicional { $num }
inspector-empty = Seleccione un elemento en el árbol del proyecto o añada un paso para empezar.
count-options = { $num } opciones
count-files = { $num } archivos
msg-deleted-undo = Eliminado. Use Deshacer (Ctrl+Z) para restaurarlo.
problems-title = Problemas
problems-errors = { $num } errores
problems-warnings = { $num } advertencias
btn-close = Cerrar
ftr-export-package = Como paquete de traducción (archivo comprimido)
ftr-export-package-hint = Crea un .zip o un .7z listo para subir: info.xml y ModuleConfig.xml traducidos más un README (solo parche), o el mod entero con los archivos traducidos (completo).
ftr-package-full = Mod completo
ftr-package-full-hint = Incluye en el archivo comprimido todos los archivos del mod, no solo los dos XML traducidos. Asegúrese de que el autor permite la redistribución.
ftr-package-name-template = Nombre:
ftr-readme-patch = Este archivo comprimido contiene la traducción ({ $langname }) del instalador de «{ $name }» (fomod/info.xml y fomod/ModuleConfig.xml). Instálelo sobre el mod original, o deje que su gestor de mods lo fusione, para que los archivos traducidos sustituyan a los originales. Solo cambian los textos del instalador; los archivos del mod en sí no están incluidos. Hecho con XIMOD Architect.
ftr-readme-full = Este archivo comprimido contiene «{ $name }» con su instalador traducido ({ $langname }; fomod/info.xml y fomod/ModuleConfig.xml). Instálelo como el mod original. Solo se han modificado los textos del instalador. Hecho con XIMOD Architect.
ftr-apply-memory = Rellenar desde la memoria
ftr-memory-size = Memoria de traducción: { $num } entradas para este par de idiomas. Cada traducción guardada se añade a ella.
ftr-memory-applied = { $num } cadenas rellenadas desde la memoria de traducción (marcadas «por revisar»).
ftr-memory-suggestion = La memoria sugiere:
ftr-use-suggestion = Usar
ftr-propagate = Propagar a las idénticas
ftr-propagate-hint = Copia esta traducción a todas las demás cadenas con el mismo texto de origen que siguen sin traducir.
ftr-propagated = { $num } cadenas idénticas rellenadas.
ftr-csv-export = Exportar CSV…
ftr-csv-import = Importar CSV…
ftr-csv-imported = { $num } cadenas actualizadas desde el archivo CSV.
ftr-csv-error = Error de CSV: { $error }
ftr-glossary = Glosario
ftr-glossary-source = Término
ftr-glossary-target = Traducción
ftr-glossary-case = Mayúsculas/minúsculas
ftr-glossary-dnt = Conservar
ftr-glossary-add = Añadir término
ftr-issue-glossary = Glosario: «{ $term }» no está traducido como se esperaba

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Abrir archivo comprimido…
filter-archive = Archivos comprimidos de mod (zip, 7z)
msg-archive-opened = Archivo comprimido abierto ({ $num } archivos extraídos): { $path }
msg-archive-reused = El archivo comprimido ya estaba extraído; se reutiliza { $path }
msg-archive-unsupported = El formato de archivo comprimido «.{ $ext }» no es compatible; extráigalo primero con 7-Zip (solo se pueden abrir .zip y .7z).
msg-archive-error = Error al abrir el archivo comprimido: { $error }
msg-archive-no-fomod = No se encontró ninguna carpeta «fomod» en el archivo comprimido ({ $path })
msg-archive-extracting = Extrayendo el archivo comprimido…
ftr-open-archive = Abrir un archivo comprimido de mod…
ftr-package-full-partial = El mod se abrió desde un archivo comprimido que solo contiene su carpeta fomod; los paquetes completos necesitan el mod extraído.
info-module-deps = Requisitos del mod
info-module-deps-hint = Archivos o indicadores que todo el mod requiere antes de ejecutar el instalador (moduleDependencies). Déjelo vacío si no hay ninguno.
info-header-advanced = Encabezado avanzado
info-title-position = Posición del título
info-title-colour = Color del título
info-title-colour-hint = Se esperan: seis dígitos hexadecimales (RRGGBB)
info-image-show = Mostrar imagen del encabezado
info-image-fade = Fundir imagen del encabezado
info-image-height = Altura de la imagen del encabezado
info-attr-default = (predeterminado)
file-always-install = Siempre
file-always-install-hint = Instalar siempre este archivo, incluso cuando la opción no está seleccionada (alwaysInstall).
file-install-if-usable = Si es usable
file-install-if-usable-hint = Instalar este archivo siempre que la opción sea usable, incluso cuando no está seleccionada (installIfUsable).
msg-import-lossy = Este FOMOD contiene { $num } construcciones que XIMOD no puede editar; se descartarán al guardar el proyecto.
fidelity-nested-deps = Grupo de dependencias anidado en { $context } (solo se admite un nivel)
fidelity-game-dep = Requisito de versión del juego { $version } en { $context }
fidelity-fomm-dep = Requisito de versión del gestor de mods { $version } en { $context }
fidelity-unknown = El elemento «{ $element }» en «{ $parent }» no es compatible ({ $context })
loc-module = los requisitos del mod
loc-step = el paso { $step } «{ $name }»
loc-installer = el instalador

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Restaurar una copia de seguridad…
backups-title = Restaurar una copia de seguridad
backups-empty = Este proyecto aún no tiene ninguna copia de seguridad. Se crea una cada vez que el proyecto se guarda sobre una versión anterior.
backups-changes = { $num } cambio(s) respecto al proyecto actual
btn-compare = Comparar
btn-restore = Restaurar
btn-delete-backups = Eliminar todas las copias de seguridad
btn-delete-backups-confirm = Haga clic de nuevo para eliminar todas las copias de seguridad
msg-backup-restored = Copia de seguridad del { $time } restaurada en el editor (aún no guardada; Deshacer la revierte)
msg-backups-deleted = { $num } copia(s) de seguridad eliminada(s)
settings-backup-count = Copias de seguridad a conservar:
settings-backup-count-hint = Número de versiones anteriores del XML del FOMOD conservadas en fomod/backups al guardar (0 = sin copias de seguridad).
settings-autosave-minutes = Guardar automáticamente una copia de recuperación cada (minutos):
settings-autosave-minutes-hint = Con este intervalo se escribe en la carpeta de configuración una copia de recuperación de cada proyecto modificado; solo se ofrece en el siguiente inicio tras un cierre anómalo (0 = desactivado).
settings-auto-masters = Añadir los masters de un plugin como condiciones
settings-auto-masters-hint = Cuando se añade un plugin (.esp/.esm/.esl) a una opción, los masters que requiere y que ni el juego ni este mod proporcionan se convierten en condiciones de archivo «Active» de la opción.
msg-author-from-plugin = Autor rellenado desde la cabecera del plugin: { $author }
msg-masters-added = { $num } master(s) de { $plugin } añadido(s) como condición(es) de archivo
issue-missing-master = { $plugin } requiere { $master }, que no está en este mod ni se ha declarado como dependencia
issue-esl-mismatch-flag = { $plugin } tiene la extensión .esl pero su marca light (ESL) no está activada
issue-esl-eligible = { $plugin } podría marcarse como light ({ $num } registros nuevos, límite { $limit })
issue-esl-too-big = { $plugin } está marcado como light pero no cumple las reglas de los plugins light ({ $num } registros nuevos, límite { $limit }, o un FormID fuera del rango permitido)
menu-plugin-report = Informe de plugins…
plugins-title = Informe de plugins
plugins-file = Archivo
plugins-kind = Tipo
plugins-light = Marca light
plugins-masters = Masters
plugins-new-records = Registros nuevos / límite
plugins-eligible = Apto para light
plugins-empty = Este proyecto no instala ningún archivo de plugin (.esp, .esm o .esl).
plugins-unreadable = ilegible

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Árbol final de archivos
preview-total-size = Tamaño total de la instalación: { $size }
preview-tree-truncated = El árbol está truncado: demasiados archivos para expandir (los tamaños anteriores son parciales).
preview-overwritten-by = Sobrescrito por { $plugin }
preview-scenario = Escenario:
preview-scenario-load = Cargar
preview-scenario-save = Guardar…
preview-scenario-delete = Eliminar
preview-scenario-name = Nombre del escenario
preview-scenario-saved = Escenario «{ $name }» guardado en fomod/scenarios
preview-scenario-unresolved = { $num } selección(es) del escenario no coincide(n) con ninguna opción de este proyecto (renombrada o eliminada)
preview-scenario-none = (sin escenario)
issue-unreachable-step = El paso «{ $step }» nunca puede mostrarse: sus condiciones de visibilidad comprueban un valor de marca que ninguna opción anterior establece
issue-unreachable-option = La opción «{ $plugin }» nunca puede seleccionarse: sus patrones de tipo utilizable comprueban un valor de marca que ninguna opción establece
issue-unreachable-cond = El conjunto condicional de archivos { $num } nunca puede aplicarse: sus condiciones comprueban un valor de marca que ninguna opción establece
size-option = Tamaño de la instalación: { $size } ({ $num } archivo(s))
size-missing = { $num } origen(es) faltante(s)
size-unknown = Tamaño de la instalación: — (ejecuta Validar para medirlo)
menu-nexus-desc = Descripción para Nexus…
nexus-title = Descripción para Nexus Mods
nexus-format = Formato:
nexus-include-requirements = Requisitos
nexus-include-options = Opciones de instalación
nexus-include-install = Instalación
nexus-include-changelog = Registro de cambios
nexus-previous = Versión anterior…
nexus-previous-none = (sin versión anterior: sin registro de cambios)
nexus-language = Idioma:
nexus-language-source = (origen)
nexus-sec-requirements = Requisitos
nexus-sec-options = Opciones de instalación
nexus-sec-install = Instalación
nexus-sec-changelog = Registro de cambios
nexus-install-text = Este mod incluye un instalador FOMOD: instálalo con un gestor de mods (Vortex, Mod Organizer 2) y elige tus opciones en el instalador.
nexus-requires = Requiere
nexus-step = Paso
nexus-added = Añadido
nexus-removed = Eliminado
nexus-changed = Modificado
btn-copy = Copiar
btn-save-as = Guardar como…
msg-copied = Copiado al portapapeles
msg-saved-to = Guardado en { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Renombrar…
condeditor-rename-exists = Ya existe una marca llamada «{ $name }»
condeditor-renamed = Marca «{ $from }» renombrada a «{ $to }» ({ $num } aparición(es))
condeditor-delete-uses = Eliminar todos los usos
condeditor-deleted-uses = Marca «{ $name }» eliminada en todas partes ({ $num } aparición(es))
condeditor-values-set = Valores establecidos:
condeditor-values-tested = Valores comprobados:
condeditor-value-never-set = { $value } — comprobado pero nunca establecido
condeditor-value-never-tested = { $value } — establecido pero nunca comprobado
condeditor-builder = Constructor de condiciones
condeditor-builder-none = Seleccione un paso, una opción, un conjunto condicional de archivos o la información del mod en la ventana principal para editar aquí sus condiciones.
condeditor-builder-pattern = Patrón:
condeditor-sentence-if = SI
condeditor-sentence-and = Y
condeditor-sentence-or = O
condeditor-sentence-flag = la marca { "{name}" } = { "{value}" }
condeditor-sentence-file = el archivo { "{name}" } está { "{value}" }
condeditor-sentence-empty = (sin condición: siempre verdadero)
condeditor-sentence-then-visible = ENTONCES se muestra el paso
condeditor-sentence-then-type = ENTONCES la opción pasa a ser { $type }
condeditor-sentence-then-install = ENTONCES se instalan los archivos
condeditor-sentence-then-module = ENTONCES el instalador puede ejecutarse (se comprueba antes de iniciarse)
issue-flag-value-never-set = La marca «{ $flag }» se comprueba con el valor «{ $value }», que ninguna opción establece
issue-flag-never-used = La marca «{ $flag }» se establece pero nunca se comprueba
menu-project-strings = Cadenas del proyecto…
strings-title = Cadenas del proyecto
strings-search = Buscar texto, ubicación o clave…
strings-kind-all = Todas
strings-kind-names = Nombres
strings-kind-descriptions = Descripciones
strings-duplicates-only = Solo duplicados
strings-replace-with = Reemplazar por:
strings-case = Coincidir mayúsculas/minúsculas
strings-whole-word = Palabra completa
strings-replace-current = Reemplazar
strings-replace-all = Reemplazar todo
strings-replaced = { $num } cadena(s) reemplazada(s)
strings-dup-badge = ×{ $num }
strings-dup-hover = Mismo texto que:
strings-count = { $num } cadena(s) · { $dups } grupo(s) de duplicados
strings-col-location = Ubicación
strings-col-field = Campo
strings-col-text = Texto

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Contenido de un archivo comprimido…
filter-bethesda-archive = Archivos comprimidos Bethesda (bsa, ba2)
archive-view-title = Contenido del archivo comprimido
archive-view-format = Formato:
archive-view-entries = { $num } entradas
archive-view-size = { $size } descomprimidos
archive-view-search = Buscar una ruta…
archive-view-col-path = Ruta
archive-view-col-size = Tamaño
archive-view-col-compressed = Comprimido
archive-view-truncated = Solo se muestran las primeras { $num } entradas coincidentes: acote la búsqueda.
archive-view-error = No se puede leer este archivo comprimido: { $error }
archive-view-hint = Ver el contenido de este archivo comprimido
issue-conflict-archive = Mismo recurso en varios archivos comprimidos: «{ $path }» está empaquetado por { $count } referencias ({ $locs }): el orden de carga de archivos comprimidos del juego decide cuál se usa.
issue-conflict-archive-loose = Archivo comprimido frente a archivo suelto: «{ $path }» está empaquetado en un archivo comprimido y a la vez instalado como archivo suelto ({ $locs }): el archivo suelto prevalece sobre el empaquetado.
preview-in-archive = (en archivo comprimido)
preview-archived-size = de los cuales { $size } empaquetados en archivos comprimidos

# --- Project tree: expand / collapse menus
tree-expand = Expandir
tree-collapse = Contraer
tree-expand-all = Expandir todo
tree-expand-selected = Expandir selección
tree-expand-from = Expandir desde la selección
tree-collapse-all = Contraer todo
tree-collapse-selected = Contraer selección
tree-collapse-from = Contraer desde la selección
tree-expand-all-hint = Expande todos los títulos
tree-expand-selected-hint = Expande solo el título seleccionado
tree-expand-from-hint = Expande el título seleccionado y todo lo que contiene
tree-collapse-all-hint = Contrae todos los títulos
tree-collapse-selected-hint = Contrae solo el título seleccionado
tree-collapse-from-hint = Contrae el título seleccionado y todo lo que contiene

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Añadir grupo
btn-remove-group-cond = Quitar grupo
dep-type-game = Versión del juego
dep-type-fomm = Versión del gestor de mods
dep-group-hint = Un grupo de condiciones combinadas con Y / O; los grupos pueden anidarse.
condeditor-sentence-game = la versión del juego ≥ { "{value}" }
condeditor-sentence-fomm = la versión del gestor de mods ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Textos únicos
ftr-uniques-hint = Mostrar una fila por cada texto de origen distinto. Traducir esa fila traduce de una vez todas las cadenas con el mismo texto.
ftr-uniques-synced = { $num } cadenas idénticas actualizadas.
ftr-uniques-group = { $num } cadenas comparten este texto; su traducción se aplica a todas.
