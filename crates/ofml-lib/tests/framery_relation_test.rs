//! Test for FRMR_ONE pricing using ocd_relation rules

use std::collections::HashMap;
use std::path::Path;

use ofml_lib::oap::ocd_relation::RelationRuleReader;

#[test]
fn test_frmr_one_relation_rules_load() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    println!("\n=== FRMR_ONE Relation Rules ===\n");
    assert!(reader.has_pricing_rules(), "Should have pricing rules");
    println!("Loaded {} var_cond rules", reader.varcond_rules.len());

    for rule in &reader.varcond_rules {
        println!("  VarCond: {} -> {:?}", rule.var_cond, rule.condition);
    }
}

#[test]
fn test_frmr_one_relation_evaluation_seat() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    // Test: ONE with adjustable seat enabled
    let mut props = HashMap::new();
    props.insert("M_ARTNO".to_string(), "ONE".to_string());
    props.insert("M_SEAT".to_string(), "YES".to_string());

    let matched = reader.evaluate(&props);
    println!("\n=== ONE with M_SEAT=YES ===");
    println!("Matched var_conds: {:?}", matched);

    assert!(matched.contains(&"PG_ADJUSTABLE_SEAT".to_string()),
        "Should match PG_ADJUSTABLE_SEAT when M_ARTNO=ONE and M_SEAT=YES");
}

#[test]
fn test_frmr_one_relation_evaluation_lan() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    // Test: LAN enabled
    let mut props = HashMap::new();
    props.insert("M_ARTNO".to_string(), "ONE_PREMIUM".to_string());
    props.insert("M_LAN".to_string(), "YES".to_string());

    let matched = reader.evaluate(&props);
    println!("\n=== ONE_PREMIUM with M_LAN=YES ===");
    println!("Matched var_conds: {:?}", matched);

    assert!(matched.contains(&"PG_LAN".to_string()),
        "Should match PG_LAN when M_LAN=YES");
}

#[test]
fn test_frmr_one_relation_evaluation_non_standard_colors() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    // Test: ONE with non-standard exterior color
    let mut props = HashMap::new();
    props.insert("M_ARTNO".to_string(), "ONE".to_string());
    props.insert("M_EXTERIOR".to_string(), "CUSTOM_BLUE".to_string()); // Not in standard list

    let matched = reader.evaluate(&props);
    println!("\n=== ONE with custom exterior color ===");
    println!("Matched var_conds: {:?}", matched);

    assert!(matched.contains(&"PG_EXTERIOR_PANEL_OPTION_COLOR".to_string()),
        "Should match PG_EXTERIOR_PANEL_OPTION_COLOR for non-standard color");
}

#[test]
fn test_frmr_one_relation_evaluation_standard_colors() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    // Test: ONE with STANDARD exterior color (RAL9016MAT is in the standard list)
    let mut props = HashMap::new();
    props.insert("M_ARTNO".to_string(), "ONE".to_string());
    props.insert("M_EXTERIOR".to_string(), "RAL9016MAT".to_string()); // Standard

    let matched = reader.evaluate(&props);
    println!("\n=== ONE with standard exterior color ===");
    println!("Matched var_conds: {:?}", matched);

    // Should NOT have the surcharge for standard colors
    assert!(!matched.contains(&"PG_EXTERIOR_PANEL_OPTION_COLOR".to_string()),
        "Should NOT match PG_EXTERIOR_PANEL_OPTION_COLOR for standard RAL9016MAT color");
}

#[test]
fn test_frmr_one_relation_evaluation_multiple_options() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    // Test: ONE with multiple upgrades
    let mut props = HashMap::new();
    props.insert("M_ARTNO".to_string(), "ONE".to_string());
    props.insert("M_SEAT".to_string(), "YES".to_string());
    props.insert("M_LAN".to_string(), "YES".to_string());
    props.insert("M_FILTER".to_string(), "YES".to_string());
    props.insert("M_SERVICE_KIT".to_string(), "YES".to_string());
    props.insert("M_EXTERIOR".to_string(), "CUSTOM_GREEN".to_string()); // Non-standard

    let matched = reader.evaluate(&props);
    println!("\n=== ONE with multiple upgrades ===");
    println!("Matched var_conds: {:?}", matched);

    // Should have all the expected surcharges
    assert!(matched.contains(&"PG_ADJUSTABLE_SEAT".to_string()), "Should have seat surcharge");
    assert!(matched.contains(&"PG_LAN".to_string()), "Should have LAN surcharge");
    assert!(matched.contains(&"PG_ACTIVATED_CARBON_FILTER".to_string()), "Should have filter surcharge");
    assert!(matched.contains(&"PG_SERVICE_KIT".to_string()), "Should have service kit surcharge");
    assert!(matched.contains(&"PG_EXTERIOR_PANEL_OPTION_COLOR".to_string()), "Should have exterior color surcharge");

    println!("Total surcharges matched: {}", matched.len());
}

#[test]
fn test_frmr_one_relation_evaluation_one_lounge() {
    let path = Path::new("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase");
    if !path.exists() {
        eprintln!("Skipping: path not found");
        return;
    }

    let reader = RelationRuleReader::from_ebase(path).expect("Should load relations");

    // Test: ONE_LOUNGE with adjustable table
    let mut props = HashMap::new();
    props.insert("M_ARTNO".to_string(), "ONE_LOUNGE".to_string());
    props.insert("M_TABLE".to_string(), "ADJUSTABLE_TABLE".to_string());

    let matched = reader.evaluate(&props);
    println!("\n=== ONE_LOUNGE with adjustable table ===");
    println!("Matched var_conds: {:?}", matched);

    assert!(matched.contains(&"PG_ADJUSTABLE_TABLE".to_string()),
        "Should match PG_ADJUSTABLE_TABLE for ONE_LOUNGE with adjustable table");

    // Seat surcharge should NOT apply to ONE_LOUNGE (condition is M_ARTNO = 'ONE')
    props.insert("M_SEAT".to_string(), "YES".to_string());
    let matched2 = reader.evaluate(&props);
    assert!(!matched2.contains(&"PG_ADJUSTABLE_SEAT".to_string()),
        "Should NOT match PG_ADJUSTABLE_SEAT for ONE_LOUNGE (only for ONE)");
}
