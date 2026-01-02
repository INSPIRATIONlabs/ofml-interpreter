# OFML Data Structure Examples

Concrete examples from actual OFML data showing real-world structures and data patterns.

## Example 1: Sedus AI Chair (Standard Structure)

### Directory Layout

```
/reference/ofmldata/sex/ai/
├── 1/                          # Version 1 (language-independent)
│   ├── odb.ebase              # 24 KB - Object database
│   ├── ofml.ebase             # 2 KB - OFML metadata
│   └── sex_ai_1.alb           # 6 MB - Geometry archive (50 files)
└── DE/                         # German language data
    └── 1/                      # Version 1
        ├── db/
        │   └── pdata.ebase    # 4 MB - OCD pricing data
        ├── oam/
        │   └── oam.ebase      # 2.5 KB - Article mapping
        ├── cat/
        │   └── xcf.zip        # 2 KB - Catalog data
        ├── image/
        │   └── image.zip      # 18 KB - Product images
        └── etc/
            ├── p_se_air_de.pdf
            ├── p_se_air_en.pdf
            ├── p_se_air_es.pdf
            ├── p_se_air_fr.pdf
            ├── p_se_air_it.pdf
            └── p_se_air_nl.pdf
```

### pdata.ebase Content (24 tables)

**Article Data:**
- `ocd_article`: 2 base articles
- `ocd_artshorttext`: 12 short descriptions
- `ocd_artlongtext`: 24 long descriptions

**Pricing:**
- `ocd_price`: 60 price entries (base + surcharges)
- `ocd_pricetext`: 60 price descriptions

**Configuration:**
- `ocd_propertyclass`: 4 property classes
- `ocd_property`: 296 properties
- `ocd_propertyvalue`: 7,536 property values
- `ocd_propertytext`: 673 property labels
- `ocd_propvaluetext`: 40,488 value labels

**Relations:**
- `ocd_relation`: 32,775 relationships
- `ocd_relationobj`: 6,274 relation objects

**Custom Table:**
- `s_mod_var_stuhl_tbl`: 7,651 chair modifier records

### odb.ebase Content (7 tables)

**3D Objects:**
- `odb3d`: 112 3D object definitions
  - Fields: odb_name (7 chars), obj_name (25 chars), visible, offsets, rotations, ctor, mat, attrib, link
  - Record size: 124 bytes

**2D Objects:**
- `odb2d`: 1 2D drawing definition

**Layers:**
- `layer`: 16 layer definitions

**Functions:**
- `funcs`: 22 CLS functions for dynamic behavior

**Attachment Points:**
- `attpt`: 0 records (not used in this series)
- `stdattpt`: 0 records
- `oppattpt`: 0 records

### ofml.ebase Content (3 tables)

**Metadata:**
- `proginfo`: 10 program info records
- `plelement`: 1 product element
- `epdfproductdb`: 4 ePDF config entries

### oam.ebase Content (3 tables)

**Mappings:**
- `oam_article2ofml`: 2 article-to-OFML mappings
- `oam_property2mat`: 20 property-to-material mappings
- `oam_article2odbparams`: 0 records

### ALB Archive Contents (50 files)

**3D Geometry (22 OBJ files):**
```
sex_ai_d3_ch_castor_tread_65.obj      (127 KB)
sex_ai_d3_ch_castor_cover_65.obj      (74 KB)
sex_ai_d3_ch_pipe.obj                 (13 KB)
sex_ai_d3_ch_pipe_fix.obj             (43 KB)
sex_ai_d3_ch_footframe_1_counter.obj  (166 KB)
sex_ai_d3_ch_lordose.obj              (218 KB)
sex_ai_d3_ch_lever.obj                (385 KB)
sex_ai_d3_ch_seat_shell_1.obj         (1.9 MB)
sex_ai_d3_ch_back_bail_1.obj          (1.7 MB)
... (13 more files)
```

**2D CAD (24 DWG files):**
```
sex_ai_d2_snap.dwg                    (23 KB)
sex_ai_d2_ch_ai_121.dwg               (25 KB)
sex_ai_d3_ch_seat_shell_1.dwg         (1.4 MB)
sex_ai_d3_ch_back_bail_1.dwg          (1.2 MB)
... (20 more files)
```

**Scene Data (2 EGMS files):**
```
sex_ai_d2_snap.egms                   (224 bytes)
sex_ai_d2_ch_ai_121.egms              (9 KB)
```

**Metadata (2 files):**
```
sex                                   (0 bytes - directory marker)
ai                                    (0 bytes - directory marker)
```

---

## Example 2: Vitra Meda (Extended Tables)

### Directory Layout

```
/reference/ofmldata/vitra/meda/
├── 1/
│   ├── odb.ebase
│   ├── ofml.ebase
│   └── vitra_meda_1.alb
└── DE/
    └── 1/
        ├── db/
        │   └── pdata.ebase    # 32 tables (more than Sedus)
        └── oam/
            └── oam.ebase
```

### pdata.ebase Extended Tables (32 tables vs 24)

**Additional Tables Not in Sedus:**
- `ocd_composite` - Composite article definitions
- `ocd_billofitems` - Bill of materials
- `ocd_identification` - Product identification data
- `ocd_propertygroup` - Property grouping
- `ocd_article2propgroup` - Article-to-group mappings
- `vitra_availability` - Manufacturer-specific availability
- `propvalue2varcond` - Property value to variant condition mapping
- `property_map` - Custom property mappings
- `propinfo` - Additional property information
- `zv00613_tab_10001_tbl` - Custom data table

---

## Example 3: Knoll ConLine (Version 2, Custom Tables)

### Directory Layout

```
/reference/ofmldata/kn/conline/
└── DE/
    └── 2/                      # Version 2 (not 1)
        ├── db/
        │   └── pdata.ebase    # 35 tables
        └── oam/
            └── oam.ebase
```

### pdata.ebase Custom Tables (35 tables)

**Knoll-Specific Tables:**
- `vb_elektro_ex_matn_tbl`: 8,310 electrical material records
- `farben_conline_tbl`: 290 color table records
- `art2aclass_map`: 353 article-to-class mappings
- `optproperty_dat`: 123 optional property definitions

**Record Structure Example (`vb_elektro_ex_matn_tbl`):**
- Record Size: 40 bytes
- Columns: line (string), name (string), value (string)

**Price Table:**
- `ocd_price`: 336 price entries
- Columns: article_nr, var_cond, price_type, price_level, price_rule, price_textnr, price (float64), is_fix, currency, date_from, date_to

---

## Example 4: Manufacturer with Global Data

### Sedus Global Directory

```
/reference/ofmldata/sex/global/
└── 1/
    ├── global.ebase           # 562 KB - Shared manufacturer data
    └── sex_global_1.alb       # 1 KB - Shared geometry
```

### global.ebase Content (24 tables)

**Shared Data Tables:**
- `go_freenumeric`: 3 records - Free numeric configurations
- Material definitions
- Finish catalogs
- Common components

This data is referenced by multiple series within the Sedus manufacturer.

---

## Example 5: Large Series with Basics

### Sedus Basics Series

```
/reference/ofmldata/sex/basics/
├── 1/
│   └── sex_basics_1.alb       # 575 MB - Very large archive!
└── ANY/                        # Language-neutral
    └── 1/
        └── ... (data files)
```

**Note:** The `basics` series uses `ANY` instead of `DE/EN/IT` for language code, indicating language-neutral content.

---

## Real Data Samples

### ocd_article Record

From `/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase`:

```
Record Size: 40 bytes
Columns:
  - article_nr: string_ref (offset=0)    # e.g., "SE_AI_D2"
  - art_type: string_ref (offset=4)      # e.g., "B" (base)
  - manufacturer: string_ref (offset=8)   # e.g., "Sedus"
  - series: string_ref (offset=12)        # e.g., "ai"
  - short_textnr: string_ref (offset=16)  # Reference to ocd_artshorttext
  - long_textnr: string_ref (offset=20)   # Reference to ocd_artlongtext
  - rel_obj: uint32 (offset=24)           # Relation object ID
  - fast_supply: uint16 (offset=28)       # Fast supply flag
  - discountable: uint8 (offset=30)       # Discount allowed
  - order_unit: string_ref (offset=32)    # e.g., "ST" (piece)
  - scheme_id: string_ref (offset=36)     # Variant code scheme
```

### ocd_price Record

```
Record Size: 64 bytes
Columns:
  - article_nr: string_ref               # Article reference
  - var_cond: string_ref                 # Variant condition (e.g., "S_PGX", "S_166")
  - price_type: string_ref               # e.g., "P" (sales price)
  - price_level: string_ref              # "B" (base) or "X" (surcharge)
  - price_rule: string_ref               # Calculation rule
  - price_textnr: string_ref             # Description reference
  - price: float64                       # Actual price value
  - is_fix: uint8                        # Fixed price flag
  - currency: string_ref                 # e.g., "EUR"
  - date_from: string_ref                # Validity start
  - date_to: string_ref                  # Validity end
  - scale_quantity: uint16               # Quantity discount threshold
  - rounding_id: string_ref              # Rounding rule reference
```

### ocd_propertyvalue Record

```
Record Size: 52 bytes
Columns:
  - prop_class: string_ref               # Property class ID
  - property: string_ref                 # Property ID (e.g., "COLOR")
  - pos_pval: uint16                     # Position/order
  - pval_textnr: string_ref              # Value label reference
  - rel_obj: uint32                      # Relation object
  - is_default: uint8                    # Default selection
  - suppress_txt: uint8                  # Hide text flag
  - op_from: string_ref                  # Range operator
  - value_from: string_ref               # Value or range start
  - op_to: string_ref                    # Range operator
  - value_to: string_ref                 # Range end
  - raster: string_ref                   # Value increment
  - date_from: string_ref                # Availability start
  - date_to: string_ref                  # Availability end
```

### odb3d Record

From `/reference/ofmldata/sex/ai/1/odb.ebase`:

```
Record Size: 124 bytes
Columns:
  - odb_name: string (7 chars)           # e.g., "ch_seat"
  - obj_name: string (25 chars)          # e.g., "sex_ai_d3_ch_seat_shell_1"
  - visible: string (1 char)             # "V" = visible, "H" = hidden
  - x_offs: string (15 chars)            # X offset expression
  - y_offs: string (7 chars)             # Y offset expression
  - z_offs: string (15 chars)            # Z offset expression
  - x_rot: string (7 chars)              # X rotation expression
  - y_rot: string (7 chars)              # Y rotation expression
  - z_rot: string (7 chars)              # Z rotation expression
  - ctor: string_ref                     # Constructor CLS code
  - mat: string_ref                      # Material reference
  - attrib: string_ref                   # Attributes
  - link: string (4 chars)               # Link ID
```

### oam_article2ofml Record

From `/reference/ofmldata/sex/ai/DE/1/oam/oam.ebase`:

```
Record Size: 24 bytes
Columns:
  - article: string_ref (offset=0)       # Article number (e.g., "SE_AI_D2")
  - ofml_type: string_ref (offset=8)     # OFML type (e.g., "OAP_ARTICLE")
  - odb_name: string_ref (offset=16)     # ODB object name
  - params: string_ref (offset=20)       # Additional parameters
```

### oam_property2mat Record

```
Record Size: 20 bytes
Columns:
  - article: string_ref                  # Article number
  - property: string_ref                 # Property ID (e.g., "FABRIC")
  - prop_value: string_ref               # Property value (e.g., "166")
  - mat_layer: string_ref                # Material layer name
  - material: string_ref                 # Material file reference
```

---

## Table Relationship Diagram

```
ocd_article
    ├─> ocd_artshorttext (via short_textnr)
    ├─> ocd_artlongtext (via long_textnr)
    ├─> ocd_price (via article_nr)
    ├─> ocd_propertyclass (via article_nr)
    ├─> ocd_articletaxes (via article_nr)
    └─> ocd_relationobj (via rel_obj)
         └─> ocd_relation (via rel_name)

ocd_propertyclass
    └─> ocd_property (via prop_class)
         ├─> ocd_propertytext (via prop_textnr)
         └─> ocd_propertyvalue (via prop_class + property)
              └─> ocd_propvaluetext (via pval_textnr)

ocd_price
    └─> ocd_pricetext (via price_textnr)

oam_article2ofml
    ├─> ocd_article (via article)
    └─> odb.odb3d (via odb_name)

oam_property2mat
    ├─> ocd_propertyvalue (via property + prop_value)
    └─> material files (via material)
```

---

## Data Loading Sequence

### Recommended Load Order

1. **Load ofml.ebase** - Get product metadata
   - `proginfo` - Series information
   - `plelement` - Product structure

2. **Load odb.ebase** - Get geometry references
   - `odb3d` - 3D object names and transforms
   - `odb2d` - 2D drawing references
   - `funcs` - CLS functions
   - `layer` - Layer definitions

3. **Load pdata.ebase** - Get pricing and configuration
   - `ocd_version` - Verify data version
   - `ocd_article` - Base articles
   - `ocd_propertyclass` - Property structure
   - `ocd_property` - Property definitions
   - `ocd_propertyvalue` - Available values
   - `ocd_price` - Pricing data
   - All text tables for labels

4. **Load oam.ebase** - Get mappings
   - `oam_article2ofml` - Article-to-geometry mapping
   - `oam_property2mat` - Material assignments

5. **Load ALB (as needed)** - Extract geometry
   - Extract specific OBJ files on demand
   - Don't extract everything upfront

6. **Load global.ebase (if exists)** - Shared data
   - Load once per manufacturer
   - Cache for all series

---

## Common Query Patterns

### Get All Articles
```sql
SELECT article_nr, art_type, manufacturer, series
FROM ocd_article
```

### Get Article Base Price
```sql
SELECT p.price, p.currency, t.text
FROM ocd_price p
JOIN ocd_pricetext t ON p.price_textnr = t.textnr
WHERE p.article_nr = 'SE_AI_D2'
  AND p.price_level = 'B'
  AND p.var_cond IN ('S_PGX', 'BASE', 'STANDARD', '')
  AND t.language = 'DE'
```

### Get Property Values for Article
```sql
SELECT pc.prop_class, p.property, pt.text, pv.value_from, pvt.text
FROM ocd_propertyclass pc
JOIN ocd_property p ON pc.prop_class = p.prop_class
JOIN ocd_propertytext pt ON p.prop_textnr = pt.textnr
JOIN ocd_propertyvalue pv ON p.prop_class = pv.prop_class AND p.property = pv.property
JOIN ocd_propvaluetext pvt ON pv.pval_textnr = pvt.textnr
WHERE pc.article_nr = 'SE_AI_D2'
  AND pt.language = 'DE'
  AND pvt.language = 'DE'
ORDER BY p.pos_prop, pv.pos_pval
```

### Get 3D Objects for Article
```sql
-- First, get odb_name from oam
SELECT odb_name FROM oam_article2ofml WHERE article = 'SE_AI_D2'

-- Then, get 3D objects
SELECT odb_name, obj_name, visible, x_offs, y_offs, z_offs
FROM odb3d
WHERE odb_name = '<result from above>'
```

### Get Material for Property Value
```sql
SELECT mat_layer, material
FROM oam_property2mat
WHERE article = 'SE_AI_D2'
  AND property = 'FABRIC'
  AND prop_value = '166'
```

---

## File Size Ranges

| File | Typical Size | Range | Notes |
|------|--------------|-------|-------|
| `pdata.ebase` | 1-50 MB | 100 KB - 200 MB | Depends on property complexity |
| `odb.ebase` | 20-500 KB | 5 KB - 5 MB | Depends on object count |
| `ofml.ebase` | 2-10 KB | 1 KB - 50 KB | Usually small |
| `oam.ebase` | 2-20 KB | 1 KB - 100 KB | Usually small |
| `global.ebase` | 100 KB - 2 MB | 50 KB - 10 MB | Shared data |
| `*.alb` | 1-50 MB | 1 KB - 600 MB | Geometry archives |
| `mt.ebase` | 10-100 KB | 5 KB - 1 MB | Metadata tables |

---

## Performance Considerations

### Large Tables
- `ocd_propvaluetext` - Can have 40,000+ records
- `ocd_relation` - Can have 30,000+ records
- `ocd_propertyvalue` - Can have 10,000+ records

### Optimization Tips
1. **Index by article_nr** - Most queries filter by article
2. **Cache text tables** - Load once, reuse for all articles
3. **Lazy load geometry** - Don't extract ALB until needed
4. **Stream large tables** - Don't load all records into memory
5. **Share global data** - Load once per manufacturer

---

**Generated**: December 31, 2024
**Source**: Actual OFML data analysis from `/reference/ofmldata/`
