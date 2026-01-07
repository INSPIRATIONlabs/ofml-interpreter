# OFML Interpreter Library Overview

This document provides an overview of the OFML Interpreter library architecture, explaining how the three crates work together to provide OFML data reading, product configuration, and user interfaces.

## Table of Contents

- [Crate Structure](#crate-structure)
- [Key Concepts](#key-concepts)
- [Quick Start](#quick-start)
- [Module Organization](#module-organization)
- [Common Use Cases](#common-use-cases)
- [Integration Guide](#integration-guide)

## Crate Structure

The OFML Interpreter is organized into three crates:

### 1. ofml-lib (Core Library)

**Location**: `/workspace/crates/ofml-lib`

The core library containing all OFML data reading and business logic. This is a standalone library that can be used by any Rust application.

**Key Features**:
- CLS bytecode interpreter for OFML class files
- EBase binary database reader
- OAP (OFML Article Properties) configurator
- 3D geometry processing (3DS, GEO, OBJ formats)
- 2D floor plan generation
- Material and texture handling
- Price calculation engine
- Configuration export to JSON

**Dependencies**:
```toml
[dependencies]
ofml-lib = { path = "../crates/ofml-lib" }
```

### 2. ofml-tui (Terminal UI)

**Location**: `/workspace/crates/ofml-tui`

A full-featured terminal user interface for browsing manufacturers, configuring products, and viewing prices.

**Key Features**:
- Interactive product catalog browser
- Property configuration with real-time validation
- Price calculation display with surcharge breakdown
- Search and filtering
- Configuration export

**Binary**: `ofml-tui`

**Usage**:
```bash
./target/release/ofml-tui /path/to/ofmldata
```

### 3. ofml-cli (Command Line Interface)

**Location**: `/workspace/crates/ofml-cli`

A command-line interface for scripting and automation tasks.

**Key Features**:
- List manufacturers and products
- Export configurations to JSON
- Batch processing support
- Shell completion generation

**Binary**: `ofml`

**Usage**:
```bash
./target/release/ofml list-manufacturers /path/to/ofmldata
./target/release/ofml export-config vitra ai AI-121 > config.json
```

## Key Concepts

### OFML Data Directory Structure

OFML data is organized by manufacturer and series:

```
/reference/ofmldata/
├── vitra/              # Manufacturer ID
│   ├── ai/             # Series ID
│   │   ├── DE/1/       # Language (DE=German) and version
│   │   │   ├── db/
│   │   │   │   └── pdata.ebase    # OCD database
│   │   │   └── Vitra.alb          # ALB archive (ZIP with CLS classes)
│   │   └── OAM/
│   │       └── oam.xml            # Article-to-Class mappings
├── sedus/
│   └── ai/
│       └── DE/1/
│           └── db/pdata.ebase
└── ...
```

### EBase Files

EBase (`.ebase`) is a proprietary binary database format containing:
- Articles (product catalog)
- Prices (base prices and surcharges)
- Properties (configuration options)
- Texts (multilingual descriptions)
- Relations (business rules)

See [DATA-FORMATS.md](DATA-FORMATS.md) for detailed format specification.

### OAM Mappings

OAM (OFML Article Mapping) XML files map article numbers to CLS class names, enabling property-based configuration.

### Product Families

Instead of showing individual SKUs (e.g., AI-121-2G3-1701), the library groups them into configurable families:
- Base article: AI-121
- Properties: Fabric color, Seat height, Backrest type
- Variants: All combinations of property values

### Pricing Model

Prices consist of:
- **Base price** (level 'B'): Fundamental product price
- **Surcharges** (level 'X'): Additional charges for options (e.g., +44 EUR for color)
- **Discounts** (level 'D'): Reductions based on conditions

See [PRICING-GUIDE.md](PRICING-GUIDE.md) for detailed pricing documentation.

## Quick Start

### Example 1: List Manufacturers

```rust
use ofml_lib::oap::manufacturers::list_manufacturers;
use std::path::Path;

fn main() {
    let data_path = Path::new("/reference/ofmldata");
    let manufacturers = list_manufacturers(data_path);

    for mfr in manufacturers {
        println!("{}: {}", mfr.id, mfr.name);
    }
}
```

### Example 2: Load and Configure a Product

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::FamilyConfiguration;
use chrono::Local;

fn main() {
    // Initialize the engine
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");

    // Load product families for Vitra
    let families = engine.load_families("vitra");

    if let Some(family) = families.first() {
        println!("Configuring: {}", family.name);

        // Get available properties
        let properties = engine.get_family_properties("vitra", &family.id);

        // Create a configuration with default values
        let mut config = FamilyConfiguration::new(&family.id, &properties);

        // Change a property value
        if let Some(prop) = properties.first() {
            if let Some(option) = prop.options.first() {
                config.set_property(&prop.key, &option.value);
            }
        }

        // Calculate price
        let price = engine.calculate_family_price(
            "vitra",
            family,
            &config,
            Local::now().date_naive(),
        );

        if let Some(price_result) = price {
            println!("Base price: {} {}", price_result.base_price, price_result.currency);
            for surcharge in &price_result.surcharges {
                println!("  + {}: {} {}",
                    surcharge.name,
                    surcharge.amount,
                    price_result.currency
                );
            }
            println!("Total: {} {}", price_result.total_price, price_result.currency);
        }
    }
}
```

### Example 3: Export Configuration to JSON

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::{export_family_json, format_german_price};

fn main() {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families("sedus");

    if let Some(family) = families.first() {
        let properties = engine.get_family_properties("sedus", &family.id);
        let config = FamilyConfiguration::new(&family.id, &properties);
        let price = engine.calculate_family_price(
            "sedus",
            family,
            &config,
            chrono::Local::now().date_naive(),
        );

        // Export to JSON
        let json = export_family_json(
            "sedus",
            "ai",
            &family.base_article_nr,
            &config,
            price.as_ref(),
            &[], // warnings
        );

        println!("{}", json);
    }
}
```

### Example 4: Read EBase Data Directly

```rust
use ofml_lib::ebase::EBaseReader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = Path::new("/reference/ofmldata/vitra/ai/DE/1/db/pdata.ebase");
    let reader = EBaseReader::open(db_path)?;

    // Read articles
    if let Some(articles_table) = reader.get_table("ocd_article") {
        for record in articles_table.records() {
            let article_nr = record.get_str("article_nr").unwrap_or("?");
            let series = record.get_str("series").unwrap_or("?");
            println!("Article: {} (Series: {})", article_nr, series);
        }
    }

    Ok(())
}
```

## Module Organization

### Core OFML Modules (ofml-lib)

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `lexer` | Tokenizes OFML source code | `Token`, `tokenize()` |
| `ast` | Abstract Syntax Tree types | `Stmt`, `Expr`, `ClassDef` |
| `parser` | Recursive descent parser | `Parser::parse()` |
| `interpreter` | Runtime execution engine | `Interpreter::execute()` |
| `geometry` | 3D geometry and transform handling | `Scene3DS`, `Mesh`, `Vertex` |
| `geometry2d` | 2D representation for floor plans | `G2DPrimitive`, `G2DCompound` |
| `ebase` | EBase file format reader | `EBaseReader`, `Table`, `Record` |
| `ebase_expr` | EBASE PostScript-like expressions | `EbaseEvaluator` |
| `material` | Material system (MAT files, textures) | `MaterialDef`, `TextureDef` |
| `ofml_classes` | OFML framework classes (Go*, Oi*) | `OfmlClassInstance` |
| `gobject` | GObject type system | `GValue` |
| `property` | Property system for products | `PropertyDef`, `PropertyValue` |
| `article` | Article configuration and variants | `ArticleConfig`, `Variant` |
| `attachment` | Attachment points system | `AttachmentPoint` |
| `operations` | High-level reusable functions | `export_to_glb()`, `export_2d_floorplan()` |

### OAP Configurator Modules (ofml-lib/oap)

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `engine` | Configuration engine with caching | `ConfigurationEngine` |
| `families` | Product family grouping | `ProductFamily`, `FamilyConfiguration` |
| `ocd` | OCD data reader (articles, prices, texts) | `OcdReader`, `OcdArticle`, `OcdPrice` |
| `ocd_properties` | OCD property definitions and values | `OcdPropertyReader`, `OcdPropertyValue` |
| `ocd_relation` | Business rule evaluation | `RelationRuleReader` |
| `price` | Pricing calculation utilities | `PriceBreakdown`, `PriceError` |
| `catalog` | Manufacturer and article discovery | `list_manufacturers()` |
| `manufacturers` | Manufacturer enumeration | `Manufacturer` |
| `oam` | OAM XML parsing | `OamData`, `ArticleMapping` |
| `variant` | Variant code generation | `generate_variant_code()` |

### TUI Modules (ofml-tui)

| Module | Purpose |
|--------|---------|
| `app` | Application state and event handling |
| `ui` | Main UI rendering |
| `views` | Individual view components (catalog, config, help) |
| `widgets` | Reusable UI widgets (forms, tables) |
| `config_store` | Configuration persistence |
| `theme` | Color schemes and styling |

### CLI Modules (ofml-cli)

| Module | Purpose |
|--------|---------|
| `main` | Command dispatch and argument parsing |

## Common Use Cases

### Building a Product Configurator

1. **Initialize the engine** with your OFML data directory
2. **List manufacturers** to populate a catalog
3. **Load families** for the selected manufacturer
4. **Get properties** for the selected family
5. **Create a configuration** with default values
6. **Update properties** as the user makes selections
7. **Calculate price** whenever properties change
8. **Export configuration** when the user is done

### Batch Price Extraction

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use chrono::Local;

fn extract_all_prices(manufacturer_id: &str) {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families(manufacturer_id);
    let price_date = Local::now().date_naive();

    for family in families {
        let properties = engine.get_family_properties(manufacturer_id, &family.id);
        let config = FamilyConfiguration::new(&family.id, &properties);

        if let Some(price) = engine.calculate_family_price(
            manufacturer_id,
            &family,
            &config,
            price_date,
        ) {
            println!("{},{},{}", family.base_article_nr, price.base_price, price.total_price);
        }
    }
}
```

### 3D Geometry Export

```rust
use ofml_lib::operations::export_to_glb;
use ofml_lib::geometry::SceneGraph;
use std::path::Path;

fn export_product_3d(output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create or load a scene
    let mut scene = SceneGraph::new();
    let block = scene.create_block("base".to_string(), [1.0, 0.5, 0.3], None);

    // Export to GLB
    export_to_glb(&scene, output_path)?;

    Ok(())
}
```

### Reading Custom Tables

```rust
use ofml_lib::ebase::EBaseReader;

fn read_custom_table(db_path: &Path, table_name: &str) {
    let reader = EBaseReader::open(db_path).unwrap();

    if let Some(table) = reader.get_table(table_name) {
        println!("Table: {}", table_name);
        println!("Columns: {:?}", table.column_names());

        for record in table.records() {
            // Access fields by name
            if let Some(value) = record.get_str("some_field") {
                println!("  {}", value);
            }
        }
    }
}
```

## Integration Guide

### Adding to Your Project

Add to `Cargo.toml`:

```toml
[dependencies]
ofml-lib = { path = "../ofml-interpreter/crates/ofml-lib" }
chrono = "0.4"
rust_decimal = "1.33"
```

### Error Handling

All major operations return `Result` types:

```rust
use ofml_lib::oap::engine::{ConfigurationEngine, EngineError};

fn safe_configuration() -> Result<(), EngineError> {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families("unknown_manufacturer");

    // Handle missing manufacturers gracefully
    if families.is_empty() {
        return Err(EngineError::MissingData(
            "No families found".to_string()
        ));
    }

    Ok(())
}
```

### Caching and Performance

The `ConfigurationEngine` includes built-in caching:

- **OAM data**: Cached per manufacturer
- **Article lists**: Cached per manufacturer
- **Family loaders**: Cached per manufacturer
- **OCD readers**: Cached per manufacturer/series

Clear caches when data changes:

```rust
let mut engine = ConfigurationEngine::new("/reference/ofmldata");
// ... use engine ...
engine.clear_cache(); // Clear all caches
```

### Thread Safety

`ofml-lib` is designed for single-threaded use. For multi-threaded applications:

1. Create separate `ConfigurationEngine` instances per thread
2. Or wrap the engine in `Arc<Mutex<ConfigurationEngine>>`

### WASM Compatibility

The core `ofml-lib` is designed to be WASM-compatible (future):

- No file I/O in core types (use readers)
- No platform-specific dependencies
- Serializable data structures

## Related Documentation

- [DATA-FORMATS.md](DATA-FORMATS.md) - OFML data format specifications
- [PRICING-GUIDE.md](PRICING-GUIDE.md) - Pricing system documentation
- [EXPORT-FORMATS.md](EXPORT-FORMATS.md) - Export format documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - Overall architecture documentation
- [OCD-PRICING-IMPLEMENTATION.md](OCD-PRICING-IMPLEMENTATION.md) - Pricing implementation details
- [OFML-EXPLAINED.md](OFML-EXPLAINED.md) - OFML language concepts
- [CLS-EXAMPLES.md](CLS-EXAMPLES.md) - CLS bytecode examples

## Support and Contributions

For questions or issues:

1. Check the [docs/](docs/) folder for detailed documentation
2. Review the [tests/](tests/) folder for usage examples
3. See [CLAUDE.md](CLAUDE.md) for development guidelines
