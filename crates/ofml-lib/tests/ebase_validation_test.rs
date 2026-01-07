//! EBase data validation tests
//!
//! Validates that EBase parsing produces sensible data across multiple manufacturers.
//! Helps distinguish between data corruption vs parser bugs.

use ofml_lib::ebase::{EBaseReader, Value};
use ofml_lib::oap::ocd::OcdReader;
use std::collections::HashMap;
use std::path::Path;

/// Test manufacturers with their ebase paths
const MANUFACTURERS: &[(&str, &str)] = &[
    ("sedus_ai", "/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase"),
    ("sedus_se_motion", "/reference/ofmldata/sex/se_motion/DE/1/db/pdata.ebase"),
    ("vitra_classic", "/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase"),
    ("haw_s100", "/reference/ofmldata/haw/s_100/DE/1/db/pdata.ebase"),
    ("interstuhl_every", "/reference/ofmldata/int/every/DE/1/db/pdata.ebase"),
    ("wilkhahn_on", "/reference/ofmldata/wkh/on/DE/1/db/pdata.ebase"),
    ("bene_arc", "/reference/ofmldata/bene/arc/DE/1/db/pdata.ebase"),
];

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

/// Check if a string contains binary garbage (control chars, etc)
fn has_binary_garbage(s: &str) -> bool {
    s.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
}

/// Check if a string looks like an article number (mostly digits, 6-10 chars)
fn looks_like_article_nr(s: &str) -> bool {
    s.len() >= 6 && s.len() <= 12 && s.chars().filter(|c| c.is_ascii_digit()).count() >= 5
}

/// Check if a string looks like a series code (short, alphanumeric)
fn looks_like_series(s: &str) -> bool {
    s.len() <= 10 && !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[test]
fn test_multi_manufacturer_data_quality() {
    println!("\n=== Multi-Manufacturer EBase Data Quality Report ===\n");

    let mut total_issues = 0;

    for (name, path) in MANUFACTURERS {
        if !Path::new(path).exists() {
            println!("[SKIP] {} - file not found", name);
            continue;
        }

        let mut reader = match EBaseReader::open(Path::new(path)) {
            Ok(r) => r,
            Err(e) => {
                println!("[ERROR] {} - failed to open: {}", name, e);
                continue;
            }
        };

        // Check ocd_article table
        let records = match reader.read_records("ocd_article", None) {
            Ok(r) => r,
            Err(e) => {
                println!("[ERROR] {} - failed to read ocd_article: {}", name, e);
                continue;
            }
        };

        let mut corrupt_series = 0;
        let mut swapped_fields = 0;
        let mut corrupt_article_nr = 0;
        let mut empty_series = 0;
        let mut empty_article_nr = 0;

        for record in &records {
            let article_nr = get_string(record, "article_nr");
            let series = get_string(record, "series");

            // Check for empty fields
            if article_nr.is_empty() {
                empty_article_nr += 1;
            }
            if series.is_empty() {
                empty_series += 1;
            }

            // Check for binary garbage
            if has_binary_garbage(&series) {
                corrupt_series += 1;
            }
            if has_binary_garbage(&article_nr) {
                corrupt_article_nr += 1;
            }

            // Check for swapped fields (article_nr looks like series, series looks like article_nr)
            if looks_like_series(&article_nr) && looks_like_article_nr(&series) {
                swapped_fields += 1;
            }
        }

        let total = records.len();
        let issues = corrupt_series + swapped_fields + corrupt_article_nr;
        total_issues += issues;

        let status = if issues == 0 { "[OK]" } else { "[ISSUES]" };

        println!("{} {} ({} articles):", status, name, total);
        if corrupt_series > 0 {
            println!("  - corrupt_series: {} ({:.1}%)", corrupt_series, 100.0 * corrupt_series as f64 / total as f64);
        }
        if swapped_fields > 0 {
            println!("  - swapped_fields: {} ({:.1}%)", swapped_fields, 100.0 * swapped_fields as f64 / total as f64);
        }
        if corrupt_article_nr > 0 {
            println!("  - corrupt_article_nr: {}", corrupt_article_nr);
        }
        if empty_series > 0 {
            println!("  - empty_series: {}", empty_series);
        }
        if empty_article_nr > 0 {
            println!("  - empty_article_nr: {}", empty_article_nr);
        }
        if issues == 0 {
            println!("  (all fields valid)");
        }
    }

    println!("\n=== Summary ===");
    println!("Total issues across all manufacturers: {}", total_issues);

    // If ONLY Vitra has issues, it's likely data corruption
    // If multiple manufacturers have issues, it's likely a parser bug
}

#[test]
fn test_ebase_string_pool_integrity() {
    println!("\n=== EBase String Pool Integrity Test ===\n");

    // Test specifically the Vitra file to understand the corruption pattern
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    let mut reader = EBaseReader::open(path).unwrap();

    // Get table info
    if let Some(table) = reader.tables.get("ocd_article") {
        println!("ocd_article table:");
        println!("  columns: {:?}", table.columns.iter().map(|c| &c.name).collect::<Vec<_>>());
    }

    // Read raw records and examine the string values
    let records = reader.read_records("ocd_article", None).unwrap();

    println!("\nExamining corrupted records:");
    let mut corrupt_count = 0;
    for record in &records {
        let article_nr = get_string(record, "article_nr");
        let series = get_string(record, "series");

        if has_binary_garbage(&series) {
            corrupt_count += 1;
            if corrupt_count <= 5 {
                println!("\n  article_nr='{}' ({} chars)", article_nr, article_nr.len());
                println!("  series bytes: {:?}", series.as_bytes().iter().take(20).collect::<Vec<_>>());
                println!("  series len: {}", series.len());

                // Check if series looks like a string pool offset issue
                // (random bytes suggest reading from wrong location)
                let has_nulls = series.bytes().any(|b| b == 0);
                let mostly_printable = series.chars().filter(|c| c.is_ascii_graphic() || c.is_whitespace()).count() > series.len() / 2;
                println!("  has_nulls: {}, mostly_printable: {}", has_nulls, mostly_printable);
            }
        }
    }
    println!("\nTotal corrupted records: {} / {}", corrupt_count, records.len());
}

#[test]
fn test_compare_ocd_reader_vs_raw() {
    println!("\n=== Compare OcdReader vs Raw EBaseReader ===\n");

    // This test checks if OcdReader processes data differently than raw reading
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    // Read with OcdReader
    let ocd_reader = OcdReader::from_ebase(path).unwrap();

    // Read raw
    let mut ebase_reader = EBaseReader::open(path).unwrap();
    let raw_records = ebase_reader.read_records("ocd_article", None).unwrap();

    println!("OcdReader articles: {}", ocd_reader.articles.len());
    println!("Raw EBase records: {}", raw_records.len());

    // Check if article counts match
    if ocd_reader.articles.len() != raw_records.len() {
        println!("WARNING: Different article counts - OcdReader may be filtering!");
    }

    // Find a corrupted record in raw and check if it appears in OcdReader
    for raw_record in raw_records.iter().take(5) {
        let raw_nr = get_string(raw_record, "article_nr");
        let raw_series = get_string(raw_record, "series");

        let ocd_article = ocd_reader.articles.iter().find(|a| a.article_nr == raw_nr);

        println!("\nRaw: article_nr='{}' series='{}'", raw_nr, raw_series.escape_debug());
        if let Some(ocd) = ocd_article {
            println!("OCD: article_nr='{}' series='{}'", ocd.article_nr, ocd.series.escape_debug());
            if raw_series != ocd.series {
                println!("MISMATCH in series field!");
            }
        } else {
            println!("OCD: NOT FOUND");
        }
    }
}

#[test]
fn test_sedus_data_quality() {
    println!("\n=== Sedus Data Quality (Known Good Manufacturer) ===\n");

    // Sedus is known to work well - use it as a baseline
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Sedus AI not found");
        return;
    }

    let reader = OcdReader::from_ebase(path).unwrap();

    println!("Articles: {}", reader.articles.len());
    for art in &reader.articles {
        let has_issues = has_binary_garbage(&art.series) || has_binary_garbage(&art.article_nr);
        println!("  {} series='{}' type='{}' {}",
            art.article_nr, art.series, art.art_type,
            if has_issues { "[ISSUE]" } else { "" });
    }

    // If Sedus has no issues, then our parser is likely correct
    let issues = reader.articles.iter()
        .filter(|a| has_binary_garbage(&a.series) || has_binary_garbage(&a.article_nr))
        .count();

    println!("\nSedus issues: {}", issues);
    assert_eq!(issues, 0, "Sedus should have no data quality issues");
}
