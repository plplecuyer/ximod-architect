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
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
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
        }

        // Development layout (running via `cargo run`).
        candidates.push(PathBuf::from("assets/data/Categories.json"));
        candidates.push(PathBuf::from("data/Categories.json"));
        candidates.push(PathBuf::from("Categories.json"));

        candidates.into_iter().find(|p| p.is_file())
    }

    /// `(game_id, official_name)` pairs, in file order, for the game dropdown.
    pub fn game_list(&self) -> Vec<(String, String)> {
        self.games
            .iter()
            .map(|(id, g)| (id.clone(), g.name.clone()))
            .collect()
    }

    /// Categories for a given game id (empty slice if unknown).
    pub fn categories_for(&self, game_id: &str) -> &[String] {
        self.games
            .get(game_id)
            .map(|g| g.categories.as_slice())
            .unwrap_or(&[])
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
