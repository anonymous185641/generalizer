use petgraph::graph::NodeIndex;
use petgraph::prelude::StableGraph;
use petgraph::visit::NodeRef;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;
use crate::automata::communicating_automata::system_cautomata::cautomata_actions::CommunicatingAutomataValuePassing;

#[derive(Ord, Eq, PartialOrd, PartialEq,Clone, Debug)]
pub struct SyncProductState{
    pub state: Vec<NodeIndex>,
}


impl SyncProductState{

    pub fn get_initial_state(system: &SystemCommunicatingAutomata)-> SyncProductState{
        let mut state: Vec<NodeIndex> = Vec::new();

        for machine in system.automata.iter(){
            state.push(machine.automaton.initial_states.first().unwrap().clone());
        }

        SyncProductState{state}
    }

    pub fn get_final_states(system: &SystemCommunicatingAutomata,graph: &StableGraph<SyncProductState,CommunicatingAutomataValuePassing>)->Vec<NodeIndex>{
        
        let mut final_states: Vec<NodeIndex> = Vec::new();
        
        for node in graph.node_indices(){
            let state = graph[node].clone();
            
            let mut is_final = true;
            for (aut,idx) in system.automata.iter().zip(state.state.iter()){
                if !aut.automaton.final_states.contains(idx){
                    is_final = false;
                    break;
                }
            }
            if is_final{
                final_states.push(node);
            }
            
        }
        
        final_states
    }
}

impl std::fmt::Display for SyncProductState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("[");
        for node in self.state.iter(){
            s.push_str(&format!("{}",node.index()));
            s.push_str(",");
        }
        s.pop();
        s.push_str("]");
        write!(f, "\"{}\"", s)
    }
}