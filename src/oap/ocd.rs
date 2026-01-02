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

use std::collections::HashMap;
use std::path::Path;

use crate::ebase::{EBaseReader, Value};

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
}

impl OcdReader {
    /// Create a new OCD reader from a pdata.ebase file
    pub fn from_ebase(path: &Path) -> Result<Self, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;

        let articles = Self::read_articles(&mut reader)?;
        let prices = Self::read_prices(&mut reader)?;
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
        })
    }

    /// Read property class mappings from ocd_propertyclass table
    fn read_property_classes(reader: &mut EBaseReader) -> Result<HashMap<String, Vec<String>>, String> {
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
                mappings
                    .entry(article_nr)
                    .or_default()
                    .push(prop_class);
            }
        }

        Ok(mappings)
    }

    /// Read propvalue2varcond mapping table
    /// This table provides direct mapping between property values and var_cond codes
    /// Returns both a precise (prop_class, prop_value) -> mapping index and a value-only index
    fn read_propvalue2varcond(
        reader: &mut EBaseReader,
    ) -> Result<
        (
            HashMap<(String, String), PropValue2VarCond>,
            HashMap<String, Vec<PropValue2VarCond>>,
        ),
        String,
    > {
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

    /// Read prices from ocd_price table
    fn read_prices(reader: &mut EBaseReader) -> Result<Vec<OcdPrice>, String> {
        if !reader.tables.contains_key("ocd_price") {
            return Ok(Vec::new());
        }

        let records = reader
            .read_records("ocd_price", None)
            .map_err(|e| e.to_string())?;

        Ok(records
            .iter()
            .map(|r| {
                // Column names from actual pdata.ebase files
                let article_nr = get_string_any(r, &["article_nr", "ArticleID"]);
                let var_cond = get_string_any(r, &["var_cond", "Variantcondition"]);
                let price_type = get_string_any(r, &["price_type", "Type"]);
                let price_level = get_string_any(r, &["price_level", "Level"]);
                let is_fix_val = get_int_any(r, &["is_fix", "FixValue"]);
                let text_id = get_string_any(r, &["price_textnr", "text_id", "TextID"]);
                let price = get_float_any(r, &["price", "PriceValue"]);
                let currency = get_string_any(r, &["currency", "Currency"]);
                let date_from = get_string_any(r, &["date_from", "DateFrom"]);
                let date_to = get_string_any(r, &["date_to", "DateTo"]);
                let scale_qty = get_int_any(r, &["scale_quantity", "scale_qty", "ScaleQuantity"]);

                OcdPrice {
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
                }
            })
            .filter(|p| !p.article_nr.is_empty())
            .collect())
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
                texts
                    .entry(text.textnr.clone())
                    .or_default()
                    .push(text);
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
            let matching: Vec<_> = texts
                .iter()
                .filter(|t| t.language == language || t.language.is_empty())
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
            let matching: Vec<_> = texts
                .iter()
                .filter(|t| t.language == language || t.language.is_empty())
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
            // First try exact language match
            let matching: Vec<_> = texts
                .iter()
                .filter(|t| t.language == language)
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
                self.prices
                    .iter()
                    .find(|p| p.article_nr == article_nr)
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
    pub fn get_prices(&self, article_nr: &str) -> Vec<&OcdPrice> {
        self.prices
            .iter()
            .filter(|p| p.article_nr == article_nr)
            .collect()
    }
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

use std::sync::OnceLock;
use std::sync::Mutex;

/// Global cache for pdata files per manufacturer
static PDATA_CACHE: OnceLock<Mutex<HashMap<String, Vec<std::path::PathBuf>>>> = OnceLock::new();

fn get_pdata_cache() -> &'static Mutex<HashMap<String, Vec<std::path::PathBuf>>> {
    PDATA_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Find all pdata.ebase files for a manufacturer (with caching)
pub fn find_pdata_files(manufacturer_path: &Path) -> Vec<std::path::PathBuf> {
    let cache_key = manufacturer_path.to_string_lossy().to_string();

    // Check cache first
    {
        let cache = get_pdata_cache().lock().unwrap();
        if let Some(files) = cache.get(&cache_key) {
            return files.clone();
        }
    }

    // Not in cache, scan directories
    let files = find_pdata_files_uncached(manufacturer_path);

    // Store in cache
    {
        let mut cache = get_pdata_cache().lock().unwrap();
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
            } else if path.file_name().map_or(false, |n| n == "pdata.ebase") {
                files.push(path);
            }
        }
    }

    files
}

/// Cache for OcdReader instances
static OCD_READER_CACHE: OnceLock<Mutex<HashMap<String, std::sync::Arc<OcdReader>>>> = OnceLock::new();

fn get_ocd_reader_cache() -> &'static Mutex<HashMap<String, std::sync::Arc<OcdReader>>> {
    OCD_READER_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Get or load an OcdReader for a pdata.ebase file (with caching)
pub fn get_ocd_reader(pdata_path: &Path) -> Option<std::sync::Arc<OcdReader>> {
    let cache_key = pdata_path.to_string_lossy().to_string();

    // Check cache first
    {
        let cache = get_ocd_reader_cache().lock().unwrap();
        if let Some(reader) = cache.get(&cache_key) {
            return Some(reader.clone());
        }
    }

    // Not in cache, load it
    match OcdReader::from_ebase(pdata_path) {
        Ok(reader) => {
            let arc_reader = std::sync::Arc::new(reader);
            let mut cache = get_ocd_reader_cache().lock().unwrap();
            cache.insert(cache_key, arc_reader.clone());
            Some(arc_reader)
        }
        Err(_) => None,
    }
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
    let mut result = Vec::new();

    for pdata_path in find_pdata_files(manufacturer_path) {
        if let Some(reader) = get_ocd_reader(&pdata_path) {
            for article in &reader.articles {
                let short_description = reader
                    .get_short_description(&article.short_textnr, language)
                    .unwrap_or_else(|| article.article_nr.clone());
                let long_description = reader
                    .get_long_description(&article.long_textnr, language)
                    .unwrap_or_default();
                result.push(ArticleWithDescriptions {
                    article: article.clone(),
                    short_description,
                    long_description,
                });
            }
        }
    }

    // Remove duplicates
    let mut seen = std::collections::HashSet::new();
    result.retain(|a| seen.insert(a.article.article_nr.clone()));

    result
}

/// Load all article-to-property-class mappings for a manufacturer
pub fn load_article_property_classes(manufacturer_path: &Path) -> HashMap<String, Vec<String>> {
    let mut mappings: HashMap<String, Vec<String>> = HashMap::new();

    for pdata_path in find_pdata_files(manufacturer_path) {
        if let Some(reader) = get_ocd_reader(&pdata_path) {
            for (article_nr, prop_classes) in &reader.article_prop_classes {
                let entry = mappings.entry(article_nr.clone()).or_default();
                for pc in prop_classes {
                    if !entry.contains(pc) {
                        entry.push(pc.clone());
                    }
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
            println!("\nArticle: {}, short_textnr: '{}'", article.article_nr, article.short_textnr);

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
            let records = reader.read_records("s_mod_var_stuhl_tbl", Some(50)).unwrap();
            println!("\ns_mod_var_stuhl_tbl (first 50 records, {} total):", reader.tables.get("s_mod_var_stuhl_tbl").map(|t| t.record_count).unwrap_or(0));
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
            if let (Some(Value::String(prop_class)), Some(Value::String(prop)), Some(Value::String(val))) =
                (rec.get("prop_class"), rec.get("property"), rec.get("value_from"))
            {
                // Only look at the AI class
                if prop_class == "KLASSE_000000000000164057" && !prop.is_empty() && !val.is_empty() {
                    by_property.entry(prop.clone()).or_default().push(val.clone());
                }
            }
        }

        println!("\nProperties and their values for AI class (KLASSE_000000000000164057):");
        let mut props: Vec<_> = by_property.keys().collect();
        props.sort();
        for prop in props {
            let values = by_property.get(prop).unwrap();
            println!("  {} ({} values): {:?}", prop, values.len(), &values[..values.len().min(10)]);
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
            println!("  [{}] var_cond='{}' price={:.2} currency={} from={} to={}",
                i, p.var_cond, p.price, p.currency, p.date_from, p.date_to);
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
}
