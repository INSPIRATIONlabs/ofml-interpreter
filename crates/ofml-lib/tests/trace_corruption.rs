//! Trace the exact cause of Vitra string corruption

use ofml_lib::ebase::{EBaseReader, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
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

#[test]
fn trace_vitra_corruption() {
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    println!("\n=== Tracing Vitra Corruption ===\n");

    // Read with EBaseReader
    let mut reader = EBaseReader::open(path).unwrap();
    let records = reader.read_records("ocd_article", Some(10)).unwrap();

    println!("First 10 records from EBaseReader:");
    for (i, record) in records.iter().enumerate() {
        let article_nr = get_string(record, "article_nr");
        let series = get_string(record, "series");

        let corrupted = series.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t');

        println!("  Record {}: article_nr='{}' series='{}' {}",
            i, article_nr, series.escape_debug(),
            if corrupted { "[CORRUPT]" } else { "" });
    }

    // Now read raw bytes to understand what the parser is doing
    println!("\n=== Raw Byte Analysis ===\n");

    let mut file = BufReader::new(File::open(path).unwrap());

    // Get table info
    if let Some(table) = reader.tables.get("ocd_article") {
        println!("Table info:");
        println!("  data_offset: 0x{:08X}", table.data_offset);
        println!("  record_count: {}", table.record_count);
        println!("  record_size: {}", table.record_size);

        // Find the series column
        if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
            println!("\nseries column:");
            println!("  type_id: {}", series_col.type_id);
            println!("  offset in record: {}", series_col.offset);
            println!("  size: {}", series_col.size);
        }

        // Read first few records raw
        file.seek(SeekFrom::Start(table.data_offset as u64)).unwrap();
        let mut row_data = vec![0u8; table.record_size as usize];

        // Find corrupted record
        for i in 0..table.record_count.min(100) {
            file.read_exact(&mut row_data).unwrap();

            if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
                let col_offset = series_col.offset as usize;
                let str_offset = u32::from_be_bytes([
                    row_data[col_offset],
                    row_data[col_offset + 1],
                    row_data[col_offset + 2],
                    row_data[col_offset + 3],
                ]);

                // Skip if offset is 0
                if str_offset == 0 {
                    continue;
                }

                // Read string manually at that offset
                let pos = file.stream_position().unwrap();
                file.seek(SeekFrom::Start(str_offset as u64)).unwrap();

                let mut len_bytes = [0u8; 2];
                file.read_exact(&mut len_bytes).unwrap();
                let str_len = u16::from_be_bytes(len_bytes) as usize;

                let mut str_data = vec![0u8; str_len.min(50)];
                file.read_exact(&mut str_data).unwrap();

                let raw_string = String::from_utf8_lossy(&str_data);
                let has_control = raw_string.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t');

                if has_control && i < 10 {
                    println!("\nRecord {} (corrupt when read raw):", i);
                    println!("  str_offset: 0x{:08X}", str_offset);
                    println!("  str_len: {}", str_len);
                    println!("  raw bytes: {:02X?}", &str_data);
                    println!("  decoded: '{}'", raw_string.escape_debug());

                    // Compare with what EBaseReader returned
                    let ebase_series = get_string(&records[i as usize], "series");
                    println!("  EBaseReader returned: '{}'", ebase_series.escape_debug());
                }

                // Restore position
                file.seek(SeekFrom::Start(pos)).unwrap();
            }
        }
    }

    // Check if the corruption pattern matches string pool boundaries
    println!("\n=== String Pool Boundary Analysis ===\n");

    // Read header to get string pool info
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut full_header = [0u8; 56];
    file.read_exact(&mut full_header).unwrap();

    let correct_pool_offset = u32::from_be_bytes([full_header[16], full_header[17], full_header[18], full_header[19]]);
    let correct_pool_size = u32::from_be_bytes([full_header[20], full_header[21], full_header[22], full_header[23]]);

    let parser_pool_offset = u32::from_be_bytes([full_header[20], full_header[21], full_header[22], full_header[23]]);
    let parser_pool_size = u32::from_be_bytes([full_header[40], full_header[41], full_header[42], full_header[43]]);

    println!("Correct string pool: offset=0x{:08X} size={}", correct_pool_offset, correct_pool_size);
    println!("Parser thinks:       offset=0x{:08X} size={}", parser_pool_offset, parser_pool_size);

    // Find articles with string offsets outside the "parser's" perceived pool
    if let Some(table) = reader.tables.get("ocd_article") {
        file.seek(SeekFrom::Start(table.data_offset as u64)).unwrap();
        let mut row_data = vec![0u8; table.record_size as usize];

        let mut outside_pool = 0;
        let mut inside_pool = 0;

        for _ in 0..table.record_count {
            file.read_exact(&mut row_data).unwrap();

            if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
                let col_offset = series_col.offset as usize;
                let str_offset = u32::from_be_bytes([
                    row_data[col_offset],
                    row_data[col_offset + 1],
                    row_data[col_offset + 2],
                    row_data[col_offset + 3],
                ]);

                if str_offset == 0 {
                    continue;
                }

                // Check if offset is within parser's perceived pool
                if parser_pool_offset > 0 {
                    let pool_end = parser_pool_offset + parser_pool_size;
                    if str_offset < parser_pool_offset || str_offset >= pool_end {
                        outside_pool += 1;
                    } else {
                        inside_pool += 1;
                    }
                } else {
                    // Parser thinks pool starts at 0 - all offsets would be "valid"
                    inside_pool += 1;
                }
            }
        }

        println!("\nString offsets inside parser's pool: {}", inside_pool);
        println!("String offsets outside parser's pool: {}", outside_pool);
    }
}
