use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "petri_nets/petri_net_io/petri_net_parsing/petri_nets_grammar.pest"]
pub struct PetriNetsGrammarParser;

