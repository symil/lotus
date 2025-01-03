use std::{collections::HashSet, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use colored::*;
use parsable::parsable;
use crate::{items::ParsedTypeQualifier, program::{BuiltinType, FunctionBlueprint, MethodDetails, ProgramContext, ScopeKind, Signature, SELF_VAR_NAME, Type, VariableInfo, VariableKind, Vasm, HAS_TARGET_METHOD_NAME, EVENT_OPTIONS_VAR_NAME, EVENT_VAR_NAME, CompilationError, SignatureContent, MethodMetaQualifier, MethodQualifier, Visibility, FunctionBody, FieldVisibility, FunctionKind}, utils::Link, wat};
use super::{ParsedEventCallbackQualifierKeyword, ParsedFunctionBody, ParsedFunctionSignature, Identifier, ParsedMethodMetaQualifier, ParsedMethodQualifier, ParsedBlockExpression, ParsedTypeParameters, ParsedVisibilityToken, ParsedExpression};

#[parsable]
pub struct ParsedFunctionOrMethodContent {
    pub meta_qualifier: Option<ParsedMethodMetaQualifier>,
    pub qualifier: Option<ParsedMethodQualifier>,
    pub name: Identifier,
    pub parameters: Option<ParsedTypeParameters>,
    pub signature: ParsedFunctionSignature,
    pub body: Option<ParsedFunctionBody>,
}

impl ParsedFunctionOrMethodContent {
    fn get_meta_qualifier(&self) -> MethodMetaQualifier {
        match &self.meta_qualifier {
            Some(qualifier) => qualifier.process(),
            None => MethodMetaQualifier::None,
        }
    }

    pub fn is_autogen(&self) -> bool {
        self.get_meta_qualifier() == MethodMetaQualifier::Autogen
    }

    pub fn process_signature(&self, context: &mut ProgramContext) -> Link<FunctionBlueprint> {
        let current_type = context.get_current_type();
        let type_id = current_type.as_ref().map(|t| t.borrow().type_id);
        let is_method = type_id.is_some();
        let qualifier = self.qualifier.as_ref().map(|keyword| keyword.process()).unwrap_or(MethodQualifier::None);
        let is_dynamic = qualifier == MethodQualifier::Dynamic;
        let is_static = qualifier == MethodQualifier::Static;
        let is_autogen = self.is_autogen();
        let parameters = match &self.parameters {
            Some(params) => params.process(context),
            None => IndexMap::new(),
        };
        let has_parameters = !parameters.is_empty();
        let is_raw_wasm = self.body.as_ref().map(|body| body.is_raw_wasm()).unwrap_or(false);

        let mut function_blueprint = FunctionBlueprint {
            name: self.name.clone(),
            visibility: Visibility::None,
            parameters,
            arguments: vec![],
            signature: Signature::undefined(),
            argument_variables: vec![],
            owner_type: current_type.clone(),
            owner_interface: None,
            closure_details: None,
            method_details: None,
            kind: FunctionKind::Standard,
            body: FunctionBody::Empty,
        };

        for details in function_blueprint.parameters.values() {
            context.rename_provider.add_occurence(&details.name, &details.name);
        }

        if let Some(type_wrapped) = &current_type {
            let mut dynamic_index = None;

            if is_dynamic {
                if has_parameters {
                    context.errors.generic(self, format!("dynamic methods cannot have parameters"));
                }

                if is_raw_wasm {
                    context.errors.generic(self, format!("dynamic methods cannot be raw wasm"));
                }

                dynamic_index = Some(-1);
            }

            function_blueprint.method_details = Some(MethodDetails {
                qualifier,
                visibility: FieldVisibility::from_name(self.name.as_str()),
                first_declared_by: Some(type_wrapped.clone()),
                dynamic_index,
                is_autogen,
            });
        } else {
            if is_static {
                context.errors.generic(self, format!("regular functions cannot be static"));
            }

            if is_dynamic {
                context.errors.generic(self, format!("regular functions cannot be dynamic"));
            }

            if is_autogen {
                context.errors.generic(self, format!("regular functions cannot be autogen"));
            }
        }

        let function_wrapped = context.functions.insert(function_blueprint, type_id);

        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        let (arguments, return_type) = self.signature.process(context);
        let signature_this_type = match is_static {
            true => None,
            false => current_type.map(|t| t.borrow().self_type.clone()),
        };

        let signature = Signature::create(
            signature_this_type,
            arguments.iter().map(|arg| arg.ty.clone()).collect(),
            return_type.unwrap_or(context.void_type())
        );

        function_wrapped.with_mut(|mut function_unwrapped| {
            function_unwrapped.arguments = arguments;
            function_unwrapped.signature = signature;

            context.rename_provider.add_occurence(&self.name, &self.name);
        });

        context.pop_scope();

        function_wrapped
    }

    pub fn process_default_arguments(&self, context: &mut ProgramContext) {
        let type_id = context.get_current_type().map(|t| t.borrow().type_id);
        let function_wrapped = context.functions.get_by_location(&self.name, type_id);

        set_function_argument_default_values(&function_wrapped, &self.signature, context);
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        let type_id = context.get_current_type().map(|t| t.borrow().type_id);
        let function_wrapped = context.functions.get_by_location(&self.name, type_id);
        let body = match &self.body {
            Some(body) => body,
            None => return context.errors.expected_function_body(self).void(),
        };

        let is_raw_wasm = body.is_raw_wasm();
        let return_type = function_wrapped.borrow().signature.return_type.clone();
        
        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(body) = body.process(Some(&return_type), context) {
            function_wrapped.with_mut(|mut function_unwrapped| {
                if let FunctionBody::Vasm(vasm) = &body {
                    if !vasm.ty.is_assignable_to(&return_type) {
                        let location = self.signature.return_type.as_ref().map(|t| t.location.clone()).unwrap_or(self.signature.location.get_end());
                        context.errors.type_mismatch(&location, &return_type, &vasm.ty);
                    }
                }

                function_unwrapped.body = body;
            });
        }

        context.pop_scope();
    }
}

pub fn set_function_argument_default_values(function_wrapped: &Link<FunctionBlueprint>, parsed_signature: &ParsedFunctionSignature, context: &mut ProgramContext) {
    let arg_types : Vec<Type> = function_wrapped.borrow().arguments.iter().map(|arg| arg.ty.clone()).collect();
    let mut arg_default_values = vec![];

    for (parsed_arg, arg_type) in parsed_signature.arguments.iter().zip(arg_types.iter()) {
        let mut vasm = None;

        if let Some(assignment) = &parsed_arg.default_value {
            if let Some(expression) = &assignment.expression {
                vasm = expression.process(arg_type.to_type_hint(), context);
            }
        }

        arg_default_values.push(vasm);
    }

    function_wrapped.with_mut(|mut function_unwrapped| {
        for (default_vasm, arg_info) in arg_default_values.into_iter().zip(function_unwrapped.arguments.iter_mut()) {
            if let Some(vasm) = default_vasm {
                arg_info.default_value = vasm;
            }
        }
    });
}