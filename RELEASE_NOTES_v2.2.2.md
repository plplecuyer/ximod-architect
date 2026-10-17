# XIMOD Architect 2.2.2

XIMOD Architect builds FOMOD installers — the format read by Vortex and Mod
Organizer 2, for Skyrim, Fallout, Starfield and many other games — on Windows,
Linux and macOS. Version 2.2.2 fixes, for good, the installers built by the
preset wizard, after TheRealDoctorNormal tested 2.2.1 on a real mod.

## Fixed

**Preset wizard: what is shown is what gets installed.** Clicking through the
pages, going back and choosing again could still install files that were not
shown as selected, or leave out files that were. The cause was in the
generated installer: several options that can be selected together wrote the
same flag, and mod managers keep the flags in a last-click-wins table. The
wizard now follows one rule — one flag, one writer. The presets write only the
preset flag; each feature page gets a two-option group on the first page,
"Use the preset's choices" / "Customise", instead of a checkbox; the page
options alone write the feature flags. The flags then always match the
screen, whatever the order of the clicks.

**Run the wizard again to update.** Structures made by 2.1.0, 2.2.1 and
2.2.2 are recognised: Project → Preset wizard… then Apply rebuilds yours the
new way, keeps the flag names and leaves the installation-mode page where you
had moved it.

## Notes

- Projects and FOMODs saved by 2.2.1 open unchanged.
- The user manual (32 languages) explains the rule in § 7.5, with a new XML example, and the wizard in § 8.29.
- Interface in 33 languages; every new string is translated in every locale.
- Building from source requires Rust 1.99.0 or later.
