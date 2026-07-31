# XIMOD Architect

A cross-platform FOMOD installer creation tool for Bethesda game mods (Skyrim, Fallout 4, Starfield, etc.).

This is a Rust port of Wenderer's original FOMOD Creation Tool, offering improved performance, cross-platform support, and a modern UI.

## Features

- **FOMOD Package Creation**: Create complete FOMOD installer packages
- **Multi-Step Wizards**: Design complex installation wizards with multiple steps
- **Plugin Groups**: Organize plugins with various selection types (SelectOne, SelectAny, etc.)
- **Conditional Installation**: Set up file installation based on user choices
- **Dependency Patterns**: Configure plugin types based on dependencies
- **Flag System**: Use condition flags to control installation flow
- **Pre/Post Save Scripts**: Execute custom scripts before or after saving
- **Multi-Language Support**: Interface available in 32 languages, including non-Latin scripts (Japanese, Simplified Chinese, Korean, Russian), with dynamic loading of the bundled Noto fonts
- **Built-in Tools**: XML editor with live validation, read-only country/language explorer, and an in-app translation editor
- **Cross-Platform**: Works on Windows, Linux, and macOS
- **Native Splash Screen**: Transparent splash screen with fade effect

## Building

### Prerequisites

- Rust 1.85+ (install from https://rustup.rs)
- On Linux, the system development libraries. The quickest way is the helper script,
  which detects your distribution (Debian/Ubuntu, Fedora, Arch):

  ```bash
  ./packaging/linux/build-deps.sh          # add -y for non-interactive
  ```

  Or install them manually on Debian/Ubuntu:
  `sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev libx11-dev`

### Compile

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run --release
```

The executable will be in `target/release/ximod-architect` (or `.exe` on Windows).

> For a detailed, step-by-step build guide (local build and automated CI builds),
> see [`COMPILING-linux.md`](COMPILING-linux.md).

### Application Icons

Place your icon files in `assets/icons/` before building:
- `ximod-architect.ico` - Windows (multi-resolution: 16-256px)
- `ximod-architect.icns` - macOS (multi-resolution: 16-1024px)
- `ximod-architect.svg` - Linux (scalable)
- `ximod-architect.png` - Runtime icon (256×256)

See `assets/icons/README.md` for detailed icon creation instructions.

## Installation

### Windows

1. Build the release executable
2. Copy to your desired location:
   - `ximod-architect.exe`
   - `ximod-architect.ico` (or `.png`) - for window icon
   - `assets/images/splash.png` (optional) - for splash screen
3. Run `ximod-architect.exe`

The application icon is embedded in the executable at compile time.

> To build a proper **`setup.exe`** installer (Start-menu shortcut, uninstaller),
> see [`INSTALLER-windows.md`](INSTALLER-windows.md). The release workflow also builds
> it automatically on each tag.

### Linux

Use the provided installation script:

```bash
# System-wide installation (requires sudo)
sudo ./packaging/linux/install.sh

# User installation
./packaging/linux/install.sh --user
```

Or manually:
```bash
# Copy binary
sudo cp target/release/ximod-architect /usr/local/bin/

# Install icon
sudo cp assets/icons/ximod-architect.svg /usr/share/icons/hicolor/scalable/apps/

# Install .desktop file
sudo cp packaging/linux/ximod-architect.desktop /usr/share/applications/

# Update caches
sudo gtk-update-icon-cache /usr/share/icons/hicolor
sudo update-desktop-database
```

### macOS

Create an application bundle (add `--target universal` for an arm64 + x86_64 binary,
`--dmg` to also produce a disk image, and `--identity "…" --notarize` to sign and
notarize for distribution):

```bash
./packaging/macos/create-bundle.sh --target universal --dmg
```

This creates `XIMOD Architect.app` (and a `.dmg`) in the `dist/` directory. Run
`./packaging/macos/create-bundle.sh --help` for all options.

## Usage

1. **Create New Project**: File → New, then select a root directory containing your mod files
2. **Set Mod Info**: Fill in the mod name, author, version, and description in the Info tab
3. **Create Steps**: Use the Steps tab to create installation steps
4. **Add Groups**: Within each step, add plugin groups with different selection types
5. **Add Plugins**: Add plugins to groups with descriptions, images, and files
6. **Configure Files**: Add files/folders to install for each plugin
7. **Set Conditions**: Configure visibility conditions and dependency patterns
8. **Save**: File → Save to generate info.xml and ModuleConfig.xml in the fomod folder

## Project Structure

```
ximod-architect/
├── src/
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── icon.rs           # Icon loading
│   ├── splash/           # Native splash screen
│   ├── models/           # Data structures (FOMOD model)
│   ├── ui/               # User interface
│   ├── xml/              # XML serialization
│   └── i18n/             # Internationalization
├── assets/
│   ├── data/             # Games/categories, countries, languages (JSON)
│   ├── fonts/            # Bundled Noto fonts (one per writing system)
│   ├── icons/            # Application icons (.ico, .icns, .svg, .png)
│   ├── images/           # Splash screen + country flags (svg/)
│   └── locales/          # Translation files (ISO 639-3 folders: eng, fra, jpn, …)
├── packaging/
│   ├── linux/            # install.sh, build-deps.sh, .desktop
│   └── macos/            # create-bundle.sh, entitlements.plist, Info.plist
├── .github/workflows/    # release.yml (CI: Linux + Windows + macOS on tags)
├── COMPILING-linux.md    # Detailed build guide
├── build.rs              # Build script (Windows icon embedding)
└── Cargo.toml
```

## Internationalization

XIMOD Architect ships with **32 interface translations**. They are loaded dynamically
from the `assets/locales/` directory at runtime. Each language has its own folder named
with the ISO 639-3 code (e.g. `assets/locales/eng/`, `fra/`, `deu/`, `jpn/`, `zho/`,
`kor/`, `rus/`, `tur/`, …). English (`eng`) and French (`fra`) are the reference translations;
the others can be reviewed and corrected directly in the built-in translation editor.

To add a new language:
1. Create a folder with the ISO 639-3 code (e.g., `assets/locales/deu/`)
2. Copy `main.ftl` from an existing language
3. Translate all strings (the built-in translation editor does this for you and protects macros/variables)
4. The language will appear automatically in Settings

## FOMOD Format

XIMOD Architect generates standard FOMOD installer files:

- `fomod/info.xml` - Mod metadata (name, author, version, etc.)
- `fomod/ModuleConfig.xml` - Installation configuration

The generated files are compatible with:
- Mod Organizer 2
- Vortex
- NMM (Nexus Mod Manager)

## Selection Types

- **SelectExactlyOne**: User must select exactly one plugin
- **SelectAtMostOne**: User can select at most one plugin (or none)
- **SelectAny**: User can select any number of plugins
- **SelectAll**: All plugins are selected by default
- **SelectAtLeastOne**: User must select at least one plugin

## Plugin Types

- **Optional**: Plugin is optional (default)
- **Required**: Plugin must be installed
- **Recommended**: Plugin is recommended but optional
- **NotUsable**: Plugin cannot be used
- **CouldBeUsable**: Plugin might be usable in certain conditions

## Configuration

Settings are stored in a `Config.ini` file. XIMOD Architect chooses its location
automatically:

- **Portable mode** — if a `Config.ini` already sits next to the executable, it is
  used directly and nothing is written elsewhere (ideal for USB sticks, mod managers,
  or the portable `.zip` archives from CI).
- **Installed mode** — otherwise the settings go to the per-user configuration
  directory: `%APPDATA%\XIMOD Architect` on Windows, `~/.config/XIMOD Architect` on
  Linux, `~/Library/Application Support/XIMOD Architect` on macOS. This lets the app be
  installed under `C:\Program Files` (all users) without needing write access there.

## License

MIT License - see LICENSE file for details.

## Credits

XIMOD Architect is a Rust port of **Wenderer's** original
[FOMOD Creation Tool](https://www.nexusmods.com/fallout4/mods/6821). Full credit and
thanks to Wenderer for the original work, on which this port is based.

Language codes are from the ISO 639-3 standard, courtesy of SIL International
(iso639-3.sil.org). Bundled fonts are Google's Noto family.

## Permissions & terms

XIMOD Architect is a derivative work of Wenderer's FOMOD Creation Tool and is published
with Wenderer's permission, under the terms he set:

- **Credit** — Wenderer is credited as the author of the original code, with a link to
  the [original tool](https://www.nexusmods.com/fallout4/mods/6821).
- **Donation Points** — 50% of the Donation Points earned from unique downloads go to
  Wenderer (direct/P2P donations excluded).
- **Open source** — the program stays open source, with its code publicly available.
- **Free for everyone** — all features, current and future, stay free of charge, with
  no premium tier or paywall, so the tool remains fully accessible to the whole modding
  community.
