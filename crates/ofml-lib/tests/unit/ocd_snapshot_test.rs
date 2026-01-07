//! Snapshot tests for OCD data parsing
//!
//! Uses insta for snapshot testing to detect regressions in OCD parsing.

use insta::assert_json_snapshot;
use ofml_lib::oap::ocd::OcdReader;
use serde::Serialize;
use std::path::Path;

const SEDUS_AI_PATH: &str = "/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase";

/// Simplified article for snapshot (to avoid internal details)
#[derive(Serialize)]
struct ArticleSnapshot {
    article_nr: String,
    series: String,
    art_type: String,
}

/// Simplified price for snapshot
#[derive(Serialize)]
struct PriceSnapshot {
    article_nr: String,
    price_level: String,
    var_cond: String,
    amount: f32,
    currency: String,
}

fn sedus_ai_exists() -> bool {
    Path::new(SEDUS_AI_PATH).exists()
}

#[test]
fn snapshot_sedus_ai_articles() {
    if !sedus_ai_exists() {
        eprintln!("Skipping: Sedus AI data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(SEDUS_AI_PATH)).unwrap();

    // Take first 5 articles for snapshot
    let articles: Vec<ArticleSnapshot> = reader
        .articles
        .iter()
        .take(5)
        .map(|a| ArticleSnapshot {
            article_nr: a.article_nr.clone(),
            series: a.series.clone(),
            art_type: a.art_type.clone(),
        })
        .collect();

    assert_json_snapshot!("sedus_ai_articles", articles);
}

#[test]
fn snapshot_sedus_ai_base_prices() {
    if !sedus_ai_exists() {
        eprintln!("Skipping: Sedus AI data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(SEDUS_AI_PATH)).unwrap();

    // Get base prices (level B) for first 5 articles
    let prices: Vec<PriceSnapshot> = reader
        .prices
        .iter()
        .filter(|p| p.price_level == "B")
        .take(5)
        .map(|p| PriceSnapshot {
            article_nr: p.article_nr.clone(),
            price_level: p.price_level.clone(),
            var_cond: p.var_cond.clone(),
            amount: p.price,
            currency: p.currency.clone(),
        })
        .collect();

    assert_json_snapshot!("sedus_ai_base_prices", prices);
}

#[test]
fn snapshot_sedus_ai_surcharges() {
    if !sedus_ai_exists() {
        eprintln!("Skipping: Sedus AI data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(SEDUS_AI_PATH)).unwrap();

    // Get surcharges (level X) - first 10
    let surcharges: Vec<PriceSnapshot> = reader
        .prices
        .iter()
        .filter(|p| p.price_level == "X")
        .take(10)
        .map(|p| PriceSnapshot {
            article_nr: p.article_nr.clone(),
            price_level: p.price_level.clone(),
            var_cond: p.var_cond.clone(),
            amount: p.price,
            currency: p.currency.clone(),
        })
        .collect();

    assert_json_snapshot!("sedus_ai_surcharges", surcharges);
}

#[test]
fn snapshot_ocd_reader_stats() {
    if !sedus_ai_exists() {
        eprintln!("Skipping: Sedus AI data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(SEDUS_AI_PATH)).unwrap();

    #[derive(Serialize)]
    struct OcdStats {
        article_count: usize,
        price_count: usize,
        base_price_count: usize,
        surcharge_count: usize,
        short_text_count: usize,
        propvalue2varcond_count: usize,
        unique_currencies: Vec<String>,
    }

    let mut currencies: Vec<String> = reader
        .prices
        .iter()
        .map(|p| p.currency.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    currencies.sort();

    let stats = OcdStats {
        article_count: reader.articles.len(),
        price_count: reader.prices.len(),
        base_price_count: reader.prices.iter().filter(|p| p.price_level == "B").count(),
        surcharge_count: reader.prices.iter().filter(|p| p.price_level == "X").count(),
        short_text_count: reader.short_texts.len(),
        propvalue2varcond_count: reader.propvalue2varcond.len(),
        unique_currencies: currencies,
    };

    assert_json_snapshot!("sedus_ai_ocd_stats", stats);
}

#[test]
fn snapshot_article_property_classes() {
    if !sedus_ai_exists() {
        eprintln!("Skipping: Sedus AI data not found");
        return;
    }

    let reader = OcdReader::from_ebase(Path::new(SEDUS_AI_PATH)).unwrap();

    // Get first 5 article -> property class mappings
    let mut mappings: Vec<(String, Vec<String>)> = reader
        .article_prop_classes
        .iter()
        .take(5)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    mappings.sort_by(|a, b| a.0.cmp(&b.0));

    assert_json_snapshot!("sedus_ai_article_prop_classes", mappings);
}
