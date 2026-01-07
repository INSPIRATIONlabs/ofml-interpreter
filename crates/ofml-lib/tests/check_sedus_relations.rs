//! Quick check for Sedus relation rules

use std::path::Path;
use ofml_lib::oap::ocd_relation::RelationRuleReader;
use ofml_lib::oap::ocd::find_pdata_files;

#[test]
fn check_sedus_all_relations() {
    let mfr_path = Path::new("/reference/ofmldata/sex");
    if !mfr_path.exists() {
        println!("Sedus not found");
        return;
    }

    let pdata_files = find_pdata_files(mfr_path);
    println!("Found {} pdata files for Sedus", pdata_files.len());

    let mut files_with_rules = 0;
    for pdata_path in &pdata_files {
        if let Some(reader) = RelationRuleReader::from_ebase(pdata_path) {
            if reader.has_pricing_rules() {
                files_with_rules += 1;
                println!("\n=== {} ===", pdata_path.display());
                println!("  {} pricing rules", reader.varcond_rules.len());
            }
        }
    }
    println!("\nTotal files with pricing rules: {}", files_with_rules);
}
