//! Integration tests for texture loading system.
//!
//! These tests verify texture loading, caching, and GLB integration.

use ofml_lib::geometry::{scene_to_glb, EmbeddedTexture, Material3DS, Scene3DS};
use ofml_lib::texture::{create_checkerboard, create_solid_color, TextureCache};
use std::collections::HashMap;

/// Test creating solid color textures
#[test]
fn test_solid_color_texture() {
    let tex = create_solid_color("red", 255, 0, 0, 255).unwrap();
    assert_eq!(tex.name, "red");
    assert_eq!(tex.width, 1);
    assert_eq!(tex.height, 1);
    assert!(!tex.png_data.is_empty());
    assert_eq!(tex.mime_type(), "image/png");
}

/// Test creating checkerboard textures
#[test]
fn test_checkerboard_texture() {
    let tex = create_checkerboard("checker", 64, [255, 255, 255, 255], [0, 0, 0, 255]).unwrap();
    assert_eq!(tex.name, "checker");
    assert_eq!(tex.width, 64);
    assert_eq!(tex.height, 64);
    assert!(tex.byte_size() > 0);
}

/// Test texture cache operations
#[test]
fn test_texture_cache() {
    let mut cache = TextureCache::new();

    // Cache should be initially empty
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);

    // Add textures
    let tex1 = create_solid_color("white", 255, 255, 255, 255).unwrap();
    let tex2 = create_solid_color("black", 0, 0, 0, 255).unwrap();

    cache.insert(tex1);
    cache.insert(tex2);

    // Check cache state
    assert_eq!(cache.len(), 2);
    assert!(!cache.is_empty());
    assert!(cache.contains("white"));
    assert!(cache.contains("black"));
    assert!(!cache.contains("red"));

    // Retrieve textures
    let white = cache.get("white").unwrap();
    assert_eq!(white.name, "white");
}

/// Test texture iteration
#[test]
fn test_texture_cache_iteration() {
    let mut cache = TextureCache::new();

    cache.insert(create_solid_color("red", 255, 0, 0, 255).unwrap());
    cache.insert(create_solid_color("green", 0, 255, 0, 255).unwrap());
    cache.insert(create_solid_color("blue", 0, 0, 255, 255).unwrap());

    // Check names
    let names: Vec<_> = cache.names().into_iter().collect();
    assert_eq!(names.len(), 3);

    // Check all textures
    let count = cache.all().count();
    assert_eq!(count, 3);
}

/// Test cache clear
#[test]
fn test_texture_cache_clear() {
    let mut cache = TextureCache::new();

    cache.insert(create_solid_color("test", 128, 128, 128, 255).unwrap());
    assert!(!cache.is_empty());

    cache.clear();
    assert!(cache.is_empty());
}

/// Test Scene3DS with embedded textures
#[test]
fn test_scene_with_textures() {
    // Create a texture
    let tex = create_solid_color("wood", 139, 90, 43, 255).unwrap();

    // Create scene with texture
    let mut scene = Scene3DS::default();
    scene.textures.push(EmbeddedTexture {
        name: tex.name.clone(),
        data: tex.png_data.clone(),
        width: tex.width,
        height: tex.height,
    });

    // Add a material that references the texture
    scene.materials.insert(
        "wood_material".to_string(),
        Material3DS {
            name: "wood_material".to_string(),
            ambient: [0.1, 0.1, 0.1],
            diffuse: [0.8, 0.6, 0.4],
            specular: [0.3, 0.3, 0.3],
            texture: Some("wood".to_string()),
            metallic: 0.0,
            roughness: 0.7,
        },
    );

    // Verify scene state
    assert_eq!(scene.textures.len(), 1);
    assert_eq!(scene.materials.len(), 1);

    // The material should reference the texture
    let mat = scene.materials.get("wood_material").unwrap();
    assert_eq!(mat.texture.as_ref().unwrap(), "wood");
}

/// Test GLB export with textures
#[test]
fn test_glb_export_with_textures() {
    use ofml_lib::geometry::{CoordSystem, Face, Mesh, Vertex};

    // Create texture
    let tex = create_solid_color("test_tex", 200, 150, 100, 255).unwrap();

    // Create a simple cube mesh
    let vertices = vec![
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
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
    ];

    let faces = vec![
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
        Face {
            a: 4,
            b: 6,
            c: 5,
            flags: 0,
        },
        Face {
            a: 4,
            b: 7,
            c: 6,
            flags: 0,
        },
    ];

    let mesh = Mesh {
        name: "cube".to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: Vec::new(),
        material_name: Some("textured_material".to_string()),
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    };

    // Create scene with texture and material
    let mut materials = HashMap::new();
    materials.insert(
        "textured_material".to_string(),
        Material3DS {
            name: "textured_material".to_string(),
            ambient: [0.1, 0.1, 0.1],
            diffuse: [1.0, 1.0, 1.0],
            specular: [0.5, 0.5, 0.5],
            texture: Some("test_tex".to_string()),
            metallic: 0.0,
            roughness: 0.5,
        },
    );

    let scene = Scene3DS {
        meshes: vec![mesh],
        materials,
        textures: vec![EmbeddedTexture {
            name: tex.name.clone(),
            data: tex.png_data.clone(),
            width: tex.width,
            height: tex.height,
        }],
    };

    // Export to GLB
    let glb = scene_to_glb(&scene).expect("GLB export should succeed");

    // Verify GLB header
    assert!(glb.len() > 12, "GLB should have header");
    assert_eq!(&glb[0..4], b"glTF", "Should have glTF magic");

    // Version should be 2
    let version = u32::from_le_bytes([glb[4], glb[5], glb[6], glb[7]]);
    assert_eq!(version, 2, "Version should be 2");

    // GLB should be larger because of embedded texture
    assert!(glb.len() > 500, "GLB with texture should be substantial");
}

/// Test material with PBR properties
#[test]
fn test_material_pbr_properties() {
    let mat = Material3DS {
        name: "metallic_mat".to_string(),
        ambient: [0.1, 0.1, 0.1],
        diffuse: [0.8, 0.8, 0.8],
        specular: [1.0, 1.0, 1.0],
        texture: None,
        metallic: 0.9,
        roughness: 0.1,
    };

    assert_eq!(mat.name, "metallic_mat");
    assert!((mat.metallic - 0.9).abs() < 0.01);
    assert!((mat.roughness - 0.1).abs() < 0.01);
}
