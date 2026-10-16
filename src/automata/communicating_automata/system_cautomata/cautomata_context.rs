use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomatonMessage, CommunicatingAutomatonName, Gate};
use crate::interactions::syntax::general_context::GeneralContext;

#[derive(Debug, Clone)]
pub struct SystemCommunicatingAutomataContext{
    pub automata_names: Vec<CommunicatingAutomatonName>,
    pub messages: Vec<CommunicatingAutomatonMessage>,
    pub gates: Vec<Gate>,
}
impl SystemCommunicatingAutomataContext{
    pub fn new() -> SystemCommunicatingAutomataContext{
        SystemCommunicatingAutomataContext{
            automata_names: Vec::new(),
            messages: Vec::new(),
            gates: Vec::new(),
        }
    }
    
    pub fn add_automaton_name(&mut self, name:CommunicatingAutomatonName){
        self.automata_names.push(name);
    }
    
    pub fn add_message(&mut self, message:CommunicatingAutomatonMessage){
        self.messages.push(message);
    }
    
    pub fn add_gate(&mut self, gate:Gate){
        self.gates.push(gate);
    }
    
    pub fn sort_and_dedup(&mut self){
        self.automata_names.sort();
        self.automata_names.dedup();
        
        self.messages.sort();
        self.messages.dedup();
        
        self.gates.sort();
        self.gates.dedup();
    }
    pub fn to_interaction_context(&self)->GeneralContext{
        todo!()
    }
}