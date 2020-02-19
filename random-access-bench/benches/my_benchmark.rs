use criterion::*;
use rand::seq::SliceRandom;
use std::collections::*;
use std::hint::unreachable_unchecked;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("random-access");
    group.sample_size(10_000);
    group.measurement_time(Duration::new(30, 0));

    let mut rng = rand::thread_rng();

    let mut indices = (0..100_000).collect::<Vec<usize>>();
    indices.shuffle(&mut rng);

    let mut data = (0..100_000).collect::<Vec<u32>>();
    data.shuffle(&mut rng);

    {
        group.bench_function("vec reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| data[*it.next().unwrap()])
        });

        group.bench_function("vec writes", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| {
                data[*it.next().unwrap()] = 2;
            });
        });
    }

    {
        let mut map_data = BTreeMap::new();
        for (e, datum) in data.iter().enumerate() {
            map_data.insert(e, *datum);
        }

        group.bench_function("btree reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| map_data[it.next().unwrap()])
        });

        group.bench_function("btree writes", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| {
                map_data.insert(*it.next().unwrap(), 2);
            })
        });
    }

    {
        let mut map_data = HashMap::new();
        for (e, datum) in data.iter().enumerate() {
            map_data.insert(e, *datum);
        }

        group.bench_function("hashmap reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| map_data[it.next().unwrap()])
        });

        group.bench_function("hashmap writes", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| {
                map_data.insert(*it.next().unwrap(), 2);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
