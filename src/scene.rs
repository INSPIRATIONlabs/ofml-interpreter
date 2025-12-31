//! Scene graph for OFML 3D objects
//!
//! This module provides a scene graph that tracks 3D objects created by CLS execution.
//! Each node can have geometry (meshes), position, rotation, and child nodes.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::geometry::{
    load_geometry_from_alb_raw, CoordSystem, GeometryError, Material3DS, Mesh, Scene3DS,
};

/// Type of geometry a scene node can have
#[derive(Clone, Debug)]
pub enum Geometry {
    /// No geometry (group node)
    None,
    /// Box primitive with dimensions [width, height, depth]
    Block { width: f32, height: f32, depth: f32 },
    /// External mesh loaded from 3DS file
    Mesh(Scene3DS),
    /// Cylinder primitive
    Cylinder { radius: f32, height: f32 },
    /// Sphere primitive
    Sphere { radius: f32 },
    /// Ellipsoid primitive with radii in x, y, z
    Ellipsoid { rx: f32, ry: f32, rz: f32 },
    /// Planar convex polygon with vertices
    Polygon {
        vertices: Vec<[f32; 2]>,
        thickness: f32,
    },
    /// Frame/border geometry (rectangular with hole)
    Frame {
        outer_width: f32,
        outer_height: f32,
        inner_width: f32,
        inner_height: f32,
        depth: f32,
    },
    /// Rotational sweep geometry (profile rotated around axis)
    Rotation {
        profile: Vec<[f32; 2]>, // 2D profile points (r, z)
        segments: u32,          // Number of segments around
        angle: f32,             // Rotation angle in radians (2π for full)
    },
    /// Extrusion/sweep geometry (profile extruded along path)
    Sweep {
        profile: Vec<[f32; 2]>, // 2D profile points
        path: Vec<[f32; 3]>,    // 3D path points
        scale_along_path: bool, // Whether to scale profile along path
    },
}

/// Rotation axis constants (matching OFML @NX, @NY, @NZ)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    X, // @NX
    Y, // @NY
    Z, // @NZ
}

/// OFML Alignment mode for each axis
/// @I = min bound (default), @C = center, @A = max bound
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignMode {
    /// Use minimum bound (default in OFML, symbol @I)
    Min,
    /// Use center (symbol @C)
    Center,
    /// Use maximum bound (symbol @A)
    Max,
}

impl Default for AlignMode {
    fn default() -> Self {
        AlignMode::Min
    }
}

/// A node in the scene graph
#[derive(Clone)]
pub struct SceneNode {
    /// Unique identifier
    pub id: u64,
    /// Node name (e.g., "geo", "leg_1")
    pub name: String,
    /// Full path name (e.g., "desk.plate.geo")
    pub full_name: String,
    /// Node type (class name like "OiPart", "OiBlock")
    pub node_type: String,
    /// Geometry data
    pub geometry: Geometry,
    /// Position in local space [x, y, z] - set via setPosition()
    pub position: [f32; 3],
    /// Alignment offset [x, y, z] - set via setAlignment(), applied to geometry
    /// This is the OFML "geo.position" that offsets geometry within the node
    pub alignment_offset: [f32; 3],
    /// Original geometry bounds before any transformations (for setAlignment calculations)
    pub original_bounds: Option<[[f32; 3]; 2]>,
    /// Rotation angles [rx, ry, rz] in radians
    pub rotation: [f32; 3],
    /// Scale factor
    pub scale: f32,
    /// Material name
    pub material: String,
    /// Parent node (weak reference to avoid cycles)
    pub parent: Option<Rc<RefCell<SceneNode>>>,
    /// Child nodes
    pub children: Vec<Rc<RefCell<SceneNode>>>,
    /// Whether this node is visible
    pub visible: bool,
    /// Whether this node is selectable
    pub selectable: bool,
    /// Custom properties
    pub properties: HashMap<String, String>,
}

impl SceneNode {
    /// Create a new scene node
    pub fn new(id: u64, name: String, node_type: String) -> Self {
        Self {
            id,
            full_name: name.clone(),
            name,
            node_type,
            geometry: Geometry::None,
            position: [0.0, 0.0, 0.0],
            alignment_offset: [0.0, 0.0, 0.0],
            original_bounds: None,
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
            material: String::new(),
            parent: None,
            children: Vec::new(),
            visible: true,
            selectable: true,
            properties: HashMap::new(),
        }
    }

    /// Set position
    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.position = pos;
    }

    /// Get position
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set alignment offset directly
    pub fn set_alignment_offset(&mut self, offset: [f32; 3]) {
        self.alignment_offset = offset;
    }

    /// Get alignment offset
    pub fn get_alignment_offset(&self) -> [f32; 3] {
        self.alignment_offset
    }

    /// Set alignment based on OFML alignment modes
    /// This implements the OFML setAlignment() function:
    /// - AlignMode::Min (@I) = use minimum bound (default)
    /// - AlignMode::Center (@C) = use center
    /// - AlignMode::Max (@A) = use maximum bound
    ///
    /// The alignment offset is set to the negative of the calculated point,
    /// so that point becomes the origin of the geometry.
    pub fn set_alignment(&mut self, align_x: AlignMode, align_y: AlignMode, align_z: AlignMode) {
        // Use original bounds if available, otherwise calculate from current geometry
        let bounds = self
            .original_bounds
            .unwrap_or_else(|| self.get_local_bounds());
        let [min, max] = bounds;

        // Calculate the alignment point for each axis
        let x = match align_x {
            AlignMode::Min => min[0],
            AlignMode::Center => (min[0] + max[0]) / 2.0,
            AlignMode::Max => max[0],
        };
        let y = match align_y {
            AlignMode::Min => min[1],
            AlignMode::Center => (min[1] + max[1]) / 2.0,
            AlignMode::Max => max[1],
        };
        let z = match align_z {
            AlignMode::Min => min[2],
            AlignMode::Center => (min[2] + max[2]) / 2.0,
            AlignMode::Max => max[2],
        };

        // Set the offset to negative of the alignment point
        // This makes the alignment point the effective origin
        self.alignment_offset = [-x, -y, -z];

        // Debug
        eprintln!("    setAlignment: bounds=[{:.2},{:.2},{:.2}]-[{:.2},{:.2},{:.2}] => offset=[{:.2}, {:.2}, {:.2}]",
            min[0], min[1], min[2], max[0], max[1], max[2],
            self.alignment_offset[0], self.alignment_offset[1], self.alignment_offset[2]);
    }

    /// Apply default alignment: center X, ground Y, center Z
    /// This is the most common alignment for furniture parts
    pub fn set_default_alignment(&mut self) {
        self.set_alignment(AlignMode::Center, AlignMode::Min, AlignMode::Center);
    }

    /// Set foot alignment for L-shaped or irregular geometry
    /// This finds the ground contact point (centroid of vertices near Y=min)
    /// and uses that as the X,Z anchor point instead of bounding box center.
    /// Useful for L-shaped legs where the bounding box center isn't the contact point.
    pub fn set_foot_alignment(&mut self) {
        let bounds = self
            .original_bounds
            .unwrap_or_else(|| self.get_local_bounds());
        let [min, max] = bounds;

        // Default to bounding box center if we can't find foot vertices
        let mut foot_x = (min[0] + max[0]) / 2.0;
        let mut foot_z = (min[2] + max[2]) / 2.0;

        // Find vertices near ground level and compute their centroid
        if let Geometry::Mesh(scene) = &self.geometry {
            let ground_threshold = min[1] + 0.02; // 2cm above minimum
            let mut sum_x = 0.0f32;
            let mut sum_z = 0.0f32;
            let mut count = 0;

            for mesh in &scene.meshes {
                for v in &mesh.vertices {
                    if v.y < ground_threshold {
                        sum_x += v.x;
                        sum_z += v.z;
                        count += 1;
                    }
                }
            }

            if count > 0 {
                foot_x = sum_x / count as f32;
                foot_z = sum_z / count as f32;
                eprintln!(
                    "    setFootAlignment: found {} ground vertices, foot at ({:.3}, {:.3})",
                    count, foot_x, foot_z
                );
            }
        }

        // Set offset to bring foot to origin, grounded
        self.alignment_offset = [-foot_x, -min[1], -foot_z];
        eprintln!(
            "    setFootAlignment: offset=[{:.2}, {:.2}, {:.2}]",
            self.alignment_offset[0], self.alignment_offset[1], self.alignment_offset[2]
        );
    }

    /// Store the original bounds of the geometry (call after loading geometry)
    pub fn store_original_bounds(&mut self) {
        self.original_bounds = Some(self.get_local_bounds());
    }

    /// Rotate around an axis by an angle (in radians)
    pub fn rotate(&mut self, axis: Axis, angle: f32) {
        match axis {
            Axis::X => self.rotation[0] += angle,
            Axis::Y => self.rotation[1] += angle,
            Axis::Z => self.rotation[2] += angle,
        }
    }

    /// Set rotation directly
    pub fn set_rotation(&mut self, axis: Axis, angle: f32) {
        match axis {
            Axis::X => self.rotation[0] = angle,
            Axis::Y => self.rotation[1] = angle,
            Axis::Z => self.rotation[2] = angle,
        }
    }

    /// Get rotation around an axis
    pub fn get_rotation(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.rotation[0],
            Axis::Y => self.rotation[1],
            Axis::Z => self.rotation[2],
        }
    }

    /// Calculate world transform (position + rotation from parent chain)
    pub fn get_world_position(&self) -> [f32; 3] {
        let mut pos = self.position;
        if let Some(ref parent) = self.parent {
            let parent_pos = parent.borrow().get_world_position();
            let parent_rot = parent.borrow().rotation;

            // Apply parent rotation to local position
            let rotated = rotate_point(pos, parent_rot);
            pos = [
                parent_pos[0] + rotated[0],
                parent_pos[1] + rotated[1],
                parent_pos[2] + rotated[2],
            ];
        }
        pos
    }

    /// Get world rotation (accumulated from parent chain)
    pub fn get_world_rotation(&self) -> [f32; 3] {
        let mut rot = self.rotation;
        if let Some(ref parent) = self.parent {
            let parent_rot = parent.borrow().get_world_rotation();
            rot = [
                rot[0] + parent_rot[0],
                rot[1] + parent_rot[1],
                rot[2] + parent_rot[2],
            ];
        }
        rot
    }

    /// Get local bounding box for geometry
    pub fn get_local_bounds(&self) -> [[f32; 3]; 2] {
        match &self.geometry {
            Geometry::None => [[0.0; 3], [0.0; 3]],
            Geometry::Block {
                width,
                height,
                depth,
            } => [[0.0, 0.0, 0.0], [*width, *height, *depth]],
            Geometry::Mesh(scene) => {
                let mut min = [f32::MAX; 3];
                let mut max = [f32::MIN; 3];
                for mesh in &scene.meshes {
                    for v in &mesh.vertices {
                        min[0] = min[0].min(v.x);
                        min[1] = min[1].min(v.y);
                        min[2] = min[2].min(v.z);
                        max[0] = max[0].max(v.x);
                        max[1] = max[1].max(v.y);
                        max[2] = max[2].max(v.z);
                    }
                }
                if min[0] == f32::MAX {
                    [[0.0; 3], [0.0; 3]]
                } else {
                    [min, max]
                }
            }
            Geometry::Cylinder { radius, height } => {
                [[-*radius, 0.0, -*radius], [*radius, *height, *radius]]
            }
            Geometry::Sphere { radius } => [[-*radius; 3], [*radius; 3]],
            Geometry::Ellipsoid { rx, ry, rz } => [[-*rx, -*ry, -*rz], [*rx, *ry, *rz]],
            Geometry::Polygon {
                vertices,
                thickness,
            } => {
                if vertices.is_empty() {
                    [[0.0; 3], [0.0; 3]]
                } else {
                    let mut min_x = f32::MAX;
                    let mut min_z = f32::MAX;
                    let mut max_x = f32::MIN;
                    let mut max_z = f32::MIN;
                    for v in vertices {
                        min_x = min_x.min(v[0]);
                        min_z = min_z.min(v[1]);
                        max_x = max_x.max(v[0]);
                        max_z = max_z.max(v[1]);
                    }
                    [[min_x, 0.0, min_z], [max_x, *thickness, max_z]]
                }
            }
            Geometry::Frame {
                outer_width,
                outer_height,
                depth,
                ..
            } => [[0.0, 0.0, 0.0], [*outer_width, *outer_height, *depth]],
            Geometry::Rotation { profile, .. } => {
                if profile.is_empty() {
                    [[0.0; 3], [0.0; 3]]
                } else {
                    let mut max_r = 0.0f32;
                    let mut min_z = f32::MAX;
                    let mut max_z = f32::MIN;
                    for p in profile {
                        max_r = max_r.max(p[0].abs());
                        min_z = min_z.min(p[1]);
                        max_z = max_z.max(p[1]);
                    }
                    [[-max_r, min_z, -max_r], [max_r, max_z, max_r]]
                }
            }
            Geometry::Sweep { profile, path, .. } => {
                if profile.is_empty() || path.is_empty() {
                    [[0.0; 3], [0.0; 3]]
                } else {
                    // Approximate bounds from path and profile extent
                    let mut min = [f32::MAX; 3];
                    let mut max = [f32::MIN; 3];
                    let mut profile_extent = 0.0f32;
                    for p in profile {
                        profile_extent = profile_extent.max(p[0].abs()).max(p[1].abs());
                    }
                    for pt in path {
                        min[0] = min[0].min(pt[0] - profile_extent);
                        min[1] = min[1].min(pt[1] - profile_extent);
                        min[2] = min[2].min(pt[2] - profile_extent);
                        max[0] = max[0].max(pt[0] + profile_extent);
                        max[1] = max[1].max(pt[1] + profile_extent);
                        max[2] = max[2].max(pt[2] + profile_extent);
                    }
                    [min, max]
                }
            }
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child: Rc<RefCell<SceneNode>>) {
        self.children.push(child);
    }

    /// Find child by name
    pub fn find_child(&self, name: &str) -> Option<Rc<RefCell<SceneNode>>> {
        for child in &self.children {
            if child.borrow().name == name {
                return Some(child.clone());
            }
        }
        None
    }

    /// Recursively find node by path (e.g., "geo.screen")
    pub fn find_by_path(&self, path: &str) -> Option<Rc<RefCell<SceneNode>>> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let first = parts[0];
        for child in &self.children {
            if child.borrow().name == first {
                if parts.len() == 1 {
                    return Some(child.clone());
                } else {
                    let rest = parts[1..].join(".");
                    return child.borrow().find_by_path(&rest);
                }
            }
        }
        None
    }
}

/// Rotate a point by euler angles [rx, ry, rz]
fn rotate_point(point: [f32; 3], rotation: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = point;
    let [rx, ry, rz] = rotation;

    // Rotate around X
    let (sin_x, cos_x) = rx.sin_cos();
    let y1 = y * cos_x - z * sin_x;
    let z1 = y * sin_x + z * cos_x;

    // Rotate around Y
    let (sin_y, cos_y) = ry.sin_cos();
    let x2 = x * cos_y + z1 * sin_y;
    let z2 = -x * sin_y + z1 * cos_y;

    // Rotate around Z
    let (sin_z, cos_z) = rz.sin_cos();
    let x3 = x2 * cos_z - y1 * sin_z;
    let y3 = x2 * sin_z + y1 * cos_z;

    [x3, y3, z2]
}

/// The scene graph manages all 3D objects
pub struct SceneGraph {
    /// All nodes indexed by ID
    nodes: HashMap<u64, Rc<RefCell<SceneNode>>>,
    /// Nodes indexed by full name
    nodes_by_name: HashMap<String, Rc<RefCell<SceneNode>>>,
    /// Root nodes (nodes without parents)
    pub roots: Vec<Rc<RefCell<SceneNode>>>,
    /// Next available node ID
    next_id: u64,
    /// Path to ALB archive for loading geometry
    pub alb_path: Option<std::path::PathBuf>,
}

impl SceneGraph {
    /// Create a new scene graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            nodes_by_name: HashMap::new(),
            roots: Vec::new(),
            next_id: 1,
            alb_path: None,
        }
    }

    /// Set the ALB archive path for geometry loading
    pub fn set_alb_path(&mut self, path: &Path) {
        self.alb_path = Some(path.to_path_buf());
    }

    /// Create a new node
    pub fn create_node(
        &mut self,
        name: String,
        node_type: String,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let id = self.next_id;
        self.next_id += 1;

        let full_name = if let Some(ref p) = parent {
            format!("{}.{}", p.borrow().full_name, name)
        } else {
            name.clone()
        };

        let node = Rc::new(RefCell::new(SceneNode::new(id, name, node_type)));
        node.borrow_mut().full_name = full_name.clone();

        if let Some(ref p) = parent {
            node.borrow_mut().parent = Some(p.clone());
            p.borrow_mut().add_child(node.clone());
        } else {
            self.roots.push(node.clone());
        }

        self.nodes.insert(id, node.clone());
        self.nodes_by_name.insert(full_name, node.clone());
        node
    }

    /// Create an OiPart node
    pub fn create_part(
        &mut self,
        name: String,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        self.create_node(name, "OiPart".to_string(), parent)
    }

    /// Create an OiBlock node with dimensions
    pub fn create_block(
        &mut self,
        name: String,
        dims: [f32; 3],
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiBlock".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Block {
            width: dims[0],
            height: dims[1],
            depth: dims[2],
        };
        node
    }

    /// Create a cylinder geometry node
    pub fn create_cylinder(
        &mut self,
        name: String,
        radius: f32,
        height: f32,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiCylinder".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Cylinder { radius, height };
        node
    }

    /// Create a sphere geometry node
    pub fn create_sphere(
        &mut self,
        name: String,
        radius: f32,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiSphere".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Sphere { radius };
        node
    }

    /// Create an ellipsoid geometry node
    pub fn create_ellipsoid(
        &mut self,
        name: String,
        rx: f32,
        ry: f32,
        rz: f32,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiEllipsoid".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Ellipsoid { rx, ry, rz };
        node
    }

    /// Create a polygon geometry node
    pub fn create_polygon(
        &mut self,
        name: String,
        vertices: Vec<[f32; 2]>,
        thickness: f32,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiPolygon".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Polygon {
            vertices,
            thickness,
        };
        node
    }

    /// Create a frame geometry node (rectangular with hole)
    pub fn create_frame(
        &mut self,
        name: String,
        outer_width: f32,
        outer_height: f32,
        inner_width: f32,
        inner_height: f32,
        depth: f32,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiFrame".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Frame {
            outer_width,
            outer_height,
            inner_width,
            inner_height,
            depth,
        };
        node
    }

    /// Create a rotation (lathe) geometry node
    pub fn create_rotation(
        &mut self,
        name: String,
        profile: Vec<[f32; 2]>,
        segments: u32,
        angle: f32,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiRotation".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Rotation {
            profile,
            segments,
            angle,
        };
        node
    }

    /// Create a sweep (extrusion) geometry node
    pub fn create_sweep(
        &mut self,
        name: String,
        profile: Vec<[f32; 2]>,
        path: Vec<[f32; 3]>,
        scale_along_path: bool,
        parent: Option<Rc<RefCell<SceneNode>>>,
    ) -> Rc<RefCell<SceneNode>> {
        let node = self.create_node(name, "OiSweep".to_string(), parent);
        node.borrow_mut().geometry = Geometry::Sweep {
            profile,
            path,
            scale_along_path,
        };
        node
    }

    /// Check if a node exists by full name
    pub fn exists(&self, full_name: &str) -> bool {
        self.nodes_by_name.contains_key(full_name)
    }

    /// Get node by full name
    pub fn get_by_name(&self, full_name: &str) -> Option<Rc<RefCell<SceneNode>>> {
        self.nodes_by_name.get(full_name).cloned()
    }

    /// Remove a node by ID
    pub fn remove_node(&mut self, id: u64) {
        if let Some(node) = self.nodes.remove(&id) {
            let full_name = node.borrow().full_name.clone();
            self.nodes_by_name.remove(&full_name);

            // Remove from parent's children
            if let Some(ref parent) = node.borrow().parent {
                parent.borrow_mut().children.retain(|c| c.borrow().id != id);
            }

            // Remove from roots if it's a root
            self.roots.retain(|r| r.borrow().id != id);

            // Recursively remove children
            let children: Vec<_> = node.borrow().children.clone();
            for child in children {
                self.remove_node(child.borrow().id);
            }
        }
    }

    /// Load geometry from ALB archive into a node (uses internal alb_path)
    /// Geometry is loaded with raw coordinates. No alignment is applied by default.
    /// Call setAlignment() to position the geometry origin.
    pub fn load_geometry(
        &mut self,
        node: Rc<RefCell<SceneNode>>,
        pattern: &str,
    ) -> Result<(), GeometryError> {
        let alb_path = self.alb_path.as_ref().ok_or_else(|| {
            GeometryError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No ALB path set",
            ))
        })?;

        let (scene, bounds) = load_geometry_from_alb_raw(alb_path, pattern)?;
        let mut node_mut = node.borrow_mut();
        node_mut.geometry = Geometry::Mesh(scene);
        node_mut.original_bounds = Some(bounds);
        // No default alignment - geometry uses raw coordinates
        // CLS can call setAlignment() to set the origin point
        Ok(())
    }

    /// Load 3D geometry (3DS or OBJ) from a specific ALB archive into a node
    /// Geometry is loaded with raw coordinates. No alignment is applied by default.
    /// Call setAlignment() to position the geometry origin.
    pub fn load_3ds_from_alb(
        &mut self,
        node: Rc<RefCell<SceneNode>>,
        alb_path: &Path,
        pattern: &str,
    ) -> Result<(), GeometryError> {
        let (scene, bounds) = load_geometry_from_alb_raw(alb_path, pattern)?;

        // Debug: print raw geometry bounds and transform
        eprintln!(
            "  Loaded '{}': raw bounds min=[{:.4}, {:.4}, {:.4}] max=[{:.4}, {:.4}, {:.4}]",
            pattern,
            bounds[0][0],
            bounds[0][1],
            bounds[0][2],
            bounds[1][0],
            bounds[1][1],
            bounds[1][2]
        );
        // Print mesh transform if available
        for mesh in &scene.meshes {
            eprintln!("    Transform: [{:.2}, {:.2}, {:.2}] [{:.2}, {:.2}, {:.2}] [{:.2}, {:.2}, {:.2}] T=[{:.2}, {:.2}, {:.2}]",
                mesh.transform[0], mesh.transform[1], mesh.transform[2],
                mesh.transform[3], mesh.transform[4], mesh.transform[5],
                mesh.transform[6], mesh.transform[7], mesh.transform[8],
                mesh.transform[9], mesh.transform[10], mesh.transform[11]);
        }

        let mut node_mut = node.borrow_mut();
        node_mut.geometry = Geometry::Mesh(scene);
        node_mut.original_bounds = Some(bounds);
        // No default alignment - geometry uses raw coordinates
        // CLS can call setAlignment() to set the origin point
        Ok(())
    }

    /// Convert scene graph to a Scene3DS for export
    pub fn to_scene(&self) -> Scene3DS {
        let mut scene = Scene3DS::default();

        for root in &self.roots {
            self.collect_meshes(&root.borrow(), &mut scene, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        scene
    }

    /// Recursively collect meshes from nodes
    fn collect_meshes(
        &self,
        node: &SceneNode,
        scene: &mut Scene3DS,
        parent_pos: [f32; 3],
        parent_rot: [f32; 3],
    ) {
        if !node.visible {
            return;
        }

        // Calculate world transform
        let local_pos = node.position;
        let rotated_pos = rotate_point(local_pos, parent_rot);
        let world_pos = [
            parent_pos[0] + rotated_pos[0],
            parent_pos[1] + rotated_pos[1],
            parent_pos[2] + rotated_pos[2],
        ];
        let world_rot = [
            parent_rot[0] + node.rotation[0],
            parent_rot[1] + node.rotation[1],
            parent_rot[2] + node.rotation[2],
        ];

        // Get alignment offset - this is applied to geometry within the node
        let align_offset = node.alignment_offset;

        // Get material name from node (may be empty)
        let material_name = if !node.material.is_empty() {
            Some(node.material.clone())
        } else {
            None
        };

        // Ensure material exists in scene if specified
        if let Some(ref mat_name) = material_name {
            if !scene.materials.contains_key(mat_name) {
                // Create a default material with a color based on the material name
                let color = material_name_to_color(mat_name);
                scene.materials.insert(
                    mat_name.clone(),
                    Material3DS {
                        name: mat_name.clone(),
                        ambient: color,
                        diffuse: color,
                        specular: [0.3, 0.3, 0.3],
                        texture: None,
                        metallic: 0.0,
                        roughness: 0.5,
                    },
                );
            }
        }

        match &node.geometry {
            Geometry::None => {}
            Geometry::Block {
                width,
                height,
                depth,
            } => {
                // For blocks, apply alignment offset to the position
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh =
                    create_box_mesh(&node.name, *width, *height, *depth, adjusted_pos, world_rot);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Mesh(src_scene) => {
                // Copy materials from source scene
                for (name, mat) in &src_scene.materials {
                    if !scene.materials.contains_key(name) {
                        scene.materials.insert(name.clone(), mat.clone());
                    }
                }

                for src_mesh in &src_scene.meshes {
                    let mut mesh = src_mesh.clone();
                    // Apply alignment offset first (geometry offset within node)
                    mesh.translate(align_offset);
                    // Then apply world transform (rotations in X, Y, Z order)
                    mesh.rotate_x(world_rot[0]);
                    mesh.rotate_y(world_rot[1]);
                    mesh.rotate_z(world_rot[2]);
                    mesh.translate(world_pos);

                    // Override material if set on node
                    if material_name.is_some() {
                        mesh.material_name = material_name.clone();
                    }

                    // Debug: print final mesh bounds
                    let (min, max) = mesh.bounds();
                    eprintln!(
                        "  Final '{}': bounds=[{:.2},{:.2},{:.2}]-[{:.2},{:.2},{:.2}]",
                        node.name, min[0], min[1], min[2], max[0], max[1], max[2]
                    );

                    scene.meshes.push(mesh);
                }
            }
            Geometry::Cylinder { radius, height } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh =
                    create_cylinder_mesh(&node.name, *radius, *height, 16, adjusted_pos, world_rot);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Sphere { radius } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh = create_sphere_mesh(&node.name, *radius, 16, 8, adjusted_pos);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Ellipsoid { rx, ry, rz } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh =
                    create_ellipsoid_mesh(&node.name, *rx, *ry, *rz, 16, 8, adjusted_pos);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Polygon {
                vertices,
                thickness,
            } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh =
                    create_polygon_mesh(&node.name, vertices, *thickness, adjusted_pos, world_rot);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Frame {
                outer_width,
                outer_height,
                inner_width,
                inner_height,
                depth,
            } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh = create_frame_mesh(
                    &node.name,
                    *outer_width,
                    *outer_height,
                    *inner_width,
                    *inner_height,
                    *depth,
                    adjusted_pos,
                    world_rot,
                );
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Rotation {
                profile,
                segments,
                angle,
            } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh =
                    create_rotation_mesh(&node.name, profile, *segments, *angle, adjusted_pos);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
            Geometry::Sweep {
                profile,
                path,
                scale_along_path,
            } => {
                let adjusted_pos = [
                    world_pos[0] + align_offset[0],
                    world_pos[1] + align_offset[1],
                    world_pos[2] + align_offset[2],
                ];
                let mut mesh =
                    create_sweep_mesh(&node.name, profile, path, *scale_along_path, adjusted_pos);
                mesh.material_name = material_name.clone();
                scene.meshes.push(mesh);
            }
        }

        // Process children
        for child in &node.children {
            self.collect_meshes(&child.borrow(), scene, world_pos, world_rot);
        }
    }

    /// Debug print the scene graph
    pub fn debug_print(&self) {
        println!("Scene Graph ({} nodes):", self.nodes.len());
        for root in &self.roots {
            self.debug_print_node(&root.borrow(), 0);
        }
    }

    fn debug_print_node(&self, node: &SceneNode, indent: usize) {
        let prefix = "  ".repeat(indent);
        let geo_info = match &node.geometry {
            Geometry::None => "".to_string(),
            Geometry::Block {
                width,
                height,
                depth,
            } => format!(" [Block {}x{}x{}]", width, height, depth),
            Geometry::Mesh(scene) => format!(" [Mesh {} meshes]", scene.meshes.len()),
            Geometry::Cylinder { radius, height } => {
                format!(" [Cylinder r={} h={}]", radius, height)
            }
            Geometry::Sphere { radius } => format!(" [Sphere r={}]", radius),
            Geometry::Ellipsoid { rx, ry, rz } => {
                format!(" [Ellipsoid rx={} ry={} rz={}]", rx, ry, rz)
            }
            Geometry::Polygon {
                vertices,
                thickness,
            } => format!(" [Polygon {} verts t={}]", vertices.len(), thickness),
            Geometry::Frame {
                outer_width,
                outer_height,
                depth,
                ..
            } => format!(" [Frame {}x{}x{}]", outer_width, outer_height, depth),
            Geometry::Rotation {
                profile, segments, ..
            } => format!(" [Rotation {} pts {} segs]", profile.len(), segments),
            Geometry::Sweep { profile, path, .. } => {
                format!(" [Sweep {} profile {} path]", profile.len(), path.len())
            }
        };
        println!(
            "{}{} ({}){}  pos={:?} rot={:?}",
            prefix, node.name, node.node_type, geo_info, node.position, node.rotation
        );

        for child in &node.children {
            self.debug_print_node(&child.borrow(), indent + 1);
        }
    }

    /// Get total mesh count
    pub fn mesh_count(&self) -> usize {
        let mut count = 0;
        for root in &self.roots {
            count += self.count_meshes(&root.borrow());
        }
        count
    }

    fn count_meshes(&self, node: &SceneNode) -> usize {
        let mut count = match &node.geometry {
            Geometry::None => 0,
            Geometry::Block { .. } => 1,
            Geometry::Mesh(scene) => scene.meshes.len(),
            Geometry::Cylinder { .. } => 1,
            Geometry::Sphere { .. } => 1,
            Geometry::Ellipsoid { .. } => 1,
            Geometry::Polygon { .. } => 1,
            Geometry::Frame { .. } => 1,
            Geometry::Rotation { .. } => 1,
            Geometry::Sweep { .. } => 1,
        };
        for child in &node.children {
            count += self.count_meshes(&child.borrow());
        }
        count
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert OFML material name to a default color
/// Material names like "::vitra::material::grey" get parsed for color hints
fn material_name_to_color(name: &str) -> [f32; 3] {
    let lower = name.to_lowercase();

    // Check for common color keywords in material name
    if lower.contains("grey") || lower.contains("gray") {
        return [0.5, 0.5, 0.5];
    }
    if lower.contains("dark") {
        return [0.2, 0.2, 0.2];
    }
    if lower.contains("light") || lower.contains("soft_light") {
        return [0.85, 0.85, 0.85];
    }
    if lower.contains("white") {
        return [0.95, 0.95, 0.95];
    }
    if lower.contains("black") {
        return [0.1, 0.1, 0.1];
    }
    if lower.contains("red") {
        return [0.8, 0.2, 0.2];
    }
    if lower.contains("blue") {
        return [0.2, 0.3, 0.8];
    }
    if lower.contains("green") {
        return [0.2, 0.7, 0.3];
    }
    if lower.contains("wood") || lower.contains("oak") || lower.contains("walnut") {
        return [0.6, 0.45, 0.3];
    }
    if lower.contains("metal") || lower.contains("chrome") || lower.contains("aluminium") {
        return [0.7, 0.72, 0.75];
    }
    if lower.contains("melamine") || lower.contains("hpl") {
        return [0.9, 0.88, 0.85];
    }

    // Default neutral color
    [0.7, 0.7, 0.7]
}

/// Create a box mesh
/// In OFML, boxes are positioned by their corner (0,0,0) and extend to (width, height, depth)
fn create_box_mesh(
    name: &str,
    width: f32,
    height: f32,
    depth: f32,
    pos: [f32; 3],
    rot: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};

    // Create 8 corners from origin (0,0,0) to (width, height, depth)
    // This matches OFML OiBlock semantics where position is the corner, not center
    let corners = [
        [0.0, 0.0, 0.0],
        [width, 0.0, 0.0],
        [width, height, 0.0],
        [0.0, height, 0.0],
        [0.0, 0.0, depth],
        [width, 0.0, depth],
        [width, height, depth],
        [0.0, height, depth],
    ];

    // Apply rotation and translation
    let vertices: Vec<Vertex> = corners
        .iter()
        .map(|c| {
            let rotated = rotate_point(*c, rot);
            Vertex {
                x: rotated[0] + pos[0],
                y: rotated[1] + pos[1],
                z: rotated[2] + pos[2],
            }
        })
        .collect();

    // Create faces (2 triangles per face, 6 faces)
    let faces = vec![
        // Front
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
        // Back
        Face {
            a: 5,
            b: 4,
            c: 7,
            flags: 0,
        },
        Face {
            a: 5,
            b: 7,
            c: 6,
            flags: 0,
        },
        // Left
        Face {
            a: 4,
            b: 0,
            c: 3,
            flags: 0,
        },
        Face {
            a: 4,
            b: 3,
            c: 7,
            flags: 0,
        },
        // Right
        Face {
            a: 1,
            b: 5,
            c: 6,
            flags: 0,
        },
        Face {
            a: 1,
            b: 6,
            c: 2,
            flags: 0,
        },
        // Top
        Face {
            a: 3,
            b: 2,
            c: 6,
            flags: 0,
        },
        Face {
            a: 3,
            b: 6,
            c: 7,
            flags: 0,
        },
        // Bottom
        Face {
            a: 4,
            b: 5,
            c: 1,
            flags: 0,
        },
        Face {
            a: 4,
            b: 1,
            c: 0,
            flags: 0,
        },
    ];

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf, // Already in GLTF coords, no conversion needed
    }
}

/// Create a cylinder mesh
fn create_cylinder_mesh(
    name: &str,
    radius: f32,
    height: f32,
    segments: u16,
    pos: [f32; 3],
    _rot: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};
    use std::f32::consts::PI;

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Bottom center
    vertices.push(Vertex {
        x: pos[0],
        y: pos[1],
        z: pos[2],
    });
    // Top center
    vertices.push(Vertex {
        x: pos[0],
        y: pos[1] + height,
        z: pos[2],
    });

    // Create ring vertices
    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * 2.0 * PI;
        let x = radius * angle.cos();
        let z = radius * angle.sin();

        // Bottom ring
        vertices.push(Vertex {
            x: pos[0] + x,
            y: pos[1],
            z: pos[2] + z,
        });
        // Top ring
        vertices.push(Vertex {
            x: pos[0] + x,
            y: pos[1] + height,
            z: pos[2] + z,
        });
    }

    // Create faces
    for i in 0..segments {
        let b1 = 2 + i * 2;
        let t1 = 3 + i * 2;
        let b2 = 2 + ((i + 1) % segments) * 2;
        let t2 = 3 + ((i + 1) % segments) * 2;

        // Bottom face
        faces.push(Face {
            a: 0,
            b: b2,
            c: b1,
            flags: 0,
        });
        // Top face
        faces.push(Face {
            a: 1,
            b: t1,
            c: t2,
            flags: 0,
        });
        // Side faces
        faces.push(Face {
            a: b1,
            b: b2,
            c: t2,
            flags: 0,
        });
        faces.push(Face {
            a: b1,
            b: t2,
            c: t1,
            flags: 0,
        });
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf, // Already in GLTF coords, no conversion needed
    }
}

/// Create a sphere mesh (simplified - uses icosphere approach)
fn create_sphere_mesh(
    name: &str,
    radius: f32,
    lat_segments: u16,
    lon_segments: u16,
    pos: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};
    use std::f32::consts::PI;

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Create vertices
    for lat in 0..=lat_segments {
        let theta = (lat as f32 / lat_segments as f32) * PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for lon in 0..=lon_segments {
            let phi = (lon as f32 / lon_segments as f32) * 2.0 * PI;
            let x = radius * sin_theta * phi.cos();
            let y = radius * cos_theta;
            let z = radius * sin_theta * phi.sin();

            vertices.push(Vertex {
                x: pos[0] + x,
                y: pos[1] + y,
                z: pos[2] + z,
            });
        }
    }

    // Create faces
    for lat in 0..lat_segments {
        for lon in 0..lon_segments {
            let first = lat * (lon_segments + 1) + lon;
            let second = first + lon_segments + 1;

            faces.push(Face {
                a: first,
                b: second,
                c: first + 1,
                flags: 0,
            });
            faces.push(Face {
                a: second,
                b: second + 1,
                c: first + 1,
                flags: 0,
            });
        }
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf, // Already in GLTF coords, no conversion needed
    }
}

/// Create an ellipsoid mesh
fn create_ellipsoid_mesh(
    name: &str,
    rx: f32,
    ry: f32,
    rz: f32,
    lat_segments: u16,
    lon_segments: u16,
    pos: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};
    use std::f32::consts::PI;

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Create vertices - same as sphere but with different radii per axis
    for lat in 0..=lat_segments {
        let theta = (lat as f32 / lat_segments as f32) * PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for lon in 0..=lon_segments {
            let phi = (lon as f32 / lon_segments as f32) * 2.0 * PI;
            let x = rx * sin_theta * phi.cos();
            let y = ry * cos_theta;
            let z = rz * sin_theta * phi.sin();

            vertices.push(Vertex {
                x: pos[0] + x,
                y: pos[1] + y,
                z: pos[2] + z,
            });
        }
    }

    // Create faces
    for lat in 0..lat_segments {
        for lon in 0..lon_segments {
            let first = lat * (lon_segments + 1) + lon;
            let second = first + lon_segments + 1;

            faces.push(Face {
                a: first,
                b: second,
                c: first + 1,
                flags: 0,
            });
            faces.push(Face {
                a: second,
                b: second + 1,
                c: first + 1,
                flags: 0,
            });
        }
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    }
}

/// Create a polygon mesh (extruded 2D polygon)
fn create_polygon_mesh(
    name: &str,
    vertices_2d: &[[f32; 2]],
    thickness: f32,
    pos: [f32; 3],
    _rot: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    if vertices_2d.len() < 3 {
        return Mesh {
            name: name.to_string(),
            vertices,
            normals: Vec::new(),
            faces,
            tex_coords: vec![],
            material_name: None,
            transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            coord_system: CoordSystem::YupGltf,
        };
    }

    let n = vertices_2d.len();

    // Bottom face vertices
    for v in vertices_2d {
        vertices.push(Vertex {
            x: pos[0] + v[0],
            y: pos[1],
            z: pos[2] + v[1],
        });
    }

    // Top face vertices
    for v in vertices_2d {
        vertices.push(Vertex {
            x: pos[0] + v[0],
            y: pos[1] + thickness,
            z: pos[2] + v[1],
        });
    }

    // Bottom face (triangle fan)
    for i in 1..(n - 1) {
        faces.push(Face {
            a: 0,
            b: (i + 1) as u16,
            c: i as u16,
            flags: 0,
        });
    }

    // Top face (triangle fan)
    for i in 1..(n - 1) {
        faces.push(Face {
            a: n as u16,
            b: (n + i) as u16,
            c: (n + i + 1) as u16,
            flags: 0,
        });
    }

    // Side faces
    for i in 0..n {
        let next = (i + 1) % n;
        let b0 = i as u16;
        let b1 = next as u16;
        let t0 = (n + i) as u16;
        let t1 = (n + next) as u16;

        faces.push(Face {
            a: b0,
            b: b1,
            c: t0,
            flags: 0,
        });
        faces.push(Face {
            a: t0,
            b: b1,
            c: t1,
            flags: 0,
        });
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    }
}

/// Create a frame mesh (rectangular with a hole)
fn create_frame_mesh(
    name: &str,
    outer_width: f32,
    outer_height: f32,
    inner_width: f32,
    inner_height: f32,
    depth: f32,
    pos: [f32; 3],
    _rot: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Calculate inner offsets (centered hole)
    let inner_offset_x = (outer_width - inner_width) / 2.0;
    let inner_offset_y = (outer_height - inner_height) / 2.0;

    // Outer rectangle vertices (bottom and top)
    let outer_verts = [
        [0.0, 0.0],
        [outer_width, 0.0],
        [outer_width, outer_height],
        [0.0, outer_height],
    ];

    // Inner rectangle vertices (bottom and top)
    let inner_verts = [
        [inner_offset_x, inner_offset_y],
        [inner_offset_x + inner_width, inner_offset_y],
        [inner_offset_x + inner_width, inner_offset_y + inner_height],
        [inner_offset_x, inner_offset_y + inner_height],
    ];

    // Create vertices: outer bottom, inner bottom, outer top, inner top
    for v in &outer_verts {
        vertices.push(Vertex {
            x: pos[0] + v[0],
            y: pos[1],
            z: pos[2] + v[1],
        });
    }
    for v in &inner_verts {
        vertices.push(Vertex {
            x: pos[0] + v[0],
            y: pos[1],
            z: pos[2] + v[1],
        });
    }
    for v in &outer_verts {
        vertices.push(Vertex {
            x: pos[0] + v[0],
            y: pos[1] + depth,
            z: pos[2] + v[1],
        });
    }
    for v in &inner_verts {
        vertices.push(Vertex {
            x: pos[0] + v[0],
            y: pos[1] + depth,
            z: pos[2] + v[1],
        });
    }

    // Bottom face (4 quads around the hole)
    // Each segment connects outer edge to inner edge
    for i in 0..4 {
        let o0 = i as u16;
        let o1 = ((i + 1) % 4) as u16;
        let i0 = (4 + i) as u16;
        let i1 = (4 + (i + 1) % 4) as u16;
        faces.push(Face {
            a: o0,
            b: o1,
            c: i0,
            flags: 0,
        });
        faces.push(Face {
            a: i0,
            b: o1,
            c: i1,
            flags: 0,
        });
    }

    // Top face (4 quads around the hole)
    for i in 0..4 {
        let o0 = (8 + i) as u16;
        let o1 = (8 + (i + 1) % 4) as u16;
        let i0 = (12 + i) as u16;
        let i1 = (12 + (i + 1) % 4) as u16;
        faces.push(Face {
            a: o0,
            b: i0,
            c: o1,
            flags: 0,
        });
        faces.push(Face {
            a: i0,
            b: i1,
            c: o1,
            flags: 0,
        });
    }

    // Outer side faces
    for i in 0..4 {
        let b0 = i as u16;
        let b1 = ((i + 1) % 4) as u16;
        let t0 = (8 + i) as u16;
        let t1 = (8 + (i + 1) % 4) as u16;
        faces.push(Face {
            a: b0,
            b: t0,
            c: b1,
            flags: 0,
        });
        faces.push(Face {
            a: t0,
            b: t1,
            c: b1,
            flags: 0,
        });
    }

    // Inner side faces (facing inward)
    for i in 0..4 {
        let b0 = (4 + i) as u16;
        let b1 = (4 + (i + 1) % 4) as u16;
        let t0 = (12 + i) as u16;
        let t1 = (12 + (i + 1) % 4) as u16;
        faces.push(Face {
            a: b0,
            b: b1,
            c: t0,
            flags: 0,
        });
        faces.push(Face {
            a: t0,
            b: b1,
            c: t1,
            flags: 0,
        });
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    }
}

/// Create a rotation (lathe) mesh
fn create_rotation_mesh(
    name: &str,
    profile: &[[f32; 2]],
    segments: u32,
    angle: f32,
    pos: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    if profile.len() < 2 || segments == 0 {
        return Mesh {
            name: name.to_string(),
            vertices,
            normals: Vec::new(),
            faces,
            tex_coords: vec![],
            material_name: None,
            transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            coord_system: CoordSystem::YupGltf,
        };
    }

    let n_profile = profile.len();

    // Generate vertices by rotating profile around Y axis
    for seg in 0..=segments {
        let theta = (seg as f32 / segments as f32) * angle;
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for pt in profile {
            let r = pt[0]; // radial distance
            let y = pt[1]; // height

            vertices.push(Vertex {
                x: pos[0] + r * cos_t,
                y: pos[1] + y,
                z: pos[2] + r * sin_t,
            });
        }
    }

    // Generate faces
    for seg in 0..segments {
        for i in 0..(n_profile - 1) {
            let base = (seg as usize) * n_profile + i;
            let next_seg = ((seg + 1) as usize) * n_profile + i;

            faces.push(Face {
                a: base as u16,
                b: next_seg as u16,
                c: (base + 1) as u16,
                flags: 0,
            });
            faces.push(Face {
                a: next_seg as u16,
                b: (next_seg + 1) as u16,
                c: (base + 1) as u16,
                flags: 0,
            });
        }
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    }
}

/// Create a sweep (extrusion) mesh
fn create_sweep_mesh(
    name: &str,
    profile: &[[f32; 2]],
    path: &[[f32; 3]],
    _scale_along_path: bool,
    pos: [f32; 3],
) -> Mesh {
    use crate::geometry::{Face, Vertex};

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    if profile.len() < 3 || path.len() < 2 {
        return Mesh {
            name: name.to_string(),
            vertices,
            normals: Vec::new(),
            faces,
            tex_coords: vec![],
            material_name: None,
            transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            coord_system: CoordSystem::YupGltf,
        };
    }

    let n_profile = profile.len();
    let n_path = path.len();

    // Simple extrusion: place profile at each path point
    // (In a full implementation, we'd compute proper normals and banking)
    for (path_idx, path_pt) in path.iter().enumerate() {
        // Calculate tangent direction
        let tangent = if path_idx == 0 {
            let next = path[1];
            [
                next[0] - path_pt[0],
                next[1] - path_pt[1],
                next[2] - path_pt[2],
            ]
        } else if path_idx == n_path - 1 {
            let prev = path[path_idx - 1];
            [
                path_pt[0] - prev[0],
                path_pt[1] - prev[1],
                path_pt[2] - prev[2],
            ]
        } else {
            let prev = path[path_idx - 1];
            let next = path[path_idx + 1];
            [
                (next[0] - prev[0]) / 2.0,
                (next[1] - prev[1]) / 2.0,
                (next[2] - prev[2]) / 2.0,
            ]
        };

        // Normalize tangent
        let len =
            (tangent[0] * tangent[0] + tangent[1] * tangent[1] + tangent[2] * tangent[2]).sqrt();
        let tangent = if len > 0.0001 {
            [tangent[0] / len, tangent[1] / len, tangent[2] / len]
        } else {
            [0.0, 1.0, 0.0]
        };

        // Calculate perpendicular vectors (simple approach)
        // Use cross product with up vector
        let up = [0.0, 1.0, 0.0];
        let right = [
            tangent[1] * up[2] - tangent[2] * up[1],
            tangent[2] * up[0] - tangent[0] * up[2],
            tangent[0] * up[1] - tangent[1] * up[0],
        ];
        let right_len = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
        let right = if right_len > 0.0001 {
            [
                right[0] / right_len,
                right[1] / right_len,
                right[2] / right_len,
            ]
        } else {
            [1.0, 0.0, 0.0]
        };

        // Calculate actual up vector
        let actual_up = [
            tangent[1] * right[2] - tangent[2] * right[1],
            tangent[2] * right[0] - tangent[0] * right[2],
            tangent[0] * right[1] - tangent[1] * right[0],
        ];

        // Place profile vertices
        for prof_pt in profile {
            let x = path_pt[0] + prof_pt[0] * right[0] + prof_pt[1] * actual_up[0];
            let y = path_pt[1] + prof_pt[0] * right[1] + prof_pt[1] * actual_up[1];
            let z = path_pt[2] + prof_pt[0] * right[2] + prof_pt[1] * actual_up[2];

            vertices.push(Vertex {
                x: pos[0] + x,
                y: pos[1] + y,
                z: pos[2] + z,
            });
        }
    }

    // Generate faces connecting adjacent profile rings
    for path_idx in 0..(n_path - 1) {
        for i in 0..n_profile {
            let next_i = (i + 1) % n_profile;

            let base = path_idx * n_profile + i;
            let base_next = path_idx * n_profile + next_i;
            let top = (path_idx + 1) * n_profile + i;
            let top_next = (path_idx + 1) * n_profile + next_i;

            faces.push(Face {
                a: base as u16,
                b: top as u16,
                c: base_next as u16,
                flags: 0,
            });
            faces.push(Face {
                a: top as u16,
                b: top_next as u16,
                c: base_next as u16,
                flags: 0,
            });
        }
    }

    // Cap the ends (simple triangle fan)
    // Start cap
    for i in 1..(n_profile - 1) {
        faces.push(Face {
            a: 0,
            b: (i + 1) as u16,
            c: i as u16,
            flags: 0,
        });
    }

    // End cap
    let end_base = (n_path - 1) * n_profile;
    for i in 1..(n_profile - 1) {
        faces.push(Face {
            a: end_base as u16,
            b: (end_base + i) as u16,
            c: (end_base + i + 1) as u16,
            flags: 0,
        });
    }

    Mesh {
        name: name.to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: vec![],
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_graph_basic() {
        let mut scene = SceneGraph::new();

        let root = scene.create_part("desk".to_string(), None);
        let plate = scene.create_part("plate".to_string(), Some(root.clone()));
        let _geo = scene.create_block("geo".to_string(), [1.6, 0.025, 0.8], Some(plate.clone()));

        assert_eq!(scene.nodes.len(), 3);
        assert!(scene.exists("desk.plate.geo"));
    }

    #[test]
    fn test_world_position() {
        let mut scene = SceneGraph::new();

        let root = scene.create_part("root".to_string(), None);
        root.borrow_mut().set_position([1.0, 0.0, 0.0]);

        let child = scene.create_part("child".to_string(), Some(root.clone()));
        child.borrow_mut().set_position([0.0, 1.0, 0.0]);

        let world_pos = child.borrow().get_world_position();
        assert!((world_pos[0] - 1.0).abs() < 0.001);
        assert!((world_pos[1] - 1.0).abs() < 0.001);
    }
}
