use criterion::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use necklace_matching::*;

const ENABLE1: &str = include_str!("../inputs/enable1.txt");

pub fn words() -> Vec<&'static str> {
    ENABLE1.trim().split("\n").collect()
}

pub fn bench_is_necklace(c: &mut Criterion) {
    let words = words();
    let mut data = words.windows(2).cycle();
    let mut group = c.benchmark_group("is_necklace");
    group.warm_up_time(Duration::new(5, 0));
    group.measurement_time(Duration::new(25, 0));
    group.bench_function("fast", |b| {
        b.iter(|| {
            let ab = data.next().unwrap();
            is_necklace(ab[0], ab[1])
        })
    });
    group.finish();
}

pub fn bench_canonicalize(c: &mut Criterion) {
    let words = words();
    let mut data = words.iter().cycle();
    let mut group = c.benchmark_group("canon");
    group.warm_up_time(Duration::new(5, 0));
    group.measurement_time(Duration::new(40, 0));
    // group.bench_function("string", |b| {
    //     b.iter(|| {
    //         let word = data.next().unwrap();
    //         Necklace::new(word).to_string()
    //     })
    // });
    // group.bench_function("rotation", |b| {
    //     b.iter(|| {
    //         let word = data.next().unwrap();
    //         canonicalize_rotation(word).to_string()
    //     })
    // });
    group.bench_function("slices", |b| {
        b.iter(|| {
            let word = data.next().unwrap();
            canonicalize_slices(word)
        })
    });
    group.finish();
}

pub fn bench_solution(c: &mut Criterion) {
    let words = words();
    let mut data: Vec<&str> = words.iter().cloned().collect();
    let mut group = c.benchmark_group("solution");
    group.warm_up_time(Duration::new(10, 0));
    group.sample_size(30);
    group.measurement_time(Duration::new(60 * 8, 0));
    // group.bench_function("binary", |b| {
    //     b.iter(|| {
    //         find_the_four_binary(&data);
    //     })
    // });
    group.bench_function("fast", |b| {
        b.iter(|| {
            find_the_four_counters(&data);
        })
    });
    group.finish();
}

pub fn bench_full(mut c: &mut Criterion) {
    bench_is_necklace(&mut c);
    // bench_canonicalize(&mut c);
    // bench_solution(&mut c);
}

pub fn primitive(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitive");
    group.warm_up_time(Duration::new(1, 0));
    group.sample_size(1_000);
    group.measurement_time(Duration::new(2, 0));

    let mut rng = rand::thread_rng();
    group.bench_function("init/vector", |b| b.iter(|| Vec::<char>::with_capacity(20)));
    group.bench_function("init/string", |b| {
        b.iter(|| String::from("12345678901234567890"))
    });
    group.bench_function("init/vector-deque", |b| {
        b.iter(|| VecDeque::<char>::with_capacity(20))
    });
    let mut d: VecDeque<char> = "12345678901234567890".chars().collect();
    group.bench_function("rotate/vector-deque", |b| b.iter(|| d.rotate_left(1)));

    let mut hasher = DefaultHasher::new();
    group.bench_function("hasher/string", |b| {
        b.iter(|| {
            "12345678901234567890".hash(&mut hasher);
            hasher.finish()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_full);
criterion_main!(benches);
