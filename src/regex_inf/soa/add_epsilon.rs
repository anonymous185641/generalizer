use petgraph::Direction;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::soa::soa::SOA;

impl SOA{
    pub fn add_epsilon(&mut self) { //OK
        // description ..
        let mut bad_src_successor = Vec::new();
        let src = self.src();

        //We add a new vertex labelled Epsilon
        let epsilon_node = self.soa.add_node(Regex::Epsilon);
        self.soa.update_edge(src,epsilon_node,());
        //We collect all the 'bad' successors of src
        for node in self.soa.node_indices() {
            // Check if the node has only the given node as its predecessor
            let predecessors: Vec<_> = self.soa.neighbors_directed(node, Direction::Incoming).collect();
            if predecessors.len() > 1 && predecessors.contains(&src)  {
                bad_src_successor.push(node);
            }
        }

        //Redirection
        for bad_succ in bad_src_successor{
            //We first delete the edge between src and bad_succ
            self.soa.remove_edge(self.soa.find_edge(src,bad_succ).unwrap());

            //We add a new edge between epsilon and  bad_succ
            self.soa.update_edge(epsilon_node,bad_succ,());
        }

    }
}