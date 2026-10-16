use petgraph::algo::{all_simple_paths, has_path_connecting};
use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::global_counter::counter::fresh_number;

impl SystemCFSM{
    /*
    pub fn introduce_empty_transition(&self) -> SystemCFSM{
        let mut new_system = self.clone();

        for machine in new_system.machines.iter_mut(){

            let q0 = machine.initial_states[0].clone();

            //We check whether there is a path frow q0 to q0
            let mut self_path = false;

            let self_loops = all_simple_paths::<Vec<_>, _>(&machine.graph,q0,q0,0,None).collect::<Vec<_>>();
            if self_loops.len() > 0{
                self_path = true;
            }



            let incoming_neighbors = machine.graph.neighbors_directed(q0,petgraph::Direction::Incoming);
            for neighbor in incoming_neighbors{
                let path = all_simple_paths::<Vec<_>, _>(&machine.graph,q0,neighbor,0,None).collect::<Vec<_>>();
                if path.len() > 0{
                    self_path = true;
                    break;
                }
            }

            if self_path {
                //If there is a loop from the initial node, then we create an empty transition
                let new_node = machine.graph.add_node(format!("q{}",fresh_number()));

                // We add a silent transition from q0 to new_node
                machine.graph.add_edge(q0,new_node,"Epsilon".to_string());
            }


        }

        new_system
    }

     */

    pub fn introduce_empty_transition(&self) -> SystemCFSM{
        let mut new_system = self.clone();

        for machine in new_system.machines.iter_mut(){

            //q_0 is autotatically a final state
            let q0 = machine.initial_states[0].clone();
            //If there is a loop from the initial node, then we create an empty transition
            let new_node = machine.graph.add_node(format!("q{}",fresh_number()));

            // We add a silent transition from q0 to new_node
            machine.graph.add_edge(q0,new_node,"Epsilon".to_string());

            machine.final_states.push(new_node.clone());
           // machine.final_states.push(q0);
            //We check whether there is a path frow q0 to q0

            //The nodes with no outgoing edges are final states
            
            for node in machine.graph.node_indices(){
                if machine.graph.neighbors_directed(node,petgraph::Direction::Outgoing).count() == 0{
                    machine.final_states.push(node);
                }
            }

        }

        new_system
    }
}
















