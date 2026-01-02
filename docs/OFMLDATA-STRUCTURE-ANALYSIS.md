# OFML Data Directory Structure Analysis

Comprehensive analysis of `/reference/ofmldata/` directory structure based on investigation of actual data (Dec 2024).

## Executive Summary

The OFML data repository contains furniture product data from **117+ manufacturers** using a hierarchical structure with multiple data formats. Key findings:

- **14,235 EBase files** across different types (pdata, odb, ofml, oam, etc.)
- **2,706 ALB archives** containing 3D geometry and CAD data
- **Multiple directory patterns** based on manufacturer and series organization
- **Language-specific data** (primarily DE, EN, IT, ANY)
- **Version-controlled series** (mostly version 1, some version 2)

## Directory Organization

### Top-Level Structure

```
/reference/ofmldata/
├── {mfr_id}/              # Manufacturer ID (e.g., sex, kn, vitra)
│   ├── {series}/          # Product series (e.g., ai, jet3, meda)
│   │   ├── {lang}/        # Language code (DE, EN, IT, ANY)
│   │   │   └── {version}/ # Version number (1, 2, 3)
│   │   │       ├── db/    # Database files (pdata.ebase)
│   │   │       ├── oam/   # Article mapping (oam.ebase)
│   │   │       ├── cat/   # Catalog data (xcf.zip)
│   │   │       ├── image/ # Product images (image.zip)
│   │   │       ├── etc/   # Documentation (PDFs)
│   │   │       └── meta/  # Metadata (mt.ebase) [optional]
│   │   └── {version}/     # Language-independent data
│   │       ├── odb.ebase  # Object database
│   │       ├── ofml.ebase # OFML metadata
│   │       └── *.alb      # Geometry archives
│   └── global/            # Shared manufacturer data
│       └── 1/
│           └── global.ebase
```

### Example Manufacturers

| Manufacturer | ID | Series Count | Notes |
|--------------|-----|--------------|-------|
| Sedus | `sex` | 48 | Includes global/, basics/, land/ |
| Knoll | `kn` | 97+ | Largest series collection |
| Vitra | `vitra` | 52 | Includes home collection |
| Steelcase/SBU | `sbu` | 178+ | Massive collection |
| Haworth | `haw` | (not counted) | |

## File Types Breakdown

### EBase Database Files (14,235 total)

| File | Count | Purpose | Location |
|------|-------|---------|----------|
| `odb.ebase` | 3,163 | Object database (3D/2D geometry refs) | `{series}/{version}/` |
| `ofml.ebase` | 3,075 | OFML metadata & product info | `{series}/{version}/` |
| `oam.ebase` | 3,029 | Article-to-model mapping | `{lang}/{version}/oam/` |
| `pdata.ebase` | 3,014 | **PRIMARY DATA** - OCD pricing & properties | `{lang}/{version}/db/` |
| `oap.ebase` | 458 | OAP configurator data | Various |
| `mt.ebase` | 373 | Metadata/material tables | `{lang}/{version}/meta/` |
| `md.ebase` | 59 | Model data | Various |
| `global.ebase` | 13 | Shared manufacturer data | `global/{version}/` |
| Others | ~1000 | Series-specific tables | Various locations |

### Archive Files

| Extension | Count | Purpose |
|-----------|-------|---------|
| `.alb` | 2,706 | ZIP archives with 3D models (OBJ, DWG, EGMS) |
| `.zip` | 1,266 | Catalog images, product photos |

### Geometry & CAD Files

| Extension | Count | Purpose |
|-----------|-------|---------|
| `.geo` | 18,454 | Proprietary geometry format |
| `.dwg` | 20,825 | AutoCAD 2D drawings |
| `.obj` | 133 | 3D mesh files (also inside ALB) |
| `.glb` | 46 | Binary glTF 3D models |
| `.3ds` | 1 | 3DS Max format |

### Material & Appearance

| Extension | Count | Purpose |
|-----------|-------|---------|
| `.mat` | 8,776 | Material definitions |
| `.jpg` | 93,509 | Textures, product images |
| `.png` | 9,832 | Textures, transparent images |

### Configuration & Data

| Extension | Count | Purpose |
|-----------|-------|---------|
| `.csv` | 9,989 | Data tables, property mappings |
| `.cfg` | 4,123 | Configuration files |
| `.xml` | 24 | Structured data |
| `.ini` | 32 | Settings |

### Other Files

| Extension | Count | Purpose |
|-----------|-------|---------|
| `.pdf` | 1,827 | Product documentation |
| `.egms` | 3,189 | EGMS geometry/scene files |
| `.pec` | 2,451 | PEC configuration files |
| `.cls` | 9 | CLS bytecode (product logic) |
| `.html/.htm` | 130 | Documentation |

## EBase Table Structure

### pdata.ebase (Primary OCD Data)

Standard OCD 4.3 tables found in most pdata.ebase files:

| Table | Records | Purpose |
|-------|---------|---------|
| `ocd_article` | 2-2000 | Article definitions (base products) |
| `ocd_price` | 60-10000+ | Pricing data (base + surcharges) |
| `ocd_pricetext` | 60-1000 | Price descriptions (multilingual) |
| `ocd_propertyclass` | 4-500 | Property class mappings |
| `ocd_property` | 26-500 | Property definitions |
| `ocd_propertyvalue` | 100-10000+ | Available property values |
| `ocd_propertytext` | 400-2000 | Property labels (multilingual) |
| `ocd_propvaluetext` | 1000-50000+ | Property value labels |
| `ocd_artshorttext` | 12-2000 | Short article descriptions |
| `ocd_artlongtext` | 24-1000 | Long article descriptions |
| `ocd_relation` | 100-50000 | Relationship definitions |
| `ocd_relationobj` | 100-10000 | Relationship objects |
| `ocd_version` | 1 | Data version info |
| `ocd_packaging` | 0-100 | Packaging specs |
| `ocd_articletaxes` | 0-50 | Tax information |
| `ocd_taxscheme` | 0-5 | Tax schemes |

**Manufacturer-Specific Tables** (examples):

- `s_mod_var_stuhl_tbl` - Sedus chair modifiers
- `vb_elektro_ex_matn_tbl` - Knoll electrical material tables
- `farben_conline_tbl` - Knoll Conline color tables
- `vitra_availability` - Vitra availability data
- `art2aclass_map` - Article to class mappings
- `propvalue2varcond` - Property value to variant condition mappings

Additional tables in some files:
- `ocd_composite` - Composite articles
- `ocd_billofitems` - Bill of materials
- `ocd_identification` - Product identification
- `ocd_propertygroup` - Property grouping
- `ocd_article2propgroup` - Article-to-property-group mapping
- `ocd_codescheme` - Variant code schemes
- `ocd_rounding` - Price rounding rules

### odb.ebase (Object Database)

Contains geometry and scene graph data:

| Table | Purpose |
|-------|---------|
| `odb3d` | 3D object definitions with transforms |
| `odb2d` | 2D drawing objects |
| `layer` | Layer definitions |
| `funcs` | CLS functions for dynamic objects |
| `attpt` | Attachment points |
| `stdattpt` | Standard attachment points |
| `oppattpt` | Opposite attachment points |

### ofml.ebase (OFML Metadata)

| Table | Purpose |
|-------|---------|
| `proginfo` | Program/series information |
| `plelement` | Product element definitions |
| `epdfproductdb` | ePDF product database config |

### oam.ebase (Article Mapping)

| Table | Purpose |
|-------|---------|
| `oam_article2ofml` | Maps articles to OFML objects |
| `oam_property2mat` | Maps property values to materials |
| `oam_article2odbparams` | Maps articles to ODB parameters |

### global.ebase (Shared Data)

Contains manufacturer-wide shared tables (materials, colors, etc.)

## ALB Archive Contents

ALB files are ZIP archives containing geometry and CAD files:

**Typical Contents:**
- **3D Geometry**: `.obj` files (mesh data)
- **2D CAD**: `.dwg` files (AutoCAD drawings)
- **Scene Data**: `.egms` files (EGMS scene definitions)
- **Metadata**: Manifest files

**Example from sex_ai_1.alb (50 files):**
- 22 `.obj` files (chair components)
- 24 `.dwg` files (2D CAD)
- 2 `.egms` files (scene data)
- 2 metadata files

## Language Support

Languages found in directory structure:

| Code | Language | Usage |
|------|----------|-------|
| `DE` | German | Primary (most common) |
| `EN` | English | Secondary |
| `IT` | Italian | Limited |
| `ANY` | Language-neutral | Special cases |

Most manufacturers use `DE` as the default. Some series use `ANY` for language-neutral data.

## Version Management

- **Version 1**: 2,426 instances (96% of data)
- **Version 2**: 91 instances (4% of data)
- **Version 3+**: Rare

Version numbers appear in path: `{lang}/{version}/`

## Special Directories

### Global Series

Some manufacturers have `global/` directories containing shared data:
- Materials
- Finishes
- Common components

Example: `/reference/ofmldata/sex/global/1/global.ebase`

### Basics/Standard Series

Common naming patterns:
- `basics` - Basic/standard components
- `land` - Landscape/planning elements
- `liefer` - Delivery/supply items
- `ver` - Version/variant data

### Catalog Series

Some manufacturers have catalog-only series:
- `catalog`
- `catseating`
- `catsystem`

These may not have full product data, only presentation content.

## Data Variations by Manufacturer

### Sedus (sex)
- 48 series
- Extensive use of custom tables (e.g., `s_mod_var_stuhl_tbl`)
- Has `global/` for shared data
- Version 1 throughout

### Knoll (kn)
- 97+ series
- Custom material tables (`vb_elektro_ex_matn_tbl`, `farben_conline_tbl`)
- Custom mappings (`art2aclass_map`)
- Mix of version 1 and 2

### Vitra (vitra)
- 52 series
- Additional tables: `vitra_availability`, `ocd_composite`, `ocd_billofitems`
- Property mapping extensions (`propvalue2varcond`, `property_map`)
- `global/` and `global2/` directories

### Steelcase/SBU (sbu)
- 178+ series (largest)
- Extensive meta/ directories
- Complex property systems

## File Locations Quick Reference

| Data Type | Path Template |
|-----------|---------------|
| OCD Pricing | `{mfr}/{series}/{lang}/{ver}/db/pdata.ebase` |
| Article Mapping | `{mfr}/{series}/{lang}/{ver}/oam/oam.ebase` |
| 3D Objects | `{mfr}/{series}/{ver}/odb.ebase` |
| OFML Metadata | `{mfr}/{series}/{ver}/ofml.ebase` |
| Geometry Archive | `{mfr}/{series}/{ver}/{mfr}_{series}_{ver}.alb` |
| Catalog Images | `{mfr}/{series}/{lang}/{ver}/cat/xcf.zip` |
| Product Images | `{mfr}/{series}/{lang}/{ver}/image/image.zip` |
| Documentation | `{mfr}/{series}/{lang}/{ver}/etc/*.pdf` |
| Metadata | `{mfr}/{series}/{lang}/{ver}/meta/mt.ebase` |
| Shared Data | `{mfr}/global/{ver}/global.ebase` |

## Implementation Requirements

A comprehensive OFML data extraction library must handle:

### 1. Directory Navigation
- Multiple hierarchy patterns (with/without language dirs)
- Version detection
- Series enumeration
- File type detection

### 2. EBase Reader
- All standard OCD tables
- Custom manufacturer tables
- Dynamic table discovery
- Multiple EBase versions (1.0, 1.1)

### 3. Table Types to Support

**Standard OCD Tables:**
- Article definitions
- Pricing (base + surcharges)
- Properties & property values
- Relations & relation objects
- Text tables (multilingual)
- Packaging & taxes

**Geometry Tables:**
- odb3d/odb2d objects
- Layers & functions
- Attachment points

**Mapping Tables:**
- Article-to-OFML
- Property-to-material
- Custom manufacturer mappings

### 4. Archive Handling
- ALB (ZIP) extraction
- Image ZIP files
- Catalog ZIP files

### 5. Special Cases
- Global/shared data loading
- Multi-version series
- Language fallbacks
- Missing files (graceful degradation)

### 6. Geometry Files
- GEO format reader
- DWG parsing (basic metadata)
- OBJ loading
- GLB/glTF support

## Data Quality Notes

### Consistency Issues
- Not all manufacturers use same table structures
- Some series missing expected files
- Version numbering inconsistent
- Language support varies

### Common Patterns
- Most use OCD 4.3 core tables
- Custom tables follow `{prefix}_{name}_tbl` pattern
- ALB naming: `{mfr}_{series}_{ver}.alb`
- ZIP archives in cat/, image/ directories

### Edge Cases
- Series with no pdata.ebase (catalog-only)
- Multiple language variations of same series
- Deeply nested manufacturer hierarchies
- Very large ALB files (>500MB in some cases)

## Recommended Reading Order

1. Load manufacturer list from directory scan
2. For each manufacturer, enumerate series
3. For each series, detect language/version structure
4. Load version-independent data (odb, ofml, alb)
5. Load language-specific data (pdata, oam)
6. Load global/shared data if present
7. Extract geometry from ALB as needed
8. Parse custom tables dynamically

## Testing Strategy

Essential test cases:
- **Sedus AI** (`sex/ai`) - Clean, standard structure
- **Vitra Meda** (`vitra/meda`) - Extended tables, composites
- **Knoll ConLine** (`kn/conline`) - Custom tables, version 2
- **SBU Series** - Metadata handling
- **Series with global/** - Shared data loading
- **Multi-language series** - Language detection

## File Count Statistics

Total files by category:
- **Images**: 103,341 (jpg, png)
- **Geometry**: 39,412 (geo, dwg, obj)
- **Data**: 14,235 (ebase)
- **Materials**: 8,776 (mat)
- **Tables**: 9,989 (csv)
- **Archives**: 3,972 (alb, zip)
- **Config**: 4,123 (cfg)
- **Other**: 3,000+ (pdf, html, egms, etc.)

**Total estimated files**: 180,000+

## Conclusion

The OFML data directory is a complex, hierarchical structure with:
- **Multiple file formats** requiring specialized readers
- **Manufacturer variations** requiring flexible parsing
- **Language-specific data** requiring fallback logic
- **Version management** requiring directory scanning
- **Large archives** requiring efficient extraction

A robust library must:
1. Handle all directory patterns dynamically
2. Support standard OCD + custom tables
3. Provide efficient archive access
4. Gracefully handle missing/malformed data
5. Support incremental loading (don't load everything)
6. Cache parsed structures for performance

---

**Generated**: December 31, 2024
**Data Source**: `/reference/ofmldata/` (pCon.planner distribution)
**Analysis Depth**: 117+ manufacturers, 2000+ series, 180,000+ files
