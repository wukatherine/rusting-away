// module's purpose: Graph struct to represent a graph, with methods to add directed edges, sort the graph lists, and create a directed graph, used to represent the data from the txt file. 

pub mod graphs {
    // Define some datatype synonyms
    pub type Vertex = usize;
    pub type ListOfEdges = Vec<(Vertex,Vertex)>;
    pub type AdjacencyLists = Vec<Vec<Vertex>>;

    #[derive(Debug)]
    pub struct Graph {
        pub n: usize, // vertex labels in {0,...,n-1}
        pub outedges: AdjacencyLists,
    }

    impl Graph { // used lecture12 and lect14 as a guide
        pub fn add_directed_edges(&mut self, edges:&ListOfEdges) { // adds directed edges to the Graph given a list of edges
            for (u,v) in edges {
                self.outedges[*u].push(*v);
            }
        }
        pub fn sort_graph_lists(&mut self) { // sorts the graph's lists, mostly used in the create_directed function
            for l in self.outedges.iter_mut() {
                l.sort();
            }
        }
        pub fn create_directed(n:usize,edges:&ListOfEdges) -> Graph { // very useful function for creating a directed graph from the Amazon data
            let mut g = Graph{n,outedges:vec![vec![];n]}; // initialize Graph with n and empty vec of vec with n entries
            g.add_directed_edges(edges); // adds the directed edges to outedges
            g.sort_graph_lists(); // sorts each list in the Graph
            g                                        
        }
    }
}