//! Dynamic font loading.
//!
//! XIMOD ships ~67 distinct Regular fonts (Noto family) covering every writing
//! system of the language table. Loading them all at once would waste a lot of
//! memory, so instead only the fonts actually needed are installed:
//!
//! - the font of the current interface language, and
//! - the fonts of every language listed for the selected country (that is what
//!   the language drop-down displays).
//!
//! In practice this is a single font for most countries (median 1, maximum 28
//! for India), so the cost stays small.
//!
//! The loaded fonts are appended *after* egui's built-in ones: Latin text keeps
//! using the default typeface, and any glyph missing from it falls through to
//! our fonts. That is what makes scripts such as Nyiakeng Puachue Hmong render
//! instead of showing tofu boxes (□).

use eframe::egui;
use std::path::PathBuf;

/// Locate the fonts directory (`assets/fonts`), production layout first.
pub fn fonts_dir() -> Option<PathBuf> {
    let rel = PathBuf::from("assets").join("fonts");
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join(&rel));
            if let Some(up) = dir.parent() {
                candidates.push(up.join("Resources").join(&rel));
            }
        }
    }
    candidates.push(rel);

    candidates.into_iter().find(|p| p.is_dir())
}

/// Build a `FontDefinitions` containing egui's defaults plus every font in
/// `rel_paths` (paths relative to `assets/fonts`), appended as fallbacks.
///
/// Unreadable or missing files are skipped, so a broken entry in
/// `Languages.json` degrades to tofu instead of crashing the application.
/// Named font family used by the translation editor to preview a language in
/// its own typeface, independently of the interface font.
pub const PREVIEW_FAMILY: &str = "ximod-preview";

/// Same as [`build_font_definitions`], plus an optional font registered under
/// [`PREVIEW_FAMILY`] so a single widget can be drawn with it.
pub fn build_font_definitions_with_preview(
    rel_paths: &[String],
    preview: Option<&str>,
) -> egui::FontDefinitions {
    let mut fonts = build_defs(rel_paths);

    // The preview family always falls back to the default proportional fonts so
    // Latin text and punctuation still render if the chosen font lacks a glyph.
    let mut chain: Vec<String> = Vec::new();
    if let Some(rel) = preview.filter(|s| !s.is_empty()) {
        let key = rel.to_string();
        if !fonts.font_data.contains_key(&key) {
            if let Some(dir) = fonts_dir() {
                let path = dir.join(rel.replace('\\', "/"));
                match std::fs::read(&path) {
                    Ok(bytes) => {
                        fonts
                            .font_data
                            .insert(key.clone(), egui::FontData::from_owned(bytes));
                    }
                    Err(e) => tracing::warn!("Preview font {:?}: {}", path, e),
                }
            }
        }
        if fonts.font_data.contains_key(&key) {
            chain.push(key);
        }
    }
    if let Some(defaults) = fonts.families.get(&egui::FontFamily::Proportional) {
        chain.extend(defaults.iter().cloned());
    }
    fonts
        .families
        .insert(egui::FontFamily::Name(PREVIEW_FAMILY.into()), chain);

    fonts
}

fn build_defs(rel_paths: &[String]) -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    let Some(dir) = fonts_dir() else {
        tracing::warn!("assets/fonts not found: falling back to built-in fonts");
        return fonts;
    };

    for rel in rel_paths {
        if rel.is_empty() {
            continue;
        }
        let path = dir.join(rel.replace('\\', "/"));
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!("Font {:?} could not be read: {}", path, e);
                continue;
            }
        };

        // The relative path doubles as a unique key.
        let key = rel.clone();
        fonts
            .font_data
            .insert(key.clone(), egui::FontData::from_owned(bytes));

        // Append as a fallback to both families.
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push(key.clone());
        }
    }

    fonts
}

/// Open a URL in the user's default browser (used for the Google Fonts link).
///
/// Failure is not an error worth interrupting the user for: it is logged and
/// ignored, since the address is also shown in the interface.
pub fn open_url(url: &str) {
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let result = std::process::Command::new("xdg-open").arg(url).spawn();

    if let Err(e) = result {
        tracing::warn!("Could not open {}: {}", url, e);
    }
}

/// Reveal a folder in the system file manager (used after building the
/// translation archive, so the user can attach it to their e-mail).
pub fn open_path(path: &std::path::Path) {
    let s = path.to_string_lossy().to_string();
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("explorer").arg(&s).spawn();
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(&s).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let result = std::process::Command::new("xdg-open").arg(&s).spawn();

    if let Err(e) = result {
        tracing::warn!("Could not open {}: {}", s, e);
    }
}
