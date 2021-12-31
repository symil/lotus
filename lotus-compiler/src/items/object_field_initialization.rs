use colored::Colorize;
use parsable::parsable;
use crate::{items::ObjectInitResult, program::{CompilationError, ProgramContext, Type, VI, Vasm}};
use super::{Expression, Identifier};

#[parsable]
pub struct ObjectFieldInitialization {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub value: Option<Expression>
}

impl ObjectFieldInitialization {
    pub fn process(&self, object_type: &Type, context: &mut ProgramContext) -> ObjectInitResult {
        let result = match object_type.get_field(self.name.as_str()) {
            Some(field_info) => {
                let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);

                match &self.value {
                    Some(expr) => {
                        context.access_shared_identifier(&field_info.name, &self.name);
                        
                        match expr.process(Some(&field_type), context) {
                            Some(vasm) => match vasm.ty.is_assignable_to(&field_type) {
                                true => {
                                    Some(vasm)
                                },
                                false => {
                                    context.errors.type_mismatch(expr, &field_type, &vasm.ty);
                                    None
                                }
                            },
                            None => None,
                        }
                    },
                    None => match context.access_var(&self.name) {
                        Some(var_info) => {
                            Some(Vasm::new(field_type, vec![], vec![VI::get_var(&var_info, Some(context.get_function_level()))]))
                        },
                        None => {
                            context.errors.generic(&self.name, format!("undefined variable `{}`", &self.name.as_str().bold()));
                            None
                        },
                    }
                }
            },
            None => {
                context.errors.generic(&self.name, format!("type `{}` has no field `{}`", &object_type, self.name.as_str().bold()));
                None
            },
        };

        let mut object_init_result = ObjectInitResult::default();

        if let Some(vasm) = result {
            object_init_result.fields.push((self.name.to_string(), vasm));
        }

        object_init_result
    }
}