// module's purpose: measures of similarity, including clustering coefficient, which shows likelihood that friends of friends are also friends, and jaccard similarity, which shows overlap between friends

pub mod similarities {
    use crate::graphs::graphs::Graph;
    
    pub fn clustering_coefficient(graph: &Graph, node: usize) -> f64 { // how likely that friends of friends are also friends
        if graph.outedges[node].len() < 2 { // if the node has 0 or 1 adjacent nodes, this prevents division by 0 and subsequent return of NaN from the final step
            return 0.0;
        }
        let mut connection_count = 0; // counts the friend connections shared by the node's friends
        for i in 0..graph.outedges[node].len() {
            for j in (i+1)..graph.outedges[node].len() {
                let u = graph.outedges[node][i]; // friend 1
                let v = graph.outedges[node][j]; // friend 2
                if graph.outedges[u].contains(&v) || graph.outedges[v].contains(&u) { // if u is linked to v or if v is linked to u, increase connection count
                    connection_count += 1;
                }
            }
        }
        2.0 * connection_count as f64 / (graph.outedges[node].len() as f64 * (graph.outedges[node].len() as f64 - 1.0)) // value closer to 1 means all friends are friends with each other (or in Amazon link case, all products are featured on each other's pages), 0 means none of the node's friends are friends with each other
    }
    pub fn jaccard_similarity(graph:&Graph, node1: usize, node2: usize) -> f64 { // calculates jaccard similarity between two nodes - seeing if friends overlap
        let node1_friends = graph.outedges[node1].clone(); // friends of first node
        let node2_friends = graph.outedges[node2].clone(); // friends of second node
        let mut intersection = 0; // intersection - elements shared by both nodes
        let mut intersected = vec![]; // keep track of shared values that were found
        let mut union = 0; // union - elements found in either node
        for index in 0..node1_friends.len() {
            if node2_friends.contains(&node1_friends[index]) {
                intersection += 1;
                intersected.push(node1_friends[index].clone());
            }
            union += 1;
        }
        for index in 0..node2_friends.len() {
            if intersected.contains(&node2_friends[index]) == false {
                union += 1; // avoids double count of intersected values
            }
        }
        intersection as f64 / union as f64
    }
    pub fn full_similarity(graph: &Graph) -> Vec<usize> { // returns vec with vertices whose friends are all friends with each other (clustering coeff of 1.0)
        let mut final_vec = vec![];
        for node in 0..graph.n { // iterate through nodes
            let cluster_coeff = clustering_coefficient(graph, node);
            if cluster_coeff == 1.0 {
                final_vec.push(node);
            }
        }
        final_vec
    }
    pub fn no_similarity(graph: &Graph) -> Vec<usize> { // returns vec with vertices with clustering coefficient of 0 - no similarity between friends
        let mut final_vec = vec![];
        for node in 0..graph.n { // iterate through nodes
            let cluster_coeff = clustering_coefficient(graph, node);
            if cluster_coeff == 0.0 {
                final_vec.push(node);
            }
        }
        final_vec
    }
    pub fn greatest_jaccard(graph: &Graph) -> (usize, usize) { // returns pair of friends that have the greatest level of similarity with each other (closest to 1)
        let mut largest_sim = 0.0;
        let mut final_pair = (0, 1);
        for node1 in 0..graph.n { // first node
            for node2 in graph.outedges[node1].clone() { // second node, from node1's friends
                let jaccard_sim = jaccard_similarity(graph, node1, node2);
                if jaccard_sim > largest_sim {
                    final_pair = (node1, node2);
                    largest_sim = jaccard_sim;
                }
            }
        }
        final_pair
    }
    pub fn least_jaccard(graph:&Graph) -> (usize, usize) { // returns pair of friends that have the lowest level of similarity with each other (closest to 0)
        let mut lowest_sim = 1.0;
        let mut final_pair = (0, 1);
        for node1 in 0..graph.n { // first node
            for node2 in graph.outedges[node1].clone() { // second node, from node1's friends
                let jaccard_sim = jaccard_similarity(graph, node1, node2);
                if jaccard_sim < lowest_sim {
                    final_pair = (node1, node2);
                    lowest_sim = jaccard_sim;
                }
            }
        }
        final_pair
    }
}

#[cfg(test)]
mod test {
    use super::similarities::jaccard_similarity;
    use crate::{graphs::graphs::Graph, similarities::similarities::clustering_coefficient};

    #[test]
    fn test_clustering() { // test of clustering coefficient
        let graph = Graph::create_directed(4, &vec![(1, 2), (2, 3), (0, 2), (0, 1), (0, 3)]); // creating simple sample graph
        let clustering_coeff = clustering_coefficient(&graph, 0); // clustering coefficient for node 0
        assert_eq!(clustering_coeff, 2.0/3.0); // the clustering coefficient should be 2/3
    }
    
    #[test]
    fn test_jaccard() { // test of jaccard similarity
        let graph = Graph::create_directed(6, &vec![(0, 1), (0, 2), (0, 3), (0, 5), (1, 2), (1, 3), (1, 4)]); // sample graph
        let jaccard_sim = jaccard_similarity(&graph, 0, 1); // similarity between nodes 0 and 1
        assert_eq!(jaccard_sim, 2.0/5.0);
    }
}