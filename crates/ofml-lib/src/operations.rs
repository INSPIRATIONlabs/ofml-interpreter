//! High-level operations for OFML processing.
//!
//! This module provides reusable functions for common OFML operations,
//! designed for use by CLIs, GUIs, and other applications.

use crate::ebase::{EBaseReader, Odb2dRecord, Odb3dRecord};
use crate::geometry::{self, BoundingBox, Scene3DS};
use crate::geometry2d::{process_odb2d_records, G2DCompound};
use crate::ofml::AlbArchive;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Result type for operations
pub type Result<T> = std::result::Result<T, OperationError>;

/// Error type for operations
#[derive(Debug)]
pub enum OperationError {
    Io(std::io::Error),
    Parse(String),
    NotFound(String),
    InvalidFormat(String),
    NoGeometry,
}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Parse(msg) => write!(f, "Parse error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Self::NoGeometry => write!(f, "No geometry found"),
        }
    }
}

impl std::error::Error for OperationError {}

impl From<std::io::Error> for OperationError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Validation result for geometry
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub vertex_count: usize,
    pub face_count: usize,
    pub mesh_count: usize,
    pub material_count: usize,
    pub bounding_box: BoundingBox,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Load geometry from a file, automatically detecting format by extension.
///
/// Supports: .3ds, .geo, .obj
pub fn load_geometry_file(path: &Path) -> Result<Scene3DS> {
    let data = fs::read(path)?;
    load_geometry_data(&data, path)
}

/// Load geometry from raw data, using path for format detection.
pub fn load_geometry_data(data: &[u8], path: &Path) -> Result<Scene3DS> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "3ds" => geometry::parse_3ds(data).map_err(|e| OperationError::Parse(e.to_string())),
        "geo" => geometry::parse_geo(data).map_err(|e| OperationError::Parse(e.to_string())),
        "obj" => geometry::parse_obj(data).map_err(|e| OperationError::Parse(e.to_string())),
        _ => Err(OperationError::InvalidFormat(format!(
            "Unsupported format: .{} (supported: .3ds, .geo, .obj)",
            ext
        ))),
    }
}

/// Validate a geometry scene and return detailed metrics.
pub fn validate_geometry(scene: &Scene3DS) -> ValidationResult {
    let bbox = scene.bounding_box();
    let dims = bbox.dimensions();
    let vertex_count = scene.vertex_count();
    let face_count = scene.face_count();

    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // Check for basic validity
    if vertex_count == 0 {
        errors.push("No vertices found".to_string());
    }
    if face_count == 0 {
        errors.push("No faces found".to_string());
    }
    if !bbox.is_valid() {
        errors.push("Invalid bounding box".to_string());
    }

    // Check for reasonable dimensions (furniture: 1mm to 10m)
    let max_dim = dims.iter().cloned().fold(0.0f32, f32::max);
    let min_dim = dims.iter().cloned().fold(f32::MAX, f32::min);

    if max_dim > 10.0 {
        warnings.push(format!(
            "Very large dimension: {:.2}m (expected < 10m)",
            max_dim
        ));
    }
    if max_dim > 0.0 && min_dim < 0.001 {
        warnings.push(format!(
            "Very small dimension: {:.4}m (expected > 1mm)",
            min_dim
        ));
    }

    // Check for degenerate triangles
    let mut degen_count = 0;
    for mesh in &scene.meshes {
        for face in &mesh.faces {
            if face.a == face.b || face.b == face.c || face.a == face.c {
                degen_count += 1;
            }
        }
    }
    if degen_count > 0 {
        warnings.push(format!("{} degenerate triangles found", degen_count));
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        vertex_count,
        face_count,
        mesh_count: scene.meshes.len(),
        material_count: scene.materials.len(),
        bounding_box: bbox,
        warnings,
        errors,
    }
}

/// Apply scale and offset transforms to a scene.
pub fn apply_transforms(scene: &mut Scene3DS, offset: &[f32; 3], scale: &[f32; 3]) {
    for mesh in &mut scene.meshes {
        for vertex in &mut mesh.vertices {
            vertex.x = vertex.x * scale[0] + offset[0];
            vertex.y = vertex.y * scale[1] + offset[1];
            vertex.z = vertex.z * scale[2] + offset[2];
        }
    }
}

/// Merge multiple scenes into one.
pub fn merge_scenes(scenes: Vec<Scene3DS>) -> Scene3DS {
    let mut combined = Scene3DS::default();
    for scene in scenes {
        combined.meshes.extend(scene.meshes);
        combined.materials.extend(scene.materials);
    }
    combined
}

/// Load multiple geometry files and merge them.
pub fn load_and_merge_geometry(paths: &[&Path]) -> Result<Scene3DS> {
    let mut scenes = Vec::new();

    for path in paths {
        match load_geometry_file(path) {
            Ok(mut scene) => {
                // Rename meshes to include filename for uniqueness
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("mesh");
                for (i, mesh) in scene.meshes.iter_mut().enumerate() {
                    if mesh.name == "geo_mesh" || mesh.name.is_empty() {
                        mesh.name = format!("{}_{}", stem, i);
                    }
                }
                scenes.push(scene);
            }
            Err(e) => {
                // Log warning but continue with other files
                eprintln!("Warning: failed to load {}: {}", path.display(), e);
            }
        }
    }

    if scenes.is_empty() {
        return Err(OperationError::NoGeometry);
    }

    Ok(merge_scenes(scenes))
}

/// Convert a scene to GLB format.
pub fn export_to_glb(scene: &Scene3DS) -> Result<Vec<u8>> {
    geometry::scene_to_glb(scene).map_err(|e| OperationError::Parse(e.to_string()))
}

/// Product assembly configuration
#[derive(Debug, Clone, Default)]
pub struct ProductConfig {
    /// Specific article to assemble (None = all)
    pub article: Option<String>,
    /// Custom properties to override
    pub properties: HashMap<String, f64>,
}

/// Product assembly result
#[derive(Debug)]
pub struct ProductResult {
    pub scene: Scene3DS,
    pub articles_found: Vec<String>,
    pub geometry_loaded: usize,
    pub geometry_missing: Vec<String>,
}

/// Assemble a product from OFML data directory.
///
/// The product_path should contain an odb.ebase file and optionally ALB archives.
pub fn assemble_product(product_path: &Path, config: &ProductConfig) -> Result<ProductResult> {
    // Find odb.ebase
    let (odb_path, actual_dir) = find_odb_ebase(product_path)?;

    // Open EBASE
    let mut reader = EBaseReader::open(&odb_path)
        .map_err(|e| OperationError::Parse(format!("Failed to open ODB: {}", e)))?;

    // Read odb3d records
    let records = reader
        .read_records("odb3d", None)
        .map_err(|e| OperationError::Parse(format!("Failed to read odb3d: {}", e)))?;

    // Find ALB archive
    let alb_path = find_alb_file(&actual_dir);
    let mut alb_archive = alb_path.as_ref().and_then(|p| AlbArchive::open(p).ok());

    // Collect geometry references
    let mut geo_refs: Vec<(String, [f32; 3], [f32; 3])> = Vec::new();
    let mut articles_found: Vec<String> = Vec::new();

    for record in &records {
        if let Some(odb_rec) = Odb3dRecord::from_record(record) {
            // Filter by article if specified
            if let Some(ref art) = config.article {
                if !odb_rec.odb_name.eq_ignore_ascii_case(art) {
                    continue;
                }
            }

            if !articles_found.contains(&odb_rec.odb_name) {
                articles_found.push(odb_rec.odb_name.clone());
            }

            if let Some((geo_name, scale)) = odb_rec.parse_ctor() {
                let offset = odb_rec.parse_offset();
                geo_refs.push((geo_name, offset, scale));
            }
        }
    }

    // Load geometry
    let mut combined_scene = Scene3DS::default();
    let mut loaded_count = 0;
    let mut missing = Vec::new();

    for (geo_name, offset, scale) in &geo_refs {
        let loaded = load_geometry_for_product(
            geo_name,
            &actual_dir,
            alb_archive.as_mut(),
            offset,
            scale,
            &mut combined_scene,
        );

        if loaded {
            loaded_count += 1;
        } else {
            missing.push(geo_name.clone());
        }
    }

    // Fallback: scan for geometry files if nothing loaded
    if combined_scene.meshes.is_empty() {
        scan_directory_for_geometry(&actual_dir, &mut combined_scene)?;
        loaded_count = combined_scene.meshes.len();
    }

    if combined_scene.meshes.is_empty() {
        return Err(OperationError::NoGeometry);
    }

    Ok(ProductResult {
        scene: combined_scene,
        articles_found,
        geometry_loaded: loaded_count,
        geometry_missing: missing,
    })
}

/// Find odb.ebase file in product directory or version subdirectories.
fn find_odb_ebase(product_path: &Path) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    // Check direct path
    let direct = product_path.join("odb.ebase");
    if direct.exists() {
        return Ok((direct, product_path.to_path_buf()));
    }

    // Check version subdirectories
    for version in &["1", "2", "3", "current"] {
        let version_dir = product_path.join(version);
        let odb = version_dir.join("odb.ebase");
        if odb.exists() {
            return Ok((odb, version_dir));
        }
    }

    Err(OperationError::NotFound(format!(
        "odb.ebase not found in {}",
        product_path.display()
    )))
}

/// Find ALB file in directory.
fn find_alb_file(dir: &Path) -> Option<std::path::PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map(|s| s == "alb").unwrap_or(false) {
                return Some(path);
            }
        }
    }
    None
}

/// Load geometry for a product, trying disk then ALB.
fn load_geometry_for_product(
    geo_name: &str,
    product_dir: &Path,
    alb_archive: Option<&mut AlbArchive>,
    offset: &[f32; 3],
    scale: &[f32; 3],
    combined_scene: &mut Scene3DS,
) -> bool {
    // Try disk first
    let patterns = [
        format!("{}.geo", geo_name),
        format!("{}.3ds", geo_name),
        format!("{}.obj", geo_name),
    ];

    for pattern in &patterns {
        let geo_path = product_dir.join(pattern);
        if geo_path.exists() {
            if let Ok(mut scene) = load_geometry_file(&geo_path) {
                apply_transforms(&mut scene, offset, scale);
                combined_scene.meshes.extend(scene.meshes);
                combined_scene.materials.extend(scene.materials);
                return true;
            }
        }
    }

    // Try ALB archive
    if let Some(archive) = alb_archive {
        let all_files = archive.list_files();
        for pattern in &patterns {
            let pattern_lower = pattern.to_lowercase();
            for file in &all_files {
                let file_lower = file.to_lowercase();
                if file_lower.ends_with(&pattern_lower)
                    || file_lower.contains(&format!("/{}", pattern_lower))
                {
                    if let Ok(data) = archive.extract(file) {
                        let ext = Path::new(file)
                            .extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or("");
                        let scene_result = match ext.to_lowercase().as_str() {
                            "obj" => geometry::parse_obj(&data),
                            "3ds" => geometry::parse_3ds(&data),
                            "geo" => geometry::parse_geo(&data),
                            _ => continue,
                        };
                        if let Ok(mut scene) = scene_result {
                            apply_transforms(&mut scene, offset, scale);
                            combined_scene.meshes.extend(scene.meshes);
                            combined_scene.materials.extend(scene.materials);
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

/// Scan directory for geometry files.
fn scan_directory_for_geometry(dir: &Path, scene: &mut Scene3DS) -> Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            if ext == "geo" || ext == "3ds" || ext == "obj" {
                if let Ok(s) = load_geometry_file(&path) {
                    scene.meshes.extend(s.meshes);
                    scene.materials.extend(s.materials);
                }
            }
        }
    }
    Ok(())
}

/// Export 2D floor plan from EBASE odb2d table.
pub fn export_2d_floorplan(ebase_path: &Path) -> Result<G2DCompound> {
    let mut reader = EBaseReader::open(ebase_path)
        .map_err(|e| OperationError::Parse(format!("Failed to open EBASE: {}", e)))?;

    if !reader.tables.contains_key("odb2d") {
        return Err(OperationError::NotFound(
            "odb2d table not found".to_string(),
        ));
    }

    let records = reader
        .read_records("odb2d", None)
        .map_err(|e| OperationError::Parse(format!("Failed to read odb2d: {}", e)))?;

    let odb2d_records: Vec<Odb2dRecord> = records
        .iter()
        .filter_map(Odb2dRecord::from_record)
        .filter(|r| !r.prim_type.is_empty())
        .collect();

    if odb2d_records.is_empty() {
        return Err(OperationError::NoGeometry);
    }

    Ok(process_odb2d_records(&odb2d_records))
}

/// Evaluate an EBASE expression.
pub fn evaluate_expression(
    expr: &str,
    props: &HashMap<String, f64>,
) -> std::result::Result<crate::ebase_expr::EbaseResult, String> {
    let mut evaluator = crate::ebase_expr::EbaseEvaluator::new();
    evaluator.evaluate(expr, props).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_scene() {
        let scene = Scene3DS::default();
        let result = validate_geometry(&scene);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("No vertices")));
    }

    #[test]
    fn test_validate_valid_scene() {
        use crate::geometry::{Face, Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "cube".to_string(),
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
            ],
            faces: vec![Face {
                a: 0,
                b: 1,
                c: 2,
                flags: 0,
            }],
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(result.is_valid);
        assert_eq!(result.vertex_count, 3);
        assert_eq!(result.face_count, 1);
    }

    #[test]
    fn test_validate_degenerate_triangle() {
        use crate::geometry::{Face, Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "bad".to_string(),
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
            ],
            faces: vec![Face {
                a: 0,
                b: 0,
                c: 1,
                flags: 0,
            }], // degenerate
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(result.warnings.iter().any(|w| w.contains("degenerate")));
    }

    #[test]
    fn test_apply_transforms() {
        use crate::geometry::{Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }],
            ..Default::default()
        });

        apply_transforms(&mut scene, &[10.0, 20.0, 30.0], &[2.0, 2.0, 2.0]);

        let v = &scene.meshes[0].vertices[0];
        assert_eq!(v.x, 12.0); // 1*2 + 10
        assert_eq!(v.y, 24.0); // 2*2 + 20
        assert_eq!(v.z, 36.0); // 3*2 + 30
    }

    #[test]
    fn test_merge_scenes() {
        use crate::geometry::{Mesh, Vertex};

        let scene1 = Scene3DS {
            meshes: vec![Mesh {
                name: "mesh1".to_string(),
                vertices: vec![Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }],
                ..Default::default()
            }],
            ..Default::default()
        };

        let scene2 = Scene3DS {
            meshes: vec![Mesh {
                name: "mesh2".to_string(),
                vertices: vec![Vertex {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }],
                ..Default::default()
            }],
            ..Default::default()
        };

        let merged = merge_scenes(vec![scene1, scene2]);
        assert_eq!(merged.meshes.len(), 2);
        assert_eq!(merged.meshes[0].name, "mesh1");
        assert_eq!(merged.meshes[1].name, "mesh2");
    }

    #[test]
    fn test_export_to_glb() {
        use crate::geometry::{Face, Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "cube".to_string(),
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
            ],
            faces: vec![Face {
                a: 0,
                b: 1,
                c: 2,
                flags: 0,
            }],
            ..Default::default()
        });

        let glb = export_to_glb(&scene).expect("GLB export failed");
        assert!(!glb.is_empty());
        // GLB magic number
        assert_eq!(&glb[0..4], b"glTF");
    }

    #[test]
    fn test_evaluate_expression() {
        let props = HashMap::new();
        let result = evaluate_expression(r#""test.geo" 1 1 1 imp"#, &props);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_geometry_data_invalid_format() {
        let path = Path::new("test.xyz");
        let result = load_geometry_data(b"data", path);
        assert!(matches!(result, Err(OperationError::InvalidFormat(_))));
    }

    #[test]
    fn test_validation_large_dimension_warning() {
        use crate::geometry::{Face, Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "large".to_string(),
            vertices: vec![
                Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vertex {
                    x: 15.0,
                    y: 0.0,
                    z: 0.0,
                }, // 15m > 10m
                Vertex {
                    x: 15.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            faces: vec![Face {
                a: 0,
                b: 1,
                c: 2,
                flags: 0,
            }],
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(result.warnings.iter().any(|w| w.contains("Very large")));
    }

    #[test]
    fn test_operation_error_display() {
        let io_err = OperationError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        assert!(format!("{}", io_err).contains("I/O error"));

        let parse_err = OperationError::Parse("invalid syntax".to_string());
        assert!(format!("{}", parse_err).contains("Parse error"));
        assert!(format!("{}", parse_err).contains("invalid syntax"));

        let not_found = OperationError::NotFound("missing file".to_string());
        assert!(format!("{}", not_found).contains("Not found"));

        let invalid = OperationError::InvalidFormat("unsupported".to_string());
        assert!(format!("{}", invalid).contains("Invalid format"));

        let no_geo = OperationError::NoGeometry;
        assert!(format!("{}", no_geo).contains("No geometry"));
    }

    #[test]
    fn test_operation_error_debug() {
        let err = OperationError::NoGeometry;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("NoGeometry"));
    }

    #[test]
    fn test_operation_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let op_err: OperationError = io_err.into();
        assert!(matches!(op_err, OperationError::Io(_)));
    }

    #[test]
    fn test_validation_result_clone() {
        let result = ValidationResult {
            is_valid: true,
            vertex_count: 100,
            face_count: 50,
            mesh_count: 2,
            material_count: 3,
            bounding_box: BoundingBox::empty(),
            warnings: vec!["warning1".to_string()],
            errors: vec![],
        };

        let cloned = result.clone();
        assert_eq!(cloned.is_valid, true);
        assert_eq!(cloned.vertex_count, 100);
        assert_eq!(cloned.face_count, 50);
        assert_eq!(cloned.warnings.len(), 1);
    }

    #[test]
    fn test_validation_result_debug() {
        let result = ValidationResult {
            is_valid: true,
            vertex_count: 10,
            face_count: 5,
            mesh_count: 1,
            material_count: 1,
            bounding_box: BoundingBox::empty(),
            warnings: vec![],
            errors: vec![],
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("ValidationResult"));
    }

    #[test]
    fn test_validation_small_dimension_warning() {
        use crate::geometry::{Face, Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "tiny".to_string(),
            vertices: vec![
                Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vertex {
                    x: 0.0005, // 0.5mm < 1mm threshold
                    y: 1.0,
                    z: 0.0,
                },
                Vertex {
                    x: 0.0005,
                    y: 1.0,
                    z: 1.0,
                },
            ],
            faces: vec![Face {
                a: 0,
                b: 1,
                c: 2,
                flags: 0,
            }],
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(result.warnings.iter().any(|w| w.contains("Very small")));
    }

    #[test]
    fn test_validation_no_faces() {
        use crate::geometry::{Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "no_faces".to_string(),
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
            ],
            faces: vec![], // No faces
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(result.errors.iter().any(|e| e.contains("No faces")));
    }

    #[test]
    fn test_product_config_default() {
        let config = ProductConfig::default();
        assert!(config.article.is_none());
        assert!(config.properties.is_empty());
    }

    #[test]
    fn test_product_config_clone() {
        let mut config = ProductConfig::default();
        config.article = Some("test_article".to_string());
        config.properties.insert("width".to_string(), 100.0);

        let cloned = config.clone();
        assert_eq!(cloned.article, Some("test_article".to_string()));
        assert_eq!(cloned.properties.get("width"), Some(&100.0));
    }

    #[test]
    fn test_product_config_debug() {
        let config = ProductConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ProductConfig"));
    }

    #[test]
    fn test_product_result_debug() {
        let result = ProductResult {
            scene: Scene3DS::default(),
            articles_found: vec!["article1".to_string()],
            geometry_loaded: 1,
            geometry_missing: vec![],
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("ProductResult"));
    }

    #[test]
    fn test_load_geometry_file_not_found() {
        let result = load_geometry_file(Path::new("/nonexistent/path/file.3ds"));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OperationError::Io(_)));
    }

    #[test]
    fn test_load_geometry_data_no_extension() {
        let path = Path::new("noextension");
        let result = load_geometry_data(b"data", path);
        assert!(matches!(result, Err(OperationError::InvalidFormat(_))));
    }

    #[test]
    fn test_load_and_merge_empty_paths() {
        let result = load_and_merge_geometry(&[]);
        assert!(matches!(result, Err(OperationError::NoGeometry)));
    }

    #[test]
    fn test_load_and_merge_nonexistent_files() {
        let paths: Vec<&Path> = vec![
            Path::new("/nonexistent1.geo"),
            Path::new("/nonexistent2.3ds"),
        ];
        // Should fail because all files fail to load
        let result = load_and_merge_geometry(&paths);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_expression_simple() {
        let props = HashMap::new();
        let result = evaluate_expression("1 2 +", &props);
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_expression_with_props() {
        let mut props = HashMap::new();
        props.insert("width".to_string(), 100.0);
        // Just test that the function handles properties without crashing
        let _result = evaluate_expression("100 10 *", &props);
    }

    #[test]
    fn test_evaluate_expression_invalid() {
        let props = HashMap::new();
        // Just check it returns something (may be error or success depending on expression)
        let _result = evaluate_expression("", &props);
    }

    #[test]
    fn test_merge_scenes_empty() {
        let merged = merge_scenes(vec![]);
        assert!(merged.meshes.is_empty());
        assert!(merged.materials.is_empty());
    }

    #[test]
    fn test_merge_scenes_single() {
        use crate::geometry::{Mesh, Vertex};

        let scene = Scene3DS {
            meshes: vec![Mesh {
                name: "single".to_string(),
                vertices: vec![Vertex {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                }],
                ..Default::default()
            }],
            ..Default::default()
        };

        let merged = merge_scenes(vec![scene]);
        assert_eq!(merged.meshes.len(), 1);
        assert_eq!(merged.meshes[0].name, "single");
    }

    #[test]
    fn test_merge_scenes_materials() {
        use crate::geometry::Material3DS;

        let mut scene1 = Scene3DS::default();
        scene1.materials.insert("mat1".to_string(), Material3DS {
            name: "mat1".to_string(),
            ..Default::default()
        });

        let mut scene2 = Scene3DS::default();
        scene2.materials.insert("mat2".to_string(), Material3DS {
            name: "mat2".to_string(),
            ..Default::default()
        });

        let merged = merge_scenes(vec![scene1, scene2]);
        assert_eq!(merged.materials.len(), 2);
    }

    #[test]
    fn test_apply_transforms_identity() {
        use crate::geometry::{Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex {
                x: 5.0,
                y: 10.0,
                z: 15.0,
            }],
            ..Default::default()
        });

        // Identity transform: scale 1, offset 0
        apply_transforms(&mut scene, &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);

        let v = &scene.meshes[0].vertices[0];
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 10.0);
        assert_eq!(v.z, 15.0);
    }

    #[test]
    fn test_apply_transforms_scale_only() {
        use crate::geometry::{Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }],
            ..Default::default()
        });

        apply_transforms(&mut scene, &[0.0, 0.0, 0.0], &[3.0, 3.0, 3.0]);

        let v = &scene.meshes[0].vertices[0];
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 6.0);
        assert_eq!(v.z, 9.0);
    }

    #[test]
    fn test_apply_transforms_offset_only() {
        use crate::geometry::{Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "test".to_string(),
            vertices: vec![Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }],
            ..Default::default()
        });

        apply_transforms(&mut scene, &[100.0, 200.0, 300.0], &[1.0, 1.0, 1.0]);

        let v = &scene.meshes[0].vertices[0];
        assert_eq!(v.x, 100.0);
        assert_eq!(v.y, 200.0);
        assert_eq!(v.z, 300.0);
    }

    #[test]
    fn test_assemble_product_not_found() {
        let result = assemble_product(Path::new("/nonexistent/path"), &ProductConfig::default());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OperationError::NotFound(_)));
    }

    #[test]
    fn test_export_2d_floorplan_not_found() {
        let result = export_2d_floorplan(Path::new("/nonexistent/path/odb.ebase"));
        assert!(result.is_err());
    }

    // ========== Additional Coverage Tests ==========

    #[test]
    fn test_operation_error_display_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = OperationError::Io(io_err);
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_operation_error_display_parse() {
        let err = OperationError::Parse("syntax error".to_string());
        assert!(err.to_string().contains("Parse error"));
        assert!(err.to_string().contains("syntax error"));
    }

    #[test]
    fn test_operation_error_display_not_found() {
        let err = OperationError::NotFound("item.3ds".to_string());
        assert!(err.to_string().contains("Not found"));
        assert!(err.to_string().contains("item.3ds"));
    }

    #[test]
    fn test_operation_error_display_invalid_format() {
        let err = OperationError::InvalidFormat("unsupported".to_string());
        assert!(err.to_string().contains("Invalid format"));
    }

    #[test]
    fn test_operation_error_display_no_geometry() {
        let err = OperationError::NoGeometry;
        assert!(err.to_string().contains("No geometry"));
    }

    #[test]
    fn test_operation_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let op_err: OperationError = io_err.into();
        assert!(matches!(op_err, OperationError::Io(_)));
    }

    #[test]
    fn test_validation_result_fields() {
        let result = ValidationResult {
            is_valid: true,
            vertex_count: 100,
            face_count: 50,
            mesh_count: 3,
            material_count: 2,
            bounding_box: BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [1.0, 1.0, 1.0],
            },
            warnings: vec!["small dimension".to_string()],
            errors: vec![],
        };
        assert!(result.is_valid);
        assert_eq!(result.vertex_count, 100);
        assert_eq!(result.face_count, 50);
        assert_eq!(result.mesh_count, 3);
        assert_eq!(result.material_count, 2);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_no_faces() {
        use crate::geometry::{Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "test".to_string(),
            vertices: vec![
                Vertex { x: 0.0, y: 0.0, z: 0.0 },
                Vertex { x: 1.0, y: 0.0, z: 0.0 },
                Vertex { x: 1.0, y: 1.0, z: 0.0 },
            ],
            faces: vec![],
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("No faces")));
    }

    #[test]
    fn test_validate_large_dimension_warning() {
        use crate::geometry::{Face, Mesh, Vertex};

        let mut scene = Scene3DS::default();
        scene.meshes.push(Mesh {
            name: "big".to_string(),
            vertices: vec![
                Vertex { x: 0.0, y: 0.0, z: 0.0 },
                Vertex { x: 20.0, y: 0.0, z: 0.0 }, // > 10m
                Vertex { x: 10.0, y: 10.0, z: 0.0 },
            ],
            faces: vec![Face { a: 0, b: 1, c: 2, flags: 0 }],
            ..Default::default()
        });

        let result = validate_geometry(&scene);
        assert!(result.warnings.iter().any(|w| w.contains("Very large dimension")));
    }

    #[test]
    fn test_load_and_merge_geometry_no_valid_files() {
        let paths: Vec<&Path> = vec![
            Path::new("/nonexistent1.3ds"),
            Path::new("/nonexistent2.3ds"),
        ];
        let result = load_and_merge_geometry(&paths);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OperationError::NoGeometry));
    }

    #[test]
    fn test_load_geometry_data_unsupported_format() {
        let data = b"fake data";
        let result = load_geometry_data(data, Path::new("file.xyz"));
        assert!(matches!(result, Err(OperationError::InvalidFormat(_))));
    }

    #[test]
    fn test_product_config_with_properties() {
        let mut props = HashMap::new();
        props.insert("WIDTH".to_string(), 1200.0);

        let config = ProductConfig {
            article: Some("ART-001".to_string()),
            properties: props,
        };
        assert_eq!(config.article, Some("ART-001".to_string()));
        assert_eq!(config.properties.get("WIDTH"), Some(&1200.0));
    }

    #[test]
    fn test_product_result_struct() {
        let result = ProductResult {
            scene: Scene3DS::default(),
            articles_found: vec!["ART-001".to_string()],
            geometry_loaded: 3,
            geometry_missing: vec!["missing.3ds".to_string()],
        };
        assert_eq!(result.articles_found.len(), 1);
        assert_eq!(result.geometry_loaded, 3);
        assert_eq!(result.geometry_missing.len(), 1);
    }
}
