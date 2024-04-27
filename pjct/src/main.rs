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
        let nodes: Vec<&str> = line.split_whitespace().collect(); // Splitting the line string then putting the split substrings in the vector 'nodes'
        let start = nodes[0].parse::<usize>().unwrap(); // Parsing each string into usize
        let end = nodes[1].parse::<usize>().unwrap();
        let start_index = *node_indices.entry(start).or_insert_with(|| graph.add_node(())); // Checking if a node exists in our graph via the hashmap
        let end_index = *node_indices.entry(end).or_insert_with(|| graph.add_node(())); // If not, we add the node to the graph
        graph.add_edge(start_index, end_index, 1); // Adding edges between nodes
    }

    let mut rng = thread_rng(); // Random num generator
    let nodes: Vec<NodeIndex> = node_indices.values().cloned().collect(); // List of all node indices

    let mut path_lengths = vec![];
    for _ in 0..1000 { // Randomly selecting 1000 pairs of nodes
        let &start = nodes.choose(&mut rng).unwrap();
        let &end = nodes.choose(&mut rng).unwrap();
        let length = bfs_shortest_path(&graph, start, end); // Using bfs func to find the shortest path length from the selected nodes
        if let Some(l) = length {
            path_lengths.push(l); // Checking if the len is valid
        }
    }
    
    let average_path_length: f64 = path_lengths.iter().sum::<u32>() as f64 / path_lengths.len() as f64; // Calculating and reporting the avg path len
    println!("Average shortest path length: {}", average_path_length);

    // Calculate median and standard deviation
    path_lengths.sort_unstable(); // Sorting the path lengths for median calculation
    let median = if path_lengths.len() % 2 == 0 {
        let mid = path_lengths.len() / 2;
        (path_lengths[mid - 1] + path_lengths[mid]) / 2
    } else {
        path_lengths[path_lengths.len() / 2]
    };

    let mean: f64 = path_lengths.iter().sum::<u32>() as f64 / path_lengths.len() as f64;
    let variance: f64 = path_lengths.iter().map(|&x| ((x as f64 - mean).powi(2))).sum::<f64>() / path_lengths.len() as f64;
    let std_deviation = variance.sqrt();

    println!("Median of shortest path lengths: {}", median);
    println!("Standard deviation of shortest path lengths: {:.2}", std_deviation);

    if average_path_length <= 6.0 {
        println!("The average degree of separation supports the 'small world hypothesis'.");
    } else {
        println!("The average degree of separation does not support the 'small world hypothesis'.");
    }

    let mut reach_counts = vec![];
    let total_nodes = graph.node_count() as f64; // Total number of nodes in the graph
    for _ in 0..1000 { // Checking reachability from 1000 randomly selected nodes
        let &start = nodes.choose(&mut rng).unwrap();
        let reach_count = bfs_reachability(&graph, start, 6); // Calculate how many nodes are reachable within 6
        reach_counts.push(reach_count); // Storing result in our vector
    }
    
    let average_reach: f64 = reach_counts.iter().sum::<u32>() as f64 / reach_counts.len() as f64; 
    let percentage_reachable = (average_reach / total_nodes) * 100.0; // Calculate the avg num of reachable nodes then into a percentage of the network
    println!("Average percentage of network reachable within 6 steps: {:.2}%", percentage_reachable);

// Calculating the avg clustering coefficient
let mut clustering_coefficients = vec![];
for node in graph.node_indices() { // iterate through each node in the graph
    let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect(); // Collecting the neighbors of a node
    let mut edges_between_neighbors = 0; // Counting the edges between neighbors 
    // Nested loop to count the num of edges between the neighbors of the current node
    for (index, &neighbor) in neighbors.iter().enumerate() {
        for edge in graph.edges(neighbor) {
            let target = edge.target();
            // Making sure that each edge is only counted once
            if neighbors.contains(&target) && neighbors.iter().position(|&n| n == target).unwrap() > index {
                edges_between_neighbors += 1;
            }
        }
    }
    // Calc the clustering coe for the node
    let total_possible_edges = neighbors.len() * (neighbors.len() - 1) / 2; // max num of edges between all neighbors
    let clustering_coefficient = if total_possible_edges > 0 { // Ration of actual edges between neighbors and total possible edges
        edges_between_neighbors as f64 / total_possible_edges as f64
    } else { // If there are no edges, then it's 0
        0.0
    };
    // Storing the coe
    clustering_coefficients.push(clustering_coefficient);
}
// Getting the avg coe by summing them and dividing by the num of nodes
let average_clustering_coefficient: f64 = clustering_coefficients.iter().sum::<f64>() / clustering_coefficients.len() as f64;
println!("Average clustering coefficient: {:.4}", average_clustering_coefficient);

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
