//! Snapshot tests for Vitra Classic data parsing
//!
//! Tests that Vitra Classic data is parsed correctly and detect regressions.
//! Note: Vitra Classic has known data corruption issues (swapped fields, binary garbage).

use insta::assert_json_snapshot;
use ofml_lib::oap::families::FamilyLoader;
use ofml_lib::oap::ocd::OcdReader;
use serde::Serialize;
use std::path::Path;

const VITRA_CLASSIC_EBASE: &str = "/reference/ofmldata/vitra/classic/DE/1/db/pdata.ebase";
const VITRA_CLASSIC_PATH: &str = "/reference/ofmldata/vitra/classic";

fn vitra_classic_exists() -> bool {
    Path::new(VITRA_CLASSIC_EBASE).exists()
}

/// Family snapshot data
#[derive(Serialize)]
struct FamilySnapshot {
    id: String,
    name: String,
    base_article_nr: String,
    variant_count: usize,
    is_configurable: bool,
    prop_class_count: usize,
}

/// Data quality stats
#[derive(Serialize)]
struct DataQualityStats {
    total_articles: usize,
    valid_series_articles: usize,
    corrupt_series_articles: usize,
    articles_with_prop_class: usize,
    base_prices: usize,
    surcharges: usize,
    family_count: usize,
}

#[test]
fn snapshot_vitra_classic_families() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let loader = FamilyLoader::load(Path::new(VITRA_CLASSIC_PATH), "DE");
    let families = loader.get_families();

    let snapshots: Vec<FamilySnapshot> = families
        .iter()
        .map(|f| FamilySnapshot {
            id: f.id.clone(),
            name: f.name.clone(),
            base_article_nr: f.base_article_nr.clone(),
            variant_count: f.variant_count,
            is_configurable: f.is_configurable,
            prop_class_count: f.prop_classes.len(),
        })
        .collect();

    assert_json_snapshot!("vitra_classic_families", snapshots);
}

#[test]
fn snapshot_vitra_classic_data_quality() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(VITRA_CLASSIC_EBASE)).unwrap();
    let loader = FamilyLoader::load(Path::new(VITRA_CLASSIC_PATH), "DE");

    let valid_series = reader
        .articles
        .iter()
        .filter(|a| {
            !a.series.chars().any(|c| c.is_control())
                && a.series
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        })
        .count();

    let corrupt_series = reader.articles.len() - valid_series;

    let stats = DataQualityStats {
        total_articles: reader.articles.len(),
        valid_series_articles: valid_series,
        corrupt_series_articles: corrupt_series,
        articles_with_prop_class: reader.article_prop_classes.len(),
        base_prices: reader.prices.iter().filter(|p| p.price_level == "B").count(),
        surcharges: reader.prices.iter().filter(|p| p.price_level == "X").count(),
        family_count: loader.get_families().len(),
    };

    assert_json_snapshot!("vitra_classic_data_quality", stats);
}

#[test]
fn snapshot_vitra_classic_c_family_articles() {
    if !vitra_classic_exists() {
        eprintln!("Skipping: Vitra classic data not found");
        return;
    }

    let loader = FamilyLoader::load(Path::new(VITRA_CLASSIC_PATH), "DE");
    let families = loader.get_families();

    // Get the C family (largest family with most property classes)
    if let Some(c_fam) = families.iter().find(|f| f.id == "C") {
        #[derive(Serialize)]
        struct CFamilySnapshot {
            article_count: usize,
            first_10_articles: Vec<String>,
            prop_class_count: usize,
            first_5_prop_classes: Vec<String>,
        }

        let snapshot = CFamilySnapshot {
            article_count: c_fam.article_nrs.len(),
            first_10_articles: c_fam.article_nrs.iter().take(10).cloned().collect(),
            prop_class_count: c_fam.prop_classes.len(),
            first_5_prop_classes: c_fam.prop_classes.iter().take(5).cloned().collect(),
        };

        assert_json_snapshot!("vitra_classic_c_family", snapshot);
    }
}
