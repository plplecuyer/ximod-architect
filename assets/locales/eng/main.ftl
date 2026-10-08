# XIMOD Architect - translation metadata
# @language = eng
# @font = Noto_Sans/static/NotoSans-Regular.ttf
# @langname = English
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
menu-open = Open Folder…
menu-open-file = Open File…
menu-save = Save
menu-recent = Recent
menu-exit = Exit
menu-merge = Merge FOMOD…
menu-export = Export distribution archive…
# Menu - Options
menu-options = Options
menu-settings = Settings…
menu-pre-save-script = Pre-Save Script…
menu-post-save-script = Post-Save Script…
menu-translation = Translate the interface…
# Menu - Help
menu-help = Help
menu-check-updates = Check for updates…
menu-about = About

# Update check
update-checking = Checking for updates…
update-up-to-date = XIMOD Architect is up to date.
update-check-failed = Could not check for updates. Please try again later.
update-available-status = Version { $version } is available.
update-banner-text = XIMOD Architect { $version } is available.
update-download = Download:
update-skip = Skip this version
update-later = Later

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
label-plugin-name = Option Name:
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
btn-add-plugin = Add Option
btn-remove-plugin = Remove Option
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = Destination for the whole group
label-page-dest = Install destination (whole page)
btn-apply-group-dest = Apply to all options in this group
btn-apply-page-dest = Apply to all options on this page
group-dest-hint = Set one install destination for every file of every option in this group.
page-dest-hint = Set one install destination for every file of every option on this page (all groups).
bulk-dest-nofiles = No files to update yet — add files to the options first.
status-dest-applied = Destination applied to { $num } file(s).
preview-hidden-steps = { $num } step(s) hidden by current selections.
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
settings-check-updates = Check for updates on startup
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
about-credit = Rust port of the original tool by Wenderer:

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
label-plugin-dependencies = Option dependencies
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
validation-no-plugins = Step { $step }, group "{ $name }" has no options

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
default-plugin-name = Option { $num }
pattern-label = Pattern { $num }

# Selection prompts
msg-select-group-first = Select a group first.
msg-select-plugin-edit = Select an option to edit.
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

# Referenced-file verification (V2)
verify-no-root = File verification skipped: no root folder is set
loc-header = header image
loc-required = required files
loc-conditional = conditional set { $num }
loc-plugin = step { $step }, group { $group }, option "{ $plugin }"
verify-missing-file = Missing file: { $path } ({ $loc })
verify-missing-folder = Missing folder: { $path } ({ $loc })
verify-missing-image = Missing image: { $path } ({ $loc })
verify-absolute = Absolute path (not portable): { $path } ({ $loc })
verify-outside = Path escapes the root folder: { $path } ({ $loc })
verify-orphan = Orphan file (not referenced by any option): { $path }
conflict-certain = Destination conflict: "{ $path }" is written by { $count } options ({ $locs }) — they would overwrite each other.
conflict-potential = Possible destination conflict: "{ $path }" is targeted by { $count } references ({ $locs }) — overwrite depends on the selection/conditions.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = Close FOMOD
menu-close-all-fomods = Close all FOMODs
tab-untitled = (untitled)
msg-drop-not-fomod = Dropped item is not a FOMOD (no "fomod" folder found)
exit-title = Unsaved changes
exit-unsaved = A FOMOD has not been saved. Do you want to save it?
tab-close-hint = Close this FOMOD
menu-new-from-folder = New from folder…
menu-templates = Templates…
templates-title = Reusable templates
templates-empty = No templates saved yet. Save the selected step above to create one.
templates-insert = Insert
templates-save-step = Save selected step
templates-name-hint = Template name (optional)
msg-wizard-success = Skeleton created from folder: { $num } option(s).
msg-wizard-error = Error: { $error }
msg-template-saved = Template saved: { $name }
msg-template-inserted = Template inserted into the project.
msg-template-no-step = Select a step first to save it as a template.
msg-template-no-dir = Could not locate the templates directory.
msg-drop-assigned = Added { $added } source(s) to the option ({ $rejected } outside the root ignored).
menu-compare = Compare with…
compare-title = FOMOD comparison
compare-none = No differences.
btn-optimize-image = Optimize image
msg-image-optimized = Header image optimized.
msg-image-ok = Header image already within limits.
msg-no-header-image = No header image to optimize.
verify-image-large = Image too large ({ $width }×{ $height }): { $path }
verify-image-format = Unsupported image format (.{ $ext }): { $path }
verify-image-unreadable = Unreadable image: { $path }
menu-condition-editor = Condition editor…
condeditor-title = Condition editor
condeditor-set-by = Set by:
condeditor-used-by = Used by:
condeditor-filedeps = File dependencies
condeditor-empty = No flags or dependencies in this project.
condeditor-orphan-set = set but never used
condeditor-orphan-used = used but never set
msg-img-optimized = Image optimized.
msg-img-ok = Image already within limits.
msg-img-none = No image to optimize.
msg-crash-recovery = The previous session ended unexpectedly. A backup of your project was saved to { $path }
export-progress-title = Creating the distribution archive…
export-progress-files = { $done } / { $total } files
msg-export-cancelled = Export cancelled; the partial archive was removed.
verify-running = Checking files on disk…
verify-stale = Note: the project changed while the files were being checked; run the validation again.
prop-col-name = Name
menu-save-as = Save as…
menu-project = Project
menu-tools = Tools
menu-manual = User manual
msg-manual-missing = The user manual (PDF) was not found next to the application.
toolbar-new = New
toolbar-open = Open
toolbar-save = Save
toolbar-validate = Validate
toolbar-preview = Preview
toolbar-export = Export
dialog-choose-root = Choose the mod's root folder
exit-unsaved-docs = Unsaved: { $names }
status-summary = { $steps } steps · { $options } options
section-groups = Groups
section-options = Options
section-flags = Condition flags
section-files = Files to install
hint-group-type = How the installer lets the user pick options in this group.
hint-default-type = How the option is offered when none of its dependency patterns match: required, optional, recommended, not usable…
hint-operator = All conditions must be true (AND), or any one of them (OR).
hint-flags = Flags are named values this option sets when selected. Other steps and options can test them to show, hide or require themselves.
hint-plugin-dependencies = Patterns that change the option's type depending on flags or on files present in the game: for example “Required” when another mod is installed.
hint-files = Files and folders copied into the game's Data folder when this option is selected. Destination is relative to Data; a higher priority wins when two options write the same file.
hint-visibility = Conditions that must be met for this step to be shown at all. Leave empty to always show it.
seltype-exactly-one = Exactly one (required)
seltype-at-most-one = At most one
seltype-any = Any number
seltype-all = All (no choice)
seltype-at-least-one = At least one
plugtype-required = Required
plugtype-optional = Optional
plugtype-recommended = Recommended
plugtype-not-usable = Not usable
plugtype-could-be-usable = Could be usable
plugtype-required-hint = Always installed; the user cannot uncheck it.
plugtype-optional-hint = Offered unchecked; the user decides.
plugtype-recommended-hint = Offered pre-checked; the user may uncheck it.
plugtype-not-usable-hint = Shown greyed out and cannot be selected.
plugtype-could-be-usable-hint = Selectable, but the installer warns that it may not work.
op-and = All conditions (AND)
op-or = Any condition (OR)
theme-dark = Dark
theme-light = Light
theme-system = Follow the system
condeditor-setter-loc = Step { "{step}" } / Group { "{group}" } / «{ "{name}" }»
condeditor-pattern-of = Pattern of «{ "{name}" }» → { "{type}" }
condeditor-visibility-of = Visibility of step { "{step}" }
condeditor-cond-set = Conditional set { "{num}" }
condeditor-needs = { "{ctx}" } (needs = { "{value}" })
condeditor-file-dep = { "{ctx}" }: file «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = Translate a FOMOD…
ftr-title = Translate a FOMOD
ftr-open-folder = Open a mod folder…
ftr-from-active = From the active project
ftr-from-active-hint = Translate the FOMOD of the project that is open in the main window (it must be saved first).
ftr-no-fomod = No FOMOD loaded.
ftr-encoding = Encoding of the original files; the translated files are written with the same encoding.
ftr-source-lang = From
ftr-target-lang = to
ftr-lang-locked = (languages are fixed once a FOMOD is loaded)
ftr-translator = Translator:
ftr-save = Save translation
ftr-export = Export the translated files
ftr-export-sibling = To a fomod_<lang> folder
ftr-export-sibling-hint = Writes the translated info.xml and ModuleConfig.xml next to the original fomod folder; the original files are not touched.
ftr-export-inplace = Over the original files
ftr-export-inplace-hint = Replaces fomod/info.xml and fomod/ModuleConfig.xml after making a timestamped .bak copy of each.
ftr-force-explicit-order = Keep the original order
ftr-warn-order = Lists sorted by name (order="Ascending") would be re-sorted by the translated names in the mod manager. This forces order="Explicit" so options keep their current order.
ftr-update = Update from folder
ftr-update-hint = Re-read the FOMOD from disk and merge the translation with it: new, changed and removed strings are reported.
ftr-preview-translated = Preview translated
ftr-progress = { $done } / { $total } translated
ftr-filter-all = All
ftr-filter-untranslated = Untranslated
ftr-filter-review = To review
ftr-filter-issues = With problems
ftr-filter-locked = Locked
ftr-type-all = All fields
ftr-type-names = Names
ftr-type-descriptions = Descriptions
ftr-type-meta = Mod information
ftr-search-hint = Search source, translation or context…
ftr-next-untranslated = Next untranslated
ftr-show-whitespace = Show spaces and line breaks
ftr-discard-question = The current translation has unsaved edits. Discard them and load the other FOMOD?
ftr-discard-yes = Discard
ftr-unsaved-close = The translation has unsaved edits.
ftr-col-num = #
ftr-col-status = { "" }
ftr-col-context = Context
ftr-col-source = Source
ftr-col-target = Translation
ftr-col-issues = { "" }
ftr-empty-hint = Open a mod folder, or load the active project, to list its translatable strings.
ftr-empty-filter = No string matches the current filter.
ftr-select-row = Select a row to edit its translation.
ftr-copy-source = Copy source
ftr-clear-target = Clear
ftr-lock = Do not translate
ftr-lock-hint = Locked strings are written unchanged (author, website, proper names…).
ftr-note = Note:
ftr-status-untranslated = Untranslated
ftr-status-translated = Translated
ftr-status-auto = Pre-filled automatically — please review
ftr-status-fuzzy = The source text changed since this was translated — please review
ftr-status-obsolete = No longer present in the FOMOD
ftr-status-locked = Locked (written unchanged)
ftr-field-info-name = Mod name (info.xml)
ftr-field-module-name = Installer title (ModuleConfig.xml)
ftr-field-author = Author
ftr-field-website = Website
ftr-field-description = Mod description
ftr-field-step = Step name
ftr-field-group = Group name
ftr-field-plugin = Option name
ftr-field-plugin-desc = Option description
ftr-issue-empty = Empty translation
ftr-issue-whitespace = The translation contains only spaces
ftr-issue-edge-whitespace = Leading or trailing spaces differ from the source
ftr-issue-token = Protected tokens differ — missing: { $missing } ; extra: { $extra }
ftr-issue-newline-name = A name cannot contain a line break
ftr-issue-control = Contains characters that XML cannot store
ftr-issue-length = Unusual length compared to the source (×{ $ratio })
ftr-issue-identical = Identical to the source
ftr-issue-duplicate = Same source text translated differently in { $key }
ftr-issue-cdata = The sequence ]]> is not allowed here
ftr-load-error = Could not load the FOMOD: { $error }
ftr-extracted = { $num } translatable strings found.
ftr-sidecar-found = Existing translation loaded and merged: { $new } new, { $changed } changed, { $removed } removed.
ftr-saved = Translation saved to { $path }
ftr-save-error = Could not save the translation: { $error }
ftr-save-first = Save the project first, then translate it.
ftr-export-success = { $count } strings written to { $path }
ftr-export-error = Export failed: { $error }
ftr-export-blocked = { $num } blocking problems must be fixed before exporting.
ftr-export-stale = { $num } strings were skipped because the FOMOD changed; use “Update from folder”.
ftr-update-report = Updated: { $new } new, { $changed } changed, { $moved } moved, { $removed } removed, { $unchanged } unchanged.
menu-edit = Edit
menu-undo = Undo
menu-redo = Redo
tree-title = Project
tree-mod-info = Mod information
tree-steps = Installation steps
tree-required = Required files
tree-conditional = Conditional installs
tree-empty-steps = No step yet — click + to add one.
tree-duplicate = Duplicate
tree-delete = Delete
tree-save-template = Save as template…
tree-drop-hint = Drop here to move
cond-set-label = Conditional set { $num }
inspector-empty = Select an item in the project tree, or add a step to start.
count-options = { $num } options
count-files = { $num } files
msg-deleted-undo = Deleted. Use Undo (Ctrl+Z) to restore it.
problems-title = Problems
problems-errors = { $num } errors
problems-warnings = { $num } warnings
btn-close = Close
ftr-export-package = As a translation package (archive)
ftr-export-package-hint = Builds a .zip or .7z ready to upload: the translated info.xml and ModuleConfig.xml plus a README (patch only), or the whole mod with the translated files (full).
ftr-package-full = Full mod
ftr-package-full-hint = Include every file of the mod in the archive, not only the two translated XML files. Make sure the author allows redistribution.
ftr-package-name-template = Name:
ftr-readme-patch = This archive contains the { $langname } translation of the installer of "{ $name }" (fomod/info.xml and fomod/ModuleConfig.xml). Install it over the original mod, or let your mod manager merge it, so the translated files replace the original ones. Only the installer texts change; the mod files themselves are not included. Made with XIMOD Architect.
ftr-readme-full = This archive contains "{ $name }" with its installer translated into { $langname } (fomod/info.xml and fomod/ModuleConfig.xml). Install it like the original mod. Only the installer texts were changed. Made with XIMOD Architect.
ftr-apply-memory = Fill from memory
ftr-memory-size = Translation memory: { $num } entries for this language pair. Every saved translation is added to it.
ftr-memory-applied = { $num } strings filled from the translation memory (marked “to review”).
ftr-memory-suggestion = Memory suggests:
ftr-use-suggestion = Use
ftr-propagate = Propagate to identical
ftr-propagate-hint = Copy this translation to every other string with the same source text that is still untranslated.
ftr-propagated = { $num } identical strings filled.
ftr-csv-export = Export CSV…
ftr-csv-import = Import CSV…
ftr-csv-imported = { $num } strings updated from the CSV file.
ftr-csv-error = CSV error: { $error }
ftr-glossary = Glossary
ftr-glossary-source = Term
ftr-glossary-target = Translation
ftr-glossary-case = Case
ftr-glossary-dnt = Keep
ftr-glossary-add = Add term
ftr-issue-glossary = Glossary: “{ $term }” is not translated as expected

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = Open Archive…
filter-archive = Mod archives (zip, 7z)
msg-archive-opened = Archive opened ({ $num } files extracted): { $path }
msg-archive-reused = Archive already extracted, reusing { $path }
msg-archive-unsupported = Archive format ".{ $ext }" is not supported; extract it with 7-Zip first (only .zip and .7z can be opened).
msg-archive-error = Error opening the archive: { $error }
msg-archive-no-fomod = No "fomod" folder found in the archive ({ $path })
msg-archive-extracting = Extracting the archive…
ftr-open-archive = Open a mod archive…
ftr-package-full-partial = The mod was opened from an archive with only its fomod folder; full packages need the extracted mod.
info-module-deps = Mod requirements
info-module-deps-hint = Files or flags the whole mod requires before the installer runs (moduleDependencies). Leave empty for none.
info-header-advanced = Advanced header
info-title-position = Title position
info-title-colour = Title colour
info-title-colour-hint = Expected: six hex digits (RRGGBB)
info-image-show = Show header image
info-image-fade = Fade header image
info-image-height = Header image height
info-attr-default = (default)
file-always-install = Always
file-always-install-hint = Always install this file, even when the option is not selected (alwaysInstall).
file-install-if-usable = If usable
file-install-if-usable-hint = Install this file whenever the option is usable, even when it is not selected (installIfUsable).
msg-import-lossy = This FOMOD contains { $num } constructs XIMOD cannot edit; they will be dropped when the project is saved.
fidelity-nested-deps = Nested dependency group in { $context } (only one level is supported)
fidelity-game-dep = Game version requirement { $version } in { $context }
fidelity-fomm-dep = Mod manager version requirement { $version } in { $context }
fidelity-unknown = Element "{ $element }" in "{ $parent }" is not supported ({ $context })
loc-module = the mod requirements
loc-step = step { $step } "{ $name }"
loc-installer = the installer

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = Restore a backup…
backups-title = Restore a backup
backups-empty = This project has no backup yet. One is made each time the project is saved over a previous version.
backups-changes = { $num } change(s) from the current project
btn-compare = Compare
btn-restore = Restore
btn-delete-backups = Delete all backups
btn-delete-backups-confirm = Click again to delete every backup
msg-backup-restored = Backup of { $time } restored into the editor (not saved yet; Undo reverts it)
msg-backups-deleted = { $num } backup(s) deleted
settings-backup-count = Backups to keep:
settings-backup-count-hint = Number of previous versions of the FOMOD XML kept under fomod/backups when saving (0 = no backups).
settings-autosave-minutes = Autosave recovery copy every (minutes):
settings-autosave-minutes-hint = A recovery copy of every modified project is written in the configuration folder at this interval; it is offered on the next start only after an abnormal exit (0 = off).
settings-auto-masters = Add the masters of a plugin as conditions
settings-auto-masters-hint = When a plugin (.esp/.esm/.esl) is added to an option, the masters it requires that neither the game nor this mod provides become "Active" file conditions of the option.
msg-author-from-plugin = Author filled in from the plugin header: { $author }
msg-masters-added = { $num } master(s) of { $plugin } added as file condition(s)
issue-missing-master = { $plugin } requires { $master }, which is neither in this mod nor declared as a dependency
issue-esl-mismatch-flag = { $plugin } has the .esl extension but its light (ESL) flag is not set
issue-esl-eligible = { $plugin } could be flagged as light ({ $num } new records, limit { $limit })
issue-esl-too-big = { $plugin } is flagged as light but does not fit the light-plugin rules ({ $num } new records, limit { $limit }, or a FormID outside the allowed range)
menu-plugin-report = Plugin report…
plugins-title = Plugin report
plugins-file = File
plugins-kind = Kind
plugins-light = Light flag
plugins-masters = Masters
plugins-new-records = New records / limit
plugins-eligible = Light eligible
plugins-empty = This project installs no plugin file (.esp, .esm or .esl).
plugins-unreadable = unreadable

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = Final file tree
preview-total-size = Total install size: { $size }
preview-tree-truncated = The tree is truncated: too many files to expand (the sizes above are partial).
preview-overwritten-by = Overwritten by { $plugin }
preview-scenario = Scenario:
preview-scenario-load = Load
preview-scenario-save = Save…
preview-scenario-delete = Delete
preview-scenario-name = Scenario name
preview-scenario-saved = Scenario "{ $name }" saved under fomod/scenarios
preview-scenario-unresolved = { $num } selection(s) of the scenario match no option of this project (renamed or removed)
preview-scenario-none = (no scenario)
issue-unreachable-step = Step "{ $step }" can never be shown: its visibility conditions test a flag value that no earlier option sets
issue-unreachable-option = Option "{ $plugin }" can never be selected: its usable type patterns test a flag value that no option sets
issue-unreachable-cond = Conditional file set { $num } can never apply: its conditions test a flag value that no option sets
size-option = Install size: { $size } ({ $num } file(s))
size-missing = { $num } missing source(s)
size-unknown = Install size: — (run Validate to measure)
menu-nexus-desc = Nexus description…
nexus-title = Nexus Mods description
nexus-format = Format:
nexus-include-requirements = Requirements
nexus-include-options = Installation options
nexus-include-install = Installation
nexus-include-changelog = Changelog
nexus-previous = Previous version…
nexus-previous-none = (no previous version: no changelog)
nexus-language = Language:
nexus-language-source = (source)
nexus-sec-requirements = Requirements
nexus-sec-options = Installation options
nexus-sec-install = Installation
nexus-sec-changelog = Changelog
nexus-install-text = This mod ships a FOMOD installer: install it with a mod manager (Vortex, Mod Organizer 2) and pick your options in the installer.
nexus-requires = Requires
nexus-step = Step
nexus-added = Added
nexus-removed = Removed
nexus-changed = Changed
btn-copy = Copy
btn-save-as = Save as…
msg-copied = Copied to the clipboard
msg-saved-to = Saved to { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = Rename…
condeditor-rename-exists = A flag named "{ $name }" already exists
condeditor-renamed = Flag "{ $from }" renamed to "{ $to }" ({ $num } occurrence(s))
condeditor-delete-uses = Delete all uses
condeditor-deleted-uses = Flag "{ $name }" removed everywhere ({ $num } occurrence(s))
condeditor-values-set = Values set:
condeditor-values-tested = Values tested:
condeditor-value-never-set = { $value } — tested but never set
condeditor-value-never-tested = { $value } — set but never tested
condeditor-builder = Condition builder
condeditor-builder-none = Select a step, an option, a conditional file set or the mod information in the main window to edit its conditions here.
condeditor-builder-pattern = Pattern:
condeditor-sentence-if = IF
condeditor-sentence-and = AND
condeditor-sentence-or = OR
condeditor-sentence-flag = flag { "{name}" } = { "{value}" }
condeditor-sentence-file = file { "{name}" } is { "{value}" }
condeditor-sentence-empty = (no condition: always true)
condeditor-sentence-then-visible = THEN the step is shown
condeditor-sentence-then-type = THEN the option becomes { $type }
condeditor-sentence-then-install = THEN the files are installed
condeditor-sentence-then-module = THEN the installer can run (checked before it starts)
issue-flag-value-never-set = Flag "{ $flag }" is tested with value "{ $value }", which no option sets
issue-flag-never-used = Flag "{ $flag }" is set but never tested anywhere
menu-project-strings = Project strings…
strings-title = Project strings
strings-search = Search text, location or key…
strings-kind-all = All
strings-kind-names = Names
strings-kind-descriptions = Descriptions
strings-duplicates-only = Duplicates only
strings-replace-with = Replace with:
strings-case = Match case
strings-whole-word = Whole word
strings-replace-current = Replace
strings-replace-all = Replace all
strings-replaced = { $num } string(s) replaced
strings-dup-badge = ×{ $num }
strings-dup-hover = Same text as:
strings-count = { $num } string(s) · { $dups } duplicate group(s)
strings-col-location = Location
strings-col-field = Field
strings-col-text = Text

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = Archive contents…
filter-bethesda-archive = Bethesda archives (bsa, ba2)
archive-view-title = Archive contents
archive-view-format = Format:
archive-view-entries = { $num } entries
archive-view-size = { $size } unpacked
archive-view-search = Search a path…
archive-view-col-path = Path
archive-view-col-size = Size
archive-view-col-compressed = Compressed
archive-view-truncated = Only the first { $num } matching entries are shown — refine the search.
archive-view-error = This archive cannot be read: { $error }
archive-view-hint = View the contents of this archive
issue-conflict-archive = Same asset in several archives: "{ $path }" is packed by { $count } references ({ $locs }) — the game's archive load order decides which one is used.
issue-conflict-archive-loose = Archive vs loose file: "{ $path }" is both packed in an archive and installed loose ({ $locs }) — the loose file wins over the archived one.
preview-in-archive = (in archive)
preview-archived-size = of which { $size } packed in archives

# --- Project tree: expand / collapse menus
tree-expand = Expand
tree-collapse = Collapse
tree-expand-all = Expand All
tree-expand-selected = Expand Selected
tree-expand-from = Expand from Selected
tree-collapse-all = Collapse All
tree-collapse-selected = Collapse Selected
tree-collapse-from = Collapse from Selected
tree-expand-all-hint = Expands every heading
tree-expand-selected-hint = Expands the selected heading only
tree-expand-from-hint = Expands the selected heading and everything under it
tree-collapse-all-hint = Collapses every heading
tree-collapse-selected-hint = Collapses the selected heading only
tree-collapse-from-hint = Collapses the selected heading and everything under it

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = Add group
btn-remove-group-cond = Remove group
dep-type-game = Game version
dep-type-fomm = Mod manager version
dep-group-hint = A group of conditions combined with And / Or; groups can be nested.
condeditor-sentence-game = game version ≥ { "{value}" }
condeditor-sentence-fomm = mod manager version ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = Unique texts
ftr-uniques-hint = Show one row per distinct source text. Translating that row translates every string with the same text at once.
ftr-uniques-synced = { $num } identical strings updated.
ftr-uniques-group = { $num } strings share this text; its translation applies to all of them.
