//! GObject Type System - Extended value types with GObject semantics.
//!
//! This module implements the GObject type system for OFML compatibility,
//! providing GStringObj, GSymbolObj, GIntegerObj, GFloatObj, GSequenceObj, and GDictObj.

use std::collections::HashMap;

use crate::errors::GObjectError;
use crate::value::Value;

/// Extended value type with GObject semantics.
///
/// This enum extends the basic Value type to support additional
/// OFML GObject types like Symbol and Dict.
#[derive(Debug, Clone)]
pub enum GValue {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// Integer value (GIntegerObj)
    Int(i64),
    /// Floating-point value (GFloatObj)
    Float(f64),
    /// String value (GStringObj)
    String(String),
    /// Symbol value (@name syntax, GSymbolObj)
    Symbol(String),
    /// Sequence/array value (GSequenceObj)
    Sequence(Vec<GValue>),
    /// Dictionary value (GDictObj)
    Dict(HashMap<String, GValue>),
}

impl GValue {
    /// Create a null value.
    pub fn null() -> Self {
        GValue::Null
    }

    /// Create a boolean value.
    pub fn bool(b: bool) -> Self {
        GValue::Bool(b)
    }

    /// Create an integer value.
    pub fn int(i: i64) -> Self {
        GValue::Int(i)
    }

    /// Create a float value.
    pub fn float(f: f64) -> Self {
        GValue::Float(f)
    }

    /// Create a string value.
    pub fn string(s: impl Into<String>) -> Self {
        GValue::String(s.into())
    }

    /// Create a symbol value.
    pub fn symbol(name: impl Into<String>) -> Self {
        GValue::Symbol(name.into())
    }

    /// Create an empty sequence.
    pub fn sequence() -> Self {
        GValue::Sequence(Vec::new())
    }

    /// Create a sequence from values.
    pub fn sequence_from(values: Vec<GValue>) -> Self {
        GValue::Sequence(values)
    }

    /// Create an empty dictionary.
    pub fn dict() -> Self {
        GValue::Dict(HashMap::new())
    }

    /// Create a dictionary from key-value pairs.
    pub fn dict_from(entries: HashMap<String, GValue>) -> Self {
        GValue::Dict(entries)
    }

    /// Check if this value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, GValue::Null)
    }

    /// Get the type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            GValue::Null => "null",
            GValue::Bool(_) => "boolean",
            GValue::Int(_) => "integer",
            GValue::Float(_) => "float",
            GValue::String(_) => "string",
            GValue::Symbol(_) => "symbol",
            GValue::Sequence(_) => "sequence",
            GValue::Dict(_) => "dict",
        }
    }

    /// Convert to bool, if possible.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            GValue::Bool(b) => Some(*b),
            GValue::Int(i) => Some(*i != 0),
            _ => None,
        }
    }

    /// Convert to i64, if possible.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            GValue::Int(i) => Some(*i),
            GValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    /// Convert to f64, if possible.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            GValue::Float(f) => Some(*f),
            GValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get as string reference, if this is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            GValue::String(s) => Some(s),
            GValue::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Get as sequence reference, if this is a sequence.
    pub fn as_sequence(&self) -> Option<&Vec<GValue>> {
        match self {
            GValue::Sequence(v) => Some(v),
            _ => None,
        }
    }

    /// Get as mutable sequence reference, if this is a sequence.
    pub fn as_sequence_mut(&mut self) -> Option<&mut Vec<GValue>> {
        match self {
            GValue::Sequence(v) => Some(v),
            _ => None,
        }
    }

    /// Get as dict reference, if this is a dict.
    pub fn as_dict(&self) -> Option<&HashMap<String, GValue>> {
        match self {
            GValue::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Get as mutable dict reference, if this is a dict.
    pub fn as_dict_mut(&mut self) -> Option<&mut HashMap<String, GValue>> {
        match self {
            GValue::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Parse an OFML expression string (implements parseCb).
    ///
    /// # Arguments
    ///
    /// * `expression` - The expression string to parse
    ///
    /// # Returns
    ///
    /// The parsed GValue, or an error if parsing failed.
    pub fn parse(expression: &str) -> Result<GValue, GObjectError> {
        let trimmed = expression.trim();

        if trimmed.is_empty() || trimmed == "NULL" {
            return Ok(GValue::Null);
        }

        // Boolean
        if trimmed == "true" || trimmed == "TRUE" {
            return Ok(GValue::Bool(true));
        }
        if trimmed == "false" || trimmed == "FALSE" {
            return Ok(GValue::Bool(false));
        }

        // Symbol (@name)
        if trimmed.starts_with('@') {
            return Ok(GValue::Symbol(trimmed[1..].to_string()));
        }

        // String (quoted)
        if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
            return Ok(GValue::String(trimmed[1..trimmed.len() - 1].to_string()));
        }

        // Number (integer or float)
        if let Ok(i) = trimmed.parse::<i64>() {
            return Ok(GValue::Int(i));
        }
        if let Ok(f) = trimmed.parse::<f64>() {
            return Ok(GValue::Float(f));
        }

        // Array [a, b, c]
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let inner = &trimmed[1..trimmed.len() - 1];
            let elements: Result<Vec<GValue>, GObjectError> =
                inner.split(',').map(|s| GValue::parse(s.trim())).collect();
            return elements.map(GValue::Sequence);
        }

        // Default: treat as string
        Ok(GValue::String(trimmed.to_string()))
    }

    /// Get an element from a sequence by index.
    pub fn get_index(&self, index: usize) -> Result<&GValue, GObjectError> {
        match self {
            GValue::Sequence(v) => v.get(index).ok_or(GObjectError::IndexOutOfBounds {
                index,
                length: v.len(),
            }),
            _ => Err(GObjectError::InvalidOperation {
                type_name: self.type_name().to_string(),
                operation: "index access".to_string(),
            }),
        }
    }

    /// Get a value from a dict by key.
    pub fn get_key(&self, key: &str) -> Result<&GValue, GObjectError> {
        match self {
            GValue::Dict(d) => d.get(key).ok_or(GObjectError::KeyNotFound(key.to_string())),
            _ => Err(GObjectError::InvalidOperation {
                type_name: self.type_name().to_string(),
                operation: "key access".to_string(),
            }),
        }
    }

    /// Set an element in a sequence by index.
    pub fn set_index(&mut self, index: usize, value: GValue) -> Result<(), GObjectError> {
        match self {
            GValue::Sequence(v) => {
                if index >= v.len() {
                    return Err(GObjectError::IndexOutOfBounds {
                        index,
                        length: v.len(),
                    });
                }
                v[index] = value;
                Ok(())
            }
            _ => Err(GObjectError::InvalidOperation {
                type_name: self.type_name().to_string(),
                operation: "index assignment".to_string(),
            }),
        }
    }

    /// Set a value in a dict by key.
    pub fn set_key(&mut self, key: impl Into<String>, value: GValue) -> Result<(), GObjectError> {
        match self {
            GValue::Dict(d) => {
                d.insert(key.into(), value);
                Ok(())
            }
            _ => Err(GObjectError::InvalidOperation {
                type_name: self.type_name().to_string(),
                operation: "key assignment".to_string(),
            }),
        }
    }

    /// Push a value to a sequence.
    pub fn push(&mut self, value: GValue) -> Result<(), GObjectError> {
        match self {
            GValue::Sequence(v) => {
                v.push(value);
                Ok(())
            }
            _ => Err(GObjectError::InvalidOperation {
                type_name: self.type_name().to_string(),
                operation: "push".to_string(),
            }),
        }
    }

    /// Get the length of a sequence or dict.
    pub fn len(&self) -> Result<usize, GObjectError> {
        match self {
            GValue::Sequence(v) => Ok(v.len()),
            GValue::Dict(d) => Ok(d.len()),
            GValue::String(s) => Ok(s.len()),
            _ => Err(GObjectError::InvalidOperation {
                type_name: self.type_name().to_string(),
                operation: "length".to_string(),
            }),
        }
    }

    /// Check if a sequence or dict is empty.
    pub fn is_empty(&self) -> Result<bool, GObjectError> {
        self.len().map(|l| l == 0)
    }
}

impl From<Value> for GValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => GValue::Null,
            Value::Bool(b) => GValue::Bool(b),
            Value::Int(i) => GValue::Int(i),
            Value::Float(f) => GValue::Float(f),
            Value::String(s) => GValue::String((*s).clone()),
            Value::Symbol(s) => GValue::Symbol((*s).clone()),
            Value::Array(arr) => {
                let borrowed = arr.borrow();
                GValue::Sequence(borrowed.iter().cloned().map(GValue::from).collect())
            }
            Value::Object(_) => GValue::Dict(HashMap::new()), // Objects become empty dicts
            _ => GValue::Null,
        }
    }
}

impl PartialEq for GValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GValue::Null, GValue::Null) => true,
            (GValue::Bool(a), GValue::Bool(b)) => a == b,
            (GValue::Int(a), GValue::Int(b)) => a == b,
            (GValue::Float(a), GValue::Float(b)) => (a - b).abs() < 1e-10,
            (GValue::String(a), GValue::String(b)) => a == b,
            (GValue::Symbol(a), GValue::Symbol(b)) => a == b,
            (GValue::Sequence(a), GValue::Sequence(b)) => a == b,
            (GValue::Dict(a), GValue::Dict(b)) => a == b,
            // Cross-type numeric comparison
            (GValue::Int(a), GValue::Float(b)) => (*a as f64 - b).abs() < 1e-10,
            (GValue::Float(a), GValue::Int(b)) => (a - *b as f64).abs() < 1e-10,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gvalue_creation() {
        assert!(GValue::null().is_null());
        assert_eq!(GValue::bool(true).as_bool(), Some(true));
        assert_eq!(GValue::int(42).as_i64(), Some(42));
        assert_eq!(GValue::float(3.14).as_f64(), Some(3.14));
        assert_eq!(GValue::string("hello").as_str(), Some("hello"));
        assert_eq!(GValue::symbol("test").as_str(), Some("test"));
    }

    #[test]
    fn test_gvalue_parse() {
        assert!(GValue::parse("NULL").unwrap().is_null());
        assert_eq!(GValue::parse("true").unwrap(), GValue::Bool(true));
        assert_eq!(GValue::parse("42").unwrap(), GValue::Int(42));
        assert_eq!(GValue::parse("3.14").unwrap(), GValue::Float(3.14));
        assert_eq!(
            GValue::parse("@symbol").unwrap(),
            GValue::Symbol("symbol".to_string())
        );
        assert_eq!(
            GValue::parse("\"hello\"").unwrap(),
            GValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_gvalue_parse_array() {
        let result = GValue::parse("[1, 2, 3]").unwrap();
        if let GValue::Sequence(v) = result {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], GValue::Int(1));
            assert_eq!(v[1], GValue::Int(2));
            assert_eq!(v[2], GValue::Int(3));
        } else {
            panic!("Expected Sequence");
        }
    }

    #[test]
    fn test_gvalue_sequence_operations() {
        let mut seq = GValue::sequence();
        seq.push(GValue::int(1)).unwrap();
        seq.push(GValue::int(2)).unwrap();

        assert_eq!(seq.len().unwrap(), 2);
        assert_eq!(seq.get_index(0).unwrap(), &GValue::Int(1));
        assert_eq!(seq.get_index(1).unwrap(), &GValue::Int(2));

        seq.set_index(0, GValue::int(10)).unwrap();
        assert_eq!(seq.get_index(0).unwrap(), &GValue::Int(10));
    }

    #[test]
    fn test_gvalue_dict_operations() {
        let mut dict = GValue::dict();
        dict.set_key("name", GValue::string("test")).unwrap();
        dict.set_key("value", GValue::int(42)).unwrap();

        assert_eq!(dict.len().unwrap(), 2);
        assert_eq!(
            dict.get_key("name").unwrap(),
            &GValue::String("test".to_string())
        );
        assert_eq!(dict.get_key("value").unwrap(), &GValue::Int(42));
    }

    #[test]
    fn test_gvalue_index_out_of_bounds() {
        let seq = GValue::sequence_from(vec![GValue::int(1)]);
        let result = seq.get_index(5);
        assert!(matches!(
            result,
            Err(GObjectError::IndexOutOfBounds {
                index: 5,
                length: 1
            })
        ));
    }

    #[test]
    fn test_gvalue_key_not_found() {
        let dict = GValue::dict();
        let result = dict.get_key("missing");
        assert!(matches!(result, Err(GObjectError::KeyNotFound(_))));
    }

    #[test]
    fn test_gvalue_type_name() {
        assert_eq!(GValue::Null.type_name(), "null");
        assert_eq!(GValue::Bool(true).type_name(), "boolean");
        assert_eq!(GValue::Int(1).type_name(), "integer");
        assert_eq!(GValue::Float(1.0).type_name(), "float");
        assert_eq!(GValue::String("".to_string()).type_name(), "string");
        assert_eq!(GValue::Symbol("".to_string()).type_name(), "symbol");
        assert_eq!(GValue::Sequence(vec![]).type_name(), "sequence");
        assert_eq!(GValue::Dict(HashMap::new()).type_name(), "dict");
    }

    #[test]
    fn test_gvalue_equality() {
        assert_eq!(GValue::Null, GValue::Null);
        assert_eq!(GValue::Bool(true), GValue::Bool(true));
        assert_eq!(GValue::Int(42), GValue::Int(42));
        assert_eq!(GValue::Float(3.14), GValue::Float(3.14));

        // Cross-type numeric equality
        assert_eq!(GValue::Int(42), GValue::Float(42.0));
        assert_eq!(GValue::Float(42.0), GValue::Int(42));
    }
}
