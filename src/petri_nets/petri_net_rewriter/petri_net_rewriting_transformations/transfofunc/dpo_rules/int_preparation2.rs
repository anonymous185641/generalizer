use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::Direction;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alt_preparation2::{AlternativePreparation2Frontier, AlternativePreparation2Pattern};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::interleaving::{InterleavingFrontier, InterleavingPattern};
use crate::regex_inf::regex::regex::Regex;



pub fn can_apply_interleaving_preparation2(petri: &PetriNet<Regex<String>>) ->bool{
    if let Some(_) = InterleavingPreparation2Pattern::search_pattern(petri){
        println!("Interleaving Preparation 2 found");
        return true;
    }

    false
}


pub fn apply_interleaving_preparation2(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
    if let Some(int_prep2_pattern) = InterleavingPreparation2Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        InterleavingPreparation2Pattern::replace_pattern(&mut new_petri_net,&int_prep2_pattern);
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for interleaving preparation 2, while the precondition is satisfied.");
}



pub struct InterleavingPreparation2Pattern{
    pub p_left: PetriPlace ,
    pub p_right: PetriPlace,
    pub p1_above: PetriPlace,
    pub p1_below: PetriPlace,
    pub p2_above: PetriPlace,
    pub p2_below: PetriPlace,
    pub transitions: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>),
    pub t1_above: Option<PetriTransition<Regex<String>>>,
    pub t1_below: Option<PetriTransition<Regex<String>>>,
    pub t2_above: Option<PetriTransition<Regex<String>>>,
    pub t2_below: Option<PetriTransition<Regex<String>>>,
}



impl InterleavingPreparation2Pattern{

    pub fn new(p_left: PetriPlace,p_right: PetriPlace,p1_above: PetriPlace,p1_below: PetriPlace,p2_above: PetriPlace,p2_below: PetriPlace,
    transitions: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>),t1_above: Option<PetriTransition<Regex<String>>>,
    t1_below: Option<PetriTransition<Regex<String>>>, t2_above: Option<PetriTransition<Regex<String>>>, t2_below: Option<PetriTransition<Regex<String>>>)->Self{
        Self{ p_left,p_right,p1_above,p1_below,p2_above,p2_below,transitions,t1_above,t1_below,t2_above,t2_below}
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>) -> Option<InterleavingPreparation2Pattern>{

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


            for path_pair in simple_paths.iter().combinations(2){

                let path1 = path_pair[0].clone();
                let path2 = path_pair[1].clone();
                let path1_as_nodes = path1.iter().map(|x| petri.graph[*x].clone()).collect::<Vec<_>>();
                let path2_as_nodes = path2.iter().map(|x| petri.graph[*x].clone()).collect::<Vec<_>>();
                interleaving_paths.push(path1_as_nodes.clone());
                interleaving_paths.push(path2_as_nodes.clone());

                //We return a result if we fine two paths

                //Little check
                let outgoing_nbr_t1 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();
                let incoming_nbr_t2 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(), Direction::Incoming).collect::<Vec<_>>();

                if outgoing_nbr_t1.len() < 2 || incoming_nbr_t2.len() < 2{
                    continue;
                }

                //We search for p1 and p2
                let incoming_nbr_t1 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(), Direction::Incoming).collect::<Vec<_>>();
                let outgoing_nbr_t2 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();

                for nbr1 in incoming_nbr_t1.iter(){
                    for nbr2 in outgoing_nbr_t2.iter(){
                        let p1 = petri.graph[*nbr1].clone().unfold_place().unwrap();
                        let p2 = petri.graph[*nbr2].clone().unfold_place().unwrap();



                        //Search for other transitions
                        let p1_above = interleaving_paths[0][1].clone().unfold_place().unwrap();
                        let p2_above = interleaving_paths[0][3].clone().unfold_place().unwrap();

                        let p1_below = interleaving_paths[1][1].clone().unfold_place().unwrap();
                        let p2_below = interleaving_paths[1][3].clone().unfold_place().unwrap();

                        //Search for t1_above
                        let mut t1_above = None;
                        let p1_above_outgoing_nbr = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p1_above.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();

                        if p1_above_outgoing_nbr.len()>2{
                            continue;
                        }

                        for nbr in p1_above_outgoing_nbr {
                            let node = petri.graph.node_weight(nbr).unwrap().unfold_transition().unwrap();
                            if node!=interleaving_paths[0][2].clone().unfold_transition().unwrap(){
                                t1_above = Some(node);
                            }
                        }


                        //Search for t1_below
                        let mut t1_below = None;
                        let p1_below_outgoing_nbr = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p1_below.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();

                        if p1_below_outgoing_nbr.len()>2{
                            continue;
                        }
                        for nbr in p1_below_outgoing_nbr {
                            let node = petri.graph.node_weight(nbr).unwrap().unfold_transition().unwrap();
                            if node!=interleaving_paths[1][2].clone().unfold_transition().unwrap(){
                                t1_below = Some(node);
                            }
                        }


                        //Search for t2_above
                        let mut t2_above = None;
                        let p2_above_outgoing_nbr = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p2_above.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();

                        if p2_above_outgoing_nbr.len()>2{
                            continue;
                        }
                        for nbr in p2_above_outgoing_nbr {
                            let node = petri.graph.node_weight(nbr).unwrap().unfold_transition().unwrap();
                            if node!=interleaving_paths[0][4].clone().unfold_transition().unwrap(){
                                t2_above = Some(node);
                            }
                        }
                        //Search for t2_below
                        let mut t2_below = None;
                        let p2_below_outgoing_nbr = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p2_below.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();

                        if p2_below_outgoing_nbr.len()>2{
                            continue;
                        }
                        for nbr in p2_below_outgoing_nbr {
                            let node = petri.graph.node_weight(nbr).unwrap().unfold_transition().unwrap();
                            if node!=interleaving_paths[0][4].clone().unfold_transition().unwrap(){
                                t2_below = Some(node);
                            }
                        }



                        if t1_above.is_none() && t1_below.is_none() && t2_above.is_none() && t2_below.is_none(){
                            continue;
                        }

                        if p1_above == p1 || p1_below == p1 || p2_above == p2 || p2_below == p2{
                            continue;
                        }
                        println!("Found Interleaving Preparation 2: p1: {:?} --- p2: {:?}",p1,p2);
                        println!("t1: {:?} --- t2: {:?}",t1,t2);
                        println!("t1_above: {:?} --- t1_below: {:?}",t1_above,t1_below);
                        println!("t2_above: {:?} --- t2_below: {:?}",t2_above,t2_below);
                        println!("p1_above: {:?} --- p1_below: {:?}",p1_above,p1_below);
                        println!("p2_above: {:?} --- p2_below: {:?}",p2_above,p2_below);
                        println!("----------------------------------------\n");
                        return Some(InterleavingPreparation2Pattern::new(p1, p2, p1_above, p1_below, p2_above, p2_below, (t1, t2), t1_above, t1_below, t2_above, t2_below))




                    }
                }



            }
        }

        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &InterleavingPreparation2Pattern){

        if let Some(t1_above) = pattern.t1_above.clone() {
            petri.remove_edge(&PetriNode::Place(pattern.p1_above.clone()),&PetriNode::Transition(t1_above.clone()));
            petri.update_edge(&PetriNode::Place(pattern.p_left.clone()),&PetriNode::Transition(t1_above.clone()));
        }

        if let Some(t1_below) = pattern.t1_below.clone() {
            petri.remove_edge(&PetriNode::Place(pattern.p1_below.clone()),&PetriNode::Transition(t1_below.clone()));
            petri.update_edge(&PetriNode::Place(pattern.p_left.clone()),&PetriNode::Transition(t1_below.clone()));
        }

        if let Some(t2_above) = pattern.t2_above.clone() {
            petri.remove_edge(&PetriNode::Place(pattern.p2_above.clone()),&PetriNode::Transition(t2_above.clone()));
            petri.update_edge(&PetriNode::Place(pattern.p_right.clone()),&PetriNode::Transition(t2_above.clone()));
        }

        if let Some(t2_below) = pattern.t2_below.clone() {
            petri.remove_edge(&PetriNode::Place(pattern.p2_below.clone()),&PetriNode::Transition(t2_below.clone()));
            petri.update_edge(&PetriNode::Place(pattern.p_right.clone()),&PetriNode::Transition(t2_below.clone()));
        }
    }
}

impl PetriNode<Regex<String>>{
    pub fn unfold_place(&self)->Option<PetriPlace>{
        if let PetriNode::Place(p) = self{
            return Some(p.clone());
        }
        None
    }

    pub fn unfold_transition(&self)->Option<PetriTransition<Regex<String>>>{
        if let PetriNode::Transition(t) = self{
            return Some(t.clone());
        }
        None
    }
}