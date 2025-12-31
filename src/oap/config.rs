//! Configuration state management for OAP articles
//!
//! This module provides the Configuration struct that represents
//! a configured product instance with property values.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::{ExportData, ExportSurcharge, PriceResult};
use crate::oap::format_german_price;
use crate::property::PropertyManager;

/// Represents a configured product instance with property values.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Source article ID
    pub article_id: String,
    /// Parent manufacturer
    pub manufacturer_id: String,
    /// Article number from EBASE
    pub article_number: Option<String>,
    /// Property values and states
    pub properties: PropertyManager,
    /// Generated variant code
    pub variant_code: String,
    /// Looked up price (optional)
    pub price: Option<PriceResult>,
    /// Child configurations (sub-articles)
    pub sub_articles: Vec<Configuration>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Configuration {
    /// Create a new configuration for an article
    pub fn new(article_id: String, manufacturer_id: String) -> Self {
        Self {
            article_id,
            manufacturer_id,
            article_number: None,
            properties: PropertyManager::new(),
            variant_code: String::new(),
            price: None,
            sub_articles: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Create a new configuration with property manager
    pub fn with_properties(
        article_id: String,
        manufacturer_id: String,
        properties: PropertyManager,
    ) -> Self {
        Self {
            article_id,
            manufacturer_id,
            article_number: None,
            properties,
            variant_code: String::new(),
            price: None,
            sub_articles: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Update the variant code based on current property values
    pub fn update_variant_code(&mut self) {
        self.variant_code = super::variant::generate_variant_code(&self.properties);
    }

    /// Check if the configuration is valid (all required properties set)
    pub fn is_valid(&self) -> bool {
        // For now, consider valid if we have a non-empty variant code
        // or at least some properties set
        !self.variant_code.is_empty() || !self.properties.values.is_empty()
    }

    /// Convert to export data format
    pub fn to_export_data(&self) -> ExportData {
        let mut properties = HashMap::new();

        for (name, value) in &self.properties.values {
            let json_value = match value {
                crate::property::PropertyValue::Bool(b) => serde_json::Value::Bool(*b),
                crate::property::PropertyValue::Int(i) => {
                    serde_json::Value::Number(serde_json::Number::from(*i))
                }
                crate::property::PropertyValue::Float(f) => serde_json::Value::Number(
                    serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0)),
                ),
                crate::property::PropertyValue::String(s) => serde_json::Value::String(s.clone()),
                crate::property::PropertyValue::Symbol(s) => serde_json::Value::String(s.clone()),
            };
            properties.insert(name.clone(), json_value);
        }

        let (base_price, surcharges, total_price, currency, price_date) =
            if let Some(ref price) = self.price {
                (
                    Some(format_german_price(price.base_price)),
                    Some(
                        price
                            .surcharges
                            .iter()
                            .map(|s| ExportSurcharge {
                                name: s.name.clone(),
                                amount: format_german_price(s.amount),
                                is_percentage: s.is_percentage,
                            })
                            .collect(),
                    ),
                    Some(format_german_price(price.total_price)),
                    Some(price.currency.clone()),
                    Some(price.price_date.format("%Y-%m-%d").to_string()),
                )
            } else {
                (None, None, None, None, None)
            };

        ExportData {
            manufacturer: self.manufacturer_id.clone(),
            article: self.article_id.clone(),
            article_number: self.article_number.clone(),
            variant_code: self.variant_code.clone(),
            properties,
            base_price,
            surcharges,
            total_price,
            currency,
            price_date,
            sub_articles: self
                .sub_articles
                .iter()
                .map(|s| s.to_export_data())
                .collect(),
            exported_at: Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_new() {
        let config = Configuration::new("ViTable_Round".to_string(), "vitra".to_string());
        assert_eq!(config.article_id, "ViTable_Round");
        assert_eq!(config.manufacturer_id, "vitra");
        assert!(config.variant_code.is_empty());
        assert!(config.price.is_none());
        assert!(config.sub_articles.is_empty());
    }

    #[test]
    fn test_configuration_to_export_data() {
        let mut config = Configuration::new("ViTable_Round".to_string(), "vitra".to_string());
        config.variant_code = "H720_D1200".to_string();
        config.article_number = Some("48-123-456".to_string());

        let export = config.to_export_data();
        assert_eq!(export.manufacturer, "vitra");
        assert_eq!(export.article, "ViTable_Round");
        assert_eq!(export.variant_code, "H720_D1200");
        assert!(!export.exported_at.is_empty());
    }
}
