//! Integration tests for article configuration system.
//!
//! These tests verify article configuration, property resolution, and variant selection.

use ofml_lib::article::{
    ArticleConfig, ArticleLoader, Properties, PropertyValue, Variant, VariantGroup,
};
use std::collections::HashMap;

/// Test basic article configuration
#[test]
fn test_article_config_basic() {
    let mut config = ArticleConfig::new("T1600800", "desk_standard");

    // Set properties
    config.set("M__BREITE", 1600i64);
    config.set("M__TIEFE", 800i64);
    config.set("M__HOEHE", 740i64);
    config.set("SH__BASIC", "::egr::aci::ACI5");

    // Verify properties
    assert_eq!(config.article_nr, "T1600800");
    assert_eq!(config.odb_name, "desk_standard");
    assert_eq!(config.get_int("M__BREITE", 0), 1600);
    assert_eq!(config.get_int("M__TIEFE", 0), 800);
    assert_eq!(config.get_string("SH__BASIC", ""), "::egr::aci::ACI5");
}

/// Test default property values
#[test]
fn test_article_config_defaults() {
    let mut config = ArticleConfig::new("test", "test");

    // Set defaults
    config.set_default("M__HOEHE", 740i64);
    config.set_default("VARIANT", "standard");

    // Verify defaults are used when property not set
    assert_eq!(config.get_int("M__HOEHE", 0), 740);
    assert_eq!(config.get_string("VARIANT", ""), "standard");

    // Override default
    config.set("M__HOEHE", 720i64);
    assert_eq!(config.get_int("M__HOEHE", 0), 720);
}

/// Test property value types
#[test]
fn test_property_value_types() {
    // Integer
    let int_val = PropertyValue::Int(1600);
    assert_eq!(int_val.as_int(), Some(1600));
    assert_eq!(int_val.as_float(), Some(1600.0));

    // Float
    let float_val = PropertyValue::Float(3.14159);
    assert!((float_val.as_float().unwrap() - 3.14159).abs() < 0.0001);
    assert_eq!(float_val.as_int(), Some(3));

    // String
    let str_val = PropertyValue::String("wood_oak".to_string());
    assert_eq!(str_val.as_str(), Some("wood_oak"));
    assert_eq!(str_val.as_int(), None);

    // Bool
    let bool_val = PropertyValue::Bool(true);
    assert_eq!(bool_val.as_bool(), Some(true));
}

/// Test property value display
#[test]
fn test_property_value_display() {
    assert_eq!(format!("{}", PropertyValue::Int(42)), "42");
    assert_eq!(format!("{}", PropertyValue::Float(3.14)), "3.14");
    assert_eq!(
        format!("{}", PropertyValue::String("test".to_string())),
        "test"
    );
    assert_eq!(format!("{}", PropertyValue::Bool(true)), "true");
}

/// Test property value from conversions
#[test]
fn test_property_value_from() {
    let from_i64: PropertyValue = 42i64.into();
    assert_eq!(from_i64, PropertyValue::Int(42));

    let from_f64: PropertyValue = 3.14f64.into();
    assert_eq!(from_f64, PropertyValue::Float(3.14));

    let from_str: PropertyValue = "test".into();
    assert_eq!(from_str, PropertyValue::String("test".to_string()));

    let from_bool: PropertyValue = true.into();
    assert_eq!(from_bool, PropertyValue::Bool(true));
}

/// Test parsing property values from strings
#[test]
fn test_property_value_from_string() {
    // Integer
    assert_eq!(PropertyValue::from_string("42"), PropertyValue::Int(42));
    assert_eq!(PropertyValue::from_string("-100"), PropertyValue::Int(-100));

    // Float
    assert_eq!(
        PropertyValue::from_string("3.14"),
        PropertyValue::Float(3.14)
    );
    assert_eq!(
        PropertyValue::from_string("-0.5"),
        PropertyValue::Float(-0.5)
    );

    // Bool
    assert_eq!(
        PropertyValue::from_string("true"),
        PropertyValue::Bool(true)
    );
    assert_eq!(
        PropertyValue::from_string("false"),
        PropertyValue::Bool(false)
    );
    assert_eq!(PropertyValue::from_string("yes"), PropertyValue::Bool(true));
    assert_eq!(PropertyValue::from_string("no"), PropertyValue::Bool(false));

    // String (fallback)
    assert_eq!(
        PropertyValue::from_string("hello world"),
        PropertyValue::String("hello world".to_string())
    );
}

/// Test to_f64_map conversion
#[test]
fn test_to_f64_map() {
    let mut config = ArticleConfig::new("test", "test");
    config.set("WIDTH", 1600i64);
    config.set("HEIGHT", 740.5f64);
    config.set("DEPTH", 800i64);
    config.set("NAME", "desk"); // String - should be excluded

    let map = config.to_f64_map();

    assert_eq!(map.get("WIDTH"), Some(&1600.0));
    assert_eq!(map.get("HEIGHT"), Some(&740.5));
    assert_eq!(map.get("DEPTH"), Some(&800.0));
    assert!(map.get("NAME").is_none());
}

/// Test variant definition
#[test]
fn test_variant() {
    let variant = Variant::new("large", "Large Desk (1800x900)")
        .with_property("M__BREITE", 1800i64)
        .with_property("M__TIEFE", 900i64)
        .with_available(true);

    assert_eq!(variant.id, "large");
    assert_eq!(variant.name, "Large Desk (1800x900)");
    assert!(variant.available);
    assert_eq!(
        variant.properties.get("M__BREITE"),
        Some(&PropertyValue::Int(1800))
    );
}

/// Test variant availability
#[test]
fn test_variant_availability() {
    let unavailable = Variant::new("special", "Special Edition").with_available(false);

    assert!(!unavailable.available);
}

/// Test variant group
#[test]
fn test_variant_group() {
    let mut group = VariantGroup::new("size", "Desk Size");
    group.add_variant(Variant::new("small", "Small (1200x600)"));
    group.add_variant(Variant::new("medium", "Medium (1600x800)"));
    group.add_variant(Variant::new("large", "Large (1800x900)"));

    assert_eq!(group.variants.len(), 3);
    assert!(group.selected.is_none());
}

/// Test variant group selection
#[test]
fn test_variant_group_selection() {
    let mut group = VariantGroup::new("size", "Desk Size");
    group.add_variant(Variant::new("small", "Small"));
    group.add_variant(Variant::new("medium", "Medium"));
    group.add_variant(Variant::new("large", "Large"));

    // Select a variant
    assert!(group.select("medium").is_ok());
    assert_eq!(group.selected, Some(1));

    let selected = group.get_selected();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "medium");

    // Try selecting invalid variant
    assert!(group.select("invalid").is_err());

    // Selection should remain unchanged after error
    assert_eq!(group.selected, Some(1));
}

/// Test variant group with unavailable variants
#[test]
fn test_variant_group_unavailable() {
    let mut group = VariantGroup::new("size", "Desk Size");
    group.add_variant(Variant::new("available", "Available").with_available(true));
    group.add_variant(Variant::new("unavailable", "Unavailable").with_available(false));

    // Should fail to select unavailable variant
    assert!(group.select("unavailable").is_err());
    assert!(group.selected.is_none());

    // Should succeed for available variant
    assert!(group.select("available").is_ok());
    assert_eq!(group.selected, Some(0));
}

/// Test article loader
#[test]
fn test_article_loader() {
    let mut loader = ArticleLoader::new();

    // Set default height
    loader.set_default("M__HOEHE", 740i64);

    // Create article
    let article = loader.create_desk_article("1600x800", 1600, 800, 740);

    assert_eq!(article.get_int("M__BREITE", 0), 1600);
    assert_eq!(article.get_int("M__TIEFE", 0), 800);
    assert_eq!(article.get_int("M__HOEHE", 0), 740);
}

/// Test article loader with custom properties
#[test]
fn test_article_loader_custom() {
    let loader = ArticleLoader::new();

    let mut properties: Properties = HashMap::new();
    properties.insert("M__BREITE".to_string(), PropertyValue::Int(1600));
    properties.insert("M__TIEFE".to_string(), PropertyValue::Int(800));
    properties.insert(
        "SH__BASIC".to_string(),
        PropertyValue::String("::egr::aci::ACI5".to_string()),
    );

    let article = loader.create_article("custom_desk", "desk_odb", properties);

    assert_eq!(article.article_nr, "custom_desk");
    assert_eq!(article.odb_name, "desk_odb");
    assert_eq!(article.get_int("M__BREITE", 0), 1600);
    assert_eq!(article.get_string("SH__BASIC", ""), "::egr::aci::ACI5");
}

/// Test property names enumeration
#[test]
fn test_property_names() {
    let mut config = ArticleConfig::new("test", "test");
    config.set("WIDTH", 1600i64);
    config.set("HEIGHT", 740i64);
    config.set_default("DEPTH", 800i64);

    let names = config.property_names();
    assert!(names.len() >= 3);

    assert!(config.has("WIDTH"));
    assert!(config.has("HEIGHT"));
    assert!(config.has("DEPTH"));
    assert!(!config.has("MISSING"));
}
