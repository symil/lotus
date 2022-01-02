use parsable::{parsable, DataLocation};
use crate::{program::{ProgramContext, Vasm, Type, VI, VariableInfo, TUPLE_FIRST_ASSOCIATED_TYPE_NAME, TUPLE_SECOND_ASSOCIATED_TYPE_NAME, TUPLE_FIRST_METHOD_NAME, TUPLE_SECOND_METHOD_NAME}};
use super::Identifier;

#[parsable]
pub struct VarDeclarationNames {
    pub content: VarDeclarationNamesContent
}

#[parsable]
pub enum VarDeclarationNamesContent {
    Single(Identifier),
    #[parsable(brackets="()", separator=",")]
    Multiple(Vec<Identifier>)
}

impl VarDeclarationNames {
    pub fn process(&self, required_type: Option<&Type>, assigned_vasm: Vasm, location: &DataLocation, context: &mut ProgramContext) -> Option<(Vec<VariableInfo>, Vasm)> {
        let current_function_level = Some(context.get_function_level());
        let variable_type = match required_type {
            Some(ty) => {
                if !assigned_vasm.ty.is_assignable_to(ty) {
                    context.errors.type_mismatch(location, ty, &assigned_vasm.ty);
                }

                ty.clone()
            },
            None => {
                if assigned_vasm.ty.is_ambiguous() {
                    context.errors.generic(self, format!("insufficient infered type `{}` (consider declaring the variable type explicitly)", &assigned_vasm.ty));
                }
                
                assigned_vasm.ty.clone()
            },
        };
        
        match &self.content {
            VarDeclarationNamesContent::Single(name) => {
                let var_info = context.declare_local_variable(name.clone(), variable_type.clone());

                Some((
                    vec![var_info.clone()],
                    context.vasm()
                        .declare_variable(&var_info)
                        .init_var(&var_info)
                        .set_var(&var_info, current_function_level, assigned_vasm)
                        .set_type(context.void_type())
                ))
            },
            VarDeclarationNamesContent::Multiple(names) => {
                if names.len() != 2 {
                    context.errors.generic(self, format!("tuples can only be declared as pairs"));
                    None
                } else {
                    // if names[0].as_str() == "a" && names[0].as_str() == "b" {
                    //     assigned_vasm.ty.print();
                    // }

                    let mut result_vasm = context.vasm();
                    let tmp_var_info = VariableInfo::tmp("tmp", variable_type.clone());
                    let var_1 = context.declare_local_variable(names[0].clone(), Type::Undefined);
                    let var_2 = context.declare_local_variable(names[1].clone(), Type::Undefined);

                    if let Some(first_type) = variable_type.get_associated_type(TUPLE_FIRST_ASSOCIATED_TYPE_NAME) {
                        var_1.set_type(first_type);
                    }

                    if let Some(second_type) = variable_type.get_associated_type(TUPLE_SECOND_ASSOCIATED_TYPE_NAME) {
                        var_2.set_type(second_type);
                    }

                    if !var_1.ty().is_undefined() && !var_2.ty().is_undefined() {
                        result_vasm = context.vasm()
                            .declare_variable(&tmp_var_info)
                            .declare_variable(&var_1)
                            .declare_variable(&var_2)
                            .append(assigned_vasm)
                            .set_tmp_var(&tmp_var_info)
                            .init_var(&var_1)
                            .init_var(&var_2)
                            .set_var(&var_1, current_function_level, context.vasm()
                                .get_tmp_var(&tmp_var_info)
                                .call_regular_method(&variable_type, TUPLE_FIRST_METHOD_NAME, &[], vec![], context)
                            )
                            .set_var(&var_2, current_function_level, context.vasm()
                                .get_tmp_var(&tmp_var_info)
                                .call_regular_method(&variable_type, TUPLE_SECOND_METHOD_NAME, &[], vec![], context)
                            )
                            .set_type(context.void_type());
                    } else if !variable_type.is_undefined() {
                        context.errors.generic(self, format!("cannot destructure type `{}` into 2 values", &variable_type));
                    }
                    
                    Some((vec![var_1.clone(), var_2.clone()], result_vasm))
                }
            },
        }
    }
}