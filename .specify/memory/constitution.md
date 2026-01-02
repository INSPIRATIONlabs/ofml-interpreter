<!--
  ============================================================================
  SYNC IMPACT REPORT
  ============================================================================
  Version Change: N/A (initial) -> 1.0.0

  Added Principles:
    - I. Data-Driven Genericity (NEW)
    - II. Reference Compatibility (NEW)
    - III. Test-First Quality (NEW)
    - IV. Performance & Efficiency (NEW)
    - V. User Experience Consistency (NEW)
    - VI. Agent-Assisted Development (NEW)
    - VII. Maximum Code Reusability (NEW)
    - VIII. Comprehensive Documentation (NEW)

  Added Sections:
    - Available Agents
    - Quality Gates
    - Governance

  Templates Requiring Updates:
    - .specify/templates/plan-template.md: Constitution Check section references
      updated principles (pending user-driven update on next /speckit.plan)
    - .specify/templates/spec-template.md: No updates required
    - .specify/templates/tasks-template.md: No updates required
    - .specify/templates/checklist-template.md: No updates required

  Follow-up TODOs: None
  ============================================================================
-->

# OFML Interpreter Constitution

## Core Principles

### I. Data-Driven Genericity

All implementations MUST be data-driven and manufacturer-agnostic. The codebase
MUST NOT contain manufacturer-specific code paths, conditionals, or hardcoded
values.

**Non-Negotiables:**
- MUST parse and interpret manufacturer data using only OFML/OCD specifications
- MUST NOT add `if manufacturer == "xxx"` style conditionals
- When manufacturer data varies, MUST find the generic pattern that covers all
  cases or extend the data model to accommodate variation
- Configuration differences MUST be handled through data, not code
- Use the `ofmldata-investigator` agent to analyze patterns across manufacturers
  before implementing any data handling logic

**Rationale:** The OFML specification defines a universal data format. If one
manufacturer's data requires special handling, it indicates either a gap in our
specification understanding or a data quality issue, not a need for special code.

### II. Reference Compatibility

The library MUST produce results identical to or better than the reference
applications (pcon.basket and pcon.configurator). Pricing calculations are the
critical path.

**Non-Negotiables:**
- Price calculations MUST match pcon.basket/pcon.configurator output exactly
- Product configuration MUST support all options the reference apps support
- When discrepancies exist, use `pcon-basket-analyst` and
  `pcon-configurator-analyst` agents to understand reference behavior
- Use `pcon-dataclient-analyst` to understand data acquisition and format
- Document any intentional deviations with justification
- Test against multiple manufacturers (minimum: 5 diverse manufacturers)

**Rationale:** Users expect identical results when switching tools. Price
discrepancies erode trust and cause business issues.

### III. Test-First Quality

All features MUST have comprehensive tests that verify correctness against
known good values.

**Non-Negotiables:**
- Tests MUST be written before or alongside implementation
- Price calculation tests MUST compare against reference application output
- Integration tests MUST cover real manufacturer data from `/reference/ofmldata`
- Unit tests MUST cover edge cases in parsing and calculation logic
- Tests MUST NOT use mocked manufacturer data when real data is available
- Run `cargo test` passes before any commit

**Test Categories:**
- **Unit tests**: Core parsing, calculation, and data transformation logic
- **Integration tests**: End-to-end flows with real OFML data
- **Compatibility tests**: Output comparison with reference applications

**Rationale:** The EBase format is undocumented. Tests against real data are
the only way to verify correctness.

### IV. Performance & Efficiency

The library MUST handle large manufacturer catalogs efficiently and provide
responsive user experiences in the TUI.

**Non-Negotiables:**
- EBase file reading MUST be lazy/streaming where possible
- Price lookups MUST complete in <100ms for typical articles
- TUI navigation MUST maintain 60fps responsiveness
- Memory usage MUST scale linearly with data, not exponentially
- Profile before optimizing; measure after

**Constraints:**
- `/reference/ofmldata` contains ~50 manufacturers with varying catalog sizes
- Users MUST be able to switch between manufacturers without UI freezing
- Use the `tui-ux-expert` agent to identify performance bottlenecks in TUI

**Rationale:** A slow tool is an unused tool. Users configure products
interactively and expect immediate feedback.

### V. User Experience Consistency

The TUI MUST provide a consistent, intuitive experience across all
manufacturers and product types.

**Non-Negotiables:**
- Navigation patterns MUST be identical regardless of manufacturer
- Property editing MUST work the same way for all property types
- Price display format MUST be consistent (currency, precision, breakdown)
- Error messages MUST be actionable and user-friendly
- Use `tui-ux-expert` agent for all TUI design decisions and reviews
- Support keyboard-only navigation throughout

**Standards:**
- Property labels: Use `ocd_pricetext` descriptions in user's language
- Currency: Display with proper symbol and locale formatting
- Surcharges: Always show breakdown, not just total

**Rationale:** Users configure products from many manufacturers. Inconsistent
UX forces users to relearn the tool for each manufacturer.

### VI. Agent-Assisted Development

Development MUST leverage specialized agents for analysis, investigation, and
documentation tasks.

**Available Agents and Their Purposes:**

| Agent | Purpose |
|-------|---------|
| `pcon-configurator-analyst` | Understand pcon.configurator reference behavior |
| `pcon-basket-analyst` | Understand pcon.basket reference behavior |
| `pcon-dataclient-analyst` | Understand data acquisition and EBase format |
| `ofmldata-investigator` | Investigate manufacturer data patterns |
| `tui-ux-expert` | Design and review TUI components |
| `docs-writer` | Create and maintain documentation |
| `ofml-spec-expert` | Consult OFML/OCD specifications |

**Non-Negotiables:**
- MUST use `ofmldata-investigator` before implementing new data parsing
- MUST use `pcon-*-analyst` agents when behavior differs from reference
- MUST use `tui-ux-expert` for any TUI changes affecting user interaction
- MUST use `docs-writer` to document new features and APIs
- Delegate research tasks to appropriate agents instead of ad-hoc exploration

**Rationale:** Specialized agents have deep context on specific domains.
Delegation produces more accurate results and faster development.

### VII. Maximum Code Reusability

All functionality MUST be implemented as composable, reusable components.
No feature exists in isolation.

**Non-Negotiables:**
- Library-first: Core logic MUST be usable without TUI or CLI
- Clear module boundaries with documented public APIs
- No circular dependencies between modules
- Shared data types for cross-module communication
- DRY: Extract common patterns into reusable functions/traits

**Module Structure:**
- `oap/`: OAP configurator engine (reusable)
- `ebase.rs`: EBase reader (reusable)
- `tui/`: Terminal UI (consumes oap/)
- `cli/`: Command-line interface (consumes oap/)

**Rationale:** A library that only works inside one application is not a
library. Other tools MUST be able to embed OFML interpretation.

### VIII. Comprehensive Documentation

All public APIs, data formats, and implementation decisions MUST be documented.

**Non-Negotiables:**
- All public functions MUST have rustdoc comments
- New features MUST be documented in `/docs` before merge
- Implementation quirks MUST be documented in code comments
- Use `docs-writer` agent for documentation tasks
- Document deviations from OFML specification with rationale

**Documentation Types:**
- API docs: rustdoc for all public interfaces
- User guides: How to use the TUI and CLI
- Implementation notes: How complex features work internally
- Specification references: Links to relevant OFML spec sections

**Rationale:** The OFML ecosystem is complex and poorly documented publicly.
Our documentation fills this gap for future maintainers and users.

## Quality Gates

All code changes MUST pass these gates before merge:

| Gate | Requirement | Tool |
|------|-------------|------|
| Build | `cargo build --features tui --release` succeeds | CI |
| Tests | `cargo test` passes (all tests) | CI |
| Lint | `cargo clippy` has no warnings | CI |
| Format | `cargo fmt --check` passes | CI |
| Compatibility | Price tests match reference output | Manual |
| Documentation | New public APIs documented | Review |
| Genericity | No manufacturer-specific code | Review |

## Governance

### Amendment Process

1. Propose changes via discussion with justification
2. Document impact on existing implementations
3. Update all dependent templates if principles change
4. Version bump according to semantic versioning rules

### Versioning Policy

- **MAJOR**: Principle removal or fundamental redefinition
- **MINOR**: New principle added or material guidance expansion
- **PATCH**: Clarifications, wording improvements, typo fixes

### Compliance

- All pull requests MUST verify compliance with this constitution
- Code reviewers MUST check for manufacturer-specific code
- Test coverage MUST include multiple manufacturers
- Use CLAUDE.md for runtime development guidance

**Version**: 1.0.0 | **Ratified**: 2026-01-02 | **Last Amended**: 2026-01-02
