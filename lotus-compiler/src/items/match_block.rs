use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, INT_NONE_VALUE, IS_METHOD_NAME, IS_NONE_METHOD_NAME, NONE_LITERAL, NONE_METHOD_NAME, ProgramContext, ScopeKind, Type, TypeCategory, VI, VariableInfo, VariableKind, Vasm}, vasm, wat};
use super::{Expression, Identifier, ParsedType, TypeQualifier, type_qualifier};

#[parsable]
pub struct MatchBlock {
    #[parsable(prefix="match")]
    pub value_to_match: Box<Expression>,
    #[parsable(separator=",",brackets="{}")]
    pub branches: Vec<MatchBranch>
}

#[parsable]
pub struct MatchBranch {
    pub variant_name: ParsedType,
    #[parsable(brackets="()")]
    pub var_name: Option<Identifier>,
    #[parsable(prefix="=>")]
    pub expr: Expression
}

impl MatchBlock {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(matched_vasm) = self.value_to_match.process(None, context) {
            match &matched_vasm.ty {
                Type::Undefined => {},
                Type::Actual(info) => info.type_blueprint.clone().with_ref(|type_unwrapped| {
                    if !type_unwrapped.is_enum() && !type_unwrapped.is_class() && !matched_vasm.ty.is_bool() {
                        context.errors.add_generic(&self.value_to_match, format!("expected enum or class type, got `{}`", &matched_vasm.ty));
                    } else {
                        let tmp_var = VariableInfo::tmp("tmp", context.int_type());
                        let result_var = VariableInfo::tmp("result", Type::Undefined);
                        let mut returned_type : Option<Type> = None;
                        let mut content = vec![];

                        for branch in &self.branches {
                            let mut var_vasm = vasm![];

                            let test_vasm_opt = match &type_unwrapped.category {
                                TypeCategory::Type => match &branch.variant_name.as_single_identifier() {
                                    Some(name) => match name.as_str() {
                                        NONE_LITERAL | "false" => Some(vasm![
                                            VI::raw(wat!["i32.eqz"]),
                                            VI::raw(wat!["i32.eqz"])
                                        ]),
                                        "true" => Some(vasm![
                                            VI::raw(wat!["i32.eqz"])
                                        ]),
                                        _ => context.errors.add_generic_and_none(&self.value_to_match, format!("type `{}` has no variant `{}`", &matched_vasm.ty, name)),
                                    },
                                    None => context.errors.add_generic_and_none(&self.value_to_match, format!("expected variant name")),
                                },
                                TypeCategory::Class => match branch.variant_name.as_single_identifier().map(|name| name.as_str()).contains(&NONE_LITERAL) {
                                    true => Some(vasm![
                                        VI::call_regular_method(&matched_vasm.ty, IS_NONE_METHOD_NAME, &[], vec![], context),
                                        VI::Eqz
                                    ]),
                                    false => match branch.variant_name.process(true, context) {
                                        Some(ty) => match ty.match_builtin_interface(BuiltinInterface::Object, context) {
                                            true => {
                                                if let Some(var_name) = &branch.var_name {
                                                    let var_info = context.declare_local_variable(var_name.clone(), ty.clone());

                                                    var_vasm = Vasm::new(ty.clone(), vec![var_info.clone()], vec![
                                                        VI::get_tmp_var(&tmp_var),
                                                        VI::set_tmp_var(&var_info)
                                                    ]);
                                                }

                                                Some(vasm![
                                                    VI::call_static_method(&ty, IS_METHOD_NAME, &[], vec![], context),
                                                    VI::Eqz
                                                ])
                                            },
                                            false => context.errors.add_generic_and_none(&branch.variant_name, format!("type `{}` is not a class", &ty)),
                                        },
                                        None => None,
                                    }
                                }
                                TypeCategory::Enum => {
                                    match &branch.variant_name.as_single_identifier() {
                                        Some(name) => match name.as_str() {
                                            NONE_LITERAL => Some(vasm![
                                                VI::int(INT_NONE_VALUE),
                                                VI::raw(wat!["i32.ne"])
                                            ]),
                                            _ => match type_unwrapped.enum_variants.get(name.as_str()) {
                                                Some(variant_info) => Some(vasm![
                                                    VI::int(variant_info.value),
                                                    VI::raw(wat!["i32.ne"])
                                                ]),
                                                None => context.errors.add_generic_and_none(&self.value_to_match, format!("enum `{}` has no variant `{}`", &matched_vasm.ty, name)),
                                            }
                                        },
                                        None => context.errors.add_generic_and_none(&self.value_to_match, format!("expected variant name")),
                                    }
                                },
                            };

                            if let Some(test_vasm) = test_vasm_opt {
                                context.push_scope(ScopeKind::Branch);

                                if let Some(branch_vasm) = branch.expr.process(type_hint, context) {
                                    let new_expected_type = match &returned_type {
                                        Some(ty) => ty.get_common_type(&branch_vasm.ty).cloned(),
                                        None => Some(branch_vasm.ty.clone()),
                                    };

                                    match new_expected_type {
                                        Some(ty) => {
                                            let vasm = VI::block(vasm![
                                                VI::get_tmp_var(&tmp_var),
                                                test_vasm,
                                                VI::jump_if_from_stack(0),
                                                var_vasm,
                                                branch_vasm,
                                                VI::set_tmp_var(&result_var),
                                                VI::jump(1)
                                            ]);

                                            content.push(vasm);
                                            returned_type = Some(ty);
                                        },
                                        None => {
                                            context.errors.add_generic(&branch.expr, format!("expected `{}`, got `{}`", returned_type.as_ref().unwrap() , &branch_vasm.ty))
                                        },
                                    }
                                }
                                context.pop_scope();
                            }
                        }

                        let final_type = returned_type.unwrap_or(context.void_type());

                        result_var.set_type(final_type.clone());

                        if !final_type.is_undefined() {
                            content.extend(vec![
                                VI::call_static_method(&final_type, NONE_METHOD_NAME, &[], vec![], context),
                                VI::set_tmp_var(&result_var)
                            ]);
                        }

                        let branches_vasm = Vasm::new(final_type.clone(), vec![tmp_var.clone(), result_var.clone()], vec![
                            VI::set_tmp_var(&tmp_var),
                            VI::block(content),
                            VI::get_tmp_var(&result_var)
                        ]);

                        result = Some(Vasm::merge(vec![matched_vasm, branches_vasm]));
                    }
                }),
                _ => {
                    context.errors.add_generic(&self.value_to_match, format!("expected enum type, got `{}`", &matched_vasm.ty));
                }
            }
        }

        result
    }
}