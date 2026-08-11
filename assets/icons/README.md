# XIMOD Architect - Icons

This directory contains the application icons for all platforms.

## Required Icon Files

### Windows (`ximod-architect.ico`)
A multi-resolution ICO file containing:
- 16×16 pixels
- 24×24 pixels
- 32×32 pixels
- 48×48 pixels
- 64×64 pixels
- 128×128 pixels
- 256×256 pixels

**Usage:**
- Embedded in the executable at compile time (via `build.rs`)
- Displayed in Windows Explorer, taskbar, and window title

### Linux (`ximod-architect.svg` + PNGs)
- `ximod-architect.svg` - Scalable vector icon for desktop integration
- PNG files at various sizes for compatibility:
  - `ximod-architect_16x16.png`
  - `ximod-architect_24x24.png`
  - `ximod-architect_32x32.png`
  - `ximod-architect_48x48.png`
  - `ximod-architect_64x64.png`
  - `ximod-architect_128x128.png`
  - `ximod-architect_256x256.png`
  - `ximod-architect_512x512.png`

**Usage:**
- Installed to `/usr/share/icons/hicolor/` via install script
- Referenced in `.desktop` file

### macOS (`ximod-architect.icns`)
An ICNS file containing (64×64 is optional and may be omitted):
- 16×16 pixels (icon_16x16)
- 32×32 pixels (icon_16x16@2x, icon_32x32)
- 128×128 pixels (icon_128x128)
- 256×256 pixels (icon_128x128@2x, icon_256x256)
- 512×512 pixels (icon_256x256@2x, icon_512x512)
- 1024×1024 pixels (icon_512x512@2x)

Note: The 64×64 size (icon_32x32@2x) is optional. If not present, macOS will scale from nearby sizes automatically.

**Usage:**
- Included in the .app bundle
- Referenced in Info.plist

## Creating Icons

### From a Master PNG (1024×1024 recommended)

#### Windows ICO

Using ImageMagick:
```bash
convert master.png -define icon:auto-resize=256,128,64,48,32,24,16 ximod-architect.ico
```

Or using `icotool` (from icoutils):
```bash
# Create individual sizes first
for size in 16 24 32 48 64 128 256; do
    convert master.png -resize ${size}x${size} icon_${size}.png
done
# Combine into ICO
icotool -c -o ximod-architect.ico icon_*.png
```

#### Linux PNGs

```bash
for size in 16 24 32 48 64 128 256 512; do
    convert master.png -resize ${size}x${size} ximod-architect_${size}x${size}.png
done
```

#### Linux SVG

Either:
1. Create the icon directly in a vector graphics program (Inkscape, Adobe Illustrator)
2. Or trace the PNG using Inkscape: File → Import PNG → Path → Trace Bitmap

#### macOS ICNS

Using `iconutil` (macOS only):
```bash
# Create iconset directory
mkdir ximod-architect.iconset

# Create all required sizes (64x64 / icon_32x32@2x is optional)
sips -z 16 16     master.png --out ximod-architect.iconset/icon_16x16.png
sips -z 32 32     master.png --out ximod-architect.iconset/icon_16x16@2x.png
sips -z 32 32     master.png --out ximod-architect.iconset/icon_32x32.png
# sips -z 64 64   master.png --out ximod-architect.iconset/icon_32x32@2x.png  # Optional
sips -z 128 128   master.png --out ximod-architect.iconset/icon_128x128.png
sips -z 256 256   master.png --out ximod-architect.iconset/icon_128x128@2x.png
sips -z 256 256   master.png --out ximod-architect.iconset/icon_256x256.png
sips -z 512 512   master.png --out ximod-architect.iconset/icon_256x256@2x.png
sips -z 512 512   master.png --out ximod-architect.iconset/icon_512x512.png
sips -z 1024 1024 master.png --out ximod-architect.iconset/icon_512x512@2x.png

# Convert to icns
iconutil -c icns ximod-architect.iconset
```

Using ImageMagick on Linux (64x64 optional):
```bash
# Create PNG files first, then use png2icns
convert master.png -resize 1024x1024 icon_1024.png
convert master.png -resize 512x512 icon_512.png
convert master.png -resize 256x256 icon_256.png
convert master.png -resize 128x128 icon_128.png
# convert master.png -resize 64x64 icon_64.png  # Optional
convert master.png -resize 32x32 icon_32.png
convert master.png -resize 16x16 icon_16.png

# Use png2icns (from libicns) - 64x64 optional
png2icns ximod-architect.icns icon_16.png icon_32.png icon_128.png icon_256.png icon_512.png icon_1024.png
# Or with 64x64: png2icns ximod-architect.icns icon_16.png icon_32.png icon_64.png icon_128.png icon_256.png icon_512.png icon_1024.png
```

## Icon Placement

After creating the icons, place them as follows:

```
ximod-architect/
├── assets/
│   └── icons/
│       ├── ximod-architect.ico          # Windows
│       ├── ximod-architect.icns         # macOS
│       ├── ximod-architect.svg          # Linux (vector)
│       ├── ximod-architect.png          # Generic (256x256)
│       ├── ximod-architect_16x16.png    # Linux
│       ├── ximod-architect_24x24.png    # Linux
│       ├── ximod-architect_32x32.png    # Linux
│       ├── ximod-architect_48x48.png    # Linux
│       ├── ximod-architect_64x64.png    # Linux
│       ├── ximod-architect_128x128.png  # Linux
│       ├── ximod-architect_256x256.png  # Linux
│       └── ximod-architect_512x512.png  # Linux
```

## Runtime Icon Loading

For the window icon at runtime:
- **Windows**: Place `ximod-architect.ico` or `ximod-architect.png` next to the executable
- **Linux**: Place `ximod-architect.png` (256×256) next to the executable
- **macOS**: Icon is loaded from the app bundle (ximod-architect.icns in Resources/)

The application will automatically load the icon at startup and display it in the window title bar and taskbar.
