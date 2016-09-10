use std::ops::{Range};
use std::cmp;

pub type Node = u32;
pub type Edge = u32;

//use std::ops::{Add};
//use std::iter::{Step};
//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
//pub struct Node(pub u32);
//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
//pub struct Edge(pub u32);

// Ranges for custom types don't work with the stable rust
// since they need #![feature(step_trait)]
//
//impl<'a> Add for &'a Node {
//    type Output = Node;
//    fn add(self, rhs: Self) -> Self::Output {
//        Node(self.0 + rhs.0)
//    }
//}
//
//impl Step for Node {
//    fn step(&self, by: &Self) -> Option<Self> {
//        Some(Node(self.0 + by.0))
//    }
//
//    fn steps_between(start: &Self, end: &Self, by: &Self) -> Option<usize> {
//        let steps_by_one = end.0 - start.0;
//        if steps_by_one % by.0 == 0 {
//            Some((steps_by_one / by.0) as usize)
//        } else {
//            None
//        }
//    }
//
//    fn steps_between_by_one(start: &Self, end: &Self) -> Option<usize> {
//        Some((end.0 - start.0) as usize)
//    }
//
//    fn is_negative(&self) -> bool {
//        false
//    }
//
//    fn replace_one(&mut self) -> Self {
//        Node(1)
//    }
//
//    fn replace_zero(&mut self) -> Self {
//        Node(0)
//    }
//
//    fn add_one(&self) -> Self { Node(self.0 + 1)}
//    fn sub_one(&self) -> Self { Node(self.0 - 1)}
//}


pub trait Graph<T> {
    type N;
    type E;

    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn nodes(&self) -> Range<Self::N>;
    fn edges(&self, id: Self::N) -> Range<Self::E>;
    fn target(&self, id: Self::E) -> Self::N;
    fn data(&self, id: Self::E) -> &T;
}

pub struct AdjArrayGraph<T> {
    offsets: Vec<u32>,
    targets: Vec<Node>,
    data: Vec<T>
}

impl<T: Ord> AdjArrayGraph<T> {
    pub fn new(mut input_edges: Vec<(Node, Node, T)>) -> AdjArrayGraph<T> {
        let mut offsets : Vec<u32> = vec![0];
        let mut targets : Vec<Node> = Vec::new();
        let mut data : Vec<T> = Vec::new();

        // firts sort by start and then target node
        input_edges.sort();

        let first_start = match input_edges.first() {
            Some(&(start, _, _)) => Some(start),
            None => None
        };

        if let Some(first_start_node) = first_start {
            // now construct the prefix array
            let mut offset : usize = 0;
            let mut last_start : Node = first_start_node;
            let mut max_node_id : Node = 0;

            for (start, target, d) in input_edges.drain(0..) {
                if start != last_start {
                    offsets.push(offset as u32);
                    last_start = start;
                }
                targets.push(target);
                data.push(d);
                offset += 1;
                max_node_id = cmp::max(max_node_id, start);
                max_node_id = cmp::max(max_node_id, target);
            }

            // make sure we have an entry for each referenced node id
            // plus the sentinel
            while offsets.len() <= max_node_id as usize {
                offsets.push(offset as u32);
            }
        }

        AdjArrayGraph {offsets: offsets, targets: targets, data: data}
    }
}

impl<T> Graph<T> for AdjArrayGraph<T> {
    type N = Node;
    type E = Edge;

    fn num_nodes(&self) -> usize {
        self.offsets.len() - 1
    }

    fn num_edges(&self) -> usize {
        self.data.len()
    }

    fn nodes(&self) -> Range<Node> {
        let start_node : Node = 0;
        let end_node : Node = (self.offsets.len() - 1) as u32;
        start_node..end_node
    }

    fn edges(&self, id: Node) -> Range<Edge> {
        let idx = id as usize;
        let start_edge : Edge = self.offsets[idx];
        let end_edge : Edge = self.offsets[idx + 1];
        start_edge..end_edge
    }

    fn target(&self, id: Edge) -> Node {
        let idx = id as usize;
        self.targets[idx]
    }

    fn data(&self, id: Edge) -> &T {
        let idx = id as usize;
        &self.data[idx]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let g : AdjArrayGraph<()> = AdjArrayGraph::new(vec![]);
        assert_eq!(g.num_nodes(), 0);
        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.nodes().len(), 0);
    }

    #[test]
    fn scan_graph() {
        let g: AdjArrayGraph<()> = AdjArrayGraph::new(vec![
        (0, 1, ()), (0, 2, ()), (1, 2, ())
        ]);

        let test_sources = vec![0, 1, 2];
        let test_targets = vec![vec![1, 2], vec![2], vec![]];

        for (source_idx, source) in g.nodes().enumerate() {
            assert_eq!(source, test_sources[source_idx]);
            for (edge_idx, e) in g.edges(source).enumerate() {
                let target = g.target(e);
                assert_eq!(target, test_targets[source as usize][edge_idx]);
            }
        }
    }
}
