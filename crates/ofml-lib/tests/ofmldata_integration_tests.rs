//! Integration tests using actual OFML data from /workspace/ofmldata.
//!
//! These tests verify the interpreter works with real-world data from multiple manufacturers.
//! Tests are designed to skip gracefully if data is not available.

use ofml_lib::ebase::EBaseReader;
use ofml_lib::ofml::AlbArchive;
use ofml_lib::operations::{self, ProductConfig};
use std::path::{Path, PathBuf};

/// Base path to OFML data directory
const OFMLDATA_BASE: &str = "/reference/ofmldata";

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
                if let Ok(mut parser) = ofml_lib::parser::Parser::new(&content) {
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
    let records = reader
        .read_records("Manufacturers", None)
        .expect("Failed to read records");

    println!(
        "\n=== Searching for sex/Sedus in {} records ===\n",
        records.len()
    );

    for record in &records {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record
            .get("man-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if man == "sex"
            || man.to_lowercase().contains("sedus")
            || man_name.to_lowercase().contains("sedus")
        {
            println!(
                "FOUND: man='{}', man-id='{}', man-name='{}'",
                man, man_id, man_name
            );
        }
    }

    // Also print a few sample records to understand the format
    println!("\n=== First 5 records ===\n");
    for (i, record) in records.iter().take(5).enumerate() {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record
            .get("man-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        println!(
            "Record {}: man='{}', man-id='{}', man-name='{}'",
            i, man, man_id, man_name
        );
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
    let records = reader
        .read_records("Manufacturers", None)
        .expect("Failed to read records");

    println!("\n=== Looking for 'sex' directory ===\n");

    for record in &records {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record
            .get("man-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if man == "sex" {
            println!(
                "FOUND 'sex': man='{}', man-id='{}', man-name='{}'",
                man, man_id, man_name
            );
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
    let records = reader
        .read_records("Manufacturers", None)
        .expect("Failed to read records");

    println!("\n=== All records containing 'sex' or 'sedus' in any field ===\n");

    for (i, record) in records.iter().enumerate() {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record
            .get("man-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let concern_id = record
            .get("concern-id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let all = format!("{} {} {} {}", man, man_id, man_name, concern_id).to_lowercase();
        if all.contains("sex") || all.contains("sedus") {
            println!(
                "Record {}: man='{}', man-id='{}', man-name='{}', concern-id='{}'",
                i, man, man_id, man_name, concern_id
            );
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
    let records = reader
        .read_records("Manufacturers", None)
        .expect("Failed to read records");

    println!("\n=== DataClient: All records containing 'sex' or 'sedus' in any field ===\n");

    for (i, record) in records.iter().enumerate() {
        let man = record.get("man").and_then(|v| v.as_str()).unwrap_or("");
        let man_id = record.get("man-id").and_then(|v| v.as_str()).unwrap_or("");
        let man_name = record
            .get("man-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let concern_id = record
            .get("concern-id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let all = format!("{} {} {} {}", man, man_id, man_name, concern_id).to_lowercase();
        if all.contains("sex") || all.contains("sedus") {
            println!(
                "Record {}: man='{}', man-id='{}', man-name='{}', concern-id='{}'",
                i, man, man_id, man_name, concern_id
            );
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
    let records = reader
        .read_records("Concerns", None)
        .expect("Failed to read records");

    println!("\n=== Concerns table - looking for SE ===\n");

    for record in &records {
        let concern = record.get("concern").and_then(|v| v.as_str()).unwrap_or("");
        let concern_name = record
            .get("concern-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if concern == "SE"
            || concern.to_lowercase().contains("sedus")
            || concern_name.to_lowercase().contains("sedus")
        {
            println!("concern='{}', concern-name='{}'", concern, concern_name);
        }
    }

    println!("\n=== First 10 concerns ===\n");
    for record in records.iter().take(10) {
        let concern = record.get("concern").and_then(|v| v.as_str()).unwrap_or("");
        let concern_name = record
            .get("concern-name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        println!("concern='{}', concern-name='{}'", concern, concern_name);
    }
}

// ============================================================================
// OCD Price Table Tests
// ============================================================================

#[test]
fn test_ocd_price_columns_sedus() {
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!(
            "Skipping test: Sedus AI pdata.ebase not found at {:?}",
            path
        );
        return;
    }

    let reader = EBaseReader::open(path).expect("Failed to open pdata.ebase");

    println!("\n=== OCD Price Table Structure ===\n");

    if let Some(table) = reader.tables.get("ocd_price") {
        println!("ocd_price table: {} records", table.record_count);
        println!("\nColumns:");
        for col in &table.columns {
            println!("  {} (type_id={})", col.name, col.type_id);
        }
    } else {
        eprintln!("No ocd_price table found!");
    }
}

#[test]
fn test_ocd_price_reading_with_price_level() {
    use ofml_lib::oap::ocd::OcdReader;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping test: Sedus AI pdata.ebase not found");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Failed to load OCD");

    println!("\n=== OCD Price Reading Test ===\n");
    println!("Total prices: {}", reader.prices.len());

    // Count by price_level
    let base_prices: Vec<_> = reader
        .prices
        .iter()
        .filter(|p| p.price_level == "B")
        .collect();
    let surcharges: Vec<_> = reader
        .prices
        .iter()
        .filter(|p| p.price_level == "X")
        .collect();
    let discounts: Vec<_> = reader
        .prices
        .iter()
        .filter(|p| p.price_level == "D")
        .collect();
    let empty_level: Vec<_> = reader
        .prices
        .iter()
        .filter(|p| p.price_level.is_empty())
        .collect();

    println!("  Base prices (level='B'): {}", base_prices.len());
    println!("  Surcharges (level='X'): {}", surcharges.len());
    println!("  Discounts (level='D'): {}", discounts.len());
    println!("  Empty level: {}", empty_level.len());

    // Show some sample prices
    println!("\n=== Sample Prices ===\n");

    // Sample base price
    if let Some(base) = base_prices.first() {
        println!("Base price example:");
        println!("  article_nr: {}", base.article_nr);
        println!("  var_cond: '{}'", base.var_cond);
        println!("  price_level: '{}'", base.price_level);
        println!("  is_fix: {}", base.is_fix);
        println!("  price: {:.2} {}", base.price, base.currency);
    }

    // Sample surcharge
    if let Some(surcharge) = surcharges.first() {
        println!("\nSurcharge example:");
        println!("  article_nr: {}", surcharge.article_nr);
        println!("  var_cond: '{}'", surcharge.var_cond);
        println!("  price_level: '{}'", surcharge.price_level);
        println!("  is_fix: {}", surcharge.is_fix);
        println!("  price: {:.2} {}", surcharge.price, surcharge.currency);
    }

    // Test get_base_price
    if !reader.articles.is_empty() {
        let first_article = &reader.articles[0].article_nr;
        println!("\n=== get_base_price test for {} ===\n", first_article);

        if let Some(base) = reader.get_base_price(first_article) {
            println!("Found base price:");
            println!("  price_level: '{}'", base.price_level);
            println!("  price: {:.2} {}", base.price, base.currency);
        } else {
            println!("No base price found");
        }

        // Test get_surcharges
        let surcharges = reader.get_surcharges(first_article);
        println!("\nSurcharges for {}: {}", first_article, surcharges.len());
        for s in surcharges.iter().take(5) {
            println!(
                "  var_cond='{}' price={:.2} is_fix={}",
                s.var_cond, s.price, s.is_fix
            );
        }
    }
}

#[test]
fn test_multi_manufacturer_price_reading() {
    use ofml_lib::oap::ocd::OcdReader;

    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }

    let manufacturers = ["sex", "vitra", "kn", "sbu", "haw", "aix"];

    println!("\n=== Multi-Manufacturer Price Reading ===\n");

    for mfr in &manufacturers {
        // Find pdata.ebase files
        let mfr_path = PathBuf::from(OFMLDATA_BASE).join(mfr);
        if !mfr_path.exists() {
            continue;
        }

        let pdata_files = find_pdata_files_recursive(&mfr_path);
        if pdata_files.is_empty() {
            continue;
        }

        // Read first pdata.ebase
        let pdata_path = &pdata_files[0];
        match OcdReader::from_ebase(pdata_path) {
            Ok(reader) => {
                let base_count = reader
                    .prices
                    .iter()
                    .filter(|p| p.price_level == "B")
                    .count();
                let surcharge_count = reader
                    .prices
                    .iter()
                    .filter(|p| p.price_level == "X")
                    .count();
                let empty_count = reader
                    .prices
                    .iter()
                    .filter(|p| p.price_level.is_empty())
                    .count();

                println!(
                    "{}: {} prices (B:{}, X:{}, empty:{})",
                    mfr,
                    reader.prices.len(),
                    base_count,
                    surcharge_count,
                    empty_count
                );

                // Show if price_level field is being used
                if base_count > 0 || surcharge_count > 0 {
                    println!("  ✓ price_level field is populated");
                } else if empty_count > 0 {
                    println!("  ⚠ price_level field is empty (using var_cond fallback)");
                }
            }
            Err(e) => {
                println!("{}: Error reading pdata - {}", mfr, e);
            }
        }
    }
}

/// Find all pdata.ebase files recursively
fn find_pdata_files_recursive(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                files.extend(find_pdata_files_recursive(&entry_path));
            } else if entry_path.file_name().map_or(false, |n| n == "pdata.ebase") {
                files.push(entry_path);
            }
        }
    }
    files
}

#[test]
fn test_debug_variant_matching() {
    use ofml_lib::oap::families::{FamilyConfiguration, FamilyLoader};
    use ofml_lib::oap::ocd::OcdReader;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: Sedus AI not found");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("load");

    // Get prices for AI-121
    let prices = reader.get_prices("AI-121");

    println!("\n=== AI-121 Prices ===");
    println!("Total: {}", prices.len());

    // Show base price
    if let Some(base) = prices.iter().find(|p| p.price_level == "B") {
        println!(
            "\nBase price: {:.2} EUR (var_cond='{}')",
            base.price, base.var_cond
        );
    }

    // Show all unique var_cond patterns
    println!("\nAll unique var_cond patterns:");
    let mut var_conds: Vec<_> = prices.iter().map(|p| p.var_cond.as_str()).collect();
    var_conds.sort();
    var_conds.dedup();
    for vc in &var_conds {
        let price = prices.iter().find(|p| &p.var_cond == vc).unwrap();
        println!(
            "  '{}' -> {:.2} EUR (level='{}')",
            vc, price.price, price.price_level
        );
    }

    // Now check what the variant code looks like
    let mfr_path = Path::new("/reference/ofmldata/sex");
    let loader = FamilyLoader::load(mfr_path, "DE");

    // Find AI family
    for family in loader.get_families() {
        if family.base_article_nr.starts_with("AI-") {
            println!("\n=== Family: {} ===", family.name);
            println!("Base article: {}", family.base_article_nr);

            let properties = loader.get_properties_for_family(family);
            println!("Properties: {}", properties.len());

            for prop in &properties {
                println!(
                    "  {} ({} options): default={:?}",
                    prop.key,
                    prop.options.len(),
                    prop.default_value
                );
                for opt in prop.options.iter().take(5) {
                    println!("    - '{}' ({})", opt.value, opt.label);
                }
                if prop.options.len() > 5 {
                    println!("    ... and {} more", prop.options.len() - 5);
                }
            }

            let config = FamilyConfiguration::new(&family.id, &properties);
            println!("\nGenerated variant_code: '{}'", config.variant_code);

            // Show selections
            println!("\nSelections:");
            for (k, v) in &config.selections {
                println!("  {}='{}'", k, v);
            }

            break;
        }
    }
}

#[test]
fn test_sedus_price_mapping_tables() {
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: Sedus AI not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("load");

    println!("\n=== All Tables in pdata.ebase ===\n");
    for (table_name, table) in &reader.tables {
        println!("{}: {} records", table_name, table.record_count);
    }

    // Look at ocd_relation table - contains the mapping logic
    println!("\n=== ocd_relation Sample Records ===\n");
    if let Ok(records) = reader.read_records("ocd_relation", None) {
        // Find records referencing surcharge codes
        let surcharge_codes = ["S_1801", "S_166", "S_1513", "S_PGX"];

        for code in &surcharge_codes {
            println!("Looking for rel_name containing '{}':", code);
            for rec in records
                .iter()
                .filter(|r| {
                    r.get("rel_name")
                        .and_then(|v| v.as_str())
                        .map_or(false, |s| s.contains(code))
                })
                .take(5)
            {
                let rel_name = rec.get("rel_name").and_then(|v| v.as_str()).unwrap_or("");
                let rel_blocknr = rec.get("rel_blocknr").and_then(|v| v.as_i64()).unwrap_or(0);
                let rel_block = rec.get("rel_block").and_then(|v| v.as_str()).unwrap_or("");
                println!(
                    "  rel_name='{}' blocknr={} block='{}'",
                    rel_name, rel_blocknr, rel_block
                );
            }
            println!();
        }

        // Look at first few relations to understand structure
        println!("First 10 ocd_relation records:");
        for rec in records.iter().take(10) {
            let rel_name = rec.get("rel_name").and_then(|v| v.as_str()).unwrap_or("");
            let rel_blocknr = rec.get("rel_blocknr").and_then(|v| v.as_i64()).unwrap_or(0);
            let rel_block = rec.get("rel_block").and_then(|v| v.as_str()).unwrap_or("");
            println!(
                "  rel_name='{}' blocknr={} block='{}'",
                rel_name, rel_blocknr, rel_block
            );
        }
    }

    // Look at ocd_price - check the actual var_cond format
    println!("\n=== ocd_price Surcharge Codes ===\n");
    if let Ok(records) = reader.read_records("ocd_price", None) {
        for rec in records
            .iter()
            .filter(|r| {
                r.get("price_level")
                    .and_then(|v| v.as_str())
                    .map_or(false, |s| s == "X")
            })
            .take(15)
        {
            let article_nr = rec.get("article_nr").and_then(|v| v.as_str()).unwrap_or("");
            let var_cond = rec.get("var_cond").and_then(|v| v.as_str()).unwrap_or("");
            let price = rec.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            println!(
                "  article='{}' var_cond='{}' price={:.2}",
                article_nr, var_cond, price
            );
        }
    }

    // Check ocd_propertyvalue for rel_obj that might link to surcharges
    println!("\n=== ocd_propertyvalue with rel_obj ===\n");
    if let Ok(records) = reader.read_records("ocd_propertyvalue", None) {
        // Show records with non-zero rel_obj
        for rec in records
            .iter()
            .filter(|r| {
                r.get("rel_obj")
                    .and_then(|v| v.as_i64())
                    .map_or(false, |i| i != 0)
            })
            .take(20)
        {
            let prop_class = rec.get("prop_class").and_then(|v| v.as_str()).unwrap_or("");
            let property = rec.get("property").and_then(|v| v.as_str()).unwrap_or("");
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
            println!(
                "  prop_class='{}' property='{}' rel_obj={}",
                prop_class, property, rel_obj
            );
        }

        // Also show fabric/color related properties
        println!("\nS_STOFF property values:");
        for rec in records
            .iter()
            .filter(|r| {
                r.get("property")
                    .and_then(|v| v.as_str())
                    .map_or(false, |s| s.starts_with("S_STOFF") || s == "S_FUSSFARBE")
            })
            .take(10)
        {
            let prop_class = rec.get("prop_class").and_then(|v| v.as_str()).unwrap_or("");
            let property = rec.get("property").and_then(|v| v.as_str()).unwrap_or("");
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
            let pos_pval = rec.get("pos_pval").and_then(|v| v.as_i64()).unwrap_or(0);
            println!(
                "  prop_class='{}' property='{}' pos={} rel_obj={}",
                prop_class, property, pos_pval, rel_obj
            );
        }
    }

    // Look at ocd_relationobj with Domain='P' for pricing
    println!("\n=== ocd_relationobj (pricing) ===\n");
    if let Ok(records) = reader.read_records("ocd_relationobj", None) {
        println!("First 10 relationobj records:");
        for rec in records.iter().take(10) {
            for (k, v) in rec {
                print!("{}='{:?}' ", k, v);
            }
            println!();
        }
    }
}

#[test]
fn test_sedus_article_base_prices() {
    use ofml_lib::oap::ocd::OcdReader;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: Sedus AI not found");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("load");

    println!("\n=== Base Prices for Different Articles ===\n");

    // Get all articles
    let articles = &reader.articles;
    println!("Total articles in OCD: {}", articles.len());

    for art in articles {
        println!("\nArticle: {}", art.article_nr);

        // Get base price (level='B')
        if let Some(base) = reader.get_base_price(&art.article_nr) {
            println!(
                "  Base price: {:.2} {} (level='{}', var_cond='{}')",
                base.price, base.currency, base.price_level, base.var_cond
            );
        } else {
            println!("  No base price found");
        }

        // Count surcharges
        let surcharges = reader.get_surcharges(&art.article_nr);
        println!("  Surcharges: {} entries", surcharges.len());

        // Show unique var_cond codes
        let codes: std::collections::HashSet<_> =
            surcharges.iter().map(|s| s.var_cond.as_str()).collect();
        if !codes.is_empty() {
            println!("  Surcharge codes: {:?}", codes);
        }
    }
}

#[test]
fn test_sedus_price_rules() {
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: Sedus AI not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("load");

    println!("\n=== ocd_price with price_rule ===\n");
    if let Ok(records) = reader.read_records("ocd_price", None) {
        for rec in records.iter().take(20) {
            let article_nr = rec.get("article_nr").and_then(|v| v.as_str()).unwrap_or("");
            let var_cond = rec.get("var_cond").and_then(|v| v.as_str()).unwrap_or("");
            let price_rule = rec.get("price_rule").and_then(|v| v.as_str()).unwrap_or("");
            let price_level = rec
                .get("price_level")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let price = rec.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);

            if !price_rule.is_empty() || price_level == "X" {
                println!(
                    "article='{}' var_cond='{}' level='{}' price={:.2} rule='{}'",
                    article_nr, var_cond, price_level, price, price_rule
                );
            }
        }
    }
}
