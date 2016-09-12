# rust-shortestpath

[![Build Status](https://travis-ci.org/TheMarex/rust-shortestpath.svg?branch=master)](https://travis-ci.org/TheMarex/rust-shortestpath)

This rust library aims to implement some shortest path algorithms and related data structures in a generic way. Currently only a simple Dijkstra search is implemented.

To import some real-world road networks `shorestpath::graph_builder` implements a loader for OSM data converted to geojson using [minjur](https://github.com/mapbox/minjur).
This should be replaced with loading `.osm.pbf` files directly once Rust bindings for [libosmium](https://github.com/osmcode/libosmium) exist.

## Example

```rust
extern crate shortestpath;

use shortestpath::graph_builder::from_geojson;
use shortestpath::search::dijkstra;
use shortestpath::graph::Graph;
use shortestpath::addressable_heap::AddressableBinaryHeap;

// import a graph from geojson and create graph and ID translation table
let (graph, id_map) = from_geojson(&String::from("data/monaco.geojson")).unwrap();
// lookup graph internal IDs for OSM IDs
let start = *id_map.get(&3883559266_i64).unwrap();
let target = *id_map.get(&25193709_i64).unwrap();
// create a heap for the search
let mut heap : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(graph.num_nodes());
// run simple dijkstra search on the graph
let weight = dijkstra(&graph, &mut heap, start, target).unwrap();
```
