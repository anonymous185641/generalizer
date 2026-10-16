use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t6(petri: &PetriNet<Regex<String>>) ->bool{
    match petri.is_there_loop_pattern(){
        Some(_) => true,
        None => false
    }
}

pub fn apply_t6(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
    match petri.is_there_loop_pattern(){
        Some(path) => {
           let mut new_petri_net = petri.clone();

            let new_place = PetriPlace::new_from_nothing();

            //We concatenate the labels of transitions
            let mut regex = Regex::Empty;
            for node in path.iter(){
                if let PetriNode::Transition(t) = node{
                    regex = Regex::concatenate(&regex,&t.label);
                }
            }
            regex = Regex::plus(&regex);
            let new_transition = PetriTransition::new_from_name(&regex);

            //We remove de old nodes, except the source of the loop
            for i in 1..path.len()-1{
                new_petri_net.remove_node(&path[i]);
            }

            //We add the new nodes
            new_petri_net.add_place(&new_place);
            new_petri_net.add_transition(&new_transition);

            //We add the new edges
            new_petri_net.update_edge(&path[0],&PetriNode::Transition(new_transition.clone()));
            new_petri_net.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(new_place.clone()));


            vec![new_petri_net]
        }
        None => panic!("We have not found a transformation for T6, while the precondition is satisfied.")
    }
}


impl PetriNet<Regex<String>>{
    
    
    pub fn is_there_loop_pattern(&self)->Option<Vec<PetriNode<Regex<String>>>>{
        for p in self.places.iter(){
            //****** I center the loop around the node that has the token or has at least two incoming edges
            let incoming_edges = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(p.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();

            if !self.initial_marking.tokens.contains_key(&p) && incoming_edges.len()<2 {
                continue;
            }
            //******


            //We search for a path between p and p with exactly 3 intermediates nodes

            let simple_paths = all_simple_paths::<Vec<_>, _>(
                &self.graph,
                self.get_node_index(&PetriNode::Place(p.clone())).unwrap(),
                self.get_node_index(&PetriNode::Place(p.clone())).unwrap(),
                3,
                Some(3),
            ).collect::<Vec<_>>();

            //println!("{:?}",simple_paths);
            if simple_paths.len()>0{
                let path_as_nodes = simple_paths[0].iter().map(|x| self.graph[*x].clone()).collect::<Vec<_>>();
                if self.is_pure_path(&path_as_nodes){
                    return Some(path_as_nodes);
                }
            }
        }

        None
    }
}















