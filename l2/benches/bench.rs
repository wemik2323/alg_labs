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

    let mut len = 50;
    while len <= 1000 {
        let arr = gen_rand_str(len);

        group.bench_function(
            BenchmarkId::new("Последовательный", len),
            |b| {
                b.iter(|| {
                    linear_search(arr.as_bytes(), gen_rand_str(5).as_bytes())
                })
            },
        );
        group.bench_function(BenchmarkId::new("Рабина", len), |b| {
            b.iter(|| rabin_search(arr.as_bytes(), gen_rand_str(5).as_bytes()))
        });
        group.bench_function(BenchmarkId::new("Кнута", len), |b| {
            b.iter(|| knuth_search(arr.as_bytes(), gen_rand_str(5).as_bytes()))
        });

        len += 50;
    }

    group.finish();
}

criterion_group!(benches, bench_searches);
criterion_main!(benches);
