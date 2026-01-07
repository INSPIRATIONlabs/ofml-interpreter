# OFML Interpreter - Architecture Documentation

**Version**: 1.0.0
**Last Updated**: 2026-01-04

This document provides a comprehensive overview of the OFML Interpreter architecture, including module organization, data flow, and design decisions.

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Module Organization](#module-organization)
3. [Data Flow](#data-flow)
4. [Core Components](#core-components)
5. [OAP Configurator](#oap-configurator)
6. [TUI Architecture](#tui-architecture)
7. [Performance Architecture](#performance-architecture)
8. [Extension Points](#extension-points)

---

## System Overview

The OFML Interpreter is a multi-layered system for working with Office Furniture Modeling Language data:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              CLI / TUI                                   │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │
│  │   Commands    │  │  TUI Views    │  │    Config Store           │   │
│  │  (main.rs)    │  │  (tui/)       │  │  (favorites, history)     │   │
│  └───────┬───────┘  └───────┬───────┘  └───────────────────────────┘   │
└──────────┼──────────────────┼───────────────────────────────────────────┘
           │                  │
           ▼                  ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         Operations Layer                                 │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  operations.rs - High-level reusable functions                  │    │
│  │  - assemble_product()  - export_to_glb()  - validate_geometry() │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                       OAP Configurator                                   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │   engine    │ │    ocd      │ │  families   │ │    price    │       │
│  │ (config)    │ │  (reader)   │ │ (grouping)  │ │ (calculate) │       │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘       │
└─────────────────────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          Core Layer                                      │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │
│  │  lexer   │ │  parser  │ │ interp.  │ │  ebase   │ │ geometry │      │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘      │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │
│  │  scene   │ │ material │ │ texture  │ │  ofml    │ │   alb    │      │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘      │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Module Organization

### Directory Structure

```
src/
├── lib.rs                 # Library entry point, re-exports
├── main.rs                # CLI application
│
├── Core CLS Interpreter
│   ├── lexer.rs           # Tokenizer (logos-based)
│   ├── ast.rs             # Abstract Syntax Tree types
│   ├── parser.rs          # Recursive descent parser
│   ├── interpreter.rs     # Runtime execution engine
│   ├── env.rs             # Variable environment/scoping
│   ├── value.rs           # Runtime value types
│   └── errors.rs          # Error types
│
├── Geometry Pipeline
│   ├── geometry.rs        # 3D geometry (3DS, GEO, OBJ, GLB)
│   ├── geometry2d.rs      # 2D geometry (floor plans, SVG)
│   ├── scene.rs           # Scene graph for composition
│   ├── material.rs        # Material definitions
│   └── texture.rs         # Texture loading/caching
│
├── OFML Framework
│   ├── ofml.rs            # ALB archive handling
│   ├── alb_loader.rs      # ALB file extraction
│   ├── ofml_classes.rs    # Framework classes (Go*, Oi*)
│   ├── xoi_framework.rs   # Extended OI classes
│   ├── gobject.rs         # GObject type system
│   ├── property.rs        # Property system
│   ├── article.rs         # Article/variant handling
│   └── attachment.rs      # Attachment points
│
├── Data Layer
│   ├── ebase.rs           # EBase binary format reader
│   └── ebase_expr.rs      # PostScript expression evaluator
│
├── Operations
│   └── operations.rs      # High-level reusable functions
│
├── oap/                   # OAP Configurator
│   ├── mod.rs             # Module exports
│   ├── engine.rs          # Configuration engine
│   ├── ocd.rs             # OCD catalog data reader
│   ├── ocd_properties.rs  # Property definitions
│   ├── ocd_relation.rs    # Table relations
│   ├── families.rs        # Product family grouping
│   ├── manufacturers.rs   # Manufacturer loading
│   ├── catalog.rs         # Catalog browsing
│   ├── price.rs           # Price calculation
│   ├── property.rs        # Property management
│   ├── variant.rs         # Variant handling
│   ├── config.rs          # Configuration state
│   ├── oam.rs             # OAM article mappings
│   └── actions.rs         # User actions
│
└── tui/                   # Terminal UI
    ├── mod.rs             # Module exports
    ├── app.rs             # Application state machine
    ├── ui.rs              # Rendering logic
    ├── config_store.rs    # Persistence (configs, favorites)
    ├── views/             # Screen views
    │   ├── manufacturers.rs
    │   ├── families.rs
    │   ├── articles.rs
    │   ├── family_config.rs
    │   ├── properties.rs
    │   ├── catalog.rs
    │   ├── tables.rs
    │   └── help.rs
    └── widgets/           # Reusable UI components
        ├── list.rs
        └── form.rs
```

### Module Dependencies

```
                    ┌─────────────────┐
                    │     main.rs     │
                    │    (CLI/TUI)    │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
        ┌──────────┐  ┌──────────┐  ┌──────────┐
        │   tui/   │  │operations│  │   oap/   │
        └────┬─────┘  └────┬─────┘  └────┬─────┘
             │             │             │
             └─────────────┼─────────────┘
                           ▼
              ┌────────────────────────┐
              │      Core Modules      │
              │ ┌────────┐ ┌────────┐  │
              │ │geometry│ │ ebase  │  │
              │ └────────┘ └────────┘  │
              │ ┌────────┐ ┌────────┐  │
              │ │ scene  │ │material│  │
              │ └────────┘ └────────┘  │
              │ ┌────────────────────┐ │
              │ │ interpreter/parser │ │
              │ └────────────────────┘ │
              └────────────────────────┘
```

---

## Data Flow

### CLS Execution Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Source    │───>│    Lexer    │───>│   Parser    │───>│    AST      │
│   (.cls)    │    │  (tokens)   │    │ (recursive) │    │   (tree)    │
└─────────────┘    └─────────────┘    └─────────────┘    └──────┬──────┘
                                                                │
                                                                ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Output    │<───│  GLB Export │<───│ Scene Graph │<───│ Interpreter │
│   (.glb)    │    │             │    │             │    │  (execute)  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### OAP Configurator Pipeline

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         User Interaction Flow                            │
└─────────────────────────────────────────────────────────────────────────┘

    ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
    │ Select   │────>│ Select   │────>│ Select   │────>│ Configure│
    │ Mfr      │     │ Family   │     │ Article  │     │ Props    │
    └──────────┘     └──────────┘     └──────────┘     └────┬─────┘
                                                            │
                                                            ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           Data Loading Flow                              │
└─────────────────────────────────────────────────────────────────────────┘

    ┌──────────────┐
    │ ofmldata/    │
    │  ├─ mfr/     │
    │  │  └─ ser/  │
    │  │     └─ DE/│
    │  │        └─1│
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
    │ pdata.ebase  │────>│  OcdReader   │────>│ FamilyLoader │
    │              │     │  (parallel)  │     │  (parallel)  │
    └──────────────┘     └──────────────┘     └──────┬───────┘
                                                     │
           ┌─────────────────────────────────────────┤
           ▼                                         ▼
    ┌──────────────┐                          ┌──────────────┐
    │  Properties  │                          │   Articles   │
    │  (grouped)   │                          │  (families)  │
    └──────┬───────┘                          └──────┬───────┘
           │                                         │
           └──────────────────┬──────────────────────┘
                              ▼
                       ┌──────────────┐
                       │ ConfigEngine │
                       │  (pricing)   │
                       └──────────────┘
```

### EBase Expression Evaluation

```
┌─────────────────────────────────────────────────────────────────────────┐
│ EBASE Expression: '"chair.geo" 0.8 1.0 0.8 imp'                         │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Tokenization                                      │
│  ["chair.geo", 0.8, 1.0, 0.8, imp]                                      │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     Stack-Based Evaluation                               │
│  Step 1: Push "chair.geo"  → Stack: ["chair.geo"]                       │
│  Step 2: Push 0.8          → Stack: ["chair.geo", 0.8]                  │
│  Step 3: Push 1.0          → Stack: ["chair.geo", 0.8, 1.0]             │
│  Step 4: Push 0.8          → Stack: ["chair.geo", 0.8, 1.0, 0.8]        │
│  Step 5: Execute imp       → Import geometry with scale factors         │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          Result                                          │
│  EbaseResult::Import { file: "chair.geo", scale: [0.8, 1.0, 0.8] }      │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### Lexer (`lexer.rs`)

Uses the `logos` crate for efficient tokenization:

```rust
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("class")]    Class,
    #[token("func")]     Func,
    #[token("var")]      Var,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]  Ident(String),
    #[regex(r"-?[0-9]+(\.[0-9]+)?")]     Number(f64),
    // ... 50+ token types
}
```

**Key Features:**
- Zero-copy string handling
- Span tracking for error messages
- Comment and whitespace handling

### Parser (`parser.rs`)

Recursive descent parser generating an AST:

```rust
pub struct Parser<'a> {
    tokens: Vec<SpannedToken>,
    current: usize,
    source: &'a str,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }
}
```

**Supported Constructs:**
- Class definitions with inheritance
- Function definitions and calls
- Control flow (if/else, while, for)
- Expressions with operator precedence
- Property access and method calls

### Interpreter (`interpreter.rs`)

Tree-walking interpreter with scene graph integration:

```rust
pub struct Interpreter {
    pub env: Environment,
    pub scene: SceneGraph,
    pub ofml_registry: OfmlClassRegistry,
    pub texture_cache: TextureCache,
}

impl Interpreter {
    pub fn execute(&mut self, statements: &[Statement]) -> Result<()> {
        for stmt in statements {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }
}
```

**Features:**
- Lexical scoping with nested environments
- OFML framework class support (OiPart, OiBlock, etc.)
- Scene graph construction during execution
- Material and texture handling

### EBase Reader (`ebase.rs`)

Binary format reader for OFML databases:

```rust
pub struct EBaseReader {
    pub tables: HashMap<String, TableInfo>,
    file_path: PathBuf,
}

impl EBaseReader {
    pub fn read_records(
        &mut self,
        table_name: &str,
        columns: Option<&[&str]>,
    ) -> Result<Vec<Record>> {
        // Binary parsing with byteorder crate
    }
}
```

**Supported Tables:**
- `odb3d` - 3D geometry references
- `odb2d` - 2D floor plan geometry
- `mat` - Material definitions
- `ocd_article` - Article catalog
- `ocd_price` - Pricing information
- 30+ additional table types

### Scene Graph (`scene.rs`)

Hierarchical 3D scene representation:

```rust
pub struct SceneGraph {
    pub root: Rc<RefCell<SceneNode>>,
}

pub struct SceneNode {
    pub name: String,
    pub geometry: Option<Geometry>,
    pub transform: Transform3D,
    pub material: String,
    pub children: Vec<Rc<RefCell<SceneNode>>>,
}
```

**Capabilities:**
- Hierarchical transforms
- Material assignment per node
- Mesh merging for GLB export
- Coordinate system conversion (Y-up for glTF)

---

## OAP Configurator

### Engine (`oap/engine.rs`)

Central configuration management:

```rust
pub struct ConfigEngine {
    manufacturer: String,
    series: String,
    article_nr: String,
    property_values: HashMap<String, String>,
    ocd_data: Arc<OcdData>,
}

impl ConfigEngine {
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.property_values.insert(key.to_string(), value.to_string());
        self.recalculate_price();
    }

    pub fn calculate_price(&self) -> PriceResult {
        // Matches variant conditions against current property values
    }
}
```

### OCD Reader (`oap/ocd.rs`)

Parallel data loading from pdata.ebase:

```rust
pub struct OcdReader {
    pub articles: Vec<OcdArticle>,
    pub prices: Vec<OcdPrice>,
    pub properties: HashMap<String, Vec<OcdPropertyValue>>,
    pub property_classes: HashMap<String, OcdPropertyClass>,
}

impl OcdReader {
    pub fn from_ebase(path: &Path) -> Result<Self> {
        // Uses rayon for parallel table loading
    }
}
```

### Family Loader (`oap/families.rs`)

Groups articles into product families with parallel processing:

```rust
pub struct FamilyLoader {
    families: Vec<ProductFamily>,
}

impl FamilyLoader {
    pub fn load(manufacturer_path: &Path, region: &str) -> Self {
        // Parallel loading using rayon::par_iter()
        let series_dirs: Vec<_> = fs::read_dir(manufacturer_path)
            .into_par_iter()
            .map(|entry| load_series(entry))
            .collect();
    }
}
```

### Price Calculation (`oap/price.rs`)

Variant-condition-based pricing:

```rust
pub fn calculate_price(
    article_nr: &str,
    property_values: &HashMap<String, String>,
    ocd_data: &OcdData,
) -> PriceResult {
    // 1. Find base price (var_cond is empty or contains base indicator)
    // 2. Match surcharges against property values
    // 3. Sum base + matching surcharges
}
```

**Matching Strategies:**
1. Direct match: `var_cond == "S_166"` matches property value `"166"`
2. Suffix match: Property value ends with surcharge code
3. Numeric prefix match: Property value starts with surcharge code

---

## TUI Architecture

### Application State Machine (`tui/app.rs`)

```rust
pub enum AppState {
    ManufacturerList,
    FamilyList,
    ArticleList,
    FamilyConfig,
    PropertyEditor,
    CatalogBrowser,
    TableExplorer,
    Help,
}

pub struct App {
    state: AppState,
    manufacturer_view: ManufacturerView,
    family_view: FamilyView,
    config_view: ConfigView,
    // ... other views
}

impl App {
    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        match self.state {
            AppState::ManufacturerList => self.manufacturer_view.handle(event),
            AppState::FamilyList => self.family_view.handle(event),
            // ... dispatch to active view
        }
    }
}
```

### View Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Application (app.rs)                            │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                      Active View                                  │   │
│  │  ┌─────────────────────────────────────────────────────────┐    │   │
│  │  │              View Content                                │    │   │
│  │  │  ┌─────────────────┐  ┌──────────────────────────────┐  │    │   │
│  │  │  │    List Widget  │  │    Property Form Widget      │  │    │   │
│  │  │  │  (reusable)     │  │    (reusable)                │  │    │   │
│  │  │  └─────────────────┘  └──────────────────────────────┘  │    │   │
│  │  └─────────────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                      Status Bar                                   │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### Rendering (`tui/ui.rs`)

Uses ratatui for terminal rendering:

```rust
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(1),  // Status
        ])
        .split(frame.area());

    draw_header(frame, chunks[0], app);
    draw_content(frame, chunks[1], app);
    draw_status(frame, chunks[2], app);
}
```

### Config Store (`tui/config_store.rs`)

Persistence for user data:

```rust
// ~/.ofml-configs/
//   ├── favorites.json      # Starred product families
//   ├── history.json        # Recently configured products
//   └── *.json              # Saved configurations

pub fn save_configuration(config: &SavedConfiguration) -> Result<PathBuf>;
pub fn load_favorites() -> FavoritesStore;
pub fn add_to_history(mfr: &str, series: &str, article: &str) -> Result<()>;
```

---

## Performance Architecture

### Parallel Processing Strategy

The application uses `rayon` for CPU-bound parallel operations:

```rust
// Parallel manufacturer loading
manufacturers
    .into_par_iter()
    .map(|mfr| FamilyLoader::load(&mfr.path, region))
    .collect()

// Parallel EBase table loading
let (articles, prices, properties) = rayon::join(
    || load_articles(reader),
    || rayon::join(
        || load_prices(reader),
        || load_properties(reader),
    ),
);
```

### Performance Benchmarks

| Operation | Single-threaded | Parallel | Speedup |
|-----------|-----------------|----------|---------|
| Sedus Families | ~500ms | ~268ms | 1.87x |
| Bisley Families | ~80ms | ~42ms | 1.90x |
| Vitra Families | ~320ms | ~172ms | 1.86x |

### Optimization Techniques

1. **Lazy Loading**: Data loaded on-demand per view
2. **Caching**:
   - Texture cache for repeated image access
   - EBase table metadata caching
3. **Parallel I/O**:
   - Concurrent EBase table reads
   - Parallel ALB extraction
4. **Memory Efficiency**:
   - Arc/Rc for shared data structures
   - String interning for repeated values

### Build Optimizations

```toml
[profile.release]
lto = "thin"           # Link-time optimization
codegen-units = 1      # Better optimization
panic = "abort"        # Smaller binary
strip = true           # Strip symbols

[profile.release-max]
inherits = "release"
lto = "fat"            # Maximum LTO
opt-level = 3          # Aggressive optimization
```

---

## Extension Points

### Adding New OFML Classes

1. Implement in `ofml_classes.rs`:

```rust
pub fn create_oi_custom(
    _interp: &mut Interpreter,
    args: &[Value],
) -> Result<Value> {
    // Validate arguments
    // Create geometry
    // Return object
}
```

2. Register in interpreter:

```rust
registry.register("OiCustom", create_oi_custom);
```

### Adding New CLI Commands

1. Add clap subcommand in `main.rs`:

```rust
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Custom command")]
    Custom {
        #[arg(help = "Input file")]
        input: PathBuf,
    },
}
```

2. Implement handler using operations module.

### Adding New TUI Views

1. Create view in `tui/views/`:

```rust
pub struct CustomView {
    state: ViewState,
}

impl CustomView {
    pub fn handle(&mut self, event: Event) -> Option<Action>;
    pub fn draw(&self, frame: &mut Frame, area: Rect);
}
```

2. Add state variant in `app.rs`.

### Adding New File Formats

1. Implement reader:

```rust
pub fn read_custom_format(path: &Path) -> Result<Scene3DS>;
```

2. Add format detection in `operations.rs`.

---

## Design Decisions

### Why Tree-Walking Interpreter?

- **Simplicity**: Easy to understand and modify
- **Debugging**: Direct AST access for inspection
- **Flexibility**: Easy to add new language features
- **Performance**: Adequate for configuration use case (not compute-intensive)

### Why Parallel with Rayon?

- **Ergonomics**: Minimal code changes for parallelism
- **Safety**: Rust's ownership prevents data races
- **Performance**: Near-linear scaling on multi-core
- **Composability**: Works well with iterators

### Why ratatui for TUI?

- **Cross-platform**: Works on Windows, macOS, Linux
- **Immediate mode**: Simple rendering model
- **Widgets**: Rich set of built-in components
- **Performance**: Efficient terminal updates

### Why GLB as Output Format?

- **Standard**: glTF 2.0 is widely supported
- **Binary**: Compact, single-file output
- **Features**: Materials, textures, animations
- **Tooling**: Many viewers and converters available

---

## Future Considerations

1. **WebAssembly**: Core interpreter could be compiled to WASM
2. **LSP Support**: Language Server Protocol for CLS editing
3. **GPU Rendering**: Scene preview in TUI using GPU acceleration
4. **Incremental Parsing**: Faster re-parsing for live editing
5. **Plugin System**: Dynamic loading of manufacturer-specific extensions

---

## Appendix: Key Data Structures

### Value (Runtime Values)

```rust
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Object(Rc<RefCell<ObjInstance>>),
    Func(FuncValue),
    Class(ClassValue),
    // ...
}
```

### Scene3DS (3D Geometry)

```rust
pub struct Scene3DS {
    pub meshes: Vec<Mesh3DS>,
    pub materials: HashMap<String, Material3DS>,
}

pub struct Mesh3DS {
    pub name: String,
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[u16; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub material: String,
}
```

### OcdData (Configuration Data)

```rust
pub struct OcdData {
    pub articles: Vec<OcdArticle>,
    pub prices: Vec<OcdPrice>,
    pub property_classes: HashMap<String, OcdPropertyClass>,
    pub property_values: HashMap<String, Vec<OcdPropertyValue>>,
    pub texts: HashMap<String, OcdText>,
}
```
