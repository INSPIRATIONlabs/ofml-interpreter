# Quickstart: Universal OCD Data Reader

## Prerequisites

- Rust 2021 Edition
- OFML data directory (e.g., `/reference/ofmldata`)

## Build

```bash
# Build library + TUI
cargo build --features tui --release

# Build library only
cargo build --release
```

## Run TUI

```bash
./target/release/ofml-interpreter tui /reference/ofmldata
```

### TUI Navigation

| Key | Action |
|-----|--------|
| ↑/↓ | Navigate lists |
| Enter | Select item |
| Esc | Go back |
| q | Quit |
| ? | Show help |

## Library Usage

### Basic Example

```rust
use ofml_interpreter::oap::{
    discover_manufacturers,
    load_series,
    create_configuration,
    set_property,
    calculate_price,
    export_json,
};
use std::path::Path;

fn main() {
    let ofml_path = Path::new("/reference/ofmldata");

    // 1. Discover manufacturers
    let manufacturers = discover_manufacturers(ofml_path);
    println!("Found {} manufacturers", manufacturers.len());

    // 2. Load a series
    let sedus = manufacturers.iter().find(|m| m.id == "sedus").unwrap();
    let ai_series = load_series(sedus, "ai").unwrap();

    // 3. Create configuration
    let mut config = create_configuration(&ai_series, "SE:AI-100").unwrap();

    // 4. Configure options
    set_property(&mut config, "M_FRAME", "FRAME_BLACK").unwrap();
    set_property(&mut config, "M_FABRIC", "FABRIC_166").unwrap();

    // 5. Get price
    let price = calculate_price(&config);
    println!("Total: {:.2} {}", price.total, price.currency);

    // 6. Export
    let json = export_json(&config);
    println!("{}", json);
}
```

### Handling Warnings

```rust
use ofml_interpreter::oap::{get_warnings, has_warnings};

// After loading/configuring
if has_warnings(&config) {
    for warning in get_warnings(&config) {
        eprintln!("[{}] {}: {}",
            warning.severity,
            warning.code,
            warning.message
        );
    }
}
```

### Batch Export

```rust
use ofml_interpreter::oap::export_json_batch;

let configs = vec![config1, config2, config3];
let json = export_json_batch(&configs);
// Returns JSON array: [{ ... }, { ... }, { ... }]
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_sedus_pricing -- --nocapture

# Run multi-manufacturer tests
cargo test multi_manufacturer -- --nocapture
```

## Common Patterns

### Iterate All Products

```rust
for mfr in discover_manufacturers(ofml_path) {
    for series in list_series(&mfr) {
        if let Some(loaded) = load_series(&mfr, &series.id) {
            for article in &loaded.articles {
                println!("{}/{}: {}", mfr.id, series.id, article.article_nr);
            }
        }
    }
}
```

### Price Comparison

```rust
let price1 = calculate_price(&config);
set_property(&mut config, "M_FABRIC", "FABRIC_PREMIUM").unwrap();
let price2 = calculate_price(&config);

let diff = price2.total - price1.total;
println!("Upgrade cost: {:.2} EUR", diff);
```

### Check Surcharges

```rust
let price = calculate_price(&config);
for surcharge in &price.surcharges {
    println!("  {} +{:.2} EUR",
        surcharge.description.as_deref().unwrap_or(&surcharge.var_cond),
        surcharge.amount
    );
}
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Article not found" | Check article_nr matches ocd_article table |
| Price shows 0 | Check for DataWarning about missing base price |
| Wrong surcharges | Verify var_cond matching pattern |
| Slow loading | First load caches data; subsequent loads are faster |
