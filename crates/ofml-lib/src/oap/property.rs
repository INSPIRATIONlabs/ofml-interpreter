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
    use std::cell::RefCell;
    use std::rc::Rc;

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

        // Also test "int" prefix
        let pt2 = parse_property_type(&Some("int -50 50".to_string()));
        if let PropertyType::Int { min, max } = pt2 {
            assert_eq!(min, Some(-50));
            assert_eq!(max, Some(50));
        } else {
            panic!("Expected Int type");
        }
    }

    #[test]
    fn test_parse_property_type_float() {
        let pt = parse_property_type(&Some("f 0.0 1.0".to_string()));
        if let PropertyType::Float { min, max } = pt {
            assert_eq!(min, Some(0.0));
            assert_eq!(max, Some(1.0));
        } else {
            panic!("Expected Float type");
        }

        // Also test "float" prefix
        let pt2 = parse_property_type(&Some("float -100.5 100.5".to_string()));
        if let PropertyType::Float { min, max } = pt2 {
            assert_eq!(min, Some(-100.5));
            assert_eq!(max, Some(100.5));
        } else {
            panic!("Expected Float type");
        }
    }

    #[test]
    fn test_parse_property_type_bool() {
        let pt = parse_property_type(&Some("bool".to_string()));
        assert!(matches!(pt, PropertyType::Bool));

        let pt2 = parse_property_type(&Some("b".to_string()));
        assert!(matches!(pt2, PropertyType::Bool));
    }

    #[test]
    fn test_parse_property_type_string() {
        let pt = parse_property_type(&Some("string".to_string()));
        assert!(matches!(pt, PropertyType::String));

        let pt2 = parse_property_type(&Some("s".to_string()));
        assert!(matches!(pt2, PropertyType::String));
    }

    #[test]
    fn test_parse_property_type_default() {
        // Unknown or None defaults to String
        let pt = parse_property_type(&None);
        assert!(matches!(pt, PropertyType::String));

        let pt2 = parse_property_type(&Some("unknown_type".to_string()));
        assert!(matches!(pt2, PropertyType::String));
    }

    #[test]
    fn test_value_to_property_value_int() {
        let v = Value::Int(42);
        let pv = value_to_property_value(&v);
        assert_eq!(pv, Some(PropertyValue::Int(42)));
    }

    #[test]
    fn test_value_to_property_value_float() {
        let v = Value::Float(3.14);
        let pv = value_to_property_value(&v);
        assert_eq!(pv, Some(PropertyValue::Float(3.14)));
    }

    #[test]
    fn test_value_to_property_value_bool() {
        let v = Value::Bool(true);
        let pv = value_to_property_value(&v);
        assert_eq!(pv, Some(PropertyValue::Bool(true)));
    }

    #[test]
    fn test_value_to_property_value_string() {
        let v = Value::String(Rc::new("test".to_string()));
        let pv = value_to_property_value(&v);
        assert_eq!(pv, Some(PropertyValue::String("test".to_string())));
    }

    #[test]
    fn test_value_to_property_value_symbol() {
        let v = Value::Symbol(Rc::new("sym".to_string()));
        let pv = value_to_property_value(&v);
        assert_eq!(pv, Some(PropertyValue::Symbol("sym".to_string())));
    }

    #[test]
    fn test_value_to_property_value_unsupported() {
        let v = Value::Null;
        let pv = value_to_property_value(&v);
        assert_eq!(pv, None);

        let v2 = Value::Array(Rc::new(RefCell::new(vec![])));
        assert_eq!(value_to_property_value(&v2), None);
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
    fn test_validate_int_no_bounds() {
        let def = PropertyDef {
            name: "count".to_string(),
            label: "Count".to_string(),
            prop_type: PropertyType::Int { min: None, max: None },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::Int(-1000)).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Int(1000)).is_ok());
    }

    #[test]
    fn test_validate_float_in_range() {
        let def = PropertyDef {
            name: "ratio".to_string(),
            label: "Ratio".to_string(),
            prop_type: PropertyType::Float {
                min: Some(0.0),
                max: Some(1.0),
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::Float(0.5)).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Float(-0.1)).is_err());
        assert!(validate_property_value(&def, &PropertyValue::Float(1.1)).is_err());
    }

    #[test]
    fn test_validate_float_no_bounds() {
        let def = PropertyDef {
            name: "value".to_string(),
            label: "Value".to_string(),
            prop_type: PropertyType::Float { min: None, max: None },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::Float(-1000.0)).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Float(1000.0)).is_ok());
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
        assert!(validate_property_value(&def, &PropertyValue::String("black".to_string())).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Symbol("red".to_string())).is_err());
    }

    #[test]
    fn test_validate_bool() {
        let def = PropertyDef {
            name: "enabled".to_string(),
            label: "Enabled".to_string(),
            prop_type: PropertyType::Bool,
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::Bool(true)).is_ok());
        assert!(validate_property_value(&def, &PropertyValue::Bool(false)).is_ok());
    }

    #[test]
    fn test_validate_string() {
        let def = PropertyDef {
            name: "name".to_string(),
            label: "Name".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        assert!(validate_property_value(&def, &PropertyValue::String("test".to_string())).is_ok());
    }

    #[test]
    fn test_validate_type_mismatch() {
        let def = PropertyDef {
            name: "count".to_string(),
            label: "Count".to_string(),
            prop_type: PropertyType::Int { min: None, max: None },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        // String value for Int type
        let result = validate_property_value(&def, &PropertyValue::String("test".to_string()));
        assert!(result.is_err());
        if let Err(PropertyResolutionError::InvalidPropertyType(msg)) = result {
            assert!(msg.contains("Type mismatch"));
        }
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

        let invalid = parse_property_value(&def, "not_a_number");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_parse_property_value_float() {
        let def = PropertyDef {
            name: "ratio".to_string(),
            label: "Ratio".to_string(),
            prop_type: PropertyType::Float { min: None, max: None },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        let result = parse_property_value(&def, "3.14");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Float(3.14));

        let invalid = parse_property_value(&def, "not_a_float");
        assert!(invalid.is_err());
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
            parse_property_value(&def, "yes").unwrap(),
            PropertyValue::Bool(true)
        );
        assert_eq!(
            parse_property_value(&def, "1").unwrap(),
            PropertyValue::Bool(true)
        );
        assert_eq!(
            parse_property_value(&def, "0").unwrap(),
            PropertyValue::Bool(false)
        );
        assert_eq!(
            parse_property_value(&def, "false").unwrap(),
            PropertyValue::Bool(false)
        );
        assert_eq!(
            parse_property_value(&def, "no").unwrap(),
            PropertyValue::Bool(false)
        );
        assert_eq!(
            parse_property_value(&def, "nein").unwrap(),
            PropertyValue::Bool(false)
        );

        let invalid = parse_property_value(&def, "maybe");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_parse_property_value_choice() {
        let def = PropertyDef {
            name: "color".to_string(),
            label: "Color".to_string(),
            prop_type: PropertyType::Choice {
                options: vec!["red".to_string(), "blue".to_string()],
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        let result = parse_property_value(&def, "red");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Symbol("red".to_string()));

        let invalid = parse_property_value(&def, "green");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_parse_property_value_string() {
        let def = PropertyDef {
            name: "text".to_string(),
            label: "Text".to_string(),
            prop_type: PropertyType::String,
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        };

        let result = parse_property_value(&def, "any text here");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::String("any text here".to_string()));
    }

    #[test]
    fn test_property_resolution_error_display() {
        let e1 = PropertyResolutionError::PropertyNotFound("test".to_string());
        assert!(format!("{}", e1).contains("not found"));

        let e2 = PropertyResolutionError::InvalidPropertyType("bad type".to_string());
        assert!(format!("{}", e2).contains("Invalid property type"));

        let e3 = PropertyResolutionError::ValueOutOfRange("too big".to_string());
        assert!(format!("{}", e3).contains("out of range"));

        let e4 = PropertyResolutionError::InvalidChoiceValue("invalid".to_string());
        assert!(format!("{}", e4).contains("Invalid choice"));
    }

    #[test]
    fn test_extract_properties_from_cls_null() {
        // Non-object value should return empty manager
        let v = Value::Null;
        let manager = extract_properties_from_cls(&v);
        assert!(manager.definitions.is_empty());
        assert!(manager.values.is_empty());
    }

    #[test]
    fn test_extract_properties_from_cls_empty_object() {
        use crate::value::ObjInstance;

        let obj = ObjInstance::default();
        let v = Value::Object(Rc::new(RefCell::new(obj)));
        let manager = extract_properties_from_cls(&v);

        // Empty object should return empty manager
        assert!(manager.definitions.is_empty());
        assert!(manager.values.is_empty());
    }

    #[test]
    fn test_extract_properties_from_cls_with_properties() {
        use crate::value::{ObjInstance, PropertyDef as ValuePropertyDef};

        let mut obj = ObjInstance::default();

        // Add a property definition
        obj.prop_defs.insert("color".to_string(), ValuePropertyDef {
            name: "color".to_string(),
            type_info: "ch @red @blue @green".to_string(),
            description: "Color Selection".to_string(),
            sort_order: 1,
            group: 0,
            choices: vec![],
            state: 3,
            default_value: Value::Symbol(Rc::new("red".to_string())),
        });

        // Add property state
        obj.prop_states.insert("color".to_string(), 3);

        // Add current value
        obj.properties.insert("color".to_string(), Value::Symbol(Rc::new("blue".to_string())));

        let v = Value::Object(Rc::new(RefCell::new(obj)));
        let manager = extract_properties_from_cls(&v);

        // Should have one definition
        assert_eq!(manager.definitions.len(), 1);
        let def = manager.definitions.get("color").unwrap();
        assert_eq!(def.name, "color");
        assert_eq!(def.label, "Color Selection");

        // Should have one value
        assert_eq!(manager.values.len(), 1);
        let val = manager.values.get("color").unwrap();
        assert_eq!(*val, PropertyValue::Symbol("blue".to_string()));
    }
}
