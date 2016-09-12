use geojson;
use geojson::{Feature, Value, GeoJson, Geometry, FeatureCollection};

use rustc_serialize::json::Json;

use graph::{Node, Graph, AdjArrayGraph};
use wgs84::{WGS84, haversine};
use dijkstra::{WeightedData};

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeData {
    forward: bool,
    backward: bool,
    weight: u32
}

impl WeightedData<u32> for EdgeData {
    fn weight(&self) -> u32 {
        self.weight
    }
}

fn is_road(feature: &Feature) -> bool {
    match feature.properties {
        None => false,
        Some(ref map) => match map.get("highway") {
            Some(&Json::String(ref value)) => match value.as_ref() {
                "motorway" => true,
                "motorway_link" => true,
                "trunk" => true,
                "trunk_link" => true,
                "primary" => true,
                "primary_link" => true,
                "secondary" => true,
                "secondary_link" => true,
                "tertiary" => true,
                "tertiary_link" => true,
                "unclassified" => true,
                "residential" => true,
                "living_street" => true,
                "service" => true,
                "ferry" => true,
                "movable" => true,
                "shuttle_train" => true,
                _ => false
            },
            _ => false
        }
    }
}

fn compute_highway_accessibility(feature: &Feature) -> (bool, bool) {
    match feature.properties {
        None => (true, true),
        Some(ref map) => match map.get("oneway") {
            Some(&Json::String(ref value)) => match value.as_ref() {
                "1" | "yes" => (true, false),
                "-1" => (false, true),
                "no" => (true, true),
                _ => (true, true)
            },
            _ => (true, true)
        }
    }
}

fn compute_highway_speed(feature: &Feature) -> f64 {
    // speed in km/h
    match feature.properties {
        None => 5.0,
        Some(ref map) => match map.get("highway") {
            Some(&Json::String(ref value)) => match value.as_ref() {
                "motorway" => 90.0,
                "motorway_link" => 45.0,
                "trunk" => 85.0,
                "trunk_link" => 40.0,
                "primary" => 65.0,
                "primary_link" => 30.0,
                "secondary" => 55.0,
                "secondary_link" => 25.0,
                "tertiary" => 40.0,
                "tertiary_link" => 20.0,
                "unclassified" => 25.0,
                "residential" => 25.0,
                "living_street" => 10.0,
                "service" => 15.0,
                "ferry" => 5.0,
                "movable" => 5.0,
                "shuttle_train" => 10.0,
                _ => 5.0
            },
            _ => 5.0
        }
    }
}

type OSMEdge = (i64, i64, EdgeData);
fn roads_to_edges(features: Vec<Feature>) -> Vec<OSMEdge> {
    let mut edges = Vec::new();

    for feature in features {
        let speed = compute_highway_speed(&feature);
        let accessibility = compute_highway_accessibility(&feature);

        if accessibility == (false, false) {
            continue;
        }

        if feature.geometry.is_none() {
            continue;
        }

        let ref line_string = match feature.geometry {
            None => continue,
            Some(ref geometry) => match geometry.value {
                Value::LineString(ref line_value) => line_value,
                _ => continue
            }
        };

        if line_string.len() < 2 {
            continue;
        }

        let ref nodes = match feature.properties {
            None => continue,
            Some(ref map) => match map.get("@nodes") {
                Some(&Json::Array(ref array_value)) => array_value,
                _ => continue,
            }
        };

        assert!(nodes.len() == line_string.len());

        for index in 0..nodes.len()-1 {
            let ref prev_node = nodes[index];
            let ref prev_coordinate = line_string[index];
            let ref current_node = nodes[index+1];
            let ref current_coordinate = line_string[index+1];
            let distance = haversine(&WGS84 {lon: prev_coordinate[0], lat: prev_coordinate[1]}, &WGS84 {lon: current_coordinate[0], lat: current_coordinate[1]});
            let duration = (distance / speed * 10.0).round() as u32;
            edges.push((prev_node.as_i64().unwrap(), current_node.as_i64().unwrap(), EdgeData {forward: accessibility.0, backward: accessibility.1, weight: duration}));
        }
    }

    edges
}

pub type IdMap = BTreeMap<i64, Node>;
type InputEdge = (Node, Node, EdgeData);
// TODO this could be a BFS ordering
fn renumber_edges(mut osm_edges : Vec<OSMEdge>) -> (Vec<InputEdge>, IdMap) {
    let mut id_map = IdMap::new();
    for ref e in &osm_edges {
        let first_id = id_map.len() as Node;
        id_map.entry(e.0).or_insert(first_id);
        let second_id = id_map.len() as Node;
        id_map.entry(e.1).or_insert(second_id);
    }

    let mut input_edges = Vec::new();
    for e in osm_edges.drain(0..) {
        input_edges.push((*id_map.get(&e.0).unwrap(), *id_map.get(&e.1).unwrap(), e.2));
    }

    (input_edges, id_map)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    GeoJson(geojson::Error),
    NoFeature
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::Io(other)
    }
}

impl From<geojson::Error> for Error {
    fn from(other: geojson::Error) -> Error {
        Error::GeoJson(other)
    }
}

pub fn from_geojson(path: &String) -> Result<(AdjArrayGraph<EdgeData>, IdMap), Error> {
    let mut features : Vec<Feature> = Vec::new();
    let mut reader = BufReader::new(try!(File::open(path)));

    let mut data = String::new();
    while try!(reader.read_line(&mut data)) > 0 {
        let feature = {
            let geojson = try!(data.parse::<GeoJson>());
            match geojson {
                GeoJson::Feature(f) => f,
                // everything other then a feature collection is malformed
                _ => return Err(Error::NoFeature),
            }
        };
        if is_road(&feature) {
            features.push(feature);
        }
        data.clear();
    }

    let (edges, id_map) = renumber_edges(roads_to_edges(features));

    Ok((AdjArrayGraph::new(edges), id_map))
}


#[cfg(test)]
mod tests {
    use super::*;
    use graph::Graph;

    #[test]
    fn load_sample() {
        let (g, _) = from_geojson(&String::from("data/sample.geojson")).unwrap();
        assert_eq!(g.num_nodes(), 9);
        assert_eq!(g.num_edges(), 8);
    }
}
