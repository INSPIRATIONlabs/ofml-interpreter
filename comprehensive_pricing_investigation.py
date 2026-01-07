#!/usr/bin/env python3
"""
Comprehensive Multi-Manufacturer Pricing Investigation
Analyzes pricing patterns across multiple manufacturers to find common structures.
"""

import subprocess
import json
import sys
from pathlib import Path
from collections import defaultdict

def run_ofml_command(db_path, table_name):
    """Run OFML ebase command and return parsed JSON"""
    try:
        cmd = [
            "/workspace/target/release/ofml-interpreter",
            "ebase",
            db_path,
            table_name,
            "--json"
        ]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        if result.returncode == 0 and result.stdout.strip():
            return json.loads(result.stdout)
        return None
    except Exception as e:
        print(f"Error running command for {db_path}/{table_name}: {e}", file=sys.stderr)
        return None

def analyze_manufacturer(mfr_name, series_name, db_path):
    """Analyze pricing patterns for a single manufacturer/series"""
    print(f"\n{'='*80}")
    print(f"ANALYZING: {mfr_name}/{series_name}")
    print(f"Path: {db_path}")
    print(f"{'='*80}")

    results = {
        'manufacturer': mfr_name,
        'series': series_name,
        'db_path': db_path,
        'base_prices': [],
        'surcharges': [],
        'has_propvalue2varcond': False,
        'propvalue2varcond_samples': [],
        'property_classes': [],
        'property_values': [],
        'articles': []
    }

    # 1. Check ocd_price table
    print("\n--- OCD_PRICE TABLE ---")
    price_data = run_ofml_command(db_path, "ocd_price")
    if price_data:
        base_count = 0
        surcharge_count = 0
        var_cond_patterns = defaultdict(int)

        for record in price_data:
            level = record.get('level', '')
            var_cond = record.get('var_cond', '')
            article_nr = record.get('article_nr', '')
            price = record.get('price', 0)

            if level == 'B':
                base_count += 1
                var_cond_patterns[f"BASE:{var_cond}"] += 1
                if base_count <= 10:  # Sample first 10
                    results['base_prices'].append({
                        'article_nr': article_nr,
                        'var_cond': var_cond,
                        'price': price
                    })
            elif level == 'X':
                surcharge_count += 1
                var_cond_patterns[f"SURCH:{var_cond}"] += 1
                if surcharge_count <= 10:  # Sample first 10
                    results['surcharges'].append({
                        'article_nr': article_nr,
                        'var_cond': var_cond,
                        'price': price
                    })

        print(f"Total base prices (level B): {base_count}")
        print(f"Total surcharges (level X): {surcharge_count}")
        print(f"\nvar_cond patterns (top 20):")
        for pattern, count in sorted(var_cond_patterns.items(), key=lambda x: x[1], reverse=True)[:20]:
            print(f"  {pattern}: {count}")

        if results['base_prices']:
            print(f"\nSample base prices:")
            for bp in results['base_prices'][:5]:
                print(f"  article_nr={bp['article_nr']}, var_cond='{bp['var_cond']}', price={bp['price']}")

        if results['surcharges']:
            print(f"\nSample surcharges:")
            for sc in results['surcharges'][:5]:
                print(f"  article_nr={sc['article_nr']}, var_cond='{sc['var_cond']}', price={sc['price']}")

    # 2. Check ocd_propvalue2varcond table
    print("\n--- OCD_PROPVALUE2VARCOND TABLE ---")
    p2v_data = run_ofml_command(db_path, "ocd_propvalue2varcond")
    if p2v_data:
        results['has_propvalue2varcond'] = True
        print(f"Table EXISTS! Found {len(p2v_data)} mappings")
        for record in p2v_data[:10]:
            results['propvalue2varcond_samples'].append(record)
        print("Sample mappings:")
        for rec in results['propvalue2varcond_samples'][:5]:
            print(f"  {rec}")
    else:
        print("Table does NOT exist or is empty")

    # 3. Check ocd_propertyclass
    print("\n--- OCD_PROPERTYCLASS TABLE ---")
    propclass_data = run_ofml_command(db_path, "ocd_propertyclass")
    if propclass_data:
        article_specific = sum(1 for r in propclass_data if r.get('article_nr') != '*')
        wildcard = sum(1 for r in propclass_data if r.get('article_nr') == '*')
        print(f"Total property class mappings: {len(propclass_data)}")
        print(f"  Article-specific: {article_specific}")
        print(f"  Wildcard (*): {wildcard}")

        # Sample mappings
        for rec in propclass_data[:5]:
            results['property_classes'].append(rec)
            print(f"  article_nr={rec.get('article_nr')}, property_class={rec.get('property_class')}")

    # 4. Check ocd_propertyvalue
    print("\n--- OCD_PROPERTYVALUE TABLE ---")
    propval_data = run_ofml_command(db_path, "ocd_propertyvalue")
    if propval_data:
        print(f"Total property values: {len(propval_data)}")
        property_classes = set(r.get('property_class') for r in propval_data)
        print(f"Unique property classes: {len(property_classes)}")

        # Sample values
        for rec in propval_data[:10]:
            results['property_values'].append(rec)
        print("Sample property values:")
        for rec in results['property_values'][:5]:
            print(f"  class={rec.get('property_class')}, property={rec.get('property')}, value={rec.get('value')}")

    # 5. Check ocd_article
    print("\n--- OCD_ARTICLE TABLE ---")
    article_data = run_ofml_command(db_path, "ocd_article")
    if article_data:
        print(f"Total articles: {len(article_data)}")
        for rec in article_data[:5]:
            results['articles'].append(rec)
            print(f"  article_nr={rec.get('article_nr')}, description={rec.get('description')}")

    return results

def main():
    # Build the OFML tool first
    print("Building OFML tool...")
    subprocess.run(["cargo", "build", "--release"], cwd="/workspace", check=True)

    # Define manufacturers to investigate
    manufacturers = [
        # Framery - phone booths (should be thousands of EUR)
        ('framery', 'frmr_one', '/reference/ofmldata/framery/frmr_one/ANY/1/db/pdata.ebase'),
        ('framery', 'frmr_2q', '/reference/ofmldata/framery/frmr_2q/ANY/1/db/pdata.ebase'),
        ('framery', 'frmr_q', '/reference/ofmldata/framery/frmr_q/ANY/1/db/pdata.ebase'),

        # FAST - wall decorations
        ('fast', 'kr', '/reference/ofmldata/fast/kr/DE/1/db/pdata.ebase'),
        ('fast', 'wkm', '/reference/ofmldata/fast/wkm/DE/1/db/pdata.ebase'),

        # Sedus - office chairs
        ('sedus', 'ai', '/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase'),

        # Additional manufacturers for comparison
        ('bisley', 'systemfile', '/reference/ofmldata/bisley/systemfile/DE/1/db/pdata.ebase'),
        ('arper', 'catifa46', '/reference/ofmldata/arper/catifa46/DE/1/db/pdata.ebase'),
    ]

    all_results = []

    for mfr, series, db_path in manufacturers:
        if Path(db_path).exists():
            try:
                result = analyze_manufacturer(mfr, series, db_path)
                all_results.append(result)
            except Exception as e:
                print(f"ERROR analyzing {mfr}/{series}: {e}", file=sys.stderr)
        else:
            print(f"SKIPPING {mfr}/{series}: {db_path} does not exist")

    # Generate comparison table
    print("\n" + "="*120)
    print("PRICING PATTERNS COMPARISON TABLE")
    print("="*120)
    print(f"{'Manufacturer':<15} {'Series':<20} {'Base Pattern':<30} {'Surch Pattern':<30} {'propvalue2varcond?':<20}")
    print("-"*120)

    for r in all_results:
        base_pattern = "UNKNOWN"
        if r['base_prices']:
            var_conds = set(bp['var_cond'] for bp in r['base_prices'])
            base_pattern = ', '.join(sorted(var_conds)[:3])
            if len(var_conds) > 3:
                base_pattern += '...'

        surch_pattern = "UNKNOWN"
        if r['surcharges']:
            var_conds = set(sc['var_cond'] for sc in r['surcharges'][:10])
            surch_pattern = ', '.join(sorted(var_conds)[:2])
            if len(var_conds) > 2:
                surch_pattern += '...'

        has_p2v = "YES" if r['has_propvalue2varcond'] else "NO"

        print(f"{r['manufacturer']:<15} {r['series']:<20} {base_pattern:<30} {surch_pattern:<30} {has_p2v:<20}")

    # Save detailed results to JSON
    with open('/workspace/pricing_investigation_results.json', 'w') as f:
        json.dump(all_results, f, indent=2)
    print("\nDetailed results saved to: /workspace/pricing_investigation_results.json")

if __name__ == '__main__':
    main()
