use itertools::Itertools;
use petgraph::Direction;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_alternative_preparation(petri: &PetriNet<Regex<String>>) -> bool {
    if let Some(_) = AlternativePreparationPattern::search_pattern(petri){
        println!("Alternative Preparation found");
        return true;
    }

    false
}


pub fn apply_alternative_preparation(petri: &PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
    if let Some(alt_prep_pattern) = AlternativePreparationPattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        AlternativePreparationPattern::replace_pattern(&mut new_petri_net,&alt_prep_pattern);
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for alternative preparation, while the precondition is satisfied.");
}

pub struct AlternativePreparationFrontier{
    pub p: PetriPlace,
    pub t_above: PetriTransition<Regex<String>>,
    pub t_below: PetriTransition<Regex<String>>,
    pub t_right: PetriTransition<Regex<String>>,
}
impl AlternativePreparationFrontier{
    pub fn new(p: PetriPlace, t_above: PetriTransition<Regex<String>>, t_below: PetriTransition<Regex<String>>, t_right: PetriTransition<Regex<String>>) -> Self{
        Self{ p, t_above, t_below, t_right }
    }
}

pub struct AlternativePreparationPattern{
    pub frontier: AlternativePreparationFrontier,
    pub redex: (PetriPlace,PetriPlace)
}


impl AlternativePreparationPattern{
    pub fn new(frontier: AlternativePreparationFrontier, redex: (PetriPlace,PetriPlace)) -> Self{
        Self{ frontier, redex }
    }
    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<AlternativePreparationPattern>{

        for pair in petri.transitions.iter().permutations(2) {
            let (t_above, t_below) = (pair[0].clone(), pair[1].clone());

            //Is there a common predecessor???
            let incoming_nbr_t1_above = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t_above.clone())).unwrap(), Direction::Incoming).collect::<Vec<_>>();
            let incoming_nbr_t_below = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t_below.clone())).unwrap(), Direction::Incoming).collect::<Vec<_>>();
            let mut common_predecessor = None;
            for n1 in incoming_nbr_t1_above.iter(){
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

            //Search for t_right
            for t_right in petri.transitions.iter(){
                if t_right ==&t_above || t_right ==&t_below{
                    continue;
                }

                if let (Some(p1),Some(p2)) = (petri.pure_path_len_1_transitions(&t_above,&t_right), petri.pure_path_len_1_transitions(&t_below, &t_right)){

                    //Pattern found
                    let frontier = AlternativePreparationFrontier::new(frontier_p.clone(), t_above.clone(), t_below.clone(), t_right.clone());
                    return Some(AlternativePreparationPattern::new(frontier,(p1,p2)));
                }
            }


        }

        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &AlternativePreparationPattern){
        //Deletion
        petri.remove_node(&PetriNode::Place(pattern.redex.0.clone()));
        petri.remove_node(&PetriNode::Place(pattern.redex.1.clone()));

        //Creation
        let new_place = PetriPlace::new_from_nothing();
        petri.add_place(&new_place);
        petri.update_edge(&PetriNode::Transition(pattern.frontier.t_above.clone()),&PetriNode::Place(new_place.clone()));
        petri.update_edge(&PetriNode::Transition(pattern.frontier.t_below.clone()),&PetriNode::Place(new_place.clone()));
        petri.update_edge(&PetriNode::Place(new_place.clone()),&PetriNode::Transition(pattern.frontier.t_right.clone()));
    }
}


impl PetriNet<Regex<String>>{
    pub fn pure_path_len_1_transitions(&self, t1: &PetriTransition<Regex<String>>, t2:&PetriTransition<Regex<String>>) ->Option<PetriPlace>{
        let t1_index = self.get_node_index(&PetriNode::Transition(t1.clone())).unwrap();
        let t2_index = self.get_node_index(&PetriNode::Transition(t2.clone())).unwrap();

        let outgoing_t1_nbr = self.graph.neighbors_directed(t1_index,Direction::Outgoing).collect::<Vec<_>>();
        let outgoing_t2_nbr = self.graph.neighbors_directed(t2_index,Direction::Incoming).collect::<Vec<_>>();

        //Intersection
        for n1 in outgoing_t1_nbr.iter(){
            for n2 in outgoing_t2_nbr.iter(){
                if *n1==*n2{
                    //First, we make sure that the path is pure
                    if self.graph.neighbors_directed(*n1,Direction::Outgoing).count()!=1 || self.graph.neighbors_directed(*n1,Direction::Incoming).count()!=1{
                        continue;
                    }

                    let node = self.graph.node_weight(*n1).unwrap();
                    if let PetriNode::Place(p) = node{
                        return Some(p.clone());
                    }
                    else{
                        panic!("We there is place between two places, but not a transition. Impossible.");
                    }
                }
            }
        }

        None
    }


}














