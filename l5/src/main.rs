use l5::*;

fn main() {
    let g = Graph::new(
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
    );

    let path = floyd_shrtst_path(&g, 0, 8);
    println!("{path:?}");
}
