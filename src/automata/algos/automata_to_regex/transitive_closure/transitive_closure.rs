use std::collections::BTreeMap;
use petgraph::graph::NodeIndex;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;

impl Automaton{

    /**/

    pub fn to_regex_with_transitive_closure(&self)->Result<Regex<String>,String>{
        let mut final_regex = Regex::Empty;

        let mut r: Vec<Vec<Vec<Regex<String>>>> = vec![vec![vec![Regex::Empty; self.graph.node_count()]; self.graph.node_count()]; self.graph.node_count()];

        let mut r_init: Vec<Vec<Regex<String>>> = vec![vec![Regex::Empty; self.graph.node_count()]; self.graph.node_count()];


        let mut starting_state_index = self.initial_states.first().unwrap().clone();

        let mut nodes_indices = self.graph.node_indices().collect::<Vec<_>>();
        nodes_indices.sort();
        nodes_indices.dedup();

        let mut node_mapping = BTreeMap::new();
        for (i,node) in nodes_indices.iter().enumerate(){
            node_mapping.insert(node.clone(),i);
        }

        let n = nodes_indices.len();

        for i in 0..n{
            r_init[i][i] = Regex::Epsilon;
        }

        //Initialisation of the matrix
        for (i,node) in nodes_indices.iter().enumerate(){

            for (j,node2) in nodes_indices.iter().enumerate(){
                //Initialization
                if self.graph.contains_edge(*node,*node2){

                    let regex_label= self.trans_closure_regex_between(node,node2);

                    if node == node2{
                        r_init[i][j] = Regex::or_vec_retain_non_empty(vec![regex_label,Regex::Epsilon]);
                    }
                    else{
                        r_init[i][j] = regex_label;
                    }

                }

            }
        }

        //Induction
        for k in 0..self.graph.node_count(){
            for i in 0..self.graph.node_count(){
                for j in 0..self.graph.node_count(){
                    if k==0{
                        let kk_kleene = Regex::kleene(&r_init[k][k].clone());
                        let factor_left = Regex::concatenate_vec(vec![r_init[i][k].clone(),kk_kleene,r_init[k][j].clone()]);
                        r[i][j][k] = Regex::or_vec_retain_non_empty(vec![factor_left,r_init[i][j].clone()]);
                        //println!("i={} j={} k={} {:?}",i,j,k,r[i][j][k]);
                    }
                    else{
                        let kk_kleene = Regex::kleene(&r[k][k][k-1].clone());
                        let factor_left = Regex::concatenate_vec(vec![r[i][k][k-1].clone(),kk_kleene,r[k][j][k-1].clone()]);
                        r[i][j][k] = Regex::or_vec_retain_non_empty(vec![factor_left,r[i][j][k-1].clone()]);
                        //println!("i={} j={} k={} {:?}",i,j,k,r[i][j][k]);
                    }

                }
            }
        }

        let index_starting_state = node_mapping[&starting_state_index];
        for final_state in self.final_states.iter(){
            let index_final = node_mapping[final_state];
            final_regex = Regex::or_vec_retain_non_empty(vec![final_regex,r[index_starting_state][index_final][self.graph.node_count()-1].clone()]);
        }

        Ok(final_regex.simplify())
    }



    /*
    pub fn to_regex_with_transitive_closure(&self)->Result<Regex<String>,String>{
        let mut final_regex = Regex::Empty;



        let mut nodes_indices = self.graph.node_indices().collect::<Vec<_>>();
        nodes_indices.sort();
        nodes_indices.dedup();

        let n = nodes_indices.len();

        if n == 0 {
            return Ok(Regex::Empty);
        }


        let mut node_mapping = BTreeMap::new();
        for (i,node) in nodes_indices.iter().enumerate(){
            node_mapping.insert(node.clone(),i);
        }


        ///////////////////////////////////////////////
        //Initialization
        let mut prev =
            vec![vec![Regex::Empty; n]; n];

        // ε paths on the diagonal
        for i in 0..n {
            prev[i][i] = Regex::Epsilon;
        }

        for edge in self.graph.edge_indices().into_iter() {
            let (src, tgt) =
                self.graph.edge_endpoints(edge).unwrap();

            let i = node_mapping[&src];
            let j = node_mapping[&tgt];

            let label =
                self.graph.edge_weight(edge).unwrap();

            let regex_label = if label == "Epsilon" {
                Regex::Epsilon
            } else {
                Regex::Literal(label.clone())
            };

            prev[i][j] = Regex::or(
                &prev[i][j],
                &regex_label,
            );
        }


        //Induction
        /*
        * Kleene recurrence
         */
        for k in 0..n {
            let mut next =
                vec![vec![Regex::Empty; n]; n];

            for i in 0..n {
                for j in 0..n {
                    let through_k =
                        Regex::concatenate_vec(vec![
                            prev[i][k].clone(),
                            Regex::kleene(&prev[k][k]),
                            prev[k][j].clone(),
                        ]);

                    next[i][j] =
                        Regex::or(
                            &prev[i][j],
                            &through_k,
                        );
                    //println!("i={} j={} k={} {:?}",i,j,k,next[i][j]);
                }
            }

            prev = next;
        }

        let initial_state = self.initial_states.first().unwrap().clone();
        let start = node_mapping[&initial_state];

        for final_state in &self.final_states {
            let final_index =
                node_mapping[final_state];

            final_regex = Regex::or(
                &final_regex,
                &prev[start][final_index],
            );
        }

        Ok(final_regex.simplify())
    }

 */


    pub fn trans_closure_regex_between(&self,n1: &NodeIndex,n2: &NodeIndex)->Regex<String>{
        let mut regex = Regex::Empty;

        for edge in self.graph.edge_indices().into_iter(){

            let (src,tgt) = self.graph.edge_endpoints(edge).unwrap();

            if src == *n1 && tgt == *n2{

                let label = self.graph.edge_weight(edge).unwrap().clone();

                let regex_label = if label.eq("Epsilon") {
                    Regex::Epsilon
                }
                else{
                    Regex::Literal(label.clone())
                };

                regex = Regex::or(&regex,&regex_label);

            }
        }

        regex
    }

}







































































