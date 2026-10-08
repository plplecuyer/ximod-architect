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
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock, PoisonError};

/// Fonts directory found by the first successful [`fonts_dir`] lookup.
static FONTS_DIR: OnceLock<PathBuf> = OnceLock::new();

/// One cached font file: its contents plus the size and modification time it
/// had when read, used to notice a file replaced while the program runs.
struct CachedFont {
    len: u64,
    modified: Option<std::time::SystemTime>,
    bytes: &'static [u8],
}

/// Process-wide cache of font file contents, keyed by file path, so each file
/// is read from disk only once however often the wanted-font set changes.
///
/// egui 0.29's `FontData` stores a `Cow<'static, [u8]>` (there is no `Arc`
/// variant yet), so the bytes are leaked once and handed out as `&'static`
/// slices: rebuilding the `FontDefinitions` then costs neither a disk read nor
/// a copy. The leak is bounded by the shipped font set, and is exactly what a
/// never-evicted cache would hold anyway.
static FONT_BYTES: OnceLock<Mutex<HashMap<PathBuf, CachedFont>>> = OnceLock::new();

/// Return the contents of the font file at `path`, reading it only when it is
/// not cached yet or when its size / modification time changed (a `stat` is
/// much cheaper than reading the file).
///
/// Failures are *not* cached: a missing file is retried (and reported) on the
/// next call, as before.
fn font_bytes(path: &Path) -> std::io::Result<&'static [u8]> {
    let meta = std::fs::metadata(path)?;
    let (len, modified) = (meta.len(), meta.modified().ok());

    let cache = FONT_BYTES.get_or_init(|| Mutex::new(HashMap::new()));
    // A poisoned lock only means another thread panicked mid-insert; the map
    // itself is still valid.
    let mut cache = cache.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(hit) = cache.get(path)
        && hit.len == len
        && hit.modified == modified
    {
        return Ok(hit.bytes);
    }
    let bytes: &'static [u8] = Box::leak(std::fs::read(path)?.into_boxed_slice());
    cache.insert(path.to_path_buf(), CachedFont { len, modified, bytes });
    Ok(bytes)
}

/// Locate the fonts directory (`assets/fonts`), production layout first.
///
/// A successful lookup is cached for the lifetime of the process (the
/// executable does not move while running); a failed one is retried.
pub fn fonts_dir() -> Option<PathBuf> {
    if let Some(dir) = FONTS_DIR.get() {
        return Some(dir.clone());
    }
    let dir = locate_fonts_dir()?;
    Some(FONTS_DIR.get_or_init(|| dir).clone())
}

/// Uncached directory probe behind [`fonts_dir`].
fn locate_fonts_dir() -> Option<PathBuf> {
    let rel = PathBuf::from("assets").join("fonts");
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        candidates.push(dir.join(&rel));
        if let Some(up) = dir.parent() {
            candidates.push(up.join("Resources").join(&rel));
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
pub fn build_font_definitions_with_preview(rel_paths: &[String], preview: Option<&str>) -> egui::FontDefinitions {
    let mut fonts = build_defs(rel_paths);

    // The preview family always falls back to the default proportional fonts so
    // Latin text and punctuation still render if the chosen font lacks a glyph.
    let mut chain: Vec<String> = Vec::new();
    if let Some(rel) = preview.filter(|s| !s.is_empty()) {
        let key = rel.to_string();
        if !fonts.font_data.contains_key(&key)
            && let Some(dir) = fonts_dir()
        {
            let path = dir.join(rel.replace('\\', "/"));
            match font_bytes(&path) {
                Ok(bytes) => {
                    fonts.font_data.insert(key.clone(), egui::FontData::from_static(bytes));
                }
                Err(e) => tracing::warn!("Preview font {:?}: {}", path, e),
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
    // The icon font is part of every definition set, since `set_fonts`
    // replaces the whole set each time the language fonts change.
    crate::ui::theme::add_icons(&mut fonts);
    let Some(dir) = fonts_dir() else {
        tracing::warn!("assets/fonts not found: falling back to built-in fonts");
        return fonts;
    };

    for rel in rel_paths {
        if rel.is_empty() {
            continue;
        }
        let path = dir.join(rel.replace('\\', "/"));
        let bytes = match font_bytes(&path) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!("Font {:?} could not be read: {}", path, e);
                continue;
            }
        };

        // The relative path doubles as a unique key.
        let key = rel.clone();
        fonts.font_data.insert(key.clone(), egui::FontData::from_static(bytes));

        // Append as a fallback to both families.
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push(key.clone());
        }
    }

    fonts
}

/// Font files (relative to `assets/fonts`) needed to display `text`, by the
/// Unicode blocks its characters fall in. Latin, Greek and Cyrillic are
/// covered by the default Noto Sans and need nothing; everything else maps
/// to the matching Noto script font shipped with the application. Used so
/// a project (or a translation) written in Japanese, Korean, Arabic… renders
/// whatever the interface language is.
pub fn fonts_for_text(text: &str, out: &mut std::collections::BTreeSet<String>) {
    for c in text.chars() {
        if let Some(f) = font_for_char(c)
            && !out.contains(f)
        {
            out.insert(f.to_string());
        }
    }
}

/// The script font a character needs, if any (see [`fonts_for_text`]).
pub fn font_for_char(c: char) -> Option<&'static str> {
    const JP: &str = "Noto_Sans_JP/static/NotoSansJP-Regular.ttf";
    const KR: &str = "Noto_Sans_KR/static/NotoSansKR-Regular.ttf";
    const SC: &str = "Noto_Sans_SC/static/NotoSansSC-Regular.ttf";
    let u = c as u32;
    Some(match u {
        0x0000..=0x052F => return None, // Latin, Greek, Cyrillic: default font
        0x0530..=0x058F => "Noto_Sans_Armenian/static/NotoSansArmenian-Regular.ttf",
        0x0590..=0x05FF | 0xFB1D..=0xFB4F => "Noto_Sans_Hebrew/static/NotoSansHebrew-Regular.ttf",
        0x0600..=0x06FF | 0x0750..=0x077F | 0x08A0..=0x08FF | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF => {
            "Noto_Sans_Arabic/static/NotoSansArabic-Regular.ttf"
        }
        0x0700..=0x074F => "Noto_Sans_Syriac/static/NotoSansSyriac-Regular.ttf",
        0x0780..=0x07BF => "Noto_Sans_Thaana/static/NotoSansThaana-Regular.ttf",
        0x07C0..=0x07FF => "Noto_Sans_NKo/static/NotoSansNKo-Regular.ttf",
        0x0900..=0x097F | 0xA8E0..=0xA8FF => "Noto_Sans_Devanagari/static/NotoSansDevanagari-Regular.ttf",
        0x0980..=0x09FF => "Noto_Sans_Bengali/static/NotoSansBengali-Regular.ttf",
        0x0A00..=0x0A7F => "Noto_Sans_Gurmukhi/static/NotoSansGurmukhi-Regular.ttf",
        0x0A80..=0x0AFF => "Noto_Sans_Gujarati/static/NotoSansGujarati-Regular.ttf",
        0x0B00..=0x0B7F => "Noto_Sans_Oriya/static/NotoSansOriya-Regular.ttf",
        0x0B80..=0x0BFF => "Noto_Sans_Tamil/static/NotoSansTamil-Regular.ttf",
        0x0C00..=0x0C7F => "Noto_Sans_Telugu/static/NotoSansTelugu-Regular.ttf",
        0x0C80..=0x0CFF => "Noto_Sans_Kannada/static/NotoSansKannada-Regular.ttf",
        0x0D00..=0x0D7F => "Noto_Sans_Malayalam/static/NotoSansMalayalam-Regular.ttf",
        0x0D80..=0x0DFF => "Noto_Sans_Sinhala/static/NotoSansSinhala-Regular.ttf",
        0x0E00..=0x0E7F => "Noto_Sans_Thai/static/NotoSansThai-Regular.ttf",
        0x0E80..=0x0EFF => "Noto_Sans_Lao/static/NotoSansLao-Regular.ttf",
        0x0F00..=0x0FFF => "Noto_Serif_Tibetan/static/NotoSerifTibetan-Regular.ttf",
        0x1000..=0x109F | 0xA9E0..=0xA9FF | 0xAA60..=0xAA7F => "Noto_Sans_Myanmar/NotoSansMyanmar-Regular.ttf",
        0x10A0..=0x10FF | 0x2D00..=0x2D2F => "Noto_Sans_Georgian/static/NotoSansGeorgian-Regular.ttf",
        0x1100..=0x11FF | 0x3130..=0x318F | 0xA960..=0xA97F | 0xAC00..=0xD7FF => KR,
        0x1200..=0x139F | 0x2D80..=0x2DDF => "Noto_Sans_Ethiopic/static/NotoSansEthiopic-Regular.ttf",
        0x13A0..=0x13FF | 0xAB70..=0xABBF => "Noto_Sans_Cherokee/static/NotoSansCherokee-Regular.ttf",
        0x1400..=0x167F | 0x18B0..=0x18FF => {
            "Noto_Sans_Canadian_Aboriginal/static/NotoSansCanadianAboriginal-Regular.ttf"
        }
        0x16A0..=0x16FF => "Noto_Sans_Runic/NotoSansRunic-Regular.ttf",
        0x1780..=0x17FF | 0x19E0..=0x19FF => "Noto_Sans_Khmer/static/NotoSansKhmer-Regular.ttf",
        0x1800..=0x18AF => "Noto_Sans_Mongolian/NotoSansMongolian-Regular.ttf",
        0x1C50..=0x1C7F => "Noto_Sans_Ol_Chiki/static/NotoSansOlChiki-Regular.ttf",
        0x2D30..=0x2D7F => "Noto_Sans_Tifinagh/static/NotoSansTifinagh-Regular.ttf",
        // Japanese kana first: a text with kana is Japanese, and its Han
        // characters should come from the Japanese font.
        0x3040..=0x30FF | 0x31F0..=0x31FF | 0xFF66..=0xFF9F => JP,
        // Han, CJK punctuation and full-width forms: Simplified Chinese font
        // (the JP/KR fonts also cover them when the text is Japanese/Korean,
        // since every needed font is appended to the fallback chain).
        0x2E80..=0x303F
        | 0x3100..=0x312F
        | 0x3190..=0x31EF
        | 0x3200..=0x9FFF
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F
        | 0xFF00..=0xFF65
        | 0xFFA0..=0xFFEF
        | 0x20000..=0x3134F => SC,
        0xA000..=0xA4CF => "Noto_Sans_Yi/NotoSansYi-Regular.ttf",
        0xA4D0..=0xA4FF => "Noto_Sans_Lisu/static/NotoSansLisu-Regular.ttf",
        0xA500..=0xA63F => "Noto_Sans_Vai/NotoSansVai-Regular.ttf",
        0xA980..=0xA9DF => "Noto_Sans_Javanese/static/NotoSansJavanese-Regular.ttf",
        0xAA00..=0xAA5F => "Noto_Sans_Cham/static/NotoSansCham-Regular.ttf",
        0xAA80..=0xAADF => "Noto_Sans_Tai_Viet/NotoSansTaiViet-Regular.ttf",
        _ => return None,
    })
}

/// Open a URL in the user's default browser (used for the Google Fonts link).
///
/// Failure is not an error worth interrupting the user for: it is logged and
/// ignored, since the address is also shown in the interface.
pub fn open_url(url: &str) {
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn();
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

#[cfg(test)]
mod script_tests {
    use super::*;

    #[test]
    fn fonts_for_text_maps_scripts() {
        let mut out = std::collections::BTreeSet::new();
        fonts_for_text("Latin, Ελληνικά, Кириллица", &mut out);
        assert!(out.is_empty(), "default font covers these: {out:?}");
        fonts_for_text("日本語 テクスチャ", &mut out);
        assert!(out.iter().any(|f| f.contains("NotoSansJP")));
        assert!(out.iter().any(|f| f.contains("NotoSansSC")), "Han characters: {out:?}");
        fonts_for_text("한국어", &mut out);
        assert!(out.iter().any(|f| f.contains("NotoSansKR")));
        fonts_for_text("العربية עברית ไทย हिन्दी", &mut out);
        for f in ["Arabic", "Hebrew", "Thai", "Devanagari"] {
            assert!(out.iter().any(|p| p.contains(f)), "{f} missing in {out:?}");
        }
    }
}
