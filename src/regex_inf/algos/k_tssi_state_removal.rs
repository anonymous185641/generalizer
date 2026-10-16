use crate::automata::algos::automata_mining::ktssi::KTSSI;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

pub struct KTssiStateRemoval {}

impl KTssiStateRemoval {
    pub fn k_tssi_state_removal(k: usize,sample: &Sample) ->Regex<String>{
        let automata = KTSSI::ktssi(k,sample);
        todo!()
       // Automaton::dfa_to_regex_state_removal(&automata)
    }
}