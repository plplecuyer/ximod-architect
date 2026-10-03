# XIMOD Architect v1.0.4

XIMOD Architect is a cross-platform tool for creating FOMOD installers for
Bethesda games (Skyrim, Fallout, Starfield, Oblivion, Morrowind…).

This release adds a bulk "same destination" action for whole groups and pages,
refreshes the application's identity as **XIMOD Architect — XML Builder** with a
new splash screen, and polishes the interactive preview.

## New features

- **Same destination for a whole group / page.** You can now assign one install
  destination to every plugin in a group — or across an entire step/page — in a
  single action, instead of editing each option's file list by hand. In the
  *Install Steps* tab, the group editor gains a **Destination for the whole group**
  field with an **Apply to all plugins in this group** button; at the step level,
  a collapsible **Install destination (whole page)** panel offers the same with an
  **Apply to all plugins on this page** button. Each action writes the destination
  onto every file of every plugin concerned and reports how many files were
  updated. The button stays disabled until the plugins actually contain files, and
  you can still fine-tune any individual destination afterwards in the plugin's
  *Source* table.
- **New "XML Builder" identity.** Refreshed branding and a brand-new splash
  screen. The application is now subtitled *XML Builder* — *The XML installer,
  designed for your Mods.*

## Fixes and polish

- **Preview — description formatting.** The interactive preview now honours the
  *"Process newlines in descriptions"* setting: literal `\n` / `\r\n` sequences are
  rendered as real line breaks, so a description looks the way a mod manager will
  display it.
- **Preview — hidden steps are now explained.** When later steps are hidden by
  unmet visibility conditions, the preview shows a short hint
  (*"N step(s) hidden by current selections"*), making clear that extra pages
  appear once the matching options are chosen — instead of looking as though the
  preview stops at the first screen.

## Documentation

- The user manual has been updated in **all 32 languages** with a new subsection
  (§4.2) documenting the group / page destination controls, and now carries the
  new cover art.
- The project **wiki**, **specifications** and **requirements** have been updated
  for 1.0.4.

## Other changes

- Version metadata bumped to 1.0.4; generated `info.xml` / `ModuleConfig.xml` now
  carry the comment `<!-- Created with XIMOD Architect 1.0.4 -->`.
- Still built with **Rust 1.98.1** (2024 edition); no toolchain change since
  1.0.3.

## Downloads

Pick the package for your system:

- **Windows** — installer `XIMOD_Architect_1.0.4_Setup.exe`, or the portable
  archive `ximod-architect-1.0.4-windows-x86_64.zip` (extract and keep the
  executable next to its `assets` folder).
- **Linux** — `ximod-architect-1.0.4-linux-x86_64.tar.gz` (extract and keep the
  executable next to its `assets` folder).
- **macOS** — `ximod-architect-1.0.4-macos-universal.dmg`, or
  `ximod-architect-1.0.4-macos-universal.app.zip` (Apple Silicon + Intel,
  universal binary).

## Note on macOS / Windows signing

The macOS build is signed **ad-hoc** (not notarised): on first launch, right-click
the app and choose **Open**, then confirm, to bypass Gatekeeper. On Windows,
SmartScreen may warn about an unrecognised publisher — choose **More info →
Run anyway**. These warnings are expected for an unsigned open-source build.

## Thanks

Thanks to everyone who reported issues and suggested improvements — in
particular **TheRealDoctorNormal**, whose request to assign a single destination
to a whole group or page is exactly what this release's headline feature adds.
