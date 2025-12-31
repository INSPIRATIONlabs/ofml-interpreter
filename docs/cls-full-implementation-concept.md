# OFML CLS Full Implementation Concept

This document provides a comprehensive concept for implementing full CLS (Configuration Language Script) support in the Rust OFML interpreter, based on analysis of ConceptOffice7/pCon.basket source code.

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Current State Analysis](#current-state-analysis)
3. [ConceptOffice7 Architecture Analysis](#conceptoffice7-architecture-analysis)
4. [OFML Data Flow](#ofml-data-flow)
5. [Implementation Architecture](#implementation-architecture)
6. [Component Specifications](#component-specifications)
7. [File Format Handling](#file-format-handling)
8. [Implementation Roadmap](#implementation-roadmap)

---

## Executive Summary

The goal is to achieve feature parity with pCon.basket's OFML interpreter for rendering furniture products. This requires:

1. **Complete CLS interpreter** - Full OFML 2.0 language support with all standard library classes
2. **OFML Framework Classes** - Implementation of `::ofml::go::*` and `::ofml::xoi::*` class hierarchies
3. **EBASE Expression Evaluator** - PostScript-like stack-based expression evaluation for odb3d constructor references
4. **Multi-format Geometry Loading** - Unified handling of 3DS, GEO, OBJ with proper coordinate transforms
5. **Material System** - Loading and applying materials from MAT files and EBASE tables
6. **Article Configuration** - Property-driven product variants using article properties (M__BREITE, SH__BASIC, etc.)

---

## Current State Analysis

### What We Have (Rust Implementation)

| Component | Status | Location |
|-----------|--------|----------|
| Lexer | Complete | `src/lexer.rs` |
| Parser | Complete | `src/parser.rs` |
| AST | Complete | `src/ast.rs` |
| Basic Interpreter | Partial | `src/interpreter.rs` |
| Scene Graph | Complete | `src/scene.rs` |
| EBASE Reader | Complete | `src/ebase.rs` |
| 3DS Parser | Complete | `src/geometry.rs` |
| GEO Parser | Complete | `src/geometry.rs` |
| OBJ Parser | Complete | `src/geometry.rs` |
| ALB Archive | Complete | `src/ofml.rs` |
| GLB Export | Complete | `src/geometry.rs` |

### What's Missing

1. **OFML Framework Classes**
   - `::ofml::go::*` - Geometry operations (GoXLRTransYLRTrans, GoYLTrans, etc.)
   - `::ofml::xoi::*` - Extended object interface classes
   - `::egr::aci::*` - Application configuration interface

2. **EBASE Expression Evaluation**
   - Stack-based PostScript expression interpreter for `ctor` fields
   - Operations: `imp`, `egms`, `clsref`, arithmetic, conditionals

3. **Article-Driven Configuration**
   - Property resolution from article tables
   - Variant selection based on article properties
   - Conditional geometry loading

4. **Material Application**
   - MAT file parsing
   - Material table lookup
   - Texture coordinate handling

---

## ConceptOffice7 Architecture Analysis

### GObject Hierarchy (from pcon-basket/ebasket.dll)

The OFML interpreter uses a GObject-based object system:

```
GObject (base class)
├── GStringObj     - String values
├── GIntegerObj    - Integer values
├── GSymbolObj     - Symbol/identifier values
├── GSequenceObj   - Arrays/lists
├── GDictObj       - Dictionaries/maps
├── GProc          - Procedure/function
└── GOperator      - Built-in operators
```

Key functions from decompiled code:
- `GObject::parseCb()` - Parse callback for expression evaluation
- `GObject::addRef()` / `GObject::release()` - Reference counting
- `GStringObj::getValue()` - Get string value
- `GSequenceObj::getItem()` - Array access

### OFML Article Data Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                      ARTICLE CONFIGURATION                           │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Article Properties (from catalog/OAM)                           │
│     M__BREITE = 1600                                                │
│     M__TIEFE = 800                                                  │
│     SH__BASIC = "::egr::aci::ACI5"                                  │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. ODB Lookup (odb.ebase)                                          │
│     - Find article in `articles` table                              │
│     - Get odb_name reference                                        │
│     - Load odb3d records for that product                          │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Constructor Evaluation                                          │
│     odb3d.ctor = "${M__BREITE:-100} 1000 / FBREITE ==" evaluates   │
│     with M__BREITE=1600 → FBREITE = 1.6                            │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Geometry Loading                                                │
│     Mode A: Direct import                                           │
│       ctor = "\"filename\" 1 1 1 imp"                              │
│       → Load filename.3ds/geo/obj from ALB                         │
│                                                                     │
│     Mode B: CLS class instantiation                                 │
│       ctor = "width depth height \"ClassName\" clsref"             │
│       → Instantiate CLS class with parameters                      │
│                                                                     │
│     Mode C: EGMS (EasternGraphics Modeling System)                 │
│       ctor = "\"objectname\" egms"                                 │
│       → Load from EGMS geometry system                             │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Material Application                                            │
│     odb3d.mat = "${SH__BASIC:-\"::egr::aci::ACI5\"}"               │
│     → Resolve material from MAT table or class reference           │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  6. Transform Application                                           │
│     x_offs, y_offs, z_offs → Position                              │
│     x_rot, y_rot, z_rot → Rotation                                 │
│     Scale from ctor parameters                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## OFML Data Flow

### Directory Structure

```
ofmldata/
└── <manufacturer>/
    └── <series>/
        └── <version>/
            ├── odb.ebase           # Object database (articles, odb3d, funcs)
            ├── <series>.alb        # Geometry archive (3DS, GEO, OBJ)
            └── db/
                ├── <series>.cls    # Class definitions
                └── <series>.mat    # Material definitions
```

### EBASE Tables

| Table | Purpose |
|-------|---------|
| `articles` | Article catalog with properties |
| `odb3d` | 3D object definitions (geometry, position, material) |
| `odb2d` | 2D symbol definitions |
| `funcs` | Named function definitions |
| `layer` | Layer configuration |
| `attpt` | Attachment points |
| `oppattpt` | Opposite attachment points |
| `stdattpt` | Standard attachment points |

### Key odb3d Fields

| Field | Description |
|-------|-------------|
| `odb_name` | Product/article reference |
| `obj_name` | Object path (e.g., "o1.o2.geo") |
| `visible` | Visibility expression |
| `x_offs`, `y_offs`, `z_offs` | Position offset expressions |
| `x_rot`, `y_rot`, `z_rot` | Rotation expressions |
| `ctor` | Constructor expression (geometry loading) |
| `mat` | Material expression |
| `attrib` | Attribute expressions |
| `link` | Link to related objects |

---

## Implementation Architecture

### Module Structure

```
src/
├── lib.rs                    # Library exports
├── main.rs                   # CLI interface
├── lexer.rs                  # OFML tokenizer
├── parser.rs                 # OFML parser
├── ast.rs                    # Abstract syntax tree
├── interpreter.rs            # Core interpreter
├── env.rs                    # Variable environment
├── value.rs                  # Runtime values
├── scene.rs                  # Scene graph
├── geometry.rs               # Geometry parsing (3DS, GEO, OBJ)
├── ebase.rs                  # EBASE database reader
├── ofml.rs                   # ALB archive handling
│
├── ofml_framework/           # NEW: OFML standard library
│   ├── mod.rs
│   ├── go.rs                 # ::ofml::go::* classes
│   ├── xoi.rs                # ::ofml::xoi::* classes
│   └── egr.rs                # ::egr::* classes
│
├── ebase_eval/               # NEW: EBASE expression evaluator
│   ├── mod.rs
│   ├── lexer.rs              # PostScript tokenizer
│   ├── stack.rs              # Stack machine
│   └── ops.rs                # Built-in operators
│
├── material/                 # NEW: Material system
│   ├── mod.rs
│   ├── mat_parser.rs         # MAT file parser
│   └── resolver.rs           # Material lookup
│
└── article/                  # NEW: Article configuration
    ├── mod.rs
    ├── properties.rs         # Property resolution
    └── variants.rs           # Variant selection
```

---

## Component Specifications

### 1. EBASE Expression Evaluator

The `ctor` fields in odb3d use a PostScript-like stack language:

```postscript
# Direct import: load geometry file with scale
"filename" 1 1 1 imp

# Class reference: instantiate CLS class
1600 800 740 "::ofml::go::GoYLTrans" clsref

# EGMS reference: load from EGMS
"objectname" egms

# Variable substitution and arithmetic
${M__BREITE:-100} 1000 / FBREITE ==    # FBREITE = M__BREITE / 1000

# Conditional
FBREITE 0.36 == { ... } if
```

**Implementation:**

```rust
// src/ebase_eval/mod.rs

pub struct EbaseEvaluator {
    stack: Vec<EbaseValue>,
    variables: HashMap<String, EbaseValue>,
}

pub enum EbaseValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<EbaseValue>),
    Proc(Vec<Token>),  // Deferred execution block
}

pub enum EbaseResult {
    // Results from evaluation
    Import { filename: String, scale: [f32; 3] },
    ClsRef { class: String, params: Vec<f64> },
    Egms { name: String },
    Value(EbaseValue),
}

impl EbaseEvaluator {
    /// Evaluate a ctor expression
    pub fn evaluate(&mut self, expr: &str, props: &Properties) -> Result<EbaseResult, Error> {
        // 1. Substitute ${VAR:-default} patterns
        let expanded = self.expand_variables(expr, props)?;

        // 2. Tokenize
        let tokens = self.tokenize(&expanded)?;

        // 3. Execute stack machine
        for token in tokens {
            match token {
                Token::Number(n) => self.stack.push(EbaseValue::Float(n)),
                Token::String(s) => self.stack.push(EbaseValue::String(s)),
                Token::Ident(name) => self.lookup_or_exec(&name)?,
                Token::Operator(op) => self.exec_operator(op)?,
            }
        }

        // 4. Return final result
        self.get_result()
    }

    fn exec_operator(&mut self, op: &str) -> Result<(), Error> {
        match op {
            // Import geometry: "filename" sx sy sz imp
            "imp" => {
                let sz = self.pop_float()?;
                let sy = self.pop_float()?;
                let sx = self.pop_float()?;
                let filename = self.pop_string()?;
                self.result = Some(EbaseResult::Import {
                    filename,
                    scale: [sx as f32, sy as f32, sz as f32],
                });
            }

            // Class reference: p1 p2 p3 "ClassName" clsref
            "clsref" => {
                let class = self.pop_string()?;
                let param_count = self.pop_int()?;
                let mut params = Vec::new();
                for _ in 0..param_count {
                    params.push(self.pop_float()?);
                }
                params.reverse();
                self.result = Some(EbaseResult::ClsRef { class, params });
            }

            // EGMS reference: "name" egms
            "egms" => {
                let name = self.pop_string()?;
                self.result = Some(EbaseResult::Egms { name });
            }

            // Arithmetic
            "+" => binary_op!(self, |a, b| a + b),
            "-" => binary_op!(self, |a, b| a - b),
            "*" => binary_op!(self, |a, b| a * b),
            "/" => binary_op!(self, |a, b| a / b),
            "neg" => { let v = self.pop_float()?; self.push(EbaseValue::Float(-v)); }

            // Comparison
            "==" => { let b = self.pop()?; let a = self.pop()?; self.push(EbaseValue::Bool(a == b)); }
            "!=" => { let b = self.pop()?; let a = self.pop()?; self.push(EbaseValue::Bool(a != b)); }

            // Assignment (define variable)
            "==" if is_assignment => {
                let name = self.pop_string()?;
                let value = self.pop()?;
                self.variables.insert(name, value);
            }

            _ => return Err(Error::UnknownOperator(op.to_string())),
        }
        Ok(())
    }
}
```

### 2. OFML Framework Classes

The `::ofml::go::*` namespace contains geometry operation classes:

```rust
// src/ofml_framework/go.rs

/// GoMetaType - Base class for configurable products
pub struct GoMetaType {
    pub width: f64,
    pub depth: f64,
    pub height: f64,
}

/// GoXLRTransYLRTrans - Stretches geometry in X and Y based on parameters
/// Used for table tops, shelves, panels
pub struct GoXLRTransYLRTrans {
    pub x_min: f64,
    pub x_max: f64,
    pub x_scale: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub y_scale: f64,
    pub num_params: i32,
    pub base_class: String,
}

impl GoXLRTransYLRTrans {
    /// Create from clsref parameters
    /// clsref format: x_min x_max x_scale y_min y_max y_scale param_count "class" clsref
    pub fn from_params(params: &[f64], class: &str) -> Self {
        Self {
            x_min: params[0],
            x_max: params[1],
            x_scale: params[2],
            y_min: params[3],
            y_max: params[4],
            y_scale: params[5],
            num_params: params.get(6).copied().unwrap_or(6.0) as i32,
            base_class: class.to_string(),
        }
    }

    /// Apply transformation to geometry
    pub fn transform(&self, geometry: &mut Scene3DS) {
        for mesh in &mut geometry.meshes {
            for vertex in &mut mesh.vertices {
                // Scale X vertices based on position
                if vertex.x < self.x_min as f32 {
                    // Keep left edge fixed
                } else if vertex.x > self.x_max as f32 {
                    // Translate right edge
                    vertex.x += (self.x_scale - 1.0) as f32 * (self.x_max as f32 - self.x_min as f32);
                } else {
                    // Interpolate middle region
                    let t = (vertex.x - self.x_min as f32) / (self.x_max - self.x_min) as f32;
                    vertex.x = self.x_min as f32 + t * self.x_scale as f32 * (self.x_max - self.x_min) as f32;
                }

                // Same for Y
                if vertex.y < self.y_min as f32 {
                    // Keep bottom fixed
                } else if vertex.y > self.y_max as f32 {
                    vertex.y += (self.y_scale - 1.0) as f32 * (self.y_max as f32 - self.y_min as f32);
                } else {
                    let t = (vertex.y - self.y_min as f32) / (self.y_max - self.y_min) as f32;
                    vertex.y = self.y_min as f32 + t * self.y_scale as f32 * (self.y_max - self.y_min) as f32;
                }
            }
        }
    }
}

/// GoYLTrans - Stretches geometry in Y (height) dimension
/// Common for legs, supports, vertical elements
pub struct GoYLTrans {
    pub base_height: f64,
    pub target_height: f64,
    pub stretch_min: f64,
    pub num_params: i32,
    pub base_class: String,
}

impl GoYLTrans {
    pub fn from_params(params: &[f64], class: &str) -> Self {
        Self {
            base_height: params[0],
            target_height: params[1],
            stretch_min: params[2],
            num_params: params.get(3).copied().unwrap_or(3.0) as i32,
            base_class: class.to_string(),
        }
    }

    pub fn transform(&self, geometry: &mut Scene3DS) {
        let scale = self.target_height / self.base_height;
        for mesh in &mut geometry.meshes {
            for vertex in &mut mesh.vertices {
                if vertex.y > self.stretch_min as f32 {
                    vertex.y = self.stretch_min as f32 +
                        (vertex.y - self.stretch_min as f32) * scale as f32;
                }
            }
        }
    }
}
```

### 3. Material System

```rust
// src/material/mod.rs

pub struct MaterialResolver {
    mat_cache: HashMap<String, Material>,
    ebase_materials: HashMap<String, EbaseMaterial>,
}

pub struct Material {
    pub name: String,
    pub diffuse_color: [f32; 4],
    pub ambient_color: [f32; 4],
    pub specular_color: [f32; 4],
    pub shininess: f32,
    pub texture_file: Option<String>,
}

impl MaterialResolver {
    /// Resolve a material expression from odb3d.mat
    pub fn resolve(&self, expr: &str, props: &Properties) -> Result<Material, Error> {
        // Expand variables: ${SH__BASIC:-"::egr::aci::ACI5"}
        let expanded = expand_variables(expr, props)?;

        // Check for class reference: ::egr::aci::ACI5
        if expanded.starts_with("::") {
            return self.resolve_class_material(&expanded);
        }

        // Check EBASE mat table
        if let Some(mat) = self.ebase_materials.get(&expanded) {
            return Ok(mat.to_material());
        }

        // Check MAT file cache
        if let Some(mat) = self.mat_cache.get(&expanded) {
            return Ok(mat.clone());
        }

        Err(Error::MaterialNotFound(expanded))
    }

    /// Load materials from MAT file in ALB
    pub fn load_from_alb(&mut self, archive: &mut AlbArchive, mat_file: &str) -> Result<(), Error> {
        let data = archive.extract(mat_file)?;
        let materials = parse_mat_file(&data)?;
        for mat in materials {
            self.mat_cache.insert(mat.name.clone(), mat);
        }
        Ok(())
    }
}
```

### 4. Article Configuration

```rust
// src/article/mod.rs

pub struct ArticleConfig {
    /// Article number (e.g., "1600x800")
    pub article_nr: String,
    /// Resolved properties
    pub properties: Properties,
    /// ODB name for geometry lookup
    pub odb_name: String,
}

pub type Properties = HashMap<String, PropertyValue>;

pub enum PropertyValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl ArticleConfig {
    /// Load article configuration from EBASE
    pub fn load(ebase: &mut EBaseReader, article_nr: &str) -> Result<Self, Error> {
        // 1. Find article in articles table
        let articles = ebase.read_records("articles", Some(&format!("article_nr = '{}'", article_nr)))?;
        let article = articles.first().ok_or(Error::ArticleNotFound)?;

        // 2. Extract properties from article record
        let mut properties = Properties::new();
        for (key, value) in article {
            if key.starts_with("M__") || key.starts_with("SH__") || key.starts_with("CO__") {
                properties.insert(key.clone(), value.into());
            }
        }

        // 3. Get ODB name
        let odb_name = article.get("odb_name")
            .and_then(|v| v.as_str())
            .unwrap_or(article_nr)
            .to_string();

        Ok(ArticleConfig {
            article_nr: article_nr.to_string(),
            properties,
            odb_name,
        })
    }

    /// Get property value with default
    pub fn get(&self, name: &str, default: PropertyValue) -> PropertyValue {
        self.properties.get(name).cloned().unwrap_or(default)
    }
}
```

---

## File Format Handling

### Geometry Format Detection

```rust
// src/geometry.rs

pub enum GeometryFormat {
    Auto,      // Detect from file content/extension
    ThreeDS,   // .3ds - 3D Studio format
    Geo,       // .geo - EasternGraphics geometry
    Obj,       // .obj - Wavefront OBJ
}

pub fn load_geometry(data: &[u8], format: GeometryFormat) -> Result<Scene3DS, Error> {
    let format = if format == GeometryFormat::Auto {
        detect_format(data)?
    } else {
        format
    };

    match format {
        GeometryFormat::ThreeDS => parse_3ds(data),
        GeometryFormat::Geo => parse_geo(data),
        GeometryFormat::Obj => parse_obj(data),
        GeometryFormat::Auto => unreachable!(),
    }
}

fn detect_format(data: &[u8]) -> Result<GeometryFormat, Error> {
    // Check magic bytes
    if data.len() >= 2 {
        let magic = u16::from_le_bytes([data[0], data[1]]);
        if magic == 0x4D4D {  // 3DS magic
            return Ok(GeometryFormat::ThreeDS);
        }
    }

    // Check for OBJ text content
    if data.starts_with(b"#") || data.starts_with(b"v ") || data.starts_with(b"o ") {
        return Ok(GeometryFormat::Obj);
    }

    // Check GEO header
    if data.len() >= 4 && &data[0..4] == b"GEOF" {
        return Ok(GeometryFormat::Geo);
    }

    // Default to 3DS (most common in OFML)
    Ok(GeometryFormat::ThreeDS)
}
```

### Coordinate System Handling

Different geometry formats use different coordinate systems:

| Format | Up Axis | Right-Handed |
|--------|---------|--------------|
| 3DS | Z-up | Yes |
| GEO | Y-up | Yes |
| OBJ | Y-up | Yes |
| GLB | Y-up | Yes |

```rust
pub enum CoordSystem {
    ZUp,   // 3DS: X-right, Y-forward, Z-up
    YUp,   // GEO/OBJ/GLB: X-right, Y-up, Z-forward
}

pub fn convert_coord_system(scene: &mut Scene3DS, from: CoordSystem, to: CoordSystem) {
    if from == to {
        return;
    }

    match (from, to) {
        (CoordSystem::ZUp, CoordSystem::YUp) => {
            // Rotate -90 degrees around X axis: (x, y, z) -> (x, -z, y)
            for mesh in &mut scene.meshes {
                for v in &mut mesh.vertices {
                    let old_y = v.y;
                    let old_z = v.z;
                    v.y = old_z;
                    v.z = -old_y;
                }
                // Also rotate normals
                for n in &mut mesh.normals {
                    let old_y = n[1];
                    let old_z = n[2];
                    n[1] = old_z;
                    n[2] = -old_y;
                }
            }
        }
        (CoordSystem::YUp, CoordSystem::ZUp) => {
            // Rotate 90 degrees around X axis: (x, y, z) -> (x, z, -y)
            for mesh in &mut scene.meshes {
                for v in &mut mesh.vertices {
                    let old_y = v.y;
                    let old_z = v.z;
                    v.y = -old_z;
                    v.z = old_y;
                }
            }
        }
    }
}
```

---

## Implementation Roadmap

### Phase 1: EBASE Expression Evaluator (Priority: Critical)

**Goal:** Evaluate all odb3d.ctor expressions to determine geometry loading mode.

| Task | Effort | Dependencies |
|------|--------|--------------|
| PostScript tokenizer | 2 days | None |
| Stack machine core | 2 days | Tokenizer |
| Variable substitution (`${VAR:-default}`) | 1 day | Stack machine |
| `imp` operator | 1 day | Stack machine |
| `clsref` operator | 1 day | Stack machine |
| `egms` operator | 1 day | Stack machine |
| Arithmetic operators | 1 day | Stack machine |
| Comparison operators | 1 day | Stack machine |
| Conditional execution | 2 days | Stack machine |

**Estimated total: 12 days**

### Phase 2: OFML Framework Classes (Priority: High)

**Goal:** Implement core geometry transformation classes.

| Task | Effort | Dependencies |
|------|--------|--------------|
| Class registry system | 2 days | None |
| GoMetaType base | 1 day | Registry |
| GoXLRTransYLRTrans | 3 days | Registry, Geometry |
| GoYLTrans | 2 days | Registry, Geometry |
| GoXLTrans | 1 day | GoYLTrans |
| GoZLTrans | 1 day | GoYLTrans |
| GoMirror | 2 days | Registry, Geometry |
| ::egr::aci::* materials | 2 days | Material system |

**Estimated total: 14 days**

### Phase 3: Material System (Priority: Medium)

**Goal:** Load and apply materials from MAT files and EBASE.

| Task | Effort | Dependencies |
|------|--------|--------------|
| MAT file parser | 2 days | None |
| EBASE mat table reader | 1 day | EBASE reader |
| Material resolver | 2 days | MAT parser, EBASE |
| Class material lookup | 1 day | Resolver |
| GLB material export | 2 days | GLB export |
| Texture support (PNG/JPG) | 3 days | Material system |

**Estimated total: 11 days**

### Phase 4: Article Configuration (Priority: Medium)

**Goal:** Configure products based on article properties.

| Task | Effort | Dependencies |
|------|--------|--------------|
| Property system | 2 days | None |
| Article loader from EBASE | 2 days | EBASE reader |
| Property-driven geometry | 2 days | EBASE eval, Loader |
| Variant selection | 3 days | Property system |
| Complete product assembly | 3 days | All above |

**Estimated total: 12 days**

### Phase 5: Integration & Testing (Priority: High)

**Goal:** End-to-end product rendering with all features.

| Task | Effort | Dependencies |
|------|--------|--------------|
| Integration testing framework | 2 days | All phases |
| Test with SBU products | 3 days | All phases |
| Test with KN products | 2 days | All phases |
| Test with GSX (SEDUS) products | 2 days | All phases |
| Test with Vitra products | 2 days | All phases |
| Performance optimization | 3 days | Testing |
| Documentation | 3 days | All above |

**Estimated total: 17 days**

---

## Total Estimated Effort

| Phase | Days |
|-------|------|
| Phase 1: EBASE Expression Evaluator | 12 |
| Phase 2: OFML Framework Classes | 14 |
| Phase 3: Material System | 11 |
| Phase 4: Article Configuration | 12 |
| Phase 5: Integration & Testing | 17 |
| **Total** | **66 days** |

---

## Success Criteria

1. **Parse all manufacturer CLS files** without errors
2. **Evaluate all odb3d.ctor expressions** returning correct geometry references
3. **Load geometry from ALB** in all formats (3DS, GEO, OBJ)
4. **Apply transformations** from GoXLRTransYLRTrans, GoYLTrans, etc.
5. **Export valid GLB files** viewable in standard 3D viewers
6. **Match pCon.basket output** for test products within 1mm tolerance

---

---

## Additional Findings from pCon.planner Analysis

Analysis of `/reference/ConceptOffice7/sources/pcon-planner/` revealed additional implementation details.

### GObject Parser (from eplanner.dll)

The core OFML expression parser is `GObject::parseCb()`:

```cpp
// Parse an OFML expression string, returns parsed GObject
GObject* GObject::parseCb(const char* expression, char** endptr);

// Reference counting
void GObject::addRef();
void GObject::release();
```

**Usage pattern:**
```cpp
GObject* result = GObject::parseCb("[1.0, 2.0, 3.0]", &endptr);
if (result != NULL) {
    GObject::addRef(result);
    // ... use result ...
    GObject::release(result);
}
```

### GObject Type System with RTTI

```cpp
// Type descriptors for dynamic_cast operations
GObject::RTTI_Type_Descriptor       // Base class
GSequenceObj::RTTI_Type_Descriptor  // Arrays [1, 2, 3]
GSymbolObj::RTTI_Type_Descriptor    // Symbols @name
GStringObj::RTTI_Type_Descriptor    // Strings "text"
GIntegerObj::RTTI_Type_Descriptor   // Integers 42
GFloatObj::RTTI_Type_Descriptor     // Floats 3.14

// Value extraction
byte* GSymbolObj::getValue();       // Returns symbol name
byte* GStringObj::getValue();       // Returns string content
int   GIntegerObj::getValue();      // Returns integer value
```

### Geometry Library (egeo.dll)

The `egr::geo::base::Mesh` class provides the geometry API:

```cpp
namespace egr::geo::base {
    class Mesh {
    public:
        struct Vertex { float x, y, z; };

        // Accessors
        vector<Vertex>* getVertices();
        vector<Vec3>* getNormals();
        vector<Vec2>* getUVs();
        vector<Face>* getFaces();

        // Modifiers
        void addVertex(const Vec3<double>& v);
        void addNormal(const Vec3<double>& n);
        void addUV(const Vec2<double>& uv);
        void addFace(const Face& f);

        // Queries
        bool hasNormals();
        bool hasUVs();
    };

    class Material {
        // Material properties with textures
        enum TexIdx { DIFFUSE, NORMAL, ROUGHNESS, METALLIC };
    };

    class Texture {
        // Texture data
    };
}
```

### File Format Support (egeo.dll)

```cpp
namespace egr::geo::io {
    // GLTF/GLB export
    class GltfFileImpl {
        int getMaterial(shared_ptr<Material>);
        void writeTexTrans(const Mat4&);  // KHR_texture_transform
    };

    // USD format support
    class UsdFileImpl {
        shared_ptr<Texture> readUvTexture(const UsdShadeShader&);
        shared_ptr<Material> getMaterial(const UsdPrim&);
        bool tryGetTexture(const UsdShadeInput&, shared_ptr<Material>, TexIdx);
    };

    // FBX format support
    class FbxFileImpl {
        FbxSurfaceMaterial* getMaterial(shared_ptr<Material>);
        FbxFileTexture* getTexture(...);
    };
}
```

### MetaType Object Creation (xbasket.dll)

The `mt_createObject` function creates OFML MetaType objects:

```cpp
void mt_createObject(FModule* module, ...);

// OFML expressions executed during object creation:
"::ofml::go::GoMTAppEnvironment()"  // Initialize app environment
"::ofml::go::GoMetaType"            // Create MetaType
"c.setPosition([0.0,0.0,0.0])"      // Position the object
"::ofml::go::GoMetaType::getGOContext(\"...\")"  // Get GO context
```

### Property System (eproduct.dll)

```cpp
namespace egr::eai::eproduct {
    class PropEdit {
        class PropChangeCallout;
        class UpdateCallout;
        class SystemCallbackAdapter;
        class PropValueChangedEvent;
    };

    class Property {
        // Base property class
    };

    class ChoiceProperty : public Property {
        // Property with choices
    };

    class BoolProperty : public Property;
    class IntProperty : public Property;
    class FloatProperty : public Property;
    class StringProperty : public Property;
}
```

### OFML Namespaces Discovered

| Namespace | Purpose |
|-----------|---------|
| `::ofml::oi` | Object Interface (OiPart, OiBlock, etc.) |
| `::ofml::xoi` | Extended Object Interface |
| `::ofml::go` | Geometry Operations (GoMetaType, GoYLTrans, etc.) |
| `::ofml::app` | Application classes |
| `::egr::aci` | Application Configuration Interface |
| `::egr::geo::base` | Geometry primitives (Mesh, Material) |
| `::egr::geo::io` | File I/O (GLTF, USD, FBX) |
| `::egr::eai::eproduct` | Product/property handling |

### Source File Paths (from debug info)

```
D:\gef\eai\eproduct\pe_PropEdit.C       # Property editing
H:\p-pl-x_8.13\x3g\src\libx3gpl\...     # X3G Planner
C:\Users\sschmidt\src\pl-8.13-eai\...   # pCon.planner
```

---

## Updated Implementation Architecture

Based on pcon-planner analysis, the implementation should follow this pattern:

```
┌─────────────────────────────────────────────────────────────┐
│                    OFML Interpreter                          │
├─────────────────────────────────────────────────────────────┤
│  GObject System                                              │
│  ├── GStringObj    (strings)                                │
│  ├── GSymbolObj    (symbols)                                │
│  ├── GIntegerObj   (integers)                               │
│  ├── GSequenceObj  (arrays)                                 │
│  └── GDictObj      (dictionaries)                           │
├─────────────────────────────────────────────────────────────┤
│  parseCb() - Expression Parser                               │
│  └── PostScript-like stack evaluation                       │
├─────────────────────────────────────────────────────────────┤
│  OFML Framework Classes                                      │
│  ├── ::ofml::oi::* (OiPart, OiBlock, OiImport)             │
│  ├── ::ofml::go::* (GoMetaType, GoYLTrans, GoXLRTrans)     │
│  └── ::ofml::xoi::* (Extended Object Interface)            │
├─────────────────────────────────────────────────────────────┤
│  Geometry System (egr::geo::base)                           │
│  ├── Mesh          (vertices, normals, UVs, faces)         │
│  ├── Material      (diffuse, textures)                      │
│  └── Texture       (image data)                             │
├─────────────────────────────────────────────────────────────┤
│  File I/O (egr::geo::io)                                    │
│  ├── 3DS/GEO/OBJ   (input)                                 │
│  ├── ALB Archive   (encrypted ZIP)                          │
│  └── GLTF/GLB      (output)                                │
└─────────────────────────────────────────────────────────────┘
```

---

## References

1. ConceptOffice7/pcon-basket decompiled source (ebasket.dll)
2. ConceptOffice7/pcon-planner decompiled sources:
   - egeo.dll - Geometry library
   - eproduct.dll - Product/property handling
   - eplanner.dll - Planner with GObject parser
   - xbasket.dll - Basket module with MT creation
3. OFML 2.0 Specification (EasternGraphics)
4. pCon.DataClient OFML storage system documentation
5. GLB/glTF 2.0 Specification (Khronos Group)
6. 3DS file format documentation
