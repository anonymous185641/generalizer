use std::collections::HashSet;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::soa::soa::SOA;

impl SOA{
    pub fn extract(&self, nodes_to_extract: &Vec<NodeIndex>)-> SOA {
        /*Build a new SOA from the given set of nodes;
            The edges coming fom outside the component is made coming from a new source,
            the edges going outside the component is made going into a new sink
        */
        let mut graph: DiGraph<Regex<String>,()> = DiGraph::new();

        //We add a new source and sink

        let src = graph.add_node(Regex::SRC);
        let snk = graph.add_node(Regex::SNK);


        //We add the other nodes

        for node in nodes_to_extract{
            let node_weight = self.soa.node_weight(*node).unwrap();
            graph.add_node(node_weight.clone());
        }
        //Handling incoming Edges
        for node in nodes_to_extract{
            let new_index_of_node = Self::get_node_index_from_weight(&graph,self.soa.node_weight(*node).unwrap().clone()).unwrap();
            for edge_index in self.soa.clone().edges_directed(*node, petgraph::Direction::Incoming) {
                let source = self.soa.clone().edge_endpoints(edge_index.id()).unwrap().0;
                /*
                 match Self::get_node_index_from_weight(&graph,self.soa.node_weight(source).unwrap().clone()){
                    Some(new_index_of_source) =>{ if new_index_of_source!= new_index_of_node {
                        graph.update_edge(new_index_of_source,new_index_of_node,());
                    }},
                    None =>{ graph.update_edge(src,new_index_of_node,());},
                };

                 */
                match Self::get_node_index_from_weight(&graph,self.soa.node_weight(source).unwrap().clone()){
                    Some(new_index_of_source) => graph.update_edge(new_index_of_source,new_index_of_node,()),
                    None =>graph.update_edge(src,new_index_of_node,()),
                };
            }
        }
        //Handling outgoing edges
        for node in nodes_to_extract{
            let new_index_of_node = Self::get_node_index_from_weight(&graph,self.soa.node_weight(*node).unwrap().clone()).unwrap();
            for edge_index in self.soa.clone().edges_directed(*node, petgraph::Direction::Outgoing) {
                let target = self.soa.clone().edge_endpoints(edge_index.id()).unwrap().1;
                /*
                 match Self::get_node_index_from_weight(&graph,self.soa.node_weight(target).unwrap().clone()){
                    Some(new_index_of_target) => {if new_index_of_node!= new_index_of_target {
                        graph.update_edge(new_index_of_node,new_index_of_target,());
                    }},
                    None => {graph.update_edge(new_index_of_node,snk,());},
                };

                 */
                match Self::get_node_index_from_weight(&graph,self.soa.node_weight(target).unwrap().clone()){
                    Some(new_index_of_target) => graph.update_edge(new_index_of_node,new_index_of_target,()),
                    None => graph.update_edge(new_index_of_node,snk,()),
                };


            }
        }
        let mut result =SOA{
            soa: graph,
        };

        result
    }
}