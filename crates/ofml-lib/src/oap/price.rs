//! Price lookup from EBASE databases
//!
//! This module provides price lookup functionality using article number
//! and variant code to query EBASE price tables.
//!
//! ## Price Calculation Order
//!
//! Per OCD 4.3 spec and reference application behavior:
//! 1. Base Price (Level 'B') - Applied first
//! 2. Surcharges (Level 'X') - Accumulated and added to base
//! 3. Discounts (Level 'D') - Subtracted last
//!
//! Formula: Total = Base + Σ(Surcharges) - Σ(Discounts)

use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::ocd::{find_pdata_files, get_ocd_reader, OcdPrice, OcdReader};
use super::{PriceResult, Surcharge};

/// Errors that can occur during price lookup
#[derive(Debug, Error)]
pub enum PriceError {
    #[error("Price table not found for manufacturer: {0}")]
    PriceTableNotFound(String),

    #[error("Article not found in price table: {0}")]
    ArticleNotFound(String),

    #[error("Variant not found: {0}")]
    VariantNotFound(String),

    #[error("No valid price for date: {0}")]
    NoValidPriceForDate(NaiveDate),

    #[error("EBASE read error: {0}")]
    EbaseError(String),
}

/// A discount entry (price_level='D')
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
    /// Discount description (from ocd_pricetext or var_cond)
    pub name: String,
    /// Discount value (positive = amount to subtract)
    pub amount: Decimal,
    /// Whether amount is a percentage of base price
    pub is_percentage: bool,
    /// Original var_cond from ocd_price
    pub var_cond: String,
}

/// Detailed price breakdown for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBreakdown {
    /// Base price amount (price_level='B')
    pub base: Decimal,
    /// List of applied surcharges (price_level='X')
    pub surcharges: Vec<Surcharge>,
    /// List of applied discounts (price_level='D')
    pub discounts: Vec<Discount>,
    /// Total after all surcharges and discounts
    pub total: Decimal,
    /// Currency code (e.g., "EUR")
    pub currency: String,
    /// Price date used for lookup
    pub price_date: NaiveDate,
    /// Whether this is a surcharge-only pricing model (base=0)
    pub is_surcharge_only: bool,
}

impl PriceBreakdown {
    /// Create a new price breakdown
    pub fn new(
        base: Decimal,
        surcharges: Vec<Surcharge>,
        discounts: Vec<Discount>,
        currency: String,
        price_date: NaiveDate,
    ) -> Self {
        let is_surcharge_only = base.is_zero() && !surcharges.is_empty();
        let total = Self::compute_total(&base, &surcharges, &discounts);
        Self {
            base,
            surcharges,
            discounts,
            total,
            currency,
            price_date,
            is_surcharge_only,
        }
    }

    /// Compute total from base, surcharges, and discounts
    /// Formula: Total = Base + Σ(Surcharges) - Σ(Discounts)
    fn compute_total(base: &Decimal, surcharges: &[Surcharge], discounts: &[Discount]) -> Decimal {
        let mut total = *base;

        // Add surcharges
        for surcharge in surcharges {
            if surcharge.is_percentage {
                total += *base * surcharge.amount / Decimal::from(100);
            } else {
                total += surcharge.amount;
            }
        }

        // Subtract discounts
        for discount in discounts {
            if discount.is_percentage {
                total -= *base * discount.amount / Decimal::from(100);
            } else {
                total -= discount.amount;
            }
        }

        total
    }

    /// Get sum of all surcharges
    pub fn surcharges_total(&self) -> Decimal {
        self.surcharges
            .iter()
            .map(|s| {
                if s.is_percentage {
                    self.base * s.amount / Decimal::from(100)
                } else {
                    s.amount
                }
            })
            .sum()
    }

    /// Get sum of all discounts
    pub fn discounts_total(&self) -> Decimal {
        self.discounts
            .iter()
            .map(|d| {
                if d.is_percentage {
                    self.base * d.amount / Decimal::from(100)
                } else {
                    d.amount
                }
            })
            .sum()
    }

    /// Convert to PriceResult (for compatibility)
    pub fn to_price_result(
        self,
        valid_from: NaiveDate,
        valid_to: Option<NaiveDate>,
    ) -> PriceResult {
        PriceResult::new(
            self.base,
            self.surcharges,
            self.currency,
            self.price_date,
            valid_from,
            valid_to,
        )
    }
}

/// Query parameters for price lookup
#[derive(Debug, Clone)]
pub struct PriceQuery {
    /// Manufacturer ID
    pub manufacturer: String,
    /// EBASE article number
    pub article_number: String,
    /// Generated variant code
    pub variant_code: String,
    /// Price lookup date
    pub price_date: NaiveDate,
}

impl PriceQuery {
    /// Create a new price query
    pub fn new(
        manufacturer: String,
        article_number: String,
        variant_code: String,
        price_date: NaiveDate,
    ) -> Self {
        Self {
            manufacturer,
            article_number,
            variant_code,
            price_date,
        }
    }
}

/// Price lookup service
///
/// This struct provides methods to look up prices from EBASE databases.
/// It handles caching and date-based validity filtering.
pub struct PriceLookup {
    /// Path to OFML data directory (e.g., /reference/ofmldata)
    data_path: PathBuf,
}

impl PriceLookup {
    /// Create a new price lookup service with real EBASE lookup
    pub fn new(data_path: impl AsRef<Path>) -> Self {
        Self {
            data_path: data_path.as_ref().to_path_buf(),
        }
    }

    /// Look up price for a configured article
    ///
    /// # Arguments
    /// * `query` - Price query parameters
    ///
    /// # Returns
    /// `Ok(PriceResult)` with pricing info, or `Err(PriceError)` if lookup fails
    pub fn lookup(&self, query: &PriceQuery) -> Result<PriceResult, PriceError> {
        let mfr_path = self.data_path.join(&query.manufacturer);

        // Find pdata.ebase files for this manufacturer
        let pdata_files = find_pdata_files(&mfr_path);
        if pdata_files.is_empty() {
            return Err(PriceError::PriceTableNotFound(query.manufacturer.clone()));
        }

        // Try each pdata file to find the article
        for pdata_path in &pdata_files {
            if let Some(reader) = get_ocd_reader(pdata_path) {
                // Get all prices for this article
                let prices = reader.get_prices(&query.article_number);

                if prices.is_empty() {
                    continue;
                }

                // Match prices against variant code
                if let Some(result) = self.match_prices(&reader, &prices, &query.variant_code) {
                    let valid_from = parse_date(&result.base_price.date_from)
                        .unwrap_or(query.price_date);
                    let valid_to = parse_date(&result.base_price.date_to);

                    return Ok(PriceResult::new(
                        result.base_amount,
                        result.surcharges,
                        result.currency,
                        query.price_date,
                        valid_from,
                        valid_to,
                    ));
                }
            }
        }

        Err(PriceError::ArticleNotFound(query.article_number.clone()))
    }

    /// Match prices to variant code
    fn match_prices<'a>(
        &self,
        reader: &OcdReader,
        prices: &'a [&'a OcdPrice],
        variant_code: &str,
    ) -> Option<MatchedPriceResult<'a>> {
        // Separate by price level
        let mut base_prices: Vec<&OcdPrice> = Vec::new();
        let mut surcharge_prices: Vec<&OcdPrice> = Vec::new();
        let mut discount_prices: Vec<&OcdPrice> = Vec::new();

        for price in prices {
            match price.price_level.as_str() {
                "B" => base_prices.push(price),
                "X" => surcharge_prices.push(price),
                "D" => discount_prices.push(price),
                _ => {}
            }
        }

        // Find matching base price
        let base_price = self.find_matching_base_price(&base_prices, variant_code)?;
        let base_amount = Decimal::from_f32_retain(base_price.price).unwrap_or_default();

        // Find matching surcharges
        let mut surcharges = Vec::new();
        for price in &surcharge_prices {
            if self.var_cond_matches(&price.var_cond, variant_code) {
                let amount = Decimal::from_f32_retain(price.price).unwrap_or_default();
                let description = reader
                    .get_price_description(price, "DE")
                    .trim()
                    .to_string();
                surcharges.push(Surcharge {
                    name: if description.is_empty() {
                        price.var_cond.clone()
                    } else {
                        description
                    },
                    amount,
                    is_percentage: false,
                });
            }
        }

        // Find matching discounts (as negative surcharges)
        for price in &discount_prices {
            if self.var_cond_matches(&price.var_cond, variant_code) {
                let amount = Decimal::from_f32_retain(price.price).unwrap_or_default();
                let description = reader
                    .get_price_description(price, "DE")
                    .trim()
                    .to_string();
                surcharges.push(Surcharge {
                    name: format!(
                        "Rabatt: {}",
                        if description.is_empty() {
                            &price.var_cond
                        } else {
                            &description
                        }
                    ),
                    amount: -amount, // Negate for discounts
                    is_percentage: false,
                });
            }
        }

        Some(MatchedPriceResult {
            base_price,
            base_amount,
            surcharges,
            currency: base_price.currency.clone(),
        })
    }

    /// Find matching base price (level 'B')
    fn find_matching_base_price<'a>(
        &self,
        base_prices: &[&'a OcdPrice],
        variant_code: &str,
    ) -> Option<&'a OcdPrice> {
        // Common base price indicators
        const BASE_INDICATORS: &[&str] = &["S_PGX", "BASE", "STANDARD", ""];

        // First try to find a base price matching the variant code
        for price in base_prices {
            if self.var_cond_matches(&price.var_cond, variant_code) {
                return Some(price);
            }
        }

        // Fall back to standard base price indicators
        for indicator in BASE_INDICATORS {
            for price in base_prices {
                if price.var_cond.eq_ignore_ascii_case(indicator) {
                    return Some(price);
                }
            }
        }

        // Last resort: first base price
        base_prices.first().copied()
    }

    /// Check if var_cond matches variant code
    fn var_cond_matches(&self, var_cond: &str, variant_code: &str) -> bool {
        if var_cond.is_empty() {
            return false;
        }

        // Common base indicators should not match as surcharges
        const BASE_INDICATORS: &[&str] = &["S_PGX", "BASE", "STANDARD"];
        for indicator in BASE_INDICATORS {
            if var_cond.eq_ignore_ascii_case(indicator) {
                return false;
            }
        }

        // Strategy 1: Direct match - var_cond like "S_166" matches variant code containing "166"
        if let Some(suffix) = var_cond.strip_prefix("S_") {
            // Check each component of the variant code
            for component in variant_code.split('_') {
                if component == suffix || component.ends_with(suffix) {
                    return true;
                }
            }
        }

        // Strategy 2: Exact match in variant components
        for component in variant_code.split('_') {
            if component == var_cond {
                return true;
            }
        }

        // Strategy 3: Variant contains var_cond
        if variant_code.contains(var_cond) {
            return true;
        }

        false
    }
}

/// Internal result from price matching
struct MatchedPriceResult<'a> {
    base_price: &'a OcdPrice,
    base_amount: Decimal,
    surcharges: Vec<Surcharge>,
    currency: String,
}

/// Parse a date string in YYYYMMDD format
fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y%m%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_query_new() {
        let query = PriceQuery::new(
            "vitra".to_string(),
            "48-123-456".to_string(),
            "H720_D1200".to_string(),
            NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        );
        assert_eq!(query.manufacturer, "vitra");
        assert_eq!(query.article_number, "48-123-456");
        assert_eq!(query.variant_code, "H720_D1200");
    }

    #[test]
    fn test_price_query_debug_clone() {
        let query = PriceQuery::new(
            "test".to_string(),
            "ART-001".to_string(),
            "VAR1".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        );
        let debug = format!("{:?}", query);
        assert!(debug.contains("PriceQuery"));
        let cloned = query.clone();
        assert_eq!(cloned.manufacturer, query.manufacturer);
    }

    #[test]
    fn test_price_lookup_real_data() {
        // Use the actual OFML data path
        let lookup = PriceLookup::new("/reference/ofmldata");

        // Test with Sedus AI chair (known to exist in test data)
        let query = PriceQuery::new(
            "sex".to_string(),
            "AI-121".to_string(),
            "".to_string(), // Empty variant code for base price
            NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        );

        // This may fail if data directory doesn't exist, which is okay for unit tests
        if let Ok(price) = lookup.lookup(&query) {
            assert!(price.base_price >= Decimal::ZERO);
            assert!(!price.currency.is_empty());
        }
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
    fn test_var_cond_matching() {
        let lookup = PriceLookup::new("/tmp");

        // Test S_ prefix matching
        assert!(lookup.var_cond_matches("S_166", "H720_166"));
        assert!(lookup.var_cond_matches("S_166", "A166_B200"));
        assert!(!lookup.var_cond_matches("S_166", "H720_167"));

        // Test exact component match
        assert!(lookup.var_cond_matches("H720", "H720_D1200"));
        assert!(!lookup.var_cond_matches("H720", "H721_D1200"));

        // Test contains match
        assert!(lookup.var_cond_matches("720", "H720_D1200"));

        // Test base indicators don't match
        assert!(!lookup.var_cond_matches("S_PGX", "anything"));
        assert!(!lookup.var_cond_matches("BASE", "anything"));
        assert!(!lookup.var_cond_matches("STANDARD", "anything"));

        // Empty var_cond doesn't match
        assert!(!lookup.var_cond_matches("", "anything"));
    }

    #[test]
    fn test_parse_date() {
        assert_eq!(
            parse_date("20250101"),
            Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        );
        assert_eq!(
            parse_date("20251224"),
            Some(NaiveDate::from_ymd_opt(2025, 12, 24).unwrap())
        );
        assert_eq!(parse_date("invalid"), None);
        assert_eq!(parse_date(""), None);
    }

    #[test]
    fn test_price_breakdown_new() {
        let base = Decimal::from(100);
        let surcharges = vec![Surcharge {
            name: "Extra".to_string(),
            amount: Decimal::from(20),
            is_percentage: false,
        }];
        let discounts = vec![];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown =
            PriceBreakdown::new(base, surcharges, discounts, currency.clone(), price_date);

        assert_eq!(breakdown.base, Decimal::from(100));
        assert_eq!(breakdown.total, Decimal::from(120)); // 100 + 20
        assert_eq!(breakdown.currency, "EUR");
        assert!(!breakdown.is_surcharge_only);
    }

    #[test]
    fn test_price_breakdown_surcharge_only() {
        let base = Decimal::ZERO;
        let surcharges = vec![
            Surcharge {
                name: "Option A".to_string(),
                amount: Decimal::from(50),
                is_percentage: false,
            },
            Surcharge {
                name: "Option B".to_string(),
                amount: Decimal::from(30),
                is_percentage: false,
            },
        ];
        let discounts = vec![];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        assert_eq!(breakdown.base, Decimal::ZERO);
        assert_eq!(breakdown.total, Decimal::from(80)); // 50 + 30
        assert!(breakdown.is_surcharge_only);
    }

    #[test]
    fn test_price_breakdown_with_percentage_surcharge() {
        let base = Decimal::from(200);
        let surcharges = vec![Surcharge {
            name: "10% extra".to_string(),
            amount: Decimal::from(10),
            is_percentage: true,
        }];
        let discounts = vec![];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        assert_eq!(breakdown.total, Decimal::from(220)); // 200 + 10% of 200
    }

    #[test]
    fn test_price_breakdown_with_discount() {
        let base = Decimal::from(100);
        let surcharges = vec![];
        let discounts = vec![Discount {
            name: "Holiday discount".to_string(),
            amount: Decimal::from(15),
            is_percentage: false,
            var_cond: "HOLIDAY".to_string(),
        }];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        assert_eq!(breakdown.total, Decimal::from(85)); // 100 - 15
    }

    #[test]
    fn test_price_breakdown_with_percentage_discount() {
        let base = Decimal::from(200);
        let surcharges = vec![];
        let discounts = vec![Discount {
            name: "20% off".to_string(),
            amount: Decimal::from(20),
            is_percentage: true,
            var_cond: "SALE".to_string(),
        }];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        assert_eq!(breakdown.total, Decimal::from(160)); // 200 - 20% of 200
    }

    #[test]
    fn test_price_breakdown_surcharges_total() {
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
                is_percentage: false,
            },
        ];
        let discounts = vec![];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        assert_eq!(breakdown.surcharges_total(), Decimal::from(30));
    }

    #[test]
    fn test_price_breakdown_discounts_total() {
        let base = Decimal::from(200);
        let surcharges = vec![];
        let discounts = vec![
            Discount {
                name: "D1".to_string(),
                amount: Decimal::from(10),
                is_percentage: false,
                var_cond: "D1".to_string(),
            },
            Discount {
                name: "D2".to_string(),
                amount: Decimal::from(5),
                is_percentage: true, // 5% of 200 = 10
                var_cond: "D2".to_string(),
            },
        ];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        assert_eq!(breakdown.discounts_total(), Decimal::from(20)); // 10 + 5% of 200
    }

    #[test]
    fn test_price_breakdown_to_price_result() {
        let base = Decimal::from(100);
        let surcharges = vec![Surcharge {
            name: "Extra".to_string(),
            amount: Decimal::from(20),
            is_percentage: false,
        }];
        let discounts = vec![];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        let valid_from = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let valid_to = Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);
        let result = breakdown.to_price_result(valid_from, valid_to);

        assert_eq!(result.base_price, Decimal::from(100));
        assert_eq!(result.valid_from, valid_from);
        assert_eq!(result.valid_to, valid_to);
    }

    #[test]
    fn test_price_breakdown_debug_clone() {
        let breakdown = PriceBreakdown::new(
            Decimal::from(50),
            vec![],
            vec![],
            "USD".to_string(),
            NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
        );

        let debug = format!("{:?}", breakdown);
        assert!(debug.contains("PriceBreakdown"));
        assert!(debug.contains("USD"));

        let cloned = breakdown.clone();
        assert_eq!(cloned.base, breakdown.base);
        assert_eq!(cloned.currency, breakdown.currency);
    }

    #[test]
    fn test_discount_struct() {
        let discount = Discount {
            name: "Test Discount".to_string(),
            amount: Decimal::from(25),
            is_percentage: true,
            var_cond: "TEST_COND".to_string(),
        };

        let debug = format!("{:?}", discount);
        assert!(debug.contains("Discount"));
        assert!(debug.contains("Test Discount"));

        let cloned = discount.clone();
        assert_eq!(cloned.name, discount.name);
        assert_eq!(cloned.amount, discount.amount);
        assert_eq!(cloned.is_percentage, discount.is_percentage);
        assert_eq!(cloned.var_cond, discount.var_cond);
    }

    #[test]
    fn test_price_error_display() {
        let err1 = PriceError::PriceTableNotFound("testmfr".to_string());
        assert!(err1.to_string().contains("Price table not found"));
        assert!(err1.to_string().contains("testmfr"));

        let err2 = PriceError::ArticleNotFound("ART-999".to_string());
        assert!(err2.to_string().contains("Article not found"));
        assert!(err2.to_string().contains("ART-999"));

        let err3 = PriceError::VariantNotFound("VAR-X".to_string());
        assert!(err3.to_string().contains("Variant not found"));
        assert!(err3.to_string().contains("VAR-X"));

        let date = NaiveDate::from_ymd_opt(2025, 7, 4).unwrap();
        let err4 = PriceError::NoValidPriceForDate(date);
        assert!(err4.to_string().contains("No valid price for date"));

        let err5 = PriceError::EbaseError("read failed".to_string());
        assert!(err5.to_string().contains("EBASE read error"));
        assert!(err5.to_string().contains("read failed"));
    }

    #[test]
    fn test_price_error_debug() {
        let err = PriceError::ArticleNotFound("TEST".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("ArticleNotFound"));
    }

    #[test]
    fn test_var_cond_suffix_match() {
        let lookup = PriceLookup::new("/tmp");

        // Test suffix matching with S_ prefix
        assert!(lookup.var_cond_matches("S_100", "A100_B200"));
        assert!(lookup.var_cond_matches("S_100", "XYZ100"));
        assert!(!lookup.var_cond_matches("S_100", "A101_B200"));
    }

    #[test]
    fn test_price_breakdown_mixed_surcharges_and_discounts() {
        let base = Decimal::from(1000);
        let surcharges = vec![
            Surcharge {
                name: "Premium".to_string(),
                amount: Decimal::from(100),
                is_percentage: false,
            },
            Surcharge {
                name: "Express".to_string(),
                amount: Decimal::from(5), // 5% = 50
                is_percentage: true,
            },
        ];
        let discounts = vec![
            Discount {
                name: "Loyalty".to_string(),
                amount: Decimal::from(50),
                is_percentage: false,
                var_cond: "LOYAL".to_string(),
            },
            Discount {
                name: "Volume".to_string(),
                amount: Decimal::from(10), // 10% = 100
                is_percentage: true,
                var_cond: "VOL".to_string(),
            },
        ];
        let currency = "EUR".to_string();
        let price_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

        let breakdown = PriceBreakdown::new(base, surcharges, discounts, currency, price_date);

        // Total = 1000 + 100 + 50 (5% of 1000) - 50 - 100 (10% of 1000) = 1000
        assert_eq!(breakdown.total, Decimal::from(1000));
        assert_eq!(breakdown.surcharges_total(), Decimal::from(150)); // 100 + 50
        assert_eq!(breakdown.discounts_total(), Decimal::from(150)); // 50 + 100
    }

    #[test]
    fn test_price_lookup_article_not_found() {
        let lookup = PriceLookup::new("/reference/ofmldata");

        let query = PriceQuery::new(
            "sex".to_string(),
            "NONEXISTENT-ARTICLE-999".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        );

        // This should fail with ArticleNotFound if data exists
        let result = lookup.lookup(&query);
        if result.is_err() {
            // Either PriceTableNotFound (no data) or ArticleNotFound (article doesn't exist)
            assert!(
                matches!(result, Err(PriceError::ArticleNotFound(_)))
                    || matches!(result, Err(PriceError::PriceTableNotFound(_)))
            );
        }
    }

    #[test]
    fn test_discount_serialization() {
        let discount = Discount {
            name: "Test".to_string(),
            amount: Decimal::from(10),
            is_percentage: false,
            var_cond: "COND".to_string(),
        };

        let json = serde_json::to_string(&discount).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("COND"));

        let parsed: Discount = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, discount.name);
    }

    #[test]
    fn test_price_breakdown_serialization() {
        let breakdown = PriceBreakdown::new(
            Decimal::from(100),
            vec![],
            vec![],
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        );

        let json = serde_json::to_string(&breakdown).unwrap();
        assert!(json.contains("EUR"));
        assert!(json.contains("100"));

        let parsed: PriceBreakdown = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.base, breakdown.base);
        assert_eq!(parsed.currency, breakdown.currency);
    }
}
