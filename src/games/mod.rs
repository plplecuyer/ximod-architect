//! Dynamic game & category data.
//!
//! Games and their Nexus category lists are loaded at runtime from an external
//! `Categories.json` file (next to the executable), so new games or categories
//! can be added without recompiling. The file is never embedded in the binary.

use indexmap::IndexMap;
use serde::Deserialize;
use std::path::PathBuf;

/// A single game entry as described in `Categories.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct GameEntry {
    /// Official game name (e.g. "The Elder Scrolls V: Skyrim Special Edition").
    pub name: String,
    /// Slug used in the Nexus Mods URL (e.g. "skyrimspecialedition").
    #[serde(rename = "nexusSlug", default)]
    pub nexus_slug: String,
    /// Category names available for this game on Nexus.
    #[serde(default)]
    pub categories: Vec<String>,
    /// Plugins every installation of the game provides (the main master and
    /// the official DLC / Creation Club files): a plugin depending on them
    /// needs no file dependency for them.
    #[serde(default)]
    pub masters: Vec<String>,
    /// Record limit of a light plugin for this game (`2048` or `4096`); the
    /// conservative 2048 applies when absent.
    #[serde(rename = "eslLimit", default)]
    pub esl_limit: Option<u32>,
}

/// The whole games dataset. `games` preserves the JSON file order thanks to
/// `IndexMap`, so the dropdown lists games in the intended order.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GamesData {
    #[serde(default)]
    #[allow(dead_code)]
    pub version: String,
    #[serde(default)]
    pub games: IndexMap<String, GameEntry>,
}

impl GamesData {
    /// Load the dataset from disk. Returns an empty dataset (never panics) if
    /// the file is missing or malformed, so the app keeps working.
    pub fn load() -> Self {
        let Some(path) = Self::data_path() else {
            tracing::warn!("Categories.json not found in any known location");
            return Self::default();
        };

        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<GamesData>(&content) {
                Ok(data) => {
                    tracing::info!("Loaded {} games from {:?}", data.games.len(), path);
                    data
                }
                Err(e) => {
                    tracing::error!("Failed to parse {:?}: {}", path, e);
                    Self::default()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read {:?}: {}", path, e);
                Self::default()
            }
        }
    }

    /// Locate `Categories.json`, trying several locations for robustness.
    /// Primary location is `assets/data/`; the old `data/` layout is kept as a
    /// fallback.
    fn data_path() -> Option<PathBuf> {
        let mut candidates: Vec<PathBuf> = Vec::new();

        // Next to the executable (production layout).
        if let Ok(exe) = std::env::current_exe()
            && let Some(dir) = exe.parent()
        {
            candidates.push(dir.join("assets").join("data").join("Categories.json"));
            candidates.push(dir.join("data").join("Categories.json"));
            candidates.push(dir.join("Categories.json"));
            // macOS .app bundle: Contents/Resources/…
            if let Some(macos_dir) = dir.parent() {
                let res = macos_dir.join("Resources");
                candidates.push(res.join("assets").join("data").join("Categories.json"));
                candidates.push(res.join("data").join("Categories.json"));
            }
        }

        // Development layout (running via `cargo run`).
        candidates.push(PathBuf::from("assets/data/Categories.json"));
        candidates.push(PathBuf::from("data/Categories.json"));
        candidates.push(PathBuf::from("Categories.json"));

        candidates.into_iter().find(|p| p.is_file())
    }

    /// `(game_id, official_name)` pairs, in file order, for the game dropdown.
    pub fn game_list(&self) -> Vec<(String, String)> {
        self.games.iter().map(|(id, g)| (id.clone(), g.name.clone())).collect()
    }

    /// Categories for a given game id (empty slice if unknown).
    pub fn categories_for(&self, game_id: &str) -> &[String] {
        self.games.get(game_id).map(|g| g.categories.as_slice()).unwrap_or(&[])
    }

    /// Plugins the game itself provides (empty slice if unknown).
    pub fn base_masters_for(&self, game_id: &str) -> &[String] {
        self.games.get(game_id).map(|g| g.masters.as_slice()).unwrap_or(&[])
    }

    /// Whether `name` (a plugin file name, any case) is a base master of the game.
    #[cfg(test)]
    pub fn is_base_master(&self, game_id: &str, name: &str) -> bool {
        self.base_masters_for(game_id)
            .iter()
            .any(|m| m.eq_ignore_ascii_case(name))
    }

    /// Light-plugin record limit of a game (2048 unless the dataset says
    /// otherwise, e.g. 4096 for Starfield).
    pub fn esl_limit_for(&self, game_id: &str) -> u32 {
        self.games
            .get(game_id)
            .and_then(|g| g.esl_limit)
            .unwrap_or(crate::models::plugin_header::ESL_LIMIT_CLASSIC)
    }

    /// Official name for a game id, if present.
    pub fn name_for(&self, game_id: &str) -> Option<&str> {
        self.games.get(game_id).map(|g| g.name.as_str())
    }

    /// Nexus slug for a game id, if present.
    #[allow(dead_code)]
    pub fn nexus_slug_for(&self, game_id: &str) -> Option<&str> {
        self.games.get(game_id).map(|g| g.nexus_slug.as_str())
    }

    /// Whether any games were loaded.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.games.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masters_and_esl_limit_parse_with_defaults() {
        let json = r#"{
            "version": "1",
            "games": {
                "skyrimSpecialEdition": {"name": "SSE", "masters": ["Skyrim.esm", "Update.esm"]},
                "starfield": {"name": "SF", "masters": ["Starfield.esm"], "eslLimit": 4096},
                "morrowind": {"name": "MW"}
            }
        }"#;
        let data: GamesData = serde_json::from_str(json).unwrap();
        assert_eq!(data.base_masters_for("skyrimSpecialEdition").len(), 2);
        assert!(data.is_base_master("skyrimSpecialEdition", "update.ESM"));
        assert!(!data.is_base_master("skyrimSpecialEdition", "Dawnguard.esm"));
        assert!(data.base_masters_for("morrowind").is_empty());
        assert!(data.base_masters_for("unknown").is_empty());
        assert_eq!(data.esl_limit_for("starfield"), 4096);
        assert_eq!(data.esl_limit_for("skyrimSpecialEdition"), 2048);
        assert_eq!(data.esl_limit_for("unknown"), 2048);
    }

    /// The shipped dataset carries the base masters of the main games.
    #[test]
    fn shipped_dataset_has_base_masters() {
        let content = std::fs::read_to_string("assets/data/Categories.json").unwrap();
        let data: GamesData = serde_json::from_str(&content).unwrap();
        assert!(data.is_base_master("skyrimSpecialEdition", "Dragonborn.esm"));
        assert!(data.is_base_master("skyrimSpecialEdition", "_ResourcePack.esl"));
        assert!(data.is_base_master("skyrim", "HearthFires.esm"));
        assert!(!data.is_base_master("skyrim", "_ResourcePack.esl"));
        assert!(data.is_base_master("fallout4", "DLCNukaWorld.esm"));
        assert!(data.is_base_master("falloutNewVegas", "TribalPack.esm"));
        assert!(data.is_base_master("fallout3", "Zeta.esm"));
        assert!(data.is_base_master("oblivion", "Oblivion.esm"));
        assert!(data.is_base_master("morrowind", "Bloodmoon.esm"));
        assert!(data.is_base_master("starfield", "SFBGS008.esm"));
        assert_eq!(data.esl_limit_for("starfield"), 4096);
        assert_eq!(data.esl_limit_for("fallout4"), 2048);
    }
}
