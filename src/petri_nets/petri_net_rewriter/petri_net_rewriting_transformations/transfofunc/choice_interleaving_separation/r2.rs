use std::fmt::Debug;
use std::hash::Hash;
use itertools::Itertools;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriNetTransitionLabelBound, PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_r2(petri: &PetriNet<Regex<String>>) ->bool{
    for (p1,p2) in petri.places.iter().tuple_combinations(){

        if p1 == p2{
            continue;
        }
        //We retrieve the set of incoming transitions of p1
        let mut outgoing_transitions_of_p1 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p1.clone())).unwrap(),
                                                                            petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();

        //We retrieve the set of incoming transitions of p2
        let mut outgoing_transitions_of_p2 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p2.clone())).unwrap(),
                                                                            petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();
        //We compare the two sets
        outgoing_transitions_of_p1.sort();
        outgoing_transitions_of_p2.sort();
        if (outgoing_transitions_of_p1 == outgoing_transitions_of_p2)&& (outgoing_transitions_of_p1.len() > 1) {
            return true;
        }

    }

    false
}

pub fn apply_r2(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    for (p1,p2) in petri.places.iter().tuple_combinations(){
        println!("{:?}",(p1,p2));
        if p1 == p2{
            continue;
        }
        //We retrieve the set of incoming transitions of p1
        let mut outgoing_transitions_of_p1 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p1.clone())).unwrap(),
                                                                            petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();

        //We retrieve the set of incoming transitions of p2
        let mut outgoing_transitions_of_p2 = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p2.clone())).unwrap(),
                                                                            petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();
        //We compare the two sets
        outgoing_transitions_of_p1.sort();
        outgoing_transitions_of_p2.sort();
        if (outgoing_transitions_of_p1 == outgoing_transitions_of_p2)&& (outgoing_transitions_of_p1.len() > 1){
            //Here, we do the transformation
            let mut new_petri_net = petri.clone();

            //First, we remove the old incoming edges p1 and p2
            for outgoing_transition in outgoing_transitions_of_p1.iter(){
                let p1_index = new_petri_net.get_node_index(&PetriNode::Place(p1.clone())).unwrap();
                if let Some(e) = new_petri_net.graph.find_edge(p1_index,outgoing_transition.clone()){
                    new_petri_net.graph.remove_edge(e);
                }

                let p2_index = new_petri_net.get_node_index(&PetriNode::Place(p2.clone())).unwrap();
                if let Some(e) = new_petri_net.graph.find_edge(p2_index,outgoing_transition.clone()){
                    new_petri_net.graph.remove_edge(e);
                }

            }



            //We create the new places and transitions
            let new_place = PetriPlace::new_from_nothing();
            let new_transition = PetriTransition::new_from_name(&Regex::Epsilon);
            new_petri_net.add_place(&new_place);
            new_petri_net.add_transition(&new_transition);

            //We update the graph
            new_petri_net.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(new_place.clone()));
            new_petri_net.update_edge(&PetriNode::Place(p1.clone()),&PetriNode::Transition(new_transition.clone()));
            new_petri_net.update_edge(&PetriNode::Place(p2.clone()),&PetriNode::Transition(new_transition.clone()));

            for outgoing_transition in outgoing_transitions_of_p1{
                new_petri_net.update_edge(&PetriNode::Place(new_place.clone()),&new_petri_net.clone().graph.node_weight(outgoing_transition).unwrap());
            }

            return vec![new_petri_net];

        }

    }

    //If we reach this point, we have not found a transformation
    panic!("We have not found a transformation for R1, while the precondition is satisfied.");
}