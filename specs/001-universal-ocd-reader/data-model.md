# Data Model: Universal OCD Data Reader

**Feature**: 001-universal-ocd-reader
**Date**: 2026-01-02

## Entity Relationship Diagram

```
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│  Manufacturer   │1─────*│     Series      │1─────*│     Article     │
│                 │       │   (Family)      │       │                 │
│  - id: String   │       │  - id: String   │       │ - article_nr    │
│  - path: Path   │       │  - name: String │       │ - property_     │
│                 │       │  - mfr_id: FK   │       │   classes: Vec  │
└─────────────────┘       │  - pdata_path   │       └────────┬────────┘
                          └─────────────────┘                │
                                                             │1
                                                             │
                                                             *
┌─────────────────┐       ┌─────────────────┐       ┌────────┴────────┐
│  PropertyClass  │1─────*│    Property     │1─────*│  PropertyValue  │
│                 │       │                 │       │                 │
│ - class_id      │       │ - property_id   │       │ - value_id      │
│ - name: String  │       │ - name: String  │       │ - label: String │
│                 │       │ - label: String │       │ - var_cond      │
└─────────────────┘       │ - class_id: FK  │       │ - property_id   │
                          └─────────────────┘       └─────────────────┘

┌─────────────────┐       ┌─────────────────┐
│   PriceRecord   │*─────1│   PriceText     │
│                 │       │                 │
│ - article_nr    │       │ - text_id       │
│ - var_cond      │       │ - language      │
│ - price_type    │       │ - description   │
│ - price_level   │       └─────────────────┘
│ - price: f32    │
│ - currency      │
│ - date_from     │
│ - date_to       │
│ - is_fix: bool  │
│ - text_id: FK   │
└─────────────────┘

┌─────────────────┐
│  Configuration  │ (runtime state)
│                 │
│ - article_nr    │
│ - selections:   │
│   Map<prop_id,  │
│       value_id> │
│ - calculated_   │
│   price: Price  │
└─────────────────┘

┌─────────────────┐
│     Price       │ (calculated)
│                 │
│ - base: f32     │
│ - surcharges:   │
│   Vec<Surcharge>│
│ - discounts:    │
│   Vec<Discount> │
│ - total: f32    │
│ - currency      │
└─────────────────┘
```

## Entities

### Manufacturer

Represents a furniture brand with data in the OFML directory.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | String | Yes | Short identifier (e.g., "sedus", "framery") |
| path | PathBuf | Yes | Absolute path to manufacturer directory |
| series | Vec\<Series\> | Derived | Discovered series from directory scan |

**Identity**: Unique by `id` (directory name)

### Series (Family)

A product line containing related articles.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | String | Yes | Series identifier (e.g., "ai", "frmr_one_compact") |
| name | String | Yes | Display name from ocd_family or derived |
| manufacturer_id | String | Yes | FK to Manufacturer |
| pdata_path | PathBuf | Yes | Path to pdata.ebase file |
| base_article_nr | String | Yes | Default article for the family |

**Identity**: Unique by `(manufacturer_id, id)`

### Article

A configurable product.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| article_nr | String | Yes | Unique article number within series |
| property_classes | Vec\<String\> | Yes | Associated property class IDs |
| description | String | No | Article description from ocd_article |

**Identity**: Unique by `article_nr` within series context

### PropertyClass

Groups related properties.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| class_id | String | Yes | Property class identifier |
| name | String | Yes | Display name |
| properties | Vec\<Property\> | Derived | Properties in this class |

**Identity**: Unique by `class_id`

### Property

A configurable attribute.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| property_id | String | Yes | Property identifier (e.g., "M_FRAME") |
| name | String | Yes | Internal name |
| label | String | Yes | Display label (from ocd_propertytext) |
| class_id | String | Yes | FK to PropertyClass |
| default_value | String | No | Default value ID |

**Identity**: Unique by `property_id` within class

### PropertyValue

A valid option for a property.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| value_id | String | Yes | Value identifier |
| label | String | Yes | Display label (from ocd_propertyvaluetext) |
| property_id | String | Yes | FK to Property |
| var_cond | Option\<String\> | No | Inferred variant condition for surcharge lookup |

**Identity**: Unique by `(property_id, value_id)`

### PriceRecord

A pricing entry from ocd_price table.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| article_nr | String | Yes | Article number or "*" for wildcard |
| var_cond | String | Yes | Variant condition (empty for base prices) |
| price_type | String | Yes | "S" (Sales) or "P" (Purchase) |
| price_level | String | Yes | "B" (Base), "X" (Surcharge), "D" (Discount) |
| price | f32 | Yes | Price amount |
| currency | String | Yes | ISO 4217 code (default: "EUR") |
| date_from | String | Yes | Valid from date (YYYYMMDD) |
| date_to | String | Yes | Valid to date (YYYYMMDD) |
| is_fix | bool | Yes | true = fixed amount, false = percentage |
| text_id | String | No | FK to PriceText |
| scale_qty | u32 | No | Minimum quantity for scaled pricing |

**Identity**: Unique by `(article_nr, var_cond, price_level, date_from)`

**Validation Rules**:
- `price_level` must be one of: "B", "X", "D"
- `price_type` must be one of: "S", "P"
- `currency` must be 3 characters (ISO 4217)
- If `price_level == "B"`, then `is_fix` must be true

### PriceText

Multilingual descriptions for price entries.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| text_id | String | Yes | Text identifier |
| language | String | Yes | Language code (e.g., "DE", "EN") |
| description | String | Yes | Human-readable description |

**Identity**: Unique by `(text_id, language)`

### Configuration (Runtime)

Current configuration state for an article.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| article_nr | String | Yes | Currently configured article |
| series_id | String | Yes | Parent series |
| manufacturer_id | String | Yes | Parent manufacturer |
| selections | HashMap\<String, String\> | Yes | property_id → value_id |
| calculated_price | Price | Derived | Calculated price with breakdown |
| warnings | Vec\<DataWarning\> | Derived | Any data issues encountered |

### Price (Calculated)

Calculated price with breakdown.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| base | f32 | Yes | Base price amount |
| surcharges | Vec\<Surcharge\> | Yes | Applied surcharges |
| discounts | Vec\<Discount\> | Yes | Applied discounts |
| total | f32 | Yes | Final calculated total |
| currency | String | Yes | Currency code |

### Surcharge

An applied surcharge.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| var_cond | String | Yes | Matched variant condition |
| description | String | No | From PriceText |
| amount | f32 | Yes | Surcharge amount |
| is_percentage | bool | Yes | true if percentage of base |

### DataWarning

A recoverable data issue.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| severity | WarningSeverity | Yes | Info, Warning, Error |
| code | String | Yes | Warning code (e.g., "CORRUPTED_RECORD") |
| message | String | Yes | Human-readable description |
| source | String | No | File/record that caused the warning |

## State Transitions

### Configuration Lifecycle

```
                    ┌──────────────┐
                    │   Created    │
                    │ (no article) │
                    └──────┬───────┘
                           │ select_article()
                           ▼
                    ┌──────────────┐
              ┌────>│  Configured  │<────┐
              │     │ (has prices) │     │
              │     └──────┬───────┘     │
              │            │             │
    change_property()      │      change_article()
              │            │             │
              │            ▼             │
              │     ┌──────────────┐     │
              └─────│ Recalculating│─────┘
                    │   (<100ms)   │
                    └──────────────┘
```

### Price Calculation Flow

```
1. Load base price (price_level='B')
   ├── Exact article match first
   └── Fallback to wildcard (article_nr='*')

2. For each selected property value:
   ├── Infer var_cond from value
   ├── Find matching surcharge (price_level='X')
   └── Add to surcharges list

3. Apply discounts (price_level='D')
   └── Subtract from total

4. Return Price with breakdown
```

## Data Normalization Rules

All string fields from EBase are normalized on read:

1. **Trim whitespace**: `"B  "` → `"B"`
2. **Uppercase price_level**: `"b"` → `"B"`
3. **Uppercase var_cond**: `"pg_table"` → `"PG_TABLE"`
4. **Default currency**: Empty/malformed → `"EUR"`
5. **Default dates**: Missing date_from → `"19000101"`, date_to → `"99991231"`
