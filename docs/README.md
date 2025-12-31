# OFML Interpreter Documentation

**Version**: 1.0.0
**Last Updated**: 2025-12-22

A comprehensive Rust implementation of the Office Furniture Modeling Language (OFML) for parsing, executing, and converting furniture product data to modern 3D formats.

---

## Related Documentation

- **[Understanding OFML](./OFML-EXPLAINED.md)** - Start here if you're new! Explains what OFML is, why CLS files exist, and what each file format is used for.
- **[CLS Examples](./CLS-EXAMPLES.md)** - Real CLS scripts from manufacturers with line-by-line explanations
- [Quickstart Guide](./QUICKSTART.md) - Get started in 5 minutes
- [API Reference](./API.md) - Rust library API for developers

---

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Supported File Formats](#supported-file-formats)
5. [CLI Commands Reference](#cli-commands-reference)
6. [Workflows](#workflows)
7. [Architecture](#architecture)
8. [Library API](#library-api)
9. [OFML Language Features](#ofml-language-features)
10. [Troubleshooting](#troubleshooting)

---

## Overview

### What is OFML?

OFML (Office Furniture Modeling Language) is an industry-standard format used by furniture manufacturers to distribute 3D product data. It includes:

- **CLS files**: Class definition scripts that define parametric furniture geometry
- **EBASE databases**: Binary databases containing product catalogs, geometry references, and configuration data
- **ALB archives**: Encrypted ZIP archives containing geometry, textures, and CLS scripts
- **Geometry files**: 3DS, GEO, and OBJ formats for 3D models
- **Material files**: MAT files defining surface properties and textures

### What This Interpreter Does

The OFML Interpreter provides:

1. **CLS Script Execution**: Parse and execute OFML class scripts
2. **Geometry Conversion**: Convert 3DS/GEO/OBJ to modern glTF/GLB format
3. **Product Assembly**: Assemble complete furniture products from EBASE data
4. **2D Floor Plans**: Generate SVG floor plans from odb2d data
5. **Data Exploration**: Browse and extract OFML data archives

### Key Features

| Feature | Description |
|---------|-------------|
| Full CLS Parser | Recursive descent parser for OFML 2.0 syntax |
| PostScript Evaluator | EBASE expression evaluator (ctor fields) |
| 3D Geometry Pipeline | 3DS, GEO, OBJ parsing with GLB export |
| 2D Geometry Pipeline | odb2d processing with SVG export |
| Material System | MAT file parsing, texture loading (PNG/JPG/TGA/BMP) |
| Property System | Product configuration with variants |
| Attachment Points | Connection point system for furniture assembly |
| ALB Archive Support | Encrypted archive extraction |

---

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Building from Source

```bash
cd ofml-interpreter
cargo build --release
```

The binary will be at `target/release/ofml`.

### Running Tests

```bash
cargo test                    # Run all tests
cargo test --release          # Run tests in release mode (faster)
cargo test ebase              # Run EBASE-related tests only
cargo test geometry           # Run geometry tests only
```

### Benchmarks

```bash
cargo bench                   # Run all benchmarks
```

---

## Quick Start

### Converting a 3DS File to GLB

```bash
# Convert a single geometry file
./ofml convert path/to/model.3ds

# Output: path/to/model.glb
```

### Exploring OFML Product Data

```bash
# List all manufacturers in an OFML data directory
./ofml ofml /path/to/ofmldata

# List products for a specific manufacturer
./ofml ofml /path/to/ofmldata vitra

# Show details for a specific product
./ofml ofml /path/to/ofmldata vitra ac
```

### Assembling a Product

```bash
# List available articles in a product
./ofml product /path/to/ofmldata/vitra/ac/1

# Assemble a specific article
./ofml product /path/to/ofmldata/vitra/ac/1 AC_CHAIR output.glb
```

### Running a CLS Script

```bash
# Execute a CLS file and export to GLB
./ofml export script.cls

# Just check syntax
./ofml check script.cls
```

---

## Supported File Formats

### Input Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| 3D Studio | `.3ds` | Legacy 3D Studio MAX format |
| GEO | `.geo` | OFML-specific geometry format |
| OBJ | `.obj` | Wavefront OBJ format |
| EBASE | `.ebase` | Binary OFML database |
| ALB | `.alb` | Encrypted OFML archive |
| CLS | `.cls` | OFML class script |
| MAT | `.mat` | Material definition |
| PNG/JPG/TGA/BMP | Various | Texture images |

### Output Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| GLB | `.glb` | Binary glTF 2.0 (recommended) |
| SVG | `.svg` | 2D floor plan vector graphics |

---

## CLI Commands Reference

### Geometry Commands

#### `convert` - Convert Geometry to GLB

Converts 3DS, GEO, or OBJ files to GLB format.

```bash
./ofml convert <input_file>
```

**Example:**
```bash
./ofml convert chair.3ds
# Output: chair.glb
```

**What it does:**
1. Detects format by file extension
2. Parses geometry (vertices, faces, normals)
3. Converts coordinate system (Y-up for glTF)
4. Exports to binary glTF

---

#### `merge` - Merge Multiple Geometries

Combines multiple geometry files into a single GLB.

```bash
./ofml merge <output.glb> <file1> <file2> [file3...]
```

**Example:**
```bash
./ofml merge furniture.glb seat.geo back.geo armrest_left.geo armrest_right.geo
```

**What it does:**
1. Loads all input geometry files
2. Renames meshes to prevent conflicts
3. Combines into single scene
4. Exports merged GLB

---

#### `validate` - Validate Geometry

Checks geometry for issues and displays metrics.

```bash
./ofml validate <geometry_file>
```

**Example:**
```bash
./ofml validate model.3ds
```

**Output includes:**
- Vertex count, face count, mesh count
- Bounding box dimensions
- Warnings (large dimensions, degenerate triangles)
- Errors (missing vertices, invalid data)

---

### Product Assembly Commands

#### `product` - Assemble Product from OFML Data

Assembles a complete product from EBASE database and geometry files.

```bash
# List available articles
./ofml product <product_path>

# Assemble specific article
./ofml product <product_path> <article_name> [output.glb]

# Assemble all articles
./ofml product <product_path> output.glb
```

**Example:**
```bash
# List articles
./ofml product /ofmldata/vitra/ac/1
# Output: Available articles: AC_CHAIR, AC_STOOL, AC_TABLE

# Assemble one article
./ofml product /ofmldata/vitra/ac/1 AC_CHAIR chair.glb
```

**What it does:**
1. Locates `odb.ebase` file in product directory
2. Reads `odb3d` table to find geometry references
3. Parses `ctor` field to extract geometry filenames and scales
4. Loads geometry from disk or ALB archive
5. Applies position offsets and scale factors
6. Combines all meshes into single GLB

**Note:** This command does NOT execute CLS scripts. It only processes simple geometry imports from the EBASE `ctor` field. For full parametric geometry, use the `build` command.

---

#### `gsx` - Convert GSX/Sedus Products

Specialized command for products using OBJ geometry (common in GSX/Sedus products).

```bash
./ofml gsx <product_path> [output.glb]
```

**Example:**
```bash
./ofml gsx /ofmldata/gsx/eo/1 eo_chair.glb
```

**What it does:**
1. Reads EBASE odb3d table
2. Loads OBJ geometry files (preferred over 3DS/GEO)
3. Applies transforms
4. Exports to GLB

---

#### `build` - Build Product with CLS Execution

Fully executes CLS scripts from ALB archive to build parametric geometry.

```bash
./ofml build <alb_file> [class_name]
```

**Example:**
```bash
./ofml build /ofmldata/vitra/ac/1/data.alb MyChairClass
```

**What it does:**
1. Opens ALB archive
2. Extracts and parses all CLS files
3. Executes CLS code with interpreter
4. Builds scene graph from executed geometry
5. Exports final GLB

**When to use:** Use `build` instead of `product` when:
- Products have parametric geometry (dimension-dependent shapes)
- CLS scripts contain `clsref` operators (class references)
- Complex conditional geometry is needed

---

### 2D Floor Plan Commands

#### `svg` - Export 2D Floor Plan

Generates SVG floor plan from EBASE odb2d table.

```bash
./ofml svg <ebase_path> [output.svg]
```

**Example:**
```bash
./ofml svg /ofmldata/vitra/ac/1/odb.ebase floorplan.svg
```

**What it does:**
1. Reads `odb2d` table from EBASE
2. Processes 2D primitives (lines, polygons, arcs, text)
3. Applies 2D transforms
4. Generates SVG output

**Supported 2D primitives:**
- G2DLines, G2DLineStrip, G2DLineLoop
- G2DPolygon, G2DRectangle
- G2DCircle, G2DEllipse, G2DArc
- G2DText (with font attributes)
- G2DPoints

---

### EBASE Database Commands

#### `ebase` - Explore EBASE Database

Dumps contents of EBASE database tables.

```bash
# List all tables
./ofml ebase <ebase_path>

# Dump specific table
./ofml ebase <ebase_path> <table_name>
```

**Example:**
```bash
# List tables
./ofml ebase /ofmldata/vitra/ac/1/odb.ebase
# Output: Tables: odb3d, odb2d, mat, attpt, oppattpt, stdattpt, ...

# Dump odb3d table
./ofml ebase /ofmldata/vitra/ac/1/odb.ebase odb3d
```

**Common EBASE tables:**

| Table | Description |
|-------|-------------|
| `odb3d` | 3D geometry references with constructors |
| `odb2d` | 2D floor plan geometry |
| `mat` | Material definitions |
| `attpt` | Attachment points |
| `oppattpt` | Opposite attachment points |
| `stdattpt` | Standard attachment points |
| `article` | Article catalog |
| `variant` | Product variants |
| `propdef` | Property definitions |

---

#### `expr` - Evaluate EBASE Expression

Evaluates a PostScript-like EBASE expression.

```bash
./ofml expr "<expression>"
```

**Example:**
```bash
./ofml expr '"chair.geo" 1 1 1 imp'
# Output: Import: chair.geo with scale [1, 1, 1]

./ofml expr '10 20 add 5 mul'
# Output: Stack: [150.0]
```

**Supported operators:**
- Arithmetic: `add`, `sub`, `mul`, `div`, `neg`
- Comparison: `eq`, `ne`, `lt`, `gt`, `le`, `ge`
- Stack: `dup`, `pop`, `exch`, `roll`
- Control: `if`, `ifelse`
- OFML: `imp` (import), `clsref` (class reference), `egms`

---

### ALB Archive Commands

#### `alb` - List/Extract ALB Contents

Lists or extracts files from encrypted ALB archives.

```bash
# List all files
./ofml alb <alb_path>

# List files matching pattern
./ofml alb <alb_path> "*.cls"

# Extract to directory
./ofml alb <alb_path> "*" <output_dir>
```

**Example:**
```bash
# List contents
./ofml alb data.alb
# Output: 45 files (geometry: 12, textures: 8, scripts: 5, ...)

# Extract all CLS files
./ofml alb data.alb "*.cls" ./extracted/
```

---

#### `cls` - List/Extract CLS Scripts

Specialized command for CLS files in ALB archives.

```bash
# List CLS files
./ofml cls <alb_path>

# Extract specific CLS
./ofml cls <alb_path> <filename>
```

**Example:**
```bash
./ofml cls data.alb
# Output: CLS Files: Chair.cls, Table.cls, Common.cls

./ofml cls data.alb Chair.cls
# Output: (CLS source code)
```

---

#### `extract` - Extract Single File

Extracts a specific file from ALB archive.

```bash
./ofml extract <alb_path> <filename>
```

**Example:**
```bash
./ofml extract data.alb textures/wood.png > wood.png
```

---

### OFML Data Explorer

#### `ofml` - Browse OFML Data Directory

Explores OFML data directory structure.

```bash
# List manufacturers
./ofml ofml <data_path>

# List products for manufacturer
./ofml ofml <data_path> <manufacturer>

# Show product details
./ofml ofml <data_path> <manufacturer> <product>
```

**Example:**
```bash
# List all manufacturers
./ofml ofml /ofmldata
# Output: Found 28 manufacturers: aix, arper, buzzispace, cassina, cor, ...

# List Vitra products
./ofml ofml /ofmldata vitra
# Output: Products: ac, ad, allstar, ...

# Show product info
./ofml ofml /ofmldata vitra ac
# Output: Version: 1, Tables: [odb3d, odb2d, mat], ALB: data.alb
```

---

### CLS Script Commands

#### `tokenize` - Show Tokens

Displays lexer tokens from CLS source.

```bash
./ofml tokenize <cls_file>
```

**Example:**
```bash
./ofml tokenize script.cls
# Output: Token list with positions
```

---

#### `parse` - Show AST

Parses CLS and displays Abstract Syntax Tree.

```bash
./ofml parse <cls_file>
```

**Example:**
```bash
./ofml parse script.cls
# Output: AST structure with classes, functions, statements
```

---

#### `check` - Syntax Check

Validates CLS syntax without execution.

```bash
./ofml check <cls_file>
```

**Example:**
```bash
./ofml check script.cls
# Output: script.cls: OK (Classes: 2, Functions: 5, Variables: 3)
```

---

#### `run` - Execute CLS

Executes CLS script and shows results.

```bash
./ofml run <cls_file>
```

**Example:**
```bash
./ofml run script.cls
# Output: Execution results, created objects, scene info
```

---

#### `export` - Execute and Export

Executes CLS script and exports scene to GLB.

```bash
./ofml export <cls_file>
```

**Example:**
```bash
./ofml export chair.cls
# Output: chair.glb
```

---

## Workflows

### Workflow 1: Converting Manufacturer Data to GLB

**Goal:** Convert OFML product data to GLB for use in web viewers or 3D applications.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  OFML Data      │────>│  OFML CLI       │────>│  GLB Output     │
│  (ofmldata/)    │     │  (product cmd)  │     │  (model.glb)    │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

**Steps:**

1. **Explore available data:**
   ```bash
   ./ofml ofml /ofmldata
   ./ofml ofml /ofmldata vitra
   ```

2. **Check what's in a product:**
   ```bash
   ./ofml product /ofmldata/vitra/ac/1
   ```

3. **Convert to GLB:**
   ```bash
   ./ofml product /ofmldata/vitra/ac/1 AC_CHAIR output.glb
   ```

4. **Validate output:**
   ```bash
   # Use glTF validator or load in 3D viewer
   ```

---

### Workflow 2: Generating Floor Plans

**Goal:** Create 2D SVG floor plans from OFML product data.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  odb.ebase      │────>│  svg command    │────>│  floorplan.svg  │
│  (odb2d table)  │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

**Steps:**

1. **Check if odb2d exists:**
   ```bash
   ./ofml ebase /ofmldata/vitra/ac/1/odb.ebase
   # Look for "odb2d" in table list
   ```

2. **Generate SVG:**
   ```bash
   ./ofml svg /ofmldata/vitra/ac/1/odb.ebase chair_plan.svg
   ```

3. **View in browser or vector editor**

---

### Workflow 3: Full CLS Execution (Parametric Products)

**Goal:** Build products with full parametric geometry from CLS scripts.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  ALB Archive    │────>│  build command  │────>│  GLB Output     │
│  (CLS scripts)  │     │  (interpreter)  │     │  (parametric)   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

**Steps:**

1. **List CLS files in archive:**
   ```bash
   ./ofml cls /ofmldata/vitra/ac/1/data.alb
   ```

2. **View a CLS file:**
   ```bash
   ./ofml cls /ofmldata/vitra/ac/1/data.alb Chair.cls
   ```

3. **Build with interpreter:**
   ```bash
   ./ofml build /ofmldata/vitra/ac/1/data.alb ChairClass
   ```

---

### Workflow 4: Batch Processing Multiple Products

**Goal:** Convert all products from a manufacturer.

**Script example:**
```bash
#!/bin/bash
MANUFACTURER="vitra"
DATA_DIR="/ofmldata"
OUTPUT_DIR="./output"

mkdir -p "$OUTPUT_DIR"

# Get list of products
for product in $(ls "$DATA_DIR/$MANUFACTURER"); do
    echo "Processing $MANUFACTURER/$product..."
    ./ofml product "$DATA_DIR/$MANUFACTURER/$product/1" "$OUTPUT_DIR/${MANUFACTURER}_${product}.glb" 2>/dev/null
done

echo "Done! Check $OUTPUT_DIR"
```

---

### Workflow 5: Debugging EBASE Data

**Goal:** Understand structure of OFML product data.

**Steps:**

1. **List all tables:**
   ```bash
   ./ofml ebase /path/to/odb.ebase
   ```

2. **Examine odb3d (geometry references):**
   ```bash
   ./ofml ebase /path/to/odb.ebase odb3d
   ```

3. **Examine materials:**
   ```bash
   ./ofml ebase /path/to/odb.ebase mat
   ```

4. **Test ctor expressions:**
   ```bash
   # Copy a ctor value from odb3d output
   ./ofml expr '"chair.geo" 0.8 1.0 0.8 imp'
   ```

---

## Architecture

### Module Overview

```
ofml-interpreter/
├── src/
│   ├── lib.rs              # Library entry point, re-exports
│   ├── main.rs             # CLI application
│   │
│   ├── lexer.rs            # CLS tokenizer
│   ├── parser.rs           # CLS recursive descent parser
│   ├── ast.rs              # Abstract Syntax Tree types
│   ├── interpreter.rs      # CLS runtime execution
│   ├── env.rs              # Variable environment/scoping
│   ├── value.rs            # Runtime value types
│   │
│   ├── geometry.rs         # 3D geometry (3DS, GEO, OBJ, GLB)
│   ├── geometry2d.rs       # 2D geometry (floor plans, SVG)
│   ├── scene.rs            # Scene graph for 3D composition
│   │
│   ├── ebase.rs            # EBASE binary database reader
│   ├── ebase_expr.rs       # PostScript expression evaluator
│   │
│   ├── ofml.rs             # ALB archive handling
│   ├── ofml_classes.rs     # OFML framework classes (Go*, Oi*)
│   │
│   ├── material.rs         # Material system
│   ├── texture.rs          # Texture loading/caching
│   │
│   ├── property.rs         # Property system (configuration)
│   ├── article.rs          # Article/variant loading
│   ├── attachment.rs       # Attachment point system
│   ├── gobject.rs          # GObject type system
│   │
│   ├── operations.rs       # High-level reusable operations
│   └── errors.rs           # Error types
│
├── tests/
│   ├── *.rs                # Unit and integration tests
│   └── fixtures/           # Test data files
│
└── benches/
    └── *.rs                # Performance benchmarks
```

### Data Flow

```
                                    ┌─────────────────┐
                                    │   CLS Source    │
                                    │   (.cls file)   │
                                    └────────┬────────┘
                                             │
                                    ┌────────▼────────┐
                                    │     Lexer       │
                                    │  (tokenize)     │
                                    └────────┬────────┘
                                             │
                                    ┌────────▼────────┐
                                    │     Parser      │
                                    │  (parse to AST) │
                                    └────────┬────────┘
                                             │
                                    ┌────────▼────────┐
                                    │   Interpreter   │
                                    │  (execute AST)  │
                                    └────────┬────────┘
                                             │
┌─────────────┐                     ┌────────▼────────┐
│  Geometry   │────────────────────>│   Scene Graph   │
│ (3DS/GEO/OBJ)                     │                 │
└─────────────┘                     └────────┬────────┘
                                             │
                                    ┌────────▼────────┐
                                    │   GLB Export    │
                                    │                 │
                                    └────────┬────────┘
                                             │
                                    ┌────────▼────────┐
                                    │   Output GLB    │
                                    └─────────────────┘
```

### Key Classes

| Class | Purpose |
|-------|---------|
| `Interpreter` | Executes CLS AST, manages environment |
| `SceneGraph` | Hierarchical 3D scene composition |
| `EBaseReader` | Binary EBASE database access |
| `EbaseEvaluator` | PostScript expression evaluation |
| `AlbArchive` | Encrypted ZIP archive handling |
| `Scene3DS` | 3D geometry container (meshes, materials) |
| `G2DCompound` | 2D geometry container |

---

## Library API

The interpreter can be used as a Rust library:

### Basic Usage

```rust
use ofml_interpreter::{Parser, Interpreter};

fn main() {
    // Parse CLS source
    let source = r#"
        class MyClass {
            func greet() {
                return "Hello, OFML!";
            }
        }
        var obj = MyClass();
        var msg = obj.greet();
    "#;

    let mut parser = Parser::new(source).unwrap();
    let ast = parser.parse().unwrap();

    // Execute
    let mut interp = Interpreter::new();
    interp.execute(&ast).unwrap();

    // Access results
    let msg = interp.env.get("msg").unwrap();
    println!("Result: {:?}", msg);
}
```

### Geometry Conversion

```rust
use ofml_interpreter::operations::{load_geometry_file, export_to_glb};
use std::path::Path;

fn convert_to_glb(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scene = load_geometry_file(Path::new(input))?;
    let glb_data = export_to_glb(&scene)?;
    std::fs::write(output, glb_data)?;
    Ok(())
}
```

### EBASE Reading

```rust
use ofml_interpreter::ebase::EBaseReader;

fn read_ebase(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = EBaseReader::open(path)?;

    // List tables
    for table in reader.tables.keys() {
        println!("Table: {}", table);
    }

    // Read records
    let records = reader.read_records("odb3d", None)?;
    for record in records {
        println!("Record: {:?}", record);
    }

    Ok(())
}
```

### Expression Evaluation

```rust
use ofml_interpreter::ebase_expr::EbaseEvaluator;
use std::collections::HashMap;

fn evaluate_ctor(expr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut evaluator = EbaseEvaluator::new();
    let props = HashMap::new();

    let result = evaluator.evaluate(expr, &props)?;
    println!("Result: {:?}", result);

    Ok(())
}
```

---

## OFML Language Features

### Supported CLS Syntax

```cls
// Package declaration
package ::manufacturer::product;

// Imports
import ::ofml::oi::*;
import ::ofml::go::GoYLTrans;

// Class definition
class MyChair : OiPart {
    // Instance variables
    var width = 0.6;
    var height = 0.8;

    // Constructor (called automatically)
    func initialize() {
        // Create geometry
        OiBlock(self, @seat, [width, 0.05, 0.5]);

        // Set material
        setMaterial("::materials::wood::oak");

        // Setup properties
        setupProperty(@width, ["Width", NULL, NULL, 0, "fl 0.4 0.8"], 1);
        setPropValue(@width, 0.6);
    }

    // Methods
    func getWidth() {
        return width;
    }
}

// Create instance
var chair = MyChair();
```

### Supported OFML Classes

| Class | Description |
|-------|-------------|
| `OiPart` | Base class for furniture parts |
| `OiBlock` | Cuboid primitive geometry |
| `OiCylinder` | Cylinder primitive |
| `OiSphere` | Sphere primitive |
| `OiEllipsoid` | Ellipsoid primitive |
| `OiPolygon` | Planar polygon |
| `OiImport` | External geometry import |
| `OiFrame` | Frame/border geometry |
| `OiRotation` | Rotational sweep |
| `OiSweep` | Extrusion/sweep |
| `GoYLTrans` | Y-axis stretching |
| `GoXLTrans` | X-axis stretching |
| `GoZLTrans` | Z-axis stretching |
| `GoXLRTransYLRTrans` | Combined X/Y stretching |
| `GoMirror` | Geometry mirroring |

### Built-in Functions

| Function | Description |
|----------|-------------|
| `setPropValue(key, value)` | Set property value |
| `getPropValue(key)` | Get property value |
| `setupProperty(key, def, order)` | Define property |
| `setPropState(key, state)` | Set property state |
| `getPropState(key)` | Get property state |
| `removeProperty(key)` | Remove property |
| `getPropertyKeys()` | List property keys |
| `setMaterial(name)` | Apply material |
| `getMaterial()` | Get current material |
| `getPDManager()` | Get product database manager |
| `fabs(x)` | Absolute value |
| `Mod(a, b)` | Division with remainder |
| `sin(x)`, `cos(x)`, `tan(x)` | Trigonometry |
| `sqrt(x)` | Square root |
| `min(a, b)`, `max(a, b)` | Min/max |

---

## Troubleshooting

### Common Issues

#### "odb.ebase not found"

**Problem:** The `product` command cannot find the EBASE database.

**Solution:** Check the directory structure. OFML data typically uses:
```
/manufacturer/product/1/odb.ebase
```
The version number directory (1, 2, etc.) is required.

---

#### "No geometry found"

**Problem:** Product assembly produces empty output.

**Solutions:**
1. Check if geometry files exist:
   ```bash
   ./ofml alb /path/to/data.alb "*.geo"
   ./ofml alb /path/to/data.alb "*.3ds"
   ```

2. Check the ctor expressions:
   ```bash
   ./ofml ebase /path/to/odb.ebase odb3d
   ```

3. Try the `gsx` command for OBJ-based products:
   ```bash
   ./ofml gsx /path/to/product output.glb
   ```

---

#### "Invalid ALB archive"

**Problem:** Cannot open ALB file.

**Solutions:**
1. Verify the file is a valid ALB (encrypted ZIP):
   ```bash
   file /path/to/data.alb
   # Should show: Zip archive data
   ```

2. The built-in decryption password may not work for all archives.

---

#### "CLS parse error"

**Problem:** CLS file fails to parse.

**Solutions:**
1. Check syntax:
   ```bash
   ./ofml check script.cls
   ```

2. View tokens to find issue location:
   ```bash
   ./ofml tokenize script.cls
   ```

3. Some CLS features may not be fully implemented yet.

---

#### GLB file appears empty in viewer

**Problem:** Exported GLB loads but shows nothing.

**Solutions:**
1. Check geometry dimensions:
   ```bash
   ./ofml validate input.3ds
   ```

2. OFML geometry may use meters. Scale in viewer if needed.

3. Check materials - may need texture files.

---

### Getting Help

- Check existing issues on GitHub
- Run commands with verbose output where available
- Examine EBASE data to understand product structure

---

## Performance

### Benchmarks (typical results)

| Operation | Time |
|-----------|------|
| Parse small OBJ (1K vertices) | ~1ms |
| Parse large OBJ (100K vertices) | ~50ms |
| 3DS to GLB conversion | ~10ms |
| EBASE table read | ~5ms |
| CLS parse + execute | ~20ms |

### Memory Usage

- Small products: < 50MB
- Large products: < 200MB
- Maximum recommended: 512MB

---

## License

This project is part of the OfficeRocket4000 system.

---

## Version History

### 1.0.0 (2025-12-22)

- Initial release
- Full CLS parser and interpreter
- 3DS/GEO/OBJ to GLB conversion
- 2D SVG floor plan generation
- EBASE database support
- ALB archive extraction
- 339 passing tests
- 28+ manufacturer compatibility verified
