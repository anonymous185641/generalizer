use std::any::Any;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use petgraph::graph::Node;
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use crate::petri_nets::petri_net::place_transition::{PetriNetTransitionLabelBound, PetriPlace, PetriTransition};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlaceNumberToken(pub usize);



#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PetriNode<T: PetriNetTransitionLabelBound>{
    Place(PetriPlace),
    Transition(PetriTransition<T>)
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Marking{
    pub tokens: BTreeMap<PetriPlace, PlaceNumberToken>
}

#[derive(Debug, Clone)]
pub struct PetriNet<T:PetriNetTransitionLabelBound> {
    pub places: Vec<PetriPlace>,
    pub transitions: Vec<PetriTransition<T>>,
    pub graph: StableGraph<PetriNode<T>, ()>,
    pub initial_marking: Marking,
}

impl Marking{
    pub fn new() -> Self{
        Self{
            tokens: BTreeMap::new()
        }
    }

    pub fn from_map(tokens: BTreeMap<PetriPlace, PlaceNumberToken>) -> Self{
        Self{
            tokens
        }
    }
}


impl<T: PetriNetTransitionLabelBound> PetriNet<T>{
    pub fn new() -> Self{
        Self{
            places: Vec::new(),
            transitions: Vec::new(),
            graph: StableGraph::new(),
            initial_marking: Marking::new(),
        }
    }
    pub fn size(&self) -> usize{
        self.places.len() + self.transitions.len()
    }
    pub fn is_safe(&self, marking: &Marking) -> bool{
        todo!()
    }
    
    pub fn get_place_from_label(&self,label:&String) -> Option<PetriPlace>{
        for node_id in self.graph.node_indices(){
            match &self.graph[node_id]{
                PetriNode::Place(p)=>{
                    if p.label ==*label{
                        return Some(p.clone());
                    }
                },
                PetriNode::Transition(_)=>{
                }
            }
        }
        None
    }
    pub fn get_transition_from_label(&self,label:&T) -> Option<PetriTransition<T>>{
        for node_id in self.graph.node_indices(){
            match &self.graph[node_id]{
                PetriNode::Place(_)=>{
                },
                PetriNode::Transition(t)=>{
                    if t.label ==*label{
                        return Some(t.clone());
                    }
                }
            }
        }
        None
    }
    
    
    pub fn add_place(&mut self, place: &PetriPlace){
        self.places.push(place.clone());
        self.graph.add_node(PetriNode::Place(place.clone()));
    }
    pub fn add_transition(&mut self, transition: &PetriTransition<T>){
        self.transitions.push(transition.clone());
        self.graph.add_node(PetriNode::Transition(transition.clone()));
    }

    pub fn get_or_add_place_from_label(&mut self, label: &String) ->PetriPlace{
        match self.get_place_from_label(label){
            Some(p)=>p,
            None=>{
                let p = PetriPlace::new_from_name(label);
                self.add_place(&p);
                p
            }
        }
    }
    pub fn get_or_add_transition_from_label(&mut self, label:&T) -> PetriTransition<T>{
        match self.get_transition_from_label(label){
            Some(t)=>t,
            None=>{
                let t = PetriTransition::new_from_name(label);
                self.add_transition(&t);
                t
            }
        }
    }

    pub fn get_node_index(&self, node: &PetriNode<T>)->Option<NodeIndex>{
        for node_id in self.graph.node_indices(){
            if self.graph[node_id] == node.clone(){
                return Some(node_id);
            }
        }
        None
    }

    pub fn get_place_between_from_labels(&self,label1:&T,label2:&T)->Option<PetriPlace>{
        //We assume that there is a unipe place between label1 and label2

        match (self.get_transition_from_label(label1),self.get_transition_from_label(label2)){
            (Some(t1),Some(t2))=>{
                //search t1 -> p -> t2

                //We retrieve the outgoing neighbors of t1
                let t1_neightbors = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t1.clone())).unwrap(),petgraph::Direction::Outgoing);

                //We retrieve the incoming neighbors of t2
                let t2_neightbors = self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(t2.clone())).unwrap(),petgraph::Direction::Incoming);

                for t1_neighbor in t1_neightbors{
                   for t2_neighbor in t2_neightbors.clone(){
                       if t1_neighbor == t2_neighbor{
                           if let PetriNode::Place(p) = self.graph[t2_neighbor].clone(){
                               return Some(p);
                           }
                       }
                   }
                }
                None
            },
            _ => None
        }
    }
    
    // To do: improve the error handling
    pub fn update_edge(&mut self,source: &PetriNode<T>, target: &PetriNode<T>){
        match (self.get_node_index(source),self.get_node_index(target)){
            (Some(source_index),Some(target_index))=>{
                let edge_id = self.graph.update_edge(source_index,target_index,());
            },
            _ => {
                panic!("The source or the target node is not in the graph.\
                To do:improve the error handling");
            }
        }        
    }
    pub fn remove_edge(&mut self,source: &PetriNode<T>,target: &PetriNode<T>){
        match (self.get_node_index(source),self.get_node_index(target)){
            (Some(source_index),Some(target_index))=>{
                let edge_id = self.graph.find_edge(source_index,target_index);
                if let Some(e) = edge_id{
                    self.graph.remove_edge(e);
                }
            },
            _ => {
                panic!("The source or the target node is not in the graph.\
                To do:improve the error handling");
            }
        }
    }

    pub fn remove_node(&mut self, node: &PetriNode<T>){
        match self.get_node_index(node){
            Some(node_index)=>{
                self.graph.remove_node(node_index);
                match node{
                    PetriNode::Place(p)=>self.places.retain(|x| *x != p.clone()),
                    PetriNode::Transition(t)=>self.transitions.retain(|x| *x != t.clone())
                }
            },
            _ => { }
        }
    }
    
    
    pub fn preset_generic(&self, node: &PetriNode<T>) ->BTreeSet<PetriNode<T>>{
        let mut incoming_neighbors = BTreeSet::new();
        for neighbor in self.graph.neighbors_directed(self.get_node_index(node).unwrap(),petgraph::Direction::Incoming){
            incoming_neighbors.insert(self.graph[neighbor].clone());
        }
        incoming_neighbors
    }
    
    pub fn postset_generic(&self, node: &PetriNode<T>) ->BTreeSet<PetriNode<T>>{
        let mut outgoing_neighbors = BTreeSet::new();
        for neighbor in self.graph.neighbors_directed(self.get_node_index(node).unwrap(),petgraph::Direction::Outgoing){
            outgoing_neighbors.insert(self.graph[neighbor].clone());
        }
        outgoing_neighbors
    }
    
    pub fn preset_transition(&self,node: &PetriTransition<T>) ->BTreeSet<PetriPlace>{
        let mut incoming_neighbors = BTreeSet::new();
        for neighbor in self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(node.clone())).unwrap(),petgraph::Direction::Incoming){
            if let PetriNode::Place(p) = self.graph[neighbor].clone(){
                incoming_neighbors.insert(p);
            }
        }
        incoming_neighbors
    }
    
    pub fn postset_transition(&self,node: &PetriTransition<T>) ->BTreeSet<PetriPlace>{
        let mut outgoing_neighbors = BTreeSet::new();
        for neighbor in self.graph.neighbors_directed(self.get_node_index(&PetriNode::Transition(node.clone())).unwrap(),petgraph::Direction::Outgoing){
            if let PetriNode::Place(p) = self.graph[neighbor].clone(){
                outgoing_neighbors.insert(p);
            }
        }
        outgoing_neighbors
    }

    pub fn preset_place(&self,node: &PetriPlace) ->BTreeSet<PetriTransition<T>>{
        let mut incoming_neighbors = BTreeSet::new();
        for neighbor in self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(node.clone())).unwrap(),petgraph::Direction::Incoming){
            if let PetriNode::Transition(t) = self.graph[neighbor].clone(){
                incoming_neighbors.insert(t);
            }
        }
        incoming_neighbors
    }

    pub fn postset_place(&self,node: &PetriPlace) ->BTreeSet<PetriTransition<T>>{
        let mut outgoing_neighbors = BTreeSet::new();
        for neighbor in self.graph.neighbors_directed(self.get_node_index(&PetriNode::Place(node.clone())).unwrap(),petgraph::Direction::Outgoing){
            if let PetriNode::Transition(t) = self.graph[neighbor].clone(){
                outgoing_neighbors.insert(t);
            }
        }
        outgoing_neighbors
    }
    
}

impl<T:PetriNetTransitionLabelBound> PartialEq for PetriNet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.places == other.places
            && self.transitions == other.transitions
            && self.initial_marking == other.initial_marking
            && self.graph.node_count() == other.graph.node_count()
            && self.graph.edge_count() == other.graph.edge_count()
            && self.graph.node_indices().all(|idx| {
            self.graph.node_weight(idx) == other.graph.node_weight(idx)
        })
    }
}

impl<T: PetriNetTransitionLabelBound> Eq for PetriNet<T> {}


impl<T: PetriNetTransitionLabelBound> Hash for PetriNet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.places.hash(state);
        self.transitions.hash(state);
        self.initial_marking.hash(state);

        // Hash node weights
        self.graph.node_count().hash(state);
        for idx in self.graph.node_indices() {
            idx.index().hash(state);
            self.graph.node_weight(idx).unwrap().hash(state);
        }

        // Hash graph structure (ignore edge weights)
        self.graph.edge_count().hash(state);
        for edge in self.graph.edge_indices() {
            let (source, target) = self.graph.edge_endpoints(edge).unwrap();
            source.index().hash(state);
            target.index().hash(state);
        }
    }
}