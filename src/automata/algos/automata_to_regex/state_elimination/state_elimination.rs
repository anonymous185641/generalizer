use std::collections::BTreeMap;
use petgraph::data::DataMap;
use petgraph::dot::Dot;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::regex::regex::Regex::Kleene;

impl Automaton{
    pub fn to_regex_with_state_elimination(&self) ->Result<Regex<String>,String>{

        let mut graph = self.state_removal_prep();
        //println!("graph: {:?}",Dot::with_config(&graph.clone(), &[]));

        while let Some(node) = Self::state_removal_pick_state_to_elim(&graph){
            Self::state_removal_eliminate_state(&mut graph,node);
        }

        //println!("graph: {:?}",Dot::with_config(&graph.clone(), &[]));

        if graph.node_count() > 2 {
           return Err("State Removal failed. More than 2 node at the end".to_string());
        }

        let mut regex = Regex::Empty;


        for edge in graph.edge_indices().into_iter(){
            let (src,tgt) = graph.edge_endpoints(edge).unwrap();
            let src_label = graph.node_weight(src).unwrap().clone();
            let tgt_label = graph.node_weight(tgt).unwrap().clone();

            if !src_label.eq(&"qSEInit".to_string()) || !tgt_label.eq(&"qSEFinal".to_string()){
                return Err("State Removal failed. At the end, there is an edge that is not from the src to the snk.".to_string());
            }

            let label = graph.edge_weight(edge).unwrap().clone();
            regex = Regex::or(&regex,&label);
        }

         Ok(regex)
    }


    pub fn state_removal_eliminate_state(graph: &mut StableGraph<String,Regex<String>>,state: NodeIndex){
        let mut state_pred = graph.neighbors_directed(state,petgraph::Direction::Incoming).collect::<Vec<_>>();
        let mut state_succ = graph.neighbors_directed(state,petgraph::Direction::Outgoing).collect::<Vec<_>>();

        state_pred.sort();
        state_succ.sort();
        state_pred.dedup();
        state_succ.dedup();

       // println!("state:  {:?}",state);
        for q_in in state_pred.iter(){
            if *q_in==state{
                continue;
            }
            for q_out in state_succ.iter(){
                if *q_out==state{
                    continue;
                }
                //println!("q_in:  {:?}  q_out {:?}",q_in,q_out);
               //Regex into state
                let lab_in = Self::state_removal_regex_between(q_in,&state,graph);
               // println!("LAB IN {:?}",lab_in);
                //Regex out of state
                let lab_out = Self::state_removal_regex_between(&state,q_out,graph);
               // println!("LAB OUT {:?}",lab_out);
                //Self loops at state

                let self_loop = Self::state_removal_regex_between(&state,&state,graph);
                //println!("SELF LOOP {:?}",self_loop);

                let new_label = match self_loop{
                    Regex::Empty=> Regex::concatenate_vec(vec![lab_in,lab_out]),
                    _ => {
                        let k_self_loop = Regex::kleene(&self_loop.clone());
                        Regex::concatenate_vec(vec![lab_in,k_self_loop,lab_out])
                    }
                };


                graph.add_edge(*q_in,*q_out,new_label);

            }
        }

        graph.remove_node(state);
    }
    pub fn state_removal_pick_state_to_elim(graph: &StableGraph<String,Regex<String>>)->Option<NodeIndex>{
        for graph_node in graph.node_indices(){
            let label = graph[graph_node].clone();
            if !label.eq(&"qSEInit".to_string())&& !label.eq(&"qSEFinal".to_string()){
                return Some(graph_node.clone());
            }
        }

        None
    }

    pub fn state_removal_prep(&self)->StableGraph<String,Regex<String>>{
        //Translating the graph
        let mut new_aut = StableGraph::new();
        let mut node_mapping = BTreeMap::new();
        
        for edge in self.graph.edge_indices(){
            let (old_src,old_tgt) = self.graph.edge_endpoints(edge).unwrap();
            
            let label = if self.graph.edge_weight(edge).unwrap().clone().eq(&"Epsilon".to_string()){
                Regex::Epsilon
            }else{
                Regex::Literal(self.graph.edge_weight(edge).unwrap().clone())
            };
            
            let new_src = node_mapping.entry(old_src).or_insert_with(|| new_aut.add_node(self.graph.node_weight(old_src).unwrap().clone())).clone();
            let new_tgt = node_mapping.entry(old_tgt).or_insert_with(|| new_aut.add_node(self.graph.node_weight(old_tgt).unwrap().clone())).clone();
            
            new_aut.add_edge(new_src,new_tgt,label);
        }

        //Adding the new initial and final states
        let new_q0 = new_aut.add_node("qSEInit".to_string());
        let new_qfinal = new_aut.add_node("qSEFinal".to_string());

        for q in self.initial_states.iter(){
            new_aut.add_edge(new_q0,node_mapping[q].clone(),Regex::Epsilon);
        }

        for q in self.final_states.iter(){
            new_aut.add_edge(node_mapping[q].clone(),new_qfinal.clone(),Regex::Epsilon);
        }
        
        new_aut
    }

    pub fn state_removal_regex_between(n1: &NodeIndex,n2: &NodeIndex,graph: &StableGraph<String,Regex<String>>)->Regex<String>{
        let mut regex = Regex::Empty;

        for edge in graph.edge_indices().into_iter(){

            let (src,tgt) = graph.edge_endpoints(edge).unwrap();

            if src == *n1 && tgt == *n2{

                let label = graph.edge_weight(edge).unwrap().clone();
                regex = Regex::or(&regex,&label);

            }
        }
        
        regex
    }
}







































