use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l6::*;

fn bench_maze_gens(c: &mut Criterion) {
    let mut group = c.benchmark_group("Построение лабиринта");
    let mut rng = thread_rng();

    for i in [5, 10, 15] {
        group.bench_function(
            BenchmarkId::new("Итеративный возврат", i),
            |b| {
                b.iter(|| {
                    iter_backtr_maze_gen(
                        i,
                        rng.gen_range(0..i),
                        rng.gen_range(0..i),
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("Олдоса-Бродера", i),
            |b| {
                b.iter(|| {
                    aldous_broder_maze_gen(
                        i,
                        rng.gen_range(0..i),
                        rng.gen_range(0..i),
                    )
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_maze_gens);
criterion_main!(benches);
