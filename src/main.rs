use std::fs::File; // object providing access to an open file on filesystem
use std::io::BufRead; // bufread for files
mod graphs; use graphs::graphs::Graph;
use graphs::graphs::ListOfEdges;
mod similarities; use similarities::similarities::greatest_jaccard;
use similarities::similarities::jaccard_similarity;
use similarities::similarities::least_jaccard;
use similarities::similarities::no_similarity;
use similarities::similarities::full_similarity;

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

fn main() {
    let edges: Vec<(usize, usize)> = create_edges("amazon0601.txt"); // creating edges from txt file
    let graph = Graph::create_directed(403394,&edges); // creating graph out of edges with 403394 nodes
    println!("greatest jaccard similarity: {:?}", greatest_jaccard(&graph));
    println!("similarity: {:?}", jaccard_similarity(&graph, 34158, 25793));
    println!("smallest jaccard similarity: {:?}", least_jaccard(&graph));
    println!("similarity: {:?}", jaccard_similarity(&graph, 1, 4));
    println!("number of nodes that have a clustering coefficient of 1.0 - all their friends are friends: {:?}", full_similarity(&graph).len());
    println!("number of nodes that have a clustering coefficient of 0.0 - none of their friends are friends: {:?}", no_similarity(&graph).len());
}