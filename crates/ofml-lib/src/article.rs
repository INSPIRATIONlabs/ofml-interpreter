//! Article Configuration System - Product configuration from OFML data.
//!
//! This module implements article configuration for OFML products including:
//! - Loading article properties from EBASE tables
//! - Property resolution with defaults
//! - Variant selection based on article properties
//! - ODB lookup for geometry loading

use std::collections::HashMap;

use crate::errors::ArticleError;

/// A property value that can be stored in an article configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Integer value (e.g., dimensions in mm)
    Int(i64),
    /// Floating-point value
    Float(f64),
    /// String value (e.g., material codes)
    String(String),
    /// Boolean value
    Bool(bool),
}

impl PropertyValue {
    /// Convert to i64 if possible.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            PropertyValue::Int(v) => Some(*v),
            PropertyValue::Float(v) => Some(*v as i64),
            _ => None,
        }
    }

    /// Convert to f64 if possible.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            PropertyValue::Int(v) => Some(*v as f64),
            PropertyValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    /// Get as string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as bool if possible.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PropertyValue::Bool(v) => Some(*v),
            PropertyValue::Int(v) => Some(*v != 0),
            _ => None,
        }
    }

    /// Create from string value, attempting to parse as number.
    pub fn from_string(s: &str) -> Self {
        // Try parsing as integer
        if let Ok(i) = s.parse::<i64>() {
            return PropertyValue::Int(i);
        }
        // Try parsing as float
        if let Ok(f) = s.parse::<f64>() {
            return PropertyValue::Float(f);
        }
        // Try parsing as bool
        match s.to_lowercase().as_str() {
            "true" | "yes" | "1" => return PropertyValue::Bool(true),
            "false" | "no" | "0" => return PropertyValue::Bool(false),
            _ => {}
        }
        // Default to string
        PropertyValue::String(s.to_string())
    }
}

impl std::fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyValue::Int(v) => write!(f, "{}", v),
            PropertyValue::Float(v) => write!(f, "{}", v),
            PropertyValue::String(v) => write!(f, "{}", v),
            PropertyValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

impl From<i64> for PropertyValue {
    fn from(v: i64) -> Self {
        PropertyValue::Int(v)
    }
}

impl From<f64> for PropertyValue {
    fn from(v: f64) -> Self {
        PropertyValue::Float(v)
    }
}

impl From<String> for PropertyValue {
    fn from(v: String) -> Self {
        PropertyValue::String(v)
    }
}

impl From<&str> for PropertyValue {
    fn from(v: &str) -> Self {
        PropertyValue::String(v.to_string())
    }
}

impl From<bool> for PropertyValue {
    fn from(v: bool) -> Self {
        PropertyValue::Bool(v)
    }
}

/// Collection of properties for an article.
pub type Properties = HashMap<String, PropertyValue>;

/// Article configuration loaded from OFML data.
///
/// An article represents a configured product with specific dimensions,
/// materials, and options selected.
#[derive(Debug, Clone)]
pub struct ArticleConfig {
    /// Article number (e.g., "1600x800")
    pub article_nr: String,
    /// Product/ODB name for geometry lookup
    pub odb_name: String,
    /// Resolved properties for this article
    properties: Properties,
    /// Default values for properties
    defaults: Properties,
}

impl ArticleConfig {
    /// Create a new article configuration.
    pub fn new(article_nr: impl Into<String>, odb_name: impl Into<String>) -> Self {
        Self {
            article_nr: article_nr.into(),
            odb_name: odb_name.into(),
            properties: HashMap::new(),
            defaults: HashMap::new(),
        }
    }

    /// Set a property value.
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<PropertyValue>) {
        self.properties.insert(name.into(), value.into());
    }

    /// Set a default value for a property.
    pub fn set_default(&mut self, name: impl Into<String>, value: impl Into<PropertyValue>) {
        self.defaults.insert(name.into(), value.into());
    }

    /// Get a property value, falling back to default if not set.
    pub fn get(&self, name: &str) -> Option<&PropertyValue> {
        self.properties
            .get(name)
            .or_else(|| self.defaults.get(name))
    }

    /// Get a property value as i64, with optional default.
    pub fn get_int(&self, name: &str, default: i64) -> i64 {
        self.get(name).and_then(|v| v.as_int()).unwrap_or(default)
    }

    /// Get a property value as f64, with optional default.
    pub fn get_float(&self, name: &str, default: f64) -> f64 {
        self.get(name).and_then(|v| v.as_float()).unwrap_or(default)
    }

    /// Get a property value as string, with optional default.
    pub fn get_string(&self, name: &str, default: &str) -> String {
        self.get(name)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| default.to_string())
    }

    /// Get a property value as bool, with optional default.
    pub fn get_bool(&self, name: &str, default: bool) -> bool {
        self.get(name).and_then(|v| v.as_bool()).unwrap_or(default)
    }

    /// Check if a property exists.
    pub fn has(&self, name: &str) -> bool {
        self.properties.contains_key(name) || self.defaults.contains_key(name)
    }

    /// Get all property names.
    pub fn property_names(&self) -> Vec<&String> {
        let mut names: Vec<_> = self.properties.keys().collect();
        for key in self.defaults.keys() {
            if !self.properties.contains_key(key) {
                names.push(key);
            }
        }
        names
    }

    /// Convert properties to f64 map for EBASE evaluation.
    pub fn to_f64_map(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        for (name, value) in &self.defaults {
            if let Some(f) = value.as_float() {
                result.insert(name.clone(), f);
            }
        }
        for (name, value) in &self.properties {
            if let Some(f) = value.as_float() {
                result.insert(name.clone(), f);
            }
        }
        result
    }
}

/// Product variant definition.
///
/// A variant represents a specific configuration option for a product,
/// such as size, color, or material.
#[derive(Debug, Clone)]
pub struct Variant {
    /// Variant identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Property overrides for this variant
    pub properties: Properties,
    /// Whether this variant is available
    pub available: bool,
}

impl Variant {
    /// Create a new variant.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            properties: HashMap::new(),
            available: true,
        }
    }

    /// Add a property override.
    pub fn with_property(
        mut self,
        name: impl Into<String>,
        value: impl Into<PropertyValue>,
    ) -> Self {
        self.properties.insert(name.into(), value.into());
        self
    }

    /// Set availability.
    pub fn with_available(mut self, available: bool) -> Self {
        self.available = available;
        self
    }
}

/// Variant group - collection of mutually exclusive variants.
#[derive(Debug, Clone)]
pub struct VariantGroup {
    /// Group identifier (e.g., "size", "color")
    pub id: String,
    /// Display name
    pub name: String,
    /// Available variants in this group
    pub variants: Vec<Variant>,
    /// Currently selected variant index
    pub selected: Option<usize>,
}

impl VariantGroup {
    /// Create a new variant group.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            variants: Vec::new(),
            selected: None,
        }
    }

    /// Add a variant to the group.
    pub fn add_variant(&mut self, variant: Variant) {
        self.variants.push(variant);
    }

    /// Select a variant by ID.
    pub fn select(&mut self, variant_id: &str) -> Result<(), ArticleError> {
        let index = self
            .variants
            .iter()
            .position(|v| v.id == variant_id)
            .ok_or_else(|| ArticleError::InvalidConfiguration {
                article: self.id.clone(),
                message: format!("Variant '{}' not found in group '{}'", variant_id, self.id),
            })?;

        if !self.variants[index].available {
            return Err(ArticleError::InvalidConfiguration {
                article: self.id.clone(),
                message: format!("Variant '{}' is not available", variant_id),
            });
        }

        self.selected = Some(index);
        Ok(())
    }

    /// Get the selected variant.
    pub fn get_selected(&self) -> Option<&Variant> {
        self.selected.and_then(|i| self.variants.get(i))
    }
}

/// Standard dimension property names used in OFML.
pub const DIM_WIDTH: &str = "M__BREITE";
pub const DIM_DEPTH: &str = "M__TIEFE";
pub const DIM_HEIGHT: &str = "M__HOEHE";

/// Standard material property names.
pub const MAT_BASIC: &str = "SH__BASIC";
pub const MAT_COLOR_PREFIX: &str = "CO__";

/// Article loader that creates configurations from OFML data.
#[derive(Debug, Default)]
pub struct ArticleLoader {
    /// Default property values
    defaults: Properties,
}

impl ArticleLoader {
    /// Create a new article loader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a default property value.
    pub fn set_default(&mut self, name: impl Into<String>, value: impl Into<PropertyValue>) {
        self.defaults.insert(name.into(), value.into());
    }

    /// Create an article configuration from properties.
    pub fn create_article(
        &self,
        article_nr: &str,
        odb_name: &str,
        properties: Properties,
    ) -> ArticleConfig {
        let mut config = ArticleConfig::new(article_nr, odb_name);

        // Apply defaults
        for (name, value) in &self.defaults {
            config.set_default(name.clone(), value.clone());
        }

        // Apply properties
        for (name, value) in properties {
            config.set(name, value);
        }

        config
    }

    /// Create a standard desk article with width/depth/height.
    pub fn create_desk_article(
        &self,
        article_nr: &str,
        width_mm: i64,
        depth_mm: i64,
        height_mm: i64,
    ) -> ArticleConfig {
        let mut properties = Properties::new();
        properties.insert("M__BREITE".to_string(), PropertyValue::Int(width_mm));
        properties.insert("M__TIEFE".to_string(), PropertyValue::Int(depth_mm));
        properties.insert("M__HOEHE".to_string(), PropertyValue::Int(height_mm));

        self.create_article(article_nr, article_nr, properties)
    }

    /// Load an article from EBASE articles table record.
    ///
    /// Expected record columns: article_nr, odb_name, and property columns.
    pub fn load_from_ebase_record(&self, record: &crate::ebase::Record) -> Option<ArticleConfig> {
        use crate::ebase::Value;

        let article_nr = record
            .get("article_nr")
            .or_else(|| record.get("art_nr"))
            .and_then(|v| v.as_str())?
            .to_string();

        let odb_name = record
            .get("odb_name")
            .or_else(|| record.get("odb"))
            .and_then(|v| v.as_str())
            .unwrap_or(&article_nr)
            .to_string();

        let mut properties = Properties::new();

        // Load all properties from the record
        for (key, value) in record {
            if key == "article_nr" || key == "art_nr" || key == "odb_name" || key == "odb" {
                continue;
            }

            let prop_value = match value {
                Value::Int(i) => PropertyValue::Int(*i),
                Value::UInt(u) => PropertyValue::Int(*u as i64),
                Value::Float(f) => PropertyValue::Float(*f),
                Value::String(s) => PropertyValue::from_string(s),
                _ => continue,
            };

            properties.insert(key.clone(), prop_value);
        }

        Some(self.create_article(&article_nr, &odb_name, properties))
    }
}

impl ArticleConfig {
    /// Get dimension properties in millimeters.
    ///
    /// Returns [width, depth, height] from M__BREITE, M__TIEFE, M__HOEHE properties.
    pub fn get_dimensions_mm(&self) -> [i64; 3] {
        [
            self.get_int(DIM_WIDTH, 1000),
            self.get_int(DIM_DEPTH, 600),
            self.get_int(DIM_HEIGHT, 740),
        ]
    }

    /// Get dimension properties in meters (for geometry).
    ///
    /// Converts from millimeters to meters.
    pub fn get_dimensions_m(&self) -> [f64; 3] {
        let dims = self.get_dimensions_mm();
        [
            dims[0] as f64 / 1000.0,
            dims[1] as f64 / 1000.0,
            dims[2] as f64 / 1000.0,
        ]
    }

    /// Set dimension properties.
    pub fn set_dimensions_mm(&mut self, width: i64, depth: i64, height: i64) {
        self.set(DIM_WIDTH, width);
        self.set(DIM_DEPTH, depth);
        self.set(DIM_HEIGHT, height);
    }

    /// Get the basic material/surface code.
    pub fn get_basic_material(&self) -> Option<String> {
        self.get(MAT_BASIC)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Set the basic material/surface code.
    pub fn set_basic_material(&mut self, material: impl Into<String>) {
        self.set(MAT_BASIC, material.into());
    }

    /// Get a color property by suffix (e.g., "TISCH" for CO__TISCH).
    pub fn get_color(&self, suffix: &str) -> Option<String> {
        let key = format!("{}{}", MAT_COLOR_PREFIX, suffix);
        self.get(&key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Set a color property by suffix.
    pub fn set_color(&mut self, suffix: &str, color: impl Into<String>) {
        let key = format!("{}{}", MAT_COLOR_PREFIX, suffix);
        self.set(key, color.into());
    }

    /// Get all color properties.
    pub fn get_colors(&self) -> HashMap<String, String> {
        let mut colors = HashMap::new();
        for name in self.property_names() {
            if let Some(suffix) = name.strip_prefix(MAT_COLOR_PREFIX) {
                if let Some(value) = self.get(name).and_then(|v| v.as_str()) {
                    colors.insert(suffix.to_string(), value.to_string());
                }
            }
        }
        colors
    }

    /// Evaluate a visibility expression.
    ///
    /// Visibility expressions can be:
    /// - Empty or "1" → visible
    /// - "0" → not visible
    /// - Property references like "${PROP:-default}" → evaluate with properties
    pub fn evaluate_visibility(&self, expr: &str) -> bool {
        let expr = expr.trim();

        // Empty or "1" means visible
        if expr.is_empty() || expr == "1" || expr.to_lowercase() == "true" {
            return true;
        }

        // "0" means not visible
        if expr == "0" || expr.to_lowercase() == "false" {
            return false;
        }

        // Try to evaluate as EBASE expression
        let props = self.to_f64_map();
        let mut evaluator = crate::ebase_expr::EbaseEvaluator::new();
        match evaluator.evaluate_to_number(expr, &props) {
            Ok(val) => val != 0.0,
            Err(_) => true, // Default to visible on error
        }
    }

    /// Apply variant properties to this configuration.
    ///
    /// This updates the configuration with the variant's property overrides.
    pub fn apply_variant(&mut self, variant: &Variant) {
        for (name, value) in &variant.properties {
            self.set(name.clone(), value.clone());
        }
    }

    /// Apply variant group selection.
    ///
    /// Returns error if no variant is selected.
    pub fn apply_variant_group(&mut self, group: &VariantGroup) -> Result<(), ArticleError> {
        let variant = group
            .get_selected()
            .ok_or_else(|| ArticleError::InvalidConfiguration {
                article: self.article_nr.clone(),
                message: format!("No variant selected in group '{}'", group.id),
            })?;
        self.apply_variant(variant);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_value_conversion() {
        let int_val = PropertyValue::Int(42);
        assert_eq!(int_val.as_int(), Some(42));
        assert_eq!(int_val.as_float(), Some(42.0));
        assert_eq!(int_val.as_bool(), Some(true));

        let float_val = PropertyValue::Float(3.14);
        assert_eq!(float_val.as_float(), Some(3.14));
        assert_eq!(float_val.as_int(), Some(3));

        let string_val = PropertyValue::String("test".to_string());
        assert_eq!(string_val.as_str(), Some("test"));
        assert_eq!(string_val.as_int(), None);

        let bool_val = PropertyValue::Bool(true);
        assert_eq!(bool_val.as_bool(), Some(true));
    }

    #[test]
    fn test_property_value_from_string() {
        assert_eq!(PropertyValue::from_string("42"), PropertyValue::Int(42));
        assert_eq!(
            PropertyValue::from_string("3.14"),
            PropertyValue::Float(3.14)
        );
        assert_eq!(
            PropertyValue::from_string("true"),
            PropertyValue::Bool(true)
        );
        assert_eq!(
            PropertyValue::from_string("hello"),
            PropertyValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_article_config() {
        let mut config = ArticleConfig::new("1600x800", "desk_1600x800");
        config.set("M__BREITE", 1600i64);
        config.set("M__TIEFE", 800i64);
        config.set_default("M__HOEHE", 740i64);

        assert_eq!(config.get_int("M__BREITE", 0), 1600);
        assert_eq!(config.get_int("M__TIEFE", 0), 800);
        assert_eq!(config.get_int("M__HOEHE", 0), 740); // from default
        assert_eq!(config.get_int("MISSING", 100), 100); // fallback
    }

    #[test]
    fn test_to_f64_map() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("WIDTH", 1600i64);
        config.set("HEIGHT", 740.0f64);
        config.set("NAME", "desk");

        let map = config.to_f64_map();
        assert_eq!(map.get("WIDTH"), Some(&1600.0));
        assert_eq!(map.get("HEIGHT"), Some(&740.0));
        assert!(map.get("NAME").is_none()); // strings not included
    }

    #[test]
    fn test_variant() {
        let variant = Variant::new("large", "Large (1800x900)")
            .with_property("M__BREITE", 1800i64)
            .with_property("M__TIEFE", 900i64);

        assert_eq!(variant.id, "large");
        assert!(variant.available);
        assert_eq!(
            variant.properties.get("M__BREITE"),
            Some(&PropertyValue::Int(1800))
        );
    }

    #[test]
    fn test_variant_group() {
        let mut group = VariantGroup::new("size", "Size");
        group.add_variant(Variant::new("small", "Small"));
        group.add_variant(Variant::new("medium", "Medium"));
        group.add_variant(Variant::new("large", "Large"));

        assert!(group.select("medium").is_ok());
        assert_eq!(
            group.get_selected().map(|v| &v.id),
            Some(&"medium".to_string())
        );

        assert!(group.select("invalid").is_err());
    }

    #[test]
    fn test_article_loader() {
        let mut loader = ArticleLoader::new();
        loader.set_default("M__HOEHE", 740i64);

        let article = loader.create_desk_article("1600x800", 1600, 800, 740);
        assert_eq!(article.get_int("M__BREITE", 0), 1600);
        assert_eq!(article.get_int("M__TIEFE", 0), 800);
        assert_eq!(article.get_int("M__HOEHE", 0), 740);
    }

    #[test]
    fn test_dimension_resolution() {
        let mut config = ArticleConfig::new("test", "test");
        config.set_dimensions_mm(1600, 800, 740);

        let dims_mm = config.get_dimensions_mm();
        assert_eq!(dims_mm, [1600, 800, 740]);

        let dims_m = config.get_dimensions_m();
        assert!((dims_m[0] - 1.6).abs() < 0.001);
        assert!((dims_m[1] - 0.8).abs() < 0.001);
        assert!((dims_m[2] - 0.74).abs() < 0.001);
    }

    #[test]
    fn test_material_resolution() {
        let mut config = ArticleConfig::new("test", "test");
        config.set_basic_material("wood_oak");
        config.set_color("TISCH", "white");
        config.set_color("GESTELL", "chrome");

        assert_eq!(config.get_basic_material(), Some("wood_oak".to_string()));
        assert_eq!(config.get_color("TISCH"), Some("white".to_string()));
        assert_eq!(config.get_color("GESTELL"), Some("chrome".to_string()));

        let colors = config.get_colors();
        assert_eq!(colors.len(), 2);
        assert_eq!(colors.get("TISCH"), Some(&"white".to_string()));
    }

    #[test]
    fn test_visibility_evaluation() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("M__BREITE", 1600i64);
        config.set("SHOW_OPTION", 1i64);

        // Simple cases
        assert!(config.evaluate_visibility(""));
        assert!(config.evaluate_visibility("1"));
        assert!(config.evaluate_visibility("true"));
        assert!(!config.evaluate_visibility("0"));
        assert!(!config.evaluate_visibility("false"));

        // Expression evaluation
        assert!(config.evaluate_visibility("${SHOW_OPTION:-0}"));
    }

    #[test]
    fn test_variant_application() {
        let mut config = ArticleConfig::new("test", "test");
        config.set_dimensions_mm(1600, 800, 740);

        let variant = Variant::new("large", "Large")
            .with_property("M__BREITE", 1800i64)
            .with_property("M__TIEFE", 900i64);

        config.apply_variant(&variant);

        assert_eq!(config.get_int("M__BREITE", 0), 1800);
        assert_eq!(config.get_int("M__TIEFE", 0), 900);
        assert_eq!(config.get_int("M__HOEHE", 0), 740); // Unchanged
    }

    #[test]
    fn test_variant_group_application() {
        let mut config = ArticleConfig::new("test", "test");
        config.set_dimensions_mm(1600, 800, 740);

        let mut group = VariantGroup::new("size", "Size");
        group.add_variant(Variant::new("small", "Small").with_property("M__BREITE", 1200i64));
        group.add_variant(Variant::new("large", "Large").with_property("M__BREITE", 1800i64));

        group.select("large").unwrap();
        config.apply_variant_group(&group).unwrap();

        assert_eq!(config.get_int("M__BREITE", 0), 1800);
    }

    #[test]
    fn test_load_from_ebase_record() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert(
            "article_nr".to_string(),
            Value::String("desk_001".to_string()),
        );
        record.insert(
            "odb_name".to_string(),
            Value::String("desk_model".to_string()),
        );
        record.insert("M__BREITE".to_string(), Value::Int(1600));
        record.insert("M__TIEFE".to_string(), Value::Int(800));

        let loader = ArticleLoader::new();
        let config = loader.load_from_ebase_record(&record).unwrap();

        assert_eq!(config.article_nr, "desk_001");
        assert_eq!(config.odb_name, "desk_model");
        assert_eq!(config.get_int("M__BREITE", 0), 1600);
        assert_eq!(config.get_int("M__TIEFE", 0), 800);
    }

    #[test]
    fn test_property_value_as_str() {
        let str_val = PropertyValue::String("hello".to_string());
        assert_eq!(str_val.as_str(), Some("hello"));

        let int_val = PropertyValue::Int(42);
        assert!(int_val.as_str().is_none());
    }

    #[test]
    fn test_property_value_as_bool() {
        let bool_val = PropertyValue::Bool(true);
        assert_eq!(bool_val.as_bool(), Some(true));

        let int_val = PropertyValue::Int(1);
        assert_eq!(int_val.as_bool(), Some(true));

        let zero_val = PropertyValue::Int(0);
        assert_eq!(zero_val.as_bool(), Some(false));

        let str_val = PropertyValue::String("hello".to_string());
        assert!(str_val.as_bool().is_none());
    }

    #[test]
    fn test_property_value_display() {
        let int_val = PropertyValue::Int(42);
        assert_eq!(format!("{}", int_val), "42");

        let float_val = PropertyValue::Float(3.14);
        let s = format!("{}", float_val);
        assert!(s.starts_with("3.14"));

        let str_val = PropertyValue::String("hello".to_string());
        assert_eq!(format!("{}", str_val), "hello");

        let bool_val = PropertyValue::Bool(true);
        assert_eq!(format!("{}", bool_val), "true");
    }

    #[test]
    fn test_property_value_from_bool() {
        let pv: PropertyValue = true.into();
        assert!(matches!(pv, PropertyValue::Bool(true)));

        let pv: PropertyValue = false.into();
        assert!(matches!(pv, PropertyValue::Bool(false)));
    }

    #[test]
    fn test_article_config_get_float() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("WIDTH", 1600.5f64);
        assert!((config.get_float("WIDTH", 0.0) - 1600.5).abs() < 0.001);
        assert!((config.get_float("MISSING", 100.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_article_config_get_string() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("NAME", "desk");
        assert_eq!(config.get_string("NAME", "default"), "desk");
        assert_eq!(config.get_string("MISSING", "default"), "default");
    }

    #[test]
    fn test_article_config_get_bool() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("ACTIVE", PropertyValue::Bool(true));
        assert!(config.get_bool("ACTIVE", false));
        assert!(!config.get_bool("MISSING", false));
    }

    #[test]
    fn test_article_config_has() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("PROP1", 100i64);
        config.set_default("PROP2", 200i64);

        assert!(config.has("PROP1"));
        assert!(config.has("PROP2"));
        assert!(!config.has("PROP3"));
    }

    #[test]
    fn test_article_config_property_names_with_defaults() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("PROP1", 100i64);
        config.set_default("PROP2", 200i64);
        config.set_default("PROP3", 300i64);

        let names = config.property_names();
        // PROP1 from properties, PROP2 and PROP3 from defaults
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn test_article_config_to_f64_map_with_defaults() {
        let mut config = ArticleConfig::new("test", "test");
        config.set("WIDTH", 1600i64);
        config.set_default("HEIGHT", 740i64);
        config.set_default("DEPTH", 800.5f64);

        let map = config.to_f64_map();
        // Both properties and defaults are included
        assert_eq!(map.get("WIDTH"), Some(&1600.0));
        assert_eq!(map.get("HEIGHT"), Some(&740.0));
        assert!((map.get("DEPTH").unwrap() - 800.5).abs() < 0.001);
    }

    #[test]
    fn test_variant_with_available() {
        let variant = Variant::new("test", "Test")
            .with_available(false);
        assert!(!variant.available);
    }

    #[test]
    fn test_variant_group_select_unavailable() {
        let mut group = VariantGroup::new("size", "Size");
        group.add_variant(Variant::new("small", "Small").with_available(false));

        let result = group.select("small");
        assert!(result.is_err());
    }

    #[test]
    fn test_article_apply_variant_group_no_selection() {
        let mut config = ArticleConfig::new("test", "test");
        let group = VariantGroup::new("size", "Size");

        let result = config.apply_variant_group(&group);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_ebase_record_uint() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("article_nr".to_string(), Value::String("test_001".to_string()));
        record.insert("uint_prop".to_string(), Value::UInt(1234));

        let loader = ArticleLoader::new();
        let config = loader.load_from_ebase_record(&record).unwrap();
        assert_eq!(config.get_int("uint_prop", 0), 1234);
    }

    #[test]
    fn test_load_from_ebase_record_float() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("article_nr".to_string(), Value::String("test_001".to_string()));
        record.insert("float_prop".to_string(), Value::Float(3.14));

        let loader = ArticleLoader::new();
        let config = loader.load_from_ebase_record(&record).unwrap();
        assert!((config.get_float("float_prop", 0.0) - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_load_from_ebase_record_string_value() {
        use crate::ebase::{Record, Value};

        let mut record = Record::new();
        record.insert("article_nr".to_string(), Value::String("test_001".to_string()));
        record.insert("string_prop".to_string(), Value::String("hello".to_string()));

        let loader = ArticleLoader::new();
        let config = loader.load_from_ebase_record(&record).unwrap();
        assert_eq!(config.get_string("string_prop", ""), "hello");
    }

    #[test]
    fn test_evaluate_visibility_expression_error() {
        let config = ArticleConfig::new("test", "test");
        // Invalid expression should default to true (visible)
        let result = config.evaluate_visibility("invalid_expression_xyz");
        assert!(result); // Defaults to visible on error
    }
}
