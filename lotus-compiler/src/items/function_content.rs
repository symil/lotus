use std::{collections::HashSet, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use colored::*;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{BuiltinType, FunctionBlueprint, MethodDetails, ProgramContext, ScopeKind, Signature, THIS_VAR_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, utils::Link, vasm};
use super::{EventCallbackQualifier, FunctionBody, FunctionConditionList, FunctionSignature, Identifier, MethodMetaQualifier, MethodQualifier, BlockExpression, TypeParameters, Visibility};

#[parsable]
pub struct FunctionContent {
    pub meta_qualifier: Option<MethodMetaQualifier>,
    pub qualifier: Option<MethodQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub name: Identifier,
    pub parameters: TypeParameters,
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
            closure_details: None,
            method_details: None,
        };

        let mut method_details = MethodDetails {
            qualifier: self.qualifier,
            event_callback_details: None,
            owner_type: None,
            owner_interface: None,
            first_declared_by: None,
            dynamic_index: -1,
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

        function_blueprint.parameters = parameters;

        if let Some(type_wrapped) = &current_type {
            if is_dynamic {
                if has_parameters {
                    context.errors.add_generic(self, format!("dynamic methods cannot have parameters"));
                }

                if is_raw_wasm {
                    context.errors.add_generic(self, format!("dynamic methods cannot be raw wasm"));
                }
            }

            method_details.owner_type = Some(type_wrapped.clone());
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
                    if event_type_wrapped.borrow().self_type.inherits_from(BuiltinType::Event.get_name()) {
                        if let Some(type_wrapped) = context.get_current_type() {
                            function_wrapped.with_mut(|mut function_unwrapped| {
                                function_unwrapped.argument_names = vec![
                                    Identifier::new("evt", self),
                                    Identifier::new("__output", self),
                                ];
                                function_unwrapped.signature = Signature {
                                    this_type: Some(type_wrapped.borrow().self_type.clone()),
                                    argument_types: vec![
                                        event_type_wrapped.borrow().self_type.clone(),
                                        context.get_builtin_type(BuiltinType::EventOutput, vec![])
                                    ],
                                    return_type: Type::Void,
                                };

                                function_unwrapped.method_details.as_mut().unwrap().event_callback_details.insert((qualifier.clone(), event_type_wrapped.clone()));
                            });
                        }
                    } else {
                        context.errors.add_generic(&self.name, format!("type `{}` is not an event", &self.name));
                    }
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
        
        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(vasm) = self.body.process(Some(&return_type), context) {
            function_wrapped.with_mut(|mut function_unwrapped| {
                if let FunctionBody::Block(block) = &self.body {
                    if !vasm.ty.is_assignable_to(&return_type) {
                        context.errors.add_generic(&block, format!("expected `{}`, got `{}`", &return_type, &vasm.ty));
                    }
                }

                function_unwrapped.body = vasm;
                function_unwrapped.is_raw_wasm = is_raw_wasm;
            });
        }

        context.pop_scope();
    }
}