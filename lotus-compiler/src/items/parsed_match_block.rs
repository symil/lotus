use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, INT_NONE_VALUE, IS_METHOD_NAME, IS_NONE_METHOD_NAME, NONE_LITERAL, NONE_METHOD_NAME, ProgramContext, ScopeKind, Type, TypeCategory, VariableInfo, VariableKind, Vasm, TypeContent}, wat};
use super::{ParsedExpression, Identifier, ParsedType, ParsedTypeQualifier, ParsedDoubleColon, ParsedOpeningRoundBracket, ParsedClosingRoundBracket, ParsedArrow, ParsedNoneLiteral, ParsedNumberLiteral, ParsedStringLiteral, ParsedCharLiteral, ParsedMatchBlockItem, ParsedMatchBlockBody, ParsedVarDeclarationNames};

#[parsable]
pub struct ParsedMatchBlock {
    #[parsable(prefix="match")]
    pub value_to_match: Box<ParsedExpression>,
    #[parsable(separator=",", brackets="{}")]
    pub branches: Vec<ParsedMatchBranch>
}

#[parsable]
pub struct ParsedMatchBranch {
    pub item: ParsedMatchBlockItem,
    #[parsable(brackets="()")]
    pub variable: Option<ParsedVarDeclarationNames>,
    pub body: Option<ParsedMatchBlockBody>,
}

impl ParsedMatchBlock {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let matched_vasm = self.value_to_match.process(None, context)?;
        let tmp_var = VariableInfo::tmp("tmp", matched_vasm.ty.clone());
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

        for branch in &self.branches {
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
                    if let Some(body_vasm) = body.process(type_hint, context) {
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
                }

                context.pop_scope();
            }

            vasm = vasm.block(branch_vasm);
        }

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