use std::collections::HashMap;
use itertools::Itertools;
use petgraph::adj::NodeIndex;
use petgraph::visit::NodeCount;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;


//Not sure.....


impl Automaton {

    pub fn transitive_closure_old(&self) -> Regex<String>{


        let mut final_regex = Regex::Empty;

        let mut r: Vec<Vec<Vec<Regex<String>>>> = vec![vec![vec![Regex::Empty; self.graph.node_count()]; self.graph.node_count()]; self.graph.node_count()];

        let mut final_states_indices = Vec::new();
        let mut starting_state_index = None;

        let nodes_indices = self.graph.node_indices().collect::<Vec<_>>();
        //Initialisation of the matrix
        for (i,node) in nodes_indices.iter().enumerate(){
            if self.final_states.contains(&node){
                final_states_indices.push(i);
            }
            if self.initial_states.contains(&node){
                starting_state_index = Some(i);
            }

            for (j,node2) in nodes_indices.iter().enumerate(){
                //Initialization
                if self.graph.contains_edge(*node,*node2){
                      let edge_id = self.graph.find_edge(*node,*node2).unwrap();
                      let label = self.graph.edge_weight(edge_id).unwrap().clone();
                    let regex_label = if label.eq("Epsilon") {
                        Regex::Epsilon
                    } else {
                        Regex::Literal(label.clone())
                    };
                    println!("{:?} -> {:?} : {:?}",node,node2,regex_label);

                    if node == node2{
                        r[i][j][0] = Regex::or_vec_retain_non_empty(vec![regex_label,Regex::Epsilon]);
                    }
                    else{
                        r[i][j][0] = regex_label;
                    }

                }

            }
        }
        println!("FS: {:?}",starting_state_index);

        //Induction
        for k in 1..self.graph.node_count(){
            for i in 0..self.graph.node_count(){
                for j in 0..self.graph.node_count(){
                    let kk_kleene = Regex::kleene(&r[k][k][k-1].clone());
                    let factor_left = Regex::concatenate_vec(vec![r[i][k][k-1].clone(),kk_kleene,r[k][j][k-1].clone()]);
                    r[i][j][k] = Regex::or_vec_retain_non_empty(vec![factor_left,r[i][j][k-1].clone()]);
                    println!("TABBNBBB: i={} j={} k={} {:?}",i,j,k,r[i][j][k]);
                }
            }
        }

        if starting_state_index.is_none(){
            panic!("No starting state found");
        }

        for i in final_states_indices.iter(){
            final_regex = Regex::or_vec_retain_non_empty(vec![final_regex,r[starting_state_index.unwrap()][*i][self.graph.node_count()-1].clone()]);
        }

        final_regex


    }
}












