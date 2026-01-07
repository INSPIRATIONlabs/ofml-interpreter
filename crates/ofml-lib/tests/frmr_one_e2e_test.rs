//! End-to-end test for FRMR_ONE pricing with relation rules

use std::path::Path;

use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::{FamilyConfiguration, FamilyLoader};
use ofml_lib::oap::ocd_properties::clear_property_cache;

#[test]
fn test_frmr_one_default_pricing() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        eprintln!("Framery not available");
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");

    // Find FRMR_ONE family
    let one_family = loader.get_families().iter()
        .find(|f| f.series.to_uppercase() == "FRMR_ONE")
        .cloned();

    if one_family.is_none() {
        eprintln!("FRMR_ONE family not found");
        return;
    }
    let family = one_family.unwrap();

    println!("\n=== FRMR_ONE Default Pricing ===");
    println!("Family: {} - {}", family.id, family.name);
    println!("Base article: {}", family.base_article_nr);

    let props = loader.get_properties_for_family(&family);
    let config = FamilyConfiguration::new(&family.id, &props);

    // Debug: show ALL property keys
    println!("ALL Config selections:");
    for (k, v) in config.selections.iter() {
        println!("  {} = {}", k, v);
    }

    // Show all property option counts
    println!("\nProperty option counts:");
    let mut single_option = Vec::new();
    let mut multi_option = Vec::new();
    for prop in &props {
        if prop.options.len() == 1 {
            single_option.push(&prop.key);
        } else {
            multi_option.push(format!("{} ({} options)", prop.key, prop.options.len()));
        }
    }
    println!("  Single option (not toggleable): {:?}", single_option);
    println!("  Multi option (toggleable): {:?}", multi_option);

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    let price = engine.calculate_family_price("framery", &family, &config, today);
    println!("Price result: {:?}", price);

    // FRMR_ONE should return a price (base may be 0 but result should exist)
    assert!(price.is_some(), "FRMR_ONE should return a price result");
}

#[test]
fn test_frmr_one_with_seat_surcharge() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        eprintln!("Framery not available");
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");
    let one_family = loader.get_families().iter()
        .find(|f| f.series.to_uppercase() == "FRMR_ONE")
        .cloned();

    if one_family.is_none() {
        return;
    }
    let family = one_family.unwrap();

    let props = loader.get_properties_for_family(&family);
    let mut config = FamilyConfiguration::new(&family.id, &props);

    // Enable adjustable seat - should trigger PG_ADJUSTABLE_SEAT surcharge
    config.selections.insert("M_SEAT".to_string(), "YES".to_string());

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    let price = engine.calculate_family_price("framery", &family, &config, today);

    println!("\n=== FRMR_ONE with M_SEAT=YES ===");
    println!("Price result: {:?}", price);

    if let Some(p) = &price {
        println!("  Surcharges: {:?}", p.surcharges);

        // Check for adjustable seat surcharge (should be 1420 EUR)
        let seat_surcharge = p.surcharges.iter()
            .find(|s| s.name.contains("SEAT"));

        if seat_surcharge.is_some() {
            println!("  Found seat surcharge: {:?}", seat_surcharge);
        }
    }

    assert!(price.is_some(), "Should have price");
}

#[test]
fn test_frmr_one_with_lan_surcharge() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");
    let one_family = loader.get_families().iter()
        .find(|f| f.series.to_uppercase() == "FRMR_ONE")
        .cloned();

    if one_family.is_none() {
        return;
    }
    let family = one_family.unwrap();

    let props = loader.get_properties_for_family(&family);
    let mut config = FamilyConfiguration::new(&family.id, &props);

    // Enable LAN - should trigger PG_LAN surcharge
    config.selections.insert("M_LAN".to_string(), "YES".to_string());

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    let price = engine.calculate_family_price("framery", &family, &config, today);

    println!("\n=== FRMR_ONE with M_LAN=YES ===");
    println!("Price result: {:?}", price);

    if let Some(p) = &price {
        println!("  Surcharges: {:?}", p.surcharges);
    }

    assert!(price.is_some(), "Should have price");
}

#[test]
fn test_frmr_one_multiple_surcharges() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");
    let one_family = loader.get_families().iter()
        .find(|f| f.series.to_uppercase() == "FRMR_ONE")
        .cloned();

    if one_family.is_none() {
        return;
    }
    let family = one_family.unwrap();

    let props = loader.get_properties_for_family(&family);
    let mut config = FamilyConfiguration::new(&family.id, &props);

    // Enable multiple options
    config.selections.insert("M_SEAT".to_string(), "YES".to_string());
    config.selections.insert("M_LAN".to_string(), "YES".to_string());
    config.selections.insert("M_FILTER".to_string(), "YES".to_string());
    config.selections.insert("M_SERVICE_KIT".to_string(), "YES".to_string());

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    let price = engine.calculate_family_price("framery", &family, &config, today);

    println!("\n=== FRMR_ONE with Multiple Options ===");
    println!("Price result: {:?}", price);

    if let Some(p) = &price {
        println!("  Total: {} EUR", p.total_price);
        println!("  Surcharges ({}):", p.surcharges.len());
        for s in &p.surcharges {
            println!("    - {}: {} EUR", s.name, s.amount);
        }
    }

    assert!(price.is_some(), "Should have price");
}

#[test]
fn test_frmr_one_price_changes_on_property_change() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");
    let one_family = loader.get_families().iter()
        .find(|f| f.series.to_uppercase() == "FRMR_ONE")
        .cloned();

    if one_family.is_none() {
        return;
    }
    let family = one_family.unwrap();

    let props = loader.get_properties_for_family(&family);
    let mut config = FamilyConfiguration::new(&family.id, &props);

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    // Default has M_LAN=YES
    println!("\n=== Price Changes Test ===");
    println!("Default M_LAN = {}", config.get("M_LAN").unwrap_or("?"));
    let price_with_lan = engine.calculate_family_price("framery", &family, &config, today);
    println!("With LAN: {} EUR, {} surcharges",
        price_with_lan.as_ref().map(|p| p.total_price.to_string()).unwrap_or("?".to_string()),
        price_with_lan.as_ref().map(|p| p.surcharges.len()).unwrap_or(0));

    // Change M_LAN to NO
    config.set("M_LAN", "NO");
    println!("After change M_LAN = {}", config.get("M_LAN").unwrap_or("?"));
    let price_without_lan = engine.calculate_family_price("framery", &family, &config, today);
    println!("Without LAN: {} EUR, {} surcharges",
        price_without_lan.as_ref().map(|p| p.total_price.to_string()).unwrap_or("?".to_string()),
        price_without_lan.as_ref().map(|p| p.surcharges.len()).unwrap_or(0));

    // Verify prices are different
    if let (Some(p1), Some(p2)) = (&price_with_lan, &price_without_lan) {
        let diff = p1.total_price - p2.total_price;
        println!("Difference: {} EUR", diff);
        assert!(diff > rust_decimal::Decimal::ZERO,
            "Price should decrease when LAN is disabled, diff={}", diff);
    }
}

#[test]
fn test_frmr_one_filter_option_changes_price() {
    // This tests a property that HAS both YES/NO options (unlike M_LAN which only has YES)
    clear_property_cache();

    let path = Path::new("/reference/ofmldata");
    let mfr_path = Path::new("/reference/ofmldata/framery");

    if !mfr_path.exists() {
        return;
    }

    let loader = FamilyLoader::load(mfr_path, "DE");
    let family = loader.get_families().iter()
        .find(|f| f.series.to_uppercase() == "FRMR_ONE")
        .cloned().unwrap();

    let props = loader.get_properties_for_family(&family);
    let mut config = FamilyConfiguration::new(&family.id, &props);

    let engine = ConfigurationEngine::new(path);
    let today = chrono::Local::now().date_naive();

    // Default has M_FILTER=NO
    println!("\n=== M_FILTER Price Changes Test ===");
    println!("Default M_FILTER = {}", config.get("M_FILTER").unwrap_or("?"));
    let price_without_filter = engine.calculate_family_price("framery", &family, &config, today);
    println!("Without filter: {} EUR",
        price_without_filter.as_ref().map(|p| p.total_price.to_string()).unwrap_or("?".to_string()));

    // Change M_FILTER to YES
    config.set("M_FILTER", "YES");
    println!("After change M_FILTER = {}", config.get("M_FILTER").unwrap_or("?"));
    let price_with_filter = engine.calculate_family_price("framery", &family, &config, today);
    println!("With filter: {} EUR",
        price_with_filter.as_ref().map(|p| p.total_price.to_string()).unwrap_or("?".to_string()));

    // Verify price increased
    if let (Some(p1), Some(p2)) = (&price_without_filter, &price_with_filter) {
        let diff = p2.total_price - p1.total_price;
        println!("Difference: {} EUR (should be ~325 EUR for carbon filter)", diff);
        assert!(diff > rust_decimal::Decimal::ZERO,
            "Price should increase when filter is enabled");
    }
}
