use colored::Colorize;
use parsable::parsable;
use crate::{items::ObjectInitResult, program::{CompilationError, ProgramContext, Type, Vasm}};
use super::{ParsedExpression, Identifier, ParsedColonToken};

#[parsable]
pub struct ParsedObjectFieldInitialization {
    pub name: Identifier,
    pub value: Option<ParsedObjectFieldInitializationValue>
}

#[parsable]
pub struct ParsedObjectFieldInitializationValue {
    pub colon: ParsedColonToken,
    pub expression: Option<ParsedExpression>
}

impl ParsedObjectFieldInitialization {
    pub fn process(&self, object_type: &Type, context: &mut ProgramContext) -> ObjectInitResult {
        context.completion_provider.add_field_completion(&self.name, object_type, false, false);

        let result = match object_type.get_field(self.name.as_str()) {
            Some(field_info) => {
                let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);

                context.rename_provider.add_occurence(&self.name, &field_info.name);
                context.definition_provider.set_definition(&self.name, &field_info.name);
                context.hover_provider.set_type(&self.name, &field_type);

                match &self.value {
                    Some(value) => {
                        match &value.expression {
                            Some(expr) => {
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
                            None => {
                                context.errors.expected_expression(value);
                                None
                            },
                        }
                    },
                    None => match context.access_var(&self.name) {
                        Some(var_info) => {
                            Some(context.vasm()
                                .get_var(&var_info, Some(context.get_function_level()))
                                .set_type(field_type)
                            )
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