# XIMOD Architect - translation metadata
# @language = lit
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = lietuvių kalba
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versija { $version }

# Status messages
status-ready = Paruošta
msg-save-success = FOMOD sėkmingai išsaugotas
msg-save-error = Klaida išsaugant FOMOD
msg-export-success = Sukurtas platinimo archyvas ({ $count } failų): { $path }
msg-export-error = Klaida kuriant platinimo archyvą: { $error }
msg-load-success = FOMOD sėkmingai įkeltas
msg-load-error = Įkeliant FOMOD įvyko klaida
msg-merge-success = FOMOD sėkmingai sujungtas
msg-merge-error = Sujungiant FOMOD įvyko klaida
msg-no-root-selected = Pirmiausia pasirinkite šakninį katalogą
msg-no-fomod-folder = Nerastas „fomod“ aplankas. Sukurti?
msg-file-outside-root = Failas yra už šakninio katalogo ribų

# Menu - File
menu-file = Failas
menu-new = Naujas
menu-open = Atidaryti aplanką…
menu-open-file = Atidaryti failą…
menu-save = Išsaugoti
menu-recent = Neseniai naudoti
menu-exit = Išeiti
menu-merge = Sujungti FOMOD…
menu-export = Eksportuoti platinimo archyvą…
# Menu - Options
menu-options = Parinktys
menu-settings = Nustatymai…
menu-pre-save-script = Skriptas prieš išsaugojimą…
menu-post-save-script = Skriptas po išsaugojimo…
menu-translation = Versti sąsają…
# Menu - Help
menu-help = Pagalba
menu-check-updates = Tikrinti naujinius…
menu-about = Apie

# Update check
update-checking = Tikrinami naujiniai…
update-up-to-date = XIMOD Architect yra naujausia.
update-check-failed = Nepavyko patikrinti naujinių. Bandykite vėliau.
update-available-status = Galima versija { $version }.
update-banner-text = Galima XIMOD Architect { $version }.
update-download = Atsisiųsti:
update-skip = Praleisti šią versiją
update-later = Vėliau

# Tabs
tab-info = Modifikacijos informacija
tab-steps = Įdiegimo žingsniai
tab-required = Būtini įdiegimai
tab-conditional = Sąlyginiai įdiegimai

# Info Tab
label-workspace = Darbo erdvė
label-root-dir = Pagrindinis katalogas:
label-mod-name = Modifikacijos pavadinimas:
label-author = Autorius:
label-version = Versija:
label-game-name = Žaidimo pavadinimas:
label-category = Kategorija:
label-url = Svetainės URL:
label-header-image = Antraštės paveikslėlis:
label-description = Aprašymas:
placeholder-select-dir = (Pasirinkite katalogą)
placeholder-select-game = (Pasirinkite žaidimą)

# Steps Tab
label-step-name = Žingsnio pavadinimas:
label-group-name = Grupės pavadinimas:
label-group-type = Grupės tipas:
label-plugin-name = Parinkties pavadinimas:
label-plugin-desc = Aprašymas:
label-plugin-type = Numatytasis tipas:
label-plugin-image = Vaizdas:
label-visibility = Matomumo sąlygos
label-operator = Operatorius:

# Buttons
btn-browse = Naršyti...
btn-clear = Išvalyti
btn-add = Pridėti
btn-remove = Pašalinti
btn-add-step = Naujas žingsnis
btn-delete-step = Ištrinti žingsnį
btn-add-group = Pridėti grupę
btn-remove-group = Pašalinti grupę
btn-add-plugin = Pridėti parinktį
btn-remove-plugin = Pašalinti parinktį
btn-add-file = Pridėti failą
btn-add-folder = Pridėti aplanką
btn-remove-file = Pašalinti
btn-add-flag = Pridėti žymę
btn-remove-flag = Pašalinti žymę
btn-add-condition = Pridėti sąlygą
btn-remove-condition = Pašalinti sąlygą
btn-add-dependency = Pridėti priklausomybę
btn-remove-dependency = Pašalinti priklausomybę
btn-add-pattern = Naujas šablonas
btn-remove-pattern = Ištrinti šabloną
btn-save = Išsaugoti
btn-cancel = Atšaukti
btn-ok = Gerai
btn-yes = Taip
btn-no = Ne

# Condition/Dependency Labels
label-flag-name = Žymos pavadinimas:
label-flag-value = Vertė:
label-condition-type = Tipas:
label-condition-name = Pavadinimas:
label-condition-value = Vertė:
label-dep-type = Priklausomybės tipas:
label-dep-name = Pavadinimas/Failas:
label-dep-value = Vertė/Būklė:

# Files
label-source = Šaltinis
label-destination = Paskirties vieta
label-priority = Prioritetas
label-file-type = Tipas

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Visos grupės paskirties vieta
label-page-dest = Diegimo paskirties vieta (visas puslapis)
btn-apply-group-dest = Taikyti visoms šios grupės parinktims
btn-apply-page-dest = Taikyti visoms šio puslapio parinktims
group-dest-hint = Nustato vieną diegimo paskirties vietą kiekvienam šios grupės kiekvienos parinkties failui.
page-dest-hint = Nustato vieną diegimo paskirties vietą kiekvienam šio puslapio kiekvienos parinkties failui (visos grupės).
bulk-dest-nofiles = Kol kas nėra failų, kuriuos reikia atnaujinti — pirmiausia pridėkite failų prie parinkčių.
status-dest-applied = Paskirties vieta pritaikyta { $num } failui(-ams).
preview-hidden-steps = { $num } veiksmas(-ai) paslėptas(-i) dėl dabartinių pasirinkimų.
label-files = Failai
label-dependencies = Priklausomybės

# Settings Dialog
settings-title = Nustatymai
settings-tab-general = Bendrieji
settings-tab-recent-files = Neseniai naudoti failai
settings-language = Kalba:
settings-theme = Tema:
settings-font-size = Šrifto dydis:
settings-replace-newlines = Aprašymuose apdoroti naujų eilučių simbolius
settings-check-updates = Tikrinti naujinius paleidžiant
settings-max-recent = Maksimalus neseniai naudotų failų skaičius:
settings-window-width = Lango plotis:
settings-window-height = Lango aukštis:
settings-no-recent-files = Nėra neseniai naudotų failų.

# Status messages for settings
status-settings-saved = Nustatymai sėkmingai išsaugoti

# About Dialog
about-title = Apie „XIMOD Architect“
about-description = Daugiaplatformis „FOMOD“ diegimo programos kūrimo įrankis, skirtas „Bethesda“ žaidimų modams.
about-license = Licencijuota pagal MIT licenciją
about-copyright = © 2025–2026 „XIMOD“ komanda
about-credit = Originalaus įrankio Rust perkėlimas, atliktas Wenderer:

# Script Dialog
script-title = Redaguoti skriptą
script-info = Skriptai vykdomi prieš arba po išsaugojimo. Galite naudoti šiuos makrokomandas:
script-macros = Galimi makrokomandos:
macro-modname = $MODNAME$ – modifikacijos pavadinimas
macro-modauthor = $MODAUTHOR$ – autoriaus vardas
macro-modversion = $MODVERSION$ – modifikacijos versija
macro-modroot = $MODROOT$ – pagrindinio katalogo kelias
macro-date = $DATE$ – dabartinė data (MMMM-MM-DD)
macro-time = $TIME$ – Dabartinis laikas (HH:MM:SS)
macro-random = $RANDOM$ – Atsitiktinis skaičius

# Plugin Dependencies
label-plugin-dependencies = Parinkties priklausomybės
label-default-type = Numatytasis tipas:
label-pattern-type = Šablono tipas:
label-pattern-operator = Šablono operatorius:

# Conditional Files
label-pattern = Šablonas

# Validation Messages
validation-no-name = Reikalingas modifikatoriaus pavadinimas
validation-no-steps = Reikalingas bent vienas žingsnis arba privalomas failas
validation-empty-step = Žingsnis { $num } neturi pavadinimo
validation-empty-group = Žingsnis { $step }, grupė { $group } neturi pavadinimo
validation-no-plugins = Žingsnis { $step }, grupė „{ $name }“ neturi parinkčių

# File States
state-active = Aktyvi
state-inactive = Neaktyvi
state-missing = Trūksta

# Confirmation
confirm-title = Patvirtinimas
confirm-delete = Ar tikrai norite ištrinti šį elementą?
confirm-discard = Turite neišsaugotų pakeitimų. Atmesti juos ir tęsti?
confirm-unsaved = Turite neišsaugotų pakeitimų. Ar norite išsaugoti prieš uždarant?
confirm-save-issues = Projekte yra šios problemos:
confirm-save-anyway = Vis tiek išsaugoti?

# Errors
error-invalid-xml = Neteisingas XML failas
error-parse-failed = Nepavyko išanalizuoti FOMOD
error-write-failed = Nepavyko įrašyti failo
error-create-dir = Nepavyko sukurti katalogo

# Default names (generated when creating new items)
default-step-name = Žingsnis { $num }
default-group-name = Grupė { $num }
default-plugin-name = Parinktis { $num }
pattern-label = Šablonas { $num }

# Selection prompts
msg-select-group-first = Pirmiausia pasirinkite grupę.
msg-select-plugin-edit = Pasirinkite parinktį, kurią norite redaguoti.
label-empty = (tuščia)
image-no-image = Nėra paveikslėlio

# File dialog filters
filter-images = Vaizdai
filter-xml = XML

# Dependency types
dep-type-flag = Vėliava
dep-type-file = Failas

# Status bar
status-modified = Pakeista

# Status messages (errors)
msg-settings-save-error = Klaida išsaugant nustatymus
msg-script-save-error = Klaida išsaugant scenarijų

# Translation editor
trans-title = Vertimų redaktorius
trans-source-lang = Rodoma kalba:
trans-target-lang = Vertimo kalba:
trans-col-key = Raktinis žodis
trans-col-source = Etiketė
trans-col-target = Vertimas
trans-saved = Vertimas išsaugotas
trans-save-error = Klaida išsaugant vertimą

# XML editor
xml-editor-title = XML redaktorius
xml-editor-edit = Redaguoti
xml-editor-apply = Taikyti
xml-editor-revert = Atšaukti
xml-editor-readonly = Tik skaityti
xml-editor-editing = Redaguojama — grafinės kortelės užrakintos
xml-editor-error = Klaida:
xml-editor-applied = XML pakeitimai pritaikyti
xml-editor-wellformed = Teisingai suformuotas XML
xml-editor-error-at = Eilutė { $line }, stulpelis { $col }: { $msg }

# Country / flag picker
settings-country-name = Šalies pavadinimas:
settings-pick-country = Spustelėkite, kad pasirinkite savo šalį
flags-title = Pasirinkite šalį
flags-filter = Filtras:
flags-none = Vėliava nerasta

# Translation editor: country & font
trans-endonym = Šalies endonimas:
trans-font = Šriftas:
trans-no-font = (nėra)
trans-browse = Naršyti…
trans-google-fonts = „Google Fonts“
trans-pick-country = Spustelėkite, kad pasirinkite šalį
trans-font-outside = Šriftas pirmiausia turi būti įdiegtas į „assets/fonts“ aplanką.
trans-font-dir-missing = Aplankas „assets/fonts“ nerastas.

# Translation submission
trans-lang-endonym = Kalbos pavadinimas:
trans-author = Autorius:
trans-submit = Siųsti…
trans-submit-hint = Sukurkite ZIP failą ir atidarykite iš anksto užpildytą el. laišką
trans-data-updated = Atnaujinti nuorodų duomenys (Languages.json / Countries.json)
trans-package-ready = Archyvas paruoštas:
trans-package-error = Nepavyko sukurti archyvo:

# ISO 639-3 requirement
trans-lang-not-iso = Vertimas galimas tik kalbai, turinčiai ISO 639-3 kodą.

# FOMOD installer preview
menu-preview = Peržiūrėti diegimo programą…
preview-title = FOMOD diegimo programos peržiūra
preview-refresh = Atnaujinti
preview-assumptions = Failų prielaidos
preview-details = Išsamiau
preview-back = Atgal
preview-next = Toliau
preview-install = Įdiegti
preview-close = Uždaryti
preview-restart = Paleisti iš naujo
preview-summary-title = Failai, kurie bus įdiegti
preview-empty = Nebus įdiegtas joks failas.
preview-none-option = (nėra)
preview-invalid = Norėdami tęsti, užpildykite privalomus laukelius.
preview-no-steps = Nėra matomų žingsnių; žr. diegimo santrauką.
preview-select-hint = Pasirinkite parinktį, kad pamatytumėte jos aprašymą.
preview-col-source = Šaltinis
preview-col-dest = Paskirties vieta
preview-col-priority = Prioritetas
preview-sel-exactlyone = Pasirinkite tiksliai vieną parinktį.
preview-sel-atmostone = Pasirinkite ne daugiau kaip vieną parinktį.
preview-sel-any = Pasirinkite bet kokį parinkčių skaičių.
preview-sel-all = Įdiegiamos visos parinktys.
preview-sel-atleastone = Pasirinkite bent vieną parinktį.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Patikrinti FOMOD
validate-report-title = FOMOD tikrinimas
validate-ok = Problemų nerasta. FOMOD atitinka schemą.
xml-editor-schema-ok = Atitinka ModConfig 5.0 schemą.
xml-editor-schema-issues = Schemos problemos:
schema-line-col = Eilutė { $line }, stulpelis { $col }: { $msg }
schema-wrong-root = Netikėtas šaknis „{ $found }“ (lauktas „{ $expected }“).
schema-unknown = Netikėtas elementas „{ $element }“ elemente „{ $parent }“.
schema-missing = „{ $parent }“ turi turėti „{ $child }“.
schema-needs-one = „{ $parent }“ turi turėti bent vieną „{ $child }“.
schema-too-many = „{ $child }“ gali pasirodyti tik vieną kartą „{ $parent }“.
schema-missing-attr = Atributas „{ $attr }“ yra privalomas elemente „{ $element }“.
schema-bad-enum = Netinkama reikšmė „{ $value }“ elementui { $element }/@{ $attr } (laukiama: { $allowed }).
schema-choose-one = „{ $parent }“ turi turėti būtent vieną iš: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Perkelti prieš
reorder-after = Perkelti po

# Country / language database explorer (Properties)
menu-properties = Savybės…
prop-title = Šalių / kalbų duomenų bazė
prop-tab-countries = Šalys
prop-tab-languages = Kalbos
prop-filter = Filtras:
prop-official-langs = Oficialios kalbos
prop-spoken-langs = Kalbos, kuriomis kalbama
prop-endonym = Šalies endonimas
prop-font = Šriftas
prop-spoken-in = Kalbama
prop-select-country = Pasirinkite šalį, kad pamatytumėte jos informaciją.
prop-select-lang = Pasirinkite kalbą, kad pamatytumėte jos informaciją.

# Direct link to Nexus Mods (game slug)
btn-nexus = „Nexus“ ↗
nexus-open-hint = Atidaryti žaidimo „Nexus Mods“ puslapį

# Referenced-file verification (V2)
verify-no-root = Failų tikrinimas praleistas: nenustatytas šakninis aplankas
loc-header = antraštės paveikslėlis
loc-required = privalomi failai
loc-conditional = sąlyginis rinkinys { $num }
loc-plugin = žingsnis { $step }, grupė { $group }, parinktis „{ $plugin }“
verify-missing-file = Trūksta failo: { $path } ({ $loc })
verify-missing-folder = Trūksta aplanko: { $path } ({ $loc })
verify-missing-image = Trūksta paveikslėlio: { $path } ({ $loc })
verify-absolute = Absoliutus kelias (neperkeliamas): { $path } ({ $loc })
verify-outside = Kelias išeina už šakninio aplanko: { $path } ({ $loc })
verify-orphan = Našlaitis failas (nenaudoja jokia parinktis): { $path }
conflict-certain = Paskirties konfliktas: „{ $path }“ rašo { $count } parinktys ({ $locs }) — jos perrašo viena kitą.
conflict-potential = Galimas paskirties konfliktas: „{ $path }“ yra { $count } nuorodų paskirtis ({ $locs }) — perrašymas priklauso nuo pasirinkimo/sąlygų.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Užverti FOMOD
menu-close-all-fomods = Užverti visus FOMOD
tab-untitled = (be pavadinimo)
msg-drop-not-fomod = Numestas elementas nėra FOMOD (aplankas „fomod“ nerastas)
exit-title = Neįrašyti pakeitimai
exit-unsaved = FOMOD neįrašytas. Ar norite jį įrašyti?
tab-close-hint = Užverti šį FOMOD
menu-new-from-folder = Naujas iš aplanko…
menu-templates = Šablonai…
templates-title = Pakartotinai naudojami šablonai
templates-empty = Dar nėra išsaugotų šablonų. Išsaugokite aukščiau pasirinktą žingsnį, kad sukurtumėte.
templates-insert = Įterpti
templates-save-step = Išsaugoti pasirinktą žingsnį
templates-name-hint = Šablono pavadinimas (neprivaloma)
msg-wizard-success = Struktūra sukurta iš aplanko: { $num } parinktis(ių).
msg-wizard-error = Klaida: { $error }
msg-template-saved = Šablonas išsaugotas: { $name }
msg-template-inserted = Šablonas įterptas į projektą.
msg-template-no-step = Pirmiausia pasirinkite žingsnį, kad išsaugotumėte jį kaip šabloną.
msg-template-no-dir = Nepavyko rasti šablonų aplanko.
msg-drop-assigned = Parinkčiai pridėta { $added } šaltinis(ių) ({ $rejected } už šaknies ignoruota).
menu-compare = Palyginti su…
compare-title = FOMOD palyginimas
compare-none = Skirtumų nėra.
btn-optimize-image = Optimizuoti vaizdą
msg-image-optimized = Antraštės vaizdas optimizuotas.
msg-image-ok = Antraštės vaizdas jau neviršija ribų.
msg-no-header-image = Nėra antraštės vaizdo optimizuoti.
verify-image-large = Vaizdas per didelis ({ $width }×{ $height }): { $path }
verify-image-format = Nepalaikomas vaizdo formatas (.{ $ext }): { $path }
verify-image-unreadable = Neperskaitomas vaizdas: { $path }
menu-condition-editor = Sąlygų rengyklė…
condeditor-title = Sąlygų rengyklė
condeditor-set-by = Nustato:
condeditor-used-by = Naudoja:
condeditor-filedeps = Failų priklausomybės
condeditor-empty = Šiame projekte nėra vėliavų ar priklausomybių.
condeditor-orphan-set = nustatyta, bet niekada nenaudota
condeditor-orphan-used = naudota, bet niekada nenustatyta
msg-img-optimized = Vaizdas optimizuotas.
msg-img-ok = Vaizdas jau neviršija ribų.
msg-img-none = Nėra vaizdo optimizuoti.
msg-crash-recovery = Ankstesnė sesija baigėsi netikėtai. Jūsų projekto atsarginė kopija išsaugota { $path }
export-progress-title = Kuriamas platinimo archyvas…
export-progress-files = { $done } / { $total } failų
msg-export-cancelled = Eksportas atšauktas; dalinis archyvas pašalintas.
verify-running = Tikrinami failai diske…
verify-stale = Pastaba: tikrinant failus projektas pasikeitė; paleiskite patikrą iš naujo.
prop-col-name = Pavadinimas
menu-save-as = Įrašyti kaip…
menu-project = Projektas
menu-tools = Įrankiai
menu-manual = Naudotojo vadovas
msg-manual-missing = Naudotojo vadovas (PDF) nerastas šalia programos.
toolbar-new = Naujas
toolbar-open = Atverti
toolbar-save = Įrašyti
toolbar-validate = Patikrinti
toolbar-preview = Peržiūra
toolbar-export = Eksportuoti
dialog-choose-root = Pasirinkite modo šakninį aplanką
exit-unsaved-docs = Neįrašyta: { $names }
status-summary = { $steps } žingsniai · { $options } parinktys
section-groups = Grupės
section-options = Parinktys
section-flags = Sąlygų žymos
section-files = Diegiami failai
hint-group-type = Kaip diegimo programa leidžia naudotojui rinktis šios grupės parinktis.
hint-default-type = Kaip parinktis siūloma, kai neatitinka nė vienas priklausomybių šablonas: privaloma, pasirenkama, rekomenduojama, nenaudotina…
hint-operator = Visos sąlygos turi būti tenkinamos (IR) arba bet kuri viena (ARBA).
hint-flags = Žymos yra pavadintos reikšmės, kurias ši parinktis nustato pasirinkus. Kiti žingsniai ir parinktys gali jas tikrinti, kad būtų rodomi, slepiami ar taptų privalomi.
hint-plugin-dependencies = Šablonai, keičiantys parinkties tipą pagal žymas ar žaidime esančius failus: pavyzdžiui, „Privaloma“, kai įdiegtas kitas modas.
hint-files = Failai ir aplankai, kopijuojami į žaidimo Data aplanką pasirinkus šią parinktį. Paskirtis nurodoma Data atžvilgiu; esant konfliktui laimi aukštesnis prioritetas.
hint-visibility = Sąlygos, kurios turi būti tenkinamos, kad šis žingsnis apskritai būtų rodomas. Palikite tuščia, kad būtų rodomas visada.
seltype-exactly-one = Lygiai viena (privaloma)
seltype-at-most-one = Ne daugiau kaip viena
seltype-any = Bet koks skaičius
seltype-all = Visos (be pasirinkimo)
seltype-at-least-one = Bent viena
plugtype-required = Privaloma
plugtype-optional = Pasirenkama
plugtype-recommended = Rekomenduojama
plugtype-not-usable = Nenaudotina
plugtype-could-be-usable = Galbūt naudotina
plugtype-required-hint = Visada įdiegiama; naudotojas negali jos atžymėti.
plugtype-optional-hint = Siūloma nepažymėta; sprendžia naudotojas.
plugtype-recommended-hint = Siūloma pažymėta; naudotojas gali ją atžymėti.
plugtype-not-usable-hint = Rodoma pilka, jos negalima pasirinkti.
plugtype-could-be-usable-hint = Pasirenkama, bet diegimo programa įspėja, kad gali neveikti.
op-and = Visos sąlygos (IR)
op-or = Bet kuri sąlyga (ARBA)
theme-dark = Tamsi
theme-light = Šviesi
theme-system = Pagal sistemą
condeditor-setter-loc = Žingsnis { "{step}" } / Grupė { "{group}" } / „{ "{name}" }“
condeditor-pattern-of = „{ "{name}" }“ šablonas → { "{type}" }
condeditor-visibility-of = Žingsnio { "{step}" } matomumas
condeditor-cond-set = Sąlyginis rinkinys { "{num}" }
condeditor-needs = { "{ctx}" } (reikia = { "{value}" })
condeditor-file-dep = { "{ctx}" }: failas „{ "{name}" }“ ({ "{state}" })
menu-translate-fomod = Versti FOMOD…
ftr-title = Versti FOMOD
ftr-open-folder = Atidaryti modo aplanką…
ftr-from-active = Iš aktyvaus projekto
ftr-from-active-hint = Verčia pagrindiniame lange atidaryto projekto FOMOD (pirmiausia jį reikia išsaugoti).
ftr-no-fomod = Neįkeltas joks FOMOD.
ftr-encoding = Originalių failų koduotė; išversti failai rašomi ta pačia koduote.
ftr-source-lang = Iš
ftr-target-lang = į
ftr-lang-locked = (įkėlus FOMOD kalbų keisti nebegalima)
ftr-translator = Vertėjas:
ftr-save = Išsaugoti vertimą
ftr-export = Eksportuoti išverstus failus
ftr-export-sibling = Į aplanką fomod_<kalba>
ftr-export-sibling-hint = Įrašo išverstus failus info.xml ir ModuleConfig.xml šalia pradinio fomod aplanko; originalūs failai neliečiami.
ftr-export-inplace = Ant originalių failų
ftr-export-inplace-hint = Pakeičia fomod/info.xml ir fomod/ModuleConfig.xml, prieš tai sukūrus kiekvieno .bak kopiją su laiko žyma.
ftr-force-explicit-order = Išlaikyti pradinę tvarką
ftr-warn-order = Pagal pavadinimą rikiuojamus sąrašus (order="Ascending") modų tvarkytuvė perrikiuotų pagal išverstus pavadinimus. Ši parinktis priverstinai nustato order="Explicit", kad parinktys išlaikytų dabartinę tvarką.
ftr-update = Atnaujinti iš aplanko
ftr-update-hint = Iš naujo nuskaito FOMOD iš disko ir sulieja su juo vertimą: pranešama apie naujas, pakeistas ir pašalintas eilutes.
ftr-preview-translated = Išversta peržiūra
ftr-progress = Išversta { $done } / { $total }
ftr-filter-all = Visos
ftr-filter-untranslated = Neišverstos
ftr-filter-review = Peržiūrėtinos
ftr-filter-issues = Su problemomis
ftr-filter-locked = Užrakintos
ftr-type-all = Visi laukai
ftr-type-names = Pavadinimai
ftr-type-descriptions = Aprašymai
ftr-type-meta = Modo informacija
ftr-search-hint = Ieškoti šaltinyje, vertime arba kontekste…
ftr-next-untranslated = Kita neišversta
ftr-show-whitespace = Rodyti tarpus ir eilučių lūžius
ftr-discard-question = Dabartiniame vertime yra neišsaugotų pakeitimų. Atmesti juos ir įkelti kitą FOMOD?
ftr-discard-yes = Atmesti
ftr-unsaved-close = Vertime yra neišsaugotų pakeitimų.
ftr-col-num = Nr.
ftr-col-status = { "" }
ftr-col-context = Kontekstas
ftr-col-source = Šaltinis
ftr-col-target = Vertimas
ftr-col-issues = { "" }
ftr-empty-hint = Atidarykite modo aplanką arba įkelkite aktyvų projektą, kad būtų parodytos jo verčiamos eilutės.
ftr-empty-filter = Jokia eilutė neatitinka dabartinio filtro.
ftr-select-row = Pasirinkite eilutę, kad galėtumėte redaguoti jos vertimą.
ftr-copy-source = Kopijuoti šaltinį
ftr-clear-target = Išvalyti
ftr-lock = Neversti
ftr-lock-hint = Užrakintos eilutės įrašomos nepakeistos (autorius, svetainė, tikriniai vardai…).
ftr-note = Pastaba:
ftr-status-untranslated = Neišversta
ftr-status-translated = Išversta
ftr-status-auto = Užpildyta automatiškai — peržiūrėkite
ftr-status-fuzzy = Šaltinio tekstas po vertimo pasikeitė — peržiūrėkite
ftr-status-obsolete = FOMOD faile nebėra
ftr-status-locked = Užrakinta (įrašoma nepakeista)
ftr-field-info-name = Modo pavadinimas (info.xml)
ftr-field-module-name = Diegimo programos antraštė (ModuleConfig.xml)
ftr-field-author = Autorius
ftr-field-website = Svetainė
ftr-field-description = Modo aprašymas
ftr-field-step = Žingsnio pavadinimas
ftr-field-group = Grupės pavadinimas
ftr-field-plugin = Parinkties pavadinimas
ftr-field-plugin-desc = Parinkties aprašymas
ftr-issue-empty = Tuščias vertimas
ftr-issue-whitespace = Vertime yra tik tarpai
ftr-issue-edge-whitespace = Tarpai pradžioje arba pabaigoje skiriasi nuo šaltinio
ftr-issue-token = Apsaugotos leksemos skiriasi — trūksta: { $missing } ; perteklinės: { $extra }
ftr-issue-newline-name = Pavadinime negali būti eilutės lūžio
ftr-issue-control = Yra simbolių, kurių XML negali saugoti
ftr-issue-length = Neįprastas ilgis, palyginti su šaltiniu (×{ $ratio })
ftr-issue-identical = Sutampa su šaltiniu
ftr-issue-duplicate = Tas pats šaltinio tekstas kitaip išverstas čia: { $key }
ftr-issue-cdata = Seka ]]> čia neleidžiama
ftr-load-error = Nepavyko įkelti FOMOD: { $error }
ftr-extracted = Rasta verčiamų eilučių: { $num }.
ftr-sidecar-found = Esamas vertimas įkeltas ir sulietas: naujų { $new }, pakeistų { $changed }, pašalintų { $removed }.
ftr-saved = Vertimas išsaugotas čia: { $path }
ftr-save-error = Nepavyko išsaugoti vertimo: { $error }
ftr-save-first = Pirmiausia išsaugokite projektą, tada jį verskite.
ftr-export-success = Eilučių, įrašytų į { $path }: { $count }
ftr-export-error = Eksportuoti nepavyko: { $error }
ftr-export-blocked = Blokuojančių problemų, kurias reikia ištaisyti prieš eksportuojant: { $num }.
ftr-export-stale = Praleista eilučių, nes FOMOD pasikeitė: { $num }; naudokite „Atnaujinti iš aplanko“.
ftr-update-report = Atnaujinta: naujų { $new }, pakeistų { $changed }, perkeltų { $moved }, pašalintų { $removed }, nepakeistų { $unchanged }.
menu-edit = Redaguoti
menu-undo = Atšaukti
menu-redo = Grąžinti
tree-title = Projektas
tree-mod-info = Modifikacijos informacija
tree-steps = Įdiegimo žingsniai
tree-required = Privalomi failai
tree-conditional = Sąlyginiai įdiegimai
tree-empty-steps = Žingsnių dar nėra — spustelėkite +, kad pridėtumėte.
tree-duplicate = Dubliuoti
tree-delete = Ištrinti
tree-save-template = Išsaugoti kaip šabloną…
tree-drop-hint = Numeskite čia, kad perkeltumėte
cond-set-label = Sąlyginis rinkinys { $num }
inspector-empty = Pasirinkite elementą projekto medyje arba pridėkite žingsnį, kad pradėtumėte.
count-options = Parinktys: { $num }
count-files = Failai: { $num }
msg-deleted-undo = Ištrinta. Norėdami atkurti, naudokite „Atšaukti“ (Ctrl+Z).
problems-title = Problemos
problems-errors = Klaidos: { $num }
problems-warnings = Įspėjimai: { $num }
btn-close = Uždaryti
ftr-export-package = Kaip vertimo paketą (archyvas)
ftr-export-package-hint = Sukuria įkelti paruoštą .zip arba .7z failą: išverstus info.xml ir ModuleConfig.xml bei README (tik pataisa) arba visą modą su išverstais failais (visas).
ftr-package-full = Visas modas
ftr-package-full-hint = Į archyvą įtraukti visus modo failus, o ne tik du išverstus XML failus. Įsitikinkite, kad autorius leidžia platinti toliau.
ftr-package-name-template = Pavadinimas:
ftr-readme-patch = Šiame archyve yra modo „{ $name }“ diegimo programos vertimas (kalba: { $langname }; failai fomod/info.xml ir fomod/ModuleConfig.xml). Įdiekite jį ant pradinio modo arba leiskite modų tvarkytuvei jį sulieti, kad išversti failai pakeistų originalius. Keičiasi tik diegimo programos tekstai; pačių modo failų čia nėra. Sukurta su XIMOD Architect.
ftr-readme-full = Šiame archyve yra modas „{ $name }“ su išversta diegimo programa (kalba: { $langname }; failai fomod/info.xml ir fomod/ModuleConfig.xml). Įdiekite jį kaip pradinį modą. Pakeisti tik diegimo programos tekstai. Sukurta su XIMOD Architect.
ftr-apply-memory = Užpildyti iš atminties
ftr-memory-size = Vertimų atmintis — įrašų šiai kalbų porai: { $num }. Į ją įtraukiamas kiekvienas išsaugotas vertimas.
ftr-memory-applied = Eilučių, užpildytų iš vertimų atminties (pažymėtos „peržiūrėtinos“): { $num }.
ftr-memory-suggestion = Atmintis siūlo:
ftr-use-suggestion = Naudoti
ftr-propagate = Pritaikyti tapačioms
ftr-propagate-hint = Nukopijuoti šį vertimą į visas kitas dar neišverstas eilutes su tokiu pačiu šaltinio tekstu.
ftr-propagated = Užpildyta tapačių eilučių: { $num }.
ftr-csv-export = Eksportuoti CSV…
ftr-csv-import = Importuoti CSV…
ftr-csv-imported = Eilučių, atnaujintų iš CSV failo: { $num }.
ftr-csv-error = CSV klaida: { $error }
ftr-glossary = Žodynėlis
ftr-glossary-source = Terminas
ftr-glossary-target = Vertimas
ftr-glossary-case = Raidžių dydis
ftr-glossary-dnt = Palikti
ftr-glossary-add = Pridėti terminą
ftr-issue-glossary = Žodynėlis: „{ $term }“ išverstas ne taip, kaip tikėtasi

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Atidaryti archyvą…
filter-archive = Modifikacijų archyvai (zip, 7z)
msg-archive-opened = Archyvas atidarytas (išskleistų failų: { $num }): { $path }
msg-archive-reused = Archyvas jau išskleistas, pakartotinai naudojamas { $path }
msg-archive-unsupported = Archyvo formatas „.{ $ext }“ nepalaikomas; pirmiausia išskleiskite jį naudodami 7-Zip (atidaryti galima tik .zip ir .7z).
msg-archive-error = Klaida atidarant archyvą: { $error }
msg-archive-no-fomod = Archyve nerastas aplankas „fomod“ ({ $path })
msg-archive-extracting = Skleidžiamas archyvas…
ftr-open-archive = Atidaryti modifikacijos archyvą…
ftr-package-full-partial = Modifikacija atidaryta iš archyvo, kuriame yra tik jos fomod aplankas; pilniems paketams reikia išskleistos modifikacijos.
info-module-deps = Modifikacijos reikalavimai
info-module-deps-hint = Failai arba vėliavėlės, kurių reikia visai modifikacijai prieš paleidžiant diegimo programą (moduleDependencies). Palikite tuščią, jei jų nėra.
info-header-advanced = Išplėstinė antraštė
info-title-position = Pavadinimo padėtis
info-title-colour = Pavadinimo spalva
info-title-colour-hint = Tikimasi: šeši šešioliktainiai skaitmenys (RRGGBB)
info-image-show = Rodyti antraštės paveikslėlį
info-image-fade = Antraštės paveikslėlio išblukimas
info-image-height = Antraštės paveikslėlio aukštis
info-attr-default = (numatytasis)
file-always-install = Visada
file-always-install-hint = Visada įdiegti šį failą, net kai parinktis nepažymėta (alwaysInstall).
file-install-if-usable = Jei tinkamas
file-install-if-usable-hint = Įdiegti šį failą, kai tik parinktis yra tinkama naudoti, net kai ji nepažymėta (installIfUsable).
msg-import-lossy = Šiame FOMOD yra konstrukcijų, kurių XIMOD negali redaguoti (kiekis: { $num }); išsaugant projektą jos bus atmestos.
fidelity-nested-deps = Įdėtoji priklausomybių grupė čia: { $context } (palaikomas tik vienas lygis)
fidelity-game-dep = Žaidimo versijos reikalavimas { $version } čia: { $context }
fidelity-fomm-dep = Modifikacijų tvarkyklės versijos reikalavimas { $version } čia: { $context }
fidelity-unknown = Elementas „{ $element }“ elemente „{ $parent }“ nepalaikomas ({ $context })
loc-module = modifikacijos reikalavimai
loc-step = žingsnis { $step } „{ $name }“
loc-installer = diegimo programa

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Atkurti atsarginę kopiją…
backups-title = Atkurti atsarginę kopiją
backups-empty = Šis projektas dar neturi atsarginės kopijos. Ji sukuriama kaskart, kai projektas išsaugomas ant ankstesnės versijos.
backups-changes = Pakeitimų, palyginti su dabartiniu projektu: { $num }
btn-compare = Palyginti
btn-restore = Atkurti
btn-delete-backups = Ištrinti visas atsargines kopijas
btn-delete-backups-confirm = Spustelėkite dar kartą, kad ištrintumėte visas atsargines kopijas
msg-backup-restored = Atsarginė kopija ({ $time }) atkurta redaktoriuje (dar neišsaugota; Atšaukti ją grąžina)
msg-backups-deleted = Ištrintų atsarginių kopijų: { $num }
settings-backup-count = Saugomų atsarginių kopijų skaičius:
settings-backup-count-hint = Ankstesnių FOMOD XML versijų, išsaugant paliekamų aplanke fomod/backups, skaičius (0 = be atsarginių kopijų).
settings-autosave-minutes = Automatiškai išsaugoti atkūrimo kopiją kas (minutės):
settings-autosave-minutes-hint = Tokiu intervalu konfigūracijos aplanke įrašoma kiekvieno pakeisto projekto atkūrimo kopija; kitą kartą paleidžiant ji siūloma tik po neįprasto išjungimo (0 = išjungta).
settings-auto-masters = Pridėti įskiepio master failus kaip sąlygas
settings-auto-masters-hint = Kai įskiepis (.esp/.esm/.esl) pridedamas prie parinkties, jo reikalaujami master failai, kurių nepateikia nei žaidimas, nei šis modas, tampa parinkties „Active“ failų sąlygomis.
msg-author-from-plugin = Autorius užpildytas iš įskiepio antraštės: { $author }
msg-masters-added = Įskiepio { $plugin } master failų, pridėtų kaip failų sąlygos: { $num }
issue-missing-master = { $plugin } reikalauja { $master }, kurio nėra nei šiame mode, nei jis deklaruotas kaip priklausomybė
issue-esl-mismatch-flag = { $plugin } turi plėtinį .esl, bet jo light (ESL) vėliavėlė nenustatyta
issue-esl-eligible = { $plugin } galėtų būti pažymėtas kaip light (naujų įrašų: { $num }, riba { $limit })
issue-esl-too-big = { $plugin } pažymėtas kaip light, bet neatitinka light įskiepių taisyklių (naujų įrašų: { $num }, riba { $limit }, arba FormID už leidžiamo intervalo ribų)
menu-plugin-report = Įskiepių ataskaita…
plugins-title = Įskiepių ataskaita
plugins-file = Failas
plugins-kind = Rūšis
plugins-light = Light vėliavėlė
plugins-masters = Master failai
plugins-new-records = Nauji įrašai / riba
plugins-eligible = Tinka light
plugins-empty = Šis projektas neįdiegia jokio įskiepio failo (.esp, .esm ar .esl).
plugins-unreadable = neįskaitomas

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Galutinis failų medis
preview-total-size = Bendras įdiegimo dydis: { $size }
preview-tree-truncated = Medis sutrumpintas: per daug failų išskleisti (aukščiau nurodyti dydžiai yra daliniai).
preview-overwritten-by = Perrašė { $plugin }
preview-scenario = Scenarijus:
preview-scenario-load = Įkelti
preview-scenario-save = Išsaugoti…
preview-scenario-delete = Ištrinti
preview-scenario-name = Scenarijaus pavadinimas
preview-scenario-saved = Scenarijus „{ $name }“ išsaugotas aplanke fomod/scenarios
preview-scenario-unresolved = Scenarijaus pasirinkimų, neatitinkančių jokios šio projekto parinkties (pervadinta arba pašalinta): { $num }
preview-scenario-none = (scenarijaus nėra)
issue-unreachable-step = Žingsnis „{ $step }“ niekada negali būti parodytas: jo matomumo sąlygos tikrina žymos reikšmę, kurios nenustato jokia ankstesnė parinktis
issue-unreachable-option = Parinktis „{ $plugin }“ niekada negali būti pasirinkta: jos naudotino tipo šablonai tikrina žymos reikšmę, kurios nenustato jokia parinktis
issue-unreachable-cond = Sąlyginis failų rinkinys { $num } niekada negali būti pritaikytas: jo sąlygos tikrina žymos reikšmę, kurios nenustato jokia parinktis
size-option = Įdiegimo dydis: { $size } (failų: { $num })
size-missing = Trūkstamų šaltinių: { $num }
size-unknown = Įdiegimo dydis: — (paleiskite Patikrinti, kad išmatuotumėte)
menu-nexus-desc = Nexus aprašymas…
nexus-title = Nexus Mods aprašymas
nexus-format = Formatas:
nexus-include-requirements = Reikalavimai
nexus-include-options = Įdiegimo parinktys
nexus-include-install = Įdiegimas
nexus-include-changelog = Pakeitimų žurnalas
nexus-previous = Ankstesnė versija…
nexus-previous-none = (nėra ankstesnės versijos: pakeitimų žurnalo nebus)
nexus-language = Kalba:
nexus-language-source = (šaltinis)
nexus-sec-requirements = Reikalavimai
nexus-sec-options = Įdiegimo parinktys
nexus-sec-install = Įdiegimas
nexus-sec-changelog = Pakeitimų žurnalas
nexus-install-text = Šis modas pateikiamas su FOMOD diegimo programa: įdiekite jį modų tvarkykle (Vortex, Mod Organizer 2) ir pasirinkite parinktis diegimo programoje.
nexus-requires = Reikalauja
nexus-step = Žingsnis
nexus-added = Pridėta
nexus-removed = Pašalinta
nexus-changed = Pakeista
btn-copy = Kopijuoti
btn-save-as = Įrašyti kaip…
msg-copied = Nukopijuota į iškarpinę
msg-saved-to = Išsaugota į { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Pervadinti…
condeditor-rename-exists = Žyma pavadinimu „{ $name }“ jau egzistuoja
condeditor-renamed = Žyma „{ $from }“ pervadinta į „{ $to }“ (pasikartojimų: { $num })
condeditor-delete-uses = Ištrinti visus naudojimus
condeditor-deleted-uses = Žyma „{ $name }“ pašalinta visur (pasikartojimų: { $num })
condeditor-values-set = Nustatomos reikšmės:
condeditor-values-tested = Tikrinamos reikšmės:
condeditor-value-never-set = { $value } — tikrinama, bet niekada nenustatoma
condeditor-value-never-tested = { $value } — nustatoma, bet niekada netikrinama
condeditor-builder = Sąlygų kūrimo įrankis
condeditor-builder-none = Pagrindiniame lange pasirinkite žingsnį, parinktį, sąlyginį failų rinkinį arba modo informaciją, kad čia redaguotumėte jų sąlygas.
condeditor-builder-pattern = Šablonas:
condeditor-sentence-if = JEI
condeditor-sentence-and = IR
condeditor-sentence-or = ARBA
condeditor-sentence-flag = žyma { "{name}" } = { "{value}" }
condeditor-sentence-file = failas { "{name}" } yra { "{value}" }
condeditor-sentence-empty = (sąlygos nėra: visada tiesa)
condeditor-sentence-then-visible = TADA žingsnis rodomas
condeditor-sentence-then-type = TADA parinktis tampa { $type }
condeditor-sentence-then-install = TADA failai įdiegiami
condeditor-sentence-then-module = TADA diegimo programa gali būti paleista (tikrinama prieš paleidžiant)
issue-flag-value-never-set = Žyma „{ $flag }“ tikrinama su reikšme „{ $value }“, kurios nenustato jokia parinktis
issue-flag-never-used = Žyma „{ $flag }“ nustatoma, bet niekur netikrinama
menu-project-strings = Projekto eilutės…
strings-title = Projekto eilutės
strings-search = Ieškoti teksto, vietos arba rakto…
strings-kind-all = Visos
strings-kind-names = Pavadinimai
strings-kind-descriptions = Aprašymai
strings-duplicates-only = Tik dublikatai
strings-replace-with = Pakeisti į:
strings-case = Skirti raidžių dydį
strings-whole-word = Visas žodis
strings-replace-current = Pakeisti
strings-replace-all = Pakeisti visus
strings-replaced = Pakeistų eilučių: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Toks pat tekstas kaip:
strings-count = Eilučių: { $num } · dublikatų grupių: { $dups }
strings-col-location = Vieta
strings-col-field = Laukas
strings-col-text = Tekstas

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Archyvo turinys…
filter-bethesda-archive = Bethesda archyvai (bsa, ba2)
archive-view-title = Archyvo turinys
archive-view-format = Formatas:
archive-view-entries = Įrašai: { $num }
archive-view-size = { $size } išpakavus
archive-view-search = Ieškoti kelio…
archive-view-col-path = Kelias
archive-view-col-size = Dydis
archive-view-col-compressed = Suglaudinta
archive-view-truncated = Rodomi tik pirmieji { $num } atitinkantys įrašai — patikslinkite paiešką.
archive-view-error = Šio archyvo nepavyksta perskaityti: { $error }
archive-view-hint = Peržiūrėti šio archyvo turinį
issue-conflict-archive = Tas pats išteklius keliuose archyvuose: „{ $path }“ supakuotas { $count } nuorodų ({ $locs }) — kuris bus naudojamas, nulemia žaidimo archyvų įkėlimo tvarka.
issue-conflict-archive-loose = Archyvas prieš laisvą failą: „{ $path }“ yra ir supakuotas archyve, ir įdiegtas kaip laisvas failas ({ $locs }) — laisvas failas turi pirmenybę prieš archyvuotą.
preview-in-archive = (archyve)
preview-archived-size = iš jų { $size } supakuota archyvuose

# --- Project tree: expand / collapse menus
tree-expand = Išskleisti
tree-collapse = Suskleisti
tree-expand-all = Išskleisti viską
tree-expand-selected = Išskleisti pasirinktą
tree-expand-from = Išskleisti nuo pasirinkto
tree-collapse-all = Suskleisti viską
tree-collapse-selected = Suskleisti pasirinktą
tree-collapse-from = Suskleisti nuo pasirinkto
tree-expand-all-hint = Išskleidžia visas antraštes
tree-expand-selected-hint = Išskleidžia tik pasirinktą antraštę
tree-expand-from-hint = Išskleidžia pasirinktą antraštę ir viską po ja
tree-collapse-all-hint = Suskleidžia visas antraštes
tree-collapse-selected-hint = Suskleidžia tik pasirinktą antraštę
tree-collapse-from-hint = Suskleidžia pasirinktą antraštę ir viską po ja

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Pridėti grupę
btn-remove-group-cond = Pašalinti grupę
dep-type-game = Žaidimo versija
dep-type-fomm = Modifikacijų tvarkyklės versija
dep-group-hint = Sąlygų grupė, sujungta naudojant IR / ARBA; grupes galima įdėti vieną į kitą.
condeditor-sentence-game = žaidimo versija ≥ { "{value}" }
condeditor-sentence-fomm = modifikacijų tvarkyklės versija ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unikalūs tekstai
ftr-uniques-hint = Rodoma po vieną įrašą kiekvienam skirtingam šaltinio tekstui. Išvertus tą įrašą, iš karto išverčiamos visos eilutės su tokiu pačiu tekstu.
ftr-uniques-synced = Atnaujinta tapačių eilučių: { $num }.
ftr-uniques-group = Šį tekstą turinčių eilučių: { $num }; jo vertimas taikomas visoms.
