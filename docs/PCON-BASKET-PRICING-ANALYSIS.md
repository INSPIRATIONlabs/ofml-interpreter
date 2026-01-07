# pCon.basket OCD Pricing Analysis

This document provides a detailed technical analysis of how pCon.basket reads and processes OCD pricing data, based on reverse engineering the decompiled source code in `/reference/ConceptOffice7/sources/pcon-basket/`.

## Table of Contents

1. [Overview](#overview)
2. [Price Table Reading](#1-price-table-reading)
3. [Var_cond Resolution](#2-var_cond-resolution)
4. [Price Calculation Order](#3-price-calculation-order)
5. [Wildcard Handling](#4-wildcard-handling)
6. [Corrupted Data Handling](#5-corrupted-data-handling)
7. [Edge Cases](#6-edge-cases)
8. [Differences from pCon.configurator](#7-differences-from-pconconfigurator)
9. [Implementation Recommendations](#8-implementation-recommendations)

---

## Overview

pCon.basket uses a layered architecture for pricing:

```
Application Layer (basket.exe)
    |
    v
EAI Layer (ebasket.dll)
    |
    +-- PriceProfileManager - Manages profiles per manufacturer
    |       |
    |       +-- OFMLPriceProfile - OFML-specific pricing
    |
    +-- ECalcItem - Calculation line items
    |       |
    |       +-- updatePrice() - Update from catalog data
    |       +-- updatePriceModifier() - Apply surcharges/discounts
    |
    +-- ArticlePrice / ArticlePriceComponent - Final price structures
```

### Key Source Files

| File | Content |
|------|---------|
| `ebasket/decompiled.renamed.c` | Core pricing logic (~300K lines) |
| `basket/decompiled.renamed.c` | Application-level price handling |

### Key Classes (from decompiled code)

| Class | Location | Purpose |
|-------|----------|---------|
| `OFMLPriceProfile` | Line 232761 | OFML-specific pricing profile |
| `PriceProfileManager` | Line 250674 | Profile management per manufacturer |
| `ECalcItem` | Multiple | Calculation line item with 44+ constructors |
| `EPriceComponent` | Line 47917 | Individual price component |
| `ArticlePrice` | Line 40 | Complete price for an article |

---

## 1. Price Table Reading

### Data Source Hierarchy

pCon.basket supports multiple data sources, checked in this order:

```c
// From ba_OFMLPriceProfile.C (line 233885)
iVar11 = util_eAssert_469(param_1,"csv:");
if (((iVar11 < 0) && (iVar11 = util_eAssert_469(param_1,"ebase:"), iVar11 < 0)) &&
   (iVar11 = util_eAssert_469(param_1,"xbase:"), iVar11 < 0)) {
    // No usable OFML price profile data
}
```

**Priority Order:**
1. `csv:` - CSV format price data
2. `ebase:` - EBase proprietary format (pdata.ebase)
3. `xbase:` - dBASE/xBase format (.DBF files)

### OCD Price Table Schema

From `eproduct.c` (pCon.planner, line 163826-163991):

```c
// PriceTable structure - table name: "ocd_price"
memmove(param_1 + 6, "ocd_price", 9);

// Fields defined in order:
GDbField("article_nr", type=4, ...);    // String field
GDbField("var_cond", type=4, ...);      // String - 0x646e6f635f726176 = "var_cond"
GDbField("price_type", type=4, ...);    // String - "price_type"
GDbField("price_level", type=4, ...);   // String - "price_level"
GDbField("price_rule", type=4, ...);    // String
GDbField("price_textnr", type=4, ...);  // String - reference to pricetext
GDbField("price", type=3, ...);         // Numeric
GDbField("is_fix", type=2, ...);        // Boolean
GDbField("currency", type=4, ...);      // String
GDbField("date_from", type=4, ...);     // Date string
GDbField("date_to", type=4, ...);       // Date string

// OCD 3.0+ adds:
GDbField("scale_quantity", type=2, ...);  // Integer

// OCD 4.0+ adds:
GDbField("rounding_id", ...);             // String
```

### Reading Process

1. **Open Database**: Uses `GDbDatabase::openDatabase()` with prefix
2. **Access Table**: `GDbTable` abstraction for record operations
3. **Build Index**: Creates index on `(article_nr, price_level)` for fast lookup
4. **Cache Records**: Loads matching records into memory structures

```c
// Index structure for price table (line 163317)
*param_1 = egr::eai::eproduct::ocd::OCDGDbProductDb::IndexPriceTable::vftable;
```

---

## 2. Var_cond Resolution

### Matching Algorithm

The var_cond matching occurs in the `updatePriceModifier` method chain. Based on analysis of `egr::eai::util::ECalcItem::updatePriceModifier` calls:

```c
// From ba_ArticleCalculation.C (line 24291)
uVar6 = egr::eai::util::ECalcItem::updatePriceModifier(
    this_00,                              // ECalcItem
    *(undefined4 *)((int)param_1 + 0x10), // Application context
    pEVar7,                               // Price reference
    *(undefined4 *)((int)param_1 + 0x30), // Date value part 1
    *(undefined4 *)((int)param_1 + 0x34), // Date value part 2
    iVar4);                               // Rounding interface
```

### Var_cond Value Types

Based on OCD specification and decompiled code analysis:

| Type | Format | Description |
|------|--------|-------------|
| Empty | `""` | Applies to all configurations |
| Base Indicator | `S_PGX`, `BASE`, `STANDARD` | Marks base price |
| Surcharge Code | `S_XXX` | Manufacturer-specific surcharge |
| Formula | `KEY=value` | Direct property match |
| Compound | `KEY=val1;KEY2=val2` | Multiple conditions (AND) |
| Comparison | `KEY>value`, `KEY<value` | Numeric comparisons |

### Matching Strategy Order

From the OFML Interpreter implementation (verified against decompiled behavior):

1. **Direct Value Match**: `var_cond` equals a selected property value
2. **Formula Parsing**: Parse `KEY=value` and compare against configuration
3. **Sedus-Style Codes**: `S_XXX` patterns match property value suffixes/prefixes
4. **Price Group Codes**: `PG11`, `GL1`, `MG1` match if present in selections
5. **propvalue2varcond Lookup**: Direct mapping table (most accurate)

### propvalue2varcond Table

Some manufacturers provide explicit mappings:

```
Property Value -> var_cond
"166"          -> "S_166"
"PP AXA BLACK" -> "PP AXA BLACK"
```

This table provides 100% accurate matching when available.

---

## 3. Price Calculation Order

### Standard Calculation Flow

```
1. Load Base Price (price_level='B')
   |
   +-- Match article_nr (exact or wildcard '*')
   +-- Filter by date range (date_from <= price_date <= date_to)
   +-- Select by var_cond (S_PGX, BASE, STANDARD, or empty)
   |
   v
2. Apply Surcharges (price_level='X')
   |
   +-- For each X price:
   |     +-- Match var_cond against configuration
   |     +-- If match: add to surcharge list
   |
   v
3. Apply Discounts (price_level='D')
   |
   +-- For each D price:
   |     +-- Check discount conditions
   |     +-- Apply as percentage or fixed amount
   |
   v
4. Calculate Final Price
   |
   +-- base_price + sum(surcharges) - sum(discounts)
```

### Price Update Method Chain

From decompiled code:

```c
// 1. Update base price
egr::eai::util::ECalcItem::updatePrice(
    this, app, param1, param2, rounding);

// 2. Update modifiers (surcharges/discounts)
egr::eai::util::ECalcItem::updatePriceModifier(
    this_00, app, price_ref, date1, date2, rounding);

// 3. Calculate subtotal
egr::eai::util::ECalcItem::setSubtotal(
    subtotal_item, app, current_total, date1, date2,
    other_total, rounding);
```

### Incremental vs Absolute Calculation

```c
// From line 24286
bVar2 = egr::eai::util::ECalcItem::isIncremental(this_00);
pEVar7 = local_38;
if (!bVar2) {
    pEVar7 = (ECurrency *)((int)param_1 + 0x110);
}
```

- **Incremental**: Surcharge applied to running total
- **Absolute**: Surcharge applied to original base price

---

## 4. Wildcard Handling

### Article Number Wildcards

The price table supports `article_nr="*"` for prices that apply to all articles:

```c
// Matching priority:
1. Exact article_nr match
2. Wildcard '*' match (if no exact match found)
```

### Implementation Recommendation

Based on the OFML Interpreter's current handling:

```rust
// FIRST PASS: Look for exact article match with base price
for pdata_path in &pdata_files {
    let has_actual_base = reader.prices.iter()
        .any(|p| p.article_nr == family.base_article_nr && p.price_level == "B");

    if has_actual_base {
        // Use this file's prices
    }
}

// SECOND PASS: Fall back to wildcard prices or surcharge-only
for pdata_path in &pdata_files {
    // Check for article_nr="*" prices
}
```

### Wildcard Surcharges

Wildcard surcharges (`article_nr="*"`, `price_level="X"`) apply globally when their `var_cond` matches:

```
article_nr=*, var_cond="PG11", price_level="X", price=50.00
```

This adds 50.00 EUR surcharge whenever price group PG11 is selected, regardless of article.

---

## 5. Corrupted Data Handling

### Error Handling in Decompiled Code

The code uses extensive assertion and error checking:

```c
// Null pointer checks
if (param_1 == (int *)0x0) {
    egr::eai::util::_eAssert(app, "eai:ebasket", "mArticle != nullptr",
        "D:\\gef\\eai\\ebasket\\ba_ArticleCalculation.C", 0x91e);
    return -0x7fffff00;
}

// Value validation
if (*(char *)((int)param_1 + 0x1d4) != '\0') {
    egr::eai::util::_eAssert(app, "eai:ebasket", "!mPurchasePriceValid",
        "D:\\gef\\eai\\ebasket\\ba_ArticleCalculation.C", 0xf38);
}
```

### Recovery Strategies

1. **Missing Prices**: Returns 0 or skips calculation
2. **Invalid Date Ranges**: Logs warning, uses current date
3. **Malformed Records**: Skips record, continues processing
4. **Invalid Currency**: Falls back to default or skips

### Debug Logging

```c
// Debug message for price lookup
if (3 < DAT_18021d2bc) {  // Debug level check
    egr::eai::util::eDebug(app, "eai:eproduct",
        "lookup price table for article...");
}

// No match found message
egr::eai::util::eDebug(app, "eai:eproduct",
    "no matching entry found");
```

---

## 6. Edge Cases

### Missing Prices

When no base price is found:

```c
// From eproduct.c line 201629
if (lVar19 == 0) {  // No matching entry
    if (3 < DAT_18021d2bc) {
        // Log "no matching entry found"
    }
    // Price remains unset
}
```

**Behavior**: Article displays without price or shows "Price on request"

### Date Range Validation

```c
// From eproduct.c line 185949
"Ignoring invalid From date \"%4\" for value of property \"%1.%2\" in %3"
"Ignoring invalid To date \"%4\" for value of property \"%1.%2\" in %3"
```

**Behavior**: Invalid dates are ignored; record treated as always-valid

### Multiple Matching Surcharges

When multiple surcharges have the same var_cond:

1. **First Match Priority**: Earlier record in database order takes precedence
2. **Accumulation**: Different var_conds accumulate (sum all matching surcharges)
3. **Deduplication**: Same var_cond only counted once

### Currency Handling

```c
// Currency validation
ECurrency_isValid  // Validates currency values
ECurrency_round    // Rounds to currency precision (typically 2 decimals)
```

---

## 7. Differences from pCon.configurator

### Architecture Differences

| Aspect | pCon.basket | pCon.configurator |
|--------|-------------|-------------------|
| Focus | Order/basket management | Product configuration |
| Pricing | Final prices for orders | Preview prices during config |
| Data Source | Profile-based (manufacturer) | Direct OCD access |
| Caching | Heavy caching | Real-time lookup |

### Price Profile Manager

pCon.basket uses `IPriceProfileManager` / `IPriceProfileManager2` interfaces:

```c
// From basket/decompiled.renamed.c line 21589
"IDL:egr/eai/basket/IPriceProfileManager2:1.0"
"IDL:egr/eai/basket/IPriceProfileManager:1.0"
```

The configurator directly accesses OCD tables without the profile abstraction layer.

### Behavioral Differences

1. **Gross Price Warning**: pCon.basket warns when user-set gross purchase price is modified:
   ```
   "The explicitely specified gross purchase price has been modified by the
   price profile. The purchase price calculation may be incorrect."
   ```

2. **Price Date Handling**: Basket enforces strict date validation for orders

3. **Multi-Currency**: Basket has full currency conversion via `CurrencyConverter`

4. **Tax Integration**: Basket integrates with `TaxCalculator` for final prices

---

## 8. Implementation Recommendations

### For OFML Interpreter

Based on this analysis, the following improvements are recommended:

#### 1. Wildcard Priority

```rust
// Match prices with correct priority
fn match_prices(article_nr: &str, prices: &[OcdPrice]) -> Vec<&OcdPrice> {
    // First: exact article match
    let exact: Vec<_> = prices.iter()
        .filter(|p| p.article_nr == article_nr)
        .collect();

    if !exact.is_empty() {
        return exact;
    }

    // Second: wildcard match
    prices.iter()
        .filter(|p| p.article_nr == "*")
        .collect()
}
```

#### 2. Date Range Validation

```rust
fn is_price_valid(price: &OcdPrice, date: NaiveDate) -> bool {
    let from = parse_date(&price.date_from).unwrap_or(NaiveDate::MIN);
    let to = parse_date(&price.date_to).unwrap_or(NaiveDate::MAX);

    date >= from && date <= to
}
```

#### 3. Surcharge Deduplication

```rust
fn collect_surcharges(prices: &[&OcdPrice], config: &Config) -> Vec<Surcharge> {
    let mut seen = HashSet::new();
    let mut surcharges = Vec::new();

    for price in prices {
        if price.price_level != "X" { continue; }
        if seen.contains(&price.var_cond) { continue; }

        if matches_var_cond(&price.var_cond, config) {
            surcharges.push(/* ... */);
            seen.insert(price.var_cond.clone());
        }
    }

    surcharges
}
```

#### 4. Error Recovery

```rust
fn calculate_price_safe(/* ... */) -> Option<PriceResult> {
    // Log issues but continue processing
    if base_price.is_none() {
        log::warn!("No base price found for {}", article_nr);
        return None;  // Price on request
    }

    // Continue with available data
    Some(calculate_from_base(base_price.unwrap(), surcharges))
}
```

---

## References

### Decompiled Source Locations

| Function/Class | File | Line |
|---------------|------|------|
| OFMLPriceProfile_ctor | ebasket/decompiled.renamed.c | 232763 |
| PriceProfileManager_ctor | ebasket/decompiled.renamed.c | 250676 |
| ECalcItem::updatePrice | ebasket/decompiled.renamed.c | 22601 |
| ECalcItem::updatePriceModifier | ebasket/decompiled.renamed.c | 24291 |
| PriceTable schema | eproduct.c (pcon-planner) | 163826 |
| applyPriceProfiles | ebasket/decompiled.renamed.c | 252572 |

### Related Documentation

- `/workspace/docs/OCD-PRICING-IMPLEMENTATION.md` - Current implementation notes
- `/workspace/docs/ofml-specs/ocd_4_3.md` - OCD specification
- `/reference/ConceptOffice7/docs/pcon-basket/pricing-system.md` - AI-generated overview
