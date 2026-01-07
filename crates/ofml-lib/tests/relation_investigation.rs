//! Investigation test for ocd_relation rules

use std::path::Path;

use ofml_lib::ebase::{EBaseReader, Value};

/// Convert a Value to a displayable string
fn value_to_string(v: &Value) -> String {
    match v {
        Value::Int(i) => i.to_string(),
        Value::UInt(u) => u.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Blob(b) => format!("<blob:{}>", b),
        Value::Null => String::new(),
    }
}

fn get_str(record: &std::collections::HashMap<String, Value>, key: &str) -> String {
    record.get(key).map(value_to_string).unwrap_or_default()
}

#[test]
fn test_dump_framery_relations() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== ocd_relation records ===\n");
    match reader.read_records("ocd_relation", None) {
        Ok(records) => {
            for (i, record) in records.iter().enumerate() {
                let rel_name = get_str(record, "rel_name");
                let rel_blocknr = get_str(record, "rel_blocknr");
                let rel_block = get_str(record, "rel_block");
                println!("  [{}] name='{}' blocknr={}", i, rel_name, rel_blocknr);
                println!("      block: {}", rel_block);
            }
            println!("  Total: {} relation rules", records.len());
        }
        Err(e) => println!("  Table 'ocd_relation' error: {}", e),
    }

    println!("\n=== ocd_relationobj records ===\n");
    match reader.read_records("ocd_relationobj", None) {
        Ok(records) => {
            for (i, record) in records.iter().enumerate() {
                let rel_obj = get_str(record, "rel_obj");
                let position = get_str(record, "position");
                let rel_name = get_str(record, "rel_name");
                let rel_type = get_str(record, "rel_type");
                let rel_domain = get_str(record, "rel_domain");
                println!("  [{}] obj={} pos={} name='{}' type='{}' domain='{}'",
                    i, rel_obj, position, rel_name, rel_type, rel_domain);
            }
        }
        Err(e) => println!("  Table 'ocd_relationobj' error: {}", e),
    }

    // Check if pricing domain relations exist
    println!("\n=== Pricing domain analysis ===\n");
    match reader.read_records("ocd_relationobj", None) {
        Ok(records) => {
            let mut pricing_count = 0;
            for record in &records {
                let rel_domain = get_str(record, "rel_domain");
                if rel_domain.contains("P") {
                    pricing_count += 1;
                    let rel_name = get_str(record, "rel_name");
                    println!("  PRICING relation: {}", rel_name);
                }
            }
            println!("\n  Total pricing relations: {}", pricing_count);
        }
        Err(_) => {}
    }
}

#[test]
fn test_dump_framery_2q_relations() {
    // Compare with frmr_2q which has working prices
    let path = Path::new("/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== frmr_2q ocd_relation records ===\n");
    match reader.read_records("ocd_relation", Some(10)) {
        Ok(records) => {
            for record in &records {
                let rel_name = get_str(record, "rel_name");
                let rel_block = get_str(record, "rel_block");
                println!("  name='{}' block: {}", rel_name, rel_block);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
}

#[test]
fn test_check_propertyvalue_relobjid() {
    // Check if propertyvalues have rel_obj IDs that link to pricing rules
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== ocd_propertyvalue with rel_obj ===\n");
    match reader.read_records("ocd_propertyvalue", Some(100)) {
        Ok(records) => {
            let mut with_rel_obj = 0;
            for record in &records {
                let rel_obj = get_str(record, "rel_obj");
                let prop_value = get_str(record, "prop_value");
                let property = get_str(record, "property");

                if !rel_obj.is_empty() && rel_obj != "0" {
                    with_rel_obj += 1;
                    println!("  {} = {} -> rel_obj={}", property, prop_value, rel_obj);
                }
            }
            println!("\n  Values with rel_obj: {}", with_rel_obj);
        }
        Err(e) => println!("  Error: {}", e),
    }
}

#[test]
fn test_analyze_framery_pricing_rules() {
    // Try to understand how Framery determines surcharges
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== Framery Pricing Analysis ===\n");

    // Get all surcharges
    println!("Surcharges (level='X'):");
    match reader.read_records("ocd_price", None) {
        Ok(records) => {
            for record in &records {
                let level = get_str(record, "price_level");
                if level == "X" {
                    let var_cond = get_str(record, "var_cond");
                    let price = get_str(record, "price");
                    println!("  {} = {} EUR", var_cond, price);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check ocd_pricetext for var_cond descriptions
    println!("\nPrice text descriptions:");
    match reader.read_records("ocd_pricetext", Some(30)) {
        Ok(records) => {
            for record in &records {
                let textnr = get_str(record, "textnr");
                let text = get_str(record, "text");
                let language = get_str(record, "language");
                if language.to_uppercase().contains("EN") || language.to_uppercase().contains("DE") {
                    println!("  [{}] {} = {}", language, textnr, text);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
}

#[test]
fn test_analyze_frmr_one_property_classes() {
    // Check what property classes map to FRMR_ONE articles
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== FRMR_ONE Property Classes ===\n");

    // Check ocd_article first
    println!("Articles:");
    match reader.read_records("ocd_article", None) {
        Ok(records) => {
            for record in &records {
                let art_nr = get_str(record, "article_nr");
                let prop_class = get_str(record, "prop_class");
                println!("  {} -> prop_class={}", art_nr, prop_class);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check property class definitions
    println!("\nProperty class definitions:");
    match reader.read_records("ocd_propertyclass", None) {
        Ok(records) => {
            for record in &records {
                let prop_class = get_str(record, "prop_class");
                let property = get_str(record, "property");
                let position = get_str(record, "position");
                println!("  {} pos={} property={}", prop_class, position, property);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check price entries
    println!("\nPrice entries:");
    match reader.read_records("ocd_price", None) {
        Ok(records) => {
            for record in &records {
                let art_nr = get_str(record, "article_nr");
                let level = get_str(record, "price_level");
                let var_cond = get_str(record, "var_cond");
                let price = get_str(record, "price");
                println!("  art={} level={} var_cond={} price={}", art_nr, level, var_cond, price);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
}

#[test]
fn test_frmr_one_varcond_structure() {
    // Deep dive: understand how Framery assigns var_cond for pricing
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== FRMR_ONE Variant Condition Analysis ===\n");

    // Check if property values have var_cond assignments
    println!("Property values with var_cond:");
    match reader.read_records("ocd_propertyvalue", None) {
        Ok(records) => {
            for record in &records {
                let property = get_str(record, "property");
                let prop_value = get_str(record, "prop_value");
                let var_cond = get_str(record, "var_cond");

                if !var_cond.is_empty() {
                    println!("  {}.{} -> var_cond={}", property, prop_value, var_cond);
                }
            }
            println!("  Total property values: {}", records.len());
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check propvalue2varcond table
    println!("\npropvalue2varcond table:");
    match reader.read_records("ocd_propvalue2varcond", None) {
        Ok(records) => {
            for record in &records {
                let prop_class = get_str(record, "prop_class");
                let property = get_str(record, "property");
                let prop_value = get_str(record, "prop_value");
                let var_cond = get_str(record, "var_cond");
                println!("  {}.{}.{} -> {}", prop_class, property, prop_value, var_cond);
            }
            println!("  Total mappings: {}", records.len());
        }
        Err(e) => println!("  propvalue2varcond: {}", e),
    }

    // Check varcond2amount table
    println!("\nvarcond2amount table:");
    match reader.read_records("ocd_varcond2amount", None) {
        Ok(records) => {
            for record in &records {
                let var_cond = get_str(record, "var_cond");
                let amount = get_str(record, "amount");
                let table_name = get_str(record, "table_name");
                println!("  {} -> amount={} table={}", var_cond, amount, table_name);
            }
            println!("  Total varcond2amount entries: {}", records.len());
        }
        Err(e) => println!("  varcond2amount: {}", e),
    }
}

#[test]
fn test_list_all_tables() {
    // List all tables in the FRMR_ONE database to understand its structure
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== Tables in FRMR_ONE pdata.ebase ===\n");
    let tables = reader.table_names();
    for name in &tables {
        if let Some(table) = reader.tables.get(*name) {
            println!("  {} ({} records, {} columns)",
                name, table.record_count, table.columns.len());
        }
    }
    println!("\n  Total: {} tables", tables.len());
}

#[test]
fn test_compare_frmr_2q_structure() {
    // Compare frmr_2q structure (has base prices) with frmr_one
    let path = Path::new("/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

    println!("\n=== FRMR_2Q Structure Analysis ===\n");

    // Check articles
    println!("Articles:");
    match reader.read_records("ocd_article", None) {
        Ok(records) => {
            for record in &records {
                let art_nr = get_str(record, "article_nr");
                let prop_class = get_str(record, "prop_class");
                println!("  {} -> prop_class={}", art_nr, prop_class);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check base prices
    println!("\nBase prices (level='B'):");
    match reader.read_records("ocd_price", None) {
        Ok(records) => {
            for record in &records {
                let level = get_str(record, "price_level");
                if level == "B" {
                    let art_nr = get_str(record, "article_nr");
                    let var_cond = get_str(record, "var_cond");
                    let price = get_str(record, "price");
                    println!("  art={} var_cond={} price={}", art_nr, var_cond, price);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check surcharges
    println!("\nSurcharges (level='X'):");
    match reader.read_records("ocd_price", None) {
        Ok(records) => {
            for record in &records {
                let level = get_str(record, "price_level");
                if level == "X" {
                    let art_nr = get_str(record, "article_nr");
                    let var_cond = get_str(record, "var_cond");
                    let price = get_str(record, "price");
                    println!("  art={} var_cond={} price={}", art_nr, var_cond, price);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Check ocd_relationobj for pricing domain
    println!("\nRelation objects:");
    match reader.read_records("ocd_relationobj", None) {
        Ok(records) => {
            for record in &records {
                let rel_name = get_str(record, "rel_name");
                let rel_domain = get_str(record, "rel_domain");
                let rel_type = get_str(record, "rel_type");
                println!("  name={} domain={} type={}", rel_name, rel_domain, rel_type);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
}

#[test]
fn test_frmr_one_with_frmr_o_base_prices() {
    // Check if FRMR_ONE base prices might come from frmr_o series
    let paths = [
        "/reference/ofmldata/framery/frmr_o/ANY/1/db/pdata.ebase",
        "/reference/ofmldata/framery/frmr_o/DE/1/db/pdata.ebase",
    ];

    for path_str in &paths {
        let path = Path::new(path_str);
        if !path.exists() {
            continue;
        }

        let mut reader = EBaseReader::open(path).expect("Failed to open ebase");

        println!("\n=== {} ===\n", path_str);

        // Check if it has ONE article prices
        println!("Articles containing 'ONE':");
        match reader.read_records("ocd_article", None) {
            Ok(records) => {
                for record in &records {
                    let art_nr = get_str(record, "article_nr");
                    if art_nr.contains("ONE") {
                        let prop_class = get_str(record, "prop_class");
                        println!("  {} -> prop_class={}", art_nr, prop_class);
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }

        // Check base prices for ONE
        println!("\nBase prices (level='B') for ONE:");
        match reader.read_records("ocd_price", None) {
            Ok(records) => {
                for record in &records {
                    let level = get_str(record, "price_level");
                    let art_nr = get_str(record, "article_nr");
                    if level == "B" && (art_nr.contains("ONE") || art_nr == "*") {
                        let var_cond = get_str(record, "var_cond");
                        let price = get_str(record, "price");
                        println!("  art={} var_cond={} price={}", art_nr, var_cond, price);
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }
    }
}
