use parsable::{DataLocation, parsable};
use crate::{generation::{NULL_ADDR, Wat}, program::{AccessType, ProgramContext, Type, VariableScope, Wasm}};
use super::{ArrayLiteral, BooleanLiteral, Expression, FloatLiteral, IntegerLiteral, NullLiteral, ObjectLiteral, ParenthesizedExpression, RootVarRef, StringLiteral, VarRef};

#[parsable]
pub enum VarPathRoot {
    NullLiteral(NullLiteral),
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Variable(RootVarRef),
    Parenthesized(ParenthesizedExpression)
}

impl VarPathRoot {
    fn is_literal(&self) -> bool {
        match self {
            VarPathRoot::Variable(_) => false,
            _ => true
        }
    }

    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        if let AccessType::Set(set_location) = access_type {
            if self.is_literal() {
                context.error(set_location, format!("cannot assign value to a literal"));

                return None;
            }
        }

        match self {
            VarPathRoot::NullLiteral(null_literal) => null_literal.process(context),
            VarPathRoot::BooleanLiteral(boolean_literal) => boolean_literal.process(context),
            VarPathRoot::FloatLiteral(float_literal) => float_literal.process(context),
            VarPathRoot::IntegerLiteral(integer_literal) => integer_literal.process(context),
            VarPathRoot::StringLiteral(string_literal) => string_literal.process(context),
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.process(context),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.process(context),
            VarPathRoot::Variable(root_var_ref) => root_var_ref.process(access_type, context),
            VarPathRoot::Parenthesized(expr) => expr.process(context),
        }
    }
}