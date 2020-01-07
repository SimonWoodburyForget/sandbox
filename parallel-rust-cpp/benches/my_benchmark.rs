use criterion::*;
use parallel_rust_cpp::*;
use rand::prelude::*;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    static FUNCTIONS: [(&'static str, fn(&mut [f32], &[f32], usize)); 4] = [
        ("v0a", step_v0a as fn(&mut [f32], &[f32], usize)),
        ("v0b", step_v0b as fn(&mut [f32], &[f32], usize)),
        ("v0c", step_v0c as fn(&mut [f32], &[f32], usize)),
        ("v1", step_v1 as fn(&mut [f32], &[f32], usize)),
    ];

    let mut group = c.benchmark_group("step");
    group.sample_size(30);

    let sec = 10;
    group.measurement_time(Duration::new(sec, 0));
    let start = 60;
    let end = 63;
    println!(
        "Estimated runtime: {} minutes",
        (sec * (end - start)) as f32 / 60 as f32
    );

    for size in 60..63 {
        let n: usize = (size + 10) * 2;
        let d: Vec<f32> = (0..).map(|_| rand::random()).take(n * n).collect();

        group.throughput(Throughput::Bytes(d.len() as u64));
        for (name, f) in FUNCTIONS.iter() {
            group.bench_with_input(BenchmarkId::new(name.to_string(), d.len()), &d, |b, d| {
                let mut r: Vec<f32> = vec![0.0; n * n];

                b.iter(|| f(&mut r, d, n))
            });
        }
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);

// fn bench_fibs(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Fibonacci");
//     for i in [20u64, 21, 22, 23, 24].iter() {
//         // group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| {
//         //     b.iter(|| fibonacci_slow(*i))
//         // });
//         group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| {
//             b.iter(|| fibonacci_fast(*i))
//         });
//     }
//     group.finish();
// }

// criterion_group!(benches, bench_fibs);

criterion_main!(benches);
