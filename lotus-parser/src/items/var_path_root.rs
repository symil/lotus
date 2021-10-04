use parsable::{DataLocation, parsable};
use crate::{program::{AccessType, ProgramContext, Type, VariableKind, Vasm}};
use super::{ArrayLiteral, BooleanLiteral, CompilerConstant, Expression, FieldOrMethodAccess, FloatLiteral, IntegerLiteral, ObjectLiteral, ParenthesizedExpression, RootVarRef, StringLiteral, ValueOrType, compiler_constant};

#[parsable]
pub enum VarPathRoot {
    CompilerConstant(CompilerConstant),
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    #[parsable(ignore_if_marker="no-object")]
    ObjectLiteral(ObjectLiteral),
    Variable(RootVarRef),
    #[parsable(unset_marker="no-object")]
    Parenthesized(ParenthesizedExpression)
}

impl VarPathRoot {
    fn is_literal(&self) -> bool {
        match self {
            VarPathRoot::Variable(_) => false,
            _ => true
        }
    }

    pub fn has_side_effects(&self) -> bool {
        match self {
            VarPathRoot::CompilerConstant(_) => false,
            VarPathRoot::BooleanLiteral(_) => false,
            VarPathRoot::FloatLiteral(_) => false,
            VarPathRoot::IntegerLiteral(_) => false,
            VarPathRoot::StringLiteral(_) => true,
            VarPathRoot::ArrayLiteral(_) => true,
            VarPathRoot::ObjectLiteral(_) => true,
            VarPathRoot::Variable(var_ref) => var_ref.has_side_effects(),
            VarPathRoot::Parenthesized(expr) => expr.has_side_effects(),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<ValueOrType> {
        if let AccessType::Set(set_location) = access_type {
            if self.is_literal() {
                context.errors.add(set_location, format!("cannot assign value to a literal"));

                return None;
            }
        }

        match self {
            VarPathRoot::CompilerConstant(compiler_constant) => compiler_constant.process_as_value(context),
            VarPathRoot::BooleanLiteral(boolean_literal) => boolean_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::FloatLiteral(float_literal) => float_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::IntegerLiteral(integer_literal) => integer_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::StringLiteral(string_literal) => string_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::Variable(root_var_ref) => root_var_ref.process(type_hint, access_type, context),
            VarPathRoot::Parenthesized(expr) => expr.process(type_hint, context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
        }
    }
}