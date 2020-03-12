use criterion::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use necklace_matching::*;

const ENABLE1: &str = include_str!("../inputs/enable1.txt");

pub fn bench(c: &mut Criterion) {
    let words: Vec<&str> = ENABLE1.trim().split("\n").collect();
    let mut data = words.windows(2).cycle();

    // let mut group = c.benchmark_group("is_necklace");
    // group.warm_up_time(Duration::new(2, 0));
    // group.sample_size(3_000);
    // group.measurement_time(Duration::new(40, 0));

    // let mut rng = rand::thread_rng();
    // group.bench_function("simple", |b| {
    //     b.iter(|| {
    //         let ab = data.next().unwrap();
    //         simple::is_necklace(ab[0], ab[1])
    //     })
    //     // b.iter(|| simple::is_necklace("abbbbb", "babbbb"))
    // });
    // group.bench_function("manual", |b| {
    //     b.iter(|| {
    //         let ab = data.next().unwrap();
    //         manual::is_necklace(ab[0], ab[1])
    //     })
    // });
    // group.bench_function("slicer", |b| {
    //     b.iter(|| {
    //         let ab = data.next().unwrap();
    //         slicer::is_necklace(ab[0], ab[1])
    //     })
    // });

    // {
    //     let mut group = c.benchmark_group("canon");
    //     let mut data = words.iter().cycle();

    //     group.warm_up_time(Duration::new(2, 0));
    //     group.sample_size(3_000);
    //     group.measurement_time(Duration::new(40, 0));

    //     group.bench_function("faster", |b| {
    //         b.iter(|| {
    //             let s = data.next().unwrap();
    //             slicer::canonicalize(s)
    //         })
    //         // b.iter(|| simple::is_necklace("abbbbb", "babbbb"))
    //     });

    //     group.finish();
    // }

    {
        let mut group = c.benchmark_group("solution");
        group.warm_up_time(Duration::new(3, 0));
        group.sample_size(10);
        group.measurement_time(Duration::new(20, 0));

        // group.bench_function("simple", |b| {
        //     b.iter(|| {
        //         simple::find_the_four(words.iter().cloned().collect());
        //     })
        //     // b.iter(|| simple::is_necklace("abbbbb", "babbbb"))
        // });

        let mut data: Vec<&str> = words.iter().cloned().collect();
        group.bench_function("faster", |b| {
            b.iter(|| {
                slicer::find_the_four(&data);
            })
            // b.iter(|| simple::is_necklace("abbbbb", "babbbb"))
        });

        group.finish();
    }
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

criterion_group!(benches, bench);
criterion_main!(benches);
