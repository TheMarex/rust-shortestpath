extern crate shortestpath;

use shortestpath::graph_builder::from_geojson;
use shortestpath::search::dijkstra;
use shortestpath::graph::Graph;
use shortestpath::addressable_heap::AddressableBinaryHeap;

#[test]
fn load_monaco() {
    let (graph, id_map) = from_geojson(&String::from("data/monaco.geojson")).unwrap();
    let mut heap : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(graph.num_nodes());
    // should be this route http://map.project-osrm.org/?z=18&center=43.732821%2C7.421045&loc=43.737282%2C7.420101&loc=43.732224%2C7.420396&hl=en&alt=0
    let start = *id_map.get(&3883559266_i64).unwrap();
    let target = *id_map.get(&25193709_i64).unwrap();
    let weight = dijkstra(&graph, &mut heap, start, target);
    assert_eq!(weight, Some(815));
}
