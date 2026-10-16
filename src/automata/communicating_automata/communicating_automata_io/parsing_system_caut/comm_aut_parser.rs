use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "automata/communicating_automata/communicating_automata_io/parsing_system_caut/system_caut.pest"]
pub struct SystemCommunicatingAutGrammarParser;