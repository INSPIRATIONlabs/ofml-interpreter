//! Unit tests for variant code generation (T015)

use ofml_lib::oap::variant::{
    generate_variant_code, truncate_variant_code, validate_variant_code_length,
};
use ofml_lib::property::{
    PropertyDef, PropertyManager, PropertyState, PropertyType, PropertyValue,
};

/// Create a test PropertyManager with sample properties
fn create_test_properties() -> PropertyManager {
    let mut pm = PropertyManager::new();

    // Add definitions with sort orders
    pm.definitions.insert(
        "height".to_string(),
        PropertyDef {
            name: "height".to_string(),
            label: "Höhe".to_string(),
            prop_type: PropertyType::Int {
                min: Some(620),
                max: Some(820),
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        },
    );

    pm.definitions.insert(
        "diameter".to_string(),
        PropertyDef {
            name: "diameter".to_string(),
            label: "Durchmesser".to_string(),
            prop_type: PropertyType::Int {
                min: Some(800),
                max: Some(1600),
            },
            state: PropertyState::Enabled,
            sort_order: 2,
            description: None,
            category: None,
        },
    );

    pm.definitions.insert(
        "color".to_string(),
        PropertyDef {
            name: "color".to_string(),
            label: "Farbe".to_string(),
            prop_type: PropertyType::Choice {
                options: vec!["white".to_string(), "black".to_string(), "oak".to_string()],
            },
            state: PropertyState::Enabled,
            sort_order: 3,
            description: None,
            category: None,
        },
    );

    // Property with sort_order 0 should NOT appear in variant code
    pm.definitions.insert(
        "internal".to_string(),
        PropertyDef {
            name: "internal".to_string(),
            label: "Internal".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Hidden,
            sort_order: 0,
            description: None,
            category: None,
        },
    );

    // Add values
    pm.values
        .insert("height".to_string(), PropertyValue::Int(720));
    pm.values
        .insert("diameter".to_string(), PropertyValue::Int(1200));
    pm.values.insert(
        "color".to_string(),
        PropertyValue::Symbol("white".to_string()),
    );
    pm.values.insert(
        "internal".to_string(),
        PropertyValue::String("test".to_string()),
    );

    pm
}

#[test]
fn test_generate_variant_code_basic() {
    let pm = create_test_properties();
    let code = generate_variant_code(&pm);

    // Properties should be ordered by sort_order: height(1), diameter(2), color(3)
    // internal(0) should be excluded
    assert_eq!(code, "720_1200_white");
}

#[test]
fn test_generate_variant_code_empty() {
    let pm = PropertyManager::new();
    let code = generate_variant_code(&pm);
    assert_eq!(code, "");
}

#[test]
fn test_generate_variant_code_single_property() {
    let mut pm = PropertyManager::new();

    pm.definitions.insert(
        "height".to_string(),
        PropertyDef {
            name: "height".to_string(),
            label: "Höhe".to_string(),
            prop_type: PropertyType::Int {
                min: None,
                max: None,
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        },
    );
    pm.values
        .insert("height".to_string(), PropertyValue::Int(720));

    let code = generate_variant_code(&pm);
    assert_eq!(code, "720");
}

#[test]
fn test_generate_variant_code_excludes_zero_sort_order() {
    let mut pm = PropertyManager::new();

    // Only add property with sort_order 0
    pm.definitions.insert(
        "hidden".to_string(),
        PropertyDef {
            name: "hidden".to_string(),
            label: "Hidden".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Hidden,
            sort_order: 0,
            description: None,
            category: None,
        },
    );
    pm.values.insert(
        "hidden".to_string(),
        PropertyValue::String("secret".to_string()),
    );

    let code = generate_variant_code(&pm);
    assert_eq!(code, "");
}

#[test]
fn test_generate_variant_code_float_whole_number() {
    let mut pm = PropertyManager::new();

    pm.definitions.insert(
        "width".to_string(),
        PropertyDef {
            name: "width".to_string(),
            label: "Breite".to_string(),
            prop_type: PropertyType::Float {
                min: None,
                max: None,
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        },
    );
    pm.values
        .insert("width".to_string(), PropertyValue::Float(100.0));

    let code = generate_variant_code(&pm);
    // Whole numbers should not have decimal point
    assert_eq!(code, "100");
}

#[test]
fn test_generate_variant_code_float_with_decimal() {
    let mut pm = PropertyManager::new();

    pm.definitions.insert(
        "width".to_string(),
        PropertyDef {
            name: "width".to_string(),
            label: "Breite".to_string(),
            prop_type: PropertyType::Float {
                min: None,
                max: None,
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        },
    );
    pm.values
        .insert("width".to_string(), PropertyValue::Float(100.5));

    let code = generate_variant_code(&pm);
    assert_eq!(code, "100.5");
}

#[test]
fn test_generate_variant_code_bool() {
    let mut pm = PropertyManager::new();

    pm.definitions.insert(
        "enabled".to_string(),
        PropertyDef {
            name: "enabled".to_string(),
            label: "Aktiviert".to_string(),
            prop_type: PropertyType::Bool,
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        },
    );
    pm.values
        .insert("enabled".to_string(), PropertyValue::Bool(true));

    let code = generate_variant_code(&pm);
    assert_eq!(code, "1");

    pm.values
        .insert("enabled".to_string(), PropertyValue::Bool(false));
    let code = generate_variant_code(&pm);
    assert_eq!(code, "0");
}

#[test]
fn test_validate_variant_code_length_valid() {
    assert!(validate_variant_code_length("H720_D1200_CWH", 50));
    assert!(validate_variant_code_length("", 50));
    assert!(validate_variant_code_length("A", 1));
}

#[test]
fn test_validate_variant_code_length_invalid() {
    assert!(!validate_variant_code_length("H720_D1200", 5));
    assert!(!validate_variant_code_length("AB", 1));
}

#[test]
fn test_truncate_variant_code_within_limit() {
    let code = "H720_D1200";
    assert_eq!(truncate_variant_code(code, 50), code);
}

#[test]
fn test_truncate_variant_code_exceeds_limit() {
    let code = "H720_D1200_CWH_FCR_MAT001";
    let truncated = truncate_variant_code(code, 10);
    assert!(truncated.len() <= 10);
    assert!(truncated.ends_with("..."));
}

#[test]
fn test_variant_code_sort_order_respected() {
    let mut pm = PropertyManager::new();

    // Add in reverse sort order to verify sorting works
    pm.definitions.insert(
        "c".to_string(),
        PropertyDef {
            name: "c".to_string(),
            label: "C".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Enabled,
            sort_order: 3,
            description: None,
            category: None,
        },
    );
    pm.definitions.insert(
        "a".to_string(),
        PropertyDef {
            name: "a".to_string(),
            label: "A".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        },
    );
    pm.definitions.insert(
        "b".to_string(),
        PropertyDef {
            name: "b".to_string(),
            label: "B".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Enabled,
            sort_order: 2,
            description: None,
            category: None,
        },
    );

    pm.values
        .insert("c".to_string(), PropertyValue::String("C".to_string()));
    pm.values
        .insert("a".to_string(), PropertyValue::String("A".to_string()));
    pm.values
        .insert("b".to_string(), PropertyValue::String("B".to_string()));

    let code = generate_variant_code(&pm);
    // Should be sorted by sort_order, not alphabetically
    assert_eq!(code, "A_B_C");
}
