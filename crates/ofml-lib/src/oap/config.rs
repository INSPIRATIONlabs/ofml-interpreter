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
    use crate::property::PropertyValue;

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

    #[test]
    fn test_configuration_with_properties() {
        let mut properties = PropertyManager::new();
        properties.set("HEIGHT", PropertyValue::Int(720));
        properties.set("DIAMETER", PropertyValue::Int(1200));

        let config = Configuration::with_properties(
            "ViTable_Round".to_string(),
            "vitra".to_string(),
            properties,
        );

        assert_eq!(config.article_id, "ViTable_Round");
        assert!(!config.properties.values.is_empty());
    }

    #[test]
    fn test_configuration_is_valid_with_properties() {
        let mut properties = PropertyManager::new();
        properties.set("HEIGHT", PropertyValue::Int(720));

        let config = Configuration::with_properties(
            "article".to_string(),
            "mfr".to_string(),
            properties,
        );

        assert!(config.is_valid());
    }

    #[test]
    fn test_configuration_is_valid_with_variant_code() {
        let mut config = Configuration::new("article".to_string(), "mfr".to_string());
        config.variant_code = "VARIANT".to_string();
        assert!(config.is_valid());
    }

    #[test]
    fn test_configuration_is_not_valid_empty() {
        let config = Configuration::new("article".to_string(), "mfr".to_string());
        assert!(!config.is_valid());
    }

    #[test]
    fn test_configuration_clone() {
        let mut config = Configuration::new("article".to_string(), "mfr".to_string());
        config.variant_code = "TEST".to_string();

        let cloned = config.clone();
        assert_eq!(cloned.article_id, "article");
        assert_eq!(cloned.variant_code, "TEST");
    }

    #[test]
    fn test_configuration_debug() {
        let config = Configuration::new("article".to_string(), "mfr".to_string());
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("Configuration"));
        assert!(debug_str.contains("article"));
    }

    #[test]
    fn test_configuration_export_with_properties() {
        let mut properties = PropertyManager::new();
        properties.set("HEIGHT", PropertyValue::Int(720));
        properties.set("WIDTH", PropertyValue::Float(100.5));
        properties.set("ACTIVE", PropertyValue::Bool(true));
        properties.set("NAME", PropertyValue::String("Test".to_string()));
        properties.set("COLOR", PropertyValue::Symbol("RED".to_string()));

        let config = Configuration::with_properties(
            "article".to_string(),
            "mfr".to_string(),
            properties,
        );

        let export = config.to_export_data();
        assert!(export.properties.contains_key("HEIGHT"));
        assert!(export.properties.contains_key("WIDTH"));
        assert!(export.properties.contains_key("ACTIVE"));
        assert!(export.properties.contains_key("NAME"));
        assert!(export.properties.contains_key("COLOR"));
    }

    #[test]
    fn test_configuration_export_no_price() {
        let config = Configuration::new("article".to_string(), "mfr".to_string());
        let export = config.to_export_data();
        assert!(export.base_price.is_none());
        assert!(export.surcharges.is_none());
        assert!(export.total_price.is_none());
        assert!(export.currency.is_none());
    }

    #[test]
    fn test_configuration_with_sub_articles() {
        let mut config = Configuration::new("parent".to_string(), "mfr".to_string());
        let child = Configuration::new("child".to_string(), "mfr".to_string());
        config.sub_articles.push(child);

        let export = config.to_export_data();
        assert_eq!(export.sub_articles.len(), 1);
        assert_eq!(export.sub_articles[0].article, "child");
    }

    #[test]
    fn test_configuration_created_at() {
        let before = Utc::now();
        let config = Configuration::new("article".to_string(), "mfr".to_string());
        let after = Utc::now();

        assert!(config.created_at >= before);
        assert!(config.created_at <= after);
    }

    #[test]
    fn test_configuration_export_with_price_and_surcharges() {
        use chrono::NaiveDate;
        use rust_decimal::Decimal;
        use super::super::{PriceResult, Surcharge};
        use std::str::FromStr;

        let mut config = Configuration::new("article".to_string(), "mfr".to_string());

        // Add surcharges
        let surcharges = vec![
            Surcharge {
                name: "Premium Finish".to_string(),
                amount: Decimal::from_str("150.00").unwrap(),
                is_percentage: false,
            },
            Surcharge {
                name: "Express Delivery".to_string(),
                amount: Decimal::from_str("10.0").unwrap(),
                is_percentage: true,
            },
        ];

        // Set price with surcharges
        config.price = Some(PriceResult::new(
            Decimal::from_str("1000.00").unwrap(),
            surcharges,
            "EUR".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            None,
        ));

        let export = config.to_export_data();

        // Check base price is set
        assert!(export.base_price.is_some());
        assert!(export.base_price.unwrap().contains("1.000"));

        // Check surcharges are exported
        assert!(export.surcharges.is_some());
        let surcharges = export.surcharges.unwrap();
        assert_eq!(surcharges.len(), 2);
        assert_eq!(surcharges[0].name, "Premium Finish");
        assert!(!surcharges[0].is_percentage);
        assert_eq!(surcharges[1].name, "Express Delivery");
        assert!(surcharges[1].is_percentage);

        // Check total and currency
        assert!(export.total_price.is_some());
        assert_eq!(export.currency, Some("EUR".to_string()));
        assert!(export.price_date.is_some());
    }
}
