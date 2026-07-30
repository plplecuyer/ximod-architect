#!/bin/bash
# XIMOD Architect - Linux Installation Script
# This script installs the application and its icons to standard Linux locations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_NAME="ximod-architect"

# Determine install prefix (default: /usr/local, or ~/.local for user install)
if [ "$1" == "--user" ]; then
    PREFIX="$HOME/.local"
    ICON_DIR="$HOME/.local/share/icons/hicolor"
    DESKTOP_DIR="$HOME/.local/share/applications"
    echo "Installing for current user to $PREFIX..."
else
    PREFIX="/usr/local"
    ICON_DIR="/usr/share/icons/hicolor"
    DESKTOP_DIR="/usr/share/applications"
    echo "Installing system-wide to $PREFIX (may require sudo)..."
fi

# Create directories
mkdir -p "$PREFIX/bin"
mkdir -p "$PREFIX/bin/assets/images"
mkdir -p "$PREFIX/bin/assets/locales"
mkdir -p "$DESKTOP_DIR"
mkdir -p "$ICON_DIR/scalable/apps"
mkdir -p "$ICON_DIR/16x16/apps"
mkdir -p "$ICON_DIR/24x24/apps"
mkdir -p "$ICON_DIR/32x32/apps"
mkdir -p "$ICON_DIR/48x48/apps"
mkdir -p "$ICON_DIR/64x64/apps"
mkdir -p "$ICON_DIR/128x128/apps"
mkdir -p "$ICON_DIR/256x256/apps"
mkdir -p "$ICON_DIR/512x512/apps"

# Install binary
if [ -f "$SCRIPT_DIR/$APP_NAME" ]; then
    cp "$SCRIPT_DIR/$APP_NAME" "$PREFIX/bin/"
    chmod +x "$PREFIX/bin/$APP_NAME"
    echo "✓ Installed binary to $PREFIX/bin/$APP_NAME"
else
    echo "✗ Binary not found: $SCRIPT_DIR/$APP_NAME"
    exit 1
fi

# Install splash image
if [ -f "$SCRIPT_DIR/assets/images/splash.png" ]; then
    cp "$SCRIPT_DIR/assets/images/splash.png" "$PREFIX/bin/assets/images/"
    echo "✓ Installed splash image"
elif [ -f "$SCRIPT_DIR/splash.png" ]; then
    mkdir -p "$PREFIX/bin/assets/images"
    cp "$SCRIPT_DIR/splash.png" "$PREFIX/bin/assets/images/"
    echo "✓ Installed splash image (from root)"
fi

# Install locale files (ISO 639-3 folders)
if [ -d "$SCRIPT_DIR/assets/locales" ]; then
    cp -r "$SCRIPT_DIR/assets/locales/"* "$PREFIX/bin/assets/locales/"
    echo "✓ Installed locale files"
fi

# Install fonts (needed to render every writing system)
if [ -d "$SCRIPT_DIR/assets/fonts" ]; then
    mkdir -p "$PREFIX/bin/assets/fonts"
    cp -r "$SCRIPT_DIR/assets/fonts/"* "$PREFIX/bin/assets/fonts/"
    echo "✓ Installed fonts"
fi

# Install flag images (SVG)
if [ -d "$SCRIPT_DIR/assets/images/svg" ]; then
    mkdir -p "$PREFIX/bin/assets/images/svg"
    cp -r "$SCRIPT_DIR/assets/images/svg/"* "$PREFIX/bin/assets/images/svg/"
    echo "✓ Installed flags"
fi

# Install game/category data (assets/data/*.json)
if [ -d "$SCRIPT_DIR/assets/data" ]; then
    mkdir -p "$PREFIX/bin/assets/data"
    cp -r "$SCRIPT_DIR/assets/data/"* "$PREFIX/bin/assets/data/"
    echo "✓ Installed game/category data"
fi

# Install SVG icon (scalable)
if [ -f "$SCRIPT_DIR/$APP_NAME.svg" ]; then
    cp "$SCRIPT_DIR/$APP_NAME.svg" "$ICON_DIR/scalable/apps/"
    echo "✓ Installed SVG icon"
fi

# Install PNG icons at various sizes
for size in 16 24 32 48 64 128 256 512; do
    png_file="$SCRIPT_DIR/icons/${APP_NAME}_${size}x${size}.png"
    if [ -f "$png_file" ]; then
        cp "$png_file" "$ICON_DIR/${size}x${size}/apps/$APP_NAME.png"
        echo "✓ Installed ${size}x${size} icon"
    fi
done

# Also copy a PNG next to the binary for runtime loading
if [ -f "$SCRIPT_DIR/icons/${APP_NAME}_256x256.png" ]; then
    cp "$SCRIPT_DIR/icons/${APP_NAME}_256x256.png" "$PREFIX/bin/$APP_NAME.png"
elif [ -f "$SCRIPT_DIR/$APP_NAME.png" ]; then
    cp "$SCRIPT_DIR/$APP_NAME.png" "$PREFIX/bin/"
fi

# Install .desktop file
DESKTOP_FILE="$SCRIPT_DIR/$APP_NAME.desktop"
if [ -f "$DESKTOP_FILE" ]; then
    # Update Exec path for user installation
    if [ "$1" == "--user" ]; then
        sed "s|Exec=ximod-architect|Exec=$PREFIX/bin/ximod-architect|g" "$DESKTOP_FILE" > "$DESKTOP_DIR/$APP_NAME.desktop"
    else
        cp "$DESKTOP_FILE" "$DESKTOP_DIR/"
    fi
    echo "✓ Installed desktop entry"
fi

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$ICON_DIR" 2>/dev/null || true
    echo "✓ Updated icon cache"
fi

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
    echo "✓ Updated desktop database"
fi

echo ""
echo "Installation complete!"
echo "You can now launch XIMOD Architect from your application menu or by running: $APP_NAME"
