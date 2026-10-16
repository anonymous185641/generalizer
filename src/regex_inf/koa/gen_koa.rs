use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use rand::Rng;
use crate::regex_inf::koa::koa::KOA;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

impl KOA{
    pub fn gen_koa(sample: &Sample, k: usize) -> Self{
        todo!()
    }




    pub fn init(k: usize,alphabet: &Vec<String>)->Vec<Vec<usize>>{//OK
        //Here we initialize 10 individuals
        let mut P : Vec<Vec<usize>> = Vec::new();
        let size_matrix = alphabet.len()*k + 1;
        let mut rng = rand::thread_rng();

        for i in 0..10{
            let mut individual: Vec<usize> = Vec::new();

            for j in 0..size_matrix*size_matrix{
                individual.push(rng.gen_range(0..2));
            }
            P.push(individual);
        }
        P
    }

    pub fn decode(k:usize, code: &Vec<usize>,alphabet: &Vec<String>)->KOA{ //OK
        //This function transform a code vector into a KOA

        let size_matrix = alphabet.len()*k + 1;

        if code.is_empty(){
            return Self::new();
        }

        //We reconstruct the adjacency matrices
        let mut code_matrix :Vec<Vec<usize>>= Vec::new();

        for i in 0..size_matrix{
            let mut temp = Vec::new();
            for j in (i*size_matrix)..(i+1)*size_matrix{
                temp.push(code[j]);
            }
            code_matrix.push(temp);
        }

        //Handling the graph
        let mut koa = DiGraph::new();

        //Nodes
        let mut lab:HashMap<String,Vec<NodeIndex>> = HashMap::new();
        //SRC
        let src = koa.add_node(Regex::SRC);
        //lab.insert(src,Regex::SRC);
        //OTHER NODES
        for letter in alphabet{
            let mut temp_nodes_indices = Vec::new();
            for i in 0..k{
                temp_nodes_indices.push(koa.add_node(Regex::Literal(letter.clone())));
            }
            lab.insert(letter.clone(),temp_nodes_indices);

        }
        //SNK
        let snk = koa.add_node(Regex::SNK);
        //lab.insert(snk,Regex::SNK);


        //Edges
        for i in 0..size_matrix{
            for j in 0..size_matrix{
                if code_matrix[i][j]==1{
                    if (i == 0)&& (j<size_matrix-1) { //src to string node
                        // let n_1 = nodes_indices_map.get(&alphabet[((i+2)/k).floor()]).unwrap()[(i+2)%k];
                        let node = lab.get(&alphabet[(j/k)as usize]).unwrap()[j%k]; //FLOOR

                        koa.add_edge(src,node,());
                    }
                    else if (i==0)&&(j==size_matrix-1){ //src to snk
                        koa.update_edge(src,snk,());
                    }
                    else if (i > 0)&& (i<size_matrix-1)&& (j<size_matrix-1){// String node to String node
                        let node_1 = lab.get(&alphabet[((i-1)/k) as usize]).unwrap()[(i-1)%k]; //FLOOR
                        let node_2 = lab.get(&alphabet[(j/k) as usize]).unwrap()[j%k]; //FLOOR
                        koa.add_edge(node_1,node_2,());
                    }
                    else if (i > 0)&& (i<size_matrix-1)&& (j==size_matrix-1) { //String node to snk
                        let node = lab.get(&alphabet[((i-1)/k) as usize]).unwrap()[(i-1)%k]; //FLOOR
                        koa.add_edge(node,snk,());
                    }
                }
            }
        }

        KOA{
            nb_occurrence: k,
            koa: koa,
        }

    }
}