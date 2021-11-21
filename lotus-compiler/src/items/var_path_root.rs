use parsable::{DataLocation, parsable};
use crate::{program::{AccessType, ProgramContext, Type, VariableKind, Vasm}};
use super::{Action, ArrayLiteral, Assignment, BlockExpression, BooleanLiteral, CharLiteral, Expression, FieldOrMethodAccess, ForBlock, Identifier, IfBlock, IterAncestors, IterFields, IterVariants, Macro, MatchBlock, NoneLiteral, NumberLiteral, ObjectLiteral, ParenthesizedExpression, StaticFieldOrMethod, StringLiteral, VarDeclaration, VarRef, WhileBlock};

#[parsable]
pub enum VarPathRoot {
    VarDeclaration(VarDeclaration),
    Action(Action),
    MatchBlock(MatchBlock),
    IfBlock(IfBlock),
    IterFields(IterFields),
    IterVariants(IterVariants),
    IterAncestors(IterAncestors),
    WhileBlock(WhileBlock),
    ForBlock(ForBlock),
    Block(BlockExpression),
    NoneLiteral(NoneLiteral),
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(NumberLiteral),
    CharLiteral(CharLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    StaticFieldOrMethod(StaticFieldOrMethod),
    #[parsable(ignore_if_marker="no-object")]
    ObjectLiteral(ObjectLiteral),
    #[parsable(unset_marker="no-object")]
    Parenthesized(ParenthesizedExpression),
    Macro(Macro),
    VarRef(VarRef),
}

impl VarPathRoot {
    fn is_var_ref(&self) -> bool {
        match self {
            VarPathRoot::VarRef(_) => true,
            _ => false
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
            VarPathRoot::Parenthesized(expr) => expr.collected_instancied_type_names(list),
            VarPathRoot::VarRef(var_ref) => var_ref.collected_instancied_type_names(list),
            _ => todo!()
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        if let AccessType::Set(location) = access_type {
            if !self.is_var_ref() {
                context.errors.add(location, format!("invalid assignment"));
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
            VarPathRoot::VarDeclaration(var_declaration) => var_declaration.process(context).map(|(_, vasm)| vasm),
            VarPathRoot::Action(action) => action.process(context),
            VarPathRoot::IfBlock(if_block) => if_block.process(type_hint, context),
            VarPathRoot::IterFields(iter_fields) => iter_fields.process(context),
            VarPathRoot::IterVariants(iter_variants) => iter_variants.process(context),
            VarPathRoot::IterAncestors(iter_ancestors) => iter_ancestors.process(context),
            VarPathRoot::WhileBlock(while_block) => while_block.process(context),
            VarPathRoot::ForBlock(for_block) => for_block.process(context),
            VarPathRoot::MatchBlock(match_block) => match_block.process(type_hint, context),
            VarPathRoot::Parenthesized(expr) => expr.process(type_hint, context),
            VarPathRoot::VarRef(var_ref) => var_ref.process(type_hint, access_type, context),
            VarPathRoot::Block(block) => block.process(type_hint, context),
        }
    }
}