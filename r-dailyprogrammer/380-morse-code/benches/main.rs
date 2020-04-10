use morse::*;

use criterion::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

fn bench_one(c: &mut Criterion) {
    let mut group = c.benchmark_group("bonus::one");
    group
        .warm_up_time(Duration::new(5, 0))
        .sample_size(80)
        .measurement_time(Duration::new(300, 0))
        .bench_function("bitter", |b| b.iter(|| bonus::one()));
    group.finish();
}

fn bench_two(c: &mut Criterion) {
    let mut group = c.benchmark_group("bonus::two");
    group
        .warm_up_time(Duration::new(5, 0))
        .sample_size(80)
        .measurement_time(Duration::new(100, 0))
        .bench_function("bitter", |b| b.iter(|| bonus::two()));
    group.finish();
}

fn bench_three(c: &mut Criterion) {
    let mut group = c.benchmark_group("bonus::three");
    group
        .warm_up_time(Duration::new(5, 0))
        .sample_size(80)
        .measurement_time(Duration::new(30, 0))
        .bench_function("bitter", |b| b.iter(|| bonus::three()));
    group.finish();
}

fn bench_four(c: &mut Criterion) {
    let mut group = c.benchmark_group("bonus::four");
    group
        .warm_up_time(Duration::new(5, 0))
        .sample_size(80)
        .measurement_time(Duration::new(30, 0))
        .bench_function("bitter", |b| b.iter(|| bonus::four()));
    group.finish();
}

fn bench_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("codes::gen");
    group
        .warm_up_time(Duration::new(5, 0))
        .sample_size(70)
        .measurement_time(Duration::new(30, 0))
        .bench_function("enable-1", |b| b.iter(|| codes::enable_1()));
    group.finish();
}

// criterion_group!(benches, bench_one, bench_two, bench_three, bench_four);
criterion_group!(benches, bench_gen);
criterion_main!(benches);
