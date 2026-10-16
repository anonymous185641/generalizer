use std::collections::HashSet;
use std::sync::atomic::AtomicI8;
use itertools::Itertools;
use petgraph::algo;
use petgraph::data::DataMap;

use petgraph::graph::{DiGraph, NodeIndex};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::automata::automaton::automaton::{Automaton, AutomatonType};
use crate::regex_inf::sample::sample::Sample;

pub struct StateMerging {}

impl StateMerging {

    pub fn mca(sample: &Sample,rgn: &mut ThreadRng) -> Automaton{ //OKKKKK
        if sample.sample.is_empty(){
            return Automaton::new();
        }

        let mut output_automaton = Automaton::new();


        for word in &sample.sample{
            if sample.separator == None {
                /*If there are no separator, we dislocate the string into chars
                 and add each char as alphabet symbols*/
                let mut char_iterator = word.chars().collect_vec();
                let size = char_iterator.len();

                if size==1 {
                    let node = output_automaton.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
                    output_automaton.initial_states.push(node);
                    output_automaton.final_states.push(node);
                }
                else if size > 1{
                    let mut node_vec = Vec::new();
                    //Nodes and Edges
                    let mut initial_node = output_automaton.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
                    output_automaton.initial_states.push(initial_node);
                    node_vec.push(initial_node);

                    let mut ct =0;
                    for letter in char_iterator{

                        ////////////////
                        let node = output_automaton.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
                        node_vec.push(node);
                        output_automaton.graph.update_edge(node_vec[ct],node_vec[ct+1],letter.to_string());

                        if ct==size-1{
                            output_automaton.final_states.push(node);
                        }
                        ct+=1;
                    }
                }

            }
            else {
                /*If there is a separator, we split the strings according the separator
                 and then add the obtained strings as alphabet symbols*/
                let mut separated = word.split_terminator(sample.separator.unwrap()).collect_vec();

                let size = separated.len();

                if size==1 {
                    let node = output_automaton.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
                    output_automaton.initial_states.push(node);
                    output_automaton.final_states.push(node);
                }
                else if size > 1{
                    let mut node_vec = Vec::new();
                    //Nodes and Edges
                    let mut initial_node = output_automaton.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
                    output_automaton.initial_states.push(initial_node);
                    node_vec.push(initial_node);

                    let mut ct =0;
                    for letter in separated{
                        let node = output_automaton.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
                        node_vec.push(node);
                        output_automaton.graph.update_edge(node_vec[ct],node_vec[ct+1],letter.to_string());

                        if ct==size-1{
                            output_automaton.final_states.push(node);
                        }
                        ct+=1;


                    }
                }


            }
        }



        output_automaton

    }
    pub fn merge(aut: &Automaton,q1: &NodeIndex,q2: &NodeIndex,rgn: &mut ThreadRng)->Automaton{ //OKKK
        let mut new_aut = Automaton::new();
        let mut nodes_to_merge = HashSet::from([q1.clone(),q2.clone()]);
        //NODES
        for node in aut.graph.node_indices(){
            if node!=*q1 && node!=*q2{
                new_aut.graph.add_node(aut.graph.node_weight(node.clone()).unwrap().clone());

            }
        }

        let fused_state = new_aut.graph.add_node(format!("q{}",rgn.gen_range(0..10000)));
        //EDGES

        for edge in aut.graph.edge_indices(){
            if let Some((source,target)) = aut.graph.clone().edge_endpoints(edge){
                if nodes_to_merge.contains(&source) && nodes_to_merge.contains(&target){
                    continue;
                }
                else if !nodes_to_merge.contains(&source) && nodes_to_merge.contains(&target) {
                    //let new_source_id = Self::get_node_index_from_weight(&new_graph,self.soa.node_weight(source).unwrap().clone()).unwrap();
                    let new_source_id = new_aut.get_node(aut.graph.node_weight(source).unwrap()).unwrap();
                    new_aut.graph.update_edge(new_source_id,fused_state,aut.graph.edge_weight(aut.graph.find_edge(source,target).unwrap()).unwrap().clone());
                }
                else if nodes_to_merge.contains(&source) && !nodes_to_merge.contains(&target){
                    //let new_target_id = Self::get_node_index_from_weight(&new_graph,self.soa.node_weight(target).unwrap().clone()).unwrap();
                    let new_target_id = new_aut.get_node(aut.graph.node_weight(target).unwrap()).unwrap();
                    new_aut.graph.update_edge(fused_state,new_target_id,aut.graph.edge_weight(aut.graph.find_edge(source,target).unwrap()).unwrap().clone());
                }
                else{
                    let new_source_id = new_aut.get_node(aut.graph.node_weight(source).unwrap()).unwrap();
                    let new_target_id = new_aut.get_node(aut.graph.node_weight(target).unwrap()).unwrap();
                    new_aut.graph.update_edge(new_source_id,new_target_id,aut.graph.edge_weight(aut.graph.find_edge(source,target).unwrap()).unwrap().clone());

                }
            }
        }

        for node in &aut.initial_states{
            if *node==*q1 || *node==*q2{
                new_aut.initial_states.push(fused_state);
            }
            else{
                new_aut.initial_states.push(new_aut.get_node(aut.graph.node_weight(node.clone()).unwrap()).unwrap());
            }
        }

        for node in &aut.final_states{
            if *node==*q1 || *node==*q2{
                new_aut.final_states.push(fused_state);
            }
            else{
                new_aut.final_states.push(new_aut.get_node(aut.graph.node_weight(node.clone()).unwrap()).unwrap());
            }
        }

        new_aut
    }

    pub fn determinize(aut: &Automaton,rgn: &mut ThreadRng) -> Automaton{
        let mut new_aut = Automaton::new();

        new_aut = aut.clone();

        while let Some((q1,q2)) = Self::pick_2_nodes_in_common_prefix_relation(&new_aut){
            new_aut = Self::merge(&new_aut,&q1,&q2,rgn);
        }
        //new_aut.aut_type = AutomatonType::DFA;
        new_aut
    }
    pub fn disambiguate(aut: &Automaton,rgn: &mut ThreadRng) -> Automaton{
        let mut new_aut = Automaton::new();

        new_aut = aut.clone();

        while let Some((q1,q2)) = Self::pick_2_nodes_in_parallel_acceptance_relation(&new_aut){
            new_aut = Self::merge(&new_aut,&q1,&q2,rgn);
        }
        //new_aut.aut_type = AutomatonType::UFA;
        new_aut
    }

    pub fn state_merging(sample: &Sample, typ: AutomatonType) -> Automaton{
        let mut rgn = rand::thread_rng();// This is for the randomization of nodes names
        let aut = Self::mca(sample,&mut rgn);

        let mut new_aut = Automaton::new();
        if typ==AutomatonType::DFA{
            new_aut = Self::determinize(&aut,&mut rgn);
        }
        else if typ == AutomatonType::UFA{
            new_aut = Self::disambiguate(&aut,&mut rgn);

        }

        new_aut
    }


    pub fn prefix_language(aut:&Automaton,node: &NodeIndex)->HashSet<Vec<String>>{

        let mut prefix_words = HashSet::new();

        //All the paths toward node
        let mut paths_to_node = Vec::new();
        for initial_state in &aut.initial_states{
            for path in algo::all_simple_paths::<Vec<_>, _>(&aut.graph, initial_state.clone(), node.clone(), 0, None).collect::<Vec<_>>(){
                paths_to_node.push(path);
            }
        }

        //Words
        for path in paths_to_node{
            if path.len()<=1{
                continue;
            }
            let mut temp_vec = Vec::new();
            for i in 0..path.clone().len()-1{
                temp_vec.push(aut.graph.edge_weight(aut.graph.find_edge(path[i],path[i+1]).unwrap()).unwrap().clone()); //????
            }
            prefix_words.insert(temp_vec);
        }

        prefix_words
    }

    pub fn suffix_language(aut:&Automaton,node: &NodeIndex)->HashSet<Vec<String>>{
        let mut suffix_words = HashSet::new();

        //All the paths toward node
        let mut paths_to_node = Vec::new();
        for final_state in &aut.final_states{
            for path in algo::all_simple_paths::<Vec<_>, _>(&aut.graph,node.clone(),final_state.clone(), 0, None).collect::<Vec<_>>(){
                paths_to_node.push(path);
            }
        }

        //Words
        for path in paths_to_node{
            if path.len()<=1{
                continue;
            }
            let mut temp_vec = Vec::new();
            for i in 0..path.clone().len()-1{
                temp_vec.push(aut.graph.edge_weight(aut.graph.find_edge(path[i],path[i+1]).unwrap()).unwrap().clone()); //????
            }
            suffix_words.insert(temp_vec);
        }

        suffix_words
    }
    pub fn are_in_common_prefix_rel(aut: &Automaton,q1: &NodeIndex,q2: &NodeIndex)->bool{
        let prefix_language_of_q1 = Self::prefix_language(aut,q1);
        let prefix_language_of_q2 = Self::prefix_language(aut,q2);

        prefix_language_of_q1.intersection(&prefix_language_of_q2).count()>0
    }
    pub fn are_in_common_suffix_rel(aut: &Automaton,q1: &NodeIndex,q2: &NodeIndex)->bool{
        let suffix_language_of_q1 = Self::suffix_language(aut,q1);
        let suffix_language_of_q2 = Self::suffix_language(aut,q2);

        suffix_language_of_q1.intersection(&suffix_language_of_q2).count()>0
    }

    pub fn are_in_parallel_acceptance_rel(aut: &Automaton,q1: &NodeIndex,q2: &NodeIndex)->bool{
        Self::are_in_common_prefix_rel(aut,q1,q2) && Self::are_in_common_suffix_rel(aut,q1,q2)
    }

    pub fn pick_2_nodes_in_common_prefix_relation(aut: &Automaton)->Option<(NodeIndex,NodeIndex)>{
        let nodes_indices = aut.graph.node_indices().collect_vec();

        if aut.initial_states.len()>=2{
            let initial_state_vec = aut.initial_states.iter().collect_vec();
            return Some((initial_state_vec[0].clone(),initial_state_vec[1].clone()));
        }


        for i in 0..nodes_indices.len(){
            for j in 0..nodes_indices.len(){
                if Self::are_in_common_prefix_rel(aut,&nodes_indices[i],&nodes_indices[j]) && i!=j{
                    return Some((nodes_indices[i].clone(),nodes_indices[j].clone()));
                }
            }
        }
        None
    }

    pub fn pick_2_nodes_in_common_suffix_relation(aut: &Automaton)->Option<(NodeIndex,NodeIndex)>{
        let nodes_indices = aut.graph.node_indices().collect_vec();

        for i in 0..nodes_indices.len(){
            for j in 0..nodes_indices.len(){
                if Self::are_in_common_suffix_rel(aut,&nodes_indices[i],&nodes_indices[j]) && i!=j{
                    return Some((nodes_indices[i].clone(),nodes_indices[j].clone()));
                }
            }
        }
        None
    }

    pub fn pick_2_nodes_in_parallel_acceptance_relation(aut: &Automaton)->Option<(NodeIndex,NodeIndex)>{
        let nodes_indices = aut.graph.node_indices().collect_vec();

        for i in 0..nodes_indices.len(){
            for j in 0..nodes_indices.len(){
                if Self::are_in_parallel_acceptance_rel(aut,&nodes_indices[i],&nodes_indices[j]) && i!=j{
                    return Some((nodes_indices[i].clone(),nodes_indices[j].clone()));
                }
            }
        }
        None
    }


}