#!/usr/bin/env python3
"""Compare Framery and FAST data structures to understand the filtering issue."""

import subprocess
import re
from pathlib import Path

def run_ebase_command(db_path, table):
    """Run ebase command and return output."""
    try:
        result = subprocess.run(
            ["cargo", "run", "--quiet", "--", "ebase", str(db_path), table],
            capture_output=True,
            text=True,
            timeout=10
        )
        return result.stdout
    except Exception as e:
        return ""

def extract_price_data(output):
    """Extract price records."""
    prices = []
    current_price = {}

    for line in output.split('\n'):
        if line.startswith("Record"):
            if current_price:
                prices.append(current_price)
            current_price = {}
        else:
            for field in ['article_nr', 'var_cond', 'price_level']:
                if f"{field}:" in line:
                    match = re.search(rf"{field}: '([^']*)'", line)
                    if match:
                        current_price[field] = match.group(1)

    if current_price:
        prices.append(current_price)

    return prices

def extract_propertyvalue_data(output):
    """Extract property value records."""
    propvalues = []
    current_pv = {}

    for line in output.split('\n'):
        if line.startswith("Record"):
            if current_pv:
                propvalues.append(current_pv)
            current_pv = {}
        else:
            for field in ['prop_class', 'property', 'value_from']:
                if f"{field}:" in line:
                    match = re.search(rf"{field}: '([^']*)'", line)
                    if match:
                        current_pv[field] = match.group(1)

    if current_pv:
        propvalues.append(current_pv)

    return propvalues

print("=" * 80)
print("FRAMERY vs FAST DATA STRUCTURE COMPARISON")
print("=" * 80)

# FRAMERY
framery_db = Path("/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase")
print(f"\n{'=' * 80}")
print(f"FRAMERY (frmr_one)")
print(f"Path: {framery_db}")
print(f"Series in path: frmr_one")
print(f"{'=' * 80}")

prop_output = run_ebase_command(framery_db, "ocd_property")
prop_classes = sorted(set(re.findall(r"prop_class: '([^']+)'", prop_output)))
print(f"\nProperty Classes: {prop_classes}")

price_output = run_ebase_command(framery_db, "ocd_price")
prices = extract_price_data(price_output)
print(f"\nSample Prices (first 5):")
for price in prices[:5]:
    print(f"  - Article: {price.get('article_nr', 'N/A'):20} Level: {price.get('price_level', 'N/A'):5} VarCond: {price.get('var_cond', 'N/A')}")

propvalue_output = run_ebase_command(framery_db, "ocd_propertyvalue")
propvalues = extract_propertyvalue_data(propvalue_output)
print(f"\nSample Property Values (first 10):")
for pv in propvalues[:10]:
    print(f"  - Class: {pv.get('prop_class', 'N/A'):25} Property: {pv.get('property', 'N/A'):30} Value: {pv.get('value_from', 'N/A')}")

# Check if property class names contain series identifier
print(f"\nDo property classes contain 'frmr_one'? {any('frmr_one' in pc.lower() for pc in prop_classes)}")
print(f"Do property classes contain 'FRMR_ONE'? {any('FRMR_ONE' in pc for pc in prop_classes)}")
print(f"Property classes are: {prop_classes}")

# FAST
fast_db = Path("/reference/ofmldata/fast/kr/DE/1/db/pdata.ebase")
print(f"\n{'=' * 80}")
print(f"FAST (kr)")
print(f"Path: {fast_db}")
print(f"Series in path: kr")
print(f"{'=' * 80}")

prop_output = run_ebase_command(fast_db, "ocd_property")
prop_classes_fast = sorted(set(re.findall(r"prop_class: '([^']+)'", prop_output)))
print(f"\nProperty Classes: {prop_classes_fast}")

price_output = run_ebase_command(fast_db, "ocd_price")
prices_fast = extract_price_data(price_output)
print(f"\nSample Prices (first 5):")
for price in prices_fast[:5]:
    print(f"  - Article: {price.get('article_nr', 'N/A'):20} Level: {price.get('price_level', 'N/A'):5} VarCond: {price.get('var_cond', 'N/A')}")

propvalue_output = run_ebase_command(fast_db, "ocd_propertyvalue")
propvalues_fast = extract_propertyvalue_data(propvalue_output)
print(f"\nSample Property Values (first 10):")
for pv in propvalues_fast[:10]:
    print(f"  - Class: {pv.get('prop_class', 'N/A'):25} Property: {pv.get('property', 'N/A'):30} Value: {pv.get('value_from', 'N/A')}")

# Check if property class names contain series identifier
print(f"\nDo property classes contain 'kr'? {any('kr' in pc.lower() for pc in prop_classes_fast)}")
print(f"Do property classes contain 'KR'? {any('KR' in pc for pc in prop_classes_fast)}")
print(f"Property classes are: {prop_classes_fast}")

# SUMMARY
print(f"\n{'=' * 80}")
print("KEY DIFFERENCES")
print(f"{'=' * 80}")

print(f"\nFramery property classes: {prop_classes}")
print(f"  - Generic names (MG_GLOBAL, MG_PROPERTIES)")
print(f"  - Shared across series")
print(f"  - NO series identifier in class name")

print(f"\nFAST property classes: {prop_classes_fast}")
print(f"  - Series-specific names")
if any('KR' in pc for pc in prop_classes_fast):
    print(f"  - CONTAINS series identifier (KR) in class name")
else:
    print(f"  - Check if contains series identifier")

print(f"\n{'=' * 80}")
print("CONCLUSION")
print(f"{'=' * 80}")

print("""
The issue is clear:

1. FAST uses series-specific property classes (e.g., containing "KR" for kr series)
   - Series-based filtering WORKS for FAST because property classes contain series name

2. Framery uses SHARED property classes (MG_GLOBAL, MG_PROPERTIES)
   - Series-based filtering BREAKS Framery because property classes don't contain series name
   - When filtering by "source_series = frmr_one", NO properties match because:
     * Property class "MG_PROPERTIES" doesn't contain "frmr_one"
     * Property class "MG_GLOBAL" doesn't contain "frmr_one"

SOLUTION NEEDED:
- Property filtering should NOT be series-based for manufacturers like Framery
- Need to detect when property classes are shared vs series-specific
- OR: use article-to-property-class mapping (ocd_propertyclass) instead of filtering
""")
