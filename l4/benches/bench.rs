use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l4::*;

fn bench_spanning_trees(c: &mut Criterion) {
    let mut group = c.benchmark_group("Остовное древо");

    let graphs = vec![
        (
            "Пример 1",
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
            "Пример 2",
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
            "Вариант 9(16)",
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
    ];

    let mut rng = thread_rng();
    for (name, graph) in graphs {
        group.bench_function(BenchmarkId::new("Крускала", name), |b| {
            b.iter(|| kruskal_spanning_tree(graph.clone()))
        });
        let mat = graph.to_adj_matrix();
        group.bench_function(BenchmarkId::new("Прима", name), |b| {
            b.iter(|| {
                prim_spanning_tree(&mat, rng.gen_range(0..graph.num_verts()))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_spanning_trees);
criterion_main!(benches);
