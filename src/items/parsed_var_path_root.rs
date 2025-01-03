use parsable::{ItemLocation, parsable};
use crate::{program::{AccessType, ProgramContext, Type, VariableKind, Vasm}};
use super::{ParsedAction, ParsedArrayLiteral, ParsedOperandBody, ParsedBlockExpression, ParsedBooleanLiteral, ParsedCharLiteral, ParsedExpression, ParsedFieldOrMethodAccess, ParsedForBlock, ParsedAnonymousFunction, Identifier, ParsedIfBlock, ParsedIterAncestorsBlock, ParsedIterFieldsBlock, ParsedIterVariantsBlock, ParsedMatchBlock, ParsedNoneLiteral, ParsedNumberLiteral, ParsedObjectLiteral, ParsedParenthesizedExpression, ParsedStaticFieldOrMethod, ParsedStringLiteral, ParsedTemplateString, ParsedVarDeclaration, ParsedVarRef, ParsedWhileBlock, ParsedMacroExpression, ParsedPrefixedVarRef, ParsedMacroDebug, ParsedColorLiteral, ParsedLoadDirective};

#[parsable]
pub enum ParsedVarPathRoot {
    LoadDirective(ParsedLoadDirective),
    Macro(ParsedMacroExpression),
    DebugMacro(ParsedMacroDebug),
    VarDeclaration(ParsedVarDeclaration),
    Action(ParsedAction),
    MatchBlock(ParsedMatchBlock),
    IfBlock(ParsedIfBlock),
    IterFields(ParsedIterFieldsBlock),
    IterVariants(ParsedIterVariantsBlock),
    IterAncestors(ParsedIterAncestorsBlock),
    WhileBlock(ParsedWhileBlock),
    ForBlock(ParsedForBlock),
    Block(ParsedBlockExpression),
    NoneLiteral(ParsedNoneLiteral),
    BooleanLiteral(ParsedBooleanLiteral),
    NumberLiteral(ParsedNumberLiteral),
    CharLiteral(ParsedCharLiteral),
    StringLiteral(ParsedStringLiteral),
    TemplateString(ParsedTemplateString),
    ArrayLiteral(ParsedArrayLiteral),
    StaticFieldOrMethod(ParsedStaticFieldOrMethod),
    #[parsable(ignore_if_marker = "no-object")]
    ObjectLiteral(ParsedObjectLiteral),
    ColorLiteral(ParsedColorLiteral),
    FunctionLiteral(ParsedAnonymousFunction),
    #[parsable(unset_marker = "no-object", unset_marker = "no-function-call")]
    Parenthesized(ParsedParenthesizedExpression),
    #[parsable(unset_marker = "no-function-call")]
    PrefixedVarRef(ParsedPrefixedVarRef),
    #[parsable(unset_marker = "no-function-call")]
    VarRef(ParsedVarRef),
}

impl ParsedVarPathRoot {
    fn is_var_ref(&self) -> bool {
        match self {
            ParsedVarPathRoot::VarRef(_) => true,
            _ => false
        }
    }

    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        match self {
            ParsedVarPathRoot::LoadDirective(_) => {},
            ParsedVarPathRoot::Macro(_) => {},
            ParsedVarPathRoot::DebugMacro(_) => {},
            ParsedVarPathRoot::NoneLiteral(_) => {},
            ParsedVarPathRoot::BooleanLiteral(_) => {},
            ParsedVarPathRoot::NumberLiteral(_) => {},
            ParsedVarPathRoot::CharLiteral(_) => {},
            ParsedVarPathRoot::StringLiteral(_) => {},
            ParsedVarPathRoot::ColorLiteral(_) => {},
            ParsedVarPathRoot::ArrayLiteral(array_literal) => array_literal.collect_instancied_type_names(list, context),
            ParsedVarPathRoot::ObjectLiteral(object_literal) => object_literal.collect_instancied_type_names(list, context),
            ParsedVarPathRoot::StaticFieldOrMethod(_) => {},
            ParsedVarPathRoot::Parenthesized(expr) => expr.collect_instancied_type_names(list, context),
            ParsedVarPathRoot::VarRef(var_ref) => var_ref.collect_instancied_type_names(list),
            ParsedVarPathRoot::VarDeclaration(_) => todo!(),
            ParsedVarPathRoot::Action(_) => todo!(),
            ParsedVarPathRoot::MatchBlock(_) => todo!(),
            ParsedVarPathRoot::IfBlock(_) => todo!(),
            ParsedVarPathRoot::IterFields(_) => todo!(),
            ParsedVarPathRoot::IterVariants(_) => todo!(),
            ParsedVarPathRoot::IterAncestors(_) => todo!(),
            ParsedVarPathRoot::WhileBlock(_) => todo!(),
            ParsedVarPathRoot::ForBlock(_) => todo!(),
            ParsedVarPathRoot::Block(_) => todo!(),
            ParsedVarPathRoot::TemplateString(_) => {}, // TODO
            ParsedVarPathRoot::FunctionLiteral(_) => {},
            ParsedVarPathRoot::PrefixedVarRef(prefixed_var_ref) => prefixed_var_ref.collect_instancied_type_names(list, context),
            
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        if let AccessType::Set(location) = access_type {
            if !self.is_var_ref() {
                context.errors.generic(location, format!("invalid assignment"));
            }
        }

        match self {
            ParsedVarPathRoot::LoadDirective(load_directive) => load_directive.process(context),
            ParsedVarPathRoot::Macro(mac) => mac.process(context),
            ParsedVarPathRoot::DebugMacro(mac) => mac.process(context),
            ParsedVarPathRoot::NoneLiteral(none_literal) => none_literal.process(type_hint, context),
            ParsedVarPathRoot::BooleanLiteral(boolean_literal) => boolean_literal.process(context),
            ParsedVarPathRoot::NumberLiteral(number_literal) => number_literal.process(type_hint, context),
            ParsedVarPathRoot::CharLiteral(char_literal) => char_literal.process(context),
            ParsedVarPathRoot::StringLiteral(string_literal) => string_literal.process(context),
            ParsedVarPathRoot::ColorLiteral(color_literal) => color_literal.process(context),
            ParsedVarPathRoot::TemplateString(template_string) => template_string.process(context),
            ParsedVarPathRoot::ArrayLiteral(array_literal) => array_literal.process(type_hint, context),
            ParsedVarPathRoot::ObjectLiteral(object_literal) => object_literal.process(type_hint, context),
            ParsedVarPathRoot::StaticFieldOrMethod(static_field_or_method) => static_field_or_method.process(type_hint, context),
            ParsedVarPathRoot::VarDeclaration(var_declaration) => var_declaration.process(context).map(|(_, vasm)| vasm),
            ParsedVarPathRoot::Action(action) => action.process(context),
            ParsedVarPathRoot::IfBlock(if_block) => if_block.process(type_hint, context),
            ParsedVarPathRoot::IterFields(iter_fields) => iter_fields.process(context),
            ParsedVarPathRoot::IterVariants(iter_variants) => iter_variants.process(context),
            ParsedVarPathRoot::IterAncestors(iter_ancestors) => iter_ancestors.process(context),
            ParsedVarPathRoot::WhileBlock(while_block) => while_block.process(context),
            ParsedVarPathRoot::ForBlock(for_block) => for_block.process(context),
            ParsedVarPathRoot::MatchBlock(match_block) => match_block.process(type_hint, context),
            ParsedVarPathRoot::Parenthesized(expr) => expr.process(type_hint, context),
            ParsedVarPathRoot::PrefixedVarRef(prefixed_var_ref) => prefixed_var_ref.process(type_hint, access_type, context),
            ParsedVarPathRoot::VarRef(var_ref) => var_ref.process(type_hint, access_type, context),
            ParsedVarPathRoot::Block(block) => block.process(type_hint, context),
            ParsedVarPathRoot::FunctionLiteral(function_literal) => function_literal.process(type_hint, context),
        }
    }
}