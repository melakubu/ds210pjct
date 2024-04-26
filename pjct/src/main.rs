// Imports
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Reading the txt file
    let file = File::open("facebook_combined.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut graph = UnGraph::<(), u32>::new_undirected(); // Creating an undirected graph
    let mut node_indices = HashMap::new(); // Storing the NodeIndex for each node ID

    // Building the graph
    for line in reader.lines() { // As we iterate through each line in the file...
        let line = line?;
        // splitting the line string then putting the split substrings in the vector 'nodes'
        let nodes: Vec<&str> = line.split_whitespace().collect(); 
        // Parsing each string into usize
        let start = nodes[0].parse::<usize>().unwrap();
        let end = nodes[1].parse::<usize>().unwrap();
        // Checking if a node exists in our graph via the hashmap
        // If not, we add the node to the graph   
        let start_index = *node_indices.entry(start).or_insert_with(|| graph.add_node(()));
        let end_index = *node_indices.entry(end).or_insert_with(|| graph.add_node(()));
        graph.add_edge(start_index, end_index, 1); // Adding edges between nodes
    }

    let mut rng = thread_rng(); // Random num generator
    let nodes: Vec<NodeIndex> = node_indices.values().cloned().collect(); // list of all node indices

    let mut path_lengths = vec![]; 
    // Randomly selecting 1000 pairs of nodes
    for _ in 0..1000 {  
        let &start = nodes.choose(&mut rng).unwrap();
        let &end = nodes.choose(&mut rng).unwrap(); 
        // Using bfs func to find the shortest path length from the selected nodes
        let length = bfs_shortest_path(&graph, start, end);
        if let Some(l) = length {
            path_lengths.push(l); // Checking if the len is valid
        }
    }
    // Calculating and reporting the avg path len
    let average_path_length: f64 = path_lengths.iter().sum::<u32>() as f64 / path_lengths.len() as f64;
    println!("Average shortest path length: {}", average_path_length);

    if average_path_length <= 6.0 {
        println!("The average degree of separation supports the 'small world hypothesis'.");
    } else {
        println!("The average degree of separation does not support the 'small world hypothesis'.");
    }

    // Checkin reachability from 1000 randomly selected nodes
    let mut reach_counts = vec![]; // empty vector to storre reachability counts
    let total_nodes = graph.node_count() as f64; // total number of nodes in the graph
    for _ in 0..1000 {
        let &start = nodes.choose(&mut rng).unwrap();
        let reach_count = bfs_reachability(&graph, start, 6); // Calc how many nodes are reachable within 6
        reach_counts.push(reach_count); // Storing result in our vector
    }
    // Calc the avg num of reachable nodes then into a percentage of the network
    let average_reach: f64 = reach_counts.iter().sum::<u32>() as f64 / reach_counts.len() as f64; 
    let percentage_reachable = (average_reach / total_nodes) * 100.0;

    println!("Average percentage of network reachable within 6 steps: {:.2}%", percentage_reachable);

    Ok(())
}

// Defining the BFS algorithm 
fn bfs_shortest_path(graph: &UnGraph<(), u32>, start: NodeIndex, end: NodeIndex) -> Option<u32> {
    let mut visited = vec![false; graph.node_count()]; // Checking if each node has been visited
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new(); // Storing the shortest dist
    // Setting up for the Start node
    visited[start.index()] = true;
    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() { // While loop to run until there are no nodes left
        let distance = *distances.get(&node).unwrap(); // Takes the current dist
        // If the current node is the destination node, return the dist
        if node == end { 
            return Some(distance);
        }

        for edge in graph.edges(node) { // As we iterate through all edges connected to the current node...
            let next = edge.target(); // Let's get the node at the other end of the edge
            if !visited[next.index()] { // If this node hasn't been visited, we mark it
                visited[next.index()] = true;
                distances.insert(next, distance + 1); // setting the dist to this node as no more than the dist to the current
                queue.push_back(next); // Adding this node to the end of the queue
            }
        }
    }
    None
}

// Function to calculate reachability within 6 steps
fn bfs_reachability(graph: &UnGraph<(), u32>, start: NodeIndex, max_depth: u32) -> u32 {
    let mut visited = vec![false; graph.node_count()];
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    visited[start.index()] = true; // Marking node as visited 
    distances.insert(start, 0); // Initializing the Distance of the Start Node
    queue.push_back((start, 0)); // Adding the Start Node to the Queue
    let mut count = 0; 
    // BFS Loop
    while let Some((node, depth)) = queue.pop_front() {
        if depth <= max_depth { // Checking if the current node's depth is within range
            count += 1; // Counting the nodes processed within the depth limit
            for edge in graph.edges(node) { // As we iterate through all edges connected to this node...
                let next = edge.target(); // Let's retrieve the node at the other end of each edge
                if !visited[next.index()] { // If it's not visited, we
                    visited[next.index()] = true; // Mark it as visited, record the depth and add it to the queue
                    distances.insert(next, depth + 1); 
                    queue.push_back((next, depth + 1));
                }
            }
        }
    }
    count
}
