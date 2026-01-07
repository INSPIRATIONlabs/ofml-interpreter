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
    use std::fs;

    #[test]
    fn test_get_display_name() {
        assert_eq!(get_display_name("vitra"), "vitra");
        assert_eq!(get_display_name("unknown"), "unknown");
        assert_eq!(get_display_name("sedus"), "sedus");
        assert_eq!(get_display_name(""), "");
    }

    #[test]
    fn test_init_from_data_path_is_noop() {
        // Should not panic
        init_from_data_path(Path::new("/nonexistent"));
        init_from_data_path(Path::new("/tmp"));
    }

    #[test]
    fn test_installed_manufacturer_struct() {
        let mfr = InstalledManufacturer {
            id: "test".to_string(),
            name: "Test Manufacturer".to_string(),
            path: PathBuf::from("/tmp/test"),
        };
        assert_eq!(mfr.id, "test");
        assert_eq!(mfr.name, "Test Manufacturer");
        assert_eq!(mfr.path, PathBuf::from("/tmp/test"));

        // Test clone
        let mfr2 = mfr.clone();
        assert_eq!(mfr2.id, mfr.id);

        // Test debug
        let debug_str = format!("{:?}", mfr);
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_get_installed_manufacturers_empty_path() {
        let temp_dir = std::env::temp_dir().join("ofml_test_empty");
        let _ = fs::create_dir_all(&temp_dir);

        let manufacturers = get_installed_manufacturers(&temp_dir);
        // Empty directory should return empty list
        assert!(manufacturers.is_empty());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_get_installed_manufacturers_nonexistent() {
        let manufacturers = get_installed_manufacturers(Path::new("/nonexistent/path"));
        assert!(manufacturers.is_empty());
    }

    #[test]
    fn test_get_installed_manufacturers_with_reference_ofmldata() {
        let path = Path::new("/reference/ofmldata");
        if path.exists() {
            let manufacturers = get_installed_manufacturers(path);
            // Should find many manufacturers
            assert!(
                manufacturers.len() > 50,
                "Should find 50+ manufacturers, found {}",
                manufacturers.len()
            );

            // Results should be sorted
            let mut sorted = manufacturers.clone();
            sorted.sort_by(|a, b| a.id.cmp(&b.id));
            for (i, mfr) in manufacturers.iter().enumerate() {
                assert_eq!(mfr.id, sorted[i].id, "Should be sorted alphabetically");
            }

            // Each manufacturer should have valid paths
            for mfr in &manufacturers {
                assert!(
                    mfr.path.exists(),
                    "Manufacturer path should exist: {:?}",
                    mfr.path
                );
                assert!(!mfr.id.is_empty(), "Manufacturer id should not be empty");
            }

            // Should find known manufacturers
            let ids: Vec<&str> = manufacturers.iter().map(|m| m.id.as_str()).collect();
            assert!(ids.contains(&"sex"), "Should find 'sex' (Sedus)");
        }
    }

    #[test]
    fn test_get_manufacturers_from_filesystem_filters_hidden() {
        let temp_dir = std::env::temp_dir().join("ofml_test_hidden");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Create hidden directory (should be filtered)
        fs::create_dir_all(temp_dir.join(".hidden/basics")).unwrap();

        // Create pCon directory (should be filtered)
        fs::create_dir_all(temp_dir.join("pConSomething/basics")).unwrap();

        // Create AddFiles directory (should be filtered)
        fs::create_dir_all(temp_dir.join("AddFiles/basics")).unwrap();

        // Create plugin directory (should be filtered)
        fs::create_dir_all(temp_dir.join("someplugin/basics")).unwrap();

        // Create valid manufacturer
        fs::create_dir_all(temp_dir.join("validmfr/basics")).unwrap();

        let manufacturers = get_manufacturers_from_filesystem(&temp_dir);

        // Should only find validmfr
        assert_eq!(manufacturers.len(), 1);
        assert_eq!(manufacturers[0].id, "validmfr");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_has_product_subdirs() {
        let temp_dir = std::env::temp_dir().join("ofml_test_subdirs");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Empty dir - no product subdirs
        assert!(!has_product_subdirs(&temp_dir));

        // Create subdir without version
        fs::create_dir_all(temp_dir.join("series1")).unwrap();
        assert!(!has_product_subdirs(&temp_dir));

        // Create version directory
        fs::create_dir_all(temp_dir.join("series1/1")).unwrap();
        assert!(has_product_subdirs(&temp_dir));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_has_product_subdirs_with_current() {
        let temp_dir = std::env::temp_dir().join("ofml_test_current");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Create subdir with 'current' version
        fs::create_dir_all(temp_dir.join("series1/current")).unwrap();
        assert!(has_product_subdirs(&temp_dir));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_get_manufacturers_from_db_nonexistent() {
        let result = get_manufacturers_from_db(
            Path::new("/nonexistent/install.db"),
            Path::new("/nonexistent"),
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_manufacturers_from_db_with_reference() {
        let db_path = Path::new("/reference/ofmldata/install.db");
        let data_path = Path::new("/reference/ofmldata");
        if db_path.exists() {
            let manufacturers = get_manufacturers_from_db(db_path, data_path);
            // Should find manufacturers from the database
            assert!(!manufacturers.is_empty(), "Should find manufacturers in install.db");
        }
    }

    #[test]
    fn test_get_manufacturers_from_filesystem_with_product_subdirs() {
        let temp_dir = std::env::temp_dir().join("ofml_test_product_subdirs");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Create manufacturer without basics/global but with product subdirs
        // This exercises the has_product_subdirs fallback (lines 99-100)
        fs::create_dir_all(temp_dir.join("mfrwithseries/series1/1")).unwrap();

        let manufacturers = get_manufacturers_from_filesystem(&temp_dir);

        // Should find the manufacturer via has_product_subdirs fallback
        assert_eq!(manufacturers.len(), 1);
        assert_eq!(manufacturers[0].id, "mfrwithseries");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_get_manufacturers_from_filesystem_with_global() {
        let temp_dir = std::env::temp_dir().join("ofml_test_global");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Create manufacturer with global directory
        fs::create_dir_all(temp_dir.join("mfrwithglobal/global")).unwrap();

        let manufacturers = get_manufacturers_from_filesystem(&temp_dir);

        // Should find the manufacturer via global dir check
        assert_eq!(manufacturers.len(), 1);
        assert_eq!(manufacturers[0].id, "mfrwithglobal");

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
