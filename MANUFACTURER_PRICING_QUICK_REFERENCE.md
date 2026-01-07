# Manufacturer Pricing Patterns - Quick Reference

## Investigation Date: 2026-01-02

---

## Summary Table

| Manufacturer | Series | Base Price Pattern | Surcharge Pattern | Has p2v Table? | Notes |
|-------------|---------|-------------------|------------------|----------------|-------|
| **Framery** | frmr_one | **NONE** | `PG_*` | NO | Missing base prices! |
| **Framery** | frmr_2q | `""` (empty) | `PG_*` | NO | 42k-44k EUR |
| **Framery** | frmr_q | `""` (empty) | `PG_*` | NO | 28k-42k EUR |
| **FAST** | kr | `SG-KR-*-*-*` | NONE | NO | All base, no surcharge |
| **FAST** | wkm | `SG-MB-WKM-*` | NONE | NO | All base, no surcharge |
| **Sedus** | ai | `S_PGX` | `S_###` | NO | Numeric suffix match |
| **Sedus** | sf | `S_PGX` | NONE | NO | Simple base only |
| **Bisley** | sf | `""` (empty) | `DE_*`, `PG_*` | NO | Like Framery |
| **Bisley** | pf | `""` (empty) | `PG_*` | NO | Like Framery |
| **Arper** | catifa46 | Complex codes | Short codes | NO | 1525 base prices! |
| **Arper** | saya | Simple codes | `STR*` | NO | Mostly base |

---

## Pattern Types

### Pattern 1: Empty Base + Named Surcharges
**Manufacturers**: Framery (2q, q), Bisley (sf, pf)

```
Base:      var_cond = ""
Surcharge: var_cond = "PG_CARPET_OPTION_COLOR"
           var_cond = "PG_WHITEBOARD_EXTERIORS"
           var_cond = "DE_ROLLADEN"
```

**Match Strategy**: Check if property value matches surcharge name (with or without prefix)

---

### Pattern 2: Standard Product Group + Numeric Surcharges
**Manufacturers**: Sedus (ai, sf)

```
Base:      var_cond = "S_PGX"
Surcharge: var_cond = "S_166"
           var_cond = "S_1513"
           var_cond = "S_2415_F2"
```

**Match Strategy**: Extract number from property value, match against `S_` codes

---

### Pattern 3: Complete Configuration Encoding (No Surcharges)
**Manufacturers**: FAST (kr, wkm)

```
Base:      var_cond = "SG-KR-KM-D54-ES"
           var_cond = "SG-MB-WKM-100X60-HOEI"
           var_cond = "SG-KR-WKM-3ER-ES"
Surcharge: (none)
```

**Match Strategy**: Build complete var_cond string from all property selections

Format: `SG-{SERIES}-{MODEL}-{SIZE}-{FINISH}`

---

### Pattern 4: Complex Code System
**Manufacturers**: Arper (catifa46, saya)

```
Base:      var_cond = "00"      (base config)
           var_cond = "08PB"    (finish 08 + plywood base)
           var_cond = "0804W"   (finish 08 + variant 04 + wood)
Surcharge: var_cond = "FUST02E" (abbreviation codes)
           var_cond = "STR0301" (structure option)
```

**Match Strategy**: Match var_cond against property values (codes embedded in properties)

---

## Price Ranges

| Manufacturer | Series | Min Price | Max Price | Currency | Product Type |
|-------------|---------|-----------|-----------|----------|--------------|
| Framery | frmr_2q | 42,070 | 44,140 | EUR | Phone booths (2-person) |
| Framery | frmr_q | 27,990 | 42,070 | EUR | Phone booths (4-person) |
| FAST | kr | 184 | 1,344 | EUR | Wall decorations |
| FAST | wkm | 100 | 403 | EUR | Wall decorations |
| Sedus | ai | 599 | 647 | EUR | Office chairs (base) |
| Sedus | sf | 239 | 251 | EUR | Office chairs |
| Bisley | sf | 39 | 1,159 | EUR | File cabinets |
| Bisley | pf | 179 | 369 | EUR | File cabinets |
| Arper | catifa46 | 195 | 604 | EUR | Chairs |
| Arper | saya | 384 | 600 | EUR | Tables |

---

## Surcharge Examples

### Framery (frmr_2q)
```
PG_TABLE_H110           = +135 EUR (table height 110cm)
PG_TABLE_H90            = +85 EUR  (table height 90cm)
PG_LAN_PORT             = +420 EUR (LAN port addition)
PG_WHITEBOARD_EXTERIORS = +1,050 EUR (whiteboard panels)
```

### Sedus (ai)
```
S_1513     = +228 EUR (upholstery option 1513)
S_166      = +44 EUR  (armrest option 166)
S_167      = +??? EUR (armrest option 167)
S_168      = +??? EUR (armrest option 168)
```

### Bisley (pf)
```
PG_AUFPREIS_FARBE_10 = +10 EUR (color surcharge)
PG_AUFPREIS_FARBE_15 = +15 EUR (color surcharge)
```

### Arper (catifa46)
```
FUST02E  = +46 EUR  (frame option)
IGN011   = +33 EUR  (ignifuge treatment)
STR03AC  = +100 EUR (structure option)
STR03AX  = +149 EUR (structure option)
```

---

## Code Patterns for Matching

### Empty var_cond (Base Price)
```rust
// Framery, Bisley: Look for empty string
prices.iter().find(|p| p.var_cond.is_empty() && p.price_level == 'B')
```

### S_PGX (Sedus Base)
```rust
// Sedus: Look for "S_PGX"
prices.iter().find(|p| p.var_cond == "S_PGX" && p.price_level == 'B')
```

### PG_ Prefix (Surcharges)
```rust
// Framery, Bisley: Match "PG_" prefix
if var_cond.starts_with("PG_") {
    let code = var_cond.strip_prefix("PG_").unwrap();
    // Check if property value contains this code
}
```

### S_ Numeric (Sedus Surcharges)
```rust
// Sedus: Extract number, match "S_{number}"
if var_cond.starts_with("S_") && var_cond != "S_PGX" {
    let num = var_cond.strip_prefix("S_").unwrap();
    // Check if property value == num or ends with num
}
```

### SG- Composite (FAST Base)
```rust
// FAST: Build complete code from properties
let varcond = format!("SG-{}-{}-{}-{}",
    props["series"], props["model"],
    props["size"], props["finish"]);
```

---

## Critical Issues

### Issue 1: Framery ONE Missing Base Prices

**Severity**: HIGH

**Description**: `framery/frmr_one` has 9 surcharges but ZERO base prices.

**Impact**: Cannot calculate total price for ONE, ONE_PREMIUM, or ONE_LOUNGE articles.

**Required Action**:
1. Investigate CLS files for hardcoded base price
2. Check if this is a data export error
3. Contact Framery data provider

**Workaround**: None available without base price

---

### Issue 2: No propvalue2varcond Table

**Severity**: MEDIUM

**Description**: None of the investigated manufacturers provide the `ocd_propvalue2varcond` mapping table.

**Impact**: Cannot use direct property→var_cond lookup. Must use pattern matching.

**Required Action**:
1. Implement pattern-based matching (already documented above)
2. Add fallback strategies for unknown patterns
3. Consider adding manufacturer-specific handlers

**Workaround**: Pattern matching as described in this document

---

## Testing Checklist

- [ ] Framery frmr_2q base price (42,070 EUR)
- [ ] Framery frmr_2q with whiteboard (+1,050 EUR)
- [ ] Sedus ai base price (599 EUR, var_cond="S_PGX")
- [ ] Sedus ai with upholstery 1513 (+228 EUR)
- [ ] FAST kr with config SG-KR-KM-D54-ES (385.71 EUR)
- [ ] Bisley sf base price (empty var_cond)
- [ ] Bisley pf with color surcharge (+10 or +15 EUR)
- [ ] Arper catifa46 base price (varies by config)
- [ ] **Framery frmr_one (SHOULD FAIL - no base price)**

---

## Files Generated

1. **Analysis Source**: `/workspace/multi_mfr_pricing_analysis.rs`
2. **Full Report**: `/workspace/MULTI_MANUFACTURER_PRICING_PATTERNS_REPORT.md`
3. **Implementation Guide**: `/workspace/PRICING_IMPLEMENTATION_RECOMMENDATIONS.md`
4. **This Quick Reference**: `/workspace/MANUFACTURER_PRICING_QUICK_REFERENCE.md`
5. **Raw Output**: `/workspace/COMPREHENSIVE_PRICING_INVESTIGATION.md`

---

## Universal Rules (Apply to ALL manufacturers)

1. **price_level field is authoritative**:
   - `'B'` = Base price (always required)
   - `'X'` = Surcharge (optional, additive)
   - `'D'` = Discount (not seen in this investigation)

2. **article_nr wildcard** `'*'` applies to all articles in that series

3. **var_cond field** contains the lookup key (format varies by manufacturer)

4. **Multiple base prices** for same article indicate different configurations

5. **Surcharges are cumulative** - sum all applicable surcharges

6. **Property values** often contain the codes needed for var_cond matching

---

## When in Doubt

**Default Strategy**:
1. Try empty var_cond for base
2. Try "S_PGX" for base
3. Try building composite var_cond
4. Use first available base price
5. For surcharges: check all common prefixes (PG_, S_, DE_, etc.)
6. Log warnings for unmatched patterns

This ensures maximum compatibility even with unknown manufacturers.
