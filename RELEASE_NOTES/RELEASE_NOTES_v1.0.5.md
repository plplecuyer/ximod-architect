# XIMOD Architect v1.0.5

XIMOD Architect is a cross-platform tool for creating FOMOD installers for
Bethesda games (Skyrim, Fallout, Starfield, Oblivion, Morrowind…).

This release gives XIMOD its new **XML Builder** identity and focuses on the
interactive preview and on handling very large FOMODs: the preview now behaves
like a real mod manager across multiple pages, and the editor stays usable with
dozens of steps or conditional installs. (It also folds in the changes that were
prepared as 1.0.4.)

## New features

- **New "XML Builder" identity.** Refreshed branding and a brand-new splash
  screen. The application is now subtitled *XML Builder* — *The XML installer,
  designed for your Mods.*
- **Same destination for a whole group / page.** Assign one install destination
  to every plugin in a group — or across an entire step/page — in a single
  action, instead of editing each option's file list by hand. The group editor
  gains a **Destination for the whole group** field with an **Apply to all
  plugins in this group** button; at step level, a collapsible **Install
  destination (whole page)** panel does the same for the whole page. A status
  message reports how many files were updated.

## Fixes and polish

- **Preview — multi-page installers now work fully.** The Back / Next / Install
  bar is pinned to the bottom of the preview window, so it can no longer be
  pushed off-screen by a long list of options. Installers with several steps
  previously looked stuck on the first page; you can now step through every page.
- **Preview — closer to a real mod manager.** Option descriptions are
  left-aligned and wrap naturally (no more stretched, justified text); the
  option image is shown as a prominent banner; and the bottom bar now labels the
  Back / Next buttons with the step they lead to and shows a progress bar, the
  way Vortex / Mod Organizer 2 / NMM do.
- **Preview — description formatting.** The preview honours the *"Process
  newlines in descriptions"* setting (literal `\n` rendered as line breaks), and
  shows a short hint when later steps are hidden by the current selections.
- **Editor — large FOMODs.** The step tabs and the conditional-install tabs are
  now paginated (8 per page, with « / » paging buttons) so the +, ◀ and ▶
  controls never slide off-screen. The groups / plugin-details split is
  responsive so the details pane never collapses, and the plugin description
  field wraps correctly.

## Documentation

- The user manual has been updated in **all 32 languages** (cover art + the new
  "group / page destination" section), along with the project **wiki**,
  **specifications** and **requirements**.

## Other changes

- Version metadata bumped to 1.0.5; generated `info.xml` / `ModuleConfig.xml` now
  carry the comment `<!-- Created with XIMOD Architect 1.0.5 -->`.
- Still built with **Rust 1.98.1** (2024 edition); no toolchain change since
  1.0.3.

## Downloads

Pick the package for your system:

- **Windows** — installer `XIMOD_Architect_1.0.5_Setup.exe`, or the portable
  archive `ximod-architect-1.0.5-windows-x86_64.zip` (extract and keep the
  executable next to its `assets` folder).
- **Linux** — `ximod-architect-1.0.5-linux-x86_64.tar.gz` (extract and keep the
  executable next to its `assets` folder).
- **macOS** — `ximod-architect-1.0.5-macos-universal.dmg`, or
  `ximod-architect-1.0.5-macos-universal.app.zip` (Apple Silicon + Intel,
  universal binary).

## Note on macOS / Windows signing

The macOS build is signed **ad-hoc** (not notarised): on first launch, right-click
the app and choose **Open**, then confirm, to bypass Gatekeeper. On Windows,
SmartScreen may warn about an unrecognised publisher — choose **More info →
Run anyway**. These warnings are expected for an unsigned open-source build.

## Thanks

Thanks to everyone who reported issues and suggested improvements — in
particular **TheRealDoctorNormal**, whose feedback drove the interactive-preview
overhaul (multi-page navigation and the more manager-like layout) and the
"same destination for a whole group / page" feature.
