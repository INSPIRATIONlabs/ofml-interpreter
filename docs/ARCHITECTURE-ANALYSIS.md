# OAP Configurator - Architecture Analysis & Specification

## Executive Summary

After comprehensive research of the OFML specification (ofml_20r3-en.md) and current implementation, I've identified the **fundamental architectural gap**: The current TUI shows articles from OCD (commercial data) but doesn't connect to the CLS interpreter, which is the core of OFML's configuration system.

**Current State:** Article list → Static properties → No pricing
**Required State:** Article → OAM mapping → CLS class → Dynamic properties → Price calculation

---

## 1. OFML Architecture Overview

Based on the OFML 20r3 specification, the complete system has these layers:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Planning Environment                         │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                     OiPlanning                              │  │
│  │  - Root of planning hierarchy                              │  │
│  │  - Manages OiPDManager (product data manager)              │  │
│  │  - Handles collision detection, planning limits            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                    │
│                              ▼                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    OiPDManager                              │  │
│  │  - Manages multiple OiProductDB instances                  │  │
│  │  - article2Class() - maps article to CLS class             │  │
│  │  - class2Article() - maps class to article number          │  │
│  │  - setupProps() - initializes properties from product DB   │  │
│  │  - getArticlePrice() - price lookup                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                    │
│              ┌───────────────┼───────────────┐                   │
│              ▼               ▼               ▼                   │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐    │
│  │   OiProductDB   │ │   OiProductDB   │ │   OiProductDB   │    │
│  │   (OCD/OAM)     │ │   (SAP R/3)     │ │   (Custom)      │    │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘    │
│                              │                                    │
│                              ▼                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    OiPlElement                              │  │
│  │  - A configurable planning element (furniture)             │  │
│  │  - Has properties that can be edited                       │  │
│  │  - Properties linked to Article interface                  │  │
│  │  - getArticleSpec() → variant code                         │  │
│  │  - getArticlePrice() → price from PDManager                │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Data Flow Analysis

### 2.1 Current Implementation (Incomplete)

```
ofmldata/
└── {manufacturer}/
    └── {series}/
        └── {region}/
            └── {version}/
                └── db/
                    └── pdata.ebase        ← OCD data (articles, prices, texts)
                        ├── ocd_article    ← List of articles
                        ├── ocd_price      ← Pricing table
                        └── ocd_artshorttext ← Descriptions
```

**Current TUI Flow:**
1. Load manufacturers from `install.db` ✓
2. Load articles from `pdata.ebase` (ocd_article) ✓
3. Display article list with descriptions ✓
4. **STOPS HERE** - No property loading, no CLS integration, no pricing

### 2.2 Required Implementation

```
ofmldata/
└── {manufacturer}/
    └── {series}/
        └── {region}/
            └── {version}/
                ├── db/
                │   └── pdata.ebase        ← OCD data
                │       ├── ocd_article    ← Articles
                │       ├── ocd_price      ← Prices with var_cond
                │       ├── ocd_property   ← Property definitions ← NOT USED
                │       └── ocd_propertyvalue ← Property options ← NOT USED
                │
                ├── oam/
                │   └── oam.ebase          ← Article mappings ← NOT USED
                │       └── oam_article2ofml ← Article → CLS class mapping
                │
                └── lib/
                    └── *.cls              ← CLS class definitions ← NOT CONNECTED
                        └── setupProperty() calls define properties
```

**Required TUI Flow:**
1. Load manufacturers from filesystem (done)
2. Load articles from `pdata.ebase` (done)
3. **User selects article**
4. **NEW:** Look up CLS class via `oam_article2ofml`
5. **NEW:** Load and parse CLS class definition
6. **NEW:** Execute class initialization to extract properties
7. Display property editor with valid options
8. User changes property → triggers price lookup
9. **NEW:** Match variant code against `ocd_price.var_cond`
10. Display price result

---

## 3. Key Discovery: OAM Article-to-Class Mapping

The `oam.ebase` file contains the crucial `oam_article2ofml` table:

```
Record example from /workspace/ofmldata/vitra/abc/DE/1/oam/oam.ebase:

Record 0:
  article: '89224052'
  ofml_type: '::vitra::abc::aAddOn'
  odb_name: '::vitra::abc::papierablage'
  params: ''
```

This mapping is essential:
- `article` → OCD article number
- `ofml_type` → CLS class that handles configuration
- `odb_name` → ODB name for geometry
- `params` → initialization parameters

**Current Status:** This data is NOT being used.

---

## 4. Missing Components

### 4.1 OAM Reader

**Location needed:** `src/oap/oam.rs`

Must read the `oam_article2ofml` table to map articles to CLS classes.

### 4.2 CLS Class → Property Integration

**Problem:** Properties are defined INSIDE CLS classes via `setupProperty()` calls.

Example from a CLS file:
```
initialize(...) {
    setupProperty(@WIDTH, ["Width", "Breite", @INT, [600, 2400], 1200]);
    setupProperty(@HEIGHT, ["Height", "Höhe", @INT, [650, 800], 720]);
    setupProperty(@COLOR, ["Color", "Farbe", @CHOICE, ["white", "black"], "white"]);
}
```

**Required:**
1. Parse CLS file
2. Find `setupProperty()` calls
3. Extract: key, labels, type, constraints, default
4. Create PropertyDef instances
5. Display in TUI

**Current Status:**
- CLS parser exists ✓
- CLS interpreter exists ✓
- Property extraction NOT connected to TUI ✗

### 4.3 Price Calculation with Variant Codes

**Problem:** Prices in `ocd_price` have a `var_cond` field:

```
ocd_price records:
  article_nr: "VT:TABLE:001"
  var_cond: ""              ← Base price
  price: 500.00

  article_nr: "VT:TABLE:001"
  var_cond: "WIDTH=1800"    ← Surcharge condition
  price: 75.00
```

**Required:**
1. Get current property values
2. Generate variant code
3. Match against `var_cond` fields
4. Sum base + matching surcharges

**Current Status:** Price lookup doesn't match `var_cond`

### 4.4 Property Dependencies

OFML supports property dependencies:
- Changing one property can hide/show others
- Changing one property can change valid options for others

**Current Status:**
- PropertyManager has callback support ✓
- CLS interpreter has rule execution ✓
- NOT connected to TUI property changes ✗

---

## 5. Proposed Implementation Phases

### Phase 1: OAM Integration

1. Create `src/oap/oam.rs`
2. Read `oam_article2ofml` table
3. Enrich article display with OAM info
4. CLI: `ofml oam-lookup <manufacturer> <article_nr>`

### Phase 2: Property Extraction from CLS

1. Create `src/oap/property_extractor.rs`
2. Parse CLS files for `setupProperty()` calls
3. Extract PropertyDef from arguments
4. CLI: `ofml cls-properties <manufacturer> <class_name>`

### Phase 3: Price Calculation Fix

1. Update `src/oap/price.rs`
2. Parse `var_cond` syntax
3. Match against current property values
4. CLI: `ofml price <article> --prop KEY=VALUE`

### Phase 4: TUI Integration

1. Connect phases 1-3 to TUI
2. When article selected → load OAM → load properties
3. Property changes → update price
4. Show price breakdown

---

## 6. Current Gaps Summary

| Component | Status | Priority |
|-----------|--------|----------|
| OAM article2ofml reading | Not implemented | HIGH |
| CLS property extraction | Not implemented | HIGH |
| var_cond price matching | Not implemented | HIGH |
| Property dependencies | Partial | MEDIUM |
| Geometry preview | Interpreter exists | LOW |

---

## 7. Key Insight

**The CLS interpreter is NOT optional** - it's the core of OFML's configuration system.

The current implementation treats OFML data as a simple database of articles and prices. But OFML is actually a **programming language** where:

1. Products are **classes** with behavior
2. Properties are defined by **code** (setupProperty calls)
3. Property changes trigger **rules** that update other properties
4. Geometry is generated **dynamically** based on properties
5. Prices are calculated using **expressions** that evaluate properties

The TUI needs to be connected to the CLS interpreter to properly configure articles.

---

## 8. Questions for Discussion

1. **How deep should CLS integration go?**
   - Option A: Extract properties statically (parse only)
   - Option B: Execute CLS initialization (full interpreter)

2. **What about articles without OAM mappings?**
   - Some articles may not have CLS classes
   - Fall back to OCD property tables?

3. **Performance considerations?**
   - CLS parsing per article vs. caching
   - In-memory vs. persistent cache

4. **Geometry preview?**
   - The interpreter can generate GLB
   - Show in TUI? External viewer?
