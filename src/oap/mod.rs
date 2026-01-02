//! OAP (OFML Article Presentation) Configuration Engine
//!
//! This module provides the core configuration engine for OFML products,
//! enabling:
//! - Manufacturer and article discovery from OFML data directories
//! - Property-based product configuration with validation
//! - Variant code generation from property values
//! - Price lookup from EBASE databases
//! - Configuration export to JSON
//!
//! The module is designed to be reusable across CLI, TUI, and future WASM targets.

pub mod actions;
pub mod catalog;
pub mod config;
pub mod engine;
pub mod families;
pub mod manufacturers;
pub mod oam;
pub mod ocd;
pub mod ocd_properties;
pub mod price;
pub mod property;
pub mod variant;

// Re-export Configuration for TUI module
pub use config::Configuration;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// User-facing strings (German defaults, organized for future i18n)
pub mod strings {
    pub const MSG_MANUFACTURERS_HEADER: &str = "Hersteller in";
    pub const MSG_ARTICLES_HEADER: &str = "Artikel für";
    pub const MSG_CONFIGURING: &str = "Konfiguriere";
    pub const MSG_PROPERTIES: &str = "Eigenschaften";
    pub const MSG_VARIANT_CODE: &str = "Variantencode";
    pub const MSG_BASE_PRICE: &str = "Grundpreis";
    pub const MSG_SURCHARGES: &str = "Aufpreise";
    pub const MSG_TOTAL_PRICE: &str = "Gesamtpreis";
    pub const MSG_PRICE_DATE: &str = "Preisdatum";
    pub const MSG_PRICE_NOT_AVAILABLE: &str = "Preis nicht verfügbar";
    pub const MSG_MANUFACTURER_NOT_FOUND: &str = "Hersteller nicht gefunden";
    pub const MSG_ARTICLE_NOT_FOUND: &str = "Artikel nicht gefunden";
    pub const MSG_INVALID_PROPERTY_VALUE: &str = "Ungültiger Eigenschaftswert";
    pub const MSG_EXPORT_SUCCESS: &str = "Export erfolgreich";
    pub const MSG_TOTAL: &str = "Gesamt";
    pub const MSG_NO_PRODUCTS: &str = "(keine Produkte)";
}

/// Represents a furniture manufacturer with OFML data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manufacturer {
    /// Directory name (e.g., "vitra")
    pub id: String,
    /// Display name
    pub name: String,
    /// Absolute path to data directory
    pub path: PathBuf,
}

/// Represents a product line within a manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    /// Series identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Parent manufacturer
    pub manufacturer_id: String,
    /// Number of articles in series
    pub article_count: usize,
}

/// Represents a configurable product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Class name (e.g., "ViTable_Round")
    pub id: String,
    /// Parent manufacturer
    pub manufacturer_id: String,
    /// Parent series (optional)
    pub series_id: Option<String>,
    /// Brief product description
    pub short_description: String,
    /// Full description (optional)
    pub long_description: Option<String>,
    /// EBASE article number
    pub base_article_number: String,
    /// Whether article has OAP properties
    pub has_configuration: bool,
}

/// Pricing information from EBASE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceResult {
    /// Base article price
    pub base_price: Decimal,
    /// Additional charges
    pub surcharges: Vec<Surcharge>,
    /// Computed total
    pub total_price: Decimal,
    /// Currency code (EUR, CHF, etc.)
    pub currency: String,
    /// Effective date for lookup
    pub price_date: NaiveDate,
    /// Price validity start
    pub valid_from: NaiveDate,
    /// Price validity end (optional)
    pub valid_to: Option<NaiveDate>,
}

impl PriceResult {
    /// Create a new PriceResult and compute total
    pub fn new(
        base_price: Decimal,
        surcharges: Vec<Surcharge>,
        currency: String,
        price_date: NaiveDate,
        valid_from: NaiveDate,
        valid_to: Option<NaiveDate>,
    ) -> Self {
        let total_price = Self::compute_total(&base_price, &surcharges);
        Self {
            base_price,
            surcharges,
            total_price,
            currency,
            price_date,
            valid_from,
            valid_to,
        }
    }

    /// Compute total from base price and surcharges
    fn compute_total(base_price: &Decimal, surcharges: &[Surcharge]) -> Decimal {
        let mut total = *base_price;
        for surcharge in surcharges {
            if surcharge.is_percentage {
                total += *base_price * surcharge.amount / Decimal::from(100);
            } else {
                total += surcharge.amount;
            }
        }
        total
    }
}

/// Price surcharge entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surcharge {
    /// Surcharge description
    pub name: String,
    /// Surcharge value
    pub amount: Decimal,
    /// Whether amount is percentage
    pub is_percentage: bool,
}

/// Exported configuration for JSON output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    /// Manufacturer ID
    pub manufacturer: String,
    /// Article ID
    pub article: String,
    /// EBASE article number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    /// Generated variant code
    pub variant_code: String,
    /// Property values
    pub properties: HashMap<String, serde_json::Value>,
    /// Formatted base price (German format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price: Option<String>,
    /// Surcharges list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharges: Option<Vec<ExportSurcharge>>,
    /// Formatted total price (German format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<String>,
    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Price date (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_date: Option<String>,
    /// Nested sub-article configurations
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sub_articles: Vec<ExportData>,
    /// Export timestamp (ISO 8601)
    pub exported_at: String,
}

/// Surcharge for export (with string amount)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSurcharge {
    pub name: String,
    pub amount: String,
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    pub is_percentage: bool,
}

/// Format a decimal price in German format (1.234,56)
pub fn format_german_price(amount: Decimal) -> String {
    let formatted = format!("{:.2}", amount);
    let parts: Vec<&str> = formatted.split('.').collect();

    let integer_part = parts[0];
    let decimal_part = parts.get(1).unwrap_or(&"00");

    // Add thousand separators
    let mut result = String::new();
    let chars: Vec<char> = integer_part.chars().collect();
    let len = chars.len();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push('.');
        }
        result.push(*c);
    }

    format!("{},{}", result, decimal_part)
}

/// Format a price with currency (German format)
pub fn format_german_price_with_currency(amount: Decimal, currency: &str) -> String {
    format!("{} {}", format_german_price(amount), currency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_german_price_basic() {
        let price = Decimal::new(123456, 2); // 1234.56
        assert_eq!(format_german_price(price), "1.234,56");
    }

    #[test]
    fn test_format_german_price_small() {
        let price = Decimal::new(4500, 2); // 45.00
        assert_eq!(format_german_price(price), "45,00");
    }

    #[test]
    fn test_format_german_price_large() {
        let price = Decimal::new(12345678900, 2); // 123456789.00
        assert_eq!(format_german_price(price), "123.456.789,00");
    }

    #[test]
    fn test_format_german_price_with_currency() {
        let price = Decimal::new(123456, 2);
        assert_eq!(
            format_german_price_with_currency(price, "EUR"),
            "1.234,56 EUR"
        );
    }

    #[test]
    fn test_price_result_compute_total() {
        let base = Decimal::new(10000, 2); // 100.00
        let surcharges = vec![
            Surcharge {
                name: "Color".to_string(),
                amount: Decimal::new(1000, 2), // 10.00
                is_percentage: false,
            },
            Surcharge {
                name: "Rush".to_string(),
                amount: Decimal::new(1000, 2), // 10%
                is_percentage: true,
            },
        ];

        let result = PriceResult::new(
            base,
            surcharges,
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            None,
        );

        // 100 + 10 + 10% of 100 = 120
        assert_eq!(result.total_price, Decimal::new(12000, 2));
    }
}
