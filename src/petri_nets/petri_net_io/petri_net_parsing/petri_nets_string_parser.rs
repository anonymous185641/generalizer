use std::fmt::Debug;
use std::hash::Hash;
use itertools::Itertools;
use pest::Parser;
use crate::global_counter::counter::fresh_number;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode,  PlaceNumberToken};
use crate::petri_nets::petri_net::place_transition::{PetriNetTransitionLabelBound, PetriPlace, PetriTransition, TransitionId};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_parser::{PetriNetsGrammarParser, Rule};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_parser::Rule::{DCL_GRAPH_NAME, DCL_GRAPH_TYPE, DCL_MARKING, DCL_PATH, DCL_TRANSITIONS, STATE_OR_TRANSITION, TRANSITION};
use crate::petri_nets::petri_net_io::petri_net_parsing::util_types::{retrieve_transition_prefix, Place, Trans};
use crate::regex_inf::regex::regex::Regex;

pub fn petri_nets_parse_string <T: PetriNetTransitionLabelBound> (unparsed_string:&String) ->  Result<PetriNet<T>,String> {

    match PetriNetsGrammarParser::parse(Rule::PEST_FILE,unparsed_string){
        Ok(ref mut got_pairs)=>{
            let mut petri_net:PetriNet<T> = PetriNet::new();
            let mut transition_labels:Vec<Trans> = Vec::new();
            let mut state_labels:Vec<Place> = Vec::new();

            for pair in got_pairs{
                if pair.as_rule() == DCL_GRAPH_NAME{
                    //Not important for now.
                }

                if pair.as_rule() == DCL_TRANSITIONS{
                    //Parsing of the transition labels
                    let pair_inner = pair.clone().into_inner();
                    for inner in pair_inner{
                        if inner.as_rule()==TRANSITION{
                            transition_labels.push(Trans(inner.as_str().to_string()));
                        }
                    }
                }

                if pair.as_rule()== DCL_GRAPH_TYPE{
                    //Not important for now.
                }
                if pair.as_rule() == DCL_PATH{
                    let pair_inner = pair.clone().into_inner();
                    let mut path:Vec<String> = Vec::new();

                    for inner in pair_inner{
                        if inner.as_rule()==STATE_OR_TRANSITION{
                            path.push(inner.as_str().to_string());
                        }
                    }

                    if path.len() < 2{
                        panic!("A path must have at least two elements.");
                    }

                    // Path parsing
                    //println!("tlabs {:?}",transition_labels);
                    //println!("path {:?}",path);
                    if transition_labels.contains( &Trans(retrieve_transition_prefix(&path[0].clone()).to_string())){
                        //The source is a transition
                        let source_transition = petri_net.get_or_add_transition_from_label(&T::from_string(&path[0]));

                        for i in 1..path.len(){
                            //println!("target:{:?} ",path[i]);
                            if transition_labels.contains(&Trans(retrieve_transition_prefix(&path[i].clone()).to_string())){
                                // This target is a transition
                                // A fresh buffer place need to be created

                                //We create the buffer place
                                let buffer_place = PetriPlace::new_from_nothing();
                                petri_net.add_place(&buffer_place);
                                petri_net.update_edge(&PetriNode::Transition(source_transition.clone()),&PetriNode::Place(buffer_place.clone()));
                                //println!("BBBB {:?}",buffer_place);
                                //We create the transition
                                let target_transition = petri_net.get_or_add_transition_from_label(&T::from_string( &path[i]));
                                petri_net.update_edge(&PetriNode::Place(buffer_place.clone()),&PetriNode::Transition(target_transition.clone()));

                            }
                            else{
                                // This target is a place, no buffer place needed

                                // We create the place
                                let target_place = petri_net.get_or_add_place_from_label(&path[i].clone());
                                petri_net.update_edge(&PetriNode::Transition(source_transition.clone()),&PetriNode::Place(target_place.clone()))
                            }
                        }
                    }
                    else{
                        //The source is a place
                        let source_place = petri_net.get_or_add_place_from_label(&path[0].clone());
                        //println!("source place {:?}",source_place);

                        for i in 1..path.len(){
                            if transition_labels.contains(&Trans(retrieve_transition_prefix(&path[i].clone()).to_string())) {
                                // This target is a transition
                                let target_transition = petri_net.get_or_add_transition_from_label(&T::from_string(&path[i].clone()));
                                petri_net.update_edge(&PetriNode::Place(source_place.clone()),&PetriNode::Transition(target_transition.clone()));

                            }
                            else{
                                // This target is a place
                                // Impossible
                                // To do: improve the error handling
                                panic!("A path cannot go from a place to a place. There must be a transition in the middle.",);
                            }
                        }

                    }

                    //HERE YOU NEED TO HANDLE DE MESS OF .g ASSOCIATIONS
                    //println!("{:?}",all_state_transitions_labels);

                }

                if pair.as_rule() == DCL_MARKING{
                    //Parsing of the initial states
                    //println!("DLC marking {:?}",pair);
                    let marking_field = pair.clone().into_inner();



                    for marking_list in marking_field{
                        let inner_markings = marking_list.clone().into_inner();

                        for marked_spec in inner_markings{
                            //println!("TTTTT  {:?}",marked_spec);
                            let marked_spec_inner = marked_spec.clone().into_inner().next().unwrap();
                            match marked_spec_inner.as_rule(){
                                Rule::PLACE_NAME =>{
                                    let label = marked_spec.as_str().to_string();
                                    
                                    if let Some(place) = petri_net.get_place_from_label(&label){
                                        petri_net.initial_marking.tokens.insert(place,PlaceNumberToken(1));
                                    }
                                },
                                Rule::MARKING_WITH_TRANSITION_PAIR =>{
                                    let marked_spec_inner_inner = marked_spec.clone().into_inner().next().unwrap().clone().into_inner();
                                    let mut labels = Vec::new();

                                    for pair in marked_spec_inner_inner{
                                        labels.push(pair.as_str().to_string());
                                    }

                                    if labels.len() != 2{
                                        panic!("A marking must be of the form <t1,t2>");
                                    }
                                    //retrieve the place between labels[0] and labels[1]
                                    if let Some(place) = petri_net.get_place_between_from_labels(& T::from_string(&labels[0]),&T::from_string(&labels[1])){
                                        petri_net.initial_marking.tokens.insert(place,PlaceNumberToken(1));
                                    }
                                    else{
                                        panic!("The place between {} and {} does not exist.",labels[0],labels[1]);
                                    }
                                }
                                _ =>{}
                            }


                        }

                    }

                }

            }

            Ok(petri_net)
        },
        Err(e)=>{
            return Err(e.to_string());
        }
    }

}



pub trait LiteralFromString<T: Debug+ Clone+ PartialEq+ Eq+ Hash+ PartialOrd+ Ord>{
    fn from_string(s:&String) ->T;
}

impl LiteralFromString<String> for String{
    fn from_string(s:&String) -> String{
        s.clone()
    }
}

impl LiteralFromString<Regex<String>> for Regex<String>{
    fn from_string(s: &String) -> Regex<String> {
        
        if s.eq(&"Epsilon".to_string()){
            return Regex::Epsilon;
        }
        else{
            Regex::Literal(s.clone())
        }
    }
    
}