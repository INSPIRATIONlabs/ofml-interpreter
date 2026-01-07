//! Investigation into Framery Four pricing issues

use std::path::Path;

use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::{FamilyConfiguration, FamilyLoader};
use ofml_lib::oap::ocd_properties::clear_property_cache;
use ofml_lib::oap::ocd_relation::RelationRuleReader;

#[test]
fn investigate_framery_four_pricing() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        eprintln!("Framery not available");
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");

    // Find FRMR_FOUR family
    let family = loader
        .get_families()
        .iter()
        .find(|f| f.series.to_uppercase() == "FRMR_FOUR")
        .cloned();

    if family.is_none() {
        eprintln!("FRMR_FOUR not found!");
        return;
    }
    let family = family.unwrap();

    println!("\n=== FRMR_FOUR Investigation ===");
    println!("Family: {} - {}", family.id, family.name);
    println!("Base article: {}", family.base_article_nr);

    // Check for relation rules
    let pdata_path = Path::new("/reference/ofmldata/framery/frmr_four/ANY/1/db/pdata.ebase");
    if pdata_path.exists() {
        if let Some(reader) = RelationRuleReader::from_ebase(pdata_path) {
            println!("\n=== Relation Rules ({}) ===", reader.varcond_rules.len());
            for rule in &reader.varcond_rules {
                println!("  {} -> {:?}", rule.var_cond, rule.condition);
            }
        } else {
            println!("\nNo relation rules found in pdata.ebase!");
        }
    } else {
        println!("\npdata.ebase not found at expected path!");
    }

    let props = loader.get_properties_for_family(&family);

    println!("\n=== Properties with multiple options ===");
    for prop in &props {
        if prop.options.len() > 1 {
            let values: Vec<&str> = prop.options.iter().map(|o| o.value.as_str()).collect();
            println!("  {} ({} options): {:?}", prop.key, prop.options.len(), values);
        }
    }

    println!("\n=== Single-option properties ===");
    for prop in &props {
        if prop.options.len() == 1 {
            println!("  {} = {}", prop.key, prop.options[0].value);
        }
    }

    let mut config = FamilyConfiguration::new(&family.id, &props);
    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    // Test which relation rules match with default config
    println!("\n=== Testing Relation Rule Matches ===");
    if let Some(reader) = RelationRuleReader::from_ebase(pdata_path) {
        use std::collections::HashMap;
        let mut props: HashMap<String, String> = config
            .selections
            .iter()
            .map(|(k, v)| {
                let key = k.to_uppercase();
                let prop_key = if key.starts_with("M_") { key } else { format!("M_{}", key) };
                (prop_key, v.to_uppercase())
            })
            .collect();
        props.insert("M_ARTNO".to_string(), family.base_article_nr.to_uppercase());

        println!("Property map for evaluation:");
        for (k, v) in props.iter().filter(|(k, _)| k.contains("LAN") || k.contains("WHITEBOARD") || k.contains("MOV") || k.contains("ARTNO")) {
            println!("  {} = {}", k, v);
        }

        let matched = reader.evaluate(&props);
        println!("\nMatched var_conds: {:?}", matched);
    }

    // Default price
    println!("\n=== Pricing Tests ===");
    let price1 = engine.calculate_family_price("framery", &family, &config, today);
    println!(
        "Default: {} EUR ({} surcharges)",
        price1.as_ref().map(|p| p.total_price.to_string()).unwrap_or("None".to_string()),
        price1.as_ref().map(|p| p.surcharges.len()).unwrap_or(0)
    );
    if let Some(ref p) = price1 {
        for s in &p.surcharges {
            println!("  + {}: {} EUR", s.name, s.amount);
        }
    }

    // Try changing properties that should affect price
    println!("\n=== Testing property changes ===");

    // Test M_LAN
    let m_lan_opts: Vec<String> = props.iter()
        .find(|p| p.key == "M_LAN")
        .map(|p| p.options.iter().map(|o| o.value.clone()).collect())
        .unwrap_or_default();
    println!("M_LAN options: {:?}", m_lan_opts);

    if m_lan_opts.len() > 1 {
        let current = config.get("M_LAN").unwrap_or("?").to_string();
        let new_val = if current == "YES" { "NO" } else { "YES" };
        println!("Changing M_LAN: {} -> {}", current, new_val);
        config.set("M_LAN", new_val);
        let price2 = engine.calculate_family_price("framery", &family, &config, today);
        println!(
            "After M_LAN={}: {} EUR ({} surcharges)",
            new_val,
            price2.as_ref().map(|p| p.total_price.to_string()).unwrap_or("None".to_string()),
            price2.as_ref().map(|p| p.surcharges.len()).unwrap_or(0)
        );
    } else {
        println!("M_LAN has only {} option(s), cannot toggle", m_lan_opts.len());
    }

    // Test M_MOVABILITY_KIT
    let m_mov_opts: Vec<String> = props.iter()
        .find(|p| p.key == "M_MOVABILITY_KIT")
        .map(|p| p.options.iter().map(|o| o.value.clone()).collect())
        .unwrap_or_default();
    println!("\nM_MOVABILITY_KIT options: {:?}", m_mov_opts);

    if m_mov_opts.contains(&"YES".to_string()) {
        config.set("M_MOVABILITY_KIT", "YES");
        let price3 = engine.calculate_family_price("framery", &family, &config, today);
        println!(
            "After M_MOVABILITY_KIT=YES: {} EUR ({} surcharges)",
            price3.as_ref().map(|p| p.total_price.to_string()).unwrap_or("None".to_string()),
            price3.as_ref().map(|p| p.surcharges.len()).unwrap_or(0)
        );
        if let Some(ref p) = price3 {
            for s in &p.surcharges {
                println!("  + {}: {} EUR", s.name, s.amount);
            }
        }
    }
}

#[test]
fn check_frmr_four_propvalue2varcond() {
    use ofml_lib::ebase::EBaseReader;
    use std::path::Path;

    let pdata_path = Path::new("/reference/ofmldata/framery/frmr_four/ANY/1/db/pdata.ebase");
    if !pdata_path.exists() {
        return;
    }

    let mut reader = EBaseReader::open(pdata_path).expect("Should open");

    // Check for propvalue2varcond table
    if let Ok(records) = reader.read_records("ocd_propvalue2varcond", None) {
        println!("\n=== FRMR_FOUR propvalue2varcond ({} records) ===", records.len());
        for record in records.iter().take(10) {
            println!("  {:?}", record);
        }
    } else {
        println!("\n=== No propvalue2varcond table in FRMR_FOUR ===");
    }
}

#[test]
fn dump_frmr_four_raw_relations() {
    use ofml_lib::ebase::EBaseReader;
    use std::path::Path;

    let pdata_path = Path::new("/reference/ofmldata/framery/frmr_four/ANY/1/db/pdata.ebase");
    if !pdata_path.exists() {
        return;
    }

    let mut reader = EBaseReader::open(pdata_path).expect("Should open");

    // Check ocd_relationobj
    println!("\n=== ocd_relationobj ===");
    if let Ok(records) = reader.read_records("ocd_relationobj", None) {
        for record in &records {
            let rel_name = record.get("rel_name").and_then(|v| v.as_str()).unwrap_or("?");
            let rel_domain = record.get("rel_domain").and_then(|v| v.as_str()).unwrap_or("?");
            if rel_domain.contains('P') {
                println!("  {} (domain={})", rel_name, rel_domain);
            }
        }
    }

    // Check ocd_relation for MOVABILITY
    println!("\n=== ocd_relation (searching for MOVABILITY) ===");
    if let Ok(records) = reader.read_records("ocd_relation", None) {
        for record in &records {
            let rel_block = record.get("rel_block").and_then(|v| v.as_str()).unwrap_or("");
            if rel_block.to_uppercase().contains("MOVABILITY") || rel_block.to_uppercase().contains("PG_MOV") {
                let rel_name = record.get("rel_name").and_then(|v| v.as_str()).unwrap_or("?");
                println!("  [{}] {}", rel_name, rel_block);
            }
        }
        println!("  (searched {} relation blocks)", records.len());
    }
}
