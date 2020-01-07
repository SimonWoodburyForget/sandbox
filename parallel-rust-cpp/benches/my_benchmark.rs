use criterion::*;
use parallel_rust_cpp::*;
use rand::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("step");
    group.sample_size(30);
    for size in 1..14 {
        let n: usize = (size + 10) * 4;
        let d: Vec<f32> = (0..).map(|_| rand::random()).take(n * n).collect();
        group.throughput(Throughput::Bytes(d.len() as u64));
        group.bench_function(format!("step input size {}", d.len()), |b| {
            let mut r: Vec<f32> = vec![0.0; n * n];
            b.iter(|| step(&mut r, &d, n))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
