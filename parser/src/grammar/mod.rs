use pest::{iterators::Pair};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct PestParser;

pub trait FromEntry : Sized {
    fn from_entry(entry: Pair<Rule>) -> Self;
}