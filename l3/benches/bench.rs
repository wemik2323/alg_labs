use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l3::*;

fn bench_optimisations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Минимум");
    let mut rng = rand::thread_rng();

    let linear = |x: f64| -0.5 * x - 3.;
    group.bench_function(
        BenchmarkId::new("Линейная", "Итеративно"),
        |b| {
            b.iter(|| {
                iter_fib_min(
                    linear,
                    rng.gen_range(-5.0..-1.),
                    rng.gen_range(1.0..5.),
                )
            })
        },
    );
    group.bench_function(
        BenchmarkId::new("Линейная", "Рекурсивно"),
        |b| {
            b.iter(|| {
                rec_fib_min(
                    linear,
                    rng.gen_range(-5.0..-1.),
                    rng.gen_range(1.0..5.),
                )
            })
        },
    );

    let quadratic = |x: f64| (2. - x).powi(2);
    group.bench_function(
        BenchmarkId::new("Квадратичная", "Итеративно"),
        |b| {
            b.iter(|| {
                iter_fib_min(
                    quadratic,
                    rng.gen_range(-2.0..1.),
                    rng.gen_range(3.0..5.),
                )
            })
        },
    );
    group.bench_function(
        BenchmarkId::new("Квадратичная", "Рекурсивно"),
        |b| {
            b.iter(|| {
                rec_fib_min(
                    quadratic,
                    rng.gen_range(-2.0..1.),
                    rng.gen_range(3.0..5.),
                )
            })
        },
    );

    let cubic = |x: f64| (x + 2.).powi(3) + 1.;
    group.bench_function(
        BenchmarkId::new("Кубическая", "Итеративно"),
        |b| {
            b.iter(|| {
                iter_fib_min(
                    cubic,
                    rng.gen_range(-4.0..-3.),
                    rng.gen_range(-2.0..0.),
                )
            })
        },
    );
    group.bench_function(
        BenchmarkId::new("Кубическая", "Рекурсивно"),
        |b| {
            b.iter(|| {
                rec_fib_min(
                    cubic,
                    rng.gen_range(-4.0..-3.),
                    rng.gen_range(-2.0..0.),
                )
            })
        },
    );

    group.finish();
}

criterion_group!(benches, bench_optimisations);
criterion_main!(benches);
