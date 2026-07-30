//! Application icon loading
//!
//! This module handles loading the application icon for the window title bar
//! and taskbar on all platforms.

use eframe::egui;
use std::path::PathBuf;

/// Get the path to the application icon based on the platform
pub fn get_icon_path() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))?;
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, look for .ico file (also works with .png)
        let ico_path = exe_dir.join("ximod-architect.ico");
        if ico_path.exists() {
            return Some(ico_path);
        }
        // Fallback to PNG
        let png_path = exe_dir.join("ximod-architect.png");
        if png_path.exists() {
            return Some(png_path);
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // On Linux, prefer PNG for runtime icon (SVG is for desktop integration)
        let png_path = exe_dir.join("ximod-architect.png");
        if png_path.exists() {
            return Some(png_path);
        }
        // Check standard locations
        let standard_paths = [
            "/usr/share/icons/hicolor/256x256/apps/ximod-architect.png",
            "/usr/local/share/icons/hicolor/256x256/apps/ximod-architect.png",
        ];
        for path in &standard_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, the icon is in the app bundle, but we can also load PNG for runtime
        let png_path = exe_dir.join("ximod-architect.png");
        if png_path.exists() {
            return Some(png_path);
        }
        // Check Resources folder in app bundle
        if let Some(resources) = exe_dir.parent().and_then(|p| Some(p.join("Resources"))) {
            let png_path = resources.join("ximod-architect.png");
            if png_path.exists() {
                return Some(png_path);
            }
        }
    }
    
    None
}

/// Load the application icon as IconData for eframe
pub fn load_icon() -> Option<egui::IconData> {
    // First try to load from file next to executable
    if let Some(path) = get_icon_path() {
        if let Ok(data) = std::fs::read(&path) {
            return load_icon_from_bytes(&data, &path);
        }
    }
    
    // Fallback: try to load embedded icon (if we embed one in the future)
    None
}

/// Load icon from bytes (supports PNG and ICO)
fn load_icon_from_bytes(data: &[u8], path: &PathBuf) -> Option<egui::IconData> {
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match extension.as_str() {
        "png" => load_png_icon(data),
        "ico" => load_ico_icon(data),
        _ => {
            // Try PNG format first, then ICO
            load_png_icon(data).or_else(|| load_ico_icon(data))
        }
    }
}

/// Load icon from PNG data
fn load_png_icon(data: &[u8]) -> Option<egui::IconData> {
    let image = image::load_from_memory(data).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    })
}

/// Load icon from ICO data (extracts the largest image)
fn load_ico_icon(data: &[u8]) -> Option<egui::IconData> {
    let icon_dir = image::codecs::ico::IcoDecoder::new(std::io::Cursor::new(data)).ok()?;
    let image = image::DynamicImage::from_decoder(icon_dir).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    })
}

/// Create viewport icon data for eframe native options
pub fn create_viewport_icon() -> Option<std::sync::Arc<egui::IconData>> {
    load_icon().map(std::sync::Arc::new)
}
