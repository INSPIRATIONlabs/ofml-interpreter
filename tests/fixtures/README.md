# Test Fixtures

This directory contains real OFML data from multiple manufacturers for testing the interpreter.

## Directory Structure

```
fixtures/
├── bordbar/basic/        # Bordbar Basic product
│   └── bordbar_basic_1.alb
├── cassina/abc/          # Cassina ABC product
│   ├── abc.ebase
│   ├── cassina_abc_1.alb
│   └── odb.ebase
├── kn/conline/           # König+Neurath Conline product
│   ├── conline.ebase
│   ├── kn_conline_2.alb
│   └── odb.ebase
└── vitra/workit/         # Vitra Workit product
    ├── odb.ebase
    └── vitra_workit_1.alb
```

## Manufacturers

- **bordbar**: Mobile bar furniture systems
- **cassina**: Italian design furniture (LC collection, etc.)
- **kn**: König+Neurath office furniture (Germany)
- **vitra**: Swiss office furniture (Workit, etc.)

## Data Format

Each product directory may contain:

- `*.alb` - OFML Archive (encrypted ZIP with geometry, CLS, textures)
- `odb.ebase` - Object Database (article configuration)
- `*.ebase` - Product-specific EBASE tables
- `ofml.ebase` - OFML metadata

## Usage in Tests

```rust
use std::path::PathBuf;

fn fixture_path(manufacturer: &str, product: &str, filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(manufacturer)
        .join(product)
        .join(filename)
}

#[test]
fn test_with_fixture() {
    let alb_path = fixture_path("vitra", "workit", "vitra_workit_1.alb");
    // ... test code
}
```

## Source

Test data copied from `/workspace/ofmldata/` which contains complete OFML catalogs.
