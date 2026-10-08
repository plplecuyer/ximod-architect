# Changelog

All notable changes to XIMOD Architect are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow
[Semantic Versioning](https://semver.org/).

## [2.0.0] — 2026-10-07

Version 2 is a complete overhaul: the interface was rebuilt around a project tree
and an inspector, the installer model now covers the whole FOMOD schema, and a
set of tools was added for checking, simulating, translating and publishing
installers. The user manual (English and French, `Manuals/`) describes every
feature; chapter 8 lists the additions of this version in detail.

### Fixed (data integrity)

- XML attributes were re-read without unescaping, so `&`, `<`, `>`, `"` and `'`
  were escaped twice on every open/save cycle.
- Default names ("Step 1", "Group 1", "Plugin 1") contained invisible Unicode
  isolation marks that ended up in `ModuleConfig.xml`.
- `<![CDATA[…]]>` descriptions were lost on load.
- Files were written non-atomically and write errors were swallowed; saves now
  go through a temporary file and report failures.
- Stale selection indexes could crash the application (release builds aborted
  silently); a crash guard now keeps an emergency copy of unsaved work.
- Dropping a project folder, Shift+Tab in Settings, image paths with
  backslashes on Linux/macOS, `fomod`/`info.xml` folder and file names in any
  letter case.

### Added — installer model

- Nested condition groups (`<dependencies>` inside `<dependencies>`) with AND/OR
  at every level, `moduleDependencies`, `alwaysInstall`, `installIfUsable`,
  explicit ordering, FOMM and game version conditions. Very large installers
  (hundreds of conditional sets) load and round-trip byte for byte.
- Import-fidelity check: elements XIMOD cannot model are reported on load
  instead of being dropped silently.

### Added — interface

- Project tree on the left (mod information, steps, groups, options, required
  files, conditional sets) with an inspector on the right; multi-document tabs.
- Undo / redo, toolbar, reorganised menus, human-readable labels and tooltips
  for every FOMOD concept, dark / light / system theme, interface font size.
- Drag-and-drop: reorder steps, groups and options; drop files from the system
  onto an option; drop a folder or an archive to open it.
- Problems panel per document, with clickable findings that select the faulty
  node; validation and export run in the background with progress.
- Expand / Collapse context menus and keyboard navigation (↑ ↓ ← → Enter,
  Home / End) in every tree: project tree, condition editor, preview file tree;
  ↑ ↓ Page Up / Page Down in every table.
- Toast notifications; safe save flow (Save asks for a root when there is none,
  Save as…, window title with the modified marker). The status bar keeps the
  last message and the step / option counts; the root folder is shown in the
  Workspace section only.
- Script detection: Japanese, Chinese, Korean, Arabic, Greek, Cyrillic… are
  rendered with the bundled Noto fonts wherever they appear.
- Free, independent windows for the installer preview and the condition
  editor; Settings and country picker stay on top at first launch.
- Startup: data files are preloaded on a worker thread behind the splash
  screen.

### Added — checks and simulation

- Destination-conflict detection, including assets packed in BSA / BA2 archives,
  with certainty levels that follow flag-gated pages.
- Referenced-file verification (missing, absolute or escaping paths, orphans).
- Plugin headers (`.esp` / `.esm` / `.esl`): masters become file conditions
  automatically; ESL eligibility check and a plugin report with a masters
  column.
- Installer preview rebuilt: mod information page (header image, title,
  author, version, description as a manager shows it), every step with hidden
  pages reported, summary with the final file tree (sizes, overwrites by
  priority, archive contents), install size per option, saved scenarios
  replayable from the CLI (`simulate`).
- Editable condition editor: "set by" / "used by" links, flag renaming
  everywhere, orphan and never-set values, visual condition builder.
- Image checks and optimisation (format, dimensions, resizing).

### Added — translating existing FOMODs

- Lossless translation of third-party installers: an XML patcher replaces only
  the targeted strings, so the original file structure is preserved.
- Translation window with source / target columns, status, filters, search,
  whitespace display, "Unique texts" mode (identical strings translated once),
  translation memory and glossary, CSV import / export, translated preview.
- Update dialog when the original mod changes (fuzzy matching of moved or
  edited strings); Nexus-ready package with a name template and README.
- Command line: `translate` for extraction, application and updates.

### Added — publishing and tools

- ZIP or 7z export, rotating backups with "Restore a version…" and a
  structural diff, FOMOD comparison, reusable templates (step / group /
  option), "New from folder" wizard.
- Nexus description generator (BBCode / Markdown) from the project.
- Project strings table (every name and description, search / replace).
- Archive content viewer for BSA / BA2 and mod archives; opening a FOMOD
  directly from a `.zip` / `.7z`.
- Experimental BA2 writer (GNRL v1, uncompressed), validated by reading its
  output back.
- Country / language window: country names shown in the language chosen in
  Settings.
- Command line: `validate`, `build`, `package`, `batch`, `inspect`, `archive`,
  `translate`, `simulate`, `ba2`.

### Changed

- Minimum supported Rust version: 1.98.1 (edition 2024).
- 33 interface languages; every key is translated in every locale, enforced
  by a test.
- `main_window.rs` split into focused modules; CI runs rustfmt, clippy
  (`-D warnings`) and the test suite (299 tests).
- Dependencies refreshed (zerocopy, zeroize, arbitrary…); unused crates removed.

## [1.0.5] and earlier

Version 1 releases (1.0.0 – 1.0.5) predate this changelog; see the GitHub
release pages for their notes.
