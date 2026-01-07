//! Test Vitra Classic product family loading
//!
//! Ensures that Vitra Classic (EA chairs) can be properly loaded with prices and properties.

use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::FamilyLoader;
use ofml_lib::oap::ocd::OcdReader;
use std::path::Path;

const OFMLDATA_BASE: &str = "/reference/ofmldata";
const VITRA_CLASSIC_EBASE: &str = "/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase";
const VITRA_CLASSIC_PATH: &str = "/reference/ofmldata/vitra/classic";

fn vitra_classic_exists() -> bool {
    Path::new(VITRA_CLASSIC_EBASE).exists()
}

#[test]
fn test_vitra_classic_ocd_articles() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(VITRA_CLASSIC_EBASE)).unwrap();

    println!("\n=== Vitra Classic OCD Data ===");
    println!("Articles: {}", reader.articles.len());
    println!("Prices: {}", reader.prices.len());
    println!("Property classes mapping: {} entries", reader.article_prop_classes.len());

    // Check that we have articles
    assert!(!reader.articles.is_empty(), "No articles found in Vitra classic");

    // Print first 10 articles
    println!("\nFirst 10 articles:");
    for art in reader.articles.iter().take(10) {
        println!("  {} (type: {}, series: {})", art.article_nr, art.art_type, art.series);
    }

    // Check for EA 117 related articles
    let ea_articles: Vec<_> = reader.articles
        .iter()
        .filter(|a| a.article_nr.contains("117") || a.article_nr.starts_with("412"))
        .collect();

    println!("\nEA 117 related articles (containing '117' or starting with '412'):");
    for art in &ea_articles {
        println!("  {} (type: {}, series: {})", art.article_nr, art.art_type, art.series);
    }

    // Check specifically for article 41236400 (from screenshot error)
    let art_41236400 = reader.articles.iter().find(|a| a.article_nr == "41236400");
    println!("\n=== Checking for 41236400 ===");
    match art_41236400 {
        Some(art) => println!("FOUND: {} type={} series={}", art.article_nr, art.art_type, art.series),
        None => {
            println!("NOT FOUND as '41236400'");
            // Check variations
            for art in reader.articles.iter() {
                if art.article_nr.contains("41236") {
                    println!("  Similar: {} type={} series={}", art.article_nr, art.art_type, art.series.escape_debug());
                }
            }
        }
    }

    // Check articles with valid series (to understand filtering)
    let valid_series_count = reader.articles.iter()
        .filter(|a| !a.series.chars().any(|c| c.is_control())
            && a.series.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
            && !a.series.is_empty())
        .count();
    println!("\nArticles with valid series: {} / {}", valid_series_count, reader.articles.len());
}

#[test]
fn test_vitra_classic_prices() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(VITRA_CLASSIC_EBASE)).unwrap();

    // Check that we have prices
    let base_prices: Vec<_> = reader.prices
        .iter()
        .filter(|p| p.price_level == "B")
        .collect();

    println!("\n=== Vitra Classic Prices ===");
    println!("Total prices: {}", reader.prices.len());
    println!("Base prices (level B): {}", base_prices.len());

    // Print first 5 base prices
    println!("\nFirst 5 base prices:");
    for p in base_prices.iter().take(5) {
        println!("  article={} level={} amount={} var_cond={}",
            p.article_nr, p.price_level, p.price, p.var_cond);
    }

    // Look for article 41236400 prices
    let art_prices: Vec<_> = reader.prices
        .iter()
        .filter(|p| p.article_nr.contains("4123640"))
        .collect();

    println!("\nPrices containing article 41236400:");
    if art_prices.is_empty() {
        println!("  NONE FOUND!");

        // Show unique article numbers from prices
        let mut unique_arts: std::collections::HashSet<_> = reader.prices.iter().map(|p| &p.article_nr).collect();
        let unique_arts: Vec<_> = unique_arts.drain().take(20).collect();
        println!("\nUnique article numbers in prices (first 20):");
        for art in unique_arts {
            println!("  {}", art);
        }
    } else {
        for p in &art_prices {
            println!("  level={} amount={} var_cond={}", p.price_level, p.price, p.var_cond);
        }
    }
}

#[test]
fn test_vitra_classic_families() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let loader = FamilyLoader::load(Path::new(VITRA_CLASSIC_PATH), "DE");
    let families = loader.get_families();

    println!("\n=== Vitra Classic Families ===");
    println!("Total families: {}", families.len());

    // Find EA 117 family
    let ea117_family = families.iter().find(|f| {
        f.name.to_lowercase().contains("ea 117") ||
        f.name.to_lowercase().contains("ea117") ||
        f.base_article_nr.contains("4123640")
    });

    match ea117_family {
        Some(family) => {
            println!("\nFound EA 117 family:");
            println!("  ID: {}", family.id);
            println!("  Name: {}", family.name);
            println!("  Base article: {}", family.base_article_nr);
            println!("  Article NRs: {:?}", family.article_nrs);
            println!("  Prop classes: {:?}", family.prop_classes);

            // Get properties for this family
            let properties = loader.get_properties_for_family(family);
            println!("  Properties: {} found", properties.len());
            for prop in properties.iter().take(10) {
                println!("    - {} ({:?}): {} options", prop.key, prop.prop_type, prop.options.len());
            }
        }
        None => {
            println!("EA 117 family NOT FOUND!");
            println!("\nAvailable families:");
            for family in families.iter().take(20) {
                println!("  {} - {} (article: {})", family.id, family.name, family.base_article_nr);
            }
        }
    }
}

#[test]
fn test_vitra_classic_property_classes() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(VITRA_CLASSIC_EBASE)).unwrap();

    println!("\n=== Vitra Classic Property Classes ===");

    // Check property classes for article 41236400
    let prop_classes = reader.article_prop_classes.get("41236400");

    println!("Property classes for 41236400:");
    match prop_classes {
        Some(classes) => {
            for pc in classes {
                println!("  prop_class={}", pc);
            }
        }
        None => {
            println!("  NONE FOUND!");

            // Show all property class article numbers to understand the format
            println!("\nArticle numbers with property classes (first 20):");
            for (i, art) in reader.article_prop_classes.keys().take(20).enumerate() {
                println!("  {}: {}", i, art);
            }
        }
    }
}

#[test]
fn test_vitra_classic_price_calculation() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let engine = ConfigurationEngine::new(OFMLDATA_BASE);
    let loader = FamilyLoader::load(Path::new(VITRA_CLASSIC_PATH), "DE");
    let families = loader.get_families();
    let price_date = chrono::Local::now().date_naive();

    println!("\n=== Vitra Classic Price Calculation ===");

    // Test first 10 families with prices
    let mut families_with_prices = 0;
    for family in families.iter().take(10) {
        let properties = loader.get_properties_for_family(family);
        let config = ofml_lib::oap::families::FamilyConfiguration::new(&family.id, &properties);

        if let Some(price) = engine.calculate_family_price("vitra", family, &config, price_date) {
            families_with_prices += 1;
            println!("  {} ({}): base={}, total={}, surcharges={}",
                family.name, family.base_article_nr,
                price.base_price, price.total_price, price.surcharges.len());
        } else {
            println!("  {} ({}): NO PRICE", family.name, family.base_article_nr);
        }
    }

    println!("\nFamilies with prices: {}/{}", families_with_prices, families.len().min(10));
}
