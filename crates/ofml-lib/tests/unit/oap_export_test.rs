//! Unit tests for ExportData serialization (T049, T054)

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde_json::json;
use std::collections::HashMap;

use ofml_lib::oap::config::Configuration;
use ofml_lib::oap::{
    ExportConfiguration, ExportData, ExportDiscountItem, ExportPricing, ExportSurcharge,
    ExportSurchargeItem, ExportWarning, PriceResult, Surcharge,
};
use ofml_lib::property::PropertyValue;

#[test]
fn test_export_data_serialization() {
    let mut properties = HashMap::new();
    properties.insert("height".to_string(), json!("720"));
    properties.insert("diameter".to_string(), json!("1200"));
    properties.insert("color".to_string(), json!("white"));

    let export = ExportData {
        manufacturer: "vitra".to_string(),
        article: "ViTable_Round".to_string(),
        article_number: Some("48-123-456".to_string()),
        variant_code: "H720_D1200_white".to_string(),
        properties,
        base_price: Some("1.234,56".to_string()),
        surcharges: Some(vec![]),
        total_price: Some("1.234,56".to_string()),
        currency: Some("EUR".to_string()),
        price_date: Some("2025-12-24".to_string()),
        sub_articles: vec![],
        exported_at: "2025-12-24T10:30:00Z".to_string(),
    };

    let json = serde_json::to_string_pretty(&export).unwrap();

    assert!(json.contains("\"manufacturer\": \"vitra\""));
    assert!(json.contains("\"article\": \"ViTable_Round\""));
    assert!(json.contains("\"variant_code\": \"H720_D1200_white\""));
    assert!(json.contains("\"base_price\": \"1.234,56\""));
    assert!(json.contains("\"currency\": \"EUR\""));
}

#[test]
fn test_export_data_deserialization() {
    let json = r#"{
        "manufacturer": "vitra",
        "article": "ViTable",
        "article_number": null,
        "variant_code": "H720",
        "properties": {"height": "720"},
        "base_price": "100,00",
        "surcharges": [],
        "total_price": "100,00",
        "currency": "EUR",
        "price_date": "2025-12-24",
        "sub_articles": [],
        "exported_at": "2025-12-24T10:30:00Z"
    }"#;

    let export: ExportData = serde_json::from_str(json).unwrap();

    assert_eq!(export.manufacturer, "vitra");
    assert_eq!(export.article, "ViTable");
    assert_eq!(export.variant_code, "H720");
    assert!(export.properties.contains_key("height"));
}

#[test]
fn test_export_data_with_surcharges() {
    let export = ExportData {
        manufacturer: "vitra".to_string(),
        article: "ViTable".to_string(),
        article_number: None,
        variant_code: "H720".to_string(),
        properties: Default::default(),
        base_price: Some("100,00".to_string()),
        surcharges: Some(vec![
            ExportSurcharge {
                name: "Materialaufpreis".to_string(),
                amount: "25,00".to_string(),
                is_percentage: false,
            },
            ExportSurcharge {
                name: "10% Aufschlag".to_string(),
                amount: "10".to_string(),
                is_percentage: true,
            },
        ]),
        total_price: Some("135,00".to_string()),
        currency: Some("EUR".to_string()),
        price_date: Some("2025-12-24".to_string()),
        sub_articles: vec![],
        exported_at: "2025-12-24T10:30:00Z".to_string(),
    };

    let json = serde_json::to_string(&export).unwrap();

    assert!(json.contains("Materialaufpreis"));
    // Note: is_percentage is skipped when false (skip_serializing_if)
    assert!(json.contains("\"is_percentage\":true"));
}

#[test]
fn test_export_data_with_sub_articles() {
    let sub_article = ExportData {
        manufacturer: "vitra".to_string(),
        article: "Leg".to_string(),
        article_number: None,
        variant_code: "chrome".to_string(),
        properties: Default::default(),
        base_price: Some("50,00".to_string()),
        surcharges: Some(vec![]),
        total_price: Some("50,00".to_string()),
        currency: Some("EUR".to_string()),
        price_date: Some("2025-12-24".to_string()),
        sub_articles: vec![],
        exported_at: "2025-12-24T10:30:00Z".to_string(),
    };

    let export = ExportData {
        manufacturer: "vitra".to_string(),
        article: "Table".to_string(),
        article_number: None,
        variant_code: "H720".to_string(),
        properties: Default::default(),
        base_price: Some("100,00".to_string()),
        surcharges: Some(vec![]),
        total_price: Some("100,00".to_string()),
        currency: Some("EUR".to_string()),
        price_date: Some("2025-12-24".to_string()),
        sub_articles: vec![sub_article],
        exported_at: "2025-12-24T10:30:00Z".to_string(),
    };

    let json = serde_json::to_string_pretty(&export).unwrap();

    assert!(json.contains("\"sub_articles\""));
    assert!(json.contains("\"article\": \"Leg\""));
}

#[test]
fn test_configuration_to_export_data() {
    let mut config = Configuration::new("TestArticle".to_string(), "vitra".to_string());
    config.article_number = Some("48-001".to_string());
    config.variant_code = "H720_white".to_string();

    config
        .properties
        .values
        .insert("height".to_string(), PropertyValue::Int(720));
    config.properties.values.insert(
        "color".to_string(),
        PropertyValue::Symbol("white".to_string()),
    );

    // Set a price
    config.price = Some(PriceResult::new(
        Decimal::new(10000, 2),
        vec![Surcharge {
            name: "Test".to_string(),
            amount: Decimal::new(500, 2),
            is_percentage: false,
        }],
        "EUR".to_string(),
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        None,
    ));

    let export = config.to_export_data();

    assert_eq!(export.manufacturer, "vitra");
    assert_eq!(export.article, "TestArticle");
    assert_eq!(export.article_number, Some("48-001".to_string()));
    assert_eq!(export.variant_code, "H720_white");
    assert_eq!(export.currency, Some("EUR".to_string()));
    assert!(export.surcharges.is_some());
    assert!(!export.surcharges.unwrap().is_empty());
}

#[test]
fn test_export_surcharge_structure() {
    // Test with is_percentage = true (to verify serialization)
    let surcharge = ExportSurcharge {
        name: "Aufpreis".to_string(),
        amount: "45,00".to_string(),
        is_percentage: true,
    };

    let json = serde_json::to_string(&surcharge).unwrap();
    assert!(json.contains("\"name\":\"Aufpreis\""));
    assert!(json.contains("\"amount\":\"45,00\""));
    assert!(json.contains("\"is_percentage\":true"));

    // Test with is_percentage = false (it's skipped in serialization)
    let surcharge_no_percent = ExportSurcharge {
        name: "Fixed".to_string(),
        amount: "10,00".to_string(),
        is_percentage: false,
    };

    let json_no_percent = serde_json::to_string(&surcharge_no_percent).unwrap();
    assert!(json_no_percent.contains("\"name\":\"Fixed\""));
    assert!(json_no_percent.contains("\"amount\":\"10,00\""));
    // is_percentage is skipped when false due to skip_serializing_if
    assert!(!json_no_percent.contains("is_percentage"));
}

#[test]
fn test_export_data_all_fields_present() {
    let export = ExportData {
        manufacturer: "test".to_string(),
        article: "test".to_string(),
        article_number: Some("123".to_string()),
        variant_code: "V1".to_string(),
        properties: Default::default(),
        base_price: Some("0,00".to_string()),
        surcharges: Some(vec![]),
        total_price: Some("0,00".to_string()),
        currency: Some("EUR".to_string()),
        price_date: Some("2025-01-01".to_string()),
        sub_articles: vec![],
        exported_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string(&export).unwrap();

    // Verify all required fields are in the JSON
    assert!(json.contains("manufacturer"));
    assert!(json.contains("article"));
    assert!(json.contains("article_number"));
    assert!(json.contains("variant_code"));
    assert!(json.contains("properties"));
    assert!(json.contains("base_price"));
    assert!(json.contains("surcharges"));
    assert!(json.contains("total_price"));
    assert!(json.contains("currency"));
    assert!(json.contains("price_date"));
    assert!(json.contains("exported_at"));
}

#[test]
fn test_export_data_roundtrip() {
    let mut properties = HashMap::new();
    properties.insert("height".to_string(), json!("720"));

    let original = ExportData {
        manufacturer: "vitra".to_string(),
        article: "Table".to_string(),
        article_number: Some("48-123".to_string()),
        variant_code: "H720_D1200".to_string(),
        properties,
        base_price: Some("1.234,56".to_string()),
        surcharges: Some(vec![ExportSurcharge {
            name: "Test".to_string(),
            amount: "10,00".to_string(),
            is_percentage: false,
        }]),
        total_price: Some("1.244,56".to_string()),
        currency: Some("EUR".to_string()),
        price_date: Some("2025-12-24".to_string()),
        sub_articles: vec![],
        exported_at: "2025-12-24T10:30:00Z".to_string(),
    };

    // Serialize
    let json = serde_json::to_string(&original).unwrap();

    // Deserialize
    let restored: ExportData = serde_json::from_str(&json).unwrap();

    // Verify equality
    assert_eq!(original.manufacturer, restored.manufacturer);
    assert_eq!(original.article, restored.article);
    assert_eq!(original.variant_code, restored.variant_code);
    assert_eq!(original.base_price, restored.base_price);
    assert_eq!(original.total_price, restored.total_price);
}

// === T054: Schema Validation Tests for ExportConfiguration ===

#[test]
fn test_export_configuration_schema_compliance() {
    // Test that ExportConfiguration matches contracts/export-schema.json structure
    let mut config = HashMap::new();
    config.insert("S_STOFF".to_string(), "GABRIEL_166".to_string());
    config.insert("S_GESTELL".to_string(), "SCHWARZ".to_string());

    let export = ExportConfiguration {
        article_nr: "SE:AI-102".to_string(),
        manufacturer: "sex".to_string(),
        series: "ai".to_string(),
        variant_code: Some("AI-102_GABRIEL_166_SCHWARZ".to_string()),
        description: Some("Sedus AI Bürostuhl".to_string()),
        configuration: config,
        property_details: vec![],
        pricing: ExportPricing {
            base: 1234.56,
            surcharges: vec![ExportSurchargeItem {
                var_cond: "S_166".to_string(),
                description: Some("Stoffgruppe 166".to_string()),
                amount: 45.00,
                is_percentage: false,
            }],
            discounts: vec![],
            net: 1279.56,
            taxes: vec![],
            total: 1279.56,
            currency: "EUR".to_string(),
            price_date: Some("2025-12-24".to_string()),
            valid_from: Some("2025-01-01".to_string()),
            valid_to: Some("2025-12-31".to_string()),
        },
        warnings: vec![],
        exported_at: "2025-12-24T10:30:00+00:00".to_string(),
    };

    let json = serde_json::to_string_pretty(&export).unwrap();

    // Verify required schema fields per contracts/export-schema.json
    assert!(json.contains("\"article_nr\""));
    assert!(json.contains("\"manufacturer\""));
    assert!(json.contains("\"series\""));
    assert!(json.contains("\"configuration\""));
    assert!(json.contains("\"pricing\""));
    assert!(json.contains("\"exported_at\""));

    // Verify pricing sub-structure
    assert!(json.contains("\"base\""));
    assert!(json.contains("\"surcharges\""));
    assert!(json.contains("\"discounts\""));
    assert!(json.contains("\"total\""));
    assert!(json.contains("\"currency\""));

    // Verify surcharge structure
    assert!(json.contains("\"var_cond\""));
    assert!(json.contains("\"is_percentage\""));
}

#[test]
fn test_export_configuration_with_warnings() {
    let export = ExportConfiguration {
        article_nr: "FRM:ONE".to_string(),
        manufacturer: "framery".to_string(),
        series: "one".to_string(),
        variant_code: None,
        description: None,
        configuration: HashMap::new(),
        property_details: vec![],
        pricing: ExportPricing {
            base: 12280.0,
            surcharges: vec![],
            discounts: vec![],
            net: 12280.0,
            taxes: vec![],
            total: 12280.0,
            currency: "EUR".to_string(),
            price_date: None,
            valid_from: None,
            valid_to: None,
        },
        warnings: vec![ExportWarning {
            severity: "warning".to_string(),
            code: "CORRUPTED_RECORD".to_string(),
            message: "Price record at offset 1234 appears corrupted, recovered via 8-byte shift"
                .to_string(),
            source: Some("pdata.ebase".to_string()),
        }],
        exported_at: "2025-12-24T10:30:00+00:00".to_string(),
    };

    let json = serde_json::to_string_pretty(&export).unwrap();

    // Verify warning structure per schema
    assert!(json.contains("\"warnings\""));
    assert!(json.contains("\"severity\""));
    assert!(json.contains("\"code\""));
    assert!(json.contains("\"message\""));
    assert!(json.contains("\"source\""));
    assert!(json.contains("\"warning\"")); // severity value
    assert!(json.contains("CORRUPTED_RECORD"));
}

#[test]
fn test_export_configuration_with_discounts() {
    let export = ExportConfiguration {
        article_nr: "TEST-001".to_string(),
        manufacturer: "test".to_string(),
        series: "test".to_string(),
        variant_code: None,
        description: None,
        configuration: HashMap::new(),
        property_details: vec![],
        pricing: ExportPricing {
            base: 1000.0,
            surcharges: vec![],
            discounts: vec![ExportDiscountItem {
                var_cond: "D_RABATT".to_string(),
                description: Some("Mengenrabatt 10%".to_string()),
                amount: 100.0,
                rule: "percent_of_base".to_string(),
            }],
            net: 900.0,
            taxes: vec![],
            total: 900.0,
            currency: "EUR".to_string(),
            price_date: None,
            valid_from: None,
            valid_to: None,
        },
        warnings: vec![],
        exported_at: "2025-12-24T10:30:00+00:00".to_string(),
    };

    let json = serde_json::to_string_pretty(&export).unwrap();

    // Verify discount structure
    assert!(json.contains("\"discounts\""));
    assert!(json.contains("\"rule\""));
    assert!(json.contains("percent_of_base"));
}

#[test]
fn test_export_configuration_roundtrip() {
    let mut config = HashMap::new();
    config.insert("prop1".to_string(), "value1".to_string());

    let original = ExportConfiguration {
        article_nr: "ART-001".to_string(),
        manufacturer: "mfr".to_string(),
        series: "ser".to_string(),
        variant_code: Some("VAR123".to_string()),
        description: Some("Test Article".to_string()),
        configuration: config,
        property_details: vec![],
        pricing: ExportPricing {
            base: 500.0,
            surcharges: vec![ExportSurchargeItem {
                var_cond: "S_TEST".to_string(),
                description: None,
                amount: 50.0,
                is_percentage: false,
            }],
            discounts: vec![],
            net: 550.0,
            taxes: vec![],
            total: 550.0,
            currency: "CHF".to_string(),
            price_date: Some("2025-12-24".to_string()),
            valid_from: Some("2025-01-01".to_string()),
            valid_to: None,
        },
        warnings: vec![],
        exported_at: "2025-12-24T10:30:00+00:00".to_string(),
    };

    // Serialize
    let json = serde_json::to_string(&original).unwrap();

    // Deserialize
    let restored: ExportConfiguration = serde_json::from_str(&json).unwrap();

    // Verify round-trip
    assert_eq!(original.article_nr, restored.article_nr);
    assert_eq!(original.manufacturer, restored.manufacturer);
    assert_eq!(original.series, restored.series);
    assert_eq!(original.pricing.base, restored.pricing.base);
    assert_eq!(original.pricing.total, restored.pricing.total);
    assert_eq!(original.pricing.currency, restored.pricing.currency);
    assert_eq!(
        original.pricing.surcharges.len(),
        restored.pricing.surcharges.len()
    );
}

#[test]
fn test_export_json_batch() {
    use ofml_lib::oap::export_json_batch;

    // Create two configurations
    let config1 = Configuration::new("Article1".to_string(), "mfr1".to_string());
    let config2 = Configuration::new("Article2".to_string(), "mfr2".to_string());

    let json = export_json_batch(&[config1, config2]);

    // Should produce a JSON array
    assert!(json.starts_with('['));
    assert!(json.ends_with(']'));
    assert!(json.contains("Article1"));
    assert!(json.contains("Article2"));
}
