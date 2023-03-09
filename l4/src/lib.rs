#[inline]
pub fn kruskal_spanning_tree(mut graph: Graph) -> Graph {
    let mut out = Graph::new(graph.num_verts, Default::default());
    let mut vert_sets: Vec<Vec<usize>> =
        (0..graph.num_verts).map(|i| vec![i]).collect();

    graph.edges.sort_by(|a, b| a.weight.cmp(&b.weight));
    for edge in graph.edges {
        let first = find_set_index(&vert_sets, edge.from);
        let second = find_set_index(&vert_sets, edge.to);

        if first != second {
            out.edges.push(edge);
            merge_sets(&mut vert_sets, first, second);
        }

        if vert_sets.len() == 1 {
            break;
        }
    }

    out
}

#[inline]
fn find_set_index(sets: &[Vec<usize>], vert_id: usize) -> usize {
    for (ind, set) in sets.iter().enumerate() {
        if set.iter().any(|id| *id == vert_id) {
            return ind;
        }
    }
    unreachable!()
}

#[inline]
fn merge_sets(sets: &mut Vec<Vec<usize>>, dest: usize, src: usize) {
    let src_vec = &mut sets[src] as *mut Vec<usize>;
    unsafe {
        sets[dest].append(src_vec.as_mut().unwrap());
    }
    sets.swap_remove(src);
}

#[inline]
pub fn prim_spanning_tree(graph_adj_mat: &[Vec<usize>], first: usize) -> Graph {
    let num_verts = graph_adj_mat.len();
    let mut out = Graph::new(num_verts, Default::default());

    let mut selected = vec![false; num_verts];
    selected[first] = true;

    for _ in 0..num_verts - 1 {
        let mut min_weight = usize::MAX;
        let mut min_row = 0;
        let mut min_col = 0;
        for i in 0..num_verts {
            if selected[i] {
                for j in 0..num_verts {
                    let weight = graph_adj_mat[i][j];
                    if !selected[j] && weight > 0 && min_weight > weight {
                        min_weight = weight;
                        min_row = i;
                        min_col = j;
                    }
                }
            }
        }

        out.edges.push(GraphEdge::new(
            min_row,
            min_col,
            graph_adj_mat[min_row][min_col],
        ));
        selected[min_col] = true;
    }

    out
}

#[derive(Debug, PartialEq, Clone)]
pub struct GraphEdge {
    from: usize,
    to: usize,
    weight: usize,
}

impl GraphEdge {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Graph {
    num_verts: usize,
    edges: Vec<GraphEdge>,
}

impl Graph {
    pub fn new(num_verts: usize, edges: Vec<GraphEdge>) -> Self {
        Self { num_verts, edges }
    }

    pub fn num_verts(&self) -> usize {
        self.num_verts
    }

    pub fn to_adj_matrix(&self) -> Vec<Vec<usize>> {
        let mut out = vec![vec![0; self.num_verts]; self.num_verts];

        for edge in &self.edges {
            out[edge.from][edge.to] = edge.weight;
            out[edge.to][edge.from] = edge.weight;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kruskal_spanning_tree_works() {
        let graph = Graph::new(
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
        );

        assert_eq!(
            kruskal_spanning_tree(graph),
            Graph::new(
                6,
                vec![
                    GraphEdge::new(1, 2, 1),
                    GraphEdge::new(5, 3, 1),
                    GraphEdge::new(0, 3, 2),
                    GraphEdge::new(1, 3, 2),
                    GraphEdge::new(2, 4, 3),
                ]
            )
        );
    }

    #[test]
    fn prim_spanning_tree_works() {
        let graph = Graph::new(
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
        );

        assert_eq!(
            prim_spanning_tree(&graph.to_adj_matrix(), 0),
            Graph::new(
                6,
                vec![
                    GraphEdge::new(0, 3, 2),
                    GraphEdge::new(3, 5, 1),
                    GraphEdge::new(3, 1, 2),
                    GraphEdge::new(1, 2, 1),
                    GraphEdge::new(2, 4, 3)
                ]
            )
        );
    }
}
