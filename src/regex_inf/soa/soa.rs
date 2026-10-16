use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use itertools::Itertools;
use petgraph::algo::{all_simple_paths, kosaraju_scc};
use petgraph::data::{Build, DataMap};
use petgraph::graph::{DiGraph, Graph, NodeIndex, EdgeReference, Node};
use petgraph::stable_graph::StableGraph;
use petgraph::prelude::EdgeRef;
use petgraph::visit::{IntoNodeReferences, Dfs, Bfs, Topo, Reversed, NodeCount};
use petgraph::{algo, Direction};
use petgraph::dot::{Config, Dot};
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::thread_rng;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

#[derive(Debug,Clone)]
pub struct SOA{
    pub soa: DiGraph<Regex<String>,()>,
}



impl SOA{

    pub fn new()->SOA{
        //Creates a new empty soa
        SOA{
            soa: DiGraph::<Regex<String>,()>::new()
        }
    }

    pub fn build_from_sample(sample: &Sample) -> SOA{//OK
        //Build a new SOA from the sample with the algorithm 2tssi
        let mut graph: DiGraph<Regex<String>,()> = DiGraph::new();

        //Source node
        let src = graph.add_node(Regex::SRC);

        //Other nodes
        for symbol in &sample.alphabet{
            graph.add_node(Regex::Literal(symbol.clone()));
        }
        //Sink Node
        let snk = graph.add_node(Regex::SNK);

        //Transitions from src

        let firsts = sample.first_symbols();

        for first in firsts.iter(){
            if !first.is_empty(){
                let first_node = Self::get_node_index_from_weight(&graph,Regex::Literal(first.clone())).unwrap();
                graph.update_edge(src,first_node,());
            }
            else {
                graph.update_edge(src,snk,());
            }

        }
        //Transitions into snk
        let lasts = sample.last_symbols();

        for last in lasts.iter(){
            if !last.is_empty(){
                let last_node = Self::get_node_index_from_weight(&graph,Regex::Literal(last.clone())).unwrap();
                graph.update_edge( last_node,snk,());
            }
            else {
                graph.update_edge(src,snk,());
            }

        }

        //Other transitions
        let two_grams = sample.get_2grams();

        for (a,b) in two_grams.iter(){
            let node_a = Self::get_node_index_from_weight(&graph,Regex::Literal(a.to_string())).unwrap();
            let node_b = Self::get_node_index_from_weight(&graph,Regex::Literal(b.to_string())).unwrap();
            graph.update_edge(node_a,node_b,());
        }

        //return value
        SOA{
            soa: graph,
        }

    }



    pub fn src(&self)->NodeIndex{ // OK
        //Returns the index of the src node
        Self::get_node_index_from_weight(&self.soa,Regex::SRC).unwrap()
    }
    pub fn snk(&self)->NodeIndex{ //OK
        //Returns the index of the snk node
       Self::get_node_index_from_weight(&self.soa,Regex::SNK).unwrap()
    }

    pub fn pred(&self,node: &NodeIndex)->Vec<NodeIndex>{ //OK
        //Returns the set of predecessors of a given node
        let predecessors: Vec<_> = self.soa.neighbors_directed(*node, Direction::Incoming).collect();

        let mut result = Vec::new();
        for element in predecessors{
            result.push(element);
        }
        result
    }
    pub fn succ(&self,node: &NodeIndex)->Vec<NodeIndex>{ //OK
        //Returns the set of successors of a given node
        let successors: Vec<_> = self.soa.neighbors_directed(*node, Direction::Outgoing).collect();

        let mut result = Vec::new();
        for element in successors{
            result.push(element);
        }
        result

    }





    pub fn nodes_with_only_src_as_predecessor(&self) -> Vec<NodeIndex>{ //OK
        // 'first' function in the paper of Li et al
        //Returns the set of nodes that only have SRC as predecessor
        let mut result = Vec::new();
        let src = self.src();

        for node in self.soa.node_indices() {
            // Check if the node has only the given node as its predecessor
            let predecessors: Vec<_> = self.soa.neighbors_directed(node, Direction::Incoming).collect();
            if predecessors.len() == 1 && predecessors[0] == src {
                result.push(node);
            }
        }

        result
    }

    pub fn from_automaton(automaton: &Automaton)->SOA{
        let mut soa_graph = DiGraph::<Regex<String>,()>::new();
        
        let src = soa_graph.add_node(Regex::SRC);
        let snk = soa_graph.add_node(Regex::SNK);
        
        let mut nodes = BTreeMap::new();
        
        for edge_idx in automaton.graph.edge_indices(){
            let label = automaton.graph.edge_weight(edge_idx).unwrap().clone();

            let regex_label = if label.eq("Epsilon") {
                Regex::Epsilon
            } else {
                Regex::Literal(label.clone())
            };
            
           let node =  soa_graph.add_node(regex_label.clone());
            nodes.insert(edge_idx,(node,regex_label));
            
            let (source, target) = automaton.graph.edge_endpoints(edge_idx).unwrap();
            soa_graph.update_edge(source,node,());
            soa_graph.update_edge(node,target,());
            
            if automaton.final_states.contains(&target){
                soa_graph.update_edge(node,snk,());
            }
            if automaton.initial_states.contains(&source){
                soa_graph.update_edge(src,node,());
            }
        }
        
        //Remaining edges
        for v in automaton.graph.edge_indices().combinations(2){
            let e1 = v[0];
            let e2 = v[1];
            
            
            let (source, target) = automaton.graph.edge_endpoints(e1).unwrap();
            let (source2, target2) = automaton.graph.edge_endpoints(e2).unwrap();
            
            if target == source2{
                let (node1,regex_label1) = nodes.get(&e1).unwrap();
                let (node2,regex_label2) = nodes.get(&e2).unwrap();
                
                soa_graph.update_edge(*node1,*node2,());
            }
            
            if target2 == source{
                let (node1,regex_label1) = nodes.get(&e1).unwrap();
                let (node2,regex_label2) = nodes.get(&e2).unwrap();
                
                soa_graph.update_edge(*node2,*node1,());
            }
        }
        
        
        SOA {soa: soa_graph}
    }

}