# OFML Export Formats

This document describes the export capabilities of the OFML Interpreter library, including GLB 3D models, SVG floor plans, and JSON configuration exports.

## Table of Contents

- [Overview](#overview)
- [JSON Configuration Export](#json-configuration-export)
- [GLB 3D Export](#glb-3d-export)
- [SVG 2D Floor Plan Export](#svg-2d-floor-plan-export)
- [Export Workflows](#export-workflows)
- [Integration Examples](#integration-examples)

## Overview

The OFML Interpreter supports exporting product configurations and geometry in multiple formats:

| Format | Purpose | Use Cases |
|--------|---------|-----------|
| **JSON** | Configuration data | ERP integration, order processing, web APIs |
| **GLB** | 3D geometry | Rendering, AR/VR, CAD import |
| **SVG** | 2D floor plans | Space planning, layout drawings |

All export functions are available in the `ofml-lib` crate and can be used programmatically.

## JSON Configuration Export

### Export Schema

JSON exports conform to a standardized schema for ERP and e-commerce integration.

**Schema Location**: `/workspace/specs/001-universal-ocd-reader/contracts/export-schema.json`

### Export Structure

```json
{
  "article_nr": "AI-121",
  "manufacturer": "sedus",
  "series": "ai",
  "variant_code": "S_STOFF=2G3;S_SITZHOEHE=1701",
  "description": "AI Chair Basic Configuration",
  "configuration": {
    "S_STOFF": "2G3",
    "S_SITZHOEHE": "1701",
    "S_LEHNE_ABW": "0000"
  },
  "property_details": [
    {
      "key": "S_STOFF",
      "label": "Stoff",
      "value": "2G3",
      "value_label": "Rubinrot",
      "group": "Material"
    }
  ],
  "pricing": {
    "base": 599.0,
    "surcharges": [
      {
        "var_cond": "S_166",
        "description": "Modellfarbe Rubinrot",
        "amount": 44.0,
        "is_percentage": false
      }
    ],
    "discounts": [],
    "net": 643.0,
    "taxes": [
      {
        "name": "MwSt (19%)",
        "rate": 19.0,
        "amount": 122.17
      }
    ],
    "total": 765.17,
    "currency": "EUR",
    "price_date": "2025-01-05",
    "valid_from": "2025-01-01",
    "valid_to": "2025-12-31"
  },
  "warnings": [],
  "exported_at": "2025-01-05T12:34:56Z"
}
```

### Field Descriptions

#### Root Fields

| Field | Type | Description |
|-------|------|-------------|
| `article_nr` | string | Base article number |
| `manufacturer` | string | Manufacturer identifier |
| `series` | string | Product series identifier |
| `variant_code` | string? | Generated variant code (null if no configuration) |
| `description` | string? | Article description |
| `configuration` | object | Property key-value pairs |
| `property_details` | array | Human-readable property information |
| `pricing` | object | Detailed pricing breakdown |
| `warnings` | array | Data warnings encountered |
| `exported_at` | string | ISO 8601 timestamp |

#### Configuration Object

Simple key-value mapping of property selections:

```json
{
  "S_STOFF": "2G3",
  "S_SITZHOEHE": "1701",
  "S_LEHNE_ABW": "0000"
}
```

#### Property Details Array

Enhanced property information for display:

```json
[
  {
    "key": "S_STOFF",              // Property identifier
    "label": "Stoff",               // Human-readable label
    "value": "2G3",                 // Selected value code
    "value_label": "Rubinrot",      // Human-readable value
    "group": "Material"             // Property group (optional)
  }
]
```

#### Pricing Object

| Field | Type | Description |
|-------|------|-------------|
| `base` | number | Base price amount |
| `surcharges` | array | Applied surcharges |
| `discounts` | array | Applied discounts |
| `net` | number | Net price (before taxes) |
| `taxes` | array | Tax breakdown |
| `total` | number | Gross total (including taxes) |
| `currency` | string | Currency code (ISO 4217) |
| `price_date` | string? | Date used for price lookup |
| `valid_from` | string? | Price validity start date |
| `valid_to` | string? | Price validity end date |

#### Surcharge Item

```json
{
  "var_cond": "S_166",
  "description": "Modellfarbe Rubinrot",
  "amount": 44.0,
  "is_percentage": false
}
```

#### Discount Item

```json
{
  "var_cond": "VOLUME_10",
  "description": "Volume discount (10+ units)",
  "amount": 50.0,
  "rule": "absolute"  // or "percentage"
}
```

#### Tax Item

```json
{
  "name": "MwSt (19%)",
  "rate": 19.0,
  "amount": 122.17
}
```

### Export Functions

#### Single Configuration Export

```rust
use ofml_lib::oap::{export_family_json, create_export_configuration};
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::FamilyConfiguration;
use chrono::Local;

fn export_single_config() {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families("sedus");
    let family = &families[0];

    let properties = engine.get_family_properties("sedus", &family.id);
    let config = FamilyConfiguration::new(&family.id, &properties);

    let price = engine.calculate_family_price(
        "sedus",
        family,
        &config,
        Local::now().date_naive(),
    );

    // Export to JSON string
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
```

#### Batch Export

```rust
use ofml_lib::oap::export_family_json_batch;

fn export_batch() {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families("vitra");

    let mut exports = Vec::new();

    for family in families.iter().take(10) {
        let properties = engine.get_family_properties("vitra", &family.id);
        let config = FamilyConfiguration::new(&family.id, &properties);
        let price = engine.calculate_family_price(
            "vitra",
            family,
            &config,
            Local::now().date_naive(),
        );

        let export = create_export_configuration(
            "vitra",
            "ai",
            &family.base_article_nr,
            &config,
            price.as_ref(),
            &[],
        );

        exports.push(export);
    }

    // Export as JSON array
    let json = export_family_json_batch(exports);
    println!("{}", json);
}
```

#### Export with Full Details

```rust
use ofml_lib::oap::create_export_configuration_with_details;

fn export_with_details() {
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");
    let families = engine.load_families("sedus");
    let family = &families[0];

    let properties = engine.get_family_properties("sedus", &family.id);
    let config = FamilyConfiguration::new(&family.id, &properties);
    let price = engine.calculate_family_price(
        "sedus",
        family,
        &config,
        Local::now().date_naive(),
    );

    // Include full property details and description
    let export = create_export_configuration_with_details(
        "sedus",
        "ai",
        &family.base_article_nr,
        &config,
        price.as_ref(),
        &[],
        Some(&family.description),
        &properties,
    );

    let json = serde_json::to_string_pretty(&export).unwrap();
    println!("{}", json);
}
```

### Price Formatting

#### German Format (1.234,56)

```rust
use ofml_lib::oap::{format_german_price, format_german_price_with_currency};
use rust_decimal::Decimal;

let amount = Decimal::new(123456, 2); // 1234.56

// Format: 1.234,56
println!("{}", format_german_price(amount));

// Format with currency: 1.234,56 EUR
println!("{}", format_german_price_with_currency(amount, "EUR"));
```

## GLB 3D Export

GLB (GL Transmission Format Binary) is a compact 3D format supported by web browsers, game engines, and CAD software.

### Export 3D Geometry

```rust
use ofml_lib::operations::export_to_glb;
use ofml_lib::geometry::SceneGraph;
use std::fs;
use std::path::Path;

fn export_glb_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create or load a scene
    let mut scene = SceneGraph::new();

    // Add geometry (example: simple desk)
    let top = scene.create_block("desk_top".to_string(), [1.2, 0.025, 0.8], None);
    top.borrow_mut().translate([0.0, 0.72, 0.0]);

    let leg1 = scene.create_block("leg1".to_string(), [0.05, 0.72, 0.05], None);
    leg1.borrow_mut().translate([0.1, 0.0, 0.1]);

    // Convert to GLB
    let glb_data = export_to_glb(&scene)?;

    // Write to file
    fs::write("desk.glb", glb_data)?;

    println!("Exported desk.glb ({} bytes)", glb_data.len());

    Ok(())
}
```

### Load Existing Geometry

```rust
use ofml_lib::operations::{load_geometry_file, export_to_glb};
use std::path::Path;

fn convert_to_glb() -> Result<(), Box<dyn std::error::Error>> {
    // Load from 3DS, GEO, or OBJ format
    let scene = load_geometry_file(Path::new("model.3ds"))?;

    // Export to GLB
    let glb_data = export_to_glb(&scene)?;

    std::fs::write("model.glb", glb_data)?;

    Ok(())
}
```

### GLB Features

**Supported**:
- Meshes with vertices and faces
- Materials (diffuse color, ambient, specular)
- Scene hierarchy
- Transformations (translation, rotation, scale)

**Not Supported** (currently):
- Textures (planned)
- Animations
- Skinning/rigging
- Cameras and lights

### GLB Structure

```
GLB File
├── JSON Header
│   ├── Scene graph
│   ├── Node hierarchy
│   ├── Mesh references
│   └── Material definitions
└── Binary Buffer
    ├── Vertex data
    ├── Face indices
    └── Normal data
```

### Validation

```rust
use ofml_lib::operations::validate_geometry;

fn validate_before_export(scene: &Scene3DS) {
    let validation = validate_geometry(scene);

    if validation.is_valid {
        println!("✓ Geometry is valid");
        println!("  Vertices: {}", validation.vertex_count);
        println!("  Faces: {}", validation.face_count);
        println!("  Meshes: {}", validation.mesh_count);
    } else {
        println!("✗ Geometry has errors:");
        for error in &validation.errors {
            println!("  - {}", error);
        }
    }

    if !validation.warnings.is_empty() {
        println!("⚠ Warnings:");
        for warning in &validation.warnings {
            println!("  - {}", warning);
        }
    }
}
```

## SVG 2D Floor Plan Export

SVG export creates top-down floor plan views of furniture configurations.

### 2D Primitives

The library supports these 2D primitives:

| Primitive | Description | Use Case |
|-----------|-------------|----------|
| `Line2D` | Straight line segment | Edges, construction lines |
| `Rect2D` | Rectangle | Desks, tables, cabinets |
| `Circle2D` | Circle | Round tables, columns |
| `Ellipse2D` | Ellipse | Oval tables |
| `Polygon2D` | Arbitrary polygon | Complex shapes |
| `Arc2D` | Circular arc | Rounded corners |
| `Bezier2D` | Bezier curve | Smooth curves |

### Create 2D Geometry

```rust
use ofml_lib::geometry2d::{G2DPrimitive, G2DCompound, Point2D, Rect2D};

fn create_floor_plan() -> G2DCompound {
    let mut compound = G2DCompound::new();

    // Add a desk (rectangle)
    let desk = G2DPrimitive::Rectangle(Rect2D::new(0.0, 0.0, 1.2, 0.8));
    compound.add_primitive(desk);

    // Add a chair (circle)
    let chair = G2DPrimitive::Circle(Circle2D::from_coords(0.6, -0.5, 0.3));
    compound.add_primitive(chair);

    compound
}
```

### Export to SVG

```rust
use ofml_lib::operations::export_2d_floorplan;

fn export_svg_example() -> Result<(), Box<dyn std::error::Error>> {
    let floor_plan = create_floor_plan();

    // Export to SVG string
    let svg = export_2d_floorplan(&floor_plan)?;

    // Write to file
    std::fs::write("floor_plan.svg", svg)?;

    println!("Exported floor_plan.svg");

    Ok(())
}
```

### SVG Options

```rust
use ofml_lib::geometry2d::{G2DAttributes, Transform2D};

fn create_styled_floor_plan() -> G2DCompound {
    let mut compound = G2DCompound::new();

    // Create desk with styling
    let mut desk_attrs = G2DAttributes::default();
    desk_attrs.stroke_color = Some("#000000".to_string());
    desk_attrs.stroke_width = Some(2.0);
    desk_attrs.fill_color = Some("#D2B48C".to_string()); // Tan color

    let desk = G2DPrimitive::Rectangle(Rect2D::new(0.0, 0.0, 1.2, 0.8));
    compound.add_primitive_with_attributes(desk, desk_attrs);

    // Create chair with different styling
    let mut chair_attrs = G2DAttributes::default();
    chair_attrs.stroke_color = Some("#000000".to_string());
    chair_attrs.fill_color = Some("#8B4513".to_string()); // Saddle brown

    let chair = G2DPrimitive::Circle(Circle2D::from_coords(0.6, -0.5, 0.3));
    compound.add_primitive_with_attributes(chair, chair_attrs);

    compound
}
```

### 2D Transforms

```rust
use ofml_lib::geometry2d::Transform2D;

fn apply_transforms() {
    let mut compound = create_floor_plan();

    // Create transform: translate, rotate, scale
    let transform = Transform2D {
        translation: [2.0, 1.0],
        rotation: std::f64::consts::PI / 4.0, // 45 degrees
        scale: [1.0, 1.0],
    };

    // Apply to all primitives
    compound.apply_transform(&transform);
}
```

## Export Workflows

### Workflow 1: Configure and Export

Complete workflow from configuration to JSON export:

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::FamilyConfiguration;
use ofml_lib::oap::export_family_json;
use chrono::Local;
use std::fs;

fn configure_and_export() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize engine
    let mut engine = ConfigurationEngine::new("/reference/ofmldata");

    // 2. Load products
    let families = engine.load_families("vitra");
    let family = &families[0];

    // 3. Get properties
    let properties = engine.get_family_properties("vitra", &family.id);

    // 4. Create configuration
    let mut config = FamilyConfiguration::new(&family.id, &properties);

    // 5. Set property values
    for prop in &properties {
        if let Some(option) = prop.options.first() {
            config.set_property(&prop.key, &option.value);
        }
    }

    // 6. Calculate price
    let price = engine.calculate_family_price(
        "vitra",
        family,
        &config,
        Local::now().date_naive(),
    );

    // 7. Export to JSON
    let json = export_family_json(
        "vitra",
        "ai",
        &family.base_article_nr,
        &config,
        price.as_ref(),
        &[],
    );

    // 8. Save to file
    fs::write("config.json", json)?;

    println!("Configuration exported to config.json");

    Ok(())
}
```

### Workflow 2: Batch Export All Manufacturers

```rust
use ofml_lib::oap::manufacturers::list_manufacturers;
use ofml_lib::oap::engine::ConfigurationEngine;
use std::path::Path;

fn export_all_manufacturers() {
    let data_root = Path::new("/reference/ofmldata");
    let manufacturers = list_manufacturers(data_root);
    let mut engine = ConfigurationEngine::new(data_root);

    for mfr in manufacturers {
        println!("Exporting {}...", mfr.name);

        let families = engine.load_families(&mfr.id);
        let mut exports = Vec::new();

        for family in families {
            let properties = engine.get_family_properties(&mfr.id, &family.id);
            let config = FamilyConfiguration::new(&family.id, &properties);
            let price = engine.calculate_family_price(
                &mfr.id,
                &family,
                &config,
                chrono::Local::now().date_naive(),
            );

            let export = create_export_configuration(
                &mfr.id,
                &family.series,
                &family.base_article_nr,
                &config,
                price.as_ref(),
                &[],
            );

            exports.push(export);
        }

        let json = export_family_json_batch(exports);
        let filename = format!("export_{}.json", mfr.id);
        std::fs::write(&filename, json).ok();

        println!("  Saved {}", filename);
    }
}
```

### Workflow 3: 3D Model Assembly

```rust
use ofml_lib::operations::{assemble_product, ProductConfig, export_to_glb};
use std::path::Path;

fn assemble_and_export_3d() -> Result<(), Box<dyn std::error::Error>> {
    let product_path = Path::new("/reference/ofmldata/vitra/ai/DE/1");

    // Assemble product geometry
    let config = ProductConfig::default();
    let result = assemble_product(product_path, &config)?;

    println!("Assembled {} articles", result.articles_found.len());
    println!("Loaded {} geometry files", result.geometry_loaded);

    // Export to GLB
    let glb_data = export_to_glb(&result.scene)?;
    std::fs::write("product.glb", glb_data)?;

    println!("Exported product.glb");

    Ok(())
}
```

## Integration Examples

### Web API Endpoint

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::export_family_json;
use std::sync::Arc;

// Example with axum web framework
async fn configure_product_handler(
    engine: Arc<ConfigurationEngine>,
    manufacturer: String,
    series: String,
    article: String,
    selections: HashMap<String, String>,
) -> String {
    let families = engine.load_families(&manufacturer);

    let family = families.iter()
        .find(|f| f.base_article_nr == article)
        .unwrap();

    let properties = engine.get_family_properties(&manufacturer, &family.id);
    let mut config = FamilyConfiguration::new(&family.id, &properties);

    for (key, value) in selections {
        config.set_property(&key, &value);
    }

    let price = engine.calculate_family_price(
        &manufacturer,
        family,
        &config,
        chrono::Local::now().date_naive(),
    );

    export_family_json(&manufacturer, &series, &article, &config, price.as_ref(), &[])
}
```

### Database Storage

```rust
use rusqlite::{Connection, params};
use ofml_lib::oap::ExportConfiguration;

fn store_configuration(conn: &Connection, config: &ExportConfiguration) -> Result<i64, rusqlite::Error> {
    let json = serde_json::to_string(config).unwrap();

    conn.execute(
        "INSERT INTO configurations (manufacturer, article_nr, variant_code, config_json, created_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        params![
            &config.manufacturer,
            &config.article_nr,
            &config.variant_code,
            &json,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}
```

### File System Export

```rust
use std::fs;
use std::path::PathBuf;

fn export_to_filesystem(
    manufacturer: &str,
    series: &str,
    article: &str,
    json: &str,
) -> Result<PathBuf, std::io::Error> {
    let dir = PathBuf::from("exports")
        .join(manufacturer)
        .join(series);

    fs::create_dir_all(&dir)?;

    let filename = dir.join(format!("{}.json", article));
    fs::write(&filename, json)?;

    Ok(filename)
}
```

## Related Documentation

- [LIBRARY-OVERVIEW.md](LIBRARY-OVERVIEW.md) - Library architecture
- [DATA-FORMATS.md](DATA-FORMATS.md) - OFML data formats
- [PRICING-GUIDE.md](PRICING-GUIDE.md) - Pricing system
- [OCD-PRICING-IMPLEMENTATION.md](OCD-PRICING-IMPLEMENTATION.md) - Implementation details
