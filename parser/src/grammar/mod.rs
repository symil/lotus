use pest::{iterators::Pair};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct PestParser;

pub trait Parsable : Sized {
    fn parse(entry: Pair<Rule>) -> Self;
}