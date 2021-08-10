use parsable::parsable;
use crate::{generation::{RESULT_VAR_NAME, Wat}, program::{FunctionAnnotation, ProgramContext, VariableScope, Wasm}};
use super::{FunctionSignature, Identifier, Statement, FullType};

#[parsable]
pub struct FunctionDeclaration {
    #[parsable(prefix="fn")]
    pub name: Identifier,
    pub signature: FunctionSignature,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

impl FunctionDeclaration {
    pub fn process_signature(&self, function_index: usize, context: &mut ProgramContext) {
        let mut function_annotation = FunctionAnnotation::default();
        let (arguments, return_type) = self.signature.process(context);

        function_annotation.index = function_index;
        function_annotation.wasm_name = format!("function_{}", self.name);
        function_annotation.this_type = None;
        function_annotation.payload_type = None;
        function_annotation.arguments = arguments;
        function_annotation.return_type = return_type;

        if context.functions.contains_key(&self.name) {
            context.error(&self.name, format!("duplicate function declaration: `{}`", &self.name));
        }

        context.functions.insert(&self.name, function_annotation);
    }

    pub fn process_body(&self, function_index: usize, context: &mut ProgramContext) -> Option<Wasm> {
        let mut ok = true;
        let mut wasm_func_name = "";
        let mut wat_args = vec![];
        let mut wat_ret = None;
        let mut wat_locals = vec![];
        let mut wat_body = vec![];

        context.reset_local_scope(VariableScope::Local);

        if let Some(function_annotation) = context.functions.get_with_predicate(&self.name, |f| f.index == function_index) {
            context.set_function_return_type(Some(function_annotation.return_type.clone()));

            for (arg_name, arg_type) in &function_annotation.arguments {
                context.push_local_var(arg_name, arg_type);
            }
        }

        for statement in &self.statements {
            if let Some(wasm) = statement.process(context) {
                wat_body.extend(wasm.wat);
            } else {
                ok = false;
            }
        }

        wat_body = vec![Wat::new("block", wat_body)];

        if let Some(function_annotation) = context.functions.get_with_predicate(&self.name, |f| f.index == function_index) {
            wasm_func_name = &function_annotation.wasm_name;
            wat_ret = function_annotation.return_type.get_wasm_type();

            for (arg_name, arg_type) in &function_annotation.arguments {
                if let Some(wasm_type) = arg_type.get_wasm_type() {
                    wat_args.push((arg_name.as_str(), wasm_type));
                } else {
                    ok = false;
                }
            }
        }

        if let Some(return_type) = context.function_return_type {
            if let Some(wasm_type) = return_type.get_wasm_type() {
                wat_locals.push((RESULT_VAR_NAME, wasm_type));
                wat_body.push(Wat::get_local(RESULT_VAR_NAME));
            }
        }

        for local_var_info in context.local_variables.values() {
            if let Some(wasm_type) = local_var_info.ty.get_wasm_type() {
                wat_locals.push((local_var_info.wasm_name.as_str(), wasm_type));
            }
        }

        match ok {
            true => Some(Wasm::untyped(Wat::declare_function(wasm_func_name, None, wat_args, wat_ret, wat_locals, wat_body))),
            false => None
        }
    }
}