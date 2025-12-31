//! Benchmarks for EBASE expression evaluation.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ofml_interpreter::ebase_expr::EbaseEvaluator;
use std::collections::HashMap;

fn bench_simple_import(c: &mut Criterion) {
    let mut evaluator = EbaseEvaluator::new();
    let props = HashMap::new();

    c.bench_function("simple_import", |b| {
        b.iter(|| {
            evaluator
                .evaluate(black_box(r#""table_top" 1 1 1 imp"#), &props)
                .unwrap()
        })
    });
}

fn bench_variable_substitution(c: &mut Criterion) {
    let mut evaluator = EbaseEvaluator::new();
    let mut props = HashMap::new();
    props.insert("M__BREITE".to_string(), 1600.0);
    props.insert("M__TIEFE".to_string(), 800.0);
    props.insert("M__HOEHE".to_string(), 720.0);

    c.bench_function("variable_substitution", |b| {
        b.iter(|| {
            evaluator
                .evaluate(
                    black_box(r#""panel" ${M__BREITE:-100} 1000 / ${M__TIEFE:-100} 1000 / 1 imp"#),
                    &props,
                )
                .unwrap()
        })
    });
}

fn bench_arithmetic_expression(c: &mut Criterion) {
    let mut evaluator = EbaseEvaluator::new();
    let props = HashMap::new();

    c.bench_function("arithmetic_expression", |b| {
        b.iter(|| {
            evaluator
                .evaluate(
                    black_box(r#""test" 100 200 + 300 * 1000 / 1 1 imp"#),
                    &props,
                )
                .unwrap()
        })
    });
}

fn bench_conditional_expression(c: &mut Criterion) {
    let mut evaluator = EbaseEvaluator::new();
    let props = HashMap::new();

    c.bench_function("conditional_expression", |b| {
        b.iter(|| {
            evaluator
                .evaluate(
                    black_box(r#"1 1 == { "true_case" } { "false_case" } ifelse 1 1 1 imp"#),
                    &props,
                )
                .unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_simple_import,
    bench_variable_substitution,
    bench_arithmetic_expression,
    bench_conditional_expression
);
criterion_main!(benches);
