# Framery ONE COMPACT Price Data Corruption Investigation Report

**Date:** 2026-01-02
**File:** `/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase`
**Affected Record:** Record 13 in ocd_price table
**Product:** ONE_COMPACT_BASE

## Executive Summary

The ocd_price table in the Framery ONE COMPACT database has corrupted records starting from record 9 through record 14 (6 records total). These records are **missing the first 4 bytes** (record ID field), causing all subsequent field values to be shifted by 4 bytes and misaligned.

## ocd_price Table Schema

The correct schema (56 bytes per record):

| Offset | Field Name      | Type          | Size |
|--------|-----------------|---------------|------|
| 0      | (record_id)     | UInt32        | 4    |
| 4      | article_nr      | StringOffset  | 4    |
| 8      | var_cond        | StringOffset  | 4    |
| 12     | price_type      | StringOffset  | 4    |
| 16     | price_level     | StringOffset  | 4    |
| 20     | price_rule      | StringOffset  | 4    |
| 24     | price_textnr    | StringOffset  | 4    |
| 28     | price           | Float32       | 4    |
| 32     | is_fix          | UInt32        | 4    |
| 36     | currency        | StringOffset  | 4    |
| 40     | date_from       | StringOffset  | 4    |
| 44     | date_to         | StringOffset  | 4    |
| 48     | scale_quantity  | UInt32        | 4    |
| 52     | rounding_id     | StringOffset  | 4    |

## Corruption Analysis

### Record 13 (ONE_COMPACT_BASE) - Corrupted State

**File offset:** 0x34d8 (13528)

**Raw bytes:**
```
0000: 00 00 00 01 00 00 9d 56 00 00 00 00 00 00 91 a2
0010: 00 00 9d 56 00 00 9f 8e 00 00 9f 92 00 00 9d 56
0020: 00 00 a1 8c 46 3f e0 00 00 00 00 01 00 00 a1 d4
0030: 00 00 a1 da 00 00 a1 e6
```

**Parsed (INCORRECT due to missing 4-byte header):**

| Field          | Read Value             | Actual Meaning                    |
|----------------|------------------------|-----------------------------------|
| article_nr     | "" (empty)             | Should be "ONE_COMPACT_BASE"      |
| var_cond       | "" (empty)             | Correct (empty)                   |
| price_type     | "ONE_COMPACT_BASE"     | Should be "S"                     |
| price_level    | "" (empty)             | Should be "B"                     |
| price_rule     | "S"                    | Should be empty                   |
| price_textnr   | "B"                    | Should be price text UUID         |
| price          | 5.64e-41 (garbage)     | Should be 12,280 EUR              |
| is_fix         | 41356                  | Should be 1                       |
| currency       | (read error)           | Should be "EUR"                   |
| date_from      | (empty)                | Should be "20220501"              |
| date_to        | "EUR"                  | Should be "99991231"              |
| scale_quantity | 41434                  | Should be 1                       |
| rounding_id    | "99991231"             | Should be empty                   |

### What the Record SHOULD Contain

**Correct interpretation (accounting for 4-byte shift):**

```
article_nr:    "ONE_COMPACT_BASE"  (from string offset 0x91a2)
var_cond:      ""                  (empty - base price indicator)
price_type:    "S"                 (Standard price type)
price_level:   "B"                 (Base price level)
price_rule:    ""                  (empty)
price_textnr:  (should be UUID)    (currently shifted to wrong position)
price:         12,280 EUR          (bytes: 46 3f e0 00, found at offset 32)
is_fix:        1                   (fixed price)
currency:      "EUR"
date_from:     "20220501"
date_to:       "99991231"
scale_quantity: 1
rounding_id:   ""
```

## Finding the Actual Price

The price bytes `46 3f e0 00` were found at byte offset 32-35 in the corrupted record (instead of the correct offset 28-31).

**Price calculation:**
```
Bytes: 0x46 0x3f 0xe0 0x00
IEEE-754 float (big-endian): 12,280.0
```

**Price location in file:**
- File offset: 0x34fc (13564 bytes from start)
- This is offset 32 within the record (shifted from correct offset 28)

## Affected Records Summary

### Records 0-8 (CORRECT)
- Record 0: ONE_COMPACT_ESSENTIALS @ 13,800 EUR
- Records 1-8: Surcharge prices (125-700 EUR range)
- All have proper structure with record ID field

### Records 9-14 (CORRUPTED - Missing 4-byte header)
- Record 9:  Surcharge (PG_TABLE_TOP_OPTION_COLOR) - 700 EUR
- Record 10: Surcharge (PG_ADJUSTABLE_SEAT) - 380 EUR
- Record 11: Surcharge (PG_SEAT_UPHOLSTERY_OPTION_COLOR) - 700 EUR
- Record 12: Surcharge (PG_JUNCTION_BOX) - 125 EUR
- **Record 13: ONE_COMPACT_BASE - 12,280 EUR** ← MAIN ISSUE
- Record 14: Surcharge (PG_POWER_OUTLETS_USB_PD65W_ESSENTIALS) - 155 EUR

All 6 corrupted records show the same pattern: missing first 4 bytes causing field misalignment.

## Comparison with ONE_COMPACT_ESSENTIALS

For reference, the correctly formatted Record 0:

```
article_nr:    "ONE_COMPACT_ESSENTIALS"
var_cond:      ""
price_type:    "S"
price_level:   "B"
price:         13,800 EUR
currency:      "EUR"
date_from:     "20220501"
date_to:       "99991231"
```

ONE_COMPACT_BASE (12,280 EUR) is priced lower than ESSENTIALS (13,800 EUR), which makes sense as "BASE" typically represents a lower-tier model.

## Root Cause

The corruption appears to be a data generation or export error where:

1. Records 0-8 were written correctly with the full 56-byte structure
2. Records 9-14 were written with only 52 bytes (missing the initial 4-byte record ID)
3. The EBase reader still expects 56 bytes per record, so it reads 4 bytes from the next record
4. This causes a cascading misalignment of all fields within these records

## Historical Price Note

The user mentioned seeing "~31,000 EUR" in an earlier version. Our search found:
- No 31,000 EUR price in current file
- No 30,000 EUR price in current file
- Current ONE_COMPACT_BASE price: **12,280 EUR**
- Current ONE_COMPACT_ESSENTIALS price: **13,800 EUR**

The 31,000 EUR price either:
1. Was from a different version/revision of the data
2. Was a combined price (base + options)
3. Was from a different product configuration

## Recommendations

1. **Fix the source data:** Regenerate the ocd_price table with correct record structure
2. **Add validation:** Implement checks to detect records with misaligned string offsets
3. **Data recovery:** If needed, the actual price values can be extracted by:
   - Reading bytes at offset +4 from the current position for each field
   - Correctly interpreting the price float at byte offset 32 instead of 28

## Code Location for Fix

The EBase reader in `/workspace/src/ebase.rs` correctly reads records assuming proper structure. The fix needs to happen in the data source/generation tool that creates these EBase files.

For a parser-side workaround, detection heuristics could check:
- If article_nr reads as empty but price_type contains an article name pattern
- If float price values are suspiciously small (< 0.0001)
- If string offsets for date fields resolve to "EUR" or other currency codes

## Verification Commands

```bash
# View record 13 raw bytes
hexdump -C -s 0x34d8 -n 56 /reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase

# Search for the 12,280 EUR price bytes
hexdump -C /reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase | grep "46 3f e0 00"

# Run investigation test
cargo test --test framery_price_investigation investigate_framery_price_corruption -- --nocapture
cargo test --test framery_binary_analysis analyze_framery_binary_corruption -- --nocapture
```

## Conclusion

**ONE_COMPACT_BASE base price: 12,280 EUR**

This price is found at file offset 0x34fc, embedded in a corrupted record that is missing its 4-byte header. The corruption affects 6 consecutive records (9-14) in the ocd_price table, all showing the same structural defect.
