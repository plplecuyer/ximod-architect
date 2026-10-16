# Changelog

All notable changes to XIMOD Architect are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow
[Semantic Versioning](https://semver.org/).

## [2.2.1] — 2026-10-16

Bug fixes and three additions for the preset wizard and for installers built
with it, after testing by TheRealDoctorNormal on a real installer of more
than 40 pages.

### Added

- Preset wizard, run again on a project it already transformed: the window
  opens with the existing pages, names and presets loaded, and Apply rebuilds
  the structure (one undoable edit) instead of stacking a second one on top of
  the first; the flag names stay the same, so a mod update keeps working.
- Conditional Installs tab: one destination for every file of every
  conditional set at once. The group and page destination controls now also
  reach the files the preset wizard moved to conditional installs (the sets
  fed by the options of the group or page).
- Preset wizard: the Preset group is a single choice ending with a "No preset
  (build my own)" option, for users who want to pick everything from the
  Customise pages; its name is set in the wizard.

### Fixed

- Exported ZIP archives (and translation packages) dated every file
  1980-01-01: each entry now keeps its file's modification date, so the files
  a mod manager deploys show their real date. 7z archives already did.
- Preset wizard — installers it generated could install the files a preset
  chose on a feature page the user had opened and then hidden again by going
  back to the first page (the mod manager keeps that page's flags). Every
  Customise flag is now set explicitly on the first page — off by the chosen
  preset or by the new "No preset (build my own)" option, on by the Customise
  option — and each file is installed only when its page is customised and
  the option is ticked there, or when the page is not customised and the
  chosen preset picked it. The Preset group becomes a single choice.
- Preset wizard — exclusive groups of type SelectAtMostOne showed two "None"
  options in Vortex (the wizard's and the manager's own): a group that
  receives the wizard's None becomes SelectExactlyOne.

### Changed

- 33 interface languages: the 5 new strings are translated in every locale
  (868 messages per locale). Test suite at 344 tests.
- Dependencies refreshed: zip 9.0.4 (every 2.x release of the crate has
  been withdrawn from crates.io, which made `cargo update` fail), arboard
  3.7.0, combine 4.6.9, equivalent 1.0.3, rustls 0.23.46, tokio 1.53.3,
  zerocopy 0.8.63; windows-sys 0.60 and its target crates are no longer
  pulled in.
- XIMOD is described as a tool for the mods of any game that Vortex or Mod
  Organizer 2 can manage, no longer for Bethesda games only: About window (33
  languages), package description, Linux desktop entry, README, manual cover
  and § 1.1 (32 languages), release notes.
- User manual (32 languages): § 4.2, § 4.4 and § 8.29 document the new
  destination controls and running the wizard again; § 7.5 explains what
  happens when the user goes back in the installer, and § 8.29 describes
  the No preset option and how the files follow the first page.

## [2.2.0] — 2026-10-14

Three additions suggested by Wenderer, author of the original FOMOD Creation
Tool, and by one of its users: moving installer pieces between FOMODs, and
keeping file references in step with the disk. The release also answers
feedback from Nexus users (TheRealDoctorNormal, VikMorroHun, rbtRvlt,
Krou705).

### Added

- Cut / copy / paste of steps, groups and options (Edit menu, tree context
  menus, Ctrl+X / Ctrl+C / Ctrl+V while the tree has the focus). The clipboard
  is shared by every open FOMOD tab, and a copy also reaches the system
  clipboard, so nodes travel between two XIMOD windows. Pasting follows the
  selection: a step goes after the selected step, a group into the selected
  step, an option into the selected group; one undoable edit. After a paste,
  XIMOD warns about files the pasted node uses that are not in this mod's
  folder and about flags it tests that nothing in the project sets.
- Rename on disk… (files tables of options, required files and conditional
  sets): renames the selected file or folder in the mod folder and updates
  every reference to it — for a folder, every reference inside it as well —
  keeping each reference's separator style. Case-only renames work on every
  file system.
- Project → Find missing files…: lists the files, folders and images the
  project uses that no longer exist, with candidates found in the mod folder
  (same name elsewhere → moved; similar name where it was → renamed). A single
  "moved" match is pre-selected; Browse… picks any other location; Apply
  updates every reference at once (one undo step) and rescans.
- The Destination cell of the files tables shows "(root of Data)" in grey
  while it is empty, and its tooltip explains the path and that the cell is
  edited by clicking in it — some users did not realise it was editable.
- The "Created with…" comment at the top of info.xml and ModuleConfig.xml now
  carries the application's Nexus page, as the original FOMOD Creation Tool
  did.

### Changed

- 33 interface languages: the 45 new strings are translated in every locale
  (863 messages per locale); Chinese "Duplicate" becomes 创建副本 so that it
  differs from the new "Copy" (复制).
- Interface labels reviewed in every language: 128 labels corrected (typos,
  incomplete phrases, a verb form out of line with the rest of its menu, one
  concept translated two ways, a space before "…", "..." instead of "…"),
  for example Greek "About", Croatian "Merge FOMOD…", Maltese "Move before /
  after", Norwegian ellipses, Japanese spacing.
- One term per concept, in the interface and the manual: "flag" is drapeau
  (French), bandera (Spanish), bandeira (Portuguese) and pavilion (Romanian);
  an installation step is tappa (Italian); an option is opció (Hungarian).
- User manual (32 languages): § 1.1 now says that FOMOD is a mod-manager
  format — installers work with every Bethesda game and most other games that
  Vortex or Mod Organizer 2 can manage; the built-in game list only fills in
  info.xml. F1 is documented as opening the manual (it was given as opening
  the About window); the tree's Move before / Move after commands are named
  correctly; menu, tab and button names now match the interface of each
  language, including the Bulgarian, Korean and Maltese editions, which
  still used English names; the original tool is called by its real name,
  FOMOD Creation Tool.

## [2.1.0] — 2026-10-12

Three additions driven by user feedback on the 2.0 interface, plus the games
editor that followed the 2.0.0 release.

### Added

- Preset wizard (Project → Preset wizard…): generates the "preset + customise"
  pattern of the manual (§ 7.5) on the current project. The first tab works
  from existing feature pages (tick the pages, name the generated elements,
  define any number of presets in a choice matrix); the second creates feature
  pages from a short description. The wizard inserts the installation-mode step
  (preset group + customise group), makes the pages conditional, flags their
  options with dynamic *Recommended* types, adds "None" options to exclusive
  groups and moves the options' files to conditional installs — one undoable
  edit, nothing installed twice.
- Tree colours (Tools → Tree colours…): one colour per kind of row of the
  project tree (mod information, steps, groups, options, required files,
  conditional installs), one set per theme, stored in `Config.ini`. Colours
  are chosen in the "Select a colour" window ported from XIMOD Translator
  (AutoCAD colour index and true colours).
- The project-relative file dialogs (Add file / Add folder / image browse)
  reopen in the parent of the last chosen file or folder, so sibling folders
  are one click away; back to the project root when the folder is gone.
- Bulk destination on the Required Installs and Conditional Installs tabs:
  one field and one button set the install destination of every required
  file, or of every file of the selected conditional set, at once — the
  counterpart of the group / page controls of the Install Steps tab.
- Games / categories editor (Tools → Games / categories…): add, correct or
  remove a game and its Nexus categories without editing `Categories.json` by
  hand; saved to the shipped file or to a per-user copy on read-only installs.
- User manual (English and French): § 7.5 "Preset with customisable defaults",
  § 7.6 "Introduction and closing pages", design rule in § 6.6, drag-and-drop
  cross-reference in § 4.2, § 8.26 to § 8.29 for the features above.

### Changed

- Minimum supported Rust version raised to 1.99.0 (`rust-version` in
  `Cargo.toml`), the current stable toolchain.
- `Cargo.lock`: toml_datetime 1.1.2, toml_edit 0.25.17, toml_parser 1.1.5,
  zerocopy / zerocopy-derive 0.8.62, cc 1.8.0, litrs 1.0.1, serde_json 1.0.154,
  smallvec 1.16.3, syn 3.0.9, unicode-ident 1.0.27, uuid 1.28.0, zopfli 0.8.4,
  deranged 0.5.9.
- 33 interface languages: every new key translated in every locale (818
  messages per locale); test suite at 323 tests.
- Release workflow: GitHub Actions moved to the Node 24 runtime
  (`upload-artifact@v7`, `download-artifact@v7`, `action-gh-release@v3`);
  release job pinned to `ubuntu-24.04` (ubuntu-latest moves to Ubuntu 26).
- UTF-16 decoding (`xml/mod.rs`, `xml/patch.rs`): `chunks_exact(2)` replaced
  by `as_chunks::<2>()` (new Clippy 1.99 lint `chunks_exact_to_as_chunks`).
- Windows splash screen (`splash/mod.rs`): `update_layered_window` takes the
  window position and size as `&POINT` / `&SIZE` (8 → 6 arguments) and passes
  shared references to `UpdateLayeredWindow`, which takes `*const` pointers
  (Clippy `too_many_arguments`, `unnecessary_mut_passed` on Windows).

### Fixed

- Translating an existing FOMOD on Windows or macOS: when the folder or files
  are spelled differently from `fomod/info.xml` / `ModuleConfig.xml` (for
  example `Fomod/Info.xml`, `moduleconfig.xml`), the case-insensitive lookup
  now returns the name actually on disk. A full translation package no longer
  contains the two XML files twice under two spellings.

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
