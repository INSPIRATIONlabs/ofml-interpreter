//! Integration tests for CLS file processing.
//!
//! These tests verify the complete pipeline from CLS parsing through
//! scene graph generation.

use ofml_lib::{
    interpreter::Interpreter, parser::Parser, scene::SceneGraph, EbaseEvaluator, EbaseResult,
    GeometryTransform, OfmlClassInstance, OfmlClassRegistry, Value,
};
use std::collections::HashMap;

/// Test CLS class definition and instantiation
#[test]
fn test_cls_class_definition() {
    let code = r#"
        class TestPart : OiPart {
            var width = 1.6;
            var height = 0.74;

            func initialize() {
                // Simple part creation
            }

            func getWidth() {
                return width;
            }
        }

        var part = TestPart();
    "#;

    let mut parser = Parser::new(code).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    let mut interp = Interpreter::new();
    interp.execute(&ast).expect("Execution should succeed");

    // Verify the part was created
    let part = interp.env.get("part").expect("part should exist");
    assert!(matches!(part, Value::Object(_)));
}

/// Test CLS class with parameters
#[test]
fn test_cls_class_with_parameters() {
    let code = r#"
        class ConfigurableDesk : OiPart {
            var M__BREITE = 1600;
            var M__TIEFE = 800;
            var M__HOEHE = 740;

            func initialize() {
                // Initialize with default values
            }

            func getVolume() {
                return M__BREITE * M__TIEFE * M__HOEHE / 1000000000.0;
            }
        }

        var desk = ConfigurableDesk();
    "#;

    let mut parser = Parser::new(code).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    let mut interp = Interpreter::new();
    interp.execute(&ast).expect("Execution should succeed");

    let desk = interp.env.get("desk").expect("desk should exist");
    if let Value::Object(obj) = desk {
        let obj = obj.borrow();
        // Check default values were set
        assert_eq!(
            obj.fields.get("M__BREITE").and_then(|v| v.to_int()),
            Some(1600)
        );
        assert_eq!(
            obj.fields.get("M__TIEFE").and_then(|v| v.to_int()),
            Some(800)
        );
        assert_eq!(
            obj.fields.get("M__HOEHE").and_then(|v| v.to_int()),
            Some(740)
        );
    } else {
        panic!("desk should be an object");
    }
}

/// Test scene graph node creation from CLS
#[test]
fn test_scene_graph_from_cls() {
    let code = r#"
        class DeskAssembly : OiPart {
            func initialize() {
                // Create a desk with block geometry
                Block(self, @top, [1.6, 0.025, 0.8]);
            }
        }

        var desk = DeskAssembly();
    "#;

    let mut parser = Parser::new(code).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    let mut interp = Interpreter::new();
    interp.execute(&ast).expect("Execution should succeed");

    // The scene graph should have at least one node
    assert!(interp.scene.roots.len() > 0 || interp.scene.mesh_count() > 0);
}

/// Test OFML class instantiation from clsref result
#[test]
fn test_clsref_class_instantiation() {
    // Simulate evaluating a clsref expression
    let class_name = "::ofml::go::GoYLTrans";
    let params = vec![100.0, 200.0, 10.0];

    let result = OfmlClassRegistry::instantiate(class_name, &params);

    match result {
        Ok(OfmlClassInstance::Transform(GeometryTransform::YStretch(trans))) => {
            assert_eq!(trans.base_height, 100.0);
            assert_eq!(trans.target_height, 200.0);
            assert_eq!(trans.stretch_min, 10.0);
        }
        _ => panic!("Expected YStretch transform"),
    }
}

/// Test complete ctor evaluation to class instantiation pipeline
#[test]
fn test_ctor_to_class_instantiation() {
    let ctor = r#"${M__BREITE:-1000} 1000 / ${M__TIEFE:-600} 1000 / ${M__HOEHE:-740} 1000 / "::ofml::go::GoYLTrans" clsref"#;

    let mut props = HashMap::new();
    props.insert("M__BREITE".to_string(), 1600.0);
    props.insert("M__TIEFE".to_string(), 800.0);
    props.insert("M__HOEHE".to_string(), 740.0);

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(ctor, &props)
        .expect("Evaluation should succeed");

    match result {
        EbaseResult::ClsRef { class, params } => {
            assert_eq!(class, "::ofml::go::GoYLTrans");
            assert!((params[0] - 1.6).abs() < 0.01);
            assert!((params[1] - 0.8).abs() < 0.01);
            assert!((params[2] - 0.74).abs() < 0.01);

            // Now instantiate the class
            let instance = OfmlClassRegistry::instantiate(&class, &params);
            assert!(instance.is_ok());
        }
        _ => panic!("Expected ClsRef result"),
    }
}

/// Test nested class hierarchy
#[test]
fn test_nested_class_hierarchy() {
    let code = r#"
        class BasePart : OiPart {
            var baseValue = 100;
            func getBaseValue() {
                return baseValue;
            }
        }

        class ChildPart : BasePart {
            var color = "white";
            var childValue = 200;

            func getColor() {
                return color;
            }
        }

        var part = ChildPart();
    "#;

    let mut parser = Parser::new(code).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    let mut interp = Interpreter::new();
    interp.execute(&ast).expect("Execution should succeed");

    let part = interp.env.get("part").expect("part should exist");
    if let Value::Object(obj) = part {
        let obj = obj.borrow();
        // Child should have its own fields
        assert_eq!(
            obj.fields.get("color").map(|v| v.to_string_val()),
            Some("white".to_string())
        );
        assert_eq!(
            obj.fields.get("childValue").and_then(|v| v.to_int()),
            Some(200)
        );
    } else {
        panic!("part should be an object");
    }
}

/// Test property system integration
#[test]
fn test_property_system_in_cls() {
    let code = r#"
        class ConfiguredPart : OiPart {
            func initialize() {
                setupProperty(@width, ["Width", NULL, NULL, 0, "num 0.5 3.0"], 1.6);
                setupProperty(@height, ["Height", NULL, NULL, 0, "num 0.3 1.2"], 0.74);
            }
        }

        var part = ConfiguredPart();
    "#;

    let mut parser = Parser::new(code).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    let mut interp = Interpreter::new();
    interp.execute(&ast).expect("Execution should succeed");

    let part = interp.env.get("part").expect("part should exist");
    if let Value::Object(obj) = part {
        let obj = obj.borrow();
        // Properties should exist
        assert!(obj.properties.contains_key("width") || obj.fields.contains_key("width"));
    }
}

/// Test transform application with multiple transformations
#[test]
fn test_multiple_transform_classes() {
    // Test that we can instantiate multiple transform classes
    let transforms = vec![
        ("GoYLTrans", vec![100.0, 200.0, 10.0]),
        ("GoXLTrans", vec![50.0, 100.0, 5.0]),
        ("GoZLTrans", vec![30.0, 60.0, 3.0]),
        ("GoMirror", vec![0.0, 25.0]),
    ];

    for (class_name, params) in transforms {
        let result = OfmlClassRegistry::instantiate(class_name, &params);
        assert!(result.is_ok(), "Failed to instantiate {}", class_name);
    }
}

/// Test primitive geometry class instantiation
#[test]
fn test_primitive_geometry_classes() {
    // Test OiBlock
    let block = OfmlClassRegistry::instantiate("OiBlock", &[1.6, 0.025, 0.8]);
    assert!(block.is_ok());

    // Test OiCylinder
    let cylinder = OfmlClassRegistry::instantiate("OiCylinder", &[0.05, 0.74]);
    assert!(cylinder.is_ok());

    // Test OiSphere
    let sphere = OfmlClassRegistry::instantiate("OiSphere", &[0.5]);
    assert!(sphere.is_ok());

    // Test OiEllipsoid
    let ellipsoid = OfmlClassRegistry::instantiate("OiEllipsoid", &[0.3, 0.4, 0.5]);
    assert!(ellipsoid.is_ok());
}

/// Test error handling for unknown classes
#[test]
fn test_unknown_class_error() {
    let result = OfmlClassRegistry::instantiate("::unknown::NonExistentClass", &[1.0, 2.0]);
    assert!(result.is_err());
}

/// Test error handling for insufficient parameters
#[test]
fn test_insufficient_params_error() {
    // OiBlock requires 3 parameters
    let result = OfmlClassRegistry::instantiate("OiBlock", &[1.0, 2.0]);
    assert!(result.is_err());
}

/// Test scene graph hierarchy
#[test]
fn test_scene_graph_hierarchy() {
    let mut scene = SceneGraph::new();

    // Create a desk with parts
    let desk = scene.create_part("desk".to_string(), None);

    // Add table top
    let top = scene.create_block("top".to_string(), [1.6, 0.025, 0.8], Some(desk.clone()));
    top.borrow_mut().set_position([0.0, 0.74, 0.0]);

    // Add legs
    let leg1 = scene.create_cylinder("leg1".to_string(), 0.025, 0.74, Some(desk.clone()));
    leg1.borrow_mut().set_position([0.05, 0.0, 0.05]);

    let leg2 = scene.create_cylinder("leg2".to_string(), 0.025, 0.74, Some(desk.clone()));
    leg2.borrow_mut().set_position([1.55, 0.0, 0.05]);

    // Verify hierarchy
    assert!(scene.exists("desk.top"));
    assert!(scene.exists("desk.leg1"));
    assert!(scene.exists("desk.leg2"));
    assert_eq!(scene.mesh_count(), 3);
}

/// Test CLS with method calls
#[test]
fn test_cls_method_calls() {
    let code = r#"
        class Calculator : OiPart {
            var result = 0;

            func add(a, b) {
                result = a + b;
                return result;
            }

            func multiply(a, b) {
                result = a * b;
                return result;
            }

            func getResult() {
                return result;
            }
        }

        var calc = Calculator();
        var sum = calc.add(10, 20);
        var product = calc.multiply(5, 6);
    "#;

    let mut parser = Parser::new(code).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    let mut interp = Interpreter::new();
    interp.execute(&ast).expect("Execution should succeed");

    let sum = interp.env.get("sum").expect("sum should exist");
    assert_eq!(sum.to_int(), Some(30));

    let product = interp.env.get("product").expect("product should exist");
    assert_eq!(product.to_int(), Some(30));
}
