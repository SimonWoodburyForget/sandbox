use criterion::*;
use std::time::*;
use yahtzee::*;

fn input() -> Vec<u32> {
    include_str!("../inputs/yahtzee-upper-1.txt")
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn small_bench(c: &mut Criterion) {
    c.bench_function("hashmap", |b| b.iter(|| yahtzee_hashmap(&[6, 6, 1, 6, 6])));
    c.bench_function("array", |b| b.iter(|| yahtzee_small([3, 6, 1, 6, 6])));
    c.bench_function("vector", |b| b.iter(|| yahtzee_vec(&[3, 6, 1, 6, 6])));
}

fn bench(c: &mut Criterion) {
    let input = input();
    let mut g = c.benchmark_group("100_000");
    g.measurement_time(Duration::new(60 * 2, 0));
    g.bench_function("vector", |b| b.iter(|| yahtzee_vec(&input)));
    g.bench_function("hashmap", |b| b.iter(|| yahtzee_hashmap(&input)));
    g.bench_function("btree", |b| b.iter(|| yahtzee_btree(&input)));
}

fn bench_half(c: &mut Criterion) {
    let input = &input()[..50_000];
    let mut g = c.benchmark_group("50_000");
    g.measurement_time(Duration::new(60, 0));
    g.bench_function("vector", |b| b.iter(|| yahtzee_vec(input)));
    g.bench_function("hashmap", |b| b.iter(|| yahtzee_hashmap(input)));
    g.bench_function("btree", |b| b.iter(|| yahtzee_btree(input)));
}

fn bench_with_io(c: &mut Criterion) {
    let input = include_str!("../inputs/yahtzee-upper-1.txt");
    let mut g = c.benchmark_group("io");
    g.measurement_time(Duration::new(60 * 2, 0));
    g.bench_function("decode", |b| b.iter(|| yahtzee_decode(input)));
    // g.bench_function("decode", |b| b.iter(|| yahtzee_decode(input)));
}

criterion_group!(benches, bench_with_io);
criterion_main!(benches);
