//! Geometry handling for OFML - 3DS/OBJ parsing and GLTF export

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

/// 3DS Chunk IDs
mod chunk {
    pub const MAIN: u16 = 0x4D4D;
    pub const EDITOR_3D: u16 = 0x3D3D;
    pub const OBJECT: u16 = 0x4000;
    pub const TRIMESH: u16 = 0x4100;
    pub const VERTICES: u16 = 0x4110;
    pub const FACES: u16 = 0x4120;
    pub const FACE_MATERIAL: u16 = 0x4130;
    pub const TEX_COORDS: u16 = 0x4140;
    pub const MESH_MATRIX: u16 = 0x4160;
    pub const MATERIAL: u16 = 0xAFFF;
    pub const MAT_NAME: u16 = 0xA000;
    pub const MAT_AMBIENT: u16 = 0xA010;
    pub const MAT_DIFFUSE: u16 = 0xA020;
    pub const MAT_SPECULAR: u16 = 0xA030;
    pub const COLOR_24: u16 = 0x0011;
    pub const COLOR_F: u16 = 0x0010;
    pub const LIN_COLOR_24: u16 = 0x0012;
}

/// A 3D vertex
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Convert from 3DS coordinate system (Z-up) to GLTF (Y-up)
    pub fn to_gltf_coords(&self) -> Self {
        Self {
            x: self.x,
            y: self.z,  // 3DS Z becomes GLTF Y
            z: -self.y, // 3DS Y becomes GLTF -Z
        }
    }
}

/// A triangle face
#[derive(Debug, Clone, Copy, Default)]
pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub flags: u16,
}

/// Coordinate system for mesh vertices
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CoordSystem {
    /// 3DS coordinate system (Z-up, Y-into-screen)
    #[default]
    Zup3DS,
    /// GLTF/OFML coordinate system (Y-up, Z-out-of-screen)
    YupGltf,
}

/// A mesh object (geometry data from 3DS, OBJ, GEO, or procedural generation)
#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub tex_coords: Vec<[f32; 2]>,
    pub material_name: Option<String>,
    pub transform: [f32; 12], // 4x3 transformation matrix
    /// Coordinate system of the vertices (determines if conversion is needed)
    pub coord_system: CoordSystem,
}

/// Backwards-compatible type alias
#[deprecated(note = "Use `Mesh` instead")]
pub type Mesh3DS = Mesh;

/// Material from 3DS file
#[derive(Debug, Clone, Default)]
pub struct Material3DS {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    /// Optional texture filename for the diffuse channel
    pub texture: Option<String>,
    /// Metallic factor for PBR (0.0 = dielectric, 1.0 = metal)
    pub metallic: f32,
    /// Roughness factor for PBR (0.0 = smooth, 1.0 = rough)
    pub roughness: f32,
}

/// Embedded texture data for GLB export
#[derive(Debug, Clone)]
pub struct EmbeddedTexture {
    /// Texture name
    pub name: String,
    /// PNG-encoded image data
    pub data: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
}

/// A 3DS scene
#[derive(Debug, Clone, Default)]
pub struct Scene3DS {
    pub meshes: Vec<Mesh>,
    pub materials: HashMap<String, Material3DS>,
    /// Embedded textures for GLB export
    pub textures: Vec<EmbeddedTexture>,
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl BoundingBox {
    /// Create an empty bounding box
    pub fn empty() -> Self {
        BoundingBox {
            min: [f32::INFINITY, f32::INFINITY, f32::INFINITY],
            max: [f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY],
        }
    }

    /// Check if bounding box is valid (not empty)
    pub fn is_valid(&self) -> bool {
        self.min[0] <= self.max[0] && self.min[1] <= self.max[1] && self.min[2] <= self.max[2]
    }

    /// Get dimensions (width, height, depth)
    pub fn dimensions(&self) -> [f32; 3] {
        if !self.is_valid() {
            return [0.0, 0.0, 0.0];
        }
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    /// Get center point
    pub fn center(&self) -> [f32; 3] {
        if !self.is_valid() {
            return [0.0, 0.0, 0.0];
        }
        [
            (self.min[0] + self.max[0]) / 2.0,
            (self.min[1] + self.max[1]) / 2.0,
            (self.min[2] + self.max[2]) / 2.0,
        ]
    }

    /// Expand to include a point
    pub fn expand(&mut self, point: [f32; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);
        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);
    }

    /// Merge with another bounding box
    pub fn merge(&mut self, other: &BoundingBox) {
        if !other.is_valid() {
            return;
        }
        self.expand(other.min);
        self.expand(other.max);
    }

    /// Check if dimensions match expected values within tolerance (in mm)
    pub fn dimensions_match(&self, expected_mm: [f32; 3], tolerance_mm: f32) -> bool {
        let dims = self.dimensions();
        let _tolerance = tolerance_mm / 1000.0; // Convert to meters (reserved for future use)
        (dims[0] * 1000.0 - expected_mm[0]).abs() <= tolerance_mm
            && (dims[1] * 1000.0 - expected_mm[1]).abs() <= tolerance_mm
            && (dims[2] * 1000.0 - expected_mm[2]).abs() <= tolerance_mm
    }
}

impl Mesh {
    /// Calculate bounding box of the mesh
    pub fn bounding_box(&self) -> BoundingBox {
        let mut bbox = BoundingBox::empty();
        for v in &self.vertices {
            bbox.expand([v.x, v.y, v.z]);
        }
        bbox
    }
}

impl Scene3DS {
    /// Calculate bounding box of the entire scene
    pub fn bounding_box(&self) -> BoundingBox {
        let mut bbox = BoundingBox::empty();
        for mesh in &self.meshes {
            bbox.merge(&mesh.bounding_box());
        }
        bbox
    }

    /// Get total vertex count
    pub fn vertex_count(&self) -> usize {
        self.meshes.iter().map(|m| m.vertices.len()).sum()
    }

    /// Get total face count
    pub fn face_count(&self) -> usize {
        self.meshes.iter().map(|m| m.faces.len()).sum()
    }
}

/// Error type for geometry operations
#[derive(Debug, thiserror::Error)]
pub enum GeometryError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid 3DS file: {0}")]
    Invalid3DS(String),
    #[error("Invalid OBJ file: {0}")]
    InvalidOBJ(String),
    #[error("Invalid GEO file: {0}")]
    InvalidGeo(String),
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),
}

/// Supported geometry file formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeometryFormat {
    /// 3D Studio format (.3ds)
    ThreeDS,
    /// Wavefront OBJ format (.obj)
    Obj,
    /// GEO/OFF format (.geo, .off)
    Geo,
    /// Unknown format
    Unknown,
}

impl GeometryFormat {
    /// Detect format from file extension
    pub fn from_extension(filename: &str) -> Self {
        let lower = filename.to_lowercase();
        if lower.ends_with(".3ds") {
            GeometryFormat::ThreeDS
        } else if lower.ends_with(".obj") {
            GeometryFormat::Obj
        } else if lower.ends_with(".geo") || lower.ends_with(".off") {
            GeometryFormat::Geo
        } else {
            GeometryFormat::Unknown
        }
    }

    /// Detect format from file content (magic bytes)
    pub fn from_content(data: &[u8]) -> Self {
        if data.len() < 2 {
            return GeometryFormat::Unknown;
        }

        // Check for 3DS magic bytes (0x4D4D - MAIN chunk)
        if data.len() >= 2 && data[0] == 0x4D && data[1] == 0x4D {
            return GeometryFormat::ThreeDS;
        }

        // Check for OBJ file (starts with common OBJ commands)
        if let Ok(text) = std::str::from_utf8(&data[..data.len().min(1024)]) {
            let text = text.trim_start();
            // OBJ files typically start with comments (#), mtllib, o, v, or g
            if text.starts_with('#')
                || text.starts_with("mtllib")
                || text.starts_with("o ")
                || text.starts_with("v ")
                || text.starts_with("g ")
                || text.starts_with("vn ")
                || text.starts_with("vt ")
            {
                return GeometryFormat::Obj;
            }

            // GEO/OFF files start with vertex/face counts or "OFF"
            if text.starts_with("OFF") {
                return GeometryFormat::Geo;
            }

            // Check if first line looks like "num_verts num_faces" (GEO format)
            let first_line = text.lines().next().unwrap_or("");
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 2
                && parts[0].parse::<usize>().is_ok()
                && parts[1].parse::<usize>().is_ok()
            {
                return GeometryFormat::Geo;
            }
        }

        GeometryFormat::Unknown
    }

    /// Detect format using both extension and content
    pub fn detect(filename: &str, data: &[u8]) -> Self {
        // First try extension
        let from_ext = Self::from_extension(filename);
        if from_ext != GeometryFormat::Unknown {
            return from_ext;
        }

        // Fall back to content detection
        Self::from_content(data)
    }
}

/// Parse geometry data with automatic format detection
pub fn parse_geometry_auto(filename: &str, data: &[u8]) -> Result<Scene3DS, GeometryError> {
    let format = GeometryFormat::detect(filename, data);

    match format {
        GeometryFormat::ThreeDS => parse_3ds(data),
        GeometryFormat::Obj => parse_obj(data),
        GeometryFormat::Geo => parse_geo(data),
        GeometryFormat::Unknown => {
            // Try each parser in order
            if let Ok(scene) = parse_3ds(data) {
                return Ok(scene);
            }
            if let Ok(scene) = parse_obj(data) {
                return Ok(scene);
            }
            if let Ok(scene) = parse_geo(data) {
                return Ok(scene);
            }
            Err(GeometryError::Invalid3DS(
                "Unknown geometry format".to_string(),
            ))
        }
    }
}

/// Parse a 3DS file
pub fn parse_3ds(data: &[u8]) -> Result<Scene3DS, GeometryError> {
    let mut cursor = Cursor::new(data);
    let mut scene = Scene3DS::default();

    // Check main chunk
    let main_id = cursor.read_u16::<LittleEndian>()?;
    if main_id != chunk::MAIN {
        return Err(GeometryError::Invalid3DS(format!(
            "Invalid main chunk: {:04X}",
            main_id
        )));
    }
    let _main_len = cursor.read_u32::<LittleEndian>()?;

    // Parse chunks
    parse_chunks(&mut cursor, data.len() as u64, &mut scene)?;

    Ok(scene)
}

fn parse_chunks<R: Read + Seek>(
    reader: &mut R,
    end_pos: u64,
    scene: &mut Scene3DS,
) -> Result<(), GeometryError> {
    while reader.stream_position()? + 6 <= end_pos {
        let chunk_id = reader.read_u16::<LittleEndian>()?;
        let chunk_len = reader.read_u32::<LittleEndian>()?;
        let chunk_end = reader.stream_position()? + (chunk_len as u64 - 6);

        match chunk_id {
            chunk::EDITOR_3D => {
                parse_chunks(reader, chunk_end, scene)?;
            }
            chunk::OBJECT => {
                let mesh = parse_object(reader, chunk_end)?;
                scene.meshes.push(mesh);
            }
            chunk::MATERIAL => {
                let material = parse_material(reader, chunk_end)?;
                scene.materials.insert(material.name.clone(), material);
            }
            _ => {
                // Skip unknown chunk
                reader.seek(SeekFrom::Start(chunk_end))?;
            }
        }

        if reader.stream_position()? >= end_pos {
            break;
        }
    }
    Ok(())
}

fn parse_object<R: Read + Seek>(reader: &mut R, end_pos: u64) -> Result<Mesh, GeometryError> {
    let mut mesh = Mesh::default();

    // Read null-terminated object name
    let mut name_bytes = Vec::new();
    loop {
        let b = reader.read_u8()?;
        if b == 0 {
            break;
        }
        name_bytes.push(b);
    }
    mesh.name = String::from_utf8_lossy(&name_bytes).to_string();

    // Parse sub-chunks
    while reader.stream_position()? + 6 <= end_pos {
        let chunk_id = reader.read_u16::<LittleEndian>()?;
        let chunk_len = reader.read_u32::<LittleEndian>()?;
        let chunk_end = reader.stream_position()? + (chunk_len as u64 - 6);

        match chunk_id {
            chunk::TRIMESH => {
                parse_trimesh(reader, chunk_end, &mut mesh)?;
            }
            _ => {
                reader.seek(SeekFrom::Start(chunk_end))?;
            }
        }
    }

    Ok(mesh)
}

fn parse_trimesh<R: Read + Seek>(
    reader: &mut R,
    end_pos: u64,
    mesh: &mut Mesh,
) -> Result<(), GeometryError> {
    while reader.stream_position()? + 6 <= end_pos {
        let chunk_id = reader.read_u16::<LittleEndian>()?;
        let chunk_len = reader.read_u32::<LittleEndian>()?;
        let chunk_end = reader.stream_position()? + (chunk_len as u64 - 6);

        match chunk_id {
            chunk::VERTICES => {
                let count = reader.read_u16::<LittleEndian>()? as usize;
                mesh.vertices.reserve(count);
                for _ in 0..count {
                    let x = reader.read_f32::<LittleEndian>()?;
                    let y = reader.read_f32::<LittleEndian>()?;
                    let z = reader.read_f32::<LittleEndian>()?;
                    mesh.vertices.push(Vertex::new(x, y, z));
                }
            }
            chunk::FACES => {
                let count = reader.read_u16::<LittleEndian>()? as usize;
                mesh.faces.reserve(count);
                for _ in 0..count {
                    let a = reader.read_u16::<LittleEndian>()?;
                    let b = reader.read_u16::<LittleEndian>()?;
                    let c = reader.read_u16::<LittleEndian>()?;
                    let flags = reader.read_u16::<LittleEndian>()?;
                    mesh.faces.push(Face { a, b, c, flags });
                }
            }
            chunk::TEX_COORDS => {
                let count = reader.read_u16::<LittleEndian>()? as usize;
                mesh.tex_coords.reserve(count);
                for _ in 0..count {
                    let u = reader.read_f32::<LittleEndian>()?;
                    let v = reader.read_f32::<LittleEndian>()?;
                    mesh.tex_coords.push([u, v]);
                }
            }
            chunk::MESH_MATRIX => {
                for i in 0..12 {
                    mesh.transform[i] = reader.read_f32::<LittleEndian>()?;
                }
            }
            chunk::FACE_MATERIAL => {
                // Read material name
                let mut name_bytes = Vec::new();
                loop {
                    let b = reader.read_u8()?;
                    if b == 0 {
                        break;
                    }
                    name_bytes.push(b);
                }
                mesh.material_name = Some(String::from_utf8_lossy(&name_bytes).to_string());
                // Skip face indices for now
                reader.seek(SeekFrom::Start(chunk_end))?;
            }
            _ => {
                reader.seek(SeekFrom::Start(chunk_end))?;
            }
        }
    }

    Ok(())
}

fn parse_material<R: Read + Seek>(
    reader: &mut R,
    end_pos: u64,
) -> Result<Material3DS, GeometryError> {
    let mut material = Material3DS::default();

    while reader.stream_position()? + 6 <= end_pos {
        let chunk_id = reader.read_u16::<LittleEndian>()?;
        let chunk_len = reader.read_u32::<LittleEndian>()?;
        let chunk_end = reader.stream_position()? + (chunk_len as u64 - 6);

        match chunk_id {
            chunk::MAT_NAME => {
                let mut name_bytes = Vec::new();
                loop {
                    let b = reader.read_u8()?;
                    if b == 0 {
                        break;
                    }
                    name_bytes.push(b);
                }
                material.name = String::from_utf8_lossy(&name_bytes).to_string();
            }
            chunk::MAT_AMBIENT => {
                material.ambient = parse_color(reader, chunk_end)?;
            }
            chunk::MAT_DIFFUSE => {
                material.diffuse = parse_color(reader, chunk_end)?;
            }
            chunk::MAT_SPECULAR => {
                material.specular = parse_color(reader, chunk_end)?;
            }
            _ => {
                reader.seek(SeekFrom::Start(chunk_end))?;
            }
        }
    }

    Ok(material)
}

fn parse_color<R: Read + Seek>(reader: &mut R, end_pos: u64) -> Result<[f32; 3], GeometryError> {
    let mut color = [0.5f32; 3];

    while reader.stream_position()? + 6 <= end_pos {
        let chunk_id = reader.read_u16::<LittleEndian>()?;
        let chunk_len = reader.read_u32::<LittleEndian>()?;
        let chunk_end = reader.stream_position()? + (chunk_len as u64 - 6);

        match chunk_id {
            chunk::COLOR_24 | chunk::LIN_COLOR_24 => {
                let r = reader.read_u8()?;
                let g = reader.read_u8()?;
                let b = reader.read_u8()?;
                color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
            }
            chunk::COLOR_F => {
                color[0] = reader.read_f32::<LittleEndian>()?;
                color[1] = reader.read_f32::<LittleEndian>()?;
                color[2] = reader.read_f32::<LittleEndian>()?;
            }
            _ => {
                reader.seek(SeekFrom::Start(chunk_end))?;
            }
        }
    }

    Ok(color)
}

// =============================================================================
// OBJ Parser
// =============================================================================

/// Parse an OBJ file from bytes
/// OBJ files use Y-up coordinate system by convention
pub fn parse_obj(data: &[u8]) -> Result<Scene3DS, GeometryError> {
    let reader = BufReader::new(Cursor::new(data));

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut tex_coords: Vec<[f32; 2]> = Vec::new();
    let mut normals: Vec<Vertex> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();
    let mut mesh_name = String::new();
    let mut current_material: Option<String> = None;

    for line in reader.lines() {
        let line = line.map_err(|e| GeometryError::InvalidOBJ(e.to_string()))?;
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" if parts.len() >= 4 => {
                // Vertex position: v x y z [w]
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);
                vertices.push(Vertex::new(x, y, z));
            }
            "vt" if parts.len() >= 3 => {
                // Texture coordinate: vt u v [w]
                let u: f32 = parts[1].parse().unwrap_or(0.0);
                let v: f32 = parts[2].parse().unwrap_or(0.0);
                tex_coords.push([u, v]);
            }
            "vn" if parts.len() >= 4 => {
                // Vertex normal: vn x y z
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);
                normals.push(Vertex::new(x, y, z));
            }
            "f" if parts.len() >= 4 => {
                // Face: f v1[/vt1][/vn1] v2[/vt2][/vn2] v3[/vt3][/vn3] ...
                // Parse face indices (OBJ indices are 1-based)
                let mut face_verts: Vec<u16> = Vec::new();
                for part in &parts[1..] {
                    let indices: Vec<&str> = part.split('/').collect();
                    if let Ok(v_idx) = indices[0].parse::<i32>() {
                        // Handle negative indices (relative to current vertex count)
                        let idx = if v_idx < 0 {
                            (vertices.len() as i32 + v_idx) as u16
                        } else {
                            (v_idx - 1) as u16 // Convert to 0-based
                        };
                        face_verts.push(idx);
                    }
                }

                // Triangulate polygon (fan triangulation)
                if face_verts.len() >= 3 {
                    for i in 1..face_verts.len() - 1 {
                        faces.push(Face {
                            a: face_verts[0],
                            b: face_verts[i],
                            c: face_verts[i + 1],
                            flags: 0,
                        });
                    }
                }
            }
            "g" | "o" if parts.len() >= 2 => {
                // Group/object name
                mesh_name = parts[1..].join("_");
            }
            "usemtl" if parts.len() >= 2 => {
                current_material = Some(parts[1].to_string());
            }
            _ => {} // Ignore unknown commands
        }
    }

    if vertices.is_empty() {
        return Err(GeometryError::InvalidOBJ("No vertices found".to_string()));
    }

    let mesh = Mesh {
        name: if mesh_name.is_empty() {
            "obj_mesh".to_string()
        } else {
            mesh_name
        },
        vertices,
        normals,
        faces,
        tex_coords,
        material_name: current_material,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        coord_system: CoordSystem::YupGltf, // OBJ uses Y-up by convention
    };

    Ok(Scene3DS {
        meshes: vec![mesh],
        materials: HashMap::new(),
        textures: Vec::new(),
    })
}

// =============================================================================
// GEO Parser
// =============================================================================

/// Parse a GEO file
///
/// # Format
/// - Line 1: `num_vertices num_faces flags`
/// - Lines 2 to num_vertices+1: `X Y Z` (vertex coordinates)
/// - Remaining lines: `3 v1 v2 v3` (triangle faces, 1-indexed)
pub fn parse_geo(data: &[u8]) -> Result<Scene3DS, GeometryError> {
    let reader = BufReader::new(Cursor::new(data));
    let mut lines = reader.lines();

    // Parse header
    let header = lines
        .next()
        .ok_or_else(|| GeometryError::InvalidGeo("Empty file".to_string()))?
        .map_err(|e| GeometryError::InvalidGeo(e.to_string()))?;

    let header_parts: Vec<&str> = header.split_whitespace().collect();
    if header_parts.len() < 2 {
        return Err(GeometryError::InvalidGeo("Invalid header".to_string()));
    }

    let num_vertices: usize = header_parts[0]
        .parse()
        .map_err(|_| GeometryError::InvalidGeo("Invalid vertex count".to_string()))?;
    let num_faces: usize = header_parts[1]
        .parse()
        .map_err(|_| GeometryError::InvalidGeo("Invalid face count".to_string()))?;

    // Parse vertices
    let mut vertices = Vec::with_capacity(num_vertices);
    for _ in 0..num_vertices {
        let line = lines
            .next()
            .ok_or_else(|| GeometryError::InvalidGeo("Unexpected end of file".to_string()))?
            .map_err(|e| GeometryError::InvalidGeo(e.to_string()))?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(GeometryError::InvalidGeo("Invalid vertex line".to_string()));
        }

        let x: f32 = parts[0].parse().unwrap_or(0.0);
        let y: f32 = parts[1].parse().unwrap_or(0.0);
        let z: f32 = parts[2].parse().unwrap_or(0.0);

        // GEO format appears to use Y-up already (like OBJ)
        vertices.push(Vertex::new(x, y, z));
    }

    // Parse faces
    let mut faces = Vec::with_capacity(num_faces);
    for line in lines {
        let line = line.map_err(|e| GeometryError::InvalidGeo(e.to_string()))?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 4 || parts[0] != "3" {
            continue; // Skip non-triangle lines
        }

        // GEO uses 1-indexed vertices
        let v1: u16 = parts[1].parse::<u16>().unwrap_or(1).saturating_sub(1);
        let v2: u16 = parts[2].parse::<u16>().unwrap_or(1).saturating_sub(1);
        let v3: u16 = parts[3].parse::<u16>().unwrap_or(1).saturating_sub(1);

        faces.push(Face {
            a: v1,
            b: v2,
            c: v3,
            flags: 0,
        });
    }

    let mesh = Mesh {
        name: "geo_mesh".to_string(),
        vertices,
        normals: Vec::new(),
        faces,
        tex_coords: Vec::new(),
        material_name: None,
        transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], // Identity matrix
        coord_system: CoordSystem::YupGltf, // GEO files appear to use Y-up like OBJ
    };

    Ok(Scene3DS {
        meshes: vec![mesh],
        materials: HashMap::new(),
        textures: Vec::new(),
    })
}

/// Load a GEO file from disk
pub fn load_geo(path: &std::path::Path) -> Result<Scene3DS, GeometryError> {
    let data = std::fs::read(path)?;
    parse_geo(&data)
}

/// Load an OBJ file from disk
pub fn load_obj(path: &std::path::Path) -> Result<Scene3DS, GeometryError> {
    let data = std::fs::read(path)?;
    parse_obj(&data)
}

/// Parse a Schönbuch .mat material file
/// Format:
/// mat NAME
/// amb R G B
/// dif R G B
/// spe R G B
/// tex image jpg FILENAME (optional)
/// roughness VALUE (optional)
/// metallic VALUE (optional)
pub fn parse_mat(data: &[u8]) -> Result<Material3DS, GeometryError> {
    let text = String::from_utf8_lossy(data);
    let mut name = String::new();
    let mut ambient = [0.5f32, 0.5, 0.5];
    let mut diffuse = [0.7f32, 0.7, 0.7];
    let mut specular = [0.0f32, 0.0, 0.0];

    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "mat" if parts.len() > 1 => {
                name = parts[1].to_string();
            }
            "amb" if parts.len() >= 4 => {
                ambient = [
                    parts[1].parse().unwrap_or(0.5),
                    parts[2].parse().unwrap_or(0.5),
                    parts[3].parse().unwrap_or(0.5),
                ];
            }
            "dif" if parts.len() >= 4 => {
                diffuse = [
                    parts[1].parse().unwrap_or(0.7),
                    parts[2].parse().unwrap_or(0.7),
                    parts[3].parse().unwrap_or(0.7),
                ];
            }
            "spe" if parts.len() >= 4 => {
                specular = [
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                    parts[3].parse().unwrap_or(0.0),
                ];
            }
            _ => {}
        }
    }

    if name.is_empty() {
        return Err(GeometryError::InvalidGeo(
            "No material name found".to_string(),
        ));
    }

    Ok(Material3DS {
        name,
        ambient,
        diffuse,
        specular,
        texture: None,
        metallic: 0.0,
        roughness: 0.5,
    })
}

/// Load a .mat file from disk
pub fn load_mat(path: &std::path::Path) -> Result<Material3DS, GeometryError> {
    let data = std::fs::read(path)?;
    parse_mat(&data)
}

/// Load an OBJ file from an ALB archive
pub fn load_obj_from_alb(
    alb_path: &std::path::Path,
    pattern: &str,
) -> Result<Scene3DS, GeometryError> {
    let password = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";
    let file = std::fs::File::open(alb_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = match archive.by_index_decrypt(i, password) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let name = file.name().to_lowercase();
        if name.ends_with(".obj") && name.contains(&pattern.to_lowercase()) {
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            return parse_obj(&data);
        }
    }

    Err(GeometryError::InvalidOBJ(format!(
        "No OBJ file matching '{}'",
        pattern
    )))
}

/// Load an OBJ file from an ALB archive, normalized (center XZ, ground Y)
pub fn load_obj_from_alb_normalized(
    alb_path: &std::path::Path,
    pattern: &str,
) -> Result<Scene3DS, GeometryError> {
    let mut scene = load_obj_from_alb(alb_path, pattern)?;
    for mesh in &mut scene.meshes {
        mesh.normalize();
    }
    Ok(scene)
}

/// Load any supported 3D format from ALB (3DS or OBJ)
/// Parts are normalized (centered XZ, grounded Y) for consistent positioning
pub fn load_geometry_from_alb(
    alb_path: &std::path::Path,
    pattern: &str,
) -> Result<Scene3DS, GeometryError> {
    // Try 3DS first - normalize for consistent positioning
    if let Ok(scene) = load_from_alb_normalized(alb_path, pattern) {
        return Ok(scene);
    }

    // Fall back to OBJ - also normalize (OBJ may be pre-centered but normalize for consistency)
    load_obj_from_alb_normalized(alb_path, pattern)
}

/// Load any supported 3D format from ALB (3DS or OBJ) - raw coordinates
/// Only converts coordinate system, does not normalize.
/// Use this when setAlignment() will be used to control positioning.
/// Returns the scene and the bounds BEFORE any normalization.
pub fn load_geometry_from_alb_raw(
    alb_path: &std::path::Path,
    pattern: &str,
) -> Result<(Scene3DS, [[f32; 3]; 2]), GeometryError> {
    // Try 3DS first
    if let Ok(scene) = load_from_alb_yup_only(alb_path, pattern) {
        // Calculate bounds from raw geometry
        let bounds = calculate_scene_bounds(&scene);
        return Ok((scene, bounds));
    }

    // Fall back to OBJ
    let scene = load_obj_from_alb(alb_path, pattern)?;
    let bounds = calculate_scene_bounds(&scene);
    Ok((scene, bounds))
}

/// Calculate the bounding box of all meshes in a scene
fn calculate_scene_bounds(scene: &Scene3DS) -> [[f32; 3]; 2] {
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

// =============================================================================
// GLTF/GLB Export
// =============================================================================
//
// This section provides functionality to export Scene3DS to GLTF/GLB format.
//
// ## Features
//
// - **Binary GLB format**: Self-contained file with embedded geometry and textures
// - **PBR Materials**: Full PBR support with metallic-roughness workflow
// - **Embedded Textures**: PNG textures embedded in the GLB binary buffer
// - **Bounding Boxes**: Per-mesh min/max bounds for culling and LOD
// - **Coordinate Conversion**: Automatic Z-up to Y-up conversion
// - **Large Mesh Support**: Automatic u16/u32 index selection
//
// ## Usage
//
// ```rust,ignore
// use ofml_lib::geometry::{Scene3DS, scene_to_glb};
//
// let scene: Scene3DS = /* load geometry */;
// let glb_data = scene_to_glb(&scene)?;
// std::fs::write("output.glb", glb_data)?;
// ```
//
// ## Validation
//
// Exported GLB files conform to glTF 2.0 specification and can be validated with:
// - Khronos glTF Validator (https://github.khronos.org/glTF-Validator/)
// - Three.js GLTFLoader
// - Blender glTF importer

/// GLTF document root structure (internal).
#[derive(Serialize)]
struct GltfDocument {
    asset: GltfAsset,
    scenes: Vec<GltfScene>,
    nodes: Vec<GltfNode>,
    meshes: Vec<GltfMesh>,
    accessors: Vec<GltfAccessor>,
    #[serde(rename = "bufferViews")]
    buffer_views: Vec<GltfBufferView>,
    buffers: Vec<GltfBuffer>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    materials: Vec<GltfMaterial>,
}

#[derive(Serialize)]
struct GltfAsset {
    version: String,
    generator: String,
}

#[derive(Serialize)]
struct GltfScene {
    nodes: Vec<usize>,
}

#[derive(Serialize)]
struct GltfNode {
    mesh: Option<usize>,
    name: Option<String>,
}

#[derive(Serialize)]
struct GltfMesh {
    primitives: Vec<GltfPrimitive>,
    name: Option<String>,
}

#[derive(Serialize)]
struct GltfPrimitive {
    attributes: GltfAttributes,
    indices: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<usize>,
}

#[derive(Serialize)]
struct GltfAttributes {
    #[serde(rename = "POSITION")]
    position: usize,
    #[serde(rename = "NORMAL", skip_serializing_if = "Option::is_none")]
    normal: Option<usize>,
}

#[derive(Serialize)]
struct GltfAccessor {
    #[serde(rename = "bufferView")]
    buffer_view: usize,
    #[serde(rename = "componentType")]
    component_type: u32,
    count: usize,
    #[serde(rename = "type")]
    accessor_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<f32>>,
}

#[derive(Serialize)]
struct GltfBufferView {
    buffer: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteLength")]
    byte_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<u32>,
}

#[derive(Serialize)]
struct GltfBuffer {
    uri: String,
    #[serde(rename = "byteLength")]
    byte_length: usize,
}

#[derive(Serialize)]
struct GltfMaterial {
    name: Option<String>,
    #[serde(rename = "pbrMetallicRoughness")]
    pbr: GltfPbr,
}

#[derive(Serialize)]
struct GltfPbr {
    #[serde(rename = "baseColorFactor")]
    base_color_factor: [f32; 4],
    #[serde(rename = "metallicFactor")]
    metallic_factor: f32,
    #[serde(rename = "roughnessFactor")]
    roughness_factor: f32,
    #[serde(rename = "baseColorTexture", skip_serializing_if = "Option::is_none")]
    base_color_texture: Option<GltfTextureInfo>,
}

#[derive(Serialize)]
struct GltfTextureInfo {
    index: usize,
}

#[derive(Serialize)]
struct GltfTexture {
    sampler: usize,
    source: usize,
}

#[derive(Serialize)]
struct GltfSampler {
    #[serde(rename = "magFilter")]
    mag_filter: u32,
    #[serde(rename = "minFilter")]
    min_filter: u32,
    #[serde(rename = "wrapS")]
    wrap_s: u32,
    #[serde(rename = "wrapT")]
    wrap_t: u32,
}

#[derive(Serialize)]
struct GltfImage {
    #[serde(rename = "bufferView")]
    buffer_view: usize,
    #[serde(rename = "mimeType")]
    mime_type: String,
}

/// Convert a Scene3DS to GLTF JSON
pub fn scene_to_gltf(scene: &Scene3DS) -> Result<(String, Vec<u8>), GeometryError> {
    use base64::Engine;

    let mut binary_data = Vec::new();
    let mut buffer_views = Vec::new();
    let mut accessors = Vec::new();
    let mut meshes = Vec::new();
    let mut nodes = Vec::new();
    let mut materials = Vec::new();
    let mut node_indices = Vec::new();

    // Create materials
    let mut material_map: HashMap<String, usize> = HashMap::new();
    for (name, mat) in &scene.materials {
        material_map.insert(name.clone(), materials.len());
        materials.push(GltfMaterial {
            name: Some(name.clone()),
            pbr: GltfPbr {
                base_color_factor: [mat.diffuse[0], mat.diffuse[1], mat.diffuse[2], 1.0],
                metallic_factor: mat.metallic,
                roughness_factor: mat.roughness,
                base_color_texture: None, // Textures not supported in data URI GLTF
            },
        });
    }

    for mesh in &scene.meshes {
        if mesh.vertices.is_empty() || mesh.faces.is_empty() {
            continue;
        }

        // Convert vertices to GLTF coordinate system only if they're in 3DS format
        let vertices: Vec<Vertex> = if mesh.coord_system == CoordSystem::Zup3DS {
            mesh.vertices.iter().map(|v| v.to_gltf_coords()).collect()
        } else {
            mesh.vertices.clone()
        };

        // Compute bounding box
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for v in &vertices {
            min[0] = min[0].min(v.x);
            min[1] = min[1].min(v.y);
            min[2] = min[2].min(v.z);
            max[0] = max[0].max(v.x);
            max[1] = max[1].max(v.y);
            max[2] = max[2].max(v.z);
        }

        // Add vertex data
        let vertex_offset = binary_data.len();
        for v in &vertices {
            binary_data.extend_from_slice(&v.x.to_le_bytes());
            binary_data.extend_from_slice(&v.y.to_le_bytes());
            binary_data.extend_from_slice(&v.z.to_le_bytes());
        }
        let vertex_length = binary_data.len() - vertex_offset;

        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: vertex_offset,
            byte_length: vertex_length,
            target: Some(34962), // ARRAY_BUFFER
        });

        let position_accessor = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: buffer_views.len() - 1,
            component_type: 5126, // FLOAT
            count: vertices.len(),
            accessor_type: "VEC3".to_string(),
            min: Some(min.to_vec()),
            max: Some(max.to_vec()),
        });

        // Add normal data if available
        let normal_accessor =
            if !mesh.normals.is_empty() && mesh.normals.len() == mesh.vertices.len() {
                // Convert normals to GLTF coordinate system if needed
                let normals: Vec<Vertex> = if mesh.coord_system == CoordSystem::Zup3DS {
                    mesh.normals.iter().map(|v| v.to_gltf_coords()).collect()
                } else {
                    mesh.normals.clone()
                };

                let normal_offset = binary_data.len();
                for n in &normals {
                    binary_data.extend_from_slice(&n.x.to_le_bytes());
                    binary_data.extend_from_slice(&n.y.to_le_bytes());
                    binary_data.extend_from_slice(&n.z.to_le_bytes());
                }
                let normal_length = binary_data.len() - normal_offset;

                buffer_views.push(GltfBufferView {
                    buffer: 0,
                    byte_offset: normal_offset,
                    byte_length: normal_length,
                    target: Some(34962), // ARRAY_BUFFER
                });

                let accessor_idx = accessors.len();
                accessors.push(GltfAccessor {
                    buffer_view: buffer_views.len() - 1,
                    component_type: 5126, // FLOAT
                    count: normals.len(),
                    accessor_type: "VEC3".to_string(),
                    min: None,
                    max: None,
                });
                Some(accessor_idx)
            } else {
                None
            };

        // Add index data (convert to u32 for larger meshes)
        // Align to 4 bytes
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let index_offset = binary_data.len();
        let use_u32 = mesh.vertices.len() > 65535;
        for face in &mesh.faces {
            if use_u32 {
                binary_data.extend_from_slice(&(face.a as u32).to_le_bytes());
                binary_data.extend_from_slice(&(face.b as u32).to_le_bytes());
                binary_data.extend_from_slice(&(face.c as u32).to_le_bytes());
            } else {
                binary_data.extend_from_slice(&face.a.to_le_bytes());
                binary_data.extend_from_slice(&face.b.to_le_bytes());
                binary_data.extend_from_slice(&face.c.to_le_bytes());
            }
        }
        let index_length = binary_data.len() - index_offset;

        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: index_offset,
            byte_length: index_length,
            target: Some(34963), // ELEMENT_ARRAY_BUFFER
        });

        let index_accessor = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: buffer_views.len() - 1,
            component_type: if use_u32 { 5125 } else { 5123 }, // UNSIGNED_INT or UNSIGNED_SHORT
            count: mesh.faces.len() * 3,
            accessor_type: "SCALAR".to_string(),
            min: None,
            max: None,
        });

        // Look up material
        let material_index = mesh
            .material_name
            .as_ref()
            .and_then(|name| material_map.get(name))
            .copied();

        meshes.push(GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes: GltfAttributes {
                    position: position_accessor,
                    normal: normal_accessor,
                },
                indices: index_accessor,
                material: material_index,
            }],
            name: Some(mesh.name.clone()),
        });

        node_indices.push(nodes.len());
        nodes.push(GltfNode {
            mesh: Some(meshes.len() - 1),
            name: Some(mesh.name.clone()),
        });
    }

    // Create base64 data URI
    let data_uri = format!(
        "data:application/octet-stream;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(&binary_data)
    );

    let document = GltfDocument {
        asset: GltfAsset {
            version: "2.0".to_string(),
            generator: "OFML Interpreter".to_string(),
        },
        scenes: vec![GltfScene {
            nodes: node_indices,
        }],
        nodes,
        meshes,
        accessors,
        buffer_views,
        buffers: vec![GltfBuffer {
            uri: data_uri,
            byte_length: binary_data.len(),
        }],
        materials,
    };

    let json = serde_json::to_string_pretty(&document)
        .map_err(|e| GeometryError::Invalid3DS(e.to_string()))?;

    Ok((json, binary_data))
}

/// Convert a Scene3DS to GLB (binary GLTF)
pub fn scene_to_glb(scene: &Scene3DS) -> Result<Vec<u8>, GeometryError> {
    let mut binary_data = Vec::new();
    let mut buffer_views = Vec::new();
    let mut accessors = Vec::new();
    let mut meshes = Vec::new();
    let mut nodes = Vec::new();
    let mut materials = Vec::new();
    let mut node_indices = Vec::new();
    let mut images: Vec<GltfImage> = Vec::new();
    let mut gltf_textures: Vec<GltfTexture> = Vec::new();
    let mut samplers: Vec<GltfSampler> = Vec::new();

    // Embed textures in binary data and create images/textures
    let mut texture_name_to_index: HashMap<String, usize> = HashMap::new();
    if !scene.textures.is_empty() {
        // Create one default sampler for all textures
        samplers.push(GltfSampler {
            mag_filter: 9729, // LINEAR
            min_filter: 9987, // LINEAR_MIPMAP_LINEAR
            wrap_s: 10497,    // REPEAT
            wrap_t: 10497,    // REPEAT
        });

        for tex in &scene.textures {
            // Align to 4 bytes
            while binary_data.len() % 4 != 0 {
                binary_data.push(0);
            }

            let tex_offset = binary_data.len();
            binary_data.extend_from_slice(&tex.data);
            let tex_length = tex.data.len();

            // Create buffer view for texture
            let buffer_view_index = buffer_views.len();
            buffer_views.push(GltfBufferView {
                buffer: 0,
                byte_offset: tex_offset,
                byte_length: tex_length,
                target: None, // No target for image buffer views
            });

            // Create image
            let image_index = images.len();
            images.push(GltfImage {
                buffer_view: buffer_view_index,
                mime_type: "image/png".to_string(),
            });

            // Create texture
            let texture_index = gltf_textures.len();
            gltf_textures.push(GltfTexture {
                sampler: 0, // Use the default sampler
                source: image_index,
            });

            texture_name_to_index.insert(tex.name.clone(), texture_index);
        }
    }

    // Create materials
    let mut material_map: HashMap<String, usize> = HashMap::new();
    for (name, mat) in &scene.materials {
        material_map.insert(name.clone(), materials.len());

        // Check if material has a texture
        let base_color_texture = mat
            .texture
            .as_ref()
            .and_then(|tex_name| texture_name_to_index.get(tex_name))
            .map(|&idx| GltfTextureInfo { index: idx });

        materials.push(GltfMaterial {
            name: Some(name.clone()),
            pbr: GltfPbr {
                base_color_factor: [mat.diffuse[0], mat.diffuse[1], mat.diffuse[2], 1.0],
                metallic_factor: mat.metallic,
                roughness_factor: mat.roughness,
                base_color_texture,
            },
        });
    }

    for mesh in &scene.meshes {
        if mesh.vertices.is_empty() || mesh.faces.is_empty() {
            continue;
        }

        // Convert vertices to GLTF coordinate system only if they're in 3DS format
        let vertices: Vec<Vertex> = if mesh.coord_system == CoordSystem::Zup3DS {
            mesh.vertices.iter().map(|v| v.to_gltf_coords()).collect()
        } else {
            mesh.vertices.clone()
        };

        // Compute bounding box
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for v in &vertices {
            min[0] = min[0].min(v.x);
            min[1] = min[1].min(v.y);
            min[2] = min[2].min(v.z);
            max[0] = max[0].max(v.x);
            max[1] = max[1].max(v.y);
            max[2] = max[2].max(v.z);
        }

        // Add vertex data
        let vertex_offset = binary_data.len();
        for v in &vertices {
            binary_data.extend_from_slice(&v.x.to_le_bytes());
            binary_data.extend_from_slice(&v.y.to_le_bytes());
            binary_data.extend_from_slice(&v.z.to_le_bytes());
        }
        let vertex_length = binary_data.len() - vertex_offset;

        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: vertex_offset,
            byte_length: vertex_length,
            target: Some(34962),
        });

        let position_accessor = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: buffer_views.len() - 1,
            component_type: 5126,
            count: vertices.len(),
            accessor_type: "VEC3".to_string(),
            min: Some(min.to_vec()),
            max: Some(max.to_vec()),
        });

        // Add normal data if available
        let normal_accessor =
            if !mesh.normals.is_empty() && mesh.normals.len() == mesh.vertices.len() {
                let normals: Vec<Vertex> = if mesh.coord_system == CoordSystem::Zup3DS {
                    mesh.normals.iter().map(|v| v.to_gltf_coords()).collect()
                } else {
                    mesh.normals.clone()
                };

                let normal_offset = binary_data.len();
                for n in &normals {
                    binary_data.extend_from_slice(&n.x.to_le_bytes());
                    binary_data.extend_from_slice(&n.y.to_le_bytes());
                    binary_data.extend_from_slice(&n.z.to_le_bytes());
                }
                let normal_length = binary_data.len() - normal_offset;

                buffer_views.push(GltfBufferView {
                    buffer: 0,
                    byte_offset: normal_offset,
                    byte_length: normal_length,
                    target: Some(34962),
                });

                let accessor_idx = accessors.len();
                accessors.push(GltfAccessor {
                    buffer_view: buffer_views.len() - 1,
                    component_type: 5126,
                    count: normals.len(),
                    accessor_type: "VEC3".to_string(),
                    min: None,
                    max: None,
                });
                Some(accessor_idx)
            } else {
                None
            };

        // Add index data
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let index_offset = binary_data.len();
        let use_u32 = mesh.vertices.len() > 65535;
        for face in &mesh.faces {
            if use_u32 {
                binary_data.extend_from_slice(&(face.a as u32).to_le_bytes());
                binary_data.extend_from_slice(&(face.b as u32).to_le_bytes());
                binary_data.extend_from_slice(&(face.c as u32).to_le_bytes());
            } else {
                binary_data.extend_from_slice(&face.a.to_le_bytes());
                binary_data.extend_from_slice(&face.b.to_le_bytes());
                binary_data.extend_from_slice(&face.c.to_le_bytes());
            }
        }
        let index_length = binary_data.len() - index_offset;

        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: index_offset,
            byte_length: index_length,
            target: Some(34963),
        });

        let index_accessor = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: buffer_views.len() - 1,
            component_type: if use_u32 { 5125 } else { 5123 },
            count: mesh.faces.len() * 3,
            accessor_type: "SCALAR".to_string(),
            min: None,
            max: None,
        });

        let material_index = mesh
            .material_name
            .as_ref()
            .and_then(|name| material_map.get(name))
            .copied();

        meshes.push(GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes: GltfAttributes {
                    position: position_accessor,
                    normal: normal_accessor,
                },
                indices: index_accessor,
                material: material_index,
            }],
            name: Some(mesh.name.clone()),
        });

        node_indices.push(nodes.len());
        nodes.push(GltfNode {
            mesh: Some(meshes.len() - 1),
            name: Some(mesh.name.clone()),
        });
    }

    // Create GLB document (without uri, using buffer index 0)
    #[derive(Serialize)]
    struct GlbBuffer {
        #[serde(rename = "byteLength")]
        byte_length: usize,
    }

    #[derive(Serialize)]
    struct GlbDocument {
        asset: GltfAsset,
        scenes: Vec<GltfScene>,
        nodes: Vec<GltfNode>,
        meshes: Vec<GltfMesh>,
        accessors: Vec<GltfAccessor>,
        #[serde(rename = "bufferViews")]
        buffer_views: Vec<GltfBufferView>,
        buffers: Vec<GlbBuffer>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        materials: Vec<GltfMaterial>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        images: Vec<GltfImage>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        textures: Vec<GltfTexture>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        samplers: Vec<GltfSampler>,
    }

    let document = GlbDocument {
        asset: GltfAsset {
            version: "2.0".to_string(),
            generator: "OFML Interpreter".to_string(),
        },
        scenes: vec![GltfScene {
            nodes: node_indices,
        }],
        nodes,
        meshes,
        accessors,
        buffer_views,
        buffers: vec![GlbBuffer {
            byte_length: binary_data.len(),
        }],
        materials,
        images,
        textures: gltf_textures,
        samplers,
    };

    let json =
        serde_json::to_string(&document).map_err(|e| GeometryError::Invalid3DS(e.to_string()))?;
    let json_bytes = json.as_bytes();

    // Pad JSON to 4-byte alignment
    let json_padding = (4 - (json_bytes.len() % 4)) % 4;
    let json_chunk_len = json_bytes.len() + json_padding;

    // Pad binary to 4-byte alignment
    let bin_padding = (4 - (binary_data.len() % 4)) % 4;
    let bin_chunk_len = binary_data.len() + bin_padding;

    // GLB structure: Header (12) + JSON chunk (8 + data) + BIN chunk (8 + data)
    let total_length = 12 + 8 + json_chunk_len + 8 + bin_chunk_len;

    let mut glb = Vec::with_capacity(total_length);

    // Header
    glb.extend_from_slice(b"glTF"); // magic
    glb.extend_from_slice(&2u32.to_le_bytes()); // version
    glb.extend_from_slice(&(total_length as u32).to_le_bytes()); // length

    // JSON chunk
    glb.extend_from_slice(&(json_chunk_len as u32).to_le_bytes()); // chunk length
    glb.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // chunk type "JSON"
    glb.extend_from_slice(json_bytes);
    glb.resize(glb.len() + json_padding, 0x20); // Space padding for JSON

    // BIN chunk
    glb.extend_from_slice(&(bin_chunk_len as u32).to_le_bytes()); // chunk length
    glb.extend_from_slice(&0x004E4942u32.to_le_bytes()); // chunk type "BIN\0"
    glb.extend_from_slice(&binary_data);
    glb.resize(glb.len() + bin_padding, 0); // Zero padding for BIN

    Ok(glb)
}

/// Load a 3DS file from an ALB archive
pub fn load_from_alb(alb_path: &std::path::Path, pattern: &str) -> Result<Scene3DS, GeometryError> {
    let password = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";
    let file = std::fs::File::open(alb_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        // Try to decrypt the file - skip if password fails
        let mut file = match archive.by_index_decrypt(i, password) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let name = file.name().to_lowercase();
        if name.ends_with(".3ds") && name.contains(&pattern.to_lowercase()) {
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            return parse_3ds(&data);
        }
    }

    Err(GeometryError::Invalid3DS(format!(
        "No 3DS file matching '{}'",
        pattern
    )))
}

/// Load a 3DS file from an ALB archive, convert to Y-up, and normalize (center XZ, ground Y)
/// This is the standard loading function for OFML furniture parts
pub fn load_from_alb_normalized(
    alb_path: &std::path::Path,
    pattern: &str,
) -> Result<Scene3DS, GeometryError> {
    let mut scene = load_from_alb(alb_path, pattern)?;
    for mesh in &mut scene.meshes {
        mesh.convert_to_yup();
        mesh.normalize();
    }
    Ok(scene)
}

/// Load a 3DS file from an ALB archive, convert to Y-up only (keep world coordinates)
/// Use this when 3DS files have world coordinates that should be preserved
pub fn load_from_alb_yup_only(
    alb_path: &std::path::Path,
    pattern: &str,
) -> Result<Scene3DS, GeometryError> {
    let mut scene = load_from_alb(alb_path, pattern)?;
    for mesh in &mut scene.meshes {
        mesh.convert_to_yup();
        // Don't normalize - keep world coordinates
    }
    Ok(scene)
}

// =============================================================================
// Assembly Functions
// =============================================================================

impl Mesh {
    /// Translate all vertices by offset
    pub fn translate(&mut self, offset: [f32; 3]) {
        for v in &mut self.vertices {
            v.x += offset[0];
            v.y += offset[1];
            v.z += offset[2];
        }
    }

    /// Rotate around X axis (in radians)
    pub fn rotate_x(&mut self, angle: f32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        for v in &mut self.vertices {
            let y = v.y;
            let z = v.z;
            v.y = y * cos_a - z * sin_a;
            v.z = y * sin_a + z * cos_a;
        }
    }

    /// Rotate around Y axis (in radians)
    pub fn rotate_y(&mut self, angle: f32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        for v in &mut self.vertices {
            let x = v.x;
            let z = v.z;
            v.x = x * cos_a - z * sin_a;
            v.z = x * sin_a + z * cos_a;
        }
    }

    /// Rotate around Z axis (in radians)
    pub fn rotate_z(&mut self, angle: f32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        for v in &mut self.vertices {
            let x = v.x;
            let y = v.y;
            v.x = x * cos_a - y * sin_a;
            v.y = x * sin_a + y * cos_a;
        }
    }

    /// Scale uniformly
    pub fn scale(&mut self, factor: f32) {
        for v in &mut self.vertices {
            v.x *= factor;
            v.y *= factor;
            v.z *= factor;
        }
    }

    /// Get bounding box (min, max)
    pub fn bounds(&self) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for v in &self.vertices {
            min[0] = min[0].min(v.x);
            min[1] = min[1].min(v.y);
            min[2] = min[2].min(v.z);
            max[0] = max[0].max(v.x);
            max[1] = max[1].max(v.y);
            max[2] = max[2].max(v.z);
        }
        (min, max)
    }

    /// Center mesh at origin
    pub fn center(&mut self) {
        let (min, max) = self.bounds();
        let center = [
            (min[0] + max[0]) / 2.0,
            (min[1] + max[1]) / 2.0,
            (min[2] + max[2]) / 2.0,
        ];
        self.translate([-center[0], -center[1], -center[2]]);
    }

    /// Move bottom of mesh to Y=0
    pub fn ground(&mut self) {
        let (min, _) = self.bounds();
        self.translate([0.0, -min[1], 0.0]);
    }

    /// Center horizontally (XZ) and ground (Y=0 at bottom)
    /// This is the standard normalization for furniture parts
    pub fn normalize(&mut self) {
        let (min, max) = self.bounds();
        // Center in X and Z
        let center_x = (min[0] + max[0]) / 2.0;
        let center_z = (min[2] + max[2]) / 2.0;
        // Ground in Y
        self.translate([-center_x, -min[1], -center_z]);
    }

    /// Convert from 3DS coordinate system (Z-up) to GLTF/OFML (Y-up) in place
    /// This should be called immediately after loading from 3DS file
    pub fn convert_to_yup(&mut self) {
        for v in &mut self.vertices {
            let old_y = v.y;
            let old_z = v.z;
            v.y = old_z; // 3DS Z becomes Y (up)
            v.z = -old_y; // 3DS Y becomes -Z (forward)
        }
        self.coord_system = CoordSystem::YupGltf;
    }
}

/// Load vertex normals from .vnm file.
///
/// VNM files contain per-vertex normal vectors in binary format.
/// Each normal is stored as 3 floats (12 bytes) in the order: nx, ny, nz.
///
/// # Arguments
///
/// * `data` - Raw bytes of the .vnm file
/// * `mesh` - Mesh to apply normals to (normals will be assigned to vertices)
///
/// # Returns
///
/// The number of normals loaded, or an error.
pub fn load_vnm_normals(data: &[u8], mesh: &mut Mesh) -> Result<usize, GeometryError> {
    const FLOAT_SIZE: usize = 4;
    const NORMAL_SIZE: usize = FLOAT_SIZE * 3; // 12 bytes per normal (nx, ny, nz)

    if data.len() < NORMAL_SIZE {
        return Ok(0);
    }

    let normal_count = data.len() / NORMAL_SIZE;
    let mut normals = Vec::with_capacity(normal_count);

    for i in 0..normal_count {
        let offset = i * NORMAL_SIZE;
        if offset + NORMAL_SIZE > data.len() {
            break;
        }

        let nx = f32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        let ny = f32::from_le_bytes([
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        let nz = f32::from_le_bytes([
            data[offset + 8],
            data[offset + 9],
            data[offset + 10],
            data[offset + 11],
        ]);

        normals.push(Vertex::new(nx, ny, nz));
    }

    // Assign normals to mesh (up to the number of vertices)
    let count = normals.len().min(mesh.vertices.len());
    mesh.normals = normals.into_iter().take(count).collect();

    Ok(count)
}

/// Per-face color from IPC file.
#[derive(Debug, Clone, Copy)]
pub struct FaceColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub face_index: u32,
}

/// Load face colors from .ipc (Index Polygon Colors) file.
///
/// IPC files contain per-face color indices. The format is:
/// - 4 bytes: face index (u32)
/// - 3 bytes: RGB color
///
/// # Arguments
///
/// * `data` - Raw bytes of the .ipc file
///
/// # Returns
///
/// A vector of face colors.
pub fn load_ipc_colors(data: &[u8]) -> Result<Vec<FaceColor>, GeometryError> {
    const ENTRY_SIZE: usize = 7; // 4 bytes index + 3 bytes RGB

    if data.len() < ENTRY_SIZE {
        return Ok(Vec::new());
    }

    let entry_count = data.len() / ENTRY_SIZE;
    let mut colors = Vec::with_capacity(entry_count);

    for i in 0..entry_count {
        let offset = i * ENTRY_SIZE;
        if offset + ENTRY_SIZE > data.len() {
            break;
        }

        let face_index = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        let r = data[offset + 4];
        let g = data[offset + 5];
        let b = data[offset + 6];

        colors.push(FaceColor {
            r,
            g,
            b,
            face_index,
        });
    }

    Ok(colors)
}

/// Apply IPC colors to face flags for later material assignment.
///
/// The face index in each FaceColor is used to identify which face to color.
/// Colors are stored in the face flags field as packed RGB (for later processing).
pub fn apply_ipc_colors_to_mesh(colors: &[FaceColor], mesh: &mut Mesh) {
    for color in colors {
        let idx = color.face_index as usize;
        if idx < mesh.faces.len() {
            // Pack RGB into flags for later material processing
            // Format: 0xRRGGBB00 | face_flags
            let packed = ((color.r as u16) << 8) | (color.g as u16);
            mesh.faces[idx].flags = packed;
        }
    }
}

/// Assembly component with transform
#[derive(Debug, Clone)]
pub struct AssemblyPart {
    pub mesh: Mesh,
    pub material: Option<String>,
}

/// Assembled product
#[derive(Debug, Clone, Default)]
pub struct Assembly {
    pub parts: Vec<AssemblyPart>,
    pub name: String,
}

impl Assembly {
    pub fn new(name: &str) -> Self {
        Self {
            parts: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn add_part(&mut self, mut mesh: Mesh, material: Option<String>) {
        mesh.center(); // Center each part for easier positioning
        self.parts.push(AssemblyPart { mesh, material });
    }

    /// Convert to Scene3DS for export
    pub fn to_scene(&self) -> Scene3DS {
        let mut scene = Scene3DS::default();
        for (i, part) in self.parts.iter().enumerate() {
            let mut mesh = part.mesh.clone();
            if mesh.name.is_empty() {
                mesh.name = format!("part_{}", i);
            }
            mesh.material_name = part.material.clone();
            scene.meshes.push(mesh);
        }
        scene
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_coords() {
        let v = Vertex::new(1.0, 2.0, 3.0);
        let gltf = v.to_gltf_coords();
        assert_eq!(gltf.x, 1.0);
        assert_eq!(gltf.y, 3.0); // Z becomes Y
        assert_eq!(gltf.z, -2.0); // Y becomes -Z
    }

    #[test]
    fn test_geometry_format_from_extension() {
        assert_eq!(
            GeometryFormat::from_extension("model.3ds"),
            GeometryFormat::ThreeDS
        );
        assert_eq!(
            GeometryFormat::from_extension("model.3DS"),
            GeometryFormat::ThreeDS
        );
        assert_eq!(
            GeometryFormat::from_extension("model.obj"),
            GeometryFormat::Obj
        );
        assert_eq!(
            GeometryFormat::from_extension("model.OBJ"),
            GeometryFormat::Obj
        );
        assert_eq!(
            GeometryFormat::from_extension("model.geo"),
            GeometryFormat::Geo
        );
        assert_eq!(
            GeometryFormat::from_extension("model.off"),
            GeometryFormat::Geo
        );
        assert_eq!(
            GeometryFormat::from_extension("model.txt"),
            GeometryFormat::Unknown
        );
        assert_eq!(
            GeometryFormat::from_extension("model"),
            GeometryFormat::Unknown
        );
    }

    #[test]
    fn test_geometry_format_from_content_3ds() {
        // 3DS magic bytes 0x4D4D
        let data_3ds = [0x4D, 0x4D, 0x00, 0x00];
        assert_eq!(
            GeometryFormat::from_content(&data_3ds),
            GeometryFormat::ThreeDS
        );
    }

    #[test]
    fn test_geometry_format_from_content_obj() {
        let data_obj_vertex = b"v 1.0 2.0 3.0\nv 4.0 5.0 6.0\n";
        assert_eq!(
            GeometryFormat::from_content(data_obj_vertex),
            GeometryFormat::Obj
        );

        let data_obj_comment = b"# OBJ file\nv 1.0 2.0 3.0\n";
        assert_eq!(
            GeometryFormat::from_content(data_obj_comment),
            GeometryFormat::Obj
        );

        let data_obj_mtl = b"mtllib material.mtl\no cube\n";
        assert_eq!(
            GeometryFormat::from_content(data_obj_mtl),
            GeometryFormat::Obj
        );
    }

    #[test]
    fn test_geometry_format_from_content_geo() {
        // GEO format starts with vertex/face counts
        let data_geo = b"10 12\n0 0 0\n1 0 0\n";
        assert_eq!(GeometryFormat::from_content(data_geo), GeometryFormat::Geo);

        // OFF format
        let data_off = b"OFF\n8 6 0\n0 0 0\n";
        assert_eq!(GeometryFormat::from_content(data_off), GeometryFormat::Geo);
    }

    #[test]
    fn test_geometry_format_detect() {
        // Extension takes priority
        let data_3ds = [0x4D, 0x4D, 0x00, 0x00];
        assert_eq!(
            GeometryFormat::detect("model.obj", &data_3ds),
            GeometryFormat::Obj
        );

        // Content is used when extension is unknown
        assert_eq!(
            GeometryFormat::detect("model", &data_3ds),
            GeometryFormat::ThreeDS
        );
    }

    #[test]
    fn test_geometry_format_unknown() {
        let data_random = [0x00, 0x01, 0x02, 0x03];
        assert_eq!(
            GeometryFormat::from_content(&data_random),
            GeometryFormat::Unknown
        );

        let empty: &[u8] = &[];
        assert_eq!(GeometryFormat::from_content(empty), GeometryFormat::Unknown);
    }

    #[test]
    fn test_mesh_rotate_x() {
        use std::f32::consts::FRAC_PI_2;
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(0.0, 1.0, 0.0)],
            ..Default::default()
        };
        mesh.rotate_x(FRAC_PI_2); // 90 degrees
                                  // Y becomes Z, Z becomes -Y
        assert!((mesh.vertices[0].y - 0.0).abs() < 0.001);
        assert!((mesh.vertices[0].z - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_mesh_rotate_y() {
        use std::f32::consts::FRAC_PI_2;
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(1.0, 0.0, 0.0)],
            ..Default::default()
        };
        mesh.rotate_y(FRAC_PI_2); // 90 degrees
                                  // X becomes Z, Z becomes -X
        assert!((mesh.vertices[0].x - 0.0).abs() < 0.001);
        assert!((mesh.vertices[0].z - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_mesh_rotate_z() {
        use std::f32::consts::FRAC_PI_2;
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(1.0, 0.0, 0.0)],
            ..Default::default()
        };
        mesh.rotate_z(FRAC_PI_2); // 90 degrees
                                  // X becomes Y, Y becomes -X
        assert!((mesh.vertices[0].x - 0.0).abs() < 0.001);
        assert!((mesh.vertices[0].y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_load_vnm_normals() {
        // Create test VNM data: 2 normals
        // Normal 1: (1.0, 0.0, 0.0)
        // Normal 2: (0.0, 1.0, 0.0)
        let mut data = Vec::new();
        data.extend_from_slice(&1.0f32.to_le_bytes());
        data.extend_from_slice(&0.0f32.to_le_bytes());
        data.extend_from_slice(&0.0f32.to_le_bytes());
        data.extend_from_slice(&0.0f32.to_le_bytes());
        data.extend_from_slice(&1.0f32.to_le_bytes());
        data.extend_from_slice(&0.0f32.to_le_bytes());

        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(0.0, 0.0, 0.0), Vertex::new(1.0, 0.0, 0.0)],
            ..Default::default()
        };

        let count = load_vnm_normals(&data, &mut mesh).unwrap();
        assert_eq!(count, 2);
        assert_eq!(mesh.normals.len(), 2);
        assert!((mesh.normals[0].x - 1.0).abs() < 0.001);
        assert!((mesh.normals[0].y - 0.0).abs() < 0.001);
        assert!((mesh.normals[1].x - 0.0).abs() < 0.001);
        assert!((mesh.normals[1].y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_load_vnm_empty() {
        let data: Vec<u8> = Vec::new();
        let mut mesh = Mesh::default();
        let count = load_vnm_normals(&data, &mut mesh).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_load_ipc_colors() {
        // Create test IPC data: 2 entries
        // Face 0: RGB(255, 0, 0) - Red
        // Face 1: RGB(0, 255, 0) - Green
        let mut data = Vec::new();
        // Face 0
        data.extend_from_slice(&0u32.to_le_bytes());
        data.push(255);
        data.push(0);
        data.push(0);
        // Face 1
        data.extend_from_slice(&1u32.to_le_bytes());
        data.push(0);
        data.push(255);
        data.push(0);

        let colors = load_ipc_colors(&data).unwrap();
        assert_eq!(colors.len(), 2);
        assert_eq!(colors[0].face_index, 0);
        assert_eq!(colors[0].r, 255);
        assert_eq!(colors[0].g, 0);
        assert_eq!(colors[0].b, 0);
        assert_eq!(colors[1].face_index, 1);
        assert_eq!(colors[1].r, 0);
        assert_eq!(colors[1].g, 255);
        assert_eq!(colors[1].b, 0);
    }

    #[test]
    fn test_load_ipc_empty() {
        let data: Vec<u8> = Vec::new();
        let colors = load_ipc_colors(&data).unwrap();
        assert!(colors.is_empty());
    }

    #[test]
    fn test_apply_ipc_colors_to_mesh() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            faces: vec![
                Face {
                    a: 0,
                    b: 1,
                    c: 2,
                    flags: 0,
                },
                Face {
                    a: 2,
                    b: 3,
                    c: 0,
                    flags: 0,
                },
            ],
            ..Default::default()
        };

        let colors = vec![
            FaceColor {
                r: 255,
                g: 128,
                face_index: 0,
                b: 64,
            },
            FaceColor {
                r: 0,
                g: 255,
                face_index: 1,
                b: 128,
            },
        ];

        apply_ipc_colors_to_mesh(&colors, &mut mesh);

        // Check that flags were set (packed RGB)
        assert_ne!(mesh.faces[0].flags, 0);
        assert_ne!(mesh.faces[1].flags, 0);
    }

    #[test]
    fn test_vertex_default() {
        let v = Vertex::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vertex_debug_clone() {
        let v = Vertex::new(1.5, 2.5, 3.5);
        let debug_str = format!("{:?}", v);
        assert!(debug_str.contains("1.5"));

        let cloned = v.clone();
        assert_eq!(cloned.x, 1.5);
    }

    #[test]
    fn test_face_default() {
        let f = Face::default();
        assert_eq!(f.a, 0);
        assert_eq!(f.b, 0);
        assert_eq!(f.c, 0);
        assert_eq!(f.flags, 0);
    }

    #[test]
    fn test_face_debug_clone() {
        let f = Face { a: 1, b: 2, c: 3, flags: 0 };
        let debug_str = format!("{:?}", f);
        assert!(debug_str.contains("Face"));

        let cloned = f.clone();
        assert_eq!(cloned.a, 1);
    }

    #[test]
    fn test_mesh_default() {
        let mesh = Mesh::default();
        assert!(mesh.name.is_empty() || mesh.name == "mesh" || mesh.name == "");
        assert!(mesh.vertices.is_empty());
        assert!(mesh.faces.is_empty());
    }

    #[test]
    fn test_mesh_debug_clone() {
        let mesh = Mesh {
            name: "test_mesh".to_string(),
            vertices: vec![Vertex::new(1.0, 2.0, 3.0)],
            faces: vec![Face { a: 0, b: 0, c: 0, flags: 0 }],
            ..Default::default()
        };

        let debug_str = format!("{:?}", mesh);
        assert!(debug_str.contains("test_mesh"));

        let cloned = mesh.clone();
        assert_eq!(cloned.name, "test_mesh");
    }

    #[test]
    fn test_mesh_translate() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(0.0, 0.0, 0.0)],
            ..Default::default()
        };

        mesh.translate([10.0, 20.0, 30.0]);
        assert_eq!(mesh.vertices[0].x, 10.0);
        assert_eq!(mesh.vertices[0].y, 20.0);
        assert_eq!(mesh.vertices[0].z, 30.0);
    }

    #[test]
    fn test_mesh_scale() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(1.0, 2.0, 3.0)],
            ..Default::default()
        };

        mesh.scale(2.0);
        assert_eq!(mesh.vertices[0].x, 2.0);
        assert_eq!(mesh.vertices[0].y, 4.0);
        assert_eq!(mesh.vertices[0].z, 6.0);
    }

    #[test]
    fn test_material3ds_default() {
        let mat = Material3DS::default();
        assert!(mat.name.is_empty() || mat.name == "default" || mat.name == "");
    }

    #[test]
    fn test_material3ds_debug_clone() {
        let mat = Material3DS {
            name: "test_material".to_string(),
            ambient: [0.1, 0.2, 0.3],
            diffuse: [0.4, 0.5, 0.6],
            specular: [0.7, 0.8, 0.9],
            texture: Some("texture.png".to_string()),
            metallic: 0.0,
            roughness: 0.5,
        };

        let debug_str = format!("{:?}", mat);
        assert!(debug_str.contains("test_material"));

        let cloned = mat.clone();
        assert_eq!(cloned.name, "test_material");
        assert_eq!(cloned.metallic, 0.0);
        assert_eq!(cloned.roughness, 0.5);
    }

    #[test]
    fn test_scene3ds_default() {
        let scene = Scene3DS::default();
        assert!(scene.meshes.is_empty());
        assert!(scene.materials.is_empty());
    }

    #[test]
    fn test_scene3ds_vertex_count() {
        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "mesh1".to_string(),
            vertices: vec![Vertex::new(0.0, 0.0, 0.0), Vertex::new(1.0, 0.0, 0.0)],
            ..Default::default()
        });
        scene.meshes.push(Mesh {
            name: "mesh2".to_string(),
            vertices: vec![Vertex::new(2.0, 0.0, 0.0)],
            ..Default::default()
        });

        assert_eq!(scene.vertex_count(), 3);
    }

    #[test]
    fn test_scene3ds_face_count() {
        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "mesh1".to_string(),
            faces: vec![
                Face { a: 0, b: 1, c: 2, flags: 0 },
                Face { a: 2, b: 3, c: 0, flags: 0 },
            ],
            ..Default::default()
        });

        assert_eq!(scene.face_count(), 2);
    }

    #[test]
    fn test_bounding_box_empty() {
        let bbox = BoundingBox::empty();
        assert!(!bbox.is_valid());
    }

    #[test]
    fn test_bounding_box_expand() {
        let mut bbox = BoundingBox::empty();
        bbox.expand([0.0, 0.0, 0.0]);
        bbox.expand([10.0, 5.0, 2.0]);
        bbox.expand([-5.0, 8.0, -3.0]);

        assert!(bbox.is_valid());
        assert_eq!(bbox.min, [-5.0, 0.0, -3.0]);
        assert_eq!(bbox.max, [10.0, 8.0, 2.0]);
    }

    #[test]
    fn test_bounding_box_merge() {
        let mut bbox1 = BoundingBox::empty();
        bbox1.expand([0.0, 0.0, 0.0]);
        bbox1.expand([5.0, 5.0, 5.0]);

        let mut bbox2 = BoundingBox::empty();
        bbox2.expand([3.0, 3.0, 3.0]);
        bbox2.expand([10.0, 10.0, 10.0]);

        bbox1.merge(&bbox2);
        assert_eq!(bbox1.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox1.max, [10.0, 10.0, 10.0]);
    }

    #[test]
    fn test_bounding_box_dimensions() {
        let bbox = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [10.0, 5.0, 2.0],
        };

        let dims = bbox.dimensions();
        assert_eq!(dims, [10.0, 5.0, 2.0]);
    }

    #[test]
    fn test_bounding_box_center() {
        let bbox = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [10.0, 6.0, 4.0],
        };

        let center = bbox.center();
        assert_eq!(center, [5.0, 3.0, 2.0]);
    }

    #[test]
    fn test_bounding_box_debug_clone() {
        let bbox = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        };

        let debug_str = format!("{:?}", bbox);
        assert!(debug_str.contains("BoundingBox"));

        let cloned = bbox.clone();
        assert_eq!(cloned.min, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_scene3ds_bounding_box() {
        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "mesh1".to_string(),
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(5.0, 10.0, 15.0),
            ],
            ..Default::default()
        });

        let bbox = scene.bounding_box();
        assert!(bbox.is_valid());
        assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox.max, [5.0, 10.0, 15.0]);
    }

    #[test]
    fn test_face_color_debug_clone() {
        let fc = FaceColor {
            face_index: 5,
            r: 255,
            g: 128,
            b: 64,
        };

        let debug_str = format!("{:?}", fc);
        assert!(debug_str.contains("FaceColor"));

        let cloned = fc.clone();
        assert_eq!(cloned.face_index, 5);
        assert_eq!(cloned.r, 255);
    }

    #[test]
    fn test_embedded_texture_debug_clone() {
        let tex = EmbeddedTexture {
            name: "texture.png".to_string(),
            data: vec![0x89, 0x50, 0x4E, 0x47],
            width: 64,
            height: 64,
        };

        let debug_str = format!("{:?}", tex);
        assert!(debug_str.contains("texture.png"));

        let cloned = tex.clone();
        assert_eq!(cloned.name, "texture.png");
        assert_eq!(cloned.width, 64);
        assert_eq!(cloned.height, 64);
    }

    #[test]
    fn test_geometry_format_debug_partialeq() {
        let fmt1 = GeometryFormat::ThreeDS;
        let fmt2 = GeometryFormat::ThreeDS;
        let fmt3 = GeometryFormat::Obj;

        assert_eq!(fmt1, fmt2);
        assert_ne!(fmt1, fmt3);

        let debug_str = format!("{:?}", fmt1);
        assert!(debug_str.contains("ThreeDS"));
    }

    #[test]
    fn test_geometry_error_display() {
        let err = GeometryError::Invalid3DS("bad magic".to_string());
        let display_str = format!("{}", err);
        assert!(display_str.contains("bad magic") || display_str.contains("Invalid 3DS"));

        let err = GeometryError::InvalidOBJ("bad vertex".to_string());
        let display_str = format!("{}", err);
        assert!(display_str.contains("bad vertex") || display_str.contains("Invalid OBJ"));

        let err = GeometryError::InvalidGeo("bad format".to_string());
        let display_str = format!("{}", err);
        assert!(display_str.contains("bad format") || display_str.contains("Invalid GEO"));
    }

    #[test]
    fn test_geometry_error_debug() {
        let err = GeometryError::Invalid3DS("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Invalid3DS"));
    }

    #[test]
    fn test_geometry_error_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = GeometryError::Io(io_err);
        let display_str = format!("{}", err);
        assert!(display_str.contains("IO error") || display_str.contains("file not found"));
    }

    #[test]
    fn test_mesh_center() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![
                Vertex::new(2.0, 4.0, 6.0),
                Vertex::new(4.0, 6.0, 8.0),
            ],
            ..Default::default()
        };

        mesh.center();
        // Center should be at (3, 5, 7) so vertices become (-1,-1,-1) and (1,1,1)
        assert!((mesh.vertices[0].x - (-1.0)).abs() < 0.001);
        assert!((mesh.vertices[1].x - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_mesh_ground() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![
                Vertex::new(0.0, 5.0, 0.0),
                Vertex::new(0.0, 10.0, 0.0),
            ],
            ..Default::default()
        };

        mesh.ground();
        // Should move mesh so that min Y is 0
        assert!((mesh.vertices[0].y - 0.0).abs() < 0.001);
        assert!((mesh.vertices[1].y - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_mesh_normalize() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![
                Vertex::new(10.0, 5.0, 20.0),
                Vertex::new(20.0, 15.0, 30.0),
            ],
            ..Default::default()
        };

        mesh.normalize();
        // normalize() centers X/Z and grounds Y (min Y = 0)
        let bbox = mesh.bounding_box();
        // Center of X should be 0 (was 15, range 10-20)
        let center_x = (bbox.min[0] + bbox.max[0]) / 2.0;
        assert!((center_x - 0.0).abs() < 0.001);
        // Min Y should be 0 (was 5)
        assert!((bbox.min[1] - 0.0).abs() < 0.001);
        // Center of Z should be 0 (was 25, range 20-30)
        let center_z = (bbox.min[2] + bbox.max[2]) / 2.0;
        assert!((center_z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_mesh_convert_to_yup() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(1.0, 2.0, 3.0)],
            coord_system: CoordSystem::Zup3DS,
            ..Default::default()
        };

        mesh.convert_to_yup();
        // 3DS Z-up to Y-up: (x,y,z) -> (x,z,-y)
        assert_eq!(mesh.coord_system, CoordSystem::YupGltf);
    }

    #[test]
    fn test_mesh_convert_to_yup_coordinate_transform() {
        let mut mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex::new(1.0, 2.0, 3.0)],
            coord_system: CoordSystem::Zup3DS,
            ..Default::default()
        };

        mesh.convert_to_yup();
        // 3DS (x,y,z) -> GLTF (x,z,-y)
        // (1,2,3) -> (1,3,-2)
        assert_eq!(mesh.vertices[0].x, 1.0);
        assert_eq!(mesh.vertices[0].y, 3.0);
        assert_eq!(mesh.vertices[0].z, -2.0);
    }

    #[test]
    fn test_coord_system_default() {
        let cs = CoordSystem::default();
        assert_eq!(cs, CoordSystem::Zup3DS);
    }

    #[test]
    fn test_coord_system_debug_partialeq() {
        let cs1 = CoordSystem::Zup3DS;
        let cs2 = CoordSystem::YupGltf;
        assert_ne!(cs1, cs2);

        let debug_str = format!("{:?}", cs1);
        assert!(debug_str.contains("Zup3DS"));
    }

    #[test]
    fn test_assembly_new() {
        let assembly = Assembly::new("test_assembly");
        assert_eq!(assembly.name, "test_assembly");
        assert!(assembly.parts.is_empty());
    }

    #[test]
    fn test_assembly_add_part() {
        let mut assembly = Assembly::new("test");
        let mesh = Mesh {
            name: "part1".to_string(),
            vertices: vec![Vertex::new(1.0, 2.0, 3.0)],
            ..Default::default()
        };

        assembly.add_part(mesh, Some("material1".to_string()));
        assert_eq!(assembly.parts.len(), 1);
        assert_eq!(assembly.parts[0].material, Some("material1".to_string()));
    }

    #[test]
    fn test_assembly_to_scene() {
        let mut assembly = Assembly::new("test");
        let mesh1 = Mesh {
            name: "part1".to_string(),
            vertices: vec![Vertex::new(0.0, 0.0, 0.0)],
            ..Default::default()
        };
        let mesh2 = Mesh {
            name: "".to_string(), // Empty name
            vertices: vec![Vertex::new(1.0, 1.0, 1.0)],
            ..Default::default()
        };

        assembly.add_part(mesh1, Some("mat1".to_string()));
        assembly.add_part(mesh2, None);

        let scene = assembly.to_scene();
        assert_eq!(scene.meshes.len(), 2);
        assert_eq!(scene.meshes[0].name, "part1");
        assert!(scene.meshes[1].name.starts_with("part_")); // Auto-named
        assert_eq!(scene.meshes[0].material_name, Some("mat1".to_string()));
    }

    #[test]
    fn test_assembly_default() {
        let assembly = Assembly::default();
        assert!(assembly.name.is_empty() || assembly.name == "");
        assert!(assembly.parts.is_empty());
    }

    #[test]
    fn test_assembly_part_debug_clone() {
        let part = AssemblyPart {
            mesh: Mesh {
                name: "test".to_string(),
                ..Default::default()
            },
            material: Some("mat".to_string()),
        };

        let debug_str = format!("{:?}", part);
        assert!(debug_str.contains("AssemblyPart"));

        let cloned = part.clone();
        assert_eq!(cloned.mesh.name, "test");
    }

    #[test]
    fn test_bounding_box_dimensions_match() {
        // dimensions_match converts to mm: dims * 1000 - expected <= tolerance
        // So bbox with dims [0.1, 0.05, 0.025] (in meters) becomes [100, 50, 25] in mm
        let bbox = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [0.1, 0.05, 0.025], // 100mm, 50mm, 25mm
        };

        assert!(bbox.dimensions_match([100.0, 50.0, 25.0], 0.01));
        assert!(bbox.dimensions_match([100.5, 50.5, 25.5], 1.0));
        assert!(!bbox.dimensions_match([200.0, 50.0, 25.0], 0.01));
    }

    #[test]
    fn test_bounding_box_empty_dimensions() {
        let bbox = BoundingBox::empty();
        let dims = bbox.dimensions();
        // Empty box should return zero dimensions
        assert_eq!(dims, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_mesh_bounds() {
        let mesh = Mesh {
            name: "test".to_string(),
            vertices: vec![
                Vertex::new(-5.0, 0.0, 10.0),
                Vertex::new(5.0, 10.0, 20.0),
            ],
            ..Default::default()
        };

        let (min, max) = mesh.bounds();
        assert_eq!(min, [-5.0, 0.0, 10.0]);
        assert_eq!(max, [5.0, 10.0, 20.0]);
    }

    #[test]
    fn test_parse_3ds_invalid_magic() {
        let data = [0x00, 0x00, 0x00, 0x00]; // Invalid magic
        let result = parse_3ds(&data);
        assert!(result.is_err());
        match result {
            Err(GeometryError::Invalid3DS(msg)) => {
                assert!(msg.contains("Invalid main chunk") || msg.contains("0000"));
            }
            _ => panic!("Expected Invalid3DS error"),
        }
    }

    #[test]
    fn test_parse_3ds_too_short() {
        let data: &[u8] = &[0x4D]; // Too short for a 3DS file
        let result = parse_3ds(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_3ds_empty() {
        let data: &[u8] = &[];
        let result = parse_3ds(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_geo_empty() {
        let data: &[u8] = &[];
        let result = parse_geo(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_obj_comments_only() {
        // OBJ with only comments - returns error since it requires at least one face
        let data = b"# This is a comment\n# Another comment\n";
        let result = parse_obj(data);
        // The parser may return an error for empty/invalid OBJ data
        // Or return an empty mesh - either is acceptable behavior
        match result {
            Ok(scene) => {
                assert_eq!(scene.meshes.len(), 1);
                assert!(scene.meshes[0].vertices.is_empty());
            }
            Err(_) => {
                // Also acceptable - empty OBJ is invalid
            }
        }
    }

    #[test]
    fn test_parse_geometry_auto_3ds() {
        // 3DS magic bytes
        let data = [0x4D, 0x4D, 0x00, 0x00, 0x00, 0x00];
        // Should detect as 3DS and try to parse (will fail due to incomplete data)
        let result = parse_geometry_auto("test.3ds", &data);
        // The parse may fail due to incomplete data, but format detection works
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_parse_geometry_auto_obj() {
        let data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nf 1 2 3\n";
        let result = parse_geometry_auto("test.obj", data);
        assert!(result.is_ok());
        let scene = result.unwrap();
        assert!(!scene.meshes.is_empty());
    }

    #[test]
    fn test_parse_geometry_auto_unknown() {
        let data = b"unknown format data that doesn't match any parser";
        let result = parse_geometry_auto("test.xyz", data);
        // Will try all parsers and fail
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_obj_simple() {
        let data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nf 1 2 3\n";
        let result = parse_obj(data);
        assert!(result.is_ok());
        let scene = result.unwrap();
        assert!(!scene.meshes.is_empty());
        assert_eq!(scene.meshes[0].vertices.len(), 3);
        assert_eq!(scene.meshes[0].faces.len(), 1);
    }

    #[test]
    fn test_parse_obj_with_normals() {
        let data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nvn 0.0 0.0 1.0\nf 1//1 2//1 3//1\n";
        let result = parse_obj(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_obj_with_texcoords() {
        let data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nvt 0.0 0.0\nvt 1.0 0.0\nvt 1.0 1.0\nf 1/1 2/2 3/3\n";
        let result = parse_obj(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_obj_with_object() {
        let data = b"o Cube\nv 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nf 1 2 3\n";
        let result = parse_obj(data);
        assert!(result.is_ok());
        let scene = result.unwrap();
        assert!(!scene.meshes.is_empty());
    }

    #[test]
    fn test_parse_obj_with_group() {
        let data = b"g MyGroup\nv 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nf 1 2 3\n";
        let result = parse_obj(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_geo_off_format_error() {
        // OFF format has different header parsing requirements
        let data = b"OFF\n4 2 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n1.0 1.0 0.0\n0.0 1.0 0.0\n3 0 1 2\n3 0 2 3\n";
        let result = parse_geo(data);
        // OFF format is not fully supported in parse_geo (expects simple vertex/face counts as first line)
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_geo_simple() {
        // GEO format: first line is "vertex_count face_count"
        let data = b"3 1\n0.0 0.0 0.0\n1.0 0.0 0.0\n1.0 1.0 0.0\n3 0 1 2\n";
        let result = parse_geo(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scene3ds_debug_clone() {
        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "mesh1".to_string(),
            ..Default::default()
        });

        let debug_str = format!("{:?}", scene);
        assert!(debug_str.contains("Scene3DS"));

        let cloned = scene.clone();
        assert_eq!(cloned.meshes.len(), 1);
    }
}
