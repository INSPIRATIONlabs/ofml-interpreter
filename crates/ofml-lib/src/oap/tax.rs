//! Tax system for calculating VAT and other taxes
//!
//! This module provides:
//! - `TaxScheme` for defining tax configurations per country/region
//! - `TaxCategory` for standard/reduced/zero tax rates
//! - `TaxType` for different types of taxes (VAT, sales tax, etc.)
//! - `TaxCalculator` for computing taxes on prices
//!
//! # Tax Calculation Model
//!
//! The tax system follows European VAT conventions:
//! - Net price (before tax) + Tax = Gross price
//! - Multiple tax rates can be applied (e.g., standard VAT + eco-tax)
//! - Tax rates vary by country and product category
//!
//! # Example
//! ```ignore
//! use ofml_lib::oap::tax::{TaxScheme, TaxCategory, TaxCalculator};
//!
//! let scheme = TaxScheme::germany();
//! let calculator = TaxCalculator::new(scheme);
//!
//! let net_price = Money::from_f64(100.0, Currency::EUR);
//! let result = calculator.calculate(&net_price, TaxCategory::Standard);
//! // result.tax_amount = 19.00 EUR
//! // result.gross_price = 119.00 EUR
//! ```

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::currency::{Currency, Money};

/// Type of tax (VAT, sales tax, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxType {
    /// Value Added Tax (Europe)
    VAT,
    /// Sales Tax (US)
    SalesTax,
    /// Goods and Services Tax (AU, CA, etc.)
    GST,
    /// Harmonized Sales Tax (Canada)
    HST,
    /// Environmental tax
    EcoTax,
    /// Custom tax type
    Custom(String),
}

impl TaxType {
    pub fn name(&self) -> &str {
        match self {
            TaxType::VAT => "VAT",
            TaxType::SalesTax => "Sales Tax",
            TaxType::GST => "GST",
            TaxType::HST => "HST",
            TaxType::EcoTax => "Eco Tax",
            TaxType::Custom(name) => name,
        }
    }
}

/// Tax category (standard, reduced, zero, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxCategory {
    /// Standard tax rate (usually the highest)
    Standard,
    /// Reduced tax rate (for certain goods)
    Reduced,
    /// Super-reduced rate (some countries have multiple reduced rates)
    SuperReduced,
    /// Zero-rated (taxable but at 0%)
    Zero,
    /// Exempt (not subject to tax)
    Exempt,
}

impl TaxCategory {
    pub fn name(&self) -> &str {
        match self {
            TaxCategory::Standard => "Standard",
            TaxCategory::Reduced => "Reduced",
            TaxCategory::SuperReduced => "Super-Reduced",
            TaxCategory::Zero => "Zero",
            TaxCategory::Exempt => "Exempt",
        }
    }
}

impl Default for TaxCategory {
    fn default() -> Self {
        TaxCategory::Standard
    }
}

/// A single tax rate entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    /// Tax type (VAT, GST, etc.)
    pub tax_type: TaxType,
    /// Tax category (standard, reduced, etc.)
    pub category: TaxCategory,
    /// Tax rate as percentage (e.g., 19 for 19%)
    pub rate: Decimal,
    /// Human-readable description
    pub description: String,
}

impl TaxRate {
    pub fn new(tax_type: TaxType, category: TaxCategory, rate: Decimal, description: &str) -> Self {
        Self {
            tax_type,
            category,
            rate,
            description: description.to_string(),
        }
    }

    /// Calculate tax amount from net price
    pub fn calculate(&self, net_price: &Money) -> Money {
        net_price.apply_percentage(self.rate)
    }

    /// Get display name (e.g., "MwSt (19%)")
    pub fn display_name(&self) -> String {
        format!("{} ({}%)", self.description, self.rate)
    }
}

/// Tax scheme for a country/region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxScheme {
    /// Country/region code (e.g., "DE", "AT", "CH")
    pub country_code: String,
    /// Country/region name
    pub country_name: String,
    /// Currency used (for consistency)
    pub currency: Currency,
    /// Tax rates indexed by category
    pub rates: HashMap<TaxCategory, TaxRate>,
    /// Whether prices are typically shown with tax included
    pub prices_include_tax: bool,
}

impl TaxScheme {
    /// Create a new tax scheme
    pub fn new(country_code: &str, country_name: &str, currency: Currency) -> Self {
        Self {
            country_code: country_code.to_string(),
            country_name: country_name.to_string(),
            currency,
            rates: HashMap::new(),
            prices_include_tax: true, // European default
        }
    }

    /// Add a tax rate
    pub fn add_rate(&mut self, rate: TaxRate) {
        self.rates.insert(rate.category, rate);
    }

    /// Get tax rate for a category
    pub fn get_rate(&self, category: TaxCategory) -> Option<&TaxRate> {
        self.rates.get(&category)
    }

    /// Get the standard tax rate
    pub fn standard_rate(&self) -> Option<&TaxRate> {
        self.get_rate(TaxCategory::Standard)
    }

    /// Create German tax scheme (MwSt)
    pub fn germany() -> Self {
        let mut scheme = Self::new("DE", "Germany", Currency::EUR);
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(19),
            "MwSt",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Reduced,
            Decimal::from(7),
            "MwSt ermäßigt",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Zero,
            Decimal::ZERO,
            "MwSt 0%",
        ));
        scheme.prices_include_tax = true;
        scheme
    }

    /// Create Austrian tax scheme
    pub fn austria() -> Self {
        let mut scheme = Self::new("AT", "Austria", Currency::EUR);
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(20),
            "USt",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Reduced,
            Decimal::from(10),
            "USt ermäßigt",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::SuperReduced,
            Decimal::from(13),
            "USt 13%",
        ));
        scheme.prices_include_tax = true;
        scheme
    }

    /// Create Swiss tax scheme
    pub fn switzerland() -> Self {
        let mut scheme = Self::new("CH", "Switzerland", Currency::CHF);
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::new(81, 1), // 8.1%
            "MwSt",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Reduced,
            Decimal::new(26, 1), // 2.6%
            "MwSt reduziert",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::SuperReduced,
            Decimal::new(38, 1), // 3.8%
            "MwSt Sondersatz",
        ));
        scheme.prices_include_tax = true;
        scheme
    }

    /// Create French tax scheme
    pub fn france() -> Self {
        let mut scheme = Self::new("FR", "France", Currency::EUR);
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(20),
            "TVA",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Reduced,
            Decimal::from(10),
            "TVA réduit",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::SuperReduced,
            Decimal::new(55, 1), // 5.5%
            "TVA super réduit",
        ));
        scheme.prices_include_tax = true;
        scheme
    }

    /// Create UK tax scheme
    pub fn united_kingdom() -> Self {
        let mut scheme = Self::new("GB", "United Kingdom", Currency::GBP);
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(20),
            "VAT",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Reduced,
            Decimal::from(5),
            "VAT reduced",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Zero,
            Decimal::ZERO,
            "VAT zero",
        ));
        scheme.prices_include_tax = true;
        scheme
    }

    /// Create US tax scheme (placeholder - varies by state)
    pub fn united_states() -> Self {
        let mut scheme = Self::new("US", "United States", Currency::USD);
        // US doesn't have federal sales tax, this is a placeholder
        scheme.add_rate(TaxRate::new(
            TaxType::SalesTax,
            TaxCategory::Standard,
            Decimal::ZERO, // Varies by state
            "Sales Tax",
        ));
        scheme.prices_include_tax = false; // US typically shows prices without tax
        scheme
    }

    /// Get tax scheme by country code
    pub fn for_country(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "DE" => Some(Self::germany()),
            "AT" => Some(Self::austria()),
            "CH" => Some(Self::switzerland()),
            "FR" => Some(Self::france()),
            "GB" | "UK" => Some(Self::united_kingdom()),
            "US" => Some(Self::united_states()),
            _ => None,
        }
    }
}

impl Default for TaxScheme {
    fn default() -> Self {
        Self::germany()
    }
}

/// Result of tax calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxResult {
    /// Net price (before tax)
    pub net_price: Money,
    /// Individual tax amounts
    pub taxes: Vec<TaxDetail>,
    /// Total tax amount
    pub tax_total: Money,
    /// Gross price (after tax)
    pub gross_price: Money,
}

/// Detail of an individual tax applied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxDetail {
    /// Tax type
    pub tax_type: TaxType,
    /// Tax category
    pub category: TaxCategory,
    /// Tax rate applied
    pub rate: Decimal,
    /// Description
    pub description: String,
    /// Calculated tax amount
    pub amount: Money,
}

impl TaxDetail {
    /// Get display name (e.g., "MwSt (19%)")
    pub fn display_name(&self) -> String {
        format!("{} ({}%)", self.description, self.rate)
    }
}

/// Tax calculator for computing taxes
#[derive(Debug, Clone)]
pub struct TaxCalculator {
    /// Active tax scheme
    scheme: TaxScheme,
}

impl TaxCalculator {
    /// Create a new tax calculator with the given scheme
    pub fn new(scheme: TaxScheme) -> Self {
        Self { scheme }
    }

    /// Create a calculator for a country
    pub fn for_country(code: &str) -> Option<Self> {
        TaxScheme::for_country(code).map(Self::new)
    }

    /// Get the current tax scheme
    pub fn scheme(&self) -> &TaxScheme {
        &self.scheme
    }

    /// Calculate tax on a net price for the given category
    pub fn calculate(&self, net_price: &Money, category: TaxCategory) -> TaxResult {
        let mut taxes = Vec::new();
        let mut tax_total = Money::zero(net_price.currency());

        if let Some(rate) = self.scheme.get_rate(category) {
            let tax_amount = rate.calculate(net_price);
            taxes.push(TaxDetail {
                tax_type: rate.tax_type.clone(),
                category: rate.category,
                rate: rate.rate,
                description: rate.description.clone(),
                amount: tax_amount,
            });
            tax_total = tax_total + tax_amount;
        }

        let gross_price = *net_price + tax_total;

        TaxResult {
            net_price: *net_price,
            taxes,
            tax_total,
            gross_price,
        }
    }

    /// Calculate standard rate tax on a net price
    pub fn calculate_standard(&self, net_price: &Money) -> TaxResult {
        self.calculate(net_price, TaxCategory::Standard)
    }

    /// Extract net price from a gross price (reverse calculation)
    pub fn extract_net(&self, gross_price: &Money, category: TaxCategory) -> TaxResult {
        if let Some(rate) = self.scheme.get_rate(category) {
            // net = gross / (1 + rate/100)
            let divisor = Decimal::ONE + rate.rate / Decimal::from(100);
            let net_amount = gross_price.amount() / divisor;
            let net_price = Money::new(net_amount, gross_price.currency());
            return self.calculate(&net_price, category);
        }

        // If no rate found, gross = net
        TaxResult {
            net_price: *gross_price,
            taxes: Vec::new(),
            tax_total: Money::zero(gross_price.currency()),
            gross_price: *gross_price,
        }
    }

    /// Calculate with multiple tax rates
    pub fn calculate_multi(&self, net_price: &Money, categories: &[TaxCategory]) -> TaxResult {
        let mut taxes = Vec::new();
        let mut tax_total = Money::zero(net_price.currency());

        for category in categories {
            if let Some(rate) = self.scheme.get_rate(*category) {
                let tax_amount = rate.calculate(net_price);
                taxes.push(TaxDetail {
                    tax_type: rate.tax_type.clone(),
                    category: rate.category,
                    rate: rate.rate,
                    description: rate.description.clone(),
                    amount: tax_amount,
                });
                tax_total = tax_total + tax_amount;
            }
        }

        let gross_price = *net_price + tax_total;

        TaxResult {
            net_price: *net_price,
            taxes,
            tax_total,
            gross_price,
        }
    }
}

impl Default for TaxCalculator {
    fn default() -> Self {
        Self::new(TaxScheme::default())
    }
}

/// Registry of tax schemes by country
#[derive(Debug, Clone, Default)]
pub struct TaxRegistry {
    schemes: HashMap<String, TaxScheme>,
}

impl TaxRegistry {
    /// Create a new registry with default schemes
    pub fn new() -> Self {
        let mut registry = Self::empty();
        registry.load_defaults();
        registry
    }

    /// Create an empty registry
    pub fn empty() -> Self {
        Self {
            schemes: HashMap::new(),
        }
    }

    /// Load default tax schemes
    fn load_defaults(&mut self) {
        for code in ["DE", "AT", "CH", "FR", "GB", "US"] {
            if let Some(scheme) = TaxScheme::for_country(code) {
                self.register(scheme);
            }
        }
    }

    /// Register a tax scheme
    pub fn register(&mut self, scheme: TaxScheme) {
        self.schemes.insert(scheme.country_code.clone(), scheme);
    }

    /// Get a tax scheme by country code
    pub fn get(&self, country_code: &str) -> Option<&TaxScheme> {
        self.schemes.get(&country_code.to_uppercase())
    }

    /// Get a calculator for a country
    pub fn calculator(&self, country_code: &str) -> Option<TaxCalculator> {
        self.get(country_code).cloned().map(TaxCalculator::new)
    }

    /// List all registered country codes
    pub fn countries(&self) -> Vec<&str> {
        self.schemes.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_type_name() {
        assert_eq!(TaxType::VAT.name(), "VAT");
        assert_eq!(TaxType::SalesTax.name(), "Sales Tax");
        assert_eq!(TaxType::Custom("MyTax".into()).name(), "MyTax");
    }

    #[test]
    fn test_tax_category_name() {
        assert_eq!(TaxCategory::Standard.name(), "Standard");
        assert_eq!(TaxCategory::Reduced.name(), "Reduced");
        assert_eq!(TaxCategory::Exempt.name(), "Exempt");
    }

    #[test]
    fn test_tax_category_default() {
        assert_eq!(TaxCategory::default(), TaxCategory::Standard);
    }

    #[test]
    fn test_tax_rate_calculate() {
        let rate = TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(19),
            "MwSt",
        );
        let net = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let tax = rate.calculate(&net);
        assert_eq!(tax.amount_cents(), 1900); // 19.00 EUR
    }

    #[test]
    fn test_tax_rate_display_name() {
        let rate = TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(19),
            "MwSt",
        );
        assert_eq!(rate.display_name(), "MwSt (19%)");
    }

    #[test]
    fn test_tax_scheme_germany() {
        let scheme = TaxScheme::germany();
        assert_eq!(scheme.country_code, "DE");
        assert_eq!(scheme.currency, Currency::EUR);

        let standard = scheme.standard_rate().unwrap();
        assert_eq!(standard.rate, Decimal::from(19));

        let reduced = scheme.get_rate(TaxCategory::Reduced).unwrap();
        assert_eq!(reduced.rate, Decimal::from(7));
    }

    #[test]
    fn test_tax_scheme_switzerland() {
        let scheme = TaxScheme::switzerland();
        assert_eq!(scheme.country_code, "CH");
        assert_eq!(scheme.currency, Currency::CHF);

        let standard = scheme.standard_rate().unwrap();
        assert_eq!(standard.rate, Decimal::new(81, 1)); // 8.1%
    }

    #[test]
    fn test_tax_scheme_for_country() {
        assert!(TaxScheme::for_country("DE").is_some());
        assert!(TaxScheme::for_country("de").is_some());
        assert!(TaxScheme::for_country("XX").is_none());
    }

    #[test]
    fn test_tax_calculator_standard() {
        let calculator = TaxCalculator::new(TaxScheme::germany());
        let net = Money::from_cents(10000, Currency::EUR); // 100.00 EUR

        let result = calculator.calculate_standard(&net);

        assert_eq!(result.net_price.amount_cents(), 10000);
        assert_eq!(result.tax_total.amount_cents(), 1900);
        assert_eq!(result.gross_price.amount_cents(), 11900);
        assert_eq!(result.taxes.len(), 1);
        assert_eq!(result.taxes[0].rate, Decimal::from(19));
    }

    #[test]
    fn test_tax_calculator_reduced() {
        let calculator = TaxCalculator::new(TaxScheme::germany());
        let net = Money::from_cents(10000, Currency::EUR);

        let result = calculator.calculate(&net, TaxCategory::Reduced);

        assert_eq!(result.tax_total.amount_cents(), 700); // 7%
        assert_eq!(result.gross_price.amount_cents(), 10700);
    }

    #[test]
    fn test_tax_calculator_zero() {
        let calculator = TaxCalculator::new(TaxScheme::germany());
        let net = Money::from_cents(10000, Currency::EUR);

        let result = calculator.calculate(&net, TaxCategory::Zero);

        assert_eq!(result.tax_total.amount_cents(), 0);
        assert_eq!(result.gross_price.amount_cents(), 10000);
    }

    #[test]
    fn test_tax_calculator_extract_net() {
        let calculator = TaxCalculator::new(TaxScheme::germany());
        let gross = Money::from_cents(11900, Currency::EUR); // 119.00 EUR

        let result = calculator.extract_net(&gross, TaxCategory::Standard);

        // Should extract net = 100.00 EUR
        assert_eq!(result.net_price.amount_cents(), 10000);
        assert_eq!(result.tax_total.amount_cents(), 1900);
    }

    #[test]
    fn test_tax_calculator_for_country() {
        let calc = TaxCalculator::for_country("DE");
        assert!(calc.is_some());
        assert_eq!(calc.unwrap().scheme().country_code, "DE");

        let calc_none = TaxCalculator::for_country("XX");
        assert!(calc_none.is_none());
    }

    #[test]
    fn test_tax_registry_new() {
        let registry = TaxRegistry::new();
        assert!(registry.get("DE").is_some());
        assert!(registry.get("AT").is_some());
        assert!(registry.get("CH").is_some());
    }

    #[test]
    fn test_tax_registry_calculator() {
        let registry = TaxRegistry::new();
        let calc = registry.calculator("DE");
        assert!(calc.is_some());
    }

    #[test]
    fn test_tax_registry_countries() {
        let registry = TaxRegistry::new();
        let countries = registry.countries();
        assert!(countries.contains(&"DE"));
        assert!(countries.contains(&"AT"));
    }

    #[test]
    fn test_tax_result_serialization() {
        let result = TaxResult {
            net_price: Money::from_cents(10000, Currency::EUR),
            taxes: vec![TaxDetail {
                tax_type: TaxType::VAT,
                category: TaxCategory::Standard,
                rate: Decimal::from(19),
                description: "MwSt".to_string(),
                amount: Money::from_cents(1900, Currency::EUR),
            }],
            tax_total: Money::from_cents(1900, Currency::EUR),
            gross_price: Money::from_cents(11900, Currency::EUR),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("10000"));
        assert!(json.contains("11900"));
        assert!(json.contains("MwSt"));
    }

    #[test]
    fn test_tax_detail_display_name() {
        let detail = TaxDetail {
            tax_type: TaxType::VAT,
            category: TaxCategory::Standard,
            rate: Decimal::from(19),
            description: "MwSt".to_string(),
            amount: Money::from_cents(1900, Currency::EUR),
        };
        assert_eq!(detail.display_name(), "MwSt (19%)");
    }

    #[test]
    fn test_tax_scheme_default() {
        let scheme = TaxScheme::default();
        assert_eq!(scheme.country_code, "DE");
    }

    #[test]
    fn test_tax_scheme_austria() {
        let scheme = TaxScheme::austria();
        assert_eq!(scheme.country_code, "AT");
        let standard = scheme.standard_rate().unwrap();
        assert_eq!(standard.rate, Decimal::from(20));
    }

    #[test]
    fn test_tax_scheme_france() {
        let scheme = TaxScheme::france();
        assert_eq!(scheme.country_code, "FR");
        let standard = scheme.standard_rate().unwrap();
        assert_eq!(standard.rate, Decimal::from(20));
    }

    #[test]
    fn test_tax_scheme_uk() {
        let scheme = TaxScheme::united_kingdom();
        assert_eq!(scheme.country_code, "GB");
        let standard = scheme.standard_rate().unwrap();
        assert_eq!(standard.rate, Decimal::from(20));
    }

    #[test]
    fn test_tax_scheme_us() {
        let scheme = TaxScheme::united_states();
        assert_eq!(scheme.country_code, "US");
        assert!(!scheme.prices_include_tax);
    }

    #[test]
    fn test_calculate_multi() {
        // Create a scheme with multiple tax categories
        let mut scheme = TaxScheme::new("TEST", "Test Country", Currency::EUR);
        scheme.add_rate(TaxRate::new(
            TaxType::VAT,
            TaxCategory::Standard,
            Decimal::from(19),
            "VAT",
        ));
        scheme.add_rate(TaxRate::new(
            TaxType::EcoTax,
            TaxCategory::Reduced,
            Decimal::from(2),
            "Eco",
        ));

        let calculator = TaxCalculator::new(scheme);
        let net = Money::from_cents(10000, Currency::EUR);

        let result = calculator.calculate_multi(&net, &[TaxCategory::Standard, TaxCategory::Reduced]);

        assert_eq!(result.taxes.len(), 2);
        assert_eq!(result.tax_total.amount_cents(), 2100); // 19% + 2%
    }

    #[test]
    fn test_tax_registry_register() {
        let mut registry = TaxRegistry::empty();
        assert!(registry.get("XX").is_none());

        let scheme = TaxScheme::new("XX", "Test Country", Currency::EUR);
        registry.register(scheme);

        assert!(registry.get("XX").is_some());
    }
}
