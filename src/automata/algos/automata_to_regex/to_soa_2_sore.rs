use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::algos::soa2sore::Soa2Sore;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::soa::soa::SOA;

impl Automaton {
    pub fn to_regex_with_soa2sore(&self)->Regex<String>{
        let mut soa = SOA::from_automaton(self);
        
        let reg = Soa2Sore::soa2sore(&mut soa);
        reg
    }
}

















