use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, INT_NONE_VALUE, IS_METHOD_NAME, IS_NONE_METHOD_NAME, NONE_LITERAL, NONE_METHOD_NAME, ProgramContext, ScopeKind, Type, TypeCategory, VariableInfo, VariableKind, Vasm, TypeContent}, wat};
use super::{ParsedExpression, Identifier, ParsedType, ParsedTypeQualifier};

#[parsable]
pub struct ParsedMatchBlock {
    #[parsable(prefix="match")]
    pub value_to_match: Box<ParsedExpression>,
    #[parsable(separator=",", brackets="{}")]
    pub branches: Vec<ParsedMatchBranch>
}

#[parsable]
pub struct ParsedMatchBranch {
    pub variant_name: ParsedType,
    #[parsable(brackets="()")]
    pub var_name: Option<Identifier>,
    #[parsable(prefix="=>")]
    pub expr: ParsedExpression
}

impl ParsedMatchBlock {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(matched_vasm) = self.value_to_match.process(None, context) {
            match &matched_vasm.ty.content() {
                TypeContent::Undefined => {},
                TypeContent::Actual(info) => info.type_blueprint.clone().with_ref(|type_unwrapped| {
                    if !type_unwrapped.is_enum() && !type_unwrapped.is_class() && !matched_vasm.ty.is_bool() {
                        context.errors.generic(&self.value_to_match, format!("expected enum or class type, got `{}`", &matched_vasm.ty));
                    } else {
                        let tmp_var = VariableInfo::tmp("tmp", context.int_type());
                        let result_var = VariableInfo::tmp("result", Type::undefined());
                        let mut returned_type : Option<Type> = None;
                        let mut content = context.vasm();

                        for branch in &self.branches {
                            let mut var_vasm = context.vasm();

                            let test_vasm_opt = match &type_unwrapped.category {
                                TypeCategory::Type => match &branch.variant_name.as_single_identifier() {
                                    Some(name) => match name.as_str() {
                                        NONE_LITERAL | "false" => Some(context.vasm()
                                            .eqz()
                                            .eqz()
                                        ),
                                        "true" => Some(context.vasm()
                                            .eqz()
                                        ),
                                        _ => {
                                            context.errors.generic(&self.value_to_match, format!("type `{}` has no variant `{}`", &matched_vasm.ty, name));
                                            None
                                        },
                                    },
                                    None => {
                                        context.errors.generic(&self.value_to_match, format!("expected variant name"));
                                        None
                                    }
                                },
                                TypeCategory::Class => match branch.variant_name.as_single_identifier().map(|name| name.as_str()).contains(&NONE_LITERAL) {
                                    true => Some(context.vasm()
                                        .call_regular_method(&matched_vasm.ty, IS_NONE_METHOD_NAME, &[], vec![], context)
                                        .eqz()
                                    ),
                                    false => match branch.variant_name.process(true, context) {
                                        Some(ty) => match ty.match_builtin_interface(BuiltinInterface::Object, context) {
                                            true => {
                                                if let Some(var_name) = &branch.var_name {
                                                    let var_info = context.declare_local_variable(var_name.clone(), ty.clone());

                                                    var_vasm = context.vasm()
                                                        .declare_variable(&var_info)
                                                        .get_tmp_var(&tmp_var)
                                                        .set_tmp_var(&var_info)
                                                        .set_type(&ty);
                                                }

                                                Some(context.vasm()
                                                    .call_static_method(&ty, IS_METHOD_NAME, &[], vec![], context)
                                                    .eqz()
                                                )
                                            },
                                            false => {
                                                context.errors.generic(&branch.variant_name, format!("type `{}` is not a class", &ty));
                                                None
                                            },
                                        },
                                        None => None,
                                    }
                                }
                                TypeCategory::Enum => {
                                    match &branch.variant_name.as_single_identifier() {
                                        Some(name) => match name.as_str() {
                                            NONE_LITERAL => Some(context.vasm()
                                                .int(INT_NONE_VALUE)
                                                .raw(wat!["i32.ne"])
                                            ),
                                            _ => match type_unwrapped.enum_variants.get(name.as_str()) {
                                                Some(variant_info) => Some(context.vasm()
                                                    .int(variant_info.value)
                                                    .raw(wat!["i32.ne"])
                                                ),
                                                None => {
                                                    context.errors.generic(&self.value_to_match, format!("enum `{}` has no variant `{}`", &matched_vasm.ty, name));
                                                    None
                                                }
                                            }
                                        },
                                        None => {
                                            context.errors.generic(&self.value_to_match, format!("expected variant name"));
                                            None
                                        }
                                    }
                                },
                            };

                            if let Some(test_vasm) = test_vasm_opt {
                                context.push_scope(ScopeKind::Branch);

                                if let Some(branch_vasm) = branch.expr.process(type_hint, context) {
                                    let new_expected_type = match &returned_type {
                                        Some(ty) => ty.get_common_type(&branch_vasm.ty),
                                        None => Some(branch_vasm.ty.clone()),
                                    };

                                    match new_expected_type {
                                        Some(ty) => {
                                            content = content.block(context.vasm()
                                                .get_tmp_var(&tmp_var)
                                                .append(test_vasm)
                                                .jump_if_from_stack(0)
                                                .append(var_vasm)
                                                .append(branch_vasm)
                                                .set_tmp_var(&result_var)
                                                .jump(1)
                                            );

                                            returned_type = Some(ty);
                                        },
                                        None => {
                                            context.errors.type_mismatch(&branch.expr, returned_type.as_ref().unwrap() , &branch_vasm.ty);
                                        },
                                    }
                                }
                                context.pop_scope();
                            }
                        }

                        let final_type = returned_type.unwrap_or(context.void_type());

                        result_var.set_type(final_type.clone());

                        if !final_type.is_undefined() {
                            content = content
                                .call_static_method(&final_type, NONE_METHOD_NAME, &[], vec![], context)
                                .set_tmp_var(&result_var);
                        }

                        let branches_vasm = context.vasm()
                            .declare_variable(&tmp_var)
                            .declare_variable(&result_var)
                            .set_tmp_var(&tmp_var)
                            .block(content)
                            .get_tmp_var(&result_var)
                            .set_type(&final_type);

                        result = Some(context.vasm()
                            .append(matched_vasm)
                            .append(branches_vasm)
                        );
                    }
                }),
                _ => {
                    context.errors.generic(&self.value_to_match, format!("expected enum type, got `{}`", &matched_vasm.ty));
                }
            }
        }

        result
    }
}