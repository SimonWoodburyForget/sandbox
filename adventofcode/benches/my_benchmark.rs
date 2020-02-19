use criterion::*;
use parallel_rust_cpp::*;
use rand::prelude::*;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day4");
    group.sample_size(600);
    group.measurement_time(Duration::new(60, 0));
    
    // group.bench_function("from_input_unchecked", |b| b.iter(|| {
    //     unsafe { from_input_str_unchecked(black_box("123456")) }
    // }));
    
    // group.bench_function("from_input_i32", |b| b.iter(|| {
    //     from_input_i32(black_box(123456))
    // }));

    // group.bench_function("is_increase", |b| b.iter(|| {
    //     is_increase(black_box(&[1, 2, 3, 4, 5, 6]))
    // }));

    // group.bench_function("is_2_digit_same_advanced", |b| b.iter(|| {
    //     is_2_digit_same_advanced(black_box(&[1, 2, 3, 4, 5, 6]))
    // }));

    // group.bench_function("is_2_digit_same",  |b| b.iter(|| {
    //     is_2_digit_same(black_box(&[1, 2, 3, 4, 5, 6]))
    // }));

    // group.bench_function("incr", |b| b.iter(|| {
    //     incr(black_box(&mut [1, 2, 3, 4, 5, 6]))
    // }));

    // group.bench_function("incr_by_value", |b| b.iter(|| {
    //     incr_by_value(black_box([1, 2, 3, 4, 5, 6]))
    // }));

    group.bench_function("solve", |b| b.iter(|| {
        solve(black_box("000000"), black_box("999999"))
    }));

    group.bench_function("solve_par", |b| b.iter(|| {
        solve_par(black_box("000000"), black_box("999999"))
    }));

    // group.bench_function("collect_range", |b| b.iter(|| {
    //     collect_range(black_box([1, 2, 3, 4, 5, 6]), black_box([3, 5, 6, 7, 8, 9]))
    // }));
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
