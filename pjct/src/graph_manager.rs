// Imports
use petgraph::graph::{UnGraph, NodeIndex};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct GraphManager {
    pub graph: UnGraph<(), u32>,
    node_indices: HashMap<usize, NodeIndex>,
}

impl GraphManager {
    // Constructor 
    pub fn new() -> Self {
        Self {
            graph: UnGraph::new_undirected(),
            node_indices: HashMap::new(),
        }
    }

    // Adding nodes and edges to the graph from a txt file
    pub fn build_graph(&mut self, reader: io::BufReader<File>) -> io::Result<()> {
        for line in reader.lines() { // As we iterate through each line in the txt file...
            let line = line?;
            let nodes: Vec<&str> = line.split_whitespace().collect(); // Splitting the string on whitespace
            let start = nodes[0].parse::<usize>().unwrap();
            let end = nodes[1].parse::<usize>().unwrap();
            let start_index = *self.node_indices.entry(start).or_insert_with(|| self.graph.add_node(())); // Checking if node is already in map
            let end_index = *self.node_indices.entry(end).or_insert_with(|| self.graph.add_node(()));
            self.graph.add_edge(start_index, end_index, 1);
        }
        Ok(())
    }

    // Getting all node indices
    pub fn get_node_indices(&self) -> Vec<NodeIndex> {
        self.node_indices.values().cloned().collect()
    }

    // Getting the node count
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}
