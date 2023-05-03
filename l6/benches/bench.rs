use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l6::*;

fn bench_maze_gens(c: &mut Criterion) {
    let mut group = c.benchmark_group("Построение лабиринта");
    let mut rng = thread_rng();

    let mut mat_len = 10;
    while mat_len <= 30 {
        group.bench_function(
            BenchmarkId::new("Итеративный возврат", mat_len),
            |b| {
                b.iter(|| {
                    iter_backtr_maze_gen(
                        mat_len,
                        rng.gen_range(0..mat_len),
                        rng.gen_range(0..mat_len),
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("Олдоса-Бродера", mat_len),
            |b| {
                b.iter(|| {
                    aldous_broder_maze_gen(
                        mat_len,
                        rng.gen_range(0..mat_len),
                        rng.gen_range(0..mat_len),
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("Рекурсивный возврат", mat_len),
            |b| {
                b.iter(|| {
                    aldous_broder_maze_gen(
                        mat_len,
                        rng.gen_range(0..mat_len),
                        rng.gen_range(0..mat_len),
                    )
                })
            },
        );
        mat_len += 1;
    }

    group.finish();
}

criterion_group!(benches, bench_maze_gens);
criterion_main!(benches);
