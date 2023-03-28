use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l5::*;

fn bench_shrtst_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("Кратчайший путь");

    let graphs = [
        (
            "Л5 пример 1",
            Graph::new(
                6,
                vec![
                    GraphEdge::new(0, 1, 7),
                    GraphEdge::new(0, 3, 2),
                    GraphEdge::new(0, 5, 4),
                    GraphEdge::new(1, 2, 1),
                    GraphEdge::new(1, 3, 2),
                    GraphEdge::new(5, 3, 1),
                    GraphEdge::new(5, 4, 8),
                    GraphEdge::new(2, 3, 2),
                    GraphEdge::new(2, 4, 3),
                    GraphEdge::new(3, 4, 6),
                ],
            ),
        ),
        (
            "Л5 пример 2",
            Graph::new(
                5,
                vec![
                    GraphEdge::new(0, 1, 1),
                    GraphEdge::new(0, 4, 10),
                    GraphEdge::new(0, 2, 2),
                    GraphEdge::new(1, 3, 3),
                    GraphEdge::new(1, 4, 6),
                    GraphEdge::new(2, 4, 7),
                    GraphEdge::new(2, 3, 4),
                    GraphEdge::new(3, 4, 11),
                ],
            ),
        ),
        (
            "Л5 вариант 9(16)",
            Graph::new(
                8,
                vec![
                    GraphEdge::new(0, 4, 9),
                    GraphEdge::new(0, 5, 10),
                    GraphEdge::new(0, 1, 4),
                    GraphEdge::new(1, 4, 3),
                    GraphEdge::new(1, 6, 2),
                    GraphEdge::new(1, 2, 1),
                    GraphEdge::new(2, 5, 1),
                    GraphEdge::new(2, 7, 7),
                    GraphEdge::new(2, 3, 1),
                    GraphEdge::new(3, 6, 8),
                    GraphEdge::new(3, 7, 6),
                    GraphEdge::new(7, 6, 5),
                    GraphEdge::new(6, 5, 1),
                    GraphEdge::new(5, 4, 4),
                ],
            ),
        ),
        (
            "Л6 пример 2",
            Graph::new(
                9,
                vec![
                    GraphEdge::new(0, 1, 7),
                    GraphEdge::new(0, 3, 10),
                    GraphEdge::new(1, 4, 9),
                    GraphEdge::new(1, 2, 27),
                    GraphEdge::new(2, 8, 15),
                    GraphEdge::new(3, 4, 8),
                    GraphEdge::new(3, 6, 31),
                    GraphEdge::new(4, 5, 11),
                    GraphEdge::new(5, 7, 17),
                    GraphEdge::new(5, 8, 15),
                    GraphEdge::new(6, 7, 32),
                    GraphEdge::new(7, 8, 21),
                ],
            ),
        ),
    ];

    let mut rng = thread_rng();
    for (name, graph) in graphs {
        group.bench_function(BenchmarkId::new("Флойда", name), |b| {
            b.iter(|| {
                floyd_shrtst_path(
                    &graph,
                    rng.gen_range(0..graph.num_verts()),
                    rng.gen_range(0..graph.num_verts()),
                )
            })
        });
        let mat = graph.to_adj_matrix();
        group.bench_function(BenchmarkId::new("Дейкстры", name), |b| {
            b.iter(|| {
                dijkstra_shrtst_path(
                    &mat,
                    rng.gen_range(0..graph.num_verts()),
                    rng.gen_range(0..graph.num_verts()),
                )
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_shrtst_paths);
criterion_main!(benches);
