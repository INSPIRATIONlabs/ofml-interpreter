#!/usr/bin/env python3
"""
Analyze Framery 2Q and ONE pricing data to find base prices
"""

import subprocess
import json
import re

def run_ebase_dump(path, table):
    """Run ebase dump and return output"""
    cmd = ['cargo', 'run', '--', 'ebase', path, table]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout + result.stderr

def parse_price_records(output):
    """Parse price records from ebase output"""
    prices = []
    current_record = {}
    in_records = False

    for line in output.split('\n'):
        if '=== Records from ocd_price' in line:
            in_records = True
            continue

        if not in_records:
            continue

        if line.startswith('Record '):
            if current_record:
                prices.append(current_record)
            current_record = {}
        elif ':' in line and not line.strip().startswith('Table:'):
            parts = line.split(':', 1)
            if len(parts) == 2:
                key = parts[0].strip()
                value = parts[1].strip().strip("'")
                current_record[key] = value

    if current_record:
        prices.append(current_record)

    return prices

def analyze_manufacturer(name, path):
    """Analyze pricing for a manufacturer"""
    print(f"\n{'='*80}")
    print(f"ANALYZING: {name}")
    print(f"Path: {path}")
    print(f"{'='*80}\n")

    output = run_ebase_dump(path, 'ocd_price')
    prices = parse_price_records(output)

    print(f"Total price records: {len(prices)}")

    # Separate by level
    base_prices = [p for p in prices if p.get('price_level') == 'B']
    surcharges = [p for p in prices if p.get('price_level') == 'X']
    corrupt = [p for p in prices if p.get('price_level') not in ['B', 'X']]

    print(f"\nBase prices (level B): {len(base_prices)}")
    print(f"Surcharges (level X): {len(surcharges)}")
    print(f"Corrupt/Unknown: {len(corrupt)}")

    # Show base prices
    if base_prices:
        print(f"\n{'-'*80}")
        print("BASE PRICES (level B):")
        print(f"{'-'*80}")
        for p in base_prices:
            article = p.get('article_nr', 'N/A')
            price = p.get('price', 'N/A')
            var_cond = p.get('var_cond', '')
            currency = p.get('currency', 'N/A')
            print(f"  Article: {article:30} Price: {price:>10} {currency:>5}  VarCond: '{var_cond}'")

    # Show surcharges summary
    if surcharges:
        print(f"\n{'-'*80}")
        print("SURCHARGES (level X) - First 10:")
        print(f"{'-'*80}")
        for p in surcharges[:10]:
            article = p.get('article_nr', 'N/A')
            price = p.get('price', 'N/A')
            var_cond = p.get('var_cond', 'N/A')
            currency = p.get('currency', 'N/A')
            print(f"  Article: {article:30} Price: {price:>10} {currency:>5}  VarCond: {var_cond}")

    # Show corrupt records
    if corrupt:
        print(f"\n{'-'*80}")
        print("CORRUPT/UNKNOWN RECORDS:")
        print(f"{'-'*80}")
        for i, p in enumerate(corrupt):
            print(f"\n  Record {i}:")
            for key, value in p.items():
                if len(str(value)) > 100:
                    value = str(value)[:100] + "..."
                print(f"    {key}: {value}")

    return {
        'name': name,
        'total': len(prices),
        'base': len(base_prices),
        'surcharges': len(surcharges),
        'corrupt': len(corrupt),
        'base_prices': base_prices
    }

def main():
    manufacturers = [
        ('Framery 2Q', '/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase'),
        ('Framery ONE', '/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase'),
    ]

    results = []
    for name, path in manufacturers:
        result = analyze_manufacturer(name, path)
        results.append(result)

    # Summary
    print(f"\n\n{'='*80}")
    print("SUMMARY")
    print(f"{'='*80}\n")

    for r in results:
        print(f"{r['name']:20} - Total: {r['total']:3}  Base: {r['base']:3}  Surcharges: {r['surcharges']:3}  Corrupt: {r['corrupt']:3}")

    # Key findings
    print(f"\n{'='*80}")
    print("KEY FINDINGS")
    print(f"{'='*80}\n")

    for r in results:
        if r['base'] == 0:
            print(f"WARNING: {r['name']} has NO base prices!")
        else:
            print(f"OK: {r['name']} has {r['base']} base price(s)")
            for bp in r['base_prices']:
                article = bp.get('article_nr', 'N/A')
                price = bp.get('price', 'N/A')
                currency = bp.get('currency', 'N/A')
                print(f"    {article}: {price} {currency}")

if __name__ == '__main__':
    main()
