use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t5(petri: &PetriNet<Regex<String>>) ->bool{
    for t in petri.transitions.iter(){
        if let Some(p) = petri.is_there_simple_loop_pattern(&t){
            return true;
        }
    }

    false
}

pub fn apply_t5(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
    for t in petri.transitions.iter(){
        if let Some(p) = petri.is_there_simple_loop_pattern(&t){
            let mut new_petri_net = petri.clone();
            
            let new_place = PetriPlace::new_from_nothing();

            //To replace

            let new_transition = PetriTransition::new_from_name(&Regex::plus(&t.label.clone()));
            
            new_petri_net.remove_node(&PetriNode::Transition(t.clone())); 
            
            new_petri_net.add_place(&new_place);
            new_petri_net.add_transition(&new_transition);
            
            new_petri_net.update_edge(&PetriNode::Place(p.clone()),&PetriNode::Transition(new_transition.clone()));
            new_petri_net.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(new_place.clone()));
            
            return vec![new_petri_net];
        }
    }
    
    panic!("We have not found a transformation for T5, while the precondition is satisfied.");
}

impl PetriNet<Regex<String>>{
    pub fn is_there_simple_loop_pattern(&self,t: &PetriTransition<Regex<String>>)->Option<PetriPlace>{
        let predecessors_of_t = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();
        let successors_of_t = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),petgraph::Direction::Outgoing).collect::<Vec<_>>();
        
        if predecessors_of_t.len()==1 && successors_of_t.len()==1{
            let prec_node = self.graph[predecessors_of_t[0]].clone();
            let succ_node = self.graph[successors_of_t[0]].clone();
            if let (PetriNode::Place(p1),PetriNode::Place(p2)) = (prec_node,succ_node){
                if p1==p2{
                    return Some(p1);
                }
            }
        } 
        None
    }
}