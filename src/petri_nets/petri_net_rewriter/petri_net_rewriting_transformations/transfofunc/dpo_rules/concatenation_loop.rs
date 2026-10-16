use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::concatenation::ConcatenationPattern;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_concatenation_loop(petri: &PetriNet<Regex<String>>) -> bool {
    if let Some(pattern) = ConcatenationLoopPattern::search_pattern(petri){
        println!("Looped Concatenation found");
        return true;
    }
    false
}


pub fn apply_concatenation_loop(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    if let Some(pattern) = ConcatenationLoopPattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        ConcatenationLoopPattern::replace_pattern(&mut new_petri_net,&pattern);
        new_petri_net.draw(&"test".to_string(),&"temp1".to_string());
        return vec![new_petri_net];
    }
    panic!("We have not found a transformation for concatenation loop, while the precondition is satisfied.");
}


pub struct ConcatenationLoopPattern{
    pub frontier: PetriPlace,
    pub redex: Vec<PetriNode<Regex<String>>>
}


impl ConcatenationLoopPattern{
    pub fn new(frontier: PetriPlace, redex: Vec<PetriNode<Regex<String>>>) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<ConcatenationLoopPattern>{
        for p in petri.places.iter(){
            //****** I center the loop around the node that has the token or has at least two incoming edges
            let incoming_edges = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();

            if !petri.initial_marking.tokens.contains_key(&p) && incoming_edges.len()<2 {
                continue;
            }
            //******


            //We search for a path between p and p with exactly 3 intermediates nodes

            let simple_paths = all_simple_paths::<Vec<_>, _>(
                &petri.graph,
                petri.get_node_index(&PetriNode::Place(p.clone())).unwrap(),
                petri.get_node_index(&PetriNode::Place(p.clone())).unwrap(),
                3,
                Some(3),
            ).collect::<Vec<_>>();

            //println!("{:?}",simple_paths);
            if simple_paths.len()>0{
                let mut path_as_nodes = simple_paths[0].iter().map(|x| petri.graph[*x].clone()).collect::<Vec<_>>();
                if petri.is_pure_path(&path_as_nodes){
                    path_as_nodes.remove(0);
                    path_as_nodes.remove(path_as_nodes.len()-1);
                    let redex = path_as_nodes;
                    return Some(ConcatenationLoopPattern::new(p.clone(),redex));
                }
            }
        }

        None
    }

    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &ConcatenationLoopPattern){
        for node in pattern.redex.iter(){
            petri.remove_node(node);
        }

        let regex = pattern.get_synthetic_regex();
        let new_transition = PetriTransition::new_from_name(&regex);

        petri.add_transition(&new_transition);
        petri.update_edge(&PetriNode::Place(pattern.frontier.clone()), &PetriNode::Transition(new_transition.clone()));
        petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(pattern.frontier.clone()));

    }

    pub fn get_synthetic_regex(&self)->Regex<String>{
        let mut regex = Regex::Empty;
        for node in self.redex.iter(){
            if let PetriNode::Transition(t) = node{
                regex =  Regex::concatenate(&regex, &t.label);
            }
        }
        regex
    }
}