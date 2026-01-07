//! Trace record 26 specifically to find the corruption source

use ofml_lib::ebase::EBaseReader;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[test]
fn trace_record_26() {
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    println!("\n=== Tracing Record 26 ===\n");

    let reader = EBaseReader::open(path).unwrap();
    let mut file = BufReader::new(File::open(path).unwrap());

    if let Some(table) = reader.tables.get("ocd_article") {
        // Seek to record 26
        let record_26_pos = table.data_offset as u64 + (26 * table.record_size as u64);
        file.seek(SeekFrom::Start(record_26_pos)).unwrap();

        let mut row_data = vec![0u8; table.record_size as usize];
        file.read_exact(&mut row_data).unwrap();

        println!("Record 26 position: 0x{:08X}", record_26_pos);
        println!("Record 26 raw bytes: {:02X?}", &row_data);

        // Find series column
        if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
            let col_offset = series_col.offset as usize;
            let str_offset_bytes = &row_data[col_offset..col_offset + 4];
            let str_offset = u32::from_be_bytes([
                str_offset_bytes[0],
                str_offset_bytes[1],
                str_offset_bytes[2],
                str_offset_bytes[3],
            ]);

            println!("\nSeries column offset in record: {}", col_offset);
            println!("Series string offset bytes: {:02X?}", str_offset_bytes);
            println!("Series string offset: 0x{:08X} ({})", str_offset, str_offset);

            // Read raw string at that offset
            file.seek(SeekFrom::Start(str_offset as u64)).unwrap();
            let mut len_bytes = [0u8; 2];
            file.read_exact(&mut len_bytes).unwrap();
            let str_len = u16::from_be_bytes(len_bytes);

            println!("\nString length field: {} (0x{:04X})", str_len, str_len);

            // Read more bytes to see what's there
            let read_len = str_len.min(100) as usize;
            let mut str_data = vec![0u8; read_len];
            file.read_exact(&mut str_data).unwrap();

            println!("String bytes: {:02X?}", &str_data);
            let decoded = String::from_utf8_lossy(&str_data);
            println!("Decoded: '{}'", decoded.escape_debug());

            // Check what's around that offset
            println!("\n=== Context around string offset 0x{:08X} ===", str_offset);

            for offset in [str_offset - 20, str_offset - 10, str_offset, str_offset + 10, str_offset + 20] {
                if offset > 0 {
                    file.seek(SeekFrom::Start(offset as u64)).unwrap();
                    let mut context = [0u8; 10];
                    file.read_exact(&mut context).unwrap();
                    let s = String::from_utf8_lossy(&context);
                    println!("  0x{:08X}: {:02X?} '{}'", offset, &context, s.escape_debug());
                }
            }
        }

        // Also check record 0 for comparison
        println!("\n=== Record 0 for comparison ===\n");

        file.seek(SeekFrom::Start(table.data_offset as u64)).unwrap();
        file.read_exact(&mut row_data).unwrap();

        if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
            let col_offset = series_col.offset as usize;
            let str_offset = u32::from_be_bytes([
                row_data[col_offset],
                row_data[col_offset + 1],
                row_data[col_offset + 2],
                row_data[col_offset + 3],
            ]);

            println!("Record 0 series offset: 0x{:08X}", str_offset);

            file.seek(SeekFrom::Start(str_offset as u64)).unwrap();
            let mut len_bytes = [0u8; 2];
            file.read_exact(&mut len_bytes).unwrap();
            let str_len = u16::from_be_bytes(len_bytes);

            println!("String length: {}", str_len);

            let mut str_data = vec![0u8; str_len as usize];
            file.read_exact(&mut str_data).unwrap();
            println!("String bytes: {:02X?}", &str_data);
            let decoded = String::from_utf8_lossy(&str_data);
            println!("Decoded: '{}'", decoded.escape_debug());
        }

        // Key question: are record 26 and record 0 using SAME or DIFFERENT string offsets?
        println!("\n=== Key Analysis ===\n");

        // Read all series offsets
        let mut offsets: Vec<(usize, u32)> = Vec::new();

        file.seek(SeekFrom::Start(table.data_offset as u64)).unwrap();

        for i in 0..table.record_count.min(50) as usize {
            file.read_exact(&mut row_data).unwrap();

            if let Some(series_col) = table.columns.iter().find(|c| c.name == "series") {
                let col_offset = series_col.offset as usize;
                let str_offset = u32::from_be_bytes([
                    row_data[col_offset],
                    row_data[col_offset + 1],
                    row_data[col_offset + 2],
                    row_data[col_offset + 3],
                ]);
                offsets.push((i, str_offset));
            }
        }

        // Find unique offsets
        let mut unique: std::collections::HashMap<u32, Vec<usize>> = std::collections::HashMap::new();
        for (i, off) in &offsets {
            unique.entry(*off).or_default().push(*i);
        }

        println!("Unique series string offsets in first 50 records:");
        for (off, records) in unique.iter() {
            let sample_records: Vec<_> = records.iter().take(5).collect();
            println!("  0x{:08X}: {} records (e.g., {:?})", off, records.len(), sample_records);
        }
    }
}
