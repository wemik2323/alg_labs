use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l3::*;

// 100 - 200 вершин у графов
// без изолированных вершин
// поиск в глубину / Боровки
fn bench_optimisations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Минимум");
    let mut rng = thread_rng();

    let func = |x: f64| f64::cos(2. * x - 3.) + 1.5;

    let mut precision = 1e-1;
    while precision > 1e-11 {
        group.bench_function(
            BenchmarkId::new("Итеративно", precision),
            |b| {
                b.iter(|| {
                    iter_gldn_ratio_min(
                        func,
                        rng.gen_range(-5.0..-1.),
                        rng.gen_range(1.0..5.),
                        precision,
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("Рекурсивно", precision),
            |b| {
                b.iter(|| {
                    rec_gldn_ratio_min(
                        func,
                        rng.gen_range(-5.0..-1.),
                        rng.gen_range(1.0..5.),
                        precision,
                    )
                })
            },
        );

        precision /= 10.;
    }
    group.finish();
}

criterion_group!(benches, bench_optimisations);
criterion_main!(benches);
