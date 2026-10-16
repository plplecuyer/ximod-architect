# XIMOD Architect 2.2.1

XIMOD Architect builds FOMOD installers — the format read by Vortex and Mod
Organizer 2 — on Windows, Linux and macOS. Version 2.2.1 fixes the preset
wizard and the installers it builds, and adds what was missing to maintain them,
after testing by TheRealDoctorNormal on a real installer of more than 40 pages.

## Fixed

**Preset wizard: files from a hidden page.** In an installer built by the
wizard, a user who opened a feature page and then went back to the first page
and hid it could still get the files the preset chose on that page — mod
managers keep the flags of a page already seen. The first page now sets every
Customise flag explicitly, and each file depends on whether its page is
customised: what is ticked on the customised pages is what gets installed.
To update an existing installer, run Project → Preset wizard… again and Apply.

**Preset wizard: two "None" options in Vortex.** Vortex adds its own None to
"select at most one" groups; a group that gets the wizard's None now becomes
"select exactly one", so only one None is shown.

**ZIP export dates.** Exported ZIP archives (and translation packages) dated
every file 1980-01-01; each file now keeps its modification date.

## Added

**Run the preset wizard again.** On a project it already transformed, the
wizard opens with the existing pages, names and presets loaded; add or change
presets and Apply rebuilds the structure in one undoable step, keeping the
flag names so that a mod update stays compatible with the previous release.

**"No preset (build my own)".** The Preset group is now a single choice that
ends with this option, for users who want to pick everything themselves.

**One destination for all conditional sets.** The Conditional Installs tab
sets one destination on every file of every conditional set at once, and the
group and page destinations now also reach the files the wizard moved to
conditional installs.

## Notes

- Projects and FOMODs saved by 2.2.0 open unchanged.
- The user manual (32 languages) documents these changes in § 4.2, § 4.4, § 7.5 and § 8.29.
- Interface in 33 languages; every new string is translated in every locale.
- Dependencies refreshed. Building from source requires Rust 1.99.0 or later.
