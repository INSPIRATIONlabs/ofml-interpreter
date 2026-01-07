//! OCD Property Reader - Reads property definitions and values from pdata.ebase
//!
//! This module extracts configuration options from OCD tables:
//! - ocd_property: Property definitions
//! - ocd_propertyvalue: Available options for each property
//! - ocd_propertytext: Labels for properties and values
//! - ocd_propertyclass: Property groupings

use std::collections::HashMap;
use std::path::Path;

use rayon::prelude::*;

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
    /// Relation object ID (for TABLE relations)
    pub rel_obj: u32,
    /// Property scope (C=Choice, R=Result, RG=Range, etc.)
    pub scope: String,
    /// Source series/family this property was loaded from (e.g., "kr", "wkm")
    /// Used to filter properties relevant to a specific family
    pub source_series: Option<String>,
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

/// A relation object definition from ocd_relationobj
#[derive(Debug, Clone)]
pub struct OcdRelationObj {
    /// Relation object ID
    pub rel_obj: u32,
    /// Relation name (links to ocd_relation)
    pub rel_name: String,
    /// Relation type (TABLE, EXPR, etc.)
    pub rel_type: String,
    /// Position
    pub position: u16,
}

/// A relation definition from ocd_relation
#[derive(Debug, Clone)]
pub struct OcdRelation {
    /// Relation name (links from ocd_relationobj)
    pub rel_name: String,
    /// Block number within relation
    pub rel_blocknr: u16,
    /// Relation expression text
    pub rel_block: String,
}

/// Parsed TABLE relation for property value lookup
#[derive(Debug, Clone)]
pub struct TableRelation {
    /// Custom table name (e.g., "wkt_groesse_tbl")
    pub table_name: String,
    /// Column mappings: (table_column, property_or_self)
    /// e.g., [("Abmessung", "Abmessung"), ("Groesse", "$SELF.Groesse")]
    pub column_mappings: Vec<(String, String)>,
    /// The target column that provides values for this property
    pub target_column: Option<String>,
}

/// A property group from ocd_propertygroup
#[derive(Debug, Clone)]
pub struct OcdPropertyGroup {
    /// Property class this group belongs to
    pub prop_class: String,
    /// Property name
    pub property: String,
    /// Group identifier
    pub prop_group: String,
    /// Display order within the group
    pub position: u16,
}

/// Article to property group mapping from ocd_article2propgroup
#[derive(Debug, Clone)]
pub struct OcdArticle2PropGroup {
    /// Article number
    pub article_nr: String,
    /// Property group identifier
    pub prop_group: String,
    /// Display order
    pub position: u16,
    /// Text number for label lookup
    pub textnr: String,
}

/// Rounding rule from ocd_rounding
#[derive(Debug, Clone)]
pub struct OcdRounding {
    /// Rounding rule identifier
    pub id: String,
    /// Rule number/sequence
    pub nr: u16,
    /// Rounding type (e.g., "ROUND", "FLOOR", "CEIL")
    pub rounding_type: String,
    /// Precision (decimal places)
    pub precision: u8,
    /// Minimum value for this rule to apply
    pub min: f64,
    /// Maximum value for this rule to apply
    pub max: f64,
    /// Amount to add before rounding
    pub add_before: f64,
    /// Amount to add after rounding
    pub add_after: f64,
}

/// Tax scheme from ocd_taxscheme
#[derive(Debug, Clone)]
pub struct OcdTaxScheme {
    /// Tax identifier
    pub tax_id: String,
    /// Country code
    pub country: String,
    /// Region code
    pub region: String,
    /// Tax category
    pub tax_category: String,
    /// Tax type
    pub tax_type: String,
    /// Tax percentage/number
    pub number: f64,
}

/// Article taxes from ocd_articletaxes
#[derive(Debug, Clone)]
pub struct OcdArticleTaxes {
    /// Article number
    pub article_nr: String,
    /// Tax identifier (links to ocd_taxscheme)
    pub tax_id: String,
    /// Valid from date (YYYYMMDD)
    pub date_from: String,
    /// Valid to date (YYYYMMDD)
    pub date_to: String,
}

/// Code scheme from ocd_codescheme
#[derive(Debug, Clone)]
pub struct OcdCodeScheme {
    /// Scheme identifier
    pub scheme_id: String,
    /// Scheme pattern/format
    pub scheme: String,
    /// Variant code separator
    pub varcode_sep: String,
    /// Value separator
    pub value_sep: String,
    /// Multi-option separator
    pub mo_sep: String,
    /// Multi-option bracket
    pub mo_bracket: String,
    /// Character for invisible options
    pub invisible_char: String,
    /// Character for unselected options
    pub unselect_char: String,
    /// Visibility flag
    pub visibility: String,
    /// Trim whitespace
    pub trim: bool,
}

/// Article base properties from ocd_artbase
#[derive(Debug, Clone)]
pub struct OcdArtBase {
    /// Article number
    pub article_nr: String,
    /// Property class
    pub prop_class: String,
    /// Property name
    pub property: String,
    /// Property value
    pub prop_value: String,
}

/// Packaging information from ocd_packaging
#[derive(Debug, Clone)]
pub struct OcdPackaging {
    /// Article number
    pub article_nr: String,
    /// Variant condition
    pub var_cond: String,
    /// Width
    pub width: f64,
    /// Depth
    pub depth: f64,
    /// Height
    pub height: f64,
    /// Net weight
    pub net_weight: f64,
    /// Tare weight
    pub tara_weight: f64,
    /// Volume
    pub volume: f64,
    /// Items per unit
    pub items_per_unit: u32,
    /// Pack units
    pub pack_units: u32,
    /// Measure unit (mm, cm, m)
    pub measure_unit: String,
    /// Weight unit (g, kg)
    pub weight_unit: String,
    /// Volume unit (l, m³)
    pub volume_unit: String,
}

/// Bill of items entry from ocd_billofitems
#[derive(Debug, Clone)]
pub struct OcdBillOfItems {
    /// Composite product ID
    pub composite_id: String,
    /// Item ID (article number or sub-composite)
    pub item_id: String,
    /// Position within the bill
    pub item_pos: u16,
    /// Quantity
    pub quantity: f64,
    /// Quantity unit
    pub quant_unit: String,
    /// Whether item is configurable
    pub configurable: bool,
    /// Relation object for configuration
    pub rel_obj: u32,
    /// Text ID for description
    pub txt_id: String,
}

/// Composite product from ocd_composite
#[derive(Debug, Clone)]
pub struct OcdComposite {
    /// Composite product ID
    pub composite_id: String,
    /// Basket mode (how to add to basket)
    pub basket_mode: String,
    /// Price mode (how to calculate price)
    pub price_mode: String,
    /// Text mode (how to display text)
    pub text_mode: String,
    /// Whether composite is configurable
    pub configurable: bool,
    /// Whether items are configurable
    pub items_configurable: bool,
    /// Whether composite is fixed
    pub is_fixed: bool,
}

/// Version information from ocd_version
#[derive(Debug, Clone)]
pub struct OcdVersion {
    /// Data version
    pub data_version: String,
    /// Format version
    pub format_version: String,
    /// Region code
    pub region: String,
    /// Valid from date
    pub date_from: String,
    /// Valid to date
    pub date_to: String,
    /// Relation coding type
    pub rel_coding: String,
    /// Comment
    pub comment: String,
    /// Available tables
    pub tables: String,
    /// Variant condition variable
    pub varcond_var: String,
    /// Placeholder enabled
    pub placeholder_on: bool,
}

/// OCD Property Reader - loads all property configuration data
pub struct OcdPropertyReader {
    /// Property definitions by (prop_class, property)
    pub properties: HashMap<(String, String), OcdPropertyDef>,
    /// Property values by (prop_class, property)
    pub values: HashMap<(String, String), Vec<OcdPropertyValue>>,
    /// Property classes by prop_class
    pub classes: HashMap<String, OcdPropertyClass>,
    /// Property texts by textnr (for property labels)
    pub texts: HashMap<String, Vec<OcdPropertyText>>,
    /// Property VALUE texts by textnr (for value descriptions like "6235" -> "Armlehne")
    pub value_texts: HashMap<String, Vec<OcdPropertyText>>,
    /// Price texts by textnr (for surcharge descriptions)
    pub price_texts: HashMap<String, Vec<OcdPropertyText>>,
    /// Property class texts by textnr (for property class/group labels)
    pub prop_class_texts: HashMap<String, Vec<OcdPropertyText>>,
    /// Hint texts by textnr (for help/tooltip texts)
    pub hint_texts: HashMap<String, Vec<OcdPropertyText>>,
    /// User messages by textnr
    pub user_messages: HashMap<String, Vec<OcdPropertyText>>,
    /// Property group texts by textnr
    pub prop_group_texts: HashMap<String, Vec<OcdPropertyText>>,
    /// Relation objects by rel_obj ID
    pub relation_objs: HashMap<u32, OcdRelationObj>,
    /// Relations by rel_name (each rel_name can have multiple blocks)
    pub relations: HashMap<String, Vec<OcdRelation>>,
    /// Custom table data: table_name -> records (each record is field->value)
    pub custom_tables: HashMap<String, Vec<HashMap<String, String>>>,
    /// Property groups by (prop_class, property)
    pub property_groups: HashMap<(String, String), OcdPropertyGroup>,
    /// Article to property group mappings by article_nr
    pub article_prop_groups: HashMap<String, Vec<OcdArticle2PropGroup>>,
    /// Rounding rules by id
    pub rounding_rules: HashMap<String, Vec<OcdRounding>>,
    /// Tax schemes by tax_id
    pub tax_schemes: HashMap<String, OcdTaxScheme>,
    /// Article taxes by article_nr
    pub article_taxes: HashMap<String, Vec<OcdArticleTaxes>>,
    /// Code schemes by scheme_id
    pub code_schemes: HashMap<String, OcdCodeScheme>,
    /// Article base properties by article_nr
    pub art_base: HashMap<String, Vec<OcdArtBase>>,
    /// Packaging info by article_nr
    pub packaging: HashMap<String, Vec<OcdPackaging>>,
    /// Bill of items by composite_id
    pub bill_of_items: HashMap<String, Vec<OcdBillOfItems>>,
    /// Composite products by composite_id
    pub composites: HashMap<String, OcdComposite>,
    /// Version information (single record)
    pub version: Option<OcdVersion>,
}

impl OcdPropertyReader {
    /// Load property data from a pdata.ebase file
    pub fn from_ebase(path: &Path) -> Result<Self, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;

        // Core property data
        let properties = Self::read_properties(&mut reader)?;
        let values = Self::read_property_values(&mut reader)?;
        let classes = Self::read_property_classes(&mut reader)?;

        // Text tables
        let texts = Self::read_property_texts(&mut reader, "ocd_propertytext")?;
        let value_texts = Self::read_property_texts(&mut reader, "ocd_propvaluetext")?;
        let price_texts = Self::read_property_texts(&mut reader, "ocd_pricetext")?;
        let prop_class_texts = Self::read_property_texts(&mut reader, "ocd_propclasstext")?;
        let hint_texts = Self::read_property_texts(&mut reader, "ocd_prophinttext")?;
        let user_messages = Self::read_property_texts(&mut reader, "ocd_usermessage")?;
        let prop_group_texts = Self::read_property_texts(&mut reader, "ocd_propgrouptext")?;

        // Relations
        let relation_objs = Self::read_relation_objs(&mut reader)?;
        let relations = Self::read_relations(&mut reader)?;

        // Property groups
        let property_groups = Self::read_property_groups(&mut reader)?;
        let article_prop_groups = Self::read_article_prop_groups(&mut reader)?;

        // Pricing support tables
        let rounding_rules = Self::read_rounding_rules(&mut reader)?;
        let tax_schemes = Self::read_tax_schemes(&mut reader)?;
        let article_taxes = Self::read_article_taxes(&mut reader)?;

        // Other tables
        let code_schemes = Self::read_code_schemes(&mut reader)?;
        let art_base = Self::read_art_base(&mut reader)?;
        let packaging = Self::read_packaging(&mut reader)?;
        let bill_of_items = Self::read_bill_of_items(&mut reader)?;
        let composites = Self::read_composites(&mut reader)?;
        let version = Self::read_version(&mut reader)?;

        // Find which custom tables are referenced by TABLE relations
        let mut custom_tables = HashMap::new();
        for rel_list in relations.values() {
            for rel in rel_list {
                if let Some(parsed) = Self::parse_table_relation(&rel.rel_block) {
                    let table_name_lower = parsed.table_name.to_lowercase() + "_tbl";
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        custom_tables.entry(table_name_lower.clone())
                    {
                        if let Ok(table_data) =
                            Self::read_custom_table(&mut reader, &table_name_lower)
                        {
                            e.insert(table_data);
                        }
                    }
                }
            }
        }

        Ok(Self {
            properties,
            values,
            classes,
            texts,
            value_texts,
            price_texts,
            prop_class_texts,
            hint_texts,
            user_messages,
            prop_group_texts,
            relation_objs,
            relations,
            custom_tables,
            property_groups,
            article_prop_groups,
            rounding_rules,
            tax_schemes,
            article_taxes,
            code_schemes,
            art_base,
            packaging,
            bill_of_items,
            composites,
            version,
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
                rel_obj: get_u32(record, "rel_obj"),
                scope: get_string(record, "scope"),
                source_series: None, // Set during manufacturer loading
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

    /// Read property texts from a text table (ocd_propertytext or ocd_propvaluetext)
    fn read_property_texts(
        reader: &mut EBaseReader,
        table_name: &str,
    ) -> Result<HashMap<String, Vec<OcdPropertyText>>, String> {
        let mut result: HashMap<String, Vec<OcdPropertyText>> = HashMap::new();

        if !reader.tables.contains_key(table_name) {
            return Ok(result);
        }

        let records = reader
            .read_records(table_name, None)
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

    /// Read relation objects from ocd_relationobj table
    fn read_relation_objs(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<u32, OcdRelationObj>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_relationobj") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_relationobj", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let rel_obj = get_u32(record, "rel_obj");
            if rel_obj == 0 {
                continue;
            }

            let obj = OcdRelationObj {
                rel_obj,
                rel_name: get_string(record, "rel_name"),
                rel_type: get_string(record, "rel_type"),
                position: get_u16(record, "position"),
            };

            result.insert(rel_obj, obj);
        }

        Ok(result)
    }

    /// Read relations from ocd_relation table
    fn read_relations(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdRelation>>, String> {
        let mut result: HashMap<String, Vec<OcdRelation>> = HashMap::new();

        if !reader.tables.contains_key("ocd_relation") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_relation", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let rel_name = get_string(record, "rel_name");
            if rel_name.is_empty() {
                continue;
            }

            let rel = OcdRelation {
                rel_name: rel_name.clone(),
                rel_blocknr: get_u16(record, "rel_blocknr"),
                rel_block: get_string(record, "rel_block"),
            };

            result.entry(rel_name).or_default().push(rel);
        }

        // Sort by block number
        for relations in result.values_mut() {
            relations.sort_by_key(|r| r.rel_blocknr);
        }

        Ok(result)
    }

    /// Parse a TABLE relation expression
    /// Format: TABLE TableName (col1=prop1, col2=$SELF.prop2)
    pub fn parse_table_relation(rel_text: &str) -> Option<TableRelation> {
        let text = rel_text.trim();
        if !text.to_uppercase().starts_with("TABLE ") {
            return None;
        }

        // Remove "TABLE " prefix
        let rest = text[6..].trim();

        // Find table name (until '(' or whitespace)
        let table_end = rest.find(|c: char| c == '(' || c.is_whitespace())?;
        let table_name = rest[..table_end].trim().to_string();

        // Find the parentheses content
        let paren_start = rest.find('(')?;
        let paren_end = rest.rfind(')')?;
        if paren_end <= paren_start {
            return None;
        }

        let mapping_text = &rest[paren_start + 1..paren_end];

        // Parse column mappings: col1=val1, col2=val2, ...
        let mut column_mappings = Vec::new();
        let mut target_column = None;

        for part in mapping_text.split(',') {
            let part = part.trim();
            if let Some(eq_pos) = part.find('=') {
                let col = part[..eq_pos].trim().to_string();
                let val = part[eq_pos + 1..].trim().to_string();

                // If value contains $SELF, this is the target column
                if val.contains("$SELF") {
                    // Extract the property name from $SELF.property
                    if val.find('.').is_some() {
                        target_column = Some(col.clone());
                    }
                }

                column_mappings.push((col, val));
            }
        }

        Some(TableRelation {
            table_name,
            column_mappings,
            target_column,
        })
    }

    /// Read a custom table dynamically
    /// Some tables use a pivot format with columns: line, name, value
    /// This function detects and handles both formats
    fn read_custom_table(
        reader: &mut EBaseReader,
        table_name: &str,
    ) -> Result<Vec<HashMap<String, String>>, String> {
        if !reader.tables.contains_key(table_name) {
            return Ok(Vec::new());
        }

        let records = reader
            .read_records(table_name, None)
            .map_err(|e| e.to_string())?;

        if records.is_empty() {
            return Ok(Vec::new());
        }

        // Check if this is a pivot table (has line, name, value columns)
        let first_record = &records[0];
        let has_line = first_record.contains_key("line");
        let has_name = first_record.contains_key("name");
        let has_value = first_record.contains_key("value");

        if has_line && has_name && has_value {
            // This is a pivot table - unpivot it
            Self::unpivot_table(&records)
        } else {
            // Normal table format
            let mut result = Vec::new();
            for record in &records {
                let mut row = HashMap::new();
                for (key, value) in record {
                    let str_value = match value {
                        Value::String(s) => s.clone(),
                        Value::Int(i) => i.to_string(),
                        Value::UInt(u) => u.to_string(),
                        Value::Float(f) => f.to_string(),
                        Value::Blob(_) | Value::Null => String::new(),
                    };
                    row.insert(key.clone(), str_value);
                }
                result.push(row);
            }
            Ok(result)
        }
    }

    /// Unpivot a table with line/name/value columns into proper rows
    fn unpivot_table(
        records: &[HashMap<String, Value>],
    ) -> Result<Vec<HashMap<String, String>>, String> {
        // Group records by line number
        let mut lines: HashMap<String, HashMap<String, String>> = HashMap::new();

        for record in records {
            let line = match record.get("line") {
                Some(Value::String(s)) => s.clone(),
                Some(Value::Int(i)) => i.to_string(),
                Some(Value::UInt(u)) => u.to_string(),
                _ => continue,
            };

            let name = match record.get("name") {
                Some(Value::String(s)) => s.to_lowercase(),
                _ => continue,
            };

            let value = match record.get("value") {
                Some(Value::String(s)) => s.clone(),
                Some(Value::Int(i)) => i.to_string(),
                Some(Value::UInt(u)) => u.to_string(),
                Some(Value::Float(f)) => f.to_string(),
                _ => String::new(),
            };

            lines.entry(line).or_default().insert(name, value);
        }

        // Convert to vector of rows
        let mut result: Vec<HashMap<String, String>> = lines.into_values().collect();

        // Sort by line number if present (for deterministic ordering)
        result.sort_by(|a, b| {
            let a_line = a
                .get("line")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            let b_line = b
                .get("line")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            a_line.cmp(&b_line)
        });

        Ok(result)
    }

    /// Read property groups from ocd_propertygroup table
    fn read_property_groups(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<(String, String), OcdPropertyGroup>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_propertygroup") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_propertygroup", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let prop_class = get_string(record, "prop_class");
            let property = get_string(record, "property");

            if prop_class.is_empty() || property.is_empty() {
                continue;
            }

            let group = OcdPropertyGroup {
                prop_class: prop_class.clone(),
                property: property.clone(),
                prop_group: get_string(record, "prop_group"),
                position: get_u16(record, "pos_prop"),
            };

            result.insert((prop_class, property), group);
        }

        Ok(result)
    }

    /// Read article to property group mappings from ocd_article2propgroup table
    fn read_article_prop_groups(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdArticle2PropGroup>>, String> {
        let mut result: HashMap<String, Vec<OcdArticle2PropGroup>> = HashMap::new();

        if !reader.tables.contains_key("ocd_article2propgroup") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_article2propgroup", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let article_nr = get_string(record, "article_nr");

            if article_nr.is_empty() {
                continue;
            }

            let mapping = OcdArticle2PropGroup {
                article_nr: article_nr.clone(),
                prop_group: get_string(record, "prop_group"),
                position: get_u16(record, "pos_group"),
                textnr: get_string(record, "textnr"),
            };

            result.entry(article_nr).or_default().push(mapping);
        }

        // Sort by position
        for mappings in result.values_mut() {
            mappings.sort_by_key(|m| m.position);
        }

        Ok(result)
    }

    /// Read rounding rules from ocd_rounding table
    fn read_rounding_rules(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdRounding>>, String> {
        let mut result: HashMap<String, Vec<OcdRounding>> = HashMap::new();

        if !reader.tables.contains_key("ocd_rounding") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_rounding", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let id = get_string(record, "id");

            if id.is_empty() {
                continue;
            }

            let rule = OcdRounding {
                id: id.clone(),
                nr: get_u16(record, "nr"),
                rounding_type: get_string(record, "type"),
                precision: get_u8(record, "precision"),
                min: get_f64(record, "min"),
                max: get_f64(record, "max"),
                add_before: get_f64(record, "add_before"),
                add_after: get_f64(record, "add_after"),
            };

            result.entry(id).or_default().push(rule);
        }

        // Sort by nr
        for rules in result.values_mut() {
            rules.sort_by_key(|r| r.nr);
        }

        Ok(result)
    }

    /// Read tax schemes from ocd_taxscheme table
    fn read_tax_schemes(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, OcdTaxScheme>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_taxscheme") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_taxscheme", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let tax_id = get_string(record, "tax_id");

            if tax_id.is_empty() {
                continue;
            }

            let scheme = OcdTaxScheme {
                tax_id: tax_id.clone(),
                country: get_string(record, "country"),
                region: get_string(record, "region"),
                tax_category: get_string(record, "tax_category"),
                tax_type: get_string(record, "tax_type"),
                number: get_f64(record, "number"),
            };

            result.insert(tax_id, scheme);
        }

        Ok(result)
    }

    /// Read article taxes from ocd_articletaxes table
    fn read_article_taxes(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdArticleTaxes>>, String> {
        let mut result: HashMap<String, Vec<OcdArticleTaxes>> = HashMap::new();

        if !reader.tables.contains_key("ocd_articletaxes") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_articletaxes", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let article_nr = get_string(record, "article_nr");

            if article_nr.is_empty() {
                continue;
            }

            let tax = OcdArticleTaxes {
                article_nr: article_nr.clone(),
                tax_id: get_string(record, "tax_id"),
                date_from: get_string(record, "date_from"),
                date_to: get_string(record, "date_to"),
            };

            result.entry(article_nr).or_default().push(tax);
        }

        Ok(result)
    }

    /// Read code schemes from ocd_codescheme table
    fn read_code_schemes(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, OcdCodeScheme>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_codescheme") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_codescheme", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let scheme_id = get_string(record, "scheme_id");

            if scheme_id.is_empty() {
                continue;
            }

            let scheme = OcdCodeScheme {
                scheme_id: scheme_id.clone(),
                scheme: get_string(record, "scheme"),
                varcode_sep: get_string(record, "varcode_sep"),
                value_sep: get_string(record, "value_sep"),
                mo_sep: get_string(record, "mo_sep"),
                mo_bracket: get_string(record, "mo_bracket"),
                invisible_char: get_string(record, "invisible_char"),
                unselect_char: get_string(record, "unselect_char"),
                visibility: get_string(record, "visibility"),
                trim: get_u8(record, "trim") != 0,
            };

            result.insert(scheme_id, scheme);
        }

        Ok(result)
    }

    /// Read article base properties from ocd_artbase table
    fn read_art_base(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdArtBase>>, String> {
        let mut result: HashMap<String, Vec<OcdArtBase>> = HashMap::new();

        if !reader.tables.contains_key("ocd_artbase") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_artbase", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let article_nr = get_string(record, "article_nr");

            if article_nr.is_empty() {
                continue;
            }

            let base = OcdArtBase {
                article_nr: article_nr.clone(),
                prop_class: get_string(record, "prop_class"),
                property: get_string(record, "property"),
                prop_value: get_string(record, "prop_value"),
            };

            result.entry(article_nr).or_default().push(base);
        }

        Ok(result)
    }

    /// Read packaging info from ocd_packaging table
    fn read_packaging(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdPackaging>>, String> {
        let mut result: HashMap<String, Vec<OcdPackaging>> = HashMap::new();

        if !reader.tables.contains_key("ocd_packaging") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_packaging", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let article_nr = get_string(record, "article_nr");

            if article_nr.is_empty() {
                continue;
            }

            let pkg = OcdPackaging {
                article_nr: article_nr.clone(),
                var_cond: get_string(record, "var_cond"),
                width: get_f64(record, "width"),
                depth: get_f64(record, "depth"),
                height: get_f64(record, "height"),
                net_weight: get_f64(record, "net_weight"),
                tara_weight: get_f64(record, "tara_weight"),
                volume: get_f64(record, "volume"),
                items_per_unit: get_u32(record, "items_per_unit"),
                pack_units: get_u32(record, "pack_units"),
                measure_unit: get_string(record, "measure_unit"),
                weight_unit: get_string(record, "weight_unit"),
                volume_unit: get_string(record, "volume_unit"),
            };

            result.entry(article_nr).or_default().push(pkg);
        }

        Ok(result)
    }

    /// Read bill of items from ocd_billofitems table
    fn read_bill_of_items(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, Vec<OcdBillOfItems>>, String> {
        let mut result: HashMap<String, Vec<OcdBillOfItems>> = HashMap::new();

        if !reader.tables.contains_key("ocd_billofitems") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_billofitems", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let composite_id = get_string(record, "composite_id");

            if composite_id.is_empty() {
                continue;
            }

            let item = OcdBillOfItems {
                composite_id: composite_id.clone(),
                item_id: get_string(record, "item_id"),
                item_pos: get_u16(record, "item_pos"),
                quantity: get_f64(record, "quanity"), // Note: typo in original schema
                quant_unit: get_string(record, "quant_unit"),
                configurable: get_u8(record, "configurable") != 0,
                rel_obj: get_u32(record, "rel_obj"),
                txt_id: get_string(record, "txt_id"),
            };

            result.entry(composite_id).or_default().push(item);
        }

        // Sort by item_pos
        for items in result.values_mut() {
            items.sort_by_key(|i| i.item_pos);
        }

        Ok(result)
    }

    /// Read composite products from ocd_composite table
    fn read_composites(
        reader: &mut EBaseReader,
    ) -> Result<HashMap<String, OcdComposite>, String> {
        let mut result = HashMap::new();

        if !reader.tables.contains_key("ocd_composite") {
            return Ok(result);
        }

        let records = reader
            .read_records("ocd_composite", None)
            .map_err(|e| e.to_string())?;

        for record in &records {
            let composite_id = get_string(record, "composite_id");

            if composite_id.is_empty() {
                continue;
            }

            let composite = OcdComposite {
                composite_id: composite_id.clone(),
                basket_mode: get_string(record, "basket_mode"),
                price_mode: get_string(record, "price_mode"),
                text_mode: get_string(record, "text_mode"),
                configurable: get_u8(record, "configurable") != 0,
                items_configurable: get_u8(record, "items_configurable") != 0,
                is_fixed: get_u8(record, "is_fixed") != 0,
            };

            result.insert(composite_id, composite);
        }

        Ok(result)
    }

    /// Read version info from ocd_version table
    fn read_version(reader: &mut EBaseReader) -> Result<Option<OcdVersion>, String> {
        if !reader.tables.contains_key("ocd_version") {
            return Ok(None);
        }

        let records = reader
            .read_records("ocd_version", Some(1))
            .map_err(|e| e.to_string())?;

        if let Some(record) = records.first() {
            Ok(Some(OcdVersion {
                data_version: get_string(record, "data_version"),
                format_version: get_string(record, "format_version"),
                region: get_string(record, "region"),
                date_from: get_string(record, "date_from"),
                date_to: get_string(record, "date_to"),
                rel_coding: get_string(record, "rel_coding"),
                comment: get_string(record, "comment"),
                tables: get_string(record, "tables"),
                varcond_var: get_string(record, "varcond_var"),
                placeholder_on: get_u8(record, "placeholder_on") != 0,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get property values from a TABLE relation
    /// Returns values by querying the custom table with current property selections
    pub fn get_table_values(
        &self,
        prop_class: &str,
        property: &str,
        current_selections: &HashMap<String, String>,
    ) -> Vec<OcdPropertyValue> {
        let mut result = Vec::new();

        // Get the property definition
        let prop_def = match self
            .properties
            .get(&(prop_class.to_string(), property.to_string()))
        {
            Some(def) => def,
            None => return result,
        };

        // Check if this property has a relation object
        if prop_def.rel_obj == 0 {
            return result;
        }

        // Get the relation object to find the rel_name
        let rel_obj = match self.relation_objs.get(&prop_def.rel_obj) {
            Some(obj) => obj,
            None => return result,
        };

        // Get the relations by rel_name
        let relations = match self.relations.get(&rel_obj.rel_name) {
            Some(rels) => rels,
            None => return result,
        };

        // Concatenate all relation blocks
        let rel_text: String = relations
            .iter()
            .map(|r| r.rel_block.clone())
            .collect::<Vec<_>>()
            .join(" ");

        // Find TABLE relation
        if let Some(table_rel) = Self::parse_table_relation(&rel_text) {
            let table_name = table_rel.table_name.to_lowercase() + "_tbl";

            // Get the custom table data
            let table_data = match self.custom_tables.get(&table_name) {
                Some(data) => data,
                None => return result,
            };

            // Find the target column (the one with $SELF reference)
            let target_col = match &table_rel.target_column {
                Some(col) => col.to_lowercase(),
                None => {
                    // If no explicit target, use the property name
                    property.to_lowercase()
                }
            };

            // Build filter conditions from column mappings
            let mut filters: Vec<(String, String)> = Vec::new();
            for (col, val) in &table_rel.column_mappings {
                if !val.contains("$SELF") {
                    // This is a filter column - value references another property
                    if let Some(selected_value) = current_selections.get(val) {
                        filters.push((col.to_lowercase(), selected_value.clone()));
                    }
                }
            }

            // Query the table and collect unique values for the target column
            let mut seen_values = std::collections::HashSet::new();
            let mut position: u16 = 1;

            // Common text/label column names to check in the table
            let text_columns = ["text", "description", "desc", "label", "name", "bezeichnung"];

            for row in table_data {
                // Check if this row matches all filters (or no filters means match all)
                let matches = filters.is_empty()
                    || filters.iter().all(|(col, expected)| {
                        row.get(col)
                            .map(|v| v.eq_ignore_ascii_case(expected))
                            .unwrap_or(false)
                    });

                if matches {
                    // Get the value from target column
                    if let Some(value) = row.get(&target_col) {
                        if !value.is_empty() && !seen_values.contains(value) {
                            seen_values.insert(value.clone());

                            // Try to find textnr from multiple sources:
                            // 1. Check for text column in the table row
                            // 2. Check if value itself is a valid textnr in value_texts
                            // 3. Use empty string (value will be used as label)
                            let textnr = text_columns
                                .iter()
                                .filter_map(|col| row.get(*col))
                                .find(|v| !v.is_empty())
                                .cloned()
                                .or_else(|| {
                                    // Check if the value itself is a valid textnr
                                    if self.value_texts.contains_key(value) {
                                        Some(value.clone())
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_default();

                            // Create a synthetic property value
                            result.push(OcdPropertyValue {
                                prop_class: prop_class.to_string(),
                                property: property.to_string(),
                                position,
                                textnr,
                                is_default: position == 1,
                                value_from: value.clone(),
                                value_to: String::new(),
                                op_from: String::new(),
                                op_to: String::new(),
                                raster: String::new(),
                            });

                            position += 1;
                        }
                    }
                }
            }
        }

        result
    }

    /// Check if a property uses TABLE relations
    pub fn property_uses_table(&self, prop_class: &str, property: &str) -> bool {
        if let Some(def) = self
            .properties
            .get(&(prop_class.to_string(), property.to_string()))
        {
            if def.rel_obj > 0 {
                if let Some(rel_obj) = self.relation_objs.get(&def.rel_obj) {
                    if let Some(relations) = self.relations.get(&rel_obj.rel_name) {
                        return relations
                            .iter()
                            .any(|r| r.rel_block.trim().to_uppercase().starts_with("TABLE "));
                    }
                }
            }
        }
        false
    }

    /// Compute var_cond from property selections using TABLE relations
    /// This handles manufacturers like FAST that use $VARCOND = PropertyName assignments
    /// with cascading TABLE lookups to derive the var_cond dynamically.
    ///
    /// Returns the computed var_cond if successful, None if this manufacturer
    /// doesn't use TABLE-based var_cond computation.
    pub fn compute_varcond_from_selections(
        &self,
        prop_class: &str,
        selections: &HashMap<String, String>,
    ) -> Option<String> {
        // Step 1: Find $VARCOND assignment in relations
        // Look for patterns like: "$VARCOND = PropertyName" or "$VARCOND=PropertyName"
        let mut varcond_property: Option<String> = None;

        for relations in self.relations.values() {
            for rel in relations {
                let block = rel.rel_block.trim().to_uppercase();
                if block.contains("$VARCOND") && block.contains('=') {
                    // Parse $VARCOND = PropertyName
                    // Handle both "$VARCOND = Artikelnummer" and "$VARCOND=Artikelnummer"
                    let parts: Vec<&str> = block.split('=').collect();
                    if parts.len() == 2 {
                        let left = parts[0].trim();
                        let right = parts[1].trim();
                        if left.contains("$VARCOND") || left.contains("VARCOND") {
                            // right is the property name that provides var_cond
                            varcond_property = Some(right.to_string());
                            break;
                        }
                    }
                }
            }
            if varcond_property.is_some() {
                break;
            }
        }

        let varcond_prop = varcond_property?;

        // Step 2: Compute the value of that property via TABLE lookups
        // The property might be computed from other properties via cascading tables
        self.compute_property_value(prop_class, &varcond_prop, selections)
    }

    /// Compute a property value via TABLE lookups
    /// Handles cascading dependencies where one property depends on others
    fn compute_property_value(
        &self,
        prop_class: &str,
        property: &str,
        selections: &HashMap<String, String>,
    ) -> Option<String> {
        // Check if this property value is directly provided in selections
        if let Some(value) = selections.get(property) {
            return Some(value.clone());
        }
        // Also check case-insensitive
        let prop_lower = property.to_lowercase();
        for (key, value) in selections {
            if key.to_lowercase() == prop_lower {
                return Some(value.clone());
            }
        }

        // Try to find a matching TABLE relation across all relations
        // This handles cases where multiple products share a property class but have
        // different TABLE relations (e.g., FAST WKM vs KR both use "Rahmen" class)
        let result = self.try_compute_from_any_table_relation(prop_class, property, selections);
        if result.is_some() {
            return result;
        }

        // Fallback: Find the property definition and use its specific relation
        let prop_key = (prop_class.to_string(), property.to_string());
        let prop_def = self.properties.get(&prop_key).or_else(|| {
            // Try case-insensitive search
            self.properties
                .iter()
                .find(|((pc, p), _)| pc == prop_class && p.to_uppercase() == property)
                .map(|(_, def)| def)
        })?;

        // Get the relation object
        if prop_def.rel_obj == 0 {
            return None;
        }
        let rel_obj = self.relation_objs.get(&prop_def.rel_obj)?;

        // Get relations
        let relations = self.relations.get(&rel_obj.rel_name)?;

        // Concatenate relation blocks
        let rel_text: String = relations
            .iter()
            .map(|r| r.rel_block.clone())
            .collect::<Vec<_>>()
            .join(" ");

        // Parse TABLE relation
        let table_rel = Self::parse_table_relation(&rel_text)?;
        let table_name = table_rel.table_name.to_lowercase() + "_tbl";

        // Get the custom table data
        let table_data = self.custom_tables.get(&table_name)?;

        // Build filter conditions - first compute any dependent property values
        let mut computed_selections = selections.clone();
        for (_col, val) in &table_rel.column_mappings {
            if !val.contains("$SELF") {
                // This column references another property - recursively compute its value
                if !computed_selections.contains_key(val) {
                    let val_upper = val.to_uppercase();
                    let found = computed_selections
                        .iter()
                        .any(|(k, _)| k.to_uppercase() == val_upper);
                    if !found {
                        // Try to compute this dependent property
                        if let Some(computed) =
                            self.compute_property_value(prop_class, val, selections)
                        {
                            computed_selections.insert(val.clone(), computed);
                        }
                    }
                }
            }
        }

        // Build filter conditions from computed selections
        let mut filters: Vec<(String, String)> = Vec::new();
        for (col, val) in &table_rel.column_mappings {
            if !val.contains("$SELF") {
                // Look for value in selections (case-insensitive)
                let val_lower = val.to_lowercase();
                let val_upper = val.to_uppercase();
                for (sel_key, sel_val) in &computed_selections {
                    if sel_key.to_lowercase() == val_lower || sel_key.to_uppercase() == val_upper {
                        filters.push((col.to_lowercase(), sel_val.clone()));
                        break;
                    }
                }
            }
        }

        // Find the target column (the one with $SELF reference)
        let target_col = table_rel
            .target_column
            .as_ref()
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| property.to_lowercase());

        // Query the table
        for row in table_data {
            // Check if this row matches all filters
            let matches = filters.is_empty()
                || filters.iter().all(|(col, expected)| {
                    row.get(col)
                        .map(|v| v.eq_ignore_ascii_case(expected))
                        .unwrap_or(false)
                });

            if matches {
                // Get the value from target column
                if let Some(value) = row.get(&target_col) {
                    if !value.is_empty() {
                        return Some(value.clone());
                    }
                }
            }
        }

        None
    }

    /// Try to compute a property value from any matching TABLE relation
    /// This handles cases where multiple products share a property class but have
    /// different TABLE relations with different input properties
    fn try_compute_from_any_table_relation(
        &self,
        _prop_class: &str,
        target_property: &str,
        selections: &HashMap<String, String>,
    ) -> Option<String> {
        let target_upper = target_property.to_uppercase();

        // Collect all TABLE relations that output the target property
        let mut candidate_relations: Vec<(&str, TableRelation)> = Vec::new();

        for (rel_name, relations) in &self.relations {
            for rel in relations {
                if let Some(table_rel) = Self::parse_table_relation(&rel.rel_block) {
                    // Check if this TABLE outputs the target property
                    let outputs_target = table_rel.column_mappings.iter().any(|(col, val)| {
                        val.contains("$SELF") && col.to_uppercase() == target_upper
                    });

                    if outputs_target {
                        candidate_relations.push((rel_name.as_str(), table_rel));
                    }
                }
            }
        }

        // Try each candidate relation - use the one that matches available selections
        for (_rel_name, table_rel) in candidate_relations {
            // Check if all required input properties are available in selections
            let mut required_inputs: Vec<String> = Vec::new();
            for (col, val) in &table_rel.column_mappings {
                if !val.contains("$SELF") {
                    // This is an input column
                    required_inputs.push(val.clone());
                }
                // Also check the column name as potential input (for direct mappings like Moos=Moos)
                if col.to_uppercase() == val.to_uppercase() && !val.contains("$SELF") {
                    required_inputs.push(col.clone());
                }
            }

            // Check if all required inputs are available (case-insensitive)
            let all_inputs_available = required_inputs.iter().all(|input| {
                let input_lower = input.to_lowercase();
                selections
                    .iter()
                    .any(|(k, _)| k.to_lowercase() == input_lower)
            });

            if !all_inputs_available {
                continue; // Try next candidate
            }

            // All inputs available - try to execute this table lookup
            let table_name = table_rel.table_name.to_lowercase() + "_tbl";
            let table_data = match self.custom_tables.get(&table_name) {
                Some(data) => data,
                None => continue,
            };

            // Build filter conditions
            let mut filters: Vec<(String, String)> = Vec::new();
            for (col, val) in &table_rel.column_mappings {
                if !val.contains("$SELF") {
                    // Look for value in selections (case-insensitive)
                    let val_lower = val.to_lowercase();
                    for (sel_key, sel_val) in selections {
                        if sel_key.to_lowercase() == val_lower {
                            filters.push((col.to_lowercase(), sel_val.clone()));
                            break;
                        }
                    }
                }
            }

            // Find target column - must match the target property we're looking for
            // When a TABLE has multiple $SELF outputs (like Groesse + Artikelnummer),
            // we need to select the one matching our target property
            let target_col = table_rel
                .column_mappings
                .iter()
                .find(|(col, val)| val.contains("$SELF") && col.to_uppercase() == target_upper)
                .map(|(col, _)| col.to_lowercase())
                .unwrap_or_else(|| target_property.to_lowercase());

            // Query the table
            for row in table_data {
                let matches = filters.is_empty()
                    || filters.iter().all(|(col, expected)| {
                        row.get(col)
                            .map(|v| v.eq_ignore_ascii_case(expected))
                            .unwrap_or(false)
                    });

                if matches {
                    if let Some(value) = row.get(&target_col) {
                        if !value.is_empty() {
                            return Some(value.clone());
                        }
                    }
                }
            }
        }

        None
    }

    /// Check if this manufacturer uses TABLE-based var_cond computation
    /// Returns true if there's a $VARCOND assignment in any relation
    pub fn uses_table_varcond(&self) -> bool {
        for relations in self.relations.values() {
            for rel in relations {
                let block = rel.rel_block.trim().to_uppercase();
                if block.contains("$VARCOND") && block.contains('=') {
                    return true;
                }
            }
        }
        false
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

    /// Get properties for a property class filtered by source series
    /// This is used when different families share a property class but have different properties
    /// (e.g., FAST KR vs WKM both use "Rahmen" but have different configurable properties)
    pub fn get_properties_for_class_and_series(
        &self,
        prop_class: &str,
        series: &str,
    ) -> Vec<&OcdPropertyDef> {
        let series_lower = series.to_lowercase();
        let mut props: Vec<_> = self
            .properties
            .iter()
            .filter(|((pc, _), def)| {
                pc == prop_class
                    && def
                        .source_series
                        .as_ref()
                        .map(|s| s.to_lowercase() == series_lower)
                        .unwrap_or(false)
            })
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
            // Try exact language match first (case-insensitive)
            let lang_lower = language.to_lowercase();
            let exact_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == lang_lower)
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

            // Fallback priority: EN > other languages (case-insensitive)
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

    /// Get property value label from ocd_propvaluetext
    pub fn get_value_label(&self, value: &OcdPropertyValue, language: &str) -> Option<String> {
        self.get_value_text(&value.textnr, language)
    }

    /// Get text from value_texts (ocd_propvaluetext)
    pub fn get_value_text(&self, textnr: &str, language: &str) -> Option<String> {
        self.value_texts.get(textnr).and_then(|texts| {
            // Try exact language match first (case-insensitive)
            let lang_lower = language.to_lowercase();
            let exact_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == lang_lower)
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

            // Fallback: EN > any language
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

    /// Get text from price_texts (ocd_pricetext) for surcharge descriptions
    pub fn get_price_text(&self, textnr: &str, language: &str) -> Option<String> {
        self.price_texts.get(textnr).and_then(|texts| {
            // Try exact language match first (case-insensitive)
            let lang_lower = language.to_lowercase();
            let exact_match: Vec<_> = texts
                .iter()
                .filter(|t| t.language.to_lowercase() == lang_lower)
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

            // Fallback: EN > any language
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

    /// Get the default value for a property
    pub fn get_default_value(&self, prop_class: &str, property: &str) -> Option<&OcdPropertyValue> {
        self.values
            .get(&(prop_class.to_string(), property.to_string()))
            .and_then(|values| {
                values
                    .iter()
                    .find(|v| v.is_default)
                    .or_else(|| values.first())
            })
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

    // ========== Helper methods for new OCD tables ==========

    /// Get display label for a property class from ocd_propclasstext
    /// This provides human-readable names for property classes like "Farbe" -> "Color"
    pub fn get_class_label(&self, textnr: &str, language: &str) -> Option<String> {
        self.prop_class_texts.get(textnr).and_then(|texts| {
            let lang_upper = language.to_uppercase();
            // Try exact language match first
            if let Some(text) = texts.iter().find(|t| t.language.to_uppercase() == lang_upper) {
                return Some(text.text.clone());
            }
            // Try English fallback
            if let Some(text) = texts.iter().find(|t| t.language.eq_ignore_ascii_case("EN")) {
                return Some(text.text.clone());
            }
            // Return any available text
            texts.first().map(|t| t.text.clone())
        })
    }

    /// Get hint text for a property from ocd_prophinttext
    /// Hints provide additional context or tooltips for properties
    pub fn get_hint_text(&self, textnr: &str, language: &str) -> Option<String> {
        self.hint_texts.get(textnr).and_then(|texts| {
            let lang_upper = language.to_uppercase();
            if let Some(text) = texts.iter().find(|t| t.language.to_uppercase() == lang_upper) {
                return Some(text.text.clone());
            }
            if let Some(text) = texts.iter().find(|t| t.language.eq_ignore_ascii_case("EN")) {
                return Some(text.text.clone());
            }
            texts.first().map(|t| t.text.clone())
        })
    }

    /// Get user message by id from ocd_usermessage
    /// User messages are displayed to the user during configuration
    pub fn get_user_message(&self, textnr: &str, language: &str) -> Option<String> {
        self.user_messages.get(textnr).and_then(|texts| {
            let lang_upper = language.to_uppercase();
            if let Some(text) = texts.iter().find(|t| t.language.to_uppercase() == lang_upper) {
                return Some(text.text.clone());
            }
            if let Some(text) = texts.iter().find(|t| t.language.eq_ignore_ascii_case("EN")) {
                return Some(text.text.clone());
            }
            texts.first().map(|t| t.text.clone())
        })
    }

    /// Get property group label from ocd_propgrouptext
    pub fn get_prop_group_label(&self, textnr: &str, language: &str) -> Option<String> {
        self.prop_group_texts.get(textnr).and_then(|texts| {
            let lang_upper = language.to_uppercase();
            if let Some(text) = texts.iter().find(|t| t.language.to_uppercase() == lang_upper) {
                return Some(text.text.clone());
            }
            if let Some(text) = texts.iter().find(|t| t.language.eq_ignore_ascii_case("EN")) {
                return Some(text.text.clone());
            }
            texts.first().map(|t| t.text.clone())
        })
    }

    /// Get all property groups for a property class
    /// Returns groups sorted by position
    pub fn get_property_groups_for_class(&self, prop_class: &str) -> Vec<&OcdPropertyGroup> {
        let mut groups: Vec<_> = self
            .property_groups
            .iter()
            .filter(|((pc, _), _)| pc == prop_class)
            .map(|(_, g)| g)
            .collect();
        groups.sort_by_key(|g| g.position);
        groups
    }

    /// Get property groups assigned to an article from ocd_article2propgroup
    pub fn get_article_property_groups(&self, article_nr: &str) -> Vec<&OcdArticle2PropGroup> {
        self.article_prop_groups
            .get(article_nr)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get rounding rules by id from ocd_rounding
    /// Returns all rounding rules for the given id, sorted by nr
    pub fn get_rounding_rules(&self, id: &str) -> Vec<&OcdRounding> {
        self.rounding_rules
            .get(id)
            .map(|rules| {
                let mut sorted: Vec<_> = rules.iter().collect();
                sorted.sort_by_key(|r| r.nr);
                sorted
            })
            .unwrap_or_default()
    }

    /// Get tax scheme by tax_id from ocd_taxscheme
    pub fn get_tax_scheme(&self, tax_id: &str) -> Option<&OcdTaxScheme> {
        self.tax_schemes.get(tax_id)
    }

    /// Get tax assignments for an article from ocd_articletaxes
    pub fn get_article_taxes(&self, article_nr: &str) -> Vec<&OcdArticleTaxes> {
        self.article_taxes
            .get(article_nr)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get code scheme by code_id from ocd_codescheme
    pub fn get_code_scheme(&self, code_id: &str) -> Option<&OcdCodeScheme> {
        self.code_schemes.get(code_id)
    }

    /// Format a variant code using the default code scheme
    /// Returns the formatted code or the original if no scheme is available
    pub fn format_variant_code(&self, variant_code: &str) -> String {
        // Try to find a default code scheme
        let scheme = self
            .code_schemes
            .get("DEFAULT")
            .or_else(|| self.code_schemes.get("1"))
            .or_else(|| self.code_schemes.values().next());

        if let Some(scheme) = scheme {
            // Apply formatting rules
            let mut formatted = variant_code.to_string();

            // Trim if specified
            if scheme.trim {
                formatted = formatted.trim().to_string();
            }

            // Replace separators if different from default
            if !scheme.varcode_sep.is_empty() && scheme.varcode_sep != "_" {
                // The raw variant code uses _ as separator
                // Replace with the scheme's separator
                formatted = formatted.replace('_', &scheme.varcode_sep);
            }

            formatted
        } else {
            variant_code.to_string()
        }
    }

    /// Get a human-readable variant code display
    /// Uses code scheme formatting if available
    pub fn format_variant_code_display(
        &self,
        variant_code: &str,
        max_length: usize,
    ) -> String {
        let formatted = self.format_variant_code(variant_code);

        // Truncate if too long
        if formatted.chars().count() > max_length {
            let truncated: String = formatted.chars().take(max_length - 3).collect();
            format!("{}...", truncated)
        } else {
            formatted
        }
    }

    /// Get article base information from ocd_artbase
    pub fn get_art_base(&self, article_nr: &str) -> Vec<&OcdArtBase> {
        self.art_base
            .get(article_nr)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get packaging information for an article from ocd_packaging
    pub fn get_packaging(&self, article_nr: &str) -> Vec<&OcdPackaging> {
        self.packaging
            .get(article_nr)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get bill of items (components) for a composite from ocd_billofitems
    pub fn get_bill_of_items(&self, composite_id: &str) -> Vec<&OcdBillOfItems> {
        self.bill_of_items
            .get(composite_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get composite product information from ocd_composite
    pub fn get_composite(&self, product_id: &str) -> Option<&OcdComposite> {
        self.composites.get(product_id)
    }

    /// Get data version information from ocd_version
    pub fn get_data_version(&self) -> Option<&OcdVersion> {
        self.version.as_ref()
    }

    /// Check if data is valid for a given date
    /// Returns None if no version info, or Some(true/false) for validity
    pub fn is_data_valid_for_date(&self, date: chrono::NaiveDate) -> Option<bool> {
        let version = self.version.as_ref()?;

        // Parse date_from (format: YYYYMMDD)
        let date_from = chrono::NaiveDate::parse_from_str(&version.date_from, "%Y%m%d").ok();
        let date_to = chrono::NaiveDate::parse_from_str(&version.date_to, "%Y%m%d").ok();

        match (date_from, date_to) {
            (Some(from), Some(to)) => Some(date >= from && date <= to),
            (Some(from), None) => Some(date >= from),
            (None, Some(to)) => Some(date <= to),
            (None, None) => None, // No dates to check
        }
    }

    /// Get a warning message if data is expired for a given date
    pub fn get_data_validity_warning(&self, date: chrono::NaiveDate) -> Option<String> {
        let version = self.version.as_ref()?;

        if let Some(valid) = self.is_data_valid_for_date(date) {
            if !valid {
                // Data is not valid for this date
                let date_to = chrono::NaiveDate::parse_from_str(&version.date_to, "%Y%m%d").ok();
                if let Some(to) = date_to {
                    if date > to {
                        return Some(format!(
                            "Preisdaten abgelaufen seit {}",
                            to.format("%d.%m.%Y")
                        ));
                    }
                }
                return Some("Preisdaten außerhalb Gültigkeitszeitraum".to_string());
            }
        }
        None
    }

    /// Apply rounding rules to a value
    /// Returns the rounded value according to the rules
    pub fn apply_rounding(&self, rounding_id: &str, value: f64) -> f64 {
        let rules = self.get_rounding_rules(rounding_id);
        if rules.is_empty() {
            return value;
        }

        // Find the applicable rule based on min/max range
        for rule in rules {
            if value >= rule.min && value <= rule.max {
                let adjusted = value + rule.add_before;
                let rounded = match rule.rounding_type.as_str() {
                    "UP" => (adjusted / 10f64.powi(rule.precision as i32)).ceil()
                        * 10f64.powi(rule.precision as i32),
                    "DOWN" => (adjusted / 10f64.powi(rule.precision as i32)).floor()
                        * 10f64.powi(rule.precision as i32),
                    _ => (adjusted / 10f64.powi(rule.precision as i32)).round()
                        * 10f64.powi(rule.precision as i32),
                };
                return rounded + rule.add_after;
            }
        }

        value
    }

    /// Calculate tax for an article based on its tax assignments
    pub fn calculate_article_tax(&self, article_nr: &str, base_price: f64) -> f64 {
        let taxes = self.get_article_taxes(article_nr);
        let mut total_tax = 0.0;

        for tax_assignment in taxes {
            if let Some(scheme) = self.get_tax_scheme(&tax_assignment.tax_id) {
                // Apply tax based on type
                match scheme.tax_type.as_str() {
                    "PERCENT" => total_tax += base_price * (scheme.number / 100.0),
                    "ABSOLUTE" => total_tax += scheme.number,
                    _ => {}
                }
            }
        }

        total_tax
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

fn get_u32(record: &HashMap<String, Value>, key: &str) -> u32 {
    record.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as u32
}

fn get_f64(record: &HashMap<String, Value>, key: &str) -> f64 {
    record
        .get(key)
        .and_then(|v| match v {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            Value::UInt(u) => Some(*u as f64),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        })
        .unwrap_or(0.0)
}

use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

/// Cache entry with TTL support
struct CacheEntry<T> {
    data: T,
    created_at: Instant,
}

impl<T> CacheEntry<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            created_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}

/// Default TTL for property cache (5 minutes)
const PROPERTY_CACHE_TTL: Duration = Duration::from_secs(300);

/// Cache for aggregated property readers per manufacturer
static PROPERTY_CACHE: OnceLock<Mutex<HashMap<String, CacheEntry<Arc<OcdPropertyReader>>>>> =
    OnceLock::new();

fn get_property_cache() -> &'static Mutex<HashMap<String, CacheEntry<Arc<OcdPropertyReader>>>> {
    PROPERTY_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Clear the property cache (useful for testing)
pub fn clear_property_cache() {
    if let Some(cache) = PROPERTY_CACHE.get() {
        if let Ok(mut guard) = cache.lock() {
            guard.clear();
        }
    }
}

/// Evict expired entries from the property cache
pub fn evict_expired_cache_entries() {
    if let Some(cache) = PROPERTY_CACHE.get() {
        if let Ok(mut guard) = cache.lock() {
            guard.retain(|_, entry| !entry.is_expired(PROPERTY_CACHE_TTL));
        }
    }
}

/// Get cache statistics (for debugging/monitoring)
pub fn get_cache_stats() -> (usize, usize) {
    let mut total = 0;
    let mut expired = 0;
    if let Some(cache) = PROPERTY_CACHE.get() {
        if let Ok(guard) = cache.lock() {
            total = guard.len();
            expired = guard.values().filter(|e| e.is_expired(PROPERTY_CACHE_TTL)).count();
        }
    }
    (total, expired)
}

/// Find all pdata.ebase files and aggregate property data (with caching)
pub fn load_manufacturer_properties(manufacturer_path: &Path) -> OcdPropertyReader {
    let cache_key = manufacturer_path.to_string_lossy().to_string();

    // Check cache first (handle poisoned mutex gracefully)
    {
        let cache = get_property_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(entry) = cache.get(&cache_key) {
            // Return cached data if not expired
            if !entry.is_expired(PROPERTY_CACHE_TTL) {
                let reader = &entry.data;
                return OcdPropertyReader {
                    properties: reader.properties.clone(),
                    values: reader.values.clone(),
                    classes: reader.classes.clone(),
                    texts: reader.texts.clone(),
                    value_texts: reader.value_texts.clone(),
                    price_texts: reader.price_texts.clone(),
                    prop_class_texts: reader.prop_class_texts.clone(),
                    hint_texts: reader.hint_texts.clone(),
                    user_messages: reader.user_messages.clone(),
                    prop_group_texts: reader.prop_group_texts.clone(),
                    relation_objs: reader.relation_objs.clone(),
                    relations: reader.relations.clone(),
                    custom_tables: reader.custom_tables.clone(),
                    property_groups: reader.property_groups.clone(),
                    article_prop_groups: reader.article_prop_groups.clone(),
                    rounding_rules: reader.rounding_rules.clone(),
                    tax_schemes: reader.tax_schemes.clone(),
                    article_taxes: reader.article_taxes.clone(),
                    code_schemes: reader.code_schemes.clone(),
                    art_base: reader.art_base.clone(),
                    packaging: reader.packaging.clone(),
                    bill_of_items: reader.bill_of_items.clone(),
                    composites: reader.composites.clone(),
                    version: reader.version.clone(),
                };
            }
            // Entry is expired, will be replaced below
        }
    }

    // Not in cache, load it
    let combined = load_manufacturer_properties_uncached(manufacturer_path);

    // Store in cache (handle poisoned mutex gracefully)
    {
        let mut cache = get_property_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.insert(
            cache_key,
            CacheEntry::new(Arc::new(OcdPropertyReader {
                properties: combined.properties.clone(),
                values: combined.values.clone(),
                classes: combined.classes.clone(),
                texts: combined.texts.clone(),
                value_texts: combined.value_texts.clone(),
                price_texts: combined.price_texts.clone(),
                prop_class_texts: combined.prop_class_texts.clone(),
                hint_texts: combined.hint_texts.clone(),
                user_messages: combined.user_messages.clone(),
                prop_group_texts: combined.prop_group_texts.clone(),
                relation_objs: combined.relation_objs.clone(),
                relations: combined.relations.clone(),
                custom_tables: combined.custom_tables.clone(),
                property_groups: combined.property_groups.clone(),
                article_prop_groups: combined.article_prop_groups.clone(),
                rounding_rules: combined.rounding_rules.clone(),
                tax_schemes: combined.tax_schemes.clone(),
                article_taxes: combined.article_taxes.clone(),
                code_schemes: combined.code_schemes.clone(),
                art_base: combined.art_base.clone(),
                packaging: combined.packaging.clone(),
                bill_of_items: combined.bill_of_items.clone(),
                composites: combined.composites.clone(),
                version: combined.version.clone(),
            })),
        );
    }

    combined
}

/// Extract series name from pdata.ebase path
/// e.g., "/reference/ofmldata/fast/kr/DE/1/db/pdata.ebase" -> Some("kr")
fn extract_series_from_path(pdata_path: &Path) -> Option<String> {
    // Path structure: manufacturer/series/LANG/version/db/pdata.ebase
    // We need to go up 4 levels from pdata.ebase to get the series
    pdata_path
        .parent() // db
        .and_then(|p| p.parent()) // version
        .and_then(|p| p.parent()) // LANG
        .and_then(|p| p.parent()) // series
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .map(|s| s.to_lowercase())
}

/// Find all pdata.ebase files and aggregate property data (uncached)
fn load_manufacturer_properties_uncached(manufacturer_path: &Path) -> OcdPropertyReader {
    let mut combined = OcdPropertyReader {
        properties: HashMap::new(),
        values: HashMap::new(),
        classes: HashMap::new(),
        texts: HashMap::new(),
        value_texts: HashMap::new(),
        price_texts: HashMap::new(),
        prop_class_texts: HashMap::new(),
        hint_texts: HashMap::new(),
        user_messages: HashMap::new(),
        prop_group_texts: HashMap::new(),
        relation_objs: HashMap::new(),
        relations: HashMap::new(),
        custom_tables: HashMap::new(),
        property_groups: HashMap::new(),
        article_prop_groups: HashMap::new(),
        rounding_rules: HashMap::new(),
        tax_schemes: HashMap::new(),
        article_taxes: HashMap::new(),
        code_schemes: HashMap::new(),
        art_base: HashMap::new(),
        packaging: HashMap::new(),
        bill_of_items: HashMap::new(),
        composites: HashMap::new(),
        version: None,
    };

    // Find all pdata.ebase files
    let pdata_paths = super::ocd::find_pdata_files(manufacturer_path);

    // Load all EBase files in parallel
    let loaded_readers: Vec<(Option<String>, OcdPropertyReader)> = pdata_paths
        .par_iter()
        .filter_map(|pdata_path| {
            let series = extract_series_from_path(pdata_path);
            OcdPropertyReader::from_ebase(pdata_path)
                .ok()
                .map(|reader| (series, reader))
        })
        .collect();

    // Merge sequentially (needed for textnr validity checks)
    for (series, reader) in loaded_readers {
        // FIRST: Merge texts so we can check textnr validity when merging properties
        for (key, texts) in reader.texts {
            combined.texts.entry(key).or_default().extend(texts);
        }
        for (key, texts) in reader.value_texts {
            combined.value_texts.entry(key).or_default().extend(texts);
        }
        for (key, texts) in reader.price_texts {
            combined.price_texts.entry(key).or_default().extend(texts);
        }

        // THEN: Merge properties - prefer properties with valid textnr
        for (key, mut prop) in reader.properties {
            // Set the source series for this property
            prop.source_series = series.clone();

            combined
                .properties
                .entry(key)
                .and_modify(|existing| {
                    // Prefer properties with valid textnr (that exists in our texts)
                    let new_has_valid_textnr =
                        !prop.textnr.is_empty() && combined.texts.contains_key(&prop.textnr);
                    let existing_has_valid_textnr = !existing.textnr.is_empty()
                        && combined.texts.contains_key(&existing.textnr);

                    // Replace if:
                    // 1. New has valid textnr and existing doesn't, OR
                    // 2. New has TABLE relation and existing doesn't
                    if (new_has_valid_textnr && !existing_has_valid_textnr)
                        || (prop.rel_obj > 0 && existing.rel_obj == 0)
                    {
                        *existing = prop.clone();
                    }
                })
                .or_insert(prop);
        }
        for (key, values) in reader.values {
            combined.values.entry(key).or_default().extend(values);
        }
        combined.classes.extend(reader.classes);

        // Merge relation data
        combined.relation_objs.extend(reader.relation_objs);
        for (key, relations) in reader.relations {
            combined.relations.entry(key).or_default().extend(relations);
        }
        for (key, table_data) in reader.custom_tables {
            combined
                .custom_tables
                .entry(key)
                .or_default()
                .extend(table_data);
        }

        // Merge new text tables
        for (key, texts) in reader.prop_class_texts {
            combined.prop_class_texts.entry(key).or_default().extend(texts);
        }
        for (key, texts) in reader.hint_texts {
            combined.hint_texts.entry(key).or_default().extend(texts);
        }
        for (key, texts) in reader.user_messages {
            combined.user_messages.entry(key).or_default().extend(texts);
        }
        for (key, texts) in reader.prop_group_texts {
            combined.prop_group_texts.entry(key).or_default().extend(texts);
        }

        // Merge property groups and article mappings
        combined.property_groups.extend(reader.property_groups);
        for (key, mappings) in reader.article_prop_groups {
            combined.article_prop_groups.entry(key).or_default().extend(mappings);
        }

        // Merge pricing support tables
        for (key, rules) in reader.rounding_rules {
            combined.rounding_rules.entry(key).or_default().extend(rules);
        }
        combined.tax_schemes.extend(reader.tax_schemes);
        for (key, taxes) in reader.article_taxes {
            combined.article_taxes.entry(key).or_default().extend(taxes);
        }

        // Merge other tables
        combined.code_schemes.extend(reader.code_schemes);
        for (key, bases) in reader.art_base {
            combined.art_base.entry(key).or_default().extend(bases);
        }
        for (key, pkgs) in reader.packaging {
            combined.packaging.entry(key).or_default().extend(pkgs);
        }
        for (key, items) in reader.bill_of_items {
            combined.bill_of_items.entry(key).or_default().extend(items);
        }
        combined.composites.extend(reader.composites);

        // Take version info if not already set
        if combined.version.is_none() {
            combined.version = reader.version;
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

    for texts in combined.value_texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    for texts in combined.price_texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    for texts in combined.prop_class_texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    for texts in combined.hint_texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    for texts in combined.user_messages.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    for texts in combined.prop_group_texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
    }

    // Deduplicate article prop groups
    for mappings in combined.article_prop_groups.values_mut() {
        mappings.sort_by_key(|m| m.position);
        mappings.dedup_by(|a, b| a.prop_group == b.prop_group);
    }

    // Deduplicate rounding rules
    for rules in combined.rounding_rules.values_mut() {
        rules.sort_by_key(|r| r.nr);
        rules.dedup_by(|a, b| a.nr == b.nr);
    }

    // Deduplicate relations
    for relations in combined.relations.values_mut() {
        relations.sort_by_key(|r| r.rel_blocknr);
        relations.dedup_by(|a, b| a.rel_blocknr == b.rel_blocknr);
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
            let label = reader
                .get_property_label(pc, prop, "DE")
                .unwrap_or_default();
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

    #[test]
    fn test_ocd_property_def_debug_clone() {
        let def = OcdPropertyDef {
            prop_class: "PC1".to_string(),
            property: "Color".to_string(),
            position: 1,
            textnr: "T100".to_string(),
            prop_type: "CHOICE".to_string(),
            digits: 5,
            dec_digits: 0,
            need_input: true,
            add_values: false,
            restrictable: true,
            multi_option: false,
            rel_obj: 0,
            scope: "C".to_string(),
            source_series: Some("classic".to_string()),
        };

        let debug_str = format!("{:?}", def);
        assert!(debug_str.contains("Color"));

        let cloned = def.clone();
        assert_eq!(cloned.property, "Color");
        assert_eq!(cloned.prop_class, "PC1");
    }

    #[test]
    fn test_ocd_property_value_debug_clone() {
        let val = OcdPropertyValue {
            prop_class: "PC1".to_string(),
            property: "Color".to_string(),
            position: 1,
            textnr: "T200".to_string(),
            is_default: true,
            value_from: "RED".to_string(),
            value_to: "".to_string(),
            op_from: "=".to_string(),
            op_to: "".to_string(),
            raster: "".to_string(),
        };

        let debug_str = format!("{:?}", val);
        assert!(debug_str.contains("RED"));

        let cloned = val.clone();
        assert_eq!(cloned.value_from, "RED");
        assert!(cloned.is_default);
    }

    #[test]
    fn test_ocd_property_class_debug_clone() {
        let class = OcdPropertyClass {
            prop_class: "Dimensions".to_string(),
            textnr: "T300".to_string(),
            position: 1,
        };

        let debug_str = format!("{:?}", class);
        assert!(debug_str.contains("Dimensions"));

        let cloned = class.clone();
        assert_eq!(cloned.prop_class, "Dimensions");
    }

    #[test]
    fn test_ocd_property_text_debug_clone() {
        let text = OcdPropertyText {
            textnr: "T100".to_string(),
            language: "DE".to_string(),
            line_nr: 1,
            text: "Farbe".to_string(),
        };

        let debug_str = format!("{:?}", text);
        assert!(debug_str.contains("Farbe"));

        let cloned = text.clone();
        assert_eq!(cloned.text, "Farbe");
        assert_eq!(cloned.language, "DE");
    }

    #[test]
    fn test_ocd_relation_obj_debug_clone() {
        let rel_obj = OcdRelationObj {
            rel_obj: 123,
            rel_name: "color_relation".to_string(),
            rel_type: "TABLE".to_string(),
            position: 1,
        };

        let debug_str = format!("{:?}", rel_obj);
        assert!(debug_str.contains("color_relation"));

        let cloned = rel_obj.clone();
        assert_eq!(cloned.rel_obj, 123);
    }

    #[test]
    fn test_ocd_relation_debug_clone() {
        let rel = OcdRelation {
            rel_name: "color_relation".to_string(),
            rel_blocknr: 1,
            rel_block: "TABLE color_tbl".to_string(),
        };

        let debug_str = format!("{:?}", rel);
        assert!(debug_str.contains("TABLE"));

        let cloned = rel.clone();
        assert_eq!(cloned.rel_name, "color_relation");
    }

    #[test]
    fn test_table_relation_debug_clone() {
        let table_rel = TableRelation {
            table_name: "color_tbl".to_string(),
            column_mappings: vec![
                ("Color".to_string(), "Color".to_string()),
                ("Code".to_string(), "$SELF.Code".to_string()),
            ],
            target_column: Some("Value".to_string()),
        };

        let debug_str = format!("{:?}", table_rel);
        assert!(debug_str.contains("color_tbl"));

        let cloned = table_rel.clone();
        assert_eq!(cloned.column_mappings.len(), 2);
    }

    #[test]
    fn test_ocd_property_group_debug_clone() {
        let group = OcdPropertyGroup {
            prop_class: "PC1".to_string(),
            property: "Size".to_string(),
            prop_group: "Dimensions".to_string(),
            position: 1,
        };

        let debug_str = format!("{:?}", group);
        assert!(debug_str.contains("Dimensions"));

        let cloned = group.clone();
        assert_eq!(cloned.prop_group, "Dimensions");
    }

    #[test]
    fn test_ocd_article2prop_group_debug_clone() {
        let mapping = OcdArticle2PropGroup {
            article_nr: "ART001".to_string(),
            prop_group: "Standard".to_string(),
            position: 1,
            textnr: "T400".to_string(),
        };

        let debug_str = format!("{:?}", mapping);
        assert!(debug_str.contains("ART001"));

        let cloned = mapping.clone();
        assert_eq!(cloned.article_nr, "ART001");
    }

    #[test]
    fn test_ocd_rounding_debug_clone() {
        let rounding = OcdRounding {
            id: "R1".to_string(),
            nr: 1,
            rounding_type: "ROUND".to_string(),
            precision: 2,
            min: 0.0,
            max: 1000.0,
            add_before: 0.0,
            add_after: 0.0,
        };

        let debug_str = format!("{:?}", rounding);
        assert!(debug_str.contains("ROUND"));

        let cloned = rounding.clone();
        assert_eq!(cloned.precision, 2);
    }

    #[test]
    fn test_ocd_tax_scheme_debug_clone() {
        let tax = OcdTaxScheme {
            tax_id: "VAT19".to_string(),
            country: "DE".to_string(),
            region: "".to_string(),
            tax_category: "standard".to_string(),
            tax_type: "VAT".to_string(),
            number: 19.0,
        };

        let debug_str = format!("{:?}", tax);
        assert!(debug_str.contains("VAT19"));

        let cloned = tax.clone();
        assert_eq!(cloned.number, 19.0);
    }

    #[test]
    fn test_ocd_article_taxes_debug_clone() {
        let tax = OcdArticleTaxes {
            article_nr: "ART001".to_string(),
            tax_id: "VAT19".to_string(),
            date_from: "20240101".to_string(),
            date_to: "20241231".to_string(),
        };

        let debug_str = format!("{:?}", tax);
        assert!(debug_str.contains("ART001"));

        let cloned = tax.clone();
        assert_eq!(cloned.tax_id, "VAT19");
    }

    #[test]
    fn test_ocd_code_scheme_debug_clone() {
        let scheme = OcdCodeScheme {
            scheme_id: "S1".to_string(),
            scheme: "{PC}.{PROP}".to_string(),
            varcode_sep: "-".to_string(),
            value_sep: "=".to_string(),
            mo_sep: ",".to_string(),
            mo_bracket: "()".to_string(),
            invisible_char: "_".to_string(),
            unselect_char: "*".to_string(),
            visibility: "Y".to_string(),
            trim: true,
        };

        let debug_str = format!("{:?}", scheme);
        assert!(debug_str.contains("S1"));

        let cloned = scheme.clone();
        assert!(cloned.trim);
    }

    #[test]
    fn test_ocd_art_base_debug_clone() {
        let art = OcdArtBase {
            article_nr: "ART001".to_string(),
            prop_class: "PC1".to_string(),
            property: "Color".to_string(),
            prop_value: "RED".to_string(),
        };

        let debug_str = format!("{:?}", art);
        assert!(debug_str.contains("ART001"));

        let cloned = art.clone();
        assert_eq!(cloned.prop_value, "RED");
    }

    #[test]
    fn test_ocd_packaging_debug_clone() {
        let pack = OcdPackaging {
            article_nr: "ART001".to_string(),
            var_cond: "*".to_string(),
            width: 100.0,
            depth: 50.0,
            height: 30.0,
            net_weight: 10.0,
            tara_weight: 2.0,
            volume: 0.15,
            items_per_unit: 1,
            pack_units: 1,
            measure_unit: "mm".to_string(),
            weight_unit: "kg".to_string(),
            volume_unit: "m3".to_string(),
        };

        let debug_str = format!("{:?}", pack);
        assert!(debug_str.contains("ART001"));

        let cloned = pack.clone();
        assert_eq!(cloned.width, 100.0);
    }

    #[test]
    fn test_ocd_bill_of_items_debug_clone() {
        let boi = OcdBillOfItems {
            composite_id: "COMP1".to_string(),
            item_id: "ITEM1".to_string(),
            item_pos: 1,
            quantity: 2.0,
            quant_unit: "PC".to_string(),
            configurable: true,
            rel_obj: 0,
            txt_id: "".to_string(),
        };

        let debug_str = format!("{:?}", boi);
        assert!(debug_str.contains("COMP1"));

        let cloned = boi.clone();
        assert!(cloned.configurable);
    }

    #[test]
    fn test_ocd_property_reader_new_empty() {
        let reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        let (props, vals, classes, texts) = reader.stats();
        assert_eq!(props, 0);
        assert_eq!(vals, 0);
        assert_eq!(classes, 0);
        assert_eq!(texts, 0);

        let classes_list = reader.get_property_classes();
        assert!(classes_list.is_empty());
    }

    #[test]
    fn test_ocd_property_reader_get_methods() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        // Add some test data
        reader.properties.insert(
            ("PC1".to_string(), "Color".to_string()),
            OcdPropertyDef {
                prop_class: "PC1".to_string(),
                property: "Color".to_string(),
                position: 1,
                textnr: "T100".to_string(),
                prop_type: "CHOICE".to_string(),
                digits: 5,
                dec_digits: 0,
                need_input: true,
                add_values: false,
                restrictable: true,
                multi_option: false,
                rel_obj: 0,
                scope: "C".to_string(),
                source_series: None,
            },
        );

        reader.values.insert(
            ("PC1".to_string(), "Color".to_string()),
            vec![OcdPropertyValue {
                prop_class: "PC1".to_string(),
                property: "Color".to_string(),
                position: 1,
                textnr: "T200".to_string(),
                is_default: true,
                value_from: "RED".to_string(),
                value_to: "".to_string(),
                op_from: "=".to_string(),
                op_to: "".to_string(),
                raster: "".to_string(),
            }],
        );

        reader.texts.insert(
            "T100".to_string(),
            vec![OcdPropertyText {
                textnr: "T100".to_string(),
                language: "DE".to_string(),
                line_nr: 1,
                text: "Farbe".to_string(),
            }],
        );

        // Test get methods
        let values = reader.get_values_for_property("PC1", "Color");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].value_from, "RED");

        let label = reader.get_property_label("PC1", "Color", "DE");
        assert_eq!(label, Some("Farbe".to_string()));

        // When EN is not found, fallback to other language (DE in this case)
        let fallback_label = reader.get_property_label("PC1", "Color", "EN");
        // Fallback exists since we added "DE" text
        assert!(fallback_label.is_some());

        // Test with non-existent property
        let no_label = reader.get_property_label("PC99", "NoProperty", "DE");
        assert!(no_label.is_none());

        let (props, _, _, _) = reader.stats();
        assert_eq!(props, 1);
    }

    #[test]
    fn test_ocd_property_reader_get_value_text() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        // Add test value texts with different languages
        reader.value_texts.insert(
            "VT100".to_string(),
            vec![
                OcdPropertyText {
                    textnr: "VT100".to_string(),
                    language: "DE".to_string(),
                    line_nr: 1,
                    text: "Rot".to_string(),
                },
                OcdPropertyText {
                    textnr: "VT100".to_string(),
                    language: "EN".to_string(),
                    line_nr: 1,
                    text: "Red".to_string(),
                },
            ],
        );

        // Add text with empty language (universal)
        reader.value_texts.insert(
            "VT200".to_string(),
            vec![OcdPropertyText {
                textnr: "VT200".to_string(),
                language: "".to_string(),
                line_nr: 1,
                text: "Universal Value".to_string(),
            }],
        );

        // Test exact language match
        let de_text = reader.get_value_text("VT100", "DE");
        assert_eq!(de_text, Some("Rot".to_string()));

        let en_text = reader.get_value_text("VT100", "EN");
        assert_eq!(en_text, Some("Red".to_string()));

        // Test empty language match (universal)
        let universal = reader.get_value_text("VT200", "FR");
        assert_eq!(universal, Some("Universal Value".to_string()));

        // Test fallback to EN
        reader.value_texts.insert(
            "VT300".to_string(),
            vec![OcdPropertyText {
                textnr: "VT300".to_string(),
                language: "EN".to_string(),
                line_nr: 1,
                text: "English Fallback".to_string(),
            }],
        );
        let fallback = reader.get_value_text("VT300", "FR");
        assert_eq!(fallback, Some("English Fallback".to_string()));

        // Test non-existent
        let none = reader.get_value_text("VT999", "DE");
        assert!(none.is_none());
    }

    #[test]
    fn test_ocd_property_reader_get_price_text() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.price_texts.insert(
            "PT100".to_string(),
            vec![OcdPropertyText {
                textnr: "PT100".to_string(),
                language: "DE".to_string(),
                line_nr: 1,
                text: "Aufpreis".to_string(),
            }],
        );

        let text = reader.get_price_text("PT100", "DE");
        assert_eq!(text, Some("Aufpreis".to_string()));

        let none = reader.get_price_text("PT999", "DE");
        assert!(none.is_none());
    }

    #[test]
    fn test_ocd_property_reader_get_default_value() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.values.insert(
            ("PC1".to_string(), "Color".to_string()),
            vec![
                OcdPropertyValue {
                    prop_class: "PC1".to_string(),
                    property: "Color".to_string(),
                    position: 1,
                    textnr: "".to_string(),
                    is_default: false,
                    value_from: "RED".to_string(),
                    value_to: "RED".to_string(),
                    op_from: "=".to_string(),
                    op_to: "".to_string(),
                    raster: "".to_string(),
                },
                OcdPropertyValue {
                    prop_class: "PC1".to_string(),
                    property: "Color".to_string(),
                    position: 2,
                    textnr: "".to_string(),
                    is_default: true,
                    value_from: "BLUE".to_string(),
                    value_to: "BLUE".to_string(),
                    op_from: "=".to_string(),
                    op_to: "".to_string(),
                    raster: "".to_string(),
                },
            ],
        );

        let default = reader.get_default_value("PC1", "Color");
        assert!(default.is_some());
        assert_eq!(default.unwrap().value_from, "BLUE");
        assert!(default.unwrap().is_default);

        // Test fallback to first when no default
        reader.values.insert(
            ("PC1".to_string(), "Size".to_string()),
            vec![OcdPropertyValue {
                prop_class: "PC1".to_string(),
                property: "Size".to_string(),
                position: 1,
                textnr: "".to_string(),
                is_default: false,
                value_from: "L".to_string(),
                value_to: "L".to_string(),
                op_from: "=".to_string(),
                op_to: "".to_string(),
                raster: "".to_string(),
            }],
        );

        let first = reader.get_default_value("PC1", "Size");
        assert!(first.is_some());
        assert_eq!(first.unwrap().value_from, "L");

        // Test non-existent
        let none = reader.get_default_value("PC99", "Missing");
        assert!(none.is_none());
    }

    #[test]
    fn test_ocd_property_reader_has_properties() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        assert!(!reader.has_properties());

        reader.properties.insert(
            ("PC1".to_string(), "Color".to_string()),
            OcdPropertyDef {
                prop_class: "PC1".to_string(),
                property: "Color".to_string(),
                position: 1,
                textnr: "".to_string(),
                prop_type: "CHOICE".to_string(),
                digits: 0,
                dec_digits: 0,
                need_input: false,
                add_values: false,
                restrictable: false,
                multi_option: false,
                rel_obj: 0,
                scope: "C".to_string(),
                source_series: None,
            },
        );

        assert!(reader.has_properties());
    }

    #[test]
    fn test_ocd_property_reader_get_class_label() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.prop_class_texts.insert(
            "CT100".to_string(),
            vec![
                OcdPropertyText {
                    textnr: "CT100".to_string(),
                    language: "DE".to_string(),
                    line_nr: 1,
                    text: "Hauptfarbe".to_string(),
                },
                OcdPropertyText {
                    textnr: "CT100".to_string(),
                    language: "EN".to_string(),
                    line_nr: 1,
                    text: "Main Color".to_string(),
                },
            ],
        );

        // Exact match
        let de = reader.get_class_label("CT100", "DE");
        assert_eq!(de, Some("Hauptfarbe".to_string()));

        // EN fallback when requested language not found
        let fr = reader.get_class_label("CT100", "FR");
        assert_eq!(fr, Some("Main Color".to_string()));

        // Non-existent
        let none = reader.get_class_label("CT999", "DE");
        assert!(none.is_none());
    }

    #[test]
    fn test_ocd_property_reader_get_hint_text() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.hint_texts.insert(
            "HT100".to_string(),
            vec![OcdPropertyText {
                textnr: "HT100".to_string(),
                language: "DE".to_string(),
                line_nr: 1,
                text: "Wählen Sie eine Farbe".to_string(),
            }],
        );

        let hint = reader.get_hint_text("HT100", "DE");
        assert_eq!(hint, Some("Wählen Sie eine Farbe".to_string()));

        let none = reader.get_hint_text("HT999", "DE");
        assert!(none.is_none());
    }

    #[test]
    fn test_ocd_property_reader_get_user_message() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.user_messages.insert(
            "UM100".to_string(),
            vec![OcdPropertyText {
                textnr: "UM100".to_string(),
                language: "EN".to_string(),
                line_nr: 1,
                text: "Please select a color".to_string(),
            }],
        );

        let msg = reader.get_user_message("UM100", "EN");
        assert_eq!(msg, Some("Please select a color".to_string()));

        // Fallback to EN when requested language not found
        let fallback = reader.get_user_message("UM100", "FR");
        assert_eq!(fallback, Some("Please select a color".to_string()));
    }

    #[test]
    fn test_ocd_property_reader_get_prop_group_label() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.prop_group_texts.insert(
            "PG100".to_string(),
            vec![OcdPropertyText {
                textnr: "PG100".to_string(),
                language: "DE".to_string(),
                line_nr: 1,
                text: "Farboptionen".to_string(),
            }],
        );

        let label = reader.get_prop_group_label("PG100", "DE");
        assert_eq!(label, Some("Farboptionen".to_string()));
    }

    #[test]
    fn test_ocd_property_reader_get_property_groups_for_class() {
        let mut reader = OcdPropertyReader {
            properties: HashMap::new(),
            values: HashMap::new(),
            classes: HashMap::new(),
            texts: HashMap::new(),
            value_texts: HashMap::new(),
            price_texts: HashMap::new(),
            prop_class_texts: HashMap::new(),
            hint_texts: HashMap::new(),
            user_messages: HashMap::new(),
            prop_group_texts: HashMap::new(),
            relation_objs: HashMap::new(),
            relations: HashMap::new(),
            custom_tables: HashMap::new(),
            property_groups: HashMap::new(),
            article_prop_groups: HashMap::new(),
            rounding_rules: HashMap::new(),
            tax_schemes: HashMap::new(),
            article_taxes: HashMap::new(),
            code_schemes: HashMap::new(),
            art_base: HashMap::new(),
            packaging: HashMap::new(),
            bill_of_items: HashMap::new(),
            composites: HashMap::new(),
            version: None,
        };

        reader.property_groups.insert(
            ("PC1".to_string(), "Color".to_string()),
            OcdPropertyGroup {
                prop_class: "PC1".to_string(),
                property: "Color".to_string(),
                prop_group: "G1".to_string(),
                position: 2,
            },
        );

        reader.property_groups.insert(
            ("PC1".to_string(), "Size".to_string()),
            OcdPropertyGroup {
                prop_class: "PC1".to_string(),
                property: "Size".to_string(),
                prop_group: "G2".to_string(),
                position: 1,
            },
        );

        reader.property_groups.insert(
            ("PC2".to_string(), "Material".to_string()),
            OcdPropertyGroup {
                prop_class: "PC2".to_string(),
                property: "Material".to_string(),
                prop_group: "G3".to_string(),
                position: 1,
            },
        );

        let groups = reader.get_property_groups_for_class("PC1");
        assert_eq!(groups.len(), 2);
        // Should be sorted by position
        assert_eq!(groups[0].property, "Size"); // position 1
        assert_eq!(groups[1].property, "Color"); // position 2

        let empty = reader.get_property_groups_for_class("PC99");
        assert!(empty.is_empty());
    }

    #[test]
    fn test_ocd_composite_debug_clone() {
        let composite = OcdComposite {
            composite_id: "COMP1".to_string(),
            basket_mode: "ADD".to_string(),
            price_mode: "SUM".to_string(),
            text_mode: "CONCAT".to_string(),
            configurable: true,
            items_configurable: false,
            is_fixed: true,
        };

        let debug_str = format!("{:?}", composite);
        assert!(debug_str.contains("OcdComposite"));
        assert!(debug_str.contains("COMP1"));

        let cloned = composite.clone();
        assert_eq!(cloned.composite_id, "COMP1");
        assert!(cloned.configurable);
        assert!(cloned.is_fixed);
    }

    #[test]
    fn test_ocd_version_debug_clone() {
        let version = OcdVersion {
            data_version: "1.0".to_string(),
            format_version: "4.3".to_string(),
            region: "DE".to_string(),
            date_from: "2024-01-01".to_string(),
            date_to: "2024-12-31".to_string(),
            rel_coding: "HEX".to_string(),
            comment: "Test data".to_string(),
            tables: "ocd_propertydef,ocd_price".to_string(),
            varcond_var: "VC".to_string(),
            placeholder_on: true,
        };

        let debug_str = format!("{:?}", version);
        assert!(debug_str.contains("OcdVersion"));
        assert!(debug_str.contains("1.0"));

        let cloned = version.clone();
        assert_eq!(cloned.data_version, "1.0");
        assert!(cloned.tables.contains("ocd_propertydef"));
        assert!(cloned.placeholder_on);
    }
}
