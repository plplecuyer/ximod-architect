#!/bin/bash
# XIMOD Architect - Linux Uninstallation Script

set -e

APP_NAME="ximod-architect"

# Determine install prefix
if [ "$1" == "--user" ]; then
    PREFIX="$HOME/.local"
    ICON_DIR="$HOME/.local/share/icons/hicolor"
    DESKTOP_DIR="$HOME/.local/share/applications"
    echo "Uninstalling from user directory..."
else
    PREFIX="/usr/local"
    ICON_DIR="/usr/share/icons/hicolor"
    DESKTOP_DIR="/usr/share/applications"
    echo "Uninstalling from system directory (may require sudo)..."
fi

# Remove binary and assets
rm -f "$PREFIX/bin/$APP_NAME"
rm -f "$PREFIX/bin/$APP_NAME.png"
rm -rf "$PREFIX/bin/assets"
echo "✓ Removed binary and assets"

# Remove icons
rm -f "$ICON_DIR/scalable/apps/$APP_NAME.svg"
for size in 16 24 32 48 64 128 256 512; do
    rm -f "$ICON_DIR/${size}x${size}/apps/$APP_NAME.png"
done
echo "✓ Removed icons"

# Remove desktop file
rm -f "$DESKTOP_DIR/$APP_NAME.desktop"
echo "✓ Removed desktop entry"

# Update caches
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$ICON_DIR" 2>/dev/null || true
fi
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
fi

echo ""
echo "Uninstallation complete!"
