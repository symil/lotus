use parsable::parsable;

use super::{ArrayLiteral, BooleanLiteral, Expression, FloatLiteral, ObjectLiteral, StringLiteral, Type, UnaryOperation, VarPath};

#[parsable]
pub enum Operand {
    // TODO: add anonymous function
    #[parsable(brackets="()")]
    Parenthesized(Box<Expression>),
    VoidLiteral = ";",
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}