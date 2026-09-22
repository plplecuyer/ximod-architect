# XIMOD Architect v1.0.2

XIMOD Architect is a cross-platform tool for creating FOMOD installers for
Bethesda games (Skyrim, Fallout, Starfield, Oblivion, Morrowind…).

This release brings the conditional-rules editors that were previously only
reachable through the XML editor into the graphical interface, so you can build
"smart" installers without hand-writing XML.

## New features

- **"Plugin dependencies" panel.** In the *Install Steps* tab, each option now
  has a Plugin dependencies section that makes its type dynamic. Keep a
  **Default Type**, then add one or more patterns: for each pattern choose an
  **Operator** (`And` / `Or`), the **type** to apply (`Optional`, `Required`,
  `Recommended`, `NotUsable`, `CouldBeUsable`) and its conditions — a `File`
  dependency on a plugin (state `Active` / `Inactive` / `Missing`) or a `Flag`
  dependency. Typical use: make an option *NotUsable* by default and *Optional*
  only when a required `.esp` is `Active`. This mirrors the "Plugin dependencies"
  panel of the original FOMOD Creation Tool.
- **"Step visibility conditions" panel.** Each step now has a collapsible
  Visibility Conditions section (its `<visible>` block): set the **Operator**
  and add `File` or `Flag` conditions so the step appears in the wizard only when
  they are met — for example a compatibility-patch step shown only when a given
  plugin is active.

Both panels read, write and preserve the corresponding FOMOD XML, and are fully
covered by the FOMOD preview and validation.

## Documentation

- The user manual has been updated in **all 32 languages** with two new
  subsections (§4.2) documenting the Plugin dependencies and Step visibility
  panels, plus a note clarifying plugin (`.esp`) dependencies.

## Other changes

- Refreshed the bundled country reference data (`Countries.json`).
- Version metadata bumped to 1.0.2; generated `info.xml` / `ModuleConfig.xml`
  now carry the comment `<!-- Created with XIMOD Architect 1.0.2 -->`.

## Downloads

Pick the package for your system:

- **Windows** — installer `XIMOD_Architect_1.0.2_Setup.exe`, or the portable
  archive `ximod-architect-1.0.2-windows-x86_64.zip` (extract and keep the
  executable next to its `assets` folder).
- **Linux** — `ximod-architect-1.0.2-linux-x86_64.tar.gz` (extract and keep the
  executable next to its `assets` folder).
- **macOS** — `ximod-architect-1.0.2-macos-universal.dmg`, or
  `ximod-architect-1.0.2-macos-universal.app.zip` (Apple Silicon + Intel,
  universal binary).

## Note on macOS / Windows signing

The macOS build is signed **ad-hoc** (not notarised): on first launch, right-click
the app and choose **Open**, then confirm, to bypass Gatekeeper. On Windows,
SmartScreen may warn about an unrecognised publisher — choose **More info →
Run anyway**. These warnings are expected for an unsigned open-source build.

## Thanks

Thanks to everyone who reported issues and suggested improvements — in
particular the users who asked for `.esp`-based dependency rules in the
interface, which is exactly what this release adds.
