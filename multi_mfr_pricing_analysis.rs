// Comprehensive Multi-Manufacturer Pricing Analysis
// Analyzes pricing patterns across multiple manufacturers to find common structures

use ofml_interpreter::oap::ocd::{OcdReader, OcdPrice};
use std::collections::{HashSet};
use std::error::Error;
use std::path::Path;

struct ManufacturerAnalysis {
    manufacturer: String,
    series: String,
    db_path: String,
    base_price_count: usize,
    surcharge_count: usize,
    base_var_cond_patterns: Vec<String>,
    surcharge_var_cond_samples: Vec<String>,
    sample_base_prices: Vec<OcdPrice>,
    sample_surcharges: Vec<OcdPrice>,
    has_propvalue2varcond: bool,
    propvalue2varcond_count: usize,
    property_class_count: usize,
    article_count: usize,
}

fn analyze_manufacturer(mfr: &str, series: &str, db_path: &str) -> Result<ManufacturerAnalysis, Box<dyn Error>> {
    println!("\n{}", "=".repeat(80));
    println!("ANALYZING: {}/{}", mfr, series);
    println!("Path: {}", db_path);
    println!("{}", "=".repeat(80));

    let path = Path::new(db_path);
    if !path.exists() {
        return Err(format!("Database file does not exist: {}", db_path).into());
    }

    let ocd = OcdReader::from_ebase(path)?;

    // 1. Analyze ocd_price table
    println!("\n--- OCD_PRICE TABLE ---");
    println!("Total price records: {}", ocd.prices.len());

    let mut base_count = 0;
    let mut surcharge_count = 0;
    let mut base_var_conds = HashSet::new();
    let mut surcharge_var_conds = HashSet::new();
    let mut sample_base_prices = Vec::new();
    let mut sample_surcharges = Vec::new();

    for price in &ocd.prices {
        if price.price_level == "B" {
            base_count += 1;
            base_var_conds.insert(price.var_cond.clone());
            if sample_base_prices.len() < 10 {
                sample_base_prices.push(price.clone());
            }
        } else if price.price_level == "X" {
            surcharge_count += 1;
            surcharge_var_conds.insert(price.var_cond.clone());
            if sample_surcharges.len() < 10 {
                sample_surcharges.push(price.clone());
            }
        }
    }

    println!("Base prices (level B): {}", base_count);
    println!("Surcharges (level X): {}", surcharge_count);

    println!("\nBase price var_cond patterns: {:?}", base_var_conds);
    if !sample_base_prices.is_empty() {
        println!("\nSample base prices (first 5):");
        for (i, bp) in sample_base_prices.iter().take(5).enumerate() {
            println!("  {}. article_nr='{}', var_cond='{}', price={} {}",
                i+1, bp.article_nr, bp.var_cond, bp.price, bp.currency);
        }
    }

    let surcharge_samples: Vec<String> = surcharge_var_conds.iter().take(10).cloned().collect();
    println!("\nSurcharge var_cond samples (first 10): {:?}", surcharge_samples);
    if !sample_surcharges.is_empty() {
        println!("\nSample surcharges (first 5):");
        for (i, sc) in sample_surcharges.iter().take(5).enumerate() {
            println!("  {}. article_nr='{}', var_cond='{}', price={} {}",
                i+1, sc.article_nr, sc.var_cond, sc.price, sc.currency);
        }
    }

    // 2. Check ocd_propvalue2varcond table
    println!("\n--- OCD_PROPVALUE2VARCOND TABLE ---");
    let has_p2v = ocd.has_varcond_mappings();
    let p2v_count = ocd.propvalue2varcond.len();
    if has_p2v {
        println!("Table EXISTS! Record count: {}", p2v_count);
        // Sample some mappings
        println!("Sample mappings (first 5):");
        for (i, ((prop_class, prop_value), mapping)) in ocd.propvalue2varcond.iter().take(5).enumerate() {
            println!("  {}. prop_class='{}', prop_value='{}' -> var_cond='{}'",
                i+1, prop_class, prop_value, mapping.var_cond);
        }
    } else {
        println!("Table does NOT exist or is empty");
    }

    // 3. Check ocd_propertyclass
    println!("\n--- OCD_PROPERTYCLASS TABLE ---");
    let propclass_count = ocd.article_prop_classes.len();
    println!("Total article property class mappings: {}", propclass_count);

    if !ocd.article_prop_classes.is_empty() {
        println!("Sample property class mappings (first 5):");
        for (i, (article_nr, classes)) in ocd.article_prop_classes.iter().take(5).enumerate() {
            println!("  {}. article_nr='{}', classes={:?}",
                i+1, article_nr, classes);
        }
    }

    // 4. Check articles
    println!("\n--- OCD_ARTICLE TABLE ---");
    println!("Total articles: {}", ocd.articles.len());

    if !ocd.articles.is_empty() {
        println!("Sample articles (first 5):");
        for (i, art) in ocd.articles.iter().take(5).enumerate() {
            println!("  {}. article_nr='{}', manufacturer='{}', series='{}'",
                i+1, art.article_nr, art.manufacturer, art.series);
        }
    }

    Ok(ManufacturerAnalysis {
        manufacturer: mfr.to_string(),
        series: series.to_string(),
        db_path: db_path.to_string(),
        base_price_count: base_count,
        surcharge_count,
        base_var_cond_patterns: base_var_conds.into_iter().collect(),
        surcharge_var_cond_samples: surcharge_samples,
        sample_base_prices,
        sample_surcharges,
        has_propvalue2varcond: has_p2v,
        propvalue2varcond_count: p2v_count,
        property_class_count: propclass_count,
        article_count: ocd.articles.len(),
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("COMPREHENSIVE MULTI-MANUFACTURER PRICING INVESTIGATION");
    println!("{}", "=".repeat(120));

    // Define manufacturers to investigate
    let manufacturers = vec![
        // Framery - phone booths (should be thousands of EUR)
        ("framery", "frmr_one", "/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase"),
        ("framery", "frmr_2q", "/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase"),
        ("framery", "frmr_q", "/reference/ofmldata/framery/frmr_q/ANY/1/db/pdata.ebase"),

        // FAST - wall decorations
        ("fast", "kr", "/reference/ofmldata/fast/kr/DE/1/db/pdata.ebase"),
        ("fast", "wkm", "/reference/ofmldata/fast/wkm/DE/1/db/pdata.ebase"),

        // Sedus - office chairs
        ("sedus", "ai", "/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase"),
        ("sedus", "sf", "/reference/ofmldata/sex/sf/DE/1/db/pdata.ebase"),

        // Bisley - file cabinets
        ("bisley", "sf", "/reference/ofmldata/bisley/sf/ANY/1/db/pdata.ebase"),
        ("bisley", "pf", "/reference/ofmldata/bisley/pf/ANY/1/db/pdata.ebase"),

        // Arper - furniture
        ("arper", "catifa46", "/reference/ofmldata/arper/catifa46/EU/1/db/pdata.ebase"),
        ("arper", "saya", "/reference/ofmldata/arper/saya/EU/1/db/pdata.ebase"),
    ];

    let mut results = Vec::new();

    for (mfr, series, db_path) in manufacturers {
        match analyze_manufacturer(mfr, series, db_path) {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("\nERROR analyzing {}/{}: {}", mfr, series, e),
        }
    }

    // Generate comparison table
    println!("\n{}", "=".repeat(120));
    println!("PRICING PATTERNS COMPARISON TABLE");
    println!("{}", "=".repeat(120));
    println!("{:<15} {:<20} {:<30} {:<30} {:<20}",
        "Manufacturer", "Series", "Base Pattern", "Surch Pattern", "propvalue2varcond?");
    println!("{}", "-".repeat(120));

    for r in &results {
        let base_pattern = if r.base_var_cond_patterns.is_empty() {
            "NONE".to_string()
        } else {
            r.base_var_cond_patterns.iter()
                .take(3)
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let surch_pattern = if r.surcharge_var_cond_samples.is_empty() {
            "NONE".to_string()
        } else {
            r.surcharge_var_cond_samples.iter()
                .take(2)
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let has_p2v = if r.has_propvalue2varcond {
            format!("YES ({})", r.propvalue2varcond_count)
        } else {
            "NO".to_string()
        };

        println!("{:<15} {:<20} {:<30} {:<30} {:<20}",
            r.manufacturer, r.series, base_pattern, surch_pattern, has_p2v);
    }

    // Print summary
    println!("\n{}", "=".repeat(120));
    println!("KEY FINDINGS:");
    println!("{}", "=".repeat(120));

    println!("\n1. BASE PRICE PATTERNS:");
    let mut all_base_patterns: HashSet<String> = HashSet::new();
    for r in &results {
        for pattern in &r.base_var_cond_patterns {
            all_base_patterns.insert(pattern.clone());
        }
    }
    println!("   All unique base var_cond values across all manufacturers:");
    for pattern in &all_base_patterns {
        println!("     - '{}'", pattern);
    }

    println!("\n2. PROPVALUE2VARCOND TABLE:");
    let manufacturers_with_p2v: Vec<_> = results.iter()
        .filter(|r| r.has_propvalue2varcond)
        .map(|r| format!("{}/{} ({} records)", r.manufacturer, r.series, r.propvalue2varcond_count))
        .collect();
    if manufacturers_with_p2v.is_empty() {
        println!("   NO manufacturers have this table");
    } else {
        println!("   Manufacturers with table:");
        for mfr in manufacturers_with_p2v {
            println!("     - {}", mfr);
        }
    }

    println!("\n3. HIGHEST BASE PRICES (looking for Framery's thousands of EUR):");
    for r in &results {
        if !r.sample_base_prices.is_empty() {
            let max_price = r.sample_base_prices.iter()
                .map(|p| p.price)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let min_price = r.sample_base_prices.iter()
                .map(|p| p.price)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            println!("   {}/{}: min={}, max={} (from {} base prices)",
                r.manufacturer, r.series, min_price, max_price, r.base_price_count);
        }
    }

    println!("\n4. DATA COMPLETENESS:");
    println!("   {:<15} {:<20} {:<12} {:<12} {:<12} {:<12}",
        "Manufacturer", "Series", "Articles", "Base Prices", "Surcharges", "Prop Classes");
    println!("   {}", "-".repeat(88));
    for r in &results {
        println!("   {:<15} {:<20} {:<12} {:<12} {:<12} {:<12}",
            r.manufacturer, r.series, r.article_count,
            r.base_price_count, r.surcharge_count, r.property_class_count);
    }

    Ok(())
}
