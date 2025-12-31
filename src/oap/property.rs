//! Property resolution from CLS classes and EBASE
//!
//! This module provides utilities for resolving property definitions
//! from CLS class instances and enriching them with EBASE metadata.

use crate::property::{PropertyDef, PropertyManager, PropertyState, PropertyType, PropertyValue};
use crate::value::Value;

/// Error type for property resolution
#[derive(Debug, thiserror::Error)]
pub enum PropertyResolutionError {
    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    #[error("Invalid property type: {0}")]
    InvalidPropertyType(String),

    #[error("Value out of range: {0}")]
    ValueOutOfRange(String),

    #[error("Invalid choice value: {0}")]
    InvalidChoiceValue(String),
}

/// Extract property definitions from a CLS class instance
///
/// This function reads the `propList` or property definitions from
/// a CLS class instance and creates PropertyDef entries.
pub fn extract_properties_from_cls(instance: &Value) -> PropertyManager {
    let mut manager = PropertyManager::new();

    if let Value::Object(obj_ref) = instance {
        let obj = obj_ref.borrow();

        // Extract from prop_defs if available
        for (name, def) in &obj.prop_defs {
            let prop_type = parse_property_type(&Some(def.type_info.clone()));
            let state = PropertyState::from_int(obj.prop_states.get(name).copied().unwrap_or(1));

            let prop_def = PropertyDef {
                name: name.clone(),
                label: def.description.clone(), // Use description as label (value::PropertyDef)
                prop_type,
                state,
                sort_order: def.sort_order,
                description: Some(def.description.clone()),
                category: None, // value::PropertyDef doesn't have category
            };

            manager.definitions.insert(name.clone(), prop_def);
        }

        // Extract current values
        for (name, value) in &obj.properties {
            if let Some(pv) = value_to_property_value(value) {
                manager.values.insert(name.clone(), pv);
            }
        }
    }

    manager
}

/// Parse a property type from the type info string
fn parse_property_type(type_info: &Option<String>) -> PropertyType {
    match type_info.as_deref() {
        Some(info) if info.starts_with("ch ") => {
            // Choice list: "ch @opt1 @opt2 @opt3"
            let options: Vec<String> = info[3..]
                .split_whitespace()
                .map(|s| s.trim_start_matches('@').to_string())
                .collect();
            PropertyType::Choice { options }
        }
        Some(info) if info.starts_with("i ") || info.starts_with("int ") => {
            // Integer with range: "i 0 100" or "int 0 100"
            let parts: Vec<&str> = info.split_whitespace().collect();
            let min = parts.get(1).and_then(|s| s.parse().ok());
            let max = parts.get(2).and_then(|s| s.parse().ok());
            PropertyType::Int { min, max }
        }
        Some(info) if info.starts_with("f ") || info.starts_with("float ") => {
            // Float with range
            let parts: Vec<&str> = info.split_whitespace().collect();
            let min = parts.get(1).and_then(|s| s.parse().ok());
            let max = parts.get(2).and_then(|s| s.parse().ok());
            PropertyType::Float { min, max }
        }
        Some(info) if info == "bool" || info == "b" => PropertyType::Bool,
        Some(info) if info == "string" || info == "s" => PropertyType::String,
        _ => PropertyType::String, // Default to string
    }
}

/// Convert a CLS Value to a PropertyValue
fn value_to_property_value(value: &Value) -> Option<PropertyValue> {
    match value {
        Value::Int(i) => Some(PropertyValue::Int(*i)),
        Value::Float(f) => Some(PropertyValue::Float(*f)),
        Value::Bool(b) => Some(PropertyValue::Bool(*b)),
        Value::String(s) => Some(PropertyValue::String(s.to_string())),
        Value::Symbol(s) => Some(PropertyValue::Symbol(s.to_string())),
        _ => None,
    }
}

/// Validate a property value against its definition
pub fn validate_property_value(
    def: &PropertyDef,
    value: &PropertyValue,
) -> Result<(), PropertyResolutionError> {
    match (&def.prop_type, value) {
        (PropertyType::Int { min, max }, PropertyValue::Int(v)) => {
            if let Some(min_val) = min {
                if v < min_val {
                    return Err(PropertyResolutionError::ValueOutOfRange(format!(
                        "{} < {} (min)",
                        v, min_val
                    )));
                }
            }
            if let Some(max_val) = max {
                if v > max_val {
                    return Err(PropertyResolutionError::ValueOutOfRange(format!(
                        "{} > {} (max)",
                        v, max_val
                    )));
                }
            }
            Ok(())
        }
        (PropertyType::Float { min, max }, PropertyValue::Float(v)) => {
            if let Some(min_val) = min {
                if v < min_val {
                    return Err(PropertyResolutionError::ValueOutOfRange(format!(
                        "{} < {} (min)",
                        v, min_val
                    )));
                }
            }
            if let Some(max_val) = max {
                if v > max_val {
                    return Err(PropertyResolutionError::ValueOutOfRange(format!(
                        "{} > {} (max)",
                        v, max_val
                    )));
                }
            }
            Ok(())
        }
        (PropertyType::Choice { options }, PropertyValue::Symbol(s))
        | (PropertyType::Choice { options }, PropertyValue::String(s)) => {
            if options.contains(s) {
                Ok(())
            } else {
                Err(PropertyResolutionError::InvalidChoiceValue(format!(
                    "{} not in {:?}",
                    s, options
                )))
            }
        }
        (PropertyType::Bool, PropertyValue::Bool(_)) => Ok(()),
        (PropertyType::String, PropertyValue::String(_)) => Ok(()),
        _ => Err(PropertyResolutionError::InvalidPropertyType(format!(
            "Type mismatch for {}",
            def.name
        ))),
    }
}

/// Parse a property value from a string based on property type
pub fn parse_property_value(
    def: &PropertyDef,
    value_str: &str,
) -> Result<PropertyValue, PropertyResolutionError> {
    match &def.prop_type {
        PropertyType::Int { .. } => {
            value_str
                .parse::<i64>()
                .map(PropertyValue::Int)
                .map_err(|_| {
                    PropertyResolutionError::InvalidPropertyType(format!(
                        "Cannot parse '{}' as integer",
                        value_str
                    ))
                })
        }
        PropertyType::Float { .. } => {
            value_str
                .parse::<f64>()
                .map(PropertyValue::Float)
                .map_err(|_| {
                    PropertyResolutionError::InvalidPropertyType(format!(
                        "Cannot parse '{}' as float",
                        value_str
                    ))
                })
        }
        PropertyType::Bool => match value_str.to_lowercase().as_str() {
            "true" | "1" | "yes" | "ja" => Ok(PropertyValue::Bool(true)),
            "false" | "0" | "no" | "nein" => Ok(PropertyValue::Bool(false)),
            _ => Err(PropertyResolutionError::InvalidPropertyType(format!(
                "Cannot parse '{}' as boolean",
                value_str
            ))),
        },
        PropertyType::Choice { options } => {
            // Try to match as symbol first
            if options.contains(&value_str.to_string()) {
                Ok(PropertyValue::Symbol(value_str.to_string()))
            } else {
                Err(PropertyResolutionError::InvalidChoiceValue(format!(
                    "'{}' not in {:?}",
                    value_str, options
                )))
            }
        }
        PropertyType::String => Ok(PropertyValue::String(value_str.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_property_type_choice() {
        let pt = parse_property_type(&Some("ch @white @black @oak".to_string()));
        if let PropertyType::Choice { options } = pt {
            assert_eq!(options, vec!["white", "black", "oak"]);
        } else {
            panic!("Expected Choice type");
        }
    }

    #[test]
    fn test_parse_property_type_int() {
        let pt = parse_property_type(&Some("i 0 100".to_string()));
        if let PropertyType::Int { min, max } = pt {
            assert_eq!(min, Some(0));
            assert_eq!(max, Some(100));
        } else {
            panic!("Expected Int type");
        }
    }

    #[test]
    fn test_validate_int_in_range() {
        let def = PropertyDef {
            name: "height".to_string(),
            label: "Height".to_string(),
            prop_type: PropertyType::Int {
                min: Some(620),
                max: Some(820),
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::Int(720)).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Int(500)).is_err());
        assert!(validate_property_value(&def, &PropertyValue::Int(900)).is_err());
    }

    #[test]
    fn test_validate_choice() {
        let def = PropertyDef {
            name: "color".to_string(),
            label: "Color".to_string(),
            prop_type: PropertyType::Choice {
                options: vec!["white".to_string(), "black".to_string()],
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::Symbol("white".to_string())).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Symbol("red".to_string())).is_err());
    }

    #[test]
    fn test_parse_property_value_int() {
        let def = PropertyDef {
            name: "height".to_string(),
            label: "Height".to_string(),
            prop_type: PropertyType::Int {
                min: None,
                max: None,
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        let result = parse_property_value(&def, "720");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Int(720));
    }

    #[test]
    fn test_parse_property_value_bool() {
        let def = PropertyDef {
            name: "enabled".to_string(),
            label: "Enabled".to_string(),
            prop_type: PropertyType::Bool,
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert_eq!(
            parse_property_value(&def, "true").unwrap(),
            PropertyValue::Bool(true)
        );
        assert_eq!(
            parse_property_value(&def, "ja").unwrap(),
            PropertyValue::Bool(true)
        );
        assert_eq!(
            parse_property_value(&def, "0").unwrap(),
            PropertyValue::Bool(false)
        );
    }
}
