//! ArticlePrice with full 8-field price model
//!
//! This module provides `ArticlePrice`, a comprehensive price structure that tracks:
//! - Single unit prices (purchase/sales/net/gross)
//! - Total prices for quantity (purchase/sales/net/gross)
//!
//! This matches the pCon.basket pricing model for full compatibility.
//!
//! # Price Fields
//!
//! | Field | Description |
//! |-------|-------------|
//! | `single_purchase_net` | Net purchase price per unit (cost to dealer) |
//! | `single_purchase_gross` | Gross purchase price per unit (with tax) |
//! | `single_sales_net` | Net sales price per unit (list price) |
//! | `single_sales_gross` | Gross sales price per unit (customer pays) |
//! | `total_purchase_net` | Net purchase price × quantity |
//! | `total_purchase_gross` | Gross purchase price × quantity |
//! | `total_sales_net` | Net sales price × quantity |
//! | `total_sales_gross` | Gross sales price × quantity |
//!
//! # Example
//! ```ignore
//! use ofml_lib::oap::article_price::{ArticlePrice, ArticlePriceBuilder};
//!
//! let price = ArticlePriceBuilder::new(Currency::EUR)
//!     .sales_net(Money::from_f64(100.0, Currency::EUR))
//!     .quantity(5)
//!     .with_tax(TaxScheme::germany())
//!     .build();
//!
//! assert_eq!(price.total_sales_gross().amount_cents(), 59500); // 100 * 5 * 1.19
//! ```

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::currency::{Currency, Money};
use super::discount::{AppliedDiscount, DiscountManager};
use super::tax::{TaxCalculator, TaxCategory, TaxScheme};
use super::{PriceResult, Surcharge};

/// Comprehensive article price with all 8 price fields
///
/// This structure tracks both single-unit and total prices across:
/// - Purchase prices (dealer cost)
/// - Sales prices (customer price)
/// - Net prices (before tax)
/// - Gross prices (after tax)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlePrice {
    // === Single Unit Prices ===
    /// Net purchase price per unit (dealer cost before tax)
    pub single_purchase_net: Money,
    /// Gross purchase price per unit (dealer cost after tax)
    pub single_purchase_gross: Money,
    /// Net sales price per unit (list price before tax)
    pub single_sales_net: Money,
    /// Gross sales price per unit (list price after tax)
    pub single_sales_gross: Money,

    // === Total Prices (quantity × single) ===
    /// Net purchase total (single_purchase_net × quantity)
    pub total_purchase_net: Money,
    /// Gross purchase total (single_purchase_gross × quantity)
    pub total_purchase_gross: Money,
    /// Net sales total (single_sales_net × quantity)
    pub total_sales_net: Money,
    /// Gross sales total (single_sales_gross × quantity)
    pub total_sales_gross: Money,

    // === Metadata ===
    /// Quantity for total calculations
    pub quantity: u32,
    /// Currency for all prices
    pub currency: Currency,
    /// Tax scheme used
    pub tax_category: TaxCategory,
    /// Tax rate applied (percentage)
    pub tax_rate: Decimal,
    /// Tax amount per unit (sales)
    pub tax_per_unit: Money,
    /// Total tax amount (sales)
    pub total_tax: Money,

    // === Price Components ===
    /// Base price before surcharges/discounts
    pub base_price: Money,
    /// Applied surcharges
    pub surcharges: Vec<ArticleSurcharge>,
    /// Total surcharge amount
    pub surcharges_total: Money,
    /// Applied discounts
    pub discounts: Vec<AppliedDiscount>,
    /// Total discount amount
    pub discounts_total: Money,

    // === Validity ===
    /// Price effective date
    pub price_date: NaiveDate,
    /// Price valid from
    pub valid_from: Option<NaiveDate>,
    /// Price valid until
    pub valid_to: Option<NaiveDate>,

    // === Margin (optional) ===
    /// Margin percentage (sales - purchase) / sales × 100
    pub margin_percent: Option<Decimal>,
    /// Margin amount per unit
    pub margin_per_unit: Option<Money>,
}

/// Surcharge applied to article price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSurcharge {
    /// Surcharge identifier/code
    pub code: String,
    /// Human-readable description
    pub description: String,
    /// Surcharge amount (per unit, net)
    pub amount: Money,
    /// Whether this is a percentage surcharge
    pub is_percentage: bool,
    /// Original percentage (if is_percentage)
    pub percentage: Option<Decimal>,
}

impl ArticlePrice {
    /// Create a zero-price article
    pub fn zero(currency: Currency) -> Self {
        Self {
            single_purchase_net: Money::zero(currency),
            single_purchase_gross: Money::zero(currency),
            single_sales_net: Money::zero(currency),
            single_sales_gross: Money::zero(currency),
            total_purchase_net: Money::zero(currency),
            total_purchase_gross: Money::zero(currency),
            total_sales_net: Money::zero(currency),
            total_sales_gross: Money::zero(currency),
            quantity: 1,
            currency,
            tax_category: TaxCategory::Standard,
            tax_rate: Decimal::ZERO,
            tax_per_unit: Money::zero(currency),
            total_tax: Money::zero(currency),
            base_price: Money::zero(currency),
            surcharges: Vec::new(),
            surcharges_total: Money::zero(currency),
            discounts: Vec::new(),
            discounts_total: Money::zero(currency),
            price_date: chrono::Local::now().date_naive(),
            valid_from: None,
            valid_to: None,
            margin_percent: None,
            margin_per_unit: None,
        }
    }

    /// Create from a simple net sales price
    pub fn from_net_sales(net_price: Money, quantity: u32) -> Self {
        let mut price = Self::zero(net_price.currency());
        price.single_sales_net = net_price;
        price.quantity = quantity;
        price.recalculate_totals();
        price
    }

    /// Create from net sales price with tax
    pub fn from_net_sales_with_tax(net_price: Money, quantity: u32, tax_scheme: &TaxScheme) -> Self {
        let mut price = Self::from_net_sales(net_price, quantity);
        price.apply_tax(tax_scheme, TaxCategory::Standard);
        price
    }

    /// Apply tax scheme to calculate gross prices
    pub fn apply_tax(&mut self, tax_scheme: &TaxScheme, category: TaxCategory) {
        let calculator = TaxCalculator::new(tax_scheme.clone());

        // Calculate tax for single sales price
        let sales_tax_result = calculator.calculate(&self.single_sales_net, category);
        self.single_sales_gross = sales_tax_result.gross_price;
        self.tax_per_unit = sales_tax_result.tax_total;

        if let Some(rate) = tax_scheme.get_rate(category) {
            self.tax_rate = rate.rate;
        }
        self.tax_category = category;

        // Calculate tax for single purchase price
        let purchase_tax_result = calculator.calculate(&self.single_purchase_net, category);
        self.single_purchase_gross = purchase_tax_result.gross_price;

        // Recalculate totals
        self.recalculate_totals();
    }

    /// Recalculate total prices from single prices × quantity
    pub fn recalculate_totals(&mut self) {
        let qty = Decimal::from(self.quantity);

        self.total_purchase_net = self.single_purchase_net.multiply(qty);
        self.total_purchase_gross = self.single_purchase_gross.multiply(qty);
        self.total_sales_net = self.single_sales_net.multiply(qty);
        self.total_sales_gross = self.single_sales_gross.multiply(qty);
        self.total_tax = self.tax_per_unit.multiply(qty);

        // Calculate margin if both prices are set
        if !self.single_sales_net.is_zero() && !self.single_purchase_net.is_zero() {
            let margin_amount = self.single_sales_net.checked_sub(&self.single_purchase_net).ok();
            if let Some(margin) = margin_amount {
                self.margin_per_unit = Some(margin);
                let margin_pct =
                    (margin.amount() / self.single_sales_net.amount()) * Decimal::from(100);
                self.margin_percent = Some(margin_pct);
            }
        }
    }

    /// Set quantity and recalculate totals
    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
        self.recalculate_totals();
    }

    /// Add a surcharge (updates net prices, must re-apply tax after)
    pub fn add_surcharge(&mut self, surcharge: ArticleSurcharge) {
        // Add to net sales price
        if surcharge.is_percentage {
            if let Some(pct) = surcharge.percentage {
                let surcharge_amount = self.base_price.apply_percentage(pct);
                self.single_sales_net = self.single_sales_net + surcharge_amount;
                self.surcharges_total = self.surcharges_total + surcharge_amount;
            }
        } else {
            self.single_sales_net = self.single_sales_net + surcharge.amount;
            self.surcharges_total = self.surcharges_total + surcharge.amount;
        }

        self.surcharges.push(surcharge);
    }

    /// Apply surcharges from legacy Surcharge type
    pub fn apply_surcharges(&mut self, surcharges: &[Surcharge]) {
        for s in surcharges {
            let article_surcharge = ArticleSurcharge {
                code: s.name.clone(),
                description: s.name.clone(),
                amount: Money::new(s.amount, self.currency),
                is_percentage: s.is_percentage,
                percentage: if s.is_percentage {
                    Some(s.amount)
                } else {
                    None
                },
            };
            self.add_surcharge(article_surcharge);
        }
    }

    /// Apply discounts using discount manager
    pub fn apply_discounts(&mut self, manager: &DiscountManager) {
        let result = manager.apply(&self.single_sales_net, self.quantity, self.price_date);

        self.discounts = result.discounts;
        self.discounts_total = result.total_discount;
        self.single_sales_net = result.discounted_price;

        // Recalculate totals after discount
        self.recalculate_totals();
    }

    /// Get the primary display price (usually total_sales_gross for B2C)
    pub fn display_price(&self) -> Money {
        self.total_sales_gross
    }

    /// Get formatted display string (German format)
    pub fn display_string(&self) -> String {
        self.total_sales_gross.format_german()
    }

    /// Check if this is a valid price (has non-zero sales price)
    pub fn is_valid(&self) -> bool {
        !self.single_sales_net.is_zero()
    }

    /// Get price breakdown as formatted string
    pub fn breakdown_string(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!("Base Price: {}", self.base_price.format_german()));

        if !self.surcharges.is_empty() {
            lines.push("Surcharges:".to_string());
            for s in &self.surcharges {
                lines.push(format!("  + {}: {}", s.description, s.amount.format_german()));
            }
            lines.push(format!(
                "  Surcharges Total: {}",
                self.surcharges_total.format_german()
            ));
        }

        if !self.discounts.is_empty() {
            lines.push("Discounts:".to_string());
            for d in &self.discounts {
                lines.push(format!("  - {}: {}", d.name, d.amount.format_german()));
            }
            lines.push(format!(
                "  Discounts Total: {}",
                self.discounts_total.format_german()
            ));
        }

        lines.push(format!(
            "Net Price (per unit): {}",
            self.single_sales_net.format_german()
        ));
        lines.push(format!(
            "Tax ({}%): {}",
            self.tax_rate,
            self.tax_per_unit.format_german()
        ));
        lines.push(format!(
            "Gross Price (per unit): {}",
            self.single_sales_gross.format_german()
        ));

        if self.quantity > 1 {
            lines.push(format!("Quantity: {}", self.quantity));
            lines.push(format!(
                "Total Net: {}",
                self.total_sales_net.format_german()
            ));
            lines.push(format!("Total Tax: {}", self.total_tax.format_german()));
            lines.push(format!(
                "Total Gross: {}",
                self.total_sales_gross.format_german()
            ));
        }

        if let Some(margin) = &self.margin_per_unit {
            if let Some(pct) = self.margin_percent {
                lines.push(format!(
                    "Margin: {} ({:.1}%)",
                    margin.format_german(),
                    pct
                ));
            }
        }

        lines.join("\n")
    }
}

impl Default for ArticlePrice {
    fn default() -> Self {
        Self::zero(Currency::EUR)
    }
}

impl From<&PriceResult> for ArticlePrice {
    /// Convert a PriceResult to ArticlePrice with full 8-field model
    fn from(result: &PriceResult) -> Self {
        let currency = Currency::from_code(&result.currency).unwrap_or(Currency::EUR);

        let mut price = ArticlePrice::zero(currency);

        // Set base price
        price.base_price = Money::new(result.base_price, currency);

        // Set sales net (base + surcharges - discounts)
        price.single_sales_net = Money::new(result.net_price, currency);

        // Set tax info
        if !result.taxes.is_empty() {
            price.tax_rate = result.taxes.iter().map(|t| t.rate).sum();
            price.tax_per_unit = Money::new(result.tax_total, currency);
        }

        // Set gross price
        price.single_sales_gross = Money::new(result.total_price, currency);

        // Convert surcharges
        for s in &result.surcharges {
            price.surcharges.push(ArticleSurcharge {
                code: s.name.clone(),
                description: s.name.clone(),
                amount: Money::new(s.amount, currency),
                is_percentage: s.is_percentage,
                percentage: if s.is_percentage {
                    Some(s.amount)
                } else {
                    None
                },
            });
            if !s.is_percentage {
                price.surcharges_total = price.surcharges_total + Money::new(s.amount, currency);
            } else {
                let surcharge_amount = price.base_price.apply_percentage(s.amount);
                price.surcharges_total = price.surcharges_total + surcharge_amount;
            }
        }

        // Set validity
        price.price_date = result.price_date;
        price.valid_from = Some(result.valid_from);
        price.valid_to = result.valid_to;

        // Recalculate totals (quantity = 1 from PriceResult)
        price.quantity = 1;
        price.recalculate_totals();

        price
    }
}

impl ArticlePrice {
    /// Create from a PriceResult with specified quantity
    pub fn from_price_result(result: &PriceResult, quantity: u32) -> Self {
        let mut price = ArticlePrice::from(result);
        price.set_quantity(quantity);
        price
    }
}

/// Builder for creating ArticlePrice instances
#[derive(Debug, Clone)]
pub struct ArticlePriceBuilder {
    currency: Currency,
    base_price: Option<Money>,
    purchase_net: Option<Money>,
    sales_net: Option<Money>,
    quantity: u32,
    surcharges: Vec<Surcharge>,
    tax_scheme: Option<TaxScheme>,
    tax_category: TaxCategory,
    price_date: NaiveDate,
    valid_from: Option<NaiveDate>,
    valid_to: Option<NaiveDate>,
    discount_manager: Option<DiscountManager>,
}

impl ArticlePriceBuilder {
    /// Create a new builder with the given currency
    pub fn new(currency: Currency) -> Self {
        Self {
            currency,
            base_price: None,
            purchase_net: None,
            sales_net: None,
            quantity: 1,
            surcharges: Vec::new(),
            tax_scheme: None,
            tax_category: TaxCategory::Standard,
            price_date: chrono::Local::now().date_naive(),
            valid_from: None,
            valid_to: None,
            discount_manager: None,
        }
    }

    /// Set the base price
    pub fn base_price(mut self, price: Money) -> Self {
        self.base_price = Some(price);
        self
    }

    /// Set the purchase net price
    pub fn purchase_net(mut self, price: Money) -> Self {
        self.purchase_net = Some(price);
        self
    }

    /// Set the sales net price
    pub fn sales_net(mut self, price: Money) -> Self {
        self.sales_net = Some(price);
        self
    }

    /// Set the quantity
    pub fn quantity(mut self, qty: u32) -> Self {
        self.quantity = qty;
        self
    }

    /// Add surcharges
    pub fn surcharges(mut self, surcharges: Vec<Surcharge>) -> Self {
        self.surcharges = surcharges;
        self
    }

    /// Add a single surcharge
    pub fn surcharge(mut self, surcharge: Surcharge) -> Self {
        self.surcharges.push(surcharge);
        self
    }

    /// Set tax scheme
    pub fn tax_scheme(mut self, scheme: TaxScheme) -> Self {
        self.tax_scheme = Some(scheme);
        self
    }

    /// Set tax category
    pub fn tax_category(mut self, category: TaxCategory) -> Self {
        self.tax_category = category;
        self
    }

    /// Set price date
    pub fn price_date(mut self, date: NaiveDate) -> Self {
        self.price_date = date;
        self
    }

    /// Set validity period
    pub fn valid(mut self, from: NaiveDate, to: NaiveDate) -> Self {
        self.valid_from = Some(from);
        self.valid_to = Some(to);
        self
    }

    /// Set discount manager
    pub fn discounts(mut self, manager: DiscountManager) -> Self {
        self.discount_manager = Some(manager);
        self
    }

    /// Build the ArticlePrice
    pub fn build(self) -> ArticlePrice {
        let mut price = ArticlePrice::zero(self.currency);

        // Set base price
        if let Some(base) = self.base_price {
            price.base_price = base;
        }

        // Set purchase net
        if let Some(pn) = self.purchase_net {
            price.single_purchase_net = pn;
        }

        // Set sales net (start from base or explicit value)
        if let Some(sn) = self.sales_net {
            price.single_sales_net = sn;
            if price.base_price.is_zero() {
                price.base_price = sn;
            }
        } else if let Some(base) = self.base_price {
            price.single_sales_net = base;
        }

        // Set metadata
        price.quantity = self.quantity;
        price.price_date = self.price_date;
        price.valid_from = self.valid_from;
        price.valid_to = self.valid_to;

        // Apply surcharges
        price.apply_surcharges(&self.surcharges);

        // Apply discounts
        if let Some(manager) = &self.discount_manager {
            price.apply_discounts(manager);
        }

        // Apply tax
        if let Some(scheme) = &self.tax_scheme {
            price.apply_tax(scheme, self.tax_category);
        }

        // Final recalculation
        price.recalculate_totals();

        price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_price_zero() {
        let price = ArticlePrice::zero(Currency::EUR);
        assert!(price.single_sales_net.is_zero());
        assert!(price.total_sales_gross.is_zero());
        assert_eq!(price.quantity, 1);
    }

    #[test]
    fn test_article_price_from_net_sales() {
        let net = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let price = ArticlePrice::from_net_sales(net, 1);

        assert_eq!(price.single_sales_net.amount_cents(), 10000);
        assert_eq!(price.total_sales_net.amount_cents(), 10000);
    }

    #[test]
    fn test_article_price_with_quantity() {
        let net = Money::from_cents(10000, Currency::EUR);
        let price = ArticlePrice::from_net_sales(net, 5);

        assert_eq!(price.single_sales_net.amount_cents(), 10000);
        assert_eq!(price.total_sales_net.amount_cents(), 50000);
    }

    #[test]
    fn test_article_price_with_tax() {
        let net = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let price = ArticlePrice::from_net_sales_with_tax(net, 1, &TaxScheme::germany());

        assert_eq!(price.single_sales_net.amount_cents(), 10000);
        assert_eq!(price.single_sales_gross.amount_cents(), 11900); // +19%
        assert_eq!(price.tax_per_unit.amount_cents(), 1900);
        assert_eq!(price.tax_rate, Decimal::from(19));
    }

    #[test]
    fn test_article_price_quantity_with_tax() {
        let net = Money::from_cents(10000, Currency::EUR);
        let price = ArticlePrice::from_net_sales_with_tax(net, 3, &TaxScheme::germany());

        assert_eq!(price.total_sales_net.amount_cents(), 30000);
        assert_eq!(price.total_sales_gross.amount_cents(), 35700);
        assert_eq!(price.total_tax.amount_cents(), 5700);
    }

    #[test]
    fn test_article_price_set_quantity() {
        let net = Money::from_cents(10000, Currency::EUR);
        let mut price = ArticlePrice::from_net_sales_with_tax(net, 1, &TaxScheme::germany());

        assert_eq!(price.total_sales_gross.amount_cents(), 11900);

        price.set_quantity(5);
        assert_eq!(price.total_sales_gross.amount_cents(), 59500);
    }

    #[test]
    fn test_article_price_margin() {
        let mut price = ArticlePrice::zero(Currency::EUR);
        price.single_purchase_net = Money::from_cents(7000, Currency::EUR); // 70.00 EUR
        price.single_sales_net = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        price.recalculate_totals();

        assert!(price.margin_per_unit.is_some());
        assert_eq!(price.margin_per_unit.unwrap().amount_cents(), 3000); // 30.00 EUR
        assert_eq!(price.margin_percent, Some(Decimal::from(30)));
    }

    #[test]
    fn test_article_price_builder_simple() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .sales_net(Money::from_cents(10000, Currency::EUR))
            .quantity(2)
            .build();

        assert_eq!(price.single_sales_net.amount_cents(), 10000);
        assert_eq!(price.total_sales_net.amount_cents(), 20000);
    }

    #[test]
    fn test_article_price_builder_with_tax() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .sales_net(Money::from_cents(10000, Currency::EUR))
            .tax_scheme(TaxScheme::germany())
            .build();

        assert_eq!(price.single_sales_gross.amount_cents(), 11900);
    }

    #[test]
    fn test_article_price_builder_with_surcharges() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .base_price(Money::from_cents(10000, Currency::EUR))
            .sales_net(Money::from_cents(10000, Currency::EUR))
            .surcharge(Surcharge {
                name: "Extra".to_string(),
                amount: Decimal::from(20),
                is_percentage: false,
            })
            .tax_scheme(TaxScheme::germany())
            .build();

        // Base 100 + 20 surcharge = 120 net
        assert_eq!(price.single_sales_net.amount_cents(), 12000);
        assert_eq!(price.single_sales_gross.amount_cents(), 14280); // 120 * 1.19
    }

    #[test]
    fn test_article_price_display() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .sales_net(Money::from_cents(123456, Currency::EUR))
            .tax_scheme(TaxScheme::germany())
            .build();

        let display = price.display_string();
        assert!(display.contains("EUR"));
    }

    #[test]
    fn test_article_price_is_valid() {
        let zero = ArticlePrice::zero(Currency::EUR);
        assert!(!zero.is_valid());

        let valid = ArticlePrice::from_net_sales(Money::from_cents(100, Currency::EUR), 1);
        assert!(valid.is_valid());
    }

    #[test]
    fn test_article_price_serialization() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .sales_net(Money::from_cents(10000, Currency::EUR))
            .quantity(2)
            .build();

        let json = serde_json::to_string(&price).unwrap();
        assert!(json.contains("EUR"));
        assert!(json.contains("10000"));

        let parsed: ArticlePrice = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.quantity, 2);
    }

    #[test]
    fn test_article_surcharge() {
        let surcharge = ArticleSurcharge {
            code: "S1".to_string(),
            description: "Color upgrade".to_string(),
            amount: Money::from_cents(500, Currency::EUR),
            is_percentage: false,
            percentage: None,
        };

        assert_eq!(surcharge.code, "S1");
        assert_eq!(surcharge.amount.amount_cents(), 500);
    }

    #[test]
    fn test_article_price_breakdown() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .base_price(Money::from_cents(10000, Currency::EUR))
            .sales_net(Money::from_cents(10000, Currency::EUR))
            .quantity(2)
            .tax_scheme(TaxScheme::germany())
            .build();

        let breakdown = price.breakdown_string();
        assert!(breakdown.contains("Base Price"));
        assert!(breakdown.contains("Quantity: 2"));
        assert!(breakdown.contains("19%"));
    }

    #[test]
    fn test_article_price_builder_full() {
        let price = ArticlePriceBuilder::new(Currency::EUR)
            .base_price(Money::from_cents(10000, Currency::EUR))
            .purchase_net(Money::from_cents(7000, Currency::EUR))
            .sales_net(Money::from_cents(10000, Currency::EUR))
            .quantity(5)
            .tax_scheme(TaxScheme::germany())
            .tax_category(TaxCategory::Standard)
            .price_date(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap())
            .valid(
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
            )
            .build();

        assert_eq!(price.quantity, 5);
        assert!(price.valid_from.is_some());
        assert!(price.valid_to.is_some());
        assert!(price.margin_percent.is_some());
    }

    #[test]
    fn test_article_price_from_price_result() {
        use super::super::{PriceResult, TaxEntry};
        use rust_decimal::Decimal;

        // Create a PriceResult
        let surcharges = vec![
            super::super::Surcharge {
                name: "Color upgrade".to_string(),
                amount: Decimal::from(20),
                is_percentage: false,
            },
        ];
        let taxes = vec![TaxEntry {
            name: "MwSt (19%)".to_string(),
            category: "standard".to_string(),
            rate: Decimal::from(19),
            amount: Decimal::new(2280, 2), // 19% of 120
        }];

        let price_result = PriceResult::with_taxes(
            Decimal::from(100), // base
            surcharges,
            taxes,
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        );

        // Convert to ArticlePrice
        let article_price = ArticlePrice::from(&price_result);

        // Verify conversion
        assert_eq!(article_price.base_price.amount_cents(), 10000); // 100.00 EUR
        assert_eq!(article_price.single_sales_net.amount_cents(), 12000); // 120.00 EUR (100 + 20)
        assert_eq!(article_price.tax_rate, Decimal::from(19));
        assert_eq!(article_price.quantity, 1);
        assert_eq!(article_price.surcharges.len(), 1);
        assert!(article_price.valid_from.is_some());
        assert!(article_price.valid_to.is_some());
    }

    #[test]
    fn test_article_price_from_price_result_with_quantity() {
        use super::super::PriceResult;
        use rust_decimal::Decimal;

        let price_result = PriceResult::new(
            Decimal::from(100),
            vec![],
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            None,
        );

        let article_price = ArticlePrice::from_price_result(&price_result, 5);

        assert_eq!(article_price.quantity, 5);
        assert_eq!(article_price.single_sales_net.amount_cents(), 10000);
        assert_eq!(article_price.total_sales_net.amount_cents(), 50000);
    }
}
