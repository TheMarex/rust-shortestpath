use addressable_heap::{AddressableHeap, AddressableBinaryHeap};
use graph::{Graph, Node, Edge, AdjArrayGraph};
use std::ops::{Add};

pub trait MetricData<K> {
    fn distance(&self) -> K;
}

pub fn dijkstra<K: Copy + Ord + Add<Output=K> + From<u32>, D: MetricData<K>, G: Graph<D, N=Node, E=Edge>, H: AddressableHeap<K, Handle=Node>>(graph: &G, heap: &mut H, source: Node, target: Node) -> Option<K> {
    heap.push(source, K::from(0));

    loop {
        match heap.pop() {
            None => {
                break;
            },
            Some((node, distance)) if node == target => {
                return Some(distance);
            },
            Some((node, parent_distance)) => {
                for adj_edge in graph.edges(node) {
                    let target = graph.target(adj_edge);
                    let edge_distance = graph.data(adj_edge).distance();
                    let total_distance = parent_distance + edge_distance;
                    if heap.in_heap(target) {
                        heap.decrease(target, total_distance);
                    } else {
                        heap.push(target, total_distance);
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
        distance: u32
    }

    impl MetricData<u32> for TestData {
        fn distance(&self) -> u32 {
            self.distance
        }
    }

    // 0 --> 1 ---> 2
    // |------------^
    #[test]
    fn dijkstra_triangle() {
        let graph : AdjArrayGraph<TestData> = AdjArrayGraph::new(vec![(0, 1, TestData {distance: 1}), (1, 2, TestData {distance: 1}), (0, 2, TestData {distance: 3})]);
        let mut heap : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(3);
        let distance = dijkstra(&graph, &mut heap, 0, 2);
        assert_eq!(distance, Some(2));
    }
}
