use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t4(petri: &PetriNet<Regex<String>>) ->bool{

    for pair in petri.places.iter().permutations(2){
        let (p1,p2) = (pair[0].clone(),pair[1].clone());
        if !petri.is_there_loop_alt_pattern(&p1.clone(),&p2.clone()).is_empty(){
            return true;
        }
    }

    false
}

pub fn apply_t4(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{

    for pair in petri.places.iter().permutations(2){
        let (p1,p2) = (pair[0].clone(),pair[1].clone());
        let transitions_loop_alt_pattern = petri.is_there_loop_alt_pattern(&p1.clone(),&p2.clone());
        if transitions_loop_alt_pattern.is_empty(){
           continue;
        }

        let mut new_petri_net = petri.clone();

        //Retrieve regex and delete nodes
        let mut regex_vec = Vec::new();
        for t in transitions_loop_alt_pattern.iter(){
            regex_vec.push(t.label.clone());
            new_petri_net.remove_node(&PetriNode::Transition(t.clone()));
        }

        let regex = Regex::plus(&Regex::or_with_plus_to_kleene_reduction_with_epsilon(&regex_vec));

        let new_transition = PetriTransition::new_from_name(&regex);
        new_petri_net.add_transition(&new_transition);

        new_petri_net.update_edge(&PetriNode::Place(p1.clone()),&PetriNode::Transition(new_transition.clone()));
        new_petri_net.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p2.clone()));

        return vec![new_petri_net];
    }



    panic!("We have not found a transformation for T4, while the precondition is satisfied.");
}


impl PetriNet<Regex<String>>{
    pub fn is_there_loop_alt_pattern(&self,p1: &PetriPlace,p2: &PetriPlace)->Vec<PetriTransition<Regex<String>>>{




        let paths_p1_to_p2 = all_simple_paths::<Vec<_>, _>(&self.graph,
                                                           self.get_node_index(&PetriNode::Place(p1.clone())).unwrap(),
                                                           self.get_node_index(&PetriNode::Place(p2.clone())).unwrap(),
                                                           1,Some(1)).collect::<Vec<_>>();


        let paths_p2_to_p1 = all_simple_paths::<Vec<_>, _>(&self.graph,
                                                           self.get_node_index(&PetriNode::Place(p2.clone())).unwrap(),
                                                           self.get_node_index(&PetriNode::Place(p1.clone())).unwrap(),
                                                           1,Some(1)).collect::<Vec<_>>();

        if paths_p1_to_p2.len() + paths_p2_to_p1.len()<3{
            return vec![];
        }

        let mut intermediary_transitions = Vec::new();

        for path in paths_p1_to_p2.iter().chain(paths_p2_to_p1.iter()){
            let node = self.graph[path[1]].clone();
            if let PetriNode::Transition(t) = node{
                intermediary_transitions.push(t);
            }else{
                panic!("There must be a transition between two places.");
            }
        }

        intermediary_transitions
    }




}