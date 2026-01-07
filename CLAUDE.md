# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

OFML Interpreter is a Rust workspace for interpreting OFML (Office Furniture Modeling Language) data. It provides:
- CLS bytecode interpreter for OFML class files
- EBase file format reader
- OAP (OFML Article Properties) configurator with pricing
- Terminal UI for product configuration

## Workspace Structure

```
crates/
├── ofml-lib/     # Core library - all OFML parsing and business logic
├── ofml-tui/     # Terminal UI - product configurator
└── ofml-cli/     # CLI - command-line interface
```

## Development Commands

```bash
# Build entire workspace
cargo build --workspace

# Build individual crates
cargo build -p ofml-lib
cargo build -p ofml-tui
cargo build -p ofml-cli

# Run tests
cargo test --workspace                    # All tests
cargo test -p ofml-lib                    # Library tests only
cargo test -p ofml-lib --test unit        # Unit tests

# Lint and format
cargo clippy --workspace
cargo fmt

# Run TUI configurator
cargo run -p ofml-tui -- /reference/ofmldata

# Run CLI
cargo run -p ofml-cli -- manufacturers /reference/ofmldata
```

## Key Modules (ofml-lib)

| Module | Purpose |
|--------|---------|
| `oap/engine.rs` | Configuration engine with price calculation |
| `oap/ocd.rs` | OCD data reader (articles, prices, texts) |
| `oap/ocd_properties.rs` | OCD property definitions and values |
| `oap/families.rs` | Product family grouping and configuration |
| `oap/price.rs` | Price lookup and calculation |
| `interpreter.rs` | CLS bytecode interpreter |
| `ebase.rs` | EBase file format reader |
| `operations.rs` | High-level operations (export, assembly) |

## OCD Pricing Model

The pricing system uses multiple data tables:

- **ocd_price**: Base prices (level 'B') and surcharges (level 'X')
- **ocd_pricetext**: Multilingual descriptions for price entries
- **ocd_propertyclass**: Maps articles to property classes
- **ocd_propertyvalue**: Available options for each property

**Variant Condition Matching:**
```rust
// Base price indicators (var_cond field)
let base_indicators = ["S_PGX", "BASE", "STANDARD", ""];

// Surcharge codes like "S_166" match property values
// Strategy 1: Direct match (value == "166")
// Strategy 2: Suffix match (value ends with "166")
// Strategy 3: Numeric prefix match (value starts with "166")
```

## Documentation

- `docs/LIBRARY-OVERVIEW.md` - Library architecture
- `docs/DATA-FORMATS.md` - OFML data format documentation
- `docs/PRICING-GUIDE.md` - Pricing system guide
- `docs/OCD-PRICING-IMPLEMENTATION.md` - Pricing implementation details
- `docs/ofml-specs/ocd_4_3.md` - OCD specification
- `docs/ofml-specs/ofml_20r3-en.md` - OFML 2.0 specification

## Key Data Paths (in devcontainer)

- OFML data directory: `/reference/ofmldata/`
- Reference codebase: `/reference/ConceptOffice7/`
- Manufacturer structure: `/reference/ofmldata/{mfr_id}/{series}/DE/1/db/pdata.ebase`
- Example: `/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase` (Sedus AI chairs)

## Code Style

- **Rust**: Standard rustfmt, clippy for linting
- Edition 2021
- Use `thiserror` for error handling
- Use `serde` for serialization

## Development Rules

- **No silent TODOs or workarounds**: Do not add `// TODO` comments or incomplete implementations without asking. Either implement the feature fully or ask if it should be implemented. Never silently defer work.
- **Complete implementations**: When implementing a feature, ensure all parts are connected and working.
- **Library-first**: All business logic goes in ofml-lib. TUI and CLI are thin wrappers.
- **Generic code**: No manufacturer-specific code. The library must handle all OFML data generically.

## Dependencies

| Crate | Purpose | Location |
|-------|---------|----------|
| `logos` | Lexer generation | ofml-lib |
| `serde/serde_json` | Serialization | all |
| `zip` | ALB archive handling | ofml-lib |
| `gltf` | GLB export | ofml-lib |
| `byteorder` | Binary parsing | ofml-lib |
| `thiserror` | Error handling | ofml-lib |
| `ratatui` | Terminal UI | ofml-tui |
| `crossterm` | Terminal handling | ofml-tui |
| `clap` | CLI parsing | ofml-tui, ofml-cli |

## Test Coverage

- 601+ tests across the workspace
- Integration tests for all 108 manufacturers in /reference/ofmldata
- Unit tests for pricing, properties, OCD parsing
- Snapshot tests for TUI rendering
