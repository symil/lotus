use colored::Colorize;
use indexmap::IndexMap;
use parsable::{parsable, ItemLocation};
use crate::{program::{FunctionBlueprint, ProgramContext, EVENT_VAR_NAME, EVENT_OPTIONS_VAR_NAME, Signature, BuiltinType, MethodDetails, Vasm, ScopeKind, SELF_VAR_NAME, Visibility, MethodQualifier, FunctionBody, FieldVisibility, ArgumentInfo, SELF_TYPE_NAME, EventCallbackStep, FunctionKind, EventCallback}, utils::Link, wat};
use super::{ParsedEventCallbackQualifierKeyword, Identifier, ParsedExpression, ParsedBlockExpression, ParsedVisibilityToken, ParsedEventCallbackIndex, FlexPrefixedWordItem, ParsedColonToken, ParsedEventCallbackEventStep};

#[parsable]
pub struct ParsedEventCallbackDeclaration {
    pub event_callback_qualifier: ParsedEventCallbackQualifierKeyword,
    pub name: Option<Identifier>,
    pub step: Option<FlexPrefixedWordItem<ParsedColonToken, ParsedEventCallbackEventStep>>,
    pub index: Option<ParsedEventCallbackIndex>,
    pub body: Option<ParsedBlockExpression>
}

impl ParsedEventCallbackDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<()> {
        let this_type = context.get_current_type().unwrap();
        let type_id = this_type.borrow().type_id;
        let qualifier = self.event_callback_qualifier.process();
        let step = self.step.as_ref().and_then(|step| step.process(context)).map(|step| step.process()).unwrap_or(EventCallbackStep::Start);

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
                        // let event_class_name = BuiltinType::Event.get_name();
                        // let is_valid_event = event_type.borrow().self_type.inherits_from(event_class_name);

                        let ty = event_type.borrow().self_type.clone();

                        match ty.is_object() {
                            true => (name, event_type),
                            false => {
                                // context.errors.generic(name, format!("type `{}` does not inherit from `{}`", event_type.borrow().name.as_str(), event_class_name));
                                context.errors.expected_class_type(name, &ty);
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

        let index_vasm = self.index.as_ref()
            .and_then(|index| index.process(context))
            .unwrap_or_else(|| context.vasm().int(qualifier.get_default_priority()));

        let event_argument = ArgumentInfo {
            name: Identifier::new(EVENT_VAR_NAME, None),
            ty: event_type.borrow().self_type.clone(),
            is_optional: false,
            default_value: context.vasm()
        };
        let output_argument = ArgumentInfo {
            name: Identifier::new(EVENT_OPTIONS_VAR_NAME, None),
            ty: context.get_builtin_type(BuiltinType::EventOptions, vec![]),
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
            method_details: None,
            kind: FunctionKind::EventCallback,
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
                                                    .get_field(&field_info.ty, field_info.offset, None)
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

        let event_type_name = event_type.borrow().name.to_string();

        this_type.with_mut(|mut type_unwrapped| {
            if step == EventCallbackStep::Start {
                let event_callback = EventCallback {
                    declarer: this_type.clone(),
                    event_type: event_type.clone(),
                    index_vasm,
                    start: function_wrapped,
                    progress: None,
                    end: None,
                };

                type_unwrapped.event_callbacks
                    .entry(event_type)
                    .or_insert_with(|| vec![])
                    .push(event_callback);
            } else {
                if let Some(index) = &self.index {
                    context.errors.generic(index, format!("cannot specify index for an event callback with the `progress` or `end` step"));
                }

                let step_location = self.step.as_ref().unwrap();
                let last_event_callback = type_unwrapped.event_callbacks
                    .get_mut(&event_type)
                    .and_then(|list| list.last_mut())
                    .filter(|event_callback| event_callback.declarer == this_type);
                
                if let Some(mut event_callback) = last_event_callback {
                    match step {
                        EventCallbackStep::Start => unreachable!(),
                        EventCallbackStep::Progress => {
                            if event_callback.progress.is_none() {
                                event_callback.progress = Some(function_wrapped);
                            } else {
                                context.errors.generic(step_location, format!("a `progress` step has already been declared for this event callback"));
                            }
                        },
                        EventCallbackStep::End => {
                            if event_callback.end.is_none() {
                                event_callback.end = Some(function_wrapped);
                            } else {
                                context.errors.generic(step_location, format!("a `end` step has already been declared for this event callback"));
                            }
                        },
                    }
                } else {
                    context.errors.generic(step_location, format!("no matching event callback for `{}` found", &event_type_name));
                }
            }
        });

        Some(())
    }
}