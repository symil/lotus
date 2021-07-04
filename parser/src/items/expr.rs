use lotus_parsable::parsable;

use super::{boolean::Boolean, identifier::Identifier, number::Number};

#[parsable]
#[derive(Debug)]
pub enum Expr {
    #[parsable(brackets="()")]
    Parenthesized(Box<Expr>),
    Number(Number),
    Boolean(Boolean),
    UnaryOperation(Box<UnaryOperation>),
    Path(PathExpr),
    BinaryOperation(Box<Operation>),
}

#[parsable]
#[derive(Debug)]
pub struct PathExpr {
    pub name: Identifier,
    pub path: Vec<PathSegment>
}

#[parsable]
#[derive(Debug)]
pub enum PathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Expr),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(Vec<Expr>)
}

#[parsable]
#[derive(Debug)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}

#[parsable]
#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Expr
}

#[parsable]
#[derive(Debug)]
pub enum BinaryOperator {
    Add = "+",
    Substract = "-",
    Multiply = "*",
    Divide = "/"
}

#[parsable]
#[derive(Debug)]
pub struct Operation {
    pub first: Expr,
    #[parsable(min=1)]
    pub others: Vec<(BinaryOperator, Expr)>
}