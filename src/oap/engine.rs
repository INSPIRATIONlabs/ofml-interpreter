//! Configuration Engine - Core of the OAP configurator
//!
//! This module provides the ConfigurationEngine that connects:
//! - OAM data (article → CLS class mappings)
//! - OCD data (articles, prices, texts)
//! - CLS interpreter (property extraction, rules)
//! - Price calculation with var_cond matching

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rust_decimal::Decimal;

use crate::alb_loader::AlbLoader;
use crate::ast::{ClassMember, Expr, Stmt};
use crate::parser::Parser;
use crate::property::{PropertyDef, PropertyManager, PropertyType, PropertyValue};

use super::families::{FamilyConfiguration, FamilyLoader, FamilyProperty, ProductFamily};
use super::oam::{load_manufacturer_oam, ArticleMapping, OamData};
use super::ocd::{load_articles_with_descriptions, OcdArticle, OcdPrice, OcdReader};
use super::{Article, PriceResult, Surcharge};

/// Error types for the configuration engine
#[derive(Debug, Clone)]
pub enum EngineError {
    /// Article not found
    ArticleNotFound(String),
    /// No CLS class mapping for article
    NotConfigurable(String),
    /// CLS class not found
    ClassNotFound(String),
    /// Property error
    PropertyError(String),
    /// Price lookup error
    PriceError(String),
    /// IO error
    IoError(String),
    /// Interpreter error
    InterpreterError(String),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::ArticleNotFound(a) => write!(f, "Article not found: {}", a),
            EngineError::NotConfigurable(a) => write!(f, "Article not configurable: {}", a),
            EngineError::ClassNotFound(c) => write!(f, "CLS class not found: {}", c),
            EngineError::PropertyError(e) => write!(f, "Property error: {}", e),
            EngineError::PriceError(e) => write!(f, "Price error: {}", e),
            EngineError::IoError(e) => write!(f, "IO error: {}", e),
            EngineError::InterpreterError(e) => write!(f, "Interpreter error: {}", e),
        }
    }
}

impl std::error::Error for EngineError {}

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

    /// Load product families for a manufacturer (cached)
    pub fn load_families(&mut self, manufacturer_id: &str) -> &[ProductFamily] {
        if !self.family_cache.contains_key(manufacturer_id) {
            let mfr_path = self.data_path.join(manufacturer_id);
            let loader = FamilyLoader::load(&mfr_path, "DE");
            self.family_cache.insert(manufacturer_id.to_string(), loader);
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
    pub fn calculate_family_price(
        &self,
        manufacturer_id: &str,
        family: &ProductFamily,
        config: &FamilyConfiguration,
        price_date: chrono::NaiveDate,
    ) -> Option<PriceResult> {
        let mfr_path = self.data_path.join(manufacturer_id);

        // Find pdata.ebase files (cached)
        let pdata_files = super::ocd::find_pdata_files(&mfr_path);

        for pdata_path in &pdata_files {
            // Use cached OcdReader
            if let Some(reader) = super::ocd::get_ocd_reader(pdata_path) {
                // Get all prices for the base article
                let prices = reader.get_prices(&family.base_article_nr);

                if prices.is_empty() {
                    // Try other articles in the family
                    for art_nr in &family.article_nrs {
                        let prices = reader.get_prices(art_nr);
                        if !prices.is_empty() {
                            return self.match_and_calculate_price(&prices, config, price_date);
                        }
                    }
                    continue;
                }

                return self.match_and_calculate_price(&prices, config, price_date);
            }
        }

        None
    }

    /// Match prices and calculate total
    fn match_and_calculate_price(
        &self,
        prices: &[&OcdPrice],
        config: &FamilyConfiguration,
        price_date: chrono::NaiveDate,
    ) -> Option<PriceResult> {
        let matched = match_prices_to_variant(prices, &config.variant_code)?;

        let valid_from =
            chrono::NaiveDate::parse_from_str(&matched.base_price.date_from, "%Y%m%d")
                .unwrap_or(price_date);
        let valid_to =
            chrono::NaiveDate::parse_from_str(&matched.base_price.date_to, "%Y%m%d").ok();

        Some(PriceResult::new(
            matched.base_amount,
            matched.surcharges,
            matched.base_price.currency.clone(),
            price_date,
            valid_from,
            valid_to,
        ))
    }

    /// Load OAM data for a manufacturer (cached)
    pub fn load_oam(&mut self, manufacturer_id: &str) -> &OamData {
        if !self.oam_cache.contains_key(manufacturer_id) {
            let mfr_path = self.data_path.join(manufacturer_id);
            let oam_data = load_manufacturer_oam(&mfr_path);
            self.oam_cache.insert(manufacturer_id.to_string(), oam_data);
        }
        self.oam_cache.get(manufacturer_id).unwrap()
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
            .ok_or_else(|| EngineError::ArticleNotFound(article_nr.to_string()))?;

        // Check if configurable
        let mapping = article
            .mapping
            .clone()
            .ok_or_else(|| EngineError::NotConfigurable(article_nr.to_string()))?;

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
                    if cls_source.classes.iter().any(|c| {
                        c == &short_class_name || c == class_name
                    }) {
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
        for (_, sources) in &self.alb_loader.sources {
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
                let matched = match_prices_to_variant(&prices, &variant_code);

                if let Some(price_result) = matched {
                    // Parse dates and create PriceResult
                    let valid_from = chrono::NaiveDate::parse_from_str(
                        &price_result.base_price.date_from,
                        "%Y%m%d",
                    )
                    .unwrap_or(price_date);
                    let valid_to =
                        chrono::NaiveDate::parse_from_str(&price_result.base_price.date_to, "%Y%m%d")
                            .ok();

                    return Ok(Some(PriceResult::new(
                        price_result.base_amount,
                        price_result.surcharges,
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
        config
            .properties
            .set(key, value)
            .map_err(|e| EngineError::PropertyError(e.to_string()))?;

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
            } else if entry_path.extension().map_or(false, |e| e == "alb") {
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
    let mut parser = Parser::new(source)
        .map_err(|e| EngineError::InterpreterError(e.to_string()))?;
    let ast = parser
        .parse()
        .map_err(|e| EngineError::InterpreterError(e.to_string()))?;

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
                            prop_type = PropertyType::Int { min: None, max: None };
                        }
                        "FLOAT" | "REAL" => {
                            prop_type = PropertyType::Float { min: None, max: None };
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
                        if let (Some(min), Some(max)) = (expr_to_int(&inner[0]), expr_to_int(&inner[1])) {
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
            prop_type = PropertyType::Int { min: min_val, max: max_val };
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

/// Matched price result with base and surcharges
struct MatchedPrice<'a> {
    base_price: &'a OcdPrice,
    base_amount: Decimal,
    surcharges: Vec<Surcharge>,
}

/// Match prices to variant code
fn match_prices_to_variant<'a>(
    prices: &'a [&'a OcdPrice],
    variant_code: &str,
) -> Option<MatchedPrice<'a>> {
    // Known base price indicators (manufacturer-specific patterns)
    let base_indicators = ["S_PGX", "BASE", "STANDARD", ""];

    // Find base price: empty var_cond, or known base indicators
    let base_price = prices
        .iter()
        .find(|p| base_indicators.iter().any(|ind| p.var_cond == *ind || p.var_cond.is_empty()))
        .or_else(|| prices.first())?;

    let base_amount = Decimal::from_f32_retain(base_price.price).unwrap_or(Decimal::ZERO);

    // Extract values from variant code for surcharge matching
    // Also build a map of key=value pairs
    let variant_values: std::collections::HashSet<&str> = variant_code
        .split(';')
        .filter_map(|part| part.split('=').nth(1))
        .collect();

    // Build variant map for heuristic matching
    let variant_map: HashMap<&str, &str> = variant_code
        .split(';')
        .filter_map(|part| {
            let mut split = part.splitn(2, '=');
            Some((split.next()?, split.next()?))
        })
        .collect();

    // Find matching surcharges
    let mut surcharges = Vec::new();
    let mut seen_var_conds = std::collections::HashSet::new();

    for price in prices {
        // Skip base price and already processed var_conds
        if price.var_cond.is_empty()
            || base_indicators.contains(&price.var_cond.as_str())
            || seen_var_conds.contains(&price.var_cond)
        {
            continue;
        }

        // Check if var_cond matches current variant using multiple strategies
        if matches_var_cond_extended(&price.var_cond, variant_code, &variant_values, &variant_map) {
            let amount = Decimal::from_f32_retain(price.price).unwrap_or(Decimal::ZERO);
            surcharges.push(Surcharge {
                name: price.var_cond.clone(),
                amount,
                is_percentage: false,
            });
            seen_var_conds.insert(price.var_cond.clone());
        }
    }

    Some(MatchedPrice {
        base_price,
        base_amount,
        surcharges,
    })
}

/// Extended matching for OCD surcharge codes
/// Supports multiple matching strategies for manufacturer-specific formats
fn matches_var_cond_extended(
    var_cond: &str,
    variant_code: &str,
    variant_values: &std::collections::HashSet<&str>,
    variant_map: &HashMap<&str, &str>,
) -> bool {
    // Strategy 1: Direct formula matching (KEY=value, KEY>value, etc.)
    if matches_var_cond(var_cond, variant_code, variant_values) {
        return true;
    }

    // Strategy 2: Sedus-style surcharge codes (S_XXXX format)
    // Check if var_cond matches a pattern like S_<property_suffix>_<value_suffix>
    // Example: S_166 might match when S_MODELLFARBE=166 or similar
    if var_cond.starts_with("S_") {
        let code = &var_cond[2..]; // Remove "S_" prefix

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

        // Pattern 3: Split code by underscore for compound codes (e.g., S_2415_F2)
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

        // Pattern 4: Numeric code as property value
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
        properties.values.insert("WIDTH".to_string(), PropertyValue::Int(1200));
        properties.values.insert("COLOR".to_string(), PropertyValue::Symbol("white".to_string()));

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
        assert!(matches_var_cond_extended("S_1701", variant, &values, &map),
            "S_1701 should match when S_SITZHOEHE=1701");

        // Pattern 4: Direct numeric match (S_6103 -> value 6103)
        assert!(matches_var_cond_extended("S_6103", variant, &values, &map),
            "S_6103 should match when S_ACCESSOIRES=6103");

        // Should NOT match codes not in variant
        assert!(!matches_var_cond_extended("S_166", variant, &values, &map),
            "S_166 should NOT match (no 166 value)");
        assert!(!matches_var_cond_extended("S_1802", variant, &values, &map),
            "S_1802 should NOT match (no 1802 value)");

        // Base price indicator should not match as surcharge
        assert!(!matches_var_cond_extended("S_PGX", variant, &values, &map),
            "S_PGX is a base indicator, not a surcharge");
    }
}
