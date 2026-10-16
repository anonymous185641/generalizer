use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;

#[derive(Debug, Clone)]
pub enum AutomataToRegexAlgo{
    Brzozowski,
    ViaPetriNets,
    StateElimination,
    TransitiveClosure
}


impl AutomataToRegexAlgo{

    pub fn get_algo_name(&self)->&str{
        match self{
            AutomataToRegexAlgo::Brzozowski=>"Brzozowski",
            AutomataToRegexAlgo::ViaPetriNets=>"ViaPetriNets",
            AutomataToRegexAlgo::StateElimination=>"StateElimination",
            AutomataToRegexAlgo::TransitiveClosure=>"TransitiveClosure"
        }
    }

    pub fn algo_fn (&self)->fn(aut: &Automaton)->Regex<String>{
        match self{
            Self::Brzozowski=> Automaton::automaton_to_regex_brzozowski,
            _=> panic!("Not Implemented")
        }
    }
}