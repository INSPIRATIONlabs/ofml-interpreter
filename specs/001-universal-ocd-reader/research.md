# Research: Universal OCD Data Reader

**Feature**: 001-universal-ocd-reader
**Date**: 2026-01-02

## 1. OCD Pricing Strategies

### Decision: Support Four Pricing Strategies

**Rationale**: Analysis of /reference/ofmldata reveals four distinct patterns used across manufacturers. A universal reader must handle all.

| Strategy | Manufacturers | Base Price Pattern | Surcharge Pattern |
|----------|---------------|-------------------|-------------------|
| Empty Base + Named | Framery, Bisley | `var_cond=""` | `PG_*`, `DE_*` prefixes |
| Product Group + Numeric | Sedus | `var_cond="S_PGX"` | `S_{number}` format |
| TABLE-Computed | FAST | Dynamically computed from properties | None (encoded in base) |
| Complex Code System | Arper | Encoded codes | Abbreviation codes |

**Alternatives Considered**:
- Single pattern assumption → Rejected: Would only work for subset of manufacturers
- Manufacturer-specific handlers → Rejected: Violates Constitution Principle I

## 2. Var_Cond Matching

### Decision: Heuristic Pattern Recognition

**Rationale**: The OCD spec defines a formal RelationObj/Relation system for setting variant conditions, but no manufacturers in our dataset use `ocd_propvalue2varcond` mapping tables. All matching must be inferred.

**Matching Strategies** (in order of application):
1. **Direct match**: Property value exactly equals var_cond suffix (e.g., value "166" matches "S_166")
2. **Prefix match**: Property value starts with numeric portion of var_cond
3. **TABLE computation**: For FAST-style manufacturers, compute var_cond from property table relations

**Base Price Indicators**: `["S_PGX", "BASE", "STANDARD", ""]` (empty string is most common)

**Alternatives Considered**:
- Full Relation evaluation → Deferred: Complex, no real-world demand yet
- SAP LOVC syntax support → Deferred: Not used in current dataset

## 3. Data Corruption Handling

### Decision: Pattern-Based Recovery

**Rationale**: Investigation of Framery data revealed systematic corruption patterns that are recoverable.

**Known Corruption Patterns**:

| Pattern | Affected | Symptoms | Recovery |
|---------|----------|----------|----------|
| 8-byte offset shift | Framery frmr_one_compact records 9-14 | Missing record_id and article_nr; all fields shifted | Read fields at offset +8 |
| Trailing whitespace | Multiple manufacturers | `price_level = "B  "` fails validation | `.trim()` all string fields |
| Empty article with valid data | Corrupted records | article_nr empty but other fields populated | Detect pattern, recover known prices |

**Alternatives Considered**:
- Fail on corruption → Rejected: Loses valid data, violates SC-005
- Manual recovery files → Rejected: Not maintainable, violates Principle I

## 4. Wildcard Pricing (article_nr = "*")

### Decision: Two-Pass Lookup

**Rationale**: Wildcard prices should only apply when no article-specific price exists. Without two-pass lookup, alphabetical file iteration causes incorrect wildcard matches.

**Algorithm**:
1. **First pass**: Search all pdata.ebase files for exact article match with price_level='B'
2. **Second pass**: If no match, use wildcard prices from any file

**Implementation**: Already implemented in `engine.rs` after Framery investigation.

**Alternatives Considered**:
- Single-pass with priority sorting → Rejected: Complex, error-prone
- File ordering assumptions → Rejected: Brittle, manufacturer-dependent

## 5. Price Level Processing

### Decision: Follow OCD Spec Order (B → X → D)

**Rationale**: OCD 4.3 spec defines processing order.

| Level | Name | Processing | Notes |
|-------|------|------------|-------|
| B | Base | First | Must be fixed amount |
| X | Surcharge | Second | Can be fixed or percentage of base; can be negative |
| D | Discount | Third | Typically percentage-based |

**Formula**: `Total = Base + Σ(Surcharges) - Σ(Discounts)`

## 6. Property to Surcharge Linking

### Decision: Heuristic Code Extraction

**Rationale**: The formal RelationObj system is not implemented. Property values often encode surcharge codes directly.

**Extraction Methods**:
1. **Suffix extraction**: "FARBE_166" → code "166" → matches "S_166"
2. **Direct value**: "PG_ADJUSTABLE_SEAT" → matches "PG_ADJUSTABLE_SEAT"
3. **Numeric prefix**: Property value "166XYZ" → code "166"

## 7. Export Format

### Decision: JSON

**Rationale**: Selected during clarification session. Most versatile for structured configuration data.

**Schema** (simplified):
```json
{
  "article_nr": "string",
  "manufacturer": "string",
  "series": "string",
  "configuration": {
    "property_id": "selected_value"
  },
  "pricing": {
    "base": 12280.00,
    "surcharges": [{"var_cond": "PG_TABLE_TOP", "amount": 700.00}],
    "discounts": [],
    "total": 12980.00,
    "currency": "EUR"
  }
}
```

## 8. Error Communication

### Decision: Silent with Warning Indicator

**Rationale**: Selected during clarification session. Maintains smooth UX while providing transparency.

**Implementation**:
- TUI: Warning icon/badge when data issues detected
- Logging: Detailed diagnostics via tracing crate
- No modal interruptions for recoverable issues

## 9. Reference Application Behavior Analysis

### Decision: Match pcon.configurator Behavior (Primary), pcon.basket as Secondary Reference

**Rationale**: Constitution Principle II requires matching reference applications. Analysis of decompiled source code reveals the exact algorithms used.

**Source**: Decompiled C code analysis from `/reference/ConceptOffice7/sources/`

### pcon.configurator Findings

**Price Table Reading** (`ocd_price` table):
- Fields: `ArticleID`, `Variantcondition`, `Level`, `FixValue`, `PriceValue`, `DateFrom`, `DateTo`
- Data sources: csv:, ebase:, xbase: (in priority order)

**Var_cond Resolution** (from `matches_var_cond_extended()`):
1. Direct `propvalue2varcond` lookup (when available)
2. TABLE-based computation (OCD_4 language)
3. Pattern matching fallbacks:
   - Case-insensitive direct match
   - Sedus S_ codes (`S_166` matches `"166"`)
   - Formula matching (`KEY=value`)
   - Price group codes (`PG11`, `GL1`, `MG1`)

**Price Calculation** (from `updatePrice()`):
```
Base (B) → Surcharges (X) → Discounts (D)
Total = base + Σ(surcharges) - Σ(discounts)
```

**Wildcard Rules**:
- `article_nr="*"` only valid for X and D (not base prices)
- Requires non-empty var_cond
- Article-specific takes precedence

### pcon.basket Findings

**Key Differences from pcon.configurator**:

| Aspect | pcon.basket | pcon.configurator |
|--------|-------------|-------------------|
| Architecture | IPriceProfileManager | Direct OCD access |
| Caching | Heavy profile caching | Real-time lookup |
| Duplicate var_cond | Counted once | All applied |
| Tax handling | Full TaxCalculator | Limited |

**Code References**:
- `OFMLPriceProfile_ctor`: line 232763 in `ebasket/decompiled.renamed.c`
- `ECalcItem::updatePrice`: line 22601
- `ECalcItem::updatePriceModifier`: line 24291

### Implementation Implications

| Reference Behavior | Our Implementation |
|-------------------|-------------------|
| propvalue2varcond lookup first | Check table before heuristics |
| Two-pass wildcard lookup | Implemented in engine.rs |
| Skip malformed records | DataWarning + continue |
| Invalid dates → always-valid | Treat as always-valid with warning |

**Full Documentation**:
- `/docs/PCON-CONFIGURATOR-PRICING-ANALYSIS.md`
- `/docs/PCON-BASKET-PRICING-ANALYSIS.md`

## Research Gaps (Deferred)

| Topic | Reason for Deferral |
|-------|---------------------|
| Full Relation evaluation | Not needed for current dataset; high complexity |
| SAP LOVC syntax | Not used by any current manufacturer |
| Multiple currency support | All manufacturers use EUR in dataset |
| Scale quantity pricing | No real-world test cases available |
