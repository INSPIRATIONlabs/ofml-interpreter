//! Multi-manufacturer integration tests for the OFML interpreter.
//!
//! These tests verify that the interpreter can parse and process OFML data
//! from multiple furniture manufacturers, ensuring the implementation is
//! truly generic and not tied to any specific product.

use ofml_lib::ebase::EBaseReader;
use ofml_lib::ofml::{extract_3ds_from_alb, extract_cls_from_alb, AlbArchive};
use std::path::PathBuf;

/// Get the path to a test fixture file
fn fixture_path(manufacturer: &str, product: &str, filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(manufacturer)
        .join(product)
        .join(filename)
}

/// Check if a fixture exists
#[allow(dead_code)]
fn fixture_exists(manufacturer: &str, product: &str, filename: &str) -> bool {
    fixture_path(manufacturer, product, filename).exists()
}

// ============================================================================
// ALB Archive Tests - Multiple Manufacturers
// ============================================================================

#[test]
fn test_vitra_alb_archive() {
    let alb_path = fixture_path("vitra", "workit", "vitra_workit_1.alb");
    if !alb_path.exists() {
        eprintln!("Skipping test: fixture not found at {:?}", alb_path);
        return;
    }

    let archive = AlbArchive::open(&alb_path).expect("Should open Vitra ALB");

    let files = archive.list_files();
    assert!(!files.is_empty(), "Vitra ALB should contain files");

    let cls_files = archive.get_cls_files();
    assert!(!cls_files.is_empty(), "Vitra ALB should contain CLS files");

    let ds_files = archive.get_3ds_files();
    assert!(!ds_files.is_empty(), "Vitra ALB should contain 3DS files");
}

#[test]
fn test_kn_alb_archive() {
    let alb_path = fixture_path("kn", "conline", "kn_conline_2.alb");
    if !alb_path.exists() {
        eprintln!("Skipping test: fixture not found at {:?}", alb_path);
        return;
    }

    let archive = AlbArchive::open(&alb_path).expect("Should open KN ALB");

    let files = archive.list_files();
    assert!(!files.is_empty(), "KN ALB should contain files");

    // KN products should have CLS files
    let cls_files = archive.get_cls_files();
    println!("KN Conline CLS files: {:?}", cls_files.len());
}

#[test]
fn test_cassina_alb_archive() {
    let alb_path = fixture_path("cassina", "abc", "cassina_abc_1.alb");
    if !alb_path.exists() {
        eprintln!("Skipping test: fixture not found at {:?}", alb_path);
        return;
    }

    let archive = AlbArchive::open(&alb_path).expect("Should open Cassina ALB");

    let files = archive.list_files();
    assert!(!files.is_empty(), "Cassina ALB should contain files");

    println!("Cassina ABC total files: {}", files.len());
}

#[test]
fn test_bordbar_alb_archive() {
    let alb_path = fixture_path("bordbar", "basic", "bordbar_basic_1.alb");
    if !alb_path.exists() {
        eprintln!("Skipping test: fixture not found at {:?}", alb_path);
        return;
    }

    let archive = AlbArchive::open(&alb_path).expect("Should open Bordbar ALB");

    let files = archive.list_files();
    assert!(!files.is_empty(), "Bordbar ALB should contain files");

    println!("Bordbar Basic total files: {}", files.len());
}

// ============================================================================
// CLS Extraction Tests - Multiple Manufacturers
// ============================================================================

#[test]
fn test_vitra_cls_extraction() {
    let alb_path = fixture_path("vitra", "workit", "vitra_workit_1.alb");
    if !alb_path.exists() {
        return;
    }

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
fn test_kn_cls_extraction() {
    let alb_path = fixture_path("kn", "conline", "kn_conline_2.alb");
    if !alb_path.exists() {
        return;
    }

    let mut archive = AlbArchive::open(&alb_path).expect("Should open ALB");

    let cls_files = archive.get_cls_files();
    for cls_name in cls_files.iter().take(3) {
        let content = archive.extract_cls(cls_name).expect("Should extract CLS");
        assert!(!content.is_empty(), "CLS {} should have content", cls_name);
    }
}

#[test]
fn test_cassina_cls_extraction() {
    let alb_path = fixture_path("cassina", "abc", "cassina_abc_1.alb");
    if !alb_path.exists() {
        return;
    }

    let mut archive = AlbArchive::open(&alb_path).expect("Should open ALB");

    let cls_files = archive.get_cls_files();
    for cls_name in cls_files.iter().take(3) {
        let content = archive.extract_cls(cls_name).expect("Should extract CLS");
        assert!(!content.is_empty(), "CLS {} should have content", cls_name);
    }
}

// ============================================================================
// 3DS Geometry Extraction Tests
// ============================================================================

#[test]
fn test_vitra_3ds_extraction() {
    let alb_path = fixture_path("vitra", "workit", "vitra_workit_1.alb");
    if !alb_path.exists() {
        return;
    }

    let mut archive = AlbArchive::open(&alb_path).expect("Should open ALB");

    let files = archive.get_3ds_files();
    if !files.is_empty() {
        let scene = archive.extract_3ds(&files[0]).expect("Should extract 3DS");
        assert!(!scene.meshes.is_empty(), "3DS should have meshes");

        // Verify mesh structure
        for mesh in &scene.meshes {
            assert!(
                !mesh.vertices.is_empty(),
                "Mesh {} should have vertices",
                mesh.name
            );
            assert!(
                !mesh.faces.is_empty(),
                "Mesh {} should have faces",
                mesh.name
            );
        }
    }
}

#[test]
fn test_kn_3ds_extraction() {
    let alb_path = fixture_path("kn", "conline", "kn_conline_2.alb");
    if !alb_path.exists() {
        return;
    }

    let mut archive = AlbArchive::open(&alb_path).expect("Should open ALB");

    let files = archive.get_3ds_files();
    for file_name in files.iter().take(5) {
        if let Ok(scene) = archive.extract_3ds(file_name) {
            assert!(
                !scene.meshes.is_empty(),
                "KN 3DS {} should have meshes",
                file_name
            );
        }
    }
}

// ============================================================================
// EBASE Database Tests - Multiple Manufacturers
// ============================================================================

#[test]
fn test_vitra_odb_ebase() {
    let ebase_path = fixture_path("vitra", "workit", "odb.ebase");
    if !ebase_path.exists() {
        return;
    }

    let reader = EBaseReader::open(&ebase_path).expect("Should open Vitra ODB");
    let tables = reader.table_names();
    assert!(!tables.is_empty(), "Vitra ODB should have tables");

    println!("Vitra ODB tables: {:?}", tables);
}

#[test]
fn test_kn_odb_ebase() {
    let ebase_path = fixture_path("kn", "conline", "odb.ebase");
    if !ebase_path.exists() {
        return;
    }

    let reader = EBaseReader::open(&ebase_path).expect("Should open KN ODB");
    let tables = reader.table_names();
    assert!(!tables.is_empty(), "KN ODB should have tables");

    println!("KN ODB tables: {:?}", tables);
}

#[test]
fn test_kn_product_ebase() {
    let ebase_path = fixture_path("kn", "conline", "conline.ebase");
    if !ebase_path.exists() {
        return;
    }

    let reader = EBaseReader::open(&ebase_path).expect("Should open KN Conline EBASE");
    let tables = reader.table_names();
    assert!(!tables.is_empty(), "KN Conline EBASE should have tables");

    println!("KN Conline product tables: {:?}", tables);
}

#[test]
fn test_cassina_odb_ebase() {
    let ebase_path = fixture_path("cassina", "abc", "odb.ebase");
    if !ebase_path.exists() {
        return;
    }

    let reader = EBaseReader::open(&ebase_path).expect("Should open Cassina ODB");
    let tables = reader.table_names();
    assert!(!tables.is_empty(), "Cassina ODB should have tables");

    println!("Cassina ODB tables: {:?}", tables);
}

// ============================================================================
// Full CLS Parsing Tests
// ============================================================================

#[test]
fn test_vitra_full_cls_parsing() {
    let alb_path = fixture_path("vitra", "workit", "vitra_workit_1.alb");
    if !alb_path.exists() {
        return;
    }

    let cls_files = extract_cls_from_alb(&alb_path).expect("Should extract CLS files");
    assert!(!cls_files.is_empty(), "Should have CLS files");

    // Try to parse each CLS file
    let mut parsed_count = 0;
    for (name, content) in &cls_files {
        match ofml_lib::parser::Parser::new(content) {
            Ok(mut parser) => match parser.parse() {
                Ok(_) => parsed_count += 1,
                Err(e) => {
                    eprintln!("Parse error in {}: {}", name, e);
                }
            },
            Err(e) => {
                eprintln!("Lexer error in {}: {}", name, e);
            }
        }
    }

    println!(
        "Vitra Workit: Parsed {}/{} CLS files",
        parsed_count,
        cls_files.len()
    );
    // We expect at least some files to parse successfully
    assert!(parsed_count > 0, "Should parse at least some CLS files");
}

#[test]
fn test_kn_full_cls_parsing() {
    let alb_path = fixture_path("kn", "conline", "kn_conline_2.alb");
    if !alb_path.exists() {
        return;
    }

    let cls_files = extract_cls_from_alb(&alb_path).expect("Should extract CLS files");

    let mut parsed_count = 0;

    for (_name, content) in &cls_files {
        if let Ok(mut parser) = ofml_lib::parser::Parser::new(content) {
            if parser.parse().is_ok() {
                parsed_count += 1;
            }
        }
    }

    println!(
        "KN Conline: Parsed {}/{} CLS files",
        parsed_count,
        cls_files.len()
    );
}

#[test]
fn test_cassina_full_cls_parsing() {
    let alb_path = fixture_path("cassina", "abc", "cassina_abc_1.alb");
    if !alb_path.exists() {
        return;
    }

    let cls_files = extract_cls_from_alb(&alb_path).expect("Should extract CLS files");

    let mut parsed_count = 0;

    for (_name, content) in &cls_files {
        if let Ok(mut parser) = ofml_lib::parser::Parser::new(content) {
            if parser.parse().is_ok() {
                parsed_count += 1;
            }
        }
    }

    println!(
        "Cassina ABC: Parsed {}/{} CLS files",
        parsed_count,
        cls_files.len()
    );
}

// ============================================================================
// Full 3DS Extraction Tests
// ============================================================================

#[test]
fn test_vitra_full_3ds_extraction() {
    let alb_path = fixture_path("vitra", "workit", "vitra_workit_1.alb");
    if !alb_path.exists() {
        return;
    }

    let scenes = extract_3ds_from_alb(&alb_path).expect("Should extract 3DS files");
    assert!(!scenes.is_empty(), "Should have 3DS scenes");

    let mut total_meshes = 0;
    let mut total_vertices = 0;

    for (_name, scene) in &scenes {
        for mesh in &scene.meshes {
            total_meshes += 1;
            total_vertices += mesh.vertices.len();
        }
    }

    println!(
        "Vitra Workit: {} scenes, {} meshes, {} vertices",
        scenes.len(),
        total_meshes,
        total_vertices
    );

    assert!(total_meshes > 0, "Should have meshes");
}

#[test]
fn test_kn_full_3ds_extraction() {
    let alb_path = fixture_path("kn", "conline", "kn_conline_2.alb");
    if !alb_path.exists() {
        return;
    }

    let scenes = extract_3ds_from_alb(&alb_path).expect("Should extract 3DS files");

    let mut total_meshes = 0;

    for (_name, scene) in &scenes {
        total_meshes += scene.meshes.len();
    }

    println!(
        "KN Conline: {} scenes, {} meshes",
        scenes.len(),
        total_meshes
    );
}

// ============================================================================
// Cross-Manufacturer Compatibility Tests
// ============================================================================

/// Test that all manufacturers' ALB files can be opened with the same API
#[test]
fn test_all_manufacturers_alb_compatible() {
    let manufacturers = [
        ("vitra", "workit", "vitra_workit_1.alb"),
        ("kn", "conline", "kn_conline_2.alb"),
        ("cassina", "abc", "cassina_abc_1.alb"),
        ("bordbar", "basic", "bordbar_basic_1.alb"),
    ];

    let mut success_count = 0;

    for (mfr, product, filename) in &manufacturers {
        let path = fixture_path(mfr, product, filename);
        if !path.exists() {
            continue;
        }

        match AlbArchive::open(&path) {
            Ok(archive) => {
                let files = archive.list_files();
                println!("{}/{}: {} files", mfr, product, files.len());
                success_count += 1;
            }
            Err(e) => {
                eprintln!("Failed to open {}/{}: {}", mfr, product, e);
            }
        }
    }

    assert!(
        success_count >= 3,
        "Should open at least 3 manufacturer ALBs"
    );
}

/// Test that EBASE files from different manufacturers use compatible formats
#[test]
fn test_all_manufacturers_ebase_compatible() {
    let ebase_files = [
        ("vitra", "workit", "odb.ebase"),
        ("kn", "conline", "odb.ebase"),
        ("cassina", "abc", "odb.ebase"),
    ];

    let mut success_count = 0;

    for (mfr, product, filename) in &ebase_files {
        let path = fixture_path(mfr, product, filename);
        if !path.exists() {
            continue;
        }

        match EBaseReader::open(&path) {
            Ok(reader) => {
                let tables = reader.table_names();
                println!("{}/{} {}: {} tables", mfr, product, filename, tables.len());
                success_count += 1;
            }
            Err(e) => {
                eprintln!("Failed to open {}/{} {}: {}", mfr, product, filename, e);
            }
        }
    }

    assert!(
        success_count >= 2,
        "Should open at least 2 manufacturer EBASE files"
    );
}
