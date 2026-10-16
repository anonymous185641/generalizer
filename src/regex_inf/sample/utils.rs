use std::hash::Hash;
use itertools::Itertools;
use petgraph::data::DataMap;
use petgraph::Graph;
use petgraph::graph::{NodeIndex, UnGraph};
use crate::regex_inf::sample::sample::Sample;

impl Sample {
    pub fn undirected_subgraph(parent_graph: &UnGraph<String,()>,node: &NodeIndex, neighborhood: bool)->UnGraph<String,()>{ //OK
        let mut new_graph:UnGraph<String,()> = Graph::new_undirected();


        if neighborhood{

            let mut neighbors = parent_graph.neighbors_undirected(node.clone()).collect::<Vec<_>>();
            neighbors.sort();
            neighbors.dedup();
            //println!("{:?}",neighbors);

            for nod in neighbors.clone(){
                let _ = new_graph.add_node(parent_graph.node_weight(nod.clone()).unwrap().clone());
            }



            for i in 0..neighbors.len(){
                for j in 0..neighbors.len(){
                    if parent_graph.contains_edge(neighbors[i],neighbors[j]) && neighbors[i]!=neighbors[j] {
                        let node_i_new_id = Self::get_node_index_from_weight(&new_graph,&parent_graph.node_weight(neighbors[i]).unwrap()).unwrap();
                        let node_j_new_id = Self::get_node_index_from_weight(&new_graph,&parent_graph.node_weight(neighbors[j]).unwrap()).unwrap();
                        new_graph.update_edge(node_i_new_id,node_j_new_id,());
                    }
                }
            }

        }
        else {
            let complement_graph = Self::complement_undirected(parent_graph);

            let new_node_id = Sample::get_node_index_from_weight(&complement_graph,&parent_graph.node_weight(node.clone()).unwrap()).unwrap();
            let mut nneighbors = complement_graph.neighbors_undirected(new_node_id.clone()).collect::<Vec<_>>();
            nneighbors.sort();
            nneighbors.dedup();

            //println!("{:?}",nneighbors);

            for nod in nneighbors.clone(){
                let _ = new_graph.add_node(complement_graph.node_weight(nod.clone()).unwrap().clone());
            }

            for i in 0..nneighbors.len(){
                for j in 0..nneighbors.len(){

                    if !complement_graph.contains_edge(nneighbors[i],nneighbors[j]) && nneighbors[i]!=nneighbors[j]{
                        let node_i_new_id = Self::get_node_index_from_weight(&new_graph,&complement_graph.node_weight(nneighbors[i]).unwrap()).unwrap();
                        let node_j_new_id = Self::get_node_index_from_weight(&new_graph,&complement_graph.node_weight(nneighbors[j]).unwrap()).unwrap();
                        new_graph.update_edge(node_i_new_id,node_j_new_id,());
                    }
                }
            }
        }

        new_graph
    }


    pub fn complement_undirected(graph:&UnGraph<String,()>)->UnGraph<String,()>{ //OK
        let mut new_graph:UnGraph<String,()> = Graph::new_undirected();

        petgraph::operator::complement(&graph,&mut new_graph,());

        new_graph
    }

    pub fn larger_set_vec<T: Clone>(a: &Vec<T>,b: &Vec<T>)->Vec<T>{ // OK
        if a.len() > b.len(){
            a.clone()
        }
        else {
            b.clone()
        }
    }

    pub fn new_element_vec<T: Hash+Clone+Eq+PartialEq>(set: &Vec<T>,node: &T)->Vec<T>{ // OK
        let mut ret = set.clone();

        ret.push(node.clone());
        ret
    }
    pub fn get_node_index_from_weight(graph: &UnGraph<String,()>, weight: &String) -> Option<NodeIndex> { //OK
        for node_index in graph.node_indices() {
            if graph[node_index] == *weight {
                return Some(node_index);
            }
        }
        None
    }

    pub fn max_word_length(&self) ->usize{
        let mut ret :usize = 0;

        if self.separator==None{
            for word in &self.sample{
                if word.len()>ret{
                    ret = word.len();
                }
            }
        }
        else {
            for word in &self.sample{
                let mut separated_iterator = word.split_terminator(self.separator.unwrap()).collect_vec();
                if separated_iterator.len()>ret{
                    ret = separated_iterator.len();
                }
            }
        }

        ret
    }
}