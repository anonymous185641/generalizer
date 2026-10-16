use std::collections::BTreeSet;
use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::concatenation::ConcatenationPattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_alternative(petri: &PetriNet<Regex<String>>) -> bool {
    if  AlternativePattern::search_pattern(petri).len()>0{
        println!("Alternative found");
        return true;
    }

    false
}


pub fn apply_alternative(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    let mut results = Vec::new();
    for alt_pattern in  AlternativePattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        AlternativePattern::replace_pattern(&mut new_petri_net,&alt_pattern);
        //return vec![new_petri_net];
        results.push(new_petri_net);
    }
    results
}




pub struct AlternativePattern{
    pub frontier: (BTreeSet<PetriPlace>,BTreeSet<PetriPlace>),
    pub redex: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>)
}


impl AlternativePattern{
    pub fn new(frontier: (BTreeSet<PetriPlace>,BTreeSet<PetriPlace>), redex: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>)) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<AlternativePattern>{
        /*
        let mut patterns = Vec::new();
        for pair in petri.places.iter().permutations(2) {
            let (p1, p2) = (pair[0].clone(), pair[1].clone());

            let mut alt_transitions = Vec::new();

            for trans in petri.transitions.iter(){

                /* We check whether p1 is the only predecessor of trans and p2 is the only successor of trans */
                let pred_trans = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(trans.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();
                let succ_trans = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(trans.clone())).unwrap(),petgraph::Direction::Outgoing).collect::<Vec<_>>();


                if pred_trans.len()==1 && succ_trans.len()==1{
                    if petri.graph[pred_trans[0]]==PetriNode::Place(p1.clone()) && petri.graph[succ_trans[0]]==PetriNode::Place(p2.clone()){
                        alt_transitions.push(trans.clone());
                    }

                }

                //We stop and return a result if we have two transitions
                if alt_transitions.len() == 2{
                    //return Some(AlternativePattern::new((p1,p2),(alt_transitions[0].clone(),alt_transitions[1].clone())));
                    patterns.push(AlternativePattern::new((p1.clone(),p2.clone()),(alt_transitions[0].clone(),alt_transitions[1].clone())));
                }
            }
        }

        patterns

         */

        ///////NEW
        let mut patterns = Vec::new();

        for (t1,t2) in petri.transitions.iter().tuple_combinations(){

            if  petri.preset_transition(t1)==petri.preset_transition(t2) && petri.postset_transition(t1)==petri.postset_transition(t2) {
                let P_in = petri.preset_transition(t1).clone();
                let P_out = petri.postset_transition(t1).clone();

                let alt_pattern = AlternativePattern::new((P_in.clone(),P_out.clone()),(t1.clone(),t2.clone()));
                patterns.push(alt_pattern);
            }
        }

        patterns
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &AlternativePattern){

        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.0.clone()));
        petri.remove_node(&PetriNode::Transition(pattern.redex.1.clone()));

        //Creation
        let regex = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&regex);
        petri.add_transition(&new_transition);

        //Update

        for p in pattern.frontier.0.iter(){
            petri.update_edge(&PetriNode::Place(p.clone()), &PetriNode::Transition(new_transition.clone()));
        }

        for p in pattern.frontier.1.iter(){
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p.clone()));
        }

    }

    pub fn get_synthetic_regex(&self)->Regex<String>{
        let mut regex = Regex::Empty;
        regex = Regex::or_vec_retain_non_empty(vec![self.redex.0.label.clone(),self.redex.1.label.clone()]);

        regex
    }

}