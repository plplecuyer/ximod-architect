# XIMOD Architect - translation metadata
# @language = ara
# @font = Noto_Sans_Arabic/static/NotoSansArabic-Regular.ttf
# @langname = العربية
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = الإصدار { $version }

# Status messages
status-ready = جاهز
msg-save-success = تم حفظ FOMOD بنجاح
msg-save-error = خطأ في حفظ FOMOD
msg-export-success = تم إنشاء أرشيف التوزيع ({ $count } ملفات): { $path }
msg-export-error = خطأ في إنشاء أرشيف التوزيع: { $error }
msg-load-success = تم تحميل FOMOD بنجاح
msg-load-error = حدث خطأ أثناء تحميل FOMOD
msg-merge-success = تم دمج FOMOD بنجاح
msg-merge-error = حدث خطأ أثناء دمج FOMOD
msg-no-root-selected = يرجى تحديد دليل جذر أولاً
msg-no-fomod-folder = لم يتم العثور على مجلد «fomod». هل تريد إنشاء واحد؟
msg-file-outside-root = الملف خارج دليل الجذر

# Menu - File
menu-file = ملف
menu-new = جديد
menu-open = فتح مجلد...
menu-open-file = فتح ملف...
menu-save = حفظ
menu-recent = الأحدث
menu-exit = خروج
menu-merge = دمج FOMOD...
menu-export = تصدير أرشيف التوزيع...

# Menu - Options
menu-options = خيارات
menu-settings = إعدادات
menu-pre-save-script = برنامج نصي قبل الحفظ...
menu-post-save-script = برنامج نصي بعد الحفظ...
menu-translation = ترجمة...

# Menu - Help
menu-help = المساعدة
menu-about = حول

# Tabs
tab-info = معلومات التعديل
tab-steps = خطوات التثبيت
tab-required = التثبيتات المطلوبة
tab-conditional = التثبيتات المشروطة

# Info Tab
label-workspace = مساحة العمل
label-root-dir = الدليل الجذري:
label-mod-name = اسم التعديل:
label-author = المؤلف:
label-version = الإصدار:
label-game-name = اسم اللعبة:
label-category = الفئة:
label-url = عنوان URL للموقع الإلكتروني:
label-header-image = صورة العنوان:
label-description = الوصف:
placeholder-select-dir = (اختر دليلًا)
placeholder-select-game = (اختر لعبة)

# Steps Tab
label-step-name = اسم الخطوة:
label-group-name = اسم المجموعة:
label-group-type = نوع المجموعة:
label-plugin-name = اسم المكون الإضافي:
label-plugin-desc = الوصف:
label-plugin-type = النوع الافتراضي:
label-plugin-image = الصورة:
label-visibility = شروط الظهور
label-operator = المشغل:

# Buttons
btn-browse = استعراض...
btn-clear = مسح
btn-add = إضافة
btn-remove = إزالة
btn-add-step = خطوة جديدة
btn-delete-step = حذف خطوة
btn-add-group = إضافة مجموعة
btn-remove-group = إزالة مجموعة
btn-add-plugin = إضافة مكون إضافي
btn-remove-plugin = إزالة مكون إضافي
btn-add-file = إضافة ملف
btn-add-folder = إضافة مجلد
btn-remove-file = إزالة
btn-add-flag = إضافة علامة
btn-remove-flag = إزالة علامة
btn-add-condition = إضافة شرط
btn-remove-condition = إزالة شرط
btn-add-dependency = إضافة تبعية
btn-remove-dependency = إزالة تبعية
btn-add-pattern = نمط جديد
btn-remove-pattern = حذف نمط
btn-save = حفظ
btn-cancel = إلغاء
btn-ok = موافق
btn-yes = نعم
btn-no = لا

# Condition/Dependency Labels
label-flag-name = اسم العلامة:
label-flag-value = القيمة:
label-condition-type = النوع:
label-condition-name = الاسم:
label-condition-value = القيمة:
label-dep-type = نوع التبعية:
label-dep-name = الاسم/الملف:
label-dep-value = القيمة/الحالة:

# Files
label-source = المصدر
label-destination = الوجهة
label-priority = الأولوية
label-file-type = النوع
label-files = الملفات
label-dependencies = التبعيات

# Settings Dialog
settings-title = الإعدادات
settings-tab-general = عام
settings-tab-recent-files = الملفات الحديثة
settings-language = اللغة:
settings-theme = السمة:
settings-font-size = حجم الخط:
settings-replace-newlines = معالجة الأسطر الجديدة في الأوصاف
settings-max-recent = الحد الأقصى للملفات الحديثة:
settings-window-width = عرض النافذة:
settings-window-height = ارتفاع النافذة:
settings-no-recent-files = لا توجد ملفات حديثة.

# Status messages for settings
status-settings-saved = تم حفظ الإعدادات بنجاح

# About Dialog
about-title = حول XIMOD Architect
about-description = أداة إنشاء مُثبِّت FOMOD متعددة المنصات لتعديلات ألعاب Bethesda.
about-license = مرخصة بموجب ترخيص MIT
about-copyright = © 2025-2026 فريق XIMOD
about-credit = نسخة Rust من الأداة الأصلية بواسطة Wenderer:

# Script Dialog
script-title = تحرير البرنامج النصي
script-info = يتم تنفيذ البرامج النصية قبل الحفظ أو بعده. يمكنك استخدام الماكروات التالية:
script-macros = الماكروات المتاحة:
macro-modname = $MODNAME$ - اسم التعديل
macro-modauthor = $MODAUTHOR$ - اسم المؤلف
macro-modversion = $MODVERSION$ - إصدار التعديل
macro-modroot = $MODROOT$ - مسار الدليل الجذري
macro-date = $DATE$ - التاريخ الحالي (YYYY-MM-DD)
macro-time = $TIME$ - الوقت الحالي (HH:MM:SS)
macro-random = $RANDOM$ - رقم عشوائي

# Plugin Dependencies
label-default-type = النوع الافتراضي:
label-pattern-type = نوع النمط:
label-pattern-operator = عامل النمط:

# Conditional Files
label-pattern = النمط

# Validation Messages
validation-no-name = اسم التعديل مطلوب
validation-no-steps = يلزم وجود خطوة واحدة على الأقل أو ملف مطلوب
validation-empty-step = الخطوة { $num } ليس لها اسم
validation-empty-group = الخطوة { $step }، المجموعة { $group } ليس لها اسم
validation-no-plugins = الخطوة { $step }، المجموعة "{ $name }" ليس لها مكونات إضافية

# File States
state-active = نشط
state-inactive = غير نشط
state-missing = مفقود

# Confirmation
confirm-title = التأكيد
confirm-delete = هل أنت متأكد من رغبتك في حذف هذا العنصر؟
confirm-discard = لديك تغييرات لم يتم حفظها. هل تريد تجاهلها والمتابعة؟
confirm-unsaved = لديك تغييرات لم يتم حفظها. هل تريد الحفظ قبل الإغلاق؟
confirm-save-issues = يحتوي المشروع على المشكلات التالية:
confirm-save-anyway = هل تريد الحفظ على أي حال؟

# Errors
error-invalid-xml = ملف XML غير صالح
error-parse-failed = فشل تحليل FOMOD
error-write-failed = فشل كتابة الملف
error-create-dir = فشل إنشاء الدليل

# Default names (generated when creating new items)
default-step-name = الخطوة { $num }
default-group-name = المجموعة { $num }
default-plugin-name = المكون الإضافي { $num }
pattern-label = النمط { $num }

# Selection prompts
msg-select-group-first = حدد مجموعة أولاً.
msg-select-plugin-edit = حدد مكونًا إضافيًا لتحريره.
label-empty = (فارغ)
image-no-image = لا توجد صورة

# File dialog filters
filter-images = الصور
filter-xml = XML

# Dependency types
dep-type-flag = علامة
dep-type-file = ملف

# Status bar
status-modified = تم التعديل

# Status messages (errors)
msg-settings-save-error = خطأ في حفظ الإعدادات
msg-script-save-error = خطأ في حفظ البرنامج النصي

# Translation editor
trans-title = محرر الترجمة
trans-source-lang = اللغة المعروضة:
trans-target-lang = اللغة المراد ترجمتها:
trans-col-key = المفتاح
trans-col-source = التسمية
trans-col-target = الترجمة
trans-saved = تم حفظ الترجمة
trans-save-error = خطأ في حفظ الترجمة

# XML editor
xml-editor-title = محرر XML
xml-editor-edit = تحرير
xml-editor-apply = تطبيق
xml-editor-revert = إلغاء
xml-editor-readonly = للقراءة فقط
xml-editor-editing = قيد التحرير — علامات التبويب الرسومية مقفلة
xml-editor-error = خطأ:
xml-editor-applied = تم تطبيق تغييرات XML
xml-editor-wellformed = XML صحيح التكوين
xml-editor-error-at = السطر { $line }، العمود { $col }: { $msg }

# Country / flag picker
settings-country-name = اسم البلد:
settings-pick-country = انقر لاختيار بلدك
flags-title = اختر بلدًا
flags-filter = تصفية:
flags-none = لم يتم العثور على أي علم

# Translation editor: country & font
trans-endonym = الاسم المحلي للبلد:
trans-font = الخط:
trans-no-font = (لا يوجد)
trans-browse = تصفح…
trans-google-fonts = خطوط Google
trans-pick-country = انقر لاختيار البلد
trans-font-outside = يجب تثبيت الخط أولاً في مجلد assets/fonts.
trans-font-dir-missing = لم يتم العثور على مجلد assets/fonts.

# Translation submission
trans-lang-endonym = الاسم المحلي للغة:
trans-author = المؤلف:
trans-submit = إرسال…
trans-submit-hint = قم بإنشاء ملف مضغوط وافتح رسالة بريد إلكتروني معدة مسبقًا
trans-data-updated = تم تحديث البيانات المرجعية (Languages.json / Countries.json)
trans-package-ready = الأرشيف جاهز:
trans-package-error = تعذر إنشاء الأرشيف:

# ISO 639-3 requirement
trans-lang-not-iso = لا يمكن الترجمة إلا للغات التي لها رمز ISO 639-3.

# FOMOD installer preview
menu-preview = معاينة المثبت…
preview-title = معاينة أداة تثبيت FOMOD
preview-refresh = تحديث
preview-assumptions = افتراضات الملفات
preview-details = التفاصيل
preview-back = رجوع
preview-next = التالي
preview-install = تثبيت
preview-close = إغلاق
preview-restart = إعادة التشغيل
preview-summary-title = الملفات التي سيتم تثبيتها
preview-empty = لن يتم تثبيت أي ملف.
preview-none-option = (لا شيء)
preview-invalid = أكمل الاختيارات المطلوبة للمتابعة.
preview-no-steps = لا توجد خطوة مرئية؛ راجع ملخص التثبيت.
preview-select-hint = حدد خيارًا لعرض وصفه.
preview-col-source = المصدر
preview-col-dest = الوجهة
preview-col-priority = الأولوية
preview-sel-exactlyone = اختر خيارًا واحدًا بالضبط.
preview-sel-atmostone = اختر خيارًا واحدًا على الأكثر.
preview-sel-any = اختر أي عدد من الخيارات.
preview-sel-all = تم تثبيت جميع الخيارات.
preview-sel-atleastone = اختر خيارًا واحدًا على الأقل.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = التحقق من صحة FOMOD
validate-report-title = التحقق من صحة FOMOD
validate-ok = لم يتم العثور على أي مشكلة. يتوافق FOMOD مع المخطط.
xml-editor-schema-ok = يتوافق مع مخطط ModConfig 5.0.
xml-editor-schema-issues = مشكلات في المخطط:
schema-line-col = السطر { $line }، العمود { $col }: { $msg }
schema-wrong-root = جذر غير متوقع "{ $found }" (المتوقع "{ $expected }").
schema-unknown = عنصر غير متوقع "{ $element }" في "{ $parent }".
schema-missing = يجب أن يحتوي "{ $parent }" على "{ $child }".
schema-needs-one = يجب أن يحتوي "{ $parent }" على "{ $child }" واحد على الأقل.
schema-too-many = لا يجوز أن يظهر "{ $child }" إلا مرة واحدة في "{ $parent }".
schema-missing-attr = السمة "{ $attr }" مطلوبة في "{ $element }".
schema-bad-enum = القيمة "{ $value }" غير صالحة لـ { $element }/@{ $attr } (المتوقع: { $allowed }).
schema-choose-one = يجب أن يحتوي "{ $parent }" على عنصر واحد فقط من: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = نقل قبل
reorder-after = نقل بعد

# Country / language database explorer (Properties)
menu-properties = الخصائص…
prop-title = قاعدة بيانات البلدان/اللغات
prop-tab-countries = البلدان
prop-tab-languages = اللغات
prop-filter = التصفية:
prop-official-langs = اللغات الرسمية
prop-spoken-langs = اللغات المستخدمة
prop-endonym = الاسم المحلي للبلد
prop-font = الخط
prop-spoken-in = تُستخدم في
prop-select-country = حدد بلدًا لعرض تفاصيله.
prop-select-lang = حدد لغة لعرض تفاصيلها.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = افتح صفحة Nexus Mods الخاصة باللعبة

# Referenced-file verification (V2)
verify-no-root = تم تخطي التحقق من الملفات: لم يتم تعيين مجلد جذر
loc-header = صورة العنوان
loc-required = الملفات المطلوبة
loc-conditional = المجموعة الشرطية { $num }
loc-plugin = الخطوة { $step }، المجموعة { $group }، الخيار "{ $plugin }"
verify-missing-file = ملف مفقود: { $path } ({ $loc })
verify-missing-folder = مجلد مفقود: { $path } ({ $loc })
verify-missing-image = صورة مفقودة: { $path } ({ $loc })
verify-absolute = مسار مطلق (غير قابل للنقل): { $path } ({ $loc })
verify-outside = المسار يخرج عن المجلد الجذر: { $path } ({ $loc })
verify-orphan = ملف يتيم (لا يشير إليه أي خيار): { $path }

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = إغلاق FOMOD
menu-close-all-fomods = إغلاق جميع ملفات FOMOD
tab-untitled = (بدون عنوان)
msg-drop-not-fomod = العنصر الذي تم إسقاطه ليس ملف FOMOD (لم يتم العثور على مجلد "fomod")
exit-title = تغييرات لم يتم حفظها
exit-unsaved = لم يتم حفظ ملف FOMOD. هل تريد حفظه؟
tab-close-hint = إغلاق ملف FOMOD هذا
