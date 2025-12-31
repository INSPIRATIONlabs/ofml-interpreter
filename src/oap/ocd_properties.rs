//! OCD Property Reader - Reads property definitions and values from pdata.ebase
//!
//! This module extracts configuration options from OCD tables:
//! - ocd_property: Property definitions
//! - ocd_propertyvalue: Available options for each property
//! - ocd_propertytext: Labels for properties and values
//! - ocd_propertyclass: Property groupings

use std::collections::HashMap;
use std::path::Path;

use crate::ebase::{EBaseReader, Value};

/// A property definition from OCD
#[derive(Debug, Clone)]
pub struct OcdPropertyDef {
    /// Property class (grouping)
    pub prop_class: String,
    /// Property name/key
    pub property: String,
    /// Display order
    pub position: u16,
    /// Text number for label lookup
    pub textnr: String,
    /// Property type (CHOICE, RANGE, INT, etc.)
    pub prop_type: String,
    /// Number of digits
    pub digits: u16,
    /// Decimal digits
    pub dec_digits: u8,
    /// Whether input is required
    pub need_input: bool,
    /// Whether additional values are allowed
    pub add_values: bool,
    /// Whether property can be restricted
    pub restrictable: bool,
    /// Allow multiple selections
    pub multi_option: bool,
}

/// A property value/option from OCD
#[derive(Debug, Clone)]
pub struct OcdPropertyValue {
    /// Property class
    pub prop_class: String,
    /// Property name
    pub property: String,
    /// Display order
    pub position: u16,
    /// Text number for label lookup
    pub textnr: String,
    /// Whether this is the default value
    pub is_default: bool,
    /// Value or range start
    pub value_from: String,
    /// Range end (for range types)
    pub value_to: String,
    /// Operator for value_from comparison
    pub op_from: String,
    /// Operator for value_to comparison
    pub op_to: String,
    /// Step/raster value
    pub raster: String,
}

/// A property class/group from OCD
#[derive(Debug, Clone)]
pub struct OcdPropertyClass {
    /// Class identifier
    pub prop_class: String,
    /// Text number for label lookup
    pub textnr: String,
    /// Display order
    pub position: u16,
}

/// A property text label from OCD
#[derive(Debug, Clone)]
pub struct OcdPropertyText {
    /// Text number (reference key)
    pub textnr: String,
    /// Language code
    pub language: String,
    /// Line number
    pub line_nr: u8,
    /// Text content
    pub text: String,
}

/// OCD Property Reader - loads all property configuration data
pub struct OcdPropertyReader {
    /// Property definitions by (prop_class, property)
    pub properties: HashMap<(String, String), OcdPropertyDef>,
    /// Property values by (prop_class, property)
    pub values: HashMap<(String, String), Vec<OcdPropertyValue>>,
    /// Property classes by prop_class
    pub classes: HashMap<String, OcdPropertyClass>,
    /// Property texts by textnr
    pub texts: HashMap<String, Vec<OcdPropertyText>>,
}

impl OcdPropertyReader {
    /// Load property data from a pdata.ebase file
    pub fn from_ebase(path: &Path) -> Result<Self, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;

        let properties = Self::read_properties(&mut reader)?;
        let values = Self::read_property_values(&mut reader)?;
        let classes = Self::read_property_classes(&mut reader)?;
        let texts = Self::read_property_texts(&mut reader)?;

        Ok(Self {
            properties,
            values,
            classes,
            texts,
        })
    }

    /// Read property definitions from ocd_property table
    fn read_properties(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<(String, String), OcdPropertyDef>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_property") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_property", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let prop_class = get_string(record, "prop_class");
            let property = get_string(record, "property");

            if prop_class.is_empty() || property.is_empty() {
                continue;
            }

            let def = OcdPropertyDef {
                prop_class: prop_class.clone(),
                property: property.clone(),
                position: get_u16(record, "pos_prop"),
                textnr: get_string(record, "prop_textnr"),
                prop_type: get_string(record, "prop_type"),
                digits: get_u16(record, "digits"),
                dec_digits: get_u8(record, "dec_digits"),
                need_input: get_u8(record, "need_input") != 0,
                add_values: get_u8(record, "add_values") != 0,
                restrictable: get_u8(record, "restrictable") != 0,
                multi_option: get_u8(record, "multi_option") != 0,
            };

            result.insert((prop_class, property), def);
        }

        Ok(result)
    }

    /// Read property values from ocd_propertyvalue table
    fn read_property_values(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<(String, String), Vec<OcdPropertyValue>>, String> {
        let mut result: HashMap<(String, String), Vec<OcdPropertyValue>> = HashMap::new();

        if !reader.tables.contains_key("ocd_propertyvalue") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_propertyvalue", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let prop_class = get_string(record, "prop_class");
            let property = get_string(record, "property");

            if prop_class.is_empty() || property.is_empty() {
                continue;
            }

            let value = OcdPropertyValue {
                prop_class: prop_class.clone(),
                property: property.clone(),
                position: get_u16(record, "pos_pval"),
                textnr: get_string(record, "pval_textnr"),
                is_default: get_u8(record, "is_default") != 0,
                value_from: get_string(record, "value_from"),
                value_to: get_string(record, "value_to"),
                op_from: get_string(record, "op_from"),
                op_to: get_string(record, "op_to"),
                raster: get_string(record, "raster"),
            };

            result
                .entry((prop_class, property))
                .or_default()
                .push(value);
        }

        // Sort values by position
        for values in result.values_mut() {
            values.sort_by_key(|v| v.position);
        }

        Ok(result)
    }

    /// Read property classes from ocd_propertyclass table
    fn read_property_classes(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, OcdPropertyClass>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_propertyclass") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_propertyclass", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let prop_class = get_string(record, "prop_class");

            if prop_class.is_empty() {
                continue;
            }

            let class = OcdPropertyClass {
                prop_class: prop_class.clone(),
                textnr: get_string(record, "textnr"),
                position: get_u16(record, "pos_pclass"),
            };

            result.insert(prop_class, class);
        }

        Ok(result)
    }

    /// Read property texts from ocd_propertytext table
    fn read_property_texts(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdPropertyText>>, String> {
        let mut result: HashMap<String, Vec<OcdPropertyText>> = HashMap::new();

        if !reader.tables.contains_key("ocd_propertytext") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_propertytext", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let textnr = get_string(record, "textnr");

            if textnr.is_empty() {
                continue;
            }

            let text = OcdPropertyText {
                textnr: textnr.clone(),
                language: get_string(record, "language"),
                line_nr: get_u8(record, "line_nr"),
                text: get_string(record, "text"),
            };

            result.entry(textnr).or_default().push(text);
        }

        // Sort by line number
        for texts in result.values_mut() {
            texts.sort_by_key(|t| t.line_nr);
        }

        Ok(result)
    }

    /// Get all properties for a property class
    pub fn get_properties_for_class(&self, prop_class: &str) -> Vec<&OcdPropertyDef> {
        let mut props: Vec<_> = self
            .properties
            .iter()
            .filter(|((pc, _), _)| pc == prop_class)
            .map(|(_, def)| def)
            .collect();
        props.sort_by_key(|p| p.position);
        props
    }

    /// Get all values for a property
    pub fn get_values_for_property(
        &self,
        prop_class: &str,
        property: &str,
    ) -> Vec<&OcdPropertyValue> {
        self.values
            .get(&(prop_class.to_string(), property.to_string()))
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get text for a textnr in a specific language
    pub fn get_text(&self, textnr: &str, language: &str) -> Option<String> {
        self.texts.get(textnr).and_then(|texts| {
            // Try exact language match first
            let exact_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language == language)
                .map(|t| t.text.clone())
                .collect();

            if !exact_match.is_empty() {
                return Some(exact_match.join(" "));
            }

            // Try empty language (universal)
            let empty_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language.is_empty())
                .map(|t| t.text.clone())
                .collect();

            if !empty_match.is_empty() {
                return Some(empty_match.join(" "));
            }

            // Fallback priority: EN > other languages
            let en_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language == "EN")
                .map(|t| t.text.clone())
                .collect();

            if !en_match.is_empty() {
                return Some(en_match.join(" "));
            }

            // Last resort: any language
            texts.first().map(|t| t.text.clone())
        })
    }

    /// Get property label
    pub fn get_property_label(
        &self,
        prop_class: &str,
        property: &str,
        language: &str,
    ) -> Option<String> {
        self.properties
            .get(&(prop_class.to_string(), property.to_string()))
            .and_then(|def| self.get_text(&def.textnr, language))
    }

    /// Get property value label
    pub fn get_value_label(&self, value: &OcdPropertyValue, language: &str) -> Option<String> {
        self.get_text(&value.textnr, language)
    }

    /// Get the default value for a property
    pub fn get_default_value(&self, prop_class: &str, property: &str) -> Option<&OcdPropertyValue> {
        self.values
            .get(&(prop_class.to_string(), property.to_string()))
            .and_then(|values| values.iter().find(|v| v.is_default).or_else(|| values.first()))
    }

    /// Get all unique property classes
    pub fn get_property_classes(&self) -> Vec<&str> {
        let mut classes: Vec<_> = self.classes.keys().map(|s| s.as_str()).collect();
        classes.sort();
        classes.dedup();
        classes
    }

    /// Check if this reader has any property data
    pub fn has_properties(&self) -> bool {
        !self.properties.is_empty()
    }

    /// Get statistics about loaded data
    pub fn stats(&self) -> (usize, usize, usize, usize) {
        (
            self.properties.len(),
            self.values.values().map(|v| v.len()).sum(),
            self.classes.len(),
            self.texts.len(),
        )
    }
}

// Helper functions
fn get_string(record: &HashMap<String, Value>, key: &str) -> String {
    record
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn get_u8(record: &HashMap<String, Value>, key: &str) -> u8 {
    record.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as u8
}

fn get_u16(record: &HashMap<String, Value>, key: &str) -> u16 {
    record.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as u16
}

use std::sync::{Arc, OnceLock, Mutex};

/// Cache for aggregated property readers per manufacturer
static PROPERTY_CACHE: OnceLock<Mutex<HashMap<String, Arc<OcdPropertyReader>>>> = OnceLock::new();

fn get_property_cache() -> &'static Mutex<HashMap<String, Arc<OcdPropertyReader>>> {
    PROPERTY_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Find all pdata.ebase files and aggregate property data (with caching)
pub fn load_manufacturer_properties(manufacturer_path: &Path) -> OcdPropertyReader {
    let cache_key = manufacturer_path.to_string_lossy().to_string();

    // Check cache first
    {
        let cache = get_property_cache().lock().unwrap();
        if let Some(reader) = cache.get(&cache_key) {
            return OcdPropertyReader {
                properties: reader.properties.clone(),
                values: reader.values.clone(),
                classes: reader.classes.clone(),
                texts: reader.texts.clone(),
            };
        }
    }

    // Not in cache, load it
    let combined = load_manufacturer_properties_uncached(manufacturer_path);

    // Store in cache
    {
        let mut cache = get_property_cache().lock().unwrap();
        cache.insert(cache_key, Arc::new(OcdPropertyReader {
            properties: combined.properties.clone(),
            values: combined.values.clone(),
            classes: combined.classes.clone(),
            texts: combined.texts.clone(),
        }));
    }

    combined
}

/// Find all pdata.ebase files and aggregate property data (uncached)
fn load_manufacturer_properties_uncached(manufacturer_path: &Path) -> OcdPropertyReader {
    let mut combined = OcdPropertyReader {
        properties: HashMap::new(),
        values: HashMap::new(),
        classes: HashMap::new(),
        texts: HashMap::new(),
    };

    // Find all pdata.ebase files
    for pdata_path in super::ocd::find_pdata_files(manufacturer_path) {
        if let Ok(reader) = OcdPropertyReader::from_ebase(&pdata_path) {
            // Merge data
            combined.properties.extend(reader.properties);
            for (key, values) in reader.values {
                combined.values.entry(key).or_default().extend(values);
            }
            combined.classes.extend(reader.classes);
            for (key, texts) in reader.texts {
                combined.texts.entry(key).or_default().extend(texts);
            }
        }
    }

    // Deduplicate and sort
    for values in combined.values.values_mut() {
        values.sort_by_key(|v| v.position);
        values.dedup_by(|a, b| a.value_from == b.value_from);
    }

    for texts in combined.texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_vitra_classic_properties() {
        let path = Path::new("/workspace/ofmldata/vitra/classic/DE/1/db/pdata.ebase");
        if !path.exists() {
            return;
        }

        let reader = OcdPropertyReader::from_ebase(path).expect("Should load");
        let (props, vals, classes, texts) = reader.stats();

        println!("Properties: {}", props);
        println!("Values: {}", vals);
        println!("Classes: {}", classes);
        println!("Texts: {}", texts);

        assert!(props > 0, "Should have properties");
        assert!(vals > 0, "Should have values");

        // Print some property classes
        println!("\nProperty classes:");
        for class in reader.get_property_classes().iter().take(10) {
            println!("  {}", class);
        }

        // Print some properties with their values
        println!("\nSample properties:");
        for ((pc, prop), def) in reader.properties.iter().take(5) {
            let label = reader.get_property_label(pc, prop, "DE").unwrap_or_default();
            println!("  {}.{} = {} (type: {})", pc, prop, label, def.prop_type);

            let values = reader.get_values_for_property(pc, prop);
            for val in values.iter().take(3) {
                let val_label = reader.get_value_label(val, "DE").unwrap_or_default();
                println!("    - {} = {}", val.value_from, val_label);
            }
        }
    }

    #[test]
    fn test_load_manufacturer_properties() {
        let path = Path::new("/workspace/ofmldata/vitra");
        if !path.exists() {
            return;
        }

        let reader = load_manufacturer_properties(path);
        let (props, vals, classes, texts) = reader.stats();

        println!("Total properties: {}", props);
        println!("Total values: {}", vals);
        println!("Total classes: {}", classes);
        println!("Total texts: {}", texts);

        assert!(props > 100, "Should have many properties across series");
    }
}
