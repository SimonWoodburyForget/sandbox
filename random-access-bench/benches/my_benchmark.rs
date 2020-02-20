use criterion::*;
use rand::seq::SliceRandom;
use std::hint::unreachable_unchecked;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("random-access");
    group.warm_up_time(Duration::new(1, 0));
    group.sample_size(1_000);
    group.measurement_time(Duration::new(5, 0));

    const SIZE: usize = 512 * 512;

    let (data, indices) = {
        let mut rng = rand::thread_rng();

        let mut indices = (0..SIZE).collect::<Vec<usize>>();
        indices.shuffle(&mut rng);

        let mut data = (0..SIZE as u32).collect::<Vec<u32>>();
        data.shuffle(&mut rng);

        (data, indices)
    };

    {
        let mut vector = vec![0; SIZE];
        for (e, datum) in data.iter().enumerate() {
            vector[e] = *datum;
        }

        group.bench_function("vec reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| vector[*it.next().unwrap()])
        });

        group.bench_function("vec writes", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| {
                vector[*it.next().unwrap()] = 2;
            });
        });
    }

    {
        let mut array = [0; SIZE];
        for (e, datum) in data.iter().enumerate() {
            array[e] = *datum;
        }

        group.bench_function("array reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| array[*it.next().unwrap()])
        });

        group.bench_function("array writes", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| {
                array[*it.next().unwrap()] = 2;
            });
        });
    }

    {
        use std::collections::BTreeMap;
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
        use std::collections::HashMap;
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

    {
        use hashbrown::HashMap;
        let mut map_data = HashMap::new();
        for (e, datum) in data.iter().enumerate() {
            map_data.insert(e, *datum);
        }

        group.bench_function("hashbrown reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| map_data[it.next().unwrap()])
        });

        group.bench_function("hashbrown writes", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| {
                map_data.insert(*it.next().unwrap(), 2);
            })
        });
    }

    {
        use hashbrown::HashMap;
        let mut map_data = HashMap::new();
        for (e, datum) in data.iter().enumerate() {
            map_data.insert(e, *datum);
        }

        group.bench_function("hashbrown reads", |b| {
            let mut it = indices.iter().cycle();
            b.iter(|| map_data[it.next().unwrap()])
        });

        group.bench_function("hashbrown writes", |b| {
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
