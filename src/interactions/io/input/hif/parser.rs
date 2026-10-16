








#[allow(unused_imports)]
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "interactions/io/input/hif/hif_syntax.pest"]
pub struct HifParser;

