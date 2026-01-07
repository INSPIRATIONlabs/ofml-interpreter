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
        if let Some(symbol) = trimmed.strip_prefix('@') {
            return Ok(GValue::Symbol(symbol.to_string()));
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

    #[test]
    fn test_gvalue_debug() {
        let val = GValue::Int(42);
        let debug_str = format!("{:?}", val);
        assert!(debug_str.contains("Int"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_gvalue_clone() {
        let original = GValue::String("test".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_gvalue_as_bool_from_int() {
        assert_eq!(GValue::Int(1).as_bool(), Some(true));
        assert_eq!(GValue::Int(0).as_bool(), Some(false));
        assert_eq!(GValue::Int(-1).as_bool(), Some(true));
    }

    #[test]
    fn test_gvalue_as_bool_from_float() {
        assert_eq!(GValue::Float(1.0).as_bool(), None);
        assert_eq!(GValue::String("test".to_string()).as_bool(), None);
    }

    #[test]
    fn test_gvalue_as_i64_from_float() {
        assert_eq!(GValue::Float(42.5).as_i64(), Some(42));
        assert_eq!(GValue::Float(-10.9).as_i64(), Some(-10));
    }

    #[test]
    fn test_gvalue_as_i64_from_other() {
        assert_eq!(GValue::String("42".to_string()).as_i64(), None);
        assert_eq!(GValue::Null.as_i64(), None);
    }

    #[test]
    fn test_gvalue_as_f64_from_int() {
        assert_eq!(GValue::Int(42).as_f64(), Some(42.0));
        assert_eq!(GValue::Int(-10).as_f64(), Some(-10.0));
    }

    #[test]
    fn test_gvalue_as_f64_from_other() {
        assert_eq!(GValue::String("3.14".to_string()).as_f64(), None);
        assert_eq!(GValue::Null.as_f64(), None);
    }

    #[test]
    fn test_gvalue_as_str_from_symbol() {
        assert_eq!(GValue::Symbol("mySymbol".to_string()).as_str(), Some("mySymbol"));
    }

    #[test]
    fn test_gvalue_as_str_from_other() {
        assert_eq!(GValue::Int(42).as_str(), None);
        assert_eq!(GValue::Null.as_str(), None);
    }

    #[test]
    fn test_gvalue_as_sequence_none() {
        assert!(GValue::Int(42).as_sequence().is_none());
        assert!(GValue::String("test".to_string()).as_sequence().is_none());
    }

    #[test]
    fn test_gvalue_as_sequence_mut() {
        let mut seq = GValue::sequence_from(vec![GValue::Int(1), GValue::Int(2)]);
        if let Some(v) = seq.as_sequence_mut() {
            v.push(GValue::Int(3));
        }
        assert_eq!(seq.len().unwrap(), 3);
    }

    #[test]
    fn test_gvalue_as_sequence_mut_none() {
        let mut val = GValue::Int(42);
        assert!(val.as_sequence_mut().is_none());
    }

    #[test]
    fn test_gvalue_as_dict_none() {
        assert!(GValue::Int(42).as_dict().is_none());
        assert!(GValue::Sequence(vec![]).as_dict().is_none());
    }

    #[test]
    fn test_gvalue_as_dict_mut() {
        let mut dict = GValue::dict();
        if let Some(d) = dict.as_dict_mut() {
            d.insert("key".to_string(), GValue::Int(42));
        }
        assert_eq!(dict.len().unwrap(), 1);
    }

    #[test]
    fn test_gvalue_as_dict_mut_none() {
        let mut val = GValue::Int(42);
        assert!(val.as_dict_mut().is_none());
    }

    #[test]
    fn test_gvalue_parse_empty() {
        let result = GValue::parse("").unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn test_gvalue_parse_false() {
        assert_eq!(GValue::parse("false").unwrap(), GValue::Bool(false));
        assert_eq!(GValue::parse("FALSE").unwrap(), GValue::Bool(false));
    }

    #[test]
    fn test_gvalue_parse_true_uppercase() {
        assert_eq!(GValue::parse("TRUE").unwrap(), GValue::Bool(true));
    }

    #[test]
    fn test_gvalue_parse_float() {
        assert_eq!(GValue::parse("3.14").unwrap(), GValue::Float(3.14));
        assert_eq!(GValue::parse("-2.5").unwrap(), GValue::Float(-2.5));
    }

    #[test]
    fn test_gvalue_parse_negative_int() {
        assert_eq!(GValue::parse("-42").unwrap(), GValue::Int(-42));
    }

    #[test]
    fn test_gvalue_parse_whitespace() {
        assert_eq!(GValue::parse("  42  ").unwrap(), GValue::Int(42));
        assert_eq!(GValue::parse("  true  ").unwrap(), GValue::Bool(true));
    }

    #[test]
    fn test_gvalue_parse_unquoted_string() {
        let result = GValue::parse("hello_world").unwrap();
        assert_eq!(result, GValue::String("hello_world".to_string()));
    }

    #[test]
    fn test_gvalue_get_index_invalid_type() {
        let val = GValue::Int(42);
        let result = val.get_index(0);
        assert!(matches!(result, Err(GObjectError::InvalidOperation { .. })));
    }

    #[test]
    fn test_gvalue_get_key_invalid_type() {
        let val = GValue::Int(42);
        let result = val.get_key("key");
        assert!(matches!(result, Err(GObjectError::InvalidOperation { .. })));
    }

    #[test]
    fn test_gvalue_set_index_out_of_bounds() {
        let mut seq = GValue::sequence_from(vec![GValue::Int(1)]);
        let result = seq.set_index(5, GValue::Int(10));
        assert!(matches!(result, Err(GObjectError::IndexOutOfBounds { .. })));
    }

    #[test]
    fn test_gvalue_set_index_invalid_type() {
        let mut val = GValue::Int(42);
        let result = val.set_index(0, GValue::Int(10));
        assert!(matches!(result, Err(GObjectError::InvalidOperation { .. })));
    }

    #[test]
    fn test_gvalue_set_key_invalid_type() {
        let mut val = GValue::Int(42);
        let result = val.set_key("key", GValue::Int(10));
        assert!(matches!(result, Err(GObjectError::InvalidOperation { .. })));
    }

    #[test]
    fn test_gvalue_push_invalid_type() {
        let mut val = GValue::Int(42);
        let result = val.push(GValue::Int(10));
        assert!(matches!(result, Err(GObjectError::InvalidOperation { .. })));
    }

    #[test]
    fn test_gvalue_len_string() {
        let val = GValue::String("hello".to_string());
        assert_eq!(val.len().unwrap(), 5);
    }

    #[test]
    fn test_gvalue_len_invalid_type() {
        let val = GValue::Int(42);
        let result = val.len();
        assert!(matches!(result, Err(GObjectError::InvalidOperation { .. })));
    }

    #[test]
    fn test_gvalue_is_empty() {
        let empty_seq = GValue::sequence();
        assert!(empty_seq.is_empty().unwrap());

        let non_empty_seq = GValue::sequence_from(vec![GValue::Int(1)]);
        assert!(!non_empty_seq.is_empty().unwrap());
    }

    #[test]
    fn test_gvalue_from_value_null() {
        let value = Value::Null;
        let gvalue: GValue = value.into();
        assert!(gvalue.is_null());
    }

    #[test]
    fn test_gvalue_from_value_bool() {
        let value = Value::Bool(true);
        let gvalue: GValue = value.into();
        assert_eq!(gvalue, GValue::Bool(true));
    }

    #[test]
    fn test_gvalue_from_value_int() {
        let value = Value::Int(42);
        let gvalue: GValue = value.into();
        assert_eq!(gvalue, GValue::Int(42));
    }

    #[test]
    fn test_gvalue_from_value_float() {
        let value = Value::Float(3.14);
        let gvalue: GValue = value.into();
        assert_eq!(gvalue, GValue::Float(3.14));
    }

    #[test]
    fn test_gvalue_from_value_string() {
        let value = Value::String(std::rc::Rc::new("test".to_string()));
        let gvalue: GValue = value.into();
        assert_eq!(gvalue, GValue::String("test".to_string()));
    }

    #[test]
    fn test_gvalue_from_value_symbol() {
        let value = Value::Symbol(std::rc::Rc::new("sym".to_string()));
        let gvalue: GValue = value.into();
        assert_eq!(gvalue, GValue::Symbol("sym".to_string()));
    }

    #[test]
    fn test_gvalue_equality_different_types() {
        assert_ne!(GValue::Int(42), GValue::String("42".to_string()));
        assert_ne!(GValue::Bool(true), GValue::Int(1));
        assert_ne!(GValue::Null, GValue::Int(0));
    }

    #[test]
    fn test_gvalue_equality_sequences() {
        let seq1 = GValue::sequence_from(vec![GValue::Int(1), GValue::Int(2)]);
        let seq2 = GValue::sequence_from(vec![GValue::Int(1), GValue::Int(2)]);
        let seq3 = GValue::sequence_from(vec![GValue::Int(1), GValue::Int(3)]);

        assert_eq!(seq1, seq2);
        assert_ne!(seq1, seq3);
    }

    #[test]
    fn test_gvalue_equality_dicts() {
        let mut entries1 = HashMap::new();
        entries1.insert("a".to_string(), GValue::Int(1));

        let mut entries2 = HashMap::new();
        entries2.insert("a".to_string(), GValue::Int(1));

        let dict1 = GValue::dict_from(entries1);
        let dict2 = GValue::dict_from(entries2);

        assert_eq!(dict1, dict2);
    }

    #[test]
    fn test_gvalue_equality_string_vs_symbol() {
        let string = GValue::String("test".to_string());
        let symbol = GValue::Symbol("test".to_string());
        assert_ne!(string, symbol);
    }

    #[test]
    fn test_gvalue_as_sequence() {
        let seq = GValue::sequence_from(vec![GValue::Int(1), GValue::Int(2)]);
        assert!(seq.as_sequence().is_some());
        assert_eq!(seq.as_sequence().unwrap().len(), 2);

        let not_seq = GValue::Int(42);
        assert!(not_seq.as_sequence().is_none());
    }

    #[test]
    fn test_gvalue_as_dict() {
        let mut entries = HashMap::new();
        entries.insert("key".to_string(), GValue::Int(1));
        let dict = GValue::dict_from(entries);

        assert!(dict.as_dict().is_some());
        assert!(dict.as_dict().unwrap().contains_key("key"));

        let not_dict = GValue::Int(42);
        assert!(not_dict.as_dict().is_none());
    }

    #[test]
    fn test_gvalue_from_value_array() {
        use crate::value::Value;
        use std::cell::RefCell;
        use std::rc::Rc;

        // Create a Value::Array and convert to GValue
        let arr = Value::Array(Rc::new(RefCell::new(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ])));

        let gvalue = GValue::from(arr);
        assert!(gvalue.as_sequence().is_some());
        let seq = gvalue.as_sequence().unwrap();
        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0], GValue::Int(1));
    }

    #[test]
    fn test_gvalue_from_value_object() {
        use crate::value::{ObjInstance, Value};
        use std::cell::RefCell;
        use std::rc::Rc;

        // Create a Value::Object and convert to GValue (becomes empty dict)
        let obj = ObjInstance::default();
        let val = Value::Object(Rc::new(RefCell::new(obj)));

        let gvalue = GValue::from(val);
        // Objects become empty dicts
        assert!(gvalue.as_dict().is_some());
        assert!(gvalue.as_dict().unwrap().is_empty());
    }

    #[test]
    fn test_gvalue_from_value_hash() {
        use crate::value::Value;
        use std::cell::RefCell;
        use std::rc::Rc;

        // Create a Value::Hash and convert to GValue (falls through to Null)
        let hash = Value::Hash(Rc::new(RefCell::new(HashMap::new())));
        let gvalue = GValue::from(hash);
        // Hash falls through to the _ => GValue::Null case
        assert_eq!(gvalue, GValue::Null);
    }
}
