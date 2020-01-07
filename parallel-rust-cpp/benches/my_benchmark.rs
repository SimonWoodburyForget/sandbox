use criterion::*;
use parallel_rust_cpp::*;
use rand::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("step");
    group.sample_size(30);
    for size in 8..14 {
        let n: usize = (size + 10) * 4;
        let d: Vec<f32> = (0..).map(|_| rand::random()).take(n * n).collect();
        group.throughput(Throughput::Bytes(d.len() as u64));

        group.bench_with_input(BenchmarkId::new("v0a", d.len()), &d, |b, d| {
            let mut r: Vec<f32> = vec![0.0; n * n];
            b.iter(|| step_v0a(&mut r, d, n))
        });
        group.bench_with_input(BenchmarkId::new("v0b", d.len()), &d, |b, d| {
            let mut r: Vec<f32> = vec![0.0; n * n];
            b.iter(|| step_v0b(&mut r, d, n))
        });
        group.bench_with_input(BenchmarkId::new("v0c", d.len()), &d, |b, d| {
            let mut r: Vec<f32> = vec![0.0; n * n];
            b.iter(|| step_v0c(&mut r, d, n))
        });

        // group.bench_function(format!("step input size {}", d.len()), |b| {

        //     b.iter(|| step(&mut r, &d, n))
        // });
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
