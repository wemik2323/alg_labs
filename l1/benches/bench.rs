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
    let mut group = c.benchmark_group("Сортировка");

    for i in [300, 600, 900] {
        // Т.к функция сортировки запускается много раз с одними и теми же
        // агрументами, мы вынуждены постоянно генерировать новый массив.
        // Время, занимаемое созданием массива, составляет тысячные
        // доли процентов от времени сортировки, следовательно им можно
        // пренебречь
        group.bench_function(BenchmarkId::new("Выбором", i), |b| {
            b.iter(|| selection_sort(&mut gen_rand_vec(i)))
        });
        group.bench_function(BenchmarkId::new("Пузырьком", i), |b| {
            b.iter(|| bubble_sort(&mut gen_rand_vec(i)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
