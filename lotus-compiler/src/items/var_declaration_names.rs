use parsable::{parsable, DataLocation};
use crate::{program::{ProgramContext, Vasm, Type, VI, VariableInfo, TUPLE_FIRST_ASSOCIATED_TYPE_NAME, TUPLE_SECOND_ASSOCIATED_TYPE_NAME, TUPLE_FIRST_METHOD_NAME, TUPLE_SECOND_METHOD_NAME}, vasm};
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
                    context.errors.generic(location, format!("insufficient infered type `{}` (consider declaring the variable type explicitly)", &assigned_vasm.ty));
                }
                
                assigned_vasm.ty.clone()
            },
        };
        
        match &self.content {
            VarDeclarationNamesContent::Single(name) => {
                let var_info = context.declare_local_variable(name.clone(), variable_type.clone());

                Some((
                    vec![var_info.clone()],
                    Vasm::new(Type::Void, vec![var_info.clone()], vec![
                        VI::init_var(&var_info),
                        VI::set_var(&var_info, current_function_level, assigned_vasm)
                    ])
                ))
            },
            VarDeclarationNamesContent::Multiple(names) => {
                if names.len() != 2 {
                    context.errors.generic(self, format!("tuples can only be declared as pairs"));
                    None
                } else {
                    let mut result_vasm = vasm![];
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
                        result_vasm = Vasm::new(Type::Void, vec![tmp_var_info.clone(), var_1.clone(), var_2.clone()], vasm![
                            assigned_vasm,
                            VI::set_tmp_var(&tmp_var_info),
                            VI::init_var(&var_1),
                            VI::init_var(&var_2),
                            VI::set_var(&var_1, current_function_level, vec![
                                VI::get_tmp_var(&tmp_var_info),
                                VI::call_regular_method(&variable_type, TUPLE_FIRST_METHOD_NAME, &[], vec![], context)
                            ]),
                            VI::set_var(&var_2, current_function_level, vec![
                                VI::get_tmp_var(&tmp_var_info),
                                VI::call_regular_method(&variable_type, TUPLE_SECOND_METHOD_NAME, &[], vec![], context)
                            ])
                        ])
                    } else if !variable_type.is_undefined() {
                        context.errors.generic(location, format!("cannot destructure type `{}` into 2 values", &variable_type));
                    }
                    
                    Some((vec![var_1.clone(), var_2.clone()], result_vasm))
                }
            },
        }
    }
}