
use is_empty::IsEmpty;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriNetTransitionLabelBound, PetriTransition};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;
use crate::regex_inf::regex::regex::Regex;



pub fn can_apply_t1(petri: &PetriNet<Regex<String>>) ->bool{

    for pair in petri.places.iter().permutations(2){
        let (p1,p2) = (pair[0].clone(),pair[1].clone());
        if !petri.is_there_pure_sequential_path(&PetriNode::Place(p1.clone()),&PetriNode::Place(p2.clone())).is_empty(){
            return true;
        }
    }

    false
}

pub fn apply_t1(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{

        for pair in petri.places.iter().permutations(2){
            let (p1,p2) = (pair[0].clone(),pair[1].clone());

            let seq_path = petri.is_there_pure_sequential_path(&PetriNode::Place(p1.clone()),&PetriNode::Place(p2.clone()));
            if !seq_path.is_empty(){
                //Here, we do the transformation
                let mut new_petri_net = petri.clone();

                new_petri_net.merge_pure_sequential_path(&seq_path);

                return vec![new_petri_net];
            }
        }

        panic!("We have not found a transformation for T1, while the precondition is satisfied.");
}


impl PetriNet<Regex<String>>{
    pub fn is_there_pure_sequential_path(&self,p1: &PetriNode<Regex<String>>,p2: &PetriNode<Regex<String>>)->Vec<PetriNode<Regex<String>>>{
        let mut seq_path = Vec::new();

        //We search for a path between p1 and p2 with at least 3 intermediates nodes
        let simple_paths = all_simple_paths::<Vec<_>, _>(
            &self.graph,
            self.get_node_index(&p1).unwrap(),
            self.get_node_index(&p2).unwrap(),
            3,
            None,
        ).collect::<Vec<_>>();

        for path in simple_paths.iter(){

            let path_as_nodes = path.iter().map(|x| self.graph[*x].clone()).collect::<Vec<_>>();
            //println!("We have found a path: {:?}",path_as_nodes);
            if self.is_pure_path(&path_as_nodes){
                seq_path = path_as_nodes;
                //println!("We have found a pure sequential path: {:?}",seq_path);
                return seq_path;
            }
        }
         seq_path
    }



    pub fn is_pure_path(&self,seq_path: &Vec<PetriNode<Regex<String>>>)->bool{
        //The outgoing degrees of nodes in indices 1..seq_path.len()-1 must be 1
        for i in 1..seq_path.len()-1{
            if self.graph.neighbors_directed(self.get_node_index(&seq_path[i]).unwrap(),petgraph::Direction::Outgoing).count() != 1
            || self.graph.neighbors_directed(self.get_node_index(&seq_path[i]).unwrap(),petgraph::Direction::Incoming).count() != 1{
                return false;
            }
        }
        true
    }

    pub fn merge_pure_sequential_path(&mut self,seq_path: &Vec<PetriNode<Regex<String>>>){

        let mut regex = Regex::Empty;

        for node in seq_path.iter(){

            if let PetriNode::Transition(t) = node{

                if t.label.is_empty(){
                    continue;
                }
                regex =  Regex::concatenate(&regex, &t.label);

            }
        }

        //We remove the old nodes
        for i in 1..seq_path.len()-1{
            self.remove_node(&seq_path[i]);
        }
        //We add the new node
        let new_transition = PetriTransition::new_from_name(&regex);
        self.add_transition(&new_transition);
        self.update_edge(&seq_path[0],&PetriNode::Transition(new_transition.clone()));
        self.update_edge(&PetriNode::Transition(new_transition.clone()),&seq_path[seq_path.len()-1]);

    }

}



















