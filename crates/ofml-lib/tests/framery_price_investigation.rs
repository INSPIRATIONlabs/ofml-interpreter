use ofml_lib::ebase::{EBaseReader, Value};
use std::path::Path;

#[test]
fn investigate_framery_price_corruption() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase");

    let mut reader = EBaseReader::open(path).expect("Failed to open Framery ONE COMPACT data");

    println!("\n=== Framery ONE COMPACT Price Investigation ===");
    println!("Version: {}.{}", reader.major_version, reader.minor_version);

    // Examine ocd_price table structure
    let table = reader
        .get_table("ocd_price")
        .expect("No ocd_price table found")
        .clone();

    println!("\n=== ocd_price Table Schema ===");
    println!("Record count: {}", table.record_count);
    println!("Record size: {} bytes", table.record_size);
    println!("\nColumns:");
    for (i, col) in table.columns.iter().enumerate() {
        let col_type = col
            .column_type()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|| format!("Unknown({})", col.type_id));
        println!(
            "  [{}] {:20} type={:20} offset={:4} size={:4} flags={}",
            i, col.name, col_type, col.offset, col.size, col.flags
        );
    }

    // Read all price records
    let records = reader
        .read_records("ocd_price", None)
        .expect("Failed to read records");

    println!("\n=== All Price Records ({} total) ===", records.len());
    for (idx, record) in records.iter().enumerate() {
        println!("\n--- Record {} ---", idx);

        // Print all fields
        for (key, value) in record {
            match value {
                Value::String(s) => println!("  {:20} = \"{}\"", key, s),
                Value::Float(f) => println!("  {:20} = {}", key, f),
                Value::Int(i) => println!("  {:20} = {}", key, i),
                Value::UInt(u) => println!("  {:20} = {}", key, u),
                Value::Null => println!("  {:20} = (null)", key),
                Value::Blob(b) => println!("  {:20} = Blob({})", key, b),
            }
        }
    }

    // Now let's examine the raw binary data for record 13
    println!("\n=== Raw Binary Analysis for Record 13 ===");
    if records.len() > 13 {
        let record_offset = table.data_offset as u64 + (13 * table.record_size as u64);
        println!(
            "Record offset in file: 0x{:08x} ({})",
            record_offset, record_offset
        );
        println!("Record size: {} bytes", table.record_size);
    }
}
