//! Configuration persistence for TUI
//!
//! This module handles saving and loading product configurations to/from JSON files.
//! It provides three main features:
//!
//! ## Saved Configurations
//! Save and load complete product configurations including all property selections,
//! variant codes, and pricing information. Stored in `~/.ofml-configs/`.
//!
//! ## Favorites System
//! Mark product families as favorites for quick access. Favorites are stored in
//! `~/.ofml-configs/favorites.json`.
//!
//! ## History System
//! Automatically track recently configured products with access counts.
//! Limited to 20 entries, stored in `~/.ofml-configs/history.json`.
//!
//! # Example
//! ```ignore
//! use ofml_interpreter::tui::config_store::*;
//!
//! // Add a product to favorites
//! add_favorite("sedus", "ai", "AI Drehstuhl")?;
//!
//! // Check if it's a favorite
//! assert!(is_favorite("sedus", "ai"));
//!
//! // Remove from favorites
//! remove_favorite("sedus", "ai")?;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// A saved product configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConfiguration {
    /// Manufacturer ID
    pub manufacturer: String,
    /// Series/Product family ID
    pub series: String,
    /// Article number
    pub article_nr: String,
    /// Property selections (key -> value)
    pub properties: HashMap<String, String>,
    /// Variant code
    pub variant_code: String,
    /// Description from catalog
    #[serde(default)]
    pub description: String,
    /// Price date used
    #[serde(default)]
    pub price_date: Option<String>,
    /// Saved timestamp
    pub saved_at: String,
}

/// Configuration store directory location
fn get_config_dir() -> PathBuf {
    // Use ~/.ofml-configs for storing configurations
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".ofml-configs")
}

/// Ensure config directory exists
fn ensure_config_dir() -> std::io::Result<PathBuf> {
    let dir = get_config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

/// Generate a filename for a configuration
fn config_filename(manufacturer: &str, article: &str, variant_code: &str) -> String {
    // Sanitize for filename
    let safe_article = article.replace([':', '/', '\\'], "_");
    let safe_variant = variant_code.replace([':', '/', '\\', '.'], "_");
    format!("{}_{}_{}_{}.json", manufacturer, safe_article, safe_variant,
            chrono::Local::now().format("%Y%m%d_%H%M%S"))
}

/// Save a configuration to a JSON file
pub fn save_configuration(config: &SavedConfiguration) -> Result<PathBuf, String> {
    let dir = ensure_config_dir().map_err(|e| format!("Failed to create config directory: {}", e))?;

    let filename = config_filename(&config.manufacturer, &config.article_nr, &config.variant_code);
    let path = dir.join(&filename);

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize configuration: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write configuration file: {}", e))?;

    Ok(path)
}

/// Load a configuration from a JSON file
pub fn load_configuration(path: &Path) -> Result<SavedConfiguration, String> {
    let json = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read configuration file: {}", e))?;

    serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse configuration: {}", e))
}

/// List all saved configurations
pub fn list_configurations() -> Result<Vec<(PathBuf, SavedConfiguration)>, String> {
    let dir = get_config_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut configs = Vec::new();

    let entries = fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read config directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(config) = load_configuration(&path) {
                configs.push((path, config));
            }
        }
    }

    // Sort by saved_at descending (newest first)
    configs.sort_by(|a, b| b.1.saved_at.cmp(&a.1.saved_at));

    Ok(configs)
}

/// Delete a saved configuration
pub fn delete_configuration(path: &Path) -> Result<(), String> {
    fs::remove_file(path)
        .map_err(|e| format!("Failed to delete configuration: {}", e))
}

// === Favorites System ===

/// A favorite product family for quick access
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Favorite {
    /// Manufacturer ID
    pub manufacturer: String,
    /// Series/Product family ID
    pub series: String,
    /// Human-readable name
    pub name: String,
    /// When this was added to favorites
    pub added_at: String,
}

/// Favorites storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FavoritesStore {
    pub favorites: Vec<Favorite>,
}

/// Get the favorites file path
fn get_favorites_path() -> PathBuf {
    get_config_dir().join("favorites.json")
}

/// Load favorites from disk
pub fn load_favorites() -> FavoritesStore {
    let path = get_favorites_path();
    if !path.exists() {
        return FavoritesStore::default();
    }

    fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

/// Save favorites to disk
pub fn save_favorites(store: &FavoritesStore) -> Result<(), String> {
    let _ = ensure_config_dir();
    let path = get_favorites_path();
    let json = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Failed to serialize favorites: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write favorites: {}", e))
}

/// Add a favorite
pub fn add_favorite(manufacturer: &str, series: &str, name: &str) -> Result<(), String> {
    let mut store = load_favorites();

    // Check if already exists
    if store
        .favorites
        .iter()
        .any(|f| f.manufacturer == manufacturer && f.series == series)
    {
        return Ok(()); // Already a favorite
    }

    store.favorites.push(Favorite {
        manufacturer: manufacturer.to_string(),
        series: series.to_string(),
        name: name.to_string(),
        added_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    });

    save_favorites(&store)
}

/// Remove a favorite
pub fn remove_favorite(manufacturer: &str, series: &str) -> Result<(), String> {
    let mut store = load_favorites();
    store
        .favorites
        .retain(|f| !(f.manufacturer == manufacturer && f.series == series));
    save_favorites(&store)
}

/// Check if a product family is a favorite
pub fn is_favorite(manufacturer: &str, series: &str) -> bool {
    let store = load_favorites();
    store
        .favorites
        .iter()
        .any(|f| f.manufacturer == manufacturer && f.series == series)
}

// === History System ===

/// Maximum number of history entries
const MAX_HISTORY_ENTRIES: usize = 20;

/// A history entry for a recently configured product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Manufacturer ID
    pub manufacturer: String,
    /// Series/Product family ID
    pub series: String,
    /// Article number
    pub article_nr: String,
    /// Human-readable name
    pub name: String,
    /// Last access timestamp
    pub last_accessed: String,
    /// Access count
    pub access_count: u32,
}

/// History storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryStore {
    pub entries: Vec<HistoryEntry>,
}

/// Get the history file path
fn get_history_path() -> PathBuf {
    get_config_dir().join("history.json")
}

/// Load history from disk
pub fn load_history() -> HistoryStore {
    let path = get_history_path();
    if !path.exists() {
        return HistoryStore::default();
    }

    fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

/// Save history to disk
fn save_history(store: &HistoryStore) -> Result<(), String> {
    let _ = ensure_config_dir();
    let path = get_history_path();
    let json = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Failed to serialize history: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write history: {}", e))
}

/// Add or update a history entry
pub fn add_to_history(
    manufacturer: &str,
    series: &str,
    article_nr: &str,
    name: &str,
) -> Result<(), String> {
    let mut store = load_history();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Check if entry already exists
    if let Some(entry) = store.entries.iter_mut().find(|e| {
        e.manufacturer == manufacturer && e.series == series && e.article_nr == article_nr
    }) {
        entry.last_accessed = now;
        entry.access_count += 1;
    } else {
        // Add new entry
        store.entries.push(HistoryEntry {
            manufacturer: manufacturer.to_string(),
            series: series.to_string(),
            article_nr: article_nr.to_string(),
            name: name.to_string(),
            last_accessed: now,
            access_count: 1,
        });
    }

    // Sort by last_accessed descending
    store
        .entries
        .sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));

    // Trim to max entries
    store.entries.truncate(MAX_HISTORY_ENTRIES);

    save_history(&store)
}

/// Get recent history entries
pub fn get_recent_history(limit: usize) -> Vec<HistoryEntry> {
    let store = load_history();
    store.entries.into_iter().take(limit).collect()
}

/// Clear history
pub fn clear_history() -> Result<(), String> {
    save_history(&HistoryStore::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_filename() {
        let filename = config_filename("sex", "AI:100A", "PG3_CSE166");
        assert!(filename.starts_with("sex_AI_100A_PG3_CSE166_"));
        assert!(filename.ends_with(".json"));
    }

    #[test]
    fn test_config_filename_special_chars() {
        // Test that special characters in article/variant are sanitized
        let filename = config_filename("mfr", "ART/123", "VAR:456.789");
        // Path separators should be converted to underscores
        assert!(!filename.contains('/'));
        assert!(!filename.contains('\\'));
        // The filename ends with .json, so we check the base name doesn't have colons/dots
        let base = filename.strip_suffix(".json").unwrap();
        // Colons in article and variant should be replaced with underscores
        assert!(!base[base.find("ART").unwrap()..].contains(':'));
        // Dots in variant should be replaced with underscores
        assert!(!base[base.find("VAR").unwrap()..].contains('.'));
        assert!(filename.ends_with(".json"));
    }

    #[test]
    fn test_saved_configuration_serialization() {
        let config = SavedConfiguration {
            manufacturer: "sedus".to_string(),
            series: "ai".to_string(),
            article_nr: "SE:AI-100".to_string(),
            properties: HashMap::from([
                ("PG".to_string(), "3".to_string()),
                ("CSE".to_string(), "166".to_string()),
            ]),
            variant_code: "PG=3;CSE=166".to_string(),
            description: "Drehstuhl".to_string(),
            price_date: Some("2024-01-15".to_string()),
            saved_at: "2024-01-15 10:30:00".to_string(),
        };

        let json = serde_json::to_string(&config).expect("Serialization should work");
        assert!(json.contains("sedus"));
        assert!(json.contains("ai"));
        assert!(json.contains("SE:AI-100"));

        let parsed: SavedConfiguration =
            serde_json::from_str(&json).expect("Deserialization should work");
        assert_eq!(parsed.manufacturer, "sedus");
        assert_eq!(parsed.series, "ai");
        assert_eq!(parsed.properties.len(), 2);
    }

    #[test]
    fn test_favorite_serialization() {
        let fav = Favorite {
            manufacturer: "vitra".to_string(),
            series: "abc".to_string(),
            name: "Add System".to_string(),
            added_at: "2024-01-15 10:00:00".to_string(),
        };

        let json = serde_json::to_string(&fav).expect("Serialization should work");
        let parsed: Favorite = serde_json::from_str(&json).expect("Deserialization should work");
        assert_eq!(parsed.manufacturer, "vitra");
        assert_eq!(parsed.series, "abc");
        assert_eq!(parsed.name, "Add System");
    }

    #[test]
    fn test_favorites_store_serialization() {
        let store = FavoritesStore {
            favorites: vec![
                Favorite {
                    manufacturer: "sedus".to_string(),
                    series: "ai".to_string(),
                    name: "AI Chair".to_string(),
                    added_at: "2024-01-01".to_string(),
                },
                Favorite {
                    manufacturer: "vitra".to_string(),
                    series: "allstar".to_string(),
                    name: "Allstar Chair".to_string(),
                    added_at: "2024-01-02".to_string(),
                },
            ],
        };

        let json = serde_json::to_string(&store).expect("Serialization should work");
        let parsed: FavoritesStore =
            serde_json::from_str(&json).expect("Deserialization should work");
        assert_eq!(parsed.favorites.len(), 2);
    }

    #[test]
    fn test_history_entry_serialization() {
        let entry = HistoryEntry {
            manufacturer: "sedus".to_string(),
            series: "ai".to_string(),
            article_nr: "SE:AI-100".to_string(),
            name: "AI Chair".to_string(),
            last_accessed: "2024-01-15 10:30:00".to_string(),
            access_count: 5,
        };

        let json = serde_json::to_string(&entry).expect("Serialization should work");
        let parsed: HistoryEntry =
            serde_json::from_str(&json).expect("Deserialization should work");
        assert_eq!(parsed.manufacturer, "sedus");
        assert_eq!(parsed.access_count, 5);
    }

    #[test]
    fn test_history_store_serialization() {
        let store = HistoryStore {
            entries: vec![
                HistoryEntry {
                    manufacturer: "sedus".to_string(),
                    series: "ai".to_string(),
                    article_nr: "SE:AI-100".to_string(),
                    name: "AI Chair".to_string(),
                    last_accessed: "2024-01-15 10:30:00".to_string(),
                    access_count: 3,
                },
                HistoryEntry {
                    manufacturer: "vitra".to_string(),
                    series: "abc".to_string(),
                    article_nr: "VIT:ABC-200".to_string(),
                    name: "Add System".to_string(),
                    last_accessed: "2024-01-14 09:00:00".to_string(),
                    access_count: 1,
                },
            ],
        };

        let json = serde_json::to_string(&store).expect("Serialization should work");
        let parsed: HistoryStore =
            serde_json::from_str(&json).expect("Deserialization should work");
        assert_eq!(parsed.entries.len(), 2);
    }

    #[test]
    fn test_favorite_equality() {
        let fav1 = Favorite {
            manufacturer: "sedus".to_string(),
            series: "ai".to_string(),
            name: "AI Chair".to_string(),
            added_at: "2024-01-01".to_string(),
        };
        let fav2 = Favorite {
            manufacturer: "sedus".to_string(),
            series: "ai".to_string(),
            name: "AI Chair".to_string(),
            added_at: "2024-01-01".to_string(),
        };
        let fav3 = Favorite {
            manufacturer: "vitra".to_string(),
            series: "ai".to_string(),
            name: "AI Chair".to_string(),
            added_at: "2024-01-01".to_string(),
        };

        // Identical favorites are equal
        assert_eq!(fav1, fav2);
        // Different manufacturer means not equal
        assert_ne!(fav1, fav3);
    }

    #[test]
    fn test_max_history_entries_constant() {
        // Verify the constant exists and has a reasonable value
        assert!(MAX_HISTORY_ENTRIES > 0);
        assert!(MAX_HISTORY_ENTRIES <= 100);
    }
}
