use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t3_prime(petri: &PetriNet<Regex<String>>) ->bool{
    for t in petri.transitions.iter(){
        if petri.is_there_interleaving_prime_pattern(t).len()>0{
            return true;
        }
    }
    false
}


pub fn apply_t3_prime(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
    for t in petri.transitions.iter(){
        let interleaving_paths = petri.is_there_interleaving_prime_pattern(t);

        if interleaving_paths.len()==0{
           continue;
        }

        let mut new_petri_net = petri.clone();

        //Retrieve regex and delete nodes
        let mut regex = Regex::Empty;
        for path in interleaving_paths.iter(){
            if let PetriNode::Transition(t) = path[2].clone(){
                regex = Regex::interleave(&regex,&t.label);
            }else{
                panic!("No transition between two places in the path.")
            }
            new_petri_net.remove_node(&path[1]);
            new_petri_net.remove_node(&path[2]);
            new_petri_net.remove_node(&path[3]);
        }
        new_petri_net.remove_node(&PetriNode::Transition(t.clone()));

        regex = Regex::concatenate(&t.label,&regex);

        //New transition
        let new_transition = PetriTransition::new_from_name(&regex);
        new_petri_net.add_transition(&new_transition);

        let new_place = PetriPlace::new_from_nothing();
        new_petri_net.add_place(&new_place);

        new_petri_net.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(new_place.clone()));
        //Retrieving the incoming nodes of the transition
        let incoming_nodes = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();

        let incoming_as_nodes = incoming_nodes.iter().map(|n| new_petri_net.graph[n.clone()].clone()).collect::<Vec<_>>();

        for incoming_node in incoming_as_nodes.iter(){
            new_petri_net.update_edge(&incoming_node,&PetriNode::Transition(new_transition.clone()));
        }

        return vec![new_petri_net];
    }



   panic!("We have not found a transformation for T3', while the precondition is satisfied.");
}


impl PetriNet<Regex<String>>{
    pub fn is_there_interleaving_prime_pattern(&self,t: &PetriTransition<Regex<String>>)->Vec<Vec<PetriNode<Regex<String>>>>{
        let mut interleaving_paths = Vec::new();


        //First we save the places that do not have any outgoing transition
        let mut target_places = Vec::new();
        for place in self.places.iter(){
            if self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(place.clone())).unwrap(),petgraph::Direction::Outgoing).count()==0{
                target_places.push(place.clone());
            }
        }
        //We search for path of length 3 between p0 and places in target_places
        let mut paths = Vec::new();
        for place in target_places.iter(){
            let simple_paths = all_simple_paths::<Vec<_>, _>(
                &self.graph,
                self.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),
                self.get_node_index(&PetriNode::Place(place.clone())).unwrap(),
                2,
                Some(2),
            ).collect::<Vec<_>>();

            if simple_paths.len()==1{
                let path_as_nodes = simple_paths[0].iter().map(|x| self.graph[*x].clone()).collect::<Vec<_>>();
                paths.push(path_as_nodes);
            }
        }

        if paths.len()>1{
            interleaving_paths = paths;
        }



        interleaving_paths
    }
}