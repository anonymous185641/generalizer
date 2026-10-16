use std::collections::BTreeSet;
use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_loop1(petri: &PetriNet<Regex<String>>) -> bool {
    if Loop1Pattern::search_pattern(petri).len()>0{
        println!("Found a loop1");
        return true;
    }
    false
}



pub fn apply_loop1(petri: &PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
    let mut results = Vec::new();
    for pattern in Loop1Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        Loop1Pattern::replace_pattern(&mut new_petri_net,&pattern);
        results.push(new_petri_net);
    }
    results
}



pub struct Loop1Redex{
    pub t1: PetriTransition<Regex<String>>,
    pub t2: PetriTransition<Regex<String>>,
    pub t3: PetriTransition<Regex<String>>,
    pub p1: PetriPlace,
}

pub struct Loop1Pattern{
    pub frontier: (BTreeSet<PetriPlace>,BTreeSet<PetriPlace>),
    pub redex: Loop1Redex,
}


impl Loop1Pattern{
    pub fn new(frontier: (BTreeSet<PetriPlace>,BTreeSet<PetriPlace>), redex: Loop1Redex) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<Loop1Pattern>{
        let mut patterns = Vec::new();
        if petri.transitions.len()<3{
            return patterns;
        }

        for triple in petri.transitions.iter().permutations(3){
            let (t1,t2,t3) = (triple[0].clone(),triple[1].clone(),triple[2].clone());


            let post_t1 = petri.postset_transition(&t1);

            let pre_t2 = petri.preset_transition(&t2);
            let post_t2 = petri.postset_transition(&t2);

            let pre_t3 = petri.preset_transition(&t3);


            if post_t1==pre_t2 && pre_t2 == post_t2 && pre_t3==post_t2 && post_t1.len()==1{
                let P_in = petri.preset_transition(&t1);
                let P_out = petri.postset_transition(&t3);

                let loop1_redex = Loop1Redex{t1: t1.clone(),t2: t2.clone(), t3: t3.clone(), p1: pre_t2.first().unwrap().clone()};
                let loop1_pattern = Loop1Pattern::new((P_in,P_out),loop1_redex);
                patterns.push(loop1_pattern);
            }
        }

        patterns
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &Loop1Pattern){
        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.t2.clone()));
        petri.remove_node(&PetriNode::Place(pattern.redex.p1.clone()));

        //Creation
        let regex = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&regex);
        petri.add_transition(&new_transition);

        let p2 = PetriPlace::new_from_nothing();
        petri.add_place(&p2);

        let p3 = PetriPlace::new_from_nothing();
        petri.add_place(&p3);

       petri.update_edge(&PetriNode::Transition(pattern.redex.t1.clone()),&PetriNode::Place(p2.clone()));
        petri.update_edge(&PetriNode::Place(p2.clone()),&PetriNode::Transition(new_transition.clone()));
        petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p3.clone()));
        petri.update_edge(&PetriNode::Place(p3.clone()),&PetriNode::Transition(pattern.redex.t3.clone()));

    }
    pub fn get_synthetic_regex(&self)->Regex<String>{
         Regex::kleene(&self.redex.t2.label)
    }
}





















