use parsable::parsable;

use super::{boolean::Boolean, identifier::Identifier, number::Number};

pub type Expr = Operation;

#[parsable(located)]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>
}

#[parsable(located)]
pub enum Operand {
    #[parsable(brackets="()")]
    Parenthesized(Box<Operation>),
    Number(Number),
    Boolean(Boolean),
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}

#[parsable(located)]
pub struct VarPath {
    pub prefix: Option<Pound>,
    pub name: Identifier,
    pub path: Vec<PathSegment>
}

#[parsable(located)]
pub enum Pound {
    Pound = "#"
}

#[parsable(located)]
pub enum PathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Operation),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(Vec<Operation>)
}

#[parsable(located)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}

#[parsable(located)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

#[parsable(located)]
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

#[parsable(located)]
pub struct TernaryOperation {
    pub condition: Operation,
    #[parsable(prefix="?")]
    pub if_expr: Operation,
    #[parsable(prefix=":")]
    pub else_expr: Operation
}