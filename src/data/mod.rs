//! Reference data loaded at runtime from external JSON files.
//!
//! - `Countries.json` (assets/data/) : one entry per country (ISO 3166-1 codes, names, flag,
//!   font, and the languages spoken there). Feeds the "Properties" window
//!   (country → languages → flag).
//! - `Languages.json` (assets/data/) : the *canonical* language table (one entry per ISO 639-3
//!   code, with its ISO 639-1 code, display/endonym name and font). Meant to
//!   replace the hard-coded `LANGUAGE_MAP` used by the i18n module.
//!
//! Both files are loaded from `assets/data/` next to the executable (with dev
//! fallbacks) and never embedded in the binary, so they can be edited or
//! regenerated without recompiling. Missing/invalid files degrade gracefully
//! to an empty dataset instead of panicking.
//!
//! Nothing here is wired into the UI yet — this is the loading layer for the
//! upcoming country/language management feature.
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Locate a data file (e.g. `"Countries.json"`), trying several locations.
/// Primary location is `assets/data/`; the old `data/` layout is kept as a
/// fallback.
pub fn find_data_file(filename: &str) -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("assets").join("data").join(filename));
            candidates.push(dir.join("data").join(filename));
            candidates.push(dir.join(filename));
            // macOS .app bundle: Contents/Resources/…
            if let Some(up) = dir.parent() {
                let res = up.join("Resources");
                candidates.push(res.join("assets").join("data").join(filename));
                candidates.push(res.join("data").join(filename));
            }
        }
    }
    // Development layout (running via `cargo run`).
    candidates.push(PathBuf::from("assets").join("data").join(filename));
    candidates.push(PathBuf::from("data").join(filename));
    candidates.push(PathBuf::from(filename));

    candidates.into_iter().find(|p| p.is_file())
}

/// Locate the flags directory (`assets/images/svg`), trying the production
/// layout (next to the executable, macOS bundle) then the development one.
pub fn flags_dir() -> Option<PathBuf> {
    let rel = PathBuf::from("assets").join("images").join("svg");
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

/// Generic JSON loader: never panics, returns `T::default()` on any error.
fn load_json<T>(filename: &str) -> T
where
    T: for<'de> Deserialize<'de> + Default,
{
    let Some(path) = find_data_file(filename) else {
        tracing::warn!("{} not found in any known location", filename);
        return T::default();
    };
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            // Tolerate a leading UTF-8 BOM (EF BB BF): editors such as Notepad++
            // may save "UTF-8 with BOM", which serde_json otherwise rejects.
            let content = content.strip_prefix('\u{feff}').unwrap_or(&content);
            match serde_json::from_str::<T>(content) {
                Ok(data) => {
                    tracing::info!("Loaded {:?}", path);
                    data
                }
                Err(e) => {
                    tracing::error!("Failed to parse {:?}: {}", path, e);
                    T::default()
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to read {:?}: {}", path, e);
            T::default()
        }
    }
}

// ============================================================================
// Languages (canonical) — data/Languages.json
// ============================================================================

/// One canonical language: a single entry per ISO 639-3 code.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageEntry {
    /// ISO 639-3 code (e.g. "eng"). Also the name of the `assets/locales/<code>/` folder.
    #[serde(rename = "iso639_3")]
    pub iso3: String,
    /// ISO 639-1 code (e.g. "en"); empty when the language has no 2-letter code.
    #[serde(rename = "iso639_1", default)]
    pub iso1: String,
    /// Display name / canonical endonym (e.g. "English", "Deutsch").
    pub name: String,
    /// Font covering this language's script: path relative to `assets/fonts/`
    /// (e.g. "Noto_Sans_Thai/static/NotoSansThai-Regular.ttf").
    #[serde(default)]
    pub font: String,
    /// Countries where the language is spoken (ISO 3166-1 alpha-3), for reverse
    /// lookup language → countries.
    #[serde(default)]
    pub countries: Vec<String>,
}

/// The canonical language table (replacement for `LANGUAGE_MAP`).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LanguagesData {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub languages: Vec<LanguageEntry>,
    /// iso639-3 → index (built after load, not part of the JSON).
    #[serde(skip)]
    index_by_iso3: HashMap<String, usize>,
    /// iso639-1 → index (built after load, not part of the JSON).
    #[serde(skip)]
    index_by_iso1: HashMap<String, usize>,
}

impl LanguagesData {
    /// Load `assets/data/Languages.json` and build the lookup indexes.
    pub fn load() -> Self {
        let mut data: LanguagesData = load_json("Languages.json");
        data.rebuild_index();
        data
    }

    fn rebuild_index(&mut self) {
        self.index_by_iso3.clear();
        self.index_by_iso1.clear();
        for (i, lang) in self.languages.iter().enumerate() {
            self.index_by_iso3.insert(lang.iso3.clone(), i);
            if !lang.iso1.is_empty() {
                self.index_by_iso1.insert(lang.iso1.clone(), i);
            }
        }
    }

    fn get_iso3(&self, iso3: &str) -> Option<&LanguageEntry> {
        self.index_by_iso3.get(iso3).map(|&i| &self.languages[i])
    }

    /// True when the language is part of the ISO 639-3 reference table.
    pub fn contains(&self, iso3: &str) -> bool {
        self.index_by_iso3.contains_key(iso3)
    }

    /// Display name / endonym for an ISO 639-3 code.
    pub fn display_name(&self, iso3: &str) -> Option<&str> {
        self.get_iso3(iso3).map(|l| l.name.as_str())
    }

    /// Font covering the script of an ISO 639-3 code, if declared.
    pub fn font_for(&self, iso3: &str) -> Option<&str> {
        self.get_iso3(iso3)
            .map(|l| l.font.as_str())
            .filter(|s| !s.is_empty())
    }

    /// ISO 639-3 → ISO 639-1 (None if the language has no 2-letter code).
    pub fn iso3_to_iso1(&self, iso3: &str) -> Option<&str> {
        self.get_iso3(iso3)
            .map(|l| l.iso1.as_str())
            .filter(|s| !s.is_empty())
    }

    /// ISO 639-1 → ISO 639-3.
    pub fn iso1_to_iso3(&self, iso1: &str) -> Option<&str> {
        self.index_by_iso1
            .get(iso1)
            .map(|&i| self.languages[i].iso3.as_str())
    }

    /// Countries (ISO 3166-1 alpha-3) where a language is spoken.
    /// Reverse lookup, useful when offering a language to translate.
    pub fn countries_for(&self, iso3: &str) -> &[String] {
        self.get_iso3(iso3)
            .map(|l| l.countries.as_slice())
            .unwrap_or(&[])
    }

    pub fn is_empty(&self) -> bool {
        self.languages.is_empty()
    }
}

// ============================================================================
// Country → spoken languages — assets/data/CountryLanguages.json
// ============================================================================

/// All languages *spoken* in each country (ISO 3166-1 alpha-3 → ISO 639-3
/// codes). Distinct from `CountriesData`, which lists only official languages.
///
/// Feeds the language drop-down of the settings window once a country has been
/// picked from the flag selector.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CountryLanguagesData {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub countries: HashMap<String, Vec<String>>,
}

impl CountryLanguagesData {
    /// Load `assets/data/CountryLanguages.json`.
    pub fn load() -> Self {
        load_json("CountryLanguages.json")
    }

    /// Languages spoken in a country (empty slice if unknown).
    pub fn languages_for(&self, a3: &str) -> &[String] {
        self.countries
            .get(&a3.to_uppercase())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn is_empty(&self) -> bool {
        self.countries.is_empty()
    }
}

// ============================================================================
// Countries — data/Countries.json
// ============================================================================

/// One official language of a country: its code plus the country's endonym in
/// that language. The source database's parallel lists (CISO639-3 / NEP) are
/// zipped into objects so they can never drift out of sync.
///
/// The language's own endonym and font are *not* duplicated here — they live in
/// `Languages.json`, the single source of truth for languages.
#[derive(Debug, Clone, Deserialize)]
pub struct CountryLanguage {
    /// ISO 639-3 code of the language (from CISO639-3).
    #[serde(rename = "iso639_3")]
    pub iso3: String,
    /// Endonym of the country in this language (from NEP), e.g.
    /// "Republiek van Suid-Afrika" for Afrikaans.
    #[serde(rename = "countryEndonym", default)]
    pub country_endonym: String,
}

/// One country entry.
#[derive(Debug, Clone, Deserialize)]
pub struct CountryEntry {
    /// ISO 3166-1 alpha-3 (CA3), e.g. "ZAF".
    pub a3: String,
    /// English name (NAP).
    #[serde(rename = "nameEn", default)]
    pub name_en: String,
    /// French name (NFP).
    #[serde(rename = "nameFr", default)]
    pub name_fr: String,
    /// Flag image file name in SVG (Flags), e.g. "ZAF.svg".
    #[serde(default)]
    pub flag: String,
    /// Official languages of the country (CISO639-3 / NEP zipped together).
    #[serde(default)]
    pub languages: Vec<CountryLanguage>,
}

/// The countries dataset.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CountriesData {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub countries: Vec<CountryEntry>,
}

impl CountriesData {
    /// Load `assets/data/Countries.json`.
    pub fn load() -> Self {
        load_json("Countries.json")
    }

    /// `(alpha-3, French name)` pairs, sorted by French name, for the country
    /// dropdown in the Properties window.
    pub fn country_list(&self) -> Vec<(String, String)> {
        let mut list: Vec<(String, String)> = self
            .countries
            .iter()
            .map(|c| (c.a3.clone(), c.name_fr.clone()))
            .collect();
        list.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
        list
    }

    /// Country by ISO 3166-1 alpha-3 code.
    pub fn by_a3(&self, a3: &str) -> Option<&CountryEntry> {
        self.countries.iter().find(|c| c.a3 == a3)
    }

    /// Languages spoken in a country (empty slice if unknown).
    pub fn languages_for(&self, a3: &str) -> &[CountryLanguage] {
        self.by_a3(a3).map(|c| c.languages.as_slice()).unwrap_or(&[])
    }

    /// Flag file name for a country (e.g. "ZAF.svg").
    pub fn flag_for(&self, a3: &str) -> Option<&str> {
        self.by_a3(a3)
            .map(|c| c.flag.as_str())
            .filter(|s| !s.is_empty())
    }

    /// The country's endonym in a given language, e.g. ("CHE", "ita") →
    /// "Confederazione Svizzera". Falls back to the French then English name
    /// when that language has no recorded endonym.
    pub fn endonym_for(&self, a3: &str, iso3: &str) -> Option<&str> {
        let c = self.by_a3(a3)?;
        let exact = c
            .languages
            .iter()
            .find(|l| l.iso3 == iso3)
            .map(|l| l.country_endonym.as_str())
            .filter(|s| !s.is_empty());
        exact
            .or(Some(c.name_fr.as_str()).filter(|s| !s.is_empty()))
            .or(Some(c.name_en.as_str()).filter(|s| !s.is_empty()))
    }

    /// Exact recorded country endonym for (country, language), with NO fallback
    /// to the French/English country name. `None` when the pair has no endonym.
    pub fn endonym_exact(&self, a3: &str, iso3: &str) -> Option<&str> {
        self.by_a3(a3)?
            .languages
            .iter()
            .find(|l| l.iso3 == iso3)
            .map(|l| l.country_endonym.as_str())
            .filter(|s| !s.is_empty())
    }

    pub fn is_empty(&self) -> bool {
        self.countries.is_empty()
    }
}
