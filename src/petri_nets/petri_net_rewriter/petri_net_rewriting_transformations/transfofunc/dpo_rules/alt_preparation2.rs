use itertools::Itertools;
use petgraph::Direction;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alt_preparation::{AlternativePreparationFrontier, AlternativePreparationPattern};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_alternative_preparation2(petri: &PetriNet<Regex<String>>) ->bool{
    if let Some(_) = AlternativePreparation2Pattern::search_pattern(petri){
        println!("Alternative Preparation 2 found");
        return true;
    }

    false
}


pub fn apply_alternative_preparation2(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
    if let Some(alt_prep2_pattern) = AlternativePreparation2Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        AlternativePreparation2Pattern::replace_pattern(&mut new_petri_net,&alt_prep2_pattern);
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for alternative preparation 2, while the precondition is satisfied.");
}

pub struct AlternativePreparation2Frontier{
    pub p: PetriPlace,
    pub t_above: PetriTransition<Regex<String>>,
    pub t_below: PetriTransition<Regex<String>>,
}

impl AlternativePreparation2Frontier{
    pub fn new(p: PetriPlace, t_above: PetriTransition<Regex<String>>, t_below: PetriTransition<Regex<String>>) -> Self{
        Self{ p, t_above, t_below }
    }
}
pub struct AlternativePreparation2Pattern{
    pub frontier: AlternativePreparation2Frontier,
    pub redex: (PetriPlace,PetriPlace)
}

impl AlternativePreparation2Pattern{
    pub fn new(frontier: AlternativePreparation2Frontier, redex: (PetriPlace,PetriPlace)) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<AlternativePreparation2Pattern>{
        for pair in petri.transitions.iter().permutations(2) {
            let (t_above, t_below) = (pair[0].clone(), pair[1].clone());

            //Is there a common predecessor???
            let incoming_nbr_t_above = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t_above.clone())).unwrap(), Direction::Incoming).collect::<Vec<_>>();
            let incoming_nbr_t_below = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t_below.clone())).unwrap(), Direction::Incoming).collect::<Vec<_>>();
            let mut common_predecessor = None;
            for n1 in incoming_nbr_t_above.iter(){
                for n2 in incoming_nbr_t_below.iter(){
                    if *n1==*n2{
                        common_predecessor = Some(n1.clone());
                    }
                }
            }
            if common_predecessor.is_none(){
                continue;
            }
            let common_predecessor = common_predecessor.unwrap();
            let frontier_p = if let PetriNode::Place(p) = petri.graph.node_weight(common_predecessor).unwrap().clone(){
                p.clone()
            }
            else{
                panic!("We have not found a place in the common predecessor of the two transitions.");
            };

            //Search for the pair of places
            let outgoing_nbr_t_above = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t_above.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();
            let outgoing_nbr_t_below = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t_below.clone())).unwrap(), Direction::Outgoing).collect::<Vec<_>>();
            let mut p1 = None;
            let mut p2 = None;

            for n in outgoing_nbr_t_above.iter(){
                //If n has no outgoing edge, then it is p1
                let outgoing_nbr_n = petri.graph.neighbors_directed(*n, Direction::Outgoing).collect::<Vec<_>>();
                if outgoing_nbr_n.len()==0{
                    let node = petri.graph.node_weight(*n).unwrap();
                    if let PetriNode::Place(p) = node{
                        p1 = Some(p.clone());
                    }
                    else{
                        panic!("We have found a transition which is a successor to a transition. Impossible.");
                    }
                }
            }

            for n in outgoing_nbr_t_below.iter(){
                //If n has no outgoing edge, then it is p2
                let outgoing_nbr_n = petri.graph.neighbors_directed(*n, Direction::Outgoing).collect::<Vec<_>>();
                if outgoing_nbr_n.len()==0{
                    let node = petri.graph.node_weight(*n).unwrap();
                    if let PetriNode::Place(p) = node{
                        p2 = Some(p.clone());
                    }
                    else{
                        panic!("We have found a transition which is a successor to a transition. Impossible.");
                    }
                }
            }

            //Wrapping up
            if p1.is_none() || p2.is_none(){
                continue;
            }


            let p1 = p1.unwrap();
            let p2 = p2.unwrap();

            //Little gluing condition check
            if !petri.are_all_neighbor_in(&p1,&vec![t_above.clone()]) || !petri.are_all_neighbor_in(&p2,&vec![t_below.clone()]){
                continue;
            }

            //Pattern found
            let frontier = AlternativePreparation2Frontier::new(frontier_p.clone(), t_above.clone(), t_below.clone());
            return Some(AlternativePreparation2Pattern::new(frontier,(p1,p2)));

        }

        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &AlternativePreparation2Pattern){
        //Deletion
        petri.remove_node(&PetriNode::Place(pattern.redex.0.clone()));
        petri.remove_node(&PetriNode::Place(pattern.redex.1.clone()));

        //Creation
        let new_place = PetriPlace::new_from_nothing();
        petri.add_place(&new_place);
        petri.update_edge(&PetriNode::Transition(pattern.frontier.t_above.clone()),&PetriNode::Place(new_place.clone()));
        petri.update_edge(&PetriNode::Transition(pattern.frontier.t_below.clone()),&PetriNode::Place(new_place.clone()));
    }
}





































