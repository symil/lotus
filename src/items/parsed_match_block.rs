use std::iter::FromIterator;

use colored::Colorize;
use enum_iterator::IntoEnumIterator;
use indexmap::IndexSet;
use parsable::{parsable, Token};
use crate::{program::{BuiltinInterface, INT_NONE_VALUE, IS_METHOD_NAME, IS_NONE_METHOD_NAME, NONE_LITERAL, NONE_METHOD_NAME, ProgramContext, ScopeKind, Type, TypeCategory, VariableInfo, VariableKind, Vasm, TypeContent, MATCH_KEYWORD}, wat};
use super::{ParsedExpression, Identifier, ParsedType, ParsedTypeQualifier, ParsedDoubleColonToken, ParsedOpeningRoundBracket, ParsedClosingRoundBracket, ParsedArrowToken, ParsedNoneLiteral, ParsedNumberLiteral, ParsedStringLiteral, ParsedCharLiteral, ParsedMatchBranchItem, ParsedMatchBranchBody, ParsedVarDeclarationNames, ParsedOpeningCurlyBracket, ParsedClosingCurlyBracket, ParsedBooleanLiteralToken, unwrap_item};

#[parsable]
pub struct ParsedMatchBlock {
    pub match_keyword: Token<MATCH_KEYWORD>,
    #[parsable(declare_marker="no-object")]
    pub expression: Option<Box<ParsedExpression>>,
    pub body: Option<ParsedMatchBody>,
}

#[parsable]
pub struct ParsedMatchBody {
    pub opening_bracket: ParsedOpeningCurlyBracket,
    pub branches: ParsedMatchBranchList,
    pub closing_bracket: ParsedClosingCurlyBracket
}

#[parsable]
pub struct ParsedMatchBranchList {
    #[parsable(separator=",")]
    pub list: Vec<ParsedMatchBranch>
}

#[parsable]
pub struct ParsedMatchBranch {
    pub item: ParsedMatchBranchItem,
    #[parsable(brackets="()")]
    pub variable: Option<ParsedVarDeclarationNames>,
    pub body: Option<ParsedMatchBranchBody>,
}

impl ParsedMatchBlock {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let expression = unwrap_item(&self.expression, &self.match_keyword, context)?;
        let matched_vasm = expression.process(None, context)?;
        let matched_type = matched_vasm.ty.clone();
        let tmp_var = VariableInfo::tmp("tmp", matched_type.clone());
        let result_var = VariableInfo::tmp("result", Type::undefined());
        let tested_vasm = context.vasm()
            .get_tmp_var(&tmp_var)
            .set_type(&matched_vasm.ty);
        let mut returned_type : Option<Type> = None;
        let mut vasm = context.vasm()
            .declare_variable(&tmp_var)
            .declare_variable(&result_var)
            .append(matched_vasm)
            .set_tmp_var(&tmp_var)
            .set_type(tmp_var.ty().clone());
        let mut content_vasm = context.vasm();

        let body = match &self.body {
            Some(body) => body,
            None => {
                context.errors.expected_block(self);
                return None;
            },
        };

        let first_half_location = body.opening_bracket.location.until(&body.branches);
        let second_half_location = body.branches.location.until(&body.closing_bracket);

        context.add_match_item_completion_area(&first_half_location, &matched_type);
        context.add_match_item_completion_area(&second_half_location, &matched_type);

        for branch in &body.branches.list {
            let mut branch_vasm = context.vasm();

            if let Some((item_type, item_vasm)) = branch.item.process(tested_vasm.clone(), context) {
                context.push_scope(ScopeKind::Branch);

                branch_vasm = branch_vasm
                    .jump_if(0, item_vasm.eqz());

                if let Some(variable) = &branch.variable {
                    if let Some((_, var_init_vasm)) = variable.process(None, tested_vasm.clone().set_type(item_type), None, context) {
                        branch_vasm = branch_vasm.append(var_init_vasm);
                    }
                }

                if let Some(body) = &branch.body {
                    let hint = type_hint.or(returned_type.as_ref());

                    if let Some(body_vasm) = body.process(hint, context) {
                        branch_vasm = branch_vasm
                            .append(body_vasm)
                            .set_tmp_var(&result_var)
                            .jump(1);
                        
                        returned_type = match &returned_type {
                            Some(current_returned_type) => match current_returned_type.get_common_type(&branch_vasm.ty) {
                                Some(common_type) => Some(common_type),
                                None => {
                                    context.errors.type_mismatch(body.expression.as_ref().unwrap(), &current_returned_type , &branch_vasm.ty);
                                    returned_type.clone()
                                },
                            },
                            None => Some(branch_vasm.ty.clone()),
                        };
                    }
                } else {
                    context.errors.expected_token(branch, "=>");
                }

                context.pop_scope();
            }

            if !branch.item.is_enum_variant() {
                context.add_match_item_completion_area(&branch.item, &matched_type);
            }

            vasm = vasm.block(branch_vasm);
        }

        let generate_fill_match_arms = || {
            let mut result = None;
            let all_variants = if matched_type.is_enum() {
                let type_name = matched_type.to_string();

                matched_type.get_all_variants().iter()
                    .map(|variant_info| format!("{}::{}", type_name, variant_info.name.as_str()))
                    .collect::<Vec<String>>()
            } else if matched_type.is_bool() {
                ParsedBooleanLiteralToken::into_enum_iter()
                    .map(|value| value.as_str().to_string())
                    .collect::<Vec<String>>()
            } else {
                vec![]
            };
            let mut variant_set : IndexSet<String> = IndexSet::from_iter(all_variants);

            for branch in &body.branches.list {
                if let Some(variant) = branch.item.get_variant_name() {
                    variant_set.remove(&variant);
                }
            }

            if !variant_set.is_empty() {
                result = Some(variant_set.iter()
                    .map(|variant| format!("{} => none,", variant))
                    .collect::<Vec<String>>()
                    .join("\n")
                );
            }

            result
        };

        context.code_actions_provider.add_replace_action(&first_half_location, "Fill match arms", None, generate_fill_match_arms);
        context.code_actions_provider.add_replace_action(&second_half_location, "Fill match arms", None, generate_fill_match_arms);

        let final_type = returned_type.unwrap_or(context.void_type());

        result_var.set_type(final_type.clone());

        if !final_type.is_undefined() {
            vasm = vasm
                .call_static_method(&final_type, NONE_METHOD_NAME, &[], vec![], context)
                .set_tmp_var(&result_var);
        }

        Some(context.vasm()
            .block(vasm)
            .get_tmp_var(&result_var)
            .set_type(&final_type)
        )
    }
}