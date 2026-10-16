use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;

impl SystemCommunicatingAutomata {

    pub fn draw(&self,name:&String,folder:&String) {

        for (i,cautomaton) in self.automata.iter().enumerate() {
            cautomaton.automaton.draw(&format!("{}_{}",name,cautomaton.name.0),folder);
        }
    }


}