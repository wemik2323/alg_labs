#[inline]
pub fn kruskal_spanning_tree(mut graph: Graph) -> Graph {
    let mut out = Graph::new(graph.node_count, Default::default());
    let mut vert_sets: Vec<Vec<usize>> =
        (0..graph.node_count).map(|i| vec![i]).collect();

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

fn find_set_index(sets: &Vec<Vec<usize>>, node_id: usize) -> usize {
    for (ind, set) in sets.iter().enumerate() {
        if let Some(_) = set.iter().find(|id| **id == node_id) {
            return ind;
        }
    }
    unreachable!()
}

fn merge_sets(sets: &mut Vec<Vec<usize>>, dest: usize, src: usize) {
    let src_vec = &mut sets[src] as *mut Vec<usize>;
    unsafe {
        sets[dest].append(src_vec.as_mut().unwrap());
    }
    sets.swap_remove(src);
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
    node_count: usize,
    edges: Vec<GraphEdge>,
}

impl Graph {
    pub fn new(node_count: usize, edges: Vec<GraphEdge>) -> Self {
        Self { node_count, edges }
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
}
