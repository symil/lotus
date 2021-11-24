use std::{array, collections::HashSet, slice::from_ref};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::{DataLocation, parsable};
use crate::{items::{MethodQualifier, Visibility}, program::{FunctionBlueprint, ProgramContext, ScopeKind, Signature, Type, VI, VariableInfo, VariableKind, Vasm}, vasm};
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
        let (arg_types, return_type) = match type_hint {
            Some(Type::Function(signature)) => (signature.argument_types.as_slice(), &signature.return_type),
            _ => (EMPTY_TYPE_LIST.as_slice(), &Type::Void)
        };

        let mut argument_names = vec![];
        let mut argument_types = vec![];

        for (i, name) in arg_names.iter().enumerate() {
            let arg_type = match arg_types.get(i) {
                Some(ty) => ty.clone(),
                None => {
                    context.errors.add(name, format!("cannot infer type of `{}`", name.as_str().bold()));
                    Type::Undefined
                },
            };

            argument_names.push(name.clone());
            argument_types.push(arg_type);
        }

        let hint_signature = Signature {
            this_type: None,
            argument_types,
            return_type: return_type.clone(),
        };

        let function_wrapped = context.functions.insert(FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: Identifier::new("anonymous_function", self),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            argument_names,
            signature: hint_signature.clone(),
            argument_variables: vec![],
            is_raw_wasm: false,
            closure_details: None,
            method_details: None,
            body: vasm![],
        }, None);

        context.push_scope(ScopeKind::Function(function_wrapped.clone()));

        if let Some(vasm) = self.body.process(Some(return_type), context) {
            let signature = function_wrapped.with_mut(|mut function_unwrapped| {
                function_unwrapped.signature.return_type = vasm.ty.clone();
                function_unwrapped.body = vasm;

                function_unwrapped.signature.clone()
            });

            result = Some(Vasm::new(Type::Function(Box::new(signature)), vec![], vec![VI::function_index(&function_wrapped, &[])]));
        }

        context.pop_scope();

        result
    }
}