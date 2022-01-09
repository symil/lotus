use colored::Colorize;
use indexmap::IndexMap;
use parsable::{parsable, DataLocation};
use crate::{program::{FunctionBlueprint, ProgramContext, EVENT_VAR_NAME, EVENT_OUTPUT_VAR_NAME, Signature, BuiltinType, MethodDetails, EventCallbackDetails, Vasm, ScopeKind, SELF_VAR_NAME, Visibility, MethodQualifier, FunctionBody}, utils::Link, wat};
use super::{EventCallbackQualifierKeyword, Identifier, Expression, BlockExpression, VisibilityKeywordValue};

#[parsable]
pub struct EventCallbackDeclaration {
    pub event_callback_qualifier: EventCallbackQualifierKeyword,
    pub name: Option<Identifier>,
    #[parsable(brackets="[]")]
    pub priority: Option<Expression>,
    pub body: Option<BlockExpression>
}

impl EventCallbackDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Link<FunctionBlueprint>> {
        let this_type = context.get_current_type().unwrap();
        let type_id = this_type.borrow().type_id;
        let qualifier = self.event_callback_qualifier.process();

        context.add_event_completion_area(&self.event_callback_qualifier);

        let (name, event_type) = match &self.name {
            Some(name) => match context.types.get_by_identifier(name) {
                Some(event_type) => {
                    let event_class_name = BuiltinType::Event.get_name();
                    let is_valid_event = event_type.borrow().self_type.inherits_from(event_class_name);

                    match is_valid_event {
                        true => (name, event_type),
                        false => {
                            return context.errors.generic(name, format!("type `{}` does not inherit from `{}`", event_type.borrow().name.as_str(), event_class_name)).none();
                        },
                    }
                },
                None => return context.errors.undefined_type(name).none()
            },
            None => return context.errors.expected_identifier(&self.event_callback_qualifier).none()
        };

        context.rename_provider.add_occurence(name, &event_type.borrow().name);
        context.hover_provider.set_definition(name, &event_type.borrow().name);
        context.add_event_completion_area(name);

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

        let function_blueprint = FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: name.clone(),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            argument_names: vec![
                Identifier::new(EVENT_VAR_NAME, None),
                Identifier::new(EVENT_OUTPUT_VAR_NAME, None),
            ],
            signature: Signature::create(
                Some(this_type.borrow().self_type.clone()),
                vec![
                    event_type.borrow().self_type.clone(),
                    context.get_builtin_type(BuiltinType::EventOutput, vec![])
                ],
                context.void_type()
            ),
            argument_variables: vec![],
            owner_type: Some(this_type.clone()),
            owner_interface: None,
            closure_details: None,
            method_details: Some(MethodDetails {
                qualifier: MethodQualifier::None,
                event_callback_details: Some(EventCallbackDetails {
                    event_type: event_type.clone(),
                    qualifier: qualifier,
                    priority: priority_vasm,
                }),
                first_declared_by: Some(this_type.clone()),
                dynamic_index: None,
            }),
            body: FunctionBody::Empty,
        };

        let function_wrapped = Link::new(function_blueprint);

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