use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use criterion::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

use necklace_matching::*;

// const ENABLE1: &str = include_str!("../inputs/enable1.txt");

// pub fn words() -> Vec<&'static str> {
//     ENABLE1.trim().split("\n").collect()
// }

// pub fn bench_is_necklace(c: &mut Criterion) {
//     let words = words();
//     let mut data = words.windows(2).cycle();
//     let mut group = c.benchmark_group("is_necklace");
//     group.warm_up_time(Duration::new(5, 0));
//     group.measurement_time(Duration::new(25, 0));
//     group.bench_function("fast", |b| {
//         b.iter(|| {
//             let ab = data.next().unwrap();
//             is_necklace(ab[0], ab[1])
//         })
//     });
//     group.finish();
// }

// pub fn bench_canonicalize(c: &mut Criterion) {
//     let words = words();
//     let mut data = words.iter().cycle();
//     let mut group = c.benchmark_group("canon");
//     group.warm_up_time(Duration::new(5, 0));
//     group.measurement_time(Duration::new(40, 0));
//     // group.bench_function("string", |b| {
//     //     b.iter(|| {
//     //         let word = data.next().unwrap();
//     //         Necklace::new(word).to_string()
//     //     })
//     // });
//     // group.bench_function("rotation", |b| {
//     //     b.iter(|| {
//     //         let word = data.next().unwrap();
//     //         canonicalize_rotation(word).to_string()
//     //     })
//     // });
//     group.bench_function("slices", |b| {
//         b.iter(|| {
//             let word = data.next().unwrap();
//             canonicalize_slices(word)
//         })
//     });
//     group.finish();
// }

// pub fn bench_solution(c: &mut Criterion) {
//     let words = words();
//     let mut data: Vec<&str> = words.iter().cloned().collect();
//     let mut group = c.benchmark_group("solution");
//     group.warm_up_time(Duration::new(10, 0));
//     group.sample_size(30);
//     group.measurement_time(Duration::new(60 * 8, 0));
//     // group.bench_function("binary", |b| {
//     //     b.iter(|| {
//     //         find_the_four_binary(&data);
//     //     })
//     // });
//     group.bench_function("fast", |b| {
//         b.iter(|| {
//             find_the_four_counters(&data);
//         })
//     });
//     group.finish();
// }

// pub fn bench_full(mut c: &mut Criterion) {
//     bench_is_necklace(&mut c);
//     // bench_canonicalize(&mut c);
//     // bench_solution(&mut c);
// }

// pub fn primitive(c: &mut Criterion) {
//     let mut group = c.benchmark_group("primitive");
//     group.warm_up_time(Duration::new(1, 0));
//     group.sample_size(1_000);
//     group.measurement_time(Duration::new(2, 0));

//     let mut rng = rand::thread_rng();
//     group.bench_function("init/vector", |b| b.iter(|| Vec::<char>::with_capacity(20)));
//     group.bench_function("init/string", |b| {
//         b.iter(|| String::from("12345678901234567890"))
//     });
//     group.bench_function("init/vector-deque", |b| {
//         b.iter(|| VecDeque::<char>::with_capacity(20))
//     });
//     let mut d: VecDeque<char> = "12345678901234567890".chars().collect();
//     group.bench_function("rotate/vector-deque", |b| b.iter(|| d.rotate_left(1)));

//     let mut hasher = DefaultHasher::new();
//     group.bench_function("hasher/string", |b| {
//         b.iter(|| {
//             "12345678901234567890".hash(&mut hasher);
//             hasher.finish()
//         })
//     });

//     group.finish();
// }

pub fn bench_necklaces(c: &mut Criterion) {
    let mut group = c.benchmark_group("necklaces");
    group
        .warm_up_time(Duration::new(1, 0))
        .sample_size(15)
        .measurement_time(Duration::new(10, 0));

    group.bench_function("sieve_erato-100", |b| b.iter(|| Primes::sieve_erato(100)));

    group.bench_function("sieve_erato-1000", |b| b.iter(|| Primes::sieve_erato(1000)));

    let primes = Primes::sieve_erato(1500);
    let mut rng = rand::thread_rng();
    let mut n: Vec<usize> = (1..100_000).map(|x| x % 99 + 1).collect();
    n.shuffle(&mut rng);
    let mut n = n.into_iter().cycle();

    group.bench_function("relative", |b| {
        b.iter(|| primes.relative(n.next().unwrap()).sum::<usize>())
    });

    group.bench_function("phi", |b| b.iter(|| primes.phi(n.next().unwrap())));

    group.bench_function("necklaces-n-100", |b| {
        b.iter(|| primes.necklaces(1, n.next().unwrap()))
    });

    group.bench_function("necklaces-n-1000", |b| {
        b.iter(|| primes.necklaces(1, n.next().unwrap() * 10))
    });

    let mut k: Vec<usize> = (1..100_000).map(|x| x % 99 + 1).collect();
    k.shuffle(&mut rng);
    let mut k = k.into_iter().cycle();

    group.bench_function("necklaces-k-100", |b| {
        b.iter(|| primes.necklaces(k.next().unwrap(), 1))
    });

    group.bench_function("necklaces-k-1000", |b| {
        b.iter(|| primes.necklaces(k.next().unwrap() * 10, 1))
    });

    group.bench_function("necklaces-k-n", |b| {
        b.iter(|| primes.necklaces(k.next().unwrap(), n.next().unwrap()))
    });

    group.bench_function("necklaces-big-k-n", |b| {
        b.iter(|| primes.necklaces_big(k.next().unwrap(), n.next().unwrap()))
    });

    group.bench_function("necklaces-big-3-90", |b| {
        b.iter(|| primes.necklaces_big(3, 90))
    });

    group.bench_function("necklaces-123-18", |b| b.iter(|| primes.necklaces(123, 18)));

    group
        .warm_up_time(Duration::new(10, 0))
        .sample_size(15)
        .measurement_time(Duration::new(100, 0));

    group.bench_function("necklaces-big-1024-512", |b| {
        b.iter(|| primes.necklaces_big(1024, 512))
    });
}

criterion_group!(benches, bench_necklaces);
criterion_main!(benches);
