use graphviz_dot_builder::traits::DotTranslatable;
use crate::automata::algos::automata_mining::ktssi::KTSSI;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;
use crate::regex_inf::soa::soa::SOA;

pub struct KTssiBrzozowski {}

impl KTssiBrzozowski {
    pub fn k_tssi_brzozowski(k: usize,sample: &Sample) ->Regex<String>{
        let automata = KTSSI::ktssi(k,sample);
        println!("{}",automata.to_dot().to_dot_string());
        Automaton::to_regex_brzozowski(&automata)
    }
}

