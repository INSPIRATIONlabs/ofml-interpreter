//! OCD Relation Rules Parser and Evaluator
//!
//! This module handles the ocd_relation rules that assign $VarCond values
//! based on property conditions. These are used for pricing surcharge matching
//! when the simpler propvalue2varcond table is not present.
//!
//! Rule syntax examples:
//! - `$VarCond = 'PG_ADJUSTABLE_SEAT' if (M_ARTNO = 'ONE' and M_SEAT = 'YES')`
//! - `$VarCond = 'PG_LAN' if M_LAN = 'YES'`
//! - `M_TXT_COLOR = 'RAL9005' if M_EXTERIOR in ('RAL9016MAT','NCSS2010Y20R')`

use std::collections::{HashMap, HashSet};
use std::path::Path;

use tracing::{debug, trace, warn};

use crate::ebase::EBaseReader;

/// A parsed relation rule for $VarCond assignment
#[derive(Debug, Clone)]
pub struct VarCondRule {
    /// The var_cond value to assign if condition is true
    pub var_cond: String,
    /// The condition expression
    pub condition: Condition,
}

/// A condition expression
#[derive(Debug, Clone)]
pub enum Condition {
    /// Property equals value: `M_PROP = 'value'`
    Equals { property: String, value: String },
    /// Property not equals value: `M_PROP <> 'value'`
    NotEquals { property: String, value: String },
    /// Property in list: `M_PROP in ('v1','v2')`
    In { property: String, values: Vec<String> },
    /// Property not in list: `M_PROP not in ('v1','v2')`
    NotIn { property: String, values: Vec<String> },
    /// Logical AND of conditions
    And(Box<Condition>, Box<Condition>),
    /// Logical OR of conditions
    Or(Box<Condition>, Box<Condition>),
    /// Always true (for unconditional rules)
    True,
}

impl Condition {
    /// Evaluate condition against property values
    pub fn evaluate(&self, properties: &HashMap<String, String>) -> bool {
        match self {
            Condition::Equals { property, value } => {
                properties.get(property).map(|v| v == value).unwrap_or(false)
            }
            Condition::NotEquals { property, value } => {
                properties.get(property).map(|v| v != value).unwrap_or(true)
            }
            Condition::In { property, values } => {
                properties.get(property).map(|v| values.contains(v)).unwrap_or(false)
            }
            Condition::NotIn { property, values } => {
                properties.get(property).map(|v| !values.contains(v)).unwrap_or(true)
            }
            Condition::And(left, right) => {
                left.evaluate(properties) && right.evaluate(properties)
            }
            Condition::Or(left, right) => {
                left.evaluate(properties) || right.evaluate(properties)
            }
            Condition::True => true,
        }
    }
}

/// Relation rule reader and evaluator
pub struct RelationRuleReader {
    /// Parsed $VarCond assignment rules
    pub varcond_rules: Vec<VarCondRule>,
    /// Raw relation blocks (for debugging)
    pub raw_relations: Vec<(String, String)>, // (rel_name, rel_block)
}

impl RelationRuleReader {
    /// Load relation rules from an ebase file
    pub fn from_ebase(path: &Path) -> Option<Self> {
        let mut reader = EBaseReader::open(path).ok()?;

        // First, find which relation names have domain='P' (Pricing)
        let pricing_relation_names: HashSet<String> = reader
            .read_records("ocd_relationobj", None)
            .ok()?
            .iter()
            .filter_map(|record| {
                let rel_domain = record.get("rel_domain")?.as_str()?.to_string();
                if rel_domain.contains('P') {
                    Some(record.get("rel_name")?.as_str()?.to_string())
                } else {
                    None
                }
            })
            .collect();

        if pricing_relation_names.is_empty() {
            return None;
        }

        trace!(?pricing_relation_names, "Found pricing relations");

        // Load relation blocks for pricing relations
        let relations = reader.read_records("ocd_relation", None).ok()?;

        let mut varcond_rules = Vec::new();
        let mut raw_relations = Vec::new();

        for record in &relations {
            let rel_name = record.get("rel_name").and_then(|v| v.as_str()).unwrap_or("");
            let rel_block = record.get("rel_block").and_then(|v| v.as_str()).unwrap_or("");

            if !pricing_relation_names.contains(rel_name) {
                continue;
            }

            raw_relations.push((rel_name.to_string(), rel_block.to_string()));

            // Parse $VarCond assignment rules
            if let Some(rule) = parse_varcond_rule(rel_block) {
                debug!(
                    rel_name = rel_name,
                    var_cond = rule.var_cond,
                    "Parsed VarCond rule"
                );
                varcond_rules.push(rule);
            }
        }

        if varcond_rules.is_empty() && raw_relations.is_empty() {
            return None;
        }

        Some(Self {
            varcond_rules,
            raw_relations,
        })
    }

    /// Evaluate all rules and return matching var_cond values
    pub fn evaluate(&self, properties: &HashMap<String, String>) -> Vec<String> {
        self.varcond_rules
            .iter()
            .filter_map(|rule| {
                if rule.condition.evaluate(properties) {
                    trace!(
                        var_cond = rule.var_cond,
                        "VarCond rule matched"
                    );
                    Some(rule.var_cond.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if any pricing relations exist
    pub fn has_pricing_rules(&self) -> bool {
        !self.varcond_rules.is_empty()
    }
}

/// Parse a $VarCond assignment rule from a relation block
fn parse_varcond_rule(block: &str) -> Option<VarCondRule> {
    // Pattern: $VarCond = 'VALUE' if CONDITION
    let block = block.trim();

    if !block.starts_with("$VarCond") {
        return None;
    }

    // Split on " if " to separate assignment from condition
    let parts: Vec<&str> = block.splitn(2, " if ").collect();

    // Extract var_cond value from: $VarCond = 'VALUE'
    let assignment = parts.first()?;
    let var_cond = extract_quoted_value(assignment)?;

    // Parse condition if present
    let condition = if parts.len() > 1 {
        parse_condition(parts[1].trim())
    } else {
        Condition::True
    };

    Some(VarCondRule { var_cond, condition })
}

/// Extract a quoted value from text like "$VarCond = 'VALUE'"
fn extract_quoted_value(text: &str) -> Option<String> {
    let start = text.find('\'')?;
    let end = text.rfind('\'')?;
    if end > start {
        Some(text[start + 1..end].to_string())
    } else {
        None
    }
}

/// Parse a condition expression
///
/// Parses expressions like:
/// - `M_PROP = 'value'`
/// - `M_PROP <> 'value'`
/// - `M_PROP in ('v1','v2')`
/// - `M_PROP not in ('v1','v2')`
/// - `cond1 and cond2`
/// - `cond1 or cond2`
pub fn parse_condition(expr: &str) -> Condition {
    let expr = expr.trim();

    // Remove trailing comma if present (some rules end with a comma)
    let expr = expr.trim_end_matches(',');

    // Remove outer parentheses if they're balanced
    let expr = strip_outer_parens(expr);

    // Check for OR at the top level (lowest precedence)
    if let Some(pos) = find_operator_at_level(expr, " or ") {
        let left = parse_condition(&expr[..pos]);
        let right = parse_condition(&expr[pos + 4..]);
        return Condition::Or(Box::new(left), Box::new(right));
    }

    // Check for AND at the top level
    if let Some(pos) = find_operator_at_level(expr, " and ") {
        let left = parse_condition(&expr[..pos]);
        let right = parse_condition(&expr[pos + 5..]);
        return Condition::And(Box::new(left), Box::new(right));
    }

    // Parse simple comparisons
    if let Some(cond) = parse_not_in(expr) {
        return cond;
    }
    if let Some(cond) = parse_in(expr) {
        return cond;
    }
    if let Some(cond) = parse_not_equals(expr) {
        return cond;
    }
    if let Some(cond) = parse_equals(expr) {
        return cond;
    }

    // Fallback to true for unparseable conditions
    warn!(expr = expr, "Could not parse condition expression");
    Condition::True
}

/// Strip outer parentheses if they wrap the entire expression
fn strip_outer_parens(expr: &str) -> &str {
    let expr = expr.trim();
    if !expr.starts_with('(') || !expr.ends_with(')') {
        return expr;
    }

    // Check if the opening paren at the start matches the closing at the end
    let inner = &expr[1..expr.len()-1];
    let mut depth = 0;
    for (i, c) in inner.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                // If depth goes negative before the end, the parens don't match
                if depth < 0 && i < inner.len() - 1 {
                    return expr;
                }
            }
            _ => {}
        }
    }

    // If depth is 0 at the end, the outer parens match
    if depth == 0 {
        inner.trim()
    } else {
        expr
    }
}

/// Find operator at top level (not inside parentheses or quotes)
fn find_operator_at_level(expr: &str, op: &str) -> Option<usize> {
    let mut depth = 0;
    let mut in_quote = false;
    let expr_lower = expr.to_lowercase();
    let op_lower = op.to_lowercase();

    let bytes = expr_lower.as_bytes();
    let op_bytes = op_lower.as_bytes();
    let expr_bytes = expr.as_bytes(); // For checking quotes in original case

    for i in 0..bytes.len() {
        // Track quotes
        if expr_bytes[i] == b'\'' {
            in_quote = !in_quote;
            continue;
        }

        // Skip if inside quotes
        if in_quote {
            continue;
        }

        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && i + op_bytes.len() <= bytes.len() => {
                if &bytes[i..i + op_bytes.len()] == op_bytes {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Parse: M_PROP = 'value'
fn parse_equals(expr: &str) -> Option<Condition> {
    // Find = but not <> or in ()
    let parts: Vec<&str> = expr.splitn(2, " = ").collect();
    if parts.len() != 2 {
        // Try without spaces around =
        let parts: Vec<&str> = expr.splitn(2, '=').collect();
        if parts.len() != 2 {
            return None;
        }
        let property = parts[0].trim().to_string();
        let value = extract_quoted_value(parts[1].trim())?;
        return Some(Condition::Equals { property, value });
    }
    let property = parts[0].trim().to_string();
    let value = extract_quoted_value(parts[1].trim())?;
    Some(Condition::Equals { property, value })
}

/// Parse: M_PROP <> 'value'
fn parse_not_equals(expr: &str) -> Option<Condition> {
    let parts: Vec<&str> = expr.splitn(2, " <> ").collect();
    if parts.len() != 2 {
        // Try without spaces
        let parts: Vec<&str> = expr.splitn(2, "<>").collect();
        if parts.len() != 2 {
            return None;
        }
        let property = parts[0].trim().to_string();
        let value = extract_quoted_value(parts[1].trim())?;
        return Some(Condition::NotEquals { property, value });
    }
    let property = parts[0].trim().to_string();
    let value = extract_quoted_value(parts[1].trim())?;
    Some(Condition::NotEquals { property, value })
}

/// Parse: M_PROP in ('v1','v2')
fn parse_in(expr: &str) -> Option<Condition> {
    let expr_lower = expr.to_lowercase();
    let in_pos = expr_lower.find(" in ")?;

    let property = expr[..in_pos].trim().to_string();
    let values_part = expr[in_pos + 4..].trim();

    // Extract values from parentheses
    let values = extract_value_list(values_part)?;

    Some(Condition::In { property, values })
}

/// Parse: M_PROP not in ('v1','v2')
fn parse_not_in(expr: &str) -> Option<Condition> {
    let expr_lower = expr.to_lowercase();
    let not_in_pos = expr_lower.find(" not in ")?;

    let property = expr[..not_in_pos].trim().to_string();
    let values_part = expr[not_in_pos + 8..].trim();

    // Extract values from parentheses
    let values = extract_value_list(values_part)?;

    Some(Condition::NotIn { property, values })
}

/// Extract values from a parenthesized list like ('v1','v2','v3')
fn extract_value_list(text: &str) -> Option<Vec<String>> {
    let text = text.trim();
    if !text.starts_with('(') || !text.ends_with(')') {
        return None;
    }

    let inner = &text[1..text.len()-1];
    let values: Vec<String> = inner
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
                Some(s[1..s.len()-1].to_string())
            } else {
                None
            }
        })
        .collect();

    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_equals() {
        let rule = parse_varcond_rule("$VarCond = 'PG_LAN' if M_LAN = 'YES'").unwrap();
        assert_eq!(rule.var_cond, "PG_LAN");

        let mut props = HashMap::new();
        props.insert("M_LAN".to_string(), "YES".to_string());
        assert!(rule.condition.evaluate(&props));

        props.insert("M_LAN".to_string(), "NO".to_string());
        assert!(!rule.condition.evaluate(&props));
    }

    #[test]
    fn test_parse_and_condition() {
        let rule = parse_varcond_rule(
            "$VarCond = 'PG_ADJUSTABLE_SEAT' if (M_ARTNO = 'ONE' and M_SEAT = 'YES')"
        ).unwrap();
        assert_eq!(rule.var_cond, "PG_ADJUSTABLE_SEAT");

        let mut props = HashMap::new();
        props.insert("M_ARTNO".to_string(), "ONE".to_string());
        props.insert("M_SEAT".to_string(), "YES".to_string());
        assert!(rule.condition.evaluate(&props));

        // Missing one condition
        props.insert("M_SEAT".to_string(), "NO".to_string());
        assert!(!rule.condition.evaluate(&props));
    }

    #[test]
    fn test_parse_not_in() {
        let rule = parse_varcond_rule(
            "$VarCond = 'PG_EXTERIOR_PANEL_OPTION_COLOR' if (M_ARTNO = 'ONE' and M_EXTERIOR not in ('RAL9016MAT','S7500N','RAL9005'))"
        ).unwrap();
        assert_eq!(rule.var_cond, "PG_EXTERIOR_PANEL_OPTION_COLOR");

        let mut props = HashMap::new();
        props.insert("M_ARTNO".to_string(), "ONE".to_string());
        props.insert("M_EXTERIOR".to_string(), "CUSTOM_COLOR".to_string());
        assert!(rule.condition.evaluate(&props));

        // Standard color - should NOT match
        props.insert("M_EXTERIOR".to_string(), "RAL9005".to_string());
        assert!(!rule.condition.evaluate(&props));
    }

    #[test]
    fn test_parse_in_list() {
        let rule = parse_varcond_rule(
            "$VarCond = 'PG_KICKPLATE' if M_KICKPLATE = 'YES' and M_ARTNO in ('ONE','ONE_PREMIUM')"
        ).unwrap();

        let mut props = HashMap::new();
        props.insert("M_KICKPLATE".to_string(), "YES".to_string());
        props.insert("M_ARTNO".to_string(), "ONE".to_string());
        assert!(rule.condition.evaluate(&props));

        props.insert("M_ARTNO".to_string(), "ONE_LOUNGE".to_string());
        assert!(!rule.condition.evaluate(&props));
    }
}
