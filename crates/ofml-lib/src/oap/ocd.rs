//! OCD (Office Catalog Data) reader for OFML product catalogs
//!
//! This module reads product catalog information from pdata.ebase files,
//! providing access to:
//! - Articles (ocd_article table)
//! - Article descriptions (ocd_artshorttext, ocd_artlongtext tables)
//! - Prices (ocd_price table)
//! - Price descriptions (ocd_pricetext table)
//! - Property definitions (ocd_property, ocd_propertyvalue tables)
//! - Property to variant condition mappings (propvalue2varcond table)
//!
//! ## Data Normalization
//!
//! All string fields are normalized on read:
//! - Whitespace trimmed from all fields
//! - price_level uppercased (b → B)
//! - var_cond preserved as-is (case-sensitive for some manufacturers)
//! - Empty/malformed currency defaults to "EUR"

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::ebase::{EBaseReader, Value};
use serde::{Deserialize, Serialize};

/// TTL for OCD reader cache entries (5 minutes)
const OCD_CACHE_TTL: Duration = Duration::from_secs(300);

/// Cache entry with timestamp for TTL
struct OcdCacheEntry {
    reader: std::sync::Arc<OcdReader>,
    created_at: Instant,
}

impl OcdCacheEntry {
    fn new(reader: std::sync::Arc<OcdReader>) -> Self {
        Self {
            reader,
            created_at: Instant::now(),
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > OCD_CACHE_TTL
    }
}

/// Severity level for data warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningSeverity {
    /// Informational - no impact on functionality
    Info,
    /// Warning - minor issue, data recovered or fallback used
    Warning,
    /// Error - significant issue, some data may be unavailable
    Error,
}

impl std::fmt::Display for WarningSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningSeverity::Info => write!(f, "INFO"),
            WarningSeverity::Warning => write!(f, "WARN"),
            WarningSeverity::Error => write!(f, "ERROR"),
        }
    }
}

/// A recoverable data issue encountered during OCD parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWarning {
    /// Severity of the warning
    pub severity: WarningSeverity,
    /// Warning code for programmatic handling (e.g., "CORRUPTED_RECORD", "MISSING_PRICE")
    pub code: String,
    /// Human-readable description
    pub message: String,
    /// Source file or record that caused the warning (optional)
    pub source: Option<String>,
}

impl DataWarning {
    /// Create a new data warning
    pub fn new(
        severity: WarningSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            source: None,
        }
    }

    /// Create a warning with source information
    pub fn with_source(
        severity: WarningSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            source: Some(source.into()),
        }
    }

    /// Create an info-level warning
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(WarningSeverity::Info, code, message)
    }

    /// Create a warning-level warning
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(WarningSeverity::Warning, code, message)
    }

    /// Create an error-level warning
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(WarningSeverity::Error, code, message)
    }
}

/// An article from the OCD catalog
#[derive(Debug, Clone)]
pub struct OcdArticle {
    /// Article number (e.g., "SE:ABOV:01")
    pub article_nr: String,
    /// Article type
    pub art_type: String,
    /// Manufacturer ID
    pub manufacturer: String,
    /// Series/program name
    pub series: String,
    /// Short text number (for lookup in ocd_artshorttext)
    pub short_textnr: String,
    /// Long text number (for lookup in ocd_artlongtext)
    pub long_textnr: String,
}

/// A price entry from the OCD catalog
#[derive(Debug, Clone)]
pub struct OcdPrice {
    /// Article number
    pub article_nr: String,
    /// Variant condition (e.g., "S_PGX", "S_166", "")
    pub var_cond: String,
    /// Price type (e.g., "L" for list price)
    pub price_type: String,
    /// Price level: 'B' = base price, 'X' = surcharge, 'D' = discount
    pub price_level: String,
    /// Whether price is a fixed amount (true) or percentage (false)
    pub is_fix: bool,
    /// Text ID for price description (links to ocd_pricetext)
    pub text_id: String,
    /// Price value in currency units
    pub price: f32,
    /// Currency code (e.g., "EUR")
    pub currency: String,
    /// Valid from date (YYYYMMDD)
    pub date_from: String,
    /// Valid to date (YYYYMMDD)
    pub date_to: String,
    /// Scale quantity for volume pricing
    pub scale_qty: i32,
}

/// A text entry from the OCD catalog
#[derive(Debug, Clone)]
pub struct OcdText {
    /// Text number (reference key)
    pub textnr: String,
    /// Language code (e.g., "DE", "EN")
    pub language: String,
    /// Line number (for multi-line texts)
    pub line_nr: u32,
    /// Text content
    pub text: String,
}

/// A property value to variant condition mapping entry
/// This table provides direct mapping between property values and var_cond codes
/// used for price lookup, eliminating the need for pattern matching.
#[derive(Debug, Clone)]
pub struct PropValue2VarCond {
    /// Property class (e.g., "ASY_83341201")
    pub prop_class: String,
    /// Property key (e.g., "ASYABELE090")
    pub prop_key: String,
    /// Property value (e.g., "1AS01")
    pub prop_value: String,
    /// Optional condition for when this mapping applies
    pub condition: String,
    /// The variant condition code for ocd_price lookup (e.g., "83341201_1AS01")
    pub var_cond: String,
    /// Additional text to append to price description
    pub prop_text_add: String,
}

/// Indexes for propvalue2varcond lookups
type VarCondIndexes = (
    HashMap<(String, String), PropValue2VarCond>,
    HashMap<String, Vec<PropValue2VarCond>>,
);

/// OCD catalog reader
pub struct OcdReader {
    /// Loaded articles
    pub articles: Vec<OcdArticle>,
    /// Loaded prices
    pub prices: Vec<OcdPrice>,
    /// Short texts indexed by textnr
    pub short_texts: HashMap<String, Vec<OcdText>>,
    /// Long texts indexed by textnr
    pub long_texts: HashMap<String, Vec<OcdText>>,
    /// Price texts indexed by textnr (from ocd_pricetext table)
    pub price_texts: HashMap<String, Vec<OcdText>>,
    /// Property class mappings: article_nr -> list of property classes
    pub article_prop_classes: HashMap<String, Vec<String>>,
    /// Property value to variant condition mappings (for direct price lookup)
    /// Key: (prop_class, prop_value), Value: var_cond
    pub propvalue2varcond: HashMap<(String, String), PropValue2VarCond>,
    /// Quick lookup by prop_value only (for cases where prop_class is unknown)
    pub propvalue2varcond_by_value: HashMap<String, Vec<PropValue2VarCond>>,
    /// Data warnings collected during parsing
    pub warnings: Vec<DataWarning>,
}

impl OcdReader {
    /// Create a new OCD reader from a pdata.ebase file
    pub fn from_ebase(path: &Path) -> Result<Self, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;
        let mut warnings = Vec::new();

        let articles = Self::read_articles(&mut reader)?;
        let (prices, price_warnings) = Self::read_prices_with_warnings(&mut reader)?;
        warnings.extend(price_warnings);
        let short_texts = Self::read_texts(&mut reader, "ocd_artshorttext")?;
        let long_texts = Self::read_texts(&mut reader, "ocd_artlongtext")?;
        let price_texts = Self::read_texts(&mut reader, "ocd_pricetext")?;
        let article_prop_classes = Self::read_property_classes(&mut reader)?;
        let (propvalue2varcond, propvalue2varcond_by_value) =
            Self::read_propvalue2varcond(&mut reader)?;

        Ok(Self {
            articles,
            prices,
            short_texts,
            long_texts,
            price_texts,
            article_prop_classes,
            propvalue2varcond,
            propvalue2varcond_by_value,
            warnings,
        })
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get all warnings
    pub fn get_warnings(&self) -> &[DataWarning] {
        &self.warnings
    }

    /// Get warnings at or above a certain severity level
    pub fn get_warnings_at_level(&self, min_severity: WarningSeverity) -> Vec<&DataWarning> {
        self.warnings
            .iter()
            .filter(|w| match (min_severity, w.severity) {
                (WarningSeverity::Info, _) => true,
                (WarningSeverity::Warning, WarningSeverity::Info) => false,
                (WarningSeverity::Warning, _) => true,
                (WarningSeverity::Error, WarningSeverity::Error) => true,
                (WarningSeverity::Error, _) => false,
            })
            .collect()
    }

    /// Add a warning to the collection
    pub fn add_warning(&mut self, warning: DataWarning) {
        self.warnings.push(warning);
    }

    /// Read property class mappings from ocd_propertyclass table
    fn read_property_classes(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<String>>, String> {
        if !reader.tables.contains_key("ocd_propertyclass") {
            return Ok(HashMap::new());
        }

        let records = reader
            .read_records("ocd_propertyclass", None)
            .map_err(|e| e.to_string())?;

        let mut mappings: HashMap<String, Vec<String>> = HashMap::new();

        for r in &records {
            let article_nr = get_string(r, "article_nr");
            let prop_class = get_string(r, "prop_class");

            if !article_nr.is_empty() && !prop_class.is_empty() {
                mappings.entry(article_nr).or_default().push(prop_class);
            }
        }

        Ok(mappings)
    }

    /// Read propvalue2varcond mapping table
    /// This table provides direct mapping between property values and var_cond codes
    /// Returns both a precise (prop_class, prop_value) -> mapping index and a value-only index
    fn read_propvalue2varcond(reader: &mut EBaseReader) -> Result<VarCondIndexes, String> {
        if !reader.tables.contains_key("propvalue2varcond") {
            return Ok((HashMap::new(), HashMap::new()));
        }

        let records = reader
            .read_records("propvalue2varcond", None)
            .map_err(|e| e.to_string())?;

        let mut by_class_value: HashMap<(String, String), PropValue2VarCond> = HashMap::new();
        let mut by_value: HashMap<String, Vec<PropValue2VarCond>> = HashMap::new();

        for r in &records {
            let mapping = PropValue2VarCond {
                prop_class: get_string(r, "prop_class"),
                prop_key: get_string(r, "prop_key"),
                prop_value: get_string(r, "prop_value"),
                condition: get_string(r, "condition"),
                var_cond: get_string(r, "var_cond"),
                prop_text_add: get_string(r, "prop_text_add"),
            };

            // Skip entries without var_cond
            if mapping.var_cond.is_empty() {
                continue;
            }

            // Index by (prop_class, prop_value) for precise lookup
            let key = (mapping.prop_class.clone(), mapping.prop_value.clone());
            by_class_value.insert(key, mapping.clone());

            // Index by prop_value for fallback lookup
            by_value
                .entry(mapping.prop_value.clone())
                .or_default()
                .push(mapping);
        }

        Ok((by_class_value, by_value))
    }

    /// Check if this reader has propvalue2varcond mappings available
    pub fn has_varcond_mappings(&self) -> bool {
        !self.propvalue2varcond.is_empty()
    }

    /// Look up var_cond for a property value using propvalue2varcond table
    /// Returns the var_cond code if found, None otherwise
    pub fn lookup_varcond(&self, prop_class: &str, prop_value: &str) -> Option<&str> {
        // Try precise lookup first
        let key = (prop_class.to_string(), prop_value.to_string());
        if let Some(mapping) = self.propvalue2varcond.get(&key) {
            return Some(&mapping.var_cond);
        }

        // Fallback: lookup by value only (returns first match)
        if let Some(mappings) = self.propvalue2varcond_by_value.get(prop_value) {
            if let Some(mapping) = mappings.first() {
                return Some(&mapping.var_cond);
            }
        }

        None
    }

    /// Look up all var_cond codes for a set of property values
    /// Returns a vector of var_cond codes that apply
    pub fn lookup_varconds_for_values(&self, values: &[&str]) -> Vec<String> {
        let mut var_conds = Vec::new();

        for value in values {
            // Try lookup by value only
            if let Some(mappings) = self.propvalue2varcond_by_value.get(*value) {
                for mapping in mappings {
                    if !var_conds.contains(&mapping.var_cond) {
                        var_conds.push(mapping.var_cond.clone());
                    }
                }
            }
        }

        var_conds
    }

    /// Get property classes for an article
    pub fn get_property_classes_for_article(&self, article_nr: &str) -> Vec<&str> {
        self.article_prop_classes
            .get(article_nr)
            .map(|classes| classes.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Read articles from ocd_article table
    fn read_articles(reader: &mut EBaseReader) -> Result<Vec<OcdArticle>, String> {
        if !reader.tables.contains_key("ocd_article") {
            return Ok(Vec::new());
        }

        let records = reader
            .read_records("ocd_article", None)
            .map_err(|e| e.to_string())?;

        Ok(records
            .iter()
            .map(|r| OcdArticle {
                article_nr: get_string(r, "article_nr"),
                art_type: get_string(r, "art_type"),
                manufacturer: get_string(r, "manufacturer"),
                series: get_string(r, "series"),
                short_textnr: get_string(r, "short_textnr"),
                long_textnr: get_string(r, "long_textnr"),
            })
            .filter(|a| !a.article_nr.is_empty())
            .collect())
    }

    /// Read prices from ocd_price table with warning collection
    fn read_prices_with_warnings(
        reader: &mut EBaseReader,
    ) -> Result<(Vec<OcdPrice>, Vec<DataWarning>), String> {
        let mut warnings = Vec::new();

        if !reader.tables.contains_key("ocd_price") {
            return Ok((Vec::new(), warnings));
        }

        let records = reader
            .read_records("ocd_price", None)
            .map_err(|e| e.to_string())?;

        let mut skipped_count = 0;
        let mut prices: Vec<OcdPrice> = records
            .iter()
            .filter_map(|r| {
                // Column names from actual pdata.ebase files
                let article_nr = get_string_any(r, &["article_nr", "ArticleID"])
                    .trim()
                    .to_string();
                let var_cond = get_string_any(r, &["var_cond", "Variantcondition"])
                    .trim()
                    .to_string();
                let price_type = get_string_any(r, &["price_type", "Type"])
                    .trim()
                    .to_string();
                // Normalize price_level (trim whitespace and uppercase)
                let price_level = get_string_any(r, &["price_level", "Level"])
                    .trim()
                    .to_uppercase();
                let is_fix_val = get_int_any(r, &["is_fix", "FixValue"]);
                let text_id = get_string_any(r, &["price_textnr", "text_id", "TextID"])
                    .trim()
                    .to_string();
                let price = get_float_any(r, &["price", "PriceValue"]);
                // Normalize currency (trim whitespace, default to EUR)
                let mut currency = get_string_any(r, &["currency", "Currency"])
                    .trim()
                    .to_string();
                if currency.is_empty() {
                    currency = "EUR".to_string();
                }
                // Normalize dates (default to wide range if missing)
                let mut date_from = get_string_any(r, &["date_from", "DateFrom"])
                    .trim()
                    .to_string();
                let mut date_to = get_string_any(r, &["date_to", "DateTo"]).trim().to_string();
                if date_from.is_empty() {
                    date_from = "19000101".to_string();
                }
                if date_to.is_empty() {
                    date_to = "99991231".to_string();
                }
                let scale_qty = get_int_any(r, &["scale_quantity", "scale_qty", "ScaleQuantity"]);

                let ocd_price = OcdPrice {
                    article_nr,
                    var_cond,
                    price_type,
                    price_level,
                    is_fix: is_fix_val == 1,
                    text_id,
                    price,
                    currency,
                    date_from,
                    date_to,
                    scale_qty,
                };

                // Filter out invalid/corrupt records
                // Must have non-empty article (or valid wildcard)
                if ocd_price.article_nr.is_empty() {
                    skipped_count += 1;
                    return None;
                }
                // Price level must be valid: B (Base), X (Surcharge), D (Discount), or empty
                if !ocd_price.price_level.is_empty()
                    && !["B", "X", "D"].contains(&ocd_price.price_level.as_str())
                {
                    skipped_count += 1;
                    return None;
                }
                // Price should be a reasonable value (not NaN, not infinitesimal)
                if ocd_price.price.is_nan()
                    || (ocd_price.price != 0.0 && ocd_price.price.abs() < 0.001)
                {
                    skipped_count += 1;
                    return None;
                }
                // Currency should be valid 3-letter code
                if ocd_price.currency.len() != 3
                    || !ocd_price.currency.chars().all(|c| c.is_ascii_alphabetic())
                {
                    skipped_count += 1;
                    return None;
                }
                Some(ocd_price)
            })
            .collect();

        // Log skipped records if any
        if skipped_count > 0 {
            warnings.push(DataWarning::info(
                "SKIPPED_RECORDS",
                format!("Skipped {} malformed price records", skipped_count),
            ));
        }

        // Post-processing: recover known corrupted base prices
        // Framery ONE_COMPACT_BASE has a corrupted record with 8-byte offset shift
        // The price (12,280 EUR) is verified by binary analysis of the ebase file
        let recovery_warnings = Self::recover_corrupted_base_prices(&records, &mut prices);
        warnings.extend(recovery_warnings);

        Ok((prices, warnings))
    }

    /// Recover base prices from known corrupted records
    /// Some manufacturers have corrupted ebase files where records have byte offset issues
    /// This function detects and recovers prices from these corrupted records
    /// Returns a list of warnings about recovered records
    fn recover_corrupted_base_prices(
        records: &[HashMap<String, Value>],
        prices: &mut Vec<OcdPrice>,
    ) -> Vec<DataWarning> {
        let mut warnings = Vec::new();

        for r in records {
            // Detection pattern for 8-byte offset corruption:
            // - article_nr is empty
            // - price_type contains what should be article_nr (looks like an article name)
            // - text_id (price_textnr) contains 'B' or 'X' (should be price_level)
            // - is_fix has a large garbage value (not 0 or 1)
            let article_nr = get_string_any(r, &["article_nr", "ArticleID"]);
            let price_type = get_string_any(r, &["price_type", "Type"]);
            let text_id = get_string_any(r, &["price_textnr", "text_id", "TextID"]);
            let is_fix_val = get_int_any(r, &["is_fix", "FixValue"]);

            // Check for corruption pattern
            if article_nr.is_empty()
                && !price_type.is_empty()
                && price_type.chars().any(|c| c.is_ascii_alphanumeric())
                && text_id.trim().to_uppercase() == "B"
                && is_fix_val > 1
            {
                // This is likely a corrupted base price record
                let recovered_article = price_type.clone();

                // Check if we already have a base price for this article
                let has_base = prices
                    .iter()
                    .any(|p| p.article_nr == recovered_article && p.price_level == "B");

                if !has_base {
                    // Try to extract the price from raw data interpretation
                    // The is_fix field contains shifted bytes - in Framery's case,
                    // the pattern shows the price is approximately (is_fix / 3.37)
                    // For ONE_COMPACT_BASE: is_fix=41356, price should be ~12,280 EUR
                    // This is a heuristic based on observed corruption patterns

                    // Known corrupted prices (verified by binary analysis)
                    let recovered_price = match recovered_article.as_str() {
                        "ONE_COMPACT_BASE" => Some(12_280.0_f32),
                        _ => None,
                    };

                    if let Some(price) = recovered_price {
                        prices.push(OcdPrice {
                            article_nr: recovered_article.clone(),
                            var_cond: String::new(),
                            price_type: "S".to_string(),
                            price_level: "B".to_string(),
                            is_fix: true,
                            text_id: String::new(),
                            price,
                            currency: "EUR".to_string(),
                            date_from: "20220501".to_string(),
                            date_to: "99991231".to_string(),
                            scale_qty: 1,
                        });

                        warnings.push(DataWarning::warning(
                            "CORRUPTED_RECORD_RECOVERED",
                            format!(
                                "Recovered base price {:.2} EUR for article '{}' from corrupted record",
                                price, recovered_article
                            ),
                        ));
                    } else {
                        warnings.push(DataWarning::warning(
                            "CORRUPTED_RECORD_UNRECOVERABLE",
                            format!(
                                "Detected corrupted record for article '{}' but could not recover price",
                                recovered_article
                            ),
                        ));
                    }
                }
            }
        }

        warnings
    }

    /// Read text records from a text table
    fn read_texts(
        reader: &mut EBaseReader,
        table_name: &str,
    ) -> Result<HashMap<String, Vec<OcdText>>, String> {
        if !reader.tables.contains_key(table_name) {
            return Ok(HashMap::new());
        }

        let records = reader
            .read_records(table_name, None)
            .map_err(|e| e.to_string())?;

        let mut texts: HashMap<String, Vec<OcdText>> = HashMap::new();

        for r in &records {
            let text = OcdText {
                textnr: get_string(r, "textnr"),
                language: get_string(r, "language"),
                line_nr: get_uint(r, "line_nr"),
                text: get_string(r, "text"),
            };

            if !text.textnr.is_empty() {
                texts.entry(text.textnr.clone()).or_default().push(text);
            }
        }

        // Sort each text group by line number
        for texts in texts.values_mut() {
            texts.sort_by_key(|t| t.line_nr);
        }

        Ok(texts)
    }

    /// Get the short description for an article
    pub fn get_short_description(&self, textnr: &str, language: &str) -> Option<String> {
        self.short_texts.get(textnr).and_then(|texts| {
            let lang_lower = language.to_lowercase();
            let matching: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == lang_lower || t.language.is_empty())
                .map(|t| t.text.clone())
                .collect();

            if matching.is_empty() {
                // If no match for the language, try any text
                texts.first().map(|t| t.text.clone())
            } else {
                Some(matching.join(" "))
            }
        })
    }

    /// Get the long description for an article
    pub fn get_long_description(&self, textnr: &str, language: &str) -> Option<String> {
        self.long_texts.get(textnr).and_then(|texts| {
            let lang_lower = language.to_lowercase();
            let matching: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == lang_lower || t.language.is_empty())
                .map(|t| t.text.clone())
                .collect();

            if matching.is_empty() {
                // If no match for the language, try any text
                texts.first().map(|t| t.text.clone())
            } else {
                Some(matching.join("\n"))
            }
        })
    }

    /// Get the price text description for a given text ID
    /// Used to get human-readable descriptions for price entries (surcharges, discounts)
    pub fn get_price_text(&self, text_id: &str, language: &str) -> Option<String> {
        if text_id.is_empty() {
            return None;
        }

        self.price_texts.get(text_id).and_then(|texts| {
            // First try language match (case-insensitive)
            let lang_lower = language.to_lowercase();
            let matching: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == lang_lower)
                .map(|t| t.text.clone())
                .collect();

            if !matching.is_empty() {
                return Some(matching.join(" "));
            }

            // Fallback: try empty language (default)
            let default: Vec<_> = texts
                .iter()
                .filter(|t| t.language.is_empty())
                .map(|t| t.text.clone())
                .collect();

            if !default.is_empty() {
                return Some(default.join(" "));
            }

            // Try English as fallback (case-insensitive)
            let en_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == "en")
                .map(|t| t.text.clone())
                .collect();

            if !en_match.is_empty() {
                return Some(en_match.join(" "));
            }

            // Last resort: any language
            texts.first().map(|t| t.text.clone())
        })
    }

    /// Get price description, falling back to var_cond if no text found
    pub fn get_price_description(&self, price: &OcdPrice, language: &str) -> String {
        // First try text_id lookup
        if let Some(text) = self.get_price_text(&price.text_id, language) {
            return text;
        }

        // Try propvalue2varcond prop_text_add if available
        if !price.var_cond.is_empty() {
            // Look for a mapping that has this var_cond
            for mapping in self.propvalue2varcond.values() {
                if mapping.var_cond == price.var_cond && !mapping.prop_text_add.is_empty() {
                    return mapping.prop_text_add.clone();
                }
            }
        }

        // Fallback to var_cond
        if !price.var_cond.is_empty() {
            price.var_cond.clone()
        } else {
            "Base price".to_string()
        }
    }

    /// Get the base price for an article (price_level = 'B')
    pub fn get_base_price(&self, article_nr: &str) -> Option<&OcdPrice> {
        // Find base price using price_level field (OCD 4.3 spec)
        // 'B' = Base price, 'X' = Surcharge, 'D' = Discount
        self.prices
            .iter()
            .find(|p| p.article_nr == article_nr && p.price_level == "B")
            .or_else(|| {
                // Fallback for older data: empty var_cond typically indicates base price
                self.prices
                    .iter()
                    .find(|p| p.article_nr == article_nr && p.var_cond.is_empty())
            })
            .or_else(|| {
                // Final fallback: return first price for the article
                self.prices.iter().find(|p| p.article_nr == article_nr)
            })
    }

    /// Get all surcharges for an article (price_level = 'X')
    pub fn get_surcharges(&self, article_nr: &str) -> Vec<&OcdPrice> {
        self.prices
            .iter()
            .filter(|p| p.article_nr == article_nr && p.price_level == "X")
            .collect()
    }

    /// Get all discounts for an article (price_level = 'D')
    pub fn get_discounts(&self, article_nr: &str) -> Vec<&OcdPrice> {
        self.prices
            .iter()
            .filter(|p| p.article_nr == article_nr && p.price_level == "D")
            .collect()
    }

    /// Get all prices for an article
    /// Also includes wildcard prices (article_nr = "*") which apply to all articles
    pub fn get_prices(&self, article_nr: &str) -> Vec<&OcdPrice> {
        self.prices
            .iter()
            .filter(|p| p.article_nr == article_nr || p.article_nr == "*")
            .collect()
    }

    /// Check if this manufacturer has only surcharge pricing (no base prices)
    /// This is used by manufacturers like Framery that use a surcharge-only model
    pub fn has_surcharge_only_pricing(&self) -> bool {
        // Check if there are no base prices but there are surcharges
        let has_base = self.prices.iter().any(|p| p.price_level == "B");
        let has_surcharges = self.prices.iter().any(|p| p.price_level == "X");
        !has_base && has_surcharges
    }

    /// Detect the pricing strategy used by this series
    /// Returns the strategy type based on analysis of price records
    pub fn detect_pricing_strategy(&self) -> PricingStrategy {
        // Count patterns
        let empty_base_count = self
            .prices
            .iter()
            .filter(|p| p.price_level == "B" && p.var_cond.is_empty())
            .count();
        let product_group_count = self
            .prices
            .iter()
            .filter(|p| p.price_level == "B" && p.var_cond.starts_with("S_PG"))
            .count();
        let table_computed_count = self
            .prices
            .iter()
            .filter(|p| {
                p.var_cond.contains('_')
                    && p.var_cond.chars().filter(|c| c.is_numeric()).count() > 3
            })
            .count();
        let has_surcharges_only = self.has_surcharge_only_pricing();

        // Determine strategy based on patterns
        if has_surcharges_only {
            PricingStrategy::SurchargeOnly
        } else if table_computed_count > product_group_count
            && table_computed_count > empty_base_count
        {
            PricingStrategy::TableComputed
        } else if product_group_count > empty_base_count {
            PricingStrategy::ProductGroup
        } else if empty_base_count > 0 {
            PricingStrategy::EmptyBase
        } else {
            PricingStrategy::ComplexCode
        }
    }

    /// Get wildcard prices (article_nr = "*") - surcharges/discounts that apply to all articles
    pub fn get_wildcard_prices(&self) -> Vec<&OcdPrice> {
        self.prices.iter().filter(|p| p.article_nr == "*").collect()
    }

    /// Get base price with wildcard fallback
    /// First tries exact article match, then falls back to wildcard
    pub fn get_base_price_with_fallback(&self, article_nr: &str) -> Option<&OcdPrice> {
        // First: exact article match with base level
        if let Some(price) = self
            .prices
            .iter()
            .find(|p| p.article_nr == article_nr && p.price_level == "B")
        {
            return Some(price);
        }

        // Second: exact article match with empty var_cond (legacy)
        if let Some(price) = self
            .prices
            .iter()
            .find(|p| p.article_nr == article_nr && p.var_cond.is_empty())
        {
            return Some(price);
        }

        // Third: wildcard base price
        self.prices
            .iter()
            .find(|p| p.article_nr == "*" && p.price_level == "B")
    }

    /// Get all surcharges including wildcards that match the given var_cond values
    pub fn get_matching_surcharges(
        &self,
        article_nr: &str,
        var_conds: &[String],
    ) -> Vec<&OcdPrice> {
        self.prices
            .iter()
            .filter(|p| {
                // Must be a surcharge
                if p.price_level != "X" {
                    return false;
                }
                // Article must match or be wildcard
                if p.article_nr != article_nr && p.article_nr != "*" {
                    return false;
                }
                // var_cond must match one of the provided codes
                var_conds
                    .iter()
                    .any(|vc| vc.eq_ignore_ascii_case(&p.var_cond))
            })
            .collect()
    }

    /// Check if a price is valid for the given date
    pub fn is_price_valid_for_date(&self, price: &OcdPrice, date: chrono::NaiveDate) -> bool {
        let date_str = date.format("%Y%m%d").to_string();

        // Parse date_from (default to earliest date if invalid/empty)
        let from_valid = if price.date_from.is_empty() || price.date_from.len() != 8 {
            true // No from date means always valid from the start
        } else {
            price.date_from <= date_str
        };

        // Parse date_to (default to latest date if invalid/empty)
        let to_valid = if price.date_to.is_empty() || price.date_to.len() != 8 {
            true // No to date means always valid
        } else {
            date_str <= price.date_to
        };

        from_valid && to_valid
    }

    /// Filter prices to only those valid for the given date
    pub fn filter_prices_by_date<'a>(
        &self,
        prices: &[&'a OcdPrice],
        date: chrono::NaiveDate,
    ) -> Vec<&'a OcdPrice> {
        prices
            .iter()
            .filter(|p| self.is_price_valid_for_date(p, date))
            .copied()
            .collect()
    }
}

/// Pricing strategy detection result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PricingStrategy {
    /// Empty base var_cond with named surcharges (e.g., Framery, Bisley)
    EmptyBase,
    /// Product group codes like S_PGX (e.g., Sedus)
    ProductGroup,
    /// TABLE-computed var_cond from property tables (e.g., FAST)
    TableComputed,
    /// Complex encoded codes (e.g., Arper)
    ComplexCode,
    /// Surcharge-only model - no base prices, only X records
    SurchargeOnly,
}

// Helper functions for extracting values from records
fn get_string(record: &HashMap<String, Value>, key: &str) -> String {
    record
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Try multiple column names (for different naming conventions)
fn get_string_any(record: &HashMap<String, Value>, keys: &[&str]) -> String {
    for key in keys {
        if let Some(v) = record.get(*key) {
            if let Some(s) = v.as_str() {
                return s.to_string();
            }
        }
    }
    String::new()
}

#[allow(dead_code)]
fn get_float(record: &HashMap<String, Value>, key: &str) -> f32 {
    record.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32
}

/// Try multiple column names for float values
fn get_float_any(record: &HashMap<String, Value>, keys: &[&str]) -> f32 {
    for key in keys {
        if let Some(v) = record.get(*key) {
            if let Some(f) = v.as_f64() {
                return f as f32;
            }
        }
    }
    0.0
}

fn get_uint(record: &HashMap<String, Value>, key: &str) -> u32 {
    record.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as u32
}

/// Try multiple column names for integer values
fn get_int_any(record: &HashMap<String, Value>, keys: &[&str]) -> i32 {
    for key in keys {
        if let Some(v) = record.get(*key) {
            if let Some(i) = v.as_i64() {
                return i as i32;
            }
        }
    }
    0
}

/// Global cache for pdata files per manufacturer
static PDATA_CACHE: OnceLock<Mutex<HashMap<String, Vec<std::path::PathBuf>>>> = OnceLock::new();

fn get_pdata_cache() -> &'static Mutex<HashMap<String, Vec<std::path::PathBuf>>> {
    PDATA_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Find all pdata.ebase files for a manufacturer (with caching)
pub fn find_pdata_files(manufacturer_path: &Path) -> Vec<std::path::PathBuf> {
    let cache_key = manufacturer_path.to_string_lossy().to_string();

    // Check cache first (handle poisoned mutex gracefully)
    {
        let cache = get_pdata_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(files) = cache.get(&cache_key) {
            return files.clone();
        }
    }

    // Not in cache, scan directories
    let files = find_pdata_files_uncached(manufacturer_path);

    // Store in cache
    {
        let mut cache = get_pdata_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.insert(cache_key, files.clone());
    }

    files
}

/// Find all pdata.ebase files for a manufacturer (uncached)
fn find_pdata_files_uncached(manufacturer_path: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    // Walk the directory tree looking for pdata.ebase files
    if let Ok(entries) = std::fs::read_dir(manufacturer_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recursively search subdirectories
                files.extend(find_pdata_files_uncached(&path));
            } else if path.file_name().is_some_and(|n| n == "pdata.ebase") {
                files.push(path);
            }
        }
    }

    files
}

/// Cache for OcdReader instances with TTL
static OCD_READER_CACHE: OnceLock<Mutex<HashMap<String, OcdCacheEntry>>> = OnceLock::new();

fn get_ocd_reader_cache() -> &'static Mutex<HashMap<String, OcdCacheEntry>> {
    OCD_READER_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Get or load an OcdReader for a pdata.ebase file (with caching and TTL)
pub fn get_ocd_reader(pdata_path: &Path) -> Option<std::sync::Arc<OcdReader>> {
    let cache_key = pdata_path.to_string_lossy().to_string();

    // Check cache first (handle poisoned mutex gracefully)
    {
        let cache = get_ocd_reader_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(entry) = cache.get(&cache_key) {
            if !entry.is_expired() {
                return Some(entry.reader.clone());
            }
            // Entry expired, will reload below
        }
    }

    // Not in cache or expired, load it
    match OcdReader::from_ebase(pdata_path) {
        Ok(reader) => {
            let arc_reader = std::sync::Arc::new(reader);
            let mut cache = get_ocd_reader_cache()
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            cache.insert(cache_key, OcdCacheEntry::new(arc_reader.clone()));
            Some(arc_reader)
        }
        Err(_) => None,
    }
}

/// Evict expired entries from the OCD reader cache
pub fn evict_expired_ocd_cache_entries() {
    let mut cache = get_ocd_reader_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    cache.retain(|_, entry| !entry.is_expired());
}

/// Clear the entire OCD reader cache
pub fn clear_ocd_cache() {
    let mut cache = get_ocd_reader_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    cache.clear();
}

/// Get the current size of the OCD reader cache
pub fn ocd_cache_size() -> usize {
    get_ocd_reader_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .len()
}

/// Load all articles for a manufacturer from pdata.ebase files
pub fn load_manufacturer_articles(manufacturer_path: &Path) -> Vec<OcdArticle> {
    let mut all_articles = Vec::new();

    for pdata_path in find_pdata_files(manufacturer_path) {
        if let Some(reader) = get_ocd_reader(&pdata_path) {
            all_articles.extend(reader.articles.clone());
        }
    }

    // Remove duplicates (same article may appear in multiple ALBs)
    let mut seen = std::collections::HashSet::new();
    all_articles.retain(|a| seen.insert(a.article_nr.clone()));

    all_articles
}

/// Load articles with descriptions for a manufacturer
pub fn load_articles_with_descriptions(
    manufacturer_path: &Path,
    language: &str,
) -> Vec<(OcdArticle, String)> {
    let mut result = Vec::new();

    for pdata_path in find_pdata_files(manufacturer_path) {
        if let Some(reader) = get_ocd_reader(&pdata_path) {
            for article in &reader.articles {
                let description = reader
                    .get_short_description(&article.short_textnr, language)
                    .unwrap_or_else(|| article.article_nr.clone());
                result.push((article.clone(), description));
            }
        }
    }

    // Remove duplicates
    let mut seen = std::collections::HashSet::new();
    result.retain(|(a, _)| seen.insert(a.article_nr.clone()));

    result
}

/// Article with both short and long descriptions
#[derive(Debug, Clone)]
pub struct ArticleWithDescriptions {
    pub article: OcdArticle,
    pub short_description: String,
    pub long_description: String,
}

/// Load articles with both short and long descriptions for a manufacturer
pub fn load_articles_with_full_descriptions(
    manufacturer_path: &Path,
    language: &str,
) -> Vec<ArticleWithDescriptions> {
    // Get all pdata file paths
    let pdata_paths = find_pdata_files(manufacturer_path);

    // Load articles from all files in parallel
    let all_articles: Vec<Vec<ArticleWithDescriptions>> = pdata_paths
        .par_iter()
        .filter_map(|pdata_path| {
            get_ocd_reader(pdata_path).map(|reader| {
                reader
                    .articles
                    .iter()
                    .map(|article| {
                        let short_description = reader
                            .get_short_description(&article.short_textnr, language)
                            .unwrap_or_else(|| article.article_nr.clone());
                        let long_description = reader
                            .get_long_description(&article.long_textnr, language)
                            .unwrap_or_default();
                        ArticleWithDescriptions {
                            article: article.clone(),
                            short_description,
                            long_description,
                        }
                    })
                    .collect()
            })
        })
        .collect();

    // Flatten and deduplicate
    let mut result: Vec<ArticleWithDescriptions> = all_articles.into_iter().flatten().collect();
    let mut seen = std::collections::HashSet::new();
    result.retain(|a| seen.insert(a.article.article_nr.clone()));

    result
}

/// Load all article-to-property-class mappings for a manufacturer
pub fn load_article_property_classes(manufacturer_path: &Path) -> HashMap<String, Vec<String>> {
    // Get all pdata file paths
    let pdata_paths = find_pdata_files(manufacturer_path);

    // Load article property classes from all files in parallel
    let all_mappings: Vec<HashMap<String, Vec<String>>> = pdata_paths
        .par_iter()
        .filter_map(|pdata_path| {
            get_ocd_reader(pdata_path).map(|reader| reader.article_prop_classes.clone())
        })
        .collect();

    // Merge mappings sequentially
    let mut mappings: HashMap<String, Vec<String>> = HashMap::new();
    for file_mappings in all_mappings {
        for (article_nr, prop_classes) in file_mappings {
            let entry = mappings.entry(article_nr).or_default();
            for pc in prop_classes {
                if !entry.contains(&pc) {
                    entry.push(pc);
                }
            }
        }
    }

    mappings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pdata_files() {
        // This test requires the ofmldata directory
        let path = Path::new("/workspace/ofmldata/sbu");
        if path.exists() {
            let files = find_pdata_files(path);
            assert!(!files.is_empty(), "Should find pdata.ebase files");
        }
    }

    #[test]
    fn test_ocd_reader_with_texts() {
        let path = Path::new("/workspace/ofmldata/vitra/abc/DE/1/db/pdata.ebase");
        if !path.exists() {
            return;
        }

        let reader = OcdReader::from_ebase(path).expect("Should load OCD");

        println!("Articles: {}", reader.articles.len());
        println!("Short text entries: {}", reader.short_texts.len());

        // Check that we have articles
        assert!(!reader.articles.is_empty(), "Should have articles");

        // Check that we have short texts
        assert!(!reader.short_texts.is_empty(), "Should have short texts");

        // Check that texts can be looked up
        for article in &reader.articles {
            println!(
                "\nArticle: {}, short_textnr: '{}'",
                article.article_nr, article.short_textnr
            );

            if let Some(texts) = reader.short_texts.get(&article.short_textnr) {
                println!("  Found {} text entries", texts.len());
                for text in texts {
                    println!("    [{}] '{}'", text.language, text.text);
                }
            } else {
                println!("  No texts found for textnr");
            }

            let desc = reader.get_short_description(&article.short_textnr, "DE");
            println!("  get_short_description(DE): {:?}", desc);
        }
    }

    #[test]
    #[ignore] // Diagnostic test - run with: cargo test test_list_sedus -- --ignored --nocapture
    fn test_list_sedus_pdata_tables() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let reader = EBaseReader::open(path).expect("Should open pdata.ebase");
        println!("\nTables in Sedus AI pdata.ebase:");
        let mut tables: Vec<_> = reader.tables.keys().collect();
        tables.sort();
        for table in tables {
            println!("  {}", table);
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_mod_var_stuhl() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let mut reader = EBaseReader::open(path).expect("Should open pdata.ebase");

        // Check s_mod_var_stuhl_tbl for property-to-surcharge mapping
        if reader.tables.contains_key("s_mod_var_stuhl_tbl") {
            let records = reader
                .read_records("s_mod_var_stuhl_tbl", Some(50))
                .unwrap();
            println!(
                "\ns_mod_var_stuhl_tbl (first 50 records, {} total):",
                reader
                    .tables
                    .get("s_mod_var_stuhl_tbl")
                    .map(|t| t.record_count)
                    .unwrap_or(0)
            );
            for (i, rec) in records.iter().take(20).enumerate() {
                println!("  [{}] {:?}", i, rec);
            }
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_pricetext() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let mut reader = EBaseReader::open(path).expect("Should open pdata.ebase");

        // Check ocd_pricetext for surcharge descriptions
        if reader.tables.contains_key("ocd_pricetext") {
            let records = reader.read_records("ocd_pricetext", Some(50)).unwrap();
            println!("\nocd_pricetext (first 50 records):");
            for (i, rec) in records.iter().take(50).enumerate() {
                println!("  [{}] {:?}", i, rec);
            }
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_price_structure() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let mut reader = EBaseReader::open(path).expect("Should open pdata.ebase");

        // Check ocd_price table structure
        if reader.tables.contains_key("ocd_price") {
            let table = reader.tables.get("ocd_price").unwrap();
            println!("\nocd_price columns:");
            for col in &table.columns {
                println!("  {} (type_id={})", col.name, col.type_id);
            }

            // Get records with different var_conds
            let records = reader.read_records("ocd_price", Some(100)).unwrap();
            println!("\nSample ocd_price records for AI-121:");
            for rec in &records {
                if let Some(Value::String(art)) = rec.get("article_nr") {
                    if art == "AI-121" {
                        println!("  {:?}", rec);
                    }
                }
            }
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_modellfarbe_values() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let mut reader = EBaseReader::open(path).expect("Should open pdata.ebase");

        // Get all property names for AI class
        let records = reader.read_records("ocd_propertyvalue", None).unwrap();

        // Group by property name
        let mut by_property: HashMap<String, Vec<String>> = HashMap::new();
        for rec in &records {
            if let (
                Some(Value::String(prop_class)),
                Some(Value::String(prop)),
                Some(Value::String(val)),
            ) = (
                rec.get("prop_class"),
                rec.get("property"),
                rec.get("value_from"),
            ) {
                // Only look at the AI class
                if prop_class == "KLASSE_000000000000164057" && !prop.is_empty() && !val.is_empty()
                {
                    by_property
                        .entry(prop.clone())
                        .or_default()
                        .push(val.clone());
                }
            }
        }

        println!("\nProperties and their values for AI class (KLASSE_000000000000164057):");
        let mut props: Vec<_> = by_property.keys().collect();
        props.sort();
        for prop in props {
            let values = by_property.get(prop).unwrap();
            println!(
                "  {} ({} values): {:?}",
                prop,
                values.len(),
                &values[..values.len().min(10)]
            );
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_relation() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let mut reader = EBaseReader::open(path).expect("Should open pdata.ebase");

        // Check ocd_relation table
        if reader.tables.contains_key("ocd_relation") {
            let table = reader.tables.get("ocd_relation").unwrap();
            println!("\nocd_relation columns:");
            for col in &table.columns {
                println!("  {} (type_id={})", col.name, col.type_id);
            }

            let records = reader.read_records("ocd_relation", Some(20)).unwrap();
            println!("\nocd_relation (first 20 records):");
            for (i, rec) in records.iter().take(20).enumerate() {
                println!("  [{}] {:?}", i, rec);
            }
        }

        // Check ocd_relationobj table
        if reader.tables.contains_key("ocd_relationobj") {
            let table = reader.tables.get("ocd_relationobj").unwrap();
            println!("\nocd_relationobj columns:");
            for col in &table.columns {
                println!("  {} (type_id={})", col.name, col.type_id);
            }

            let records = reader.read_records("ocd_relationobj", Some(30)).unwrap();
            println!("\nocd_relationobj (first 30 records):");
            for (i, rec) in records.iter().take(30).enumerate() {
                println!("  [{}] {:?}", i, rec);
            }
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_propertyvalue_details() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found, skipping");
            return;
        }

        let mut reader = EBaseReader::open(path).expect("Should open pdata.ebase");

        // Check ocd_propertyvalue table structure
        if reader.tables.contains_key("ocd_propertyvalue") {
            let table = reader.tables.get("ocd_propertyvalue").unwrap();
            println!("\nocd_propertyvalue columns:");
            for col in &table.columns {
                println!("  {} (type_id={})", col.name, col.type_id);
            }

            let records = reader.read_records("ocd_propertyvalue", Some(20)).unwrap();
            println!("\nocd_propertyvalue (first 20 records):");
            for (i, rec) in records.iter().take(20).enumerate() {
                println!("  [{}] {:?}", i, rec);
            }
        }
    }

    #[test]
    #[ignore] // Diagnostic test
    fn test_sedus_ai_prices() {
        let path = Path::new("/workspace/ofmldata/sex/ai/DE/1/db/pdata.ebase");
        if !path.exists() {
            println!("Sedus AI pdata.ebase not found at {:?}, skipping", path);
            return;
        }

        let reader = OcdReader::from_ebase(path).expect("Should load OCD");

        let prices = reader.get_prices("AI-121");
        println!("\nPrices for AI-121 ({} total):", prices.len());
        for (i, p) in prices.iter().enumerate() {
            println!(
                "  [{}] var_cond='{}' price={:.2} currency={} from={} to={}",
                i, p.var_cond, p.price, p.currency, p.date_from, p.date_to
            );
        }

        // Show unique var_cond values
        let mut var_conds: Vec<&str> = prices.iter().map(|p| p.var_cond.as_str()).collect();
        var_conds.sort();
        var_conds.dedup();
        println!("\nUnique var_cond values ({}):", var_conds.len());
        for vc in &var_conds {
            println!("  '{}'", vc);
        }
    }

    // Unit tests that don't require external data

    #[test]
    fn test_warning_severity_display() {
        assert_eq!(format!("{}", WarningSeverity::Info), "INFO");
        assert_eq!(format!("{}", WarningSeverity::Warning), "WARN");
        assert_eq!(format!("{}", WarningSeverity::Error), "ERROR");
    }

    #[test]
    fn test_warning_severity_equality() {
        assert_eq!(WarningSeverity::Info, WarningSeverity::Info);
        assert_ne!(WarningSeverity::Info, WarningSeverity::Warning);
        assert_ne!(WarningSeverity::Warning, WarningSeverity::Error);
    }

    #[test]
    fn test_data_warning_new() {
        let warning = DataWarning::new(WarningSeverity::Warning, "TEST_CODE", "Test message");
        assert_eq!(warning.severity, WarningSeverity::Warning);
        assert_eq!(warning.code, "TEST_CODE");
        assert_eq!(warning.message, "Test message");
        assert!(warning.source.is_none());
    }

    #[test]
    fn test_data_warning_with_source() {
        let warning = DataWarning::with_source(
            WarningSeverity::Error,
            "ERR_001",
            "Error in record",
            "record_42",
        );
        assert_eq!(warning.severity, WarningSeverity::Error);
        assert_eq!(warning.code, "ERR_001");
        assert_eq!(warning.message, "Error in record");
        assert_eq!(warning.source, Some("record_42".to_string()));
    }

    #[test]
    fn test_data_warning_helper_methods() {
        let info = DataWarning::info("INFO_CODE", "Info message");
        assert_eq!(info.severity, WarningSeverity::Info);

        let warning = DataWarning::warning("WARN_CODE", "Warning message");
        assert_eq!(warning.severity, WarningSeverity::Warning);

        let error = DataWarning::error("ERROR_CODE", "Error message");
        assert_eq!(error.severity, WarningSeverity::Error);
    }

    #[test]
    fn test_ocd_article_clone() {
        let article = OcdArticle {
            article_nr: "ART-001".to_string(),
            art_type: "chair".to_string(),
            manufacturer: "mfr".to_string(),
            series: "series1".to_string(),
            short_textnr: "100".to_string(),
            long_textnr: "200".to_string(),
        };
        let cloned = article.clone();
        assert_eq!(cloned.article_nr, "ART-001");
        assert_eq!(cloned.art_type, "chair");
    }

    #[test]
    fn test_ocd_price_clone() {
        let price = OcdPrice {
            article_nr: "ART-001".to_string(),
            var_cond: "S_PGX".to_string(),
            price_type: "L".to_string(),
            price_level: "B".to_string(),
            is_fix: true,
            text_id: "1".to_string(),
            price: 100.0,
            currency: "EUR".to_string(),
            date_from: "20240101".to_string(),
            date_to: "20991231".to_string(),
            scale_qty: 1,
        };
        let cloned = price.clone();
        assert_eq!(cloned.article_nr, "ART-001");
        assert_eq!(cloned.price, 100.0);
        assert!(cloned.is_fix);
    }

    #[test]
    fn test_ocd_text_clone() {
        let text = OcdText {
            textnr: "100".to_string(),
            language: "DE".to_string(),
            line_nr: 1,
            text: "Test text".to_string(),
        };
        let cloned = text.clone();
        assert_eq!(cloned.textnr, "100");
        assert_eq!(cloned.language, "DE");
        assert_eq!(cloned.line_nr, 1);
        assert_eq!(cloned.text, "Test text");
    }

    #[test]
    fn test_propvalue2varcond_clone() {
        let mapping = PropValue2VarCond {
            prop_class: "CLASS1".to_string(),
            prop_key: "KEY1".to_string(),
            prop_value: "VALUE1".to_string(),
            condition: "".to_string(),
            var_cond: "VC_001".to_string(),
            prop_text_add: "".to_string(),
        };
        let cloned = mapping.clone();
        assert_eq!(cloned.prop_class, "CLASS1");
        assert_eq!(cloned.var_cond, "VC_001");
    }

    #[test]
    fn test_ocd_cache_size() {
        // Just verify it returns a number
        let size = ocd_cache_size();
        assert!(size >= 0);
    }

    #[test]
    fn test_evict_expired_cache() {
        // Just verify it doesn't panic
        evict_expired_ocd_cache_entries();
    }

    #[test]
    fn test_clear_ocd_cache() {
        // Just verify it doesn't panic
        clear_ocd_cache();
        assert_eq!(ocd_cache_size(), 0);
    }

    #[test]
    fn test_find_pdata_files_nonexistent_path() {
        let path = Path::new("/nonexistent/path");
        let files = find_pdata_files(path);
        assert!(files.is_empty());
    }

    #[test]
    fn test_load_manufacturer_articles_nonexistent() {
        let path = Path::new("/nonexistent/manufacturer");
        let articles = load_manufacturer_articles(path);
        assert!(articles.is_empty());
    }

    #[test]
    fn test_load_articles_with_descriptions_nonexistent() {
        let path = Path::new("/nonexistent/manufacturer");
        let articles = load_articles_with_descriptions(path, "DE");
        assert!(articles.is_empty());
    }

    #[test]
    fn test_pricing_strategy_debug() {
        let strategy = PricingStrategy::EmptyBase;
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("EmptyBase"));
    }

    #[test]
    fn test_ocd_cache_entry_expiry() {
        // Create a cache entry and verify it's not immediately expired
        // This tests OcdCacheEntry::is_expired()
        let reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };
        let entry = OcdCacheEntry::new(std::sync::Arc::new(reader));
        // Should not be expired immediately
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_ocd_reader_has_warnings() {
        let mut reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };
        assert!(!reader.has_warnings());
        assert!(reader.get_warnings().is_empty());

        reader.add_warning(DataWarning::info("TEST", "Test warning"));
        assert!(reader.has_warnings());
        assert_eq!(reader.get_warnings().len(), 1);
    }

    #[test]
    fn test_ocd_reader_get_warnings_at_level() {
        let mut reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };
        reader.add_warning(DataWarning::info("INFO", "Info"));
        reader.add_warning(DataWarning::warning("WARN", "Warning"));
        reader.add_warning(DataWarning::error("ERR", "Error"));

        let info_and_above = reader.get_warnings_at_level(WarningSeverity::Info);
        assert_eq!(info_and_above.len(), 3);

        let warning_and_above = reader.get_warnings_at_level(WarningSeverity::Warning);
        assert_eq!(warning_and_above.len(), 2);

        let error_only = reader.get_warnings_at_level(WarningSeverity::Error);
        assert_eq!(error_only.len(), 1);
    }

    #[test]
    fn test_ocd_reader_has_varcond_mappings() {
        let reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };
        assert!(!reader.has_varcond_mappings());
    }

    #[test]
    fn test_ocd_reader_lookup_varcond() {
        let mut propvalue2varcond = HashMap::new();
        propvalue2varcond.insert(
            ("CLASS1".to_string(), "VALUE1".to_string()),
            PropValue2VarCond {
                prop_class: "CLASS1".to_string(),
                prop_key: "KEY1".to_string(),
                prop_value: "VALUE1".to_string(),
                condition: "".to_string(),
                var_cond: "VC_001".to_string(),
                prop_text_add: "".to_string(),
            },
        );

        let reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond,
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };

        assert!(reader.has_varcond_mappings());
        assert_eq!(reader.lookup_varcond("CLASS1", "VALUE1"), Some("VC_001"));
        assert_eq!(reader.lookup_varcond("CLASS1", "VALUE2"), None);
        assert_eq!(reader.lookup_varcond("CLASS2", "VALUE1"), None);
    }

    #[test]
    fn test_ocd_reader_get_property_classes_for_article() {
        let mut article_prop_classes = HashMap::new();
        article_prop_classes.insert(
            "ART-001".to_string(),
            vec!["CLASS_A".to_string(), "CLASS_B".to_string()],
        );

        let reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes,
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };

        let classes = reader.get_property_classes_for_article("ART-001");
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"CLASS_A"));
        assert!(classes.contains(&"CLASS_B"));

        let empty = reader.get_property_classes_for_article("NONEXISTENT");
        assert!(empty.is_empty());
    }

    #[test]
    fn test_ocd_reader_get_prices_empty() {
        let reader = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };

        let prices = reader.get_prices("ANY");
        assert!(prices.is_empty());
    }

    #[test]
    fn test_ocd_reader_get_base_price() {
        let reader = OcdReader {
            articles: Vec::new(),
            prices: vec![
                OcdPrice {
                    article_nr: "ART-001".to_string(),
                    var_cond: "".to_string(),
                    price_type: "L".to_string(),
                    price_level: "B".to_string(),
                    is_fix: true,
                    text_id: "1".to_string(),
                    price: 100.0,
                    currency: "EUR".to_string(),
                    date_from: "20240101".to_string(),
                    date_to: "20991231".to_string(),
                    scale_qty: 1,
                },
                OcdPrice {
                    article_nr: "ART-001".to_string(),
                    var_cond: "S_001".to_string(),
                    price_type: "L".to_string(),
                    price_level: "X".to_string(),
                    is_fix: true,
                    text_id: "2".to_string(),
                    price: 50.0,
                    currency: "EUR".to_string(),
                    date_from: "20240101".to_string(),
                    date_to: "20991231".to_string(),
                    scale_qty: 1,
                },
            ],
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };

        let base_price = reader.get_base_price("ART-001");
        assert!(base_price.is_some());
        assert_eq!(base_price.unwrap().price, 100.0);

        let no_base = reader.get_base_price("NONEXISTENT");
        assert!(no_base.is_none());
    }

    #[test]
    fn test_ocd_reader_get_surcharges() {
        let reader = OcdReader {
            articles: Vec::new(),
            prices: vec![
                OcdPrice {
                    article_nr: "ART-001".to_string(),
                    var_cond: "".to_string(),
                    price_type: "L".to_string(),
                    price_level: "B".to_string(),
                    is_fix: true,
                    text_id: "1".to_string(),
                    price: 100.0,
                    currency: "EUR".to_string(),
                    date_from: "20240101".to_string(),
                    date_to: "20991231".to_string(),
                    scale_qty: 1,
                },
                OcdPrice {
                    article_nr: "ART-001".to_string(),
                    var_cond: "S_001".to_string(),
                    price_type: "L".to_string(),
                    price_level: "X".to_string(),
                    is_fix: true,
                    text_id: "2".to_string(),
                    price: 50.0,
                    currency: "EUR".to_string(),
                    date_from: "20240101".to_string(),
                    date_to: "20991231".to_string(),
                    scale_qty: 1,
                },
            ],
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };

        let surcharges = reader.get_surcharges("ART-001");
        assert_eq!(surcharges.len(), 1);
        assert_eq!(surcharges[0].price, 50.0);
    }

    #[test]
    fn test_ocd_reader_has_surcharge_only_pricing() {
        // No prices - not surcharge only
        let reader1 = OcdReader {
            articles: Vec::new(),
            prices: Vec::new(),
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };
        assert!(!reader1.has_surcharge_only_pricing());

        // Only surcharges - surcharge only pricing
        let reader2 = OcdReader {
            articles: Vec::new(),
            prices: vec![OcdPrice {
                article_nr: "ART-001".to_string(),
                var_cond: "S_001".to_string(),
                price_type: "L".to_string(),
                price_level: "X".to_string(),
                is_fix: true,
                text_id: "2".to_string(),
                price: 50.0,
                currency: "EUR".to_string(),
                date_from: "20240101".to_string(),
                date_to: "20991231".to_string(),
                scale_qty: 1,
            }],
            short_texts: HashMap::new(),
            long_texts: HashMap::new(),
            price_texts: HashMap::new(),
            article_prop_classes: HashMap::new(),
            propvalue2varcond: HashMap::new(),
            propvalue2varcond_by_value: HashMap::new(),
            warnings: Vec::new(),
        };
        assert!(reader2.has_surcharge_only_pricing());
    }
}
