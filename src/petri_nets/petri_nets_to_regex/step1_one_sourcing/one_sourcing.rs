use std::collections::{BTreeMap, HashMap};
use is_empty::IsEmpty;
use petgraph::Direction;
use petgraph::visit::IntoNeighborsDirected;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode, PlaceNumberToken};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

impl PetriNet<Regex<String>>{
    /*
    pub fn one_sourcing(&mut self){
        /*In this function, we assume that the Petri net is safe*/

        if self.initial_marking.tokens.len()>1{
            let new_place = PetriPlace::new_from_nothing();
            let silent_transition = PetriTransition::new_from_name(&Regex::Epsilon);

            //Add the new place and transition to the net
            self.add_place(&new_place);
            self.add_transition(&silent_transition);

            //Add the new arcs
            self.update_edge(&PetriNode::Place(new_place.clone()),&PetriNode::Transition(silent_transition.clone()));
            for (old_initial_place,_) in self.initial_marking.tokens.clone().iter(){
                self.update_edge(&PetriNode::Transition(silent_transition.clone()),&PetriNode::Place(old_initial_place.clone()))
            }

            //Update the marking
            self.initial_marking.tokens = BTreeMap::from([(new_place,PlaceNumberToken(1))]);

        }
    }

     */
    /* Old version
    pub fn one_sourcing_old(&mut self){
        /*In this function, we assume that the Petri net is safe*/

         if self.initial_marking.tokens.len()>1{

            let new_place = PetriPlace::new_from_nothing();
            let silent_transition = PetriTransition::new_from_name(&Regex::Epsilon);

            //Add the new place and transition to the net
            self.add_place(&new_place);
            self.add_transition(&silent_transition);

            //Add the new arcs
            self.update_edge(&PetriNode::Place(new_place.clone()),&PetriNode::Transition(silent_transition.clone()));
            for (old_initial_place,_) in self.initial_marking.tokens.clone().iter(){
                self.update_edge(&PetriNode::Transition(silent_transition.clone()),&PetriNode::Place(old_initial_place.clone()))
            }



            //We relocate the extra silent transition
            if let Some(t) = self.get_extra_silent_transition_pair(){

                //We delete its previous incoming edges
                let preds = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t.clone())).unwrap(),Direction::Incoming).collect::<Vec<_>>();

                for pred in preds.iter(){
                    let edge = self.graph.find_edge(pred.clone(),self.get_node_index(&PetriNode::Transition(t.clone())).unwrap()).unwrap();
                    self.graph.remove_edge(edge);

                }

                //We connect it to the new place
                self.update_edge(&PetriNode::Place(new_place.clone()),&PetriNode::Transition(t.clone()));
            }

            //Update the marking
            self.initial_marking.tokens = BTreeMap::from([(new_place.clone(),PlaceNumberToken(1))]);

        }
    }
    */
    pub fn one_sourcing(&mut self){
        /*In this function, we assume that the Petri net is safe*/

            //If there is only one initial place such that it does not have an incoming arc, we do nothing
        if self.initial_marking.tokens.len()==1{
            let p0 = self.initial_marking.tokens.keys().next().unwrap();
            if self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(p0.clone())).unwrap(),Direction::Incoming).count()==0{
                return;
            }
        }
        //Otherwise, we create a new place and a new silent transition
        
        let new_place = PetriPlace::new_from_nothing();
        let silent_transition = PetriTransition::new_from_name(&Regex::Epsilon);

        //Add the new place and transition to the net
        self.add_place(&new_place);
        self.add_transition(&silent_transition);

        //Add the new arcs
        self.update_edge(&PetriNode::Place(new_place.clone()),&PetriNode::Transition(silent_transition.clone()));
        for (old_initial_place,_) in self.initial_marking.tokens.clone().iter(){
            self.update_edge(&PetriNode::Transition(silent_transition.clone()),&PetriNode::Place(old_initial_place.clone()))
        }

        //Update the marking
        self.initial_marking.tokens = BTreeMap::from([(new_place.clone(),PlaceNumberToken(1))]);

        
    }
    
    pub fn one_ending(&mut self){
        //If there are more than one final place (i.e., without an outgoing node), connect them to a unique finial transition
        let mut  final_places = Vec::new();
        
        for place in self.places.clone().iter(){
            if self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(place.clone())).unwrap(),Direction::Outgoing).count()==0{
                final_places.push(place.clone());
            }
        }
        
        
        if final_places.len()>1{
            let new_transition = PetriTransition::new_from_name(&Regex::Epsilon);
            self.add_transition(&new_transition);

            let new_place = PetriPlace::new_from_nothing();
            self.add_place(&new_place);
            
            self.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(new_place.clone()));

            for place in final_places.iter(){
                self.update_edge(&PetriNode::Place(place.clone()),&PetriNode::Transition(new_transition.clone()));
            }
        }
        
    }

    pub fn get_extra_silent_transition_pair(&self)->Option<PetriTransition<Regex<String>>>{
        for transition in self.transitions.clone().iter(){
            if transition.label==Regex::Epsilon{

                //We check if all its predecessors are marked places
                let predecessors = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(transition.clone())).unwrap(),Direction::Incoming).collect::<Vec<_>>();

                for precursor in predecessors.iter(){
                    if let PetriNode::Place(p) = self.graph[*precursor].clone(){
                        if !self.initial_marking.tokens.contains_key(&p){
                            return None;
                        }
                    }
                    else{
                        panic!("The predecessor of a transition is not a place");
                    }
                }


                //We check if it has only one child and one grandchild
                let childs = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(transition.clone())).unwrap(),Direction::Outgoing).collect::<Vec<_>>();

                if childs.len()==1{
                    let grand_childs = self.graph.neighbors_directed(childs[0],Direction::Outgoing).collect::<Vec<_>>();
                    if grand_childs.is_empty(){
                        let child_place = if let PetriNode::Place(place) = self.graph[childs[0]].clone(){
                            place
                        }
                        else{
                            return None;
                        };
                        return Some(transition.clone())
                    }
                }

            }
        }

        None
    }


}



















