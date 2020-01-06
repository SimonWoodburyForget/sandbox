use criterion::*;
use parallel_rust_cpp::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("fib 25", |b| b.iter(|| fibonacci(black_box(25))));

    c.bench_with_input(BenchmarkId::new("fib input", 10), &10, |b, &s| {
        b.iter(|| fibonacci(s))
    });

    let mut group = c.benchmark_group("fib inputs");
    for size in 2..32 {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("fib input", size), &size, |b, &s| {
            b.iter(|| fibonacci(s))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
