use itertools::Itertools;
use petgraph::Direction;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::kleene2::Kleene2Pattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_kleene_preparation(petri: &PetriNet<Regex<String>>) ->bool{
    if let Some(_) = KleenePreparationPattern::search_pattern(petri){
        println!("Kleene Preparation found");
        return true;
    }

    false
}


pub fn apply_kleene_preparation(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    if let Some(kleene_prep_pattern) = KleenePreparationPattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        KleenePreparationPattern::replace_pattern(&mut new_petri_net,&kleene_prep_pattern);
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for kleenePreparation, while the precondition is satisfied.");
}


pub struct KleenePreparationPattern{
    pub frontier: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>),
    pub redex: PetriPlace
}

impl KleenePreparationPattern{
    pub fn new(frontier: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>), redex: PetriPlace) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<KleenePreparationPattern>{

        for pair in petri.transitions.iter().permutations(2) {
            let (t1, t2) = (pair[0].clone(), pair[1].clone());

            let t1_index = petri.get_node_index(&PetriNode::Transition(t1.clone())).unwrap();
            let t2_index = petri.get_node_index(&PetriNode::Transition(t2.clone())).unwrap();

            let out_nbr_t1 = petri.graph.neighbors_directed(t1_index,Direction::Outgoing).collect_vec();


            let out_nbr_t2 = petri.graph.neighbors_directed(t2_index,Direction::Outgoing).collect_vec();
            let inc_nfr_t2 = petri.graph.neighbors_directed(t2_index,Direction::Incoming).collect_vec();

            for n1 in out_nbr_t1.iter(){
                for n2 in out_nbr_t2.iter(){
                    for n3 in inc_nfr_t2.iter(){
                        if n1==n2 && n2==n3{

                            let node = petri.graph.node_weight(*n1).unwrap();
                            if let PetriNode::Place(p) = node{

                                if petri.are_all_neighbor_in(p,&vec![t1.clone(),t2.clone()]){
                                    return Some(Self::new((t1,t2),p.clone()));
                                }

                            }

                        }

                    }
                }
            }
            /*
            for p in petri.places.iter(){

                if petri.neighboring_transitions(&p)==vec![t1.clone(),t2.clone()] || petri.neighboring_transitions(&p)==vec![t2.clone(),t1.clone()] {
                    let t1_index = petri.get_node_index(&PetriNode::Transition(t1.clone())).unwrap();
                    let t2_index = petri.get_node_index(&PetriNode::Transition(t2.clone())).unwrap();

                    let p_index = petri.get_node_index(&PetriNode::Place(p.clone())).unwrap();

                    let out_nbr_t1 = petri.graph.neighbors_directed(t1_index,Direction::Outgoing).collect_vec();


                    let out_nbr_t2 = petri.graph.neighbors_directed(t2_index,Direction::Outgoing).collect_vec();
                    let inc_nfr_t2 = petri.graph.neighbors_directed(t2_index,Direction::Incoming).collect_vec();

                    //We check that p is an outgoing neighbor of t1
                    if !out_nbr_t1.contains(&p_index){
                        continue;
                    }

                    //We check that t2 loops to p
                    if !out_nbr_t2.contains(&p_index) || !inc_nfr_t2.contains(&t1_index){
                        continue;
                    }

                    return Some(Self::new((t1,t2),p.clone()));
                }

            }*/
        }


        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &KleenePreparationPattern){

        let new_transition = PetriTransition::new_from_name(&Regex::Epsilon);
        petri.add_transition(&new_transition);
        petri.update_edge(&PetriNode::Place(pattern.redex.clone()), &PetriNode::Transition(new_transition.clone()));

        let new_place = PetriPlace::new_from_nothing();
        petri.add_place(&new_place);
        petri.update_edge(&PetriNode::Transition(new_transition),&PetriNode::Place(new_place.clone()));
    }
}













