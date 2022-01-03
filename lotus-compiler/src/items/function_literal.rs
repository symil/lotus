use std::{array, collections::HashSet, slice::from_ref};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::{DataLocation, parsable};
use crate::{items::{MethodQualifier, Visibility}, program::{BuiltinType, FunctionBlueprint, ProgramContext, RETAIN_METHOD_NAME, ScopeKind, Signature, Type, VariableInfo, VariableKind, Vasm, SignatureContent, TypeContent}, utils::Link};
use super::{BlockExpression, Expression, FunctionLiteralArguments, FunctionLiteralBody, Identifier};

#[parsable]
pub struct FunctionLiteral {
    pub arguments: FunctionLiteralArguments,
    #[parsable(prefix="=>")]
    pub body: FunctionLiteralBody
}

const EMPTY_TYPE_LIST : [Type; 0] = [];

// A closure is a memory block containing 3 values, in this order:
// - Map<int, int> associating a variable id with its address
// - A function index, to be called with `call_indirect` with the Map as its last argument
// - A `retain` function index, to be called when the closure is retained

impl FunctionLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;
        let arg_names = self.arguments.process(type_hint, context);
        let void_type = context.void_type();
        let (arg_types, return_type) = match type_hint.map(|t| t.content()) {
            Some(TypeContent::Function(signature)) => (signature.argument_types.as_slice(), &signature.return_type),
            _ => (EMPTY_TYPE_LIST.as_slice(), &void_type)
        };

        let mut argument_names = vec![];
        let mut argument_types = vec![];

        for (i, name) in arg_names.iter().enumerate() {
            let arg_type = match arg_types.get(i) {
                Some(ty) => ty.clone(),
                None => {
                    context.errors.generic(name, format!("cannot infer type of `{}`", name.as_str().bold()));
                    Type::undefined()
                },
            };

            argument_names.push(name.clone());
            argument_types.push(arg_type);
        }

        let mut signature = Signature::create(None, argument_types, return_type.clone());
        let function_wrapped = context.functions.insert(FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: Identifier::new("anonymous_function", self),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            argument_names,
            signature: signature.clone(),
            argument_variables: vec![],
            owner_type: context.get_current_type(),
            owner_interface: context.get_current_interface(),
            is_raw_wasm: false,
            closure_details: None,
            method_details: None,
            body: context.vasm(),
        }, None);

        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(vasm) = self.body.process(Some(return_type), context) {
            signature = Signature::create(None, signature.argument_types.clone(), vasm.ty.clone());

            function_wrapped.with_mut(|mut function_unwrapped| {
                function_unwrapped.signature = signature.clone();
                function_unwrapped.body = vasm;
            });
        }

        result = Some(context.vasm()
            .function_index(&function_wrapped, &[])
            .set_type(Type::function(signature))
        );

        context.pop_scope();

        function_wrapped.with_mut(|mut function_unwrapped| {
            if let Some(closure_details) = &mut function_unwrapped.closure_details {
                let mut function = FunctionBlueprint::new(Identifier::new("retain_function", self), context);
                let closure_args_var = VariableInfo::create(Identifier::unique("closure_args"), context.int_type(), VariableKind::Argument, 0);

                function.argument_variables = vec![closure_args_var.clone()];

                for arg in &closure_details.variables {
                    let map_type = context.get_builtin_type(BuiltinType::Map, vec![context.int_type(), context.int_type()]);
                    let pointer_type = context.get_builtin_type(BuiltinType::Pointer, vec![arg.ty().clone()]);

                    if !arg.ty().is_undefined() {
                        function.body = function.body
                            .call_static_method(&arg.ty(), RETAIN_METHOD_NAME, &[], vec![context.vasm()
                                .get_tmp_var(&closure_args_var)
                                .call_regular_method(&map_type, "get", &[], vec![context.vasm().int(arg.get_name_hash())], context)
                                .call_regular_method(&pointer_type, "get_at", &[], vec![ context.vasm().int(0i32) ], context)
                                ], context);
                            }
                }

                closure_details.retain_function = Some(Link::new(function));
            }
        });

        result
    }
}