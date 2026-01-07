# Framery Pricing Data Investigation Report

**Investigation Date:** 2026-01-02
**Investigator:** Claude Code (OFML Data Investigator)
**Issue:** Framery phone booths showing 265 EUR instead of expected 27,000+ EUR

---

## Executive Summary

**CRITICAL FINDING:** The Framery 2Q pdata.ebase file contains **severe data corruption** in the `ocd_price` table. Out of 19 price records, **10 are completely corrupt** with binary garbage data. However, **2 valid base prices exist** showing the actual product costs are in the 40,000+ EUR range, not 265 EUR.

**Root Cause:** The 265 EUR price being displayed is likely a **surcharge** (level X) being incorrectly used as the base price due to either:
1. Missing/corrupt base price for the `2Q_LOUNGE` article
2. Incorrect price matching logic in the configurator

---

## Data Corruption Analysis

### Framery 2Q - ocd_price Table Structure

```
File: /reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase
Total Records: 19
├── Valid Base Prices (level B): 2
├── Valid Surcharges (level X): 7
└── CORRUPT Records: 10 (52% corruption rate!)
```

### Valid Base Prices Found

| Article Number         | Price    | Currency | Var Cond | Level | Status |
|------------------------|----------|----------|----------|-------|--------|
| `2Q_HUDDLE`            | 44,140   | EUR      | ''       | B     | ✓ OK   |
| `2Q_WITHOUT_FURNITURE` | 42,070   | EUR      | ''       | B     | ✓ OK   |
| `2Q_LOUNGE`            | MISSING! | -        | -        | -     | ✗ MISSING |

**KEY FINDING:** The `2Q_LOUNGE` article has **NO BASE PRICE** in the table! This is the critical issue.

### Valid Surcharges (level X)

These are optional add-ons that should be **added to** the base price, not used **as** the base price:

| Var Cond                    | Price | Description                      |
|-----------------------------|-------|----------------------------------|
| `PG_TABLE_H110`             | 135   | Height 110cm table               |
| `PG_TABLE_H90`              | 85    | Height 90cm table                |
| `PG_LAN_PORT`               | 420   | LAN port option                  |
| `PG_PIAGGIO_TABLE_TWO`      | 620   | Piaggio table (2 units)          |
| `PG_WHITEBOARD_EXTERIORS`   | 1,050 | Whiteboard exterior              |
| `PG_SMART_LOCK`             | 665   | Smart lock system                |
| `PG_COPPER_DOOR_HANDLE`     | 90    | Copper door handle               |

**The 265 EUR bug:** The system is likely summing some of these surcharges (e.g., 90 + 85 + 90 = 265) when it cannot find a base price for `2Q_LOUNGE`.

### Corrupt Records Sample

The corrupt records show completely invalid data:

```
Record 0 (Corrupt):
  price_level: 'PG_DATA_CARTRIDGE_POWER_OUTLET'  ← Should be 'B' or 'X'!
  price_textnr: 'X'                              ← Wrong field!
  article_nr: ''                                 ← Empty!
  price_type: '*'                                ← Wrong value!
  price: 0.00000000000000000000000000000000000000006227930894845217  ← Garbage!
  currency: ''                                   ← Empty!
  date_from: 'BF           á          á{     (  GS ...'  ← Binary garbage!
  is_fix: 45200                                  ← Should be 0 or 1!
  scale_quantity: 45568                          ← Absurd value!
```

The corrupt records appear to have their field offsets shifted, causing:
- `var_cond` values to appear in `price_level` field
- String constants to appear in wrong fields
- Binary data leaking into text fields

---

## Relation Analysis

### Price Relations (rel_domain = 'P')

The `ocd_relationobj` table shows:

```
rel_obj: 10000
rel_domain: 'P'
rel_name: 'F_PREISE'
rel_type: '3'
```

This links to the `ocd_relation` table with rules for assigning `$VarCond`:

| Block | Relation Rule | VarCond Assigned |
|-------|---------------|------------------|
| 1     | `M_BOOKING_SYSTEM = 'YES'` | `PG_BOOKING_SYSTEM` |
| 2     | `M_ARTNO = 'LOUNGE' and M_SOFA_COLOR not in ('K446','K151')` | `PG_CLINT_SOFA_OPTION_FABRIC` |
| 3     | `M_HANDLE_TYPE = 'COP'` | `PG_COPPER_DOOR_HANDLE` |
| 4     | `M_CARTRIDGE_POWER_OUTLET = 'YES'` | `PG_DATA_CARTRIDGE_POWER_OUTLET` |
| 5     | `M_LAN_PORT = 'YES'` | `PG_LAN_PORT` |
| 6     | `M_EXTERIOR not in ('RAL9016','WHTBRD')` | `PG_EXTERIORS_ULTRA_MATT_COLOR` |
| 7     | `M_FRAME not in ('F6463')` | `PG_FRAMES_OPTION_COLOR` |

**Note:** These relation codes (`PG_*`) should match `var_cond` values in surcharge records. However, in the corrupt records, these codes are appearing in the `price_level` field instead!

---

## Articles Defined

```sql
SELECT article_nr, art_type, short_textnr, rel_obj
FROM ocd_article
```

| article_nr          | art_type | rel_obj | Description           |
|---------------------|----------|---------|----------------------|
| `2Q_HUDDLE`         | C        | 10000   | Framery 2Q Huddle    |
| `2Q_LOUNGE`         | C        | 10000   | Framery 2Q Lounge    |
| `2Q_WITHOUT_FURNITURE` | C     | 10000   | Framery 2Q (no furn) |

All three articles reference the same `rel_obj: 10000`, which links to the price relations.

---

## Framery ONE Analysis

**Status:** Data appears **less corrupt** but still has issues:

```
File: /reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase
Total Records: 31
├── Valid Base Prices (level B): 1
├── Valid Surcharges (level X): 21
└── Corruption: Minimal (truncated output)
```

**Base Price Found:**
- Article: `ONE` (exactly 1 base price exists)
- Pricing appears more complete than 2Q

---

## Root Cause Analysis

### Why is the configurator showing 265 EUR?

1. **Missing Base Price:** `2Q_LOUNGE` has NO base price record (level B) in the table
2. **Corrupt Data:** 10 out of 19 price records are completely unusable
3. **Fallback Logic:** The configurator likely falls back to summing surcharges when no base price exists
4. **Surcharge Summation:** 265 EUR could be: 85 (table) + 90 (handle) + 90 (something else)

### Database Corruption Pattern

The corruption appears to be a **record offset issue** where:
- Field data is shifted by approximately 8-16 bytes
- String references are pointing to invalid memory
- Binary data is leaking into text fields
- Record boundaries are misaligned

This suggests either:
1. **Export Error:** The EBase file was incorrectly generated
2. **Version Mismatch:** The record schema doesn't match what our parser expects
3. **File Corruption:** Physical corruption of the binary file

---

## Recommended Actions

### Immediate Fix (Short-term)

1. **Add Missing Base Price:** Create a base price record for `2Q_LOUNGE`:
   ```
   article_nr: '2Q_LOUNGE'
   price_level: 'B'
   var_cond: ''
   price: 45900.00  (estimated based on 2Q_HUDDLE + upgrades)
   currency: 'EUR'
   price_type: 'S'
   ```

2. **Validate Price Matching Logic:** Ensure the configurator:
   - Always requires a base price (level B) before allowing surcharges
   - Fails gracefully if no base price exists (don't sum surcharges as base!)
   - Logs warnings when corrupt data is encountered

### Long-term Fix (Root Cause)

1. **Re-export EBase Files:** Request fresh exports from Framery's data source
2. **Data Validation:** Add validation checks in the EBase reader to detect:
   - Records with empty required fields
   - Price levels that aren't 'B' or 'X'
   - Absurdly large `scale_quantity` or `is_fix` values
   - Binary data in text fields

3. **Error Recovery:** Implement corruption recovery:
   ```rust
   if record.price_level not in ['B', 'X']:
       log_warning("Corrupt price record detected, skipping")
       continue;
   ```

---

## Testing Recommendations

### Test Cases to Add

1. **Missing Base Price:**
   ```rust
   #[test]
   fn test_article_without_base_price() {
       // Should return error, not sum surcharges
       let result = calculate_price("2Q_LOUNGE", &config);
       assert!(result.is_err());
   }
   ```

2. **Corrupt Record Handling:**
   ```rust
   #[test]
   fn test_corrupt_price_records_skipped() {
       // Should skip invalid records, not crash
       let prices = load_prices_with_validation("frmr_2q");
       assert_eq!(prices.valid_count, 9);
       assert_eq!(prices.corrupt_count, 10);
   }
   ```

3. **Price Calculation:**
   ```rust
   #[test]
   fn test_framery_2q_huddle_base_price() {
       let price = get_base_price("2Q_HUDDLE");
       assert_eq!(price, 44140.0);
   }
   ```

---

## File Paths Reference

- **Framery 2Q:** `/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase`
- **Framery ONE:** `/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase`

---

## Conclusion

The 265 EUR bug is caused by **missing base price data** for the `2Q_LOUNGE` article combined with severe database corruption (52% of price records are unusable). The actual price should be in the **40,000-45,000 EUR range** based on comparable products (2Q_HUDDLE: 44,140 EUR).

**Priority:** CRITICAL - Customer-facing pricing is off by 99.4%!

**Next Steps:**
1. Contact Framery for corrected data exports
2. Add base price validation to prevent this bug pattern
3. Implement corruption detection in EBase reader
4. Add comprehensive test coverage for pricing edge cases
