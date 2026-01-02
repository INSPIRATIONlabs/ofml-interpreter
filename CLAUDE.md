# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

OFML Interpreter is a Rust library and CLI for interpreting OFML (Office Furniture Modeling Language) data. It provides:
- CLS bytecode interpreter for OFML class files
- EBase file format reader
- OAP (OFML Article Properties) configurator with pricing
- Terminal UI for product configuration

## Development Commands

```bash
cargo build --features tui --release    # Build with TUI support
cargo build                             # Build without TUI
cargo test                              # Run all tests
cargo test test_name -- --nocapture     # Run specific test with output
cargo clippy                            # Lint code
cargo fmt                               # Format code

# Run TUI configurator
./target/release/ofml tui /reference/ofmldata
```

## Key Modules

| Module | Purpose |
|--------|---------|
| `src/oap/` | OAP (OFML Article Properties) configurator |
| `src/oap/engine.rs` | Configuration engine with price calculation |
| `src/oap/ocd.rs` | OCD data reader (articles, prices, texts) |
| `src/oap/ocd_properties.rs` | OCD property definitions and values |
| `src/oap/families.rs` | Product family grouping and configuration |
| `src/tui/` | Terminal UI for product configuration |
| `src/interpreter.rs` | CLS bytecode interpreter |
| `src/ebase.rs` | EBase file format reader |

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

- `docs/OCD-PRICING-IMPLEMENTATION.md` - Pricing implementation details
- `docs/OFML-EXPLAINED.md` - OFML concepts
- `docs/CLS-EXAMPLES.md` - CLS bytecode examples
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
- **Complete implementations**: When implementing a feature, ensure all parts are connected and working. For example, if adding price calculation, ensure it's triggered when relevant (e.g., on property change).

## Dependencies

Key crates:
- `logos` - Lexer generation
- `serde/serde_json` - Serialization
- `zip` - ALB archive handling
- `gltf` - GLB export
- `byteorder` - Binary parsing
- `thiserror` - Error handling
- `ratatui` - Terminal UI (feature: tui)
- `crossterm` - Terminal handling (feature: tui)
