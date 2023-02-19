use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l1::*;

fn gen_rand_vec(len: usize) -> Vec<i32> {
    let mut ret = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        ret.push(rng.gen());
    }

    ret
}

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort");

    for i in [2000, 4000, 6000].iter() {
        let mut v1 = gen_rand_vec(*i);
        let mut v2 = v1.clone();

        group.bench_function(BenchmarkId::new("Selection", i), |b| {
            b.iter(|| selection_sort(&mut v1))
        });
        group.bench_function(BenchmarkId::new("Bubble", i), |b| {
            b.iter(|| bubble_sort(&mut v2))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
