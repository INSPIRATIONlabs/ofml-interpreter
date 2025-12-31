//! OFML Interpreter - A Rust implementation of the Office Furniture Modeling Language
//!
//! This interpreter implements the OFML 2.0 specification for parsing and executing
//! CLS (Class) files used in office furniture configuration systems.
//!
//! ## Architecture
//!
//! - `lexer`: Tokenizes OFML source code
//! - `ast`: Abstract Syntax Tree types
//! - `parser`: Recursive descent parser
//! - `interpreter`: Runtime execution engine
//! - `geometry`: 3D geometry and transform handling
//! - `ebase_expr`: EBASE PostScript-like expression evaluator
//! - `gobject`: GObject type system
//! - `property`: Property system for product configuration
//! - `ofml_classes`: OFML framework classes (Go*, Oi*)
//! - `material`: Material system (MAT files, textures)
//! - `geometry2d`: 2D representation for floor plans
//! - `attachment`: Attachment points system

// Core modules (existing)
pub mod ast;
pub mod ebase;
pub mod env;
pub mod geometry;
pub mod interpreter;
pub mod lexer;
pub mod ofml;
pub mod parser;
pub mod scene;
pub mod value;

// New modules for OFML CLS interpreter completion
pub mod alb_loader;
pub mod article;
pub mod attachment;
pub mod ebase_expr;
pub mod errors;
pub mod geometry2d;
pub mod gobject;
pub mod material;
pub mod ofml_classes;
pub mod operations;
pub mod property;
pub mod texture;
pub mod xoi_framework;

// OAP Configurator modules
pub mod oap;
#[cfg(feature = "tui")]
pub mod tui;

// Re-exports for convenient access
pub use env::{Environment, Scope};
pub use interpreter::Interpreter;
pub use lexer::{tokenize, LexError, SpannedToken, Token};
pub use scene::{Axis, Geometry, SceneGraph, SceneNode};
pub use value::{ClassValue, FuncValue, ObjInstance, PropertyDef, Value};

// New re-exports
pub use article::{ArticleConfig, ArticleLoader, PropertyValue, Variant, VariantGroup};
pub use article::{DIM_DEPTH, DIM_HEIGHT, DIM_WIDTH, MAT_BASIC, MAT_COLOR_PREFIX};
pub use attachment::{
    load_all_attachments, load_attachment_from_record, load_attpt_table, load_oppattpt_table,
    load_stdattpt_table, AttachmentPoint, AttachmentPointBuilder, AttachmentPointSet,
    AttachmentType,
};
pub use ebase::{read_ocd, read_odb2d, OcdRecord, Odb2dRecord};
pub use ebase_expr::{EbaseEvaluator, EbaseResult, EbaseToken, EbaseValue};
pub use errors::{
    ArticleError, AttachmentError, EbaseExprError, GObjectError, Geometry2DError, MaterialError,
    OfmlClassError, PropertyError,
};
pub use geometry2d::{
    odb2d_to_attributes, odb2d_to_primitive, process_odb2d_records, G2DAttributes, G2DCompound,
    G2DPrimitive, Transform2D,
};
pub use gobject::GValue;
pub use material::{MaterialDef, TextureDef, TextureProjection};
pub use ofml_classes::{
    GeometryTransform, GoMirrorParams, GoXLRTransYLRTransParams, GoXLTransParams, GoYLTransParams,
    GoZLTransParams, MirrorAxis, OfmlClassInstance, OfmlClassRegistry, OfmlClassType,
    PrimitiveGeometry,
};
pub use property::{PropertyState, PropertyType};
pub use texture::{TextureCache, TextureData};

// Operations - high-level reusable functions
pub use operations::{
    apply_transforms, assemble_product, evaluate_expression, export_2d_floorplan, export_to_glb,
    load_and_merge_geometry, load_geometry_data, load_geometry_file, merge_scenes,
    validate_geometry, OperationError, ProductConfig, ProductResult, ValidationResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create and run OFML code
    fn run_ofml(code: &str) -> Interpreter {
        let mut parser = parser::Parser::new(code).expect("Failed to create parser");
        let ast = parser.parse().expect("Failed to parse");
        let mut interp = Interpreter::new();
        interp.execute(&ast).expect("Failed to execute");
        interp
    }

    #[test]
    fn test_property_system_basic() {
        let code = r#"
            class TestObj : OiPart {
                func initialize() {
                    setupProperty(@myProp, ["Label", NULL, NULL, 0, "ch @opt1 @opt2"], 1);
                    setPropValue(@myProp, @opt1);
                }
            }
            var obj = TestObj();
        "#;

        let interp = run_ofml(code);

        // Check that object was created
        let obj = interp.env.get("obj").expect("obj should exist");
        if let Value::Object(instance) = obj {
            let inst = instance.borrow();
            // Check property was set
            assert!(
                inst.properties.contains_key("myProp"),
                "myProp should exist"
            );
            // Check property definition was created
            assert!(
                inst.prop_defs.contains_key("myProp"),
                "property definition should exist"
            );
            let prop_def = &inst.prop_defs["myProp"];
            assert_eq!(prop_def.sort_order, 1);
        } else {
            panic!("obj should be an Object");
        }
    }

    #[test]
    fn test_get_set_prop_value() {
        let code = r#"
            class TestObj : OiPart {
                func initialize() {
                    setPropValue(@color, @red);
                }
                func getColor() {
                    return getPropValue(@color);
                }
            }
            var obj = TestObj();
            var color = obj.getColor();
        "#;

        let interp = run_ofml(code);

        let color = interp.env.get("color").expect("color should exist");
        if let Value::Symbol(s) = color {
            assert_eq!(s.as_str(), "red");
        } else {
            panic!("color should be a Symbol");
        }
    }

    #[test]
    fn test_prop_state() {
        let code = r#"
            class TestObj : OiPart {
                func initialize() {
                    setupProperty(@myProp, ["Label", NULL, NULL, 0, NULL], 1);
                    setPropState(@myProp, 0);
                }
            }
            var obj = TestObj();
        "#;

        let interp = run_ofml(code);

        let obj = interp.env.get("obj").expect("obj should exist");
        if let Value::Object(instance) = obj {
            let inst = instance.borrow();
            assert_eq!(inst.prop_states.get("myProp"), Some(&0));
        } else {
            panic!("obj should be an Object");
        }
    }

    #[test]
    fn test_get_property_keys() {
        let code = r#"
            class TestObj : OiPart {
                func initialize() {
                    setPropValue(@prop1, 10);
                    setPropValue(@prop2, 20);
                }
                func getKeys() {
                    return getPropertyKeys();
                }
            }
            var obj = TestObj();
            var keys = obj.getKeys();
        "#;

        let interp = run_ofml(code);

        let keys = interp.env.get("keys").expect("keys should exist");
        if let Value::Array(arr) = keys {
            let arr = arr.borrow();
            assert_eq!(arr.len(), 2);
        } else {
            panic!("keys should be an Array");
        }
    }

    #[test]
    fn test_oi_block_creation() {
        let code = r#"
            class TestPart : OiPart {
                func initialize() {
                    OiBlock(self, @geo, [0.1, 0.2, 0.3]);
                }
            }
            var part = TestPart();
        "#;

        let interp = run_ofml(code);

        // Check that the scene has a block
        assert!(
            interp.scene.mesh_count() >= 1,
            "Scene should have at least one mesh"
        );
    }

    #[test]
    fn test_set_material() {
        let code = r#"
            class TestBlock : OiPart {
                func initialize() {
                    OiBlock(self, @geo, [1, 1, 1]);
                    setMaterial("::test::material::grey");
                }
            }
            var block = TestBlock();
        "#;

        let interp = run_ofml(code);

        let block = interp.env.get("block").expect("block should exist");
        if let Value::Object(instance) = block {
            let inst = instance.borrow();
            let mat = inst.properties.get("material");
            assert!(mat.is_some(), "material property should exist");
            if let Some(Value::String(s)) = mat {
                assert!(s.contains("grey"));
            }
        } else {
            panic!("block should be an Object");
        }
    }

    #[test]
    fn test_get_pd_manager() {
        let code = r#"
            var pdm = getPDManager();
            var pdb = pdm.getProductDB(@test_product);
        "#;

        let interp = run_ofml(code);

        // Check PDManager was created
        let pdm = interp.env.get("pdm").expect("pdm should exist");
        if let Value::Object(instance) = pdm {
            assert_eq!(instance.borrow().class.name, "PDManager");
        } else {
            panic!("pdm should be an Object");
        }

        // Check ProductDB was created
        let pdb = interp.env.get("pdb").expect("pdb should exist");
        if let Value::Object(instance) = pdb {
            assert_eq!(instance.borrow().class.name, "ProductDB");
        } else {
            panic!("pdb should be an Object");
        }
    }

    #[test]
    fn test_math_functions() {
        let code = r#"
            var a = fabs(-5.5);
            var m = Mod(10.5, 3);  // [3, 1.5]
        "#;

        let interp = run_ofml(code);

        let a = interp.env.get("a").expect("a should exist");
        if let Value::Float(f) = a {
            assert!((f - 5.5).abs() < 0.001);
        } else {
            panic!("a should be a Float");
        }

        let m = interp.env.get("m").expect("m should exist");
        if let Value::Array(arr) = m {
            let arr = arr.borrow();
            assert_eq!(arr.len(), 2);
            if let Value::Float(quot) = &arr[0] {
                assert!((quot - 3.0).abs() < 0.001);
            }
        } else {
            panic!("m should be an Array");
        }
    }

    #[test]
    fn test_remove_property() {
        let code = r#"
            class TestObj : OiPart {
                func initialize() {
                    setPropValue(@myProp, 42);
                    removeProperty(@myProp);
                }
            }
            var obj = TestObj();
        "#;

        let interp = run_ofml(code);

        let obj = interp.env.get("obj").expect("obj should exist");
        if let Value::Object(instance) = obj {
            let inst = instance.borrow();
            assert!(
                !inst.properties.contains_key("myProp"),
                "myProp should be removed"
            );
        } else {
            panic!("obj should be an Object");
        }
    }

    #[test]
    fn test_scene_graph_material_color() {
        // Test that material colors are derived from material names
        use scene::SceneGraph;

        let mut sg = SceneGraph::new();
        let node = sg.create_block("test".to_string(), [1.0, 1.0, 1.0], None);
        node.borrow_mut().material = "::test::material::grey".to_string();

        let scene3ds = sg.to_scene();

        // Check that material was created
        assert!(scene3ds.materials.contains_key("::test::material::grey"));
        let mat = &scene3ds.materials["::test::material::grey"];
        // Grey material should have grey-ish colors
        assert!((mat.diffuse[0] - 0.5).abs() < 0.1);
        assert!((mat.diffuse[1] - 0.5).abs() < 0.1);
        assert!((mat.diffuse[2] - 0.5).abs() < 0.1);
    }
}
