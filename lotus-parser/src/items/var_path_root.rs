use parsable::parsable;

use super::{ArrayLiteral, BooleanLiteral, FloatLiteral, IntegerLiteral, ObjectLiteral, RootVarRef, StringLiteral, VarRef};

#[parsable]
pub enum VarPathRoot {
    NullLiteral = "null",
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Variable(RootVarRef)
}