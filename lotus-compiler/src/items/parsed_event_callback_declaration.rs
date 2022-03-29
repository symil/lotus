use colored::Colorize;
use indexmap::IndexMap;
use parsable::{parsable, ItemLocation};
use crate::{program::{FunctionBlueprint, ProgramContext, EVENT_VAR_NAME, EVENT_OUTPUT_VAR_NAME, Signature, BuiltinType, MethodDetails, EventCallbackDetails, Vasm, ScopeKind, SELF_VAR_NAME, Visibility, MethodQualifier, FunctionBody, FieldVisibility, ArgumentInfo, SELF_TYPE_NAME}, utils::Link, wat};
use super::{ParsedEventCallbackQualifierKeyword, Identifier, ParsedExpression, ParsedBlockExpression, ParsedVisibilityToken};

#[parsable]
pub struct ParsedEventCallbackDeclaration {
    pub event_callback_qualifier: ParsedEventCallbackQualifierKeyword,
    pub name: Option<Identifier>,
    #[parsable(brackets="[]")]
    pub priority: Option<ParsedExpression>,
    pub body: Option<ParsedBlockExpression>
}

impl ParsedEventCallbackDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Link<FunctionBlueprint>> {
        let this_type = context.get_current_type().unwrap();
        let type_id = this_type.borrow().type_id;
        let qualifier = self.event_callback_qualifier.process();

        context.add_event_completion_area(&self.event_callback_qualifier, self.body.is_none());

        if let Some(name) = &self.name {
            context.add_event_completion_area(name, self.body.is_none());
        }

        let (name, event_type) = match &self.name {
            Some(name) => {
                let type_wrapped = match name.as_str() {
                    SELF_TYPE_NAME => Some(this_type.clone()),
                    _ => context.types.get_by_identifier(name)
                };

                match type_wrapped {
                    Some(event_type) => {
                        let event_class_name = BuiltinType::Event.get_name();
                        let is_valid_event = event_type.borrow().self_type.inherits_from(event_class_name);

                        match is_valid_event {
                            true => (name, event_type),
                            false => {
                                context.errors.generic(name, format!("type `{}` does not inherit from `{}`", event_type.borrow().name.as_str(), event_class_name));
                                return None;
                            },
                        }
                    },
                    None => {
                        context.errors.undefined_type(name);
                        return None;
                    }
                }
            },
            None => {
                context.errors.expected_identifier(&self.event_callback_qualifier);
                return None;
            }
        };

        if name.as_str() != SELF_TYPE_NAME {
            context.rename_provider.add_occurence(name, &event_type.borrow().name);
        }
        
        context.definition_provider.set_definition(name, &event_type.borrow().name);

        let priority_vasm = match &self.priority {
            Some(expression) => match expression.process(Some(&context.int_type()), context) {
                Some(vasm) => {
                    if !vasm.ty.is_int() {
                        context.errors.type_mismatch(expression, &context.int_type(), &vasm.ty);
                    }

                    vasm
                },
                None => context.vasm(),
            },
            None => context.vasm().int(qualifier.get_default_priority())
        };

        let event_argument = ArgumentInfo {
            name: Identifier::new(EVENT_VAR_NAME, None),
            ty: event_type.borrow().self_type.clone(),
            is_optional: false,
            default_value: context.vasm()
        };
        let output_argument = ArgumentInfo {
            name: Identifier::new(EVENT_OUTPUT_VAR_NAME, None),
            ty: context.get_builtin_type(BuiltinType::EventOutput, vec![]),
            is_optional: false,
            default_value: context.vasm()
        };
        let arguments = vec![ event_argument, output_argument ];
        let signature = Signature::create(
            Some(this_type.borrow().self_type.clone()),
            arguments.iter().map(|arg| arg.ty.clone()).collect(),
            context.void_type()
        );

        let function_blueprint = FunctionBlueprint {
            name: name.clone(),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            arguments,
            signature,
            argument_variables: vec![],
            owner_type: Some(this_type.clone()),
            owner_interface: None,
            closure_details: None,
            method_details: Some(MethodDetails {
                qualifier: MethodQualifier::None,
                visibility: FieldVisibility::Private,
                event_callback_details: Some(EventCallbackDetails {
                    event_type: event_type.clone(),
                    qualifier: qualifier,
                    priority: priority_vasm,
                }),
                first_declared_by: Some(this_type.clone()),
                dynamic_index: None,
                is_autogen: false
            }),
            is_default_function: false,
            body: FunctionBody::Empty,
        };

        let function_wrapped = context.functions.insert(function_blueprint, None);

        if let Some(body) = &self.body {
            let return_type = context.void_type();
            
            context.push_scope(ScopeKind::Function(function_wrapped.clone()));

            if let Some(vasm) = body.process(Some(&return_type), context) {
                let mut body_vasm = context.vasm()
                    .append(vasm)
                    .set_void(context);

                function_wrapped.with_mut(|mut function_unwrapped| {
                    if let Some(event_field_name) = qualifier.get_event_field_name() {
                        let event_var_info = function_unwrapped.argument_variables.iter().find(|var_info| var_info.name().is(EVENT_VAR_NAME)).unwrap();
                        let self_var_info = function_unwrapped.argument_variables.iter().find(|var_info| var_info.name().is(SELF_VAR_NAME)).unwrap();

                        match event_var_info.ty().get_field(event_field_name) {
                            Some(field_info) => {
                                match field_info.ty.is_object() {
                                    true => {
                                        let current_function_level = Some(context.get_function_level());

                                        body_vasm = context.vasm()
                                            .if_then_else(None,
                                                context.vasm()
                                                    .get_var(&event_var_info, current_function_level)
                                                    .get_field(&field_info.ty, field_info.offset)
                                                    .get_var(&self_var_info, current_function_level)
                                                    .raw(wat!["i32.eq"]),
                                                context.vasm(),
                                                context.vasm()
                                                    .return_value(context.vasm().none(&return_type, context)))
                                            .append(body_vasm);
                                    },
                                    false => {
                                        context.errors.generic(&self.event_callback_qualifier, format!("field `{}` of type `{}` is not an object", event_field_name.bold(), event_var_info.ty()));
                                    },
                                }
                            },
                            None => {
                                context.errors.generic(&self.event_callback_qualifier, format!("type `{}` has no field `{}`", event_var_info.ty(), event_field_name.bold()));
                            },
                        }
                    }

                    function_unwrapped.body = FunctionBody::Vasm(body_vasm);
                });
            }

            context.pop_scope();
        } else {
            context.errors.expected_function_body(self);
        }

        Some(function_wrapped)
    }
}