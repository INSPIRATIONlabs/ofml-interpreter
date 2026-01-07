//! End-to-end integration tests for the OFML interpreter.
//!
//! These tests verify the complete pipeline from article configuration
//! through expression evaluation, geometry generation, and GLB export.

use ofml_lib::{
    article::{ArticleConfig, ArticleLoader},
    ebase_expr::{EbaseEvaluator, EbaseResult},
    geometry::{
        scene_to_glb, CoordSystem, EmbeddedTexture, Face, Material3DS, Mesh, Scene3DS, Vertex,
    },
    material::{MaterialDef, MaterialResolver},
    scene::SceneGraph,
    texture::create_solid_color,
};
use std::collections::HashMap;

/// Test complete pipeline: Article -> Expression -> Geometry
#[test]
fn test_full_article_to_geometry_pipeline() {
    // 1. Create article configuration
    let loader = ArticleLoader::new();
    let article = loader.create_desk_article("DESK1600x800", 1600, 800, 740);

    assert_eq!(article.get_int("M__BREITE", 0), 1600);
    assert_eq!(article.get_int("M__TIEFE", 0), 800);

    // 2. Evaluate EBASE expression with article properties
    let ctor = r#""desk_geometry" ${M__BREITE:-1000} 1000 / ${M__TIEFE:-600} 1000 / ${M__HOEHE:-740} 1000 / imp"#;
    let props = article.to_f64_map();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator
        .evaluate(ctor, &props)
        .expect("Expression should evaluate");

    // 3. Verify expression result
    match result {
        EbaseResult::Import { filename, scale } => {
            assert_eq!(filename, "desk_geometry");
            assert!((scale[0] - 1.6).abs() < 0.01);
            assert!((scale[1] - 0.8).abs() < 0.01);
            assert!((scale[2] - 0.74).abs() < 0.01);
        }
        _ => panic!("Expected Import result"),
    }
}

/// Test material resolution with article properties
#[test]
fn test_material_resolution_with_article() {
    // Configure article with material reference
    let mut article = ArticleConfig::new("test", "test");
    article.set("SH__BASIC", "::egr::aci::ACI5");

    // Resolve material
    let resolver = MaterialResolver::new();
    let material_name = article.get_string("SH__BASIC", "default");

    // Material resolver should return a material (or fallback)
    let resolved = resolver.resolve(&material_name);
    assert!(!resolved.name.is_empty());
}

/// Test scene graph to GLB export
#[test]
fn test_scene_to_glb_export() {
    let mut scene_graph = SceneGraph::new();

    // Create a simple desk structure
    let desk = scene_graph.create_part("desk".to_string(), None);

    // Add desk top
    let top = scene_graph.create_block("top".to_string(), [1.6, 0.025, 0.8], Some(desk.clone()));
    top.borrow_mut().set_position([0.0, 0.74, 0.0]);
    top.borrow_mut().material = "oak_wood".to_string();

    // Convert to Scene3DS
    let scene3ds = scene_graph.to_scene();

    // Export to GLB
    let glb = scene_to_glb(&scene3ds).expect("GLB export should succeed");

    // Verify GLB structure
    assert!(glb.len() > 12);
    assert_eq!(&glb[0..4], b"glTF");
}

/// Test complete desk assembly with materials
#[test]
fn test_desk_assembly_complete() {
    let mut scene_graph = SceneGraph::new();

    // Create desk structure
    let desk = scene_graph.create_part("desk".to_string(), None);
    desk.borrow_mut().set_position([0.0, 0.0, 0.0]);

    // Table top
    let top = scene_graph.create_block("top".to_string(), [1.6, 0.025, 0.8], Some(desk.clone()));
    top.borrow_mut().set_position([0.0, 0.74, 0.0]);
    top.borrow_mut().material = "wood_oak".to_string();

    // Four legs
    let leg_radius = 0.025;
    let leg_height = 0.74;
    let leg_positions = [
        [0.05, 0.0, 0.05],
        [1.55, 0.0, 0.05],
        [0.05, 0.0, 0.75],
        [1.55, 0.0, 0.75],
    ];

    for (i, pos) in leg_positions.iter().enumerate() {
        let leg = scene_graph.create_cylinder(
            format!("leg{}", i + 1),
            leg_radius,
            leg_height,
            Some(desk.clone()),
        );
        leg.borrow_mut().set_position(*pos);
        leg.borrow_mut().material = "metal_chrome".to_string();
    }

    // Verify structure
    assert!(scene_graph.exists("desk.top"));
    assert!(scene_graph.exists("desk.leg1"));
    assert!(scene_graph.exists("desk.leg2"));
    assert!(scene_graph.exists("desk.leg3"));
    assert!(scene_graph.exists("desk.leg4"));
    assert_eq!(scene_graph.mesh_count(), 5);

    // Export and verify
    let scene3ds = scene_graph.to_scene();
    assert_eq!(scene3ds.meshes.len(), 5);

    let glb = scene_to_glb(&scene3ds).expect("GLB export should succeed");
    assert!(glb.len() > 1000); // Should be substantial
}

/// Test variant selection affecting geometry
#[test]
fn test_variant_selection_geometry() {
    use ofml_lib::article::{Variant, VariantGroup};

    // Define size variants
    let mut sizes = VariantGroup::new("size", "Desk Size");
    sizes.add_variant(
        Variant::new("small", "Small (1200x600)")
            .with_property("M__BREITE", 1200i64)
            .with_property("M__TIEFE", 600i64),
    );
    sizes.add_variant(
        Variant::new("medium", "Medium (1600x800)")
            .with_property("M__BREITE", 1600i64)
            .with_property("M__TIEFE", 800i64),
    );
    sizes.add_variant(
        Variant::new("large", "Large (1800x900)")
            .with_property("M__BREITE", 1800i64)
            .with_property("M__TIEFE", 900i64),
    );

    // Select variant
    sizes.select("large").expect("Selection should succeed");
    let selected = sizes.get_selected().expect("Should have selection");

    // Apply variant to article
    let mut article = ArticleConfig::new("desk", "desk_odb");
    for (name, value) in &selected.properties {
        article.set(name.clone(), value.clone());
    }

    // Verify variant applied
    assert_eq!(article.get_int("M__BREITE", 0), 1800);
    assert_eq!(article.get_int("M__TIEFE", 0), 900);

    // Evaluate expression with variant properties
    let ctor = r#""desk" ${M__BREITE:-1000} 1000 / ${M__TIEFE:-600} 1000 / 1 imp"#;
    let props = article.to_f64_map();

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(ctor, &props).expect("Should evaluate");

    match result {
        EbaseResult::Import { scale, .. } => {
            assert!((scale[0] - 1.8).abs() < 0.01);
            assert!((scale[1] - 0.9).abs() < 0.01);
        }
        _ => panic!("Expected Import result"),
    }
}

/// Test GLB with embedded texture
#[test]
fn test_glb_with_texture() {
    // Create texture
    let wood_texture =
        create_solid_color("wood_texture", 139, 90, 43, 255).expect("Should create texture");

    // Create simple mesh
    let mesh = Mesh {
        name: "box".to_string(),
        vertices: vec![
            Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vertex {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vertex {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Vertex {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ],
        normals: Vec::new(),
        faces: vec![
            Face {
                a: 0,
                b: 1,
                c: 2,
                flags: 0,
            },
            Face {
                a: 0,
                b: 2,
                c: 3,
                flags: 0,
            },
        ],
        tex_coords: Vec::new(),
        material_name: Some("wood_mat".to_string()),
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    };

    // Create material with texture reference
    let mut materials = HashMap::new();
    materials.insert(
        "wood_mat".to_string(),
        Material3DS {
            name: "wood_mat".to_string(),
            ambient: [0.1, 0.1, 0.1],
            diffuse: [0.8, 0.6, 0.4],
            specular: [0.3, 0.3, 0.3],
            texture: Some("wood_texture".to_string()),
            metallic: 0.0,
            roughness: 0.7,
        },
    );

    // Create scene with texture
    let scene = Scene3DS {
        meshes: vec![mesh],
        materials,
        textures: vec![EmbeddedTexture {
            name: wood_texture.name.clone(),
            data: wood_texture.png_data.clone(),
            width: wood_texture.width,
            height: wood_texture.height,
        }],
    };

    // Export to GLB
    let glb = scene_to_glb(&scene).expect("GLB export should succeed");

    // Verify GLB is valid
    assert!(glb.len() > 100);
    assert_eq!(&glb[0..4], b"glTF");

    // GLB should be larger due to embedded texture
    assert!(glb.len() > 200);
}

/// Test conditional geometry based on article properties
#[test]
fn test_conditional_geometry_expression() {
    let mut evaluator = EbaseEvaluator::new();

    // Test conditional: if width > 1500, use large geometry
    let ctor = r#"${WIDTH:-1000} 1500 > { "desk_large" } { "desk_small" } ifelse 1 1 1 imp"#;

    // Width = 1600 (> 1500)
    let mut props = HashMap::new();
    props.insert("WIDTH".to_string(), 1600.0);

    let result = evaluator.evaluate(ctor, &props).expect("Should evaluate");
    match result {
        EbaseResult::Import { filename, .. } => {
            assert_eq!(filename, "desk_large");
        }
        _ => panic!("Expected Import result"),
    }

    // Width = 1200 (< 1500)
    props.insert("WIDTH".to_string(), 1200.0);
    let result = evaluator.evaluate(ctor, &props).expect("Should evaluate");
    match result {
        EbaseResult::Import { filename, .. } => {
            assert_eq!(filename, "desk_small");
        }
        _ => panic!("Expected Import result"),
    }
}

/// Test clsref result with parameters
#[test]
fn test_clsref_with_article_params() {
    let mut article = ArticleConfig::new("test", "test");
    article.set("M__BREITE", 1600i64);
    article.set("M__TIEFE", 800i64);
    article.set("M__HOEHE", 740i64);

    let props = article.to_f64_map();

    let ctor = r#"${M__BREITE:-1000} 1000 / ${M__TIEFE:-600} 1000 / ${M__HOEHE:-740} 1000 / "::ofml::go::GoYLTrans" clsref"#;

    let mut evaluator = EbaseEvaluator::new();
    let result = evaluator.evaluate(ctor, &props).expect("Should evaluate");

    match result {
        EbaseResult::ClsRef { class, params } => {
            assert_eq!(class, "::ofml::go::GoYLTrans");
            assert_eq!(params.len(), 3);
            assert!((params[0] - 1.6).abs() < 0.01);
            assert!((params[1] - 0.8).abs() < 0.01);
            assert!((params[2] - 0.74).abs() < 0.01);
        }
        _ => panic!("Expected ClsRef result"),
    }
}

/// Test complete material workflow
#[test]
fn test_material_workflow() {
    let mut resolver = MaterialResolver::new();

    // Register custom material
    let wood_mat = MaterialDef::new("wood_oak")
        .with_diffuse(0.6, 0.4, 0.2, 1.0)
        .with_specular(0.3, 0.3, 0.3, 1.0)
        .with_shininess(32.0);

    resolver.register(wood_mat);

    // Resolve by name
    let resolved = resolver.resolve("wood_oak");
    assert!((resolved.diffuse[0] - 0.6).abs() < 0.01);

    // Fallback for unknown materials
    let unknown = resolver.resolve("unknown_material");
    assert!((unknown.diffuse[0] - 0.8).abs() < 0.01); // Magenta fallback
}
