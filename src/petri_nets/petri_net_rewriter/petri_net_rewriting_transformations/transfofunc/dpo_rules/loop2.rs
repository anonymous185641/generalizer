use std::collections::BTreeSet;
use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop1::Loop1Pattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_loop2(petri: &PetriNet<Regex<String>>) -> bool {
    if Loop2Pattern::search_pattern(petri).len()>0{
        println!("Found a loop2");
        return true;
    }
    false
}



pub fn apply_loop2(petri: &PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
    let mut results = Vec::new();
    for pattern in Loop2Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        Loop2Pattern::replace_pattern(&mut new_petri_net, &pattern);
        results.push(new_petri_net);
    }
    results
}


pub struct Loop2Frontier{
    pub p_in : BTreeSet<PetriPlace>,
    pub p_out1: BTreeSet<PetriPlace>,
    pub p_out2: BTreeSet<PetriPlace>
}
pub struct Loop2Redex{
    pub t1: PetriTransition<Regex<String>>,
    pub t2: PetriTransition<Regex<String>>,
    pub t3: PetriTransition<Regex<String>>,
    pub t4: PetriTransition<Regex<String>>,
    pub p1: PetriPlace,
    pub p2: PetriPlace,
}

pub struct Loop2Pattern{
    pub frontier: Loop2Frontier,
    pub redex: Loop2Redex,
}
impl Loop2Pattern{
    pub fn new(frontier: Loop2Frontier,redex: Loop2Redex)->Self{
        Self{frontier,redex}
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<Loop2Pattern>{
        let mut patterns = Vec::new();

        if petri.transitions.len()<4{
            return patterns;
        }

        for quadruplet in petri.transitions.iter().permutations(4){
            let (t1,t2,t3,t4) = (quadruplet[0].clone(),quadruplet[1].clone(),quadruplet[2].clone(),quadruplet[3].clone());

            for pair in petri.places.iter().permutations(2){
                let (p1,p2) = (pair[0].clone(),pair[1].clone());

                let postset_t1 = petri.postset_transition(&t1);

                let postset_t2 = petri.postset_transition(&t2);
                let preset_t2 = petri.preset_transition(&t2);

                let preset_t3 = petri.preset_transition(&t3);

                let preset_t4 = petri.preset_transition(&t4);

                if postset_t1 == BTreeSet::from([p1.clone(),p2.clone()])
                    && preset_t2 == BTreeSet::from([p1.clone(),p2.clone()])
                    && postset_t2 == BTreeSet::from([p1.clone(),p2.clone()])
                    && preset_t3 == BTreeSet::from([p1.clone()])
                    && preset_t4 == BTreeSet::from([p2.clone()]){

                    //A little check on p1 and p2 for dangling connexions
                    if petri.preset_place(&p1) == BTreeSet::from([t1.clone(),t2.clone()])
                        && petri.postset_place(&p1) == BTreeSet::from([t2.clone(),t3.clone()])
                    && petri.preset_place(&p2) == BTreeSet::from([t1.clone(),t2.clone()])
                    && petri.postset_place(&p2) == BTreeSet::from([t2.clone(),t4.clone()]){

                        //Wrapping up
                        let P_in = petri.preset_transition(&t1);
                        let P_out1 = petri.postset_transition(&t3);
                        let P_out2 = petri.postset_transition(&t4);

                        let frontier = Loop2Frontier{p_in: P_in,p_out1: P_out1,p_out2: P_out2};
                        let redex = Loop2Redex{t1: t1.clone(),t2: t2.clone(),t3: t3.clone(),t4: t4.clone(),p1,p2};
                        patterns.push(Loop2Pattern::new(frontier,redex));
                    }



                    }
            }
        }


        patterns
    }


    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &Loop2Pattern){

        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.t2.clone()));
        petri.remove_edge(&PetriNode::Place(pattern.redex.p1.clone()),&PetriNode::Transition(pattern.redex.t3.clone()));
        petri.remove_edge(&PetriNode::Place(pattern.redex.p2.clone()),&PetriNode::Transition(pattern.redex.t4.clone()));

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
        petri.update_edge(&PetriNode::Place(p4.clone()),&PetriNode::Transition(pattern.redex.t4.clone()));

    }

    pub fn get_synthetic_regex(&self)->Regex<String>{
        Regex::kleene(&self.redex.t2.label)
    }
}
