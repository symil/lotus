use std::{array, slice::from_ref};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::{DataLocation, parsable};
use crate::{items::{MethodQualifier, Visibility}, program::{FunctionBlueprint, ProgramContext, ScopeKind, Type, VI, VariableInfo, VariableKind, Vasm}};
use super::{BlockExpression, Expression, FunctionLiteralArguments, FunctionLiteralBody, Identifier};

#[parsable]
pub struct FunctionLiteral {
    pub arguments: FunctionLiteralArguments,
    #[parsable(prefix="=>")]
    pub body: FunctionLiteralBody
}

const EMPTY_TYPE_LIST : [Type; 0] = [];

impl FunctionLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;
        let arg_names = self.arguments.process(type_hint, context);
        let (arg_types, return_type) = match type_hint {
            Some(Type::Function(signature)) => (signature.argument_types.as_slice(), &signature.return_type),
            _ => (EMPTY_TYPE_LIST.as_slice(), &Type::Void)
        };

        let mut arguments = vec![];

        for (i, name) in arg_names.iter().enumerate() {
            let arg_type = match arg_types.get(i) {
                Some(ty) => ty.clone(),
                None => {
                    context.errors.add(name, format!("cannot infer type of `{}`", name.as_str().bold()));
                    Type::Undefined
                },
            };

            arguments.push(VariableInfo::from(name.clone(), arg_type, VariableKind::Argument));
        }


        context.push_scope(ScopeKind::Function);

        for arg in &arguments {
            context.push_var(&arg);
        }

        if let Some(vasm) = self.body.process(Some(return_type), context) {
            let function_blueprint = FunctionBlueprint {
                function_id: self.location.get_hash(),
                name: Identifier::new("anonymous", self),
                visibility: Visibility::Private,
                qualifier: MethodQualifier::Regular,
                parameters: IndexMap::new(),
                event_callback_qualifier: None,
                owner_type: None,
                owner_interface: None,
                first_declared_by: None,
                conditions: vec![],
                this_arg: None,
                payload_arg: None,
                arguments,
                return_type: vasm.ty.clone(),
                is_raw_wasm: false,
                dynamic_index: -1,
                body: vasm,
            };
            let signature = function_blueprint.get_signature();
            let function_wrapped = context.functions.insert(function_blueprint, None);

            result = Some(Vasm::new(Type::Function(Box::new(signature)), vec![], vec![VI::function_index(&function_wrapped, &[])]));
        }

        context.pop_scope();

        result
    }
}