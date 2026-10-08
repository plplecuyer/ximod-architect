# XIMOD Architect - translation metadata
# @language = est
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Eesti
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versioon { $version }

# Status messages
status-ready = Valmis
msg-save-success = FOMOD salvestati edukalt
msg-save-error = Viga FOMOD-i salvestamisel
msg-export-success = Jaotamisarhiiv loodud ({ $count } faili): { $path }
msg-export-error = Viga jaotamisarhiivi loomisel: { $error }
msg-load-success = FOMOD laaditi edukalt
msg-load-error = FOMOD-i laadimisel tekkis viga
msg-merge-success = FOMOD ühendati edukalt
msg-merge-error = FOMOD-i ühendamisel tekkis viga
msg-no-root-selected = Palun vali esmalt juurkataloog
msg-no-fomod-folder = 'fomod'-kataloogi ei leitud. Kas soovid selle luua?
msg-file-outside-root = Fail asub väljaspool juurkataloogi

# Menu - File
menu-file = Fail
menu-new = Uus
menu-open = Ava kaust…
menu-open-file = Ava fail…
menu-save = Salvesta
menu-recent = Viimased
menu-exit = Välju
menu-merge = Ühenda FOMOD…
menu-export = Ekspordi levitusarhiiv…
# Menu - Options
menu-options = Valikud
menu-settings = Seaded…
menu-pre-save-script = Skript enne salvestamist…
menu-post-save-script = Skript pärast salvestamist…
menu-translation = Tõlgi kasutajaliides…
# Menu - Help
menu-help = Abi
menu-check-updates = Otsi uuendusi…
menu-about = Info

# Update check
update-checking = Uuenduste otsimine…
update-up-to-date = XIMOD Architect on ajakohane.
update-check-failed = Uuendusi ei saanud kontrollida. Proovige hiljem uuesti.
update-available-status = Versioon { $version } on saadaval.
update-banner-text = XIMOD Architect { $version } on saadaval.
update-download = Laadi alla:
update-skip = Jäta see versioon vahele
update-later = Hiljem

# Tabs
tab-info = Modi info
tab-steps = Paigaldamise sammud
tab-required = Nõutavad paigaldused
tab-conditional = Tingimuslikud paigaldused

# Info Tab
label-workspace = Töökeskkond
label-root-dir = Juurkataloog:
label-mod-name = Modi nimi:
label-author = Autor:
label-version = Versioon:
label-game-name = Mängu nimi:
label-category = Kategooria:
label-url = Veebisaidi URL:
label-header-image = Pealkirja pilt:
label-description = Kirjeldus:
placeholder-select-dir = (Vali kataloog)
placeholder-select-game = (Vali mäng)

# Steps Tab
label-step-name = Sammu nimi:
label-group-name = Rühma nimi:
label-group-type = Rühma tüüp:
label-plugin-name = Valiku nimi:
label-plugin-desc = Kirjeldus:
label-plugin-type = Vaikimisi tüüp:
label-plugin-image = Pilt:
label-visibility = Nähtavuse tingimused
label-operator = Operaator:

# Buttons
btn-browse = Sirvi...
btn-clear = Tühjenda
btn-add = Lisa
btn-remove = Eemalda
btn-add-step = Uus samm
btn-delete-step = Kustuta samm
btn-add-group = Lisa rühm
btn-remove-group = Eemalda rühm
btn-add-plugin = Lisa valik
btn-remove-plugin = Eemalda valik
btn-add-file = Lisa fail
btn-add-folder = Lisa kaust
btn-remove-file = Eemalda
btn-add-flag = Lisa märge
btn-remove-flag = Eemalda märge
btn-add-condition = Lisa tingimus
btn-remove-condition = Eemalda tingimus
btn-add-dependency = Lisa sõltuvus
btn-remove-dependency = Eemalda sõltuvus
btn-add-pattern = Uus muster
btn-remove-pattern = Kustuta muster
btn-save = Salvesta
btn-cancel = Tühista
btn-ok = OK
btn-yes = Jah
btn-no = Ei

# Condition/Dependency Labels
label-flag-name = Lipu nimi:
label-flag-value = Väärtus:
label-condition-type = Tüüp:
label-condition-name = Nimi:
label-condition-value = Väärtus:
label-dep-type = Sõltuvuse tüüp:
label-dep-name = Nimi/fail:
label-dep-value = Väärtus/seisund:

# Files
label-source = Allikas
label-destination = Sihtkoht
label-priority = Prioriteet
label-file-type = Tüüp

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Kogu grupi sihtkoht
label-page-dest = Paigalduse sihtkoht (kogu leht)
btn-apply-group-dest = Rakenda kõigile selle grupi valikutele
btn-apply-page-dest = Rakenda kõigile selle lehe valikutele
group-dest-hint = Määrab ühe paigalduse sihtkoha selle grupi iga valiku igale failile.
page-dest-hint = Määrab ühe paigalduse sihtkoha selle lehe iga valiku igale failile (kõik grupid).
bulk-dest-nofiles = Veel pole faile, mida uuendada — lisa esmalt valikutele faile.
status-dest-applied = Sihtkoht rakendatud { $num } failile.
preview-hidden-steps = { $num } sammu peidetud praeguste valikutega.
label-files = Failid
label-dependencies = Sõltuvused

# Settings Dialog
settings-title = Seaded
settings-tab-general = Üldine
settings-tab-recent-files = Viimased failid
settings-language = Keel:
settings-theme = Teema:
settings-font-size = Kirjasuurus:
settings-replace-newlines = Töötle kirjeldustes olevaid ridadevahetusi
settings-check-updates = Otsi uuendusi käivitamisel
settings-max-recent = Viimaste failide maksimaalne arv:
settings-window-width = Akna laius:
settings-window-height = Akna kõrgus:
settings-no-recent-files = Viimaseid faile pole.

# Status messages for settings
status-settings-saved = Seaded salvestati edukalt

# About Dialog
about-title = Teave XIMOD Architecti kohta
about-description = Platvormiülene FOMOD-installeri loomise tööriist Bethesda mängude modifikatsioonidele.
about-license = Litsentsitud MIT-litsentsi alusel
about-copyright = © 2025–2026 XIMOD Team
about-credit = Wenderer originaaltööriista Rust port:

# Script Dialog
script-title = Skripti redigeerimine
script-info = Skripte täidetakse enne või pärast salvestamist. Võite kasutada järgmisi makrosid:
script-macros = Saadaval olevad makrod:
macro-modname = $MODNAME$ – modifikatsiooni nimi
macro-modauthor = $MODAUTHOR$ – autori nimi
macro-modversion = $MODVERSION$ – modifikatsiooni versioon
macro-modroot = $MODROOT$ – juurkataloogi tee
macro-date = $DATE$ – praegune kuupäev (AAAA-KK-PP)
macro-time = $TIME$ – praegune kellaaeg (HH:MM:SS)
macro-random = $RANDOM$ – juhuslik number

# Plugin Dependencies
label-plugin-dependencies = Valiku sõltuvused
label-default-type = Vaikimisi tüüp:
label-pattern-type = Mustri tüüp:
label-pattern-operator = Mustri operaator:

# Conditional Files
label-pattern = Muster

# Validation Messages
validation-no-name = Mooduli nimi on kohustuslik
validation-no-steps = Vaja on vähemalt ühte sammu või kohustuslikku faili
validation-empty-step = Sammul { $num } puudub nimi
validation-empty-group = Sammul { $step }, rühmal { $group } puudub nimi
validation-no-plugins = Sammul { $step }, rühmal "{ $name }" puuduvad valikud

# File States
state-active = Aktiivne
state-inactive = Mitteaktiivne
state-missing = Puudub

# Confirmation
confirm-title = Kinnitus
confirm-delete = Kas soovite seda elementi kindlasti kustutada?
confirm-discard = Teil on salvestamata muudatusi. Kas soovite need tühistada ja jätkata?
confirm-unsaved = Teil on salvestamata muudatusi. Kas soovite enne sulgemist salvestada?
confirm-save-issues = Projektil on järgmised probleemid:
confirm-save-anyway = Kas soovite ikkagi salvestada?

# Errors
error-invalid-xml = Kehtetu XML-fail
error-parse-failed = FOMOD-i analüüsimine ebaõnnestus
error-write-failed = Faili kirjutamine ebaõnnestus
error-create-dir = Kataloogi loomine ebaõnnestus

# Default names (generated when creating new items)
default-step-name = Samm { $num }
default-group-name = Rühm { $num }
default-plugin-name = Valik { $num }
pattern-label = Muster { $num }

# Selection prompts
msg-select-group-first = Valige kõigepealt grupp.
msg-select-plugin-edit = Valige redigeeritav valik.
label-empty = (tühi)
image-no-image = Pilt puudub

# File dialog filters
filter-images = Pildid
filter-xml = XML

# Dependency types
dep-type-flag = Lipuke
dep-type-file = Fail

# Status bar
status-modified = Muudetud

# Status messages (errors)
msg-settings-save-error = Viga seadete salvestamisel
msg-script-save-error = Viga skripti salvestamisel

# Translation editor
trans-title = Tõlkeeditor
trans-source-lang = Kuvatav keel:
trans-target-lang = Tõlgitav keel:
trans-col-key = Võti
trans-col-source = Silt
trans-col-target = Tõlge
trans-saved = Tõlge salvestatud
trans-save-error = Tõlke salvestamisel tekkis viga

# XML editor
xml-editor-title = XML-redaktor
xml-editor-edit = Redigeeri
xml-editor-apply = Rakenda
xml-editor-revert = Tühista
xml-editor-readonly = Ainult lugemiseks
xml-editor-editing = Redigeerimine — graafilised vahekaardid on lukustatud
xml-editor-error = Viga:
xml-editor-applied = XML-muudatused rakendatud
xml-editor-wellformed = Korrektselt vormindatud XML
xml-editor-error-at = Rida { $line }, veerg { $col }: { $msg }

# Country / flag picker
settings-country-name = Riigi nimi:
settings-pick-country = Klõpsa, et valida oma riik
flags-title = Vali riik
flags-filter = Filter:
flags-none = Lipu ei leitud

# Translation editor: country & font
trans-endonym = Riigi endonüüm:
trans-font = Font:
trans-no-font = (puudub)
trans-browse = Sirvi…
trans-google-fonts = Google Fonts
trans-pick-country = Klõpsake, et valida riik
trans-font-outside = Font tuleb esmalt installida kausta assets/fonts.
trans-font-dir-missing = Kausta assets/fonts ei leitud.

# Translation submission
trans-lang-endonym = Keele endonüüm:
trans-author = Autor:
trans-submit = Saada…
trans-submit-hint = Koosta zip-fail ja ava eeltäidetud e-kiri
trans-data-updated = Viited andmed uuendatud (Languages.json / Countries.json)
trans-package-ready = Arhiiv valmis:
trans-package-error = Arhiivi ei õnnestunud koostada:

# ISO 639-3 requirement
trans-lang-not-iso = Tõlkimine on võimalik ainult keelte puhul, millel on ISO 639-3 kood.

# FOMOD installer preview
menu-preview = Paigaldaja eelvaade…
preview-title = FOMODi paigaldaja eelvaade
preview-refresh = Värskenda
preview-assumptions = Failide eeldused
preview-details = Detailid
preview-back = Tagasi
preview-next = Edasi
preview-install = Paigalda
preview-close = Sulge
preview-restart = Käivita uuesti
preview-summary-title = Paigaldatavad failid
preview-empty = Ühtegi faili ei paigaldata.
preview-none-option = (puudub)
preview-invalid = Jätkamiseks täitke nõutud valikud.
preview-no-steps = Ühtegi sammu ei ole näha; vaata paigaldamise kokkuvõtet.
preview-select-hint = Vali valik, et näha selle kirjeldust.
preview-col-source = Allikas
preview-col-dest = Sihtkoht
preview-col-priority = Prioriteet
preview-sel-exactlyone = Vali täpselt üks valik.
preview-sel-atmostone = Vali maksimaalselt üks valik.
preview-sel-any = Valige suvaline arv valikuid.
preview-sel-all = Kõik valikud on paigaldatud.
preview-sel-atleastone = Valige vähemalt üks valik.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Valideeri FOMOD
validate-report-title = FOMOD-valideerimine
validate-ok = Probleeme ei leitud. FOMOD vastab skeemile.
xml-editor-schema-ok = Vastab ModConfig 5.0 skeemile.
xml-editor-schema-issues = Skeemiprobleemid:
schema-line-col = Rida { $line }, veerg { $col }: { $msg }
schema-wrong-root = Ootamatu juur „{ $found }“ (ootati „{ $expected }“).
schema-unknown = Ootamatu element „{ $element }” elemendis „{ $parent }”.
schema-missing = „{ $parent }” peab sisaldama „{ $child }”.
schema-needs-one = „{ $parent }” peab sisaldama vähemalt ühte „{ $child }”.
schema-too-many = „{ $child }“ võib esineda „{ $parent }“-is ainult üks kord.
schema-missing-attr = Atribuut „{ $attr }“ on „{ $element }“-is kohustuslik.
schema-bad-enum = Kehtetu väärtus „{ $value }“ elemendile { $element }/@{ $attr } (ootus: { $allowed }).
schema-choose-one = „{ $parent }“ peab sisaldama täpselt ühte järgmistest: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Liiguta ettepoole
reorder-after = Liiguta tahapoole

# Country / language database explorer (Properties)
menu-properties = Omadused…
prop-title = Riigi / keele andmebaas
prop-tab-countries = Riigid
prop-tab-languages = Keeled
prop-filter = Filter:
prop-official-langs = Ametlikud keeled
prop-spoken-langs = Räägitavad keeled
prop-endonym = Riigi endonüüm
prop-font = Font
prop-spoken-in = Räägitakse
prop-select-country = Vali riik, et näha selle üksikasju.
prop-select-lang = Vali keel, et näha selle üksikasju.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Ava mängu Nexus Modsi lehekülg

# Referenced-file verification (V2)
verify-no-root = Failide kontroll jäeti vahele: juurkausta pole määratud
loc-header = päisepilt
loc-required = nõutavad failid
loc-conditional = tingimuslik komplekt { $num }
loc-plugin = samm { $step }, rühm { $group }, valik „{ $plugin }“
verify-missing-file = Puuduv fail: { $path } ({ $loc })
verify-missing-folder = Puuduv kaust: { $path } ({ $loc })
verify-missing-image = Puuduv pilt: { $path } ({ $loc })
verify-absolute = Absoluutne tee (mitte kaasaskantav): { $path } ({ $loc })
verify-outside = Tee väljub juurkaustast: { $path } ({ $loc })
verify-orphan = Orvufail (ükski valik ei viita sellele): { $path }
conflict-certain = Sihtkoha konflikt: „{ $path }“ kirjutab { $count } valikut ({ $locs }) – need kirjutavad üksteist üle.
conflict-potential = Võimalik sihtkoha konflikt: „{ $path }“ on { $count } viite sihtkoht ({ $locs }) – ülekirjutamine sõltub valikust/tingimustest.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Sulge FOMOD
menu-close-all-fomods = Sulge kõik FOMODid
tab-untitled = (pealkirjata)
msg-drop-not-fomod = Lohistatud üksus ei ole FOMOD (kausta „fomod“ ei leitud)
exit-title = Salvestamata muudatused
exit-unsaved = Üks FOMOD on salvestamata. Kas soovite selle salvestada?
tab-close-hint = Sulge see FOMOD
menu-new-from-folder = Uus kaustast…
menu-templates = Mallid…
templates-title = Korduvkasutatavad mallid
templates-empty = Malle pole veel salvestatud. Salvestage ülal valitud samm, et luua uus.
templates-insert = Lisa
templates-save-step = Salvesta valitud samm
templates-name-hint = Malli nimi (valikuline)
msg-wizard-success = Struktuur loodud kaustast: { $num } valik(ut).
msg-wizard-error = Viga: { $error }
msg-template-saved = Mall salvestatud: { $name }
msg-template-inserted = Mall lisatud projekti.
msg-template-no-step = Valige esmalt samm, et see mallina salvestada.
msg-template-no-dir = Mallikausta ei leitud.
msg-drop-assigned = Lisatud { $added } allikas(t) valikule ({ $rejected } juurest väljas eiratud).
menu-compare = Võrdle kaustaga…
compare-title = FOMODi võrdlus
compare-none = Erinevusi pole.
btn-optimize-image = Optimeeri pilt
msg-image-optimized = Päisepilt optimeeritud.
msg-image-ok = Päisepilt on juba piirides.
msg-no-header-image = Pole päisepilti, mida optimeerida.
verify-image-large = Pilt liiga suur ({ $width }×{ $height }): { $path }
verify-image-format = Toetamata pildivorming (.{ $ext }): { $path }
verify-image-unreadable = Loetamatu pilt: { $path }
menu-condition-editor = Tingimuste redaktor…
condeditor-title = Tingimuste redaktor
condeditor-set-by = Määrab:
condeditor-used-by = Kasutab:
condeditor-filedeps = Failisõltuvused
condeditor-empty = Selles projektis pole lippe ega sõltuvusi.
condeditor-orphan-set = määratud, kuid kunagi kasutamata
condeditor-orphan-used = kasutatud, kuid kunagi määramata
msg-img-optimized = Pilt optimeeritud.
msg-img-ok = Pilt on juba piirides.
msg-img-none = Pole pilti, mida optimeerida.
msg-crash-recovery = Eelmine seanss lõppes ootamatult. Teie projekti varukoopia salvestati asukohta { $path }
export-progress-title = Levitusarhiivi loomine…
export-progress-files = { $done } / { $total } faili
msg-export-cancelled = Eksport tühistati; poolik arhiiv eemaldati.
verify-running = Failide kontrollimine kettal…
verify-stale = Märkus: projekt muutus failide kontrollimise ajal; käivitage valideerimine uuesti.
prop-col-name = Nimi
menu-save-as = Salvesta nimega…
menu-project = Projekt
menu-tools = Tööriistad
menu-manual = Kasutusjuhend
msg-manual-missing = Kasutusjuhendit (PDF) ei leitud rakenduse kõrvalt.
toolbar-new = Uus
toolbar-open = Ava
toolbar-save = Salvesta
toolbar-validate = Valideeri
toolbar-preview = Eelvaade
toolbar-export = Ekspordi
dialog-choose-root = Valige modi juurkaust
exit-unsaved-docs = Salvestamata: { $names }
status-summary = { $steps } sammu · { $options } valikut
section-groups = Rühmad
section-options = Valikud
section-flags = Tingimuslipud
section-files = Paigaldatavad failid
hint-group-type = Kuidas paigaldaja laseb kasutajal selle rühma valikuid valida.
hint-default-type = Kuidas valikut pakutakse, kui ükski sõltuvusmuster ei sobi: nõutav, valikuline, soovitatav, kasutamatu…
hint-operator = Kõik tingimused peavad kehtima (JA) või ükskõik milline neist (VÕI).
hint-flags = Lipud on nimega väärtused, mille see valik valimisel seab. Teised sammud ja valikud saavad neid kontrollida, et end näidata, peita või nõutavaks muuta.
hint-plugin-dependencies = Mustrid, mis muudavad valiku tüüpi lippude või mängus olevate failide järgi: näiteks „Nõutav“, kui teine mod on paigaldatud.
hint-files = Failid ja kaustad, mis kopeeritakse mängu Data-kausta, kui see valik on valitud. Sihtkoht on Data suhtes; konflikti korral võidab kõrgem prioriteet.
hint-visibility = Tingimused, mis peavad kehtima, et seda sammu üldse näidataks. Jätke tühjaks, et alati näidata.
seltype-exactly-one = Täpselt üks (nõutav)
seltype-at-most-one = Kõige rohkem üks
seltype-any = Suvaline arv
seltype-all = Kõik (valikuta)
seltype-at-least-one = Vähemalt üks
plugtype-required = Nõutav
plugtype-optional = Valikuline
plugtype-recommended = Soovitatav
plugtype-not-usable = Kasutamatu
plugtype-could-be-usable = Võib olla kasutatav
plugtype-required-hint = Paigaldatakse alati; kasutaja ei saa seda eemaldada.
plugtype-optional-hint = Pakutakse märkimata; kasutaja otsustab.
plugtype-recommended-hint = Pakutakse märgituna; kasutaja võib selle eemaldada.
plugtype-not-usable-hint = Kuvatakse hallina ja seda ei saa valida.
plugtype-could-be-usable-hint = Valitav, kuid paigaldaja hoiatab, et see ei pruugi töötada.
op-and = Kõik tingimused (JA)
op-or = Ükskõik milline tingimus (VÕI)
theme-dark = Tume
theme-light = Hele
theme-system = Süsteemi järgi
condeditor-setter-loc = Samm { "{step}" } / Rühm { "{group}" } / „{ "{name}" }“
condeditor-pattern-of = Muster „{ "{name}" }“ → { "{type}" }
condeditor-visibility-of = Sammu { "{step}" } nähtavus
condeditor-cond-set = Tingimuslik komplekt { "{num}" }
condeditor-needs = { "{ctx}" } (nõuab = { "{value}" })
condeditor-file-dep = { "{ctx}" }: fail „{ "{name}" }“ ({ "{state}" })
menu-translate-fomod = Tõlgi FOMOD…
ftr-title = FOMOD-i tõlkimine
ftr-open-folder = Ava modi kaust…
ftr-from-active = Aktiivsest projektist
ftr-from-active-hint = Tõlgib peaaknas avatud projekti FOMOD-i (see tuleb esmalt salvestada).
ftr-no-fomod = Ühtegi FOMOD-i pole laaditud.
ftr-encoding = Algfailide kodeering; tõlgitud failid kirjutatakse sama kodeeringuga.
ftr-source-lang = Keelest
ftr-target-lang = keelde
ftr-lang-locked = (keeled on pärast FOMOD-i laadimist fikseeritud)
ftr-translator = Tõlkija:
ftr-save = Salvesta tõlge
ftr-export = Ekspordi tõlgitud failid
ftr-export-sibling = Kausta fomod_<keel>
ftr-export-sibling-hint = Kirjutab tõlgitud failid info.xml ja ModuleConfig.xml algse fomod-kausta kõrvale; algfaile ei puudutata.
ftr-export-inplace = Algfailide peale
ftr-export-inplace-hint = Asendab failid fomod/info.xml ja fomod/ModuleConfig.xml pärast seda, kui kummastki on tehtud ajatempliga .bak-koopia.
ftr-force-explicit-order = Säilita algne järjestus
ftr-warn-order = Nime järgi sorditud loendid (order="Ascending") sorditaks modihalduris tõlgitud nimede järgi ümber. See valik sunnib peale order="Explicit", et valikud säilitaksid praeguse järjestuse.
ftr-update = Uuenda kaustast
ftr-update-hint = Loeb FOMOD-i kettalt uuesti ja ühendab tõlke sellega: uutest, muudetud ja eemaldatud sõnedest antakse teada.
ftr-preview-translated = Tõlgitud eelvaade
ftr-progress = { $done } / { $total } tõlgitud
ftr-filter-all = Kõik
ftr-filter-untranslated = Tõlkimata
ftr-filter-review = Üle vaadata
ftr-filter-issues = Probleemidega
ftr-filter-locked = Lukustatud
ftr-type-all = Kõik väljad
ftr-type-names = Nimed
ftr-type-descriptions = Kirjeldused
ftr-type-meta = Modi teave
ftr-search-hint = Otsi lähtetekstist, tõlkest või kontekstist…
ftr-next-untranslated = Järgmine tõlkimata
ftr-show-whitespace = Näita tühikuid ja reavahetusi
ftr-discard-question = Praeguses tõlkes on salvestamata muudatusi. Kas hüljata need ja laadida teine FOMOD?
ftr-discard-yes = Hülga
ftr-unsaved-close = Tõlkes on salvestamata muudatusi.
ftr-col-num = Nr
ftr-col-status = { "" }
ftr-col-context = Kontekst
ftr-col-source = Lähtetekst
ftr-col-target = Tõlge
ftr-col-issues = { "" }
ftr-empty-hint = Ava modi kaust või laadi aktiivne projekt, et kuvada selle tõlgitavad sõned.
ftr-empty-filter = Ükski sõne ei vasta praegusele filtrile.
ftr-select-row = Vali rida, et muuta selle tõlget.
ftr-copy-source = Kopeeri lähtetekst
ftr-clear-target = Tühjenda
ftr-lock = Ära tõlgi
ftr-lock-hint = Lukustatud sõned kirjutatakse muutmata kujul (autor, veebisait, pärisnimed…).
ftr-note = Märkus:
ftr-status-untranslated = Tõlkimata
ftr-status-translated = Tõlgitud
ftr-status-auto = Automaatselt eeltäidetud — palun vaata üle
ftr-status-fuzzy = Lähtetekst on pärast tõlkimist muutunud — palun vaata üle
ftr-status-obsolete = FOMOD-is enam olemas ei ole
ftr-status-locked = Lukustatud (kirjutatakse muutmata kujul)
ftr-field-info-name = Modi nimi (info.xml)
ftr-field-module-name = Installeri pealkiri (ModuleConfig.xml)
ftr-field-author = Autor
ftr-field-website = Veebisait
ftr-field-description = Modi kirjeldus
ftr-field-step = Sammu nimi
ftr-field-group = Rühma nimi
ftr-field-plugin = Valiku nimi
ftr-field-plugin-desc = Valiku kirjeldus
ftr-issue-empty = Tühi tõlge
ftr-issue-whitespace = Tõlge sisaldab ainult tühikuid
ftr-issue-edge-whitespace = Tühikud alguses või lõpus erinevad lähtetekstist
ftr-issue-token = Kaitstud märgendid erinevad — puudu: { $missing } ; üleliigsed: { $extra }
ftr-issue-newline-name = Nimi ei tohi sisaldada reavahetust
ftr-issue-control = Sisaldab märke, mida XML ei saa salvestada
ftr-issue-length = Ebatavaline pikkus võrreldes lähtetekstiga (×{ $ratio })
ftr-issue-identical = Identne lähtetekstiga
ftr-issue-duplicate = Sama lähtetekst on kohas { $key } tõlgitud teisiti
ftr-issue-cdata = Jada ]]> ei ole siin lubatud
ftr-load-error = FOMOD-i ei õnnestunud laadida: { $error }
ftr-extracted = Leitud tõlgitavaid sõnesid: { $num }.
ftr-sidecar-found = Olemasolev tõlge laaditud ja ühendatud: uusi { $new }, muudetud { $changed }, eemaldatud { $removed }.
ftr-saved = Tõlge salvestatud asukohta { $path }
ftr-save-error = Tõlget ei õnnestunud salvestada: { $error }
ftr-save-first = Salvesta esmalt projekt ja seejärel tõlgi see.
ftr-export-success = Asukohta { $path } kirjutatud sõnesid: { $count }
ftr-export-error = Eksport ebaõnnestus: { $error }
ftr-export-blocked = Enne eksportimist tuleb lahendada blokeerivad probleemid: { $num }.
ftr-export-stale = FOMOD-i muutumise tõttu vahele jäetud sõnesid: { $num }; kasuta käsku „Uuenda kaustast“.
ftr-update-report = Uuendatud: uusi { $new }, muudetud { $changed }, teisaldatud { $moved }, eemaldatud { $removed }, muutmata { $unchanged }.
menu-edit = Redigeeri
menu-undo = Võta tagasi
menu-redo = Tee uuesti
tree-title = Projekt
tree-mod-info = Modi teave
tree-steps = Paigaldamise sammud
tree-required = Nõutavad failid
tree-conditional = Tingimuslikud paigaldused
tree-empty-steps = Samme veel pole — lisamiseks klõpsake +.
tree-duplicate = Dubleeri
tree-delete = Kustuta
tree-save-template = Salvesta mallina…
tree-drop-hint = Teisaldamiseks kukutage siia
cond-set-label = Tingimuslik komplekt { $num }
inspector-empty = Valige projektipuust element või lisage alustamiseks samm.
count-options = Valikuid: { $num }
count-files = Faile: { $num }
msg-deleted-undo = Kustutatud. Taastamiseks kasutage käsku „Võta tagasi“ (Ctrl+Z).
problems-title = Probleemid
problems-errors = Vigu: { $num }
problems-warnings = Hoiatusi: { $num }
btn-close = Sulge
ftr-export-package = Tõlkepaketina (arhiiv)
ftr-export-package-hint = Loob üleslaadimiseks valmis .zip- või .7z-faili: tõlgitud info.xml ja ModuleConfig.xml koos README-failiga (ainult paik) või kogu mod koos tõlgitud failidega (täielik).
ftr-package-full = Kogu mod
ftr-package-full-hint = Lisa arhiivi kõik modi failid, mitte ainult kaks tõlgitud XML-faili. Veendu, et autor lubab edasilevitamist.
ftr-package-name-template = Nimi:
ftr-readme-patch = See arhiiv sisaldab modi „{ $name }“ installeri tõlget (keel: { $langname }; failid fomod/info.xml ja fomod/ModuleConfig.xml). Paigalda see algse modi peale või lase modihalduril see ühendada, et tõlgitud failid asendaksid algsed. Muutuvad ainult installeri tekstid; modi enda faile arhiivis ei ole. Loodud rakendusega XIMOD Architect.
ftr-readme-full = See arhiiv sisaldab modi „{ $name }“ koos tõlgitud installeriga (keel: { $langname }; failid fomod/info.xml ja fomod/ModuleConfig.xml). Paigalda see nagu algne mod. Muudetud on ainult installeri tekste. Loodud rakendusega XIMOD Architect.
ftr-apply-memory = Täida mälust
ftr-memory-size = Tõlkemälu — kirjeid selle keelepaari jaoks: { $num }. Iga salvestatud tõlge lisatakse sinna.
ftr-memory-applied = Tõlkemälust täidetud sõnesid (märgitud „üle vaadata“): { $num }.
ftr-memory-suggestion = Mälu pakub:
ftr-use-suggestion = Kasuta
ftr-propagate = Kanna üle samasugustele
ftr-propagate-hint = Kopeerib selle tõlke kõigisse teistesse sama lähtetekstiga sõnedesse, mis on veel tõlkimata.
ftr-propagated = Täidetud samasuguseid sõnesid: { $num }.
ftr-csv-export = Ekspordi CSV…
ftr-csv-import = Impordi CSV…
ftr-csv-imported = CSV-failist uuendatud sõnesid: { $num }.
ftr-csv-error = CSV viga: { $error }
ftr-glossary = Sõnastik
ftr-glossary-source = Termin
ftr-glossary-target = Tõlge
ftr-glossary-case = Tõstutundlik
ftr-glossary-dnt = Säilita
ftr-glossary-add = Lisa termin
ftr-issue-glossary = Sõnastik: „{ $term }“ ei ole tõlgitud ootuspäraselt

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Ava arhiiv…
filter-archive = Modi arhiivid (zip, 7z)
msg-archive-opened = Arhiiv avatud (lahti pakitud faile: { $num }): { $path }
msg-archive-reused = Arhiiv on juba lahti pakitud, taaskasutatakse { $path }
msg-archive-unsupported = Arhiivivormingut „.{ $ext }“ ei toetata; pakkige see esmalt 7-Zip abil lahti (avada saab ainult .zip ja .7z).
msg-archive-error = Viga arhiivi avamisel: { $error }
msg-archive-no-fomod = Arhiivist ei leitud kausta „fomod“ ({ $path })
msg-archive-extracting = Arhiivi lahtipakkimine…
ftr-open-archive = Ava modi arhiiv…
ftr-package-full-partial = Mod avati arhiivist, mis sisaldab ainult selle fomod-kausta; täispaketid vajavad lahti pakitud modi.
info-module-deps = Modi nõuded
info-module-deps-hint = Failid või lipud, mida kogu mod enne paigaldaja käivitumist nõuab (moduleDependencies). Jätke tühjaks, kui neid pole.
info-header-advanced = Täpsem päis
info-title-position = Pealkirja asukoht
info-title-colour = Pealkirja värv
info-title-colour-hint = Oodatav: kuus kuueteistkümnendsüsteemi numbrit (RRGGBB)
info-image-show = Näita päisepilti
info-image-fade = Hajuta päisepilt
info-image-height = Päisepildi kõrgus
info-attr-default = (vaikimisi)
file-always-install = Alati
file-always-install-hint = Paigalda see fail alati, ka siis, kui valik pole valitud (alwaysInstall).
file-install-if-usable = Kui sobiv
file-install-if-usable-hint = Paigalda see fail alati, kui valik on kasutatav, ka siis, kui see pole valitud (installIfUsable).
msg-import-lossy = See FOMOD sisaldab konstruktsioone, mida XIMOD ei saa muuta (arv: { $num }); need jäetakse projekti salvestamisel välja.
fidelity-nested-deps = Pesastatud sõltuvusrühm kohas { $context } (toetatakse ainult ühte taset)
fidelity-game-dep = Mängu versiooni nõue { $version } kohas { $context }
fidelity-fomm-dep = Modihalduri versiooni nõue { $version } kohas { $context }
fidelity-unknown = Elementi „{ $element }“ elemendis „{ $parent }“ ei toetata ({ $context })
loc-module = modi nõuded
loc-step = samm { $step } „{ $name }“
loc-installer = paigaldaja

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Taasta varukoopia…
backups-title = Taasta varukoopia
backups-empty = Sellel projektil pole veel varukoopiat. Varukoopia luuakse iga kord, kui projekt salvestatakse eelmise versiooni peale.
backups-changes = Erinevusi praeguse projektiga: { $num }
btn-compare = Võrdle
btn-restore = Taasta
btn-delete-backups = Kustuta kõik varukoopiad
btn-delete-backups-confirm = Klõpsake uuesti, et kustutada kõik varukoopiad
msg-backup-restored = Varukoopia ajast { $time } taastati redaktorisse (pole veel salvestatud; Võta tagasi tühistab selle)
msg-backups-deleted = Kustutatud varukoopiaid: { $num }
settings-backup-count = Säilitatavaid varukoopiaid:
settings-backup-count-hint = Mitu FOMOD XML-i eelmist versiooni salvestamisel kaustas fomod/backups säilitatakse (0 = varukoopiaid ei tehta).
settings-autosave-minutes = Salvesta taastekoopia automaatselt iga (minutit):
settings-autosave-minutes-hint = Selle intervalliga kirjutatakse iga muudetud projekti taastekoopia seadistuskausta; järgmisel käivitamisel pakutakse seda ainult pärast ebanormaalset sulgemist (0 = väljas).
settings-auto-masters = Lisa plugina master-failid tingimustena
settings-auto-masters-hint = Kui plugin (.esp/.esm/.esl) lisatakse valikule, muutuvad selle nõutavad master-failid, mida ei paku ei mäng ega see mod, valiku „Active“ failitingimusteks.
msg-author-from-plugin = Autor täideti plugina päisest: { $author }
msg-masters-added = Plugina { $plugin } master-faile lisati failitingimustena: { $num }
issue-missing-master = { $plugin } nõuab faili { $master }, mida pole selles modis ega ole deklareeritud sõltuvusena
issue-esl-mismatch-flag = Failil { $plugin } on laiend .esl, kuid selle light-lipp (ESL) pole seatud
issue-esl-eligible = { $plugin } võiks olla märgitud light-pluginaks (uusi kirjeid: { $num }, piirang { $limit })
issue-esl-too-big = { $plugin } on märgitud light-pluginaks, kuid ei vasta light-pluginate reeglitele (uusi kirjeid: { $num }, piirang { $limit }, või FormID väljaspool lubatud vahemikku)
menu-plugin-report = Pluginate aruanne…
plugins-title = Pluginate aruanne
plugins-file = Fail
plugins-kind = Liik
plugins-light = Light-lipp
plugins-masters = Master-failid
plugins-new-records = Uued kirjed / piirang
plugins-eligible = Light-kõlblik
plugins-empty = See projekt ei paigalda ühtegi pluginafaili (.esp, .esm ega .esl).
plugins-unreadable = loetamatu

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Lõplik failipuu
preview-total-size = Paigalduse kogumaht: { $size }
preview-tree-truncated = Puu on kärbitud: liiga palju faile, et neid lahti rullida (ülaltoodud mahud on osalised).
preview-overwritten-by = Üle kirjutanud { $plugin }
preview-scenario = Stsenaarium:
preview-scenario-load = Laadi
preview-scenario-save = Salvesta…
preview-scenario-delete = Kustuta
preview-scenario-name = Stsenaariumi nimi
preview-scenario-saved = Stsenaarium „{ $name }“ salvestati kausta fomod/scenarios
preview-scenario-unresolved = Stsenaariumi kirjeid, mis ei vasta ühelegi selle projekti valikule (ümber nimetatud või eemaldatud): { $num }
preview-scenario-none = (stsenaarium puudub)
issue-unreachable-step = Sammu „{ $step }“ ei saa kunagi näidata: selle nähtavustingimused kontrollivad lipu väärtust, mida ükski varasem valik ei sea
issue-unreachable-option = Valikut „{ $plugin }“ ei saa kunagi valida: selle kasutatava tüübi mustrid kontrollivad lipu väärtust, mida ükski valik ei sea
issue-unreachable-cond = Tingimuslik failikomplekt { $num } ei saa kunagi rakenduda: selle tingimused kontrollivad lipu väärtust, mida ükski valik ei sea
size-option = Paigalduse maht: { $size } (faile: { $num })
size-missing = Puuduvaid allikaid: { $num }
size-unknown = Paigalduse maht: — (mõõtmiseks käivita Valideeri)
menu-nexus-desc = Nexuse kirjeldus…
nexus-title = Nexus Modsi kirjeldus
nexus-format = Vorming:
nexus-include-requirements = Nõuded
nexus-include-options = Paigaldusvalikud
nexus-include-install = Paigaldamine
nexus-include-changelog = Muudatuste logi
nexus-previous = Eelmine versioon…
nexus-previous-none = (eelmist versiooni pole: muudatuste logi puudub)
nexus-language = Keel:
nexus-language-source = (lähtekeel)
nexus-sec-requirements = Nõuded
nexus-sec-options = Paigaldusvalikud
nexus-sec-install = Paigaldamine
nexus-sec-changelog = Muudatuste logi
nexus-install-text = See mod on varustatud FOMOD paigaldajaga: paigalda see modihalduriga (Vortex, Mod Organizer 2) ja vali oma valikud paigaldajas.
nexus-requires = Nõuab
nexus-step = Samm
nexus-added = Lisatud
nexus-removed = Eemaldatud
nexus-changed = Muudetud
btn-copy = Kopeeri
btn-save-as = Salvesta nimega…
msg-copied = Kopeeritud lõikelauale
msg-saved-to = Salvestatud asukohta { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Nimeta ümber…
condeditor-rename-exists = Lipp nimega „{ $name }“ on juba olemas
condeditor-renamed = Lipp „{ $from }“ nimetati ümber lipuks „{ $to }“ (esinemisi: { $num })
condeditor-delete-uses = Kustuta kõik kasutused
condeditor-deleted-uses = Lipp „{ $name }“ eemaldati kõikjalt (esinemisi: { $num })
condeditor-values-set = Seatud väärtused:
condeditor-values-tested = Kontrollitud väärtused:
condeditor-value-never-set = { $value } — kontrollitakse, kuid ei seata kunagi
condeditor-value-never-tested = { $value } — seatakse, kuid ei kontrollita kunagi
condeditor-builder = Tingimuste koostaja
condeditor-builder-none = Vali peaaknas samm, valik, tingimuslik failikomplekt või modi teave, et selle tingimusi siin muuta.
condeditor-builder-pattern = Muster:
condeditor-sentence-if = KUI
condeditor-sentence-and = JA
condeditor-sentence-or = VÕI
condeditor-sentence-flag = lipp { "{name}" } = { "{value}" }
condeditor-sentence-file = fail { "{name}" } on { "{value}" }
condeditor-sentence-empty = (tingimus puudub: alati tõene)
condeditor-sentence-then-visible = SIIS sammu näidatakse
condeditor-sentence-then-type = SIIS muutub valik olekuks { $type }
condeditor-sentence-then-install = SIIS failid paigaldatakse
condeditor-sentence-then-module = SIIS saab paigaldaja käivituda (kontrollitakse enne käivitumist)
issue-flag-value-never-set = Lippu „{ $flag }“ kontrollitakse väärtusega „{ $value }“, mida ükski valik ei sea
issue-flag-never-used = Lipp „{ $flag }“ seatakse, kuid seda ei kontrollita kusagil
menu-project-strings = Projekti tekstid…
strings-title = Projekti tekstid
strings-search = Otsi teksti, asukohta või võtit…
strings-kind-all = Kõik
strings-kind-names = Nimed
strings-kind-descriptions = Kirjeldused
strings-duplicates-only = Ainult duplikaadid
strings-replace-with = Asenda tekstiga:
strings-case = Tõstutundlik
strings-whole-word = Terve sõna
strings-replace-current = Asenda
strings-replace-all = Asenda kõik
strings-replaced = Asendatud tekste: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Sama tekst kui:
strings-count = Tekste: { $num } · duplikaadirühmi: { $dups }
strings-col-location = Asukoht
strings-col-field = Väli
strings-col-text = Tekst

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Arhiivi sisu…
filter-bethesda-archive = Bethesda arhiivid (bsa, ba2)
archive-view-title = Arhiivi sisu
archive-view-format = Vorming:
archive-view-entries = Kirjeid: { $num }
archive-view-size = { $size } lahtipakituna
archive-view-search = Otsi teed…
archive-view-col-path = Tee
archive-view-col-size = Maht
archive-view-col-compressed = Pakitud
archive-view-truncated = Kuvatakse ainult esimesed { $num } sobivat kirjet – täpsustage otsingut.
archive-view-error = Seda arhiivi ei saa lugeda: { $error }
archive-view-hint = Vaata selle arhiivi sisu
issue-conflict-archive = Sama ressurss mitmes arhiivis: „{ $path }“ on pakitud { $count } viite poolt ({ $locs }) – mängu arhiivide laadimisjärjekord otsustab, millist kasutatakse.
issue-conflict-archive-loose = Arhiiv vs lahtine fail: „{ $path }“ on nii arhiivi pakitud kui ka lahtise failina paigaldatud ({ $locs }) – lahtine fail on arhiivis oleva ees ülimuslik.
preview-in-archive = (arhiivis)
preview-archived-size = millest { $size } on pakitud arhiividesse

# --- Project tree: expand / collapse menus
tree-expand = Laienda
tree-collapse = Ahenda
tree-expand-all = Laienda kõik
tree-expand-selected = Laienda valitud
tree-expand-from = Laienda alates valitust
tree-collapse-all = Ahenda kõik
tree-collapse-selected = Ahenda valitud
tree-collapse-from = Ahenda alates valitust
tree-expand-all-hint = Laiendab kõik pealkirjad
tree-expand-selected-hint = Laiendab ainult valitud pealkirja
tree-expand-from-hint = Laiendab valitud pealkirja ja kõik selle all oleva
tree-collapse-all-hint = Ahendab kõik pealkirjad
tree-collapse-selected-hint = Ahendab ainult valitud pealkirja
tree-collapse-from-hint = Ahendab valitud pealkirja ja kõik selle all oleva

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Lisa rühm
btn-remove-group-cond = Eemalda rühm
dep-type-game = Mängu versioon
dep-type-fomm = Modihalduri versioon
dep-group-hint = Rühm tingimusi, mis on ühendatud JA / VÕI abil; rühmi saab üksteise sisse paigutada.
condeditor-sentence-game = mängu versioon ≥ { "{value}" }
condeditor-sentence-fomm = modihalduri versioon ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Kordumatud tekstid
ftr-uniques-hint = Näitab iga erineva lähteteksti kohta ühte rida. Selle rea tõlkimine tõlgib korraga kõik sama tekstiga sõned.
ftr-uniques-synced = Uuendatud samasuguseid sõnesid: { $num }.
ftr-uniques-group = Seda teksti jagavaid sõnesid: { $num }; selle tõlge rakendub neile kõigile.
