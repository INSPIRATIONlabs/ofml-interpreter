# Framery Data Structure Investigation Report

**Date:** 2026-01-02
**Context:** After adding series-based property filtering to fix FAST KR pricing, Framery configuration is broken - many options are missing and prices are incorrect.

---

## Executive Summary

**ROOT CAUSE IDENTIFIED:** The series-based property filtering that was added to fix FAST is **incompatible** with Framery's data structure.

- **FAST** uses series-specific property classes (different classes per series)
- **Framery** uses **SHARED property classes** across all series (`MG_GLOBAL`, `MG_PROPERTIES`)
- The current filtering code in `ocd_properties.rs` filters properties by `source_series`, which breaks Framery

---

## 1. Framery Series/Families Structure

### 1.1 pdata.ebase Locations

Framery has **8 series** with pdata.ebase files:

```
/reference/ofmldata/framery/accessories/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_four/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_o/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_q/ANY/1/db/pdata.ebase
/reference/ofmldata/framery/frmr_six/ANY/1/db/pdata.ebase
```

**Note:** Uses `ANY` country code instead of `DE`, different from FAST.

### 1.2 Property Classes Per Series

| Series              | Property Classes                                    | Article Count |
|---------------------|-----------------------------------------------------|---------------|
| accessories         | (none - simple accessory)                           | 1             |
| frmr_2q             | MG_GLOBAL, MG_PROPERTIES                            | 3             |
| frmr_four           | MG_GLOBAL, MG_PROPERTIES                            | 4             |
| frmr_o              | MG_GLOBAL, MG_PROPERTIES                            | 3             |
| frmr_one            | MG_GLOBAL, MG_PROPERTIES                            | 3             |
| frmr_one_compact    | MG_GLOBAL, MG_PROPERTIES                            | 2             |
| frmr_q              | MG_GLOBAL, MG_PROPERTIES, MG_PROPERTIES_FLIP_FOLD   | 4             |
| frmr_six            | MG_GLOBAL, MG_PROPERTIES                            | 3             |

### 1.3 Key Finding: SHARED Property Classes

**7 out of 8 series** use the **exact same property classes**: `MG_GLOBAL` and `MG_PROPERTIES`

- These are **manufacturer-level** property classes, not series-specific
- Only exception: `frmr_q` has one additional specialized class `MG_PROPERTIES_FLIP_FOLD`
- Property class names do **NOT** contain the series identifier (no "frmr_one", "frmr_2q", etc.)

---

## 2. Property Classes Definition (ocd_property)

### 2.1 Example from frmr_one

```
Property Class: MG_PROPERTIES
Properties (19 total):
  - M_EXTERIOR (exterior panel color)
  - M_INTERIOR (interior panel color)
  - M_CARPET (carpet option)
  - M_SEAT_UPHOLSTERY (seat upholstery option)
  - M_TABLE_TOP (table top option)
  - M_MONITOR_WALL_MOUNT
  - M_SEISMIC_KIT
  - M_TABLE
  ... etc
```

### 2.2 Example Property Values

```
Class: MG_PROPERTIES, Property: M_EXTERIOR, Values:
  - RAL9016MAT
  - RAL9005
  - S7500N
  - NCSS7020R90B
  - NCS3421R86B
  ... (many color codes)
```

---

## 3. Article-to-Property-Class Mapping (ocd_propertyclass)

### 3.1 Mapping Structure

Articles are mapped to property classes via the `ocd_propertyclass` table:

**frmr_one examples:**
```
Article: ONE           -> Classes: [MG_GLOBAL, MG_PROPERTIES]
Article: ONE_PREMIUM   -> Classes: [MG_GLOBAL, MG_PROPERTIES]
Article: ONE_LOUNGE    -> Classes: [MG_GLOBAL, MG_PROPERTIES]
```

**frmr_2q examples:**
```
Article: 2Q_HUDDLE              -> Classes: [MG_GLOBAL, MG_PROPERTIES]
Article: 2Q_LOUNGE              -> Classes: [MG_GLOBAL, MG_PROPERTIES]
Article: 2Q_WITHOUT_FURNITURE   -> Classes: [MG_GLOBAL, MG_PROPERTIES]
```

### 3.2 Key Insight

**All articles within a series use the same shared property classes.**
**All articles across different series (except frmr_q's flip_fold variant) also use the same classes.**

---

## 4. Pricing Structure (ocd_price)

### 4.1 Price Records from frmr_one

```
Article: *    Level: X    VarCond: PG_SEAT_UPHOLSTERY_OPTION_COLOR
Article: *    Level: X    VarCond: PG_CARPET_OPTION_COLOR
Article: *    Level: X    VarCond: PG_TABLE_TOP_OPTION_COLOR
Article: *    Level: X    VarCond: PG_INTERIOR_PANEL_OPTION_COLOR
Article: *    Level: X    VarCond: PG_EXTERIOR_PANEL_OPTION_COLOR
... (17 total price records)
```

### 4.2 Pricing Pattern

- Uses wildcard article `*` for surcharges (level X)
- `var_cond` codes use `PG_` prefix (e.g., `PG_SEAT_UPHOLSTERY_OPTION_COLOR`)
- Prices are option-based, not article-specific

---

## 5. Comparison: Framery vs FAST

### 5.1 Property Class Organization

| Aspect                    | Framery                          | FAST                              |
|---------------------------|----------------------------------|-----------------------------------|
| **Property Classes**      | Shared across series             | Series-specific (one per series)  |
| **Class Names**           | `MG_GLOBAL`, `MG_PROPERTIES`     | `Rahmen` (generic)                |
| **Series in Class Name**  | **NO**                           | **NO** (also generic!)            |
| **Path Series**           | `frmr_one`, `frmr_2q`, etc.      | `kr`, `wkm`, `km`, etc.           |

### 5.2 Critical Discovery

**BOTH manufacturers use generic property class names that don't contain the series identifier!**

- Framery: `MG_PROPERTIES` (used across frmr_one, frmr_2q, frmr_o, etc.)
- FAST: `Rahmen` (used across kr, wkm, km, etc.)

**This means the series-based filtering is WRONG for both manufacturers!**

### 5.3 Why FAST Works

FAST works because each series has its **own separate pdata.ebase** with **different property values**, even though the class name is the same (`Rahmen`).

When loading from `/fast/kr/`, only KR properties are loaded.
When loading from `/fast/wkm/`, only WKM properties are loaded.

The filtering by `source_series` happens to work because:
1. Each series directory has its own `pdata.ebase`
2. Properties are tagged with `source_series` during load
3. Only one series is queried at a time

### 5.4 Why Framery Breaks

When loading Framery manufacturer data:
1. All series directories are scanned
2. Properties from ALL series get the same class name (`MG_PROPERTIES`)
3. Each property is tagged with its `source_series` (frmr_one, frmr_2q, etc.)
4. When filtering for `frmr_one`, code looks for:
   - Property class: `MG_PROPERTIES` ✓
   - Source series: `frmr_one` ✓
5. **BUT** if querying at manufacturer level or wrong series context, properties are filtered out

---

## 6. Source Series Pattern Analysis

### 6.1 Path Extraction

Series is extracted from path:
```rust
// From: /reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase
// Extracts: "frmr_one"
```

### 6.2 Source Series Assignment

```rust
// In ocd_properties.rs, line 1289
prop.source_series = series.clone();  // Sets to "frmr_one", "frmr_2q", etc.
```

### 6.3 Filtering Logic (THE PROBLEM)

```rust
// In ocd_properties.rs, lines 1014-1021
.filter(|((pc, _), def)| {
    pc == prop_class  // "MG_PROPERTIES"
        && def
            .source_series
            .as_ref()
            .map(|s| s.to_lowercase() == series_lower)  // "frmr_one" == "frmr_one"?
            .unwrap_or(false)
})
```

**This filtering requires BOTH:**
1. Property class match (works)
2. **Source series exact match** (breaks if context is wrong)

---

## 7. The Bug Scenario

### 7.1 What Happens When Loading frmr_one Article

1. User selects Framery ONE article
2. Code looks up property classes for article: `[MG_GLOBAL, MG_PROPERTIES]`
3. Code calls `get_properties_for_class("MG_PROPERTIES", "frmr_one")`
4. Filter checks:
   - Property class = "MG_PROPERTIES"? ✓
   - Source series = "frmr_one"? ✓
5. **Works correctly**

### 7.2 What Happens in Wrong Context

If the series parameter is wrong or comes from a different source:

1. Code calls `get_properties_for_class("MG_PROPERTIES", "FRMR_ONE")` (uppercase)
2. Filter checks:
   - Property class = "MG_PROPERTIES"? ✓
   - Source series = "FRMR_ONE" vs stored "frmr_one"? **✗ (case mismatch)**
3. **No properties returned**

OR:

1. Code calls `get_properties_for_class("MG_PROPERTIES", "framery")` (manufacturer instead of series)
2. Filter checks:
   - Property class = "MG_PROPERTIES"? ✓
   - Source series = "framery" vs stored "frmr_one"? **✗**
3. **No properties returned**

---

## 8. Additional Data Points

### 8.1 Article Structure

Example from frmr_one:
```
Article: ONE
  Series: FRMR_ONE (uppercase!)
  Manufacturer: FRAMERY
  Art Type: C (configurable)
  Property Classes: [MG_GLOBAL, MG_PROPERTIES]
```

**Note:** Article's `series` field is `FRMR_ONE` (uppercase), but path series is `frmr_one` (lowercase).

### 8.2 Case Sensitivity Issue

- Path series: `frmr_one` (lowercase, from directory name)
- Article series field: `FRMR_ONE` (uppercase, from database)
- Filter uses `.to_lowercase()` for comparison, which should handle this
- **But** if series comes from article.series without normalization, mismatch occurs

---

## 9. Root Cause Analysis

### 9.1 Design Assumption (WRONG)

The series-based filtering assumes:
- Each series has unique property class names
- Property classes are scoped to series

**This is NOT true for Framery (and possibly other manufacturers).**

### 9.2 Actual Data Model

Framery uses:
- **Shared property classes** across series (manufacturer-level)
- **Article-to-property-class mapping** to determine which classes apply to which articles
- The `ocd_propertyclass` table explicitly maps articles to classes

### 9.3 Correct Approach

Instead of filtering properties by series, should:
1. Look up which property classes the article uses (from `ocd_propertyclass`)
2. Get ALL properties for those classes (no series filter)
3. Let the article mapping control scope, not series matching

---

## 10. Recommended Solution

### 10.1 Remove Series-Based Property Filtering

The `source_series` filter in `get_properties_for_class()` should be **removed or made optional**.

### 10.2 Use Article-Based Scoping

When configuring a product:
1. Get article's property classes from `ocd_propertyclass` table
2. For each property class, get ALL properties (ignoring source_series)
3. The article mapping already controls which properties are valid

### 10.3 Alternative: Hybrid Approach

Keep series filtering as an **optimization** for manufacturers that need it (like FAST with large datasets), but:
1. Detect if property classes are shared vs series-specific
2. Only apply series filter if classes are series-specific
3. Use article-based scoping for shared classes

### 10.4 Detection Logic

```rust
fn property_classes_are_series_specific(manufacturer_path: &Path) -> bool {
    // Load all series
    // Check if property class names contain series identifier
    // OR: Check if same class name appears in multiple series with different properties
    // Return true only if truly series-specific
}
```

---

## 11. Impact Assessment

### 11.1 Affected Manufacturers

Need to check which manufacturers use shared property classes:
- ✓ **Framery** - confirmed shared classes
- ? **FAST** - uses "Rahmen" across series (might be shared too)
- ? **Other manufacturers** - need investigation

### 11.2 Test Cases Needed

1. Framery ONE article configuration
2. Framery 2Q article configuration
3. FAST KR article configuration (regression test)
4. Cross-series property access
5. Case sensitivity (FRMR_ONE vs frmr_one)

---

## 12. Files Requiring Changes

1. **`src/oap/ocd_properties.rs`**
   - Line 1014-1021: `get_properties_for_class()` filter logic
   - Line 1289: `source_series` assignment (possibly keep for metadata)

2. **`src/oap/families.rs`**
   - Already uses `load_article_property_classes()` ✓
   - Verify property class lookup uses article mapping

3. **`src/oap/engine.rs`**
   - Verify configuration engine uses correct property lookup
   - Check series parameter source

---

## 13. Data Quality Notes

### 13.1 Accessories Series

The `accessories` series has:
- 1 article: "Framery Movability Kit"
- **No property classes** (empty ocd_property table)
- This is a simple, non-configurable product

### 13.2 Flip-Fold Variant

The `frmr_q` series has special article `Q_FLIP_FOLD` that uses `MG_PROPERTIES_FLIP_FOLD` instead of `MG_PROPERTIES`. This suggests:
- Some customization per product line
- Shared base + specialized variants pattern

---

## 14. Validation Commands

To verify the data structure:

```bash
# Check property classes for each series
for series in frmr_one frmr_2q frmr_o frmr_four; do
  echo "=== $series ==="
  cargo run -- ebase /reference/ofmldata/framery/$series/ANY/1/db/pdata.ebase ocd_property | grep "prop_class:"
done

# Check article mappings
cargo run -- ebase /reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase ocd_propertyclass

# Check pricing
cargo run -- ebase /reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase ocd_price
```

---

## 15. Conclusion

The series-based property filtering is **architecturally incompatible** with Framery's shared property class model. The fix requires either:

1. **Remove series filtering** entirely and rely on article-to-property-class mappings
2. **Make filtering optional** based on manufacturer data structure detection
3. **Use hybrid approach** with fallback to article-based scoping

**Recommendation:** Option 1 (remove series filtering) is safest and aligns with the actual OCD data model where `ocd_propertyclass` table explicitly defines article-to-class relationships.
