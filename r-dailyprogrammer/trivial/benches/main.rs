use criterion::*;
use std::time::*;
use trivial::sorting::*;
use trivial::*;

mod gens {
    //! Module which implements some sequence generators.

    use rand::prelude::*;

    fn ones(size: usize) -> Vec<u32> {
        vec![1; size]
    }
    
    fn sorted_range(max: u32) -> Vec<u32> {
        (0..max).rev().collect()
    }
    
    fn reversed_range(max: u32) -> Vec<u32> {
        (0..max).rev().collect()
    }

    fn shuffled_range(max: u32) -> Vec<u32> {
        let mut v = (0..max).collect();
        v.shuffle(&mut rand::thread_rng());
        v
    }
}

fn sort_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("sort-ones");
    g.measurement_time(Duration::new(60, 0));
    g.bench_function("built-in", |b| b.iter(
}

fn fizzbuzz_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("fizzbuzz");
    g.measurement_time(Duration::new(60, 0));
    g.bench_function("fold-write", |b| b.iter(|| fizzbuzz_folder_write()));
    g.bench_function("fold-to-string", |b| b.iter(|| fizzbuzz_folder_to_string()));
}

// fn bench_with_io(c: &mut Criterion) {
//     let input = include_str!("../inputs/yahtzee-upper-1.txt");
//     let mut g = c.benchmark_group("io");
//     g.measurement_time(Duration::new(60 * 2, 0));
//     g.bench_function("decode", |b| b.iter(|| yahtzee_decode(input)));
//     // g.bench_function("decode", |b| b.iter(|| yahtzee_decode(input)));
// }

criterion_group!(benches, fizzbuzz_bench);
criterion_main!(benches);
