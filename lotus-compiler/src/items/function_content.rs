use std::{collections::HashSet, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use colored::*;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{BuiltinType, FunctionBlueprint, MethodDetails, ProgramContext, ScopeKind, Signature, SELF_VAR_NAME, Type, VI, VariableInfo, VariableKind, Vasm, EventCallbackDetails, HAS_TARGET_METHOD_NAME, EVENT_OUTPUT_VAR_NAME, EVENT_VAR_NAME, CompilationError}, utils::Link, vasm, wat};
use super::{EventCallbackQualifier, FunctionBody, FunctionConditionList, FunctionSignature, Identifier, MethodMetaQualifier, MethodQualifier, BlockExpression, TypeParameters, Visibility, Expression};

#[parsable]
pub struct FunctionContent {
    pub meta_qualifier: Option<MethodMetaQualifier>,
    pub qualifier: Option<MethodQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub name: Identifier,
    pub parameters: TypeParameters,
    #[parsable(brackets="[]")]
    pub priority: Option<Expression>,
    pub signature: Option<FunctionSignature>,
    pub body: FunctionBody,
}

impl FunctionContent {
    pub fn is_autogen(&self) -> bool {
        self.meta_qualifier.contains(&MethodMetaQualifier::Autogen)
    }

    pub fn process_signature(&self, context: &mut ProgramContext) -> Link<FunctionBlueprint> {
        let mut function_blueprint = FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: self.name.clone(),
            visibility: Visibility::Private,
            parameters: IndexMap::new(),
            is_raw_wasm: false,
            body: Vasm::void(),
            argument_names: vec![],
            signature: Signature::default(),
            argument_variables: vec![],
            owner_type: None,
            owner_interface: None,
            closure_details: None,
            method_details: None,
        };

        let mut method_details = MethodDetails {
            event_callback_details: None,
            first_declared_by: None,
            dynamic_index: None,
        };

        let current_type = context.get_current_type();
        let type_id = current_type.as_ref().map(|t| t.borrow().type_id);
        let is_method = type_id.is_some();
        let is_dynamic = self.qualifier.contains(&MethodQualifier::Dynamic);
        let is_static = self.qualifier.contains(&MethodQualifier::Static);
        let is_autogen = self.meta_qualifier.contains(&MethodMetaQualifier::Autogen);
        let parameters = self.parameters.process(context);
        let has_parameters = !parameters.is_empty();
        let is_raw_wasm = self.body.is_raw_wasm();

        if self.event_callback_qualifier.is_none() {
            context.declare_shared_identifier(&self.name);

            for details in parameters.values() {
                context.declare_shared_identifier(&details.name);
            }
        }

        function_blueprint.parameters = parameters;

        if let Some(type_wrapped) = &current_type {
            if is_dynamic {
                if has_parameters {
                    context.errors.add_generic(self, format!("dynamic methods cannot have parameters"));
                }

                if is_raw_wasm {
                    context.errors.add_generic(self, format!("dynamic methods cannot be raw wasm"));
                }

                method_details.dynamic_index = Some(-1);
            }

            function_blueprint.owner_type = Some(type_wrapped.clone());
            method_details.first_declared_by = Some(type_wrapped.clone());

            if !is_static {
                function_blueprint.signature.this_type = Some(type_wrapped.borrow().self_type.clone());
            }

            function_blueprint.method_details = Some(method_details);
        } else {
            if is_static {
                context.errors.add_generic(self, format!("regular functions cannot be static"));
            }

            if is_dynamic {
                context.errors.add_generic(self, format!("regular functions cannot be dynamic"));
            }

            if is_autogen {
                context.errors.add_generic(self, format!("regular functions cannot be autogen"));
            }
        }

        let function_wrapped = context.functions.insert(function_blueprint, type_id);

        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(signature) = &self.signature {
            let (arguments, return_type) = signature.process(context);

            function_wrapped.with_mut(|mut function_unwrapped| {
                function_unwrapped.argument_names = arguments.iter().map(|(name, ty)| name.clone()).collect();
                function_unwrapped.signature.argument_types = arguments.iter().map(|(name, ty)| ty.clone()).collect();
                function_unwrapped.signature.return_type = return_type.unwrap_or(context.void_type());
            });
        }
        // else if is_dynamic {
        //     if let Some(type_wrapped) = context.get_current_type() {
        //         type_wrapped.with_ref(|type_unwrapped| {
        //             if let Some(parent) = &type_unwrapped.parent {
        //                 parent.ty.get_type_blueprint().with_ref(|parent_type_unwrapped| {
        //                     if let Some(prev_method) = parent_type_unwrapped.regular_methods.get(self.name.as_str()) {
        //                         prev_method.function.with_ref(|prev_method_unwrapped| {
        //                             let hash = self.location.get_hash();
        //                             let arguments = prev_method_unwrapped.arguments.iter().map(|var_info| var_info.replace_type_parameters(&parent.ty, hash)).collect();
        //                             let return_type = prev_method_unwrapped.return_type.replace_parameters(Some(&parent.ty), &[]);

        //                             signature_inferred = true;

        //                             function_blueprint.with_mut(|mut function_unwrapped| {
        //                                 function_unwrapped.arguments = arguments;
        //                                 function_unwrapped.return_type = return_type;
        //                             });
        //                         });
        //                     }
        //                 });
        //             }
        //         });
        //     }
        // }

        if let Some(qualifier) = &self.event_callback_qualifier {
            if let Some(type_wrapped) = context.get_current_type() {
                if let Some(signature) = &self.signature {
                    context.errors.add_generic(signature, format!("unexpected function signature"));
                }

                if is_static {
                    context.errors.add_generic(self, format!("event callbacks cannot be static"));
                }

                if !self.parameters.list.is_empty() {
                    context.errors.add_generic(self, format!("event callbacks cannot have parameters"));
                }

                if let Some(event_type_wrapped) = context.types.get_by_identifier(&self.name) {
                    context.access_wrapped_shared_identifier(&event_type_wrapped, &self.name);

                    // if event_type_wrapped.borrow().self_type.inherits_from(BuiltinType::Event.get_name()) {
                        if let Some(type_wrapped) = context.get_current_type() {
                            function_wrapped.with_mut(|mut function_unwrapped| {
                                function_unwrapped.argument_names = vec![
                                    Identifier::unlocated(EVENT_VAR_NAME),
                                    Identifier::unlocated(EVENT_OUTPUT_VAR_NAME),
                                ];
                                function_unwrapped.signature = Signature {
                                    this_type: Some(type_wrapped.borrow().self_type.clone()),
                                    argument_types: vec![
                                        event_type_wrapped.borrow().self_type.clone(),
                                        context.get_builtin_type(BuiltinType::EventOutput, vec![])
                                    ],
                                    return_type: Type::Void,
                                };

                                function_unwrapped.method_details.as_mut().unwrap().event_callback_details.insert(EventCallbackDetails {
                                    event_type: event_type_wrapped.clone(),
                                    qualifier: qualifier.clone(),
                                    priority: vasm![],
                                });
                            });
                        }
                    // } else {
                        // context.errors.add_generic(&self.name, format!("type `{}` is not an event", &self.name));
                    // }
                } else {
                    context.errors.add_generic(&self.name, format!("undefined type `{}`", &self.name.as_str().bold()));
                }
            } else {
                context.errors.add_generic(self, format!("regular functions cannot be event callbacks"));
            }
        } else {
            if self.signature.is_none() {
                context.errors.add_generic(&self.name, format!("missing function signature"));
            }
        }

        context.pop_scope();

        function_wrapped
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        let type_id = context.get_current_type().map(|t| t.borrow().type_id);
        let function_wrapped = context.functions.get_by_location(&self.name, type_id);
        let is_raw_wasm = self.body.is_raw_wasm();
        let return_type = function_wrapped.borrow().signature.return_type.clone();

        let priority_vasm = match &self.priority {
            Some(expression) => match expression.process(Some(&context.int_type()), context) {
                Some(vasm) => {
                    if !vasm.ty.is_int() {
                        context.errors.add(CompilationError::type_mismatch(expression, &context.int_type(), &vasm.ty));
                    }

                    vasm
                },
                None => vasm![],
            },
            None => match &self.event_callback_qualifier {
                Some(qualifier) => vasm![VI::int(qualifier.get_default_priority())],
                None => vasm![],
            },
        };

        function_wrapped.with_mut(|mut function_unwrapped| {
            if let Some(method_details) = &mut function_unwrapped.method_details {
                if let Some(event_callback_details) = &mut method_details.event_callback_details {
                    event_callback_details.priority = priority_vasm;
                }
            }
        });
        
        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(mut vasm) = self.body.process(Some(&return_type), context) {
            function_wrapped.with_mut(|mut function_unwrapped| {
                if let FunctionBody::Block(block) = &self.body {
                    if self.event_callback_qualifier.is_some() {
                        vasm.instructions.push(VI::drop(&vasm.ty));
                    } else if !vasm.ty.is_assignable_to(&return_type) {
                        context.errors.add_generic(&block, format!("expected `{}`, got `{}`", &return_type, &vasm.ty));
                    }
                }

                if let Some(method_details) = &function_unwrapped.method_details {
                    if let Some(event_callback_details) = &method_details.event_callback_details {
                        if let Some(event_field_name) = event_callback_details.qualifier.get_event_field_name() {
                            let event_var_info = function_unwrapped.argument_variables.iter().find(|var_info| var_info.name().is(EVENT_VAR_NAME)).unwrap();
                            let self_var_info = function_unwrapped.argument_variables.iter().find(|var_info| var_info.name().is(SELF_VAR_NAME)).unwrap();

                            match event_var_info.ty().get_field(event_field_name) {
                                Some(field_info) => {
                                    match field_info.ty.is_object() {
                                        true => {
                                            let current_function_level = Some(context.get_function_level());

                                            vasm.instructions.insert(0, VI::if_then_else(None, vasm![
                                                VI::get_var(&event_var_info, current_function_level),
                                                VI::get_field(&field_info.ty, field_info.offset),
                                                VI::get_var(&self_var_info, current_function_level),
                                                VI::raw(wat!["i32.eq"])
                                            ], vasm![], vasm![
                                                VI::return_value(vasm![VI::none(&return_type, context)])
                                            ]));
                                        },
                                        false => {
                                            context.errors.add_generic(&event_callback_details.qualifier, format!("field `{}` of type `{}` is not an object", event_field_name.bold(), event_var_info.ty()));
                                        },
                                    }
                                },
                                None => {
                                    context.errors.add_generic(&event_callback_details.qualifier, format!("type `{}` has no field `{}`", event_var_info.ty(), event_field_name.bold()));
                                },
                            }
                        }
                    }
                }

                function_unwrapped.body = vasm;
                function_unwrapped.is_raw_wasm = is_raw_wasm;
            });
        }

        context.pop_scope();
    }
}