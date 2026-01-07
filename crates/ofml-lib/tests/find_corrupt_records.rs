//! Find and display actual corrupt records

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

#[test]
fn find_corrupt_records() {
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    println!("\n=== Finding Corrupt Records ===\n");

    let mut reader = EBaseReader::open(path).unwrap();
    let records = reader.read_records("ocd_article", None).unwrap();

    println!("Total records: {}", records.len());

    let mut corrupt_count = 0;
    let mut corrupt_samples = Vec::new();

    for (i, record) in records.iter().enumerate() {
        let article_nr = get_string(record, "article_nr");
        let series = get_string(record, "series");

        let series_corrupt = has_binary_garbage(&series);
        let article_corrupt = has_binary_garbage(&article_nr);

        if series_corrupt || article_corrupt {
            corrupt_count += 1;
            if corrupt_samples.len() < 10 {
                corrupt_samples.push((i, article_nr.clone(), series.clone(), series_corrupt, article_corrupt));
            }
        }
    }

    println!("Corrupt records: {} / {}", corrupt_count, records.len());

    if !corrupt_samples.is_empty() {
        println!("\nFirst {} corrupt records:", corrupt_samples.len());
        for (i, article_nr, series, series_bad, article_bad) in &corrupt_samples {
            println!("\n  Record {}:", i);
            println!("    article_nr: '{}' (len={}) {}", article_nr.escape_debug(), article_nr.len(),
                if *article_bad { "[CORRUPT]" } else { "" });
            println!("    article_nr bytes: {:02X?}", article_nr.as_bytes());
            println!("    series: '{}' (len={}) {}", series.escape_debug(), series.len(),
                if *series_bad { "[CORRUPT]" } else { "" });
            println!("    series bytes: {:02X?}", series.as_bytes());
        }
    }

    // Now let's also print some "good" records for comparison
    println!("\n=== Sample Good Records ===\n");
    for (i, record) in records.iter().enumerate().take(5) {
        let article_nr = get_string(record, "article_nr");
        let series = get_string(record, "series");

        if !has_binary_garbage(&series) && !has_binary_garbage(&article_nr) {
            println!("  Record {}: article_nr='{}' series='{}'", i, article_nr, series);
        }
    }

    // Check if the "corrupt" data might actually be valid multi-byte chars
    println!("\n=== Checking for Multi-byte Characters ===\n");
    for (i, article_nr, series, series_bad, article_bad) in &corrupt_samples {
        if *series_bad {
            let bytes = series.as_bytes();
            let is_valid_utf8 = std::str::from_utf8(bytes).is_ok();
            println!("  Record {} series: utf8_valid={} bytes={:02X?}", i, is_valid_utf8, bytes);
        }
    }
}
