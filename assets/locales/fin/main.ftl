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
menu-settings = Asetukset
menu-pre-save-script = Skripti ennen tallennusta…
menu-post-save-script = Skripti tallennuksen jälkeen…
menu-translation = Käännös…

# Menu - Help
menu-help = Ohje
menu-about = Tietoja

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
label-plugin-name = Liitännäisen nimi:
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
btn-add-plugin = Lisää liitännäinen
btn-remove-plugin = Poista liitännäinen
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
validation-no-plugins = Vaiheella { $step }, ryhmällä ”{ $name }” ei ole liitännäisiä

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
default-plugin-name = Liitännäinen { $num }
pattern-label = Malli { $num }

# Selection prompts
msg-select-group-first = Valitse ensin ryhmä.
msg-select-plugin-edit = Valitse muokattava liitännäinen.
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
