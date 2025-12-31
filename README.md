# OFML Interpreter

A Rust implementation of the Office Furniture Modeling Language (OFML) for parsing, executing, and converting furniture product data to modern 3D formats.

## Features

- **CLS Script Execution** - Parse and execute OFML class definition scripts
- **3D Geometry Conversion** - Convert 3DS, GEO, OBJ files to glTF/GLB format
- **Product Assembly** - Assemble complete furniture products from EBASE databases
- **2D Floor Plans** - Generate SVG floor plans from odb2d data
- **Material System** - MAT file parsing with texture support (PNG/JPG/TGA/BMP)
- **ALB Archive Support** - Extract files from encrypted OFML archives

## Quick Start

```bash
# Build
cargo build --release

# Convert a 3DS file to GLB
./target/release/ofml convert model.3ds

# Assemble a product from OFML data
./target/release/ofml product /path/to/ofmldata/vitra/ac/1 AC_CHAIR output.glb

# Generate 2D floor plan
./target/release/ofml svg /path/to/odb.ebase floorplan.svg

# Explore OFML data
./target/release/ofml ofml /path/to/ofmldata
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `convert` | Convert 3DS/GEO/OBJ to GLB |
| `merge` | Merge multiple geometry files |
| `product` | Assemble product from OFML data |
| `gsx` | Convert GSX/Sedus products (OBJ-based) |
| `build` | Execute CLS scripts and export GLB |
| `svg` | Generate 2D SVG floor plan |
| `ebase` | Explore EBASE database tables |
| `alb` | List/extract ALB archive contents |
| `cls` | List/extract CLS scripts from ALB |
| `ofml` | Browse OFML data directory |
| `export` | Execute CLS and export to GLB |
| `run` | Execute CLS script |
| `check` | Validate CLS syntax |
| `parse` | Display CLS AST |
| `tokenize` | Display CLS tokens |
| `validate` | Validate geometry and show metrics |
| `expr` | Evaluate EBASE expression |

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

## Library Usage

```rust
use ofml_interpreter::operations::{load_geometry_file, export_to_glb};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = load_geometry_file(Path::new("model.3ds"))?;
    let glb = export_to_glb(&scene)?;
    std::fs::write("output.glb", glb)?;
    Ok(())
}
```

## Documentation

- [Understanding OFML](docs/OFML-EXPLAINED.md) - **Start here!** What is OFML, why CLS files exist, file formats explained
- [Full Documentation](docs/README.md) - Comprehensive feature and command documentation
- [Quickstart Guide](docs/QUICKSTART.md) - Get started in 5 minutes
- [API Reference](docs/API.md) - Rust library API for developers

## Testing

```bash
cargo test              # Run all tests (339 tests)
cargo test --release    # Faster test execution
cargo bench             # Run benchmarks
```

## Statistics

- **Tests**: 339 passing
- **Code Coverage**: ~49%
- **Manufacturers Verified**: 28+
- **Modules**: 14 core modules

## Architecture

```
src/
├── lexer.rs         # CLS tokenizer
├── parser.rs        # CLS parser
├── interpreter.rs   # CLS execution
├── geometry.rs      # 3D geometry (3DS/GEO/OBJ/GLB)
├── geometry2d.rs    # 2D geometry (SVG)
├── ebase.rs         # EBASE database reader
├── ebase_expr.rs    # PostScript expression evaluator
├── ofml.rs          # ALB archive handling
├── ofml_classes.rs  # OFML framework classes (Go*, Oi*)
├── material.rs      # Material system
├── operations.rs    # High-level reusable operations
└── ...
```

## OFML Compliance

Implements the following OFML specifications:
- OFML 2.0 R3 (core language)
- GO 1.12.0 (geometry operations)
- ODB 2.4 (EBASE database)
- OMATS 2.2 (materials)
- OLAYERS 1.3.1 (2D layers)

## License

Part of the OfficeRocket4000 project.
