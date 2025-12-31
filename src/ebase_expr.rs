//! EBASE Expression Evaluator - PostScript-like stack machine.
//!
//! This module implements evaluation of EBASE `ctor` field expressions
//! used in odb3d tables to determine geometry loading mode.
//!
//! ## Expression Syntax
//!
//! EBASE expressions use PostScript-like syntax:
//! - Stack-based: operators follow operands
//! - Variables: `${VAR:-default}` syntax
//! - Operators: `imp`, `clsref`, `egms`, arithmetic, conditionals
//!
//! ## Examples
//!
//! ```text
//! # Import geometry with scale
//! "table_top" 1 1 1 imp
//!
//! # CLS class instantiation with calculated parameters
//! ${M__BREITE:-100} 1000 / "::ofml::go::GoYLTrans" clsref
//!
//! # Conditional geometry
//! ${VARIANT} @left == { "left_panel" } { "right_panel" } ifelse 1 1 1 imp
//! ```

use std::collections::HashMap;

use crate::errors::EbaseExprError;

/// A value on the EBASE expression stack.
#[derive(Debug, Clone, PartialEq)]
pub enum EbaseValue {
    /// Integer value
    Int(i64),
    /// Floating-point value
    Float(f64),
    /// String value (quoted text)
    String(String),
    /// Boolean value
    Bool(bool),
    /// Procedure block (deferred execution)
    Proc(Vec<EbaseToken>),
}

impl EbaseValue {
    /// Convert to f64, if possible.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            EbaseValue::Float(f) => Some(*f),
            EbaseValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Convert to string representation.
    pub fn as_string(&self) -> String {
        match self {
            EbaseValue::Int(i) => i.to_string(),
            EbaseValue::Float(f) => f.to_string(),
            EbaseValue::String(s) => s.clone(),
            EbaseValue::Bool(b) => b.to_string(),
            EbaseValue::Proc(_) => "<procedure>".to_string(),
        }
    }

    /// Get type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            EbaseValue::Int(_) => "integer",
            EbaseValue::Float(_) => "float",
            EbaseValue::String(_) => "string",
            EbaseValue::Bool(_) => "boolean",
            EbaseValue::Proc(_) => "procedure",
        }
    }
}

/// Token types in EBASE expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum EbaseToken {
    /// Numeric literal (integer or float)
    Number(f64),
    /// Quoted string literal
    String(String),
    /// Identifier (variable name or operator)
    Ident(String),
    /// Symbol (starts with @)
    Symbol(String),
    /// Executable block start `{`
    BlockStart,
    /// Executable block end `}`
    BlockEnd,
}

/// Result of evaluating an odb3d.ctor expression.
#[derive(Debug, Clone)]
pub enum EbaseResult {
    /// Direct geometry import: "filename" sx sy sz imp
    Import {
        /// Geometry filename (without extension)
        filename: String,
        /// Scale factors [sx, sy, sz]
        scale: [f32; 3],
    },
    /// CLS class instantiation: params... "ClassName" clsref
    ClsRef {
        /// Fully qualified class name (e.g., "::ofml::go::GoYLTrans")
        class: String,
        /// Constructor parameters
        params: Vec<f64>,
    },
    /// EGMS geometry reference: "objectname" egms
    Egms {
        /// EGMS object name
        name: String,
    },
    /// No geometry result (expression evaluated for side effects)
    None,
}

/// PostScript-like stack machine for EBASE expressions.
///
/// # Example
///
/// ```
/// use ofml_interpreter::ebase_expr::{EbaseEvaluator, EbaseResult};
/// use std::collections::HashMap;
///
/// let mut evaluator = EbaseEvaluator::new();
/// let props = HashMap::new();
/// let result = evaluator.evaluate(r#""table_top" 1 1 1 imp"#, &props).unwrap();
///
/// match result {
///     EbaseResult::Import { filename, scale } => {
///         assert_eq!(filename, "table_top");
///         assert_eq!(scale, [1.0, 1.0, 1.0]);
///     }
///     _ => panic!("Expected Import result"),
/// }
/// ```
pub struct EbaseEvaluator {
    /// Operand stack
    stack: Vec<EbaseValue>,
    /// Variable bindings
    variables: HashMap<String, EbaseValue>,
    /// Final evaluation result
    result: Option<EbaseResult>,
}

impl EbaseEvaluator {
    /// Create a new evaluator.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            result: None,
        }
    }

    /// Evaluate an EBASE expression with the given property values.
    ///
    /// # Arguments
    ///
    /// * `expression` - The EBASE expression string
    /// * `properties` - Property values for variable substitution
    ///
    /// # Returns
    ///
    /// The result of evaluating the expression (Import, ClsRef, Egms, or None).
    pub fn evaluate(
        &mut self,
        expression: &str,
        properties: &HashMap<String, f64>,
    ) -> Result<EbaseResult, EbaseExprError> {
        // Reset state
        self.stack.clear();
        self.result = None;

        // Substitute variables
        let substituted = self.substitute_variables(expression, properties)?;

        // Tokenize
        let tokens = self.tokenize(&substituted)?;

        // Execute
        self.execute(&tokens)?;

        Ok(self.result.clone().unwrap_or(EbaseResult::None))
    }

    /// Evaluate a simple numeric expression.
    ///
    /// This is used for offset/rotation fields that contain arithmetic expressions
    /// rather than geometry loading commands (imp, clsref, egms).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut eval = EbaseEvaluator::new();
    /// let props = HashMap::new();
    ///
    /// // Simple number
    /// assert_eq!(eval.evaluate_to_number("0.5", &props).unwrap(), 0.5);
    ///
    /// // Arithmetic expression
    /// assert_eq!(eval.evaluate_to_number("1000 1000 /", &props).unwrap(), 1.0);
    ///
    /// // With variables
    /// let mut props = HashMap::new();
    /// props.insert("WIDTH".to_string(), 1600.0);
    /// assert_eq!(eval.evaluate_to_number("${WIDTH:-1000} 1000 /", &props).unwrap(), 1.6);
    /// ```
    pub fn evaluate_to_number(
        &mut self,
        expression: &str,
        properties: &HashMap<String, f64>,
    ) -> Result<f64, EbaseExprError> {
        // Reset state
        self.stack.clear();
        self.result = None;

        // Substitute variables
        let substituted = self.substitute_variables(expression, properties)?;

        // Tokenize
        let tokens = self.tokenize(&substituted)?;

        // Execute
        self.execute(&tokens)?;

        // Get the numeric result from the stack
        if let Some(val) = self.stack.pop() {
            val.as_f64().ok_or_else(|| EbaseExprError::TypeError {
                expected: "number",
                found: val.type_name().to_string(),
            })
        } else {
            // If stack is empty, try to parse the substituted expression directly
            // This handles simple cases like "0.5" that might not push to stack
            substituted
                .trim()
                .parse::<f64>()
                .map_err(|_| EbaseExprError::StackUnderflow {
                    expected: 1,
                    found: 0,
                })
        }
    }

    /// Substitute ${VAR:-default} patterns in the expression.
    fn substitute_variables(
        &self,
        expression: &str,
        properties: &HashMap<String, f64>,
    ) -> Result<String, EbaseExprError> {
        let mut result = String::new();
        let mut chars = expression.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume '{'

                // Read variable name
                let mut var_name = String::new();
                let mut default_value = None;

                loop {
                    match chars.next() {
                        Some(':') if chars.peek() == Some(&'-') => {
                            chars.next(); // consume '-'
                            let mut default = String::new();
                            loop {
                                match chars.next() {
                                    Some('}') => break,
                                    Some(c) => default.push(c),
                                    None => {
                                        return Err(EbaseExprError::VariableSubstitution(
                                            "Unterminated variable substitution".to_string(),
                                        ))
                                    }
                                }
                            }
                            default_value = Some(default);
                            break;
                        }
                        Some('}') => break,
                        Some(c) => var_name.push(c),
                        None => {
                            return Err(EbaseExprError::VariableSubstitution(
                                "Unterminated variable substitution".to_string(),
                            ))
                        }
                    }
                }

                // Look up variable
                if let Some(value) = properties.get(&var_name) {
                    result.push_str(&value.to_string());
                } else if let Some(default) = default_value {
                    result.push_str(&default);
                } else {
                    return Err(EbaseExprError::VariableSubstitution(format!(
                        "Variable '{}' not found and no default provided",
                        var_name
                    )));
                }
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    /// Tokenize the expression string.
    fn tokenize(&self, expression: &str) -> Result<Vec<EbaseToken>, EbaseExprError> {
        let mut tokens = Vec::new();
        let mut chars = expression.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // Whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }

                // Block delimiters
                '{' => {
                    chars.next();
                    tokens.push(EbaseToken::BlockStart);
                }
                '}' => {
                    chars.next();
                    tokens.push(EbaseToken::BlockEnd);
                }

                // String literal
                '"' => {
                    chars.next();
                    let mut s = String::new();
                    loop {
                        match chars.next() {
                            Some('"') => break,
                            Some('\\') => {
                                if let Some(escaped) = chars.next() {
                                    s.push(match escaped {
                                        'n' => '\n',
                                        't' => '\t',
                                        '\\' => '\\',
                                        '"' => '"',
                                        other => other,
                                    });
                                }
                            }
                            Some(c) => s.push(c),
                            None => {
                                return Err(EbaseExprError::TokenError {
                                    position: 0,
                                    message: "Unterminated string literal".to_string(),
                                })
                            }
                        }
                    }
                    tokens.push(EbaseToken::String(s));
                }

                // Symbol (starts with @)
                '@' => {
                    chars.next();
                    let mut name = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(EbaseToken::Symbol(name));
                }

                // Number (including negative)
                c if c.is_ascii_digit() || c == '-' || c == '.' => {
                    let mut num_str = String::new();

                    // Handle negative sign
                    if c == '-' {
                        num_str.push(c);
                        chars.next();
                        // Check if next char is a digit or dot
                        if chars
                            .peek()
                            .map_or(true, |&c| !c.is_ascii_digit() && c != '.')
                        {
                            // It's the 'neg' or subtraction operator, not a negative number
                            tokens.push(EbaseToken::Ident("-".to_string()));
                            continue;
                        }
                    }

                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match num_str.parse::<f64>() {
                        Ok(n) => tokens.push(EbaseToken::Number(n)),
                        Err(_) => {
                            return Err(EbaseExprError::TokenError {
                                position: 0,
                                message: format!("Invalid number: {}", num_str),
                            })
                        }
                    }
                }

                // Identifier
                c if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(EbaseToken::Ident(ident));
                }

                // Operators
                '+' | '*' | '/' => {
                    chars.next();
                    tokens.push(EbaseToken::Ident(c.to_string()));
                }

                // Comparison operators
                '=' | '!' | '<' | '>' => {
                    chars.next();
                    let mut op = c.to_string();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        op.push('=');
                    }
                    tokens.push(EbaseToken::Ident(op));
                }

                _ => {
                    return Err(EbaseExprError::TokenError {
                        position: 0,
                        message: format!("Unexpected character: {}", c),
                    })
                }
            }
        }

        Ok(tokens)
    }

    /// Execute the tokenized expression.
    fn execute(&mut self, tokens: &[EbaseToken]) -> Result<(), EbaseExprError> {
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                EbaseToken::Number(n) => {
                    self.stack.push(EbaseValue::Float(*n));
                }
                EbaseToken::String(s) => {
                    self.stack.push(EbaseValue::String(s.clone()));
                }
                EbaseToken::Symbol(s) => {
                    self.stack.push(EbaseValue::String(format!("@{}", s)));
                }
                EbaseToken::BlockStart => {
                    // Collect tokens until matching BlockEnd
                    let mut block_tokens = Vec::new();
                    let mut depth = 1;
                    i += 1;
                    while i < tokens.len() && depth > 0 {
                        match &tokens[i] {
                            EbaseToken::BlockStart => {
                                depth += 1;
                                block_tokens.push(tokens[i].clone());
                            }
                            EbaseToken::BlockEnd => {
                                depth -= 1;
                                if depth > 0 {
                                    block_tokens.push(tokens[i].clone());
                                }
                            }
                            _ => {
                                block_tokens.push(tokens[i].clone());
                            }
                        }
                        i += 1;
                    }
                    if depth != 0 {
                        return Err(EbaseExprError::InvalidProcedure(
                            "Unbalanced braces".to_string(),
                        ));
                    }
                    self.stack.push(EbaseValue::Proc(block_tokens));
                    continue; // Skip the i += 1 at the end
                }
                EbaseToken::BlockEnd => {
                    return Err(EbaseExprError::InvalidProcedure(
                        "Unexpected '}'".to_string(),
                    ));
                }
                EbaseToken::Ident(op) => {
                    self.execute_operator(op)?;
                }
            }
            i += 1;
        }

        Ok(())
    }

    /// Execute a single operator.
    fn execute_operator(&mut self, op: &str) -> Result<(), EbaseExprError> {
        match op {
            // Geometry operators
            "imp" => {
                let sz = self.pop_number()?;
                let sy = self.pop_number()?;
                let sx = self.pop_number()?;
                let filename = self.pop_string()?;
                self.result = Some(EbaseResult::Import {
                    filename,
                    scale: [sx as f32, sy as f32, sz as f32],
                });
            }
            "clsref" => {
                let class = self.pop_string()?;
                // Collect all remaining numbers as parameters
                let mut params = Vec::new();
                loop {
                    match self.stack.last() {
                        Some(EbaseValue::Float(n)) => {
                            params.push(*n);
                            self.stack.pop();
                        }
                        Some(EbaseValue::Int(n)) => {
                            params.push(*n as f64);
                            self.stack.pop();
                        }
                        _ => break,
                    }
                }
                params.reverse();
                self.result = Some(EbaseResult::ClsRef { class, params });
            }
            "egms" => {
                let name = self.pop_string()?;
                self.result = Some(EbaseResult::Egms { name });
            }

            // Arithmetic operators
            "+" | "add" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Float(a + b));
            }
            "-" | "sub" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Float(a - b));
            }
            "*" | "mul" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Float(a * b));
            }
            "/" | "div" => {
                let b = self.pop_number()?;
                if b == 0.0 {
                    return Err(EbaseExprError::DivisionByZero);
                }
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Float(a / b));
            }
            "neg" => {
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Float(-a));
            }

            // Comparison operators
            "==" | "eq" => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = match (&a, &b) {
                    (EbaseValue::Float(x), EbaseValue::Float(y)) => (x - y).abs() < 1e-10,
                    (EbaseValue::Int(x), EbaseValue::Int(y)) => x == y,
                    (EbaseValue::String(x), EbaseValue::String(y)) => x == y,
                    (EbaseValue::Bool(x), EbaseValue::Bool(y)) => x == y,
                    _ => false,
                };
                self.stack.push(EbaseValue::Bool(result));
            }
            "!=" | "ne" => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = match (&a, &b) {
                    (EbaseValue::Float(x), EbaseValue::Float(y)) => (x - y).abs() >= 1e-10,
                    (EbaseValue::Int(x), EbaseValue::Int(y)) => x != y,
                    (EbaseValue::String(x), EbaseValue::String(y)) => x != y,
                    (EbaseValue::Bool(x), EbaseValue::Bool(y)) => x != y,
                    _ => true,
                };
                self.stack.push(EbaseValue::Bool(result));
            }
            "<" | "lt" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Bool(a < b));
            }
            ">" | "gt" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Bool(a > b));
            }
            "<=" | "le" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Bool(a <= b));
            }
            ">=" | "ge" => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.stack.push(EbaseValue::Bool(a >= b));
            }

            // Logical operators
            "and" => {
                let b = self.pop_bool()?;
                let a = self.pop_bool()?;
                self.stack.push(EbaseValue::Bool(a && b));
            }
            "or" => {
                let b = self.pop_bool()?;
                let a = self.pop_bool()?;
                self.stack.push(EbaseValue::Bool(a || b));
            }
            "not" => {
                let a = self.pop_bool()?;
                self.stack.push(EbaseValue::Bool(!a));
            }

            // Conditional operators
            "if" => {
                let proc = self.pop_proc()?;
                let cond = self.pop_bool()?;
                if cond {
                    self.execute(&proc)?;
                }
            }
            "ifelse" => {
                let else_proc = self.pop_proc()?;
                let then_proc = self.pop_proc()?;
                let cond = self.pop_bool()?;
                if cond {
                    self.execute(&then_proc)?;
                } else {
                    self.execute(&else_proc)?;
                }
            }

            // Stack operations
            "dup" => {
                let v = self.pop_value()?;
                self.stack.push(v.clone());
                self.stack.push(v);
            }
            "pop" => {
                self.pop_value()?;
            }
            "exch" => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                self.stack.push(b);
                self.stack.push(a);
            }

            _ => {
                // Check if it's a variable reference
                if let Some(value) = self.variables.get(op) {
                    self.stack.push(value.clone());
                } else {
                    return Err(EbaseExprError::UnknownOperator(op.to_string()));
                }
            }
        }

        Ok(())
    }

    /// Pop a number from the stack.
    fn pop_number(&mut self) -> Result<f64, EbaseExprError> {
        match self.stack.pop() {
            Some(EbaseValue::Float(n)) => Ok(n),
            Some(EbaseValue::Int(n)) => Ok(n as f64),
            Some(v) => Err(EbaseExprError::TypeError {
                expected: "number",
                found: v.type_name().to_string(),
            }),
            None => Err(EbaseExprError::StackUnderflow {
                expected: 1,
                found: 0,
            }),
        }
    }

    /// Pop a string from the stack.
    fn pop_string(&mut self) -> Result<String, EbaseExprError> {
        match self.stack.pop() {
            Some(EbaseValue::String(s)) => Ok(s),
            Some(v) => Err(EbaseExprError::TypeError {
                expected: "string",
                found: v.type_name().to_string(),
            }),
            None => Err(EbaseExprError::StackUnderflow {
                expected: 1,
                found: 0,
            }),
        }
    }

    /// Pop a boolean from the stack.
    fn pop_bool(&mut self) -> Result<bool, EbaseExprError> {
        match self.stack.pop() {
            Some(EbaseValue::Bool(b)) => Ok(b),
            Some(v) => Err(EbaseExprError::TypeError {
                expected: "boolean",
                found: v.type_name().to_string(),
            }),
            None => Err(EbaseExprError::StackUnderflow {
                expected: 1,
                found: 0,
            }),
        }
    }

    /// Pop a procedure from the stack.
    fn pop_proc(&mut self) -> Result<Vec<EbaseToken>, EbaseExprError> {
        match self.stack.pop() {
            Some(EbaseValue::Proc(tokens)) => Ok(tokens),
            Some(v) => Err(EbaseExprError::TypeError {
                expected: "procedure",
                found: v.type_name().to_string(),
            }),
            None => Err(EbaseExprError::StackUnderflow {
                expected: 1,
                found: 0,
            }),
        }
    }

    /// Pop any value from the stack.
    fn pop_value(&mut self) -> Result<EbaseValue, EbaseExprError> {
        self.stack.pop().ok_or(EbaseExprError::StackUnderflow {
            expected: 1,
            found: 0,
        })
    }
}

impl Default for EbaseEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_import() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();
        let result = evaluator
            .evaluate(r#""table_top" 1 1 1 imp"#, &props)
            .unwrap();

        match result {
            EbaseResult::Import { filename, scale } => {
                assert_eq!(filename, "table_top");
                assert_eq!(scale, [1.0, 1.0, 1.0]);
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_import_with_scale() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();
        let result = evaluator
            .evaluate(r#""panel" 0.5 2.0 1.5 imp"#, &props)
            .unwrap();

        match result {
            EbaseResult::Import { filename, scale } => {
                assert_eq!(filename, "panel");
                assert_eq!(scale, [0.5, 2.0, 1.5]);
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_clsref() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();
        let result = evaluator
            .evaluate(r#"100 200 300 "::ofml::go::GoYLTrans" clsref"#, &props)
            .unwrap();

        match result {
            EbaseResult::ClsRef { class, params } => {
                assert_eq!(class, "::ofml::go::GoYLTrans");
                assert_eq!(params, vec![100.0, 200.0, 300.0]);
            }
            _ => panic!("Expected ClsRef result"),
        }
    }

    #[test]
    fn test_egms() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();
        let result = evaluator.evaluate(r#""chair_model" egms"#, &props).unwrap();

        match result {
            EbaseResult::Egms { name } => {
                assert_eq!(name, "chair_model");
            }
            _ => panic!("Expected Egms result"),
        }
    }

    #[test]
    fn test_variable_substitution() {
        let mut evaluator = EbaseEvaluator::new();
        let mut props = HashMap::new();
        props.insert("M__BREITE".to_string(), 1600.0);

        let result = evaluator
            .evaluate(r#""panel" ${M__BREITE:-100} 1000 / 1 1 imp"#, &props)
            .unwrap();

        match result {
            EbaseResult::Import { filename, scale } => {
                assert_eq!(filename, "panel");
                assert!((scale[0] - 1.6).abs() < 0.001);
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_variable_default_value() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        let result = evaluator
            .evaluate(r#""panel" ${M__BREITE:-100} 1000 / 1 1 imp"#, &props)
            .unwrap();

        match result {
            EbaseResult::Import { filename, scale } => {
                assert_eq!(filename, "panel");
                assert!((scale[0] - 0.1).abs() < 0.001);
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_arithmetic() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        // Test with calculation
        let result = evaluator
            .evaluate(r#""test" 10 5 + 100 / 1 1 imp"#, &props)
            .unwrap();

        match result {
            EbaseResult::Import { scale, .. } => {
                assert!((scale[0] - 0.15).abs() < 0.001);
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_conditional_if() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        // True condition - should execute proc
        let result = evaluator
            .evaluate(r#"1 1 == { "true_case" } if 1 1 1 imp"#, &props)
            .unwrap();

        match result {
            EbaseResult::Import { filename, .. } => {
                assert_eq!(filename, "true_case");
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_conditional_ifelse() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        // False condition - should execute else proc
        let result = evaluator
            .evaluate(
                r#"1 2 == { "true_case" } { "false_case" } ifelse 1 1 1 imp"#,
                &props,
            )
            .unwrap();

        match result {
            EbaseResult::Import { filename, .. } => {
                assert_eq!(filename, "false_case");
            }
            _ => panic!("Expected Import result"),
        }
    }

    #[test]
    fn test_stack_underflow() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        let result = evaluator.evaluate("1 imp", &props);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_operator() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        let result = evaluator.evaluate("unknown_op", &props);
        assert!(result.is_err());
    }

    #[test]
    fn test_division_by_zero() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        let result = evaluator.evaluate("10 0 /", &props);
        assert!(matches!(result, Err(EbaseExprError::DivisionByZero)));
    }

    #[test]
    fn test_comparison_operators() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        // Less than
        evaluator.stack.clear();
        evaluator.evaluate("5 10 <", &props).unwrap();
        assert_eq!(evaluator.stack.pop(), Some(EbaseValue::Bool(true)));

        // Greater than
        evaluator.stack.clear();
        evaluator.evaluate("10 5 >", &props).unwrap();
        assert_eq!(evaluator.stack.pop(), Some(EbaseValue::Bool(true)));

        // Equal
        evaluator.stack.clear();
        evaluator.evaluate("5 5 ==", &props).unwrap();
        assert_eq!(evaluator.stack.pop(), Some(EbaseValue::Bool(true)));

        // Not equal
        evaluator.stack.clear();
        evaluator.evaluate("5 10 !=", &props).unwrap();
        assert_eq!(evaluator.stack.pop(), Some(EbaseValue::Bool(true)));
    }

    #[test]
    fn test_evaluate_to_number_simple() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        // Simple number
        assert!((evaluator.evaluate_to_number("0.5", &props).unwrap() - 0.5).abs() < 1e-10);
        assert!((evaluator.evaluate_to_number("-0.3", &props).unwrap() - (-0.3)).abs() < 1e-10);
        assert!((evaluator.evaluate_to_number("0", &props).unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_to_number_arithmetic() {
        let mut evaluator = EbaseEvaluator::new();
        let props = HashMap::new();

        // Division (common in OFML: 1000 1000 / = 1.0)
        assert!((evaluator.evaluate_to_number("1000 1000 /", &props).unwrap() - 1.0).abs() < 1e-10);

        // Multiplication
        assert!((evaluator.evaluate_to_number("0.5 2 *", &props).unwrap() - 1.0).abs() < 1e-10);

        // Addition
        assert!((evaluator.evaluate_to_number("0.3 0.2 +", &props).unwrap() - 0.5).abs() < 1e-10);

        // Negation
        assert!((evaluator.evaluate_to_number("0.5 neg", &props).unwrap() - (-0.5)).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_to_number_with_variables() {
        let mut evaluator = EbaseEvaluator::new();
        let mut props = HashMap::new();
        props.insert("M__BREITE".to_string(), 1600.0);
        props.insert("M__TIEFE".to_string(), 800.0);

        // Variable substitution with default
        let result = evaluator
            .evaluate_to_number("${M__BREITE:-1000} 1000 /", &props)
            .unwrap();
        assert!((result - 1.6).abs() < 1e-10);

        // Variable without value uses default
        let result2 = evaluator
            .evaluate_to_number("${UNKNOWN:-500} 1000 /", &props)
            .unwrap();
        assert!((result2 - 0.5).abs() < 1e-10);
    }
}
