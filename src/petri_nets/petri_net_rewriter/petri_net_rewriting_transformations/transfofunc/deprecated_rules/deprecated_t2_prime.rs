use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t2_prime(petri: &PetriNet<Regex<String>>) ->bool{
    for p0 in petri.places.iter(){
        if petri.is_there_alt_prime_pattern(p0).len()>0{
            return true;
        }
    }

    false
}


pub fn apply_t2_prime(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{

    for p0 in petri.places.iter(){

          let alt_paths = petri.is_there_alt_prime_pattern(p0);

          if alt_paths.len()==0{
              continue;
          }

        let mut new_petri_net = petri.clone();

        //Retrieve regex and delete nodes
        let mut regex_vec = Vec::new();
        for path in alt_paths.iter(){
            if let PetriNode::Transition(t) = path[1].clone(){
                regex_vec.push(t.label.clone());
            }else{
                panic!("No transition between two places in the path.")
            }
            new_petri_net.remove_node(&path[1]);
            new_petri_net.remove_node(&path[2]);

        }

        let regex = Regex::or_with_plus_to_kleene_reduction_with_epsilon(&regex_vec);


        let new_place = PetriPlace::new_from_nothing();
        let new_transition = PetriTransition::new_from_name(&regex);

        new_petri_net.add_place(&new_place);
        new_petri_net.add_transition(&new_transition);

        new_petri_net.update_edge(&PetriNode::Place(p0.clone()),&PetriNode::Transition(new_transition.clone()));
        new_petri_net.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(
            new_place.clone()
        ));

        return vec![new_petri_net];
    }

    panic!("We have not found a transformation for T2', while the precondition is satisfied.");
}


impl PetriNet<Regex<String>>{
    pub fn is_there_alt_prime_pattern(&self,p0: &PetriPlace)->Vec<Vec<PetriNode<Regex<String>>>>{
        let mut alt_paths = Vec::new();

        //First we save the places that do not have any outgoing transition
        let mut target_places = Vec::new();
        for place in self.places.iter(){
            if place==p0{
                continue;
            }
            if self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(place.clone())).unwrap(),petgraph::Direction::Outgoing).count()==0{
                target_places.push(place.clone());
            }
        }
        //We search for path of length 3 between p0 and places in target_places
        let mut paths = Vec::new();
        for place in target_places.iter(){
            let simple_paths = all_simple_paths::<Vec<_>, _>(
                &self.graph,
                self.get_node_index(&PetriNode::Place(p0.clone())).unwrap(),
                self.get_node_index(&PetriNode::Place(place.clone())).unwrap(),
                1,
                Some(1),
            ).collect::<Vec<_>>();

            if simple_paths.len()==1{
                let path_as_nodes = simple_paths[0].iter().map(|x| self.graph[*x].clone()).collect::<Vec<_>>();
                paths.push(path_as_nodes);
            }
        }

        if paths.len()>1{
            alt_paths = paths;
        }



        alt_paths
    }
}


impl Regex<String>{
    pub fn or_with_plus_to_kleene_reduction_with_epsilon(regex_vec: &Vec<Regex<String>>) ->Regex<String>{
        let contains_epsilon = regex_vec.contains(&Regex::Epsilon);
        let mut contains_plus = false;

        for r in regex_vec.iter(){
            if let Regex::Plus(_) = r{
                contains_plus = true;
                break;
            }
        }
        let mut regex = Regex::Empty;

        for r in regex_vec.iter(){
            match r {
                Regex::Epsilon=>{
                    //regex = Regex::or(&regex,r);

                    if contains_plus{
                        continue;
                    }
                    else{
                        regex = Regex::or(&regex,r);
                    }


                },
                Regex::Plus(inner)=>{
                    if contains_epsilon{
                        let inner_kleene = Regex::kleene(&*inner.clone());
                        regex = Regex::or(&regex, &inner_kleene);
                    }
                    else{
                        regex = Regex::or(&regex,r);
                    }
                },
                _ =>{
                    regex = Regex::or(&regex,r);
                }
            }
        }

        regex
    }
}








