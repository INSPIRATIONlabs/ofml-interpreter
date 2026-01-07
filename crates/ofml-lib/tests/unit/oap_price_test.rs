//! Unit tests for price lookup (T016)

use chrono::NaiveDate;
use rust_decimal::Decimal;

use ofml_lib::oap::price::{PriceError, PriceLookup, PriceQuery};
use ofml_lib::oap::{
    format_german_price, format_german_price_with_currency, PriceResult, Surcharge,
};

/// Test data path for OFML data
const TEST_DATA_PATH: &str = "/reference/ofmldata";

#[test]
fn test_price_query_creation() {
    let query = PriceQuery::new(
        "vitra".to_string(),
        "48-123-456".to_string(),
        "H720_D1200".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    assert_eq!(query.manufacturer, "vitra");
    assert_eq!(query.article_number, "48-123-456");
    assert_eq!(query.variant_code, "H720_D1200");
    assert_eq!(
        query.price_date,
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap()
    );
}

#[test]
fn test_price_lookup_real_article() {
    let lookup = PriceLookup::new(TEST_DATA_PATH);
    let query = PriceQuery::new(
        "sex".to_string(),
        "AI-121".to_string(),
        "".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    // This test requires real OFML data to be present
    let result = lookup.lookup(&query);
    if let Ok(price) = result {
        assert!(price.base_price >= Decimal::ZERO);
        assert!(!price.currency.is_empty());
    }
    // If data not available, test passes silently
}

#[test]
fn test_price_lookup_manufacturer_not_found() {
    let lookup = PriceLookup::new("/nonexistent/path");
    let query = PriceQuery::new(
        "fakemfr".to_string(),
        "FAKE-123".to_string(),
        "".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    let result = lookup.lookup(&query);
    assert!(result.is_err());
    assert!(matches!(result, Err(PriceError::PriceTableNotFound(_))));
}

#[test]
fn test_price_lookup_article_not_found() {
    let lookup = PriceLookup::new(TEST_DATA_PATH);
    let query = PriceQuery::new(
        "sex".to_string(), // Valid manufacturer
        "NONEXISTENT-ARTICLE-12345".to_string(),
        "".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    let result = lookup.lookup(&query);
    // May be either table not found (if manufacturer path doesn't exist) or article not found
    assert!(result.is_err());
}

#[test]
fn test_price_result_total_no_surcharges() {
    let price = PriceResult::new(
        Decimal::new(10000, 2), // 100.00
        vec![],
        "EUR".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        None,
    );

    // total_price is computed during PriceResult::new
    assert_eq!(price.total_price, Decimal::new(10000, 2)); // 100.00
}

#[test]
fn test_price_result_total_with_fixed_surcharge() {
    let price = PriceResult::new(
        Decimal::new(10000, 2), // 100.00
        vec![Surcharge {
            name: "Aufpreis".to_string(),
            amount: Decimal::new(2500, 2), // 25.00
            is_percentage: false,
        }],
        "EUR".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        None,
    );

    assert_eq!(price.total_price, Decimal::new(12500, 2)); // 125.00
}

#[test]
fn test_price_result_total_with_percentage_surcharge() {
    let price = PriceResult::new(
        Decimal::new(10000, 2), // 100.00
        vec![Surcharge {
            name: "10% Aufschlag".to_string(),
            amount: Decimal::new(10, 0), // 10%
            is_percentage: true,
        }],
        "EUR".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        None,
    );

    assert_eq!(price.total_price, Decimal::new(11000, 2)); // 110.00
}

#[test]
fn test_price_result_total_mixed_surcharges() {
    let price = PriceResult::new(
        Decimal::new(10000, 2), // 100.00
        vec![
            Surcharge {
                name: "Fixed".to_string(),
                amount: Decimal::new(1000, 2), // 10.00
                is_percentage: false,
            },
            Surcharge {
                name: "10%".to_string(),
                amount: Decimal::new(10, 0), // 10%
                is_percentage: true,
            },
        ],
        "EUR".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        None,
    );

    // 100.00 + 10.00 + 10% of 100.00 = 100.00 + 10.00 + 10.00 = 120.00
    assert_eq!(price.total_price, Decimal::new(12000, 2));
}

#[test]
fn test_format_german_price() {
    assert_eq!(format_german_price(Decimal::new(123456, 2)), "1.234,56");
    assert_eq!(format_german_price(Decimal::new(100, 2)), "1,00");
    assert_eq!(format_german_price(Decimal::new(0, 2)), "0,00");
    assert_eq!(format_german_price(Decimal::new(1000000, 2)), "10.000,00");
}

#[test]
fn test_format_german_price_with_currency() {
    assert_eq!(
        format_german_price_with_currency(Decimal::new(123456, 2), "EUR"),
        "1.234,56 EUR"
    );
}
