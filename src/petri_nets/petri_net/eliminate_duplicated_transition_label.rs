use graphviz_rust::attributes::label;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::PetriTransition;
use crate::regex_inf::regex::regex::Regex;

impl PetriNet<Regex<String>>{
    pub fn eliminate_duplicated_transition_label(&mut self){

        let mut new_transitions = Vec::new();
        for transition in self.transitions.clone().iter(){
            let new_label = if let Regex::Literal(label) = transition.label.clone(){
                let temp = match label.rsplit_once('/') {
                    Some((prefix, suffix)) if suffix.parse::<usize>().is_ok() => prefix,
                    _ => &label,
                };
                //Maybe do this better??
                if temp=="Epsilon"{
                    Regex::Epsilon
                }
                else{
                    Regex::Literal(temp.to_string())
                }

            }
            else{
                transition.label.clone()
            };

            //println!("{:?}",new_label);
            //We update the node
            let new_transition = PetriTransition::new(&transition.id, &new_label);
            let transition_index = self.get_node_index(&PetriNode::Transition(transition.clone())).unwrap();
            self.graph[transition_index] = PetriNode::Transition(new_transition.clone());
            new_transitions.push(new_transition);
        }

        self.transitions = new_transitions;
    }
}