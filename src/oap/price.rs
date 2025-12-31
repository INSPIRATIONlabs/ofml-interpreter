//! Price lookup from EBASE databases
//!
//! This module provides price lookup functionality using article number
//! and variant code to query EBASE price tables.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use thiserror::Error;

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
    // Cache and state will be added when EBASE integration is complete
}

impl PriceLookup {
    /// Create a new price lookup service
    pub fn new() -> Self {
        Self {}
    }

    /// Look up price for a configured article
    ///
    /// # Arguments
    /// * `query` - Price query parameters
    ///
    /// # Returns
    /// `Ok(PriceResult)` with pricing info, or `Err(PriceError)` if lookup fails
    pub fn lookup(&self, query: &PriceQuery) -> Result<PriceResult, PriceError> {
        // TODO: Implement actual EBASE lookup
        // For now, return a mock price for testing
        self.mock_lookup(query)
    }

    /// Mock price lookup for testing
    fn mock_lookup(&self, query: &PriceQuery) -> Result<PriceResult, PriceError> {
        // Generate a deterministic mock price based on query parameters
        let base_amount = (query.article_number.len() as i64 * 100 + 1000) * 100;
        let base_price = Decimal::new(base_amount, 2);

        // Add a mock surcharge
        let surcharges = if !query.variant_code.is_empty() {
            vec![Surcharge {
                name: format!("Variante: {}", query.variant_code),
                amount: Decimal::new(4500, 2), // 45.00
                is_percentage: false,
            }]
        } else {
            vec![]
        };

        Ok(PriceResult::new(
            base_price,
            surcharges,
            "EUR".to_string(),
            query.price_date,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            None,
        ))
    }
}

impl Default for PriceLookup {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_price_lookup_mock() {
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
        assert!(!price.surcharges.is_empty());
    }
}
