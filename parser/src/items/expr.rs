use lotus_parsable::parsable;

use super::{boolean::Boolean, identifier::Identifier, number::Number};

pub type Expr = Operation;

#[parsable(located)]
#[derive(Debug)]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>
}

#[parsable(located)]
#[derive(Debug)]
pub enum Operand {
    #[parsable(brackets="()")]
    Parenthesized(Box<Operation>),
    Number(Number),
    Boolean(Boolean),
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}

#[parsable(located)]
#[derive(Debug)]
pub struct VarPath {
    pub name: Identifier,
    pub path: Vec<PathSegment>
}

#[parsable(located)]
#[derive(Debug)]
pub enum PathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Operation),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(Vec<Operation>)
}

#[parsable(located)]
#[derive(Debug)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}

#[parsable(located)]
#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

#[parsable(located)]
#[derive(Debug)]
pub enum BinaryOperator {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%",
    And = "&&",
    Or = "||",
    Eq = "==",
    Neq = "!=",
    Gte = ">=",
    Gt = ">",
    Lte = "<=",
    Lt = "<"
}