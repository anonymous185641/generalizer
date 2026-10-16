use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::{PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::concatenation_loop::ConcatenationLoopPattern;
use crate::regex_inf::regex::regex::Regex;





pub fn can_apply_alternative_loop(petri: &PetriNet<Regex<String>>) -> bool {
    if let Some(pattern) = AlternativeLoopPattern::search_pattern(petri){
        println!("Looped Alternative found");
        return true;
    }
    false
}


pub fn apply_alternative_loop(petri: &PetriNet<Regex<String>>)->Vec<PetriNet<Regex<String>>>{
    if let Some(pattern) = AlternativeLoopPattern::search_pattern(petri){
        let mut new_petri_net = petri.clone();
        AlternativeLoopPattern::replace_pattern(&mut new_petri_net,&pattern);
        new_petri_net.draw(&"test".to_string(),&"temp1".to_string());
        return vec![new_petri_net];
    }
    panic!("We have not found a transformation for alternative loop, while the precondition is satisfied.");
}

pub struct AlternativeLoopPattern{
    pub frontier: PetriPlace,
    pub redex: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>)
}

impl AlternativeLoopPattern{
    pub fn new(frontier: PetriPlace, redex: (PetriTransition<Regex<String>>,PetriTransition<Regex<String>>)) -> Self{
        Self{ frontier, redex }
    }

    pub fn search_pattern(petri: &PetriNet<Regex<String>>)->Option<AlternativeLoopPattern>{

        for p in petri.places.iter(){
            let p_successors = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p.clone())).unwrap(),petgraph::Direction::Outgoing).collect::<Vec<_>>();
            let p_predecessors = petri.graph.neighbors_directed(petri.get_node_index(&PetriNode::Place(p.clone())).unwrap(),petgraph::Direction::Incoming).collect::<Vec<_>>();

            let mut transitions = Vec::new();

            for succ in p_successors.iter(){
                if p_predecessors.contains(&succ.clone()) {
                    transitions.push(succ.clone());
                }
            }

            if transitions.len()<2{
                continue;
            }

            let trans_as_nodes = transitions.iter().map(|x| petri.graph[*x].clone()).collect::<Vec<_>>();

            let mut redex_transitions = Vec::new();
            for node in trans_as_nodes.iter(){
                if redex_transitions.len()==2{
                    break;
                }
                let t = node.unfold_transition().unwrap();
                if petri.neighboring_places_trans(&t)==vec![p.clone()]{
                    redex_transitions.push(t.clone());
                }
            }
            println!("GGGGGGGGGGGGGGGG {:?}",redex_transitions);
            if redex_transitions.len()!=2{
                continue;
            }
           return  Some(AlternativeLoopPattern::new(p.clone(),(redex_transitions[0].clone(),redex_transitions[1].clone())));

        }


        None
    }


    pub fn replace_pattern(petri: &mut PetriNet<Regex<String>>,pattern: &AlternativeLoopPattern){
        //Deletion
        petri.remove_node(&PetriNode::Transition(pattern.redex.0.clone()));
        petri.remove_node(&PetriNode::Transition(pattern.redex.1.clone()));

        //Creation
        let new_transition = PetriTransition::new_from_name(&Regex::or_vec_retain_non_empty(vec![pattern.redex.0.label.clone(),pattern.redex.1.label.clone()]));
        petri.add_transition(&new_transition);
        petri.update_edge(&PetriNode::Place(pattern.frontier.clone()), &PetriNode::Transition(new_transition.clone()));
        petri.update_edge(&PetriNode::Transition(new_transition.clone()),&PetriNode::Place(pattern.frontier.clone()));
    }

}






