use std::collections::{HashMap, HashSet};
use autour_core::nfa::AutNFA;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::system_cautomata::cautomata::CommunicatingAutomaton;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::general_context::GeneralContext;

impl CommunicatingAutomaton{
    pub fn to_autour_nfa(&self,gen_ctx: &GeneralContext,alphabet: &Vec<Action>)->AutNFA<usize>{

        let usize_alphabet : HashSet<usize> = (0..alphabet.len()).collect();

        let mut initials = HashSet::new();
        let mut finals = HashSet::new();
        let mut transitions = Vec::new();
        
        //Initialization of transitions
        for n in 0..self.automaton.graph.node_count(){
            transitions.push(HashMap::<usize,HashSet<usize>>::new());
            for l in 0..alphabet.len(){
                transitions[n].insert(l,HashSet::new());
            }
        }
        
        //Initial states
        for node_id in self.automaton.initial_states.iter(){
            initials.insert(node_id.index());
        }
        
        //Final states
        for node_id in self.automaton.final_states.iter(){
            finals.insert(node_id.index());
        }
        
        //Transitions
        for edge in self.automaton.graph.edge_indices(){
            let (src,tgt) = self.automaton.graph.edge_endpoints(edge).unwrap();
            let label = self.automaton.graph.edge_weight(edge).unwrap();
            let letter_id:usize= match self.label_conversion(label,gen_ctx,alphabet){
                Some(id)=>id,
                None=>alphabet.len() //Empty transition ????
            }; 
            transitions[src.index()].get_mut(&letter_id).unwrap().insert(tgt.index());
        }
        
        AutNFA{alphabet: usize_alphabet, initials, finals, transitions}
    }
}


















