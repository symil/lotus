use std::{array, collections::HashSet, slice::from_ref};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::{ItemLocation, parsable};
use crate::{items::{ParsedMethodQualifier, ParsedVisibilityToken}, program::{BuiltinType, FunctionBlueprint, ProgramContext, RETAIN_METHOD_NAME, ScopeKind, Signature, Type, VariableInfo, VariableKind, Vasm, SignatureContent, TypeContent, Visibility, GET_AT_INDEX_FUNC_NAME, FunctionBody, ANONYMOUS_FUNCTION_NAME}, utils::Link};
use super::{ParsedBlockExpression, ParsedExpression, ParsedAnonymousFunctionArguments, ParsedAnonymousFunctionBody, Identifier};

#[parsable]
pub struct ParsedAnonymousFunction {
    pub arguments: ParsedAnonymousFunctionArguments,
    #[parsable(prefix="=>")]
    pub body: ParsedAnonymousFunctionBody
}

const EMPTY_TYPE_LIST : [Type; 0] = [];

// A closure is a memory block containing 3 values, in this order:
// - Map<int, int> associating a variable id with its address
// - A function index, to be called with `call_indirect` with the Map as its last argument
// - A `retain` function index, to be called when the closure is retained

impl ParsedAnonymousFunction {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;
        let arguments = self.arguments.process(type_hint, context);
        let provided_argument_count = arguments.len();
        let void_type = context.void_type();
        let return_type_hint = match type_hint.map(|t| t.content()) {
            Some(TypeContent::Function(signature)) => Some(signature.return_type.clone()),
            _ => None,
        };
        let (expected_arg_types, expected_return_type) = match type_hint.map(|t| t.content()) {
            Some(TypeContent::Function(signature)) => (signature.argument_types.as_slice(), &signature.return_type),
            _ => (EMPTY_TYPE_LIST.as_slice(), &void_type)
        };

        let mut argument_names = vec![];
        let mut argument_types = vec![];

        for (i, (name, typt_opt)) in arguments.into_iter().enumerate() {
            let arg_type = match typt_opt {
                Some(ty) => ty,
                None => match expected_arg_types.get(i) {
                    Some(ty) => ty.clone(),
                    None => {
                        context.errors.generic(name, format!("cannot infer type of `{}`", name.as_str().bold()));
                        Type::undefined()
                    },
                },
            };

            argument_names.push(name.clone());
            argument_types.push(arg_type);
        }

        if expected_arg_types.len() > provided_argument_count {
            for (i, ty) in expected_arg_types[provided_argument_count..].iter().enumerate() {
                argument_names.push(Identifier::unlocated(&format!("__unused_arg_{}", i + provided_argument_count)));
                argument_types.push(ty.clone());
            }
        }

        let parameters = match context.get_named_current_function() {
            Some(function_wrapped) => function_wrapped.borrow().parameters.clone(),
            None => IndexMap::new(),
        };
        let parameter_types : Vec<Type> = parameters.values().map(|parameter_type_info| Type::function_parameter(parameter_type_info)).collect();

        let mut signature = Signature::create(None, argument_types, expected_return_type.clone());
        let function_wrapped = context.functions.insert(FunctionBlueprint {
            name: Identifier::new(ANONYMOUS_FUNCTION_NAME, Some(self)),
            visibility: Visibility::None,
            parameters,
            argument_names,
            signature: signature.clone(),
            argument_variables: vec![],
            owner_type: context.get_current_type(),
            owner_interface: context.get_current_interface(),
            closure_details: None,
            method_details: None,
            body: FunctionBody::Empty,
        }, None);

        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(mut vasm) = self.body.process(Some(expected_return_type), context) {
            if let Some(ty) = return_type_hint {
                if ty.is_void() && !vasm.ty.is_void() {
                    vasm = vasm.set_void(context);
                }
            }

            signature = Signature::create(None, signature.argument_types.clone(), vasm.ty.clone());

            function_wrapped.with_mut(|mut function_unwrapped| {
                function_unwrapped.signature = signature.clone();
                function_unwrapped.body = FunctionBody::Vasm(vasm);
            });
        }

        result = Some(context.vasm()
            .function_index(&function_wrapped, &parameter_types)
            .set_type(Type::function(signature))
        );

        context.pop_scope();

        function_wrapped.with_mut(|mut function_unwrapped| {
            if let Some(closure_details) = &mut function_unwrapped.closure_details {
                let mut function = FunctionBlueprint::new(Identifier::new("retain_function", Some(self)), context);
                let closure_args_var = VariableInfo::create(Identifier::unique("closure_args"), context.int_type(), VariableKind::Argument, 0, None);
                let mut retain_vasm = context.vasm();

                function.argument_variables = vec![closure_args_var.clone()];

                for arg in &closure_details.variables {
                    let map_type = context.get_builtin_type(BuiltinType::Map, vec![context.int_type(), context.int_type()]);
                    let pointer_type = context.get_builtin_type(BuiltinType::Pointer, vec![arg.ty().clone()]);

                    if !arg.ty().is_undefined() {
                        retain_vasm = retain_vasm
                            .call_static_method(&arg.ty(), RETAIN_METHOD_NAME, &[], vec![context.vasm()
                                .get_tmp_var(&closure_args_var)
                                .call_regular_method(&map_type, "get", &[], vec![context.vasm().int(arg.get_name_hash())], context)
                                .call_regular_method(&pointer_type, GET_AT_INDEX_FUNC_NAME, &[], vec![ context.vasm().int(0i32) ], context)
                                ], context);
                    }
                }

                function.body = FunctionBody::Vasm(retain_vasm);
                closure_details.retain_function = Some(Link::new(function));
            }
        });

        result
    }
}