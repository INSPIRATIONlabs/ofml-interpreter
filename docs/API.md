# OFML Interpreter API Reference

This document describes the public Rust API for using the OFML Interpreter as a library.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Modules](#core-modules)
3. [Operations Module](#operations-module)
4. [Geometry Module](#geometry-module)
5. [EBASE Module](#ebase-module)
6. [Interpreter Module](#interpreter-module)
7. [2D Geometry Module](#2d-geometry-module)
8. [Material System](#material-system)
9. [Error Handling](#error-handling)

---

## Getting Started

### Add to Cargo.toml

```toml
[dependencies]
ofml-interpreter = { path = "../ofml-interpreter" }
```

### Basic Import

```rust
use ofml_interpreter::{
    // Core types
    Interpreter, Parser, Value,

    // Geometry
    geometry::{Scene3DS, Mesh, Vertex, Face},

    // Operations (high-level API)
    operations::{
        load_geometry_file, export_to_glb, validate_geometry,
        assemble_product, ProductConfig, ProductResult,
    },

    // EBASE
    ebase::{EBaseReader, Odb3dRecord, Odb2dRecord},

    // Expression evaluator
    ebase_expr::{EbaseEvaluator, EbaseResult, EbaseValue},
};
```

---

## Core Modules

### Module: `value`

Runtime value types for the interpreter.

```rust
use ofml_interpreter::Value;

// Value variants
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),           // @symbol
    Array(Rc<RefCell<Vec<Value>>>),
    Object(Rc<RefCell<ObjInstance>>),
    Class(ClassValue),
    Func(FuncValue),
    NativeFunc(NativeFuncValue),
}

// Conversion helpers
let v = Value::Int(42);
if let Value::Int(n) = v {
    println!("Got integer: {}", n);
}

// Array creation
let arr = Value::array(vec![Value::Int(1), Value::Int(2)]);

// Check types
v.is_null();
v.is_truthy();
v.type_name();  // "Int", "Float", "Array", etc.
```

### Module: `env`

Variable environment with scoping.

```rust
use ofml_interpreter::{Environment, Scope};

let mut env = Environment::new();

// Define variables
env.define("x", Value::Int(10));
env.define("name", Value::String("test".to_string()));

// Get variables
let x = env.get("x");  // Option<Value>

// Scoped execution
env.push_scope(Scope::Block);
env.define("local", Value::Int(20));
// ... local is visible here ...
env.pop_scope();
// ... local is gone ...

// Set existing variable
env.set("x", Value::Int(100));
```

---

## Operations Module

High-level operations for common tasks. **This is the recommended API for most use cases.**

### Load Geometry

```rust
use ofml_interpreter::operations::{load_geometry_file, load_geometry_data};
use std::path::Path;

// Load from file path (auto-detects format by extension)
let scene = load_geometry_file(Path::new("model.3ds"))?;
let scene = load_geometry_file(Path::new("model.geo"))?;
let scene = load_geometry_file(Path::new("model.obj"))?;

// Load from raw bytes (needs path for format detection)
let data = std::fs::read("model.3ds")?;
let scene = load_geometry_data(&data, Path::new("model.3ds"))?;
```

### Export to GLB

```rust
use ofml_interpreter::operations::export_to_glb;

let scene = load_geometry_file(Path::new("model.3ds"))?;
let glb_bytes = export_to_glb(&scene)?;
std::fs::write("output.glb", glb_bytes)?;
```

### Validate Geometry

```rust
use ofml_interpreter::operations::{validate_geometry, ValidationResult};

let scene = load_geometry_file(Path::new("model.3ds"))?;
let result: ValidationResult = validate_geometry(&scene);

println!("Valid: {}", result.is_valid);
println!("Vertices: {}", result.vertex_count);
println!("Faces: {}", result.face_count);
println!("Meshes: {}", result.mesh_count);
println!("Materials: {}", result.material_count);
println!("Bounding box: {:?}", result.bounding_box);

for warning in &result.warnings {
    eprintln!("Warning: {}", warning);
}
for error in &result.errors {
    eprintln!("Error: {}", error);
}
```

### Merge Multiple Geometries

```rust
use ofml_interpreter::operations::{load_and_merge_geometry, merge_scenes};

// Load and merge multiple files
let paths = [Path::new("a.geo"), Path::new("b.geo"), Path::new("c.geo")];
let refs: Vec<&Path> = paths.iter().map(|p| *p).collect();
let combined = load_and_merge_geometry(&refs)?;

// Or merge existing scenes
let scene1 = load_geometry_file(Path::new("a.geo"))?;
let scene2 = load_geometry_file(Path::new("b.geo"))?;
let combined = merge_scenes(vec![scene1, scene2]);
```

### Apply Transforms

```rust
use ofml_interpreter::operations::apply_transforms;

let mut scene = load_geometry_file(Path::new("model.geo"))?;

// Apply offset and scale
let offset = [0.0, 0.5, 0.0];  // Move up 0.5 meters
let scale = [1.0, 1.0, 1.0];   // No scaling
apply_transforms(&mut scene, &offset, &scale);
```

### Assemble Product

```rust
use ofml_interpreter::operations::{assemble_product, ProductConfig, ProductResult};
use std::collections::HashMap;

// Basic assembly
let config = ProductConfig::default();
let result: ProductResult = assemble_product(Path::new("/ofmldata/vitra/ac/1"), &config)?;

// Assembly with specific article
let config = ProductConfig {
    article: Some("AC_CHAIR".to_string()),
    properties: HashMap::new(),
};
let result = assemble_product(Path::new("/ofmldata/vitra/ac/1"), &config)?;

// Access results
println!("Articles found: {:?}", result.articles_found);
println!("Geometry loaded: {}", result.geometry_loaded);
println!("Missing geometry: {:?}", result.geometry_missing);

// Export the scene
let glb = export_to_glb(&result.scene)?;
```

### Export 2D Floor Plan

```rust
use ofml_interpreter::operations::export_2d_floorplan;
use ofml_interpreter::geometry2d::G2DCompound;

let compound: G2DCompound = export_2d_floorplan(Path::new("/path/to/odb.ebase"))?;

// Convert to SVG
let svg = compound.to_svg();
std::fs::write("floorplan.svg", svg)?;
```

### Evaluate Expression

```rust
use ofml_interpreter::operations::evaluate_expression;
use std::collections::HashMap;

let props = HashMap::new();
let result = evaluate_expression(r#""chair.geo" 1 1 1 imp"#, &props)?;

// Result can be:
// - EbaseResult::Import { filename, scale }
// - EbaseResult::ClassRef { classname, params }
// - EbaseResult::Value(stack)
```

---

## Geometry Module

Low-level 3D geometry handling.

### Scene3DS

Container for 3D geometry.

```rust
use ofml_interpreter::geometry::{Scene3DS, Mesh, Vertex, Face, Material3DS};

// Create scene
let mut scene = Scene3DS::default();

// Add mesh
let mesh = Mesh {
    name: "cube".to_string(),
    vertices: vec![
        Vertex { x: 0.0, y: 0.0, z: 0.0 },
        Vertex { x: 1.0, y: 0.0, z: 0.0 },
        Vertex { x: 1.0, y: 1.0, z: 0.0 },
        Vertex { x: 0.0, y: 1.0, z: 0.0 },
    ],
    faces: vec![
        Face { a: 0, b: 1, c: 2, flags: 0 },
        Face { a: 0, b: 2, c: 3, flags: 0 },
    ],
    normals: vec![],
    tex_coords: vec![],
    material_name: Some("default".to_string()),
    transform: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    coord_system: CoordSystem::YupGltf,
};
scene.meshes.push(mesh);

// Add material
scene.materials.insert("default".to_string(), Material3DS {
    name: "default".to_string(),
    diffuse: [0.8, 0.8, 0.8],
    ambient: [0.2, 0.2, 0.2],
    specular: [0.3, 0.3, 0.3],
    texture: None,
    metallic: 0.0,
    roughness: 0.5,
});

// Query scene
let bbox = scene.bounding_box();
let dims = bbox.dimensions();
let vertex_count = scene.vertex_count();
let face_count = scene.face_count();
```

### Parsing Functions

```rust
use ofml_interpreter::geometry::{parse_3ds, parse_geo, parse_obj, scene_to_glb};

// Parse 3DS
let data = std::fs::read("model.3ds")?;
let scene = parse_3ds(&data)?;

// Parse GEO (OFML format)
let data = std::fs::read("model.geo")?;
let scene = parse_geo(&data)?;

// Parse OBJ
let data = std::fs::read("model.obj")?;
let scene = parse_obj(&data)?;

// Export to GLB
let glb_bytes = scene_to_glb(&scene)?;
```

### Bounding Box

```rust
use ofml_interpreter::geometry::BoundingBox;

let scene = load_geometry_file(Path::new("model.3ds"))?;
let bbox: BoundingBox = scene.bounding_box();

println!("Min: [{}, {}, {}]", bbox.min[0], bbox.min[1], bbox.min[2]);
println!("Max: [{}, {}, {}]", bbox.max[0], bbox.max[1], bbox.max[2]);

let dims = bbox.dimensions();  // [width, height, depth]
let center = bbox.center();    // [x, y, z]
let is_valid = bbox.is_valid();
```

---

## EBASE Module

Reading OFML binary databases.

### EBaseReader

```rust
use ofml_interpreter::ebase::EBaseReader;

// Open database
let mut reader = EBaseReader::open("/path/to/odb.ebase")?;

// List tables
for (name, info) in &reader.tables {
    println!("Table: {} ({} records)", name, info.record_count);
}

// Check if table exists
if reader.tables.contains_key("odb3d") {
    // Read all records
    let records = reader.read_records("odb3d", None)?;

    for record in &records {
        // record is HashMap<String, String>
        println!("Record: {:?}", record);
    }
}

// Read specific columns
let records = reader.read_records("odb3d", Some(&["odb_name", "ctor", "x_offs"]))?;
```

### Odb3dRecord

Helper for parsing odb3d records.

```rust
use ofml_interpreter::ebase::{EBaseReader, Odb3dRecord};

let mut reader = EBaseReader::open("/path/to/odb.ebase")?;
let records = reader.read_records("odb3d", None)?;

for record in &records {
    if let Some(odb_rec) = Odb3dRecord::from_record(record) {
        println!("Name: {}", odb_rec.odb_name);
        println!("Ctor: {}", odb_rec.ctor);

        // Parse constructor expression
        if let Some((geo_name, scale)) = odb_rec.parse_ctor() {
            println!("Geometry: {} @ scale {:?}", geo_name, scale);
        }

        // Get position offset
        let offset = odb_rec.parse_offset();  // [x, y, z]
    }
}
```

### Odb2dRecord

Helper for parsing odb2d records.

```rust
use ofml_interpreter::ebase::{EBaseReader, Odb2dRecord};

let mut reader = EBaseReader::open("/path/to/odb.ebase")?;
let records = reader.read_records("odb2d", None)?;

for record in &records {
    if let Some(odb2d) = Odb2dRecord::from_record(record) {
        println!("Type: {}", odb2d.prim_type);  // G2DLines, G2DPolygon, etc.
        println!("Coords: {:?}", odb2d.coords);
        println!("Layer: {}", odb2d.layer);
    }
}
```

---

## Interpreter Module

CLS script execution.

### Basic Execution

```rust
use ofml_interpreter::{Parser, Interpreter, Value};

// Parse source code
let source = r#"
    class Calculator {
        func add(a, b) {
            return a + b;
        }
    }
    var calc = Calculator();
    var result = calc.add(10, 20);
"#;

let mut parser = Parser::new(source)?;
let ast = parser.parse()?;

// Execute
let mut interp = Interpreter::new();
interp.execute(&ast)?;

// Access results
let result = interp.env.get("result");
if let Some(Value::Int(n)) = result {
    println!("Result: {}", n);  // 30
}
```

### Scene Graph Access

```rust
use ofml_interpreter::{Parser, Interpreter};

let source = r#"
    class MyPart : OiPart {
        func initialize() {
            OiBlock(self, @cube, [1, 1, 1]);
        }
    }
    var part = MyPart();
"#;

let mut parser = Parser::new(source)?;
let ast = parser.parse()?;

let mut interp = Interpreter::new();
interp.execute(&ast)?;

// Access generated scene
let scene = interp.scene.to_scene();  // Scene3DS
println!("Meshes: {}", scene.meshes.len());

// Export to GLB
let glb = scene_to_glb(&scene)?;
```

### ALB Integration

```rust
use ofml_interpreter::Interpreter;
use std::path::PathBuf;

let mut interp = Interpreter::new();

// Set ALB path for geometry loading
interp.set_alb_path(PathBuf::from("/path/to/data.alb"));

// Now OiImport can load from ALB
```

---

## 2D Geometry Module

2D floor plan generation.

### G2DCompound

Container for 2D graphics.

```rust
use ofml_interpreter::geometry2d::{G2DCompound, G2DPrimitive, G2DAttributes};
use ofml_interpreter::ebase::Odb2dRecord;

// Process odb2d records
let records: Vec<Odb2dRecord> = /* load from EBASE */;
let compound = process_odb2d_records(&records);

// Generate SVG
let svg_content = compound.to_svg();
std::fs::write("output.svg", svg_content)?;
```

### Manual 2D Construction

```rust
use ofml_interpreter::geometry2d::{
    G2DCompound, G2DPrimitive, G2DAttributes, Transform2D,
    Point2D, Line2D, Rect2D,
};

let mut compound = G2DCompound::new();

// Add line
compound.add_primitive(G2DPrimitive::Lines {
    points: vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 100.0, y: 100.0 },
    ],
});

// Add rectangle
compound.add_primitive(G2DPrimitive::Rectangle {
    rect: Rect2D {
        x: 50.0, y: 50.0,
        width: 100.0, height: 80.0,
    },
    filled: true,
});

// Add with attributes
let attrs = G2DAttributes {
    stroke_color: Some("#FF0000".to_string()),
    stroke_width: Some(2.0),
    fill_color: Some("#00FF00".to_string()),
    layer: "furniture".to_string(),
};
compound.add_with_attributes(
    G2DPrimitive::Circle { center: Point2D { x: 100.0, y: 100.0 }, radius: 25.0 },
    attrs,
);
```

---

## Material System

Material and texture handling.

### MaterialDef

```rust
use ofml_interpreter::material::{MaterialDef, TextureDef, TextureProjection};

let material = MaterialDef {
    name: "wood_oak".to_string(),
    ambient: [0.2, 0.15, 0.1],
    diffuse: [0.6, 0.45, 0.3],
    specular: [0.3, 0.25, 0.2],
    shininess: 32.0,
    transparency: 0.0,
    texture: Some(TextureDef {
        filename: "oak.png".to_string(),
        projection: TextureProjection::Uv,
        scale: [1.0, 1.0],
        offset: [0.0, 0.0],
    }),
};
```

### TextureCache

```rust
use ofml_interpreter::texture::{TextureCache, TextureData};

let mut cache = TextureCache::new();

// Load texture (caches automatically)
let texture: Option<&TextureData> = cache.load("textures/wood.png")?;

if let Some(tex) = texture {
    println!("Size: {}x{}", tex.width, tex.height);
    println!("Channels: {}", tex.channels);
    // tex.data contains raw pixel bytes
}
```

---

## Error Handling

### Operation Errors

```rust
use ofml_interpreter::operations::OperationError;

fn process_file(path: &str) -> Result<(), OperationError> {
    let scene = load_geometry_file(Path::new(path))?;
    // ...
    Ok(())
}

// Handle errors
match process_file("model.xyz") {
    Ok(_) => println!("Success"),
    Err(OperationError::Io(e)) => eprintln!("IO error: {}", e),
    Err(OperationError::Parse(msg)) => eprintln!("Parse error: {}", msg),
    Err(OperationError::NotFound(msg)) => eprintln!("Not found: {}", msg),
    Err(OperationError::InvalidFormat(msg)) => eprintln!("Invalid format: {}", msg),
    Err(OperationError::NoGeometry) => eprintln!("No geometry found"),
}
```

### Specific Error Types

```rust
use ofml_interpreter::errors::{
    ArticleError,
    AttachmentError,
    EbaseExprError,
    GObjectError,
    Geometry2DError,
    MaterialError,
    OfmlClassError,
    PropertyError,
};

// Each module has its own error type
// They all implement std::error::Error and Display
```

---

## Complete Example

```rust
use ofml_interpreter::{
    Parser, Interpreter,
    operations::{load_geometry_file, export_to_glb, assemble_product, ProductConfig},
    ebase::EBaseReader,
    geometry::scene_to_glb,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Convert geometry file
    let scene = load_geometry_file(Path::new("chair.3ds"))?;
    let glb = export_to_glb(&scene)?;
    std::fs::write("chair.glb", glb)?;
    println!("Converted chair.3ds -> chair.glb");

    // Example 2: Assemble product from OFML data
    let config = ProductConfig {
        article: Some("AC_CHAIR".to_string()),
        ..Default::default()
    };
    let result = assemble_product(Path::new("/ofmldata/vitra/ac/1"), &config)?;
    let glb = export_to_glb(&result.scene)?;
    std::fs::write("ac_chair.glb", glb)?;
    println!("Assembled AC_CHAIR -> ac_chair.glb");

    // Example 3: Execute CLS script
    let source = r#"
        class TestPart : OiPart {
            func initialize() {
                OiBlock(self, @base, [0.5, 0.05, 0.5]);
            }
        }
        var part = TestPart();
    "#;

    let mut parser = Parser::new(source)?;
    let ast = parser.parse()?;
    let mut interp = Interpreter::new();
    interp.execute(&ast)?;

    let scene = interp.scene.to_scene();
    let glb = scene_to_glb(&scene)?;
    std::fs::write("test_part.glb", glb)?;
    println!("Executed CLS -> test_part.glb");

    // Example 4: Read EBASE data
    let mut reader = EBaseReader::open("/ofmldata/vitra/ac/1/odb.ebase")?;
    println!("Tables: {:?}", reader.tables.keys().collect::<Vec<_>>());

    Ok(())
}
```

---

## Thread Safety

- `Scene3DS`, `Mesh`, `Vertex`, `Face` are `Clone + Send + Sync`
- `Interpreter` is NOT thread-safe (uses `Rc<RefCell<>>` internally)
- `EBaseReader` is NOT thread-safe (holds file handle)
- For parallel processing, create separate instances per thread

---

## Performance Tips

1. **Reuse EBaseReader** - Opening is expensive, reading is cheap
2. **Batch geometry operations** - Merge scenes before export
3. **Use release builds** - Debug builds are 10-50x slower
4. **Stream large files** - Use `load_geometry_data` with memory-mapped files for large datasets

---

## Version Compatibility

| Rust Version | Status |
|--------------|--------|
| 1.70+ | Supported |
| 1.60-1.69 | May work |
| < 1.60 | Not supported |
