use std::rc::Rc;

use indexmap::IndexSet;
use parsable::parsable;
use crate::{generation::Wat, items::TypeQualifier, program::{FunctionBlueprint, PAYLOAD_VAR_NAME, ProgramContext, RESULT_VAR_NAME, ScopeKind, THIS_VAR_NAME, Type, VariableInfo, VariableKind, Vasm}};
use super::{EventCallbackQualifier, FunctionBody, FunctionConditionList, FunctionQualifier, FunctionSignature, Identifier, StatementList, Visibility};

#[parsable]
pub struct FunctionContent {
    pub qualifier: Option<FunctionQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub name: Identifier,
    pub conditions: Option<FunctionConditionList>,
    pub signature: Option<FunctionSignature>,
    pub body: FunctionBody,
}

impl FunctionContent {
    pub fn process_signature(&self, context: &mut ProgramContext) -> FunctionBlueprint {
        let mut function_blueprint = FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: self.name.clone(),
            visibility: Visibility::Private,
            event_callback_qualifier: None,
            owner_type: None,
            this_arg: None,
            payload_arg: None,
            conditions: vec![],
            arguments: vec![],
            return_value: None,
            is_raw_wasm: false,
            body: Vasm::empty()
        };

        let is_static = self.qualifier.contains(&FunctionQualifier::Static);

        if let Some(type_blueprint) = &context.current_type {
            function_blueprint.owner_type = Some(type_blueprint.clone());

            if !is_static {
                function_blueprint.this_arg = Some(VariableInfo::new(Identifier::new(THIS_VAR_NAME, self), Type::Actual(type_blueprint.get_info()), VariableKind::Argument));
            }
        } else if is_static {
            context.errors.add(self, "regular functions cannot be static");
        }

        if let Some(signature) = &self.signature {
            let (arguments, return_type) = signature.process(context);

            function_blueprint.arguments = arguments.into_iter().map(|(name, ty)| VariableInfo::new(name, ty, VariableKind::Argument)).collect();
            function_blueprint.return_value = return_type.and_then(|ty| Some(VariableInfo::new(Identifier::unlocated(RESULT_VAR_NAME), ty, VariableKind::Local)))
        }

        if let Some(qualifier) = &self.event_callback_qualifier {
            if let Some(type_id) = context.current_type {
                if let Some(signature) = &self.signature {
                    context.errors.add(signature, "event callbacks do not take arguments nor have a return type");
                }

                if is_static {
                    context.errors.add(self, "event callbacks cannot be static");
                }

                if let Some(event_type_blueprint) = context.types.get_by_name(&self.name) {
                    function_blueprint.payload_arg = Some(VariableInfo::new(Identifier::new(PAYLOAD_VAR_NAME, self), Type::Actual(event_type_blueprint.get_info()), VariableKind::Argument));

                    if event_type_blueprint.borrow().qualifier != TypeQualifier::Class {
                        context.errors.add(&self.name, format!("type `{}` is not a class", &self.name));
                    } else if let Some(conditions) = &self.conditions {
                        function_blueprint.conditions = conditions.process(&event_type_blueprint, context);
                    }
                } else {
                    context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
                }
            } else {
                context.errors.add(self, "regular functions cannot be event callbacks");
            }
        } else {
            if self.conditions.is_some() {
                context.errors.add(self, "only event callbacks can have conditions");
            }

            if self.signature.is_none() {
                context.errors.add(&self.name, "missing function signature");
            }
        }

        function_blueprint
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        let function_blueprint = context.functions.get_by_location(&self.name);

        context.current_function = Some(function_blueprint.clone());
        context.reset_local_scope();
        context.push_scope(ScopeKind::Function);

        let is_raw_wasm = self.body.is_raw_wasm();

        if let Some(vasm) = self.body.process(context) {
            function_blueprint.borrow_mut().body = vasm;
            function_blueprint.borrow_mut().is_raw_wasm = is_raw_wasm;
        }

        if !is_raw_wasm {
            if let Some(return_type) = &function_blueprint.borrow().return_value {
                if !context.return_found {
                    context.errors.add(&self.signature.as_ref().unwrap().return_type.as_ref().unwrap(), format!("not all branches return a value for the function"));
                }
            }
        }

        context.pop_scope();
        context.current_function = None;
    }
}