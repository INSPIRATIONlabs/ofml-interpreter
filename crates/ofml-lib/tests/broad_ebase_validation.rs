//! Broad EBase validation across many manufacturers
//!
//! Tests data quality across all available manufacturers to determine
//! if issues are isolated or systemic.

use ofml_lib::ebase::{EBaseReader, Value};
use std::collections::HashMap;
use std::path::Path;

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::UInt(u) => u.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Blob(b) => format!("blob:{}", b),
        Value::Null => String::new(),
    }
}

fn get_string(record: &HashMap<String, Value>, key: &str) -> String {
    record.get(key).map(value_to_string).unwrap_or_default()
}

fn has_binary_garbage(s: &str) -> bool {
    s.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
}

/// Comprehensive test across all available pdata files
#[test]
fn test_all_available_manufacturers() {
    let test_files = [
        // Different manufacturers
        ("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase", "sedus_ai"),
        ("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase", "vitra_classic"),
        ("/reference/ofmldata/sbu/abov/DE/1/db/pdata.ebase", "steelcase_abov"),
        ("/reference/ofmldata/sbu/aki/DE/1/db/pdata.ebase", "steelcase_aki"),
        ("/reference/ofmldata/bordbar/bordbar/DE/1/db/pdata.ebase", "bordbar"),
        ("/reference/ofmldata/chatboard/ch/ANY/1/db/pdata.ebase", "chatboard"),
        ("/reference/ofmldata/frmr/four/DE/1/db/pdata.ebase", "framery_four"),
        ("/reference/ofmldata/frmr/one/DE/1/db/pdata.ebase", "framery_one"),
        ("/reference/ofmldata/fat/fat/DE/1/db/pdata.ebase", "fast"),
        // Other Vitra series to see if it's all Vitra
        ("/reference/ofmldata/vitra/accessories/DE/1/db/pdata.ebase", "vitra_accessories"),
        ("/reference/ofmldata/vitra/prouve/DE/1/db/pdata.ebase", "vitra_prouve"),
        ("/reference/ofmldata/vitra/vitra_home/DE/1/db/pdata.ebase", "vitra_home"),
    ];

    println!("\n=== Broad EBase Validation ===\n");
    println!("{:<25} {:>8} {:>10} {:>10} {:>8}", "Manufacturer", "Articles", "Corrupt", "Empty", "Status");
    println!("{}", "-".repeat(65));

    let mut manufacturers_with_issues = Vec::new();
    let mut manufacturers_ok = Vec::new();

    for (path, name) in &test_files {
        if !Path::new(path).exists() {
            println!("{:<25} {:>8}", name, "SKIP");
            continue;
        }

        let mut reader = match EBaseReader::open(Path::new(path)) {
            Ok(r) => r,
            Err(e) => {
                println!("{:<25} ERROR: {}", name, e);
                continue;
            }
        };

        let records = match reader.read_records("ocd_article", None) {
            Ok(r) => r,
            Err(_) => {
                println!("{:<25} {:>8}", name, "NO TABLE");
                continue;
            }
        };

        let total = records.len();
        let mut corrupt = 0;
        let mut empty = 0;

        for record in &records {
            let series = get_string(record, "series");
            let article_nr = get_string(record, "article_nr");

            if has_binary_garbage(&series) || has_binary_garbage(&article_nr) {
                corrupt += 1;
            }
            if series.is_empty() || article_nr.is_empty() {
                empty += 1;
            }
        }

        let status = if corrupt > 0 { "CORRUPT" } else if empty > total / 2 { "EMPTY" } else { "OK" };

        println!("{:<25} {:>8} {:>10} {:>10} {:>8}",
            name, total, corrupt, empty, status);

        if corrupt > 0 {
            manufacturers_with_issues.push((*name, corrupt, total));
        } else {
            manufacturers_ok.push(*name);
        }
    }

    println!("\n=== Analysis ===");
    println!("Manufacturers with corruption issues: {}", manufacturers_with_issues.len());
    for (name, corrupt, total) in &manufacturers_with_issues {
        println!("  - {}: {} corrupted out of {} ({:.1}%)",
            name, corrupt, total, 100.0 * *corrupt as f64 / *total as f64);
    }

    println!("\nManufacturers with clean data: {}", manufacturers_ok.len());
    for name in &manufacturers_ok {
        println!("  - {}", name);
    }

    // Conclusion
    println!("\n=== Conclusion ===");
    if manufacturers_with_issues.len() == 1 && manufacturers_with_issues[0].0.starts_with("vitra") {
        println!("ONLY Vitra has data corruption - likely DATA ISSUE, not parser bug");
    } else if manufacturers_with_issues.len() > 1 {
        println!("Multiple manufacturers affected - likely PARSER BUG");
    } else {
        println!("No corruption found - data quality is good");
    }
}

/// Test specifically looking at the binary pattern of corrupt data
#[test]
fn test_vitra_corruption_pattern() {
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    let mut reader = EBaseReader::open(path).unwrap();
    let records = reader.read_records("ocd_article", None).unwrap();

    println!("\n=== Vitra Corruption Pattern Analysis ===\n");

    // Group corrupted records by their series byte pattern
    let mut patterns: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    for record in &records {
        let series = get_string(record, "series");
        let article_nr = get_string(record, "article_nr");

        if has_binary_garbage(&series) {
            let first_bytes: Vec<u8> = series.bytes().take(10).collect();
            patterns.entry(first_bytes).or_default().push(article_nr);
        }
    }

    println!("Unique corruption patterns: {}", patterns.len());
    for (pattern, articles) in &patterns {
        println!("\nPattern (hex): {:02X?}", pattern);
        println!("  Affects {} articles: {:?}", articles.len(), articles.iter().take(5).collect::<Vec<_>>());

        // Try to interpret the pattern
        // Check if it starts with printable chars then binary
        let as_string: String = pattern.iter().take_while(|&&b| b >= 32 && b < 127).map(|&b| b as char).collect();
        if !as_string.is_empty() {
            println!("  Starts with printable: '{}'", as_string);
        }
    }
}
