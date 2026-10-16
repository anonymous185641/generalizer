use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "automata/cfsm/system_cfsm_io/parsing_system_cfsm/system_cfsm.pest"]
pub struct CFSMGrammarParser;
