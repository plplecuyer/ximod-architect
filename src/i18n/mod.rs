//! Internationalization module
//!
//! Uses Fluent for translations, loading files dynamically from disk.
//! Languages are organized by ISO 639-3 codes (fra, eng, deu, etc.)

use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;

use crate::data::LanguagesData;

/// Built-in ISO 639-1 ↔ ISO 639-3 ↔ display-name table.
///
/// This is now only a **fallback**: the canonical language data is loaded at
/// runtime from `assets/data/Languages.json` (see `LanguagesData`). This table
/// is used to migrate legacy ISO 639-1 `Config.ini` values and as a safety net
/// if `Languages.json` is missing.
const LANGUAGE_MAP: &[(&str, &str, &str)] = &[
    // (ISO 639-1, ISO 639-3, Display Name)
    ("en", "eng", "English"),
    ("fr", "fra", "Français"),
    ("de", "deu", "Deutsch"),
    ("es", "spa", "Español"),
    ("it", "ita", "Italiano"),
    ("pt", "por", "Português"),
    ("ru", "rus", "Русский"),
    ("zh", "zho", "中文"),
    ("ja", "jpn", "日本語"),
    ("ko", "kor", "한국어"),
    ("pl", "pol", "Polski"),
    ("nl", "nld", "Nederlands"),
    ("sv", "swe", "Svenska"),
    ("da", "dan", "Dansk"),
    ("no", "nor", "Norsk"),
    ("fi", "fin", "Suomi"),
    ("cs", "ces", "Čeština"),
    ("hu", "hun", "Magyar"),
    ("tr", "tur", "Türkçe"),
    ("ar", "ara", "العربية"),
    ("he", "heb", "עברית"),
    ("th", "tha", "ไทย"),
    ("vi", "vie", "Tiếng Việt"),
    ("uk", "ukr", "Українська"),
    ("el", "ell", "Ελληνικά"),
    ("ro", "ron", "Română"),
    ("bg", "bul", "Български"),
    ("hr", "hrv", "Hrvatski"),
    ("sk", "slk", "Slovenčina"),
    ("sl", "slv", "Slovenščina"),
    ("et", "est", "Eesti"),
    ("lv", "lav", "Latviešu"),
    ("lt", "lit", "Lietuvių"),
    ("ga", "gle", "Gaeilge"),
    ("mt", "mlt", "Malti"),
    ("rm", "roh", "Rumantsch"),
];

/// Convert ISO 639-1 code to ISO 639-3 code.
///
/// Legacy helper (superseded by `normalize_locale`, which preserves ISO 639-3
/// codes absent from the built-in table). Kept for tests and reference.
#[allow(dead_code)]
pub fn iso639_1_to_3(code: &str) -> String {
    let code_lower = code.to_lowercase();
    
    // Check if it's already an ISO 639-3 code
    if code_lower.len() == 3 {
        if LANGUAGE_MAP.iter().any(|(_, iso3, _)| *iso3 == code_lower) {
            return code_lower;
        }
    }
    
    // Map from ISO 639-1 to ISO 639-3
    for (iso1, iso3, _) in LANGUAGE_MAP {
        if *iso1 == code_lower {
            return iso3.to_string();
        }
    }
    
    // Default to English if unknown
    "eng".to_string()
}

/// Convert ISO 639-3 code to ISO 639-1 code
pub fn iso639_3_to_1(code: &str) -> String {
    let code_lower = code.to_lowercase();
    
    // Check if it's already an ISO 639-1 code
    if code_lower.len() == 2 {
        if LANGUAGE_MAP.iter().any(|(iso1, _, _)| *iso1 == code_lower) {
            return code_lower;
        }
    }
    
    // Map from ISO 639-3 to ISO 639-1
    for (iso1, iso3, _) in LANGUAGE_MAP {
        if *iso3 == code_lower {
            return iso1.to_string();
        }
    }
    
    // Default to English if unknown
    "en".to_string()
}

/// Get display name for a locale code (accepts both ISO 639-1 and ISO 639-3)
pub fn locale_display_name(locale: &str) -> &'static str {
    let locale_lower = locale.to_lowercase();
    
    for (iso1, iso3, name) in LANGUAGE_MAP {
        if *iso1 == locale_lower || *iso3 == locale_lower {
            return name;
        }
    }
    
    "Unknown"
}

/// Normalize a locale code to ISO 639-3.
///
/// A 2-letter code is treated as ISO 639-1 and mapped to its ISO 639-3 form via
/// the built-in table (used to migrate legacy `Config.ini` values). Anything
/// else is assumed to already be ISO 639-3 and passed through unchanged — so
/// languages without an ISO 639-1 code (e.g. "nso") are preserved instead of
/// being coerced to English.
pub fn normalize_locale(code: &str) -> String {
    let lower = code.trim().to_lowercase();
    if lower.len() == 2 {
        for (iso1, iso3, _) in LANGUAGE_MAP {
            if *iso1 == lower {
                return iso3.to_string();
            }
        }
    }
    lower
}

/// Internationalization manager
pub struct I18n {
    bundle: Option<FluentBundle<FluentResource>>,
    fallback_bundle: Option<FluentBundle<FluentResource>>,
    current_locale: String,       // ISO 639-3 code
    locales_dir: PathBuf,
    available_locales: Vec<String>, // ISO 639-3 codes
    /// Canonical language table loaded from assets/data/Languages.json.
    languages: LanguagesData,
}

impl I18n {
    /// Create a new I18n instance
    pub fn new() -> Self {
        let locales_dir = Self::get_locales_dir();
        let available = Self::scan_available_locales(&locales_dir);
        let languages = LanguagesData::load();

        tracing::info!("Locales directory: {:?}", locales_dir);
        tracing::info!("Available locales: {:?}", available);

        Self {
            bundle: None,
            fallback_bundle: None,
            current_locale: "eng".to_string(),
            locales_dir,
            available_locales: available,
            languages,
        }
    }
    
    /// Get the locales directory path (next to executable)
    /// Locate the locales directory. Primary location is `assets/locales/`;
    /// the old `locales/` layout is kept as a fallback. Works both next to the
    /// executable (production) and from the project root (`cargo run`).
    fn get_locales_dir() -> PathBuf {
        let mut candidates: Vec<PathBuf> = Vec::new();

        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                candidates.push(dir.join("assets").join("locales"));
                candidates.push(dir.join("locales"));
                // macOS .app bundle: Contents/Resources/…
                if let Some(up) = dir.parent() {
                    let res = up.join("Resources");
                    candidates.push(res.join("assets").join("locales"));
                    candidates.push(res.join("locales"));
                }
            }
        }
        // Development layout (running via `cargo run`).
        candidates.push(PathBuf::from("assets").join("locales"));
        candidates.push(PathBuf::from("locales"));

        // First existing directory wins; otherwise fall back to assets/locales.
        candidates
            .iter()
            .find(|p| p.is_dir())
            .cloned()
            .unwrap_or_else(|| PathBuf::from("assets/locales"))
    }
    
    /// Scan for available locales by checking for directories with main.ftl
    fn scan_available_locales(locales_dir: &PathBuf) -> Vec<String> {
        let mut locales = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(locales_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let main_ftl = path.join("main.ftl");
                    if main_ftl.exists() {
                        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                            // Folder names are ISO 639-3; normalize handles a
                            // stray 2-letter folder too, and preserves any
                            // 3-letter code (incl. those absent from the table).
                            locales.push(normalize_locale(dir_name));
                        }
                    }
                }
            }
        }
        
        // Sort alphabetically
        locales.sort();
        locales
    }
    
    /// Refresh the list of available locales
    #[allow(dead_code)]
    pub fn refresh_available_locales(&mut self) {
        self.available_locales = Self::scan_available_locales(&self.locales_dir);
    }

    /// Set the current locale (accepts both ISO 639-1 and ISO 639-3 codes)
    pub fn set_locale(&mut self, locale: &str) {
        let iso3_code = normalize_locale(locale);
        
        tracing::info!("Setting locale: {} -> {}", locale, iso3_code);
        
        // Clear current bundles
        self.bundle = None;
        
        // Load the requested locale
        match self.load_locale_bundle(&iso3_code) {
            Some(bundle) => {
                self.bundle = Some(bundle);
                self.current_locale = iso3_code.clone();
                tracing::info!("Loaded locale: {}", iso3_code);
            }
            _ => {
                tracing::warn!("Failed to load locale: {}, trying fallback", iso3_code);
            }
        }
        
        // Load English as fallback if not already the current locale
        if iso3_code != "eng" {
            if self.fallback_bundle.is_none() {
                self.fallback_bundle = self.load_locale_bundle("eng");
            }
        } else {
            self.fallback_bundle = None;
        }
        
        // If no bundle loaded, try English
        if self.bundle.is_none() && iso3_code != "eng" {
            if let Some(bundle) = self.load_locale_bundle("eng") {
                self.bundle = Some(bundle);
                self.current_locale = "eng".to_string();
                tracing::info!("Fell back to English");
            }
        }
    }
    
    /// Load a locale bundle from disk
    fn load_locale_bundle(&self, iso3_code: &str) -> Option<FluentBundle<FluentResource>> {
        let locale_dir = self.locales_dir.join(iso3_code);
        let main_ftl = locale_dir.join("main.ftl");
        
        tracing::debug!("Loading locale file: {:?}", main_ftl);
        
        // Read main.ftl
        let content = match std::fs::read_to_string(&main_ftl) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Failed to read {:?}: {}", main_ftl, e);
                return None;
            }
        };
        
        // Fluent language identifier: prefer the ISO 639-1 from Languages.json,
        // fall back to the built-in table, then to "en".
        let iso1_code = self
            .languages
            .iso3_to_iso1(iso3_code)
            .map(|s| s.to_string())
            .unwrap_or_else(|| iso639_3_to_1(iso3_code));
        let lang_id: LanguageIdentifier = iso1_code.parse()
            .unwrap_or_else(|_| "en".parse().unwrap());
        
        // Create the resource
        let resource = match FluentResource::try_new(content) {
            Ok(res) => res,
            Err((res, errors)) => {
                for err in &errors {
                    tracing::warn!("Fluent parse error in {}: {:?}", iso3_code, err);
                }
                res // Use partial resource
            }
        };
        
        // Create the bundle
        let mut bundle = FluentBundle::new(vec![lang_id]);
        if let Err(errors) = bundle.add_resource(resource) {
            for err in errors {
                tracing::warn!("Fluent bundle error in {}: {:?}", iso3_code, err);
            }
        }
        
        // Load additional .ftl files from the same directory
        if let Ok(entries) = std::fs::read_dir(&locale_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "ftl" && path.file_name() != Some(std::ffi::OsStr::new("main.ftl")) {
                            if let Ok(additional_content) = std::fs::read_to_string(&path) {
                                if let Ok(additional_resource) = FluentResource::try_new(additional_content) {
                                    let _ = bundle.add_resource(additional_resource);
                                    tracing::debug!("Loaded additional file: {:?}", path);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Some(bundle)
    }

    /// Get current locale (ISO 639-3 code)
    pub fn current_locale(&self) -> &str {
        &self.current_locale
    }

    /// True when the code is a known ISO 639-3 language (present in
    /// `Languages.json`). Translating is only allowed for such languages.
    pub fn has_language(&self, iso3: &str) -> bool {
        self.languages.contains(iso3)
    }

    /// Font covering a language's script: path relative to `assets/fonts/`.
    pub fn font_for(&self, iso3: &str) -> Option<&str> {
        self.languages.font_for(iso3)
    }

    /// Human-readable name for a locale code (ISO 639-3, or ISO 639-1 for the
    /// built-in fallback). Prefers `Languages.json`; falls back to the built-in
    /// table; last resort returns the code itself (never "Unknown").
    pub fn display_name(&self, code: &str) -> String {
        if let Some(name) = self.languages.display_name(code) {
            return name.to_string();
        }
        let builtin = locale_display_name(code);
        if builtin != "Unknown" {
            return builtin.to_string();
        }
        code.to_string()
    }
    
    /// Get current locale as ISO 639-1 code (for Config.ini compatibility)
    #[allow(dead_code)]
    pub fn current_locale_iso1(&self) -> String {
        iso639_3_to_1(&self.current_locale)
    }

    /// Get available locales (ISO 639-3 codes)
    pub fn available_locales(&self) -> &[String] {
        &self.available_locales
    }
    
    /// Get available locales as ISO 639-1 codes
    #[allow(dead_code)]
    pub fn available_locales_iso1(&self) -> Vec<String> {
        self.available_locales.iter()
            .map(|iso3| iso639_3_to_1(iso3))
            .collect()
    }

    /// The canonical language table (Languages.json), for the Properties window.
    pub fn languages(&self) -> &crate::data::LanguagesData {
        &self.languages
    }

    /// Path to the locales directory (used by the translation editor).
    pub fn locales_dir(&self) -> &std::path::Path {
        &self.locales_dir
    }

    /// Translate a key
    pub fn t(&self, key: &str) -> String {
        self.t_with_args(key, None)
    }

    /// Translate a key with arguments
    pub fn t_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        // Try current locale bundle first
        if let Some(bundle) = &self.bundle {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let result = bundle.format_pattern(pattern, args, &mut errors);
                    return result.to_string();
                }
            }
        }

        // Fall back to English bundle
        if let Some(fallback) = &self.fallback_bundle {
            if let Some(msg) = fallback.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let result = fallback.format_pattern(pattern, args, &mut errors);
                    return result.to_string();
                }
            }
        }

        // Return key if not found
        key.to_string()
    }

    /// Translate with count for pluralization
    #[allow(dead_code)]
    pub fn t_count(&self, key: &str, count: i64) -> String {
        let mut args = FluentArgs::new();
        args.set("count", count);
        self.t_with_args(key, Some(&args))
    }

    /// Translate with a numeric argument named "num"
    /// Used for default names like "Step 1", "Group 2", etc.
    pub fn t_num(&self, key: &str, num: i64) -> String {
        let mut args = FluentArgs::new();
        args.set("num", num);
        self.t_with_args(key, Some(&args))
    }

    /// Translate with a single string argument
    /// The argument is available in the FTL pattern under the given name.
    pub fn t_arg(&self, key: &str, arg_name: &str, arg_value: &str) -> String {
        let mut args = FluentArgs::new();
        args.set(arg_name, arg_value.to_string());
        self.t_with_args(key, Some(&args))
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_conversion() {
        assert_eq!(iso639_1_to_3("en"), "eng");
        assert_eq!(iso639_1_to_3("fr"), "fra");
        assert_eq!(iso639_1_to_3("de"), "deu");
        assert_eq!(iso639_1_to_3("eng"), "eng"); // Already ISO 639-3
        
        assert_eq!(iso639_3_to_1("eng"), "en");
        assert_eq!(iso639_3_to_1("fra"), "fr");
        assert_eq!(iso639_3_to_1("deu"), "de");
        assert_eq!(iso639_3_to_1("en"), "en"); // Already ISO 639-1
    }
    
    #[test]
    fn test_locale_display_name() {
        assert_eq!(locale_display_name("en"), "English");
        assert_eq!(locale_display_name("eng"), "English");
        assert_eq!(locale_display_name("fr"), "Français");
        assert_eq!(locale_display_name("fra"), "Français");
    }
}
