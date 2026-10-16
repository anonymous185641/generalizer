use std::collections::{BTreeMap, BTreeSet};
use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::product::sync_product::sync_product::SyncProduct;

impl SyncProduct {

    pub fn to_automaton_string(&self)->Automaton{

        let mut automaton = Automaton::new();

        let mut node_correspondence = BTreeMap::new();

        //Adding nodes
        for node in self.graph.node_indices(){
            let weight = self.graph[node].clone();
            let mut w = "q".to_string();
            for q in weight.state{
                w.push_str(&q.index().to_string());
                w.push_str("_");
            }
            w.pop();
            let new_node = automaton.graph.add_node(w.to_string());
            node_correspondence.insert(node,new_node);
        }

        //Adding edges
        for edge in self.graph.edge_indices(){
            let (src,tgt) = self.graph.edge_endpoints(edge).unwrap();
            let label = self.graph.edge_weight(edge).unwrap();
            automaton.graph.add_edge(node_correspondence[&src],node_correspondence[&tgt],label.to_string());
        }

        //Initial and final states
        automaton.initial_states = vec![node_correspondence[&self.initial_state.clone()]];

        for node in self.final_states.iter(){
            automaton.final_states.push(node_correspondence[node]);
        }

        automaton
    }



}















