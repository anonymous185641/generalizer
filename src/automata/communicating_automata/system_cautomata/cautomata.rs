use strum_macros::ToString;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::system_cautomata::cautomata_context::SystemCommunicatingAutomataContext;
use crate::automata::new_automaton::finite_automaton::FiniteAutomaton;
use crate::interactions::syntax::general_context::GeneralContext;

#[derive(Debug, Clone,Ord,PartialOrd,Eq,PartialEq)]
pub struct CommunicatingAutomatonName(pub String);

#[derive(Debug, Clone,Ord,PartialOrd,Eq,PartialEq)]
pub struct CommunicatingAutomatonMessage(pub String);


//TO DO: Harmonize with the notion of gates of interactions models
#[derive(Debug, Clone,Ord,PartialOrd,Eq,PartialEq)]
pub struct Gate(pub usize);

#[derive(Debug, Clone)]
pub struct CommunicatingAutomaton{
    pub automaton: Automaton,
    pub name: CommunicatingAutomatonName,
}
impl CommunicatingAutomaton{
    pub fn new(automaton: Automaton, name: CommunicatingAutomatonName) -> CommunicatingAutomaton {
        CommunicatingAutomaton {automaton, name}
    }
    pub fn size(&self)->usize{
        self.automaton.size()
    }
}

#[derive(Debug, Clone)]
pub struct SystemCommunicatingAutomata {
    pub context : SystemCommunicatingAutomataContext,
    pub automata: Vec<CommunicatingAutomaton>,
    pub system_name: String,
}


impl SystemCommunicatingAutomata {
    pub fn new() -> SystemCommunicatingAutomata {
        SystemCommunicatingAutomata {context:SystemCommunicatingAutomataContext::new(), automata: Vec::new(), system_name: String::new()
            }
    }
}
















