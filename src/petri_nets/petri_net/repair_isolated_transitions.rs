use petgraph::{Directed, Direction};
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriNetTransitionLabelBound, PetriPlace};
use crate::regex_inf::regex::regex::Regex;

impl<T: PetriNetTransitionLabelBound> PetriNet<T> {
    
    pub fn repair_isolated_transitions(&mut self){
        for transitions in self.transitions.clone().iter(){
            let transition_index = self.get_node_index(&PetriNode::Transition(transitions.clone())).unwrap();
            
            let outgoing_neighbors = self.graph.neighbors_directed(transition_index,Direction::Outgoing).collect::<Vec<_>>();
            if outgoing_neighbors.len() == 0{
                let new_place = PetriPlace::new_from_nothing();
                self.add_place(&new_place);
                self.update_edge(&PetriNode::Transition(transitions.clone()),&PetriNode::Place(new_place.clone()));
            }
        }
    }
}





