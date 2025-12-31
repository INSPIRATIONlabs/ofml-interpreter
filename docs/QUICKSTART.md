# OFML Interpreter Quickstart Guide

Get started with converting OFML product data to GLB in 5 minutes.

---

## Prerequisites

1. Build the interpreter:
   ```bash
   cd ofml-interpreter
   cargo build --release
   ```

2. You have OFML data in a directory structure like:
   ```
   ofmldata/
   ├── vitra/
   │   ├── ac/
   │   │   └── 1/
   │   │       ├── odb.ebase
   │   │       └── data.alb
   │   └── allstar/
   │       └── 1/
   │           └── ...
   └── gsx/
       └── ...
   ```

---

## Task 1: Convert a Single 3DS File to GLB

**Input:** `chair.3ds`
**Output:** `chair.glb`

```bash
./target/release/ofml convert chair.3ds
```

That's it! The GLB file is created next to the input file.

---

## Task 2: Explore Available Products

**Step 1:** List all manufacturers
```bash
./target/release/ofml ofml /path/to/ofmldata
```

Output:
```
OFML Data: /path/to/ofmldata
Found 28 manufacturers:
  aix (12 products)
  arper (8 products)
  buzzispace (15 products)
  vitra (45 products)
  ...
```

**Step 2:** List products for a manufacturer
```bash
./target/release/ofml ofml /path/to/ofmldata vitra
```

Output:
```
Manufacturer: vitra
Products:
  ac (version 1)
  ad (version 1)
  allstar (version 1)
  ...
```

**Step 3:** See product details
```bash
./target/release/ofml ofml /path/to/ofmldata vitra ac
```

Output:
```
Product: vitra/ac
Version: 1
Path: /path/to/ofmldata/vitra/ac/1

EBASE Tables:
  - odb3d (156 records)
  - odb2d (89 records)
  - mat (23 records)
  - attpt (45 records)

ALB Archive: data.alb
  - 67 files
  - Geometry: 23 files
  - CLS Scripts: 12 files
```

---

## Task 3: Convert a Product to GLB

**Step 1:** List available articles
```bash
./target/release/ofml product /path/to/ofmldata/vitra/ac/1
```

Output:
```
Product: /path/to/ofmldata/vitra/ac/1
Available articles:
  - AC_CHAIR
  - AC_LOUNGE
  - AC_STOOL
  - AC_TABLE
```

**Step 2:** Convert specific article
```bash
./target/release/ofml product /path/to/ofmldata/vitra/ac/1 AC_CHAIR my_chair.glb
```

Output:
```
Loading product from: /path/to/ofmldata/vitra/ac/1
Article: AC_CHAIR (15 geometry refs)
Combined scene: 15 meshes
Written: my_chair.glb (245678 bytes)
```

**Step 3:** Convert all articles at once
```bash
./target/release/ofml product /path/to/ofmldata/vitra/ac/1 all_articles.glb
```

---

## Task 4: Generate a 2D Floor Plan (SVG)

```bash
./target/release/ofml svg /path/to/ofmldata/vitra/ac/1/odb.ebase floorplan.svg
```

Output:
```
Reading odb2d from: /path/to/ofmldata/vitra/ac/1/odb.ebase
Found 89 2D records
Generated SVG: floorplan.svg (12345 bytes)
```

Open `floorplan.svg` in any browser or vector editor.

---

## Task 5: Inspect EBASE Database

**List tables:**
```bash
./target/release/ofml ebase /path/to/odb.ebase
```

**View specific table:**
```bash
./target/release/ofml ebase /path/to/odb.ebase odb3d
```

**Common tables to explore:**

| Table | What it contains |
|-------|-----------------|
| `odb3d` | 3D geometry references |
| `odb2d` | 2D floor plan shapes |
| `mat` | Material definitions |
| `article` | Product catalog |
| `attpt` | Attachment points |

---

## Task 6: Extract Files from ALB Archive

**List contents:**
```bash
./target/release/ofml alb /path/to/data.alb
```

**List specific file types:**
```bash
./target/release/ofml alb /path/to/data.alb "*.cls"    # CLS scripts
./target/release/ofml alb /path/to/data.alb "*.geo"    # Geometry
./target/release/ofml alb /path/to/data.alb "*.png"    # Textures
```

**Extract to directory:**
```bash
./target/release/ofml alb /path/to/data.alb "*" ./extracted/
```

---

## Task 7: Run CLS Script and Export GLB

If you have a standalone CLS file:

```bash
./target/release/ofml export myscript.cls
```

Output: `myscript.glb`

For CLS files inside an ALB archive:

```bash
./target/release/ofml build /path/to/data.alb MyClassName
```

---

## Common Patterns

### Batch Convert All Products from a Manufacturer

```bash
#!/bin/bash
OFML="./target/release/ofml"
DATA="/path/to/ofmldata"
MANUFACTURER="vitra"
OUTPUT="./output"

mkdir -p "$OUTPUT"

for product in $(ls "$DATA/$MANUFACTURER"); do
    version_dir="$DATA/$MANUFACTURER/$product/1"
    if [ -f "$version_dir/odb.ebase" ]; then
        echo "Converting $product..."
        $OFML product "$version_dir" "$OUTPUT/${product}.glb" 2>/dev/null
    fi
done

echo "Done! Files in $OUTPUT/"
```

### Find Products with 2D Data

```bash
#!/bin/bash
OFML="./target/release/ofml"
DATA="/path/to/ofmldata"

for manufacturer in $(ls "$DATA"); do
    for product in $(ls "$DATA/$manufacturer" 2>/dev/null); do
        ebase="$DATA/$manufacturer/$product/1/odb.ebase"
        if [ -f "$ebase" ]; then
            if $OFML ebase "$ebase" 2>/dev/null | grep -q "odb2d"; then
                echo "$manufacturer/$product has 2D data"
            fi
        fi
    done
done
```

### Validate All Geometry Files

```bash
#!/bin/bash
OFML="./target/release/ofml"

for file in *.3ds *.geo *.obj; do
    if [ -f "$file" ]; then
        echo "=== $file ==="
        $OFML validate "$file"
        echo
    fi
done
```

---

## Troubleshooting Quick Fixes

| Problem | Quick Fix |
|---------|-----------|
| "odb.ebase not found" | Add `/1` to path (version directory) |
| "No geometry found" | Try `gsx` command instead of `product` |
| Empty GLB | Check `./ofml validate input.3ds` for issues |
| Parse error | Run `./ofml check file.cls` for details |
| Can't open ALB | Verify it's a valid ZIP: `file data.alb` |

---

## Next Steps

- Read the full [README.md](./README.md) for detailed documentation
- Explore the [API Reference](./API.md) for library usage
- Check `cargo test` output for usage examples in tests
