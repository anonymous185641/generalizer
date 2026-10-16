

use dot_parser::*;
use dot_parser::ast::Stmt;
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use crate::automata::automaton::automaton::Automaton;

impl  Automaton {

    pub fn from_dot_file(file_path: &str) -> Result<Automaton,String> {


        if let Ok(graph) = ast::Graph::from_file(file_path){
            //println!("{:?}",graph);

            let mut stable_graph = StableGraph::<String,String>::new();
            let mut initial_states = Vec::new();
            let mut final_states = Vec::new();

            //Adding nodes to the graph
            for stmt in graph.stmts.clone() {
                //println!("{:?}",stmt);

                match stmt {
                    Stmt::NodeStmt(node)=>{
                        if node.node.id =="init"{
                            continue;
                        }
                        else{
                            let node_index = stable_graph.add_node(node.node.id.clone());
                            if let Some(attr) = node.attr{
                                let mut attr_mut = attr.flatten();
                                //attr_mut.flatten();
                                for al in  attr_mut.elems{
                                    if al==("shape".to_string(), "doublecircle".to_string()){
                                        final_states.push(node_index);
                                    }
                                }
                            }
                        }
                    },
                    Stmt::Subgraph(subgraph)=>{
                        panic!("Subgraph not supported");
                    },
                    _ => continue
                }
            }

            //Adding edges
            for stmt in graph.stmts {
                //println!("{:?}",stmt);

                match stmt {
                    Stmt::EdgeStmt(edge)=>{
                        let left = edge.from.left().unwrap();
                        let source_index = Self::get_node_index_by_weight(&stable_graph,&left.id).unwrap_or(NodeIndex::new(10000));

                        let right = edge.next.to.left().unwrap();
                        let target_index = Self::get_node_index_by_weight(&stable_graph,&right.id).unwrap();

                        let attr = edge.attr;

                        //Initial state handling
                        if left.id == "init".to_string(){
                              initial_states.push(target_index);
                        }

                        //Other edges
                        if let Some(attr) = attr{
                            let attr_mut = attr.flatten();
                            //attr_mut.flatten();


                            for el in attr_mut.elems{
                                if el.0 == "label".to_string(){
                                    //Add the edge
                                    stable_graph.add_edge(source_index,target_index,el.1.clone().trim_matches('"').to_string());
                                }
                            }
                        }
                    },
                    Stmt::Subgraph(subgraph)=>{
                        panic!("Subgraph not supported");
                    },
                    _ => continue
                }
            }

            return Ok(Automaton{initial_states,final_states,graph:stable_graph});


        }


        Err("Error parsing automaton the file".to_string())
    }

    fn get_node_index_by_weight(graph: &StableGraph<String,String>,weight: &String)-> Option<NodeIndex>{
        for node in graph.node_indices(){
            if graph[node]==*weight{
                return Some(node);
            }
        }
        None
    }

}