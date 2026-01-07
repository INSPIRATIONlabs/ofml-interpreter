//! Benchmarks for product family loading with parallel processing.
//!
//! These benchmarks measure the performance of loading manufacturer data,
//! including EBase parsing, OAM loading, and article grouping.
//!
//! Run with: `cargo bench --bench family_loading_bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::path::Path;

/// Benchmark loading a single manufacturer's families
/// This measures the full parallel loading pipeline:
/// - Property loading (parallel EBase parsing)
/// - OAM loading (parallel)
/// - Article loading and grouping (parallel)
fn bench_family_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("family_loading");

    // Configure for longer measurement time due to I/O
    group.sample_size(20);
    group.measurement_time(std::time::Duration::from_secs(10));

    // Test with available manufacturers
    let manufacturers = [
        ("/reference/ofmldata/sex", "Sedus"),
        ("/reference/ofmldata/bisley", "Bisley"),
        ("/reference/ofmldata/vitra", "Vitra"),
        ("/reference/ofmldata/frmr", "Framery"),
    ];

    for (path, name) in manufacturers {
        let mfr_path = Path::new(path);
        if !mfr_path.exists() {
            continue;
        }

        group.bench_with_input(
            BenchmarkId::new("load_families", name),
            &mfr_path,
            |b, path| {
                b.iter(|| {
                    ofml_lib::oap::families::FamilyLoader::load(
                        black_box(path),
                        "DE",
                    )
                })
            },
        );
    }

    group.finish();
}

/// Benchmark loading OCD properties only (parallel EBase parsing)
fn bench_property_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("property_loading");
    group.sample_size(20);
    group.measurement_time(std::time::Duration::from_secs(10));

    let manufacturers = [
        ("/reference/ofmldata/sex", "Sedus"),
        ("/reference/ofmldata/bisley", "Bisley"),
    ];

    for (path, name) in manufacturers {
        let mfr_path = Path::new(path);
        if !mfr_path.exists() {
            continue;
        }

        group.bench_with_input(
            BenchmarkId::new("load_properties", name),
            &mfr_path,
            |b, path| {
                b.iter(|| {
                    ofml_lib::oap::ocd_properties::load_manufacturer_properties(
                        black_box(path),
                    )
                })
            },
        );
    }

    group.finish();
}

/// Benchmark single EBase file reading (baseline for parallel comparison)
fn bench_ocd_reader(c: &mut Criterion) {
    let pdata_path = Path::new("/reference/ofmldata/sex/ai/DE/1/db/pdata.ebase");
    if !pdata_path.exists() {
        return;
    }

    c.bench_function("ocd_reader_single_file", |b| {
        b.iter(|| {
            ofml_lib::oap::ocd::OcdReader::from_ebase(black_box(pdata_path))
        })
    });
}

/// Benchmark article loading with descriptions (parallel)
fn bench_article_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("article_loading");
    group.sample_size(20);

    let manufacturers = [
        ("/reference/ofmldata/sex", "Sedus"),
        ("/reference/ofmldata/bisley", "Bisley"),
    ];

    for (path, name) in manufacturers {
        let mfr_path = Path::new(path);
        if !mfr_path.exists() {
            continue;
        }

        group.bench_with_input(
            BenchmarkId::new("load_articles", name),
            &mfr_path,
            |b, path| {
                b.iter(|| {
                    ofml_lib::oap::ocd::load_articles_with_full_descriptions(
                        black_box(path),
                        "DE",
                    )
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_family_loading,
    bench_property_loading,
    bench_ocd_reader,
    bench_article_loading,
);
criterion_main!(benches);
