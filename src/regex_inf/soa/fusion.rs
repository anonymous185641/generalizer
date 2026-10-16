use std::collections::HashSet;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::{DiGraph, NodeIndex};
use rand::prelude::IndexedRandom;
use rand::thread_rng;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;
use crate::regex_inf::soa::soa::SOA;

impl SOA{
    pub fn fuse_nodes(&mut self, nodes_to_merge: &Vec<NodeIndex>, fused_node_weight: Regex<String>)->NodeIndex{
        let mut new_graph: DiGraph<Regex<String>,()>= DiGraph::new();

        //NODES
        for node in self.soa.node_indices(){
            if !nodes_to_merge.contains(&node){
                new_graph.add_node(self.soa.node_weight(node.clone()).unwrap().clone());

            }
        }

        let fused_node = new_graph.add_node(fused_node_weight);

        //EDGES

        for edge in self.soa.edge_indices(){
            if let Some((source,target)) = self.soa.clone().edge_endpoints(edge){
                if nodes_to_merge.contains(&source) && nodes_to_merge.contains(&target){
                    continue;
                }
                else if !nodes_to_merge.contains(&source) && nodes_to_merge.contains(&target) {
                    let new_source_id = Self::get_node_index_from_weight(&new_graph,self.soa.node_weight(source).unwrap().clone()).unwrap();
                    new_graph.update_edge(new_source_id,fused_node,());
                }
                else if nodes_to_merge.contains(&source) && !nodes_to_merge.contains(&target){
                    let new_target_id = Self::get_node_index_from_weight(&new_graph,self.soa.node_weight(target).unwrap().clone()).unwrap();
                    new_graph.update_edge(fused_node,new_target_id,());
                }
                else{
                    let new_source_id = Self::get_node_index_from_weight(&new_graph,self.soa.node_weight(source).unwrap().clone()).unwrap();
                    let new_target_id = Self::get_node_index_from_weight(&new_graph,self.soa.node_weight(target).unwrap().clone()).unwrap();
                    new_graph.update_edge(new_source_id,new_target_id,());

                }
            }
        }

        //WRAP
        self.soa = new_graph.clone();
        fused_node
    }


    pub fn contract_with_alt_old(&mut self) -> SOA{ // For soa2chare
        let strongly_connected_components = kosaraju_scc(&self.soa);



        for component in strongly_connected_components{
            if component.len()==1 && self.soa.contains_edge(component[0],component[0]){
                self.fuse_nodes(&component,Regex::plus(self.soa.node_weight(component[0]).unwrap()));
            }
            else if component.contains(&self.src()) || component.contains(&self.snk()){
                continue;
            }
            else if component.len()>1{

                // New label with alt
                let mut new_label = Regex::Empty;
                for node in component.clone(){
                    new_label =  Regex::or(&new_label,self.soa.node_weight(node).unwrap());
                }
                new_label = Regex::Plus(Box::new(new_label.clone()));
                //Fusion
                self.fuse_nodes(&component,new_label);
            }

        }

        self.clone()
    }

    pub fn contract_with_alt(&mut self) -> SOA{ // NO

        while let Some(component) = self.valid_stcc() {
            if component.len() == 1 && self.soa.contains_edge(component[0], component[0]) {
                self.fuse_nodes(&component, Regex::plus(self.soa.node_weight(component[0]).unwrap()));
            } else if component.contains(&self.src()) || component.contains(&self.snk()) {
                continue;
            } else if component.len() > 1 {
                // New label with alt
                let mut new_label = Regex::Empty;
                for node in component.clone(){
                    new_label =  Regex::or(&new_label,self.soa.node_weight(node).unwrap());
                }
                new_label = Regex::Plus(Box::new(new_label.clone()));
                //Fusion
                self.fuse_nodes(&component,new_label);
            }
        }

        self.clone()
    }
    pub fn contract_with_repair(&mut self, given_sample: &Sample) -> SOA{ // NO

        while let Some(component) = self.valid_stcc() {
            if component.len() == 1 && self.soa.contains_edge(component[0], component[0]) {
                self.fuse_nodes(&component, Regex::plus(self.soa.node_weight(component[0]).unwrap()));
            } else if component.contains(&self.src()) || component.contains(&self.snk()) {
                continue;
            } else if component.len() > 1 {
                let mut label_component = Vec::new();
                for nod in component.clone() {
                    if let Some(Regex::Literal(s)) = self.soa.node_weight(nod) {
                        label_component.push(s.clone());
                    } else {
                        panic!("Non literal weight in gen_ichare");
                    }
                }

                let new_sample = given_sample.filter(label_component.clone());
                self.fuse_nodes(&component, Sample::repair(label_component, &new_sample));
            }
        }

        self.clone()
    }

    pub fn valid_stcc(&self)->Option<Vec<NodeIndex>>{
        let mut strongly_connected_components = kosaraju_scc(&self.soa);
       // strongly_connected_components.sort();

        self.pick_a_valid_stcc(strongly_connected_components)
    }
    pub fn pick_a_valid_stcc(&self, components: Vec<Vec<NodeIndex>>)-> Option<Vec<NodeIndex>>{
        //let mut stcc = Vec::new();

        for component in components{
            if (component.len()>1) || (component.len()==1 && self.soa.contains_edge(component[0],component[0])){
                //stcc.push(component);
                return Some(component);
            }
        }
       None
    }
}