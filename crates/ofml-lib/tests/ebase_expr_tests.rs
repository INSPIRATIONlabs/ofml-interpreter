//! Integration tests for EBASE expression evaluation.
//!
//! These tests verify the complete expression evaluation pipeline.

use ofml_lib::ebase_expr::{EbaseEvaluator, EbaseResult};
use std::collections::HashMap;

/// Test basic expression evaluation (T034)
#[test]
fn test_basic_expression_evaluation() {
    // Test a simple arithmetic expression
    let expr = "10 20 +";
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(expr, &vars);

    assert!(result.is_ok(), "Evaluation should succeed");
}

/// Test import expression
#[test]
fn test_import_expression() {
    // Format: "filename" sx sy sz imp
    let expr = r#""geometry/desk.3ds" 1.0 1.0 1.0 imp"#;
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(expr, &vars)
        .expect("Evaluation should succeed");

    match result {
        EbaseResult::Import { filename, scale } => {
            assert_eq!(filename, "geometry/desk.3ds");
            assert!((scale[0] - 1.0).abs() < 0.01);
            assert!((scale[1] - 1.0).abs() < 0.01);
            assert!((scale[2] - 1.0).abs() < 0.01);
        }
        _ => panic!("Expected Import result, got {:?}", result),
    }
}

/// Test scaled import expression
#[test]
fn test_scaled_import_expression() {
    // Format: "filename" sx sy sz imp
    let expr = r#""geometry/desk.3ds" 1.8 0.72 0.9 imp"#;
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(expr, &vars)
        .expect("Evaluation should succeed");

    match result {
        EbaseResult::Import { filename, scale } => {
            assert_eq!(filename, "geometry/desk.3ds");
            assert!((scale[0] - 1.8).abs() < 0.01);
            assert!((scale[1] - 0.72).abs() < 0.01);
            assert!((scale[2] - 0.9).abs() < 0.01);
        }
        _ => panic!("Expected Import result"),
    }
}

/// Test variable substitution
#[test]
fn test_variable_substitution() {
    let expr = "${WIDTH:-1000} 1000 /";
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("WIDTH".to_string(), 1800.0);

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(expr, &vars);
    assert!(result.is_ok(), "Variable substitution should succeed");
}

/// Test default values for missing variables
#[test]
fn test_default_values() {
    let expr = "${MISSING:-500} 1000 /";
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(expr, &vars);
    assert!(
        result.is_ok(),
        "Default values should work for missing variables"
    );
}

/// Test clsref operator
#[test]
fn test_clsref_operator() {
    // Format: params... "ClassName" clsref
    let expr = r#"1.0 0.5 0.8 "components/leg.cls" clsref"#;
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(expr, &vars)
        .expect("Evaluation should succeed");

    match result {
        EbaseResult::ClsRef { class, params } => {
            assert_eq!(class, "components/leg.cls");
            assert_eq!(params.len(), 3);
            assert!((params[0] - 1.0).abs() < 0.01);
            assert!((params[1] - 0.5).abs() < 0.01);
            assert!((params[2] - 0.8).abs() < 0.01);
        }
        _ => panic!("Expected ClsRef result"),
    }
}

/// Test egms operator
#[test]
fn test_egms_operator() {
    let expr = r#""table_top" egms"#;
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(expr, &vars)
        .expect("Evaluation should succeed");

    match result {
        EbaseResult::Egms { name } => {
            assert_eq!(name, "table_top");
        }
        _ => panic!("Expected Egms result"),
    }
}

/// Test arithmetic operations
#[test]
fn test_arithmetic_operations() {
    // Addition
    let mut evaluator = EbaseEvaluator::new();
    let vars: HashMap<String, f64> = HashMap::new();
    let _ = evaluator.evaluate("10 5 +", &vars);

    // Subtraction
    let _ = evaluator.evaluate("10 5 -", &vars);

    // Multiplication
    let _ = evaluator.evaluate("10 5 *", &vars);

    // Division
    let _ = evaluator.evaluate("10 5 /", &vars);

    // Negation
    let _ = evaluator.evaluate("10 neg", &vars);
}

/// Test comparison operators
#[test]
fn test_comparison_operators() {
    let mut evaluator = EbaseEvaluator::new();
    let vars: HashMap<String, f64> = HashMap::new();

    // Equal
    let _ = evaluator.evaluate("10 10 ==", &vars);

    // Not equal
    let _ = evaluator.evaluate("10 5 !=", &vars);

    // Less than
    let _ = evaluator.evaluate("5 10 <", &vars);

    // Greater than
    let _ = evaluator.evaluate("10 5 >", &vars);

    // Less than or equal
    let _ = evaluator.evaluate("10 10 <=", &vars);

    // Greater than or equal
    let _ = evaluator.evaluate("10 10 >=", &vars);
}

/// Test conditional execution with comparison producing boolean
#[test]
fn test_conditional_if() {
    // condition { proc } if - use comparison for boolean
    let expr = "1 1 == { 42 } if";
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(expr, &vars);
    assert!(
        result.is_ok(),
        "If conditional should succeed: {:?}",
        result
    );
}

/// Test conditional ifelse with comparison producing boolean
#[test]
fn test_conditional_ifelse() {
    // condition { then } { else } ifelse - use comparison for boolean
    let expr = "1 1 == { 42 } { 0 } ifelse";
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(expr, &vars);
    assert!(
        result.is_ok(),
        "Ifelse conditional should succeed: {:?}",
        result
    );
}

/// Test logical operators
#[test]
fn test_logical_operators() {
    let mut evaluator = EbaseEvaluator::new();
    let vars: HashMap<String, f64> = HashMap::new();

    // And - need booleans from comparisons
    let _ = evaluator.evaluate("1 1 == 2 2 == and", &vars);

    // Or - need booleans from comparisons
    let _ = evaluator.evaluate("1 1 == 1 2 == or", &vars);

    // Not - need boolean from comparison
    let _ = evaluator.evaluate("1 2 == not", &vars);
}

/// Test stack operations
#[test]
fn test_stack_operations() {
    let mut evaluator = EbaseEvaluator::new();
    let vars: HashMap<String, f64> = HashMap::new();

    // Dup
    let _ = evaluator.evaluate("5 dup +", &vars);

    // Pop
    let _ = evaluator.evaluate("5 10 pop", &vars);

    // Exch
    let _ = evaluator.evaluate("5 10 exch -", &vars);
}

/// Test None result (no geometry)
#[test]
fn test_none_result() {
    let expr = "5 10 +"; // Arithmetic only, no geometry operation
    let vars: HashMap<String, f64> = HashMap::new();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(expr, &vars)
        .expect("Evaluation should succeed");

    // Result should be None (no geometry operation)
    assert!(matches!(result, EbaseResult::None));
}

/// Test complex expression with variables
#[test]
fn test_complex_expression_with_variables() {
    // Format: "filename" sx sy sz imp (with variable calculations)
    let expr = r#""geometry/desk.3ds" ${WIDTH:-1600} 1000 / ${HEIGHT:-750} 1000 / ${DEPTH:-800} 1000 / imp"#;

    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("WIDTH".to_string(), 1800.0);
    vars.insert("HEIGHT".to_string(), 720.0);
    vars.insert("DEPTH".to_string(), 900.0);

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(expr, &vars)
        .expect("Evaluation should succeed");

    match result {
        EbaseResult::Import { filename, scale } => {
            assert_eq!(filename, "geometry/desk.3ds");
            assert!((scale[0] - 1.8).abs() < 0.01);
            assert!((scale[1] - 0.72).abs() < 0.01);
            assert!((scale[2] - 0.9).abs() < 0.01);
        }
        _ => panic!("Expected Import result"),
    }
}
