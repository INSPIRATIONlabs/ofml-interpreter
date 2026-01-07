# OCD Pricing Patterns Research Summary

## Overview

This document summarizes research on OCD (OFML Commercial Data) pricing patterns across different manufacturers in `/reference/ofmldata/`. The investigation covered 11 manufacturer/series combinations across 5 manufacturers: Framery, FAST, Sedus, Bisley, and Arper.

---

## 1. Pricing Strategies

### Strategy A: Empty Base + Named Surcharges
**Manufacturers**: Framery (frmr_2q, frmr_q), Bisley (sf, pf)

- Base price: `var_cond = ""`  (empty string)
- Surcharges: Descriptive codes like `PG_WHITEBOARD_EXTERIORS`, `DE_ROLLADEN`
- Matching: Property values match surcharge names (with or without prefix)

```
Base:      article_nr='2Q_HUDDLE', var_cond='', price=44140 EUR
Surcharge: article_nr='*', var_cond='PG_WHITEBOARD_EXTERIORS', price=1050 EUR
```

### Strategy B: Standard Product Group + Numeric Surcharges
**Manufacturers**: Sedus (ai, sf)

- Base price: `var_cond = "S_PGX"` (Sedus Product Group X)
- Surcharges: Numeric codes like `S_166`, `S_1513`, `S_2415_F2`
- Matching: Extract number from property value, match against `S_{number}`

```
Base:      article_nr='AI-121', var_cond='S_PGX', price=599 EUR
Surcharge: article_nr='AI-121', var_cond='S_1513', price=228 EUR
```

### Strategy C: TABLE-Computed var_cond (No Surcharges)
**Manufacturers**: FAST (kr, wkm)

- Base price: Complete configuration encoded in var_cond (e.g., `SG-KR-KM-D54-ES`)
- Surcharges: None - all pricing via base prices
- Matching: Build composite var_cond from property selections using TABLE relation lookups

```
Base:      article_nr='sG-Kr', var_cond='SG-KR-KM-D54-ES', price=385.71 EUR
```

The var_cond is computed dynamically by:
1. Finding `$VARCOND = PropertyName` in `ocd_relation`
2. Cascading TABLE lookups to resolve property dependencies
3. Building the final var_cond string from resolved values

### Strategy D: Complex Code System
**Manufacturers**: Arper (catifa46, saya)

- Base price: Encoded codes like `"00"`, `"08PB"`, `"0804W"`
- Surcharges: Abbreviation codes like `FUST02E`, `STR0301`
- Matching: Property values contain embedded codes; 1525 base prices for catifa46 alone

---

## 2. Data Corruption Patterns

### Issue: Framery ONE_COMPACT Missing Header Bytes

**File**: `/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase`

Records 9-14 in `ocd_price` are corrupted - missing first 8 bytes (record_id + article_nr offset fields), causing all subsequent fields to misalign.

**Impact**:
- Parser reads garbage values for article_nr, var_cond, price_level
- Base price of 12,280 EUR for ONE_COMPACT_BASE appears as 5.64e-41 (garbage)
- Price data IS present but shifted by 8 bytes

**Recovery**: Reading at offset +8 reveals correct data:
```
Correct: article_nr='ONE_COMPACT_BASE', var_cond='', level='B', price=12280.00 EUR
```

### Issue: Framery ONE Missing Base Prices

**File**: `/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase`

- Contains 9 surcharges (level 'X') but ZERO base prices (level 'B')
- Root cause: Different from corruption - appears to be incomplete data export
- Impact: Cannot calculate total price without base

---

## 3. Wildcard Pricing (article_nr = "*")

Wildcard article numbers apply surcharges globally to all articles in a series.

**Usage Pattern**:
| Manufacturer | Series | Wildcard Usage |
|-------------|--------|----------------|
| Framery | frmr_2q, frmr_q, frmr_one | ALL surcharges use `*` |
| Bisley | sf, pf | Some surcharges use `*` |
| Sedus | ai | Article-specific surcharges only |
| FAST | all | No surcharges (all base prices) |

**Example** (Framery frmr_2q):
```
article_nr='*', var_cond='PG_TABLE_H110', price=135 EUR  -> Applies to 2Q_HUDDLE, 2Q_FOCUS
article_nr='*', var_cond='PG_LAN_PORT', price=420 EUR   -> Applies to all articles
```

**Implementation**: When looking up surcharges, must query BOTH:
1. `article_nr = {specific_article}`
2. `article_nr = '*'`

---

## 4. Universal Rules

Patterns that work across ALL investigated manufacturers:

1. **price_level field is authoritative**:
   - `'B'` = Base price
   - `'X'` = Surcharge (additive)
   - `'D'` = Discount

2. **No manufacturer uses ocd_propvalue2varcond table** - must rely on pattern matching

3. **Surcharges are cumulative** - sum all applicable surcharges to base

4. **Multiple base prices** indicate different configurations (e.g., FAST encodes full config in var_cond)

---

## 5. Recommended Lookup Order

For maximum compatibility with unknown manufacturers:

```
1. Try empty var_cond for base price  (Framery, Bisley)
2. Try "S_PGX" for base price         (Sedus)
3. Try TABLE-computed var_cond        (FAST)
4. Try property value as var_cond     (Arper)
5. Fall back to first base price
```

For surcharges:
```
1. Match PG_* prefix against property values  (Framery, Bisley)
2. Match S_{number} against property values   (Sedus)
3. Direct code match                          (Arper)
4. Include wildcard article_nr='*' surcharges
```

---

## 6. Key Findings Summary

| Finding | Details |
|---------|---------|
| No universal var_cond format | Each manufacturer has unique encoding |
| propvalue2varcond unused | Must use heuristic pattern matching |
| Wildcard pricing common | `article_nr='*'` for global surcharges |
| TABLE relations for FAST | var_cond computed from cascading lookups |
| Data corruption exists | Framery ONE_COMPACT has shifted records |
| Missing data exists | Framery ONE has no base prices |

---

## References

- `/workspace/docs/OCD-PRICING-IMPLEMENTATION.md` - Implementation details
- `/workspace/docs/ofml-specs/ocd_4_3.md` - OCD 4.3 specification
- `/workspace/MULTI_MANUFACTURER_PRICING_PATTERNS_REPORT.md` - Full investigation report
- `/workspace/FRAMERY_PRICE_CORRUPTION_SUMMARY.md` - Corruption analysis
