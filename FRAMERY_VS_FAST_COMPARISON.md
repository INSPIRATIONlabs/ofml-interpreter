# Framery vs FAST: Property Class Organization

## Quick Comparison Table

| Aspect | Framery | FAST |
|--------|---------|------|
| **Property Class Names** | `MG_GLOBAL`, `MG_PROPERTIES` | `Rahmen` |
| **Shared Across Series?** | **YES** (7 of 8 series) | **YES** (same class name) |
| **Series in Class Name?** | **NO** | **NO** |
| **Series Count** | 8 | 8+ |
| **pdata.ebase per Series** | Yes | Yes |

## The Problem

```
Current Code (ocd_properties.rs:1014-1021):
  Filter properties by:
    1. Property class name (e.g., "MG_PROPERTIES") ✓
    2. Source series match (e.g., "frmr_one") ✗ BREAKS FRAMERY
```

## Why It Breaks Framery

```
Framery Data Structure:

  /framery/frmr_one/ANY/1/db/pdata.ebase
    └── Property: "M_EXTERIOR"
        ├── Class: "MG_PROPERTIES"
        ├── Source Series: "frmr_one" (tagged during load)
        └── Values: [RAL9016MAT, RAL9005, ...]

  /framery/frmr_2q/ANY/1/db/pdata.ebase
    └── Property: "M_EXTERIOR"
        ├── Class: "MG_PROPERTIES" (SAME CLASS!)
        ├── Source Series: "frmr_2q" (different tag)
        └── Values: [RAL9016MAT, RAL9005, ...] (might be same values)

When filtering for frmr_one:
  - Looks for: class="MG_PROPERTIES" AND source_series="frmr_one"
  - Finds: Only frmr_one properties
  - Misses: Any properties loaded from other series
  - Result: Missing options if context is wrong
```

## Why It (Accidentally) Works for FAST

```
FAST Data Structure:

  /fast/kr/DE/1/db/pdata.ebase
    └── Property: "Artikelnummer"
        ├── Class: "Rahmen"
        ├── Source Series: "kr"
        └── Values: [sG-Kr-Km-3er-ES, ...]

  /fast/wkm/DE/1/db/pdata.ebase
    └── Property: "Artikelnummer"
        ├── Class: "Rahmen" (SAME CLASS!)
        ├── Source Series: "wkm"
        └── Values: [different values]

When filtering for kr:
  - Looks for: class="Rahmen" AND source_series="kr"
  - Finds: Only kr properties
  - Result: Works, but only because:
    1. Each series has different property VALUES
    2. Series context is always correct
    3. No cross-series queries happen
```

## The Real OCD Data Model

According to OCD 4.3 spec, the correct relationship is:

```
ocd_article
  └── article_nr: "ONE"

ocd_propertyclass (mapping table)
  └── article_nr: "ONE" → prop_class: ["MG_GLOBAL", "MG_PROPERTIES"]

ocd_property
  └── prop_class: "MG_PROPERTIES"
      └── property: "M_EXTERIOR"

ocd_propertyvalue
  └── (prop_class, property): ("MG_PROPERTIES", "M_EXTERIOR")
      └── values: [RAL9016MAT, RAL9005, ...]
```

**The mapping is article → property classes, NOT series → property classes!**

## Solution

### Option 1: Remove Series Filtering (RECOMMENDED)

```rust
// OLD (ocd_properties.rs:1014-1021)
.filter(|((pc, _), def)| {
    pc == prop_class
        && def.source_series.as_ref().map(|s| s.to_lowercase() == series_lower).unwrap_or(false)
})

// NEW
.filter(|((pc, _), def)| pc == prop_class)
```

Let the `ocd_propertyclass` table (article-to-class mapping) control scope.

### Option 2: Make It Optional

```rust
pub fn get_properties_for_class(&self, prop_class: &str, series: Option<&str>) -> Vec<&OcdPropertyDefinition> {
    self.properties
        .iter()
        .filter(|((pc, _), def)| {
            if pc != prop_class {
                return false;
            }
            if let Some(series_filter) = series {
                def.source_series.as_ref().map(|s| s.to_lowercase() == series_filter.to_lowercase()).unwrap_or(false)
            } else {
                true  // No series filter
            }
        })
        .map(|(_, def)| def)
        .collect()
}
```

### Option 3: Auto-Detect

```rust
// Detect if manufacturer uses shared property classes
let uses_shared_classes = detect_shared_property_classes(manufacturer_path);

// Only apply series filter if NOT shared
if uses_shared_classes {
    get_properties_for_class(prop_class, None)  // No filter
} else {
    get_properties_for_class(prop_class, Some(series))  // Filter by series
}
```

## Recommended Action

**Remove series-based filtering** and rely on the `ocd_propertyclass` article-to-class mappings.

This aligns with the OCD specification and works for both:
- Manufacturers with shared property classes (Framery)
- Manufacturers with series-specific property classes (if any exist)
