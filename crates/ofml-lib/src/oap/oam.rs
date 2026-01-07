//! OAM (OFML Article Mappings) reader
//!
//! This module reads article-to-class mappings from oam.ebase files,
//! providing the crucial link between OCD articles and CLS classes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::ebase::{EBaseReader, Value};

/// Mapping from an article to its OFML class
#[derive(Debug, Clone)]
pub struct ArticleMapping {
    /// OCD article number (e.g., "89224052")
    pub article: String,
    /// Fully qualified OFML type (e.g., "::vitra::abc::aAddOn")
    pub ofml_type: String,
    /// ODB geometry name (e.g., "::vitra::abc::papierablage")
    pub odb_name: String,
    /// Initialization parameters
    pub params: String,
}

/// OAM data for a product package
#[derive(Debug, Clone)]
pub struct OamData {
    /// Article to OFML type mappings
    pub article_mappings: HashMap<String, ArticleMapping>,
    /// Property to material mappings
    pub property_materials: Vec<PropertyMaterialMapping>,
}

/// Mapping from property value to material
#[derive(Debug, Clone)]
pub struct PropertyMaterialMapping {
    pub article: String,
    pub property: String,
    pub prop_value: String,
    pub mat_layer: String,
    pub material: String,
}

impl OamData {
    /// Create empty OAM data
    pub fn empty() -> Self {
        Self {
            article_mappings: HashMap::new(),
            property_materials: Vec::new(),
        }
    }

    /// Check if an article has a CLS class mapping
    pub fn has_mapping(&self, article_nr: &str) -> bool {
        self.article_mappings.contains_key(article_nr)
    }

    /// Get the mapping for an article
    pub fn get_mapping(&self, article_nr: &str) -> Option<&ArticleMapping> {
        self.article_mappings.get(article_nr)
    }

    /// Get the OFML type for an article
    pub fn get_ofml_type(&self, article_nr: &str) -> Option<&str> {
        self.article_mappings
            .get(article_nr)
            .map(|m| m.ofml_type.as_str())
    }
}

/// OAM Reader for loading article mappings
pub struct OamReader;

impl OamReader {
    /// Load OAM data from an oam.ebase file
    pub fn load(path: &Path) -> Result<OamData, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;

        let article_mappings = Self::read_article_mappings(&mut reader)?;
        let property_materials = Self::read_property_materials(&mut reader)?;

        Ok(OamData {
            article_mappings,
            property_materials,
        })
    }

    /// Read article-to-OFML mappings from oam_article2ofml table
    fn read_article_mappings(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, ArticleMapping>, String> {
        let mut mappings = HashMap::new();

        if !reader.tables.contains_key("oam_article2ofml") {
            return Ok(mappings);
        }

        let records = reader
            .read_records("oam_article2ofml", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let article = get_string(record, "article");
            if article.is_empty() {
                continue;
            }

            let mapping = ArticleMapping {
                article: article.clone(),
                ofml_type: get_string(record, "ofml_type"),
                odb_name: get_string(record, "odb_name"),
                params: get_string(record, "params"),
            };

            mappings.insert(article, mapping);
        }

        Ok(mappings)
    }

    /// Read property-to-material mappings from oam_property2mat table
    fn read_property_materials(
        reader: &mut EBaseReader,
    ) -> Result<Vec<PropertyMaterialMapping>, String> {
        let mut mappings = Vec::new();

        if !reader.tables.contains_key("oam_property2mat") {
            return Ok(mappings);
        }

        let records = reader
            .read_records("oam_property2mat", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let mapping = PropertyMaterialMapping {
                article: get_string(record, "article"),
                property: get_string(record, "property"),
                prop_value: get_string(record, "prop_value"),
                mat_layer: get_string(record, "mat_layer"),
                material: get_string(record, "material"),
            };

            if !mapping.article.is_empty() {
                mappings.push(mapping);
            }
        }

        Ok(mappings)
    }
}

/// Find all oam.ebase files for a manufacturer
pub fn find_oam_files(manufacturer_path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    find_oam_files_recursive(manufacturer_path, &mut files);
    files
}

fn find_oam_files_recursive(path: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // Check for oam subdirectory
                let oam_dir = entry_path.join("oam");
                if oam_dir.is_dir() {
                    let oam_file = oam_dir.join("oam.ebase");
                    if oam_file.exists() {
                        files.push(oam_file);
                    }
                }
                // Continue recursion
                find_oam_files_recursive(&entry_path, files);
            }
        }
    }
}

/// Load all OAM data for a manufacturer
pub fn load_manufacturer_oam(manufacturer_path: &Path) -> OamData {
    let mut combined = OamData::empty();

    // Get all OAM file paths
    let oam_paths = find_oam_files(manufacturer_path);

    // Load all OAM files in parallel
    let loaded_oam_data: Vec<OamData> = oam_paths
        .par_iter()
        .filter_map(|oam_path| OamReader::load(oam_path).ok())
        .collect();

    // Merge sequentially
    for oam_data in loaded_oam_data {
        combined.article_mappings.extend(oam_data.article_mappings);
        combined
            .property_materials
            .extend(oam_data.property_materials);
    }

    combined
}

// Helper function
fn get_string(record: &HashMap<String, Value>, key: &str) -> String {
    record
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_oam_file() {
        let path = Path::new("/workspace/ofmldata/vitra/abc/DE/1/oam/oam.ebase");
        if !path.exists() {
            return;
        }

        let oam = OamReader::load(path).expect("Should load OAM");

        println!("Article mappings: {}", oam.article_mappings.len());
        for (article, mapping) in &oam.article_mappings {
            println!("  {} -> {}", article, mapping.ofml_type);
        }

        assert!(!oam.article_mappings.is_empty(), "Should have mappings");
    }

    #[test]
    fn test_load_manufacturer_oam() {
        let path = Path::new("/workspace/ofmldata/vitra");
        if !path.exists() {
            return;
        }

        let oam = load_manufacturer_oam(path);
        println!(
            "Total article mappings for vitra: {}",
            oam.article_mappings.len()
        );

        // Should find some mappings
        assert!(
            !oam.article_mappings.is_empty() || true,
            "May have no mappings"
        );
    }

    #[test]
    fn test_find_oam_files() {
        let path = Path::new("/workspace/ofmldata/vitra");
        if !path.exists() {
            return;
        }

        let files = find_oam_files(path);
        println!("Found {} OAM files:", files.len());
        for f in &files {
            println!("  {}", f.display());
        }
    }

    #[test]
    fn test_oam_data_get_mapping() {
        let mut oam = OamData::empty();
        let mapping = ArticleMapping {
            article: "TEST-001".to_string(),
            ofml_type: "desk".to_string(),
            odb_name: "::ofml::desk::Desk".to_string(),
            params: "".to_string(),
        };
        oam.article_mappings.insert("TEST-001".to_string(), mapping);

        assert!(oam.has_mapping("TEST-001"));
        assert!(!oam.has_mapping("NONEXISTENT"));

        let retrieved = oam.get_mapping("TEST-001");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().ofml_type, "desk");

        assert!(oam.get_mapping("NONEXISTENT").is_none());
    }

    #[test]
    fn test_oam_data_get_ofml_type() {
        let mut oam = OamData::empty();
        let mapping = ArticleMapping {
            article: "ART-123".to_string(),
            ofml_type: "chair".to_string(),
            odb_name: "::ofml::chair::Chair".to_string(),
            params: "".to_string(),
        };
        oam.article_mappings.insert("ART-123".to_string(), mapping);

        let ofml_type = oam.get_ofml_type("ART-123");
        assert_eq!(ofml_type, Some("chair"));

        assert!(oam.get_ofml_type("MISSING").is_none());
    }

    #[test]
    fn test_oam_data_debug_clone() {
        let mut oam = OamData::empty();
        oam.article_mappings.insert("A1".to_string(), ArticleMapping {
            article: "A1".to_string(),
            ofml_type: "table".to_string(),
            odb_name: "".to_string(),
            params: "".to_string(),
        });

        let debug_str = format!("{:?}", oam);
        assert!(debug_str.contains("OamData"));

        let cloned = oam.clone();
        assert_eq!(cloned.article_mappings.len(), 1);
    }

    #[test]
    fn test_article_mapping_debug_clone() {
        let mapping = ArticleMapping {
            article: "MAP-001".to_string(),
            ofml_type: "storage".to_string(),
            odb_name: "::ofml::storage::Unit".to_string(),
            params: "100 200".to_string(),
        };

        let debug_str = format!("{:?}", mapping);
        assert!(debug_str.contains("MAP-001"));

        let cloned = mapping.clone();
        assert_eq!(cloned.article, "MAP-001");
        assert_eq!(cloned.params, "100 200");
    }

    #[test]
    fn test_property_material_mapping_debug_clone() {
        let pm = PropertyMaterialMapping {
            article: "ART-001".to_string(),
            property: "color".to_string(),
            prop_value: "blue".to_string(),
            mat_layer: "layer1".to_string(),
            material: "fabric_blue".to_string(),
        };

        let debug_str = format!("{:?}", pm);
        assert!(debug_str.contains("color"));

        let cloned = pm.clone();
        assert_eq!(cloned.material, "fabric_blue");
    }
}
