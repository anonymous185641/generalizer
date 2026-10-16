
use graphviz_dot_builder::traits::DotBuildable;
use petgraph::graph::NodeIndex;
use petgraph::prelude::{EdgeRef, StableGraph};
use petgraph::stable_graph::EdgeIndex;

#[derive(Debug,Clone,Eq, PartialEq)]
pub enum AutomatonType{
    DFA,
    NFA,
    UFA,
    NDEF // Not defined
}
#[derive(Debug,Clone)]
pub struct Automaton{
    pub initial_states: Vec<NodeIndex>,
    pub final_states: Vec<NodeIndex>,
    pub graph: StableGraph<String,String>,
    //pub aut_type: AutomatonType,
}


impl Automaton {
    pub fn new()->Automaton{
        Automaton{
            initial_states: Vec::new(),
            final_states: Vec::new(),
            graph: StableGraph::new(),
            //aut_type: AutomatonType::NDEF
        }
    }
    
    pub fn new_from_fields(graph: StableGraph<String,String>,initial_states: Vec<NodeIndex>,final_states: Vec<NodeIndex>,aut_type: AutomatonType)->Self{
        Automaton{initial_states,final_states,graph/*,aut_type*/}
    }
    
    pub fn get_or_create_node(&mut self, label:&String) ->NodeIndex{
        match self.get_node(label){
            Some(node_id) => node_id.clone(),
            None => self.graph.add_node(label.clone()),
        }
    }
    pub fn get_node(&self,label:&String)->Option<NodeIndex>{
        for node_index in self.graph.node_indices() {
            if self.graph[node_index] == *label {
                return Some(node_index);
            }
        }
        None
    }
    
    pub fn create_edge_between(&mut self, node1: &NodeIndex, node2: &NodeIndex, label:&String) ->EdgeIndex{
        //We just make sure the same edge is not created twice.
        for edges in self.graph.edges_connecting(node1.clone(),node2.clone()) {
            if edges.weight() == label {
                return edges.id();
            }
        }
        self.graph.add_edge(node1.clone(),node2.clone(),label.clone())
    }
    pub fn get_node_weight(&self,node_id: &NodeIndex)->Option<String>{
        for node_index in self.graph.node_indices() {
            if  node_index== *node_id {
                return Some(self.graph[node_index].clone());
            }
        }
        None
    }



    pub fn get_node_by_weight<S: Eq+PartialEq,T>(graph: &StableGraph<S,T>,weight: &S)-> Option<NodeIndex>{
        for node in graph.node_indices(){
            if graph.node_weight(node).unwrap().clone()==weight.clone(){
                return Some(node);
            }
        }
        None
    }
    pub fn to_soa(&self){
        todo!();
    }
    
    pub fn size(&self)->usize{
        self.graph.node_count()
    }
}