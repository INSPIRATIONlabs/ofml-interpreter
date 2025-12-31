//! OFML (Open Furniture Modeling Language) data reader.
//!
//! This module provides a high-level interface for reading OFML data directories
//! used by EasternGraphics furniture configuration software.
//!
//! OFML Directory Structure:
//! ```text
//! {manufacturer}/
//!     global/
//!         1/
//!             global.ebase    - Global type definitions, articles, properties
//!             ofml.ebase      - OFML metadata
//!             *.alb           - Album files
//!     {product}/
//!         1/
//!             odb.ebase       - Object database (3D/2D definitions)
//!             ofml.ebase      - OFML metadata
//!             *.alb           - Album files (3D geometry)
//!             *.geo           - Geometry files (SEDUS)
//!             *.mat           - Material files (SEDUS)
//!         {country}/
//!             1/
//!                 db/
//!                     pdata.ebase - Pricing data
//!                 oam/
//!                     oam.ebase   - Article master data
//!                 oap/
//!                     oap.ebase   - Article pictures metadata
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use crate::ebase::{EBaseError, EBaseReader};
use crate::geometry::{parse_3ds, parse_obj, GeometryError, Scene3DS};

/// ALB archive password (ROT13 encoded original)
pub const ALB_PASSWORD: &[u8] = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";

/// Error types for OFML operations
#[derive(Debug)]
pub enum OFMLError {
    Io(io::Error),
    EBase(EBaseError),
    Geometry(GeometryError),
    Zip(zip::result::ZipError),
    NotFound(String),
    InvalidStructure(String),
}

impl From<io::Error> for OFMLError {
    fn from(err: io::Error) -> Self {
        OFMLError::Io(err)
    }
}

impl From<EBaseError> for OFMLError {
    fn from(err: EBaseError) -> Self {
        OFMLError::EBase(err)
    }
}

impl From<GeometryError> for OFMLError {
    fn from(err: GeometryError) -> Self {
        OFMLError::Geometry(err)
    }
}

impl From<zip::result::ZipError> for OFMLError {
    fn from(err: zip::result::ZipError) -> Self {
        OFMLError::Zip(err)
    }
}

impl std::fmt::Display for OFMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OFMLError::Io(e) => write!(f, "IO error: {}", e),
            OFMLError::EBase(e) => write!(f, "EBase error: {}", e),
            OFMLError::Geometry(e) => write!(f, "Geometry error: {}", e),
            OFMLError::Zip(e) => write!(f, "ZIP error: {}", e),
            OFMLError::NotFound(s) => write!(f, "Not found: {}", s),
            OFMLError::InvalidStructure(s) => write!(f, "Invalid structure: {}", s),
        }
    }
}

impl std::error::Error for OFMLError {}

/// An ALB archive (encrypted ZIP with 3D assets and CLS files)
pub struct AlbArchive {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    path: PathBuf,
}

impl AlbArchive {
    /// Open an ALB archive
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, OFMLError> {
        let path = path.as_ref().to_path_buf();
        let data = fs::read(&path)?;
        let cursor = Cursor::new(data);
        let archive = ZipArchive::new(cursor)?;
        Ok(AlbArchive { archive, path })
    }

    /// List all files in the archive
    pub fn list_files(&self) -> Vec<String> {
        (0..self.archive.len())
            .filter_map(|i| self.archive.name_for_index(i).map(|n| n.to_string()))
            .collect()
    }

    /// List files matching a pattern (glob-like)
    pub fn list_files_matching(&self, extension: &str) -> Vec<String> {
        self.list_files()
            .into_iter()
            .filter(|name| name.to_lowercase().ends_with(extension))
            .collect()
    }

    /// Extract a file by name
    pub fn extract(&mut self, name: &str) -> Result<Vec<u8>, OFMLError> {
        // Find the file index by name
        let index = (0..self.archive.len())
            .find(|&i| {
                self.archive
                    .name_for_index(i)
                    .map(|n| n == name)
                    .unwrap_or(false)
            })
            .ok_or_else(|| OFMLError::NotFound(name.to_string()))?;

        // Extract by index with decryption
        let mut file = match self.archive.by_index_decrypt(index, ALB_PASSWORD) {
            Ok(f) => f,
            Err(e) => return Err(OFMLError::Zip(e)),
        };

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    /// Extract and parse a CLS file
    pub fn extract_cls(&mut self, name: &str) -> Result<String, OFMLError> {
        let data = self.extract(name)?;
        String::from_utf8(data)
            .map_err(|_| OFMLError::InvalidStructure("Invalid UTF-8 in CLS file".to_string()))
    }

    /// Extract and parse a 3DS file
    pub fn extract_3ds(&mut self, name: &str) -> Result<Scene3DS, OFMLError> {
        let data = self.extract(name)?;
        parse_3ds(&data).map_err(OFMLError::from)
    }

    /// Extract and parse an OBJ file
    pub fn extract_obj(&mut self, name: &str) -> Result<Scene3DS, OFMLError> {
        let data = self.extract(name)?;
        parse_obj(&data).map_err(OFMLError::from)
    }

    /// Extract and parse any geometry file (3DS or OBJ)
    pub fn extract_geometry(&mut self, name: &str) -> Result<Scene3DS, OFMLError> {
        let lower = name.to_lowercase();
        if lower.ends_with(".obj") {
            self.extract_obj(name)
        } else if lower.ends_with(".3ds") {
            self.extract_3ds(name)
        } else {
            Err(OFMLError::InvalidStructure(format!(
                "Unknown geometry format: {}",
                name
            )))
        }
    }

    /// Get all CLS files
    pub fn get_cls_files(&self) -> Vec<String> {
        self.list_files_matching(".cls")
    }

    /// Get all 3DS files
    pub fn get_3ds_files(&self) -> Vec<String> {
        self.list_files_matching(".3ds")
    }

    /// Get all OBJ files (Wavefront)
    pub fn get_obj_files(&self) -> Vec<String> {
        self.list_files_matching(".obj")
    }

    /// Get all geometry files (3DS + OBJ)
    pub fn get_geometry_files(&self) -> Vec<String> {
        let mut files = self.get_3ds_files();
        files.extend(self.get_obj_files());
        files
    }

    /// Get all string resource files
    pub fn get_sr_files(&self) -> Vec<String> {
        self.list_files_matching(".sr")
    }

    /// Get the archive path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// OFML product data
pub struct OFMLProduct {
    pub manufacturer: String,
    pub product_name: String,
    pub base_path: PathBuf,
    pub album_files: Vec<PathBuf>,
    pub geo_files: Vec<PathBuf>,
    pub mat_files: Vec<PathBuf>,
}

impl OFMLProduct {
    /// Check if product has EBASE databases
    pub fn has_odb(&self) -> bool {
        self.base_path.join("odb.ebase").exists()
    }

    /// Check if product has ALB files (Vitra-style)
    pub fn has_alb(&self) -> bool {
        !self.album_files.is_empty()
    }

    /// Check if product has GEO files (SEDUS-style)
    pub fn has_geo(&self) -> bool {
        !self.geo_files.is_empty()
    }

    /// Open the ODB database
    pub fn open_odb(&self) -> Result<EBaseReader, OFMLError> {
        let path = self.base_path.join("odb.ebase");
        EBaseReader::open(&path).map_err(OFMLError::from)
    }

    /// Open the OFML metadata database
    pub fn open_ofml(&self) -> Result<EBaseReader, OFMLError> {
        let path = self.base_path.join("ofml.ebase");
        EBaseReader::open(&path).map_err(OFMLError::from)
    }

    /// Open an ALB archive
    pub fn open_alb(&self, index: usize) -> Result<AlbArchive, OFMLError> {
        if index >= self.album_files.len() {
            return Err(OFMLError::NotFound(format!("ALB file index {}", index)));
        }
        AlbArchive::open(&self.album_files[index])
    }

    /// Get available countries
    pub fn get_countries(&self) -> Vec<String> {
        let mut countries = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.base_path.parent().unwrap_or(&self.base_path)) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.len() == 2 && name.chars().all(|c| c.is_ascii_uppercase()) {
                    countries.push(name);
                }
            }
        }
        countries.sort();
        countries
    }
}

/// OFML data reader for exploring manufacturer data
pub struct OFMLDataReader {
    pub data_path: PathBuf,
}

impl OFMLDataReader {
    /// Create a new OFML data reader
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        OFMLDataReader {
            data_path: path.as_ref().to_path_buf(),
        }
    }

    /// Discover all manufacturers in the data directory
    pub fn discover_manufacturers(&self) -> Vec<String> {
        let mut manufacturers = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.data_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Skip special files
                    if !name.starts_with('.')
                        && name != "install.db"
                        && name != "profiles"
                        && name != "registry"
                    {
                        manufacturers.push(name);
                    }
                }
            }
        }
        manufacturers.sort();
        manufacturers
    }

    /// Discover all products for a manufacturer
    pub fn discover_products(&self, manufacturer: &str) -> Vec<String> {
        let mfr_path = self.data_path.join(manufacturer);
        let mut products = Vec::new();

        if let Ok(entries) = fs::read_dir(&mfr_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Skip global and country code directories
                    if name != "global" && name != "global2" && !name.starts_with('.') {
                        // Check if it has a version directory (1, 2, etc.) with data
                        let product_dir = entry.path();
                        if let Ok(version_entries) = fs::read_dir(&product_dir) {
                            for ver_entry in version_entries.filter_map(|e| e.ok()) {
                                let ver_name = ver_entry.file_name().to_string_lossy().to_string();
                                // Check for numeric version directories
                                if ver_name.chars().all(|c| c.is_ascii_digit()) {
                                    let version_dir = ver_entry.path();
                                    let has_data = version_dir.join("odb.ebase").exists()
                                        || version_dir.join("ofml.ebase").exists()
                                        || version_dir
                                            .read_dir()
                                            .map(|mut d| {
                                                d.any(|e| {
                                                    e.ok()
                                                        .map(|e| {
                                                            e.path()
                                                                .extension()
                                                                .map(|ext| {
                                                                    ext == "alb" || ext == "geo"
                                                                })
                                                                .unwrap_or(false)
                                                        })
                                                        .unwrap_or(false)
                                                })
                                            })
                                            .unwrap_or(false);
                                    if has_data {
                                        products.push(name.clone());
                                        break; // Found at least one version with data
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        products.sort();
        products
    }

    /// Load a product's data
    pub fn load_product(
        &self,
        manufacturer: &str,
        product: &str,
    ) -> Result<OFMLProduct, OFMLError> {
        let product_base = self.data_path.join(manufacturer).join(product);
        if !product_base.exists() {
            return Err(OFMLError::NotFound(format!("{}/{}", manufacturer, product)));
        }

        // Find version directory (usually "1")
        let mut version_dir = None;
        if let Ok(entries) = fs::read_dir(&product_base) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.chars().all(|c| c.is_ascii_digit()) && entry.path().is_dir() {
                    version_dir = Some(entry.path());
                    break;
                }
            }
        }

        let base_path = version_dir.ok_or_else(|| {
            OFMLError::InvalidStructure(format!(
                "No version directory in {}/{}",
                manufacturer, product
            ))
        })?;

        // Find files
        let mut album_files = Vec::new();
        let mut geo_files = Vec::new();
        let mut mat_files = Vec::new();

        if let Ok(entries) = fs::read_dir(&base_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                        "alb" => album_files.push(path),
                        "geo" => geo_files.push(path),
                        "mat" => mat_files.push(path),
                        _ => {}
                    }
                }
            }
        }

        album_files.sort();
        geo_files.sort();
        mat_files.sort();

        Ok(OFMLProduct {
            manufacturer: manufacturer.to_string(),
            product_name: product.to_string(),
            base_path,
            album_files,
            geo_files,
            mat_files,
        })
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> OFMLSummary {
        let manufacturers = self.discover_manufacturers();
        let mut total_products = 0;
        let mut total_alb_files = 0;
        let mut total_geo_files = 0;
        let mut product_counts = HashMap::new();

        for mfr in &manufacturers {
            let products = self.discover_products(mfr);
            product_counts.insert(mfr.clone(), products.len());
            total_products += products.len();

            for prod in &products {
                if let Ok(p) = self.load_product(mfr, prod) {
                    total_alb_files += p.album_files.len();
                    total_geo_files += p.geo_files.len();
                }
            }
        }

        OFMLSummary {
            manufacturers,
            product_counts,
            total_products,
            total_alb_files,
            total_geo_files,
        }
    }
}

/// Summary of OFML data
#[derive(Debug)]
pub struct OFMLSummary {
    pub manufacturers: Vec<String>,
    pub product_counts: HashMap<String, usize>,
    pub total_products: usize,
    pub total_alb_files: usize,
    pub total_geo_files: usize,
}

/// Extract all CLS files from an ALB archive
pub fn extract_cls_from_alb<P: AsRef<Path>>(
    alb_path: P,
) -> Result<HashMap<String, String>, OFMLError> {
    let mut archive = AlbArchive::open(alb_path)?;
    let cls_files = archive.get_cls_files();
    let mut result = HashMap::new();

    for name in cls_files {
        if let Ok(content) = archive.extract_cls(&name) {
            // Use just the filename without path
            let key = Path::new(&name)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(name.clone());
            result.insert(key, content);
        }
    }

    Ok(result)
}

/// Extract all 3DS geometries from an ALB archive
pub fn extract_3ds_from_alb<P: AsRef<Path>>(
    alb_path: P,
) -> Result<HashMap<String, Scene3DS>, OFMLError> {
    let mut archive = AlbArchive::open(alb_path)?;
    let files = archive.get_3ds_files();
    let mut result = HashMap::new();

    for name in files {
        if let Ok(scene) = archive.extract_3ds(&name) {
            let key = Path::new(&name)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(name.clone());
            result.insert(key, scene);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Get a test ALB path, preferring fixtures directory
    fn get_test_alb_path() -> Option<std::path::PathBuf> {
        // Try fixtures first (for CI/clean checkout)
        let fixture_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/vitra/workit/vitra_workit_1.alb");
        if fixture_path.exists() {
            return Some(fixture_path);
        }

        // Fall back to ofmldata (local development)
        let ofmldata_path =
            std::path::PathBuf::from("/workspace/ofmldata/vitra/workit/1/vitra_workit_1.alb");
        if ofmldata_path.exists() {
            return Some(ofmldata_path);
        }

        None
    }

    #[test]
    fn test_ofml_data_reader_discover() {
        // Test with ofmldata directory if available
        let ofmldata_path = "/workspace/ofmldata";
        if !Path::new(ofmldata_path).exists() {
            return;
        }

        let reader = OFMLDataReader::new(ofmldata_path);
        let manufacturers = reader.discover_manufacturers();

        // Should find multiple manufacturers
        assert!(
            manufacturers.len() >= 1,
            "Should find at least one manufacturer"
        );

        // Test any manufacturer found
        for mfr in manufacturers.iter().take(3) {
            let products = reader.discover_products(mfr);
            println!("Manufacturer {}: {} products", mfr, products.len());
        }
    }

    #[test]
    fn test_alb_archive_open() {
        let alb_path = match get_test_alb_path() {
            Some(p) => p,
            None => return,
        };

        let archive = AlbArchive::open(&alb_path).expect("Should open ALB");

        let files = archive.list_files();
        assert!(!files.is_empty(), "ALB should contain files");

        let cls_files = archive.get_cls_files();
        assert!(!cls_files.is_empty(), "ALB should contain CLS files");

        let ds_files = archive.get_3ds_files();
        assert!(!ds_files.is_empty(), "ALB should contain 3DS files");
    }

    #[test]
    fn test_alb_extract_cls() {
        let alb_path = match get_test_alb_path() {
            Some(p) => p,
            None => return,
        };

        let mut archive = AlbArchive::open(&alb_path).expect("Should open ALB");

        let cls_files = archive.get_cls_files();
        if !cls_files.is_empty() {
            let content = archive
                .extract_cls(&cls_files[0])
                .expect("Should extract CLS");
            assert!(!content.is_empty(), "CLS content should not be empty");
            // CLS files should contain valid OFML syntax
            assert!(
                content.contains("class") || content.contains("//") || content.contains("package"),
                "CLS should contain OFML constructs"
            );
        }
    }

    #[test]
    fn test_alb_extract_3ds() {
        let alb_path = match get_test_alb_path() {
            Some(p) => p,
            None => return,
        };

        let mut archive = AlbArchive::open(&alb_path).expect("Should open ALB");

        let files = archive.get_3ds_files();
        if !files.is_empty() {
            let scene = archive.extract_3ds(&files[0]).expect("Should extract 3DS");
            assert!(!scene.meshes.is_empty(), "3DS should have meshes");
        }
    }

    #[test]
    fn test_sedus_product_geo() {
        let ofmldata_path = "/workspace/ofmldata";
        if !Path::new(ofmldata_path).exists() {
            return;
        }

        let reader = OFMLDataReader::new(ofmldata_path);
        // Test with any sbu product that has GEO files
        if let Ok(product) = reader.load_product("sbu", "cosmo") {
            if product.has_geo() {
                assert!(!product.geo_files.is_empty(), "Should have GEO files");
            }
        }
    }

    #[test]
    fn test_vitra_product_alb() {
        let ofmldata_path = "/workspace/ofmldata";
        if !Path::new(ofmldata_path).exists() {
            return;
        }

        let reader = OFMLDataReader::new(ofmldata_path);
        if let Ok(product) = reader.load_product("vitra", "workit") {
            assert!(product.has_alb(), "Product should have ALB files");
            assert!(!product.album_files.is_empty(), "Should have ALB files");
        }
    }

    #[test]
    fn test_extract_all_cls() {
        let alb_path = match get_test_alb_path() {
            Some(p) => p,
            None => return,
        };

        let cls_files = extract_cls_from_alb(&alb_path).expect("Should extract CLS files");
        assert!(!cls_files.is_empty(), "Should have CLS files");

        // Verify at least one CLS file can be parsed
        let mut parsed = 0;
        for (_name, content) in cls_files.iter().take(5) {
            if crate::parser::Parser::new(content)
                .and_then(|mut p| p.parse())
                .is_ok()
            {
                parsed += 1;
            }
        }
        assert!(parsed > 0, "Should parse at least one CLS file");
    }

    #[test]
    fn test_extract_all_3ds() {
        let alb_path = match get_test_alb_path() {
            Some(p) => p,
            None => return,
        };

        let scenes = extract_3ds_from_alb(&alb_path).expect("Should extract 3DS files");
        assert!(!scenes.is_empty(), "Should have 3DS scenes");

        // Verify each scene has valid geometry
        for (name, scene) in &scenes {
            assert!(
                !scene.meshes.is_empty(),
                "Scene {} should have meshes",
                name
            );
        }
    }
}
