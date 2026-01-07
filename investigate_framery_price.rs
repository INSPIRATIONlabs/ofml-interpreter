use std::path::Path;

mod ebase;
use ebase::EBaseReader;

fn main() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase");

    println!("Opening: {}", path.display());

    let mut reader = match EBaseReader::open(path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open: {}", e);
            return;
        }
    };

    println!("\nVersion: {}.{}", reader.major_version, reader.minor_version);
    println!("\nAvailable tables:");
    for table_name in reader.table_names() {
        let table = reader.get_table(table_name).unwrap();
        println!("  {} - {} records, {} bytes/record",
                 table_name, table.record_count, table.record_size);
    }

    // Examine ocd_price table structure
    if let Some(table) = reader.get_table("ocd_price") {
        println!("\n=== ocd_price Table Schema ===");
        println!("Record count: {}", table.record_count);
        println!("Record size: {} bytes", table.record_size);
        println!("\nColumns:");
        for (i, col) in table.columns.iter().enumerate() {
            println!("  [{}] {} - type_id={}, offset={}, size={}, flags={}",
                     i, col.name, col.type_id, col.offset, col.size, col.flags);
        }

        // Read all price records
        match reader.read_records("ocd_price", None) {
            Ok(records) => {
                println!("\n=== All Price Records ===");
                for (idx, record) in records.iter().enumerate() {
                    println!("\n--- Record {} ---", idx);

                    // Get all fields
                    let article_nr = record.get("article_nr")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(null)");
                    let price_type = record.get("price_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(null)");
                    let price_textnr = record.get("price_textnr")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(null)");
                    let level = record.get("level")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(null)");
                    let var_cond = record.get("var_cond")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(null)");
                    let price = record.get("price")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let currency = record.get("currency")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(null)");

                    println!("  article_nr: {}", article_nr);
                    println!("  price_type: {}", price_type);
                    println!("  price_textnr: {}", price_textnr);
                    println!("  level: {}", level);
                    println!("  var_cond: {}", var_cond);
                    println!("  price: {}", price);
                    println!("  currency: {}", currency);

                    // Print all fields to see what's there
                    println!("  All fields:");
                    for (key, value) in record {
                        println!("    {}: {:?}", key, value);
                    }
                }
            }
            Err(e) => eprintln!("Failed to read records: {}", e),
        }
    } else {
        println!("\nNo ocd_price table found!");
    }
}
