# Framery ONE_COMPACT_BASE Price Corruption - Investigation Summary

## Answer to Your Question

**ONE_COMPACT_BASE base price: 12,280 EUR**

This price is stored in the file but corrupted due to missing header bytes in record 13.

## The Corruption

### What's Wrong

Records 9-14 in the `ocd_price` table are **missing the first 8 bytes** of data:
- Missing: 4-byte record_id field
- Missing: 4-byte article_nr string offset field

This causes all subsequent fields to be misaligned by 8 bytes.

### ocd_price Schema (Correct)

```
Offset | Field          | Type    | Size
-------|----------------|---------|-----
0      | record_id      | UInt32  | 4
4      | article_nr     | StrOff  | 4  ← MISSING in records 9-14
8      | var_cond       | StrOff  | 4  ← MISSING in records 9-14
12     | price_type     | StrOff  | 4  ← Now reads from offset 4
16     | price_level    | StrOff  | 4  ← Now reads from offset 8
20     | price_rule     | StrOff  | 4  ← Now reads from offset 12
24     | price_textnr   | StrOff  | 4  ← Now reads from offset 16
28     | price          | Float32 | 4  ← Now reads from offset 20
32     | is_fix         | UInt32  | 4  ← Now reads from offset 24
36     | currency       | StrOff  | 4  ← Now reads from offset 28
40     | date_from      | StrOff  | 4  ← Now reads from offset 32
44     | date_to        | StrOff  | 4  ← Now reads from offset 36
48     | scale_quantity | UInt32  | 4  ← Now reads from offset 40
52     | rounding_id    | StrOff  | 4  ← Now reads from offset 44
```

### Record 13 - Corrupted Data

**File location:** `/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase`
**Record offset:** 0x34d8 (13528 bytes)
**Record size:** 56 bytes (but missing first 8)

**Raw bytes:**
```
Offset  Hex Data                         ASCII
------  -------------------------------  ----------------
0x34d8  00 00 00 01 00 00 9d 56 00 00   .......V..
0x34e2  00 00 00 00 91 a2 00 00 9d 56   ...........V
0x34ec  00 00 9f 8e 00 00 9f 92 00 00   ............
0x34f6  9d 56 00 00 a1 8c 46 3f e0 00   .V....F?..
0x3500  00 00 00 01 00 00 a1 d4 00 00   ............
0x350a  a1 da 00 00 a1 e6               ......
```

**What the parser reads (INCORRECT):**

| Field        | Read From Offset | Value Read     | Result            |
|--------------|------------------|----------------|-------------------|
| article_nr   | 4                | 0x00009d56     | "" (empty string) |
| var_cond     | 8                | 0x00000000     | "" (empty)        |
| price_type   | 12               | 0x000091a2     | "ONE_COMPACT_BASE"|
| price_level  | 16               | 0x00009d56     | "" (empty)        |
| price        | 28               | 0x00009d56     | 5.64e-41 (garbage)|

**What SHOULD be read (CORRECT with 8-byte shift):**

| Field        | Should Read From | Actual Offset | Bytes          | Value               |
|--------------|------------------|---------------|----------------|---------------------|
| article_nr   | 4                | 12            | 00 00 91 a2    | "ONE_COMPACT_BASE"  |
| var_cond     | 8                | 16            | 00 00 9d 56    | "" (empty = base)   |
| price_type   | 12               | 20            | 00 00 9f 8e    | "S"                 |
| price_level  | 16               | 24            | 00 00 9f 92    | "B"                 |
| price        | 28               | 36            | 46 3f e0 00    | **12,280.0 EUR**    |
| currency     | 36               | 44            | 00 00 a1 d4    | "EUR"               |

### Price Byte Analysis

The price bytes `46 3f e0 00` at file offset 0x34fc (record offset 36):

```python
import struct
price_bytes = bytes([0x46, 0x3f, 0xe0, 0x00])
price = struct.unpack('>f', price_bytes)[0]
# Result: 12280.0
```

## Comparison: ONE_COMPACT Models

| Model                   | Price     | Level | Record |
|-------------------------|-----------|-------|--------|
| ONE_COMPACT_ESSENTIALS  | 13,800 EUR| B     | 0 (OK) |
| **ONE_COMPACT_BASE**    | **12,280 EUR** | **B** | **13 (corrupted)** |

The BASE model is €1,520 cheaper than ESSENTIALS, which makes sense for a lower-tier offering.

## About the "~31,000 EUR" Price

You mentioned seeing approximately 31,000 EUR in an earlier version. Investigation shows:

- **Not found** in current file (searched entire file for bytes `46 f2 30 00` = 31,000 EUR)
- **Not found** for 30,000 EUR or 32,000 EUR either

Possible explanations:
1. Different version/revision of the price list
2. Total price including selected options/surcharges
3. Different product configuration
4. Price from a different market/currency (converted)

## Affected Records

All six corrupted records (9-14) with recovered prices:

| Record | Article  | Var Cond                              | Level | Price (EUR) |
|--------|----------|---------------------------------------|-------|-------------|
| 9      | *        | PG_TABLE_TOP_OPTION_COLOR             | X     | 700         |
| 10     | *        | PG_ADJUSTABLE_SEAT                    | X     | 380         |
| 11     | *        | PG_SEAT_UPHOLSTERY_OPTION_COLOR       | X     | 700         |
| 12     | *        | PG_JUNCTION_BOX                       | X     | 125         |
| **13** | **ONE_COMPACT_BASE** | **(empty = base price)**  | **B** | **12,280**  |
| 14     | *        | PG_POWER_OUTLETS_USB_PD65W_ESSENTIALS | X     | 155         |

All surcharges (records 9-12, 14) also have their prices at offset +8 from where they should be.

## Technical Details

### Column Types

From EBase schema inspection:

```
[0]  article_nr     StringOffset   offset=4    size=4
[1]  var_cond       StringOffset   offset=8    size=4
[2]  price_type     StringOffset   offset=12   size=4
[3]  price_level    StringOffset   offset=16   size=4
[4]  price_rule     StringOffset   offset=20   size=4
[5]  price_textnr   StringOffset   offset=24   size=4
[6]  price          Float32        offset=28   size=4  ← Price here!
[7]  is_fix         Int32Unsigned  offset=32   size=4
[8]  currency       StringOffset   offset=36   size=4
[9]  date_from      StringOffset   offset=40   size=4
[10] date_to        StringOffset   offset=44   size=4
[11] scale_quantity Int32Unsigned  offset=48   size=4
[12] rounding_id    StringOffset   offset=52   size=4
```

### String Pool Offsets

Key string pool locations:

- 0x91a2: "ONE_COMPACT_BASE"
- 0x9f8e: "S" (price type)
- 0x9f92: "B" (base price level)
- 0x9d56: "" (empty string)
- 0xa1d4: "EUR"
- 0xa1da: "20220501" (date_from)
- 0xa1e6: "99991231" (date_to)

## Root Cause

The corruption originates from the data generation/export process. The most likely scenarios:

1. **Partial record write**: Records 9-14 were written without their header fields
2. **Schema mismatch**: Generator used a 48-byte schema instead of 56-byte
3. **Buffer underrun**: Missing initialization of first 8 bytes before record write

This is NOT a reader bug - the EBase format specification is being followed correctly by the parser.

## Verification

Run these tests to verify the findings:

```bash
# View the corrupted record
cargo test --test framery_price_investigation -- --nocapture

# Binary-level analysis
cargo test --test framery_binary_analysis -- --nocapture

# Direct hex dump inspection
hexdump -C -s 0x34d8 -n 56 /reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase
```

## Conclusion

**The ONE_COMPACT_BASE base price is definitively 12,280 EUR.**

This value is stored in the file at:
- File offset: 0x34fc
- Bytes: `46 3f e0 00`
- IEEE-754 float (big-endian): 12,280.0

The corruption does not destroy the data - it merely shifts it by 8 bytes, making it readable with the correct offset adjustment.
