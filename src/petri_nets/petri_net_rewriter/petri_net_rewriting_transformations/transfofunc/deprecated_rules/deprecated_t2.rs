use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t2(petri: &PetriNet<Regex<String>>) ->bool{

    for pair in petri.places.iter().permutations(2) {
        let (p1, p2) = (pair[0].clone(), pair[1].clone());
        if petri.is_there_alt_pattern(&p1,&p2).len()>1{
            return true;
        }
    }

    false
}


pub fn apply_t2(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{

    for pair in petri.places.iter().permutations(2) {
        let (p1, p2) = (pair[0].clone(), pair[1].clone());
        let alt_transitions = petri.is_there_alt_pattern(&p1,&p2);

        if alt_transitions.len() > 1 {
            let mut new_petri_net = petri.clone();
            new_petri_net.merge_alt_pattern(&p1,&p2,&alt_transitions);
            return vec![new_petri_net];
        }
    }

    panic!("We have not found a transformation for T2, while the precondition is satisfied.");
}


impl PetriNet<Regex<String>>{
    pub fn is_there_alt_pattern(&self,p1: &PetriPlace,p2: &PetriPlace)->Vec<PetriTransition<Regex<String>>>{
        let mut alt_transitions = Vec::new();

        for trans in self.transitions.iter(){

            /* We check whether p1 is the only predecessor of trans and p2 is the only successor of trans */
            let pred_trans = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(trans.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();
            let succ_trans = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(trans.clone())).unwrap(),petgraph::Direction::Outgoing).collect::<Vec<_>>();


            if pred_trans.len()==1 && succ_trans.len()==1{
                if self.graph[pred_trans[0]]==PetriNode::Place(p1.clone()) && self.graph[succ_trans[0]]==PetriNode::Place(p2.clone()){
                    alt_transitions.push(trans.clone());
                }

            }
        }

        alt_transitions
    }

    pub fn merge_alt_pattern(&mut self,p1: &PetriPlace, p2: &PetriPlace,alt_transitions: &Vec<PetriTransition<Regex<String>>>){
        let mut regex = Regex::Empty;
        for k in 0..alt_transitions.len(){
            regex = Regex::or(&regex,&alt_transitions[k].label);
        }

        //regex = alt_transitions.iter().map(|x| x.name.clone()).join(")|(");
        //regex.push_str(")");

        let new_transition = PetriTransition::new_from_name(&regex);
        //We add the new transition
        self.add_transition(&new_transition);

        for trans in alt_transitions.iter(){
            //We remove the old transitions
            self.remove_node(&PetriNode::Transition(trans.clone()));
            //We add the new edges
            self.update_edge(&PetriNode::Place(p1.clone()),&PetriNode::Transition(new_transition.clone()));
            self.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p2.clone()));
        }
    }
}