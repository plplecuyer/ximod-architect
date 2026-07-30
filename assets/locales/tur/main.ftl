# XIMOD Architect - translation metadata
# @language = tur
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = Türkçe
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Sürüm { $version }

# Status messages
status-ready = Hazır
msg-save-success = FOMOD başarıyla kaydedildi
msg-save-error = FOMOD'u kaydederken hata oluştu
msg-export-success = Dağıtım arşivi oluşturuldu ({ $count } dosya): { $path }
msg-export-error = Dağıtım arşivini oluştururken hata oluştu: { $error }
msg-load-success = FOMOD başarıyla yüklendi
msg-load-error = FOMOD yüklenirken hata oluştu
msg-merge-success = FOMOD başarıyla birleştirildi
msg-merge-error = FOMOD birleştirilirken hata oluştu
msg-no-root-selected = Lütfen önce bir kök dizin seçin
msg-no-fomod-folder = 'fomod' klasörü bulunamadı. Oluşturmak ister misiniz?
msg-file-outside-root = Dosya kök dizinin dışında

# Menu - File
menu-file = Dosya
menu-new = Yeni
menu-open = Klasörü Aç...
menu-open-file = Dosyayı Aç...
menu-save = Kaydet
menu-recent = Son Kullanılanlar
menu-exit = Çıkış
menu-merge = FOMOD'u Birleştir...
menu-export = Dağıtım arşivini dışa aktar...

# Menu - Options
menu-options = Seçenekler
menu-settings = Ayarlar
menu-pre-save-script = Kaydetme Öncesi Komut Dosyası...
menu-post-save-script = Kaydetme Sonrası Komut Dosyası...
menu-translation = Çeviri...

# Menu - Help
menu-help = Yardım
menu-about = Hakkında

# Tabs
tab-info = Mod Bilgisi
tab-steps = Kurulum Adımları
tab-required = Gerekli Kurulumlar
tab-conditional = Koşullu Kurulumlar

# Info Tab
label-workspace = Çalışma Alanı
label-root-dir = Kök Dizin:
label-mod-name = Mod Adı:
label-author = Yazar:
label-version = Sürüm:
label-game-name = Oyun Adı:
label-category = Kategori:
label-url = Web Sitesi URL'si:
label-header-image = Başlık Görüntüsü:
label-description = Açıklama:
placeholder-select-dir = (Bir dizin seçin)
placeholder-select-game = (Bir oyun seçin)

# Steps Tab
label-step-name = Adım Adı:
label-group-name = Grup Adı:
label-group-type = Grup Türü:
label-plugin-name = Eklenti Adı:
label-plugin-desc = Açıklama:
label-plugin-type = Varsayılan Tür:
label-plugin-image = Resim:
label-visibility = Görünürlük Koşulları
label-operator = İşleç:

# Buttons
btn-browse = Gözat...
btn-clear = Temizle
btn-add = Ekle
btn-remove = Kaldır
btn-add-step = Yeni Adım
btn-delete-step = Adımı Sil
btn-add-group = Grup Ekle
btn-remove-group = Grubu Kaldır
btn-add-plugin = Eklenti Ekle
btn-remove-plugin = Eklentiyi Kaldır
btn-add-file = Dosya Ekle
btn-add-folder = Klasör Ekle
btn-remove-file = Kaldır
btn-add-flag = İşaret Ekle
btn-remove-flag = İşareti Kaldır
btn-add-condition = Koşul Ekle
btn-remove-condition = Koşulu Kaldır
btn-add-dependency = Bağımlılık Ekle
btn-remove-dependency = Bağımlılığı Kaldır
btn-add-pattern = Yeni Desen
btn-remove-pattern = Deseni Sil
btn-save = Kaydet
btn-cancel = İptal
btn-ok = Tamam
btn-yes = Evet
btn-no = Hayır

# Condition/Dependency Labels
label-flag-name = Bayrak Adı:
label-flag-value = Değer:
label-condition-type = Tür:
label-condition-name = Ad:
label-condition-value = Değer:
label-dep-type = Bağımlılık Türü:
label-dep-name = Ad/Dosya:
label-dep-value = Değer/Durum:

# Files
label-source = Kaynak
label-destination = Hedef
label-priority = Öncelik
label-file-type = Tür
label-files = Dosyalar
label-dependencies = Bağımlılıklar

# Settings Dialog
settings-title = Ayarlar
settings-tab-general = Genel
settings-tab-recent-files = Son Kullanılan Dosyalar
settings-language = Dil:
settings-theme = Tema:
settings-font-size = Yazı Tipi Boyutu:
settings-replace-newlines = Açıklamalardaki satır sonlarını işleme
settings-max-recent = Son Dosyaların Maksimum Sayısı:
settings-window-width = Pencere Genişliği:
settings-window-height = Pencere Yüksekliği:
settings-no-recent-files = Son dosya yok.

# Status messages for settings
status-settings-saved = Ayarlar başarıyla kaydedildi

# About Dialog
about-title = XIMOD Architect Hakkında
about-description = Bethesda oyun modları için platformlar arası bir FOMOD yükleyici oluşturma aracı.
about-license = MIT Lisansı altında lisanslanmıştır
about-copyright = © 2025-2026 XIMOD Ekibi
about-credit = Wenderer'in orijinal aracının Rust portu:

# Script Dialog
script-title = Komut Dosyasını Düzenle
script-info = Komut dosyaları, kaydetme işleminden önce veya sonra çalıştırılır. Aşağıdaki makroları kullanabilirsiniz:
script-macros = Kullanılabilir Makrolar:
macro-modname = $MODNAME$ - Mod adı
macro-modauthor = $MODAUTHOR$ - Yazar adı
macro-modversion = $MODVERSION$ - Mod sürümü
macro-modroot = $MODROOT$ - Kök dizin yolu
macro-date = $DATE$ - Güncel tarih (YYYY-MM-DD)
macro-time = $TIME$ - Güncel saat (SS:DD:SS)
macro-random = $RANDOM$ - Rastgele sayı

# Plugin Dependencies
label-default-type = Varsayılan Tür:
label-pattern-type = Desen Türü:
label-pattern-operator = Desen İşleci:

# Conditional Files
label-pattern = Desen

# Validation Messages
validation-no-name = Mod adı gereklidir
validation-no-steps = En az bir adım veya gerekli dosya gereklidir
validation-empty-step = { $num } numaralı adımın adı yok
validation-empty-group = { $step } numaralı adımın, { $group } numaralı grubun adı yok
validation-no-plugins = { $step } numaralı adımın, "{ $name }" numaralı grubun eklentisi yok

# File States
state-active = Etkin
state-inactive = Etkin değil
state-missing = Eksik

# Confirmation
confirm-title = Onay
confirm-delete = Bu öğeyi silmek istediğinizden emin misiniz?
confirm-discard = Kaydedilmemiş değişiklikleriniz var. Bunları iptal edip devam etmek ister misiniz?
confirm-unsaved = Kaydedilmemiş değişiklikleriniz var. Kapatmadan önce kaydetmek ister misiniz?
confirm-save-issues = Projede aşağıdaki sorunlar var:
confirm-save-anyway = Yine de kaydetmek ister misiniz?

# Errors
error-invalid-xml = Geçersiz XML dosyası
error-parse-failed = FOMOD'u ayrıştırılamadı
error-write-failed = Dosya yazma işlemi başarısız
error-create-dir = Dizin oluşturulamadı

# Default names (generated when creating new items)
default-step-name = Adım { $num }
default-group-name = Grup { $num }
default-plugin-name = Eklenti { $num }
pattern-label = Desen { $num }

# Selection prompts
msg-select-group-first = Önce bir grup seçin.
msg-select-plugin-edit = Düzenlemek için bir eklenti seçin.
label-empty = (boş)
image-no-image = Resim yok

# File dialog filters
filter-images = Görüntüler
filter-xml = XML

# Dependency types
dep-type-flag = Bayrak
dep-type-file = Dosya

# Status bar
status-modified = Değiştirildi

# Status messages (errors)
msg-settings-save-error = Ayarları kaydetme hatası
msg-script-save-error = Komut dosyasını kaydetme hatası

# Translation editor
trans-title = Çeviri Düzenleyicisi
trans-source-lang = Görüntülenen dil:
trans-target-lang = Çevrilecek dil:
trans-col-key = Anahtar
trans-col-source = Etiket
trans-col-target = Çeviri
trans-saved = Çeviri kaydedildi
trans-save-error = Çeviri kaydedilirken hata oluştu

# XML editor
xml-editor-title = XML Düzenleyicisi
xml-editor-edit = Düzenle
xml-editor-apply = Uygula
xml-editor-revert = İptal
xml-editor-readonly = Salt okunur
xml-editor-editing = Düzenleme — grafik sekmeleri kilitli
xml-editor-error = Hata:
xml-editor-applied = XML değişiklikleri uygulandı
xml-editor-wellformed = Biçim kurallarına uygun XML
xml-editor-error-at = Satır { $line }, sütun { $col }: { $msg }

# Country / flag picker
settings-country-name = Ülke adı:
settings-pick-country = Ülkenizi seçmek için tıklayın
flags-title = Bir ülke seçin
flags-filter = Filtre:
flags-none = Bayrak bulunamadı

# Translation editor: country & font
trans-endonym = Ülke adı:
trans-font = Yazı tipi:
trans-no-font = (yok)
trans-browse = Gözat…
trans-google-fonts = Google Yazı Tipleri
trans-pick-country = Ülkeyi seçmek için tıklayın
trans-font-outside = Yazı tipi önce assets/fonts klasörüne yüklenmelidir.
trans-font-dir-missing = assets/fonts klasörü bulunamadı.

# Translation submission
trans-lang-endonym = Dil adı:
trans-author = Yazar:
trans-submit = Gönder…
trans-submit-hint = Bir zip dosyası oluşturun ve önceden doldurulmuş e-postayı açın
trans-data-updated = Referans verileri güncellendi (Languages.json / Countries.json)
trans-package-ready = Arşiv hazır:
trans-package-error = Arşiv oluşturulamadı:

# ISO 639-3 requirement
trans-lang-not-iso = Çeviri yalnızca ISO 639-3 kodu olan diller için mümkündür.

# FOMOD installer preview
menu-preview = Yükleyiciyi önizle…
preview-title = FOMOD yükleyici önizlemesi
preview-refresh = Yenile
preview-assumptions = Dosya varsayımları
preview-details = Ayrıntılar
preview-back = Geri
preview-next = İleri
preview-install = Yükle
preview-close = Kapat
preview-restart = Yeniden başlat
preview-summary-title = Yüklenecek dosyalar
preview-empty = Yüklenecek dosya yok.
preview-none-option = (yok)
preview-invalid = Devam etmek için gerekli seçimleri tamamlayın.
preview-no-steps = Görünür adım yok; kurulum özetine bakın.
preview-select-hint = Açıklamasını görmek için bir seçenek seçin.
preview-col-source = Kaynak
preview-col-dest = Hedef
preview-col-priority = Öncelik
preview-sel-exactlyone = Tam olarak bir seçenek seçin.
preview-sel-atmostone = En fazla bir seçenek seçin.
preview-sel-any = İstediğiniz sayıda seçenek seçin.
preview-sel-all = Tüm seçenekler yüklenir.
preview-sel-atleastone = En az bir seçenek seçin.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = FOMOD'u doğrula
validate-report-title = FOMOD doğrulaması
validate-ok = Herhangi bir sorun bulunamadı. FOMOD, şemaya uygundur.
xml-editor-schema-ok = ModConfig 5.0 şemasına uygundur.
xml-editor-schema-issues = Şema sorunları:
schema-line-col = Satır { $line }, sütun { $col }: { $msg }
schema-wrong-root = Beklenmeyen kök "{ $found }" (beklenen "{ $expected }").
schema-unknown = "{ $parent }" içinde beklenmeyen "{ $element }" öğesi.
schema-missing = "{ $parent }" içinde "{ $child }" bulunmalıdır.
schema-needs-one = "{ $parent }" içinde en az bir "{ $child }" bulunmalıdır.
schema-too-many = "{ $child }", "{ $parent }" içinde yalnızca bir kez görünebilir.
schema-missing-attr = "{ $element }" öğesinde "{ $attr }" özniteliği zorunludur.
schema-bad-enum = { $element }/@{ $attr } için "{ $value }" değeri geçersiz (beklenen: { $allowed }).
schema-choose-one = "{ $parent }", { $options } öğelerinden tam olarak birini içermelidir.

# Reordering (steps / groups / plugins)
reorder-before = Önüne taşı
reorder-after = Arkasına taşı

# Country / language database explorer (Properties)
menu-properties = Özellikler…
prop-title = Ülke / dil veritabanı
prop-tab-countries = Ülkeler
prop-tab-languages = Diller
prop-filter = Filtre:
prop-official-langs = Resmi diller
prop-spoken-langs = Konuşulan diller
prop-endonym = Ülke adı
prop-font = Yazı tipi
prop-spoken-in = Konuşulduğu yer
prop-select-country = Ayrıntılarını görmek için bir ülke seçin.
prop-select-lang = Ayrıntılarını görmek için bir dil seçin.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Oyunun Nexus Mods sayfasını aç
