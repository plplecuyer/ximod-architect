# XIMOD Architect - translation metadata
# @language = hun
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Magyar
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Verzió { $version }

# Status messages
status-ready = Kész
msg-save-success = A FOMOD sikeresen mentve
msg-save-error = Hiba a FOMOD mentésekor
msg-export-success = Terjesztési archívum létrehozva ({ $count } fájl): { $path }
msg-export-error = Hiba a terjesztési archívum létrehozásakor: { $error }
msg-load-success = A FOMOD sikeresen betöltve
msg-load-error = Hiba a FOMOD betöltésekor
msg-merge-success = A FOMOD sikeresen egyesítve
msg-merge-error = Hiba a FOMOD egyesítésekor
msg-no-root-selected = Először válasszon gyökérkönyvtárat
msg-no-fomod-folder = Nem található „fomod” mappa. Létrehozza?
msg-file-outside-root = A fájl a gyökérkönyvtáron kívül van

# Menu - File
menu-file = Fájl
menu-new = Új
menu-open = Mappa megnyitása…
menu-open-file = Fájl megnyitása…
menu-save = Mentés
menu-recent = Legutóbbi
menu-exit = Kilépés
menu-merge = FOMOD egyesítése…
menu-export = Terjesztési archívum exportálása…

# Menu - Options
menu-options = Beállítások
menu-settings = Beállítások
menu-pre-save-script = Mentés előtti parancsfájl…
menu-post-save-script = Mentés utáni parancsfájl…
menu-translation = Fordítás…

# Menu - Help
menu-help = Súgó
menu-about = Névjegy

# Tabs
tab-info = Mod-információ
tab-steps = Telepítési lépések
tab-required = Kötelező telepítések
tab-conditional = Feltételes telepítések

# Info Tab
label-workspace = Munkaterület
label-root-dir = Gyökérkönyvtár:
label-mod-name = Mod neve:
label-author = Szerző:
label-version = Verzió:
label-game-name = Játék neve:
label-category = Kategória:
label-url = Webhely URL-címe:
label-header-image = Fejléckép:
label-description = Leírás:
placeholder-select-dir = (Válasszon könyvtárat)
placeholder-select-game = (Válasszon játékot)

# Steps Tab
label-step-name = Lépés neve:
label-group-name = Csoport neve:
label-group-type = Csoport típusa:
label-plugin-name = Bővítmény neve:
label-plugin-desc = Leírás:
label-plugin-type = Alapértelmezett típus:
label-plugin-image = Kép:
label-visibility = Láthatósági feltételek
label-operator = Operátor:

# Buttons
btn-browse = Tallózás…
btn-clear = Törlés
btn-add = Hozzáadás
btn-remove = Eltávolítás
btn-add-step = Új lépés
btn-delete-step = Lépés törlése
btn-add-group = Csoport hozzáadása
btn-remove-group = Csoport eltávolítása
btn-add-plugin = Bővítmény hozzáadása
btn-remove-plugin = Bővítmény eltávolítása
btn-add-file = Fájl hozzáadása
btn-add-folder = Mappa hozzáadása
btn-remove-file = Eltávolítás
btn-add-flag = Jelző hozzáadása
btn-remove-flag = Jelző eltávolítása
btn-add-condition = Feltétel hozzáadása
btn-remove-condition = Feltétel eltávolítása
btn-add-dependency = Függőség hozzáadása
btn-remove-dependency = Függőség eltávolítása
btn-add-pattern = Új minta
btn-remove-pattern = Minta törlése
btn-save = Mentés
btn-cancel = Mégse
btn-ok = OK
btn-yes = Igen
btn-no = Nem

# Condition/Dependency Labels
label-flag-name = Jelző neve:
label-flag-value = Érték:
label-condition-type = Típus:
label-condition-name = Név:
label-condition-value = Érték:
label-dep-type = Függőség típusa:
label-dep-name = Név/fájl:
label-dep-value = Érték/állapot:

# Files
label-source = Forrás
label-destination = Cél
label-priority = Prioritás
label-file-type = Típus
label-files = Fájlok
label-dependencies = Függőségek

# Settings Dialog
settings-title = Beállítások
settings-tab-general = Általános
settings-tab-recent-files = Legutóbbi fájlok
settings-language = Nyelv:
settings-theme = Téma:
settings-font-size = Betűméret:
settings-replace-newlines = Sortörések feldolgozása a leírásokban
settings-max-recent = Legutóbbi fájlok max. száma:
settings-window-width = Ablakszélesség:
settings-window-height = Ablakmagasság:
settings-no-recent-files = Nincsenek legutóbbi fájlok.

# Status messages for settings
status-settings-saved = A beállítások sikeresen mentve

# About Dialog
about-title = A XIMOD Architect névjegye
about-description = Platformfüggetlen eszköz FOMOD-telepítők készítéséhez Bethesda-játékok modjaihoz.
about-license = MIT licenc alatt licencelve
about-copyright = © 2024 XIMOD Team
about-credit = Az eredeti eszköz Rust portolása Wenderer:

# Script Dialog
script-title = Parancsfájl szerkesztése
script-info = A parancsfájlok mentés előtt vagy után futnak le. A következő makrókat használhatja:
script-macros = Elérhető makrók:
macro-modname = $MODNAME$ - Mod neve
macro-modauthor = $MODAUTHOR$ - Szerző neve
macro-modversion = $MODVERSION$ - Mod verziója
macro-modroot = $MODROOT$ - Gyökérkönyvtár útvonala
macro-date = $DATE$ - Aktuális dátum (ÉÉÉÉ-HH-NN)
macro-time = $TIME$ - Aktuális idő (ÓÓ:PP:MM)
macro-random = $RANDOM$ - Véletlen szám

# Plugin Dependencies
label-default-type = Alapértelmezett típus:
label-pattern-type = Minta típusa:
label-pattern-operator = Minta operátora:

# Conditional Files
label-pattern = Minta

# Validation Messages
validation-no-name = A mod neve kötelező
validation-no-steps = Legalább egy lépés vagy egy kötelező fájl szükséges
validation-empty-step = A(z) { $num }. lépésnek nincs neve
validation-empty-group = A(z) { $step }. lépés { $group }. csoportjának nincs neve
validation-no-plugins = A(z) { $step }. lépés „{ $name }” csoportjának nincsenek bővítményei

# File States
state-active = Aktív
state-inactive = Inaktív
state-missing = Hiányzik

# Confirmation
confirm-title = Megerősítés
confirm-delete = Biztosan törli ezt az elemet?
confirm-discard = Nem mentett módosításai vannak. Elveti őket és folytatja?
confirm-unsaved = Nem mentett módosításai vannak. Menti bezárás előtt?
confirm-save-issues = A projekt a következő problémákat tartalmazza:
confirm-save-anyway = Mentés ennek ellenére?

# Errors
error-invalid-xml = Érvénytelen XML-fájl
error-parse-failed = A FOMOD elemzése sikertelen
error-write-failed = A fájl írása sikertelen
error-create-dir = A könyvtár létrehozása sikertelen

# Default names (generated when creating new items)
default-step-name = . lépés { $num }
default-group-name = . csoport { $num }
default-plugin-name = . bővítmény { $num }
pattern-label = . minta { $num }

# Selection prompts
msg-select-group-first = Először válasszon csoportot.
msg-select-plugin-edit = Válasszon szerkesztendő bővítményt.
label-empty = (üres)
image-no-image = Nincs kép

# File dialog filters
filter-images = Képek
filter-xml = XML

# Dependency types
dep-type-flag = Jelző
dep-type-file = Fájl

# Status bar
status-modified = Módosítva

# Status messages (errors)
msg-settings-save-error = Hiba a beállítások mentésekor
msg-script-save-error = Hiba a parancsfájl mentésekor

# Translation editor
trans-title = Fordításszerkesztő
trans-source-lang = Megjelenített nyelv:
trans-target-lang = Fordítandó nyelv:
trans-col-key = Kulcs
trans-col-source = Címke
trans-col-target = Fordítás
trans-saved = Fordítás mentve
trans-save-error = Hiba a fordítás mentésekor

# XML editor
xml-editor-title = XML-szerkesztő
xml-editor-edit = Szerkesztés
xml-editor-apply = Alkalmaz
xml-editor-revert = Mégse
xml-editor-readonly = Csak olvasható
xml-editor-editing = Szerkesztés — a grafikus lapok zárolva vannak
xml-editor-error = Hiba:
xml-editor-applied = Az XML-módosítások alkalmazva
xml-editor-wellformed = Jól formázott XML
xml-editor-error-at = { $line }. sor, { $col }. oszlop: { $msg }

# Country / flag picker
settings-country-name = Ország neve:
settings-pick-country = Kattintson az ország kiválasztásához
flags-title = Válasszon országot
flags-filter = Szűrő:
flags-none = Nem található zászló

# Translation editor: country & font
trans-endonym = Ország endonimája:
trans-font = Betűtípus:
trans-no-font = (nincs)
trans-browse = Tallózás…
trans-google-fonts = Google Fonts
trans-pick-country = Kattintson az ország kiválasztásához
trans-font-outside = A betűtípust először telepíteni kell az assets/fonts mappába.
trans-font-dir-missing = Az assets/fonts mappa nem található.

# Translation submission
trans-lang-endonym = Nyelv endonimája:
trans-author = Szerző:
trans-submit = Küldés…
trans-submit-hint = Zip létrehozása és előre kitöltött e-mail megnyitása
trans-data-updated = A referenciaadatok frissítve (Languages.json / Countries.json)
trans-package-ready = Archívum kész:
trans-package-error = Az archívum nem hozható létre:

# ISO 639-3 requirement
trans-lang-not-iso = A fordítás csak ISO 639-3 kóddal rendelkező nyelvhez lehetséges.

# FOMOD installer preview
menu-preview = Telepítő előnézete…
preview-title = FOMOD telepítő előnézete
preview-refresh = Frissítés
preview-assumptions = Fájlfeltételezések
preview-details = Részletek
preview-back = Vissza
preview-next = Tovább
preview-install = Telepítés
preview-close = Bezárás
preview-restart = Újraindítás
preview-summary-title = Telepítésre kerülő fájlok
preview-empty = Egyetlen fájl sem kerülne telepítésre.
preview-none-option = (nincs)
preview-invalid = A folytatáshoz töltse ki a kötelező választásokat.
preview-no-steps = Nincs látható lépés; lásd a telepítési összegzést.
preview-select-hint = Válasszon egy lehetőséget a leírásának megtekintéséhez.
preview-col-source = Forrás
preview-col-dest = Cél
preview-col-priority = Prioritás
preview-sel-exactlyone = Pontosan egy lehetőséget válasszon.
preview-sel-atmostone = Legfeljebb egy lehetőséget válasszon.
preview-sel-any = Tetszőleges számú lehetőséget választhat.
preview-sel-all = Minden lehetőség telepítésre kerül.
preview-sel-atleastone = Legalább egy lehetőséget válasszon.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = FOMOD ellenőrzése
validate-report-title = FOMOD-ellenőrzés
validate-ok = Nem található probléma. A FOMOD megfelel a sémának.
xml-editor-schema-ok = Megfelel a ModConfig 5.0 sémának.
xml-editor-schema-issues = Sémaproblémák:
schema-line-col = { $line }. sor, { $col }. oszl.: { $msg }
schema-wrong-root = Váratlan gyökérelem: „{ $found }” (elvárt: „{ $expected }”).
schema-unknown = Váratlan elem: „{ $element }” a következőben: „{ $parent }”.
schema-missing = A(z) „{ $parent }” elemnek tartalmaznia kell egy „{ $child }” elemet.
schema-needs-one = A(z) „{ $parent }” elemnek legalább egy „{ $child }” elemet tartalmaznia kell.
schema-too-many = A(z) „{ $child }” csak egyszer szerepelhet a(z) „{ $parent }” elemben.
schema-missing-attr = A(z) „{ $attr }” attribútum kötelező a(z) „{ $element }” elemen.
schema-bad-enum = Érvénytelen érték: „{ $value }” a(z) { $element }/@{ $attr } esetén (elvárt: { $allowed }).
schema-choose-one = A(z) „{ $parent }” pontosan egyet tartalmazzon a következők közül: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Áthelyezés elé
reorder-after = Áthelyezés mögé

# Country / language database explorer (Properties)
menu-properties = Tulajdonságok…
prop-title = Ország- / nyelvadatbázis
prop-tab-countries = Országok
prop-tab-languages = Nyelvek
prop-filter = Szűrő:
prop-official-langs = Hivatalos nyelvek
prop-spoken-langs = Beszélt nyelvek
prop-endonym = Ország endonimája
prop-font = Betűtípus
prop-spoken-in = Beszélik itt
prop-select-country = Válasszon egy országot a részletek megtekintéséhez.
prop-select-lang = Válasszon egy nyelvet a részletek megtekintéséhez.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = A játék Nexus Mods oldalának megnyitása
