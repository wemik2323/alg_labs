use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

use l5::*;

fn gen_rand_graph(num_verts: usize) -> Graph {
    let mut edges = Vec::new();
    let mut chosen = vec![false; num_verts];
    let mut rng = thread_rng();

    let num_chosen = num_verts / 10 * 2;
    for _ in 0..num_chosen {
        let mut c = rng.gen_range(0..num_verts);
        while chosen[c] {
            c = rng.gen_range(0..num_verts);
        }
        chosen[c] = true;

        for i in 0..num_verts {
            if chosen[i] {
                continue;
            }

            edges.push(GraphEdge::new(c, i, rng.gen_range(1..=50)));
        }
    }

    Graph::new(num_verts, edges)
}

fn bench_shrtst_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("Кратчайший путь");

    let mut rng = thread_rng();
    let mut num_verts = 10;
    while num_verts <= 200 {
        let graph = gen_rand_graph(num_verts);

        group.bench_function(
            BenchmarkId::new("Флойда", num_verts),
            |b| {
                b.iter(|| {
                    floyd_shrtst_path(
                        &graph,
                        rng.gen_range(0..graph.num_verts()),
                        rng.gen_range(0..graph.num_verts()),
                    )
                })
            },
        );
        let mat = graph.to_adj_matrix();
        group.bench_function(
            BenchmarkId::new("Дейкстры", num_verts),
            |b| {
                b.iter(|| {
                    dijkstra_shrtst_path(
                        &mat,
                        rng.gen_range(0..graph.num_verts()),
                        rng.gen_range(0..graph.num_verts()),
                    )
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("Беллмана-Форда", num_verts),
            |b| {
                b.iter(|| {
                    bellford_shrtst_path(
                        &graph,
                        rng.gen_range(0..graph.num_verts()),
                        rng.gen_range(0..graph.num_verts()),
                    )
                })
            },
        );

        num_verts += 10;
    }

    group.finish();
}

criterion_group!(benches, bench_shrtst_paths);
criterion_main!(benches);
