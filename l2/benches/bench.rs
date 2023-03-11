use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l2::*;

fn gen_rand_str(len: usize) -> String {
    let mut ret = String::with_capacity(len);
    let mut rng = thread_rng();

    for _ in 0..len {
        ret.push(rng.gen_range('a'..'p'));
    }

    ret
}

fn bench_searches(c: &mut Criterion) {
    let mut group = c.benchmark_group("Поиск подстроки");

    for i in [3000, 6000, 9000] {
        let arr = gen_rand_str(i);

        group.bench_function(
            BenchmarkId::new("Последовательный", i),
            |b| {
                b.iter(|| {
                    linear_search(arr.as_bytes(), gen_rand_str(5).as_bytes())
                })
            },
        );
        group.bench_function(BenchmarkId::new("Рабина", i), |b| {
            b.iter(|| rabin_search(arr.as_bytes(), gen_rand_str(5).as_bytes()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_searches);
criterion_main!(benches);
