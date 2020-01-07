use criterion::*;
use parallel_rust_cpp::*;
use rand::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("step");
    for size in 1..101 {
        let n: usize = size * 10;
        group.throughput(Throughput::Bytes(n as u64));

        let d: Vec<f32> = (0..1 * n * n).map(|_| rand::random()).collect();
        let mut r: Vec<f32> = vec![0.0; n * n];
        group.bench_function(format!("step input {}", size), |b| {
            b.iter(|| step(&mut r, d.as_slice(), n))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
