# XIMOD Architect - translation metadata
# @language = fin
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Suomi
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Versio { $version }

# Status messages
status-ready = Valmis
msg-save-success = FOMOD tallennettu onnistuneesti
msg-save-error = Virhe FOMODin tallennuksessa
msg-export-success = Jakelupaketti luotu ({ $count } tiedostoa): { $path }
msg-export-error = Virhe jakelupaketin luonnissa: { $error }
msg-load-success = FOMOD ladattu onnistuneesti
msg-load-error = Virhe FOMODin latauksessa
msg-merge-success = FOMOD yhdistetty onnistuneesti
msg-merge-error = Virhe FOMODin yhdistämisessä
msg-no-root-selected = Valitse ensin juurihakemisto
msg-no-fomod-folder = ”fomod”-kansiota ei löytynyt. Luodaanko se?
msg-file-outside-root = Tiedosto on juurihakemiston ulkopuolella

# Menu - File
menu-file = Tiedosto
menu-new = Uusi
menu-open = Avaa kansio…
menu-open-file = Avaa tiedosto…
menu-save = Tallenna
menu-recent = Viimeisimmät
menu-exit = Lopeta
menu-merge = Yhdistä FOMOD…
menu-export = Vie jakelupaketti…
# Menu - Options
menu-options = Asetukset
menu-settings = Asetukset…
menu-pre-save-script = Skripti ennen tallennusta…
menu-post-save-script = Skripti tallennuksen jälkeen…
menu-translation = Käännä käyttöliittymä…
# Menu - Help
menu-help = Ohje
menu-check-updates = Tarkista päivitykset…
menu-about = Tietoja

# Update check
update-checking = Tarkistetaan päivityksiä…
update-up-to-date = XIMOD Architect on ajan tasalla.
update-check-failed = Päivityksiä ei voitu tarkistaa. Yritä myöhemmin uudelleen.
update-available-status = Versio { $version } on saatavilla.
update-banner-text = XIMOD Architect { $version } on saatavilla.
update-download = Lataa:
update-skip = Ohita tämä versio
update-later = Myöhemmin

# Tabs
tab-info = Modin tiedot
tab-steps = Asennusvaiheet
tab-required = Pakolliset asennukset
tab-conditional = Ehdolliset asennukset

# Info Tab
label-workspace = Työtila
label-root-dir = Juurihakemisto:
label-mod-name = Modin nimi:
label-author = Tekijä:
label-version = Versio:
label-game-name = Pelin nimi:
label-category = Luokka:
label-url = Verkkosivuston URL:
label-header-image = Otsikkokuva:
label-description = Kuvaus:
placeholder-select-dir = (Valitse hakemisto)
placeholder-select-game = (Valitse peli)

# Steps Tab
label-step-name = Vaiheen nimi:
label-group-name = Ryhmän nimi:
label-group-type = Ryhmän tyyppi:
label-plugin-name = Vaihtoehdon nimi:
label-plugin-desc = Kuvaus:
label-plugin-type = Oletustyyppi:
label-plugin-image = Kuva:
label-visibility = Näkyvyysehdot
label-operator = Operaattori:

# Buttons
btn-browse = Selaa…
btn-clear = Tyhjennä
btn-add = Lisää
btn-remove = Poista
btn-add-step = Uusi vaihe
btn-delete-step = Poista vaihe
btn-add-group = Lisää ryhmä
btn-remove-group = Poista ryhmä
btn-add-plugin = Lisää vaihtoehto
btn-remove-plugin = Poista vaihtoehto
btn-add-file = Lisää tiedosto
btn-add-folder = Lisää kansio
btn-remove-file = Poista
btn-add-flag = Lisää lippu
btn-remove-flag = Poista lippu
btn-add-condition = Lisää ehto
btn-remove-condition = Poista ehto
btn-add-dependency = Lisää riippuvuus
btn-remove-dependency = Poista riippuvuus
btn-add-pattern = Uusi malli
btn-remove-pattern = Poista malli
btn-save = Tallenna
btn-cancel = Peruuta
btn-ok = OK
btn-yes = Kyllä
btn-no = Ei

# Condition/Dependency Labels
label-flag-name = Lipun nimi:
label-flag-value = Arvo:
label-condition-type = Tyyppi:
label-condition-name = Nimi:
label-condition-value = Arvo:
label-dep-type = Riippuvuustyyppi:
label-dep-name = Nimi/tiedosto:
label-dep-value = Arvo/tila:

# Files
label-source = Lähde
label-destination = Kohde
label-priority = Prioriteetti
label-file-type = Tyyppi

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Koko ryhmän kohde
label-page-dest = Asennuskohde (koko sivu)
btn-apply-group-dest = Käytä kaikkiin tämän ryhmän vaihtoehtoihin
btn-apply-page-dest = Käytä kaikkiin tämän sivun vaihtoehtoihin
group-dest-hint = Asettaa yhden asennuskohteen tämän ryhmän jokaisen vaihtoehdon jokaiselle tiedostolle.
page-dest-hint = Asettaa yhden asennuskohteen tämän sivun jokaisen vaihtoehdon jokaiselle tiedostolle (kaikki ryhmät).
bulk-dest-nofiles = Ei vielä päivitettäviä tiedostoja — lisää ensin tiedostoja vaihtoehtoihin.
status-dest-applied = Kohde otettu käyttöön { $num } tiedostolle.
preview-hidden-steps = { $num } vaihe(tta) piilotettu nykyisillä valinnoilla.
label-files = Tiedostot
label-dependencies = Riippuvuudet

# Settings Dialog
settings-title = Asetukset
settings-tab-general = Yleiset
settings-tab-recent-files = Viimeisimmät tiedostot
settings-language = Kieli:
settings-theme = Teema:
settings-font-size = Fonttikoko:
settings-replace-newlines = Käsittele rivinvaihdot kuvauksissa
settings-check-updates = Tarkista päivitykset käynnistettäessä
settings-max-recent = Viimeisimpiä tiedostoja enint.:
settings-window-width = Ikkunan leveys:
settings-window-height = Ikkunan korkeus:
settings-no-recent-files = Ei viimeisimpiä tiedostoja.

# Status messages for settings
status-settings-saved = Asetukset tallennettu onnistuneesti

# About Dialog
about-title = Tietoja XIMOD Architectista
about-description = Alustariippumaton työkalu FOMOD-asennusohjelmien luomiseen Bethesda-pelien modeille.
about-license = Lisensoitu MIT-lisenssillä
about-copyright = © 2024 XIMOD Team
about-credit = Wenderer alkuperäisen työkalun Rust-portti:

# Script Dialog
script-title = Muokkaa skriptiä
script-info = Skriptit suoritetaan ennen tallennusta tai sen jälkeen. Voit käyttää seuraavia makroja:
script-macros = Käytettävissä olevat makrot:
macro-modname = $MODNAME$ - Modin nimi
macro-modauthor = $MODAUTHOR$ - Tekijän nimi
macro-modversion = $MODVERSION$ - Modin versio
macro-modroot = $MODROOT$ - Juurihakemiston polku
macro-date = $DATE$ - Nykyinen päivämäärä (VVVV-KK-PP)
macro-time = $TIME$ - Nykyinen kellonaika (TT:MM:SS)
macro-random = $RANDOM$ - Satunnaisluku

# Plugin Dependencies
label-plugin-dependencies = Vaihtoehdon riippuvuudet
label-default-type = Oletustyyppi:
label-pattern-type = Mallin tyyppi:
label-pattern-operator = Mallin operaattori:

# Conditional Files
label-pattern = Malli

# Validation Messages
validation-no-name = Modin nimi vaaditaan
validation-no-steps = Tarvitaan vähintään yksi vaihe tai pakollinen tiedosto
validation-empty-step = Vaiheella { $num } ei ole nimeä
validation-empty-group = Vaiheella { $step }, ryhmällä { $group } ei ole nimeä
validation-no-plugins = Vaiheella { $step }, ryhmällä ”{ $name }” ei ole vaihtoehtoja

# File States
state-active = Aktiivinen
state-inactive = Ei-aktiivinen
state-missing = Puuttuu

# Confirmation
confirm-title = Vahvistus
confirm-delete = Haluatko varmasti poistaa tämän kohteen?
confirm-discard = Sinulla on tallentamattomia muutoksia. Hylätäänkö ne ja jatketaan?
confirm-unsaved = Sinulla on tallentamattomia muutoksia. Haluatko tallentaa ennen sulkemista?
confirm-save-issues = Projektissa on seuraavat ongelmat:
confirm-save-anyway = Tallennetaanko silti?

# Errors
error-invalid-xml = Virheellinen XML-tiedosto
error-parse-failed = FOMODin jäsentäminen epäonnistui
error-write-failed = Tiedoston kirjoittaminen epäonnistui
error-create-dir = Hakemiston luominen epäonnistui

# Default names (generated when creating new items)
default-step-name = Vaihe { $num }
default-group-name = Ryhmä { $num }
default-plugin-name = Vaihtoehto { $num }
pattern-label = Malli { $num }

# Selection prompts
msg-select-group-first = Valitse ensin ryhmä.
msg-select-plugin-edit = Valitse muokattava vaihtoehto.
label-empty = (tyhjä)
image-no-image = Ei kuvaa

# File dialog filters
filter-images = Kuvat
filter-xml = XML

# Dependency types
dep-type-flag = Lippu
dep-type-file = Tiedosto

# Status bar
status-modified = Muokattu

# Status messages (errors)
msg-settings-save-error = Virhe asetusten tallennuksessa
msg-script-save-error = Virhe skriptin tallennuksessa

# Translation editor
trans-title = Käännöseditori
trans-source-lang = Näytettävä kieli:
trans-target-lang = Käännettävä kieli:
trans-col-key = Avain
trans-col-source = Selite
trans-col-target = Käännös
trans-saved = Käännös tallennettu
trans-save-error = Virhe käännöksen tallennuksessa

# XML editor
xml-editor-title = XML-editori
xml-editor-edit = Muokkaa
xml-editor-apply = Ota käyttöön
xml-editor-revert = Peruuta
xml-editor-readonly = Vain luku
xml-editor-editing = Muokataan — graafiset välilehdet on lukittu
xml-editor-error = Virhe:
xml-editor-applied = XML-muutokset otettu käyttöön
xml-editor-wellformed = Muotoiltu XML on kelvollinen
xml-editor-error-at = Rivi { $line }, sarake { $col }: { $msg }

# Country / flag picker
settings-country-name = Maan nimi:
settings-pick-country = Valitse maasi napsauttamalla
flags-title = Valitse maa
flags-filter = Suodata:
flags-none = Lippua ei löytynyt

# Translation editor: country & font
trans-endonym = Maan endonyymi:
trans-font = Fontti:
trans-no-font = (ei mitään)
trans-browse = Selaa…
trans-google-fonts = Google Fonts
trans-pick-country = Valitse maa napsauttamalla
trans-font-outside = Fontti on ensin asennettava kansioon assets/fonts.
trans-font-dir-missing = assets/fonts-kansiota ei löytynyt.

# Translation submission
trans-lang-endonym = Kielen endonyymi:
trans-author = Tekijä:
trans-submit = Lähetä…
trans-submit-hint = Luo zip ja avaa esitäytetty sähköposti
trans-data-updated = Viitetiedot päivitetty (Languages.json / Countries.json)
trans-package-ready = Paketti valmis:
trans-package-error = Paketin luonti epäonnistui:

# ISO 639-3 requirement
trans-lang-not-iso = Kääntäminen on mahdollista vain kielelle, jolla on ISO 639-3 -koodi.

# FOMOD installer preview
menu-preview = Esikatsele asennusohjelmaa…
preview-title = FOMOD-asennusohjelman esikatselu
preview-refresh = Päivitä
preview-assumptions = Tiedosto-oletukset
preview-details = Tiedot
preview-back = Takaisin
preview-next = Seuraava
preview-install = Asenna
preview-close = Sulje
preview-restart = Aloita alusta
preview-summary-title = Asennettavat tiedostot
preview-empty = Yhtään tiedostoa ei asennettaisi.
preview-none-option = (ei mitään)
preview-invalid = Täytä vaaditut valinnat jatkaaksesi.
preview-no-steps = Yhtään vaihetta ei ole näkyvissä; katso asennuksen yhteenveto.
preview-select-hint = Valitse vaihtoehto nähdäksesi sen kuvauksen.
preview-col-source = Lähde
preview-col-dest = Kohde
preview-col-priority = Prioriteetti
preview-sel-exactlyone = Valitse tasan yksi vaihtoehto.
preview-sel-atmostone = Valitse enintään yksi vaihtoehto.
preview-sel-any = Valitse mikä tahansa määrä vaihtoehtoja.
preview-sel-all = Kaikki vaihtoehdot asennetaan.
preview-sel-atleastone = Valitse vähintään yksi vaihtoehto.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Tarkista FOMOD
validate-report-title = FOMODin tarkistus
validate-ok = Ongelmia ei löytynyt. FOMOD on skeeman mukainen.
xml-editor-schema-ok = ModConfig 5.0 -skeeman mukainen.
xml-editor-schema-issues = Skeemaongelmat:
schema-line-col = Rivi { $line }, sar. { $col }: { $msg }
schema-wrong-root = Odottamaton juuri ”{ $found }” (odotettiin ”{ $expected }”).
schema-unknown = Odottamaton elementti ”{ $element }” elementissä ”{ $parent }”.
schema-missing = ”{ $parent }” on sisällettävä ”{ $child }”.
schema-needs-one = ”{ $parent }” on sisällettävä vähintään yksi ”{ $child }”.
schema-too-many = ”{ $child }” saa esiintyä vain kerran elementissä ”{ $parent }”.
schema-missing-attr = Attribuutti ”{ $attr }” vaaditaan elementissä ”{ $element }”.
schema-bad-enum = Virheellinen arvo ”{ $value }” kohteelle { $element }/@{ $attr } (odotettiin: { $allowed }).
schema-choose-one = ”{ $parent }” on sisällettävä tasan yksi seuraavista: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Siirrä eteen
reorder-after = Siirrä taakse

# Country / language database explorer (Properties)
menu-properties = Ominaisuudet…
prop-title = Maa- ja kielitietokanta
prop-tab-countries = Maat
prop-tab-languages = Kielet
prop-filter = Suodata:
prop-official-langs = Viralliset kielet
prop-spoken-langs = Puhutut kielet
prop-endonym = Maan endonyymi
prop-font = Fontti
prop-spoken-in = Puhutaan alueella
prop-select-country = Valitse maa nähdäksesi sen tiedot.
prop-select-lang = Valitse kieli nähdäksesi sen tiedot.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Avaa pelin Nexus Mods -sivu

# Referenced-file verification (V2)
verify-no-root = Tiedostojen tarkistus ohitettiin: juurikansiota ei ole asetettu
loc-header = otsikkokuva
loc-required = pakolliset tiedostot
loc-conditional = ehdollinen joukko { $num }
loc-plugin = vaihe { $step }, ryhmä { $group }, vaihtoehto ”{ $plugin }”
verify-missing-file = Puuttuva tiedosto: { $path } ({ $loc })
verify-missing-folder = Puuttuva kansio: { $path } ({ $loc })
verify-missing-image = Puuttuva kuva: { $path } ({ $loc })
verify-absolute = Absoluuttinen polku (ei siirrettävä): { $path } ({ $loc })
verify-outside = Polku poistuu juurikansiosta: { $path } ({ $loc })
verify-orphan = Orpo tiedosto (mikään vaihtoehto ei viittaa siihen): { $path }
conflict-certain = Kohderistiriita: ”{ $path }” kirjoittaa { $count } vaihtoehtoa ({ $locs }) – ne korvaavat toisensa.
conflict-potential = Mahdollinen kohderistiriita: ”{ $path }” on { $count } viittauksen kohde ({ $locs }) – korvaaminen riippuu valinnasta/ehdoista.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Sulje FOMOD
menu-close-all-fomods = Sulje kaikki FOMODit
tab-untitled = (nimetön)
msg-drop-not-fomod = Pudotettu kohde ei ole FOMOD (”fomod”-kansiota ei löytynyt)
exit-title = Tallentamattomat muutokset
exit-unsaved = FOMODia ei ole tallennettu. Haluatko tallentaa sen?
tab-close-hint = Sulje tämä FOMOD
menu-new-from-folder = Uusi kansiosta…
menu-templates = Mallit…
templates-title = Uudelleenkäytettävät mallit
templates-empty = Malleja ei ole vielä tallennettu. Tallenna yllä valittu vaihe luodaksesi sellaisen.
templates-insert = Lisää
templates-save-step = Tallenna valittu vaihe
templates-name-hint = Mallin nimi (valinnainen)
msg-wizard-success = Runko luotu kansiosta: { $num } vaihtoehto(a).
msg-wizard-error = Virhe: { $error }
msg-template-saved = Malli tallennettu: { $name }
msg-template-inserted = Malli lisätty projektiin.
msg-template-no-step = Valitse ensin vaihe tallentaaksesi sen mallina.
msg-template-no-dir = Mallikansiota ei löytynyt.
msg-drop-assigned = Lisätty { $added } lähde(ttä) vaihtoehtoon ({ $rejected } juuren ulkopuolella ohitettu).
menu-compare = Vertaa kohteeseen…
compare-title = FOMOD-vertailu
compare-none = Ei eroja.
btn-optimize-image = Optimoi kuva
msg-image-optimized = Otsikkokuva optimoitu.
msg-image-ok = Otsikkokuva on jo rajoissa.
msg-no-header-image = Ei optimoitavaa otsikkokuvaa.
verify-image-large = Kuva liian suuri ({ $width }×{ $height }): { $path }
verify-image-format = Kuvamuotoa ei tueta (.{ $ext }): { $path }
verify-image-unreadable = Lukukelvoton kuva: { $path }
menu-condition-editor = Ehtomuokkain…
condeditor-title = Ehtomuokkain
condeditor-set-by = Asettaa:
condeditor-used-by = Käyttää:
condeditor-filedeps = Tiedostoriippuvuudet
condeditor-empty = Ei lippuja tai riippuvuuksia tässä projektissa.
condeditor-orphan-set = asetettu mutta ei koskaan käytetty
condeditor-orphan-used = käytetty mutta ei koskaan asetettu
msg-img-optimized = Kuva optimoitu.
msg-img-ok = Kuva on jo rajoissa.
msg-img-none = Ei optimoitavaa kuvaa.
msg-crash-recovery = Edellinen istunto päättyi odottamatta. Projektisi varmuuskopio tallennettiin kansioon { $path }
export-progress-title = Luodaan jakelupakettia…
export-progress-files = { $done } / { $total } tiedostoa
msg-export-cancelled = Vienti peruutettiin; keskeneräinen paketti poistettiin.
verify-running = Tarkistetaan levyllä olevia tiedostoja…
verify-stale = Huomautus: projekti muuttui tiedostojen tarkistuksen aikana; suorita tarkistus uudelleen.
prop-col-name = Nimi
menu-save-as = Tallenna nimellä…
menu-project = Projekti
menu-tools = Työkalut
menu-manual = Käyttöopas
msg-manual-missing = Käyttöopasta (PDF) ei löytynyt sovelluksen vierestä.
toolbar-new = Uusi
toolbar-open = Avaa
toolbar-save = Tallenna
toolbar-validate = Tarkista
toolbar-preview = Esikatselu
toolbar-export = Vie
dialog-choose-root = Valitse modin juurikansio
exit-unsaved-docs = Tallentamatta: { $names }
status-summary = { $steps } vaihetta · { $options } vaihtoehtoa
section-groups = Ryhmät
section-options = Vaihtoehdot
section-flags = Ehtoliput
section-files = Asennettavat tiedostot
hint-group-type = Miten asennusohjelma antaa käyttäjän valita tämän ryhmän vaihtoehtoja.
hint-default-type = Miten vaihtoehto tarjotaan, kun mikään riippuvuusmalli ei täsmää: pakollinen, valinnainen, suositeltu, ei käytettävissä…
hint-operator = Kaikkien ehtojen on täytyttävä (JA) tai minkä tahansa niistä (TAI).
hint-flags = Liput ovat nimettyjä arvoja, jotka tämä vaihtoehto asettaa valittaessa. Muut vaiheet ja vaihtoehdot voivat testata niitä näkyäkseen, piiloutuakseen tai tullakseen pakollisiksi.
hint-plugin-dependencies = Mallit, jotka muuttavat vaihtoehdon tyyppiä lippujen tai pelissä olevien tiedostojen mukaan: esimerkiksi ”Pakollinen”, kun toinen modi on asennettu.
hint-files = Tiedostot ja kansiot, jotka kopioidaan pelin Data-kansioon, kun tämä vaihtoehto valitaan. Kohde on suhteessa Data-kansioon; ristiriidassa korkeampi prioriteetti voittaa.
hint-visibility = Ehdot, joiden on täytyttävä, jotta tämä vaihe näytetään. Jätä tyhjäksi näyttääksesi sen aina.
seltype-exactly-one = Täsmälleen yksi (pakollinen)
seltype-at-most-one = Enintään yksi
seltype-any = Mikä tahansa määrä
seltype-all = Kaikki (ei valintaa)
seltype-at-least-one = Vähintään yksi
plugtype-required = Pakollinen
plugtype-optional = Valinnainen
plugtype-recommended = Suositeltu
plugtype-not-usable = Ei käytettävissä
plugtype-could-be-usable = Mahdollisesti käytettävissä
plugtype-required-hint = Asennetaan aina; käyttäjä ei voi poistaa valintaa.
plugtype-optional-hint = Tarjotaan valitsemattomana; käyttäjä päättää.
plugtype-recommended-hint = Tarjotaan valittuna; käyttäjä voi poistaa valinnan.
plugtype-not-usable-hint = Näytetään harmaana, eikä sitä voi valita.
plugtype-could-be-usable-hint = Valittavissa, mutta asennusohjelma varoittaa, ettei se ehkä toimi.
op-and = Kaikki ehdot (JA)
op-or = Mikä tahansa ehto (TAI)
theme-dark = Tumma
theme-light = Vaalea
theme-system = Järjestelmän mukaan
condeditor-setter-loc = Vaihe { "{step}" } / Ryhmä { "{group}" } / ”{ "{name}" }”
condeditor-pattern-of = Malli ”{ "{name}" }” → { "{type}" }
condeditor-visibility-of = Vaiheen { "{step}" } näkyvyys
condeditor-cond-set = Ehdollinen joukko { "{num}" }
condeditor-needs = { "{ctx}" } (vaatii = { "{value}" })
condeditor-file-dep = { "{ctx}" }: tiedosto ”{ "{name}" }” ({ "{state}" })
menu-translate-fomod = Käännä FOMOD…
ftr-title = Käännä FOMOD
ftr-open-folder = Avaa modikansio…
ftr-from-active = Aktiivisesta projektista
ftr-from-active-hint = Kääntää pääikkunassa avoinna olevan projektin FOMODin (se on ensin tallennettava).
ftr-no-fomod = FOMODia ei ole ladattu.
ftr-encoding = Alkuperäisten tiedostojen merkistökoodaus; käännetyt tiedostot kirjoitetaan samalla koodauksella.
ftr-source-lang = Kielestä
ftr-target-lang = kieleen
ftr-lang-locked = (kieliä ei voi vaihtaa, kun FOMOD on ladattu)
ftr-translator = Kääntäjä:
ftr-save = Tallenna käännös
ftr-export = Vie käännetyt tiedostot
ftr-export-sibling = Kansioon fomod_<kieli>
ftr-export-sibling-hint = Kirjoittaa käännetyt tiedostot info.xml ja ModuleConfig.xml alkuperäisen fomod-kansion viereen; alkuperäisiin tiedostoihin ei kosketa.
ftr-export-inplace = Alkuperäisten tiedostojen päälle
ftr-export-inplace-hint = Korvaa tiedostot fomod/info.xml ja fomod/ModuleConfig.xml sen jälkeen, kun kummastakin on tehty aikaleimattu .bak-kopio.
ftr-force-explicit-order = Säilytä alkuperäinen järjestys
ftr-warn-order = Nimen mukaan lajitellut luettelot (order="Ascending") lajiteltaisiin modienhallinnassa uudelleen käännettyjen nimien mukaan. Tämä pakottaa arvon order="Explicit", jotta vaihtoehdot säilyttävät nykyisen järjestyksensä.
ftr-update = Päivitä kansiosta
ftr-update-hint = Lukee FOMODin uudelleen levyltä ja yhdistää käännöksen siihen: uusista, muuttuneista ja poistetuista merkkijonoista ilmoitetaan.
ftr-preview-translated = Käännetty esikatselu
ftr-progress = { $done } / { $total } käännetty
ftr-filter-all = Kaikki
ftr-filter-untranslated = Kääntämättömät
ftr-filter-review = Tarkistettavat
ftr-filter-issues = Ongelmalliset
ftr-filter-locked = Lukitut
ftr-type-all = Kaikki kentät
ftr-type-names = Nimet
ftr-type-descriptions = Kuvaukset
ftr-type-meta = Modin tiedot
ftr-search-hint = Hae lähdetekstistä, käännöksestä tai kontekstista…
ftr-next-untranslated = Seuraava kääntämätön
ftr-show-whitespace = Näytä välilyönnit ja rivinvaihdot
ftr-discard-question = Nykyisessä käännöksessä on tallentamattomia muutoksia. Hylätäänkö ne ja ladataan toinen FOMOD?
ftr-discard-yes = Hylkää
ftr-unsaved-close = Käännöksessä on tallentamattomia muutoksia.
ftr-col-num = Nro
ftr-col-status = { "" }
ftr-col-context = Konteksti
ftr-col-source = Lähdeteksti
ftr-col-target = Käännös
ftr-col-issues = { "" }
ftr-empty-hint = Avaa modikansio tai lataa aktiivinen projekti, niin sen käännettävät merkkijonot luetellaan.
ftr-empty-filter = Mikään merkkijono ei vastaa nykyistä suodatinta.
ftr-select-row = Valitse rivi muokataksesi sen käännöstä.
ftr-copy-source = Kopioi lähdeteksti
ftr-clear-target = Tyhjennä
ftr-lock = Älä käännä
ftr-lock-hint = Lukitut merkkijonot kirjoitetaan muuttamattomina (tekijä, verkkosivusto, erisnimet…).
ftr-note = Huomautus:
ftr-status-untranslated = Kääntämätön
ftr-status-translated = Käännetty
ftr-status-auto = Esitäytetty automaattisesti — tarkista
ftr-status-fuzzy = Lähdeteksti on muuttunut kääntämisen jälkeen — tarkista
ftr-status-obsolete = Ei ole enää FOMODissa
ftr-status-locked = Lukittu (kirjoitetaan muuttamattomana)
ftr-field-info-name = Modin nimi (info.xml)
ftr-field-module-name = Asennusohjelman otsikko (ModuleConfig.xml)
ftr-field-author = Tekijä
ftr-field-website = Verkkosivusto
ftr-field-description = Modin kuvaus
ftr-field-step = Vaiheen nimi
ftr-field-group = Ryhmän nimi
ftr-field-plugin = Vaihtoehdon nimi
ftr-field-plugin-desc = Vaihtoehdon kuvaus
ftr-issue-empty = Tyhjä käännös
ftr-issue-whitespace = Käännös sisältää vain välilyöntejä
ftr-issue-edge-whitespace = Alun tai lopun välilyönnit poikkeavat lähdetekstistä
ftr-issue-token = Suojatut tunnisteet poikkeavat — puuttuu: { $missing } ; ylimääräisiä: { $extra }
ftr-issue-newline-name = Nimessä ei voi olla rivinvaihtoa
ftr-issue-control = Sisältää merkkejä, joita XML ei voi tallentaa
ftr-issue-length = Epätavallinen pituus lähdetekstiin verrattuna (×{ $ratio })
ftr-issue-identical = Sama kuin lähdeteksti
ftr-issue-duplicate = Sama lähdeteksti on käännetty eri tavalla kohdassa { $key }
ftr-issue-cdata = Merkkijono ]]> ei ole sallittu tässä
ftr-load-error = FOMODin lataaminen epäonnistui: { $error }
ftr-extracted = Käännettäviä merkkijonoja löytyi: { $num }.
ftr-sidecar-found = Aiempi käännös ladattu ja yhdistetty: uusia { $new }, muuttuneita { $changed }, poistettuja { $removed }.
ftr-saved = Käännös tallennettu kohteeseen { $path }
ftr-save-error = Käännöksen tallentaminen epäonnistui: { $error }
ftr-save-first = Tallenna ensin projekti ja käännä se sitten.
ftr-export-success = Kohteeseen { $path } kirjoitettuja merkkijonoja: { $count }
ftr-export-error = Vienti epäonnistui: { $error }
ftr-export-blocked = Ennen vientiä on korjattava estäviä ongelmia: { $num }.
ftr-export-stale = FOMODin muuttumisen vuoksi ohitettuja merkkijonoja: { $num }; käytä toimintoa ”Päivitä kansiosta”.
ftr-update-report = Päivitetty: uusia { $new }, muuttuneita { $changed }, siirrettyjä { $moved }, poistettuja { $removed }, muuttumattomia { $unchanged }.
menu-edit = Muokkaa
menu-undo = Kumoa
menu-redo = Tee uudelleen
tree-title = Projekti
tree-mod-info = Modin tiedot
tree-steps = Asennusvaiheet
tree-required = Pakolliset tiedostot
tree-conditional = Ehdolliset asennukset
tree-empty-steps = Ei vielä vaiheita — lisää vaihe napsauttamalla +.
tree-duplicate = Monista
tree-delete = Poista
tree-save-template = Tallenna mallina…
tree-drop-hint = Siirrä pudottamalla tähän
cond-set-label = Ehdollinen joukko { $num }
inspector-empty = Valitse kohde projektipuusta tai aloita lisäämällä vaihe.
count-options = Vaihtoehtoja: { $num }
count-files = Tiedostoja: { $num }
msg-deleted-undo = Poistettu. Voit palauttaa sen Kumoa-toiminnolla (Ctrl+Z).
problems-title = Ongelmat
problems-errors = Virheitä: { $num }
problems-warnings = Varoituksia: { $num }
btn-close = Sulje
ftr-export-package = Käännöspakettina (arkisto)
ftr-export-package-hint = Luo ladattavaksi valmiin .zip- tai .7z-tiedoston: käännetyt info.xml ja ModuleConfig.xml sekä README (vain korjaus) tai koko modin käännettyine tiedostoineen (täysi).
ftr-package-full = Koko modi
ftr-package-full-hint = Sisällytä arkistoon kaikki modin tiedostot, ei vain kahta käännettyä XML-tiedostoa. Varmista, että tekijä sallii uudelleenjakelun.
ftr-package-name-template = Nimi:
ftr-readme-patch = Tämä arkisto sisältää modin ”{ $name }” asennusohjelman käännöksen (kieli: { $langname }; tiedostot fomod/info.xml ja fomod/ModuleConfig.xml). Asenna se alkuperäisen modin päälle tai anna modienhallinnan yhdistää se, jotta käännetyt tiedostot korvaavat alkuperäiset. Vain asennusohjelman tekstit muuttuvat; itse modin tiedostot eivät ole mukana. Tehty XIMOD Architect -ohjelmalla.
ftr-readme-full = Tämä arkisto sisältää modin ”{ $name }”, jonka asennusohjelma on käännetty (kieli: { $langname }; tiedostot fomod/info.xml ja fomod/ModuleConfig.xml). Asenna se kuten alkuperäinen modi. Vain asennusohjelman tekstejä on muutettu. Tehty XIMOD Architect -ohjelmalla.
ftr-apply-memory = Täytä muistista
ftr-memory-size = Käännösmuisti — merkintöjä tälle kieliparille: { $num }. Jokainen tallennettu käännös lisätään siihen.
ftr-memory-applied = Käännösmuistista täytettyjä merkkijonoja (merkitty tilaan ”tarkistettavat”): { $num }.
ftr-memory-suggestion = Muisti ehdottaa:
ftr-use-suggestion = Käytä
ftr-propagate = Kopioi samanlaisiin
ftr-propagate-hint = Kopioi tämän käännöksen kaikkiin muihin vielä kääntämättömiin merkkijonoihin, joilla on sama lähdeteksti.
ftr-propagated = Täytettyjä samanlaisia merkkijonoja: { $num }.
ftr-csv-export = Vie CSV…
ftr-csv-import = Tuo CSV…
ftr-csv-imported = CSV-tiedostosta päivitettyjä merkkijonoja: { $num }.
ftr-csv-error = CSV-virhe: { $error }
ftr-glossary = Sanasto
ftr-glossary-source = Termi
ftr-glossary-target = Käännös
ftr-glossary-case = Kirjainkoko
ftr-glossary-dnt = Säilytä
ftr-glossary-add = Lisää termi
ftr-issue-glossary = Sanasto: ”{ $term }” ei ole käännetty odotetulla tavalla

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Avaa arkisto…
filter-archive = Modiarkistot (zip, 7z)
msg-archive-opened = Arkisto avattu (purettuja tiedostoja: { $num }): { $path }
msg-archive-reused = Arkisto on jo purettu, käytetään uudelleen { $path }
msg-archive-unsupported = Arkistomuotoa ”.{ $ext }” ei tueta; pura se ensin 7-Zip-ohjelmalla (vain .zip ja .7z voidaan avata).
msg-archive-error = Virhe arkistoa avattaessa: { $error }
msg-archive-no-fomod = Arkistosta ei löytynyt ”fomod”-kansiota ({ $path })
msg-archive-extracting = Puretaan arkistoa…
ftr-open-archive = Avaa modiarkisto…
ftr-package-full-partial = Modi avattiin arkistosta, joka sisältää vain sen fomod-kansion; täydet paketit vaativat puretun modin.
info-module-deps = Modin vaatimukset
info-module-deps-hint = Tiedostot tai liput, joita koko modi vaatii ennen asennusohjelman käynnistymistä (moduleDependencies). Jätä tyhjäksi, jos niitä ei ole.
info-header-advanced = Otsakkeen lisäasetukset
info-title-position = Otsikon sijainti
info-title-colour = Otsikon väri
info-title-colour-hint = Odotetaan: kuusi heksadesimaalinumeroa (RRGGBB)
info-image-show = Näytä otsakekuva
info-image-fade = Häivytä otsakekuva
info-image-height = Otsakekuvan korkeus
info-attr-default = (oletus)
file-always-install = Aina
file-always-install-hint = Asenna tämä tiedosto aina, myös kun vaihtoehtoa ei ole valittu (alwaysInstall).
file-install-if-usable = Jos kelpaa
file-install-if-usable-hint = Asenna tämä tiedosto aina, kun vaihtoehto on käyttökelpoinen, myös kun sitä ei ole valittu (installIfUsable).
msg-import-lossy = Tämä FOMOD sisältää rakenteita, joita XIMOD ei voi muokata (määrä: { $num }); ne hylätään projektia tallennettaessa.
fidelity-nested-deps = Sisäkkäinen riippuvuusryhmä kohteessa { $context } (vain yksi taso tuetaan)
fidelity-game-dep = Pelin versiovaatimus { $version } kohteessa { $context }
fidelity-fomm-dep = Modinhallintaohjelman versiovaatimus { $version } kohteessa { $context }
fidelity-unknown = Elementtiä ”{ $element }” elementissä ”{ $parent }” ei tueta ({ $context })
loc-module = modin vaatimukset
loc-step = vaihe { $step } ”{ $name }”
loc-installer = asennusohjelma

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Palauta varmuuskopio…
backups-title = Palauta varmuuskopio
backups-empty = Tällä projektilla ei ole vielä varmuuskopiota. Sellainen luodaan aina, kun projekti tallennetaan aiemman version päälle.
backups-changes = Muutoksia nykyiseen projektiin verrattuna: { $num }
btn-compare = Vertaa
btn-restore = Palauta
btn-delete-backups = Poista kaikki varmuuskopiot
btn-delete-backups-confirm = Napsauta uudelleen poistaaksesi kaikki varmuuskopiot
msg-backup-restored = Varmuuskopio ajalta { $time } palautettu editoriin (ei vielä tallennettu; Kumoa peruu sen)
msg-backups-deleted = Poistettuja varmuuskopioita: { $num }
settings-backup-count = Säilytettävät varmuuskopiot:
settings-backup-count-hint = Kuinka monta aiempaa versiota FOMOD-XML:stä säilytetään kansiossa fomod/backups tallennettaessa (0 = ei varmuuskopioita).
settings-autosave-minutes = Tallenna palautuskopio automaattisesti joka (minuuttia):
settings-autosave-minutes-hint = Jokaisesta muutetusta projektista kirjoitetaan tällä aikavälillä palautuskopio asetuskansioon; sitä tarjotaan seuraavassa käynnistyksessä vain epänormaalin sulkeutumisen jälkeen (0 = pois).
settings-auto-masters = Lisää liitännäisen master-tiedostot ehdoiksi
settings-auto-masters-hint = Kun liitännäinen (.esp/.esm/.esl) lisätään vaihtoehtoon, sen vaatimista master-tiedostoista ne, joita peli tai tämä modi ei tarjoa, muuttuvat vaihtoehdon ”Active”-tiedostoehdoiksi.
msg-author-from-plugin = Tekijä täytetty liitännäisen otsakkeesta: { $author }
msg-masters-added = Liitännäisen { $plugin } master-tiedostoja lisätty tiedostoehdoiksi: { $num }
issue-missing-master = { $plugin } vaatii tiedoston { $master }, joka ei ole tässä modissa eikä sitä ole ilmoitettu riippuvuudeksi
issue-esl-mismatch-flag = Tiedostolla { $plugin } on .esl-pääte, mutta sen light-lippua (ESL) ei ole asetettu
issue-esl-eligible = { $plugin } voitaisiin merkitä light-liitännäiseksi (uusia tietueita: { $num }, raja { $limit })
issue-esl-too-big = { $plugin } on merkitty light-liitännäiseksi, mutta ei täytä light-liitännäisten sääntöjä (uusia tietueita: { $num }, raja { $limit }, tai FormID sallitun alueen ulkopuolella)
menu-plugin-report = Liitännäisraportti…
plugins-title = Liitännäisraportti
plugins-file = Tiedosto
plugins-kind = Tyyppi
plugins-light = Light-lippu
plugins-masters = Master-tiedostot
plugins-new-records = Uusia tietueita / raja
plugins-eligible = Light-kelpoinen
plugins-empty = Tämä projekti ei asenna yhtään liitännäistiedostoa (.esp, .esm tai .esl).
plugins-unreadable = lukukelvoton

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Lopullinen tiedostopuu
preview-total-size = Asennuksen kokonaiskoko: { $size }
preview-tree-truncated = Puu on katkaistu: liian monta tiedostoa laajennettavaksi (yllä olevat koot ovat osittaisia).
preview-overwritten-by = Ylikirjoittanut { $plugin }
preview-scenario = Skenaario:
preview-scenario-load = Lataa
preview-scenario-save = Tallenna…
preview-scenario-delete = Poista
preview-scenario-name = Skenaarion nimi
preview-scenario-saved = Skenaario ”{ $name }” tallennettu kansioon fomod/scenarios
preview-scenario-unresolved = Skenaarion valintoja, jotka eivät vastaa mitään tämän projektin vaihtoehtoa (nimetty uudelleen tai poistettu): { $num }
preview-scenario-none = (ei skenaariota)
issue-unreachable-step = Vaihetta ”{ $step }” ei voida koskaan näyttää: sen näkyvyysehdot testaavat lipun arvoa, jota mikään aiempi vaihtoehto ei aseta
issue-unreachable-option = Vaihtoehtoa ”{ $plugin }” ei voida koskaan valita: sen käytettävän tyypin mallit testaavat lipun arvoa, jota mikään vaihtoehto ei aseta
issue-unreachable-cond = Ehdollinen tiedostojoukko { $num } ei voi koskaan tulla voimaan: sen ehdot testaavat lipun arvoa, jota mikään vaihtoehto ei aseta
size-option = Asennuksen koko: { $size } (tiedostoja: { $num })
size-missing = Puuttuvia lähteitä: { $num }
size-unknown = Asennuksen koko: — (mittaa suorittamalla Tarkista)
menu-nexus-desc = Nexus-kuvaus…
nexus-title = Nexus Mods -kuvaus
nexus-format = Muoto:
nexus-include-requirements = Vaatimukset
nexus-include-options = Asennusvaihtoehdot
nexus-include-install = Asennus
nexus-include-changelog = Muutosloki
nexus-previous = Edellinen versio…
nexus-previous-none = (ei edellistä versiota: ei muutoslokia)
nexus-language = Kieli:
nexus-language-source = (lähde)
nexus-sec-requirements = Vaatimukset
nexus-sec-options = Asennusvaihtoehdot
nexus-sec-install = Asennus
nexus-sec-changelog = Muutosloki
nexus-install-text = Tämän modin mukana tulee FOMOD-asennusohjelma: asenna se modinhallintaohjelmalla (Vortex, Mod Organizer 2) ja valitse vaihtoehtosi asennusohjelmassa.
nexus-requires = Vaatii
nexus-step = Vaihe
nexus-added = Lisätty
nexus-removed = Poistettu
nexus-changed = Muutettu
btn-copy = Kopioi
btn-save-as = Tallenna nimellä…
msg-copied = Kopioitu leikepöydälle
msg-saved-to = Tallennettu kohteeseen { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Nimeä uudelleen…
condeditor-rename-exists = Lippu nimeltä ”{ $name }” on jo olemassa
condeditor-renamed = Lippu ”{ $from }” nimettiin uudelleen nimelle ”{ $to }” (esiintymiä: { $num })
condeditor-delete-uses = Poista kaikki käyttökohdat
condeditor-deleted-uses = Lippu ”{ $name }” poistettiin kaikkialta (esiintymiä: { $num })
condeditor-values-set = Asetetut arvot:
condeditor-values-tested = Testatut arvot:
condeditor-value-never-set = { $value } — testataan, mutta ei koskaan aseteta
condeditor-value-never-tested = { $value } — asetetaan, mutta ei koskaan testata
condeditor-builder = Ehtojen rakennin
condeditor-builder-none = Valitse pääikkunassa vaihe, vaihtoehto, ehdollinen tiedostojoukko tai modin tiedot muokataksesi sen ehtoja tässä.
condeditor-builder-pattern = Malli:
condeditor-sentence-if = JOS
condeditor-sentence-and = JA
condeditor-sentence-or = TAI
condeditor-sentence-flag = lippu { "{name}" } = { "{value}" }
condeditor-sentence-file = tiedosto { "{name}" } on { "{value}" }
condeditor-sentence-empty = (ei ehtoa: aina tosi)
condeditor-sentence-then-visible = NIIN vaihe näytetään
condeditor-sentence-then-type = NIIN vaihtoehdosta tulee { $type }
condeditor-sentence-then-install = NIIN tiedostot asennetaan
condeditor-sentence-then-module = NIIN asennusohjelma voidaan suorittaa (tarkistetaan ennen käynnistystä)
issue-flag-value-never-set = Lippua ”{ $flag }” testataan arvolla ”{ $value }”, jota mikään vaihtoehto ei aseta
issue-flag-never-used = Lippu ”{ $flag }” asetetaan, mutta sitä ei testata missään
menu-project-strings = Projektin merkkijonot…
strings-title = Projektin merkkijonot
strings-search = Hae tekstiä, sijaintia tai avainta…
strings-kind-all = Kaikki
strings-kind-names = Nimet
strings-kind-descriptions = Kuvaukset
strings-duplicates-only = Vain kaksoiskappaleet
strings-replace-with = Korvaa tekstillä:
strings-case = Sama kirjainkoko
strings-whole-word = Koko sana
strings-replace-current = Korvaa
strings-replace-all = Korvaa kaikki
strings-replaced = Korvattuja merkkijonoja: { $num }
strings-dup-badge = ×{ $num }
strings-dup-hover = Sama teksti kuin:
strings-count = Merkkijonoja: { $num } · kaksoiskappaleryhmiä: { $dups }
strings-col-location = Sijainti
strings-col-field = Kenttä
strings-col-text = Teksti

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Arkiston sisältö…
filter-bethesda-archive = Bethesda-arkistot (bsa, ba2)
archive-view-title = Arkiston sisältö
archive-view-format = Muoto:
archive-view-entries = Merkintöjä: { $num }
archive-view-size = { $size } purettuna
archive-view-search = Hae polkua…
archive-view-col-path = Polku
archive-view-col-size = Koko
archive-view-col-compressed = Pakattu
archive-view-truncated = Vain ensimmäiset { $num } osuvaa merkintää näytetään – tarkenna hakua.
archive-view-error = Tätä arkistoa ei voi lukea: { $error }
archive-view-hint = Näytä tämän arkiston sisältö
issue-conflict-archive = Sama resurssi useassa arkistossa: ”{ $path }” on pakattu { $count } viittauksen kautta ({ $locs }) – pelin arkistojen latausjärjestys ratkaisee, mitä niistä käytetään.
issue-conflict-archive-loose = Arkisto vastaan irrallinen tiedosto: ”{ $path }” on sekä pakattu arkistoon että asennettu irrallisena tiedostona ({ $locs }) – irrallinen tiedosto ohittaa arkistoidun.
preview-in-archive = (arkistossa)
preview-archived-size = josta { $size } pakattuna arkistoihin

# --- Project tree: expand / collapse menus
tree-expand = Laajenna
tree-collapse = Kutista
tree-expand-all = Laajenna kaikki
tree-expand-selected = Laajenna valittu
tree-expand-from = Laajenna valitusta alkaen
tree-collapse-all = Kutista kaikki
tree-collapse-selected = Kutista valittu
tree-collapse-from = Kutista valitusta alkaen
tree-expand-all-hint = Laajentaa kaikki otsikot
tree-expand-selected-hint = Laajentaa vain valitun otsikon
tree-expand-from-hint = Laajentaa valitun otsikon ja kaiken sen alla olevan
tree-collapse-all-hint = Kutistaa kaikki otsikot
tree-collapse-selected-hint = Kutistaa vain valitun otsikon
tree-collapse-from-hint = Kutistaa valitun otsikon ja kaiken sen alla olevan

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Lisää ryhmä
btn-remove-group-cond = Poista ryhmä
dep-type-game = Pelin versio
dep-type-fomm = Modinhallintaohjelman versio
dep-group-hint = Ryhmä ehtoja, jotka yhdistetään JA / TAI -operaattorilla; ryhmiä voi asettaa sisäkkäin.
condeditor-sentence-game = pelin versio ≥ { "{value}" }
condeditor-sentence-fomm = modinhallintaohjelman versio ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Yksilölliset tekstit
ftr-uniques-hint = Näyttää yhden rivin kutakin erillistä lähdetekstiä kohden. Rivin kääntäminen kääntää kerralla kaikki samansisältöiset merkkijonot.
ftr-uniques-synced = Päivitettyjä samanlaisia merkkijonoja: { $num }.
ftr-uniques-group = Tämän tekstin jakavia merkkijonoja: { $num }; sen käännös koskee niitä kaikkia.
