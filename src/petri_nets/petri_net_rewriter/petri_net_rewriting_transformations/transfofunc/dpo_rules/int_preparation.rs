use itertools::Itertools;
use petgraph::Direction;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alt_preparation::AlternativePreparationPattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_interleaving_preparation(petri: &PetriNet<Regex<String>>) -> bool {

    if let Some(_) = InterleavingPreparationPattern::search_pattern(petri){
        println!("Interleaving preparation found");
        return true;
    }

    false
}


pub fn apply_interleaving_preparation(petri: &PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
    if let Some(int_prep_pattern) = InterleavingPreparationPattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        InterleavingPreparationPattern::replace_pattern(&mut new_petri_net,&int_prep_pattern);
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for interleaving preparation, while the precondition is satisfied.");
}


pub struct InterleavingPreparationFrontier{
    pub p_above: PetriPlace,
    pub p_below: PetriPlace,
    pub t_left: PetriTransition<Regex<String>>,
    pub t_above: PetriTransition<Regex<String>>,
    pub t_below: PetriTransition<Regex<String>>,
    pub t_right: PetriTransition<Regex<String>>,
}
impl InterleavingPreparationFrontier{
    pub fn new(p_above: PetriPlace, p_below: PetriPlace, t_left: PetriTransition<Regex<String>>, t_above: PetriTransition<Regex<String>>, t_below: PetriTransition<Regex<String>>, t_right: PetriTransition<Regex<String>>) -> Self{
        Self{ p_above, p_below, t_left, t_above, t_below, t_right }
    }
}
pub struct InterleavingPreparationPattern{
    pub frontier: InterleavingPreparationFrontier,
    pub redex: PetriPlace
}

impl InterleavingPreparationPattern{
    pub fn new(frontier: InterleavingPreparationFrontier, redex: PetriPlace) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<InterleavingPreparationPattern>{
        if petri.transitions.len()<4{
            return None;
        }

        for q in petri.transitions.iter().permutations(4) {
            let (t_left, t_above, t_below, t_right) = (q[0].clone(), q[1].clone(), q[2].clone(), q[3].clone());


            if let (Some(p_above),Some(p_below)) = (petri.pure_path_len_1_transitions(&t_left,&t_above),petri.pure_path_len_1_transitions(&t_left,&t_below)){

                let places_between_t_above_and_t_right = petri.places_between(&t_above,&t_right);
                let places_between_t_below_and_t_right = petri.places_between(&t_below,&t_right);
                let mut common_place = None;
                for p1 in places_between_t_above_and_t_right.iter(){
                    for p2 in places_between_t_below_and_t_right.iter(){
                        if *p1==*p2{
                            common_place = Some(p1.clone());
                        }
                    }
                }
                if common_place.is_none(){
                    continue;
                }

                let frontier = InterleavingPreparationFrontier::new(p_above.clone(),p_below.clone(),t_left.clone(),t_above.clone(),t_below.clone(),t_right.clone());
                return Some(InterleavingPreparationPattern::new(frontier,common_place.unwrap()));
            }
        }



        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &InterleavingPreparationPattern){
        //Deletion
        petri.remove_node(&PetriNode::Place(pattern.redex.clone()));

        //Creation
        let new_p1 = PetriPlace::new_from_nothing();
        let new_p2 = PetriPlace::new_from_nothing();
        petri.add_place(&new_p1);
        petri.add_place(&new_p2);
        petri.update_edge(&PetriNode::Transition(pattern.frontier.t_above.clone()),&PetriNode::Place(new_p1.clone()));
        petri.update_edge(&PetriNode::Transition(pattern.frontier.t_below.clone()),&PetriNode::Place(new_p1.clone()));

        petri.update_edge(&PetriNode::Place(new_p1.clone()),&PetriNode::Transition(pattern.frontier.t_right.clone()));
        petri.update_edge(&PetriNode::Place(new_p1.clone()),&PetriNode::Transition(pattern.frontier.t_right.clone()));

    }
}


impl PetriNet<Regex<String>>{
    pub fn places_between(&self,t1: &PetriTransition<Regex<String>>, t2: &PetriTransition<Regex<String>>) -> Vec<PetriPlace>{
        let outgoing_nbr_t1 = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(),Direction::Outgoing).collect::<Vec<_>>();
        let incoming_nbr_t2 = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(),Direction::Incoming).collect::<Vec<_>>();

        let mut places = vec![];
        for n1 in outgoing_nbr_t1.iter(){
            for n2 in incoming_nbr_t2.iter(){
                if *n1==*n2{
                    if let PetriNode::Place(p) = self.graph.node_weight(*n1).unwrap(){
                        places.push(p.clone());
                    }
                    else{
                        panic!("There is a transition between two transitions. Impossible.");
                    }
                }
            }
        }
        places
    }
}