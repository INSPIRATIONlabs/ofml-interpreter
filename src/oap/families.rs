//! Product Family Loader - Groups articles into configurable product families
//!
//! Instead of showing hundreds of individual article SKUs, this module groups
//! them into product families that users can configure with options.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::oam::{load_manufacturer_oam, OamData};
use super::ocd::{load_article_property_classes, load_articles_with_descriptions, OcdArticle};
use super::ocd_properties::{load_manufacturer_properties, OcdPropertyReader, OcdPropertyValue};

/// A product family - a group of related articles with configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFamily {
    /// Unique identifier (typically base article or series code)
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Series identifier
    pub series: String,
    /// Base article number (for price lookup)
    pub base_article_nr: String,
    /// Property class for configuration
    pub prop_class: Option<String>,
    /// Number of variants/articles in this family
    pub variant_count: usize,
    /// Whether this family has configuration options
    pub is_configurable: bool,
    /// All article numbers in this family
    pub article_nrs: Vec<String>,
    /// Article descriptions (parallel to article_nrs)
    pub article_descriptions: Vec<String>,
}

/// A configurable property for a product family
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyProperty {
    /// Property key/identifier
    pub key: String,
    /// Display label
    pub label: String,
    /// Property group (for UI sections)
    pub group: String,
    /// Property type
    pub prop_type: PropertyType,
    /// Whether this property is required
    pub required: bool,
    /// Available options
    pub options: Vec<PropertyOption>,
    /// Default value
    pub default_value: Option<String>,
    /// Display order
    pub position: u16,
}

/// Type of property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    /// Selection from a list
    Choice,
    /// Numeric range
    Range { min: f64, max: f64, step: f64 },
    /// Integer value
    Integer { min: i64, max: i64 },
    /// Boolean toggle
    Boolean,
    /// Free text
    Text,
}

/// An option for a choice property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyOption {
    /// Value code
    pub value: String,
    /// Display label
    pub label: String,
    /// Whether this is the default
    pub is_default: bool,
}

/// Product Family Loader - loads and manages product families
pub struct FamilyLoader {
    /// Loaded product families
    pub families: Vec<ProductFamily>,
    /// OCD property data
    pub properties: OcdPropertyReader,
    /// OAM data for configurability check
    pub oam: OamData,
    /// Language for text lookups
    language: String,
}

impl FamilyLoader {
    /// Load product families for a manufacturer
    pub fn load(manufacturer_path: &Path, language: &str) -> Self {
        // Load property data
        let properties = load_manufacturer_properties(manufacturer_path);

        // Load OAM data
        let oam = load_manufacturer_oam(manufacturer_path);

        // Load and group articles
        let families = Self::group_articles_into_families(
            manufacturer_path,
            &properties,
            &oam,
            language,
        );

        Self {
            families,
            properties,
            oam,
            language: language.to_string(),
        }
    }

    /// Group articles into product families
    fn group_articles_into_families(
        manufacturer_path: &Path,
        properties: &OcdPropertyReader,
        oam: &OamData,
        language: &str,
    ) -> Vec<ProductFamily> {
        let articles_with_desc = load_articles_with_descriptions(manufacturer_path, language);

        // Load article-to-property-class mappings from ocd_propertyclass tables
        let article_prop_class_map = load_article_property_classes(manufacturer_path);

        // Filter out internal and invalid articles
        let articles: Vec<_> = articles_with_desc
            .into_iter()
            .filter(|(art, _)| {
                // Skip internal articles
                if art.article_nr.starts_with('@') {
                    return false;
                }
                // Skip articles with control characters in series
                if art.series.chars().any(|c| c.is_control()) {
                    return false;
                }
                // Skip articles with non-ASCII series (encoding issues)
                if !art.series.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
                    return false;
                }
                // Skip empty series
                if art.series.is_empty() {
                    return false;
                }
                true
            })
            .collect();

        // Group by series
        let mut series_groups: HashMap<String, Vec<(OcdArticle, String)>> = HashMap::new();
        for (article, desc) in articles {
            series_groups
                .entry(article.series.clone())
                .or_default()
                .push((article, desc));
        }

        // Convert groups to families
        let mut families: Vec<ProductFamily> = series_groups
            .into_iter()
            .filter(|(series, articles)| !series.is_empty() && !articles.is_empty())
            .map(|(series, articles)| {
                // Find best description - prefer one that doesn't look like an article number
                let best_desc = articles
                    .iter()
                    .map(|(art, desc)| {
                        // Score the description - higher is better
                        let score = if desc.is_empty() || desc == &art.article_nr {
                            0
                        } else if desc.chars().all(|c| c.is_ascii_digit()) {
                            1 // Just numbers - probably article number
                        } else if desc.len() < 5 {
                            2 // Too short
                        } else {
                            10 + desc.len().min(50) // Longer descriptions are better
                        };
                        (art, desc, score)
                    })
                    .max_by_key(|(_, _, score)| *score)
                    .map(|(_, d, _)| d.clone());

                let name = best_desc
                    .as_ref()
                    .filter(|d| !d.chars().all(|c| c.is_ascii_digit()))
                    .map(|d| Self::extract_family_name(d))
                    .unwrap_or_else(|| format!("Serie {}", series));

                let description = best_desc.unwrap_or_default();

                // Check if any article is configurable
                let is_configurable = articles
                    .iter()
                    .any(|(art, _)| oam.has_mapping(&art.article_nr));

                // Get property class using article-to-property-class mapping from ocd_propertyclass table
                // This is the primary and correct way to find property classes for articles
                let prop_class = articles.iter()
                    .find_map(|(art, _)| {
                        article_prop_class_map.get(&art.article_nr).and_then(|classes| {
                            // Find first property class that has actual properties defined
                            classes.iter()
                                .filter(|pc| !properties.get_properties_for_class(pc).is_empty())
                                .next()
                                .cloned()
                        })
                    })
                    // Fallback to old matching strategies if no mapping found
                    .or_else(|| {
                        let all_prop_classes = properties.get_property_classes();
                        all_prop_classes.iter()
                            .filter(|pc| !properties.get_properties_for_class(pc).is_empty())
                            .find(|pc| pc.to_uppercase() == series.to_uppercase())
                            .map(|s| s.to_string())
                    });

                // Use first article as base
                let base_article_nr = articles
                    .iter()
                    .min_by_key(|(art, _)| &art.article_nr)
                    .map(|(art, _)| art.article_nr.clone())
                    .unwrap_or_default();

                // Sort articles by article number for consistent ordering
                let mut sorted_articles = articles.clone();
                sorted_articles.sort_by(|(a, _), (b, _)| a.article_nr.cmp(&b.article_nr));

                let article_nrs: Vec<_> = sorted_articles
                    .iter()
                    .map(|(art, _)| art.article_nr.clone())
                    .collect();

                let article_descriptions: Vec<_> = sorted_articles
                    .iter()
                    .map(|(_, desc)| desc.clone())
                    .collect();

                ProductFamily {
                    id: series.clone(),
                    name,
                    description,
                    series: series.clone(),
                    base_article_nr,
                    prop_class,
                    variant_count: articles.len(),
                    is_configurable,
                    article_nrs,
                    article_descriptions,
                }
            })
            .collect();

        // Sort by name
        families.sort_by(|a, b| a.name.cmp(&b.name));

        families
    }

    /// Extract a clean family name from description
    fn extract_family_name(description: &str) -> String {
        // Only split at comma or semicolon, NOT at dash (which is used in names like "se:air")
        let name = description
            .split(|c| c == ',' || c == ';')
            .next()
            .unwrap_or(description)
            .trim();

        // Handle UTF-8 properly - count characters, not bytes
        let char_count = name.chars().count();
        if char_count > 50 {
            let truncated: String = name.chars().take(47).collect();
            format!("{}...", truncated)
        } else {
            name.to_string()
        }
    }

    /// Get all product families
    pub fn get_families(&self) -> &[ProductFamily] {
        &self.families
    }

    /// Get a family by ID
    pub fn get_family(&self, id: &str) -> Option<&ProductFamily> {
        self.families.iter().find(|f| f.id == id)
    }

    /// Get configurable properties for a family
    pub fn get_properties_for_family(&self, family: &ProductFamily) -> Vec<FamilyProperty> {
        let mut result = Vec::new();

        // Try to get properties from OCD property class
        if let Some(ref prop_class) = family.prop_class {
            let props = self.properties.get_properties_for_class(prop_class);

            for prop in props {
                let label = self
                    .properties
                    .get_property_label(&prop.prop_class, &prop.property, &self.language)
                    .unwrap_or_else(|| prop.property.clone());

                let values = self
                    .properties
                    .get_values_for_property(&prop.prop_class, &prop.property);

                let options: Vec<PropertyOption> = values
                    .iter()
                    .map(|v| {
                        let val_label = self
                            .properties
                            .get_value_label(v, &self.language)
                            .unwrap_or_else(|| v.value_from.clone());
                        PropertyOption {
                            value: v.value_from.clone(),
                            label: val_label,
                            is_default: v.is_default,
                        }
                    })
                    .collect();

                let default_value = self
                    .properties
                    .get_default_value(&prop.prop_class, &prop.property)
                    .map(|v| v.value_from.clone());

                let prop_type = Self::parse_property_type(&prop.prop_type, &values);

                result.push(FamilyProperty {
                    key: prop.property.clone(),
                    label,
                    group: prop.prop_class.clone(),
                    prop_type,
                    required: prop.need_input,
                    options,
                    default_value,
                    position: prop.position,
                });
            }
        }

        // Sort by position
        result.sort_by_key(|p| p.position);

        // Filter out properties with no options (they can't be configured)
        result.retain(|p| !p.options.is_empty());

        result
    }

    /// Parse OCD property type string into enum
    fn parse_property_type(
        type_str: &str,
        values: &[&OcdPropertyValue],
    ) -> PropertyType {
        match type_str.to_uppercase().as_str() {
            "B" | "BOOL" | "BOOLEAN" => PropertyType::Boolean,
            "I" | "INT" | "INTEGER" => {
                // Try to get min/max from values
                let min = values
                    .iter()
                    .filter_map(|v| v.value_from.parse::<i64>().ok())
                    .min()
                    .unwrap_or(0);
                let max = values
                    .iter()
                    .filter_map(|v| {
                        if v.value_to.is_empty() {
                            v.value_from.parse::<i64>().ok()
                        } else {
                            v.value_to.parse::<i64>().ok()
                        }
                    })
                    .max()
                    .unwrap_or(100);
                PropertyType::Integer { min, max }
            }
            "R" | "RANGE" | "FLOAT" => {
                let min = values
                    .iter()
                    .filter_map(|v| v.value_from.parse::<f64>().ok())
                    .fold(f64::INFINITY, f64::min);
                let max = values
                    .iter()
                    .filter_map(|v| {
                        if v.value_to.is_empty() {
                            v.value_from.parse::<f64>().ok()
                        } else {
                            v.value_to.parse::<f64>().ok()
                        }
                    })
                    .fold(f64::NEG_INFINITY, f64::max);
                let step = values
                    .first()
                    .and_then(|v| v.raster.parse::<f64>().ok())
                    .unwrap_or(1.0);
                PropertyType::Range {
                    min: if min.is_finite() { min } else { 0.0 },
                    max: if max.is_finite() { max } else { 100.0 },
                    step,
                }
            }
            "T" | "TEXT" | "STRING" => PropertyType::Text,
            _ => {
                // Default to Choice if we have multiple values
                PropertyType::Choice
            }
        }
    }

    /// Check if a family has any configurable properties
    pub fn family_has_properties(&self, family: &ProductFamily) -> bool {
        if let Some(ref prop_class) = family.prop_class {
            !self.properties.get_properties_for_class(prop_class).is_empty()
        } else {
            false
        }
    }

    /// Get statistics about loaded data
    pub fn stats(&self) -> FamilyLoaderStats {
        let configurable = self.families.iter().filter(|f| f.is_configurable).count();
        let with_properties = self
            .families
            .iter()
            .filter(|f| self.family_has_properties(f))
            .count();
        let total_variants: usize = self.families.iter().map(|f| f.variant_count).sum();

        FamilyLoaderStats {
            total_families: self.families.len(),
            configurable_families: configurable,
            families_with_properties: with_properties,
            total_variants,
        }
    }
}

/// Statistics about loaded families
#[derive(Debug, Clone)]
pub struct FamilyLoaderStats {
    pub total_families: usize,
    pub configurable_families: usize,
    pub families_with_properties: usize,
    pub total_variants: usize,
}

/// A user's configuration selections for a product family
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyConfiguration {
    /// The product family ID
    pub family_id: String,
    /// Selected property values
    pub selections: HashMap<String, String>,
    /// Generated variant code
    pub variant_code: String,
    /// Calculated article number
    pub article_nr: Option<String>,
}

impl FamilyConfiguration {
    /// Create a new configuration with default values
    pub fn new(family_id: &str, properties: &[FamilyProperty]) -> Self {
        let mut selections = HashMap::new();

        for prop in properties {
            if let Some(ref default) = prop.default_value {
                selections.insert(prop.key.clone(), default.clone());
            } else if !prop.options.is_empty() {
                // Use first option as default
                selections.insert(prop.key.clone(), prop.options[0].value.clone());
            }
        }

        let variant_code = Self::generate_variant_code(&selections);

        Self {
            family_id: family_id.to_string(),
            selections,
            variant_code,
            article_nr: None,
        }
    }

    /// Set a property value
    pub fn set(&mut self, key: &str, value: &str) {
        self.selections.insert(key.to_string(), value.to_string());
        self.variant_code = Self::generate_variant_code(&self.selections);
    }

    /// Get a property value
    pub fn get(&self, key: &str) -> Option<&str> {
        self.selections.get(key).map(|s| s.as_str())
    }

    /// Generate variant code from selections
    fn generate_variant_code(selections: &HashMap<String, String>) -> String {
        let mut parts: Vec<_> = selections
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        parts.sort();
        parts.join(";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_sex_families() {
        let path = Path::new("/workspace/ofmldata/sex");
        if !path.exists() {
            return;
        }

        let loader = FamilyLoader::load(path, "DE");
        let stats = loader.stats();

        println!("Families: {}", stats.total_families);
        println!("With properties: {}", stats.families_with_properties);

        // Find MMZ or similar
        println!("\nFamilies with MM in name:");
        for family in loader.get_families() {
            if family.id.to_uppercase().contains("MM") || family.name.to_uppercase().contains("MM") {
                println!("  {} - {} ({} variants, prop_class={:?})",
                    family.id, family.name, family.variant_count, family.prop_class);

                let props = loader.get_properties_for_family(family);
                println!("    Properties: {}", props.len());
                for prop in props.iter().take(3) {
                    println!("      - {}: {} options", prop.key, prop.options.len());
                }
            }
        }

        // Show some property classes
        println!("\nSample property classes:");
        for pc in loader.properties.get_property_classes().iter().take(15) {
            println!("  {}", pc);
        }
    }

    #[test]
    fn test_load_vitra_families() {
        let path = Path::new("/workspace/ofmldata/vitra");
        if !path.exists() {
            return;
        }

        let loader = FamilyLoader::load(path, "DE");
        let stats = loader.stats();

        println!("Families: {}", stats.total_families);
        println!("Configurable: {}", stats.configurable_families);
        println!("With properties: {}", stats.families_with_properties);
        println!("Total variants: {}", stats.total_variants);

        // Print some families
        println!("\nSample families:");
        for family in loader.get_families().iter().take(10) {
            println!(
                "  {} - {} ({} variants, config={})",
                family.id, family.name, family.variant_count, family.is_configurable
            );

            // Show properties if available
            let props = loader.get_properties_for_family(family);
            if !props.is_empty() {
                println!("    Properties:");
                for prop in props.iter().take(3) {
                    println!(
                        "      {} ({}): {} options",
                        prop.key,
                        prop.label,
                        prop.options.len()
                    );
                }
            }
        }
    }

    #[test]
    fn test_family_configuration() {
        let path = Path::new("/workspace/ofmldata/vitra");
        if !path.exists() {
            return;
        }

        let loader = FamilyLoader::load(path, "DE");

        // Find a family with properties
        if let Some(family) = loader
            .get_families()
            .iter()
            .find(|f| loader.family_has_properties(f))
        {
            println!("Testing family: {} - {}", family.id, family.name);

            let props = loader.get_properties_for_family(family);
            println!("Properties: {}", props.len());

            // Create configuration
            let mut config = FamilyConfiguration::new(&family.id, &props);
            println!("Initial variant code: {}", config.variant_code);

            // Change a property if available
            if !props.is_empty() {
                let prop = &props[0];
                if prop.options.len() > 1 {
                    config.set(&prop.key, &prop.options[1].value);
                    println!("After change: {}", config.variant_code);
                }
            }
        }
    }
}
