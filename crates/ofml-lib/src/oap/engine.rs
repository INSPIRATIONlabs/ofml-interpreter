//! Configuration Engine - Core of the OAP configurator
//!
//! This module provides the ConfigurationEngine that connects:
//! - OAM data (article → CLS class mappings)
//! - OCD data (articles, prices, texts)
//! - CLS interpreter (property extraction, rules)
//! - Price calculation with var_cond matching
//!
//! ## Price Calculation Flow
//!
//! 1. Two-pass lookup: exact article match first, then wildcard fallback
//! 2. Base price (level 'B') applied first
//! 3. Surcharges (level 'X') accumulated and added
//! 4. Discounts (level 'D') subtracted last
//!
//! See `/docs/OCD-PRICING-IMPLEMENTATION.md` for full details.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rust_decimal::Decimal;
use tracing::{debug, instrument, trace, warn};

use crate::alb_loader::AlbLoader;
use crate::ast::{ClassMember, Expr, Stmt};
use crate::parser::Parser;
use crate::property::{PropertyDef, PropertyManager, PropertyType, PropertyValue};

use super::families::{FamilyConfiguration, FamilyLoader, FamilyProperty, ProductFamily};
use super::oam::{load_manufacturer_oam, ArticleMapping, OamData};
use super::ocd::{load_articles_with_descriptions, OcdArticle, OcdPrice, OcdReader};
use super::ocd_relation::RelationRuleReader;
use super::{Article, PriceResult, Surcharge};

/// Helper struct for rounded prices
struct RoundedPrices {
    base_price: Decimal,
    net_price: Decimal,
    total_price: Decimal,
}

/// Error types for the configuration engine
#[derive(Debug, Clone, thiserror::Error)]
pub enum EngineError {
    /// Article not found in manufacturer data
    #[error("Article '{article}' not found in manufacturer '{manufacturer}' data. Check if the article number is correct and the data directory is up to date.")]
    ArticleNotFound {
        article: String,
        manufacturer: String,
    },

    /// Legacy: Article not found (simple)
    #[error("Article not found: {0}")]
    ArticleNotFoundSimple(String),

    /// Article exists but is not configurable (no CLS class mapping)
    #[error("Article '{article}' is not configurable. It exists in the catalog but has no CLS class mapping for property configuration.")]
    NotConfigurable { article: String },

    /// CLS class referenced by OAM mapping was not found
    #[error("CLS class '{class}' not found for article '{article}'. The ALB package may be missing or corrupted.")]
    ClassNotFound { article: String, class: String },

    /// Property manipulation error
    #[error("Property error for '{property}': {message}")]
    PropertyError { property: String, message: String },

    /// Price lookup or calculation error
    #[error("Price calculation failed for article '{article}': {message}")]
    PriceError { article: String, message: String },

    /// File or data I/O error
    #[error("I/O error accessing '{path}': {message}")]
    IoError { path: String, message: String },

    /// CLS interpreter execution error
    #[error("Interpreter error in class '{class}': {message}")]
    InterpreterError { class: String, message: String },

    /// Data format error in OCD or EBase files
    #[error("Data format error in '{file}': {message}")]
    DataFormatError { file: String, message: String },

    /// Missing required data
    #[error("Missing required data: {0}")]
    MissingData(String),
}

/// Cache statistics for debugging and monitoring
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached OAM entries
    pub oam_entries: usize,
    /// Number of cached article lists
    pub article_entries: usize,
    /// Number of cached family loaders
    pub family_entries: usize,
    /// Number of cached OCD readers
    pub ocd_entries: usize,
}

/// An article enriched with OAM mapping information
#[derive(Debug, Clone)]
pub struct EnrichedArticle {
    /// Original OCD article data
    pub article: OcdArticle,
    /// Short description
    pub description: String,
    /// Whether this article has a CLS class (is configurable)
    pub is_configurable: bool,
    /// OAM mapping (if available)
    pub mapping: Option<ArticleMapping>,
}

impl EnrichedArticle {
    /// Convert to simple Article for display
    pub fn to_article(&self) -> Article {
        Article {
            id: self.article.article_nr.clone(),
            manufacturer_id: self.article.manufacturer.clone(),
            series_id: Some(self.article.series.clone()),
            short_description: self.description.clone(),
            long_description: None,
            base_article_number: self.article.article_nr.clone(),
            has_configuration: self.is_configurable,
        }
    }
}

/// A loaded configuration with properties from CLS class
#[derive(Debug, Clone)]
pub struct LoadedConfiguration {
    /// Article information
    pub article: EnrichedArticle,
    /// Manufacturer ID
    pub manufacturer_id: String,
    /// Property manager with definitions and values
    pub properties: PropertyManager,
    /// Current price (if available)
    pub price: Option<PriceResult>,
    /// Path to the ALB/package that contains this article
    pub package_path: Option<PathBuf>,
    /// OFML class name
    pub ofml_class: Option<String>,
}

/// Configuration Engine - main API for article configuration
pub struct ConfigurationEngine {
    /// Path to ofmldata directory
    data_path: PathBuf,
    /// Cached OAM data per manufacturer
    oam_cache: HashMap<String, OamData>,
    /// Cached article lists per manufacturer
    article_cache: HashMap<String, Vec<EnrichedArticle>>,
    /// Cached family loaders per manufacturer
    family_cache: HashMap<String, FamilyLoader>,
    /// ALB loader for loading CLS classes
    alb_loader: AlbLoader,
}

impl ConfigurationEngine {
    /// Create a new configuration engine
    pub fn new(data_path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = data_path.into();
        Self {
            alb_loader: AlbLoader::new(&path),
            data_path: path,
            oam_cache: HashMap::new(),
            article_cache: HashMap::new(),
            family_cache: HashMap::new(),
        }
    }

    /// Get cache statistics (includes global OCD cache)
    #[allow(dead_code)]
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            oam_entries: self.oam_cache.len(),
            article_entries: self.article_cache.len(),
            family_entries: self.family_cache.len(),
            ocd_entries: super::ocd::ocd_cache_size(),
        }
    }

    /// Load product families for a manufacturer (cached)
    #[instrument(skip(self), fields(manufacturer = %manufacturer_id))]
    pub fn load_families(&mut self, manufacturer_id: &str) -> &[ProductFamily] {
        if !self.family_cache.contains_key(manufacturer_id) {
            let mfr_path = self.data_path.join(manufacturer_id);
            let loader = FamilyLoader::load(&mfr_path, "DE");
            self.family_cache
                .insert(manufacturer_id.to_string(), loader);
        }
        self.family_cache
            .get(manufacturer_id)
            .map(|l| l.get_families())
            .unwrap_or(&[])
    }

    /// Get a product family by ID
    pub fn get_family(&mut self, manufacturer_id: &str, family_id: &str) -> Option<ProductFamily> {
        self.load_families(manufacturer_id);
        self.family_cache
            .get(manufacturer_id)
            .and_then(|l| l.get_family(family_id))
            .cloned()
    }

    /// Get configurable properties for a product family
    pub fn get_family_properties(
        &mut self,
        manufacturer_id: &str,
        family_id: &str,
    ) -> Vec<FamilyProperty> {
        self.load_families(manufacturer_id);
        if let Some(loader) = self.family_cache.get(manufacturer_id) {
            if let Some(family) = loader.get_family(family_id) {
                return loader.get_properties_for_family(family);
            }
        }
        Vec::new()
    }

    /// Get configurable properties for a product family with current selections
    ///
    /// This method takes the current property selections into account when
    /// determining available values for TABLE-based properties. Use this
    /// when you need to refresh property options after a selection change.
    pub fn get_family_properties_with_selections(
        &mut self,
        manufacturer_id: &str,
        family_id: &str,
        current_selections: &std::collections::HashMap<String, String>,
    ) -> Vec<FamilyProperty> {
        self.load_families(manufacturer_id);
        if let Some(loader) = self.family_cache.get(manufacturer_id) {
            if let Some(family) = loader.get_family(family_id) {
                return loader.get_properties_for_family_with_selections(family, current_selections);
            }
        }
        Vec::new()
    }

    /// Get packaging information for an article
    ///
    /// Returns the first matching packaging entry for the given article.
    /// Dimensions are in the unit specified in the packaging data (usually mm or cm).
    pub fn get_packaging_for_article(
        &self,
        manufacturer_id: &str,
        article_nr: &str,
    ) -> Option<super::ocd_properties::OcdPackaging> {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        let packaging = prop_reader.get_packaging(article_nr);
        packaging.first().cloned().cloned()
    }

    /// Get data version information for a manufacturer
    ///
    /// Returns a formatted version string including validity dates if available.
    pub fn get_data_version(&self, manufacturer_id: &str) -> Option<String> {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        prop_reader.get_data_version().map(|v| {
            if !v.date_from.is_empty() && !v.date_to.is_empty() {
                format!("{} ({} - {})", v.data_version, v.date_from, v.date_to)
            } else if !v.date_from.is_empty() {
                format!("{} (ab {})", v.data_version, v.date_from)
            } else {
                v.data_version.clone()
            }
        })
    }

    /// Check if data is valid for a given date and return a warning if not
    pub fn get_data_validity_warning(
        &self,
        manufacturer_id: &str,
        date: chrono::NaiveDate,
    ) -> Option<String> {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        prop_reader.get_data_validity_warning(date)
    }

    /// Format a variant code using manufacturer's code scheme
    pub fn format_variant_code(&self, manufacturer_id: &str, variant_code: &str) -> String {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        prop_reader.format_variant_code(variant_code)
    }

    /// Get the variant code separator from manufacturer's code scheme
    ///
    /// Returns the separator character(s) used in variant codes, defaults to "_"
    pub fn get_varcode_separator(&self, manufacturer_id: &str) -> String {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);

        // Try to find a code scheme and return its separator
        if let Some(scheme) = prop_reader
            .get_code_scheme("DEFAULT")
            .or_else(|| prop_reader.get_code_scheme("1"))
        {
            if !scheme.varcode_sep.is_empty() {
                return scheme.varcode_sep.clone();
            }
        }
        "_".to_string()
    }

    /// Check if a product is a composite
    pub fn is_composite(&self, manufacturer_id: &str, product_id: &str) -> bool {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        prop_reader.get_composite(product_id).is_some()
    }

    /// Get composite information for a product
    pub fn get_composite_info(
        &self,
        manufacturer_id: &str,
        product_id: &str,
    ) -> Option<super::ocd_properties::OcdComposite> {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        prop_reader.get_composite(product_id).cloned()
    }

    /// Get components (bill of items) for a composite product
    pub fn get_composite_components(
        &self,
        manufacturer_id: &str,
        composite_id: &str,
    ) -> Vec<super::ocd_properties::OcdBillOfItems> {
        let mfr_path = self.data_path.join(manufacturer_id);
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);
        prop_reader
            .get_bill_of_items(composite_id)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Create a new configuration for a product family
    pub fn create_family_configuration(
        &mut self,
        manufacturer_id: &str,
        family_id: &str,
    ) -> Option<FamilyConfiguration> {
        let properties = self.get_family_properties(manufacturer_id, family_id);
        if properties.is_empty() {
            // No OCD properties - try to create from family's article
            let family = self.get_family(manufacturer_id, family_id)?;
            Some(FamilyConfiguration::new(&family.id, &[]))
        } else {
            Some(FamilyConfiguration::new(family_id, &properties))
        }
    }

    /// Calculate price for a family configuration
    #[instrument(skip(self, family, config), fields(manufacturer = %manufacturer_id, family = %family.id))]
    pub fn calculate_family_price(
        &self,
        manufacturer_id: &str,
        family: &ProductFamily,
        config: &FamilyConfiguration,
        price_date: chrono::NaiveDate,
    ) -> Option<PriceResult> {
        let mfr_path = self.data_path.join(manufacturer_id);

        debug!(
            manufacturer = manufacturer_id,
            family = family.id,
            article = family.base_article_nr,
            "Calculating family price"
        );

        // Load property reader to compute var_cond from TABLE relations
        let prop_reader = super::ocd_properties::load_manufacturer_properties(&mfr_path);

        // Try to compute var_cond using TABLE relations (for manufacturers like FAST)
        let computed_varcond = if prop_reader.uses_table_varcond() {
            // Use first property class from the family
            let prop_class = family
                .prop_classes
                .first()
                .map(|s| s.as_str())
                .unwrap_or("");
            let result =
                prop_reader.compute_varcond_from_selections(prop_class, &config.selections);
            if let Some(ref vc) = result {
                debug!(var_cond = vc, "TABLE-computed var_cond");
            }
            result
        } else {
            None
        };

        // Find pdata.ebase files (cached)
        let pdata_files = super::ocd::find_pdata_files(&mfr_path);
        trace!(file_count = pdata_files.len(), "Found pdata.ebase files");

        // Helper to evaluate relation rules for a specific pdata file
        // Only used when the file contains our family's articles and uses relation-based pricing
        let evaluate_relation_rules = |pdata_path: &std::path::Path| -> Vec<String> {
            RelationRuleReader::from_ebase(pdata_path)
                .and_then(|rule_reader| {
                    if rule_reader.has_pricing_rules() {
                        // Build property map with M_ prefix (as used in relation rules)
                        // Note: Some property keys already have M_ prefix, so don't double it
                        let mut props: HashMap<String, String> = config
                            .selections
                            .iter()
                            .map(|(k, v)| {
                                let key = k.to_uppercase();
                                let prop_key = if key.starts_with("M_") {
                                    key
                                } else {
                                    format!("M_{}", key)
                                };
                                (prop_key, v.to_uppercase())
                            })
                            .collect();
                        // Add article number (used in conditions like M_ARTNO = 'ONE')
                        props.insert(
                            "M_ARTNO".to_string(),
                            family.base_article_nr.to_uppercase(),
                        );

                        let matched = rule_reader.evaluate(&props);
                        if !matched.is_empty() {
                            debug!(
                                file = ?pdata_path.file_name(),
                                varconds = ?matched,
                                "Relation rules matched var_conds"
                            );
                            Some(matched)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        };

        // FIRST PASS: Look for files with ACTUAL article base prices (not just wildcards)
        // This ensures we find the correct series file for the article
        for pdata_path in &pdata_files {
            if let Some(reader) = super::ocd::get_ocd_reader(pdata_path) {
                // Check if this file has the actual article with a base price
                let has_actual_base = reader
                    .prices
                    .iter()
                    .any(|p| p.article_nr == family.base_article_nr && p.price_level == "B");

                if has_actual_base {
                    debug!(
                        file = ?pdata_path.file_name(),
                        article = family.base_article_nr,
                        "Found exact article base price"
                    );
                    let prices = reader.get_prices(&family.base_article_nr);
                    // Evaluate relation rules for this file (some families use both base prices AND relation rules)
                    let file_relation_varconds = evaluate_relation_rules(pdata_path);
                    return self.match_and_calculate_price_with_varcond(
                        &reader,
                        &prices,
                        config,
                        price_date,
                        computed_varcond.as_deref(),
                        &file_relation_varconds,
                        &prop_reader,
                        &family.base_article_nr,
                    );
                }
            }
        }

        trace!("No exact base price found, trying alternate articles and wildcards");

        // SECOND PASS: Try alternate articles in the family (they may have base prices)
        for pdata_path in &pdata_files {
            if let Some(reader) = super::ocd::get_ocd_reader(pdata_path) {
                // First, try other articles in the family that have actual base prices
                for art_nr in &family.article_nrs {
                    // Check if this file has actual base price for this article
                    let has_base = reader
                        .prices
                        .iter()
                        .any(|p| p.article_nr == *art_nr && p.price_level == "B");

                    if has_base {
                        let prices = reader.get_prices(art_nr);
                        debug!(
                            file = ?pdata_path.file_name(),
                            article = art_nr,
                            "Found base price via alternate family article"
                        );
                        // Evaluate relation rules for this file
                        let file_relation_varconds = evaluate_relation_rules(pdata_path);
                        return self.match_and_calculate_price_with_varcond(
                            &reader,
                            &prices,
                            config,
                            price_date,
                            computed_varcond.as_deref(),
                            &file_relation_varconds,
                            &prop_reader,
                            art_nr,
                        );
                    }
                }
            }
        }

        trace!("No alternate article base price found, trying surcharge-only or wildcard");

        // THIRD PASS: Fall back to surcharge-only pricing (only use if file contains the article)
        for pdata_path in &pdata_files {
            if let Some(reader) = super::ocd::get_ocd_reader(pdata_path) {
                // Only use surcharge-only pricing if this file is specifically for this series
                // Check if any of the family's articles appear in this file's article table
                let file_has_family_articles = family.article_nrs.iter().any(|art| {
                    reader.articles.iter().any(|a| a.article_nr == *art)
                });

                if !file_has_family_articles {
                    // This file isn't for our family, skip it
                    continue;
                }

                // For surcharge-only pricing, get wildcard prices
                let prices = reader.get_prices(&family.base_article_nr);
                if prices.is_empty() {
                    continue;
                }

                // Verify this is actually surcharge-only (no base prices for our articles)
                let has_any_base = family.article_nrs.iter().any(|art| {
                    reader
                        .prices
                        .iter()
                        .any(|p| p.article_nr == *art && p.price_level == "B")
                });

                if has_any_base {
                    // This file has base prices, but we didn't find them in earlier passes
                    // Something went wrong, skip
                    continue;
                }

                let strategy = reader.detect_pricing_strategy();
                // For surcharge-only pricing, evaluate relation rules from this specific file
                // These rules determine which surcharges apply based on property conditions
                let file_relation_varconds = evaluate_relation_rules(pdata_path);

                debug!(
                    file = ?pdata_path.file_name(),
                    price_count = prices.len(),
                    strategy = ?strategy,
                    relation_rules = file_relation_varconds.len(),
                    "Using surcharge-only prices from series file"
                );

                return self.match_and_calculate_price_with_varcond(
                    &reader,
                    &prices,
                    config,
                    price_date,
                    computed_varcond.as_deref(),
                    &file_relation_varconds,
                    &prop_reader,
                    &family.base_article_nr,
                );
            }
        }

        warn!(
            manufacturer = manufacturer_id,
            article = family.base_article_nr,
            "No price found for article"
        );
        None
    }

    /// Match prices and calculate total, with optional computed var_cond
    #[allow(clippy::too_many_arguments)]
    fn match_and_calculate_price_with_varcond(
        &self,
        reader: &OcdReader,
        prices: &[&OcdPrice],
        config: &FamilyConfiguration,
        price_date: chrono::NaiveDate,
        computed_varcond: Option<&str>,
        relation_varconds: &[String],
        prop_reader: &super::ocd_properties::OcdPropertyReader,
        article_nr: &str,
    ) -> Option<PriceResult> {
        let matched = match_prices_to_variant_with_computed_varcond(
            reader,
            prices,
            &config.variant_code,
            computed_varcond,
            relation_varconds,
        )?;

        let valid_from = chrono::NaiveDate::parse_from_str(&matched.base_price.date_from, "%Y%m%d")
            .unwrap_or(price_date);
        let valid_to =
            chrono::NaiveDate::parse_from_str(&matched.base_price.date_to, "%Y%m%d").ok();

        // Combine surcharges and discounts (discounts as negative surcharges)
        let mut all_surcharges = matched.surcharges;
        for discount in matched.discounts {
            all_surcharges.push(Surcharge {
                name: format!("Rabatt: {}", discount.name),
                amount: -discount.amount, // Negate amount for discounts
                is_percentage: discount.is_percentage,
            });
        }

        // Calculate net price first (for tax calculation)
        let net_price = PriceResult::compute_net_static(&matched.base_amount, &all_surcharges);

        // Look up taxes for this article
        let taxes = self.calculate_taxes_for_article(prop_reader, article_nr, net_price);

        // Apply rounding rules if available
        // Look for a default rounding rule (commonly "DEFAULT" or "PRICE")
        let rounded_prices = self.apply_rounding_rules(prop_reader, &matched.base_amount, &net_price, &taxes);

        Some(PriceResult::with_taxes_and_rounding(
            rounded_prices.base_price,
            all_surcharges,
            taxes,
            rounded_prices.net_price,
            rounded_prices.total_price,
            matched.base_price.currency.clone(),
            price_date,
            valid_from,
            valid_to,
        ))
    }

    /// Apply rounding rules to prices
    fn apply_rounding_rules(
        &self,
        prop_reader: &super::ocd_properties::OcdPropertyReader,
        base_price: &rust_decimal::Decimal,
        net_price: &rust_decimal::Decimal,
        taxes: &[super::TaxEntry],
    ) -> RoundedPrices {
        use rust_decimal::Decimal;

        // Try common rounding rule IDs
        let rounding_ids = ["DEFAULT", "PRICE", "STANDARD", "1"];

        for id in &rounding_ids {
            let rules = prop_reader.get_rounding_rules(id);
            if !rules.is_empty() {
                // Apply rounding to total price
                let tax_total: Decimal = taxes.iter().map(|t| t.amount).sum();
                let total = *net_price + tax_total;

                let rounded_total = Decimal::from_f64_retain(
                    prop_reader.apply_rounding(id, total.to_string().parse::<f64>().unwrap_or(0.0))
                ).unwrap_or(total);

                return RoundedPrices {
                    base_price: *base_price,
                    net_price: *net_price,
                    total_price: rounded_total,
                };
            }
        }

        // No rounding rules found, return unrounded prices
        let tax_total: Decimal = taxes.iter().map(|t| t.amount).sum();
        RoundedPrices {
            base_price: *base_price,
            net_price: *net_price,
            total_price: *net_price + tax_total,
        }
    }

    /// Calculate taxes for an article based on tax schemes
    fn calculate_taxes_for_article(
        &self,
        prop_reader: &super::ocd_properties::OcdPropertyReader,
        article_nr: &str,
        net_price: rust_decimal::Decimal,
    ) -> Vec<super::TaxEntry> {
        use rust_decimal::Decimal;

        let article_taxes = prop_reader.get_article_taxes(article_nr);
        if article_taxes.is_empty() {
            return Vec::new();
        }

        let mut taxes = Vec::new();
        for tax_assignment in article_taxes {
            if let Some(scheme) = prop_reader.get_tax_scheme(&tax_assignment.tax_id) {
                // Only apply percentage-based taxes (most common)
                if scheme.tax_type.eq_ignore_ascii_case("PERCENT")
                    || scheme.tax_type.eq_ignore_ascii_case("VAT")
                {
                    let rate = Decimal::from_f64_retain(scheme.number).unwrap_or_default();
                    let amount = net_price * rate / Decimal::from(100);
                    taxes.push(super::TaxEntry {
                        name: format!("MwSt ({}%)", scheme.number),
                        category: scheme.tax_category.clone(),
                        rate,
                        amount,
                    });
                }
            }
        }
        taxes
    }

    /// Load OAM data for a manufacturer (cached)
    pub fn load_oam(&mut self, manufacturer_id: &str) -> &OamData {
        // Use entry API to avoid separate contains_key + get + unwrap
        self.oam_cache
            .entry(manufacturer_id.to_string())
            .or_insert_with(|| {
                let mfr_path = self.data_path.join(manufacturer_id);
                load_manufacturer_oam(&mfr_path)
            })
    }

    /// Load articles for a manufacturer with OAM enrichment
    pub fn load_articles(&mut self, manufacturer_id: &str) -> Vec<EnrichedArticle> {
        // Check cache first
        if let Some(cached) = self.article_cache.get(manufacturer_id) {
            return cached.clone();
        }

        let mfr_path = self.data_path.join(manufacturer_id);

        // Load OAM data first
        let oam_data = load_manufacturer_oam(&mfr_path);

        // Load OCD articles
        let articles_with_desc = load_articles_with_descriptions(&mfr_path, "DE");

        // Enrich articles with OAM data and filter out internal/cryptic articles
        let enriched: Vec<EnrichedArticle> = articles_with_desc
            .into_iter()
            .filter(|(article, _)| {
                // Filter out internal articles (starting with @)
                if article.article_nr.starts_with('@') {
                    return false;
                }
                // Filter out articles with control characters in series
                if article.series.chars().any(|c| c.is_control()) {
                    return false;
                }
                true
            })
            .map(|(article, description)| {
                let mapping = oam_data.get_mapping(&article.article_nr).cloned();
                let is_configurable = mapping.is_some();

                EnrichedArticle {
                    article,
                    description,
                    is_configurable,
                    mapping,
                }
            })
            .collect();

        // Cache and return
        self.oam_cache.insert(manufacturer_id.to_string(), oam_data);
        self.article_cache
            .insert(manufacturer_id.to_string(), enriched.clone());

        enriched
    }

    /// Get an article by its number
    pub fn get_article(
        &mut self,
        manufacturer_id: &str,
        article_nr: &str,
    ) -> Option<EnrichedArticle> {
        let articles = self.load_articles(manufacturer_id);
        articles
            .into_iter()
            .find(|a| a.article.article_nr == article_nr)
    }

    /// Load configuration for an article using CLS interpreter
    pub fn load_configuration(
        &mut self,
        manufacturer_id: &str,
        article_nr: &str,
    ) -> Result<LoadedConfiguration, EngineError> {
        // Get the enriched article
        let article = self
            .get_article(manufacturer_id, article_nr)
            .ok_or_else(|| EngineError::ArticleNotFound {
                article: article_nr.to_string(),
                manufacturer: manufacturer_id.to_string(),
            })?;

        // Check if configurable
        let mapping = article
            .mapping
            .clone()
            .ok_or_else(|| EngineError::NotConfigurable {
                article: article_nr.to_string(),
            })?;

        // Try to extract properties from the CLS class
        let (properties, package_path) =
            self.extract_properties_for_class(manufacturer_id, &mapping)?;

        Ok(LoadedConfiguration {
            article,
            manufacturer_id: manufacturer_id.to_string(),
            properties,
            price: None,
            package_path,
            ofml_class: Some(mapping.ofml_type.clone()),
        })
    }

    /// Extract properties from a CLS class using the interpreter
    fn extract_properties_for_class(
        &mut self,
        manufacturer_id: &str,
        mapping: &ArticleMapping,
    ) -> Result<(PropertyManager, Option<PathBuf>), EngineError> {
        let class_name = &mapping.ofml_type;

        // Parse the class path to find ALB files
        // e.g., "::vitra::abc::aAddOn" -> vitra/abc
        let parts: Vec<&str> = class_name.split("::").filter(|s| !s.is_empty()).collect();

        if parts.len() < 2 {
            // Generic OFML class like ::ofml::oi::OiOdbPlElement - no custom properties
            return Ok((PropertyManager::new(), None));
        }

        // Skip if it's a built-in OFML class
        if parts.first() == Some(&"ofml") {
            return Ok((PropertyManager::new(), None));
        }

        let mfr_path = self.data_path.join(manufacturer_id);
        let series = parts.get(1).unwrap_or(&"");

        // Find ALB files for this series
        let alb_files = find_alb_files(&mfr_path, series);

        if alb_files.is_empty() {
            return Ok((PropertyManager::new(), None));
        }

        // Load ALB files and try to find the class
        let short_class_name = parts.last().unwrap_or(&"").to_string();

        for alb_path in &alb_files {
            if let Ok(cls_sources) = self.alb_loader.load_alb(alb_path) {
                // Check if any source contains our class
                for cls_source in &cls_sources {
                    if cls_source
                        .classes
                        .iter()
                        .any(|c| c == &short_class_name || c == class_name)
                    {
                        // Found the class - try to parse and extract properties
                        if let Ok(properties) =
                            extract_properties_from_source(&cls_source.source, &short_class_name)
                        {
                            if !properties.definitions.is_empty() {
                                return Ok((properties, Some(alb_path.clone())));
                            }
                        }
                    }
                }
            }
        }

        // Also try to load basics/global ALBs which may contain parent classes
        for base_dir in &["basics", "global"] {
            let base_albs = find_alb_files(&mfr_path, base_dir);
            for alb_path in &base_albs {
                let _ = self.alb_loader.load_alb(alb_path);
            }
        }

        // Fallback: extract from any loaded source that matches
        for sources in self.alb_loader.sources.values() {
            for cls_source in sources {
                if cls_source.classes.contains(&short_class_name) {
                    if let Ok(properties) =
                        extract_properties_from_source(&cls_source.source, &short_class_name)
                    {
                        if !properties.definitions.is_empty() {
                            return Ok((properties, None));
                        }
                    }
                }
            }
        }

        Ok((PropertyManager::new(), None))
    }

    /// Calculate price for a configuration using var_cond matching
    pub fn calculate_price(
        &self,
        manufacturer_id: &str,
        article_nr: &str,
        properties: &PropertyManager,
        price_date: chrono::NaiveDate,
    ) -> Result<Option<PriceResult>, EngineError> {
        let mfr_path = self.data_path.join(manufacturer_id);

        // Find pdata.ebase files
        let pdata_files = super::ocd::find_pdata_files(&mfr_path);

        for pdata_path in &pdata_files {
            if let Ok(reader) = OcdReader::from_ebase(pdata_path) {
                // Get all prices for this article
                let prices = reader.get_prices(article_nr);

                if prices.is_empty() {
                    continue;
                }

                // Generate variant code from properties
                let variant_code = generate_variant_code(properties);

                // Match prices against variant code
                let matched = match_prices_to_variant(&reader, &prices, &variant_code);

                if let Some(price_result) = matched {
                    // Parse dates and create PriceResult
                    let valid_from = chrono::NaiveDate::parse_from_str(
                        &price_result.base_price.date_from,
                        "%Y%m%d",
                    )
                    .unwrap_or(price_date);
                    let valid_to = chrono::NaiveDate::parse_from_str(
                        &price_result.base_price.date_to,
                        "%Y%m%d",
                    )
                    .ok();

                    // Combine surcharges and discounts (discounts as negative surcharges)
                    let mut all_surcharges = price_result.surcharges;
                    for discount in price_result.discounts {
                        all_surcharges.push(Surcharge {
                            name: format!("Rabatt: {}", discount.name),
                            amount: -discount.amount, // Negate amount for discounts
                            is_percentage: discount.is_percentage,
                        });
                    }

                    return Ok(Some(PriceResult::new(
                        price_result.base_amount,
                        all_surcharges,
                        price_result.base_price.currency.clone(),
                        price_date,
                        valid_from,
                        valid_to,
                    )));
                }
            }
        }

        Ok(None)
    }

    /// Set a property value and trigger updates
    pub fn set_property(
        &mut self,
        config: &mut LoadedConfiguration,
        key: &str,
        value: PropertyValue,
    ) -> Result<(), EngineError> {
        config.properties.set(key, value).map_err(|e| {
            EngineError::PropertyError {
                property: key.to_string(),
                message: e.to_string(),
            }
        })?;

        Ok(())
    }
}

/// Find ALB files for a series within a manufacturer directory
fn find_alb_files(mfr_path: &Path, series: &str) -> Vec<PathBuf> {
    let mut alb_files = Vec::new();

    // Look in the series directory
    let series_path = mfr_path.join(series);
    if series_path.is_dir() {
        find_alb_recursive(&series_path, &mut alb_files);
    }

    alb_files
}

fn find_alb_recursive(path: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                find_alb_recursive(&entry_path, files);
            } else if entry_path.extension().is_some_and(|e| e == "alb") {
                files.push(entry_path);
            }
        }
    }
}

/// Extract properties from CLS source code
fn extract_properties_from_source(
    source: &str,
    class_name: &str,
) -> Result<PropertyManager, EngineError> {
    let mut properties = PropertyManager::new();

    // Parse the source
    let mut parser = Parser::new(source).map_err(|e| EngineError::InterpreterError {
        class: class_name.to_string(),
        message: format!("Parse error: {}", e),
    })?;
    let ast = parser.parse().map_err(|e| EngineError::InterpreterError {
        class: class_name.to_string(),
        message: format!("Parse error: {}", e),
    })?;

    // Find the class definition in statements and look for setupProperty calls
    for stmt in &ast.statements {
        if let Stmt::Class(class_def) = stmt {
            if class_def.name == class_name || class_name.ends_with(&class_def.name) {
                // Search ALL methods for setupProperty calls
                // Properties can be defined in initialize, setup, updateSize, or other methods
                for member in &class_def.members {
                    if let ClassMember::Func(func) = member {
                        // Extract setupProperty calls from any method body
                        if let Some(body) = &func.body {
                            extract_setup_properties_from_stmts(&body.stmts, &mut properties);
                        }
                    }
                }
            }
        }
    }

    Ok(properties)
}

/// Extract property definitions from setupProperty calls in AST statements
fn extract_setup_properties_from_stmts(stmts: &[Stmt], properties: &mut PropertyManager) {
    for stmt in stmts {
        match stmt {
            Stmt::Expr(expr) => {
                extract_setup_properties_from_expr(expr, properties);
            }
            Stmt::If(if_stmt) => {
                if let Stmt::Block(block) = if_stmt.then_branch.as_ref() {
                    extract_setup_properties_from_stmts(&block.stmts, properties);
                }
                if let Some(else_stmt) = &if_stmt.else_branch {
                    if let Stmt::Block(block) = else_stmt.as_ref() {
                        extract_setup_properties_from_stmts(&block.stmts, properties);
                    }
                }
            }
            Stmt::Block(block) => {
                extract_setup_properties_from_stmts(&block.stmts, properties);
            }
            _ => {}
        }
    }
}

/// Extract property definition from a setupProperty call expression
fn extract_setup_properties_from_expr(expr: &Expr, properties: &mut PropertyManager) {
    match expr {
        Expr::Call(call_expr) => {
            // Check if this is a setupProperty call
            let is_setup_property = match call_expr.callee.as_ref() {
                Expr::Ident(name) => name == "setupProperty",
                Expr::Member(member_expr) => member_expr.member == "setupProperty",
                _ => false,
            };

            if is_setup_property && !call_expr.args.is_empty() {
                // First argument is the property key (symbol)
                if let Expr::Symbol(key) = &call_expr.args[0] {
                    let prop_name = key.to_uppercase();

                    // Second argument is usually an array with property definition
                    let (label, prop_type) = if call_expr.args.len() > 1 {
                        parse_property_def_from_expr(&call_expr.args[1])
                    } else {
                        (prop_name.clone(), PropertyType::String)
                    };

                    let prop_def = PropertyDef::new(&prop_name, &label, prop_type);
                    properties.register(prop_def);
                }
            }

            // Also check nested calls
            for arg in &call_expr.args {
                extract_setup_properties_from_expr(arg, properties);
            }
        }
        Expr::Member(member_expr) => {
            extract_setup_properties_from_expr(&member_expr.object, properties);
        }
        _ => {}
    }
}

/// Parse property definition from array expression
fn parse_property_def_from_expr(expr: &Expr) -> (String, PropertyType) {
    if let Expr::Array(elements) = expr {
        let mut label = String::new();
        let mut prop_type = PropertyType::String;
        let mut options: Vec<String> = Vec::new();
        let mut min_val: Option<i64> = None;
        let mut max_val: Option<i64> = None;

        for (i, elem) in elements.iter().enumerate() {
            match elem {
                Expr::String(s) => {
                    if i == 0 || i == 1 {
                        // First or second string is the label
                        if label.is_empty() {
                            label = s.clone();
                        }
                    }
                }
                Expr::Symbol(s) => {
                    let upper = s.to_uppercase();
                    match upper.as_str() {
                        "INT" | "INTEGER" => {
                            prop_type = PropertyType::Int {
                                min: None,
                                max: None,
                            };
                        }
                        "FLOAT" | "REAL" => {
                            prop_type = PropertyType::Float {
                                min: None,
                                max: None,
                            };
                        }
                        "BOOL" | "BOOLEAN" => {
                            prop_type = PropertyType::Bool;
                        }
                        "CHOICE" | "ENUM" => {
                            // Options should follow
                        }
                        _ => {
                            // Could be an option value
                            options.push(s.clone());
                        }
                    }
                }
                Expr::Array(inner) => {
                    // Could be range [min, max] or options list
                    if inner.len() == 2 {
                        // Range
                        if let (Some(min), Some(max)) =
                            (expr_to_int(&inner[0]), expr_to_int(&inner[1]))
                        {
                            min_val = Some(min);
                            max_val = Some(max);
                        }
                    } else {
                        // Options list
                        for opt in inner {
                            if let Expr::String(s) = opt {
                                options.push(s.clone());
                            } else if let Expr::Symbol(s) = opt {
                                options.push(s.clone());
                            }
                        }
                    }
                }
                Expr::Int(n) => {
                    // Could be min/max or default
                    if min_val.is_none() {
                        min_val = Some(*n);
                    } else if max_val.is_none() {
                        max_val = Some(*n);
                    }
                }
                _ => {}
            }
        }

        // Finalize property type
        if !options.is_empty() {
            prop_type = PropertyType::Choice { options };
        } else if let PropertyType::Int { .. } = prop_type {
            prop_type = PropertyType::Int {
                min: min_val,
                max: max_val,
            };
        }

        if label.is_empty() {
            label = "Unknown".to_string();
        }

        (label, prop_type)
    } else {
        ("Unknown".to_string(), PropertyType::String)
    }
}

fn expr_to_int(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Int(n) => Some(*n),
        Expr::Float(f) => Some(*f as i64),
        _ => None,
    }
}

/// Generate variant code from property values
fn generate_variant_code(properties: &PropertyManager) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Sort properties by name for consistent ordering
    let mut prop_names: Vec<_> = properties.values.keys().collect();
    prop_names.sort();

    for name in prop_names {
        if let Some(value) = properties.values.get(name) {
            let value_str = match value {
                PropertyValue::Int(i) => i.to_string(),
                PropertyValue::Float(f) => format!("{:.0}", f),
                PropertyValue::Bool(b) => if *b { "1" } else { "0" }.to_string(),
                PropertyValue::String(s) => s.clone(),
                PropertyValue::Symbol(s) => s.clone(),
            };
            parts.push(format!("{}={}", name, value_str));
        }
    }

    parts.join(";")
}

/// Matched price result with base, surcharges, and discounts
struct MatchedPrice<'a> {
    base_price: &'a OcdPrice,
    base_amount: Decimal,
    surcharges: Vec<Surcharge>,
    discounts: Vec<Surcharge>, // Using Surcharge struct for discounts too (amount is positive, we subtract)
}

/// Match prices to variant code using OCD 4.3 price_level field
/// Uses propvalue2varcond table for direct lookup when available,
/// otherwise falls back to pattern matching
fn match_prices_to_variant<'a>(
    reader: &OcdReader,
    prices: &'a [&'a OcdPrice],
    variant_code: &str,
) -> Option<MatchedPrice<'a>> {
    match_prices_to_variant_with_computed_varcond(reader, prices, variant_code, None, &[])
}

/// Match prices to variant code, with optional computed var_cond for TABLE-based manufacturers
/// and relation-evaluated var_conds from ocd_relation rules
fn match_prices_to_variant_with_computed_varcond<'a>(
    reader: &OcdReader,
    prices: &'a [&'a OcdPrice],
    variant_code: &str,
    computed_varcond: Option<&str>,
    relation_varconds: &[String],
) -> Option<MatchedPrice<'a>> {
    // Known base price indicators (fallback for older data without price_level)
    let base_indicators = ["S_PGX", "BASE", "STANDARD", ""];

    // Extract values from variant code for matching (case-insensitive)
    let variant_values: std::collections::HashSet<String> = variant_code
        .split(';')
        .filter_map(|part| part.split('=').nth(1))
        .map(|s| s.to_uppercase())
        .collect();

    // Also keep original case values for backwards compatibility
    let variant_values_original: std::collections::HashSet<&str> = variant_code
        .split(';')
        .filter_map(|part| part.split('=').nth(1))
        .collect();

    // STEP 1: Find base price
    // First try computed var_cond (from TABLE relations) if provided
    let base_price_opt = if let Some(computed) = computed_varcond {
        // Use the computed var_cond to find the matching base price
        prices.iter().find(|p| {
            p.price_level == "B"
                && !p.var_cond.is_empty()
                && p.var_cond.eq_ignore_ascii_case(computed)
        })
    } else {
        None
    }
    .or_else(|| {
        // Try to match one with matching var_cond from variant values
        // This handles manufacturers like "fast" where each variant has its own base price
        prices.iter().find(|p| {
            p.price_level == "B"
                && !p.var_cond.is_empty()
                && variant_values.contains(&p.var_cond.to_uppercase())
        })
    })
    .or_else(|| {
        // Fallback: any base price with empty var_cond or base indicators
        prices.iter().find(|p| {
            p.price_level == "B"
                && (p.var_cond.is_empty() || base_indicators.contains(&p.var_cond.as_str()))
        })
    })
    .or_else(|| {
        // Fallback: first base price
        prices.iter().find(|p| p.price_level == "B")
    })
    .or_else(|| {
        // Legacy fallback: empty var_cond or known base indicators
        prices.iter().find(|p| {
            base_indicators
                .iter()
                .any(|ind| p.var_cond == *ind || p.var_cond.is_empty())
        })
    });

    // Check if this is surcharge-only pricing (like Framery)
    let is_surcharge_only = base_price_opt.is_none() && reader.has_surcharge_only_pricing();

    // For surcharge-only pricing, use first surcharge as base reference (with zero base amount)
    // The actual pricing will come from summing applicable surcharges
    // Otherwise require a base price
    let base_price = if is_surcharge_only {
        prices.iter().find(|p| p.price_level == "X")
    } else {
        base_price_opt.or_else(|| prices.first())
    }?;

    // For surcharge-only pricing, base amount is 0 (all value comes from surcharges)
    let base_amount = if is_surcharge_only {
        Decimal::ZERO
    } else {
        Decimal::from_f32_retain(base_price.price).unwrap_or(Decimal::ZERO)
    };

    // Build variant map for heuristic matching
    let variant_map: HashMap<&str, &str> = variant_code
        .split(';')
        .filter_map(|part| {
            let mut split = part.splitn(2, '=');
            Some((split.next()?, split.next()?))
        })
        .collect();

    // STEP 2: Find matching surcharges (price_level='X')
    let mut surcharges = Vec::new();
    let mut seen_var_conds = std::collections::HashSet::new();

    // Build set of expected var_conds using propvalue2varcond direct lookup
    let direct_var_conds: std::collections::HashSet<String> = if reader.has_varcond_mappings() {
        // Use propvalue2varcond table for direct lookup (100% accurate)
        let values: Vec<&str> = variant_values_original.iter().copied().collect();
        reader
            .lookup_varconds_for_values(&values)
            .into_iter()
            .collect()
    } else {
        std::collections::HashSet::new()
    };

    // Build uppercase version of direct_var_conds for case-insensitive matching
    let direct_var_conds_upper: std::collections::HashSet<String> =
        direct_var_conds.iter().map(|s| s.to_uppercase()).collect();

    for price in prices {
        // Only process surcharges (price_level='X') or prices with non-empty var_cond
        let is_surcharge = price.price_level == "X"
            || (!price.var_cond.is_empty() && price.price_level != "B" && price.price_level != "D");

        if !is_surcharge {
            continue;
        }

        // Skip base price indicators and already processed
        if base_indicators.contains(&price.var_cond.as_str()) && price.price_level != "X" {
            continue;
        }
        if seen_var_conds.contains(&price.var_cond) {
            continue;
        }

        // Check if var_cond matches using the best available method
        let is_match = if reader.has_varcond_mappings() {
            // Use direct lookup from propvalue2varcond table (preferred)
            // Try both exact match and case-insensitive match
            direct_var_conds.contains(&price.var_cond)
                || direct_var_conds_upper.contains(&price.var_cond.to_uppercase())
        } else if !relation_varconds.is_empty() {
            // Use relation-evaluated var_conds (from ocd_relation rules like F_PREISE)
            // This handles manufacturers like Framery that use conditional rules instead of propvalue2varcond
            relation_varconds.iter().any(|vc| vc.eq_ignore_ascii_case(&price.var_cond))
        } else {
            // Fallback to pattern matching for manufacturers without mapping table
            matches_var_cond_extended(
                &price.var_cond,
                variant_code,
                &variant_values_original,
                &variant_map,
            )
        };

        if is_match {
            let amount = Decimal::from_f32_retain(price.price).unwrap_or(Decimal::ZERO);
            // Get human-readable description from ocd_pricetext, fallback to var_cond
            let description = reader.get_price_description(price, "DE");
            surcharges.push(Surcharge {
                name: description,
                amount,
                is_percentage: !price.is_fix, // is_fix=1 means fixed amount, 0 means percentage
            });
            seen_var_conds.insert(price.var_cond.clone());
        }
    }

    // STEP 3: Find matching discounts (price_level='D')
    let mut discounts = Vec::new();
    let mut seen_discount_var_conds = std::collections::HashSet::new();

    for price in prices {
        if price.price_level != "D" {
            continue;
        }

        // Skip already processed discounts
        if seen_discount_var_conds.contains(&price.var_cond) {
            continue;
        }

        // Check if var_cond matches using the best available method
        let is_match = if reader.has_varcond_mappings() {
            // Use direct lookup from propvalue2varcond table (preferred)
            direct_var_conds.contains(&price.var_cond)
                || direct_var_conds_upper.contains(&price.var_cond.to_uppercase())
        } else if price.var_cond.is_empty() {
            // Empty var_cond discount applies to all
            true
        } else {
            // Fallback to pattern matching for manufacturers without mapping table
            matches_var_cond_extended(
                &price.var_cond,
                variant_code,
                &variant_values_original,
                &variant_map,
            )
        };

        if is_match {
            let amount = Decimal::from_f32_retain(price.price).unwrap_or(Decimal::ZERO);
            let description = reader.get_price_description(price, "DE");
            discounts.push(Surcharge {
                name: description,
                amount,
                is_percentage: !price.is_fix,
            });
            seen_discount_var_conds.insert(price.var_cond.clone());
        }
    }

    Some(MatchedPrice {
        base_price,
        base_amount,
        surcharges,
        discounts,
    })
}

/// Extended matching for OCD surcharge codes
/// Supports multiple matching strategies for manufacturer-specific formats
/// Language-agnostic: works with any descriptive var_cond (German, English, Spanish, etc.)
fn matches_var_cond_extended(
    var_cond: &str,
    variant_code: &str,
    variant_values: &std::collections::HashSet<&str>,
    variant_map: &HashMap<&str, &str>,
) -> bool {
    // Strategy 0: Direct case-insensitive match (language-agnostic descriptive names)
    // Handles: "PP AXA BLACK", "CODIGO_BASE", "ESTRUCTURA_01", "STOFFGRUPPE_3", etc.
    let var_cond_upper = var_cond.to_uppercase();
    for value in variant_values {
        if value.to_uppercase() == var_cond_upper {
            return true;
        }
    }

    // Strategy 0b: Price group pattern matching (PG11, PG90, GL1, MG1, etc.)
    // These are common across manufacturers regardless of language
    if var_cond.len() >= 2 && var_cond.len() <= 6 {
        let prefix = &var_cond[..2].to_uppercase();
        if prefix == "PG" || prefix == "GL" || prefix == "MG" {
            // Check if any property value matches this price group
            for value in variant_values {
                if value.to_uppercase() == var_cond_upper {
                    return true;
                }
            }
            // Also check property values in the map
            for value in variant_map.values() {
                if value.to_uppercase() == var_cond_upper {
                    return true;
                }
            }
        }
    }

    // Strategy 1: Direct formula matching (KEY=value, KEY>value, etc.)
    if matches_var_cond(var_cond, variant_code, variant_values) {
        return true;
    }

    // Strategy 2: Sedus-style surcharge codes (S_XXXX format)
    // Check if var_cond matches a pattern like S_<property_suffix>_<value_suffix>
    // Example: S_166 might match when S_MODELLFARBE=166 or similar
    if let Some(code) = var_cond.strip_prefix("S_") {
        // Try to find a matching property value
        // Pattern 1: Direct value match (e.g., code="166" matches value "166")
        if variant_values.contains(code) {
            return true;
        }

        // Pattern 2: Value ends with the code (e.g., code="166" matches "CSE166")
        for value in variant_values {
            if value.ends_with(code) {
                return true;
            }
        }

        // Pattern 3: Embedded numeric code matching for complex values
        // e.g., code="166" matches "XST244166018" which contains "166"
        // Only apply to short numeric codes (3-4 digits) to avoid false matches
        if code.len() >= 3 && code.len() <= 4 && code.chars().all(|c| c.is_ascii_digit()) {
            for value in variant_values {
                // Check if value contains the code at a word/number boundary
                // Look for patterns where the code is embedded in a larger string
                if value.contains(code) && value.len() > code.len() {
                    // Verify it's a meaningful match (not just random substring)
                    // Accept if the value is alphanumeric and code is clearly embedded
                    return true;
                }
            }
        }

        // Pattern 4: Split code by underscore for compound codes (e.g., S_2415_F2)
        // Check if first part matches property suffix and second matches value
        if let Some(pos) = code.find('_') {
            let (num_part, suffix) = code.split_at(pos);
            let suffix = &suffix[1..]; // Remove leading underscore

            // Check each property for matching pattern
            for (key, value) in variant_map {
                // If property key ends with numeric part and value starts with suffix
                if key.ends_with(num_part) && value.starts_with(suffix) {
                    return true;
                }
                // Or if value matches the full code
                if *value == code || value.ends_with(code) {
                    return true;
                }
            }
        }

        // Pattern 5: Numeric code as property value prefix/match
        // e.g., S_1801 matches if any property has value starting with "1801"
        if code.chars().all(|c| c.is_ascii_digit()) {
            for value in variant_values {
                if value.starts_with(code) || *value == code {
                    return true;
                }
            }
        }
    }

    false
}

/// Check if a var_cond matches the variant code
fn matches_var_cond(
    var_cond: &str,
    variant_code: &str,
    variant_values: &std::collections::HashSet<&str>,
) -> bool {
    if var_cond.is_empty() {
        return true;
    }

    // Parse var_cond conditions
    // Format examples: "COLOR=white", "WIDTH>1200", "WIDTH=1200;HEIGHT=720"
    let conditions: Vec<&str> = var_cond.split(';').collect();
    let variant_parts: HashMap<&str, &str> = variant_code
        .split(';')
        .filter_map(|part| {
            let mut split = part.splitn(2, '=');
            Some((split.next()?, split.next()?))
        })
        .collect();

    for cond in conditions {
        let cond = cond.trim();
        if cond.is_empty() {
            continue;
        }

        // Handle different operators
        if let Some(pos) = cond.find('=') {
            let key = &cond[..pos];
            let expected = &cond[pos + 1..];

            if let Some(actual) = variant_parts.get(key) {
                if *actual != expected {
                    return false;
                }
            } else {
                // Key not in variant - this condition doesn't apply
                return false;
            }
        } else if cond.contains('>') {
            // Greater than comparison
            let parts: Vec<&str> = cond.split('>').collect();
            if parts.len() == 2 {
                if let Some(actual) = variant_parts.get(parts[0]) {
                    let actual_val: i64 = actual.parse().unwrap_or(0);
                    let expected_val: i64 = parts[1].parse().unwrap_or(0);
                    if actual_val <= expected_val {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else if cond.contains('<') {
            // Less than comparison
            let parts: Vec<&str> = cond.split('<').collect();
            if parts.len() == 2 {
                if let Some(actual) = variant_parts.get(parts[0]) {
                    let actual_val: i64 = actual.parse().unwrap_or(0);
                    let expected_val: i64 = parts[1].parse().unwrap_or(0);
                    if actual_val >= expected_val {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            // Simple identifier - check if it's one of the selected values
            // e.g., var_cond="S_166" matches if "S_166" is a value in the variant
            if !variant_values.contains(cond) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_engine_new() {
        let engine = ConfigurationEngine::new("/workspace/ofmldata");
        assert!(engine.oam_cache.is_empty());
        assert!(engine.article_cache.is_empty());
    }

    #[test]
    fn test_load_articles_with_enrichment() {
        let mut engine = ConfigurationEngine::new("/workspace/ofmldata");

        // Test with vitra if available
        let mfr_path = Path::new("/workspace/ofmldata/vitra");
        if !mfr_path.exists() {
            return;
        }

        let articles = engine.load_articles("vitra");
        println!("Loaded {} articles for vitra", articles.len());

        let configurable: Vec<_> = articles.iter().filter(|a| a.is_configurable).collect();
        let non_configurable: Vec<_> = articles.iter().filter(|a| !a.is_configurable).collect();

        println!("  Configurable: {}", configurable.len());
        println!("  Non-configurable: {}", non_configurable.len());

        // Print a few configurable articles
        for article in configurable.iter().take(5) {
            println!(
                "  [C] {} -> {:?}",
                article.article.article_nr,
                article.mapping.as_ref().map(|m| &m.ofml_type)
            );
        }
    }

    #[test]
    fn test_generate_variant_code() {
        let mut properties = PropertyManager::new();
        properties
            .values
            .insert("WIDTH".to_string(), PropertyValue::Int(1200));
        properties.values.insert(
            "COLOR".to_string(),
            PropertyValue::Symbol("white".to_string()),
        );

        let code = generate_variant_code(&properties);
        assert!(code.contains("WIDTH=1200"));
        assert!(code.contains("COLOR=white"));
    }

    #[test]
    fn test_matches_var_cond() {
        let variant = "COLOR=white;WIDTH=1200";
        let values: std::collections::HashSet<&str> = variant
            .split(';')
            .filter_map(|part| part.split('=').nth(1))
            .collect();

        assert!(matches_var_cond("", variant, &values));
        assert!(matches_var_cond("COLOR=white", variant, &values));
        assert!(matches_var_cond("WIDTH=1200", variant, &values));
        assert!(!matches_var_cond("COLOR=black", variant, &values));
        assert!(!matches_var_cond("WIDTH=1800", variant, &values));
    }

    #[test]
    fn test_matches_var_cond_operators() {
        let variant = "WIDTH=1500";
        let values: std::collections::HashSet<&str> = variant
            .split(';')
            .filter_map(|part| part.split('=').nth(1))
            .collect();

        assert!(matches_var_cond("WIDTH>1200", variant, &values));
        assert!(!matches_var_cond("WIDTH>1800", variant, &values));
        assert!(matches_var_cond("WIDTH<1800", variant, &values));
        assert!(!matches_var_cond("WIDTH<1200", variant, &values));
    }

    #[test]
    fn test_sedus_surcharge_matching() {
        // Simulate Sedus AI-121 variant code
        let variant = "S_STOFF=2G3;S_SITZHOEHE=1701;S_ACCESSOIRES=6103;S_LEHNE_ABW=0000";
        let values: std::collections::HashSet<&str> = variant
            .split(';')
            .filter_map(|part| part.split('=').nth(1))
            .collect();
        let map: HashMap<&str, &str> = variant
            .split(';')
            .filter_map(|part| {
                let mut split = part.splitn(2, '=');
                Some((split.next()?, split.next()?))
            })
            .collect();

        // Pattern 4: Numeric code matching (S_1701 -> value starts with 1701)
        assert!(
            matches_var_cond_extended("S_1701", variant, &values, &map),
            "S_1701 should match when S_SITZHOEHE=1701"
        );

        // Pattern 4: Direct numeric match (S_6103 -> value 6103)
        assert!(
            matches_var_cond_extended("S_6103", variant, &values, &map),
            "S_6103 should match when S_ACCESSOIRES=6103"
        );

        // Should NOT match codes not in variant
        assert!(
            !matches_var_cond_extended("S_166", variant, &values, &map),
            "S_166 should NOT match (no 166 value)"
        );
        assert!(
            !matches_var_cond_extended("S_1802", variant, &values, &map),
            "S_1802 should NOT match (no 1802 value)"
        );

        // Base price indicator should not match as surcharge
        assert!(
            !matches_var_cond_extended("S_PGX", variant, &values, &map),
            "S_PGX is a base indicator, not a surcharge"
        );
    }

    // ========== EngineError Tests ==========

    #[test]
    fn test_engine_error_article_not_found() {
        let err = EngineError::ArticleNotFound {
            article: "TEST-001".to_string(),
            manufacturer: "test_mfr".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("TEST-001"));
        assert!(msg.contains("test_mfr"));
    }

    #[test]
    fn test_engine_error_article_not_found_simple() {
        let err = EngineError::ArticleNotFoundSimple("SIMPLE-001".to_string());
        assert!(err.to_string().contains("SIMPLE-001"));
    }

    #[test]
    fn test_engine_error_not_configurable() {
        let err = EngineError::NotConfigurable {
            article: "NC-001".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("NC-001"));
        assert!(msg.contains("not configurable"));
    }

    #[test]
    fn test_engine_error_class_not_found() {
        let err = EngineError::ClassNotFound {
            article: "ART-001".to_string(),
            class: "MyClass".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("ART-001"));
        assert!(msg.contains("MyClass"));
    }

    #[test]
    fn test_engine_error_property_error() {
        let err = EngineError::PropertyError {
            property: "WIDTH".to_string(),
            message: "Invalid value".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("WIDTH"));
        assert!(msg.contains("Invalid value"));
    }

    #[test]
    fn test_engine_error_price_error() {
        let err = EngineError::PriceError {
            article: "ART-001".to_string(),
            message: "No base price".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("ART-001"));
        assert!(msg.contains("No base price"));
    }

    #[test]
    fn test_engine_error_io_error() {
        let err = EngineError::IoError {
            path: "/some/path".to_string(),
            message: "File not found".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("/some/path"));
        assert!(msg.contains("File not found"));
    }

    #[test]
    fn test_engine_error_interpreter_error() {
        let err = EngineError::InterpreterError {
            class: "TestClass".to_string(),
            message: "Parse error".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("TestClass"));
        assert!(msg.contains("Parse error"));
    }

    #[test]
    fn test_engine_error_data_format_error() {
        let err = EngineError::DataFormatError {
            file: "pdata.ebase".to_string(),
            message: "Invalid header".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("pdata.ebase"));
        assert!(msg.contains("Invalid header"));
    }

    #[test]
    fn test_engine_error_missing_data() {
        let err = EngineError::MissingData("Article table".to_string());
        assert!(err.to_string().contains("Article table"));
    }

    // ========== CacheStats Tests ==========

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats {
            oam_entries: 0,
            article_entries: 0,
            family_entries: 0,
            ocd_entries: 0,
        };
        assert_eq!(stats.oam_entries, 0);
        assert_eq!(stats.article_entries, 0);
    }

    #[test]
    fn test_cache_stats_with_values() {
        let stats = CacheStats {
            oam_entries: 5,
            article_entries: 100,
            family_entries: 20,
            ocd_entries: 15,
        };
        assert_eq!(stats.oam_entries, 5);
        assert_eq!(stats.article_entries, 100);
        assert_eq!(stats.family_entries, 20);
        assert_eq!(stats.ocd_entries, 15);
    }

    // ========== EnrichedArticle Tests ==========

    #[test]
    fn test_enriched_article_to_article() {
        let ocd_article = OcdArticle {
            article_nr: "TEST-001".to_string(),
            art_type: "A".to_string(),
            manufacturer: "test_mfr".to_string(),
            series: "test_series".to_string(),
            short_textnr: "1".to_string(),
            long_textnr: "2".to_string(),
        };
        let enriched = EnrichedArticle {
            article: ocd_article,
            description: "Test Description".to_string(),
            is_configurable: true,
            mapping: None,
        };
        let article = enriched.to_article();
        assert_eq!(article.id, "TEST-001");
        assert_eq!(article.manufacturer_id, "test_mfr");
        assert_eq!(article.short_description, "Test Description");
        assert!(article.has_configuration);
    }

    #[test]
    fn test_enriched_article_non_configurable() {
        let ocd_article = OcdArticle {
            article_nr: "NC-001".to_string(),
            art_type: "A".to_string(),
            manufacturer: "mfr".to_string(),
            series: "series".to_string(),
            short_textnr: "".to_string(),
            long_textnr: "".to_string(),
        };
        let enriched = EnrichedArticle {
            article: ocd_article,
            description: "Non-configurable".to_string(),
            is_configurable: false,
            mapping: None,
        };
        assert!(!enriched.is_configurable);
        let article = enriched.to_article();
        assert!(!article.has_configuration);
    }

    // ========== generate_variant_code Tests ==========

    #[test]
    fn test_generate_variant_code_empty() {
        let properties = PropertyManager::new();
        let code = generate_variant_code(&properties);
        assert!(code.is_empty());
    }

    #[test]
    fn test_generate_variant_code_bool_true() {
        let mut properties = PropertyManager::new();
        properties
            .values
            .insert("ENABLED".to_string(), PropertyValue::Bool(true));
        let code = generate_variant_code(&properties);
        assert!(code.contains("ENABLED=1"));
    }

    #[test]
    fn test_generate_variant_code_bool_false() {
        let mut properties = PropertyManager::new();
        properties
            .values
            .insert("ENABLED".to_string(), PropertyValue::Bool(false));
        let code = generate_variant_code(&properties);
        assert!(code.contains("ENABLED=0"));
    }

    #[test]
    fn test_generate_variant_code_float() {
        let mut properties = PropertyManager::new();
        properties
            .values
            .insert("HEIGHT".to_string(), PropertyValue::Float(1.5));
        let code = generate_variant_code(&properties);
        assert!(code.contains("HEIGHT=2")); // Rounded
    }

    #[test]
    fn test_generate_variant_code_string() {
        let mut properties = PropertyManager::new();
        properties.values.insert(
            "MATERIAL".to_string(),
            PropertyValue::String("steel".to_string()),
        );
        let code = generate_variant_code(&properties);
        assert!(code.contains("MATERIAL=steel"));
    }

    #[test]
    fn test_generate_variant_code_sorted() {
        let mut properties = PropertyManager::new();
        properties
            .values
            .insert("ZEBRA".to_string(), PropertyValue::Int(1));
        properties
            .values
            .insert("ALPHA".to_string(), PropertyValue::Int(2));
        let code = generate_variant_code(&properties);
        // Should be sorted alphabetically
        assert!(code.starts_with("ALPHA="));
    }

    // ========== matches_var_cond_extended Tests ==========

    #[test]
    fn test_matches_var_cond_extended_direct_case_insensitive() {
        let variant = "COLOR=PP AXA BLACK";
        let values: std::collections::HashSet<&str> =
            ["PP AXA BLACK"].iter().copied().collect();
        let map: HashMap<&str, &str> = [("COLOR", "PP AXA BLACK")].into_iter().collect();

        // Direct case-insensitive match
        assert!(matches_var_cond_extended(
            "PP AXA BLACK",
            variant,
            &values,
            &map
        ));
        assert!(matches_var_cond_extended(
            "pp axa black",
            variant,
            &values,
            &map
        ));
    }

    #[test]
    fn test_matches_var_cond_extended_price_group() {
        let variant = "PRICEGROUP=PG11";
        let values: std::collections::HashSet<&str> = ["PG11"].iter().copied().collect();
        let map: HashMap<&str, &str> = [("PRICEGROUP", "PG11")].into_iter().collect();

        assert!(matches_var_cond_extended("PG11", variant, &values, &map));
        // GL prefix
        let variant2 = "GLASS=GL1";
        let values2: std::collections::HashSet<&str> = ["GL1"].iter().copied().collect();
        let map2: HashMap<&str, &str> = [("GLASS", "GL1")].into_iter().collect();
        assert!(matches_var_cond_extended("GL1", variant2, &values2, &map2));
    }

    #[test]
    fn test_matches_var_cond_extended_sedus_style_suffix() {
        let variant = "FABRIC=CSE166";
        let values: std::collections::HashSet<&str> = ["CSE166"].iter().copied().collect();
        let map: HashMap<&str, &str> = HashMap::new();

        // S_166 should match value ending with "166"
        assert!(matches_var_cond_extended("S_166", variant, &values, &map));
    }

    #[test]
    fn test_matches_var_cond_extended_embedded_code() {
        let variant = "COMPLEX=XST244166018";
        let values: std::collections::HashSet<&str> =
            ["XST244166018"].iter().copied().collect();
        let map: HashMap<&str, &str> = HashMap::new();

        // S_166 should match value containing "166"
        assert!(matches_var_cond_extended("S_166", variant, &values, &map));
    }

    #[test]
    fn test_matches_var_cond_extended_compound_code() {
        let variant = "S_2415=F2";
        let values: std::collections::HashSet<&str> = ["F2"].iter().copied().collect();
        let map: HashMap<&str, &str> = [("S_2415", "F2")].into_iter().collect();

        // Compound code pattern
        assert!(matches_var_cond_extended(
            "S_2415_F2",
            variant,
            &values,
            &map
        ));
    }

    #[test]
    fn test_matches_var_cond_extended_numeric_prefix() {
        let variant = "OPTION=1801A";
        let values: std::collections::HashSet<&str> = ["1801A"].iter().copied().collect();
        let map: HashMap<&str, &str> = HashMap::new();

        // S_1801 should match value starting with "1801"
        assert!(matches_var_cond_extended("S_1801", variant, &values, &map));
    }

    #[test]
    fn test_matches_var_cond_extended_no_match() {
        let variant = "COLOR=red";
        let values: std::collections::HashSet<&str> = ["red"].iter().copied().collect();
        let map: HashMap<&str, &str> = HashMap::new();

        assert!(!matches_var_cond_extended(
            "S_blue",
            variant,
            &values,
            &map
        ));
        assert!(!matches_var_cond_extended(
            "S_9999",
            variant,
            &values,
            &map
        ));
    }

    // ========== matches_var_cond Tests (additional) ==========

    #[test]
    fn test_matches_var_cond_multiple_conditions() {
        let variant = "COLOR=white;WIDTH=1200;HEIGHT=720";
        let values: std::collections::HashSet<&str> = variant
            .split(';')
            .filter_map(|part| part.split('=').nth(1))
            .collect();

        // All conditions must match
        assert!(matches_var_cond("COLOR=white;WIDTH=1200", variant, &values));
        // Mixed match/no-match
        assert!(!matches_var_cond(
            "COLOR=white;WIDTH=1800",
            variant,
            &values
        ));
    }

    #[test]
    fn test_matches_var_cond_whitespace() {
        let variant = "COLOR=white";
        let values: std::collections::HashSet<&str> = ["white"].iter().copied().collect();

        // Whitespace in conditions should be trimmed
        assert!(matches_var_cond(" COLOR=white ", variant, &values));
    }

    #[test]
    fn test_matches_var_cond_identifier_in_values() {
        let variant = "OPTION=S_166";
        let values: std::collections::HashSet<&str> = ["S_166"].iter().copied().collect();

        // S_166 as a simple identifier should match if it's a value
        assert!(matches_var_cond("S_166", variant, &values));
    }

    // ========== expr_to_int Tests ==========

    #[test]
    fn test_expr_to_int_int() {
        let expr = Expr::Int(42);
        assert_eq!(expr_to_int(&expr), Some(42));
    }

    #[test]
    fn test_expr_to_int_float() {
        let expr = Expr::Float(3.7);
        assert_eq!(expr_to_int(&expr), Some(3));
    }

    #[test]
    fn test_expr_to_int_string() {
        let expr = Expr::String("42".to_string());
        assert_eq!(expr_to_int(&expr), None);
    }

    // ========== parse_property_def_from_expr Tests ==========

    #[test]
    fn test_parse_property_def_from_expr_empty_array() {
        let expr = Expr::Array(vec![]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Unknown");
        assert!(matches!(prop_type, PropertyType::String));
    }

    #[test]
    fn test_parse_property_def_from_expr_with_label() {
        let expr = Expr::Array(vec![Expr::String("Width".to_string())]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Width");
        assert!(matches!(prop_type, PropertyType::String));
    }

    #[test]
    fn test_parse_property_def_from_expr_int_type() {
        let expr = Expr::Array(vec![
            Expr::String("Width".to_string()),
            Expr::Symbol("INT".to_string()),
        ]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Width");
        assert!(matches!(prop_type, PropertyType::Int { min: None, max: None }));
    }

    #[test]
    fn test_parse_property_def_from_expr_float_type() {
        let expr = Expr::Array(vec![
            Expr::String("Height".to_string()),
            Expr::Symbol("FLOAT".to_string()),
        ]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Height");
        assert!(matches!(prop_type, PropertyType::Float { min: None, max: None }));
    }

    #[test]
    fn test_parse_property_def_from_expr_bool_type() {
        let expr = Expr::Array(vec![
            Expr::String("Enabled".to_string()),
            Expr::Symbol("BOOL".to_string()),
        ]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Enabled");
        assert!(matches!(prop_type, PropertyType::Bool));
    }

    #[test]
    fn test_parse_property_def_from_expr_with_range() {
        let expr = Expr::Array(vec![
            Expr::String("Width".to_string()),
            Expr::Symbol("INT".to_string()),
            Expr::Array(vec![Expr::Int(100), Expr::Int(500)]),
        ]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Width");
        assert!(matches!(prop_type, PropertyType::Int { min: Some(100), max: Some(500) }));
    }

    #[test]
    fn test_parse_property_def_from_expr_with_options() {
        // Use 3 options to avoid being interpreted as range [min, max]
        let expr = Expr::Array(vec![
            Expr::String("Color".to_string()),
            Expr::Symbol("CHOICE".to_string()),
            Expr::Array(vec![
                Expr::String("red".to_string()),
                Expr::String("blue".to_string()),
                Expr::String("green".to_string()),
            ]),
        ]);
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Color");
        if let PropertyType::Choice { options } = prop_type {
            assert_eq!(options.len(), 3);
            assert!(options.contains(&"red".to_string()));
            assert!(options.contains(&"blue".to_string()));
            assert!(options.contains(&"green".to_string()));
        } else {
            panic!("Expected Choice type");
        }
    }

    #[test]
    fn test_parse_property_def_from_expr_not_array() {
        let expr = Expr::String("not an array".to_string());
        let (label, prop_type) = parse_property_def_from_expr(&expr);
        assert_eq!(label, "Unknown");
        assert!(matches!(prop_type, PropertyType::String));
    }

    // ========== ConfigurationEngine Tests ==========

    #[test]
    fn test_configuration_engine_data_path() {
        let engine = ConfigurationEngine::new("/test/path");
        assert_eq!(engine.data_path, std::path::PathBuf::from("/test/path"));
    }

    #[test]
    fn test_find_alb_files_nonexistent() {
        let files = find_alb_files(Path::new("/nonexistent"), "series");
        assert!(files.is_empty());
    }

    // ========== LoadedConfiguration Tests ==========

    #[test]
    fn test_loaded_configuration_fields() {
        let ocd_article = OcdArticle {
            article_nr: "ART-001".to_string(),
            art_type: "A".to_string(),
            manufacturer: "mfr".to_string(),
            series: "series".to_string(),
            short_textnr: "".to_string(),
            long_textnr: "".to_string(),
        };
        let enriched = EnrichedArticle {
            article: ocd_article,
            description: "Test".to_string(),
            is_configurable: true,
            mapping: None,
        };
        let config = LoadedConfiguration {
            article: enriched,
            manufacturer_id: "mfr".to_string(),
            properties: PropertyManager::new(),
            price: None,
            package_path: None,
            ofml_class: Some("TestClass".to_string()),
        };
        assert_eq!(config.manufacturer_id, "mfr");
        assert!(config.price.is_none());
        assert_eq!(config.ofml_class.as_deref(), Some("TestClass"));
    }

    // ========== RoundedPrices Tests (implicit through struct usage) ==========

    #[test]
    fn test_rounded_prices_struct() {
        let prices = RoundedPrices {
            base_price: rust_decimal::Decimal::from(100),
            net_price: rust_decimal::Decimal::from(120),
            total_price: rust_decimal::Decimal::from(142),
        };
        assert_eq!(prices.base_price, rust_decimal::Decimal::from(100));
        assert_eq!(prices.net_price, rust_decimal::Decimal::from(120));
        assert_eq!(prices.total_price, rust_decimal::Decimal::from(142));
    }
}
