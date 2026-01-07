# OFML Pricing Guide

This document explains how pricing works in the OFML Interpreter, including price calculation algorithms, variant condition matching, and pricing strategies used by different manufacturers.

## Table of Contents

- [Overview](#overview)
- [Price Calculation Flow](#price-calculation-flow)
- [Price Components](#price-components)
- [Variant Condition Matching](#variant-condition-matching)
- [Pricing Strategies](#pricing-strategies)
- [Using the Price System](#using-the-price-system)
- [Troubleshooting](#troubleshooting)

## Overview

The OFML pricing system is based on the OCD (OFML Commercial Data) 4.3 specification. Prices are stored in EBase databases and consist of:

1. **Base prices** (Level 'B') - The fundamental price for an article
2. **Surcharges** (Level 'X') - Additional charges based on configuration options
3. **Discounts** (Level 'D') - Reductions based on conditions
4. **Taxes** - Applied to the net price (optional)

### Price Formula

```
Net Price = Base Price + Σ(Surcharges) - Σ(Discounts)
Total Price = Net Price + Σ(Taxes)
```

### Key Concepts

**Article Number**: Base product identifier (e.g., "AI-121")

**Variant Code**: Generated from property selections, identifies specific configuration:
```
S_STOFF=2G3;S_SITZHOEHE=1701;S_LEHNE_ABW=0000
```

**var_cond**: Variant condition code in price table that determines when a price applies

**Price Level**: 'B' (base), 'X' (extra/surcharge), 'D' (discount)

**Price Type**: 'S' (sales/customer price), 'P' (purchase/cost price)

## Price Calculation Flow

### 1. Initialize Configuration Engine

```rust
use ofml_lib::oap::engine::ConfigurationEngine;

let mut engine = ConfigurationEngine::new("/reference/ofmldata");
```

### 2. Load Product and Properties

```rust
let families = engine.load_families("sedus");
let family = &families[0];
let properties = engine.get_family_properties("sedus", &family.id);
```

### 3. Create Configuration

```rust
use ofml_lib::oap::families::FamilyConfiguration;

let mut config = FamilyConfiguration::new(&family.id, &properties);

// Set property values
config.set_property("S_STOFF", "2G3");
config.set_property("S_SITZHOEHE", "1701");
```

### 4. Calculate Price

```rust
use chrono::Local;

let price = engine.calculate_family_price(
    "sedus",
    family,
    &config,
    Local::now().date_naive(),
);

if let Some(price_result) = price {
    println!("Base: {} {}", price_result.base_price, price_result.currency);
    println!("Total: {} {}", price_result.total_price, price_result.currency);
}
```

### Internal Flow

```
1. Generate variant code from property selections
   ↓
2. Two-pass price lookup:
   - Pass 1: Look up prices for exact article number
   - Pass 2: Fall back to wildcard article ("*")
   ↓
3. Filter prices by date validity
   ↓
4. Match base price (level 'B'):
   - Try matching variant code
   - Fall back to base indicators (S_PGX, BASE, STANDARD, "")
   ↓
5. Match surcharges (level 'X'):
   - Apply var_cond matching algorithm
   - Accumulate all matching surcharges
   ↓
6. Match discounts (level 'D'):
   - Apply var_cond matching algorithm
   - Treat as negative surcharges
   ↓
7. Calculate totals:
   - Net = Base + Surcharges - Discounts
   - Total = Net + Taxes
```

## Price Components

### Base Price (Level 'B')

The fundamental product price. Each article typically has one base price entry.

**Example from ocd_price table**:

| article_nr | var_cond | price_level | price | currency |
|------------|----------|-------------|-------|----------|
| AI-121 | S_PGX | B | 599.00 | EUR |

**var_cond indicators for base prices**:
- `S_PGX` - Sedus product group code
- `BASE` - Generic base indicator
- `STANDARD` - Standard configuration
- `` (empty) - Default base price

### Surcharges (Level 'X')

Additional charges for specific options or features.

**Example from ocd_price table**:

| article_nr | var_cond | price_level | price | currency | price_textnr |
|------------|----------|-------------|-------|----------|--------------|
| AI-121 | S_166 | X | 44.00 | EUR | 10234 |
| AI-121 | S_1513 | X | 228.00 | EUR | 10235 |
| AI-121 | S_1801 | X | 21.00 | EUR | 10236 |

**Surcharge meanings** (from ocd_pricetext):

| price_textnr | text_de | text_en |
|--------------|---------|---------|
| 10234 | Modellfarbe Rubinrot | Model color ruby red |
| 10235 | Counterausführung | Counter design |
| 10236 | Lordosenhöhenverstellung | Lumbar height adjustment |

### Discounts (Level 'D')

Reductions applied to the price based on conditions.

**Example from ocd_price table**:

| article_nr | var_cond | price_level | price | is_fix | currency |
|------------|----------|-------------|-------|--------|----------|
| AI-121 | VOLUME_10 | D | 50.00 | 1 | EUR |
| AI-121 | DEALER_DISC | D | 15.00 | 0 | EUR |

- `is_fix=1`: Fixed amount discount (50 EUR off)
- `is_fix=0`: Percentage discount (15% off)

### Taxes

Taxes are calculated on the net price and added to produce the final total.

```rust
use ofml_lib::oap::PriceResult;
use rust_decimal::Decimal;

let tax_schemes = vec![
    ("MwSt".to_string(), "standard".to_string(), Decimal::new(19, 0)), // 19% VAT
];

let taxes = PriceResult::calculate_taxes_from_schemes(net_price, &tax_schemes);
```

## Variant Condition Matching

The `var_cond` field in the price table determines when a surcharge applies. The library uses multiple matching strategies.

### Strategy 1: Sedus-Style Surcharge Codes

For codes like `S_XXXX`:

```rust
// var_cond: "S_166"
// Matches if variant_code contains "166"

variant_code: "S_STOFF=2G3;S_FARBE=166"  // MATCH
variant_code: "S_STOFF=2G3;S_FARBE=167"  // NO MATCH
```

**Algorithm**:
1. Strip "S_" prefix from var_cond → "166"
2. Split variant_code by "_" and ";" separators
3. Check if any component equals or ends with "166"

### Strategy 2: Exact Component Match

```rust
// var_cond: "H720"
// Matches if variant_code has exact "H720" component

variant_code: "H720_D1200"  // MATCH
variant_code: "H721_D1200"  // NO MATCH
```

### Strategy 3: Contains Match

```rust
// var_cond: "720"
// Matches if variant_code contains "720"

variant_code: "H720_D1200"  // MATCH
variant_code: "H800_D1200"  // NO MATCH
```

### Strategy 4: Formula-Based Matching

Some manufacturers use formula-style var_cond:

```rust
// var_cond: "H=720;D=1200"
// Matches property values exactly

variant_code: "H=720;D=1200;W=800"  // MATCH
variant_code: "H=800;D=1200;W=800"  // NO MATCH
```

### Base Price Matching

Base prices use special indicators that should NOT match as surcharges:

```rust
const BASE_INDICATORS: &[&str] = &["S_PGX", "BASE", "STANDARD", ""];

// These var_cond values identify base prices, not surcharges
```

### Implementation

```rust
fn var_cond_matches(var_cond: &str, variant_code: &str) -> bool {
    // Empty var_cond never matches
    if var_cond.is_empty() {
        return false;
    }

    // Base indicators should not match as surcharges
    const BASE_INDICATORS: &[&str] = &["S_PGX", "BASE", "STANDARD"];
    for indicator in BASE_INDICATORS {
        if var_cond.eq_ignore_ascii_case(indicator) {
            return false;
        }
    }

    // Strategy 1: S_ prefix matching
    if let Some(suffix) = var_cond.strip_prefix("S_") {
        for component in variant_code.split('_') {
            if component == suffix || component.ends_with(suffix) {
                return true;
            }
        }
    }

    // Strategy 2: Exact component match
    for component in variant_code.split('_') {
        if component == var_cond {
            return true;
        }
    }

    // Strategy 3: Contains match
    if variant_code.contains(var_cond) {
        return true;
    }

    false
}
```

## Pricing Strategies

Different manufacturers use different pricing approaches:

### 1. EmptyBase (Framery, Bisley)

- Base price has empty var_cond: `var_cond=""`
- All options are surcharges with descriptive var_cond codes
- Total = Sum of all matching surcharges

**Example**:
```
Base:     var_cond=""           price=0.00    (or small base)
Option 1: var_cond="DOOR_GLASS" price=500.00
Option 2: var_cond="LIGHT_LED"  price=200.00
Total = 0 + 500 + 200 = 700
```

### 2. ProductGroup (Sedus)

- Base price uses product group code: `var_cond="S_PGX"`
- Surcharges use S_ prefixed codes: `var_cond="S_166"`
- Clear separation between base and options

**Example**:
```
Base:     var_cond="S_PGX"  price=599.00
Fabric:   var_cond="S_166"  price=44.00
Height:   var_cond="S_1801" price=21.00
Total = 599 + 44 + 21 = 664
```

### 3. TableComputed (FAST)

- Base price computed from TABLE relation
- var_cond generated from property combinations
- Complex conditional logic

**Example**:
```
Base:     var_cond="H=720;D=1200" price=450.00
Finish:   var_cond="F=CHROME"     price=50.00
Total = 450 + 50 = 500
```

### 4. SurchargeOnly

- No base price (base=0)
- All pricing through surcharges
- Common for highly configurable products

**Example**:
```
Base:     var_cond="BASE"        price=0.00
Frame:    var_cond="FRAME_STEEL" price=300.00
Top:      var_cond="TOP_WOOD"    price=200.00
Finish:   var_cond="FINISH_OAK"  price=100.00
Total = 0 + 300 + 200 + 100 = 600
```

### 5. ComplexCode

- Encoded var_cond with manufacturer-specific format
- May require lookup tables or rule evaluation
- Used by some legacy systems

## Using the Price System

### Example 1: Basic Price Lookup

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::FamilyConfiguration;
use chrono::Local;

fn get_product_price() {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families("sedus");

    if let Some(family) = families.first() {
        let properties = engine.get_family_properties("sedus", &family.id);
        let config = FamilyConfiguration::new(&family.id, &properties);

        let price = engine.calculate_family_price(
            "sedus",
            family,
            &config,
            Local::now().date_naive(),
        );

        if let Some(result) = price {
            println!("Article: {}", family.base_article_nr);
            println!("Base price: {} {}", result.base_price, result.currency);

            for surcharge in &result.surcharges {
                println!("  + {}: {} {}",
                    surcharge.name,
                    surcharge.amount,
                    result.currency
                );
            }

            println!("Net total: {} {}", result.net_price, result.currency);

            for tax in &result.taxes {
                println!("  + {} ({}%): {} {}",
                    tax.name,
                    tax.rate,
                    tax.amount,
                    result.currency
                );
            }

            println!("Gross total: {} {}", result.total_price, result.currency);
        }
    }
}
```

### Example 2: Price by Date

```rust
use chrono::NaiveDate;

// Get historical price
let price_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let price = engine.calculate_family_price("vitra", family, &config, price_date);

// Prices are filtered by date_from and date_to fields
```

### Example 3: Format Price for Display

```rust
use ofml_lib::oap::{format_german_price, format_german_price_with_currency};
use rust_decimal::Decimal;

let amount = Decimal::new(123456, 2); // 1234.56

// German format: 1.234,56
println!("{}", format_german_price(amount));

// With currency: 1.234,56 EUR
println!("{}", format_german_price_with_currency(amount, "EUR"));
```

### Example 4: Export Price with Configuration

```rust
use ofml_lib::oap::export_family_json;

let json = export_family_json(
    "sedus",
    "ai",
    &family.base_article_nr,
    &config,
    price.as_ref(),
    &[], // warnings
);

println!("{}", json);
```

**Output**:
```json
{
  "article_nr": "AI-121",
  "manufacturer": "sedus",
  "series": "ai",
  "variant_code": "S_STOFF=2G3;S_SITZHOEHE=1701",
  "configuration": {
    "S_STOFF": "2G3",
    "S_SITZHOEHE": "1701"
  },
  "pricing": {
    "base": 599.0,
    "surcharges": [
      {
        "var_cond": "S_166",
        "description": "Modellfarbe Rubinrot",
        "amount": 44.0,
        "is_percentage": false
      }
    ],
    "net": 643.0,
    "taxes": [
      {
        "name": "MwSt (19%)",
        "rate": 19.0,
        "amount": 122.17
      }
    ],
    "total": 765.17,
    "currency": "EUR"
  }
}
```

### Example 5: Custom Price Lookup

```rust
use ofml_lib::oap::price::{PriceLookup, PriceQuery};
use chrono::Local;

fn custom_price_lookup() {
    let lookup = PriceLookup::new("/reference/ofmldata");

    let query = PriceQuery::new(
        "sedus".to_string(),
        "AI-121".to_string(),
        "S_STOFF=2G3;S_SITZHOEHE=1701".to_string(),
        Local::now().date_naive(),
    );

    match lookup.lookup(&query) {
        Ok(price) => {
            println!("Price found: {} {}", price.total_price, price.currency);
        }
        Err(e) => {
            println!("Price lookup failed: {}", e);
        }
    }
}
```

## Troubleshooting

### Problem: No Price Found

**Symptoms**:
- `calculate_family_price()` returns `None`
- Error: "Article not found in price table"

**Solutions**:

1. **Check article number format**:
   ```rust
   // Verify the base article number matches ocd_price table
   println!("Looking up: {}", family.base_article_nr);
   ```

2. **Check for wildcard prices**:
   ```rust
   // Some manufacturers use article_nr="*" for all articles
   // The library automatically falls back to wildcard lookup
   ```

3. **Verify price date validity**:
   ```rust
   // Ensure price_date is within date_from..date_to range
   let price_date = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
   ```

4. **Check database path**:
   ```rust
   use std::path::Path;
   let db_path = Path::new("/reference/ofmldata/sedus/ai/DE/1/db/pdata.ebase");
   assert!(db_path.exists());
   ```

### Problem: Incorrect Surcharges

**Symptoms**:
- Expected surcharge not applied
- Unexpected surcharge applied

**Solutions**:

1. **Debug variant code generation**:
   ```rust
   println!("Variant code: {}", config.variant_code);
   // Should show: S_STOFF=2G3;S_SITZHOEHE=1701;...
   ```

2. **Check var_cond in database**:
   ```rust
   // Read ocd_price table directly to see exact var_cond values
   let reader = OcdReader::new(db_path)?;
   let prices = reader.read_prices("AI-121")?;
   for price in prices {
       println!("{}: var_cond={}", price.price_level, price.var_cond);
   }
   ```

3. **Test var_cond matching**:
   ```rust
   let matches = var_cond_matches("S_166", &config.variant_code);
   println!("S_166 matches: {}", matches);
   ```

### Problem: Corrupted Price Data

**Symptoms**:
- Inconsistent prices
- DataWarning with code "CORRUPTED_RECORD"

**Solutions**:

1. **Check warnings**:
   ```rust
   // Warnings are included in price calculation results
   if let Some(warning) = warnings.first() {
       println!("Warning: {} - {}", warning.code, warning.message);
   }
   ```

2. **Automatic recovery**:
   The library automatically attempts to recover corrupted records.
   Check the warning details for recovery status.

3. **Re-export data**:
   If corruption is widespread, request fresh data export from the manufacturer.

### Problem: Wrong Currency

**Symptoms**:
- Price in unexpected currency
- Multiple currency codes in results

**Solutions**:

1. **Filter by currency**:
   ```rust
   // Prices are filtered by currency in calculation
   // Ensure the article has prices in the expected currency
   ```

2. **Check ocd_price table**:
   ```rust
   // Verify currency column values
   for price in prices {
       println!("Currency: {}", price.currency);
   }
   ```

### Problem: Missing Price Descriptions

**Symptoms**:
- Surcharge name is var_cond code instead of readable text
- E.g., "S_166" instead of "Modellfarbe Rubinrot"

**Solutions**:

1. **Check ocd_pricetext table**:
   ```rust
   // Verify price_textnr references exist
   let description = reader.get_price_description(&price, "DE");
   println!("Description: {}", description);
   ```

2. **Use fallback**:
   The library automatically falls back to var_cond when description is missing.

## Related Documentation

- [DATA-FORMATS.md](DATA-FORMATS.md) - OCD table schemas
- [OCD-PRICING-IMPLEMENTATION.md](OCD-PRICING-IMPLEMENTATION.md) - Implementation details
- [LIBRARY-OVERVIEW.md](LIBRARY-OVERVIEW.md) - Library architecture
- [docs/ofml-specs/ocd_4_3.md](/workspace/docs/ofml-specs/ocd_4_3.md) - OCD specification
