use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l2::*;

fn gen_rand_vec(len: usize) -> Vec<i32> {
    let mut ret = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        ret.push(rng.gen());
    }

    ret
}

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Поиск");
    let mut rng = rand::thread_rng();

    for i in [2000, 4000, 6000] {
        let mut v = gen_rand_vec(i);

        group.bench_function(
            BenchmarkId::new("Последовательный", i),
            |b| b.iter(|| linear_search(&v, rng.gen())),
        );

        v.sort();
        group.bench_function(
            BenchmarkId::new("Последовательный (сорт.)", i),
            |b| b.iter(|| linear_search(&v, rng.gen())),
        );
        group.bench_function(BenchmarkId::new("Бинарный", i), |b| {
            b.iter(|| binary_search(&v, rng.gen()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
