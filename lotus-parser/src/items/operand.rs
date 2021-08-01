use parsable::parsable;

use super::{ArrayLiteral, BooleanLiteral, Expression, NumberLiteral, ObjectLiteral, StringLiteral, UnaryOperation, VarPath};

#[parsable]
pub enum Operand {
    // TODO: add anonymous function
    #[parsable(brackets="()")]
    Parenthesized(Box<Expression>),
    VoidLiteral = ";",
    NullLiteral = "null",
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}