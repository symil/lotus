use parsable::parsable;
use crate::{generation::{RESULT_VAR_NAME, Wat}, program::{FunctionAnnotation, ProgramContext, Type, VarKind, VariableScope, Wasm}};
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
    pub fn process_signature(&self, index: usize, context: &mut ProgramContext) {
        let mut function_annotation = FunctionAnnotation::default();
        let (arguments, return_type) = self.signature.process(context);

        if self.name.as_str() == "main" {
            if !arguments.is_empty() {
                context.error(&self.name, format!("main function must not take any argument"));
            }

            if !return_type.is_void() {
                context.error(&self.name, format!("main function must not have a return type"));
            }
        }

        function_annotation.index = index;
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

    pub fn process_body(&self, index: usize, context: &mut ProgramContext) {
        let mut ok = true;
        let mut wasm_func_name = String::new();
        let mut wat_args = vec![];
        let mut wat_ret = None;
        let mut wat_locals = vec![];
        let mut wat_body = vec![];

        let mut return_type = None;
        let mut arguments = vec![];

        if let Some(function_annotation) = context.functions.get_by_id(&self.name, index) {
            return_type = match function_annotation.return_type {
                Type::Void => None,
                _ => Some(function_annotation.return_type.clone())
            };
            arguments = function_annotation.arguments.clone();
        }

        context.reset_local_scope(VariableScope::Local);
        context.set_function_return_type(return_type);

        for (arg_name, arg_type) in &arguments {
            context.push_argument_var(arg_name, arg_type);
        }

        for statement in &self.statements {
            if let Some(wasm) = statement.process(context) {
                wat_body.extend(wasm.wat);
            } else {
                ok = false;
            }
        }

        wat_body = vec![Wat::new("block", wat_body)];

        if let Some(function_annotation) = context.functions.get_by_id(&self.name, index) {
            wasm_func_name = function_annotation.wasm_name.clone();
            wat_ret = function_annotation.return_type.get_wasm_type();

            for (arg_name, arg_type) in &function_annotation.arguments {
                if let Some(wasm_type) = arg_type.get_wasm_type() {
                    wat_args.push((arg_name.to_string(), wasm_type));
                } else {
                    ok = false;
                }
            }
        }

        if let Some(return_type) = &context.function_return_type {
            if let Some(wasm_type) = return_type.get_wasm_type() {
                wat_locals.push((RESULT_VAR_NAME.to_string(), wasm_type));
                wat_body.push(Wat::get_local(RESULT_VAR_NAME));
            }
        }

        for local_var_info in context.local_variables.values() {
            if local_var_info.kind == VarKind::Local {
                if let Some(wasm_type) = local_var_info.ty.get_wasm_type() {
                    wat_locals.push((local_var_info.wasm_name.clone(), wasm_type));
                }
            }
        }

        if context.function_return_type.is_some() && !context.return_found {
            context.error(&self.signature.return_type.as_ref().unwrap(), format!("not all branches return a valid value for the function"));
            ok = false;
        }

        let wat_args : Vec<(&str, &str)> = wat_args.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.clone())).collect();
        let wat_locals : Vec<(&str, &str)> = wat_locals.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.clone())).collect();

        if let Some(function_annotation) = context.functions.get_mut_by_id(&self.name, index) {
            function_annotation.wat = Wat::declare_function(&wasm_func_name, None, wat_args, wat_ret, wat_locals, wat_body);
        }
    }
}