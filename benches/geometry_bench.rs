//! Benchmarks for geometry loading and processing.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ofml_interpreter::geometry::{parse_obj, scene_to_glb};

// Sample OBJ data
fn create_sample_obj_data() -> Vec<u8> {
    let obj_content = r#"
# Simple cube OBJ
v -0.5 -0.5 0.5
v 0.5 -0.5 0.5
v 0.5 0.5 0.5
v -0.5 0.5 0.5
v -0.5 -0.5 -0.5
v 0.5 -0.5 -0.5
v 0.5 0.5 -0.5
v -0.5 0.5 -0.5
f 1 2 3 4
f 5 6 7 8
f 1 5 6 2
f 2 6 7 3
f 3 7 8 4
f 4 8 5 1
"#;
    obj_content.as_bytes().to_vec()
}

fn bench_parse_obj(c: &mut Criterion) {
    let data = create_sample_obj_data();

    c.bench_function("parse_obj_cube", |b| b.iter(|| parse_obj(black_box(&data))));
}

fn bench_parse_obj_large(c: &mut Criterion) {
    // Create a larger OBJ with many vertices
    let mut obj_content = String::from("# Large mesh\n");
    for i in 0..1000 {
        let x = (i % 10) as f32;
        let y = ((i / 10) % 10) as f32;
        let z = (i / 100) as f32;
        obj_content.push_str(&format!("v {} {} {}\n", x, y, z));
    }
    // Add some faces
    for i in 0..333 {
        let base = i * 3 + 1;
        obj_content.push_str(&format!("f {} {} {}\n", base, base + 1, base + 2));
    }
    let data = obj_content.as_bytes().to_vec();

    c.bench_function("parse_obj_large", |b| {
        b.iter(|| parse_obj(black_box(&data)))
    });
}

fn bench_scene_to_glb(c: &mut Criterion) {
    use ofml_interpreter::geometry::{CoordSystem, Face, Material3DS, Mesh, Scene3DS, Vertex};

    // Create a simple scene
    let mut scene = Scene3DS::default();
    let mesh = Mesh {
        name: "cube".to_string(),
        vertices: vec![
            Vertex {
                x: -0.5,
                y: -0.5,
                z: 0.5,
            },
            Vertex {
                x: 0.5,
                y: -0.5,
                z: 0.5,
            },
            Vertex {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Vertex {
                x: -0.5,
                y: 0.5,
                z: 0.5,
            },
            Vertex {
                x: -0.5,
                y: -0.5,
                z: -0.5,
            },
            Vertex {
                x: 0.5,
                y: -0.5,
                z: -0.5,
            },
            Vertex {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            Vertex {
                x: -0.5,
                y: 0.5,
                z: -0.5,
            },
        ],
        normals: vec![],
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
            Face {
                a: 4,
                b: 5,
                c: 6,
                flags: 0,
            },
            Face {
                a: 4,
                b: 6,
                c: 7,
                flags: 0,
            },
        ],
        tex_coords: vec![],
        material_name: Some("default".to_string()),
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    };
    scene.meshes.push(mesh);
    scene.materials.insert(
        "default".to_string(),
        Material3DS {
            name: "default".to_string(),
            diffuse: [0.8, 0.8, 0.8],
            ambient: [0.2, 0.2, 0.2],
            specular: [0.3, 0.3, 0.3],
            texture: None,
            metallic: 0.0,
            roughness: 0.5,
        },
    );

    c.bench_function("scene_to_glb", |b| {
        b.iter(|| scene_to_glb(black_box(&scene)))
    });
}

fn bench_glb_export_large(c: &mut Criterion) {
    use ofml_interpreter::geometry::{CoordSystem, Face, Material3DS, Mesh, Scene3DS, Vertex};

    // Create a larger scene
    let mut scene = Scene3DS::default();
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Generate a grid of vertices
    for i in 0..100 {
        for j in 0..100 {
            vertices.push(Vertex {
                x: i as f32 / 100.0,
                y: j as f32 / 100.0,
                z: ((i + j) as f32 / 200.0).sin() * 0.1,
            });
        }
    }

    // Generate faces
    for i in 0..99 {
        for j in 0..99 {
            let base = i * 100 + j;
            faces.push(Face {
                a: base as u16,
                b: (base + 1) as u16,
                c: (base + 100) as u16,
                flags: 0,
            });
            faces.push(Face {
                a: (base + 1) as u16,
                b: (base + 101) as u16,
                c: (base + 100) as u16,
                flags: 0,
            });
        }
    }

    scene.meshes.push(Mesh {
        name: "terrain".to_string(),
        vertices,
        normals: vec![],
        faces,
        tex_coords: vec![],
        material_name: Some("terrain_mat".to_string()),
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    });

    scene.materials.insert(
        "terrain_mat".to_string(),
        Material3DS {
            name: "terrain_mat".to_string(),
            diffuse: [0.5, 0.7, 0.4],
            ambient: [0.2, 0.2, 0.2],
            specular: [0.1, 0.1, 0.1],
            texture: None,
            metallic: 0.0,
            roughness: 0.8,
        },
    );

    c.bench_function("glb_export_large", |b| {
        b.iter(|| scene_to_glb(black_box(&scene)))
    });
}

criterion_group!(
    benches,
    bench_parse_obj,
    bench_parse_obj_large,
    bench_scene_to_glb,
    bench_glb_export_large
);
criterion_main!(benches);
