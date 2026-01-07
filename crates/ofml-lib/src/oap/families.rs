//! Product Family Loader - Groups articles into configurable product families
//!
//! Instead of showing hundreds of individual article SKUs, this module groups
//! them into product families that users can configure with options.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::oam::{load_manufacturer_oam, OamData};
use super::ocd::{
    load_article_property_classes, load_articles_with_full_descriptions, ArticleWithDescriptions,
};
use super::ocd_properties::{load_manufacturer_properties, OcdPropertyReader, OcdPropertyValue};

/// A product family - a group of related articles with configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFamily {
    /// Unique identifier (typically base article or series code)
    pub id: String,
    /// Display name
    pub name: String,
    /// Short description (from ocd_artshorttext)
    pub description: String,
    /// Long description (from ocd_artlongtext)
    pub long_description: String,
    /// Series identifier
    pub series: String,
    /// Base article number (for price lookup)
    pub base_article_nr: String,
    /// Property classes for configuration (articles can belong to multiple classes)
    pub prop_classes: Vec<String>,
    /// Number of variants/articles in this family
    pub variant_count: usize,
    /// Whether this family has configuration options
    pub is_configurable: bool,
    /// All article numbers in this family
    pub article_nrs: Vec<String>,
    /// Article short descriptions (parallel to article_nrs)
    pub article_descriptions: Vec<String>,
    /// Article long descriptions (parallel to article_nrs)
    pub article_long_descriptions: Vec<String>,
}

/// A configurable property for a product family
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyProperty {
    /// Property key/identifier
    pub key: String,
    /// Display label
    pub label: String,
    /// Property group (for UI sections) - internal key
    pub group: String,
    /// Human-readable group label (from ocd_propgrouptext or ocd_propclasstext)
    pub group_label: String,
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
    /// Hint text for this property (tooltip/help)
    pub hint: Option<String>,
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
        // Load property data and OAM data in parallel
        let (properties, oam) = rayon::join(
            || load_manufacturer_properties(manufacturer_path),
            || load_manufacturer_oam(manufacturer_path),
        );

        // Load and group articles
        let families =
            Self::group_articles_into_families(manufacturer_path, &properties, &oam, language);

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
        // Load articles and property class mappings in parallel
        let (articles_with_desc, article_prop_class_map) = rayon::join(
            || load_articles_with_full_descriptions(manufacturer_path, language),
            || load_article_property_classes(manufacturer_path),
        );

        // Filter out internal and invalid articles, repairing swapped fields where possible
        let articles: Vec<_> = articles_with_desc
            .into_iter()
            .filter_map(|mut awd| {
                let art = &mut awd.article;

                // Skip internal articles
                if art.article_nr.starts_with('@') {
                    return None;
                }

                // Detect and repair swapped article_nr and series fields
                // Pattern: article_nr is short alpha (like "VI") and series is long numeric (like "41210001")
                let nr_looks_like_series = art.article_nr.len() <= 4
                    && !art.article_nr.is_empty()
                    && art.article_nr.chars().all(|c| c.is_ascii_alphabetic());
                let series_looks_like_article = art.series.len() >= 6
                    && art.series.chars().all(|c| c.is_ascii_digit());

                if nr_looks_like_series && series_looks_like_article {
                    // Swap them back
                    std::mem::swap(&mut art.article_nr, &mut art.series);
                }

                // Skip articles with control characters in series (binary corruption)
                if art.series.chars().any(|c| c.is_control()) {
                    return None;
                }

                // Skip articles with non-ASCII series (encoding issues)
                if !art
                    .series
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
                {
                    return None;
                }

                // Skip empty series
                if art.series.is_empty() {
                    return None;
                }

                Some(awd)
            })
            .collect();

        // Group by series
        let mut series_groups: HashMap<String, Vec<ArticleWithDescriptions>> = HashMap::new();
        for awd in articles {
            series_groups
                .entry(awd.article.series.clone())
                .or_default()
                .push(awd);
        }

        // Convert groups to families
        let mut families: Vec<ProductFamily> = series_groups
            .into_iter()
            .filter(|(series, articles)| !series.is_empty() && !articles.is_empty())
            .map(|(series, articles)| {
                // Find best description - prefer one that doesn't look like an article number
                let best_article = articles
                    .iter()
                    .map(|awd| {
                        // Score the description - higher is better
                        let desc = &awd.short_description;
                        let score = if desc.is_empty() || desc == &awd.article.article_nr {
                            0
                        } else if desc.chars().all(|c| c.is_ascii_digit()) {
                            1 // Just numbers - probably article number
                        } else if desc.len() < 5 {
                            2 // Too short
                        } else {
                            10 + desc.len().min(50) // Longer descriptions are better
                        };
                        (awd, score)
                    })
                    .max_by_key(|(_, score)| *score)
                    .map(|(awd, _)| awd);

                let name = best_article
                    .as_ref()
                    .filter(|awd| !awd.short_description.chars().all(|c| c.is_ascii_digit()))
                    .map(|awd| Self::extract_family_name(&awd.short_description))
                    .unwrap_or_else(|| format!("Serie {}", series));

                let description = best_article
                    .map(|a| a.short_description.clone())
                    .unwrap_or_default();
                let long_description = best_article
                    .map(|a| a.long_description.clone())
                    .unwrap_or_default();

                // Check if any article is configurable
                let is_configurable = articles
                    .iter()
                    .any(|awd| oam.has_mapping(&awd.article.article_nr));

                // Get ALL property classes from article-to-property-class mapping
                // Articles can belong to multiple property classes (e.g., CHAT BOARD articles)
                let mut prop_classes: Vec<String> = Vec::new();
                let mut seen_classes = std::collections::HashSet::new();

                for awd in &articles {
                    if let Some(classes) = article_prop_class_map.get(&awd.article.article_nr) {
                        for pc in classes {
                            // Only add classes that have properties defined and haven't been seen
                            // Note: insert() returns true if value was not present
                            if !properties.get_properties_for_class(pc).is_empty()
                                && seen_classes.insert(pc.clone())
                            {
                                prop_classes.push(pc.clone());
                            }
                        }
                    }
                }

                // Fallback to old matching strategies if no mapping found
                if prop_classes.is_empty() {
                    let all_prop_classes = properties.get_property_classes();
                    if let Some(pc) = all_prop_classes
                        .iter()
                        .filter(|pc| !properties.get_properties_for_class(pc).is_empty())
                        .find(|pc| pc.to_uppercase() == series.to_uppercase())
                    {
                        prop_classes.push(pc.to_string());
                    }
                }

                // Use first article as base
                let base_article_nr = articles
                    .iter()
                    .min_by_key(|awd| &awd.article.article_nr)
                    .map(|awd| awd.article.article_nr.clone())
                    .unwrap_or_default();

                // Collect article data and sort by article number
                // Avoid cloning the entire EnrichedArticle vector
                let mut article_data: Vec<_> = articles
                    .iter()
                    .map(|awd| {
                        (
                            awd.article.article_nr.clone(),
                            awd.short_description.clone(),
                            awd.long_description.clone(),
                        )
                    })
                    .collect();
                article_data.sort_by(|a, b| a.0.cmp(&b.0));

                let (article_nrs, article_descriptions, article_long_descriptions): (
                    Vec<_>,
                    Vec<_>,
                    Vec<_>,
                ) = article_data.into_iter().fold(
                    (Vec::new(), Vec::new(), Vec::new()),
                    |(mut nrs, mut descs, mut long_descs), (nr, desc, long_desc)| {
                        nrs.push(nr);
                        descs.push(desc);
                        long_descs.push(long_desc);
                        (nrs, descs, long_descs)
                    },
                );

                ProductFamily {
                    id: series.clone(),
                    name,
                    description,
                    long_description,
                    series: series.clone(),
                    base_article_nr,
                    prop_classes,
                    variant_count: articles.len(),
                    is_configurable,
                    article_nrs,
                    article_descriptions,
                    article_long_descriptions,
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
            .split([',', ';'])
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
        self.get_properties_for_family_with_selections(family, &HashMap::new())
    }

    /// Get configurable properties for a family with current selections
    /// (for TABLE relations that depend on other property values)
    pub fn get_properties_for_family_with_selections(
        &self,
        family: &ProductFamily,
        current_selections: &HashMap<String, String>,
    ) -> Vec<FamilyProperty> {
        let mut result = Vec::new();
        let mut seen_property_keys = std::collections::HashSet::new();

        // Get properties from ALL property classes (articles can belong to multiple)
        for prop_class in &family.prop_classes {
            // Get all properties for this class without series filtering
            // The OCD data model uses ocd_propertyclass to map articles to property classes,
            // NOT series to property classes. Properties with no applicable values are
            // filtered out downstream, so showing all properties is safe.
            //
            // Note: Series-based filtering was removed because:
            // - Framery uses shared property classes across all series (MG_PROPERTIES)
            // - When properties are merged during loading, they get tagged with whichever
            //   series was loaded first, causing incorrect filtering
            // - The property value availability already controls which options are shown
            let props = self.properties.get_properties_for_class(prop_class);

            // First, collect TABLE relation values for cross-property filtering
            // Properties with TABLE relations can restrict values of other properties
            let table_restriction_values =
                self.collect_table_restriction_values(prop_class, &props, current_selections);

            for prop in props {
                // Skip properties we've already added from another class
                if seen_property_keys.contains(&prop.property) {
                    continue;
                }
                // Filter properties by scope:
                // - "C" (Choice): Configurable by user - SHOW
                // - "RV" (Read-only Visible): Display only - SHOW
                // - "" (empty): Default visible - SHOW
                // - "R" (Result): Internal/computed - HIDE
                // - "RG" (Range/Graphics): Internal - HIDE
                // - Custom scopes (e.g., AIX "Desk_2020"): Assume visible - SHOW
                let scope_upper = prop.scope.to_uppercase();
                if scope_upper == "R" || scope_upper == "RG" {
                    // Skip internal/computed properties
                    continue;
                }

                let label = self
                    .properties
                    .get_property_label(&prop.prop_class, &prop.property, &self.language)
                    .unwrap_or_else(|| prop.property.clone());

                // First try standard property values
                let mut values: Vec<&OcdPropertyValue> = self
                    .properties
                    .get_values_for_property(&prop.prop_class, &prop.property);

                // If no standard values and property uses TABLE relation, get values from table
                let table_values: Vec<OcdPropertyValue>;
                if values.is_empty() && prop.rel_obj > 0 {
                    table_values = self.properties.get_table_values(
                        &prop.prop_class,
                        &prop.property,
                        current_selections,
                    );
                    values = table_values.iter().collect();
                }

                // If still no values, try to get values from TABLE restrictions
                // This handles properties like Farbe_Rahmen that get values from TABLE columns
                let table_column_values: Vec<OcdPropertyValue>;
                if values.is_empty() {
                    let property_key = prop.property.to_lowercase();
                    if let Some(allowed_values) = table_restriction_values.get(&property_key) {
                        table_column_values = allowed_values
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                use super::ocd_properties::OcdPropertyValue;
                                OcdPropertyValue {
                                    prop_class: prop.prop_class.clone(),
                                    property: prop.property.clone(),
                                    position: i as u16 + 1,
                                    textnr: String::new(),
                                    is_default: i == 0,
                                    value_from: v.clone(),
                                    value_to: String::new(),
                                    op_from: String::new(),
                                    op_to: String::new(),
                                    raster: String::new(),
                                }
                            })
                            .collect();
                        values = table_column_values.iter().collect();
                    }
                }

                // Apply TABLE-based restrictions from other properties
                // This filters values based on what's valid according to TABLE lookups
                let property_key = prop.property.to_lowercase();
                let restricted_values: Option<&std::collections::HashSet<String>> =
                    table_restriction_values.get(&property_key);

                // Deduplicate options by value (some data sources have duplicate entries)
                let mut seen_values = std::collections::HashSet::new();
                let options: Vec<PropertyOption> = values
                    .iter()
                    .filter(|v| {
                        // If there are restrictions for this property, filter by them
                        if let Some(allowed) = restricted_values {
                            allowed.contains(&v.value_from.to_uppercase())
                        } else {
                            true
                        }
                    })
                    .filter(|v| {
                        // Skip duplicate values
                        seen_values.insert(v.value_from.to_uppercase())
                    })
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

                let default_value = if !options.is_empty() {
                    options
                        .iter()
                        .find(|o| o.is_default)
                        .or(options.first())
                        .map(|o| o.value.clone())
                } else {
                    None
                };

                let prop_type = Self::parse_property_type(&prop.prop_type, &values);

                // Get human-readable group label
                // Try prop_group_texts first (more specific), then prop_class_texts as fallback
                let group_label = self
                    .properties
                    .get_prop_group_label(&prop.prop_class, &self.language)
                    .or_else(|| {
                        self.properties
                            .get_class_label(&prop.prop_class, &self.language)
                    })
                    .unwrap_or_else(|| prop.prop_class.clone());

                // Try to get hint text for this property using its textnr
                let hint = self.properties.get_hint_text(&prop.textnr, &self.language);

                result.push(FamilyProperty {
                    key: prop.property.clone(),
                    label,
                    group: prop.prop_class.clone(),
                    group_label,
                    prop_type,
                    required: prop.need_input,
                    options,
                    default_value,
                    position: prop.position,
                    hint,
                });

                // Track that we've added this property
                seen_property_keys.insert(prop.property.clone());
            }
        }

        // Sort by position
        result.sort_by_key(|p| p.position);

        // Filter out properties with no options (they can't be configured)
        result.retain(|p| !p.options.is_empty());

        result
    }

    /// Collect values from TABLE relations that can restrict other property values
    /// This enables cross-property filtering where TABLE on one property restricts another
    fn collect_table_restriction_values(
        &self,
        _prop_class: &str,
        props: &[&super::ocd_properties::OcdPropertyDef],
        current_selections: &HashMap<String, String>,
    ) -> HashMap<String, std::collections::HashSet<String>> {
        use super::ocd_properties::OcdPropertyReader;

        let mut restrictions: HashMap<String, std::collections::HashSet<String>> = HashMap::new();

        // Find all properties with TABLE relations
        for prop in props {
            if prop.rel_obj == 0 {
                continue;
            }

            // Get the relation object
            let rel_obj = match self.properties.relation_objs.get(&prop.rel_obj) {
                Some(obj) => obj,
                None => continue,
            };

            // Get the relations
            let relations = match self.properties.relations.get(&rel_obj.rel_name) {
                Some(rels) => rels,
                None => continue,
            };

            // Concatenate relation blocks
            let rel_text: String = relations
                .iter()
                .map(|r| r.rel_block.clone())
                .collect::<Vec<_>>()
                .join(" ");

            // Parse TABLE relation
            if let Some(table_rel) = OcdPropertyReader::parse_table_relation(&rel_text) {
                let table_name = table_rel.table_name.to_lowercase() + "_tbl";

                // Get the custom table
                let table_data = match self.properties.custom_tables.get(&table_name) {
                    Some(data) => data,
                    None => continue,
                };

                // Build filter conditions from current selections
                let mut filters: Vec<(String, String)> = Vec::new();
                for (col, val) in &table_rel.column_mappings {
                    if !val.contains("$SELF") {
                        // This column references another property
                        if let Some(selected_value) = current_selections.get(val) {
                            filters.push((col.to_lowercase(), selected_value.clone()));
                        }
                    }
                }

                // For each column mapping that references a property, collect valid values
                for (col, val) in &table_rel.column_mappings {
                    if !val.contains("$SELF") {
                        // This column represents a property - collect all valid values
                        let prop_name = val.to_lowercase();
                        let col_lower = col.to_lowercase();

                        // Query table with current filters (except this column)
                        let other_filters: Vec<_> = filters
                            .iter()
                            .filter(|(c, _)| c != &col_lower)
                            .cloned()
                            .collect();

                        let mut valid_values: std::collections::HashSet<String> =
                            std::collections::HashSet::new();

                        for row in table_data {
                            // Check if row matches other filters
                            let matches = other_filters.is_empty()
                                || other_filters.iter().all(|(c, expected)| {
                                    row.get(c)
                                        .map(|v| v.eq_ignore_ascii_case(expected))
                                        .unwrap_or(false)
                                });

                            if matches {
                                if let Some(value) = row.get(&col_lower) {
                                    if !value.is_empty() {
                                        valid_values.insert(value.to_uppercase());
                                    }
                                }
                            }
                        }

                        if !valid_values.is_empty() {
                            restrictions
                                .entry(prop_name)
                                .or_default()
                                .extend(valid_values);
                        }
                    }
                }
            }
        }

        restrictions
    }

    /// Parse OCD property type string into enum
    fn parse_property_type(type_str: &str, values: &[&OcdPropertyValue]) -> PropertyType {
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
        family.prop_classes.iter().any(|prop_class| {
            !self
                .properties
                .get_properties_for_class(prop_class)
                .is_empty()
        })
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

    /// Export configuration to JSON-serializable format
    pub fn export(
        &self,
        manufacturer_id: &str,
        family: &ProductFamily,
        price: Option<&super::PriceResult>,
    ) -> super::ExportData {
        use super::{format_german_price, ExportData, ExportSurcharge};

        let now = chrono::Utc::now();

        // Convert selections to JSON values with labels
        let properties: std::collections::HashMap<String, serde_json::Value> = self
            .selections
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
            .collect();

        ExportData {
            manufacturer: manufacturer_id.to_string(),
            article: family.id.clone(),
            article_number: Some(family.base_article_nr.clone()),
            variant_code: self.variant_code.clone(),
            properties,
            base_price: price.map(|p| format_german_price(p.base_price)),
            surcharges: price.map(|p| {
                p.surcharges
                    .iter()
                    .map(|s| ExportSurcharge {
                        name: s.name.clone(),
                        amount: if s.is_percentage {
                            format!("{}%", s.amount)
                        } else {
                            format_german_price(s.amount)
                        },
                        is_percentage: s.is_percentage,
                    })
                    .collect()
            }),
            total_price: price.map(|p| format_german_price(p.total_price)),
            currency: price.map(|p| p.currency.clone()),
            price_date: price.map(|p| p.price_date.format("%Y-%m-%d").to_string()),
            sub_articles: Vec::new(),
            exported_at: now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        }
    }

    /// Export configuration to JSON string
    pub fn export_json(
        &self,
        manufacturer_id: &str,
        family: &ProductFamily,
        price: Option<&super::PriceResult>,
    ) -> Result<String, serde_json::Error> {
        let export_data = self.export(manufacturer_id, family, price);
        serde_json::to_string_pretty(&export_data)
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
            if family.id.to_uppercase().contains("MM") || family.name.to_uppercase().contains("MM")
            {
                println!(
                    "  {} - {} ({} variants, prop_classes={:?})",
                    family.id, family.name, family.variant_count, family.prop_classes
                );

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
    fn test_load_bisley_families() {
        let path = Path::new("/reference/ofmldata/bisley");
        if !path.exists() {
            return;
        }

        let loader = FamilyLoader::load(path, "DE");
        let stats = loader.stats();

        println!("\n=== Bisley Manufacturer Data ===");
        println!("Total Families: {}", stats.total_families);
        println!("Configurable: {}", stats.configurable_families);
        println!("With properties: {}", stats.families_with_properties);
        println!("Total variants: {}", stats.total_variants);

        // Group families by series
        let mut by_series: std::collections::HashMap<String, Vec<_>> =
            std::collections::HashMap::new();
        for family in loader.get_families() {
            let series = family.series.clone();
            by_series.entry(series).or_default().push(family.clone());
        }

        println!("\nFamilies per series:");
        let mut series_list: Vec<_> = by_series.keys().collect();
        series_list.sort();
        for series in &series_list {
            let families = &by_series[*series];
            let with_props = families
                .iter()
                .filter(|f| loader.family_has_properties(f))
                .count();
            println!(
                "  {}: {} families ({} with properties)",
                series,
                families.len(),
                with_props
            );
        }

        // Check for issues
        println!("\nChecking for anomalies...");
        let mut issues = 0;
        for family in loader.get_families() {
            let props = loader.get_properties_for_family(family);
            // Check for families without properties but with configurable flag
            if family.is_configurable && props.is_empty() {
                println!(
                    "  WARN: '{}' ({}) marked configurable but no properties",
                    family.name, family.base_article_nr
                );
                issues += 1;
            }
            // Check for empty names
            if family.name.is_empty() || family.name.trim().is_empty() {
                println!("  WARN: Family '{}' has empty name", family.base_article_nr);
                issues += 1;
            }
        }

        if issues == 0 {
            println!("  No issues found!");
        } else {
            println!("  Total issues: {}", issues);
        }

        // Basic assertions
        assert!(stats.total_families > 0, "Should have loaded some families");
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

    // Unit tests that don't require external data

    #[test]
    fn test_property_option_clone_debug() {
        let option = PropertyOption {
            value: "VAL1".to_string(),
            label: "Value 1".to_string(),
            is_default: true,
        };
        let cloned = option.clone();
        assert_eq!(cloned.value, "VAL1");
        assert!(cloned.is_default);
        let debug_str = format!("{:?}", option);
        assert!(debug_str.contains("VAL1"));
    }

    #[test]
    fn test_family_property_clone_debug() {
        let prop = FamilyProperty {
            key: "COLOR".to_string(),
            label: "Color".to_string(),
            group: "appearance".to_string(),
            group_label: "Appearance".to_string(),
            prop_type: PropertyType::Choice,
            required: false,
            options: vec![],
            default_value: None,
            position: 1,
            hint: None,
        };
        let cloned = prop.clone();
        assert_eq!(cloned.key, "COLOR");
        assert_eq!(cloned.position, 1);
        let debug_str = format!("{:?}", prop);
        assert!(debug_str.contains("COLOR"));
    }

    #[test]
    fn test_product_family_clone_debug() {
        let family = ProductFamily {
            id: "FAM1".to_string(),
            name: "Family 1".to_string(),
            description: "Short desc".to_string(),
            long_description: "Long desc".to_string(),
            series: "ser".to_string(),
            base_article_nr: "ART-001".to_string(),
            prop_classes: vec!["CLASS1".to_string()],
            variant_count: 5,
            is_configurable: true,
            article_nrs: vec!["ART-001".to_string()],
            article_descriptions: vec![],
            article_long_descriptions: vec![],
        };
        let cloned = family.clone();
        assert_eq!(cloned.id, "FAM1");
        assert_eq!(cloned.variant_count, 5);
        assert!(cloned.is_configurable);
        let debug_str = format!("{:?}", family);
        assert!(debug_str.contains("FAM1"));
    }

    #[test]
    fn test_family_loader_stats_clone_debug() {
        let stats = FamilyLoaderStats {
            total_families: 100,
            configurable_families: 50,
            families_with_properties: 45,
            total_variants: 500,
        };
        let cloned = stats.clone();
        assert_eq!(cloned.total_families, 100);
        assert_eq!(cloned.configurable_families, 50);
        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("100"));
    }

    #[test]
    fn test_family_configuration_empty() {
        let properties: Vec<FamilyProperty> = vec![];
        let config = FamilyConfiguration::new("test-family", &properties);
        assert_eq!(config.family_id, "test-family");
        assert!(config.selections.is_empty());
        assert_eq!(config.variant_code, "");
    }

    #[test]
    fn test_family_configuration_with_properties() {
        let properties = vec![
            FamilyProperty {
                key: "SIZE".to_string(),
                label: "Size".to_string(),
                group: "dimensions".to_string(),
                group_label: "Dimensions".to_string(),
                prop_type: PropertyType::Choice,
                required: false,
                options: vec![
                    PropertyOption {
                        value: "S".to_string(),
                        label: "Small".to_string(),
                        is_default: true,
                    },
                    PropertyOption {
                        value: "L".to_string(),
                        label: "Large".to_string(),
                        is_default: false,
                    },
                ],
                default_value: Some("S".to_string()),
                position: 1,
                hint: None,
            },
        ];
        let mut config = FamilyConfiguration::new("test-family", &properties);

        // Default should be selected
        assert_eq!(config.get("SIZE"), Some("S"));

        // Change selection
        config.set("SIZE", "L");
        assert_eq!(config.get("SIZE"), Some("L"));
    }

    #[test]
    fn test_family_configuration_clone_debug() {
        let properties: Vec<FamilyProperty> = vec![];
        let config = FamilyConfiguration::new("test-family", &properties);

        let cloned = config.clone();
        assert_eq!(cloned.family_id, "test-family");

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("test-family"));
    }

    #[test]
    fn test_property_type_debug_clone() {
        let choice = PropertyType::Choice;
        let choice_cloned = choice.clone();
        assert!(matches!(choice_cloned, PropertyType::Choice));

        let range = PropertyType::Range { min: 0.0, max: 100.0, step: 1.0 };
        let range_cloned = range.clone();
        if let PropertyType::Range { min, max, step } = range_cloned {
            assert_eq!(min, 0.0);
            assert_eq!(max, 100.0);
            assert_eq!(step, 1.0);
        }

        let debug_str = format!("{:?}", choice);
        assert!(debug_str.contains("Choice"));
    }
}
