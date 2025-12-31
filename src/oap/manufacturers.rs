//! Manufacturer discovery from OFML data directory
//!
//! This module discovers installed manufacturers from the ofmldata directory.
//! Manufacturers are identified by their directory name.

use std::path::{Path, PathBuf};

use rusqlite::Connection;

/// Installed manufacturer with path
#[derive(Debug, Clone)]
pub struct InstalledManufacturer {
    /// Directory name (e.g., "vitra")
    pub id: String,
    /// Display name (same as id, capitalized)
    pub name: String,
    /// Path to manufacturer directory
    pub path: PathBuf,
}

/// Initialize manufacturer names (no-op, kept for API compatibility)
#[allow(unused_variables)]
pub fn init_from_data_path(_data_path: &Path) {
    // No-op - manufacturers are discovered from filesystem
}

/// Get list of installed manufacturers from install.db
/// Falls back to directory scanning if install.db doesn't exist
pub fn get_installed_manufacturers(data_path: &Path) -> Vec<InstalledManufacturer> {
    let db_path = data_path.join("install.db");

    let mut result = if db_path.exists() {
        get_manufacturers_from_db(&db_path, data_path)
    } else {
        get_manufacturers_from_filesystem(data_path)
    };

    // Sort alphabetically by id
    result.sort_by(|a, b| a.id.cmp(&b.id));
    result
}

/// Get manufacturers from install.db SQLite database
fn get_manufacturers_from_db(db_path: &Path, data_path: &Path) -> Vec<InstalledManufacturer> {
    let mut result = Vec::new();

    if let Ok(conn) = Connection::open(db_path) {
        let query = "SELECT name FROM install WHERE name LIKE 'manufacturer:%' ORDER BY name";

        if let Ok(mut stmt) = conn.prepare(query) {
            if let Ok(rows) = stmt.query_map([], |row| {
                let name: String = row.get(0)?;
                Ok(name)
            }) {
                for row in rows.flatten() {
                    // Parse 'manufacturer:vitra' -> 'vitra'
                    if let Some(id) = row.strip_prefix("manufacturer:") {
                        let mfr_path = data_path.join(id);
                        if mfr_path.exists() {
                            result.push(InstalledManufacturer {
                                id: id.to_string(),
                                name: id.to_string(),
                                path: mfr_path,
                            });
                        }
                    }
                }
            }
        }
    }

    result
}

/// Get manufacturers by scanning the filesystem
fn get_manufacturers_from_filesystem(data_path: &Path) -> Vec<InstalledManufacturer> {
    let mut result = Vec::new();

    if let Ok(entries) = std::fs::read_dir(data_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip hidden directories and known non-manufacturer directories
                    if name.starts_with('.')
                        || name.starts_with("pCon")
                        || name == "AddFiles"
                        || name == "addfiles"
                        || name.contains("plugin")
                        || name.contains("setup")
                        || name.ends_with(".exe")
                    {
                        continue;
                    }

                    // Check if it looks like a manufacturer directory
                    // (has subdirectories with product data)
                    let has_products = path.join("basics").exists()
                        || path.join("global").exists()
                        || has_product_subdirs(&path);

                    if has_products {
                        result.push(InstalledManufacturer {
                            id: name.to_string(),
                            name: name.to_string(),
                            path: path.clone(),
                        });
                    }
                }
            }
        }
    }

    result
}

/// Check if a directory has product subdirectories (directories with version folders)
fn has_product_subdirs(path: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let subpath = entry.path();
            if subpath.is_dir() {
                // Check for version directories (1, 2, 3, etc.)
                if subpath.join("1").exists() || subpath.join("current").exists() {
                    return true;
                }
            }
        }
    }
    false
}

/// Get display name for a directory name (just returns the name as-is)
pub fn get_display_name(dir_name: &str) -> String {
    dir_name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_display_name() {
        assert_eq!(get_display_name("vitra"), "vitra");
        assert_eq!(get_display_name("unknown"), "unknown");
    }

    #[test]
    fn test_get_installed_manufacturers_with_ofmldata() {
        let path = Path::new("/workspace/ofmldata");
        if path.exists() {
            let manufacturers = get_installed_manufacturers(path);
            // Should find some manufacturers
            assert!(!manufacturers.is_empty(), "Should find manufacturers in ofmldata");

            // Each manufacturer should have valid paths
            for mfr in &manufacturers {
                assert!(mfr.path.exists(), "Manufacturer path should exist: {:?}", mfr.path);
                assert!(!mfr.id.is_empty(), "Manufacturer id should not be empty");
            }
        }
    }
}
