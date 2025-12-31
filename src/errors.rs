//! Error types for the OFML interpreter.
//!
//! This module provides unified error handling using `thiserror` for all
//! components of the OFML interpreter engine.

use thiserror::Error;

/// Errors that can occur during EBASE expression evaluation.
#[derive(Debug, Error)]
pub enum EbaseExprError {
    /// Unknown operator encountered in expression
    #[error("Unknown operator: {0}")]
    UnknownOperator(String),

    /// Stack underflow during operation
    #[error("Stack underflow: expected {expected} values, found {found}")]
    StackUnderflow { expected: usize, found: usize },

    /// Type mismatch during operation
    #[error("Type error: expected {expected}, found {found}")]
    TypeError {
        expected: &'static str,
        found: String,
    },

    /// Variable substitution failed
    #[error("Variable substitution failed: {0}")]
    VariableSubstitution(String),

    /// Invalid procedure block
    #[error("Invalid procedure block: {0}")]
    InvalidProcedure(String),

    /// Division by zero
    #[error("Division by zero")]
    DivisionByZero,

    /// Tokenization error
    #[error("Tokenization error at position {position}: {message}")]
    TokenError { position: usize, message: String },
}

/// Errors that can occur during material handling.
#[derive(Debug, Error)]
pub enum MaterialError {
    /// MAT file parsing error
    #[error("MAT file parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    /// Material not found
    #[error("Material not found: {0}")]
    NotFound(String),

    /// Texture loading error
    #[error("Texture loading error for '{filename}': {message}")]
    TextureError { filename: String, message: String },

    /// Texture load error (during image decoding)
    #[error("Failed to load texture '{filename}': {message}")]
    TextureLoadError { filename: String, message: String },

    /// Invalid material property
    #[error("Invalid material property '{property}': {message}")]
    InvalidProperty { property: String, message: String },

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Errors that can occur during OFML class instantiation.
#[derive(Debug, Error)]
pub enum OfmlClassError {
    /// Unknown class
    #[error("Unknown OFML class: {0}")]
    UnknownClass(String),

    /// Invalid parameter count
    #[error("Class {class} expected {expected} parameters, got {got}")]
    InvalidParamCount {
        class: String,
        expected: usize,
        got: usize,
    },

    /// Invalid parameter type
    #[error("Class {class} parameter {index}: expected {expected}, got {got}")]
    InvalidParamType {
        class: String,
        index: usize,
        expected: &'static str,
        got: String,
    },

    /// Transformation error
    #[error("Transformation error in {class}: {message}")]
    TransformError { class: String, message: String },
}

/// Errors that can occur during property handling.
#[derive(Debug, Error)]
pub enum PropertyError {
    /// Property not found
    #[error("Property not found: {0}")]
    NotFound(String),

    /// Invalid property value
    #[error("Invalid value for property '{property}': {message}")]
    InvalidValue { property: String, message: String },

    /// Property is read-only
    #[error("Property '{0}' is read-only")]
    ReadOnly(String),

    /// Property validation failed
    #[error("Validation failed for property '{property}': {message}")]
    ValidationFailed { property: String, message: String },
}

/// Errors that can occur during 2D geometry operations.
#[derive(Debug, Error)]
pub enum Geometry2DError {
    /// Invalid primitive
    #[error("Invalid 2D primitive: {0}")]
    InvalidPrimitive(String),

    /// Transform error
    #[error("Transform error: {0}")]
    TransformError(String),

    /// SVG export error
    #[error("SVG export error: {0}")]
    SvgExportError(String),

    /// Invalid attribute
    #[error("Invalid attribute '{attribute}': {message}")]
    InvalidAttribute { attribute: String, message: String },
}

/// Errors that can occur during attachment point handling.
#[derive(Debug, Error)]
pub enum AttachmentError {
    /// Attachment point not found
    #[error("Attachment point not found: {0}")]
    NotFound(String),

    /// Invalid attachment direction
    #[error("Invalid attachment direction: must be unit vector")]
    InvalidDirection,

    /// Incompatible attachment types
    #[error("Incompatible attachment types: {point1} and {point2}")]
    IncompatibleTypes { point1: String, point2: String },
}

/// Errors that can occur during article configuration.
#[derive(Debug, Error)]
pub enum ArticleError {
    /// Article not found
    #[error("Article not found: {0}")]
    NotFound(String),

    /// Missing required property
    #[error("Missing required property '{property}' for article '{article}'")]
    MissingProperty { article: String, property: String },

    /// Invalid article configuration
    #[error("Invalid configuration for article '{article}': {message}")]
    InvalidConfiguration { article: String, message: String },

    /// ODB record not found
    #[error("ODB record not found for '{0}'")]
    OdbNotFound(String),
}

/// Errors that can occur during GObject operations.
#[derive(Debug, Error)]
pub enum GObjectError {
    /// Parse error
    #[error("GObject parse error: {0}")]
    ParseError(String),

    /// Type conversion error
    #[error("Cannot convert {from} to {to}")]
    ConversionError { from: String, to: String },

    /// Invalid operation
    #[error("Invalid operation on {type_name}: {operation}")]
    InvalidOperation {
        type_name: String,
        operation: String,
    },

    /// Index out of bounds
    #[error("Index {index} out of bounds for sequence of length {length}")]
    IndexOutOfBounds { index: usize, length: usize },

    /// Key not found in dictionary
    #[error("Key not found in dictionary: {0}")]
    KeyNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebase_expr_error_display() {
        let err = EbaseExprError::UnknownOperator("foo".to_string());
        assert_eq!(format!("{}", err), "Unknown operator: foo");

        let err = EbaseExprError::StackUnderflow {
            expected: 2,
            found: 1,
        };
        assert_eq!(
            format!("{}", err),
            "Stack underflow: expected 2 values, found 1"
        );

        let err = EbaseExprError::TypeError {
            expected: "number",
            found: "string".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Type error: expected number, found string"
        );
    }

    #[test]
    fn test_material_error_display() {
        let err = MaterialError::NotFound("wood_oak".to_string());
        assert_eq!(format!("{}", err), "Material not found: wood_oak");
    }

    #[test]
    fn test_ofml_class_error_display() {
        let err = OfmlClassError::UnknownClass("::unknown::Class".to_string());
        assert_eq!(format!("{}", err), "Unknown OFML class: ::unknown::Class");

        let err = OfmlClassError::InvalidParamCount {
            class: "GoYLTrans".to_string(),
            expected: 3,
            got: 2,
        };
        assert_eq!(
            format!("{}", err),
            "Class GoYLTrans expected 3 parameters, got 2"
        );
    }

    #[test]
    fn test_property_error_display() {
        let err = PropertyError::NotFound("M__BREITE".to_string());
        assert_eq!(format!("{}", err), "Property not found: M__BREITE");

        let err = PropertyError::ReadOnly("readonly_prop".to_string());
        assert_eq!(format!("{}", err), "Property 'readonly_prop' is read-only");
    }

    #[test]
    fn test_gobject_error_display() {
        let err = GObjectError::KeyNotFound("missing_key".to_string());
        assert_eq!(
            format!("{}", err),
            "Key not found in dictionary: missing_key"
        );

        let err = GObjectError::IndexOutOfBounds {
            index: 5,
            length: 3,
        };
        assert_eq!(
            format!("{}", err),
            "Index 5 out of bounds for sequence of length 3"
        );
    }
}
