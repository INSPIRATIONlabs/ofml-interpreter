//! Integration tests for geometry loading and processing.
//!
//! These tests verify the geometry loading pipeline.

use ofml_interpreter::scene::{Geometry, SceneGraph};

/// Test creating various geometry primitives
#[test]
fn test_create_all_primitives() {
    let mut scene = SceneGraph::new();

    // Create block
    let block = scene.create_block("block".to_string(), [1.0, 2.0, 0.5], None);
    assert!(matches!(
        block.borrow().geometry,
        Geometry::Block { width, height, depth }
        if (width - 1.0).abs() < 0.01 && (height - 2.0).abs() < 0.01 && (depth - 0.5).abs() < 0.01
    ));

    // Create cylinder
    let cylinder = scene.create_cylinder("cylinder".to_string(), 0.5, 1.0, None);
    assert!(matches!(
        cylinder.borrow().geometry,
        Geometry::Cylinder { radius, height }
        if (radius - 0.5).abs() < 0.01 && (height - 1.0).abs() < 0.01
    ));

    // Create sphere
    let sphere = scene.create_sphere("sphere".to_string(), 0.3, None);
    assert!(matches!(
        sphere.borrow().geometry,
        Geometry::Sphere { radius }
        if (radius - 0.3).abs() < 0.01
    ));

    // Create ellipsoid
    let ellipsoid = scene.create_ellipsoid("ellipsoid".to_string(), 1.0, 0.5, 0.75, None);
    assert!(matches!(
        ellipsoid.borrow().geometry,
        Geometry::Ellipsoid { rx, ry, rz }
        if (rx - 1.0).abs() < 0.01 && (ry - 0.5).abs() < 0.01 && (rz - 0.75).abs() < 0.01
    ));

    // Create polygon
    let vertices = vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]];
    let polygon = scene.create_polygon("polygon".to_string(), vertices.clone(), 0.1, None);
    assert!(matches!(
        &polygon.borrow().geometry,
        Geometry::Polygon { vertices: v, thickness }
        if v.len() == 3 && (*thickness - 0.1).abs() < 0.01
    ));

    // Create frame
    let frame = scene.create_frame("frame".to_string(), 2.0, 1.5, 1.6, 1.1, 0.05, None);
    assert!(matches!(
        frame.borrow().geometry,
        Geometry::Frame { outer_width, outer_height, inner_width, inner_height, depth }
        if (outer_width - 2.0).abs() < 0.01 && (outer_height - 1.5).abs() < 0.01
           && (inner_width - 1.6).abs() < 0.01 && (inner_height - 1.1).abs() < 0.01
           && (depth - 0.05).abs() < 0.01
    ));

    // Create rotation
    let profile = vec![[0.0, 0.0], [0.5, 0.0], [0.5, 1.0], [0.3, 1.0]];
    let rotation = scene.create_rotation(
        "rotation".to_string(),
        profile.clone(),
        32,
        std::f32::consts::TAU,
        None,
    );
    assert!(matches!(
        &rotation.borrow().geometry,
        Geometry::Rotation { profile: p, segments, angle }
        if p.len() == 4 && *segments == 32 && (*angle - std::f32::consts::TAU).abs() < 0.01
    ));

    // Create sweep
    let sweep_profile = vec![[0.0, 0.0], [0.1, 0.0], [0.1, 0.1], [0.0, 0.1]];
    let path = vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 1.0]];
    let sweep = scene.create_sweep(
        "sweep".to_string(),
        sweep_profile.clone(),
        path.clone(),
        false,
        None,
    );
    assert!(matches!(
        &sweep.borrow().geometry,
        Geometry::Sweep { profile: p, path: pa, scale_along_path }
        if p.len() == 4 && pa.len() == 3 && !*scale_along_path
    ));
}

/// Test scene hierarchy and world position calculation
#[test]
fn test_scene_hierarchy() {
    let mut scene = SceneGraph::new();

    // Create a simple desk structure
    let desk = scene.create_part("desk".to_string(), None);
    desk.borrow_mut().set_position([0.0, 0.0, 0.0]);

    let top = scene.create_block("top".to_string(), [1.6, 0.025, 0.8], Some(desk.clone()));
    top.borrow_mut().set_position([0.0, 0.75, 0.0]);

    let leg1 = scene.create_cylinder("leg1".to_string(), 0.025, 0.75, Some(desk.clone()));
    leg1.borrow_mut().set_position([0.1, 0.0, 0.1]);

    let leg2 = scene.create_cylinder("leg2".to_string(), 0.025, 0.75, Some(desk.clone()));
    leg2.borrow_mut().set_position([1.5, 0.0, 0.1]);

    // Verify hierarchy
    assert!(scene.exists("desk.top"));
    assert!(scene.exists("desk.leg1"));
    assert!(scene.exists("desk.leg2"));

    // Verify world positions
    let top_pos = top.borrow().get_world_position();
    assert!((top_pos[1] - 0.75).abs() < 0.01);

    let leg1_pos = leg1.borrow().get_world_position();
    assert!((leg1_pos[0] - 0.1).abs() < 0.01);
    assert!((leg1_pos[2] - 0.1).abs() < 0.01);
}

/// Test bounding box calculations for primitives
#[test]
fn test_bounding_boxes() {
    let mut scene = SceneGraph::new();

    // Block bounds
    let block = scene.create_block("block".to_string(), [2.0, 1.0, 0.5], None);
    let bounds = block.borrow().get_local_bounds();
    assert!((bounds[1][0] - 2.0).abs() < 0.01);
    assert!((bounds[1][1] - 1.0).abs() < 0.01);
    assert!((bounds[1][2] - 0.5).abs() < 0.01);

    // Sphere bounds
    let sphere = scene.create_sphere("sphere".to_string(), 0.5, None);
    let bounds = sphere.borrow().get_local_bounds();
    assert!((bounds[0][0] - (-0.5)).abs() < 0.01);
    assert!((bounds[1][0] - 0.5).abs() < 0.01);

    // Ellipsoid bounds
    let ellipsoid = scene.create_ellipsoid("ellipsoid".to_string(), 1.0, 0.5, 0.75, None);
    let bounds = ellipsoid.borrow().get_local_bounds();
    assert!((bounds[0][0] - (-1.0)).abs() < 0.01);
    assert!((bounds[1][0] - 1.0).abs() < 0.01);
    assert!((bounds[0][1] - (-0.5)).abs() < 0.01);
    assert!((bounds[1][1] - 0.5).abs() < 0.01);
}

/// Test scene export to Scene3DS
#[test]
fn test_scene_export() {
    let mut scene = SceneGraph::new();

    // Create a simple scene
    let root = scene.create_part("root".to_string(), None);
    let _block = scene.create_block("geo".to_string(), [1.0, 1.0, 1.0], Some(root.clone()));

    // Export to Scene3DS
    let scene3ds = scene.to_scene();

    // Should have at least one mesh
    assert!(!scene3ds.meshes.is_empty());

    // The mesh should have vertices and faces
    let mesh = &scene3ds.meshes[0];
    assert!(!mesh.vertices.is_empty());
    assert!(!mesh.faces.is_empty());
}

/// Test primitive mesh generation
#[test]
fn test_primitive_mesh_generation() {
    let mut scene = SceneGraph::new();

    // Create primitives of each type
    let root = scene.create_part("root".to_string(), None);
    let _sphere = scene.create_sphere("sphere".to_string(), 0.5, Some(root.clone()));
    let _cylinder = scene.create_cylinder("cylinder".to_string(), 0.25, 1.0, Some(root.clone()));
    let _ellipsoid =
        scene.create_ellipsoid("ellipsoid".to_string(), 0.3, 0.2, 0.4, Some(root.clone()));

    // Export and verify meshes are generated
    let scene3ds = scene.to_scene();

    // Should have 3 meshes
    assert_eq!(scene3ds.meshes.len(), 3);

    // Each mesh should have valid geometry
    for mesh in &scene3ds.meshes {
        assert!(!mesh.vertices.is_empty(), "Mesh should have vertices");
        assert!(!mesh.faces.is_empty(), "Mesh should have faces");
    }
}

/// Test mesh count tracking
#[test]
fn test_mesh_count() {
    let mut scene = SceneGraph::new();

    // Start with empty scene
    assert_eq!(scene.mesh_count(), 0);

    // Add primitives
    let _block = scene.create_block("block".to_string(), [1.0, 1.0, 1.0], None);
    assert_eq!(scene.mesh_count(), 1);

    let _sphere = scene.create_sphere("sphere".to_string(), 0.5, None);
    assert_eq!(scene.mesh_count(), 2);

    let _cylinder = scene.create_cylinder("cylinder".to_string(), 0.3, 1.0, None);
    assert_eq!(scene.mesh_count(), 3);
}

/// Test node removal
#[test]
fn test_node_removal() {
    let mut scene = SceneGraph::new();

    let root = scene.create_part("root".to_string(), None);
    let child = scene.create_block("child".to_string(), [1.0, 1.0, 1.0], Some(root.clone()));

    assert!(scene.exists("root.child"));
    assert_eq!(scene.mesh_count(), 1);

    // Remove the child node
    let child_id = child.borrow().id;
    scene.remove_node(child_id);

    assert!(!scene.exists("root.child"));
    assert_eq!(scene.mesh_count(), 0);
}
