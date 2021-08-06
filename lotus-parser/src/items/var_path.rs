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
    NullLiteral = "null",
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Variable(Variable)
}

#[parsable]
pub struct Variable {
    pub prefix: Option<VarPrefix>,
    pub name: Identifier
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarPrefix {
    This = "#",
    Payload = "$",
    System = "@"
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

impl Variable {
    pub fn has_this_prefix(&self) -> bool {
        match self.prefix {
            Some(VarPrefix::This) => true,
            _ => false
        }
    }

    pub fn has_payload_prefix(&self) -> bool {
        match self.prefix {
            Some(VarPrefix::Payload) => true,
            _ => false
        }
    }
}

impl PathSegment {
    pub fn is_function_call(&self) -> bool {
        match self {
            PathSegment::FunctionCall(_) => true,
            _ => false
        }
    }
}

impl ArgumentList {
    pub fn as_vec(&self) -> &Vec<Expression> {
        &self.list
    }
}