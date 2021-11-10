use colored::Colorize;
use parsable::parsable;
use crate::{program::{INT_NONE_VALUE, NONE_LITERAL, NONE_METHOD_NAME, ProgramContext, ScopeKind, Type, VI, VariableInfo, VariableKind, Vasm}, vasm, wat};
use super::{Expression, Identifier, TypeQualifier};

#[parsable]
pub struct MatchBlock {
    #[parsable(prefix="match")]
    pub value_to_match: Box<Expression>,
    #[parsable(separator=",",brackets="{}")]
    pub branches: Vec<MatchBranch>
}

#[parsable]
pub struct MatchBranch {
    pub variant_name: Identifier,
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
                    if type_unwrapped.qualifier == TypeQualifier::Enum {
                        let tmp_var = VariableInfo::new(Identifier::unique("tmp", self), context.int_type(), VariableKind::Local);
                        let result_var = VariableInfo::new(Identifier::unique("result", self), type_hint.cloned().unwrap_or(context.int_type()), VariableKind::Local);
                        let mut returned_type : Option<Type> = None;
                        let mut content = vec![];

                        for branch in &self.branches {
                            let variant_int_value = if branch.variant_name.as_str() == NONE_LITERAL {
                                Some(INT_NONE_VALUE)
                            } else {
                                type_unwrapped.enum_variants.get(branch.variant_name.as_str()).and_then(|info| Some(info.value as i32))
                            };

                            if let Some(value) = variant_int_value {
                                context.push_scope(ScopeKind::Branch);
                                if let Some(branch_vasm) = branch.expr.process(type_hint, context) {
                                    let new_expected_type = match type_hint {
                                        Some(expected_type) => match branch_vasm.ty.is_assignable_to(expected_type) {
                                            true => Some(expected_type.clone()),
                                            false => None,
                                        },
                                        None => match &returned_type {
                                            Some(ty) => ty.get_common_type(&branch_vasm.ty).cloned(),
                                            None => Some(branch_vasm.ty.clone()),
                                        },
                                    };

                                    match new_expected_type {
                                        Some(ty) => {
                                            let vasm = VI::block(vasm![
                                                VI::get_var(&tmp_var),
                                                VI::int(value),
                                                VI::raw(wat!["i32.ne"]),
                                                VI::jump_if_from_stack(0),
                                                branch_vasm,
                                                VI::set_var_from_stack(&result_var),
                                                VI::jump(1)
                                            ]);

                                            content.push(vasm);
                                            returned_type = Some(ty);
                                        },
                                        None => {
                                            context.errors.add(&branch.expr, format!("expected `{}`, got `{}`", type_hint.or(returned_type.as_ref()).unwrap() , &branch_vasm.ty))
                                        },
                                    }
                                }
                                context.pop_scope();
                            } else {
                                context.errors.add(&self.value_to_match, format!("enum `{}` has no variant `{}`", &matched_vasm.ty, branch.variant_name.as_str().bold()));
                            }
                        }

                        let final_type = returned_type.unwrap_or(Type::Void);

                        if !final_type.is_void() && !final_type.is_undefined() {
                            content.extend(vec![
                                VI::call_static_method(&final_type, NONE_METHOD_NAME, &[], vec![], context),
                                VI::set_var_from_stack(&result_var)
                            ]);
                        }

                        let branches_vasm = Vasm::new(final_type.clone(), vec![tmp_var.clone(), result_var.clone()], vec![
                            VI::set_var_from_stack(&tmp_var),
                            VI::block(content),
                            VI::get_var(&result_var)
                        ]);

                        result = Some(Vasm::merge(vec![matched_vasm, branches_vasm]));
                    } else {
                        context.errors.add(&self.value_to_match, format!("expected enum type, got `{}`", &matched_vasm.ty));
                    }
                }),
                _ => {
                    context.errors.add(&self.value_to_match, format!("expected enum type, got `{}`", &matched_vasm.ty));
                }
            }
        }

        result
    }
}