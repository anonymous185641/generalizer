use std::collections::BTreeSet;
use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop1::Loop1Pattern;
use crate::regex_inf::regex::regex::Regex;


pub fn can_apply_loop0(petri: &PetriNet<Regex<String>>) -> bool {
    if Loop0Pattern::search_pattern(petri).len()>0{
        println!("Found a loop0");
        return true;
    }
    false
}



pub fn apply_loop0(petri: &PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
    let mut results = Vec::new();
    for pattern in Loop0Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        Loop0Pattern::replace_pattern(&mut new_petri_net,&pattern);
        results.push(new_petri_net);
    }
    results
}


#[derive(Ord, Eq, PartialEq, PartialOrd)]
#[derive(Clone)]
pub struct Loop0Pattern{
    pub looped_transition: PetriTransition<Regex<String>>,
    pub looped_places: BTreeSet<PetriPlace>,
}


impl Loop0Pattern{
    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<Loop0Pattern>{
        let mut patterns = BTreeSet::new();

        for trans in petri.transitions.iter(){

            let preset_trans = petri.preset_transition(&trans);
            let postset_trans = petri.postset_transition(&trans);

            if preset_trans==postset_trans{
                let pattern = Loop0Pattern{looped_transition: trans.clone(),looped_places: preset_trans.clone()};
                patterns.insert(pattern);
            }

        }

        //We make sure that no two transition have the same set of places
        for (pt1,pt2) in patterns.clone().iter().tuple_combinations(){
            if pt1.looped_places==pt2.looped_places{
                patterns.remove(&pt1);
                patterns.remove(&pt2);
            }
        }

        //return
       patterns.into_iter().collect()
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &Loop0Pattern){

        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.looped_transition.clone()));

        //Creation
        let label = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&label);
        petri.add_transition(&new_transition);


        for p in pattern.looped_places.iter(){
            let p_prime = PetriPlace::new_from_nothing();
            petri.add_place(&p_prime);

            let outgoint_transtitiion = petri.postset_place(&p);

            //We delete outgoing edges from place p
            for t in outgoint_transtitiion.iter(){
                petri.remove_edge(&PetriNode::Place(p.clone()),&PetriNode::Transition(t.clone()));

            }

            //Those outgoing edges are transferred to p_prime
            for t in outgoint_transtitiion.iter(){
                petri.update_edge(&PetriNode::Place(p_prime.clone()),&PetriNode::Transition(t.clone()));
            }

            //Kleene transition in between p and p_prime
            petri.update_edge(&PetriNode::Place(p.clone()),&PetriNode::Transition(new_transition.clone()));
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p_prime.clone()));

        }

    }

    pub fn get_synthetic_regex(&self)->Regex<String>{
        Regex::kleene(&self.looped_transition.label)
    }

}