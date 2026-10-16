use std::collections::BTreeSet;
use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop1::Loop1Pattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_loop3(petri: &PetriNet<Regex<String>>) -> bool {
    if Loop3Pattern::search_pattern(petri).len()>0{
        println!("Found a loop3");
        return true;
    }
    false
}



pub fn apply_loop3(petri: &PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
    let mut results = Vec::new();
    for pattern in Loop3Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        Loop3Pattern::replace_pattern(&mut new_petri_net, &pattern);
        results.push(new_petri_net);
    }
    results
}


pub struct Loop3Frontier{
    pub p_in : BTreeSet<PetriPlace>,
    pub p_out: BTreeSet<PetriPlace>,

}
pub struct Loop3Redex{
    pub t1: PetriTransition<Regex<String>>,
    pub t2: PetriTransition<Regex<String>>,
    pub t3: PetriTransition<Regex<String>>,
    pub p1: PetriPlace,
    pub p2: PetriPlace,
}

pub struct Loop3Pattern{
    pub frontier: Loop3Frontier,
    pub redex: Loop3Redex,
}
impl Loop3Pattern{
    pub fn new(frontier: Loop3Frontier,redex: Loop3Redex)->Self{
        Self{frontier,redex}
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<Loop3Pattern>{
        let mut patterns = Vec::new();

        if petri.transitions.len()<3{
            return patterns;
        }

        for triple in petri.transitions.iter().permutations(3){
            let (t1,t2,t3) = (triple[0].clone(), triple[1].clone(), triple[2].clone());

            for pair in petri.places.iter().permutations(2){
                let (p1,p2) = (pair[0].clone(),pair[1].clone());

                let postset_t1 = petri.postset_transition(&t1);

                let postset_t2 = petri.postset_transition(&t2);
                let preset_t2 = petri.preset_transition(&t2);

                let preset_t3 = petri.preset_transition(&t3);

                if postset_t1 == BTreeSet::from([p1.clone(),p2.clone()])
                    && preset_t2 == BTreeSet::from([p1.clone(),p2.clone()])
                    && postset_t2 == BTreeSet::from([p1.clone(),p2.clone()])
                    && preset_t3 == BTreeSet::from([p1.clone(),p2.clone()])
                    {

                    //A little check on p1 and p2 for dangling connexions
                    if petri.preset_place(&p1) == BTreeSet::from([t1.clone(),t2.clone()])
                        && petri.postset_place(&p1) == BTreeSet::from([t2.clone(),t3.clone()])
                        && petri.preset_place(&p2) == BTreeSet::from([t1.clone(),t2.clone()])
                        && petri.postset_place(&p2) == BTreeSet::from([t2.clone(),t3.clone()]){

                        //Wrapping up
                        let P_in = petri.preset_transition(&t1);
                        let P_out = petri.postset_transition(&t3);


                        let frontier = Loop3Frontier{p_in: P_in,p_out: P_out};
                        let redex = Loop3Redex{t1: t1.clone(),t2: t2.clone(),t3: t3.clone(),p1,p2};
                        patterns.push(Loop3Pattern::new(frontier,redex));
                    }



                }
            }
        }


        patterns
    }


    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &Loop3Pattern){

        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.t2.clone()));
        petri.remove_edge(&PetriNode::Place(pattern.redex.p1.clone()),&PetriNode::Transition(pattern.redex.t3.clone()));
        petri.remove_edge(&PetriNode::Place(pattern.redex.p2.clone()),&PetriNode::Transition(pattern.redex.t3.clone()));


        //Creation
        let regex = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&regex);
        petri.add_transition(&new_transition);

        let p3 = PetriPlace::new_from_nothing();
        let p4 = PetriPlace::new_from_nothing();
        petri.add_place(&p3);
        petri.add_place(&p4);

        petri.update_edge(&PetriNode::Place(pattern.redex.p1.clone()),&PetriNode::Transition(new_transition.clone()));
        petri.update_edge(&PetriNode::Place(pattern.redex.p2.clone()),&PetriNode::Transition(new_transition.clone()));
        petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p3.clone()));
        petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p4.clone()));

        petri.update_edge(&PetriNode::Place(p3.clone()),&PetriNode::Transition(pattern.redex.t3.clone()));
        petri.update_edge(&PetriNode::Place(p4.clone()),&PetriNode::Transition(pattern.redex.t3.clone()));

    }

    pub fn get_synthetic_regex(&self)->Regex<String>{
        Regex::kleene(&self.redex.t2.label)
    }
}
