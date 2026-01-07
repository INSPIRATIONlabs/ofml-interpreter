# Framery Data Examples

Raw data excerpts showing the actual structure.

## 1. Property Classes (ocd_property)

### frmr_one Series

```
Property Classes Found: MG_GLOBAL, MG_PROPERTIES

Sample Properties in MG_PROPERTIES:
  pos_prop: 1   property: 'M_EXTERIOR'
  pos_prop: 2   property: 'M_INTERIOR'
  pos_prop: 3   property: 'M_CARPET'
  pos_prop: 4   property: 'M_TABLE'
  pos_prop: 5   property: 'M_SEAT_UPHOLSTERY'
  pos_prop: 6   property: 'M_TABLE_TOP'
  pos_prop: 7   property: 'M_EXTERIOR_STRUCTURE'
  pos_prop: 8   property: 'M_VENTILATION'
  pos_prop: 9   property: 'M_DOOR_OPENING'
  pos_prop: 10  property: 'M_SEISMIC_KIT'
  pos_prop: 11  property: 'M_DUAL_POWER_SUPPLY'
  pos_prop: 12  property: 'M_SOUND_DAMPENING_PAD'
  pos_prop: 13  property: 'M_WIRELESS_QI_CHARGER'
  pos_prop: 14  property: 'M_INDUSTRIAL_VENTILATION'
  pos_prop: 15  property: 'M_OCCUPANCY_SENSOR'
  pos_prop: 16  property: 'M_EXTENDED_CABLE'
  pos_prop: 17  property: 'M_CABLE_COVER'
  pos_prop: 18  property: 'M_MONITOR_WALL_MOUNT'
```

### frmr_2q Series

```
Property Classes Found: MG_GLOBAL, MG_PROPERTIES

Sample Properties in MG_PROPERTIES:
  (Similar or identical to frmr_one)
```

**Observation:** Both series use the SAME property class names.

---

## 2. Property Values (ocd_propertyvalue)

### Exterior Color Options (M_EXTERIOR)

```
prop_class: 'MG_PROPERTIES'
property: 'M_EXTERIOR'
Values (61 total in frmr_one):
  pos_pval: 0   value_from: 'RAL9016MAT'    (White matte)
  pos_pval: 1   value_from: 'RAL9005'       (Black)
  pos_pval: 2   value_from: 'S7500N'        (Gray)
  pos_pval: 3   value_from: 'NCSS7020R90B'
  pos_pval: 4   value_from: 'NCS3421R86B'
  pos_pval: 5   value_from: 'NCSS2010Y20R'
  pos_pval: 6   value_from: 'NCS2728R01B'
  pos_pval: 7   value_from: 'S7010G10Y'
  ... (many more NCS/RAL color codes)
```

---

## 3. Article Definitions (ocd_article)

### frmr_one Series Articles

```
Record 0:
  article_nr: 'ONE'
  series: 'FRMR_ONE'          ← Uppercase in database
  manufacturer: 'FRAMERY'
  art_type: 'C'                ← Configurable
  rel_obj: 10000
  scheme_id: 'SCHEME_1'

Record 1:
  article_nr: 'ONE_PREMIUM'
  series: 'FRMR_ONE'
  manufacturer: 'FRAMERY'
  art_type: 'C'
  rel_obj: 10000
  scheme_id: 'SCHEME_2'

Record 2:
  article_nr: 'ONE_LOUNGE'
  series: 'FRMR_ONE'
  manufacturer: 'FRAMERY'
  art_type: 'C'
  rel_obj: 10000
  scheme_id: 'SCHEME_3'
```

**Note:** `series` field is `FRMR_ONE` (uppercase), but directory path is `frmr_one` (lowercase).

---

## 4. Article-to-Property-Class Mappings (ocd_propertyclass)

### frmr_one Mappings

```
Record 0:
  article_nr: 'ONE'
  prop_class: 'MG_GLOBAL'
  pos_class: 1

Record 1:
  article_nr: 'ONE'
  prop_class: 'MG_PROPERTIES'
  pos_class: 2

Record 2:
  article_nr: 'ONE_PREMIUM'
  prop_class: 'MG_GLOBAL'
  pos_class: 1

Record 3:
  article_nr: 'ONE_PREMIUM'
  prop_class: 'MG_PROPERTIES'
  pos_class: 2

Record 4:
  article_nr: 'ONE_LOUNGE'
  prop_class: 'MG_GLOBAL'
  pos_class: 1

Record 5:
  article_nr: 'ONE_LOUNGE'
  prop_class: 'MG_PROPERTIES'
  pos_class: 2
```

**All 3 articles in frmr_one series map to the same 2 property classes.**

### frmr_2q Mappings

```
Article: '2Q_HUDDLE'              -> Classes: ['MG_GLOBAL', 'MG_PROPERTIES']
Article: '2Q_LOUNGE'              -> Classes: ['MG_GLOBAL', 'MG_PROPERTIES']
Article: '2Q_WITHOUT_FURNITURE'   -> Classes: ['MG_GLOBAL', 'MG_PROPERTIES']
```

**All 3 articles in frmr_2q series map to the same 2 property classes (same as frmr_one!).**

---

## 5. Pricing (ocd_price)

### Price Records from frmr_one (17 total)

```
Record 0:
  article_nr: '*'                                   ← Wildcard
  var_cond: 'PG_SEAT_UPHOLSTERY_OPTION_COLOR'
  price_level: 'X'                                   ← Surcharge
  price: (float value)
  currency: 'EUR'

Record 1:
  article_nr: '*'
  var_cond: 'PG_CARPET_OPTION_COLOR'
  price_level: 'X'
  price: (float value)
  currency: 'EUR'

Record 2:
  article_nr: '*'
  var_cond: 'PG_TABLE_TOP_OPTION_COLOR'
  price_level: 'X'
  price: (float value)
  currency: 'EUR'

Record 3:
  article_nr: '*'
  var_cond: 'PG_INTERIOR_PANEL_OPTION_COLOR'
  price_level: 'X'
  price: (float value)
  currency: 'EUR'

Record 4:
  article_nr: '*'
  var_cond: 'PG_EXTERIOR_PANEL_OPTION_COLOR'
  price_level: 'X'
  price: (float value)
  currency: 'EUR'

... (more surcharge records)
```

**Pattern:**
- Most prices use wildcard article `*`
- `var_cond` codes start with `PG_` prefix
- All are surcharges (level X)
- No base prices found (base might be in different table or use different pattern)

---

## 6. Cross-Series Property Comparison

### Properties in frmr_one vs frmr_2q

Both series have `MG_PROPERTIES` class with these common properties:
- M_EXTERIOR
- M_INTERIOR
- M_CARPET
- M_SEAT_UPHOLSTERY
- M_TABLE_TOP
- (and more)

**The property definitions are likely identical or very similar across series.**

This confirms that `MG_PROPERTIES` is a **manufacturer-level shared property class**, not series-specific.

---

## 7. Special Case: frmr_q Series

```
Property Classes Found: MG_GLOBAL, MG_PROPERTIES, MG_PROPERTIES_FLIP_FOLD

Articles:
  Q_MEETING_MAGGIE         -> Classes: [MG_GLOBAL, MG_PROPERTIES]
  Q_MEETING_MAGGIE_PREMIUM -> Classes: [MG_GLOBAL, MG_PROPERTIES]
  Q_FLOW                   -> Classes: [MG_GLOBAL, MG_PROPERTIES]
  Q_FLIP_FOLD              -> Classes: [MG_GLOBAL, MG_PROPERTIES_FLIP_FOLD]  ← Different!
```

**Observation:** One article (Q_FLIP_FOLD) uses a specialized property class `MG_PROPERTIES_FLIP_FOLD` instead of the standard `MG_PROPERTIES`. This shows:
- Shared base property classes (MG_GLOBAL, MG_PROPERTIES)
- Specialized variants for specific products (MG_PROPERTIES_FLIP_FOLD)

---

## 8. Data File Locations

All Framery series follow this pattern:

```
/reference/ofmldata/framery/{series}/ANY/1/db/pdata.ebase

Examples:
  /reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/frmr_o/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/frmr_four/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/frmr_six/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/frmr_one_compact/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/frmr_q/ANY/1/db/pdata.ebase
  /reference/ofmldata/framery/accessories/ANY/1/db/pdata.ebase
```

**Note:** Uses `ANY` country code, not `DE` like FAST.

---

## 9. Key Takeaway

The data clearly shows:

1. **Property classes are SHARED** across series (same class names)
2. **Articles map to property classes** via `ocd_propertyclass` table
3. **Series information is in the path**, not in property class names
4. **Case sensitivity matters**: database has `FRMR_ONE`, path has `frmr_one`

The correct approach is to:
- Use `ocd_propertyclass` to determine which classes an article uses
- Get ALL properties for those classes (no series filtering)
- Let the article mapping control scope
