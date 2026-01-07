# Implementation Plan: Universal OCD Data Reader

**Branch**: `001-universal-ocd-reader` | **Date**: 2026-01-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-universal-ocd-reader/spec.md`

## Summary

Rock-solid universal OCD data reader that can parse all manufacturer data formats in `/reference/ofmldata`, handle data corruption/variations gracefully, and provide accurate pricing for any configuration. The library will be manufacturer-agnostic with the TUI serving as the demo interface.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: byteorder (binary parsing), serde/serde_json (serialization), ratatui/crossterm (TUI), chrono (dates), tracing (logging)
**Storage**: Read-only access to EBase files (pdata.ebase) in OFML data directory
**Testing**: cargo test with unit, integration, and compatibility tests against real manufacturer data
**Target Platform**: Linux/macOS/Windows terminal (80x24 minimum with color support)
**Project Type**: Single project (library + TUI + CLI)
**Performance Goals**: <100ms price recalculation, 60fps TUI responsiveness
**Constraints**: No manufacturer-specific code branches, memory scales linearly with data
**Scale/Scope**: ~50 manufacturers in /reference/ofmldata, varying catalog sizes

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Data-Driven Genericity | ✅ PASS | SC-006 explicitly requires no manufacturer-specific code branches |
| II. Reference Compatibility | ✅ PASS | SC-002/SC-008 require matching pcon.basket/configurator output |
| III. Test-First Quality | ✅ PASS | Tests against real /reference/ofmldata; compatibility tests planned |
| IV. Performance & Efficiency | ✅ PASS | FR-009 requires <100ms price update; SC-003 measurable |
| V. UX Consistency | ✅ PASS | FR-010/FR-014 define consistent TUI and labeling |
| VI. Agent-Assisted Dev | ✅ PASS | Will use ofmldata-investigator, pcon-*-analyst, tui-ux-expert |
| VII. Code Reusability | ✅ PASS | Library-first design; TUI consumes library |
| VIII. Documentation | ✅ PASS | docs-writer agent for feature documentation |

**Gate Result**: PASS - No violations. Proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-universal-ocd-reader/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (library API contracts)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Library entry point
├── ebase.rs             # EBase binary file reader (existing)
├── main.rs              # CLI entry point
├── oap/                 # OAP Configurator (core library)
│   ├── mod.rs           # Module exports
│   ├── ocd.rs           # OCD data reader (prices, articles, texts)
│   ├── ocd_properties.rs # Property definitions and values
│   ├── families.rs      # Product family grouping
│   ├── engine.rs        # Configuration engine with price calculation
│   ├── manufacturers.rs # Manufacturer discovery
│   ├── catalog.rs       # Catalog management
│   ├── price.rs         # Price calculation utilities
│   ├── variant.rs       # Variant condition matching
│   ├── property.rs      # Property handling
│   ├── config.rs        # Configuration state
│   ├── actions.rs       # User actions
│   └── oam.rs           # OAM file support
└── tui/                 # Terminal UI (consumes oap/)
    ├── mod.rs           # TUI module
    ├── app.rs           # Application state
    ├── ui.rs            # Rendering logic
    ├── views/           # Screen views
    │   ├── manufacturers.rs
    │   ├── families.rs
    │   ├── family_config.rs
    │   ├── articles.rs
    │   ├── properties.rs
    │   └── help.rs
    └── widgets/         # Reusable UI components

tests/
├── unit/                # Unit tests
│   ├── oap_price_test.rs
│   ├── oap_variant_test.rs
│   ├── propvalue2varcond_test.rs
│   └── table_relation_test.rs
├── integration/         # Integration tests
│   └── cli_configure_test.rs
├── multi_manufacturer_tests.rs      # Multi-manufacturer compatibility
├── multi_manufacturer_price_test.rs # Price accuracy across manufacturers
└── ofmldata_integration_tests.rs    # Real data integration tests
```

**Structure Decision**: Single project with library-first architecture. The `oap/` module is the reusable library core; `tui/` is the demo consumer.

## Complexity Tracking

> **No violations to justify - Constitution Check passed.**

N/A - All constitution gates passed without violations.
