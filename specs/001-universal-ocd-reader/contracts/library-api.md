# Library API Contract: Universal OCD Data Reader

**Module**: `ofml_interpreter::oap`
**Version**: 0.1.0

## Public API

### Manufacturer Discovery

```rust
/// Discover all manufacturers in an OFML data directory
///
/// # Arguments
/// * `ofml_data_path` - Path to the OFML data directory (e.g., /reference/ofmldata)
///
/// # Returns
/// * `Vec<Manufacturer>` - List of discovered manufacturers
///
/// # Errors
/// * Returns empty vec if directory doesn't exist or is empty
pub fn discover_manufacturers(ofml_data_path: &Path) -> Vec<Manufacturer>;

/// Load a specific manufacturer by ID
///
/// # Arguments
/// * `ofml_data_path` - Path to the OFML data directory
/// * `manufacturer_id` - Manufacturer identifier (directory name)
///
/// # Returns
/// * `Option<Manufacturer>` - The manufacturer if found
pub fn load_manufacturer(ofml_data_path: &Path, manufacturer_id: &str) -> Option<Manufacturer>;
```

### Series/Family Operations

```rust
/// List all series for a manufacturer
///
/// # Arguments
/// * `manufacturer` - Reference to loaded manufacturer
///
/// # Returns
/// * `Vec<Series>` - All series with valid pdata.ebase files
pub fn list_series(manufacturer: &Manufacturer) -> Vec<Series>;

/// Load a series with full article and property data
///
/// # Arguments
/// * `manufacturer` - Reference to manufacturer
/// * `series_id` - Series identifier
///
/// # Returns
/// * `Option<LoadedSeries>` - Series with parsed OCD data
pub fn load_series(manufacturer: &Manufacturer, series_id: &str) -> Option<LoadedSeries>;
```

### Article Configuration

```rust
/// Create a new configuration for an article
///
/// # Arguments
/// * `series` - Reference to loaded series
/// * `article_nr` - Article number to configure
///
/// # Returns
/// * `Result<Configuration, ConfigError>` - New configuration with default values
pub fn create_configuration(
    series: &LoadedSeries,
    article_nr: &str
) -> Result<Configuration, ConfigError>;

/// Get all configurable properties for an article
///
/// # Arguments
/// * `config` - Current configuration
///
/// # Returns
/// * `Vec<PropertyWithValues>` - All properties with their valid values
pub fn get_properties(config: &Configuration) -> Vec<PropertyWithValues>;

/// Update a property value in the configuration
///
/// # Arguments
/// * `config` - Mutable configuration
/// * `property_id` - Property to update
/// * `value_id` - New value
///
/// # Returns
/// * `Result<(), ConfigError>` - Error if property/value invalid
///
/// # Side Effects
/// * Triggers price recalculation (<100ms)
pub fn set_property(
    config: &mut Configuration,
    property_id: &str,
    value_id: &str
) -> Result<(), ConfigError>;
```

### Price Calculation

```rust
/// Calculate the current price for a configuration
///
/// # Arguments
/// * `config` - Current configuration
///
/// # Returns
/// * `Price` - Calculated price with breakdown
///
/// # Performance
/// * MUST complete in <100ms
pub fn calculate_price(config: &Configuration) -> Price;

/// Get price breakdown details
///
/// # Arguments
/// * `price` - Calculated price
///
/// # Returns
/// * `PriceBreakdown` - Detailed breakdown with base, surcharges, discounts
pub fn get_price_breakdown(price: &Price) -> PriceBreakdown;
```

### Export

```rust
/// Export configuration to JSON
///
/// # Arguments
/// * `config` - Configuration to export
///
/// # Returns
/// * `String` - JSON representation
pub fn export_json(config: &Configuration) -> String;

/// Export multiple configurations to JSON array
///
/// # Arguments
/// * `configs` - Configurations to export
///
/// # Returns
/// * `String` - JSON array representation
pub fn export_json_batch(configs: &[Configuration]) -> String;
```

### Warnings/Diagnostics

```rust
/// Get any data warnings encountered during loading/calculation
///
/// # Arguments
/// * `config` - Configuration
///
/// # Returns
/// * `Vec<DataWarning>` - List of non-fatal issues
pub fn get_warnings(config: &Configuration) -> Vec<DataWarning>;

/// Check if configuration has any warnings
///
/// # Arguments
/// * `config` - Configuration
///
/// # Returns
/// * `bool` - true if warnings present
pub fn has_warnings(config: &Configuration) -> bool;
```

## Data Types

### Manufacturer

```rust
pub struct Manufacturer {
    pub id: String,
    pub path: PathBuf,
}
```

### Series

```rust
pub struct Series {
    pub id: String,
    pub name: String,
    pub manufacturer_id: String,
    pub pdata_path: PathBuf,
}
```

### LoadedSeries

```rust
pub struct LoadedSeries {
    pub series: Series,
    pub articles: Vec<Article>,
    pub property_classes: Vec<PropertyClass>,
    pub prices: Vec<PriceRecord>,
}
```

### Configuration

```rust
pub struct Configuration {
    pub article_nr: String,
    pub series_id: String,
    pub manufacturer_id: String,
    pub selections: HashMap<String, String>,  // property_id -> value_id
    pub(crate) series_data: Arc<LoadedSeries>,
    pub(crate) cached_price: Option<Price>,
    pub(crate) warnings: Vec<DataWarning>,
}
```

### Price

```rust
pub struct Price {
    pub base: f32,
    pub surcharges: Vec<Surcharge>,
    pub discounts: Vec<Discount>,
    pub total: f32,
    pub currency: String,
}

pub struct Surcharge {
    pub var_cond: String,
    pub description: Option<String>,
    pub amount: f32,
    pub is_percentage: bool,
}

pub struct Discount {
    pub var_cond: String,
    pub description: Option<String>,
    pub amount: f32,
    pub rule: DiscountRule,  // PercentOfBase, PercentOfAccumulated
}

pub struct PriceBreakdown {
    pub base_label: String,
    pub base_amount: f32,
    pub surcharge_lines: Vec<(String, f32)>,  // (description, amount)
    pub discount_lines: Vec<(String, f32)>,
    pub subtotal: f32,
    pub total: f32,
    pub currency: String,
}
```

### PropertyWithValues

```rust
pub struct PropertyWithValues {
    pub property: Property,
    pub values: Vec<PropertyValue>,
    pub selected_value: Option<String>,
}

pub struct Property {
    pub property_id: String,
    pub label: String,
    pub class_id: String,
}

pub struct PropertyValue {
    pub value_id: String,
    pub label: String,
    pub has_surcharge: bool,
}
```

### DataWarning

```rust
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

pub struct DataWarning {
    pub severity: WarningSeverity,
    pub code: String,
    pub message: String,
    pub source: Option<String>,
}
```

### ConfigError

```rust
pub enum ConfigError {
    ArticleNotFound(String),
    PropertyNotFound(String),
    InvalidValue { property: String, value: String },
    PriceCalculationFailed(String),
    DataCorruption(String),
}
```

## Error Handling

| Error | Handling |
|-------|----------|
| File not found | Return None/empty, log warning |
| Parse error | Skip record, add DataWarning |
| Missing base price | Use wildcard or 0, add DataWarning |
| Corrupted record | Attempt recovery, add DataWarning |
| Invalid property/value | Return ConfigError |

## Performance Requirements

| Operation | Max Latency |
|-----------|-------------|
| `discover_manufacturers` | 1s (directory scan) |
| `load_series` | 500ms (EBase parsing) |
| `calculate_price` | 100ms |
| `set_property` | 100ms (includes recalculation) |
| `export_json` | 50ms |
