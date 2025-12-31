# OCD Pricing Implementation Notes

This document describes the implementation details of OCD (OFML Commercial Data) pricing in the ofml-interpreter, including findings from analyzing real manufacturer data.

## Overview

The OCD pricing model is defined in the OFML specification (Part IV, OCD 4.3). Price calculation involves:

1. **Base prices** (Level 'B') - The fundamental price for an article
2. **Surcharges** (Level 'X') - Additional charges based on configuration options
3. **Discounts** (Level 'D') - Reductions based on conditions

## OCD Tables Used

### ocd_price

The main price table with the following key fields:

| Field | Description |
|-------|-------------|
| `article_nr` | Article number |
| `var_cond` | Variant condition (surcharge identifier) |
| `price_type` | 'S' (sales) or 'P' (purchase) |
| `price_level` | 'B' (base), 'X' (extra/surcharge), 'D' (discount) |
| `price` | Price amount |
| `is_fix` | 1=fixed amount, 0=percentage |
| `currency` | Currency code (EUR, CHF, etc.) |
| `date_from` / `date_to` | Validity period |
| `price_textnr` | Reference to ocd_pricetext for descriptions |

### ocd_pricetext

Contains multilingual descriptions for price entries (base price name, surcharge reasons, etc.).

### ocd_relation / ocd_relationobj

Contains business rules (Beziehungswissen) for determining when variant conditions apply. These use SAP-style LOVC syntax or OCD-specific syntax.

## Implementation in ofml-interpreter

### Price Matching Strategy

The implementation uses multiple matching strategies since the var_cond format varies by manufacturer:

```rust
fn matches_var_cond_extended(
    var_cond: &str,
    variant_code: &str,
    variant_values: &HashSet<&str>,
    variant_map: &HashMap<&str, &str>,
) -> bool
```

#### Strategy 1: Direct Formula Matching

Matches var_cond patterns like:
- `KEY=value` - Exact match
- `KEY>value` - Greater than comparison
- `KEY<value` - Less than comparison
- `KEY=value1;KEY2=value2` - Multiple conditions (AND)

#### Strategy 2: Sedus-Style Surcharge Codes

For codes like `S_XXXX`:
- `S_166` matches if value "166" is selected
- `S_1701` matches if any value starts with "1701"
- `S_2415_F2` matches compound patterns

### Manufacturer-Specific Findings

#### Sedus (sex)

Analyzed Sedus AI chair pricing data:

**Price Structure:**
- Base price indicator: `S_PGX` (e.g., 599 EUR for AI-121)
- Surcharge codes: `S_166`, `S_167`, `S_168`, `S_1513`, `S_1801`, `S_2415_F2`, `S_6004`, `S_6044`

**Surcharge Code Meanings (from ocd_pricetext):**
| Code | German | English | Amount |
|------|--------|---------|--------|
| S_PGX | Basispreis | Basic price | 599-647 EUR |
| S_1513 | Counterausführung | Counter design | 228 EUR |
| S_166 | Modellfarbe Rubinrot | Model colour ruby red | 44 EUR |
| S_167 | Modellfarbe Salbeigrün | Model colour sage green | 44 EUR |
| S_168 | Modellfarbe Nachtblau | Model colour midnight blue | 44 EUR |
| S_1801 | Lordosenhöhenverstellung | Lumbar height adjustment | 21 EUR |
| S_2415_F2 | Kunststofffuß farbig | Plastic base coloured | 26 EUR |
| S_6004 | Brandschutz | Fire protection | 10 EUR |
| S_6044 | Brandschutz | Fire protection | 10 EUR |

**Property Classes:**
- Properties are grouped into classes like `KLASSE_000000000000164057`
- The `ocd_propertyclass` table maps article numbers to property classes
- Property values are codes like `0000`, `2G3`, `CSE29`, `1701`, etc.

**Key Insight:**
The surcharge codes (e.g., `S_166`) don't directly map to property values in a simple way. They are applied based on:
1. Business rules in `ocd_relation` tables
2. SAP variant condition logic
3. Manufacturer-specific naming conventions

### Variant Code Generation

The variant code is generated from property selections:

```
S_STOFF=2G3;S_SITZHOEHE=1701;S_LEHNE_ABW=0000;...
```

Format: `PROPERTY_KEY=selected_value;...` (sorted alphabetically)

### Current Limitations

1. **Complex Rule Evaluation**: The full SAP-style variant condition logic in `ocd_relation` is not implemented. This would require a rule evaluation engine.

2. **Indirect Mappings**: Some surcharges are triggered by combinations of property values, not individual values.

3. **Manufacturer Variations**: Each manufacturer may use different conventions for surcharge codes.

## Usage

```rust
let engine = ConfigurationEngine::new("/path/to/ofmldata");

let price = engine.calculate_family_price(
    "sex",                    // manufacturer_id
    &family,                  // ProductFamily
    &config,                  // FamilyConfiguration
    NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(),
);

if let Some(result) = price {
    println!("Base: {} EUR", result.base_price);
    println!("Total: {} EUR", result.total_price);
    for surcharge in &result.surcharges {
        println!("  + {} {} EUR", surcharge.name, surcharge.amount);
    }
}
```

## Future Improvements

1. **Rule Engine**: Implement evaluation of `ocd_relation` rules to properly determine variant conditions.

2. **Property-to-Surcharge Mapping**: Create explicit mapping tables for manufacturer-specific surcharge conventions.

3. **Multi-Currency Support**: Handle currency conversion for international pricing.

4. **Discount Calculation**: Implement Level 'D' discount rules.

## References

- OCD 4.3 Specification: `/workspace/docs/ofml-specs/ocd_4_3.md`
- OAM 1.0 Specification: `/workspace/docs/ofml-specs/oam_1_0-en.md`
- OFML 2.0r3 Specification: `/workspace/docs/ofml-specs/ofml_20r3-en.md`
