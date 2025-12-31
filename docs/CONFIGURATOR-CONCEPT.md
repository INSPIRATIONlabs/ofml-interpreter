# OAP Configurator - Correct Architecture Concept

## Key Discovery

The current implementation is **fundamentally wrong** in its approach:

**Current (Wrong):** Shows individual articles as separate products
**Correct:** Shows **product families** with **configurable properties/options**

---

## 1. Understanding OFML Product Structure

### 1.1 What the "Numbered Articles" Actually Are

The articles in OCD (like "89224052") are **NOT** individual products to select from. They are:

1. **Base Article Numbers** - The starting point for a product family
2. **Variant SKUs** - Pre-configured combinations that map to specific prices
3. **Internal References** - System articles for planning (starting with @)

### 1.2 The Correct Product Hierarchy

```
Manufacturer (vitra)
  └── Product Family (e.g., "Eames Chair")
      ├── Base Article: "21054600"
      ├── Properties:
      │   ├── Shell Color: [white, black, red, blue]
      │   ├── Base Type: [wood, chrome, wire]
      │   ├── Upholstery: [none, leather, fabric]
      │   └── Arm Rests: [with, without]
      └── Price = Base + Surcharges per option
```

### 1.3 OCD Property Tables (Currently Unused!)

The OCD tables contain **property definitions and values** that we're ignoring:

| Table | Records (classic) | Purpose |
|-------|-------------------|---------|
| `ocd_property` | 1,667 | Property definitions |
| `ocd_propertyvalue` | 11,628 | Property options |
| `ocd_propertyclass` | 189 | Property groupings |
| `ocd_propertytext` | 1,278 | Property labels |

**This is the configuration data we need!**

---

## 2. OCD Property Structure

### 2.1 ocd_property Table

```
prop_class     : "SHELL"              # Property group
property       : "COLOR"              # Property name
pos_prop       : 1                    # Display order
prop_textnr    : "TX001"              # → "Shell Color" label
prop_type      : "CHOICE"             # Type: CHOICE, RANGE, INT, etc.
need_input     : 1                    # Required?
multi_option   : 0                    # Allow multiple selections?
```

### 2.2 ocd_propertyvalue Table

```
prop_class     : "SHELL"
property       : "COLOR"
pos_pval       : 1                    # Option order
pval_textnr    : "TX002"              # → "White" label
value_from     : "WHITE"              # Value code
is_default     : 1                    # Default selection?
```

### 2.3 Linking Properties to Articles

Articles link to property classes via:
- `ocd_article2propgroup` - Maps article to property groups
- `ocd_artbase` - Base property values for article
- Article `prop_class` field - Direct property class reference

---

## 3. Correct Configurator Flow

### 3.1 Product Family View (NOT Article List)

```
┌─────────────────────────────────────────────────────────────┐
│  VITRA - Produktfamilien                                    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ▸ Eames Plastic Chair                    [12 Varianten]    │
│    Klassischer Schalenstuhl                                 │
│                                                              │
│  ▸ Eames Lounge Chair                     [8 Varianten]     │
│    Loungestuhl mit Ottoman                                  │
│                                                              │
│  ▸ Vitra Allstar                          [24 Varianten]    │
│    Bürodrehstuhl                                            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Configuration View

```
┌─────────────────────────────────────────────────────────────┐
│  Eames Plastic Chair - Konfiguration                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Schale                                                      │
│  ├── Farbe:     [▼ Weiß        ]  ← Dropdown with options   │
│  └── Finish:    [▼ Matt        ]                            │
│                                                              │
│  Untergestell                                                │
│  ├── Typ:       [▼ Holz (Esche)]                            │
│  └── Farbe:     [▼ Natur       ]                            │
│                                                              │
│  Polsterung                                                  │
│  ├── Art:       [▼ Sitzpolster ]                            │
│  └── Stoff:     [▼ Hopsak      ]                            │
│                                                              │
│  ─────────────────────────────────────────────────────────  │
│                                                              │
│  Grundpreis:              450,00 EUR                         │
│  + Holzgestell:           +80,00 EUR                         │
│  + Sitzpolster Hopsak:    +95,00 EUR                         │
│  ─────────────────────────────────────────────────────────  │
│  Gesamtpreis:             625,00 EUR                         │
│                                                              │
│  Artikelnummer:  DSW-WHT-ASH-HOP                            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. Data Loading Strategy

### 4.1 Step 1: Load Product Families

```rust
// Group articles by base product/series
struct ProductFamily {
    id: String,                    // Base article or series ID
    name: String,                  // Display name from text
    description: String,           // Long description
    base_article_nr: String,       // Base SKU
    property_class: String,        // Links to ocd_property
    variant_count: usize,          // Number of possible variants
}

fn load_product_families(mfr_path: &Path) -> Vec<ProductFamily> {
    // 1. Load ocd_propertyclass to find property groups
    // 2. Load ocd_article and group by series/prop_class
    // 3. Create one ProductFamily per group
}
```

### 4.2 Step 2: Load Properties for Family

```rust
struct PropertyDefinition {
    key: String,                   // Property identifier
    label: String,                 // Display label
    group: String,                 // Property group (for UI sections)
    prop_type: PropertyType,       // CHOICE, RANGE, INT, BOOL
    required: bool,
    options: Vec<PropertyOption>,  // Available values
    default_value: String,
}

struct PropertyOption {
    value: String,                 // Value code
    label: String,                 // Display label
    is_default: bool,
    surcharge: Option<Decimal>,    // Price impact
}

fn load_properties_for_family(reader: &OcdReader, prop_class: &str)
    -> Vec<PropertyDefinition>
{
    // 1. Read ocd_property WHERE prop_class = prop_class
    // 2. For each property, read ocd_propertyvalue
    // 3. Look up labels from ocd_propertytext
    // 4. Return structured property definitions
}
```

### 4.3 Step 3: Calculate Price from Selection

```rust
fn calculate_price(
    base_article: &str,
    selections: &HashMap<String, String>,
    reader: &OcdReader
) -> PriceResult {
    // 1. Find base price (var_cond = "")
    // 2. Generate variant code from selections
    // 3. Find matching surcharges from ocd_price
    // 4. Sum and return
}
```

---

## 5. The Two Configuration Sources

### 5.1 OCD Properties (Commercial Data)

- Stored in `pdata.ebase`
- Tables: `ocd_property`, `ocd_propertyvalue`, `ocd_propertytext`
- Defines: What options are available and their prices
- Use when: Article has OCD property definitions

### 5.2 CLS Properties (Technical Data)

- Stored in `.alb` archives as `.cls` files
- Extracted from: `setupProperty()` calls in CLS source
- Defines: Geometry parameters and technical constraints
- Use when: Article has OAM mapping to CLS class

### 5.3 Priority

```
1. Check if article has OCD property class → Use OCD properties
2. Check if article has OAM mapping → Use CLS properties
3. Neither → Not configurable (simple article)
```

---

## 6. Implementation Plan

### Phase 1: OCD Property Reader

Create `/src/oap/ocd_properties.rs`:

```rust
pub struct OcdPropertyReader {
    properties: HashMap<(String, String), PropertyDef>,
    values: HashMap<(String, String), Vec<PropertyValue>>,
    texts: HashMap<String, String>,
}

impl OcdPropertyReader {
    pub fn from_ebase(path: &Path) -> Result<Self, Error>;
    pub fn get_properties(&self, prop_class: &str) -> Vec<PropertyDef>;
    pub fn get_values(&self, prop_class: &str, property: &str) -> Vec<PropertyValue>;
}
```

### Phase 2: Product Family Aggregation

Create `/src/oap/families.rs`:

```rust
pub struct ProductFamilyLoader {
    // Groups articles into product families
}

impl ProductFamilyLoader {
    pub fn load_families(mfr_path: &Path) -> Vec<ProductFamily>;
    pub fn get_family(&self, id: &str) -> Option<&ProductFamily>;
}
```

### Phase 3: TUI Restructure

```
Screen 1: Manufacturers      (existing, works)
Screen 2: Product Families   (NEW - replace article list)
Screen 3: Configuration      (NEW - property selection)
Screen 4: Summary/Export     (existing, extend)
```

---

## 7. Example: Vitra Classic Products

From `/workspace/ofmldata/vitra/classic/DE/1/db/pdata.ebase`:

### ocd_propertyclass (189 records)
```
"SHELL" - Shell options
"BASE" - Base/leg options
"UPHOLSTERY" - Upholstery options
"ARMREST" - Armrest options
```

### ocd_property (1,667 records)
```
prop_class: SHELL, property: COLOR, type: CHOICE
prop_class: SHELL, property: FINISH, type: CHOICE
prop_class: BASE, property: TYPE, type: CHOICE
prop_class: BASE, property: MATERIAL, type: CHOICE
...
```

### ocd_propertyvalue (11,628 records)
```
SHELL.COLOR: WHITE, BLACK, RED, BLUE, GREEN, ...
SHELL.FINISH: GLOSS, MATT, TEXTURED
BASE.TYPE: WOOD, CHROME, WIRE, SLED
BASE.MATERIAL: ASH, OAK, MAPLE, WALNUT
...
```

---

## 8. Key Changes Required

### 8.1 Remove Article List Display

The current article list showing 847 individual items is wrong. Users don't want to scroll through hundreds of SKU numbers.

### 8.2 Add Product Family Grouping

Group articles by:
- Series (from `ocd_article.series`)
- Property class (from property linkage)
- Base article (first characters of article_nr)

### 8.3 Add Property Selection UI

For each product family:
1. Load properties from OCD or CLS
2. Display as form with dropdowns/sliders
3. Update price on each change
4. Show final configuration

### 8.4 Generate Article Number

From selections, generate the variant article number:
```
Base: "21054"
+ Shell White: "600"
+ Base Wood Ash: "01"
= "21054600-01"
```

---

## 9. Summary

| Aspect | Current (Wrong) | Correct |
|--------|-----------------|---------|
| Display | Individual articles | Product families |
| Selection | Pick one from list | Configure options |
| Properties | Not shown | Dropdowns/form fields |
| Options | None | From OCD or CLS |
| Price | Static lookup | Base + surcharges |
| Output | Article number | Config + variant code |

The key insight is: **OFML is a configuration system, not a product catalog.**

Users should:
1. Select a product family (e.g., "Eames Chair")
2. Configure options (color, base, upholstery)
3. See the price update in real-time
4. Get the final variant code/article number

NOT:
1. Scroll through 847 article numbers
2. Pick one
3. Hope it's what they wanted
