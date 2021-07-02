use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct PestParser;