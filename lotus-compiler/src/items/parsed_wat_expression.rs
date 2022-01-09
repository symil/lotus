use parsable::parsable;
use crate::{program::{ProgramContext, Wat}};
use super::ParsedWatToken;

#[parsable]
pub enum ParsedWatExpression {
    Leaf(ParsedWatToken),
    #[parsable(brackets="()")]
    Tree(ParsedWatToken, Vec<ParsedWatExpression>)
}

impl ParsedWatExpression {
    pub fn process(&self, context: &mut ProgramContext) -> Wat {
        match self {
            ParsedWatExpression::Leaf(token) => token.process(context),
            ParsedWatExpression::Tree(keyword, items) => {
                let mut wat = keyword.process(context);

                for wasm_expression in items {
                    wat.push(wasm_expression.process(context));
                }
            
                wat
            },
        }
    }
}