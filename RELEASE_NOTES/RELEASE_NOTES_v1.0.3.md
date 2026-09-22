# XIMOD Architect v1.0.3

XIMOD Architect is a cross-platform tool for creating FOMOD installers for
Bethesda games (Skyrim, Fallout, Starfield, Oblivion, Morrowind…).

This release adds an optional update check, makes the tool windows independent
and freely movable, and moves the build to the latest stable Rust toolchain.

## New features

- **Update notifications.** XIMOD can now tell you when a newer release is
  available. On startup — at most once per day — it checks the GitHub Releases
  page and, if a newer version exists, shows a discreet banner below the menu bar
  with download links (**Nexus** and **GitHub**), a **Skip this version** action
  and a **Later** action. You can also check on demand with
  **Help → Check for updates**, and turn the whole thing off in
  **Settings → Check for updates on startup**.
  - *Privacy:* this is the only network request the application makes. It sends
    no personal data — just a single secure (HTTPS) request to the public GitHub
    Releases API to read the latest version number. XIMOD never downloads or
    installs anything by itself; you always choose whether to follow the download
    link.
- **Independent, freely movable windows.** The Settings, About, Pre-save script,
  Post-save script and FOMOD validation windows are now independent OS-level
  windows instead of modal panels. You can move them anywhere (including onto a
  second screen), the main window stays usable while they are open, and their
  position (and size, where applicable) is remembered in `Config.ini`. The
  Settings and About windows keep a fixed size; the script and validation windows
  are resizable.

## Under the hood

- **Now built with Rust 1.98.1** (2024 edition). Thanks to Rust's
  within-edition backward compatibility, moving the compiler forward required no
  code changes. The declared minimum supported Rust version stays
  `rust-version = "1.97.1"`; version 1.0.2 was built with 1.97.1, and 1.0.3 and
  later are built with 1.98.1.

## Fixes and polish

- **Flag picker:** several country captions in non-Latin scripts (Georgian,
  Japanese, Khmer, Armenian, Korean, Burmese, Lao, Chinese, Bengali, Hebrew…)
  no longer show as empty boxes — all writing-system fonts are now loaded while
  the picker is open.
- **French wording:** corrected capitalisation in the pre-/post-save script menu
  entries ("sauvegarde" in lower case).
- New `Config.ini` keys for the update check (`CheckUpdates`, `LastUpdateCheck`,
  `SkipUpdateVersion`).

## Documentation

- The user manual has been updated in **all 32 languages** with a new
  "Checking for updates" section (§4.16), including the privacy note. The manuals
  now use an automatic (field-based) table of contents.

## Downloads

Pick the package for your system:

- **Windows** — installer `XIMOD_Architect_1.0.3_Setup.exe`, or the portable
  archive `ximod-architect-1.0.3-windows-x86_64.zip` (extract and keep the
  executable next to its `assets` folder).
- **Linux** — `ximod-architect-1.0.3-linux-x86_64.tar.gz` (extract and keep the
  executable next to its `assets` folder).
- **macOS** — `ximod-architect-1.0.3-macos-universal.dmg`, or
  `ximod-architect-1.0.3-macos-universal.app.zip` (Apple Silicon + Intel,
  universal binary).

## Note on macOS / Windows signing

The macOS build is signed **ad-hoc** (not notarised): on first launch, right-click
the app and choose **Open**, then confirm, to bypass Gatekeeper. On Windows,
SmartScreen may warn about an unrecognised publisher — choose **More info →
Run anyway**. These warnings are expected for an unsigned open-source build.

## Thanks

Thanks to everyone who reported issues and suggested improvements — in
particular the user who asked for an in-app notification when a new version is
released, which is exactly what this release adds.
