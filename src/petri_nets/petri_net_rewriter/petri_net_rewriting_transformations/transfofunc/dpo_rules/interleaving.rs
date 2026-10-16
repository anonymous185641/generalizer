use std::collections::BTreeSet;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alternative::AlternativePattern;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::concatenation::ConcatenationPattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_interleaving(petri: &PetriNet<Regex<String>>) -> bool {

    if InterleavingPattern::search_pattern(petri).len()>0{
        println!("Interleaving found");
       return  true;
    }

    false
}


pub fn apply_interleaving(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    let mut results = Vec::new();
    for int_pattern in InterleavingPattern::search_pattern(petri){
        println!("Interleaving found");
        let mut new_petri_net = petri.clone();
        InterleavingPattern::replace_pattern(&mut new_petri_net,&int_pattern);
        results.push(new_petri_net);
    }
    results
}


//pub struct InterleavingFrontier(PetriPlace,PetriTransition<Regex<String>>);
pub struct InterleavingFrontier{
    pub subset1: BTreeSet<PetriPlace>,
    pub subset2: BTreeSet<PetriPlace>,
}
pub struct InterleavingPattern{
    pub frontier: (InterleavingFrontier, InterleavingFrontier),
    pub redex:(PetriTransition<Regex<String>>,PetriTransition<Regex<String>>)
}

impl InterleavingPattern{
    pub fn new(frontier: (InterleavingFrontier, InterleavingFrontier), redex: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>)) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Vec<InterleavingPattern>{
        /*
        for pair in petri.transitions.iter().permutations(2) {
            let (t1, t2) = (pair[0].clone(), pair[1].clone());

            let mut interleaving_paths = Vec::new();

            //We search for a path between t1 and t2 with exactly 3 intermediates nodes
            let simple_paths = all_simple_paths::<Vec<_>, _>(
                &petri.graph,
                petri.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(),
                petri.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(),
                3,
                Some(3),
            ).collect::<Vec<_>>();

            if simple_paths.len()<2{
                continue;
            }


            for path in simple_paths.iter(){

                let path_as_nodes = path.iter().map(|x| petri.graph[*x].clone()).collect::<Vec<_>>();

                if petri.is_pure_path(&path_as_nodes){
                    //println!("Interleaving path: {:?}",path_as_nodes);
                    interleaving_paths.push(path_as_nodes.clone());
                }

                //We return a result if we fine two paths
                if interleaving_paths.len()==2{

                    let frontier_t1 = InterleavingFrontier(t1.clone());
                    let frontier_t2 = InterleavingFrontier(t2.clone());

                    let redex_t1 = interleaving_paths[0][1..interleaving_paths[0].len()-1].to_vec();
                    let redex_t2 = interleaving_paths[1][1..interleaving_paths[1].len()-1].to_vec();

                    return Some(InterleavingPattern::new((frontier_t1,frontier_t2),(redex_t1,redex_t2)));
                }
            }
        }

        None

         */

        ///////NEW
        let mut patterns = Vec::new();

       'global_loop: for (t1,t2) in petri.transitions.iter().tuple_combinations(){

            let P_in1 = petri.preset_transition(t1).clone();
            let P_out1 = petri.postset_transition(t1).clone();

            let P_in2 = petri.preset_transition(t2).clone();
            let P_out2 = petri.postset_transition(t2).clone();

           //We check that: P_in1 and P_in2 are disjoint;  P_out1 and P_out2 are disjoint
            if (P_in1.intersection(&P_in2).cloned().collect::<Vec<_>>().len()!=0)|| (P_out1.intersection(&P_out2).cloned().collect::<Vec<_>>().len()!=0) {
                continue;
            }

           /////////We check that each place in P_in1 and P_in2 have the same preset
            let p_in_witness = P_in1.clone().first().unwrap().clone();
            let preset_witness = petri.preset_generic(&PetriNode::Place(p_in_witness.clone()));
            for p in P_in1.iter().chain(P_in2.iter()){
                let preset_p = petri.preset_generic(&PetriNode::Place(p.clone()));
                if preset_p!=preset_witness{
                    continue 'global_loop;
                }
            }

           ///////We check that each place in P_out1 and P_out2 have the same postset
            let p_out_witness = P_out1.clone().first().unwrap().clone();
            let postset_witness = petri.postset_generic(&PetriNode::Place(p_out_witness.clone()));
            for p in P_out1.iter().chain(P_out2.iter()){
                let postset_p = petri.postset_generic(&PetriNode::Place(p.clone()));
                if postset_p!=postset_witness{
                    continue 'global_loop;
                }
            }

           //Ok Pattern
           let frontier_in = InterleavingFrontier{ subset1: P_in1.clone(), subset2: P_in2.clone() };
           let frontier_out = InterleavingFrontier{ subset1: P_out1.clone(), subset2: P_out2.clone() };
           let int_pattern = InterleavingPattern::new((frontier_in,frontier_out),(t1.clone(),t2.clone()));
           patterns.push(int_pattern);
        }

        patterns
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &InterleavingPattern){
        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.0.clone()));
        petri.remove_node(&PetriNode::Transition(pattern.redex.1.clone()));

        //Creation
        let regex = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&regex);
        petri.add_transition(&new_transition);

        for p in pattern.frontier.0.subset1.iter().chain(pattern.frontier.0.subset2.iter()){
            petri.update_edge(&PetriNode::Place(p.clone()), &PetriNode::Transition(new_transition.clone()));
        }

        for p in pattern.frontier.1.subset1.iter().chain(pattern.frontier.1.subset2.iter()){
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(p.clone()));
        }
    }

    pub fn get_synthetic_regex(&self)->Regex<String>{
        let mut  regex = Regex::Empty;

        let r1 = self.redex.0.label.clone();

        let r2 = self.redex.1.label.clone();

        regex = Regex::interleave(&r1,&r2);

        regex
    }
}