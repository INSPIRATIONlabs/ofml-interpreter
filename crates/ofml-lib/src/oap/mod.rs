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
pub mod ocd_relation;
pub mod price;
pub mod property;
pub mod variant;

// Re-export Configuration for TUI module
pub use config::Configuration;

// Re-export warning types from ocd module
pub use ocd::{DataWarning, PricingStrategy, WarningSeverity};

// Re-export price types from price module
pub use price::{Discount, PriceBreakdown, PriceError};

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
    /// Net total (before taxes)
    pub net_price: Decimal,
    /// Applicable taxes
    pub taxes: Vec<TaxEntry>,
    /// Total tax amount
    pub tax_total: Decimal,
    /// Gross total (including taxes) - this is the final price
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
    /// Create a new PriceResult and compute total (without taxes - legacy compatibility)
    pub fn new(
        base_price: Decimal,
        surcharges: Vec<Surcharge>,
        currency: String,
        price_date: NaiveDate,
        valid_from: NaiveDate,
        valid_to: Option<NaiveDate>,
    ) -> Self {
        Self::with_taxes(
            base_price,
            surcharges,
            Vec::new(),
            currency,
            price_date,
            valid_from,
            valid_to,
        )
    }

    /// Create a new PriceResult with taxes
    pub fn with_taxes(
        base_price: Decimal,
        surcharges: Vec<Surcharge>,
        taxes: Vec<TaxEntry>,
        currency: String,
        price_date: NaiveDate,
        valid_from: NaiveDate,
        valid_to: Option<NaiveDate>,
    ) -> Self {
        let net_price = Self::compute_net(&base_price, &surcharges);
        let tax_total = taxes.iter().map(|t| t.amount).sum();
        let total_price = net_price + tax_total;
        Self {
            base_price,
            surcharges,
            net_price,
            taxes,
            tax_total,
            total_price,
            currency,
            price_date,
            valid_from,
            valid_to,
        }
    }

    /// Create a new PriceResult with taxes and pre-calculated (potentially rounded) prices
    #[allow(clippy::too_many_arguments)]
    pub fn with_taxes_and_rounding(
        base_price: Decimal,
        surcharges: Vec<Surcharge>,
        taxes: Vec<TaxEntry>,
        net_price: Decimal,
        total_price: Decimal,
        currency: String,
        price_date: NaiveDate,
        valid_from: NaiveDate,
        valid_to: Option<NaiveDate>,
    ) -> Self {
        let tax_total = taxes.iter().map(|t| t.amount).sum();
        Self {
            base_price,
            surcharges,
            net_price,
            taxes,
            tax_total,
            total_price,
            currency,
            price_date,
            valid_from,
            valid_to,
        }
    }

    /// Compute net price from base price and surcharges (before taxes)
    fn compute_net(base_price: &Decimal, surcharges: &[Surcharge]) -> Decimal {
        Self::compute_net_static(base_price, surcharges)
    }

    /// Static version of compute_net for use before PriceResult creation
    pub fn compute_net_static(base_price: &Decimal, surcharges: &[Surcharge]) -> Decimal {
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

    /// Calculate taxes from tax schemes and add to the result
    pub fn calculate_taxes_from_schemes(
        net_price: Decimal,
        tax_schemes: &[(String, String, Decimal)], // (name, category, rate)
    ) -> Vec<TaxEntry> {
        tax_schemes
            .iter()
            .map(|(name, category, rate)| {
                let amount = net_price * rate / Decimal::from(100);
                TaxEntry {
                    name: name.clone(),
                    category: category.clone(),
                    rate: *rate,
                    amount,
                }
            })
            .collect()
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

/// Tax entry for price calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxEntry {
    /// Tax description (e.g., "MwSt", "VAT")
    pub name: String,
    /// Tax category (e.g., "standard", "reduced")
    pub category: String,
    /// Tax rate as percentage (e.g., 19.0 for 19%)
    pub rate: Decimal,
    /// Calculated tax amount
    pub amount: Decimal,
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
        if i > 0 && (len - i).rem_euclid(3) == 0 {
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

// === Export Functions (T051-T052, T055-T056) ===

/// Export data conforming to contracts/export-schema.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfiguration {
    /// Article number
    pub article_nr: String,
    /// Manufacturer identifier
    pub manufacturer: String,
    /// Series identifier
    pub series: String,
    /// Generated variant code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_code: Option<String>,
    /// Article description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property selections (property_id -> value_id)
    pub configuration: HashMap<String, String>,
    /// Property details with labels (for human-readable output)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub property_details: Vec<ExportPropertyDetail>,
    /// Pricing information
    pub pricing: ExportPricing,
    /// Data warnings if any
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<ExportWarning>,
    /// Export timestamp (ISO 8601)
    pub exported_at: String,
}

/// Detailed property information for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPropertyDetail {
    /// Property key/ID
    pub key: String,
    /// Human-readable property label
    pub label: String,
    /// Selected value code
    pub value: String,
    /// Human-readable value label
    pub value_label: String,
    /// Property group (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// Pricing section for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPricing {
    /// Base price
    pub base: f64,
    /// Surcharges applied (positive amounts)
    pub surcharges: Vec<ExportSurchargeItem>,
    /// Discounts applied (shown as positive amounts that are subtracted)
    pub discounts: Vec<ExportDiscountItem>,
    /// Net price (before taxes)
    pub net: f64,
    /// Taxes applied
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub taxes: Vec<ExportTaxItem>,
    /// Total calculated price (including taxes)
    pub total: f64,
    /// Currency code (EUR, CHF, etc.)
    pub currency: String,
    /// Price date used for lookup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_date: Option<String>,
    /// Price valid from date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// Price valid until date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}

/// Tax item for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaxItem {
    /// Tax name (e.g., "MwSt (19%)")
    pub name: String,
    /// Tax rate as percentage
    pub rate: f64,
    /// Tax amount
    pub amount: f64,
}

/// Surcharge item for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSurchargeItem {
    /// Variant condition code
    pub var_cond: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Amount (absolute or percentage)
    pub amount: f64,
    /// Whether amount is a percentage
    pub is_percentage: bool,
}

/// Discount item for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDiscountItem {
    /// Variant condition code
    pub var_cond: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Discount amount
    pub amount: f64,
    /// Discount rule
    pub rule: String,
}

/// Warning for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportWarning {
    /// Severity level
    pub severity: String,
    /// Warning code
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Source of the warning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl From<&DataWarning> for ExportWarning {
    fn from(w: &DataWarning) -> Self {
        Self {
            severity: match w.severity {
                WarningSeverity::Info => "info".to_string(),
                WarningSeverity::Warning => "warning".to_string(),
                WarningSeverity::Error => "error".to_string(),
            },
            code: w.code.clone(),
            message: w.message.clone(),
            source: w.source.clone(),
        }
    }
}

/// Export a configuration to JSON string
///
/// # Arguments
/// * `config` - Configuration to export
///
/// # Returns
/// * `String` - JSON representation conforming to export-schema.json
pub fn export_json(config: &Configuration) -> String {
    let export = config.to_export_data();
    serde_json::to_string_pretty(&export)
        .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e))
}

/// Export multiple configurations to JSON array
///
/// # Arguments
/// * `configs` - Configurations to export
///
/// # Returns
/// * `String` - JSON array representation
pub fn export_json_batch(configs: &[Configuration]) -> String {
    let exports: Vec<ExportData> = configs.iter().map(|c| c.to_export_data()).collect();
    serde_json::to_string_pretty(&exports)
        .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize batch: {}\"}}", e))
}

/// Export a family configuration to schema-compliant JSON
///
/// This produces output conforming to contracts/export-schema.json
pub fn export_family_json(
    manufacturer_id: &str,
    series_id: &str,
    article_nr: &str,
    config: &families::FamilyConfiguration,
    price: Option<&PriceResult>,
    warnings: &[DataWarning],
) -> String {
    let export = create_export_configuration(
        manufacturer_id,
        series_id,
        article_nr,
        config,
        price,
        warnings,
    );
    serde_json::to_string_pretty(&export)
        .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e))
}

/// Export multiple family configurations to JSON array
pub fn export_family_json_batch(exports: Vec<ExportConfiguration>) -> String {
    serde_json::to_string_pretty(&exports)
        .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize batch: {}\"}}", e))
}

/// Create an ExportConfiguration from family configuration data
pub fn create_export_configuration(
    manufacturer_id: &str,
    series_id: &str,
    article_nr: &str,
    config: &families::FamilyConfiguration,
    price: Option<&PriceResult>,
    warnings: &[DataWarning],
) -> ExportConfiguration {
    create_export_configuration_with_details(
        manufacturer_id,
        series_id,
        article_nr,
        config,
        price,
        warnings,
        None, // No description
        &[],  // No property details
    )
}

/// Create an ExportConfiguration with full details including property labels
pub fn create_export_configuration_with_details(
    manufacturer_id: &str,
    series_id: &str,
    article_nr: &str,
    config: &families::FamilyConfiguration,
    price: Option<&PriceResult>,
    warnings: &[DataWarning],
    description: Option<&str>,
    properties: &[families::FamilyProperty],
) -> ExportConfiguration {
    use chrono::Utc;
    use rust_decimal::Decimal;

    let pricing = if let Some(p) = price {
        // Separate surcharges (positive) from discounts (negative)
        let (surcharges, discounts): (Vec<_>, Vec<_>) = p
            .surcharges
            .iter()
            .partition(|s| s.amount >= Decimal::ZERO);

        ExportPricing {
            base: p.base_price.to_string().parse().unwrap_or(0.0),
            surcharges: surcharges
                .iter()
                .map(|s| ExportSurchargeItem {
                    var_cond: s.name.clone(),
                    description: Some(s.name.clone()),
                    amount: s.amount.to_string().parse().unwrap_or(0.0),
                    is_percentage: s.is_percentage,
                })
                .collect(),
            discounts: discounts
                .iter()
                .map(|s| {
                    let name = s.name.strip_prefix("Rabatt: ").unwrap_or(&s.name);
                    ExportDiscountItem {
                        var_cond: name.to_string(),
                        description: Some(name.to_string()),
                        amount: s.amount.abs().to_string().parse().unwrap_or(0.0),
                        rule: if s.is_percentage {
                            "percentage".to_string()
                        } else {
                            "absolute".to_string()
                        },
                    }
                })
                .collect(),
            net: p.net_price.to_string().parse().unwrap_or(0.0),
            taxes: p
                .taxes
                .iter()
                .map(|t| ExportTaxItem {
                    name: t.name.clone(),
                    rate: t.rate.to_string().parse().unwrap_or(0.0),
                    amount: t.amount.to_string().parse().unwrap_or(0.0),
                })
                .collect(),
            total: p.total_price.to_string().parse().unwrap_or(0.0),
            currency: p.currency.clone(),
            price_date: Some(p.price_date.format("%Y-%m-%d").to_string()),
            valid_from: Some(p.valid_from.format("%Y-%m-%d").to_string()),
            valid_to: p.valid_to.map(|d| d.format("%Y-%m-%d").to_string()),
        }
    } else {
        ExportPricing {
            base: 0.0,
            surcharges: Vec::new(),
            discounts: Vec::new(),
            net: 0.0,
            taxes: Vec::new(),
            total: 0.0,
            currency: "EUR".to_string(),
            price_date: None,
            valid_from: None,
            valid_to: None,
        }
    };

    // Build property details from properties and selections
    let property_details: Vec<ExportPropertyDetail> = properties
        .iter()
        .filter_map(|prop| {
            let value = config.selections.get(&prop.key)?;
            let value_label = prop
                .options
                .iter()
                .find(|o| &o.value == value)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| value.clone());

            Some(ExportPropertyDetail {
                key: prop.key.clone(),
                label: prop.label.clone(),
                value: value.clone(),
                value_label,
                group: if prop.group.is_empty() {
                    None
                } else {
                    Some(prop.group.clone())
                },
            })
        })
        .collect();

    ExportConfiguration {
        article_nr: article_nr.to_string(),
        manufacturer: manufacturer_id.to_string(),
        series: series_id.to_string(),
        variant_code: Some(config.variant_code.clone()),
        description: description.map(|s| s.to_string()),
        configuration: config.selections.clone(),
        property_details,
        pricing,
        warnings: warnings.iter().map(ExportWarning::from).collect(),
        exported_at: Utc::now().to_rfc3339(),
    }
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

    #[test]
    fn test_manufacturer_struct() {
        let mfr = Manufacturer {
            id: "vitra".to_string(),
            name: "Vitra AG".to_string(),
            path: PathBuf::from("/data/vitra"),
        };
        let debug = format!("{:?}", mfr);
        assert!(debug.contains("vitra"));
        let cloned = mfr.clone();
        assert_eq!(cloned.id, mfr.id);
    }

    #[test]
    fn test_series_struct() {
        let series = Series {
            id: "workit".to_string(),
            name: "WorkIt Collection".to_string(),
            manufacturer_id: "vitra".to_string(),
            article_count: 15,
        };
        let debug = format!("{:?}", series);
        assert!(debug.contains("workit"));
        let cloned = series.clone();
        assert_eq!(cloned.article_count, 15);
    }

    #[test]
    fn test_article_struct() {
        let article = Article {
            id: "VT-001".to_string(),
            manufacturer_id: "vitra".to_string(),
            series_id: Some("workit".to_string()),
            short_description: "Office Desk".to_string(),
            long_description: Some("A beautiful office desk".to_string()),
            base_article_number: "12345".to_string(),
            has_configuration: true,
        };
        let debug = format!("{:?}", article);
        assert!(debug.contains("VT-001"));
        let cloned = article.clone();
        assert_eq!(cloned.short_description, "Office Desk");
    }

    #[test]
    fn test_surcharge_struct() {
        let surcharge = Surcharge {
            name: "Color upgrade".to_string(),
            amount: Decimal::from(50),
            is_percentage: false,
        };
        let debug = format!("{:?}", surcharge);
        assert!(debug.contains("Color upgrade"));
        let cloned = surcharge.clone();
        assert_eq!(cloned.amount, Decimal::from(50));
    }

    #[test]
    fn test_tax_entry_struct() {
        let tax = TaxEntry {
            name: "VAT".to_string(),
            category: "standard".to_string(),
            rate: Decimal::from(19),
            amount: Decimal::from(38),
        };
        let debug = format!("{:?}", tax);
        assert!(debug.contains("VAT"));
        let cloned = tax.clone();
        assert_eq!(cloned.rate, Decimal::from(19));
    }

    #[test]
    fn test_price_result_with_taxes() {
        let base = Decimal::from(100);
        let surcharges = vec![];
        let taxes = vec![TaxEntry {
            name: "VAT".to_string(),
            category: "standard".to_string(),
            rate: Decimal::from(19),
            amount: Decimal::from(19),
        }];

        let result = PriceResult::with_taxes(
            base,
            surcharges,
            taxes,
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            None,
        );

        assert_eq!(result.net_price, Decimal::from(100));
        assert_eq!(result.tax_total, Decimal::from(19));
        assert_eq!(result.total_price, Decimal::from(119));
    }

    #[test]
    fn test_price_result_with_taxes_and_rounding() {
        let base = Decimal::from(100);
        let surcharges = vec![];
        let taxes = vec![];
        let net = Decimal::from(100);
        let total = Decimal::new(10050, 2); // 100.50 (after rounding)

        let result = PriceResult::with_taxes_and_rounding(
            base,
            surcharges,
            taxes,
            net,
            total,
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            None,
        );

        assert_eq!(result.total_price, Decimal::new(10050, 2));
    }

    #[test]
    fn test_compute_net_static() {
        let base = Decimal::from(100);
        let surcharges = vec![
            Surcharge {
                name: "S1".to_string(),
                amount: Decimal::from(10),
                is_percentage: false,
            },
            Surcharge {
                name: "S2".to_string(),
                amount: Decimal::from(20),
                is_percentage: true, // 20% of 100 = 20
            },
        ];

        let net = PriceResult::compute_net_static(&base, &surcharges);
        assert_eq!(net, Decimal::from(130)); // 100 + 10 + 20
    }

    #[test]
    fn test_calculate_taxes_from_schemes() {
        let net = Decimal::from(100);
        let schemes = vec![
            (
                "VAT".to_string(),
                "standard".to_string(),
                Decimal::from(19),
            ),
            (
                "Local Tax".to_string(),
                "local".to_string(),
                Decimal::from(1),
            ),
        ];

        let taxes = PriceResult::calculate_taxes_from_schemes(net, &schemes);
        assert_eq!(taxes.len(), 2);
        assert_eq!(taxes[0].amount, Decimal::from(19));
        assert_eq!(taxes[1].amount, Decimal::from(1));
    }

    #[test]
    fn test_export_data_serialization() {
        let mut properties = HashMap::new();
        properties.insert(
            "color".to_string(),
            serde_json::Value::String("red".to_string()),
        );

        let export = ExportData {
            manufacturer: "test".to_string(),
            article: "ART-1".to_string(),
            article_number: Some("123".to_string()),
            variant_code: "V1".to_string(),
            properties,
            base_price: Some("100,00".to_string()),
            surcharges: None,
            total_price: Some("120,00".to_string()),
            currency: Some("EUR".to_string()),
            price_date: Some("2025-01-01".to_string()),
            sub_articles: vec![],
            exported_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&export).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("ART-1"));
    }

    #[test]
    fn test_export_surcharge_serialization() {
        let surcharge = ExportSurcharge {
            name: "Test".to_string(),
            amount: "10,00".to_string(),
            is_percentage: false,
        };
        let json = serde_json::to_string(&surcharge).unwrap();
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_export_configuration_serialization() {
        let pricing = ExportPricing {
            base: 100.0,
            surcharges: vec![],
            discounts: vec![],
            net: 100.0,
            taxes: vec![],
            total: 100.0,
            currency: "EUR".to_string(),
            price_date: None,
            valid_from: None,
            valid_to: None,
        };

        let export = ExportConfiguration {
            article_nr: "ART-1".to_string(),
            manufacturer: "test".to_string(),
            series: "series1".to_string(),
            variant_code: Some("V1".to_string()),
            description: Some("Test article".to_string()),
            configuration: HashMap::new(),
            property_details: vec![],
            pricing,
            warnings: vec![],
            exported_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&export).unwrap();
        assert!(json.contains("ART-1"));
        assert!(json.contains("series1"));
    }

    #[test]
    fn test_export_property_detail_struct() {
        let detail = ExportPropertyDetail {
            key: "color".to_string(),
            label: "Color".to_string(),
            value: "red".to_string(),
            value_label: "Red".to_string(),
            group: Some("Appearance".to_string()),
        };
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("color"));
        assert!(json.contains("Appearance"));
    }

    #[test]
    fn test_export_tax_item_struct() {
        let tax = ExportTaxItem {
            name: "VAT".to_string(),
            rate: 19.0,
            amount: 38.0,
        };
        let json = serde_json::to_string(&tax).unwrap();
        assert!(json.contains("VAT"));
        assert!(json.contains("19"));
    }

    #[test]
    fn test_export_surcharge_item_struct() {
        let item = ExportSurchargeItem {
            var_cond: "S_COLOR".to_string(),
            description: Some("Color upgrade".to_string()),
            amount: 50.0,
            is_percentage: false,
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("S_COLOR"));
    }

    #[test]
    fn test_export_discount_item_struct() {
        let item = ExportDiscountItem {
            var_cond: "D_BULK".to_string(),
            description: Some("Bulk discount".to_string()),
            amount: 10.0,
            rule: "percentage".to_string(),
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("D_BULK"));
        assert!(json.contains("percentage"));
    }

    #[test]
    fn test_export_warning_struct() {
        let warning = ExportWarning {
            severity: "warning".to_string(),
            code: "W001".to_string(),
            message: "Test warning".to_string(),
            source: Some("ocd".to_string()),
        };
        let json = serde_json::to_string(&warning).unwrap();
        assert!(json.contains("W001"));
        assert!(json.contains("warning"));
    }

    #[test]
    fn test_export_warning_from_data_warning() {
        let data_warning = DataWarning {
            severity: WarningSeverity::Info,
            code: "I001".to_string(),
            message: "Info message".to_string(),
            source: Some("test".to_string()),
        };
        let export_warning = ExportWarning::from(&data_warning);
        assert_eq!(export_warning.severity, "info");
        assert_eq!(export_warning.code, "I001");

        let data_warning2 = DataWarning {
            severity: WarningSeverity::Error,
            code: "E001".to_string(),
            message: "Error message".to_string(),
            source: None,
        };
        let export_warning2 = ExportWarning::from(&data_warning2);
        assert_eq!(export_warning2.severity, "error");
    }

    #[test]
    fn test_format_german_price_zero() {
        let price = Decimal::ZERO;
        assert_eq!(format_german_price(price), "0,00");
    }

    #[test]
    fn test_format_german_price_negative() {
        let price = Decimal::new(-12345, 2); // -123.45
        // Note: this tests the current behavior - may need adjustment
        let result = format_german_price(price);
        assert!(result.contains("-"));
    }

    #[test]
    fn test_strings_module() {
        assert!(!strings::MSG_MANUFACTURERS_HEADER.is_empty());
        assert!(!strings::MSG_ARTICLES_HEADER.is_empty());
        assert!(!strings::MSG_CONFIGURING.is_empty());
        assert!(!strings::MSG_PROPERTIES.is_empty());
        assert!(!strings::MSG_VARIANT_CODE.is_empty());
        assert!(!strings::MSG_BASE_PRICE.is_empty());
        assert!(!strings::MSG_SURCHARGES.is_empty());
        assert!(!strings::MSG_TOTAL_PRICE.is_empty());
        assert!(!strings::MSG_PRICE_DATE.is_empty());
        assert!(!strings::MSG_PRICE_NOT_AVAILABLE.is_empty());
        assert!(!strings::MSG_MANUFACTURER_NOT_FOUND.is_empty());
        assert!(!strings::MSG_ARTICLE_NOT_FOUND.is_empty());
        assert!(!strings::MSG_INVALID_PROPERTY_VALUE.is_empty());
        assert!(!strings::MSG_EXPORT_SUCCESS.is_empty());
        assert!(!strings::MSG_TOTAL.is_empty());
        assert!(!strings::MSG_NO_PRODUCTS.is_empty());
    }

    #[test]
    fn test_price_result_serialization() {
        let result = PriceResult::new(
            Decimal::from(100),
            vec![],
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        );

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("EUR"));
        assert!(json.contains("100"));

        let parsed: PriceResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.base_price, result.base_price);
    }

    #[test]
    fn test_export_json_with_config() {
        let config = Configuration::new("TestArticle".to_string(), "testmfr".to_string());
        let json = export_json(&config);
        assert!(json.contains("TestArticle"));
        assert!(json.contains("testmfr"));
    }

    #[test]
    fn test_export_json_batch_empty() {
        let configs: Vec<Configuration> = vec![];
        let json = export_json_batch(&configs);
        assert_eq!(json.trim(), "[]");
    }

    #[test]
    fn test_export_json_batch_with_configs() {
        let config1 = Configuration::new("Article1".to_string(), "mfr1".to_string());
        let config2 = Configuration::new("Article2".to_string(), "mfr2".to_string());
        let json = export_json_batch(&[config1, config2]);
        assert!(json.contains("Article1"));
        assert!(json.contains("Article2"));
    }

    #[test]
    fn test_export_family_json() {
        use families::{FamilyConfiguration, FamilyProperty};

        let properties: Vec<FamilyProperty> = vec![];
        let mut config = FamilyConfiguration::new("test-family", &properties);
        config.set("COLOR", "RED");

        let json = export_family_json("testmfr", "series1", "ART-001", &config, None, &[]);
        assert!(json.contains("testmfr"));
        assert!(json.contains("series1"));
        assert!(json.contains("ART-001"));
    }

    #[test]
    fn test_export_family_json_with_price() {
        use families::{FamilyConfiguration, FamilyProperty};

        let properties: Vec<FamilyProperty> = vec![];
        let config = FamilyConfiguration::new("test-family", &properties);
        let price = PriceResult::new(
            Decimal::from(500),
            vec![],
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            None,
        );

        let json = export_family_json("mfr", "ser", "art", &config, Some(&price), &[]);
        assert!(json.contains("500"));
        assert!(json.contains("EUR"));
    }

    #[test]
    fn test_export_family_json_with_warnings() {
        use families::{FamilyConfiguration, FamilyProperty};

        let properties: Vec<FamilyProperty> = vec![];
        let config = FamilyConfiguration::new("test-family", &properties);
        let warnings = vec![DataWarning {
            severity: WarningSeverity::Warning,
            code: "W001".to_string(),
            message: "Test warning".to_string(),
            source: Some("test".to_string()),
        }];

        let json = export_family_json("mfr", "ser", "art", &config, None, &warnings);
        assert!(json.contains("W001"));
        assert!(json.contains("Test warning"));
    }

    #[test]
    fn test_export_family_json_batch_with_exports() {
        let export = ExportConfiguration {
            article_nr: "ART-1".to_string(),
            manufacturer: "test".to_string(),
            series: "series1".to_string(),
            variant_code: None,
            description: None,
            configuration: HashMap::new(),
            property_details: vec![],
            pricing: ExportPricing {
                base: 100.0,
                surcharges: vec![],
                discounts: vec![],
                net: 100.0,
                taxes: vec![],
                total: 100.0,
                currency: "EUR".to_string(),
                price_date: None,
                valid_from: None,
                valid_to: None,
            },
            warnings: vec![],
            exported_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let json = export_family_json_batch(vec![export]);
        assert!(json.contains("ART-1"));
        assert!(json.contains("series1"));
    }

    #[test]
    fn test_create_export_configuration_basic() {
        use families::{FamilyConfiguration, FamilyProperty};

        let properties: Vec<FamilyProperty> = vec![];
        let mut config = FamilyConfiguration::new("test-family", &properties);
        config.set("WIDTH", "1000");

        let export = create_export_configuration("mfr", "ser", "art", &config, None, &[]);
        assert_eq!(export.manufacturer, "mfr");
        assert_eq!(export.series, "ser");
        assert_eq!(export.article_nr, "art");
        assert!(export.configuration.contains_key("WIDTH"));
    }
}
