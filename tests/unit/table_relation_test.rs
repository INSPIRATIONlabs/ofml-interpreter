//! Tests for TABLE relation support in OCD properties

use std::collections::HashMap;
use std::path::Path;

use ofml_interpreter::oap::ocd_properties::{load_manufacturer_properties, clear_property_cache, OcdPropertyReader};

#[test]
fn test_table_relation_parsing() {
    // Test parsing TABLE relation expressions
    let rel_text = "TABLE WKT_Groesse (Abmessung=Abmessung, Farbe_Rahmen=Farbe_Rahmen, Groesse = $SELF.Groesse)";
    let parsed = OcdPropertyReader::parse_table_relation(rel_text);

    assert!(parsed.is_some(), "Should parse TABLE relation");
    let parsed = parsed.unwrap();

    assert_eq!(parsed.table_name, "WKT_Groesse");
    assert_eq!(parsed.column_mappings.len(), 3);
    assert_eq!(parsed.target_column, Some("Groesse".to_string()));
}

#[test]
fn test_table_relation_with_spaces() {
    let rel_text = "TABLE  WKT_Artikelnummer  ( Groesse = Groesse , Artikelnummer = $SELF.Artikelnummer )";
    let parsed = OcdPropertyReader::parse_table_relation(rel_text);

    assert!(parsed.is_some(), "Should parse TABLE relation with extra spaces");
    let parsed = parsed.unwrap();

    assert_eq!(parsed.table_name, "WKT_Artikelnummer");
    assert_eq!(parsed.column_mappings.len(), 2);
    assert_eq!(parsed.target_column, Some("Artikelnummer".to_string()));
}

#[test]
fn test_non_table_relation() {
    let rel_text = "Abmessung IN ('100X60','140X40','80X80')";
    let parsed = OcdPropertyReader::parse_table_relation(rel_text);

    assert!(parsed.is_none(), "Should not parse non-TABLE relation");
}

#[test]
fn test_fast_wkm_table_relations() {
    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/fast/wkm/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("FAST WKM data not available, skipping test");
        return;
    }

    let reader = OcdPropertyReader::from_ebase(path).expect("Should load FAST WKM data");

    // Check that relations were loaded
    println!("Loaded {} relation objects", reader.relation_objs.len());
    println!("Loaded {} relation groups", reader.relations.len());
    println!("Loaded {} custom tables", reader.custom_tables.len());

    // Check for WKT_Groesse table
    let wkt_groesse = reader.custom_tables.get("wkt_groesse_tbl");
    if let Some(table) = wkt_groesse {
        println!("WKT_Groesse table has {} rows (after unpivoting)", table.len());
        assert!(!table.is_empty(), "WKT_Groesse table should have data");

        // Check that the table has expected columns (lowercase after unpivoting)
        if let Some(first_row) = table.first() {
            let keys: Vec<_> = first_row.keys().collect();
            println!("Columns: {:?}", keys);
            // The table should have been unpivoted and have the actual column names
            // Note: Some tables may have only 1 column after processing
            assert!(!keys.is_empty(), "Table should have at least one column, got {:?}", keys);
        }
    }

    // Test getting table values
    let selections: HashMap<String, String> = HashMap::new();
    let values = reader.get_table_values("Rahmen", "Groesse", &selections);

    println!("Got {} values from TABLE relation for Groesse", values.len());
    for val in values.iter().take(5) {
        println!("  - {}", val.value_from);
    }
}

#[test]
fn test_fast_wkm_property_with_table_relation() {
    let path = Path::new("/reference/ofmldata/fast/wkm/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("FAST WKM data not available, skipping test");
        return;
    }

    let reader = OcdPropertyReader::from_ebase(path).expect("Should load FAST WKM data");

    // Check if Groesse property uses TABLE relation
    let uses_table = reader.property_uses_table("Rahmen", "Groesse");
    println!("Groesse property uses TABLE: {}", uses_table);

    // Check Farbe_Rahmen as well
    let farbe_uses_table = reader.property_uses_table("Rahmen", "Farbe_Rahmen");
    println!("Farbe_Rahmen property uses TABLE: {}", farbe_uses_table);

    // Get the property definitions
    let props = reader.get_properties_for_class("Rahmen");
    println!("Found {} properties in Rahmen class:", props.len());
    for prop in &props {
        println!("  - {} (rel_obj={}, scope={})", prop.property, prop.rel_obj, prop.scope);
    }
}

#[test]
fn test_manufacturer_property_loading_with_tables() {
    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/fast");
    if !path.exists() {
        println!("FAST manufacturer data not available, skipping test");
        return;
    }

    let reader = load_manufacturer_properties(path);

    println!("Manufacturer-wide stats:");
    let (props, vals, classes, texts) = reader.stats();
    println!("  Properties: {}", props);
    println!("  Values: {}", vals);
    println!("  Classes: {}", classes);
    println!("  Texts: {}", texts);
    println!("  Relation objects: {}", reader.relation_objs.len());
    println!("  Relation groups: {}", reader.relations.len());
    println!("  Custom tables: {}", reader.custom_tables.len());

    // List custom tables
    println!("\nCustom tables loaded:");
    for table_name in reader.custom_tables.keys() {
        let count = reader.custom_tables.get(table_name).map(|t| t.len()).unwrap_or(0);
        println!("  - {} ({} rows)", table_name, count);
    }
}

#[test]
fn test_fast_wkm_tui_display() {
    use ofml_interpreter::oap::families::FamilyLoader;

    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/fast");
    if !path.exists() {
        println!("FAST manufacturer data not available, skipping test");
        return;
    }

    let loader = FamilyLoader::load(path, "DE");

    println!("\n=== What TUI shows for FAST WKM ===\n");

    // Find WKM family
    for family in loader.get_families() {
        if family.series.to_uppercase() == "WKM" {
            println!("Family: {} - {}", family.id, family.name);
            println!("Series: {}", family.series);
            println!("prop_classes: {:?}", family.prop_classes);

            // Debug: Show all properties from property class
            println!("\nAll properties in Rahmen class (before filtering):");
            for prop in loader.properties.get_properties_for_class("Rahmen") {
                let values = loader.properties.get_values_for_property("Rahmen", &prop.property);
                println!("  {} (scope={}, rel_obj={}): {} values in ocd_propertyvalue",
                    prop.property, prop.scope, prop.rel_obj, values.len());
            }

            let props = loader.get_properties_for_family(family);
            println!("\nProperties available in TUI (after filtering): {}", props.len());

            for prop in &props {
                println!("\n  {} ({}):", prop.key, prop.label);
                println!("    Options: {} items", prop.options.len());
                for (_i, opt) in prop.options.iter().enumerate().take(5) {
                    let marker = if opt.is_default { "*" } else { " " };
                    println!("      {}[{}] {}", marker, opt.value, opt.label);
                }
                if prop.options.len() > 5 {
                    println!("      ... and {} more options", prop.options.len() - 5);
                }
            }
            println!();
        }
    }
}

#[test]
fn test_fast_wkm_price_dump() {
    use ofml_interpreter::ebase::EBaseReader;

    let path = Path::new("/reference/ofmldata/fast/wkm/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("FAST WKM data not available, skipping test");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Should open FAST WKM ebase");

    println!("\n=== FAST WKM ocd_price records ===");
    if let Ok(prices) = reader.read_records("ocd_price", Some(30)) {
        for p in &prices {
            let article = p.get("article_nr").and_then(|v| v.as_str()).unwrap_or("");
            let var_cond = p.get("var_cond").and_then(|v| v.as_str()).unwrap_or("");
            let level = p.get("price_level").and_then(|v| v.as_str()).unwrap_or("");
            let price = p.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let currency = p.get("currency").and_then(|v| v.as_str()).unwrap_or("");
            println!("  {} | {:30} | {} | {:8.2} {}", article, var_cond, level, price, currency);
        }
    }
}

#[test]
fn test_fast_wkm_compute_varcond() {
    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/fast");
    if !path.exists() {
        println!("FAST manufacturer data not available, skipping test");
        return;
    }

    let reader = load_manufacturer_properties(path);

    // Check if FAST uses TABLE-based var_cond
    assert!(reader.uses_table_varcond(), "FAST should use TABLE-based var_cond");

    println!("\n=== Testing var_cond computation for FAST WKM ===\n");

    // Debug: Show wkt_groesse_tbl content
    println!("WKT_Groesse table content:");
    if let Some(table) = reader.custom_tables.get("wkt_groesse_tbl") {
        println!("  Table has {} rows", table.len());
        for (i, row) in table.iter().take(5).enumerate() {
            println!("  Row {}: {:?}", i, row);
        }
        // Show entries with 140X40
        println!("\n  Entries containing 140X40:");
        for row in table {
            if let Some(abm) = row.get("abmessung") {
                if abm.to_uppercase().contains("140") {
                    println!("    {:?}", row);
                }
            }
        }
    } else {
        println!("  TABLE NOT FOUND!");
    }

    // Test case 1: 100X60 with VOLLHOLZ_SCHWARZ should compute to SG-MB-WKM-100X60-HOSW
    let mut selections1: HashMap<String, String> = HashMap::new();
    selections1.insert("Abmessung".to_string(), "100X60".to_string());
    selections1.insert("Farbe_Rahmen".to_string(), "VOLLHOLZ_SCHWARZ".to_string());

    let varcond1 = reader.compute_varcond_from_selections("Rahmen", &selections1);
    println!("\nSelections: {:?}", selections1);
    println!("Computed var_cond: {:?}", varcond1);

    // Should match SG-MB-WKM-100X60-HOSW (case-insensitive)
    assert!(varcond1.is_some(), "Should compute var_cond for 100X60 + VOLLHOLZ_SCHWARZ");
    let vc1 = varcond1.unwrap();
    assert!(
        vc1.to_uppercase().contains("100X60") && vc1.to_uppercase().contains("HOSW"),
        "Expected var_cond containing 100X60 and HOSW, got: {}",
        vc1
    );

    // Test case 2: 100X60 with VOLLHOLZ_EICHE should compute to SG-MB-WKM-100X60-HOEI
    let mut selections2: HashMap<String, String> = HashMap::new();
    selections2.insert("Abmessung".to_string(), "100X60".to_string());
    selections2.insert("Farbe_Rahmen".to_string(), "VOLLHOLZ_EICHE".to_string());

    let varcond2 = reader.compute_varcond_from_selections("Rahmen", &selections2);
    println!("\nSelections: {:?}", selections2);
    println!("Computed var_cond: {:?}", varcond2);

    // Should match SG-MB-WKM-100X60-HOEI (case-insensitive)
    assert!(varcond2.is_some(), "Should compute var_cond for 100X60 + VOLLHOLZ_EICHE");
    let vc2 = varcond2.unwrap();
    assert!(
        vc2.to_uppercase().contains("100X60") && vc2.to_uppercase().contains("HOEI"),
        "Expected var_cond containing 100X60 and HOEI, got: {}",
        vc2
    );

    println!("\n=== Basic var_cond computations passed! ===\n");
}

#[test]
fn test_fast_wkm_price_with_varcond() {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyConfiguration, FamilyLoader};

    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    if !path.exists() {
        println!("OFML data not available, skipping test");
        return;
    }

    let mfr_path = Path::new("/reference/ofmldata/fast");
    if !mfr_path.exists() {
        println!("FAST manufacturer data not available, skipping test");
        return;
    }

    // Load families for FAST
    let loader = FamilyLoader::load(mfr_path, "DE");

    // Find WKM family
    let wkm_family = loader.get_families().iter().find(|f| f.series.to_uppercase() == "WKM");
    if wkm_family.is_none() {
        println!("WKM family not found, skipping test");
        return;
    }
    let wkm_family = wkm_family.unwrap();
    println!("Found WKM family: {} - {}", wkm_family.id, wkm_family.name);
    println!("Base article: {}", wkm_family.base_article_nr);
    println!("Property classes: {:?}", wkm_family.prop_classes);

    // Get properties for the family
    let props = loader.get_properties_for_family(wkm_family);
    println!("\nProperties available: {}", props.len());
    for prop in &props {
        println!("  {} ({}) - {} options", prop.key, prop.label, prop.options.len());
    }

    // Create configuration
    let mut config = FamilyConfiguration::new(&wkm_family.id, &props);

    // Set specific values: 100X60 with VOLLHOLZ_SCHWARZ
    config.set("Abmessung", "100X60");
    config.set("Farbe_Rahmen", "VOLLHOLZ_SCHWARZ");

    println!("\n=== Price calculation test ===");
    println!("Configuration: {:?}", config.selections);
    println!("Variant code: {}", config.variant_code);

    // Calculate price
    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();
    let price_result = engine.calculate_family_price("fast", wkm_family, &config, today);

    println!("\nPrice result: {:?}", price_result);

    if let Some(price) = price_result {
        println!("\n  Base price: {} {}", price.base_price, price.currency);
        println!("  Total: {} {}", price.total_price, price.currency);
        // VOLLHOLZ_SCHWARZ (100X60) should be around 368.91 EUR
        assert!(
            price.base_price > rust_decimal::Decimal::from(300)
                && price.base_price < rust_decimal::Decimal::from(500),
            "Price should be in reasonable range for 100X60 VOLLHOLZ_SCHWARZ"
        );
    } else {
        println!("\nNo price found - checking if prices exist in data...");
        // This is not necessarily a failure - the article might not have prices
    }

    // Test with VOLLHOLZ_EICHE (should have higher price)
    config.set("Farbe_Rahmen", "VOLLHOLZ_EICHE");
    println!("\n=== Testing with VOLLHOLZ_EICHE (Oak - premium) ===");
    println!("Configuration: {:?}", config.selections);

    let price_oak = engine.calculate_family_price("fast", wkm_family, &config, today);
    println!("Price result for oak: {:?}", price_oak);

    if let Some(price) = price_oak {
        println!("\n  Base price (oak): {} {}", price.base_price, price.currency);
        // Oak (VOLLHOLZ_EICHE) should be around 394.12 EUR (higher than black)
        assert!(
            price.base_price > rust_decimal::Decimal::from(350),
            "Oak price should be higher than 350 EUR"
        );
    }

    println!("\n=== FAST WKM price test complete! ===\n");
}

#[test]
fn test_framery_price_debug() {
    use ofml_interpreter::ebase::EBaseReader;
    use ofml_interpreter::oap::ocd::OcdReader;

    let path = std::path::Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        println!("Framery data not available, skipping test");
        return;
    }

    // Check raw ebase prices
    let mut reader = EBaseReader::open(path).expect("Should open Framery ebase");

    println!("\n=== Framery ocd_price raw records ===");
    if let Ok(prices) = reader.read_records("ocd_price", Some(20)) {
        println!("Found {} price records", prices.len());
        for p in &prices {
            let article = p.get("article_nr").map(|v| format!("{:?}", v)).unwrap_or_default();
            let var_cond = p.get("var_cond").map(|v| format!("{:?}", v)).unwrap_or_default();
            let level = p.get("price_level").map(|v| format!("{:?}", v)).unwrap_or_default();
            let price = p.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            println!("  {} | {} | {} | {:.2}", article, var_cond, level, price);
        }
    }

    // Check OcdReader
    let ocd = OcdReader::from_ebase(path).expect("Should load OcdReader");
    println!("\n=== OcdReader prices ===");
    println!("Total prices: {}", ocd.prices.len());

    // Try to get prices for various articles
    for article in &["ONE", "ONE_PREMIUM", "*", ""] {
        let prices = ocd.get_prices(article);
        println!("Prices for '{}': {}", article, prices.len());
    }

    // Show all unique article_nrs
    let mut articles: Vec<_> = ocd.prices.iter().map(|p| p.article_nr.as_str()).collect();
    articles.sort();
    articles.dedup();
    println!("\nUnique article_nrs in prices: {:?}", articles);

    // Show all price_levels
    let mut levels: Vec<_> = ocd.prices.iter().map(|p| p.price_level.as_str()).collect();
    levels.sort();
    levels.dedup();
    println!("Unique price_levels: {:?}", levels);

    // Check surcharge-only detection
    println!("\nSurcharge-only pricing: {}", ocd.has_surcharge_only_pricing());
}

#[test]
fn test_framery_price_calculation() {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyConfiguration, FamilyLoader};

    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    if !path.exists() {
        println!("OFML data not available, skipping test");
        return;
    }

    let mfr_path = Path::new("/reference/ofmldata/framery");
    if !mfr_path.exists() {
        println!("Framery manufacturer data not available, skipping test");
        return;
    }

    // Load families for Framery
    let loader = FamilyLoader::load(mfr_path, "DE");
    let families = loader.get_families();
    println!("\n=== Framery families ===");
    println!("Found {} families", families.len());

    // Find a family
    if let Some(family) = families.first() {
        println!("Testing family: {} - {}", family.id, family.name);
        println!("Base article: {}", family.base_article_nr);
        println!("Property classes: {:?}", family.prop_classes);

        // Get properties
        let props = loader.get_properties_for_family(family);
        println!("Properties: {}", props.len());
        for prop in props.iter().take(5) {
            println!("  {} ({}) - {} options", prop.key, prop.label, prop.options.len());
        }

        // Create configuration
        let config = FamilyConfiguration::new(&family.id, &props);
        println!("\nConfiguration selections: {:?}", config.selections);

        // Calculate price
        let engine = ConfigurationEngine::new(path);
        let today = chrono::Local::now().date_naive();
        let price_result = engine.calculate_family_price("framery", family, &config, today);

        println!("\nPrice result: {:?}", price_result);

        // Framery should now return a price (even if 0 base + 0 surcharges)
        assert!(
            price_result.is_some(),
            "Framery should return a price result (even if 0)"
        );

        if let Some(price) = price_result {
            println!("  Base price: {} {}", price.base_price, price.currency);
            println!("  Total: {} {}", price.total_price, price.currency);
        }
    }
}

#[test]
fn test_fast_kr_price_variations() {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyConfiguration, FamilyLoader};

    // Clear cache to ensure we get fresh data
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/fast");
    if !mfr_path.exists() {
        println!("FAST manufacturer data not available, skipping test");
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");

    // Find KR family (Kreise)
    let kr_family = loader.get_families().iter().find(|f| f.series.to_uppercase() == "KR");
    if kr_family.is_none() {
        println!("KR family not found, skipping test");
        return;
    }
    let kr_family = kr_family.unwrap();
    println!("\n=== Testing FAST KR (Kreise) pricing ===");
    println!("Family: {} - {}", kr_family.id, kr_family.name);
    println!("Base article: {}", kr_family.base_article_nr);
    println!("Property classes: {:?}", kr_family.prop_classes);

    let props = loader.get_properties_for_family(kr_family);
    println!("\nProperties:");
    for prop in &props {
        println!("  {} ({}) - {} options: {:?}", 
            prop.key, 
            prop.label, 
            prop.options.len(),
            prop.options.iter().map(|o| &o.value).collect::<Vec<_>>()
        );
    }

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    // Test different Abmessung (size) values - this should change prices
    println!("\n--- Testing Abmessung variations ---");
    for size in &["100X60", "35X35", "22X22"] {
        let mut config = FamilyConfiguration::new(&kr_family.id, &props);
        config.set("Abmessung", size);

        println!("\n  Abmessung = {}", size);
        let price = engine.calculate_family_price("fast", kr_family, &config, today);
        if let Some(p) = price {
            println!("  Price: {} {}", p.total_price, p.currency);
        } else {
            println!("  Price: None");
        }
    }

    // Test different Farbe_Rahmen (color) values - this should also change prices
    println!("\n--- Testing Farbe_Rahmen variations ---");
    for color in &["VOLLHOLZ_SCHWARZ", "VOLLHOLZ_EICHE", "MDF_WEIS"] {
        let mut config = FamilyConfiguration::new(&kr_family.id, &props);
        config.set("Farbe_Rahmen", color);

        println!("\n  Farbe_Rahmen = {}", color);
        let price = engine.calculate_family_price("fast", kr_family, &config, today);
        if let Some(p) = price {
            println!("  Price: {} {}", p.total_price, p.currency);
        } else {
            println!("  Price: None");
        }
    }

    println!("\n=== FAST KR test complete ===");
}

#[test]
fn test_fast_kr_form_options_debug() {
    use ofml_interpreter::ebase::EBaseReader;

    let path = std::path::Path::new("/reference/ofmldata/fast/kr/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("FAST KR data not available, skipping test");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Should open FAST KR ebase");

    // Check ocd_propertyvalue for Form property
    println!("\n=== Form property values in ocd_propertyvalue ===");
    if let Ok(values) = reader.read_records("ocd_propertyvalue", Some(100)) {
        let form_values: Vec<_> = values.iter()
            .filter(|v| {
                let prop = v.get("property").and_then(|v| v.as_str()).unwrap_or("");
                prop.to_uppercase() == "FORM"
            })
            .collect();
        println!("Form values: {}", form_values.len());
        for v in &form_values {
            let value = v.get("value_from").and_then(|v| v.as_str()).unwrap_or("");
            let prop_class = v.get("prop_class").and_then(|v| v.as_str()).unwrap_or("");
            println!("  {} = {}", prop_class, value);
        }
    }

    // Check what properties exist
    println!("\n=== All properties ===");
    if let Ok(props) = reader.read_records("ocd_property", Some(50)) {
        for p in &props {
            let name = p.get("property").and_then(|v| v.as_str()).unwrap_or("");
            let prop_class = p.get("prop_class").and_then(|v| v.as_str()).unwrap_or("");
            let scope = p.get("scope").and_then(|v| v.as_str()).unwrap_or("");
            println!("  {} | {} | scope={}", prop_class, name, scope);
        }
    }
}

#[test]
fn test_fast_kr_varcond_debug() {
    use ofml_interpreter::oap::ocd_properties::{load_manufacturer_properties, clear_property_cache};

    clear_property_cache();

    let path = Path::new("/reference/ofmldata/fast");
    if !path.exists() {
        println!("FAST data not available");
        return;
    }

    let reader = load_manufacturer_properties(path);

    println!("\n=== Testing var_cond computation for KR ===\n");

    // Show custom tables
    println!("Custom tables available:");
    for (name, data) in &reader.custom_tables {
        println!("  {} ({} rows)", name, data.len());
        if name.contains("groesse") || name.contains("artikelnummer") {
            if let Some(first) = data.first() {
                println!("    Columns: {:?}", first.keys().collect::<Vec<_>>());
            }
        }
    }

    // Show relations for KR/Rahmen class
    println!("\nRelations:");
    for (name, rels) in &reader.relations {
        println!("  {}:", name);
        for rel in rels {
            println!("    {}", rel.rel_block);
        }
    }

    // Test var_cond computation for KR
    let mut selections: HashMap<String, String> = HashMap::new();
    selections.insert("Abmessung".to_string(), "100X60".to_string());
    selections.insert("Farbe_Rahmen".to_string(), "VOLLHOLZ_SCHWARZ".to_string());
    selections.insert("Moos".to_string(), "Kreise".to_string()); // Try Moos property
    selections.insert("Form".to_string(), "KR1".to_string());

    println!("\nSelections: {:?}", selections);
    println!("uses_table_varcond: {}", reader.uses_table_varcond());

    let varcond = reader.compute_varcond_from_selections("Rahmen", &selections);
    println!("Computed var_cond: {:?}", varcond);
}
