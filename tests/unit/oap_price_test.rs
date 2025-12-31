//! Unit tests for price lookup (T016)

use chrono::NaiveDate;
use rust_decimal::Decimal;

use ofml_interpreter::oap::price::{PriceLookup, PriceQuery};
use ofml_interpreter::oap::{
    format_german_price, format_german_price_with_currency, PriceResult, Surcharge,
};

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
fn test_price_lookup_returns_result() {
    let lookup = PriceLookup::new();
    let query = PriceQuery::new(
        "vitra".to_string(),
        "48-123-456".to_string(),
        "H720_D1200".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    let result = lookup.lookup(&query);
    assert!(result.is_ok());

    let price = result.unwrap();
    assert!(price.base_price > Decimal::ZERO);
    assert_eq!(price.currency, "EUR");
}

#[test]
fn test_price_lookup_with_variant_includes_surcharge() {
    let lookup = PriceLookup::new();
    let query = PriceQuery::new(
        "vitra".to_string(),
        "ViTable".to_string(),
        "H720_D1200_white".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    let result = lookup.lookup(&query);
    assert!(result.is_ok());

    let price = result.unwrap();
    // Non-empty variant code should trigger surcharge in mock
    assert!(!price.surcharges.is_empty());
}

#[test]
fn test_price_lookup_empty_variant_no_surcharge() {
    let lookup = PriceLookup::new();
    let query = PriceQuery::new(
        "vitra".to_string(),
        "SimpleArticle".to_string(),
        "".to_string(), // Empty variant
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
    );

    let result = lookup.lookup(&query);
    assert!(result.is_ok());

    let price = result.unwrap();
    // Empty variant code means no surcharges in mock
    assert!(price.surcharges.is_empty());
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
fn test_format_german_price_basic() {
    let price = Decimal::new(4500, 2); // 45.00
    assert_eq!(format_german_price(price), "45,00");
}

#[test]
fn test_format_german_price_with_thousands() {
    let price = Decimal::new(123456, 2); // 1234.56
    assert_eq!(format_german_price(price), "1.234,56");
}

#[test]
fn test_format_german_price_large() {
    let price = Decimal::new(12345678900, 2); // 123456789.00
    assert_eq!(format_german_price(price), "123.456.789,00");
}

#[test]
fn test_format_german_price_with_currency() {
    let price = Decimal::new(123456, 2); // 1234.56
    assert_eq!(
        format_german_price_with_currency(price, "EUR"),
        "1.234,56 EUR"
    );
}

#[test]
fn test_format_german_price_small() {
    let price = Decimal::new(99, 2); // 0.99
    assert_eq!(format_german_price(price), "0,99");
}

#[test]
fn test_format_german_price_zero() {
    let price = Decimal::new(0, 2);
    assert_eq!(format_german_price(price), "0,00");
}

#[test]
fn test_surcharge_structure() {
    let surcharge = Surcharge {
        name: "Materialaufpreis".to_string(),
        amount: Decimal::new(5000, 2), // 50.00
        is_percentage: false,
    };

    assert_eq!(surcharge.name, "Materialaufpreis");
    assert_eq!(surcharge.amount, Decimal::new(5000, 2));
    assert!(!surcharge.is_percentage);
}

#[test]
fn test_price_result_dates() {
    let lookup_date = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
    let valid_from = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let valid_to = Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());

    let price = PriceResult::new(
        Decimal::new(10000, 2),
        vec![],
        "EUR".to_string(),
        lookup_date,
        valid_from,
        valid_to,
    );

    assert_eq!(price.price_date, lookup_date);
    assert_eq!(price.valid_from, valid_from);
    assert_eq!(
        price.valid_to,
        Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
    );
}
