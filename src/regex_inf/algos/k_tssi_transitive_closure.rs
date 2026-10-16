use crate::automata::algos::automata_mining::ktssi::KTSSI;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

pub struct KTssiTransitiveClosure {}

impl KTssiTransitiveClosure {
    pub fn k_tssi_transitive_closure(k: usize,sample: &Sample) ->Regex<String>{
        let automata = KTSSI::ktssi(k,sample);
        
       automata.transitive_closure_old()
    }
}