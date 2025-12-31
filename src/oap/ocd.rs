//! OCD (Office Catalog Data) reader for OFML product catalogs
//!
//! This module reads product catalog information from pdata.ebase files,
//! providing access to:
//! - Articles (ocd_article table)
//! - Article descriptions (ocd_artshorttext, ocd_artlongtext tables)
//! - Prices (ocd_price table)
//! - Property definitions (ocd_property, ocd_propertyvalue tables)

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
    /// Variant condition
    pub var_cond: String,
    /// Price type
    pub price_type: String,
    /// Price value in cents or smallest currency unit
    pub price: f32,
    /// Currency code (e.g., "EUR")
    pub currency: String,
    /// Valid from date
    pub date_from: String,
    /// Valid to date
    pub date_to: String,
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
    /// Property class mappings: article_nr -> list of property classes
    pub article_prop_classes: HashMap<String, Vec<String>>,
}

impl OcdReader {
    /// Create a new OCD reader from a pdata.ebase file
    pub fn from_ebase(path: &Path) -> Result<Self, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;

        let articles = Self::read_articles(&mut reader)?;
        let prices = Self::read_prices(&mut reader)?;
        let short_texts = Self::read_texts(&mut reader, "ocd_artshorttext")?;
        let long_texts = Self::read_texts(&mut reader, "ocd_artlongtext")?;
        let article_prop_classes = Self::read_property_classes(&mut reader)?;

        Ok(Self {
            articles,
            prices,
            short_texts,
            long_texts,
            article_prop_classes,
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
            .map(|r| OcdPrice {
                article_nr: get_string(r, "article_nr"),
                var_cond: get_string(r, "var_cond"),
                price_type: get_string(r, "price_type"),
                price: get_float(r, "price"),
                currency: get_string(r, "currency"),
                date_from: get_string(r, "date_from"),
                date_to: get_string(r, "date_to"),
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

    /// Get the price for an article (base price, no variant)
    pub fn get_base_price(&self, article_nr: &str) -> Option<&OcdPrice> {
        // First try to find a price with empty var_cond (true base price)
        let base = self.prices
            .iter()
            .find(|p| p.article_nr == article_nr && p.var_cond.is_empty());

        if base.is_some() {
            return base;
        }

        // Fallback: return first price for the article (if all have var_cond)
        self.prices
            .iter()
            .find(|p| p.article_nr == article_nr)
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

fn get_float(record: &HashMap<String, Value>, key: &str) -> f32 {
    record.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32
}

fn get_uint(record: &HashMap<String, Value>, key: &str) -> u32 {
    record.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as u32
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
