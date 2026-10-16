use std::collections::HashSet;
use petgraph::algo::{all_simple_paths, kosaraju_scc};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::Dfs;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::soa::soa::SOA;

impl SOA{




    pub fn reach(&self, node: &NodeIndex)-> Vec<NodeIndex>{ //OK
        //Return the set of nodes reachable from the given node
        //Don't know...
        let mut result : Vec<NodeIndex> = Vec::new();

        let mut dfs = Dfs::new(&self.soa, *node);

        // Perform DFS traversal
        while let Some(nod) = dfs.next(&self.soa) {
            if nod == *node {
                if self.soa.contains_edge(nod,nod) {
                    result.push(nod);
                }
            }
            else{
                result.push(nod);
            }

        }
        result.dedup();
        result
    }



    pub fn get_node_index_from_weight(graph: &DiGraph<Regex<String>,()>, weight: Regex<String>) -> Option<NodeIndex> { //OK
        for node_index in graph.node_indices() {
            if graph[node_index] == weight {
                return Some(node_index);
            }
        }
        None
    }

    pub fn is_successor(&self,pred: &NodeIndex, succ: &NodeIndex)->bool{ //Ok
        self.succ(pred).contains(succ)
    }

    pub fn has_cycle(&self)->bool{
        let components = kosaraju_scc(&self.soa);

        for component in components{
            if component.len() > 1 {
                return true;
            }
            if let Some(e) = self.soa.find_edge(component[0],component[0]){
                return true;
            }
        }
        false
    }


    pub fn intersection(v1: &Vec<NodeIndex>, v2: &Vec<NodeIndex>) -> Vec<NodeIndex>{
        let mut result = Vec::new();

        for i in v1.iter(){
            for j in v2.iter(){
                if i == j{
                    result.push(*i);
                }
            }
        }

        result.dedup();
        result
    }


    pub fn are_vectors_equals(v1: &Vec<NodeIndex>, v2: &Vec<NodeIndex>) -> bool{
        let v1_set:HashSet<NodeIndex> = HashSet::from_iter(v1.iter().cloned());
        let v2_set:HashSet<NodeIndex> = HashSet::from_iter(v2.iter().cloned());

        v1_set == v2_set
    }
}