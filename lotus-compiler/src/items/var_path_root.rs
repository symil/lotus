use parsable::{DataLocation, parsable};
use crate::{program::{AccessType, ProgramContext, Type, VariableKind, Vasm}};
use super::{ArrayLiteral, BooleanLiteral, CharLiteral, Expression, FieldOrMethodAccess, Identifier, Macro, MatchBlock, NoneLiteral, NumberLiteral, ObjectLiteral, ParenthesizedExpression, VarRef, StaticFieldOrMethod, StringLiteral};

#[parsable]
pub enum VarPathRoot {
    MatchBlock(MatchBlock),
    NoneLiteral(NoneLiteral),
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(NumberLiteral),
    CharLiteral(CharLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    StaticFieldOrMethod(StaticFieldOrMethod),
    #[parsable(ignore_if_marker="no-object")]
    ObjectLiteral(ObjectLiteral),
    Variable(VarRef),
    #[parsable(unset_marker="no-object")]
    Parenthesized(ParenthesizedExpression),
    Macro(Macro),
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
            VarPathRoot::NumberLiteral(_) => false,
            VarPathRoot::CharLiteral(_) => false,
            VarPathRoot::StringLiteral(_) => true,
            VarPathRoot::ArrayLiteral(_) => true,
            VarPathRoot::ObjectLiteral(_) => true,
            VarPathRoot::StaticFieldOrMethod(_) => true,
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
            VarPathRoot::NumberLiteral(_) => {},
            VarPathRoot::CharLiteral(_) => {},
            VarPathRoot::StringLiteral(_) => {},
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.collected_instancied_type_names(list),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.collected_instancied_type_names(list),
            VarPathRoot::StaticFieldOrMethod(_) => {},
            VarPathRoot::Variable(root_var_ref) => {},
            VarPathRoot::Parenthesized(expr) => expr.collected_instancied_type_names(list),
            VarPathRoot::MatchBlock(_) => {}, // TODO
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        if let AccessType::Set(set_location) = access_type {
            if self.is_literal() {
                context.errors.add(set_location, format!("cannot assign value to a literal"));

                return None;
            }
        }

        match self {
            VarPathRoot::Macro(mac) => mac.process_as_value(context),
            VarPathRoot::NoneLiteral(none_literal) => none_literal.process(type_hint, context),
            VarPathRoot::BooleanLiteral(boolean_literal) => boolean_literal.process(context),
            VarPathRoot::NumberLiteral(number_literal) => number_literal.process(context),
            VarPathRoot::CharLiteral(char_literal) => char_literal.process(context),
            VarPathRoot::StringLiteral(string_literal) => string_literal.process(context),
            VarPathRoot::ArrayLiteral(array_literal) => array_literal.process(type_hint, context),
            VarPathRoot::ObjectLiteral(object_literal) => object_literal.process(context),
            VarPathRoot::StaticFieldOrMethod(static_field_or_method) => static_field_or_method.process(type_hint, context),
            VarPathRoot::Variable(root_var_ref) => root_var_ref.process(type_hint, access_type, context),
            VarPathRoot::Parenthesized(expr) => expr.process(type_hint, context),
            VarPathRoot::MatchBlock(match_block) => match_block.process(type_hint, context),
        }
    }
}