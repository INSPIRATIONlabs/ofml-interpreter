//! Variant code generation from property values
//!
//! This module implements the OFML variant code generation algorithm,
//! which concatenates property value codes with underscore separators,
//! ordered by property sort order.

use crate::property::{PropertyManager, PropertyValue};

/// Generate an OFML variant code from property values.
///
/// Properties are sorted by their sort_order and concatenated with underscores.
/// The resulting code identifies a specific product configuration for pricing.
///
/// # Arguments
/// * `properties` - PropertyManager containing values and definitions
///
/// # Returns
/// A variant code string (e.g., "H720_D1200_CWH")
pub fn generate_variant_code(properties: &PropertyManager) -> String {
    let mut codes: Vec<(i32, String)> = Vec::new();

    // Collect properties with sort order > 0 (participating in variant code)
    for (name, value) in &properties.values {
        // Get sort order from definition, default to 0 if not defined
        let sort_order = properties
            .definitions
            .get(name)
            .map(|def| def.sort_order)
            .unwrap_or(0);

        // Skip properties with sort_order <= 0 (not part of variant code)
        if sort_order <= 0 {
            continue;
        }

        let code = value_to_code(value);
        if !code.is_empty() {
            codes.push((sort_order, code));
        }
    }

    // Sort by sort_order
    codes.sort_by_key(|(order, _)| *order);

    // Join with underscores
    codes
        .into_iter()
        .map(|(_, c)| c)
        .collect::<Vec<_>>()
        .join("_")
}

/// Convert a property value to its variant code representation
fn value_to_code(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Symbol(s) => s.clone(),
        PropertyValue::Int(i) => format!("{}", i),
        PropertyValue::Float(f) => {
            // Format without decimal if it's a whole number
            if f.fract() == 0.0 {
                format!("{:.0}", f)
            } else {
                format!("{}", f)
            }
        }
        PropertyValue::Bool(b) => {
            if *b {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        PropertyValue::String(s) => s.clone(),
    }
}

/// Validate variant code length against EBASE constraints
///
/// # Arguments
/// * `code` - The variant code to validate
/// * `max_length` - Maximum allowed length (typically 50)
///
/// # Returns
/// `true` if valid, `false` if too long
pub fn validate_variant_code_length(code: &str, max_length: usize) -> bool {
    code.len() <= max_length
}

/// Truncate or hash a variant code if it exceeds maximum length
///
/// # Arguments
/// * `code` - The variant code to process
/// * `max_length` - Maximum allowed length
///
/// # Returns
/// The original code if within limits, or a truncated version
pub fn truncate_variant_code(code: &str, max_length: usize) -> String {
    if code.len() <= max_length {
        code.to_string()
    } else {
        // Truncate with ellipsis indicator
        let truncate_to = max_length.saturating_sub(3);
        format!("{}...", &code[..truncate_to])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::{PropertyDef, PropertyType};

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
                state: crate::property::PropertyState::Enabled,
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
                state: crate::property::PropertyState::Enabled,
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
                    options: vec!["white".to_string(), "black".to_string()],
                },
                state: crate::property::PropertyState::Enabled,
                sort_order: 3,
                description: None,
                category: None,
            },
        );

        // Property without sort order (should not appear in variant code)
        pm.definitions.insert(
            "internal".to_string(),
            PropertyDef {
                name: "internal".to_string(),
                label: "Internal".to_string(),
                prop_type: PropertyType::String,
                state: crate::property::PropertyState::Hidden,
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
    fn test_generate_variant_code() {
        let pm = create_test_properties();
        let code = generate_variant_code(&pm);
        assert_eq!(code, "720_1200_white");
    }

    #[test]
    fn test_generate_variant_code_empty() {
        let pm = PropertyManager::new();
        let code = generate_variant_code(&pm);
        assert_eq!(code, "");
    }

    #[test]
    fn test_value_to_code_int() {
        assert_eq!(value_to_code(&PropertyValue::Int(720)), "720");
    }

    #[test]
    fn test_value_to_code_float_whole() {
        assert_eq!(value_to_code(&PropertyValue::Float(100.0)), "100");
    }

    #[test]
    fn test_value_to_code_float_decimal() {
        assert_eq!(value_to_code(&PropertyValue::Float(100.5)), "100.5");
    }

    #[test]
    fn test_value_to_code_bool() {
        assert_eq!(value_to_code(&PropertyValue::Bool(true)), "1");
        assert_eq!(value_to_code(&PropertyValue::Bool(false)), "0");
    }

    #[test]
    fn test_value_to_code_symbol() {
        assert_eq!(
            value_to_code(&PropertyValue::Symbol("white".to_string())),
            "white"
        );
    }

    #[test]
    fn test_validate_variant_code_length() {
        assert!(validate_variant_code_length("H720_D1200", 50));
        assert!(!validate_variant_code_length("H720_D1200", 5));
    }

    #[test]
    fn test_truncate_variant_code() {
        let code = "H720_D1200_CWH_FCR_MAT001";
        assert_eq!(truncate_variant_code(code, 50), code);
        assert_eq!(truncate_variant_code(code, 10), "H720_D1...");
    }
}
