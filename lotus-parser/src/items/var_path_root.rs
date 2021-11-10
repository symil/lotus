use parsable::{DataLocation, parsable};
use crate::{program::{AccessType, ProgramContext, Type, VariableKind, Vasm}};
use super::{ArrayLiteral, BooleanLiteral, CharLiteral, Expression, FieldOrMethodAccess, FloatLiteral, Identifier, IntegerLiteral, Macro, MatchBlock, NoneLiteral, ObjectLiteral, ParenthesizedExpression, RootVarRef, StringLiteral, ValueOrType, array_literal, char_literal, none_literal};

#[parsable]
pub enum VarPathRoot {
    Macro(Macro),
    MatchBlock(MatchBlock),
    NoneLiteral(NoneLiteral),
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    CharLiteral(CharLiteral),
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

    pub fn as_single_local_variable(&self) -> Option<&Identifier> {
        match self {
            VarPathRoot::Variable(var_ref) => var_ref.as_single_local_variable(),
            _ => None
        }
    }

    pub fn has_side_effects(&self) -> bool {
        match self {
            VarPathRoot::Macro(_) => false,
            VarPathRoot::NoneLiteral(_) => false,
            VarPathRoot::BooleanLiteral(_) => false,
            VarPathRoot::FloatLiteral(_) => false,
            VarPathRoot::IntegerLiteral(_) => false,
            VarPathRoot::CharLiteral(_) => false,
            VarPathRoot::StringLiteral(_) => true,
            VarPathRoot::ArrayLiteral(_) => true,
            VarPathRoot::ObjectLiteral(_) => true,
            VarPathRoot::Variable(var_ref) => var_ref.has_side_effects(),
            VarPathRoot::Parenthesized(expr) => expr.has_side_effects(),
            VarPathRoot::MatchBlock(match_block) => true,
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        match self {
            VarPathRoot::Macro(_) => {},
            VarPathRoot::NoneLiteral(_) => {},
            VarPathRoot::BooleanLiteral(_) => {},
            VarPathRoot::FloatLiteral(_) => {},
            VarPathRoot::IntegerLiteral(_) => {},
            VarPathRoot::CharLiteral(_) => {},
            VarPathRoot::StringLiteral(_) => {},
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.collected_instancied_type_names(list),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.collected_instancied_type_names(list),
            VarPathRoot::Variable(root_var_ref) => {},
            VarPathRoot::Parenthesized(expr) => expr.collected_instancied_type_names(list),
            VarPathRoot::MatchBlock(_) => {}, // TODO
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
            VarPathRoot::Macro(mac) => mac.process_as_value(context),
            VarPathRoot::NoneLiteral(none_literal) => none_literal.process(type_hint, context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::BooleanLiteral(boolean_literal) => boolean_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::FloatLiteral(float_literal) => float_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::IntegerLiteral(integer_literal) => integer_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::CharLiteral(char_literal) => char_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::StringLiteral(string_literal) => string_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.process(type_hint, context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.process(context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::Variable(root_var_ref) => root_var_ref.process(type_hint, access_type, context),
            VarPathRoot::Parenthesized(expr) => expr.process(type_hint, context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
            VarPathRoot::MatchBlock(match_block) => match_block.process(type_hint, context).and_then(|vasm| Some(ValueOrType::Value(vasm))),
        }
    }
}