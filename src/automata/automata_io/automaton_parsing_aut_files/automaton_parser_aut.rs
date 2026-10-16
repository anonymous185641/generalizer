use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "automata/automata_io/automaton_parsing_aut_files/automaton_aut_grammar.pest"]
pub struct AutomatonAUTFileGrammarParser;