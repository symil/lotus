use parsable::parsable;

use super::{ArgumentList, Expression, Identifier};

#[parsable]
pub enum VarPathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Expression),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(ArgumentList)
}

impl VarPathSegment {
    pub fn is_function_call(&self) -> bool {
        match self {
            VarPathSegment::FunctionCall(_) => true,
            _ => false
        }
    }
}