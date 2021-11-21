use std::rc::Rc;
use indexmap::{IndexMap, IndexSet};
use colored::*;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{FunctionBlueprint, PAYLOAD_VAR_NAME, ProgramContext, RESULT_VAR_NAME, ScopeKind, THIS_VAR_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, utils::Link, vasm};
use super::{EventCallbackQualifier, FunctionBody, FunctionConditionList, FunctionSignature, Identifier, MethodMetaQualifier, MethodQualifier, BlockExpression, TypeParameters, Visibility};

#[parsable]
pub struct FunctionContent {
    pub meta_qualifier: Option<MethodMetaQualifier>,
    pub qualifier: Option<MethodQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub name: Identifier,
    pub parameters: TypeParameters,
    pub conditions: Option<FunctionConditionList>,
    pub signature: Option<FunctionSignature>,
    pub body: FunctionBody,
}

impl FunctionContent {
    pub fn is_autogen(&self) -> bool {
        self.meta_qualifier.contains(&MethodMetaQualifier::Autogen)
    }

    pub fn process_signature(&self, context: &mut ProgramContext) -> Link<FunctionBlueprint> {
        let function_unwrapped = FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: self.name.clone(),
            visibility: Visibility::Private,
            qualifier: self.qualifier.unwrap_or(MethodQualifier::Regular),
            parameters: IndexMap::new(),
            event_callback_qualifier: None,
            owner_type: None,
            owner_interface: None,
            first_declared_by: None,
            this_arg: None,
            payload_arg: None,
            conditions: vec![],
            arguments: vec![],
            return_type: Type::Undefined,
            dynamic_index: -1,
            is_raw_wasm: false,
            body: Vasm::void(),
        };

        let type_id = context.current_type.as_ref().and_then(|t| Some(t.borrow().type_id));
        let is_dynamic = function_unwrapped.is_dynamic();
        let function_blueprint = context.functions.insert(function_unwrapped, type_id);
        let is_static = self.qualifier.contains(&MethodQualifier::Static);
        let is_autogen = self.meta_qualifier.contains(&MethodMetaQualifier::Autogen);
        let parameters = self.parameters.process(context);
        let has_parameters = !parameters.is_empty();
        let mut signature_inferred = false;

        context.current_function = Some(function_blueprint.clone());

        function_blueprint.with_mut(|mut function_unwrapped| {
            function_unwrapped.parameters = parameters;

            if let Some(type_blueprint) = &context.current_type {
                if is_dynamic {
                    if has_parameters {
                        context.errors.add(self, "dynamic methods cannot have parameters");
                    }

                    if self.body.is_raw_wasm() {
                        context.errors.add(self, "dynamic methods cannot be raw wasm");
                    }
                }

                function_unwrapped.owner_type = Some(type_blueprint.clone());
                function_unwrapped.first_declared_by = Some(type_blueprint.clone());

                if !is_static {
                    function_unwrapped.this_arg = Some(VariableInfo::from(Identifier::new(THIS_VAR_NAME, self), type_blueprint.borrow().self_type.clone(), VariableKind::Argument));
                }
            } else {
                if is_static {
                    context.errors.add(self, "regular functions cannot be static");
                }
                
                if function_unwrapped.is_dynamic() {
                    context.errors.add(self, "regular functions cannot be dynamic");
                }

                if is_autogen {
                    context.errors.add(self, "regular functions cannot be autogen");
                }
            }
        });

        if let Some(signature) = &self.signature {
            let (arguments, return_type) = signature.process(context);

            function_blueprint.with_mut(|mut function_unwrapped| {
                function_unwrapped.arguments = arguments.into_iter().map(|(name, ty)| VariableInfo::from(name, ty, VariableKind::Argument)).collect();
                function_unwrapped.return_type = return_type.unwrap_or(context.void_type());
            });
        } else if is_dynamic {
            if let Some(type_wrapped) = context.get_current_type() {
                type_wrapped.with_ref(|type_unwrapped| {
                    if let Some(parent) = &type_unwrapped.parent {
                        parent.ty.get_type_blueprint().with_ref(|parent_type_unwrapped| {
                            if let Some(prev_method) = parent_type_unwrapped.regular_methods.get(self.name.as_str()) {
                                prev_method.function.with_ref(|prev_method_unwrapped| {
                                    let hash = self.location.get_hash();
                                    let arguments = prev_method_unwrapped.arguments.iter().map(|var_info| var_info.replace_type_parameters(&parent.ty, hash)).collect();
                                    let return_type = prev_method_unwrapped.return_type.replace_parameters(Some(&parent.ty), &[]);

                                    signature_inferred = true;

                                    function_blueprint.with_mut(|mut function_unwrapped| {
                                        function_unwrapped.arguments = arguments;
                                        function_unwrapped.return_type = return_type;
                                    });
                                });
                            }
                        });
                    }
                });
            }
        }

        if let Some(qualifier) = &self.event_callback_qualifier {
            if let Some(type_wrapped) = context.get_current_type() {
                if let Some(signature) = &self.signature {
                    context.errors.add(signature, "event callbacks do not take arguments nor have a return type");
                }

                if is_static {
                    context.errors.add(self, "event callbacks cannot be static");
                }

                if !self.parameters.list.is_empty() {
                    context.errors.add(self, "event callbacks cannot have parameters");
                }

                if let Some(event_type_blueprint) = context.types.get_by_identifier(&self.name) {
                    function_blueprint.borrow_mut().payload_arg = Some(VariableInfo::from(Identifier::new(PAYLOAD_VAR_NAME, self), event_type_blueprint.borrow().self_type.clone(), VariableKind::Argument));

                    if !event_type_blueprint.borrow().is_class() {
                        context.errors.add(&self.name, format!("type `{}` is not a class", &self.name));
                    } else if let Some(conditions) = &self.conditions {
                        function_blueprint.borrow_mut().conditions = conditions.process(&event_type_blueprint, context);
                    }
                } else {
                    context.errors.add(&self.name, format!("undefined type `{}`", &self.name.as_str().bold()));
                }
            } else {
                context.errors.add(self, "regular functions cannot be event callbacks");
            }
        } else {
            if self.conditions.is_some() {
                context.errors.add(self, "only event callbacks can have conditions");
            }

            if self.signature.is_none() && !signature_inferred {
                context.errors.add(&self.name, "missing function signature");
            }
        }

        context.current_function = None;

        function_blueprint
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        let type_id = context.current_type.as_ref().and_then(|t| Some(t.borrow().type_id));
        let function_wrapped = context.functions.get_by_location(&self.name, type_id);
        let mut return_type = Type::Void;
        
        context.current_function = Some(function_wrapped.clone());
        context.reset_local_scope();
        context.push_scope(ScopeKind::Function);

        function_wrapped.with_ref(|function_unwrapped| {
            if let Some(this_var) = &function_unwrapped.this_arg {
                context.push_var(this_var);
            }

            for arg in &function_unwrapped.arguments {
                context.push_var(arg);
            }

            return_type = function_unwrapped.return_type.clone();
        });

        let is_raw_wasm = self.body.is_raw_wasm();

        if let Some(vasm) = self.body.process(Some(&return_type), context) {
            function_wrapped.with_mut(|mut function_unwrapped| {
                if let FunctionBody::Block(block) = &self.body {
                    if !vasm.ty.is_assignable_to(&function_unwrapped.return_type) {
                        context.errors.add(&block, format!("expected `{}`, got `{}`", &function_unwrapped.return_type, &vasm.ty));
                    }
                }

                function_unwrapped.body = vasm;
                function_unwrapped.is_raw_wasm = is_raw_wasm;
            });
        }

        context.pop_scope();
        context.current_function = None;
    }
}