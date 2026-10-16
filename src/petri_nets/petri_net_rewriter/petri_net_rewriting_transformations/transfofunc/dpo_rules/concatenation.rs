use std::collections::BTreeSet;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;


pub fn can_apply_concatenation(petri: &PetriNet<Regex<String>>) -> bool {
    if ConcatenationPattern::search_pattern(petri).len()>0{
        println!("Concatenation found");
        return true;
    }
    false
}


pub fn apply_concatenation(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
   for pattern in ConcatenationPattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        ConcatenationPattern::replace_pattern(&mut new_petri_net,&pattern);
        return vec![new_petri_net];
    }
    panic!("We have not found a transformation for concatenation, while the precondition is satisfied.");
}


pub struct ConcatenationPattern{
    pub frontier: (BTreeSet<PetriPlace>, BTreeSet<PetriPlace>),
    pub redex: (PetriTransition<Regex<String>>,BTreeSet<PetriPlace> ,PetriTransition<Regex<String>>)
}

impl ConcatenationPattern{
    pub fn new(frontier: (BTreeSet<PetriPlace>, BTreeSet<PetriPlace>), redex: (PetriTransition<Regex<String>>,BTreeSet<PetriPlace> ,PetriTransition<Regex<String>>)) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<ConcatenationPattern>{

        //////NEW VERSION
        let mut patterns = Vec::new();
        'global_loop: for pair in petri.transitions.iter().permutations(2){
            let (t1,t2) = (pair[0].clone(),pair[1].clone());
            
            if petri.postset_transition(&t1)==petri.preset_transition(&t2.clone()){
                let P_m = petri.postset_transition(&t1);
                //Check of conditions on P_m
                //let p_m_witness = P_m.first().unwrap();
                //let witness_preset = petri.preset_generic(&PetriNode::Place(p_m_witness.clone()));
                //let witness_postset = petri.postset_generic(&PetriNode::Place(p_m_witness.clone()));

                let witness_preset = BTreeSet::from([PetriNode::Transition(t1.clone())]);
                let witness_postset = BTreeSet::from([PetriNode::Transition(t2.clone())]);

                for p in P_m.iter(){
                    if petri.preset_generic(&PetriNode::Place(p.clone())) != witness_preset || petri.postset_generic(&PetriNode::Place(p.clone())) != witness_postset{
                        continue 'global_loop;
                    }
                }

                //Wrapping up
                let P_in = petri.preset_transition(&t1);
                let P_out = petri.postset_transition(&t2);
                
                let concat_pattern = ConcatenationPattern::new((P_in,P_out),(t1,P_m,t2));
                patterns.push(concat_pattern);
            }
        }
        
        patterns
    }
    
    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &ConcatenationPattern){
        
        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.0.clone()));
        for p in pattern.redex.1.iter(){
            petri.remove_node(&PetriNode::Place(p.clone()));
        }
        petri.remove_node(&PetriNode::Transition(pattern.redex.2.clone()));
        
        //Creation

        let regex = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&regex);
        petri.add_transition(&new_transition);
        
        for p in pattern.frontier.0.iter(){
            petri.update_edge(&PetriNode::Place(p.clone()), &PetriNode::Transition(new_transition.clone()));
        }
        
        for p in pattern.frontier.1.iter(){
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p.clone()));
        }
    }
    pub fn get_synthetic_regex(&self)->Regex<String>{
        let mut regex = Regex::Empty;
        regex =  Regex::concatenate(&regex, &self.redex.0.label);
        regex =  Regex::concatenate(&regex, &self.redex.2.label);
        
        regex
    }


}