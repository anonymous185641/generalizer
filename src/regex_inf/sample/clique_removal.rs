use std::collections::HashMap;
use petgraph::graph::{NodeIndex, UnGraph};
use crate::regex_inf::sample::sample::Sample;


impl Sample {
    pub fn clique_removal(un_graph: &UnGraph<String,()>)->(Vec<String>,Vec<Vec<String>>){//OK
        let mut graph  = un_graph.clone();

        let mut cliques = Vec::new();
        let mut isets = Vec::new();

        let (mut c_i,mut i_i) = Self::ramsey(&graph);

        cliques.push(c_i.clone());
        isets.push(i_i.clone());

        while  graph.node_count()>0{
            //Remove nodes from c_i
            for label in c_i.iter() {
                graph.remove_node(Self::get_node_index_from_weight(&graph,&label).unwrap());
            }

            (c_i,i_i) = Self::ramsey(&graph);

            if !c_i.is_empty(){
                cliques.push(c_i.clone());
            }

            if !i_i.is_empty(){
                isets.push(i_i.clone());
            }
        }

        let maxiset = isets.iter().max_by_key(|set| set.len()).unwrap();

        (maxiset.clone(),cliques)
    }
    pub fn ramsey(graph: &UnGraph<String,()>)->(Vec<String>,Vec<String>){
        //Return (Maximum clique, Maximum independent set)

        if graph.node_count()==0{
            (Vec::new(),Vec::new())
        }
        else{
            //let mut rng = thread_rng();
            //let node = graph.node_indices().collect::<Vec<_>>().choose(&mut rng).unwrap().clone();
            let node = graph.node_indices().collect::<Vec<_>>()[0];
            let label = graph.node_weight(node).unwrap();

            let (c_1,i_1) = Self::ramsey(&Self::undirected_subgraph(graph,&node,true));
            let (c_2,i_2) = Self::ramsey(&Self::undirected_subgraph(graph,&node,false));

            return (Self::larger_set_vec(&Self::new_element_vec(&c_1,label),&c_2), Self::larger_set_vec(&i_1,&Self::new_element_vec(&i_2,label)) )
        }
    }


}





























/*
impl Sample{


   pub fn max_clique(graph: &UnGraph<String, ()>) -> Vec<String> {
       let nodes: Vec<NodeIndex> = graph.node_indices().collect();
       let clique = Self::max_clique_rec(graph, &nodes);
       clique.into_iter().map(|i| graph[i].clone()).collect()
   }

   fn max_clique_rec(
       graph: &UnGraph<String, ()>,
       candidates: &[NodeIndex],
   ) -> Vec<NodeIndex> {
       if candidates.is_empty() {
           return Vec::new();
       }

       // Pick a pivot (simple: first element)
       let v = candidates[0];

       // Neighbors of v within candidates
       let neighbors: Vec<NodeIndex> = candidates
           .iter()
           .cloned()
           .filter(|&u| graph.contains_edge(v, u))
           .collect();

       // Branch 1: include v
       let mut with_v = vec![v];
       with_v.extend(Self::max_clique_rec(graph, &neighbors));

       // Branch 2: exclude v
       let without_v = Self::max_clique_rec(graph, &candidates[1..]);

       if with_v.len() > without_v.len() {
           with_v
       } else {
           without_v
       }
   }

   pub fn complement(graph: &UnGraph<String, ()>) -> UnGraph<String, ()> {
       let mut comp = UnGraph::<String, ()>::new_undirected();
       let mut map = HashMap::new();

       // Copy nodes
       for i in graph.node_indices() {
           let ni = comp.add_node(graph[i].clone());
           map.insert(i, ni);
       }

       let nodes: Vec<_> = graph.node_indices().collect();

       // Add missing edges
       for i in 0..nodes.len() {
           for j in i + 1..nodes.len() {
               let u = nodes[i];
               let v = nodes[j];
               if !graph.contains_edge(u, v) {
                   comp.add_edge(map[&u], map[&v], ());
               }
           }
       }

       comp
   }

   pub fn max_independent_set(graph: &UnGraph<String, ()>) -> Vec<String> {
       let comp = Self::complement(graph);
       Self::max_clique(&comp)
   }

   pub fn clique_removal(
       graph: &UnGraph<String, ()>,
   ) -> (Vec<String>, Vec<Vec<String>>) {
       let mut g = graph.clone();
       let mut cliques = Vec::new();
       let mut all_isets = Vec::new();

       while g.node_count() > 0 {
           let clique = Self::max_clique(&g);
           if clique.is_empty() {
               break;
           }

           // Remove clique nodes
           for label in &clique {
               if let Some(idx) = g
                   .node_indices()
                   .find(|&i| g[i] == *label)
               {
                   g.remove_node(idx);
               }
           }

           cliques.push(clique);
           all_isets.push(Self::max_independent_set(&g));
       }

       let max_iset = all_isets
           .into_iter()
           .max_by_key(|s| s.len())
           .unwrap_or_default();

       (max_iset, cliques)
   }


}
*/