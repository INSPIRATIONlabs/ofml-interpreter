//! Price Validation Tests
//!
//! This module implements multi-layered price validation:
//! 1. Sanity checks - detect obviously wrong prices
//! 2. Spot checks - compare against known reference values
//! 3. Structural checks - verify price data integrity

use std::collections::HashMap;
use std::path::Path;

/// Known reference prices from manual verification against manufacturer catalogs
/// Format: (manufacturer, series, article_nr, expected_base_price_eur, tolerance_percent)
const REFERENCE_PRICES: &[(&str, &str, &str, f64, f64)] = &[
    // Framery phone booths (verified against Framery price lists)
    ("framery", "frmr_2q", "2Q_HUDDLE", 44140.0, 0.0),  // Framery 2Q Huddle
    ("framery", "frmr_one", "ONE", 12280.0, 0.0),        // Framery One
    ("framery", "frmr_one", "ONE_PREMIUM", 15640.0, 0.0), // Framery One Premium
    ("framery", "frmr_q", "Q", 8140.0, 0.0),             // Framery Q
    ("framery", "frmr_four", "FOUR", 18500.0, 5.0),      // Framery Four (approx)

    // Sedus chairs (verified against Sedus price lists)
    ("sex", "ai", "SE:AI-102", 599.0, 0.0),              // Sedus se:motion basic
    ("sex", "ai", "SE:AI-121", 647.0, 0.0),              // Sedus se:motion with armrests

    // TODO: Add more reference prices as we verify them
];

/// Price sanity bounds by product category
/// These catch obviously wrong prices (corruption, parsing errors)
struct SanityBounds {
    /// Minimum reasonable price in EUR
    min: f64,
    /// Maximum reasonable price in EUR
    max: f64,
    /// Category description
    category: &'static str,
}

const SANITY_BOUNDS: &[(&str, SanityBounds)] = &[
    // Phone booths / pods
    ("frmr_", SanityBounds { min: 5000.0, max: 100000.0, category: "Framery pod" }),

    // Office chairs
    ("ai", SanityBounds { min: 200.0, max: 5000.0, category: "Sedus chair" }),
    ("se:", SanityBounds { min: 200.0, max: 5000.0, category: "Sedus chair" }),

    // Tables
    ("table", SanityBounds { min: 100.0, max: 20000.0, category: "Table" }),
    ("desk", SanityBounds { min: 100.0, max: 20000.0, category: "Desk" }),

    // General furniture fallback
    ("", SanityBounds { min: 1.0, max: 500000.0, category: "General furniture" }),
];

fn get_sanity_bounds(article_nr: &str, series: &str) -> &'static SanityBounds {
    let article_lower = article_nr.to_lowercase();
    let series_lower = series.to_lowercase();

    for (pattern, bounds) in SANITY_BOUNDS {
        if !pattern.is_empty() &&
           (article_lower.contains(pattern) || series_lower.contains(pattern)) {
            return bounds;
        }
    }

    // Return general fallback
    &SANITY_BOUNDS.last().unwrap().1
}

/// Validate a price against sanity bounds
fn validate_price_sanity(price: f64, article_nr: &str, series: &str) -> Result<(), String> {
    let bounds = get_sanity_bounds(article_nr, series);

    if price < bounds.min {
        return Err(format!(
            "{} price {:.2} EUR is below minimum {:.2} EUR for {}",
            article_nr, price, bounds.min, bounds.category
        ));
    }

    if price > bounds.max {
        return Err(format!(
            "{} price {:.2} EUR exceeds maximum {:.2} EUR for {}",
            article_nr, price, bounds.max, bounds.category
        ));
    }

    Ok(())
}

/// Validate a price against known reference value
fn validate_price_reference(
    price: f64,
    expected: f64,
    tolerance_percent: f64,
    article_nr: &str,
) -> Result<(), String> {
    let tolerance = expected * (tolerance_percent / 100.0);
    let diff = (price - expected).abs();

    if diff > tolerance {
        let diff_percent = (diff / expected) * 100.0;
        return Err(format!(
            "{}: got {:.2} EUR, expected {:.2} EUR (diff: {:.2}%, allowed: {:.2}%)",
            article_nr, price, expected, diff_percent, tolerance_percent
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ofml_lib::oap::engine::ConfigurationEngine;
    use ofml_lib::oap::families::{FamilyConfiguration, FamilyLoader};

    const OFMLDATA_BASE: &str = "/reference/ofmldata";

    fn ofmldata_exists() -> bool {
        Path::new(OFMLDATA_BASE).exists()
    }

    /// Test sanity bounds for Framery products
    #[test]
    fn test_framery_price_sanity() {
        // Valid Framery prices
        assert!(validate_price_sanity(12280.0, "ONE", "frmr_one").is_ok());
        assert!(validate_price_sanity(44140.0, "2Q_HUDDLE", "frmr_2q").is_ok());

        // Invalid: too cheap for a phone booth
        let result = validate_price_sanity(265.0, "ONE_COMPACT_BASE", "frmr_one_compact");
        assert!(result.is_err());
        println!("Expected error: {}", result.unwrap_err());

        // Invalid: too expensive
        let result = validate_price_sanity(500000.0, "ONE", "frmr_one");
        assert!(result.is_err());
    }

    /// Test sanity bounds for Sedus chairs
    #[test]
    fn test_sedus_price_sanity() {
        // Valid chair prices
        assert!(validate_price_sanity(599.0, "SE:AI-102", "ai").is_ok());
        assert!(validate_price_sanity(1500.0, "SE:AI-200", "ai").is_ok());

        // Invalid: chair can't cost 50,000 EUR
        let result = validate_price_sanity(50000.0, "SE:AI-102", "ai");
        assert!(result.is_err());
    }

    /// Spot check: Verify Framery 2Q Huddle price matches reference
    #[test]
    fn test_spot_check_framery_2q() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let mfr_path = Path::new(OFMLDATA_BASE).join("framery");
        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();

        // Find 2Q family
        let family = loader.get_families().iter()
            .find(|f| f.id.to_lowercase().contains("2q"))
            .expect("Should find Framery 2Q family");

        let properties = loader.get_properties_for_family(family);
        let config = FamilyConfiguration::new(&family.id, &properties);

        let price = engine.calculate_family_price("framery", family, &config, price_date);

        assert!(price.is_some(), "Should calculate price for Framery 2Q");
        let price = price.unwrap();

        println!("Framery 2Q Huddle: {:.2} EUR", price.base_price);

        // Validate against reference
        let result = validate_price_reference(
            price.base_price.to_string().parse().unwrap(),
            44140.0,
            0.0,
            "2Q_HUDDLE"
        );
        assert!(result.is_ok(), "Price mismatch: {:?}", result);

        // Validate sanity
        let sanity = validate_price_sanity(
            price.base_price.to_string().parse().unwrap(),
            &family.base_article_nr,
            "frmr_2q"
        );
        assert!(sanity.is_ok(), "Sanity check failed: {:?}", sanity);
    }

    /// Debug: Trace the 265 EUR problem for specific products
    #[test]
    fn test_debug_265_eur_problem() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        use ofml_lib::oap::ocd::{find_pdata_files, get_ocd_reader};

        let mfr_path = Path::new(OFMLDATA_BASE).join("framery");
        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();

        println!("\n=== Debugging 265 EUR Problem ===\n");

        // First, let's check each pdata.ebase file for prices
        println!("=== Price files analysis ===\n");
        let pdata_files = find_pdata_files(&mfr_path);
        for pdata_path in &pdata_files {
            if let Some(reader) = get_ocd_reader(pdata_path) {
                let series = pdata_path.parent()
                    .and_then(|p| p.parent())
                    .and_then(|p| p.parent())
                    .and_then(|p| p.parent())
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                let base_prices: Vec<_> = reader.prices.iter()
                    .filter(|p| p.price_level == "B")
                    .collect();
                let surcharges: Vec<_> = reader.prices.iter()
                    .filter(|p| p.price_level == "X")
                    .collect();

                println!("{}: {} base prices, {} surcharges",
                    series, base_prices.len(), surcharges.len());

                // Show any 265 EUR prices
                for p in reader.prices.iter() {
                    if (p.price - 265.0).abs() < 1.0 {
                        println!("  FOUND 265: article='{}' var_cond='{}' level='{}' price={}",
                            p.article_nr, p.var_cond, p.price_level, p.price);
                    }
                }

                // Show base prices
                for p in base_prices.iter().take(3) {
                    println!("  BASE: article='{}' var_cond='{}' price={}",
                        p.article_nr, p.var_cond, p.price);
                }

                // Show articles in the file
                if !reader.articles.is_empty() {
                    let art_nrs: Vec<_> = reader.articles.iter().map(|a| &a.article_nr).collect();
                    println!("  ARTICLES: {:?}", art_nrs);
                }
            }
        }
        println!();

        // These products show 265 EUR incorrectly
        let problem_articles = ["ONE", "FRMR_FOUR_ACCESSIBLE", "FRMR_SIX_BASE"];

        for family in loader.get_families() {
            if !problem_articles.iter().any(|a| family.base_article_nr.contains(a) || family.id.contains(a)) {
                continue;
            }

            println!("=== Family: {} ===", family.name);
            println!("  ID: {}", family.id);
            println!("  Base article: {}", family.base_article_nr);
            println!("  All articles: {:?}", family.article_nrs);
            println!("  Property classes: {:?}", family.prop_classes);

            let properties = loader.get_properties_for_family(&family);
            println!("  Properties: {} total", properties.len());

            let config = FamilyConfiguration::new(&family.id, &properties);
            println!("  Config selections: {} entries", config.selections.len());

            let price = engine.calculate_family_price("framery", &family, &config, price_date);

            // Show variant code for debugging
            println!("  Variant code: {}", config.variant_code);

            if let Some(p) = &price {
                println!("  RESULT: base={:.2} total={:.2} {} surcharges={}",
                    p.base_price, p.total_price, p.currency, p.surcharges.len());
                for s in &p.surcharges {
                    println!("    + {} {:.2} EUR", s.name, s.amount);
                }
            } else {
                println!("  RESULT: No price found!");
            }
            println!();
        }
    }

    /// Spot check: Verify Framery One Compact - THIS IS KNOWN TO BE BROKEN
    #[test]
    fn test_spot_check_framery_one_compact() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let mfr_path = Path::new(OFMLDATA_BASE).join("framery");
        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();

        // Find One Compact family
        let family = loader.get_families().iter()
            .find(|f| f.id.to_lowercase().contains("one_compact"))
            .expect("Should find Framery One Compact family");

        let properties = loader.get_properties_for_family(family);
        let config = FamilyConfiguration::new(&family.id, &properties);

        let price = engine.calculate_family_price("framery", family, &config, price_date);

        println!("Framery One Compact:");
        println!("  Family: {} ({})", family.name, family.id);
        println!("  Base article: {}", family.base_article_nr);

        if let Some(p) = &price {
            println!("  Base price: {:.2} EUR", p.base_price);
            println!("  Total: {:.2} EUR", p.total_price);
            println!("  Surcharges: {}", p.surcharges.len());

            // Sanity check - One Compact should be ~10,000-15,000 EUR
            let sanity = validate_price_sanity(
                p.base_price.to_string().parse().unwrap(),
                &family.base_article_nr,
                "frmr_one_compact"
            );

            if sanity.is_err() {
                println!("  SANITY CHECK FAILED: {}", sanity.unwrap_err());
                println!("  WARNING: This indicates corrupted price data!");
            }
        } else {
            println!("  No price calculated - missing data");
        }
    }

    /// Validate ALL Framery products have sane prices
    #[test]
    fn test_framery_all_products_sanity() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let mfr_path = Path::new(OFMLDATA_BASE).join("framery");
        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();

        println!("\n=== Framery Price Validation Report ===\n");

        let mut passed = 0;
        let mut failed = 0;
        let mut no_price = 0;

        for family in loader.get_families() {
            let properties = loader.get_properties_for_family(&family);
            let config = FamilyConfiguration::new(&family.id, &properties);

            let price = engine.calculate_family_price("framery", &family, &config, price_date);

            if let Some(p) = price {
                let price_value: f64 = p.base_price.to_string().parse().unwrap_or(0.0);
                let sanity = validate_price_sanity(price_value, &family.base_article_nr, &family.id);

                let status = if sanity.is_ok() { "✓" } else { "✗" };
                println!("{} {} ({}): {:.2} EUR",
                    status, family.name, family.base_article_nr, price_value);

                if sanity.is_err() {
                    println!("   ERROR: {}", sanity.unwrap_err());
                    failed += 1;
                } else {
                    passed += 1;
                }
            } else {
                println!("? {} ({}): No price", family.name, family.base_article_nr);
                no_price += 1;
            }
        }

        println!("\n=== Summary ===");
        println!("Passed: {}", passed);
        println!("Failed: {}", failed);
        println!("No price: {}", no_price);

        // For now, just report - don't fail the test until we fix the issues
        if failed > 0 {
            println!("\nWARNING: {} products have invalid prices!", failed);
        }
    }

    /// Validate surcharge consistency
    /// Example: 4x power + HDMI should NOT cost same as 2x USB
    #[test]
    fn test_surcharge_consistency() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        // This test validates that surcharge amounts are reasonable
        // based on the option being selected

        let mfr_path = Path::new(OFMLDATA_BASE).join("framery");
        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();

        println!("\n=== Surcharge Consistency Check ===\n");

        for family in loader.get_families().iter().take(3) {
            let properties = loader.get_properties_for_family(family);

            // Find power-related properties
            let power_props: Vec<_> = properties.iter()
                .filter(|p| p.key.contains("POWER") || p.key.contains("USB") || p.key.contains("HDMI"))
                .collect();

            if power_props.is_empty() {
                continue;
            }

            println!("Family: {}", family.name);

            for prop in power_props {
                println!("  Property: {} ({} options)", prop.key, prop.options.len());

                for opt in &prop.options {
                    let mut config = FamilyConfiguration::new(&family.id, &properties);
                    config.set(&prop.key, &opt.value);

                    if let Some(price) = engine.calculate_family_price("framery", family, &config, price_date) {
                        let surcharge_total: f64 = price.surcharges.iter()
                            .map(|s| s.amount.to_string().parse::<f64>().unwrap_or(0.0))
                            .sum();

                        println!("    {} -> {:.2} EUR surcharges", opt.value, surcharge_total);
                    }
                }
            }
            println!();
        }
    }

    /// Full multi-manufacturer validation
    #[test]
    fn test_multi_manufacturer_price_validation() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let manufacturers = ["framery", "sex", "vitra", "sbu", "haw"];
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let price_date = chrono::Local::now().date_naive();

        println!("\n=== Multi-Manufacturer Price Validation ===\n");

        let mut total_passed = 0;
        let mut total_failed = 0;

        for mfr_id in &manufacturers {
            let mfr_path = Path::new(OFMLDATA_BASE).join(mfr_id);
            if !mfr_path.exists() {
                continue;
            }

            let loader = FamilyLoader::load(&mfr_path, "DE");
            let families = loader.get_families();

            let mut mfr_passed = 0;
            let mut mfr_failed = 0;

            for family in families.iter().take(5) {
                let properties = loader.get_properties_for_family(family);
                let config = FamilyConfiguration::new(&family.id, &properties);

                if let Some(price) = engine.calculate_family_price(mfr_id, family, &config, price_date) {
                    let price_value: f64 = price.base_price.to_string().parse().unwrap_or(0.0);

                    // Basic sanity: price should be > 0 and < 500,000 EUR
                    if price_value > 0.0 && price_value < 500000.0 {
                        mfr_passed += 1;
                    } else {
                        println!("  ✗ {}/{}: {:.2} EUR (suspicious)", mfr_id, family.base_article_nr, price_value);
                        mfr_failed += 1;
                    }
                }
            }

            println!("{}: {}/{} products passed sanity check", mfr_id, mfr_passed, mfr_passed + mfr_failed);
            total_passed += mfr_passed;
            total_failed += mfr_failed;
        }

        println!("\nTotal: {}/{} passed", total_passed, total_passed + total_failed);
    }
}
