//! Debug test to verify header offset parsing

use ofml_lib::ebase::EBaseReader;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[test]
fn debug_header_offsets() {
    let path = Path::new("/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Skipping: Vitra classic not found");
        return;
    }

    println!("\n=== Header Offset Debug ===\n");

    // Read raw header
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut full_header = [0u8; 56];
    file.read_exact(&mut full_header).unwrap();

    println!("Raw header bytes:");
    for (i, chunk) in full_header.chunks(8).enumerate() {
        print!("  {:02X}: ", i * 8);
        for b in chunk {
            print!("{:02X} ", b);
        }
        // Also show as ASCII
        print!(" |");
        for b in chunk {
            if *b >= 32 && *b < 127 {
                print!("{}", *b as char);
            } else {
                print!(".");
            }
        }
        println!("|");
    }

    // What our parser would read (after skipping 6-byte magic)
    println!("\nParser's header array (starting at file offset 6):");
    let parser_header = &full_header[6..];
    println!("  header[0..4]  (file 6-9):   {:02X?} - could be version", &parser_header[0..4]);
    println!("  header[2..4]  (file 8-9):   {:02X?} - parser reads as major", &parser_header[2..4]);
    println!("  header[4..6]  (file 10-11): {:02X?} - parser reads as minor", &parser_header[4..6]);
    println!("  header[14..18] (file 20-23): {:02X?} - parser reads as string_pool_offset", &parser_header[14..18]);
    println!("  header[34..38] (file 40-43): {:02X?} - parser reads as string_data_size", &parser_header[34..38]);
    println!("  header[38..42] (file 44-47): {:02X?} - parser reads as num_tables", &parser_header[38..42]);

    // What the correct values should be
    println!("\nCorrect header interpretation (from file offsets 16-19 for string pool):");
    println!("  File 16-19: {:02X?} -> string_pool_offset = 0x{:08X}",
        &full_header[16..20],
        u32::from_be_bytes([full_header[16], full_header[17], full_header[18], full_header[19]]));
    println!("  File 20-23: {:02X?} -> string_pool_size = {}",
        &full_header[20..24],
        u32::from_be_bytes([full_header[20], full_header[21], full_header[22], full_header[23]]));

    // What our EBaseReader actually loaded
    let reader = EBaseReader::open(path).unwrap();
    println!("\nEBaseReader parsed values:");
    println!("  version: {}.{}", reader.major_version, reader.minor_version);
    println!("  tables: {:?}", reader.tables.keys().collect::<Vec<_>>());

    // Compare with Sedus to see if it has similar structure
    println!("\n=== Sedus Comparison ===\n");
    let sedus_path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if sedus_path.exists() {
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut file2 = BufReader::new(File::open(sedus_path).unwrap());
        let mut sedus_header = [0u8; 56];
        file2.read_exact(&mut sedus_header).unwrap();

        println!("Sedus raw header:");
        for (i, chunk) in sedus_header.chunks(8).enumerate() {
            print!("  {:02X}: ", i * 8);
            for b in chunk {
                print!("{:02X} ", b);
            }
            println!();
        }

        println!("\nSedus correct values (file offsets 16-19 for string pool):");
        println!("  File 16-19: {:02X?} -> string_pool_offset = 0x{:08X}",
            &sedus_header[16..20],
            u32::from_be_bytes([sedus_header[16], sedus_header[17], sedus_header[18], sedus_header[19]]));
        println!("  File 20-23: {:02X?} -> string_pool_size = {}",
            &sedus_header[20..24],
            u32::from_be_bytes([sedus_header[20], sedus_header[21], sedus_header[22], sedus_header[23]]));

        let sedus_reader = EBaseReader::open(sedus_path).unwrap();
        println!("\nSedus EBaseReader parsed values:");
        println!("  version: {}.{}", sedus_reader.major_version, sedus_reader.minor_version);
        println!("  tables: {:?}", sedus_reader.tables.keys().collect::<Vec<_>>());
    }
}
