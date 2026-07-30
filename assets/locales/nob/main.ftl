# XIMOD Architect - translation metadata
# @language = nob
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = norsk bokmål
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = Version { $version }

# Status messages
status-ready = Ready
msg-save-success = FOMOD saved successfully
msg-save-error = Error saving FOMOD
msg-export-success = Distribution archive created ({ $count } files): { $path }
msg-export-error = Error creating the distribution archive: { $error }
msg-load-success = FOMOD loaded successfully
msg-load-error = Error loading FOMOD
msg-merge-success = FOMOD merged successfully
msg-merge-error = Error merging FOMOD
msg-no-root-selected = Please select a root directory first
msg-no-fomod-folder = No 'fomod' folder found. Create one?
msg-file-outside-root = File is outside root directory

# Menu - File
menu-file = File
menu-new = New
menu-open = Open Folder...
menu-open-file = Open File...
menu-save = Save
menu-recent = Recent
menu-exit = Exit
menu-merge = Merge FOMOD...
menu-export = Export distribution archive...

# Menu - Options
menu-options = Options
menu-settings = Settings
menu-pre-save-script = Pre-Save Script...
menu-post-save-script = Post-Save Script...
menu-translation = Translation...

# Menu - Help
menu-help = Help
menu-about = About

# Tabs
tab-info = Mod Info
tab-steps = Install Steps
tab-required = Required Installs
tab-conditional = Conditional Installs

# Info Tab
label-workspace = Workspace
label-root-dir = Root Directory:
label-mod-name = Mod Name:
label-author = Author:
label-version = Version:
label-game-name = Game Name:
label-category = Category:
label-url = Website URL:
label-header-image = Header Image:
label-description = Description:
placeholder-select-dir = (Select a directory)
placeholder-select-game = (Select a game)

# Steps Tab
label-step-name = Step Name:
label-group-name = Group Name:
label-group-type = Group Type:
label-plugin-name = Plugin Name:
label-plugin-desc = Description:
label-plugin-type = Default Type:
label-plugin-image = Image:
label-visibility = Visibility Conditions
label-operator = Operator:

# Buttons
btn-browse = Browse...
btn-clear = Clear
btn-add = Add
btn-remove = Remove
btn-add-step = New Step
btn-delete-step = Delete Step
btn-add-group = Add Group
btn-remove-group = Remove Group
btn-add-plugin = Add Plugin
btn-remove-plugin = Remove Plugin
btn-add-file = Add File
btn-add-folder = Add Folder
btn-remove-file = Remove
btn-add-flag = Add Flag
btn-remove-flag = Remove Flag
btn-add-condition = Add Condition
btn-remove-condition = Remove Condition
btn-add-dependency = Add Dependency
btn-remove-dependency = Remove Dependency
btn-add-pattern = New Pattern
btn-remove-pattern = Delete Pattern
btn-save = Save
btn-cancel = Cancel
btn-ok = OK
btn-yes = Yes
btn-no = No

# Condition/Dependency Labels
label-flag-name = Flag Name:
label-flag-value = Value:
label-condition-type = Type:
label-condition-name = Name:
label-condition-value = Value:
label-dep-type = Dependency Type:
label-dep-name = Name/File:
label-dep-value = Value/State:

# Files
label-source = Source
label-destination = Destination
label-priority = Priority
label-file-type = Type
label-files = Files
label-dependencies = Dependencies

# Settings Dialog
settings-title = Settings
settings-tab-general = General
settings-tab-recent-files = Recent Files
settings-language = Language:
settings-theme = Theme:
settings-font-size = Font Size:
settings-replace-newlines = Process newlines in descriptions
settings-max-recent = Max Recent Files:
settings-window-width = Window Width:
settings-window-height = Window Height:
settings-no-recent-files = No recent files.

# Status messages for settings
status-settings-saved = Settings saved successfully

# About Dialog
about-title = About XIMOD Architect
about-description = A cross-platform FOMOD installer creation tool for Bethesda game mods.
about-license = Licensed under MIT License
about-copyright = © 2025-2026 XIMOD Team
about-credit = Rust-portering av det originale verktøyet av Wenderer:

# Script Dialog
script-title = Edit Script
script-info = Scripts are executed before or after saving. You can use the following macros:
script-macros = Available Macros:
macro-modname = $MODNAME$ - Mod name
macro-modauthor = $MODAUTHOR$ - Author name
macro-modversion = $MODVERSION$ - Mod version
macro-modroot = $MODROOT$ - Root directory path
macro-date = $DATE$ - Current date (YYYY-MM-DD)
macro-time = $TIME$ - Current time (HH:MM:SS)
macro-random = $RANDOM$ - Random number

# Plugin Dependencies
label-default-type = Default Type:
label-pattern-type = Pattern Type:
label-pattern-operator = Pattern Operator:

# Conditional Files
label-pattern = Pattern

# Validation Messages
validation-no-name = Mod name is required
validation-no-steps = At least one step or required file is needed
validation-empty-step = Step { $num } has no name
validation-empty-group = Step { $step }, group { $group } has no name
validation-no-plugins = Step { $step }, group "{ $name }" has no plugins

# File States
state-active = Active
state-inactive = Inactive
state-missing = Missing

# Confirmation
confirm-title = Confirmation
confirm-delete = Are you sure you want to delete this item?
confirm-discard = You have unsaved changes. Discard them and continue?
confirm-unsaved = You have unsaved changes. Do you want to save before closing?
confirm-save-issues = The project has the following issues:
confirm-save-anyway = Save anyway?

# Errors
error-invalid-xml = Invalid XML file
error-parse-failed = Failed to parse FOMOD
error-write-failed = Failed to write file
error-create-dir = Failed to create directory

# Default names (generated when creating new items)
default-step-name = Step { $num }
default-group-name = Group { $num }
default-plugin-name = Plugin { $num }
pattern-label = Pattern { $num }

# Selection prompts
msg-select-group-first = Select a group first.
msg-select-plugin-edit = Select a plugin to edit.
label-empty = (empty)
image-no-image = No image

# File dialog filters
filter-images = Images
filter-xml = XML

# Dependency types
dep-type-flag = Flag
dep-type-file = File

# Status bar
status-modified = Modified

# Status messages (errors)
msg-settings-save-error = Error saving settings
msg-script-save-error = Error saving script

# Translation editor
trans-title = Translation Editor
trans-source-lang = Displayed language:
trans-target-lang = Language to translate:
trans-col-key = Key
trans-col-source = Label
trans-col-target = Translation
trans-saved = Translation saved
trans-save-error = Error saving translation

# XML editor
xml-editor-title = XML Editor
xml-editor-edit = Edit
xml-editor-apply = Apply
xml-editor-revert = Cancel
xml-editor-readonly = Read-only
xml-editor-editing = Editing — graphical tabs are locked
xml-editor-error = Error:
xml-editor-applied = XML changes applied
xml-editor-wellformed = Well-formed XML
xml-editor-error-at = Line { $line }, column { $col }: { $msg }

# Country / flag picker
settings-country-name = Country name:
settings-pick-country = Click to choose your country
flags-title = Choose a country
flags-filter = Filter:
flags-none = No flag found

# Translation editor: country & font
trans-endonym = Country endonym:
trans-font = Font:
trans-no-font = (none)
trans-browse = Browse…
trans-google-fonts = Google Fonts
trans-pick-country = Click to choose the country
trans-font-outside = The font must be installed in assets/fonts first.
trans-font-dir-missing = The assets/fonts folder was not found.

# Translation submission
trans-lang-endonym = Language endonym:
trans-author = Author:
trans-submit = Send…
trans-submit-hint = Build a zip and open a pre-filled e-mail
trans-data-updated = Reference data updated (Languages.json / Countries.json)
trans-package-ready = Archive ready:
trans-package-error = Could not build the archive:

# ISO 639-3 requirement
trans-lang-not-iso = Translation is only possible for a language with an ISO 639-3 code.

# FOMOD installer preview
menu-preview = Preview installer…
preview-title = FOMOD installer preview
preview-refresh = Refresh
preview-assumptions = File assumptions
preview-details = Details
preview-back = Back
preview-next = Next
preview-install = Install
preview-close = Close
preview-restart = Restart
preview-summary-title = Files that will be installed
preview-empty = No file would be installed.
preview-none-option = (none)
preview-invalid = Complete the required choices to continue.
preview-no-steps = No step is visible; see the install summary.
preview-select-hint = Select an option to see its description.
preview-col-source = Source
preview-col-dest = Destination
preview-col-priority = Priority
preview-sel-exactlyone = Choose exactly one option.
preview-sel-atmostone = Choose at most one option.
preview-sel-any = Choose any number of options.
preview-sel-all = All options are installed.
preview-sel-atleastone = Choose at least one option.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = Validate FOMOD
validate-report-title = FOMOD validation
validate-ok = No problem found. The FOMOD conforms to the schema.
xml-editor-schema-ok = Conforms to the ModConfig 5.0 schema.
xml-editor-schema-issues = Schema issues:
schema-line-col = Line { $line }, col. { $col }: { $msg }
schema-wrong-root = Unexpected root "{ $found }" (expected "{ $expected }").
schema-unknown = Unexpected element "{ $element }" in "{ $parent }".
schema-missing = "{ $parent }" must contain "{ $child }".
schema-needs-one = "{ $parent }" must contain at least one "{ $child }".
schema-too-many = "{ $child }" may appear only once in "{ $parent }".
schema-missing-attr = Attribute "{ $attr }" is required on "{ $element }".
schema-bad-enum = Invalid value "{ $value }" for { $element }/@{ $attr } (expected: { $allowed }).
schema-choose-one = "{ $parent }" must contain exactly one of: { $options }.

# Reordering (steps / groups / plugins)
reorder-before = Move before
reorder-after = Move after

# Country / language database explorer (Properties)
menu-properties = Properties…
prop-title = Country / language database
prop-tab-countries = Countries
prop-tab-languages = Languages
prop-filter = Filter:
prop-official-langs = Official languages
prop-spoken-langs = Languages spoken
prop-endonym = Country endonym
prop-font = Font
prop-spoken-in = Spoken in
prop-select-country = Select a country to see its details.
prop-select-lang = Select a language to see its details.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = Open the game's Nexus Mods page
