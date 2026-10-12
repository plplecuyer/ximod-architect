# XIMOD Architect 2.1.0

XIMOD Architect builds FOMOD installers for Bethesda mods (Skyrim, Fallout,
Starfield, Oblivion, Morrowind) on Windows, Linux and macOS. Version 2.1.0 adds
four features suggested by mod authors using version 2, plus a games /
categories editor. The user manual (English and French) is included in
`Manuals/`; chapter 7 explains the techniques and § 8.26 to § 8.29 the new
tools.

## Highlights

**Preset wizard.** Project → Preset wizard… builds a "preset + customise"
installer in one step: pick a preset and be done, pick a preset and still adjust
a few features, or build everything by hand — without any file ever being
installed twice. Work from existing feature pages (tick them, define any number
of presets in a choice matrix) or describe the features and let the wizard
create the pages. It inserts an installation-mode page, shows each feature page
only on demand with the preset's choices pre-ticked (dynamic *Recommended*
types), adds *None* options and moves the files to conditional installs — one
undoable edit.

**Tree colours.** Tools → Tree colours… gives each kind of row of the project
tree its own colour (mod information, steps, groups, options, required files,
conditional installs), one set per theme, chosen in the "Select a colour"
window shared with XIMOD Translator (AutoCAD colour index or true colours).

**File dialogs remember where you were.** Add file / Add folder / image Browse
reopen in the parent of the last file or folder you picked, so sibling folders
are one click away.

**Destination for a whole set.** The Required Installs and Conditional Installs
tabs get the one-click destination that steps and groups already had.

**Games / categories editor.** Tools → Games / categories… adds, corrects or
removes a game and its Nexus categories without editing `Categories.json`;
edits are saved to a per-user copy when the install folder is read-only.

## Manual

New sections in English and French: § 7.5 "Preset with customisable defaults",
§ 7.6 "Introduction and closing pages", a design rule in § 6.6, and § 8.26 to
§ 8.29. The 30 other translated editions are updated as well.

## Notes

- Projects and FOMODs saved by 2.0.0 open unchanged.
- Interface in 33 languages; every new string is translated in every locale.
- Building from source requires Rust 1.98.1 or later.
