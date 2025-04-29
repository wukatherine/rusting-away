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
        pub fn add_directed_edges(&mut self, edges:&ListOfEdges) {
            for (u,v) in edges {
                self.outedges[*u].push(*v);
            }
        }
        pub fn sort_graph_lists(&mut self) {
            for l in self.outedges.iter_mut() {
                l.sort();
            }
        }
        pub fn create_directed(n:usize,edges:&ListOfEdges) -> Graph { // very useful function for creating a directed graph from the Amazon data
            let mut g = Graph{n,outedges:vec![vec![];n]};
            g.add_directed_edges(edges);
            g.sort_graph_lists();
            g                                        
        }
        pub fn print_adjlists(&self) { // function for printing the graph's adjacency lists
            for vertex in 0..self.n { // iterate through vertex labels
                println!("{}: {:?}", vertex, self.outedges[vertex]);
            }
        }
    }
}