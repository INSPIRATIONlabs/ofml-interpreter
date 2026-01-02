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
