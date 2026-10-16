use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::kleene1::{Kleene1Pattern, Kleene1Redex};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_kleene2(petri: &PetriNet<Regex<String>>) -> bool {
    if let Some(_) = Kleene2Pattern::search_pattern(petri){
        println!("Kleene2 found");
        return true;
    }

    false
}


pub fn apply_kleene2(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    if let Some(kleene2_pattern) = Kleene2Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        Kleene2Pattern::replace_pattern(&mut new_petri_net,&kleene2_pattern);
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for kleene2, while the precondition is satisfied.");
}


pub struct Kleene2Redex{
    pub places: (PetriPlace,PetriPlace),
    pub r1: Option<PetriTransition<Regex<String>>>,
    pub r2: PetriTransition<Regex<String>>,
    pub r3: Option<PetriTransition<Regex<String>>>,
    pub r4: Option<PetriTransition<Regex<String>>>,
}
pub struct Kleene2Pattern{
    pub frontier: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>),
    pub redex: Kleene2Redex
}

impl Kleene2Pattern{
    pub fn new(frontier: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>), redex: Kleene2Redex) -> Self{
        Self{ frontier, redex }
    }


    //TO DO: check whether p1 and p2 do not have dangling links
    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<Kleene2Pattern>{
        for pair in petri.transitions.iter().permutations(2) {
            let (t1, t2) = (pair[0].clone(), pair[1].clone());

            let t1_successors = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(),petgraph::Direction::Outgoing).collect::<Vec<_>>();
            let t2_predecessors = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();

            for t1_succ in t1_successors.iter(){
                for t2_pred in t2_predecessors.iter(){
                    let p1 = if let PetriNode::Place(p) = petri.graph[t1_succ.clone()].clone(){
                        p
                    }
                    else{
                        panic!("We have found a transition which successor is not a place. Error.");
                    };

                    let p2 = if let PetriNode::Place(p) =  petri.graph[t2_pred.clone()].clone(){
                        p
                    } else{
                        panic!("We have found a transition which predecessor is not a place. Error.");
                    };

                    //We check that p1 and p2 are different
                    if p1==p2{
                        continue;
                    }

                    //The following vector is to make sure that no other external transition is linked to p1 and p2.
                    let mut valid_neighors = Vec::from([t1.clone(),t2.clone()]);

                    //r1 search
                    let r1 = petri.pure_path_len_1_places(&p1, &p1);

                    if let Some(t) = r1.clone(){
                        if t==t1.clone() || t==t2.clone(){
                            continue;
                        }
                        valid_neighors.push(t.clone());
                    }

                    //r2 search; mandatory
                    let r2 = if let Some(t) = petri.pure_path_len_1_places(&p1, &p2){
                        if t==t1.clone() || t==t2.clone(){
                            continue;
                        }
                        valid_neighors.push(t.clone());
                        t
                    }
                    else{
                       continue;
                    };

                    //r3 search
                    let r3 = petri.pure_path_len_1_places(&p2, &p1);

                    if let Some(t) = r3.clone(){
                        if t==t1.clone() || t==t2.clone(){
                            continue;
                        }
                        valid_neighors.push(t.clone());
                    }
                    //r4 search
                    let r4 = petri.pure_path_len_1_places(&p2, &p2);
                    if let Some(t) = r4.clone(){
                        if t==t1.clone() || t==t2.clone(){
                            continue;
                        }
                        valid_neighors.push(t.clone());
                    }

                    //We check that p1 and p2 have the right neighboring transitions
                    if !petri.are_all_neighbor_in(&p1,&valid_neighors) || !petri.are_all_neighbor_in(&p2,&valid_neighors){
                        continue;
                    }
                    //We have to make sure that at least one of r1,r3,r4 is not None
                    if r1.is_some() || r3.is_some() || r4.is_some(){
                        return Some(Self::new((t1.clone(),t2.clone()),Kleene2Redex{places:(p1,p2),r1,r2,r3,r4}));
                    }
                }
            }
        }

        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &Kleene2Pattern){
        //////////////////////////
        //Deletion
        /////////////////////////
        petri.remove_node(&PetriNode::Place(pattern.redex.places.0.clone()));
        petri.remove_node(&PetriNode::Place(pattern.redex.places.1.clone()));

        if let Some(r1) = pattern.redex.r1.clone(){
            petri.remove_node(&PetriNode::Transition(r1.clone()));
            println!("r1 {:?}",r1);
        }
        petri.remove_node(&PetriNode::Transition(pattern.redex.r2.clone()));
        println!("r2 {:?}",pattern.redex.r2);

        if let Some(r3) = pattern.redex.r3.clone(){
            petri.remove_node(&PetriNode::Transition(r3.clone()));
            println!("r3 {:?}",r3);
        }
        if let Some(r4) = pattern.redex.r4.clone(){
            petri.remove_node(&PetriNode::Transition(r4.clone()));
            println!("r4 {:?}",r4);
        }
        /////////////////////////
        //Creation
        ////////////////////////
        println!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTt {:?} --- {:?}",pattern.frontier.0,pattern.frontier.1);
        let p1 = PetriPlace::new_from_nothing();
        let p2 = PetriPlace::new_from_nothing();
        petri.add_place(&p1);
        petri.add_place(&p2);
        petri.update_edge(&PetriNode::Transition(pattern.frontier.0.clone()), &PetriNode::Place(p1.clone()));
        petri.update_edge(&PetriNode::Place(p2.clone()), &PetriNode::Transition(pattern.frontier.1.clone()));



        //regex
        let regex = match (pattern.redex.r1.clone(),pattern.redex.r3.clone(),pattern.redex.r4.clone()){
            (Some(trans1),Some(trans3),Some(trans4)) => {
                let r1_star = Regex::kleene(&trans1.label);
                let r2 = pattern.redex.r2.label.clone();
                let r3 = trans3.label.clone();
                let r4 = trans4.label.clone();

                let or_bloc = Regex::or(&r4,&Regex::concatenate_vec(vec![r3,r1_star.clone(),r2.clone()]));
                Regex::concatenate_vec(vec![
                    r1_star,
                    r2.clone(),
                   Regex::kleene(&or_bloc)
                ])
            },
            (Some(trans1),Some(trans3),None)=>{
                let r1_star = Regex::kleene(&trans1.label);
                let r2 = pattern.redex.r2.label.clone();
                let r3 = trans3.label.clone();

                let or_bloc = Regex::concatenate_vec(vec![r3,r1_star.clone(),r2.clone()]);
                Regex::concatenate_vec(vec![
                    r1_star,
                    r2.clone(),
                    Regex::kleene(&or_bloc)
                ])
            },
            (Some(trans1),None,Some(trans4))=>{
                let r1_star = Regex::kleene(&trans1.label);
                let r2 = pattern.redex.r2.label.clone();
                let r4 = trans4.label.clone();

                let or_bloc = r4;
                Regex::concatenate_vec(vec![
                    r1_star,
                    r2.clone(),
                    Regex::kleene(&or_bloc)
                ])
            },
            (None,Some(trans3),Some(trans4))=>{

                let r2 = pattern.redex.r2.label.clone();
                let r3 = trans3.label.clone();
                let r4 = trans4.label.clone();

                let or_bloc = Regex::or(&r4,&Regex::concatenate_vec(vec![r3,r2.clone()]));
                Regex::concatenate_vec(vec![
                    r2.clone(),
                    Regex::kleene(&or_bloc)
                ])
            },
            (None,None,Some(trans4))=>{
                let r2 = pattern.redex.r2.label.clone();
                let r4 = trans4.label.clone();

                let or_bloc = r4;
                Regex::concatenate_vec(vec![
                    r2.clone(),
                    Regex::kleene(&or_bloc)
                ])
            },
            (Some(trans1),None,None)=>{
                let r1_star = Regex::kleene(&trans1.label);
                let r2 = pattern.redex.r2.label.clone();

                Regex::concatenate_vec(vec![
                    r1_star,
                    r2.clone(),
                ])
            },
            (None,Some(trans3),None)=>{
                let r2 = pattern.redex.r2.label.clone();
                let r3 = trans3.label.clone();

                let or_bloc = Regex::concatenate_vec(vec![r3,r2.clone()]);
                Regex::concatenate_vec(vec![
                    r2.clone(),
                    Regex::kleene(&or_bloc)
                ])
            }
            (None,None,None)=>{
                panic!("Invalid rule application; the patterns r1, r3 and r4 are all None. Error.")
            }
        };

        let new_transition = PetriTransition::new_from_name(&regex);
        petri.add_transition(&new_transition);
        petri.update_edge(&PetriNode::Place(p1.clone()), &PetriNode::Transition(new_transition.clone()));
        petri.update_edge(&PetriNode::Transition(new_transition.clone()), &PetriNode::Place(p2.clone()));

    }
}