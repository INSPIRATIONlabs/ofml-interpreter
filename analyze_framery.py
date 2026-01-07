#!/usr/bin/env python3
"""Analyze Framery data structure to understand property class organization."""

import subprocess
import json
import re
from pathlib import Path

FRAMERY_BASE = Path("/reference/ofmldata/framery")

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
        print(f"Error running ebase: {e}")
        return ""

def extract_property_classes(output):
    """Extract unique property classes from ocd_property output."""
    classes = set()
    for line in output.split('\n'):
        if "prop_class:" in line:
            match = re.search(r"prop_class: '([^']+)'", line)
            if match:
                classes.add(match.group(1))
    return sorted(classes)

def extract_articles(output):
    """Extract articles from ocd_article output."""
    articles = []
    current_article = {}

    for line in output.split('\n'):
        if line.startswith("Record"):
            if current_article:
                articles.append(current_article)
            current_article = {}
        elif "article_nr:" in line:
            match = re.search(r"article_nr: '([^']+)'", line)
            if match:
                current_article['article_nr'] = match.group(1)
        elif "series:" in line:
            match = re.search(r"series: '([^']+)'", line)
            if match:
                current_article['series'] = match.group(1)

    if current_article:
        articles.append(current_article)

    return articles

def extract_propertyclass_mappings(output):
    """Extract property class mappings from ocd_propertyclass output."""
    mappings = []
    current_mapping = {}

    for line in output.split('\n'):
        if line.startswith("Record"):
            if current_mapping:
                mappings.append(current_mapping)
            current_mapping = {}
        elif "article_nr:" in line:
            match = re.search(r"article_nr: '([^']+)'", line)
            if match:
                current_mapping['article_nr'] = match.group(1)
        elif "prop_class:" in line:
            match = re.search(r"prop_class: '([^']+)'", line)
            if match:
                current_mapping['prop_class'] = match.group(1)

    if current_mapping:
        mappings.append(current_mapping)

    return mappings

def main():
    print("=" * 80)
    print("FRAMERY DATA STRUCTURE INVESTIGATION")
    print("=" * 80)

    # Find all pdata.ebase files
    pdata_files = sorted(FRAMERY_BASE.glob("*/ANY/1/db/pdata.ebase"))

    print(f"\nFound {len(pdata_files)} series:\n")

    all_series_data = {}

    for pdata_file in pdata_files:
        series_name = pdata_file.parts[-5]  # Extract series from path
        print(f"\n{'=' * 80}")
        print(f"SERIES: {series_name}")
        print(f"Path: {pdata_file}")
        print(f"{'=' * 80}")

        # Get property classes
        prop_output = run_ebase_command(pdata_file, "ocd_property")
        prop_classes = extract_property_classes(prop_output)

        print(f"\nProperty Classes ({len(prop_classes)}):")
        for pc in prop_classes:
            print(f"  - {pc}")

        # Get articles
        article_output = run_ebase_command(pdata_file, "ocd_article")
        articles = extract_articles(article_output)

        print(f"\nArticles ({len(articles)}):")
        for art in articles:
            print(f"  - {art.get('article_nr', 'N/A')} (series: {art.get('series', 'N/A')})")

        # Get property class mappings
        propclass_output = run_ebase_command(pdata_file, "ocd_propertyclass")
        mappings = extract_propertyclass_mappings(propclass_output)

        print(f"\nProperty Class Mappings ({len(mappings)}):")
        for mapping in mappings[:10]:  # Show first 10
            print(f"  - Article '{mapping.get('article_nr', 'N/A')}' -> Class '{mapping.get('prop_class', 'N/A')}'")
        if len(mappings) > 10:
            print(f"  ... and {len(mappings) - 10} more")

        all_series_data[series_name] = {
            'path': str(pdata_file),
            'property_classes': prop_classes,
            'articles': articles,
            'mappings': mappings
        }

    # Summary
    print(f"\n{'=' * 80}")
    print("SUMMARY")
    print(f"{'=' * 80}")

    # Check if property classes are shared
    all_prop_classes = set()
    for series_data in all_series_data.values():
        all_prop_classes.update(series_data['property_classes'])

    print(f"\nUnique property classes across ALL series: {sorted(all_prop_classes)}")

    # Check if all series use the same property classes
    if len(all_series_data) > 0:
        first_series_classes = set(list(all_series_data.values())[0]['property_classes'])
        all_same = all(
            set(data['property_classes']) == first_series_classes
            for data in all_series_data.values()
        )

        if all_same:
            print("\n** ALL SERIES USE THE SAME PROPERTY CLASSES **")
            print("   This means property classes are SHARED across series, not series-specific!")
        else:
            print("\n** SERIES USE DIFFERENT PROPERTY CLASSES **")
            print("   Property classes are series-specific")

            # Show differences
            for series_name, data in all_series_data.items():
                classes = set(data['property_classes'])
                print(f"\n  {series_name}: {sorted(classes)}")

if __name__ == "__main__":
    main()
