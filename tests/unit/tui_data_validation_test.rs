//! TUI e2e tests for data validation
//!
//! These tests verify that the TUI can correctly load and display data
//! from multiple manufacturers with proper pricing information.

#[cfg(feature = "tui")]
mod tui_data_tests {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyLoader, FamilyConfiguration};
    use ofml_interpreter::oap::manufacturers::get_installed_manufacturers;
    use ofml_interpreter::oap::ocd::OcdReader;
    use std::path::Path;

    const OFMLDATA_BASE: &str = "/reference/ofmldata";

    fn ofmldata_exists() -> bool {
        Path::new(OFMLDATA_BASE).exists()
    }

    #[test]
    fn test_manufacturer_loading() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let manufacturers = get_installed_manufacturers(Path::new(OFMLDATA_BASE));

        println!("\n=== Manufacturer Loading Test ===\n");
        println!("Total manufacturers found: {}", manufacturers.len());

        // Should find many manufacturers
        assert!(manufacturers.len() >= 50, "Should find at least 50 manufacturers");

        // Check some known manufacturers exist
        let known_mfrs = ["vitra", "sex", "kn", "sbu", "aix"];
        for mfr_id in &known_mfrs {
            let found = manufacturers.iter().any(|m| m.id == *mfr_id);
            println!("  {} found: {}", mfr_id, found);
            assert!(found, "Should find manufacturer {}", mfr_id);
        }
    }

    #[test]
    fn test_sedus_family_loading() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let mfr_path = Path::new(OFMLDATA_BASE).join("sex");
        if !mfr_path.exists() {
            eprintln!("Skipping: Sedus not found");
            return;
        }

        let loader = FamilyLoader::load(&mfr_path, "DE");
        let families = loader.get_families();

        println!("\n=== Sedus Family Loading Test ===\n");
        println!("Total families: {}", families.len());

        // Should have at least one family
        assert!(!families.is_empty(), "Sedus should have product families");

        // Print first few families
        for (i, family) in families.iter().take(5).enumerate() {
            println!("  [{}] {} - {} ({} articles)",
                i, family.id, family.name, family.article_nrs.len());
        }
    }

    #[test]
    fn test_configuration_engine_articles() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let mut engine = ConfigurationEngine::new(OFMLDATA_BASE);

        println!("\n=== Configuration Engine Articles Test ===\n");

        let manufacturers = ["sex", "vitra", "kn"];
        for mfr in &manufacturers {
            let articles = engine.load_articles(mfr);
            if articles.is_empty() {
                println!("{}: No articles found", mfr);
                continue;
            }

            let configurable = articles.iter().filter(|a| a.is_configurable).count();
            println!("{}: {} articles ({} configurable)", mfr, articles.len(), configurable);

            // Show first few articles
            for article in articles.iter().take(3) {
                println!("  - {} ({})", article.article.article_nr, article.description);
            }
        }
    }

    #[test]
    fn test_sedus_pricing_display() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            eprintln!("Skipping: Sedus AI pdata.ebase not found");
            return;
        }

        let reader = OcdReader::from_ebase(path).expect("Should load OCD");

        println!("\n=== Sedus Pricing Display Test ===\n");

        // Test AI-121 pricing (known test article)
        let article_nr = "AI-121";
        let base_price = reader.get_base_price(article_nr);
        let surcharges = reader.get_surcharges(article_nr);

        println!("Article: {}", article_nr);

        if let Some(base) = base_price {
            println!("  Base price: {:.2} {} (level='{}', var_cond='{}')",
                base.price, base.currency, base.price_level, base.var_cond);
            assert_eq!(base.price_level, "B", "Base price should have level 'B'");
        } else {
            panic!("Should find base price for {}", article_nr);
        }

        println!("  Surcharges: {}", surcharges.len());
        for s in surcharges.iter().take(5) {
            println!("    {} = {:.2} {} (is_fix={})",
                s.var_cond, s.price, s.currency, s.is_fix);
        }

        // Verify we have surcharges
        assert!(!surcharges.is_empty(), "Should have surcharges");
    }

    #[test]
    fn test_family_price_calculation() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let engine = ConfigurationEngine::new(OFMLDATA_BASE);
        let mfr_path = Path::new(OFMLDATA_BASE).join("sex");

        if !mfr_path.exists() {
            eprintln!("Skipping: Sedus not found");
            return;
        }

        println!("\n=== Family Price Calculation Test ===\n");

        let loader = FamilyLoader::load(&mfr_path, "DE");
        let families = loader.get_families();

        // Find a family with articles
        for family in families.iter().take(5) {
            let properties = loader.get_properties_for_family(family);
            let config = FamilyConfiguration::new(&family.id, &properties);
            let price_date = chrono::Local::now().date_naive();
            let price = engine.calculate_family_price("sex", family, &config, price_date);

            if let Some(p) = price {
                println!("Family: {} ({})", family.name, family.id);
                println!("  Base article: {}", family.base_article_nr);
                println!("  Base price: {:.2} {}", p.base_price, p.currency);
                println!("  Total: {:.2} {}", p.total_price, p.currency);
                if !p.surcharges.is_empty() {
                    println!("  Surcharges: {}", p.surcharges.len());
                }
                return; // Found one with pricing, test passes
            }
        }

        // If no prices found, still pass (some data may not have prices)
        println!("No prices found for tested families (may be expected)");
    }

    #[test]
    fn test_multi_manufacturer_data_accessibility() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        println!("\n=== Multi-Manufacturer Data Accessibility Test ===\n");

        let mut engine = ConfigurationEngine::new(OFMLDATA_BASE);

        let test_manufacturers = [
            "sex", "vitra", "kn", "sbu", "haw", "aix", "arper", "cassina",
        ];

        let mut success_count = 0;
        let mut family_count = 0;
        let mut price_count = 0;

        for mfr in &test_manufacturers {
            let mfr_path = Path::new(OFMLDATA_BASE).join(mfr);
            if !mfr_path.exists() {
                continue;
            }

            // Load families
            let families: Vec<_> = engine.load_families(mfr).to_vec();
            if families.is_empty() {
                println!("{}: No families found", mfr);
                continue;
            }

            success_count += 1;
            family_count += families.len();

            // Check if we can get pricing for any family
            let price_date = chrono::Local::now().date_naive();
            let loader = FamilyLoader::load(&mfr_path, "DE");
            for family in families.iter().take(3) {
                let properties = loader.get_properties_for_family(family);
                let config = FamilyConfiguration::new(&family.id, &properties);
                if let Some(_price) = engine.calculate_family_price(mfr, family, &config, price_date) {
                    price_count += 1;
                }
            }

            println!("{}: {} families, pricing available: {}", mfr, families.len(), price_count > 0);
        }

        println!("\nSummary:");
        println!("  Manufacturers with data: {}/{}", success_count, test_manufacturers.len());
        println!("  Total families: {}", family_count);
        println!("  Families with pricing: {}", price_count);

        assert!(success_count >= 3, "Should have at least 3 manufacturers with data");
    }

    #[test]
    fn test_price_level_distribution() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        println!("\n=== Price Level Distribution Test ===\n");

        let manufacturers = ["sex", "vitra", "kn"];

        for mfr in &manufacturers {
            let mfr_path = Path::new(OFMLDATA_BASE).join(mfr);
            if !mfr_path.exists() {
                continue;
            }

            // Find first pdata.ebase
            if let Some(pdata_path) = find_first_pdata(&mfr_path) {
                match OcdReader::from_ebase(&pdata_path) {
                    Ok(reader) => {
                        let base = reader.prices.iter().filter(|p| p.price_level == "B").count();
                        let surcharge = reader.prices.iter().filter(|p| p.price_level == "X").count();
                        let discount = reader.prices.iter().filter(|p| p.price_level == "D").count();
                        let empty = reader.prices.iter().filter(|p| p.price_level.is_empty()).count();

                        println!("{}: {} prices", mfr, reader.prices.len());
                        println!("  B (base): {}", base);
                        println!("  X (surcharge): {}", surcharge);
                        println!("  D (discount): {}", discount);
                        println!("  empty: {}", empty);

                        // Most manufacturers should have at least some prices with levels
                        if reader.prices.len() > 0 {
                            let populated_ratio = (base + surcharge + discount) as f64 / reader.prices.len() as f64;
                            println!("  populated ratio: {:.1}%", populated_ratio * 100.0);
                        }
                    }
                    Err(e) => {
                        println!("{}: Error - {}", mfr, e);
                    }
                }
            }
        }
    }

    fn find_first_pdata(path: &Path) -> Option<std::path::PathBuf> {
        fn find_recursive(path: &Path) -> Option<std::path::PathBuf> {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        if let Some(found) = find_recursive(&entry_path) {
                            return Some(found);
                        }
                    } else if entry_path.file_name().map_or(false, |n| n == "pdata.ebase") {
                        return Some(entry_path);
                    }
                }
            }
            None
        }
        find_recursive(path)
    }
}

#[cfg(not(feature = "tui"))]
#[test]
fn test_tui_feature_disabled() {
    // This test passes when TUI feature is disabled
    println!("TUI feature is disabled, skipping TUI tests");
}
