use addressable_heap::AddressableHeap;
use graph::{Graph, Node, Edge};
use std::ops::{Add};

pub trait WeightedData<K> {
    fn weight(&self) -> K;
}

pub fn dijkstra<K: Copy + Ord + Add<Output=K> + From<u32>, D: WeightedData<K>, G: Graph<D, N=Node, E=Edge>, H: AddressableHeap<K, Handle=Node>>(graph: &G, heap: &mut H, source: Node, target: Node) -> Option<K> {
    heap.push(source, K::from(0));

    loop {
        match heap.pop() {
            None => {
                break;
            },
            Some((node, weight)) if node == target => {
                return Some(weight);
            },
            Some((node, parent_weight)) => {
                for adj_edge in graph.edges(node) {
                    let target = graph.target(adj_edge);
                    let edge_weight = graph.data(adj_edge).weight();
                    let total_weight = parent_weight + edge_weight;
                    if heap.in_heap(target) {
                        heap.decrease(target, total_weight);
                    } else {
                        heap.push(target, total_weight);
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;
    use addressable_heap::*;

    #[derive(PartialEq,Eq,PartialOrd,Ord)]
    struct TestData {
        weight: u32
    }

    impl WeightedData<u32> for TestData {
        fn weight(&self) -> u32 {
            self.weight
        }
    }

    // 0 --> 1 ---> 2
    // |------------^
    #[test]
    fn dijkstra_triangle() {
        let graph : AdjArrayGraph<TestData> = AdjArrayGraph::new(vec![(0, 1, TestData {weight: 1}), (1, 2, TestData {weight: 1}), (0, 2, TestData {weight: 3})]);
        let mut heap : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(3);
        let weight = dijkstra(&graph, &mut heap, 0, 2);
        assert_eq!(weight, Some(2));
    }
}
