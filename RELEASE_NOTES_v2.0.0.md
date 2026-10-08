# XIMOD Architect 2.0.0

XIMOD Architect builds FOMOD installers for Bethesda mods (Skyrim, Fallout,
Starfield, Oblivion, Morrowind) on Windows, Linux and macOS. Version 2.0.0 is a
complete overhaul of version 1: a new interface, full coverage of the FOMOD
format, and tools to check, simulate, translate and publish installers. The
user manual (English and French) is included in `Manuals/`.

## Highlights

**A new interface.** A project tree on the left and an inspector on the right
replace the stacked tabs. Undo / redo, drag-and-drop reordering, a toolbar,
plain-language labels with tooltips on every FOMOD concept, dark / light /
system themes, and keyboard navigation everywhere (↑ ↓ ← → Enter in trees,
Page Up / Page Down in tables). Japanese, Chinese, Korean, Arabic, Greek and
Cyrillic text now display correctly throughout.

**The whole FOMOD format.** Nested AND / OR condition groups, module
dependencies, `alwaysInstall` / `installIfUsable`, explicit ordering, FOMM and
game version conditions. Third-party installers open without loss — very large
ones included — and anything XIMOD cannot model is reported instead of being
dropped.

**Fewer broken installers.** A Problems panel with clickable findings;
destination-conflict detection that looks inside BSA / BA2 archives and follows
flag-gated pages; referenced-file checks; masters read from plugin headers and
turned into conditions; ESL eligibility and a plugin report; image checks. An
editable condition editor shows, for every flag, what sets it and what tests it,
renames it everywhere and builds conditions visually.

**See the installer as players will.** The preview starts on the mod
information page, walks every step (hidden pages reported) and ends with the
final file tree: sizes, overwrites by priority, archive contents, install size
per option. Scenarios can be saved and replayed from the command line.

**Translate existing FOMODs.** Open any installer (folder or `.zip` / `.7z`),
translate its strings in a table with memory, glossary, "Unique texts" mode and
CSV exchange, and export a Nexus-ready package. The original XML is patched
in place, never rewritten; an update dialog carries translations over when the
mod changes.

**Publish.** ZIP or 7z export, rotating backups with restore and diff, FOMOD
comparison, reusable templates, a "New from folder" wizard, a Nexus description
generator (BBCode / Markdown), and a command line (`validate`, `build`,
`package`, `batch`, `translate`, `simulate`, `inspect`, `archive`, `ba2`) for
automation.

## Fixes worth knowing about

Version 1 double-escaped XML attributes (`&`, `<`, `"`…) on every save and wrote
invisible characters into default names; both are fixed, and files are now
written atomically with a crash guard that keeps unsaved work. CDATA
descriptions, backslash image paths on Linux / macOS, and `Fomod` / `Info.xml`
spelled in any case are handled.

## Notes

- Projects saved by version 1 open unchanged. Files saved by 2.0.0 use the same
  layout, with the corrections above.
- Interface in 33 languages; English and French are the reference translations.
- Building from source requires Rust 1.98.1 or later.

The full list of changes is in `CHANGELOG.md`.
