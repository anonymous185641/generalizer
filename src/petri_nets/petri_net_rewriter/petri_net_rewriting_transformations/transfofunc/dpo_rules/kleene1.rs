use std::collections::{BTreeSet, HashSet};
use itertools::Itertools;
use petgraph::Direction;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alternative::AlternativePattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_kleene1(petri: &PetriNet<Regex<String>>) -> bool {


    if let Some(_) = Kleene1Pattern::search_pattern(petri){
        println!("Kleene1 found");
        return true;
    }

    false
}


pub fn apply_kleene1(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{

    if let Some(kleene1_pattern) = Kleene1Pattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        Kleene1Pattern::replace_pattern(&mut new_petri_net,&kleene1_pattern);
        //new_petri_net.draw(&"test".to_string(),&"temp1".to_string());
        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for kleene1, while the precondition is satisfied.");
}

#[derive(Debug,Clone)]
pub struct Kleene1Redex{
    pub central_place: PetriPlace,
    pub r1: PetriTransition<Regex<String>>,
    pub r2: Option<PetriTransition<Regex<String>>>,
    pub r3: PetriTransition<Regex<String>>,
    pub r4: Option<PetriTransition<Regex<String>>>,
    pub r5: Option<PetriTransition<Regex<String>>>,
}

#[derive(Debug,Clone)]
pub struct Kleene1Pattern{
    pub frontier: (PetriPlace,PetriPlace),
    pub redex: Kleene1Redex
}

impl Kleene1Pattern{
    pub fn new(frontier: (PetriPlace,PetriPlace), redex: Kleene1Redex) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<Kleene1Pattern>{

        for triple in petri.places.iter().permutations(3) {
            let (p1, p2,p3) = (triple[0].clone(), triple[1].clone(), triple[2].clone());

            //We check that p1 and p3 are the only neighboring places of p2
            if petri.neighboring_places(&p2)!=vec![p1.clone(),p3.clone()] && petri.neighboring_places(&p2)!=vec![p3.clone(),p1.clone()]{
                continue
            }

            let mut valid_neighbors = Vec::new();

            //r1 search
            let r1 = if let Some(t) = petri.pure_path_len_1_places(&p1, &p2){
                    valid_neighbors.push(t.clone());
                    t
            }
            else{
                continue;//r1 is mandatory, if not found, we can't apply the transformation
            };


            //r3 search
            let r3 = if let Some(t) = petri.pure_path_len_1_places(&p2, &p3){
                valid_neighbors.push(t.clone());
                t
            }
            else{
                continue;//r3 is mandatory, if not found, we can't apply the transformation
            };

            //Mandatory r5 search : I really dont know why, but I have to do this.
            /**/
            let r5 =  petri.pure_path_len_1_places(&p2, &p2);
            if let Some(t) = r5.clone(){
                valid_neighbors.push(t.clone());
            }
            else{
                continue;//Temporarily, I make r5 mandatory.
            }


            //Optional r2 search
            let r2 =  petri.pure_path_len_1_places(&p2, &p1);
            if let Some(t) = r2.clone(){
                valid_neighbors.push(t.clone());
            }

            //Optional r4 search
            let r4 =  petri.pure_path_len_1_places(&p3, &p2);
            if let Some(t) = r4.clone(){
                valid_neighbors.push(t.clone());
            }



            //We have to make sure that at least one of r5,r2,r4 is not None
            if r5.is_some() || r2.is_some() || r4.is_some(){

                //We check whether the neighbors of p2 are valid
                if !petri.are_all_neighbor_in(&p2,&valid_neighbors){
                    continue;
                }

                //Pattern found
                let kleene1 = Self::new((p1.clone(),p3.clone()),Kleene1Redex{central_place:p2,r1,r2,r3,r4,r5});
                println!("Kleene1 found:{:?}",kleene1);
                return Some(kleene1);
            }

        }

        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &Kleene1Pattern){


        //////////////////////////
        //Deletion
        /////////////////////////
        petri.remove_node(&PetriNode::Place(pattern.redex.central_place.clone()));
        petri.remove_node(&PetriNode::Transition(pattern.redex.r1.clone()));
        petri.remove_node(&PetriNode::Transition(pattern.redex.r3.clone()));

        if let Some(r2) = pattern.redex.r2.clone(){
            petri.remove_node(&PetriNode::Transition(r2.clone()));
        }
        if let Some(r5) = pattern.redex.r5.clone(){
            petri.remove_node(&PetriNode::Transition(r5.clone()));
        }
        if let Some(r4) = pattern.redex.r4.clone(){
            petri.remove_node(&PetriNode::Transition(r4.clone()));
        }


        /////////////////////////
        //Creation
        ////////////////////////
        //r1r5*r2, r2 is mandatory
        let r152 = match (pattern.redex.r2.clone(),pattern.redex.r5.clone()){
            (Some(r2),Some(r5)) => {
                let r5_star = Regex::kleene(&r5.label);
                let r1 = pattern.redex.r1.label.clone();
                let r2 = r2.label.clone();
                Some(Regex::concatenate_vec(vec![r1,r5_star,r2]))
            },
            (Some(r2),None) => {
                let r1 = pattern.redex.r1.label.clone();
                let r2 = r2.label.clone();
                Some(Regex::concatenate_vec(vec![r1,r2]))
            },
            (None,Some(r5)) => {
                //If r2 is absent, then there is no loop
                None
            },
            (None,None) => {
                //If r2 is absent, then there is no loop
              None
            },
        };
        if let Some(r) = r152{
            //LOOP with frontier p1, or frontier.0
            let new_transition = PetriTransition::new_from_name(&r);
            petri.add_transition(&new_transition);
            petri.update_edge(&PetriNode::Place(pattern.frontier.0.clone()), &PetriNode::Transition(new_transition.clone()));
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(pattern.frontier.0.clone()));
        }

        //r1r5*r3
        let r153 = match pattern.redex.r5.clone(){
            Some(r5)=>{
                let r5_star = Regex::kleene(&r5.label);
                let r1 = pattern.redex.r1.label.clone();
                let r3 = pattern.redex.r3.label.clone();
                Some(Regex::concatenate_vec(vec![r1,r5_star,r3]))
            },
            None =>{
                let r1 = pattern.redex.r1.label.clone();
                let r3 = pattern.redex.r3.label.clone();
                Some(Regex::concatenate_vec(vec![r1,r3]))
            }
        };

        if let Some(r) = r153{
            //p1 -> r153 -> p2
            let new_transition = PetriTransition::new_from_name(&r);
            petri.add_transition(&new_transition);
            petri.update_edge(&PetriNode::Place(pattern.frontier.0.clone()), &PetriNode::Transition(new_transition.clone()));
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(pattern.frontier.1.clone()));
        }

        //r4r5*r3 , r4 is mandatory
        let r453 = match (pattern.redex.r4.clone(),pattern.redex.r5.clone()){
            (Some(r4),Some(r5)) => {
                let r5_star = Regex::kleene(&r5.label);
                let r4 = r4.label.clone();
                let r3 = pattern.redex.r3.label.clone();
                Some(Regex::concatenate_vec(vec![r4,r5_star,r3]))
            },
            (Some(r4),None) => {
                let r4 = r4.label.clone();
                let r3 = pattern.redex.r3.label.clone();
                Some(Regex::concatenate_vec(vec![r4,r3]))
            },
            (None,_) => {
                None
            }
        };

        if let Some(r) = r453{
            //LOOP with frontier p1, or frontier.0
            let new_transition = PetriTransition::new_from_name(&r);
            petri.add_transition(&new_transition);
            petri.update_edge(&PetriNode::Place(pattern.frontier.1.clone()), &PetriNode::Transition(new_transition.clone()));
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(pattern.frontier.1.clone()));
        }

        //r4r5*r2, r2 and r4 are mandatory
        let r452 = match (pattern.redex.r2.clone(),pattern.redex.r4.clone(),pattern.redex.r5.clone()){
            (Some(r2),Some(r4),Some(r5)) => {
                let r5_star = Regex::kleene(&r5.label);
                let r2 = r2.label.clone();
                let r4 = r4.label.clone();
                Some(Regex::concatenate_vec(vec![r4,r5_star,r2]))
            },
            (Some(r2),Some(r4),None) => {
                let r2 = r2.label.clone();
                let r4 = r4.label.clone();
                Some(Regex::concatenate_vec(vec![r4,r2]))
            },
            _ => {
                None //Both r2 and r4 are mandadory
            }
        };

        if let Some(r) = r452{
            let new_transition = PetriTransition::new_from_name(&r);
            petri.add_transition(&new_transition);
            petri.update_edge(&PetriNode::Place(pattern.frontier.1.clone()), &PetriNode::Transition(new_transition.clone()));
            petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(pattern.frontier.0.clone()));
        }

    }
}

impl PetriNet<Regex<String>>{
    pub fn pure_path_len_1_places(&self, p1: &PetriPlace, p2:&PetriPlace) ->Option<PetriTransition<Regex<String>>>{
        let p1_index = self.get_node_index(&PetriNode::Place(p1.clone())).unwrap();
        let p2_index = self.get_node_index(&PetriNode::Place(p2.clone())).unwrap();

       let outgoing_p1_nbr = self.graph.neighbors_directed(p1_index,Direction::Outgoing).collect::<Vec<_>>();
        let outgoing_p2_nbr = self.graph.neighbors_directed(p2_index,Direction::Incoming).collect::<Vec<_>>();

        //Intersection
        for n1 in outgoing_p1_nbr.iter(){
            for n2 in outgoing_p2_nbr.iter(){
                if *n1==*n2{
                    //First, we make sure that the path is pure
                    if self.graph.neighbors_directed(*n1,Direction::Outgoing).count()!=1 || self.graph.neighbors_directed(*n1,Direction::Incoming).count()!=1{
                        continue;
                    }

                    let node = self.graph.node_weight(*n1).unwrap();
                    if let PetriNode::Transition(t) = node{
                        return Some(t.clone());
                    }
                    else{
                        panic!("We there is place between two places, but not a transition. Impossible.");
                    }
                }
            }
        }

        None
    }

    pub fn neighboring_places(&self,p: &PetriPlace)->Vec<PetriPlace>{
        let mut places = Vec::new();

        for p_prime in self.places.iter(){
            if p_prime==p{
                continue;
            }
            if self.pure_path_len_1_places(p, p_prime).is_some()|| self.pure_path_len_1_places(p_prime, p).is_some(){
                places.push(p_prime.clone());
            }
        }

        places
    }

    pub fn neighboring_places_trans(&self,t: &PetriTransition<Regex<String>>)->Vec<PetriPlace>{
        let mut places = Vec::new();
        let neighbors_out = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),Direction::Outgoing).collect::<Vec<_>>();
        let neighbors_in = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),Direction::Incoming).collect::<Vec<_>>();
        for n in neighbors_out.iter().chain(neighbors_in.iter()){
            let node = self.graph.node_weight(*n).unwrap();
            if let PetriNode::Place(p) = node{
                places.push(p.clone());
            }
            else{
                panic!("We there is place between two places, but not a transition. Impossible.");
            }
        }

        places.sort();
        places.dedup();

        places
    }

    pub fn neighboring_transitions(&self,p: &PetriPlace)->Vec<PetriTransition<Regex<String>>>{
        let mut transitions = Vec::new();
        let neighbors_out = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(p.clone())).unwrap(),Direction::Outgoing).collect::<Vec<_>>();
        let neighbors_in = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(p.clone())).unwrap(),Direction::Incoming).collect::<Vec<_>>();

        for n in neighbors_out.iter().chain(neighbors_in.iter()){
            let node = self.graph.node_weight(*n).unwrap();
            if let PetriNode::Transition(t) = node{
                transitions.push(t.clone());
            }
            else{
                panic!("We there is place between two places, but not a transition. Impossible.");
            }
        }
        transitions
    }

    pub fn are_all_neighbor_in(&self,p: &PetriPlace,transitions:&Vec<PetriTransition<Regex<String>>>) -> bool{

        let neighbors = self.neighboring_transitions(p);

        for n in neighbors.iter(){
            if !transitions.contains(n){
                return false;
            }
        }

        true
    }
}























