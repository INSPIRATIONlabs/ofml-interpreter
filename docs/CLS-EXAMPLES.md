# CLS Scripts: What They Actually Do

This document shows real CLS scripts from manufacturers and explains what each part does.

---

## The Reality: CLS Scripts Are Complex Business Logic

Looking at real manufacturer CLS files (Vitra, Sedus, etc.), they're **not primarily about geometry creation**. Instead, they handle:

1. **Property Management** - What options users can select
2. **Pricing Logic** - How options affect price
3. **Availability Checking** - Is this configuration available?
4. **Article Numbers** - Generating SKUs for orders
5. **Database Queries** - Reading from EBASE tables
6. **Geometry Assembly** - Loading and positioning 3D parts

The actual geometry is usually:
- Pre-made 3DS/GEO/OBJ files
- Loaded via `OiImport` or EBASE `ctor` expressions
- Positioned and scaled based on properties

---

## Real Example 1: Geometry Import (VitraOiImport)

This class loads 3D geometry from files:

```cls
// vitraoiimport.cls - Loads 3DS geometry with quality levels
package ::vitra::home;
import ::ofml::oi::*;

public class VitraOiImport: OiPart
{
    var mName;      // Geometry filename
    var mZif;       // Quality flag
    var mScale;     // Scale factor [x, y, z]

    public func initialize(pFa, pNa, pNu, p3ds)
    {
        // Call parent constructor
        ::ofml::oi::OiPart::initialize(pFa, pNa);

        // Configure transform axes
        setRtAxis(7);    // Rotation axis
        setTrAxis(7);    // Translation axis
        notSelectable(); // Can't select this in UI

        // Store parameters
        mName = p3ds;           // e.g., "chair_seat"
        mScale = [1, 1, 1];     // Default scale
        mZif = pNu;             // Quality number

        // Get quality setting from scene
        var tQ;
        try {
            tQ = getScene().getInfo(getProgram()).getQuality();
        } catch(&e : Error) {
            tQ = @Q3ds2;  // Default to medium quality
        }

        // Load the geometry
        update3DS(tQ);
    }

    // Load geometry at specified quality level
    public func update3DS(tQ)
    {
        // Remove existing geometry
        try { remove(geo); }

        var str = mName;

        // Quality levels use different file suffixes
        // Q3ds1 = low quality  -> filename + "_"
        // Q3ds2 = medium       -> filename (no suffix)
        // Q3ds3 = high quality -> filename + "__"
        if (tQ == @Q3ds1 && mZif != 2 && mZif != 4 && mZif != 6)
            str = mName + "_";
        else if (tQ == @Q3ds2 && mZif != 1 && mZif != 4 && mZif != 5)
            str = mName;
        else if (tQ == @Q3ds3 && mZif != 1 && mZif != 2 && mZif != 3)
            str = mName + "__";

        // Import the 3DS file
        try {
            OiImport(self, @geo, @G3DS, str);
            self.geo.setScale(mScale);
        } catch(&e : Error) {
            // Fallback: try base filename
            try { remove(geo); }
            OiImport(self, @geo, @G3DS, mName);
            self.geo.setScale(mScale);
        }
    }

    // Set scale of loaded geometry
    public func setScale(pVal)
    {
        mScale = pVal;
        self.geo.setScale(pVal);
    }
}
```

**What this does:**
1. Takes a geometry filename as parameter
2. Checks quality setting (low/medium/high)
3. Loads appropriate geometry file (with `_` or `__` suffix)
4. Applies scale transform
5. Handles errors gracefully with fallback

---

## Real Example 2: Product Element with Properties

This is the main class for configurable products. It handles:

```cls
// vitraoibtgplelement3.cls - Configurable product element
package ::vitra::basics;
import ::ofml::oi::*;
import ::ofml::xoi::*;

public class VitraOiBTGPlElement3: xOiBTGPlElement3
{
    public func initialize(pFa, pNa)
    {
        // Call parent (which does the actual geometry creation)
        ::ofml::xoi::xOiBTGPlElement3::initialize(pFa, pNa);
        setResolution(1);
    }

    // When user changes a property value
    public func setPropValue(pPKey, pPValue)
    {
        // Let parent handle the change
        var tRet = xOiBTGPlElement3::setPropValue(pPKey, pPValue);
        return 1;
    }

    // Get available options with prices
    public func getPropChoiceList(pKey)
    {
        // Get base options from parent
        var tRet = xOiBTGPlElement3::getPropChoiceList(pKey);

        // Add prices to each option
        var tArticle = getArticleSpec();
        if (tArticle == NULL) return tRet;

        var tDBPath = getDataBasePath();
        if (tDBPath == NULL) return tRet;

        var tVCHash = __VC_getVarCondHash(tArticle, tDBPath);
        if (tVCHash == NULL) return tRet;

        // Add price info to each choice
        return __VC_addPrices2Prop(pKey, tRet, tArticle, ...);
    }

    // Called when properties change
    public func propsChanged(pPKeys, pVal)
    {
        var tKey;
        foreach(tKey; pPKeys) {
            // Handle "apply to all similar items" feature
            if (tKey == @VI_Changes && getPropValue(@VI_Changes) == @allarticles) {
                // Find all similar products and apply same properties
                var tEls = getPlanning().getElements();
                var tEl;
                foreach(tEl; tEls) {
                    if (tEl != getFather() &&
                        tEl.isA(::vitra::basics::aMetaType) &&
                        tEl.hasProperty(@GBA_Serie)) {
                        // Copy properties to similar items
                        if (tEl.getPropValue(@GBA_Family) == getFather().getPropValue(@GBA_Family)) {
                            var tProp;
                            foreach(tProp; getPropertyKeys()) {
                                if (tProp != @VI_Changes) {
                                    if (tEl.hasProperty(tProp)) {
                                        tEl.setPropValue(tProp, getPropValue(tProp));
                                    }
                                }
                            }
                        }
                    }
                }
                return(0);
            }
        }

        // Let parent handle property change (regenerates geometry)
        var tRet = xOiBTGPlElement3::propsChanged(pPKeys, pVal);
        return(tRet);
    }

    // Check product availability
    public func getAvailabilityInfo()
    {
        var tArtNbr = getArticleSpec();
        if (tArtNbr != NULL) {
            var tDBPath = getDataBasePath();
            if (tDBPath == NULL) return(NULL);

            // Open availability table in EBASE
            var tTblID = "VAR EBASE,CSV " + tDBPath + " pdata vitra_availability";
            var tTable = xOiTable2(tTblID, sVitraAvailability);

            if (tTable.open()) {
                // Look up this article's availability
                var tRes = tTable.read1EntryFor(
                    @(["article_nr", tArtNbr]),
                    @("date_from", "date_to"),
                    0
                );

                if (tRes != NULL && tRes[0] != "") {
                    var date_from = tRes[0];

                    // Parse date and check if in future
                    if (date_from.size() >= 6) {
                        var tYear = Int(date_from.substr(0, 4));
                        var tMonth = Int(date_from.substr(4, 2));

                        var tTime = ::time::localtime(::time::time());

                        // If available date is in future, show message
                        if (tYear > tTime[@YEAR] ||
                            (tYear == tTime[@YEAR] && tMonth > tTime[@MONTH])) {
                            oiOutput(@MESSAGE,
                                "Product available from: " + tMonth + "/" + tYear);
                        }
                    }
                }
                tTable.close();
            }
        }
        return(NULL);
    }
}
```

**What this does:**
1. **Property Changes**: When user changes fabric/color/size, triggers geometry rebuild
2. **Pricing**: Reads prices from EBASE and shows price differences per option
3. **Apply to All**: Can apply selected options to all similar products
4. **Availability**: Checks if product is available now or future date

---

## Real Example 3: Modular Furniture (Alcove)

The Alcove is a modular sofa/space divider. CLS handles:

```cls
// ametatypealcove.cls - Alcove modular element
package ::vitra::alcove;
import ::ofml::go::*;
import ::ofml::oi::*;

public class aMetaTypeAlcove: aMetaType
{
    public func initialize(pFa, pNa)
    {
        // Parent does geometry setup
        ::vitra::alcove::aMetaType::initialize(pFa, pNa);
        selectable();  // User can click on this element
    }

    // Called when properties change (size, angle, model)
    public func propsChanged(pProps, pCheck)
    {
        var tRet = aMetaType::propsChanged(pProps, pCheck);
        if (!tRet) return(tRet);

        var tFather = getFather();

        // If angle or size changed, update connections to neighbors
        if (pCheck && isMetaInitialized() &&
            tFather.isA(VitraOiLayoutGroupALC)) {

            if (pProps.find(@GALCP_Angle) >= 0 ||
                pProps.find(@GSize) >= 0 ||
                pProps.find(@GModel) >= 0) {

                // Tell parent group that dimensions changed
                var tAttPt = @ApOapAP_R;
                tRet = tFather.dimensionChanged2(self, tAttPt, 1);

                if (tRet == 1 && pProps.find(@GSize) >= 0) {
                    tAttPt = @ApOapAP_L;
                    tRet = tFather.dimensionChanged2(self, tAttPt, 1);
                }
            }
        }

        return(tRet);
    }

    // Can this element move inward/outward?
    public func canMove(pRefObj, pDirection)
    {
        var tRet = 0;
        var tFather = getFather();

        if (tFather.isA(VitraOiLayoutGroupALC)) {
            // Get current angle
            var tAngle = parseSymbol2Float(getPropValue(@GALCP_Angle));

            // Check if movement is allowed based on angle limits
            if (pDirection == @OUTER && tAngle > -90) {
                tRet = 1;
            } else if (pDirection == @INNER && tAngle < 33.75) {
                tRet = 1;
            }
        }

        return(tRet);
    }

    // Move element in specified direction
    public func move(pRefObj, pDirection)
    {
        var tEl = (pRefObj == @NEXT) ? self :
                  getFather().getNeighbor(self, @ApOapAP_L);

        var currentAngle = parseSymbol2Float(
            tEl.getPropValue(@GALCP_Angle));

        // Rotate by 11.25 degrees per step
        if (pDirection == @OUTER && currentAngle > -90) {
            tEl.setPropValue(@GALCP_Angle,
                parseFloat2Symbol(currentAngle - 11.25));
        } else if (pDirection == @INNER && currentAngle < 33.75) {
            tEl.setPropValue(@GALCP_Angle,
                parseFloat2Symbol(currentAngle + 11.25));
        }
    }

    // Delete this element from the group
    public func deleteSelf()
    {
        var tFather = getFather();

        // Handle edge cases (leftmost/rightmost element)
        if (tFather.getCatElements(@CatLeftMost)[0] == self) {
            var tNeighbour = tFather.getNeighbor(self, @ApOapAP_R);
            tFather.removeElementCat(self, @CatLeftMost);
            tFather.assignElementCat(tNeighbour, @CatLeftMost);
        } else if (tFather.getCatElements(@CatRightMost)[0] == self) {
            var tNeighbour = tFather.getNeighbor(self, @ApOapAP_L);
            tFather.removeElementCat(self, @CatRightMost);
            tFather.assignElementCat(tNeighbour, @CatRightMost);
        }

        // Remove from parent group
        tFather.remove(self);
    }

    // Calculate position for "move inner" button
    public func getMoveNextInnerInteractorX()
    {
        var tAngle = deg2rad(-parseSymbol2Float(getPropValue(@GALCP_Angle)));
        var tWidth = parseSymbol2Float(getPropValue(@GALCP_Width)) / 2;

        // Calculate X position using trigonometry
        return(cos(tAngle) * 0.4 - sin(tAngle) * (-0.25) + tWidth);
    }
}
```

**What this does:**
1. **Modular Assembly**: Handles connected elements in a group
2. **Angle Control**: Elements can rotate -90° to +33.75°
3. **Neighbor Awareness**: When one element moves, neighbors adjust
4. **UI Interactors**: Calculates where move buttons appear in 3D view
5. **Group Management**: Tracks leftmost/rightmost elements

---

## Simplified Example: What Basic CLS Looks Like

For clarity, here's what a simpler CLS would look like:

```cls
// SimpleTable.cls - A basic configurable table
package ::example::furniture;
import ::ofml::oi::*;

public class SimpleTable: OiPart
{
    // Default dimensions (meters)
    var width = 1.6;
    var depth = 0.8;
    var height = 0.72;
    var legDiameter = 0.05;

    public func initialize(pFa, pNa)
    {
        OiPart::initialize(pFa, pNa);

        // Create tabletop (box)
        OiBlock(self, @tabletop, [width, 0.025, depth]);
        self.tabletop.setPosition(0, height - 0.025, 0);
        self.tabletop.setMaterial("::materials::wood::oak");

        // Create 4 legs
        createLeg(@leg1, -width/2 + 0.05, -depth/2 + 0.05);
        createLeg(@leg2,  width/2 - 0.05, -depth/2 + 0.05);
        createLeg(@leg3, -width/2 + 0.05,  depth/2 - 0.05);
        createLeg(@leg4,  width/2 - 0.05,  depth/2 - 0.05);

        // Setup configurable properties
        setupProperty(@width,
            ["Table Width", 1.2, 2.0, 2, "range"],
            1);

        setupProperty(@depth,
            ["Table Depth", 0.6, 1.0, 2, "range"],
            2);

        setupProperty(@material,
            ["Surface", NULL, NULL, 0,
             "ch @oak \"Oak\" @walnut \"Walnut\" @white \"White\""],
            3);

        setPropValue(@width, 1.6);
        setPropValue(@depth, 0.8);
        setPropValue(@material, @oak);
    }

    func createLeg(name, x, z)
    {
        var legHeight = height - 0.025;
        OiCylinder(self, name, [legDiameter/2, legHeight]);
        self[name].setPosition(x, legHeight/2, z);
        self[name].setMaterial("::materials::metal::chrome");
    }

    // Called when width property changes
    public func onWidthChanged(newWidth)
    {
        width = newWidth;
        rebuild();  // Regenerate all geometry
    }

    // Called when depth property changes
    public func onDepthChanged(newDepth)
    {
        depth = newDepth;
        rebuild();
    }

    // Called when material property changes
    public func onMaterialChanged(newMaterial)
    {
        if (newMaterial == @oak) {
            self.tabletop.setMaterial("::materials::wood::oak");
        } else if (newMaterial == @walnut) {
            self.tabletop.setMaterial("::materials::wood::walnut");
        } else if (newMaterial == @white) {
            self.tabletop.setMaterial("::materials::laminate::white");
        }
    }

    func rebuild()
    {
        // Remove existing geometry
        remove(tabletop);
        remove(leg1);
        remove(leg2);
        remove(leg3);
        remove(leg4);

        // Recreate with new dimensions
        OiBlock(self, @tabletop, [width, 0.025, depth]);
        self.tabletop.setPosition(0, height - 0.025, 0);

        createLeg(@leg1, -width/2 + 0.05, -depth/2 + 0.05);
        createLeg(@leg2,  width/2 - 0.05, -depth/2 + 0.05);
        createLeg(@leg3, -width/2 + 0.05,  depth/2 - 0.05);
        createLeg(@leg4,  width/2 - 0.05,  depth/2 - 0.05);
    }
}
```

---

## What CLS Does vs. What EBASE/ctor Does

### EBASE ctor (Simple Path)

In the EBASE `odb3d` table, the `ctor` field contains simple expressions:

```
"seat.geo" 1 1 1 imp
```

This means:
- Load `seat.geo` file
- Scale by [1, 1, 1] (no scaling)
- Import into scene

More complex:
```
${WIDTH} /w exch def
"frame.geo" w 1 1 imp
```

This means:
- Get WIDTH property value
- Store as variable `w`
- Load `frame.geo` scaled by [w, 1, 1]

### CLS (Complex Path)

CLS scripts do everything the ctor can't:

| Feature | ctor | CLS |
|---------|------|-----|
| Load geometry file | Yes | Yes |
| Simple scaling | Yes | Yes |
| Conditional geometry | Limited | Full |
| Complex calculations | No | Yes |
| Property UI setup | No | Yes |
| Pricing logic | No | Yes |
| Availability check | No | Yes |
| Modular assembly | No | Yes |
| Neighbor connections | No | Yes |
| User interaction | No | Yes |

---

## Summary: What CLS Scripts Really Are

CLS scripts are **full programming code** that:

1. **Define Product Behavior**
   - What properties exist
   - What values are allowed
   - What happens when values change

2. **Manage Geometry**
   - Load 3D files
   - Position and scale parts
   - Conditionally show/hide parts

3. **Handle Business Logic**
   - Pricing rules
   - Availability dates
   - Article number generation
   - Order information

4. **Support Modular Products**
   - Connect elements to each other
   - Handle group operations
   - Manage neighbor relationships

5. **Provide UI Integration**
   - Where to show interactors
   - What labels to display
   - Error messages and warnings

**The geometry itself is typically pre-made 3DS/GEO/OBJ files.**

CLS scripts are the **business logic layer** that:
- Loads the right geometry for the configuration
- Positions it correctly
- Applies the right materials
- Handles all the rules about what's valid
