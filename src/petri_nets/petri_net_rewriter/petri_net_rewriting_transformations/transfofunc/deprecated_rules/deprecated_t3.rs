use is_empty::IsEmpty;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net::place_transition::PetriTransition;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;
use crate::regex_inf::regex::regex::Regex;

pub fn can_apply_t3(petri: &PetriNet<Regex<String>>) ->bool{
    for pair in petri.transitions.iter().permutations(2) {
        let (t1, t2) = (pair[0].clone(), pair[1].clone());
        if petri.is_there_interleaving_pattern(&t1, &t2).len() > 1 {
            return true;
        }
    }
    false
}

pub fn apply_t3(petri: &PetriNet<Regex<String>>) ->Vec<PetriNet<Regex<String>>>{
    for pair in petri.transitions.iter().permutations(2) {
        let (t1, t2) = (pair[0].clone(), pair[1].clone());
        let interleaving_paths = petri.is_there_interleaving_pattern(&t1, &t2);
        if interleaving_paths.len() > 1 {
            let mut new_petri_net = petri.clone();

            //println!("Interleaving paths: {:?}",interleaving_paths);
            new_petri_net.merge_interleaving_pattern(&interleaving_paths);

            return vec![new_petri_net];
        }

    }
    panic!("We have not found a transformation for T3, while the precondition is satisfied.");
}

impl PetriNet<Regex<String>>{
    pub fn is_there_interleaving_pattern(&self,t1: &PetriTransition<Regex<String>>,t2: &PetriTransition<Regex<String>>)->Vec<Vec<PetriNode<Regex<String>>>>{
        let mut interleaving_paths = Vec::new();

        //We search for a path between t1 and t2 with exactly 3 intermediates nodes
        let simple_paths = all_simple_paths::<Vec<_>, _>(
            &self.graph,
            self.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(),
            self.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(),
            3,
            Some(3),
        ).collect::<Vec<_>>();

        for path in simple_paths.iter(){

            let path_as_nodes = path.iter().map(|x| self.graph[*x].clone()).collect::<Vec<_>>();

            if self.is_pure_path(&path_as_nodes){
                    //println!("Interleaving path: {:?}",path_as_nodes);
                interleaving_paths.push(path_as_nodes);
            }

        }

        interleaving_paths
    }

    pub fn merge_interleaving_pattern(&mut self,interleaving_path: &Vec<Vec<PetriNode<Regex<String>>>>){
        let mut regex = Regex::Empty;



        for path in interleaving_path.iter(){
            for i in 1..path.len()-1{
                //We retrieve the labels

                if let PetriNode::Transition(t) = &path[i]{
                    regex = Regex::interleave(&regex,&t.label);

                    /*
                    if regex.is_empty(){ //To replace
                        regex.push_str(&format!("({})",t.label.to_string()));
                    }
                    else{
                        regex.push_str(&format!("&({})",t.label.to_string()));
                    }
                     */
                }

                //We delete the nodes in the path (except the extremities)
                self.remove_node(&path[i]);
            }
        }
        let t1 = &interleaving_path[0][0];
        let t2 = &interleaving_path[0][interleaving_path[0].len()-1];
        if let (PetriNode::Transition(tr1),PetriNode::Transition(tr2)) = (t1,t2){
            regex = Regex::concatenate_vec(vec![tr1.label.clone(),regex,tr2.label.clone()])
            /*
            if !tr1.label.is_empty(){
                regex = format!("({}).({})", tr1.label.to_string(), regex);
            }
            if !tr2.label.is_empty(){
                regex = format!("({}).({})",regex,tr2.label.to_string());
            }
             */
        }
        //We create the new transition
        let new_transition = PetriTransition::new_from_name(&regex);
        self.add_transition(&new_transition);
        //We add the edges
        let t1 = &interleaving_path[0][0];
        let t2 = &interleaving_path[0][interleaving_path[0].len()-1];

        let predecessors_of_t1 = self.graph.neighbors_directed(self.get_node_index(&t1).unwrap(), petgraph::Direction::Incoming).collect::<Vec<_>>();
        let successors_of_t2 = self.graph.neighbors_directed(self.get_node_index(&t2).unwrap(),petgraph::Direction::Outgoing).collect::<Vec<_>>();

        self.remove_node(&t1);
        self.remove_node(&t2);

        for prec_t1 in predecessors_of_t1.iter(){
            let prec_t1_as_node = self.graph[*prec_t1].clone();
            self.update_edge(&prec_t1_as_node,&PetriNode::Transition(new_transition.clone()));
        }
        for succ_t2 in successors_of_t2.iter(){
            let succ_t2_as_node = self.graph[*succ_t2].clone();
            self.update_edge(&PetriNode::Transition(new_transition.clone()),&succ_t2_as_node);
        }
    }



}