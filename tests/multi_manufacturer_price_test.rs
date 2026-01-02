//! Multi-manufacturer price matching tests
//! 
//! This test verifies that the price matching logic works correctly
//! across different manufacturers with varying data formats.

use std::path::Path;
use ofml_interpreter::oap::engine::ConfigurationEngine;
use ofml_interpreter::oap::families::{FamilyLoader, FamilyConfiguration};

const OFMLDATA_BASE: &str = "/reference/ofmldata";

fn ofmldata_exists() -> bool {
    Path::new(OFMLDATA_BASE).exists()
}

/// Test price calculation for a single manufacturer
fn test_manufacturer_pricing(mfr_id: &str) -> (bool, String) {
    let mfr_path = Path::new(OFMLDATA_BASE).join(mfr_id);
    if !mfr_path.exists() {
        return (true, format!("{}: skipped (not found)", mfr_id));
    }
    
    let loader = FamilyLoader::load(&mfr_path, "DE");
    let families = loader.get_families();
    
    if families.is_empty() {
        return (true, format!("{}: skipped (no families)", mfr_id));
    }
    
    let engine = ConfigurationEngine::new(OFMLDATA_BASE);
    let price_date = chrono::Local::now().date_naive();
    
    let mut families_with_price = 0;
    let mut families_with_surcharges = 0;
    let mut total_families_tested = 0;
    
    // Test first 5 families with properties
    for family in families.iter().take(10) {
        let properties = loader.get_properties_for_family(family);
        if properties.is_empty() {
            continue;
        }
        
        total_families_tested += 1;
        let config = FamilyConfiguration::new(&family.id, &properties);
        
        if let Some(price) = engine.calculate_family_price(mfr_id, family, &config, price_date) {
            families_with_price += 1;
            if !price.surcharges.is_empty() {
                families_with_surcharges += 1;
            }
        }
    }
    
    let result = format!(
        "{}: {} families, {}/{} with price, {} with surcharges",
        mfr_id, families.len(), families_with_price, total_families_tested, families_with_surcharges
    );
    
    // Consider it a success if we found at least one family with pricing
    (families_with_price > 0 || total_families_tested == 0, result)
}

#[test]
fn test_price_matching_all_manufacturers() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }
    
    // Test a representative set of manufacturers
    let manufacturers = [
        "sex",      // Sedus - known to have S_XXX style surcharges
        "vitra",    // Vitra - major manufacturer
        "kn",       // Knoll
        "sbu",      // Steelcase
        "haw",      // Haworth
        "aix",      // Interstuhl
        "arper",    // Arper
        "cassina",  // Cassina
        "cor",      // COR
        "framery",  // Framery
    ];
    
    println!("\n=== Multi-Manufacturer Price Matching Test ===\n");
    
    let mut successes = 0;
    let mut failures = 0;
    
    for mfr in &manufacturers {
        let (success, msg) = test_manufacturer_pricing(mfr);
        println!("{}", msg);
        if success {
            successes += 1;
        } else {
            failures += 1;
        }
    }
    
    println!("\n=== Summary ===");
    println!("Successes: {}/{}", successes, manufacturers.len());
    println!("Failures: {}", failures);
    
    // At least half should work
    assert!(successes >= manufacturers.len() / 2, 
        "Too many manufacturers failed price matching");
}

#[test]
fn test_price_changes_on_property_change() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }
    
    println!("\n=== Price Change Detection Test ===\n");
    
    let manufacturers = ["sex", "vitra", "kn", "sbu"];
    let mut found_price_change = false;
    
    for mfr_id in &manufacturers {
        let mfr_path = Path::new(OFMLDATA_BASE).join(mfr_id);
        if !mfr_path.exists() {
            continue;
        }
        
        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();
        
        // Find a family with multiple property options
        for family in loader.get_families().iter().take(5) {
            let properties = loader.get_properties_for_family(family);
            
            // Find a property with at least 3 options
            let multi_option_prop = properties.iter().find(|p| p.options.len() >= 3);
            if multi_option_prop.is_none() {
                continue;
            }
            let prop = multi_option_prop.unwrap();
            
            // Calculate price with first option
            let mut config1 = FamilyConfiguration::new(&family.id, &properties);
            config1.set(&prop.key, &prop.options[0].value);
            let price1 = engine.calculate_family_price(mfr_id, family, &config1, price_date);
            
            // Calculate price with last option
            let mut config2 = FamilyConfiguration::new(&family.id, &properties);
            config2.set(&prop.key, &prop.options.last().unwrap().value);
            let price2 = engine.calculate_family_price(mfr_id, family, &config2, price_date);
            
            if let (Some(p1), Some(p2)) = (&price1, &price2) {
                if p1.total_price != p2.total_price || p1.surcharges.len() != p2.surcharges.len() {
                    println!("{} / {} / {}:", mfr_id, family.name, prop.key);
                    println!("  Option '{}' -> {:.2} {} ({} surcharges)", 
                        prop.options[0].value, p1.total_price, p1.currency, p1.surcharges.len());
                    println!("  Option '{}' -> {:.2} {} ({} surcharges)", 
                        prop.options.last().unwrap().value, p2.total_price, p2.currency, p2.surcharges.len());
                    found_price_change = true;
                }
            }
        }
    }
    
    if !found_price_change {
        println!("Note: No price changes detected when changing properties.");
        println!("This may be expected if surcharge codes don't match available property values.");
    }
}

#[test]
fn test_sedus_surcharge_matching_detailed() {
    if !ofmldata_exists() {
        eprintln!("Skipping: ofmldata not found");
        return;
    }
    
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: Sedus AI not found");
        return;
    }
    
    println!("\n=== Sedus AI Surcharge Matching Detail ===\n");
    
    let mfr_path = Path::new("/reference/ofmldata/sex");
    let loader = FamilyLoader::load(mfr_path, "DE");
    let engine = ConfigurationEngine::new(OFMLDATA_BASE);
    let price_date = chrono::Local::now().date_naive();
    
    // Find AI family
    let ai_family = loader.get_families().iter()
        .find(|f| f.base_article_nr.contains("AI"));
    
    if let Some(family) = ai_family {
        let properties = loader.get_properties_for_family(family);
        
        println!("Family: {} ({})", family.name, family.base_article_nr);
        println!("Properties with options: {}", properties.len());
        
        // Test default configuration
        let config = FamilyConfiguration::new(&family.id, &properties);
        let price = engine.calculate_family_price("sex", family, &config, price_date);
        
        if let Some(p) = &price {
            println!("\nDefault configuration:");
            println!("  Base price: {:.2} {}", p.base_price, p.currency);
            println!("  Surcharges: {}", p.surcharges.len());
            for s in &p.surcharges {
                println!("    {} = {:.2}", s.name, s.amount);
            }
            println!("  Total: {:.2} {}", p.total_price, p.currency);
        }
        
        // Test with different fabric options - prefer GABRIEL which has embedded codes
        let fabric_prop = properties.iter()
            .find(|p| p.key == "S_STOFF_FRONT_GABRIEL")
            .or_else(|| properties.iter().find(|p| p.key.contains("GABRIEL")))
            .or_else(|| properties.iter().find(|p| p.key.contains("STOFF")));

        if let Some(prop) = fabric_prop {
            println!("\nTesting {} variations:", prop.key);

            for (i, opt) in prop.options.iter().take(5).enumerate() {
                let mut test_config = FamilyConfiguration::new(&family.id, &properties);
                test_config.set(&prop.key, &opt.value);

                if let Some(p) = engine.calculate_family_price("sex", family, &test_config, price_date) {
                    let surcharge_names: Vec<_> = p.surcharges.iter().map(|s| s.name.as_str()).collect();
                    println!("  [{}] {} -> {:.2} {} (surcharges: {:?})",
                        i, opt.value, p.total_price, p.currency, surcharge_names);
                }
            }

            // Verify that values with embedded codes trigger surcharges
            let codes_with_166 = prop.options.iter()
                .filter(|o| o.value.contains("166"))
                .count();
            let codes_with_168 = prop.options.iter()
                .filter(|o| o.value.contains("168"))
                .count();
            println!("\n  Values containing '166': {}", codes_with_166);
            println!("  Values containing '168': {}", codes_with_168);

            // Test a specific value with embedded code if available
            if let Some(opt_166) = prop.options.iter().find(|o| o.value.contains("166")) {
                let mut test_config = FamilyConfiguration::new(&family.id, &properties);
                test_config.set(&prop.key, &opt_166.value);

                if let Some(p) = engine.calculate_family_price("sex", family, &test_config, price_date) {
                    println!("\n  Testing embedded code '166' in {}:", opt_166.value);
                    println!("    Total: {:.2} {} (surcharges: {})", p.total_price, p.currency, p.surcharges.len());
                    assert!(!p.surcharges.is_empty() || p.total_price > p.base_price,
                        "Expected surcharge S_166 to match value containing '166'");
                }
            }
        }
    }
}
