use parsable::parsable;

use super::{boolean::Boolean, identifier::Identifier, number::Number};

pub type Expression = Operation;

#[parsable]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>
}

#[parsable]
pub enum Operand {
    // TODO: add anonymous function
    #[parsable(brackets="()")]
    Parenthesized(Box<Operation>),
    Number(Number),
    Boolean(Boolean),
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}

#[parsable]
pub struct VarPath {
    pub prefix: Option<VarPrefix>,
    pub name: Identifier,
    pub path: Vec<PathSegment>
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
    BracketIndexing(Operation),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(Vec<Operation>)
}

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

#[parsable(impl_display=true)]
pub enum BinaryOperator {
    Pow = "**",
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

#[parsable]
pub struct TernaryOperation {
    pub condition: Operation,
    #[parsable(prefix="?")]
    pub if_expr: Operation,
    #[parsable(prefix=":")]
    pub else_expr: Operation
}