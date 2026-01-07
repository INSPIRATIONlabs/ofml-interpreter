//! Debug test to understand Vitra EBase corruption

use ofml_lib::ebase::EBaseReader;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[test]
fn debug_vitra_ebase_structure() {
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    println!("\n=== Vitra EBase Debug ===\n");

    // Open file directly for raw access
    let mut file = BufReader::new(File::open(path).unwrap());

    // Read header
    let mut header = [0u8; 52];
    file.read_exact(&mut header).unwrap();

    println!("Header bytes (first 52):");
    for (i, chunk) in header.chunks(16).enumerate() {
        print!("  {:04X}: ", i * 16);
        for b in chunk {
            print!("{:02X} ", b);
        }
        println!();
    }

    // Parse header
    let magic = &header[0..6];
    let major = u16::from_be_bytes([header[6], header[7]]);
    let minor = u16::from_be_bytes([header[8], header[9]]);
    let string_pool_offset = u32::from_be_bytes([header[0x10], header[0x11], header[0x12], header[0x13]]);
    let string_pool_size = u32::from_be_bytes([header[0x14], header[0x15], header[0x16], header[0x17]]);

    println!("\nParsed header:");
    println!("  Magic: {:?}", String::from_utf8_lossy(magic));
    println!("  Version: {}.{}", major, minor);
    println!("  String pool offset: 0x{:08X} ({})", string_pool_offset, string_pool_offset);
    println!("  String pool size: 0x{:08X} ({})", string_pool_size, string_pool_size);

    // Also open with EBaseReader to compare
    let mut reader = EBaseReader::open(path).unwrap();
    println!("\nEBaseReader version: {}.{}", reader.major_version, reader.minor_version);

    // Get ocd_article table info
    if let Some(table) = reader.tables.get("ocd_article") {
        println!("\nocd_article table:");
        println!("  data_offset: {}", table.data_offset);
        println!("  record_count: {}", table.record_count);
        println!("  record_size: {}", table.record_size);
        println!("  Columns:");
        for col in &table.columns {
            println!("    {} type_id={} offset={} size={}", col.name, col.type_id, col.offset, col.size);
        }
    }

    // Read a few raw records to see the bytes
    let records = reader.read_records("ocd_article", Some(5)).unwrap();
    println!("\nFirst 5 records (raw column values):");

    // Read raw bytes for first record
    if let Some(table) = reader.tables.get("ocd_article") {
        file.seek(SeekFrom::Start(table.data_offset as u64)).unwrap();
        let mut row_data = vec![0u8; table.record_size as usize];

        for i in 0..5.min(table.record_count as usize) {
            file.read_exact(&mut row_data).unwrap();

            println!("\n  Record {}:", i);
            println!("    Raw bytes: {:02X?}", &row_data[..row_data.len().min(40)]);

            // Parse series field (offset and string)
            if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
                let col_offset = series_col.offset as usize;
                if col_offset + 4 <= row_data.len() {
                    let str_offset = u32::from_be_bytes([
                        row_data[col_offset],
                        row_data[col_offset + 1],
                        row_data[col_offset + 2],
                        row_data[col_offset + 3],
                    ]);
                    println!("    series string offset: 0x{:08X} ({})", str_offset, str_offset);

                    // Read the string at that offset
                    if str_offset > 0 {
                        file.seek(SeekFrom::Start(str_offset as u64)).unwrap();
                        let mut str_header = [0u8; 2];
                        file.read_exact(&mut str_header).unwrap();
                        let str_len = u16::from_be_bytes(str_header);
                        println!("    series string length: {} (0x{:04X})", str_len, str_len);

                        if str_len < 200 {
                            let mut str_data = vec![0u8; str_len as usize];
                            file.read_exact(&mut str_data).unwrap();
                            println!("    series string bytes: {:02X?}", &str_data[..str_data.len().min(30)]);
                            let s = String::from_utf8_lossy(&str_data);
                            println!("    series string value: '{}'", s.escape_debug());
                        }

                        // Restore position
                        file.seek(SeekFrom::Start((table.data_offset as u64) + ((i + 1) as u64 * table.record_size as u64))).unwrap();
                    }
                }
            }
        }
    }

    // Check string pool directly
    println!("\n=== String Pool Analysis ===");
    file.seek(SeekFrom::Start(string_pool_offset as u64)).unwrap();
    let mut pool_sample = [0u8; 100];
    file.read_exact(&mut pool_sample).unwrap();
    println!("String pool starts with: {:02X?}", &pool_sample[..50]);

    // Try to read a few strings from the pool
    for offset in [string_pool_offset, string_pool_offset + 10, string_pool_offset + 100] {
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let mut len_bytes = [0u8; 2];
        if file.read_exact(&mut len_bytes).is_ok() {
            let len = u16::from_be_bytes(len_bytes);
            if len < 100 {
                let mut data = vec![0u8; len as usize];
                if file.read_exact(&mut data).is_ok() {
                    let s = String::from_utf8_lossy(&data);
                    println!("  String at 0x{:08X}: len={} value='{}'", offset, len, s.escape_debug());
                }
            } else {
                println!("  String at 0x{:08X}: len={} (too long, suspicious)", offset, len);
            }
        }
    }
}

#[test]
fn compare_vitra_vs_sedus_string_offsets() {
    println!("\n=== Compare Vitra vs Sedus String Pool Structure ===\n");

    for (name, path) in [
        ("Vitra", "/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase"),
        ("Sedus", "/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase"),
    ] {
        let p = Path::new(path);
        if !p.exists() {
            println!("{}: SKIP", name);
            continue;
        }

        let mut file = BufReader::new(File::open(p).unwrap());
        let mut header = [0u8; 52];
        file.read_exact(&mut header).unwrap();

        let major = u16::from_be_bytes([header[6], header[7]]);
        let minor = u16::from_be_bytes([header[8], header[9]]);
        let string_pool_offset = u32::from_be_bytes([header[0x10], header[0x11], header[0x12], header[0x13]]);
        let string_pool_size = u32::from_be_bytes([header[0x14], header[0x15], header[0x16], header[0x17]]);

        println!("{}:", name);
        println!("  Version: {}.{}", major, minor);
        println!("  String pool: offset=0x{:08X} size={}", string_pool_offset, string_pool_size);

        // Read first string in pool
        if string_pool_offset > 0 {
            file.seek(SeekFrom::Start(string_pool_offset as u64)).unwrap();
            let mut len_bytes = [0u8; 2];
            file.read_exact(&mut len_bytes).unwrap();
            let len = u16::from_be_bytes(len_bytes);
            println!("  First string length: {} (BE: {:02X}{:02X})", len, len_bytes[0], len_bytes[1]);

            // Also try little endian
            let len_le = u16::from_le_bytes(len_bytes);
            println!("  If LE interpretation: {}", len_le);
        }
        println!();
    }
}
