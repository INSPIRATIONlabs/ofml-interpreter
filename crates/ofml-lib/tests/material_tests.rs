//! Integration tests for material system.
//!
//! These tests verify material loading and resolution.

use ofml_lib::material::{
    MatParser, MaterialDef, MaterialResolver, TextureDef, TextureProjection,
};

/// Test parsing standard MAT file format
#[test]
fn test_mat_file_parsing() {
    let mat_content = r#"
        amb 0.1 0.1 0.1 1.0
        dif 0.8 0.6 0.4 1.0
        spe 0.5 0.5 0.5 1.0
        shi 32
        tra 0.0
    "#;

    let result = MatParser::parse(mat_content, "test_material");
    assert!(result.is_ok(), "MAT content should parse");

    let material = result.unwrap();

    // Check diffuse - primary color component
    assert!((material.diffuse[0] - 0.8).abs() < 0.01);
    assert!((material.diffuse[1] - 0.6).abs() < 0.01);
    assert!((material.diffuse[2] - 0.4).abs() < 0.01);
}

/// Test inline material definition parsing ($ syntax)
#[test]
fn test_inline_material_parsing() {
    let inline = "$ amb 0.2 0.2 0.2 1.0; dif 0.7 0.5 0.3 1.0";

    let result = MatParser::parse_inline(inline, "inline_test");
    assert!(result.is_ok(), "Inline material should parse: {:?}", result);

    let material = result.unwrap();
    assert!((material.diffuse[0] - 0.7).abs() < 0.01);
}

/// Test texture projection modes
#[test]
fn test_texture_projection_modes() {
    // All projection modes should be constructible
    let modes = [
        TextureProjection::ProjectX,
        TextureProjection::ProjectY,
        TextureProjection::ProjectZ,
        TextureProjection::Cylindrical,
        TextureProjection::Spherical,
        TextureProjection::Conical,
        TextureProjection::Circle,
    ];

    for mode in modes {
        // Just verify they can be created
        let _ = format!("{:?}", mode);
    }
}

/// Test texture definition
#[test]
fn test_texture_definition() {
    let texture = TextureDef::new("wood.png")
        .with_projection(TextureProjection::ProjectY)
        .with_scale(2.0, 2.0)
        .with_offset(0.5, 0.5)
        .with_rotation(45.0);

    assert_eq!(texture.filename, "wood.png");
    assert!(matches!(texture.projection, TextureProjection::ProjectY));
    assert!((texture.u_scale - 2.0).abs() < 0.01);
    assert!((texture.u_offset - 0.5).abs() < 0.01);
    assert!((texture.rotation - 45.0).abs() < 0.01);
}

/// Test material resolver caching
#[test]
fn test_material_resolver_caching() {
    let mut resolver = MaterialResolver::new();

    // Create and register a material
    let material = MaterialDef::new("grey").with_diffuse(0.7, 0.7, 0.7, 1.0);

    resolver.register(material);

    // Resolve the material
    let resolved = resolver.resolve("grey");
    assert!((resolved.diffuse[0] - 0.7).abs() < 0.01);

    // Check material exists
    assert!(resolver.contains("grey"));
}

/// Test fallback material for unknowns
#[test]
fn test_fallback_material() {
    let resolver = MaterialResolver::new();

    // Resolve unknown material - should return fallback
    let resolved = resolver.resolve("unknown_material");

    // Fallback is magenta (0.8, 0.0, 0.8) for visibility
    assert!((resolved.diffuse[0] - 0.8).abs() < 0.01);
    assert!((resolved.diffuse[1] - 0.0).abs() < 0.01);
    assert!((resolved.diffuse[2] - 0.8).abs() < 0.01);
}

/// Test resolve_or_create for color-based materials
#[test]
fn test_resolve_or_create() {
    let resolver = MaterialResolver::new();

    // Resolve with color name should create material
    let material = resolver.resolve_or_create("::test::material::grey");

    // Should have grey-ish diffuse color
    let avg = (material.diffuse[0] + material.diffuse[1] + material.diffuse[2]) / 3.0;
    assert!(
        avg > 0.3 && avg < 0.7,
        "Grey material should have mid-range color"
    );
}

/// Test MaterialDef builder pattern
#[test]
fn test_material_def_builder() {
    let material = MaterialDef::new("test_material")
        .with_ambient(0.1, 0.1, 0.1, 1.0)
        .with_diffuse(0.8, 0.6, 0.4, 1.0)
        .with_specular(0.5, 0.5, 0.5, 1.0)
        .with_shininess(32.0);

    assert_eq!(material.name, "test_material");
    assert!((material.diffuse[0] - 0.8).abs() < 0.01);
    assert!((material.shininess - 32.0).abs() < 0.01);
}

/// Test material from color name
#[test]
fn test_material_from_color_name() {
    // Common color names should produce appropriate materials
    let color_tests = [("grey", 0.5), ("white", 0.9), ("black", 0.1)];

    for (name, expected_brightness) in color_tests {
        let material = MaterialDef::from_color_name(name);
        let avg = (material.diffuse[0] + material.diffuse[1] + material.diffuse[2]) / 3.0;
        assert!(
            (avg - expected_brightness).abs() < 0.3,
            "Color '{}' brightness mismatch: got {}, expected ~{}",
            name,
            avg,
            expected_brightness
        );
    }
}

/// Test material names collection
#[test]
fn test_material_names() {
    let mut resolver = MaterialResolver::new();

    resolver.register(MaterialDef::new("material_a"));
    resolver.register(MaterialDef::new("material_b"));
    resolver.register(MaterialDef::new("material_c"));

    let names = resolver.names();
    assert_eq!(names.len(), 3);
}
