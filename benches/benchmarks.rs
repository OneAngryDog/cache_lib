use criterion::{criterion_group, criterion_main, Criterion, black_box};
use cache_lib::{Cache, LRU};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TestKey {
    id: u64,
}

#[derive(Debug, PartialEq, Clone)]
struct TestValue {
    data: String,
}

fn cache_set_benchmark(c: &mut Criterion) {
    let eviction_policy = Box::new(LRU::new());
    let mut cache = Cache::new(eviction_policy, 10000);

    c.bench_function("cache_set", |b| {
        b.iter(|| {
            for i in 0..10000 {
                let key = TestKey { id: i };
                let value = TestValue { data: format!("value{}", i) };
                cache.set(key, black_box(value));
            }
        })
    });
}

fn cache_get_benchmark(c: &mut Criterion) {
    let eviction_policy = Box::new(LRU::new());
    let mut cache = Cache::new(eviction_policy, 10000);

    // Pre-fill the cache
    for i in 0..10000 {
        let key = TestKey { id: i };
        let value = TestValue { data: format!("value{}", i) };
        cache.set(key, value);
    }

    c.bench_function("cache_get", |b| {
        b.iter(|| {
            for i in 0..10000 {
                let key = TestKey { id: i };
                black_box(cache.get(&key));
            }
        })
    });
}

fn cache_remove_benchmark(c: &mut Criterion) {
    let eviction_policy = Box::new(LRU::new());
    let mut cache = Cache::new(eviction_policy, 10000);

    // Pre-fill the cache
    for i in 0..10000 {
        let key = TestKey { id: i };
        let value = TestValue { data: format!("value{}", i) };
        cache.set(key, value);
    }

    c.bench_function("cache_remove", |b| {
        b.iter(|| {
            for i in 0..10000 {
                let key = TestKey { id: i };
                black_box(cache.remove(&key));
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(10)).measurement_time(Duration::from_secs(20));
    targets = cache_set_benchmark, cache_get_benchmark, cache_remove_benchmark
}
criterion_main!(benches);
