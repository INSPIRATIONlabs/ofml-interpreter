# OFML Data Formats

This document describes the data formats used in the OFML ecosystem, including binary database formats, archive structures, and data organization.

## Table of Contents

- [Overview](#overview)
- [Directory Structure](#directory-structure)
- [EBase File Format](#ebase-file-format)
- [ALB Archives](#alb-archives)
- [OAM Mappings](#oam-mappings)
- [OCD Tables](#ocd-tables)
- [Working with Data](#working-with-data)

## Overview

OFML (Office Furniture Modeling Language) data is organized in a standardized directory structure containing:

1. **EBase databases** (`.ebase` files) - Binary database files with product data
2. **ALB archives** (`.alb` files) - ZIP-based archives containing CLS class definitions
3. **OAM mappings** (`oam.xml` files) - XML files linking article numbers to CLS classes

## Directory Structure

### Manufacturer Data Organization

OFML data is organized by manufacturer, series, language, and version:

```
/reference/ofmldata/
├── {manufacturer_id}/          # e.g., "vitra", "sedus", "fast"
│   ├── {series_id}/            # e.g., "ai", "workit", "rahmen"
│   │   ├── {lang}/{version}/   # e.g., "DE/1", "EN/1", "FR/1"
│   │   │   ├── db/
│   │   │   │   └── pdata.ebase    # OCD database (required)
│   │   │   ├── {Manufacturer}.alb # ALB archive (optional)
│   │   │   └── ... (geometry, textures)
│   │   └── OAM/
│   │       └── oam.xml            # Article-to-class mappings
│   └── ... (other series)
└── ... (other manufacturers)
```

### Example: Vitra Data

```
/reference/ofmldata/vitra/
├── ai/                    # Vitra AI chair series
│   ├── DE/1/
│   │   ├── db/
│   │   │   └── pdata.ebase
│   │   ├── Vitra.alb
│   │   └── geometry/
│   └── OAM/
│       └── oam.xml
└── workit/                # Vitra Workit desk series
    ├── DE/1/
    │   ├── db/
    │   │   └── pdata.ebase
    │   └── Vitra.alb
    └── OAM/
        └── oam.xml
```

### Path Components

| Component | Description | Examples |
|-----------|-------------|----------|
| `manufacturer_id` | Short manufacturer identifier | `vitra`, `sedus`, `fast`, `fram` |
| `series_id` | Product line/series identifier | `ai`, `workit`, `rahmen`, `one` |
| `lang` | ISO 639-1 language code | `DE` (German), `EN` (English), `FR` (French) |
| `version` | Data version number | `1`, `2`, `3` |

### Standard File Locations

| File | Path Pattern | Purpose |
|------|--------------|---------|
| OCD Database | `{mfr}/{series}/{lang}/{ver}/db/pdata.ebase` | Product catalog and pricing |
| ALB Archive | `{mfr}/{series}/{lang}/{ver}/{Manufacturer}.alb` | CLS class definitions |
| OAM Mappings | `{mfr}/{series}/OAM/oam.xml` | Article-to-class links |
| Geometry | `{mfr}/{series}/{lang}/{ver}/geometry/*.3ds` | 3D models |
| Textures | `{mfr}/{series}/{lang}/{ver}/texture/*.jpg` | Material textures |

## EBase File Format

EBase is a proprietary binary database format developed by EasternGraphics for OFML data storage.

### File Structure

```
+------------------+
| Header (52 bytes)|
+------------------+
| Table Directory  |
+------------------+
| Table Definitions|
+------------------+
| Record Data      |
+------------------+
| String Pool      |
+------------------+
| Blob Data        |
+------------------+
```

### Header Format

| Offset | Size | Type | Description |
|--------|------|------|-------------|
| 0x00 | 6 | bytes | Magic: `EBDBF\0` |
| 0x06 | 2 | u16 BE | Header version |
| 0x08 | 2 | u16 BE | Major version (always 1) |
| 0x0A | 2 | u16 BE | Minor version |
| 0x0C | 4 | u32 BE | Reserved |
| 0x10 | 4 | u32 BE | String pool offset |
| 0x14 | 4 | u32 BE | Reserved |
| 0x18 | 4 | u32 BE | Reserved |
| 0x1C | 4 | u32 BE | Reserved |
| 0x20 | 4 | u32 BE | Reserved |
| 0x24 | 4 | u32 BE | String data size |
| 0x28 | 4 | u32 BE | Number of tables |

### Column Types

EBase supports the following column data types:

| Type ID | Name | Size | Rust Type | Description |
|---------|------|------|-----------|-------------|
| 1 | Int8Signed | 1 | `i8` | Signed 8-bit integer |
| 2 | Int8Unsigned | 1 | `u8` | Unsigned 8-bit integer |
| 3 | Int16Signed | 2 | `i16` | Signed 16-bit integer (BE) |
| 4 | Int16Unsigned | 2 | `u16` | Unsigned 16-bit integer (BE) |
| 5 | Int32Signed | 4 | `i32` | Signed 32-bit integer (BE) |
| 6 | Int32Unsigned | 4 | `u32` | Unsigned 32-bit integer (BE) |
| 7 | Float32 | 4 | `f32` | 32-bit float (BE) |
| 8 | Float64 | 8 | `f64` | 64-bit double (BE) |
| 9 | StringInline | var | `String` | Inline string (length-prefixed) |
| 10 | StringOffset | 4 | `String` | String pool offset (u32 BE) |
| 11 | Blob | 4 | `Vec<u8>` | Binary data offset |

### Reading EBase Files

```rust
use ofml_lib::ebase::EBaseReader;
use std::path::Path;

fn read_ebase_example() -> Result<(), Box<dyn std::error::Error>> {
    // Open database
    let db_path = Path::new("/reference/ofmldata/vitra/ai/DE/1/db/pdata.ebase");
    let reader = EBaseReader::open(db_path)?;

    println!("EBase version: {}.{}", reader.major_version, reader.minor_version);
    println!("Tables: {}", reader.tables.len());

    // List all tables
    for table_name in reader.tables.keys() {
        println!("  - {}", table_name);
    }

    // Read a specific table
    if let Some(table) = reader.get_table("ocd_article") {
        println!("\nTable: ocd_article");
        println!("Columns: {:?}", table.column_names());
        println!("Records: {}", table.record_count);

        // Iterate records
        for record in table.records().take(5) {
            let article_nr = record.get_str("article_nr").unwrap_or("?");
            let series = record.get_str("series").unwrap_or("?");
            println!("  Article: {} (Series: {})", article_nr, series);
        }
    }

    Ok(())
}
```

### String Pool

Strings are stored in a central pool to reduce duplication:

1. Column values contain a 4-byte offset (big-endian) into the string pool
2. At the offset, a 2-byte length prefix (big-endian) indicates string length
3. The actual string data follows (UTF-8 or Latin-1 encoded)
4. Empty strings use offset 0

### Data Corruption Recovery

The library includes automatic recovery for corrupted records:

- **Detection**: Identifies 8-byte offset shift patterns
- **Recovery**: Attempts to parse with shifted offsets
- **Reporting**: Generates `DataWarning` for tracking

## ALB Archives

ALB (Archive Library) files are ZIP archives containing CLS (Class) source files.

### Archive Structure

```
{Manufacturer}.alb (ZIP file)
├── cls/
│   ├── {ClassName}.cls       # Individual class files
│   ├── {ClassName2}.cls
│   └── ...
├── manifest.xml              # Package metadata (optional)
└── dependencies.xml          # Dependency information (optional)
```

### Example: Vitra.alb

```
Vitra.alb
├── cls/
│   ├── ViTable_Round.cls
│   ├── ViChair_AI.cls
│   ├── ViDesk_Workit.cls
│   └── ... (hundreds of classes)
```

### CLS File Format

CLS files contain OFML class definitions in a C-like syntax:

```cls
// Package declaration
package ::vitra::ai;

// Imports
import ::vitra::basics::ViChair;

// Class definition
class ViChair_AI : ViChair {
    func initialize() {
        // Property setup
        setupProperty(@color, ["Color", NULL, NULL, 0, "ch @red @blue @green"], 1);
        setPropValue(@color, @red);

        // Geometry creation
        OiBlock(self, @seat, [0.5, 0.05, 0.5]);
        setMaterial("::vitra::material::fabric::red");
    }

    func getPrice() {
        return baseprice + getPropExtra(@color);
    }
}
```

### Loading ALB Archives

```rust
use ofml_lib::alb_loader::AlbLoader;
use std::path::Path;

fn load_alb_example() -> Result<(), Box<dyn std::error::Error>> {
    let alb_path = Path::new("/reference/ofmldata/vitra/ai/DE/1/Vitra.alb");
    let mut loader = AlbLoader::new();

    // Load ALB (automatically handles dependencies)
    loader.load_alb(alb_path)?;

    // Get available classes
    let classes = loader.list_classes();
    println!("Loaded {} classes", classes.len());

    for class_name in classes.iter().take(10) {
        println!("  - {}", class_name);
    }

    Ok(())
}
```

### Password Protection

ALB files are ZIP archives encrypted with a standard password:

```rust
const ALB_PASSWORD: &[u8] = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";
```

The library automatically handles decryption when reading ALB files.

## OAM Mappings

OAM (OFML Article Mapping) files link article numbers to CLS class names.

### XML Format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<oam version="1.0">
    <articles>
        <article number="AI-121" class="ViChair_AI_Basic"/>
        <article number="AI-121-2G3" class="ViChair_AI_Fabric"/>
        <article number="AI-121-LEA" class="ViChair_AI_Leather"/>
    </articles>
    <dependencies>
        <package name="vitra.basics" required="true"/>
        <package name="vitra.materials" required="true"/>
    </dependencies>
</oam>
```

### Reading OAM Files

```rust
use ofml_lib::oap::oam::load_manufacturer_oam;
use std::path::Path;

fn read_oam_example() -> Result<(), Box<dyn std::error::Error>> {
    let oam_path = Path::new("/reference/ofmldata/vitra/ai/OAM/oam.xml");
    let oam_data = load_manufacturer_oam(oam_path)?;

    println!("OAM version: {}", oam_data.version);
    println!("Article mappings: {}", oam_data.articles.len());

    for (article_nr, class_name) in &oam_data.articles {
        println!("  {} -> {}", article_nr, class_name);
    }

    Ok(())
}
```

## OCD Tables

OCD (OFML Commercial Data) tables in EBase files follow a standardized schema.

### Core Tables

| Table Name | Description | Key Fields |
|------------|-------------|------------|
| `ocd_article` | Product catalog | `article_nr`, `series`, `description` |
| `ocd_price` | Pricing data | `article_nr`, `var_cond`, `price`, `price_level` |
| `ocd_pricetext` | Price descriptions | `price_textnr`, `language`, `text` |
| `ocd_propertyclass` | Article property classes | `article_nr`, `propclass_name` |
| `ocd_propertyvalue` | Property value options | `propclass_name`, `prop_name`, `value` |
| `ocd_relation` | Business rules | `relation_id`, `rule_type`, `condition` |
| `ocd_artshorttext` | Short article descriptions | `article_nr`, `language`, `text` |
| `ocd_artlongtext` | Long article descriptions | `article_nr`, `language`, `text` |

### ocd_article Schema

Primary product catalog table:

| Column | Type | Description |
|--------|------|-------------|
| `article_nr` | String | Article number (primary key) |
| `series` | String | Product series identifier |
| `manufacturer` | String | Manufacturer code |
| `status` | Int | Active status (1=active, 0=inactive) |
| `created_date` | Date | Creation timestamp |
| `modified_date` | Date | Last modification timestamp |

### ocd_price Schema

Price and surcharge data:

| Column | Type | Description |
|--------|------|-------------|
| `article_nr` | String | Article number (or "*" for wildcard) |
| `var_cond` | String | Variant condition code |
| `price_type` | String | 'S' (sales) or 'P' (purchase) |
| `price_level` | String | 'B' (base), 'X' (surcharge), 'D' (discount) |
| `price` | Decimal | Price amount |
| `is_fix` | Int | 1=fixed amount, 0=percentage |
| `currency` | String | Currency code (EUR, CHF, USD, etc.) |
| `date_from` | Date | Validity start date |
| `date_to` | Date | Validity end date (nullable) |
| `price_textnr` | Int | Reference to ocd_pricetext |

### ocd_propertyclass Schema

Maps articles to property classes:

| Column | Type | Description |
|--------|------|-------------|
| `article_nr` | String | Article number |
| `propclass_name` | String | Property class name |
| `position` | Int | Display order |

### ocd_propertyvalue Schema

Property value options:

| Column | Type | Description |
|--------|------|-------------|
| `propclass_name` | String | Property class name |
| `prop_name` | String | Property identifier |
| `value` | String | Value code |
| `text` | String | Display label |
| `is_default` | Int | 1=default, 0=not default |
| `position` | Int | Display order |

### Reading OCD Tables

```rust
use ofml_lib::oap::ocd::OcdReader;
use std::path::Path;

fn read_ocd_example() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = Path::new("/reference/ofmldata/sedus/ai/DE/1/db/pdata.ebase");
    let reader = OcdReader::new(db_path)?;

    // Read articles
    let articles = reader.read_articles()?;
    println!("Found {} articles", articles.len());

    // Read prices
    let prices = reader.read_prices("AI-121")?;
    println!("Found {} price entries for AI-121", prices.len());

    for price in &prices {
        println!("  {} {}: {} {} (var_cond: {})",
            price.price_level,
            price.price_type,
            price.price,
            price.currency,
            price.var_cond
        );
    }

    // Read property classes
    let prop_classes = reader.read_property_classes("AI-121")?;
    println!("Property classes: {:?}", prop_classes);

    Ok(())
}
```

## Working with Data

### Complete Example: Loading Manufacturer Data

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::manufacturers::list_manufacturers;
use std::path::Path;

fn load_manufacturer_data() -> Result<(), Box<dyn std::error::Error>> {
    let data_root = Path::new("/reference/ofmldata");

    // List all manufacturers
    let manufacturers = list_manufacturers(data_root);
    println!("Found {} manufacturers:", manufacturers.len());

    for mfr in &manufacturers {
        println!("  {} ({})", mfr.name, mfr.id);
    }

    // Initialize configuration engine
    let mut engine = ConfigurationEngine::new(data_root);

    // Load families for a manufacturer
    let families = engine.load_families("vitra");
    println!("\nFound {} product families for Vitra", families.len());

    for family in families.iter().take(5) {
        println!("  {}: {} ({} variants)",
            family.id,
            family.name,
            family.variant_count
        );
    }

    Ok(())
}
```

### Performance Considerations

**EBase Reading**:
- Tables are loaded on-demand
- String pool is cached automatically
- Records are parsed lazily

**ALB Loading**:
- Classes are parsed only when accessed
- Dependency resolution is cached
- Multiple ALBs can be loaded in dependency order

**Caching**:
- Use `ConfigurationEngine` for automatic caching
- Clear caches when data changes: `engine.clear_cache()`

### Error Handling

```rust
use ofml_lib::ebase::{EBaseReader, EBaseError};

fn safe_ebase_reading() -> Result<(), EBaseError> {
    let reader = EBaseReader::open("pdata.ebase")?;

    // Safe table access
    match reader.get_table("ocd_article") {
        Some(table) => {
            println!("Found {} records", table.record_count);
        }
        None => {
            println!("Table not found");
        }
    }

    Ok(())
}
```

## Related Documentation

- [LIBRARY-OVERVIEW.md](LIBRARY-OVERVIEW.md) - Library architecture and quick start
- [PRICING-GUIDE.md](PRICING-GUIDE.md) - Pricing calculation system
- [OCD-PRICING-IMPLEMENTATION.md](OCD-PRICING-IMPLEMENTATION.md) - Implementation details
- [OFML-TABLE-SCHEMAS.md](OFML-TABLE-SCHEMAS.md) - Complete table schemas
- [docs/ofml-specs/ocd_4_3.md](/workspace/docs/ofml-specs/ocd_4_3.md) - OCD 4.3 specification
