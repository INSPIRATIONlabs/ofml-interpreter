//! Unit tests for OCD features (tax, rounding, packaging, composites)

use std::path::Path;

use ofml_lib::oap::ocd_properties::{load_manufacturer_properties, clear_property_cache};

#[test]
fn test_get_data_version() {
    clear_property_cache();

    // Test with a manufacturer that should have version data
    let path = Path::new("/reference/ofmldata/vitra");
    if !path.exists() {
        return; // Skip if test data not available
    }

    let reader = load_manufacturer_properties(path);
    let version = reader.get_data_version();

    // Version may or may not exist, but the method should work
    if let Some(v) = version {
        assert!(!v.data_version.is_empty() || !v.format_version.is_empty());
    }
}

#[test]
fn test_get_packaging() {
    clear_property_cache();

    // Test with a manufacturer that should have packaging data
    let path = Path::new("/reference/ofmldata/sedus");
    if !path.exists() {
        return; // Skip if test data not available
    }

    let reader = load_manufacturer_properties(path);

    // Try to find any article with packaging
    for (article_nr, _) in reader.packaging.iter().take(1) {
        let packaging = reader.get_packaging(article_nr);
        assert!(!packaging.is_empty(), "Should have packaging for {}", article_nr);

        let pkg = packaging[0];
        // Verify package has some dimension data
        assert!(
            pkg.width >= 0.0 && pkg.depth >= 0.0 && pkg.height >= 0.0,
            "Package dimensions should be non-negative"
        );
    }
}

#[test]
fn test_data_validity_warning() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/vitra");
    if !path.exists() {
        return; // Skip if test data not available
    }

    let reader = load_manufacturer_properties(path);

    // Test with a date far in the future
    let future_date = chrono::NaiveDate::from_ymd_opt(2099, 1, 1).unwrap();
    let warning = reader.get_data_validity_warning(future_date);
    // A warning about expired data may or may not be returned depending on the data
    // Just verify the method runs without error
    let _ = warning;

    // Test with current date
    let today = chrono::Local::now().date_naive();
    let _ = reader.get_data_validity_warning(today);
}

#[test]
fn test_format_variant_code() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/sedus");
    if !path.exists() {
        return; // Skip if test data not available
    }

    let reader = load_manufacturer_properties(path);

    // Test variant code formatting
    let input = "STOFF_2G3";
    let formatted = reader.format_variant_code(input);

    // The formatted code should at least contain the property values
    assert!(formatted.contains("STOFF") || formatted.contains("2G3"));
}

#[test]
fn test_get_composite() {
    clear_property_cache();

    // Vitra has composite products
    let path = Path::new("/reference/ofmldata/vitra");
    if !path.exists() {
        return; // Skip if test data not available
    }

    let reader = load_manufacturer_properties(path);

    // Just verify the method works
    // Check if any composite exists
    if !reader.composites.is_empty() {
        let (composite_id, _) = reader.composites.iter().next().unwrap();
        let composite = reader.get_composite(composite_id);
        assert!(composite.is_some(), "Should find composite {}", composite_id);
    }
}

#[test]
fn test_get_bill_of_items() {
    clear_property_cache();

    let path = Path::new("/reference/ofmldata/vitra");
    if !path.exists() {
        return; // Skip if test data not available
    }

    let reader = load_manufacturer_properties(path);

    // Check if any bill of items exists
    if !reader.bill_of_items.is_empty() {
        let (composite_id, _) = reader.bill_of_items.iter().next().unwrap();
        let items = reader.get_bill_of_items(composite_id);
        assert!(!items.is_empty(), "Should have items for {}", composite_id);

        // Verify item structure
        let item = items[0];
        assert!(!item.item_id.is_empty(), "Item ID should not be empty");
        assert!(item.quantity >= 0.0, "Quantity should be non-negative");
    }
}

#[test]
fn test_get_tax_scheme() {
    clear_property_cache();

    // Try to find a manufacturer with tax data
    for mfr in &["vitra", "sedus", "bisley"] {
        let path = Path::new("/reference/ofmldata").join(mfr);
        if !path.exists() {
            continue;
        }

        let reader = load_manufacturer_properties(&path);

        // Check if any tax scheme exists
        if !reader.tax_schemes.is_empty() {
            let (scheme_id, _) = reader.tax_schemes.iter().next().unwrap();
            let scheme = reader.get_tax_scheme(scheme_id);
            assert!(scheme.is_some(), "Should find tax scheme {}", scheme_id);

            let s = scheme.unwrap();
            assert!(!s.tax_id.is_empty(), "Tax ID should not be empty");
            return; // Test passed
        }
    }

    // If no tax data found in any manufacturer, that's okay
    // The feature just won't be used
}

#[test]
fn test_get_rounding_rules() {
    clear_property_cache();

    for mfr in &["vitra", "sedus", "bisley"] {
        let path = Path::new("/reference/ofmldata").join(mfr);
        if !path.exists() {
            continue;
        }

        let reader = load_manufacturer_properties(&path);

        // Check if any rounding rules exist
        if !reader.rounding_rules.is_empty() {
            let (rule_id, _) = reader.rounding_rules.iter().next().unwrap();
            let rules = reader.get_rounding_rules(rule_id);

            // If rules exist, verify they have valid data
            if !rules.is_empty() {
                let rule = rules[0];
                // Precision should be a reasonable value
                assert!(rule.precision <= 10, "Precision should be reasonable");
                return; // Test passed
            }
        }
    }
}
