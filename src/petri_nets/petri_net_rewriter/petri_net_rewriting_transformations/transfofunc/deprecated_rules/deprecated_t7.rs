use petgraph::algo::all_simple_paths;
use petgraph::Direction;
use petgraph::visit::IntoNeighborsDirected;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t7(petri: &PetriNet<Regex<String>>) ->bool{
    match petri.is_there_kleene_loop_pattern(){
        Some(r) => true,
        None => false
    }
}



pub fn apply_t7(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
  match petri.is_there_kleene_loop_pattern(){
       Some(r) => vec![petri.merge_kleene_loop_pattern(&r)],
       None => vec![]
   }
}


impl PetriNet<Regex<String>>{
    pub fn is_there_kleene_loop_pattern(&self)->Option<Regex<String>>{
        if self.graph.node_count() != 5{
            return None;
        }

        let initial_place = self.initial_marking.tokens.keys().collect::<Vec<_>>()[0].clone();

        let two_transitions = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(initial_place.clone())).unwrap(),Direction::Outgoing).collect::<Vec<_>>();
        if two_transitions.len() == 2{
            let t1_node = self.graph[two_transitions[0]].clone();
            let t2_node = self.graph[two_transitions[1]].clone();

            let p1 = self.graph[self.graph.neighbors_directed(two_transitions[0],Direction::Outgoing).collect::<Vec<_>>()[0]].clone();
            let p2 = self.graph[self.graph.neighbors_directed(two_transitions[1],Direction::Outgoing).collect::<Vec<_>>()[0]].clone();

            if p1 != PetriNode::Place(initial_place.clone()) && p2!=PetriNode::Place(initial_place.clone()){

                if let (PetriNode::Transition(t1),PetriNode::Transition(t2)) = (t1_node, t2_node){
                    match (t1.label,t2.label){
                        (Regex::Epsilon,Regex::Plus(r))
                        |(Regex::Plus(r),Regex::Epsilon)=>{
                            let r1 = *r.clone();
                            return Some(Regex::kleene(&r1))
                        },
                        _=> {}
                    }
                }

            }
        }
       None
    }


    pub fn merge_kleene_loop_pattern(&self,regex:&Regex<String>) ->PetriNet<Regex<String>>{
        let initial_place = self.initial_marking.tokens.keys().collect::<Vec<_>>()[0].clone();
        let mut new_petri_net = self.clone();

        //We delete all places and transisions other than the initial place
        for node in self.graph.node_indices(){
            let petri_node = new_petri_net.graph[node].clone();

            if let PetriNode::Place(place) = &petri_node{
                if *place != initial_place{
                    new_petri_net.remove_node(&petri_node);
                }
            }
            else {
                new_petri_net.remove_node(&petri_node);
            }
        }

        //Create the new stuff
        let new_place = PetriPlace::new_from_nothing();
        let new_transition = PetriTransition::new_from_name(regex);

        new_petri_net.add_transition(&new_transition);
        new_petri_net.add_place(&new_place);

        new_petri_net.update_edge(&PetriNode::Place(initial_place),&PetriNode::Transition(new_transition.clone()));
        new_petri_net.update_edge(&PetriNode::Transition(new_transition),&PetriNode::Place(new_place.clone()));

        new_petri_net
    }
}













