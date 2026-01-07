//! Property System - OFML property types and management.
//!
//! This module implements the property system for product configuration,
//! including property types, states, and change callbacks.

use std::collections::HashMap;

use crate::errors::PropertyError;

/// Property visibility/editability state.
///
/// Maps to OFML property states:
/// - 0 = Hidden
/// - 1 = Enabled
/// - 2 = ReadOnly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertyState {
    /// Property is hidden (state = 0)
    Hidden = 0,
    /// Property is visible and editable (state = 1)
    #[default]
    Enabled = 1,
    /// Property is visible but read-only (state = 2)
    ReadOnly = 2,
}

impl PropertyState {
    /// Create from integer state value.
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => PropertyState::Hidden,
            1 => PropertyState::Enabled,
            2 => PropertyState::ReadOnly,
            _ => PropertyState::Hidden,
        }
    }

    /// Convert to integer state value.
    pub fn to_int(self) -> i32 {
        self as i32
    }

    /// Check if property is visible.
    pub fn is_visible(self) -> bool {
        !matches!(self, PropertyState::Hidden)
    }

    /// Check if property is editable.
    pub fn is_editable(self) -> bool {
        matches!(self, PropertyState::Enabled)
    }
}

/// Property type enumeration.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    /// Boolean property (on/off)
    Bool,
    /// Integer property with optional range
    Int { min: Option<i64>, max: Option<i64> },
    /// Float property with optional range
    Float { min: Option<f64>, max: Option<f64> },
    /// String property
    String,
    /// Choice property with fixed options
    Choice { options: Vec<String> },
}

impl PropertyType {
    /// Create an unbounded integer property type.
    pub fn int() -> Self {
        PropertyType::Int {
            min: None,
            max: None,
        }
    }

    /// Create an integer property type with range.
    pub fn int_range(min: i64, max: i64) -> Self {
        PropertyType::Int {
            min: Some(min),
            max: Some(max),
        }
    }

    /// Create an unbounded float property type.
    pub fn float() -> Self {
        PropertyType::Float {
            min: None,
            max: None,
        }
    }

    /// Create a float property type with range.
    pub fn float_range(min: f64, max: f64) -> Self {
        PropertyType::Float {
            min: Some(min),
            max: Some(max),
        }
    }

    /// Create a choice property type.
    pub fn choice(options: Vec<String>) -> Self {
        PropertyType::Choice { options }
    }

    /// Get the type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            PropertyType::Bool => "bool",
            PropertyType::Int { .. } => "int",
            PropertyType::Float { .. } => "float",
            PropertyType::String => "string",
            PropertyType::Choice { .. } => "choice",
        }
    }
}

/// Property value types.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// Float value
    Float(f64),
    /// String value
    String(String),
    /// Symbol value (choice selection)
    Symbol(String),
}

impl PropertyValue {
    /// Get as bool, if applicable.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PropertyValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as i64, with type coercion.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PropertyValue::Int(i) => Some(*i),
            PropertyValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    /// Get as f64, with type coercion.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PropertyValue::Float(f) => Some(*f),
            PropertyValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get as string reference.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            PropertyValue::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Get type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            PropertyValue::Bool(_) => "bool",
            PropertyValue::Int(_) => "int",
            PropertyValue::Float(_) => "float",
            PropertyValue::String(_) => "string",
            PropertyValue::Symbol(_) => "symbol",
        }
    }
}

/// Complete property definition including metadata.
#[derive(Debug, Clone)]
pub struct PropertyDef {
    /// Property name/key
    pub name: String,
    /// Display label (localized)
    pub label: String,
    /// Property type
    pub prop_type: PropertyType,
    /// Current state
    pub state: PropertyState,
    /// Sort order for UI display
    pub sort_order: i32,
    /// Tooltip/description
    pub description: Option<String>,
    /// Category for grouping
    pub category: Option<String>,
}

impl PropertyDef {
    /// Create a new property definition.
    pub fn new(name: impl Into<String>, label: impl Into<String>, prop_type: PropertyType) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            prop_type,
            state: PropertyState::Enabled,
            sort_order: 0,
            description: None,
            category: None,
        }
    }

    /// Set the sort order.
    pub fn with_sort_order(mut self, order: i32) -> Self {
        self.sort_order = order;
        self
    }

    /// Set the state.
    pub fn with_state(mut self, state: PropertyState) -> Self {
        self.state = state;
        self
    }

    /// Set the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the category.
    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.category = Some(cat.into());
        self
    }

    /// Validate a value against this property's type.
    pub fn validate(&self, value: &PropertyValue) -> Result<(), PropertyError> {
        match (&self.prop_type, value) {
            (PropertyType::Bool, PropertyValue::Bool(_)) => Ok(()),
            (PropertyType::Int { min, max }, PropertyValue::Int(i)) => {
                if let Some(min) = min {
                    if i < min {
                        return Err(PropertyError::ValidationFailed {
                            property: self.name.clone(),
                            message: format!("Value {} is less than minimum {}", i, min),
                        });
                    }
                }
                if let Some(max) = max {
                    if i > max {
                        return Err(PropertyError::ValidationFailed {
                            property: self.name.clone(),
                            message: format!("Value {} is greater than maximum {}", i, max),
                        });
                    }
                }
                Ok(())
            }
            (PropertyType::Float { min, max }, PropertyValue::Float(f)) => {
                if let Some(min) = min {
                    if f < min {
                        return Err(PropertyError::ValidationFailed {
                            property: self.name.clone(),
                            message: format!("Value {} is less than minimum {}", f, min),
                        });
                    }
                }
                if let Some(max) = max {
                    if f > max {
                        return Err(PropertyError::ValidationFailed {
                            property: self.name.clone(),
                            message: format!("Value {} is greater than maximum {}", f, max),
                        });
                    }
                }
                Ok(())
            }
            (PropertyType::String, PropertyValue::String(_)) => Ok(()),
            (PropertyType::Choice { options }, PropertyValue::Symbol(s)) => {
                if options.contains(s) {
                    Ok(())
                } else {
                    Err(PropertyError::ValidationFailed {
                        property: self.name.clone(),
                        message: format!(
                            "Invalid choice '{}', valid options are: {:?}",
                            s, options
                        ),
                    })
                }
            }
            // Allow int for float
            (PropertyType::Float { .. }, PropertyValue::Int(_)) => Ok(()),
            _ => Err(PropertyError::InvalidValue {
                property: self.name.clone(),
                message: format!(
                    "Expected type {}, got {}",
                    self.prop_type.type_name(),
                    value.type_name()
                ),
            }),
        }
    }
}

/// Property manager for tracking property changes.
#[derive(Debug, Clone)]
pub struct PropertyManager {
    /// Property definitions
    pub definitions: HashMap<String, PropertyDef>,
    /// Current property values
    pub values: HashMap<String, PropertyValue>,
    /// Property states (may differ from definition state)
    pub states: HashMap<String, PropertyState>,
    /// Change callbacks (property name -> callback names)
    pub callbacks: HashMap<String, Vec<String>>,
}

impl PropertyManager {
    /// Create a new property manager.
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            values: HashMap::new(),
            states: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    /// Register a property definition.
    pub fn register(&mut self, def: PropertyDef) {
        let name = def.name.clone();
        self.states.insert(name.clone(), def.state);
        self.definitions.insert(name, def);
    }

    /// Get a property value.
    pub fn get(&self, name: &str) -> Option<&PropertyValue> {
        self.values.get(name)
    }

    /// Set a property value.
    pub fn set(&mut self, name: &str, value: PropertyValue) -> Result<(), PropertyError> {
        // Check if read-only
        if let Some(state) = self.states.get(name) {
            if *state == PropertyState::ReadOnly {
                return Err(PropertyError::ReadOnly(name.to_string()));
            }
        }

        // Validate if definition exists
        if let Some(def) = self.definitions.get(name) {
            def.validate(&value)?;
        }

        self.values.insert(name.to_string(), value);
        Ok(())
    }

    /// Get property state.
    pub fn get_state(&self, name: &str) -> Option<PropertyState> {
        self.states.get(name).copied()
    }

    /// Set property state.
    pub fn set_state(&mut self, name: &str, state: PropertyState) {
        self.states.insert(name.to_string(), state);
    }

    /// Register a change callback.
    pub fn add_callback(&mut self, property: &str, callback: impl Into<String>) {
        self.callbacks
            .entry(property.to_string())
            .or_default()
            .push(callback.into());
    }

    /// Get callbacks for a property.
    pub fn get_callbacks(&self, property: &str) -> Option<&Vec<String>> {
        self.callbacks.get(property)
    }

    /// Remove a property.
    pub fn remove(&mut self, name: &str) {
        self.definitions.remove(name);
        self.values.remove(name);
        self.states.remove(name);
        self.callbacks.remove(name);
    }

    /// Get all property names.
    pub fn property_names(&self) -> Vec<&String> {
        self.definitions.keys().collect()
    }

    /// Check if property exists.
    pub fn contains(&self, name: &str) -> bool {
        self.definitions.contains_key(name)
    }
}

impl Default for PropertyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_state() {
        assert_eq!(PropertyState::from_int(0), PropertyState::Hidden);
        assert_eq!(PropertyState::from_int(1), PropertyState::Enabled);
        assert_eq!(PropertyState::from_int(2), PropertyState::ReadOnly);
        assert_eq!(PropertyState::from_int(99), PropertyState::Hidden);

        assert!(!PropertyState::Hidden.is_visible());
        assert!(PropertyState::Enabled.is_visible());
        assert!(PropertyState::ReadOnly.is_visible());

        assert!(!PropertyState::Hidden.is_editable());
        assert!(PropertyState::Enabled.is_editable());
        assert!(!PropertyState::ReadOnly.is_editable());
    }

    #[test]
    fn test_property_type() {
        let int_type = PropertyType::int();
        assert_eq!(int_type.type_name(), "int");

        let int_range = PropertyType::int_range(0, 100);
        if let PropertyType::Int { min, max } = int_range {
            assert_eq!(min, Some(0));
            assert_eq!(max, Some(100));
        } else {
            panic!("Expected Int type");
        }

        let choice = PropertyType::choice(vec!["a".to_string(), "b".to_string()]);
        if let PropertyType::Choice { options } = choice {
            assert_eq!(options.len(), 2);
        } else {
            panic!("Expected Choice type");
        }
    }

    #[test]
    fn test_property_value() {
        let bool_val = PropertyValue::Bool(true);
        assert_eq!(bool_val.as_bool(), Some(true));
        assert_eq!(bool_val.type_name(), "bool");

        let int_val = PropertyValue::Int(42);
        assert_eq!(int_val.as_i64(), Some(42));
        assert_eq!(int_val.as_f64(), Some(42.0));

        let float_val = PropertyValue::Float(3.14);
        assert_eq!(float_val.as_f64(), Some(3.14));
        assert_eq!(float_val.as_i64(), Some(3));

        let str_val = PropertyValue::String("hello".to_string());
        assert_eq!(str_val.as_str(), Some("hello"));
    }

    #[test]
    fn test_property_def() {
        let def = PropertyDef::new("width", "Width", PropertyType::float())
            .with_sort_order(1)
            .with_description("Product width in mm")
            .with_category("Dimensions");

        assert_eq!(def.name, "width");
        assert_eq!(def.label, "Width");
        assert_eq!(def.sort_order, 1);
        assert_eq!(def.description, Some("Product width in mm".to_string()));
        assert_eq!(def.category, Some("Dimensions".to_string()));
    }

    #[test]
    fn test_property_def_validation() {
        let int_def = PropertyDef::new("count", "Count", PropertyType::int_range(0, 10));

        assert!(int_def.validate(&PropertyValue::Int(5)).is_ok());
        assert!(int_def.validate(&PropertyValue::Int(-1)).is_err());
        assert!(int_def.validate(&PropertyValue::Int(11)).is_err());

        let choice_def = PropertyDef::new(
            "color",
            "Color",
            PropertyType::choice(vec!["red".to_string(), "blue".to_string()]),
        );

        assert!(choice_def
            .validate(&PropertyValue::Symbol("red".to_string()))
            .is_ok());
        assert!(choice_def
            .validate(&PropertyValue::Symbol("green".to_string()))
            .is_err());
    }

    #[test]
    fn test_property_manager() {
        let mut manager = PropertyManager::new();

        let def = PropertyDef::new("width", "Width", PropertyType::float());
        manager.register(def);

        manager.set("width", PropertyValue::Float(1000.0)).unwrap();

        assert_eq!(manager.get("width"), Some(&PropertyValue::Float(1000.0)));

        manager.set_state("width", PropertyState::ReadOnly);
        assert!(manager.set("width", PropertyValue::Float(2000.0)).is_err());
    }

    #[test]
    fn test_property_manager_callbacks() {
        let mut manager = PropertyManager::new();

        manager.add_callback("width", "updateGeometry");
        manager.add_callback("width", "updatePrice");

        let callbacks = manager.get_callbacks("width").unwrap();
        assert_eq!(callbacks.len(), 2);
        assert!(callbacks.contains(&"updateGeometry".to_string()));
        assert!(callbacks.contains(&"updatePrice".to_string()));
    }

    #[test]
    fn test_property_state_to_int() {
        assert_eq!(PropertyState::Hidden.to_int(), 0);
        assert_eq!(PropertyState::Enabled.to_int(), 1);
        assert_eq!(PropertyState::ReadOnly.to_int(), 2);
    }

    #[test]
    fn test_property_state_default() {
        let state = PropertyState::default();
        assert_eq!(state, PropertyState::Enabled);
    }

    #[test]
    fn test_property_type_float_range() {
        let float_type = PropertyType::float_range(0.0, 100.0);
        if let PropertyType::Float { min, max } = float_type {
            assert_eq!(min, Some(0.0));
            assert_eq!(max, Some(100.0));
        } else {
            panic!("Expected Float type");
        }
    }

    #[test]
    fn test_property_type_names() {
        assert_eq!(PropertyType::Bool.type_name(), "bool");
        assert_eq!(PropertyType::int().type_name(), "int");
        assert_eq!(PropertyType::float().type_name(), "float");
        assert_eq!(PropertyType::String.type_name(), "string");
        assert_eq!(
            PropertyType::choice(vec!["a".to_string()]).type_name(),
            "choice"
        );
    }

    #[test]
    fn test_property_value_type_names() {
        assert_eq!(PropertyValue::Bool(true).type_name(), "bool");
        assert_eq!(PropertyValue::Int(42).type_name(), "int");
        assert_eq!(PropertyValue::Float(3.14).type_name(), "float");
        assert_eq!(PropertyValue::String("test".to_string()).type_name(), "string");
        assert_eq!(PropertyValue::Symbol("sym".to_string()).type_name(), "symbol");
    }

    #[test]
    fn test_property_value_as_str_symbol() {
        let sym = PropertyValue::Symbol("option1".to_string());
        assert_eq!(sym.as_str(), Some("option1"));
    }

    #[test]
    fn test_property_value_as_none_cases() {
        let bool_val = PropertyValue::Bool(true);
        assert_eq!(bool_val.as_i64(), None);
        assert_eq!(bool_val.as_f64(), None);
        assert_eq!(bool_val.as_str(), None);

        let int_val = PropertyValue::Int(42);
        assert_eq!(int_val.as_bool(), None);
        assert_eq!(int_val.as_str(), None);

        let str_val = PropertyValue::String("test".to_string());
        assert_eq!(str_val.as_bool(), None);
        assert_eq!(str_val.as_i64(), None);
        assert_eq!(str_val.as_f64(), None);
    }

    #[test]
    fn test_property_def_with_state() {
        let def = PropertyDef::new("width", "Width", PropertyType::float())
            .with_state(PropertyState::ReadOnly);
        assert_eq!(def.state, PropertyState::ReadOnly);
    }

    #[test]
    fn test_property_def_validate_float_range() {
        let float_def = PropertyDef::new("size", "Size", PropertyType::float_range(0.0, 100.0));

        assert!(float_def.validate(&PropertyValue::Float(50.0)).is_ok());
        assert!(float_def.validate(&PropertyValue::Float(-1.0)).is_err());
        assert!(float_def.validate(&PropertyValue::Float(101.0)).is_err());
        // Int is allowed for float type
        assert!(float_def.validate(&PropertyValue::Int(50)).is_ok());
    }

    #[test]
    fn test_property_def_validate_bool() {
        let bool_def = PropertyDef::new("enabled", "Enabled", PropertyType::Bool);
        assert!(bool_def.validate(&PropertyValue::Bool(true)).is_ok());
        assert!(bool_def.validate(&PropertyValue::Bool(false)).is_ok());
        assert!(bool_def.validate(&PropertyValue::Int(1)).is_err());
    }

    #[test]
    fn test_property_def_validate_string() {
        let str_def = PropertyDef::new("name", "Name", PropertyType::String);
        assert!(str_def.validate(&PropertyValue::String("hello".to_string())).is_ok());
        assert!(str_def.validate(&PropertyValue::Int(42)).is_err());
    }

    #[test]
    fn test_property_def_validate_type_mismatch() {
        let int_def = PropertyDef::new("count", "Count", PropertyType::int());
        let result = int_def.validate(&PropertyValue::String("not a number".to_string()));
        assert!(result.is_err());
        if let Err(PropertyError::InvalidValue { property, message }) = result {
            assert_eq!(property, "count");
            assert!(message.contains("Expected type"));
        }
    }

    #[test]
    fn test_property_manager_remove() {
        let mut manager = PropertyManager::new();
        let def = PropertyDef::new("width", "Width", PropertyType::float());
        manager.register(def);
        manager.set("width", PropertyValue::Float(100.0)).unwrap();
        manager.add_callback("width", "callback");

        assert!(manager.contains("width"));

        manager.remove("width");

        assert!(!manager.contains("width"));
        assert!(manager.get("width").is_none());
        assert!(manager.get_callbacks("width").is_none());
        assert!(manager.get_state("width").is_none());
    }

    #[test]
    fn test_property_manager_property_names() {
        let mut manager = PropertyManager::new();
        manager.register(PropertyDef::new("width", "Width", PropertyType::float()));
        manager.register(PropertyDef::new("height", "Height", PropertyType::float()));
        manager.register(PropertyDef::new("depth", "Depth", PropertyType::float()));

        let names = manager.property_names();
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn test_property_manager_contains() {
        let mut manager = PropertyManager::new();
        manager.register(PropertyDef::new("width", "Width", PropertyType::float()));

        assert!(manager.contains("width"));
        assert!(!manager.contains("height"));
    }

    #[test]
    fn test_property_manager_get_state() {
        let mut manager = PropertyManager::new();
        let def = PropertyDef::new("width", "Width", PropertyType::float())
            .with_state(PropertyState::Hidden);
        manager.register(def);

        assert_eq!(manager.get_state("width"), Some(PropertyState::Hidden));
        assert_eq!(manager.get_state("nonexistent"), None);
    }

    #[test]
    fn test_property_manager_default() {
        let manager = PropertyManager::default();
        assert!(manager.definitions.is_empty());
        assert!(manager.values.is_empty());
    }

    #[test]
    fn test_property_manager_set_without_definition() {
        let mut manager = PropertyManager::new();
        // Set without registering definition - should work but no validation
        manager.set("width", PropertyValue::Float(100.0)).unwrap();
        assert_eq!(manager.get("width"), Some(&PropertyValue::Float(100.0)));
    }

    #[test]
    fn test_property_def_debug_clone() {
        let def = PropertyDef::new("width", "Width", PropertyType::float());
        let debug_str = format!("{:?}", def);
        assert!(debug_str.contains("width"));

        let cloned = def.clone();
        assert_eq!(cloned.name, "width");
    }

    #[test]
    fn test_property_state_debug_clone() {
        let state = PropertyState::Enabled;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Enabled"));

        let cloned = state.clone();
        assert_eq!(cloned, PropertyState::Enabled);
    }

    #[test]
    fn test_property_value_debug_clone() {
        let value = PropertyValue::Int(42);
        let debug_str = format!("{:?}", value);
        assert!(debug_str.contains("42"));

        let cloned = value.clone();
        assert_eq!(cloned, PropertyValue::Int(42));
    }

    #[test]
    fn test_property_type_debug_clone_partialeq() {
        let prop_type = PropertyType::int_range(0, 100);
        let debug_str = format!("{:?}", prop_type);
        assert!(debug_str.contains("Int"));

        let cloned = prop_type.clone();
        assert_eq!(cloned, PropertyType::int_range(0, 100));
        assert_ne!(cloned, PropertyType::float());
    }

    #[test]
    fn test_property_manager_debug_clone() {
        let mut manager = PropertyManager::new();
        manager.register(PropertyDef::new("width", "Width", PropertyType::float()));

        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("PropertyManager"));

        let cloned = manager.clone();
        assert!(cloned.contains("width"));
    }

    #[test]
    fn test_property_value_partialeq() {
        let val1 = PropertyValue::Int(42);
        let val2 = PropertyValue::Int(42);
        let val3 = PropertyValue::Int(43);

        assert_eq!(val1, val2);
        assert_ne!(val1, val3);

        let str1 = PropertyValue::String("test".to_string());
        let str2 = PropertyValue::String("test".to_string());
        assert_eq!(str1, str2);
    }
}
