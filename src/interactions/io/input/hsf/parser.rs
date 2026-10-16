

#[allow(unused_imports)]
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "interactions/io/input/hsf/hsf_syntax.pest"]
pub struct HsfParser;
