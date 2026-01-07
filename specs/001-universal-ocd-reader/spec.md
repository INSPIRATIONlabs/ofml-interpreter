# Feature Specification: Universal OCD Data Reader

**Feature Branch**: `001-universal-ocd-reader`
**Created**: 2026-01-02
**Status**: Draft
**Input**: User description: "Solve the various problems we have with different manufacturer datasets for all times. Sometimes we are not able to read / update the prices when changing configurations in the TUI. Part of the problem seems to be that there are different ways of saving the data in the tables in ebase. We need a rock solid super stable solution which is able to read all manufacturer data, list all prices, calculate new prices on changes of options. We need to be able to read everything which can the pcon.configurator and pcon.basket read. The data is downloaded into the folder /reference/ofmldata and there are various manufacturers and datasets. We must be the super pro in reading these datasets. The TUI should be our demo to reflect the features of the library."

## Clarifications

### Session 2026-01-02

- Q: How should data issues (corrupted records, missing prices) be communicated to users? → A: Silent with warning indicator (icon/badge in TUI, details in log)
- Q: What format should the export functionality use? → A: JSON (structured, machine-readable, widely compatible)

## Reference Behavior (pcon.configurator / pcon.basket)

This section documents how the reference applications read and process OCD pricing data. Our implementation MUST match this behavior per SC-002 and SC-008.

### Price Calculation Order

Both applications follow this sequence:

```
1. Base Price (Level 'B') - Applied first
2. Surcharges (Level 'X') - Accumulated and added to base
3. Discounts  (Level 'D') - Subtracted last

Formula: Total = Base + Σ(Surcharges) - Σ(Discounts)
```

### Var_cond Resolution Priority

Both apps use this matching priority:

1. **Direct `propvalue2varcond` lookup** - When table exists (most accurate)
2. **TABLE-based computation** - For OCD_4 language relations
3. **Pattern matching fallbacks:**
   - Direct match (case-insensitive)
   - Sedus S_ code matching (`S_166` → value `"166"`)
   - Formula matching (`KEY=value`, `KEY>value`)
   - Price group codes (`PG11`, `GL1`, `MG1`)

**Base Price Indicators** (not treated as surcharges): `S_PGX`, `BASE`, `STANDARD`, `""` (empty)

### Wildcard Handling

- Wildcards (`article_nr="*"`) are **only valid for surcharges (X) and discounts (D)**
- **Article-specific prices always take precedence** over wildcards
- Two-pass lookup required: exact match first, then wildcard fallback

### Key Behavioral Differences

| Aspect | pCon.basket | pCon.configurator |
|--------|-------------|-------------------|
| Architecture | Profile-based caching | Direct OCD access |
| Focus | Final order prices | Real-time preview |
| Multiple surcharges | Same var_cond counted once | All matching applied |

**Implementation Decision**: Match pcon.configurator behavior (all matching surcharges applied). This is more conservative - users see the full cost impact. If a surcharge should only apply once, the data should use distinct var_cond values.

### Corrupted Data Handling

Both apps:
- Skip malformed records, continue processing
- Log warnings for invalid data
- Fall back to "Price on request" for missing prices
- Treat invalid date ranges as always-valid

**Reference**: Full analysis in `/docs/PCON-CONFIGURATOR-PRICING-ANALYSIS.md` and `/docs/PCON-BASKET-PRICING-ANALYSIS.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Browse Any Manufacturer's Products (Priority: P1)

As a furniture configurator user, I want to browse products from any manufacturer in the OFML data directory and see accurate pricing, so that I can configure and price any supported furniture product regardless of the data format used by the manufacturer.

**Why this priority**: This is the core value proposition - universal compatibility with all manufacturer data. Without this, users cannot trust the system to work with their specific manufacturer's data.

**Independent Test**: Can be fully tested by loading any manufacturer from /reference/ofmldata, selecting a product family, and verifying that base prices display correctly. Delivers immediate value by providing accurate pricing visibility.

**Acceptance Scenarios**:

1. **Given** a manufacturer with standard OCD pricing (e.g., Sedus), **When** I select a product family, **Then** I see the correct base price in EUR matching the source data
2. **Given** a manufacturer with corrupted/shifted data records (e.g., Framery frmr_one_compact), **When** I select a product, **Then** the system recovers and displays the correct base price
3. **Given** a manufacturer with surcharge-only pricing model, **When** I select a product, **Then** I see the base price as 0 with surcharges itemized
4. **Given** a manufacturer with wildcard pricing (article_nr = "*"), **When** I select any article, **Then** the wildcard prices are correctly applied

---

### User Story 2 - Dynamic Price Recalculation on Configuration Change (Priority: P1)

As a furniture configurator user, I want prices to update instantly when I change product options, so that I can see the cost impact of each configuration choice in real-time.

**Why this priority**: Real-time price feedback is essential for the configurator's core purpose - helping users understand the cost of their choices.

**Independent Test**: Can be fully tested by selecting a product, changing an option (e.g., fabric color), and verifying that the total price updates. Delivers value by enabling informed configuration decisions.

**Acceptance Scenarios**:

1. **Given** a configured product with base price displayed, **When** I change an option that has a surcharge, **Then** the total price increases by the surcharge amount immediately
2. **Given** a product with multiple surcharge options selected, **When** I change one option to a non-surcharged value, **Then** the total price decreases by the removed surcharge
3. **Given** a manufacturer using var_cond matching for surcharges, **When** I select an option, **Then** the correct surcharge is matched and applied based on the var_cond pattern

---

### User Story 3 - View All Available Properties and Options (Priority: P2)

As a furniture configurator user, I want to see all configurable properties and their valid options for any product, so that I can understand what customization choices are available.

**Why this priority**: Property display is essential for configuration, but secondary to pricing accuracy. Users need to see options to configure, but pricing is the critical feedback.

**Independent Test**: Can be fully tested by selecting a product family and verifying all properties from ocd_property and ocd_propertyvalue tables are displayed with their labels.

**Acceptance Scenarios**:

1. **Given** a product with multiple property classes, **When** I view the configuration screen, **Then** I see all properties organized by class
2. **Given** a property with multiple options, **When** I expand that property, **Then** I see all valid options from ocd_propertyvalue
3. **Given** a manufacturer using shared property classes across series, **When** I select a product, **Then** I see only the properties relevant to that product's property classes

---

### User Story 4 - Load Any Supported Manufacturer (Priority: P2)

As a furniture configurator user, I want to load any manufacturer from the OFML data directory without errors, so that I can work with any supported furniture brand.

**Why this priority**: Broad manufacturer support demonstrates the universal reader capability, but individual manufacturers can be added incrementally.

**Independent Test**: Can be fully tested by iterating through all manufacturers in /reference/ofmldata and verifying each loads without errors.

**Acceptance Scenarios**:

1. **Given** the OFML data directory with multiple manufacturers, **When** I start the application, **Then** I see a list of all available manufacturers
2. **Given** a manufacturer with multiple series, **When** I select that manufacturer, **Then** I see all series as product families
3. **Given** a series with multiple articles, **When** I select that series, **Then** I can browse and configure any article

---

### User Story 5 - Export Configured Product Information (Priority: P3)

As a furniture configurator user, I want to export my configured product with its final price, so that I can use this information for quotes or orders.

**Why this priority**: Export functionality adds value but depends on accurate reading and pricing being complete first.

**Independent Test**: Can be fully tested by configuring a product, triggering export, and verifying output contains correct article, options, and price.

**Acceptance Scenarios**:

1. **Given** a fully configured product, **When** I request an export, **Then** I receive a JSON file with article number, selected options, and total price
2. **Given** multiple products configured, **When** I export, **Then** each product's configuration is preserved as a JSON array

---

### Edge Cases

- What happens when an EBase file has corrupted records with shifted byte offsets?
  - System attempts automatic recovery using known corruption patterns
- What happens when a base price is missing for an article?
  - System falls back to wildcard prices or surcharge-only model
- What happens when multiple price records match for the same article?
  - System prioritizes: 1) exact article match with base price, 2) wildcard with matching var_cond, 3) first valid match
- What happens when property classes are shared across multiple series?
  - System loads properties from all relevant property classes for the product
- What happens when date ranges on prices don't include today?
  - System uses the price with the closest valid date range
- What happens when currency is missing or malformed?
  - System defaults to EUR with a warning
- How are unrecoverable data issues communicated to the user?
  - Silent operation with warning indicator (icon/badge in TUI); detailed diagnostics written to log file

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST read EBase files from any manufacturer in the OFML data directory structure
- **FR-002**: System MUST parse ocd_price table and extract base prices (level 'B'), surcharges (level 'X'), and discounts (level 'D')
- **FR-003**: System MUST normalize data fields that may have trailing whitespace or inconsistent formatting
- **FR-004**: System MUST detect and recover from known data corruption patterns (e.g., 8-byte offset shifts)
- **FR-005**: System MUST support wildcard pricing (article_nr = "*") that applies to all articles
- **FR-006**: System MUST match surcharges to configurations using var_cond patterns
- **FR-007**: System MUST parse ocd_article, ocd_property, ocd_propertyvalue, and ocd_propertyclass tables
- **FR-008**: System MUST calculate total price as base price plus applicable surcharges minus applicable discounts
- **FR-009**: System MUST update displayed price within 100ms of configuration change
- **FR-010**: System MUST provide TUI interface demonstrating all library capabilities
- **FR-011**: System MUST support manufacturers with surcharge-only pricing models (no base prices)
- **FR-012**: System MUST locate correct pricing data from series-specific EBase files (prefer actual article base prices over wildcards from other files)
- **FR-013**: System MUST read product family definitions and group articles appropriately
- **FR-014**: System MUST display property labels in the appropriate language (defaulting to German/DE)
- **FR-015**: System MUST handle multiple pricing strategies: direct article prices, var_cond-based surcharges, TABLE-computed var_cond

### Key Entities

- **Manufacturer**: A furniture brand with data in the OFML directory (e.g., "sedus", "framery", "fast")
- **Series** (displayed as "Family" in TUI): A product line within a manufacturer containing related articles (e.g., "ai" for Sedus AI chairs)
- **Article**: A configurable product with a unique article number, associated property classes, and pricing
- **Property**: A configurable attribute of an article (e.g., "M_FRAME", "M_EXTERIOR") with valid values
- **Property Value**: A valid option for a property, potentially linked to a surcharge via var_cond
- **Price Record**: A pricing entry specifying base price (B), surcharge (X), or discount (D) with optional var_cond matching
- **EBase File**: Binary database file (pdata.ebase) containing OCD tables for a series

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All manufacturers in /reference/ofmldata load without errors (100% success rate)
- **SC-002**: Base prices match reference application (pcon.basket/pcon.configurator) within 0.01 EUR for all tested articles
- **SC-003**: Price recalculation completes within 100ms of configuration change (measured in TUI)
- **SC-004**: TUI displays correct prices for 100% of tested manufacturer/article combinations from acceptance test suite
- **SC-005**: System recovers valid prices from 100% of known corrupted data patterns (e.g., Framery ONE_COMPACT_BASE)
- **SC-006**: No manufacturer-specific code branches in core reading logic - all variations handled through generic patterns
- **SC-007**: All property options visible in TUI match the source ocd_propertyvalue data for each property class
- **SC-008**: Surcharges correctly apply/remove when toggling options, matching pcon.configurator behavior

## Assumptions

- EBase file format follows the documented structure with string tables and typed columns
- Manufacturers use OCD 4.x format for pricing and property data
- All required data for pricing is contained within pdata.ebase files (no external price lists required)
- The TUI will run in terminals supporting at least 80x24 characters with color support
- German (DE) language data is available for all manufacturers; other languages may not be present
- Prices are in EUR unless explicitly specified in the currency field
- When multiple valid prices exist for the same article/var_cond, the most recent (by date_from) takes precedence

## Out of Scope

- Creating new manufacturer data files (read-only system)
- Real-time synchronization with external pCon services
- Order submission or quote generation workflows
- Multi-language UI (TUI remains in English; data labels use source language)
- 3D visualization of configured products (geometry/materials)
- User authentication or multi-user scenarios
