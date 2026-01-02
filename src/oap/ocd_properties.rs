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
    /// Relation object ID (for TABLE relations)
    pub rel_obj: u32,
    /// Property scope (C=Choice, R=Result, RG=Range, etc.)
    pub scope: String,
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
    /// Relation objects by rel_obj ID
    pub relation_objs: HashMap<u32, OcdRelationObj>,
    /// Relations by rel_name (each rel_name can have multiple blocks)
    pub relations: HashMap<String, Vec<OcdRelation>>,
    /// Custom table data: table_name -> records (each record is field->value)
    pub custom_tables: HashMap<String, Vec<HashMap<String, String>>>,
}

impl OcdPropertyReader {
    /// Load property data from a pdata.ebase file
    pub fn from_ebase(path: &Path) -> Result<Self, String> {
        let mut reader = EBaseReader::open(path).map_err(|e| e.to_string())?;

        let properties = Self::read_properties(&mut reader)?;
        let values = Self::read_property_values(&mut reader)?;
        let classes = Self::read_property_classes(&mut reader)?;
        let texts = Self::read_property_texts(&mut reader, "ocd_propertytext")?;
        let value_texts = Self::read_property_texts(&mut reader, "ocd_propvaluetext")?;
        let relation_objs = Self::read_relation_objs(&mut reader)?;
        let relations = Self::read_relations(&mut reader)?;

        // Find which custom tables are referenced by TABLE relations
        let mut custom_tables = HashMap::new();
        for rel_list in relations.values() {
            for rel in rel_list {
                if let Some(parsed) = Self::parse_table_relation(&rel.rel_block) {
                    let table_name_lower = parsed.table_name.to_lowercase() + "_tbl";
                    if !custom_tables.contains_key(&table_name_lower) {
                        if let Ok(table_data) = Self::read_custom_table(&mut reader, &table_name_lower) {
                            custom_tables.insert(table_name_lower, table_data);
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
            relation_objs,
            relations,
            custom_tables,
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
            let a_line = a.get("line").and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
            let b_line = b.get("line").and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
            a_line.cmp(&b_line)
        });

        Ok(result)
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
        let prop_def = match self.properties.get(&(prop_class.to_string(), property.to_string())) {
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
        let rel_text: String = relations.iter()
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

            for row in table_data {
                // Check if this row matches all filters (or no filters means match all)
                let matches = filters.is_empty() || filters.iter().all(|(col, expected)| {
                    row.get(col)
                        .map(|v| v.eq_ignore_ascii_case(expected))
                        .unwrap_or(false)
                });

                if matches {
                    // Get the value from target column
                    if let Some(value) = row.get(&target_col) {
                        if !value.is_empty() && !seen_values.contains(value) {
                            seen_values.insert(value.clone());

                            // Create a synthetic property value
                            result.push(OcdPropertyValue {
                                prop_class: prop_class.to_string(),
                                property: property.to_string(),
                                position,
                                textnr: String::new(), // TODO: Try to find text
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
        if let Some(def) = self.properties.get(&(prop_class.to_string(), property.to_string())) {
            if def.rel_obj > 0 {
                if let Some(rel_obj) = self.relation_objs.get(&def.rel_obj) {
                    if let Some(relations) = self.relations.get(&rel_obj.rel_name) {
                        return relations.iter().any(|r| {
                            r.rel_block.trim().to_uppercase().starts_with("TABLE ")
                        });
                    }
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

fn get_u32(record: &HashMap<String, Value>, key: &str) -> u32 {
    record.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as u32
}

use std::sync::{Arc, OnceLock, Mutex};

/// Cache for aggregated property readers per manufacturer
static PROPERTY_CACHE: OnceLock<Mutex<HashMap<String, Arc<OcdPropertyReader>>>> = OnceLock::new();

fn get_property_cache() -> &'static Mutex<HashMap<String, Arc<OcdPropertyReader>>> {
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
                value_texts: reader.value_texts.clone(),
                relation_objs: reader.relation_objs.clone(),
                relations: reader.relations.clone(),
                custom_tables: reader.custom_tables.clone(),
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
            value_texts: combined.value_texts.clone(),
            relation_objs: combined.relation_objs.clone(),
            relations: combined.relations.clone(),
            custom_tables: combined.custom_tables.clone(),
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
        value_texts: HashMap::new(),
        relation_objs: HashMap::new(),
        relations: HashMap::new(),
        custom_tables: HashMap::new(),
    };

    // Find all pdata.ebase files
    for pdata_path in super::ocd::find_pdata_files(manufacturer_path) {
        if let Ok(reader) = OcdPropertyReader::from_ebase(&pdata_path) {
            // Merge properties - prefer properties with TABLE relations (non-zero rel_obj)
            for (key, prop) in reader.properties {
                combined.properties
                    .entry(key)
                    .and_modify(|existing| {
                        // If new property has TABLE relation and existing doesn't, replace
                        if prop.rel_obj > 0 && existing.rel_obj == 0 {
                            *existing = prop.clone();
                        }
                    })
                    .or_insert(prop);
            }
            for (key, values) in reader.values {
                combined.values.entry(key).or_default().extend(values);
            }
            combined.classes.extend(reader.classes);
            for (key, texts) in reader.texts {
                combined.texts.entry(key).or_default().extend(texts);
            }
            for (key, texts) in reader.value_texts {
                combined.value_texts.entry(key).or_default().extend(texts);
            }
            // Merge relation data
            combined.relation_objs.extend(reader.relation_objs);
            for (key, relations) in reader.relations {
                combined.relations.entry(key).or_default().extend(relations);
            }
            for (key, table_data) in reader.custom_tables {
                combined.custom_tables.entry(key).or_default().extend(table_data);
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

    for texts in combined.value_texts.values_mut() {
        texts.sort_by_key(|t| t.line_nr);
        texts.dedup_by(|a, b| a.language == b.language && a.line_nr == b.line_nr);
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
