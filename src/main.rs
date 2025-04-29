use std::fs::File;        // object providing access to an open file on filesystem
use std::io::BufRead; // bufread for files
mod graphs; // import graphs module
use crate::graphs::graphs::Graph; // use struct Graph as declared in graphs
use crate::graphs::graphs::Vertex; // use the types declared in graphs
use crate::graphs::graphs::ListOfEdges;
use std::collections::HashMap;
use std::collections::VecDeque;

fn create_edges(path: &str) -> ListOfEdges { // creating edge list from file, similar to function created from hw7
    let mut result: Vec<(usize, usize)> = Vec::new(); // making the vector of edge tuples that will be returned
    let file = File::open(path).expect("Could not open file");

    // Create a buffered reader for the file
    // .lines() returns an iterator over the lines of the file
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut values_reached = false; // since the first few lines of the txt file don't contain nodes

    for line in buf_reader { // returns a Result<String, Error>
        // Unwrap or print error
        let line_str = line.expect("Error reading");
        // Split the line into a vector of strings
        let v: Vec<&str> = line_str.trim().split('\t').collect(); // split at whitespace and collect into a vector (convert iterator to vector)
        if v.len() == 2 { // adds edges for lines w/ from and to nodes
            if values_reached == false { // since the third line returns a vec with len 2 but still isn't the nodes
                values_reached = true;
            }
            else {
                let x = v[0].parse::<usize>().unwrap();
                let y = v[1].parse::<usize>().unwrap();
                // Add the x-y tuple to the result vector
                result.push((x, y));
            }
        }
    }
    return result;
}

fn bfs(graph: &Graph, start: usize) -> (usize, u32) { // given graph w/ adjacency lists, returns node and max
    let mut distance: Vec<Option<u32>> = vec![None; graph.n]; // starts adj list at none
    distance[start] = Some(0); // adds start node 0

    let mut queue: VecDeque<usize> = VecDeque::new(); 
    queue.push_back(start); // queue contains start node 0

    let mut farthest_node = start; 
    let mut max_distance = 0; // max distance will be adjusted as it increases

    while let Some(v) = queue.pop_front() { // while there are nodes to deque/pop - new unprocessed vertex
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
                if distance[*u].unwrap() > max_distance { // if the distance of u is greater than the max distance
                    max_distance = distance[*u].unwrap(); 
                    farthest_node = *u; // set equal to u index
                }
            }
        }
    }

    return (farthest_node, max_distance); // return farthest node and max distance
}

fn main() {
    let edges: Vec<(usize, usize)> = create_edges("amazon0601.txt"); // creating edges from txt file
    let graph = Graph::create_directed(403394,&edges); // creating graph out of edges with 403394 nodes
    graph.print_adjlists();
    println!("{:?}", bfs(&graph, 0));
}