# Framery Pricing Investigation - Quick Summary

## The Problem
Framery 2Q phone booths showing **265 EUR** instead of expected **27,000+ EUR**

## Root Cause Found
**CRITICAL:** The `2Q_LOUNGE` article has **NO BASE PRICE** in the database, and the price table has **52% data corruption** (10 out of 19 records).

## What We Found

### Valid Base Prices (Level B)
```
Article: 2Q_HUDDLE             Price: 44,140 EUR ✓
Article: 2Q_WITHOUT_FURNITURE  Price: 42,070 EUR ✓
Article: 2Q_LOUNGE             Price: MISSING!   ✗✗✗
```

### Valid Surcharges (Level X)
These are ADD-ONS, not base prices:
```
PG_TABLE_H110               135 EUR   (table height 110cm)
PG_TABLE_H90                 85 EUR   (table height 90cm)
PG_LAN_PORT                 420 EUR   (LAN port)
PG_PIAGGIO_TABLE_TWO        620 EUR   (Piaggio table x2)
PG_WHITEBOARD_EXTERIORS   1,050 EUR   (whiteboard exterior)
PG_SMART_LOCK               665 EUR   (smart lock)
PG_COPPER_DOOR_HANDLE        90 EUR   (copper handle)
```

**The 265 EUR bug:** System is summing surcharges (e.g., 90+85+90=265) when no base price exists.

### Data Corruption Example
```
CORRUPT RECORD:
  price_level: 'PG_DATA_CARTRIDGE_POWER_OUTLET'  ← Should be 'B' or 'X'!
  price_textnr: 'X'                              ← Wrong field!
  article_nr: ''                                 ← Empty!
  price: 0.0000000000000000000000000000000000000000062...  ← Garbage!
  date_from: 'BF           á          á{     (  GS ...'  ← Binary junk!
```

## The Fix

### Immediate (Code Fix)
1. **Add validation:** Require a base price before calculating total
2. **Error handling:** Don't sum surcharges as base price
3. **Missing data alert:** Log warning when base price not found

### Data Fix (Requires Framery)
1. **Missing base price:** Need to add `2Q_LOUNGE` base price (~45,900 EUR estimated)
2. **Re-export data:** Request fresh EBase files from Framery to fix corruption

## Files Investigated
```
/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase
```

## Detailed Report
See: `/workspace/FRAMERY_PRICING_INVESTIGATION_REPORT.md`

---

**Status:** CRITICAL - Pricing is 99.4% incorrect!
**Impact:** Customer sees 265 EUR instead of 40,000+ EUR
**Priority:** Fix validation logic immediately, request corrected data from Framery
