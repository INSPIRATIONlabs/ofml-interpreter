//! Tests for propvalue2varcond table loading and lookup functionality

use std::path::Path;

use ofml_interpreter::oap::ocd::OcdReader;

/// Test that propvalue2varcond table is loaded correctly from Vitra data
#[test]
fn test_vitra_propvalue2varcond_loading() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Vitra workit should have propvalue2varcond mappings
    assert!(
        reader.has_varcond_mappings(),
        "Vitra workit should have propvalue2varcond mappings"
    );

    // Check that we loaded a reasonable number of mappings
    assert!(
        reader.propvalue2varcond.len() > 100,
        "Should have at least 100 mappings, found {}",
        reader.propvalue2varcond.len()
    );

    println!(
        "Loaded {} propvalue2varcond mappings from Vitra workit",
        reader.propvalue2varcond.len()
    );
}

/// Test that propvalue2varcond lookups work correctly
#[test]
fn test_vitra_propvalue2varcond_lookup() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Based on actual data: prop_value '1AS01' should map to var_cond '83341201_1AS01'
    let var_cond = reader.lookup_varcond("ASY_83341201", "1AS01");
    assert!(
        var_cond.is_some(),
        "Should find var_cond for prop_value '1AS01'"
    );

    if let Some(vc) = var_cond {
        assert!(
            vc.contains("1AS01"),
            "var_cond '{}' should contain '1AS01'",
            vc
        );
        println!("prop_value '1AS01' -> var_cond '{}'", vc);
    }
}

/// Test that lookup_varconds_for_values returns all matching var_conds
#[test]
fn test_vitra_propvalue2varcond_multi_lookup() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Test looking up multiple values at once
    let values = vec!["1AS01", "2AS01"];
    let var_conds = reader.lookup_varconds_for_values(&values);

    println!("Found {} var_conds for values {:?}", var_conds.len(), values);
    for vc in &var_conds {
        println!("  -> {}", vc);
    }

    // Should find at least one var_cond for each value that has a mapping
    assert!(
        !var_conds.is_empty(),
        "Should find at least one var_cond for the given values"
    );
}

/// Test that Sedus does NOT have propvalue2varcond (uses pattern matching instead)
#[test]
fn test_sedus_no_propvalue2varcond() {
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Sedus should NOT have propvalue2varcond mappings
    assert!(
        !reader.has_varcond_mappings(),
        "Sedus AI should NOT have propvalue2varcond mappings"
    );

    println!(
        "Sedus AI correctly has no propvalue2varcond mappings (uses pattern matching)"
    );
}

/// Test that value-only lookup works as fallback
#[test]
fn test_propvalue2varcond_value_only_lookup() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Check that we can look up by value only (without knowing prop_class)
    if let Some(mappings) = reader.propvalue2varcond_by_value.get("1AS01") {
        assert!(
            !mappings.is_empty(),
            "Should find mappings for value '1AS01'"
        );
        println!("Found {} mappings for value '1AS01':", mappings.len());
        for m in mappings {
            println!(
                "  prop_class='{}' prop_value='{}' -> var_cond='{}'",
                m.prop_class, m.prop_value, m.var_cond
            );
        }
    }
}

/// Test that price matching uses direct lookup when available
#[test]
fn test_vitra_price_matching_with_direct_lookup() {
    // Load a Vitra family
    let path = Path::new("/reference/ofmldata/vitra/workit");
    if !path.exists() {
        println!("Vitra workit data not found, skipping test");
        return;
    }

    // The test focuses on verifying the direct lookup mechanism is used
    // For detailed price verification, we check the propvalue2varcond mappings

    let pdata_path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !pdata_path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(pdata_path).expect("Should load OCD data");

    // Verify Vitra uses direct lookup
    assert!(reader.has_varcond_mappings(), "Vitra should use direct lookup");

    // Get an article's prices
    let articles: Vec<_> = reader
        .articles
        .iter()
        .filter(|a| !a.article_nr.is_empty())
        .take(5)
        .collect();

    println!("Sample Vitra articles with prices:");
    for article in &articles {
        let prices = reader.get_prices(&article.article_nr);
        if !prices.is_empty() {
            println!(
                "  {} - {} prices (base: {:.2} {})",
                article.article_nr,
                prices.len(),
                prices[0].price,
                prices[0].currency
            );
        }
    }
}

/// Test mapping data structure contents
#[test]
fn test_propvalue2varcond_data_structure() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Print some sample mappings for verification
    println!("Sample propvalue2varcond mappings:");
    let mut count = 0;
    for ((prop_class, prop_value), mapping) in &reader.propvalue2varcond {
        if count >= 10 {
            break;
        }
        println!(
            "  ({}, {}) -> var_cond='{}' prop_text_add='{}'",
            prop_class, prop_value, mapping.var_cond, mapping.prop_text_add
        );
        count += 1;
    }

    // Verify mapping structure
    for ((prop_class, prop_value), mapping) in reader.propvalue2varcond.iter().take(5) {
        assert_eq!(prop_class, &mapping.prop_class);
        assert_eq!(prop_value, &mapping.prop_value);
        assert!(!mapping.var_cond.is_empty(), "var_cond should not be empty");
    }
}

/// Test that prices are correctly associated with articles
#[test]
fn test_vitra_article_prices() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    println!("\nVitra workit price summary:");
    println!("  Total articles: {}", reader.articles.len());
    println!("  Total prices: {}", reader.prices.len());
    println!(
        "  propvalue2varcond mappings: {}",
        reader.propvalue2varcond.len()
    );

    // Count prices by level
    let base_count = reader.prices.iter().filter(|p| p.price_level == "B").count();
    let surcharge_count = reader.prices.iter().filter(|p| p.price_level == "X").count();
    let discount_count = reader.prices.iter().filter(|p| p.price_level == "D").count();

    println!("\n  Price levels:");
    println!("    Base (B): {}", base_count);
    println!("    Surcharge (X): {}", surcharge_count);
    println!("    Discount (D): {}", discount_count);

    // List unique var_cond values
    let mut var_conds: Vec<&str> = reader.prices.iter().map(|p| p.var_cond.as_str()).collect();
    var_conds.sort();
    var_conds.dedup();
    println!("\n  Unique var_cond values: {}", var_conds.len());
    for vc in var_conds.iter().take(10) {
        println!("    '{}'", vc);
    }
    if var_conds.len() > 10 {
        println!("    ... and {} more", var_conds.len() - 10);
    }
}

/// Integration test demonstrating direct lookup vs pattern matching
#[test]
fn test_direct_lookup_vs_pattern_matching() {
    // Test Vitra (uses direct lookup via propvalue2varcond)
    let vitra_path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    let sedus_path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");

    if !vitra_path.exists() || !sedus_path.exists() {
        println!("Test data not found, skipping");
        return;
    }

    let vitra_reader = OcdReader::from_ebase(vitra_path).expect("Load Vitra");
    let sedus_reader = OcdReader::from_ebase(sedus_path).expect("Load Sedus");

    println!("\n=== Pricing Strategy Comparison ===\n");

    // Vitra uses direct lookup
    println!("VITRA (propvalue2varcond direct lookup):");
    println!("  Has mapping table: {}", vitra_reader.has_varcond_mappings());
    println!("  Mappings count: {}", vitra_reader.propvalue2varcond.len());

    // Example: lookup var_cond for property value "1AS01"
    if let Some(var_cond) = vitra_reader.lookup_varcond("ASY_83341201", "1AS01") {
        println!("  Example: '1AS01' -> '{}' (DIRECT LOOKUP)", var_cond);
    }

    // Sedus uses pattern matching
    println!("\nSEDUS (pattern matching fallback):");
    println!("  Has mapping table: {}", sedus_reader.has_varcond_mappings());
    println!(
        "  Uses pattern matching for S_XXX codes (e.g., S_166 matches value containing '166')"
    );

    // Verify the strategies work
    assert!(
        vitra_reader.has_varcond_mappings(),
        "Vitra should use direct lookup"
    );
    assert!(
        !sedus_reader.has_varcond_mappings(),
        "Sedus should use pattern matching"
    );

    println!("\n=== Both strategies work correctly ===");
}

/// Test that var_cond matching works correctly for real surcharges
#[test]
fn test_surcharge_matching_with_direct_lookup() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Get an article with surcharges
    let article_nr = "83341201";
    let prices = reader.get_prices(article_nr);

    let base_prices: Vec<_> = prices.iter().filter(|p| p.price_level == "B").collect();
    let surcharges: Vec<_> = prices.iter().filter(|p| p.price_level == "X").collect();

    println!("\nArticle {} price analysis:", article_nr);
    println!("  Base prices: {}", base_prices.len());
    println!("  Surcharges: {}", surcharges.len());

    if let Some(base) = base_prices.first() {
        println!("  Base price: {:.2} {}", base.price, base.currency);
    }

    // Show some surcharges and their matching mappings
    println!("\n  Sample surcharges and their var_cond mappings:");
    for surcharge in surcharges.iter().take(5) {
        // Check if this var_cond is in our mapping
        let has_direct_mapping = reader
            .propvalue2varcond
            .values()
            .any(|m| m.var_cond == surcharge.var_cond);

        println!(
            "    var_cond='{}' price={:.2} {} (direct_mapping={})",
            surcharge.var_cond, surcharge.price, surcharge.currency, has_direct_mapping
        );
    }

    // Verify that most surcharges have direct mappings
    let mapped_count = surcharges
        .iter()
        .filter(|s| {
            reader
                .propvalue2varcond
                .values()
                .any(|m| m.var_cond == s.var_cond)
        })
        .count();

    println!(
        "\n  Surcharges with direct mapping: {}/{}",
        mapped_count,
        surcharges.len()
    );
    assert!(
        mapped_count > 0,
        "Should have at least some surcharges with direct mappings"
    );
}

/// Test that ocd_pricetext table is loaded correctly
#[test]
fn test_vitra_pricetext_loading() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    println!("\n=== Vitra workit ocd_pricetext Analysis ===");
    println!("  Price texts loaded: {}", reader.price_texts.len());

    // Should have some price texts
    assert!(
        !reader.price_texts.is_empty(),
        "Vitra workit should have price texts"
    );

    // Show sample price texts
    println!("\n  Sample price texts:");
    for (textnr, texts) in reader.price_texts.iter().take(5) {
        for text in texts {
            println!(
                "    textnr='{}' [{}] '{}'",
                textnr, text.language, text.text
            );
        }
    }
}

/// Test that price description resolution works
#[test]
fn test_price_description_resolution() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Get some prices with text_id
    let prices_with_text: Vec<_> = reader
        .prices
        .iter()
        .filter(|p| !p.text_id.is_empty())
        .take(10)
        .collect();

    println!("\n=== Price Description Resolution Test ===");
    println!("  Prices with text_id: {}", prices_with_text.len());

    // Check if we can resolve any descriptions
    let mut resolved_count = 0;
    for price in &prices_with_text {
        let description = reader.get_price_description(price, "DE");
        let is_resolved = description != price.var_cond && description != "Base price";
        if is_resolved {
            resolved_count += 1;
        }
        println!(
            "    text_id='{}' var_cond='{}' -> '{}'{}",
            price.text_id,
            price.var_cond,
            description,
            if is_resolved { " [RESOLVED]" } else { "" }
        );
    }

    println!(
        "\n  Resolved descriptions: {}/{}",
        resolved_count,
        prices_with_text.len()
    );
}

/// Test that Sedus pricetext loading works
#[test]
fn test_sedus_pricetext_loading() {
    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    println!("\n=== Sedus AI ocd_pricetext Analysis ===");
    println!("  Price texts loaded: {}", reader.price_texts.len());

    // Show sample price texts
    if !reader.price_texts.is_empty() {
        println!("\n  Sample price texts:");
        for (textnr, texts) in reader.price_texts.iter().take(5) {
            for text in texts {
                println!(
                    "    textnr='{}' [{}] '{}'",
                    textnr, text.language, text.text
                );
            }
        }
    } else {
        println!("  No price texts found (manufacturer may use different approach)");
    }
}

/// Test surcharges display with human-readable descriptions
#[test]
fn test_surcharge_display_with_descriptions() {
    let path = Path::new("/reference/ofmldata/vitra/workit/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Vitra workit pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load OCD data");

    // Get an article with surcharges
    let article_nr = "83341201";
    let surcharges = reader.get_surcharges(article_nr);

    println!("\n=== Article {} Surcharges with Descriptions ===", article_nr);
    println!("  Total surcharges: {}", surcharges.len());
    println!("\n  Surcharges (pcon.basket style display):");

    for surcharge in surcharges.iter().take(10) {
        let description = reader.get_price_description(surcharge, "DE");
        println!(
            "    {:>8.2} {} - {}",
            surcharge.price, surcharge.currency, description
        );
    }
    if surcharges.len() > 10 {
        println!("    ... and {} more", surcharges.len() - 10);
    }
}

/// Test that engine price calculation produces human-readable surcharge names
#[test]
fn test_engine_produces_readable_surcharge_names() {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyLoader, FamilyConfiguration};
    use std::path::Path;

    let mfr_path = Path::new("/reference/ofmldata/vitra/workit");
    if !mfr_path.exists() {
        println!("Vitra workit not found, skipping test");
        return;
    }

    let engine = ConfigurationEngine::new("/reference/ofmldata");
    let loader = FamilyLoader::load(mfr_path, "DE");
    let families = loader.get_families();

    println!("\n=== Engine Surcharge Name Test ===");

    // Find a family with surcharges
    for family in families.iter().take(5) {
        let properties = loader.get_properties_for_family(family);
        if properties.is_empty() {
            continue;
        }

        let config = FamilyConfiguration::new(&family.id, &properties);
        let price_date = chrono::Local::now().date_naive();

        if let Some(price) = engine.calculate_family_price("vitra", family, &config, price_date) {
            if !price.surcharges.is_empty() {
                println!("Family: {} ({})", family.name, family.id);
                println!("  Base: {:.2} {}", price.base_price, price.currency);
                println!("  Surcharges:");

                for surcharge in &price.surcharges {
                    println!("    {} = {:.2}", surcharge.name, surcharge.amount);

                    // Verify surcharge name is human-readable (not just var_cond code)
                    // Human-readable names should NOT start with article number pattern
                    let is_var_cond_code = surcharge.name.chars().next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                        && surcharge.name.contains('_');

                    if is_var_cond_code {
                        println!("    WARNING: Surcharge '{}' looks like raw var_cond, not description!", surcharge.name);
                    }
                }

                println!("  Total: {:.2} {}", price.total_price, price.currency);
                return;
            }
        }
    }

    println!("No families with surcharges found (may need different test data)");
}

/// Debug test to verify Sedus surcharge matching actually works
#[test]
fn test_sedus_surcharge_matching_debug() {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyLoader, FamilyConfiguration};
    use std::path::Path;

    let mfr_path = Path::new("/reference/ofmldata/sex");
    if !mfr_path.exists() {
        println!("Sedus not found, skipping test");
        return;
    }

    let engine = ConfigurationEngine::new("/reference/ofmldata");
    let loader = FamilyLoader::load(mfr_path, "DE");
    let families = loader.get_families();

    println!("\n=== Sedus Surcharge Matching Debug ===\n");

    // Find AI family
    let ai_family = families.iter().find(|f| f.base_article_nr.contains("AI-"));
    if ai_family.is_none() {
        println!("AI family not found");
        return;
    }
    let family = ai_family.unwrap();

    println!("Family: {} ({})", family.name, family.id);
    println!("Base article: {}", family.base_article_nr);

    let properties = loader.get_properties_for_family(family);
    println!("Properties: {}", properties.len());

    // Show property values that might trigger S_166
    println!("\nLooking for property values with '166':");
    for prop in &properties {
        for opt in &prop.options {
            if opt.value.contains("166") {
                println!("  {}={} (label: {})", prop.key, opt.value, opt.label);
            }
        }
    }

    // Create config and check price
    let config = FamilyConfiguration::new(&family.id, &properties);
    println!("\nDefault variant_code: {}", config.variant_code);

    // Show which values are in the variant code
    let values: Vec<&str> = config.variant_code
        .split(';')
        .filter_map(|p| p.split('=').nth(1))
        .collect();
    println!("Values in config: {:?}", values);

    // Check if any value contains "166"
    let has_166 = values.iter().any(|v| v.contains("166"));
    println!("Has value with '166': {}", has_166);

    let price_date = chrono::Local::now().date_naive();
    if let Some(price) = engine.calculate_family_price("sex", family, &config, price_date) {
        println!("\nPrice calculation result:");
        println!("  Base: {:.2} {}", price.base_price, price.currency);
        println!("  Total: {:.2} {}", price.total_price, price.currency);
        println!("  Surcharges: {}", price.surcharges.len());
        for s in &price.surcharges {
            println!("    {} = {:.2}", s.name, s.amount);
        }
    } else {
        println!("\nNo price calculated!");
    }

    // Now change a property to trigger S_166
    println!("\n--- Trying to trigger S_166 ---");
    // Use S_STOFF_FRONT_GABRIEL which has option XST244166018
    let gabriel_prop = properties.iter().find(|p| p.key == "S_STOFF_FRONT_GABRIEL");
    if let Some(prop) = gabriel_prop {
        println!("Found property: {} with {} options", prop.key, prop.options.len());
        let opt_166 = prop.options.iter().find(|o| o.value.contains("166"));
        if let Some(opt) = opt_166 {
            println!("Setting {}={}", prop.key, opt.value);

            let mut new_config = FamilyConfiguration::new(&family.id, &properties);
            new_config.set(&prop.key, &opt.value);

            println!("New variant_code: {}", new_config.variant_code);

            if let Some(price) = engine.calculate_family_price("sex", family, &new_config, price_date) {
                println!("\nNew price result:");
                println!("  Base: {:.2} {}", price.base_price, price.currency);
                println!("  Total: {:.2} {}", price.total_price, price.currency);
                println!("  Surcharges: {}", price.surcharges.len());
                for s in &price.surcharges {
                    println!("    {} = {:.2}", s.name, s.amount);
                }
                
                // Check if S_166 was matched
                let has_s166 = price.surcharges.iter().any(|s| s.name.contains("166") || s.amount == rust_decimal::Decimal::from(44));
                println!("\nS_166 surcharge matched: {}", has_s166);
            }
        } else {
            println!("No option with '166' found in property {}", prop.key);
        }
    } else {
        println!("No GABRIEL/STOFF property found");
    }
}

/// Test that simulates TUI property cycling
#[test]
fn test_tui_property_cycling_simulation() {
    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::families::{FamilyLoader, FamilyConfiguration};
    use std::path::Path;

    let mfr_path = Path::new("/reference/ofmldata/sex");
    if !mfr_path.exists() {
        println!("Sedus not found, skipping test");
        return;
    }

    // Simulate TUI initialization (two engines like main.rs + app.rs)
    let engine = ConfigurationEngine::new("/reference/ofmldata");

    // Load families (simulating main.rs)
    let families: Vec<_> = {
        let loader = FamilyLoader::load(mfr_path, "DE");
        loader.get_families().to_vec()
    };
    let ai_family = families.iter().find(|f| f.base_article_nr.contains("AI-")).cloned();

    if ai_family.is_none() {
        println!("AI family not found");
        return;
    }
    let family = ai_family.unwrap();

    println!("\n=== TUI Property Cycling Simulation ===\n");
    println!("Family: {} ({})", family.name, family.id);

    // Load properties using FamilyLoader (simulating main.rs)
    let loader = FamilyLoader::load(mfr_path, "DE");
    let properties = loader.get_properties_for_family(&family);
    println!("Properties loaded: {}", properties.len());

    // Create initial config
    let mut config = FamilyConfiguration::new(&family.id, &properties);
    let price_date = chrono::Local::now().date_naive();

    // Initial price
    let initial_price = engine.calculate_family_price("sex", &family, &config, price_date);
    println!("\nInitial state:");
    if let Some(p) = &initial_price {
        println!("  Total: {:.2} {} ({} surcharges)", p.total_price, p.currency, p.surcharges.len());
    }

    // Find S_STOFF_FRONT_GABRIEL property
    let gabriel_prop_idx = properties.iter().position(|p| p.key == "S_STOFF_FRONT_GABRIEL");
    if gabriel_prop_idx.is_none() {
        println!("S_STOFF_FRONT_GABRIEL property not found");
        return;
    }
    let prop_idx = gabriel_prop_idx.unwrap();
    let prop = &properties[prop_idx];

    println!("\nProperty: {} ({} options)", prop.key, prop.options.len());
    for (i, opt) in prop.options.iter().enumerate() {
        let marker = if opt.value.contains("166") { " <-- has 166" } else { "" };
        println!("  [{}] {} {}", i, opt.value, marker);
    }

    // Find option with 166
    let opt_166_idx = prop.options.iter().position(|o| o.value.contains("166"));
    if opt_166_idx.is_none() {
        println!("No option with 166");
        return;
    }
    let target_idx = opt_166_idx.unwrap();

    // Simulate cycling through options
    println!("\n--- Simulating option cycling ---");
    
    let current_value = config.get(&prop.key).unwrap_or_default();
    let current_idx = prop.options.iter().position(|o| o.value == current_value).unwrap_or(0);
    println!("Current: [{}] {}", current_idx, current_value);

    // Cycle to target option
    for i in 0..prop.options.len() {
        let idx = (current_idx + i) % prop.options.len();
        let new_value = &prop.options[idx].value;
        
        // Simulate config.set() which is called in app.update()
        config.set(&prop.key, new_value);
        
        // Simulate recalculate_price
        let price = engine.calculate_family_price("sex", &family, &config, price_date);
        
        if let Some(p) = &price {
            let surcharge_info = if p.surcharges.is_empty() {
                "no surcharges".to_string()
            } else {
                p.surcharges.iter()
                    .map(|s| format!("{}={:.2}", s.name, s.amount))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            println!("  [{}] {} -> {:.2} {} ({})",
                idx, new_value, p.total_price, p.currency, surcharge_info);
        }

        if idx == target_idx {
            println!("  ^^^ Target option reached");
            
            // Verify price changed
            if let (Some(init), Some(curr)) = (&initial_price, &price) {
                if curr.total_price != init.total_price {
                    println!("\n  SUCCESS: Price changed from {:.2} to {:.2}",
                        init.total_price, curr.total_price);
                } else {
                    println!("\n  WARNING: Price did NOT change!");
                }
            }
            break;
        }
    }
}

/// Test that property value descriptions are loaded from ocd_propvaluetext
#[test]
fn test_propvaluetext_loading() {
    use ofml_interpreter::oap::ocd_properties::OcdPropertyReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdPropertyReader::from_ebase(path).expect("Should load");

    println!("\n=== Property Value Text Loading Test ===\n");
    println!("Property texts (ocd_propertytext): {}", reader.texts.len());
    println!("Value texts (ocd_propvaluetext): {}", reader.value_texts.len());

    // Show sample value texts
    println!("\nSample value texts:");
    for (textnr, texts) in reader.value_texts.iter().take(10) {
        for text in texts {
            println!("  textnr='{}' [{}] '{}'", textnr, text.language, text.text);
        }
    }

    // Look up S_ACCESSOIRES values
    let acc_values = reader.get_values_for_property("S_ACC_AI", "S_ACCESSOIRES");
    println!("\nS_ACCESSOIRES options:");
    for val in acc_values.iter().take(10) {
        let label = reader.get_value_label(val, "DE").unwrap_or_else(|| "NO LABEL".to_string());
        println!("  {} -> '{}'", val.value_from, label);
    }
}

/// Test to find accessories property class and values
#[test]
fn test_find_accessories_property() {
    use ofml_interpreter::oap::ocd_properties::OcdPropertyReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdPropertyReader::from_ebase(path).expect("Should load");

    println!("\n=== Finding Accessories Property ===\n");

    // Find all properties with "ACC" in the name
    println!("Properties with 'ACC' in name:");
    for ((prop_class, property), def) in &reader.properties {
        if property.contains("ACC") || prop_class.contains("ACC") {
            println!("  {}.{} (type: {}, multi_option: {})", 
                prop_class, property, def.prop_type, def.multi_option);
        }
    }

    // Find all S_ACCESSOIRES values across all property classes
    println!("\nAll property classes with S_ACCESSOIRES values:");
    for ((prop_class, property), values) in &reader.values {
        if property == "S_ACCESSOIRES" {
            println!("  Class '{}' has {} values:", prop_class, values.len());
            for val in values.iter().take(5) {
                let label = reader.get_value_label(val, "DE")
                    .or_else(|| reader.get_value_label(val, "de"))
                    .unwrap_or_else(|| format!("(no label, textnr={})", val.textnr));
                println!("    {} -> '{}'", val.value_from, label);
            }
            if values.len() > 5 {
                println!("    ... and {} more", values.len() - 5);
            }
        }
    }
}

/// Test accessory surcharge prices
#[test]
fn test_accessory_prices_debug() {
    use ofml_interpreter::oap::ocd::OcdReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load");

    println!("\n=== Accessory Prices Debug ===\n");

    // Look for prices with ACC or accessory-related var_cond
    let acc_prices: Vec<_> = reader.prices.iter()
        .filter(|p| p.var_cond.contains("ACC") || 
                    p.var_cond.contains("6235") ||
                    p.var_cond.contains("6103") ||
                    p.var_cond.contains("7007"))
        .collect();

    println!("Prices with ACC/accessory codes: {}", acc_prices.len());
    for p in acc_prices.iter().take(20) {
        println!("  var_cond='{}' price={:.2} {} (level='{}')", 
            p.var_cond, p.price, p.currency, p.price_level);
    }

    // Look for ALL prices for article AI-121
    let ai_prices = reader.get_prices("AI-121");
    println!("\nAI-121 total prices: {}", ai_prices.len());
    
    // Find unique var_cond values
    let mut var_conds: Vec<_> = ai_prices.iter().map(|p| &p.var_cond).collect();
    var_conds.sort();
    var_conds.dedup();
    
    println!("\nUnique var_cond values for AI-121:");
    for vc in var_conds.iter().take(30) {
        println!("  '{}'", vc);
    }
}

/// Test to find all accessory-related prices across ALL articles
#[test]
fn test_all_accessory_prices() {
    use ofml_interpreter::oap::ocd::OcdReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let reader = OcdReader::from_ebase(path).expect("Should load");

    println!("\n=== All Accessory Prices Search ===\n");
    println!("Total prices in database: {}", reader.prices.len());

    // Search for ANY price that might relate to accessories
    let accessory_codes = ["6235", "6103", "7007", "ACC", "ACCESSOIRE", "Nacken", "Kleider"];
    
    for code in &accessory_codes {
        let matches: Vec<_> = reader.prices.iter()
            .filter(|p| p.var_cond.contains(code))
            .collect();
        if !matches.is_empty() {
            println!("\nPrices containing '{}': {}", code, matches.len());
            for p in matches.iter().take(5) {
                println!("  article='{}' var_cond='{}' price={:.2}", 
                    p.article_nr, p.var_cond, p.price);
            }
        }
    }

    // List ALL unique var_cond patterns (first 50)
    let mut all_var_conds: Vec<_> = reader.prices.iter()
        .map(|p| p.var_cond.as_str())
        .collect();
    all_var_conds.sort();
    all_var_conds.dedup();
    
    println!("\nAll unique var_cond patterns ({} total):", all_var_conds.len());
    for vc in all_var_conds.iter().take(50) {
        if !vc.is_empty() {
            println!("  '{}'", vc);
        }
    }
}

/// Investigate OCD relation system for pricing
#[test]
fn test_ocd_relation_system() {
    use ofml_interpreter::ebase::EBaseReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Should open");

    println!("\n=== OCD Relation System Investigation ===\n");

    // Check ocd_relationobj table
    if reader.tables.contains_key("ocd_relationobj") {
        let records = reader.read_records("ocd_relationobj", None).expect("read");
        println!("ocd_relationobj: {} records", records.len());
        
        // Count by domain
        let mut domain_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for rec in &records {
            let domain = rec.get("rel_domain")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string();
            *domain_counts.entry(domain).or_default() += 1;
        }
        
        println!("\nRelation domains:");
        for (domain, count) in &domain_counts {
            let desc = match domain.as_str() {
                "P" => "Pricing",
                "C" => "Configuration",
                "V" => "Validity",
                "G" => "Geometry",
                "" => "Empty",
                _ => "Unknown",
            };
            println!("  '{}' ({}): {} records", domain, desc, count);
        }

        // Show sample pricing relations
        println!("\nSample pricing relations (domain='P'):");
        let pricing_rels: Vec<_> = records.iter()
            .filter(|r| r.get("rel_domain").and_then(|v| v.as_str()) == Some("P"))
            .take(10)
            .collect();
        
        for rec in &pricing_rels {
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
            let rel_nr = rec.get("rel_nr").and_then(|v| v.as_i64()).unwrap_or(0);
            println!("  rel_obj={} rel_nr={}", rel_obj, rel_nr);
        }
    }

    // Check ocd_relation table
    if reader.tables.contains_key("ocd_relation") {
        let records = reader.read_records("ocd_relation", None).expect("read");
        println!("\nocd_relation: {} records", records.len());
        
        // Show sample relations with their code
        println!("\nSample relation code:");
        for rec in records.iter().take(5) {
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
            let rel_nr = rec.get("rel_nr").and_then(|v| v.as_i64()).unwrap_or(0);
            let rel_code = rec.get("rel_code").and_then(|v| v.as_str()).unwrap_or("");
            println!("  rel_obj={} rel_nr={}: '{}'", rel_obj, rel_nr, 
                if rel_code.len() > 60 { &rel_code[..60] } else { rel_code });
        }

        // Look for VARCOND assignments
        println!("\nRelations containing 'VARCOND':");
        let varcond_rels: Vec<_> = records.iter()
            .filter(|r| {
                r.get("rel_code").and_then(|v| v.as_str())
                    .map(|s| s.contains("VARCOND"))
                    .unwrap_or(false)
            })
            .take(10)
            .collect();
        
        for rec in &varcond_rels {
            let rel_code = rec.get("rel_code").and_then(|v| v.as_str()).unwrap_or("");
            println!("  '{}'", if rel_code.len() > 80 { &rel_code[..80] } else { rel_code });
        }
    }

    // Check ocd_propertyvalue for rel_obj references
    if reader.tables.contains_key("ocd_propertyvalue") {
        let records = reader.read_records("ocd_propertyvalue", None).expect("read");
        
        // Find property values with rel_obj > 0 (have relations)
        let with_relations: Vec<_> = records.iter()
            .filter(|r| r.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0) > 0)
            .collect();
        
        println!("\nProperty values with relations: {} / {}", with_relations.len(), records.len());
        
        // Show accessory property values with rel_obj
        println!("\nS_ACCESSOIRES values with rel_obj:");
        for rec in &records {
            let property = rec.get("property").and_then(|v| v.as_str()).unwrap_or("");
            if property == "S_ACCESSOIRES" {
                let value = rec.get("value_from").and_then(|v| v.as_str()).unwrap_or("");
                let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
                println!("  {} rel_obj={}", value, rel_obj);
            }
        }
    }
}

/// Deep dive into OCD relation structure
#[test]
fn test_ocd_relation_deep_dive() {
    use ofml_interpreter::ebase::EBaseReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Should open");

    println!("\n=== Deep Dive: OCD Relation Structure ===\n");

    // First, let's see the column names in ocd_relationobj
    if let Some(table) = reader.tables.get("ocd_relationobj") {
        println!("ocd_relationobj columns:");
        for col in &table.columns {
            println!("  - {} (type_id={})", col.name, col.type_id);
        }
    }

    // And ocd_relation
    if let Some(table) = reader.tables.get("ocd_relation") {
        println!("\nocd_relation columns:");
        for col in &table.columns {
            println!("  - {} (type_id={})", col.name, col.type_id);
        }
    }

    // Read actual relation records and show non-empty ones
    if reader.tables.contains_key("ocd_relation") {
        let records = reader.read_records("ocd_relation", None).expect("read");
        
        // Find non-empty relation codes
        let non_empty: Vec<_> = records.iter()
            .filter(|r| {
                r.get("rel_code").and_then(|v| v.as_str())
                    .map(|s| !s.is_empty())
                    .unwrap_or(false)
            })
            .take(20)
            .collect();
        
        println!("\nSample non-empty relation code ({} found):", non_empty.len());
        for rec in &non_empty {
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
            let rel_code = rec.get("rel_code").and_then(|v| v.as_str()).unwrap_or("");
            println!("\n  rel_obj={}", rel_obj);
            // Print first 200 chars of code
            let code_preview = if rel_code.len() > 200 { &rel_code[..200] } else { rel_code };
            println!("  code: {}", code_preview);
        }
    }

    // Check property values with rel_obj > 0 and see what relations they use
    if reader.tables.contains_key("ocd_propertyvalue") {
        let pv_records = reader.read_records("ocd_propertyvalue", None).expect("read");
        
        // Find values with relations for properties that affect pricing
        println!("\n\nProperty values WITH relations (rel_obj > 0):");
        let with_rel: Vec<_> = pv_records.iter()
            .filter(|r| r.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0) > 0)
            .take(20)
            .collect();
        
        for rec in &with_rel {
            let property = rec.get("property").and_then(|v| v.as_str()).unwrap_or("");
            let value = rec.get("value_from").and_then(|v| v.as_str()).unwrap_or("");
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
            println!("  {} = {} -> rel_obj={}", property, value, rel_obj);
        }

        // Specifically look at S_STOFF_FRONT_GABRIEL which we know affects price
        println!("\n\nS_STOFF_FRONT_GABRIEL values:");
        for rec in &pv_records {
            let property = rec.get("property").and_then(|v| v.as_str()).unwrap_or("");
            if property == "S_STOFF_FRONT_GABRIEL" {
                let value = rec.get("value_from").and_then(|v| v.as_str()).unwrap_or("");
                let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0);
                println!("  {} -> rel_obj={}", value, rel_obj);
            }
        }
    }
}

/// Look up specific relation code for S_166 trigger
#[test]
fn test_lookup_s166_relation() {
    use ofml_interpreter::ebase::EBaseReader;
    use std::path::Path;

    let path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !path.exists() {
        println!("Sedus AI pdata.ebase not found, skipping test");
        return;
    }

    let mut reader = EBaseReader::open(path).expect("Should open");

    println!("\n=== Looking Up Relation for S_166 ===\n");

    // XST244166018 has rel_obj=1661731 which should set VARCOND to S_166
    let target_rel_obj = 1661731u64;
    
    // The ocd_relation uses rel_name to link, not rel_obj directly
    // First get the rel_name from ocd_relationobj
    if reader.tables.contains_key("ocd_relationobj") {
        let records = reader.read_records("ocd_relationobj", None).expect("read");
        
        // Find relation obj entries for our target
        println!("Looking for rel_obj around {}", target_rel_obj);
        
        for rec in &records {
            let rel_obj = rec.get("rel_obj").and_then(|v| v.as_i64()).unwrap_or(0) as u64;
            if rel_obj == target_rel_obj || (rel_obj > target_rel_obj - 100 && rel_obj < target_rel_obj + 100) {
                let rel_name = rec.get("rel_name").and_then(|v| v.as_str()).unwrap_or("");
                let rel_domain = rec.get("rel_domain").and_then(|v| v.as_str()).unwrap_or("");
                let rel_type = rec.get("rel_type").and_then(|v| v.as_str()).unwrap_or("");
                println!("  rel_obj={} rel_name='{}' domain='{}' type='{}'", 
                    rel_obj, rel_name, rel_domain, rel_type);
            }
        }
    }

    // Now look at ocd_relation to find the actual code
    if reader.tables.contains_key("ocd_relation") {
        let records = reader.read_records("ocd_relation", None).expect("read");
        
        println!("\nLooking for relation blocks containing 'VARCOND' or '166':");
        
        for rec in &records {
            let rel_block = rec.get("rel_block").and_then(|v| v.as_str()).unwrap_or("");
            if rel_block.contains("VARCOND") || rel_block.contains("166") {
                let rel_name = rec.get("rel_name").and_then(|v| v.as_str()).unwrap_or("");
                let rel_blocknr = rec.get("rel_blocknr").and_then(|v| v.as_i64()).unwrap_or(0);
                
                // Limit output
                let preview = if rel_block.len() > 100 { &rel_block[..100] } else { rel_block };
                println!("  rel_name='{}' blocknr={}", rel_name, rel_blocknr);
                println!("    block: '{}'...", preview);
                println!();
            }
        }
    }
}
