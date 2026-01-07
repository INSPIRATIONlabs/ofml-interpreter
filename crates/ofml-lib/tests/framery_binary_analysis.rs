use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[test]
fn analyze_framery_binary_corruption() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase");

    let mut file = File::open(path).expect("Failed to open file");

    // Record 13 is at offset 0x34d8 (13528 decimal)
    // Record size is 56 bytes
    let record_offset = 0x34d8u64;

    println!("\n=== Binary Analysis of Record 13 ===");
    println!("Record offset: 0x{:08x} ({})", record_offset, record_offset);

    // Read the entire record
    file.seek(SeekFrom::Start(record_offset)).unwrap();
    let mut record_data = [0u8; 56];
    file.read_exact(&mut record_data).unwrap();

    println!("\nRaw record bytes:");
    for (i, chunk) in record_data.chunks(16).enumerate() {
        print!("  {:04x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }

    // Parse the string offsets according to schema
    let article_nr_offset = u32::from_be_bytes([
        record_data[4],
        record_data[5],
        record_data[6],
        record_data[7],
    ]);
    let var_cond_offset = u32::from_be_bytes([
        record_data[8],
        record_data[9],
        record_data[10],
        record_data[11],
    ]);
    let price_type_offset = u32::from_be_bytes([
        record_data[12],
        record_data[13],
        record_data[14],
        record_data[15],
    ]);
    let price_level_offset = u32::from_be_bytes([
        record_data[16],
        record_data[17],
        record_data[18],
        record_data[19],
    ]);
    let price_rule_offset = u32::from_be_bytes([
        record_data[20],
        record_data[21],
        record_data[22],
        record_data[23],
    ]);
    let price_textnr_offset = u32::from_be_bytes([
        record_data[24],
        record_data[25],
        record_data[26],
        record_data[27],
    ]);
    let price_float = f32::from_be_bytes([
        record_data[28],
        record_data[29],
        record_data[30],
        record_data[31],
    ]);
    let is_fix = u32::from_be_bytes([
        record_data[32],
        record_data[33],
        record_data[34],
        record_data[35],
    ]);
    let currency_offset = u32::from_be_bytes([
        record_data[36],
        record_data[37],
        record_data[38],
        record_data[39],
    ]);
    let date_from_offset = u32::from_be_bytes([
        record_data[40],
        record_data[41],
        record_data[42],
        record_data[43],
    ]);
    let date_to_offset = u32::from_be_bytes([
        record_data[44],
        record_data[45],
        record_data[46],
        record_data[47],
    ]);
    let scale_quantity = u32::from_be_bytes([
        record_data[48],
        record_data[49],
        record_data[50],
        record_data[51],
    ]);
    let rounding_id_offset = u32::from_be_bytes([
        record_data[52],
        record_data[53],
        record_data[54],
        record_data[55],
    ]);

    println!("\n=== Parsed Field Offsets ===");
    println!(
        "  article_nr (offset 4):    0x{:08x} ({})",
        article_nr_offset, article_nr_offset
    );
    println!(
        "  var_cond (offset 8):      0x{:08x} ({})",
        var_cond_offset, var_cond_offset
    );
    println!(
        "  price_type (offset 12):   0x{:08x} ({})",
        price_type_offset, price_type_offset
    );
    println!(
        "  price_level (offset 16):  0x{:08x} ({})",
        price_level_offset, price_level_offset
    );
    println!(
        "  price_rule (offset 20):   0x{:08x} ({})",
        price_rule_offset, price_rule_offset
    );
    println!(
        "  price_textnr (offset 24): 0x{:08x} ({})",
        price_textnr_offset, price_textnr_offset
    );
    println!(
        "  price (offset 28):        {} (raw bytes: {:02x} {:02x} {:02x} {:02x})",
        price_float, record_data[28], record_data[29], record_data[30], record_data[31]
    );
    println!("  is_fix (offset 32):       {}", is_fix);
    println!(
        "  currency (offset 36):     0x{:08x} ({})",
        currency_offset, currency_offset
    );
    println!(
        "  date_from (offset 40):    0x{:08x} ({})",
        date_from_offset, date_from_offset
    );
    println!(
        "  date_to (offset 44):      0x{:08x} ({})",
        date_to_offset, date_to_offset
    );
    println!("  scale_quantity (offset 48): {}", scale_quantity);
    println!(
        "  rounding_id (offset 52):  0x{:08x} ({})",
        rounding_id_offset, rounding_id_offset
    );

    // Read strings from string pool
    fn read_string_at(file: &mut File, offset: u32) -> String {
        if offset == 0 || offset == 1 {
            return String::from("(empty)");
        }

        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let mut len_bytes = [0u8; 2];
        if file.read_exact(&mut len_bytes).is_err() {
            return String::from("(read error)");
        }
        let str_len = u16::from_be_bytes(len_bytes) as usize;

        if str_len == 0 || str_len > 1000 {
            return format!("(invalid length: {})", str_len);
        }

        let mut data = vec![0u8; str_len];
        if file.read_exact(&mut data).is_err() {
            return String::from("(read error)");
        }

        String::from_utf8(data)
            .unwrap_or_else(|_| String::from("(utf8 error)"))
            .trim_end_matches('\0')
            .to_string()
    }

    println!("\n=== String Values from String Pool ===");
    println!(
        "  article_nr:    \"{}\"",
        read_string_at(&mut file, article_nr_offset)
    );
    println!(
        "  var_cond:      \"{}\"",
        read_string_at(&mut file, var_cond_offset)
    );
    println!(
        "  price_type:    \"{}\"",
        read_string_at(&mut file, price_type_offset)
    );
    println!(
        "  price_level:   \"{}\"",
        read_string_at(&mut file, price_level_offset)
    );
    println!(
        "  price_rule:    \"{}\"",
        read_string_at(&mut file, price_rule_offset)
    );
    println!(
        "  price_textnr:  \"{}\"",
        read_string_at(&mut file, price_textnr_offset)
    );
    println!(
        "  currency:      \"{}\"",
        read_string_at(&mut file, currency_offset)
    );
    println!(
        "  date_from:     \"{}\"",
        read_string_at(&mut file, date_from_offset)
    );
    println!(
        "  date_to:       \"{}\"",
        read_string_at(&mut file, date_to_offset)
    );
    println!(
        "  rounding_id:   \"{}\"",
        read_string_at(&mut file, rounding_id_offset)
    );

    // Now compare with record 0 (ONE_COMPACT_ESSENTIALS at 13800 EUR)
    println!("\n=== Comparison with Record 0 (ONE_COMPACT_ESSENTIALS) ===");
    let record0_offset = 0x34d8u64 - (13 * 56);
    file.seek(SeekFrom::Start(record0_offset)).unwrap();
    let mut record0_data = [0u8; 56];
    file.read_exact(&mut record0_data).unwrap();

    let r0_article_offset = u32::from_be_bytes([
        record0_data[4],
        record0_data[5],
        record0_data[6],
        record0_data[7],
    ]);
    let r0_price = f32::from_be_bytes([
        record0_data[28],
        record0_data[29],
        record0_data[30],
        record0_data[31],
    ]);

    println!("Record 0 offset: 0x{:08x}", record0_offset);
    println!(
        "Record 0 article_nr: \"{}\"",
        read_string_at(&mut file, r0_article_offset)
    );
    println!("Record 0 price: {} EUR", r0_price);

    println!("\n=== ANALYSIS ===");
    println!("The record appears to be MISSING the first 4 bytes (record ID field).");
    println!("All string pointers are shifted by 4 bytes, causing:");
    println!("  - article_nr (offset 4) reads from var_cond position");
    println!("  - var_cond (offset 8) reads from price_type position");
    println!("  - price_type (offset 12) reads from price_level position");
    println!("  - And so on...");
    println!("\nThe actual data SHOULD be:");
    println!("  article_nr:   \"ONE_COMPACT_BASE\" (from offset 0x91a2)");
    println!("  var_cond:     \"\" (empty)");
    println!("  price_type:   \"S\"");
    println!("  price_level:  \"B\"");
    println!("  price:        ??? (corrupted float value)");

    // Try to find what the price should be by looking at surrounding data
    println!("\n=== Looking for Missing Price Data ===");
    println!("Searching for price pattern around record 13...");
}
