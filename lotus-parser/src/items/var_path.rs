use std::ops::Deref;

use parsable::parsable;

use super::{ArrayLiteral, BooleanLiteral, Expression, FloatLiteral, Identifier, IntegerLiteral, ObjectLiteral, StringLiteral};

#[parsable]
pub struct VarPath {
    pub root: PathRoot,
    pub path: Vec<PathSegment>
}

#[parsable]
pub enum PathRoot {
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Variable(Option<VarPrefix>, Identifier),
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarPrefix {
    This = "#",
    Payload = "$"
}

#[parsable]
pub enum PathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Expression),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(ArgumentList)
}

#[parsable]
pub struct ArgumentList {
    #[parsable(brackets="()", sep=",")]
    pub list: Vec<Expression>
}

impl PathSegment {
    pub fn is_function_call(&self) -> bool {
        match self {
            PathSegment::FunctionCall(_) => true,
            _ => false
        }
    }
}

impl Deref for ArgumentList {
    type Target = Vec<Expression>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}