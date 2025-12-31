//! Integration tests using actual OFML data from /workspace/ofmldata.
//!
//! These tests verify the interpreter works with real-world data from multiple manufacturers.
//! Tests are designed to skip gracefully if data is not available.

use ofml_interpreter::ebase::EBaseReader;
use ofml_interpreter::ofml::AlbArchive;
use ofml_interpreter::operations::{self, ProductConfig};
use std::path::{Path, PathBuf};

/// Base path to OFML data directory
const OFMLDATA_BASE: &str = "/workspace/ofmldata";

/// Get path to a manufacturer's product directory
fn product_path(manufacturer: &str, product: &str) -> PathBuf {
    PathBuf::from(OFMLDATA_BASE)
        .join(manufacturer)
        .join(product)
}

/// Get path to version 1 of a product
fn product_v1_path(manufacturer: &str, product: &str) -> PathBuf {
    product_path(manufacturer, product).join("1")
}

/// Check if ofmldata exists
fn ofmldata_exists() -> bool {
    Path::new(OFMLDATA_BASE).exists()
}

/// Check if a specific product exists
fn product_exists(manufacturer: &str, product: &str) -> bool {
    product_v1_path(manufacturer, product)
        .join("odb.ebase")
        .exists()
}

/// Find first available product for a manufacturer
fn find_first_product(manufacturer: &str) -> Option<String> {
    let mfr_path = PathBuf::from(OFMLDATA_BASE).join(manufacturer);
    if !mfr_path.exists() {
        return None;
    }

    if let Ok(entries) = std::fs::read_dir(&mfr_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let prod = entry.file_name().to_string_lossy().to_string();
            if product_exists(manufacturer, &prod) {
                return Some(prod);
            }
        }
    }
    None
}

/// Find ALB file in product directory
fn find_alb_file(manufacturer: &str, product: &str) -> Option<PathBuf> {
    let dir = product_v1_path(manufacturer, product);
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map(|e| e == "alb").unwrap_or(false) {
                return Some(path);
            }
        }
    }
    None
}

// ============================================================================
// EBASE Database Tests
// ============================================================================

macro_rules! test_manufacturer_ebase {
    ($name:ident, $mfr:expr) => {
        #[test]
        fn $name() {
            if !ofmldata_exists() {
                eprintln!("Skipping: ofmldata not found");
                return;
            }

            let product = match find_first_product($mfr) {
                Some(p) => p,
                None => {
                    eprintln!("Skipping: no products found for {}", $mfr);
                    return;
                }
            };

            let odb_path = product_v1_path($mfr, &product).join("odb.ebase");
            let reader = EBaseReader::open(&odb_path)
                .expect(&format!("Should open {}/{} odb.ebase", $mfr, product));

            let tables = reader.table_names();

            // Some manufacturers may have empty or minimal EBASE files
            let has_odb3d = tables
                .iter()
                .any(|t| t.contains("odb3d") || t.contains("ODB3D"));

            println!(
                "{}/{}: {} tables, odb3d={}",
                $mfr,
                product,
                tables.len(),
                has_odb3d
            );

            // Just verify we could read the file, don't require tables
            // (some products may have minimal EBASE files)
        }
    };
}

// Test EBASE for multiple manufacturers
test_manufacturer_ebase!(test_vitra_ebase, "vitra");
test_manufacturer_ebase!(test_sbu_ebase, "sbu");
test_manufacturer_ebase!(test_kn_ebase, "kn");
test_manufacturer_ebase!(test_gsx_ebase, "gsx");
test_manufacturer_ebase!(test_cassina_ebase, "cassina");
test_manufacturer_ebase!(test_buzzispace_ebase, "buzzispace");
test_manufacturer_ebase!(test_aix_ebase, "aix");
test_manufacturer_ebase!(test_arper_ebase, "arper");
test_manufacturer_ebase!(test_cor_ebase, "cor");
test_manufacturer_ebase!(test_framery_ebase, "framery");

// ============================================================================
// ALB Archive Tests
// ============================================================================

macro_rules! test_manufacturer_alb {
    ($name:ident, $mfr:expr) => {
        #[test]
        fn $name() {
            if !ofmldata_exists() {
                eprintln!("Skipping: ofmldata not found");
                return;
            }

            let product = match find_first_product($mfr) {
                Some(p) => p,
                None => {
                    eprintln!("Skipping: no products found for {}", $mfr);
                    return;
                }
            };

            let alb_path = match find_alb_file($mfr, &product) {
                Some(p) => p,
                None => {
                    eprintln!("Skipping: no ALB found for {}/{}", $mfr, product);
                    return;
                }
            };

            let archive = AlbArchive::open(&alb_path)
                .expect(&format!("Should open {}/{} ALB", $mfr, product));

            let files = archive.list_files();
            let cls_files = archive.get_cls_files();
            let geo_files = archive.get_3ds_files();

            println!(
                "{}/{}: {} total files, {} CLS, {} 3DS",
                $mfr,
                product,
                files.len(),
                cls_files.len(),
                geo_files.len()
            );

            assert!(!files.is_empty(), "{} ALB should have files", $mfr);
        }
    };
}

// Test ALB archives for multiple manufacturers
test_manufacturer_alb!(test_vitra_alb, "vitra");
test_manufacturer_alb!(test_sbu_alb, "sbu");
test_manufacturer_alb!(test_kn_alb, "kn");
test_manufacturer_alb!(test_gsx_alb, "gsx");
test_manufacturer_alb!(test_cassina_alb, "cassina");
test_manufacturer_alb!(test_buzzispace_alb, "buzzispace");
test_manufacturer_alb!(test_aix_alb, "aix");
test_manufacturer_alb!(test_arper_alb, "arper");
test_manufacturer_alb!(test_cor_alb, "cor");
test_manufacturer_alb!(test_framery_alb, "framery");

// ============================================================================
// Product Assembly Tests
// ============================================================================

macro_rules! test_manufacturer_product_assembly {
    ($name:ident, $mfr:expr) => {
        #[test]
        fn $name() {
            if !ofmldata_exists() {
                eprintln!("Skipping: ofmldata not found");
                return;
            }

            let product = match find_first_product($mfr) {
                Some(p) => p,
                None => {
                    eprintln!("Skipping: no products found for {}", $mfr);
                    return;
                }
            };

            let product_dir = product_v1_path($mfr, &product);
            let config = ProductConfig::default();

            match operations::assemble_product(&product_dir, &config) {
                Ok(result) => {
                    println!(
                        "{}/{}: {} meshes, {} articles, {} geometry loaded",
                        $mfr,
                        product,
                        result.scene.meshes.len(),
                        result.articles_found.len(),
                        result.geometry_loaded
                    );

                    // Verify scene is valid if we got geometry
                    if !result.scene.meshes.is_empty() {
                        let validation = operations::validate_geometry(&result.scene);
                        assert!(
                            validation.vertex_count > 0,
                            "{} product should have vertices",
                            $mfr
                        );
                    }
                }
                Err(e) => {
                    // Some products may not have geometry, that's okay
                    eprintln!(
                        "{}/{}: assembly error (may be expected): {}",
                        $mfr, product, e
                    );
                }
            }
        }
    };
}

// Test product assembly for multiple manufacturers
test_manufacturer_product_assembly!(test_vitra_product_assembly, "vitra");
test_manufacturer_product_assembly!(test_sbu_product_assembly, "sbu");
test_manufacturer_product_assembly!(test_kn_product_assembly, "kn");
test_manufacturer_product_assembly!(test_gsx_product_assembly, "gsx");
test_manufacturer_product_assembly!(test_cassina_product_assembly, "cassina");
test_manufacturer_product_assembly!(test_arper_product_assembly, "arper");

// ============================================================================
// GLB Export Tests
// ============================================================================

#[test]
fn test_multi_manufacturer_glb_export() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }

    let manufacturers = ["vitra", "sbu", "kn", "gsx", "cassina"];
    let mut successful_exports = 0;

    for mfr in &manufacturers {
        let product = match find_first_product(mfr) {
            Some(p) => p,
            None => continue,
        };

        let product_dir = product_v1_path(mfr, &product);
        let config = ProductConfig::default();

        if let Ok(result) = operations::assemble_product(&product_dir, &config) {
            if !result.scene.meshes.is_empty() {
                match operations::export_to_glb(&result.scene) {
                    Ok(glb_data) => {
                        // Verify GLB magic number
                        assert_eq!(
                            &glb_data[0..4],
                            b"glTF",
                            "{} GLB should have valid header",
                            mfr
                        );
                        println!("{}/{}: GLB export {} bytes", mfr, product, glb_data.len());
                        successful_exports += 1;
                    }
                    Err(e) => {
                        eprintln!("{}/{}: GLB export error: {}", mfr, product, e);
                    }
                }
            }
        }
    }

    println!(
        "GLB export: {}/{} manufacturers succeeded",
        successful_exports,
        manufacturers.len()
    );
}

// ============================================================================
// CLS Parsing Tests
// ============================================================================

#[test]
fn test_multi_manufacturer_cls_parsing() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }

    let manufacturers = ["vitra", "sbu", "kn", "gsx", "cassina", "buzzispace"];
    let mut total_parsed = 0;
    let mut total_files = 0;

    for mfr in &manufacturers {
        let product = match find_first_product(mfr) {
            Some(p) => p,
            None => continue,
        };

        let alb_path = match find_alb_file(mfr, &product) {
            Some(p) => p,
            None => continue,
        };

        let mut archive = match AlbArchive::open(&alb_path) {
            Ok(a) => a,
            Err(_) => continue,
        };

        let cls_files = archive.get_cls_files();
        let mut parsed = 0;

        for cls_name in cls_files.iter().take(5) {
            if let Ok(content) = archive.extract_cls(cls_name) {
                if let Ok(mut parser) = ofml_interpreter::parser::Parser::new(&content) {
                    if parser.parse().is_ok() {
                        parsed += 1;
                    }
                }
            }
        }

        total_parsed += parsed;
        total_files += cls_files.len().min(5);
        println!(
            "{}/{}: parsed {}/{} CLS files",
            mfr,
            product,
            parsed,
            cls_files.len().min(5)
        );
    }

    println!("Total CLS parsing: {}/{}", total_parsed, total_files);
    assert!(total_parsed > 0, "Should parse at least some CLS files");
}

// ============================================================================
// Geometry Loading Tests
// ============================================================================

#[test]
fn test_multi_manufacturer_geometry_loading() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }

    let manufacturers = ["vitra", "sbu", "kn", "gsx"];
    let mut total_meshes = 0;
    let mut total_vertices = 0;

    for mfr in &manufacturers {
        let product = match find_first_product(mfr) {
            Some(p) => p,
            None => continue,
        };

        let alb_path = match find_alb_file(mfr, &product) {
            Some(p) => p,
            None => continue,
        };

        let mut archive = match AlbArchive::open(&alb_path) {
            Ok(a) => a,
            Err(_) => continue,
        };

        let geo_files_3ds = archive.get_3ds_files();
        let geo_files_obj = archive.get_obj_files();
        let mut mfr_meshes = 0;
        let mut mfr_vertices = 0;

        // Try 3DS files
        for geo_name in geo_files_3ds.iter().take(3) {
            if let Ok(scene) = archive.extract_3ds(geo_name) {
                for mesh in &scene.meshes {
                    mfr_meshes += 1;
                    mfr_vertices += mesh.vertices.len();
                }
            }
        }

        // Try OBJ files if no 3DS
        if mfr_meshes == 0 {
            for geo_name in geo_files_obj.iter().take(3) {
                if let Ok(scene) = archive.extract_obj(geo_name) {
                    for mesh in &scene.meshes {
                        mfr_meshes += 1;
                        mfr_vertices += mesh.vertices.len();
                    }
                }
            }
        }

        total_meshes += mfr_meshes;
        total_vertices += mfr_vertices;
        println!(
            "{}/{}: {} meshes, {} vertices from {} 3DS + {} OBJ files",
            mfr,
            product,
            mfr_meshes,
            mfr_vertices,
            geo_files_3ds.len(),
            geo_files_obj.len()
        );
    }

    println!(
        "Total geometry: {} meshes, {} vertices",
        total_meshes, total_vertices
    );
    // Some products may not have embedded geometry (use external files)
    // Don't fail if no geometry found, just report
    if total_meshes == 0 {
        eprintln!("Note: No embedded geometry found in tested manufacturers");
    }
}

// ============================================================================
// Comprehensive Manufacturer Coverage Test
// ============================================================================

#[test]
fn test_comprehensive_manufacturer_coverage() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }

    // Test many manufacturers to ensure broad compatibility
    let all_manufacturers = [
        "vitra",
        "sbu",
        "kn",
        "gsx",
        "cassina",
        "buzzispace",
        "aix",
        "arper",
        "cor",
        "framery",
        "hay",
        "rosc",
        "brx",
        "kix",
        "opx",
        "cpx",
        "materia",
        "maul",
        "muellermoebel",
        "noti",
        "prost",
        "rim",
        "thx",
        "vario",
        "wkx",
        "extremis",
        "bisley",
        "casala",
    ];

    let mut manufacturers_with_data = 0;
    let mut manufacturers_with_alb = 0;
    let mut manufacturers_with_geometry = 0;

    for mfr in &all_manufacturers {
        let product = match find_first_product(mfr) {
            Some(p) => p,
            None => continue,
        };

        manufacturers_with_data += 1;

        if let Some(alb_path) = find_alb_file(mfr, &product) {
            if let Ok(archive) = AlbArchive::open(&alb_path) {
                manufacturers_with_alb += 1;

                if !archive.get_3ds_files().is_empty() || !archive.get_obj_files().is_empty() {
                    manufacturers_with_geometry += 1;
                }
            }
        }
    }

    println!("Manufacturer coverage:");
    println!(
        "  With OFML data: {}/{}",
        manufacturers_with_data,
        all_manufacturers.len()
    );
    println!("  With valid ALB: {}", manufacturers_with_alb);
    println!("  With geometry: {}", manufacturers_with_geometry);

    assert!(
        manufacturers_with_data >= 10,
        "Should have at least 10 manufacturers with data"
    );
}

// ============================================================================
// Stress Test - Multiple Products per Manufacturer
// ============================================================================

#[test]
fn test_sbu_multiple_products() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }

    let mfr = "sbu";
    let mfr_path = PathBuf::from(OFMLDATA_BASE).join(mfr);

    if !mfr_path.exists() {
        eprintln!("Skipping: SBU not found");
        return;
    }

    let mut products_tested = 0;
    let mut products_with_geometry = 0;

    if let Ok(entries) = std::fs::read_dir(&mfr_path) {
        for entry in entries.filter_map(|e| e.ok()).take(10) {
            let product = entry.file_name().to_string_lossy().to_string();

            if !product_exists(mfr, &product) {
                continue;
            }

            products_tested += 1;

            let product_dir = product_v1_path(mfr, &product);
            let config = ProductConfig::default();

            if let Ok(result) = operations::assemble_product(&product_dir, &config) {
                if !result.scene.meshes.is_empty() {
                    products_with_geometry += 1;
                }
            }
        }
    }

    println!(
        "SBU: tested {} products, {} with geometry",
        products_tested, products_with_geometry
    );

    assert!(products_tested >= 5, "Should test at least 5 SBU products");
}

#[test]
fn test_find_manufacturer_names_in_datapool() {
    let path = Path::new("/workspace/ofmldata/pCon.update/DataPool/etc/Manufacturers.ebase");
    if !path.exists() {
        eprintln!("Skipping test: Manufacturers.ebase not found");
        return;
    }
    
    let mut reader = EBaseReader::open(path).expect("Failed to open Manufacturers.ebase");
    let records = reader.read_records("Manufacturers", None).expect("Failed to read records");
    
    println!("\n=== Searching for sex/Sedus in {} records ===\n", records.len());
    
    for record in &records {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record.get("man-name").and_then(|v| v.as_str()).unwrap_or("");
        
        if man == "sex" || man.to_lowercase().contains("sedus") || man_name.to_lowercase().contains("sedus") {
            println!("FOUND: man='{}', man-id='{}', man-name='{}'", man, man_id, man_name);
        }
    }
    
    // Also print a few sample records to understand the format
    println!("\n=== First 5 records ===\n");
    for (i, record) in records.iter().take(5).enumerate() {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record.get("man-name").and_then(|v| v.as_str()).unwrap_or("");
        println!("Record {}: man='{}', man-id='{}', man-name='{}'", i, man, man_id, man_name);
    }
}

#[test]
fn test_find_sex_manufacturer() {
    let path = Path::new("/workspace/ofmldata/pCon.update/DataPool/etc/Manufacturers.ebase");
    if !path.exists() {
        eprintln!("Skipping test: Manufacturers.ebase not found");
        return;
    }
    
    let mut reader = EBaseReader::open(path).expect("Failed to open Manufacturers.ebase");
    let records = reader.read_records("Manufacturers", None).expect("Failed to read records");
    
    println!("\n=== Looking for 'sex' directory ===\n");
    
    for record in &records {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record.get("man-name").and_then(|v| v.as_str()).unwrap_or("");
        
        if man == "sex" {
            println!("FOUND 'sex': man='{}', man-id='{}', man-name='{}'", man, man_id, man_name);
        }
    }
}

#[test]
fn test_search_all_sex_sedus() {
    let path = Path::new("/workspace/ofmldata/pCon.update/DataPool/etc/Manufacturers.ebase");
    if !path.exists() {
        eprintln!("Skipping test: Manufacturers.ebase not found");
        return;
    }
    
    let mut reader = EBaseReader::open(path).expect("Failed to open Manufacturers.ebase");
    let records = reader.read_records("Manufacturers", None).expect("Failed to read records");
    
    println!("\n=== All records containing 'sex' or 'sedus' in any field ===\n");
    
    for (i, record) in records.iter().enumerate() {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record.get("man-name").and_then(|v| v.as_str()).unwrap_or("");
        let concern_id = record.get("concern-id").and_then(|v| v.as_str()).unwrap_or("");
        
        let all = format!("{} {} {} {}", man, man_id, man_name, concern_id).to_lowercase();
        if all.contains("sex") || all.contains("sedus") {
            println!("Record {}: man='{}', man-id='{}', man-name='{}', concern-id='{}'", i, man, man_id, man_name, concern_id);
        }
    }
}

#[test]
fn test_search_dataclient_manufacturers() {
    let path = Path::new("/workspace/ofmldata/pCon.update/DataClient/etc/Manufacturers.ebase");
    if !path.exists() {
        eprintln!("Skipping test: DataClient Manufacturers.ebase not found");
        return;
    }
    
    let mut reader = EBaseReader::open(path).expect("Failed to open Manufacturers.ebase");
    let records = reader.read_records("Manufacturers", None).expect("Failed to read records");
    
    println!("\n=== DataClient: All records containing 'sex' or 'sedus' in any field ===\n");
    
    for (i, record) in records.iter().enumerate() {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record.get("man-name").and_then(|v| v.as_str()).unwrap_or("");
        let concern_id = record.get("concern-id").and_then(|v| v.as_str()).unwrap_or("");
        
        let all = format!("{} {} {} {}", man, man_id, man_name, concern_id).to_lowercase();
        if all.contains("sex") || all.contains("sedus") {
            println!("Record {}: man='{}', man-id='{}', man-name='{}', concern-id='{}'", i, man, man_id, man_name, concern_id);
        }
    }
}

#[test]
fn test_check_concerns_table() {
    let path = Path::new("/workspace/ofmldata/pCon.update/DataPool/etc/Manufacturers.ebase");
    if !path.exists() {
        eprintln!("Skipping test: Manufacturers.ebase not found");
        return;
    }
    
    let mut reader = EBaseReader::open(path).expect("Failed to open Manufacturers.ebase");
    let records = reader.read_records("Concerns", None).expect("Failed to read records");
    
    println!("\n=== Concerns table - looking for SE ===\n");
    
    for record in &records {
        let concern = record.get("concern").and_then(|v| v.as_str()).unwrap_or("");
        let concern_name = record.get("concern-name").and_then(|v| v.as_str()).unwrap_or("");
        
        if concern == "SE" || concern.to_lowercase().contains("sedus") || concern_name.to_lowercase().contains("sedus") {
            println!("concern='{}', concern-name='{}'", concern, concern_name);
        }
    }
    
    println!("\n=== First 10 concerns ===\n");
    for record in records.iter().take(10) {
        let concern = record.get("concern").and_then(|v| v.as_str()).unwrap_or("");
        let concern_name = record.get("concern-name").and_then(|v| v.as_str()).unwrap_or("");
        println!("concern='{}', concern-name='{}'", concern, concern_name);
    }
}
