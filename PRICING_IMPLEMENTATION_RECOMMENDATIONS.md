# OFML Pricing Implementation Recommendations

## Based on Multi-Manufacturer Investigation (2026-01-02)

This document provides concrete implementation recommendations based on analysis of 11 manufacturer/series combinations across 5 different manufacturers.

---

## Executive Summary

**Key Finding**: There is **NO universal var_cond pattern** across manufacturers. Each uses their own encoding scheme.

**Solution**: Implement a **tiered matching strategy** that tries multiple approaches in order of likelihood.

**Critical Issues**:
1. Framery ONE has ZERO base prices (data error?)
2. NO manufacturers use `ocd_propvalue2varcond` table in our dataset
3. var_cond formats vary wildly: `""`, `"S_PGX"`, `"PG_*"`, `"SG-*-*-*-*"`, numeric codes

---

## Implementation Strategy

### Phase 1: Base Price Lookup (REQUIRED)

```rust
pub fn find_base_price(
    ocd: &OcdReader,
    article_nr: &str,
    properties: &HashMap<String, String>
) -> Result<f32, PriceError> {
    // Get all base prices for this article
    let base_prices: Vec<&OcdPrice> = ocd.prices
        .iter()
        .filter(|p| p.price_level == "B" &&
                   (p.article_nr == article_nr || p.article_nr == "*"))
        .collect();

    if base_prices.is_empty() {
        return Err(PriceError::NoBasePriceFound(article_nr.to_string()));
    }

    // STRATEGY 1: Empty var_cond (Framery, Bisley, Arper)
    if let Some(price) = base_prices.iter()
        .find(|p| p.var_cond.is_empty())
        .map(|p| p.price)
    {
        return Ok(price);
    }

    // STRATEGY 2: S_PGX indicator (Sedus)
    if let Some(price) = base_prices.iter()
        .find(|p| p.var_cond == "S_PGX")
        .map(|p| p.price)
    {
        return Ok(price);
    }

    // STRATEGY 3: Build var_cond from properties (FAST)
    let constructed_varcond = construct_varcond_from_properties(properties);
    if let Some(price) = base_prices.iter()
        .find(|p| p.var_cond == constructed_varcond)
        .map(|p| p.price)
    {
        return Ok(price);
    }

    // STRATEGY 4: Match against property values (Arper complex codes)
    for base_price in &base_prices {
        if properties.values().any(|v| v == &base_price.var_cond) {
            return Ok(base_price.price);
        }
    }

    // FALLBACK: Use first available base price
    Ok(base_prices[0].price)
}
```

### Phase 2: Surcharge Matching (REQUIRED)

```rust
pub fn find_applicable_surcharges(
    ocd: &OcdReader,
    article_nr: &str,
    properties: &HashMap<String, String>
) -> Vec<f32> {
    let mut applicable_surcharges = Vec::new();

    // Get all surcharges for this article (including wildcard '*')
    let all_surcharges: Vec<&OcdPrice> = ocd.prices
        .iter()
        .filter(|p| p.price_level == "X" &&
                   (p.article_nr == article_nr || p.article_nr == "*"))
        .collect();

    for surcharge in all_surcharges {
        if is_surcharge_applicable(&surcharge.var_cond, properties) {
            applicable_surcharges.push(surcharge.price);
        }
    }

    applicable_surcharges
}

fn is_surcharge_applicable(
    var_cond: &str,
    properties: &HashMap<String, String>
) -> bool {
    // PATTERN 1: PG_ prefix (Framery, Bisley)
    if var_cond.starts_with("PG_") {
        let code = var_cond.strip_prefix("PG_").unwrap();
        // Check if any property value matches the code
        return properties.values().any(|v|
            v == code ||
            v.ends_with(code) ||
            v.contains(&format!("_{}", code))
        );
    }

    // PATTERN 2: S_ prefix with numbers (Sedus)
    if var_cond.starts_with("S_") {
        let code = var_cond.strip_prefix("S_").unwrap();
        // Extract numeric portion
        return properties.values().any(|v|
            v == code ||           // Exact match
            v.ends_with(code) ||   // Suffix match
            v.starts_with(code)    // Prefix match
        );
    }

    // PATTERN 3: DE_ or regional prefix (Bisley)
    if var_cond.starts_with("DE_") || var_cond.starts_with("GB_") {
        let code = var_cond.split('_').nth(1).unwrap_or("");
        return properties.values().any(|v| v.contains(code));
    }

    // PATTERN 4: Direct code match (Arper)
    // Check if var_cond appears as a property value
    properties.values().any(|v| v == var_cond)
}
```

### Phase 3: Total Price Calculation

```rust
pub fn calculate_total_price(
    ocd: &OcdReader,
    article_nr: &str,
    properties: &HashMap<String, String>
) -> Result<f32, PriceError> {
    // 1. Get base price
    let base_price = find_base_price(ocd, article_nr, properties)?;

    // 2. Find applicable surcharges
    let surcharges = find_applicable_surcharges(ocd, article_nr, properties);

    // 3. Sum it up
    let total = base_price + surcharges.iter().sum::<f32>();

    Ok(total)
}
```

---

## Manufacturer-Specific Notes

### Framery (Phone Booths)

**Pattern**:
- Base: `var_cond = ""`
- Surcharges: `var_cond = "PG_*"`

**Implementation**:
```rust
// Base price: Always empty var_cond
let base = find_price(level='B', var_cond="");

// Surcharges: Match PG_ prefix against property selections
// Example: Property "CARPET_OPTION_COLOR" selected
//          → Matches surcharge "PG_CARPET_OPTION_COLOR"
```

**CRITICAL ISSUE**: `frmr_one` has NO base prices! Must investigate:
1. Check CLS files for hardcoded price
2. Verify this isn't a data export error
3. Check if there's a price in a different table

### FAST (Wall Decorations)

**Pattern**:
- Base: `var_cond = "SG-{SERIES}-{MODEL}-{SIZE}-{FINISH}"`
- Surcharges: NONE (all prices are base level)

**Implementation**:
```rust
// Build complete var_cond from property selections
fn build_fast_varcond(props: &HashMap<String, String>) -> String {
    let series = props.get("series").unwrap_or("KR");
    let model = props.get("model").unwrap_or("KM");
    let size = props.get("size").unwrap_or("D54");
    let finish = props.get("finish").unwrap_or("ES");

    format!("SG-{}-{}-{}-{}", series, model, size, finish)
}

let varcond = build_fast_varcond(&properties);
let price = find_price(level='B', var_cond=varcond);
```

### Sedus (Office Chairs)

**Pattern**:
- Base: `var_cond = "S_PGX"`
- Surcharges: `var_cond = "S_{NUMBER}"`

**Implementation**:
```rust
// Base price: Always "S_PGX"
let base = find_price(level='B', var_cond="S_PGX");

// Surcharges: Extract number from property value
// Example: Property "upholstery" = "1513"
//          → Matches surcharge "S_1513"
//
// Example: Property "armrest" = "166"
//          → Matches surcharge "S_166"

for (prop_name, prop_value) in properties {
    let varcond = format!("S_{}", prop_value);
    if let Some(surcharge) = find_price(level='X', var_cond=varcond) {
        total += surcharge;
    }
}
```

### Bisley (File Cabinets)

**Pattern**:
- Base: `var_cond = ""`
- Surcharges: `var_cond = "PG_*"` or `"DE_*"`

**Implementation**:
```rust
// Similar to Framery
let base = find_price(level='B', var_cond="");

// Surcharges with region prefix
// Example: "DE_ROLLADEN" = German shutter option
```

### Arper (Furniture)

**Pattern**:
- Base: Complex codes (`"00"`, `"01"`, `"0804W"`, etc.)
- Surcharges: Abbreviation codes (`"FUST02E"`, `"IGN011"`, etc.)

**Implementation**:
```rust
// Most complex! Requires understanding Arper's encoding:
// - "00" = base configuration
// - "08" = finish option 8
// - "PB" = plywood base
// - "0804W" = finish 08, variant 04, wood

// Strategy: Match var_cond against property values
for base_price in base_prices {
    if properties.values().any(|v| v == base_price.var_cond) {
        return base_price.price;
    }
}
```

---

## Testing Strategy

### Test Case 1: Framery 2Q (Simple)

```rust
#[test]
fn test_framery_2q_pricing() {
    let ocd = OcdReader::from_ebase("framery/frmr_2q/ANY/1/db/pdata.ebase").unwrap();

    // Base configuration (no options)
    let props = HashMap::new();
    let price = calculate_total_price(&ocd, "2Q_HUDDLE", &props).unwrap();
    assert_eq!(price, 44140.0);

    // With whiteboard option
    let mut props = HashMap::new();
    props.insert("exterior".to_string(), "WHITEBOARD_EXTERIORS".to_string());
    let price = calculate_total_price(&ocd, "2Q_HUDDLE", &props).unwrap();
    assert_eq!(price, 44140.0 + 1050.0);  // Base + whiteboard
}
```

### Test Case 2: Sedus AI (Numeric Matching)

```rust
#[test]
fn test_sedus_ai_pricing() {
    let ocd = OcdReader::from_ebase("sedus/ai/DE/1/db/pdata.ebase").unwrap();

    // Base chair
    let props = HashMap::new();
    let price = calculate_total_price(&ocd, "AI-121", &props).unwrap();
    assert_eq!(price, 599.0);  // S_PGX base

    // With upholstery option "1513"
    let mut props = HashMap::new();
    props.insert("upholstery".to_string(), "1513".to_string());
    let price = calculate_total_price(&ocd, "AI-121", &props).unwrap();
    assert_eq!(price, 599.0 + 228.0);  // Base + S_1513 surcharge
}
```

### Test Case 3: FAST KR (Complete var_cond)

```rust
#[test]
fn test_fast_kr_pricing() {
    let ocd = OcdReader::from_ebase("fast/kr/DE/1/db/pdata.ebase").unwrap();

    // Must build complete var_cond
    let mut props = HashMap::new();
    props.insert("series".to_string(), "KR".to_string());
    props.insert("model".to_string(), "KM".to_string());
    props.insert("size".to_string(), "D54".to_string());
    props.insert("finish".to_string(), "ES".to_string());

    let price = calculate_total_price(&ocd, "sG-Kr", &props).unwrap();
    assert_eq!(price, 385.7143);  // SG-KR-KM-D54-ES
}
```

---

## Error Handling

```rust
#[derive(Debug)]
pub enum PriceError {
    NoBasePriceFound(String),           // Article has no base price
    AmbiguousBasePrice(String, usize),  // Multiple base prices, unclear which to use
    InvalidConfiguration(String),        // Properties don't match any valid price
    DatabaseError(String),              // EBase read error
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PriceError::NoBasePriceFound(article) =>
                write!(f, "No base price found for article: {}", article),
            PriceError::AmbiguousBasePrice(article, count) =>
                write!(f, "Article {} has {} base prices, cannot determine correct one", article, count),
            PriceError::InvalidConfiguration(msg) =>
                write!(f, "Invalid configuration: {}", msg),
            PriceError::DatabaseError(msg) =>
                write!(f, "Database error: {}", msg),
        }
    }
}
```

---

## Data Quality Checks

Add validation to detect issues early:

```rust
pub fn validate_pricing_data(ocd: &OcdReader) -> Vec<String> {
    let mut warnings = Vec::new();

    // Check 1: Articles with no base prices
    for article in &ocd.articles {
        let base_count = ocd.prices.iter()
            .filter(|p| p.article_nr == article.article_nr && p.price_level == "B")
            .count();

        if base_count == 0 {
            warnings.push(format!(
                "WARNING: Article '{}' has no base prices",
                article.article_nr
            ));
        }
    }

    // Check 2: Surcharges with no matching base
    let has_wildcard_base = ocd.prices.iter()
        .any(|p| p.article_nr == "*" && p.price_level == "B");

    if !has_wildcard_base {
        let wildcard_surcharges = ocd.prices.iter()
            .filter(|p| p.article_nr == "*" && p.price_level == "X")
            .count();

        if wildcard_surcharges > 0 {
            warnings.push(format!(
                "WARNING: {} wildcard surcharges but no wildcard base price",
                wildcard_surcharges
            ));
        }
    }

    // Check 3: Unusual price values
    for price in &ocd.prices {
        if price.price < 0.0 {
            warnings.push(format!(
                "WARNING: Negative price {} for article '{}'",
                price.price, price.article_nr
            ));
        }
        if price.price > 100000.0 {
            warnings.push(format!(
                "INFO: Very high price {} for article '{}' (may be correct for large items)",
                price.price, price.article_nr
            ));
        }
    }

    warnings
}
```

---

## Next Steps

1. **Implement tiered base price lookup** with fallback strategies
2. **Implement pattern-based surcharge matching** for known prefixes
3. **Add comprehensive tests** for each manufacturer
4. **Add data validation** to catch issues like Framery ONE
5. **Document manufacturer patterns** in code comments
6. **Consider adding manufacturer detection** to optimize matching
7. **Investigate Framery ONE base price** - check CLS files or contact data provider

---

## Files Reference

- Analysis script: `/workspace/multi_mfr_pricing_analysis.rs`
- Full report: `/workspace/MULTI_MANUFACTURER_PRICING_PATTERNS_REPORT.md`
- Raw output: `/workspace/COMPREHENSIVE_PRICING_INVESTIGATION.md`
- This document: `/workspace/PRICING_IMPLEMENTATION_RECOMMENDATIONS.md`
