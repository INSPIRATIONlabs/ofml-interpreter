# Multi-Manufacturer Pricing Patterns Investigation Report

## Executive Summary

**CRITICAL FINDING**: **NO manufacturers use the `ocd_propvalue2varcond` table** in the investigated dataset.

This investigation analyzed pricing patterns across 11 different manufacturer/series combinations to identify the common structure that works for ALL manufacturers. The goal was to find patterns that would allow generic price calculation without manufacturer-specific workarounds.

## Key Discoveries

### 1. FRAMERY BASE PRICES FOUND!

**Framery pricing is CORRECT - phone booths cost tens of thousands of EUR:**
- `frmr_2q`: 42,070 - 44,140 EUR (2-person booths)
- `frmr_q`: 27,990 - 42,070 EUR (4-person booths)

**However, Framery ONE has a PROBLEM:**
- Has **ZERO base prices** (level 'B')
- Only has surcharges (level 'X')
- This means there's NO starting price - the product cannot be priced!

### 2. Base Price Patterns (level 'B', var_cond field)

Three distinct patterns emerged:

| Pattern | Manufacturers | var_cond Values | Interpretation |
|---------|--------------|-----------------|----------------|
| **Empty String** | Framery (frmr_2q, frmr_q), Bisley (sf, pf) | `""` | Simple base price, no conditions |
| **S_PGX** | Sedus (ai, sf) | `"S_PGX"` | "Standard Product Group X" indicator |
| **Complex Codes** | FAST (kr, wkm), Arper (catifa46, saya) | `"SG-KR-KM-D54-ES"`, `"0004W"`, etc. | Encoded configuration combinations |

### 3. Surcharge Patterns (level 'X', var_cond field)

| Manufacturer | Pattern | Example Codes | Strategy |
|-------------|---------|---------------|----------|
| **Framery** | `PG_*` prefix | `PG_CARPET_OPTION_COLOR`, `PG_WHITEBOARD_EXTERIORS` | Property group code |
| **Sedus** | `S_*` with numbers | `S_166`, `S_167`, `S_168`, `S_1513` | Numeric suffix matching |
| **Bisley** | `PG_*` or `DE_*` | `PG_AUFPREIS_FARBE_10`, `DE_ROLLADEN` | Property/region codes |
| **FAST** | NO SURCHARGES | (all prices are base 'B') | Everything in base price |
| **Arper** | Short codes | `FUST02E`, `IGN011`, `STR0301` | Abbreviation codes |

### 4. Property to var_cond Mapping (The Missing Link)

**CRITICAL**: The `ocd_propvalue2varcond` table does NOT exist for ANY manufacturer in this dataset.

This means we CANNOT rely on this table for generic var_cond lookup. Instead, we must:

1. **Parse property values for embedded codes**
2. **Match var_cond patterns against property values**
3. **Use manufacturer-specific heuristics**

## Pricing Patterns by Manufacturer

### Framery (Phone Booths)

```
frmr_2q:  2 base prices (var_cond=""), 7 surcharges (var_cond="PG_*")
frmr_q:   2 base prices (var_cond=""), 7 surcharges (var_cond="PG_*")
frmr_one: 0 base prices, 9 surcharges (var_cond="PG_*")  ⚠️ BROKEN
```

**Base Price**: Empty var_cond `""` indicates standard base price
**Surcharges**: `PG_` prefix indicates "Property Group" - must match property values

**Example**:
- Base: `article_nr='2Q_HUDDLE', var_cond='', price=44140 EUR`
- Surcharge: `article_nr='*', var_cond='PG_WHITEBOARD_EXTERIORS', price=1050 EUR`

### FAST (Wall Decorations)

```
kr:  8 base prices (complex var_cond), 0 surcharges
wkm: 21 base prices (complex var_cond), 0 surcharges
```

**Pattern**: ALL pricing is in base prices (level 'B'). NO surcharges.
**var_cond codes**: Encode the complete configuration
- Format: `SG-{SERIES}-{MODEL}-{SIZE}-{FINISH}`
- Example: `SG-KR-KM-D54-ES` = "SoundGrid, KR series, KM model, D54 size, ES finish"

**Strategy**: Must build the complete var_cond string from property selections

### Sedus (Office Chairs)

```
ai: 8 base prices (var_cond="S_PGX"), 52 surcharges (var_cond="S_###")
sf: 3 base prices (var_cond="S_PGX"), 0 surcharges
```

**Base Price**: `S_PGX` indicates "Sedus Product Group X" base
**Surcharges**: Numeric codes like `S_166`, `S_1513`

**Strategy**:
1. Base price: Match `var_cond = "S_PGX"`
2. Surcharges: Extract number from property value, match against `S_` codes

**Example**:
- Property value: `"166"` → matches surcharge `var_cond='S_166'`
- Property value: `"1513"` → matches surcharge `var_cond='S_1513'`

### Bisley (File Cabinets)

```
sf: 21 base prices (var_cond=""), 1 surcharge (var_cond="DE_ROLLADEN")
pf: 7 base prices (var_cond=""), 2 surcharges (var_cond="PG_AUFPREIS_FARBE_*")
```

**Pattern**: Similar to Framery - empty base, descriptive surcharges

### Arper (Furniture)

```
catifa46: 1525 base prices (complex codes), 145 surcharges (short codes)
saya:     75 base prices (simple codes), 2 surcharges
```

**Base Price Codes**: Highly complex system with hundreds of var_cond values
- Short codes: `"00"`, `"01"`, `"02"`, ..., `"12"`
- Extended: `"00CT"`, `"08PB"`, `"0804W"`, etc.

**This is the MOST COMPLEX pricing structure** - likely encodes:
- Finish codes
- Material codes
- Configuration combinations

## The Common Denominator

After analyzing all manufacturers, here's what works universally:

### ✅ UNIVERSAL TRUTHS

1. **Level Field is Sacred**:
   - `level = 'B'` → Base price
   - `level = 'X'` → Surcharge
   - This is ALWAYS reliable

2. **Empty var_cond for Simple Base Prices**:
   - Manufacturers: Framery, Bisley
   - When `var_cond = ""` and `level = 'B'`, it's a simple base price

3. **Wildcard article_nr for Global Surcharges**:
   - Manufacturers: Framery, Bisley
   - `article_nr = '*'` means surcharge applies to ALL articles

4. **var_cond Contains the Lookup Key**:
   - For base prices: May be `""`, `"S_PGX"`, or an encoded configuration
   - For surcharges: Must match against property selections

### ❌ UNRELIABLE PATTERNS

1. **ocd_propvalue2varcond table**: Does NOT exist in this dataset
2. **Consistent var_cond format**: Varies wildly by manufacturer
3. **Surcharge code format**: No standard (PG_, S_, DE_, etc.)

## Recommended Implementation Strategy

### 1. Base Price Lookup (in priority order)

```rust
fn find_base_price(article_nr: &str, properties: &Properties) -> Option<f32> {
    // Strategy 1: Empty var_cond (Framery, Bisley)
    if let Some(price) = find_price(article_nr, level='B', var_cond="") {
        return Some(price);
    }

    // Strategy 2: Standard product group (Sedus)
    if let Some(price) = find_price(article_nr, level='B', var_cond="S_PGX") {
        return Some(price);
    }

    // Strategy 3: Build var_cond from properties (FAST, Arper)
    let var_cond = build_varcond_from_properties(properties);
    if let Some(price) = find_price(article_nr, level='B', var_cond=&var_cond) {
        return Some(price);
    }

    // Strategy 4: Try all base prices for this article and pick first
    find_first_base_price(article_nr)
}
```

### 2. Surcharge Matching (by pattern recognition)

```rust
fn find_applicable_surcharges(article_nr: &str, properties: &Properties) -> Vec<f32> {
    let mut surcharges = vec![];
    let all_surcharges = get_surcharges(article_nr);  // Includes wildcard '*'

    for surcharge in all_surcharges {
        if matches_property_selection(&surcharge.var_cond, properties) {
            surcharges.push(surcharge.price);
        }
    }

    surcharges
}

fn matches_property_selection(var_cond: &str, properties: &Properties) -> bool {
    for (prop, value) in properties {
        // Strategy 1: Direct match (Framery)
        if var_cond == format!("PG_{}", value) {
            return true;
        }

        // Strategy 2: Numeric suffix (Sedus)
        if var_cond.starts_with("S_") {
            let num = var_cond.strip_prefix("S_").unwrap();
            if value.contains(num) || value.ends_with(num) {
                return true;
            }
        }

        // Strategy 3: Exact code match (Arper)
        if var_cond == value {
            return true;
        }
    }

    false
}
```

### 3. Special Case: FAST-style Complete var_cond

For manufacturers that encode full configuration in var_cond:

```rust
fn build_varcond_from_properties(properties: &Properties) -> String {
    // Extract components from properties
    let series = properties.get("series");
    let model = properties.get("model");
    let size = properties.get("size");
    let finish = properties.get("finish");

    // Build composite code
    format!("SG-{}-{}-{}-{}", series, model, size, finish)
}
```

## Data Completeness Summary

| Manufacturer | Series | Articles | Base Prices | Surcharges | Prop Classes |
|-------------|---------|----------|-------------|------------|--------------|
| framery | frmr_one | 3 | **0** ⚠️ | 9 | 3 |
| framery | frmr_2q | 3 | 2 | 7 | 3 |
| framery | frmr_q | 4 | 2 | 7 | 4 |
| fast | kr | 1 | 8 | 0 | 1 |
| fast | wkm | 1 | 21 | 0 | 1 |
| sedus | ai | 2 | 8 | 52 | 2 |
| sedus | sf | 1 | 3 | 0 | 1 |
| bisley | sf | 101 | 21 | 1 | 51 |
| bisley | pf | 11 | 7 | 2 | 9 |
| arper | catifa46 | 62 | 1525 | 145 | 31 |
| arper | saya | 10 | 75 | 2 | 3 |

## Critical Issues Found

### Issue 1: Framery ONE Missing Base Prices

**Problem**: `framery/frmr_one` has ZERO base prices, only surcharges.

**Impact**: Cannot calculate total price - no starting point.

**Resolution Needed**:
1. Investigate CLS files for price calculation logic
2. Check if base price is hardcoded in code
3. Verify if this is a data error or intentional design

### Issue 2: No propvalue2varcond Table

**Problem**: None of the manufacturers provide the `ocd_propvalue2varcond` mapping table.

**Impact**: Must use heuristic pattern matching instead of direct lookup.

**Current Workaround**: Implement pattern matching as described in strategies above.

## Conclusions

1. **There is NO single universal pattern** for var_cond matching across all manufacturers
2. **The OCD 4.3 spec is correct** about using `level` field ('B'/'X'/'D')
3. **propvalue2varcond table is optional** - manufacturers don't consistently use it
4. **Pattern recognition is required** - each manufacturer has their own var_cond encoding scheme
5. **Framery's high prices are CORRECT** - 27k-44k EUR for phone booths is accurate

## Recommended Next Steps

1. **Fix Framery ONE**: Investigate why it has no base prices
2. **Implement tiered matching**: Try multiple strategies in order
3. **Add manufacturer detection**: Different logic for different var_cond patterns
4. **Test thoroughly**: Verify price calculations match expected values
5. **Document patterns**: Create manufacturer-specific pricing guides

## Files Generated

- `/workspace/COMPREHENSIVE_PRICING_INVESTIGATION.md` - Full output
- `/workspace/multi_mfr_pricing_analysis.rs` - Analysis source code
- `/workspace/MULTI_MANUFACTURER_PRICING_PATTERNS_REPORT.md` - This report
