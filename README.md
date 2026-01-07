# OFML Interpreter

A Rust workspace for interpreting OFML (Office Furniture Modeling Language) data, providing parsing, execution, and conversion of furniture product data to modern 3D formats.

## Workspace Structure

```
crates/
├── ofml-lib/     # Core library - OFML parsing and business logic
├── ofml-tui/     # Terminal UI - interactive product configurator
└── ofml-cli/     # CLI - command-line interface
```

## Features

- **CLS Script Execution** - Parse and execute OFML class definition scripts
- **3D Geometry Conversion** - Convert 3DS, GEO, OBJ files to glTF/GLB format
- **Product Assembly** - Assemble complete furniture products from EBASE databases
- **2D Floor Plans** - Generate SVG floor plans from odb2d data
- **Material System** - MAT file parsing with texture support (PNG/JPG/TGA/BMP)
- **ALB Archive Support** - Extract files from encrypted OFML archives
- **TUI Product Configurator** - Interactive terminal interface for configuring products
- **Parallel Processing** - Multi-threaded data loading with rayon
- **Price Calculation** - Full OCD pricing with base prices and surcharges

## Quick Start

```bash
# Build entire workspace
cargo build --workspace --release

# Run the TUI Configurator
cargo run -p ofml-tui --release -- /path/to/ofmldata

# Run CLI commands
cargo run -p ofml-cli --release -- manufacturers /path/to/ofmldata
cargo run -p ofml-cli --release -- configure /path/to/ofmldata vitra ac

# Or use the built binaries
./target/release/ofml-tui /path/to/ofmldata
./target/release/ofml manufacturers /path/to/ofmldata
```

## TUI Configurator

The interactive Terminal User Interface allows you to:
- Browse manufacturers and product families
- Configure products with property selection
- Calculate prices with surcharges
- Export configurations to JSON
- Export geometry to GLB
- Save/load configurations (`Ctrl+S`/`Ctrl+O`)
- Mark favorites (`f` key)
- Toggle light/dark theme (`T` key)

### TUI Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑↓` | Navigate |
| `←→` | Change property value |
| `Enter` | Select |
| `/` | Search |
| `f` | Toggle favorite |
| `T` | Toggle theme |
| `Ctrl+S` | Save configuration |
| `Ctrl+O` | Load configuration |
| `Ctrl+G` | Export geometry (GLB) |
| `e` | Export to JSON |
| `t` | Table browser |
| `F12` | Debug panel |
| `?` | Help |
| `q` | Quit |

## CLI Commands

| Command | Description |
|---------|-------------|
| `manufacturers` | List available manufacturers |
| `articles` | List articles for a manufacturer |
| `configure` | Configure a product and display pricing |
| `catalog` | Browse XCF catalog structure |
| `convert` | Convert 3DS/GEO/OBJ to GLB |
| `merge` | Merge multiple geometry files |
| `product` | Assemble product from OFML data |
| `svg` | Generate 2D SVG floor plan |
| `ebase` | Explore EBASE database tables |
| `alb` | List/extract ALB archive contents |
| `cls` | List/extract CLS scripts from ALB |
| `completions` | Generate shell completions |

## Library Usage

```rust
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::FamilyConfiguration;
use std::path::Path;

fn main() {
    let data_path = Path::new("/path/to/ofmldata");
    let mut engine = ConfigurationEngine::new(data_path);

    // Load product families
    let families = engine.load_families("vitra");

    for family in families {
        println!("{} - {} variants", family.name, family.variant_count);

        // Get properties
        let properties = engine.get_family_properties("vitra", &family.id);

        // Create configuration
        let config = FamilyConfiguration::new(&family.id, &properties);

        // Calculate price
        if let Some(price) = engine.calculate_family_price(
            "vitra",
            &family,
            &config,
            chrono::Local::now().date_naive(),
        ) {
            println!("  Price: {:.2} {}", price.total_price, price.currency);
        }
    }
}
```

## Supported Formats

### Input
- 3DS (3D Studio)
- GEO (OFML geometry)
- OBJ (Wavefront)
- EBASE (OFML database)
- ALB (OFML archive)
- CLS (OFML scripts)
- MAT (materials)
- PNG/JPG/TGA/BMP (textures)

### Output
- GLB (binary glTF 2.0)
- SVG (2D floor plans)
- JSON (configuration export)

## Testing

```bash
# Run all tests (600+ tests)
cargo test --workspace

# Run library tests only
cargo test -p ofml-lib

# Run with output
cargo test -p ofml-lib -- --nocapture

# Run benchmarks
cargo bench -p ofml-lib
```

## Statistics

- **Tests**: 601+ passing
- **Manufacturers**: 108 supported
- **Crates**: 3 (lib, tui, cli)
- **Binary Size**: ~6MB (release, stripped)

## Documentation

- [Library Overview](docs/LIBRARY-OVERVIEW.md) - Architecture and crate structure
- [Data Formats](docs/DATA-FORMATS.md) - EBASE, OCD, ALB format documentation
- [Pricing Guide](docs/PRICING-GUIDE.md) - Price calculation details
- [OFML Explained](docs/OFML-EXPLAINED.md) - What is OFML, file formats
- [OCD Specification](docs/ofml-specs/ocd_4_3.md) - OCD 4.3 specification

## OFML Compliance

Implements the following OFML specifications:
- OFML 2.0 R3 (core language)
- GO 1.12.0 (geometry operations)
- ODB 2.4 (EBASE database)
- OMATS 2.2 (materials)
- OLAYERS 1.3.1 (2D layers)
- OCD 4.3 (catalog data)

## License

Part of the OfficeRocket4000 project.
