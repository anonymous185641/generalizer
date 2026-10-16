use petgraph::Graph;
use petgraph::graph::UnGraph;
use crate::regex_inf::sample::sample::Sample;

impl Sample{
    pub fn transitive_closure(&self)-> Vec<(String,String)>{//To Test
        //This function computes the transitive closure of substrings in the sample
        let mut return_set= Vec::new();

        for string in &self.sample{
            if self.separator==None{
                let mut separated_iterator = string.chars();
                let size = separated_iterator.clone().count();

                //Some duplicated code here, to optimize
                for i in 0..size{
                    for j in i+1..=size{
                        if let (Some(a),Some(b)) = (separated_iterator.clone().nth(i),separated_iterator.clone().nth(j)) {
                            if a != b {
                                return_set.push((a.to_string(),b.to_string()));
                            }

                        }
                    }

                }
            }
            else {
                let mut separated_iterator = string.split_terminator(self.separator.unwrap());

                let size = separated_iterator.clone().count();

                //Some duplicated code here, to optimize
                for i in 0..size{
                    for j in i+1..=size{
                        if let (Some(a),Some(b)) = (separated_iterator.clone().nth(i),separated_iterator.clone().nth(j)) {
                            if a != b{
                                return_set.push((a.to_string(),b.to_string()));
                            }

                        }
                    }

                }


            }
        }


        return_set
    }

    pub fn tran_reduction(&self)->(Vec<(String,String)>,Vec<(String,String)>){//To Test
        let mut cs  = Vec::new();
        let mut ncs = Vec::new();

        let trans_closure = self.transitive_closure();

        for (a,b) in trans_closure.iter(){

            if trans_closure.contains(&(b.clone(),a.clone())){
                cs.push((a.clone(),b.clone()));
                cs.push((b.clone(),a.clone()));
            }
            else{
                ncs.push((a.clone(),b.clone()));
            }
        }

        cs.sort();
        cs.dedup();

        ncs.sort();
        ncs.dedup();

        (cs,ncs)
    }

    pub fn get_alphabet_from_cs(&self)->Vec<String>{
        let mut alphabet = Vec::new();
        let (cs,ncs) = self.tran_reduction();

        for (a,b) in cs{

            alphabet.push(a);
            alphabet.push(b);
        }

        alphabet.sort();
        alphabet.dedup();
        alphabet
    }
    pub fn graph_from_cs(&self)-> UnGraph<String,()> {
        let alphabet = self.get_alphabet_from_cs();
        let mut graph = Graph::new_undirected();
        let (cs, ncs) = self.tran_reduction();
        //println!("CS--: {:?}",cs);

        for letter in alphabet {
            graph.add_node(letter);
        }

        for (a,b) in cs.iter() {
            let a_id = Self::get_node_index_from_weight(&graph, &a.to_string()).unwrap();
            let b_id = Self::get_node_index_from_weight(&graph, &b.to_string()).unwrap();
            graph.update_edge(a_id, b_id, ());
        }

        graph
    }
}