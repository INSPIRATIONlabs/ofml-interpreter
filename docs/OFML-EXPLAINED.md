# Understanding OFML: A Complete Guide

This document explains what OFML is, why it exists, and what each file type is used for.

---

## Table of Contents

1. [What is OFML?](#what-is-ofml)
2. [The Problem OFML Solves](#the-problem-ofml-solves)
3. [How Furniture Configuration Works](#how-furniture-configuration-works)
4. [File Types Explained](#file-types-explained)
5. [CLS Files Deep Dive](#cls-files-deep-dive)
6. [EBASE Database Explained](#ebase-database-explained)
7. [The Complete Data Flow](#the-complete-data-flow)
8. [Real-World Example](#real-world-example)

---

## What is OFML?

**OFML** stands for **Office Furniture Modeling Language**. It is an industry standard developed by the German furniture industry (specifically by EasternGraphics/Cyncly) for distributing 3D product data.

### Who Uses OFML?

- **Furniture Manufacturers**: Vitra, Sedus, Wilkhahn, Steelcase, Herman Miller, and 100+ others
- **Dealers/Resellers**: Furniture dealers who create quotes and room layouts
- **Software Vendors**: Space planning and configuration software
- **Architects**: For furniture placement in building designs

### Why Was OFML Created?

Before OFML, every furniture manufacturer had their own proprietary format. Dealers needed different software for each manufacturer. OFML created a **universal standard** so:

1. Manufacturers export their products once in OFML format
2. Any OFML-compatible software can load any manufacturer's products
3. Dealers can mix products from different manufacturers in one project

---

## The Problem OFML Solves

### The Furniture Configuration Challenge

Office furniture is not like buying a simple product. Consider an office chair:

- **Dimensions**: Available in different seat widths (40cm, 45cm, 50cm)
- **Materials**: 20+ fabric options, 5 leather options, 3 mesh options
- **Colors**: Each material in multiple colors
- **Options**: With/without armrests, headrest, lumbar support
- **Bases**: 4-star, 5-star, sled base, in chrome or black

**One chair model can have 10,000+ possible configurations!**

### Why Not Just Use Static 3D Models?

Storing 10,000 separate 3D files would be:
- **Storage**: Terabytes of data per manufacturer
- **Updates**: Impossible to maintain
- **Download**: Users would need to download everything

### OFML's Solution: Parametric Geometry

Instead of storing every variant, OFML stores:

1. **Base geometry** - The basic 3D shapes
2. **Rules** - How geometry changes with parameters
3. **Materials** - Separate from geometry, applied dynamically
4. **Configuration logic** - Which options are valid together

This is where **CLS files** come in.

---

## How Furniture Configuration Works

### The Configuration Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                    USER INTERFACE                                    │
│  User selects: Width=45cm, Material=Leather, Color=Black            │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    PROPERTY SYSTEM                                   │
│  Properties: {width: 0.45, material: "leather", color: "black"}     │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    CLS SCRIPT EXECUTION                              │
│  CLS code evaluates properties and generates geometry               │
│  - If width > 0.45: add extra support beam                          │
│  - Material "leather" → use leather texture                         │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    3D SCENE OUTPUT                                   │
│  Final 3D model with correct dimensions, materials, options         │
└─────────────────────────────────────────────────────────────────────┘
```

### What Happens When You Change a Property?

1. User changes "width" from 45cm to 50cm
2. Property system updates the value
3. CLS script re-executes with new parameter
4. Geometry is regenerated (seat gets wider, frame adjusts)
5. 3D view updates in real-time

---

## File Types Explained

### Overview Table

| Extension | Full Name | Purpose | Human Readable? |
|-----------|-----------|---------|-----------------|
| `.cls` | Class Script | Parametric geometry logic | Yes (code) |
| `.ebase` | EBASE Database | Product catalog & data tables | No (binary) |
| `.alb` | Archive Library | Encrypted container for all files | No (encrypted ZIP) |
| `.3ds` | 3D Studio | Static 3D geometry | No (binary) |
| `.geo` | Geometry | OFML-specific geometry format | No (binary) |
| `.obj` | Wavefront OBJ | Static 3D geometry | Yes (text) |
| `.mat` | Material | Surface properties & textures | Yes (text) |
| `.png/.jpg/.tga/.bmp` | Textures | Surface images | No (image) |

---

### CLS Files (Class Scripts)

**What they are:** Programming scripts that define how furniture is built.

**Why they exist:** To create **parametric** (configurable) geometry that changes based on user selections.

**Language:** OFML is similar to JavaScript/Java with object-oriented features.

**Example - A simple configurable table:**

```cls
// Table.cls - A table with configurable dimensions
package ::manufacturer::tables;

import ::ofml::oi::*;

class Table : OiPart {
    // Default dimensions (in meters)
    var width = 1.6;
    var depth = 0.8;
    var height = 0.72;

    func initialize() {
        // Create the tabletop
        // OiBlock creates a box with [width, height, depth]
        OiBlock(self, @tabletop, [width, 0.025, depth]);

        // Position tabletop at correct height
        setPosition(0, height - 0.025, 0);

        // Create 4 legs
        createLeg(-width/2 + 0.05, -depth/2 + 0.05);  // front-left
        createLeg(width/2 - 0.05, -depth/2 + 0.05);   // front-right
        createLeg(-width/2 + 0.05, depth/2 - 0.05);   // back-left
        createLeg(width/2 - 0.05, depth/2 - 0.05);    // back-right

        // Setup user-configurable properties
        setupProperty(@width, ["Width (cm)", NULL, NULL, 0, "ch 120 140 160 180 200"], 1);
        setupProperty(@depth, ["Depth (cm)", NULL, NULL, 0, "ch 60 80 100"], 2);
    }

    func createLeg(x, z) {
        var leg = OiCylinder(self, @leg, [0.03, height - 0.025]);
        leg.setPosition(x, 0, z);
    }

    // Called when user changes width property
    func onWidthChanged(newWidth) {
        width = newWidth / 100;  // Convert cm to meters
        rebuild();  // Regenerate geometry
    }
}
```

**What this CLS does:**
1. Defines a `Table` class
2. Creates a tabletop using `OiBlock` primitive
3. Creates 4 cylindrical legs using `OiCylinder`
4. Registers properties that users can change
5. When width changes, the whole table rebuilds with new dimensions

---

### EBASE Files (Database)

**What they are:** Binary databases containing structured product data.

**Why they exist:** To store large amounts of product information efficiently.

**What's inside:** Multiple tables, similar to a SQL database.

#### Common EBASE Tables

| Table Name | Purpose | What It Contains |
|------------|---------|------------------|
| `odb3d` | 3D Object Database | Geometry references and constructors |
| `odb2d` | 2D Object Database | Floor plan shapes |
| `mat` | Materials | Material definitions |
| `article` | Article Catalog | Product SKUs and names |
| `variant` | Variants | Configuration options |
| `propdef` | Property Definitions | What properties exist |
| `attpt` | Attachment Points | Where furniture connects |
| `oppattpt` | Opposite Attachments | Matching attachment pairs |
| `stdattpt` | Standard Attachments | Common attachment types |
| `pricelist` | Pricing | Product prices |
| `texttable` | Text | Translations and labels |

#### The `odb3d` Table in Detail

This is the most important table for 3D rendering. Each record represents one piece of geometry.

| Column | Description | Example |
|--------|-------------|---------|
| `odb_name` | Article/variant identifier | `"CHAIR_V1"` |
| `ctor` | Constructor expression | `"seat.geo" 1 1 1 imp` |
| `x_offs` | X position offset | `0.0` |
| `y_offs` | Y position offset | `0.45` |
| `z_offs` | Z position offset | `0.0` |
| `x_rot` | X rotation (degrees) | `0` |
| `y_rot` | Y rotation (degrees) | `90` |
| `z_rot` | Z rotation (degrees) | `0` |

#### The `ctor` Field (Constructor)

The `ctor` field contains a **PostScript-like expression** that tells the system what geometry to load and how to transform it.

**Simple example:**
```
"seat.geo" 1 1 1 imp
```
This means: Load `seat.geo` file with scale [1, 1, 1] and import it.

**Complex example:**
```
${WIDTH:-0.5} /w exch def
${DEPTH:-0.4} /d exch def
"frame.geo" w d 1 imp
${HAS_ARMREST} 1 eq { "armrest.geo" 1 1 1 imp } if
```
This means:
1. Get WIDTH property (default 0.5), store as `w`
2. Get DEPTH property (default 0.4), store as `d`
3. Load frame.geo scaled by [w, d, 1]
4. If HAS_ARMREST equals 1, also load armrest.geo

**Why PostScript?** PostScript is a stack-based language that's compact and can express conditional logic. This allows geometry loading to be data-driven without needing full CLS execution.

---

### ALB Files (Archives)

**What they are:** Password-encrypted ZIP archives.

**Why they exist:**
1. **Single file distribution** - All product data in one file
2. **Copy protection** - Manufacturers protect their data
3. **Integrity** - Ensures files aren't modified

**What's inside an ALB:**

```
data.alb (encrypted ZIP)
├── cls/
│   ├── Chair.cls
│   ├── Table.cls
│   └── Common.cls
├── geometry/
│   ├── seat.geo
│   ├── back.3ds
│   └── frame.obj
├── textures/
│   ├── fabric_blue.png
│   ├── leather_black.jpg
│   └── wood_oak.tga
├── materials/
│   └── materials.mat
└── odb.ebase
```

---

### Geometry Files (.3ds, .geo, .obj)

**What they are:** Static 3D mesh data (vertices, faces, normals).

**Why multiple formats?**

| Format | Origin | Pros | Cons |
|--------|--------|------|------|
| `.3ds` | 3D Studio MAX (1990s) | Widely supported | Old format, limited features |
| `.geo` | OFML native | Compact, fast | OFML-specific |
| `.obj` | Wavefront | Human-readable, universal | Large file size |

**Important:** These are **static** meshes. They don't contain configuration logic. A chair seat is just a 3D shape - it doesn't know it's a chair seat.

**The CLS files give meaning to geometry:**
```cls
// Without CLS: just a mesh called "object_001"
// With CLS: "this is a seat that should be at height Y and use fabric material"
```

---

### Material Files (.mat)

**What they are:** Text files defining surface appearance.

**What they contain:**

```
# Material definition
newmtl oak_wood
Ka 0.2 0.15 0.1          # Ambient color (RGB)
Kd 0.6 0.45 0.3          # Diffuse color (RGB)
Ks 0.3 0.25 0.2          # Specular highlight (RGB)
Ns 32.0                   # Shininess
d 1.0                     # Opacity (1 = solid)
map_Kd textures/oak.png   # Diffuse texture map
```

**Why separate from geometry?**

The same chair geometry can have:
- Blue fabric material
- Red fabric material
- Black leather material
- Grey mesh material

Instead of 4 separate 3D files, you have:
- 1 geometry file
- 4 material definitions

---

### Texture Files (.png, .jpg, .tga, .bmp)

**What they are:** Image files wrapped onto 3D surfaces.

**Common texture types:**

| Type | Purpose |
|------|---------|
| Diffuse (color) map | Base color of surface |
| Normal map | Fake surface bumps |
| Roughness map | Shiny vs matte areas |
| Ambient occlusion | Soft shadows in creases |

**Sizes:** Typically 512x512, 1024x1024, or 2048x2048 pixels.

---

## CLS Files Deep Dive

### Why CLS Exists

**Problem:** How do you represent a chair that:
- Comes in 3 seat widths
- Has optional armrests
- Has 5 base types
- Can be upholstered in 50 fabrics

**Solution 1: Static files** (Bad)
- 3 × 2 × 5 × 50 = 1,500 separate 3D files
- Maintenance nightmare
- Huge download size

**Solution 2: CLS scripts** (Good)
- 1 CLS file with logic
- ~10 geometry files for parts
- 50 material definitions
- Script assembles correct combination

### CLS Language Features

#### Classes and Inheritance

```cls
// Base class for all seating
class Seating : OiPart {
    var seatHeight = 0.45;

    func initialize() {
        // Common initialization
    }
}

// Office chair extends seating
class OfficeChair : Seating {
    var hasArmrests = true;

    func initialize() {
        super.initialize();  // Call parent
        if (hasArmrests) {
            addArmrests();
        }
    }
}

// Stool is simpler seating
class Stool : Seating {
    func initialize() {
        super.initialize();
        seatHeight = 0.65;  // Higher seat
    }
}
```

#### Geometry Primitives

CLS provides built-in primitives for creating geometry:

| Primitive | Creates | Parameters |
|-----------|---------|------------|
| `OiBlock` | Box/cuboid | [width, height, depth] |
| `OiCylinder` | Cylinder | [radius, height] |
| `OiSphere` | Sphere | [radius] |
| `OiEllipsoid` | Ellipsoid | [rx, ry, rz] |
| `OiPolygon` | Flat polygon | [points] |
| `OiImport` | External file | "filename.geo" |

#### Geometry Transforms

Modify geometry position, rotation, scale:

| Class | Effect |
|-------|--------|
| `GoYLTrans` | Stretch along Y axis |
| `GoXLTrans` | Stretch along X axis |
| `GoZLTrans` | Stretch along Z axis |
| `GoMirror` | Mirror geometry |
| `GoXLRTransYLRTrans` | Combined X/Y stretch |

Example - Creating a table leg that stretches with table height:

```cls
class TableLeg : GoYLTrans {
    // GoYLTrans automatically stretches child geometry in Y
    // based on the parent object's height property

    func initialize() {
        OiCylinder(self, @leg, [0.03, 1.0]);  // Base 1m tall
        // When parent table height changes, leg stretches
    }
}
```

#### Property System

Properties allow user configuration:

```cls
func initialize() {
    // Define a choice property
    setupProperty(
        @seatWidth,                              // Property key
        [
            "Seat Width",                        // Display label
            NULL,                                // Min value
            NULL,                                // Max value
            0,                                   // Decimal places
            "ch 40 45 50"                        // Choices: 40, 45, or 50
        ],
        1                                        // Sort order
    );

    // Define a boolean property
    setupProperty(
        @hasArmrests,
        ["Armrests", NULL, NULL, 0, "bool"],
        2
    );

    // Define a range property
    setupProperty(
        @backAngle,
        ["Back Angle", 90, 120, 0, "range"],    // Range 90-120
        3
    );
}
```

#### Conditional Geometry

```cls
func initialize() {
    // Always create seat
    OiBlock(self, @seat, [width, 0.05, depth]);

    // Conditional: only if hasArmrests is true
    if (getPropValue(@hasArmrests)) {
        createArmrest(@left, -width/2);
        createArmrest(@right, width/2);
    }

    // Conditional: different geometry based on base type
    var baseType = getPropValue(@baseType);
    if (baseType == @fourStar) {
        createFourStarBase();
    } else if (baseType == @fiveStar) {
        createFiveStarBase();
    } else if (baseType == @sledBase) {
        createSledBase();
    }
}
```

---

## EBASE Database Explained

### Why a Database?

OFML products have thousands of data records:
- Article catalog (SKUs, names, prices)
- Geometry references
- Material mappings
- Attachment points
- Translations

A binary database is:
- **Fast** to query
- **Compact** in size
- **Structured** for relationships

### Table Relationships

```
┌─────────────┐       ┌─────────────┐
│   article   │──────>│   variant   │
│  (products) │       │  (options)  │
└─────────────┘       └─────────────┘
       │                     │
       │                     │
       ▼                     ▼
┌─────────────┐       ┌─────────────┐
│   odb3d     │       │    mat      │
│ (3D refs)   │       │ (materials) │
└─────────────┘       └─────────────┘
       │
       ▼
┌─────────────┐
│   attpt     │
│ (attach pts)│
└─────────────┘
```

### How Records Connect

**Article** "CHAIR_BASIC" has:
- **Variants**: CHAIR_BASIC_SMALL, CHAIR_BASIC_MEDIUM, CHAIR_BASIC_LARGE
- **odb3d records**: For each variant, which geometry files to load
- **Materials**: Which materials can be applied
- **Attachment points**: Where it connects to desks

---

## The Complete Data Flow

### Scenario: User Opens a Chair Product

```
1. USER ACTION
   User opens product "vitra/ac" in configurator

2. LOAD EBASE
   Software reads /vitra/ac/1/odb.ebase
   - Gets article list from "article" table
   - Gets available variants from "variant" table
   - Gets property definitions from "propdef" table

3. DISPLAY OPTIONS
   UI shows:
   - Seat width: [40cm] [45cm] [50cm]
   - Material: [Fabric] [Leather] [Mesh]
   - Color: [Blue] [Red] [Black] ...
   - Armrests: [Yes] [No]

4. USER SELECTS OPTIONS
   User picks: 45cm, Leather, Black, Yes armrests

5. FIND MATCHING VARIANT
   Query "variant" table for configuration
   → Finds "AC_CHAIR_45_LEATHER_BLACK_ARM"

6. LOAD GEOMETRY
   Query "odb3d" table for variant
   → Returns records with ctor expressions:
     - "seat.geo" 0.45 0.05 0.4 imp
     - "back.geo" 1 1 1 imp
     - "armrest_left.geo" 1 1 1 imp
     - "armrest_right.geo" 1 1 1 imp

7. EXECUTE CONSTRUCTORS
   For each ctor:
   - Parse expression
   - Load geometry from ALB archive
   - Apply scale and position
   - Add to scene

8. APPLY MATERIALS
   Query "mat" table for "LEATHER_BLACK"
   → Apply to all geometry

9. RENDER
   Display final 3D model
```

### Scenario: Using Full CLS Execution

For complex products, simple ctor expressions aren't enough:

```
1. USER ACTION
   User opens complex product with parametric geometry

2. LOAD CLS FILES
   Extract .cls files from ALB archive

3. PARSE CLS
   Lexer → Parser → AST

4. EXECUTE CLS
   Interpreter runs the code:
   - Creates class instances
   - Executes initialize() methods
   - Builds scene graph

5. PROPERTY CHANGES
   When user changes option:
   - Property callback fires
   - CLS re-evaluates
   - Geometry regenerates

6. EXPORT
   Scene graph → GLB file
```

---

## Real-World Example

### Vitra AC Chair Product

```
/ofmldata/vitra/ac/1/
├── odb.ebase              # Product database
└── data.alb               # Archive containing:
    ├── AC_Chair.cls       # Main chair class
    ├── AC_Base.cls        # Base variants
    ├── AC_Common.cls      # Shared utilities
    ├── seat_45.geo        # Seat geometry (45cm)
    ├── seat_50.geo        # Seat geometry (50cm)
    ├── back.3ds           # Backrest
    ├── armrest.obj        # Armrest
    ├── base_4star.geo     # 4-star base
    ├── base_5star.geo     # 5-star base
    ├── fabric_blue.png    # Textures
    ├── fabric_red.png
    ├── leather_black.jpg
    └── materials.mat      # Material definitions
```

### What Happens

1. **Simple Mode (product command):**
   - Read odb.ebase
   - Parse ctor fields
   - Load geometry files directly
   - Apply materials
   - Export GLB

2. **Full Mode (build command):**
   - Load AC_Chair.cls
   - Execute: creates Chair instance
   - Chair.initialize() runs
   - Based on properties, creates seat, back, armrests
   - Applies transforms for dimensions
   - Scene graph built programmatically
   - Export GLB

### When to Use Which Mode

| Situation | Use |
|-----------|-----|
| Static preview of product | `product` command |
| Fixed configuration export | `product` command |
| Parametric geometry needed | `build` command |
| Real-time configurator | `build` with CLS execution |
| Batch conversion | `product` (faster) |

---

## Summary

### File Type Purpose Summary

| File | One-Line Purpose |
|------|-----------------|
| `.cls` | Code that builds parametric furniture |
| `.ebase` | Database of product catalog and geometry references |
| `.alb` | Encrypted container holding all product files |
| `.3ds/.geo/.obj` | Static 3D mesh data (vertices, triangles) |
| `.mat` | Surface appearance (colors, shininess) |
| `.png/.jpg` | Surface images (wood grain, fabric pattern) |

### Why This Architecture?

1. **Separation of concerns**: Geometry, materials, logic are separate
2. **Reusability**: Same geometry with different materials
3. **Configurability**: CLS allows infinite variations
4. **Efficiency**: Database queries faster than file scanning
5. **Distribution**: ALB bundles everything in one file
6. **Protection**: Encryption prevents unauthorized copying

### The Key Insight

**CLS files are the "brains" of OFML products.**

Without CLS:
- Static geometry files are just shapes
- No understanding of what parts mean
- No configuration possible

With CLS:
- Geometry becomes meaningful (this is a seat, this is a leg)
- Parts can be conditional (add armrests if selected)
- Dimensions can vary (stretch frame for wider seat)
- Materials can be applied correctly (seat gets fabric, legs get chrome)

This is why the OFML Interpreter includes a full CLS execution engine - it's the only way to properly render complex parametric furniture products.
