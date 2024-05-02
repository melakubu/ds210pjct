// Declaring modules
mod graph_manager;
mod statistics;

// Imports
use graph_manager::GraphManager;
use statistics::Statistics;
use petgraph::graph::{NodeIndex, UnGraph}; // Correctly import UnGraph here
use petgraph::visit::EdgeRef; // Import EdgeRef trait for target() method
use rand::{rngs::StdRng, SeedableRng, prelude::*};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    // Reading the txt file
    let file = File::open("facebook_combined.txt")?;
    let reader = io::BufReader::new(file);

    // Creating and building the graph
    let mut graph_manager = GraphManager::new();
    graph_manager.build_graph(reader)?;

    // Seeding the random number generator for consistent results
    let seed = 12345; 
    let mut rng = StdRng::seed_from_u64(seed); // Seeded RNG

    let nodes = graph_manager.get_node_indices(); // List of all node indices

    let mut statistics = Statistics::new();
    for _ in 0..1000 { // Randomly selecting 1000 pairs of nodes
        let &start = nodes.choose(&mut rng).unwrap();
        let &end = nodes.choose(&mut rng).unwrap();
        if let Some(length) = bfs_shortest_path(&graph_manager.graph, start, end) {
            statistics.add_length(length);
        }
    }
    // Using the methods to calc all three values
    let (average_path_length, median, std_deviation) = statistics.compute();  
    println!("Average shortest path length: {}", average_path_length);
    println!("Median of shortest path lengths: {}", median);
    println!("Standard deviation of shortest path lengths: {:.2}", std_deviation);

    // Checking if the avg degree of separation is under 6 and supports the small world hypothesis
    if average_path_length <= 6.0 {
        println!("The average degree of separation supports the 'small world hypothesis'.");
    } else {
        println!("The average degree of separation does not support the 'small world hypothesis'.");
    }

    let mut reach_counts = vec![];
    let total_nodes = graph_manager.node_count() as f64; // Total num of nodes in the graph
    for _ in 0..1000 { // Checking reachability from 1000 randomly selected nodes
        let &start = nodes.choose(&mut rng).unwrap();
        let reach_count = bfs_reachability(&graph_manager.graph, start, 6); // Calculating how many nodes are reachable within 6 steps
        reach_counts.push(reach_count); // Storing result in our vector
    }
    
    let average_reach: f64 = reach_counts.iter().sum::<u32>() as f64 / reach_counts.len() as f64; 
    let percentage_reachable = (average_reach / total_nodes) * 100.0; // Calculate the average number of reachable nodes then into a percentage of the network
    println!("Average percentage of network reachable within 6 steps: {:.2}%", percentage_reachable);

    // Calculate the average clustering coefficient
    let mut clustering_coefficients = vec![];
    for node in graph_manager.graph.node_indices() { // Iterate through each node in the graph
        let neighbors: Vec<NodeIndex> = graph_manager.graph.neighbors(node).collect(); // Collecting the neighbors of a node
        let mut edges_between_neighbors = 0; // Counting the edges between neighbors 
        // Nested loop to count the number of edges between the neighbors of the current node
        for (index, &neighbor) in neighbors.iter().enumerate() {
            for edge in graph_manager.graph.edges(neighbor) {
                let target = edge.target();
                // Making sure that each edge is only counted once
                if neighbors.contains(&target) && neighbors.iter().position(|&n| n == target).unwrap() > index {
                    edges_between_neighbors += 1;
                }
            }
        }
        // Calculate the clustering coefficient for the node
        let total_possible_edges = neighbors.len() * (neighbors.len() - 1) / 2; // Maximum number of edges between all neighbors
        let clustering_coefficient = if total_possible_edges > 0 { // Ratio of actual edges between neighbors to total possible edges
            edges_between_neighbors as f64 / total_possible_edges as f64
        } else { // If there are no edges, then it's 0
            0.0
        };
        // Storing the coefficient
        clustering_coefficients.push(clustering_coefficient);
    }
    // Getting the avg coefficient by summing them and dividing by the num of nodes
    let average_clustering_coefficient: f64 = clustering_coefficients.iter().sum::<f64>() / clustering_coefficients.len() as f64;
    println!("Average clustering coefficient: {:.4}", average_clustering_coefficient);

    Ok(())
}



// Def the BFS algorithm 
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

#[cfg(test)]
mod tests {
    use super::*;

    // Testing the BFS alg for correctness on a 3 node graph
    #[test] // Indicating the func is a test case 
    fn test_bfs_shortest_path() {
        let mut graph = UnGraph::<(), u32>::new_undirected(); // Creating a new, undirected graph to test
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(()); // Adding nodes 1-3
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n3, 1); // And edges between them 
        // making sure that the shortest path between n1 and n3 is 2 edges
        assert_eq!(bfs_shortest_path(&graph, n1, n3), Some(2)); 
    }

    // Test the BFS reachability within 6 steps.
    #[test]
    fn test_bfs_reachability() {
        let mut graph = UnGraph::<(), u32>::new_undirected(); // INitializing a new test graph
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        let n4 = graph.add_node(());
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n3, 1);
        graph.add_edge(n3, n4, 1);

        // Since n4 is 3 steps away from n1, it should be reachable within 6 steps
        assert_eq!(bfs_reachability(&graph, n1, 6), 4);
    }

    // Testing mean, median, and SD values
    #[test]
    fn test_statistics_computation() {
        let mut stats = Statistics::new(); // Creating a new instance of the Statistics struct
        stats.add_length(1);
        stats.add_length(2);
        stats.add_length(3);
        stats.add_length(4);
        stats.add_length(5); // Adding individual data pts to the instance to test the correctness
        // Now testing if each value is the expected value 
        let (mean, median, std_dev) = stats.compute(); 
        assert_eq!(mean, 3.0);
        assert_eq!(median, 3);
        assert_eq!((std_dev * 1000.0).round() / 1000.0, 1.414); // Rounded for comparison.
    }
}
