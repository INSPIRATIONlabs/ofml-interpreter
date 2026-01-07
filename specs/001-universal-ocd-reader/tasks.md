# Tasks: Universal OCD Data Reader

**Input**: Design documents from `/specs/001-universal-ocd-reader/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story label (US1-US5)
- All file paths are relative to repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify existing project structure and ensure dependencies are configured

- [ ] T001 Verify Rust project structure matches plan.md in Cargo.toml
- [ ] T002 [P] Ensure tracing dependency is properly configured for logging in Cargo.toml
- [ ] T003 [P] Verify serde/serde_json configured for JSON export in Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**Testing Approach**: Existing tests in tests/unit/ and tests/integration/ cover foundational components. New tests added alongside implementation per Constitution III.

**⚠️ CRITICAL**: These tasks establish the data reading foundation all stories depend on

- [ ] T004 Refactor OcdPrice struct to include all fields from data-model.md in src/oap/ocd.rs
- [ ] T005 [P] Implement DataWarning struct and WarningSeverity enum in src/oap/ocd.rs
- [ ] T006 [P] Add data normalization (trim, uppercase) for all string fields on EBase read in src/oap/ocd.rs
- [ ] T007 Implement corrupted record detection pattern (8-byte offset shift) in src/oap/ocd.rs
- [ ] T008 [P] Implement corrupted record recovery function in src/oap/ocd.rs
- [ ] T009 Add warning collection mechanism to OcdReader in src/oap/ocd.rs
- [ ] T010 Create Price struct with base/surcharges/discounts/total in src/oap/price.rs
- [ ] T011 [P] Create Surcharge and Discount structs in src/oap/price.rs
- [ ] T012 [P] Create PriceBreakdown struct for detailed price display in src/oap/price.rs

**Checkpoint**: Foundation ready - all OCD reading infrastructure in place

---

## Phase 3: User Story 1 - Browse Any Manufacturer's Products (Priority: P1) 🎯 MVP

**Goal**: Universal compatibility with all manufacturer data formats, displaying accurate base prices

**Independent Test**: Load any manufacturer from /reference/ofmldata, select product family, verify base price displays correctly

### Implementation for User Story 1

- [ ] T013 [US1] Implement four pricing strategy detection in src/oap/ocd.rs (Empty Base, Product Group, TABLE-Computed, Complex Code)
- [ ] T014 [US1] Implement base price lookup with wildcard fallback in src/oap/engine.rs
- [ ] T015 [P] [US1] Implement two-pass price lookup (exact article first, then wildcard) in src/oap/engine.rs
- [ ] T016 [US1] Add surcharge-only pricing model support (base=0) in src/oap/engine.rs
- [ ] T017 [P] [US1] Implement date range validation for prices in src/oap/ocd.rs
- [ ] T018 [US1] Add price recovery for known corrupted patterns (Framery ONE_COMPACT_BASE) in src/oap/ocd.rs
- [ ] T019 [US1] Integrate warning collection into price calculation flow in src/oap/engine.rs
- [ ] T020 [US1] Update TUI family_config view to display base price with breakdown in src/tui/views/family_config.rs
- [ ] T021 [P] [US1] Add warning indicator (icon/badge) to TUI when data issues detected in src/tui/views/family_config.rs
- [ ] T022 [US1] Add tracing logs for price lookup decisions in src/oap/engine.rs

**Checkpoint**: User Story 1 complete - base prices display correctly for all manufacturers including corrupted data

---

## Phase 4: User Story 2 - Dynamic Price Recalculation (Priority: P1)

**Goal**: Prices update instantly (<100ms) when changing product options

**Independent Test**: Select product, change option with surcharge, verify total price updates immediately

### Implementation for User Story 2

- [ ] T023 [US2] Implement var_cond matching with heuristic pattern recognition in src/oap/variant.rs
- [ ] T023a [US2] Check propvalue2varcond table first before falling back to heuristics in src/oap/variant.rs
- [ ] T024 [P] [US2] Add direct match strategy (value equals var_cond suffix) in src/oap/variant.rs
- [ ] T025 [P] [US2] Add prefix match strategy (value starts with numeric portion) in src/oap/variant.rs
- [ ] T026 [US2] Implement TABLE-computed var_cond for FAST-style manufacturers in src/oap/variant.rs
- [ ] T027 [US2] Implement surcharge application (price_level='X') in src/oap/engine.rs
- [ ] T028 [P] [US2] Implement discount application (price_level='D') in src/oap/engine.rs
- [ ] T029 [US2] Add price caching with invalidation on property change in src/oap/engine.rs
- [ ] T030 [US2] Ensure calculate_price completes in <100ms (add timing check) in src/oap/engine.rs
- [ ] T031 [US2] Update TUI to trigger recalculation on property change in src/tui/views/family_config.rs
- [ ] T032 [US2] Display surcharge breakdown in TUI price area in src/tui/views/family_config.rs

**Checkpoint**: User Story 2 complete - prices update in real-time with full breakdown

---

## Phase 5: User Story 3 - View All Properties and Options (Priority: P2)

**Goal**: Display all configurable properties with valid options for any product

**Independent Test**: Select product family, verify all properties from ocd_property/ocd_propertyvalue display with labels

### Implementation for User Story 3

- [ ] T033 [US3] Implement PropertyClass loading from ocd_propertyclass table in src/oap/ocd_properties.rs
- [ ] T034 [P] [US3] Implement Property loading with label from ocd_propertytext in src/oap/ocd_properties.rs
- [ ] T035 [P] [US3] Implement PropertyValue loading with label from ocd_propertyvaluetext in src/oap/ocd_properties.rs
- [ ] T036 [US3] Filter properties by article's associated property classes in src/oap/ocd_properties.rs
- [ ] T037 [US3] Handle shared property classes across series in src/oap/ocd_properties.rs
- [ ] T038 [US3] Implement get_properties() function per library-api.md contract in src/oap/mod.rs
- [ ] T039 [US3] Create PropertyWithValues struct for TUI consumption in src/oap/ocd_properties.rs
- [ ] T040 [US3] Update TUI properties view to display all options organized by class in src/tui/views/properties.rs
- [ ] T041 [P] [US3] Add property label language support (default to DE) in src/oap/ocd_properties.rs

**Checkpoint**: User Story 3 complete - all properties visible and organized by class

---

## Phase 6: User Story 4 - Load Any Supported Manufacturer (Priority: P2)

**Goal**: Load any manufacturer from OFML directory without errors (100% success rate)

**Independent Test**: Iterate all manufacturers in /reference/ofmldata, verify each loads without errors

### Implementation for User Story 4

- [ ] T042 [US4] Implement discover_manufacturers() scanning /reference/ofmldata in src/oap/manufacturers.rs
- [ ] T043 [P] [US4] Implement load_manufacturer() by ID in src/oap/manufacturers.rs
- [ ] T044 [US4] Implement list_series() for manufacturer in src/oap/families.rs
- [ ] T045 [P] [US4] Implement load_series() with full OCD parsing in src/oap/families.rs
- [ ] T046 [US4] Handle manufacturer with multiple series correctly in src/oap/families.rs
- [ ] T047 [US4] Implement graceful error handling for malformed manufacturer data in src/oap/manufacturers.rs
- [ ] T048 [US4] Update TUI manufacturers view to list all discovered manufacturers in src/tui/views/manufacturers.rs
- [ ] T049 [P] [US4] Update TUI families view to show all series for selected manufacturer in src/tui/views/families.rs
- [ ] T050 [US4] Add integration test iterating all manufacturers in tests/multi_manufacturer_tests.rs

**Checkpoint**: User Story 4 complete - all ~50 manufacturers load without errors

---

## Phase 7: User Story 5 - Export Configured Product Information (Priority: P3)

**Goal**: Export configured product with final price as JSON

**Independent Test**: Configure product, trigger export, verify JSON contains correct article/options/price

### Implementation for User Story 5

- [ ] T051 [US5] Implement export_json() per library-api.md contract in src/oap/mod.rs
- [ ] T052 [P] [US5] Implement export_json_batch() for multiple configurations in src/oap/mod.rs
- [ ] T053 [US5] Add serde Serialize derives to Configuration, Price, Surcharge structs in src/oap/
- [ ] T054 [US5] Validate exported JSON against contracts/export-schema.json in tests/unit/oap_export_test.rs
- [ ] T055 [US5] Add export timestamp (exported_at) to JSON output in src/oap/mod.rs
- [ ] T056 [US5] Include warnings array in export if present in src/oap/mod.rs
- [ ] T057 [US5] Add TUI export action (key binding) in src/tui/app.rs
- [ ] T058 [P] [US5] Display export success/failure notification in TUI in src/tui/ui.rs

**Checkpoint**: User Story 5 complete - configurations export to valid JSON

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements affecting multiple user stories

- [ ] T059 [P] Add rustdoc comments to all public API functions in src/oap/mod.rs
- [ ] T060 [P] Document exported functions in library-api.md format in src/oap/
- [ ] T061 Run cargo clippy and fix all warnings
- [ ] T062 Run cargo fmt to ensure consistent formatting
- [ ] T063 [P] Add compatibility test comparing prices to pcon.basket reference in tests/
- [ ] T064 Verify SC-006: no manufacturer-specific code branches (code review)
- [ ] T065 Run quickstart.md examples and verify they work
- [ ] T066 [P] Update docs/OCD-PRICING-IMPLEMENTATION.md with new patterns

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup) → Phase 2 (Foundational) → [User Stories can run in parallel]
                                          ├── Phase 3 (US1 - Browse/Pricing) 🎯 MVP
                                          ├── Phase 4 (US2 - Recalculation)
                                          ├── Phase 5 (US3 - Properties)
                                          ├── Phase 6 (US4 - Manufacturers)
                                          └── Phase 7 (US5 - Export)

All User Stories → Phase 8 (Polish)
```

### User Story Dependencies

| Story | Depends On | Can Start After |
|-------|------------|-----------------|
| US1 (P1) | Foundational | Phase 2 complete |
| US2 (P1) | US1 (base price needed for surcharges) | T020 complete |
| US3 (P2) | Foundational | Phase 2 complete |
| US4 (P2) | Foundational | Phase 2 complete |
| US5 (P3) | US1, US2 (needs working config + pricing) | Phase 4 complete |

### Within Each User Story

1. Core logic before TUI integration
2. Data structures before algorithms
3. Library functions before TUI views
4. Logging/warnings alongside implementation

### Parallel Opportunities

**Phase 2 (Foundational)**: T005, T006, T008, T011, T012 can run in parallel

**Phase 3 (US1)**: T015, T017, T021 can run in parallel

**Phase 4 (US2)**: T024, T025, T028 can run in parallel

**Phase 5 (US3)**: T034, T035, T041 can run in parallel

**Phase 6 (US4)**: T043, T045, T049 can run in parallel

**Phase 7 (US5)**: T052, T058 can run in parallel

**Cross-Story Parallelism**: US3 and US4 can run in parallel after Foundational

---

## Parallel Execution Examples

### Example 1: Foundational Phase

```bash
# Run in parallel (different files, no dependencies):
Task: T005 - Implement DataWarning struct in src/oap/ocd.rs
Task: T006 - Add data normalization in src/oap/ocd.rs
Task: T010 - Create Price struct in src/oap/price.rs
Task: T011 - Create Surcharge struct in src/oap/price.rs
Task: T012 - Create PriceBreakdown struct in src/oap/price.rs
```

### Example 2: User Story 2 Var_Cond Matching

```bash
# Run in parallel (different matching strategies):
Task: T024 - Direct match strategy in src/oap/variant.rs
Task: T025 - Prefix match strategy in src/oap/variant.rs
```

### Example 3: Cross-Story Parallelism

```bash
# After Foundational complete, start in parallel:
Developer A: Phase 3 (US1) - Browse/Pricing MVP
Developer B: Phase 5 (US3) - Properties display
Developer C: Phase 6 (US4) - Manufacturer loading
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T012)
3. Complete Phase 3: User Story 1 (T013-T022)
4. **STOP and VALIDATE**: Test with multiple manufacturers
5. Demo if ready - basic pricing works universally

### Incremental Delivery

1. Setup + Foundational → Infrastructure ready
2. Add US1 → Test → **MVP: Universal base pricing**
3. Add US2 → Test → Real-time surcharge calculation
4. Add US3 → Test → Property display complete
5. Add US4 → Test → All manufacturers load
6. Add US5 → Test → JSON export works
7. Polish → Full feature complete

### Success Criteria Mapping

| Success Criteria | Verified By Task |
|------------------|------------------|
| SC-001: 100% manufacturer load | T050 |
| SC-002: Price accuracy | T063 |
| SC-003: <100ms recalculation | T030 |
| SC-004: TUI correct prices | T020, T032 |
| SC-005: Corruption recovery | T018 |
| SC-006: No mfr-specific code | T064 |
| SC-007: Property options match | T040 |
| SC-008: Surcharges apply/remove | T027, T031 |

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- All code must pass `cargo test`, `cargo clippy`, `cargo fmt --check`
