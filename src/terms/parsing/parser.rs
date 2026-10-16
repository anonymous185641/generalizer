use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "terms/parsing/file_grammar.pest"]
pub struct GrammarParser;
