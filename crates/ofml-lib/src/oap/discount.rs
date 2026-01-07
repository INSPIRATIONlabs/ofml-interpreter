//! Discount system for applying various discount types
//!
//! This module provides:
//! - `DiscountType` for categorizing discounts (vendor, customer, volume, promotional)
//! - `DiscountRule` for discount calculation rules (percentage, fixed, tiered)
//! - `DiscountEntry` for individual discount definitions
//! - `DiscountManager` for managing and applying multiple discounts
//!
//! # Discount Calculation Order
//!
//! Discounts are applied in the following order:
//! 1. Vendor discounts (from manufacturer)
//! 2. Volume discounts (quantity-based)
//! 3. Customer discounts (negotiated)
//! 4. Promotional discounts (time-limited)
//!
//! # Example
//! ```ignore
//! use ofml_lib::oap::discount::{DiscountManager, DiscountEntry, DiscountType, DiscountRule};
//!
//! let mut manager = DiscountManager::new();
//! manager.add(DiscountEntry::percentage(
//!     "VENDOR10",
//!     "10% Vendor Discount",
//!     DiscountType::Vendor,
//!     10.0,
//! ));
//!
//! let net_price = Money::from_f64(1000.0, Currency::EUR);
//! let result = manager.apply(&net_price);
//! // result.discounted_price = 900.00 EUR
//! ```

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::currency::Money;

/// Type of discount
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DiscountType {
    /// Manufacturer/vendor discount
    #[default]
    Vendor,
    /// Customer-specific discount (negotiated)
    Customer,
    /// Quantity-based volume discount
    Volume,
    /// Time-limited promotional discount
    Promotional,
    /// OCD-defined discount (from var_cond price_level='D')
    OcdDiscount,
    /// Custom discount type
    Custom(String),
}

impl DiscountType {
    pub fn name(&self) -> &str {
        match self {
            DiscountType::Vendor => "Vendor",
            DiscountType::Customer => "Customer",
            DiscountType::Volume => "Volume",
            DiscountType::Promotional => "Promotional",
            DiscountType::OcdDiscount => "OCD",
            DiscountType::Custom(name) => name,
        }
    }

    /// Get the priority order (lower = applied first)
    pub fn priority(&self) -> u8 {
        match self {
            DiscountType::Vendor => 1,
            DiscountType::Volume => 2,
            DiscountType::Customer => 3,
            DiscountType::Promotional => 4,
            DiscountType::OcdDiscount => 5,
            DiscountType::Custom(_) => 10,
        }
    }
}

/// How the discount is calculated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscountRule {
    /// Percentage discount (e.g., 10 = 10%)
    Percentage(Decimal),
    /// Fixed amount discount
    FixedAmount(Decimal),
    /// Tiered percentage discount based on quantity
    TieredPercentage(Vec<(u32, Decimal)>),
    /// Tiered fixed amount discount based on quantity
    TieredFixed(Vec<(u32, Decimal)>),
    /// Buy X get Y free
    BuyXGetYFree { buy: u32, free: u32 },
}

impl DiscountRule {
    /// Calculate discount amount for a given price and quantity
    pub fn calculate(&self, price: &Money, quantity: u32) -> Money {
        match self {
            DiscountRule::Percentage(pct) => price.apply_percentage(*pct),
            DiscountRule::FixedAmount(amount) => {
                Money::new(*amount, price.currency()) * quantity as i32
            }
            DiscountRule::TieredPercentage(tiers) => {
                let pct = find_tier_value(tiers, quantity);
                price.apply_percentage(pct)
            }
            DiscountRule::TieredFixed(tiers) => {
                let amount = find_tier_value(tiers, quantity);
                Money::new(amount, price.currency()) * quantity as i32
            }
            DiscountRule::BuyXGetYFree { buy, free } => {
                if quantity >= *buy {
                    let free_items = (quantity / buy) * free;
                    let unit_price = price.amount() / Decimal::from(quantity);
                    Money::new(unit_price * Decimal::from(free_items), price.currency())
                } else {
                    Money::zero(price.currency())
                }
            }
        }
    }

    /// Get description of the rule
    pub fn description(&self) -> String {
        match self {
            DiscountRule::Percentage(pct) => format!("{}%", pct),
            DiscountRule::FixedAmount(amount) => format!("{} off", amount),
            DiscountRule::TieredPercentage(tiers) => {
                format!("Tiered: {:?}", tiers)
            }
            DiscountRule::TieredFixed(tiers) => {
                format!("Tiered fixed: {:?}", tiers)
            }
            DiscountRule::BuyXGetYFree { buy, free } => {
                format!("Buy {} get {} free", buy, free)
            }
        }
    }
}

/// Find the applicable tier value for a quantity
fn find_tier_value(tiers: &[(u32, Decimal)], quantity: u32) -> Decimal {
    let mut result = Decimal::ZERO;
    for (min_qty, value) in tiers {
        if quantity >= *min_qty {
            result = *value;
        } else {
            break;
        }
    }
    result
}

/// A discount entry definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountEntry {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: String,
    /// Discount type
    pub discount_type: DiscountType,
    /// How to calculate the discount
    pub rule: DiscountRule,
    /// Whether this discount is currently active
    pub active: bool,
    /// Valid from date (optional)
    pub valid_from: Option<NaiveDate>,
    /// Valid to date (optional)
    pub valid_to: Option<NaiveDate>,
    /// Minimum order value required (optional)
    pub min_order_value: Option<Decimal>,
    /// Conditions for applying this discount
    pub conditions: HashMap<String, String>,
}

impl DiscountEntry {
    /// Create a new percentage discount
    pub fn percentage(
        id: &str,
        name: &str,
        discount_type: DiscountType,
        percentage: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: format!("{}% off", percentage),
            discount_type,
            rule: DiscountRule::Percentage(Decimal::try_from(percentage).unwrap_or_default()),
            active: true,
            valid_from: None,
            valid_to: None,
            min_order_value: None,
            conditions: HashMap::new(),
        }
    }

    /// Create a new fixed amount discount
    pub fn fixed_amount(
        id: &str,
        name: &str,
        discount_type: DiscountType,
        amount: Decimal,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: format!("{} off", amount),
            discount_type,
            rule: DiscountRule::FixedAmount(amount),
            active: true,
            valid_from: None,
            valid_to: None,
            min_order_value: None,
            conditions: HashMap::new(),
        }
    }

    /// Create a tiered percentage discount
    pub fn tiered(
        id: &str,
        name: &str,
        discount_type: DiscountType,
        tiers: Vec<(u32, f64)>,
    ) -> Self {
        let decimal_tiers: Vec<(u32, Decimal)> = tiers
            .into_iter()
            .map(|(qty, pct)| (qty, Decimal::try_from(pct).unwrap_or_default()))
            .collect();

        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: "Tiered volume discount".to_string(),
            discount_type,
            rule: DiscountRule::TieredPercentage(decimal_tiers),
            active: true,
            valid_from: None,
            valid_to: None,
            min_order_value: None,
            conditions: HashMap::new(),
        }
    }

    /// Create from OCD var_cond discount
    pub fn from_ocd(var_cond: &str, amount: Decimal, description: &str) -> Self {
        Self {
            id: var_cond.to_string(),
            name: description.to_string(),
            description: description.to_string(),
            discount_type: DiscountType::OcdDiscount,
            rule: DiscountRule::FixedAmount(amount),
            active: true,
            valid_from: None,
            valid_to: None,
            min_order_value: None,
            conditions: HashMap::new(),
        }
    }

    /// Set validity period
    pub fn with_validity(mut self, from: NaiveDate, to: NaiveDate) -> Self {
        self.valid_from = Some(from);
        self.valid_to = Some(to);
        self
    }

    /// Set minimum order value
    pub fn with_min_order(mut self, min_value: Decimal) -> Self {
        self.min_order_value = Some(min_value);
        self
    }

    /// Add a condition
    pub fn with_condition(mut self, key: &str, value: &str) -> Self {
        self.conditions.insert(key.to_string(), value.to_string());
        self
    }

    /// Check if discount is valid on a given date
    pub fn is_valid_on(&self, date: NaiveDate) -> bool {
        if !self.active {
            return false;
        }
        if let Some(from) = self.valid_from {
            if date < from {
                return false;
            }
        }
        if let Some(to) = self.valid_to {
            if date > to {
                return false;
            }
        }
        true
    }

    /// Check if discount applies to an order value
    pub fn applies_to_value(&self, order_value: &Money) -> bool {
        if let Some(min) = self.min_order_value {
            return order_value.amount() >= min;
        }
        true
    }

    /// Calculate discount amount
    pub fn calculate(&self, price: &Money, quantity: u32) -> Money {
        self.rule.calculate(price, quantity)
    }
}

/// Applied discount detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedDiscount {
    /// Discount ID
    pub id: String,
    /// Discount name
    pub name: String,
    /// Discount type
    pub discount_type: DiscountType,
    /// Rule description
    pub rule_description: String,
    /// Discount amount
    pub amount: Money,
    /// Percentage (if applicable)
    pub percentage: Option<Decimal>,
}

/// Result of applying discounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountResult {
    /// Original price
    pub original_price: Money,
    /// Applied discounts
    pub discounts: Vec<AppliedDiscount>,
    /// Total discount amount
    pub total_discount: Money,
    /// Final discounted price
    pub discounted_price: Money,
}

impl DiscountResult {
    /// Check if any discounts were applied
    pub fn has_discounts(&self) -> bool {
        !self.discounts.is_empty()
    }

    /// Get total discount as percentage of original price
    pub fn discount_percentage(&self) -> Decimal {
        if self.original_price.is_zero() {
            Decimal::ZERO
        } else {
            (self.total_discount.amount() / self.original_price.amount()) * Decimal::from(100)
        }
    }
}

/// Manager for handling multiple discounts
#[derive(Debug, Clone, Default)]
pub struct DiscountManager {
    /// Registered discounts
    discounts: Vec<DiscountEntry>,
    /// Whether to apply discounts cumulatively
    cumulative: bool,
}

impl DiscountManager {
    /// Create a new discount manager
    pub fn new() -> Self {
        Self {
            discounts: Vec::new(),
            cumulative: true,
        }
    }

    /// Create a non-cumulative discount manager (only best discount applies)
    pub fn non_cumulative() -> Self {
        Self {
            discounts: Vec::new(),
            cumulative: false,
        }
    }

    /// Add a discount
    pub fn add(&mut self, discount: DiscountEntry) {
        self.discounts.push(discount);
    }

    /// Remove a discount by ID
    pub fn remove(&mut self, id: &str) -> bool {
        let len = self.discounts.len();
        self.discounts.retain(|d| d.id != id);
        self.discounts.len() < len
    }

    /// Get a discount by ID
    pub fn get(&self, id: &str) -> Option<&DiscountEntry> {
        self.discounts.iter().find(|d| d.id == id)
    }

    /// List all discounts
    pub fn list(&self) -> &[DiscountEntry] {
        &self.discounts
    }

    /// Get active discounts on a given date
    pub fn active_on(&self, date: NaiveDate) -> Vec<&DiscountEntry> {
        self.discounts.iter().filter(|d| d.is_valid_on(date)).collect()
    }

    /// Apply all applicable discounts to a price
    pub fn apply(&self, price: &Money, quantity: u32, date: NaiveDate) -> DiscountResult {
        let mut applied_discounts = Vec::new();
        let mut total_discount = Money::zero(price.currency());

        // Get applicable discounts sorted by priority
        let mut applicable: Vec<&DiscountEntry> = self
            .discounts
            .iter()
            .filter(|d| d.is_valid_on(date) && d.applies_to_value(price))
            .collect();

        applicable.sort_by_key(|d| d.discount_type.priority());

        if self.cumulative {
            // Apply all discounts cumulatively
            let mut running_price = *price;
            for discount in applicable {
                let discount_amount = discount.calculate(&running_price, quantity);
                if !discount_amount.is_zero() {
                    let percentage = match &discount.rule {
                        DiscountRule::Percentage(pct) => Some(*pct),
                        _ => None,
                    };

                    applied_discounts.push(AppliedDiscount {
                        id: discount.id.clone(),
                        name: discount.name.clone(),
                        discount_type: discount.discount_type.clone(),
                        rule_description: discount.rule.description(),
                        amount: discount_amount,
                        percentage,
                    });

                    total_discount = total_discount + discount_amount;
                    running_price = running_price - discount_amount;
                }
            }
        } else {
            // Apply only the best (largest) discount
            let mut best_discount: Option<(Money, &DiscountEntry)> = None;

            for discount in applicable {
                let discount_amount = discount.calculate(price, quantity);
                if let Some((ref best_amount, _)) = best_discount {
                    if discount_amount.amount() > best_amount.amount() {
                        best_discount = Some((discount_amount, discount));
                    }
                } else if !discount_amount.is_zero() {
                    best_discount = Some((discount_amount, discount));
                }
            }

            if let Some((amount, discount)) = best_discount {
                let percentage = match &discount.rule {
                    DiscountRule::Percentage(pct) => Some(*pct),
                    _ => None,
                };

                applied_discounts.push(AppliedDiscount {
                    id: discount.id.clone(),
                    name: discount.name.clone(),
                    discount_type: discount.discount_type.clone(),
                    rule_description: discount.rule.description(),
                    amount,
                    percentage,
                });

                total_discount = amount;
            }
        }

        let discounted_price = *price - total_discount;

        DiscountResult {
            original_price: *price,
            discounts: applied_discounts,
            total_discount,
            discounted_price,
        }
    }

    /// Apply discounts using today's date
    pub fn apply_today(&self, price: &Money, quantity: u32) -> DiscountResult {
        self.apply(price, quantity, chrono::Local::now().date_naive())
    }
}

/// Builder for creating discount configurations
#[derive(Debug, Clone, Default)]
pub struct DiscountBuilder {
    id: String,
    name: String,
    description: String,
    discount_type: DiscountType,
    rule: Option<DiscountRule>,
    valid_from: Option<NaiveDate>,
    valid_to: Option<NaiveDate>,
    min_order_value: Option<Decimal>,
    conditions: HashMap<String, String>,
}

impl DiscountBuilder {
    /// Start building a new discount
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            discount_type: DiscountType::Vendor,
            ..Default::default()
        }
    }

    /// Set the name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Set the description
    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set the discount type
    pub fn discount_type(mut self, dt: DiscountType) -> Self {
        self.discount_type = dt;
        self
    }

    /// Set a percentage discount
    pub fn percentage(mut self, pct: f64) -> Self {
        self.rule = Some(DiscountRule::Percentage(
            Decimal::try_from(pct).unwrap_or_default(),
        ));
        self
    }

    /// Set a fixed amount discount
    pub fn fixed_amount(mut self, amount: Decimal) -> Self {
        self.rule = Some(DiscountRule::FixedAmount(amount));
        self
    }

    /// Set tiered percentage discount
    pub fn tiered(mut self, tiers: Vec<(u32, f64)>) -> Self {
        let decimal_tiers: Vec<(u32, Decimal)> = tiers
            .into_iter()
            .map(|(qty, pct)| (qty, Decimal::try_from(pct).unwrap_or_default()))
            .collect();
        self.rule = Some(DiscountRule::TieredPercentage(decimal_tiers));
        self
    }

    /// Set validity dates
    pub fn valid(mut self, from: NaiveDate, to: NaiveDate) -> Self {
        self.valid_from = Some(from);
        self.valid_to = Some(to);
        self
    }

    /// Set minimum order value
    pub fn min_order(mut self, value: Decimal) -> Self {
        self.min_order_value = Some(value);
        self
    }

    /// Add a condition
    pub fn condition(mut self, key: &str, value: &str) -> Self {
        self.conditions.insert(key.to_string(), value.to_string());
        self
    }

    /// Build the discount entry
    pub fn build(self) -> DiscountEntry {
        DiscountEntry {
            id: self.id,
            name: self.name,
            description: self.description,
            discount_type: self.discount_type,
            rule: self.rule.unwrap_or(DiscountRule::Percentage(Decimal::ZERO)),
            active: true,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            min_order_value: self.min_order_value,
            conditions: self.conditions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oap::currency::Currency;

    fn today() -> NaiveDate {
        chrono::Local::now().date_naive()
    }

    #[test]
    fn test_discount_type_name() {
        assert_eq!(DiscountType::Vendor.name(), "Vendor");
        assert_eq!(DiscountType::Customer.name(), "Customer");
        assert_eq!(DiscountType::Volume.name(), "Volume");
        assert_eq!(DiscountType::Promotional.name(), "Promotional");
        assert_eq!(DiscountType::Custom("Special".into()).name(), "Special");
    }

    #[test]
    fn test_discount_type_priority() {
        assert!(DiscountType::Vendor.priority() < DiscountType::Customer.priority());
        assert!(DiscountType::Volume.priority() < DiscountType::Promotional.priority());
    }

    #[test]
    fn test_discount_rule_percentage() {
        let rule = DiscountRule::Percentage(Decimal::from(10));
        let price = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let discount = rule.calculate(&price, 1);
        assert_eq!(discount.amount_cents(), 1000); // 10.00 EUR
    }

    #[test]
    fn test_discount_rule_fixed() {
        let rule = DiscountRule::FixedAmount(Decimal::from(5));
        let price = Money::from_cents(10000, Currency::EUR);
        let discount = rule.calculate(&price, 3);
        assert_eq!(discount.amount_cents(), 1500); // 5 * 3 = 15.00 EUR
    }

    #[test]
    fn test_discount_rule_tiered() {
        let rule = DiscountRule::TieredPercentage(vec![
            (1, Decimal::from(0)),
            (5, Decimal::from(5)),
            (10, Decimal::from(10)),
            (20, Decimal::from(15)),
        ]);
        let price = Money::from_cents(10000, Currency::EUR);

        // 1-4 units: 0%
        assert_eq!(rule.calculate(&price, 1).amount_cents(), 0);
        // 5-9 units: 5%
        assert_eq!(rule.calculate(&price, 5).amount_cents(), 500);
        // 10-19 units: 10%
        assert_eq!(rule.calculate(&price, 10).amount_cents(), 1000);
        // 20+ units: 15%
        assert_eq!(rule.calculate(&price, 25).amount_cents(), 1500);
    }

    #[test]
    fn test_discount_entry_percentage() {
        let entry = DiscountEntry::percentage("D1", "10% Off", DiscountType::Vendor, 10.0);
        assert_eq!(entry.id, "D1");
        assert_eq!(entry.name, "10% Off");
        assert!(entry.active);
    }

    #[test]
    fn test_discount_entry_fixed() {
        let entry =
            DiscountEntry::fixed_amount("D2", "5 EUR Off", DiscountType::Customer, Decimal::from(5));
        let price = Money::from_cents(10000, Currency::EUR);
        let discount = entry.calculate(&price, 1);
        assert_eq!(discount.amount_cents(), 500);
    }

    #[test]
    fn test_discount_entry_tiered() {
        let entry = DiscountEntry::tiered(
            "VOL1",
            "Volume Discount",
            DiscountType::Volume,
            vec![(5, 5.0), (10, 10.0)],
        );
        let price = Money::from_cents(10000, Currency::EUR);
        assert_eq!(entry.calculate(&price, 3).amount_cents(), 0);
        assert_eq!(entry.calculate(&price, 7).amount_cents(), 500);
        assert_eq!(entry.calculate(&price, 15).amount_cents(), 1000);
    }

    #[test]
    fn test_discount_entry_validity() {
        let from = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

        let entry = DiscountEntry::percentage("D1", "Test", DiscountType::Promotional, 10.0)
            .with_validity(from, to);

        assert!(entry.is_valid_on(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()));
        assert!(!entry.is_valid_on(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()));
        assert!(!entry.is_valid_on(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()));
    }

    #[test]
    fn test_discount_entry_min_order() {
        let entry = DiscountEntry::percentage("D1", "Test", DiscountType::Volume, 10.0)
            .with_min_order(Decimal::from(100));

        let small = Money::from_cents(5000, Currency::EUR); // 50 EUR
        let large = Money::from_cents(15000, Currency::EUR); // 150 EUR

        assert!(!entry.applies_to_value(&small));
        assert!(entry.applies_to_value(&large));
    }

    #[test]
    fn test_discount_entry_from_ocd() {
        let entry = DiscountEntry::from_ocd("D_SPECIAL", Decimal::from(25), "Special Discount");
        assert_eq!(entry.id, "D_SPECIAL");
        assert_eq!(entry.discount_type, DiscountType::OcdDiscount);
    }

    #[test]
    fn test_discount_manager_add_remove() {
        let mut manager = DiscountManager::new();
        assert!(manager.list().is_empty());

        manager.add(DiscountEntry::percentage("D1", "Test", DiscountType::Vendor, 10.0));
        assert_eq!(manager.list().len(), 1);
        assert!(manager.get("D1").is_some());

        assert!(manager.remove("D1"));
        assert!(manager.list().is_empty());
        assert!(!manager.remove("D1")); // Already removed
    }

    #[test]
    fn test_discount_manager_apply_cumulative() {
        let mut manager = DiscountManager::new();
        manager.add(DiscountEntry::percentage("D1", "10% Off", DiscountType::Vendor, 10.0));
        manager.add(DiscountEntry::percentage("D2", "5% Off", DiscountType::Customer, 5.0));

        let price = Money::from_cents(10000, Currency::EUR); // 100.00 EUR
        let result = manager.apply(&price, 1, today());

        assert_eq!(result.discounts.len(), 2);
        // 10% of 100 = 10, then 5% of 90 = 4.50, total = 14.50
        assert_eq!(result.discounted_price.amount_cents(), 8550);
    }

    #[test]
    fn test_discount_manager_apply_non_cumulative() {
        let mut manager = DiscountManager::non_cumulative();
        manager.add(DiscountEntry::percentage("D1", "10% Off", DiscountType::Vendor, 10.0));
        manager.add(DiscountEntry::percentage("D2", "5% Off", DiscountType::Customer, 5.0));

        let price = Money::from_cents(10000, Currency::EUR);
        let result = manager.apply(&price, 1, today());

        // Only best discount (10%) applied
        assert_eq!(result.discounts.len(), 1);
        assert_eq!(result.total_discount.amount_cents(), 1000);
        assert_eq!(result.discounted_price.amount_cents(), 9000);
    }

    #[test]
    fn test_discount_result_percentage() {
        let result = DiscountResult {
            original_price: Money::from_cents(10000, Currency::EUR),
            discounts: vec![],
            total_discount: Money::from_cents(2000, Currency::EUR),
            discounted_price: Money::from_cents(8000, Currency::EUR),
        };

        assert_eq!(result.discount_percentage(), Decimal::from(20));
    }

    #[test]
    fn test_discount_builder() {
        let discount = DiscountBuilder::new("B1")
            .name("Builder Discount")
            .description("Created with builder")
            .discount_type(DiscountType::Promotional)
            .percentage(15.0)
            .min_order(Decimal::from(50))
            .condition("category", "furniture")
            .build();

        assert_eq!(discount.id, "B1");
        assert_eq!(discount.name, "Builder Discount");
        assert_eq!(discount.discount_type, DiscountType::Promotional);
        assert!(discount.min_order_value.is_some());
        assert!(discount.conditions.contains_key("category"));
    }

    #[test]
    fn test_buy_x_get_y_free() {
        let rule = DiscountRule::BuyXGetYFree { buy: 3, free: 1 };
        // 6 items at 100 EUR each = 600 EUR total
        // Buy 3 get 1 free means: 6 items = 2 free items
        let unit_price = Money::from_cents(10000, Currency::EUR);
        let total_price = unit_price * 6;
        let discount = rule.calculate(&total_price, 6);
        // 2 free items * 100 EUR = 200 EUR discount
        assert_eq!(discount.amount_cents(), 20000);
    }

    #[test]
    fn test_discount_manager_active_on() {
        let mut manager = DiscountManager::new();

        let from = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2025, 6, 30).unwrap();

        manager.add(
            DiscountEntry::percentage("D1", "First Half", DiscountType::Promotional, 10.0)
                .with_validity(from, to),
        );
        manager.add(DiscountEntry::percentage("D2", "Always", DiscountType::Vendor, 5.0));

        let june = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        let july = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();

        assert_eq!(manager.active_on(june).len(), 2);
        assert_eq!(manager.active_on(july).len(), 1);
    }

    #[test]
    fn test_discount_rule_description() {
        assert_eq!(
            DiscountRule::Percentage(Decimal::from(10)).description(),
            "10%"
        );
        assert_eq!(
            DiscountRule::FixedAmount(Decimal::from(5)).description(),
            "5 off"
        );
        assert!(DiscountRule::BuyXGetYFree { buy: 2, free: 1 }
            .description()
            .contains("Buy 2"));
    }

    #[test]
    fn test_discount_serialization() {
        let entry = DiscountEntry::percentage("D1", "Test", DiscountType::Vendor, 10.0);
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("D1"));
        assert!(json.contains("Vendor"));

        let parsed: DiscountEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "D1");
    }

    #[test]
    fn test_applied_discount_serialization() {
        let applied = AppliedDiscount {
            id: "D1".to_string(),
            name: "Test".to_string(),
            discount_type: DiscountType::Vendor,
            rule_description: "10%".to_string(),
            amount: Money::from_cents(1000, Currency::EUR),
            percentage: Some(Decimal::from(10)),
        };

        let json = serde_json::to_string(&applied).unwrap();
        assert!(json.contains("D1"));
        assert!(json.contains("Vendor"));
    }

    #[test]
    fn test_discount_result_has_discounts() {
        let empty = DiscountResult {
            original_price: Money::from_cents(10000, Currency::EUR),
            discounts: vec![],
            total_discount: Money::zero(Currency::EUR),
            discounted_price: Money::from_cents(10000, Currency::EUR),
        };
        assert!(!empty.has_discounts());

        let with_discount = DiscountResult {
            original_price: Money::from_cents(10000, Currency::EUR),
            discounts: vec![AppliedDiscount {
                id: "D1".to_string(),
                name: "Test".to_string(),
                discount_type: DiscountType::Vendor,
                rule_description: "10%".to_string(),
                amount: Money::from_cents(1000, Currency::EUR),
                percentage: Some(Decimal::from(10)),
            }],
            total_discount: Money::from_cents(1000, Currency::EUR),
            discounted_price: Money::from_cents(9000, Currency::EUR),
        };
        assert!(with_discount.has_discounts());
    }
}
